use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
}
pub type cairo_bool_t = libc::c_int;
pub type cairo_atomic_int_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freed_pool_t {
    pub pool: [*mut libc::c_void; 16],
    pub top: cairo_atomic_int_t,
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_set_relaxed(
    mut x: *mut cairo_atomic_int_t,
    mut val: cairo_atomic_int_t,
) {
    ::std::intrinsics::atomic_store_relaxed(x, val);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_get(
    mut x: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_cmpxchg_impl(
    mut x: *mut *mut libc::c_void,
    mut oldv: *mut libc::c_void,
    mut newv: *mut libc::c_void,
) -> cairo_bool_t {
    let mut expected: *mut libc::c_void = oldv;
    let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh0.0;
    return fresh0.1 as cairo_bool_t;
}
#[inline(always)]
unsafe extern "C" fn _atomic_fetch(
    mut slot: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        ptr = _cairo_atomic_ptr_get(slot);
        if !(_cairo_atomic_ptr_cmpxchg_impl(slot, ptr, 0 as *mut libc::c_void) == 0) {
            break;
        }
    }
    return ptr;
}
#[inline(always)]
unsafe extern "C" fn _atomic_store(
    mut slot: *mut *mut libc::c_void,
    mut ptr: *mut libc::c_void,
) -> cairo_bool_t {
    return _cairo_atomic_ptr_cmpxchg_impl(slot, 0 as *mut libc::c_void, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn _freed_pool_get_search(
    mut pool: *mut freed_pool_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    i = (::std::mem::size_of::<[*mut libc::c_void; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        as libc::c_int;
    loop {
        let fresh1 = i;
        i = i - 1;
        if !(fresh1 != 0) {
            break;
        }
        ptr = _atomic_fetch(&mut *((*pool).pool).as_mut_ptr().offset(i as isize));
        if !ptr.is_null() {
            _cairo_atomic_int_set_relaxed(&mut (*pool).top, i);
            return ptr;
        }
    }
    _cairo_atomic_int_set_relaxed(&mut (*pool).top, 0 as libc::c_int);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _freed_pool_put_search(
    mut pool: *mut freed_pool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[*mut libc::c_void; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            as libc::c_int
    {
        if _atomic_store(&mut *((*pool).pool).as_mut_ptr().offset(i as isize), ptr) != 0
        {
            _cairo_atomic_int_set_relaxed(&mut (*pool).top, i + 1 as libc::c_int);
            return;
        }
        i += 1;
    }
    _cairo_atomic_int_set_relaxed(&mut (*pool).top, i);
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn _freed_pool_reset(mut pool: *mut freed_pool_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[*mut libc::c_void; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            as libc::c_int
    {
        free((*pool).pool[i as usize]);
        let ref mut fresh2 = (*pool).pool[i as usize];
        *fresh2 = 0 as *mut libc::c_void;
        i += 1;
    }
    _cairo_atomic_int_set_relaxed(&mut (*pool).top, 0 as libc::c_int);
}
