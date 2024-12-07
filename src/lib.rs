use digest::Digest;
use sha2::Sha256;

struct ColSet {
    feature_vector: Vec<String>,
    colset_hash: String,
    nickname: String,
}

/// Takes an owned vector of strings and returns a hash string of the sorted, deduplicated list
pub fn hash_colset(strings: &Vec<String>) -> String {
    let mut unique_strings = strings.clone();
    unique_strings.sort();
    unique_strings.dedup();

    let joined = unique_strings.join("");

    let hash = Sha256::new().chain_update(joined.as_bytes()).finalize();
    format!("{:x}", hash)
}

fn identical_colset_hash(colset1: &ColSet, colset2: &ColSet) -> bool {
    colset1.colset_hash == colset2.colset_hash
}

/// Constructor for the ColSet Struct that computes the `colset_hash` as a String.
/// Takes a feature_vector and an optional nickname.
/// If the nickname is not given, the last 6 characters of the resulting `colset_hash` will be used as the `nickname`.
impl ColSet {
    fn new(feature_vector: Vec<String>) -> Self {
        let colset_hash = hash_colset(&feature_vector);
        let nickname = colset_hash[colset_hash.len() - 6..].to_string();

        ColSet {
            feature_vector,
            colset_hash,
            nickname,
        }
    }

    fn new_with_nickname(feature_vector: Vec<String>, nickname: String) -> Self {
        let colset_hash: String = hash_colset(&feature_vector);

        ColSet {
            feature_vector,
            colset_hash,
            nickname,
        }
    }

    fn get_nickname(&self) -> &String {
        &self.nickname
    }
}

// get_hashname
// hash_colset
// add_one
// subtract_one

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
