use std::{
    ffi::{c_char, c_void},
    mem
};

use ash::vk;

use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceMemoryCallbacks {
    pub pfn_allocate: AllocateDeviceMemoryFunction,
    pub pfn_free: FreeDeviceMemoryFunction,
    pub user_data: *mut c_void
}

impl Default for DeviceMemoryCallbacks {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VulkanFunctions {
    pub vk_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    pub vk_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    pub vk_get_physical_device_properties: vk::PFN_vkGetPhysicalDeviceProperties,
    pub vk_get_physical_device_memory_properties: vk::PFN_vkGetPhysicalDeviceMemoryProperties,
    pub vk_allocate_memory: vk::PFN_vkAllocateMemory,
    pub vk_free_memory: vk::PFN_vkFreeMemory,
    pub vk_map_memory: vk::PFN_vkMapMemory,
    pub vk_unmap_memory: vk::PFN_vkUnmapMemory,
    pub vk_flush_mapped_memory_ranges: vk::PFN_vkFlushMappedMemoryRanges,
    pub vk_invalidate_mapped_memory_ranges: vk::PFN_vkInvalidateMappedMemoryRanges,
    pub vk_bind_buffer_memory: vk::PFN_vkBindBufferMemory,
    pub vk_bind_image_memory: vk::PFN_vkBindImageMemory,
    pub vk_get_buffer_memory_requirements: vk::PFN_vkGetBufferMemoryRequirements,
    pub vk_get_image_memory_requirements: vk::PFN_vkGetImageMemoryRequirements,
    pub vk_create_buffer: vk::PFN_vkCreateBuffer,
    pub vk_destroy_buffer: vk::PFN_vkDestroyBuffer,
    pub vk_create_image: vk::PFN_vkCreateImage,
    pub vk_destroy_image: vk::PFN_vkDestroyImage,
    pub vk_cmd_copy_buffer: vk::PFN_vkCmdCopyBuffer,
    pub vk_get_buffer_memory_requirements_2_khr: vk::PFN_vkGetBufferMemoryRequirements2,
    pub vk_get_image_memory_requirements_2_kr: vk::PFN_vkGetImageMemoryRequirements2,
    pub vk_bind_buffer_memory_2_khr: vk::PFN_vkBindBufferMemory2,
    pub vk_bind_image_memory_2_khr: vk::PFN_vkBindImageMemory2,
    pub vk_get_physical_device_memory_properties_2_khr: vk::PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub vk_get_device_buffer_memory_requirements: vk::PFN_vkGetDeviceBufferMemoryRequirements,
    pub vk_get_device_image_memory_requirements: vk::PFN_vkGetDeviceImageMemoryRequirements
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocatorCreateInfo {
    pub flags: AllocatorCreateFlags,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
    pub preferred_large_heap_block_size: vk::DeviceSize,
    pub allocation_callbacks: *const vk::AllocationCallbacks,
    pub device_memory_callbacks: *const DeviceMemoryCallbacks,
    pub heap_size_limit: *const vk::DeviceSize,
    pub vulkan_functions: *const VulkanFunctions,
    pub instance: vk::Instance,
    pub vulkan_api_version: u32,
    pub type_external_memory_handle_types: *const vk::ExternalMemoryHandleTypeFlagsKHR
}

impl Default for AllocatorCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct AllocatorInfo {
    pub instance: vk::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Statistics {
    pub block_count: u32,
    pub allocation_count: u32,
    pub block_bytes: vk::DeviceSize,
    pub allocation_bytes: vk::DeviceSize
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DetailedStatistics {
    pub statistics: Statistics,
    pub unused_range_count: u32,
    pub allocation_size_min: vk::DeviceSize,
    pub allocation_size_max: vk::DeviceSize,
    pub unused_range_size_min: vk::DeviceSize,
    pub unused_range_size_max: vk::DeviceSize
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct TotalStatistics {
    pub memory_type: [DetailedStatistics; vk::MAX_MEMORY_TYPES],
    pub memory_heap: [DetailedStatistics; vk::MAX_MEMORY_HEAPS],
    pub total: DetailedStatistics
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Budget {
    pub statistics: Statistics,
    pub usage: vk::DeviceSize,
    pub budget: vk::DeviceSize
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AllocationCreateInfo {
    pub flags: AllocationCreateFlags,
    pub usage: MemoryUsage,
    pub required_flags: vk::MemoryPropertyFlags,
    pub preferred_flags: vk::MemoryPropertyFlags,
    pub memory_type_bits: u32,
    pub pool: Pool,
    pub user_data: *mut c_void,
    pub priority: f32
}

impl Default for AllocationCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PoolCreateInfo {
    pub memory_type_index: u32,
    pub flags: PoolCreateFlags,
    pub block_size: vk::DeviceSize,
    pub min_block_count: usize,
    pub max_block_count: usize,
    pub priority: f32,
    pub min_allocation_alignment: vk::DeviceSize,
    pub memory_allocate_next: *mut c_void
}

impl Default for PoolCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AllocationInfo {
    pub memory_type: u32,
    pub device_memory: vk::DeviceMemory,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub mapped_data: *mut c_void,
    pub user_data: *mut c_void,
    pub name: *const c_char
}

impl Default for AllocationInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DefragmentationInfo {
    pub flags: DefragmentationFlags,
    pub pool: Pool,
    pub max_bytes_per_pass: vk::DeviceSize,
    pub max_allocations_per_pass: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DefragmentationMove {
    pub operation: DefragmentationMoveOperation,
    pub src_allocation: Allocation,
    pub dst_tmp_allocation: Allocation
}

impl Default for DefragmentationMove {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DefragmentationPassMoveInfo {
    pub move_count: u32,
    pub moves: *mut DefragmentationMove
}

impl Default for DefragmentationPassMoveInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DefragmentationStats {
    pub bytes_moved: vk::DeviceSize,
    pub bytes_freed: vk::DeviceSize,
    pub allocations_moved: u32,
    pub device_memory_blocks_freed: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VirtualBlockCreateInfo {
    pub size: vk::DeviceSize,
    pub flags: VirtualBlockCreateFlags,
    pub allocation_callbacks: *const vk::AllocationCallbacks
}

impl Default for VirtualBlockCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VirtualAllocationCreateInfo {
    pub size: vk::DeviceSize,
    pub alignment: vk::DeviceSize,
    pub flags: VirtualAllocationCreateFlags,
    pub user_data: *mut c_void
}

impl Default for VirtualAllocationCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VirtualAllocationInfo {
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub user_data: *mut c_void
}

impl Default for VirtualAllocationInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
