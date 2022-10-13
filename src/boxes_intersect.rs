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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_limit(
        boxes: *mut cairo_boxes_t,
        limits: *const cairo_box_t,
        num_limits: libc::c_int,
    );
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_clear(boxes: *mut cairo_boxes_t);
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
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
pub type rectangle_t = _rectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rectangle {
    pub left: edge_t,
    pub right: edge_t,
    pub top: int32_t,
    pub bottom: int32_t,
}
pub type edge_t = _edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _edge {
    pub next: *mut edge_t,
    pub prev: *mut edge_t,
    pub right: *mut edge_t,
    pub x: cairo_fixed_t,
    pub top: cairo_fixed_t,
    pub a_or_b: libc::c_int,
    pub dir: libc::c_int,
}
pub type sweep_line_t = _sweep_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sweep_line {
    pub rectangles: *mut *mut rectangle_t,
    pub pq: pqueue_t,
    pub head: edge_t,
    pub tail: edge_t,
    pub insert_left: *mut edge_t,
    pub insert_right: *mut edge_t,
    pub current_y: int32_t,
    pub last_y: int32_t,
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
unsafe extern "C" fn rectangle_compare_start(
    mut a: *const rectangle_t,
    mut b: *const rectangle_t,
) -> libc::c_int {
    return (*a).top - (*b).top;
}
#[inline]
unsafe extern "C" fn rectangle_compare_stop(
    mut a: *const rectangle_t,
    mut b: *const rectangle_t,
) -> libc::c_int {
    return (*a).bottom - (*b).bottom;
}
#[inline]
unsafe extern "C" fn pqueue_init(mut pq: *mut pqueue_t) {
    (*pq)
        .max_size = (::std::mem::size_of::<[*mut rectangle_t; 1024]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong)
        as libc::c_int;
    (*pq).size = 0 as libc::c_int;
    let ref mut fresh8 = (*pq).elements;
    *fresh8 = ((*pq).elements_embedded).as_mut_ptr();
    let ref mut fresh9 = *((*pq).elements).offset(1 as libc::c_int as isize);
    *fresh9 = 0 as *mut rectangle_t;
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
    let ref mut fresh10 = (*pq).elements;
    *fresh10 = new_elements;
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
    if (*sweep).pq.size + 1 as libc::c_int == (*sweep).pq.max_size {
        if pqueue_grow(&mut (*sweep).pq) == 0 {
            longjmp(
                ((*sweep).unwind).as_mut_ptr(),
                _cairo_error(CAIRO_STATUS_NO_MEMORY) as libc::c_int,
            );
        }
    }
    elements = (*sweep).pq.elements;
    let ref mut fresh11 = (*sweep).pq.size;
    *fresh11 += 1;
    i = *fresh11;
    while i != 1 as libc::c_int
        && {
            parent = i >> 1 as libc::c_int;
            rectangle_compare_stop(rectangle, *elements.offset(parent as isize))
                < 0 as libc::c_int
        }
    {
        let ref mut fresh12 = *elements.offset(i as isize);
        *fresh12 = *elements.offset(parent as isize);
        i = parent;
    }
    let ref mut fresh13 = *elements.offset(i as isize);
    *fresh13 = rectangle;
}
#[inline]
unsafe extern "C" fn pqueue_pop(mut pq: *mut pqueue_t) {
    let mut elements: *mut *mut rectangle_t = (*pq).elements;
    let mut tail: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let ref mut fresh14 = (*pq).size;
    let fresh15 = *fresh14;
    *fresh14 = *fresh14 - 1;
    tail = *elements.offset(fresh15 as isize);
    if (*pq).size == 0 as libc::c_int {
        let ref mut fresh16 = *elements.offset(1 as libc::c_int as isize);
        *fresh16 = 0 as *mut rectangle_t;
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
        let ref mut fresh17 = *elements.offset(i as isize);
        *fresh17 = *elements.offset(child as isize);
        i = child;
    }
    let ref mut fresh18 = *elements.offset(i as isize);
    *fresh18 = tail;
}
#[inline]
unsafe extern "C" fn rectangle_pop_start(
    mut sweep_line: *mut sweep_line_t,
) -> *mut rectangle_t {
    let ref mut fresh19 = (*sweep_line).rectangles;
    let fresh20 = *fresh19;
    *fresh19 = (*fresh19).offset(1);
    return *fresh20;
}
#[inline]
unsafe extern "C" fn rectangle_peek_stop(
    mut sweep_line: *mut sweep_line_t,
) -> *mut rectangle_t {
    return *((*sweep_line).pq.elements).offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn _rectangle_sort(
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
                let ref mut fresh21 = *base.offset(i as isize);
                *fresh21 = *base.offset(j as isize);
                let ref mut fresh22 = *base.offset(j as isize);
                *fresh22 = tmp;
                swapped = 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        if !(swapped != 0) {
            break;
        }
    };
}
unsafe extern "C" fn sweep_line_init(
    mut sweep_line: *mut sweep_line_t,
    mut rectangles: *mut *mut rectangle_t,
    mut num_rectangles: libc::c_int,
) {
    _rectangle_sort(rectangles, num_rectangles as libc::c_uint);
    let ref mut fresh23 = *rectangles.offset(num_rectangles as isize);
    *fresh23 = 0 as *mut rectangle_t;
    let ref mut fresh24 = (*sweep_line).rectangles;
    *fresh24 = rectangles;
    (*sweep_line).head.x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh25 = (*sweep_line).head.right;
    *fresh25 = 0 as *mut edge_t;
    (*sweep_line).head.dir = 0 as libc::c_int;
    let ref mut fresh26 = (*sweep_line).head.next;
    *fresh26 = &mut (*sweep_line).tail;
    (*sweep_line).tail.x = 2147483647 as libc::c_int;
    let ref mut fresh27 = (*sweep_line).tail.right;
    *fresh27 = 0 as *mut edge_t;
    (*sweep_line).tail.dir = 0 as libc::c_int;
    let ref mut fresh28 = (*sweep_line).tail.prev;
    *fresh28 = &mut (*sweep_line).head;
    let ref mut fresh29 = (*sweep_line).insert_left;
    *fresh29 = &mut (*sweep_line).tail;
    let ref mut fresh30 = (*sweep_line).insert_right;
    *fresh30 = &mut (*sweep_line).tail;
    (*sweep_line).current_y = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*sweep_line).last_y = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    pqueue_init(&mut (*sweep_line).pq);
}
unsafe extern "C" fn sweep_line_fini(mut sweep_line: *mut sweep_line_t) {
    pqueue_fini(&mut (*sweep_line).pq);
}
unsafe extern "C" fn end_box(
    mut sweep_line: *mut sweep_line_t,
    mut left: *mut edge_t,
    mut bot: int32_t,
    mut out: *mut cairo_boxes_t,
) {
    if (*left).top < bot {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        box_0.p1.x = (*left).x;
        box_0.p1.y = (*left).top;
        box_0.p2.x = (*(*left).right).x;
        box_0.p2.y = bot;
        status = _cairo_boxes_add(out, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        if status as u64 != 0 {
            longjmp(((*sweep_line).unwind).as_mut_ptr(), status as libc::c_int);
        }
    }
    let ref mut fresh31 = (*left).right;
    *fresh31 = 0 as *mut edge_t;
}
#[inline]
unsafe extern "C" fn start_or_continue_box(
    mut sweep_line: *mut sweep_line_t,
    mut left: *mut edge_t,
    mut right: *mut edge_t,
    mut top: libc::c_int,
    mut out: *mut cairo_boxes_t,
) {
    if (*left).right == right {
        return;
    }
    if !((*left).right).is_null() {
        if !right.is_null() && (*(*left).right).x == (*right).x {
            let ref mut fresh32 = (*left).right;
            *fresh32 = right;
            return;
        }
        end_box(sweep_line, left, top, out);
    }
    if !right.is_null() && (*left).x != (*right).x {
        (*left).top = top;
        let ref mut fresh33 = (*left).right;
        *fresh33 = right;
    }
}
#[inline]
unsafe extern "C" fn is_zero(mut winding: *const libc::c_int) -> libc::c_int {
    return (*winding.offset(0 as libc::c_int as isize) == 0 as libc::c_int
        || *winding.offset(1 as libc::c_int as isize) == 0 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn active_edges(
    mut sweep: *mut sweep_line_t,
    mut out: *mut cairo_boxes_t,
) {
    let mut top: libc::c_int = (*sweep).current_y;
    let mut winding: [libc::c_int; 2] = [0 as libc::c_int, 0];
    let mut pos: *mut edge_t = 0 as *mut edge_t;
    if (*sweep).last_y == (*sweep).current_y {
        return;
    }
    pos = (*sweep).head.next;
    if pos == &mut (*sweep).tail as *mut edge_t {
        return;
    }
    's_34: loop {
        let mut left: *mut edge_t = 0 as *mut edge_t;
        let mut right: *mut edge_t = 0 as *mut edge_t;
        left = pos;
        loop {
            winding[(*left).a_or_b as usize] += (*left).dir;
            if is_zero(winding.as_mut_ptr()) == 0 {
                break;
            }
            if (*left).next == &mut (*sweep).tail as *mut edge_t {
                break 's_34;
            }
            if !((*left).right).is_null() {
                end_box(sweep, left, top, out);
            }
            left = (*left).next;
        }
        right = (*left).next;
        loop {
            if !((*right).right).is_null() {
                end_box(sweep, right, top, out);
            }
            winding[(*right).a_or_b as usize] += (*right).dir;
            if is_zero(winding.as_mut_ptr()) != 0 {
                if (*right).x != (*(*right).next).x {
                    break;
                }
            }
            right = (*right).next;
        }
        start_or_continue_box(sweep, left, right, top, out);
        pos = (*right).next;
        if !(pos != &mut (*sweep).tail as *mut edge_t) {
            break;
        }
    }
    (*sweep).last_y = (*sweep).current_y;
}
#[inline]
unsafe extern "C" fn sweep_line_delete_edge(
    mut sweep_line: *mut sweep_line_t,
    mut edge: *mut edge_t,
    mut out: *mut cairo_boxes_t,
) {
    if !((*edge).right).is_null() {
        let mut next: *mut edge_t = (*edge).next;
        if (*next).x == (*edge).x {
            (*next).top = (*edge).top;
            let ref mut fresh34 = (*next).right;
            *fresh34 = (*edge).right;
        } else {
            end_box(sweep_line, edge, (*sweep_line).current_y, out);
        }
    }
    if (*sweep_line).insert_left == edge {
        let ref mut fresh35 = (*sweep_line).insert_left;
        *fresh35 = (*edge).next;
    }
    if (*sweep_line).insert_right == edge {
        let ref mut fresh36 = (*sweep_line).insert_right;
        *fresh36 = (*edge).next;
    }
    let ref mut fresh37 = (*(*edge).prev).next;
    *fresh37 = (*edge).next;
    let ref mut fresh38 = (*(*edge).next).prev;
    *fresh38 = (*edge).prev;
}
#[inline]
unsafe extern "C" fn sweep_line_delete(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
    mut out: *mut cairo_boxes_t,
) {
    sweep_line_delete_edge(sweep, &mut (*rectangle).left, out);
    sweep_line_delete_edge(sweep, &mut (*rectangle).right, out);
    pqueue_pop(&mut (*sweep).pq);
}
#[inline]
unsafe extern "C" fn insert_edge(mut edge: *mut edge_t, mut pos: *mut edge_t) {
    if (*pos).x != (*edge).x {
        if (*pos).x > (*edge).x {
            while !((*(*pos).prev).x <= (*edge).x) {
                pos = (*pos).prev;
                if (*(*pos).prev).x <= (*edge).x {
                    break;
                }
                pos = (*pos).prev;
                if (*(*pos).prev).x <= (*edge).x {
                    break;
                }
                pos = (*pos).prev;
            }
        } else {
            loop {
                pos = (*pos).next;
                if (*pos).x >= (*edge).x {
                    break;
                }
                pos = (*pos).next;
                if (*pos).x >= (*edge).x {
                    break;
                }
                pos = (*pos).next;
                if (*pos).x >= (*edge).x {
                    break;
                }
            }
        }
    }
    let ref mut fresh39 = (*(*pos).prev).next;
    *fresh39 = edge;
    let ref mut fresh40 = (*edge).prev;
    *fresh40 = (*pos).prev;
    let ref mut fresh41 = (*edge).next;
    *fresh41 = pos;
    let ref mut fresh42 = (*pos).prev;
    *fresh42 = edge;
}
#[inline]
unsafe extern "C" fn sweep_line_insert(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) {
    let mut pos: *mut edge_t = 0 as *mut edge_t;
    pos = (*sweep).insert_right;
    insert_edge(&mut (*rectangle).right, pos);
    let ref mut fresh43 = (*sweep).insert_right;
    *fresh43 = &mut (*rectangle).right;
    pos = (*sweep).insert_left;
    if (*pos).x > (*(*sweep).insert_right).x {
        pos = (*(*sweep).insert_right).prev;
    }
    insert_edge(&mut (*rectangle).left, pos);
    let ref mut fresh44 = (*sweep).insert_left;
    *fresh44 = &mut (*rectangle).left;
    pqueue_push(sweep, rectangle);
}
unsafe extern "C" fn intersect(
    mut rectangles: *mut *mut rectangle_t,
    mut num_rectangles: libc::c_int,
    mut out: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut sweep_line: sweep_line_t = sweep_line_t {
        rectangles: 0 as *mut *mut rectangle_t,
        pq: pqueue_t {
            size: 0,
            max_size: 0,
            elements: 0 as *mut *mut rectangle_t,
            elements_embedded: [0 as *mut rectangle_t; 1024],
        },
        head: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            a_or_b: 0,
            dir: 0,
        },
        tail: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            a_or_b: 0,
            dir: 0,
        },
        insert_left: 0 as *mut edge_t,
        insert_right: 0 as *mut edge_t,
        current_y: 0,
        last_y: 0,
        unwind: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    let mut rectangle: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    sweep_line_init(&mut sweep_line, rectangles, num_rectangles);
    status = _setjmp((sweep_line.unwind).as_mut_ptr()) as cairo_status_t;
    if !(status as u64 != 0) {
        rectangle = rectangle_pop_start(&mut sweep_line);
        loop {
            if (*rectangle).top != sweep_line.current_y {
                let mut stop: *mut rectangle_t = 0 as *mut rectangle_t;
                stop = rectangle_peek_stop(&mut sweep_line);
                while !stop.is_null() && (*stop).bottom < (*rectangle).top {
                    if (*stop).bottom != sweep_line.current_y {
                        active_edges(&mut sweep_line, out);
                        sweep_line.current_y = (*stop).bottom;
                    }
                    sweep_line_delete(&mut sweep_line, stop, out);
                    stop = rectangle_peek_stop(&mut sweep_line);
                }
                active_edges(&mut sweep_line, out);
                sweep_line.current_y = (*rectangle).top;
            }
            sweep_line_insert(&mut sweep_line, rectangle);
            rectangle = rectangle_pop_start(&mut sweep_line);
            if rectangle.is_null() {
                break;
            }
        }
        loop {
            rectangle = rectangle_peek_stop(&mut sweep_line);
            if rectangle.is_null() {
                break;
            }
            if (*rectangle).bottom != sweep_line.current_y {
                active_edges(&mut sweep_line, out);
                sweep_line.current_y = (*rectangle).bottom;
            }
            sweep_line_delete(&mut sweep_line, rectangle, out);
        }
    }
    sweep_line_fini(&mut sweep_line);
    return status;
}
unsafe extern "C" fn _cairo_boxes_intersect_with_box(
    mut boxes: *const cairo_boxes_t,
    mut box_0: *const cairo_box_t,
    mut out: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if out == boxes as *mut cairo_boxes_t {
        let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
        (*out).num_boxes = 0 as libc::c_int;
        chunk = &mut (*out).chunks;
        while !chunk.is_null() {
            j = 0 as libc::c_int;
            i = j;
            while i < (*chunk).count {
                let mut b: *mut cairo_box_t = &mut *((*chunk).base).offset(i as isize)
                    as *mut cairo_box_t;
                (*b)
                    .p1
                    .x = if (*b).p1.x > (*box_0).p1.x {
                    (*b).p1.x
                } else {
                    (*box_0).p1.x
                };
                (*b)
                    .p1
                    .y = if (*b).p1.y > (*box_0).p1.y {
                    (*b).p1.y
                } else {
                    (*box_0).p1.y
                };
                (*b)
                    .p2
                    .x = if (*b).p2.x < (*box_0).p2.x {
                    (*b).p2.x
                } else {
                    (*box_0).p2.x
                };
                (*b)
                    .p2
                    .y = if (*b).p2.y < (*box_0).p2.y {
                    (*b).p2.y
                } else {
                    (*box_0).p2.y
                };
                if (*b).p1.x < (*b).p2.x && (*b).p1.y < (*b).p2.y {
                    if i != j {
                        *((*chunk).base).offset(j as isize) = *b;
                    }
                    j += 1;
                }
                i += 1;
            }
            (*chunk).count = j;
            (*out).num_boxes += j;
            chunk = (*chunk).next;
        }
    } else {
        let mut chunk_0: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
        _cairo_boxes_clear(out);
        _cairo_boxes_limit(out, box_0, 1 as libc::c_int);
        chunk_0 = &(*boxes).chunks;
        while !chunk_0.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk_0).count {
                status = _cairo_boxes_add(
                    out,
                    CAIRO_ANTIALIAS_DEFAULT,
                    &mut *((*chunk_0).base).offset(i as isize),
                );
                if status as u64 != 0 {
                    return status;
                }
                i += 1;
            }
            chunk_0 = (*chunk_0).next;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_boxes_intersect(
    mut a: *const cairo_boxes_t,
    mut b: *const cairo_boxes_t,
    mut out: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut stack_rectangles: [rectangle_t; 23] = [rectangle_t {
        left: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            a_or_b: 0,
            dir: 0,
        },
        right: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            a_or_b: 0,
            dir: 0,
        },
        top: 0,
        bottom: 0,
    }; 23];
    let mut rectangles: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut stack_rectangles_ptrs: [*mut rectangle_t; 24] = [0 as *mut rectangle_t; 24];
    let mut rectangles_ptrs: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if (*a).num_boxes == 0 as libc::c_int || (*b).num_boxes == 0 as libc::c_int {
        _cairo_boxes_clear(out);
        return CAIRO_STATUS_SUCCESS;
    }
    if (*a).num_boxes == 1 as libc::c_int {
        let mut box_0: cairo_box_t = *((*a).chunks.base)
            .offset(0 as libc::c_int as isize);
        return _cairo_boxes_intersect_with_box(b, &mut box_0, out);
    }
    if (*b).num_boxes == 1 as libc::c_int {
        let mut box_1: cairo_box_t = *((*b).chunks.base)
            .offset(0 as libc::c_int as isize);
        return _cairo_boxes_intersect_with_box(a, &mut box_1, out);
    }
    rectangles = stack_rectangles.as_mut_ptr();
    rectangles_ptrs = stack_rectangles_ptrs.as_mut_ptr();
    count = (*a).num_boxes + (*b).num_boxes;
    if count
        > (::std::mem::size_of::<[rectangle_t; 23]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        rectangles = _cairo_malloc_ab_plus_c(
            count as size_t,
            (::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
                ),
            ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
        ) as *mut rectangle_t;
        if rectangles.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        rectangles_ptrs = rectangles.offset(count as isize) as *mut *mut rectangle_t;
    }
    j = 0 as libc::c_int;
    chunk = &(*a).chunks;
    while !chunk.is_null() {
        let mut box_2: *const cairo_box_t = (*chunk).base;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            if (*box_2.offset(i as isize)).p1.x < (*box_2.offset(i as isize)).p2.x {
                (*rectangles.offset(j as isize))
                    .left
                    .x = (*box_2.offset(i as isize)).p1.x;
                (*rectangles.offset(j as isize)).left.dir = 1 as libc::c_int;
                (*rectangles.offset(j as isize))
                    .right
                    .x = (*box_2.offset(i as isize)).p2.x;
                (*rectangles.offset(j as isize)).right.dir = -(1 as libc::c_int);
            } else {
                (*rectangles.offset(j as isize))
                    .right
                    .x = (*box_2.offset(i as isize)).p1.x;
                (*rectangles.offset(j as isize)).right.dir = 1 as libc::c_int;
                (*rectangles.offset(j as isize))
                    .left
                    .x = (*box_2.offset(i as isize)).p2.x;
                (*rectangles.offset(j as isize)).left.dir = -(1 as libc::c_int);
            }
            (*rectangles.offset(j as isize)).left.a_or_b = 0 as libc::c_int;
            let ref mut fresh45 = (*rectangles.offset(j as isize)).left.right;
            *fresh45 = 0 as *mut edge_t;
            (*rectangles.offset(j as isize)).right.a_or_b = 0 as libc::c_int;
            let ref mut fresh46 = (*rectangles.offset(j as isize)).right.right;
            *fresh46 = 0 as *mut edge_t;
            (*rectangles.offset(j as isize)).top = (*box_2.offset(i as isize)).p1.y;
            (*rectangles.offset(j as isize)).bottom = (*box_2.offset(i as isize)).p2.y;
            let ref mut fresh47 = *rectangles_ptrs.offset(j as isize);
            *fresh47 = &mut *rectangles.offset(j as isize) as *mut rectangle_t;
            j += 1;
            i += 1;
        }
        chunk = (*chunk).next;
    }
    chunk = &(*b).chunks;
    while !chunk.is_null() {
        let mut box_3: *const cairo_box_t = (*chunk).base;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            if (*box_3.offset(i as isize)).p1.x < (*box_3.offset(i as isize)).p2.x {
                (*rectangles.offset(j as isize))
                    .left
                    .x = (*box_3.offset(i as isize)).p1.x;
                (*rectangles.offset(j as isize)).left.dir = 1 as libc::c_int;
                (*rectangles.offset(j as isize))
                    .right
                    .x = (*box_3.offset(i as isize)).p2.x;
                (*rectangles.offset(j as isize)).right.dir = -(1 as libc::c_int);
            } else {
                (*rectangles.offset(j as isize))
                    .right
                    .x = (*box_3.offset(i as isize)).p1.x;
                (*rectangles.offset(j as isize)).right.dir = 1 as libc::c_int;
                (*rectangles.offset(j as isize))
                    .left
                    .x = (*box_3.offset(i as isize)).p2.x;
                (*rectangles.offset(j as isize)).left.dir = -(1 as libc::c_int);
            }
            (*rectangles.offset(j as isize)).left.a_or_b = 1 as libc::c_int;
            let ref mut fresh48 = (*rectangles.offset(j as isize)).left.right;
            *fresh48 = 0 as *mut edge_t;
            (*rectangles.offset(j as isize)).right.a_or_b = 1 as libc::c_int;
            let ref mut fresh49 = (*rectangles.offset(j as isize)).right.right;
            *fresh49 = 0 as *mut edge_t;
            (*rectangles.offset(j as isize)).top = (*box_3.offset(i as isize)).p1.y;
            (*rectangles.offset(j as isize)).bottom = (*box_3.offset(i as isize)).p2.y;
            let ref mut fresh50 = *rectangles_ptrs.offset(j as isize);
            *fresh50 = &mut *rectangles.offset(j as isize) as *mut rectangle_t;
            j += 1;
            i += 1;
        }
        chunk = (*chunk).next;
    }
    if j == count {} else {
        __assert_fail(
            b"j == count\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-boxes-intersect.c\0" as *const u8 as *const libc::c_char,
            682 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"cairo_status_t _cairo_boxes_intersect(const cairo_boxes_t *, const cairo_boxes_t *, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_boxes_clear(out);
    status = intersect(rectangles_ptrs, j, out);
    if rectangles != stack_rectangles.as_mut_ptr() {
        free(rectangles as *mut libc::c_void);
    }
    return status;
}
