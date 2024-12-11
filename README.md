# `featspace`

This crate is a work in progress that offers tools to manipulate _containers_ of feature sets, ostensibly to be used declaratively in model training efforts.

Eventually it will be a Rust library for feature space, where a feature is a string name that refers to a dataset column, normally the name of a predictor variable, a.k.a. _feature_.


## Rationale

Why?

Before the age of LLMs with ridiculously large numbers of parameters, models were smaller and the combinatorics were not prohibitive for statisticians to strive to understand which combinations of features improved model accuracy more than others.

By examining models iteratively, sets of features could be investigated and compared for accuracy with the goal of finding the set of features for the _most parsimonious model_.

It also serves as a toy project in the amazing Rust language.
