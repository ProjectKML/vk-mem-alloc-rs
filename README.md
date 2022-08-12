<!-- markdownlint-disable-file MD041 -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🌋 vk-mem-alloc-rs`

**A very lightweight wrapper around the Vulkan Memory Allocator 🦀**

[![crates][crates-badge]][crates-url]
[![license][license-badge]][license-url]
[![vma][vma-badge]][vma-url]
[![dependency-status][dependency-badge]][dependency-url]

[crates-badge]: https://img.shields.io/crates/v/vk-mem-alloc.svg
[crates-url]: https://crates.io/crates/vk-mem-alloc

[license-badge]: https://img.shields.io/badge/License-MIT/Apache_2.0-blue.svg
[license-url]: LICENSE-MIT

[vma-badge]: https://img.shields.io/badge/Vulkan%20Memory%20Allocator-3.0.1-orange
[vma-url]: https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator

[dependency-badge]: https://deps.rs/repo/github/projectkml/vk-mem-alloc-rs/status.svg
[dependency-url]: https://deps.rs/repo/github/projectkml/vk-mem-alloc-rs

</div>

```toml
[dependencies]
vk-mem-alloc = "0.1.0"
```

### Simple Vulkan Memory Allocator example
```Rust
// Create the allocator
let allocator = vk_mem_alloc::create_allocator(&instance, physical_device, &device).unwrap();

let buffer_create_info = vk::BufferCreateInfo::default()
    .size(size)
    .usage(vk::BufferUsageFlags::STORAGE_BUFFER);

let allocation_create_info = vk_mem_alloc::AllocationCreateInfo {
    usage: vk_mem_alloc::MemoryUsage::AUTO_PREFER_DEVICE,
    ..Default::default()
};

// Create the buffer
let (buffer, allocation, allocation_info) = vk_mem_alloc::create_buffer(allocator, &buffer_create_info, &allocation_create_info).unwrap();

....

// Destroy the buffer
vk_mem_alloc::destroy_buffer(allocator, buffer, allocation);

// Destroy the allocator
vk_mem_alloc::destroy_allocator(allocator);
```

### Credits
* [AMD](https://gpuopen.com/vulkan-memory-allocator/) for creating the Vulkan Memory Allocator.
* [The Ash community](https://github.com/ash-rs/ash) for creating such an awesome rust wrapper around Vulkan.
* [Graham Wihlidal](https://github.com/gwihlidal/vk-mem-rs) for creating `vk-mem`, my buildscript is based on its build script.