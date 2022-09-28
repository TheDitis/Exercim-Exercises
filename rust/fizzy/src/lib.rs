use std::ops::Rem;

pub struct Matcher<T> {
    pub matcher: Box<dyn Fn(T) -> bool>,
    pub phrase: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static>(matcher: F, phrase: &str) -> Matcher<T> {
        Matcher {
            matcher: Box::new(matcher),
            phrase: phrase.to_string(),
        }
    }
}

#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Copy + ToString + PartialEq + Rem<Output = T>> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |item| {
            let output = self.matchers.iter()
                .filter(|m| (m.matcher)(item))
                .map(|m| m.phrase.clone())
                .collect::<String>();
            if output.is_empty() { item.to_string() } else { output }
        })
    }
}

pub fn fizz_buzz<T: Copy + ToString + From<u8> + PartialEq + Rem<Output = T>>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|item: T| item % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|item: T| item % T::from(5) == T::from(0), "buzz"))
}
