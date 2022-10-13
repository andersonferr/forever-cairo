use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    pub type _cairo_xcb_shm_mem_pool;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _pixman_format_from_masks(
        masks: *mut cairo_format_masks_t,
        format_ret: *mut pixman_format_code_t,
    ) -> cairo_bool_t;
    static mut _cairo_xcb_connections_mutex: cairo_mutex_t;
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    fn xcb_screen_allowed_depths_iterator(
        R: *const xcb_screen_t,
    ) -> xcb_depth_iterator_t;
    fn xcb_screen_next(i: *mut xcb_screen_iterator_t);
    fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
    fn xcb_create_pixmap_checked(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_pixmap(
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> libc::c_int;
    fn xcb_get_maximum_request_length(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t);
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
    fn xcb_prefetch_extension_data(c: *mut xcb_connection_t, ext: *mut xcb_extension_t);
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    static mut xcb_render_id: xcb_extension_t;
    fn _cairo_xcb_font_close(font: *mut cairo_xcb_font_t);
    fn _cairo_xcb_screen_finish(screen: *mut cairo_xcb_screen_t);
    fn _cairo_xcb_connection_shm_mem_pools_flush(
        connection: *mut cairo_xcb_connection_t,
    );
    fn _cairo_xcb_connection_shm_mem_pools_fini(connection: *mut cairo_xcb_connection_t);
    fn _cairo_freepool_fini(freepool: *mut cairo_freepool_t);
    fn _cairo_freepool_init(freepool: *mut cairo_freepool_t, nodesize: libc::c_uint);
    fn _cairo_device_init(
        device: *mut cairo_device_t,
        backend: *const cairo_device_backend_t,
    );
    fn _cairo_device_set_error(
        device: *mut cairo_device_t,
        error: cairo_status_t,
    ) -> cairo_status_t;
    fn xcb_render_query_pict_formats_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_pict_formats_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_pict_formats_reply_t;
    fn xcb_render_query_pict_formats_subpixels(
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> *mut uint32_t;
    fn xcb_render_query_pict_formats_screens_iterator(
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_render_pictscreen_iterator_t;
    fn xcb_render_query_pict_formats_formats_iterator(
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_render_pictforminfo_iterator_t;
    fn xcb_render_query_pict_formats(
        c: *mut xcb_connection_t,
    ) -> xcb_render_query_pict_formats_cookie_t;
    fn xcb_render_query_version_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_version_reply_t;
    fn xcb_render_query_version(
        c: *mut xcb_connection_t,
        client_major_version: uint32_t,
        client_minor_version: uint32_t,
    ) -> xcb_render_query_version_cookie_t;
    fn xcb_render_pictscreen_next(i: *mut xcb_render_pictscreen_iterator_t);
    fn xcb_render_pictscreen_depths_iterator(
        R: *const xcb_render_pictscreen_t,
    ) -> xcb_render_pictdepth_iterator_t;
    fn xcb_render_pictdepth_next(i: *mut xcb_render_pictdepth_iterator_t);
    fn xcb_render_pictdepth_visuals_iterator(
        R: *const xcb_render_pictdepth_t,
    ) -> xcb_render_pictvisual_iterator_t;
    fn xcb_render_pictvisual_next(i: *mut xcb_render_pictvisual_iterator_t);
    fn xcb_render_pictforminfo_next(i: *mut xcb_render_pictforminfo_iterator_t);
    static mut xcb_big_requests_id: xcb_extension_t;
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
    fn shmdt(__shmaddr: *const libc::c_void) -> libc::c_int;
    fn shmctl(
        __shmid: libc::c_int,
        __cmd: libc::c_int,
        __buf: *mut shmid_ds,
    ) -> libc::c_int;
    fn shmat(
        __shmid: libc::c_int,
        __shmaddr: *const libc::c_void,
        __shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    static mut xcb_shm_id: xcb_extension_t;
    fn xcb_shm_query_version(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t;
    fn xcb_shm_query_version_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_shm_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_query_version_reply_t;
    fn xcb_shm_attach_checked(
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shmid: uint32_t,
        read_only: uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_shm_detach_checked(
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
    ) -> xcb_void_cookie_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_ulong_t = libc::c_ulong;
pub type key_t = __key_t;
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
pub type cairo_t = _cairo;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_surface {
    pub backend: *const cairo_surface_backend_t,
    pub device: *mut cairo_device_t,
    pub type_0: cairo_surface_type_t,
    pub content: cairo_content_t,
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub unique_id: libc::c_uint,
    pub serial: libc::c_uint,
    pub damage: *mut cairo_damage_t,
    #[bitfield(name = "_finishing", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "finished", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_clear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "has_font_options", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "owns_device", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "is_vector", ty = "libc::c_uint", bits = "5..=5")]
    pub _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub user_data: cairo_user_data_array_t,
    pub mime_data: cairo_user_data_array_t,
    pub device_transform: cairo_matrix_t,
    pub device_transform_inverse: cairo_matrix_t,
    pub device_transform_observers: cairo_list_t,
    pub x_resolution: libc::c_double,
    pub y_resolution: libc::c_double,
    pub x_fallback_resolution: libc::c_double,
    pub y_fallback_resolution: libc::c_double,
    pub snapshot_of: *mut cairo_surface_t,
    pub snapshot_detach: cairo_surface_func_t,
    pub snapshots: cairo_list_t,
    pub snapshot: cairo_list_t,
    pub font_options: cairo_font_options_t,
}
pub type cairo_font_options_t = _cairo_font_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_font_options {
    pub antialias: cairo_antialias_t,
    pub subpixel_order: cairo_subpixel_order_t,
    pub lcd_filter: cairo_lcd_filter_t,
    pub hint_style: cairo_hint_style_t,
    pub hint_metrics: cairo_hint_metrics_t,
    pub round_glyph_positions: cairo_round_glyph_positions_t,
    pub variations: *mut libc::c_char,
    pub color_mode: cairo_color_mode_t,
    pub palette_index: libc::c_uint,
}
pub type cairo_color_mode_t = _cairo_color_mode;
pub type _cairo_color_mode = libc::c_uint;
pub const CAIRO_COLOR_MODE_COLOR: _cairo_color_mode = 2;
pub const CAIRO_COLOR_MODE_NO_COLOR: _cairo_color_mode = 1;
pub const CAIRO_COLOR_MODE_DEFAULT: _cairo_color_mode = 0;
pub type cairo_round_glyph_positions_t = _cairo_round_glyph_positions;
pub type _cairo_round_glyph_positions = libc::c_uint;
pub const CAIRO_ROUND_GLYPH_POS_OFF: _cairo_round_glyph_positions = 2;
pub const CAIRO_ROUND_GLYPH_POS_ON: _cairo_round_glyph_positions = 1;
pub const CAIRO_ROUND_GLYPH_POS_DEFAULT: _cairo_round_glyph_positions = 0;
pub type cairo_hint_metrics_t = _cairo_hint_metrics;
pub type _cairo_hint_metrics = libc::c_uint;
pub const CAIRO_HINT_METRICS_ON: _cairo_hint_metrics = 2;
pub const CAIRO_HINT_METRICS_OFF: _cairo_hint_metrics = 1;
pub const CAIRO_HINT_METRICS_DEFAULT: _cairo_hint_metrics = 0;
pub type cairo_hint_style_t = _cairo_hint_style;
pub type _cairo_hint_style = libc::c_uint;
pub const CAIRO_HINT_STYLE_FULL: _cairo_hint_style = 4;
pub const CAIRO_HINT_STYLE_MEDIUM: _cairo_hint_style = 3;
pub const CAIRO_HINT_STYLE_SLIGHT: _cairo_hint_style = 2;
pub const CAIRO_HINT_STYLE_NONE: _cairo_hint_style = 1;
pub const CAIRO_HINT_STYLE_DEFAULT: _cairo_hint_style = 0;
pub type cairo_lcd_filter_t = _cairo_lcd_filter;
pub type _cairo_lcd_filter = libc::c_uint;
pub const CAIRO_LCD_FILTER_FIR5: _cairo_lcd_filter = 4;
pub const CAIRO_LCD_FILTER_FIR3: _cairo_lcd_filter = 3;
pub const CAIRO_LCD_FILTER_INTRA_PIXEL: _cairo_lcd_filter = 2;
pub const CAIRO_LCD_FILTER_NONE: _cairo_lcd_filter = 1;
pub const CAIRO_LCD_FILTER_DEFAULT: _cairo_lcd_filter = 0;
pub type cairo_subpixel_order_t = _cairo_subpixel_order;
pub type _cairo_subpixel_order = libc::c_uint;
pub const CAIRO_SUBPIXEL_ORDER_VBGR: _cairo_subpixel_order = 4;
pub const CAIRO_SUBPIXEL_ORDER_VRGB: _cairo_subpixel_order = 3;
pub const CAIRO_SUBPIXEL_ORDER_BGR: _cairo_subpixel_order = 2;
pub const CAIRO_SUBPIXEL_ORDER_RGB: _cairo_subpixel_order = 1;
pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: _cairo_subpixel_order = 0;
pub type cairo_antialias_t = _cairo_antialias;
pub type _cairo_antialias = libc::c_uint;
pub const CAIRO_ANTIALIAS_BEST: _cairo_antialias = 6;
pub const CAIRO_ANTIALIAS_GOOD: _cairo_antialias = 5;
pub const CAIRO_ANTIALIAS_FAST: _cairo_antialias = 4;
pub const CAIRO_ANTIALIAS_SUBPIXEL: _cairo_antialias = 3;
pub const CAIRO_ANTIALIAS_GRAY: _cairo_antialias = 2;
pub const CAIRO_ANTIALIAS_NONE: _cairo_antialias = 1;
pub const CAIRO_ANTIALIAS_DEFAULT: _cairo_antialias = 0;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
pub type cairo_surface_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_surface_t) -> (),
>;
pub type cairo_surface_t = _cairo_surface;
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
pub type cairo_damage_t = _cairo_damage;
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
pub type cairo_content_t = _cairo_content;
pub type _cairo_content = libc::c_uint;
pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;
pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
pub type cairo_surface_type_t = _cairo_surface_type;
pub type _cairo_surface_type = libc::c_uint;
pub const CAIRO_SURFACE_TYPE_COGL: _cairo_surface_type = 24;
pub const CAIRO_SURFACE_TYPE_SUBSURFACE: _cairo_surface_type = 23;
pub const CAIRO_SURFACE_TYPE_SKIA: _cairo_surface_type = 22;
pub const CAIRO_SURFACE_TYPE_XML: _cairo_surface_type = 21;
pub const CAIRO_SURFACE_TYPE_TEE: _cairo_surface_type = 20;
pub const CAIRO_SURFACE_TYPE_DRM: _cairo_surface_type = 19;
pub const CAIRO_SURFACE_TYPE_GL: _cairo_surface_type = 18;
pub const CAIRO_SURFACE_TYPE_VG: _cairo_surface_type = 17;
pub const CAIRO_SURFACE_TYPE_RECORDING: _cairo_surface_type = 16;
pub const CAIRO_SURFACE_TYPE_QT: _cairo_surface_type = 15;
pub const CAIRO_SURFACE_TYPE_SCRIPT: _cairo_surface_type = 14;
pub const CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: _cairo_surface_type = 13;
pub const CAIRO_SURFACE_TYPE_WIN32_PRINTING: _cairo_surface_type = 12;
pub const CAIRO_SURFACE_TYPE_OS2: _cairo_surface_type = 11;
pub const CAIRO_SURFACE_TYPE_SVG: _cairo_surface_type = 10;
pub const CAIRO_SURFACE_TYPE_DIRECTFB: _cairo_surface_type = 9;
pub const CAIRO_SURFACE_TYPE_BEOS: _cairo_surface_type = 8;
pub const CAIRO_SURFACE_TYPE_WIN32: _cairo_surface_type = 7;
pub const CAIRO_SURFACE_TYPE_QUARTZ: _cairo_surface_type = 6;
pub const CAIRO_SURFACE_TYPE_GLITZ: _cairo_surface_type = 5;
pub const CAIRO_SURFACE_TYPE_XCB: _cairo_surface_type = 4;
pub const CAIRO_SURFACE_TYPE_XLIB: _cairo_surface_type = 3;
pub const CAIRO_SURFACE_TYPE_PS: _cairo_surface_type = 2;
pub const CAIRO_SURFACE_TYPE_PDF: _cairo_surface_type = 1;
pub const CAIRO_SURFACE_TYPE_IMAGE: _cairo_surface_type = 0;
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
pub type cairo_surface_backend_t = _cairo_surface_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_backend {
    pub type_0: cairo_surface_type_t,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub create_context: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
    >,
    pub create_similar: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_content_t,
            libc::c_int,
            libc::c_int,
        ) -> *mut cairo_surface_t,
    >,
    pub create_similar_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_format_t,
            libc::c_int,
            libc::c_int,
        ) -> *mut cairo_surface_t,
    >,
    pub map_to_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_rectangle_int_t,
        ) -> *mut cairo_image_surface_t,
    >,
    pub unmap_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
        ) -> cairo_int_status_t,
    >,
    pub source: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_rectangle_int_t,
        ) -> *mut cairo_surface_t,
    >,
    pub acquire_source_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *mut cairo_image_surface_t,
            *mut *mut libc::c_void,
        ) -> cairo_status_t,
    >,
    pub release_source_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
            *mut libc::c_void,
        ) -> (),
    >,
    pub snapshot: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
    >,
    pub copy_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub show_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub get_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_rectangle_int_t,
        ) -> cairo_bool_t,
    >,
    pub get_font_options: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_font_options_t) -> (),
    >,
    pub flush: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> cairo_status_t,
    >,
    pub mark_dirty_rectangle: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> cairo_status_t,
    >,
    pub paint: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub mask: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_pattern_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub stroke: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_path_fixed_t,
            *const cairo_stroke_style_t,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub fill: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_path_fixed_t,
            cairo_fill_rule_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub fill_stroke: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            cairo_fill_rule_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_path_fixed_t,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_stroke_style_t,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub show_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *mut cairo_glyph_t,
            libc::c_int,
            *mut cairo_scaled_font_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub has_show_text_glyphs: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
    pub show_text_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const libc::c_char,
            libc::c_int,
            *mut cairo_glyph_t,
            libc::c_int,
            *const cairo_text_cluster_t,
            libc::c_int,
            cairo_text_cluster_flags_t,
            *mut cairo_scaled_font_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub get_supported_mime_types: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut *const libc::c_char,
    >,
    pub tag: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_bool_t,
            *const libc::c_char,
            *const libc::c_char,
        ) -> cairo_int_status_t,
    >,
}
pub type cairo_int_status_t = _cairo_int_status;
pub type _cairo_int_status = libc::c_uint;
pub const CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN: _cairo_int_status = 105;
pub const CAIRO_INT_STATUS_IMAGE_FALLBACK: _cairo_int_status = 104;
pub const CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY: _cairo_int_status = 103;
pub const CAIRO_INT_STATUS_NOTHING_TO_DO: _cairo_int_status = 102;
pub const CAIRO_INT_STATUS_DEGENERATE: _cairo_int_status = 101;
pub const CAIRO_INT_STATUS_UNSUPPORTED: _cairo_int_status = 100;
pub const CAIRO_INT_STATUS_LAST_STATUS: _cairo_int_status = 44;
pub const CAIRO_INT_STATUS_DWRITE_ERROR: _cairo_int_status = 43;
pub const CAIRO_INT_STATUS_TAG_ERROR: _cairo_int_status = 42;
pub const CAIRO_INT_STATUS_WIN32_GDI_ERROR: _cairo_int_status = 41;
pub const CAIRO_INT_STATUS_FREETYPE_ERROR: _cairo_int_status = 40;
pub const CAIRO_INT_STATUS_PNG_ERROR: _cairo_int_status = 39;
pub const CAIRO_INT_STATUS_JBIG2_GLOBAL_MISSING: _cairo_int_status = 38;
pub const CAIRO_INT_STATUS_DEVICE_FINISHED: _cairo_int_status = 37;
pub const CAIRO_INT_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_int_status = 36;
pub const CAIRO_INT_STATUS_DEVICE_ERROR: _cairo_int_status = 35;
pub const CAIRO_INT_STATUS_DEVICE_TYPE_MISMATCH: _cairo_int_status = 34;
pub const CAIRO_INT_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_int_status = 33;
pub const CAIRO_INT_STATUS_INVALID_SIZE: _cairo_int_status = 32;
pub const CAIRO_INT_STATUS_INVALID_WEIGHT: _cairo_int_status = 31;
pub const CAIRO_INT_STATUS_INVALID_SLANT: _cairo_int_status = 30;
pub const CAIRO_INT_STATUS_INVALID_CLUSTERS: _cairo_int_status = 29;
pub const CAIRO_INT_STATUS_NEGATIVE_COUNT: _cairo_int_status = 28;
pub const CAIRO_INT_STATUS_USER_FONT_ERROR: _cairo_int_status = 27;
pub const CAIRO_INT_STATUS_USER_FONT_IMMUTABLE: _cairo_int_status = 26;
pub const CAIRO_INT_STATUS_FONT_TYPE_MISMATCH: _cairo_int_status = 25;
pub const CAIRO_INT_STATUS_INVALID_STRIDE: _cairo_int_status = 24;
pub const CAIRO_INT_STATUS_TEMP_FILE_ERROR: _cairo_int_status = 23;
pub const CAIRO_INT_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_int_status = 22;
pub const CAIRO_INT_STATUS_INVALID_INDEX: _cairo_int_status = 21;
pub const CAIRO_INT_STATUS_INVALID_DSC_COMMENT: _cairo_int_status = 20;
pub const CAIRO_INT_STATUS_INVALID_DASH: _cairo_int_status = 19;
pub const CAIRO_INT_STATUS_FILE_NOT_FOUND: _cairo_int_status = 18;
pub const CAIRO_INT_STATUS_INVALID_VISUAL: _cairo_int_status = 17;
pub const CAIRO_INT_STATUS_INVALID_FORMAT: _cairo_int_status = 16;
pub const CAIRO_INT_STATUS_INVALID_CONTENT: _cairo_int_status = 15;
pub const CAIRO_INT_STATUS_PATTERN_TYPE_MISMATCH: _cairo_int_status = 14;
pub const CAIRO_INT_STATUS_SURFACE_TYPE_MISMATCH: _cairo_int_status = 13;
pub const CAIRO_INT_STATUS_SURFACE_FINISHED: _cairo_int_status = 12;
pub const CAIRO_INT_STATUS_WRITE_ERROR: _cairo_int_status = 11;
pub const CAIRO_INT_STATUS_READ_ERROR: _cairo_int_status = 10;
pub const CAIRO_INT_STATUS_INVALID_PATH_DATA: _cairo_int_status = 9;
pub const CAIRO_INT_STATUS_INVALID_STRING: _cairo_int_status = 8;
pub const CAIRO_INT_STATUS_NULL_POINTER: _cairo_int_status = 7;
pub const CAIRO_INT_STATUS_INVALID_STATUS: _cairo_int_status = 6;
pub const CAIRO_INT_STATUS_INVALID_MATRIX: _cairo_int_status = 5;
pub const CAIRO_INT_STATUS_NO_CURRENT_POINT: _cairo_int_status = 4;
pub const CAIRO_INT_STATUS_INVALID_POP_GROUP: _cairo_int_status = 3;
pub const CAIRO_INT_STATUS_INVALID_RESTORE: _cairo_int_status = 2;
pub const CAIRO_INT_STATUS_NO_MEMORY: _cairo_int_status = 1;
pub const CAIRO_INT_STATUS_SUCCESS: _cairo_int_status = 0;
pub type cairo_clip_t = _cairo_clip;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip {
    pub extents: cairo_rectangle_int_t,
    pub path: *mut cairo_clip_path_t,
    pub boxes: *mut cairo_box_t,
    pub num_boxes: libc::c_int,
    pub region: *mut cairo_region_t,
    pub is_region: cairo_bool_t,
    pub embedded_box: cairo_box_t,
}
pub type cairo_box_t = _cairo_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_line {
    pub p1: cairo_point_t,
    pub p2: cairo_point_t,
}
pub type cairo_point_t = _cairo_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point {
    pub x: cairo_fixed_t,
    pub y: cairo_fixed_t,
}
pub type cairo_fixed_t = int32_t;
pub type cairo_region_t = _cairo_region;
pub type cairo_clip_path_t = _cairo_clip_path;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip_path {
    pub ref_count: cairo_reference_count_t,
    pub path: cairo_path_fixed_t,
    pub fill_rule: cairo_fill_rule_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
    pub prev: *mut cairo_clip_path_t,
}
pub type cairo_fill_rule_t = _cairo_fill_rule;
pub type _cairo_fill_rule = libc::c_uint;
pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;
pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
pub type cairo_path_fixed_t = _cairo_path_fixed;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_path_fixed {
    pub last_move_point: cairo_point_t,
    pub current_point: cairo_point_t,
    #[bitfield(name = "has_current_point", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "needs_move_to", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "has_extents", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "has_curve_to", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "stroke_is_rectilinear", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "fill_is_rectilinear", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "fill_maybe_region", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fill_is_empty", ty = "libc::c_uint", bits = "7..=7")]
    pub has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub extents: cairo_box_t,
    pub buf: cairo_path_buf_fixed_t,
}
pub type cairo_path_buf_fixed_t = _cairo_path_buf_fixed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_buf_fixed {
    pub base: cairo_path_buf_t,
    pub op: [cairo_path_op_t; 27],
    pub points: [cairo_point_t; 54],
}
pub type cairo_path_op_t = libc::c_char;
pub type cairo_path_buf_t = _cairo_path_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_buf {
    pub link: cairo_list_t,
    pub num_ops: libc::c_uint,
    pub size_ops: libc::c_uint,
    pub num_points: libc::c_uint,
    pub size_points: libc::c_uint,
    pub op: *mut cairo_path_op_t,
    pub points: *mut cairo_point_t,
}
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_int {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type cairo_scaled_font_t = _cairo_scaled_font;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_scaled_font {
    pub hash_entry: cairo_hash_entry_t,
    pub status: cairo_status_t,
    pub ref_count: cairo_reference_count_t,
    pub user_data: cairo_user_data_array_t,
    pub original_font_face: *mut cairo_font_face_t,
    pub font_face: *mut cairo_font_face_t,
    pub font_matrix: cairo_matrix_t,
    pub ctm: cairo_matrix_t,
    pub options: cairo_font_options_t,
    #[bitfield(name = "placeholder", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "holdover", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "finished", ty = "libc::c_uint", bits = "2..=2")]
    pub placeholder_holdover_finished: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub scale: cairo_matrix_t,
    pub scale_inverse: cairo_matrix_t,
    pub max_scale: libc::c_double,
    pub extents: cairo_font_extents_t,
    pub fs_extents: cairo_font_extents_t,
    pub mutex: cairo_recursive_mutex_t,
    pub glyphs: *mut cairo_hash_table_t,
    pub glyph_pages: cairo_list_t,
    pub cache_frozen: cairo_bool_t,
    pub global_cache_frozen: cairo_bool_t,
    pub recording_surfaces_to_free: cairo_array_t,
    pub dev_privates: cairo_list_t,
    pub backend: *const cairo_scaled_font_backend_t,
    pub link: cairo_list_t,
}
pub type cairo_scaled_font_backend_t = _cairo_scaled_font_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_backend {
    pub type_0: cairo_font_type_t,
    pub fini: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scaled_glyph_init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_scaled_glyph_t,
            cairo_scaled_glyph_info_t,
            *const cairo_color_t,
        ) -> cairo_int_status_t,
    >,
    pub text_to_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            *const libc::c_char,
            libc::c_int,
            *mut *mut cairo_glyph_t,
            *mut libc::c_int,
            *mut *mut cairo_text_cluster_t,
            *mut libc::c_int,
            *mut cairo_text_cluster_flags_t,
        ) -> cairo_int_status_t,
    >,
    pub ucs4_to_index: Option::<
        unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_ulong,
    >,
    pub load_truetype_table: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_ulong,
            libc::c_long,
            *mut libc::c_uchar,
            *mut libc::c_ulong,
        ) -> cairo_int_status_t,
    >,
    pub index_to_ucs4: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_ulong,
            *mut uint32_t,
        ) -> cairo_int_status_t,
    >,
    pub is_synthetic: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_bool_t) -> cairo_int_status_t,
    >,
    pub index_to_glyph_name: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *mut libc::c_char,
            libc::c_int,
            libc::c_ulong,
            *mut libc::c_ulong,
        ) -> cairo_int_status_t,
    >,
    pub load_type1_data: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_long,
            *mut libc::c_uchar,
            *mut libc::c_ulong,
        ) -> cairo_int_status_t,
    >,
    pub has_color_glyphs: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
}
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
pub type cairo_color_t = _cairo_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_color {
    pub red: libc::c_double,
    pub green: libc::c_double,
    pub blue: libc::c_double,
    pub alpha: libc::c_double,
    pub red_short: libc::c_ushort,
    pub green_short: libc::c_ushort,
    pub blue_short: libc::c_ushort,
    pub alpha_short: libc::c_ushort,
}
pub type cairo_scaled_glyph_info_t = _cairo_scaled_glyph_info;
pub type _cairo_scaled_glyph_info = libc::c_uint;
pub const CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE: _cairo_scaled_glyph_info = 16;
pub const CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE: _cairo_scaled_glyph_info = 8;
pub const CAIRO_SCALED_GLYPH_INFO_PATH: _cairo_scaled_glyph_info = 4;
pub const CAIRO_SCALED_GLYPH_INFO_SURFACE: _cairo_scaled_glyph_info = 2;
pub const CAIRO_SCALED_GLYPH_INFO_METRICS: _cairo_scaled_glyph_info = 1;
pub type cairo_scaled_glyph_t = _cairo_scaled_glyph;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_scaled_glyph {
    pub hash_entry: cairo_hash_entry_t,
    pub metrics: cairo_text_extents_t,
    pub fs_metrics: cairo_text_extents_t,
    pub bbox: cairo_box_t,
    pub x_advance: int16_t,
    pub y_advance: int16_t,
    pub has_info: libc::c_uint,
    pub surface: *mut cairo_image_surface_t,
    pub path: *mut cairo_path_fixed_t,
    pub recording_surface: *mut cairo_surface_t,
    pub color_surface: *mut cairo_image_surface_t,
    pub dev_private_key: *const libc::c_void,
    pub dev_private: *mut libc::c_void,
    pub dev_privates: cairo_list_t,
    pub foreground_color: cairo_color_t,
    #[bitfield(name = "uses_foreground_color", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "color_glyph_set", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "color_glyph", ty = "libc::c_uint", bits = "2..=2")]
    pub uses_foreground_color_color_glyph_set_color_glyph: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type cairo_image_surface_t = _cairo_image_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_text_extents_t {
    pub x_bearing: libc::c_double,
    pub y_bearing: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
}
pub type cairo_hash_entry_t = _cairo_hash_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_hash_entry {
    pub hash: uintptr_t,
}
pub type uintptr_t = libc::c_ulong;
pub type cairo_font_type_t = _cairo_font_type;
pub type _cairo_font_type = libc::c_uint;
pub const CAIRO_FONT_TYPE_DWRITE: _cairo_font_type = 5;
pub const CAIRO_FONT_TYPE_USER: _cairo_font_type = 4;
pub const CAIRO_FONT_TYPE_QUARTZ: _cairo_font_type = 3;
pub const CAIRO_FONT_TYPE_WIN32: _cairo_font_type = 2;
pub const CAIRO_FONT_TYPE_FT: _cairo_font_type = 1;
pub const CAIRO_FONT_TYPE_TOY: _cairo_font_type = 0;
pub type cairo_hash_table_t = _cairo_hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_font_extents_t {
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub height: libc::c_double,
    pub max_x_advance: libc::c_double,
    pub max_y_advance: libc::c_double,
}
pub type cairo_font_face_t = _cairo_font_face;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_font_face {
    pub hash_entry: cairo_hash_entry_t,
    pub status: cairo_status_t,
    pub ref_count: cairo_reference_count_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_font_face_backend_t,
}
pub type cairo_font_face_backend_t = _cairo_font_face_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_font_face_backend {
    pub type_0: cairo_font_type_t,
    pub create_for_toy: Option::<
        unsafe extern "C" fn(
            *mut cairo_toy_font_face_t,
            *mut *mut cairo_font_face_t,
        ) -> cairo_status_t,
    >,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t>,
    pub scaled_font_create: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            *const cairo_font_options_t,
            *mut *mut cairo_scaled_font_t,
        ) -> cairo_status_t,
    >,
    pub get_implementation: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            *const cairo_font_options_t,
        ) -> *mut cairo_font_face_t,
    >,
}
pub type cairo_toy_font_face_t = _cairo_toy_font_face;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_toy_font_face {
    pub base: cairo_font_face_t,
    pub family: *const libc::c_char,
    pub owns_family: cairo_bool_t,
    pub slant: cairo_font_slant_t,
    pub weight: cairo_font_weight_t,
    pub impl_face: *mut cairo_font_face_t,
}
pub type cairo_font_weight_t = _cairo_font_weight;
pub type _cairo_font_weight = libc::c_uint;
pub const CAIRO_FONT_WEIGHT_BOLD: _cairo_font_weight = 1;
pub const CAIRO_FONT_WEIGHT_NORMAL: _cairo_font_weight = 0;
pub type cairo_font_slant_t = _cairo_font_slant;
pub type _cairo_font_slant = libc::c_uint;
pub const CAIRO_FONT_SLANT_OBLIQUE: _cairo_font_slant = 2;
pub const CAIRO_FONT_SLANT_ITALIC: _cairo_font_slant = 1;
pub const CAIRO_FONT_SLANT_NORMAL: _cairo_font_slant = 0;
pub type cairo_pattern_t = _cairo_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pattern {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub observers: cairo_list_t,
    pub type_0: cairo_pattern_type_t,
    pub filter: cairo_filter_t,
    pub extend: cairo_extend_t,
    pub has_component_alpha: cairo_bool_t,
    pub is_userfont_foreground: cairo_bool_t,
    pub matrix: cairo_matrix_t,
    pub opacity: libc::c_double,
}
pub type cairo_extend_t = _cairo_extend;
pub type _cairo_extend = libc::c_uint;
pub const CAIRO_EXTEND_PAD: _cairo_extend = 3;
pub const CAIRO_EXTEND_REFLECT: _cairo_extend = 2;
pub const CAIRO_EXTEND_REPEAT: _cairo_extend = 1;
pub const CAIRO_EXTEND_NONE: _cairo_extend = 0;
pub type cairo_filter_t = _cairo_filter;
pub type _cairo_filter = libc::c_uint;
pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;
pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
pub type cairo_pattern_type_t = _cairo_pattern_type;
pub type _cairo_pattern_type = libc::c_uint;
pub const CAIRO_PATTERN_TYPE_RASTER_SOURCE: _cairo_pattern_type = 5;
pub const CAIRO_PATTERN_TYPE_MESH: _cairo_pattern_type = 4;
pub const CAIRO_PATTERN_TYPE_RADIAL: _cairo_pattern_type = 3;
pub const CAIRO_PATTERN_TYPE_LINEAR: _cairo_pattern_type = 2;
pub const CAIRO_PATTERN_TYPE_SURFACE: _cairo_pattern_type = 1;
pub const CAIRO_PATTERN_TYPE_SOLID: _cairo_pattern_type = 0;
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
pub type cairo_stroke_style_t = _cairo_stroke_style;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_stroke_style {
    pub line_width: libc::c_double,
    pub line_cap: cairo_line_cap_t,
    pub line_join: cairo_line_join_t,
    pub miter_limit: libc::c_double,
    pub dash: *mut libc::c_double,
    pub num_dashes: libc::c_uint,
    pub dash_offset: libc::c_double,
    pub is_hairline: cairo_bool_t,
    pub pre_hairline_line_width: libc::c_double,
}
pub type cairo_line_join_t = _cairo_line_join;
pub type _cairo_line_join = libc::c_uint;
pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;
pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
pub type cairo_line_cap_t = _cairo_line_cap;
pub type _cairo_line_cap = libc::c_uint;
pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;
pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
pub type cairo_format_t = _cairo_format;
pub type _cairo_format = libc::c_int;
pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type pixman_format_code_t = libc::c_uint;
pub const PIXMAN_yv12: pixman_format_code_t = 201785344;
pub const PIXMAN_yuy2: pixman_format_code_t = 268828672;
pub const PIXMAN_g1: pixman_format_code_t = 17104896;
pub const PIXMAN_a1: pixman_format_code_t = 16846848;
pub const PIXMAN_g4: pixman_format_code_t = 67436544;
pub const PIXMAN_c4: pixman_format_code_t = 67371008;
pub const PIXMAN_a1b1g1r1: pixman_format_code_t = 67309841;
pub const PIXMAN_a1r1g1b1: pixman_format_code_t = 67244305;
pub const PIXMAN_b1g2r1: pixman_format_code_t = 67305761;
pub const PIXMAN_r1g2b1: pixman_format_code_t = 67240225;
pub const PIXMAN_a4: pixman_format_code_t = 67190784;
pub const PIXMAN_x4g4: pixman_format_code_t = 134545408;
pub const PIXMAN_x4c4: pixman_format_code_t = 134479872;
pub const PIXMAN_x4a4: pixman_format_code_t = 134299648;
pub const PIXMAN_g8: pixman_format_code_t = 134545408;
pub const PIXMAN_c8: pixman_format_code_t = 134479872;
pub const PIXMAN_a2b2g2r2: pixman_format_code_t = 134423074;
pub const PIXMAN_a2r2g2b2: pixman_format_code_t = 134357538;
pub const PIXMAN_b2g3r3: pixman_format_code_t = 134415154;
pub const PIXMAN_r3g3b2: pixman_format_code_t = 134349618;
pub const PIXMAN_a8: pixman_format_code_t = 134316032;
pub const PIXMAN_x4b4g4r4: pixman_format_code_t = 268633156;
pub const PIXMAN_a4b4g4r4: pixman_format_code_t = 268649540;
pub const PIXMAN_x4r4g4b4: pixman_format_code_t = 268567620;
pub const PIXMAN_a4r4g4b4: pixman_format_code_t = 268584004;
pub const PIXMAN_x1b5g5r5: pixman_format_code_t = 268633429;
pub const PIXMAN_a1b5g5r5: pixman_format_code_t = 268637525;
pub const PIXMAN_x1r5g5b5: pixman_format_code_t = 268567893;
pub const PIXMAN_a1r5g5b5: pixman_format_code_t = 268571989;
pub const PIXMAN_b5g6r5: pixman_format_code_t = 268633445;
pub const PIXMAN_r5g6b5: pixman_format_code_t = 268567909;
pub const PIXMAN_b8g8r8: pixman_format_code_t = 402851976;
pub const PIXMAN_r8g8b8: pixman_format_code_t = 402786440;
pub const PIXMAN_a8r8g8b8_sRGB: pixman_format_code_t = 537561224;
pub const PIXMAN_a2b10g10r10: pixman_format_code_t = 537078442;
pub const PIXMAN_x2b10g10r10: pixman_format_code_t = 537070250;
pub const PIXMAN_a2r10g10b10: pixman_format_code_t = 537012906;
pub const PIXMAN_x2r10g10b10: pixman_format_code_t = 537004714;
pub const PIXMAN_x14r6g6b6: pixman_format_code_t = 537003622;
pub const PIXMAN_r8g8b8x8: pixman_format_code_t = 537462920;
pub const PIXMAN_r8g8b8a8: pixman_format_code_t = 537495688;
pub const PIXMAN_b8g8r8x8: pixman_format_code_t = 537397384;
pub const PIXMAN_b8g8r8a8: pixman_format_code_t = 537430152;
pub const PIXMAN_x8b8g8r8: pixman_format_code_t = 537069704;
pub const PIXMAN_a8b8g8r8: pixman_format_code_t = 537102472;
pub const PIXMAN_x8r8g8b8: pixman_format_code_t = 537004168;
pub const PIXMAN_a8r8g8b8: pixman_format_code_t = 537036936;
pub const PIXMAN_rgb_float: pixman_format_code_t = 214631492;
pub const PIXMAN_rgba_float: pixman_format_code_t = 281756740;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cache {
    pub hash_table: *mut cairo_hash_table_t,
    pub predicate: cairo_cache_predicate_func_t,
    pub entry_destroy: cairo_destroy_func_t,
    pub max_size: libc::c_ulong,
    pub size: libc::c_ulong,
    pub freeze_count: libc::c_int,
}
pub type cairo_cache_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_cache_t = _cairo_cache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_private {
    pub link: cairo_list_t,
    pub key: *const libc::c_void,
    pub destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_font_private_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
}
pub type cairo_scaled_font_private_t = _cairo_scaled_font_private;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_format_masks {
    pub bpp: libc::c_int,
    pub alpha_mask: libc::c_ulong,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
}
pub type cairo_format_masks_t = _cairo_format_masks;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
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
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub visuals_len: uint16_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
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
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: libc::c_uint,
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
pub type xcb_render_pict_type_t = libc::c_uint;
pub const XCB_RENDER_PICT_TYPE_DIRECT: xcb_render_pict_type_t = 1;
pub const XCB_RENDER_PICT_TYPE_INDEXED: xcb_render_pict_type_t = 0;
pub type xcb_render_sub_pixel_t = libc::c_uint;
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 5;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 4;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 3;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 2;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 1;
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0;
pub type xcb_render_glyph_t = uint32_t;
pub type xcb_render_glyphset_t = uint32_t;
pub type xcb_render_pictformat_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_directformat_t {
    pub red_shift: uint16_t,
    pub red_mask: uint16_t,
    pub green_shift: uint16_t,
    pub green_mask: uint16_t,
    pub blue_shift: uint16_t,
    pub blue_mask: uint16_t,
    pub alpha_shift: uint16_t,
    pub alpha_mask: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictforminfo_t {
    pub id: xcb_render_pictformat_t,
    pub type_0: uint8_t,
    pub depth: uint8_t,
    pub pad0: [uint8_t; 2],
    pub direct: xcb_render_directformat_t,
    pub colormap: xcb_colormap_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictforminfo_iterator_t {
    pub data: *mut xcb_render_pictforminfo_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictvisual_t {
    pub visual: xcb_visualid_t,
    pub format: xcb_render_pictformat_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictvisual_iterator_t {
    pub data: *mut xcb_render_pictvisual_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictdepth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub num_visuals: uint16_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictdepth_iterator_t {
    pub data: *mut xcb_render_pictdepth_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictscreen_t {
    pub num_depths: uint32_t,
    pub fallback: xcb_render_pictformat_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictscreen_iterator_t {
    pub data: *mut xcb_render_pictscreen_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_version_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_version_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint32_t,
    pub minor_version: uint32_t,
    pub pad1: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub num_formats: uint32_t,
    pub num_screens: uint32_t,
    pub num_depths: uint32_t,
    pub num_visuals: uint32_t,
    pub num_subpixel: uint32_t,
    pub pad1: [uint8_t; 4],
}
pub type cairo_xcb_connection_t = _cairo_xcb_connection;
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
pub type cairo_freepool_t = _cairo_freepool;
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
pub type cairo_freelist_pool_t = _cairo_freelist_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_pool {
    pub next: *mut cairo_freelist_pool_t,
    pub size: libc::c_uint,
    pub rem: libc::c_uint,
    pub data: *mut uint8_t,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
pub const CAIRO_XCB_HAS_SHM: C2RustUnnamed_1 = 2147483648;
pub const CAIRO_XCB_SHM_MASK: C2RustUnnamed_1 = 2147483648;
pub const CAIRO_XCB_RENDER_HAS_GRADIENTS: C2RustUnnamed_1 = 512;
pub const CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT: C2RustUnnamed_1 = 256;
pub const CAIRO_XCB_RENDER_HAS_PDF_OPERATORS: C2RustUnnamed_1 = 128;
pub const CAIRO_XCB_RENDER_HAS_FILTERS: C2RustUnnamed_1 = 64;
pub const CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM: C2RustUnnamed_1 = 32;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS: C2RustUnnamed_1 = 8;
pub const CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES: C2RustUnnamed_1 = 2;
pub const CAIRO_XCB_RENDER_HAS_FILTER_BEST: C2RustUnnamed_1 = 2048;
pub const CAIRO_XCB_RENDER_HAS_FILTER_GOOD: C2RustUnnamed_1 = 1024;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS: C2RustUnnamed_1 = 16;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE: C2RustUnnamed_1 = 4;
pub const CAIRO_XCB_HAS_RENDER: C2RustUnnamed_1 = 1;
pub const CAIRO_XCB_RENDER_MASK: C2RustUnnamed_1 = 4095;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub nodesize: libc::c_uint,
}
pub type cairo_freelist_t = _cairo_freelist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_font {
    pub base: cairo_scaled_font_private_t,
    pub scaled_font: *mut cairo_scaled_font_t,
    pub connection: *mut cairo_xcb_connection_t,
    pub glyphset_info: [cairo_xcb_font_glyphset_info_t; 3],
    pub link: cairo_list_t,
}
pub type cairo_xcb_font_glyphset_info_t = _cairo_xcb_font_glyphset_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_font_glyphset_info {
    pub glyphset: xcb_render_glyphset_t,
    pub format: cairo_format_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pending_free_glyphs: *mut cairo_xcb_font_glyphset_free_glyphs_t,
}
pub type cairo_xcb_font_glyphset_free_glyphs_t = _cairo_xcb_font_glyphset_free_glyphs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_font_glyphset_free_glyphs {
    pub glyphset: xcb_render_glyphset_t,
    pub glyph_count: libc::c_int,
    pub glyph_indices: [xcb_render_glyph_t; 128],
}
pub type cairo_xcb_font_t = _cairo_xcb_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_screen {
    pub connection: *mut cairo_xcb_connection_t,
    pub xcb_screen: *mut xcb_screen_t,
    pub subpixel_order: xcb_render_sub_pixel_t,
    pub gc: [xcb_gcontext_t; 4],
    pub gc_depths: [uint8_t; 4],
    pub stock_colors: [*mut cairo_surface_t; 3],
    pub solid_cache: [C2RustUnnamed_0; 16],
    pub solid_cache_size: libc::c_int,
    pub linear_pattern_cache: cairo_cache_t,
    pub radial_pattern_cache: cairo_cache_t,
    pub pattern_cache_entry_freelist: cairo_freelist_t,
    pub link: cairo_list_t,
    pub surfaces: cairo_list_t,
    pub pictures: cairo_list_t,
    pub has_font_options: cairo_bool_t,
    pub font_options: cairo_font_options_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub picture: *mut cairo_surface_t,
    pub color: cairo_color_t,
}
pub type cairo_xcb_screen_t = _cairo_xcb_screen;
pub type cairo_xcb_shm_mem_pool_t = _cairo_xcb_shm_mem_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_shm_info {
    pub connection: *mut cairo_xcb_connection_t,
    pub shm: uint32_t,
    pub offset: uint32_t,
    pub size: size_t,
    pub mem: *mut libc::c_void,
    pub pool: *mut cairo_xcb_shm_mem_pool_t,
    pub sync: xcb_get_input_focus_cookie_t,
    pub pending: cairo_list_t,
}
pub type cairo_xcb_shm_info_t = _cairo_xcb_shm_info;
pub type C2RustUnnamed_1 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: __time_t,
    pub shm_dtime: __time_t,
    pub shm_ctime: __time_t,
    pub shm_cpid: __pid_t,
    pub shm_lpid: __pid_t,
    pub shm_nattch: shmatt_t,
    pub __glibc_reserved5: __syscall_ulong_t,
    pub __glibc_reserved6: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
    pub __key: __key_t,
    pub uid: __uid_t,
    pub gid: __gid_t,
    pub cuid: __uid_t,
    pub cgid: __gid_t,
    pub mode: __mode_t,
    pub __seq: libc::c_ushort,
    pub __pad2: libc::c_ushort,
    pub __glibc_reserved1: __syscall_ulong_t,
    pub __glibc_reserved2: __syscall_ulong_t,
}
pub type xcb_shm_seg_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shm_query_version_reply_t {
    pub response_type: uint8_t,
    pub shared_pixmaps: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint16_t,
    pub minor_version: uint16_t,
    pub uid: uint16_t,
    pub gid: uint16_t,
    pub pixmap_format: uint8_t,
    pub pad0: [uint8_t; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shm_query_version_cookie_t {
    pub sequence: libc::c_uint,
}
pub type cairo_xcb_xrender_format_t = _cairo_xcb_xrender_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_xrender_format {
    pub key: cairo_hash_entry_t,
    pub xrender_format: xcb_render_pictformat_t,
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_destroy(
    mut connection: *mut cairo_xcb_connection_t,
) {
    cairo_device_destroy(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh0 = (*entry).next;
    *fresh0 = entry;
    let ref mut fresh1 = (*entry).prev;
    *fresh1 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh2 = (*next).prev;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).next;
    *fresh3 = next;
    let ref mut fresh4 = (*entry).prev;
    *fresh4 = prev;
    let ref mut fresh5 = (*prev).next;
    *fresh5 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh6 = (*next).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = next;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn cairo_list_del(mut entry: *mut cairo_list_t) {
    _cairo_list_del(entry);
    cairo_list_init(entry);
}
#[inline]
unsafe extern "C" fn cairo_list_move(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_del((*entry).prev, (*entry).next);
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
static mut connections: cairo_list_t = cairo_list_t {
    next: 0 as *const _cairo_list as *mut _cairo_list,
    prev: 0 as *const _cairo_list as *mut _cairo_list,
};
unsafe extern "C" fn _cairo_xcb_connection_find_visual_formats(
    mut connection: *mut cairo_xcb_connection_t,
    mut formats: *const xcb_render_query_pict_formats_reply_t,
) -> cairo_status_t {
    let mut screens: xcb_render_pictscreen_iterator_t = xcb_render_pictscreen_iterator_t {
        data: 0 as *mut xcb_render_pictscreen_t,
        rem: 0,
        index: 0,
    };
    let mut depths: xcb_render_pictdepth_iterator_t = xcb_render_pictdepth_iterator_t {
        data: 0 as *mut xcb_render_pictdepth_t,
        rem: 0,
        index: 0,
    };
    let mut visuals: xcb_render_pictvisual_iterator_t = xcb_render_pictvisual_iterator_t {
        data: 0 as *mut xcb_render_pictvisual_t,
        rem: 0,
        index: 0,
    };
    screens = xcb_render_query_pict_formats_screens_iterator(formats);
    while screens.rem != 0 {
        depths = xcb_render_pictscreen_depths_iterator(screens.data);
        while depths.rem != 0 {
            visuals = xcb_render_pictdepth_visuals_iterator(depths.data);
            while visuals.rem != 0 {
                let mut f: *mut cairo_xcb_xrender_format_t = 0
                    as *mut cairo_xcb_xrender_format_t;
                let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
                f = (if ::std::mem::size_of::<cairo_xcb_xrender_format_t>()
                    as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
                {
                    malloc(
                        ::std::mem::size_of::<cairo_xcb_xrender_format_t>()
                            as libc::c_ulong,
                    )
                } else {
                    0 as *mut libc::c_void
                }) as *mut cairo_xcb_xrender_format_t;
                if f.is_null() {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
                (*f).key.hash = (*visuals.data).visual as uintptr_t;
                (*f).xrender_format = (*visuals.data).format;
                status = _cairo_hash_table_insert(
                    (*connection).visual_to_xrender_format,
                    &mut (*f).key,
                );
                if status as u64 != 0 {
                    return status;
                }
                xcb_render_pictvisual_next(&mut visuals);
            }
            xcb_render_pictdepth_next(&mut depths);
        }
        xcb_render_pictscreen_next(&mut screens);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xcb_connection_parse_xrender_formats(
    mut connection: *mut cairo_xcb_connection_t,
    mut formats: *const xcb_render_query_pict_formats_reply_t,
) -> cairo_status_t {
    let mut i: xcb_render_pictforminfo_iterator_t = xcb_render_pictforminfo_iterator_t {
        data: 0 as *mut xcb_render_pictforminfo_t,
        rem: 0,
        index: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    i = xcb_render_query_pict_formats_formats_iterator(formats);
    while i.rem != 0 {
        let mut masks: cairo_format_masks_t = cairo_format_masks_t {
            bpp: 0,
            alpha_mask: 0,
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
        };
        let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
        if !((*i.data).type_0 as libc::c_int
            != XCB_RENDER_PICT_TYPE_DIRECT as libc::c_int)
        {
            masks
                .alpha_mask = ((*i.data).direct.alpha_mask as libc::c_ulong)
                << (*i.data).direct.alpha_shift as libc::c_int;
            masks
                .red_mask = ((*i.data).direct.red_mask as libc::c_ulong)
                << (*i.data).direct.red_shift as libc::c_int;
            masks
                .green_mask = ((*i.data).direct.green_mask as libc::c_ulong)
                << (*i.data).direct.green_shift as libc::c_int;
            masks
                .blue_mask = ((*i.data).direct.blue_mask as libc::c_ulong)
                << (*i.data).direct.blue_shift as libc::c_int;
            masks.bpp = (*i.data).depth as libc::c_int;
            if _pixman_format_from_masks(&mut masks, &mut pixman_format) != 0 {
                let mut key: cairo_hash_entry_t = cairo_hash_entry_t { hash: 0 };
                key.hash = pixman_format as uintptr_t;
                if (_cairo_hash_table_lookup((*connection).xrender_formats, &mut key))
                    .is_null()
                {
                    let mut f: *mut cairo_xcb_xrender_format_t = 0
                        as *mut cairo_xcb_xrender_format_t;
                    f = (if ::std::mem::size_of::<cairo_xcb_xrender_format_t>()
                        as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
                    {
                        malloc(
                            ::std::mem::size_of::<cairo_xcb_xrender_format_t>()
                                as libc::c_ulong,
                        )
                    } else {
                        0 as *mut libc::c_void
                    }) as *mut cairo_xcb_xrender_format_t;
                    if f.is_null() {
                        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    }
                    (*f).key.hash = pixman_format as uintptr_t;
                    (*f).xrender_format = (*i.data).id;
                    status = _cairo_hash_table_insert(
                        (*connection).xrender_formats,
                        &mut (*f).key,
                    );
                    if status as u64 != 0 {
                        return status;
                    }
                }
            }
        }
        xcb_render_pictforminfo_next(&mut i);
    }
    status = _cairo_xcb_connection_find_visual_formats(connection, formats);
    if status as u64 != 0 {
        return status;
    }
    (*connection)
        .standard_formats[CAIRO_FORMAT_A1 as libc::c_int
        as usize] = _cairo_xcb_connection_get_xrender_format(connection, PIXMAN_a1);
    (*connection)
        .standard_formats[CAIRO_FORMAT_A8 as libc::c_int
        as usize] = _cairo_xcb_connection_get_xrender_format(connection, PIXMAN_a8);
    (*connection)
        .standard_formats[CAIRO_FORMAT_RGB24 as libc::c_int
        as usize] = _cairo_xcb_connection_get_xrender_format(
        connection,
        ((24 as libc::c_int) << 24 as libc::c_int
            | (2 as libc::c_int) << 16 as libc::c_int
            | (0 as libc::c_int) << 12 as libc::c_int
            | (8 as libc::c_int) << 8 as libc::c_int
            | (8 as libc::c_int) << 4 as libc::c_int | 8 as libc::c_int)
            as pixman_format_code_t,
    );
    if (*connection).standard_formats[CAIRO_FORMAT_RGB24 as libc::c_int as usize]
        as libc::c_long == 0 as libc::c_long
    {
        (*connection)
            .standard_formats[CAIRO_FORMAT_RGB24 as libc::c_int
            as usize] = _cairo_xcb_connection_get_xrender_format(
            connection,
            ((24 as libc::c_int) << 24 as libc::c_int
                | (3 as libc::c_int) << 16 as libc::c_int
                | (0 as libc::c_int) << 12 as libc::c_int
                | (8 as libc::c_int) << 8 as libc::c_int
                | (8 as libc::c_int) << 4 as libc::c_int | 8 as libc::c_int)
                as pixman_format_code_t,
        );
    }
    (*connection)
        .standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int
        as usize] = _cairo_xcb_connection_get_xrender_format(
        connection,
        PIXMAN_a8r8g8b8,
    );
    if (*connection).standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int as usize]
        as libc::c_long == 0 as libc::c_long
    {
        (*connection)
            .standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int
            as usize] = _cairo_xcb_connection_get_xrender_format(
            connection,
            PIXMAN_a8b8g8r8,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn pixmap_depths_usable(
    mut connection: *mut cairo_xcb_connection_t,
    mut missing: uint32_t,
    mut root: xcb_drawable_t,
) -> cairo_bool_t {
    let mut c: *mut xcb_connection_t = (*connection).xcb_connection;
    let mut create_cookie: [xcb_void_cookie_t; 32] = [xcb_void_cookie_t {
        sequence: 0,
    }; 32];
    let mut pixmap: xcb_pixmap_t = 0;
    let mut success: cairo_bool_t = 1 as libc::c_int;
    let mut depth: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    pixmap = xcb_generate_id((*connection).xcb_connection);
    depth = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while depth <= 32 as libc::c_int {
        if missing & ((1 as libc::c_int) << depth - 1 as libc::c_int) as libc::c_uint
            != 0
        {
            create_cookie[i
                as usize] = xcb_create_pixmap_checked(
                c,
                depth as uint8_t,
                pixmap,
                root,
                1 as libc::c_int as uint16_t,
                1 as libc::c_int as uint16_t,
            );
            xcb_free_pixmap(c, pixmap);
            if create_cookie[i as usize].sequence == 0 {
                success = 0 as libc::c_int;
                break;
            } else {
                i += 1;
            }
        }
        depth += 1;
    }
    j = 0 as libc::c_int;
    while j < i {
        let mut create_error: *mut xcb_generic_error_t = xcb_request_check(
            c,
            create_cookie[j as usize],
        );
        success
            &= (create_error == 0 as *mut libc::c_void as *mut xcb_generic_error_t)
                as libc::c_int;
        free(create_error as *mut libc::c_void);
        j += 1;
    }
    return success;
}
unsafe extern "C" fn has_required_depths(
    mut connection: *mut cairo_xcb_connection_t,
) -> cairo_bool_t {
    let mut screens: xcb_screen_iterator_t = xcb_screen_iterator_t {
        data: 0 as *mut xcb_screen_t,
        rem: 0,
        index: 0,
    };
    screens = xcb_setup_roots_iterator((*connection).root);
    while screens.rem != 0 {
        let mut depths: xcb_depth_iterator_t = xcb_depth_iterator_t {
            data: 0 as *mut xcb_depth_t,
            rem: 0,
            index: 0,
        };
        let mut missing: uint32_t = ((1 as libc::c_int)
            << 1 as libc::c_int - 1 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int
            | (1 as libc::c_int) << 24 as libc::c_int - 1 as libc::c_int
            | (1 as libc::c_int) << 32 as libc::c_int - 1 as libc::c_int) as uint32_t;
        depths = xcb_screen_allowed_depths_iterator(screens.data);
        while depths.rem != 0 {
            missing
                &= !((1 as libc::c_int)
                    << (*depths.data).depth as libc::c_int - 1 as libc::c_int)
                    as libc::c_uint;
            xcb_depth_next(&mut depths);
        }
        if !(missing == 0 as libc::c_int as libc::c_uint) {
            if pixmap_depths_usable(connection, missing, (*screens.data).root) == 0 {
                return 0 as libc::c_int;
            }
        }
        xcb_screen_next(&mut screens);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _render_restrict_env(
    mut version: *mut xcb_render_query_version_reply_t,
) -> *mut xcb_render_query_version_reply_t {
    let mut env: *const libc::c_char = 0 as *const libc::c_char;
    if version.is_null() {
        return 0 as *mut xcb_render_query_version_reply_t;
    }
    env = getenv(b"CAIRO_DEBUG\0" as *const u8 as *const libc::c_char);
    if !env.is_null() {
        env = strstr(env, b"xrender-version=\0" as *const u8 as *const libc::c_char);
    }
    if !env.is_null() {
        let mut max_render_major: libc::c_int = 0;
        let mut max_render_minor: libc::c_int = 0;
        env = env
            .offset(
                (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        if sscanf(
            env,
            b"%d.%d\0" as *const u8 as *const libc::c_char,
            &mut max_render_major as *mut libc::c_int,
            &mut max_render_minor as *mut libc::c_int,
        ) != 2 as libc::c_int
        {
            max_render_minor = -(1 as libc::c_int);
            max_render_major = max_render_minor;
        }
        if max_render_major < 0 as libc::c_int || max_render_minor < 0 as libc::c_int {
            free(version as *mut libc::c_void);
            return 0 as *mut xcb_render_query_version_reply_t;
        }
        if max_render_major < (*version).major_version as libc::c_int
            || max_render_major == (*version).major_version as libc::c_int
                && max_render_minor < (*version).minor_version as libc::c_int
        {
            (*version).major_version = max_render_major as uint32_t;
            (*version).minor_version = max_render_minor as uint32_t;
        }
    }
    return version;
}
unsafe extern "C" fn _cairo_xcb_connection_query_render(
    mut connection: *mut cairo_xcb_connection_t,
) -> cairo_status_t {
    let mut c: *mut xcb_connection_t = (*connection).xcb_connection;
    let mut version_cookie: xcb_render_query_version_cookie_t = xcb_render_query_version_cookie_t {
        sequence: 0,
    };
    let mut formats_cookie: xcb_render_query_pict_formats_cookie_t = xcb_render_query_pict_formats_cookie_t {
        sequence: 0,
    };
    let mut version: *mut xcb_render_query_version_reply_t = 0
        as *mut xcb_render_query_version_reply_t;
    let mut formats: *mut xcb_render_query_pict_formats_reply_t = 0
        as *mut xcb_render_query_pict_formats_reply_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut present: cairo_bool_t = 0;
    version_cookie = xcb_render_query_version(
        c,
        0 as libc::c_int as uint32_t,
        11 as libc::c_int as uint32_t,
    );
    formats_cookie = xcb_render_query_pict_formats(c);
    present = has_required_depths(connection);
    version = xcb_render_query_version_reply(
        c,
        version_cookie,
        0 as *mut *mut xcb_generic_error_t,
    );
    formats = xcb_render_query_pict_formats_reply(
        c,
        formats_cookie,
        0 as *mut *mut xcb_generic_error_t,
    );
    version = _render_restrict_env(version);
    if present == 0 || version.is_null() || formats.is_null() {
        free(version as *mut libc::c_void);
        free(formats as *mut libc::c_void);
        return CAIRO_STATUS_SUCCESS;
    }
    (*connection).flags |= CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint;
    (*connection).flags |= CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint;
    (*connection).flags
        |= CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS as libc::c_int as libc::c_uint;
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 1 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 4 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 6 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 6 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_FILTERS as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 11 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_PDF_OPERATORS as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 10 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 10 as libc::c_int as libc::c_uint
    {
        (*connection).flags
            |= CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint;
    }
    if (*version).major_version > 0 as libc::c_int as libc::c_uint
        || (*version).major_version == 0 as libc::c_int as libc::c_uint
            && (*version).minor_version >= 6 as libc::c_int as libc::c_uint
    {
        let mut screen: uint32_t = 0;
        let mut subpixel: *mut uint32_t = xcb_render_query_pict_formats_subpixels(
            formats,
        );
        screen = 0 as libc::c_int as uint32_t;
        while screen < (*formats).num_subpixel
            && screen < (*(*connection).root).roots_len as libc::c_uint
        {
            *((*connection).subpixel_orders)
                .offset(
                    screen as isize,
                ) = *subpixel.offset(screen as isize) as xcb_render_sub_pixel_t;
            screen = screen.wrapping_add(1);
        }
    }
    free(version as *mut libc::c_void);
    status = _cairo_xcb_connection_parse_xrender_formats(connection, formats);
    free(formats as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn can_use_shm(
    mut connection: *mut cairo_xcb_connection_t,
) -> cairo_bool_t {
    let mut success: cairo_bool_t = 1 as libc::c_int;
    let mut c: *mut xcb_connection_t = (*connection).xcb_connection;
    let mut cookie: [xcb_void_cookie_t; 2] = [xcb_void_cookie_t { sequence: 0 }; 2];
    let mut error: *mut xcb_generic_error_t = 0 as *mut xcb_generic_error_t;
    let mut shmid: libc::c_int = 0;
    let mut shmseg: uint32_t = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    shmid = shmget(
        0 as libc::c_int,
        0x1000 as libc::c_int as size_t,
        0o1000 as libc::c_int | 0o600 as libc::c_int,
    );
    if shmid == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    ptr = shmat(shmid, 0 as *const libc::c_void, 0 as libc::c_int);
    if ptr == -(1 as libc::c_int) as *mut libc::c_char as *mut libc::c_void {
        shmctl(shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
        return 0 as libc::c_int;
    }
    shmseg = xcb_generate_id((*connection).xcb_connection);
    cookie[0 as libc::c_int
        as usize] = xcb_shm_attach_checked(
        c,
        shmseg,
        shmid as uint32_t,
        0 as libc::c_int as uint8_t,
    );
    cookie[1 as libc::c_int as usize] = xcb_shm_detach_checked(c, shmseg);
    error = xcb_request_check(c, cookie[0 as libc::c_int as usize]);
    if !error.is_null() {
        success = 0 as libc::c_int;
    }
    error = xcb_request_check(c, cookie[1 as libc::c_int as usize]);
    if !error.is_null() {
        success = 0 as libc::c_int;
    }
    shmctl(shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
    shmdt(ptr);
    return success;
}
unsafe extern "C" fn _cairo_xcb_connection_query_shm(
    mut connection: *mut cairo_xcb_connection_t,
) {
    let mut c: *mut xcb_connection_t = (*connection).xcb_connection;
    let mut version: *mut xcb_shm_query_version_reply_t = 0
        as *mut xcb_shm_query_version_reply_t;
    version = xcb_shm_query_version_reply(
        c,
        xcb_shm_query_version(c),
        0 as *mut *mut xcb_generic_error_t,
    );
    if version.is_null() {
        return;
    }
    free(version as *mut libc::c_void);
    if can_use_shm(connection) != 0 {
        (*connection).flags |= CAIRO_XCB_HAS_SHM as libc::c_uint;
    }
}
unsafe extern "C" fn _device_flush(mut device: *mut libc::c_void) -> cairo_status_t {
    let mut connection: *mut cairo_xcb_connection_t = device
        as *mut cairo_xcb_connection_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_device_acquire(&mut (*connection).device);
    if status as u64 != 0 {
        return status;
    }
    _cairo_xcb_connection_shm_mem_pools_flush(connection);
    xcb_flush((*connection).xcb_connection);
    cairo_device_release(&mut (*connection).device);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _pluck_xrender_format(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    _cairo_hash_table_remove(
        closure as *mut cairo_hash_table_t,
        entry as *mut cairo_hash_entry_t,
    );
    free(entry);
}
unsafe extern "C" fn _device_finish(mut device: *mut libc::c_void) {
    let mut connection: *mut cairo_xcb_connection_t = device
        as *mut cairo_xcb_connection_t;
    let mut was_cached: cairo_bool_t = 0 as libc::c_int;
    if cairo_list_is_empty(&mut (*connection).link) == 0 {
        pthread_mutex_lock(&mut _cairo_xcb_connections_mutex);
        cairo_list_del(&mut (*connection).link);
        pthread_mutex_unlock(&mut _cairo_xcb_connections_mutex);
        was_cached = 1 as libc::c_int;
    }
    while cairo_list_is_empty(&mut (*connection).fonts) == 0 {
        let mut font: *mut cairo_xcb_font_t = 0 as *mut cairo_xcb_font_t;
        font = ({
            let mut mptr__: *const cairo_list_t = (*connection).fonts.next;
            (mptr__ as *mut libc::c_char).offset(-(120 as libc::c_ulong as isize))
                as *mut cairo_xcb_font_t
        });
        _cairo_xcb_font_close(font);
    }
    while cairo_list_is_empty(&mut (*connection).screens) == 0 {
        let mut screen: *mut cairo_xcb_screen_t = 0 as *mut cairo_xcb_screen_t;
        screen = ({
            let mut mptr__: *const cairo_list_t = (*connection).screens.next;
            (mptr__ as *mut libc::c_char).offset(-(952 as libc::c_ulong as isize))
                as *mut cairo_xcb_screen_t
        });
        _cairo_xcb_screen_finish(screen);
    }
    _cairo_xcb_connection_shm_mem_pools_flush(connection);
    if was_cached != 0 {
        cairo_device_destroy(device as *mut cairo_device_t);
    }
}
unsafe extern "C" fn _device_destroy(mut device: *mut libc::c_void) {
    let mut connection: *mut cairo_xcb_connection_t = device
        as *mut cairo_xcb_connection_t;
    _cairo_hash_table_foreach(
        (*connection).xrender_formats,
        Some(
            _pluck_xrender_format
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*connection).xrender_formats as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*connection).xrender_formats);
    _cairo_hash_table_foreach(
        (*connection).visual_to_xrender_format,
        Some(
            _pluck_xrender_format
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*connection).visual_to_xrender_format as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*connection).visual_to_xrender_format);
    _cairo_xcb_connection_shm_mem_pools_fini(connection);
    _cairo_freepool_fini(&mut (*connection).shm_info_freelist);
    pthread_mutex_destroy(&mut (*connection).shm_mutex);
    pthread_mutex_destroy(&mut (*connection).screens_mutex);
    free((*connection).subpixel_orders as *mut libc::c_void);
    free(connection as *mut libc::c_void);
}
static mut _cairo_xcb_device_backend: cairo_device_backend_t = unsafe {
    {
        let mut init = _cairo_device_backend {
            type_0: CAIRO_DEVICE_TYPE_XCB,
            lock: None,
            unlock: None,
            flush: Some(
                _device_flush
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            finish: Some(
                _device_finish as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            destroy: Some(
                _device_destroy as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_get(
    mut xcb_connection: *mut xcb_connection_t,
) -> *mut cairo_xcb_connection_t {
    let mut current_block: u64;
    let mut connection: *mut cairo_xcb_connection_t = 0 as *mut cairo_xcb_connection_t;
    let mut ext: *const xcb_query_extension_reply_t = 0
        as *const xcb_query_extension_reply_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    pthread_mutex_lock(&mut _cairo_xcb_connections_mutex);
    if (connections.next).is_null() {
        cairo_list_init(&mut connections);
    }
    connection = ({
        let mut mptr__: *const cairo_list_t = connections.next;
        (mptr__ as *mut libc::c_char).offset(-(1384 as libc::c_ulong as isize))
            as *mut cairo_xcb_connection_t
    });
    loop {
        if !(&mut (*connection).link as *mut cairo_list_t
            != &mut connections as *mut cairo_list_t)
        {
            current_block = 4166486009154926805;
            break;
        }
        if (*connection).xcb_connection == xcb_connection {
            if connections.next != &mut (*connection).link as *mut cairo_list_t {
                cairo_list_move(&mut (*connection).link, &mut connections);
            }
            current_block = 3135099802553498569;
            break;
        } else {
            connection = ({
                let mut mptr__: *const cairo_list_t = (*connection).link.next;
                (mptr__ as *mut libc::c_char).offset(-(1384 as libc::c_ulong as isize))
                    as *mut cairo_xcb_connection_t
            });
        }
    }
    match current_block {
        4166486009154926805 => {
            connection = (if ::std::mem::size_of::<cairo_xcb_connection_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_xcb_connection_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_xcb_connection_t;
            if !connection.is_null() {
                _cairo_device_init(
                    &mut (*connection).device,
                    &_cairo_xcb_device_backend,
                );
                let ref mut fresh8 = (*connection).xcb_connection;
                *fresh8 = xcb_connection;
                cairo_list_init(&mut (*connection).fonts);
                cairo_list_init(&mut (*connection).screens);
                cairo_list_init(&mut (*connection).link);
                let ref mut fresh9 = (*connection).xrender_formats;
                *fresh9 = _cairo_hash_table_create(None);
                if ((*connection).xrender_formats).is_null() {
                    pthread_mutex_destroy(&mut (*connection).device.mutex);
                    free(connection as *mut libc::c_void);
                    connection = 0 as *mut cairo_xcb_connection_t;
                } else {
                    let ref mut fresh10 = (*connection).visual_to_xrender_format;
                    *fresh10 = _cairo_hash_table_create(None);
                    if ((*connection).visual_to_xrender_format).is_null() {
                        _cairo_hash_table_destroy((*connection).xrender_formats);
                        pthread_mutex_destroy(&mut (*connection).device.mutex);
                        free(connection as *mut libc::c_void);
                        connection = 0 as *mut cairo_xcb_connection_t;
                    } else {
                        cairo_list_init(&mut (*connection).shm_pools);
                        cairo_list_init(&mut (*connection).shm_pending);
                        _cairo_freepool_init(
                            &mut (*connection).shm_info_freelist,
                            ::std::mem::size_of::<cairo_xcb_shm_info_t>()
                                as libc::c_ulong as libc::c_uint,
                        );
                        (*connection)
                            .maximum_request_length = xcb_get_maximum_request_length(
                            xcb_connection,
                        );
                        let mut _tmp_mutex: cairo_mutex_t = pthread_mutex_t {
                            __data: {
                                let mut init = __pthread_mutex_s {
                                    __lock: 0 as libc::c_int,
                                    __count: 0 as libc::c_int as libc::c_uint,
                                    __owner: 0 as libc::c_int,
                                    __nusers: 0 as libc::c_int as libc::c_uint,
                                    __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                                    __spins: 0 as libc::c_int as libc::c_short,
                                    __elision: 0 as libc::c_int as libc::c_short,
                                    __list: {
                                        let mut init = __pthread_internal_list {
                                            __prev: 0 as *mut __pthread_internal_list,
                                            __next: 0 as *mut __pthread_internal_list,
                                        };
                                        init
                                    },
                                };
                                init
                            },
                        };
                        memcpy(
                            &mut (*connection).shm_mutex as *mut cairo_mutex_t
                                as *mut libc::c_void,
                            &mut _tmp_mutex as *mut cairo_mutex_t as *const libc::c_void,
                            ::std::mem::size_of::<cairo_mutex_t>() as libc::c_ulong,
                        );
                        let mut _tmp_mutex_0: cairo_mutex_t = pthread_mutex_t {
                            __data: {
                                let mut init = __pthread_mutex_s {
                                    __lock: 0 as libc::c_int,
                                    __count: 0 as libc::c_int as libc::c_uint,
                                    __owner: 0 as libc::c_int,
                                    __nusers: 0 as libc::c_int as libc::c_uint,
                                    __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                                    __spins: 0 as libc::c_int as libc::c_short,
                                    __elision: 0 as libc::c_int as libc::c_short,
                                    __list: {
                                        let mut init = __pthread_internal_list {
                                            __prev: 0 as *mut __pthread_internal_list,
                                            __next: 0 as *mut __pthread_internal_list,
                                        };
                                        init
                                    },
                                };
                                init
                            },
                        };
                        memcpy(
                            &mut (*connection).screens_mutex as *mut cairo_mutex_t
                                as *mut libc::c_void,
                            &mut _tmp_mutex_0 as *mut cairo_mutex_t
                                as *const libc::c_void,
                            ::std::mem::size_of::<cairo_mutex_t>() as libc::c_ulong,
                        );
                        pthread_mutex_lock(&mut (*connection).device.mutex);
                        (*connection).flags = 0 as libc::c_int as libc::c_uint;
                        (*connection).force_precision = -(1 as libc::c_int);
                        xcb_prefetch_extension_data(
                            xcb_connection,
                            &mut xcb_big_requests_id,
                        );
                        xcb_prefetch_extension_data(xcb_connection, &mut xcb_render_id);
                        xcb_prefetch_extension_data(xcb_connection, &mut xcb_shm_id);
                        xcb_prefetch_maximum_request_length(xcb_connection);
                        let ref mut fresh11 = (*connection).root;
                        *fresh11 = xcb_get_setup(xcb_connection);
                        let ref mut fresh12 = (*connection).render;
                        *fresh12 = 0 as *const xcb_query_extension_reply_t;
                        let ref mut fresh13 = (*connection).subpixel_orders;
                        *fresh13 = calloc(
                            (*(*connection).root).roots_len as libc::c_ulong,
                            ::std::mem::size_of::<xcb_render_sub_pixel_t>()
                                as libc::c_ulong,
                        ) as *mut xcb_render_sub_pixel_t;
                        if ((*connection).subpixel_orders).is_null() {
                            pthread_mutex_unlock(&mut (*connection).device.mutex);
                            _cairo_xcb_connection_destroy(connection);
                            connection = 0 as *mut cairo_xcb_connection_t;
                        } else {
                            ext = xcb_get_extension_data(
                                xcb_connection,
                                &mut xcb_render_id,
                            );
                            if !ext.is_null() && (*ext).present as libc::c_int != 0 {
                                status = _cairo_xcb_connection_query_render(connection);
                                if status as u64 != 0 {
                                    pthread_mutex_unlock(&mut (*connection).device.mutex);
                                    _cairo_xcb_connection_destroy(connection);
                                    connection = 0 as *mut cairo_xcb_connection_t;
                                    current_block = 3135099802553498569;
                                } else {
                                    let ref mut fresh14 = (*connection).render;
                                    *fresh14 = ext;
                                    current_block = 16203797167131938757;
                                }
                            } else {
                                current_block = 16203797167131938757;
                            }
                            match current_block {
                                3135099802553498569 => {}
                                _ => {
                                    let ref mut fresh15 = (*connection).shm;
                                    *fresh15 = 0 as *const xcb_query_extension_reply_t;
                                    ext = xcb_get_extension_data(
                                        xcb_connection,
                                        &mut xcb_shm_id,
                                    );
                                    if !ext.is_null() && (*ext).present as libc::c_int != 0 {
                                        _cairo_xcb_connection_query_shm(connection);
                                        let ref mut fresh16 = (*connection).shm;
                                        *fresh16 = ext;
                                    }
                                    (*connection).original_flags = (*connection).flags;
                                    pthread_mutex_unlock(&mut (*connection).device.mutex);
                                    cairo_list_add(&mut (*connection).link, &mut connections);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    pthread_mutex_unlock(&mut _cairo_xcb_connections_mutex);
    return connection;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_get_xrender_format(
    mut connection: *mut cairo_xcb_connection_t,
    mut pixman_format: pixman_format_code_t,
) -> xcb_render_pictformat_t {
    let mut key: cairo_hash_entry_t = cairo_hash_entry_t { hash: 0 };
    let mut format: *mut cairo_xcb_xrender_format_t = 0
        as *mut cairo_xcb_xrender_format_t;
    key.hash = pixman_format as uintptr_t;
    format = _cairo_hash_table_lookup((*connection).xrender_formats, &mut key)
        as *mut cairo_xcb_xrender_format_t;
    return (if !format.is_null() {
        (*format).xrender_format as libc::c_long
    } else {
        0 as libc::c_long
    }) as xcb_render_pictformat_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_get_xrender_format_for_visual(
    mut connection: *mut cairo_xcb_connection_t,
    visual: xcb_visualid_t,
) -> xcb_render_pictformat_t {
    let mut key: cairo_hash_entry_t = cairo_hash_entry_t { hash: 0 };
    let mut format: *mut cairo_xcb_xrender_format_t = 0
        as *mut cairo_xcb_xrender_format_t;
    key.hash = visual as uintptr_t;
    format = _cairo_hash_table_lookup((*connection).visual_to_xrender_format, &mut key)
        as *mut cairo_xcb_xrender_format_t;
    return (if !format.is_null() {
        (*format).xrender_format as libc::c_long
    } else {
        0 as libc::c_long
    }) as xcb_render_pictformat_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_device_get_connection(
    mut device: *mut cairo_device_t,
) -> *mut xcb_connection_t {
    if (*(*device).backend).type_0 as libc::c_int != CAIRO_DEVICE_TYPE_XCB as libc::c_int
    {
        return 0 as *mut xcb_connection_t;
    }
    return (*(device as *mut cairo_xcb_connection_t)).xcb_connection;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_device_debug_cap_xshm_version(
    mut device: *mut cairo_device_t,
    mut major_version: libc::c_int,
    mut minor_version: libc::c_int,
) {
    let mut connection: *mut cairo_xcb_connection_t = device
        as *mut cairo_xcb_connection_t;
    if (*(*device).backend).type_0 as libc::c_int != CAIRO_DEVICE_TYPE_XCB as libc::c_int
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
        return;
    }
    (*connection).flags
        |= (*connection).original_flags & CAIRO_XCB_SHM_MASK as libc::c_uint;
    if major_version < 0 as libc::c_int && minor_version < 0 as libc::c_int {
        (*connection).flags &= !(CAIRO_XCB_HAS_SHM as libc::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_device_debug_cap_xrender_version(
    mut device: *mut cairo_device_t,
    mut major_version: libc::c_int,
    mut minor_version: libc::c_int,
) {
    let mut connection: *mut cairo_xcb_connection_t = device
        as *mut cairo_xcb_connection_t;
    if (*(*device).backend).type_0 as libc::c_int != CAIRO_DEVICE_TYPE_XCB as libc::c_int
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
        return;
    }
    (*connection).flags
        |= (*connection).original_flags
            & CAIRO_XCB_RENDER_MASK as libc::c_int as libc::c_uint;
    if major_version < 0 as libc::c_int && minor_version < 0 as libc::c_int {
        (*connection).flags
            &= !(CAIRO_XCB_HAS_RENDER as libc::c_int
                | CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int
                | CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS as libc::c_int
                | CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int
                | CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int
                | CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM as libc::c_int
                | CAIRO_XCB_RENDER_HAS_FILTERS as libc::c_int
                | CAIRO_XCB_RENDER_HAS_FILTER_GOOD as libc::c_int
                | CAIRO_XCB_RENDER_HAS_FILTER_BEST as libc::c_int
                | CAIRO_XCB_RENDER_HAS_PDF_OPERATORS as libc::c_int
                | CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT as libc::c_int
                | CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int) as libc::c_uint;
    } else {
        let mut version: xcb_render_query_version_reply_t = xcb_render_query_version_reply_t {
            response_type: 0,
            pad0: 0,
            sequence: 0,
            length: 0,
            major_version: 0,
            minor_version: 0,
            pad1: [0; 16],
        };
        version.major_version = major_version as uint32_t;
        version.minor_version = minor_version as uint32_t;
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 1 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int)
                    as libc::c_uint;
        }
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 4 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int)
                    as libc::c_uint;
        }
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 6 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM as libc::c_int)
                    as libc::c_uint;
        }
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 6 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_FILTERS as libc::c_int) as libc::c_uint;
        }
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 11 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_PDF_OPERATORS as libc::c_int) as libc::c_uint;
        }
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 10 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT as libc::c_int)
                    as libc::c_uint;
        }
        if !(version.major_version > 0 as libc::c_int as libc::c_uint
            || version.major_version == 0 as libc::c_int as libc::c_uint
                && version.minor_version >= 10 as libc::c_int as libc::c_uint)
        {
            (*connection).flags
                &= !(CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int) as libc::c_uint;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_device_debug_set_precision(
    mut device: *mut cairo_device_t,
    mut precision: libc::c_int,
) {
    if device.is_null() || (*device).status as libc::c_uint != 0 {
        return;
    }
    if (*(*device).backend).type_0 as libc::c_int != CAIRO_DEVICE_TYPE_XCB as libc::c_int
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
        return;
    }
    (*(device as *mut cairo_xcb_connection_t)).force_precision = precision;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_device_debug_get_precision(
    mut device: *mut cairo_device_t,
) -> libc::c_int {
    if device.is_null() || (*device).status as libc::c_uint != 0 {
        return -(1 as libc::c_int);
    }
    if (*(*device).backend).type_0 as libc::c_int != CAIRO_DEVICE_TYPE_XCB as libc::c_int
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
        return -(1 as libc::c_int);
    }
    return (*(device as *mut cairo_xcb_connection_t)).force_precision;
}
