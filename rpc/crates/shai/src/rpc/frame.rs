#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(u16);

impl MessageId {
    pub const RESPONSE_MASK: u16 = 0x8000;

    pub const fn from_u16(id: u16) -> Self {
        Self(id)
    }

    pub const fn request(id: u16) -> Self {
        assert!(id < Self::RESPONSE_MASK);
        Self(id)
    }

    pub const fn response(id: u16) -> Self {
        assert!(id < Self::RESPONSE_MASK);
        Self(id | Self::RESPONSE_MASK)
    }

    pub const fn is_response(self) -> bool {
        self.0 & Self::RESPONSE_MASK != 0
    }

    pub const fn into_response(self) -> Self {
        Self(self.0 | Self::RESPONSE_MASK)
    }

    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(u8);

impl Flags {
    pub const EMPTY: Self = Self(0);

    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn from_u8(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Ok = 0,
    NotFound = 1,
    DecodeError = 2,
    EncodeError = 3,
    InternalError = 4,
    Unauthorized = 5,
    Unknown = 255,
}

impl Status {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::NotFound,
            2 => Self::DecodeError,
            3 => Self::EncodeError,
            4 => Self::InternalError,
            5 => Self::Unauthorized,
            _ => Self::Unknown,
        }
    }
}
