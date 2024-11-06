#[link(name = "dnd_character_api", kind = "dylib")]
extern "C" {
    pub fn get_name() -> *const i8;
}
