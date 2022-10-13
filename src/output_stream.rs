use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_get_locale_decimal_point() -> *const libc::c_char;
    fn _cairo_fopen(
        filename: *const libc::c_char,
        mode: *const libc::c_char,
        file_out: *mut *mut FILE,
    ) -> cairo_status_t;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_append_multiple(
        array: *mut cairo_array_t,
        elements: *const libc::c_void,
        num_elements: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub type cairo_matrix_t = _cairo_matrix;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_matrix {
    pub xx: libc::c_double,
    pub yx: libc::c_double,
    pub xy: libc::c_double,
    pub yy: libc::c_double,
    pub x0: libc::c_double,
    pub y0: libc::c_double,
}
pub type cairo_array_t = _cairo_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_array {
    pub size: libc::c_uint,
    pub num_elements: libc::c_uint,
    pub element_size: libc::c_uint,
    pub elements: *mut libc::c_char,
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
pub type cairo_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
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
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
pub type cairo_output_stream_with_closure_t = _cairo_output_stream_with_closure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_output_stream_with_closure {
    pub base: cairo_output_stream_t,
    pub write_func: cairo_write_func_t,
    pub close_func: cairo_close_func_t,
    pub closure: *mut libc::c_void,
}
pub const LENGTH_MODIFIER_LONG_LONG: C2RustUnnamed = 512;
pub const LENGTH_MODIFIER_LONG: C2RustUnnamed = 256;
pub type stdio_stream_t = _stdio_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stdio_stream {
    pub base: cairo_output_stream_t,
    pub file: *mut FILE,
}
pub type memory_stream_t = _memory_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _memory_stream {
    pub base: cairo_output_stream_t,
    pub array: cairo_array_t,
}
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn _cairo_isdigit(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_init(
    mut stream: *mut cairo_output_stream_t,
    mut write_func: cairo_output_stream_write_func_t,
    mut flush_func: cairo_output_stream_flush_func_t,
    mut close_func: cairo_output_stream_close_func_t,
) {
    let ref mut fresh0 = (*stream).write_func;
    *fresh0 = write_func;
    let ref mut fresh1 = (*stream).flush_func;
    *fresh1 = flush_func;
    let ref mut fresh2 = (*stream).close_func;
    *fresh2 = close_func;
    (*stream).position = 0 as libc::c_int as libc::c_longlong;
    (*stream).status = CAIRO_STATUS_SUCCESS;
    (*stream).closed = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_fini(
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    return _cairo_output_stream_close(stream);
}
#[no_mangle]
pub static mut _cairo_output_stream_nil: cairo_output_stream_t = {
    let mut init = _cairo_output_stream {
        write_func: None,
        flush_func: None,
        close_func: None,
        position: 0 as libc::c_int as libc::c_longlong,
        status: CAIRO_STATUS_NO_MEMORY,
        closed: 0 as libc::c_int,
    };
    init
};
static mut _cairo_output_stream_nil_write_error: cairo_output_stream_t = {
    let mut init = _cairo_output_stream {
        write_func: None,
        flush_func: None,
        close_func: None,
        position: 0 as libc::c_int as libc::c_longlong,
        status: CAIRO_STATUS_WRITE_ERROR,
        closed: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn closure_write(
    mut stream: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream_with_closure: *mut cairo_output_stream_with_closure_t = stream
        as *mut cairo_output_stream_with_closure_t;
    if ((*stream_with_closure).write_func).is_none() {
        return CAIRO_STATUS_SUCCESS;
    }
    return ((*stream_with_closure).write_func)
        .expect(
            "non-null function pointer",
        )((*stream_with_closure).closure, data, length);
}
unsafe extern "C" fn closure_close(
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream_with_closure: *mut cairo_output_stream_with_closure_t = stream
        as *mut cairo_output_stream_with_closure_t;
    if ((*stream_with_closure).close_func).is_some() {
        return ((*stream_with_closure).close_func)
            .expect("non-null function pointer")((*stream_with_closure).closure)
    } else {
        return CAIRO_STATUS_SUCCESS
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_create(
    mut write_func: cairo_write_func_t,
    mut close_func: cairo_close_func_t,
    mut closure: *mut libc::c_void,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut cairo_output_stream_with_closure_t = 0
        as *mut cairo_output_stream_with_closure_t;
    stream = (if ::std::mem::size_of::<cairo_output_stream_with_closure_t>()
        as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
    {
        malloc(
            ::std::mem::size_of::<cairo_output_stream_with_closure_t>() as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_output_stream_with_closure_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            closure_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            closure_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh3 = (*stream).write_func;
    *fresh3 = write_func;
    let ref mut fresh4 = (*stream).close_func;
    *fresh4 = close_func;
    let ref mut fresh5 = (*stream).closure;
    *fresh5 = closure;
    return &mut (*stream).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    if status as libc::c_uint == CAIRO_STATUS_NO_MEMORY as libc::c_int as libc::c_uint {
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    if status as libc::c_uint == CAIRO_STATUS_WRITE_ERROR as libc::c_int as libc::c_uint
    {
        return &_cairo_output_stream_nil_write_error as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    stream = (if ::std::mem::size_of::<cairo_output_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_output_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_output_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(stream, None, None, None);
    (*stream).status = status;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_flush(
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stream).closed != 0 {
        return (*stream).status;
    }
    if stream
        == &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t
        || stream
            == &_cairo_output_stream_nil_write_error as *const cairo_output_stream_t
                as *mut cairo_output_stream_t
    {
        return (*stream).status;
    }
    if ((*stream).flush_func).is_some() {
        status = ((*stream).flush_func).expect("non-null function pointer")(stream);
        if (*stream).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            (*stream).status = status;
        }
    }
    return (*stream).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_close(
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*stream).closed != 0 {
        return (*stream).status;
    }
    if stream
        == &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t
        || stream
            == &_cairo_output_stream_nil_write_error as *const cairo_output_stream_t
                as *mut cairo_output_stream_t
    {
        return (*stream).status;
    }
    if ((*stream).close_func).is_some() {
        status = ((*stream).close_func).expect("non-null function pointer")(stream);
        if (*stream).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            (*stream).status = status;
        }
    }
    (*stream).closed = 1 as libc::c_int;
    return (*stream).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_destroy(
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !stream.is_null() {} else {
        __assert_fail(
            b"stream != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-output-stream.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"cairo_status_t _cairo_output_stream_destroy(cairo_output_stream_t *)\0"))
                .as_ptr(),
        );
    }
    if stream
        == &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t
        || stream
            == &_cairo_output_stream_nil_write_error as *const cairo_output_stream_t
                as *mut cairo_output_stream_t
    {
        return (*stream).status;
    }
    status = _cairo_output_stream_fini(stream);
    free(stream as *mut libc::c_void);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_write(
    mut stream: *mut cairo_output_stream_t,
    mut data: *const libc::c_void,
    mut length: size_t,
) {
    if length == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if (*stream).status as u64 != 0 {
        return;
    }
    (*stream)
        .status = ((*stream).write_func)
        .expect(
            "non-null function pointer",
        )(stream, data as *const libc::c_uchar, length as libc::c_uint);
    let ref mut fresh6 = (*stream).position;
    *fresh6 = (*fresh6 as libc::c_ulonglong).wrapping_add(length as libc::c_ulonglong)
        as libc::c_longlong as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_write_hex_string(
    mut stream: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: size_t,
) {
    let hex_chars: [libc::c_char; 17] = *::std::mem::transmute::<
        &[u8; 17],
        &[libc::c_char; 17],
    >(b"0123456789abcdef\0");
    let mut buffer: [libc::c_char; 2] = [0; 2];
    let mut i: libc::c_uint = 0;
    let mut column: libc::c_uint = 0;
    if (*stream).status as u64 != 0 {
        return;
    }
    i = 0 as libc::c_int as libc::c_uint;
    column = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < length {
        if column == 38 as libc::c_int as libc::c_uint {
            _cairo_output_stream_write(
                stream,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            column = 0 as libc::c_int as libc::c_uint;
        }
        buffer[0 as libc::c_int
            as usize] = hex_chars[(*data.offset(i as isize) as libc::c_int
            >> 4 as libc::c_int & 0xf as libc::c_int) as usize];
        buffer[1 as libc::c_int
            as usize] = hex_chars[(*data.offset(i as isize) as libc::c_int
            & 0xf as libc::c_int) as usize];
        _cairo_output_stream_write(
            stream,
            buffer.as_mut_ptr() as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        i = i.wrapping_add(1);
        column = column.wrapping_add(1);
    }
}
unsafe extern "C" fn _cairo_dtostr(
    mut buffer: *mut libc::c_char,
    mut size: size_t,
    mut d: libc::c_double,
    mut limited_precision: cairo_bool_t,
) {
    let mut decimal_point: *const libc::c_char = 0 as *const libc::c_char;
    let mut decimal_point_len: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut decimal_len: libc::c_int = 0;
    let mut num_zeros: libc::c_int = 0;
    let mut decimal_digits: libc::c_int = 0;
    if d == 0.0f64 {
        d = 0.0f64;
    }
    decimal_point = _cairo_get_locale_decimal_point();
    decimal_point_len = strlen(decimal_point) as libc::c_int;
    if decimal_point_len != 0 as libc::c_int {} else {
        __assert_fail(
            b"decimal_point_len != 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-output-stream.c\0" as *const u8 as *const libc::c_char,
            319 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void _cairo_dtostr(char *, size_t, double, cairo_bool_t)\0"))
                .as_ptr(),
        );
    }
    if limited_precision != 0 {
        snprintf(
            buffer,
            size,
            b"%.*f\0" as *const u8 as *const libc::c_char,
            (8 as libc::c_int as libc::c_double * 0.301029996f64
                + 1 as libc::c_int as libc::c_double) as libc::c_int,
            d,
        );
    } else if fabs(d) >= 0.1f64 {
        snprintf(buffer, size, b"%f\0" as *const u8 as *const libc::c_char, d);
    } else {
        snprintf(buffer, size, b"%.18f\0" as *const u8 as *const libc::c_char, d);
        p = buffer;
        if *p as libc::c_int == '+' as i32 || *p as libc::c_int == '-' as i32 {
            p = p.offset(1);
        }
        while _cairo_isdigit(*p as libc::c_int) != 0 {
            p = p.offset(1);
        }
        if strncmp(p, decimal_point, decimal_point_len as libc::c_ulong)
            == 0 as libc::c_int
        {
            p = p.offset(decimal_point_len as isize);
        }
        num_zeros = 0 as libc::c_int;
        loop {
            let fresh7 = p;
            p = p.offset(1);
            if !(*fresh7 as libc::c_int == '0' as i32) {
                break;
            }
            num_zeros += 1;
        }
        decimal_digits = num_zeros + 6 as libc::c_int;
        if decimal_digits < 18 as libc::c_int {
            snprintf(
                buffer,
                size,
                b"%.*f\0" as *const u8 as *const libc::c_char,
                decimal_digits,
                d,
            );
        }
    }
    p = buffer;
    if *p as libc::c_int == '+' as i32 || *p as libc::c_int == '-' as i32 {
        p = p.offset(1);
    }
    while _cairo_isdigit(*p as libc::c_int) != 0 {
        p = p.offset(1);
    }
    if strncmp(p, decimal_point, decimal_point_len as libc::c_ulong) == 0 as libc::c_int
    {
        *p = '.' as i32 as libc::c_char;
        decimal_len = strlen(p.offset(decimal_point_len as isize)) as libc::c_int;
        memmove(
            p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            p.offset(decimal_point_len as isize) as *const libc::c_void,
            decimal_len as libc::c_ulong,
        );
        *p
            .offset(
                (1 as libc::c_int + decimal_len) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        p = p.offset(decimal_len as isize);
        while *p as libc::c_int == '0' as i32 {
            *p = 0 as libc::c_int as libc::c_char;
            p = p.offset(-1);
        }
        if *p as libc::c_int == '.' as i32 {
            *p = 0 as libc::c_int as libc::c_char;
            p = p.offset(-1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_vprintf(
    mut stream: *mut cairo_output_stream_t,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut single_fmt: [libc::c_char; 32] = [0; 32];
    let mut single_fmt_length: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut length_modifier: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut var_width: cairo_bool_t = 0;
    if (*stream).status as u64 != 0 {
        return;
    }
    f = fmt;
    p = buffer.as_mut_ptr();
    while *f as libc::c_int != '\0' as i32 {
        if p
            == buffer
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                        as isize,
                )
        {
            _cairo_output_stream_write(
                stream,
                buffer.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
            p = buffer.as_mut_ptr();
        }
        if *f as libc::c_int != '%' as i32 {
            let fresh8 = f;
            f = f.offset(1);
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = *fresh8;
        } else {
            start = f;
            f = f.offset(1);
            if *f as libc::c_int == '0' as i32 {
                f = f.offset(1);
            }
            var_width = 0 as libc::c_int;
            if *f as libc::c_int == '*' as i32 {
                var_width = 1 as libc::c_int;
                f = f.offset(1);
            }
            while _cairo_isdigit(*f as libc::c_int) != 0 {
                f = f.offset(1);
            }
            length_modifier = 0 as libc::c_int;
            if *f as libc::c_int == 'l' as i32 {
                length_modifier = LENGTH_MODIFIER_LONG as libc::c_int;
                f = f.offset(1);
                if *f as libc::c_int == 'l' as i32 {
                    length_modifier = LENGTH_MODIFIER_LONG_LONG as libc::c_int;
                    f = f.offset(1);
                }
            }
            single_fmt_length = (f.offset_from(start) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int;
            if single_fmt_length + 1 as libc::c_int <= 32 as libc::c_int {} else {
                __assert_fail(
                    b"single_fmt_length + 1 <= SINGLE_FMT_BUFFER_SIZE\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-output-stream.c\0" as *const u8
                        as *const libc::c_char,
                    455 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"void _cairo_output_stream_vprintf(cairo_output_stream_t *, const char *, struct __va_list_tag *)\0",
                    ))
                        .as_ptr(),
                );
            }
            memcpy(
                single_fmt.as_mut_ptr() as *mut libc::c_void,
                start as *const libc::c_void,
                single_fmt_length as libc::c_ulong,
            );
            single_fmt[single_fmt_length as usize] = '\0' as i32 as libc::c_char;
            _cairo_output_stream_write(
                stream,
                buffer.as_mut_ptr() as *const libc::c_void,
                p.offset_from(buffer.as_mut_ptr()) as libc::c_long as size_t,
            );
            match *f as libc::c_int | length_modifier {
                37 => {
                    buffer[0 as libc::c_int as usize] = *f;
                    buffer[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                }
                100 | 117 | 111 | 120 | 88 => {
                    if var_width != 0 {
                        width = ap.arg::<libc::c_int>();
                        snprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                as libc::c_ulong,
                            single_fmt.as_mut_ptr(),
                            width,
                            ap.arg::<libc::c_int>(),
                        );
                    } else {
                        snprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                as libc::c_ulong,
                            single_fmt.as_mut_ptr(),
                            ap.arg::<libc::c_int>(),
                        );
                    }
                }
                356 | 373 | 367 | 376 | 344 => {
                    if var_width != 0 {
                        width = ap.arg::<libc::c_int>();
                        snprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                as libc::c_ulong,
                            single_fmt.as_mut_ptr(),
                            width,
                            ap.arg::<libc::c_long>(),
                        );
                    } else {
                        snprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                as libc::c_ulong,
                            single_fmt.as_mut_ptr(),
                            ap.arg::<libc::c_long>(),
                        );
                    }
                }
                612 | 629 | 623 | 632 | 600 => {
                    if var_width != 0 {
                        width = ap.arg::<libc::c_int>();
                        snprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                as libc::c_ulong,
                            single_fmt.as_mut_ptr(),
                            width,
                            ap.arg::<libc::c_longlong>(),
                        );
                    } else {
                        snprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                as libc::c_ulong,
                            single_fmt.as_mut_ptr(),
                            ap.arg::<libc::c_longlong>(),
                        );
                    }
                }
                115 => {
                    let mut s: *const libc::c_char = ap.arg::<*const libc::c_char>();
                    let mut len: libc::c_int = strlen(s) as libc::c_int;
                    _cairo_output_stream_write(
                        stream,
                        s as *const libc::c_void,
                        len as size_t,
                    );
                    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                }
                102 => {
                    _cairo_dtostr(
                        buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        ap.arg::<libc::c_double>(),
                        0 as libc::c_int,
                    );
                }
                103 => {
                    _cairo_dtostr(
                        buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        ap.arg::<libc::c_double>(),
                        1 as libc::c_int,
                    );
                }
                99 => {
                    buffer[0 as libc::c_int
                        as usize] = ap.arg::<libc::c_int>() as libc::c_char;
                    buffer[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                }
                _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-output-stream.c\0" as *const u8
                                as *const libc::c_char,
                            532 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 97],
                                &[libc::c_char; 97],
                            >(
                                b"void _cairo_output_stream_vprintf(cairo_output_stream_t *, const char *, struct __va_list_tag *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                }
            }
            p = buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as isize);
            f = f.offset(1);
        }
    }
    _cairo_output_stream_write(
        stream,
        buffer.as_mut_ptr() as *const libc::c_void,
        p.offset_from(buffer.as_mut_ptr()) as libc::c_long as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_printf(
    mut stream: *mut cairo_output_stream_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    _cairo_output_stream_vprintf(stream, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_print_matrix(
    mut stream: *mut cairo_output_stream_t,
    mut matrix: *const cairo_matrix_t,
) {
    let mut m: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut s: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    m = *matrix;
    s = fabs(m.xx);
    if fabs(m.xy) > s {
        s = fabs(m.xy);
    }
    if fabs(m.yx) > s {
        s = fabs(m.yx);
    }
    if fabs(m.yy) > s {
        s = fabs(m.yy);
    }
    e = s * 1e-12f64;
    if fabs(m.xx) < e {
        m.xx = 0 as libc::c_int as libc::c_double;
    }
    if fabs(m.xy) < e {
        m.xy = 0 as libc::c_int as libc::c_double;
    }
    if fabs(m.yx) < e {
        m.yx = 0 as libc::c_int as libc::c_double;
    }
    if fabs(m.yy) < e {
        m.yy = 0 as libc::c_int as libc::c_double;
    }
    if fabs(m.x0) < e {
        m.x0 = 0 as libc::c_int as libc::c_double;
    }
    if fabs(m.y0) < e {
        m.y0 = 0 as libc::c_int as libc::c_double;
    }
    _cairo_output_stream_printf(
        stream,
        b"%f %f %f %f %f %f\0" as *const u8 as *const libc::c_char,
        m.xx,
        m.yx,
        m.xy,
        m.yy,
        m.x0,
        m.y0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_get_position(
    mut stream: *mut cairo_output_stream_t,
) -> libc::c_longlong {
    return (*stream).position;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_get_status(
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    return (*stream).status;
}
unsafe extern "C" fn stdio_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut stdio_stream_t = base as *mut stdio_stream_t;
    if fwrite(
        data as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        length as libc::c_ulong,
        (*stream).file,
    ) != length as libc::c_ulong
    {
        return _cairo_error(CAIRO_STATUS_WRITE_ERROR);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn stdio_flush(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut stdio_stream_t = base as *mut stdio_stream_t;
    fflush((*stream).file);
    if ferror((*stream).file) != 0 {
        return _cairo_error(CAIRO_STATUS_WRITE_ERROR)
    } else {
        return CAIRO_STATUS_SUCCESS
    };
}
unsafe extern "C" fn stdio_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut stream: *mut stdio_stream_t = base as *mut stdio_stream_t;
    status = stdio_flush(base);
    fclose((*stream).file);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_create_for_file(
    mut file: *mut FILE,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut stdio_stream_t = 0 as *mut stdio_stream_t;
    if file.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_WRITE_ERROR);
        return &_cairo_output_stream_nil_write_error as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    stream = (if ::std::mem::size_of::<stdio_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<stdio_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut stdio_stream_t;
    if stream.is_null() {
        let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            stdio_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        Some(
            stdio_flush
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
        Some(
            stdio_flush
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh10 = (*stream).file;
    *fresh10 = file;
    return &mut (*stream).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_output_stream_create_for_filename(
    mut filename: *const libc::c_char,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut stdio_stream_t = 0 as *mut stdio_stream_t;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if filename.is_null() {
        return _cairo_null_stream_create();
    }
    status = _cairo_fopen(
        filename,
        b"wb\0" as *const u8 as *const libc::c_char,
        &mut file,
    );
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return _cairo_output_stream_create_in_error(status);
    }
    if file.is_null() {
        match *__errno_location() {
            12 => {
                let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                return &_cairo_output_stream_nil as *const cairo_output_stream_t
                    as *mut cairo_output_stream_t;
            }
            _ => {
                let mut status___0: cairo_status_t = _cairo_error(
                    CAIRO_STATUS_WRITE_ERROR,
                );
                return &_cairo_output_stream_nil_write_error
                    as *const cairo_output_stream_t as *mut cairo_output_stream_t;
            }
        }
    }
    stream = (if ::std::mem::size_of::<stdio_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<stdio_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut stdio_stream_t;
    if stream.is_null() {
        fclose(file);
        let mut status___1: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            stdio_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        Some(
            stdio_flush
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
        Some(
            stdio_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh11 = (*stream).file;
    *fresh11 = file;
    return &mut (*stream).base;
}
unsafe extern "C" fn memory_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut memory_stream_t = base as *mut memory_stream_t;
    return _cairo_array_append_multiple(
        &mut (*stream).array,
        data as *const libc::c_void,
        length,
    );
}
unsafe extern "C" fn memory_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut memory_stream_t = base as *mut memory_stream_t;
    _cairo_array_fini(&mut (*stream).array);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_memory_stream_create() -> *mut cairo_output_stream_t {
    let mut stream: *mut memory_stream_t = 0 as *mut memory_stream_t;
    stream = (if ::std::mem::size_of::<memory_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<memory_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut memory_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            memory_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            memory_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    _cairo_array_init(&mut (*stream).array, 1 as libc::c_int as libc::c_uint);
    return &mut (*stream).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_memory_stream_destroy(
    mut abstract_stream: *mut cairo_output_stream_t,
    mut data_out: *mut *mut libc::c_uchar,
    mut length_out: *mut libc::c_ulong,
) -> cairo_status_t {
    let mut stream: *mut memory_stream_t = 0 as *mut memory_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = (*abstract_stream).status;
    if status as u64 != 0 {
        return _cairo_output_stream_destroy(abstract_stream);
    }
    stream = abstract_stream as *mut memory_stream_t;
    *length_out = _cairo_array_num_elements(&mut (*stream).array) as libc::c_ulong;
    *data_out = (if *length_out != 0 as libc::c_int as libc::c_ulong {
        malloc(*length_out)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if (*data_out).is_null() {
        status = _cairo_output_stream_destroy(abstract_stream);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-output-stream.c\0" as *const u8 as *const libc::c_char,
                775 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"cairo_status_t _cairo_memory_stream_destroy(cairo_output_stream_t *, unsigned char **, unsigned long *)\0",
                ))
                    .as_ptr(),
            );
        }
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    memcpy(
        *data_out as *mut libc::c_void,
        _cairo_array_index(&mut (*stream).array, 0 as libc::c_int as libc::c_uint),
        *length_out,
    );
    return _cairo_output_stream_destroy(abstract_stream);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_memory_stream_copy(
    mut base: *mut cairo_output_stream_t,
    mut dest: *mut cairo_output_stream_t,
) {
    let mut stream: *mut memory_stream_t = base as *mut memory_stream_t;
    if (*dest).status as u64 != 0 {
        return;
    }
    if (*base).status as u64 != 0 {
        (*dest).status = (*base).status;
        return;
    }
    _cairo_output_stream_write(
        dest,
        _cairo_array_index(&mut (*stream).array, 0 as libc::c_int as libc::c_uint),
        _cairo_array_num_elements(&mut (*stream).array) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_memory_stream_length(
    mut base: *mut cairo_output_stream_t,
) -> libc::c_int {
    let mut stream: *mut memory_stream_t = base as *mut memory_stream_t;
    return _cairo_array_num_elements(&mut (*stream).array) as libc::c_int;
}
unsafe extern "C" fn null_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_null_stream_create() -> *mut cairo_output_stream_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    stream = (if ::std::mem::size_of::<cairo_output_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_output_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_output_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        stream,
        Some(
            null_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        None,
    );
    return stream;
}
