use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{ReturnType, Types};
use rangemap::RangeSet;
use std::ops::Range;

#[derive(Debug)]
pub struct FailMap {
    fails: HashMap<ReturnType, RangeSet<usize>>,
}

impl FailMap {
    pub fn new() -> Self {
        Self {
            fails: HashMap::new(),
        }
    }
    pub fn insert(&mut self, types: Types, range: Range<usize>) {
        if range.len() != 0 {
            for return_type in types.get_iter() {
                self.fails
                    .entry(return_type)
                    .or_insert_with(|| RangeSet::new())
                    .insert(range.clone());
            }
        }
    }
    /// does the failmap wholy contain the current types
    pub fn contains(&self, types: Types, location: usize) -> bool {
        for return_type in types.get_iter() {
            if !self
                .fails
                .get(&return_type)
                .is_some_and(|range| range.contains(&location))
            {
                return false;
            }
        }
        return true;
    }
    // pub fn remove(&mut self, alias: &'static str) {
    //     for flag in Types::all().iter() {
    //         self.fails.remove(&(alias, flag));
    //     }
    // }
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}
