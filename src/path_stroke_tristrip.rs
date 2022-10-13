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
    fn _cairo_pen_find_active_ccw_vertex_index(
        pen: *const cairo_pen_t,
        slope: *const cairo_slope_t,
    ) -> libc::c_int;
    fn _cairo_pen_find_active_cw_vertex_index(
        pen: *const cairo_pen_t,
        slope: *const cairo_slope_t,
    ) -> libc::c_int;
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
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
    fn _cairo_tristrip_move_to(
        strip: *mut cairo_tristrip_t,
        point: *const cairo_point_t,
    );
    fn _cairo_tristrip_add_point(
        strip: *mut cairo_tristrip_t,
        point: *const cairo_point_t,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tristrip {
    pub status: cairo_status_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_points: libc::c_int,
    pub size_points: libc::c_int,
    pub points: *mut cairo_point_t,
    pub points_embedded: [cairo_point_t; 64],
}
pub type cairo_tristrip_t = _cairo_tristrip;
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
    pub strip: *mut cairo_tristrip_t,
    pub ctm: *const cairo_matrix_t,
    pub ctm_inverse: *const cairo_matrix_t,
    pub tolerance: libc::c_double,
    pub ctm_det_positive: cairo_bool_t,
    pub pen: cairo_pen_t,
    pub has_sub_path: cairo_bool_t,
    pub first_point: cairo_point_t,
    pub has_current_face: cairo_bool_t,
    pub current_face: cairo_stroke_face_t,
    pub has_first_face: cairo_bool_t,
    pub first_face: cairo_stroke_face_t,
    pub limit: cairo_box_t,
    pub has_limits: cairo_bool_t,
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
unsafe extern "C" fn _cairo_slope_init(
    mut slope: *mut cairo_slope_t,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
) {
    (*slope).dx = (*b).x - (*a).x;
    (*slope).dy = (*b).y - (*a).y;
}
unsafe extern "C" fn translate_point(
    mut point: *mut cairo_point_t,
    mut offset: *const cairo_point_t,
) {
    let ref mut fresh0 = (*point).x;
    *fresh0 += (*offset).x;
    let ref mut fresh1 = (*point).y;
    *fresh1 += (*offset).y;
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
#[inline]
unsafe extern "C" fn range_step(
    mut i: libc::c_int,
    mut step: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    i += step;
    if i < 0 as libc::c_int {
        i = max - 1 as libc::c_int;
    }
    if i >= max {
        i = 0 as libc::c_int;
    }
    return i;
}
unsafe extern "C" fn add_fan(
    mut stroker: *mut stroker,
    mut in_vector: *const cairo_slope_t,
    mut out_vector: *const cairo_slope_t,
    mut midpt: *const cairo_point_t,
    mut inpt: *const cairo_point_t,
    mut outpt: *const cairo_point_t,
    mut clockwise: cairo_bool_t,
) {
    let mut start: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut npoints: libc::c_int = 0;
    if clockwise != 0 {
        step = 1 as libc::c_int;
        start = _cairo_pen_find_active_cw_vertex_index(&mut (*stroker).pen, in_vector);
        if _cairo_slope_compare(
            &mut (*((*stroker).pen.vertices).offset(start as isize)).slope_cw,
            in_vector,
        ) < 0 as libc::c_int
        {
            start = range_step(start, 1 as libc::c_int, (*stroker).pen.num_vertices);
        }
        stop = _cairo_pen_find_active_cw_vertex_index(&mut (*stroker).pen, out_vector);
        if _cairo_slope_compare(
            &mut (*((*stroker).pen.vertices).offset(stop as isize)).slope_ccw,
            out_vector,
        ) > 0 as libc::c_int
        {
            stop = range_step(stop, -(1 as libc::c_int), (*stroker).pen.num_vertices);
            if _cairo_slope_compare(
                &mut (*((*stroker).pen.vertices).offset(stop as isize)).slope_cw,
                in_vector,
            ) < 0 as libc::c_int
            {
                return;
            }
        }
        npoints = stop - start;
    } else {
        step = -(1 as libc::c_int);
        start = _cairo_pen_find_active_ccw_vertex_index(&mut (*stroker).pen, in_vector);
        if _cairo_slope_compare(
            &mut (*((*stroker).pen.vertices).offset(start as isize)).slope_ccw,
            in_vector,
        ) < 0 as libc::c_int
        {
            start = range_step(start, -(1 as libc::c_int), (*stroker).pen.num_vertices);
        }
        stop = _cairo_pen_find_active_ccw_vertex_index(&mut (*stroker).pen, out_vector);
        if _cairo_slope_compare(
            &mut (*((*stroker).pen.vertices).offset(stop as isize)).slope_cw,
            out_vector,
        ) > 0 as libc::c_int
        {
            stop = range_step(stop, 1 as libc::c_int, (*stroker).pen.num_vertices);
            if _cairo_slope_compare(
                &mut (*((*stroker).pen.vertices).offset(stop as isize)).slope_ccw,
                in_vector,
            ) < 0 as libc::c_int
            {
                return;
            }
        }
        npoints = start - stop;
    }
    stop = range_step(stop, step, (*stroker).pen.num_vertices);
    if npoints < 0 as libc::c_int {
        npoints += (*stroker).pen.num_vertices;
    }
    if npoints <= 1 as libc::c_int {
        return;
    }
    i = start;
    while i != stop {
        let mut p: cairo_point_t = *midpt;
        translate_point(
            &mut p,
            &mut (*((*stroker).pen.vertices).offset(i as isize)).point,
        );
        i = range_step(i, step, (*stroker).pen.num_vertices);
    }
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
    if clockwise != 0 {
        outpt = &(*out).ccw;
    } else {
        outpt = &(*out).cw;
    };
}
unsafe extern "C" fn inner_close(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *mut cairo_stroke_face_t,
) {
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    if join_is_clockwise(in_0, out) != 0 {
        inpt = &mut (*out).ccw;
    } else {
        inpt = &mut (*out).cw;
    };
}
unsafe extern "C" fn outer_close(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
) {
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
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
    } else {
        inpt = &(*in_0).ccw;
        outpt = &(*out).ccw;
    }
    match (*stroker).style.line_join as libc::c_uint {
        1 => {
            add_fan(
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
                if slope_compare_sgn(fdx1, fdy1, mdx, mdy)
                    != slope_compare_sgn(fdx2, fdy2, mdx, mdy)
                {
                    let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
                    p.x = _cairo_fixed_from_double(mx);
                    p.y = _cairo_fixed_from_double(my);
                    return;
                }
            }
        }
    };
}
unsafe extern "C" fn outer_join(
    mut stroker: *mut stroker,
    mut in_0: *const cairo_stroke_face_t,
    mut out: *const cairo_stroke_face_t,
    mut clockwise: libc::c_int,
) {
    let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
    if (*in_0).cw.x == (*out).cw.x && (*in_0).cw.y == (*out).cw.y
        && (*in_0).ccw.x == (*out).ccw.x && (*in_0).ccw.y == (*out).ccw.y
    {
        return;
    }
    if clockwise != 0 {
        inpt = &(*in_0).cw;
        outpt = &(*out).cw;
    } else {
        inpt = &(*in_0).ccw;
        outpt = &(*out).ccw;
    }
    match (*stroker).style.line_join as libc::c_uint {
        1 => {
            add_fan(
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
                if slope_compare_sgn(fdx1, fdy1, mdx, mdy)
                    != slope_compare_sgn(fdx2, fdy2, mdx, mdy)
                {
                    let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
                    p.x = _cairo_fixed_from_double(mx);
                    p.y = _cairo_fixed_from_double(my);
                    return;
                }
            }
        }
    };
}
unsafe extern "C" fn add_cap(
    mut stroker: *mut stroker,
    mut f: *const cairo_stroke_face_t,
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
                &(*f).ccw,
                &(*f).cw,
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
            dx *= (*stroker).style.line_width / 2.0f64;
            dy *= (*stroker).style.line_width / 2.0f64;
            cairo_matrix_transform_distance((*stroker).ctm, &mut dx, &mut dy);
            fvector.dx = _cairo_fixed_from_double(dx);
            fvector.dy = _cairo_fixed_from_double(dy);
            quad[0 as libc::c_int as usize] = (*f).ccw;
            quad[1 as libc::c_int as usize].x = (*f).ccw.x + fvector.dx;
            quad[1 as libc::c_int as usize].y = (*f).ccw.y + fvector.dy;
            quad[2 as libc::c_int as usize].x = (*f).cw.x + fvector.dx;
            quad[2 as libc::c_int as usize].y = (*f).cw.y + fvector.dy;
            quad[3 as libc::c_int as usize] = (*f).cw;
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn add_leading_cap(
    mut stroker: *mut stroker,
    mut face: *const cairo_stroke_face_t,
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
    add_cap(stroker, &mut reversed);
}
unsafe extern "C" fn add_trailing_cap(
    mut stroker: *mut stroker,
    mut face: *const cairo_stroke_face_t,
) {
    add_cap(stroker, face);
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
            b"../src/cairo-path-stroke-tristrip.c\0" as *const u8 as *const libc::c_char,
            683 as libc::c_int as libc::c_uint,
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
            face_dx = -slope_dy * ((*stroker).style.line_width / 2.0f64);
            face_dy = slope_dx * ((*stroker).style.line_width / 2.0f64);
        } else {
            face_dx = slope_dy * ((*stroker).style.line_width / 2.0f64);
            face_dy = -slope_dx * ((*stroker).style.line_width / 2.0f64);
        }
        cairo_matrix_transform_distance((*stroker).ctm, &mut face_dx, &mut face_dy);
    } else {
        face_dx = -slope_dy * ((*stroker).style.line_width / 2.0f64);
        face_dy = slope_dx * ((*stroker).style.line_width / 2.0f64);
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
    if (*stroker).has_sub_path != 0 && (*stroker).has_first_face == 0
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
        add_leading_cap(stroker, &mut face);
        add_trailing_cap(stroker, &mut face);
    } else {
        if (*stroker).has_current_face != 0 {
            add_trailing_cap(stroker, &mut (*stroker).current_face);
        }
        if (*stroker).has_first_face != 0 {
            add_leading_cap(stroker, &mut (*stroker).first_face);
        }
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
    (*stroker).has_sub_path = 0 as libc::c_int;
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
    (*stroker).has_sub_path = 1 as libc::c_int;
    if (*p1).x == (*point).x && (*p1).y == (*point).y {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_slope_init(&mut dev_slope, p1, point);
    compute_face(p1, &mut dev_slope, stroker, &mut start);
    if (*stroker).has_current_face != 0 {
        let mut clockwise: libc::c_int = join_is_clockwise(
            &mut (*stroker).current_face,
            &mut start,
        );
        outer_join(stroker, &mut (*stroker).current_face, &mut start, clockwise);
        inner_join(stroker, &mut (*stroker).current_face, &mut start, clockwise);
    } else {
        if (*stroker).has_first_face == 0 {
            (*stroker).first_face = start;
            _cairo_tristrip_move_to((*stroker).strip, &mut start.cw);
            (*stroker).has_first_face = 1 as libc::c_int;
        }
        (*stroker).has_current_face = 1 as libc::c_int;
        _cairo_tristrip_add_point((*stroker).strip, &mut start.cw);
        _cairo_tristrip_add_point((*stroker).strip, &mut start.ccw);
    }
    (*stroker).current_face = start;
    (*stroker).current_face.point = *point;
    let ref mut fresh2 = (*stroker).current_face.ccw.x;
    *fresh2 += dev_slope.dx;
    let ref mut fresh3 = (*stroker).current_face.ccw.y;
    *fresh3 += dev_slope.dy;
    let ref mut fresh4 = (*stroker).current_face.cw.x;
    *fresh4 += dev_slope.dx;
    let ref mut fresh5 = (*stroker).current_face.cw.y;
    *fresh5 += dev_slope.dy;
    _cairo_tristrip_add_point((*stroker).strip, &mut (*stroker).current_face.cw);
    _cairo_tristrip_add_point((*stroker).strip, &mut (*stroker).current_face.ccw);
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
    if (*tangent).dx == 0 as libc::c_int && (*tangent).dy == 0 as libc::c_int {
        let mut inpt: *const cairo_point_t = 0 as *const cairo_point_t;
        let mut outpt: *const cairo_point_t = 0 as *const cairo_point_t;
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
            inpt = &mut (*stroker).current_face.cw;
            outpt = &mut face.cw;
        } else {
            inpt = &mut (*stroker).current_face.ccw;
            outpt = &mut face.ccw;
        }
        add_fan(
            stroker,
            &mut (*stroker).current_face.dev_vector,
            &mut face.dev_vector,
            &mut (*stroker).current_face.point,
            inpt,
            outpt,
            clockwise,
        );
    } else {
        compute_face(point, tangent, stroker, &mut face);
        if face.dev_slope.x * (*stroker).current_face.dev_slope.x
            + face.dev_slope.y * (*stroker).current_face.dev_slope.y
            < 0 as libc::c_int as libc::c_double
        {
            let mut inpt_0: *const cairo_point_t = 0 as *const cairo_point_t;
            let mut outpt_0: *const cairo_point_t = 0 as *const cairo_point_t;
            let mut clockwise_0: libc::c_int = join_is_clockwise(
                &mut (*stroker).current_face,
                &mut face,
            );
            let ref mut fresh6 = (*stroker).current_face.cw.x;
            *fresh6 += face.point.x - (*stroker).current_face.point.x;
            let ref mut fresh7 = (*stroker).current_face.cw.y;
            *fresh7 += face.point.y - (*stroker).current_face.point.y;
            let ref mut fresh8 = (*stroker).current_face.ccw.x;
            *fresh8 += face.point.x - (*stroker).current_face.point.x;
            let ref mut fresh9 = (*stroker).current_face.ccw.y;
            *fresh9 += face.point.y - (*stroker).current_face.point.y;
            if clockwise_0 != 0 {
                inpt_0 = &mut (*stroker).current_face.cw;
                outpt_0 = &mut face.cw;
            } else {
                inpt_0 = &mut (*stroker).current_face.ccw;
                outpt_0 = &mut face.ccw;
            }
            add_fan(
                stroker,
                &mut (*stroker).current_face.dev_vector,
                &mut face.dev_vector,
                &mut (*stroker).current_face.point,
                inpt_0,
                outpt_0,
                clockwise_0,
            );
        }
        _cairo_tristrip_add_point((*stroker).strip, &mut face.cw);
        _cairo_tristrip_add_point((*stroker).strip, &mut face.ccw);
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
    if (*stroker).has_limits != 0 {
        if _cairo_spline_intersects(
            &mut (*stroker).current_face.point,
            b,
            c,
            d,
            &mut (*stroker).limit,
        ) == 0
        {
            return line_to(closure, d);
        }
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
            _cairo_tristrip_move_to((*stroker).strip, &mut face.cw);
            (*stroker).has_first_face = 1 as libc::c_int;
        }
        (*stroker).has_current_face = 1 as libc::c_int;
        _cairo_tristrip_add_point((*stroker).strip, &mut face.cw);
        _cairo_tristrip_add_point((*stroker).strip, &mut face.ccw);
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
    } else {
        add_caps(stroker);
    }
    (*stroker).has_sub_path = 0 as libc::c_int;
    (*stroker).has_first_face = 0 as libc::c_int;
    (*stroker).has_current_face = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_to_tristrip(
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut strip: *mut cairo_tristrip_t,
) -> cairo_int_status_t {
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
        strip: 0 as *mut cairo_tristrip_t,
        ctm: 0 as *const cairo_matrix_t,
        ctm_inverse: 0 as *const cairo_matrix_t,
        tolerance: 0.,
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
        has_sub_path: 0,
        first_point: cairo_point_t { x: 0, y: 0 },
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
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        has_limits: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if (*style).num_dashes != 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    stroker.style = *style;
    stroker.ctm = ctm;
    stroker.ctm_inverse = ctm_inverse;
    stroker.tolerance = tolerance;
    stroker
        .ctm_det_positive = (_cairo_matrix_compute_determinant(ctm) >= 0.0f64)
        as libc::c_int;
    status = _cairo_pen_init(
        &mut stroker.pen,
        (*style).line_width / 2.0f64,
        tolerance,
        ctm,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if stroker.pen.num_vertices <= 1 as libc::c_int {
        return CAIRO_INT_STATUS_NOTHING_TO_DO;
    }
    stroker.has_current_face = 0 as libc::c_int;
    stroker.has_first_face = 0 as libc::c_int;
    stroker.has_sub_path = 0 as libc::c_int;
    stroker.has_limits = ((*strip).num_limits > 0 as libc::c_int) as libc::c_int;
    stroker.limit = *((*strip).limits).offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < (*strip).num_limits {
        _cairo_box_add_box(&mut stroker.limit, &*((*strip).limits).offset(i as isize));
        i += 1;
    }
    stroker.strip = strip;
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
    ) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        add_caps(&mut stroker);
    }
    _cairo_pen_fini(&mut stroker.pen);
    return status;
}
