// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-rustfix
#![allow(unused_mut)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

struct GetFalsePositive {
    arr: [u32; 3],
}

impl GetFalsePositive {
    fn get(&self, pos: usize) -> Option<&u32> {
        self.arr.get(pos)
    }
    fn get_mut(&mut self, pos: usize) -> Option<&mut u32> {
        self.arr.get_mut(pos)
    }
}

fn main() {
    let mut boxed_slice: Box<[u8]> = Box::new([0, 1, 2, 3]);
    let mut some_slice = &mut [0, 1, 2, 3];
    let mut some_vec = vec![0, 1, 2, 3];
    let mut some_vecdeque: VecDeque<_> = some_vec.iter().cloned().collect();
    let mut some_hashmap: HashMap<u8, char> = HashMap::from_iter(vec![(1, 'a'), (2, 'b')]);
    let mut some_btreemap: BTreeMap<u8, char> = BTreeMap::from_iter(vec![(1, 'a'), (2, 'b')]);
    let mut false_positive = GetFalsePositive { arr: [0, 1, 2] };

    {
        // Test `get().unwrap()`
        let _ = boxed_slice.get(1).unwrap();
        let _ = some_slice.get(0).unwrap();
        let _ = some_vec.get(0).unwrap();
        let _ = some_vecdeque.get(0).unwrap();
        let _ = some_hashmap.get(&1).unwrap();
        let _ = some_btreemap.get(&1).unwrap();
        let _ = false_positive.get(0).unwrap();
        // Test with deref
        let _: u8 = *boxed_slice.get(1).unwrap();
    }

    {
        // Test `get_mut().unwrap()`
        *boxed_slice.get_mut(0).unwrap() = 1;
        *some_slice.get_mut(0).unwrap() = 1;
        *some_vec.get_mut(0).unwrap() = 1;
        *some_vecdeque.get_mut(0).unwrap() = 1;
        // Check false positives
        *some_hashmap.get_mut(&1).unwrap() = 'b';
        *some_btreemap.get_mut(&1).unwrap() = 'b';
        *false_positive.get_mut(0).unwrap() = 1;
    }

    {
        // Test `get().unwrap().foo()` and `get_mut().unwrap().bar()`
        let _ = some_vec.get(0..1).unwrap().to_vec();
        let _ = some_vec.get_mut(0..1).unwrap().to_vec();
    }
}
