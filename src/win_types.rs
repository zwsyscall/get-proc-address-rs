use std::ffi::c_void;

#[repr(C)]
pub struct Peb {
    pub reserved1: [u8; 2],
    pub being_debugged: u8,
    pub reserved2: [u8; 1],
    pub reserved3: [*mut c_void; 2],
    pub ldr: *mut LdrData,
    pub process_parameters: *mut c_void,
    pub reserved4: [*mut c_void; 3],
    pub atl_thunk_slist_ptr: *mut c_void,
    pub reserved5: *mut c_void,
    pub reserved6: u32,
    pub reserved7: *mut c_void,
    pub reserved8: u32,
    pub atl_thunk_slist_ptr32: u32,
    pub reserved9: [*mut c_void; 45],
    pub reserved10: [u8; 96],
    pub post_process_init_routine: u32,
    pub reserved11: [u8; 128],
    pub reserved12: [*mut c_void; 1],
    pub session_id: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct LdrData {
    pub reserved1: [u8; 8],
    pub reserved2: [*mut c_void; 3],
    pub in_memory_order_module_list: ListEntry,
}

#[repr(C)]
#[derive(Debug)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

#[repr(C)]
pub struct LdrDataTableEntry {
    pub reserved1: [*mut c_void; 2],
    pub in_memory_order_links: ListEntry,
    pub reserved2: [*mut c_void; 2],
    pub dll_base: *mut c_void,
    pub reserved3: [*mut c_void; 2],
    pub full_dll_name: UnicodeString,
    pub reserved4: [u8; 8],
    pub reserved5: [*mut c_void; 3],
    pub anonymous: *mut c_void,
    pub time_date_stamp: u32,
}

#[repr(C)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *mut u16,
}

#[repr(C, packed(2))]
pub struct ImageDosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct ImageNtHeaders {
    pub signature: u32,
    pub file_header: [u8; 20],
    pub optional_header: ImageOptionalHeader,
}

#[cfg(target_arch = "x86_64")]
#[repr(C, packed(4))]
pub struct ImageOptionalHeader {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; 16],
}

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct ImageNtHeaders {
    pub signature: u32,
    pub file_header: [u8; 20],
    pub optional_header: ImageOptionalHeader,
}

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct ImageOptionalHeader {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; 16],
}

#[repr(C)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageExportDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub name: u32,
    pub base: u32,
    pub number_of_functions: u32,
    pub number_of_names: u32,
    pub address_of_functions: u32,
    pub address_of_names: u32,
    pub address_of_name_ordinals: u32,
}
