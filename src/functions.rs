use std::{
    ffi::{c_char, c_void},
    mem::MaybeUninit,
    ptr
};

use ash::{prelude::VkResult, vk, Device, Instance};

use crate::*;

#[inline]
fn result_from_ffi(result: vk::Result) -> VkResult<()> {
    match result {
        vk::Result::SUCCESS => Ok(()),
        _ => Err(result)
    }
}

pub unsafe fn create_allocator(instance: &Instance, physical_device: vk::PhysicalDevice, device: &Device, allocator_create_info: Option<&AllocatorCreateInfo>) -> VkResult<Allocator> {
    unsafe extern "system" fn get_instance_proc_addr_stub(_instance: vk::Instance, _name: *const c_char) -> vk::PFN_vkVoidFunction {
        panic!("VMA_DYNAMIC_VULKAN_FUNCTIONS is unsupported")
    }

    unsafe extern "system" fn get_get_device_proc_stub(_device: vk::Device, _name: *const c_char) -> vk::PFN_vkVoidFunction {
        panic!("VMA_DYNAMIC_VULKAN_FUNCTIONS is unsupported")
    }

    let vulkan_functions = VulkanFunctions {
        vk_get_instance_proc_addr: get_instance_proc_addr_stub,
        vk_get_device_proc_addr: get_get_device_proc_stub,
        vk_get_physical_device_properties: instance.fp_v1_0().get_physical_device_properties,
        vk_get_physical_device_memory_properties: instance.fp_v1_0().get_physical_device_memory_properties,
        vk_allocate_memory: device.fp_v1_0().allocate_memory,
        vk_free_memory: device.fp_v1_0().free_memory,
        vk_map_memory: device.fp_v1_0().map_memory,
        vk_unmap_memory: device.fp_v1_0().unmap_memory,
        vk_flush_mapped_memory_ranges: device.fp_v1_0().flush_mapped_memory_ranges,
        vk_invalidate_mapped_memory_ranges: device.fp_v1_0().invalidate_mapped_memory_ranges,
        vk_bind_buffer_memory: device.fp_v1_0().bind_buffer_memory,
        vk_bind_image_memory: device.fp_v1_0().bind_image_memory,
        vk_get_buffer_memory_requirements: device.fp_v1_0().get_buffer_memory_requirements,
        vk_get_image_memory_requirements: device.fp_v1_0().get_image_memory_requirements,
        vk_create_buffer: device.fp_v1_0().create_buffer,
        vk_destroy_buffer: device.fp_v1_0().destroy_buffer,
        vk_create_image: device.fp_v1_0().create_image,
        vk_destroy_image: device.fp_v1_0().destroy_image,
        vk_cmd_copy_buffer: device.fp_v1_0().cmd_copy_buffer,
        vk_get_buffer_memory_requirements_2_khr: device.fp_v1_1().get_buffer_memory_requirements2,
        vk_get_image_memory_requirements_2_kr: device.fp_v1_1().get_image_memory_requirements2,
        vk_bind_buffer_memory_2_khr: device.fp_v1_1().bind_buffer_memory2,
        vk_bind_image_memory_2_khr: device.fp_v1_1().bind_image_memory2,
        vk_get_physical_device_memory_properties_2_khr: instance.fp_v1_1().get_physical_device_memory_properties2,
        vk_get_device_buffer_memory_requirements: device.fp_v1_3().get_device_buffer_memory_requirements,
        vk_get_device_image_memory_requirements: device.fp_v1_3().get_device_image_memory_requirements
    };

    let mut allocator_create_info = allocator_create_info
        .map(|i| {
            assert_eq!(i.instance, vk::Instance::null());
            assert_eq!(i.physical_device, vk::PhysicalDevice::null());
            assert_eq!(i.device, vk::Device::null());
            assert_eq!(i.vulkan_functions, ptr::null());
            *i
        })
        .unwrap_or_default();
    allocator_create_info.instance = instance.handle();
    allocator_create_info.physical_device = physical_device;
    allocator_create_info.device = device.handle();
    allocator_create_info.vulkan_functions = &vulkan_functions;

    create_allocator_raw(&allocator_create_info)
}

#[inline]
pub unsafe fn create_allocator_raw(allocator_create_info: &AllocatorCreateInfo) -> VkResult<Allocator> {
    let mut allocator = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateAllocator(
        allocator_create_info as *const AllocatorCreateInfo as *const _,
        &mut allocator as *mut _ as *mut _
    ))?;
    Ok(allocator.assume_init())
}

#[inline]
pub unsafe fn destroy_allocator(allocator: Allocator) {
    ffi::vmaDestroyAllocator(allocator.as_raw() as _);
}

#[inline]
pub unsafe fn get_allocator_info(allocator: Allocator) -> AllocatorInfo {
    let mut allocator_info = MaybeUninit::uninit();
    ffi::vmaGetAllocatorInfo(allocator.as_raw() as _, &mut allocator_info as *mut _ as *mut _);
    allocator_info.assume_init()
}

#[inline]
pub unsafe fn get_physical_device_properties(allocator: Allocator) -> *const vk::PhysicalDeviceProperties {
    let mut physical_device_properties = MaybeUninit::uninit();
    ffi::vmaGetPhysicalDeviceProperties(allocator.as_raw() as _, &mut physical_device_properties as *mut _ as *mut _);
    physical_device_properties.assume_init()
}

#[inline]
pub unsafe fn get_memory_properties(allocator: Allocator) -> *const vk::PhysicalDeviceMemoryProperties {
    let mut memory_properties = MaybeUninit::uninit();
    ffi::vmaGetMemoryProperties(allocator.as_raw() as _, &mut memory_properties as *mut _ as *mut _);
    memory_properties.assume_init()
}

#[inline]
pub unsafe fn get_memory_type_properties(allocator: Allocator, memory_type_index: u32) -> vk::MemoryPropertyFlags {
    let mut memory_property_flags = MaybeUninit::uninit();
    ffi::vmaGetMemoryTypeProperties(allocator.as_raw() as _, memory_type_index, &mut memory_property_flags as *mut _ as *mut _);
    memory_property_flags.assume_init()
}

#[inline]
pub unsafe fn set_current_frame_index(allocator: Allocator, frame_index: u32) {
    ffi::vmaSetCurrentFrameIndex(allocator.as_raw() as _, frame_index);
}

#[inline]
pub unsafe fn calculate_statistics(allocator: Allocator) -> TotalStatistics {
    let mut total_statistics = MaybeUninit::uninit();
    ffi::vmaCalculateStatistics(allocator.as_raw() as _, &mut total_statistics as *mut _ as *mut _);
    total_statistics.assume_init()
}

#[inline]
pub unsafe fn get_heap_budgets(allocator: Allocator) -> Vec<Budget> {
    let len = (*get_memory_properties(allocator)).memory_heap_count as _;

    let mut budgets = Vec::with_capacity(len);
    ffi::vmaGetHeapBudgets(allocator.as_raw() as _, budgets.as_mut_ptr() as *mut _);
    budgets.set_len(len);

    budgets
}

#[inline]
pub unsafe fn find_memory_type_index(allocator: Allocator, memory_type_bits: u32, allocation_create_info: &AllocationCreateInfo) -> VkResult<u32> {
    let mut memory_type_index = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaFindMemoryTypeIndex(
        allocator.as_raw() as _,
        memory_type_bits,
        allocation_create_info as *const AllocationCreateInfo as *const _,
        &mut memory_type_index as *mut _ as *mut _
    ))?;
    Ok(memory_type_index.assume_init())
}

#[inline]
pub unsafe fn find_memory_type_index_for_buffer_info(allocator: Allocator, buffer_create_info: &vk::BufferCreateInfo, allocation_create_info: &AllocationCreateInfo) -> VkResult<u32> {
    let mut memory_type_index = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaFindMemoryTypeIndexForBufferInfo(
        allocator.as_raw() as _,
        buffer_create_info as *const _,
        allocation_create_info as *const AllocationCreateInfo as *const _,
        &mut memory_type_index as *mut _ as *mut _
    ))?;
    Ok(memory_type_index.assume_init())
}

#[inline]
pub unsafe fn find_memory_type_index_for_image_info(allocator: Allocator, image_create_info: &vk::ImageCreateInfo, allocation_create_info: &AllocationCreateInfo) -> VkResult<u32> {
    let mut memory_type_index = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaFindMemoryTypeIndexForImageInfo(
        allocator.as_raw() as _,
        image_create_info as *const _,
        allocation_create_info as *const AllocationCreateInfo as *const _,
        &mut memory_type_index as *mut _ as *mut _
    ))?;
    Ok(memory_type_index.assume_init())
}

#[inline]
pub unsafe fn create_pool(allocator: Allocator, create_info: &PoolCreateInfo) -> VkResult<Pool> {
    let mut pool = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreatePool(
        allocator.as_raw() as _,
        create_info as *const PoolCreateInfo as *const _,
        &mut pool as *mut _ as *mut _
    ))?;
    Ok(pool.assume_init())
}

#[inline]
pub unsafe fn destroy_pool(allocator: Allocator, pool: Pool) {
    ffi::vmaDestroyPool(allocator.as_raw() as _, pool.as_raw() as _);
}

#[inline]
pub unsafe fn get_pool_statistics(allocator: Allocator, pool: Pool) -> Statistics {
    let mut statistics = MaybeUninit::uninit();
    ffi::vmaGetPoolStatistics(allocator.as_raw() as _, pool.as_raw() as _, &mut statistics as *mut _ as *mut _);
    statistics.assume_init()
}

#[inline]
pub unsafe fn calculate_pool_statistics(allocator: Allocator) -> DetailedStatistics {
    let mut statistics = MaybeUninit::uninit();
    ffi::vmaCalculateStatistics(allocator.as_raw() as _, &mut statistics as *mut _ as *mut _);
    statistics.assume_init()
}

#[inline]
pub unsafe fn check_pool_corruption(allocator: Allocator, pool: Pool) -> VkResult<()> {
    result_from_ffi(ffi::vmaCheckPoolCorruption(allocator.as_raw() as _, pool.as_raw() as _))
}

#[inline]
pub unsafe fn get_pool_name(allocator: Allocator, pool: Pool) -> *const c_char {
    let mut name = MaybeUninit::uninit();
    ffi::vmaGetPoolName(allocator.as_raw() as _, pool.as_raw() as _, &mut name as *mut _ as *mut _);
    name.assume_init()
}

#[inline]
pub unsafe fn set_pool_name(allocator: Allocator, pool: Pool, name: *const c_char) {
    ffi::vmaSetPoolName(allocator.as_raw() as _, pool.as_raw() as _, name);
}

#[inline]
pub unsafe fn allocate_memory(allocator: Allocator, memory_requirements: &vk::MemoryRequirements, create_info: &AllocationCreateInfo) -> VkResult<(Allocation, AllocationInfo)> {
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaAllocateMemory(
        allocator.as_raw() as _,
        memory_requirements as *const _ as *const _,
        create_info as *const _ as *const _,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn allocate_memory_pages(
    allocator: Allocator,
    memory_requirements: &vk::MemoryRequirements,
    create_info: &AllocationCreateInfo,
    allocation_count: usize
) -> VkResult<(Allocation, AllocationInfo)> {
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaAllocateMemoryPages(
        allocator.as_raw() as _,
        memory_requirements as *const _ as *const _,
        create_info as *const _ as *const _,
        allocation_count,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn allocate_memory_for_buffer(allocator: Allocator, buffer: vk::Buffer, create_info: *const AllocationCreateInfo) -> VkResult<(Allocation, AllocationInfo)> {
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaAllocateMemoryForBuffer(
        allocator.as_raw() as _,
        buffer,
        create_info as *const _ as *const _,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn allocate_memory_for_image(allocator: Allocator, image: vk::Image, create_info: *const AllocationCreateInfo) -> VkResult<(Allocation, AllocationInfo)> {
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaAllocateMemoryForImage(
        allocator.as_raw() as _,
        image,
        create_info as *const _ as *const _,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn free_memory_pages(allocator: Allocator, allocation_count: usize, allocations: *mut Allocation) {
    ffi::vmaFreeMemoryPages(allocator.as_raw() as _, allocation_count, allocations.cast());
}

#[inline]
pub unsafe fn get_allocation_info(allocator: Allocator, allocation: Allocation) -> AllocationInfo {
    let mut allocation_info = MaybeUninit::uninit();
    ffi::vmaGetAllocationInfo(allocator.as_raw() as _, allocation.as_raw() as _, &mut allocation_info as *mut _ as *mut _);
    allocation_info.assume_init()
}

#[inline]
pub unsafe fn set_allocation_user_data(allocator: Allocator, allocation: Allocation, user_data: *mut c_void) {
    ffi::vmaSetAllocationUserData(allocator.as_raw() as _, allocation.as_raw() as _, user_data);
}

#[inline]
pub unsafe fn set_allocation_name(allocator: Allocator, allocation: Allocation, name: *const c_char) {
    ffi::vmaSetAllocationName(allocator.as_raw() as _, allocation.as_raw() as _, name);
}

#[inline]
pub unsafe fn get_allocation_memory_properties(allocator: Allocator, allocation: Allocation) -> vk::MemoryPropertyFlags {
    let mut memory_property_flags = MaybeUninit::uninit();
    ffi::vmaGetAllocationMemoryProperties(allocator.as_raw() as _, allocation.as_raw() as _, &mut memory_property_flags as *mut _ as *mut _);
    memory_property_flags.assume_init()
}

#[inline]
pub unsafe fn map_memory(allocator: Allocator, allocation: Allocation) -> VkResult<*mut c_void> {
    let mut data = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaMapMemory(allocator.as_raw() as _, allocation.as_raw() as _, &mut data as *mut _ as *mut _))?;
    Ok(data.assume_init())
}

#[inline]
pub unsafe fn unmap_memory(allocator: Allocator, allocation: Allocation) {
    ffi::vmaUnmapMemory(allocator.as_raw() as _, allocation.as_raw() as _);
}

#[inline]
pub unsafe fn flush_allocation(allocator: Allocator, allocation: Allocation, offset: vk::DeviceSize, size: vk::DeviceSize) -> VkResult<()> {
    result_from_ffi(ffi::vmaFlushAllocation(allocator.as_raw() as _, allocation.as_raw() as _, offset, size))
}

#[inline]
pub unsafe fn invalidate_allocation(allocator: Allocator, allocation: Allocation, offset: vk::DeviceSize, size: vk::DeviceSize) -> VkResult<()> {
    result_from_ffi(ffi::vmaInvalidateAllocation(allocator.as_raw() as _, allocation.as_raw() as _, offset, size))
}

#[inline]
pub unsafe fn flush_allocations(allocator: Allocator, allocation_count: u32, allocations: *mut Allocation, offsets: *const vk::DeviceSize, sizes: *const vk::DeviceSize) -> VkResult<()> {
    result_from_ffi(ffi::vmaFlushAllocations(allocator.as_raw() as _, allocation_count, allocations.cast(), offsets, sizes))
}

#[inline]
pub unsafe fn invalidate_allocations(
    allocator: Allocator,
    allocation_count: u32,
    allocations: *mut Allocation,
    offsets: *const vk::DeviceSize,
    sizes: *const vk::DeviceSize
) -> VkResult<()> {
    result_from_ffi(ffi::vmaInvalidateAllocations(allocator.as_raw() as _, allocation_count, allocations.cast(), offsets, sizes))
}

#[inline]
pub unsafe fn check_corruption(allocator: Allocator, memory_type_bits: u32) -> VkResult<()> {
    result_from_ffi(ffi::vmaCheckCorruption(allocator.as_raw() as _, memory_type_bits))
}

#[inline]
pub unsafe fn begin_defragmentation(allocator: Allocator, info: &DefragmentationInfo) -> VkResult<DefragmentationContext> {
    let mut defragmentation_context = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaBeginDefragmentation(
        allocator.as_raw() as _,
        info as *const _ as *const _,
        &mut defragmentation_context as *mut _ as *mut _
    ))?;
    Ok(defragmentation_context.assume_init())
}

#[inline]
pub unsafe fn end_defragmentation(allocator: Allocator, context: DefragmentationContext) -> DefragmentationStats {
    let mut defragmentation_stats = MaybeUninit::uninit();
    ffi::vmaEndDefragmentation(allocator.as_raw() as _, context.as_raw() as _, &mut defragmentation_stats as *mut _ as *mut _);
    defragmentation_stats.assume_init()
}

#[inline]
pub unsafe fn begin_defragmentation_pass(allocator: Allocator, context: DefragmentationContext) -> VkResult<DefragmentationPassMoveInfo> {
    let mut defragmentation_pass_move_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaBeginDefragmentationPass(
        allocator.as_raw() as _,
        context.as_raw() as _,
        &mut defragmentation_pass_move_info as *mut _ as *mut _
    ))?;
    Ok(defragmentation_pass_move_info.assume_init())
}

#[inline]
pub unsafe fn end_defragmentation_pass(allocator: Allocator, context: DefragmentationContext) -> VkResult<DefragmentationPassMoveInfo> {
    let mut defragmentation_pass_move_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaEndDefragmentationPass(
        allocator.as_raw() as _,
        context.as_raw() as _,
        &mut defragmentation_pass_move_info as *mut _ as *mut _
    ))?;
    Ok(defragmentation_pass_move_info.assume_init())
}

#[inline]
pub unsafe fn bind_buffer_memory(allocator: Allocator, allocation: Allocation, buffer: vk::Buffer) -> VkResult<()> {
    result_from_ffi(ffi::vmaBindBufferMemory(allocator.as_raw() as _, allocation.as_raw() as _, buffer))
}

#[inline]
pub unsafe fn bind_buffer_memory2(allocator: Allocator, allocation: Allocation, allocation_local_offset: vk::DeviceSize, buffer: vk::Buffer, next: *const c_void) -> VkResult<()> {
    result_from_ffi(ffi::vmaBindBufferMemory2(
        allocator.as_raw() as _,
        allocation.as_raw() as _,
        allocation_local_offset,
        buffer,
        next
    ))
}

#[inline]
pub unsafe fn bind_image_memory(allocator: Allocator, allocation: Allocation, image: vk::Image) -> VkResult<()> {
    result_from_ffi(ffi::vmaBindImageMemory(allocator.as_raw() as _, allocation.as_raw() as _, image))
}

#[inline]
pub unsafe fn bind_image_memory2(allocator: Allocator, allocation: Allocation, allocation_local_offset: vk::DeviceSize, image: vk::Image, next: *const c_void) -> VkResult<()> {
    result_from_ffi(ffi::vmaBindImageMemory2(allocator.as_raw() as _, allocation.as_raw() as _, allocation_local_offset, image, next))
}

#[inline]
pub unsafe fn create_buffer(
    allocator: Allocator,
    buffer_create_info: &vk::BufferCreateInfo,
    allocation_create_info: &AllocationCreateInfo
) -> VkResult<(vk::Buffer, Allocation, AllocationInfo)> {
    let mut buffer = MaybeUninit::uninit();
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateBuffer(
        allocator.as_raw() as _,
        buffer_create_info as *const _,
        allocation_create_info as *const _ as *const _,
        &mut buffer as *mut _ as *mut _,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((buffer.assume_init(), allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn create_buffer_with_alignment(
    allocator: Allocator,
    buffer_create_info: &vk::BufferCreateInfo,
    allocation_create_info: &AllocationCreateInfo,
    min_alignment: vk::DeviceSize
) -> VkResult<(vk::Buffer, Allocation, AllocationInfo)> {
    let mut buffer = MaybeUninit::uninit();
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateBufferWithAlignment(
        allocator.as_raw() as _,
        buffer_create_info as *const _,
        allocation_create_info as *const AllocationCreateInfo as *const _,
        min_alignment,
        &mut buffer as *mut _ as *mut _,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((buffer.assume_init(), allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn create_aliasing_buffer(allocator: Allocator, allocation: Allocation, buffer_create_info: &vk::BufferCreateInfo) -> VkResult<vk::Buffer> {
    let mut buffer = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateAliasingBuffer(
        allocator.as_raw() as _,
        allocation.as_raw() as _,
        buffer_create_info as *const _,
        &mut buffer as *mut _ as *mut _
    ))?;
    Ok(buffer.assume_init())
}

#[inline]
pub unsafe fn destroy_buffer(allocator: Allocator, buffer: vk::Buffer, allocation: Allocation) {
    ffi::vmaDestroyBuffer(allocator.as_raw() as _, buffer, allocation.as_raw() as _);
}

#[inline]
pub unsafe fn create_image(
    allocator: Allocator,
    image_create_info: &vk::ImageCreateInfo,
    allocation_create_info: &AllocationCreateInfo
) -> VkResult<(vk::Image, Allocation, AllocationInfo)> {
    let mut image = MaybeUninit::uninit();
    let mut allocation = MaybeUninit::uninit();
    let mut allocation_info = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateImage(
        allocator.as_raw() as _,
        image_create_info as *const _,
        allocation_create_info as *const AllocationCreateInfo as *const _,
        &mut image as *mut _ as *mut _,
        &mut allocation as *mut _ as *mut _,
        &mut allocation_info as *mut _ as *mut _
    ))?;
    Ok((image.assume_init(), allocation.assume_init(), allocation_info.assume_init()))
}

#[inline]
pub unsafe fn create_aliasing_image(allocator: Allocator, allocation: Allocation, image_create_info: &vk::ImageCreateInfo) -> VkResult<vk::Image> {
    let mut image = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateAliasingImage(
        allocator.as_raw() as _,
        allocation.as_raw() as _,
        image_create_info as *const _,
        &mut image as *mut _ as *mut _
    ))?;
    Ok(image.assume_init())
}

#[inline]
pub unsafe fn destroy_image(allocator: Allocator, image: vk::Image, allocation: Allocation) {
    ffi::vmaDestroyImage(allocator.as_raw() as _, image, allocation.as_raw() as _);
}

#[inline]
pub unsafe fn create_virtual_block(create_info: &VirtualBlockCreateInfo) -> VkResult<VirtualBlock> {
    let mut virtual_block = MaybeUninit::uninit();
    result_from_ffi(ffi::vmaCreateVirtualBlock(create_info as *const _ as *const _, &mut virtual_block as *mut _ as *mut _))?;
    Ok(virtual_block.assume_init())
}

#[inline]
pub unsafe fn destroy_virtual_block(virtual_block: VirtualBlock) {
    ffi::vmaDestroyVirtualBlock(virtual_block.as_raw() as _);
}

#[inline]
pub unsafe fn is_virtual_block_empty(virtual_block: VirtualBlock) -> bool {
    ffi::vmaIsVirtualBlockEmpty(virtual_block.as_raw() as _) != 0
}

#[inline]
pub unsafe fn get_virtual_allocation_info(virtual_block: VirtualBlock, allocation: VirtualAllocation) -> VirtualAllocationInfo {
    let mut virtual_alloc_info = MaybeUninit::uninit();
    ffi::vmaGetVirtualAllocationInfo(virtual_block.as_raw() as _, allocation.as_raw() as _, &mut virtual_alloc_info as *mut _ as *mut _);
    virtual_alloc_info.assume_init()
}

#[inline]
pub unsafe fn virtual_allocate(virtual_block: VirtualBlock, create_info: &VirtualAllocationCreateInfo) -> VkResult<(VirtualAllocation, vk::DeviceSize)> {
    let mut allocation = MaybeUninit::uninit();
    let mut offset = 0;
    result_from_ffi(ffi::vmaVirtualAllocate(
        virtual_block.as_raw() as _,
        create_info as *const _ as *const _,
        &mut allocation as *mut _ as *mut _,
        &mut offset
    ))?;
    Ok((allocation.assume_init(), offset))
}

#[inline]
pub unsafe fn virtual_free(virtual_block: VirtualBlock, allocation: VirtualAllocation) {
    ffi::vmaVirtualFree(virtual_block.as_raw() as _, allocation.as_raw() as _);
}

#[inline]
pub unsafe fn clear_virtual_block(virtual_block: VirtualBlock) {
    ffi::vmaClearVirtualBlock(virtual_block.as_raw() as _);
}

#[inline]
pub unsafe fn set_virtual_allocation_user_data(virtual_block: VirtualBlock, allocation: VirtualAllocation, user_data: *mut c_void) {
    ffi::vmaSetVirtualAllocationUserData(virtual_block.as_raw() as _, allocation.as_raw() as _, user_data);
}

#[inline]
pub unsafe fn get_virtual_block_statistics(virtual_block: VirtualBlock) -> Statistics {
    let mut statistics = MaybeUninit::uninit();
    ffi::vmaGetVirtualBlockStatistics(virtual_block.as_raw() as _, &mut statistics as *mut _ as *mut _);
    statistics.assume_init()
}

#[inline]
pub unsafe fn calculate_virtual_block_statistics(virtual_block: VirtualBlock) -> DetailedStatistics {
    let mut statistics = MaybeUninit::uninit();
    ffi::vmaCalculateVirtualBlockStatistics(virtual_block.as_raw() as _, &mut statistics as *mut _ as *mut _);
    statistics.assume_init()
}

#[inline]
pub unsafe fn build_virtual_block_stats_string(virtual_block: VirtualBlock, detailed_map: bool) -> *mut c_char {
    let mut stats_string = MaybeUninit::uninit();
    ffi::vmaBuildVirtualBlockStatsString(virtual_block.as_raw() as _, &mut stats_string as *mut _ as *mut _, detailed_map as _);
    stats_string.assume_init()
}

#[inline]
pub unsafe fn free_virtual_block_stats_string(virtual_block: VirtualBlock, stats_string: *mut c_char) {
    ffi::vmaFreeVirtualBlockStatsString(virtual_block.as_raw() as _, stats_string);
}

#[inline]
pub unsafe fn build_stats_string(allocator: Allocator, detailed_map: bool) -> *mut c_char {
    let mut stats_string = MaybeUninit::uninit();
    ffi::vmaBuildStatsString(allocator.as_raw() as _, &mut stats_string as *mut _ as *mut _, detailed_map as _);
    stats_string.assume_init()
}

#[inline]
pub unsafe fn free_stats_string(allocator: Allocator, stats_string: *mut c_char) {
    ffi::vmaFreeStatsString(allocator.as_raw() as _, stats_string);
}
