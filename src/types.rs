use std::ffi::c_void;

use ash::vk;

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

define_handle!(Allocator);
define_handle!(Pool);
define_handle!(Allocation);
define_handle!(DefragmentationContext);
define_non_dispatchable_handle!(VirtualAllocation);
define_handle!(VirtualBlock);

pub type AllocateDeviceMemoryFunction = Option<unsafe extern "C" fn(allocator: Allocator, memory_type: u32, memory: vk::DeviceMemory, size: vk::DeviceSize, user_data: *mut c_void)>;
pub type FreeDeviceMemoryFunction = Option<unsafe extern "C" fn(allocator: Allocator, memory_type: u32, memory: vk::DeviceMemory, size: vk::DeviceSize, user_data: *mut c_void)>;
