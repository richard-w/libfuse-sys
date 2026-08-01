use std::os::raw::c_void;

use libfuse_sys::fuse::{fuse_main, fuse_main_real, fuse_unmount};

#[test]
fn test_libfuse_linked() {
    println!("{:?}", fuse_main as *const c_void);
    println!("{:?}", fuse_main_real as *const c_void);
    println!("{:?}", fuse_unmount as *const c_void);
}
