use crate::error::{Error::InvalidNodeType, *};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
    Internal = 0,
    Leaf = 1,
}
pub struct NodeHeader {
    pub node_type: NodeType,
    pub num_cells: u16,
    pub free_start: u16,
    pub free_end: u16,
    pub next_leaf: u32,
}
pub const HEADER_SIZE: usize = 11;

impl NodeHeader {
    ///Serilize Self to bytes using little-endian conversions
    pub fn write_to(&self, buf: &mut [u8]) {
        buf[0] = self.node_type as u8;
        buf[1..3].copy_from_slice(&self.num_cells.to_le_bytes());
        buf[3..5].copy_from_slice(&self.free_start.to_le_bytes());
        buf[5..7].copy_from_slice(&self.free_end.to_le_bytes());
        buf[7..11].copy_from_slice(&self.next_leaf.to_le_bytes());
    }

    /// returns deserilized node header data
    pub fn read_from(buf: &[u8]) -> Result<Self> {
        let node_type = match buf[0] {
            0 => NodeType::Internal,
            1 => NodeType::Leaf,
            n => return Err(InvalidNodeType(n)),
        };
        Ok(Self {
            node_type,
            num_cells: u16::from_le_bytes([buf[1], buf[2]]),
            free_start: u16::from_le_bytes([buf[3], buf[4]]),
            free_end: u16::from_le_bytes([buf[5], buf[6]]),
            next_leaf: u32::from_le_bytes([buf[7], buf[8], buf[9], buf[10]]),
        })
    }
}
