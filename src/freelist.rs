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
}
pub type __uint8_t = libc::c_uchar;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub nodesize: libc::c_uint,
}
pub type cairo_freelist_t = _cairo_freelist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_pool {
    pub next: *mut cairo_freelist_pool_t,
    pub size: libc::c_uint,
    pub rem: libc::c_uint,
    pub data: *mut uint8_t,
}
pub type cairo_freelist_pool_t = _cairo_freelist_pool;
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
pub type cairo_freepool_t = _cairo_freepool;
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
    let ref mut fresh0 = (*pool).data;
    *fresh0 = (*fresh0).offset((*freepool).nodesize as isize);
    let ref mut fresh1 = (*pool).rem;
    *fresh1 = (*fresh1).wrapping_sub((*freepool).nodesize);
    return ptr as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_free(
    mut freepool: *mut cairo_freepool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut node: *mut cairo_freelist_node_t = ptr as *mut cairo_freelist_node_t;
    let ref mut fresh2 = (*node).next;
    *fresh2 = (*freepool).first_free_node;
    let ref mut fresh3 = (*freepool).first_free_node;
    *fresh3 = node;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freelist_init(
    mut freelist: *mut cairo_freelist_t,
    mut nodesize: libc::c_uint,
) {
    memset(
        freelist as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_freelist_t>() as libc::c_ulong,
    );
    (*freelist).nodesize = nodesize;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freelist_fini(mut freelist: *mut cairo_freelist_t) {
    let mut node: *mut cairo_freelist_node_t = (*freelist).first_free_node;
    while !node.is_null() {
        let mut next: *mut cairo_freelist_node_t = 0 as *mut cairo_freelist_node_t;
        next = (*node).next;
        free(node as *mut libc::c_void);
        node = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freelist_alloc(
    mut freelist: *mut cairo_freelist_t,
) -> *mut libc::c_void {
    if !((*freelist).first_free_node).is_null() {
        let mut node: *mut cairo_freelist_node_t = 0 as *mut cairo_freelist_node_t;
        node = (*freelist).first_free_node;
        let ref mut fresh4 = (*freelist).first_free_node;
        *fresh4 = (*node).next;
        return node as *mut libc::c_void;
    }
    return if (*freelist).nodesize != 0 as libc::c_int as libc::c_uint {
        malloc((*freelist).nodesize as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freelist_calloc(
    mut freelist: *mut cairo_freelist_t,
) -> *mut libc::c_void {
    let mut node: *mut libc::c_void = _cairo_freelist_alloc(freelist);
    if !node.is_null() {
        memset(node, 0 as libc::c_int, (*freelist).nodesize as libc::c_ulong);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freelist_free(
    mut freelist: *mut cairo_freelist_t,
    mut voidnode: *mut libc::c_void,
) {
    let mut node: *mut cairo_freelist_node_t = voidnode as *mut cairo_freelist_node_t;
    if !node.is_null() {
        let ref mut fresh5 = (*node).next;
        *fresh5 = (*freelist).first_free_node;
        let ref mut fresh6 = (*freelist).first_free_node;
        *fresh6 = node;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freepool_init(
    mut freepool: *mut cairo_freepool_t,
    mut nodesize: libc::c_uint,
) {
    let ref mut fresh7 = (*freepool).first_free_node;
    *fresh7 = 0 as *mut cairo_freelist_node_t;
    let ref mut fresh8 = (*freepool).pools;
    *fresh8 = &mut (*freepool).embedded_pool;
    let ref mut fresh9 = (*freepool).freepools;
    *fresh9 = 0 as *mut cairo_freelist_pool_t;
    (*freepool).nodesize = nodesize;
    let ref mut fresh10 = (*freepool).embedded_pool.next;
    *fresh10 = 0 as *mut cairo_freelist_pool_t;
    (*freepool)
        .embedded_pool
        .size = ::std::mem::size_of::<[uint8_t; 1000]>() as libc::c_ulong
        as libc::c_uint;
    (*freepool)
        .embedded_pool
        .rem = ::std::mem::size_of::<[uint8_t; 1000]>() as libc::c_ulong as libc::c_uint;
    let ref mut fresh11 = (*freepool).embedded_pool.data;
    *fresh11 = ((*freepool).embedded_data).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freepool_fini(mut freepool: *mut cairo_freepool_t) {
    let mut pool: *mut cairo_freelist_pool_t = 0 as *mut cairo_freelist_pool_t;
    pool = (*freepool).pools;
    while pool != &mut (*freepool).embedded_pool as *mut cairo_freelist_pool_t {
        let mut next: *mut cairo_freelist_pool_t = (*pool).next;
        free(pool as *mut libc::c_void);
        pool = next;
    }
    pool = (*freepool).freepools;
    while !pool.is_null() {
        let mut next_0: *mut cairo_freelist_pool_t = (*pool).next;
        free(pool as *mut libc::c_void);
        pool = next_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freepool_alloc_from_new_pool(
    mut freepool: *mut cairo_freepool_t,
) -> *mut libc::c_void {
    let mut pool: *mut cairo_freelist_pool_t = 0 as *mut cairo_freelist_pool_t;
    let mut poolsize: libc::c_int = 0;
    if !((*freepool).freepools).is_null() {
        pool = (*freepool).freepools;
        let ref mut fresh12 = (*freepool).freepools;
        *fresh12 = (*pool).next;
        poolsize = (*pool).size as libc::c_int;
    } else {
        if (*freepool).pools
            != &mut (*freepool).embedded_pool as *mut cairo_freelist_pool_t
        {
            poolsize = (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*(*freepool).pools).size) as libc::c_int;
        } else {
            poolsize = ((128 as libc::c_int as libc::c_uint)
                .wrapping_mul((*freepool).nodesize)
                .wrapping_add(8191 as libc::c_int as libc::c_uint)
                & -(8192 as libc::c_int) as libc::c_uint) as libc::c_int;
        }
        pool = (if (::std::mem::size_of::<cairo_freelist_pool_t>() as libc::c_ulong)
            .wrapping_add(poolsize as libc::c_ulong) != 0 as libc::c_int as libc::c_ulong
        {
            malloc(
                (::std::mem::size_of::<cairo_freelist_pool_t>() as libc::c_ulong)
                    .wrapping_add(poolsize as libc::c_ulong),
            )
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_freelist_pool_t;
        if pool.is_null() {
            return pool as *mut libc::c_void;
        }
        (*pool).size = poolsize as libc::c_uint;
    }
    let ref mut fresh13 = (*pool).next;
    *fresh13 = (*freepool).pools;
    let ref mut fresh14 = (*freepool).pools;
    *fresh14 = pool;
    (*pool).rem = (poolsize as libc::c_uint).wrapping_sub((*freepool).nodesize);
    let ref mut fresh15 = (*pool).data;
    *fresh15 = (pool.offset(1 as libc::c_int as isize) as *mut uint8_t)
        .offset((*freepool).nodesize as isize);
    return pool.offset(1 as libc::c_int as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_freepool_alloc_array(
    mut freepool: *mut cairo_freepool_t,
    mut count: libc::c_int,
    mut array: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    loop {
        if !(i < count) {
            current_block = 1841672684692190573;
            break;
        }
        let mut node: *mut cairo_freelist_node_t = 0 as *mut cairo_freelist_node_t;
        node = (*freepool).first_free_node;
        if !node.is_null() {
            let ref mut fresh16 = (*freepool).first_free_node;
            *fresh16 = (*node).next;
        } else {
            node = _cairo_freepool_alloc_from_pool(freepool)
                as *mut cairo_freelist_node_t;
            if node.is_null() {
                current_block = 5423986621528524350;
                break;
            }
        }
        let ref mut fresh17 = *array.offset(i as isize);
        *fresh17 = node as *mut libc::c_void;
        i += 1;
    }
    match current_block {
        1841672684692190573 => return CAIRO_STATUS_SUCCESS,
        _ => {
            loop {
                let fresh18 = i;
                i = i - 1;
                if !(fresh18 != 0) {
                    break;
                }
                _cairo_freepool_free(freepool, *array.offset(i as isize));
            }
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    };
}
