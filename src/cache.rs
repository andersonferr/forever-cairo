use ::libc;
extern "C" {
    pub type _cairo_hash_table;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_hash_table_random_entry(
        hash_table: *mut cairo_hash_table_t,
        predicate: cairo_hash_predicate_func_t,
    ) -> *mut libc::c_void;
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
}
pub type __uint8_t = libc::c_uchar;
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
pub type cairo_hash_entry_t = _cairo_hash_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_hash_entry {
    pub hash: uintptr_t,
}
pub type uintptr_t = libc::c_ulong;
pub type cairo_hash_table_t = _cairo_hash_table;
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cache {
    pub hash_table: *mut cairo_hash_table_t,
    pub predicate: cairo_cache_predicate_func_t,
    pub entry_destroy: cairo_destroy_func_t,
    pub max_size: libc::c_ulong,
    pub size: libc::c_ulong,
    pub freeze_count: libc::c_int,
}
pub type cairo_cache_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_cache_t = _cairo_cache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cache_entry {
    pub hash: uintptr_t,
    pub size: libc::c_ulong,
}
pub type cairo_cache_entry_t = _cairo_cache_entry;
pub type cairo_cache_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_cache_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cairo_hash_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
unsafe extern "C" fn _cairo_cache_entry_is_non_zero(
    mut entry: *const libc::c_void,
) -> cairo_bool_t {
    return (*(entry as *const cairo_cache_entry_t)).size as cairo_bool_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_init(
    mut cache: *mut cairo_cache_t,
    mut keys_equal: cairo_cache_keys_equal_func_t,
    mut predicate: cairo_cache_predicate_func_t,
    mut entry_destroy: cairo_destroy_func_t,
    mut max_size: libc::c_ulong,
) -> cairo_status_t {
    let ref mut fresh0 = (*cache).hash_table;
    *fresh0 = _cairo_hash_table_create(keys_equal);
    if ((*cache).hash_table).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    if predicate.is_none() {
        predicate = Some(
            _cairo_cache_entry_is_non_zero
                as unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
        );
    }
    let ref mut fresh1 = (*cache).predicate;
    *fresh1 = predicate;
    let ref mut fresh2 = (*cache).entry_destroy;
    *fresh2 = entry_destroy;
    (*cache).max_size = max_size;
    (*cache).size = 0 as libc::c_int as libc::c_ulong;
    (*cache).freeze_count = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_cache_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    _cairo_cache_remove(
        closure as *mut cairo_cache_t,
        entry as *mut cairo_cache_entry_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_fini(mut cache: *mut cairo_cache_t) {
    _cairo_hash_table_foreach(
        (*cache).hash_table,
        Some(
            _cairo_cache_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        cache as *mut libc::c_void,
    );
    if (*cache).size == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cache->size == 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cache.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void _cairo_cache_fini(cairo_cache_t *)\0"))
                .as_ptr(),
        );
    }
    _cairo_hash_table_destroy((*cache).hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_freeze(mut cache: *mut cairo_cache_t) {
    if (*cache).freeze_count >= 0 as libc::c_int {} else {
        __assert_fail(
            b"cache->freeze_count >= 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cache.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void _cairo_cache_freeze(cairo_cache_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh3 = (*cache).freeze_count;
    *fresh3 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_thaw(mut cache: *mut cairo_cache_t) {
    if (*cache).freeze_count > 0 as libc::c_int {} else {
        __assert_fail(
            b"cache->freeze_count > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cache.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void _cairo_cache_thaw(cairo_cache_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh4 = (*cache).freeze_count;
    *fresh4 -= 1;
    if *fresh4 == 0 as libc::c_int {
        _cairo_cache_shrink_to_accommodate(cache, 0 as libc::c_int as libc::c_ulong);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_lookup(
    mut cache: *mut cairo_cache_t,
    mut key: *mut cairo_cache_entry_t,
) -> *mut libc::c_void {
    return _cairo_hash_table_lookup((*cache).hash_table, key as *mut cairo_hash_entry_t);
}
unsafe extern "C" fn _cairo_cache_remove_random(
    mut cache: *mut cairo_cache_t,
) -> cairo_bool_t {
    let mut entry: *mut cairo_cache_entry_t = 0 as *mut cairo_cache_entry_t;
    entry = _cairo_hash_table_random_entry((*cache).hash_table, (*cache).predicate)
        as *mut cairo_cache_entry_t;
    if entry.is_null() {
        return 0 as libc::c_int;
    }
    _cairo_cache_remove(cache, entry);
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_cache_shrink_to_accommodate(
    mut cache: *mut cairo_cache_t,
    mut additional: libc::c_ulong,
) {
    while ((*cache).size).wrapping_add(additional) > (*cache).max_size {
        if _cairo_cache_remove_random(cache) == 0 {
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_insert(
    mut cache: *mut cairo_cache_t,
    mut entry: *mut cairo_cache_entry_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*entry).size != 0 && (*cache).freeze_count == 0 {
        _cairo_cache_shrink_to_accommodate(cache, (*entry).size);
    }
    status = _cairo_hash_table_insert(
        (*cache).hash_table,
        entry as *mut cairo_hash_entry_t,
    );
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh5 = (*cache).size;
    *fresh5 = (*fresh5).wrapping_add((*entry).size);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_remove(
    mut cache: *mut cairo_cache_t,
    mut entry: *mut cairo_cache_entry_t,
) {
    let ref mut fresh6 = (*cache).size;
    *fresh6 = (*fresh6).wrapping_sub((*entry).size);
    _cairo_hash_table_remove((*cache).hash_table, entry as *mut cairo_hash_entry_t);
    if ((*cache).entry_destroy).is_some() {
        ((*cache).entry_destroy)
            .expect("non-null function pointer")(entry as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cache_foreach(
    mut cache: *mut cairo_cache_t,
    mut cache_callback: cairo_cache_callback_func_t,
    mut closure: *mut libc::c_void,
) {
    _cairo_hash_table_foreach((*cache).hash_table, cache_callback, closure);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_string(mut c: *const libc::c_char) -> uintptr_t {
    let mut hash: uintptr_t = 5381 as libc::c_int as uintptr_t;
    while !c.is_null() && *c as libc::c_int != 0 {
        let fresh7 = c;
        c = c.offset(1);
        hash = (hash << 5 as libc::c_int)
            .wrapping_add(hash)
            .wrapping_add(*fresh7 as libc::c_ulong);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_bytes(
    mut hash: uintptr_t,
    mut ptr: *const libc::c_void,
    mut length: libc::c_uint,
) -> uintptr_t {
    let mut bytes: *const uint8_t = ptr as *const uint8_t;
    loop {
        let fresh8 = length;
        length = length.wrapping_sub(1);
        if !(fresh8 != 0) {
            break;
        }
        let fresh9 = bytes;
        bytes = bytes.offset(1);
        hash = (hash << 5 as libc::c_int)
            .wrapping_add(hash)
            .wrapping_add(*fresh9 as libc::c_ulong);
    }
    return hash;
}
