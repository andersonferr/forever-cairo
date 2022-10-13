use ::libc;
extern "C" {
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_shm_attach(
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shmid: uint32_t,
        read_only: uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_shm_detach(
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
    ) -> xcb_void_cookie_t;
    fn xcb_shm_put_image(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        total_width: uint16_t,
        total_height: uint16_t,
        src_x: uint16_t,
        src_y: uint16_t,
        src_width: uint16_t,
        src_height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        format: uint8_t,
        send_event: uint8_t,
        shmseg: xcb_shm_seg_t,
        offset: uint32_t,
    ) -> xcb_void_cookie_t;
    fn xcb_shm_get_image(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        plane_mask: uint32_t,
        format: uint8_t,
        shmseg: xcb_shm_seg_t,
        offset: uint32_t,
    ) -> xcb_shm_get_image_cookie_t;
    fn xcb_shm_get_image_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_shm_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_get_image_reply_t;
}
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
pub type xcb_gcontext_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const CAIRO_XCB_SHM_MASK: C2RustUnnamed = 2147483648;
pub const CAIRO_XCB_RENDER_MASK: C2RustUnnamed = 4095;
pub const CAIRO_XCB_HAS_SHM: C2RustUnnamed = 2147483648;
pub const CAIRO_XCB_RENDER_HAS_FILTER_BEST: C2RustUnnamed = 2048;
pub const CAIRO_XCB_RENDER_HAS_FILTER_GOOD: C2RustUnnamed = 1024;
pub const CAIRO_XCB_RENDER_HAS_GRADIENTS: C2RustUnnamed = 512;
pub const CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT: C2RustUnnamed = 256;
pub const CAIRO_XCB_RENDER_HAS_PDF_OPERATORS: C2RustUnnamed = 128;
pub const CAIRO_XCB_RENDER_HAS_FILTERS: C2RustUnnamed = 64;
pub const CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM: C2RustUnnamed = 32;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS: C2RustUnnamed = 16;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS: C2RustUnnamed = 8;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE: C2RustUnnamed = 4;
pub const CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES: C2RustUnnamed = 2;
pub const CAIRO_XCB_HAS_RENDER: C2RustUnnamed = 1;
pub type xcb_shm_seg_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shm_get_image_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shm_get_image_cookie_t {
    pub sequence: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_shm_attach(
    mut connection: *mut cairo_xcb_connection_t,
    mut id: uint32_t,
    mut readonly: cairo_bool_t,
) -> uint32_t {
    let mut segment: uint32_t = xcb_generate_id((*connection).xcb_connection);
    if (*connection).flags & CAIRO_XCB_HAS_SHM as libc::c_uint != 0 {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_SHM\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-shm.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"uint32_t _cairo_xcb_connection_shm_attach(cairo_xcb_connection_t *, uint32_t, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_shm_attach((*connection).xcb_connection, segment, id, readonly as uint8_t);
    return segment;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_shm_put_image(
    mut connection: *mut cairo_xcb_connection_t,
    mut dst: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut total_width: uint16_t,
    mut total_height: uint16_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
    mut dst_x: int16_t,
    mut dst_y: int16_t,
    mut depth: uint8_t,
    mut shm: uint32_t,
    mut offset: uint32_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_SHM as libc::c_uint != 0 {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_SHM\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-shm.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 204],
                &[libc::c_char; 204],
            >(
                b"void _cairo_xcb_connection_shm_put_image(cairo_xcb_connection_t *, xcb_drawable_t, xcb_gcontext_t, uint16_t, uint16_t, int16_t, int16_t, uint16_t, uint16_t, int16_t, int16_t, uint8_t, uint32_t, uint32_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_shm_put_image(
        (*connection).xcb_connection,
        dst,
        gc,
        total_width,
        total_height,
        src_x as uint16_t,
        src_y as uint16_t,
        width,
        height,
        dst_x,
        dst_y,
        depth,
        XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        shm,
        offset,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_shm_get_image(
    mut connection: *mut cairo_xcb_connection_t,
    mut src: xcb_drawable_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
    mut shmseg: uint32_t,
    mut offset: uint32_t,
) -> cairo_status_t {
    let mut reply: *mut xcb_shm_get_image_reply_t = 0 as *mut xcb_shm_get_image_reply_t;
    if (*connection).flags & CAIRO_XCB_HAS_SHM as libc::c_uint != 0 {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_SHM\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-shm.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 151],
                &[libc::c_char; 151],
            >(
                b"cairo_status_t _cairo_xcb_connection_shm_get_image(cairo_xcb_connection_t *, xcb_drawable_t, int16_t, int16_t, uint16_t, uint16_t, uint32_t, uint32_t)\0",
            ))
                .as_ptr(),
        );
    }
    reply = xcb_shm_get_image_reply(
        (*connection).xcb_connection,
        xcb_shm_get_image(
            (*connection).xcb_connection,
            src,
            src_x,
            src_y,
            width,
            height,
            -(1 as libc::c_int) as uint32_t,
            XCB_IMAGE_FORMAT_Z_PIXMAP as libc::c_int as uint8_t,
            shmseg,
            offset,
        ),
        0 as *mut *mut xcb_generic_error_t,
    );
    free(reply as *mut libc::c_void);
    if reply.is_null() {
        return _cairo_error(CAIRO_STATUS_READ_ERROR);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_shm_detach(
    mut connection: *mut cairo_xcb_connection_t,
    mut segment: uint32_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_SHM as libc::c_uint != 0 {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_SHM\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-shm.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void _cairo_xcb_connection_shm_detach(cairo_xcb_connection_t *, uint32_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_shm_detach((*connection).xcb_connection, segment);
}
