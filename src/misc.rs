use ::libc;
extern "C" {
    pub type __locale_data;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo_hash_table;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strtod_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: locale_t,
    ) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_utf8_to_ucs4(
        str: *const libc::c_char,
        len: libc::c_int,
        result: *mut *mut uint32_t,
        items_written: *mut libc::c_int,
    ) -> cairo_status_t;
    static mut _cairo_intern_string_mutex: cairo_mutex_t;
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
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
    fn localeconv() -> *mut lconv;
    fn newlocale(
        __category_mask: libc::c_int,
        __locale: *const libc::c_char,
        __base: locale_t,
    ) -> locale_t;
    fn freelocale(__dataset: locale_t);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type uint32_t = __uint32_t;
pub type cairo_text_cluster_flags_t = _cairo_text_cluster_flags;
pub type _cairo_text_cluster_flags = libc::c_uint;
pub const CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: _cairo_text_cluster_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_text_cluster_t {
    pub num_bytes: libc::c_int,
    pub num_glyphs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_glyph_t {
    pub index: libc::c_ulong,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_hash_entry_t = _cairo_hash_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_hash_entry {
    pub hash: uintptr_t,
}
pub type uintptr_t = libc::c_ulong;
pub type cairo_hash_table_t = _cairo_hash_table;
pub type cairo_operator_t = _cairo_operator;
pub type _cairo_operator = libc::c_uint;
pub const CAIRO_OPERATOR_HSL_LUMINOSITY: _cairo_operator = 28;
pub const CAIRO_OPERATOR_HSL_COLOR: _cairo_operator = 27;
pub const CAIRO_OPERATOR_HSL_SATURATION: _cairo_operator = 26;
pub const CAIRO_OPERATOR_HSL_HUE: _cairo_operator = 25;
pub const CAIRO_OPERATOR_EXCLUSION: _cairo_operator = 24;
pub const CAIRO_OPERATOR_DIFFERENCE: _cairo_operator = 23;
pub const CAIRO_OPERATOR_SOFT_LIGHT: _cairo_operator = 22;
pub const CAIRO_OPERATOR_HARD_LIGHT: _cairo_operator = 21;
pub const CAIRO_OPERATOR_COLOR_BURN: _cairo_operator = 20;
pub const CAIRO_OPERATOR_COLOR_DODGE: _cairo_operator = 19;
pub const CAIRO_OPERATOR_LIGHTEN: _cairo_operator = 18;
pub const CAIRO_OPERATOR_DARKEN: _cairo_operator = 17;
pub const CAIRO_OPERATOR_OVERLAY: _cairo_operator = 16;
pub const CAIRO_OPERATOR_SCREEN: _cairo_operator = 15;
pub const CAIRO_OPERATOR_MULTIPLY: _cairo_operator = 14;
pub const CAIRO_OPERATOR_SATURATE: _cairo_operator = 13;
pub const CAIRO_OPERATOR_ADD: _cairo_operator = 12;
pub const CAIRO_OPERATOR_XOR: _cairo_operator = 11;
pub const CAIRO_OPERATOR_DEST_ATOP: _cairo_operator = 10;
pub const CAIRO_OPERATOR_DEST_OUT: _cairo_operator = 9;
pub const CAIRO_OPERATOR_DEST_IN: _cairo_operator = 8;
pub const CAIRO_OPERATOR_DEST_OVER: _cairo_operator = 7;
pub const CAIRO_OPERATOR_DEST: _cairo_operator = 6;
pub const CAIRO_OPERATOR_ATOP: _cairo_operator = 5;
pub const CAIRO_OPERATOR_OUT: _cairo_operator = 4;
pub const CAIRO_OPERATOR_IN: _cairo_operator = 3;
pub const CAIRO_OPERATOR_OVER: _cairo_operator = 2;
pub const CAIRO_OPERATOR_SOURCE: _cairo_operator = 1;
pub const CAIRO_OPERATOR_CLEAR: _cairo_operator = 0;
pub type uint16_t = __uint16_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: uint32_t,
    pub f: libc::c_float,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const CAIRO_OPERATOR_BOUND_BY_SOURCE: C2RustUnnamed_0 = 4;
pub const CAIRO_OPERATOR_BOUND_BY_MASK: C2RustUnnamed_0 = 2;
pub type cairo_intern_string_t = _cairo_intern_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_intern_string {
    pub hash_entry: cairo_hash_entry_t,
    pub len: libc::c_int,
    pub string: *mut libc::c_char,
}
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
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
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_get(
    mut x: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_cmpxchg_impl(
    mut x: *mut *mut libc::c_void,
    mut oldv: *mut libc::c_void,
    mut newv: *mut libc::c_void,
) -> cairo_bool_t {
    let mut expected: *mut libc::c_void = oldv;
    let fresh2 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh2.0;
    return fresh2.1 as cairo_bool_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_status_to_string(
    mut status: cairo_status_t,
) -> *const libc::c_char {
    match status as libc::c_uint {
        0 => return b"no error has occurred\0" as *const u8 as *const libc::c_char,
        1 => return b"out of memory\0" as *const u8 as *const libc::c_char,
        2 => {
            return b"cairo_restore() without matching cairo_save()\0" as *const u8
                as *const libc::c_char;
        }
        3 => {
            return b"no saved group to pop, i.e. cairo_pop_group() without matching cairo_push_group()\0"
                as *const u8 as *const libc::c_char;
        }
        4 => return b"no current point defined\0" as *const u8 as *const libc::c_char,
        5 => {
            return b"invalid matrix (not invertible)\0" as *const u8
                as *const libc::c_char;
        }
        6 => {
            return b"invalid value for an input cairo_status_t\0" as *const u8
                as *const libc::c_char;
        }
        7 => return b"NULL pointer\0" as *const u8 as *const libc::c_char,
        8 => return b"input string not valid UTF-8\0" as *const u8 as *const libc::c_char,
        9 => return b"input path data not valid\0" as *const u8 as *const libc::c_char,
        10 => {
            return b"error while reading from input stream\0" as *const u8
                as *const libc::c_char;
        }
        11 => {
            return b"error while writing to output stream\0" as *const u8
                as *const libc::c_char;
        }
        12 => {
            return b"the target surface has been finished\0" as *const u8
                as *const libc::c_char;
        }
        13 => {
            return b"the surface type is not appropriate for the operation\0"
                as *const u8 as *const libc::c_char;
        }
        14 => {
            return b"the pattern type is not appropriate for the operation\0"
                as *const u8 as *const libc::c_char;
        }
        15 => {
            return b"invalid value for an input cairo_content_t\0" as *const u8
                as *const libc::c_char;
        }
        16 => {
            return b"invalid value for an input cairo_format_t\0" as *const u8
                as *const libc::c_char;
        }
        17 => {
            return b"invalid value for an input Visual*\0" as *const u8
                as *const libc::c_char;
        }
        18 => return b"file not found\0" as *const u8 as *const libc::c_char,
        19 => {
            return b"invalid value for a dash setting\0" as *const u8
                as *const libc::c_char;
        }
        20 => {
            return b"invalid value for a DSC comment\0" as *const u8
                as *const libc::c_char;
        }
        21 => {
            return b"invalid index passed to getter\0" as *const u8
                as *const libc::c_char;
        }
        22 => {
            return b"clip region not representable in desired format\0" as *const u8
                as *const libc::c_char;
        }
        23 => {
            return b"error creating or writing to a temporary file\0" as *const u8
                as *const libc::c_char;
        }
        24 => return b"invalid value for stride\0" as *const u8 as *const libc::c_char,
        25 => {
            return b"the font type is not appropriate for the operation\0" as *const u8
                as *const libc::c_char;
        }
        26 => return b"the user-font is immutable\0" as *const u8 as *const libc::c_char,
        27 => {
            return b"error occurred in a user-font callback function\0" as *const u8
                as *const libc::c_char;
        }
        28 => {
            return b"negative number used where it is not allowed\0" as *const u8
                as *const libc::c_char;
        }
        29 => {
            return b"input clusters do not represent the accompanying text and glyph arrays\0"
                as *const u8 as *const libc::c_char;
        }
        30 => {
            return b"invalid value for an input cairo_font_slant_t\0" as *const u8
                as *const libc::c_char;
        }
        31 => {
            return b"invalid value for an input cairo_font_weight_t\0" as *const u8
                as *const libc::c_char;
        }
        32 => {
            return b"invalid value (typically too big) for the size of the input (surface, pattern, etc.)\0"
                as *const u8 as *const libc::c_char;
        }
        33 => {
            return b"user-font method not implemented\0" as *const u8
                as *const libc::c_char;
        }
        34 => {
            return b"the device type is not appropriate for the operation\0" as *const u8
                as *const libc::c_char;
        }
        35 => {
            return b"an operation to the device caused an unspecified error\0"
                as *const u8 as *const libc::c_char;
        }
        36 => {
            return b"invalid operation during mesh pattern construction\0" as *const u8
                as *const libc::c_char;
        }
        37 => {
            return b"the target device has been finished\0" as *const u8
                as *const libc::c_char;
        }
        38 => {
            return b"CAIRO_MIME_TYPE_JBIG2_GLOBAL_ID used but no CAIRO_MIME_TYPE_JBIG2_GLOBAL data provided\0"
                as *const u8 as *const libc::c_char;
        }
        39 => {
            return b"error occurred in libpng while reading from or writing to a PNG file\0"
                as *const u8 as *const libc::c_char;
        }
        40 => {
            return b"error occurred in libfreetype\0" as *const u8 as *const libc::c_char;
        }
        41 => {
            return b"error occurred in the Windows Graphics Device Interface\0"
                as *const u8 as *const libc::c_char;
        }
        42 => {
            return b"invalid tag name, attributes, or nesting\0" as *const u8
                as *const libc::c_char;
        }
        43 => return b"Window Direct Write error\0" as *const u8 as *const libc::c_char,
        44 | _ => return b"<unknown error status>\0" as *const u8 as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_glyph_allocate(
    mut num_glyphs: libc::c_int,
) -> *mut cairo_glyph_t {
    if num_glyphs <= 0 as libc::c_int {
        return 0 as *mut cairo_glyph_t;
    }
    return _cairo_malloc_ab(
        num_glyphs as size_t,
        ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
    ) as *mut cairo_glyph_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_glyph_free(mut glyphs: *mut cairo_glyph_t) {
    free(glyphs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_text_cluster_allocate(
    mut num_clusters: libc::c_int,
) -> *mut cairo_text_cluster_t {
    if num_clusters <= 0 as libc::c_int {
        return 0 as *mut cairo_text_cluster_t;
    }
    return _cairo_malloc_ab(
        num_clusters as size_t,
        ::std::mem::size_of::<cairo_text_cluster_t>() as libc::c_ulong,
    ) as *mut cairo_text_cluster_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_text_cluster_free(
    mut clusters: *mut cairo_text_cluster_t,
) {
    free(clusters as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_validate_text_clusters(
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *const cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut clusters: *const cairo_text_cluster_t,
    mut num_clusters: libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut n_bytes: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n_glyphs: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    loop {
        if !(i < num_clusters) {
            current_block = 4166486009154926805;
            break;
        }
        let mut cluster_bytes: libc::c_int = (*clusters.offset(i as isize)).num_bytes;
        let mut cluster_glyphs: libc::c_int = (*clusters.offset(i as isize)).num_glyphs;
        if cluster_bytes < 0 as libc::c_int || cluster_glyphs < 0 as libc::c_int {
            current_block = 3378096811297028624;
            break;
        }
        if cluster_bytes == 0 as libc::c_int && cluster_glyphs == 0 as libc::c_int {
            current_block = 3378096811297028624;
            break;
        }
        if n_bytes.wrapping_add(cluster_bytes as libc::c_uint) > utf8_len as libc::c_uint
            || n_glyphs.wrapping_add(cluster_glyphs as libc::c_uint)
                > num_glyphs as libc::c_uint
        {
            current_block = 3378096811297028624;
            break;
        }
        status = _cairo_utf8_to_ucs4(
            utf8.offset(n_bytes as isize),
            cluster_bytes,
            0 as *mut *mut uint32_t,
            0 as *mut libc::c_int,
        );
        if status as u64 != 0 {
            return _cairo_error(CAIRO_STATUS_INVALID_CLUSTERS);
        }
        n_bytes = n_bytes.wrapping_add(cluster_bytes as libc::c_uint);
        n_glyphs = n_glyphs.wrapping_add(cluster_glyphs as libc::c_uint);
        i += 1;
    }
    match current_block {
        4166486009154926805 => {
            if !(n_bytes != utf8_len as libc::c_uint
                || n_glyphs != num_glyphs as libc::c_uint)
            {
                return CAIRO_STATUS_SUCCESS;
            }
        }
        _ => {}
    }
    return _cairo_error(CAIRO_STATUS_INVALID_CLUSTERS);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_operator_bounded_by_mask(
    mut op: cairo_operator_t,
) -> cairo_bool_t {
    match op as libc::c_uint {
        0 | 1 | 2 | 5 | 6 | 7 | 9 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21
        | 22 | 23 | 24 | 25 | 26 | 27 | 28 => return 1 as libc::c_int,
        4 | 3 | 8 | 10 => return 0 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-misc.c\0" as *const u8 as *const libc::c_char,
                    408 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 63],
                        &[libc::c_char; 63],
                    >(
                        b"cairo_bool_t _cairo_operator_bounded_by_mask(cairo_operator_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_operator_bounded_by_source(
    mut op: cairo_operator_t,
) -> cairo_bool_t {
    match op as libc::c_uint {
        2 | 5 | 6 | 7 | 9 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22
        | 23 | 24 | 25 | 26 | 27 | 28 => return 1 as libc::c_int,
        0 | 1 | 4 | 3 | 8 | 10 => return 0 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-misc.c\0" as *const u8 as *const libc::c_char,
                    463 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"cairo_bool_t _cairo_operator_bounded_by_source(cairo_operator_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_operator_bounded_by_either(
    mut op: cairo_operator_t,
) -> uint32_t {
    match op as libc::c_uint {
        2 | 5 | 6 | 7 | 9 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22
        | 23 | 24 | 25 | 26 | 27 | 28 => {
            return (CAIRO_OPERATOR_BOUND_BY_MASK as libc::c_int
                | CAIRO_OPERATOR_BOUND_BY_SOURCE as libc::c_int) as uint32_t;
        }
        0 | 1 => return CAIRO_OPERATOR_BOUND_BY_MASK as libc::c_int as uint32_t,
        4 | 3 | 8 | 10 => return 0 as libc::c_int as uint32_t,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-misc.c\0" as *const u8 as *const libc::c_char,
                    505 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 61],
                        &[libc::c_char; 61],
                    >(b"uint32_t _cairo_operator_bounded_by_either(cairo_operator_t)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int as uint32_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_half_from_float(mut f: libc::c_float) -> uint16_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    let mut s: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    u.f = f;
    s = (u.ui >> 16 as libc::c_int & 0x8000 as libc::c_int as libc::c_uint)
        as libc::c_int;
    e = (u.ui >> 23 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        .wrapping_sub((127 as libc::c_int - 15 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    m = (u.ui & 0x7fffff as libc::c_int as libc::c_uint) as libc::c_int;
    if e <= 0 as libc::c_int {
        if e < -(10 as libc::c_int) {
            return 0 as libc::c_int as uint16_t;
        }
        m = (m | 0x800000 as libc::c_int) >> 1 as libc::c_int - e;
        if m & 0x1000 as libc::c_int != 0 {
            m += 0x2000 as libc::c_int;
        }
        return (s | m >> 13 as libc::c_int) as uint16_t;
    } else if e == 0xff as libc::c_int - (127 as libc::c_int - 15 as libc::c_int) {
        if m == 0 as libc::c_int {
            return (s | 0x7c00 as libc::c_int) as uint16_t
        } else {
            m >>= 13 as libc::c_int;
            return (s | 0x7c00 as libc::c_int | m
                | (m == 0 as libc::c_int) as libc::c_int) as uint16_t;
        }
    } else {
        if m & 0x1000 as libc::c_int != 0 {
            m += 0x2000 as libc::c_int;
            if m & 0x800000 as libc::c_int != 0 {
                m = 0 as libc::c_int;
                e += 1 as libc::c_int;
            }
        }
        if e > 30 as libc::c_int {
            return (s | 0x7c00 as libc::c_int) as uint16_t;
        }
        return (s | e << 10 as libc::c_int | m >> 13 as libc::c_int) as uint16_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_get_locale_decimal_point() -> *const libc::c_char {
    let mut locale_data: *mut lconv = localeconv();
    return (*locale_data).decimal_point;
}
static mut C_locale: locale_t = 0 as *const __locale_struct as *mut __locale_struct;
unsafe extern "C" fn get_C_locale() -> locale_t {
    let mut C: locale_t = 0 as *mut __locale_struct;
    loop {
        C = _cairo_atomic_ptr_get(
            &mut C_locale as *mut locale_t as *mut *mut libc::c_void,
        ) as locale_t;
        if !C.is_null() {
            break;
        }
        C = newlocale(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 12 as libc::c_int,
            b"C\0" as *const u8 as *const libc::c_char,
            0 as locale_t,
        );
        if !(_cairo_atomic_ptr_cmpxchg_impl(
            &mut C_locale as *mut locale_t as *mut *mut libc::c_void,
            0 as *mut libc::c_void,
            C as *mut libc::c_void,
        ) == 0)
        {
            break;
        }
        freelocale(C_locale);
    }
    return C;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_strtod(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
) -> libc::c_double {
    return strtod_l(nptr, endptr, get_C_locale());
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_fopen(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut file_out: *mut *mut FILE,
) -> cairo_status_t {
    let mut result: *mut FILE = 0 as *mut FILE;
    result = fopen(filename, mode);
    *file_out = result;
    return CAIRO_STATUS_SUCCESS;
}
static mut _cairo_intern_string_ht: *mut cairo_hash_table_t = 0
    as *const cairo_hash_table_t as *mut cairo_hash_table_t;
#[no_mangle]
pub unsafe extern "C" fn _cairo_string_hash(
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) -> libc::c_ulong {
    let mut p: *const libc::c_schar = str as *const libc::c_schar;
    let mut h: libc::c_uint = *p as libc::c_uint;
    p = p.offset(1 as libc::c_int as isize);
    while len > 0 as libc::c_int {
        h = (h << 5 as libc::c_int).wrapping_sub(h).wrapping_add(*p as libc::c_uint);
        len -= 1;
        p = p.offset(1);
    }
    return h as libc::c_ulong;
}
unsafe extern "C" fn _intern_string_equal(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_intern_string_t = _a as *const cairo_intern_string_t;
    let mut b: *const cairo_intern_string_t = _b as *const cairo_intern_string_t;
    if (*a).len != (*b).len {
        return 0 as libc::c_int;
    }
    return (memcmp(
        (*a).string as *const libc::c_void,
        (*b).string as *const libc::c_void,
        (*a).len as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_intern_string(
    mut str_inout: *mut *const libc::c_char,
    mut len: libc::c_int,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut str: *mut libc::c_char = *str_inout as *mut libc::c_char;
    let mut tmpl: cairo_intern_string_t = cairo_intern_string_t {
        hash_entry: cairo_hash_entry_t { hash: 0 },
        len: 0,
        string: 0 as *mut libc::c_char,
    };
    let mut istring: *mut cairo_intern_string_t = 0 as *mut cairo_intern_string_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if len < 0 as libc::c_int {
        len = strlen(str) as libc::c_int;
    }
    tmpl.hash_entry.hash = _cairo_string_hash(str, len);
    tmpl.len = len;
    tmpl.string = str;
    pthread_mutex_lock(&mut _cairo_intern_string_mutex);
    if _cairo_intern_string_ht.is_null() {
        _cairo_intern_string_ht = _cairo_hash_table_create(
            Some(
                _intern_string_equal
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> cairo_bool_t,
            ),
        );
        if _cairo_intern_string_ht.is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            current_block = 4186910337947001759;
        } else {
            current_block = 13586036798005543211;
        }
    } else {
        current_block = 13586036798005543211;
    }
    match current_block {
        13586036798005543211 => {
            istring = _cairo_hash_table_lookup(
                _cairo_intern_string_ht,
                &mut tmpl.hash_entry,
            ) as *mut cairo_intern_string_t;
            if istring.is_null() {
                istring = (if (::std::mem::size_of::<cairo_intern_string_t>()
                    as libc::c_ulong)
                    .wrapping_add(len as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    != 0 as libc::c_int as libc::c_ulong
                {
                    malloc(
                        (::std::mem::size_of::<cairo_intern_string_t>() as libc::c_ulong)
                            .wrapping_add(len as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    )
                } else {
                    0 as *mut libc::c_void
                }) as *mut cairo_intern_string_t;
                if !istring.is_null() {
                    (*istring).hash_entry.hash = tmpl.hash_entry.hash;
                    (*istring).len = tmpl.len;
                    let ref mut fresh3 = (*istring).string;
                    *fresh3 = istring.offset(1 as libc::c_int as isize)
                        as *mut libc::c_char;
                    memcpy(
                        (*istring).string as *mut libc::c_void,
                        str as *const libc::c_void,
                        len as libc::c_ulong,
                    );
                    *((*istring).string)
                        .offset(len as isize) = '\0' as i32 as libc::c_char;
                    status = _cairo_hash_table_insert(
                        _cairo_intern_string_ht,
                        &mut (*istring).hash_entry,
                    );
                    if status as u64 != 0 {
                        free(istring as *mut libc::c_void);
                        current_block = 4186910337947001759;
                    } else {
                        current_block = 14818589718467733107;
                    }
                } else {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    current_block = 4186910337947001759;
                }
            } else {
                current_block = 14818589718467733107;
            }
            match current_block {
                4186910337947001759 => {}
                _ => {
                    *str_inout = (*istring).string;
                }
            }
        }
        _ => {}
    }
    pthread_mutex_unlock(&mut _cairo_intern_string_mutex);
    return status;
}
unsafe extern "C" fn _intern_string_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    _cairo_hash_table_remove(
        closure as *mut cairo_hash_table_t,
        entry as *mut cairo_hash_entry_t,
    );
    free(entry);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_intern_string_reset_static_data() {
    pthread_mutex_lock(&mut _cairo_intern_string_mutex);
    if !_cairo_intern_string_ht.is_null() {
        _cairo_hash_table_foreach(
            _cairo_intern_string_ht,
            Some(
                _intern_string_pluck
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            _cairo_intern_string_ht as *mut libc::c_void,
        );
        _cairo_hash_table_destroy(_cairo_intern_string_ht);
        _cairo_intern_string_ht = 0 as *mut cairo_hash_table_t;
    }
    pthread_mutex_unlock(&mut _cairo_intern_string_mutex);
}
