use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_spline_decompose(
        spline: *mut cairo_spline_t,
        tolerance: libc::c_double,
    ) -> cairo_status_t;
    fn _cairo_spline_init(
        spline: *mut cairo_spline_t,
        add_point_func: cairo_spline_add_point_func_t,
        closure: *mut libc::c_void,
        a: *const cairo_point_t,
        b: *const cairo_point_t,
        c: *const cairo_point_t,
        d: *const cairo_point_t,
    ) -> cairo_bool_t;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type cairo_bool_t = libc::c_int;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
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
pub type cairo_fill_rule_t = _cairo_fill_rule;
pub type _cairo_fill_rule = libc::c_uint;
pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;
pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
pub type cairo_path_fixed_t = _cairo_path_fixed;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_path_fixed {
    pub last_move_point: cairo_point_t,
    pub current_point: cairo_point_t,
    #[bitfield(name = "has_current_point", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "needs_move_to", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "has_extents", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "has_curve_to", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "stroke_is_rectilinear", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "fill_is_rectilinear", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "fill_maybe_region", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fill_is_empty", ty = "libc::c_uint", bits = "7..=7")]
    pub has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub extents: cairo_box_t,
    pub buf: cairo_path_buf_fixed_t,
}
pub type cairo_path_buf_fixed_t = _cairo_path_buf_fixed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_buf_fixed {
    pub base: cairo_path_buf_t,
    pub op: [cairo_path_op_t; 27],
    pub points: [cairo_point_t; 54],
}
pub type cairo_path_op_t = libc::c_char;
pub type cairo_path_buf_t = _cairo_path_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_buf {
    pub link: cairo_list_t,
    pub num_ops: libc::c_uint,
    pub size_ops: libc::c_uint,
    pub num_points: libc::c_uint,
    pub size_points: libc::c_uint,
    pub op: *mut cairo_path_op_t,
    pub points: *mut cairo_point_t,
}
pub type cairo_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
pub type cairo_spline_add_point_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const cairo_point_t,
        *const cairo_slope_t,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_spline_knots {
    pub a: cairo_point_t,
    pub b: cairo_point_t,
    pub c: cairo_point_t,
    pub d: cairo_point_t,
}
pub type cairo_spline_knots_t = _cairo_spline_knots;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_spline {
    pub add_point_func: cairo_spline_add_point_func_t,
    pub closure: *mut libc::c_void,
    pub knots: cairo_spline_knots_t,
    pub initial_slope: cairo_slope_t,
    pub final_slope: cairo_slope_t,
    pub has_point: cairo_bool_t,
    pub last_point: cairo_point_t,
}
pub type cairo_spline_t = _cairo_spline;
pub type cairo_path_fixed_move_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_line_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_curve_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
    *const cairo_point_t,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_close_path_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> cairo_status_t;
pub type cairo_in_fill_t = cairo_in_fill;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_in_fill {
    pub tolerance: libc::c_double,
    pub on_edge: cairo_bool_t,
    pub winding: libc::c_int,
    pub x: cairo_fixed_t,
    pub y: cairo_fixed_t,
    pub has_current_point: cairo_bool_t,
    pub current_point: cairo_point_t,
    pub first_point: cairo_point_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_fill_is_empty(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).fill_is_empty() as cairo_bool_t;
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
unsafe extern "C" fn _cairo_in_fill_init(
    mut in_fill: *mut cairo_in_fill_t,
    mut tolerance: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    (*in_fill).on_edge = 0 as libc::c_int;
    (*in_fill).winding = 0 as libc::c_int;
    (*in_fill).tolerance = tolerance;
    (*in_fill).x = _cairo_fixed_from_double(x);
    (*in_fill).y = _cairo_fixed_from_double(y);
    (*in_fill).has_current_point = 0 as libc::c_int;
    (*in_fill).current_point.x = 0 as libc::c_int;
    (*in_fill).current_point.y = 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_in_fill_fini(mut in_fill: *mut cairo_in_fill_t) {}
unsafe extern "C" fn edge_compare_for_y_against_x(
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut y: cairo_fixed_t,
    mut x: cairo_fixed_t,
) -> libc::c_int {
    let mut adx: cairo_fixed_t = 0;
    let mut ady: cairo_fixed_t = 0;
    let mut dx: cairo_fixed_t = 0;
    let mut dy: cairo_fixed_t = 0;
    let mut L: cairo_int64_t = 0;
    let mut R: cairo_int64_t = 0;
    adx = (*p2).x - (*p1).x;
    dx = x - (*p1).x;
    if adx == 0 as libc::c_int {
        return -dx;
    }
    if adx ^ dx < 0 as libc::c_int {
        return adx;
    }
    dy = y - (*p1).y;
    ady = (*p2).y - (*p1).y;
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
unsafe extern "C" fn _cairo_in_fill_add_edge(
    mut in_fill: *mut cairo_in_fill_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) {
    let mut dir: libc::c_int = 0;
    if (*in_fill).on_edge != 0 {
        return;
    }
    dir = 1 as libc::c_int;
    if (*p2).y < (*p1).y {
        let mut tmp: *const cairo_point_t = 0 as *const cairo_point_t;
        tmp = p1;
        p1 = p2;
        p2 = tmp;
        dir = -(1 as libc::c_int);
    }
    if (*p1).x == (*in_fill).x && (*p1).y == (*in_fill).y
        || (*p2).x == (*in_fill).x && (*p2).y == (*in_fill).y
        || !((*p2).y < (*in_fill).y || (*p1).y > (*in_fill).y
            || (*p1).x > (*in_fill).x && (*p2).x > (*in_fill).x
            || (*p1).x < (*in_fill).x && (*p2).x < (*in_fill).x)
            && edge_compare_for_y_against_x(p1, p2, (*in_fill).y, (*in_fill).x)
                == 0 as libc::c_int
    {
        (*in_fill).on_edge = 1 as libc::c_int;
        return;
    }
    if (*p2).y <= (*in_fill).y || (*p1).y > (*in_fill).y {
        return;
    }
    if (*p1).x >= (*in_fill).x && (*p2).x >= (*in_fill).x {
        return;
    }
    if (*p1).x <= (*in_fill).x && (*p2).x <= (*in_fill).x
        || edge_compare_for_y_against_x(p1, p2, (*in_fill).y, (*in_fill).x)
            < 0 as libc::c_int
    {
        (*in_fill).winding += dir;
    }
}
unsafe extern "C" fn _cairo_in_fill_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut in_fill: *mut cairo_in_fill_t = closure as *mut cairo_in_fill_t;
    if (*in_fill).has_current_point != 0 {
        _cairo_in_fill_add_edge(
            in_fill,
            &mut (*in_fill).current_point,
            &mut (*in_fill).first_point,
        );
    }
    (*in_fill).first_point = *point;
    (*in_fill).current_point = *point;
    (*in_fill).has_current_point = 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_in_fill_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut in_fill: *mut cairo_in_fill_t = closure as *mut cairo_in_fill_t;
    if (*in_fill).has_current_point != 0 {
        _cairo_in_fill_add_edge(in_fill, &mut (*in_fill).current_point, point);
    }
    (*in_fill).current_point = *point;
    (*in_fill).has_current_point = 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_in_fill_add_point(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    return _cairo_in_fill_line_to(closure, point);
}
unsafe extern "C" fn _cairo_in_fill_curve_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_status_t {
    let mut in_fill: *mut cairo_in_fill_t = closure as *mut cairo_in_fill_t;
    let mut spline: cairo_spline_t = cairo_spline_t {
        add_point_func: None,
        closure: 0 as *mut libc::c_void,
        knots: cairo_spline_knots_t {
            a: cairo_point_t { x: 0, y: 0 },
            b: cairo_point_t { x: 0, y: 0 },
            c: cairo_point_t { x: 0, y: 0 },
            d: cairo_point_t { x: 0, y: 0 },
        },
        initial_slope: cairo_slope_t { dx: 0, dy: 0 },
        final_slope: cairo_slope_t { dx: 0, dy: 0 },
        has_point: 0,
        last_point: cairo_point_t { x: 0, y: 0 },
    };
    let mut top: cairo_fixed_t = 0;
    let mut bot: cairo_fixed_t = 0;
    let mut left: cairo_fixed_t = 0;
    top = (*in_fill).current_point.y;
    bot = top;
    if (*b).y < top {
        top = (*b).y;
    }
    if (*b).y > bot {
        bot = (*b).y;
    }
    if (*c).y < top {
        top = (*c).y;
    }
    if (*c).y > bot {
        bot = (*c).y;
    }
    if (*d).y < top {
        top = (*d).y;
    }
    if (*d).y > bot {
        bot = (*d).y;
    }
    if bot < (*in_fill).y || top > (*in_fill).y {
        (*in_fill).current_point = *d;
        return CAIRO_STATUS_SUCCESS;
    }
    left = (*in_fill).current_point.x;
    if (*b).x < left {
        left = (*b).x;
    }
    if (*c).x < left {
        left = (*c).x;
    }
    if (*d).x < left {
        left = (*d).x;
    }
    if left > (*in_fill).x {
        (*in_fill).current_point = *d;
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_spline_init(
        &mut spline,
        Some(
            _cairo_in_fill_add_point
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        ),
        in_fill as *mut libc::c_void,
        &mut (*in_fill).current_point,
        b,
        c,
        d,
    ) == 0
    {
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_spline_decompose(&mut spline, (*in_fill).tolerance);
}
unsafe extern "C" fn _cairo_in_fill_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut in_fill: *mut cairo_in_fill_t = closure as *mut cairo_in_fill_t;
    if (*in_fill).has_current_point != 0 {
        _cairo_in_fill_add_edge(
            in_fill,
            &mut (*in_fill).current_point,
            &mut (*in_fill).first_point,
        );
        (*in_fill).has_current_point = 0 as libc::c_int;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_in_fill(
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> cairo_bool_t {
    let mut in_fill: cairo_in_fill_t = cairo_in_fill_t {
        tolerance: 0.,
        on_edge: 0,
        winding: 0,
        x: 0,
        y: 0,
        has_current_point: 0,
        current_point: cairo_point_t { x: 0, y: 0 },
        first_point: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut is_inside: cairo_bool_t = 0;
    if _cairo_path_fixed_fill_is_empty(path) != 0 {
        return 0 as libc::c_int;
    }
    _cairo_in_fill_init(&mut in_fill, tolerance, x, y);
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _cairo_in_fill_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_in_fill_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_in_fill_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_in_fill_close_path
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut in_fill as *mut cairo_in_fill_t as *mut libc::c_void,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-in-fill.c\0" as *const u8 as *const libc::c_char,
            276 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"cairo_bool_t _cairo_path_fixed_in_fill(const cairo_path_fixed_t *, cairo_fill_rule_t, double, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_in_fill_close_path(&mut in_fill as *mut cairo_in_fill_t as *mut libc::c_void);
    if in_fill.on_edge != 0 {
        is_inside = 1 as libc::c_int;
    } else {
        match fill_rule as libc::c_uint {
            1 => {
                is_inside = in_fill.winding & 1 as libc::c_int;
            }
            0 => {
                is_inside = (in_fill.winding != 0 as libc::c_int) as libc::c_int;
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-path-in-fill.c\0" as *const u8
                            as *const libc::c_char,
                        290 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 110],
                            &[libc::c_char; 110],
                        >(
                            b"cairo_bool_t _cairo_path_fixed_in_fill(const cairo_path_fixed_t *, cairo_fill_rule_t, double, double, double)\0",
                        ))
                            .as_ptr(),
                    );
                }
                is_inside = 0 as libc::c_int;
            }
        }
    }
    _cairo_in_fill_fini(&mut in_fill);
    return is_inside;
}
