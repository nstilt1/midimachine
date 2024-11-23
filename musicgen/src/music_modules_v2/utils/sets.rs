use std::collections::hash_set::{Difference, Intersection, Union};
use std::collections::{HashSet, BTreeSet};
use std::collections::btree_set::{Union as BUnion, Difference as BDifference, Intersection as BIntersection};
use std::hash::{BuildHasher, Hash};

/// Methods for intermediate HashSet operations such as union, difference, and
/// intersection.
pub trait SetOpsCollection {
    type Inner: Eq + Clone;
    type Set;
    /// Converts the intermediate result into a HashSet.
    #[allow(unused)]
    fn to_set(self) -> Self::Set;
    /// Converts the intermediate result into a Vector.
    #[allow(unused)]
    fn to_vec(self) -> Vec<Self::Inner>;
}

macro_rules! impl_hashset_ops_collection {
    ($operation:ident, $set:ty, $bound:ident) => {
        impl<'a, T: Eq + $bound + Clone, S: BuildHasher> SetOpsCollection for $operation<'a, T, S> {
            type Inner = T;
            type Set = $set;
            #[inline]
            fn to_set(self) -> Self::Set {
                self.cloned().collect()
            }
            #[inline]
            fn to_vec(self) -> Vec<Self::Inner> {
                self.cloned().collect()
            }
        }
    };
}

macro_rules! impl_btreeset_ops_collection {
    ($operation:ident, $set:ty, $bound:ident) => {
        impl<'a, T: Eq + Hash + $bound + Clone> SetOpsCollection for $operation<'a, T> {
            type Inner = T;
            type Set = $set;
            #[inline]
            fn to_set(self) -> Self::Set {
                self.cloned().collect()
            }
            #[inline]
            fn to_vec(self) -> Vec<Self::Inner> {
                self.cloned().collect()
            }
        }
    };
}

impl_hashset_ops_collection!(Union, HashSet<T>, Hash);
impl_hashset_ops_collection!(Difference, HashSet<T>, Hash);
impl_hashset_ops_collection!(Intersection, HashSet<T>, Hash);
impl_btreeset_ops_collection!(BUnion, BTreeSet<T>, Ord);
impl_btreeset_ops_collection!(BDifference, BTreeSet<T>, Ord);
impl_btreeset_ops_collection!(BIntersection, BTreeSet<T>, Ord);


/// Math operators for HashSets
pub trait SetMath {
    type Inner: Eq + Hash + Clone;
    fn to_vec(&self) -> Vec<Self::Inner>;
}

impl<'a, S: BuildHasher, T: Eq + Hash + Clone> SetMath for HashSet<T, S> {
    type Inner = T;
    #[inline]
    fn to_vec(&self) -> Vec<Self::Inner> {
        self.iter().cloned().collect()
    }
}

pub trait ToSet {
    type Inner: Eq + Hash + Clone;
    #[allow(unused)]
    fn to_set(&self) -> HashSet<Self::Inner>;
}

impl<T: Eq + Hash + Clone> ToSet for [T] {
    type Inner = T;
    #[inline]
    fn to_set(&self) -> HashSet<Self::Inner> {
        self.iter().cloned().collect()
    }
}

// Tests to demonstrate usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashsets() {
        let set: HashSet<i32> = HashSet::from([1,2,3,4]);
        let set2: HashSet<i32> = HashSet::from([4,5,6,7,8]);

        let intersect = set.intersection(&set2).to_set();
        assert_eq!(intersect, HashSet::from([4]));

        let intersect_vec = set.intersection(&set2).to_vec();
        assert_eq!(intersect_vec, vec![4]);

        let union = set.union(&set2).to_set();
        assert_eq!(union, [1,2,3,4,5,6,7,8].to_set());

        let mut union_vec = set.union(&set2).to_vec();
        union_vec.sort();
        assert_eq!(union_vec, vec![1,2,3,4,5,6,7,8]);

        let diff_1 = set.difference(&set2).to_set();
        assert_eq!(diff_1, [1,2,3].to_set());

        let mut diff_1_vec = set.difference(&set2).to_vec();
        diff_1_vec.sort();
        assert_eq!(diff_1_vec, vec![1,2,3]);
    }
}