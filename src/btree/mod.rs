use std::collections::binary_heap::Iter;
use std::mem::zeroed;
pub mod nodeheader;
use crate::btree::nodeheader::*;
use crate::error::*;
use crate::pager::metapage::MetaPage;
use crate::pager::*;
pub struct BTree<'a, P: Pager> {
    pager: &'a mut P,
    root: PageNo,
}

impl<'a, P: Pager> BTree<'a, P> {
    ///Create a new btree
    pub fn create(pager: &'a mut P) -> Result<Self> {
        //allocate fresh page for root
        let root_id = pager.allocate_page()?;
        //fill buf with zeros
        let mut buf = Page::zeroed();
        //write node header to buf
        NodeHeader {
            node_type: NodeType::Leaf,
            num_cells: 0,
            free_start: HEADER_SIZE as u16,
            free_end: PAGE_SIZE as u16,
            next_leaf: 0,
        }
        .write_to(&mut buf.as_slice_mut());
        //persist the buf
        pager.write_page(root_id, &buf)?;

        //return
        Ok(Self {
            pager,
            root: root_id,
        })
    }

    ///opens the btree
    pub fn open(pager: &'a mut P, root: PageNo) -> Self {
        Self { pager, root }
    }

    ///return the root node
    pub fn root(&self) -> PageNo {
        self.root
    }

    //insert into btree
    pub fn insert(&mut self, key: i64, payload: &[u8]) -> Result<()> {
        //1.Read root page
        let mut page = self.pager.read_page(self.root)?;

        //2.header
        let mut header = NodeHeader::read_from(page.as_slice())?;

        //3.binary search
        todo!()
    }
    pub fn get(&mut self, key: i64) -> Result<Option<Vec<u8>>> {
        todo!()
    }
    pub fn iter(&mut self) -> Result<Iter<'_, P>> {
        todo!()
    }
}
