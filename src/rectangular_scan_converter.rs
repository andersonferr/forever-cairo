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
    fn _cairo_box_from_rectangle(
        box_0: *mut cairo_box_t,
        rectangle: *const cairo_rectangle_int_t,
    );
    fn _cairo_freepool_alloc_from_new_pool(
        freepool: *mut cairo_freepool_t,
    ) -> *mut libc::c_void;
    fn _cairo_freepool_init(freepool: *mut cairo_freepool_t, nodesize: libc::c_uint);
    fn _cairo_freepool_fini(freepool: *mut cairo_freepool_t);
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type uint16_t = __uint16_t;
pub type cairo_fixed_unsigned_t = uint32_t;
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
pub struct _cairo_rectangular_scan_converter {
    pub base: cairo_scan_converter_t,
    pub extents: cairo_box_t,
    pub chunks: _cairo_rectangular_scan_converter_chunk,
    pub tail: *mut _cairo_rectangular_scan_converter_chunk,
    pub buf: [libc::c_char; 2048],
    pub num_rectangles: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangular_scan_converter_chunk {
    pub next: *mut _cairo_rectangular_scan_converter_chunk,
    pub base: *mut libc::c_void,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_rectangular_scan_converter_t = _cairo_rectangular_scan_converter;
pub type rectangle_t = _rectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rectangle {
    pub next: *mut _rectangle,
    pub prev: *mut _rectangle,
    pub left: cairo_fixed_t,
    pub right: cairo_fixed_t,
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub top_y: int32_t,
    pub bottom_y: int32_t,
    pub dir: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sweep_line_t {
    pub start: *mut *mut rectangle_t,
    pub stop: pqueue_t,
    pub head: rectangle_t,
    pub tail: rectangle_t,
    pub insert_cursor: *mut rectangle_t,
    pub current_y: int32_t,
    pub xmin: int32_t,
    pub xmax: int32_t,
    pub coverage: coverage,
    pub spans_stack: [cairo_half_open_span_t; 256],
    pub spans: *mut cairo_half_open_span_t,
    pub num_spans: libc::c_uint,
    pub size_spans: libc::c_uint,
    pub jmpbuf: jmp_buf,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coverage {
    pub head: cell,
    pub tail: cell,
    pub cursor: *mut cell,
    pub count: libc::c_uint,
    pub pool: cairo_freepool_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub prev: *mut cell,
    pub next: *mut cell,
    pub x: libc::c_int,
    pub covered: libc::c_int,
    pub uncovered: libc::c_int,
}
pub type pqueue_t = _pqueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pqueue {
    pub size: libc::c_int,
    pub max_size: libc::c_int,
    pub elements: *mut *mut rectangle_t,
    pub elements_embedded: [*mut rectangle_t; 1024],
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
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
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
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
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
unsafe extern "C" fn _cairo_freepool_reset(mut freepool: *mut cairo_freepool_t) {
    while (*freepool).pools
        != &mut (*freepool).embedded_pool as *mut cairo_freelist_pool_t
    {
        let mut pool: *mut cairo_freelist_pool_t = (*freepool).pools;
        let ref mut fresh9 = (*freepool).pools;
        *fresh9 = (*pool).next;
        let ref mut fresh10 = (*pool).next;
        *fresh10 = (*freepool).freepools;
        let ref mut fresh11 = (*freepool).freepools;
        *fresh11 = pool;
    }
    (*freepool)
        .embedded_pool
        .rem = ::std::mem::size_of::<[uint8_t; 1000]>() as libc::c_ulong as libc::c_uint;
    let ref mut fresh12 = (*freepool).embedded_pool.data;
    *fresh12 = ((*freepool).embedded_data).as_mut_ptr();
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
    let ref mut fresh13 = (*pool).data;
    *fresh13 = (*fresh13).offset((*freepool).nodesize as isize);
    let ref mut fresh14 = (*pool).rem;
    *fresh14 = (*fresh14).wrapping_sub((*freepool).nodesize);
    return ptr as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn rectangle_compare_start(
    mut a: *const rectangle_t,
    mut b: *const rectangle_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = (*a).top_y - (*b).top_y;
    if cmp != 0 {
        return cmp;
    }
    return (*a).left - (*b).left;
}
#[inline]
unsafe extern "C" fn rectangle_compare_stop(
    mut a: *const rectangle_t,
    mut b: *const rectangle_t,
) -> libc::c_int {
    return (*a).bottom_y - (*b).bottom_y;
}
#[inline]
unsafe extern "C" fn pqueue_init(mut pq: *mut pqueue_t) {
    (*pq)
        .max_size = (::std::mem::size_of::<[*mut rectangle_t; 1024]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong)
        as libc::c_int;
    (*pq).size = 0 as libc::c_int;
    let ref mut fresh15 = (*pq).elements;
    *fresh15 = ((*pq).elements_embedded).as_mut_ptr();
    let ref mut fresh16 = *((*pq).elements).offset(1 as libc::c_int as isize);
    *fresh16 = 0 as *mut rectangle_t;
}
#[inline]
unsafe extern "C" fn pqueue_fini(mut pq: *mut pqueue_t) {
    if (*pq).elements != ((*pq).elements_embedded).as_mut_ptr() {
        free((*pq).elements as *mut libc::c_void);
    }
}
unsafe extern "C" fn pqueue_grow(mut pq: *mut pqueue_t) -> cairo_bool_t {
    let mut new_elements: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    (*pq).max_size *= 2 as libc::c_int;
    if (*pq).elements == ((*pq).elements_embedded).as_mut_ptr() {
        new_elements = _cairo_malloc_ab(
            (*pq).max_size as size_t,
            ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
        ) as *mut *mut rectangle_t;
        if new_elements.is_null() {
            return 0 as libc::c_int;
        }
        memcpy(
            new_elements as *mut libc::c_void,
            ((*pq).elements_embedded).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[*mut rectangle_t; 1024]>() as libc::c_ulong,
        );
    } else {
        new_elements = _cairo_realloc_ab(
            (*pq).elements as *mut libc::c_void,
            (*pq).max_size as size_t,
            ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
        ) as *mut *mut rectangle_t;
        if new_elements.is_null() {
            return 0 as libc::c_int;
        }
    }
    let ref mut fresh17 = (*pq).elements;
    *fresh17 = new_elements;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn pqueue_push(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) {
    let mut elements: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut i: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    if (*sweep).stop.size + 1 as libc::c_int == (*sweep).stop.max_size {
        if pqueue_grow(&mut (*sweep).stop) == 0 {
            longjmp(
                ((*sweep).jmpbuf).as_mut_ptr(),
                _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
            );
        }
    }
    elements = (*sweep).stop.elements;
    let ref mut fresh18 = (*sweep).stop.size;
    *fresh18 += 1;
    i = *fresh18;
    while i != 1 as libc::c_int
        && {
            parent = i >> 1 as libc::c_int;
            rectangle_compare_stop(rectangle, *elements.offset(parent as isize))
                < 0 as libc::c_int
        }
    {
        let ref mut fresh19 = *elements.offset(i as isize);
        *fresh19 = *elements.offset(parent as isize);
        i = parent;
    }
    let ref mut fresh20 = *elements.offset(i as isize);
    *fresh20 = rectangle;
}
#[inline]
unsafe extern "C" fn pqueue_pop(mut pq: *mut pqueue_t) {
    let mut elements: *mut *mut rectangle_t = (*pq).elements;
    let mut tail: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let ref mut fresh21 = (*pq).size;
    let fresh22 = *fresh21;
    *fresh21 = *fresh21 - 1;
    tail = *elements.offset(fresh22 as isize);
    if (*pq).size == 0 as libc::c_int {
        let ref mut fresh23 = *elements.offset(1 as libc::c_int as isize);
        *fresh23 = 0 as *mut rectangle_t;
        return;
    }
    i = 1 as libc::c_int;
    loop {
        child = i << 1 as libc::c_int;
        if !(child <= (*pq).size) {
            break;
        }
        if child != (*pq).size
            && rectangle_compare_stop(
                *elements.offset((child + 1 as libc::c_int) as isize),
                *elements.offset(child as isize),
            ) < 0 as libc::c_int
        {
            child += 1;
        }
        if rectangle_compare_stop(*elements.offset(child as isize), tail)
            >= 0 as libc::c_int
        {
            break;
        }
        let ref mut fresh24 = *elements.offset(i as isize);
        *fresh24 = *elements.offset(child as isize);
        i = child;
    }
    let ref mut fresh25 = *elements.offset(i as isize);
    *fresh25 = tail;
}
#[inline]
unsafe extern "C" fn peek_stop(mut sweep: *mut sweep_line_t) -> *mut rectangle_t {
    return *((*sweep).stop.elements).offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn rectangle_sort(
    mut base: *mut *mut rectangle_t,
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
            if rectangle_compare_start(
                *base.offset(i as isize),
                *base.offset(j as isize),
            ) > 0 as libc::c_int
            {
                let mut tmp: *mut rectangle_t = 0 as *mut rectangle_t;
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
unsafe extern "C" fn sweep_line_init(mut sweep: *mut sweep_line_t) {
    (*sweep).head.left = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh28 = (*sweep).head.next;
    *fresh28 = &mut (*sweep).tail;
    (*sweep).tail.left = 2147483647 as libc::c_int;
    let ref mut fresh29 = (*sweep).tail.prev;
    *fresh29 = &mut (*sweep).head;
    let ref mut fresh30 = (*sweep).insert_cursor;
    *fresh30 = &mut (*sweep).tail;
    _cairo_freepool_init(
        &mut (*sweep).coverage.pool,
        ::std::mem::size_of::<cell>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh31 = (*sweep).spans;
    *fresh31 = ((*sweep).spans_stack).as_mut_ptr();
    (*sweep)
        .size_spans = (::std::mem::size_of::<[cairo_half_open_span_t; 256]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong)
        as libc::c_int as libc::c_uint;
    let ref mut fresh32 = (*sweep).coverage.head.prev;
    *fresh32 = 0 as *mut cell;
    (*sweep).coverage.head.x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh33 = (*sweep).coverage.tail.next;
    *fresh33 = 0 as *mut cell;
    (*sweep).coverage.tail.x = 2147483647 as libc::c_int;
    pqueue_init(&mut (*sweep).stop);
}
unsafe extern "C" fn sweep_line_fini(mut sweep: *mut sweep_line_t) {
    _cairo_freepool_fini(&mut (*sweep).coverage.pool);
    pqueue_fini(&mut (*sweep).stop);
    if (*sweep).spans != ((*sweep).spans_stack).as_mut_ptr() {
        free((*sweep).spans as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn add_cell(
    mut sweep: *mut sweep_line_t,
    mut x: libc::c_int,
    mut covered: libc::c_int,
    mut uncovered: libc::c_int,
) {
    let mut current_block: u64;
    let mut cell: *mut cell = 0 as *mut cell;
    cell = (*sweep).coverage.cursor;
    if (*cell).x > x {
        while !((*(*cell).prev).x < x) {
            cell = (*cell).prev;
            if (*(*cell).prev).x < x {
                break;
            }
            cell = (*cell).prev;
            if (*(*cell).prev).x < x {
                break;
            }
            cell = (*cell).prev;
        }
        current_block = 18386322304582297246;
    } else if (*cell).x == x {
        current_block = 12263044188136309141;
    } else {
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
        current_block = 18386322304582297246;
    }
    match current_block {
        18386322304582297246 => {
            if x != (*cell).x {
                let mut c: *mut cell = 0 as *mut cell;
                let ref mut fresh34 = (*sweep).coverage.count;
                *fresh34 = (*fresh34).wrapping_add(1);
                c = _cairo_freepool_alloc(&mut (*sweep).coverage.pool) as *mut cell;
                if c.is_null() {
                    longjmp(
                        ((*sweep).jmpbuf).as_mut_ptr(),
                        _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
                    );
                }
                let ref mut fresh35 = (*(*cell).prev).next;
                *fresh35 = c;
                let ref mut fresh36 = (*c).prev;
                *fresh36 = (*cell).prev;
                let ref mut fresh37 = (*c).next;
                *fresh37 = cell;
                let ref mut fresh38 = (*cell).prev;
                *fresh38 = c;
                (*c).x = x;
                (*c).covered = 0 as libc::c_int;
                (*c).uncovered = 0 as libc::c_int;
                cell = c;
            }
        }
        _ => {}
    }
    (*cell).covered += covered;
    (*cell).uncovered += uncovered;
    let ref mut fresh39 = (*sweep).coverage.cursor;
    *fresh39 = cell;
}
#[inline]
unsafe extern "C" fn _active_edges_to_spans(mut sweep: *mut sweep_line_t) {
    let mut y: int32_t = (*sweep).current_y;
    let mut rectangle: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut coverage: libc::c_int = 0;
    let mut prev_coverage: libc::c_int = 0;
    let mut prev_x: libc::c_int = 0;
    let mut cell: *mut cell = 0 as *mut cell;
    (*sweep).num_spans = 0 as libc::c_int as libc::c_uint;
    if (*sweep).head.next == &mut (*sweep).tail as *mut rectangle_t {
        return;
    }
    let ref mut fresh40 = (*sweep).coverage.head.next;
    *fresh40 = &mut (*sweep).coverage.tail;
    let ref mut fresh41 = (*sweep).coverage.tail.prev;
    *fresh41 = &mut (*sweep).coverage.head;
    let ref mut fresh42 = (*sweep).coverage.cursor;
    *fresh42 = &mut (*sweep).coverage.tail;
    (*sweep).coverage.count = 0 as libc::c_int as libc::c_uint;
    let mut current_block_16: u64;
    rectangle = (*sweep).head.next;
    while rectangle != &mut (*sweep).tail as *mut rectangle_t {
        let mut height: libc::c_int = 0;
        let mut frac: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        if y == (*rectangle).bottom_y {
            height = (*rectangle).bottom
                & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                    >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
            if height == 0 as libc::c_int {
                current_block_16 = 13183875560443969876;
            } else {
                current_block_16 = 1054647088692577877;
            }
        } else {
            height = (1 as libc::c_int) << 8 as libc::c_int;
            current_block_16 = 1054647088692577877;
        }
        match current_block_16 {
            1054647088692577877 => {
                if y == (*rectangle).top_y {
                    height
                        -= (*rectangle).top
                            & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
                }
                height *= (*rectangle).dir;
                i = _cairo_fixed_integer_part((*rectangle).left);
                frac = _cairo_fixed_fractional_part((*rectangle).left);
                add_cell(
                    sweep,
                    i,
                    (((1 as libc::c_int) << 8 as libc::c_int) - frac) * height,
                    frac * height,
                );
                i = _cairo_fixed_integer_part((*rectangle).right);
                frac = _cairo_fixed_fractional_part((*rectangle).right);
                add_cell(
                    sweep,
                    i,
                    -(((1 as libc::c_int) << 8 as libc::c_int) - frac) * height,
                    -frac * height,
                );
            }
            _ => {}
        }
        rectangle = (*rectangle).next;
    }
    if (2 as libc::c_int as libc::c_uint).wrapping_mul((*sweep).coverage.count)
        >= (*sweep).size_spans
    {
        let mut size: libc::c_uint = 0;
        size = (*sweep).size_spans;
        while size
            <= (2 as libc::c_int as libc::c_uint).wrapping_mul((*sweep).coverage.count)
        {
            size <<= 1 as libc::c_int;
        }
        if (*sweep).spans != ((*sweep).spans_stack).as_mut_ptr() {
            free((*sweep).spans as *mut libc::c_void);
        }
        let ref mut fresh43 = (*sweep).spans;
        *fresh43 = _cairo_malloc_ab(
            size as size_t,
            ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
        ) as *mut cairo_half_open_span_t;
        if ((*sweep).spans).is_null() {
            longjmp(
                ((*sweep).jmpbuf).as_mut_ptr(),
                _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
            );
        }
        (*sweep).size_spans = size;
    }
    coverage = 0 as libc::c_int;
    prev_coverage = coverage;
    prev_x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    cell = (*sweep).coverage.head.next;
    while cell != &mut (*sweep).coverage.tail as *mut cell {
        if (*cell).x != prev_x && coverage != prev_coverage {
            let ref mut fresh44 = (*sweep).num_spans;
            let fresh45 = *fresh44;
            *fresh44 = (*fresh44).wrapping_add(1);
            let mut n: libc::c_int = fresh45 as libc::c_int;
            let mut c: libc::c_int = coverage
                >> 8 as libc::c_int * 2 as libc::c_int - 8 as libc::c_int;
            (*((*sweep).spans).offset(n as isize)).x = prev_x;
            (*((*sweep).spans).offset(n as isize)).inverse = 0 as libc::c_int as uint8_t;
            (*((*sweep).spans).offset(n as isize))
                .coverage = (c - (c >> 8 as libc::c_int)) as uint8_t;
            prev_coverage = coverage;
        }
        coverage += (*cell).covered;
        if coverage != prev_coverage {
            let ref mut fresh46 = (*sweep).num_spans;
            let fresh47 = *fresh46;
            *fresh46 = (*fresh46).wrapping_add(1);
            let mut n_0: libc::c_int = fresh47 as libc::c_int;
            let mut c_0: libc::c_int = coverage
                >> 8 as libc::c_int * 2 as libc::c_int - 8 as libc::c_int;
            (*((*sweep).spans).offset(n_0 as isize)).x = (*cell).x;
            (*((*sweep).spans).offset(n_0 as isize))
                .inverse = 0 as libc::c_int as uint8_t;
            (*((*sweep).spans).offset(n_0 as isize))
                .coverage = (c_0 - (c_0 >> 8 as libc::c_int)) as uint8_t;
            prev_coverage = coverage;
        }
        coverage += (*cell).uncovered;
        prev_x = (*cell).x + 1 as libc::c_int;
        cell = (*cell).next;
    }
    _cairo_freepool_reset(&mut (*sweep).coverage.pool);
    if (*sweep).num_spans != 0 {
        if prev_x <= (*sweep).xmax {
            let ref mut fresh48 = (*sweep).num_spans;
            let fresh49 = *fresh48;
            *fresh48 = (*fresh48).wrapping_add(1);
            let mut n_1: libc::c_int = fresh49 as libc::c_int;
            let mut c_1: libc::c_int = coverage
                >> 8 as libc::c_int * 2 as libc::c_int - 8 as libc::c_int;
            (*((*sweep).spans).offset(n_1 as isize)).x = prev_x;
            (*((*sweep).spans).offset(n_1 as isize))
                .inverse = 0 as libc::c_int as uint8_t;
            (*((*sweep).spans).offset(n_1 as isize))
                .coverage = (c_1 - (c_1 >> 8 as libc::c_int)) as uint8_t;
        }
        if coverage != 0 && prev_x < (*sweep).xmax {
            let ref mut fresh50 = (*sweep).num_spans;
            let fresh51 = *fresh50;
            *fresh50 = (*fresh50).wrapping_add(1);
            let mut n_2: libc::c_int = fresh51 as libc::c_int;
            (*((*sweep).spans).offset(n_2 as isize)).x = (*sweep).xmax;
            (*((*sweep).spans).offset(n_2 as isize))
                .inverse = 1 as libc::c_int as uint8_t;
            (*((*sweep).spans).offset(n_2 as isize))
                .coverage = 0 as libc::c_int as uint8_t;
        }
    }
}
#[inline]
unsafe extern "C" fn sweep_line_delete(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) {
    if (*sweep).insert_cursor == rectangle {
        let ref mut fresh52 = (*sweep).insert_cursor;
        *fresh52 = (*rectangle).next;
    }
    let ref mut fresh53 = (*(*rectangle).prev).next;
    *fresh53 = (*rectangle).next;
    let ref mut fresh54 = (*(*rectangle).next).prev;
    *fresh54 = (*rectangle).prev;
    pqueue_pop(&mut (*sweep).stop);
}
#[inline]
unsafe extern "C" fn sweep_line_insert(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) {
    let mut pos: *mut rectangle_t = 0 as *mut rectangle_t;
    pos = (*sweep).insert_cursor;
    if (*pos).left != (*rectangle).left {
        if (*pos).left > (*rectangle).left {
            while !((*(*pos).prev).left < (*rectangle).left) {
                pos = (*pos).prev;
                if (*(*pos).prev).left < (*rectangle).left {
                    break;
                }
                pos = (*pos).prev;
                if (*(*pos).prev).left < (*rectangle).left {
                    break;
                }
                pos = (*pos).prev;
            }
        } else {
            loop {
                pos = (*pos).next;
                if (*pos).left >= (*rectangle).left {
                    break;
                }
                pos = (*pos).next;
                if (*pos).left >= (*rectangle).left {
                    break;
                }
                pos = (*pos).next;
                if (*pos).left >= (*rectangle).left {
                    break;
                }
            }
        }
    }
    let ref mut fresh55 = (*(*pos).prev).next;
    *fresh55 = rectangle;
    let ref mut fresh56 = (*rectangle).prev;
    *fresh56 = (*pos).prev;
    let ref mut fresh57 = (*rectangle).next;
    *fresh57 = pos;
    let ref mut fresh58 = (*pos).prev;
    *fresh58 = rectangle;
    let ref mut fresh59 = (*sweep).insert_cursor;
    *fresh59 = rectangle;
    pqueue_push(sweep, rectangle);
}
unsafe extern "C" fn render_rows(
    mut sweep_line: *mut sweep_line_t,
    mut renderer: *mut cairo_span_renderer_t,
    mut height: libc::c_int,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _active_edges_to_spans(sweep_line);
    status = ((*renderer).render_rows)
        .expect(
            "non-null function pointer",
        )(
        renderer as *mut libc::c_void,
        (*sweep_line).current_y,
        height,
        (*sweep_line).spans,
        (*sweep_line).num_spans,
    );
    if status as u64 != 0 {
        longjmp(((*sweep_line).jmpbuf).as_mut_ptr(), status as libc::c_int);
    }
}
unsafe extern "C" fn generate(
    mut self_0: *mut cairo_rectangular_scan_converter_t,
    mut renderer: *mut cairo_span_renderer_t,
    mut rectangles: *mut *mut rectangle_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut sweep_line: sweep_line_t = sweep_line_t {
        start: 0 as *mut *mut rectangle_t,
        stop: pqueue_t {
            size: 0,
            max_size: 0,
            elements: 0 as *mut *mut rectangle_t,
            elements_embedded: [0 as *mut rectangle_t; 1024],
        },
        head: rectangle_t {
            next: 0 as *mut _rectangle,
            prev: 0 as *mut _rectangle,
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            top_y: 0,
            bottom_y: 0,
            dir: 0,
        },
        tail: rectangle_t {
            next: 0 as *mut _rectangle,
            prev: 0 as *mut _rectangle,
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            top_y: 0,
            bottom_y: 0,
            dir: 0,
        },
        insert_cursor: 0 as *mut rectangle_t,
        current_y: 0,
        xmin: 0,
        xmax: 0,
        coverage: coverage {
            head: cell {
                prev: 0 as *mut cell,
                next: 0 as *mut cell,
                x: 0,
                covered: 0,
                uncovered: 0,
            },
            tail: cell {
                prev: 0 as *mut cell,
                next: 0 as *mut cell,
                x: 0,
                covered: 0,
                uncovered: 0,
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
        spans_stack: [cairo_half_open_span_t {
            x: 0,
            coverage: 0,
            inverse: 0,
        }; 256],
        spans: 0 as *mut cairo_half_open_span_t,
        num_spans: 0,
        size_spans: 0,
        jmpbuf: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    let mut start: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut stop: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    sweep_line_init(&mut sweep_line);
    sweep_line.xmin = _cairo_fixed_integer_part((*self_0).extents.p1.x);
    sweep_line.xmax = _cairo_fixed_integer_part((*self_0).extents.p2.x);
    sweep_line.start = rectangles;
    status = _setjmp((sweep_line.jmpbuf).as_mut_ptr()) as cairo_status_t;
    if !(status as u64 != 0) {
        sweep_line.current_y = _cairo_fixed_integer_part((*self_0).extents.p1.y);
        let fresh60 = sweep_line.start;
        sweep_line.start = (sweep_line.start).offset(1);
        start = *fresh60;
        's_44: loop {
            if (*start).top_y != sweep_line.current_y {
                render_rows(
                    &mut sweep_line,
                    renderer,
                    (*start).top_y - sweep_line.current_y,
                );
                sweep_line.current_y = (*start).top_y;
            }
            loop {
                sweep_line_insert(&mut sweep_line, start);
                let fresh61 = sweep_line.start;
                sweep_line.start = (sweep_line.start).offset(1);
                start = *fresh61;
                if start.is_null() {
                    break 's_44;
                }
                if (*start).top_y != sweep_line.current_y {
                    break;
                }
            }
            render_rows(&mut sweep_line, renderer, 1 as libc::c_int);
            stop = peek_stop(&mut sweep_line);
            while (*stop).bottom_y == sweep_line.current_y {
                sweep_line_delete(&mut sweep_line, stop);
                stop = peek_stop(&mut sweep_line);
                if stop.is_null() {
                    break;
                }
            }
            sweep_line.current_y += 1;
            while !stop.is_null() && (*stop).bottom_y < (*start).top_y {
                if (*stop).bottom_y != sweep_line.current_y {
                    render_rows(
                        &mut sweep_line,
                        renderer,
                        (*stop).bottom_y - sweep_line.current_y,
                    );
                    sweep_line.current_y = (*stop).bottom_y;
                }
                render_rows(&mut sweep_line, renderer, 1 as libc::c_int);
                loop {
                    sweep_line_delete(&mut sweep_line, stop);
                    stop = peek_stop(&mut sweep_line);
                    if !(!stop.is_null() && (*stop).bottom_y == sweep_line.current_y) {
                        break;
                    }
                }
                sweep_line.current_y += 1;
            }
        }
        render_rows(&mut sweep_line, renderer, 1 as libc::c_int);
        stop = peek_stop(&mut sweep_line);
        loop {
            if !((*stop).bottom_y == sweep_line.current_y) {
                current_block = 3938820862080741272;
                break;
            }
            sweep_line_delete(&mut sweep_line, stop);
            stop = peek_stop(&mut sweep_line);
            if stop.is_null() {
                current_block = 7923380806181908292;
                break;
            }
        }
        match current_block {
            7923380806181908292 => {}
            _ => {
                's_222: loop {
                    sweep_line.current_y += 1;
                    if !(sweep_line.current_y
                        < _cairo_fixed_integer_part((*self_0).extents.p2.y))
                    {
                        break;
                    }
                    if (*stop).bottom_y != sweep_line.current_y {
                        render_rows(
                            &mut sweep_line,
                            renderer,
                            (*stop).bottom_y - sweep_line.current_y,
                        );
                        sweep_line.current_y = (*stop).bottom_y;
                    }
                    render_rows(&mut sweep_line, renderer, 1 as libc::c_int);
                    loop {
                        sweep_line_delete(&mut sweep_line, stop);
                        stop = peek_stop(&mut sweep_line);
                        if stop.is_null() {
                            break 's_222;
                        }
                        if !((*stop).bottom_y == sweep_line.current_y) {
                            break;
                        }
                    }
                }
            }
        }
    }
    sweep_line_fini(&mut sweep_line);
    return status;
}
unsafe extern "C" fn generate_row(
    mut renderer: *mut cairo_span_renderer_t,
    mut r: *const rectangle_t,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut coverage: uint16_t,
) {
    let mut spans: [cairo_half_open_span_t; 4] = [cairo_half_open_span_t {
        x: 0,
        coverage: 0,
        inverse: 0,
    }; 4];
    let mut num_spans: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut x1: libc::c_int = _cairo_fixed_integer_part((*r).left);
    let mut x2: libc::c_int = _cairo_fixed_integer_part((*r).right);
    if x2 > x1 {
        if _cairo_fixed_is_integer((*r).left) == 0 {
            spans[num_spans as usize].x = x1;
            spans[num_spans as usize]
                .coverage = (coverage as libc::c_int
                * (256 as libc::c_int - _cairo_fixed_fractional_part((*r).left))
                >> 8 as libc::c_int) as uint8_t;
            num_spans = num_spans.wrapping_add(1);
            x1 += 1;
        }
        if x2 > x1 {
            spans[num_spans as usize].x = x1;
            spans[num_spans as usize]
                .coverage = (coverage as libc::c_int
                - (coverage as libc::c_int >> 8 as libc::c_int)) as uint8_t;
            num_spans = num_spans.wrapping_add(1);
        }
        if _cairo_fixed_is_integer((*r).right) == 0 {
            let fresh62 = x2;
            x2 = x2 + 1;
            spans[num_spans as usize].x = fresh62;
            spans[num_spans as usize]
                .coverage = (coverage as libc::c_int
                * _cairo_fixed_fractional_part((*r).right) >> 8 as libc::c_int)
                as uint8_t;
            num_spans = num_spans.wrapping_add(1);
        }
    } else {
        let fresh63 = x2;
        x2 = x2 + 1;
        spans[num_spans as usize].x = fresh63;
        spans[num_spans as usize]
            .coverage = (coverage as libc::c_int * ((*r).right - (*r).left)
            >> 8 as libc::c_int) as uint8_t;
        num_spans = num_spans.wrapping_add(1);
    }
    spans[num_spans as usize].x = x2;
    spans[num_spans as usize].coverage = 0 as libc::c_int as uint8_t;
    num_spans = num_spans.wrapping_add(1);
    ((*renderer).render_rows)
        .expect(
            "non-null function pointer",
        )(renderer as *mut libc::c_void, y, h, spans.as_mut_ptr(), num_spans);
}
unsafe extern "C" fn generate_box(
    mut self_0: *mut cairo_rectangular_scan_converter_t,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut r: *const rectangle_t = (*self_0).chunks.base as *const rectangle_t;
    let mut y1: libc::c_int = _cairo_fixed_integer_part((*r).top);
    let mut y2: libc::c_int = _cairo_fixed_integer_part((*r).bottom);
    if y2 > y1 {
        if _cairo_fixed_is_integer((*r).top) == 0 {
            generate_row(
                renderer,
                r,
                y1,
                1 as libc::c_int,
                (256 as libc::c_int - _cairo_fixed_fractional_part((*r).top)) as uint16_t,
            );
            y1 += 1;
        }
        if y2 > y1 {
            generate_row(renderer, r, y1, y2 - y1, 256 as libc::c_int as uint16_t);
        }
        if _cairo_fixed_is_integer((*r).bottom) == 0 {
            generate_row(
                renderer,
                r,
                y2,
                1 as libc::c_int,
                _cairo_fixed_fractional_part((*r).bottom) as uint16_t,
            );
        }
    } else {
        generate_row(
            renderer,
            r,
            y1,
            1 as libc::c_int,
            ((*r).bottom - (*r).top) as uint16_t,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectangular_scan_converter_generate(
    mut converter: *mut libc::c_void,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_rectangular_scan_converter_t = converter
        as *mut cairo_rectangular_scan_converter_t;
    let mut rectangles_stack: [*mut rectangle_t; 256] = [0 as *mut rectangle_t; 256];
    let mut rectangles: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut chunk: *mut _cairo_rectangular_scan_converter_chunk = 0
        as *mut _cairo_rectangular_scan_converter_chunk;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*self_0).num_rectangles == 0 as libc::c_int {
        return ((*renderer).render_rows)
            .expect(
                "non-null function pointer",
            )(
            renderer as *mut libc::c_void,
            _cairo_fixed_integer_part((*self_0).extents.p1.y),
            _cairo_fixed_integer_part((*self_0).extents.p2.y - (*self_0).extents.p1.y),
            0 as *const cairo_half_open_span_t,
            0 as libc::c_int as libc::c_uint,
        );
    }
    if (*self_0).num_rectangles == 1 as libc::c_int {
        return generate_box(self_0, renderer);
    }
    rectangles = rectangles_stack.as_mut_ptr();
    if (*self_0).num_rectangles
        >= (::std::mem::size_of::<[*mut rectangle_t; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        rectangles = _cairo_malloc_ab(
            ((*self_0).num_rectangles + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
        ) as *mut *mut rectangle_t;
        if rectangles.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    j = 0 as libc::c_int;
    chunk = &mut (*self_0).chunks;
    while !chunk.is_null() {
        let mut rectangle: *mut rectangle_t = 0 as *mut rectangle_t;
        rectangle = (*chunk).base as *mut rectangle_t;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let fresh64 = j;
            j = j + 1;
            let ref mut fresh65 = *rectangles.offset(fresh64 as isize);
            *fresh65 = &mut *rectangle.offset(i as isize) as *mut rectangle_t;
            i += 1;
        }
        chunk = (*chunk).next;
    }
    rectangle_sort(rectangles, j as libc::c_uint);
    let ref mut fresh66 = *rectangles.offset(j as isize);
    *fresh66 = 0 as *mut rectangle_t;
    status = generate(self_0, renderer, rectangles);
    if rectangles != rectangles_stack.as_mut_ptr() {
        free(rectangles as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _allocate_rectangle(
    mut self_0: *mut cairo_rectangular_scan_converter_t,
) -> *mut rectangle_t {
    let mut rectangle: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut chunk: *mut _cairo_rectangular_scan_converter_chunk = 0
        as *mut _cairo_rectangular_scan_converter_chunk;
    chunk = (*self_0).tail;
    if (*chunk).count == (*chunk).size {
        let mut size: libc::c_int = 0;
        size = (*chunk).size * 2 as libc::c_int;
        let ref mut fresh67 = (*chunk).next;
        *fresh67 = _cairo_malloc_ab_plus_c(
            size as size_t,
            ::std::mem::size_of::<rectangle_t>() as libc::c_ulong,
            ::std::mem::size_of::<_cairo_rectangular_scan_converter_chunk>()
                as libc::c_ulong,
        ) as *mut _cairo_rectangular_scan_converter_chunk;
        if ((*chunk).next).is_null() {
            return 0 as *mut rectangle_t;
        }
        chunk = (*chunk).next;
        let ref mut fresh68 = (*chunk).next;
        *fresh68 = 0 as *mut _cairo_rectangular_scan_converter_chunk;
        (*chunk).count = 0 as libc::c_int;
        (*chunk).size = size;
        let ref mut fresh69 = (*chunk).base;
        *fresh69 = chunk.offset(1 as libc::c_int as isize) as *mut libc::c_void;
        let ref mut fresh70 = (*self_0).tail;
        *fresh70 = chunk;
    }
    rectangle = (*chunk).base as *mut rectangle_t;
    let ref mut fresh71 = (*chunk).count;
    let fresh72 = *fresh71;
    *fresh71 = *fresh71 + 1;
    return rectangle.offset(fresh72 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rectangular_scan_converter_add_box(
    mut self_0: *mut cairo_rectangular_scan_converter_t,
    mut box_0: *const cairo_box_t,
    mut dir: libc::c_int,
) -> cairo_status_t {
    let mut rectangle: *mut rectangle_t = 0 as *mut rectangle_t;
    rectangle = _allocate_rectangle(self_0);
    if rectangle.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*rectangle).dir = dir;
    (*rectangle)
        .left = if (*box_0).p1.x > (*self_0).extents.p1.x {
        (*box_0).p1.x
    } else {
        (*self_0).extents.p1.x
    };
    (*rectangle)
        .right = if (*box_0).p2.x < (*self_0).extents.p2.x {
        (*box_0).p2.x
    } else {
        (*self_0).extents.p2.x
    };
    if (*rectangle).right <= (*rectangle).left {
        let ref mut fresh73 = (*(*self_0).tail).count;
        *fresh73 -= 1;
        return CAIRO_STATUS_SUCCESS;
    }
    (*rectangle)
        .top = if (*box_0).p1.y > (*self_0).extents.p1.y {
        (*box_0).p1.y
    } else {
        (*self_0).extents.p1.y
    };
    (*rectangle).top_y = _cairo_fixed_integer_floor((*rectangle).top);
    (*rectangle)
        .bottom = if (*box_0).p2.y < (*self_0).extents.p2.y {
        (*box_0).p2.y
    } else {
        (*self_0).extents.p2.y
    };
    (*rectangle).bottom_y = _cairo_fixed_integer_floor((*rectangle).bottom);
    if (*rectangle).bottom > (*rectangle).top {
        let ref mut fresh74 = (*self_0).num_rectangles;
        *fresh74 += 1;
    } else {
        let ref mut fresh75 = (*(*self_0).tail).count;
        *fresh75 -= 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_rectangular_scan_converter_destroy(
    mut converter: *mut libc::c_void,
) {
    let mut self_0: *mut cairo_rectangular_scan_converter_t = converter
        as *mut cairo_rectangular_scan_converter_t;
    let mut chunk: *mut _cairo_rectangular_scan_converter_chunk = 0
        as *mut _cairo_rectangular_scan_converter_chunk;
    let mut next: *mut _cairo_rectangular_scan_converter_chunk = 0
        as *mut _cairo_rectangular_scan_converter_chunk;
    chunk = (*self_0).chunks.next;
    while !chunk.is_null() {
        next = (*chunk).next;
        free(chunk as *mut libc::c_void);
        chunk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rectangular_scan_converter_init(
    mut self_0: *mut cairo_rectangular_scan_converter_t,
    mut extents: *const cairo_rectangle_int_t,
) {
    let ref mut fresh76 = (*self_0).base.destroy;
    *fresh76 = Some(
        _cairo_rectangular_scan_converter_destroy
            as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh77 = (*self_0).base.generate;
    *fresh77 = Some(
        _cairo_rectangular_scan_converter_generate
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut cairo_span_renderer_t,
            ) -> cairo_status_t,
    );
    _cairo_box_from_rectangle(&mut (*self_0).extents, extents);
    let ref mut fresh78 = (*self_0).chunks.base;
    *fresh78 = ((*self_0).buf).as_mut_ptr() as *mut libc::c_void;
    let ref mut fresh79 = (*self_0).chunks.next;
    *fresh79 = 0 as *mut _cairo_rectangular_scan_converter_chunk;
    (*self_0).chunks.count = 0 as libc::c_int;
    (*self_0)
        .chunks
        .size = (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
        as libc::c_int;
    let ref mut fresh80 = (*self_0).tail;
    *fresh80 = &mut (*self_0).chunks;
    (*self_0).num_rectangles = 0 as libc::c_int;
}
