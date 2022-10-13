use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t,
        antialias: cairo_antialias_t,
    );
    fn cairo_font_options_set_subpixel_order(
        options: *mut cairo_font_options_t,
        subpixel_order: cairo_subpixel_order_t,
    );
    fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t,
        hint_style: cairo_hint_style_t,
    );
    fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t,
        hint_metrics: cairo_hint_metrics_t,
    );
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_cache_init(
        cache: *mut cairo_cache_t,
        keys_equal: cairo_cache_keys_equal_func_t,
        predicate: cairo_cache_predicate_func_t,
        entry_destroy: cairo_destroy_func_t,
        max_size: libc::c_ulong,
    ) -> cairo_status_t;
    fn _cairo_cache_fini(cache: *mut cairo_cache_t);
    fn _cairo_cache_lookup(
        cache: *mut cairo_cache_t,
        key: *mut cairo_cache_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_cache_insert(
        cache: *mut cairo_cache_t,
        entry: *mut cairo_cache_entry_t,
    ) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_font_options_set_lcd_filter(
        options: *mut cairo_font_options_t,
        lcd_filter: cairo_lcd_filter_t,
    );
    fn _cairo_font_options_set_round_glyph_positions(
        options: *mut cairo_font_options_t,
        round: cairo_round_glyph_positions_t,
    );
    fn xcb_screen_next(i: *mut xcb_screen_iterator_t);
    fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
    fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
    fn _cairo_xcb_resources_get(
        screen: *mut cairo_xcb_screen_t,
        resources: *mut cairo_xcb_resources_t,
    );
    fn _cairo_xcb_connection_create_gc(
        connection: *mut cairo_xcb_connection_t,
        drawable: xcb_drawable_t,
        value_mask: uint32_t,
        values: *mut uint32_t,
    ) -> xcb_gcontext_t;
    fn _cairo_xcb_connection_get(
        connection: *mut xcb_connection_t,
    ) -> *mut cairo_xcb_connection_t;
    fn _cairo_radial_pattern_equal(
        a: *const cairo_radial_pattern_t,
        b: *const cairo_radial_pattern_t,
    ) -> cairo_bool_t;
    fn _cairo_linear_pattern_equal(
        a: *const cairo_linear_pattern_t,
        b: *const cairo_linear_pattern_t,
    ) -> cairo_bool_t;
    fn _cairo_radial_pattern_hash(
        hash: uintptr_t,
        radial: *const cairo_radial_pattern_t,
    ) -> uintptr_t;
    fn _cairo_linear_pattern_hash(
        hash: uintptr_t,
        linear: *const cairo_linear_pattern_t,
    ) -> uintptr_t;
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_init_static_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    );
    fn _cairo_pattern_init_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_freelist_free(freelist: *mut cairo_freelist_t, node: *mut libc::c_void);
    fn _cairo_freelist_alloc(freelist: *mut cairo_freelist_t) -> *mut libc::c_void;
    fn _cairo_freelist_fini(freelist: *mut cairo_freelist_t);
    fn _cairo_freelist_init(freelist: *mut cairo_freelist_t, nodesize: libc::c_uint);
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
pub type cairo_raster_source_acquire_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
        *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t,
>;
pub type cairo_raster_source_release_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
    ) -> (),
>;
pub type cairo_raster_source_snapshot_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> cairo_status_t,
>;
pub type cairo_raster_source_copy_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *const cairo_pattern_t,
    ) -> cairo_status_t,
>;
pub type cairo_raster_source_finish_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> (),
>;
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
pub struct _cairo_boxes_t {
    pub status: cairo_status_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_boxes: libc::c_int,
    pub is_pixel_aligned: libc::c_uint,
    pub chunks: _cairo_boxes_chunk,
    pub tail: *mut _cairo_boxes_chunk,
    pub boxes_embedded: [cairo_box_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_chunk {
    pub next: *mut _cairo_boxes_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_boxes_t = _cairo_boxes_t;
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
pub struct _cairo_color_stop {
    pub red: libc::c_double,
    pub green: libc::c_double,
    pub blue: libc::c_double,
    pub alpha: libc::c_double,
    pub red_short: uint16_t,
    pub green_short: uint16_t,
    pub blue_short: uint16_t,
    pub alpha_short: uint16_t,
}
pub type cairo_color_stop_t = _cairo_color_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_circle_double {
    pub center: cairo_point_double_t,
    pub radius: libc::c_double,
}
pub type cairo_circle_double_t = _cairo_circle_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cache_entry {
    pub hash: uintptr_t,
    pub size: libc::c_ulong,
}
pub type cairo_cache_entry_t = _cairo_cache_entry;
pub type cairo_cache_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
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
pub type xcb_gc_t = libc::c_uint;
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
pub const XCB_GC_FONT: xcb_gc_t = 16384;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
pub const XCB_GC_TILE: xcb_gc_t = 1024;
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
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
pub type xcb_render_picture_t = uint32_t;
pub type xcb_render_pictformat_t = uint32_t;
pub type xcb_render_fixed_t = int32_t;
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
pub struct _cairo_freelist {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub nodesize: libc::c_uint,
}
pub type cairo_freelist_t = _cairo_freelist;
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
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_stop {
    pub offset: libc::c_double,
    pub color: cairo_color_stop_t,
}
pub type cairo_gradient_stop_t = _cairo_gradient_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_pattern {
    pub base: cairo_pattern_t,
    pub n_stops: libc::c_uint,
    pub stops_size: libc::c_uint,
    pub stops: *mut cairo_gradient_stop_t,
    pub stops_embedded: [cairo_gradient_stop_t; 2],
}
pub type cairo_gradient_pattern_t = _cairo_gradient_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_linear_pattern {
    pub base: cairo_gradient_pattern_t,
    pub pd1: cairo_point_double_t,
    pub pd2: cairo_point_double_t,
}
pub type cairo_linear_pattern_t = _cairo_linear_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_radial_pattern {
    pub base: cairo_gradient_pattern_t,
    pub cd1: cairo_circle_double_t,
    pub cd2: cairo_circle_double_t,
}
pub type cairo_radial_pattern_t = _cairo_radial_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_gradient_pattern_union_t {
    pub base: cairo_gradient_pattern_t,
    pub linear: cairo_linear_pattern_t,
    pub radial: cairo_radial_pattern_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_patch {
    pub points: [[cairo_point_double_t; 4]; 4],
    pub colors: [cairo_color_t; 4],
}
pub type cairo_mesh_patch_t = _cairo_mesh_patch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_pattern {
    pub base: cairo_pattern_t,
    pub patches: cairo_array_t,
    pub current_patch: *mut cairo_mesh_patch_t,
    pub current_side: libc::c_int,
    pub has_control_point: [cairo_bool_t; 4],
    pub has_color: [cairo_bool_t; 4],
}
pub type cairo_mesh_pattern_t = _cairo_mesh_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_raster_source_pattern {
    pub base: cairo_pattern_t,
    pub content: cairo_content_t,
    pub extents: cairo_rectangle_int_t,
    pub acquire: cairo_raster_source_acquire_func_t,
    pub release: cairo_raster_source_release_func_t,
    pub snapshot: cairo_raster_source_snapshot_func_t,
    pub copy: cairo_raster_source_copy_func_t,
    pub finish: cairo_raster_source_finish_func_t,
    pub user_data: *mut libc::c_void,
}
pub type cairo_raster_source_pattern_t = _cairo_raster_source_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_pattern_union_t {
    pub base: cairo_pattern_t,
    pub solid: cairo_solid_pattern_t,
    pub surface: cairo_surface_pattern_t,
    pub gradient: cairo_gradient_pattern_union_t,
    pub mesh: cairo_mesh_pattern_t,
    pub raster_source: cairo_raster_source_pattern_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_screen {
    pub connection: *mut cairo_xcb_connection_t,
    pub xcb_screen: *mut xcb_screen_t,
    pub subpixel_order: xcb_render_sub_pixel_t,
    pub gc: [xcb_gcontext_t; 4],
    pub gc_depths: [uint8_t; 4],
    pub stock_colors: [*mut cairo_surface_t; 3],
    pub solid_cache: [C2RustUnnamed; 16],
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
pub struct C2RustUnnamed {
    pub picture: *mut cairo_surface_t,
    pub color: cairo_color_t,
}
pub type cairo_xcb_screen_t = _cairo_xcb_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_surface {
    pub base: cairo_surface_t,
    pub fallback: *mut cairo_image_surface_t,
    pub fallback_damage: cairo_boxes_t,
    pub connection: *mut cairo_xcb_connection_t,
    pub screen: *mut cairo_xcb_screen_t,
    pub drawable: xcb_drawable_t,
    pub owns_pixmap: cairo_bool_t,
    pub deferred_clear: cairo_bool_t,
    pub deferred_clear_color: cairo_color_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub picture: xcb_render_picture_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pixman_format: pixman_format_code_t,
    pub precision: uint32_t,
    pub link: cairo_list_t,
}
pub type cairo_xcb_surface_t = _cairo_xcb_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_picture {
    pub base: cairo_surface_t,
    pub screen: *mut cairo_xcb_screen_t,
    pub picture: xcb_render_picture_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pixman_format: pixman_format_code_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub extend: cairo_extend_t,
    pub filter: cairo_filter_t,
    pub has_component_alpha: cairo_bool_t,
    pub transform: xcb_render_transform_t,
    pub x0: libc::c_int,
    pub y0: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub link: cairo_list_t,
}
pub type cairo_xcb_picture_t = _cairo_xcb_picture;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_resources {
    pub xft_antialias: cairo_bool_t,
    pub xft_lcdfilter: libc::c_int,
    pub xft_hinting: cairo_bool_t,
    pub xft_hintstyle: libc::c_int,
    pub xft_rgba: libc::c_int,
}
pub type cairo_xcb_resources_t = _cairo_xcb_resources;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_cache_entry {
    pub key: cairo_cache_entry_t,
    pub screen: *mut cairo_xcb_screen_t,
    pub pattern: cairo_pattern_union_t,
    pub picture: *mut cairo_surface_t,
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_release(
    mut connection: *mut cairo_xcb_connection_t,
) {
    cairo_device_release(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_acquire(
    mut connection: *mut cairo_xcb_connection_t,
) -> cairo_status_t {
    return cairo_device_acquire(&mut (*connection).device);
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
unsafe extern "C" fn _cairo_xcb_init_screen_font_options(
    mut screen: *mut cairo_xcb_screen_t,
) {
    let mut res: cairo_xcb_resources_t = cairo_xcb_resources_t {
        xft_antialias: 0,
        xft_lcdfilter: 0,
        xft_hinting: 0,
        xft_hintstyle: 0,
        xft_rgba: 0,
    };
    let mut antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
    let mut subpixel_order: cairo_subpixel_order_t = CAIRO_SUBPIXEL_ORDER_DEFAULT;
    let mut lcd_filter: cairo_lcd_filter_t = CAIRO_LCD_FILTER_DEFAULT;
    let mut hint_style: cairo_hint_style_t = CAIRO_HINT_STYLE_DEFAULT;
    _cairo_xcb_resources_get(screen, &mut res);
    if res.xft_hinting != 0 {
        match res.xft_hintstyle {
            0 => {
                hint_style = CAIRO_HINT_STYLE_NONE;
            }
            1 => {
                hint_style = CAIRO_HINT_STYLE_SLIGHT;
            }
            2 => {
                hint_style = CAIRO_HINT_STYLE_MEDIUM;
            }
            3 => {
                hint_style = CAIRO_HINT_STYLE_FULL;
            }
            _ => {
                hint_style = CAIRO_HINT_STYLE_DEFAULT;
            }
        }
    } else {
        hint_style = CAIRO_HINT_STYLE_NONE;
    }
    match res.xft_rgba {
        1 => {
            subpixel_order = CAIRO_SUBPIXEL_ORDER_RGB;
        }
        2 => {
            subpixel_order = CAIRO_SUBPIXEL_ORDER_BGR;
        }
        3 => {
            subpixel_order = CAIRO_SUBPIXEL_ORDER_VRGB;
        }
        4 => {
            subpixel_order = CAIRO_SUBPIXEL_ORDER_VBGR;
        }
        0 | 5 | _ => {
            subpixel_order = CAIRO_SUBPIXEL_ORDER_DEFAULT;
        }
    }
    match res.xft_lcdfilter {
        0 => {
            lcd_filter = CAIRO_LCD_FILTER_NONE;
        }
        1 => {
            lcd_filter = CAIRO_LCD_FILTER_FIR5;
        }
        2 => {
            lcd_filter = CAIRO_LCD_FILTER_FIR3;
        }
        3 => {
            lcd_filter = CAIRO_LCD_FILTER_INTRA_PIXEL;
        }
        _ => {
            lcd_filter = CAIRO_LCD_FILTER_DEFAULT;
        }
    }
    if res.xft_antialias != 0 {
        if subpixel_order as libc::c_uint
            == CAIRO_SUBPIXEL_ORDER_DEFAULT as libc::c_int as libc::c_uint
        {
            antialias = CAIRO_ANTIALIAS_GRAY;
        } else {
            antialias = CAIRO_ANTIALIAS_SUBPIXEL;
        }
    } else {
        antialias = CAIRO_ANTIALIAS_NONE;
    }
    cairo_font_options_set_hint_style(&mut (*screen).font_options, hint_style);
    cairo_font_options_set_antialias(&mut (*screen).font_options, antialias);
    cairo_font_options_set_subpixel_order(&mut (*screen).font_options, subpixel_order);
    _cairo_font_options_set_lcd_filter(&mut (*screen).font_options, lcd_filter);
    cairo_font_options_set_hint_metrics(
        &mut (*screen).font_options,
        CAIRO_HINT_METRICS_ON,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_finish(mut screen: *mut cairo_xcb_screen_t) {
    let mut i: libc::c_int = 0;
    pthread_mutex_lock(&mut (*(*screen).connection).screens_mutex);
    cairo_list_del(&mut (*screen).link);
    pthread_mutex_unlock(&mut (*(*screen).connection).screens_mutex);
    while cairo_list_is_empty(&mut (*screen).surfaces) == 0 {
        let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        surface = &mut (*({
            let mut mptr__: *const cairo_list_t = (*screen).surfaces.next;
            (mptr__ as *mut libc::c_char).offset(-(1040 as libc::c_ulong as isize))
                as *mut cairo_xcb_surface_t
        }))
            .base;
        cairo_surface_finish(surface);
    }
    while cairo_list_is_empty(&mut (*screen).pictures) == 0 {
        let mut surface_0: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        surface_0 = &mut (*({
            let mut mptr__: *const cairo_list_t = (*screen).pictures.next;
            (mptr__ as *mut libc::c_char).offset(-(432 as libc::c_ulong as isize))
                as *mut cairo_xcb_picture_t
        }))
            .base;
        cairo_surface_finish(surface_0);
    }
    i = 0 as libc::c_int;
    while i < (*screen).solid_cache_size {
        cairo_surface_destroy((*screen).solid_cache[i as usize].picture);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[*mut cairo_surface_t; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut cairo_surface_t>() as libc::c_ulong)
            as libc::c_int
    {
        cairo_surface_destroy((*screen).stock_colors[i as usize]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[xcb_gcontext_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_gcontext_t>() as libc::c_ulong)
            as libc::c_int
    {
        if (*screen).gc_depths[i as usize] as libc::c_int != 0 as libc::c_int {
            xcb_free_gc(
                (*(*screen).connection).xcb_connection,
                (*screen).gc[i as usize],
            );
        }
        i += 1;
    }
    _cairo_cache_fini(&mut (*screen).linear_pattern_cache);
    _cairo_cache_fini(&mut (*screen).radial_pattern_cache);
    _cairo_freelist_fini(&mut (*screen).pattern_cache_entry_freelist);
    free(screen as *mut libc::c_void);
}
unsafe extern "C" fn _linear_pattern_cache_entry_equal(
    mut A: *const libc::c_void,
    mut B: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const pattern_cache_entry = A as *const pattern_cache_entry;
    let mut b: *const pattern_cache_entry = B as *const pattern_cache_entry;
    return _cairo_linear_pattern_equal(
        &(*a).pattern.gradient.linear,
        &(*b).pattern.gradient.linear,
    );
}
unsafe extern "C" fn _radial_pattern_cache_entry_equal(
    mut A: *const libc::c_void,
    mut B: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const pattern_cache_entry = A as *const pattern_cache_entry;
    let mut b: *const pattern_cache_entry = B as *const pattern_cache_entry;
    return _cairo_radial_pattern_equal(
        &(*a).pattern.gradient.radial,
        &(*b).pattern.gradient.radial,
    );
}
unsafe extern "C" fn _pattern_cache_entry_destroy(mut closure: *mut libc::c_void) {
    let mut entry: *mut pattern_cache_entry = closure as *mut pattern_cache_entry;
    _cairo_pattern_fini(&mut (*entry).pattern.base);
    cairo_surface_destroy((*entry).picture);
    _cairo_freelist_free(
        &mut (*(*entry).screen).pattern_cache_entry_freelist,
        entry as *mut libc::c_void,
    );
}
unsafe extern "C" fn _get_screen_index(
    mut xcb_connection: *mut cairo_xcb_connection_t,
    mut xcb_screen: *mut xcb_screen_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut iter: xcb_screen_iterator_t = xcb_setup_roots_iterator(
        (*xcb_connection).root,
    );
    while iter.rem != 0 {
        if (*iter.data).root == (*xcb_screen).root {
            return idx;
        }
        xcb_screen_next(&mut iter);
        idx += 1;
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-screen.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"int _get_screen_index(cairo_xcb_connection_t *, xcb_screen_t *)\0"))
                .as_ptr(),
        );
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_get(
    mut xcb_connection: *mut xcb_connection_t,
    mut xcb_screen: *mut xcb_screen_t,
) -> *mut cairo_xcb_screen_t {
    let mut current_block: u64;
    let mut connection: *mut cairo_xcb_connection_t = 0 as *mut cairo_xcb_connection_t;
    let mut screen: *mut cairo_xcb_screen_t = 0 as *mut cairo_xcb_screen_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut screen_idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    connection = _cairo_xcb_connection_get(xcb_connection);
    if connection.is_null() {
        return 0 as *mut cairo_xcb_screen_t;
    }
    pthread_mutex_lock(&mut (*connection).screens_mutex);
    screen = ({
        let mut mptr__: *const cairo_list_t = (*connection).screens.next;
        (mptr__ as *mut libc::c_char).offset(-(952 as libc::c_ulong as isize))
            as *mut cairo_xcb_screen_t
    });
    loop {
        if !(&mut (*screen).link as *mut cairo_list_t
            != &mut (*connection).screens as *mut cairo_list_t)
        {
            current_block = 1841672684692190573;
            break;
        }
        if (*screen).xcb_screen == xcb_screen {
            if &mut (*screen).link as *mut cairo_list_t != (*connection).screens.next {
                cairo_list_move(&mut (*screen).link, &mut (*connection).screens);
            }
            current_block = 5838563839104241728;
            break;
        } else {
            screen = ({
                let mut mptr__: *const cairo_list_t = (*screen).link.next;
                (mptr__ as *mut libc::c_char).offset(-(952 as libc::c_ulong as isize))
                    as *mut cairo_xcb_screen_t
            });
        }
    }
    match current_block {
        1841672684692190573 => {
            screen = (if ::std::mem::size_of::<cairo_xcb_screen_t>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_xcb_screen_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_xcb_screen_t;
            if !screen.is_null() {
                screen_idx = _get_screen_index(connection, xcb_screen);
                let ref mut fresh8 = (*screen).connection;
                *fresh8 = connection;
                let ref mut fresh9 = (*screen).xcb_screen;
                *fresh9 = xcb_screen;
                (*screen).has_font_options = 0 as libc::c_int;
                (*screen)
                    .subpixel_order = *((*connection).subpixel_orders)
                    .offset(screen_idx as isize);
                _cairo_freelist_init(
                    &mut (*screen).pattern_cache_entry_freelist,
                    ::std::mem::size_of::<pattern_cache_entry>() as libc::c_ulong
                        as libc::c_uint,
                );
                cairo_list_init(&mut (*screen).link);
                cairo_list_init(&mut (*screen).surfaces);
                cairo_list_init(&mut (*screen).pictures);
                memset(
                    ((*screen).gc_depths).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
                );
                memset(
                    ((*screen).gc).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[xcb_gcontext_t; 4]>() as libc::c_ulong,
                );
                (*screen).solid_cache_size = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i
                    < (::std::mem::size_of::<[*mut cairo_surface_t; 3]>()
                        as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut cairo_surface_t>()
                                as libc::c_ulong,
                        ) as libc::c_int
                {
                    let ref mut fresh10 = (*screen).stock_colors[i as usize];
                    *fresh10 = 0 as *mut cairo_surface_t;
                    i += 1;
                }
                status = _cairo_cache_init(
                    &mut (*screen).linear_pattern_cache,
                    Some(
                        _linear_pattern_cache_entry_equal
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> cairo_bool_t,
                    ),
                    None,
                    Some(
                        _pattern_cache_entry_destroy
                            as unsafe extern "C" fn(*mut libc::c_void) -> (),
                    ),
                    16 as libc::c_int as libc::c_ulong,
                );
                if status as u64 != 0 {
                    current_block = 3530157135506094377;
                } else {
                    status = _cairo_cache_init(
                        &mut (*screen).radial_pattern_cache,
                        Some(
                            _radial_pattern_cache_entry_equal
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> cairo_bool_t,
                        ),
                        None,
                        Some(
                            _pattern_cache_entry_destroy
                                as unsafe extern "C" fn(*mut libc::c_void) -> (),
                        ),
                        4 as libc::c_int as libc::c_ulong,
                    );
                    if status as u64 != 0 {
                        _cairo_cache_fini(&mut (*screen).linear_pattern_cache);
                        current_block = 3530157135506094377;
                    } else {
                        cairo_list_add(&mut (*screen).link, &mut (*connection).screens);
                        current_block = 5838563839104241728;
                    }
                }
                match current_block {
                    5838563839104241728 => {}
                    _ => {
                        pthread_mutex_unlock(&mut (*connection).screens_mutex);
                        free(screen as *mut libc::c_void);
                        return 0 as *mut cairo_xcb_screen_t;
                    }
                }
            }
        }
        _ => {}
    }
    pthread_mutex_unlock(&mut (*connection).screens_mutex);
    return screen;
}
unsafe extern "C" fn _create_gc(
    mut screen: *mut cairo_xcb_screen_t,
    mut drawable: xcb_drawable_t,
) -> xcb_gcontext_t {
    let mut values: [uint32_t; 1] = [0 as libc::c_int as uint32_t];
    return _cairo_xcb_connection_create_gc(
        (*screen).connection,
        drawable,
        XCB_GC_GRAPHICS_EXPOSURES as libc::c_int as uint32_t,
        values.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_get_gc(
    mut screen: *mut cairo_xcb_screen_t,
    mut drawable: xcb_drawable_t,
    mut depth: libc::c_int,
) -> xcb_gcontext_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[xcb_gcontext_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_gcontext_t>() as libc::c_ulong)
            as libc::c_int
    {
        if (*screen).gc_depths[i as usize] as libc::c_int == depth {
            (*screen).gc_depths[i as usize] = 0 as libc::c_int as uint8_t;
            return (*screen).gc[i as usize];
        }
        i += 1;
    }
    return _create_gc(screen, drawable);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_put_gc(
    mut screen: *mut cairo_xcb_screen_t,
    mut depth: libc::c_int,
    mut gc: xcb_gcontext_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[xcb_gcontext_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_gcontext_t>() as libc::c_ulong)
            as libc::c_int
    {
        if (*screen).gc_depths[i as usize] as libc::c_int == 0 as libc::c_int {
            break;
        }
        i += 1;
    }
    if i
        == (::std::mem::size_of::<[xcb_gcontext_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_gcontext_t>() as libc::c_ulong)
            as libc::c_int
    {
        i = rand()
            % (::std::mem::size_of::<[xcb_gcontext_t; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<xcb_gcontext_t>() as libc::c_ulong)
                as libc::c_int;
        xcb_free_gc((*(*screen).connection).xcb_connection, (*screen).gc[i as usize]);
    }
    (*screen).gc[i as usize] = gc;
    (*screen).gc_depths[i as usize] = depth as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_store_linear_picture(
    mut screen: *mut cairo_xcb_screen_t,
    mut linear: *const cairo_linear_pattern_t,
    mut picture: *mut cairo_surface_t,
) -> cairo_status_t {
    let mut entry: *mut pattern_cache_entry = 0 as *mut pattern_cache_entry;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    entry = _cairo_freelist_alloc(&mut (*screen).pattern_cache_entry_freelist)
        as *mut pattern_cache_entry;
    if entry.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*entry)
        .key
        .hash = _cairo_linear_pattern_hash(5381 as libc::c_int as uintptr_t, linear);
    (*entry).key.size = 1 as libc::c_int as libc::c_ulong;
    status = _cairo_pattern_init_copy(&mut (*entry).pattern.base, &(*linear).base.base);
    if status as u64 != 0 {
        _cairo_freelist_free(
            &mut (*screen).pattern_cache_entry_freelist,
            entry as *mut libc::c_void,
        );
        return status;
    }
    let ref mut fresh11 = (*entry).picture;
    *fresh11 = cairo_surface_reference(picture);
    let ref mut fresh12 = (*entry).screen;
    *fresh12 = screen;
    status = _cairo_cache_insert(&mut (*screen).linear_pattern_cache, &mut (*entry).key);
    if status as u64 != 0 {
        cairo_surface_destroy(picture);
        _cairo_freelist_free(
            &mut (*screen).pattern_cache_entry_freelist,
            entry as *mut libc::c_void,
        );
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_lookup_linear_picture(
    mut screen: *mut cairo_xcb_screen_t,
    mut linear: *const cairo_linear_pattern_t,
) -> *mut cairo_surface_t {
    let mut picture: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut tmpl: pattern_cache_entry = pattern_cache_entry {
        key: cairo_cache_entry_t {
            hash: 0,
            size: 0,
        },
        screen: 0 as *mut cairo_xcb_screen_t,
        pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_FAST,
                extend: CAIRO_EXTEND_NONE,
                has_component_alpha: 0,
                is_userfont_foreground: 0,
                matrix: cairo_matrix_t {
                    xx: 0.,
                    yx: 0.,
                    xy: 0.,
                    yy: 0.,
                    x0: 0.,
                    y0: 0.,
                },
                opacity: 0.,
            },
        },
        picture: 0 as *mut cairo_surface_t,
    };
    let mut entry: *mut pattern_cache_entry = 0 as *mut pattern_cache_entry;
    tmpl.key.hash = _cairo_linear_pattern_hash(5381 as libc::c_int as uintptr_t, linear);
    _cairo_pattern_init_static_copy(&mut tmpl.pattern.base, &(*linear).base.base);
    entry = _cairo_cache_lookup(&mut (*screen).linear_pattern_cache, &mut tmpl.key)
        as *mut pattern_cache_entry;
    if !entry.is_null() {
        picture = cairo_surface_reference((*entry).picture);
    }
    return picture;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_store_radial_picture(
    mut screen: *mut cairo_xcb_screen_t,
    mut radial: *const cairo_radial_pattern_t,
    mut picture: *mut cairo_surface_t,
) -> cairo_status_t {
    let mut entry: *mut pattern_cache_entry = 0 as *mut pattern_cache_entry;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    entry = _cairo_freelist_alloc(&mut (*screen).pattern_cache_entry_freelist)
        as *mut pattern_cache_entry;
    if entry.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*entry)
        .key
        .hash = _cairo_radial_pattern_hash(5381 as libc::c_int as uintptr_t, radial);
    (*entry).key.size = 1 as libc::c_int as libc::c_ulong;
    status = _cairo_pattern_init_copy(&mut (*entry).pattern.base, &(*radial).base.base);
    if status as u64 != 0 {
        _cairo_freelist_free(
            &mut (*screen).pattern_cache_entry_freelist,
            entry as *mut libc::c_void,
        );
        return status;
    }
    let ref mut fresh13 = (*entry).picture;
    *fresh13 = cairo_surface_reference(picture);
    let ref mut fresh14 = (*entry).screen;
    *fresh14 = screen;
    status = _cairo_cache_insert(&mut (*screen).radial_pattern_cache, &mut (*entry).key);
    if status as u64 != 0 {
        cairo_surface_destroy(picture);
        _cairo_freelist_free(
            &mut (*screen).pattern_cache_entry_freelist,
            entry as *mut libc::c_void,
        );
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_lookup_radial_picture(
    mut screen: *mut cairo_xcb_screen_t,
    mut radial: *const cairo_radial_pattern_t,
) -> *mut cairo_surface_t {
    let mut picture: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut tmpl: pattern_cache_entry = pattern_cache_entry {
        key: cairo_cache_entry_t {
            hash: 0,
            size: 0,
        },
        screen: 0 as *mut cairo_xcb_screen_t,
        pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_FAST,
                extend: CAIRO_EXTEND_NONE,
                has_component_alpha: 0,
                is_userfont_foreground: 0,
                matrix: cairo_matrix_t {
                    xx: 0.,
                    yx: 0.,
                    xy: 0.,
                    yy: 0.,
                    x0: 0.,
                    y0: 0.,
                },
                opacity: 0.,
            },
        },
        picture: 0 as *mut cairo_surface_t,
    };
    let mut entry: *mut pattern_cache_entry = 0 as *mut pattern_cache_entry;
    tmpl.key.hash = _cairo_radial_pattern_hash(5381 as libc::c_int as uintptr_t, radial);
    _cairo_pattern_init_static_copy(&mut tmpl.pattern.base, &(*radial).base.base);
    entry = _cairo_cache_lookup(&mut (*screen).radial_pattern_cache, &mut tmpl.key)
        as *mut pattern_cache_entry;
    if !entry.is_null() {
        picture = cairo_surface_reference((*entry).picture);
    }
    return picture;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_screen_get_font_options(
    mut screen: *mut cairo_xcb_screen_t,
) -> *mut cairo_font_options_t {
    if (*screen).has_font_options == 0 {
        _cairo_font_options_init_default(&mut (*screen).font_options);
        _cairo_font_options_set_round_glyph_positions(
            &mut (*screen).font_options,
            CAIRO_ROUND_GLYPH_POS_ON,
        );
        if 0 as libc::c_int != 0
            && _cairo_xcb_connection_acquire((*screen).connection) as u64 == 0
        {
            _cairo_xcb_init_screen_font_options(screen);
            _cairo_xcb_connection_release((*screen).connection);
        }
        (*screen).has_font_options = 1 as libc::c_int;
    }
    return &mut (*screen).font_options;
}
