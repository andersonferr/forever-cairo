use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
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
pub type cairo_mono_scan_converter_t = _cairo_mono_scan_converter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mono_scan_converter {
    pub base: cairo_scan_converter_t,
    pub converter: [mono_scan_converter; 1],
    pub fill_rule: cairo_fill_rule_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mono_scan_converter {
    pub polygon: [polygon; 1],
    pub head: edge,
    pub tail: edge,
    pub is_vertical: libc::c_int,
    pub spans: *mut cairo_half_open_span_t,
    pub spans_embedded: [cairo_half_open_span_t; 64],
    pub num_spans: libc::c_int,
    pub xmin: int32_t,
    pub xmax: int32_t,
    pub ymin: int32_t,
    pub ymax: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge {
    pub next: *mut edge,
    pub prev: *mut edge,
    pub height_left: int32_t,
    pub dir: int32_t,
    pub vertical: int32_t,
    pub dy: int32_t,
    pub x: quorem,
    pub dxdy: quorem,
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
    pub ymin: int32_t,
    pub ymax: int32_t,
    pub num_edges: libc::c_int,
    pub edges: *mut edge,
    pub y_buckets: *mut *mut edge,
    pub y_buckets_embedded: [*mut edge; 64],
    pub edges_embedded: [edge; 32],
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
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
unsafe extern "C" fn polygon_init(
    mut polygon: *mut polygon,
    mut ymin: libc::c_int,
    mut ymax: libc::c_int,
) -> cairo_status_t {
    let mut h: libc::c_uint = (ymax - ymin + 1 as libc::c_int) as libc::c_uint;
    let ref mut fresh2 = (*polygon).y_buckets;
    *fresh2 = ((*polygon).y_buckets_embedded).as_mut_ptr();
    if h
        > (::std::mem::size_of::<[*mut edge; 64]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut edge>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        let ref mut fresh3 = (*polygon).y_buckets;
        *fresh3 = _cairo_malloc_ab(
            h as size_t,
            ::std::mem::size_of::<*mut edge>() as libc::c_ulong,
        ) as *mut *mut edge;
        if ((*polygon).y_buckets).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    memset(
        (*polygon).y_buckets as *mut libc::c_void,
        0 as libc::c_int,
        (h as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut edge>() as libc::c_ulong),
    );
    let ref mut fresh4 = *((*polygon).y_buckets)
        .offset(h.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    *fresh4 = -(1 as libc::c_int) as *mut libc::c_void as *mut edge;
    (*polygon).ymin = ymin;
    (*polygon).ymax = ymax;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn polygon_fini(mut polygon: *mut polygon) {
    if (*polygon).y_buckets != ((*polygon).y_buckets_embedded).as_mut_ptr() {
        free((*polygon).y_buckets as *mut libc::c_void);
    }
    if (*polygon).edges != ((*polygon).edges_embedded).as_mut_ptr() {
        free((*polygon).edges as *mut libc::c_void);
    }
}
unsafe extern "C" fn _polygon_insert_edge_into_its_y_bucket(
    mut polygon: *mut polygon,
    mut e: *mut edge,
    mut y: libc::c_int,
) {
    let mut ptail: *mut *mut edge = &mut *((*polygon).y_buckets)
        .offset((y - (*polygon).ymin) as isize) as *mut *mut edge;
    if !(*ptail).is_null() {
        let ref mut fresh5 = (**ptail).prev;
        *fresh5 = e;
    }
    let ref mut fresh6 = (*e).next;
    *fresh6 = *ptail;
    let ref mut fresh7 = (*e).prev;
    *fresh7 = 0 as *mut edge;
    *ptail = e;
}
#[inline]
unsafe extern "C" fn polygon_add_edge(
    mut polygon: *mut polygon,
    mut edge: *const cairo_edge_t,
) {
    let mut e: *mut edge = 0 as *mut edge;
    let mut dx: cairo_fixed_t = 0;
    let mut dy: cairo_fixed_t = 0;
    let mut y: libc::c_int = 0;
    let mut ytop: libc::c_int = 0;
    let mut ybot: libc::c_int = 0;
    let mut ymin: libc::c_int = (*polygon).ymin;
    let mut ymax: libc::c_int = (*polygon).ymax;
    y = _cairo_fixed_integer_round_down((*edge).top);
    ytop = if y > ymin { y } else { ymin };
    y = _cairo_fixed_integer_round_down((*edge).bottom);
    ybot = if y < ymax { y } else { ymax };
    if ybot <= ytop {
        return;
    }
    let ref mut fresh8 = (*polygon).num_edges;
    let fresh9 = *fresh8;
    *fresh8 = *fresh8 + 1;
    e = ((*polygon).edges).offset(fresh9 as isize);
    (*e).height_left = ybot - ytop;
    (*e).dir = (*edge).dir;
    dx = (*edge).line.p2.x - (*edge).line.p1.x;
    dy = (*edge).line.p2.y - (*edge).line.p1.y;
    if dx == 0 as libc::c_int {
        (*e).vertical = 1 as libc::c_int;
        (*e).x.quo = (*edge).line.p1.x;
        (*e).x.rem = 0 as libc::c_int;
        (*e).dxdy.quo = 0 as libc::c_int;
        (*e).dxdy.rem = 0 as libc::c_int;
        (*e).dy = 0 as libc::c_int;
    } else {
        (*e).vertical = 0 as libc::c_int;
        (*e).dxdy = floored_muldivrem(dx, (1 as libc::c_int) << 8 as libc::c_int, dy);
        (*e).dy = dy;
        (*e)
            .x = floored_muldivrem(
            ytop * ((1 as libc::c_int) << 8 as libc::c_int)
                + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                    >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                    / 2 as libc::c_int - (*edge).line.p1.y,
            dx,
            dy,
        );
        let ref mut fresh10 = (*e).x.quo;
        *fresh10 += (*edge).line.p1.x;
    }
    let ref mut fresh11 = (*e).x.rem;
    *fresh11 -= dy;
    _polygon_insert_edge_into_its_y_bucket(polygon, e, ytop);
}
unsafe extern "C" fn merge_sorted_edges(
    mut head_a: *mut edge,
    mut head_b: *mut edge,
) -> *mut edge {
    let mut current_block: u64;
    let mut head: *mut edge = 0 as *mut edge;
    let mut next: *mut *mut edge = 0 as *mut *mut edge;
    let mut prev: *mut edge = 0 as *mut edge;
    let mut x: int32_t = 0;
    prev = (*head_a).prev;
    next = &mut head;
    if (*head_a).x.quo <= (*head_b).x.quo {
        head = head_a;
        current_block = 13513818773234778473;
    } else {
        head = head_b;
        let ref mut fresh12 = (*head_b).prev;
        *fresh12 = prev;
        current_block = 77293541581027619;
    }
    loop {
        match current_block {
            77293541581027619 => {
                x = (*head_a).x.quo;
                while !head_b.is_null() && (*head_b).x.quo <= x {
                    prev = head_b;
                    next = &mut (*head_b).next;
                    head_b = (*head_b).next;
                }
                let ref mut fresh14 = (*head_a).prev;
                *fresh14 = prev;
                *next = head_a;
                if head_b.is_null() {
                    return head;
                }
                current_block = 13513818773234778473;
            }
            _ => {
                x = (*head_b).x.quo;
                while !head_a.is_null() && (*head_a).x.quo <= x {
                    prev = head_a;
                    next = &mut (*head_a).next;
                    head_a = (*head_a).next;
                }
                let ref mut fresh13 = (*head_b).prev;
                *fresh13 = prev;
                *next = head_b;
                if head_a.is_null() {
                    return head;
                }
                current_block = 77293541581027619;
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
        let ref mut fresh15 = (*head_other).next;
        *fresh15 = 0 as *mut edge;
    } else {
        *head_out = head_other;
        let ref mut fresh16 = (*head_other).prev;
        *fresh16 = (*list).prev;
        let ref mut fresh17 = (*head_other).next;
        *fresh17 = list;
        let ref mut fresh18 = (*list).prev;
        *fresh18 = head_other;
        let ref mut fresh19 = (*list).next;
        *fresh19 = 0 as *mut edge;
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
    mut head: *mut edge,
    mut unsorted: *mut edge,
) -> *mut edge {
    sort_edges(
        unsorted,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
        &mut unsorted,
    );
    return merge_sorted_edges(head, unsorted);
}
#[inline]
unsafe extern "C" fn active_list_merge_edges(
    mut c: *mut mono_scan_converter,
    mut edges: *mut edge,
) {
    let mut e: *mut edge = 0 as *mut edge;
    e = edges;
    while (*c).is_vertical != 0 && !e.is_null() {
        (*c).is_vertical = (*e).vertical;
        e = (*e).next;
    }
    let ref mut fresh20 = (*c).head.next;
    *fresh20 = merge_unsorted_edges((*c).head.next, edges);
}
#[inline]
unsafe extern "C" fn add_span(
    mut c: *mut mono_scan_converter,
    mut x1: libc::c_int,
    mut x2: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    if x1 < (*c).xmin {
        x1 = (*c).xmin;
    }
    if x2 > (*c).xmax {
        x2 = (*c).xmax;
    }
    if x2 <= x1 {
        return;
    }
    let ref mut fresh21 = (*c).num_spans;
    let fresh22 = *fresh21;
    *fresh21 = *fresh21 + 1;
    n = fresh22;
    (*((*c).spans).offset(n as isize)).x = x1;
    (*((*c).spans).offset(n as isize)).coverage = 255 as libc::c_int as uint8_t;
    let ref mut fresh23 = (*c).num_spans;
    let fresh24 = *fresh23;
    *fresh23 = *fresh23 + 1;
    n = fresh24;
    (*((*c).spans).offset(n as isize)).x = x2;
    (*((*c).spans).offset(n as isize)).coverage = 0 as libc::c_int as uint8_t;
}
#[inline]
unsafe extern "C" fn row(mut c: *mut mono_scan_converter, mut mask: libc::c_uint) {
    let mut edge: *mut edge = (*c).head.next;
    let mut xstart: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut prev_x: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut winding: libc::c_int = 0 as libc::c_int;
    (*c).num_spans = 0 as libc::c_int;
    while &mut (*c).tail as *mut edge != edge {
        let mut next: *mut edge = (*edge).next;
        let mut xend: libc::c_int = _cairo_fixed_integer_round_down((*edge).x.quo);
        let ref mut fresh25 = (*edge).height_left;
        *fresh25 -= 1;
        if *fresh25 != 0 {
            if (*edge).vertical == 0 {
                let ref mut fresh26 = (*edge).x.quo;
                *fresh26 += (*edge).dxdy.quo;
                let ref mut fresh27 = (*edge).x.rem;
                *fresh27 += (*edge).dxdy.rem;
                if (*edge).x.rem >= 0 as libc::c_int {
                    let ref mut fresh28 = (*edge).x.quo;
                    *fresh28 += 1;
                    let ref mut fresh29 = (*edge).x.rem;
                    *fresh29 -= (*edge).dy;
                }
            }
            if (*edge).x.quo < prev_x {
                let mut pos: *mut edge = (*edge).prev;
                let ref mut fresh30 = (*pos).next;
                *fresh30 = next;
                let ref mut fresh31 = (*next).prev;
                *fresh31 = pos;
                loop {
                    pos = (*pos).prev;
                    if !((*edge).x.quo < (*pos).x.quo) {
                        break;
                    }
                }
                let ref mut fresh32 = (*(*pos).next).prev;
                *fresh32 = edge;
                let ref mut fresh33 = (*edge).next;
                *fresh33 = (*pos).next;
                let ref mut fresh34 = (*edge).prev;
                *fresh34 = pos;
                let ref mut fresh35 = (*pos).next;
                *fresh35 = edge;
            } else {
                prev_x = (*edge).x.quo;
            }
        } else {
            let ref mut fresh36 = (*(*edge).prev).next;
            *fresh36 = next;
            let ref mut fresh37 = (*next).prev;
            *fresh37 = (*edge).prev;
        }
        winding += (*edge).dir;
        if winding as libc::c_uint & mask == 0 as libc::c_int as libc::c_uint {
            if _cairo_fixed_integer_round_down((*next).x.quo) > xend + 1 as libc::c_int {
                add_span(c, xstart, xend);
                xstart = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            }
        } else if xstart == -(2147483647 as libc::c_int) - 1 as libc::c_int {
            xstart = xend;
        }
        edge = next;
    }
}
unsafe extern "C" fn _mono_scan_converter_init(
    mut c: *mut mono_scan_converter,
    mut xmin: libc::c_int,
    mut ymin: libc::c_int,
    mut xmax: libc::c_int,
    mut ymax: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut max_num_spans: libc::c_int = 0;
    status = polygon_init(((*c).polygon).as_mut_ptr(), ymin, ymax);
    if status as u64 != 0 {
        return status;
    }
    max_num_spans = xmax - xmin + 1 as libc::c_int;
    if max_num_spans
        > (::std::mem::size_of::<[cairo_half_open_span_t; 64]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
            ) as libc::c_int
    {
        let ref mut fresh38 = (*c).spans;
        *fresh38 = _cairo_malloc_ab(
            max_num_spans as size_t,
            ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
        ) as *mut cairo_half_open_span_t;
        if ((*c).spans).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    } else {
        let ref mut fresh39 = (*c).spans;
        *fresh39 = ((*c).spans_embedded).as_mut_ptr();
    }
    (*c).xmin = xmin;
    (*c).xmax = xmax;
    (*c).ymin = ymin;
    (*c).ymax = ymax;
    (*c).head.vertical = 1 as libc::c_int;
    (*c).head.height_left = 2147483647 as libc::c_int;
    (*c)
        .head
        .x
        .quo = _cairo_fixed_from_int(
        _cairo_fixed_integer_part(-(2147483647 as libc::c_int) - 1 as libc::c_int),
    );
    let ref mut fresh40 = (*c).head.prev;
    *fresh40 = 0 as *mut edge;
    let ref mut fresh41 = (*c).head.next;
    *fresh41 = &mut (*c).tail;
    let ref mut fresh42 = (*c).tail.prev;
    *fresh42 = &mut (*c).head;
    let ref mut fresh43 = (*c).tail.next;
    *fresh43 = 0 as *mut edge;
    (*c)
        .tail
        .x
        .quo = _cairo_fixed_from_int(
        _cairo_fixed_integer_part(2147483647 as libc::c_int),
    );
    (*c).tail.height_left = 2147483647 as libc::c_int;
    (*c).tail.vertical = 1 as libc::c_int;
    (*c).is_vertical = 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _mono_scan_converter_fini(mut self_0: *mut mono_scan_converter) {
    if (*self_0).spans != ((*self_0).spans_embedded).as_mut_ptr() {
        free((*self_0).spans as *mut libc::c_void);
    }
    polygon_fini(((*self_0).polygon).as_mut_ptr());
}
unsafe extern "C" fn mono_scan_converter_allocate_edges(
    mut c: *mut mono_scan_converter,
    mut num_edges: libc::c_int,
) -> cairo_status_t {
    (*((*c).polygon).as_mut_ptr()).num_edges = 0 as libc::c_int;
    let ref mut fresh44 = (*((*c).polygon).as_mut_ptr()).edges;
    *fresh44 = ((*((*c).polygon).as_mut_ptr()).edges_embedded).as_mut_ptr();
    if num_edges
        > (::std::mem::size_of::<[edge; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<edge>() as libc::c_ulong) as libc::c_int
    {
        let ref mut fresh45 = (*((*c).polygon).as_mut_ptr()).edges;
        *fresh45 = _cairo_malloc_ab(
            num_edges as size_t,
            ::std::mem::size_of::<edge>() as libc::c_ulong,
        ) as *mut edge;
        if ((*((*c).polygon).as_mut_ptr()).edges).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn mono_scan_converter_add_edge(
    mut c: *mut mono_scan_converter,
    mut edge: *const cairo_edge_t,
) {
    polygon_add_edge(((*c).polygon).as_mut_ptr(), edge);
}
unsafe extern "C" fn step_edges(
    mut c: *mut mono_scan_converter,
    mut count: libc::c_int,
) {
    let mut edge: *mut edge = 0 as *mut edge;
    edge = (*c).head.next;
    while edge != &mut (*c).tail as *mut edge {
        let ref mut fresh46 = (*edge).height_left;
        *fresh46 -= count;
        if (*edge).height_left == 0 {
            let ref mut fresh47 = (*(*edge).prev).next;
            *fresh47 = (*edge).next;
            let ref mut fresh48 = (*(*edge).next).prev;
            *fresh48 = (*edge).prev;
        }
        edge = (*edge).next;
    }
}
unsafe extern "C" fn mono_scan_converter_render(
    mut c: *mut mono_scan_converter,
    mut winding_mask: libc::c_uint,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut polygon: *mut polygon = ((*c).polygon).as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut h: libc::c_int = (*c).ymax - (*c).ymin;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    i = 0 as libc::c_int;
    while i < h {
        j = i + 1 as libc::c_int;
        if !(*((*polygon).y_buckets).offset(i as isize)).is_null() {
            active_list_merge_edges(c, *((*polygon).y_buckets).offset(i as isize));
        }
        if (*c).is_vertical != 0 {
            let mut min_height: libc::c_int = 0;
            let mut e: *mut edge = 0 as *mut edge;
            e = (*c).head.next;
            min_height = (*e).height_left;
            while e != &mut (*c).tail as *mut edge {
                if (*e).height_left < min_height {
                    min_height = (*e).height_left;
                }
                e = (*e).next;
            }
            loop {
                min_height -= 1;
                if !(min_height >= 1 as libc::c_int
                    && (*((*polygon).y_buckets).offset(j as isize)).is_null())
                {
                    break;
                }
                j += 1;
            }
            if j != i + 1 as libc::c_int {
                step_edges(c, j - (i + 1 as libc::c_int));
            }
        }
        row(c, winding_mask);
        if (*c).num_spans != 0 {
            status = ((*renderer).render_rows)
                .expect(
                    "non-null function pointer",
                )(
                renderer as *mut libc::c_void,
                (*c).ymin + i,
                j - i,
                (*c).spans,
                (*c).num_spans as libc::c_uint,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        if (*c).head.next == &mut (*c).tail as *mut edge {
            (*c).is_vertical = 1 as libc::c_int;
        }
        i = j;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_mono_scan_converter_destroy(
    mut converter: *mut libc::c_void,
) {
    let mut self_0: *mut cairo_mono_scan_converter_t = converter
        as *mut cairo_mono_scan_converter_t;
    _mono_scan_converter_fini(((*self_0).converter).as_mut_ptr());
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mono_scan_converter_add_polygon(
    mut converter: *mut libc::c_void,
    mut polygon: *const cairo_polygon_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_mono_scan_converter_t = converter
        as *mut cairo_mono_scan_converter_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    status = mono_scan_converter_allocate_edges(
        ((*self_0).converter).as_mut_ptr(),
        (*polygon).num_edges,
    );
    if status as u64 != 0 {
        return status;
    }
    i = 0 as libc::c_int;
    while i < (*polygon).num_edges {
        mono_scan_converter_add_edge(
            ((*self_0).converter).as_mut_ptr(),
            &mut *((*polygon).edges).offset(i as isize),
        );
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_mono_scan_converter_generate(
    mut converter: *mut libc::c_void,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_mono_scan_converter_t = converter
        as *mut cairo_mono_scan_converter_t;
    return mono_scan_converter_render(
        ((*self_0).converter).as_mut_ptr(),
        (if (*self_0).fill_rule as libc::c_uint
            == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
        {
            !(0 as libc::c_int)
        } else {
            1 as libc::c_int
        }) as libc::c_uint,
        renderer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mono_scan_converter_create(
    mut xmin: libc::c_int,
    mut ymin: libc::c_int,
    mut xmax: libc::c_int,
    mut ymax: libc::c_int,
    mut fill_rule: cairo_fill_rule_t,
) -> *mut cairo_scan_converter_t {
    let mut self_0: *mut cairo_mono_scan_converter_t = 0
        as *mut cairo_mono_scan_converter_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    self_0 = (if ::std::mem::size_of::<_cairo_mono_scan_converter>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<_cairo_mono_scan_converter>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_mono_scan_converter_t;
    if self_0.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        let ref mut fresh49 = (*self_0).base.destroy;
        *fresh49 = Some(
            _cairo_mono_scan_converter_destroy
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        let ref mut fresh50 = (*self_0).base.generate;
        *fresh50 = Some(
            _cairo_mono_scan_converter_generate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_span_renderer_t,
                ) -> cairo_status_t,
        );
        status = _mono_scan_converter_init(
            ((*self_0).converter).as_mut_ptr(),
            xmin,
            ymin,
            xmax,
            ymax,
        );
        if status as u64 != 0 {
            ((*self_0).base.destroy)
                .expect(
                    "non-null function pointer",
                )(
                &mut (*self_0).base as *mut cairo_scan_converter_t as *mut libc::c_void,
            );
        } else {
            (*self_0).fill_rule = fill_rule;
            return &mut (*self_0).base;
        }
    }
    return _cairo_scan_converter_create_in_error(status);
}
