//! A datastructure for holding key map entries
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Node<Value: Debug> {
    label: u8,
    children: Vec<Node<Value>>,
    value: Option<Value>,
}

impl<Value: Debug> Node<Value> {
    fn new(label: u8) -> Self {
        Self {
            label,
            children: Vec::new(),
            value: None,
        }
    }

    fn insert(&mut self, key: &[u8], value: Value) {
        if key.is_empty() {
            // We've reached the leaf
            self.value = Some(value);
            return;
        }
        match self.children.binary_search_by(|node| node.label.cmp(&key[0])) {
            Ok(idx) => {
                self.children[idx].insert(&key[1..], value);
            },
            Err(idx) => {
                self.children.insert(idx, Node::new(key[0]));
                self.children[idx].insert(&key[1..], value);
            },
        }
    }

    fn lookup(&self, key: &[u8], depth: usize) -> NodeFind<&Value> {
        if key.is_empty() {
            // We've matched the maximum extent of the input key.
            if self.children.is_empty() {
                match self.value.as_ref() {
                    Some(value) => {
                        // An unambiguous match for the entire input
                        return NodeFind::Exact(depth, value);
                    },
                    None => panic!("Node has no children and no value!?"),
                }
            }
            return match self.value.as_ref() {
                Some(value) => NodeFind::AmbiguousMatch(depth, value),
                None => NodeFind::AmbiguousBackTrack,
            };
        }

        match self.children.binary_search_by(|node| node.label.cmp(&key[0])) {
            Ok(idx) => {
                match self.children[idx].lookup(&key[1..], depth + 1) {
                    NodeFind::AmbiguousBackTrack => {
                        // The child didn't have an exact match, so check
                        // whether we do
                        match self.value.as_ref() {
                            Some(value) => NodeFind::AmbiguousMatch(depth, value),
                            None => NodeFind::AmbiguousBackTrack,
                        }
                    },
                    result => result,
                }
            },
            Err(_) => {
                if depth == 0 {
                    NodeFind::None
                } else {
                    match self.value.as_ref() {
                        Some(value) => NodeFind::Exact(depth, value),
                        None => NodeFind::AmbiguousBackTrack,
                    }
                }
            },
        }
    }
}

/// Internal lookup disposition
enum NodeFind<Value> {
    /// No possible matches
    None,
    /// Found an exact match. (match-len, value)
    Exact(usize, Value),
    /// Didn't find an exact match at the full extent of the key,
    /// so ask the upper layer to back track to find a partial.
    AmbiguousBackTrack,
    /// After backtracking, found a prefix match, but we know that
    /// there might be a more specific match given more data. (match-len,
    /// value).
    AmbiguousMatch(usize, Value),
}

/// Holds the result of a lookup operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Found<Value> {
    /// There are definitively no possible matches
    None,
    /// We found an unambiguous match.
    /// The data is (length-of-match, value)
    Exact(usize, Value),
    /// We found a match, but there are other longer matches
    /// that are possible.  Ideally we'd accumulate more data
    /// to know for sure.
    /// The data is (length-of-shortest-match, value)
    Ambiguous(usize, Value),
    /// If we had more data, we might match something
    NeedData,
}

/// The `KeyMap` struct is intended to hold the text sequences
/// generated by unix terminal programs.  Those sequences have
/// overlapping/ambiguous meaning which requires having more
/// data to correctly interpret the sequence.
/// The `lookup` operation returns an enum describing the
/// confidence of a match rather than a simple map lookup.
#[derive(Debug, Clone)]
pub struct KeyMap<Value: Debug + Clone> {
    root: Node<Value>,
}

impl<Value: Debug + Clone> Default for KeyMap<Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Value: Debug + Clone> KeyMap<Value> {
    pub fn new() -> Self {
        Self { root: Node::new(0) }
    }

    /// Insert a value into the keymap
    pub fn insert<K: AsRef<[u8]>>(&mut self, key: K, value: Value) {
        self.root.insert(key.as_ref(), value);
    }

    /// Perform a lookup for `key`.
    /// `key` can be a string consisting of a sequence of bytes.
    /// The `lookup` operation considers the prefix of `key` and searches
    /// for a match.
    ///
    /// If `Found::None` is returned then the prefix for key has no matching
    /// keymap entry.
    ///
    /// If `Found::Exact` is returned then the returned value informs the caller
    /// of the length of the key that was matched; the remainder of the key
    /// was not considered and is something that should be considered again
    /// in a subsequent lookup operation.
    ///
    /// If `Found::Ambiguous` is returned then the key matched a valid
    /// entry (which is returned as the value), but there is at least one
    /// other entry that could match if more data were available.  If the caller
    /// knows that no more data is available immediately then it may be valid
    /// to treat this result as equivalent to `Found::Exact`.  The intended
    /// use for this variant is to handle the case where a sequence straddles
    /// a buffer boundary (eg: fixed size buffer receives a partial sequence,
    /// and the remainder is immediately available on the next read) without
    /// misinterpreting the read data.
    ///
    /// If `Found::NeedData` is returned it indicates that `key` is too short
    /// to determine a match.  The purpose is similar to the `Found::Ambiguous`
    /// case; if the caller knows that no more data is available this can be
    /// treated as `Found::None`, but otherwise it would be best to read more
    /// data from the stream and retry with a longer input.
    pub fn lookup<S: AsRef<[u8]>>(&self, key: S) -> Found<Value> {
        match self.root.lookup(key.as_ref(), 0) {
            NodeFind::None => Found::None,
            NodeFind::AmbiguousBackTrack => Found::NeedData,
            NodeFind::Exact(depth, value) => Found::Exact(depth, value.clone()),
            NodeFind::AmbiguousMatch(depth, value) => Found::Ambiguous(depth, value.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lookup_empty() {
        let km: KeyMap<bool> = KeyMap::new();
        assert_eq!(km.lookup("boo"), Found::None);
    }

    #[test]
    fn lookup() {
        let mut km = KeyMap::new();
        km.insert("boa", true);
        km.insert("boo", true);
        km.insert("boom", false);
        assert_eq!(km.lookup("b"), Found::NeedData);
        assert_eq!(km.lookup("bo"), Found::NeedData);
        assert_eq!(km.lookup("boa"), Found::Exact(3, true),);
        assert_eq!(km.lookup("boo"), Found::Ambiguous(3, true),);
        assert_eq!(km.lookup("boom"), Found::Exact(4, false),);
        assert_eq!(km.lookup("boom!"), Found::Exact(4, false),);
    }

    #[test]
    fn sequence() {
        let mut km = KeyMap::new();
        km.insert("\x03", true);
        km.insert("\x27", true);
        km.insert("\x03XYZ", true);
        assert_eq!(km.lookup("\x03"), Found::Ambiguous(1, true),);
        assert_eq!(km.lookup("\x03foo"), Found::Exact(1, true),);
        assert_eq!(km.lookup("\x03X"), Found::Ambiguous(1, true),);
    }
}
