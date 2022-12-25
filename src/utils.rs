macro_rules! define_handle {
    ($name: ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(*mut u8);

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::null()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            #[inline]
            pub fn as_raw(self) -> u64 {
                self.0 as u64
            }

            #[inline]
            pub fn from_raw(x: u64) -> Self {
                Self(x as _)
            }

            #[inline]
            pub const fn null() -> Self {
                Self(::std::ptr::null_mut())
            }
        }

        impl ::std::fmt::Pointer for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Pointer::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}

pub(crate) use define_handle;

macro_rules! define_non_dispatchable_handle {
    ($name: ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(u64);

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::null()
            }
        }

        impl $name {
            #[inline]
            pub fn as_raw(self) -> u64 {
                self.0
            }

            #[inline]
            pub fn from_raw(x: u64) -> Self {
                Self(x)
            }

            #[inline]
            pub const fn null() -> Self {
                Self(0)
            }
        }

        impl ::std::fmt::Pointer for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}

pub(crate) use define_non_dispatchable_handle;

macro_rules! assert_size_and_align {
    ($type_: ty, $ffi_type: ty) => {
        ::static_assertions::assert_eq_size!($type_, $ffi_type);
        ::static_assertions::assert_eq_align!($type_, $ffi_type);
    };
}

pub(crate) use assert_size_and_align;
