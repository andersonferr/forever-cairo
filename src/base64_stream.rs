use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    static _cairo_output_stream_nil: cairo_output_stream_t;
    fn _cairo_output_stream_init(
        stream: *mut cairo_output_stream_t,
        write_func: cairo_output_stream_write_func_t,
        flush_func: cairo_output_stream_flush_func_t,
        close_func: cairo_output_stream_close_func_t,
    );
    fn _cairo_output_stream_create_in_error(
        status: cairo_status_t,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_write(
        stream: *mut cairo_output_stream_t,
        data: *const libc::c_void,
        length: size_t,
    );
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
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
pub struct _cairo_output_stream {
    pub write_func: cairo_output_stream_write_func_t,
    pub flush_func: cairo_output_stream_flush_func_t,
    pub close_func: cairo_output_stream_close_func_t,
    pub position: libc::c_longlong,
    pub status: cairo_status_t,
    pub closed: cairo_bool_t,
}
pub type cairo_output_stream_close_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
>;
pub type cairo_output_stream_t = _cairo_output_stream;
pub type cairo_output_stream_flush_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
>;
pub type cairo_output_stream_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_output_stream_t,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
pub type cairo_base64_stream_t = _cairo_base64_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_base64_stream {
    pub base: cairo_output_stream_t,
    pub output: *mut cairo_output_stream_t,
    pub in_mem: libc::c_uint,
    pub trailing: libc::c_uint,
    pub src: [libc::c_uchar; 3],
}
static mut base64_table: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &[libc::c_char; 64],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
};
unsafe extern "C" fn _cairo_base64_stream_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut cairo_base64_stream_t = base as *mut cairo_base64_stream_t;
    let mut src: *mut libc::c_uchar = ((*stream).src).as_mut_ptr();
    let mut i: libc::c_uint = 0;
    if ((*stream).in_mem).wrapping_add(length) < 3 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as libc::c_uint;
        while i < length {
            let fresh0 = data;
            data = data.offset(1);
            *src.offset(i.wrapping_add((*stream).in_mem) as isize) = *fresh0;
            i = i.wrapping_add(1);
        }
        let ref mut fresh1 = (*stream).in_mem;
        *fresh1 = (*fresh1).wrapping_add(length);
        return CAIRO_STATUS_SUCCESS;
    }
    loop {
        let mut dst: [libc::c_uchar; 4] = [0; 4];
        i = (*stream).in_mem;
        while i < 3 as libc::c_int as libc::c_uint {
            let fresh2 = data;
            data = data.offset(1);
            *src.offset(i as isize) = *fresh2;
            length = length.wrapping_sub(1);
            i = i.wrapping_add(1);
        }
        (*stream).in_mem = 0 as libc::c_int as libc::c_uint;
        dst[0 as libc::c_int
            as usize] = base64_table[(*src.offset(0 as libc::c_int as isize)
            as libc::c_int >> 2 as libc::c_int) as usize] as libc::c_uchar;
        dst[1 as libc::c_int
            as usize] = base64_table[((*src.offset(0 as libc::c_int as isize)
            as libc::c_int & 0x3 as libc::c_int) << 4 as libc::c_int
            | *src.offset(1 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int)
            as usize] as libc::c_uchar;
        dst[2 as libc::c_int
            as usize] = base64_table[((*src.offset(1 as libc::c_int as isize)
            as libc::c_int & 0xf as libc::c_int) << 2 as libc::c_int
            | *src.offset(2 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int)
            as usize] as libc::c_uchar;
        dst[3 as libc::c_int
            as usize] = base64_table[(*src.offset(2 as libc::c_int as isize)
            as libc::c_int & 0xfc as libc::c_int >> 2 as libc::c_int) as usize]
            as libc::c_uchar;
        let mut current_block_19: u64;
        match (*stream).trailing {
            2 => {
                dst[2 as libc::c_int as usize] = '=' as i32 as libc::c_uchar;
                current_block_19 = 2588195503434756995;
            }
            1 => {
                current_block_19 = 2588195503434756995;
            }
            _ => {
                current_block_19 = 11307063007268554308;
            }
        }
        match current_block_19 {
            2588195503434756995 => {
                dst[3 as libc::c_int as usize] = '=' as i32 as libc::c_uchar;
            }
            _ => {}
        }
        _cairo_output_stream_write(
            (*stream).output,
            dst.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        if !(length >= 3 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let fresh3 = data;
        data = data.offset(1);
        *src.offset(i as isize) = *fresh3;
        i = i.wrapping_add(1);
    }
    (*stream).in_mem = length;
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _cairo_base64_stream_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut cairo_base64_stream_t = base as *mut cairo_base64_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stream).in_mem > 0 as libc::c_int as libc::c_uint {
        memset(
            ((*stream).src).as_mut_ptr().offset((*stream).in_mem as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (3 as libc::c_int as libc::c_uint).wrapping_sub((*stream).in_mem)
                as libc::c_ulong,
        );
        (*stream)
            .trailing = (3 as libc::c_int as libc::c_uint)
            .wrapping_sub((*stream).in_mem);
        (*stream).in_mem = 3 as libc::c_int as libc::c_uint;
        status = _cairo_base64_stream_write(
            base,
            0 as *const libc::c_uchar,
            0 as libc::c_int as libc::c_uint,
        );
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_base64_stream_create(
    mut output: *mut cairo_output_stream_t,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut cairo_base64_stream_t = 0 as *mut cairo_base64_stream_t;
    if (*output).status as u64 != 0 {
        return _cairo_output_stream_create_in_error((*output).status);
    }
    stream = (if ::std::mem::size_of::<cairo_base64_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_base64_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_base64_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            _cairo_base64_stream_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            _cairo_base64_stream_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh4 = (*stream).output;
    *fresh4 = output;
    (*stream).in_mem = 0 as libc::c_int as libc::c_uint;
    (*stream).trailing = 0 as libc::c_int as libc::c_uint;
    return &mut (*stream).base;
}
