use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CanFrame {
    id: u32,
    dlc: u8,
    data: u64,
}

enum CanFrameIdFlags {
    IdeMask = 0x80000000,
    RtrMask = 0x40000000,
    ExtMask = 0x1FFFFFFF,
}

impl CanFrame {
    /// Least significant byte of data attribute corresponds to first byte of data field in CAN message.
    /// Just think about it as a char-array.
    /// Bit order within each byte should not be a concern
    /// (bits not really addressable in a byte-addressable system).
    /// Just think about it as least significant bit in each byte is also
    /// least significant bit in CAN message and at receiver.
    pub fn new(id: u32, ide: bool, rtr: bool, dlc: u8, data: u64) -> Self {
        Self {
            id: id
                | (if ide {
                    CanFrameIdFlags::IdeMask as u32
                } else {
                    0x0u32
                })
                | (if rtr {
                    CanFrameIdFlags::RtrMask as u32
                } else {
                    0x0u32
                }),
            dlc,
            data,
        }
    }

    pub fn key(&self) -> u32 {
        self.id
    }

    #[allow(unused)]
    pub fn get_id(&self) -> u32 {
        self.id & CanFrameIdFlags::ExtMask as u32
    }
    #[allow(unused)]
    pub fn get_ide_flag(&self) -> bool {
        (self.id & CanFrameIdFlags::IdeMask as u32) != 0
    }
    #[allow(unused)]
    pub fn get_rtr_flag(&self) -> bool {
        (self.id & CanFrameIdFlags::RtrMask as u32) != 0
    }
    #[allow(unused)]
    pub fn get_dlc(&self) -> u8 {
        self.dlc
    }
    pub fn get_data_u64(&self) -> u64 {
        self.data
    }
    #[allow(dead_code)]
    pub fn get_data_8u8(&self) -> [u8; 8] {
        unsafe { std::mem::transmute::<u64, [u8; 8]>(self.data) }
    }
}


#[derive(Debug, Clone)]
pub enum CanError {
    #[allow(unused)]
    Io(Arc<std::io::Error>),
    #[allow(unused)]
    Disconnect(String),
    #[allow(unused)]
    Can(u64),
}

impl CanError {
    pub fn erno(&self) -> u64 {
        match &self {
            CanError::Io(_) => u64::MAX,
            CanError::Disconnect(_) => u64::MAX,
            CanError::Can(can_error) => *can_error,
        }
    }
}
