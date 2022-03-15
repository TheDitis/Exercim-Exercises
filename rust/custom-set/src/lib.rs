
#[derive(Debug, PartialEq)]
pub struct CustomSet<T: PartialEq + PartialOrd + Copy> {
    items: Vec<T>,
}

impl<T> CustomSet<T>
where T: PartialEq + PartialOrd + Copy
{
    /// Create a new CustomSet from an array of items
    pub fn new(input: &[T]) -> Self {
        let mut items: Vec<T> = Vec::new();
        for item in input {
            CustomSet::insort_unique(&mut items, item);
        }
        CustomSet { items }
    }

    /// Check if this set contains a given item
    pub fn contains(&self, item: &T) -> bool {
        // Could be made more efficient with binary search, keeping simple for now
        self.items.iter().any(|existing_item| item == existing_item)
    }

    /// Add an item to the set
    pub fn add(&mut self, item: T) {
        CustomSet::insort_unique(&mut self.items, &item);
    }

    /// Checks if this set contains only items that exist in another set
    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.iter().all(|item| other.contains(item))
    }

    /// Check if this set is empty
    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    /// Checks if there is overlap between this set and that given (false if overlap)
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.items.iter().all(|item| !other.contains(item))
    }

    /// Get a set containing only items that are common between this set and another
    pub fn intersection(&self, other: &Self) -> Self {
        let overlapping_items: Vec<T> = other.items.clone().into_iter()
            .filter(|item| self.contains(item))
            .collect();
        CustomSet::new(&overlapping_items[..])
    }

    /// Get a set containing items that exist in this set, but not that given
    pub fn difference(&self, other: &Self) -> Self {
        let non_overlapping_items: Vec<T> = self.items.clone().into_iter()
            .filter(|item| !other.contains(item))
            .collect();
        CustomSet::new(&non_overlapping_items[..])
    }

    /// Get a set containing all items from this one and that passed
    pub fn union(&self, other: &Self) -> Self {
        let mut joined = self.items.clone();
        joined.extend(&other.items);
        CustomSet::new(&joined[..])
    }

    /// Insert an item into a vec in sorted (descending) position if it doesn't already exist
    /// Note: input vec must be sorted
    fn insort_unique(vec: &mut Vec<T>, new_val: &T) {
        for (i, val) in vec.iter().enumerate() {
            if new_val == val { return; } // prevent duplicates
            if new_val > val {
                vec.insert(i, *new_val);
                return;
            }
        }
        vec.insert(vec.len(), *new_val);
    }
}
