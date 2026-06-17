//! In-memory address book for storing and looking up labeled Stellar addresses.

use std::collections::HashMap;
use std::fmt;

/// Errors that can occur when working with the address book.
#[derive(Debug, PartialEq)]
pub enum AddressBookError {
    /// A label already exists in the book.
    DuplicateLabel,
    /// The provided label is empty.
    EmptyLabel,
    /// The provided address is empty.
    EmptyAddress,
}

impl fmt::Display for AddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressBookError::DuplicateLabel => write!(f, "Label already exists in address book"),
            AddressBookError::EmptyLabel => write!(f, "Label must not be empty"),
            AddressBookError::EmptyAddress => write!(f, "Address must not be empty"),
        }
    }
}

/// An in-memory store that maps human-readable labels to Stellar addresses.
///
/// # Example
/// ```
/// use soroban_toolkit::address_book::AddressBook;
///
/// let mut book = AddressBook::new();
/// book.add("alice", "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG").unwrap();
/// assert_eq!(book.get("alice"), Some("GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG"));
/// ```
#[derive(Debug, Default)]
pub struct AddressBook {
    entries: HashMap<String, String>,
}

impl AddressBook {
    /// Creates a new, empty `AddressBook`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a label → address mapping.
    ///
    /// Returns [`AddressBookError::DuplicateLabel`] if `label` already exists,
    /// [`AddressBookError::EmptyLabel`] if `label` is blank, or
    /// [`AddressBookError::EmptyAddress`] if `address` is blank.
    pub fn add(&mut self, label: &str, address: &str) -> Result<(), AddressBookError> {
        if label.is_empty() {
            return Err(AddressBookError::EmptyLabel);
        }
        if address.is_empty() {
            return Err(AddressBookError::EmptyAddress);
        }
        if self.entries.contains_key(label) {
            return Err(AddressBookError::DuplicateLabel);
        }
        self.entries.insert(label.to_string(), address.to_string());
        Ok(())
    }

    /// Returns the address for `label`, or `None` if not found.
    pub fn get(&self, label: &str) -> Option<&str> {
        self.entries.get(label).map(String::as_str)
    }

    /// Removes the entry for `label`. Returns `true` if it existed.
    pub fn remove(&mut self, label: &str) -> bool {
        self.entries.remove(label).is_some()
    }

    /// Returns all `(label, address)` pairs in unspecified order.
    pub fn list(&self) -> Vec<(&str, &str)> {
        self.entries
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADDR: &str = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";
    const ADDR2: &str = "CCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";

    #[test]
    fn add_and_get() {
        let mut book = AddressBook::new();
        assert!(book.add("alice", ADDR).is_ok());
        assert_eq!(book.get("alice"), Some(ADDR));
    }

    #[test]
    fn get_missing_returns_none() {
        let book = AddressBook::new();
        assert_eq!(book.get("nobody"), None);
    }

    #[test]
    fn duplicate_label_is_error() {
        let mut book = AddressBook::new();
        book.add("alice", ADDR).unwrap();
        assert_eq!(
            book.add("alice", ADDR2),
            Err(AddressBookError::DuplicateLabel)
        );
    }

    #[test]
    fn empty_label_is_error() {
        let mut book = AddressBook::new();
        assert_eq!(book.add("", ADDR), Err(AddressBookError::EmptyLabel));
    }

    #[test]
    fn empty_address_is_error() {
        let mut book = AddressBook::new();
        assert_eq!(book.add("alice", ""), Err(AddressBookError::EmptyAddress));
    }

    #[test]
    fn remove_existing_returns_true() {
        let mut book = AddressBook::new();
        book.add("alice", ADDR).unwrap();
        assert!(book.remove("alice"));
        assert_eq!(book.get("alice"), None);
    }

    #[test]
    fn remove_missing_returns_false() {
        let mut book = AddressBook::new();
        assert!(!book.remove("ghost"));
    }

    #[test]
    fn list_returns_all_entries() {
        let mut book = AddressBook::new();
        book.add("alice", ADDR).unwrap();
        book.add("bob", ADDR2).unwrap();
        let mut entries = book.list();
        entries.sort();
        assert_eq!(entries, vec![("alice", ADDR), ("bob", ADDR2)]);
    }

    #[test]
    fn list_empty_book() {
        let book = AddressBook::new();
        assert!(book.list().is_empty());
    }

    #[test]
    fn add_after_remove_succeeds() {
        let mut book = AddressBook::new();
        book.add("alice", ADDR).unwrap();
        book.remove("alice");
        assert!(book.add("alice", ADDR2).is_ok());
        assert_eq!(book.get("alice"), Some(ADDR2));
    }
}
