use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use rand::Rng;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

type UpdateFunc<'a, T> = dyn 'a + Fn(&[T]) -> T;


struct ComputeCell<'a, T: Copy + PartialEq> {
    value: T,
    update_fn: Box<UpdateFunc<'a, T>>,
    deps: Vec<CellId>
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    pub fn new(value: T, deps: &[CellId], update_fn: Box<UpdateFunc<'a, T>>) -> ComputeCell<'a, T> {
        ComputeCell {
            value,
            deps: deps.to_vec(),
            update_fn,
        }
    }

    pub fn update(&mut self, dep_values: &[T]) {
        self.value = (self.update_fn)(dep_values);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

#[derive(Default)]
pub struct Reactor<'a, T: 'a + Copy + PartialEq> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    dependency_map: HashMap<CellId, Vec<ComputeCellId>>,
    cell_callback_map: HashMap<CellId, Vec<CallbackId>>,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) -> T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: 'a + Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            dependency_map: HashMap::new(),
            cell_callback_map: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(generate_id());
        self.input_cells.insert(id, initial);
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // Make sure all deps exist, & return error if not
        for dep_id in dependencies {
            if !match dep_id {
                CellId::Input(id) => self.input_cells.contains_key(id),
                CellId::Compute(id) => self.compute_cells.contains_key(id),
            } { return Err(*dep_id) }
        }
        // create compute cell id, add it to the dependency_map, with dependency vec as the value
        let id = ComputeCellId(generate_id());
        for source in dependencies {
            let entry = self.dependency_map.entry(*source).or_insert(Vec::new());
            entry.push(id)
        }
        // add the updater fn to the store vector
        let arg_vals = self.get_values(dependencies);
        let initial_val = compute_func(&arg_vals[..]);
        self.compute_cells.insert(
            id,
            ComputeCell::new(initial_val, dependencies, Box::new(compute_func)),
        );
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        Some(*match id {
            CellId::Input(id) => self.input_cells.get(&id)?,
            CellId::Compute(id) => &self.compute_cells.get(&id)?.value,
        })
    }

    fn get_values(&self, ids: &[CellId]) -> Vec<T> {
        ids.iter()
            .map(|dep_id| match dep_id {
                CellId::Input(id) => self.input_cells.get(id).unwrap(),
                CellId::Compute(id) => &self.compute_cells.get(id).unwrap().value,
            })
            .cloned()
            .collect()
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_val: T) -> bool {
        if let Some(v) = self.input_cells.get_mut(&id) {
            *v = new_val;
            true
        } else { false }
    }

    fn update_deps(&mut self, id: CellId) {
        if self.dependency_map.contains_key(&id) {
            let deps = self.dependency_map.get(&id).unwrap();
            for dep_id in deps {
                if self.compute_cells.contains_key(dep_id) {

                }
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<Fm: FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: Fm,
    ) -> Option<CallbackId> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        unimplemented!(
            "Remove the callback identified by the CallbackId {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}

fn generate_id() -> u64 {
    rand::thread_rng().gen_range(0..u64::MAX)
}
