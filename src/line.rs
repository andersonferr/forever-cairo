use ::libc;
extern "C" {
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
}
pub type __int128_t = i128;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_line {
    pub p1: cairo_point_t,
    pub p2: cairo_point_t,
}
pub type cairo_point_t = _cairo_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point {
    pub x: cairo_fixed_t,
    pub y: cairo_fixed_t,
}
pub type cairo_fixed_t = int32_t;
pub type cairo_int64_t = int64_t;
pub type int128_t = __int128_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
pub type cairo_line_t = _cairo_line;
pub const HAVE_BOTH: C2RustUnnamed_0 = 3;
pub const HAVE_BX: C2RustUnnamed_0 = 2;
pub const HAVE_AX: C2RustUnnamed_0 = 1;
pub const HAVE_ALL: C2RustUnnamed = 7;
pub const HAVE_DX_BDX: C2RustUnnamed = 5;
pub const HAVE_DX_ADX: C2RustUnnamed = 3;
pub const HAVE_ADX_BDX: C2RustUnnamed = 6;
pub const HAVE_BDX: C2RustUnnamed = 4;
pub const HAVE_ADX: C2RustUnnamed = 2;
pub const HAVE_DX: C2RustUnnamed = 1;
pub const HAVE_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const HAVE_NEITHER: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn cairo_lines_equal(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
) -> libc::c_int {
    return ((*a).p1.x == (*b).p1.x && (*a).p1.y == (*b).p1.y && (*a).p2.x == (*b).p2.x
        && (*a).p2.y == (*b).p2.y) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_slope_init(
    mut slope: *mut cairo_slope_t,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
) {
    (*slope).dx = (*b).x - (*a).x;
    (*slope).dy = (*b).y - (*a).y;
}
unsafe extern "C" fn line_compare_for_y_against_x(
    mut a: *const cairo_line_t,
    mut y: int32_t,
    mut x: int32_t,
) -> libc::c_int {
    let mut adx: int32_t = 0;
    let mut ady: int32_t = 0;
    let mut dx: int32_t = 0;
    let mut dy: int32_t = 0;
    let mut L: cairo_int64_t = 0;
    let mut R: cairo_int64_t = 0;
    if x < (*a).p1.x && x < (*a).p2.x {
        return 1 as libc::c_int;
    }
    if x > (*a).p1.x && x > (*a).p2.x {
        return -(1 as libc::c_int);
    }
    adx = (*a).p2.x - (*a).p1.x;
    dx = x - (*a).p1.x;
    if adx == 0 as libc::c_int {
        return -dx;
    }
    if dx == 0 as libc::c_int || adx ^ dx < 0 as libc::c_int {
        return adx;
    }
    dy = y - (*a).p1.y;
    ady = (*a).p2.y - (*a).p1.y;
    L = dy as int64_t * adx as libc::c_long;
    R = dx as int64_t * ady as libc::c_long;
    return if L == R {
        0 as libc::c_int
    } else if L < R {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn lines_compare_x_for_y_general(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
    mut y: int32_t,
) -> libc::c_int {
    let mut dx: int32_t = 0 as libc::c_int;
    let mut adx: int32_t = 0 as libc::c_int;
    let mut ady: int32_t = 0 as libc::c_int;
    let mut bdx: int32_t = 0 as libc::c_int;
    let mut bdy: int32_t = 0 as libc::c_int;
    let mut have_dx_adx_bdx: C2RustUnnamed = HAVE_ALL;
    ady = (*a).p2.y - (*a).p1.y;
    adx = (*a).p2.x - (*a).p1.x;
    if adx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_ADX as libc::c_int) as libc::c_uint);
    }
    bdy = (*b).p2.y - (*b).p1.y;
    bdx = (*b).p2.x - (*b).p1.x;
    if bdx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_BDX as libc::c_int) as libc::c_uint);
    }
    dx = (*a).p1.x - (*b).p1.x;
    if dx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_DX as libc::c_int) as libc::c_uint);
    }
    match have_dx_adx_bdx as libc::c_uint {
        1 => return dx,
        2 => return adx,
        4 => return -bdx,
        6 => {
            if adx ^ bdx < 0 as libc::c_int {
                return adx
            } else if (*a).p1.y == (*b).p1.y {
                let mut adx_bdy: cairo_int64_t = 0;
                let mut bdx_ady: cairo_int64_t = 0;
                adx_bdy = adx as int64_t * bdy as libc::c_long;
                bdx_ady = bdx as int64_t * ady as libc::c_long;
                return if adx_bdy == bdx_ady {
                    0 as libc::c_int
                } else if adx_bdy < bdx_ady {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            } else {
                return if (adx as int64_t * bdy as libc::c_long) as int128_t
                    * (y - (*a).p1.y) as int64_t as i128
                    == (bdx as int64_t * ady as libc::c_long) as int128_t
                        * (y - (*b).p1.y) as int64_t as i128
                {
                    0 as libc::c_int
                } else if ((adx as int64_t * bdy as libc::c_long) as int128_t
                    * (y - (*a).p1.y) as int64_t as i128)
                    < (bdx as int64_t * ady as libc::c_long) as int128_t
                        * (y - (*b).p1.y) as int64_t as i128
                {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }
            }
        }
        3 => {
            if -adx ^ dx < 0 as libc::c_int {
                return dx
            } else {
                let mut ady_dx: cairo_int64_t = 0;
                let mut dy_adx: cairo_int64_t = 0;
                ady_dx = ady as int64_t * dx as libc::c_long;
                dy_adx = ((*a).p1.y - y) as int64_t * adx as libc::c_long;
                return if ady_dx == dy_adx {
                    0 as libc::c_int
                } else if ady_dx < dy_adx {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
        }
        5 => {
            if bdx ^ dx < 0 as libc::c_int {
                return dx
            } else {
                let mut bdy_dx: cairo_int64_t = 0;
                let mut dy_bdx: cairo_int64_t = 0;
                bdy_dx = bdy as int64_t * dx as libc::c_long;
                dy_bdx = (y - (*b).p1.y) as int64_t * bdx as libc::c_long;
                return if bdy_dx == dy_bdx {
                    0 as libc::c_int
                } else if bdy_dx < dy_bdx {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
        }
        7 => {
            return if (ady as int64_t * bdy as libc::c_long) as int128_t
                * dx as int64_t as i128
                == (bdx as int64_t * ady as libc::c_long) as int128_t
                    * (y - (*b).p1.y) as int64_t as i128
                    - (adx as int64_t * bdy as libc::c_long) as int128_t
                        * (y - (*a).p1.y) as int64_t as i128
            {
                0 as libc::c_int
            } else if ((ady as int64_t * bdy as libc::c_long) as int128_t
                * dx as int64_t as i128)
                < (bdx as int64_t * ady as libc::c_long) as int128_t
                    * (y - (*b).p1.y) as int64_t as i128
                    - (adx as int64_t * bdy as libc::c_long) as int128_t
                        * (y - (*a).p1.y) as int64_t as i128
            {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            };
        }
        0 | _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn lines_compare_x_for_y(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
    mut y: int32_t,
) -> libc::c_int {
    let mut have_ax_bx: C2RustUnnamed_0 = HAVE_BOTH;
    let mut ax: int32_t = 0 as libc::c_int;
    let mut bx: int32_t = 0 as libc::c_int;
    if y == (*a).p1.y {
        ax = (*a).p1.x;
    } else if y == (*a).p2.y {
        ax = (*a).p2.x;
    } else {
        have_ax_bx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_ax_bx as libc::c_uint & !(HAVE_AX as libc::c_int) as libc::c_uint);
    }
    if y == (*b).p1.y {
        bx = (*b).p1.x;
    } else if y == (*b).p2.y {
        bx = (*b).p2.x;
    } else {
        have_ax_bx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_ax_bx as libc::c_uint & !(HAVE_BX as libc::c_int) as libc::c_uint);
    }
    match have_ax_bx as libc::c_uint {
        1 => return -line_compare_for_y_against_x(b, y, ax),
        2 => return line_compare_for_y_against_x(a, y, bx),
        3 => return ax - bx,
        0 | _ => return lines_compare_x_for_y_general(a, b, y),
    };
}
unsafe extern "C" fn bbox_compare(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
) -> libc::c_int {
    let mut amin: int32_t = 0;
    let mut amax: int32_t = 0;
    let mut bmin: int32_t = 0;
    let mut bmax: int32_t = 0;
    if (*a).p1.x < (*a).p2.x {
        amin = (*a).p1.x;
        amax = (*a).p2.x;
    } else {
        amin = (*a).p2.x;
        amax = (*a).p1.x;
    }
    if (*b).p1.x < (*b).p2.x {
        bmin = (*b).p1.x;
        bmax = (*b).p2.x;
    } else {
        bmin = (*b).p2.x;
        bmax = (*b).p1.x;
    }
    if amax < bmin {
        return -(1 as libc::c_int);
    }
    if amin > bmax {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_lines_compare_at_y(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut sa: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut sb: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut ret: libc::c_int = 0;
    if cairo_lines_equal(a, b) != 0 {
        return 0 as libc::c_int;
    }
    ret = bbox_compare(a, b);
    if ret != 0 {
        return ret;
    }
    ret = lines_compare_x_for_y(a, b, y);
    if ret != 0 {
        return ret;
    }
    _cairo_slope_init(&mut sa, &(*a).p1, &(*a).p2);
    _cairo_slope_init(&mut sb, &(*b).p1, &(*b).p2);
    return _cairo_slope_compare(&mut sb, &mut sa);
}
