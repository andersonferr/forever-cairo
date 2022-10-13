use ::libc;
extern "C" {
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_hash_table {
    pub keys_equal: cairo_hash_keys_equal_func_t,
    pub cache: [*mut cairo_hash_entry_t; 32],
    pub table_size: *const libc::c_ulong,
    pub entries: *mut *mut cairo_hash_entry_t,
    pub live_entries: libc::c_ulong,
    pub free_entries: libc::c_ulong,
    pub iterating: libc::c_ulong,
}
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
static mut hash_table_sizes: [libc::c_ulong; 25] = [
    43 as libc::c_int as libc::c_ulong,
    73 as libc::c_int as libc::c_ulong,
    151 as libc::c_int as libc::c_ulong,
    283 as libc::c_int as libc::c_ulong,
    571 as libc::c_int as libc::c_ulong,
    1153 as libc::c_int as libc::c_ulong,
    2269 as libc::c_int as libc::c_ulong,
    4519 as libc::c_int as libc::c_ulong,
    9013 as libc::c_int as libc::c_ulong,
    18043 as libc::c_int as libc::c_ulong,
    36109 as libc::c_int as libc::c_ulong,
    72091 as libc::c_int as libc::c_ulong,
    144409 as libc::c_int as libc::c_ulong,
    288361 as libc::c_int as libc::c_ulong,
    576883 as libc::c_int as libc::c_ulong,
    1153459 as libc::c_int as libc::c_ulong,
    2307163 as libc::c_int as libc::c_ulong,
    4613893 as libc::c_int as libc::c_ulong,
    9227641 as libc::c_int as libc::c_ulong,
    18455029 as libc::c_int as libc::c_ulong,
    36911011 as libc::c_int as libc::c_ulong,
    73819861 as libc::c_int as libc::c_ulong,
    147639589 as libc::c_int as libc::c_ulong,
    295279081 as libc::c_int as libc::c_ulong,
    590559793 as libc::c_int as libc::c_ulong,
];
unsafe extern "C" fn _cairo_hash_table_uid_keys_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_create(
    mut keys_equal: cairo_hash_keys_equal_func_t,
) -> *mut cairo_hash_table_t {
    let mut hash_table: *mut cairo_hash_table_t = 0 as *mut cairo_hash_table_t;
    hash_table = (if ::std::mem::size_of::<cairo_hash_table_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_hash_table_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_hash_table_t;
    if hash_table.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_hash_table_t;
    }
    if keys_equal.is_none() {
        let ref mut fresh0 = (*hash_table).keys_equal;
        *fresh0 = Some(
            _cairo_hash_table_uid_keys_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        );
    } else {
        let ref mut fresh1 = (*hash_table).keys_equal;
        *fresh1 = keys_equal;
    }
    memset(
        &mut (*hash_table).cache as *mut [*mut cairo_hash_entry_t; 32]
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut cairo_hash_entry_t; 32]>() as libc::c_ulong,
    );
    let ref mut fresh2 = (*hash_table).table_size;
    *fresh2 = &*hash_table_sizes.as_ptr().offset(0 as libc::c_int as isize)
        as *const libc::c_ulong;
    let ref mut fresh3 = (*hash_table).entries;
    *fresh3 = calloc(
        *(*hash_table).table_size,
        ::std::mem::size_of::<*mut cairo_hash_entry_t>() as libc::c_ulong,
    ) as *mut *mut cairo_hash_entry_t;
    if ((*hash_table).entries).is_null() {
        let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        free(hash_table as *mut libc::c_void);
        return 0 as *mut cairo_hash_table_t;
    }
    (*hash_table).live_entries = 0 as libc::c_int as libc::c_ulong;
    (*hash_table).free_entries = *(*hash_table).table_size;
    (*hash_table).iterating = 0 as libc::c_int as libc::c_ulong;
    return hash_table;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_destroy(
    mut hash_table: *mut cairo_hash_table_t,
) {
    if (*hash_table).live_entries == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hash_table->live_entries == 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void _cairo_hash_table_destroy(cairo_hash_table_t *)\0"))
                .as_ptr(),
        );
    }
    if (*hash_table).iterating == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hash_table->iterating == 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void _cairo_hash_table_destroy(cairo_hash_table_t *)\0"))
                .as_ptr(),
        );
    }
    free((*hash_table).entries as *mut libc::c_void);
    free(hash_table as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_hash_table_lookup_unique_key(
    mut hash_table: *mut cairo_hash_table_t,
    mut key: *mut cairo_hash_entry_t,
) -> *mut *mut cairo_hash_entry_t {
    let mut table_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut idx: libc::c_ulong = 0;
    let mut step: libc::c_ulong = 0;
    let mut entry: *mut *mut cairo_hash_entry_t = 0 as *mut *mut cairo_hash_entry_t;
    table_size = *(*hash_table).table_size;
    idx = ((*key).hash).wrapping_rem(table_size);
    entry = &mut *((*hash_table).entries).offset(idx as isize)
        as *mut *mut cairo_hash_entry_t;
    if !(*entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t) {
        return entry;
    }
    i = 1 as libc::c_int as libc::c_ulong;
    step = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            ((*key).hash)
                .wrapping_rem(table_size.wrapping_sub(2 as libc::c_int as libc::c_ulong)),
        );
    loop {
        idx = idx.wrapping_add(step);
        if idx >= table_size {
            idx = idx.wrapping_sub(table_size);
        }
        entry = &mut *((*hash_table).entries).offset(idx as isize)
            as *mut *mut cairo_hash_entry_t;
        if !(*entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t) {
            return entry;
        }
        i = i.wrapping_add(1);
        if !(i < table_size) {
            break;
        }
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"cairo_hash_entry_t **_cairo_hash_table_lookup_unique_key(cairo_hash_table_t *, cairo_hash_entry_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return 0 as *mut *mut cairo_hash_entry_t;
}
unsafe extern "C" fn _cairo_hash_table_manage(
    mut hash_table: *mut cairo_hash_table_t,
) -> cairo_status_t {
    let mut tmp: cairo_hash_table_t = cairo_hash_table_t {
        keys_equal: None,
        cache: [0 as *mut cairo_hash_entry_t; 32],
        table_size: 0 as *const libc::c_ulong,
        entries: 0 as *mut *mut cairo_hash_entry_t,
        live_entries: 0,
        free_entries: 0,
        iterating: 0,
    };
    let mut new_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut live_high: libc::c_ulong = *(*hash_table).table_size >> 1 as libc::c_int;
    let mut live_low: libc::c_ulong = live_high >> 2 as libc::c_int;
    let mut free_low: libc::c_ulong = live_high >> 1 as libc::c_int;
    tmp = *hash_table;
    if (*hash_table).live_entries > live_high {
        tmp.table_size = ((*hash_table).table_size).offset(1 as libc::c_int as isize);
        if ((tmp.table_size).offset_from(hash_table_sizes.as_ptr()) as libc::c_long)
            < (::std::mem::size_of::<[libc::c_ulong; 25]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                as libc::c_int as libc::c_long
        {} else {
            __assert_fail(
                b"tmp.table_size - hash_table_sizes < ARRAY_LENGTH (hash_table_sizes)\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"cairo_status_t _cairo_hash_table_manage(cairo_hash_table_t *)\0"))
                    .as_ptr(),
            );
        }
    } else if (*hash_table).live_entries < live_low {
        if (*hash_table).table_size
            == &*hash_table_sizes.as_ptr().offset(0 as libc::c_int as isize)
                as *const libc::c_ulong
        {
            tmp.table_size = (*hash_table).table_size;
        } else {
            tmp
                .table_size = ((*hash_table).table_size)
                .offset(-(1 as libc::c_int as isize));
        }
    }
    if tmp.table_size == (*hash_table).table_size
        && (*hash_table).free_entries > free_low
    {
        return CAIRO_STATUS_SUCCESS;
    }
    new_size = *tmp.table_size;
    tmp
        .entries = calloc(
        new_size,
        ::std::mem::size_of::<*mut cairo_hash_entry_t>() as libc::c_ulong,
    ) as *mut *mut cairo_hash_entry_t;
    if (tmp.entries).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < *(*hash_table).table_size {
        if *((*hash_table).entries).offset(i as isize)
            > 0x1 as libc::c_int as *mut cairo_hash_entry_t
        {
            let ref mut fresh4 = *_cairo_hash_table_lookup_unique_key(
                &mut tmp,
                *((*hash_table).entries).offset(i as isize),
            );
            *fresh4 = *((*hash_table).entries).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    free((*hash_table).entries as *mut libc::c_void);
    let ref mut fresh5 = (*hash_table).entries;
    *fresh5 = tmp.entries;
    let ref mut fresh6 = (*hash_table).table_size;
    *fresh6 = tmp.table_size;
    (*hash_table).free_entries = new_size.wrapping_sub((*hash_table).live_entries);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_lookup(
    mut hash_table: *mut cairo_hash_table_t,
    mut key: *mut cairo_hash_entry_t,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut entry: *mut cairo_hash_entry_t = 0 as *mut cairo_hash_entry_t;
    let mut table_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut idx: libc::c_ulong = 0;
    let mut step: libc::c_ulong = 0;
    let mut hash: uintptr_t = (*key).hash;
    entry = (*hash_table).cache[(hash & 31 as libc::c_int as libc::c_ulong) as usize];
    if !entry.is_null() && (*entry).hash == hash
        && ((*hash_table).keys_equal)
            .expect(
                "non-null function pointer",
            )(key as *const libc::c_void, entry as *const libc::c_void) != 0
    {
        return entry as *mut libc::c_void;
    }
    table_size = *(*hash_table).table_size;
    idx = hash.wrapping_rem(table_size);
    entry = *((*hash_table).entries).offset(idx as isize);
    if entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t {
        if (*entry).hash == hash
            && ((*hash_table).keys_equal)
                .expect(
                    "non-null function pointer",
                )(key as *const libc::c_void, entry as *const libc::c_void) != 0
        {
            current_block = 12613343639736369461;
        } else {
            current_block = 11650488183268122163;
        }
    } else {
        if entry.is_null() {
            return 0 as *mut libc::c_void;
        }
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            i = 1 as libc::c_int as libc::c_ulong;
            step = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    hash
                        .wrapping_rem(
                            table_size.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                        ),
                );
            loop {
                idx = idx.wrapping_add(step);
                if idx >= table_size {
                    idx = idx.wrapping_sub(table_size);
                }
                entry = *((*hash_table).entries).offset(idx as isize);
                if entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t {
                    if (*entry).hash == hash
                        && ((*hash_table).keys_equal)
                            .expect(
                                "non-null function pointer",
                            )(key as *const libc::c_void, entry as *const libc::c_void)
                            != 0
                    {
                        current_block = 12613343639736369461;
                        break;
                    }
                } else if entry.is_null() {
                    return 0 as *mut libc::c_void
                }
                i = i.wrapping_add(1);
                if !(i < table_size) {
                    current_block = 2668756484064249700;
                    break;
                }
            }
            match current_block {
                12613343639736369461 => {}
                _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
                            374 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 75],
                                &[libc::c_char; 75],
                            >(
                                b"void *_cairo_hash_table_lookup(cairo_hash_table_t *, cairo_hash_entry_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    return 0 as *mut libc::c_void;
                }
            }
        }
        _ => {}
    }
    let ref mut fresh7 = (*hash_table)
        .cache[(hash & 31 as libc::c_int as libc::c_ulong) as usize];
    *fresh7 = entry;
    return entry as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_random_entry(
    mut hash_table: *mut cairo_hash_table_t,
    mut predicate: cairo_hash_predicate_func_t,
) -> *mut libc::c_void {
    let mut entry: *mut cairo_hash_entry_t = 0 as *mut cairo_hash_entry_t;
    let mut hash: libc::c_ulong = 0;
    let mut table_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut idx: libc::c_ulong = 0;
    let mut step: libc::c_ulong = 0;
    if predicate.is_some() {} else {
        __assert_fail(
            b"predicate != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"void *_cairo_hash_table_random_entry(cairo_hash_table_t *, cairo_hash_predicate_func_t)\0",
            ))
                .as_ptr(),
        );
    }
    table_size = *(*hash_table).table_size;
    hash = rand() as libc::c_ulong;
    idx = hash.wrapping_rem(table_size);
    entry = *((*hash_table).entries).offset(idx as isize);
    if entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t
        && predicate.expect("non-null function pointer")(entry as *const libc::c_void)
            != 0
    {
        return entry as *mut libc::c_void;
    }
    i = 1 as libc::c_int as libc::c_ulong;
    step = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            hash.wrapping_rem(table_size.wrapping_sub(2 as libc::c_int as libc::c_ulong)),
        );
    loop {
        idx = idx.wrapping_add(step);
        if idx >= table_size {
            idx = idx.wrapping_sub(table_size);
        }
        entry = *((*hash_table).entries).offset(idx as isize);
        if entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t
            && predicate
                .expect("non-null function pointer")(entry as *const libc::c_void) != 0
        {
            return entry as *mut libc::c_void;
        }
        i = i.wrapping_add(1);
        if !(i < table_size) {
            break;
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_insert(
    mut hash_table: *mut cairo_hash_table_t,
    mut key_and_value: *mut cairo_hash_entry_t,
) -> cairo_status_t {
    let mut entry: *mut *mut cairo_hash_entry_t = 0 as *mut *mut cairo_hash_entry_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*hash_table).iterating == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hash_table->iterating == 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"cairo_status_t _cairo_hash_table_insert(cairo_hash_table_t *, cairo_hash_entry_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_hash_table_manage(hash_table);
    if status as u64 != 0 {
        return status;
    }
    entry = _cairo_hash_table_lookup_unique_key(hash_table, key_and_value);
    if (*entry).is_null() {
        let ref mut fresh8 = (*hash_table).free_entries;
        *fresh8 = (*fresh8).wrapping_sub(1);
    }
    *entry = key_and_value;
    let ref mut fresh9 = (*hash_table)
        .cache[((*key_and_value).hash & 31 as libc::c_int as libc::c_ulong) as usize];
    *fresh9 = key_and_value;
    let ref mut fresh10 = (*hash_table).live_entries;
    *fresh10 = (*fresh10).wrapping_add(1);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_hash_table_lookup_exact_key(
    mut hash_table: *mut cairo_hash_table_t,
    mut key: *mut cairo_hash_entry_t,
) -> *mut *mut cairo_hash_entry_t {
    let mut table_size: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut idx: libc::c_ulong = 0;
    let mut step: libc::c_ulong = 0;
    let mut entry: *mut *mut cairo_hash_entry_t = 0 as *mut *mut cairo_hash_entry_t;
    table_size = *(*hash_table).table_size;
    idx = ((*key).hash).wrapping_rem(table_size);
    entry = &mut *((*hash_table).entries).offset(idx as isize)
        as *mut *mut cairo_hash_entry_t;
    if *entry == key {
        return entry;
    }
    i = 1 as libc::c_int as libc::c_ulong;
    step = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            ((*key).hash)
                .wrapping_rem(table_size.wrapping_sub(2 as libc::c_int as libc::c_ulong)),
        );
    loop {
        idx = idx.wrapping_add(step);
        if idx >= table_size {
            idx = idx.wrapping_sub(table_size);
        }
        entry = &mut *((*hash_table).entries).offset(idx as isize)
            as *mut *mut cairo_hash_entry_t;
        if *entry == key {
            return entry;
        }
        i = i.wrapping_add(1);
        if !(i < table_size) {
            break;
        }
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-hash.c\0" as *const u8 as *const libc::c_char,
            506 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"cairo_hash_entry_t **_cairo_hash_table_lookup_exact_key(cairo_hash_table_t *, cairo_hash_entry_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return 0 as *mut *mut cairo_hash_entry_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_remove(
    mut hash_table: *mut cairo_hash_table_t,
    mut key: *mut cairo_hash_entry_t,
) {
    let ref mut fresh11 = *_cairo_hash_table_lookup_exact_key(hash_table, key);
    *fresh11 = 0x1 as libc::c_int as *mut cairo_hash_entry_t;
    let ref mut fresh12 = (*hash_table).live_entries;
    *fresh12 = (*fresh12).wrapping_sub(1);
    let ref mut fresh13 = (*hash_table)
        .cache[((*key).hash & 31 as libc::c_int as libc::c_ulong) as usize];
    *fresh13 = 0 as *mut cairo_hash_entry_t;
    if (*hash_table).iterating == 0 as libc::c_int as libc::c_ulong {
        _cairo_hash_table_manage(hash_table);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hash_table_foreach(
    mut hash_table: *mut cairo_hash_table_t,
    mut hash_callback: cairo_hash_callback_func_t,
    mut closure: *mut libc::c_void,
) {
    let mut i: libc::c_ulong = 0;
    let mut entry: *mut cairo_hash_entry_t = 0 as *mut cairo_hash_entry_t;
    let ref mut fresh14 = (*hash_table).iterating;
    *fresh14 = (*fresh14).wrapping_add(1);
    i = 0 as libc::c_int as libc::c_ulong;
    while i < *(*hash_table).table_size {
        entry = *((*hash_table).entries).offset(i as isize);
        if entry > 0x1 as libc::c_int as *mut cairo_hash_entry_t {
            hash_callback
                .expect(
                    "non-null function pointer",
                )(entry as *mut libc::c_void, closure);
        }
        i = i.wrapping_add(1);
    }
    let ref mut fresh15 = (*hash_table).iterating;
    *fresh15 = (*fresh15).wrapping_sub(1);
    if *fresh15 == 0 as libc::c_int as libc::c_ulong {
        _cairo_hash_table_manage(hash_table);
    }
}
