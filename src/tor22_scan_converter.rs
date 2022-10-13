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
    fn _cairo_scan_converter_set_error(
        abstract_converter: *mut libc::c_void,
        error: cairo_status_t,
    ) -> cairo_status_t;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type uint32_t = __uint32_t;
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
pub type cairo_tor22_scan_converter_t = _cairo_tor22_scan_converter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tor22_scan_converter {
    pub base: cairo_scan_converter_t,
    pub converter: [glitter_scan_converter_t; 1],
    pub fill_rule: cairo_fill_rule_t,
    pub antialias: cairo_antialias_t,
    pub jmp: jmp_buf,
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
    pub spans: *mut cairo_half_open_span_t,
    pub spans_embedded: [cairo_half_open_span_t; 64],
    pub xmin: grid_scaled_x_t,
    pub xmax: grid_scaled_x_t,
    pub ymin: grid_scaled_y_t,
    pub ymax: grid_scaled_y_t,
}
pub type grid_scaled_y_t = libc::c_int;
pub type grid_scaled_x_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell_list {
    pub head: cell,
    pub tail: cell,
    pub cursor: *mut cell,
    pub rewind: *mut cell,
    pub cell_pool: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub base: [pool; 1],
    pub embedded: [cell; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub next: *mut cell,
    pub x: libc::c_int,
    pub uncovered_area: int16_t,
    pub covered_height: int16_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct active_list {
    pub head: edge,
    pub tail: edge,
    pub min_height: grid_scaled_y_t,
    pub is_vertical: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge {
    pub next: *mut edge,
    pub prev: *mut edge,
    pub height_left: grid_scaled_y_t,
    pub dir: libc::c_int,
    pub vertical: libc::c_int,
    pub x: quorem,
    pub dxdy: quorem,
    pub ytop: grid_scaled_y_t,
    pub dy: grid_scaled_y_t,
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
    pub edge_pool: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub base: [pool; 1],
    pub embedded: [edge; 32],
}
pub type glitter_status_t = cairo_status_t;
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
unsafe extern "C" fn cell_list_set_rewind(mut cells: *mut cell_list) {
    let ref mut fresh15 = (*cells).rewind;
    *fresh15 = (*cells).cursor;
}
unsafe extern "C" fn cell_list_init(mut cells: *mut cell_list, mut jmp: *mut jmp_buf) {
    pool_init(
        ((*cells).cell_pool.base).as_mut_ptr(),
        jmp,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
        ::std::mem::size_of::<[cell; 32]>() as libc::c_ulong,
    );
    let ref mut fresh16 = (*cells).tail.next;
    *fresh16 = 0 as *mut cell;
    (*cells).tail.x = 2147483647 as libc::c_int;
    (*cells).head.x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh17 = (*cells).head.next;
    *fresh17 = &mut (*cells).tail;
    cell_list_rewind(cells);
}
unsafe extern "C" fn cell_list_fini(mut cells: *mut cell_list) {
    pool_fini(((*cells).cell_pool.base).as_mut_ptr());
}
#[inline]
unsafe extern "C" fn cell_list_reset(mut cells: *mut cell_list) {
    cell_list_rewind(cells);
    let ref mut fresh18 = (*cells).head.next;
    *fresh18 = &mut (*cells).tail;
    pool_reset(((*cells).cell_pool.base).as_mut_ptr());
}
#[inline]
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
    let ref mut fresh19 = (*cell).next;
    *fresh19 = (*tail).next;
    let ref mut fresh20 = (*tail).next;
    *fresh20 = cell;
    (*cell).x = x;
    *(&mut (*cell).uncovered_area as *mut int16_t
        as *mut uint32_t) = 0 as libc::c_int as uint32_t;
    return cell;
}
#[inline]
unsafe extern "C" fn cell_list_find(
    mut cells: *mut cell_list,
    mut x: libc::c_int,
) -> *mut cell {
    let mut tail: *mut cell = (*cells).cursor;
    if (*tail).x == x {
        return tail;
    }
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
    let ref mut fresh21 = (*cells).cursor;
    *fresh21 = tail;
    return *fresh21;
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
        pair.cell1 = cell_list_alloc(cells, pair.cell1, x1);
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
        pair.cell2 = cell_list_alloc(cells, pair.cell2, x2);
    }
    let ref mut fresh22 = (*cells).cursor;
    *fresh22 = pair.cell2;
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
    if x1 == x2 {
        return;
    }
    fx1 = x1 & ((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int;
    ix1 = x1 >> 2 as libc::c_int;
    fx2 = x2 & ((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int;
    ix2 = x2 >> 2 as libc::c_int;
    if ix1 != ix2 {
        let mut p: cell_pair = cell_pair {
            cell1: 0 as *mut cell,
            cell2: 0 as *mut cell,
        };
        p = cell_list_find_pair(cells, ix1, ix2);
        let ref mut fresh23 = (*p.cell1).uncovered_area;
        *fresh23 = (*fresh23 as libc::c_int + 2 as libc::c_int * fx1) as int16_t;
        let ref mut fresh24 = (*p.cell1).covered_height;
        *fresh24 += 1;
        let ref mut fresh25 = (*p.cell2).uncovered_area;
        *fresh25 = (*fresh25 as libc::c_int - 2 as libc::c_int * fx2) as int16_t;
        let ref mut fresh26 = (*p.cell2).covered_height;
        *fresh26 -= 1;
    } else {
        let mut cell: *mut cell = cell_list_find(cells, ix1);
        let ref mut fresh27 = (*cell).uncovered_area;
        *fresh27 = (*fresh27 as libc::c_int + 2 as libc::c_int * (fx1 - fx2)) as int16_t;
    };
}
unsafe extern "C" fn cell_list_render_edge(
    mut cells: *mut cell_list,
    mut edge: *mut edge,
    mut sign: libc::c_int,
) {
    let mut fx: grid_scaled_x_t = 0;
    let mut cell: *mut cell = 0 as *mut cell;
    let mut ix: libc::c_int = 0;
    fx = (*edge).x.quo & ((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int;
    ix = (*edge).x.quo >> 2 as libc::c_int;
    cell = cell_list_find(cells, ix);
    let ref mut fresh28 = (*cell).covered_height;
    *fresh28 = (*fresh28 as libc::c_int
        + sign * ((1 as libc::c_int) << 2 as libc::c_int)) as int16_t;
    let ref mut fresh29 = (*cell).uncovered_area;
    *fresh29 = (*fresh29 as libc::c_int
        + sign * 2 as libc::c_int * fx * ((1 as libc::c_int) << 2 as libc::c_int))
        as int16_t;
}
unsafe extern "C" fn polygon_init(mut polygon: *mut polygon, mut jmp: *mut jmp_buf) {
    let ref mut fresh30 = (*polygon).ymax;
    *fresh30 = 0 as libc::c_int;
    (*polygon).ymin = *fresh30;
    let ref mut fresh31 = (*polygon).y_buckets;
    *fresh31 = ((*polygon).y_buckets_embedded).as_mut_ptr();
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
) -> glitter_status_t {
    let mut current_block: u64;
    let mut h: libc::c_uint = (ymax - ymin) as libc::c_uint;
    let mut num_buckets: libc::c_uint = ((ymax + ((1 as libc::c_int) << 2 as libc::c_int)
        - 1 as libc::c_int - ymin) / ((1 as libc::c_int) << 2 as libc::c_int))
        as libc::c_uint;
    pool_reset(((*polygon).edge_pool.base).as_mut_ptr());
    if !(h
        > (0x7fffffff as libc::c_uint)
            .wrapping_sub(((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint))
    {
        if (*polygon).y_buckets != ((*polygon).y_buckets_embedded).as_mut_ptr() {
            free((*polygon).y_buckets as *mut libc::c_void);
        }
        let ref mut fresh32 = (*polygon).y_buckets;
        *fresh32 = ((*polygon).y_buckets_embedded).as_mut_ptr();
        if num_buckets
            > (::std::mem::size_of::<[*mut edge; 64]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut edge>() as libc::c_ulong)
                as libc::c_int as libc::c_uint
        {
            let ref mut fresh33 = (*polygon).y_buckets;
            *fresh33 = _cairo_malloc_ab(
                num_buckets as size_t,
                ::std::mem::size_of::<*mut edge>() as libc::c_ulong,
            ) as *mut *mut edge;
            if ((*polygon).y_buckets).is_null() {
                current_block = 9169308322834185463;
            } else {
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            9169308322834185463 => {}
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
    let mut ix: libc::c_uint = (((*e).ytop - (*polygon).ymin)
        / ((1 as libc::c_int) << 2 as libc::c_int)) as libc::c_uint;
    let mut ptail: *mut *mut edge = &mut *((*polygon).y_buckets).offset(ix as isize)
        as *mut *mut edge;
    let ref mut fresh34 = (*e).next;
    *fresh34 = *ptail;
    *ptail = e;
}
#[inline]
unsafe extern "C" fn polygon_add_edge(
    mut polygon: *mut polygon,
    mut edge: *const cairo_edge_t,
) {
    let mut e: *mut edge = 0 as *mut edge;
    let mut dx: grid_scaled_x_t = 0;
    let mut dy: grid_scaled_y_t = 0;
    let mut ytop: grid_scaled_y_t = 0;
    let mut ybot: grid_scaled_y_t = 0;
    let mut ymin: grid_scaled_y_t = (*polygon).ymin;
    let mut ymax: grid_scaled_y_t = (*polygon).ymax;
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
    } else {
        (*e).vertical = 0 as libc::c_int;
        (*e).dxdy = floored_divrem(dx, dy);
        if ytop == (*edge).line.p1.y {
            (*e).x.quo = (*edge).line.p1.x;
            (*e).x.rem = 0 as libc::c_int;
        } else {
            (*e).x = floored_muldivrem(ytop - (*edge).line.p1.y, dx, dy);
            let ref mut fresh35 = (*e).x.quo;
            *fresh35 += (*edge).line.p1.x;
        }
    }
    _polygon_insert_edge_into_its_y_bucket(polygon, e);
    let ref mut fresh36 = (*e).x.rem;
    *fresh36 -= dy;
}
unsafe extern "C" fn active_list_reset(mut active: *mut active_list) {
    (*active).head.vertical = 1 as libc::c_int;
    (*active).head.height_left = 2147483647 as libc::c_int;
    (*active).head.x.quo = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let ref mut fresh37 = (*active).head.prev;
    *fresh37 = 0 as *mut edge;
    let ref mut fresh38 = (*active).head.next;
    *fresh38 = &mut (*active).tail;
    let ref mut fresh39 = (*active).tail.prev;
    *fresh39 = &mut (*active).head;
    let ref mut fresh40 = (*active).tail.next;
    *fresh40 = 0 as *mut edge;
    (*active).tail.x.quo = 2147483647 as libc::c_int;
    (*active).tail.height_left = 2147483647 as libc::c_int;
    (*active).tail.vertical = 1 as libc::c_int;
    (*active).min_height = 0 as libc::c_int;
    (*active).is_vertical = 1 as libc::c_int;
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
    let mut prev: *mut edge = 0 as *mut edge;
    let mut x: int32_t = 0;
    prev = (*head_a).prev;
    next = &mut head;
    if (*head_a).x.quo <= (*head_b).x.quo {
        head = head_a;
        current_block = 13513818773234778473;
    } else {
        head = head_b;
        let ref mut fresh41 = (*head_b).prev;
        *fresh41 = prev;
        current_block = 17834779195348318434;
    }
    loop {
        match current_block {
            17834779195348318434 => {
                x = (*head_a).x.quo;
                while !head_b.is_null() && (*head_b).x.quo <= x {
                    prev = head_b;
                    next = &mut (*head_b).next;
                    head_b = (*head_b).next;
                }
                let ref mut fresh43 = (*head_a).prev;
                *fresh43 = prev;
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
                let ref mut fresh42 = (*head_b).prev;
                *fresh42 = prev;
                *next = head_b;
                if head_a.is_null() {
                    return head;
                }
                current_block = 17834779195348318434;
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
        let ref mut fresh44 = (*head_other).next;
        *fresh44 = 0 as *mut edge;
    } else {
        *head_out = head_other;
        let ref mut fresh45 = (*head_other).prev;
        *fresh45 = (*list).prev;
        let ref mut fresh46 = (*head_other).next;
        *fresh46 = list;
        let ref mut fresh47 = (*list).prev;
        *fresh47 = head_other;
        let ref mut fresh48 = (*list).next;
        *fresh48 = 0 as *mut edge;
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
unsafe extern "C" fn can_do_full_row(mut active: *mut active_list) -> libc::c_int {
    let mut e: *const edge = 0 as *const edge;
    if (*active).min_height <= 0 as libc::c_int {
        let mut min_height: libc::c_int = 2147483647 as libc::c_int;
        let mut is_vertical: libc::c_int = 1 as libc::c_int;
        e = (*active).head.next;
        while !e.is_null() {
            if (*e).height_left < min_height {
                min_height = (*e).height_left;
            }
            is_vertical &= (*e).vertical;
            e = (*e).next;
        }
        (*active).is_vertical = is_vertical;
        (*active).min_height = min_height;
    }
    if (*active).min_height < (1 as libc::c_int) << 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (*active).is_vertical;
}
#[inline]
unsafe extern "C" fn active_list_merge_edges_from_bucket(
    mut active: *mut active_list,
    mut edges: *mut edge,
) {
    let ref mut fresh49 = (*active).head.next;
    *fresh49 = merge_unsorted_edges((*active).head.next, edges);
}
#[inline]
unsafe extern "C" fn polygon_fill_buckets(
    mut active: *mut active_list,
    mut edge: *mut edge,
    mut y: libc::c_int,
    mut buckets: *mut *mut edge,
) {
    let mut min_height: grid_scaled_y_t = (*active).min_height;
    let mut is_vertical: libc::c_int = (*active).is_vertical;
    while !edge.is_null() {
        let mut next: *mut edge = (*edge).next;
        let mut suby: libc::c_int = (*edge).ytop - y;
        if !(*buckets.offset(suby as isize)).is_null() {
            let ref mut fresh50 = (**buckets.offset(suby as isize)).prev;
            *fresh50 = edge;
        }
        let ref mut fresh51 = (*edge).next;
        *fresh51 = *buckets.offset(suby as isize);
        let ref mut fresh52 = (*edge).prev;
        *fresh52 = 0 as *mut edge;
        let ref mut fresh53 = *buckets.offset(suby as isize);
        *fresh53 = edge;
        if (*edge).height_left < min_height {
            min_height = (*edge).height_left;
        }
        is_vertical &= (*edge).vertical;
        edge = next;
    }
    (*active).is_vertical = is_vertical;
    (*active).min_height = min_height;
}
#[inline]
unsafe extern "C" fn sub_row(
    mut active: *mut active_list,
    mut coverages: *mut cell_list,
    mut mask: libc::c_uint,
) {
    let mut edge: *mut edge = (*active).head.next;
    let mut xstart: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut prev_x: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut winding: libc::c_int = 0 as libc::c_int;
    cell_list_rewind(coverages);
    while &mut (*active).tail as *mut edge != edge {
        let mut next: *mut edge = (*edge).next;
        let mut xend: libc::c_int = (*edge).x.quo;
        let ref mut fresh54 = (*edge).height_left;
        *fresh54 -= 1;
        if *fresh54 != 0 {
            let ref mut fresh55 = (*edge).x.quo;
            *fresh55 += (*edge).dxdy.quo;
            let ref mut fresh56 = (*edge).x.rem;
            *fresh56 += (*edge).dxdy.rem;
            if (*edge).x.rem >= 0 as libc::c_int {
                let ref mut fresh57 = (*edge).x.quo;
                *fresh57 += 1;
                let ref mut fresh58 = (*edge).x.rem;
                *fresh58 -= (*edge).dy;
            }
            if (*edge).x.quo < prev_x {
                let mut pos: *mut edge = (*edge).prev;
                let ref mut fresh59 = (*pos).next;
                *fresh59 = next;
                let ref mut fresh60 = (*next).prev;
                *fresh60 = pos;
                loop {
                    pos = (*pos).prev;
                    if !((*edge).x.quo < (*pos).x.quo) {
                        break;
                    }
                }
                let ref mut fresh61 = (*(*pos).next).prev;
                *fresh61 = edge;
                let ref mut fresh62 = (*edge).next;
                *fresh62 = (*pos).next;
                let ref mut fresh63 = (*edge).prev;
                *fresh63 = pos;
                let ref mut fresh64 = (*pos).next;
                *fresh64 = edge;
            } else {
                prev_x = (*edge).x.quo;
            }
        } else {
            let ref mut fresh65 = (*(*edge).prev).next;
            *fresh65 = next;
            let ref mut fresh66 = (*next).prev;
            *fresh66 = (*edge).prev;
        }
        winding += (*edge).dir;
        if winding as libc::c_uint & mask == 0 as libc::c_int as libc::c_uint {
            if (*next).x.quo != xend {
                cell_list_add_subspan(coverages, xstart, xend);
                xstart = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            }
        } else if xstart == -(2147483647 as libc::c_int) - 1 as libc::c_int {
            xstart = xend;
        }
        edge = next;
    }
}
#[inline]
unsafe extern "C" fn dec(mut e: *mut edge, mut h: libc::c_int) {
    let ref mut fresh67 = (*e).height_left;
    *fresh67 -= h;
    if (*e).height_left == 0 as libc::c_int {
        let ref mut fresh68 = (*(*e).prev).next;
        *fresh68 = (*e).next;
        let ref mut fresh69 = (*(*e).next).prev;
        *fresh69 = (*e).prev;
    }
}
unsafe extern "C" fn full_row(
    mut active: *mut active_list,
    mut coverages: *mut cell_list,
    mut mask: libc::c_uint,
) {
    let mut left: *mut edge = (*active).head.next;
    while &mut (*active).tail as *mut edge != left {
        let mut right: *mut edge = 0 as *mut edge;
        let mut winding: libc::c_int = 0;
        dec(left, (1 as libc::c_int) << 2 as libc::c_int);
        winding = (*left).dir;
        right = (*left).next;
        loop {
            dec(right, (1 as libc::c_int) << 2 as libc::c_int);
            winding += (*right).dir;
            if winding as libc::c_uint & mask == 0 as libc::c_int as libc::c_uint
                && (*(*right).next).x.quo != (*right).x.quo
            {
                break;
            }
            right = (*right).next;
        }
        cell_list_set_rewind(coverages);
        cell_list_render_edge(coverages, left, 1 as libc::c_int);
        cell_list_render_edge(coverages, right, -(1 as libc::c_int));
        left = (*right).next;
    }
}
unsafe extern "C" fn _glitter_scan_converter_init(
    mut converter: *mut glitter_scan_converter_t,
    mut jmp: *mut jmp_buf,
) {
    polygon_init(((*converter).polygon).as_mut_ptr(), jmp);
    active_list_init(((*converter).active).as_mut_ptr());
    cell_list_init(((*converter).coverages).as_mut_ptr(), jmp);
    (*converter).xmin = 0 as libc::c_int;
    (*converter).ymin = 0 as libc::c_int;
    (*converter).xmax = 0 as libc::c_int;
    (*converter).ymax = 0 as libc::c_int;
}
unsafe extern "C" fn _glitter_scan_converter_fini(
    mut self_0: *mut glitter_scan_converter_t,
) {
    if (*self_0).spans != ((*self_0).spans_embedded).as_mut_ptr() {
        free((*self_0).spans as *mut libc::c_void);
    }
    polygon_fini(((*self_0).polygon).as_mut_ptr());
    cell_list_fini(((*self_0).coverages).as_mut_ptr());
    (*self_0).xmin = 0 as libc::c_int;
    (*self_0).ymin = 0 as libc::c_int;
    (*self_0).xmax = 0 as libc::c_int;
    (*self_0).ymax = 0 as libc::c_int;
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
    mut xmin: libc::c_int,
    mut ymin: libc::c_int,
    mut xmax: libc::c_int,
    mut ymax: libc::c_int,
) -> glitter_status_t {
    let mut status: glitter_status_t = CAIRO_STATUS_SUCCESS;
    let mut max_num_spans: libc::c_int = 0;
    (*converter).xmin = 0 as libc::c_int;
    (*converter).xmax = 0 as libc::c_int;
    (*converter).ymin = 0 as libc::c_int;
    (*converter).ymax = 0 as libc::c_int;
    max_num_spans = xmax - xmin + 1 as libc::c_int;
    if max_num_spans
        > (::std::mem::size_of::<[cairo_half_open_span_t; 64]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
            ) as libc::c_int
    {
        let ref mut fresh70 = (*converter).spans;
        *fresh70 = _cairo_malloc_ab(
            max_num_spans as size_t,
            ::std::mem::size_of::<cairo_half_open_span_t>() as libc::c_ulong,
        ) as *mut cairo_half_open_span_t;
        if ((*converter).spans).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    } else {
        let ref mut fresh71 = (*converter).spans;
        *fresh71 = ((*converter).spans_embedded).as_mut_ptr();
    }
    xmin = int_to_grid_scaled(xmin, (1 as libc::c_int) << 2 as libc::c_int);
    ymin = int_to_grid_scaled(ymin, (1 as libc::c_int) << 2 as libc::c_int);
    xmax = int_to_grid_scaled(xmax, (1 as libc::c_int) << 2 as libc::c_int);
    ymax = int_to_grid_scaled(ymax, (1 as libc::c_int) << 2 as libc::c_int);
    active_list_reset(((*converter).active).as_mut_ptr());
    cell_list_reset(((*converter).coverages).as_mut_ptr());
    status = polygon_reset(((*converter).polygon).as_mut_ptr(), ymin, ymax);
    if status as u64 != 0 {
        return status;
    }
    (*converter).xmin = xmin;
    (*converter).xmax = xmax;
    (*converter).ymin = ymin;
    (*converter).ymax = ymax;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn glitter_scan_converter_add_edge(
    mut converter: *mut glitter_scan_converter_t,
    mut edge: *const cairo_edge_t,
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
    e.top = (*edge).top >> 8 as libc::c_int - 2 as libc::c_int;
    e.bottom = (*edge).bottom >> 8 as libc::c_int - 2 as libc::c_int;
    if e.top >= e.bottom {
        return;
    }
    e.line.p1.y = (*edge).line.p1.y >> 8 as libc::c_int - 2 as libc::c_int;
    e.line.p2.y = (*edge).line.p2.y >> 8 as libc::c_int - 2 as libc::c_int;
    if e.line.p1.y == e.line.p2.y {
        e.line.p2.y += 1;
    }
    e.line.p1.x = (*edge).line.p1.x >> 8 as libc::c_int - 2 as libc::c_int;
    e.line.p2.x = (*edge).line.p2.x >> 8 as libc::c_int - 2 as libc::c_int;
    e.dir = (*edge).dir;
    polygon_add_edge(((*converter).polygon).as_mut_ptr(), &mut e);
}
unsafe extern "C" fn step_edges(mut active: *mut active_list, mut count: libc::c_int) {
    let mut edge: *mut edge = 0 as *mut edge;
    count *= (1 as libc::c_int) << 2 as libc::c_int;
    edge = (*active).head.next;
    while edge != &mut (*active).tail as *mut edge {
        let ref mut fresh72 = (*edge).height_left;
        *fresh72 -= count;
        if (*edge).height_left == 0 {
            let ref mut fresh73 = (*(*edge).prev).next;
            *fresh73 = (*edge).next;
            let ref mut fresh74 = (*(*edge).next).prev;
            *fresh74 = (*edge).prev;
        }
        edge = (*edge).next;
    }
}
unsafe extern "C" fn blit_a8(
    mut cells: *mut cell_list,
    mut renderer: *mut cairo_span_renderer_t,
    mut spans: *mut cairo_half_open_span_t,
    mut y: libc::c_int,
    mut height: libc::c_int,
    mut xmin: libc::c_int,
    mut xmax: libc::c_int,
) -> glitter_status_t {
    let mut cell: *mut cell = (*cells).head.next;
    let mut prev_x: libc::c_int = xmin;
    let mut last_x: libc::c_int = -(1 as libc::c_int);
    let mut cover: int16_t = 0 as libc::c_int as int16_t;
    let mut last_cover: int16_t = 0 as libc::c_int as int16_t;
    let mut num_spans: libc::c_uint = 0;
    if cell == &mut (*cells).tail as *mut cell {
        return CAIRO_STATUS_SUCCESS;
    }
    while (*cell).x < xmin {
        cover = (cover as libc::c_int + (*cell).covered_height as libc::c_int)
            as int16_t;
        cell = (*cell).next;
    }
    cover = (cover as libc::c_int
        * (((1 as libc::c_int) << 2 as libc::c_int) * 2 as libc::c_int)) as int16_t;
    num_spans = 0 as libc::c_int as libc::c_uint;
    while (*cell).x < xmax {
        let mut x: libc::c_int = (*cell).x;
        let mut area: int16_t = 0;
        if x > prev_x && cover as libc::c_int != last_cover as libc::c_int {
            (*spans.offset(num_spans as isize)).x = prev_x;
            (*spans.offset(num_spans as isize))
                .coverage = ((cover as libc::c_int) << 3 as libc::c_int
                | -((cover as libc::c_int & 0x20 as libc::c_int) >> 5 as libc::c_int))
                as uint8_t;
            last_cover = cover;
            last_x = prev_x;
            num_spans = num_spans.wrapping_add(1);
        }
        cover = (cover as libc::c_int
            + (*cell).covered_height as libc::c_int
                * ((1 as libc::c_int) << 2 as libc::c_int) * 2 as libc::c_int)
            as int16_t;
        area = (cover as libc::c_int - (*cell).uncovered_area as libc::c_int) as int16_t;
        if area as libc::c_int != last_cover as libc::c_int {
            (*spans.offset(num_spans as isize)).x = x;
            (*spans.offset(num_spans as isize))
                .coverage = ((area as libc::c_int) << 3 as libc::c_int
                | -((area as libc::c_int & 0x20 as libc::c_int) >> 5 as libc::c_int))
                as uint8_t;
            last_cover = area;
            last_x = x;
            num_spans = num_spans.wrapping_add(1);
        }
        prev_x = x + 1 as libc::c_int;
        cell = (*cell).next;
    }
    if prev_x <= xmax && cover as libc::c_int != last_cover as libc::c_int {
        (*spans.offset(num_spans as isize)).x = prev_x;
        (*spans.offset(num_spans as isize))
            .coverage = ((cover as libc::c_int) << 3 as libc::c_int
            | -((cover as libc::c_int & 0x20 as libc::c_int) >> 5 as libc::c_int))
            as uint8_t;
        last_cover = cover;
        last_x = prev_x;
        num_spans = num_spans.wrapping_add(1);
    }
    if last_x < xmax && last_cover as libc::c_int != 0 {
        (*spans.offset(num_spans as isize)).x = xmax;
        (*spans.offset(num_spans as isize)).coverage = 0 as libc::c_int as uint8_t;
        num_spans = num_spans.wrapping_add(1);
    }
    return ((*renderer).render_rows)
        .expect(
            "non-null function pointer",
        )(renderer as *mut libc::c_void, y, height, spans, num_spans);
}
unsafe extern "C" fn blit_a1(
    mut cells: *mut cell_list,
    mut renderer: *mut cairo_span_renderer_t,
    mut spans: *mut cairo_half_open_span_t,
    mut y: libc::c_int,
    mut height: libc::c_int,
    mut xmin: libc::c_int,
    mut xmax: libc::c_int,
) -> glitter_status_t {
    let mut cell: *mut cell = (*cells).head.next;
    let mut prev_x: libc::c_int = xmin;
    let mut last_x: libc::c_int = -(1 as libc::c_int);
    let mut cover: int16_t = 0 as libc::c_int as int16_t;
    let mut coverage: uint8_t = 0;
    let mut last_cover: uint8_t = 0 as libc::c_int as uint8_t;
    let mut num_spans: libc::c_uint = 0;
    if cell == &mut (*cells).tail as *mut cell {
        return CAIRO_STATUS_SUCCESS;
    }
    while (*cell).x < xmin {
        cover = (cover as libc::c_int + (*cell).covered_height as libc::c_int)
            as int16_t;
        cell = (*cell).next;
    }
    cover = (cover as libc::c_int
        * (((1 as libc::c_int) << 2 as libc::c_int) * 2 as libc::c_int)) as int16_t;
    num_spans = 0 as libc::c_int as libc::c_uint;
    while (*cell).x < xmax {
        let mut x: libc::c_int = (*cell).x;
        let mut area: int16_t = 0;
        coverage = (if (cover as libc::c_int) << 3 as libc::c_int
            | -((cover as libc::c_int & 0x20 as libc::c_int) >> 5 as libc::c_int)
            > 127 as libc::c_int
        {
            255 as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint8_t;
        if x > prev_x && coverage as libc::c_int != last_cover as libc::c_int {
            let ref mut fresh75 = (*spans.offset(num_spans as isize)).x;
            *fresh75 = prev_x;
            last_x = *fresh75;
            let ref mut fresh76 = (*spans.offset(num_spans as isize)).coverage;
            *fresh76 = coverage;
            last_cover = *fresh76;
            num_spans = num_spans.wrapping_add(1);
        }
        cover = (cover as libc::c_int
            + (*cell).covered_height as libc::c_int
                * ((1 as libc::c_int) << 2 as libc::c_int) * 2 as libc::c_int)
            as int16_t;
        area = (cover as libc::c_int - (*cell).uncovered_area as libc::c_int) as int16_t;
        coverage = (if (area as libc::c_int) << 3 as libc::c_int
            | -((area as libc::c_int & 0x20 as libc::c_int) >> 5 as libc::c_int)
            > 127 as libc::c_int
        {
            255 as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint8_t;
        if coverage as libc::c_int != last_cover as libc::c_int {
            let ref mut fresh77 = (*spans.offset(num_spans as isize)).x;
            *fresh77 = x;
            last_x = *fresh77;
            let ref mut fresh78 = (*spans.offset(num_spans as isize)).coverage;
            *fresh78 = coverage;
            last_cover = *fresh78;
            num_spans = num_spans.wrapping_add(1);
        }
        prev_x = x + 1 as libc::c_int;
        cell = (*cell).next;
    }
    coverage = (if (cover as libc::c_int) << 3 as libc::c_int
        | -((cover as libc::c_int & 0x20 as libc::c_int) >> 5 as libc::c_int)
        > 127 as libc::c_int
    {
        255 as libc::c_int
    } else {
        0 as libc::c_int
    }) as uint8_t;
    if prev_x <= xmax && coverage as libc::c_int != last_cover as libc::c_int {
        let ref mut fresh79 = (*spans.offset(num_spans as isize)).x;
        *fresh79 = prev_x;
        last_x = *fresh79;
        let ref mut fresh80 = (*spans.offset(num_spans as isize)).coverage;
        *fresh80 = coverage;
        last_cover = *fresh80;
        num_spans = num_spans.wrapping_add(1);
    }
    if last_x < xmax && last_cover as libc::c_int != 0 {
        (*spans.offset(num_spans as isize)).x = xmax;
        (*spans.offset(num_spans as isize)).coverage = 0 as libc::c_int as uint8_t;
        num_spans = num_spans.wrapping_add(1);
    }
    if num_spans == 1 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    return ((*renderer).render_rows)
        .expect(
            "non-null function pointer",
        )(renderer as *mut libc::c_void, y, height, spans, num_spans);
}
unsafe extern "C" fn glitter_scan_converter_render(
    mut converter: *mut glitter_scan_converter_t,
    mut winding_mask: libc::c_uint,
    mut antialias: libc::c_int,
    mut renderer: *mut cairo_span_renderer_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ymax_i: libc::c_int = (*converter).ymax
        / ((1 as libc::c_int) << 2 as libc::c_int);
    let mut ymin_i: libc::c_int = (*converter).ymin
        / ((1 as libc::c_int) << 2 as libc::c_int);
    let mut xmin_i: libc::c_int = 0;
    let mut xmax_i: libc::c_int = 0;
    let mut h: libc::c_int = ymax_i - ymin_i;
    let mut polygon: *mut polygon = ((*converter).polygon).as_mut_ptr();
    let mut coverages: *mut cell_list = ((*converter).coverages).as_mut_ptr();
    let mut active: *mut active_list = ((*converter).active).as_mut_ptr();
    let mut buckets: [*mut edge; 4] = [
        0 as *mut edge,
        0 as *mut edge,
        0 as *mut edge,
        0 as *mut edge,
    ];
    xmin_i = (*converter).xmin / ((1 as libc::c_int) << 2 as libc::c_int);
    xmax_i = (*converter).xmax / ((1 as libc::c_int) << 2 as libc::c_int);
    if xmin_i >= xmax_i {
        return;
    }
    let mut current_block_36: u64;
    i = 0 as libc::c_int;
    while i < h {
        let mut do_full_row: libc::c_int = 0 as libc::c_int;
        j = i + 1 as libc::c_int;
        if (*((*polygon).y_buckets).offset(i as isize)).is_null() {
            if (*active).head.next == &mut (*active).tail as *mut edge {
                (*active).min_height = 2147483647 as libc::c_int;
                (*active).is_vertical = 1 as libc::c_int;
                while j < h && (*((*polygon).y_buckets).offset(j as isize)).is_null() {
                    j += 1;
                }
                current_block_36 = 13183875560443969876;
            } else {
                do_full_row = can_do_full_row(active);
                current_block_36 = 13797916685926291137;
            }
        } else {
            current_block_36 = 13797916685926291137;
        }
        match current_block_36 {
            13797916685926291137 => {
                if do_full_row != 0 {
                    full_row(active, coverages, winding_mask);
                    if (*active).is_vertical != 0 {
                        while j < h
                            && (*((*polygon).y_buckets).offset(j as isize)).is_null()
                            && (*active).min_height
                                >= 2 as libc::c_int
                                    * ((1 as libc::c_int) << 2 as libc::c_int)
                        {
                            let ref mut fresh81 = (*active).min_height;
                            *fresh81 -= (1 as libc::c_int) << 2 as libc::c_int;
                            j += 1;
                        }
                        if j != i + 1 as libc::c_int {
                            step_edges(active, j - (i + 1 as libc::c_int));
                        }
                    }
                } else {
                    let mut sub: libc::c_int = 0;
                    polygon_fill_buckets(
                        active,
                        *((*polygon).y_buckets).offset(i as isize),
                        (i + ymin_i) * ((1 as libc::c_int) << 2 as libc::c_int),
                        buckets.as_mut_ptr(),
                    );
                    sub = 0 as libc::c_int;
                    while sub < (1 as libc::c_int) << 2 as libc::c_int {
                        if !(buckets[sub as usize]).is_null() {
                            active_list_merge_edges_from_bucket(
                                active,
                                buckets[sub as usize],
                            );
                            buckets[sub as usize] = 0 as *mut edge;
                        }
                        sub_row(active, coverages, winding_mask);
                        sub += 1;
                    }
                }
                if antialias != 0 {
                    blit_a8(
                        coverages,
                        renderer,
                        (*converter).spans,
                        i + ymin_i,
                        j - i,
                        xmin_i,
                        xmax_i,
                    );
                } else {
                    blit_a1(
                        coverages,
                        renderer,
                        (*converter).spans,
                        i + ymin_i,
                        j - i,
                        xmin_i,
                        xmax_i,
                    );
                }
                cell_list_reset(coverages);
                let ref mut fresh82 = (*active).min_height;
                *fresh82 -= (1 as libc::c_int) << 2 as libc::c_int;
            }
            _ => {}
        }
        i = j;
    }
}
unsafe extern "C" fn _cairo_tor22_scan_converter_destroy(
    mut converter: *mut libc::c_void,
) {
    let mut self_0: *mut cairo_tor22_scan_converter_t = converter
        as *mut cairo_tor22_scan_converter_t;
    if self_0.is_null() {
        return;
    }
    _glitter_scan_converter_fini(((*self_0).converter).as_mut_ptr());
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tor22_scan_converter_add_polygon(
    mut converter: *mut libc::c_void,
    mut polygon: *const cairo_polygon_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_tor22_scan_converter_t = converter
        as *mut cairo_tor22_scan_converter_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*polygon).num_edges {
        glitter_scan_converter_add_edge(
            ((*self_0).converter).as_mut_ptr(),
            &mut *((*polygon).edges).offset(i as isize),
        );
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_tor22_scan_converter_generate(
    mut converter: *mut libc::c_void,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    let mut self_0: *mut cairo_tor22_scan_converter_t = converter
        as *mut cairo_tor22_scan_converter_t;
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
        (if (*self_0).fill_rule as libc::c_uint
            == CAIRO_FILL_RULE_WINDING as libc::c_int as libc::c_uint
        {
            !(0 as libc::c_int)
        } else {
            1 as libc::c_int
        }) as libc::c_uint,
        ((*self_0).antialias as libc::c_uint
            != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint) as libc::c_int,
        renderer,
    );
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tor22_scan_converter_create(
    mut xmin: libc::c_int,
    mut ymin: libc::c_int,
    mut xmax: libc::c_int,
    mut ymax: libc::c_int,
    mut fill_rule: cairo_fill_rule_t,
    mut antialias: cairo_antialias_t,
) -> *mut cairo_scan_converter_t {
    let mut self_0: *mut cairo_tor22_scan_converter_t = 0
        as *mut cairo_tor22_scan_converter_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    self_0 = (if ::std::mem::size_of::<_cairo_tor22_scan_converter>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<_cairo_tor22_scan_converter>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_tor22_scan_converter_t;
    if self_0.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        let ref mut fresh83 = (*self_0).base.destroy;
        *fresh83 = Some(
            _cairo_tor22_scan_converter_destroy
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        let ref mut fresh84 = (*self_0).base.generate;
        *fresh84 = Some(
            _cairo_tor22_scan_converter_generate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_span_renderer_t,
                ) -> cairo_status_t,
        );
        _glitter_scan_converter_init(
            ((*self_0).converter).as_mut_ptr(),
            &mut (*self_0).jmp,
        );
        status = glitter_scan_converter_reset(
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
            (*self_0).antialias = antialias;
            return &mut (*self_0).base;
        }
    }
    return _cairo_scan_converter_create_in_error(status);
}
