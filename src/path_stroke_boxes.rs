use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn _cairo_path_fixed_is_stroke_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_clear(boxes: *mut cairo_boxes_t);
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
    fn _cairo_bentley_ottmann_tessellate_boxes(
        in_0: *const cairo_boxes_t,
        fill_rule: cairo_fill_rule_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
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
pub type cairo_antialias_t = _cairo_antialias;
pub type _cairo_antialias = libc::c_uint;
pub const CAIRO_ANTIALIAS_BEST: _cairo_antialias = 6;
pub const CAIRO_ANTIALIAS_GOOD: _cairo_antialias = 5;
pub const CAIRO_ANTIALIAS_FAST: _cairo_antialias = 4;
pub const CAIRO_ANTIALIAS_SUBPIXEL: _cairo_antialias = 3;
pub const CAIRO_ANTIALIAS_GRAY: _cairo_antialias = 2;
pub const CAIRO_ANTIALIAS_NONE: _cairo_antialias = 1;
pub const CAIRO_ANTIALIAS_DEFAULT: _cairo_antialias = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_t {
    pub status: cairo_status_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_boxes: libc::c_int,
    pub is_pixel_aligned: libc::c_uint,
    pub chunks: _cairo_boxes_chunk,
    pub tail: *mut _cairo_boxes_chunk,
    pub boxes_embedded: [cairo_box_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_chunk {
    pub next: *mut _cairo_boxes_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_boxes_t = _cairo_boxes_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
pub type cairo_line_t = _cairo_line;
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
pub type cairo_rectilinear_stroker_t = _cairo_rectilinear_stroker;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectilinear_stroker {
    pub stroke_style: *const cairo_stroke_style_t,
    pub ctm: *const cairo_matrix_t,
    pub antialias: cairo_antialias_t,
    pub half_line_x: cairo_fixed_t,
    pub half_line_y: cairo_fixed_t,
    pub boxes: *mut cairo_boxes_t,
    pub current_point: cairo_point_t,
    pub first_point: cairo_point_t,
    pub open_sub_path: cairo_bool_t,
    pub dash: cairo_stroker_dash_t,
    pub has_bounds: cairo_bool_t,
    pub bounds: cairo_box_t,
    pub num_segments: libc::c_int,
    pub segments_size: libc::c_int,
    pub segments: *mut segment_t,
    pub segments_embedded: [segment_t; 8],
}
pub type segment_t = _segment_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _segment_t {
    pub p1: cairo_point_t,
    pub p2: cairo_point_t,
    pub flags: libc::c_uint,
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
unsafe extern "C" fn _cairo_path_fixed_stroke_is_rectilinear(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).stroke_is_rectilinear() as cairo_bool_t;
}
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
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
unsafe extern "C" fn _cairo_matrix_is_scale(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64) as libc::c_int;
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
unsafe extern "C" fn _cairo_rectilinear_stroker_limit(
    mut stroker: *mut cairo_rectilinear_stroker_t,
    mut boxes: *const cairo_box_t,
    mut num_boxes: libc::c_int,
) {
    (*stroker).has_bounds = 1 as libc::c_int;
    _cairo_boxes_get_extents(boxes, num_boxes, &mut (*stroker).bounds);
    let ref mut fresh4 = (*stroker).bounds.p1.x;
    *fresh4 -= (*stroker).half_line_x;
    let ref mut fresh5 = (*stroker).bounds.p2.x;
    *fresh5 += (*stroker).half_line_x;
    let ref mut fresh6 = (*stroker).bounds.p1.y;
    *fresh6 -= (*stroker).half_line_y;
    let ref mut fresh7 = (*stroker).bounds.p2.y;
    *fresh7 += (*stroker).half_line_y;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_init(
    mut stroker: *mut cairo_rectilinear_stroker_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut antialias: cairo_antialias_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_bool_t {
    if (*stroke_style).line_join as libc::c_uint
        != CAIRO_LINE_JOIN_MITER as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*stroke_style).miter_limit < 1.41421356237309504880f64 {
        return 0 as libc::c_int;
    }
    if !((*stroke_style).line_cap as libc::c_uint
        == CAIRO_LINE_CAP_BUTT as libc::c_int as libc::c_uint
        || (*stroke_style).line_cap as libc::c_uint
            == CAIRO_LINE_CAP_SQUARE as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    if _cairo_matrix_is_scale(ctm) == 0 {
        return 0 as libc::c_int;
    }
    let ref mut fresh8 = (*stroker).stroke_style;
    *fresh8 = stroke_style;
    let ref mut fresh9 = (*stroker).ctm;
    *fresh9 = ctm;
    (*stroker).antialias = antialias;
    (*stroker)
        .half_line_x = _cairo_fixed_from_double(
        fabs((*ctm).xx) * (*stroke_style).line_width / 2.0f64,
    );
    (*stroker)
        .half_line_y = _cairo_fixed_from_double(
        fabs((*ctm).yy) * (*stroke_style).line_width / 2.0f64,
    );
    (*stroker).open_sub_path = 0 as libc::c_int;
    let ref mut fresh10 = (*stroker).segments;
    *fresh10 = ((*stroker).segments_embedded).as_mut_ptr();
    (*stroker)
        .segments_size = (::std::mem::size_of::<[segment_t; 8]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<segment_t>() as libc::c_ulong)
        as libc::c_int;
    (*stroker).num_segments = 0 as libc::c_int;
    _cairo_stroker_dash_init(&mut (*stroker).dash, stroke_style);
    (*stroker).has_bounds = 0 as libc::c_int;
    let ref mut fresh11 = (*stroker).boxes;
    *fresh11 = boxes;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_fini(
    mut stroker: *mut cairo_rectilinear_stroker_t,
) {
    if (*stroker).segments != ((*stroker).segments_embedded).as_mut_ptr() {
        free((*stroker).segments as *mut libc::c_void);
    }
}
unsafe extern "C" fn _cairo_rectilinear_stroker_add_segment(
    mut stroker: *mut cairo_rectilinear_stroker_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    if (*stroker).num_segments == (*stroker).segments_size {
        let mut new_size: libc::c_int = (*stroker).segments_size * 2 as libc::c_int;
        let mut new_segments: *mut segment_t = 0 as *mut segment_t;
        if (*stroker).segments == ((*stroker).segments_embedded).as_mut_ptr() {
            new_segments = _cairo_malloc_ab(
                new_size as size_t,
                ::std::mem::size_of::<segment_t>() as libc::c_ulong,
            ) as *mut segment_t;
            if new_segments.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            memcpy(
                new_segments as *mut libc::c_void,
                (*stroker).segments as *const libc::c_void,
                ((*stroker).num_segments as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<segment_t>() as libc::c_ulong),
            );
        } else {
            new_segments = _cairo_realloc_ab(
                (*stroker).segments as *mut libc::c_void,
                new_size as size_t,
                ::std::mem::size_of::<segment_t>() as libc::c_ulong,
            ) as *mut segment_t;
            if new_segments.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
        (*stroker).segments_size = new_size;
        let ref mut fresh12 = (*stroker).segments;
        *fresh12 = new_segments;
    }
    (*((*stroker).segments).offset((*stroker).num_segments as isize)).p1 = *p1;
    (*((*stroker).segments).offset((*stroker).num_segments as isize)).p2 = *p2;
    (*((*stroker).segments).offset((*stroker).num_segments as isize)).flags = flags;
    let ref mut fresh13 = (*stroker).num_segments;
    *fresh13 += 1;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_emit_segments(
    mut stroker: *mut cairo_rectilinear_stroker_t,
) -> cairo_status_t {
    let mut line_cap: cairo_line_cap_t = (*(*stroker).stroke_style).line_cap;
    let mut half_line_x: cairo_fixed_t = (*stroker).half_line_x;
    let mut half_line_y: cairo_fixed_t = (*stroker).half_line_y;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*stroker).num_segments {
        let mut lengthen_initial: cairo_bool_t = 0;
        let mut lengthen_final: cairo_bool_t = 0;
        let mut a: *mut cairo_point_t = 0 as *mut cairo_point_t;
        let mut b: *mut cairo_point_t = 0 as *mut cairo_point_t;
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        a = &mut (*((*stroker).segments).offset(i as isize)).p1;
        b = &mut (*((*stroker).segments).offset(i as isize)).p2;
        j = if i == 0 as libc::c_int {
            (*stroker).num_segments - 1 as libc::c_int
        } else {
            i - 1 as libc::c_int
        };
        lengthen_initial = (((*((*stroker).segments).offset(i as isize)).flags
            ^ (*((*stroker).segments).offset(j as isize)).flags)
            & 0x1 as libc::c_int as libc::c_uint) as cairo_bool_t;
        j = if i == (*stroker).num_segments - 1 as libc::c_int {
            0 as libc::c_int
        } else {
            i + 1 as libc::c_int
        };
        lengthen_final = (((*((*stroker).segments).offset(i as isize)).flags
            ^ (*((*stroker).segments).offset(j as isize)).flags)
            & 0x1 as libc::c_int as libc::c_uint) as cairo_bool_t;
        if (*stroker).open_sub_path != 0 {
            if i == 0 as libc::c_int {
                lengthen_initial = (line_cap as libc::c_uint
                    != CAIRO_LINE_CAP_BUTT as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
            if i == (*stroker).num_segments - 1 as libc::c_int {
                lengthen_final = (line_cap as libc::c_uint
                    != CAIRO_LINE_CAP_BUTT as libc::c_int as libc::c_uint)
                    as libc::c_int;
            }
        }
        if lengthen_initial | lengthen_final != 0 {
            if (*a).y == (*b).y {
                if (*a).x < (*b).x {
                    if lengthen_initial != 0 {
                        let ref mut fresh14 = (*a).x;
                        *fresh14 -= half_line_x;
                    }
                    if lengthen_final != 0 {
                        let ref mut fresh15 = (*b).x;
                        *fresh15 += half_line_x;
                    }
                } else {
                    if lengthen_initial != 0 {
                        let ref mut fresh16 = (*a).x;
                        *fresh16 += half_line_x;
                    }
                    if lengthen_final != 0 {
                        let ref mut fresh17 = (*b).x;
                        *fresh17 -= half_line_x;
                    }
                }
            } else if (*a).y < (*b).y {
                if lengthen_initial != 0 {
                    let ref mut fresh18 = (*a).y;
                    *fresh18 -= half_line_y;
                }
                if lengthen_final != 0 {
                    let ref mut fresh19 = (*b).y;
                    *fresh19 += half_line_y;
                }
            } else {
                if lengthen_initial != 0 {
                    let ref mut fresh20 = (*a).y;
                    *fresh20 += half_line_y;
                }
                if lengthen_final != 0 {
                    let ref mut fresh21 = (*b).y;
                    *fresh21 -= half_line_y;
                }
            }
        }
        if (*a).y == (*b).y {
            let ref mut fresh22 = (*a).y;
            *fresh22 -= half_line_y;
            let ref mut fresh23 = (*b).y;
            *fresh23 += half_line_y;
        } else {
            let ref mut fresh24 = (*a).x;
            *fresh24 -= half_line_x;
            let ref mut fresh25 = (*b).x;
            *fresh25 += half_line_x;
        }
        if (*a).x < (*b).x {
            box_0.p1.x = (*a).x;
            box_0.p2.x = (*b).x;
        } else {
            box_0.p1.x = (*b).x;
            box_0.p2.x = (*a).x;
        }
        if (*a).y < (*b).y {
            box_0.p1.y = (*a).y;
            box_0.p2.y = (*b).y;
        } else {
            box_0.p1.y = (*b).y;
            box_0.p2.y = (*a).y;
        }
        status = _cairo_boxes_add((*stroker).boxes, (*stroker).antialias, &mut box_0);
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    (*stroker).num_segments = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_emit_segments_dashed(
    mut stroker: *mut cairo_rectilinear_stroker_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut line_cap: cairo_line_cap_t = (*(*stroker).stroke_style).line_cap;
    let mut half_line_x: cairo_fixed_t = (*stroker).half_line_x;
    let mut half_line_y: cairo_fixed_t = (*stroker).half_line_y;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*stroker).num_segments {
        let mut a: *mut cairo_point_t = 0 as *mut cairo_point_t;
        let mut b: *mut cairo_point_t = 0 as *mut cairo_point_t;
        let mut is_horizontal: cairo_bool_t = 0;
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        a = &mut (*((*stroker).segments).offset(i as isize)).p1;
        b = &mut (*((*stroker).segments).offset(i as isize)).p2;
        is_horizontal = ((*((*stroker).segments).offset(i as isize)).flags
            & 0x1 as libc::c_int as libc::c_uint) as cairo_bool_t;
        if line_cap as libc::c_uint == CAIRO_LINE_CAP_BUTT as libc::c_int as libc::c_uint
            && (*((*stroker).segments).offset(i as isize)).flags
                & 0x4 as libc::c_int as libc::c_uint != 0
            && (i != (*stroker).num_segments - 1 as libc::c_int
                || (*stroker).open_sub_path == 0 && (*stroker).dash.dash_starts_on != 0)
        {
            let mut out_slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            let mut j: libc::c_int = (i + 1 as libc::c_int) % (*stroker).num_segments;
            let mut forwards: cairo_bool_t = ((*((*stroker).segments).offset(i as isize))
                .flags & 0x2 as libc::c_int as libc::c_uint != 0) as libc::c_int;
            _cairo_slope_init(
                &mut out_slope,
                &mut (*((*stroker).segments).offset(j as isize)).p1,
                &mut (*((*stroker).segments).offset(j as isize)).p2,
            );
            box_0.p1 = (*((*stroker).segments).offset(i as isize)).p2;
            box_0.p2 = box_0.p1;
            if is_horizontal != 0 {
                if forwards != 0 {
                    box_0.p2.x += half_line_x;
                } else {
                    box_0.p1.x -= half_line_x;
                }
                if out_slope.dy > 0 as libc::c_int {
                    box_0.p1.y -= half_line_y;
                } else {
                    box_0.p2.y += half_line_y;
                }
            } else {
                if forwards != 0 {
                    box_0.p2.y += half_line_y;
                } else {
                    box_0.p1.y -= half_line_y;
                }
                if out_slope.dx > 0 as libc::c_int {
                    box_0.p1.x -= half_line_x;
                } else {
                    box_0.p2.x += half_line_x;
                }
            }
            status = _cairo_boxes_add(
                (*stroker).boxes,
                (*stroker).antialias,
                &mut box_0,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        if is_horizontal != 0 {
            if line_cap as libc::c_uint
                == CAIRO_LINE_CAP_SQUARE as libc::c_int as libc::c_uint
            {
                if (*a).x <= (*b).x {
                    let ref mut fresh26 = (*a).x;
                    *fresh26 -= half_line_x;
                    let ref mut fresh27 = (*b).x;
                    *fresh27 += half_line_x;
                } else {
                    let ref mut fresh28 = (*a).x;
                    *fresh28 += half_line_x;
                    let ref mut fresh29 = (*b).x;
                    *fresh29 -= half_line_x;
                }
            }
            let ref mut fresh30 = (*a).y;
            *fresh30 += half_line_y;
            let ref mut fresh31 = (*b).y;
            *fresh31 -= half_line_y;
        } else {
            if line_cap as libc::c_uint
                == CAIRO_LINE_CAP_SQUARE as libc::c_int as libc::c_uint
            {
                if (*a).y <= (*b).y {
                    let ref mut fresh32 = (*a).y;
                    *fresh32 -= half_line_y;
                    let ref mut fresh33 = (*b).y;
                    *fresh33 += half_line_y;
                } else {
                    let ref mut fresh34 = (*a).y;
                    *fresh34 += half_line_y;
                    let ref mut fresh35 = (*b).y;
                    *fresh35 -= half_line_y;
                }
            }
            let ref mut fresh36 = (*a).x;
            *fresh36 += half_line_x;
            let ref mut fresh37 = (*b).x;
            *fresh37 -= half_line_x;
        }
        if !((*a).x == (*b).x && (*a).y == (*b).y) {
            if (*a).x < (*b).x {
                box_0.p1.x = (*a).x;
                box_0.p2.x = (*b).x;
            } else {
                box_0.p1.x = (*b).x;
                box_0.p2.x = (*a).x;
            }
            if (*a).y < (*b).y {
                box_0.p1.y = (*a).y;
                box_0.p2.y = (*b).y;
            } else {
                box_0.p1.y = (*b).y;
                box_0.p2.y = (*a).y;
            }
            status = _cairo_boxes_add(
                (*stroker).boxes,
                (*stroker).antialias,
                &mut box_0,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        i += 1;
    }
    (*stroker).num_segments = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_rectilinear_stroker_t = closure
        as *mut cairo_rectilinear_stroker_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stroker).dash.dashed != 0 {
        status = _cairo_rectilinear_stroker_emit_segments_dashed(stroker);
    } else {
        status = _cairo_rectilinear_stroker_emit_segments(stroker);
    }
    if status as u64 != 0 {
        return status;
    }
    _cairo_stroker_dash_start(&mut (*stroker).dash);
    (*stroker).current_point = *point;
    (*stroker).first_point = *point;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_line_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_rectilinear_stroker_t = closure
        as *mut cairo_rectilinear_stroker_t;
    let mut a: *mut cairo_point_t = &mut (*stroker).current_point;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*a).x == (*b).x || (*a).y == (*b).y {} else {
        __assert_fail(
            b"a->x == b->x || a->y == b->y\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-stroke-boxes.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"cairo_status_t _cairo_rectilinear_stroker_line_to(void *, const cairo_point_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*a).x == (*b).x && (*a).y == (*b).y {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_rectilinear_stroker_add_segment(
        stroker,
        a,
        b,
        (((*a).y == (*b).y) as libc::c_int | 0x4 as libc::c_int) as libc::c_uint,
    );
    (*stroker).current_point = *b;
    (*stroker).open_sub_path = 1 as libc::c_int;
    return status;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_line_to_dashed(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut stroker: *mut cairo_rectilinear_stroker_t = closure
        as *mut cairo_rectilinear_stroker_t;
    let mut a: *const cairo_point_t = &mut (*stroker).current_point;
    let mut b: *const cairo_point_t = point;
    let mut fully_in_bounds: cairo_bool_t = 0;
    let mut sf: libc::c_double = 0.;
    let mut sign: libc::c_double = 0.;
    let mut remain: libc::c_double = 0.;
    let mut mag: cairo_fixed_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut segment: cairo_line_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut dash_on: cairo_bool_t = 0 as libc::c_int;
    let mut is_horizontal: libc::c_uint = 0;
    if (*a).x == (*b).x && (*a).y == (*b).y {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*a).x == (*b).x || (*a).y == (*b).y {} else {
        __assert_fail(
            b"a->x == b->x || a->y == b->y\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-stroke-boxes.c\0" as *const u8 as *const libc::c_char,
            492 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"cairo_status_t _cairo_rectilinear_stroker_line_to_dashed(void *, const cairo_point_t *)\0",
            ))
                .as_ptr(),
        );
    }
    fully_in_bounds = 1 as libc::c_int;
    if (*stroker).has_bounds != 0
        && (_cairo_box_contains_point(&mut (*stroker).bounds, a) == 0
            || _cairo_box_contains_point(&mut (*stroker).bounds, b) == 0)
    {
        fully_in_bounds = 0 as libc::c_int;
    }
    is_horizontal = ((*a).y == (*b).y) as libc::c_int as libc::c_uint;
    if is_horizontal != 0 {
        mag = (*b).x - (*a).x;
        sf = fabs((*(*stroker).ctm).xx);
    } else {
        mag = (*b).y - (*a).y;
        sf = fabs((*(*stroker).ctm).yy);
    }
    if mag < 0 as libc::c_int {
        remain = _cairo_fixed_to_double(-mag);
        sign = 1.0f64;
    } else {
        remain = _cairo_fixed_to_double(mag);
        is_horizontal |= 0x2 as libc::c_int as libc::c_uint;
        sign = -1.0f64;
    }
    segment.p1 = *a;
    segment.p2 = segment.p1;
    while remain > 0.0f64 {
        let mut step_length: libc::c_double = 0.;
        step_length = if sf * (*stroker).dash.dash_remain < remain {
            sf * (*stroker).dash.dash_remain
        } else {
            remain
        };
        remain -= step_length;
        mag = _cairo_fixed_from_double(sign * remain);
        if is_horizontal & 0x1 as libc::c_int as libc::c_uint != 0 {
            segment.p2.x = (*b).x + mag;
        } else {
            segment.p2.y = (*b).y + mag;
        }
        if (*stroker).dash.dash_on != 0
            && (fully_in_bounds != 0
                || _cairo_box_intersects_line_segment(
                    &mut (*stroker).bounds,
                    &mut segment,
                ) != 0)
        {
            status = _cairo_rectilinear_stroker_add_segment(
                stroker,
                &mut segment.p1,
                &mut segment.p2,
                is_horizontal
                    | (((remain <= 0.0f64) as libc::c_int) << 2 as libc::c_int)
                        as libc::c_uint,
            );
            if status as u64 != 0 {
                return status;
            }
            dash_on = 1 as libc::c_int;
        } else {
            dash_on = 0 as libc::c_int;
        }
        _cairo_stroker_dash_step(&mut (*stroker).dash, step_length / sf);
        segment.p1 = segment.p2;
    }
    if (*stroker).dash.dash_on != 0 && dash_on == 0
        && (fully_in_bounds != 0
            || _cairo_box_intersects_line_segment(&mut (*stroker).bounds, &mut segment)
                != 0)
    {
        status = _cairo_rectilinear_stroker_add_segment(
            stroker,
            &mut segment.p1,
            &mut segment.p1,
            is_horizontal | 0x4 as libc::c_int as libc::c_uint,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    (*stroker).current_point = *point;
    (*stroker).open_sub_path = 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectilinear_stroker_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut stroker: *mut cairo_rectilinear_stroker_t = closure
        as *mut cairo_rectilinear_stroker_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stroker).open_sub_path == 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*stroker).dash.dashed != 0 {
        status = _cairo_rectilinear_stroker_line_to_dashed(
            stroker as *mut libc::c_void,
            &mut (*stroker).first_point,
        );
    } else {
        status = _cairo_rectilinear_stroker_line_to(
            stroker as *mut libc::c_void,
            &mut (*stroker).first_point,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    (*stroker).open_sub_path = 0 as libc::c_int;
    if (*stroker).dash.dashed != 0 {
        status = _cairo_rectilinear_stroker_emit_segments_dashed(stroker);
    } else {
        status = _cairo_rectilinear_stroker_emit_segments(stroker);
    }
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_stroke_rectilinear_to_boxes(
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut antialias: cairo_antialias_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut rectilinear_stroker: cairo_rectilinear_stroker_t = cairo_rectilinear_stroker_t {
        stroke_style: 0 as *const cairo_stroke_style_t,
        ctm: 0 as *const cairo_matrix_t,
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        half_line_x: 0,
        half_line_y: 0,
        boxes: 0 as *mut cairo_boxes_t,
        current_point: cairo_point_t { x: 0, y: 0 },
        first_point: cairo_point_t { x: 0, y: 0 },
        open_sub_path: 0,
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
        bounds: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        num_segments: 0,
        segments_size: 0,
        segments: 0 as *mut segment_t,
        segments_embedded: [segment_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
            flags: 0,
        }; 8],
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if _cairo_path_fixed_stroke_is_rectilinear(path) != 0 {} else {
        __assert_fail(
            b"_cairo_path_fixed_stroke_is_rectilinear (path)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-path-stroke-boxes.c\0" as *const u8 as *const libc::c_char,
            620 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 183],
                &[libc::c_char; 183],
            >(
                b"cairo_int_status_t _cairo_path_fixed_stroke_rectilinear_to_boxes(const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, cairo_antialias_t, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_rectilinear_stroker_init(
        &mut rectilinear_stroker,
        stroke_style,
        ctm,
        antialias,
        boxes,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if rectilinear_stroker.dash.dashed == 0
        && _cairo_path_fixed_is_stroke_box(path, &mut box_0) != 0
        && box_0.p2.x - box_0.p1.x > 2 as libc::c_int * rectilinear_stroker.half_line_x
        && box_0.p2.y - box_0.p1.y > 2 as libc::c_int * rectilinear_stroker.half_line_y
    {
        let mut b: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        b.p1.x = box_0.p1.x - rectilinear_stroker.half_line_x;
        b.p2.x = box_0.p2.x + rectilinear_stroker.half_line_x;
        b.p1.y = box_0.p1.y - rectilinear_stroker.half_line_y;
        b.p2.y = box_0.p1.y + rectilinear_stroker.half_line_y;
        status = _cairo_boxes_add(boxes, antialias, &mut b) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-path-stroke-boxes.c\0" as *const u8
                    as *const libc::c_char,
                643 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 183],
                    &[libc::c_char; 183],
                >(
                    b"cairo_int_status_t _cairo_path_fixed_stroke_rectilinear_to_boxes(const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, cairo_antialias_t, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        b.p1.x = box_0.p1.x - rectilinear_stroker.half_line_x;
        b.p2.x = box_0.p1.x + rectilinear_stroker.half_line_x;
        b.p1.y = box_0.p1.y + rectilinear_stroker.half_line_y;
        b.p2.y = box_0.p2.y - rectilinear_stroker.half_line_y;
        status = _cairo_boxes_add(boxes, antialias, &mut b) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-path-stroke-boxes.c\0" as *const u8
                    as *const libc::c_char,
                651 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 183],
                    &[libc::c_char; 183],
                >(
                    b"cairo_int_status_t _cairo_path_fixed_stroke_rectilinear_to_boxes(const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, cairo_antialias_t, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        b.p1.x = box_0.p2.x - rectilinear_stroker.half_line_x;
        b.p2.x = box_0.p2.x + rectilinear_stroker.half_line_x;
        b.p1.y = box_0.p1.y + rectilinear_stroker.half_line_y;
        b.p2.y = box_0.p2.y - rectilinear_stroker.half_line_y;
        status = _cairo_boxes_add(boxes, antialias, &mut b) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-path-stroke-boxes.c\0" as *const u8
                    as *const libc::c_char,
                659 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 183],
                    &[libc::c_char; 183],
                >(
                    b"cairo_int_status_t _cairo_path_fixed_stroke_rectilinear_to_boxes(const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, cairo_antialias_t, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        b.p1.x = box_0.p1.x - rectilinear_stroker.half_line_x;
        b.p2.x = box_0.p2.x + rectilinear_stroker.half_line_x;
        b.p1.y = box_0.p2.y - rectilinear_stroker.half_line_y;
        b.p2.y = box_0.p2.y + rectilinear_stroker.half_line_y;
        status = _cairo_boxes_add(boxes, antialias, &mut b) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-path-stroke-boxes.c\0" as *const u8
                    as *const libc::c_char,
                667 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 183],
                    &[libc::c_char; 183],
                >(
                    b"cairo_int_status_t _cairo_path_fixed_stroke_rectilinear_to_boxes(const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, cairo_antialias_t, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        if (*boxes).num_limits != 0 {
            _cairo_rectilinear_stroker_limit(
                &mut rectilinear_stroker,
                (*boxes).limits,
                (*boxes).num_limits,
            );
        }
        status = _cairo_path_fixed_interpret(
            path,
            Some(
                _cairo_rectilinear_stroker_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            if rectilinear_stroker.dash.dashed != 0 {
                Some(
                    _cairo_rectilinear_stroker_line_to_dashed
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const cairo_point_t,
                        ) -> cairo_status_t,
                )
            } else {
                Some(
                    _cairo_rectilinear_stroker_line_to
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const cairo_point_t,
                        ) -> cairo_status_t,
                )
            },
            None,
            Some(
                _cairo_rectilinear_stroker_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut rectilinear_stroker as *mut cairo_rectilinear_stroker_t
                as *mut libc::c_void,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            current_block = 6523494653378992949;
        } else {
            if rectilinear_stroker.dash.dashed != 0 {
                status = _cairo_rectilinear_stroker_emit_segments_dashed(
                    &mut rectilinear_stroker,
                ) as cairo_int_status_t;
            } else {
                status = _cairo_rectilinear_stroker_emit_segments(
                    &mut rectilinear_stroker,
                ) as cairo_int_status_t;
            }
            if status as u64 != 0 {
                current_block = 6523494653378992949;
            } else {
                status = _cairo_bentley_ottmann_tessellate_boxes(
                    boxes,
                    CAIRO_FILL_RULE_WINDING,
                    boxes,
                ) as cairo_int_status_t;
                if status as u64 != 0 {
                    current_block = 6523494653378992949;
                } else {
                    current_block = 15984154738040588190;
                }
            }
        }
        match current_block {
            15984154738040588190 => {}
            _ => {
                _cairo_rectilinear_stroker_fini(&mut rectilinear_stroker);
                _cairo_boxes_clear(boxes);
                return status;
            }
        }
    }
    _cairo_rectilinear_stroker_fini(&mut rectilinear_stroker);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
