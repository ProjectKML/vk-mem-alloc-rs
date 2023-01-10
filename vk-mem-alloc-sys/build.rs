use std::{env, fs, path::Path};

use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct BindgenCallbacks;

impl ParseCallbacks for BindgenCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("Vk") {
            Some(original_item_name.trim_start_matches("Vk").to_string())
        } else if original_item_name.starts_with("PFN_vk") && original_item_name.ends_with("KHR") {
            Some(original_item_name.trim_end_matches("KHR").to_string())
        } else {
            None
        }
    }
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("I./wrapper")
        .clang_arg("-I./vendor/Vulkan-Headers/include")
        .header("vendor/VulkanMemoryAllocator/include/vk_mem_alloc.h")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .allowlist_function("vma.*")
        .allowlist_function("PFN_vma.*")
        .allowlist_type("Vma.*")
        .parse_callbacks(Box::new(BindgenCallbacks))
        .blocklist_type("Vk.*")
        .blocklist_type("PFN_vk.*")
        .raw_line("use ash::vk::*;")
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings");

    let bindings_str = bindings
        .to_string()
        .replace(" AllocationCallbacks", " AllocationCallbacks<'a>")
        .replace(" VmaAllocatorCreateInfo", " VmaAllocatorCreateInfo<'a>")
        .replace(" VmaVirtualBlockCreateInfo", " VmaVirtualBlockCreateInfo<'a>")
        .replace("vmaCreateAllocator", "vmaCreateAllocator<'a>")
        .replace("vmaCreateVirtualBlock", "vmaCreateVirtualBlock<'a>");

    fs::create_dir_all("gen").unwrap();
    fs::write(Path::new("gen/bindings.rs"), bindings_str).expect("Failed to write bindings to file");
}

fn main() {
    let mut build = cc::Build::new();

    build.define("VMA_STATIC_VULKAN_FUNCTIONS", "0").define("VMA_DYNAMIC_VULKAN_FUNCTIONS", "0");

    #[cfg(feature = "recording")]
    build.define("VMA_RECORDING_ENABLED", "1");

    let target = env::var("TARGET").unwrap();

    if target.contains("gnu") || target.contains("darwin") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-nullability-completeness")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag("-Wno-parentheses")
            .flag("-Wno-implicit-fallthrough");
    }

    build
        .cpp(true)
        .include("vendor/Vulkan-Headers/include")
        .include("vendor/VulkanMemoryAllocator/include")
        .file("wrapper/vk_mem_alloc.cpp")
        .compile("vk_mem_alloc_sys_cc");

    generate_bindings();
}
