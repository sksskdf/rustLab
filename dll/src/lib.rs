use std::ffi::{CString, c_char};
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

extern crate winreg;

#[no_mangle]
pub extern "cdecl" fn add_numbers(a: i32, b: i32) -> i32 { a + b }

#[no_mangle]
pub extern "cdecl" fn collect_shellbag_mru() -> *mut c_char {
    let bagmru_path = r"Local Settings\Software\Microsoft\Windows\Shell\BagMRU";

    let hkey_classes_root = RegKey::predef(HKEY_CLASSES_ROOT);
    let mut output = String::new();

    if let Ok(bagmru_key) = hkey_classes_root.open_subkey(bagmru_path) {
        for (key, value) in bagmru_key.enum_values().filter_map(Result::ok) {
            output.push_str(&format!("Key: {}, Value: {:?}\n", key, value));
        }
    } else {
        output.push_str("Failed to open BagMRU key.");
    }

    let c_string = CString::new(output).unwrap_or_else(|_| CString::new("Error creating CString").unwrap());
    c_string.into_raw()
}

#[no_mangle]
pub extern "cdecl" fn free_bagmru_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(ptr);
    }
}
