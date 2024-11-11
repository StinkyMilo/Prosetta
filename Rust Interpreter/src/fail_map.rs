use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::Types;
use rangemap::RangeSet;
use std::ops::Range;

#[derive(Debug)]
pub struct FailMap {
    remaps: HashSet<(&'static str, Types)>,
    fails: HashMap<(&'static str, Types), Rc<RefCell<RangeSet<usize>>>>,
}


impl FailMap {
    pub fn new() -> Self {
        Self {
            remaps: HashSet::new(),
            fails: HashMap::new(),
        }
    }
    pub fn insert(&mut self, alias: &'static str, types: Types, range: Range<usize>) {
        for flag in types.iter() {
            self.fails
                .entry((alias, flag))
                .or_insert_with(|| Rc::new(RefCell::new(RangeSet::new())))
                .borrow_mut()
                .insert(range.clone());
        }
    }
    pub fn contains(&self, alias: &'static str, types: Types, location: usize) -> bool {
        self.fails
            .get(&(alias, types))
            .is_some_and(|range| range.borrow_mut().contains(&location))
    }
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}
