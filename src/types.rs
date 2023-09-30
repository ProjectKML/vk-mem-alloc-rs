use std::ffi::c_void;

use ash::vk;

use crate::utils::{define_handle, define_non_dispatchable_handle};

define_handle!(Allocator);
define_handle!(Pool);
define_handle!(Allocation);
define_handle!(DefragmentationContext);
define_non_dispatchable_handle!(VirtualAllocation);
define_handle!(VirtualBlock);

pub type PfnAllocateDeviceMemoryFunction = Option<unsafe extern "C" fn(allocator: Allocator, memory_type: u32, memory: vk::DeviceMemory, size: vk::DeviceSize, user_data: *mut c_void)>;
pub type PfnFreeDeviceMemoryFunction = Option<unsafe extern "C" fn(allocator: Allocator, memory_type: u32, memory: vk::DeviceMemory, size: vk::DeviceSize, user_data: *mut c_void)>;

pub type PfnCheckDefragmentationBreakFunction = Option<unsafe extern "C" fn(pUserData: *mut c_void) -> vk::Bool32>;
