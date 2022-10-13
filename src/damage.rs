use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn cairo_region_destroy(region: *mut cairo_region_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_region_create_from_boxes(
        boxes: *const cairo_box_t,
        count: libc::c_int,
    ) -> *mut cairo_region_t;
    fn _cairo_region_get_boxes(
        region: *const cairo_region_t,
        nbox: *mut libc::c_int,
    ) -> *mut cairo_box_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type cairo_damage_t = _cairo_damage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_damage {
    pub status: cairo_status_t,
    pub region: *mut cairo_region_t,
    pub dirty: libc::c_int,
    pub remain: libc::c_int,
    pub chunks: _cairo_damage_chunk,
    pub tail: *mut _cairo_damage_chunk,
    pub boxes: [cairo_box_t; 32],
}
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
pub struct _cairo_damage_chunk {
    pub next: *mut _cairo_damage_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
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
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_int {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
static mut __cairo_damage__nil: cairo_damage_t = {
    let mut init = _cairo_damage {
        status: CAIRO_STATUS_NO_MEMORY,
        region: 0 as *const cairo_region_t as *mut cairo_region_t,
        dirty: 0,
        remain: 0,
        chunks: _cairo_damage_chunk {
            next: 0 as *const _cairo_damage_chunk as *mut _cairo_damage_chunk,
            base: 0 as *const cairo_box_t as *mut cairo_box_t,
            count: 0,
            size: 0,
        },
        tail: 0 as *const _cairo_damage_chunk as *mut _cairo_damage_chunk,
        boxes: [cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        }; 32],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_damage_t {
    let mut status__: cairo_status_t = _cairo_error(status);
    return &__cairo_damage__nil as *const cairo_damage_t as *mut cairo_damage_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_create() -> *mut cairo_damage_t {
    let mut damage: *mut cairo_damage_t = 0 as *mut cairo_damage_t;
    damage = (if ::std::mem::size_of::<cairo_damage_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_damage_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_damage_t;
    if damage.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &__cairo_damage__nil as *const cairo_damage_t as *mut cairo_damage_t;
    }
    (*damage).status = CAIRO_STATUS_SUCCESS;
    let ref mut fresh0 = (*damage).region;
    *fresh0 = 0 as *mut cairo_region_t;
    (*damage).dirty = 0 as libc::c_int;
    let ref mut fresh1 = (*damage).tail;
    *fresh1 = &mut (*damage).chunks;
    let ref mut fresh2 = (*damage).chunks.base;
    *fresh2 = ((*damage).boxes).as_mut_ptr();
    (*damage)
        .chunks
        .size = (::std::mem::size_of::<[cairo_box_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
        as libc::c_int;
    (*damage).chunks.count = 0 as libc::c_int;
    let ref mut fresh3 = (*damage).chunks.next;
    *fresh3 = 0 as *mut _cairo_damage_chunk;
    (*damage).remain = (*damage).chunks.size;
    return damage;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_destroy(mut damage: *mut cairo_damage_t) {
    let mut chunk: *mut _cairo_damage_chunk = 0 as *mut _cairo_damage_chunk;
    let mut next: *mut _cairo_damage_chunk = 0 as *mut _cairo_damage_chunk;
    if damage == &__cairo_damage__nil as *const cairo_damage_t as *mut cairo_damage_t {
        return;
    }
    chunk = (*damage).chunks.next;
    while !chunk.is_null() {
        next = (*chunk).next;
        free(chunk as *mut libc::c_void);
        chunk = next;
    }
    cairo_region_destroy((*damage).region);
    free(damage as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_damage_add_boxes(
    mut damage: *mut cairo_damage_t,
    mut boxes: *const cairo_box_t,
    mut count: libc::c_int,
) -> *mut cairo_damage_t {
    let mut chunk: *mut _cairo_damage_chunk = 0 as *mut _cairo_damage_chunk;
    let mut n: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if damage.is_null() {
        damage = _cairo_damage_create();
    }
    if (*damage).status as u64 != 0 {
        return damage;
    }
    (*damage).dirty += count;
    n = count;
    if n > (*damage).remain {
        n = (*damage).remain;
    }
    memcpy(
        ((*(*damage).tail).base).offset((*(*damage).tail).count as isize)
            as *mut libc::c_void,
        boxes as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong),
    );
    count -= n;
    (*(*damage).tail).count += n;
    (*damage).remain -= n;
    if count == 0 as libc::c_int {
        return damage;
    }
    size = 2 as libc::c_int * (*(*damage).tail).size;
    if size < count {
        size = count + 64 as libc::c_int & !(63 as libc::c_int);
    }
    chunk = (if (::std::mem::size_of::<_cairo_damage_chunk>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
        ) != 0 as libc::c_int as libc::c_ulong
    {
        malloc(
            (::std::mem::size_of::<_cairo_damage_chunk>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
                        .wrapping_mul(size as libc::c_ulong),
                ),
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut _cairo_damage_chunk;
    if chunk.is_null() {
        _cairo_damage_destroy(damage);
        return &__cairo_damage__nil as *const cairo_damage_t as *mut cairo_damage_t;
    }
    let ref mut fresh4 = (*chunk).next;
    *fresh4 = 0 as *mut _cairo_damage_chunk;
    let ref mut fresh5 = (*chunk).base;
    *fresh5 = chunk.offset(1 as libc::c_int as isize) as *mut cairo_box_t;
    (*chunk).size = size;
    (*chunk).count = count;
    let ref mut fresh6 = (*(*damage).tail).next;
    *fresh6 = chunk;
    let ref mut fresh7 = (*damage).tail;
    *fresh7 = chunk;
    memcpy(
        (*(*damage).tail).base as *mut libc::c_void,
        boxes.offset(n as isize) as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong),
    );
    (*damage).remain = size - count;
    return damage;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_add_box(
    mut damage: *mut cairo_damage_t,
    mut box_0: *const cairo_box_t,
) -> *mut cairo_damage_t {
    return _cairo_damage_add_boxes(damage, box_0, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_add_rectangle(
    mut damage: *mut cairo_damage_t,
    mut r: *const cairo_rectangle_int_t,
) -> *mut cairo_damage_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    box_0.p1.x = (*r).x;
    box_0.p1.y = (*r).y;
    box_0.p2.x = (*r).x + (*r).width;
    box_0.p2.y = (*r).y + (*r).height;
    return _cairo_damage_add_boxes(damage, &mut box_0, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_add_region(
    mut damage: *mut cairo_damage_t,
    mut region: *const cairo_region_t,
) -> *mut cairo_damage_t {
    let mut boxes: *mut cairo_box_t = 0 as *mut cairo_box_t;
    let mut nbox: libc::c_int = 0;
    boxes = _cairo_region_get_boxes(region, &mut nbox);
    return _cairo_damage_add_boxes(damage, boxes, nbox);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_damage_reduce(
    mut damage: *mut cairo_damage_t,
) -> *mut cairo_damage_t {
    let mut free_boxes: *mut cairo_box_t = 0 as *mut cairo_box_t;
    let mut boxes: *mut cairo_box_t = 0 as *mut cairo_box_t;
    let mut b: *mut cairo_box_t = 0 as *mut cairo_box_t;
    let mut chunk: *mut _cairo_damage_chunk = 0 as *mut _cairo_damage_chunk;
    let mut last: *mut _cairo_damage_chunk = 0 as *mut _cairo_damage_chunk;
    if damage.is_null() || (*damage).status as libc::c_uint != 0 || (*damage).dirty == 0
    {
        return damage;
    }
    if !((*damage).region).is_null() {
        let mut region: *mut cairo_region_t = 0 as *mut cairo_region_t;
        region = (*damage).region;
        let ref mut fresh8 = (*damage).region;
        *fresh8 = 0 as *mut cairo_region_t;
        damage = _cairo_damage_add_region(damage, region);
        cairo_region_destroy(region);
        if (*damage).status as u64 != 0 {
            return damage;
        }
    }
    boxes = (*(*damage).tail).base;
    if (*damage).dirty > (*(*damage).tail).size {
        free_boxes = (if ((*damage).dirty as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(
                ((*damage).dirty as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong),
            )
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_box_t;
        boxes = free_boxes;
        if boxes.is_null() {
            _cairo_damage_destroy(damage);
            return &__cairo_damage__nil as *const cairo_damage_t as *mut cairo_damage_t;
        }
        b = boxes;
        last = 0 as *mut _cairo_damage_chunk;
    } else {
        b = boxes.offset((*(*damage).tail).count as isize);
        last = (*damage).tail;
    }
    chunk = &mut (*damage).chunks;
    while chunk != last {
        memcpy(
            b as *mut libc::c_void,
            (*chunk).base as *const libc::c_void,
            ((*chunk).count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong),
        );
        b = b.offset((*chunk).count as isize);
        chunk = (*chunk).next;
    }
    let ref mut fresh9 = (*damage).region;
    *fresh9 = _cairo_region_create_from_boxes(boxes, (*damage).dirty);
    free(free_boxes as *mut libc::c_void);
    if (*(*damage).region).status as u64 != 0 {
        _cairo_damage_destroy(damage);
        return &__cairo_damage__nil as *const cairo_damage_t as *mut cairo_damage_t;
    }
    (*damage).dirty = 0 as libc::c_int;
    return damage;
}
