use ::libc;
extern "C" {
    pub type internal_state;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
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
    fn deflateInit_(
        strm: z_streamp,
        level: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
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
pub type cairo_deflate_stream_t = _cairo_deflate_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_deflate_stream {
    pub base: cairo_output_stream_t,
    pub output: *mut cairo_output_stream_t,
    pub zlib_stream: z_stream,
    pub input_buf: [libc::c_uchar; 16384],
    pub output_buf: [libc::c_uchar; 16384],
}
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = libc::c_ulong;
pub type voidpf = *mut libc::c_void;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type uInt = libc::c_uint;
pub type Bytef = Byte;
pub type Byte = libc::c_uchar;
pub type z_streamp = *mut z_stream;
unsafe extern "C" fn cairo_deflate_stream_deflate(
    mut stream: *mut cairo_deflate_stream_t,
    mut flush: cairo_bool_t,
) {
    let mut ret: libc::c_int = 0;
    let mut finished: cairo_bool_t = 0;
    loop {
        ret = deflate(
            &mut (*stream).zlib_stream,
            if flush != 0 { 4 as libc::c_int } else { 0 as libc::c_int },
        );
        if flush != 0
            || (*stream).zlib_stream.avail_out == 0 as libc::c_int as libc::c_uint
        {
            _cairo_output_stream_write(
                (*stream).output,
                ((*stream).output_buf).as_mut_ptr() as *const libc::c_void,
                (16384 as libc::c_int as libc::c_uint)
                    .wrapping_sub((*stream).zlib_stream.avail_out) as size_t,
            );
            let ref mut fresh0 = (*stream).zlib_stream.next_out;
            *fresh0 = ((*stream).output_buf).as_mut_ptr();
            (*stream).zlib_stream.avail_out = 16384 as libc::c_int as uInt;
        }
        finished = 1 as libc::c_int;
        if (*stream).zlib_stream.avail_in != 0 as libc::c_int as libc::c_uint {
            finished = 0 as libc::c_int;
        }
        if flush != 0 && ret != 1 as libc::c_int {
            finished = 0 as libc::c_int;
        }
        if !(finished == 0) {
            break;
        }
    }
    let ref mut fresh1 = (*stream).zlib_stream.next_in;
    *fresh1 = ((*stream).input_buf).as_mut_ptr();
}
unsafe extern "C" fn _cairo_deflate_stream_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut cairo_deflate_stream_t = base as *mut cairo_deflate_stream_t;
    let mut count: libc::c_uint = 0;
    let mut p: *const libc::c_uchar = data;
    while length != 0 {
        count = length;
        if count
            > (16384 as libc::c_int as libc::c_uint)
                .wrapping_sub((*stream).zlib_stream.avail_in)
        {
            count = (16384 as libc::c_int as libc::c_uint)
                .wrapping_sub((*stream).zlib_stream.avail_in);
        }
        memcpy(
            ((*stream).input_buf)
                .as_mut_ptr()
                .offset((*stream).zlib_stream.avail_in as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            count as libc::c_ulong,
        );
        p = p.offset(count as isize);
        let ref mut fresh2 = (*stream).zlib_stream.avail_in;
        *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(count) as uInt as uInt;
        length = length.wrapping_sub(count);
        if (*stream).zlib_stream.avail_in == 16384 as libc::c_int as libc::c_uint {
            cairo_deflate_stream_deflate(stream, 0 as libc::c_int);
        }
    }
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _cairo_deflate_stream_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut cairo_deflate_stream_t = base as *mut cairo_deflate_stream_t;
    cairo_deflate_stream_deflate(stream, 1 as libc::c_int);
    deflateEnd(&mut (*stream).zlib_stream);
    return _cairo_output_stream_get_status((*stream).output);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_deflate_stream_create(
    mut output: *mut cairo_output_stream_t,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut cairo_deflate_stream_t = 0 as *mut cairo_deflate_stream_t;
    if (*output).status as u64 != 0 {
        return _cairo_output_stream_create_in_error((*output).status);
    }
    stream = (if ::std::mem::size_of::<cairo_deflate_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_deflate_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_deflate_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            _cairo_deflate_stream_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            _cairo_deflate_stream_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh3 = (*stream).output;
    *fresh3 = output;
    let ref mut fresh4 = (*stream).zlib_stream.zalloc;
    *fresh4 = None;
    let ref mut fresh5 = (*stream).zlib_stream.zfree;
    *fresh5 = None;
    let ref mut fresh6 = (*stream).zlib_stream.opaque;
    *fresh6 = 0 as voidpf;
    if deflateInit_(
        &mut (*stream).zlib_stream,
        -(1 as libc::c_int),
        b"1.2.11\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    ) != 0 as libc::c_int
    {
        free(stream as *mut libc::c_void);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    let ref mut fresh7 = (*stream).zlib_stream.next_in;
    *fresh7 = ((*stream).input_buf).as_mut_ptr();
    (*stream).zlib_stream.avail_in = 0 as libc::c_int as uInt;
    let ref mut fresh8 = (*stream).zlib_stream.next_out;
    *fresh8 = ((*stream).output_buf).as_mut_ptr();
    (*stream).zlib_stream.avail_out = 16384 as libc::c_int as uInt;
    return &mut (*stream).base;
}
