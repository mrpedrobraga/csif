use super::{errors::ParsingError, SizedPointer};

/// Trait for types which can be pulled from a vector of bytes in a sized fasion,
/// i.e., have constant size within any schema.
pub trait SizedField : Sized {
    const SIZE: usize;

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError>;

    fn csif_into(&self) -> Vec<u8>;
}

/// Trait for types which can be pulled from a vector of bytes, but don't
/// have a fixed size in the schema (instead, the size is stored dynamically)
/// in the serialized buffer.
/// 
/// Sizes like this are stored through a [`SizedPointer`], and have just
/// a little bit of overhead (the cost of fetching, the size of the SizedPointer).
pub trait DynSizedField : Sized {
    fn csif_from(slice: &Vec<u8>, sized_pointer: SizedPointer) -> Result<Self, ParsingError>;

    fn csif_into(&self) -> Vec<u8>;

    /// Pulls a pointer at pointer_index, then pulls a value through that pointer.
    fn csif_from_thru_sized_ref(slice: &Vec<u8>, pointer_index: usize) -> Result<Self, ParsingError> {
        Self::csif_from(slice, SizedPointer::csif_from(slice, pointer_index)?)
    }
}

impl SizedField for bool {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(*slice.get(index).ok_or(ParsingError::OutOfBounds(index..(index+1)))? != 0x0)
    }

    fn csif_into(&self) -> Vec<u8> {
        if *self { Vec::from([0x00]) } else { Vec::from([0xFF]) }
    }
}

impl SizedField for SizedPointer {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            SizedPointer::new(
                u32::csif_from(slice, index)? as usize,
                u32::csif_from(slice, index + 4)? as usize
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
       [self.index.csif_into(), self.size.csif_into()].concat()
    }
}

impl SizedField for u8 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for u16 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for u32 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for usize {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for u64 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for i8 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for i16 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for i32 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for i64 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for f32 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl SizedField for f64 {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn csif_from(slice: &Vec<u8>, index: usize) -> Result<Self, ParsingError> {
        Ok(
            Self::from_le_bytes(
                slice.get(index..(index + Self::SIZE)).ok_or(ParsingError::OutOfBounds(index..(index + Self::SIZE)))?
                .try_into()
                .map_err(|e| ParsingError::FromSliceError(e))?
            )
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}


impl DynSizedField for String {
    fn csif_from(slice: &Vec<u8>, pointer: SizedPointer) -> Result<String, ParsingError> {
        Ok(
            String::from_utf8(
                slice.get(pointer.as_range()).ok_or(ParsingError::OutOfBounds(pointer.as_range()))?.to_vec())
                .map_err(|e| ParsingError::Utf8Error(e.utf8_error()))?
        )
    }

    fn csif_into(&self) -> Vec<u8> {
        self.clone().into_bytes().to_vec()
    }
}
