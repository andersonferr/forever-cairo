use ::libc;
extern "C" {
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_spline_bound(
        add_point_func: cairo_spline_add_point_func_t,
        closure: *mut libc::c_void,
        p0: *const cairo_point_t,
        p1: *const cairo_point_t,
        p2: *const cairo_point_t,
        p3: *const cairo_point_t,
    ) -> cairo_status_t;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type cairo_bool_t = libc::c_int;
pub type cairo_status_t = _cairo_status;
pub type _cairo_status = libc::c_uint;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 44;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub type cairo_box_t = _cairo_line;
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
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_int {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
pub type cairo_int64_t = int64_t;
pub type cairo_fixed_unsigned_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
pub type cairo_line_t = _cairo_line;
pub type cairo_spline_add_point_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const cairo_point_t,
        *const cairo_slope_t,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_double(mut d: libc::c_double) -> cairo_fixed_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
    u
        .d = d
        + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
            as libc::c_double * 1.5f64;
    return u.i[0 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_ceil(mut f: cairo_fixed_t) -> libc::c_int {
    if f > 0 as libc::c_int {
        return (f - 1 as libc::c_int >> 8 as libc::c_int) + 1 as libc::c_int
    } else {
        return -((f as cairo_fixed_unsigned_t).wrapping_neg() as cairo_fixed_t
            >> 8 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn _cairo_box_add_point(
    mut box_0: *mut cairo_box_t,
    mut point: *const cairo_point_t,
) {
    if (*point).x < (*box_0).p1.x {
        (*box_0).p1.x = (*point).x;
    } else if (*point).x > (*box_0).p2.x {
        (*box_0).p2.x = (*point).x;
    }
    if (*point).y < (*box_0).p1.y {
        (*box_0).p1.y = (*point).y;
    } else if (*point).y > (*box_0).p2.y {
        (*box_0).p2.y = (*point).y;
    }
}
#[inline]
unsafe extern "C" fn _cairo_box_add_box(
    mut box_0: *mut cairo_box_t,
    mut add: *const cairo_box_t,
) {
    if (*add).p1.x < (*box_0).p1.x {
        (*box_0).p1.x = (*add).p1.x;
    }
    if (*add).p2.x > (*box_0).p2.x {
        (*box_0).p2.x = (*add).p2.x;
    }
    if (*add).p1.y < (*box_0).p1.y {
        (*box_0).p1.y = (*add).p1.y;
    }
    if (*add).p2.y > (*box_0).p2.y {
        (*box_0).p2.y = (*add).p2.y;
    }
}
#[inline]
unsafe extern "C" fn _cairo_box_contains_point(
    mut box_0: *const cairo_box_t,
    mut point: *const cairo_point_t,
) -> cairo_bool_t {
    return ((*box_0).p1.x <= (*point).x && (*point).x <= (*box_0).p2.x
        && (*box_0).p1.y <= (*point).y && (*point).y <= (*box_0).p2.y) as libc::c_int;
}
#[no_mangle]
pub static mut _cairo_empty_rectangle: cairo_rectangle_int_t = {
    let mut init = _cairo_rectangle_int {
        x: 0 as libc::c_int,
        y: 0 as libc::c_int,
        width: 0 as libc::c_int,
        height: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut _cairo_unbounded_rectangle: cairo_rectangle_int_t = {
    let mut init = _cairo_rectangle_int {
        x: -(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int,
        y: -(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int,
        width: (2147483647 as libc::c_int >> 8 as libc::c_int)
            - (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int),
        height: (2147483647 as libc::c_int >> 8 as libc::c_int)
            - (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int),
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_box_from_doubles(
    mut box_0: *mut cairo_box_t,
    mut x1: *mut libc::c_double,
    mut y1: *mut libc::c_double,
    mut x2: *mut libc::c_double,
    mut y2: *mut libc::c_double,
) {
    (*box_0).p1.x = _cairo_fixed_from_double(*x1);
    (*box_0).p1.y = _cairo_fixed_from_double(*y1);
    (*box_0).p2.x = _cairo_fixed_from_double(*x2);
    (*box_0).p2.y = _cairo_fixed_from_double(*y2);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_box_to_doubles(
    mut box_0: *const cairo_box_t,
    mut x1: *mut libc::c_double,
    mut y1: *mut libc::c_double,
    mut x2: *mut libc::c_double,
    mut y2: *mut libc::c_double,
) {
    *x1 = _cairo_fixed_to_double((*box_0).p1.x);
    *y1 = _cairo_fixed_to_double((*box_0).p1.y);
    *x2 = _cairo_fixed_to_double((*box_0).p2.x);
    *y2 = _cairo_fixed_to_double((*box_0).p2.y);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_box_from_rectangle(
    mut box_0: *mut cairo_box_t,
    mut rect: *const cairo_rectangle_int_t,
) {
    (*box_0).p1.x = _cairo_fixed_from_int((*rect).x);
    (*box_0).p1.y = _cairo_fixed_from_int((*rect).y);
    (*box_0).p2.x = _cairo_fixed_from_int((*rect).x + (*rect).width);
    (*box_0).p2.y = _cairo_fixed_from_int((*rect).y + (*rect).height);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_get_extents(
    mut boxes: *const cairo_box_t,
    mut num_boxes: libc::c_int,
    mut extents: *mut cairo_box_t,
) {
    if num_boxes > 0 as libc::c_int {} else {
        __assert_fail(
            b"num_boxes > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-rectangle.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void _cairo_boxes_get_extents(const cairo_box_t *, int, cairo_box_t *)\0",
            ))
                .as_ptr(),
        );
    }
    *extents = *boxes;
    loop {
        num_boxes -= 1;
        if !(num_boxes != 0) {
            break;
        }
        boxes = boxes.offset(1);
        _cairo_box_add_box(extents, boxes);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_box_round_to_rectangle(
    mut box_0: *const cairo_box_t,
    mut rectangle: *mut cairo_rectangle_int_t,
) {
    (*rectangle).x = _cairo_fixed_integer_floor((*box_0).p1.x);
    (*rectangle).y = _cairo_fixed_integer_floor((*box_0).p1.y);
    (*rectangle).width = _cairo_fixed_integer_ceil((*box_0).p2.x) - (*rectangle).x;
    (*rectangle).height = _cairo_fixed_integer_ceil((*box_0).p2.y) - (*rectangle).y;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rectangle_intersect(
    mut dst: *mut cairo_rectangle_int_t,
    mut src: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    x1 = if (*dst).x > (*src).x { (*dst).x } else { (*src).x };
    y1 = if (*dst).y > (*src).y { (*dst).y } else { (*src).y };
    x2 = if (*dst).x + (*dst).width < (*src).x + (*src).width {
        (*dst).x + (*dst).width
    } else {
        (*src).x + (*src).width
    };
    y2 = if (*dst).y + (*dst).height < (*src).y + (*src).height {
        (*dst).y + (*dst).height
    } else {
        (*src).y + (*src).height
    };
    if x1 >= x2 || y1 >= y2 {
        (*dst).x = 0 as libc::c_int;
        (*dst).y = 0 as libc::c_int;
        (*dst).width = 0 as libc::c_int;
        (*dst).height = 0 as libc::c_int;
        return 0 as libc::c_int;
    } else {
        (*dst).x = x1;
        (*dst).y = y1;
        (*dst).width = x2 - x1;
        (*dst).height = y2 - y1;
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rectangle_union(
    mut dst: *mut cairo_rectangle_int_t,
    mut src: *const cairo_rectangle_int_t,
) {
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    x1 = if (*dst).x < (*src).x { (*dst).x } else { (*src).x };
    y1 = if (*dst).y < (*src).y { (*dst).y } else { (*src).y };
    x2 = if (*dst).x + (*dst).width > (*src).x + (*src).width {
        (*dst).x + (*dst).width
    } else {
        (*src).x + (*src).width
    };
    y2 = if (*dst).y + (*dst).height > (*src).y + (*src).height {
        (*dst).y + (*dst).height
    } else {
        (*src).y + (*src).height
    };
    (*dst).x = x1;
    (*dst).y = y1;
    (*dst).width = x2 - x1;
    (*dst).height = y2 - y1;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_box_intersects_line_segment(
    mut box_0: *const cairo_box_t,
    mut line: *mut cairo_line_t,
) -> cairo_bool_t {
    let mut t1: cairo_fixed_t = 0 as libc::c_int;
    let mut t2: cairo_fixed_t = 0 as libc::c_int;
    let mut t3: cairo_fixed_t = 0 as libc::c_int;
    let mut t4: cairo_fixed_t = 0 as libc::c_int;
    let mut t1y: cairo_int64_t = 0;
    let mut t2y: cairo_int64_t = 0;
    let mut t3x: cairo_int64_t = 0;
    let mut t4x: cairo_int64_t = 0;
    let mut xlen: cairo_fixed_t = 0;
    let mut ylen: cairo_fixed_t = 0;
    if _cairo_box_contains_point(box_0, &mut (*line).p1) != 0
        || _cairo_box_contains_point(box_0, &mut (*line).p2) != 0
    {
        return 1 as libc::c_int;
    }
    xlen = (*line).p2.x - (*line).p1.x;
    ylen = (*line).p2.y - (*line).p1.y;
    if xlen != 0 {
        if xlen > 0 as libc::c_int {
            t1 = (*box_0).p1.x - (*line).p1.x;
            t2 = (*box_0).p2.x - (*line).p1.x;
        } else {
            t1 = (*line).p1.x - (*box_0).p2.x;
            t2 = (*line).p1.x - (*box_0).p1.x;
            xlen = -xlen;
        }
        if (t1 < 0 as libc::c_int || t1 > xlen) && (t2 < 0 as libc::c_int || t2 > xlen) {
            return 0 as libc::c_int;
        }
    } else if (*line).p1.x < (*box_0).p1.x || (*line).p1.x > (*box_0).p2.x {
        return 0 as libc::c_int
    }
    if ylen != 0 {
        if ylen > 0 as libc::c_int {
            t3 = (*box_0).p1.y - (*line).p1.y;
            t4 = (*box_0).p2.y - (*line).p1.y;
        } else {
            t3 = (*line).p1.y - (*box_0).p2.y;
            t4 = (*line).p1.y - (*box_0).p1.y;
            ylen = -ylen;
        }
        if (t3 < 0 as libc::c_int || t3 > ylen) && (t4 < 0 as libc::c_int || t4 > ylen) {
            return 0 as libc::c_int;
        }
    } else if (*line).p1.y < (*box_0).p1.y || (*line).p1.y > (*box_0).p2.y {
        return 0 as libc::c_int
    }
    if (*line).p1.x == (*line).p2.x || (*line).p1.y == (*line).p2.y {
        return 1 as libc::c_int;
    }
    t1y = t1 as int64_t * ylen as libc::c_long;
    t2y = t2 as int64_t * ylen as libc::c_long;
    t3x = t3 as int64_t * xlen as libc::c_long;
    t4x = t4 as int64_t * xlen as libc::c_long;
    if t1y < t4x && t3x < t2y {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_box_add_spline_point(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    _cairo_box_add_point(closure as *mut cairo_box_t, point);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_box_add_curve_to(
    mut extents: *mut cairo_box_t,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) {
    _cairo_box_add_point(extents, d);
    if _cairo_box_contains_point(extents, b) == 0
        || _cairo_box_contains_point(extents, c) == 0
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_spline_bound(
            Some(
                _cairo_box_add_spline_point
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                        *const cairo_slope_t,
                    ) -> cairo_status_t,
            ),
            extents as *mut libc::c_void,
            a,
            b,
            c,
            d,
        );
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-rectangle.c\0" as *const u8 as *const libc::c_char,
                287 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 136],
                    &[libc::c_char; 136],
                >(
                    b"void _cairo_box_add_curve_to(cairo_box_t *, const cairo_point_t *, const cairo_point_t *, const cairo_point_t *, const cairo_point_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rectangle_int_from_double(
    mut recti: *mut cairo_rectangle_int_t,
    mut rectf: *const cairo_rectangle_t,
) {
    (*recti).x = floor((*rectf).x) as libc::c_int;
    (*recti).y = floor((*rectf).y) as libc::c_int;
    (*recti)
        .width = (ceil((*rectf).x + (*rectf).width) - floor((*rectf).x)) as libc::c_int;
    (*recti)
        .height = (ceil((*rectf).y + (*rectf).height) - floor((*rectf).y))
        as libc::c_int;
}
