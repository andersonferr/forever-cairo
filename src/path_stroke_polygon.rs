use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn cairo_matrix_transform_distance(
        matrix: *const cairo_matrix_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_spline_intersects(
        a: *const cairo_point_t,
        b: *const cairo_point_t,
        c: *const cairo_point_t,
        d: *const cairo_point_t,
        box_0: *const cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to_0: Option::<cairo_path_fixed_move_to_func_t>,
        line_to_0: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to_0: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path_0: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_pen_fini(pen: *mut cairo_pen_t);
    fn _cairo_polygon_add_contour(
        polygon: *mut cairo_polygon_t,
        contour: *const cairo_contour_t,
    ) -> cairo_status_t;
    fn _cairo_pen_find_active_ccw_vertices(
        pen: *const cairo_pen_t,
        in_0: *const cairo_slope_t,
        out: *const cairo_slope_t,
        start: *mut libc::c_int,
        stop: *mut libc::c_int,
    );
    fn _cairo_pen_find_active_cw_vertices(
        pen: *const cairo_pen_t,
        in_0: *const cairo_slope_t,
        out: *const cairo_slope_t,
        start: *mut libc::c_int,
        stop: *mut libc::c_int,
    );
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
    fn _cairo_pen_init(
        pen: *mut cairo_pen_t,
        radius: libc::c_double,
        tolerance: libc::c_double,
        ctm: *const cairo_matrix_t,
    ) -> cairo_status_t;
    fn _cairo_matrix_compute_determinant(
        matrix: *const cairo_matrix_t,
    ) -> libc::c_double;
    fn _cairo_stroke_style_max_distance_from_path(
        style: *const cairo_stroke_style_t,
        path: *const cairo_path_fixed_t,
        ctm: *const cairo_matrix_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn _cairo_path_fixed_stroke_dashed_to_polygon(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_contour_init(contour: *mut cairo_contour_t, direction: libc::c_int);
    fn __cairo_contour_add_point(
        contour: *mut cairo_contour_t,
        point: *const cairo_point_t,
    ) -> cairo_int_status_t;
    fn _cairo_contour_reset(contour: *mut cairo_contour_t);
    fn _cairo_contour_fini(contour: *mut cairo_contour_t);
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub type cairo_int_status_t = _cairo_int_status;
pub type _cairo_int_status = libc::c_uint;
pub const CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN: _cairo_int_status = 105;
pub const CAIRO_INT_STATUS_IMAGE_FALLBACK: _cairo_int_status = 104;
pub const CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY: _cairo_int_status = 103;
pub const CAIRO_INT_STATUS_NOTHING_TO_DO: _cairo_int_status = 102;
pub const CAIRO_INT_STATUS_DEGENERATE: _cairo_int_status = 101;
pub const CAIRO_INT_STATUS_UNSUPPORTED: _cairo_int_status = 100;
pub const CAIRO_INT_STATUS_LAST_STATUS: _cairo_int_status = 44;
pub const CAIRO_INT_STATUS_DWRITE_ERROR: _cairo_int_status = 43;
pub const CAIRO_INT_STATUS_TAG_ERROR: _cairo_int_status = 42;
pub const CAIRO_INT_STATUS_WIN32_GDI_ERROR: _cairo_int_status = 41;
pub const CAIRO_INT_STATUS_FREETYPE_ERROR: _cairo_int_status = 40;
pub const CAIRO_INT_STATUS_PNG_ERROR: _cairo_int_status = 39;
pub const CAIRO_INT_STATUS_JBIG2_GLOBAL_MISSING: _cairo_int_status = 38;
pub const CAIRO_INT_STATUS_DEVICE_FINISHED: _cairo_int_status = 37;
pub const CAIRO_INT_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_int_status = 36;
pub const CAIRO_INT_STATUS_DEVICE_ERROR: _cairo_int_status = 35;
pub const CAIRO_INT_STATUS_DEVICE_TYPE_MISMATCH: _cairo_int_status = 34;
pub const CAIRO_INT_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_int_status = 33;
pub const CAIRO_INT_STATUS_INVALID_SIZE: _cairo_int_status = 32;
pub const CAIRO_INT_STATUS_INVALID_WEIGHT: _cairo_int_status = 31;
pub const CAIRO_INT_STATUS_INVALID_SLANT: _cairo_int_status = 30;
pub const CAIRO_INT_STATUS_INVALID_CLUSTERS: _cairo_int_status = 29;
pub const CAIRO_INT_STATUS_NEGATIVE_COUNT: _cairo_int_status = 28;
pub const CAIRO_INT_STATUS_USER_FONT_ERROR: _cairo_int_status = 27;
pub const CAIRO_INT_STATUS_USER_FONT_IMMUTABLE: _cairo_int_status = 26;
pub const CAIRO_INT_STATUS_FONT_TYPE_MISMATCH: _cairo_int_status = 25;
pub const CAIRO_INT_STATUS_INVALID_STRIDE: _cairo_int_status = 24;
pub const CAIRO_INT_STATUS_TEMP_FILE_ERROR: _cairo_int_status = 23;
pub const CAIRO_INT_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_int_status = 22;
pub const CAIRO_INT_STATUS_INVALID_INDEX: _cairo_int_status = 21;
pub const CAIRO_INT_STATUS_INVALID_DSC_COMMENT: _cairo_int_status = 20;
pub const CAIRO_INT_STATUS_INVALID_DASH: _cairo_int_status = 19;
pub const CAIRO_INT_STATUS_FILE_NOT_FOUND: _cairo_int_status = 18;
pub const CAIRO_INT_STATUS_INVALID_VISUAL: _cairo_int_status = 17;
pub const CAIRO_INT_STATUS_INVALID_FORMAT: _cairo_int_status = 16;
pub const CAIRO_INT_STATUS_INVALID_CONTENT: _cairo_int_status = 15;
pub const CAIRO_INT_STATUS_PATTERN_TYPE_MISMATCH: _cairo_int_status = 14;
pub const CAIRO_INT_STATUS_SURFACE_TYPE_MISMATCH: _cairo_int_status = 13;
pub const CAIRO_INT_STATUS_SURFACE_FINISHED: _cairo_int_status = 12;
pub const CAIRO_INT_STATUS_WRITE_ERROR: _cairo_int_status = 11;
pub const CAIRO_INT_STATUS_READ_ERROR: _cairo_int_status = 10;
pub const CAIRO_INT_STATUS_INVALID_PATH_DATA: _cairo_int_status = 9;
pub const CAIRO_INT_STATUS_INVALID_STRING: _cairo_int_status = 8;
pub const CAIRO_INT_STATUS_NULL_POINTER: _cairo_int_status = 7;
pub const CAIRO_INT_STATUS_INVALID_STATUS: _cairo_int_status = 6;
pub const CAIRO_INT_STATUS_INVALID_MATRIX: _cairo_int_status = 5;
pub const CAIRO_INT_STATUS_NO_CURRENT_POINT: _cairo_int_status = 4;
pub const CAIRO_INT_STATUS_INVALID_POP_GROUP: _cairo_int_status = 3;
pub const CAIRO_INT_STATUS_INVALID_RESTORE: _cairo_int_status = 2;
pub const CAIRO_INT_STATUS_NO_MEMORY: _cairo_int_status = 1;
pub const CAIRO_INT_STATUS_SUCCESS: _cairo_int_status = 0;
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
pub type uint64_t = __uint64_t;
pub type cairo_uint64_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_contour {
    pub next: cairo_list_t,
    pub direction: libc::c_int,
    pub chain: cairo_contour_chain_t,
    pub tail: *mut cairo_contour_chain_t,
    pub embedded_points: [cairo_point_t; 64],
}
pub type cairo_contour_chain_t = _cairo_contour_chain;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_contour_chain {
    pub points: *mut cairo_point_t,
    pub num_points: libc::c_int,
    pub size_points: libc::c_int,
    pub next: *mut _cairo_contour_chain,
}
pub type cairo_contour_t = _cairo_contour;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pen_vertex {
    pub point: cairo_point_t,
    pub slope_ccw: cairo_slope_t,
    pub slope_cw: cairo_slope_t,
}
pub type cairo_pen_vertex_t = _cairo_pen_vertex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pen {
    pub radius: libc::c_double,
    pub tolerance: libc::c_double,
    pub num_vertices: libc::c_int,
    pub vertices: *mut cairo_pen_vertex_t,
    pub vertices_embedded: [cairo_pen_vertex_t; 32],
}
pub type cairo_pen_t = _cairo_pen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_stroke_face {
    pub ccw: cairo_point_t,
    pub point: cairo_point_t,
    pub cw: cairo_point_t,
    pub dev_vector: cairo_slope_t,
    pub dev_slope: cairo_point_double_t,
    pub usr_vector: cairo_point_double_t,
    pub length: libc::c_double,
}
pub type cairo_stroke_face_t = _cairo_stroke_face;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stroker {
    pub style: cairo_stroke_style_t,
    pub cw: stroke_contour,
    pub ccw: stroke_contour,
    pub contour_tolerance: cairo_uint64_t,
    pub polygon: *mut cairo_polygon_t,
    pub ctm: *const cairo_matrix_t,
    pub ctm_inverse: *const cairo_matrix_t,
    pub tolerance: libc::c_double,
    pub spline_cusp_tolerance: libc::c_double,
    pub half_line_width: libc::c_double,
    pub ctm_det_positive: cairo_bool_t,
    pub pen: cairo_pen_t,
    pub first_point: cairo_point_t,
    pub has_initial_sub_path: cairo_bool_t,
    pub has_current_face: cairo_bool_t,
    pub current_face: cairo_stroke_face_t,
    pub has_first_face: cairo_bool_t,
    pub first_face: cairo_stroke_face_t,
    pub has_bounds: cairo_bool_t,
    pub bounds: cairo_box_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stroke_contour {
    pub contour: cairo_contour_t,
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
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
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
#[inline]
unsafe extern "C" fn _cairo_contour_add_point(
    mut contour: *mut cairo_contour_t,
    mut point: *const cairo_point_t,
) -> cairo_int_status_t {
    let mut tail: *mut _cairo_contour_chain = (*contour).tail;
    if (*tail).num_points == (*tail).size_points {
        return __cairo_contour_add_point(contour, point);
    }
    let ref mut fresh0 = (*tail).num_points;
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    *((*tail).points).offset(fresh1 as isize) = *point;
    return CAIRO_INT_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn _cairo_contour_first_point(
    mut c: *mut cairo_contour_t,
) -> *mut cairo_point_t {
    return &mut *((*c).chain.points).offset(0 as libc::c_int as isize)
        as *mut cairo_point_t;
}
#[inline]
unsafe extern "C" fn _cairo_contour_last_point(
    mut c: *mut cairo_contour_t,
) -> *mut cairo_point_t {
    return &mut *((*(*c).tail).points)
        .offset(((*(*c).tail).num_points - 1 as libc::c_int) as isize)
        as *mut cairo_point_t;
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
unsafe extern "C" fn point_distance_sq(
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) -> cairo_uint64_t {
    let mut dx: int32_t = (*p1).x - (*p2).x;
    let mut dy: int32_t = (*p1).y - (*p2).y;
    return (dx as int64_t * dx as libc::c_long + dy as int64_t * dy as libc::c_long)
        as cairo_uint64_t;
}
unsafe extern "C" fn within_tolerance(
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut tolerance: cairo_uint64_t,
) -> cairo_bool_t {
    return 0 as libc::c_int;
}
unsafe extern "C" fn contour_add_point(
    mut stroker: *mut stroker,
    mut c: *mut stroke_contour,
    mut point: *const cairo_point_t,
) {
    if within_tolerance(
        point,
        _cairo_contour_last_point(&mut (*c).contour),
        (*stroker).contour_tolerance,
    ) == 0
    {
        _cairo_contour_add_point(&mut (*c).contour, point);
    }
}
unsafe extern "C" fn translate_point(
    mut point: *mut cairo_point_t,
    mut offset: *const cairo_point_t,
) {
    let ref mut fresh2 = (*point).x;
    *fresh2 += (*offset).x;
    let ref mut fresh3 = (*point).y;
    *fresh3 += (*offset).y;
}
unsafe extern "C" fn slope_compare_sgn(
    mut dx1: libc::c_double,
    mut dy1: libc::c_double,
    mut dx2: libc::c_double,
    mut dy2: libc::c_double,
) -> libc::c_int {
    let mut c: libc::c_double = dx1 * dy2 - dx2 * dy1;
    if c > 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if c < 0 as libc::c_int as libc::c_double {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_fan(
    mut stroker: *mut stroker,
    mut in_vector: *const cairo_slope_t,
    mut out_vector: *const cairo_slope_t,
    mut midpt: *const cairo_point_t,
    mut clockwise: cairo_bool_t,
    mut c: *mut stroke_contour,
) {
    let mut pen: *mut cairo_pen_t = &mut (*stroker).pen;
    let mut start: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    if (*stroker).has_bounds != 0
        && _cairo_box_contains_point(&mut (*stroker).bounds, midpt) == 0
    {
        return;
    }
    if (*stroker).pen.num_vertices != 0 {} else {
        __assert_fail(
            b"stroker->pen.num_vertices\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-stroke-polygon.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 139],
                &[libc::c_char; 139],
            >(
                b"void add_fan(struct stroker *, const cairo_slope_t *, const cairo_slope_t *, const cairo_point_t *, cairo_bool_t, struct stroke_contour *)\0",
            ))
                .as_ptr(),
        );
    }
    if clockwise != 0 {
        _cairo_pen_find_active_cw_vertices(
            pen,
            in_vector,
            out_vector,
            &mut start,
            &mut stop,
        );
        while start != stop {
            let mut p: cairo_point_t = *midpt;
            translate_point(
                &mut p,
                &mut (*((*pen).vertices).offset(start as isize)).point,
            );
            contour_add_point(stroker, c, &mut p);
            start += 1;
            if start == (*pen).num_vertices {
                start = 0 as libc::c_int;
            }
        }
    } else {
        _cairo_pen_find_active_ccw_vertices(
            pen,
            in_vector,
            out_vector,
            &mut start,
            &mut stop,
        );
        while start != stop {
            let mut p_0: cairo_point_t = *midpt;
            translate_point(
                &mut p_0,
                &mut (*((*pen).vertices).offset(start as isize)).point,
            );
            contour_add_point(stroker, c, &mut p_0);
            let fresh4 = start;
            start = start - 1;
            if fresh4 == 0 as libc::c_int {
                start += (*pen).num_vertices;
            }
        }
    };
}
unsafe extern "C" fn join_is_clockwise(
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
) -> libc::c_int {
    return (_cairo_slope_compare(&(*in_0).dev_vector, &(*out).dev_vector)
        < 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn inner_join(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
    mut clockwise: libc::c_int,
) {
    let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut inner: *mut stroke_contour = 0 as *mut stroke_contour;
    if clockwise != 0 {
        inner = &mut (*stroker).ccw;
        outpt = &(*out).ccw;
    } else {
        inner = &mut (*stroker).cw;
        outpt = &(*out).cw;
    }
    contour_add_point(stroker, inner, &(*in_0).point);
    contour_add_point(stroker, inner, outpt);
}
unsafe extern "C" fn inner_close(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *mut cairo_stroke_face_t,
) {
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut inner: *mut stroke_contour = 0 as *mut stroke_contour;
    if join_is_clockwise(in_0, out) != 0 {
        inner = &mut (*stroker).ccw;
        inpt = &mut (*out).ccw;
    } else {
        inner = &mut (*stroker).cw;
        inpt = &mut (*out).cw;
    }
    contour_add_point(stroker, inner, &(*in_0).point);
    contour_add_point(stroker, inner, inpt);
    *_cairo_contour_first_point(
        &mut (*inner).contour,
    ) = *_cairo_contour_last_point(&mut (*inner).contour);
}
unsafe extern "C" fn outer_close(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
) {
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outer: *mut stroke_contour = 0 as *mut stroke_contour;
    let mut clockwise: libc::c_int = 0;
    if (*in_0).cw.x == (*out).cw.x && (*in_0).cw.y == (*out).cw.y
        && (*in_0).ccw.x == (*out).ccw.x && (*in_0).ccw.y == (*out).ccw.y
    {
        return;
    }
    clockwise = join_is_clockwise(in_0, out);
    if clockwise != 0 {
        inpt = &(*in_0).cw;
        outpt = &(*out).cw;
        outer = &mut (*stroker).cw;
    } else {
        inpt = &(*in_0).ccw;
        outpt = &(*out).ccw;
        outer = &mut (*stroker).ccw;
    }
    if within_tolerance(inpt, outpt, (*stroker).contour_tolerance) != 0 {
        *_cairo_contour_first_point(
            &mut (*outer).contour,
        ) = *_cairo_contour_last_point(&mut (*outer).contour);
        return;
    }
    let mut current_block_46: u64;
    match (*stroker).style.line_join as libc::c_uint {
        1 => {
            if (*in_0).dev_slope.x * (*out).dev_slope.x
                + (*in_0).dev_slope.y * (*out).dev_slope.y
                < (*stroker).spline_cusp_tolerance
            {
                add_fan(
                    stroker,
                    &(*in_0).dev_vector,
                    &(*out).dev_vector,
                    &(*in_0).point,
                    clockwise,
                    outer,
                );
                current_block_46 = 9627623479216730126;
            } else {
                current_block_46 = 13797916685926291137;
            }
        }
        2 => {
            current_block_46 = 9627623479216730126;
        }
        0 | _ => {
            current_block_46 = 13797916685926291137;
        }
    }
    match current_block_46 {
        13797916685926291137 => {
            let mut in_dot_out: libc::c_double = (*in_0).dev_slope.x * (*out).dev_slope.x
                + (*in_0).dev_slope.y * (*out).dev_slope.y;
            let mut ml: libc::c_double = (*stroker).style.miter_limit;
            if 2 as libc::c_int as libc::c_double
                <= ml * ml * (1 as libc::c_int as libc::c_double + in_dot_out)
            {
                let mut x1: libc::c_double = 0.;
                let mut y1: libc::c_double = 0.;
                let mut x2: libc::c_double = 0.;
                let mut y2: libc::c_double = 0.;
                let mut mx: libc::c_double = 0.;
                let mut my: libc::c_double = 0.;
                let mut dx1: libc::c_double = 0.;
                let mut dx2: libc::c_double = 0.;
                let mut dy1: libc::c_double = 0.;
                let mut dy2: libc::c_double = 0.;
                let mut ix: libc::c_double = 0.;
                let mut iy: libc::c_double = 0.;
                let mut fdx1: libc::c_double = 0.;
                let mut fdy1: libc::c_double = 0.;
                let mut fdx2: libc::c_double = 0.;
                let mut fdy2: libc::c_double = 0.;
                let mut mdx: libc::c_double = 0.;
                let mut mdy: libc::c_double = 0.;
                x1 = _cairo_fixed_to_double((*inpt).x);
                y1 = _cairo_fixed_to_double((*inpt).y);
                dx1 = (*in_0).dev_slope.x;
                dy1 = (*in_0).dev_slope.y;
                x2 = _cairo_fixed_to_double((*outpt).x);
                y2 = _cairo_fixed_to_double((*outpt).y);
                dx2 = (*out).dev_slope.x;
                dy2 = (*out).dev_slope.y;
                my = ((x2 - x1) * dy1 * dy2 - y2 * dx2 * dy1 + y1 * dx1 * dy2)
                    / (dx1 * dy2 - dx2 * dy1);
                if fabs(dy1) >= fabs(dy2) {
                    mx = (my - y1) * dx1 / dy1 + x1;
                } else {
                    mx = (my - y2) * dx2 / dy2 + x2;
                }
                ix = _cairo_fixed_to_double((*in_0).point.x);
                iy = _cairo_fixed_to_double((*in_0).point.y);
                fdx1 = x1 - ix;
                fdy1 = y1 - iy;
                fdx2 = x2 - ix;
                fdy2 = y2 - iy;
                mdx = mx - ix;
                mdy = my - iy;
                if slope_compare_sgn(fdx1, fdy1, mdx, mdy)
                    != slope_compare_sgn(fdx2, fdy2, mdx, mdy)
                {
                    let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
                    p.x = _cairo_fixed_from_double(mx);
                    p.y = _cairo_fixed_from_double(my);
                    *_cairo_contour_last_point(&mut (*outer).contour) = p;
                    *_cairo_contour_first_point(&mut (*outer).contour) = p;
                    return;
                }
            }
        }
        _ => {}
    }
    contour_add_point(stroker, outer, outpt);
}
unsafe extern "C" fn outer_join(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
    mut clockwise: libc::c_int,
) {
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outer: *mut stroke_contour = 0 as *mut stroke_contour;
    if (*in_0).cw.x == (*out).cw.x && (*in_0).cw.y == (*out).cw.y
        && (*in_0).ccw.x == (*out).ccw.x && (*in_0).ccw.y == (*out).ccw.y
    {
        return;
    }
    if clockwise != 0 {
        inpt = &(*in_0).cw;
        outpt = &(*out).cw;
        outer = &mut (*stroker).cw;
    } else {
        inpt = &(*in_0).ccw;
        outpt = &(*out).ccw;
        outer = &mut (*stroker).ccw;
    }
    match (*stroker).style.line_join as libc::c_uint {
        1 => {
            add_fan(
                stroker,
                &(*in_0).dev_vector,
                &(*out).dev_vector,
                &(*in_0).point,
                clockwise,
                outer,
            );
        }
        2 => {}
        0 | _ => {
            let mut in_dot_out: libc::c_double = (*in_0).dev_slope.x * (*out).dev_slope.x
                + (*in_0).dev_slope.y * (*out).dev_slope.y;
            let mut ml: libc::c_double = (*stroker).style.miter_limit;
            if 2 as libc::c_int as libc::c_double
                <= ml * ml * (1 as libc::c_int as libc::c_double + in_dot_out)
            {
                let mut x1: libc::c_double = 0.;
                let mut y1: libc::c_double = 0.;
                let mut x2: libc::c_double = 0.;
                let mut y2: libc::c_double = 0.;
                let mut mx: libc::c_double = 0.;
                let mut my: libc::c_double = 0.;
                let mut dx1: libc::c_double = 0.;
                let mut dx2: libc::c_double = 0.;
                let mut dy1: libc::c_double = 0.;
                let mut dy2: libc::c_double = 0.;
                let mut ix: libc::c_double = 0.;
                let mut iy: libc::c_double = 0.;
                let mut fdx1: libc::c_double = 0.;
                let mut fdy1: libc::c_double = 0.;
                let mut fdx2: libc::c_double = 0.;
                let mut fdy2: libc::c_double = 0.;
                let mut mdx: libc::c_double = 0.;
                let mut mdy: libc::c_double = 0.;
                x1 = _cairo_fixed_to_double((*inpt).x);
                y1 = _cairo_fixed_to_double((*inpt).y);
                dx1 = (*in_0).dev_slope.x;
                dy1 = (*in_0).dev_slope.y;
                x2 = _cairo_fixed_to_double((*outpt).x);
                y2 = _cairo_fixed_to_double((*outpt).y);
                dx2 = (*out).dev_slope.x;
                dy2 = (*out).dev_slope.y;
                my = ((x2 - x1) * dy1 * dy2 - y2 * dx2 * dy1 + y1 * dx1 * dy2)
                    / (dx1 * dy2 - dx2 * dy1);
                if fabs(dy1) >= fabs(dy2) {
                    mx = (my - y1) * dx1 / dy1 + x1;
                } else {
                    mx = (my - y2) * dx2 / dy2 + x2;
                }
                ix = _cairo_fixed_to_double((*in_0).point.x);
                iy = _cairo_fixed_to_double((*in_0).point.y);
                fdx1 = x1 - ix;
                fdy1 = y1 - iy;
                fdx2 = x2 - ix;
                fdy2 = y2 - iy;
                mdx = mx - ix;
                mdy = my - iy;
                if slope_compare_sgn(fdx1, fdy1, mdx, mdy)
                    != slope_compare_sgn(fdx2, fdy2, mdx, mdy)
                {
                    let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
                    p.x = _cairo_fixed_from_double(mx);
                    p.y = _cairo_fixed_from_double(my);
                    *_cairo_contour_last_point(&mut (*outer).contour) = p;
                    return;
                }
            }
        }
    }
    contour_add_point(stroker, outer, outpt);
}
unsafe extern "C" fn add_cap(
    mut stroker: *mut stroker,
    mut f: *const cairo_stroke_face_t,
    mut c: *mut stroke_contour,
) {
    match (*stroker).style.line_cap as libc::c_uint {
        1 => {
            let mut slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            slope.dx = -(*f).dev_vector.dx;
            slope.dy = -(*f).dev_vector.dy;
            add_fan(
                stroker,
                &(*f).dev_vector,
                &mut slope,
                &(*f).point,
                0 as libc::c_int,
                c,
            );
        }
        2 => {
            let mut fvector: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
            let mut dx: libc::c_double = 0.;
            let mut dy: libc::c_double = 0.;
            dx = (*f).usr_vector.x;
            dy = (*f).usr_vector.y;
            dx *= (*stroker).half_line_width;
            dy *= (*stroker).half_line_width;
            cairo_matrix_transform_distance((*stroker).ctm, &mut dx, &mut dy);
            fvector.dx = _cairo_fixed_from_double(dx);
            fvector.dy = _cairo_fixed_from_double(dy);
            p.x = (*f).ccw.x + fvector.dx;
            p.y = (*f).ccw.y + fvector.dy;
            contour_add_point(stroker, c, &mut p);
            p.x = (*f).cw.x + fvector.dx;
            p.y = (*f).cw.y + fvector.dy;
            contour_add_point(stroker, c, &mut p);
        }
        0 | _ => {}
    }
    contour_add_point(stroker, c, &(*f).cw);
}
unsafe extern "C" fn add_leading_cap(
    mut stroker: *mut stroker,
    mut face: *const cairo_stroke_face_t,
    mut c: *mut stroke_contour,
) {
    let mut reversed: cairo_stroke_face_t = cairo_stroke_face_t {
        ccw: cairo_point_t { x: 0, y: 0 },
        point: cairo_point_t { x: 0, y: 0 },
        cw: cairo_point_t { x: 0, y: 0 },
        dev_vector: cairo_slope_t { dx: 0, dy: 0 },
        dev_slope: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        usr_vector: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        length: 0.,
    };
    let mut t: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    reversed = *face;
    reversed.usr_vector.x = -reversed.usr_vector.x;
    reversed.usr_vector.y = -reversed.usr_vector.y;
    reversed.dev_vector.dx = -reversed.dev_vector.dx;
    reversed.dev_vector.dy = -reversed.dev_vector.dy;
    t = reversed.cw;
    reversed.cw = reversed.ccw;
    reversed.ccw = t;
    add_cap(stroker, &mut reversed, c);
}
unsafe extern "C" fn add_trailing_cap(
    mut stroker: *mut stroker,
    mut face: *const cairo_stroke_face_t,
    mut c: *mut stroke_contour,
) {
    add_cap(stroker, face, c);
}
#[inline]
unsafe extern "C" fn normalize_slope(
    mut dx: *mut libc::c_double,
    mut dy: *mut libc::c_double,
) -> libc::c_double {
    let mut dx0: libc::c_double = *dx;
    let mut dy0: libc::c_double = *dy;
    let mut mag: libc::c_double = 0.;
    if dx0 != 0.0f64 || dy0 != 0.0f64 {} else {
        __assert_fail(
            b"dx0 != 0.0 || dy0 != 0.0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-stroke-polygon.c\0" as *const u8 as *const libc::c_char,
            829 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"double normalize_slope(double *, double *)\0"))
                .as_ptr(),
        );
    }
    if dx0 == 0.0f64 {
        *dx = 0.0f64;
        if dy0 > 0.0f64 {
            mag = dy0;
            *dy = 1.0f64;
        } else {
            mag = -dy0;
            *dy = -1.0f64;
        }
    } else if dy0 == 0.0f64 {
        *dy = 0.0f64;
        if dx0 > 0.0f64 {
            mag = dx0;
            *dx = 1.0f64;
        } else {
            mag = -dx0;
            *dx = -1.0f64;
        }
    } else {
        mag = hypot(dx0, dy0);
        *dx = dx0 / mag;
        *dy = dy0 / mag;
    }
    return mag;
}
unsafe extern "C" fn compute_face(
    mut point: *const cairo_point_t,
    mut dev_slope: *const cairo_slope_t,
    mut stroker: *mut stroker,
    mut face: *mut cairo_stroke_face_t,
) {
    let mut face_dx: libc::c_double = 0.;
    let mut face_dy: libc::c_double = 0.;
    let mut offset_ccw: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut offset_cw: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut slope_dx: libc::c_double = 0.;
    let mut slope_dy: libc::c_double = 0.;
    slope_dx = _cairo_fixed_to_double((*dev_slope).dx);
    slope_dy = _cairo_fixed_to_double((*dev_slope).dy);
    (*face).length = normalize_slope(&mut slope_dx, &mut slope_dy);
    (*face).dev_slope.x = slope_dx;
    (*face).dev_slope.y = slope_dy;
    if _cairo_matrix_is_identity((*stroker).ctm_inverse) == 0 {
        cairo_matrix_transform_distance(
            (*stroker).ctm_inverse,
            &mut slope_dx,
            &mut slope_dy,
        );
        normalize_slope(&mut slope_dx, &mut slope_dy);
        if (*stroker).ctm_det_positive != 0 {
            face_dx = -slope_dy * (*stroker).half_line_width;
            face_dy = slope_dx * (*stroker).half_line_width;
        } else {
            face_dx = slope_dy * (*stroker).half_line_width;
            face_dy = -slope_dx * (*stroker).half_line_width;
        }
        cairo_matrix_transform_distance((*stroker).ctm, &mut face_dx, &mut face_dy);
    } else {
        face_dx = -slope_dy * (*stroker).half_line_width;
        face_dy = slope_dx * (*stroker).half_line_width;
    }
    offset_ccw.x = _cairo_fixed_from_double(face_dx);
    offset_ccw.y = _cairo_fixed_from_double(face_dy);
    offset_cw.x = -offset_ccw.x;
    offset_cw.y = -offset_ccw.y;
    (*face).ccw = *point;
    translate_point(&mut (*face).ccw, &mut offset_ccw);
    (*face).point = *point;
    (*face).cw = *point;
    translate_point(&mut (*face).cw, &mut offset_cw);
    (*face).usr_vector.x = slope_dx;
    (*face).usr_vector.y = slope_dy;
    (*face).dev_vector = *dev_slope;
}
unsafe extern "C" fn add_caps(mut stroker: *mut stroker) {
    if (*stroker).has_initial_sub_path != 0 && (*stroker).has_first_face == 0
        && (*stroker).has_current_face == 0
        && (*stroker).style.line_cap as libc::c_uint
            == CAIRO_LINE_CAP_ROUND as libc::c_int as libc::c_uint
    {
        let mut slope: cairo_slope_t = {
            let mut init = _cairo_slope {
                dx: (1 as libc::c_int) << 8 as libc::c_int,
                dy: 0 as libc::c_int,
            };
            init
        };
        let mut face: cairo_stroke_face_t = cairo_stroke_face_t {
            ccw: cairo_point_t { x: 0, y: 0 },
            point: cairo_point_t { x: 0, y: 0 },
            cw: cairo_point_t { x: 0, y: 0 },
            dev_vector: cairo_slope_t { dx: 0, dy: 0 },
            dev_slope: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            usr_vector: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            length: 0.,
        };
        compute_face(&mut (*stroker).first_point, &mut slope, stroker, &mut face);
        add_leading_cap(stroker, &mut face, &mut (*stroker).ccw);
        add_trailing_cap(stroker, &mut face, &mut (*stroker).ccw);
        _cairo_contour_add_point(
            &mut (*stroker).ccw.contour,
            _cairo_contour_first_point(&mut (*stroker).ccw.contour),
        );
        _cairo_polygon_add_contour((*stroker).polygon, &mut (*stroker).ccw.contour);
        _cairo_contour_reset(&mut (*stroker).ccw.contour);
    } else {
        if (*stroker).has_current_face != 0 {
            add_trailing_cap(stroker, &mut (*stroker).current_face, &mut (*stroker).ccw);
        }
        _cairo_polygon_add_contour((*stroker).polygon, &mut (*stroker).ccw.contour);
        _cairo_contour_reset(&mut (*stroker).ccw.contour);
        if (*stroker).has_first_face != 0 {
            _cairo_contour_add_point(
                &mut (*stroker).ccw.contour,
                &mut (*stroker).first_face.cw,
            );
            add_leading_cap(stroker, &mut (*stroker).first_face, &mut (*stroker).ccw);
            _cairo_polygon_add_contour((*stroker).polygon, &mut (*stroker).ccw.contour);
            _cairo_contour_reset(&mut (*stroker).ccw.contour);
        }
        _cairo_polygon_add_contour((*stroker).polygon, &mut (*stroker).cw.contour);
        _cairo_contour_reset(&mut (*stroker).cw.contour);
    };
}
unsafe extern "C" fn move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut stroker = closure as *mut stroker;
    add_caps(stroker);
    (*stroker).has_first_face = 0 as libc::c_int;
    (*stroker).has_current_face = 0 as libc::c_int;
    (*stroker).has_initial_sub_path = 0 as libc::c_int;
    (*stroker).first_point = *point;
    (*stroker).current_face.point = *point;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut stroker = closure as *mut stroker;
    let mut start: cairo_stroke_face_t = cairo_stroke_face_t {
        ccw: cairo_point_t { x: 0, y: 0 },
        point: cairo_point_t { x: 0, y: 0 },
        cw: cairo_point_t { x: 0, y: 0 },
        dev_vector: cairo_slope_t { dx: 0, dy: 0 },
        dev_slope: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        usr_vector: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        length: 0.,
    };
    let mut p1: *mut cairo_point_t = &mut (*stroker).current_face.point;
    let mut dev_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    (*stroker).has_initial_sub_path = 1 as libc::c_int;
    if (*p1).x == (*point).x && (*p1).y == (*point).y {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_slope_init(&mut dev_slope, p1, point);
    compute_face(p1, &mut dev_slope, stroker, &mut start);
    if (*stroker).has_current_face != 0 {
        let mut clockwise: libc::c_int = _cairo_slope_compare(
            &mut (*stroker).current_face.dev_vector,
            &mut start.dev_vector,
        );
        if clockwise != 0 {
            clockwise = (clockwise < 0 as libc::c_int) as libc::c_int;
            if within_tolerance(
                &mut (*stroker).current_face.ccw,
                &mut start.ccw,
                (*stroker).contour_tolerance,
            ) == 0
                || within_tolerance(
                    &mut (*stroker).current_face.cw,
                    &mut start.cw,
                    (*stroker).contour_tolerance,
                ) == 0
            {
                outer_join(stroker, &mut (*stroker).current_face, &mut start, clockwise);
                inner_join(stroker, &mut (*stroker).current_face, &mut start, clockwise);
            }
        }
    } else {
        if (*stroker).has_first_face == 0 {
            (*stroker).first_face = start;
            (*stroker).has_first_face = 1 as libc::c_int;
        }
        (*stroker).has_current_face = 1 as libc::c_int;
        contour_add_point(stroker, &mut (*stroker).cw, &mut start.cw);
        contour_add_point(stroker, &mut (*stroker).ccw, &mut start.ccw);
    }
    (*stroker).current_face = start;
    (*stroker).current_face.point = *point;
    let ref mut fresh5 = (*stroker).current_face.ccw.x;
    *fresh5 += dev_slope.dx;
    let ref mut fresh6 = (*stroker).current_face.ccw.y;
    *fresh6 += dev_slope.dy;
    let ref mut fresh7 = (*stroker).current_face.cw.x;
    *fresh7 += dev_slope.dx;
    let ref mut fresh8 = (*stroker).current_face.cw.y;
    *fresh8 += dev_slope.dy;
    contour_add_point(stroker, &mut (*stroker).cw, &mut (*stroker).current_face.cw);
    contour_add_point(stroker, &mut (*stroker).ccw, &mut (*stroker).current_face.ccw);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn spline_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    let mut stroker: *mut stroker = closure as *mut stroker;
    let mut face: cairo_stroke_face_t = cairo_stroke_face_t {
        ccw: cairo_point_t { x: 0, y: 0 },
        point: cairo_point_t { x: 0, y: 0 },
        cw: cairo_point_t { x: 0, y: 0 },
        dev_vector: cairo_slope_t { dx: 0, dy: 0 },
        dev_slope: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        usr_vector: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        length: 0.,
    };
    if (*tangent).dx | (*tangent).dy == 0 as libc::c_int {
        let mut outer: *mut stroke_contour = 0 as *mut stroke_contour;
        let mut t: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        let mut clockwise: libc::c_int = 0;
        face = (*stroker).current_face;
        face.usr_vector.x = -face.usr_vector.x;
        face.usr_vector.y = -face.usr_vector.y;
        face.dev_vector.dx = -face.dev_vector.dx;
        face.dev_vector.dy = -face.dev_vector.dy;
        t = face.cw;
        face.cw = face.ccw;
        face.ccw = t;
        clockwise = join_is_clockwise(&mut (*stroker).current_face, &mut face);
        if clockwise != 0 {
            outer = &mut (*stroker).cw;
        } else {
            outer = &mut (*stroker).ccw;
        }
        add_fan(
            stroker,
            &mut (*stroker).current_face.dev_vector,
            &mut face.dev_vector,
            &mut (*stroker).current_face.point,
            clockwise,
            outer,
        );
    } else {
        compute_face(point, tangent, stroker, &mut face);
        if face.dev_slope.x * (*stroker).current_face.dev_slope.x
            + face.dev_slope.y * (*stroker).current_face.dev_slope.y
            < (*stroker).spline_cusp_tolerance
        {
            let mut outer_0: *mut stroke_contour = 0 as *mut stroke_contour;
            let mut clockwise_0: libc::c_int = join_is_clockwise(
                &mut (*stroker).current_face,
                &mut face,
            );
            let ref mut fresh9 = (*stroker).current_face.cw.x;
            *fresh9 += face.point.x - (*stroker).current_face.point.x;
            let ref mut fresh10 = (*stroker).current_face.cw.y;
            *fresh10 += face.point.y - (*stroker).current_face.point.y;
            contour_add_point(
                stroker,
                &mut (*stroker).cw,
                &mut (*stroker).current_face.cw,
            );
            let ref mut fresh11 = (*stroker).current_face.ccw.x;
            *fresh11 += face.point.x - (*stroker).current_face.point.x;
            let ref mut fresh12 = (*stroker).current_face.ccw.y;
            *fresh12 += face.point.y - (*stroker).current_face.point.y;
            contour_add_point(
                stroker,
                &mut (*stroker).ccw,
                &mut (*stroker).current_face.ccw,
            );
            if clockwise_0 != 0 {
                outer_0 = &mut (*stroker).cw;
            } else {
                outer_0 = &mut (*stroker).ccw;
            }
            add_fan(
                stroker,
                &mut (*stroker).current_face.dev_vector,
                &mut face.dev_vector,
                &mut (*stroker).current_face.point,
                clockwise_0,
                outer_0,
            );
        }
        contour_add_point(stroker, &mut (*stroker).cw, &mut face.cw);
        contour_add_point(stroker, &mut (*stroker).ccw, &mut face.ccw);
    }
    (*stroker).current_face = face;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn curve_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut stroker = closure as *mut stroker;
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
    let mut face: cairo_stroke_face_t = cairo_stroke_face_t {
        ccw: cairo_point_t { x: 0, y: 0 },
        point: cairo_point_t { x: 0, y: 0 },
        cw: cairo_point_t { x: 0, y: 0 },
        dev_vector: cairo_slope_t { dx: 0, dy: 0 },
        dev_slope: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        usr_vector: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        length: 0.,
    };
    if (*stroker).has_bounds != 0
        && _cairo_spline_intersects(
            &mut (*stroker).current_face.point,
            b,
            c,
            d,
            &mut (*stroker).bounds,
        ) == 0
    {
        return line_to(closure, d);
    }
    if _cairo_spline_init(
        &mut spline,
        Some(
            spline_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        ),
        stroker as *mut libc::c_void,
        &mut (*stroker).current_face.point,
        b,
        c,
        d,
    ) == 0
    {
        return line_to(closure, d);
    }
    compute_face(
        &mut (*stroker).current_face.point,
        &mut spline.initial_slope,
        stroker,
        &mut face,
    );
    if (*stroker).has_current_face != 0 {
        let mut clockwise: libc::c_int = join_is_clockwise(
            &mut (*stroker).current_face,
            &mut face,
        );
        outer_join(stroker, &mut (*stroker).current_face, &mut face, clockwise);
        inner_join(stroker, &mut (*stroker).current_face, &mut face, clockwise);
    } else {
        if (*stroker).has_first_face == 0 {
            (*stroker).first_face = face;
            (*stroker).has_first_face = 1 as libc::c_int;
        }
        (*stroker).has_current_face = 1 as libc::c_int;
        contour_add_point(stroker, &mut (*stroker).cw, &mut face.cw);
        contour_add_point(stroker, &mut (*stroker).ccw, &mut face.ccw);
    }
    (*stroker).current_face = face;
    return _cairo_spline_decompose(&mut spline, (*stroker).tolerance);
}
unsafe extern "C" fn close_path(mut closure: *mut libc::c_void) -> cairo_status_t {
    let mut stroker: *mut stroker = closure as *mut stroker;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = line_to(stroker as *mut libc::c_void, &mut (*stroker).first_point);
    if status as u64 != 0 {
        return status;
    }
    if (*stroker).has_first_face != 0 && (*stroker).has_current_face != 0 {
        outer_close(stroker, &mut (*stroker).current_face, &mut (*stroker).first_face);
        inner_close(stroker, &mut (*stroker).current_face, &mut (*stroker).first_face);
        _cairo_polygon_add_contour((*stroker).polygon, &mut (*stroker).cw.contour);
        _cairo_polygon_add_contour((*stroker).polygon, &mut (*stroker).ccw.contour);
        _cairo_contour_reset(&mut (*stroker).cw.contour);
        _cairo_contour_reset(&mut (*stroker).ccw.contour);
    } else {
        add_caps(stroker);
    }
    (*stroker).has_initial_sub_path = 0 as libc::c_int;
    (*stroker).has_first_face = 0 as libc::c_int;
    (*stroker).has_current_face = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_to_polygon(
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut polygon: *mut cairo_polygon_t,
) -> cairo_status_t {
    let mut stroker: stroker = stroker {
        style: cairo_stroke_style_t {
            line_width: 0.,
            line_cap: CAIRO_LINE_CAP_BUTT,
            line_join: CAIRO_LINE_JOIN_MITER,
            miter_limit: 0.,
            dash: 0 as *mut libc::c_double,
            num_dashes: 0,
            dash_offset: 0.,
            is_hairline: 0,
            pre_hairline_line_width: 0.,
        },
        cw: stroke_contour {
            contour: cairo_contour_t {
                next: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                },
                direction: 0,
                chain: cairo_contour_chain_t {
                    points: 0 as *mut cairo_point_t,
                    num_points: 0,
                    size_points: 0,
                    next: 0 as *mut _cairo_contour_chain,
                },
                tail: 0 as *mut cairo_contour_chain_t,
                embedded_points: [cairo_point_t { x: 0, y: 0 }; 64],
            },
        },
        ccw: stroke_contour {
            contour: cairo_contour_t {
                next: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                },
                direction: 0,
                chain: cairo_contour_chain_t {
                    points: 0 as *mut cairo_point_t,
                    num_points: 0,
                    size_points: 0,
                    next: 0 as *mut _cairo_contour_chain,
                },
                tail: 0 as *mut cairo_contour_chain_t,
                embedded_points: [cairo_point_t { x: 0, y: 0 }; 64],
            },
        },
        contour_tolerance: 0,
        polygon: 0 as *mut cairo_polygon_t,
        ctm: 0 as *const cairo_matrix_t,
        ctm_inverse: 0 as *const cairo_matrix_t,
        tolerance: 0.,
        spline_cusp_tolerance: 0.,
        half_line_width: 0.,
        ctm_det_positive: 0,
        pen: cairo_pen_t {
            radius: 0.,
            tolerance: 0.,
            num_vertices: 0,
            vertices: 0 as *mut cairo_pen_vertex_t,
            vertices_embedded: [cairo_pen_vertex_t {
                point: cairo_point_t { x: 0, y: 0 },
                slope_ccw: cairo_slope_t { dx: 0, dy: 0 },
                slope_cw: cairo_slope_t { dx: 0, dy: 0 },
            }; 32],
        },
        first_point: cairo_point_t { x: 0, y: 0 },
        has_initial_sub_path: 0,
        has_current_face: 0,
        current_face: cairo_stroke_face_t {
            ccw: cairo_point_t { x: 0, y: 0 },
            point: cairo_point_t { x: 0, y: 0 },
            cw: cairo_point_t { x: 0, y: 0 },
            dev_vector: cairo_slope_t { dx: 0, dy: 0 },
            dev_slope: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            usr_vector: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            length: 0.,
        },
        has_first_face: 0,
        first_face: cairo_stroke_face_t {
            ccw: cairo_point_t { x: 0, y: 0 },
            point: cairo_point_t { x: 0, y: 0 },
            cw: cairo_point_t { x: 0, y: 0 },
            dev_vector: cairo_slope_t { dx: 0, dy: 0 },
            dev_slope: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            usr_vector: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            length: 0.,
        },
        has_bounds: 0,
        bounds: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*style).num_dashes != 0 {
        return _cairo_path_fixed_stroke_dashed_to_polygon(
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            polygon,
        );
    }
    stroker.has_bounds = (*polygon).num_limits;
    if stroker.has_bounds != 0 {
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        let mut fdx: cairo_fixed_t = 0;
        let mut fdy: cairo_fixed_t = 0;
        let mut i: libc::c_int = 0;
        stroker.bounds = *((*polygon).limits).offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int;
        while i < (*polygon).num_limits {
            _cairo_box_add_box(
                &mut stroker.bounds,
                &*((*polygon).limits).offset(i as isize),
            );
            i += 1;
        }
        _cairo_stroke_style_max_distance_from_path(style, path, ctm, &mut dx, &mut dy);
        fdx = _cairo_fixed_from_double(dx);
        fdy = _cairo_fixed_from_double(dy);
        stroker.bounds.p1.x -= fdx;
        stroker.bounds.p2.x += fdx;
        stroker.bounds.p1.y -= fdy;
        stroker.bounds.p2.y += fdy;
    }
    stroker.style = *style;
    stroker.ctm = ctm;
    stroker.ctm_inverse = ctm_inverse;
    stroker.tolerance = tolerance;
    stroker.half_line_width = (*style).line_width / 2.0f64;
    stroker
        .spline_cusp_tolerance = 1 as libc::c_int as libc::c_double
        - tolerance / stroker.half_line_width;
    stroker.spline_cusp_tolerance *= stroker.spline_cusp_tolerance;
    stroker.spline_cusp_tolerance *= 2 as libc::c_int as libc::c_double;
    stroker.spline_cusp_tolerance -= 1 as libc::c_int as libc::c_double;
    stroker
        .ctm_det_positive = (_cairo_matrix_compute_determinant(ctm) >= 0.0f64)
        as libc::c_int;
    stroker.pen.num_vertices = 0 as libc::c_int;
    if (*path).has_curve_to() as libc::c_int != 0
        || (*style).line_join as libc::c_uint
            == CAIRO_LINE_JOIN_ROUND as libc::c_int as libc::c_uint
        || (*style).line_cap as libc::c_uint
            == CAIRO_LINE_CAP_ROUND as libc::c_int as libc::c_uint
    {
        status = _cairo_pen_init(
            &mut stroker.pen,
            stroker.half_line_width,
            tolerance,
            ctm,
        );
        if status as u64 != 0 {
            return status;
        }
        if stroker.pen.num_vertices <= 1 as libc::c_int {
            return CAIRO_STATUS_SUCCESS;
        }
    }
    stroker.has_current_face = 0 as libc::c_int;
    stroker.has_first_face = 0 as libc::c_int;
    stroker.has_initial_sub_path = 0 as libc::c_int;
    _cairo_contour_init(&mut stroker.cw.contour, 1 as libc::c_int);
    _cairo_contour_init(&mut stroker.ccw.contour, -(1 as libc::c_int));
    tolerance *= ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
    tolerance *= tolerance;
    stroker.contour_tolerance = tolerance as cairo_uint64_t;
    stroker.polygon = polygon;
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(close_path as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t),
        &mut stroker as *mut stroker as *mut libc::c_void,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        add_caps(&mut stroker);
    }
    _cairo_contour_fini(&mut stroker.cw.contour);
    _cairo_contour_fini(&mut stroker.ccw.contour);
    if stroker.pen.num_vertices != 0 {
        _cairo_pen_fini(&mut stroker.pen);
    }
    return status;
}
