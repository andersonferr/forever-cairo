use ::libc;
extern "C" {
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
    fn _cairo_freepool_init(freepool: *mut cairo_freepool_t, nodesize: libc::c_uint);
    fn _cairo_freepool_fini(freepool: *mut cairo_freepool_t);
    fn _cairo_freepool_alloc_from_new_pool(
        freepool: *mut cairo_freepool_t,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
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
pub type C2RustUnnamed = libc::c_uint;
pub const CAIRO_RTREE_NODE_OCCUPIED: C2RustUnnamed = 2;
pub const CAIRO_RTREE_NODE_DIVIDED: C2RustUnnamed = 1;
pub const CAIRO_RTREE_NODE_AVAILABLE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rtree_node {
    pub children: [*mut _cairo_rtree_node; 4],
    pub parent: *mut _cairo_rtree_node,
    pub link: cairo_list_t,
    pub pinned: uint16_t,
    pub state: uint16_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
pub type cairo_rtree_node_t = _cairo_rtree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rtree {
    pub root: cairo_rtree_node_t,
    pub min_size: libc::c_int,
    pub pinned: cairo_list_t,
    pub available: cairo_list_t,
    pub evictable: cairo_list_t,
    pub destroy: Option::<unsafe extern "C" fn(*mut cairo_rtree_node_t) -> ()>,
    pub node_freepool: cairo_freepool_t,
}
pub type cairo_rtree_t = _cairo_rtree;
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
unsafe extern "C" fn _cairo_freepool_alloc(
    mut freepool: *mut cairo_freepool_t,
) -> *mut libc::c_void {
    let mut node: *mut cairo_freelist_node_t = 0 as *mut cairo_freelist_node_t;
    node = (*freepool).first_free_node;
    if node.is_null() {
        return _cairo_freepool_alloc_from_pool(freepool);
    }
    let ref mut fresh2 = (*freepool).first_free_node;
    *fresh2 = (*node).next;
    return node as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_free(
    mut freepool: *mut cairo_freepool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut node: *mut cairo_freelist_node_t = ptr as *mut cairo_freelist_node_t;
    let ref mut fresh3 = (*node).next;
    *fresh3 = (*freepool).first_free_node;
    let ref mut fresh4 = (*freepool).first_free_node;
    *fresh4 = node;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh5 = (*entry).next;
    *fresh5 = entry;
    let ref mut fresh6 = (*entry).prev;
    *fresh6 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh7 = (*next).prev;
    *fresh7 = entry;
    let ref mut fresh8 = (*entry).next;
    *fresh8 = next;
    let ref mut fresh9 = (*entry).prev;
    *fresh9 = prev;
    let ref mut fresh10 = (*prev).next;
    *fresh10 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh11 = (*next).prev;
    *fresh11 = prev;
    let ref mut fresh12 = (*prev).next;
    *fresh12 = next;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn cairo_list_del(mut entry: *mut cairo_list_t) {
    _cairo_list_del(entry);
    cairo_list_init(entry);
}
#[inline]
unsafe extern "C" fn cairo_list_move(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_del((*entry).prev, (*entry).next);
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_node_create(
    mut rtree: *mut cairo_rtree_t,
    mut parent: *mut cairo_rtree_node_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_rtree_node_t {
    let mut node: *mut cairo_rtree_node_t = 0 as *mut cairo_rtree_node_t;
    node = _cairo_freepool_alloc(&mut (*rtree).node_freepool) as *mut cairo_rtree_node_t;
    if node.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_rtree_node_t;
    }
    let ref mut fresh13 = (*node).children[0 as libc::c_int as usize];
    *fresh13 = 0 as *mut _cairo_rtree_node;
    let ref mut fresh14 = (*node).parent;
    *fresh14 = parent;
    (*node).state = CAIRO_RTREE_NODE_AVAILABLE as libc::c_int as uint16_t;
    (*node).pinned = 0 as libc::c_int as uint16_t;
    (*node).x = x as uint16_t;
    (*node).y = y as uint16_t;
    (*node).width = width as uint16_t;
    (*node).height = height as uint16_t;
    cairo_list_add(&mut (*node).link, &mut (*rtree).available);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_node_destroy(
    mut rtree: *mut cairo_rtree_t,
    mut node: *mut cairo_rtree_node_t,
) {
    let mut i: libc::c_int = 0;
    cairo_list_del(&mut (*node).link);
    if (*node).state as libc::c_int == CAIRO_RTREE_NODE_OCCUPIED as libc::c_int {
        ((*rtree).destroy).expect("non-null function pointer")(node);
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && !((*node).children[i as usize]).is_null() {
            _cairo_rtree_node_destroy(rtree, (*node).children[i as usize]);
            i += 1;
        }
    }
    _cairo_freepool_free(&mut (*rtree).node_freepool, node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_node_collapse(
    mut rtree: *mut cairo_rtree_t,
    mut node: *mut cairo_rtree_node_t,
) {
    let mut i: libc::c_int = 0;
    loop {
        if (*node).state as libc::c_int == CAIRO_RTREE_NODE_DIVIDED as libc::c_int
        {} else {
            __assert_fail(
                b"node->state == CAIRO_RTREE_NODE_DIVIDED\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-rtree.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"void _cairo_rtree_node_collapse(cairo_rtree_t *, cairo_rtree_node_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && !((*node).children[i as usize]).is_null() {
            if (*(*node).children[i as usize]).state as libc::c_int
                != CAIRO_RTREE_NODE_AVAILABLE as libc::c_int
            {
                return;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && !((*node).children[i as usize]).is_null() {
            _cairo_rtree_node_destroy(rtree, (*node).children[i as usize]);
            i += 1;
        }
        let ref mut fresh15 = (*node).children[0 as libc::c_int as usize];
        *fresh15 = 0 as *mut _cairo_rtree_node;
        (*node).state = CAIRO_RTREE_NODE_AVAILABLE as libc::c_int as uint16_t;
        cairo_list_move(&mut (*node).link, &mut (*rtree).available);
        node = (*node).parent;
        if node.is_null() {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_node_insert(
    mut rtree: *mut cairo_rtree_t,
    mut node: *mut cairo_rtree_node_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut out: *mut *mut cairo_rtree_node_t,
) -> cairo_status_t {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*node).state as libc::c_int == CAIRO_RTREE_NODE_AVAILABLE as libc::c_int
    {} else {
        __assert_fail(
            b"node->state == CAIRO_RTREE_NODE_AVAILABLE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-rtree.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"cairo_status_t _cairo_rtree_node_insert(cairo_rtree_t *, cairo_rtree_node_t *, int, int, cairo_rtree_node_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if (*node).pinned as libc::c_int == 0 as libc::c_int {} else {
        __assert_fail(
            b"node->pinned == FALSE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-rtree.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"cairo_status_t _cairo_rtree_node_insert(cairo_rtree_t *, cairo_rtree_node_t *, int, int, cairo_rtree_node_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if (*node).width as libc::c_int - width > (*rtree).min_size
        || (*node).height as libc::c_int - height > (*rtree).min_size
    {
        w = (*node).width as libc::c_int - width;
        h = (*node).height as libc::c_int - height;
        i = 0 as libc::c_int;
        let ref mut fresh16 = (*node).children[i as usize];
        *fresh16 = _cairo_rtree_node_create(
            rtree,
            node,
            (*node).x as libc::c_int,
            (*node).y as libc::c_int,
            width,
            height,
        );
        if ((*node).children[i as usize]).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        i += 1;
        if w > (*rtree).min_size {
            let ref mut fresh17 = (*node).children[i as usize];
            *fresh17 = _cairo_rtree_node_create(
                rtree,
                node,
                (*node).x as libc::c_int + width,
                (*node).y as libc::c_int,
                w,
                height,
            );
            if ((*node).children[i as usize]).is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            i += 1;
        }
        if h > (*rtree).min_size {
            let ref mut fresh18 = (*node).children[i as usize];
            *fresh18 = _cairo_rtree_node_create(
                rtree,
                node,
                (*node).x as libc::c_int,
                (*node).y as libc::c_int + height,
                width,
                h,
            );
            if ((*node).children[i as usize]).is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            i += 1;
            if w > (*rtree).min_size {
                let ref mut fresh19 = (*node).children[i as usize];
                *fresh19 = _cairo_rtree_node_create(
                    rtree,
                    node,
                    (*node).x as libc::c_int + width,
                    (*node).y as libc::c_int + height,
                    w,
                    h,
                );
                if ((*node).children[i as usize]).is_null() {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
                i += 1;
            }
        }
        if i < 4 as libc::c_int {
            let ref mut fresh20 = (*node).children[i as usize];
            *fresh20 = 0 as *mut _cairo_rtree_node;
        }
        (*node).state = CAIRO_RTREE_NODE_DIVIDED as libc::c_int as uint16_t;
        cairo_list_move(&mut (*node).link, &mut (*rtree).evictable);
        node = (*node).children[0 as libc::c_int as usize];
    }
    (*node).state = CAIRO_RTREE_NODE_OCCUPIED as libc::c_int as uint16_t;
    cairo_list_move(&mut (*node).link, &mut (*rtree).evictable);
    *out = node;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_node_remove(
    mut rtree: *mut cairo_rtree_t,
    mut node: *mut cairo_rtree_node_t,
) {
    if (*node).state as libc::c_int == CAIRO_RTREE_NODE_OCCUPIED as libc::c_int {} else {
        __assert_fail(
            b"node->state == CAIRO_RTREE_NODE_OCCUPIED\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-rtree.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void _cairo_rtree_node_remove(cairo_rtree_t *, cairo_rtree_node_t *)\0"))
                .as_ptr(),
        );
    }
    if (*node).pinned as libc::c_int == 0 as libc::c_int {} else {
        __assert_fail(
            b"node->pinned == FALSE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-rtree.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void _cairo_rtree_node_remove(cairo_rtree_t *, cairo_rtree_node_t *)\0"))
                .as_ptr(),
        );
    }
    ((*rtree).destroy).expect("non-null function pointer")(node);
    (*node).state = CAIRO_RTREE_NODE_AVAILABLE as libc::c_int as uint16_t;
    cairo_list_move(&mut (*node).link, &mut (*rtree).available);
    _cairo_rtree_node_collapse(rtree, (*node).parent);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_insert(
    mut rtree: *mut cairo_rtree_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut out: *mut *mut cairo_rtree_node_t,
) -> cairo_int_status_t {
    let mut node: *mut cairo_rtree_node_t = 0 as *mut cairo_rtree_node_t;
    node = ({
        let mut mptr__: *const cairo_list_t = (*rtree).available.next;
        (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
            as *mut cairo_rtree_node_t
    });
    while &mut (*node).link as *mut cairo_list_t
        != &mut (*rtree).available as *mut cairo_list_t
    {
        if (*node).width as libc::c_int >= width
            && (*node).height as libc::c_int >= height
        {
            return _cairo_rtree_node_insert(rtree, node, width, height, out)
                as cairo_int_status_t;
        }
        node = ({
            let mut mptr__: *const cairo_list_t = (*node).link.next;
            (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                as *mut cairo_rtree_node_t
        });
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
unsafe extern "C" fn hars_petruska_f54_1_random() -> uint32_t {
    static mut x: uint32_t = 0;
    x = (x ^ (x << 5 as libc::c_int | x >> 32 as libc::c_int - 5 as libc::c_int)
        ^ (x << 24 as libc::c_int | x >> 32 as libc::c_int - 24 as libc::c_int))
        .wrapping_add(0x37798849 as libc::c_int as libc::c_uint);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_evict_random(
    mut rtree: *mut cairo_rtree_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut out: *mut *mut cairo_rtree_node_t,
) -> cairo_int_status_t {
    let mut ret: cairo_int_status_t = CAIRO_INT_STATUS_UNSUPPORTED;
    let mut node: *mut cairo_rtree_node_t = 0 as *mut cairo_rtree_node_t;
    let mut next: *mut cairo_rtree_node_t = 0 as *mut cairo_rtree_node_t;
    let mut tmp_pinned: cairo_list_t = cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    };
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    cairo_list_init(&mut tmp_pinned);
    node = ({
        let mut mptr__: *const cairo_list_t = (*rtree).pinned.next;
        (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
            as *mut cairo_rtree_node_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*node).link.next;
        (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
            as *mut cairo_rtree_node_t
    });
    while &mut (*node).link as *mut cairo_list_t
        != &mut (*rtree).pinned as *mut cairo_list_t
    {
        node = (*node).parent;
        while !node.is_null() && (*node).pinned == 0 {
            (*node).pinned = 1 as libc::c_int as uint16_t;
            cairo_list_move(&mut (*node).link, &mut tmp_pinned);
            node = (*node).parent;
        }
        node = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                as *mut cairo_rtree_node_t
        });
    }
    cnt = 0 as libc::c_int;
    node = ({
        let mut mptr__: *const cairo_list_t = (*rtree).evictable.next;
        (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
            as *mut cairo_rtree_node_t
    });
    while &mut (*node).link as *mut cairo_list_t
        != &mut (*rtree).evictable as *mut cairo_list_t
    {
        if (*node).width as libc::c_int >= width
            && (*node).height as libc::c_int >= height
        {
            cnt += 1;
        }
        node = ({
            let mut mptr__: *const cairo_list_t = (*node).link.next;
            (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                as *mut cairo_rtree_node_t
        });
    }
    if !(cnt == 0 as libc::c_int) {
        cnt = (hars_petruska_f54_1_random()).wrapping_rem(cnt as libc::c_uint)
            as libc::c_int;
        node = ({
            let mut mptr__: *const cairo_list_t = (*rtree).evictable.next;
            (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                as *mut cairo_rtree_node_t
        });
        while &mut (*node).link as *mut cairo_list_t
            != &mut (*rtree).evictable as *mut cairo_list_t
        {
            if (*node).width as libc::c_int >= width
                && (*node).height as libc::c_int >= height
                && {
                    let fresh21 = cnt;
                    cnt = cnt - 1;
                    fresh21 == 0 as libc::c_int
                }
            {
                if (*node).state as libc::c_int
                    == CAIRO_RTREE_NODE_OCCUPIED as libc::c_int
                {
                    ((*rtree).destroy).expect("non-null function pointer")(node);
                } else {
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int
                        && !((*node).children[i as usize]).is_null()
                    {
                        _cairo_rtree_node_destroy(rtree, (*node).children[i as usize]);
                        i += 1;
                    }
                    let ref mut fresh22 = (*node).children[0 as libc::c_int as usize];
                    *fresh22 = 0 as *mut _cairo_rtree_node;
                }
                (*node).state = CAIRO_RTREE_NODE_AVAILABLE as libc::c_int as uint16_t;
                cairo_list_move(&mut (*node).link, &mut (*rtree).available);
                *out = node;
                ret = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                break;
            } else {
                node = ({
                    let mut mptr__: *const cairo_list_t = (*node).link.next;
                    (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                        as *mut cairo_rtree_node_t
                });
            }
        }
    }
    while cairo_list_is_empty(&mut tmp_pinned) == 0 {
        node = ({
            let mut mptr__: *const cairo_list_t = tmp_pinned.next;
            (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                as *mut cairo_rtree_node_t
        });
        (*node).pinned = 0 as libc::c_int as uint16_t;
        cairo_list_move(&mut (*node).link, &mut (*rtree).evictable);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_unpin(mut rtree: *mut cairo_rtree_t) {
    while cairo_list_is_empty(&mut (*rtree).pinned) == 0 {
        let mut node: *mut cairo_rtree_node_t = ({
            let mut mptr__: *const cairo_list_t = (*rtree).pinned.next;
            (mptr__ as *mut libc::c_char).offset(-(40 as libc::c_ulong as isize))
                as *mut cairo_rtree_node_t
        });
        (*node).pinned = 0 as libc::c_int as uint16_t;
        cairo_list_move(&mut (*node).link, &mut (*rtree).evictable);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_init(
    mut rtree: *mut cairo_rtree_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut min_size: libc::c_int,
    mut node_size: libc::c_int,
    mut destroy: Option::<unsafe extern "C" fn(*mut cairo_rtree_node_t) -> ()>,
) {
    if node_size
        >= ::std::mem::size_of::<cairo_rtree_node_t>() as libc::c_ulong as libc::c_int
    {} else {
        __assert_fail(
            b"node_size >= (int) sizeof (cairo_rtree_node_t)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-rtree.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"void _cairo_rtree_init(cairo_rtree_t *, int, int, int, int, void (*)(cairo_rtree_node_t *))\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_freepool_init(&mut (*rtree).node_freepool, node_size as libc::c_uint);
    cairo_list_init(&mut (*rtree).available);
    cairo_list_init(&mut (*rtree).pinned);
    cairo_list_init(&mut (*rtree).evictable);
    (*rtree).min_size = min_size;
    let ref mut fresh23 = (*rtree).destroy;
    *fresh23 = destroy;
    memset(
        &mut (*rtree).root as *mut cairo_rtree_node_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_rtree_node_t>() as libc::c_ulong,
    );
    (*rtree).root.width = width as uint16_t;
    (*rtree).root.height = height as uint16_t;
    (*rtree).root.state = CAIRO_RTREE_NODE_AVAILABLE as libc::c_int as uint16_t;
    cairo_list_add(&mut (*rtree).root.link, &mut (*rtree).available);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_reset(mut rtree: *mut cairo_rtree_t) {
    let mut i: libc::c_int = 0;
    if (*rtree).root.state as libc::c_int == CAIRO_RTREE_NODE_OCCUPIED as libc::c_int {
        ((*rtree).destroy).expect("non-null function pointer")(&mut (*rtree).root);
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && !((*rtree).root.children[i as usize]).is_null() {
            _cairo_rtree_node_destroy(rtree, (*rtree).root.children[i as usize]);
            i += 1;
        }
        let ref mut fresh24 = (*rtree).root.children[0 as libc::c_int as usize];
        *fresh24 = 0 as *mut _cairo_rtree_node;
    }
    cairo_list_init(&mut (*rtree).available);
    cairo_list_init(&mut (*rtree).evictable);
    cairo_list_init(&mut (*rtree).pinned);
    (*rtree).root.state = CAIRO_RTREE_NODE_AVAILABLE as libc::c_int as uint16_t;
    (*rtree).root.pinned = 0 as libc::c_int as uint16_t;
    cairo_list_add(&mut (*rtree).root.link, &mut (*rtree).available);
}
unsafe extern "C" fn _cairo_rtree_node_foreach(
    mut node: *mut cairo_rtree_node_t,
    mut func: Option::<
        unsafe extern "C" fn(*mut cairo_rtree_node_t, *mut libc::c_void) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int && !((*node).children[i as usize]).is_null() {
        _cairo_rtree_node_foreach((*node).children[i as usize], func, data);
        i += 1;
    }
    func.expect("non-null function pointer")(node, data);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_foreach(
    mut rtree: *mut cairo_rtree_t,
    mut func: Option::<
        unsafe extern "C" fn(*mut cairo_rtree_node_t, *mut libc::c_void) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    if (*rtree).root.state as libc::c_int == CAIRO_RTREE_NODE_OCCUPIED as libc::c_int {
        func.expect("non-null function pointer")(&mut (*rtree).root, data);
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && !((*rtree).root.children[i as usize]).is_null() {
            _cairo_rtree_node_foreach((*rtree).root.children[i as usize], func, data);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rtree_fini(mut rtree: *mut cairo_rtree_t) {
    let mut i: libc::c_int = 0;
    if (*rtree).root.state as libc::c_int == CAIRO_RTREE_NODE_OCCUPIED as libc::c_int {
        ((*rtree).destroy).expect("non-null function pointer")(&mut (*rtree).root);
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int && !((*rtree).root.children[i as usize]).is_null() {
            _cairo_rtree_node_destroy(rtree, (*rtree).root.children[i as usize]);
            i += 1;
        }
    }
    _cairo_freepool_fini(&mut (*rtree).node_freepool);
}
