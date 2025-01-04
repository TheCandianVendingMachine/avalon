use crate::event::error;

#[derive(Debug, Copy, Clone)]
pub enum Entry {
    Bool(bool),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl TryFrom<Entry> for bool {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::Bool(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for f32 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::F32(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for f64 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::F64(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for i8 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::I8(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for i16 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::I16(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for i32 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::I32(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for i64 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::I64(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for i128 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::I128(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for u8 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::U8(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for u16 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::U16(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for u32 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::U32(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for u64 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::U64(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl TryFrom<Entry> for u128 {
    type Error = error::Entry;
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        if let Entry::U128(stored) = value {
            Ok(stored)
        } else {
            Err(error::Entry::TypeMismatch)
        }
    }
}

impl From<bool> for Entry {
    fn from(value: bool) -> Entry {
        Entry::Bool(value)
    }
}

impl From<f32> for Entry {
    fn from(value: f32) -> Entry {
        Entry::F32(value)
    }
}

impl From<f64> for Entry {
    fn from(value: f64) -> Entry {
        Entry::F64(value)
    }
}

impl From<i8> for Entry {
    fn from(value: i8) -> Entry {
        Entry::I8(value)
    }
}

impl From<i16> for Entry {
    fn from(value: i16) -> Entry {
        Entry::I16(value)
    }
}

impl From<i32> for Entry {
    fn from(value: i32) -> Entry {
        Entry::I32(value)
    }
}

impl From<i64> for Entry {
    fn from(value: i64) -> Entry {
        Entry::I64(value)
    }
}

impl From<i128> for Entry {
    fn from(value: i128) -> Entry {
        Entry::I128(value)
    }
}

impl From<u8> for Entry {
    fn from(value: u8) -> Entry {
        Entry::U8(value)
    }
}

impl From<u16> for Entry {
    fn from(value: u16) -> Entry {
        Entry::U16(value)
    }
}

impl From<u32> for Entry {
    fn from(value: u32) -> Entry {
        Entry::U32(value)
    }
}

impl From<u64> for Entry {
    fn from(value: u64) -> Entry {
        Entry::U64(value)
    }
}

impl From<u128> for Entry {
    fn from(value: u128) -> Entry {
        Entry::U128(value)
    }
}
