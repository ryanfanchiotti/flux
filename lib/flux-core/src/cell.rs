use flux_attrs::*;

defs! {
    use crate::ptr::{valid, aligned_to};
}

#[extern_spec(core::cell)]
struct UnsafeCell<T>;

#[extern_spec(core::cell)]
impl<T> UnsafeCell<T> {
    #[spec(fn(&Self[@s]) -> *mut {v: valid(v, T::size_of()) && aligned_to(v, T::align_of())} T)]
    const fn get(&self) -> *mut T;
}
