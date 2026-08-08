#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    return ::core::slice::from_raw_parts(
        (p as *const T) as *const u8, 
        ::core::mem::size_of::<T>()
    );
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn any_slice_as_u8_slice<T: Sized>(p: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        p.as_ptr() as *const u8,
        std::mem::size_of::<T>() * p.len(),
    )
}
