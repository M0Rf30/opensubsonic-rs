// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Crate-private builder for endpoint query parameters.

/// An ordered list of query parameters; keys may repeat.
#[derive(Debug, Default, Clone)]
pub(crate) struct Params {
    pairs: Vec<(&'static str, String)>,
}

impl Params {
    /// Create an empty parameter list.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Always append `key=value`.
    pub(crate) fn with(mut self, key: &'static str, value: impl ToString) -> Self {
        self.pairs.push((key, value.to_string()));
        self
    }

    /// Append `key=value` only when `value` is `Some`.
    pub(crate) fn with_opt<T: ToString>(mut self, key: &'static str, value: Option<T>) -> Self {
        if let Some(v) = value {
            self.pairs.push((key, v.to_string()));
        }
        self
    }

    /// Append one pair per item (repeated key, e.g. `id=1&id=2`).
    pub(crate) fn with_all<T: ToString>(
        mut self,
        key: &'static str,
        values: impl IntoIterator<Item = T>,
    ) -> Self {
        self.pairs
            .extend(values.into_iter().map(|v| (key, v.to_string())));
        self
    }

    /// Iterate over `(key, value)` pairs in insertion order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.pairs.iter().map(|(k, v)| (*k, v.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(p: &Params) -> Vec<(&str, &str)> {
        p.iter().collect()
    }

    #[test]
    fn empty() {
        assert!(collect(&Params::new()).is_empty());
    }

    #[test]
    fn with_appends_in_order() {
        let p = Params::new().with("id", 42).with("name", "x");
        assert_eq!(collect(&p), vec![("id", "42"), ("name", "x")]);
    }

    #[test]
    fn with_opt_skips_none() {
        let p = Params::new()
            .with_opt("a", Some(1))
            .with_opt("b", None::<i32>)
            .with_opt("c", Some(true));
        assert_eq!(collect(&p), vec![("a", "1"), ("c", "true")]);
    }

    #[test]
    fn with_all_repeats_key() {
        let p = Params::new().with_all("id", ["1", "2", "3"]);
        assert_eq!(collect(&p), vec![("id", "1"), ("id", "2"), ("id", "3")]);
        let p = Params::new().with_all("id", Vec::<String>::new());
        assert!(collect(&p).is_empty());
    }
}
