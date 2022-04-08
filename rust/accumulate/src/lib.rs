/// What should the type of _function be?
pub fn map<F: FnMut(I) -> O, I, O>(input: Vec<I>, mut func: F) -> Vec<O> {
    let mut output: Vec<O> = Vec::new();
    for v in input {
        output.push(func(v))
    }
    output
}
