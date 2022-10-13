use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn _cairo_box_add_curve_to(
        extents: *mut cairo_box_t,
        a: *const cairo_point_t,
        b: *const cairo_point_t,
        c: *const cairo_point_t,
        d: *const cairo_point_t,
    );
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_stroke_style_max_distance_from_path(
        style: *const cairo_stroke_style_t,
        path: *const cairo_path_fixed_t,
        ctm: *const cairo_matrix_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn _cairo_polygon_fini(polygon: *mut cairo_polygon_t);
    fn _cairo_path_fixed_stroke_to_polygon(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_polygon_init(
        polygon: *mut cairo_polygon_t,
        boxes: *const cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_matrix_transformed_circle_major_axis(
        matrix: *const cairo_matrix_t,
        radius: libc::c_double,
    ) -> libc::c_double;
}
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
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_int {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
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
pub type cairo_line_t = _cairo_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_edge {
    pub line: cairo_line_t,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
    pub dir: libc::c_int,
}
pub type cairo_edge_t = _cairo_edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_polygon {
    pub status: cairo_status_t,
    pub extents: cairo_box_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_edges: libc::c_int,
    pub edges_size: libc::c_int,
    pub edges: *mut cairo_edge_t,
    pub edges_embedded: [cairo_edge_t; 32],
}
pub type cairo_polygon_t = _cairo_polygon;
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
pub type cairo_path_bounder_t = _cairo_path_bounder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_bounder {
    pub current_point: cairo_point_t,
    pub has_extents: cairo_bool_t,
    pub extents: cairo_box_t,
}
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
unsafe extern "C" fn _cairo_box_set(
    mut box_0: *mut cairo_box_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) {
    (*box_0).p1 = *p1;
    (*box_0).p2 = *p2;
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
unsafe extern "C" fn _cairo_path_bounder_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut bounder: *mut cairo_path_bounder_t = closure as *mut cairo_path_bounder_t;
    (*bounder).current_point = *point;
    if (*bounder).has_extents != 0 {
        _cairo_box_add_point(&mut (*bounder).extents, point);
    } else {
        (*bounder).has_extents = 1 as libc::c_int;
        _cairo_box_set(&mut (*bounder).extents, point, point);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_bounder_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut bounder: *mut cairo_path_bounder_t = closure as *mut cairo_path_bounder_t;
    (*bounder).current_point = *point;
    _cairo_box_add_point(&mut (*bounder).extents, point);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_bounder_curve_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_status_t {
    let mut bounder: *mut cairo_path_bounder_t = closure as *mut cairo_path_bounder_t;
    _cairo_box_add_curve_to(
        &mut (*bounder).extents,
        &mut (*bounder).current_point,
        b,
        c,
        d,
    );
    (*bounder).current_point = *d;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_bounder_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_bounder_extents(
    mut path: *const cairo_path_fixed_t,
    mut extents: *mut cairo_box_t,
) -> cairo_bool_t {
    let mut bounder: cairo_path_bounder_t = cairo_path_bounder_t {
        current_point: cairo_point_t { x: 0, y: 0 },
        has_extents: 0,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    bounder.has_extents = 0 as libc::c_int;
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _cairo_path_bounder_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_path_bounder_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_path_bounder_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_path_bounder_close_path
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut bounder as *mut cairo_path_bounder_t as *mut libc::c_void,
    );
    if status as u64 == 0 {} else {
        __assert_fail(
            b"!status\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-bounds.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"cairo_bool_t _cairo_path_bounder_extents(const cairo_path_fixed_t *, cairo_box_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if bounder.has_extents != 0 {
        *extents = bounder.extents;
    }
    return bounder.has_extents;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_approximate_clip_extents(
    mut path: *const cairo_path_fixed_t,
    mut extents: *mut cairo_rectangle_int_t,
) {
    _cairo_path_fixed_approximate_fill_extents(path, extents);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_approximate_fill_extents(
    mut path: *const cairo_path_fixed_t,
    mut extents: *mut cairo_rectangle_int_t,
) {
    _cairo_path_fixed_fill_extents(
        path,
        CAIRO_FILL_RULE_WINDING,
        0 as libc::c_int as libc::c_double,
        extents,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_fill_extents(
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut extents: *mut cairo_rectangle_int_t,
) {
    if (*path).extents.p1.x < (*path).extents.p2.x
        && (*path).extents.p1.y < (*path).extents.p2.y
    {
        _cairo_box_round_to_rectangle(&(*path).extents, extents);
    } else {
        let ref mut fresh0 = (*extents).y;
        *fresh0 = 0 as libc::c_int;
        (*extents).x = *fresh0;
        let ref mut fresh1 = (*extents).height;
        *fresh1 = 0 as libc::c_int;
        (*extents).width = *fresh1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_approximate_stroke_extents(
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut is_vector: cairo_bool_t,
    mut extents: *mut cairo_rectangle_int_t,
) {
    if (*path).has_extents() != 0 {
        let mut box_extents: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        _cairo_stroke_style_max_distance_from_path(style, path, ctm, &mut dx, &mut dy);
        if is_vector != 0 {
            let mut min: libc::c_double = _cairo_fixed_to_double(
                1 as libc::c_int * 2 as libc::c_int,
            );
            if dx < min {
                dx = min;
            }
            if dy < min {
                dy = min;
            }
        }
        box_extents = (*path).extents;
        box_extents.p1.x -= _cairo_fixed_from_double(dx);
        box_extents.p1.y -= _cairo_fixed_from_double(dy);
        box_extents.p2.x += _cairo_fixed_from_double(dx);
        box_extents.p2.y += _cairo_fixed_from_double(dy);
        _cairo_box_round_to_rectangle(&mut box_extents, extents);
    } else {
        let ref mut fresh2 = (*extents).y;
        *fresh2 = 0 as libc::c_int;
        (*extents).x = *fresh2;
        let ref mut fresh3 = (*extents).height;
        *fresh3 = 0 as libc::c_int;
        (*extents).width = *fresh3;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_extents(
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut style: cairo_stroke_style_t = cairo_stroke_style_t {
        line_width: 0.,
        line_cap: CAIRO_LINE_CAP_BUTT,
        line_join: CAIRO_LINE_JOIN_MITER,
        miter_limit: 0.,
        dash: 0 as *mut libc::c_double,
        num_dashes: 0,
        dash_offset: 0.,
        is_hairline: 0,
        pre_hairline_line_width: 0.,
    };
    let mut min_line_width: libc::c_double = _cairo_matrix_transformed_circle_major_axis(
        ctm_inverse,
        1.0f64,
    );
    if (*stroke_style).line_width < min_line_width {
        style = *stroke_style;
        style.line_width = min_line_width;
        stroke_style = &mut style;
    }
    _cairo_polygon_init(&mut polygon, 0 as *const cairo_box_t, 0 as libc::c_int);
    status = _cairo_path_fixed_stroke_to_polygon(
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        &mut polygon,
    );
    _cairo_box_round_to_rectangle(&mut polygon.extents, extents);
    _cairo_polygon_fini(&mut polygon);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_extents(
    mut path: *const cairo_path_fixed_t,
    mut box_0: *mut cairo_box_t,
) -> cairo_bool_t {
    *box_0 = (*path).extents;
    return (*path).has_extents() as cairo_bool_t;
}
