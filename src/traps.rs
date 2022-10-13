use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cairo_region_create_rectangles(
        rects: *const cairo_rectangle_int_t,
        count: libc::c_int,
    ) -> *mut cairo_region_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_mono_scan_converter_create(
        xmin: libc::c_int,
        ymin: libc::c_int,
        xmax: libc::c_int,
        ymax: libc::c_int,
        fill_rule: cairo_fill_rule_t,
    ) -> *mut cairo_scan_converter_t;
    fn _cairo_mono_scan_converter_add_polygon(
        converter: *mut libc::c_void,
        polygon: *const cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_init(boxes: *mut cairo_boxes_t);
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn _cairo_path_fixed_move_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_line_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_close_path(path: *mut cairo_path_fixed_t) -> cairo_status_t;
    fn _cairo_lines_compare_at_y(
        a: *const cairo_line_t,
        b: *const cairo_line_t,
        y: libc::c_int,
    ) -> libc::c_int;
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_reference_count_t {
    pub ref_count: cairo_atomic_int_t,
}
pub type cairo_atomic_int_t = libc::c_int;
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
pub type cairo_clip_t = _cairo_clip;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip {
    pub extents: cairo_rectangle_int_t,
    pub path: *mut cairo_clip_path_t,
    pub boxes: *mut cairo_box_t,
    pub num_boxes: libc::c_int,
    pub region: *mut cairo_region_t,
    pub is_region: cairo_bool_t,
    pub embedded_box: cairo_box_t,
}
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
pub type cairo_region_t = _cairo_region;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_region {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub rgn: pixman_region32_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type cairo_clip_path_t = _cairo_clip_path;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip_path {
    pub ref_count: cairo_reference_count_t,
    pub path: cairo_path_fixed_t,
    pub fill_rule: cairo_fill_rule_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
    pub prev: *mut cairo_clip_path_t,
}
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
pub type uint32_t = __uint32_t;
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type cairo_int64_t = int64_t;
pub type cairo_fixed_unsigned_t = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_half_open_span {
    pub x: int32_t,
    pub coverage: uint8_t,
    pub inverse: uint8_t,
}
pub type cairo_half_open_span_t = _cairo_half_open_span;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_span_renderer {
    pub status: cairo_status_t,
    pub destroy: cairo_destroy_func_t,
    pub render_rows: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            *const cairo_half_open_span_t,
            libc::c_uint,
        ) -> cairo_status_t,
    >,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
}
pub type cairo_span_renderer_t = _cairo_span_renderer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scan_converter {
    pub destroy: cairo_destroy_func_t,
    pub generate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_span_renderer_t,
        ) -> cairo_status_t,
    >,
    pub status: cairo_status_t,
}
pub type cairo_scan_converter_t = _cairo_scan_converter;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_trap_renderer {
    pub base: cairo_span_renderer_t,
    pub traps: *mut cairo_traps_t,
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
unsafe extern "C" fn _cairo_fixed_mul(
    mut a: cairo_fixed_t,
    mut b: cairo_fixed_t,
) -> cairo_fixed_t {
    let mut temp: cairo_int64_t = a as int64_t * b as libc::c_long;
    return (temp as uint64_t >> 8 as libc::c_int) as int64_t as int32_t;
}
#[inline]
unsafe extern "C" fn _cairo_int64_32_div(
    mut num: cairo_int64_t,
    mut den: int32_t,
) -> int32_t {
    return (num / den as libc::c_long) as int32_t;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
}
#[inline]
unsafe extern "C" fn _cairo_fixed_round_down(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return _cairo_fixed_floor(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_round_down(
    mut f: cairo_fixed_t,
) -> libc::c_int {
    return _cairo_fixed_integer_part(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn _cairo_fixed_mul_div_floor(
    mut a: cairo_fixed_t,
    mut b: cairo_fixed_t,
    mut c: cairo_fixed_t,
) -> cairo_fixed_t {
    return _cairo_int64_32_div(a as int64_t * b as libc::c_long, c);
}
#[inline]
unsafe extern "C" fn _cairo_edge_compute_intersection_x_for_y(
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut y: cairo_fixed_t,
) -> cairo_fixed_t {
    let mut x: cairo_fixed_t = 0;
    let mut dy: cairo_fixed_t = 0;
    if y == (*p1).y {
        return (*p1).x;
    }
    if y == (*p2).y {
        return (*p2).x;
    }
    x = (*p1).x;
    dy = (*p2).y - (*p1).y;
    if dy != 0 as libc::c_int {
        x += _cairo_fixed_mul_div_floor(y - (*p1).y, (*p2).x - (*p1).x, dy);
    }
    return x;
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
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_init(mut traps: *mut cairo_traps_t) {
    (*traps).status = CAIRO_STATUS_SUCCESS;
    (*traps).set_maybe_region(1 as libc::c_int as libc::c_uint);
    (*traps).set_is_rectilinear(0 as libc::c_int as libc::c_uint);
    (*traps).set_is_rectangular(0 as libc::c_int as libc::c_uint);
    (*traps).num_traps = 0 as libc::c_int;
    (*traps)
        .traps_size = (::std::mem::size_of::<[cairo_trapezoid_t; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_trapezoid_t>() as libc::c_ulong)
        as libc::c_int;
    let ref mut fresh4 = (*traps).traps;
    *fresh4 = ((*traps).traps_embedded).as_mut_ptr();
    (*traps).num_limits = 0 as libc::c_int;
    (*traps).set_has_intersections(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_limit(
    mut traps: *mut cairo_traps_t,
    mut limits: *const cairo_box_t,
    mut num_limits: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let ref mut fresh5 = (*traps).limits;
    *fresh5 = limits;
    (*traps).num_limits = num_limits;
    (*traps).bounds = *limits.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < num_limits {
        _cairo_box_add_box(&mut (*traps).bounds, &*limits.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_init_with_clip(
    mut traps: *mut cairo_traps_t,
    mut clip: *const cairo_clip_t,
) {
    _cairo_traps_init(traps);
    if !clip.is_null() {
        _cairo_traps_limit(traps, (*clip).boxes, (*clip).num_boxes);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_clear(mut traps: *mut cairo_traps_t) {
    (*traps).status = CAIRO_STATUS_SUCCESS;
    (*traps).set_maybe_region(1 as libc::c_int as libc::c_uint);
    (*traps).set_is_rectilinear(0 as libc::c_int as libc::c_uint);
    (*traps).set_is_rectangular(0 as libc::c_int as libc::c_uint);
    (*traps).num_traps = 0 as libc::c_int;
    (*traps).set_has_intersections(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_fini(mut traps: *mut cairo_traps_t) {
    if (*traps).traps != ((*traps).traps_embedded).as_mut_ptr() {
        free((*traps).traps as *mut libc::c_void);
    }
}
unsafe extern "C" fn _cairo_traps_grow(mut traps: *mut cairo_traps_t) -> cairo_bool_t {
    let mut new_traps: *mut cairo_trapezoid_t = 0 as *mut cairo_trapezoid_t;
    let mut new_size: libc::c_int = 4 as libc::c_int * (*traps).traps_size;
    if (*traps).traps == ((*traps).traps_embedded).as_mut_ptr() {
        new_traps = _cairo_malloc_ab(
            new_size as size_t,
            ::std::mem::size_of::<cairo_trapezoid_t>() as libc::c_ulong,
        ) as *mut cairo_trapezoid_t;
        if !new_traps.is_null() {
            memcpy(
                new_traps as *mut libc::c_void,
                (*traps).traps as *const libc::c_void,
                ::std::mem::size_of::<[cairo_trapezoid_t; 16]>() as libc::c_ulong,
            );
        }
    } else {
        new_traps = _cairo_realloc_ab(
            (*traps).traps as *mut libc::c_void,
            new_size as size_t,
            ::std::mem::size_of::<cairo_trapezoid_t>() as libc::c_ulong,
        ) as *mut cairo_trapezoid_t;
    }
    if new_traps.is_null() {
        (*traps).status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as libc::c_int;
    }
    let ref mut fresh6 = (*traps).traps;
    *fresh6 = new_traps;
    (*traps).traps_size = new_size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_add_trap(
    mut traps: *mut cairo_traps_t,
    mut top: cairo_fixed_t,
    mut bottom: cairo_fixed_t,
    mut left: *const cairo_line_t,
    mut right: *const cairo_line_t,
) {
    let mut trap: *mut cairo_trapezoid_t = 0 as *mut cairo_trapezoid_t;
    if (*left).p1.y != (*left).p2.y {} else {
        __assert_fail(
            b"left->p1.y != left->p2.y\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void _cairo_traps_add_trap(cairo_traps_t *, cairo_fixed_t, cairo_fixed_t, const cairo_line_t *, const cairo_line_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*right).p1.y != (*right).p2.y {} else {
        __assert_fail(
            b"right->p1.y != right->p2.y\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void _cairo_traps_add_trap(cairo_traps_t *, cairo_fixed_t, cairo_fixed_t, const cairo_line_t *, const cairo_line_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if bottom > top {} else {
        __assert_fail(
            b"bottom > top\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void _cairo_traps_add_trap(cairo_traps_t *, cairo_fixed_t, cairo_fixed_t, const cairo_line_t *, const cairo_line_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*traps).num_traps == (*traps).traps_size {
        if _cairo_traps_grow(traps) == 0 {
            return;
        }
    }
    let ref mut fresh7 = (*traps).num_traps;
    let fresh8 = *fresh7;
    *fresh7 = *fresh7 + 1;
    trap = &mut *((*traps).traps).offset(fresh8 as isize) as *mut cairo_trapezoid_t;
    (*trap).top = top;
    (*trap).bottom = bottom;
    (*trap).left = *left;
    (*trap).right = *right;
}
unsafe extern "C" fn _cairo_traps_add_clipped_trap(
    mut traps: *mut cairo_traps_t,
    mut _top: cairo_fixed_t,
    mut _bottom: cairo_fixed_t,
    mut _left: *const cairo_line_t,
    mut _right: *const cairo_line_t,
) {
    if (*traps).num_limits != 0 {
        let mut b: *const cairo_box_t = &mut (*traps).bounds;
        let mut top: cairo_fixed_t = _top;
        let mut bottom: cairo_fixed_t = _bottom;
        let mut left: cairo_line_t = *_left;
        let mut right: cairo_line_t = *_right;
        if left.p1.x >= (*b).p2.x && left.p2.x >= (*b).p2.x {
            return;
        }
        if right.p1.x <= (*b).p1.x && right.p2.x <= (*b).p1.x {
            return;
        }
        if top >= (*b).p2.y || bottom <= (*b).p1.y {
            return;
        }
        if top < (*b).p1.y {
            top = (*b).p1.y;
        }
        if bottom > (*b).p2.y {
            bottom = (*b).p2.y;
        }
        if left.p1.x <= (*b).p1.x && left.p2.x <= (*b).p1.x {
            left.p2.x = (*b).p1.x;
            left.p1.x = left.p2.x;
        }
        if right.p1.x >= (*b).p2.x && right.p2.x >= (*b).p2.x {
            right.p2.x = (*b).p2.x;
            right.p1.x = right.p2.x;
        }
        if top >= bottom {
            return;
        }
        if right.p1.x <= left.p1.x && right.p1.y == left.p1.y && right.p2.x <= left.p2.x
            && right.p2.y == left.p2.y
        {
            return;
        }
        _cairo_traps_add_trap(traps, top, bottom, &mut left, &mut right);
    } else {
        _cairo_traps_add_trap(traps, _top, _bottom, _left, _right);
    };
}
unsafe extern "C" fn _compare_point_fixed_by_y(
    mut av: *const libc::c_void,
    mut bv: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const cairo_point_t = av as *const cairo_point_t;
    let mut b: *const cairo_point_t = bv as *const cairo_point_t;
    let mut ret: libc::c_int = (*a).y - (*b).y;
    if ret == 0 as libc::c_int {
        ret = (*a).x - (*b).x;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_tessellate_convex_quad(
    mut traps: *mut cairo_traps_t,
    mut q: *const cairo_point_t,
) {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ab: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut ad: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut b_left_of_d: cairo_bool_t = 0;
    let mut left: cairo_line_t = cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut right: cairo_line_t = cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    a = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        if _compare_point_fixed_by_y(
            &*q.offset(i as isize) as *const cairo_point_t as *const libc::c_void,
            &*q.offset(a as isize) as *const cairo_point_t as *const libc::c_void,
        ) < 0 as libc::c_int
        {
            a = i;
        }
        i += 1;
    }
    b = (a + 1 as libc::c_int) % 4 as libc::c_int;
    c = (a + 2 as libc::c_int) % 4 as libc::c_int;
    d = (a + 3 as libc::c_int) % 4 as libc::c_int;
    if _compare_point_fixed_by_y(
        &*q.offset(d as isize) as *const cairo_point_t as *const libc::c_void,
        &*q.offset(b as isize) as *const cairo_point_t as *const libc::c_void,
    ) < 0 as libc::c_int
    {
        b = (a + 3 as libc::c_int) % 4 as libc::c_int;
        d = (a + 1 as libc::c_int) % 4 as libc::c_int;
    }
    if (*q.offset(a as isize)).x == (*q.offset(b as isize)).x
        && (*q.offset(a as isize)).y == (*q.offset(b as isize)).y
    {
        _cairo_slope_init(&mut ab, &*q.offset(a as isize), &*q.offset(c as isize));
    } else {
        _cairo_slope_init(&mut ab, &*q.offset(a as isize), &*q.offset(b as isize));
    }
    _cairo_slope_init(&mut ad, &*q.offset(a as isize), &*q.offset(d as isize));
    b_left_of_d = (_cairo_slope_compare(&mut ab, &mut ad) > 0 as libc::c_int)
        as libc::c_int;
    if (*q.offset(c as isize)).y <= (*q.offset(d as isize)).y {
        if b_left_of_d != 0 {
            left.p1 = *q.offset(a as isize);
            left.p2 = *q.offset(b as isize);
            right.p1 = *q.offset(a as isize);
            right.p2 = *q.offset(d as isize);
            _cairo_traps_add_clipped_trap(
                traps,
                (*q.offset(a as isize)).y,
                (*q.offset(b as isize)).y,
                &mut left,
                &mut right,
            );
            left.p1 = *q.offset(b as isize);
            left.p2 = *q.offset(c as isize);
            _cairo_traps_add_clipped_trap(
                traps,
                (*q.offset(b as isize)).y,
                (*q.offset(c as isize)).y,
                &mut left,
                &mut right,
            );
            left.p1 = *q.offset(c as isize);
            left.p2 = *q.offset(d as isize);
            _cairo_traps_add_clipped_trap(
                traps,
                (*q.offset(c as isize)).y,
                (*q.offset(d as isize)).y,
                &mut left,
                &mut right,
            );
        } else {
            left.p1 = *q.offset(a as isize);
            left.p2 = *q.offset(d as isize);
            right.p1 = *q.offset(a as isize);
            right.p2 = *q.offset(b as isize);
            _cairo_traps_add_clipped_trap(
                traps,
                (*q.offset(a as isize)).y,
                (*q.offset(b as isize)).y,
                &mut left,
                &mut right,
            );
            right.p1 = *q.offset(b as isize);
            right.p2 = *q.offset(c as isize);
            _cairo_traps_add_clipped_trap(
                traps,
                (*q.offset(b as isize)).y,
                (*q.offset(c as isize)).y,
                &mut left,
                &mut right,
            );
            right.p1 = *q.offset(c as isize);
            right.p2 = *q.offset(d as isize);
            _cairo_traps_add_clipped_trap(
                traps,
                (*q.offset(c as isize)).y,
                (*q.offset(d as isize)).y,
                &mut left,
                &mut right,
            );
        }
    } else if b_left_of_d != 0 {
        left.p1 = *q.offset(a as isize);
        left.p2 = *q.offset(b as isize);
        right.p1 = *q.offset(a as isize);
        right.p2 = *q.offset(d as isize);
        _cairo_traps_add_clipped_trap(
            traps,
            (*q.offset(a as isize)).y,
            (*q.offset(b as isize)).y,
            &mut left,
            &mut right,
        );
        left.p1 = *q.offset(b as isize);
        left.p2 = *q.offset(c as isize);
        _cairo_traps_add_clipped_trap(
            traps,
            (*q.offset(b as isize)).y,
            (*q.offset(d as isize)).y,
            &mut left,
            &mut right,
        );
        right.p1 = *q.offset(d as isize);
        right.p2 = *q.offset(c as isize);
        _cairo_traps_add_clipped_trap(
            traps,
            (*q.offset(d as isize)).y,
            (*q.offset(c as isize)).y,
            &mut left,
            &mut right,
        );
    } else {
        left.p1 = *q.offset(a as isize);
        left.p2 = *q.offset(d as isize);
        right.p1 = *q.offset(a as isize);
        right.p2 = *q.offset(b as isize);
        _cairo_traps_add_clipped_trap(
            traps,
            (*q.offset(a as isize)).y,
            (*q.offset(b as isize)).y,
            &mut left,
            &mut right,
        );
        right.p1 = *q.offset(b as isize);
        right.p2 = *q.offset(c as isize);
        _cairo_traps_add_clipped_trap(
            traps,
            (*q.offset(b as isize)).y,
            (*q.offset(d as isize)).y,
            &mut left,
            &mut right,
        );
        left.p1 = *q.offset(d as isize);
        left.p2 = *q.offset(c as isize);
        _cairo_traps_add_clipped_trap(
            traps,
            (*q.offset(d as isize)).y,
            (*q.offset(c as isize)).y,
            &mut left,
            &mut right,
        );
    };
}
unsafe extern "C" fn add_tri(
    mut traps: *mut cairo_traps_t,
    mut y1: libc::c_int,
    mut y2: libc::c_int,
    mut left: *const cairo_line_t,
    mut right: *const cairo_line_t,
) {
    if y2 < y1 {
        let mut tmp: libc::c_int = y1;
        y1 = y2;
        y2 = tmp;
    }
    if _cairo_lines_compare_at_y(left, right, y1) > 0 as libc::c_int {
        let mut tmp_0: *const cairo_line_t = left;
        left = right;
        right = tmp_0;
    }
    _cairo_traps_add_clipped_trap(traps, y1, y2, left, right);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_tessellate_triangle_with_edges(
    mut traps: *mut cairo_traps_t,
    mut t: *const cairo_point_t,
    mut edges: *const cairo_point_t,
) {
    let mut lines: [cairo_line_t; 3] = [cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    }; 3];
    if (*edges.offset(0 as libc::c_int as isize)).y
        <= (*edges.offset(1 as libc::c_int as isize)).y
    {
        lines[0 as libc::c_int as usize].p1 = *edges.offset(0 as libc::c_int as isize);
        lines[0 as libc::c_int as usize].p2 = *edges.offset(1 as libc::c_int as isize);
    } else {
        lines[0 as libc::c_int as usize].p1 = *edges.offset(1 as libc::c_int as isize);
        lines[0 as libc::c_int as usize].p2 = *edges.offset(0 as libc::c_int as isize);
    }
    if (*edges.offset(2 as libc::c_int as isize)).y
        <= (*edges.offset(3 as libc::c_int as isize)).y
    {
        lines[1 as libc::c_int as usize].p1 = *edges.offset(2 as libc::c_int as isize);
        lines[1 as libc::c_int as usize].p2 = *edges.offset(3 as libc::c_int as isize);
    } else {
        lines[1 as libc::c_int as usize].p1 = *edges.offset(3 as libc::c_int as isize);
        lines[1 as libc::c_int as usize].p2 = *edges.offset(2 as libc::c_int as isize);
    }
    if (*t.offset(1 as libc::c_int as isize)).y
        == (*t.offset(2 as libc::c_int as isize)).y
    {
        add_tri(
            traps,
            (*t.offset(0 as libc::c_int as isize)).y,
            (*t.offset(1 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(0 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        return;
    }
    if (*t.offset(1 as libc::c_int as isize)).y
        <= (*t.offset(2 as libc::c_int as isize)).y
    {
        lines[2 as libc::c_int as usize].p1 = *t.offset(1 as libc::c_int as isize);
        lines[2 as libc::c_int as usize].p2 = *t.offset(2 as libc::c_int as isize);
    } else {
        lines[2 as libc::c_int as usize].p1 = *t.offset(2 as libc::c_int as isize);
        lines[2 as libc::c_int as usize].p2 = *t.offset(1 as libc::c_int as isize);
    }
    if ((*t.offset(1 as libc::c_int as isize)).y
        - (*t.offset(0 as libc::c_int as isize)).y < 0 as libc::c_int) as libc::c_int
        ^ ((*t.offset(2 as libc::c_int as isize)).y
            - (*t.offset(0 as libc::c_int as isize)).y < 0 as libc::c_int) as libc::c_int
        != 0
    {
        add_tri(
            traps,
            (*t.offset(0 as libc::c_int as isize)).y,
            (*t.offset(1 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(0 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(2 as libc::c_int as isize),
        );
        add_tri(
            traps,
            (*t.offset(0 as libc::c_int as isize)).y,
            (*t.offset(2 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(1 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(2 as libc::c_int as isize),
        );
    } else if abs(
        (*t.offset(1 as libc::c_int as isize)).y
            - (*t.offset(0 as libc::c_int as isize)).y,
    )
        < abs(
            (*t.offset(2 as libc::c_int as isize)).y
                - (*t.offset(0 as libc::c_int as isize)).y,
        )
    {
        add_tri(
            traps,
            (*t.offset(0 as libc::c_int as isize)).y,
            (*t.offset(1 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(0 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        add_tri(
            traps,
            (*t.offset(1 as libc::c_int as isize)).y,
            (*t.offset(2 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(2 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
    } else {
        add_tri(
            traps,
            (*t.offset(0 as libc::c_int as isize)).y,
            (*t.offset(2 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(1 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        add_tri(
            traps,
            (*t.offset(1 as libc::c_int as isize)).y,
            (*t.offset(2 as libc::c_int as isize)).y,
            &mut *lines.as_mut_ptr().offset(2 as libc::c_int as isize),
            &mut *lines.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_init_boxes(
    mut traps: *mut cairo_traps_t,
    mut boxes: *const cairo_boxes_t,
) -> cairo_status_t {
    let mut trap: *mut cairo_trapezoid_t = 0 as *mut cairo_trapezoid_t;
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    _cairo_traps_init(traps);
    while (*traps).traps_size < (*boxes).num_boxes {
        if _cairo_traps_grow(traps) == 0 {
            _cairo_traps_fini(traps);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    (*traps).num_traps = (*boxes).num_boxes;
    (*traps).set_is_rectilinear(1 as libc::c_int as libc::c_uint);
    (*traps).set_is_rectangular(1 as libc::c_int as libc::c_uint);
    (*traps).set_maybe_region((*boxes).is_pixel_aligned);
    trap = &mut *((*traps).traps).offset(0 as libc::c_int as isize)
        as *mut cairo_trapezoid_t;
    chunk = &(*boxes).chunks;
    while !chunk.is_null() {
        let mut box_0: *const cairo_box_t = 0 as *const cairo_box_t;
        let mut i: libc::c_int = 0;
        box_0 = (*chunk).base;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            (*trap).top = (*box_0).p1.y;
            (*trap).bottom = (*box_0).p2.y;
            (*trap).left.p1 = (*box_0).p1;
            (*trap).left.p2.x = (*box_0).p1.x;
            (*trap).left.p2.y = (*box_0).p2.y;
            (*trap).right.p1.x = (*box_0).p2.x;
            (*trap).right.p1.y = (*box_0).p1.y;
            (*trap).right.p2 = (*box_0).p2;
            box_0 = box_0.offset(1);
            trap = trap.offset(1);
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_tessellate_rectangle(
    mut traps: *mut cairo_traps_t,
    mut top_left: *const cairo_point_t,
    mut bottom_right: *const cairo_point_t,
) -> cairo_status_t {
    let mut left: cairo_line_t = cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut right: cairo_line_t = cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut top: cairo_fixed_t = 0;
    let mut bottom: cairo_fixed_t = 0;
    if (*top_left).y == (*bottom_right).y {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*top_left).x == (*bottom_right).x {
        return CAIRO_STATUS_SUCCESS;
    }
    left.p2.x = (*top_left).x;
    left.p1.x = left.p2.x;
    right.p1.y = (*top_left).y;
    left.p1.y = right.p1.y;
    right.p2.x = (*bottom_right).x;
    right.p1.x = right.p2.x;
    right.p2.y = (*bottom_right).y;
    left.p2.y = right.p2.y;
    top = (*top_left).y;
    bottom = (*bottom_right).y;
    if (*traps).num_limits != 0 {
        let mut reversed: cairo_bool_t = 0;
        let mut n: libc::c_int = 0;
        if top >= (*traps).bounds.p2.y || bottom <= (*traps).bounds.p1.y {
            return CAIRO_STATUS_SUCCESS;
        }
        reversed = ((*top_left).x > (*bottom_right).x) as libc::c_int;
        if reversed != 0 {
            right.p2.x = (*top_left).x;
            right.p1.x = right.p2.x;
            left.p2.x = (*bottom_right).x;
            left.p1.x = left.p2.x;
        }
        if left.p1.x >= (*traps).bounds.p2.x || right.p1.x <= (*traps).bounds.p1.x {
            return CAIRO_STATUS_SUCCESS;
        }
        n = 0 as libc::c_int;
        while n < (*traps).num_limits {
            let mut limits: *const cairo_box_t = &*((*traps).limits).offset(n as isize)
                as *const cairo_box_t;
            let mut _left: cairo_line_t = cairo_line_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            let mut _right: cairo_line_t = cairo_line_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            let mut _top: cairo_fixed_t = 0;
            let mut _bottom: cairo_fixed_t = 0;
            if !(top >= (*limits).p2.y) {
                if !(bottom <= (*limits).p1.y) {
                    if !(left.p1.x >= (*limits).p2.x) {
                        if !(right.p1.x <= (*limits).p1.x) {
                            _top = top;
                            if _top < (*limits).p1.y {
                                _top = (*limits).p1.y;
                            }
                            _bottom = bottom;
                            if _bottom > (*limits).p2.y {
                                _bottom = (*limits).p2.y;
                            }
                            if !(_bottom <= _top) {
                                _left = left;
                                if _left.p1.x < (*limits).p1.x {
                                    _left.p1.x = (*limits).p1.x;
                                    _left.p1.y = (*limits).p1.y;
                                    _left.p2.x = (*limits).p1.x;
                                    _left.p2.y = (*limits).p2.y;
                                }
                                _right = right;
                                if _right.p1.x > (*limits).p2.x {
                                    _right.p1.x = (*limits).p2.x;
                                    _right.p1.y = (*limits).p1.y;
                                    _right.p2.x = (*limits).p2.x;
                                    _right.p2.y = (*limits).p2.y;
                                }
                                if !(left.p1.x >= right.p1.x) {
                                    if reversed != 0 {
                                        _cairo_traps_add_trap(
                                            traps,
                                            _top,
                                            _bottom,
                                            &mut _right,
                                            &mut _left,
                                        );
                                    } else {
                                        _cairo_traps_add_trap(
                                            traps,
                                            _top,
                                            _bottom,
                                            &mut _left,
                                            &mut _right,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            n += 1;
        }
    } else {
        _cairo_traps_add_trap(traps, top, bottom, &mut left, &mut right);
    }
    return (*traps).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_translate(
    mut traps: *mut cairo_traps_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut xoff: cairo_fixed_t = 0;
    let mut yoff: cairo_fixed_t = 0;
    let mut t: *mut cairo_trapezoid_t = 0 as *mut cairo_trapezoid_t;
    let mut i: libc::c_int = 0;
    xoff = _cairo_fixed_from_int(x);
    yoff = _cairo_fixed_from_int(y);
    i = 0 as libc::c_int;
    t = (*traps).traps;
    while i < (*traps).num_traps {
        let ref mut fresh9 = (*t).top;
        *fresh9 += yoff;
        let ref mut fresh10 = (*t).bottom;
        *fresh10 += yoff;
        let ref mut fresh11 = (*t).left.p1.x;
        *fresh11 += xoff;
        let ref mut fresh12 = (*t).left.p1.y;
        *fresh12 += yoff;
        let ref mut fresh13 = (*t).left.p2.x;
        *fresh13 += xoff;
        let ref mut fresh14 = (*t).left.p2.y;
        *fresh14 += yoff;
        let ref mut fresh15 = (*t).right.p1.x;
        *fresh15 += xoff;
        let ref mut fresh16 = (*t).right.p1.y;
        *fresh16 += yoff;
        let ref mut fresh17 = (*t).right.p2.x;
        *fresh17 += xoff;
        let ref mut fresh18 = (*t).right.p2.y;
        *fresh18 += yoff;
        i += 1;
        t = t.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_trapezoid_array_translate_and_scale(
    mut offset_traps: *mut cairo_trapezoid_t,
    mut src_traps: *mut cairo_trapezoid_t,
    mut num_traps: libc::c_int,
    mut tx: libc::c_double,
    mut ty: libc::c_double,
    mut sx: libc::c_double,
    mut sy: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut xoff: cairo_fixed_t = _cairo_fixed_from_double(tx);
    let mut yoff: cairo_fixed_t = _cairo_fixed_from_double(ty);
    if sx == 1.0f64 && sy == 1.0f64 {
        i = 0 as libc::c_int;
        while i < num_traps {
            (*offset_traps.offset(i as isize))
                .top = (*src_traps.offset(i as isize)).top + yoff;
            (*offset_traps.offset(i as isize))
                .bottom = (*src_traps.offset(i as isize)).bottom + yoff;
            (*offset_traps.offset(i as isize))
                .left
                .p1
                .x = (*src_traps.offset(i as isize)).left.p1.x + xoff;
            (*offset_traps.offset(i as isize))
                .left
                .p1
                .y = (*src_traps.offset(i as isize)).left.p1.y + yoff;
            (*offset_traps.offset(i as isize))
                .left
                .p2
                .x = (*src_traps.offset(i as isize)).left.p2.x + xoff;
            (*offset_traps.offset(i as isize))
                .left
                .p2
                .y = (*src_traps.offset(i as isize)).left.p2.y + yoff;
            (*offset_traps.offset(i as isize))
                .right
                .p1
                .x = (*src_traps.offset(i as isize)).right.p1.x + xoff;
            (*offset_traps.offset(i as isize))
                .right
                .p1
                .y = (*src_traps.offset(i as isize)).right.p1.y + yoff;
            (*offset_traps.offset(i as isize))
                .right
                .p2
                .x = (*src_traps.offset(i as isize)).right.p2.x + xoff;
            (*offset_traps.offset(i as isize))
                .right
                .p2
                .y = (*src_traps.offset(i as isize)).right.p2.y + yoff;
            i += 1;
        }
    } else {
        let mut xsc: cairo_fixed_t = _cairo_fixed_from_double(sx);
        let mut ysc: cairo_fixed_t = _cairo_fixed_from_double(sy);
        i = 0 as libc::c_int;
        while i < num_traps {
            (*offset_traps.offset(i as isize))
                .top = _cairo_fixed_mul((*src_traps.offset(i as isize)).top + yoff, ysc);
            (*offset_traps.offset(i as isize))
                .bottom = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).bottom + yoff,
                ysc,
            );
            (*offset_traps.offset(i as isize))
                .left
                .p1
                .x = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).left.p1.x + xoff,
                xsc,
            );
            (*offset_traps.offset(i as isize))
                .left
                .p1
                .y = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).left.p1.y + yoff,
                ysc,
            );
            (*offset_traps.offset(i as isize))
                .left
                .p2
                .x = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).left.p2.x + xoff,
                xsc,
            );
            (*offset_traps.offset(i as isize))
                .left
                .p2
                .y = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).left.p2.y + yoff,
                ysc,
            );
            (*offset_traps.offset(i as isize))
                .right
                .p1
                .x = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).right.p1.x + xoff,
                xsc,
            );
            (*offset_traps.offset(i as isize))
                .right
                .p1
                .y = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).right.p1.y + yoff,
                ysc,
            );
            (*offset_traps.offset(i as isize))
                .right
                .p2
                .x = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).right.p2.x + xoff,
                xsc,
            );
            (*offset_traps.offset(i as isize))
                .right
                .p2
                .y = _cairo_fixed_mul(
                (*src_traps.offset(i as isize)).right.p2.y + yoff,
                ysc,
            );
            i += 1;
        }
    };
}
unsafe extern "C" fn _cairo_trap_contains(
    mut t: *mut cairo_trapezoid_t,
    mut pt: *mut cairo_point_t,
) -> cairo_bool_t {
    let mut slope_left: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut slope_pt: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut slope_right: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    if (*t).top > (*pt).y {
        return 0 as libc::c_int;
    }
    if (*t).bottom < (*pt).y {
        return 0 as libc::c_int;
    }
    _cairo_slope_init(&mut slope_left, &mut (*t).left.p1, &mut (*t).left.p2);
    _cairo_slope_init(&mut slope_pt, &mut (*t).left.p1, pt);
    if _cairo_slope_compare(&mut slope_left, &mut slope_pt) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    _cairo_slope_init(&mut slope_right, &mut (*t).right.p1, &mut (*t).right.p2);
    _cairo_slope_init(&mut slope_pt, &mut (*t).right.p1, pt);
    if _cairo_slope_compare(&mut slope_pt, &mut slope_right) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_contain(
    mut traps: *const cairo_traps_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut point: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    point.x = _cairo_fixed_from_double(x);
    point.y = _cairo_fixed_from_double(y);
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        if _cairo_trap_contains(&mut *((*traps).traps).offset(i as isize), &mut point)
            != 0
        {
            return 1 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _line_compute_intersection_x_for_y(
    mut line: *const cairo_line_t,
    mut y: cairo_fixed_t,
) -> cairo_fixed_t {
    return _cairo_edge_compute_intersection_x_for_y(&(*line).p1, &(*line).p2, y);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_extents(
    mut traps: *const cairo_traps_t,
    mut extents: *mut cairo_box_t,
) {
    let mut i: libc::c_int = 0;
    if (*traps).num_traps == 0 as libc::c_int {
        let ref mut fresh19 = (*extents).p1.y;
        *fresh19 = 0 as libc::c_int;
        (*extents).p1.x = *fresh19;
        let ref mut fresh20 = (*extents).p2.y;
        *fresh20 = 0 as libc::c_int;
        (*extents).p2.x = *fresh20;
        return;
    }
    let ref mut fresh21 = (*extents).p1.y;
    *fresh21 = 2147483647 as libc::c_int;
    (*extents).p1.x = *fresh21;
    let ref mut fresh22 = (*extents).p2.y;
    *fresh22 = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*extents).p2.x = *fresh22;
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        let mut trap: *const cairo_trapezoid_t = &mut *((*traps).traps)
            .offset(i as isize) as *mut cairo_trapezoid_t;
        if (*trap).top < (*extents).p1.y {
            (*extents).p1.y = (*trap).top;
        }
        if (*trap).bottom > (*extents).p2.y {
            (*extents).p2.y = (*trap).bottom;
        }
        if (*trap).left.p1.x < (*extents).p1.x {
            let mut x: cairo_fixed_t = (*trap).left.p1.x;
            if (*trap).top != (*trap).left.p1.y {
                x = _line_compute_intersection_x_for_y(&(*trap).left, (*trap).top);
                if x < (*extents).p1.x {
                    (*extents).p1.x = x;
                }
            } else {
                (*extents).p1.x = x;
            }
        }
        if (*trap).left.p2.x < (*extents).p1.x {
            let mut x_0: cairo_fixed_t = (*trap).left.p2.x;
            if (*trap).bottom != (*trap).left.p2.y {
                x_0 = _line_compute_intersection_x_for_y(&(*trap).left, (*trap).bottom);
                if x_0 < (*extents).p1.x {
                    (*extents).p1.x = x_0;
                }
            } else {
                (*extents).p1.x = x_0;
            }
        }
        if (*trap).right.p1.x > (*extents).p2.x {
            let mut x_1: cairo_fixed_t = (*trap).right.p1.x;
            if (*trap).top != (*trap).right.p1.y {
                x_1 = _line_compute_intersection_x_for_y(&(*trap).right, (*trap).top);
                if x_1 > (*extents).p2.x {
                    (*extents).p2.x = x_1;
                }
            } else {
                (*extents).p2.x = x_1;
            }
        }
        if (*trap).right.p2.x > (*extents).p2.x {
            let mut x_2: cairo_fixed_t = (*trap).right.p2.x;
            if (*trap).bottom != (*trap).right.p2.y {
                x_2 = _line_compute_intersection_x_for_y(&(*trap).right, (*trap).bottom);
                if x_2 > (*extents).p2.x {
                    (*extents).p2.x = x_2;
                }
            } else {
                (*extents).p2.x = x_2;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn _mono_edge_is_vertical(
    mut line: *const cairo_line_t,
) -> cairo_bool_t {
    return (_cairo_fixed_integer_round_down((*line).p1.x)
        == _cairo_fixed_integer_round_down((*line).p2.x)) as libc::c_int;
}
unsafe extern "C" fn _traps_are_pixel_aligned(
    mut traps: *mut cairo_traps_t,
    mut antialias: cairo_antialias_t,
) -> cairo_bool_t {
    let mut i: libc::c_int = 0;
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            if _mono_edge_is_vertical(&mut (*((*traps).traps).offset(i as isize)).left)
                == 0
                || _mono_edge_is_vertical(
                    &mut (*((*traps).traps).offset(i as isize)).right,
                ) == 0
            {
                (*traps).set_maybe_region(0 as libc::c_int as libc::c_uint);
                return 0 as libc::c_int;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            if (*((*traps).traps).offset(i as isize)).left.p1.x
                != (*((*traps).traps).offset(i as isize)).left.p2.x
                || (*((*traps).traps).offset(i as isize)).right.p1.x
                    != (*((*traps).traps).offset(i as isize)).right.p2.x
                || _cairo_fixed_is_integer((*((*traps).traps).offset(i as isize)).top)
                    == 0
                || _cairo_fixed_is_integer((*((*traps).traps).offset(i as isize)).bottom)
                    == 0
                || _cairo_fixed_is_integer(
                    (*((*traps).traps).offset(i as isize)).left.p1.x,
                ) == 0
                || _cairo_fixed_is_integer(
                    (*((*traps).traps).offset(i as isize)).right.p1.x,
                ) == 0
            {
                (*traps).set_maybe_region(0 as libc::c_int as libc::c_uint);
                return 0 as libc::c_int;
            }
            i += 1;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_extract_region(
    mut traps: *mut cairo_traps_t,
    mut antialias: cairo_antialias_t,
    mut region: *mut *mut cairo_region_t,
) -> cairo_int_status_t {
    let mut stack_rects: [cairo_rectangle_int_t; 128] = [cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 128];
    let mut rects: *mut cairo_rectangle_int_t = stack_rects.as_mut_ptr();
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut rect_count: libc::c_int = 0;
    if antialias as libc::c_uint != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        && (*traps).maybe_region() == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if _traps_are_pixel_aligned(traps, antialias) == 0 {
        (*traps).set_maybe_region(0 as libc::c_int as libc::c_uint);
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*traps).num_traps
        > (::std::mem::size_of::<[cairo_rectangle_int_t; 128]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<cairo_rectangle_int_t>() as libc::c_ulong,
            ) as libc::c_int
    {
        rects = _cairo_malloc_ab(
            (*traps).num_traps as size_t,
            ::std::mem::size_of::<cairo_rectangle_int_t>() as libc::c_ulong,
        ) as *mut cairo_rectangle_int_t;
        if rects.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    rect_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        let mut x1: libc::c_int = 0;
        let mut y1: libc::c_int = 0;
        let mut x2: libc::c_int = 0;
        let mut y2: libc::c_int = 0;
        if antialias as libc::c_uint
            == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        {
            x1 = _cairo_fixed_integer_round_down(
                (*((*traps).traps).offset(i as isize)).left.p1.x,
            );
            y1 = _cairo_fixed_integer_round_down(
                (*((*traps).traps).offset(i as isize)).top,
            );
            x2 = _cairo_fixed_integer_round_down(
                (*((*traps).traps).offset(i as isize)).right.p1.x,
            );
            y2 = _cairo_fixed_integer_round_down(
                (*((*traps).traps).offset(i as isize)).bottom,
            );
        } else {
            x1 = _cairo_fixed_integer_part(
                (*((*traps).traps).offset(i as isize)).left.p1.x,
            );
            y1 = _cairo_fixed_integer_part((*((*traps).traps).offset(i as isize)).top);
            x2 = _cairo_fixed_integer_part(
                (*((*traps).traps).offset(i as isize)).right.p1.x,
            );
            y2 = _cairo_fixed_integer_part(
                (*((*traps).traps).offset(i as isize)).bottom,
            );
        }
        if x2 > x1 && y2 > y1 {
            (*rects.offset(rect_count as isize)).x = x1;
            (*rects.offset(rect_count as isize)).y = y1;
            (*rects.offset(rect_count as isize)).width = x2 - x1;
            (*rects.offset(rect_count as isize)).height = y2 - y1;
            rect_count += 1;
        }
        i += 1;
    }
    *region = cairo_region_create_rectangles(rects, rect_count);
    status = (**region).status as cairo_int_status_t;
    if rects != stack_rects.as_mut_ptr() {
        free(rects as *mut libc::c_void);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_to_boxes(
    mut traps: *mut cairo_traps_t,
    mut antialias: cairo_antialias_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_bool_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        if (*((*traps).traps).offset(i as isize)).left.p1.x
            != (*((*traps).traps).offset(i as isize)).left.p2.x
            || (*((*traps).traps).offset(i as isize)).right.p1.x
                != (*((*traps).traps).offset(i as isize)).right.p2.x
        {
            return 0 as libc::c_int;
        }
        i += 1;
    }
    _cairo_boxes_init(boxes);
    (*boxes).num_boxes = (*traps).num_traps;
    let ref mut fresh23 = (*boxes).chunks.base;
    *fresh23 = (*traps).traps as *mut cairo_box_t;
    (*boxes).chunks.count = (*traps).num_traps;
    (*boxes).chunks.size = (*traps).num_traps;
    if antialias as libc::c_uint != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            let mut x1: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).left.p1.x;
            let mut x2: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .right
                .p1
                .x;
            let mut y1: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).top;
            let mut y2: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).bottom;
            (*((*boxes).chunks.base).offset(i as isize)).p1.x = x1;
            (*((*boxes).chunks.base).offset(i as isize)).p1.y = y1;
            (*((*boxes).chunks.base).offset(i as isize)).p2.x = x2;
            (*((*boxes).chunks.base).offset(i as isize)).p2.y = y2;
            if (*boxes).is_pixel_aligned != 0 {
                (*boxes)
                    .is_pixel_aligned = (_cairo_fixed_is_integer(x1) != 0
                    && _cairo_fixed_is_integer(y1) != 0
                    && _cairo_fixed_is_integer(x2) != 0
                    && _cairo_fixed_is_integer(y2) != 0) as libc::c_int as libc::c_uint;
            }
            i += 1;
        }
    } else {
        (*boxes).is_pixel_aligned = 1 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            let mut x1_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .left
                .p1
                .x;
            let mut x2_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .right
                .p1
                .x;
            let mut y1_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).top;
            let mut y2_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).bottom;
            (*((*boxes).chunks.base).offset(i as isize))
                .p1
                .x = _cairo_fixed_round_down(x1_0);
            (*((*boxes).chunks.base).offset(i as isize))
                .p1
                .y = _cairo_fixed_round_down(y1_0);
            (*((*boxes).chunks.base).offset(i as isize))
                .p2
                .x = _cairo_fixed_round_down(x2_0);
            (*((*boxes).chunks.base).offset(i as isize))
                .p2
                .y = _cairo_fixed_round_down(y2_0);
            i += 1;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _sanitize_trap(mut t: *mut cairo_trapezoid_t) {
    let mut s: cairo_trapezoid_t = *t;
    if (*t).left.p1.y != (*t).top {
        (*t)
            .left
            .p1
            .x = s.left.p2.x
            + _cairo_fixed_mul_div_floor(
                s.left.p1.x - s.left.p2.x,
                s.top - s.left.p2.y,
                s.left.p1.y - s.left.p2.y,
            );
        (*t).left.p1.y = s.top;
    }
    if (*t).left.p2.y != (*t).bottom {
        (*t)
            .left
            .p2
            .x = s.left.p2.x
            + _cairo_fixed_mul_div_floor(
                s.left.p1.x - s.left.p2.x,
                s.bottom - s.left.p2.y,
                s.left.p1.y - s.left.p2.y,
            );
        (*t).left.p2.y = s.bottom;
    }
    if (*t).right.p1.y != (*t).top {
        (*t)
            .right
            .p1
            .x = s.right.p2.x
            + _cairo_fixed_mul_div_floor(
                s.right.p1.x - s.right.p2.x,
                s.top - s.right.p2.y,
                s.right.p1.y - s.right.p2.y,
            );
        (*t).right.p1.y = s.top;
    }
    if (*t).right.p2.y != (*t).bottom {
        (*t)
            .right
            .p2
            .x = s.right.p2.x
            + _cairo_fixed_mul_div_floor(
                s.right.p1.x - s.right.p2.x,
                s.bottom - s.right.p2.y,
                s.right.p1.y - s.right.p2.y,
            );
        (*t).right.p2.y = s.bottom;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_path(
    mut traps: *const cairo_traps_t,
    mut path: *mut cairo_path_fixed_t,
) -> cairo_status_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        let mut trap: cairo_trapezoid_t = *((*traps).traps).offset(i as isize);
        if !(trap.top == trap.bottom) {
            _sanitize_trap(&mut trap);
            status = _cairo_path_fixed_move_to(path, trap.left.p1.x, trap.top);
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_path_fixed_line_to(path, trap.right.p1.x, trap.top);
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_path_fixed_line_to(path, trap.right.p2.x, trap.bottom);
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_path_fixed_line_to(path, trap.left.p2.x, trap.bottom);
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_path_fixed_close_path(path);
            if status as u64 != 0 {
                return status;
            }
        }
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_traps(
    mut file: *mut FILE,
    mut traps: *const cairo_traps_t,
) {
    let mut extents: cairo_box_t = cairo_line_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut n: libc::c_int = 0;
    _cairo_traps_extents(traps, &mut extents);
    fprintf(
        file,
        b"extents=(%d, %d, %d, %d)\n\0" as *const u8 as *const libc::c_char,
        extents.p1.x,
        extents.p1.y,
        extents.p2.x,
        extents.p2.y,
    );
    n = 0 as libc::c_int;
    while n < (*traps).num_traps {
        fprintf(
            file,
            b"%d %d L:(%d, %d), (%d, %d) R:(%d, %d), (%d, %d)\n\0" as *const u8
                as *const libc::c_char,
            (*((*traps).traps).offset(n as isize)).top,
            (*((*traps).traps).offset(n as isize)).bottom,
            (*((*traps).traps).offset(n as isize)).left.p1.x,
            (*((*traps).traps).offset(n as isize)).left.p1.y,
            (*((*traps).traps).offset(n as isize)).left.p2.x,
            (*((*traps).traps).offset(n as isize)).left.p2.y,
            (*((*traps).traps).offset(n as isize)).right.p1.x,
            (*((*traps).traps).offset(n as isize)).right.p1.y,
            (*((*traps).traps).offset(n as isize)).right.p2.x,
            (*((*traps).traps).offset(n as isize)).right.p2.y,
        );
        n += 1;
    }
}
unsafe extern "C" fn span_to_traps(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_trap_renderer = abstract_renderer as *mut cairo_trap_renderer;
    let mut top: cairo_fixed_t = 0;
    let mut bot: cairo_fixed_t = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    top = _cairo_fixed_from_int(y);
    bot = _cairo_fixed_from_int(y + h);
    loop {
        if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
            let mut x0: cairo_fixed_t = _cairo_fixed_from_int(
                (*spans.offset(0 as libc::c_int as isize)).x,
            );
            let mut x1: cairo_fixed_t = _cairo_fixed_from_int(
                (*spans.offset(1 as libc::c_int as isize)).x,
            );
            let mut left: cairo_line_t = {
                let mut init = _cairo_line {
                    p1: {
                        let mut init = _cairo_point { x: x0, y: top };
                        init
                    },
                    p2: {
                        let mut init = _cairo_point { x: x0, y: bot };
                        init
                    },
                };
                init
            };
            let mut right: cairo_line_t = {
                let mut init = _cairo_line {
                    p1: {
                        let mut init = _cairo_point { x: x1, y: top };
                        init
                    },
                    p2: {
                        let mut init = _cairo_point { x: x1, y: bot };
                        init
                    },
                };
                init
            };
            _cairo_traps_add_trap((*r).traps, top, bot, &mut left, &mut right);
        }
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rasterise_polygon_to_traps(
    mut polygon: *mut cairo_polygon_t,
    mut fill_rule: cairo_fill_rule_t,
    mut antialias: cairo_antialias_t,
    mut traps: *mut cairo_traps_t,
) -> cairo_int_status_t {
    let mut renderer: cairo_trap_renderer = cairo_trap_renderer {
        base: cairo_span_renderer_t {
            status: CAIRO_STATUS_SUCCESS,
            destroy: None,
            render_rows: None,
            finish: None,
        },
        traps: 0 as *mut cairo_traps_t,
    };
    let mut converter: *mut cairo_scan_converter_t = 0 as *mut cairo_scan_converter_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"antialias == CAIRO_ANTIALIAS_NONE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps.c\0" as *const u8 as *const libc::c_char,
            1108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"cairo_int_status_t _cairo_rasterise_polygon_to_traps(cairo_polygon_t *, cairo_fill_rule_t, cairo_antialias_t, cairo_traps_t *)\0",
            ))
                .as_ptr(),
        );
    }
    renderer.traps = traps;
    renderer
        .base
        .render_rows = Some(
        span_to_traps
            as unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                libc::c_int,
                *const cairo_half_open_span_t,
                libc::c_uint,
            ) -> cairo_status_t,
    );
    _cairo_box_round_to_rectangle(&mut (*polygon).extents, &mut r);
    converter = _cairo_mono_scan_converter_create(
        r.x,
        r.y,
        r.x + r.width,
        r.y + r.height,
        fill_rule,
    );
    status = _cairo_mono_scan_converter_add_polygon(
        converter as *mut libc::c_void,
        polygon,
    ) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = ((*converter).generate)
            .expect(
                "non-null function pointer",
            )(converter as *mut libc::c_void, &mut renderer.base) as cairo_int_status_t;
    }
    ((*converter).destroy)
        .expect("non-null function pointer")(converter as *mut libc::c_void);
    return status;
}
