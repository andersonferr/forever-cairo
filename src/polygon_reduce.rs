use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_polygon_add_line(
        polygon: *mut cairo_polygon_t,
        line: *const cairo_line_t,
        top: libc::c_int,
        bottom: libc::c_int,
        dir: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_debug_print_polygon(stream: *mut FILE, polygon: *mut cairo_polygon_t);
    fn _cairo_int_96by64_32x64_divrem(
        num: cairo_int128_t,
        den: cairo_int64_t,
    ) -> cairo_quorem64_t;
    fn _cairo_freepool_init(freepool: *mut cairo_freepool_t, nodesize: libc::c_uint);
    fn _cairo_freepool_fini(freepool: *mut cairo_freepool_t);
    fn _cairo_freepool_alloc_from_new_pool(
        freepool: *mut cairo_freepool_t,
    ) -> *mut libc::c_void;
}
pub type __int128_t = i128;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type cairo_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_quorem64 {
    pub quo: cairo_int64_t,
    pub rem: cairo_int64_t,
}
pub type cairo_quorem64_t = _cairo_quorem64;
pub type int128_t = __int128_t;
pub type cairo_int128_t = int128_t;
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
pub type cairo_bo_start_event_t = _cairo_bo_start_event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_start_event {
    pub type_0: cairo_bo_event_type_t,
    pub point: cairo_point_t,
    pub edge: cairo_bo_edge_t,
}
pub type cairo_bo_edge_t = _cairo_bo_edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_edge {
    pub edge: cairo_edge_t,
    pub prev: *mut cairo_bo_edge_t,
    pub next: *mut cairo_bo_edge_t,
    pub deferred: cairo_bo_deferred_t,
}
pub type cairo_bo_deferred_t = _cairo_bo_deferred;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_deferred {
    pub right: *mut cairo_bo_edge_t,
    pub top: int32_t,
}
pub type cairo_bo_event_type_t = libc::c_uint;
pub const CAIRO_BO_EVENT_TYPE_START: cairo_bo_event_type_t = 2;
pub const CAIRO_BO_EVENT_TYPE_INTERSECTION: cairo_bo_event_type_t = 1;
pub const CAIRO_BO_EVENT_TYPE_STOP: cairo_bo_event_type_t = 0;
pub type cairo_bo_event_t = _cairo_bo_event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_event {
    pub type_0: cairo_bo_event_type_t,
    pub point: cairo_point_t,
}
pub type cairo_bo_event_queue_t = _cairo_bo_event_queue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_event_queue {
    pub pool: cairo_freepool_t,
    pub pqueue: pqueue_t,
    pub start_events: *mut *mut cairo_bo_event_t,
}
pub type pqueue_t = _pqueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pqueue {
    pub size: libc::c_int,
    pub max_size: libc::c_int,
    pub elements: *mut *mut cairo_bo_event_t,
    pub elements_embedded: [*mut cairo_bo_event_t; 1024],
}
pub type cairo_freepool_t = _cairo_freepool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freepool {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub pools: *mut cairo_freelist_pool_t,
    pub freepools: *mut cairo_freelist_pool_t,
    pub nodesize: libc::c_uint,
    pub embedded_pool: cairo_freelist_pool_t,
    pub embedded_data: [uint8_t; 1000],
}
pub type cairo_freelist_pool_t = _cairo_freelist_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_pool {
    pub next: *mut cairo_freelist_pool_t,
    pub size: libc::c_uint,
    pub rem: libc::c_uint,
    pub data: *mut uint8_t,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
pub type cairo_bo_point32_t = cairo_point_t;
pub type cairo_bo_queue_event_t = _cairo_bo_queue_event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_queue_event {
    pub type_0: cairo_bo_event_type_t,
    pub point: cairo_point_t,
    pub e1: *mut cairo_bo_edge_t,
    pub e2: *mut cairo_bo_edge_t,
}
pub type cairo_bo_intersect_ordinate_t = _cairo_bo_intersect_ordinate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_intersect_ordinate {
    pub ordinate: int32_t,
    pub exactness: C2RustUnnamed,
}
pub type C2RustUnnamed = libc::c_uint;
pub const INEXACT: C2RustUnnamed = 1;
pub const EXACT: C2RustUnnamed = 0;
pub type cairo_bo_intersect_point_t = _cairo_bo_intersect_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_intersect_point {
    pub x: cairo_bo_intersect_ordinate_t,
    pub y: cairo_bo_intersect_ordinate_t,
}
pub type cairo_bo_sweep_line_t = _cairo_bo_sweep_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_sweep_line {
    pub head: *mut cairo_bo_edge_t,
    pub current_y: int32_t,
    pub current_edge: *mut cairo_bo_edge_t,
}
pub const HAVE_BOTH: C2RustUnnamed_1 = 3;
pub const HAVE_BX: C2RustUnnamed_1 = 2;
pub const HAVE_AX: C2RustUnnamed_1 = 1;
pub const HAVE_ALL: C2RustUnnamed_0 = 7;
pub const HAVE_DX_BDX: C2RustUnnamed_0 = 5;
pub const HAVE_DX_ADX: C2RustUnnamed_0 = 3;
pub const HAVE_ADX_BDX: C2RustUnnamed_0 = 6;
pub const HAVE_BDX: C2RustUnnamed_0 = 4;
pub const HAVE_ADX: C2RustUnnamed_0 = 2;
pub const HAVE_DX: C2RustUnnamed_0 = 1;
pub const HAVE_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HAVE_NEITHER: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
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
unsafe extern "C" fn _cairo_fixed_mul_div_floor(
    mut a: cairo_fixed_t,
    mut b: cairo_fixed_t,
    mut c: cairo_fixed_t,
) -> cairo_fixed_t {
    return _cairo_int64_32_div(a as int64_t * b as libc::c_long, c);
}
#[inline]
unsafe extern "C" fn _cairo_int64_32_div(
    mut num: cairo_int64_t,
    mut den: int32_t,
) -> int32_t {
    return (num / den as libc::c_long) as int32_t;
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab_plus_c(
    mut a: size_t,
    mut size: size_t,
    mut c: size_t,
) -> *mut libc::c_void {
    let mut d: size_t = 0;
    let mut e: size_t = 0;
    let (fresh4, fresh5) = a.overflowing_mul(size);
    *(&mut d as *mut size_t) = fresh4;
    if fresh5 {
        return 0 as *mut libc::c_void;
    }
    let (fresh6, fresh7) = d.overflowing_add(c);
    *(&mut e as *mut size_t) = fresh6;
    if fresh7 {
        return 0 as *mut libc::c_void;
    }
    return if e != 0 as libc::c_int as libc::c_ulong {
        malloc(e)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline]
unsafe extern "C" fn _cairo_freepool_alloc(
    mut freepool: *mut cairo_freepool_t,
) -> *mut libc::c_void {
    let mut node: *mut cairo_freelist_node_t = 0 as *mut cairo_freelist_node_t;
    node = (*freepool).first_free_node;
    if node.is_null() {
        return _cairo_freepool_alloc_from_pool(freepool);
    }
    let ref mut fresh8 = (*freepool).first_free_node;
    *fresh8 = (*node).next;
    return node as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_alloc_from_pool(
    mut freepool: *mut cairo_freepool_t,
) -> *mut libc::c_void {
    let mut pool: *mut cairo_freelist_pool_t = 0 as *mut cairo_freelist_pool_t;
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    pool = (*freepool).pools;
    if (*freepool).nodesize > (*pool).rem {
        return _cairo_freepool_alloc_from_new_pool(freepool);
    }
    ptr = (*pool).data;
    let ref mut fresh9 = (*pool).data;
    *fresh9 = (*fresh9).offset((*freepool).nodesize as isize);
    let ref mut fresh10 = (*pool).rem;
    *fresh10 = (*fresh10).wrapping_sub((*freepool).nodesize);
    return ptr as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_free(
    mut freepool: *mut cairo_freepool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut node: *mut cairo_freelist_node_t = ptr as *mut cairo_freelist_node_t;
    let ref mut fresh11 = (*node).next;
    *fresh11 = (*freepool).first_free_node;
    let ref mut fresh12 = (*freepool).first_free_node;
    *fresh12 = node;
}
#[inline]
unsafe extern "C" fn _cairo_combsort_newgap(mut gap: libc::c_uint) -> libc::c_uint {
    gap = (10 as libc::c_int as libc::c_uint)
        .wrapping_mul(gap)
        .wrapping_div(13 as libc::c_int as libc::c_uint);
    if gap == 9 as libc::c_int as libc::c_uint
        || gap == 10 as libc::c_int as libc::c_uint
    {
        gap = 11 as libc::c_int as libc::c_uint;
    }
    if gap < 1 as libc::c_int as libc::c_uint {
        gap = 1 as libc::c_int as libc::c_uint;
    }
    return gap;
}
unsafe extern "C" fn _line_compute_intersection_x_for_y(
    mut line: *const cairo_line_t,
    mut y: cairo_fixed_t,
) -> cairo_fixed_t {
    let mut x: cairo_fixed_t = 0;
    let mut dy: cairo_fixed_t = 0;
    if y == (*line).p1.y {
        return (*line).p1.x;
    }
    if y == (*line).p2.y {
        return (*line).p2.x;
    }
    x = (*line).p1.x;
    dy = (*line).p2.y - (*line).p1.y;
    if dy != 0 as libc::c_int {
        x
            += _cairo_fixed_mul_div_floor(
                y - (*line).p1.y,
                (*line).p2.x - (*line).p1.x,
                dy,
            );
    }
    return x;
}
#[inline]
unsafe extern "C" fn _cairo_bo_point32_compare(
    mut a: *const cairo_bo_point32_t,
    mut b: *const cairo_bo_point32_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = (*a).y - (*b).y;
    if cmp != 0 {
        return cmp;
    }
    return (*a).x - (*b).x;
}
#[inline]
unsafe extern "C" fn _slope_compare(
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
) -> libc::c_int {
    let mut adx: int32_t = (*a).edge.line.p2.x - (*a).edge.line.p1.x;
    let mut bdx: int32_t = (*b).edge.line.p2.x - (*b).edge.line.p1.x;
    if adx == 0 as libc::c_int {
        return -bdx;
    }
    if bdx == 0 as libc::c_int {
        return adx;
    }
    if adx ^ bdx < 0 as libc::c_int {
        return adx;
    }
    let mut ady: int32_t = (*a).edge.line.p2.y - (*a).edge.line.p1.y;
    let mut bdy: int32_t = (*b).edge.line.p2.y - (*b).edge.line.p1.y;
    let mut adx_bdy: cairo_int64_t = adx as int64_t * bdy as libc::c_long;
    let mut bdx_ady: cairo_int64_t = bdx as int64_t * ady as libc::c_long;
    return if adx_bdy == bdx_ady {
        0 as libc::c_int
    } else if adx_bdy < bdx_ady {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn edges_compare_x_for_y_general(
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
    mut y: int32_t,
) -> libc::c_int {
    let mut dx: int32_t = 0;
    let mut adx: int32_t = 0;
    let mut ady: int32_t = 0;
    let mut bdx: int32_t = 0;
    let mut bdy: int32_t = 0;
    let mut have_dx_adx_bdx: C2RustUnnamed_0 = HAVE_ALL;
    let mut amin: int32_t = 0;
    let mut amax: int32_t = 0;
    let mut bmin: int32_t = 0;
    let mut bmax: int32_t = 0;
    if (*a).edge.line.p1.x < (*a).edge.line.p2.x {
        amin = (*a).edge.line.p1.x;
        amax = (*a).edge.line.p2.x;
    } else {
        amin = (*a).edge.line.p2.x;
        amax = (*a).edge.line.p1.x;
    }
    if (*b).edge.line.p1.x < (*b).edge.line.p2.x {
        bmin = (*b).edge.line.p1.x;
        bmax = (*b).edge.line.p2.x;
    } else {
        bmin = (*b).edge.line.p2.x;
        bmax = (*b).edge.line.p1.x;
    }
    if amax < bmin {
        return -(1 as libc::c_int);
    }
    if amin > bmax {
        return 1 as libc::c_int;
    }
    ady = (*a).edge.line.p2.y - (*a).edge.line.p1.y;
    adx = (*a).edge.line.p2.x - (*a).edge.line.p1.x;
    if adx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_ADX as libc::c_int) as libc::c_uint);
    }
    bdy = (*b).edge.line.p2.y - (*b).edge.line.p1.y;
    bdx = (*b).edge.line.p2.x - (*b).edge.line.p1.x;
    if bdx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_BDX as libc::c_int) as libc::c_uint);
    }
    dx = (*a).edge.line.p1.x - (*b).edge.line.p1.x;
    if dx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_DX as libc::c_int) as libc::c_uint);
    }
    match have_dx_adx_bdx as libc::c_uint {
        1 => return dx,
        2 => return adx,
        4 => return -bdx,
        6 => {
            if adx ^ bdx < 0 as libc::c_int {
                return adx
            } else if (*a).edge.line.p1.y == (*b).edge.line.p1.y {
                let mut adx_bdy: cairo_int64_t = 0;
                let mut bdx_ady: cairo_int64_t = 0;
                adx_bdy = adx as int64_t * bdy as libc::c_long;
                bdx_ady = bdx as int64_t * ady as libc::c_long;
                return if adx_bdy == bdx_ady {
                    0 as libc::c_int
                } else if adx_bdy < bdx_ady {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            } else {
                return if (adx as int64_t * bdy as libc::c_long) as int128_t
                    * (y - (*a).edge.line.p1.y) as int64_t as i128
                    == (bdx as int64_t * ady as libc::c_long) as int128_t
                        * (y - (*b).edge.line.p1.y) as int64_t as i128
                {
                    0 as libc::c_int
                } else if ((adx as int64_t * bdy as libc::c_long) as int128_t
                    * (y - (*a).edge.line.p1.y) as int64_t as i128)
                    < (bdx as int64_t * ady as libc::c_long) as int128_t
                        * (y - (*b).edge.line.p1.y) as int64_t as i128
                {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }
            }
        }
        3 => {
            if -adx ^ dx < 0 as libc::c_int {
                return dx
            } else {
                let mut ady_dx: cairo_int64_t = 0;
                let mut dy_adx: cairo_int64_t = 0;
                ady_dx = ady as int64_t * dx as libc::c_long;
                dy_adx = ((*a).edge.line.p1.y - y) as int64_t * adx as libc::c_long;
                return if ady_dx == dy_adx {
                    0 as libc::c_int
                } else if ady_dx < dy_adx {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
        }
        5 => {
            if bdx ^ dx < 0 as libc::c_int {
                return dx
            } else {
                let mut bdy_dx: cairo_int64_t = 0;
                let mut dy_bdx: cairo_int64_t = 0;
                bdy_dx = bdy as int64_t * dx as libc::c_long;
                dy_bdx = (y - (*b).edge.line.p1.y) as int64_t * bdx as libc::c_long;
                return if bdy_dx == dy_bdx {
                    0 as libc::c_int
                } else if bdy_dx < dy_bdx {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
        }
        7 => {
            return if (ady as int64_t * bdy as libc::c_long) as int128_t
                * dx as int64_t as i128
                == (bdx as int64_t * ady as libc::c_long) as int128_t
                    * (y - (*b).edge.line.p1.y) as int64_t as i128
                    - (adx as int64_t * bdy as libc::c_long) as int128_t
                        * (y - (*a).edge.line.p1.y) as int64_t as i128
            {
                0 as libc::c_int
            } else if ((ady as int64_t * bdy as libc::c_long) as int128_t
                * dx as int64_t as i128)
                < (bdx as int64_t * ady as libc::c_long) as int128_t
                    * (y - (*b).edge.line.p1.y) as int64_t as i128
                    - (adx as int64_t * bdy as libc::c_long) as int128_t
                        * (y - (*a).edge.line.p1.y) as int64_t as i128
            {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            };
        }
        0 | _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn edge_compare_for_y_against_x(
    mut a: *const cairo_bo_edge_t,
    mut y: int32_t,
    mut x: int32_t,
) -> libc::c_int {
    let mut adx: int32_t = 0;
    let mut ady: int32_t = 0;
    let mut dx: int32_t = 0;
    let mut dy: int32_t = 0;
    let mut L: cairo_int64_t = 0;
    let mut R: cairo_int64_t = 0;
    if x < (*a).edge.line.p1.x && x < (*a).edge.line.p2.x {
        return 1 as libc::c_int;
    }
    if x > (*a).edge.line.p1.x && x > (*a).edge.line.p2.x {
        return -(1 as libc::c_int);
    }
    adx = (*a).edge.line.p2.x - (*a).edge.line.p1.x;
    dx = x - (*a).edge.line.p1.x;
    if adx == 0 as libc::c_int {
        return -dx;
    }
    if dx == 0 as libc::c_int || adx ^ dx < 0 as libc::c_int {
        return adx;
    }
    dy = y - (*a).edge.line.p1.y;
    ady = (*a).edge.line.p2.y - (*a).edge.line.p1.y;
    L = dy as int64_t * adx as libc::c_long;
    R = dx as int64_t * ady as libc::c_long;
    return if L == R {
        0 as libc::c_int
    } else if L < R {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn edges_compare_x_for_y(
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
    mut y: int32_t,
) -> libc::c_int {
    let mut have_ax_bx: C2RustUnnamed_1 = HAVE_BOTH;
    let mut ax: int32_t = 0 as libc::c_int;
    let mut bx: int32_t = 0 as libc::c_int;
    if y == (*a).edge.line.p1.y {
        ax = (*a).edge.line.p1.x;
    } else if y == (*a).edge.line.p2.y {
        ax = (*a).edge.line.p2.x;
    } else {
        have_ax_bx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_1,
        >(have_ax_bx as libc::c_uint & !(HAVE_AX as libc::c_int) as libc::c_uint);
    }
    if y == (*b).edge.line.p1.y {
        bx = (*b).edge.line.p1.x;
    } else if y == (*b).edge.line.p2.y {
        bx = (*b).edge.line.p2.x;
    } else {
        have_ax_bx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_1,
        >(have_ax_bx as libc::c_uint & !(HAVE_BX as libc::c_int) as libc::c_uint);
    }
    match have_ax_bx as libc::c_uint {
        1 => return -edge_compare_for_y_against_x(b, y, ax),
        2 => return edge_compare_for_y_against_x(a, y, bx),
        3 => return ax - bx,
        0 | _ => return edges_compare_x_for_y_general(a, b, y),
    };
}
#[inline]
unsafe extern "C" fn _line_equal(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
) -> libc::c_int {
    return ((*a).p1.x == (*b).p1.x && (*a).p1.y == (*b).p1.y && (*a).p2.x == (*b).p2.x
        && (*a).p2.y == (*b).p2.y) as libc::c_int;
}
unsafe extern "C" fn _cairo_bo_sweep_line_compare_edges(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    if _line_equal(&(*a).edge.line, &(*b).edge.line) == 0 {
        cmp = edges_compare_x_for_y(a, b, (*sweep_line).current_y);
        if cmp != 0 {
            return cmp;
        }
        cmp = _slope_compare(a, b);
        if cmp != 0 {
            return cmp;
        }
    }
    return (*b).edge.bottom - (*a).edge.bottom;
}
#[inline]
unsafe extern "C" fn det32_64(
    mut a: int32_t,
    mut b: int32_t,
    mut c: int32_t,
    mut d: int32_t,
) -> cairo_int64_t {
    return a as int64_t * d as libc::c_long - b as int64_t * c as libc::c_long;
}
#[inline]
unsafe extern "C" fn det64x32_128(
    mut a: cairo_int64_t,
    mut b: int32_t,
    mut c: cairo_int64_t,
    mut d: int32_t,
) -> cairo_int128_t {
    return a as int128_t * d as int64_t as i128 - c as int128_t * b as int64_t as i128;
}
unsafe extern "C" fn intersect_lines(
    mut a: *mut cairo_bo_edge_t,
    mut b: *mut cairo_bo_edge_t,
    mut intersection: *mut cairo_bo_intersect_point_t,
) -> cairo_bool_t {
    let mut a_det: cairo_int64_t = 0;
    let mut b_det: cairo_int64_t = 0;
    let mut dx1: int32_t = (*a).edge.line.p1.x - (*a).edge.line.p2.x;
    let mut dy1: int32_t = (*a).edge.line.p1.y - (*a).edge.line.p2.y;
    let mut dx2: int32_t = (*b).edge.line.p1.x - (*b).edge.line.p2.x;
    let mut dy2: int32_t = (*b).edge.line.p1.y - (*b).edge.line.p2.y;
    let mut den_det: cairo_int64_t = 0;
    let mut R: cairo_int64_t = 0;
    let mut qr: cairo_quorem64_t = cairo_quorem64_t { quo: 0, rem: 0 };
    den_det = det32_64(dx1, dy1, dx2, dy2);
    R = det32_64(
        dx2,
        dy2,
        (*b).edge.line.p1.x - (*a).edge.line.p1.x,
        (*b).edge.line.p1.y - (*a).edge.line.p1.y,
    );
    if den_det < 0 as libc::c_int as libc::c_long {
        if !(den_det < R) {
            return 0 as libc::c_int;
        }
    } else if !(R < den_det) {
        return 0 as libc::c_int
    }
    R = det32_64(
        dy1,
        dx1,
        (*a).edge.line.p1.y - (*b).edge.line.p1.y,
        (*a).edge.line.p1.x - (*b).edge.line.p1.x,
    );
    if den_det < 0 as libc::c_int as libc::c_long {
        if !(den_det < R) {
            return 0 as libc::c_int;
        }
    } else if !(R < den_det) {
        return 0 as libc::c_int
    }
    a_det = det32_64(
        (*a).edge.line.p1.x,
        (*a).edge.line.p1.y,
        (*a).edge.line.p2.x,
        (*a).edge.line.p2.y,
    );
    b_det = det32_64(
        (*b).edge.line.p1.x,
        (*b).edge.line.p1.y,
        (*b).edge.line.p2.x,
        (*b).edge.line.p2.y,
    );
    qr = _cairo_int_96by64_32x64_divrem(det64x32_128(a_det, dx1, b_det, dx2), den_det);
    if qr.rem == den_det {
        return 0 as libc::c_int;
    }
    (*intersection).x.exactness = EXACT;
    if !(qr.rem == 0 as libc::c_int as libc::c_long) {
        if (den_det < 0 as libc::c_int as libc::c_long) as libc::c_int
            ^ (qr.rem < 0 as libc::c_int as libc::c_long) as libc::c_int != 0
        {
            qr.rem = -qr.rem;
        }
        qr.rem = qr.rem * 2 as libc::c_int as int64_t;
        if !(qr.rem < den_det) {
            qr
                .quo = qr.quo
                + (if qr.quo < 0 as libc::c_int as libc::c_long {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }) as int64_t;
        } else {
            (*intersection).x.exactness = INEXACT;
        }
    }
    (*intersection).x.ordinate = qr.quo as int32_t;
    qr = _cairo_int_96by64_32x64_divrem(det64x32_128(a_det, dy1, b_det, dy2), den_det);
    if qr.rem == den_det {
        return 0 as libc::c_int;
    }
    (*intersection).y.exactness = EXACT;
    if !(qr.rem == 0 as libc::c_int as libc::c_long) {
        if (den_det < 0 as libc::c_int as libc::c_long) as libc::c_int
            ^ (qr.rem < 0 as libc::c_int as libc::c_long) as libc::c_int != 0
        {
            qr.rem = -qr.rem;
        }
        qr.rem = qr.rem * 2 as libc::c_int as int64_t;
        if !(qr.rem < den_det) {
            qr
                .quo = qr.quo
                + (if qr.quo < 0 as libc::c_int as libc::c_long {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }) as int64_t;
        } else {
            (*intersection).y.exactness = INEXACT;
        }
    }
    (*intersection).y.ordinate = qr.quo as int32_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_bo_intersect_ordinate_32_compare(
    mut a: cairo_bo_intersect_ordinate_t,
    mut b: int32_t,
) -> libc::c_int {
    if a.ordinate > b {
        return 1 as libc::c_int;
    }
    if a.ordinate < b {
        return -(1 as libc::c_int);
    }
    return (INEXACT as libc::c_int as libc::c_uint == a.exactness as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_bo_edge_contains_intersect_point(
    mut edge: *mut cairo_bo_edge_t,
    mut point: *mut cairo_bo_intersect_point_t,
) -> cairo_bool_t {
    let mut cmp_top: libc::c_int = 0;
    let mut cmp_bottom: libc::c_int = 0;
    cmp_top = _cairo_bo_intersect_ordinate_32_compare((*point).y, (*edge).edge.top);
    cmp_bottom = _cairo_bo_intersect_ordinate_32_compare(
        (*point).y,
        (*edge).edge.bottom,
    );
    if cmp_top < 0 as libc::c_int || cmp_bottom > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if cmp_top > 0 as libc::c_int && cmp_bottom < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if cmp_top == 0 as libc::c_int {
        let mut top_x: cairo_fixed_t = 0;
        top_x = _line_compute_intersection_x_for_y(
            &mut (*edge).edge.line,
            (*edge).edge.top,
        );
        return (_cairo_bo_intersect_ordinate_32_compare((*point).x, top_x)
            > 0 as libc::c_int) as libc::c_int;
    } else {
        let mut bot_x: cairo_fixed_t = 0;
        bot_x = _line_compute_intersection_x_for_y(
            &mut (*edge).edge.line,
            (*edge).edge.bottom,
        );
        return (_cairo_bo_intersect_ordinate_32_compare((*point).x, bot_x)
            < 0 as libc::c_int) as libc::c_int;
    };
}
unsafe extern "C" fn _cairo_bo_edge_intersect(
    mut a: *mut cairo_bo_edge_t,
    mut b: *mut cairo_bo_edge_t,
    mut intersection: *mut cairo_bo_point32_t,
) -> cairo_bool_t {
    let mut quorem: cairo_bo_intersect_point_t = cairo_bo_intersect_point_t {
        x: cairo_bo_intersect_ordinate_t {
            ordinate: 0,
            exactness: EXACT,
        },
        y: cairo_bo_intersect_ordinate_t {
            ordinate: 0,
            exactness: EXACT,
        },
    };
    if intersect_lines(a, b, &mut quorem) == 0 {
        return 0 as libc::c_int;
    }
    if _cairo_bo_edge_contains_intersect_point(a, &mut quorem) == 0 {
        return 0 as libc::c_int;
    }
    if _cairo_bo_edge_contains_intersect_point(b, &mut quorem) == 0 {
        return 0 as libc::c_int;
    }
    (*intersection).x = quorem.x.ordinate;
    (*intersection).y = quorem.y.ordinate;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_bo_event_compare(
    mut a: *const cairo_bo_event_t,
    mut b: *const cairo_bo_event_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = _cairo_bo_point32_compare(&(*a).point, &(*b).point);
    if cmp != 0 {
        return cmp;
    }
    cmp = ((*a).type_0 as libc::c_uint).wrapping_sub((*b).type_0 as libc::c_uint)
        as libc::c_int;
    if cmp != 0 {
        return cmp;
    }
    return a.offset_from(b) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn _pqueue_init(mut pq: *mut pqueue_t) {
    (*pq)
        .max_size = (::std::mem::size_of::<[*mut cairo_bo_event_t; 1024]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong)
        as libc::c_int;
    (*pq).size = 0 as libc::c_int;
    let ref mut fresh13 = (*pq).elements;
    *fresh13 = ((*pq).elements_embedded).as_mut_ptr();
}
#[inline]
unsafe extern "C" fn _pqueue_fini(mut pq: *mut pqueue_t) {
    if (*pq).elements != ((*pq).elements_embedded).as_mut_ptr() {
        free((*pq).elements as *mut libc::c_void);
    }
}
unsafe extern "C" fn _pqueue_grow(mut pq: *mut pqueue_t) -> cairo_status_t {
    let mut new_elements: *mut *mut cairo_bo_event_t = 0 as *mut *mut cairo_bo_event_t;
    (*pq).max_size *= 2 as libc::c_int;
    if (*pq).elements == ((*pq).elements_embedded).as_mut_ptr() {
        new_elements = _cairo_malloc_ab(
            (*pq).max_size as size_t,
            ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
        ) as *mut *mut cairo_bo_event_t;
        if new_elements.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        memcpy(
            new_elements as *mut libc::c_void,
            ((*pq).elements_embedded).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[*mut cairo_bo_event_t; 1024]>() as libc::c_ulong,
        );
    } else {
        new_elements = _cairo_realloc_ab(
            (*pq).elements as *mut libc::c_void,
            (*pq).max_size as size_t,
            ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
        ) as *mut *mut cairo_bo_event_t;
        if new_elements.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    let ref mut fresh14 = (*pq).elements;
    *fresh14 = new_elements;
    return CAIRO_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn _pqueue_push(
    mut pq: *mut pqueue_t,
    mut event: *mut cairo_bo_event_t,
) -> cairo_status_t {
    let mut elements: *mut *mut cairo_bo_event_t = 0 as *mut *mut cairo_bo_event_t;
    let mut i: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    if (*pq).size + 1 as libc::c_int == (*pq).max_size {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _pqueue_grow(pq);
        if status as u64 != 0 {
            return status;
        }
    }
    elements = (*pq).elements;
    let ref mut fresh15 = (*pq).size;
    *fresh15 += 1;
    i = *fresh15;
    while i != 1 as libc::c_int
        && {
            parent = i >> 1 as libc::c_int;
            cairo_bo_event_compare(event, *elements.offset(parent as isize))
                < 0 as libc::c_int
        }
    {
        let ref mut fresh16 = *elements.offset(i as isize);
        *fresh16 = *elements.offset(parent as isize);
        i = parent;
    }
    let ref mut fresh17 = *elements.offset(i as isize);
    *fresh17 = event;
    return CAIRO_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn _pqueue_pop(mut pq: *mut pqueue_t) {
    let mut elements: *mut *mut cairo_bo_event_t = (*pq).elements;
    let mut tail: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let ref mut fresh18 = (*pq).size;
    let fresh19 = *fresh18;
    *fresh18 = *fresh18 - 1;
    tail = *elements.offset(fresh19 as isize);
    if (*pq).size == 0 as libc::c_int {
        let ref mut fresh20 = *elements.offset(1 as libc::c_int as isize);
        *fresh20 = 0 as *mut cairo_bo_event_t;
        return;
    }
    i = 1 as libc::c_int;
    loop {
        child = i << 1 as libc::c_int;
        if !(child <= (*pq).size) {
            break;
        }
        if child != (*pq).size
            && cairo_bo_event_compare(
                *elements.offset((child + 1 as libc::c_int) as isize),
                *elements.offset(child as isize),
            ) < 0 as libc::c_int
        {
            child += 1;
        }
        if cairo_bo_event_compare(*elements.offset(child as isize), tail)
            >= 0 as libc::c_int
        {
            break;
        }
        let ref mut fresh21 = *elements.offset(i as isize);
        *fresh21 = *elements.offset(child as isize);
        i = child;
    }
    let ref mut fresh22 = *elements.offset(i as isize);
    *fresh22 = tail;
}
#[inline]
unsafe extern "C" fn _cairo_bo_event_queue_insert(
    mut queue: *mut cairo_bo_event_queue_t,
    mut type_0: cairo_bo_event_type_t,
    mut e1: *mut cairo_bo_edge_t,
    mut e2: *mut cairo_bo_edge_t,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut event: *mut cairo_bo_queue_event_t = 0 as *mut cairo_bo_queue_event_t;
    event = _cairo_freepool_alloc(&mut (*queue).pool) as *mut cairo_bo_queue_event_t;
    if event.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*event).type_0 = type_0;
    let ref mut fresh23 = (*event).e1;
    *fresh23 = e1;
    let ref mut fresh24 = (*event).e2;
    *fresh24 = e2;
    (*event).point = *point;
    return _pqueue_push(&mut (*queue).pqueue, event as *mut cairo_bo_event_t);
}
unsafe extern "C" fn _cairo_bo_event_queue_delete(
    mut queue: *mut cairo_bo_event_queue_t,
    mut event: *mut cairo_bo_event_t,
) {
    _cairo_freepool_free(&mut (*queue).pool, event as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_bo_event_dequeue(
    mut event_queue: *mut cairo_bo_event_queue_t,
) -> *mut cairo_bo_event_t {
    let mut event: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    let mut cmp: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    event = *((*event_queue).pqueue.elements).offset(1 as libc::c_int as isize);
    cmp = *(*event_queue).start_events;
    if event.is_null()
        || !cmp.is_null() && cairo_bo_event_compare(cmp, event) < 0 as libc::c_int
    {
        event = cmp;
        let ref mut fresh25 = (*event_queue).start_events;
        *fresh25 = (*fresh25).offset(1);
    } else {
        _pqueue_pop(&mut (*event_queue).pqueue);
    }
    return event;
}
unsafe extern "C" fn _cairo_bo_event_queue_sort(
    mut base: *mut *mut cairo_bo_event_t,
    mut nmemb: libc::c_uint,
) {
    let mut gap: libc::c_uint = nmemb;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut swapped: libc::c_int = 0;
    loop {
        gap = _cairo_combsort_newgap(gap);
        swapped = (gap > 1 as libc::c_int as libc::c_uint) as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while i < nmemb.wrapping_sub(gap) {
            j = i.wrapping_add(gap);
            if cairo_bo_event_compare(*base.offset(i as isize), *base.offset(j as isize))
                > 0 as libc::c_int
            {
                let mut tmp: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
                tmp = *base.offset(i as isize);
                let ref mut fresh26 = *base.offset(i as isize);
                *fresh26 = *base.offset(j as isize);
                let ref mut fresh27 = *base.offset(j as isize);
                *fresh27 = tmp;
                swapped = 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        if !(swapped != 0) {
            break;
        }
    };
}
unsafe extern "C" fn _cairo_bo_event_queue_init(
    mut event_queue: *mut cairo_bo_event_queue_t,
    mut start_events: *mut *mut cairo_bo_event_t,
    mut num_events: libc::c_int,
) {
    _cairo_bo_event_queue_sort(start_events, num_events as libc::c_uint);
    let ref mut fresh28 = *start_events.offset(num_events as isize);
    *fresh28 = 0 as *mut cairo_bo_event_t;
    let ref mut fresh29 = (*event_queue).start_events;
    *fresh29 = start_events;
    _cairo_freepool_init(
        &mut (*event_queue).pool,
        ::std::mem::size_of::<cairo_bo_queue_event_t>() as libc::c_ulong as libc::c_uint,
    );
    _pqueue_init(&mut (*event_queue).pqueue);
    let ref mut fresh30 = *((*event_queue).pqueue.elements)
        .offset(1 as libc::c_int as isize);
    *fresh30 = 0 as *mut cairo_bo_event_t;
}
unsafe extern "C" fn _cairo_bo_event_queue_insert_stop(
    mut event_queue: *mut cairo_bo_event_queue_t,
    mut edge: *mut cairo_bo_edge_t,
) -> cairo_status_t {
    let mut point: cairo_bo_point32_t = cairo_bo_point32_t { x: 0, y: 0 };
    point.y = (*edge).edge.bottom;
    point.x = _line_compute_intersection_x_for_y(&mut (*edge).edge.line, point.y);
    return _cairo_bo_event_queue_insert(
        event_queue,
        CAIRO_BO_EVENT_TYPE_STOP,
        edge,
        0 as *mut cairo_bo_edge_t,
        &mut point,
    );
}
unsafe extern "C" fn _cairo_bo_event_queue_fini(
    mut event_queue: *mut cairo_bo_event_queue_t,
) {
    _pqueue_fini(&mut (*event_queue).pqueue);
    _cairo_freepool_fini(&mut (*event_queue).pool);
}
#[inline]
unsafe extern "C" fn _cairo_bo_event_queue_insert_if_intersect_below_current_y(
    mut event_queue: *mut cairo_bo_event_queue_t,
    mut left: *mut cairo_bo_edge_t,
    mut right: *mut cairo_bo_edge_t,
) -> cairo_status_t {
    let mut intersection: cairo_bo_point32_t = cairo_bo_point32_t { x: 0, y: 0 };
    if _line_equal(&mut (*left).edge.line, &mut (*right).edge.line) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if _slope_compare(left, right) <= 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_bo_edge_intersect(left, right, &mut intersection) == 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_bo_event_queue_insert(
        event_queue,
        CAIRO_BO_EVENT_TYPE_INTERSECTION,
        left,
        right,
        &mut intersection,
    );
}
unsafe extern "C" fn _cairo_bo_sweep_line_init(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
) {
    let ref mut fresh31 = (*sweep_line).head;
    *fresh31 = 0 as *mut cairo_bo_edge_t;
    (*sweep_line).current_y = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh32 = (*sweep_line).current_edge;
    *fresh32 = 0 as *mut cairo_bo_edge_t;
}
unsafe extern "C" fn _cairo_bo_sweep_line_insert(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut edge: *mut cairo_bo_edge_t,
) -> cairo_status_t {
    if !((*sweep_line).current_edge).is_null() {
        let mut prev: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
        let mut next: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
        let mut cmp: libc::c_int = 0;
        cmp = _cairo_bo_sweep_line_compare_edges(
            sweep_line,
            (*sweep_line).current_edge,
            edge,
        );
        if cmp < 0 as libc::c_int {
            prev = (*sweep_line).current_edge;
            next = (*prev).next;
            while !next.is_null()
                && _cairo_bo_sweep_line_compare_edges(sweep_line, next, edge)
                    < 0 as libc::c_int
            {
                prev = next;
                next = (*prev).next;
            }
            let ref mut fresh33 = (*prev).next;
            *fresh33 = edge;
            let ref mut fresh34 = (*edge).prev;
            *fresh34 = prev;
            let ref mut fresh35 = (*edge).next;
            *fresh35 = next;
            if !next.is_null() {
                let ref mut fresh36 = (*next).prev;
                *fresh36 = edge;
            }
        } else if cmp > 0 as libc::c_int {
            next = (*sweep_line).current_edge;
            prev = (*next).prev;
            while !prev.is_null()
                && _cairo_bo_sweep_line_compare_edges(sweep_line, prev, edge)
                    > 0 as libc::c_int
            {
                next = prev;
                prev = (*next).prev;
            }
            let ref mut fresh37 = (*next).prev;
            *fresh37 = edge;
            let ref mut fresh38 = (*edge).next;
            *fresh38 = next;
            let ref mut fresh39 = (*edge).prev;
            *fresh39 = prev;
            if !prev.is_null() {
                let ref mut fresh40 = (*prev).next;
                *fresh40 = edge;
            } else {
                let ref mut fresh41 = (*sweep_line).head;
                *fresh41 = edge;
            }
        } else {
            prev = (*sweep_line).current_edge;
            let ref mut fresh42 = (*edge).prev;
            *fresh42 = prev;
            let ref mut fresh43 = (*edge).next;
            *fresh43 = (*prev).next;
            if !((*prev).next).is_null() {
                let ref mut fresh44 = (*(*prev).next).prev;
                *fresh44 = edge;
            }
            let ref mut fresh45 = (*prev).next;
            *fresh45 = edge;
        }
    } else {
        let ref mut fresh46 = (*sweep_line).head;
        *fresh46 = edge;
    }
    let ref mut fresh47 = (*sweep_line).current_edge;
    *fresh47 = edge;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_bo_sweep_line_delete(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut edge: *mut cairo_bo_edge_t,
) {
    if !((*edge).prev).is_null() {
        let ref mut fresh48 = (*(*edge).prev).next;
        *fresh48 = (*edge).next;
    } else {
        let ref mut fresh49 = (*sweep_line).head;
        *fresh49 = (*edge).next;
    }
    if !((*edge).next).is_null() {
        let ref mut fresh50 = (*(*edge).next).prev;
        *fresh50 = (*edge).prev;
    }
    if (*sweep_line).current_edge == edge {
        let ref mut fresh51 = (*sweep_line).current_edge;
        *fresh51 = if !((*edge).prev).is_null() { (*edge).prev } else { (*edge).next };
    }
}
unsafe extern "C" fn _cairo_bo_sweep_line_swap(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut left: *mut cairo_bo_edge_t,
    mut right: *mut cairo_bo_edge_t,
) {
    if !((*left).prev).is_null() {
        let ref mut fresh52 = (*(*left).prev).next;
        *fresh52 = right;
    } else {
        let ref mut fresh53 = (*sweep_line).head;
        *fresh53 = right;
    }
    if !((*right).next).is_null() {
        let ref mut fresh54 = (*(*right).next).prev;
        *fresh54 = left;
    }
    let ref mut fresh55 = (*left).next;
    *fresh55 = (*right).next;
    let ref mut fresh56 = (*right).next;
    *fresh56 = left;
    let ref mut fresh57 = (*right).prev;
    *fresh57 = (*left).prev;
    let ref mut fresh58 = (*left).prev;
    *fresh58 = right;
}
#[inline]
unsafe extern "C" fn edges_colinear(
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
) -> cairo_bool_t {
    if _line_equal(&(*a).edge.line, &(*b).edge.line) != 0 {
        return 1 as libc::c_int;
    }
    if _slope_compare(a, b) != 0 {
        return 0 as libc::c_int;
    }
    if (*a).edge.line.p1.y == (*b).edge.line.p1.y {
        return ((*a).edge.line.p1.x == (*b).edge.line.p1.x) as libc::c_int
    } else if (*a).edge.line.p2.y == (*b).edge.line.p2.y {
        return ((*a).edge.line.p2.x == (*b).edge.line.p2.x) as libc::c_int
    } else if (*a).edge.line.p1.y < (*b).edge.line.p1.y {
        return (edge_compare_for_y_against_x(b, (*a).edge.line.p1.y, (*a).edge.line.p1.x)
            == 0 as libc::c_int) as libc::c_int
    } else {
        return (edge_compare_for_y_against_x(a, (*b).edge.line.p1.y, (*b).edge.line.p1.x)
            == 0 as libc::c_int) as libc::c_int
    };
}
unsafe extern "C" fn _cairo_bo_edge_end(
    mut left: *mut cairo_bo_edge_t,
    mut bot: int32_t,
    mut polygon: *mut cairo_polygon_t,
) {
    let mut d: *mut cairo_bo_deferred_t = &mut (*left).deferred;
    if (*d).top < bot {
        _cairo_polygon_add_line(
            polygon,
            &mut (*left).edge.line,
            (*d).top,
            bot,
            1 as libc::c_int,
        );
        _cairo_polygon_add_line(
            polygon,
            &mut (*(*d).right).edge.line,
            (*d).top,
            bot,
            -(1 as libc::c_int),
        );
    }
    let ref mut fresh59 = (*d).right;
    *fresh59 = 0 as *mut cairo_bo_edge_t;
}
#[inline]
unsafe extern "C" fn _cairo_bo_edge_start_or_continue(
    mut left: *mut cairo_bo_edge_t,
    mut right: *mut cairo_bo_edge_t,
    mut top: libc::c_int,
    mut polygon: *mut cairo_polygon_t,
) {
    if (*left).deferred.right == right {
        return;
    }
    if !((*left).deferred.right).is_null() {
        if !right.is_null() && edges_colinear((*left).deferred.right, right) != 0 {
            let ref mut fresh60 = (*left).deferred.right;
            *fresh60 = right;
            return;
        }
        _cairo_bo_edge_end(left, top, polygon);
    }
    if !right.is_null() && edges_colinear(left, right) == 0 {
        (*left).deferred.top = top;
        let ref mut fresh61 = (*left).deferred.right;
        *fresh61 = right;
    }
}
#[inline]
unsafe extern "C" fn _active_edges_to_polygon(
    mut left: *mut cairo_bo_edge_t,
    mut top: int32_t,
    mut fill_rule: cairo_fill_rule_t,
    mut polygon: *mut cairo_polygon_t,
) {
    let mut right: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut mask: libc::c_uint = 0;
    if fill_rule as libc::c_uint
        == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
    {
        mask = !(0 as libc::c_int) as libc::c_uint;
    } else {
        mask = 1 as libc::c_int as libc::c_uint;
    }
    while !left.is_null() {
        let mut in_out: libc::c_int = (*left).edge.dir;
        right = (*left).next;
        if ((*left).deferred.right).is_null() {
            while !right.is_null() && ((*right).deferred.right).is_null() {
                right = (*right).next;
            }
            if !right.is_null() && edges_colinear(left, right) != 0 {
                (*left).deferred = (*right).deferred;
                let ref mut fresh62 = (*right).deferred.right;
                *fresh62 = 0 as *mut cairo_bo_edge_t;
            }
        }
        right = (*left).next;
        while !right.is_null() {
            if !((*right).deferred.right).is_null() {
                _cairo_bo_edge_end(right, top, polygon);
            }
            in_out += (*right).edge.dir;
            if in_out as libc::c_uint & mask == 0 as libc::c_int as libc::c_uint {
                if ((*right).next).is_null() || edges_colinear(right, (*right).next) == 0
                {
                    break;
                }
            }
            right = (*right).next;
        }
        _cairo_bo_edge_start_or_continue(left, right, top, polygon);
        left = right;
        if !left.is_null() {
            left = (*left).next;
        }
    }
}
unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_bo_edges(
    mut start_events: *mut *mut cairo_bo_event_t,
    mut num_events: libc::c_int,
    mut fill_rule: cairo_fill_rule_t,
    mut polygon: *mut cairo_polygon_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut event_queue: cairo_bo_event_queue_t = cairo_bo_event_queue_t {
        pool: cairo_freepool_t {
            first_free_node: 0 as *mut cairo_freelist_node_t,
            pools: 0 as *mut cairo_freelist_pool_t,
            freepools: 0 as *mut cairo_freelist_pool_t,
            nodesize: 0,
            embedded_pool: cairo_freelist_pool_t {
                next: 0 as *mut cairo_freelist_pool_t,
                size: 0,
                rem: 0,
                data: 0 as *mut uint8_t,
            },
            embedded_data: [0; 1000],
        },
        pqueue: pqueue_t {
            size: 0,
            max_size: 0,
            elements: 0 as *mut *mut cairo_bo_event_t,
            elements_embedded: [0 as *mut cairo_bo_event_t; 1024],
        },
        start_events: 0 as *mut *mut cairo_bo_event_t,
    };
    let mut sweep_line: cairo_bo_sweep_line_t = cairo_bo_sweep_line_t {
        head: 0 as *mut cairo_bo_edge_t,
        current_y: 0,
        current_edge: 0 as *mut cairo_bo_edge_t,
    };
    let mut event: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    let mut left: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut right: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut e1: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut e2: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    _cairo_bo_event_queue_init(&mut event_queue, start_events, num_events);
    _cairo_bo_sweep_line_init(&mut sweep_line);
    loop {
        event = _cairo_bo_event_dequeue(&mut event_queue);
        if event.is_null() {
            break;
        }
        if (*event).point.y != sweep_line.current_y {
            _active_edges_to_polygon(
                sweep_line.head,
                sweep_line.current_y,
                fill_rule,
                polygon,
            );
            sweep_line.current_y = (*event).point.y;
        }
        match (*event).type_0 as libc::c_uint {
            2 => {
                e1 = &mut (*(event as *mut cairo_bo_start_event_t)).edge;
                status = _cairo_bo_sweep_line_insert(&mut sweep_line, e1);
                if status as u64 != 0 {
                    break;
                }
                status = _cairo_bo_event_queue_insert_stop(&mut event_queue, e1);
                if status as u64 != 0 {
                    break;
                }
                left = (*e1).prev;
                right = (*e1).next;
                if !left.is_null() {
                    status = _cairo_bo_event_queue_insert_if_intersect_below_current_y(
                        &mut event_queue,
                        left,
                        e1,
                    );
                    if status as u64 != 0 {
                        break;
                    }
                }
                if right.is_null() {
                    continue;
                }
                status = _cairo_bo_event_queue_insert_if_intersect_below_current_y(
                    &mut event_queue,
                    e1,
                    right,
                );
                if status as u64 != 0 {
                    break;
                }
            }
            0 => {
                e1 = (*(event as *mut cairo_bo_queue_event_t)).e1;
                _cairo_bo_event_queue_delete(&mut event_queue, event);
                left = (*e1).prev;
                right = (*e1).next;
                _cairo_bo_sweep_line_delete(&mut sweep_line, e1);
                if !((*e1).deferred.right).is_null() {
                    _cairo_bo_edge_end(e1, (*e1).edge.bottom, polygon);
                }
                if !(!left.is_null() && !right.is_null()) {
                    continue;
                }
                status = _cairo_bo_event_queue_insert_if_intersect_below_current_y(
                    &mut event_queue,
                    left,
                    right,
                );
                if status as u64 != 0 {
                    break;
                }
            }
            1 => {
                e1 = (*(event as *mut cairo_bo_queue_event_t)).e1;
                e2 = (*(event as *mut cairo_bo_queue_event_t)).e2;
                _cairo_bo_event_queue_delete(&mut event_queue, event);
                if e2 != (*e1).next {
                    continue;
                }
                left = (*e1).prev;
                right = (*e2).next;
                _cairo_bo_sweep_line_swap(&mut sweep_line, e1, e2);
                if !left.is_null() {
                    status = _cairo_bo_event_queue_insert_if_intersect_below_current_y(
                        &mut event_queue,
                        left,
                        e2,
                    );
                    if status as u64 != 0 {
                        break;
                    }
                }
                if right.is_null() {
                    continue;
                }
                status = _cairo_bo_event_queue_insert_if_intersect_below_current_y(
                    &mut event_queue,
                    e1,
                    right,
                );
                if status as u64 != 0 {
                    break;
                }
            }
            _ => {}
        }
    }
    _cairo_bo_event_queue_fini(&mut event_queue);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_polygon_reduce(
    mut polygon: *mut cairo_polygon_t,
    mut fill_rule: cairo_fill_rule_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut stack_events: [cairo_bo_start_event_t; 25] = [cairo_bo_start_event_t {
        type_0: CAIRO_BO_EVENT_TYPE_STOP,
        point: cairo_bo_point32_t { x: 0, y: 0 },
        edge: cairo_bo_edge_t {
            edge: cairo_edge_t {
                line: cairo_line_t {
                    p1: cairo_bo_point32_t { x: 0, y: 0 },
                    p2: cairo_bo_point32_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            },
            prev: 0 as *mut cairo_bo_edge_t,
            next: 0 as *mut cairo_bo_edge_t,
            deferred: cairo_bo_deferred_t {
                right: 0 as *mut cairo_bo_edge_t,
                top: 0,
            },
        },
    }; 25];
    let mut events: *mut cairo_bo_start_event_t = 0 as *mut cairo_bo_start_event_t;
    let mut stack_event_ptrs: [*mut cairo_bo_event_t; 26] = [0
        as *mut cairo_bo_event_t; 26];
    let mut event_ptrs: *mut *mut cairo_bo_event_t = 0 as *mut *mut cairo_bo_event_t;
    let mut num_limits: libc::c_int = 0;
    let mut num_events: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_events = (*polygon).num_edges;
    if 0 as libc::c_int == num_events {
        return CAIRO_STATUS_SUCCESS;
    }
    events = stack_events.as_mut_ptr();
    event_ptrs = stack_event_ptrs.as_mut_ptr();
    if num_events
        > (::std::mem::size_of::<[cairo_bo_start_event_t; 25]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<cairo_bo_start_event_t>() as libc::c_ulong,
            ) as libc::c_int
    {
        events = _cairo_malloc_ab_plus_c(
            num_events as size_t,
            (::std::mem::size_of::<cairo_bo_start_event_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
                ),
            ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
        ) as *mut cairo_bo_start_event_t;
        if events.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        event_ptrs = events.offset(num_events as isize) as *mut *mut cairo_bo_event_t;
    }
    i = 0 as libc::c_int;
    while i < num_events {
        let ref mut fresh63 = *event_ptrs.offset(i as isize);
        *fresh63 = &mut *events.offset(i as isize) as *mut cairo_bo_start_event_t
            as *mut cairo_bo_event_t;
        (*events.offset(i as isize)).type_0 = CAIRO_BO_EVENT_TYPE_START;
        (*events.offset(i as isize))
            .point
            .y = (*((*polygon).edges).offset(i as isize)).top;
        (*events.offset(i as isize))
            .point
            .x = _line_compute_intersection_x_for_y(
            &mut (*((*polygon).edges).offset(i as isize)).line,
            (*events.offset(i as isize)).point.y,
        );
        (*events.offset(i as isize)).edge.edge = *((*polygon).edges).offset(i as isize);
        let ref mut fresh64 = (*events.offset(i as isize)).edge.deferred.right;
        *fresh64 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh65 = (*events.offset(i as isize)).edge.prev;
        *fresh65 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh66 = (*events.offset(i as isize)).edge.next;
        *fresh66 = 0 as *mut cairo_bo_edge_t;
        i += 1;
    }
    num_limits = (*polygon).num_limits;
    (*polygon).num_limits = 0 as libc::c_int;
    (*polygon).num_edges = 0 as libc::c_int;
    status = _cairo_bentley_ottmann_tessellate_bo_edges(
        event_ptrs,
        num_events,
        fill_rule,
        polygon,
    );
    (*polygon).num_limits = num_limits;
    if events != stack_events.as_mut_ptr() {
        free(events as *mut libc::c_void);
    }
    return status;
}
