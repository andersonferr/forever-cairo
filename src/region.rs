use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn pixman_region32_init(region: *mut pixman_region32_t);
    fn pixman_region32_init_rect(
        region: *mut pixman_region32_t,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pixman_region32_init_rects(
        region: *mut pixman_region32_t,
        boxes: *const pixman_box32_t,
        count: libc::c_int,
    ) -> pixman_bool_t;
    fn pixman_region32_fini(region: *mut pixman_region32_t);
    fn pixman_region32_copy(
        dest: *mut pixman_region32_t,
        source: *mut pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_equal(
        region1: *mut pixman_region32_t,
        region2: *mut pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_extents(region: *mut pixman_region32_t) -> *mut pixman_box32_t;
    fn pixman_region32_n_rects(region: *mut pixman_region32_t) -> libc::c_int;
    fn pixman_region32_rectangles(
        region: *mut pixman_region32_t,
        n_rects: *mut libc::c_int,
    ) -> *mut pixman_box32_t;
    fn pixman_region32_not_empty(region: *mut pixman_region32_t) -> pixman_bool_t;
    fn pixman_region32_contains_rectangle(
        region: *mut pixman_region32_t,
        prect: *mut pixman_box32_t,
    ) -> pixman_region_overlap_t;
    fn pixman_region32_contains_point(
        region: *mut pixman_region32_t,
        x: libc::c_int,
        y: libc::c_int,
        box_0: *mut pixman_box32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_translate(
        region: *mut pixman_region32_t,
        x: libc::c_int,
        y: libc::c_int,
    );
    fn pixman_region32_subtract(
        reg_d: *mut pixman_region32_t,
        reg_m: *mut pixman_region32_t,
        reg_s: *mut pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_intersect(
        new_reg: *mut pixman_region32_t,
        reg1: *mut pixman_region32_t,
        reg2: *mut pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_region32_union(
        new_reg: *mut pixman_region32_t,
        reg1: *mut pixman_region32_t,
        reg2: *mut pixman_region32_t,
    ) -> pixman_bool_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_reference_count_t {
    pub ref_count: cairo_atomic_int_t,
}
pub type cairo_atomic_int_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_region {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub rgn: pixman_region32_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
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
pub type _cairo_region_overlap = libc::c_uint;
pub const CAIRO_REGION_OVERLAP_PART: _cairo_region_overlap = 2;
pub const CAIRO_REGION_OVERLAP_OUT: _cairo_region_overlap = 1;
pub const CAIRO_REGION_OVERLAP_IN: _cairo_region_overlap = 0;
pub type cairo_region_overlap_t = _cairo_region_overlap;
pub type pixman_bool_t = libc::c_int;
pub const PIXMAN_REGION_PART: pixman_region_overlap_t = 2;
pub const PIXMAN_REGION_IN: pixman_region_overlap_t = 1;
pub const PIXMAN_REGION_OUT: pixman_region_overlap_t = 0;
pub type pixman_region_overlap_t = libc::c_uint;
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
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_cmpxchg_impl(
    mut x: *mut cairo_atomic_int_t,
    mut oldv: cairo_atomic_int_t,
    mut newv: cairo_atomic_int_t,
) -> cairo_bool_t {
    let mut expected: cairo_atomic_int_t = oldv;
    let fresh2 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh2.0;
    return fresh2.1 as cairo_bool_t;
}
static mut _cairo_region_nil: cairo_region_t = {
    let mut init = _cairo_region {
        ref_count: {
            let mut init = cairo_reference_count_t {
                ref_count: -(1 as libc::c_int),
            };
            init
        },
        status: CAIRO_STATUS_NO_MEMORY,
        rgn: pixman_region32_t {
            extents: pixman_box32_t {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            },
            data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
        },
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_region_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_region_t {
    match status as libc::c_uint {
        1 => return &_cairo_region_nil as *const cairo_region_t as *mut cairo_region_t,
        0 | 44 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-region.c\0" as *const u8 as *const libc::c_char,
                    71 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"cairo_region_t *_cairo_region_create_in_error(cairo_status_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        13 | 6 | 15 | 16 | 17 | 10 | 11 | 18 | 23 | 24 | 32 | 34 | 35 | 2 | 3 | 4 | 5 | 7
        | 8 | 9 | 12 | 14 | 19 | 20 | 21 | 22 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 33
        | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | _ => {}
    }
    let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    return &_cairo_region_nil as *const cairo_region_t as *mut cairo_region_t;
}
unsafe extern "C" fn _cairo_region_set_error(
    mut region: *mut cairo_region_t,
    mut status: cairo_status_t,
) -> cairo_status_t {
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    let mut ret__: libc::c_int = 0;
    if (status as libc::c_uint) < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status < CAIRO_STATUS_LAST_STATUS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-region.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"cairo_status_t _cairo_region_set_error(cairo_region_t *, cairo_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    ret__ = _cairo_atomic_int_cmpxchg_impl(
        &mut (*region).status as *mut cairo_status_t as *mut cairo_atomic_int_t,
        CAIRO_STATUS_SUCCESS as libc::c_int,
        status as cairo_atomic_int_t,
    );
    return _cairo_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_region_init(mut region: *mut cairo_region_t) {
    (*region).status = CAIRO_STATUS_SUCCESS;
    (*region).ref_count.ref_count = 0 as libc::c_int;
    pixman_region32_init(&mut (*region).rgn);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_region_init_rectangle(
    mut region: *mut cairo_region_t,
    mut rectangle: *const cairo_rectangle_int_t,
) {
    (*region).status = CAIRO_STATUS_SUCCESS;
    (*region).ref_count.ref_count = 0 as libc::c_int;
    pixman_region32_init_rect(
        &mut (*region).rgn,
        (*rectangle).x,
        (*rectangle).y,
        (*rectangle).width as libc::c_uint,
        (*rectangle).height as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_region_fini(mut region: *mut cairo_region_t) {
    if !(_cairo_atomic_int_get(&mut (*region).ref_count.ref_count) > 0 as libc::c_int)
    {} else {
        __assert_fail(
            b"! CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&region->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-region.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void _cairo_region_fini(cairo_region_t *)\0"))
                .as_ptr(),
        );
    }
    pixman_region32_fini(&mut (*region).rgn);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_create() -> *mut cairo_region_t {
    let mut region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    region = (if ::std::mem::size_of::<cairo_region_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_region_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_region_t;
    if region.is_null() {
        return &_cairo_region_nil as *const cairo_region_t as *mut cairo_region_t;
    }
    (*region).status = CAIRO_STATUS_SUCCESS;
    (*region).ref_count.ref_count = 1 as libc::c_int;
    pixman_region32_init(&mut (*region).rgn);
    return region;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_create_rectangles(
    mut rects: *const cairo_rectangle_int_t,
    mut count: libc::c_int,
) -> *mut cairo_region_t {
    let mut stack_pboxes: [pixman_box32_t; 128] = [pixman_box32_t {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    }; 128];
    let mut pboxes: *mut pixman_box32_t = stack_pboxes.as_mut_ptr();
    let mut region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    let mut i: libc::c_int = 0;
    region = (if ::std::mem::size_of::<cairo_region_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_region_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_region_t;
    if region.is_null() {
        return _cairo_region_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    (*region).ref_count.ref_count = 1 as libc::c_int;
    (*region).status = CAIRO_STATUS_SUCCESS;
    if count == 1 as libc::c_int {
        pixman_region32_init_rect(
            &mut (*region).rgn,
            (*rects).x,
            (*rects).y,
            (*rects).width as libc::c_uint,
            (*rects).height as libc::c_uint,
        );
        return region;
    }
    if count
        > (::std::mem::size_of::<[pixman_box32_t; 128]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pixman_box32_t>() as libc::c_ulong)
            as libc::c_int
    {
        pboxes = _cairo_malloc_ab(
            count as size_t,
            ::std::mem::size_of::<pixman_box32_t>() as libc::c_ulong,
        ) as *mut pixman_box32_t;
        if pboxes.is_null() {
            free(region as *mut libc::c_void);
            return _cairo_region_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
        }
    }
    i = 0 as libc::c_int;
    while i < count {
        (*pboxes.offset(i as isize)).x1 = (*rects.offset(i as isize)).x;
        (*pboxes.offset(i as isize)).y1 = (*rects.offset(i as isize)).y;
        (*pboxes.offset(i as isize))
            .x2 = (*rects.offset(i as isize)).x + (*rects.offset(i as isize)).width;
        (*pboxes.offset(i as isize))
            .y2 = (*rects.offset(i as isize)).y + (*rects.offset(i as isize)).height;
        i += 1;
    }
    i = pixman_region32_init_rects(&mut (*region).rgn, pboxes, count);
    if pboxes != stack_pboxes.as_mut_ptr() {
        free(pboxes as *mut libc::c_void);
    }
    if i == 0 as libc::c_int {
        free(region as *mut libc::c_void);
        return _cairo_region_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    return region;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_region_create_from_boxes(
    mut boxes: *const cairo_box_t,
    mut count: libc::c_int,
) -> *mut cairo_region_t {
    let mut region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    region = (if ::std::mem::size_of::<cairo_region_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_region_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_region_t;
    if region.is_null() {
        return _cairo_region_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    (*region).ref_count.ref_count = 1 as libc::c_int;
    (*region).status = CAIRO_STATUS_SUCCESS;
    if pixman_region32_init_rects(
        &mut (*region).rgn,
        boxes as *mut pixman_box32_t,
        count,
    ) == 0
    {
        free(region as *mut libc::c_void);
        return _cairo_region_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    return region;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_region_get_boxes(
    mut region: *const cairo_region_t,
    mut nbox: *mut libc::c_int,
) -> *mut cairo_box_t {
    if (*region).status as u64 != 0 {
        nbox = 0 as *mut libc::c_int;
        return 0 as *mut cairo_box_t;
    }
    return pixman_region32_rectangles(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        nbox,
    ) as *mut cairo_box_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_create_rectangle(
    mut rectangle: *const cairo_rectangle_int_t,
) -> *mut cairo_region_t {
    let mut region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    region = (if ::std::mem::size_of::<cairo_region_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_region_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_region_t;
    if region.is_null() {
        return &_cairo_region_nil as *const cairo_region_t as *mut cairo_region_t;
    }
    (*region).status = CAIRO_STATUS_SUCCESS;
    (*region).ref_count.ref_count = 1 as libc::c_int;
    pixman_region32_init_rect(
        &mut (*region).rgn,
        (*rectangle).x,
        (*rectangle).y,
        (*rectangle).width as libc::c_uint,
        (*rectangle).height as libc::c_uint,
    );
    return region;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_copy(
    mut original: *const cairo_region_t,
) -> *mut cairo_region_t {
    let mut copy: *mut cairo_region_t = 0 as *mut cairo_region_t;
    if !original.is_null() && (*original).status as libc::c_uint != 0 {
        return &_cairo_region_nil as *const cairo_region_t as *mut cairo_region_t;
    }
    copy = cairo_region_create();
    if (*copy).status as u64 != 0 {
        return copy;
    }
    if !original.is_null()
        && pixman_region32_copy(
            &mut (*copy).rgn,
            &(*original).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        ) == 0
    {
        cairo_region_destroy(copy);
        return &_cairo_region_nil as *const cairo_region_t as *mut cairo_region_t;
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_reference(
    mut region: *mut cairo_region_t,
) -> *mut cairo_region_t {
    if region.is_null()
        || _cairo_atomic_int_get(&mut (*region).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return 0 as *mut cairo_region_t;
    }
    if _cairo_atomic_int_get(&mut (*region).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&region->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-region.c\0" as *const u8 as *const libc::c_char,
            406 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"cairo_region_t *cairo_region_reference(cairo_region_t *)\0"))
                .as_ptr(),
        );
    }
    ::std::intrinsics::atomic_xadd(&mut (*region).ref_count.ref_count, 1 as libc::c_int);
    return region;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_destroy(mut region: *mut cairo_region_t) {
    if region.is_null()
        || _cairo_atomic_int_get(&mut (*region).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return;
    }
    if _cairo_atomic_int_get(&mut (*region).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&region->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-region.c\0" as *const u8 as *const libc::c_char,
            429 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void cairo_region_destroy(cairo_region_t *)\0"))
                .as_ptr(),
        );
    }
    if !(::std::intrinsics::atomic_xsub(
        &mut (*region).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        return;
    }
    _cairo_region_fini(region);
    free(region as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_num_rectangles(
    mut region: *const cairo_region_t,
) -> libc::c_int {
    if (*region).status as u64 != 0 {
        return 0 as libc::c_int;
    }
    return pixman_region32_n_rects(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_get_rectangle(
    mut region: *const cairo_region_t,
    mut nth: libc::c_int,
    mut rectangle: *mut cairo_rectangle_int_t,
) {
    let mut pbox: *mut pixman_box32_t = 0 as *mut pixman_box32_t;
    if (*region).status as u64 != 0 {
        let ref mut fresh3 = (*rectangle).y;
        *fresh3 = 0 as libc::c_int;
        (*rectangle).x = *fresh3;
        let ref mut fresh4 = (*rectangle).height;
        *fresh4 = 0 as libc::c_int;
        (*rectangle).width = *fresh4;
        return;
    }
    pbox = (pixman_region32_rectangles(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        0 as *mut libc::c_int,
    ))
        .offset(nth as isize);
    (*rectangle).x = (*pbox).x1;
    (*rectangle).y = (*pbox).y1;
    (*rectangle).width = (*pbox).x2 - (*pbox).x1;
    (*rectangle).height = (*pbox).y2 - (*pbox).y1;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_get_extents(
    mut region: *const cairo_region_t,
    mut extents: *mut cairo_rectangle_int_t,
) {
    let mut pextents: *mut pixman_box32_t = 0 as *mut pixman_box32_t;
    if (*region).status as u64 != 0 {
        let ref mut fresh5 = (*extents).y;
        *fresh5 = 0 as libc::c_int;
        (*extents).x = *fresh5;
        let ref mut fresh6 = (*extents).height;
        *fresh6 = 0 as libc::c_int;
        (*extents).width = *fresh6;
        return;
    }
    pextents = pixman_region32_extents(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    );
    (*extents).x = (*pextents).x1;
    (*extents).y = (*pextents).y1;
    (*extents).width = (*pextents).x2 - (*pextents).x1;
    (*extents).height = (*pextents).y2 - (*pextents).y1;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_status(
    mut region: *const cairo_region_t,
) -> cairo_status_t {
    return (*region).status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_subtract(
    mut dst: *mut cairo_region_t,
    mut other: *const cairo_region_t,
) -> cairo_status_t {
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    if (*other).status as u64 != 0 {
        return _cairo_region_set_error(dst, (*other).status);
    }
    if pixman_region32_subtract(
        &mut (*dst).rgn,
        &mut (*dst).rgn,
        &(*other).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    ) == 0
    {
        return _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_subtract_rectangle(
    mut dst: *mut cairo_region_t,
    mut rectangle: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut region: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
    };
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    pixman_region32_init_rect(
        &mut region,
        (*rectangle).x,
        (*rectangle).y,
        (*rectangle).width as libc::c_uint,
        (*rectangle).height as libc::c_uint,
    );
    if pixman_region32_subtract(&mut (*dst).rgn, &mut (*dst).rgn, &mut region) == 0 {
        status = _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    pixman_region32_fini(&mut region);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_intersect(
    mut dst: *mut cairo_region_t,
    mut other: *const cairo_region_t,
) -> cairo_status_t {
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    if (*other).status as u64 != 0 {
        return _cairo_region_set_error(dst, (*other).status);
    }
    if pixman_region32_intersect(
        &mut (*dst).rgn,
        &mut (*dst).rgn,
        &(*other).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    ) == 0
    {
        return _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_intersect_rectangle(
    mut dst: *mut cairo_region_t,
    mut rectangle: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut region: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
    };
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    pixman_region32_init_rect(
        &mut region,
        (*rectangle).x,
        (*rectangle).y,
        (*rectangle).width as libc::c_uint,
        (*rectangle).height as libc::c_uint,
    );
    if pixman_region32_intersect(&mut (*dst).rgn, &mut (*dst).rgn, &mut region) == 0 {
        status = _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    pixman_region32_fini(&mut region);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_union(
    mut dst: *mut cairo_region_t,
    mut other: *const cairo_region_t,
) -> cairo_status_t {
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    if (*other).status as u64 != 0 {
        return _cairo_region_set_error(dst, (*other).status);
    }
    if pixman_region32_union(
        &mut (*dst).rgn,
        &mut (*dst).rgn,
        &(*other).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    ) == 0
    {
        return _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_union_rectangle(
    mut dst: *mut cairo_region_t,
    mut rectangle: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut region: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
    };
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    pixman_region32_init_rect(
        &mut region,
        (*rectangle).x,
        (*rectangle).y,
        (*rectangle).width as libc::c_uint,
        (*rectangle).height as libc::c_uint,
    );
    if pixman_region32_union(&mut (*dst).rgn, &mut (*dst).rgn, &mut region) == 0 {
        status = _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    pixman_region32_fini(&mut region);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_xor(
    mut dst: *mut cairo_region_t,
    mut other: *const cairo_region_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut tmp: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
    };
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    if (*other).status as u64 != 0 {
        return _cairo_region_set_error(dst, (*other).status);
    }
    pixman_region32_init(&mut tmp);
    if pixman_region32_subtract(
        &mut tmp,
        &(*other).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        &mut (*dst).rgn,
    ) == 0
        || pixman_region32_subtract(
            &mut (*dst).rgn,
            &mut (*dst).rgn,
            &(*other).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        ) == 0 || pixman_region32_union(&mut (*dst).rgn, &mut (*dst).rgn, &mut tmp) == 0
    {
        status = _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    pixman_region32_fini(&mut tmp);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_xor_rectangle(
    mut dst: *mut cairo_region_t,
    mut rectangle: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut region: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
    };
    let mut tmp: pixman_region32_t = pixman_region32_t {
        extents: pixman_box32_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        },
        data: 0 as *const pixman_region32_data_t as *mut pixman_region32_data_t,
    };
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    pixman_region32_init_rect(
        &mut region,
        (*rectangle).x,
        (*rectangle).y,
        (*rectangle).width as libc::c_uint,
        (*rectangle).height as libc::c_uint,
    );
    pixman_region32_init(&mut tmp);
    if pixman_region32_subtract(&mut tmp, &mut region, &mut (*dst).rgn) == 0
        || pixman_region32_subtract(&mut (*dst).rgn, &mut (*dst).rgn, &mut region) == 0
        || pixman_region32_union(&mut (*dst).rgn, &mut (*dst).rgn, &mut tmp) == 0
    {
        status = _cairo_region_set_error(dst, CAIRO_STATUS_NO_MEMORY);
    }
    pixman_region32_fini(&mut tmp);
    pixman_region32_fini(&mut region);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_is_empty(
    mut region: *const cairo_region_t,
) -> cairo_bool_t {
    if (*region).status as u64 != 0 {
        return 1 as libc::c_int;
    }
    return (pixman_region32_not_empty(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    ) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_translate(
    mut region: *mut cairo_region_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) {
    if (*region).status as u64 != 0 {
        return;
    }
    pixman_region32_translate(&mut (*region).rgn, dx, dy);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_contains_rectangle(
    mut region: *const cairo_region_t,
    mut rectangle: *const cairo_rectangle_int_t,
) -> cairo_region_overlap_t {
    let mut pbox: pixman_box32_t = pixman_box32_t {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    };
    let mut poverlap: pixman_region_overlap_t = PIXMAN_REGION_OUT;
    if (*region).status as u64 != 0 {
        return CAIRO_REGION_OVERLAP_OUT;
    }
    pbox.x1 = (*rectangle).x;
    pbox.y1 = (*rectangle).y;
    pbox.x2 = (*rectangle).x + (*rectangle).width;
    pbox.y2 = (*rectangle).y + (*rectangle).height;
    poverlap = pixman_region32_contains_rectangle(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        &mut pbox,
    );
    match poverlap as libc::c_uint {
        1 => return CAIRO_REGION_OVERLAP_IN,
        2 => return CAIRO_REGION_OVERLAP_PART,
        0 | _ => return CAIRO_REGION_OVERLAP_OUT,
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_contains_point(
    mut region: *const cairo_region_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> cairo_bool_t {
    let mut box_0: pixman_box32_t = pixman_box32_t {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    };
    if (*region).status as u64 != 0 {
        return 0 as libc::c_int;
    }
    return pixman_region32_contains_point(
        &(*region).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        x,
        y,
        &mut box_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_region_equal(
    mut a: *const cairo_region_t,
    mut b: *const cairo_region_t,
) -> cairo_bool_t {
    if !a.is_null() && (*a).status as libc::c_uint != 0
        || !b.is_null() && (*b).status as libc::c_uint != 0
    {
        return 0 as libc::c_int;
    }
    if a == b {
        return 1 as libc::c_int;
    }
    if a.is_null() || b.is_null() {
        return 0 as libc::c_int;
    }
    return pixman_region32_equal(
        &(*a).rgn as *const pixman_region32_t as *mut pixman_region32_t,
        &(*b).rgn as *const pixman_region32_t as *mut pixman_region32_t,
    );
}
