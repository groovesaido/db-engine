use std::collections::binary_heap::Iter;

use crate::error::*;
use crate::pager::{PageNo, Pager};
pub struct BTree<'a, P: Pager> {
    pager: &'a mut P,
    root: PageNo,
}

impl<'a, P: Pager> BTree<'a, P> {
    pub fn create(pager: &'a mut P) -> Result<Self> {
        todo!()
    }
    pub fn open(pager: &'a mut P, root: PageNo) -> Self {
        todo!()
    }
    pub fn root(&self) -> PageNo {
        todo!()
    }
    pub fn insert(&mut self, key: i64, payload: &[u8]) -> Result<()> {
        todo!()
    }
    pub fn get(&mut self, key: i64) -> Result<Option<Vec<u8>>> {
        todo!()
    }
    pub fn iter(&mut self) -> Result<Iter<'_, P>> {
        todo!()
    }
}
