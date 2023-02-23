use bitflags::bitflags;

use crate::ffi;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AllocatorCreateFlags : i32 {
        const EXTERNALLY_SYNCHRONIZED = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT;
        const KHR_DEDICATED_ALLOCATION = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT;
        const KHR_BIND_MEMORY2 = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT;
        const EXT_MEMORY_BUDGET = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXT_MEMORY_BUDGET_BIT;
        const AMD_DEVICE_COHERENT_MEMORY = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_AMD_DEVICE_COHERENT_MEMORY_BIT;
        const BUFFER_DEVICE_ADDRESS = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_BUFFER_DEVICE_ADDRESS_BIT;
        const EXT_MEMORY_PRIORITY = ffi::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryUsage(pub i32);

impl MemoryUsage {
    pub const UNKNOWN: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_UNKNOWN);
    #[deprecated]
    pub const GPU_ONLY: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY);
    #[deprecated]
    pub const CPU_ONLY: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY);
    #[deprecated]
    pub const CPU_TO_GPU: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_TO_GPU);
    #[deprecated]
    pub const GPU_TO_CPU: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_TO_CPU);
    #[deprecated]
    pub const CPU_COPY: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY);
    #[deprecated]
    pub const GPU_LAZILY_ALLOCATED: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_LAZILY_ALLOCATED);
    pub const AUTO: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_AUTO);
    pub const AUTO_PREFER_DEVICE: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_AUTO_PREFER_DEVICE);
    pub const AUTO_PREFER_HOST: Self = Self(ffi::VmaMemoryUsage_VMA_MEMORY_USAGE_AUTO_PREFER_HOST);
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AllocationCreateFlags : i32 {
        const DEDICATED_MEMORY = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT;
        const NEVER_ALLOCATE = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT;
        const MAPPED = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_MAPPED_BIT;
        const USER_DATA_COPY_STRING = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT;
        const UPPER_ADDRESS = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT;
        const DONT_BIND = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DONT_BIND_BIT;
        const WITHIN_BUDGET = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_WITHIN_BUDGET_BIT;
        const CAN_ALIAS = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_CAN_ALIAS_BIT;
        const HOST_ACCESS_SEQUENTIAL_WRITE = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT;
        const HOST_ACCESS_RANDOM = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT;
        const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT;
        const STRATEGY_MIN_MEMORY = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT;
        const STRATEGY_MIN_TIME = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT;
        const STRATEGY_MIN_OFFSET  = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT;
        const STRATEGY_BEST_FIT = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT;
        const STRATEGY_FIRST_FIT = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT;
        const STRATEGY_MASK = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MASK;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct PoolCreateFlags: i32 {
        const IGNORE_BUFFER_IMAGE_GRANULARITY = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT;
        const LINEAR_ALGORITHM = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT;
        const ALGORITHM_MASK = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_ALGORITHM_MASK;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct DefragmentationFlags: i32 {
        const ALGORITHM_FAST = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FAST_BIT;
        const ALGORITHM_BALANCED = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_BALANCED_BIT;
        const ALGORITHM_FULL = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FULL_BIT;
        const ALGORITHM_EXTENSIVE = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_EXTENSIVE_BIT;
        const ALGORITHM_MASK = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_MASK;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DefragmentationMoveOperation(i32);

impl DefragmentationMoveOperation {
    pub const COPY: Self = Self(ffi::VmaDefragmentationMoveOperation_VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY);
    pub const IGNORE: Self = Self(ffi::VmaDefragmentationMoveOperation_VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE);
    pub const DESTROY: Self = Self(ffi::VmaDefragmentationMoveOperation_VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY);
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct VirtualBlockCreateFlags: i32 {
        const LINEAR_ALGORITHM = ffi::VmaPoolCreateFlagBits_VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT;
        const ALGORITHM_MASK = ffi::VmaDefragmentationFlagBits_VMA_DEFRAGMENTATION_FLAG_ALGORITHM_MASK;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct VirtualAllocationCreateFlags: i32 {
        const UPPER_ADDRESS = ffi::VmaVirtualAllocationCreateFlagBits_VMA_VIRTUAL_ALLOCATION_CREATE_UPPER_ADDRESS_BIT;
        const STRATEGY_MIN_MEMORY = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT;
        const STRATEGY_MIN_TIME = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT;
        const STRATEGY_MIN_OFFSET = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT;
        const STRATEGY_MASK = ffi::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MASK;
    }
}
