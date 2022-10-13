use ::libc;
extern "C" {
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xcb_render_create_picture(
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: uint32_t,
        value_list: *const libc::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_render_change_picture(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: uint32_t,
        value_list: *const libc::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_render_set_picture_clip_rectangles(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        clip_x_origin: int16_t,
        clip_y_origin: int16_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_free_picture(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_composite(
        c: *mut xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        mask: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        src_x: int16_t,
        src_y: int16_t,
        mask_x: int16_t,
        mask_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_trapezoids(
        c: *mut xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: int16_t,
        src_y: int16_t,
        traps_len: uint32_t,
        traps: *const xcb_render_trapezoid_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_composite_glyphs_16(
        c: *mut xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: int16_t,
        src_y: int16_t,
        glyphcmds_len: uint32_t,
        glyphcmds: *const uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_composite_glyphs_8(
        c: *mut xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: int16_t,
        src_y: int16_t,
        glyphcmds_len: uint32_t,
        glyphcmds: *const uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_free_glyphs(
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: uint32_t,
        glyphs: *const xcb_render_glyph_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_create_glyph_set(
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_free_glyph_set(
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_add_glyphs(
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: uint32_t,
        glyphids: *const uint32_t,
        glyphs: *const xcb_render_glyphinfo_t,
        data_len: uint32_t,
        data: *const uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_set_picture_filter(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        filter_len: uint16_t,
        filter: *const libc::c_char,
        values_len: uint32_t,
        values: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_set_picture_transform(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        transform: xcb_render_transform_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_composite_glyphs_32(
        c: *mut xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: int16_t,
        src_y: int16_t,
        glyphcmds_len: uint32_t,
        glyphcmds: *const uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_fill_rectangles(
        c: *mut xcb_connection_t,
        op: uint8_t,
        dst: xcb_render_picture_t,
        color: xcb_render_color_t,
        rects_len: uint32_t,
        rects: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_create_linear_gradient(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        p1: xcb_render_pointfix_t,
        p2: xcb_render_pointfix_t,
        num_stops: uint32_t,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_create_solid_fill(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        color: xcb_render_color_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_create_radial_gradient(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        inner: xcb_render_pointfix_t,
        outer: xcb_render_pointfix_t,
        inner_radius: xcb_render_fixed_t,
        outer_radius: xcb_render_fixed_t,
        num_stops: uint32_t,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t;
    fn xcb_render_create_conical_gradient(
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        center: xcb_render_pointfix_t,
        angle: xcb_render_fixed_t,
        num_stops: uint32_t,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_drawable_t = uint32_t;
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
pub type xcb_render_glyph_t = uint32_t;
pub type xcb_render_glyphset_t = uint32_t;
pub type xcb_render_picture_t = uint32_t;
pub type xcb_render_pictformat_t = uint32_t;
pub type xcb_render_fixed_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_color_t {
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub alpha: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pointfix_t {
    pub x: xcb_render_fixed_t,
    pub y: xcb_render_fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_linefix_t {
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_trapezoid_t {
    pub top: xcb_render_fixed_t,
    pub bottom: xcb_render_fixed_t,
    pub left: xcb_render_linefix_t,
    pub right: xcb_render_linefix_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_glyphinfo_t {
    pub width: uint16_t,
    pub height: uint16_t,
    pub x: int16_t,
    pub y: int16_t,
    pub x_off: int16_t,
    pub y_off: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_transform_t {
    pub matrix11: xcb_render_fixed_t,
    pub matrix12: xcb_render_fixed_t,
    pub matrix13: xcb_render_fixed_t,
    pub matrix21: xcb_render_fixed_t,
    pub matrix22: xcb_render_fixed_t,
    pub matrix23: xcb_render_fixed_t,
    pub matrix31: xcb_render_fixed_t,
    pub matrix32: xcb_render_fixed_t,
    pub matrix33: xcb_render_fixed_t,
}
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_create_picture(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut drawable: xcb_drawable_t,
    mut format: xcb_render_pictformat_t,
    mut value_mask: uint32_t,
    mut value_list: *mut uint32_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 160],
                &[libc::c_char; 160],
            >(
                b"void _cairo_xcb_connection_render_create_picture(cairo_xcb_connection_t *, xcb_render_picture_t, xcb_drawable_t, xcb_render_pictformat_t, uint32_t, uint32_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_create_picture(
        (*connection).xcb_connection,
        picture,
        drawable,
        format,
        value_mask,
        value_list as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_change_picture(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut value_mask: uint32_t,
    mut value_list: *mut uint32_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void _cairo_xcb_connection_render_change_picture(cairo_xcb_connection_t *, xcb_render_picture_t, uint32_t, uint32_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_change_picture(
        (*connection).xcb_connection,
        picture,
        value_mask,
        value_list as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_set_picture_clip_rectangles(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut clip_x_origin: int16_t,
    mut clip_y_origin: int16_t,
    mut rectangles_len: uint32_t,
    mut rectangles: *mut xcb_rectangle_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 157],
                &[libc::c_char; 157],
            >(
                b"void _cairo_xcb_connection_render_set_picture_clip_rectangles(cairo_xcb_connection_t *, xcb_render_picture_t, int16_t, int16_t, uint32_t, xcb_rectangle_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_set_picture_clip_rectangles(
        (*connection).xcb_connection,
        picture,
        clip_x_origin,
        clip_y_origin,
        rectangles_len,
        rectangles,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_free_picture(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void _cairo_xcb_connection_render_free_picture(cairo_xcb_connection_t *, xcb_render_picture_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_free_picture((*connection).xcb_connection, picture);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_composite(
    mut connection: *mut cairo_xcb_connection_t,
    mut op: uint8_t,
    mut src: xcb_render_picture_t,
    mut mask: xcb_render_picture_t,
    mut dst: xcb_render_picture_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut mask_x: int16_t,
    mut mask_y: int16_t,
    mut dst_x: int16_t,
    mut dst_y: int16_t,
    mut width: uint16_t,
    mut height: uint16_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_COMPOSITE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 219],
                &[libc::c_char; 219],
            >(
                b"void _cairo_xcb_connection_render_composite(cairo_xcb_connection_t *, uint8_t, xcb_render_picture_t, xcb_render_picture_t, xcb_render_picture_t, int16_t, int16_t, int16_t, int16_t, int16_t, int16_t, uint16_t, uint16_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_composite(
        (*connection).xcb_connection,
        op,
        src,
        mask,
        dst,
        src_x,
        src_y,
        mask_x,
        mask_y,
        dst_x,
        dst_y,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_trapezoids(
    mut connection: *mut cairo_xcb_connection_t,
    mut op: uint8_t,
    mut src: xcb_render_picture_t,
    mut dst: xcb_render_picture_t,
    mut mask_format: xcb_render_pictformat_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut traps_len: uint32_t,
    mut traps: *mut xcb_render_trapezoid_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 203],
                &[libc::c_char; 203],
            >(
                b"void _cairo_xcb_connection_render_trapezoids(cairo_xcb_connection_t *, uint8_t, xcb_render_picture_t, xcb_render_picture_t, xcb_render_pictformat_t, int16_t, int16_t, uint32_t, xcb_render_trapezoid_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_trapezoids(
        (*connection).xcb_connection,
        op,
        src,
        dst,
        mask_format,
        src_x,
        src_y,
        traps_len,
        traps,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_create_glyph_set(
    mut connection: *mut cairo_xcb_connection_t,
    mut id: xcb_render_glyphset_t,
    mut format: xcb_render_pictformat_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"void _cairo_xcb_connection_render_create_glyph_set(cairo_xcb_connection_t *, xcb_render_glyphset_t, xcb_render_pictformat_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_create_glyph_set((*connection).xcb_connection, id, format);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_free_glyph_set(
    mut connection: *mut cairo_xcb_connection_t,
    mut glyphset: xcb_render_glyphset_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void _cairo_xcb_connection_render_free_glyph_set(cairo_xcb_connection_t *, xcb_render_glyphset_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_free_glyph_set((*connection).xcb_connection, glyphset);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_add_glyphs(
    mut connection: *mut cairo_xcb_connection_t,
    mut glyphset: xcb_render_glyphset_t,
    mut num_glyphs: uint32_t,
    mut glyphs_id: *mut uint32_t,
    mut glyphs: *mut xcb_render_glyphinfo_t,
    mut data_len: uint32_t,
    mut data: *mut uint8_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            146 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 163],
                &[libc::c_char; 163],
            >(
                b"void _cairo_xcb_connection_render_add_glyphs(cairo_xcb_connection_t *, xcb_render_glyphset_t, uint32_t, uint32_t *, xcb_render_glyphinfo_t *, uint32_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_add_glyphs(
        (*connection).xcb_connection,
        glyphset,
        num_glyphs,
        glyphs_id,
        glyphs,
        data_len,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_free_glyphs(
    mut connection: *mut cairo_xcb_connection_t,
    mut glyphset: xcb_render_glyphset_t,
    mut num_glyphs: uint32_t,
    mut glyphs: *mut xcb_render_glyph_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            157 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"void _cairo_xcb_connection_render_free_glyphs(cairo_xcb_connection_t *, xcb_render_glyphset_t, uint32_t, xcb_render_glyph_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_free_glyphs((*connection).xcb_connection, glyphset, num_glyphs, glyphs);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_composite_glyphs_8(
    mut connection: *mut cairo_xcb_connection_t,
    mut op: uint8_t,
    mut src: xcb_render_picture_t,
    mut dst: xcb_render_picture_t,
    mut mask_format: xcb_render_pictformat_t,
    mut glyphset: xcb_render_glyphset_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut glyphcmds_len: uint32_t,
    mut glyphcmds: *mut uint8_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 219],
                &[libc::c_char; 219],
            >(
                b"void _cairo_xcb_connection_render_composite_glyphs_8(cairo_xcb_connection_t *, uint8_t, xcb_render_picture_t, xcb_render_picture_t, xcb_render_pictformat_t, xcb_render_glyphset_t, int16_t, int16_t, uint32_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_composite_glyphs_8(
        (*connection).xcb_connection,
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds_len,
        glyphcmds,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_composite_glyphs_16(
    mut connection: *mut cairo_xcb_connection_t,
    mut op: uint8_t,
    mut src: xcb_render_picture_t,
    mut dst: xcb_render_picture_t,
    mut mask_format: xcb_render_pictformat_t,
    mut glyphset: xcb_render_glyphset_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut glyphcmds_len: uint32_t,
    mut glyphcmds: *mut uint8_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            190 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 220],
                &[libc::c_char; 220],
            >(
                b"void _cairo_xcb_connection_render_composite_glyphs_16(cairo_xcb_connection_t *, uint8_t, xcb_render_picture_t, xcb_render_picture_t, xcb_render_pictformat_t, xcb_render_glyphset_t, int16_t, int16_t, uint32_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_composite_glyphs_16(
        (*connection).xcb_connection,
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds_len,
        glyphcmds,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_composite_glyphs_32(
    mut connection: *mut cairo_xcb_connection_t,
    mut op: uint8_t,
    mut src: xcb_render_picture_t,
    mut dst: xcb_render_picture_t,
    mut mask_format: xcb_render_pictformat_t,
    mut glyphset: xcb_render_glyphset_t,
    mut src_x: int16_t,
    mut src_y: int16_t,
    mut glyphcmds_len: uint32_t,
    mut glyphcmds: *mut uint8_t,
) {
    if (*connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_RENDER\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 220],
                &[libc::c_char; 220],
            >(
                b"void _cairo_xcb_connection_render_composite_glyphs_32(cairo_xcb_connection_t *, uint8_t, xcb_render_picture_t, xcb_render_picture_t, xcb_render_pictformat_t, xcb_render_glyphset_t, int16_t, int16_t, uint32_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_composite_glyphs_32(
        (*connection).xcb_connection,
        op,
        src,
        dst,
        mask_format,
        glyphset,
        src_x,
        src_y,
        glyphcmds_len,
        glyphcmds,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_fill_rectangles(
    mut connection: *mut cairo_xcb_connection_t,
    mut op: uint8_t,
    mut dst: xcb_render_picture_t,
    mut color: xcb_render_color_t,
    mut num_rects: uint32_t,
    mut rects: *mut xcb_rectangle_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 156],
                &[libc::c_char; 156],
            >(
                b"void _cairo_xcb_connection_render_fill_rectangles(cairo_xcb_connection_t *, uint8_t, xcb_render_picture_t, xcb_render_color_t, uint32_t, xcb_rectangle_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_fill_rectangles(
        (*connection).xcb_connection,
        op,
        dst,
        color,
        num_rects,
        rects,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_set_picture_transform(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut transform: *mut xcb_render_transform_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 130],
                &[libc::c_char; 130],
            >(
                b"void _cairo_xcb_connection_render_set_picture_transform(cairo_xcb_connection_t *, xcb_render_picture_t, xcb_render_transform_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_set_picture_transform((*connection).xcb_connection, picture, *transform);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_set_picture_filter(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut filter_len: uint16_t,
    mut filter: *mut libc::c_char,
) {
    if (*connection).flags & CAIRO_XCB_RENDER_HAS_FILTERS as libc::c_int as libc::c_uint
        != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_FILTERS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"void _cairo_xcb_connection_render_set_picture_filter(cairo_xcb_connection_t *, xcb_render_picture_t, uint16_t, char *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_set_picture_filter(
        (*connection).xcb_connection,
        picture,
        filter_len,
        filter,
        0 as libc::c_int as uint32_t,
        0 as *const xcb_render_fixed_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_create_solid_fill(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut color: xcb_render_color_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_GRADIENTS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            250 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 120],
                &[libc::c_char; 120],
            >(
                b"void _cairo_xcb_connection_render_create_solid_fill(cairo_xcb_connection_t *, xcb_render_picture_t, xcb_render_color_t)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_create_solid_fill((*connection).xcb_connection, picture, color);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_create_linear_gradient(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut p1: xcb_render_pointfix_t,
    mut p2: xcb_render_pointfix_t,
    mut num_stops: uint32_t,
    mut stops: *mut xcb_render_fixed_t,
    mut colors: *mut xcb_render_color_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_GRADIENTS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 205],
                &[libc::c_char; 205],
            >(
                b"void _cairo_xcb_connection_render_create_linear_gradient(cairo_xcb_connection_t *, xcb_render_picture_t, xcb_render_pointfix_t, xcb_render_pointfix_t, uint32_t, xcb_render_fixed_t *, xcb_render_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_create_linear_gradient(
        (*connection).xcb_connection,
        picture,
        p1,
        p2,
        num_stops,
        stops,
        colors,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_create_radial_gradient(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut inner: xcb_render_pointfix_t,
    mut outer: xcb_render_pointfix_t,
    mut inner_radius: xcb_render_fixed_t,
    mut outer_radius: xcb_render_fixed_t,
    mut num_stops: uint32_t,
    mut stops: *mut xcb_render_fixed_t,
    mut colors: *mut xcb_render_color_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_GRADIENTS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            279 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 245],
                &[libc::c_char; 245],
            >(
                b"void _cairo_xcb_connection_render_create_radial_gradient(cairo_xcb_connection_t *, xcb_render_picture_t, xcb_render_pointfix_t, xcb_render_pointfix_t, xcb_render_fixed_t, xcb_render_fixed_t, uint32_t, xcb_render_fixed_t *, xcb_render_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_create_radial_gradient(
        (*connection).xcb_connection,
        picture,
        inner,
        outer,
        inner_radius,
        outer_radius,
        num_stops,
        stops,
        colors,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_render_create_conical_gradient(
    mut connection: *mut cairo_xcb_connection_t,
    mut picture: xcb_render_picture_t,
    mut center: xcb_render_pointfix_t,
    mut angle: xcb_render_fixed_t,
    mut num_stops: uint32_t,
    mut stops: *mut xcb_render_fixed_t,
    mut colors: *mut xcb_render_color_t,
) {
    if (*connection).flags
        & CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint != 0
    {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_RENDER_HAS_GRADIENTS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-connection-render.c\0" as *const u8
                as *const libc::c_char,
            294 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 203],
                &[libc::c_char; 203],
            >(
                b"void _cairo_xcb_connection_render_create_conical_gradient(cairo_xcb_connection_t *, xcb_render_picture_t, xcb_render_pointfix_t, xcb_render_fixed_t, uint32_t, xcb_render_fixed_t *, xcb_render_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    xcb_render_create_conical_gradient(
        (*connection).xcb_connection,
        picture,
        center,
        angle,
        num_stops,
        stops,
        colors,
    );
}
