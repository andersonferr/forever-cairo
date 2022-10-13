use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_region;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_scan_converter_create_in_error(
        error: cairo_status_t,
    ) -> *mut cairo_scan_converter_t;
    fn _cairo_clip_get_polygon(
        clip: *const cairo_clip_t,
        polygon: *mut cairo_polygon_t,
        fill_rule: *mut cairo_fill_rule_t,
        antialias: *mut cairo_antialias_t,
    ) -> cairo_int_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_scan_converter_set_error(
        abstract_converter: *mut libc::c_void,
        error: cairo_status_t,
    ) -> cairo_status_t;
    fn _cairo_polygon_fini(polygon: *mut cairo_polygon_t);
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type uint8_t = __uint8_t;
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
pub type cairo_clip_tor_scan_converter_t = _cairo_clip_tor_scan_converter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip_tor_scan_converter {
    pub base: cairo_scan_converter_t,
    pub converter: [glitter_scan_converter_t; 1],
    pub fill_rule: cairo_fill_rule_t,
    pub antialias: cairo_antialias_t,
    pub clip_fill_rule: cairo_fill_rule_t,
    pub clip_antialias: cairo_antialias_t,
    pub jmp: jmp_buf,
    pub span_pool: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub base: [pool; 1],
    pub embedded: [cairo_half_open_span_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub current: *mut _pool_chunk,
    pub jmp: *mut jmp_buf,
    pub first_free: *mut _pool_chunk,
    pub default_capacity: size_t,
    pub sentinel: [_pool_chunk; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pool_chunk {
    pub size: size_t,
    pub capacity: size_t,
    pub prev_chunk: *mut _pool_chunk,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type glitter_scan_converter_t = glitter_scan_converter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glitter_scan_converter {
    pub polygon: [polygon; 1],
    pub active: [active_list; 1],
    pub coverages: [cell_list; 1],
    pub ymin: grid_scaled_y_t,
    pub ymax: grid_scaled_y_t,
}
pub type grid_scaled_y_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell_list {
    pub head: cell,
    pub tail: cell,
    pub cursor: *mut cell,
    pub cell_pool: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub base: [pool; 1],
    pub embedded: [cell; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub next: *mut cell,
    pub x: libc::c_int,
    pub uncovered_area: grid_area_t,
    pub covered_height: grid_scaled_y_t,
    pub clipped_height: grid_scaled_y_t,
}
pub type grid_area_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct active_list {
    pub head: *mut edge,
    pub min_height: grid_scaled_y_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge {
    pub next: *mut edge,
    pub x: quorem,
    pub dxdy: quorem,
    pub dxdy_full: quorem,
    pub ytop: grid_scaled_y_t,
    pub dy: grid_scaled_y_t,
    pub height_left: grid_scaled_y_t,
    pub dir: libc::c_int,
    pub vertical: libc::c_int,
    pub clip: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quorem {
    pub quo: int32_t,
    pub rem: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polygon {
    pub ymin: grid_scaled_y_t,
    pub ymax: grid_scaled_y_t,
    pub y_buckets: *mut *mut edge,
    pub y_buckets_embedded: [*mut edge; 64],
    pub edge_pool: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub base: [pool; 1],
    pub embedded: [edge; 32],
}
pub type grid_scaled_x_t = libc::c_int;
pub type grid_scaled_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell_pair {
    pub cell1: *mut cell,
    pub cell2: *mut cell,
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
unsafe extern "C" fn floored_divrem(mut a: libc::c_int, mut b: libc::c_int) -> quorem {
    let mut qr: quorem = quorem { quo: 0, rem: 0 };
    qr.quo = a / b;
    qr.rem = a % b;
    if a ^ b < 0 as libc::c_int && qr.rem != 0 {
        qr.quo -= 1 as libc::c_int;
        qr.rem += b;
    }
    return qr;
}
unsafe extern "C" fn floored_muldivrem(
    mut x: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> quorem {
    let mut qr: quorem = quorem { quo: 0, rem: 0 };
    let mut xa: libc::c_longlong = x as libc::c_longlong * a as libc::c_longlong;
    qr.quo = (xa / b as libc::c_longlong) as int32_t;
    qr.rem = (xa % b as libc::c_longlong) as int32_t;
    if (xa >= 0 as libc::c_int as libc::c_longlong) as libc::c_int
        != (b >= 0 as libc::c_int) as libc::c_int && qr.rem != 0
    {
        qr.quo -= 1 as libc::c_int;
        qr.rem += b;
    }
    return qr;
}
unsafe extern "C" fn _pool_chunk_init(
    mut p: *mut _pool_chunk,
    mut prev_chunk: *mut _pool_chunk,
    mut capacity: size_t,
) -> *mut _pool_chunk {
    let ref mut fresh2 = (*p).prev_chunk;
    *fresh2 = prev_chunk;
    (*p).size = 0 as libc::c_int as size_t;
    (*p).capacity = capacity;
    return p;
}
unsafe extern "C" fn _pool_chunk_create(
    mut pool: *mut pool,
    mut size: size_t,
) -> *mut _pool_chunk {
    let mut p: *mut _pool_chunk = 0 as *mut _pool_chunk;
    p = (if size.wrapping_add(::std::mem::size_of::<_pool_chunk>() as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(size.wrapping_add(::std::mem::size_of::<_pool_chunk>() as libc::c_ulong))
    } else {
        0 as *mut libc::c_void
    }) as *mut _pool_chunk;
    if p.is_null() {
        longjmp(
            (*(*pool).jmp).as_mut_ptr(),
            _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
        );
    }
    return _pool_chunk_init(p, (*pool).current, size);
}
unsafe extern "C" fn pool_init(
    mut pool: *mut pool,
    mut jmp: *mut jmp_buf,
    mut default_capacity: size_t,
    mut embedded_capacity: size_t,
) {
    let ref mut fresh3 = (*pool).jmp;
    *fresh3 = jmp;
    let ref mut fresh4 = (*pool).current;
    *fresh4 = ((*pool).sentinel).as_mut_ptr();
    let ref mut fresh5 = (*pool).first_free;
    *fresh5 = 0 as *mut _pool_chunk;
    (*pool).default_capacity = default_capacity;
    _pool_chunk_init(
        ((*pool).sentinel).as_mut_ptr(),
        0 as *mut _pool_chunk,
        embedded_capacity,
    );
}
unsafe extern "C" fn pool_fini(mut pool: *mut pool) {
    let mut p: *mut _pool_chunk = (*pool).current;
    loop {
        while !p.is_null() {
            let mut prev: *mut _pool_chunk = (*p).prev_chunk;
            if p != ((*pool).sentinel).as_mut_ptr() {
                free(p as *mut libc::c_void);
            }
            p = prev;
        }
        p = (*pool).first_free;
        let ref mut fresh6 = (*pool).first_free;
        *fresh6 = 0 as *mut _pool_chunk;
        if p.is_null() {
            break;
        }
    };
}
unsafe extern "C" fn _pool_alloc_from_new_chunk(
    mut pool: *mut pool,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut chunk: *mut _pool_chunk = 0 as *mut _pool_chunk;
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut capacity: size_t = 0;
    capacity = size;
    chunk = 0 as *mut _pool_chunk;
    if size < (*pool).default_capacity {
        capacity = (*pool).default_capacity;
        chunk = (*pool).first_free;
        if !chunk.is_null() {
            let ref mut fresh7 = (*pool).first_free;
            *fresh7 = (*chunk).prev_chunk;
            _pool_chunk_init(chunk, (*pool).current, (*chunk).capacity);
        }
    }
    if chunk.is_null() {
        chunk = _pool_chunk_create(pool, capacity);
    }
    let ref mut fresh8 = (*pool).current;
    *fresh8 = chunk;
    obj = (chunk as *mut libc::c_uchar)
        .offset(::std::mem::size_of::<_pool_chunk>() as libc::c_ulong as isize)
        .offset((*chunk).size as isize) as *mut libc::c_void;
    let ref mut fresh9 = (*chunk).size;
    *fresh9 = (*fresh9 as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    return obj;
}
#[inline]
unsafe extern "C" fn pool_alloc(
    mut pool: *mut pool,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut chunk: *mut _pool_chunk = (*pool).current;
    if size <= ((*chunk).capacity).wrapping_sub((*chunk).size) {
        let mut obj: *mut libc::c_void = (chunk as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<_pool_chunk>() as libc::c_ulong as isize)
            .offset((*chunk).size as isize) as *mut libc::c_void;
        let ref mut fresh10 = (*chunk).size;
        *fresh10 = (*fresh10 as libc::c_ulong).wrapping_add(size) as size_t as size_t;
        return obj;
    } else {
        return _pool_alloc_from_new_chunk(pool, size)
    };
}
unsafe extern "C" fn pool_reset(mut pool: *mut pool) {
    let mut chunk: *mut _pool_chunk = (*pool).current;
    if chunk != ((*pool).sentinel).as_mut_ptr() {
        while (*chunk).prev_chunk != ((*pool).sentinel).as_mut_ptr() {
            chunk = (*chunk).prev_chunk;
        }
        let ref mut fresh11 = (*chunk).prev_chunk;
        *fresh11 = (*pool).first_free;
        let ref mut fresh12 = (*pool).first_free;
        *fresh12 = (*pool).current;
    }
    let ref mut fresh13 = (*pool).current;
    *fresh13 = ((*pool).sentinel).as_mut_ptr();
    (*((*pool).sentinel).as_mut_ptr()).size = 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn cell_list_rewind(mut cells: *mut cell_list) {
    let ref mut fresh14 = (*cells).cursor;
    *fresh14 = &mut (*cells).head;
}
#[inline]
unsafe extern "C" fn cell_list_maybe_rewind(
    mut cells: *mut cell_list,
    mut x: libc::c_int,
) {
    let mut tail: *mut cell = (*cells).cursor;
    if (*tail).x > x {
        cell_list_rewind(cells);
    }
}
unsafe extern "C" fn cell_list_init(mut cells: *mut cell_list, mut jmp: *mut jmp_buf) {
    pool_init(
        ((*cells).cell_pool.base).as_mut_ptr(),
        jmp,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
        ::std::mem::size_of::<[cell; 32]>() as libc::c_ulong,
    );
    let ref mut fresh15 = (*cells).tail.next;
    *fresh15 = 0 as *mut cell;
    (*cells).tail.x = 2147483647 as libc::c_int;
    (*cells).head.x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh16 = (*cells).head.next;
    *fresh16 = &mut (*cells).tail;
    cell_list_rewind(cells);
}
unsafe extern "C" fn cell_list_fini(mut cells: *mut cell_list) {
    pool_fini(((*cells).cell_pool.base).as_mut_ptr());
}
#[inline]
unsafe extern "C" fn cell_list_reset(mut cells: *mut cell_list) {
    cell_list_rewind(cells);
    let ref mut fresh17 = (*cells).head.next;
    *fresh17 = &mut (*cells).tail;
    pool_reset(((*cells).cell_pool.base).as_mut_ptr());
}
unsafe extern "C" fn cell_list_alloc(
    mut cells: *mut cell_list,
    mut tail: *mut cell,
    mut x: libc::c_int,
) -> *mut cell {
    let mut cell: *mut cell = 0 as *mut cell;
    cell = pool_alloc(
        ((*cells).cell_pool.base).as_mut_ptr(),
        ::std::mem::size_of::<cell>() as libc::c_ulong,
    ) as *mut cell;
    let ref mut fresh18 = (*cell).next;
    *fresh18 = (*tail).next;
    let ref mut fresh19 = (*tail).next;
    *fresh19 = cell;
    (*cell).x = x;
    (*cell).uncovered_area = 0 as libc::c_int;
    (*cell).covered_height = 0 as libc::c_int;
    (*cell).clipped_height = 0 as libc::c_int;
    return cell;
}
#[inline]
unsafe extern "C" fn cell_list_find(
    mut cells: *mut cell_list,
    mut x: libc::c_int,
) -> *mut cell {
    let mut tail: *mut cell = (*cells).cursor;
    while !((*(*tail).next).x > x) {
        tail = (*tail).next;
        if (*(*tail).next).x > x {
            break;
        }
        tail = (*tail).next;
        if (*(*tail).next).x > x {
            break;
        }
        tail = (*tail).next;
    }
    if (*tail).x != x {
        tail = cell_list_alloc(cells, tail, x);
    }
    let ref mut fresh20 = (*cells).cursor;
    *fresh20 = tail;
    return *fresh20;
}
#[inline]
unsafe extern "C" fn cell_list_find_pair(
    mut cells: *mut cell_list,
    mut x1: libc::c_int,
    mut x2: libc::c_int,
) -> cell_pair {
    let mut pair: cell_pair = cell_pair {
        cell1: 0 as *mut cell,
        cell2: 0 as *mut cell,
    };
    pair.cell1 = (*cells).cursor;
    while !((*(*pair.cell1).next).x > x1) {
        pair.cell1 = (*pair.cell1).next;
        if (*(*pair.cell1).next).x > x1 {
            break;
        }
        pair.cell1 = (*pair.cell1).next;
        if (*(*pair.cell1).next).x > x1 {
            break;
        }
        pair.cell1 = (*pair.cell1).next;
    }
    if (*pair.cell1).x != x1 {
        let mut cell: *mut cell = pool_alloc(
            ((*cells).cell_pool.base).as_mut_ptr(),
            ::std::mem::size_of::<cell>() as libc::c_ulong,
        ) as *mut cell;
        (*cell).x = x1;
        (*cell).uncovered_area = 0 as libc::c_int;
        (*cell).covered_height = 0 as libc::c_int;
        (*cell).clipped_height = 0 as libc::c_int;
        let ref mut fresh21 = (*cell).next;
        *fresh21 = (*pair.cell1).next;
        let ref mut fresh22 = (*pair.cell1).next;
        *fresh22 = cell;
        pair.cell1 = cell;
    }
    pair.cell2 = pair.cell1;
    while !((*(*pair.cell2).next).x > x2) {
        pair.cell2 = (*pair.cell2).next;
        if (*(*pair.cell2).next).x > x2 {
            break;
        }
        pair.cell2 = (*pair.cell2).next;
        if (*(*pair.cell2).next).x > x2 {
            break;
        }
        pair.cell2 = (*pair.cell2).next;
    }
    if (*pair.cell2).x != x2 {
        let mut cell_0: *mut cell = pool_alloc(
            ((*cells).cell_pool.base).as_mut_ptr(),
            ::std::mem::size_of::<cell>() as libc::c_ulong,
        ) as *mut cell;
        (*cell_0).uncovered_area = 0 as libc::c_int;
        (*cell_0).covered_height = 0 as libc::c_int;
        (*cell_0).clipped_height = 0 as libc::c_int;
        (*cell_0).x = x2;
        let ref mut fresh23 = (*cell_0).next;
        *fresh23 = (*pair.cell2).next;
        let ref mut fresh24 = (*pair.cell2).next;
        *fresh24 = cell_0;
        pair.cell2 = cell_0;
    }
    let ref mut fresh25 = (*cells).cursor;
    *fresh25 = pair.cell2;
    return pair;
}
#[inline]
unsafe extern "C" fn cell_list_add_subspan(
    mut cells: *mut cell_list,
    mut x1: grid_scaled_x_t,
    mut x2: grid_scaled_x_t,
) {
    let mut ix1: libc::c_int = 0;
    let mut fx1: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut fx2: libc::c_int = 0;
    fx1 = x1 & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    ix1 = x1 >> 8 as libc::c_int;
    fx2 = x2 & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    ix2 = x2 >> 8 as libc::c_int;
    if ix1 != ix2 {
        let mut p: cell_pair = cell_pair {
            cell1: 0 as *mut cell,
            cell2: 0 as *mut cell,
        };
        p = cell_list_find_pair(cells, ix1, ix2);
        let ref mut fresh26 = (*p.cell1).uncovered_area;
        *fresh26 += 2 as libc::c_int * fx1;
        let ref mut fresh27 = (*p.cell1).covered_height;
        *fresh27 += 1;
        let ref mut fresh28 = (*p.cell2).uncovered_area;
        *fresh28 -= 2 as libc::c_int * fx2;
        let ref mut fresh29 = (*p.cell2).covered_height;
        *fresh29 -= 1;
    } else {
        let mut cell: *mut cell = cell_list_find(cells, ix1);
        let ref mut fresh30 = (*cell).uncovered_area;
        *fresh30 += 2 as libc::c_int * (fx1 - fx2);
    };
}
unsafe extern "C" fn cell_list_render_edge(
    mut cells: *mut cell_list,
    mut edge: *mut edge,
    mut sign: libc::c_int,
) {
    let mut y1: grid_scaled_y_t = 0;
    let mut y2: grid_scaled_y_t = 0;
    let mut dy: grid_scaled_y_t = 0;
    let mut dx: grid_scaled_x_t = 0;
    let mut ix1: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut fx1: grid_scaled_x_t = 0;
    let mut fx2: grid_scaled_x_t = 0;
    let mut x1: quorem = (*edge).x;
    let mut x2: quorem = x1;
    if (*edge).vertical == 0 {
        x2.quo += (*edge).dxdy_full.quo;
        x2.rem += (*edge).dxdy_full.rem;
        if x2.rem >= 0 as libc::c_int {
            x2.quo += 1;
            x2.rem -= (*edge).dy;
        }
        (*edge).x = x2;
    }
    fx1 = x1.quo & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    ix1 = x1.quo >> 8 as libc::c_int;
    fx2 = x2.quo & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    ix2 = x2.quo >> 8 as libc::c_int;
    if ix1 == ix2 {
        let mut cell: *mut cell = cell_list_find(cells, ix1);
        let ref mut fresh31 = (*cell).covered_height;
        *fresh31 += sign * 15 as libc::c_int;
        let ref mut fresh32 = (*cell).uncovered_area;
        *fresh32 += sign * (fx1 + fx2) * 15 as libc::c_int;
        return;
    }
    dx = x2.quo - x1.quo;
    if dx >= 0 as libc::c_int {
        y1 = 0 as libc::c_int;
        y2 = 15 as libc::c_int;
    } else {
        let mut tmp: libc::c_int = 0;
        tmp = ix1;
        ix1 = ix2;
        ix2 = tmp;
        tmp = fx1;
        fx1 = fx2;
        fx2 = tmp;
        dx = -dx;
        sign = -sign;
        y1 = 15 as libc::c_int;
        y2 = 0 as libc::c_int;
    }
    dy = y2 - y1;
    let mut pair: cell_pair = cell_pair {
        cell1: 0 as *mut cell,
        cell2: 0 as *mut cell,
    };
    let mut y: quorem = floored_divrem(
        (((1 as libc::c_int) << 8 as libc::c_int) - fx1) * dy,
        dx,
    );
    cell_list_maybe_rewind(cells, ix1);
    pair = cell_list_find_pair(cells, ix1, ix1 + 1 as libc::c_int);
    let ref mut fresh33 = (*pair.cell1).uncovered_area;
    *fresh33 += sign * y.quo * (((1 as libc::c_int) << 8 as libc::c_int) + fx1);
    let ref mut fresh34 = (*pair.cell1).covered_height;
    *fresh34 += sign * y.quo;
    y.quo += y1;
    if (ix1 + 1 as libc::c_int) < ix2 {
        let mut dydx_full: quorem = floored_divrem(
            ((1 as libc::c_int) << 8 as libc::c_int) * dy,
            dx,
        );
        let mut cell_0: *mut cell = pair.cell2;
        ix1 += 1;
        loop {
            let mut y_skip: grid_scaled_y_t = dydx_full.quo;
            y.rem += dydx_full.rem;
            if y.rem >= dx {
                y_skip += 1;
                y.rem -= dx;
            }
            y.quo += y_skip;
            y_skip *= sign;
            let ref mut fresh35 = (*cell_0).uncovered_area;
            *fresh35 += y_skip * ((1 as libc::c_int) << 8 as libc::c_int);
            let ref mut fresh36 = (*cell_0).covered_height;
            *fresh36 += y_skip;
            ix1 += 1;
            cell_0 = cell_list_find(cells, ix1);
            if !(ix1 != ix2) {
                break;
            }
        }
        pair.cell2 = cell_0;
    }
    let ref mut fresh37 = (*pair.cell2).uncovered_area;
    *fresh37 += sign * (y2 - y.quo) * fx2;
    let ref mut fresh38 = (*pair.cell2).covered_height;
    *fresh38 += sign * (y2 - y.quo);
}
unsafe extern "C" fn polygon_init(mut polygon: *mut polygon, mut jmp: *mut jmp_buf) {
    let ref mut fresh39 = (*polygon).ymax;
    *fresh39 = 0 as libc::c_int;
    (*polygon).ymin = *fresh39;
    let ref mut fresh40 = (*polygon).y_buckets;
    *fresh40 = ((*polygon).y_buckets_embedded).as_mut_ptr();
    pool_init(
        ((*polygon).edge_pool.base).as_mut_ptr(),
        jmp,
        (8192 as libc::c_int as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<_pool_chunk>() as libc::c_ulong),
        ::std::mem::size_of::<[edge; 32]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn polygon_fini(mut polygon: *mut polygon) {
    if (*polygon).y_buckets != ((*polygon).y_buckets_embedded).as_mut_ptr() {
        free((*polygon).y_buckets as *mut libc::c_void);
    }
    pool_fini(((*polygon).edge_pool.base).as_mut_ptr());
}
unsafe extern "C" fn polygon_reset(
    mut polygon: *mut polygon,
    mut ymin: grid_scaled_y_t,
    mut ymax: grid_scaled_y_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut h: libc::c_uint = (ymax - ymin) as libc::c_uint;
    let mut num_buckets: libc::c_uint = ((ymax + 15 as libc::c_int - 1 as libc::c_int
        - ymin) / 15 as libc::c_int) as libc::c_uint;
    pool_reset(((*polygon).edge_pool.base).as_mut_ptr());
    if !(h
        > (0x7fffffff as libc::c_uint).wrapping_sub(15 as libc::c_int as libc::c_uint))
    {
        if (*polygon).y_buckets != ((*polygon).y_buckets_embedded).as_mut_ptr() {
            free((*polygon).y_buckets as *mut libc::c_void);
        }
        let ref mut fresh41 = (*polygon).y_buckets;
        *fresh41 = ((*polygon).y_buckets_embedded).as_mut_ptr();
        if num_buckets
            > (::std::mem::size_of::<[*mut edge; 64]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut edge>() as libc::c_ulong)
                as libc::c_int as libc::c_uint
        {
            let ref mut fresh42 = (*polygon).y_buckets;
            *fresh42 = _cairo_malloc_ab(
                num_buckets as size_t,
                ::std::mem::size_of::<*mut edge>() as libc::c_ulong,
            ) as *mut *mut edge;
            if ((*polygon).y_buckets).is_null() {
                current_block = 1799367245860766922;
            } else {
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            1799367245860766922 => {}
            _ => {
                memset(
                    (*polygon).y_buckets as *mut libc::c_void,
                    0 as libc::c_int,
                    (num_buckets as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut edge>() as libc::c_ulong,
                        ),
                );
                (*polygon).ymin = ymin;
                (*polygon).ymax = ymax;
                return CAIRO_STATUS_SUCCESS;
            }
        }
    }
    (*polygon).ymin = 0 as libc::c_int;
    (*polygon).ymax = 0 as libc::c_int;
    return CAIRO_STATUS_NO_MEMORY;
}
unsafe extern "C" fn _polygon_insert_edge_into_its_y_bucket(
    mut polygon: *mut polygon,
    mut e: *mut edge,
) {
    let mut ix: libc::c_uint = (((*e).ytop - (*polygon).ymin) / 15 as libc::c_int)
        as libc::c_uint;
    let mut ptail: *mut *mut edge = &mut *((*polygon).y_buckets).offset(ix as isize)
        as *mut *mut edge;
    let ref mut fresh43 = (*e).next;
    *fresh43 = *ptail;
    *ptail = e;
}
#[inline]
unsafe extern "C" fn polygon_add_edge(
    mut polygon: *mut polygon,
    mut edge: *const cairo_edge_t,
    mut clip: libc::c_int,
) {
    let mut e: *mut edge = 0 as *mut edge;
    let mut dx: grid_scaled_x_t = 0;
    let mut dy: grid_scaled_y_t = 0;
    let mut ytop: grid_scaled_y_t = 0;
    let mut ybot: grid_scaled_y_t = 0;
    let mut ymin: grid_scaled_y_t = (*polygon).ymin;
    let mut ymax: grid_scaled_y_t = (*polygon).ymax;
    if (*edge).bottom > (*edge).top {} else {
        __assert_fail(
            b"edge->bottom > edge->top\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-clip-tor-scan-converter.c\0" as *const u8
                as *const libc::c_char,
            959 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void polygon_add_edge(struct polygon *, const cairo_edge_t *, int)\0"))
                .as_ptr(),
        );
    }
    if (*edge).top >= ymax || (*edge).bottom <= ymin {
        return;
    }
    e = pool_alloc(
        ((*polygon).edge_pool.base).as_mut_ptr(),
        ::std::mem::size_of::<edge>() as libc::c_ulong,
    ) as *mut edge;
    dx = (*edge).line.p2.x - (*edge).line.p1.x;
    dy = (*edge).line.p2.y - (*edge).line.p1.y;
    (*e).dy = dy;
    (*e).dir = (*edge).dir;
    (*e).clip = clip;
    ytop = if (*edge).top >= ymin { (*edge).top } else { ymin };
    ybot = if (*edge).bottom <= ymax { (*edge).bottom } else { ymax };
    (*e).ytop = ytop;
    (*e).height_left = ybot - ytop;
    if dx == 0 as libc::c_int {
        (*e).vertical = 1 as libc::c_int;
        (*e).x.quo = (*edge).line.p1.x;
        (*e).x.rem = 0 as libc::c_int;
        (*e).dxdy.quo = 0 as libc::c_int;
        (*e).dxdy.rem = 0 as libc::c_int;
        (*e).dxdy_full.quo = 0 as libc::c_int;
        (*e).dxdy_full.rem = 0 as libc::c_int;
    } else {
        (*e).vertical = 0 as libc::c_int;
        (*e).dxdy = floored_divrem(dx, dy);
        if ytop == (*edge).line.p1.y {
            (*e).x.quo = (*edge).line.p1.x;
            (*e).x.rem = 0 as libc::c_int;
        } else {
            (*e).x = floored_muldivrem(ytop - (*edge).line.p1.y, dx, dy);
            let ref mut fresh44 = (*e).x.quo;
            *fresh44 += (*edge).line.p1.x;
        }
        if (*e).height_left >= 15 as libc::c_int {
            (*e).dxdy_full = floored_muldivrem(15 as libc::c_int, dx, dy);
        } else {
            (*e).dxdy_full.quo = 0 as libc::c_int;
            (*e).dxdy_full.rem = 0 as libc::c_int;
        }
    }
    _polygon_insert_edge_into_its_y_bucket(polygon, e);
    let ref mut fresh45 = (*e).x.rem;
    *fresh45 -= dy;
}
unsafe extern "C" fn active_list_reset(mut active: *mut active_list) {
    let ref mut fresh46 = (*active).head;
    *fresh46 = 0 as *mut edge;
    (*active).min_height = 0 as libc::c_int;
}
unsafe extern "C" fn active_list_init(mut active: *mut active_list) {
    active_list_reset(active);
}
unsafe extern "C" fn merge_sorted_edges(
    mut head_a: *mut edge,
    mut head_b: *mut edge,
) -> *mut edge {
    let mut current_block: u64;
    let mut head: *mut edge = 0 as *mut edge;
    let mut next: *mut *mut edge = 0 as *mut *mut edge;
    let mut x: int32_t = 0;
    if head_a.is_null() {
        return head_b;
    }
    next = &mut head;
    if (*head_a).x.quo <= (*head_b).x.quo {
        head = head_a;
        current_block = 13513818773234778473;
    } else {
        head = head_b;
        current_block = 5999371867469101635;
    }
    loop {
        match current_block {
            5999371867469101635 => {
                x = (*head_a).x.quo;
                while !head_b.is_null() && (*head_b).x.quo <= x {
                    next = &mut (*head_b).next;
                    head_b = (*head_b).next;
                }
                *next = head_a;
                if head_b.is_null() {
                    return head;
                }
                current_block = 13513818773234778473;
            }
            _ => {
                x = (*head_b).x.quo;
                while !head_a.is_null() && (*head_a).x.quo <= x {
                    next = &mut (*head_a).next;
                    head_a = (*head_a).next;
                }
                *next = head_b;
                if head_a.is_null() {
                    return head;
                }
                current_block = 5999371867469101635;
            }
        }
    };
}
unsafe extern "C" fn sort_edges(
    mut list: *mut edge,
    mut level: libc::c_uint,
    mut head_out: *mut *mut edge,
) -> *mut edge {
    let mut head_other: *mut edge = 0 as *mut edge;
    let mut remaining: *mut edge = 0 as *mut edge;
    let mut i: libc::c_uint = 0;
    head_other = (*list).next;
    if head_other.is_null() {
        *head_out = list;
        return 0 as *mut edge;
    }
    remaining = (*head_other).next;
    if (*list).x.quo <= (*head_other).x.quo {
        *head_out = list;
        let ref mut fresh47 = (*head_other).next;
        *fresh47 = 0 as *mut edge;
    } else {
        *head_out = head_other;
        let ref mut fresh48 = (*head_other).next;
        *fresh48 = list;
        let ref mut fresh49 = (*list).next;
        *fresh49 = 0 as *mut edge;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < level && !remaining.is_null() {
        remaining = sort_edges(remaining, i, &mut head_other);
        *head_out = merge_sorted_edges(*head_out, head_other);
        i = i.wrapping_add(1);
    }
    return remaining;
}
#[inline]
unsafe extern "C" fn active_list_can_step_full_row(
    mut active: *mut active_list,
) -> libc::c_int {
    let mut e: *const edge = 0 as *const edge;
    let mut prev_x: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    if (*active).min_height <= 0 as libc::c_int {
        let mut min_height: libc::c_int = 2147483647 as libc::c_int;
        e = (*active).head;
        while !e.is_null() {
            if (*e).height_left < min_height {
                min_height = (*e).height_left;
            }
            e = (*e).next;
        }
        (*active).min_height = min_height;
    }
    if (*active).min_height < 15 as libc::c_int {
        return 0 as libc::c_int;
    }
    e = (*active).head;
    while !e.is_null() {
        let mut x: quorem = (*e).x;
        if (*e).vertical == 0 {
            x.quo += (*e).dxdy_full.quo;
            x.rem += (*e).dxdy_full.rem;
            if x.rem >= 0 as libc::c_int {
                x.quo += 1;
            }
        }
        if x.quo <= prev_x {
            return 0 as libc::c_int;
        }
        prev_x = x.quo;
        e = (*e).next;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn active_list_merge_edges_from_polygon(
    mut active: *mut active_list,
    mut ptail: *mut *mut edge,
    mut y: grid_scaled_y_t,
    mut polygon: *mut polygon,
) {
    let mut min_height: libc::c_int = (*active).min_height;
    let mut subrow_edges: *mut edge = 0 as *mut edge;
    let mut tail: *mut edge = *ptail;
    loop {
        let mut next: *mut edge = (*tail).next;
        if y == (*tail).ytop {
            let ref mut fresh50 = (*tail).next;
            *fresh50 = subrow_edges;
            subrow_edges = tail;
            if (*tail).height_left < min_height {
                min_height = (*tail).height_left;
            }
            *ptail = next;
        } else {
            ptail = &mut (*tail).next;
        }
        tail = next;
        if tail.is_null() {
            break;
        }
    }
    if !subrow_edges.is_null() {
        sort_edges(
            subrow_edges,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
            &mut subrow_edges,
        );
        let ref mut fresh51 = (*active).head;
        *fresh51 = merge_sorted_edges((*active).head, subrow_edges);
        (*active).min_height = min_height;
    }
}
#[inline]
unsafe extern "C" fn active_list_substep_edges(mut active: *mut active_list) {
    let mut cursor: *mut *mut edge = &mut (*active).head;
    let mut prev_x: grid_scaled_x_t = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut unsorted: *mut edge = 0 as *mut edge;
    let mut edge: *mut edge = *cursor;
    loop {
        let mut next: *mut edge = 0 as *mut edge;
        if edge.is_null() {
            break;
        }
        next = (*edge).next;
        let ref mut fresh52 = (*edge).height_left;
        *fresh52 -= 1;
        if *fresh52 != 0 {
            let ref mut fresh53 = (*edge).x.quo;
            *fresh53 += (*edge).dxdy.quo;
            let ref mut fresh54 = (*edge).x.rem;
            *fresh54 += (*edge).dxdy.rem;
            if (*edge).x.rem >= 0 as libc::c_int {
                let ref mut fresh55 = (*edge).x.quo;
                *fresh55 += 1;
                let ref mut fresh56 = (*edge).x.rem;
                *fresh56 -= (*edge).dy;
            }
            if (*edge).x.quo < prev_x {
                *cursor = next;
                let ref mut fresh57 = (*edge).next;
                *fresh57 = unsorted;
                unsorted = edge;
            } else {
                prev_x = (*edge).x.quo;
                cursor = &mut (*edge).next;
            }
        } else {
            *cursor = next;
        }
        edge = next;
        let mut next_0: *mut edge = 0 as *mut edge;
        if edge.is_null() {
            break;
        }
        next_0 = (*edge).next;
        let ref mut fresh58 = (*edge).height_left;
        *fresh58 -= 1;
        if *fresh58 != 0 {
            let ref mut fresh59 = (*edge).x.quo;
            *fresh59 += (*edge).dxdy.quo;
            let ref mut fresh60 = (*edge).x.rem;
            *fresh60 += (*edge).dxdy.rem;
            if (*edge).x.rem >= 0 as libc::c_int {
                let ref mut fresh61 = (*edge).x.quo;
                *fresh61 += 1;
                let ref mut fresh62 = (*edge).x.rem;
                *fresh62 -= (*edge).dy;
            }
            if (*edge).x.quo < prev_x {
                *cursor = next_0;
                let ref mut fresh63 = (*edge).next;
                *fresh63 = unsorted;
                unsorted = edge;
            } else {
                prev_x = (*edge).x.quo;
                cursor = &mut (*edge).next;
            }
        } else {
            *cursor = next_0;
        }
        edge = next_0;
        let mut next_1: *mut edge = 0 as *mut edge;
        if edge.is_null() {
            break;
        }
        next_1 = (*edge).next;
        let ref mut fresh64 = (*edge).height_left;
        *fresh64 -= 1;
        if *fresh64 != 0 {
            let ref mut fresh65 = (*edge).x.quo;
            *fresh65 += (*edge).dxdy.quo;
            let ref mut fresh66 = (*edge).x.rem;
            *fresh66 += (*edge).dxdy.rem;
            if (*edge).x.rem >= 0 as libc::c_int {
                let ref mut fresh67 = (*edge).x.quo;
                *fresh67 += 1;
                let ref mut fresh68 = (*edge).x.rem;
                *fresh68 -= (*edge).dy;
            }
            if (*edge).x.quo < prev_x {
                *cursor = next_1;
                let ref mut fresh69 = (*edge).next;
                *fresh69 = unsorted;
                unsorted = edge;
            } else {
                prev_x = (*edge).x.quo;
                cursor = &mut (*edge).next;
            }
        } else {
            *cursor = next_1;
        }
        edge = next_1;
    }
    if !unsorted.is_null() {
        sort_edges(
            unsorted,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
            &mut unsorted,
        );
        let ref mut fresh70 = (*active).head;
        *fresh70 = merge_sorted_edges((*active).head, unsorted);
    }
}
#[inline]
unsafe extern "C" fn apply_nonzero_fill_rule_for_subrow(
    mut active: *mut active_list,
    mut coverages: *mut cell_list,
) {
    let mut edge: *mut edge = (*active).head;
    let mut winding: libc::c_int = 0 as libc::c_int;
    let mut xstart: libc::c_int = 0;
    let mut xend: libc::c_int = 0;
    cell_list_rewind(coverages);
    while !edge.is_null() {
        xstart = (*edge).x.quo;
        winding = (*edge).dir;
        loop {
            edge = (*edge).next;
            if edge.is_null() {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-clip-tor-scan-converter.c\0" as *const u8
                            as *const libc::c_char,
                        1292 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 82],
                            &[libc::c_char; 82],
                        >(
                            b"void apply_nonzero_fill_rule_for_subrow(struct active_list *, struct cell_list *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                return;
            }
            winding += (*edge).dir;
            if !(0 as libc::c_int == winding) {
                continue;
            }
            if ((*edge).next).is_null() || (*(*edge).next).x.quo != (*edge).x.quo {
                break;
            }
        }
        xend = (*edge).x.quo;
        cell_list_add_subspan(coverages, xstart, xend);
        edge = (*edge).next;
    }
}
unsafe extern "C" fn apply_evenodd_fill_rule_for_subrow(
    mut active: *mut active_list,
    mut coverages: *mut cell_list,
) {
    let mut edge: *mut edge = (*active).head;
    let mut xstart: libc::c_int = 0;
    let mut xend: libc::c_int = 0;
    cell_list_rewind(coverages);
    while !edge.is_null() {
        xstart = (*edge).x.quo;
        loop {
            edge = (*edge).next;
            if edge.is_null() {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-clip-tor-scan-converter.c\0" as *const u8
                            as *const libc::c_char,
                        1326 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 82],
                            &[libc::c_char; 82],
                        >(
                            b"void apply_evenodd_fill_rule_for_subrow(struct active_list *, struct cell_list *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                return;
            }
            if ((*edge).next).is_null() || (*(*edge).next).x.quo != (*edge).x.quo {
                break;
            }
            edge = (*edge).next;
        }
        xend = (*edge).x.quo;
        cell_list_add_subspan(coverages, xstart, xend);
        edge = (*edge).next;
    }
}
unsafe extern "C" fn apply_nonzero_fill_rule_and_step_edges(
    mut active: *mut active_list,
    mut coverages: *mut cell_list,
) {
    let mut cursor: *mut *mut edge = &mut (*active).head;
    let mut left_edge: *mut edge = 0 as *mut edge;
    left_edge = *cursor;
    while !left_edge.is_null() {
        let mut right_edge: *mut edge = 0 as *mut edge;
        let mut winding: libc::c_int = (*left_edge).dir;
        let ref mut fresh71 = (*left_edge).height_left;
        *fresh71 -= 15 as libc::c_int;
        if (*left_edge).height_left != 0 {
            cursor = &mut (*left_edge).next;
        } else {
            *cursor = (*left_edge).next;
        }
        loop {
            right_edge = *cursor;
            if right_edge.is_null() {
                cell_list_render_edge(coverages, left_edge, 1 as libc::c_int);
                return;
            }
            let ref mut fresh72 = (*right_edge).height_left;
            *fresh72 -= 15 as libc::c_int;
            if (*right_edge).height_left != 0 {
                cursor = &mut (*right_edge).next;
            } else {
                *cursor = (*right_edge).next;
            }
            winding += (*right_edge).dir;
            if 0 as libc::c_int == winding {
                if ((*right_edge).next).is_null()
                    || (*(*right_edge).next).x.quo != (*right_edge).x.quo
                {
                    break;
                }
            }
            if (*right_edge).vertical == 0 {
                let ref mut fresh73 = (*right_edge).x.quo;
                *fresh73 += (*right_edge).dxdy_full.quo;
                let ref mut fresh74 = (*right_edge).x.rem;
                *fresh74 += (*right_edge).dxdy_full.rem;
                if (*right_edge).x.rem >= 0 as libc::c_int {
                    let ref mut fresh75 = (*right_edge).x.quo;
                    *fresh75 += 1;
                    let ref mut fresh76 = (*right_edge).x.rem;
                    *fresh76 -= (*right_edge).dy;
                }
            }
        }
        cell_list_render_edge(coverages, left_edge, 1 as libc::c_int);
        cell_list_render_edge(coverages, right_edge, -(1 as libc::c_int));
        left_edge = *cursor;
    }
}
unsafe extern "C" fn apply_evenodd_fill_rule_and_step_edges(
    mut active: *mut active_list,
    mut coverages: *mut cell_list,
) {
    let mut cursor: *mut *mut edge = &mut (*active).head;
    let mut left_edge: *mut edge = 0 as *mut edge;
    left_edge = *cursor;
    while !left_edge.is_null() {
        let mut right_edge: *mut edge = 0 as *mut edge;
        let ref mut fresh77 = (*left_edge).height_left;
        *fresh77 -= 15 as libc::c_int;
        if (*left_edge).height_left != 0 {
            cursor = &mut (*left_edge).next;
        } else {
            *cursor = (*left_edge).next;
        }
        loop {
            right_edge = *cursor;
            if right_edge.is_null() {
                cell_list_render_edge(coverages, left_edge, 1 as libc::c_int);
                return;
            }
            let ref mut fresh78 = (*right_edge).height_left;
            *fresh78 -= 15 as libc::c_int;
            if (*right_edge).height_left != 0 {
                cursor = &mut (*right_edge).next;
            } else {
                *cursor = (*right_edge).next;
            }
            if ((*right_edge).next).is_null()
                || (*(*right_edge).next).x.quo != (*right_edge).x.quo
            {
                break;
            }
            if (*right_edge).vertical == 0 {
                let ref mut fresh79 = (*right_edge).x.quo;
                *fresh79 += (*right_edge).dxdy_full.quo;
                let ref mut fresh80 = (*right_edge).x.rem;
                *fresh80 += (*right_edge).dxdy_full.rem;
                if (*right_edge).x.rem >= 0 as libc::c_int {
                    let ref mut fresh81 = (*right_edge).x.quo;
                    *fresh81 += 1;
                    let ref mut fresh82 = (*right_edge).x.rem;
                    *fresh82 -= (*right_edge).dy;
                }
            }
        }
        cell_list_render_edge(coverages, left_edge, 1 as libc::c_int);
        cell_list_render_edge(coverages, right_edge, -(1 as libc::c_int));
        left_edge = *cursor;
    }
}
unsafe extern "C" fn _glitter_scan_converter_init(
    mut converter: *mut glitter_scan_converter_t,
    mut jmp: *mut jmp_buf,
) {
    polygon_init(((*converter).polygon).as_mut_ptr(), jmp);
    active_list_init(((*converter).active).as_mut_ptr());
    cell_list_init(((*converter).coverages).as_mut_ptr(), jmp);
    (*converter).ymin = 0 as libc::c_int;
    (*converter).ymax = 0 as libc::c_int;
}
unsafe extern "C" fn _glitter_scan_converter_fini(
    mut converter: *mut glitter_scan_converter_t,
) {
    polygon_fini(((*converter).polygon).as_mut_ptr());
    cell_list_fini(((*converter).coverages).as_mut_ptr());
    (*converter).ymin = 0 as libc::c_int;
    (*converter).ymax = 0 as libc::c_int;
}
unsafe extern "C" fn int_to_grid_scaled(
    mut i: libc::c_int,
    mut scale: libc::c_int,
) -> grid_scaled_t {
    if i >= 0 as libc::c_int {
        if i >= 2147483647 as libc::c_int / scale {
            i = 2147483647 as libc::c_int / scale;
        }
    } else if i <= (-(2147483647 as libc::c_int) - 1 as libc::c_int) / scale {
        i = (-(2147483647 as libc::c_int) - 1 as libc::c_int) / scale;
    }
    return i * scale;
}
unsafe extern "C" fn glitter_scan_converter_reset(
    mut converter: *mut glitter_scan_converter_t,
    mut ymin: libc::c_int,
    mut ymax: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*converter).ymin = 0 as libc::c_int;
    (*converter).ymax = 0 as libc::c_int;
    ymin = int_to_grid_scaled(ymin, 15 as libc::c_int);
    ymax = int_to_grid_scaled(ymax, 15 as libc::c_int);
    active_list_reset(((*converter).active).as_mut_ptr());
    cell_list_reset(((*converter).coverages).as_mut_ptr());
    status = polygon_reset(((*converter).polygon).as_mut_ptr(), ymin, ymax);
    if status as u64 != 0 {
        return status;
    }
    (*converter).ymin = ymin;
    (*converter).ymax = ymax;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn glitter_scan_converter_add_edge(
    mut converter: *mut glitter_scan_converter_t,
    mut edge: *const cairo_edge_t,
    mut clip: libc::c_int,
) {
    let mut e: cairo_edge_t = cairo_edge_t {
        line: cairo_line_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        top: 0,
        bottom: 0,
        dir: 0,
    };
    let mut tmp__: libc::c_longlong = 15 as libc::c_int as libc::c_longlong
        * (*edge).top as libc::c_longlong;
    tmp__ >>= 8 as libc::c_int;
    e.top = tmp__ as libc::c_int;
    let mut tmp___0: libc::c_longlong = 15 as libc::c_int as libc::c_longlong
        * (*edge).bottom as libc::c_longlong;
    tmp___0 >>= 8 as libc::c_int;
    e.bottom = tmp___0 as libc::c_int;
    if e.top >= e.bottom {
        return;
    }
    let mut tmp___1: libc::c_longlong = 15 as libc::c_int as libc::c_longlong
        * (*edge).line.p1.y as libc::c_longlong;
    tmp___1 >>= 8 as libc::c_int;
    e.line.p1.y = tmp___1 as cairo_fixed_t;
    let mut tmp___2: libc::c_longlong = 15 as libc::c_int as libc::c_longlong
        * (*edge).line.p2.y as libc::c_longlong;
    tmp___2 >>= 8 as libc::c_int;
    e.line.p2.y = tmp___2 as cairo_fixed_t;
    if e.line.p1.y == e.line.p2.y {
        return;
    }
    e.line.p1.x = (*edge).line.p1.x >> 8 as libc::c_int - 8 as libc::c_int;
    e.line.p2.x = (*edge).line.p2.x >> 8 as libc::c_int - 8 as libc::c_int;
    e.dir = (*edge).dir;
    polygon_add_edge(((*converter).polygon).as_mut_ptr(), &mut e, clip);
}
unsafe extern "C" fn active_list_is_vertical(
    mut active: *mut active_list,
) -> cairo_bool_t {
    let mut e: *mut edge = 0 as *mut edge;
    e = (*active).head;
    while !e.is_null() {
        if (*e).vertical == 0 {
            return 0 as libc::c_int;
        }
        e = (*e).next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn step_edges(mut active: *mut active_list, mut count: libc::c_int) {
    let mut cursor: *mut *mut edge = &mut (*active).head;
    let mut edge: *mut edge = 0 as *mut edge;
    edge = *cursor;
    while !edge.is_null() {
        let ref mut fresh83 = (*edge).height_left;
        *fresh83 -= 15 as libc::c_int * count;
        if (*edge).height_left != 0 {
            cursor = &mut (*edge).next;
        } else {
            *cursor = (*edge).next;
        }
        edge = *cursor;
    }
}
unsafe extern "C" fn blit_coverages(
    mut cells: *mut cell_list,
    mut renderer: *mut cairo_span_renderer_t,
    mut span_pool: *mut pool,
    mut y: libc::c_int,
    mut height: libc::c_int,
) -> cairo_status_t {
    let mut cell: *mut cell = (*cells).head.next;
    let mut prev_x: libc::c_int = -(1 as libc::c_int);
    let mut cover: libc::c_int = 0 as libc::c_int;
    let mut last_cover: libc::c_int = 0 as libc::c_int;
    let mut clip: libc::c_int = 0 as libc::c_int;
    let mut spans: *mut cairo_half_open_span_t = 0 as *mut cairo_half_open_span_t;
    let mut num_spans: libc::c_uint = 0;
    if cell != &mut (*cells).tail as *mut cell {} else {
        __assert_fail(
            b"cell != &cells->tail\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-clip-tor-scan-converter.c\0" as *const u8
                as *const libc::c_char,
            1605 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"cairo_status_t blit_coverages(struct cell_list *, cairo_span_renderer_t *, struct pool *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    let mut next: *mut cell = cell;
    num_spans = 2 as libc::c_int as libc::c_uint;
    while !((*next).next).is_null() {
        next = (*next).next;
        num_spans = num_spans.wrapping_add(1);
    }
    num_spans = (2 as libc::c_int as libc::c_uint).wrapping_mul(num_spans);
    pool_reset(span_pool);
    spans = pool_alloc(
        span_pool,
        (::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong)
            .wrapping_mul(num_spans as libc::c_ulong),
    ) as *mut cairo_half_open_span_t;
    num_spans = 0 as libc::c_int as libc::c_uint;
    while !((*cell).next).is_null() {
        let mut x: libc::c_int = (*cell).x;
        let mut area: libc::c_int = 0;
        if x > prev_x && cover != last_cover {
            (*spans.offset(num_spans as isize)).x = prev_x;
            (*spans.offset(num_spans as isize))
                .coverage = (cover + (cover << 4 as libc::c_int) + 256 as libc::c_int
                >> 9 as libc::c_int) as uint8_t;
            (*spans.offset(num_spans as isize)).inverse = 0 as libc::c_int as uint8_t;
            last_cover = cover;
            num_spans = num_spans.wrapping_add(1);
        }
        cover
            += (*cell).covered_height * ((1 as libc::c_int) << 8 as libc::c_int)
                * 2 as libc::c_int;
        clip
            += (*cell).covered_height * ((1 as libc::c_int) << 8 as libc::c_int)
                * 2 as libc::c_int;
        area = cover - (*cell).uncovered_area;
        if area != last_cover {
            (*spans.offset(num_spans as isize)).x = x;
            (*spans.offset(num_spans as isize))
                .coverage = (area + (area << 4 as libc::c_int) + 256 as libc::c_int
                >> 9 as libc::c_int) as uint8_t;
            (*spans.offset(num_spans as isize)).inverse = 0 as libc::c_int as uint8_t;
            last_cover = area;
            num_spans = num_spans.wrapping_add(1);
        }
        prev_x = x + 1 as libc::c_int;
        cell = (*cell).next;
    }
    return ((*renderer).render_rows)
        .expect(
            "non-null function pointer",
        )(renderer as *mut libc::c_void, y, height, spans, num_spans);
}
unsafe extern "C" fn glitter_scan_converter_render(
    mut converter: *mut glitter_scan_converter_t,
    mut nonzero_fill: libc::c_int,
    mut span_renderer: *mut cairo_span_renderer_t,
    mut span_pool: *mut pool,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ymax_i: libc::c_int = (*converter).ymax / 15 as libc::c_int;
    let mut ymin_i: libc::c_int = (*converter).ymin / 15 as libc::c_int;
    let mut h: libc::c_int = ymax_i - ymin_i;
    let mut polygon: *mut polygon = ((*converter).polygon).as_mut_ptr();
    let mut coverages: *mut cell_list = ((*converter).coverages).as_mut_ptr();
    let mut active: *mut active_list = ((*converter).active).as_mut_ptr();
    let mut current_block_33: u64;
    i = 0 as libc::c_int;
    while i < h {
        let mut do_full_step: libc::c_int = 0 as libc::c_int;
        j = i + 1 as libc::c_int;
        if 15 as libc::c_int == 15 as libc::c_int
            && (*((*polygon).y_buckets).offset(i as isize)).is_null()
        {
            if ((*active).head).is_null() {
                while j < h && (*((*polygon).y_buckets).offset(j as isize)).is_null() {
                    j += 1;
                }
                current_block_33 = 820271813250567934;
            } else {
                do_full_step = active_list_can_step_full_row(active);
                current_block_33 = 3512920355445576850;
            }
        } else {
            current_block_33 = 3512920355445576850;
        }
        match current_block_33 {
            3512920355445576850 => {
                if do_full_step != 0 {
                    if nonzero_fill != 0 {
                        apply_nonzero_fill_rule_and_step_edges(active, coverages);
                    } else {
                        apply_evenodd_fill_rule_and_step_edges(active, coverages);
                    }
                    if active_list_is_vertical(active) != 0 {
                        while j < h
                            && (*((*polygon).y_buckets).offset(j as isize)).is_null()
                            && (*active).min_height
                                >= 2 as libc::c_int * 15 as libc::c_int
                        {
                            let ref mut fresh84 = (*active).min_height;
                            *fresh84 -= 15 as libc::c_int;
                            j += 1;
                        }
                        if j != i + 1 as libc::c_int {
                            step_edges(active, j - (i + 1 as libc::c_int));
                        }
                    }
                } else {
                    let mut suby: grid_scaled_y_t = 0;
                    suby = 0 as libc::c_int;
                    while suby < 15 as libc::c_int {
                        let mut y: grid_scaled_y_t = (i + ymin_i) * 15 as libc::c_int
                            + suby;
                        if !(*((*polygon).y_buckets).offset(i as isize)).is_null() {
                            active_list_merge_edges_from_polygon(
                                active,
                                &mut *((*polygon).y_buckets).offset(i as isize),
                                y,
                                polygon,
                            );
                        }
                        if nonzero_fill != 0 {
                            apply_nonzero_fill_rule_for_subrow(active, coverages);
                        } else {
                            apply_evenodd_fill_rule_for_subrow(active, coverages);
                        }
                        active_list_substep_edges(active);
                        suby += 1;
                    }
                }
                blit_coverages(coverages, span_renderer, span_pool, i + ymin_i, j - i);
                cell_list_reset(coverages);
                if ((*active).head).is_null() {
                    (*active).min_height = 2147483647 as libc::c_int;
                } else {
                    let ref mut fresh85 = (*active).min_height;
                    *fresh85 -= 15 as libc::c_int;
                }
            }
            _ => {}
        }
        i = j;
    }
}
unsafe extern "C" fn _cairo_clip_tor_scan_converter_destroy(
    mut converter: *mut libc::c_void,
) {
    let mut self_0: *mut cairo_clip_tor_scan_converter_t = converter
        as *mut cairo_clip_tor_scan_converter_t;
    if self_0.is_null() {
        return;
    }
    _glitter_scan_converter_fini(((*self_0).converter).as_mut_ptr());
    pool_fini(((*self_0).span_pool.base).as_mut_ptr());
    free(self_0 as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_clip_tor_scan_converter_generate(
    mut converter: *mut libc::c_void,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_clip_tor_scan_converter_t = converter
        as *mut cairo_clip_tor_scan_converter_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _setjmp(((*self_0).jmp).as_mut_ptr()) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_scan_converter_set_error(
            self_0 as *mut libc::c_void,
            _cairo_error(status),
        );
    }
    glitter_scan_converter_render(
        ((*self_0).converter).as_mut_ptr(),
        ((*self_0).fill_rule as libc::c_uint
            == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint) as libc::c_int,
        renderer,
        ((*self_0).span_pool.base).as_mut_ptr(),
    );
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_tor_scan_converter_create(
    mut clip: *mut cairo_clip_t,
    mut polygon: *mut cairo_polygon_t,
    mut fill_rule: cairo_fill_rule_t,
    mut antialias: cairo_antialias_t,
) -> *mut cairo_scan_converter_t {
    let mut self_0: *mut cairo_clip_tor_scan_converter_t = 0
        as *mut cairo_clip_tor_scan_converter_t;
    let mut clipper: cairo_polygon_t = cairo_polygon_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    self_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_cairo_clip_tor_scan_converter>() as libc::c_ulong,
    ) as *mut cairo_clip_tor_scan_converter_t;
    if self_0.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        let ref mut fresh86 = (*self_0).base.destroy;
        *fresh86 = Some(
            _cairo_clip_tor_scan_converter_destroy
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        let ref mut fresh87 = (*self_0).base.generate;
        *fresh87 = Some(
            _cairo_clip_tor_scan_converter_generate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_span_renderer_t,
                ) -> cairo_status_t,
        );
        pool_init(
            ((*self_0).span_pool.base).as_mut_ptr(),
            &mut (*self_0).jmp,
            (250 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
                ),
            ::std::mem::size_of::<[cairo_half_open_span_t; 32]>() as libc::c_ulong,
        );
        _glitter_scan_converter_init(
            ((*self_0).converter).as_mut_ptr(),
            &mut (*self_0).jmp,
        );
        status = glitter_scan_converter_reset(
            ((*self_0).converter).as_mut_ptr(),
            (*clip).extents.y,
            (*clip).extents.y + (*clip).extents.height,
        );
        if !(status as u64 != 0) {
            (*self_0).fill_rule = fill_rule;
            (*self_0).antialias = antialias;
            i = 0 as libc::c_int;
            while i < (*polygon).num_edges {
                glitter_scan_converter_add_edge(
                    ((*self_0).converter).as_mut_ptr(),
                    &mut *((*polygon).edges).offset(i as isize),
                    0 as libc::c_int,
                );
                i += 1;
            }
            status = _cairo_clip_get_polygon(
                clip,
                &mut clipper,
                &mut (*self_0).clip_fill_rule,
                &mut (*self_0).clip_antialias,
            ) as cairo_status_t;
            if !(status as u64 != 0) {
                i = 0 as libc::c_int;
                while i < clipper.num_edges {
                    glitter_scan_converter_add_edge(
                        ((*self_0).converter).as_mut_ptr(),
                        &mut *(clipper.edges).offset(i as isize),
                        1 as libc::c_int,
                    );
                    i += 1;
                }
                _cairo_polygon_fini(&mut clipper);
                return &mut (*self_0).base;
            }
        }
        ((*self_0).base.destroy)
            .expect(
                "non-null function pointer",
            )(&mut (*self_0).base as *mut cairo_scan_converter_t as *mut libc::c_void);
    }
    return _cairo_scan_converter_create_in_error(status);
}
