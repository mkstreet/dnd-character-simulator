use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "dnd_character_api")]
extern "C" {
    pub fn get_name() -> *const c_char;
}

fn main() {
    unsafe {
        let name_ptr = get_name();
        if !name_ptr.is_null() {
            let c_str = CStr::from_ptr(name_ptr);
            if let Ok(name) = c_str.to_str() {
                println!("Character Name: {}", name);
            } else {
                eprintln!("Failed to convert character name to string.");
            }
        } else {
            eprintln!("Received null pointer from get_name.");
        }
    }
}
