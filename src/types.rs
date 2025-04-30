use crate::LdrDataTableEntry;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::slice;

pub struct DllEntry {
    pub name: String,
    pub base_address: *mut u8,
}

impl From<*const LdrDataTableEntry> for DllEntry {
    fn from(ptr: *const LdrDataTableEntry) -> Self {
        let dll_name = unsafe { &(*ptr).full_dll_name };
        let len = (dll_name.length / 2) as usize;
        let wide_slice = unsafe { slice::from_raw_parts(dll_name.buffer, len) };

        let name = OsString::from_wide(wide_slice).to_string_lossy().into_owned();

        let offset = unsafe { &(*ptr).dll_base };

        DllEntry {
            name: name.to_string(),
            base_address: offset.clone() as *mut u8,
        }
    }
}

impl DllEntry {
    pub fn module_name(&self) -> String {
        Path::new(&self.name).file_name().and_then(|s| s.to_str()).unwrap_or("").to_lowercase()
    }
}

pub struct ModuleFunction {
    pub name: String,
    pub offset: *const u8,
}
