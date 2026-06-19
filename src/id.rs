use core::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct StandardId(pub(crate)u16);

impl StandardId {
    pub const MAX: u16 = 0x7FF;
    
    pub const fn new(raw: u16) -> Option<Self> {
        if raw <= Self::MAX {
            Some(Self(raw))
        } else {
            None
        }
    }

    pub const fn as_raw(self) -> u16 {
        self.0
    }
}

impl Debug for StandardId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:>03X}", self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ExtendedId(pub(crate)u32);

impl ExtendedId {
    pub const MAX: u32 = 0x1FFF_FFFF;
    
    pub const fn new(raw: u32) -> Option<Self> {
        if raw <= Self::MAX {
            Some(Self(raw))
        } else {
            None
        }
    }

    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl Debug for ExtendedId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:>08X}", self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CanId {
    Standard(StandardId),
    Extended(ExtendedId)
}

impl Debug for CanId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Standard(id) => write!(f, "{:?}", id),
            Self::Extended(id) => write!(f, "{:?}", id)
        }
    }
}

impl CanId {
    pub const fn is_standard(&self) -> bool {
        matches!(self, CanId::Standard(_))
    }

    pub const fn is_extended(&self) -> bool {
        !self.is_standard()
    }
}

impl From<StandardId> for CanId {
    fn from(id: StandardId) -> Self {
        CanId::Standard(id)
    }
}

impl From<ExtendedId> for CanId {
    fn from(id: ExtendedId) -> Self {
        CanId::Extended(id)
    }
}
