use std::mem;
use std::slice;
use std::os::raw::c_void;

use app::{self};

extern {
    fn unsafe_random() -> f32;
}

pub fn gen_rand() -> f32 {
    unsafe {
        unsafe_random()
    }
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn initialize(width: usize, height: usize) {
    app::init(width, height);
}

// the Javascript side passes a pointer to a buffer, the size of the corresponding canvas
#[no_mangle]
pub extern "C" fn step(pointer: *mut u8, width: usize, height: usize) {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = width * height * 4;
    let buf = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    app::update(buf);
    app::render(buf);
}