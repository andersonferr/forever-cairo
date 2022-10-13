use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_region;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type cairo_int64_t = int64_t;
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
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_edge_compute_intersection_y_for_x(
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut x: cairo_fixed_t,
) -> cairo_fixed_t {
    let mut y: cairo_fixed_t = 0;
    let mut dx: cairo_fixed_t = 0;
    if x == (*p1).x {
        return (*p1).y;
    }
    if x == (*p2).x {
        return (*p2).y;
    }
    y = (*p1).y;
    dx = (*p2).x - (*p1).x;
    if dx != 0 as libc::c_int {
        y += _cairo_fixed_mul_div_floor(x - (*p1).x, (*p2).y - (*p1).y, dx);
    }
    return y;
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
unsafe extern "C" fn _cairo_int64_32_div(
    mut num: cairo_int64_t,
    mut den: int32_t,
) -> int32_t {
    return (num / den as libc::c_long) as int32_t;
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_limit(
    mut polygon: *mut cairo_polygon_t,
    mut limits: *const cairo_box_t,
    mut num_limits: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let ref mut fresh4 = (*polygon).limits;
    *fresh4 = limits;
    (*polygon).num_limits = num_limits;
    if (*polygon).num_limits != 0 {
        (*polygon).limit = *limits.offset(0 as libc::c_int as isize);
        n = 1 as libc::c_int;
        while n < num_limits {
            if (*limits.offset(n as isize)).p1.x < (*polygon).limit.p1.x {
                (*polygon).limit.p1.x = (*limits.offset(n as isize)).p1.x;
            }
            if (*limits.offset(n as isize)).p1.y < (*polygon).limit.p1.y {
                (*polygon).limit.p1.y = (*limits.offset(n as isize)).p1.y;
            }
            if (*limits.offset(n as isize)).p2.x > (*polygon).limit.p2.x {
                (*polygon).limit.p2.x = (*limits.offset(n as isize)).p2.x;
            }
            if (*limits.offset(n as isize)).p2.y > (*polygon).limit.p2.y {
                (*polygon).limit.p2.y = (*limits.offset(n as isize)).p2.y;
            }
            n += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_limit_to_clip(
    mut polygon: *mut cairo_polygon_t,
    mut clip: *const cairo_clip_t,
) {
    if !clip.is_null() {
        _cairo_polygon_limit(polygon, (*clip).boxes, (*clip).num_boxes);
    } else {
        _cairo_polygon_limit(polygon, 0 as *const cairo_box_t, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_init(
    mut polygon: *mut cairo_polygon_t,
    mut limits: *const cairo_box_t,
    mut num_limits: libc::c_int,
) {
    (*polygon).status = CAIRO_STATUS_SUCCESS;
    (*polygon).num_edges = 0 as libc::c_int;
    let ref mut fresh5 = (*polygon).edges;
    *fresh5 = ((*polygon).edges_embedded).as_mut_ptr();
    (*polygon)
        .edges_size = (::std::mem::size_of::<[cairo_edge_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong)
        as libc::c_int;
    let ref mut fresh6 = (*polygon).extents.p1.y;
    *fresh6 = 2147483647 as libc::c_int;
    (*polygon).extents.p1.x = *fresh6;
    let ref mut fresh7 = (*polygon).extents.p2.y;
    *fresh7 = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*polygon).extents.p2.x = *fresh7;
    _cairo_polygon_limit(polygon, limits, num_limits);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_init_with_clip(
    mut polygon: *mut cairo_polygon_t,
    mut clip: *const cairo_clip_t,
) {
    if !clip.is_null() {
        _cairo_polygon_init(polygon, (*clip).boxes, (*clip).num_boxes);
    } else {
        _cairo_polygon_init(polygon, 0 as *const cairo_box_t, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_init_boxes(
    mut polygon: *mut cairo_polygon_t,
    mut boxes: *const cairo_boxes_t,
) -> cairo_status_t {
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    (*polygon).status = CAIRO_STATUS_SUCCESS;
    (*polygon).num_edges = 0 as libc::c_int;
    let ref mut fresh8 = (*polygon).edges;
    *fresh8 = ((*polygon).edges_embedded).as_mut_ptr();
    (*polygon)
        .edges_size = (::std::mem::size_of::<[cairo_edge_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong)
        as libc::c_int;
    if (*boxes).num_boxes
        > (::std::mem::size_of::<[cairo_edge_t; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong)
            as libc::c_int / 2 as libc::c_int
    {
        (*polygon).edges_size = 2 as libc::c_int * (*boxes).num_boxes;
        let ref mut fresh9 = (*polygon).edges;
        *fresh9 = _cairo_malloc_ab(
            (*polygon).edges_size as size_t,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong),
        ) as *mut cairo_edge_t;
        if ((*polygon).edges).is_null() {
            let ref mut fresh10 = (*polygon).status;
            *fresh10 = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return *fresh10;
        }
    }
    let ref mut fresh11 = (*polygon).extents.p1.y;
    *fresh11 = 2147483647 as libc::c_int;
    (*polygon).extents.p1.x = *fresh11;
    let ref mut fresh12 = (*polygon).extents.p2.y;
    *fresh12 = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*polygon).extents.p2.x = *fresh12;
    let ref mut fresh13 = (*polygon).limits;
    *fresh13 = 0 as *const cairo_box_t;
    (*polygon).num_limits = 0 as libc::c_int;
    chunk = &(*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let mut p1: cairo_point_t = cairo_point_t { x: 0, y: 0 };
            let mut p2: cairo_point_t = cairo_point_t { x: 0, y: 0 };
            p1 = (*((*chunk).base).offset(i as isize)).p1;
            p2.x = p1.x;
            p2.y = (*((*chunk).base).offset(i as isize)).p2.y;
            _cairo_polygon_add_edge(polygon, &mut p1, &mut p2, 1 as libc::c_int);
            p1 = (*((*chunk).base).offset(i as isize)).p2;
            p2.x = p1.x;
            p2.y = (*((*chunk).base).offset(i as isize)).p1.y;
            _cairo_polygon_add_edge(polygon, &mut p1, &mut p2, 1 as libc::c_int);
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return (*polygon).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_init_box_array(
    mut polygon: *mut cairo_polygon_t,
    mut boxes: *mut cairo_box_t,
    mut num_boxes: libc::c_int,
) -> cairo_status_t {
    let mut i: libc::c_int = 0;
    (*polygon).status = CAIRO_STATUS_SUCCESS;
    (*polygon).num_edges = 0 as libc::c_int;
    let ref mut fresh14 = (*polygon).edges;
    *fresh14 = ((*polygon).edges_embedded).as_mut_ptr();
    (*polygon)
        .edges_size = (::std::mem::size_of::<[cairo_edge_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong)
        as libc::c_int;
    if num_boxes
        > (::std::mem::size_of::<[cairo_edge_t; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong)
            as libc::c_int / 2 as libc::c_int
    {
        (*polygon).edges_size = 2 as libc::c_int * num_boxes;
        let ref mut fresh15 = (*polygon).edges;
        *fresh15 = _cairo_malloc_ab(
            (*polygon).edges_size as size_t,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong),
        ) as *mut cairo_edge_t;
        if ((*polygon).edges).is_null() {
            let ref mut fresh16 = (*polygon).status;
            *fresh16 = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return *fresh16;
        }
    }
    let ref mut fresh17 = (*polygon).extents.p1.y;
    *fresh17 = 2147483647 as libc::c_int;
    (*polygon).extents.p1.x = *fresh17;
    let ref mut fresh18 = (*polygon).extents.p2.y;
    *fresh18 = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*polygon).extents.p2.x = *fresh18;
    let ref mut fresh19 = (*polygon).limits;
    *fresh19 = 0 as *const cairo_box_t;
    (*polygon).num_limits = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_boxes {
        let mut p1: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        let mut p2: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        p1 = (*boxes.offset(i as isize)).p1;
        p2.x = p1.x;
        p2.y = (*boxes.offset(i as isize)).p2.y;
        _cairo_polygon_add_edge(polygon, &mut p1, &mut p2, 1 as libc::c_int);
        p1 = (*boxes.offset(i as isize)).p2;
        p2.x = p1.x;
        p2.y = (*boxes.offset(i as isize)).p1.y;
        _cairo_polygon_add_edge(polygon, &mut p1, &mut p2, 1 as libc::c_int);
        i += 1;
    }
    return (*polygon).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_fini(mut polygon: *mut cairo_polygon_t) {
    if (*polygon).edges != ((*polygon).edges_embedded).as_mut_ptr() {
        free((*polygon).edges as *mut libc::c_void);
    }
}
unsafe extern "C" fn _cairo_polygon_grow(
    mut polygon: *mut cairo_polygon_t,
) -> cairo_bool_t {
    let mut new_edges: *mut cairo_edge_t = 0 as *mut cairo_edge_t;
    let mut old_size: libc::c_int = (*polygon).edges_size;
    let mut new_size: libc::c_int = 4 as libc::c_int * old_size;
    if (*polygon).edges == ((*polygon).edges_embedded).as_mut_ptr() {
        new_edges = _cairo_malloc_ab(
            new_size as size_t,
            ::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong,
        ) as *mut cairo_edge_t;
        if !new_edges.is_null() {
            memcpy(
                new_edges as *mut libc::c_void,
                (*polygon).edges as *const libc::c_void,
                (old_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong),
            );
        }
    } else {
        new_edges = _cairo_realloc_ab(
            (*polygon).edges as *mut libc::c_void,
            new_size as size_t,
            ::std::mem::size_of::<cairo_edge_t>() as libc::c_ulong,
        ) as *mut cairo_edge_t;
    }
    if new_edges.is_null() {
        (*polygon).status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as libc::c_int;
    }
    let ref mut fresh20 = (*polygon).edges;
    *fresh20 = new_edges;
    (*polygon).edges_size = new_size;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _add_edge(
    mut polygon: *mut cairo_polygon_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut top: libc::c_int,
    mut bottom: libc::c_int,
    mut dir: libc::c_int,
) {
    let mut edge: *mut cairo_edge_t = 0 as *mut cairo_edge_t;
    if top < bottom {} else {
        __assert_fail(
            b"top < bottom\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-polygon.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void _add_edge(cairo_polygon_t *, const cairo_point_t *, const cairo_point_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*polygon).num_edges == (*polygon).edges_size {
        if _cairo_polygon_grow(polygon) == 0 {
            return;
        }
    }
    let ref mut fresh21 = (*polygon).num_edges;
    let fresh22 = *fresh21;
    *fresh21 = *fresh21 + 1;
    edge = &mut *((*polygon).edges).offset(fresh22 as isize) as *mut cairo_edge_t;
    (*edge).line.p1 = *p1;
    (*edge).line.p2 = *p2;
    (*edge).top = top;
    (*edge).bottom = bottom;
    (*edge).dir = dir;
    if top < (*polygon).extents.p1.y {
        (*polygon).extents.p1.y = top;
    }
    if bottom > (*polygon).extents.p2.y {
        (*polygon).extents.p2.y = bottom;
    }
    if (*p1).x < (*polygon).extents.p1.x || (*p1).x > (*polygon).extents.p2.x {
        let mut x: cairo_fixed_t = (*p1).x;
        if top != (*p1).y {
            x = _cairo_edge_compute_intersection_x_for_y(p1, p2, top);
        }
        if x < (*polygon).extents.p1.x {
            (*polygon).extents.p1.x = x;
        }
        if x > (*polygon).extents.p2.x {
            (*polygon).extents.p2.x = x;
        }
    }
    if (*p2).x < (*polygon).extents.p1.x || (*p2).x > (*polygon).extents.p2.x {
        let mut x_0: cairo_fixed_t = (*p2).x;
        if bottom != (*p2).y {
            x_0 = _cairo_edge_compute_intersection_x_for_y(p1, p2, bottom);
        }
        if x_0 < (*polygon).extents.p1.x {
            (*polygon).extents.p1.x = x_0;
        }
        if x_0 > (*polygon).extents.p2.x {
            (*polygon).extents.p2.x = x_0;
        }
    }
}
unsafe extern "C" fn _add_clipped_edge(
    mut polygon: *mut cairo_polygon_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    top: libc::c_int,
    bottom: libc::c_int,
    dir: libc::c_int,
) {
    let mut bot_left: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut top_right: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut top_y: cairo_fixed_t = 0;
    let mut bot_y: cairo_fixed_t = 0;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*polygon).num_limits {
        let mut limits: *const cairo_box_t = &*((*polygon).limits).offset(n as isize)
            as *const cairo_box_t;
        let mut pleft: cairo_fixed_t = 0;
        let mut pright: cairo_fixed_t = 0;
        if !(top >= (*limits).p2.y) {
            if !(bottom <= (*limits).p1.y) {
                bot_left.x = (*limits).p1.x;
                bot_left.y = (*limits).p2.y;
                top_right.x = (*limits).p2.x;
                top_right.y = (*limits).p1.y;
                top_y = if top > (*limits).p1.y { top } else { (*limits).p1.y };
                bot_y = if bottom < (*limits).p2.y { bottom } else { (*limits).p2.y };
                pleft = if (*p1).x < (*p2).x { (*p1).x } else { (*p2).x };
                pright = if (*p1).x > (*p2).x { (*p1).x } else { (*p2).x };
                if (*limits).p1.x <= pleft && pright <= (*limits).p2.x {
                    _add_edge(polygon, p1, p2, top_y, bot_y, dir);
                } else if pright <= (*limits).p1.x {
                    _add_edge(polygon, &(*limits).p1, &mut bot_left, top_y, bot_y, dir);
                } else if (*limits).p2.x <= pleft {
                    _add_edge(polygon, &mut top_right, &(*limits).p2, top_y, bot_y, dir);
                } else {
                    let mut left_y: cairo_fixed_t = 0;
                    let mut right_y: cairo_fixed_t = 0;
                    let mut top_left_to_bottom_right: cairo_bool_t = 0;
                    top_left_to_bottom_right = (((*p1).x <= (*p2).x) as libc::c_int
                        == ((*p1).y <= (*p2).y) as libc::c_int) as libc::c_int;
                    if top_left_to_bottom_right != 0 {
                        if pleft >= (*limits).p1.x {
                            left_y = top_y;
                        } else {
                            left_y = _cairo_edge_compute_intersection_y_for_x(
                                p1,
                                p2,
                                (*limits).p1.x,
                            );
                            if _cairo_edge_compute_intersection_x_for_y(p1, p2, left_y)
                                < (*limits).p1.x
                            {
                                left_y += 1;
                            }
                        }
                        left_y = if left_y < bot_y { left_y } else { bot_y };
                        if top_y < left_y {
                            _add_edge(
                                polygon,
                                &(*limits).p1,
                                &mut bot_left,
                                top_y,
                                left_y,
                                dir,
                            );
                            top_y = left_y;
                        }
                        if pright <= (*limits).p2.x {
                            right_y = bot_y;
                        } else {
                            right_y = _cairo_edge_compute_intersection_y_for_x(
                                p1,
                                p2,
                                (*limits).p2.x,
                            );
                            if _cairo_edge_compute_intersection_x_for_y(p1, p2, right_y)
                                > (*limits).p2.x
                            {
                                right_y -= 1;
                            }
                        }
                        right_y = if right_y > top_y { right_y } else { top_y };
                        if bot_y > right_y {
                            _add_edge(
                                polygon,
                                &mut top_right,
                                &(*limits).p2,
                                right_y,
                                bot_y,
                                dir,
                            );
                            bot_y = right_y;
                        }
                    } else {
                        if pright <= (*limits).p2.x {
                            right_y = top_y;
                        } else {
                            right_y = _cairo_edge_compute_intersection_y_for_x(
                                p1,
                                p2,
                                (*limits).p2.x,
                            );
                            if _cairo_edge_compute_intersection_x_for_y(p1, p2, right_y)
                                > (*limits).p2.x
                            {
                                right_y += 1;
                            }
                        }
                        right_y = if right_y < bot_y { right_y } else { bot_y };
                        if top_y < right_y {
                            _add_edge(
                                polygon,
                                &mut top_right,
                                &(*limits).p2,
                                top_y,
                                right_y,
                                dir,
                            );
                            top_y = right_y;
                        }
                        if pleft >= (*limits).p1.x {
                            left_y = bot_y;
                        } else {
                            left_y = _cairo_edge_compute_intersection_y_for_x(
                                p1,
                                p2,
                                (*limits).p1.x,
                            );
                            if _cairo_edge_compute_intersection_x_for_y(p1, p2, left_y)
                                < (*limits).p1.x
                            {
                                left_y -= 1;
                            }
                        }
                        left_y = if left_y > top_y { left_y } else { top_y };
                        if bot_y > left_y {
                            _add_edge(
                                polygon,
                                &(*limits).p1,
                                &mut bot_left,
                                left_y,
                                bot_y,
                                dir,
                            );
                            bot_y = left_y;
                        }
                    }
                    if top_y != bot_y {
                        _add_edge(polygon, p1, p2, top_y, bot_y, dir);
                    }
                }
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn _cairo_polygon_add_edge(
    mut polygon: *mut cairo_polygon_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut dir: libc::c_int,
) {
    if (*p1).y == (*p2).y {
        return;
    }
    if (*p1).y > (*p2).y {
        let mut t: *const cairo_point_t = 0 as *const cairo_point_t;
        t = p1;
        p1 = p2;
        p2 = t;
        dir = -dir;
    }
    if (*polygon).num_limits != 0 {
        if (*p2).y <= (*polygon).limit.p1.y {
            return;
        }
        if (*p1).y >= (*polygon).limit.p2.y {
            return;
        }
        _add_clipped_edge(polygon, p1, p2, (*p1).y, (*p2).y, dir);
    } else {
        _add_edge(polygon, p1, p2, (*p1).y, (*p2).y, dir);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_add_external_edge(
    mut polygon: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) -> cairo_status_t {
    _cairo_polygon_add_edge(polygon as *mut cairo_polygon_t, p1, p2, 1 as libc::c_int);
    return (*(polygon as *mut cairo_polygon_t)).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_add_line(
    mut polygon: *mut cairo_polygon_t,
    mut line: *const cairo_line_t,
    mut top: libc::c_int,
    mut bottom: libc::c_int,
    mut dir: libc::c_int,
) -> cairo_status_t {
    if (*line).p1.y == (*line).p2.y {
        return CAIRO_STATUS_SUCCESS;
    }
    if bottom <= top {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*polygon).num_limits != 0 {
        if (*line).p2.y <= (*polygon).limit.p1.y {
            return CAIRO_STATUS_SUCCESS;
        }
        if (*line).p1.y >= (*polygon).limit.p2.y {
            return CAIRO_STATUS_SUCCESS;
        }
        _add_clipped_edge(polygon, &(*line).p1, &(*line).p2, top, bottom, dir);
    } else {
        _add_edge(polygon, &(*line).p1, &(*line).p2, top, bottom, dir);
    }
    return (*polygon).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_add_contour(
    mut polygon: *mut cairo_polygon_t,
    mut contour: *const cairo_contour_t,
) -> cairo_status_t {
    let mut chain: *const _cairo_contour_chain = 0 as *const _cairo_contour_chain;
    let mut prev: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut i: libc::c_int = 0;
    if (*contour).chain.num_points <= 1 as libc::c_int {
        return CAIRO_INT_STATUS_SUCCESS as libc::c_int as cairo_status_t;
    }
    prev = &mut *((*contour).chain.points).offset(0 as libc::c_int as isize)
        as *mut cairo_point_t;
    chain = &(*contour).chain;
    while !chain.is_null() {
        i = 0 as libc::c_int;
        while i < (*chain).num_points {
            _cairo_polygon_add_edge(
                polygon,
                prev,
                &mut *((*chain).points).offset(i as isize),
                (*contour).direction,
            );
            prev = &mut *((*chain).points).offset(i as isize) as *mut cairo_point_t;
            i += 1;
        }
        chain = (*chain).next;
    }
    return (*polygon).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_translate(
    mut polygon: *mut cairo_polygon_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    dx = _cairo_fixed_from_int(dx);
    dy = _cairo_fixed_from_int(dy);
    let ref mut fresh23 = (*polygon).extents.p1.x;
    *fresh23 += dx;
    let ref mut fresh24 = (*polygon).extents.p2.x;
    *fresh24 += dx;
    let ref mut fresh25 = (*polygon).extents.p1.y;
    *fresh25 += dy;
    let ref mut fresh26 = (*polygon).extents.p2.y;
    *fresh26 += dy;
    n = 0 as libc::c_int;
    while n < (*polygon).num_edges {
        let mut e: *mut cairo_edge_t = &mut *((*polygon).edges).offset(n as isize)
            as *mut cairo_edge_t;
        (*e).top += dy;
        (*e).bottom += dy;
        let ref mut fresh27 = (*e).line.p1.x;
        *fresh27 += dx;
        let ref mut fresh28 = (*e).line.p2.x;
        *fresh28 += dx;
        let ref mut fresh29 = (*e).line.p1.y;
        *fresh29 += dy;
        let ref mut fresh30 = (*e).line.p2.y;
        *fresh30 += dy;
        n += 1;
    }
}
