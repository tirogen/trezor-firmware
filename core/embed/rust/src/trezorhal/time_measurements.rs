use super::ffi;

pub fn get_ticks() {
    unsafe {
        ffi::get_ticks();
    }
}
pub fn init_ticks() {
    unsafe {
        ffi::init_ticks();
    }
}
pub fn clear_acc() {
    unsafe {
        ffi::clear_acc();
    }
}
