use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub type cairo_point_t = _cairo_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point {
    pub x: cairo_fixed_t,
    pub y: cairo_fixed_t,
}
pub type cairo_fixed_t = int32_t;
pub type uint64_t = __uint64_t;
pub type cairo_uint64_t = uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_contour_iter {
    pub point: *mut cairo_point_t,
    pub chain: *mut cairo_contour_chain_t,
}
pub type cairo_contour_iter_t = _cairo_contour_iter;
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
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
unsafe extern "C" fn _cairo_contour_add_point(
    mut contour: *mut cairo_contour_t,
    mut point: *const cairo_point_t,
) -> cairo_int_status_t {
    let mut tail: *mut _cairo_contour_chain = (*contour).tail;
    if (*tail).num_points == (*tail).size_points {
        return __cairo_contour_add_point(contour, point);
    }
    let ref mut fresh4 = (*tail).num_points;
    let fresh5 = *fresh4;
    *fresh4 = *fresh4 + 1;
    *((*tail).points).offset(fresh5 as isize) = *point;
    return CAIRO_INT_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_init(
    mut contour: *mut cairo_contour_t,
    mut direction: libc::c_int,
) {
    (*contour).direction = direction;
    let ref mut fresh6 = (*contour).chain.points;
    *fresh6 = ((*contour).embedded_points).as_mut_ptr();
    let ref mut fresh7 = (*contour).chain.next;
    *fresh7 = 0 as *mut _cairo_contour_chain;
    (*contour).chain.num_points = 0 as libc::c_int;
    (*contour)
        .chain
        .size_points = (::std::mem::size_of::<[cairo_point_t; 64]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong)
        as libc::c_int;
    let ref mut fresh8 = (*contour).tail;
    *fresh8 = &mut (*contour).chain;
}
#[no_mangle]
pub unsafe extern "C" fn __cairo_contour_add_point(
    mut contour: *mut cairo_contour_t,
    mut point: *const cairo_point_t,
) -> cairo_int_status_t {
    let mut tail: *mut cairo_contour_chain_t = (*contour).tail;
    let mut next: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    if ((*tail).next).is_null() {} else {
        __assert_fail(
            b"tail->next == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-contour.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"cairo_int_status_t __cairo_contour_add_point(cairo_contour_t *, const cairo_point_t *)\0",
            ))
                .as_ptr(),
        );
    }
    next = _cairo_malloc_ab_plus_c(
        ((*tail).size_points * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
        ::std::mem::size_of::<cairo_contour_chain_t>() as libc::c_ulong,
    ) as *mut cairo_contour_chain_t;
    if next.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    (*next).size_points = (*tail).size_points * 2 as libc::c_int;
    (*next).num_points = 1 as libc::c_int;
    let ref mut fresh9 = (*next).points;
    *fresh9 = next.offset(1 as libc::c_int as isize) as *mut cairo_point_t;
    let ref mut fresh10 = (*next).next;
    *fresh10 = 0 as *mut _cairo_contour_chain;
    let ref mut fresh11 = (*tail).next;
    *fresh11 = next;
    let ref mut fresh12 = (*contour).tail;
    *fresh12 = next;
    *((*next).points).offset(0 as libc::c_int as isize) = *point;
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn first_inc(
    mut contour: *mut cairo_contour_t,
    mut p: *mut *mut cairo_point_t,
    mut chain: *mut *mut cairo_contour_chain_t,
) {
    if *p == ((**chain).points).offset((**chain).num_points as isize) {
        if !((**chain).next).is_null() {} else {
            __assert_fail(
                b"(*chain)->next\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-contour.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void first_inc(cairo_contour_t *, cairo_point_t **, cairo_contour_chain_t **)\0",
                ))
                    .as_ptr(),
            );
        }
        *chain = (**chain).next;
        *p = &mut *((**chain).points).offset(0 as libc::c_int as isize)
            as *mut cairo_point_t;
    } else {
        *p = (*p).offset(1);
    };
}
unsafe extern "C" fn last_dec(
    mut contour: *mut cairo_contour_t,
    mut p: *mut *mut cairo_point_t,
    mut chain: *mut *mut cairo_contour_chain_t,
) {
    if *p == (**chain).points {
        let mut prev: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
        if *chain != &mut (*contour).chain as *mut cairo_contour_chain_t {} else {
            __assert_fail(
                b"*chain != &contour->chain\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-contour.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"void last_dec(cairo_contour_t *, cairo_point_t **, cairo_contour_chain_t **)\0",
                ))
                    .as_ptr(),
            );
        }
        prev = &mut (*contour).chain;
        while (*prev).next != *chain {
            prev = (*prev).next;
        }
        *chain = prev;
        *p = &mut *((**chain).points)
            .offset(((**chain).num_points - 1 as libc::c_int) as isize)
            as *mut cairo_point_t;
    } else {
        *p = (*p).offset(-1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_reverse(mut contour: *mut cairo_contour_t) {
    let mut first_chain: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    let mut last_chain: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    let mut first: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut last: *mut cairo_point_t = 0 as *mut cairo_point_t;
    (*contour).direction = -(*contour).direction;
    if (*contour).chain.num_points <= 1 as libc::c_int {
        return;
    }
    first_chain = &mut (*contour).chain;
    last_chain = (*contour).tail;
    first = &mut *((*first_chain).points).offset(0 as libc::c_int as isize)
        as *mut cairo_point_t;
    last = &mut *((*last_chain).points)
        .offset(((*last_chain).num_points - 1 as libc::c_int) as isize)
        as *mut cairo_point_t;
    while first != last {
        let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        p = *first;
        *first = *last;
        *last = p;
        first_inc(contour, &mut first, &mut first_chain);
        last_dec(contour, &mut last, &mut last_chain);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_add(
    mut dst: *mut cairo_contour_t,
    mut src: *const cairo_contour_t,
) -> cairo_int_status_t {
    let mut chain: *const cairo_contour_chain_t = 0 as *const cairo_contour_chain_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    chain = &(*src).chain;
    while !chain.is_null() {
        i = 0 as libc::c_int;
        while i < (*chain).num_points {
            status = _cairo_contour_add_point(
                dst,
                &mut *((*chain).points).offset(i as isize),
            );
            if status as u64 != 0 {
                return status;
            }
            i += 1;
        }
        chain = (*chain).next;
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn iter_next(mut iter: *mut cairo_contour_iter_t) -> cairo_bool_t {
    if (*iter).point
        == &mut *((*(*iter).chain).points)
            .offset(((*(*iter).chain).size_points - 1 as libc::c_int) as isize)
            as *mut cairo_point_t
    {
        let ref mut fresh13 = (*iter).chain;
        *fresh13 = (*(*iter).chain).next;
        if ((*iter).chain).is_null() {
            return 0 as libc::c_int;
        }
        let ref mut fresh14 = (*iter).point;
        *fresh14 = &mut *((*(*iter).chain).points).offset(0 as libc::c_int as isize)
            as *mut cairo_point_t;
        return 1 as libc::c_int;
    } else {
        let ref mut fresh15 = (*iter).point;
        *fresh15 = (*fresh15).offset(1);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn iter_equal(
    mut i1: *const cairo_contour_iter_t,
    mut i2: *const cairo_contour_iter_t,
) -> cairo_bool_t {
    return ((*i1).chain == (*i2).chain && (*i1).point == (*i2).point) as libc::c_int;
}
unsafe extern "C" fn iter_init(
    mut iter: *mut cairo_contour_iter_t,
    mut contour: *mut cairo_contour_t,
) {
    let ref mut fresh16 = (*iter).chain;
    *fresh16 = &mut (*contour).chain;
    let ref mut fresh17 = (*iter).point;
    *fresh17 = &mut *((*contour).chain.points).offset(0 as libc::c_int as isize)
        as *mut cairo_point_t;
}
unsafe extern "C" fn iter_init_last(
    mut iter: *mut cairo_contour_iter_t,
    mut contour: *mut cairo_contour_t,
) {
    let ref mut fresh18 = (*iter).chain;
    *fresh18 = (*contour).tail;
    let ref mut fresh19 = (*iter).point;
    *fresh19 = &mut *((*(*contour).tail).points)
        .offset(((*(*contour).tail).num_points - 1 as libc::c_int) as isize)
        as *mut cairo_point_t;
}
unsafe extern "C" fn prev_const_chain(
    mut contour: *const cairo_contour_t,
    mut chain: *const cairo_contour_chain_t,
) -> *const cairo_contour_chain_t {
    let mut prev: *const cairo_contour_chain_t = 0 as *const cairo_contour_chain_t;
    if chain == &(*contour).chain as *const cairo_contour_chain_t {
        return 0 as *const cairo_contour_chain_t;
    }
    prev = &(*contour).chain;
    while (*prev).next != chain as *mut _cairo_contour_chain {
        prev = (*prev).next;
    }
    return prev;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_add_reversed(
    mut dst: *mut cairo_contour_t,
    mut src: *const cairo_contour_t,
) -> cairo_int_status_t {
    let mut last: *const cairo_contour_chain_t = 0 as *const cairo_contour_chain_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if (*src).chain.num_points == 0 as libc::c_int {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    last = (*src).tail;
    while !last.is_null() {
        i = (*last).num_points - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            status = _cairo_contour_add_point(
                dst,
                &mut *((*last).points).offset(i as isize),
            );
            if status as u64 != 0 {
                return status;
            }
            i -= 1;
        }
        last = prev_const_chain(src, last);
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn point_distance_sq(
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) -> cairo_uint64_t {
    let mut dx: int32_t = (*p1).x - (*p2).x;
    let mut dy: int32_t = (*p1).y - (*p2).y;
    return (dx as int64_t * dx as libc::c_long + dy as int64_t * dy as libc::c_long)
        as cairo_uint64_t;
}
unsafe extern "C" fn _cairo_contour_simplify_chain(
    mut contour: *mut cairo_contour_t,
    tolerance: libc::c_double,
    mut first: *const cairo_contour_iter_t,
    mut last: *const cairo_contour_iter_t,
) -> cairo_bool_t {
    let mut iter: cairo_contour_iter_t = cairo_contour_iter_t {
        point: 0 as *mut cairo_point_t,
        chain: 0 as *mut cairo_contour_chain_t,
    };
    let mut furthest: cairo_contour_iter_t = cairo_contour_iter_t {
        point: 0 as *mut cairo_point_t,
        chain: 0 as *mut cairo_contour_chain_t,
    };
    let mut max_error: uint64_t = 0;
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    iter = *first;
    iter_next(&mut iter);
    if iter_equal(&mut iter, last) != 0 {
        return 0 as libc::c_int;
    }
    x0 = (*(*first).point).x;
    y0 = (*(*first).point).y;
    nx = (*(*last).point).y - y0;
    ny = x0 - (*(*last).point).x;
    count = 0 as libc::c_int;
    max_error = 0 as libc::c_int as uint64_t;
    loop {
        let mut p: *mut cairo_point_t = iter.point;
        if !((*p).x == -(2147483647 as libc::c_int) - 1 as libc::c_int
            && (*p).y == 2147483647 as libc::c_int)
        {
            let mut d: uint64_t = (nx as uint64_t)
                .wrapping_mul((x0 - (*p).x) as libc::c_ulong)
                .wrapping_add(
                    (ny as uint64_t).wrapping_mul((y0 - (*p).y) as libc::c_ulong),
                );
            if d.wrapping_mul(d) > max_error {
                max_error = d.wrapping_mul(d);
                furthest = iter;
            }
            count += 1;
        }
        iter_next(&mut iter);
        if !(iter_equal(&mut iter, last) == 0) {
            break;
        }
    }
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if max_error as libc::c_double
        > tolerance
            * (nx as uint64_t)
                .wrapping_mul(nx as libc::c_ulong)
                .wrapping_add((ny as uint64_t).wrapping_mul(ny as libc::c_ulong))
                as libc::c_double
    {
        let mut simplified: cairo_bool_t = 0;
        simplified = 0 as libc::c_int;
        simplified
            |= _cairo_contour_simplify_chain(contour, tolerance, first, &mut furthest);
        simplified
            |= _cairo_contour_simplify_chain(contour, tolerance, &mut furthest, last);
        return simplified;
    } else {
        iter = *first;
        iter_next(&mut iter);
        loop {
            (*iter.point).x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            (*iter.point).y = 2147483647 as libc::c_int;
            iter_next(&mut iter);
            if !(iter_equal(&mut iter, last) == 0) {
                break;
            }
        }
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_simplify(
    mut contour: *mut cairo_contour_t,
    mut tolerance: libc::c_double,
) {
    let mut chain: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    let mut last: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut iter: cairo_contour_iter_t = cairo_contour_iter_t {
        point: 0 as *mut cairo_point_t,
        chain: 0 as *mut cairo_contour_chain_t,
    };
    let mut furthest: cairo_contour_iter_t = cairo_contour_iter_t {
        point: 0 as *mut cairo_point_t,
        chain: 0 as *mut cairo_contour_chain_t,
    };
    let mut simplified: cairo_bool_t = 0;
    let mut max: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_int = 0;
    if (*contour).chain.num_points <= 2 as libc::c_int {
        return;
    }
    tolerance = tolerance * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
    tolerance *= tolerance;
    chain = &mut (*contour).chain;
    while !chain.is_null() {
        i = 0 as libc::c_int;
        while i < (*chain).num_points {
            if last.is_null()
                || point_distance_sq(last, &mut *((*chain).points).offset(i as isize))
                    as libc::c_double > tolerance
            {
                last = &mut *((*chain).points).offset(i as isize) as *mut cairo_point_t;
            } else {
                (*((*chain).points).offset(i as isize))
                    .x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
                (*((*chain).points).offset(i as isize)).y = 2147483647 as libc::c_int;
            }
            i += 1;
        }
        chain = (*chain).next;
    }
    loop {
        last = &mut *((*contour).chain.points).offset(0 as libc::c_int as isize)
            as *mut cairo_point_t;
        iter_init(&mut furthest, contour);
        max = 0 as libc::c_int as uint64_t;
        chain = &mut (*contour).chain;
        while !chain.is_null() {
            i = 0 as libc::c_int;
            while i < (*chain).num_points {
                let mut d: uint64_t = 0;
                if !((*((*chain).points).offset(i as isize)).x
                    == -(2147483647 as libc::c_int) - 1 as libc::c_int
                    && (*((*chain).points).offset(i as isize)).y
                        == 2147483647 as libc::c_int)
                {
                    d = point_distance_sq(
                        last,
                        &mut *((*chain).points).offset(i as isize),
                    );
                    if d > max {
                        furthest.chain = chain;
                        furthest
                            .point = &mut *((*chain).points).offset(i as isize)
                            as *mut cairo_point_t;
                        max = d;
                    }
                }
                i += 1;
            }
            chain = (*chain).next;
        }
        if max != 0 {} else {
            __assert_fail(
                b"max\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-contour.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void _cairo_contour_simplify(cairo_contour_t *, double)\0"))
                    .as_ptr(),
            );
        }
        simplified = 0 as libc::c_int;
        iter_init(&mut iter, contour);
        simplified
            |= _cairo_contour_simplify_chain(
                contour,
                tolerance,
                &mut iter,
                &mut furthest,
            );
        iter_init_last(&mut iter, contour);
        if iter_equal(&mut furthest, &mut iter) == 0 {
            simplified
                |= _cairo_contour_simplify_chain(
                    contour,
                    tolerance,
                    &mut furthest,
                    &mut iter,
                );
        }
        if !(simplified != 0) {
            break;
        }
    }
    iter_init(&mut iter, contour);
    chain = &mut (*contour).chain;
    while !chain.is_null() {
        let mut num_points: libc::c_int = (*chain).num_points;
        (*chain).num_points = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < num_points {
            if !((*((*chain).points).offset(i as isize)).x
                == -(2147483647 as libc::c_int) - 1 as libc::c_int
                && (*((*chain).points).offset(i as isize)).y
                    == 2147483647 as libc::c_int)
            {
                if iter.point
                    != &mut *((*chain).points).offset(i as isize) as *mut cairo_point_t
                {
                    *iter.point = *((*chain).points).offset(i as isize);
                }
                let ref mut fresh20 = (*iter.chain).num_points;
                *fresh20 += 1;
                iter_next(&mut iter);
            }
            i += 1;
        }
        chain = (*chain).next;
    }
    if !(iter.chain).is_null() {
        let mut next: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
        chain = (*iter.chain).next;
        while !chain.is_null() {
            next = (*chain).next;
            free(chain as *mut libc::c_void);
            chain = next;
        }
        let ref mut fresh21 = (*iter.chain).next;
        *fresh21 = 0 as *mut _cairo_contour_chain;
        let ref mut fresh22 = (*contour).tail;
        *fresh22 = iter.chain;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_reset(mut contour: *mut cairo_contour_t) {
    _cairo_contour_fini(contour);
    _cairo_contour_init(contour, (*contour).direction);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_contour_fini(mut contour: *mut cairo_contour_t) {
    let mut chain: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    let mut next: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    chain = (*contour).chain.next;
    while !chain.is_null() {
        next = (*chain).next;
        free(chain as *mut libc::c_void);
        chain = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_contour(
    mut file: *mut FILE,
    mut contour: *mut cairo_contour_t,
) {
    let mut chain: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    let mut num_points: libc::c_int = 0;
    let mut size_points: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_points = 0 as libc::c_int;
    size_points = 0 as libc::c_int;
    chain = &mut (*contour).chain;
    while !chain.is_null() {
        num_points += (*chain).num_points;
        size_points += (*chain).size_points;
        chain = (*chain).next;
    }
    fprintf(
        file,
        b"contour: direction=%d, num_points=%d / %d\n\0" as *const u8
            as *const libc::c_char,
        (*contour).direction,
        num_points,
        size_points,
    );
    num_points = 0 as libc::c_int;
    chain = &mut (*contour).chain;
    while !chain.is_null() {
        i = 0 as libc::c_int;
        while i < (*chain).num_points {
            let fresh23 = num_points;
            num_points = num_points + 1;
            fprintf(
                file,
                b"  [%d] = (%f, %f)\n\0" as *const u8 as *const libc::c_char,
                fresh23,
                _cairo_fixed_to_double((*((*chain).points).offset(i as isize)).x),
                _cairo_fixed_to_double((*((*chain).points).offset(i as isize)).y),
            );
            i += 1;
        }
        chain = (*chain).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn __cairo_contour_remove_last_chain(
    mut contour: *mut cairo_contour_t,
) {
    let mut chain: *mut cairo_contour_chain_t = 0 as *mut cairo_contour_chain_t;
    if (*contour).tail == &mut (*contour).chain as *mut cairo_contour_chain_t {
        return;
    }
    chain = &mut (*contour).chain;
    while (*chain).next != (*contour).tail {
        chain = (*chain).next;
    }
    free((*contour).tail as *mut libc::c_void);
    let ref mut fresh24 = (*contour).tail;
    *fresh24 = chain;
    let ref mut fresh25 = (*chain).next;
    *fresh25 = 0 as *mut _cairo_contour_chain;
}
