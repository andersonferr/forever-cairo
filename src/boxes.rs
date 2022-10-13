use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo_region;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub struct cairo_box_renderer {
    pub base: cairo_span_renderer_t,
    pub boxes: *mut cairo_boxes_t,
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab_plus_c(
    mut a: size_t,
    mut size: size_t,
    mut c: size_t,
) -> *mut libc::c_void {
    let mut d: size_t = 0;
    let mut e: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut d as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    let (fresh2, fresh3) = d.overflowing_add(c);
    *(&mut e as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return if e != 0 as libc::c_int as libc::c_ulong {
        malloc(e)
    } else {
        0 as *mut libc::c_void
    };
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
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh4, fresh5) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh4;
    if fresh5 {
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
unsafe extern "C" fn _cairo_box_from_integers(
    mut box_0: *mut cairo_box_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    (*box_0).p1.x = _cairo_fixed_from_int(x);
    (*box_0).p1.y = _cairo_fixed_from_int(y);
    (*box_0).p2.x = _cairo_fixed_from_int(x + w);
    (*box_0).p2.y = _cairo_fixed_from_int(y + h);
}
#[inline]
unsafe extern "C" fn _cairo_box_is_pixel_aligned(
    mut box_0: *const cairo_box_t,
) -> cairo_bool_t {
    let mut f: cairo_fixed_t = 0;
    f = 0 as libc::c_int;
    f
        |= (*box_0).p1.x
            & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
    f
        |= (*box_0).p1.y
            & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
    f
        |= (*box_0).p2.x
            & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
    f
        |= (*box_0).p2.y
            & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
    return (f == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_init(mut boxes: *mut cairo_boxes_t) {
    (*boxes).status = CAIRO_STATUS_SUCCESS;
    (*boxes).num_limits = 0 as libc::c_int;
    (*boxes).num_boxes = 0 as libc::c_int;
    let ref mut fresh6 = (*boxes).tail;
    *fresh6 = &mut (*boxes).chunks;
    let ref mut fresh7 = (*boxes).chunks.next;
    *fresh7 = 0 as *mut _cairo_boxes_chunk;
    let ref mut fresh8 = (*boxes).chunks.base;
    *fresh8 = ((*boxes).boxes_embedded).as_mut_ptr();
    (*boxes)
        .chunks
        .size = (::std::mem::size_of::<[cairo_box_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
        as libc::c_int;
    (*boxes).chunks.count = 0 as libc::c_int;
    (*boxes).is_pixel_aligned = 1 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_init_from_rectangle(
    mut boxes: *mut cairo_boxes_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    _cairo_boxes_init(boxes);
    _cairo_box_from_integers(
        &mut *((*boxes).chunks.base).offset(0 as libc::c_int as isize),
        x,
        y,
        w,
        h,
    );
    (*boxes).num_boxes = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_init_with_clip(
    mut boxes: *mut cairo_boxes_t,
    mut clip: *mut cairo_clip_t,
) {
    _cairo_boxes_init(boxes);
    if !clip.is_null() {
        _cairo_boxes_limit(boxes, (*clip).boxes, (*clip).num_boxes);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_init_for_array(
    mut boxes: *mut cairo_boxes_t,
    mut array: *mut cairo_box_t,
    mut num_boxes: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    (*boxes).status = CAIRO_STATUS_SUCCESS;
    (*boxes).num_limits = 0 as libc::c_int;
    (*boxes).num_boxes = num_boxes;
    let ref mut fresh9 = (*boxes).tail;
    *fresh9 = &mut (*boxes).chunks;
    let ref mut fresh10 = (*boxes).chunks.next;
    *fresh10 = 0 as *mut _cairo_boxes_chunk;
    let ref mut fresh11 = (*boxes).chunks.base;
    *fresh11 = array;
    (*boxes).chunks.size = num_boxes;
    (*boxes).chunks.count = num_boxes;
    n = 0 as libc::c_int;
    while n < num_boxes {
        if _cairo_fixed_is_integer((*array.offset(n as isize)).p1.x) == 0
            || _cairo_fixed_is_integer((*array.offset(n as isize)).p1.y) == 0
            || _cairo_fixed_is_integer((*array.offset(n as isize)).p2.x) == 0
            || _cairo_fixed_is_integer((*array.offset(n as isize)).p2.y) == 0
        {
            break;
        }
        n += 1;
    }
    (*boxes).is_pixel_aligned = (n == num_boxes) as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_limit(
    mut boxes: *mut cairo_boxes_t,
    mut limits: *const cairo_box_t,
    mut num_limits: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let ref mut fresh12 = (*boxes).limits;
    *fresh12 = limits;
    (*boxes).num_limits = num_limits;
    if (*boxes).num_limits != 0 {
        (*boxes).limit = *limits.offset(0 as libc::c_int as isize);
        n = 1 as libc::c_int;
        while n < num_limits {
            if (*limits.offset(n as isize)).p1.x < (*boxes).limit.p1.x {
                (*boxes).limit.p1.x = (*limits.offset(n as isize)).p1.x;
            }
            if (*limits.offset(n as isize)).p1.y < (*boxes).limit.p1.y {
                (*boxes).limit.p1.y = (*limits.offset(n as isize)).p1.y;
            }
            if (*limits.offset(n as isize)).p2.x > (*boxes).limit.p2.x {
                (*boxes).limit.p2.x = (*limits.offset(n as isize)).p2.x;
            }
            if (*limits.offset(n as isize)).p2.y > (*boxes).limit.p2.y {
                (*boxes).limit.p2.y = (*limits.offset(n as isize)).p2.y;
            }
            n += 1;
        }
    }
}
unsafe extern "C" fn _cairo_boxes_add_internal(
    mut boxes: *mut cairo_boxes_t,
    mut box_0: *const cairo_box_t,
) {
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    if (*boxes).status as u64 != 0 {
        return;
    }
    chunk = (*boxes).tail;
    if (*chunk).count == (*chunk).size {
        let mut size: libc::c_int = 0;
        size = (*chunk).size * 2 as libc::c_int;
        let ref mut fresh13 = (*chunk).next;
        *fresh13 = _cairo_malloc_ab_plus_c(
            size as size_t,
            ::std::mem::size_of::<cairo_box_t>() as libc::c_ulong,
            ::std::mem::size_of::<_cairo_boxes_chunk>() as libc::c_ulong,
        ) as *mut _cairo_boxes_chunk;
        if ((*chunk).next).is_null() {
            (*boxes).status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return;
        }
        chunk = (*chunk).next;
        let ref mut fresh14 = (*boxes).tail;
        *fresh14 = chunk;
        let ref mut fresh15 = (*chunk).next;
        *fresh15 = 0 as *mut _cairo_boxes_chunk;
        (*chunk).count = 0 as libc::c_int;
        (*chunk).size = size;
        let ref mut fresh16 = (*chunk).base;
        *fresh16 = chunk.offset(1 as libc::c_int as isize) as *mut cairo_box_t;
    }
    let ref mut fresh17 = (*chunk).count;
    let fresh18 = *fresh17;
    *fresh17 = *fresh17 + 1;
    *((*chunk).base).offset(fresh18 as isize) = *box_0;
    let ref mut fresh19 = (*boxes).num_boxes;
    *fresh19 += 1;
    if (*boxes).is_pixel_aligned != 0 {
        (*boxes).is_pixel_aligned = _cairo_box_is_pixel_aligned(box_0) as libc::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_add(
    mut boxes: *mut cairo_boxes_t,
    mut antialias: cairo_antialias_t,
    mut box_0: *const cairo_box_t,
) -> cairo_status_t {
    let mut b: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        b.p1.x = _cairo_fixed_round_down((*box_0).p1.x);
        b.p1.y = _cairo_fixed_round_down((*box_0).p1.y);
        b.p2.x = _cairo_fixed_round_down((*box_0).p2.x);
        b.p2.y = _cairo_fixed_round_down((*box_0).p2.y);
        box_0 = &mut b;
    }
    if (*box_0).p1.y == (*box_0).p2.y {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*box_0).p1.x == (*box_0).p2.x {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*boxes).num_limits != 0 {
        let mut p1: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        let mut p2: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        let mut reversed: cairo_bool_t = 0 as libc::c_int;
        let mut n: libc::c_int = 0;
        if (*box_0).p1.x < (*box_0).p2.x {
            p1.x = (*box_0).p1.x;
            p2.x = (*box_0).p2.x;
        } else {
            p2.x = (*box_0).p1.x;
            p1.x = (*box_0).p2.x;
            reversed = (reversed == 0) as libc::c_int;
        }
        if p1.x >= (*boxes).limit.p2.x || p2.x <= (*boxes).limit.p1.x {
            return CAIRO_STATUS_SUCCESS;
        }
        if (*box_0).p1.y < (*box_0).p2.y {
            p1.y = (*box_0).p1.y;
            p2.y = (*box_0).p2.y;
        } else {
            p2.y = (*box_0).p1.y;
            p1.y = (*box_0).p2.y;
            reversed = (reversed == 0) as libc::c_int;
        }
        if p1.y >= (*boxes).limit.p2.y || p2.y <= (*boxes).limit.p1.y {
            return CAIRO_STATUS_SUCCESS;
        }
        n = 0 as libc::c_int;
        while n < (*boxes).num_limits {
            let mut limits: *const cairo_box_t = &*((*boxes).limits).offset(n as isize)
                as *const cairo_box_t;
            let mut _box: cairo_box_t = cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            let mut _p1: cairo_point_t = cairo_point_t { x: 0, y: 0 };
            let mut _p2: cairo_point_t = cairo_point_t { x: 0, y: 0 };
            if !(p1.x >= (*limits).p2.x || p2.x <= (*limits).p1.x) {
                if !(p1.y >= (*limits).p2.y || p2.y <= (*limits).p1.y) {
                    _p1 = p1;
                    if _p1.x < (*limits).p1.x {
                        _p1.x = (*limits).p1.x;
                    }
                    if _p1.y < (*limits).p1.y {
                        _p1.y = (*limits).p1.y;
                    }
                    _p2 = p2;
                    if _p2.x > (*limits).p2.x {
                        _p2.x = (*limits).p2.x;
                    }
                    if _p2.y > (*limits).p2.y {
                        _p2.y = (*limits).p2.y;
                    }
                    if !(_p2.y <= _p1.y || _p2.x <= _p1.x) {
                        _box.p1.y = _p1.y;
                        _box.p2.y = _p2.y;
                        if reversed != 0 {
                            _box.p1.x = _p2.x;
                            _box.p2.x = _p1.x;
                        } else {
                            _box.p1.x = _p1.x;
                            _box.p2.x = _p2.x;
                        }
                        _cairo_boxes_add_internal(boxes, &mut _box);
                    }
                }
            }
            n += 1;
        }
    } else {
        _cairo_boxes_add_internal(boxes, box_0);
    }
    return (*boxes).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_extents(
    mut boxes: *const cairo_boxes_t,
    mut box_0: *mut cairo_box_t,
) {
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut b: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut i: libc::c_int = 0;
    if (*boxes).num_boxes == 0 as libc::c_int {
        let ref mut fresh20 = (*box_0).p2.y;
        *fresh20 = 0 as libc::c_int;
        let ref mut fresh21 = (*box_0).p2.x;
        *fresh21 = *fresh20;
        let ref mut fresh22 = (*box_0).p1.y;
        *fresh22 = *fresh21;
        (*box_0).p1.x = *fresh22;
        return;
    }
    b = *((*boxes).chunks.base).offset(0 as libc::c_int as isize);
    chunk = &(*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            if (*((*chunk).base).offset(i as isize)).p1.x < b.p1.x {
                b.p1.x = (*((*chunk).base).offset(i as isize)).p1.x;
            }
            if (*((*chunk).base).offset(i as isize)).p1.y < b.p1.y {
                b.p1.y = (*((*chunk).base).offset(i as isize)).p1.y;
            }
            if (*((*chunk).base).offset(i as isize)).p2.x > b.p2.x {
                b.p2.x = (*((*chunk).base).offset(i as isize)).p2.x;
            }
            if (*((*chunk).base).offset(i as isize)).p2.y > b.p2.y {
                b.p2.y = (*((*chunk).base).offset(i as isize)).p2.y;
            }
            i += 1;
        }
        chunk = (*chunk).next;
    }
    *box_0 = b;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_clear(mut boxes: *mut cairo_boxes_t) {
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut next: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    chunk = (*boxes).chunks.next;
    while !chunk.is_null() {
        next = (*chunk).next;
        free(chunk as *mut libc::c_void);
        chunk = next;
    }
    let ref mut fresh23 = (*boxes).tail;
    *fresh23 = &mut (*boxes).chunks;
    let ref mut fresh24 = (*boxes).chunks.next;
    *fresh24 = 0 as *mut _cairo_boxes_chunk;
    (*boxes).chunks.count = 0 as libc::c_int;
    let ref mut fresh25 = (*boxes).chunks.base;
    *fresh25 = ((*boxes).boxes_embedded).as_mut_ptr();
    (*boxes)
        .chunks
        .size = (::std::mem::size_of::<[cairo_box_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
        as libc::c_int;
    (*boxes).num_boxes = 0 as libc::c_int;
    (*boxes).is_pixel_aligned = 1 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_to_array(
    mut boxes: *const cairo_boxes_t,
    mut num_boxes: *mut libc::c_int,
) -> *mut cairo_box_t {
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut box_0: *mut cairo_box_t = 0 as *mut cairo_box_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    *num_boxes = (*boxes).num_boxes;
    box_0 = _cairo_malloc_ab(
        (*boxes).num_boxes as size_t,
        ::std::mem::size_of::<cairo_box_t>() as libc::c_ulong,
    ) as *mut cairo_box_t;
    if box_0.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_box_t;
    }
    j = 0 as libc::c_int;
    chunk = &(*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let fresh26 = j;
            j = j + 1;
            *box_0.offset(fresh26 as isize) = *((*chunk).base).offset(i as isize);
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return box_0;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_fini(mut boxes: *mut cairo_boxes_t) {
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut next: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    chunk = (*boxes).chunks.next;
    while !chunk.is_null() {
        next = (*chunk).next;
        free(chunk as *mut libc::c_void);
        chunk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_for_each_box(
    mut boxes: *mut cairo_boxes_t,
    mut func: Option::<
        unsafe extern "C" fn(*mut cairo_box_t, *mut libc::c_void) -> cairo_bool_t,
    >,
    mut data: *mut libc::c_void,
) -> cairo_bool_t {
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            if func
                .expect(
                    "non-null function pointer",
                )(&mut *((*chunk).base).offset(i as isize), data) == 0
            {
                return 0 as libc::c_int;
            }
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn span_to_boxes(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_box_renderer = abstract_renderer as *mut cairo_box_renderer;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    box_0.p1.y = _cairo_fixed_from_int(y);
    box_0.p2.y = _cairo_fixed_from_int(y + h);
    loop {
        if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
            box_0
                .p1
                .x = _cairo_fixed_from_int((*spans.offset(0 as libc::c_int as isize)).x);
            box_0
                .p2
                .x = _cairo_fixed_from_int((*spans.offset(1 as libc::c_int as isize)).x);
            status = _cairo_boxes_add((*r).boxes, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        }
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint
            && status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rasterise_polygon_to_boxes(
    mut polygon: *mut cairo_polygon_t,
    mut fill_rule: cairo_fill_rule_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut renderer: cairo_box_renderer = cairo_box_renderer {
        base: cairo_span_renderer_t {
            status: CAIRO_STATUS_SUCCESS,
            destroy: None,
            render_rows: None,
            finish: None,
        },
        boxes: 0 as *mut cairo_boxes_t,
    };
    let mut converter: *mut cairo_scan_converter_t = 0 as *mut cairo_scan_converter_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
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
    if !(status as u64 != 0) {
        renderer.boxes = boxes;
        renderer
            .base
            .render_rows = Some(
            span_to_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *const cairo_half_open_span_t,
                    libc::c_uint,
                ) -> cairo_status_t,
        );
        status = ((*converter).generate)
            .expect(
                "non-null function pointer",
            )(converter as *mut libc::c_void, &mut renderer.base) as cairo_int_status_t;
    }
    ((*converter).destroy)
        .expect("non-null function pointer")(converter as *mut libc::c_void);
    return status as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_boxes(
    mut stream: *mut FILE,
    mut boxes: *const cairo_boxes_t,
) {
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut extents: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut i: libc::c_int = 0;
    _cairo_boxes_extents(boxes, &mut extents);
    fprintf(
        stream,
        b"boxes x %d: (%f, %f) x (%f, %f)\n\0" as *const u8 as *const libc::c_char,
        (*boxes).num_boxes,
        _cairo_fixed_to_double(extents.p1.x),
        _cairo_fixed_to_double(extents.p1.y),
        _cairo_fixed_to_double(extents.p2.x),
        _cairo_fixed_to_double(extents.p2.y),
    );
    chunk = &(*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            fprintf(
                stderr,
                b"  box[%d]: (%f, %f), (%f, %f)\n\0" as *const u8 as *const libc::c_char,
                i,
                _cairo_fixed_to_double((*((*chunk).base).offset(i as isize)).p1.x),
                _cairo_fixed_to_double((*((*chunk).base).offset(i as isize)).p1.y),
                _cairo_fixed_to_double((*((*chunk).base).offset(i as isize)).p2.x),
                _cairo_fixed_to_double((*((*chunk).base).offset(i as isize)).p2.y),
            );
            i += 1;
        }
        chunk = (*chunk).next;
    }
}
