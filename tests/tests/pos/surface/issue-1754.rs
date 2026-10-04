pub struct Token<T>(*mut T);
pub struct Wrap<T>(T);

pub trait Tr {
    type Out;
    fn get(self) -> Self::Out;
}

impl<T> Tr for Wrap<T> {
    type Out = T;
    fn get(self) -> T {
        self.0
    }
}

pub fn spawn<F>(_future: impl FnOnce() -> F) -> Token<impl Sized> {
    Token(std::ptr::null_mut::<F>())
}

#[flux::spec(fn() -> Wrap<impl Iterator<Item = i32[123]>>)]
pub fn test_return_impl() -> Wrap<impl Iterator<Item = i32>> {
    Wrap(Some(123).into_iter())
}

#[flux::spec(fn() -> Token<impl Iterator<Item = i32{v:1 <= v}>>)]
pub fn test_return_impl_ptr() -> Token<impl Iterator<Item = i32>> {
    Token(Box::into_raw(Box::new(Some(22).into_iter())))
}

#[flux::spec(fn(n: i32, x: Wrap<i32{v: v >= n - 20}>))]
fn wrap_bounded(_n: i32, _x: Wrap<i32>) {}

#[flux::spec(fn(n: i32) -> impl Tr<Out = i32{v: v >= n}>)]
pub fn test_rec_bound(n: i32) -> impl Tr<Out = i32> {
    if n > 0 {
        wrap_bounded(n, test_rec_bound(n - 10));
    }
    Wrap(n)
}
