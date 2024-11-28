use pinocchio::program_error::ProgramError;

pub mod account;
pub mod account_state;
pub mod mint;
pub mod multisig;

/// Type alias for fields represented as `COption`.
pub type COption<T> = ([u8; 4], T);

/// Marker trait for types that can cast from a raw pointer.
///
/// It is up to the type implementing this trait to guarantee that the cast is safe,
/// i.e., that the fields of the type are well aligned and there are no padding bytes.
pub trait RawType {
    /// The length of the type.
    ///
    /// This must be equal to the size of each individual field in the type.
    const LEN: usize;
}

/// Trait to represent a type that can be initialized.
pub trait Initializable {
    /// Return `true` if the object is initialized.
    fn is_initialized(&self) -> bool;
}

/// Trait for `RawType`s that can be *viewed* from a byte slice.
pub trait Viewable<T: Initializable + RawType> {
    /// Return a reference for an initialized `T` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `T`.
    #[inline(always)]
    unsafe fn load(bytes: &[u8]) -> Result<&T, ProgramError> {
        Self::load_unchecked(bytes).and_then(|t| {
            // checks if the data is initialized
            if t.is_initialized() {
                Ok(t)
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        })
    }

    /// Return a `T` reference from the given bytes.
    ///
    /// This function does not check if the data is initialized.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `T`.
    #[inline(always)]
    unsafe fn load_unchecked(bytes: &[u8]) -> Result<&T, ProgramError> {
        if bytes.len() != T::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(&*(bytes.as_ptr() as *const T))
    }

    /// Return a mutable reference for an initialized `T` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `T`.
    #[inline(always)]
    unsafe fn load_mut(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
        Self::load_mut_unchecked(bytes).and_then(|t| {
            // checks if the data is initialized
            if t.is_initialized() {
                Ok(t)
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        })
    }

    /// Return a mutable `T` reference from the given bytes.
    ///
    /// This function does not check if the data is initialized.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `T`.
    #[inline(always)]
    unsafe fn load_mut_unchecked(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
        if bytes.len() != T::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(&mut *(bytes.as_mut_ptr() as *mut T))
    }
}
