use crate::error::*;
pub mod freelist;
pub mod memory;
pub mod metapage;
pub mod pageheader;
pub const PAGE_SIZE: usize = 4096; //Size of each page 4KB
pub type PageNo = u32;

#[derive(Clone)]
pub struct Page {
    data: [u8; PAGE_SIZE],
}

impl Page {
    // GETTER METHODS
    pub fn zeroed() -> Self {
        Self {
            data: [0; PAGE_SIZE],
        }
    }
    /// returns a reference to array of bytes
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    /// returns a reference to array of mut bytes
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // READERS METHODS (liitle endian)
    ///returns an u16
    pub fn read_u16(&self, offset: usize) -> u16 {
        u16::from_le_bytes(self.data[offset..offset + 2].try_into().unwrap())
    }
    ///returns a u32
    pub fn read_u32(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap())
    }

    // WRITERS MTD
    ///writes val(u16) into byte sequence
    pub fn write_u16(&mut self, offset: usize, val: u16) {
        self.data[offset..offset + 2].copy_from_slice(&val.to_le_bytes());
    }
    ///writes val(u32) into byte sequence
    pub fn write_u32(&mut self, offset: usize, val: u32) {
        self.data[offset..offset + 4].copy_from_slice(&val.to_le_bytes());
    }
}

pub trait Pager {
    ///Read a page and return a Result<T,Err> where T is a reference to page and Err is catched errors.
    /// Input:PageNo
    fn read_page(&self, page_no: PageNo) -> Result<&Page>;
    ///Write a page and return a Result<T,Err> where T is a reference to page and Err is catched errors
    /// Input: PageNo,an updated page reference
    fn write_page(&mut self, page_no: PageNo, page: &Page) -> Result<()>;
    ///Allocates a page by reusing a freed page or creating a new page returning a Result<T,Err> where T is PageNo and Err is captured errors
    fn allocate_page(&mut self) -> Result<PageNo>;
    ///Frees a page by removing data from that page return Result<(),Err> where Err is captured errors
    fn free_page(&mut self, page_no: PageNo) -> Result<()>;
    ///returns total amount of pages
    fn page_count(&self) -> u32;

    ///Reads metadata at pageNo == 0
    fn read_meta(&self) -> Result<&Page>;

    ///writes metadata at PageNo ==0
    fn write_meta(&mut self, page: &Page) -> Result<()>;
}
