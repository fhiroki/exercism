use std::ops::Rem;

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(_matcher: fn(T) -> bool, _subs: S) -> Matcher<T> {
        Self {
            matcher: _matcher,
            subs: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        _iter.map(move |x| {
            let s: String = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(x.clone()))
                .map(|m| m.subs.clone())
                .collect();
            if s.is_empty() {
                x.to_string()
            } else {
                s
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + ToString + PartialEq + From<u8> + Clone,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
