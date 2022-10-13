use ::libc;
extern "C" {
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xcb_create_pixmap(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_create_gc(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: uint32_t,
        value_list: *const libc::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: uint32_t,
        value_list: *const libc::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_copy_area(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_poly_fill_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    fn xcb_put_image(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        left_pad: uint8_t,
        depth: uint8_t,
        data_len: uint32_t,
        data: *const uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_get_image(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        plane_mask: uint32_t,
    ) -> xcb_get_image_cookie_t;
    fn xcb_get_image_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_image_reply_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_send_request(
        c: *mut xcb_connection_t,
        flags: libc::c_int,
        vector: *mut iovec,
        request: *const xcb_protocol_request_t,
    ) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
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
pub type cairo_bool_t = libc::c_int;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
pub type cairo_user_data_array_t = cairo_array_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_reference_count_t {
    pub ref_count: cairo_atomic_int_t,
}
pub type cairo_atomic_int_t = libc::c_int;
pub type cairo_device_t = _cairo_device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_device_backend_t,
    pub mutex: cairo_recursive_mutex_t,
    pub mutex_depth: libc::c_uint,
    pub finished: cairo_bool_t,
}
pub type cairo_recursive_mutex_t = cairo_recursive_mutex_impl_t;
pub type cairo_recursive_mutex_impl_t = pthread_mutex_t;
pub type cairo_device_backend_t = _cairo_device_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device_backend {
    pub type_0: cairo_device_type_t,
    pub lock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub unlock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type cairo_device_type_t = _cairo_device_type;
pub type _cairo_device_type = libc::c_int;
pub const CAIRO_DEVICE_TYPE_INVALID: _cairo_device_type = -1;
pub const CAIRO_DEVICE_TYPE_WIN32: _cairo_device_type = 7;
pub const CAIRO_DEVICE_TYPE_COGL: _cairo_device_type = 6;
pub const CAIRO_DEVICE_TYPE_XML: _cairo_device_type = 5;
pub const CAIRO_DEVICE_TYPE_XLIB: _cairo_device_type = 4;
pub const CAIRO_DEVICE_TYPE_XCB: _cairo_device_type = 3;
pub const CAIRO_DEVICE_TYPE_SCRIPT: _cairo_device_type = 2;
pub const CAIRO_DEVICE_TYPE_GL: _cairo_device_type = 1;
pub const CAIRO_DEVICE_TYPE_DRM: _cairo_device_type = 0;
pub type uint32_t = __uint32_t;
pub type cairo_hash_table_t = _cairo_hash_table;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_pixmap_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_rectangle_t {
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4],
}
pub type xcb_image_format_t = libc::c_uint;
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_put_image_request_t {
    pub major_opcode: uint8_t,
    pub format: uint8_t,
    pub length: uint16_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
    pub left_pad: uint8_t,
    pub depth: uint8_t,
    pub pad0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub pad0: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_extension_t {
    pub name: *const libc::c_char,
    pub global_id: libc::c_int,
}
pub type xcb_render_sub_pixel_t = libc::c_uint;
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 5;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 4;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 3;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 2;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 1;
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0;
pub type xcb_render_pictformat_t = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_protocol_request_t {
    pub count: size_t,
    pub ext: *mut xcb_extension_t,
    pub opcode: uint8_t,
    pub isvoid: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_connection {
    pub device: cairo_device_t,
    pub xcb_connection: *mut xcb_connection_t,
    pub standard_formats: [xcb_render_pictformat_t; 5],
    pub xrender_formats: *mut cairo_hash_table_t,
    pub visual_to_xrender_format: *mut cairo_hash_table_t,
    pub maximum_request_length: libc::c_uint,
    pub flags: libc::c_uint,
    pub original_flags: libc::c_uint,
    pub force_precision: libc::c_int,
    pub root: *const xcb_setup_t,
    pub render: *const xcb_query_extension_reply_t,
    pub shm: *const xcb_query_extension_reply_t,
    pub subpixel_orders: *mut xcb_render_sub_pixel_t,
    pub shm_mutex: cairo_mutex_t,
    pub shm_pools: cairo_list_t,
    pub shm_pending: cairo_list_t,
    pub shm_info_freelist: cairo_freepool_t,
    pub screens_mutex: cairo_mutex_t,
    pub screens: cairo_list_t,
    pub fonts: cairo_list_t,
    pub link: cairo_list_t,
}
pub type cairo_xcb_connection_t = _cairo_xcb_connection;
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_create_pixmap(
    mut connection: *mut cairo_xcb_connection_t,
    mut depth: uint8_t,
    mut drawable: xcb_drawable_t,
    mut width: uint16_t,
    mut height: uint16_t,
) -> xcb_pixmap_t {
    let mut pixmap: xcb_pixmap_t = xcb_generate_id((*connection).xcb_connection);
    if width as libc::c_int > 0 as libc::c_int {} else {
        __assert_fail(
            b"width > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-connection-core.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 120],
                &[libc::c_char; 120],
            >(
                b"xcb_pixmap_t _cairo_xcb_connection_create_pixmap(cairo_xcb_connection_t *, uint8_t, xcb_drawable_t, uint16_t, uint16_t)\0",
            ))
                .as_ptr(),
        );
    }
    if height as libc::c_int > 0 as libc::c_int {} else {
        __assert_fail(
            b"height > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-connection-core.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 120],
                &[libc::c_char; 120],
            >(
                b"xcb_pixmap_t _cairo_xcb_connection_create_pixmap(cairo_xcb_connection_t *, uint8_t, xcb_drawable_t, uint16_t, uint16_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_create_pixmap(
        (*connection).xcb_connection,
        depth,
        pixmap,
        drawable,
        width,
        height,
    );
    return pixmap;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_create_gc(
    mut connection: *mut cairo_xcb_connection_t,
    mut drawable: xcb_drawable_t,
    mut value_mask: uint32_t,
    mut values: *mut uint32_t,
) -> xcb_gcontext_t {
    let mut gc: xcb_gcontext_t = xcb_generate_id((*connection).xcb_connection);
    xcb_create_gc(
        (*connection).xcb_connection,
        gc,
        drawable,
        value_mask,
        values as *const libc::c_void,
    );
    return gc;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_change_gc(
    mut connection: *mut cairo_xcb_connection_t,
    mut gc: xcb_gcontext_t,
    mut value_mask: uint32_t,
    mut values: *mut uint32_t,
) {
    xcb_change_gc(
        (*connection).xcb_connection,
        gc,
        value_mask,
        values as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_copy_area(
    mut connection: *mut cairo_xcb_connection_t,
    mut src: xcb_drawable_t,
    mut dst: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut dst_x: int16_t,
    mut dst_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
) {
    xcb_copy_area(
        (*connection).xcb_connection,
        src,
        dst,
        gc,
        src_x,
        src_y,
        dst_x,
        dst_y,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_poly_fill_rectangle(
    mut connection: *mut cairo_xcb_connection_t,
    mut dst: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut num_rectangles: uint32_t,
    mut rectangles: *mut xcb_rectangle_t,
) {
    xcb_poly_fill_rectangle(
        (*connection).xcb_connection,
        dst,
        gc,
        num_rectangles,
        rectangles,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_put_image(
    mut connection: *mut cairo_xcb_connection_t,
    mut dst: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut width: uint16_t,
    mut height: uint16_t,
    mut dst_x: int16_t,
    mut dst_y: int16_t,
    mut depth: uint8_t,
    mut stride: uint32_t,
    mut data: *mut libc::c_void,
) {
    let req_size: uint32_t = 18 as libc::c_int as uint32_t;
    let mut length: uint32_t = (height as libc::c_uint).wrapping_mul(stride);
    let mut len: uint32_t = req_size.wrapping_add(length) >> 2 as libc::c_int;
    if len < (*connection).maximum_request_length {
        xcb_put_image(
            (*connection).xcb_connection,
            XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t,
            dst,
            gc,
            width,
            height,
            dst_x,
            dst_y,
            0 as libc::c_int as uint8_t,
            depth,
            length,
            data as *const uint8_t,
        );
    } else {
        let mut rows: libc::c_int = ((*connection).maximum_request_length)
            .wrapping_sub(req_size)
            .wrapping_sub(4 as libc::c_int as libc::c_uint)
            .wrapping_div(stride) as libc::c_int;
        if rows > 0 as libc::c_int {
            loop {
                if rows > height as libc::c_int {
                    rows = height as libc::c_int;
                }
                length = (rows as libc::c_uint).wrapping_mul(stride);
                xcb_put_image(
                    (*connection).xcb_connection,
                    XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t,
                    dst,
                    gc,
                    width,
                    rows as uint16_t,
                    dst_x,
                    dst_y,
                    0 as libc::c_int as uint8_t,
                    depth,
                    length,
                    data as *const uint8_t,
                );
                height = (height as libc::c_int - rows) as uint16_t;
                dst_y = (dst_y as libc::c_int + rows) as int16_t;
                data = (data as *mut libc::c_char).offset(length as isize)
                    as *mut libc::c_void;
                if !(height != 0) {
                    break;
                }
            }
        } else if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-connection-core.c\0" as *const u8
                    as *const libc::c_char,
                141 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 160],
                    &[libc::c_char; 160],
                >(
                    b"void _cairo_xcb_connection_put_image(cairo_xcb_connection_t *, xcb_drawable_t, xcb_gcontext_t, uint16_t, uint16_t, int16_t, int16_t, uint8_t, uint32_t, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn _cairo_xcb_connection_do_put_subimage(
    mut connection: *mut cairo_xcb_connection_t,
    mut dst: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
    mut cpp: uint16_t,
    mut stride: libc::c_int,
    mut dst_x: int16_t,
    mut dst_y: int16_t,
    mut depth: uint8_t,
    mut _data: *mut libc::c_void,
) {
    let mut xcb_req: xcb_protocol_request_t = {
        let mut init = xcb_protocol_request_t {
            count: 0 as libc::c_int as size_t,
            ext: 0 as *mut xcb_extension_t,
            opcode: 72 as libc::c_int as uint8_t,
            isvoid: 1 as libc::c_int as uint8_t,
        };
        init
    };
    let mut req: xcb_put_image_request_t = xcb_put_image_request_t {
        major_opcode: 0,
        format: 0,
        length: 0,
        drawable: 0,
        gc: 0,
        width: 0,
        height: 0,
        dst_x: 0,
        dst_y: 0,
        left_pad: 0,
        depth: 0,
        pad0: [0; 2],
    };
    let mut vec_stack: [iovec; 128] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 128];
    let mut vec: *mut iovec = vec_stack.as_mut_ptr();
    let mut len: uint32_t = 0 as libc::c_int as uint32_t;
    let mut data: *mut uint8_t = _data as *mut uint8_t;
    let mut n: libc::c_int = 3 as libc::c_int;
    let mut entries_needed: libc::c_int = height as libc::c_int + 2 as libc::c_int
        + 2 as libc::c_int;
    req.format = XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t;
    req.drawable = dst;
    req.gc = gc;
    req.width = width;
    req.height = height;
    req.dst_x = dst_x;
    req.dst_y = dst_y;
    req.left_pad = 0 as libc::c_int as uint8_t;
    req.depth = depth;
    req.pad0[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    req.pad0[1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    if entries_needed
        > (::std::mem::size_of::<[iovec; 128]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<iovec>() as libc::c_ulong) as libc::c_int
    {
        vec = _cairo_malloc_ab(
            entries_needed as size_t,
            ::std::mem::size_of::<iovec>() as libc::c_ulong,
        ) as *mut iovec;
        if vec.is_null() {
            return;
        }
    }
    data = data
        .offset(
            (src_y as libc::c_int * stride + src_x as libc::c_int * cpp as libc::c_int)
                as isize,
        );
    let ref mut fresh2 = (*vec.offset(2 as libc::c_int as isize)).iov_base;
    *fresh2 = &mut req as *mut xcb_put_image_request_t as *mut libc::c_char
        as *mut libc::c_void;
    (*vec.offset(2 as libc::c_int as isize))
        .iov_len = ::std::mem::size_of::<xcb_put_image_request_t>() as libc::c_ulong;
    loop {
        let fresh3 = height;
        height = height.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        let ref mut fresh4 = (*vec.offset(n as isize)).iov_base;
        *fresh4 = data as *mut libc::c_void;
        (*vec.offset(n as isize))
            .iov_len = (cpp as libc::c_int * width as libc::c_int) as size_t;
        len = (len as libc::c_uint)
            .wrapping_add((cpp as libc::c_int * width as libc::c_int) as libc::c_uint)
            as uint32_t as uint32_t;
        data = data.offset(stride as isize);
        n += 1;
    }
    let ref mut fresh5 = (*vec.offset(n as isize)).iov_base;
    *fresh5 = 0 as *mut libc::c_void;
    (*vec.offset(n as isize))
        .iov_len = (len.wrapping_neg() & 3 as libc::c_int as libc::c_uint) as size_t;
    n += 1;
    if n == entries_needed {} else {
        __assert_fail(
            b"n == entries_needed\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-connection-core.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 189],
                &[libc::c_char; 189],
            >(
                b"void _cairo_xcb_connection_do_put_subimage(cairo_xcb_connection_t *, xcb_drawable_t, xcb_gcontext_t, int16_t, int16_t, uint16_t, uint16_t, uint16_t, int, int16_t, int16_t, uint8_t, void *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_req.count = (n - 2 as libc::c_int) as size_t;
    xcb_send_request(
        (*connection).xcb_connection,
        0 as libc::c_int,
        &mut *vec.offset(2 as libc::c_int as isize),
        &mut xcb_req,
    );
    if vec != vec_stack.as_mut_ptr() {
        free(vec as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_put_subimage(
    mut connection: *mut cairo_xcb_connection_t,
    mut dst: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
    mut cpp: uint16_t,
    mut stride: libc::c_int,
    mut dst_x: int16_t,
    mut dst_y: int16_t,
    mut depth: uint8_t,
    mut _data: *mut libc::c_void,
) {
    let req_size: uint32_t = ::std::mem::size_of::<xcb_put_image_request_t>()
        as libc::c_ulong as uint32_t;
    let mut length: uint32_t = (height as libc::c_int * cpp as libc::c_int
        * width as libc::c_int) as uint32_t;
    let mut len: uint32_t = req_size.wrapping_add(length) >> 2 as libc::c_int;
    if len < (*connection).maximum_request_length {
        _cairo_xcb_connection_do_put_subimage(
            connection,
            dst,
            gc,
            src_x,
            src_y,
            width,
            height,
            cpp,
            stride,
            dst_x,
            dst_y,
            depth,
            _data,
        );
    } else {
        let mut rows: libc::c_int = ((*connection).maximum_request_length)
            .wrapping_sub(req_size)
            .wrapping_sub(4 as libc::c_int as libc::c_uint)
            .wrapping_div((cpp as libc::c_int * width as libc::c_int) as libc::c_uint)
            as libc::c_int;
        if rows > 0 as libc::c_int {
            loop {
                if rows > height as libc::c_int {
                    rows = height as libc::c_int;
                }
                _cairo_xcb_connection_do_put_subimage(
                    connection,
                    dst,
                    gc,
                    src_x,
                    src_y,
                    width,
                    rows as uint16_t,
                    cpp,
                    stride,
                    dst_x,
                    dst_y,
                    depth,
                    _data,
                );
                height = (height as libc::c_int - rows) as uint16_t;
                dst_y = (dst_y as libc::c_int + rows) as int16_t;
                _data = (_data as *mut libc::c_char).offset((stride * rows) as isize)
                    as *mut libc::c_void;
                if !(height != 0) {
                    break;
                }
            }
        } else if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-connection-core.c\0" as *const u8
                    as *const libc::c_char,
                263 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 186],
                    &[libc::c_char; 186],
                >(
                    b"void _cairo_xcb_connection_put_subimage(cairo_xcb_connection_t *, xcb_drawable_t, xcb_gcontext_t, int16_t, int16_t, uint16_t, uint16_t, uint16_t, int, int16_t, int16_t, uint8_t, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_get_image(
    mut connection: *mut cairo_xcb_connection_t,
    mut src: xcb_drawable_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
) -> *mut xcb_get_image_reply_t {
    return xcb_get_image_reply(
        (*connection).xcb_connection,
        xcb_get_image(
            (*connection).xcb_connection,
            XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t,
            src,
            src_x,
            src_y,
            width,
            height,
            -(1 as libc::c_int) as uint32_t,
        ),
        0 as *mut *mut xcb_generic_error_t,
    );
}
