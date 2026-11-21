use crate::error::*;
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
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // READERS METHODS (liitle endian)
    pub fn read_u16(&self, offset: usize) -> u16 {
        u16::from_le_bytes(self.data[offset..offset + 2].try_into().unwrap())
    }
    pub fn read_u32(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.data[offset..offset + 2].try_into().unwrap())
    }

    // WRITERS MTD
    pub fn write_u16(&mut self, offset: usize, val: u16) {
        self.data[offset..offset + 2].copy_from_slice(&val.to_le_bytes());
    }
    pub fn write_u32(&mut self, offset: usize, val: u32) {
        self.data[offset..offset + 4].copy_from_slice(&val.to_le_bytes());
    }
}

pub trait Pager {
    //READ a page
    fn read_page(&self, page_no: PageNo) -> Result<&Page>;
    //mutable ref to write
    fn write_page(&mut self, page_no: PageNo) -> Result<&mut Page>;
    //allocate new page
    fn allocate_page(&mut self) -> Result<PageNo>;
    //free a page to the freelist
    fn free_page(&mut self, page_no: PageNo) -> Result<()>;
    //count total no of pages
    fn page_count(&self) -> u32;
}
