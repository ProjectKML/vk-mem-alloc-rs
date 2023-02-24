use bitflags::bitflags;

use crate::ffi;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AllocatorCreateFlags : u32 {
        const EXTERNALLY_SYNCHRONIZED = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT as _;
        const KHR_DEDICATED_ALLOCATION = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT as _;
        const KHR_BIND_MEMORY2 = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT as _;
        const EXT_MEMORY_BUDGET = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXT_MEMORY_BUDGET_BIT as _;
        const AMD_DEVICE_COHERENT_MEMORY = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_AMD_DEVICE_COHERENT_MEMORY_BIT as _;
        const BUFFER_DEVICE_ADDRESS = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_BUFFER_DEVICE_ADDRESS_BIT as _;
        const EXT_MEMORY_PRIORITY = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT as _;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryUsage(pub u32);

impl MemoryUsage {
    pub const UNKNOWN: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_UNKNOWN as _);
    #[deprecated]
    pub const GPU_ONLY: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY as _);
    #[deprecated]
    pub const CPU_ONLY: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY as _);
    #[deprecated]
    pub const CPU_TO_GPU: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_TO_GPU as _);
    #[deprecated]
    pub const GPU_TO_CPU: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_TO_CPU as _);
    #[deprecated]
    pub const CPU_COPY: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY as _);
    #[deprecated]
    pub const GPU_LAZILY_ALLOCATED: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_LAZILY_ALLOCATED as _);
    pub const AUTO: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_AUTO as _);
    pub const AUTO_PREFER_DEVICE: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_AUTO_PREFER_DEVICE as _);
    pub const AUTO_PREFER_HOST: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_AUTO_PREFER_HOST as _);
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AllocationCreateFlags : u32 {
        const DEDICATED_MEMORY = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT as _;
        const NEVER_ALLOCATE = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT as _;
        const MAPPED = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_MAPPED_BIT as _;
        const USER_DATA_COPY_STRING = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT as _;
        const UPPER_ADDRESS = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT as _;
        const DONT_BIND = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DONT_BIND_BIT as _;
        const WITHIN_BUDGET = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_WITHIN_BUDGET_BIT as _;
        const CAN_ALIAS = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_CAN_ALIAS_BIT as _;
        const HOST_ACCESS_SEQUENTIAL_WRITE = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT as _;
        const HOST_ACCESS_RANDOM = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT as _;
        const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT as _;
        const STRATEGY_MIN_MEMORY = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT as _;
        const STRATEGY_MIN_TIME = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT as _;
        const STRATEGY_MIN_OFFSET  = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT as _;
        const STRATEGY_BEST_FIT = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT as _;
        const STRATEGY_FIRST_FIT = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT as _;
        const STRATEGY_MASK = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MASK as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct PoolCreateFlags: u32 {
        const IGNORE_BUFFER_IMAGE_GRANULARITY = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT as _;
        const LINEAR_ALGORITHM = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT as _;
        const ALGORITHM_MASK = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_ALGORITHM_MASK as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct DefragmentationFlags: u32 {
        const ALGORITHM_FAST = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FAST_BIT as _;
        const ALGORITHM_BALANCED = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_BALANCED_BIT as _;
        const ALGORITHM_FULL = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FULL_BIT as _;
        const ALGORITHM_EXTENSIVE = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_EXTENSIVE_BIT as _;
        const ALGORITHM_MASK = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_MASK as _;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DefragmentationMoveOperation(i32);

impl DefragmentationMoveOperation {
    pub const COPY: Self = Self(ffi::VmaDefragmentationMoveOperation_VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY as _);
    pub const IGNORE: Self = Self(ffi::VmaDefragmentationMoveOperation_VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE as _);
    pub const DESTROY: Self = Self(ffi::VmaDefragmentationMoveOperation_VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY as _);
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct VirtualBlockCreateFlags: u32 {
        const LINEAR_ALGORITHM = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT as _;
        const ALGORITHM_MASK = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_MASK as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct VirtualAllocationCreateFlags: u32 {
        const UPPER_ADDRESS = ffi::VmaVirtualAllocationCreateFlagBits_VMA_VIRTUAL_ALLOCATION_CREATE_UPPER_ADDRESS_BIT as _;
        const STRATEGY_MIN_MEMORY = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT as _;
        const STRATEGY_MIN_TIME = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT as _;
        const STRATEGY_MIN_OFFSET = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT as _;
        const STRATEGY_MASK = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MASK as _;
    }
}
