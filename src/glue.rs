use std::slice;
use std::os::raw::c_void;
use std::sync::Mutex;

use app::{self};

lazy_static! {
    static ref memory: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

extern {
    fn unsafe_random() -> f32;
    fn unsafe_log_num(num: f32);
}

pub fn gen_rand() -> f32 {
    unsafe {
        unsafe_random()
    }
}

pub fn log_num(num: f32) {
    unsafe {
        unsafe_log_num(num);
    }
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut v = memory.lock().unwrap();
    v.resize(size, 0);
    let ptr = v.as_mut_ptr();
    return ptr as *mut c_void;
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