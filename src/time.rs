use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type cairo_int64_t = int64_t;
pub type cairo_time_t = cairo_int64_t;
#[inline(always)]
unsafe extern "C" fn _cairo_double_to_int64(mut i: libc::c_double) -> cairo_int64_t {
    return i as cairo_int64_t;
}
#[inline(always)]
unsafe extern "C" fn _cairo_int64_to_double(mut i: cairo_int64_t) -> libc::c_double {
    return i as libc::c_double;
}
#[inline(always)]
unsafe extern "C" fn _cairo_time_1s() -> libc::c_double {
    return 1000000000 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_time_get() -> cairo_time_t {
    let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut r: cairo_time_t = 0;
    clock_gettime(4 as libc::c_int, &mut t);
    r = _cairo_double_to_int64(_cairo_time_1s());
    r = r * t.tv_sec;
    r = r + t.tv_nsec;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_time_cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ta: *const cairo_time_t = a as *const cairo_time_t;
    let mut tb: *const cairo_time_t = b as *const cairo_time_t;
    return if *ta == *tb {
        0 as libc::c_int
    } else if *ta < *tb {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn _cairo_time_ticks_per_sec() -> libc::c_double {
    static mut ticks: libc::c_double = 0 as libc::c_int as libc::c_double;
    if ticks == 0 as libc::c_int as libc::c_double {
        ticks = _cairo_time_1s();
    }
    return ticks;
}
unsafe extern "C" fn _cairo_time_s_per_tick() -> libc::c_double {
    static mut s: libc::c_double = 0 as libc::c_int as libc::c_double;
    if s == 0 as libc::c_int as libc::c_double {
        s = 1.0f64 / _cairo_time_ticks_per_sec();
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_time_to_s(mut t: cairo_time_t) -> libc::c_double {
    return _cairo_int64_to_double(t) * _cairo_time_s_per_tick();
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_time_from_s(mut t: libc::c_double) -> cairo_time_t {
    return _cairo_double_to_int64(t * _cairo_time_ticks_per_sec());
}
