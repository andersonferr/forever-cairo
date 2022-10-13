use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type uint16_t = __uint16_t;
pub type lzw_buf_t = _lzw_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lzw_buf {
    pub status: cairo_status_t,
    pub data: *mut libc::c_uchar,
    pub data_size: libc::c_int,
    pub num_data: libc::c_int,
    pub pending: uint32_t,
    pub pending_bits: libc::c_uint,
}
pub type lzw_symbol_table_t = _lzw_symbol_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lzw_symbol_table {
    pub table: [lzw_symbol_t; 9013],
}
pub type lzw_symbol_t = uint32_t;
unsafe extern "C" fn _lzw_buf_init(mut buf: *mut lzw_buf_t, mut size: libc::c_int) {
    if size == 0 as libc::c_int {
        size = 16 as libc::c_int;
    }
    (*buf).status = CAIRO_STATUS_SUCCESS;
    (*buf).data_size = size;
    (*buf).num_data = 0 as libc::c_int;
    (*buf).pending = 0 as libc::c_int as uint32_t;
    (*buf).pending_bits = 0 as libc::c_int as libc::c_uint;
    let ref mut fresh0 = (*buf).data;
    *fresh0 = (if size != 0 as libc::c_int {
        malloc(size as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if ((*buf).data).is_null() {
        (*buf).data_size = 0 as libc::c_int;
        (*buf).status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return;
    }
}
unsafe extern "C" fn _lzw_buf_grow(mut buf: *mut lzw_buf_t) -> cairo_status_t {
    let mut new_size: libc::c_int = (*buf).data_size * 2 as libc::c_int;
    let mut new_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*buf).status as u64 != 0 {
        return (*buf).status;
    }
    new_data = 0 as *mut libc::c_uchar;
    if new_size / 2 as libc::c_int == (*buf).data_size {
        new_data = realloc((*buf).data as *mut libc::c_void, new_size as libc::c_ulong)
            as *mut libc::c_uchar;
    }
    if new_data.is_null() {
        free((*buf).data as *mut libc::c_void);
        (*buf).data_size = 0 as libc::c_int;
        (*buf).status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return (*buf).status;
    }
    let ref mut fresh1 = (*buf).data;
    *fresh1 = new_data;
    (*buf).data_size = new_size;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _lzw_buf_store_bits(
    mut buf: *mut lzw_buf_t,
    mut value: uint16_t,
    mut num_bits: libc::c_int,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if value as libc::c_int <= ((1 as libc::c_int) << num_bits) - 1 as libc::c_int
    {} else {
        __assert_fail(
            b"value <= (1 << num_bits) - 1\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-lzw.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void _lzw_buf_store_bits(lzw_buf_t *, uint16_t, int)\0"))
                .as_ptr(),
        );
    }
    if (*buf).status as u64 != 0 {
        return;
    }
    (*buf).pending = (*buf).pending << num_bits | value as libc::c_uint;
    let ref mut fresh2 = (*buf).pending_bits;
    *fresh2 = (*fresh2).wrapping_add(num_bits as libc::c_uint);
    while (*buf).pending_bits >= 8 as libc::c_int as libc::c_uint {
        if (*buf).num_data >= (*buf).data_size {
            status = _lzw_buf_grow(buf);
            if status as u64 != 0 {
                return;
            }
        }
        let ref mut fresh3 = (*buf).num_data;
        let fresh4 = *fresh3;
        *fresh3 = *fresh3 + 1;
        *((*buf).data)
            .offset(
                fresh4 as isize,
            ) = ((*buf).pending
            >> ((*buf).pending_bits).wrapping_sub(8 as libc::c_int as libc::c_uint))
            as libc::c_uchar;
        let ref mut fresh5 = (*buf).pending_bits;
        *fresh5 = (*fresh5).wrapping_sub(8 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn _lzw_buf_store_pending(mut buf: *mut lzw_buf_t) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*buf).status as u64 != 0 {
        return;
    }
    if (*buf).pending_bits == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*buf).pending_bits < 8 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"buf->pending_bits < 8\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-lzw.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void _lzw_buf_store_pending(lzw_buf_t *)\0"))
                .as_ptr(),
        );
    }
    if (*buf).num_data >= (*buf).data_size {
        status = _lzw_buf_grow(buf);
        if status as u64 != 0 {
            return;
        }
    }
    let ref mut fresh6 = (*buf).num_data;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    *((*buf).data)
        .offset(
            fresh7 as isize,
        ) = ((*buf).pending
        << (8 as libc::c_int as libc::c_uint).wrapping_sub((*buf).pending_bits))
        as libc::c_uchar;
    (*buf).pending_bits = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn _lzw_symbol_table_init(mut table: *mut lzw_symbol_table_t) {
    memset(
        ((*table).table).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (9013 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lzw_symbol_t>() as libc::c_ulong),
    );
}
unsafe extern "C" fn _lzw_symbol_table_lookup(
    mut table: *mut lzw_symbol_table_t,
    mut symbol: lzw_symbol_t,
    mut slot_ret: *mut *mut lzw_symbol_t,
) -> cairo_bool_t {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut hash: libc::c_int = (symbol & 0xfffff as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut candidate: lzw_symbol_t = 0;
    idx = hash % 9013 as libc::c_int;
    step = 0 as libc::c_int;
    *slot_ret = 0 as *mut lzw_symbol_t;
    i = 0 as libc::c_int;
    while i < 9013 as libc::c_int {
        candidate = (*table).table[idx as usize];
        if candidate == 0 as libc::c_int as libc::c_uint {
            *slot_ret = &mut *((*table).table).as_mut_ptr().offset(idx as isize)
                as *mut lzw_symbol_t;
            return 0 as libc::c_int;
        } else {
            if candidate & 0xfffff as libc::c_int as libc::c_uint
                == symbol & 0xfffff as libc::c_int as libc::c_uint
            {
                *slot_ret = &mut *((*table).table).as_mut_ptr().offset(idx as isize)
                    as *mut lzw_symbol_t;
                return 1 as libc::c_int;
            }
        }
        if step == 0 as libc::c_int {
            step = hash % 9011 as libc::c_int;
            if step == 0 as libc::c_int {
                step = 1 as libc::c_int;
            }
        }
        idx += step;
        if idx >= 9013 as libc::c_int {
            idx -= 9013 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_lzw_compress(
    mut data: *mut libc::c_uchar,
    mut size_in_out: *mut libc::c_ulong,
) -> *mut libc::c_uchar {
    let mut bytes_remaining: libc::c_int = *size_in_out as libc::c_int;
    let mut buf: lzw_buf_t = lzw_buf_t {
        status: CAIRO_STATUS_SUCCESS,
        data: 0 as *mut libc::c_uchar,
        data_size: 0,
        num_data: 0,
        pending: 0,
        pending_bits: 0,
    };
    let mut table: lzw_symbol_table_t = lzw_symbol_table_t {
        table: [0; 9013],
    };
    let mut symbol: lzw_symbol_t = 0;
    let mut slot: *mut lzw_symbol_t = 0 as *mut lzw_symbol_t;
    let mut code_next: libc::c_int = 258 as libc::c_int;
    let mut code_bits: libc::c_int = 9 as libc::c_int;
    let mut prev: libc::c_int = 0;
    let mut next: libc::c_int = 0 as libc::c_int;
    if *size_in_out == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_uchar;
    }
    _lzw_buf_init(&mut buf, *size_in_out as libc::c_int);
    _lzw_symbol_table_init(&mut table);
    _lzw_buf_store_bits(&mut buf, 256 as libc::c_int as uint16_t, code_bits);
    loop {
        let fresh8 = data;
        data = data.offset(1);
        prev = *fresh8 as libc::c_int;
        bytes_remaining -= 1;
        if bytes_remaining != 0 {
            loop {
                let fresh9 = data;
                data = data.offset(1);
                next = *fresh9 as libc::c_int;
                bytes_remaining -= 1;
                symbol = (prev << 8 as libc::c_int | next) as lzw_symbol_t;
                if _lzw_symbol_table_lookup(&mut table, symbol, &mut slot) != 0 {
                    prev = (*slot >> 20 as libc::c_int) as libc::c_int;
                }
                if !(bytes_remaining != 0 && *slot != 0 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
            if *slot == 0 as libc::c_int as libc::c_uint {
                data = data.offset(-1);
                bytes_remaining += 1;
            }
        }
        _lzw_buf_store_bits(&mut buf, prev as uint16_t, code_bits);
        if bytes_remaining == 0 as libc::c_int {
            break;
        }
        let fresh10 = code_next;
        code_next = code_next + 1;
        *slot = (fresh10 << 20 as libc::c_int | prev << 8 as libc::c_int | next)
            as lzw_symbol_t;
        if code_next > ((1 as libc::c_int) << code_bits) - 1 as libc::c_int {
            code_bits += 1;
            if code_bits > 12 as libc::c_int {
                _lzw_symbol_table_init(&mut table);
                _lzw_buf_store_bits(
                    &mut buf,
                    256 as libc::c_int as uint16_t,
                    code_bits - 1 as libc::c_int,
                );
                code_bits = 9 as libc::c_int;
                code_next = 258 as libc::c_int;
            }
        }
    }
    _lzw_buf_store_bits(&mut buf, 257 as libc::c_int as uint16_t, code_bits);
    _lzw_buf_store_pending(&mut buf);
    if buf.status as libc::c_uint
        == CAIRO_STATUS_NO_MEMORY as libc::c_int as libc::c_uint
    {
        *size_in_out = 0 as libc::c_int as libc::c_ulong;
        return 0 as *mut libc::c_uchar;
    }
    if buf.status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"buf.status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-lzw.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"unsigned char *_cairo_lzw_compress(unsigned char *, unsigned long *)\0"))
                .as_ptr(),
        );
    }
    *size_in_out = buf.num_data as libc::c_ulong;
    return buf.data;
}
