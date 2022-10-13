use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_matrix_has_unity_scale(matrix: *const cairo_matrix_t) -> cairo_bool_t;
    fn _cairo_matrix_transformed_circle_major_axis(
        matrix: *const cairo_matrix_t,
        radius: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type cairo_bool_t = libc::c_int;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
pub type cairo_matrix_t = _cairo_matrix;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_matrix {
    pub xx: libc::c_double,
    pub yx: libc::c_double,
    pub xy: libc::c_double,
    pub yy: libc::c_double,
    pub x0: libc::c_double,
    pub y0: libc::c_double,
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
pub type cairo_stroke_style_t = _cairo_stroke_style;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_stroke_style {
    pub line_width: libc::c_double,
    pub line_cap: cairo_line_cap_t,
    pub line_join: cairo_line_join_t,
    pub miter_limit: libc::c_double,
    pub dash: *mut libc::c_double,
    pub num_dashes: libc::c_uint,
    pub dash_offset: libc::c_double,
    pub is_hairline: cairo_bool_t,
    pub pre_hairline_line_width: libc::c_double,
}
pub type cairo_line_join_t = _cairo_line_join;
pub type _cairo_line_join = libc::c_uint;
pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;
pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
pub type cairo_line_cap_t = _cairo_line_cap;
pub type _cairo_line_cap = libc::c_uint;
pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;
pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_init(mut style: *mut cairo_stroke_style_t) {
    (*style).line_width = 2.0f64;
    (*style).line_cap = CAIRO_LINE_CAP_BUTT;
    (*style).line_join = CAIRO_LINE_JOIN_MITER;
    (*style).miter_limit = 10.0f64;
    let ref mut fresh2 = (*style).dash;
    *fresh2 = 0 as *mut libc::c_double;
    (*style).num_dashes = 0 as libc::c_int as libc::c_uint;
    (*style).dash_offset = 0.0f64;
    (*style).is_hairline = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_init_copy(
    mut style: *mut cairo_stroke_style_t,
    mut other: *const cairo_stroke_style_t,
) -> cairo_status_t {
    (*style).line_width = (*other).line_width;
    (*style).line_cap = (*other).line_cap;
    (*style).line_join = (*other).line_join;
    (*style).miter_limit = (*other).miter_limit;
    (*style).num_dashes = (*other).num_dashes;
    if ((*other).dash).is_null() {
        let ref mut fresh3 = (*style).dash;
        *fresh3 = 0 as *mut libc::c_double;
    } else {
        let ref mut fresh4 = (*style).dash;
        *fresh4 = _cairo_malloc_ab(
            (*style).num_dashes as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if ((*style).dash).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        memcpy(
            (*style).dash as *mut libc::c_void,
            (*other).dash as *const libc::c_void,
            ((*style).num_dashes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    }
    (*style).dash_offset = (*other).dash_offset;
    (*style).is_hairline = (*other).is_hairline;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_fini(mut style: *mut cairo_stroke_style_t) {
    free((*style).dash as *mut libc::c_void);
    let ref mut fresh5 = (*style).dash;
    *fresh5 = 0 as *mut libc::c_double;
    (*style).num_dashes = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_max_distance_from_path(
    mut style: *const cairo_stroke_style_t,
    mut path: *const cairo_path_fixed_t,
    mut ctm: *const cairo_matrix_t,
    mut dx: *mut libc::c_double,
    mut dy: *mut libc::c_double,
) {
    let mut style_expansion: libc::c_double = 0.5f64;
    if (*style).line_cap as libc::c_uint
        == CAIRO_LINE_CAP_SQUARE as libc::c_int as libc::c_uint
    {
        style_expansion = 0.70710678118654752440f64;
    }
    if (*style).line_join as libc::c_uint
        == CAIRO_LINE_JOIN_MITER as libc::c_int as libc::c_uint
        && (*path).stroke_is_rectilinear() == 0
        && style_expansion < 1.41421356237309504880f64 * (*style).miter_limit
    {
        style_expansion = 1.41421356237309504880f64 * (*style).miter_limit;
    }
    style_expansion *= (*style).line_width;
    if _cairo_matrix_has_unity_scale(ctm) != 0 {
        *dy = style_expansion;
        *dx = *dy;
    } else {
        *dx = style_expansion * hypot((*ctm).xx, (*ctm).xy);
        *dy = style_expansion * hypot((*ctm).yy, (*ctm).yx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_max_line_distance_from_path(
    mut style: *const cairo_stroke_style_t,
    mut path: *const cairo_path_fixed_t,
    mut ctm: *const cairo_matrix_t,
    mut dx: *mut libc::c_double,
    mut dy: *mut libc::c_double,
) {
    let mut style_expansion: libc::c_double = 0.5f64 * (*style).line_width;
    if _cairo_matrix_has_unity_scale(ctm) != 0 {
        *dy = style_expansion;
        *dx = *dy;
    } else {
        *dx = style_expansion * hypot((*ctm).xx, (*ctm).xy);
        *dy = style_expansion * hypot((*ctm).yy, (*ctm).yx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_max_join_distance_from_path(
    mut style: *const cairo_stroke_style_t,
    mut path: *const cairo_path_fixed_t,
    mut ctm: *const cairo_matrix_t,
    mut dx: *mut libc::c_double,
    mut dy: *mut libc::c_double,
) {
    let mut style_expansion: libc::c_double = 0.5f64;
    if (*style).line_join as libc::c_uint
        == CAIRO_LINE_JOIN_MITER as libc::c_int as libc::c_uint
        && (*path).stroke_is_rectilinear() == 0
        && style_expansion < 1.41421356237309504880f64 * (*style).miter_limit
    {
        style_expansion = 1.41421356237309504880f64 * (*style).miter_limit;
    }
    style_expansion *= (*style).line_width;
    if _cairo_matrix_has_unity_scale(ctm) != 0 {
        *dy = style_expansion;
        *dx = *dy;
    } else {
        *dx = style_expansion * hypot((*ctm).xx, (*ctm).xy);
        *dy = style_expansion * hypot((*ctm).yy, (*ctm).yx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_dash_period(
    mut style: *const cairo_stroke_style_t,
) -> libc::c_double {
    let mut period: libc::c_double = 0.;
    let mut i: libc::c_uint = 0;
    period = 0.0f64;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*style).num_dashes {
        period += *((*style).dash).offset(i as isize);
        i = i.wrapping_add(1);
    }
    if (*style).num_dashes & 1 as libc::c_int as libc::c_uint != 0 {
        period *= 2.0f64;
    }
    return period;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_dash_stroked(
    mut style: *const cairo_stroke_style_t,
) -> libc::c_double {
    let mut stroked: libc::c_double = 0.;
    let mut cap_scale: libc::c_double = 0.;
    let mut i: libc::c_uint = 0;
    let mut current_block_5: u64;
    match (*style).line_cap as libc::c_uint {
        0 => {
            current_block_5 = 10582330624169799780;
        }
        1 => {
            cap_scale = 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                / 32 as libc::c_int as libc::c_double;
            current_block_5 = 13109137661213826276;
        }
        2 => {
            cap_scale = 1.0f64;
            current_block_5 = 13109137661213826276;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-stroke-style.c\0" as *const u8 as *const libc::c_char,
                    227 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 70],
                        &[libc::c_char; 70],
                    >(
                        b"double _cairo_stroke_style_dash_stroked(const cairo_stroke_style_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_5 = 10582330624169799780;
        }
    }
    match current_block_5 {
        10582330624169799780 => {
            cap_scale = 0.0f64;
        }
        _ => {}
    }
    stroked = 0.0f64;
    if (*style).num_dashes & 1 as libc::c_int as libc::c_uint != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*style).num_dashes {
            stroked
                += *((*style).dash).offset(i as isize)
                    + cap_scale
                        * (if *((*style).dash).offset(i as isize) < (*style).line_width {
                            *((*style).dash).offset(i as isize)
                        } else {
                            (*style).line_width
                        });
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i.wrapping_add(1 as libc::c_int as libc::c_uint) < (*style).num_dashes {
            stroked
                += *((*style).dash).offset(i as isize)
                    + cap_scale
                        * (if *((*style).dash)
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) < (*style).line_width
                        {
                            *((*style).dash)
                                .offset(
                                    i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                )
                        } else {
                            (*style).line_width
                        });
            i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
        }
    }
    return stroked;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_dash_can_approximate(
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
) -> cairo_bool_t {
    let mut period: libc::c_double = 0.;
    if (*style).num_dashes == 0 {
        return 0 as libc::c_int;
    }
    period = _cairo_stroke_style_dash_period(style);
    return (_cairo_matrix_transformed_circle_major_axis(ctm, period) < tolerance)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroke_style_dash_approximate(
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut dash_offset: *mut libc::c_double,
    mut dashes: *mut libc::c_double,
    mut num_dashes: *mut libc::c_uint,
) {
    let mut coverage: libc::c_double = 0.;
    let mut scale: libc::c_double = 0.;
    let mut offset: libc::c_double = 0.;
    let mut on: cairo_bool_t = 1 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    coverage = _cairo_stroke_style_dash_stroked(style)
        / _cairo_stroke_style_dash_period(style);
    coverage = if coverage < 1.0f64 { coverage } else { 1.0f64 };
    scale = tolerance / _cairo_matrix_transformed_circle_major_axis(ctm, 1.0f64);
    offset = (*style).dash_offset;
    while offset > 0.0f64 && offset >= *((*style).dash).offset(i as isize) {
        offset -= *((*style).dash).offset(i as isize);
        on = (on == 0) as libc::c_int;
        i = i.wrapping_add(1);
        if i == (*style).num_dashes {
            i = 0 as libc::c_int as libc::c_uint;
        }
    }
    *num_dashes = 2 as libc::c_int as libc::c_uint;
    match (*style).line_cap as libc::c_uint {
        0 => {
            *dashes.offset(0 as libc::c_int as isize) = scale * coverage;
        }
        1 => {
            *dashes
                .offset(
                    0 as libc::c_int as isize,
                ) = if scale
                * (coverage
                    - 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        / 32 as libc::c_int as libc::c_double)
                / (1.0f64
                    - 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        / 32 as libc::c_int as libc::c_double)
                > scale * coverage
                    - 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        / 32 as libc::c_int as libc::c_double * (*style).line_width
            {
                scale
                    * (coverage
                        - 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                            / 32 as libc::c_int as libc::c_double)
                    / (1.0f64
                        - 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                            / 32 as libc::c_int as libc::c_double)
            } else {
                scale * coverage
                    - 9 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                        / 32 as libc::c_int as libc::c_double * (*style).line_width
            };
        }
        2 => {
            *dashes
                .offset(
                    0 as libc::c_int as isize,
                ) = if 0.0f64 > scale * coverage - (*style).line_width {
                0.0f64
            } else {
                scale * coverage - (*style).line_width
            };
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-stroke-style.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 140],
                        &[libc::c_char; 140],
                    >(
                        b"void _cairo_stroke_style_dash_approximate(const cairo_stroke_style_t *, const cairo_matrix_t *, double, double *, double *, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            *dashes.offset(0 as libc::c_int as isize) = 0.0f64;
        }
    }
    *dashes
        .offset(
            1 as libc::c_int as isize,
        ) = scale - *dashes.offset(0 as libc::c_int as isize);
    *dash_offset = if on != 0 {
        0.0f64
    } else {
        *dashes.offset(0 as libc::c_int as isize)
    };
}
