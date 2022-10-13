use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type uint16_t = __uint16_t;
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
static mut utf8_skip_data: [libc::c_char; 256] = [
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn _utf8_get_char(mut p: *const libc::c_uchar) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut mask: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut result: uint32_t = 0;
    let mut c: libc::c_uchar = *p;
    if (c as libc::c_int) < 128 as libc::c_int {
        len = 1 as libc::c_int;
        mask = 0x7f as libc::c_int;
    } else if c as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
        len = 2 as libc::c_int;
        mask = 0x1f as libc::c_int;
    } else if c as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
        len = 3 as libc::c_int;
        mask = 0xf as libc::c_int;
    } else if c as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
        len = 4 as libc::c_int;
        mask = 0x7 as libc::c_int;
    } else if c as libc::c_int & 0xfc as libc::c_int == 0xf8 as libc::c_int {
        len = 5 as libc::c_int;
        mask = 0x3 as libc::c_int;
    } else if c as libc::c_int & 0xfe as libc::c_int == 0xfc as libc::c_int {
        len = 6 as libc::c_int;
        mask = 0x1 as libc::c_int;
    } else {
        len = -(1 as libc::c_int);
    }
    if len == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as uint32_t;
    }
    result = (*p.offset(0 as libc::c_int as isize) as libc::c_int & mask) as uint32_t;
    i = 1 as libc::c_int;
    while i < len {
        if *p.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int
            != 0x80 as libc::c_int
        {
            result = -(1 as libc::c_int) as uint32_t;
            break;
        } else {
            result <<= 6 as libc::c_int;
            result
                |= (*p.offset(i as isize) as libc::c_int & 0x3f as libc::c_int)
                    as libc::c_uint;
            i += 1;
        }
    }
    return result;
}
unsafe extern "C" fn _utf8_get_char_extended(
    mut p: *const libc::c_uchar,
    mut max_len: libc::c_long,
) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut wc: uint32_t = *p as uint32_t;
    if wc < 0x80 as libc::c_int as libc::c_uint {
        return wc
    } else {
        if wc < 0xc0 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int) as uint32_t
        } else {
            if wc < 0xe0 as libc::c_int as libc::c_uint {
                len = 2 as libc::c_int;
                wc &= 0x1f as libc::c_int as libc::c_uint;
            } else if wc < 0xf0 as libc::c_int as libc::c_uint {
                len = 3 as libc::c_int;
                wc &= 0xf as libc::c_int as libc::c_uint;
            } else if wc < 0xf8 as libc::c_int as libc::c_uint {
                len = 4 as libc::c_int;
                wc &= 0x7 as libc::c_int as libc::c_uint;
            } else if wc < 0xfc as libc::c_int as libc::c_uint {
                len = 5 as libc::c_int;
                wc &= 0x3 as libc::c_int as libc::c_uint;
            } else if wc < 0xfe as libc::c_int as libc::c_uint {
                len = 6 as libc::c_int;
                wc &= 0x1 as libc::c_int as libc::c_uint;
            } else {
                return -(1 as libc::c_int) as uint32_t
            }
        }
    }
    if max_len >= 0 as libc::c_int as libc::c_long && len as libc::c_long > max_len {
        i = 1 as libc::c_int;
        while (i as libc::c_long) < max_len {
            if *(p as *mut libc::c_uchar).offset(i as isize) as libc::c_int
                & 0xc0 as libc::c_int != 0x80 as libc::c_int
            {
                return -(1 as libc::c_int) as uint32_t;
            }
            i += 1;
        }
        return -(2 as libc::c_int) as uint32_t;
    }
    i = 1 as libc::c_int;
    while i < len {
        let mut ch: uint32_t = *(p as *mut libc::c_uchar).offset(i as isize) as uint32_t;
        if ch & 0xc0 as libc::c_int as libc::c_uint
            != 0x80 as libc::c_int as libc::c_uint
        {
            if ch != 0 {
                return -(1 as libc::c_int) as uint32_t
            } else {
                return -(2 as libc::c_int) as uint32_t
            }
        }
        wc <<= 6 as libc::c_int;
        wc |= ch & 0x3f as libc::c_int as libc::c_uint;
        i += 1;
    }
    if (if wc < 0x80 as libc::c_int as libc::c_uint {
        1 as libc::c_int
    } else {
        (if wc < 0x800 as libc::c_int as libc::c_uint {
            2 as libc::c_int
        } else {
            (if wc < 0x10000 as libc::c_int as libc::c_uint {
                3 as libc::c_int
            } else {
                (if wc < 0x200000 as libc::c_int as libc::c_uint {
                    4 as libc::c_int
                } else {
                    (if wc < 0x4000000 as libc::c_int as libc::c_uint {
                        5 as libc::c_int
                    } else {
                        6 as libc::c_int
                    })
                })
            })
        })
    }) != len
    {
        return -(1 as libc::c_int) as uint32_t;
    }
    return wc;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_utf8_get_char_validated(
    mut p: *const libc::c_char,
    mut unicode: *mut uint32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mask: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut result: uint32_t = 0;
    let mut c: libc::c_uchar = *p as libc::c_uchar;
    if (c as libc::c_int) < 128 as libc::c_int {
        len = 1 as libc::c_int;
        mask = 0x7f as libc::c_int;
    } else if c as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
        len = 2 as libc::c_int;
        mask = 0x1f as libc::c_int;
    } else if c as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
        len = 3 as libc::c_int;
        mask = 0xf as libc::c_int;
    } else if c as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
        len = 4 as libc::c_int;
        mask = 0x7 as libc::c_int;
    } else if c as libc::c_int & 0xfc as libc::c_int == 0xf8 as libc::c_int {
        len = 5 as libc::c_int;
        mask = 0x3 as libc::c_int;
    } else if c as libc::c_int & 0xfe as libc::c_int == 0xfc as libc::c_int {
        len = 6 as libc::c_int;
        mask = 0x1 as libc::c_int;
    } else {
        len = -(1 as libc::c_int);
    }
    if len == -(1 as libc::c_int) {
        if !unicode.is_null() {
            *unicode = -(1 as libc::c_int) as uint32_t;
        }
        return 1 as libc::c_int;
    }
    result = (*p.offset(0 as libc::c_int as isize) as libc::c_int & mask) as uint32_t;
    i = 1 as libc::c_int;
    while i < len {
        if *p.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int
            != 0x80 as libc::c_int
        {
            result = -(1 as libc::c_int) as uint32_t;
            break;
        } else {
            result <<= 6 as libc::c_int;
            result
                |= (*p.offset(i as isize) as libc::c_int & 0x3f as libc::c_int)
                    as libc::c_uint;
            i += 1;
        }
    }
    if !unicode.is_null() {
        *unicode = result;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_utf8_to_ucs4(
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut result: *mut *mut uint32_t,
    mut items_written: *mut libc::c_int,
) -> cairo_status_t {
    let mut str32: *mut uint32_t = 0 as *mut uint32_t;
    let mut n_chars: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut in_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let ustr: *const libc::c_uchar = str as *const libc::c_uchar;
    in_0 = ustr;
    n_chars = 0 as libc::c_int;
    while (len < 0 as libc::c_int
        || ustr.offset(len as isize).offset_from(in_0) as libc::c_long
            > 0 as libc::c_int as libc::c_long) && *in_0 as libc::c_int != 0
    {
        let mut wc: uint32_t = _utf8_get_char_extended(
            in_0,
            ustr.offset(len as isize).offset_from(in_0) as libc::c_long,
        );
        if wc & 0x80000000 as libc::c_uint != 0
            || !(wc < 0x110000 as libc::c_int as libc::c_uint
                && wc & 0xfffff800 as libc::c_uint
                    != 0xd800 as libc::c_int as libc::c_uint)
        {
            return _cairo_error(CAIRO_STATUS_INVALID_STRING);
        }
        n_chars += 1;
        if n_chars == 2147483647 as libc::c_int {
            return _cairo_error(CAIRO_STATUS_INVALID_STRING);
        }
        in_0 = in_0
            .offset(
                utf8_skip_data[*(in_0 as *mut libc::c_uchar) as usize] as libc::c_int
                    as isize,
            );
    }
    if !result.is_null() {
        str32 = _cairo_malloc_ab(
            (n_chars + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        if str32.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        in_0 = ustr;
        i = 0 as libc::c_int;
        while i < n_chars {
            *str32.offset(i as isize) = _utf8_get_char(in_0);
            in_0 = in_0
                .offset(
                    utf8_skip_data[*(in_0 as *mut libc::c_uchar) as usize] as libc::c_int
                        as isize,
                );
            i += 1;
        }
        *str32.offset(i as isize) = 0 as libc::c_int as uint32_t;
        *result = str32;
    }
    if !items_written.is_null() {
        *items_written = n_chars;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_ucs4_to_utf8(
    mut unicode: uint32_t,
    mut utf8: *mut libc::c_char,
) -> libc::c_int {
    let mut bytes: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if unicode < 0x80 as libc::c_int as libc::c_uint {
        if !utf8.is_null() {
            *utf8 = unicode as libc::c_char;
        }
        return 1 as libc::c_int;
    } else {
        if unicode < 0x800 as libc::c_int as libc::c_uint {
            bytes = 2 as libc::c_int;
        } else if unicode < 0x10000 as libc::c_int as libc::c_uint {
            bytes = 3 as libc::c_int;
        } else if unicode < 0x200000 as libc::c_int as libc::c_uint {
            bytes = 4 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
    }
    if utf8.is_null() {
        return bytes;
    }
    p = utf8.offset(bytes as isize);
    while p > utf8 {
        p = p.offset(-1);
        *p = (0x80 as libc::c_int as libc::c_uint
            | unicode & 0x3f as libc::c_int as libc::c_uint) as libc::c_char;
        unicode >>= 6 as libc::c_int;
    }
    *p = (*p as libc::c_int | (0xf0 as libc::c_int) << 4 as libc::c_int - bytes)
        as libc::c_char;
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_ucs4_to_utf16(
    mut unicode: uint32_t,
    mut utf16: *mut uint16_t,
) -> libc::c_int {
    if unicode < 0x10000 as libc::c_int as libc::c_uint {
        if !utf16.is_null() {
            *utf16.offset(0 as libc::c_int as isize) = unicode as uint16_t;
        }
        return 1 as libc::c_int;
    } else if unicode < 0x110000 as libc::c_int as libc::c_uint {
        if !utf16.is_null() {
            *utf16
                .offset(
                    0 as libc::c_int as isize,
                ) = unicode
                .wrapping_sub(0x10000 as libc::c_int as libc::c_uint)
                .wrapping_div(0x400 as libc::c_int as libc::c_uint)
                .wrapping_add(0xd800 as libc::c_int as libc::c_uint) as uint16_t;
            *utf16
                .offset(
                    1 as libc::c_int as isize,
                ) = unicode
                .wrapping_sub(0x10000 as libc::c_int as libc::c_uint)
                .wrapping_rem(0x400 as libc::c_int as libc::c_uint)
                .wrapping_add(0xdc00 as libc::c_int as libc::c_uint) as uint16_t;
        }
        return 2 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_utf8_to_utf16(
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut result: *mut *mut uint16_t,
    mut items_written: *mut libc::c_int,
) -> cairo_status_t {
    let mut str16: *mut uint16_t = 0 as *mut uint16_t;
    let mut n16: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut in_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let ustr: *const libc::c_uchar = str as *const libc::c_uchar;
    in_0 = ustr;
    n16 = 0 as libc::c_int;
    while (len < 0 as libc::c_int
        || ustr.offset(len as isize).offset_from(in_0) as libc::c_long
            > 0 as libc::c_int as libc::c_long) && *in_0 as libc::c_int != 0
    {
        let mut wc: uint32_t = _utf8_get_char_extended(
            in_0,
            ustr.offset(len as isize).offset_from(in_0) as libc::c_long,
        );
        if wc & 0x80000000 as libc::c_uint != 0
            || !(wc < 0x110000 as libc::c_int as libc::c_uint
                && wc & 0xfffff800 as libc::c_uint
                    != 0xd800 as libc::c_int as libc::c_uint)
        {
            return _cairo_error(CAIRO_STATUS_INVALID_STRING);
        }
        if wc < 0x10000 as libc::c_int as libc::c_uint {
            n16 += 1 as libc::c_int;
        } else {
            n16 += 2 as libc::c_int;
        }
        if n16 == 2147483647 as libc::c_int - 1 as libc::c_int
            || n16 == 2147483647 as libc::c_int
        {
            return _cairo_error(CAIRO_STATUS_INVALID_STRING);
        }
        in_0 = in_0
            .offset(
                utf8_skip_data[*(in_0 as *mut libc::c_uchar) as usize] as libc::c_int
                    as isize,
            );
    }
    str16 = _cairo_malloc_ab(
        (n16 + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    if str16.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    in_0 = ustr;
    i = 0 as libc::c_int;
    while i < n16 {
        let mut wc_0: uint32_t = _utf8_get_char(in_0);
        i += _cairo_ucs4_to_utf16(wc_0, str16.offset(i as isize));
        in_0 = in_0
            .offset(
                utf8_skip_data[*(in_0 as *mut libc::c_uchar) as usize] as libc::c_int
                    as isize,
            );
    }
    *str16.offset(i as isize) = 0 as libc::c_int as uint16_t;
    *result = str16;
    if !items_written.is_null() {
        *items_written = n16;
    }
    return CAIRO_STATUS_SUCCESS;
}
