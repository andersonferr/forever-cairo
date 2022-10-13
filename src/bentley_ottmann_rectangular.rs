use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn _cairo_traps_clear(traps: *mut cairo_traps_t);
    fn _cairo_traps_add_trap(
        traps: *mut cairo_traps_t,
        top: cairo_fixed_t,
        bottom: cairo_fixed_t,
        left: *const cairo_line_t,
        right: *const cairo_line_t,
    );
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
pub type cairo_fill_rule_t = _cairo_fill_rule;
pub type _cairo_fill_rule = libc::c_uint;
pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;
pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
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
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
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
    pub dir: libc::c_int,
}
pub type sweep_line_t = _sweep_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sweep_line {
    pub rectangles: *mut *mut rectangle_t,
    pub stop: *mut *mut rectangle_t,
    pub head: edge_t,
    pub tail: edge_t,
    pub insert: *mut edge_t,
    pub cursor: *mut edge_t,
    pub current_y: int32_t,
    pub last_y: int32_t,
    pub stop_size: libc::c_int,
    pub insert_x: int32_t,
    pub fill_rule: cairo_fill_rule_t,
    pub do_traps: cairo_bool_t,
    pub container: *mut libc::c_void,
    pub unwind: jmp_buf,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
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
unsafe extern "C" fn pqueue_push(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) {
    let mut elements: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut i: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    elements = (*sweep).stop;
    let ref mut fresh6 = (*sweep).stop_size;
    *fresh6 += 1;
    i = *fresh6;
    while i != 1 as libc::c_int
        && {
            parent = i >> 1 as libc::c_int;
            rectangle_compare_stop(rectangle, *elements.offset(parent as isize))
                < 0 as libc::c_int
        }
    {
        let ref mut fresh7 = *elements.offset(i as isize);
        *fresh7 = *elements.offset(parent as isize);
        i = parent;
    }
    let ref mut fresh8 = *elements.offset(i as isize);
    *fresh8 = rectangle;
}
#[inline]
unsafe extern "C" fn rectangle_pop_stop(mut sweep: *mut sweep_line_t) {
    let mut elements: *mut *mut rectangle_t = (*sweep).stop;
    let mut tail: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let ref mut fresh9 = (*sweep).stop_size;
    let fresh10 = *fresh9;
    *fresh9 = *fresh9 - 1;
    tail = *elements.offset(fresh10 as isize);
    if (*sweep).stop_size == 0 as libc::c_int {
        let ref mut fresh11 = *elements.offset(1 as libc::c_int as isize);
        *fresh11 = 0 as *mut rectangle_t;
        return;
    }
    i = 1 as libc::c_int;
    loop {
        child = i << 1 as libc::c_int;
        if !(child <= (*sweep).stop_size) {
            break;
        }
        if child != (*sweep).stop_size
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
        let ref mut fresh12 = *elements.offset(i as isize);
        *fresh12 = *elements.offset(child as isize);
        i = child;
    }
    let ref mut fresh13 = *elements.offset(i as isize);
    *fresh13 = tail;
}
#[inline]
unsafe extern "C" fn rectangle_pop_start(
    mut sweep_line: *mut sweep_line_t,
) -> *mut rectangle_t {
    let ref mut fresh14 = (*sweep_line).rectangles;
    let fresh15 = *fresh14;
    *fresh14 = (*fresh14).offset(1);
    return *fresh15;
}
#[inline]
unsafe extern "C" fn rectangle_peek_stop(
    mut sweep_line: *mut sweep_line_t,
) -> *mut rectangle_t {
    return *((*sweep_line).stop).offset(1 as libc::c_int as isize);
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
                let ref mut fresh16 = *base.offset(i as isize);
                *fresh16 = *base.offset(j as isize);
                let ref mut fresh17 = *base.offset(j as isize);
                *fresh17 = tmp;
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
    mut fill_rule: cairo_fill_rule_t,
    mut do_traps: cairo_bool_t,
    mut container: *mut libc::c_void,
) {
    let ref mut fresh18 = *rectangles.offset(-(2 as libc::c_int) as isize);
    *fresh18 = 0 as *mut rectangle_t;
    let ref mut fresh19 = *rectangles.offset(-(1 as libc::c_int) as isize);
    *fresh19 = 0 as *mut rectangle_t;
    let ref mut fresh20 = *rectangles.offset(num_rectangles as isize);
    *fresh20 = 0 as *mut rectangle_t;
    let ref mut fresh21 = (*sweep_line).rectangles;
    *fresh21 = rectangles;
    let ref mut fresh22 = (*sweep_line).stop;
    *fresh22 = rectangles.offset(-(2 as libc::c_int as isize));
    (*sweep_line).stop_size = 0 as libc::c_int;
    let ref mut fresh23 = (*sweep_line).insert;
    *fresh23 = 0 as *mut edge_t;
    (*sweep_line).insert_x = 2147483647 as libc::c_int;
    let ref mut fresh24 = (*sweep_line).cursor;
    *fresh24 = &mut (*sweep_line).tail;
    (*sweep_line).head.dir = 0 as libc::c_int;
    (*sweep_line).head.x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh25 = (*sweep_line).head.right;
    *fresh25 = 0 as *mut edge_t;
    let ref mut fresh26 = (*sweep_line).head.prev;
    *fresh26 = 0 as *mut edge_t;
    let ref mut fresh27 = (*sweep_line).head.next;
    *fresh27 = &mut (*sweep_line).tail;
    let ref mut fresh28 = (*sweep_line).tail.prev;
    *fresh28 = &mut (*sweep_line).head;
    let ref mut fresh29 = (*sweep_line).tail.next;
    *fresh29 = 0 as *mut edge_t;
    let ref mut fresh30 = (*sweep_line).tail.right;
    *fresh30 = 0 as *mut edge_t;
    (*sweep_line).tail.x = 2147483647 as libc::c_int;
    (*sweep_line).tail.dir = 0 as libc::c_int;
    (*sweep_line).current_y = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*sweep_line).last_y = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    (*sweep_line).fill_rule = fill_rule;
    let ref mut fresh31 = (*sweep_line).container;
    *fresh31 = container;
    (*sweep_line).do_traps = do_traps;
}
unsafe extern "C" fn edge_end_box(
    mut sweep_line: *mut sweep_line_t,
    mut left: *mut edge_t,
    mut bot: int32_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*left).top < bot {
        if (*sweep_line).do_traps != 0 {
            let mut _left: cairo_line_t = {
                let mut init = _cairo_line {
                    p1: {
                        let mut init = _cairo_point {
                            x: (*left).x,
                            y: (*left).top,
                        };
                        init
                    },
                    p2: {
                        let mut init = _cairo_point {
                            x: (*left).x,
                            y: bot,
                        };
                        init
                    },
                };
                init
            };
            let mut _right: cairo_line_t = {
                let mut init = _cairo_line {
                    p1: {
                        let mut init = _cairo_point {
                            x: (*(*left).right).x,
                            y: (*left).top,
                        };
                        init
                    },
                    p2: {
                        let mut init = _cairo_point {
                            x: (*(*left).right).x,
                            y: bot,
                        };
                        init
                    },
                };
                init
            };
            _cairo_traps_add_trap(
                (*sweep_line).container as *mut cairo_traps_t,
                (*left).top,
                bot,
                &mut _left,
                &mut _right,
            );
            status = (*((*sweep_line).container as *mut cairo_traps_t)).status;
        } else {
            let mut box_0: cairo_box_t = cairo_line_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            box_0.p1.x = (*left).x;
            box_0.p1.y = (*left).top;
            box_0.p2.x = (*(*left).right).x;
            box_0.p2.y = bot;
            status = _cairo_boxes_add(
                (*sweep_line).container as *mut cairo_boxes_t,
                CAIRO_ANTIALIAS_DEFAULT,
                &mut box_0,
            );
        }
    }
    if status as u64 != 0 {
        longjmp(((*sweep_line).unwind).as_mut_ptr(), status as libc::c_int);
    }
    let ref mut fresh32 = (*left).right;
    *fresh32 = 0 as *mut edge_t;
}
#[inline]
unsafe extern "C" fn edge_start_or_continue_box(
    mut sweep_line: *mut sweep_line_t,
    mut left: *mut edge_t,
    mut right: *mut edge_t,
    mut top: libc::c_int,
) {
    if (*left).right == right {
        return;
    }
    if !((*left).right).is_null() {
        if (*(*left).right).x == (*right).x {
            let ref mut fresh33 = (*left).right;
            *fresh33 = right;
            return;
        }
        edge_end_box(sweep_line, left, top);
    }
    if (*left).x != (*right).x {
        (*left).top = top;
        let ref mut fresh34 = (*left).right;
        *fresh34 = right;
    }
}
unsafe extern "C" fn merge_sorted_edges(
    mut head_a: *mut edge_t,
    mut head_b: *mut edge_t,
) -> *mut edge_t {
    let mut current_block: u64;
    let mut head: *mut edge_t = 0 as *mut edge_t;
    let mut prev: *mut edge_t = 0 as *mut edge_t;
    let mut x: int32_t = 0;
    prev = (*head_a).prev;
    if (*head_a).x <= (*head_b).x {
        head = head_a;
        current_block = 10886091980245723256;
    } else {
        let ref mut fresh35 = (*head_b).prev;
        *fresh35 = prev;
        head = head_b;
        current_block = 495373699497437599;
    }
    loop {
        match current_block {
            10886091980245723256 => {
                x = (*head_b).x;
                while !head_a.is_null() && (*head_a).x <= x {
                    prev = head_a;
                    head_a = (*head_a).next;
                }
                let ref mut fresh36 = (*head_b).prev;
                *fresh36 = prev;
                let ref mut fresh37 = (*prev).next;
                *fresh37 = head_b;
                if head_a.is_null() {
                    return head;
                }
                current_block = 495373699497437599;
            }
            _ => {
                x = (*head_a).x;
                while !head_b.is_null() && (*head_b).x <= x {
                    prev = head_b;
                    head_b = (*head_b).next;
                }
                let ref mut fresh38 = (*head_a).prev;
                *fresh38 = prev;
                let ref mut fresh39 = (*prev).next;
                *fresh39 = head_a;
                if head_b.is_null() {
                    return head;
                }
                current_block = 10886091980245723256;
            }
        }
    };
}
unsafe extern "C" fn sort_edges(
    mut list: *mut edge_t,
    mut level: libc::c_uint,
    mut head_out: *mut *mut edge_t,
) -> *mut edge_t {
    let mut head_other: *mut edge_t = 0 as *mut edge_t;
    let mut remaining: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_uint = 0;
    head_other = (*list).next;
    if head_other.is_null() {
        *head_out = list;
        return 0 as *mut edge_t;
    }
    remaining = (*head_other).next;
    if (*list).x <= (*head_other).x {
        *head_out = list;
        let ref mut fresh40 = (*head_other).next;
        *fresh40 = 0 as *mut edge_t;
    } else {
        *head_out = head_other;
        let ref mut fresh41 = (*head_other).prev;
        *fresh41 = (*list).prev;
        let ref mut fresh42 = (*head_other).next;
        *fresh42 = list;
        let ref mut fresh43 = (*list).prev;
        *fresh43 = head_other;
        let ref mut fresh44 = (*list).next;
        *fresh44 = 0 as *mut edge_t;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < level && !remaining.is_null() {
        remaining = sort_edges(remaining, i, &mut head_other);
        *head_out = merge_sorted_edges(*head_out, head_other);
        i = i.wrapping_add(1);
    }
    return remaining;
}
unsafe extern "C" fn merge_unsorted_edges(
    mut head: *mut edge_t,
    mut unsorted: *mut edge_t,
) -> *mut edge_t {
    sort_edges(
        unsorted,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
        &mut unsorted,
    );
    return merge_sorted_edges(head, unsorted);
}
unsafe extern "C" fn active_edges_insert(mut sweep: *mut sweep_line_t) {
    let mut prev: *mut edge_t = 0 as *mut edge_t;
    let mut x: libc::c_int = 0;
    x = (*sweep).insert_x;
    prev = (*sweep).cursor;
    if (*prev).x > x {
        loop {
            prev = (*prev).prev;
            if !((*prev).x > x) {
                break;
            }
        }
    } else {
        while (*(*prev).next).x < x {
            prev = (*prev).next;
        }
    }
    let ref mut fresh45 = (*prev).next;
    *fresh45 = merge_unsorted_edges((*prev).next, (*sweep).insert);
    let ref mut fresh46 = (*sweep).cursor;
    *fresh46 = (*sweep).insert;
    let ref mut fresh47 = (*sweep).insert;
    *fresh47 = 0 as *mut edge_t;
    (*sweep).insert_x = 2147483647 as libc::c_int;
}
#[inline]
unsafe extern "C" fn active_edges_to_traps(mut sweep: *mut sweep_line_t) {
    let mut top: libc::c_int = (*sweep).current_y;
    let mut pos: *mut edge_t = 0 as *mut edge_t;
    if (*sweep).last_y == (*sweep).current_y {
        return;
    }
    if !((*sweep).insert).is_null() {
        active_edges_insert(sweep);
    }
    pos = (*sweep).head.next;
    if pos == &mut (*sweep).tail as *mut edge_t {
        return;
    }
    if (*sweep).fill_rule as libc::c_uint
        == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
    {
        loop {
            let mut left: *mut edge_t = 0 as *mut edge_t;
            let mut right: *mut edge_t = 0 as *mut edge_t;
            let mut winding: libc::c_int = 0;
            left = pos;
            winding = (*left).dir;
            right = (*left).next;
            while (*right).x == (*left).x {
                if !((*right).right).is_null() {
                    if ((*left).right).is_null() {} else {
                        __assert_fail(
                            b"left->right == NULL\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-bentley-ottmann-rectangular.c\0" as *const u8
                                as *const libc::c_char,
                            479 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 43],
                                &[libc::c_char; 43],
                            >(b"void active_edges_to_traps(sweep_line_t *)\0"))
                                .as_ptr(),
                        );
                    }
                    (*left).top = (*right).top;
                    let ref mut fresh48 = (*left).right;
                    *fresh48 = (*right).right;
                    let ref mut fresh49 = (*right).right;
                    *fresh49 = 0 as *mut edge_t;
                }
                winding += (*right).dir;
                right = (*right).next;
            }
            if winding == 0 as libc::c_int {
                if !((*left).right).is_null() {
                    edge_end_box(sweep, left, top);
                }
                pos = right;
            } else {
                loop {
                    if !((*right).right).is_null() {
                        edge_end_box(sweep, right, top);
                    }
                    winding += (*right).dir;
                    if winding == 0 as libc::c_int && (*right).x != (*(*right).next).x {
                        break;
                    }
                    right = (*right).next;
                }
                edge_start_or_continue_box(sweep, left, right, top);
                pos = (*right).next;
            }
            if !(pos != &mut (*sweep).tail as *mut edge_t) {
                break;
            }
        }
    } else {
        loop {
            let mut right_0: *mut edge_t = (*pos).next;
            let mut count: libc::c_int = 0 as libc::c_int;
            loop {
                if !((*right_0).right).is_null() {
                    edge_end_box(sweep, right_0, top);
                }
                count += 1;
                if count & 1 as libc::c_int != 0 && (*right_0).x != (*(*right_0).next).x
                {
                    break;
                }
                right_0 = (*right_0).next;
            }
            edge_start_or_continue_box(sweep, pos, right_0, top);
            pos = (*right_0).next;
            if !(pos != &mut (*sweep).tail as *mut edge_t) {
                break;
            }
        }
    }
    (*sweep).last_y = (*sweep).current_y;
}
#[inline]
unsafe extern "C" fn sweep_line_delete_edge(
    mut sweep: *mut sweep_line_t,
    mut edge: *mut edge_t,
) {
    if !((*edge).right).is_null() {
        let mut next: *mut edge_t = (*edge).next;
        if (*next).x == (*edge).x {
            (*next).top = (*edge).top;
            let ref mut fresh50 = (*next).right;
            *fresh50 = (*edge).right;
        } else {
            edge_end_box(sweep, edge, (*sweep).current_y);
        }
    }
    if (*sweep).cursor == edge {
        let ref mut fresh51 = (*sweep).cursor;
        *fresh51 = (*edge).prev;
    }
    let ref mut fresh52 = (*(*edge).prev).next;
    *fresh52 = (*edge).next;
    let ref mut fresh53 = (*(*edge).next).prev;
    *fresh53 = (*edge).prev;
}
#[inline]
unsafe extern "C" fn sweep_line_delete(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) -> cairo_bool_t {
    let mut update: cairo_bool_t = 0;
    update = 1 as libc::c_int;
    if (*sweep).fill_rule as libc::c_uint
        == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
        && (*(*rectangle).left.prev).dir == (*rectangle).left.dir
    {
        update = ((*rectangle).left.next != &mut (*rectangle).right as *mut edge_t)
            as libc::c_int;
    }
    sweep_line_delete_edge(sweep, &mut (*rectangle).left);
    sweep_line_delete_edge(sweep, &mut (*rectangle).right);
    rectangle_pop_stop(sweep);
    return update;
}
#[inline]
unsafe extern "C" fn sweep_line_insert(
    mut sweep: *mut sweep_line_t,
    mut rectangle: *mut rectangle_t,
) {
    if !((*sweep).insert).is_null() {
        let ref mut fresh54 = (*(*sweep).insert).prev;
        *fresh54 = &mut (*rectangle).right;
    }
    let ref mut fresh55 = (*rectangle).right.next;
    *fresh55 = (*sweep).insert;
    let ref mut fresh56 = (*rectangle).right.prev;
    *fresh56 = &mut (*rectangle).left;
    let ref mut fresh57 = (*rectangle).left.next;
    *fresh57 = &mut (*rectangle).right;
    let ref mut fresh58 = (*rectangle).left.prev;
    *fresh58 = 0 as *mut edge_t;
    let ref mut fresh59 = (*sweep).insert;
    *fresh59 = &mut (*rectangle).left;
    if (*rectangle).left.x < (*sweep).insert_x {
        (*sweep).insert_x = (*rectangle).left.x;
    }
    pqueue_push(sweep, rectangle);
}
unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_rectangular(
    mut rectangles: *mut *mut rectangle_t,
    mut num_rectangles: libc::c_int,
    mut fill_rule: cairo_fill_rule_t,
    mut do_traps: cairo_bool_t,
    mut container: *mut libc::c_void,
) -> cairo_status_t {
    let mut sweep_line: sweep_line_t = sweep_line_t {
        rectangles: 0 as *mut *mut rectangle_t,
        stop: 0 as *mut *mut rectangle_t,
        head: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            dir: 0,
        },
        tail: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            dir: 0,
        },
        insert: 0 as *mut edge_t,
        cursor: 0 as *mut edge_t,
        current_y: 0,
        last_y: 0,
        stop_size: 0,
        insert_x: 0,
        fill_rule: CAIRO_FILL_RULE_WINDING,
        do_traps: 0,
        container: 0 as *mut libc::c_void,
        unwind: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    let mut rectangle: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut update: cairo_bool_t = 0;
    sweep_line_init(
        &mut sweep_line,
        rectangles,
        num_rectangles,
        fill_rule,
        do_traps,
        container,
    );
    status = _setjmp((sweep_line.unwind).as_mut_ptr()) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    update = 0 as libc::c_int;
    rectangle = rectangle_pop_start(&mut sweep_line);
    loop {
        if (*rectangle).top != sweep_line.current_y {
            let mut stop: *mut rectangle_t = 0 as *mut rectangle_t;
            stop = rectangle_peek_stop(&mut sweep_line);
            while !stop.is_null() && (*stop).bottom < (*rectangle).top {
                if (*stop).bottom != sweep_line.current_y {
                    if update != 0 {
                        active_edges_to_traps(&mut sweep_line);
                        update = 0 as libc::c_int;
                    }
                    sweep_line.current_y = (*stop).bottom;
                }
                update |= sweep_line_delete(&mut sweep_line, stop);
                stop = rectangle_peek_stop(&mut sweep_line);
            }
            if update != 0 {
                active_edges_to_traps(&mut sweep_line);
                update = 0 as libc::c_int;
            }
            sweep_line.current_y = (*rectangle).top;
        }
        loop {
            sweep_line_insert(&mut sweep_line, rectangle);
            rectangle = rectangle_pop_start(&mut sweep_line);
            if !(!rectangle.is_null() && sweep_line.current_y == (*rectangle).top) {
                break;
            }
        }
        update = 1 as libc::c_int;
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
            if update != 0 {
                active_edges_to_traps(&mut sweep_line);
                update = 0 as libc::c_int;
            }
            sweep_line.current_y = (*rectangle).bottom;
        }
        update |= sweep_line_delete(&mut sweep_line, rectangle);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_rectangular_traps(
    mut traps: *mut cairo_traps_t,
    mut fill_rule: cairo_fill_rule_t,
) -> cairo_status_t {
    let mut stack_rectangles: [rectangle_t; 23] = [rectangle_t {
        left: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            dir: 0,
        },
        right: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            dir: 0,
        },
        top: 0,
        bottom: 0,
    }; 23];
    let mut stack_rectangles_ptrs: [*mut rectangle_t; 26] = [0 as *mut rectangle_t; 26];
    let mut rectangles: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut rectangles_ptrs: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if (*traps).is_rectangular() != 0 {} else {
        __assert_fail(
            b"traps->is_rectangular\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-bentley-ottmann-rectangular.c\0" as *const u8
                as *const libc::c_char,
            677 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"cairo_status_t _cairo_bentley_ottmann_tessellate_rectangular_traps(cairo_traps_t *, cairo_fill_rule_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*traps).num_traps <= 1 as libc::c_int {
        if (*traps).num_traps == 1 as libc::c_int {
            let mut trap: *mut cairo_trapezoid_t = (*traps).traps;
            if (*trap).left.p1.x > (*trap).right.p1.x {
                let mut tmp: cairo_line_t = (*trap).left;
                (*trap).left = (*trap).right;
                (*trap).right = tmp;
            }
        }
        return CAIRO_STATUS_SUCCESS;
    }
    rectangles = stack_rectangles.as_mut_ptr();
    rectangles_ptrs = stack_rectangles_ptrs.as_mut_ptr();
    if (*traps).num_traps
        > (::std::mem::size_of::<[rectangle_t; 23]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        rectangles = _cairo_malloc_ab_plus_c(
            (*traps).num_traps as size_t,
            (::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
                ),
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong),
        ) as *mut rectangle_t;
        if rectangles.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        rectangles_ptrs = rectangles.offset((*traps).num_traps as isize)
            as *mut *mut rectangle_t;
    }
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        if (*((*traps).traps).offset(i as isize)).left.p1.x
            < (*((*traps).traps).offset(i as isize)).right.p1.x
        {
            (*rectangles.offset(i as isize))
                .left
                .x = (*((*traps).traps).offset(i as isize)).left.p1.x;
            (*rectangles.offset(i as isize)).left.dir = 1 as libc::c_int;
            (*rectangles.offset(i as isize))
                .right
                .x = (*((*traps).traps).offset(i as isize)).right.p1.x;
            (*rectangles.offset(i as isize)).right.dir = -(1 as libc::c_int);
        } else {
            (*rectangles.offset(i as isize))
                .right
                .x = (*((*traps).traps).offset(i as isize)).left.p1.x;
            (*rectangles.offset(i as isize)).right.dir = 1 as libc::c_int;
            (*rectangles.offset(i as isize))
                .left
                .x = (*((*traps).traps).offset(i as isize)).right.p1.x;
            (*rectangles.offset(i as isize)).left.dir = -(1 as libc::c_int);
        }
        let ref mut fresh60 = (*rectangles.offset(i as isize)).left.right;
        *fresh60 = 0 as *mut edge_t;
        let ref mut fresh61 = (*rectangles.offset(i as isize)).right.right;
        *fresh61 = 0 as *mut edge_t;
        (*rectangles.offset(i as isize))
            .top = (*((*traps).traps).offset(i as isize)).top;
        (*rectangles.offset(i as isize))
            .bottom = (*((*traps).traps).offset(i as isize)).bottom;
        let ref mut fresh62 = *rectangles_ptrs.offset((i + 2 as libc::c_int) as isize);
        *fresh62 = &mut *rectangles.offset(i as isize) as *mut rectangle_t;
        i += 1;
    }
    _rectangle_sort(
        rectangles_ptrs.offset(2 as libc::c_int as isize),
        i as libc::c_uint,
    );
    _cairo_traps_clear(traps);
    status = _cairo_bentley_ottmann_tessellate_rectangular(
        rectangles_ptrs.offset(2 as libc::c_int as isize),
        i,
        fill_rule,
        1 as libc::c_int,
        traps as *mut libc::c_void,
    );
    (*traps).set_is_rectilinear(1 as libc::c_int as libc::c_uint);
    (*traps).set_is_rectangular(1 as libc::c_int as libc::c_uint);
    if rectangles != stack_rectangles.as_mut_ptr() {
        free(rectangles as *mut libc::c_void);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_boxes(
    mut in_0: *const cairo_boxes_t,
    mut fill_rule: cairo_fill_rule_t,
    mut out: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut stack_rectangles: [rectangle_t; 23] = [rectangle_t {
        left: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            dir: 0,
        },
        right: edge_t {
            next: 0 as *mut edge_t,
            prev: 0 as *mut edge_t,
            right: 0 as *mut edge_t,
            x: 0,
            top: 0,
            dir: 0,
        },
        top: 0,
        bottom: 0,
    }; 23];
    let mut stack_rectangles_ptrs: [*mut rectangle_t; 26] = [0 as *mut rectangle_t; 26];
    let mut rectangles: *mut rectangle_t = 0 as *mut rectangle_t;
    let mut rectangles_ptrs: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut stack_rectangles_chain: [*mut rectangle_t; 256] = [0
        as *mut rectangle_t; 256];
    let mut rectangles_chain: *mut *mut rectangle_t = 0 as *mut *mut rectangle_t;
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y_min: libc::c_int = 0;
    let mut y_max: libc::c_int = 0;
    if (*in_0).num_boxes == 0 as libc::c_int {
        _cairo_boxes_clear(out);
        return CAIRO_STATUS_SUCCESS;
    }
    if (*in_0).num_boxes == 1 as libc::c_int {
        if in_0 == out as *const cairo_boxes_t {
            let mut box_0: *mut cairo_box_t = &mut *((*in_0).chunks.base)
                .offset(0 as libc::c_int as isize) as *mut cairo_box_t;
            if (*box_0).p1.x > (*box_0).p2.x {
                let mut tmp: cairo_fixed_t = (*box_0).p1.x;
                (*box_0).p1.x = (*box_0).p2.x;
                (*box_0).p2.x = tmp;
            }
        } else {
            let mut box_1: cairo_box_t = *((*in_0).chunks.base)
                .offset(0 as libc::c_int as isize);
            if box_1.p1.x > box_1.p2.x {
                let mut tmp_0: cairo_fixed_t = box_1.p1.x;
                box_1.p1.x = box_1.p2.x;
                box_1.p2.x = tmp_0;
            }
            _cairo_boxes_clear(out);
            status = _cairo_boxes_add(out, CAIRO_ANTIALIAS_DEFAULT, &mut box_1);
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-bentley-ottmann-rectangular.c\0" as *const u8
                        as *const libc::c_char,
                    786 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 114],
                        &[libc::c_char; 114],
                    >(
                        b"cairo_status_t _cairo_bentley_ottmann_tessellate_boxes(const cairo_boxes_t *, cairo_fill_rule_t, cairo_boxes_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        return CAIRO_STATUS_SUCCESS;
    }
    y_min = 2147483647 as libc::c_int;
    y_max = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    chunk = &(*in_0).chunks;
    while !chunk.is_null() {
        let mut box_2: *const cairo_box_t = (*chunk).base;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            if (*box_2.offset(i as isize)).p1.y < y_min {
                y_min = (*box_2.offset(i as isize)).p1.y;
            }
            if (*box_2.offset(i as isize)).p1.y > y_max {
                y_max = (*box_2.offset(i as isize)).p1.y;
            }
            i += 1;
        }
        chunk = (*chunk).next;
    }
    y_min = _cairo_fixed_integer_floor(y_min);
    y_max = _cairo_fixed_integer_floor(y_max) + 1 as libc::c_int;
    y_max -= y_min;
    if y_max < (*in_0).num_boxes {
        rectangles_chain = stack_rectangles_chain.as_mut_ptr();
        if y_max
            > (::std::mem::size_of::<[*mut rectangle_t; 256]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong)
                as libc::c_int
        {
            rectangles_chain = _cairo_malloc_ab(
                y_max as size_t,
                ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
            ) as *mut *mut rectangle_t;
            if rectangles_chain.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
        memset(
            rectangles_chain as *mut libc::c_void,
            0 as libc::c_int,
            (y_max as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong),
        );
    }
    rectangles = stack_rectangles.as_mut_ptr();
    rectangles_ptrs = stack_rectangles_ptrs.as_mut_ptr();
    if (*in_0).num_boxes
        > (::std::mem::size_of::<[rectangle_t; 23]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        rectangles = _cairo_malloc_ab_plus_c(
            (*in_0).num_boxes as size_t,
            (::std::mem::size_of::<rectangle_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong,
                ),
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut rectangle_t>() as libc::c_ulong),
        ) as *mut rectangle_t;
        if rectangles.is_null() {
            if rectangles_chain != stack_rectangles_chain.as_mut_ptr() {
                free(rectangles_chain as *mut libc::c_void);
            }
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        rectangles_ptrs = rectangles.offset((*in_0).num_boxes as isize)
            as *mut *mut rectangle_t;
    }
    j = 0 as libc::c_int;
    chunk = &(*in_0).chunks;
    while !chunk.is_null() {
        let mut box_3: *const cairo_box_t = (*chunk).base;
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let mut h: libc::c_int = 0;
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
            let ref mut fresh63 = (*rectangles.offset(j as isize)).left.right;
            *fresh63 = 0 as *mut edge_t;
            let ref mut fresh64 = (*rectangles.offset(j as isize)).right.right;
            *fresh64 = 0 as *mut edge_t;
            (*rectangles.offset(j as isize)).top = (*box_3.offset(i as isize)).p1.y;
            (*rectangles.offset(j as isize)).bottom = (*box_3.offset(i as isize)).p2.y;
            if !rectangles_chain.is_null() {
                h = _cairo_fixed_integer_floor((*box_3.offset(i as isize)).p1.y) - y_min;
                let ref mut fresh65 = (*rectangles.offset(j as isize)).left.next;
                *fresh65 = *rectangles_chain.offset(h as isize) as *mut edge_t;
                let ref mut fresh66 = *rectangles_chain.offset(h as isize);
                *fresh66 = &mut *rectangles.offset(j as isize) as *mut rectangle_t;
            } else {
                let ref mut fresh67 = *rectangles_ptrs
                    .offset((j + 2 as libc::c_int) as isize);
                *fresh67 = &mut *rectangles.offset(j as isize) as *mut rectangle_t;
            }
            j += 1;
            i += 1;
        }
        chunk = (*chunk).next;
    }
    if !rectangles_chain.is_null() {
        j = 2 as libc::c_int;
        y_min = 0 as libc::c_int;
        while y_min < y_max {
            let mut r: *mut rectangle_t = 0 as *mut rectangle_t;
            let mut start: libc::c_int = j;
            r = *rectangles_chain.offset(y_min as isize);
            while !r.is_null() {
                let fresh68 = j;
                j = j + 1;
                let ref mut fresh69 = *rectangles_ptrs.offset(fresh68 as isize);
                *fresh69 = r;
                r = (*r).left.next as *mut rectangle_t;
            }
            if j > start + 1 as libc::c_int {
                _rectangle_sort(
                    rectangles_ptrs.offset(start as isize),
                    (j - start) as libc::c_uint,
                );
            }
            y_min += 1;
        }
        if rectangles_chain != stack_rectangles_chain.as_mut_ptr() {
            free(rectangles_chain as *mut libc::c_void);
        }
        j -= 2 as libc::c_int;
    } else {
        _rectangle_sort(
            rectangles_ptrs.offset(2 as libc::c_int as isize),
            j as libc::c_uint,
        );
    }
    _cairo_boxes_clear(out);
    status = _cairo_bentley_ottmann_tessellate_rectangular(
        rectangles_ptrs.offset(2 as libc::c_int as isize),
        j,
        fill_rule,
        0 as libc::c_int,
        out as *mut libc::c_void,
    );
    if rectangles != stack_rectangles.as_mut_ptr() {
        free(rectangles as *mut libc::c_void);
    }
    return status;
}
