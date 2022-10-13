use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_get_extents(
        boxes: *const cairo_box_t,
        num_boxes: libc::c_int,
        extents: *mut cairo_box_t,
    );
    fn _cairo_box_intersects_line_segment(
        box_0: *const cairo_box_t,
        line: *mut cairo_line_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_to_polygon(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_pen_fini(pen: *mut cairo_pen_t);
    fn _cairo_pen_find_active_cw_vertices(
        pen: *const cairo_pen_t,
        in_0: *const cairo_slope_t,
        out: *const cairo_slope_t,
        start: *mut libc::c_int,
        stop: *mut libc::c_int,
    );
    fn _cairo_pen_find_active_ccw_vertices(
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
    fn _cairo_polygon_add_external_edge(
        polygon: *mut libc::c_void,
        p1: *const cairo_point_t,
        p2: *const cairo_point_t,
    ) -> cairo_status_t;
    fn _cairo_stroke_style_max_distance_from_path(
        style: *const cairo_stroke_style_t,
        path: *const cairo_path_fixed_t,
        ctm: *const cairo_matrix_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn _cairo_pen_init(
        pen: *mut cairo_pen_t,
        radius: libc::c_double,
        tolerance: libc::c_double,
        ctm: *const cairo_matrix_t,
    ) -> cairo_status_t;
    fn _cairo_matrix_compute_determinant(
        matrix: *const cairo_matrix_t,
    ) -> libc::c_double;
    fn _cairo_polygon_fini(polygon: *mut cairo_polygon_t);
    fn _cairo_bentley_ottmann_tessellate_polygon(
        traps: *mut cairo_traps_t,
        polygon: *const cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_polygon_init(
        polygon: *mut cairo_polygon_t,
        boxes: *const cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
    fn _cairo_stroker_dash_init(
        dash: *mut cairo_stroker_dash_t,
        style: *const cairo_stroke_style_t,
    );
    fn _cairo_stroker_dash_start(dash: *mut cairo_stroker_dash_t);
    fn _cairo_stroker_dash_step(dash: *mut cairo_stroker_dash_t, step: libc::c_double);
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_traps {
    pub status: cairo_status_t,
    pub bounds: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    #[bitfield(name = "maybe_region", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_intersections", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_rectilinear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "is_rectangular", ty = "libc::c_uint", bits = "3..=3")]
    pub maybe_region_has_intersections_is_rectilinear_is_rectangular: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub num_traps: libc::c_int,
    pub traps_size: libc::c_int,
    pub traps: *mut cairo_trapezoid_t,
    pub traps_embedded: [cairo_trapezoid_t; 16],
}
pub type cairo_trapezoid_t = _cairo_trapezoid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_trapezoid {
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub left: cairo_line_t,
    pub right: cairo_line_t,
}
pub type cairo_line_t = _cairo_line;
pub type cairo_traps_t = _cairo_traps;
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
pub type cairo_stroker_t = cairo_stroker;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_stroker {
    pub style: cairo_stroke_style_t,
    pub ctm: *const cairo_matrix_t,
    pub ctm_inverse: *const cairo_matrix_t,
    pub half_line_width: libc::c_double,
    pub tolerance: libc::c_double,
    pub spline_cusp_tolerance: libc::c_double,
    pub ctm_determinant: libc::c_double,
    pub ctm_det_positive: cairo_bool_t,
    pub closure: *mut libc::c_void,
    pub add_external_edge: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_point_t,
            *const cairo_point_t,
        ) -> cairo_status_t,
    >,
    pub add_triangle: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_point_t) -> cairo_status_t,
    >,
    pub add_triangle_fan: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_point_t,
            *const cairo_point_t,
            libc::c_int,
        ) -> cairo_status_t,
    >,
    pub add_convex_quad: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_point_t) -> cairo_status_t,
    >,
    pub pen: cairo_pen_t,
    pub current_point: cairo_point_t,
    pub first_point: cairo_point_t,
    pub has_initial_sub_path: cairo_bool_t,
    pub has_current_face: cairo_bool_t,
    pub current_face: cairo_stroke_face_t,
    pub has_first_face: cairo_bool_t,
    pub first_face: cairo_stroke_face_t,
    pub dash: cairo_stroker_dash_t,
    pub has_bounds: cairo_bool_t,
    pub bounds: cairo_box_t,
}
pub type cairo_stroker_dash_t = _cairo_stroker_dash;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_stroker_dash {
    pub dashed: cairo_bool_t,
    pub dash_index: libc::c_uint,
    pub dash_on: cairo_bool_t,
    pub dash_starts_on: cairo_bool_t,
    pub dash_remain: libc::c_double,
    pub dash_offset: libc::c_double,
    pub dashes: *const libc::c_double,
    pub num_dashes: libc::c_uint,
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
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _slow_segment_intersection(
    mut seg1_p1: *const cairo_point_t,
    mut seg1_p2: *const cairo_point_t,
    mut seg2_p1: *const cairo_point_t,
    mut seg2_p2: *const cairo_point_t,
    mut intersection: *mut cairo_point_t,
) -> cairo_bool_t {
    let mut denominator: libc::c_double = 0.;
    let mut u_a: libc::c_double = 0.;
    let mut u_b: libc::c_double = 0.;
    let mut seg1_dx: libc::c_double = 0.;
    let mut seg1_dy: libc::c_double = 0.;
    let mut seg2_dx: libc::c_double = 0.;
    let mut seg2_dy: libc::c_double = 0.;
    let mut seg_start_dx: libc::c_double = 0.;
    let mut seg_start_dy: libc::c_double = 0.;
    seg1_dx = _cairo_fixed_to_double((*seg1_p2).x - (*seg1_p1).x);
    seg1_dy = _cairo_fixed_to_double((*seg1_p2).y - (*seg1_p1).y);
    seg2_dx = _cairo_fixed_to_double((*seg2_p2).x - (*seg2_p1).x);
    seg2_dy = _cairo_fixed_to_double((*seg2_p2).y - (*seg2_p1).y);
    denominator = seg2_dy * seg1_dx - seg2_dx * seg1_dy;
    if denominator == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    seg_start_dx = _cairo_fixed_to_double((*seg1_p1).x - (*seg2_p1).x);
    seg_start_dy = _cairo_fixed_to_double((*seg1_p1).y - (*seg2_p1).y);
    u_a = (seg2_dx * seg_start_dy - seg2_dy * seg_start_dx) / denominator;
    u_b = (seg1_dx * seg_start_dy - seg1_dy * seg_start_dx) / denominator;
    if u_a <= 0 as libc::c_int as libc::c_double
        || u_a >= 1 as libc::c_int as libc::c_double
        || u_b <= 0 as libc::c_int as libc::c_double
        || u_b >= 1 as libc::c_int as libc::c_double
    {
        return 0 as libc::c_int;
    }
    (*intersection).x = (*seg1_p1).x + _cairo_fixed_from_double(u_a * seg1_dx);
    (*intersection).y = (*seg1_p1).y + _cairo_fixed_from_double(u_a * seg1_dy);
    return 1 as libc::c_int;
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
unsafe extern "C" fn _cairo_slope_init(
    mut slope: *mut cairo_slope_t,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
) {
    (*slope).dx = (*b).x - (*a).x;
    (*slope).dy = (*b).y - (*a).y;
}
unsafe extern "C" fn _cairo_stroker_limit(
    mut stroker: *mut cairo_stroker_t,
    mut path: *const cairo_path_fixed_t,
    mut boxes: *const cairo_box_t,
    mut num_boxes: libc::c_int,
) {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut fdx: cairo_fixed_t = 0;
    let mut fdy: cairo_fixed_t = 0;
    (*stroker).has_bounds = 1 as libc::c_int;
    _cairo_boxes_get_extents(boxes, num_boxes, &mut (*stroker).bounds);
    _cairo_stroke_style_max_distance_from_path(
        &mut (*stroker).style,
        path,
        (*stroker).ctm,
        &mut dx,
        &mut dy,
    );
    fdx = _cairo_fixed_from_double(dx);
    fdy = _cairo_fixed_from_double(dy);
    let ref mut fresh2 = (*stroker).bounds.p1.x;
    *fresh2 -= fdx;
    let ref mut fresh3 = (*stroker).bounds.p2.x;
    *fresh3 += fdx;
    let ref mut fresh4 = (*stroker).bounds.p1.y;
    *fresh4 -= fdy;
    let ref mut fresh5 = (*stroker).bounds.p2.y;
    *fresh5 += fdy;
}
unsafe extern "C" fn _cairo_stroker_init(
    mut stroker: *mut cairo_stroker_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut limits: *const cairo_box_t,
    mut num_limits: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*stroker).style = *stroke_style;
    let ref mut fresh6 = (*stroker).ctm;
    *fresh6 = ctm;
    let ref mut fresh7 = (*stroker).ctm_inverse;
    *fresh7 = ctm_inverse;
    (*stroker).tolerance = tolerance;
    (*stroker).half_line_width = (*stroke_style).line_width / 2.0f64;
    (*stroker)
        .spline_cusp_tolerance = 1 as libc::c_int as libc::c_double
        - tolerance / (*stroker).half_line_width;
    (*stroker).spline_cusp_tolerance *= (*stroker).spline_cusp_tolerance;
    (*stroker).spline_cusp_tolerance *= 2 as libc::c_int as libc::c_double;
    (*stroker).spline_cusp_tolerance -= 1 as libc::c_int as libc::c_double;
    (*stroker).ctm_determinant = _cairo_matrix_compute_determinant((*stroker).ctm);
    (*stroker).ctm_det_positive = ((*stroker).ctm_determinant >= 0.0f64) as libc::c_int;
    status = _cairo_pen_init(
        &mut (*stroker).pen,
        (*stroker).half_line_width,
        tolerance,
        ctm,
    );
    if status as u64 != 0 {
        return status;
    }
    (*stroker).has_current_face = 0 as libc::c_int;
    (*stroker).has_first_face = 0 as libc::c_int;
    (*stroker).has_initial_sub_path = 0 as libc::c_int;
    _cairo_stroker_dash_init(&mut (*stroker).dash, stroke_style);
    let ref mut fresh8 = (*stroker).add_external_edge;
    *fresh8 = None;
    (*stroker).has_bounds = 0 as libc::c_int;
    if num_limits != 0 {
        _cairo_stroker_limit(stroker, path, limits, num_limits);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_fini(mut stroker: *mut cairo_stroker_t) {
    _cairo_pen_fini(&mut (*stroker).pen);
}
unsafe extern "C" fn _translate_point(
    mut point: *mut cairo_point_t,
    mut offset: *const cairo_point_t,
) {
    let ref mut fresh9 = (*point).x;
    *fresh9 += (*offset).x;
    let ref mut fresh10 = (*point).y;
    *fresh10 += (*offset).y;
}
unsafe extern "C" fn _cairo_stroker_join_is_clockwise(
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
) -> libc::c_int {
    let mut in_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut out_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    _cairo_slope_init(&mut in_slope, &(*in_0).point, &(*in_0).cw);
    _cairo_slope_init(&mut out_slope, &(*out).point, &(*out).cw);
    return (_cairo_slope_compare(&mut in_slope, &mut out_slope) < 0 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_slope_compare_sgn(
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
unsafe extern "C" fn _tessellate_fan(
    mut stroker: *mut cairo_stroker_t,
    mut in_vector: *const cairo_slope_t,
    mut out_vector: *const cairo_slope_t,
    mut midpt: *const cairo_point_t,
    mut inpt: *const cairo_point_t,
    mut outpt: *const cairo_point_t,
    mut clockwise: cairo_bool_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut stack_points: [cairo_point_t; 64] = [cairo_point_t { x: 0, y: 0 }; 64];
    let mut points: *mut cairo_point_t = stack_points.as_mut_ptr();
    let mut pen: *mut cairo_pen_t = &mut (*stroker).pen;
    let mut start: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut num_points: libc::c_int = 0 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !((*stroker).has_bounds != 0
        && _cairo_box_contains_point(&mut (*stroker).bounds, midpt) == 0)
    {
        if (*stroker).pen.num_vertices != 0 {} else {
            __assert_fail(
                b"stroker->pen.num_vertices\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-path-stroke.c\0" as *const u8 as *const libc::c_char,
                250 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 179],
                    &[libc::c_char; 179],
                >(
                    b"cairo_status_t _tessellate_fan(cairo_stroker_t *, const cairo_slope_t *, const cairo_slope_t *, const cairo_point_t *, const cairo_point_t *, const cairo_point_t *, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if clockwise != 0 {
            _cairo_pen_find_active_ccw_vertices(
                pen,
                in_vector,
                out_vector,
                &mut start,
                &mut stop,
            );
            if ((*stroker).add_external_edge).is_some() {
                let mut last: cairo_point_t = cairo_point_t { x: 0, y: 0 };
                last = *inpt;
                while start != stop {
                    let mut p: cairo_point_t = *midpt;
                    _translate_point(
                        &mut p,
                        &mut (*((*pen).vertices).offset(start as isize)).point,
                    );
                    status = ((*stroker).add_external_edge)
                        .expect(
                            "non-null function pointer",
                        )((*stroker).closure, &mut last, &mut p);
                    if status as u64 != 0 {
                        return status;
                    }
                    last = p;
                    let fresh11 = start;
                    start = start - 1;
                    if fresh11 == 0 as libc::c_int {
                        start += (*pen).num_vertices;
                    }
                }
                status = ((*stroker).add_external_edge)
                    .expect(
                        "non-null function pointer",
                    )((*stroker).closure, &mut last, outpt);
                current_block = 13484060386966298149;
            } else if start == stop {
                current_block = 5299626394035605185;
            } else {
                num_points = stop - start;
                if num_points < 0 as libc::c_int {
                    num_points += (*pen).num_vertices;
                }
                num_points += 2 as libc::c_int;
                if num_points
                    > (::std::mem::size_of::<[cairo_point_t; 64]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
                        ) as libc::c_int
                {
                    points = _cairo_malloc_ab(
                        num_points as size_t,
                        ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
                    ) as *mut cairo_point_t;
                    if points.is_null() {
                        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    }
                }
                *points.offset(0 as libc::c_int as isize) = *inpt;
                num_points = 1 as libc::c_int;
                while start != stop {
                    *points.offset(num_points as isize) = *midpt;
                    _translate_point(
                        &mut *points.offset(num_points as isize),
                        &mut (*((*pen).vertices).offset(start as isize)).point,
                    );
                    num_points += 1;
                    let fresh12 = start;
                    start = start - 1;
                    if fresh12 == 0 as libc::c_int {
                        start += (*pen).num_vertices;
                    }
                }
                let fresh13 = num_points;
                num_points = num_points + 1;
                *points.offset(fresh13 as isize) = *outpt;
                current_block = 13484060386966298149;
            }
        } else {
            _cairo_pen_find_active_cw_vertices(
                pen,
                in_vector,
                out_vector,
                &mut start,
                &mut stop,
            );
            if ((*stroker).add_external_edge).is_some() {
                let mut last_0: cairo_point_t = cairo_point_t { x: 0, y: 0 };
                last_0 = *inpt;
                while start != stop {
                    let mut p_0: cairo_point_t = *midpt;
                    _translate_point(
                        &mut p_0,
                        &mut (*((*pen).vertices).offset(start as isize)).point,
                    );
                    status = ((*stroker).add_external_edge)
                        .expect(
                            "non-null function pointer",
                        )((*stroker).closure, &mut p_0, &mut last_0);
                    if status as u64 != 0 {
                        return status;
                    }
                    last_0 = p_0;
                    start += 1;
                    if start == (*pen).num_vertices {
                        start = 0 as libc::c_int;
                    }
                }
                status = ((*stroker).add_external_edge)
                    .expect(
                        "non-null function pointer",
                    )((*stroker).closure, outpt, &mut last_0);
                current_block = 13484060386966298149;
            } else if start == stop {
                current_block = 5299626394035605185;
            } else {
                num_points = stop - start;
                if num_points < 0 as libc::c_int {
                    num_points += (*pen).num_vertices;
                }
                num_points += 2 as libc::c_int;
                if num_points
                    > (::std::mem::size_of::<[cairo_point_t; 64]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
                        ) as libc::c_int
                {
                    points = _cairo_malloc_ab(
                        num_points as size_t,
                        ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
                    ) as *mut cairo_point_t;
                    if points.is_null() {
                        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    }
                }
                *points.offset(0 as libc::c_int as isize) = *inpt;
                num_points = 1 as libc::c_int;
                while start != stop {
                    *points.offset(num_points as isize) = *midpt;
                    _translate_point(
                        &mut *points.offset(num_points as isize),
                        &mut (*((*pen).vertices).offset(start as isize)).point,
                    );
                    num_points += 1;
                    start += 1;
                    if start == (*pen).num_vertices {
                        start = 0 as libc::c_int;
                    }
                }
                let fresh14 = num_points;
                num_points = num_points + 1;
                *points.offset(fresh14 as isize) = *outpt;
                current_block = 13484060386966298149;
            }
        }
        match current_block {
            5299626394035605185 => {}
            _ => {
                if num_points != 0 {
                    status = ((*stroker).add_triangle_fan)
                        .expect(
                            "non-null function pointer",
                        )((*stroker).closure, midpt, points, num_points);
                }
                if points != stack_points.as_mut_ptr() {
                    free(points as *mut libc::c_void);
                }
                return status;
            }
        }
    }
    if ((*stroker).add_external_edge).is_some() {
        if clockwise != 0 {
            return ((*stroker).add_external_edge)
                .expect("non-null function pointer")((*stroker).closure, inpt, outpt)
        } else {
            return ((*stroker).add_external_edge)
                .expect("non-null function pointer")((*stroker).closure, outpt, inpt)
        }
    } else {
        stack_points[0 as libc::c_int as usize] = *midpt;
        stack_points[1 as libc::c_int as usize] = *inpt;
        stack_points[2 as libc::c_int as usize] = *outpt;
        return ((*stroker).add_triangle)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, stack_points.as_mut_ptr() as *const cairo_point_t);
    };
}
unsafe extern "C" fn _cairo_stroker_join(
    mut stroker: *mut cairo_stroker_t,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
) -> cairo_status_t {
    let mut clockwise: libc::c_int = _cairo_stroker_join_is_clockwise(out, in_0);
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut points: [cairo_point_t; 4] = [cairo_point_t { x: 0, y: 0 }; 4];
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*in_0).cw.x == (*out).cw.x && (*in_0).cw.y == (*out).cw.y
        && (*in_0).ccw.x == (*out).ccw.x && (*in_0).ccw.y == (*out).ccw.y
    {
        return CAIRO_STATUS_SUCCESS;
    }
    if clockwise != 0 {
        if ((*stroker).add_external_edge).is_some() {
            status = ((*stroker).add_external_edge)
                .expect(
                    "non-null function pointer",
                )((*stroker).closure, &(*out).cw, &(*in_0).point);
            if status as u64 != 0 {
                return status;
            }
            status = ((*stroker).add_external_edge)
                .expect(
                    "non-null function pointer",
                )((*stroker).closure, &(*in_0).point, &(*in_0).cw);
            if status as u64 != 0 {
                return status;
            }
        }
        inpt = &(*in_0).ccw;
        outpt = &(*out).ccw;
    } else {
        if ((*stroker).add_external_edge).is_some() {
            status = ((*stroker).add_external_edge)
                .expect(
                    "non-null function pointer",
                )((*stroker).closure, &(*in_0).ccw, &(*in_0).point);
            if status as u64 != 0 {
                return status;
            }
            status = ((*stroker).add_external_edge)
                .expect(
                    "non-null function pointer",
                )((*stroker).closure, &(*in_0).point, &(*out).ccw);
            if status as u64 != 0 {
                return status;
            }
        }
        inpt = &(*in_0).cw;
        outpt = &(*out).cw;
    }
    match (*stroker).style.line_join as libc::c_uint {
        1 => {
            return _tessellate_fan(
                stroker,
                &(*in_0).dev_vector,
                &(*out).dev_vector,
                &(*in_0).point,
                inpt,
                outpt,
                clockwise,
            );
        }
        2 => {}
        0 | _ => {
            let mut in_dot_out: libc::c_double = -(*in_0).usr_vector.x
                * (*out).usr_vector.x + -(*in_0).usr_vector.y * (*out).usr_vector.y;
            let mut ml: libc::c_double = (*stroker).style.miter_limit;
            if 2 as libc::c_int as libc::c_double
                <= ml * ml * (1 as libc::c_int as libc::c_double - in_dot_out)
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
                dx1 = (*in_0).usr_vector.x;
                dy1 = (*in_0).usr_vector.y;
                cairo_matrix_transform_distance((*stroker).ctm, &mut dx1, &mut dy1);
                x2 = _cairo_fixed_to_double((*outpt).x);
                y2 = _cairo_fixed_to_double((*outpt).y);
                dx2 = (*out).usr_vector.x;
                dy2 = (*out).usr_vector.y;
                cairo_matrix_transform_distance((*stroker).ctm, &mut dx2, &mut dy2);
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
                if _cairo_slope_compare_sgn(fdx1, fdy1, mdx, mdy)
                    != _cairo_slope_compare_sgn(fdx2, fdy2, mdx, mdy)
                {
                    if ((*stroker).add_external_edge).is_some() {
                        points[0 as libc::c_int as usize]
                            .x = _cairo_fixed_from_double(mx);
                        points[0 as libc::c_int as usize]
                            .y = _cairo_fixed_from_double(my);
                        if clockwise != 0 {
                            status = ((*stroker).add_external_edge)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*stroker).closure,
                                inpt,
                                &mut *points.as_mut_ptr().offset(0 as libc::c_int as isize),
                            );
                            if status as u64 != 0 {
                                return status;
                            }
                            status = ((*stroker).add_external_edge)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*stroker).closure,
                                &mut *points.as_mut_ptr().offset(0 as libc::c_int as isize),
                                outpt,
                            );
                            if status as u64 != 0 {
                                return status;
                            }
                        } else {
                            status = ((*stroker).add_external_edge)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*stroker).closure,
                                outpt,
                                &mut *points.as_mut_ptr().offset(0 as libc::c_int as isize),
                            );
                            if status as u64 != 0 {
                                return status;
                            }
                            status = ((*stroker).add_external_edge)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*stroker).closure,
                                &mut *points.as_mut_ptr().offset(0 as libc::c_int as isize),
                                inpt,
                            );
                            if status as u64 != 0 {
                                return status;
                            }
                        }
                        return CAIRO_STATUS_SUCCESS;
                    } else {
                        points[0 as libc::c_int as usize] = (*in_0).point;
                        points[1 as libc::c_int as usize] = *inpt;
                        points[2 as libc::c_int as usize]
                            .x = _cairo_fixed_from_double(mx);
                        points[2 as libc::c_int as usize]
                            .y = _cairo_fixed_from_double(my);
                        points[3 as libc::c_int as usize] = *outpt;
                        return ((*stroker).add_convex_quad)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*stroker).closure,
                            points.as_mut_ptr() as *const cairo_point_t,
                        );
                    }
                }
            }
        }
    }
    if ((*stroker).add_external_edge).is_some() {
        if clockwise != 0 {
            return ((*stroker).add_external_edge)
                .expect("non-null function pointer")((*stroker).closure, inpt, outpt)
        } else {
            return ((*stroker).add_external_edge)
                .expect("non-null function pointer")((*stroker).closure, outpt, inpt)
        }
    } else {
        points[0 as libc::c_int as usize] = (*in_0).point;
        points[1 as libc::c_int as usize] = *inpt;
        points[2 as libc::c_int as usize] = *outpt;
        return ((*stroker).add_triangle)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, points.as_mut_ptr() as *const cairo_point_t);
    };
}
unsafe extern "C" fn _cairo_stroker_add_cap(
    mut stroker: *mut cairo_stroker_t,
    mut f: *const cairo_stroke_face_t,
) -> cairo_status_t {
    match (*stroker).style.line_cap as libc::c_uint {
        1 => {
            let mut slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            slope.dx = -(*f).dev_vector.dx;
            slope.dy = -(*f).dev_vector.dy;
            return _tessellate_fan(
                stroker,
                &(*f).dev_vector,
                &mut slope,
                &(*f).point,
                &(*f).cw,
                &(*f).ccw,
                0 as libc::c_int,
            );
        }
        2 => {
            let mut dx: libc::c_double = 0.;
            let mut dy: libc::c_double = 0.;
            let mut fvector: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            let mut quad: [cairo_point_t; 4] = [cairo_point_t { x: 0, y: 0 }; 4];
            dx = (*f).usr_vector.x;
            dy = (*f).usr_vector.y;
            dx *= (*stroker).half_line_width;
            dy *= (*stroker).half_line_width;
            cairo_matrix_transform_distance((*stroker).ctm, &mut dx, &mut dy);
            fvector.dx = _cairo_fixed_from_double(dx);
            fvector.dy = _cairo_fixed_from_double(dy);
            quad[0 as libc::c_int as usize] = (*f).ccw;
            quad[1 as libc::c_int as usize].x = (*f).ccw.x + fvector.dx;
            quad[1 as libc::c_int as usize].y = (*f).ccw.y + fvector.dy;
            quad[2 as libc::c_int as usize].x = (*f).cw.x + fvector.dx;
            quad[2 as libc::c_int as usize].y = (*f).cw.y + fvector.dy;
            quad[3 as libc::c_int as usize] = (*f).cw;
            if ((*stroker).add_external_edge).is_some() {
                let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
                status = ((*stroker).add_external_edge)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*stroker).closure,
                    &mut *quad.as_mut_ptr().offset(0 as libc::c_int as isize),
                    &mut *quad.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                if status as u64 != 0 {
                    return status;
                }
                status = ((*stroker).add_external_edge)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*stroker).closure,
                    &mut *quad.as_mut_ptr().offset(1 as libc::c_int as isize),
                    &mut *quad.as_mut_ptr().offset(2 as libc::c_int as isize),
                );
                if status as u64 != 0 {
                    return status;
                }
                status = ((*stroker).add_external_edge)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*stroker).closure,
                    &mut *quad.as_mut_ptr().offset(2 as libc::c_int as isize),
                    &mut *quad.as_mut_ptr().offset(3 as libc::c_int as isize),
                );
                if status as u64 != 0 {
                    return status;
                }
                return CAIRO_STATUS_SUCCESS;
            } else {
                return ((*stroker).add_convex_quad)
                    .expect(
                        "non-null function pointer",
                    )((*stroker).closure, quad.as_mut_ptr() as *const cairo_point_t)
            }
        }
        0 | _ => {
            if ((*stroker).add_external_edge).is_some() {
                return ((*stroker).add_external_edge)
                    .expect(
                        "non-null function pointer",
                    )((*stroker).closure, &(*f).ccw, &(*f).cw)
            } else {
                return CAIRO_STATUS_SUCCESS
            }
        }
    };
}
unsafe extern "C" fn _cairo_stroker_add_leading_cap(
    mut stroker: *mut cairo_stroker_t,
    mut face: *const cairo_stroke_face_t,
) -> cairo_status_t {
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
    return _cairo_stroker_add_cap(stroker, &mut reversed);
}
unsafe extern "C" fn _cairo_stroker_add_trailing_cap(
    mut stroker: *mut cairo_stroker_t,
    mut face: *const cairo_stroke_face_t,
) -> cairo_status_t {
    return _cairo_stroker_add_cap(stroker, face);
}
#[inline]
unsafe extern "C" fn _compute_normalized_device_slope(
    mut dx: *mut libc::c_double,
    mut dy: *mut libc::c_double,
    mut ctm_inverse: *const cairo_matrix_t,
    mut mag_out: *mut libc::c_double,
) -> cairo_bool_t {
    let mut dx0: libc::c_double = *dx;
    let mut dy0: libc::c_double = *dy;
    let mut mag: libc::c_double = 0.;
    cairo_matrix_transform_distance(ctm_inverse, &mut dx0, &mut dy0);
    if dx0 == 0.0f64 && dy0 == 0.0f64 {
        if !mag_out.is_null() {
            *mag_out = 0.0f64;
        }
        return 0 as libc::c_int;
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
    if !mag_out.is_null() {
        *mag_out = mag;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _compute_face(
    mut point: *const cairo_point_t,
    mut dev_slope: *const cairo_slope_t,
    mut slope_dx: libc::c_double,
    mut slope_dy: libc::c_double,
    mut stroker: *mut cairo_stroker_t,
    mut face: *mut cairo_stroke_face_t,
) {
    let mut face_dx: libc::c_double = 0.;
    let mut face_dy: libc::c_double = 0.;
    let mut offset_ccw: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut offset_cw: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    if (*stroker).ctm_det_positive != 0 {
        face_dx = -slope_dy * (*stroker).half_line_width;
        face_dy = slope_dx * (*stroker).half_line_width;
    } else {
        face_dx = slope_dy * (*stroker).half_line_width;
        face_dy = -slope_dx * (*stroker).half_line_width;
    }
    cairo_matrix_transform_distance((*stroker).ctm, &mut face_dx, &mut face_dy);
    offset_ccw.x = _cairo_fixed_from_double(face_dx);
    offset_ccw.y = _cairo_fixed_from_double(face_dy);
    offset_cw.x = -offset_ccw.x;
    offset_cw.y = -offset_ccw.y;
    (*face).ccw = *point;
    _translate_point(&mut (*face).ccw, &mut offset_ccw);
    (*face).point = *point;
    (*face).cw = *point;
    _translate_point(&mut (*face).cw, &mut offset_cw);
    (*face).usr_vector.x = slope_dx;
    (*face).usr_vector.y = slope_dy;
    (*face).dev_vector = *dev_slope;
}
unsafe extern "C" fn _cairo_stroker_add_caps(
    mut stroker: *mut cairo_stroker_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stroker).has_initial_sub_path != 0 && (*stroker).has_first_face == 0
        && (*stroker).has_current_face == 0
        && (*stroker).style.line_cap as libc::c_uint
            == CAIRO_LINE_CAP_ROUND as libc::c_int as libc::c_uint
    {
        let mut dx: libc::c_double = 1.0f64;
        let mut dy: libc::c_double = 0.0f64;
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
        _compute_normalized_device_slope(
            &mut dx,
            &mut dy,
            (*stroker).ctm_inverse,
            0 as *mut libc::c_double,
        );
        _compute_face(
            &mut (*stroker).first_point,
            &mut slope,
            dx,
            dy,
            stroker,
            &mut face,
        );
        status = _cairo_stroker_add_leading_cap(stroker, &mut face);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_stroker_add_trailing_cap(stroker, &mut face);
        if status as u64 != 0 {
            return status;
        }
    }
    if (*stroker).has_first_face != 0 {
        status = _cairo_stroker_add_leading_cap(stroker, &mut (*stroker).first_face);
        if status as u64 != 0 {
            return status;
        }
    }
    if (*stroker).has_current_face != 0 {
        status = _cairo_stroker_add_trailing_cap(stroker, &mut (*stroker).current_face);
        if status as u64 != 0 {
            return status;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_add_sub_edge(
    mut stroker: *mut cairo_stroker_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut dev_slope: *mut cairo_slope_t,
    mut slope_dx: libc::c_double,
    mut slope_dy: libc::c_double,
    mut start: *mut cairo_stroke_face_t,
    mut end: *mut cairo_stroke_face_t,
) -> cairo_status_t {
    _compute_face(p1, dev_slope, slope_dx, slope_dy, stroker, start);
    *end = *start;
    if (*p1).x == (*p2).x && (*p1).y == (*p2).y {
        return CAIRO_STATUS_SUCCESS;
    }
    (*end).point = *p2;
    let ref mut fresh15 = (*end).ccw.x;
    *fresh15 += (*p2).x - (*p1).x;
    let ref mut fresh16 = (*end).ccw.y;
    *fresh16 += (*p2).y - (*p1).y;
    let ref mut fresh17 = (*end).cw.x;
    *fresh17 += (*p2).x - (*p1).x;
    let ref mut fresh18 = (*end).cw.y;
    *fresh18 += (*p2).y - (*p1).y;
    if ((*stroker).add_external_edge).is_some() {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = ((*stroker).add_external_edge)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, &mut (*end).cw, &mut (*start).cw);
        if status as u64 != 0 {
            return status;
        }
        status = ((*stroker).add_external_edge)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, &mut (*start).ccw, &mut (*end).ccw);
        if status as u64 != 0 {
            return status;
        }
        return CAIRO_STATUS_SUCCESS;
    } else {
        let mut quad: [cairo_point_t; 4] = [cairo_point_t { x: 0, y: 0 }; 4];
        quad[0 as libc::c_int as usize] = (*start).cw;
        quad[1 as libc::c_int as usize] = (*end).cw;
        quad[2 as libc::c_int as usize] = (*end).ccw;
        quad[3 as libc::c_int as usize] = (*start).ccw;
        return ((*stroker).add_convex_quad)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, quad.as_mut_ptr() as *const cairo_point_t);
    };
}
unsafe extern "C" fn _cairo_stroker_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_stroker_t = closure as *mut cairo_stroker_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_stroker_dash_start(&mut (*stroker).dash);
    status = _cairo_stroker_add_caps(stroker);
    if status as u64 != 0 {
        return status;
    }
    (*stroker).first_point = *point;
    (*stroker).current_point = *point;
    (*stroker).has_first_face = 0 as libc::c_int;
    (*stroker).has_current_face = 0 as libc::c_int;
    (*stroker).has_initial_sub_path = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_stroker_t = closure as *mut cairo_stroker_t;
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
    let mut end: cairo_stroke_face_t = cairo_stroke_face_t {
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
    let mut p1: *mut cairo_point_t = &mut (*stroker).current_point;
    let mut dev_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut slope_dx: libc::c_double = 0.;
    let mut slope_dy: libc::c_double = 0.;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*stroker).has_initial_sub_path = 1 as libc::c_int;
    if (*p1).x == (*point).x && (*p1).y == (*point).y {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_slope_init(&mut dev_slope, p1, point);
    slope_dx = _cairo_fixed_to_double((*point).x - (*p1).x);
    slope_dy = _cairo_fixed_to_double((*point).y - (*p1).y);
    _compute_normalized_device_slope(
        &mut slope_dx,
        &mut slope_dy,
        (*stroker).ctm_inverse,
        0 as *mut libc::c_double,
    );
    status = _cairo_stroker_add_sub_edge(
        stroker,
        p1,
        point,
        &mut dev_slope,
        slope_dx,
        slope_dy,
        &mut start,
        &mut end,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*stroker).has_current_face != 0 {
        status = _cairo_stroker_join(stroker, &mut (*stroker).current_face, &mut start);
        if status as u64 != 0 {
            return status;
        }
    } else if (*stroker).has_first_face == 0 {
        (*stroker).first_face = start;
        (*stroker).has_first_face = 1 as libc::c_int;
    }
    (*stroker).current_face = end;
    (*stroker).has_current_face = 1 as libc::c_int;
    (*stroker).current_point = *point;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_add_point_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    return _cairo_stroker_line_to(closure, point);
}
unsafe extern "C" fn _cairo_stroker_spline_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_stroker_t = closure as *mut cairo_stroker_t;
    let mut new_face: cairo_stroke_face_t = cairo_stroke_face_t {
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
    let mut slope_dx: libc::c_double = 0.;
    let mut slope_dy: libc::c_double = 0.;
    let mut points: [cairo_point_t; 3] = [cairo_point_t { x: 0, y: 0 }; 3];
    let mut intersect_point: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    (*stroker).has_initial_sub_path = 1 as libc::c_int;
    if (*stroker).current_point.x == (*point).x
        && (*stroker).current_point.y == (*point).y
    {
        return CAIRO_STATUS_SUCCESS;
    }
    slope_dx = _cairo_fixed_to_double((*tangent).dx);
    slope_dy = _cairo_fixed_to_double((*tangent).dy);
    if _compute_normalized_device_slope(
        &mut slope_dx,
        &mut slope_dy,
        (*stroker).ctm_inverse,
        0 as *mut libc::c_double,
    ) == 0
    {
        return CAIRO_STATUS_SUCCESS;
    }
    _compute_face(point, tangent, slope_dx, slope_dy, stroker, &mut new_face);
    if (*stroker).has_current_face != 0 {} else {
        __assert_fail(
            b"stroker->has_current_face\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-stroke.c\0" as *const u8 as *const libc::c_char,
            1034 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"cairo_status_t _cairo_stroker_spline_to(void *, const cairo_point_t *, const cairo_slope_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if new_face.dev_slope.x * (*stroker).current_face.dev_slope.x
        + new_face.dev_slope.y * (*stroker).current_face.dev_slope.y
        < (*stroker).spline_cusp_tolerance
    {
        let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
        let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
        let mut clockwise: libc::c_int = _cairo_stroker_join_is_clockwise(
            &mut new_face,
            &mut (*stroker).current_face,
        );
        if clockwise != 0 {
            inpt = &mut (*stroker).current_face.cw;
            outpt = &mut new_face.cw;
        } else {
            inpt = &mut (*stroker).current_face.ccw;
            outpt = &mut new_face.ccw;
        }
        _tessellate_fan(
            stroker,
            &mut (*stroker).current_face.dev_vector,
            &mut new_face.dev_vector,
            &mut (*stroker).current_face.point,
            inpt,
            outpt,
            clockwise,
        );
    }
    if _slow_segment_intersection(
        &mut (*stroker).current_face.cw,
        &mut (*stroker).current_face.ccw,
        &mut new_face.cw,
        &mut new_face.ccw,
        &mut intersect_point,
    ) != 0
    {
        points[0 as libc::c_int as usize] = (*stroker).current_face.ccw;
        points[1 as libc::c_int as usize] = new_face.ccw;
        points[2 as libc::c_int as usize] = intersect_point;
        ((*stroker).add_triangle)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, points.as_mut_ptr() as *const cairo_point_t);
        points[0 as libc::c_int as usize] = (*stroker).current_face.cw;
        points[1 as libc::c_int as usize] = new_face.cw;
        ((*stroker).add_triangle)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, points.as_mut_ptr() as *const cairo_point_t);
    } else {
        points[0 as libc::c_int as usize] = (*stroker).current_face.ccw;
        points[1 as libc::c_int as usize] = (*stroker).current_face.cw;
        points[2 as libc::c_int as usize] = new_face.cw;
        ((*stroker).add_triangle)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, points.as_mut_ptr() as *const cairo_point_t);
        points[0 as libc::c_int as usize] = (*stroker).current_face.ccw;
        points[1 as libc::c_int as usize] = new_face.cw;
        points[2 as libc::c_int as usize] = new_face.ccw;
        ((*stroker).add_triangle)
            .expect(
                "non-null function pointer",
            )((*stroker).closure, points.as_mut_ptr() as *const cairo_point_t);
    }
    (*stroker).current_face = new_face;
    (*stroker).has_current_face = 1 as libc::c_int;
    (*stroker).current_point = *point;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_line_to_dashed(
    mut closure: *mut libc::c_void,
    mut p2: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_stroker_t = closure as *mut cairo_stroker_t;
    let mut mag: libc::c_double = 0.;
    let mut remain: libc::c_double = 0.;
    let mut step_length: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut slope_dx: libc::c_double = 0.;
    let mut slope_dy: libc::c_double = 0.;
    let mut dx2: libc::c_double = 0.;
    let mut dy2: libc::c_double = 0.;
    let mut sub_start: cairo_stroke_face_t = cairo_stroke_face_t {
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
    let mut sub_end: cairo_stroke_face_t = cairo_stroke_face_t {
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
    let mut p1: *mut cairo_point_t = &mut (*stroker).current_point;
    let mut dev_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut segment: cairo_line_t = cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut fully_in_bounds: cairo_bool_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*stroker).has_initial_sub_path = (*stroker).dash.dash_starts_on;
    if (*p1).x == (*p2).x && (*p1).y == (*p2).y {
        return CAIRO_STATUS_SUCCESS;
    }
    fully_in_bounds = 1 as libc::c_int;
    if (*stroker).has_bounds != 0
        && (_cairo_box_contains_point(&mut (*stroker).bounds, p1) == 0
            || _cairo_box_contains_point(&mut (*stroker).bounds, p2) == 0)
    {
        fully_in_bounds = 0 as libc::c_int;
    }
    _cairo_slope_init(&mut dev_slope, p1, p2);
    slope_dx = _cairo_fixed_to_double((*p2).x - (*p1).x);
    slope_dy = _cairo_fixed_to_double((*p2).y - (*p1).y);
    if _compute_normalized_device_slope(
        &mut slope_dx,
        &mut slope_dy,
        (*stroker).ctm_inverse,
        &mut mag,
    ) == 0
    {
        return CAIRO_STATUS_SUCCESS;
    }
    remain = mag;
    segment.p1 = *p1;
    while remain != 0. {
        step_length = if (*stroker).dash.dash_remain < remain {
            (*stroker).dash.dash_remain
        } else {
            remain
        };
        remain -= step_length;
        dx2 = slope_dx * (mag - remain);
        dy2 = slope_dy * (mag - remain);
        cairo_matrix_transform_distance((*stroker).ctm, &mut dx2, &mut dy2);
        segment.p2.x = _cairo_fixed_from_double(dx2) + (*p1).x;
        segment.p2.y = _cairo_fixed_from_double(dy2) + (*p1).y;
        if (*stroker).dash.dash_on != 0
            && (fully_in_bounds != 0
                || (*stroker).has_first_face == 0 && (*stroker).dash.dash_starts_on != 0
                || _cairo_box_intersects_line_segment(
                    &mut (*stroker).bounds,
                    &mut segment,
                ) != 0)
        {
            status = _cairo_stroker_add_sub_edge(
                stroker,
                &mut segment.p1,
                &mut segment.p2,
                &mut dev_slope,
                slope_dx,
                slope_dy,
                &mut sub_start,
                &mut sub_end,
            );
            if status as u64 != 0 {
                return status;
            }
            if (*stroker).has_current_face != 0 {
                status = _cairo_stroker_join(
                    stroker,
                    &mut (*stroker).current_face,
                    &mut sub_start,
                );
                if status as u64 != 0 {
                    return status;
                }
                (*stroker).has_current_face = 0 as libc::c_int;
            } else if (*stroker).has_first_face == 0
                && (*stroker).dash.dash_starts_on != 0
            {
                (*stroker).first_face = sub_start;
                (*stroker).has_first_face = 1 as libc::c_int;
            } else {
                status = _cairo_stroker_add_leading_cap(stroker, &mut sub_start);
                if status as u64 != 0 {
                    return status;
                }
            }
            if remain != 0. {
                status = _cairo_stroker_add_trailing_cap(stroker, &mut sub_end);
                if status as u64 != 0 {
                    return status;
                }
            } else {
                (*stroker).current_face = sub_end;
                (*stroker).has_current_face = 1 as libc::c_int;
            }
        } else if (*stroker).has_current_face != 0 {
            status = _cairo_stroker_add_trailing_cap(
                stroker,
                &mut (*stroker).current_face,
            );
            if status as u64 != 0 {
                return status;
            }
            (*stroker).has_current_face = 0 as libc::c_int;
        }
        _cairo_stroker_dash_step(&mut (*stroker).dash, step_length);
        segment.p1 = segment.p2;
    }
    if (*stroker).dash.dash_on != 0 && (*stroker).has_current_face == 0 {
        _compute_face(
            p2,
            &mut dev_slope,
            slope_dx,
            slope_dy,
            stroker,
            &mut (*stroker).current_face,
        );
        status = _cairo_stroker_add_leading_cap(stroker, &mut (*stroker).current_face);
        if status as u64 != 0 {
            return status;
        }
        (*stroker).has_current_face = 1 as libc::c_int;
    }
    (*stroker).current_point = *p2;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_add_point_line_to_dashed(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    return _cairo_stroker_line_to_dashed(closure, point);
}
unsafe extern "C" fn _cairo_stroker_curve_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_stroker_t = closure as *mut cairo_stroker_t;
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
    let mut line_join_save: cairo_line_join_t = CAIRO_LINE_JOIN_MITER;
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
    let mut slope_dx: libc::c_double = 0.;
    let mut slope_dy: libc::c_double = 0.;
    let mut line_to: cairo_spline_add_point_func_t = None;
    let mut spline_to: cairo_spline_add_point_func_t = None;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    line_to = if (*stroker).dash.dashed != 0 {
        Some(
            _cairo_stroker_add_point_line_to_dashed
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        )
    } else {
        Some(
            _cairo_stroker_add_point_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        )
    };
    spline_to = if (*stroker).dash.dashed != 0 {
        Some(
            _cairo_stroker_add_point_line_to_dashed
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        )
    } else {
        Some(
            _cairo_stroker_spline_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        )
    };
    if _cairo_spline_init(
        &mut spline,
        spline_to,
        stroker as *mut libc::c_void,
        &mut (*stroker).current_point,
        b,
        c,
        d,
    ) == 0
    {
        let mut fallback_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
        _cairo_slope_init(&mut fallback_slope, &mut (*stroker).current_point, d);
        return line_to
            .expect("non-null function pointer")(closure, d, &mut fallback_slope);
    }
    if (*stroker).pen.num_vertices <= 1 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*stroker).dash.dashed == 0 || (*stroker).dash.dash_on != 0 {
        slope_dx = _cairo_fixed_to_double(spline.initial_slope.dx);
        slope_dy = _cairo_fixed_to_double(spline.initial_slope.dy);
        if _compute_normalized_device_slope(
            &mut slope_dx,
            &mut slope_dy,
            (*stroker).ctm_inverse,
            0 as *mut libc::c_double,
        ) != 0
        {
            _compute_face(
                &mut (*stroker).current_point,
                &mut spline.initial_slope,
                slope_dx,
                slope_dy,
                stroker,
                &mut face,
            );
        }
        if (*stroker).has_current_face != 0 {
            status = _cairo_stroker_join(
                stroker,
                &mut (*stroker).current_face,
                &mut face,
            );
            if status as u64 != 0 {
                return status;
            }
        } else if (*stroker).has_first_face == 0 {
            (*stroker).first_face = face;
            (*stroker).has_first_face = 1 as libc::c_int;
        }
        (*stroker).current_face = face;
        (*stroker).has_current_face = 1 as libc::c_int;
    }
    line_join_save = (*stroker).style.line_join;
    (*stroker).style.line_join = CAIRO_LINE_JOIN_ROUND;
    status = _cairo_spline_decompose(&mut spline, (*stroker).tolerance);
    if status as u64 != 0 {
        return status;
    }
    if (*stroker).dash.dashed == 0 || (*stroker).dash.dash_on != 0 {
        slope_dx = _cairo_fixed_to_double(spline.final_slope.dx);
        slope_dy = _cairo_fixed_to_double(spline.final_slope.dy);
        if _compute_normalized_device_slope(
            &mut slope_dx,
            &mut slope_dy,
            (*stroker).ctm_inverse,
            0 as *mut libc::c_double,
        ) != 0
        {
            _compute_face(
                &mut (*stroker).current_point,
                &mut spline.final_slope,
                slope_dx,
                slope_dy,
                stroker,
                &mut face,
            );
        }
        status = _cairo_stroker_join(stroker, &mut (*stroker).current_face, &mut face);
        if status as u64 != 0 {
            return status;
        }
        (*stroker).current_face = face;
    }
    (*stroker).style.line_join = line_join_save;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_stroker_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut stroker: *mut cairo_stroker_t = closure as *mut cairo_stroker_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stroker).dash.dashed != 0 {
        status = _cairo_stroker_line_to_dashed(
            stroker as *mut libc::c_void,
            &mut (*stroker).first_point,
        );
    } else {
        status = _cairo_stroker_line_to(
            stroker as *mut libc::c_void,
            &mut (*stroker).first_point,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    if (*stroker).has_first_face != 0 && (*stroker).has_current_face != 0 {
        status = _cairo_stroker_join(
            stroker,
            &mut (*stroker).current_face,
            &mut (*stroker).first_face,
        );
        if status as u64 != 0 {
            return status;
        }
    } else {
        status = _cairo_stroker_add_caps(stroker);
        if status as u64 != 0 {
            return status;
        }
    }
    (*stroker).has_initial_sub_path = 0 as libc::c_int;
    (*stroker).has_first_face = 0 as libc::c_int;
    (*stroker).has_current_face = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_to_shaper(
    mut path: *mut cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut add_triangle: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_point_t) -> cairo_status_t,
    >,
    mut add_triangle_fan: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_point_t,
            *const cairo_point_t,
            libc::c_int,
        ) -> cairo_status_t,
    >,
    mut add_convex_quad: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_point_t) -> cairo_status_t,
    >,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut stroker: cairo_stroker_t = cairo_stroker_t {
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
        ctm: 0 as *const cairo_matrix_t,
        ctm_inverse: 0 as *const cairo_matrix_t,
        half_line_width: 0.,
        tolerance: 0.,
        spline_cusp_tolerance: 0.,
        ctm_determinant: 0.,
        ctm_det_positive: 0,
        closure: 0 as *mut libc::c_void,
        add_external_edge: None,
        add_triangle: None,
        add_triangle_fan: None,
        add_convex_quad: None,
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
        current_point: cairo_point_t { x: 0, y: 0 },
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
        dash: cairo_stroker_dash_t {
            dashed: 0,
            dash_index: 0,
            dash_on: 0,
            dash_starts_on: 0,
            dash_remain: 0.,
            dash_offset: 0.,
            dashes: 0 as *const libc::c_double,
            num_dashes: 0,
        },
        has_bounds: 0,
        bounds: cairo_line_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_stroker_init(
        &mut stroker,
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        0 as *const cairo_box_t,
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    stroker.add_triangle = add_triangle;
    stroker.add_triangle_fan = add_triangle_fan;
    stroker.add_convex_quad = add_convex_quad;
    stroker.closure = closure;
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _cairo_stroker_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        if stroker.dash.dashed != 0 {
            Some(
                _cairo_stroker_line_to_dashed
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            )
        } else {
            Some(
                _cairo_stroker_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            )
        },
        Some(
            _cairo_stroker_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_stroker_close_path
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut stroker as *mut cairo_stroker_t as *mut libc::c_void,
    );
    if !(status as u64 != 0) {
        status = _cairo_stroker_add_caps(&mut stroker);
    }
    _cairo_stroker_fini(&mut stroker);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_dashed_to_polygon(
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut polygon: *mut cairo_polygon_t,
) -> cairo_status_t {
    let mut stroker: cairo_stroker_t = cairo_stroker_t {
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
        ctm: 0 as *const cairo_matrix_t,
        ctm_inverse: 0 as *const cairo_matrix_t,
        half_line_width: 0.,
        tolerance: 0.,
        spline_cusp_tolerance: 0.,
        ctm_determinant: 0.,
        ctm_det_positive: 0,
        closure: 0 as *mut libc::c_void,
        add_external_edge: None,
        add_triangle: None,
        add_triangle_fan: None,
        add_convex_quad: None,
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
        current_point: cairo_point_t { x: 0, y: 0 },
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
        dash: cairo_stroker_dash_t {
            dashed: 0,
            dash_index: 0,
            dash_on: 0,
            dash_starts_on: 0,
            dash_remain: 0.,
            dash_offset: 0.,
            dashes: 0 as *const libc::c_double,
            num_dashes: 0,
        },
        has_bounds: 0,
        bounds: cairo_line_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_stroker_init(
        &mut stroker,
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        (*polygon).limits,
        (*polygon).num_limits,
    );
    if status as u64 != 0 {
        return status;
    }
    stroker
        .add_external_edge = Some(
        _cairo_polygon_add_external_edge
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *const cairo_point_t,
                *const cairo_point_t,
            ) -> cairo_status_t,
    );
    stroker.closure = polygon as *mut libc::c_void;
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _cairo_stroker_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        if stroker.dash.dashed != 0 {
            Some(
                _cairo_stroker_line_to_dashed
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            )
        } else {
            Some(
                _cairo_stroker_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            )
        },
        Some(
            _cairo_stroker_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_stroker_close_path
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut stroker as *mut cairo_stroker_t as *mut libc::c_void,
    );
    if !(status as u64 != 0) {
        status = _cairo_stroker_add_caps(&mut stroker);
    }
    _cairo_stroker_fini(&mut stroker);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_polygon_to_traps(
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut traps: *mut cairo_traps_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_line_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_line_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_line_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    _cairo_polygon_init(&mut polygon, (*traps).limits, (*traps).num_limits);
    status = _cairo_path_fixed_stroke_to_polygon(
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        &mut polygon,
    ) as cairo_int_status_t;
    if !(status as u64 != 0) {
        status = (*(&mut polygon as *mut cairo_polygon_t)).status as cairo_int_status_t;
        if !(status as u64 != 0) {
            status = _cairo_bentley_ottmann_tessellate_polygon(
                traps,
                &mut polygon,
                CAIRO_FILL_RULE_WINDING,
            ) as cairo_int_status_t;
        }
    }
    _cairo_polygon_fini(&mut polygon);
    return status;
}
