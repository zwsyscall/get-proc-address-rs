#![allow(unused_unsafe)]
//! # Get-Proc-Address-rs
//!
//! `get_proc_address` provides a rust native alternative to GetProcAddress
//!
//! ## Example
//! ```rust
//! use get_proc_address_rs::get_proc_address;
//! let pointer = get_proc_address("kernel32.dll", "IsDebuggerPresent").unwrap();
//! unsafe {
//!     let is_debugger_present: IsDebuggerPresent = std::mem::transmute(pointer);
//!     let being_debugged = is_debugger_present();
//!     assert!(!being_debugged);
//! }
//! ```
use std::{arch::asm, ffi::CStr, mem::offset_of};
mod types;
use types::*;
pub mod win_types;
pub use win_types::*;

macro_rules! ptr_at {
    ($base:expr, $rva:expr, $T:ty) => {
        unsafe { ($base as *const u8).add($rva as usize).cast::<$T>() }
    };
}

/// Fetches the PEB offset by offsetting from gs
/// # Examples
/// ```rust
/// let peb_offset = get_proc_address_rs::fetch_peb_offset();
/// assert!(!peb_offset == 0);
/// ```
#[cfg(target_arch = "x86_64")]
pub fn fetch_peb_offset() -> u64 {
    #[allow(unused_assignments)]
    let mut offset: u64 = 0;
    unsafe {
        asm!(
            "mov {}, gs:[0x60]",
            out(reg) offset,
        );
    }
    offset
}

/// Fetches the PEB offset by offsetting from fs
#[cfg(target_arch = "x86")]
pub fn fetch_peb_offset() -> u32 {
    #[allow(unused_assignments)]
    let mut offset: u32 = 0;
    unsafe {
        asm!(
            "mov {}, fs:[0x30]",
            out(reg) offset,
        );
    }
    offset
}

fn fetch_loaded_modules(ldr: &LdrData) -> Vec<DllEntry> {
    let mut dll_entries: Vec<DllEntry> = Vec::new();
    unsafe {
        // Iterate through the ldr flink
        let mut flink = ldr.in_memory_order_module_list.flink;
        let head = &ldr.in_memory_order_module_list as *const _ as *mut _;
        while flink != head {
            // We have CONTAINING_RECORD at home
            let entry = (flink as usize - offset_of!(LdrDataTableEntry, in_memory_order_links)) as *const LdrDataTableEntry;

            dll_entries.push(entry.into());
            flink = (*flink).flink;
        }
    }
    dll_entries
}

fn fetch_module_functions(base_address: *mut u8, image_export_dir: &ImageExportDirectory) -> Vec<ModuleFunction> {
    let mut functions: Vec<ModuleFunction> = Vec::new();
    // Set up required offsets
    let function_name_offset = ptr_at!(base_address, image_export_dir.address_of_names, u32);
    let function_ordinal_offset = ptr_at!(base_address, image_export_dir.address_of_name_ordinals, u16);
    let function_address_offset = ptr_at!(base_address, image_export_dir.address_of_functions, u32);

    for idx in 0..image_export_dir.number_of_names {
        // Fetch the name pointer by offsetting start of array
        let name_offset = unsafe { function_name_offset.add(idx as usize) };

        // Deref to the actual name
        let raw_name_ptr = ptr_at!(base_address, *(name_offset as *mut u32), i8);
        let name = unsafe { CStr::from_ptr(raw_name_ptr as *mut i8) };
        let name = match name.to_str() {
            Ok(valid_name) => valid_name,
            Err(_) => continue,
        };

        // Match the name to a function
        let function_address = unsafe {
            let function_index = *(function_ordinal_offset.add(idx as usize));
            let function_rva = *(function_address_offset.byte_offset((function_index * 4) as isize));
            ptr_at!(base_address, function_rva, u8)
        };

        functions.push(ModuleFunction {
            name: name.to_string(),
            offset: function_address,
        })
    }
    functions
}

/// Takes in the dll and function names and returns a pointer to the function.
/// # Example
/// ```rust
/// let pointer = get_proc_address_rs::get_proc_address("kernel32.dll", "GetProcAddress");
/// assert!(pointer.is_some());
/// ```
pub fn get_proc_address(dll_name: &str, function_name: &str) -> Option<*const u8> {
    // First, let's fetch the PEB
    let peb_offset = fetch_peb_offset();
    let peb_ptr = peb_offset as *const Peb;

    // Deref to peb
    let peb_ref = unsafe { &*peb_ptr };

    // Deref to ldr
    let ldr = unsafe { &*peb_ref.ldr };

    let modules = fetch_loaded_modules(ldr);

    if let Some(module) = modules.iter().find(|m| m.module_name() == dll_name) {
        // Find dos header
        let dos_header = module.base_address as *const ImageDosHeader;
        let e_lfanew = unsafe { (*dos_header).e_lfanew };

        // We offset with the RVA as the e_lfanew is image offset
        let nt_headers = ptr_at!(module.base_address, e_lfanew, ImageNtHeaders);
        let optional_header = unsafe { &(*nt_headers).optional_header };

        // Calculate the Export directory RVA
        let export_dir_entry: &ImageDataDirectory = &optional_header.data_directory[0];
        let export_rva = export_dir_entry.virtual_address;

        // No exports available
        if export_rva == 0 {
            return None;
        }

        let export_ptr = ptr_at!(module.base_address, export_rva, ImageExportDirectory);
        let image_export_dir: &ImageExportDirectory = unsafe { &*export_ptr };

        return fetch_module_functions(module.base_address, image_export_dir)
            .into_iter()
            .find(|f| f.name == function_name)
            .map(|f| f.offset);
    }
    return None;
}
