use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn _cairo_traps_clear(traps: *mut cairo_traps_t);
    fn _cairo_traps_add_trap(
        traps: *mut cairo_traps_t,
        top: cairo_fixed_t,
        bottom: cairo_fixed_t,
        left: *const cairo_line_t,
        right: *const cairo_line_t,
    );
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
pub type cairo_bo_event_t = _cairo_bo_event;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_event {
    pub type_0: cairo_bo_event_type_t,
    pub point: cairo_point_t,
    pub edge: *mut cairo_bo_edge_t,
}
pub type cairo_bo_edge_t = _cairo_bo_edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_edge {
    pub edge: cairo_edge_t,
    pub prev: *mut cairo_bo_edge_t,
    pub next: *mut cairo_bo_edge_t,
    pub deferred_trap: cairo_bo_trap_t,
}
pub type cairo_bo_trap_t = _cairo_bo_trap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_trap {
    pub right: *mut cairo_bo_edge_t,
    pub top: int32_t,
}
pub type cairo_bo_event_type_t = libc::c_uint;
pub const CAIRO_BO_EVENT_TYPE_STOP: cairo_bo_event_type_t = 1;
pub const CAIRO_BO_EVENT_TYPE_START: cairo_bo_event_type_t = 0;
pub type cairo_bo_sweep_line_t = _cairo_bo_sweep_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_bo_sweep_line {
    pub events: *mut *mut cairo_bo_event_t,
    pub head: *mut cairo_bo_edge_t,
    pub stopped: *mut cairo_bo_edge_t,
    pub current_y: int32_t,
    pub current_edge: *mut cairo_bo_edge_t,
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
unsafe extern "C" fn _cairo_point_compare(
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = (*a).y - (*b).y;
    if cmp != 0 {
        return cmp;
    }
    return (*a).x - (*b).x;
}
#[inline]
unsafe extern "C" fn _cairo_bo_edge_compare(
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = (*a).edge.line.p1.x - (*b).edge.line.p1.x;
    if cmp != 0 {
        return cmp;
    }
    return (*b).edge.bottom - (*a).edge.bottom;
}
#[inline]
unsafe extern "C" fn cairo_bo_event_compare(
    mut a: *const cairo_bo_event_t,
    mut b: *const cairo_bo_event_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    cmp = _cairo_point_compare(&(*a).point, &(*b).point);
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
unsafe extern "C" fn _cairo_bo_event_dequeue(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
) -> *mut cairo_bo_event_t {
    let ref mut fresh4 = (*sweep_line).events;
    let fresh5 = *fresh4;
    *fresh4 = (*fresh4).offset(1);
    return *fresh5;
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
                let ref mut fresh6 = *base.offset(i as isize);
                *fresh6 = *base.offset(j as isize);
                let ref mut fresh7 = *base.offset(j as isize);
                *fresh7 = tmp;
                swapped = 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        if !(swapped != 0) {
            break;
        }
    };
}
unsafe extern "C" fn _cairo_bo_sweep_line_init(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut events: *mut *mut cairo_bo_event_t,
    mut num_events: libc::c_int,
) {
    _cairo_bo_event_queue_sort(events, num_events as libc::c_uint);
    let ref mut fresh8 = *events.offset(num_events as isize);
    *fresh8 = 0 as *mut cairo_bo_event_t;
    let ref mut fresh9 = (*sweep_line).events;
    *fresh9 = events;
    let ref mut fresh10 = (*sweep_line).head;
    *fresh10 = 0 as *mut cairo_bo_edge_t;
    (*sweep_line).current_y = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh11 = (*sweep_line).current_edge;
    *fresh11 = 0 as *mut cairo_bo_edge_t;
}
unsafe extern "C" fn _cairo_bo_sweep_line_insert(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut edge: *mut cairo_bo_edge_t,
) {
    if !((*sweep_line).current_edge).is_null() {
        let mut prev: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
        let mut next: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
        let mut cmp: libc::c_int = 0;
        cmp = _cairo_bo_edge_compare((*sweep_line).current_edge, edge);
        if cmp < 0 as libc::c_int {
            prev = (*sweep_line).current_edge;
            next = (*prev).next;
            while !next.is_null()
                && _cairo_bo_edge_compare(next, edge) < 0 as libc::c_int
            {
                prev = next;
                next = (*prev).next;
            }
            let ref mut fresh12 = (*prev).next;
            *fresh12 = edge;
            let ref mut fresh13 = (*edge).prev;
            *fresh13 = prev;
            let ref mut fresh14 = (*edge).next;
            *fresh14 = next;
            if !next.is_null() {
                let ref mut fresh15 = (*next).prev;
                *fresh15 = edge;
            }
        } else if cmp > 0 as libc::c_int {
            next = (*sweep_line).current_edge;
            prev = (*next).prev;
            while !prev.is_null()
                && _cairo_bo_edge_compare(prev, edge) > 0 as libc::c_int
            {
                next = prev;
                prev = (*next).prev;
            }
            let ref mut fresh16 = (*next).prev;
            *fresh16 = edge;
            let ref mut fresh17 = (*edge).next;
            *fresh17 = next;
            let ref mut fresh18 = (*edge).prev;
            *fresh18 = prev;
            if !prev.is_null() {
                let ref mut fresh19 = (*prev).next;
                *fresh19 = edge;
            } else {
                let ref mut fresh20 = (*sweep_line).head;
                *fresh20 = edge;
            }
        } else {
            prev = (*sweep_line).current_edge;
            let ref mut fresh21 = (*edge).prev;
            *fresh21 = prev;
            let ref mut fresh22 = (*edge).next;
            *fresh22 = (*prev).next;
            if !((*prev).next).is_null() {
                let ref mut fresh23 = (*(*prev).next).prev;
                *fresh23 = edge;
            }
            let ref mut fresh24 = (*prev).next;
            *fresh24 = edge;
        }
    } else {
        let ref mut fresh25 = (*sweep_line).head;
        *fresh25 = edge;
    }
    let ref mut fresh26 = (*sweep_line).current_edge;
    *fresh26 = edge;
}
unsafe extern "C" fn _cairo_bo_sweep_line_delete(
    mut sweep_line: *mut cairo_bo_sweep_line_t,
    mut edge: *mut cairo_bo_edge_t,
) {
    if !((*edge).prev).is_null() {
        let ref mut fresh27 = (*(*edge).prev).next;
        *fresh27 = (*edge).next;
    } else {
        let ref mut fresh28 = (*sweep_line).head;
        *fresh28 = (*edge).next;
    }
    if !((*edge).next).is_null() {
        let ref mut fresh29 = (*(*edge).next).prev;
        *fresh29 = (*edge).prev;
    }
    if (*sweep_line).current_edge == edge {
        let ref mut fresh30 = (*sweep_line).current_edge;
        *fresh30 = if !((*edge).prev).is_null() { (*edge).prev } else { (*edge).next };
    }
}
#[inline]
unsafe extern "C" fn edges_collinear(
    mut a: *const cairo_bo_edge_t,
    mut b: *const cairo_bo_edge_t,
) -> cairo_bool_t {
    return ((*a).edge.line.p1.x == (*b).edge.line.p1.x) as libc::c_int;
}
unsafe extern "C" fn _cairo_bo_edge_end_trap(
    mut left: *mut cairo_bo_edge_t,
    mut bot: int32_t,
    mut do_traps: cairo_bool_t,
    mut container: *mut libc::c_void,
) -> cairo_status_t {
    let mut trap: *mut cairo_bo_trap_t = &mut (*left).deferred_trap;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*trap).top < bot {
        if do_traps != 0 {
            _cairo_traps_add_trap(
                container as *mut cairo_traps_t,
                (*trap).top,
                bot,
                &mut (*left).edge.line,
                &mut (*(*trap).right).edge.line,
            );
            status = (*(container as *mut cairo_traps_t)).status;
        } else {
            let mut box_0: cairo_box_t = cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            box_0.p1.x = (*left).edge.line.p1.x;
            box_0.p1.y = (*trap).top;
            box_0.p2.x = (*(*trap).right).edge.line.p1.x;
            box_0.p2.y = bot;
            status = _cairo_boxes_add(
                container as *mut cairo_boxes_t,
                CAIRO_ANTIALIAS_DEFAULT,
                &mut box_0,
            );
        }
    }
    let ref mut fresh31 = (*trap).right;
    *fresh31 = 0 as *mut cairo_bo_edge_t;
    return status;
}
#[inline]
unsafe extern "C" fn _cairo_bo_edge_start_or_continue_trap(
    mut left: *mut cairo_bo_edge_t,
    mut right: *mut cairo_bo_edge_t,
    mut top: libc::c_int,
    mut do_traps: cairo_bool_t,
    mut container: *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*left).deferred_trap.right == right {
        return CAIRO_STATUS_SUCCESS;
    }
    if !((*left).deferred_trap.right).is_null() {
        if !right.is_null() && edges_collinear((*left).deferred_trap.right, right) != 0 {
            let ref mut fresh32 = (*left).deferred_trap.right;
            *fresh32 = right;
            return CAIRO_STATUS_SUCCESS;
        }
        status = _cairo_bo_edge_end_trap(left, top, do_traps, container);
        if status as u64 != 0 {
            return status;
        }
    }
    if !right.is_null() && edges_collinear(left, right) == 0 {
        (*left).deferred_trap.top = top;
        let ref mut fresh33 = (*left).deferred_trap.right;
        *fresh33 = right;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn _active_edges_to_traps(
    mut left: *mut cairo_bo_edge_t,
    mut top: int32_t,
    mut fill_rule: cairo_fill_rule_t,
    mut do_traps: cairo_bool_t,
    mut container: *mut libc::c_void,
) -> cairo_status_t {
    let mut right: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if fill_rule as libc::c_uint
        == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
    {
        while !left.is_null() {
            let mut in_out: libc::c_int = 0;
            in_out = (*left).edge.dir;
            right = (*left).next;
            if ((*left).deferred_trap.right).is_null() {
                while !right.is_null() && ((*right).deferred_trap.right).is_null() {
                    right = (*right).next;
                }
                if !right.is_null() && edges_collinear(left, right) != 0 {
                    (*left).deferred_trap = (*right).deferred_trap;
                    let ref mut fresh34 = (*right).deferred_trap.right;
                    *fresh34 = 0 as *mut cairo_bo_edge_t;
                }
            }
            right = (*left).next;
            while !right.is_null() {
                if !((*right).deferred_trap.right).is_null() {
                    status = _cairo_bo_edge_end_trap(right, top, do_traps, container);
                    if status as u64 != 0 {
                        return status;
                    }
                }
                in_out += (*right).edge.dir;
                if in_out == 0 as libc::c_int {
                    if ((*right).next).is_null()
                        || edges_collinear(right, (*right).next) == 0
                    {
                        break;
                    }
                }
                right = (*right).next;
            }
            status = _cairo_bo_edge_start_or_continue_trap(
                left,
                right,
                top,
                do_traps,
                container,
            );
            if status as u64 != 0 {
                return status;
            }
            left = right;
            if !left.is_null() {
                left = (*left).next;
            }
        }
    } else {
        while !left.is_null() {
            let mut in_out_0: libc::c_int = 0 as libc::c_int;
            right = (*left).next;
            while !right.is_null() {
                if !((*right).deferred_trap.right).is_null() {
                    status = _cairo_bo_edge_end_trap(right, top, do_traps, container);
                    if status as u64 != 0 {
                        return status;
                    }
                }
                let fresh35 = in_out_0;
                in_out_0 = in_out_0 + 1;
                if fresh35 & 1 as libc::c_int == 0 as libc::c_int {
                    let mut next: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
                    let mut skip: cairo_bool_t = 0 as libc::c_int;
                    next = (*right).next;
                    if !next.is_null() {
                        skip = edges_collinear(right, next);
                    }
                    if skip == 0 {
                        break;
                    }
                }
                right = (*right).next;
            }
            status = _cairo_bo_edge_start_or_continue_trap(
                left,
                right,
                top,
                do_traps,
                container,
            );
            if status as u64 != 0 {
                return status;
            }
            left = right;
            if !left.is_null() {
                left = (*left).next;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_rectilinear(
    mut start_events: *mut *mut cairo_bo_event_t,
    mut num_events: libc::c_int,
    mut fill_rule: cairo_fill_rule_t,
    mut do_traps: cairo_bool_t,
    mut container: *mut libc::c_void,
) -> cairo_status_t {
    let mut sweep_line: cairo_bo_sweep_line_t = cairo_bo_sweep_line_t {
        events: 0 as *mut *mut cairo_bo_event_t,
        head: 0 as *mut cairo_bo_edge_t,
        stopped: 0 as *mut cairo_bo_edge_t,
        current_y: 0,
        current_edge: 0 as *mut cairo_bo_edge_t,
    };
    let mut event: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_bo_sweep_line_init(&mut sweep_line, start_events, num_events);
    loop {
        event = _cairo_bo_event_dequeue(&mut sweep_line);
        if event.is_null() {
            break;
        }
        if (*event).point.y != sweep_line.current_y {
            status = _active_edges_to_traps(
                sweep_line.head,
                sweep_line.current_y,
                fill_rule,
                do_traps,
                container,
            );
            if status as u64 != 0 {
                return status;
            }
            sweep_line.current_y = (*event).point.y;
        }
        match (*event).type_0 as libc::c_uint {
            0 => {
                _cairo_bo_sweep_line_insert(&mut sweep_line, (*event).edge);
            }
            1 => {
                _cairo_bo_sweep_line_delete(&mut sweep_line, (*event).edge);
                if !((*(*event).edge).deferred_trap.right).is_null() {
                    status = _cairo_bo_edge_end_trap(
                        (*event).edge,
                        sweep_line.current_y,
                        do_traps,
                        container,
                    );
                    if status as u64 != 0 {
                        return status;
                    }
                }
            }
            _ => {}
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_rectilinear_polygon_to_boxes(
    mut polygon: *const cairo_polygon_t,
    mut fill_rule: cairo_fill_rule_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut stack_events: [cairo_bo_event_t; 85] = [cairo_bo_event_t {
        type_0: CAIRO_BO_EVENT_TYPE_START,
        point: cairo_point_t { x: 0, y: 0 },
        edge: 0 as *mut cairo_bo_edge_t,
    }; 85];
    let mut events: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    let mut stack_event_ptrs: [*mut cairo_bo_event_t; 86] = [0
        as *mut cairo_bo_event_t; 86];
    let mut event_ptrs: *mut *mut cairo_bo_event_t = 0 as *mut *mut cairo_bo_event_t;
    let mut stack_edges: [cairo_bo_edge_t; 85] = [cairo_bo_edge_t {
        edge: cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        },
        prev: 0 as *mut cairo_bo_edge_t,
        next: 0 as *mut cairo_bo_edge_t,
        deferred_trap: cairo_bo_trap_t {
            right: 0 as *mut cairo_bo_edge_t,
            top: 0,
        },
    }; 85];
    let mut edges: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut num_events: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*polygon).num_edges == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    num_events = 2 as libc::c_int * (*polygon).num_edges;
    events = stack_events.as_mut_ptr();
    event_ptrs = stack_event_ptrs.as_mut_ptr();
    edges = stack_edges.as_mut_ptr();
    if num_events
        > (::std::mem::size_of::<[cairo_bo_event_t; 85]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_bo_event_t>() as libc::c_ulong)
            as libc::c_int
    {
        events = _cairo_malloc_ab_plus_c(
            num_events as size_t,
            (::std::mem::size_of::<cairo_bo_event_t>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<cairo_bo_edge_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
                ),
            ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
        ) as *mut cairo_bo_event_t;
        if events.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        event_ptrs = events.offset(num_events as isize) as *mut *mut cairo_bo_event_t;
        edges = event_ptrs.offset(num_events as isize).offset(1 as libc::c_int as isize)
            as *mut cairo_bo_edge_t;
    }
    j = 0 as libc::c_int;
    i = j;
    while i < (*polygon).num_edges {
        (*edges.offset(i as isize)).edge = *((*polygon).edges).offset(i as isize);
        let ref mut fresh36 = (*edges.offset(i as isize)).deferred_trap.right;
        *fresh36 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh37 = (*edges.offset(i as isize)).prev;
        *fresh37 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh38 = (*edges.offset(i as isize)).next;
        *fresh38 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh39 = *event_ptrs.offset(j as isize);
        *fresh39 = &mut *events.offset(j as isize) as *mut cairo_bo_event_t;
        (*events.offset(j as isize)).type_0 = CAIRO_BO_EVENT_TYPE_START;
        (*events.offset(j as isize))
            .point
            .y = (*((*polygon).edges).offset(i as isize)).top;
        (*events.offset(j as isize))
            .point
            .x = (*((*polygon).edges).offset(i as isize)).line.p1.x;
        let ref mut fresh40 = (*events.offset(j as isize)).edge;
        *fresh40 = &mut *edges.offset(i as isize) as *mut cairo_bo_edge_t;
        j += 1;
        let ref mut fresh41 = *event_ptrs.offset(j as isize);
        *fresh41 = &mut *events.offset(j as isize) as *mut cairo_bo_event_t;
        (*events.offset(j as isize)).type_0 = CAIRO_BO_EVENT_TYPE_STOP;
        (*events.offset(j as isize))
            .point
            .y = (*((*polygon).edges).offset(i as isize)).bottom;
        (*events.offset(j as isize))
            .point
            .x = (*((*polygon).edges).offset(i as isize)).line.p1.x;
        let ref mut fresh42 = (*events.offset(j as isize)).edge;
        *fresh42 = &mut *edges.offset(i as isize) as *mut cairo_bo_edge_t;
        j += 1;
        i += 1;
    }
    status = _cairo_bentley_ottmann_tessellate_rectilinear(
        event_ptrs,
        j,
        fill_rule,
        0 as libc::c_int,
        boxes as *mut libc::c_void,
    );
    if events != stack_events.as_mut_ptr() {
        free(events as *mut libc::c_void);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_bentley_ottmann_tessellate_rectilinear_traps(
    mut traps: *mut cairo_traps_t,
    mut fill_rule: cairo_fill_rule_t,
) -> cairo_status_t {
    let mut stack_events: [cairo_bo_event_t; 85] = [cairo_bo_event_t {
        type_0: CAIRO_BO_EVENT_TYPE_START,
        point: cairo_point_t { x: 0, y: 0 },
        edge: 0 as *mut cairo_bo_edge_t,
    }; 85];
    let mut events: *mut cairo_bo_event_t = 0 as *mut cairo_bo_event_t;
    let mut stack_event_ptrs: [*mut cairo_bo_event_t; 86] = [0
        as *mut cairo_bo_event_t; 86];
    let mut event_ptrs: *mut *mut cairo_bo_event_t = 0 as *mut *mut cairo_bo_event_t;
    let mut stack_edges: [cairo_bo_edge_t; 85] = [cairo_bo_edge_t {
        edge: cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        },
        prev: 0 as *mut cairo_bo_edge_t,
        next: 0 as *mut cairo_bo_edge_t,
        deferred_trap: cairo_bo_trap_t {
            right: 0 as *mut cairo_bo_edge_t,
            top: 0,
        },
    }; 85];
    let mut edges: *mut cairo_bo_edge_t = 0 as *mut cairo_bo_edge_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*traps).num_traps == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*traps).is_rectilinear() != 0 {} else {
        __assert_fail(
            b"traps->is_rectilinear\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-bentley-ottmann-rectilinear.c\0" as *const u8
                as *const libc::c_char,
            522 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"cairo_status_t _cairo_bentley_ottmann_tessellate_rectilinear_traps(cairo_traps_t *, cairo_fill_rule_t)\0",
            ))
                .as_ptr(),
        );
    }
    i = 4 as libc::c_int * (*traps).num_traps;
    events = stack_events.as_mut_ptr();
    event_ptrs = stack_event_ptrs.as_mut_ptr();
    edges = stack_edges.as_mut_ptr();
    if i
        > (::std::mem::size_of::<[cairo_bo_event_t; 85]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_bo_event_t>() as libc::c_ulong)
            as libc::c_int
    {
        events = _cairo_malloc_ab_plus_c(
            i as size_t,
            (::std::mem::size_of::<cairo_bo_event_t>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<cairo_bo_edge_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
                ),
            ::std::mem::size_of::<*mut cairo_bo_event_t>() as libc::c_ulong,
        ) as *mut cairo_bo_event_t;
        if events.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        event_ptrs = events.offset(i as isize) as *mut *mut cairo_bo_event_t;
        edges = event_ptrs.offset(i as isize).offset(1 as libc::c_int as isize)
            as *mut cairo_bo_edge_t;
    }
    k = 0 as libc::c_int;
    j = k;
    i = j;
    while i < (*traps).num_traps {
        (*edges.offset(k as isize))
            .edge
            .top = (*((*traps).traps).offset(i as isize)).top;
        (*edges.offset(k as isize))
            .edge
            .bottom = (*((*traps).traps).offset(i as isize)).bottom;
        (*edges.offset(k as isize))
            .edge
            .line = (*((*traps).traps).offset(i as isize)).left;
        (*edges.offset(k as isize)).edge.dir = 1 as libc::c_int;
        let ref mut fresh43 = (*edges.offset(k as isize)).deferred_trap.right;
        *fresh43 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh44 = (*edges.offset(k as isize)).prev;
        *fresh44 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh45 = (*edges.offset(k as isize)).next;
        *fresh45 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh46 = *event_ptrs.offset(j as isize);
        *fresh46 = &mut *events.offset(j as isize) as *mut cairo_bo_event_t;
        (*events.offset(j as isize)).type_0 = CAIRO_BO_EVENT_TYPE_START;
        (*events.offset(j as isize))
            .point
            .y = (*((*traps).traps).offset(i as isize)).top;
        (*events.offset(j as isize))
            .point
            .x = (*((*traps).traps).offset(i as isize)).left.p1.x;
        let ref mut fresh47 = (*events.offset(j as isize)).edge;
        *fresh47 = &mut *edges.offset(k as isize) as *mut cairo_bo_edge_t;
        j += 1;
        let ref mut fresh48 = *event_ptrs.offset(j as isize);
        *fresh48 = &mut *events.offset(j as isize) as *mut cairo_bo_event_t;
        (*events.offset(j as isize)).type_0 = CAIRO_BO_EVENT_TYPE_STOP;
        (*events.offset(j as isize))
            .point
            .y = (*((*traps).traps).offset(i as isize)).bottom;
        (*events.offset(j as isize))
            .point
            .x = (*((*traps).traps).offset(i as isize)).left.p1.x;
        let ref mut fresh49 = (*events.offset(j as isize)).edge;
        *fresh49 = &mut *edges.offset(k as isize) as *mut cairo_bo_edge_t;
        j += 1;
        k += 1;
        (*edges.offset(k as isize))
            .edge
            .top = (*((*traps).traps).offset(i as isize)).top;
        (*edges.offset(k as isize))
            .edge
            .bottom = (*((*traps).traps).offset(i as isize)).bottom;
        (*edges.offset(k as isize))
            .edge
            .line = (*((*traps).traps).offset(i as isize)).right;
        (*edges.offset(k as isize)).edge.dir = -(1 as libc::c_int);
        let ref mut fresh50 = (*edges.offset(k as isize)).deferred_trap.right;
        *fresh50 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh51 = (*edges.offset(k as isize)).prev;
        *fresh51 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh52 = (*edges.offset(k as isize)).next;
        *fresh52 = 0 as *mut cairo_bo_edge_t;
        let ref mut fresh53 = *event_ptrs.offset(j as isize);
        *fresh53 = &mut *events.offset(j as isize) as *mut cairo_bo_event_t;
        (*events.offset(j as isize)).type_0 = CAIRO_BO_EVENT_TYPE_START;
        (*events.offset(j as isize))
            .point
            .y = (*((*traps).traps).offset(i as isize)).top;
        (*events.offset(j as isize))
            .point
            .x = (*((*traps).traps).offset(i as isize)).right.p1.x;
        let ref mut fresh54 = (*events.offset(j as isize)).edge;
        *fresh54 = &mut *edges.offset(k as isize) as *mut cairo_bo_edge_t;
        j += 1;
        let ref mut fresh55 = *event_ptrs.offset(j as isize);
        *fresh55 = &mut *events.offset(j as isize) as *mut cairo_bo_event_t;
        (*events.offset(j as isize)).type_0 = CAIRO_BO_EVENT_TYPE_STOP;
        (*events.offset(j as isize))
            .point
            .y = (*((*traps).traps).offset(i as isize)).bottom;
        (*events.offset(j as isize))
            .point
            .x = (*((*traps).traps).offset(i as isize)).right.p1.x;
        let ref mut fresh56 = (*events.offset(j as isize)).edge;
        *fresh56 = &mut *edges.offset(k as isize) as *mut cairo_bo_edge_t;
        j += 1;
        k += 1;
        i += 1;
    }
    _cairo_traps_clear(traps);
    status = _cairo_bentley_ottmann_tessellate_rectilinear(
        event_ptrs,
        j,
        fill_rule,
        1 as libc::c_int,
        traps as *mut libc::c_void,
    );
    (*traps).set_is_rectilinear(1 as libc::c_int as libc::c_uint);
    if events != stack_events.as_mut_ptr() {
        free(events as *mut libc::c_void);
    }
    return status;
}
