extern crate flux_core;

use std::ptr::NonNull;
use flux_rs::assert;

// -- dangling --

pub fn test_dangling_eq() {
    let nn1: NonNull<i32> = NonNull::dangling();
    let nn2: NonNull<i32> = NonNull::dangling();
    assert(nn1 == nn2)
}
