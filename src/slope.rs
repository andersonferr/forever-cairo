use ::libc;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type cairo_fixed_t = int32_t;
pub type cairo_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
#[no_mangle]
pub unsafe extern "C" fn _cairo_slope_compare(
    mut a: *const cairo_slope_t,
    mut b: *const cairo_slope_t,
) -> libc::c_int {
    let mut ady_bdx: cairo_int64_t = (*a).dy as int64_t * (*b).dx as libc::c_long;
    let mut bdy_adx: cairo_int64_t = (*b).dy as int64_t * (*a).dx as libc::c_long;
    let mut cmp: libc::c_int = 0;
    cmp = if ady_bdx == bdy_adx {
        0 as libc::c_int
    } else if ady_bdx < bdy_adx {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    if cmp != 0 {
        return cmp;
    }
    if (*a).dx == 0 as libc::c_int && (*a).dy == 0 as libc::c_int
        && (*b).dx == 0 as libc::c_int && (*b).dy == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*a).dx == 0 as libc::c_int && (*a).dy == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*b).dx == 0 as libc::c_int && (*b).dy == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*a).dx ^ (*b).dx < 0 as libc::c_int || (*a).dy ^ (*b).dy < 0 as libc::c_int {
        if (*a).dx > 0 as libc::c_int
            || (*a).dx == 0 as libc::c_int && (*a).dy > 0 as libc::c_int
        {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
