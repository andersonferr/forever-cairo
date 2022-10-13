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
pub type cairo_base85_stream_t = _cairo_base85_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_base85_stream {
    pub base: cairo_output_stream_t,
    pub output: *mut cairo_output_stream_t,
    pub four_tuple: [libc::c_uchar; 4],
    pub pending: libc::c_int,
}
unsafe extern "C" fn _expand_four_tuple_to_five(
    mut four_tuple: *mut libc::c_uchar,
    mut five_tuple: *mut libc::c_uchar,
    mut all_zero: *mut cairo_bool_t,
) {
    let mut value: uint32_t = 0;
    let mut digit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    value = (*four_tuple.offset(0 as libc::c_int as isize) as uint32_t)
        << 24 as libc::c_int
        | ((*four_tuple.offset(1 as libc::c_int as isize) as libc::c_int)
            << 16 as libc::c_int) as libc::c_uint
        | ((*four_tuple.offset(2 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int) as libc::c_uint
        | *four_tuple.offset(3 as libc::c_int as isize) as libc::c_uint;
    if !all_zero.is_null() {
        *all_zero = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        digit = value.wrapping_rem(85 as libc::c_int as libc::c_uint) as libc::c_int;
        if digit != 0 as libc::c_int && !all_zero.is_null() {
            *all_zero = 0 as libc::c_int;
        }
        *five_tuple
            .offset(
                (4 as libc::c_int - i) as isize,
            ) = (digit + 33 as libc::c_int) as libc::c_uchar;
        value = value.wrapping_div(85 as libc::c_int as libc::c_uint);
        i += 1;
    }
}
unsafe extern "C" fn _cairo_base85_stream_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut cairo_base85_stream_t = base as *mut cairo_base85_stream_t;
    let mut ptr: *const libc::c_uchar = data;
    let mut five_tuple: [libc::c_uchar; 5] = [0; 5];
    let mut is_zero: cairo_bool_t = 0;
    while length != 0 {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        let ref mut fresh1 = (*stream).pending;
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        (*stream).four_tuple[fresh2 as usize] = *fresh0;
        length = length.wrapping_sub(1);
        if (*stream).pending == 4 as libc::c_int {
            _expand_four_tuple_to_five(
                ((*stream).four_tuple).as_mut_ptr(),
                five_tuple.as_mut_ptr(),
                &mut is_zero,
            );
            if is_zero != 0 {
                _cairo_output_stream_write(
                    (*stream).output,
                    b"z\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            } else {
                _cairo_output_stream_write(
                    (*stream).output,
                    five_tuple.as_mut_ptr() as *const libc::c_void,
                    5 as libc::c_int as size_t,
                );
            }
            (*stream).pending = 0 as libc::c_int;
        }
    }
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _cairo_base85_stream_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut cairo_base85_stream_t = base as *mut cairo_base85_stream_t;
    let mut five_tuple: [libc::c_uchar; 5] = [0; 5];
    if (*stream).pending != 0 {
        memset(
            ((*stream).four_tuple).as_mut_ptr().offset((*stream).pending as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int - (*stream).pending) as libc::c_ulong,
        );
        _expand_four_tuple_to_five(
            ((*stream).four_tuple).as_mut_ptr(),
            five_tuple.as_mut_ptr(),
            0 as *mut cairo_bool_t,
        );
        _cairo_output_stream_write(
            (*stream).output,
            five_tuple.as_mut_ptr() as *const libc::c_void,
            ((*stream).pending + 1 as libc::c_int) as size_t,
        );
    }
    return _cairo_output_stream_get_status((*stream).output);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_base85_stream_create(
    mut output: *mut cairo_output_stream_t,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut cairo_base85_stream_t = 0 as *mut cairo_base85_stream_t;
    if (*output).status as u64 != 0 {
        return _cairo_output_stream_create_in_error((*output).status);
    }
    stream = (if ::std::mem::size_of::<cairo_base85_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_base85_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_base85_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            _cairo_base85_stream_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            _cairo_base85_stream_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh3 = (*stream).output;
    *fresh3 = output;
    (*stream).pending = 0 as libc::c_int;
    return &mut (*stream).base;
}
