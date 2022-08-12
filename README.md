<!-- markdownlint-disable-file MD041 -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🌋 vk-mem-alloc-rs`

**A very lightweight wrapper around the Vulkan Memory Allocator 🦀**

[![license][license-badge]][license-url]
[![vma][vma-badge]][vma-url]
[![dependency-status][dependency-badge]][dependency-url]

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

<h3>Simple Vulkan Memory Allocator example</h3>
````rust
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
````

<h3>Credits</h3>
<ul>
    <li><a href="https://gpuopen.com/vulkan-memory-allocator/">AMD</a> for creating the Vulkan Memory Allocator.</li>
    <li><a href="https://github.com/ash-rs/ash">The Ash community</a> for creating such an awesome rust wrapper around Vulkan.</li>
    <li><a href="https://github.com/gwihlidal/vk-mem-rs">Graham Wihlidal</a> for creating `vk-mem`, my buildscript is based on its build script.</li>
</ul>