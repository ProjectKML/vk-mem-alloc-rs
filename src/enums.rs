use bitflags::bitflags;

bitflags! {
    pub struct AllocatorCreateFlags : u32 {
        const NONE = 0;
        const EXTERNALLY_SYNCHRONIZED = 0x00000001;
        const KHR_DEDICATED_ALLOCATION = 0x00000002;
        const KHR_BIND_MEMORY2 = 0x00000004;
        const EXT_MEMORY_BUDGET = 0x00000008;
        const AMD_DEVICE_COHERENT_MEMORY = 0x00000010;
        const BUFFER_DEVICE_ADDRESS = 0x00000020;
        const EXT_MEMORY_PRIORITY = 0x00000040;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct MemoryUsage(pub u32);

impl MemoryUsage {
    pub const UNKNOWN: Self = Self(0);
    #[deprecated]
    pub const GPU_ONLY: Self = Self(1);
    #[deprecated]
    pub const CPU_ONLY: Self = Self(2);
    #[deprecated]
    pub const CPU_TO_GPU: Self = Self(3);
    #[deprecated]
    pub const GPU_TO_CPU: Self = Self(4);
    #[deprecated]
    pub const CPU_COPY: Self = Self(5);
    #[deprecated]
    pub const GPU_LAZILY_ALLOCATED: Self = Self(6);
    pub const AUTO: Self = Self(7);
    pub const AUTO_PREFER_DEVICE: Self = Self(8);
    pub const AUTO_PREFER_HOST: Self = Self(9);
}

bitflags! {
    pub struct AllocationCreateFlags : u32 {
        const NONE = 0;
        const DEDICATED_MEMORY = 0x00000001;
        const NEVER_ALLOCATE= 0x00000002;
        const MAPPED = 0x00000004;
        const USER_DATA_COPY_STRING = 0x00000020;
        const UPPER_ADDRESS = 0x00000040;
        const DONT_BIND = 0x00000080;
        const WITHIN_BUDGET = 0x00000100;
        const CAN_ALIAS = 0x00000200;
        const HOST_ACCESS_SEQUENTIAL_WRITE = 0x00000400;
        const HOST_ACCESS_RANDOM = 0x00000800;
        const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD = 0x00001000;
        const STRATEGY_MIN_MEMORY = 0x00010000;
        const STRATEGY_MIN_TIME = 0x00020000;
        const STRATEGY_MIN_OFFSET  = 0x00040000;
        const STRATEGY_BEST_FIT = 0x00010000;
        const STRATEGY_FIRST_FIT = 0x00020000;
        const STRATEGY_MASK = 0x00010000 | 0x00020000 | 0x00040000;
    }
}

impl Default for AllocationCreateFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

bitflags! {
    pub struct PoolCreateFlags : u32 {
        const NONE = 0;
        const IGNORE_BUFFER_IMAGE_GRANULARITY = 0x00000002;
        const LINEAR_ALGORITHM = 0x00000004;
        const ALGORITHM_MASK = 0x00000004;
    }
}

impl Default for PoolCreateFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

bitflags! {
    pub struct DefragmentationFlags : u32 {
        const NONE = 0;
        const ALGORITHM_FAST = 0x1;
        const ALGORITHM_BALANCED = 0x2;
        const ALGORITHM_FULL = 0x4;
        const ALGORITHM_EXTENSIVE = 0x8;
        const ALGORITHM_MASK = 0x1 | 0x2 | 0x4 | 0x8;
    }
}

impl Default for DefragmentationFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct DefragmentationMoveOperation(pub u32);

impl DefragmentationMoveOperation {
    pub const COPY: Self = Self(0);
    pub const IGNORE: Self = Self(1);
    pub const DESTROY: Self = Self(2);
}

bitflags! {
    pub struct VirtualBlockCreateFlags : u32 {
        const NONE = 0;
        const LINEAR_ALGORITHM = 0x00000001;
        const ALGORITHM_MASK = 0x00000001;
    }
}

impl Default for VirtualBlockCreateFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

bitflags! {
    pub struct VirtualAllocationCreateFlags : u32 {
        const NONE = 0;
        const UPPER_ADDRESS = 0x00000040;
        const STRATEGY_MIN_MEMORY = 0x00010000;
        const STRATEGY_MIN_TIME = 0x00020000;
        const STRATEGY_MIN_OFFSET = 0x00040000;
        const STRATEGY_MASK = 0x00010000 | 0x00020000 | 0x00040000;
    }
}

impl Default for VirtualAllocationCreateFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}
