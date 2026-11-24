use crate::pager::PageNo;

#[derive(Default)]
pub struct MetaPage {
    pub magic: [u8; 8],
    pub page_size: u32,
    pub root: PageNo,
    pub freelist_head: PageNo,
}
impl MetaPage {
    ///create magic array filled with 0 [0;8]
    pub fn magiczeroed() -> [u8; 8] {
        [0; 8]
    }
    //serialize to bytes
    pub fn write_to(&self, buf: &mut [u8]) {
        buf[0..8].copy_from_slice(&self.magic);
        buf[8..12].copy_from_slice(&self.page_size.to_le_bytes());
        buf[12..16].copy_from_slice(&self.root.to_le_bytes());
        buf[16..20].copy_from_slice(&self.freelist_head.to_le_bytes());
    }
}
