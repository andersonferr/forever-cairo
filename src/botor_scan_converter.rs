use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_int_96by64_32x64_divrem(
        num: cairo_int128_t,
        den: cairo_int64_t,
    ) -> cairo_quorem64_t;
    fn _cairo_freepool_init(freepool: *mut cairo_freepool_t, nodesize: libc::c_uint);
    fn _cairo_freepool_fini(freepool: *mut cairo_freepool_t);
    fn _cairo_freepool_alloc_from_new_pool(
        freepool: *mut cairo_freepool_t,
    ) -> *mut libc::c_void;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type __int128_t = i128;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type cairo_bool_t = libc::c_int;
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
pub type uint32_t = __uint32_t;
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub type cairo_fixed_unsigned_t = uint32_t;
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
pub struct _cairo_botor_scan_converter {
    pub base: cairo_scan_converter_t,
    pub extents: cairo_box_t,
    pub fill_rule: cairo_fill_rule_t,
    pub xmin: libc::c_int,
    pub xmax: libc::c_int,
    pub chunks: _cairo_botor_scan_converter_chunk,
    pub tail: *mut _cairo_botor_scan_converter_chunk,
    pub buf: [libc::c_char; 2048],
    pub num_edges: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_botor_scan_converter_chunk {
    pub next: *mut _cairo_botor_scan_converter_chunk,
    pub base: *mut libc::c_void,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_botor_scan_converter_t = _cairo_botor_scan_converter;
pub type edge_t = edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge {
    pub link: cairo_list_t,
    pub edge: cairo_edge_t,
    pub dy: cairo_fixed_t,
    pub x: quorem,
    pub dxdy: quorem,
    pub dxdy_full: quorem,
    pub vertical: cairo_bool_t,
    pub flags: libc::c_uint,
    pub current_sign: libc::c_int,
    pub runs: *mut run,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct run {
    pub next: *mut run,
    pub sign: libc::c_int,
    pub y: cairo_fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quorem {
    pub quo: cairo_fixed_t,
    pub rem: cairo_fixed_t,
}
pub type start_event_t = _start_event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _start_event {
    pub y: cairo_fixed_t,
    pub type_0: event_type_t,
    pub edge: *mut edge_t,
}
pub type event_type_t = libc::c_uint;
pub const EVENT_TYPE_START: event_type_t = 2;
pub const EVENT_TYPE_INTERSECTION: event_type_t = 1;
pub const EVENT_TYPE_STOP: event_type_t = 0;
pub type event_t = _event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _event {
    pub y: cairo_fixed_t,
    pub type_0: event_type_t,
}
pub type sweep_line_t = _sweep_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sweep_line {
    pub active: cairo_list_t,
    pub stopped: cairo_list_t,
    pub insert_cursor: *mut cairo_list_t,
    pub is_vertical: cairo_bool_t,
    pub current_row: cairo_fixed_t,
    pub current_subrow: cairo_fixed_t,
    pub coverage: coverage,
    pub queue: event_queue,
    pub runs: cairo_freepool_t,
    pub unwind: jmp_buf,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_queue {
    pub pq: pqueue_t,
    pub start_events: *mut *mut event_t,
    pub pool: cairo_freepool_t,
}
pub type pqueue_t = _pqueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pqueue {
    pub size: libc::c_int,
    pub max_size: libc::c_int,
    pub elements: *mut *mut event_t,
    pub elements_embedded: [*mut event_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coverage {
    pub head: cell,
    pub tail: cell,
    pub cursor: *mut cell,
    pub count: libc::c_int,
    pub pool: cairo_freepool_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub prev: *mut cell,
    pub next: *mut cell,
    pub x: libc::c_int,
    pub uncovered_area: libc::c_int,
    pub covered_height: libc::c_int,
}
pub const START: C2RustUnnamed_2 = 1;
pub type queue_event_t = _queue_event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _queue_event {
    pub y: cairo_fixed_t,
    pub type_0: event_type_t,
    pub e1: *mut edge_t,
    pub e2: *mut edge_t,
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
pub const STOP: C2RustUnnamed_2 = 2;
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
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_ceil(mut f: cairo_fixed_t) -> libc::c_int {
    if f > 0 as libc::c_int {
        return (f - 1 as libc::c_int >> 8 as libc::c_int) + 1 as libc::c_int
    } else {
        return -((f as cairo_fixed_unsigned_t).wrapping_neg() as cairo_fixed_t
            >> 8 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
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
unsafe extern "C" fn _cairo_fixed_fractional_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
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
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh8 = (*entry).next;
    *fresh8 = entry;
    let ref mut fresh9 = (*entry).prev;
    *fresh9 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh10 = (*next).prev;
    *fresh10 = entry;
    let ref mut fresh11 = (*entry).next;
    *fresh11 = next;
    let ref mut fresh12 = (*entry).prev;
    *fresh12 = prev;
    let ref mut fresh13 = (*prev).next;
    *fresh13 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn cairo_list_add_tail(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, (*head).prev, head);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh14 = (*next).prev;
    *fresh14 = prev;
    let ref mut fresh15 = (*prev).next;
    *fresh15 = next;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn cairo_list_del(mut entry: *mut cairo_list_t) {
    _cairo_list_del(entry);
    cairo_list_init(entry);
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_reset(mut freepool: *mut cairo_freepool_t) {
    while (*freepool).pools
        != &mut (*freepool).embedded_pool as *mut cairo_freelist_pool_t
    {
        let mut pool: *mut cairo_freelist_pool_t = (*freepool).pools;
        let ref mut fresh16 = (*freepool).pools;
        *fresh16 = (*pool).next;
        let ref mut fresh17 = (*pool).next;
        *fresh17 = (*freepool).freepools;
        let ref mut fresh18 = (*freepool).freepools;
        *fresh18 = pool;
    }
    (*freepool)
        .embedded_pool
        .rem = ::std::mem::size_of::<[uint8_t; 1000]>() as libc::c_ulong as libc::c_uint;
    let ref mut fresh19 = (*freepool).embedded_pool.data;
    *fresh19 = ((*freepool).embedded_data).as_mut_ptr();
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
    let ref mut fresh20 = (*pool).data;
    *fresh20 = (*fresh20).offset((*freepool).nodesize as isize);
    let ref mut fresh21 = (*pool).rem;
    *fresh21 = (*fresh21).wrapping_sub((*freepool).nodesize);
    return ptr as *mut libc::c_void;
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
    let ref mut fresh22 = (*freepool).first_free_node;
    *fresh22 = (*node).next;
    return node as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_free(
    mut freepool: *mut cairo_freepool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut node: *mut cairo_freelist_node_t = ptr as *mut cairo_freelist_node_t;
    let ref mut fresh23 = (*node).next;
    *fresh23 = (*freepool).first_free_node;
    let ref mut fresh24 = (*freepool).first_free_node;
    *fresh24 = node;
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
#[inline(always)]
unsafe extern "C" fn floored_divrem(mut a: libc::c_int, mut b: libc::c_int) -> quorem {
    let mut qr: quorem = quorem { quo: 0, rem: 0 };
    qr.quo = a / b;
    qr.rem = a % b;
    if a ^ b < 0 as libc::c_int && qr.rem != 0 {
        qr.quo -= 1;
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
    qr.quo = (xa / b as libc::c_longlong) as cairo_fixed_t;
    qr.rem = (xa % b as libc::c_longlong) as cairo_fixed_t;
    if (xa >= 0 as libc::c_int as libc::c_longlong) as libc::c_int
        != (b >= 0 as libc::c_int) as libc::c_int && qr.rem != 0
    {
        qr.quo -= 1;
        qr.rem += b;
    }
    return qr;
}
unsafe extern "C" fn line_compute_intersection_x_for_y(
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
unsafe extern "C" fn edges_compare_x_for_y_general(
    mut a: *const cairo_edge_t,
    mut b: *const cairo_edge_t,
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
    if (*a).line.p1.x < (*a).line.p2.x {
        amin = (*a).line.p1.x;
        amax = (*a).line.p2.x;
    } else {
        amin = (*a).line.p2.x;
        amax = (*a).line.p1.x;
    }
    if (*b).line.p1.x < (*b).line.p2.x {
        bmin = (*b).line.p1.x;
        bmax = (*b).line.p2.x;
    } else {
        bmin = (*b).line.p2.x;
        bmax = (*b).line.p1.x;
    }
    if amax < bmin {
        return -(1 as libc::c_int);
    }
    if amin > bmax {
        return 1 as libc::c_int;
    }
    ady = (*a).line.p2.y - (*a).line.p1.y;
    adx = (*a).line.p2.x - (*a).line.p1.x;
    if adx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_ADX as libc::c_int) as libc::c_uint);
    }
    bdy = (*b).line.p2.y - (*b).line.p1.y;
    bdx = (*b).line.p2.x - (*b).line.p1.x;
    if bdx == 0 as libc::c_int {
        have_dx_adx_bdx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(have_dx_adx_bdx as libc::c_uint & !(HAVE_BDX as libc::c_int) as libc::c_uint);
    }
    dx = (*a).line.p1.x - (*b).line.p1.x;
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
            } else if (*a).line.p1.y == (*b).line.p1.y {
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
                    * (y - (*a).line.p1.y) as int64_t as i128
                    == (bdx as int64_t * ady as libc::c_long) as int128_t
                        * (y - (*b).line.p1.y) as int64_t as i128
                {
                    0 as libc::c_int
                } else if ((adx as int64_t * bdy as libc::c_long) as int128_t
                    * (y - (*a).line.p1.y) as int64_t as i128)
                    < (bdx as int64_t * ady as libc::c_long) as int128_t
                        * (y - (*b).line.p1.y) as int64_t as i128
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
                dy_adx = ((*a).line.p1.y - y) as int64_t * adx as libc::c_long;
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
                dy_bdx = (y - (*b).line.p1.y) as int64_t * bdx as libc::c_long;
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
                    * (y - (*b).line.p1.y) as int64_t as i128
                    - (adx as int64_t * bdy as libc::c_long) as int128_t
                        * (y - (*a).line.p1.y) as int64_t as i128
            {
                0 as libc::c_int
            } else if ((ady as int64_t * bdy as libc::c_long) as int128_t
                * dx as int64_t as i128)
                < (bdx as int64_t * ady as libc::c_long) as int128_t
                    * (y - (*b).line.p1.y) as int64_t as i128
                    - (adx as int64_t * bdy as libc::c_long) as int128_t
                        * (y - (*a).line.p1.y) as int64_t as i128
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
    mut a: *const cairo_edge_t,
    mut y: int32_t,
    mut x: int32_t,
) -> libc::c_int {
    let mut adx: int32_t = 0;
    let mut ady: int32_t = 0;
    let mut dx: int32_t = 0;
    let mut dy: int32_t = 0;
    let mut L: cairo_int64_t = 0;
    let mut R: cairo_int64_t = 0;
    if (*a).line.p1.x <= (*a).line.p2.x {
        if x < (*a).line.p1.x {
            return 1 as libc::c_int;
        }
        if x > (*a).line.p2.x {
            return -(1 as libc::c_int);
        }
    } else {
        if x < (*a).line.p2.x {
            return 1 as libc::c_int;
        }
        if x > (*a).line.p1.x {
            return -(1 as libc::c_int);
        }
    }
    adx = (*a).line.p2.x - (*a).line.p1.x;
    dx = x - (*a).line.p1.x;
    if adx == 0 as libc::c_int {
        return -dx;
    }
    if dx == 0 as libc::c_int || adx ^ dx < 0 as libc::c_int {
        return adx;
    }
    dy = y - (*a).line.p1.y;
    ady = (*a).line.p2.y - (*a).line.p1.y;
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
    mut a: *const cairo_edge_t,
    mut b: *const cairo_edge_t,
    mut y: int32_t,
) -> libc::c_int {
    let mut have_ax_bx: C2RustUnnamed_1 = HAVE_BOTH;
    let mut ax: int32_t = 0 as libc::c_int;
    let mut bx: int32_t = 0 as libc::c_int;
    if y == (*a).line.p1.y {
        ax = (*a).line.p1.x;
    } else if y == (*a).line.p2.y {
        ax = (*a).line.p2.x;
    } else {
        have_ax_bx = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_1,
        >(have_ax_bx as libc::c_uint & !(HAVE_AX as libc::c_int) as libc::c_uint);
    }
    if y == (*b).line.p1.y {
        bx = (*b).line.p1.x;
    } else if y == (*b).line.p2.y {
        bx = (*b).line.p2.x;
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
unsafe extern "C" fn slope_compare(
    mut a: *const edge_t,
    mut b: *const edge_t,
) -> libc::c_int {
    let mut L: cairo_int64_t = 0;
    let mut R: cairo_int64_t = 0;
    let mut cmp: libc::c_int = 0;
    cmp = (*a).dxdy.quo - (*b).dxdy.quo;
    if cmp != 0 {
        return cmp;
    }
    if (*a).dxdy.rem == 0 as libc::c_int {
        return -(*b).dxdy.rem;
    }
    if (*b).dxdy.rem == 0 as libc::c_int {
        return (*a).dxdy.rem;
    }
    L = (*b).dy as int64_t * (*a).dxdy.rem as libc::c_long;
    R = (*a).dy as int64_t * (*b).dxdy.rem as libc::c_long;
    return if L == R {
        0 as libc::c_int
    } else if L < R {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn line_equal(
    mut a: *const cairo_line_t,
    mut b: *const cairo_line_t,
) -> libc::c_int {
    return ((*a).p1.x == (*b).p1.x && (*a).p1.y == (*b).p1.y && (*a).p2.x == (*b).p2.x
        && (*a).p2.y == (*b).p2.y) as libc::c_int;
}
#[inline]
unsafe extern "C" fn sweep_line_compare_edges(
    mut a: *const edge_t,
    mut b: *const edge_t,
    mut y: cairo_fixed_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    if line_equal(&(*a).edge.line, &(*b).edge.line) != 0 {
        return 0 as libc::c_int;
    }
    cmp = edges_compare_x_for_y(&(*a).edge, &(*b).edge, y);
    if cmp != 0 {
        return cmp;
    }
    return slope_compare(a, b);
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
    mut a: *const edge_t,
    mut b: *const edge_t,
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
        qr
            .quo = qr.quo
            + (if qr.quo < 0 as libc::c_int as libc::c_long {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }) as int64_t;
        (*intersection).y.exactness = INEXACT;
    }
    (*intersection).y.ordinate = qr.quo as int32_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn bo_intersect_ordinate_32_compare(
    mut a: int32_t,
    mut b: int32_t,
    mut exactness: libc::c_int,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = a - b;
    if cmp != 0 {
        return cmp;
    }
    return -((INEXACT as libc::c_int == exactness) as libc::c_int);
}
unsafe extern "C" fn bo_edge_contains_intersect_point(
    mut edge: *const edge_t,
    mut point: *mut cairo_bo_intersect_point_t,
) -> cairo_bool_t {
    let mut cmp_top: libc::c_int = 0;
    let mut cmp_bottom: libc::c_int = 0;
    cmp_top = bo_intersect_ordinate_32_compare(
        (*point).y.ordinate,
        (*edge).edge.top,
        (*point).y.exactness as libc::c_int,
    );
    if cmp_top < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    cmp_bottom = bo_intersect_ordinate_32_compare(
        (*point).y.ordinate,
        (*edge).edge.bottom,
        (*point).y.exactness as libc::c_int,
    );
    if cmp_bottom > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if cmp_top > 0 as libc::c_int && cmp_bottom < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if cmp_top == 0 as libc::c_int {
        let mut top_x: cairo_fixed_t = 0;
        top_x = line_compute_intersection_x_for_y(&(*edge).edge.line, (*edge).edge.top);
        return (bo_intersect_ordinate_32_compare(
            top_x,
            (*point).x.ordinate,
            (*point).x.exactness as libc::c_int,
        ) < 0 as libc::c_int) as libc::c_int;
    } else {
        let mut bot_x: cairo_fixed_t = 0;
        bot_x = line_compute_intersection_x_for_y(
            &(*edge).edge.line,
            (*edge).edge.bottom,
        );
        return (bo_intersect_ordinate_32_compare(
            (*point).x.ordinate,
            bot_x,
            (*point).x.exactness as libc::c_int,
        ) < 0 as libc::c_int) as libc::c_int;
    };
}
unsafe extern "C" fn edge_intersect(
    mut a: *const edge_t,
    mut b: *const edge_t,
    mut intersection: *mut cairo_point_t,
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
    if (*a).edge.top != (*a).edge.line.p1.y || (*a).edge.bottom != (*a).edge.line.p2.y {
        if bo_edge_contains_intersect_point(a, &mut quorem) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*b).edge.top != (*b).edge.line.p1.y || (*b).edge.bottom != (*b).edge.line.p2.y {
        if bo_edge_contains_intersect_point(b, &mut quorem) == 0 {
            return 0 as libc::c_int;
        }
    }
    (*intersection).x = quorem.x.ordinate;
    (*intersection).y = quorem.y.ordinate;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn event_compare(
    mut a: *const event_t,
    mut b: *const event_t,
) -> libc::c_int {
    return (*a).y - (*b).y;
}
unsafe extern "C" fn pqueue_init(mut pq: *mut pqueue_t) {
    (*pq)
        .max_size = (::std::mem::size_of::<[*mut event_t; 1024]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut event_t>() as libc::c_ulong)
        as libc::c_int;
    (*pq).size = 0 as libc::c_int;
    let ref mut fresh25 = (*pq).elements;
    *fresh25 = ((*pq).elements_embedded).as_mut_ptr();
}
unsafe extern "C" fn pqueue_fini(mut pq: *mut pqueue_t) {
    if (*pq).elements != ((*pq).elements_embedded).as_mut_ptr() {
        free((*pq).elements as *mut libc::c_void);
    }
}
unsafe extern "C" fn pqueue_grow(mut pq: *mut pqueue_t) -> cairo_bool_t {
    let mut new_elements: *mut *mut event_t = 0 as *mut *mut event_t;
    (*pq).max_size *= 2 as libc::c_int;
    if (*pq).elements == ((*pq).elements_embedded).as_mut_ptr() {
        new_elements = _cairo_malloc_ab(
            (*pq).max_size as size_t,
            ::std::mem::size_of::<*mut event_t>() as libc::c_ulong,
        ) as *mut *mut event_t;
        if new_elements.is_null() {
            return 0 as libc::c_int;
        }
        memcpy(
            new_elements as *mut libc::c_void,
            ((*pq).elements_embedded).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[*mut event_t; 1024]>() as libc::c_ulong,
        );
    } else {
        new_elements = _cairo_realloc_ab(
            (*pq).elements as *mut libc::c_void,
            (*pq).max_size as size_t,
            ::std::mem::size_of::<*mut event_t>() as libc::c_ulong,
        ) as *mut *mut event_t;
        if new_elements.is_null() {
            return 0 as libc::c_int;
        }
    }
    let ref mut fresh26 = (*pq).elements;
    *fresh26 = new_elements;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn pqueue_push(
    mut sweep_line: *mut sweep_line_t,
    mut event: *mut event_t,
) {
    let mut elements: *mut *mut event_t = 0 as *mut *mut event_t;
    let mut i: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    if (*sweep_line).queue.pq.size + 1 as libc::c_int == (*sweep_line).queue.pq.max_size
    {
        if pqueue_grow(&mut (*sweep_line).queue.pq) == 0 {
            longjmp(
                ((*sweep_line).unwind).as_mut_ptr(),
                _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
            );
        }
    }
    elements = (*sweep_line).queue.pq.elements;
    let ref mut fresh27 = (*sweep_line).queue.pq.size;
    *fresh27 += 1;
    i = *fresh27;
    while i != 1 as libc::c_int
        && {
            parent = i >> 1 as libc::c_int;
            event_compare(event, *elements.offset(parent as isize)) < 0 as libc::c_int
        }
    {
        let ref mut fresh28 = *elements.offset(i as isize);
        *fresh28 = *elements.offset(parent as isize);
        i = parent;
    }
    let ref mut fresh29 = *elements.offset(i as isize);
    *fresh29 = event;
}
#[inline]
unsafe extern "C" fn pqueue_pop(mut pq: *mut pqueue_t) {
    let mut elements: *mut *mut event_t = (*pq).elements;
    let mut tail: *mut event_t = 0 as *mut event_t;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let ref mut fresh30 = (*pq).size;
    let fresh31 = *fresh30;
    *fresh30 = *fresh30 - 1;
    tail = *elements.offset(fresh31 as isize);
    if (*pq).size == 0 as libc::c_int {
        let ref mut fresh32 = *elements.offset(1 as libc::c_int as isize);
        *fresh32 = 0 as *mut event_t;
        return;
    }
    i = 1 as libc::c_int;
    loop {
        child = i << 1 as libc::c_int;
        if !(child <= (*pq).size) {
            break;
        }
        if child != (*pq).size
            && event_compare(
                *elements.offset((child + 1 as libc::c_int) as isize),
                *elements.offset(child as isize),
            ) < 0 as libc::c_int
        {
            child += 1;
        }
        if event_compare(*elements.offset(child as isize), tail) >= 0 as libc::c_int {
            break;
        }
        let ref mut fresh33 = *elements.offset(i as isize);
        *fresh33 = *elements.offset(child as isize);
        i = child;
    }
    let ref mut fresh34 = *elements.offset(i as isize);
    *fresh34 = tail;
}
#[inline]
unsafe extern "C" fn event_insert(
    mut sweep_line: *mut sweep_line_t,
    mut type_0: event_type_t,
    mut e1: *mut edge_t,
    mut e2: *mut edge_t,
    mut y: cairo_fixed_t,
) {
    let mut event: *mut queue_event_t = 0 as *mut queue_event_t;
    event = _cairo_freepool_alloc(&mut (*sweep_line).queue.pool) as *mut queue_event_t;
    if event.is_null() {
        longjmp(
            ((*sweep_line).unwind).as_mut_ptr(),
            _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
        );
    }
    (*event).y = y;
    (*event).type_0 = type_0;
    let ref mut fresh35 = (*event).e1;
    *fresh35 = e1;
    let ref mut fresh36 = (*event).e2;
    *fresh36 = e2;
    pqueue_push(sweep_line, event as *mut event_t);
}
unsafe extern "C" fn event_delete(
    mut sweep_line: *mut sweep_line_t,
    mut event: *mut event_t,
) {
    _cairo_freepool_free(&mut (*sweep_line).queue.pool, event as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn event_next(mut sweep_line: *mut sweep_line_t) -> *mut event_t {
    let mut event: *mut event_t = 0 as *mut event_t;
    let mut cmp: *mut event_t = 0 as *mut event_t;
    event = *((*sweep_line).queue.pq.elements).offset(1 as libc::c_int as isize);
    cmp = *(*sweep_line).queue.start_events;
    if event.is_null() || !cmp.is_null() && event_compare(cmp, event) < 0 as libc::c_int
    {
        event = cmp;
        let ref mut fresh37 = (*sweep_line).queue.start_events;
        *fresh37 = (*fresh37).offset(1);
    } else {
        pqueue_pop(&mut (*sweep_line).queue.pq);
    }
    return event;
}
unsafe extern "C" fn start_event_sort(
    mut base: *mut *mut event_t,
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
            if event_compare(*base.offset(i as isize), *base.offset(j as isize))
                > 0 as libc::c_int
            {
                let mut tmp: *mut event_t = 0 as *mut event_t;
                tmp = *base.offset(i as isize);
                let ref mut fresh38 = *base.offset(i as isize);
                *fresh38 = *base.offset(j as isize);
                let ref mut fresh39 = *base.offset(j as isize);
                *fresh39 = tmp;
                swapped = 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        if !(swapped != 0) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn event_insert_stop(
    mut sweep_line: *mut sweep_line_t,
    mut edge: *mut edge_t,
) {
    event_insert(
        sweep_line,
        EVENT_TYPE_STOP,
        edge,
        0 as *mut edge_t,
        (*edge).edge.bottom,
    );
}
#[inline]
unsafe extern "C" fn event_insert_if_intersect_below_current_y(
    mut sweep_line: *mut sweep_line_t,
    mut left: *mut edge_t,
    mut right: *mut edge_t,
) {
    let mut intersection: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    if (*left).edge.line.p1.x == (*right).edge.line.p1.x
        && (*left).edge.line.p1.y == (*right).edge.line.p1.y
    {
        return;
    }
    if (*left).edge.line.p2.x == (*right).edge.line.p2.x
        && (*left).edge.line.p2.y == (*right).edge.line.p2.y
    {
        return;
    }
    if slope_compare(left, right) <= 0 as libc::c_int {
        return;
    }
    if edge_intersect(left, right, &mut intersection) == 0 {
        return;
    }
    event_insert(sweep_line, EVENT_TYPE_INTERSECTION, left, right, intersection.y);
}
#[inline]
unsafe extern "C" fn link_to_edge(mut link: *mut cairo_list_t) -> *mut edge_t {
    return link as *mut edge_t;
}
unsafe extern "C" fn sweep_line_insert(
    mut sweep_line: *mut sweep_line_t,
    mut edge: *mut edge_t,
) {
    let mut pos: *mut cairo_list_t = 0 as *mut cairo_list_t;
    let mut y: cairo_fixed_t = (*sweep_line).current_subrow;
    pos = (*sweep_line).insert_cursor;
    if pos == &mut (*sweep_line).active as *mut cairo_list_t {
        pos = (*sweep_line).active.next;
    }
    if pos != &mut (*sweep_line).active as *mut cairo_list_t {
        let mut cmp: libc::c_int = 0;
        cmp = sweep_line_compare_edges(link_to_edge(pos), edge, y);
        if cmp < 0 as libc::c_int {
            while (*pos).next != &mut (*sweep_line).active as *mut cairo_list_t
                && sweep_line_compare_edges(link_to_edge((*pos).next), edge, y)
                    < 0 as libc::c_int
            {
                pos = (*pos).next;
            }
        } else if cmp > 0 as libc::c_int {
            loop {
                pos = (*pos).prev;
                if !(pos != &mut (*sweep_line).active as *mut cairo_list_t
                    && sweep_line_compare_edges(link_to_edge(pos), edge, y)
                        > 0 as libc::c_int)
                {
                    break;
                }
            }
        }
    }
    cairo_list_add(&mut (*edge).link, pos);
    let ref mut fresh40 = (*sweep_line).insert_cursor;
    *fresh40 = &mut (*edge).link;
}
#[inline]
unsafe extern "C" fn coverage_rewind(mut cells: *mut coverage) {
    let ref mut fresh41 = (*cells).cursor;
    *fresh41 = &mut (*cells).head;
}
unsafe extern "C" fn coverage_init(mut cells: *mut coverage) {
    _cairo_freepool_init(
        &mut (*cells).pool,
        ::std::mem::size_of::<cell>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh42 = (*cells).head.prev;
    *fresh42 = 0 as *mut cell;
    let ref mut fresh43 = (*cells).head.next;
    *fresh43 = &mut (*cells).tail;
    (*cells).head.x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh44 = (*cells).tail.prev;
    *fresh44 = &mut (*cells).head;
    let ref mut fresh45 = (*cells).tail.next;
    *fresh45 = 0 as *mut cell;
    (*cells).tail.x = 2147483647 as libc::c_int;
    (*cells).count = 0 as libc::c_int;
    coverage_rewind(cells);
}
unsafe extern "C" fn coverage_fini(mut cells: *mut coverage) {
    _cairo_freepool_fini(&mut (*cells).pool);
}
#[inline]
unsafe extern "C" fn coverage_reset(mut cells: *mut coverage) {
    let ref mut fresh46 = (*cells).head.next;
    *fresh46 = &mut (*cells).tail;
    let ref mut fresh47 = (*cells).tail.prev;
    *fresh47 = &mut (*cells).head;
    (*cells).count = 0 as libc::c_int;
    _cairo_freepool_reset(&mut (*cells).pool);
    coverage_rewind(cells);
}
unsafe extern "C" fn coverage_alloc(
    mut sweep_line: *mut sweep_line_t,
    mut tail: *mut cell,
    mut x: libc::c_int,
) -> *mut cell {
    let mut cell: *mut cell = 0 as *mut cell;
    cell = _cairo_freepool_alloc(&mut (*sweep_line).coverage.pool) as *mut cell;
    if cell.is_null() {
        longjmp(
            ((*sweep_line).unwind).as_mut_ptr(),
            _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
        );
    }
    let ref mut fresh48 = (*(*tail).prev).next;
    *fresh48 = cell;
    let ref mut fresh49 = (*cell).prev;
    *fresh49 = (*tail).prev;
    let ref mut fresh50 = (*cell).next;
    *fresh50 = tail;
    let ref mut fresh51 = (*tail).prev;
    *fresh51 = cell;
    (*cell).x = x;
    (*cell).uncovered_area = 0 as libc::c_int;
    (*cell).covered_height = 0 as libc::c_int;
    let ref mut fresh52 = (*sweep_line).coverage.count;
    *fresh52 += 1;
    return cell;
}
#[inline]
unsafe extern "C" fn coverage_find(
    mut sweep_line: *mut sweep_line_t,
    mut x: libc::c_int,
) -> *mut cell {
    let mut cell: *mut cell = 0 as *mut cell;
    cell = (*sweep_line).coverage.cursor;
    if (*cell).x > x {
        while !((*(*cell).prev).x < x) {
            cell = (*cell).prev;
        }
    } else {
        if (*cell).x == x {
            return cell;
        }
        loop {
            cell = (*cell).next;
            if (*cell).x >= x {
                break;
            }
            cell = (*cell).next;
            if (*cell).x >= x {
                break;
            }
            cell = (*cell).next;
            if (*cell).x >= x {
                break;
            }
        }
    }
    if (*cell).x != x {
        cell = coverage_alloc(sweep_line, cell, x);
    }
    let ref mut fresh53 = (*sweep_line).coverage.cursor;
    *fresh53 = cell;
    return *fresh53;
}
unsafe extern "C" fn coverage_render_cells(
    mut sweep_line: *mut sweep_line_t,
    mut left: cairo_fixed_t,
    mut right: cairo_fixed_t,
    mut y1: cairo_fixed_t,
    mut y2: cairo_fixed_t,
    mut sign: libc::c_int,
) {
    let mut fx1: libc::c_int = 0;
    let mut fx2: libc::c_int = 0;
    let mut ix1: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    dx = right - left;
    if dx >= 0 as libc::c_int {
        ix1 = _cairo_fixed_integer_part(left);
        fx1 = _cairo_fixed_fractional_part(left);
        ix2 = _cairo_fixed_integer_part(right);
        fx2 = _cairo_fixed_fractional_part(right);
        dy = y2 - y1;
    } else {
        ix1 = _cairo_fixed_integer_part(right);
        fx1 = _cairo_fixed_fractional_part(right);
        ix2 = _cairo_fixed_integer_part(left);
        fx2 = _cairo_fixed_fractional_part(left);
        dx = -dx;
        sign = -sign;
        dy = y1 - y2;
        y1 = y2 - dy;
        y2 = y1 + dy;
    }
    let mut y: quorem = floored_divrem(
        (((1 as libc::c_int) << 8 as libc::c_int) - fx1) * dy,
        dx,
    );
    let mut cell: *mut cell = 0 as *mut cell;
    cell = (*sweep_line).coverage.cursor;
    if (*cell).x != ix1 {
        if (*cell).x > ix1 {
            while !((*(*cell).prev).x < ix1) {
                cell = (*cell).prev;
            }
        } else {
            while !((*cell).x >= ix1) {
                cell = (*cell).next;
                if (*cell).x >= ix1 {
                    break;
                }
                cell = (*cell).next;
                if (*cell).x >= ix1 {
                    break;
                }
                cell = (*cell).next;
            }
        }
        if (*cell).x != ix1 {
            cell = coverage_alloc(sweep_line, cell, ix1);
        }
    }
    (*cell).uncovered_area
        += sign * y.quo * (((1 as libc::c_int) << 8 as libc::c_int) + fx1);
    (*cell).covered_height += sign * y.quo;
    y.quo += y1;
    cell = (*cell).next;
    ix1 += 1;
    if (*cell).x != ix1 {
        cell = coverage_alloc(sweep_line, cell, ix1);
    }
    if ix1 < ix2 {
        let mut dydx_full: quorem = floored_divrem(
            ((1 as libc::c_int) << 8 as libc::c_int) * dy,
            dx,
        );
        loop {
            let mut y_skip: cairo_fixed_t = dydx_full.quo;
            y.rem += dydx_full.rem;
            if y.rem >= dx {
                y_skip += 1;
                y.rem -= dx;
            }
            y.quo += y_skip;
            y_skip *= sign;
            (*cell).covered_height += y_skip;
            (*cell).uncovered_area += y_skip * ((1 as libc::c_int) << 8 as libc::c_int);
            cell = (*cell).next;
            ix1 += 1;
            if (*cell).x != ix1 {
                cell = coverage_alloc(sweep_line, cell, ix1);
            }
            if !(ix1 != ix2) {
                break;
            }
        }
    }
    (*cell).uncovered_area += sign * (y2 - y.quo) * fx2;
    (*cell).covered_height += sign * (y2 - y.quo);
    let ref mut fresh54 = (*sweep_line).coverage.cursor;
    *fresh54 = cell;
}
#[inline]
unsafe extern "C" fn full_inc_edge(mut edge: *mut edge_t) {
    let ref mut fresh55 = (*edge).x.quo;
    *fresh55 += (*edge).dxdy_full.quo;
    let ref mut fresh56 = (*edge).x.rem;
    *fresh56 += (*edge).dxdy_full.rem;
    if (*edge).x.rem >= 0 as libc::c_int {
        let ref mut fresh57 = (*edge).x.quo;
        *fresh57 += 1;
        let ref mut fresh58 = (*edge).x.rem;
        *fresh58 -= (*edge).dy;
    }
}
unsafe extern "C" fn full_add_edge(
    mut sweep_line: *mut sweep_line_t,
    mut edge: *mut edge_t,
    mut sign: libc::c_int,
) {
    let mut cell: *mut cell = 0 as *mut cell;
    let mut x1: cairo_fixed_t = 0;
    let mut x2: cairo_fixed_t = 0;
    let mut ix1: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut frac: libc::c_int = 0;
    (*edge).current_sign = sign;
    ix1 = _cairo_fixed_integer_part((*edge).x.quo);
    if (*edge).vertical != 0 {
        frac = _cairo_fixed_fractional_part((*edge).x.quo);
        cell = coverage_find(sweep_line, ix1);
        (*cell).covered_height += sign * ((1 as libc::c_int) << 8 as libc::c_int);
        (*cell).uncovered_area
            += sign * 2 as libc::c_int * frac * ((1 as libc::c_int) << 8 as libc::c_int);
        return;
    }
    x1 = (*edge).x.quo;
    full_inc_edge(edge);
    x2 = (*edge).x.quo;
    ix2 = _cairo_fixed_integer_part((*edge).x.quo);
    if ix1 == ix2 {
        frac = _cairo_fixed_fractional_part(x1) + _cairo_fixed_fractional_part(x2);
        cell = coverage_find(sweep_line, ix1);
        (*cell).covered_height += sign * ((1 as libc::c_int) << 8 as libc::c_int);
        (*cell).uncovered_area += sign * frac * ((1 as libc::c_int) << 8 as libc::c_int);
        return;
    }
    coverage_render_cells(
        sweep_line,
        x1,
        x2,
        0 as libc::c_int,
        (1 as libc::c_int) << 8 as libc::c_int,
        sign,
    );
}
unsafe extern "C" fn full_nonzero(mut sweep_line: *mut sweep_line_t) {
    let mut pos: *mut cairo_list_t = 0 as *mut cairo_list_t;
    (*sweep_line).is_vertical = 1 as libc::c_int;
    pos = (*sweep_line).active.next;
    loop {
        let mut left: *mut edge_t = link_to_edge(pos);
        let mut right: *mut edge_t = 0 as *mut edge_t;
        let mut winding: libc::c_int = (*left).edge.dir;
        let ref mut fresh59 = (*sweep_line).is_vertical;
        *fresh59 &= (*left).vertical;
        pos = (*left).link.next;
        loop {
            if pos == &mut (*sweep_line).active as *mut cairo_list_t {
                full_add_edge(sweep_line, left, 1 as libc::c_int);
                return;
            }
            right = link_to_edge(pos);
            pos = (*pos).next;
            let ref mut fresh60 = (*sweep_line).is_vertical;
            *fresh60 &= (*right).vertical;
            winding += (*right).edge.dir;
            if 0 as libc::c_int == winding {
                if pos == &mut (*sweep_line).active as *mut cairo_list_t
                    || (*link_to_edge(pos)).x.quo != (*right).x.quo
                {
                    break;
                }
            }
            if (*right).vertical == 0 {
                full_inc_edge(right);
            }
        }
        full_add_edge(sweep_line, left, 1 as libc::c_int);
        full_add_edge(sweep_line, right, -(1 as libc::c_int));
        if !(pos != &mut (*sweep_line).active as *mut cairo_list_t) {
            break;
        }
    };
}
unsafe extern "C" fn full_evenodd(mut sweep_line: *mut sweep_line_t) {
    let mut pos: *mut cairo_list_t = 0 as *mut cairo_list_t;
    (*sweep_line).is_vertical = 1 as libc::c_int;
    pos = (*sweep_line).active.next;
    loop {
        let mut left: *mut edge_t = link_to_edge(pos);
        let mut right: *mut edge_t = 0 as *mut edge_t;
        let mut winding: libc::c_int = 0 as libc::c_int;
        let ref mut fresh61 = (*sweep_line).is_vertical;
        *fresh61 &= (*left).vertical;
        pos = (*left).link.next;
        loop {
            if pos == &mut (*sweep_line).active as *mut cairo_list_t {
                full_add_edge(sweep_line, left, 1 as libc::c_int);
                return;
            }
            right = link_to_edge(pos);
            pos = (*pos).next;
            let ref mut fresh62 = (*sweep_line).is_vertical;
            *fresh62 &= (*right).vertical;
            winding += 1;
            if winding & 1 as libc::c_int != 0 {
                if pos == &mut (*sweep_line).active as *mut cairo_list_t
                    || (*link_to_edge(pos)).x.quo != (*right).x.quo
                {
                    break;
                }
            }
            if (*right).vertical == 0 {
                full_inc_edge(right);
            }
        }
        full_add_edge(sweep_line, left, 1 as libc::c_int);
        full_add_edge(sweep_line, right, -(1 as libc::c_int));
        if !(pos != &mut (*sweep_line).active as *mut cairo_list_t) {
            break;
        }
    };
}
unsafe extern "C" fn render_rows(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut sweep_line: *mut sweep_line_t,
    mut y: libc::c_int,
    mut height: libc::c_int,
    mut renderer: *mut cairo_span_renderer_t,
) {
    let mut spans_stack: [cairo_half_open_span_t; 256] = [cairo_half_open_span_t {
        x: 0,
        coverage: 0,
        inverse: 0,
    }; 256];
    let mut spans: *mut cairo_half_open_span_t = spans_stack.as_mut_ptr();
    let mut cell: *mut cell = 0 as *mut cell;
    let mut prev_x: libc::c_int = 0;
    let mut cover: libc::c_int = 0;
    let mut num_spans: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*sweep_line).coverage.count == 0 as libc::c_int {
        status = ((*renderer).render_rows)
            .expect(
                "non-null function pointer",
            )(
            renderer as *mut libc::c_void,
            y,
            height,
            0 as *const cairo_half_open_span_t,
            0 as libc::c_int as libc::c_uint,
        );
        if status as u64 != 0 {
            longjmp(((*sweep_line).unwind).as_mut_ptr(), status as libc::c_int);
        }
        return;
    }
    num_spans = 2 as libc::c_int * (*sweep_line).coverage.count + 2 as libc::c_int;
    if num_spans
        > (::std::mem::size_of::<[cairo_half_open_span_t; 256]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
            ) as libc::c_int
    {
        spans = _cairo_malloc_ab(
            num_spans as size_t,
            ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
        ) as *mut cairo_half_open_span_t;
        if spans.is_null() {
            longjmp(
                ((*sweep_line).unwind).as_mut_ptr(),
                _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
            );
        }
    }
    num_spans = 0 as libc::c_int;
    prev_x = (*self_0).xmin;
    cover = 0 as libc::c_int;
    cell = (*sweep_line).coverage.head.next;
    loop {
        let mut x: libc::c_int = (*cell).x;
        let mut area: libc::c_int = 0;
        if x > prev_x {
            (*spans.offset(num_spans as isize)).x = prev_x;
            (*spans.offset(num_spans as isize)).inverse = 0 as libc::c_int as uint8_t;
            (*spans.offset(num_spans as isize))
                .coverage = ((cover * 255 as libc::c_int
                + 2 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)
                    * ((1 as libc::c_int) << 8 as libc::c_int) / 2 as libc::c_int)
                / (2 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)
                    * ((1 as libc::c_int) << 8 as libc::c_int))) as uint8_t;
            num_spans += 1;
        }
        cover
            += (*cell).covered_height * ((1 as libc::c_int) << 8 as libc::c_int)
                * 2 as libc::c_int;
        area = cover - (*cell).uncovered_area;
        (*spans.offset(num_spans as isize)).x = x;
        (*spans.offset(num_spans as isize))
            .coverage = ((area * 255 as libc::c_int
            + 2 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)
                * ((1 as libc::c_int) << 8 as libc::c_int) / 2 as libc::c_int)
            / (2 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)
                * ((1 as libc::c_int) << 8 as libc::c_int))) as uint8_t;
        num_spans += 1;
        prev_x = x + 1 as libc::c_int;
        cell = (*cell).next;
        if !(cell != &mut (*sweep_line).coverage.tail as *mut cell) {
            break;
        }
    }
    if prev_x <= (*self_0).xmax {
        (*spans.offset(num_spans as isize)).x = prev_x;
        (*spans.offset(num_spans as isize)).inverse = 0 as libc::c_int as uint8_t;
        (*spans.offset(num_spans as isize))
            .coverage = ((cover * 255 as libc::c_int
            + 2 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)
                * ((1 as libc::c_int) << 8 as libc::c_int) / 2 as libc::c_int)
            / (2 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)
                * ((1 as libc::c_int) << 8 as libc::c_int))) as uint8_t;
        num_spans += 1;
    }
    if cover != 0 && prev_x < (*self_0).xmax {
        (*spans.offset(num_spans as isize)).x = (*self_0).xmax;
        (*spans.offset(num_spans as isize)).inverse = 1 as libc::c_int as uint8_t;
        (*spans.offset(num_spans as isize)).coverage = 0 as libc::c_int as uint8_t;
        num_spans += 1;
    }
    status = ((*renderer).render_rows)
        .expect(
            "non-null function pointer",
        )(renderer as *mut libc::c_void, y, height, spans, num_spans as libc::c_uint);
    if spans != spans_stack.as_mut_ptr() {
        free(spans as *mut libc::c_void);
    }
    coverage_reset(&mut (*sweep_line).coverage);
    if status as u64 != 0 {
        longjmp(((*sweep_line).unwind).as_mut_ptr(), status as libc::c_int);
    }
}
unsafe extern "C" fn full_repeat(mut sweep: *mut sweep_line_t) {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    edge = ({
        let mut mptr__: *const cairo_list_t = (*sweep).active.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut edge_t
    });
    while &mut (*edge).link as *mut cairo_list_t
        != &mut (*sweep).active as *mut cairo_list_t
    {
        if (*edge).current_sign != 0 {
            full_add_edge(sweep, edge, (*edge).current_sign);
        } else if (*edge).vertical == 0 {
            full_inc_edge(edge);
        }
        edge = ({
            let mut mptr__: *const cairo_list_t = (*edge).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut edge_t
        });
    }
}
unsafe extern "C" fn full_reset(mut sweep: *mut sweep_line_t) {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    edge = ({
        let mut mptr__: *const cairo_list_t = (*sweep).active.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut edge_t
    });
    while &mut (*edge).link as *mut cairo_list_t
        != &mut (*sweep).active as *mut cairo_list_t
    {
        (*edge).current_sign = 0 as libc::c_int;
        edge = ({
            let mut mptr__: *const cairo_list_t = (*edge).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut edge_t
        });
    }
}
unsafe extern "C" fn full_step(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut sweep_line: *mut sweep_line_t,
    mut row: cairo_fixed_t,
    mut renderer: *mut cairo_span_renderer_t,
) {
    let mut top: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    top = _cairo_fixed_integer_part((*sweep_line).current_row);
    bottom = _cairo_fixed_integer_part(row);
    if cairo_list_is_empty(&mut (*sweep_line).active) != 0 {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = ((*renderer).render_rows)
            .expect(
                "non-null function pointer",
            )(
            renderer as *mut libc::c_void,
            top,
            bottom - top,
            0 as *const cairo_half_open_span_t,
            0 as libc::c_int as libc::c_uint,
        );
        if status as u64 != 0 {
            longjmp(((*sweep_line).unwind).as_mut_ptr(), status as libc::c_int);
        }
        return;
    }
    if (*self_0).fill_rule as libc::c_uint
        == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
    {
        full_nonzero(sweep_line);
    } else {
        full_evenodd(sweep_line);
    }
    if (*sweep_line).is_vertical != 0 || bottom == top + 1 as libc::c_int {
        render_rows(self_0, sweep_line, top, bottom - top, renderer);
        full_reset(sweep_line);
        return;
    }
    let fresh63 = top;
    top = top + 1;
    render_rows(self_0, sweep_line, fresh63, 1 as libc::c_int, renderer);
    loop {
        full_repeat(sweep_line);
        render_rows(self_0, sweep_line, top, 1 as libc::c_int, renderer);
        top += 1;
        if !(top != bottom) {
            break;
        }
    }
    full_reset(sweep_line);
}
#[inline(always)]
unsafe extern "C" fn sub_inc_edge(mut edge: *mut edge_t, mut height: cairo_fixed_t) {
    if height == 1 as libc::c_int {
        let ref mut fresh64 = (*edge).x.quo;
        *fresh64 += (*edge).dxdy.quo;
        let ref mut fresh65 = (*edge).x.rem;
        *fresh65 += (*edge).dxdy.rem;
        if (*edge).x.rem >= 0 as libc::c_int {
            let ref mut fresh66 = (*edge).x.quo;
            *fresh66 += 1;
            let ref mut fresh67 = (*edge).x.rem;
            *fresh67 -= (*edge).dy;
        }
    } else {
        let ref mut fresh68 = (*edge).x.quo;
        *fresh68 += height * (*edge).dxdy.quo;
        let ref mut fresh69 = (*edge).x.rem;
        *fresh69 += height * (*edge).dxdy.rem;
        if (*edge).x.rem >= 0 as libc::c_int {
            let mut carry: libc::c_int = (*edge).x.rem / (*edge).dy + 1 as libc::c_int;
            let ref mut fresh70 = (*edge).x.quo;
            *fresh70 += carry;
            let ref mut fresh71 = (*edge).x.rem;
            *fresh71 -= carry * (*edge).dy;
        }
    };
}
unsafe extern "C" fn sub_add_run(
    mut sweep_line: *mut sweep_line_t,
    mut edge: *mut edge_t,
    mut y: libc::c_int,
    mut sign: libc::c_int,
) {
    let mut run: *mut run = 0 as *mut run;
    run = _cairo_freepool_alloc(&mut (*sweep_line).runs) as *mut run;
    if run.is_null() {
        longjmp(
            ((*sweep_line).unwind).as_mut_ptr(),
            _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
        );
    }
    (*run).y = y;
    (*run).sign = sign;
    let ref mut fresh72 = (*run).next;
    *fresh72 = (*edge).runs;
    let ref mut fresh73 = (*edge).runs;
    *fresh73 = run;
    (*edge).current_sign = sign;
}
#[inline]
unsafe extern "C" fn edges_coincident(
    mut left: *mut edge_t,
    mut right: *mut edge_t,
    mut y: cairo_fixed_t,
) -> cairo_bool_t {
    return line_equal(&mut (*left).edge.line, &mut (*right).edge.line);
}
unsafe extern "C" fn sub_nonzero(mut sweep_line: *mut sweep_line_t) {
    let mut y: cairo_fixed_t = (*sweep_line).current_subrow;
    let mut fy: cairo_fixed_t = _cairo_fixed_fractional_part(y);
    let mut pos: *mut cairo_list_t = 0 as *mut cairo_list_t;
    pos = (*sweep_line).active.next;
    loop {
        let mut left: *mut edge_t = link_to_edge(pos);
        let mut right: *mut edge_t = 0 as *mut edge_t;
        let mut winding: libc::c_int = (*left).edge.dir;
        pos = (*left).link.next;
        loop {
            if pos == &mut (*sweep_line).active as *mut cairo_list_t {
                if (*left).current_sign != 1 as libc::c_int {
                    sub_add_run(sweep_line, left, fy, 1 as libc::c_int);
                }
                return;
            }
            right = link_to_edge(pos);
            pos = (*pos).next;
            winding += (*right).edge.dir;
            if 0 as libc::c_int == winding {
                if pos == &mut (*sweep_line).active as *mut cairo_list_t
                    || edges_coincident(right, link_to_edge(pos), y) == 0
                {
                    break;
                }
            }
            if (*right).current_sign != 0 {
                sub_add_run(sweep_line, right, fy, 0 as libc::c_int);
            }
        }
        if (*left).current_sign != 1 as libc::c_int {
            sub_add_run(sweep_line, left, fy, 1 as libc::c_int);
        }
        if (*right).current_sign != -(1 as libc::c_int) {
            sub_add_run(sweep_line, right, fy, -(1 as libc::c_int));
        }
        if !(pos != &mut (*sweep_line).active as *mut cairo_list_t) {
            break;
        }
    };
}
unsafe extern "C" fn sub_evenodd(mut sweep_line: *mut sweep_line_t) {
    let mut y: cairo_fixed_t = (*sweep_line).current_subrow;
    let mut fy: cairo_fixed_t = _cairo_fixed_fractional_part(y);
    let mut pos: *mut cairo_list_t = 0 as *mut cairo_list_t;
    pos = (*sweep_line).active.next;
    loop {
        let mut left: *mut edge_t = link_to_edge(pos);
        let mut right: *mut edge_t = 0 as *mut edge_t;
        let mut winding: libc::c_int = 0 as libc::c_int;
        pos = (*left).link.next;
        loop {
            if pos == &mut (*sweep_line).active as *mut cairo_list_t {
                if (*left).current_sign != 1 as libc::c_int {
                    sub_add_run(sweep_line, left, fy, 1 as libc::c_int);
                }
                return;
            }
            right = link_to_edge(pos);
            pos = (*pos).next;
            winding += 1;
            if winding & 1 as libc::c_int != 0 {
                if pos == &mut (*sweep_line).active as *mut cairo_list_t
                    || edges_coincident(right, link_to_edge(pos), y) == 0
                {
                    break;
                }
            }
            if (*right).current_sign != 0 {
                sub_add_run(sweep_line, right, fy, 0 as libc::c_int);
            }
        }
        if (*left).current_sign != 1 as libc::c_int {
            sub_add_run(sweep_line, left, fy, 1 as libc::c_int);
        }
        if (*right).current_sign != -(1 as libc::c_int) {
            sub_add_run(sweep_line, right, fy, -(1 as libc::c_int));
        }
        if !(pos != &mut (*sweep_line).active as *mut cairo_list_t) {
            break;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn sub_step(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut sweep_line: *mut sweep_line_t,
) {
    if cairo_list_is_empty(&mut (*sweep_line).active) != 0 {
        return;
    }
    if (*self_0).fill_rule as libc::c_uint
        == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
    {
        sub_nonzero(sweep_line);
    } else {
        sub_evenodd(sweep_line);
    };
}
unsafe extern "C" fn coverage_render_runs(
    mut sweep: *mut sweep_line_t,
    mut edge: *mut edge_t,
    mut y1: cairo_fixed_t,
    mut y2: cairo_fixed_t,
) {
    let mut tail: run = run {
        next: 0 as *mut run,
        sign: 0,
        y: 0,
    };
    let mut run: *mut run = &mut tail;
    tail.next = 0 as *mut run;
    tail.y = y2;
    while !((*edge).runs).is_null() {
        let mut r: *mut run = 0 as *mut run;
        r = (*edge).runs;
        let ref mut fresh74 = (*edge).runs;
        *fresh74 = (*r).next;
        let ref mut fresh75 = (*r).next;
        *fresh75 = run;
        run = r;
    }
    if (*run).y > y1 {
        sub_inc_edge(edge, (*run).y - y1);
    }
    loop {
        let mut x1: cairo_fixed_t = 0;
        let mut x2: cairo_fixed_t = 0;
        y1 = (*run).y;
        y2 = (*(*run).next).y;
        x1 = (*edge).x.quo;
        if y2 - y1 == (1 as libc::c_int) << 8 as libc::c_int {
            full_inc_edge(edge);
        } else {
            sub_inc_edge(edge, y2 - y1);
        }
        x2 = (*edge).x.quo;
        if (*run).sign != 0 {
            let mut ix1: libc::c_int = 0;
            let mut ix2: libc::c_int = 0;
            ix1 = _cairo_fixed_integer_part(x1);
            ix2 = _cairo_fixed_integer_part(x2);
            if ix1 == ix2 {
                let mut cell: *mut cell = 0 as *mut cell;
                let mut frac: libc::c_int = 0;
                frac = _cairo_fixed_fractional_part(x1)
                    + _cairo_fixed_fractional_part(x2);
                cell = coverage_find(sweep, ix1);
                (*cell).covered_height += (*run).sign * (y2 - y1);
                (*cell).uncovered_area += (*run).sign * (y2 - y1) * frac;
            } else {
                coverage_render_cells(sweep, x1, x2, y1, y2, (*run).sign);
            }
        }
        run = (*run).next;
        if ((*run).next).is_null() {
            break;
        }
    };
}
unsafe extern "C" fn coverage_render_vertical_runs(
    mut sweep: *mut sweep_line_t,
    mut edge: *mut edge_t,
    mut y2: cairo_fixed_t,
) {
    let mut cell: *mut cell = 0 as *mut cell;
    let mut run: *mut run = 0 as *mut run;
    let mut height: libc::c_int = 0 as libc::c_int;
    run = (*edge).runs;
    while !run.is_null() {
        if (*run).sign != 0 {
            height += (*run).sign * (y2 - (*run).y);
        }
        y2 = (*run).y;
        run = (*run).next;
    }
    cell = coverage_find(sweep, _cairo_fixed_integer_part((*edge).x.quo));
    (*cell).covered_height += height;
    (*cell).uncovered_area
        += 2 as libc::c_int * _cairo_fixed_fractional_part((*edge).x.quo) * height;
}
#[inline(always)]
unsafe extern "C" fn sub_emit(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut sweep: *mut sweep_line_t,
    mut renderer: *mut cairo_span_renderer_t,
) {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    sub_step(self_0, sweep);
    edge = ({
        let mut mptr__: *const cairo_list_t = (*sweep).active.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut edge_t
    });
    while &mut (*edge).link as *mut cairo_list_t
        != &mut (*sweep).active as *mut cairo_list_t
    {
        if ((*edge).runs).is_null() {
            if (*edge).vertical == 0 {
                if (*edge).flags & START as libc::c_int as libc::c_uint != 0 {
                    sub_inc_edge(
                        edge,
                        ((1 as libc::c_int) << 8 as libc::c_int)
                            - _cairo_fixed_fractional_part((*edge).edge.top),
                    );
                    (*edge).flags &= !(START as libc::c_int) as libc::c_uint;
                } else {
                    full_inc_edge(edge);
                }
            }
        } else if (*edge).vertical != 0 {
            coverage_render_vertical_runs(
                sweep,
                edge,
                (1 as libc::c_int) << 8 as libc::c_int,
            );
        } else {
            let mut y1: libc::c_int = 0 as libc::c_int;
            if (*edge).flags & START as libc::c_int as libc::c_uint != 0 {
                y1 = _cairo_fixed_fractional_part((*edge).edge.top);
                (*edge).flags &= !(START as libc::c_int) as libc::c_uint;
            }
            coverage_render_runs(
                sweep,
                edge,
                y1,
                (1 as libc::c_int) << 8 as libc::c_int,
            );
        }
        (*edge).current_sign = 0 as libc::c_int;
        let ref mut fresh76 = (*edge).runs;
        *fresh76 = 0 as *mut run;
        edge = ({
            let mut mptr__: *const cairo_list_t = (*edge).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut edge_t
        });
    }
    edge = ({
        let mut mptr__: *const cairo_list_t = (*sweep).stopped.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut edge_t
    });
    while &mut (*edge).link as *mut cairo_list_t
        != &mut (*sweep).stopped as *mut cairo_list_t
    {
        let mut y2: libc::c_int = _cairo_fixed_fractional_part((*edge).edge.bottom);
        if (*edge).vertical != 0 {
            coverage_render_vertical_runs(sweep, edge, y2);
        } else {
            let mut y1_0: libc::c_int = 0 as libc::c_int;
            if (*edge).flags & START as libc::c_int as libc::c_uint != 0 {
                y1_0 = _cairo_fixed_fractional_part((*edge).edge.top);
            }
            coverage_render_runs(sweep, edge, y1_0, y2);
        }
        edge = ({
            let mut mptr__: *const cairo_list_t = (*edge).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut edge_t
        });
    }
    cairo_list_init(&mut (*sweep).stopped);
    _cairo_freepool_reset(&mut (*sweep).runs);
    render_rows(
        self_0,
        sweep,
        _cairo_fixed_integer_part((*sweep).current_row),
        1 as libc::c_int,
        renderer,
    );
}
unsafe extern "C" fn sweep_line_init(
    mut sweep_line: *mut sweep_line_t,
    mut start_events: *mut *mut event_t,
    mut num_events: libc::c_int,
) {
    cairo_list_init(&mut (*sweep_line).active);
    cairo_list_init(&mut (*sweep_line).stopped);
    let ref mut fresh77 = (*sweep_line).insert_cursor;
    *fresh77 = &mut (*sweep_line).active;
    (*sweep_line).current_row = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*sweep_line).current_subrow = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    coverage_init(&mut (*sweep_line).coverage);
    _cairo_freepool_init(
        &mut (*sweep_line).runs,
        ::std::mem::size_of::<run>() as libc::c_ulong as libc::c_uint,
    );
    start_event_sort(start_events, num_events as libc::c_uint);
    let ref mut fresh78 = *start_events.offset(num_events as isize);
    *fresh78 = 0 as *mut event_t;
    let ref mut fresh79 = (*sweep_line).queue.start_events;
    *fresh79 = start_events;
    _cairo_freepool_init(
        &mut (*sweep_line).queue.pool,
        ::std::mem::size_of::<queue_event_t>() as libc::c_ulong as libc::c_uint,
    );
    pqueue_init(&mut (*sweep_line).queue.pq);
    let ref mut fresh80 = *((*sweep_line).queue.pq.elements)
        .offset(1 as libc::c_int as isize);
    *fresh80 = 0 as *mut event_t;
}
unsafe extern "C" fn sweep_line_delete(
    mut sweep_line: *mut sweep_line_t,
    mut edge: *mut edge_t,
) {
    if (*sweep_line).insert_cursor == &mut (*edge).link as *mut cairo_list_t {
        let ref mut fresh81 = (*sweep_line).insert_cursor;
        *fresh81 = (*edge).link.prev;
    }
    cairo_list_del(&mut (*edge).link);
    if !((*edge).runs).is_null() {
        cairo_list_add_tail(&mut (*edge).link, &mut (*sweep_line).stopped);
    }
    (*edge).flags |= STOP as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn sweep_line_swap(
    mut sweep_line: *mut sweep_line_t,
    mut left: *mut edge_t,
    mut right: *mut edge_t,
) {
    let ref mut fresh82 = (*right).link.prev;
    *fresh82 = (*left).link.prev;
    let ref mut fresh83 = (*left).link.next;
    *fresh83 = (*right).link.next;
    let ref mut fresh84 = (*right).link.next;
    *fresh84 = &mut (*left).link;
    let ref mut fresh85 = (*left).link.prev;
    *fresh85 = &mut (*right).link;
    let ref mut fresh86 = (*(*left).link.next).prev;
    *fresh86 = &mut (*left).link;
    let ref mut fresh87 = (*(*right).link.prev).next;
    *fresh87 = &mut (*right).link;
}
unsafe extern "C" fn sweep_line_fini(mut sweep_line: *mut sweep_line_t) {
    pqueue_fini(&mut (*sweep_line).queue.pq);
    _cairo_freepool_fini(&mut (*sweep_line).queue.pool);
    coverage_fini(&mut (*sweep_line).coverage);
    _cairo_freepool_fini(&mut (*sweep_line).runs);
}
unsafe extern "C" fn botor_generate(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut start_events: *mut *mut event_t,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut sweep_line: sweep_line_t = sweep_line_t {
        active: cairo_list_t {
            next: 0 as *mut _cairo_list,
            prev: 0 as *mut _cairo_list,
        },
        stopped: cairo_list_t {
            next: 0 as *mut _cairo_list,
            prev: 0 as *mut _cairo_list,
        },
        insert_cursor: 0 as *mut cairo_list_t,
        is_vertical: 0,
        current_row: 0,
        current_subrow: 0,
        coverage: coverage {
            head: cell {
                prev: 0 as *mut cell,
                next: 0 as *mut cell,
                x: 0,
                uncovered_area: 0,
                covered_height: 0,
            },
            tail: cell {
                prev: 0 as *mut cell,
                next: 0 as *mut cell,
                x: 0,
                uncovered_area: 0,
                covered_height: 0,
            },
            cursor: 0 as *mut cell,
            count: 0,
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
        },
        queue: event_queue {
            pq: pqueue_t {
                size: 0,
                max_size: 0,
                elements: 0 as *mut *mut event_t,
                elements_embedded: [0 as *mut event_t; 1024],
            },
            start_events: 0 as *mut *mut event_t,
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
        },
        runs: cairo_freepool_t {
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
        unwind: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    let mut ybot: cairo_fixed_t = 0;
    let mut event: *mut event_t = 0 as *mut event_t;
    let mut left: *mut cairo_list_t = 0 as *mut cairo_list_t;
    let mut right: *mut cairo_list_t = 0 as *mut cairo_list_t;
    let mut e1: *mut edge_t = 0 as *mut edge_t;
    let mut e2: *mut edge_t = 0 as *mut edge_t;
    let mut bottom: libc::c_int = 0;
    sweep_line_init(&mut sweep_line, start_events, (*self_0).num_edges);
    status = _setjmp((sweep_line.unwind).as_mut_ptr()) as cairo_status_t;
    if !(status as u64 != 0) {
        ybot = (*self_0).extents.p2.y;
        sweep_line.current_subrow = (*self_0).extents.p1.y;
        sweep_line.current_row = _cairo_fixed_floor((*self_0).extents.p1.y);
        let fresh88 = sweep_line.queue.start_events;
        sweep_line.queue.start_events = (sweep_line.queue.start_events).offset(1);
        event = *fresh88;
        's_52: loop {
            if (*event).y
                >= sweep_line.current_row + ((1 as libc::c_int) << 8 as libc::c_int)
            {
                bottom = _cairo_fixed_floor((*event).y);
                full_step(self_0, &mut sweep_line, bottom, renderer);
                sweep_line.current_row = bottom;
                sweep_line.current_subrow = bottom;
            }
            loop {
                if (*event).y > sweep_line.current_subrow {
                    sub_step(self_0, &mut sweep_line);
                    sweep_line.current_subrow = (*event).y;
                }
                loop {
                    match (*event).type_0 as libc::c_uint {
                        2 => {
                            e1 = (*(event as *mut start_event_t)).edge;
                            sweep_line_insert(&mut sweep_line, e1);
                            event_insert_stop(&mut sweep_line, e1);
                            left = (*e1).link.prev;
                            right = (*e1).link.next;
                            if left != &mut sweep_line.active as *mut cairo_list_t {
                                event_insert_if_intersect_below_current_y(
                                    &mut sweep_line,
                                    link_to_edge(left),
                                    e1,
                                );
                            }
                            if right != &mut sweep_line.active as *mut cairo_list_t {
                                event_insert_if_intersect_below_current_y(
                                    &mut sweep_line,
                                    e1,
                                    link_to_edge(right),
                                );
                            }
                        }
                        0 => {
                            e1 = (*(event as *mut queue_event_t)).e1;
                            event_delete(&mut sweep_line, event);
                            left = (*e1).link.prev;
                            right = (*e1).link.next;
                            sweep_line_delete(&mut sweep_line, e1);
                            if left != &mut sweep_line.active as *mut cairo_list_t
                                && right != &mut sweep_line.active as *mut cairo_list_t
                            {
                                event_insert_if_intersect_below_current_y(
                                    &mut sweep_line,
                                    link_to_edge(left),
                                    link_to_edge(right),
                                );
                            }
                        }
                        1 => {
                            e1 = (*(event as *mut queue_event_t)).e1;
                            e2 = (*(event as *mut queue_event_t)).e2;
                            event_delete(&mut sweep_line, event);
                            if !((*e1).flags & STOP as libc::c_int as libc::c_uint != 0)
                            {
                                if !((*e2).flags & STOP as libc::c_int as libc::c_uint != 0)
                                {
                                    if !(&mut (*e2).link as *mut cairo_list_t
                                        != (*e1).link.next)
                                    {
                                        left = (*e1).link.prev;
                                        right = (*e2).link.next;
                                        sweep_line_swap(&mut sweep_line, e1, e2);
                                        if left != &mut sweep_line.active as *mut cairo_list_t {
                                            event_insert_if_intersect_below_current_y(
                                                &mut sweep_line,
                                                link_to_edge(left),
                                                e2,
                                            );
                                        }
                                        if right != &mut sweep_line.active as *mut cairo_list_t {
                                            event_insert_if_intersect_below_current_y(
                                                &mut sweep_line,
                                                e1,
                                                link_to_edge(right),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    event = event_next(&mut sweep_line);
                    if event.is_null() {
                        break 's_52;
                    }
                    if !((*event).y == sweep_line.current_subrow) {
                        break;
                    }
                }
                if !((*event).y
                    < sweep_line.current_row + ((1 as libc::c_int) << 8 as libc::c_int))
                {
                    break;
                }
            }
            bottom = sweep_line.current_row + ((1 as libc::c_int) << 8 as libc::c_int);
            sub_emit(self_0, &mut sweep_line, renderer);
            sweep_line.current_subrow = bottom;
            sweep_line.current_row = sweep_line.current_subrow;
        }
        if sweep_line.current_subrow != sweep_line.current_row {
            sub_emit(self_0, &mut sweep_line, renderer);
            sweep_line.current_row += (1 as libc::c_int) << 8 as libc::c_int;
            sweep_line.current_subrow = sweep_line.current_row;
        }
        if sweep_line.current_subrow < ybot {
            bottom = _cairo_fixed_integer_part(sweep_line.current_row);
            status = ((*renderer).render_rows)
                .expect(
                    "non-null function pointer",
                )(
                renderer as *mut libc::c_void,
                bottom,
                _cairo_fixed_integer_ceil(ybot) - bottom,
                0 as *const cairo_half_open_span_t,
                0 as libc::c_int as libc::c_uint,
            );
        }
    }
    sweep_line_fini(&mut sweep_line);
    return status;
}
unsafe extern "C" fn _cairo_botor_scan_converter_generate(
    mut converter: *mut libc::c_void,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_botor_scan_converter_t = converter
        as *mut cairo_botor_scan_converter_t;
    let mut stack_events: [start_event_t; 128] = [start_event_t {
        y: 0,
        type_0: EVENT_TYPE_STOP,
        edge: 0 as *mut edge_t,
    }; 128];
    let mut events: *mut start_event_t = 0 as *mut start_event_t;
    let mut stack_event_ptrs: [*mut event_t; 129] = [0 as *mut event_t; 129];
    let mut event_ptrs: *mut *mut event_t = 0 as *mut *mut event_t;
    let mut chunk: *mut _cairo_botor_scan_converter_chunk = 0
        as *mut _cairo_botor_scan_converter_chunk;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut num_events: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    num_events = (*self_0).num_edges;
    if 0 as libc::c_int == num_events {
        return ((*renderer).render_rows)
            .expect(
                "non-null function pointer",
            )(
            renderer as *mut libc::c_void,
            _cairo_fixed_integer_floor((*self_0).extents.p1.y),
            _cairo_fixed_integer_ceil((*self_0).extents.p2.y)
                - _cairo_fixed_integer_floor((*self_0).extents.p1.y),
            0 as *const cairo_half_open_span_t,
            0 as libc::c_int as libc::c_uint,
        );
    }
    events = stack_events.as_mut_ptr();
    event_ptrs = stack_event_ptrs.as_mut_ptr();
    if num_events
        >= (::std::mem::size_of::<[start_event_t; 128]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<start_event_t>() as libc::c_ulong)
            as libc::c_int
    {
        events = _cairo_malloc_ab_plus_c(
            num_events as size_t,
            (::std::mem::size_of::<start_event_t>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<*mut event_t>() as libc::c_ulong),
            ::std::mem::size_of::<*mut event_t>() as libc::c_ulong,
        ) as *mut start_event_t;
        if events.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        event_ptrs = events.offset(num_events as isize) as *mut *mut event_t;
    }
    j = 0 as libc::c_int;
    chunk = &mut (*self_0).chunks;
    while !chunk.is_null() {
        let mut edge: *mut edge_t = 0 as *mut edge_t;
        edge = (*chunk).base as *mut edge_t;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let ref mut fresh89 = *event_ptrs.offset(j as isize);
            *fresh89 = &mut *events.offset(j as isize) as *mut start_event_t
                as *mut event_t;
            (*events.offset(j as isize)).y = (*edge).edge.top;
            (*events.offset(j as isize)).type_0 = EVENT_TYPE_START;
            let ref mut fresh90 = (*events.offset(j as isize)).edge;
            *fresh90 = edge;
            edge = edge.offset(1);
            j += 1;
            i += 1;
        }
        chunk = (*chunk).next;
    }
    status = botor_generate(self_0, event_ptrs, renderer);
    if events != stack_events.as_mut_ptr() {
        free(events as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn botor_allocate_edge(
    mut self_0: *mut cairo_botor_scan_converter_t,
) -> *mut edge_t {
    let mut chunk: *mut _cairo_botor_scan_converter_chunk = 0
        as *mut _cairo_botor_scan_converter_chunk;
    chunk = (*self_0).tail;
    if (*chunk).count == (*chunk).size {
        let mut size: libc::c_int = 0;
        size = (*chunk).size * 2 as libc::c_int;
        let ref mut fresh91 = (*chunk).next;
        *fresh91 = _cairo_malloc_ab_plus_c(
            size as size_t,
            ::std::mem::size_of::<edge_t>() as libc::c_ulong,
            ::std::mem::size_of::<_cairo_botor_scan_converter_chunk>() as libc::c_ulong,
        ) as *mut _cairo_botor_scan_converter_chunk;
        if ((*chunk).next).is_null() {
            return 0 as *mut edge_t;
        }
        chunk = (*chunk).next;
        let ref mut fresh92 = (*chunk).next;
        *fresh92 = 0 as *mut _cairo_botor_scan_converter_chunk;
        (*chunk).count = 0 as libc::c_int;
        (*chunk).size = size;
        let ref mut fresh93 = (*chunk).base;
        *fresh93 = chunk.offset(1 as libc::c_int as isize) as *mut libc::c_void;
        let ref mut fresh94 = (*self_0).tail;
        *fresh94 = chunk;
    }
    let ref mut fresh95 = (*chunk).count;
    let fresh96 = *fresh95;
    *fresh95 = *fresh95 + 1;
    return ((*chunk).base as *mut edge_t).offset(fresh96 as isize);
}
unsafe extern "C" fn botor_add_edge(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut edge: *const cairo_edge_t,
) -> cairo_status_t {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut dx: cairo_fixed_t = 0;
    let mut dy: cairo_fixed_t = 0;
    e = botor_allocate_edge(self_0);
    if e.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    cairo_list_init(&mut (*e).link);
    (*e).edge = *edge;
    dx = (*edge).line.p2.x - (*edge).line.p1.x;
    dy = (*edge).line.p2.y - (*edge).line.p1.y;
    (*e).dy = dy;
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
        if (*edge).top == (*edge).line.p1.y {
            (*e).x.quo = (*edge).line.p1.x;
            (*e).x.rem = 0 as libc::c_int;
        } else {
            (*e).x = floored_muldivrem((*edge).top - (*edge).line.p1.y, dx, dy);
            let ref mut fresh97 = (*e).x.quo;
            *fresh97 += (*edge).line.p1.x;
        }
        if _cairo_fixed_integer_part((*edge).bottom)
            - _cairo_fixed_integer_part((*edge).top) > 1 as libc::c_int
        {
            (*e)
                .dxdy_full = floored_muldivrem(
                (1 as libc::c_int) << 8 as libc::c_int,
                dx,
                dy,
            );
        } else {
            (*e).dxdy_full.quo = 0 as libc::c_int;
            (*e).dxdy_full.rem = 0 as libc::c_int;
        }
    }
    (*e).x.rem = -(*e).dy;
    (*e).current_sign = 0 as libc::c_int;
    let ref mut fresh98 = (*e).runs;
    *fresh98 = 0 as *mut run;
    (*e).flags = START as libc::c_int as libc::c_uint;
    let ref mut fresh99 = (*self_0).num_edges;
    *fresh99 += 1;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_botor_scan_converter_add_polygon(
    mut converter: *mut cairo_botor_scan_converter_t,
    mut polygon: *const cairo_polygon_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_botor_scan_converter_t = converter;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*polygon).num_edges {
        status = botor_add_edge(self_0, &mut *((*polygon).edges).offset(i as isize));
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_botor_scan_converter_destroy(
    mut converter: *mut libc::c_void,
) {
    let mut self_0: *mut cairo_botor_scan_converter_t = converter
        as *mut cairo_botor_scan_converter_t;
    let mut chunk: *mut _cairo_botor_scan_converter_chunk = 0
        as *mut _cairo_botor_scan_converter_chunk;
    let mut next: *mut _cairo_botor_scan_converter_chunk = 0
        as *mut _cairo_botor_scan_converter_chunk;
    chunk = (*self_0).chunks.next;
    while !chunk.is_null() {
        next = (*chunk).next;
        free(chunk as *mut libc::c_void);
        chunk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_botor_scan_converter_init(
    mut self_0: *mut cairo_botor_scan_converter_t,
    mut extents: *const cairo_box_t,
    mut fill_rule: cairo_fill_rule_t,
) {
    let ref mut fresh100 = (*self_0).base.destroy;
    *fresh100 = Some(
        _cairo_botor_scan_converter_destroy
            as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh101 = (*self_0).base.generate;
    *fresh101 = Some(
        _cairo_botor_scan_converter_generate
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut cairo_span_renderer_t,
            ) -> cairo_status_t,
    );
    (*self_0).extents = *extents;
    (*self_0).fill_rule = fill_rule;
    (*self_0).xmin = _cairo_fixed_integer_floor((*extents).p1.x);
    (*self_0).xmax = _cairo_fixed_integer_ceil((*extents).p2.x);
    let ref mut fresh102 = (*self_0).chunks.base;
    *fresh102 = ((*self_0).buf).as_mut_ptr() as *mut libc::c_void;
    let ref mut fresh103 = (*self_0).chunks.next;
    *fresh103 = 0 as *mut _cairo_botor_scan_converter_chunk;
    (*self_0).chunks.count = 0 as libc::c_int;
    (*self_0)
        .chunks
        .size = (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<edge_t>() as libc::c_ulong) as libc::c_int;
    let ref mut fresh104 = (*self_0).tail;
    *fresh104 = &mut (*self_0).chunks;
    (*self_0).num_edges = 0 as libc::c_int;
}
