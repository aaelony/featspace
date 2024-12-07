//! Utilities for sets of features.

use digest::Digest;
use sha2::Sha256;

#[derive(Debug)]
pub struct ColSet {
    pub feature_vector: Vec<String>,
    pub colset_hash: String,
    pub nickname: String,
}

pub struct ColSetContainer {
    pub contents: Vec<ColSet>,
}

/// Get a string hash that uniquely identifies a set of feature names.
/// Takes an owned vector of strings and returns a hash string of the sorted, dmake eduplicated list
/// Example:
/// ```
/// use featspace::hash_colset;
/// let colset1: Vec<String> = ["foo", "bar", "baz"]
///    .iter()
///    .map(|&s| s.to_string())
///    .collect();
/// let colset1_hash = hash_colset(&colset1.to_vec());
/// ```
///
pub fn hash_colset(strings: &Vec<String>) -> String {
    let mut unique_strings = strings.clone();
    unique_strings.sort();
    unique_strings.dedup();
    let joined = unique_strings.join("");
    let hash = Sha256::new().chain_update(joined.as_bytes()).finalize();
    format!("{:x}", hash)
}

/// Create a new colset by adding a new feature string.
/// Example:
/// ```
/// use featspace::{ColSet,add_one_to_colset};
/// let colset1: Vec<String> = ["foo", "bar", "baz"]
///    .iter()
///    .map(|&s| s.to_string())
///    .collect();
/// let orig_colset = ColSet::new(colset1);
/// let new_feature = "qux".to_string();
/// let new_colset = add_one_to_colset(&orig_colset, &new_feature);
/// println!("{:?}", new_colset);
///
/// ```
pub fn add_one_to_colset(parent: &ColSet, new_feature: &String) -> ColSet {
    let mut new_feature_vector = parent.feature_vector.clone();
    new_feature_vector.push(new_feature.clone());
    let new_colset = ColSet::new(new_feature_vector.clone());
    new_colset
}

// subtract_one

/// Test 2 if ColSet objects have the same string hash
pub fn identical_colset_hash(colset1: &ColSet, colset2: &ColSet) -> bool {
    colset1.colset_hash == colset2.colset_hash
}

/// Constructor for the ColSet Struct that computes the `colset_hash` as a String.
/// Takes a feature_vector and an optional nickname.
/// If the nickname is not given, the last 6 characters of the resulting `colset_hash` will be used as the `nickname`.
impl ColSet {
    pub fn new(feature_vector: Vec<String>) -> Self {
        let colset_hash = hash_colset(&feature_vector);
        let nickname = colset_hash[colset_hash.len() - 6..].to_string();

        ColSet {
            feature_vector,
            colset_hash,
            nickname,
        }
    }

    pub fn new_with_nickname(feature_vector: Vec<String>, nickname: String) -> Self {
        let colset_hash: String = hash_colset(&feature_vector);

        ColSet {
            feature_vector,
            colset_hash,
            nickname,
        }
    }

    pub fn get_nickname(&self) -> &String {
        &self.nickname
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// colset1 and colset2 have the same elements but in a different order.
    /// This test ensures that the same hash is returned by `hash_colset` regardless of sort order.
    fn returned_hash_is_invariant_to_vector_sort_order() {
        let colset1: Vec<String> = ["foo", "bar", "baz"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        let colset2: Vec<String> = ["bar", "foo", "baz"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        let colset1_hash = hash_colset(&colset1.to_vec());
        let colset2_hash = hash_colset(&colset2.to_vec());

        let fset1 = ColSet::new(colset1);
        let fset2 = ColSet::new_with_nickname(colset2, "the_good_one".to_string());

        assert_eq!(colset1_hash, colset2_hash);
        assert_eq!(identical_colset_hash(&fset1, &fset2), true);
    }
}
