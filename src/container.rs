//! Containers hold bundles of ColSets
//!

use crate::colset::ColSet;

pub struct ColSetContainer {
    pub contents: Vec<ColSet>,
}

// TODO: form_subsets_with_an_added_element
// TODO: form_subsets_with_a_removed_element
