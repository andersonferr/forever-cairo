use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    pub type _cairo_pattern;
    pub type cairo_compositor;
    pub type _XErrorThreadInfo;
    pub type _X11XCBPrivate;
    pub type _XtransConnInfo;
    pub type _XkbInfoRec;
    pub type _XIMFilter;
    pub type _XContextDB;
    pub type _XDisplayAtoms;
    pub type _XKeytrans;
    pub type _XLockInfo;
    pub type _XrmHashBucketRec;
    pub type _XPrivate;
    pub type _cairo_xlib_shm_display;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut _cairo_xlib_display_mutex: cairo_mutex_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_device_finish(device: *mut cairo_device_t);
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_xlib_display_fini_shm(display: *mut cairo_xlib_display_t);
    fn _cairo_xlib_font_close(font: *mut cairo_xlib_font_t);
    fn _cairo_xlib_screen_destroy(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
    );
    fn _cairo_xlib_display_init_shm(display: *mut cairo_xlib_display_t);
    fn XAddExtension(_: *mut Display) -> *mut XExtCodes;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn _cairo_device_init(
        device: *mut cairo_device_t,
        backend: *const cairo_device_backend_t,
    );
    fn _cairo_device_create_in_error(status: cairo_status_t) -> *mut cairo_device_t;
    fn _cairo_device_set_error(
        device: *mut cairo_device_t,
        error: cairo_status_t,
    ) -> cairo_status_t;
    fn _cairo_xlib_traps_compositor_get() -> *const cairo_compositor_t;
    fn XRenderQueryVersion(
        dpy: *mut Display,
        major_versionp: *mut libc::c_int,
        minor_versionp: *mut libc::c_int,
    ) -> libc::c_int;
    fn XRenderFindFormat(
        dpy: *mut Display,
        mask: libc::c_ulong,
        templ: *const XRenderPictFormat,
        count: libc::c_int,
    ) -> *mut XRenderPictFormat;
    fn XRenderFindStandardFormat(
        dpy: *mut Display,
        format: libc::c_int,
    ) -> *mut XRenderPictFormat;
    fn _cairo_xlib_mask_compositor_get() -> *const cairo_compositor_t;
    fn _cairo_xlib_core_compositor_get() -> *const cairo_compositor_t;
    fn XESetCloseDisplay(
        _: *mut Display,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut Display, *mut XExtCodes) -> libc::c_int>,
    ) -> Option::<unsafe extern "C" fn(*mut Display, *mut XExtCodes) -> libc::c_int>;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
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
pub type uint8_t = __uint8_t;
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
pub type cairo_compositor_t = cairo_compositor;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExtCodes {
    pub extension: libc::c_int,
    pub major_opcode: libc::c_int,
    pub first_event: libc::c_int,
    pub first_error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGC {
    pub ext_data: *mut XExtData,
    pub gid: GContext,
    pub rects: libc::c_int,
    pub dashes: libc::c_int,
    pub dirty: libc::c_ulong,
    pub values: XGCValues,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XDisplay {
    pub ext_data: *mut XExtData,
    pub free_funcs: *mut _XFreeFuncs,
    pub fd: libc::c_int,
    pub conn_checker: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub resource_base: XID,
    pub resource_mask: XID,
    pub resource_id: XID,
    pub resource_shift: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub vnumber: libc::c_int,
    pub release: libc::c_int,
    pub head: *mut _XSQEvent,
    pub tail: *mut _XSQEvent,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub last_req: *mut libc::c_char,
    pub buffer: *mut libc::c_char,
    pub bufptr: *mut libc::c_char,
    pub bufmax: *mut libc::c_char,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub synchandler: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub keysyms: *mut KeySym,
    pub modifiermap: *mut XModifierKeymap,
    pub keysyms_per_keycode: libc::c_int,
    pub xdefaults: *mut libc::c_char,
    pub scratch_buffer: *mut libc::c_char,
    pub scratch_length: libc::c_ulong,
    pub ext_number: libc::c_int,
    pub ext_procs: *mut _XExten,
    pub event_vec: [Option::<
        unsafe extern "C" fn(*mut Display, *mut XEvent, *mut xEvent) -> libc::c_int,
    >; 128],
    pub wire_vec: [Option::<
        unsafe extern "C" fn(*mut Display, *mut XEvent, *mut xEvent) -> libc::c_int,
    >; 128],
    pub lock_meaning: KeySym,
    pub lock: *mut _XLockInfo,
    pub async_handlers: *mut _XInternalAsync,
    pub bigreq_size: libc::c_ulong,
    pub lock_fns: *mut _XLockPtrs,
    pub idlist_alloc: Option::<
        unsafe extern "C" fn(*mut Display, *mut XID, libc::c_int) -> (),
    >,
    pub key_bindings: *mut _XKeytrans,
    pub cursor_font: Font,
    pub atoms: *mut _XDisplayAtoms,
    pub mode_switch: libc::c_uint,
    pub num_lock: libc::c_uint,
    pub context_db: *mut _XContextDB,
    pub error_vec: *mut Option::<
        unsafe extern "C" fn(*mut Display, *mut XErrorEvent, *mut xError) -> libc::c_int,
    >,
    pub cms: C2RustUnnamed_31,
    pub im_filters: *mut _XIMFilter,
    pub qfree: *mut _XSQEvent,
    pub next_event_serial_num: libc::c_ulong,
    pub flushes: *mut _XExten,
    pub im_fd_info: *mut _XConnectionInfo,
    pub im_fd_length: libc::c_int,
    pub conn_watchers: *mut _XConnWatchInfo,
    pub watcher_count: libc::c_int,
    pub filedes: XPointer,
    pub savedsynchandler: Option::<unsafe extern "C" fn(*mut Display) -> libc::c_int>,
    pub resource_max: XID,
    pub xcmisc_opcode: libc::c_int,
    pub xkb_info: *mut _XkbInfoRec,
    pub trans_conn: *mut _XtransConnInfo,
    pub xcb: *mut _X11XCBPrivate,
    pub next_cookie: libc::c_uint,
    pub generic_event_vec: [Option::<
        unsafe extern "C" fn(
            *mut Display,
            *mut XGenericEventCookie,
            *mut xEvent,
        ) -> libc::c_int,
    >; 128],
    pub generic_event_copy_vec: [Option::<
        unsafe extern "C" fn(
            *mut Display,
            *mut XGenericEventCookie,
            *mut XGenericEventCookie,
        ) -> libc::c_int,
    >; 128],
    pub cookiejar: *mut libc::c_void,
    pub error_threads: *mut _XErrorThreadInfo,
    pub exit_handler: XIOErrorExitHandler,
    pub exit_handler_data: *mut libc::c_void,
}
pub type XIOErrorExitHandler = Option::<
    unsafe extern "C" fn(*mut Display, *mut libc::c_void) -> (),
>;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
pub type xEvent = _xEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xEvent {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: C2RustUnnamed_29,
    pub keyButtonPointer: C2RustUnnamed_28,
    pub enterLeave: C2RustUnnamed_27,
    pub focus: C2RustUnnamed_26,
    pub expose: C2RustUnnamed_25,
    pub graphicsExposure: C2RustUnnamed_24,
    pub noExposure: C2RustUnnamed_23,
    pub visibility: C2RustUnnamed_22,
    pub createNotify: C2RustUnnamed_21,
    pub destroyNotify: C2RustUnnamed_20,
    pub unmapNotify: C2RustUnnamed_19,
    pub mapNotify: C2RustUnnamed_18,
    pub mapRequest: C2RustUnnamed_17,
    pub reparent: C2RustUnnamed_16,
    pub configureNotify: C2RustUnnamed_15,
    pub configureRequest: C2RustUnnamed_14,
    pub gravity: C2RustUnnamed_13,
    pub resizeRequest: C2RustUnnamed_12,
    pub circulate: C2RustUnnamed_11,
    pub property: C2RustUnnamed_10,
    pub selectionClear: C2RustUnnamed_9,
    pub selectionRequest: C2RustUnnamed_8,
    pub selectionNotify: C2RustUnnamed_7,
    pub colormap: C2RustUnnamed_6,
    pub mappingNotify: C2RustUnnamed_5,
    pub clientMessage: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub l: C2RustUnnamed_4,
    pub s: C2RustUnnamed_3,
    pub b: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_0: CARD32,
    pub bytes: [INT8; 20],
}
pub type INT8 = libc::c_schar;
pub type CARD32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub type_0: CARD32,
    pub shorts0: INT16,
    pub shorts1: INT16,
    pub shorts2: INT16,
    pub shorts3: INT16,
    pub shorts4: INT16,
    pub shorts5: INT16,
    pub shorts6: INT16,
    pub shorts7: INT16,
    pub shorts8: INT16,
    pub shorts9: INT16,
}
pub type INT16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub type_0: CARD32,
    pub longs0: INT32,
    pub longs1: INT32,
    pub longs2: INT32,
    pub longs3: INT32,
    pub longs4: INT32,
}
pub type INT32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub pad00: CARD32,
    pub request: CARD8,
    pub firstKeyCode: CARD8,
    pub count: CARD8,
    pub pad1: BYTE,
}
pub type BYTE = CARD8;
pub type CARD8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub colormap: CARD32,
    pub new: BOOL,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
}
pub type BOOL = CARD8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub owner: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
    pub time: CARD32,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: CARD16,
}
pub type CARD16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub place: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub width: CARD16,
    pub height: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub sibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub valueMask: CARD16,
    pub pad1: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub aboveSibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub fromConfigure: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub state: CARD8,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub minorEvent: CARD16,
    pub majorEvent: BYTE,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub minorEvent: CARD16,
    pub count: CARD16,
    pub majorEvent: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub count: CARD16,
    pub pad2: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub mode: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub mode: BYTE,
    pub flags: BYTE,
}
pub type KeyButMask = CARD16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub sameScreen: BOOL,
    pub pad1: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub type_0: BYTE,
    pub detail: BYTE,
    pub sequenceNumber: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnWatchInfo {
    pub fn_0: XConnectionWatchProc,
    pub client_data: XPointer,
    pub next: *mut _XConnWatchInfo,
}
pub type XConnectionWatchProc = Option::<
    unsafe extern "C" fn(
        *mut Display,
        XPointer,
        libc::c_int,
        libc::c_int,
        *mut XPointer,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnectionInfo {
    pub fd: libc::c_int,
    pub read_callback: _XInternalConnectionProc,
    pub call_data: XPointer,
    pub watch_data: *mut XPointer,
    pub next: *mut _XConnectionInfo,
}
pub type _XInternalConnectionProc = Option::<
    unsafe extern "C" fn(*mut Display, libc::c_int, XPointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExten {
    pub next: *mut _XExten,
    pub codes: XExtCodes,
    pub create_GC: CreateGCType,
    pub copy_GC: CopyGCType,
    pub flush_GC: FlushGCType,
    pub free_GC: FreeGCType,
    pub create_Font: CreateFontType,
    pub free_Font: FreeFontType,
    pub close_display: CloseDisplayType,
    pub error: ErrorType,
    pub error_string: ErrorStringType,
    pub name: *mut libc::c_char,
    pub error_values: PrintErrorType,
    pub before_flush: BeforeFlushType,
    pub next_flush: *mut _XExten,
}
pub type BeforeFlushType = Option::<
    unsafe extern "C" fn(
        *mut Display,
        *mut XExtCodes,
        *const libc::c_char,
        libc::c_long,
    ) -> (),
>;
pub type PrintErrorType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
pub type ErrorStringType = Option::<
    unsafe extern "C" fn(
        *mut Display,
        libc::c_int,
        *mut XExtCodes,
        *mut libc::c_char,
        libc::c_int,
    ) -> *mut libc::c_char,
>;
pub type ErrorType = Option::<
    unsafe extern "C" fn(
        *mut Display,
        *mut xError,
        *mut XExtCodes,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xError {
    pub type_0: BYTE,
    pub errorCode: BYTE,
    pub sequenceNumber: CARD16,
    pub resourceID: CARD32,
    pub minorCode: CARD16,
    pub majorCode: CARD8,
    pub pad1: BYTE,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
pub type CloseDisplayType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XExtCodes) -> libc::c_int,
>;
pub type FreeFontType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XFontStruct, *mut XExtCodes) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: libc::c_uint,
    pub min_char_or_byte2: libc::c_uint,
    pub max_char_or_byte2: libc::c_uint,
    pub min_byte1: libc::c_uint,
    pub max_byte1: libc::c_uint,
    pub all_chars_exist: libc::c_int,
    pub default_char: libc::c_uint,
    pub n_properties: libc::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: libc::c_short,
    pub rbearing: libc::c_short,
    pub width: libc::c_short,
    pub ascent: libc::c_short,
    pub descent: libc::c_short,
    pub attributes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: libc::c_ulong,
}
pub type CreateFontType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XFontStruct, *mut XExtCodes) -> libc::c_int,
>;
pub type FreeGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
pub type FlushGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
pub type CopyGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
pub type CreateGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XSQEvent {
    pub next: *mut _XSQEvent,
    pub event: XEvent,
    pub qserial_num: libc::c_ulong,
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub defaultCCCs: XPointer,
    pub clientCmaps: XPointer,
    pub perVisualIntensityMaps: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XLockPtrs {
    pub lock_display: Option::<unsafe extern "C" fn(*mut Display) -> ()>,
    pub unlock_display: Option::<unsafe extern "C" fn(*mut Display) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XInternalAsync {
    pub next: *mut _XInternalAsync,
    pub handler: Option::<
        unsafe extern "C" fn(
            *mut Display,
            *mut xReply,
            *mut libc::c_char,
            libc::c_int,
            XPointer,
        ) -> libc::c_int,
    >,
    pub data: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union xReply {
    pub generic: xGenericReply,
    pub geom: xGetGeometryReply,
    pub tree: xQueryTreeReply,
    pub atom: xInternAtomReply,
    pub atomName: xGetAtomNameReply,
    pub property: xGetPropertyReply,
    pub listProperties: xListPropertiesReply,
    pub selection: xGetSelectionOwnerReply,
    pub grabPointer: xGrabPointerReply,
    pub grabKeyboard: xGrabKeyboardReply,
    pub pointer: xQueryPointerReply,
    pub motionEvents: xGetMotionEventsReply,
    pub coords: xTranslateCoordsReply,
    pub inputFocus: xGetInputFocusReply,
    pub textExtents: xQueryTextExtentsReply,
    pub fonts: xListFontsReply,
    pub fontPath: xGetFontPathReply,
    pub image: xGetImageReply,
    pub colormaps: xListInstalledColormapsReply,
    pub allocColor: xAllocColorReply,
    pub allocNamedColor: xAllocNamedColorReply,
    pub colorCells: xAllocColorCellsReply,
    pub colorPlanes: xAllocColorPlanesReply,
    pub colors: xQueryColorsReply,
    pub lookupColor: xLookupColorReply,
    pub bestSize: xQueryBestSizeReply,
    pub extension: xQueryExtensionReply,
    pub extensions: xListExtensionsReply,
    pub setModifierMapping: xSetModifierMappingReply,
    pub getModifierMapping: xGetModifierMappingReply,
    pub setPointerMapping: xSetPointerMappingReply,
    pub getKeyboardMapping: xGetKeyboardMappingReply,
    pub getPointerMapping: xGetPointerMappingReply,
    pub pointerControl: xGetPointerControlReply,
    pub screenSaver: xGetScreenSaverReply,
    pub hosts: xListHostsReply,
    pub error: xError,
    pub event: xEvent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListHostsReply {
    pub type_0: BYTE,
    pub enabled: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nHosts: CARD16,
    pub pad1: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetScreenSaverReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub timeout: CARD16,
    pub interval: CARD16,
    pub preferBlanking: BOOL,
    pub allowExposures: BOOL,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerControlReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub accelNumerator: CARD16,
    pub accelDenominator: CARD16,
    pub threshold: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerMappingReply {
    pub type_0: BYTE,
    pub nElts: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetKeyboardMappingReply {
    pub type_0: BYTE,
    pub keySymsPerKeyCode: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
pub type xSetMappingReply = xSetPointerMappingReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xSetPointerMappingReply {
    pub type_0: BYTE,
    pub success: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetModifierMappingReply {
    pub type_0: BYTE,
    pub numKeyPerModifier: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
pub type xSetModifierMappingReply = xSetMappingReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListExtensionsReply {
    pub type_0: BYTE,
    pub nExtensions: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryExtensionReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub present: BOOL,
    pub major_opcode: CARD8,
    pub first_event: CARD8,
    pub first_error: CARD8,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryBestSizeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub width: CARD16,
    pub height: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xLookupColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryColorsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColors: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorPlanesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub pad2: CARD16,
    pub redMask: CARD32,
    pub greenMask: CARD32,
    pub blueMask: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorCellsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub nMasks: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocNamedColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pixel: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub red: CARD16,
    pub green: CARD16,
    pub blue: CARD16,
    pub pad2: CARD16,
    pub pixel: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListInstalledColormapsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColormaps: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetImageReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub visual: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetFontPathReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPaths: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListFontsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nFonts: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTextExtentsReply {
    pub type_0: BYTE,
    pub drawDirection: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub fontAscent: INT16,
    pub fontDescent: INT16,
    pub overallAscent: INT16,
    pub overallDescent: INT16,
    pub overallWidth: INT32,
    pub overallLeft: INT32,
    pub overallRight: INT32,
    pub pad: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetInputFocusReply {
    pub type_0: BYTE,
    pub revertTo: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub focus: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xTranslateCoordsReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub child: CARD32,
    pub dstX: INT16,
    pub dstY: INT16,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetMotionEventsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nEvents: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryPointerReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub winX: INT16,
    pub winY: INT16,
    pub mask: CARD16,
    pub pad1: CARD16,
    pub pad: CARD32,
}
pub type xGrabPointerReply = xGrabKeyboardReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGrabKeyboardReply {
    pub type_0: BYTE,
    pub status: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetSelectionOwnerReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub owner: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListPropertiesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nProperties: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPropertyReply {
    pub type_0: BYTE,
    pub format: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub propertyType: CARD32,
    pub bytesAfter: CARD32,
    pub nItems: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetAtomNameReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nameLength: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xInternAtomReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub atom: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTreeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub parent: CARD32,
    pub nChildren: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetGeometryReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub pad1: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGenericReply {
    pub type_0: BYTE,
    pub data1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub data00: CARD32,
    pub data01: CARD32,
    pub data02: CARD32,
    pub data03: CARD32,
    pub data04: CARD32,
    pub data05: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: libc::c_int,
    pub modifiermap: *mut KeyCode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XFreeFuncs {
    pub atoms: FreeFuncType,
    pub modifiermap: FreeModmapType,
    pub key_bindings: FreeFuncType,
    pub context_db: FreeFuncType,
    pub defaultCCCs: FreeFuncType,
    pub clientCmaps: FreeFuncType,
    pub intensityMaps: FreeFuncType,
    pub im_filters: FreeFuncType,
    pub xkb: FreeFuncType,
}
pub type FreeFuncType = Option::<unsafe extern "C" fn(*mut Display) -> ()>;
pub type FreeModmapType = Option::<
    unsafe extern "C" fn(*mut XModifierKeymap) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_32;
pub type XErrorHandler = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
>;
pub type GlyphSet = XID;
pub type PictFormat = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: libc::c_short,
    pub redMask: libc::c_short,
    pub green: libc::c_short,
    pub greenMask: libc::c_short,
    pub blue: libc::c_short,
    pub blueMask: libc::c_short,
    pub alpha: libc::c_short,
    pub alphaMask: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_0: libc::c_int,
    pub depth: libc::c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
pub type cairo_xlib_display_t = _cairo_xlib_display;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_xlib_display {
    pub base: cairo_device_t,
    pub next: *mut cairo_xlib_display_t,
    pub display: *mut Display,
    pub screens: cairo_list_t,
    pub fonts: cairo_list_t,
    pub shm: *mut cairo_xlib_shm_display_t,
    pub compositor: *const cairo_compositor_t,
    pub render_major: libc::c_int,
    pub render_minor: libc::c_int,
    pub cached_xrender_formats: [*mut XRenderPictFormat; 6],
    pub force_precision: libc::c_int,
    pub white: *mut cairo_surface_t,
    pub alpha: [*mut cairo_surface_t; 256],
    pub solid: [*mut cairo_surface_t; 32],
    pub solid_cache: [uint32_t; 32],
    pub last_solid_cache: [C2RustUnnamed_33; 2],
    #[bitfield(name = "buggy_gradients", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "buggy_pad_reflect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "buggy_repeat", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "closed", ty = "libc::c_uint", bits = "3..=3")]
    pub buggy_gradients_buggy_pad_reflect_buggy_repeat_closed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub color: uint32_t,
    pub index: libc::c_int,
}
pub type cairo_xlib_shm_display_t = _cairo_xlib_shm_display;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_screen {
    pub link: cairo_list_t,
    pub device: *mut cairo_device_t,
    pub screen: *mut Screen,
    pub surfaces: cairo_list_t,
    pub has_font_options: cairo_bool_t,
    pub font_options: cairo_font_options_t,
    pub gc: [GC; 4],
    pub gc_depths: [uint8_t; 4],
    pub visuals: cairo_list_t,
}
pub type cairo_xlib_screen_t = _cairo_xlib_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_font_glyphset {
    pub glyphset: GlyphSet,
    pub format: cairo_format_t,
    pub xrender_format: *mut XRenderPictFormat,
    pub to_free: _cairo_xlib_font_glyphset_free_glyphs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_font_glyphset_free_glyphs {
    pub count: libc::c_int,
    pub indices: [libc::c_ulong; 128],
}
pub type cairo_xlib_font_glyphset_t = _cairo_xlib_font_glyphset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_font {
    pub base: cairo_scaled_font_private_t,
    pub font: *mut cairo_scaled_font_t,
    pub device: *mut cairo_device_t,
    pub link: cairo_list_t,
    pub glyphset: [cairo_xlib_font_glyphset_t; 3],
}
pub type cairo_xlib_font_t = _cairo_xlib_font;
pub type cairo_xlib_error_func_t = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn _cairo_xlib_vendor_is_xorg(mut dpy: *mut Display) -> cairo_bool_t {
    let vendor: *const libc::c_char = (*(dpy as _XPrivDisplay)).vendor;
    return (!(strstr(vendor, b"X.Org\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(vendor, b"Xorg\0" as *const u8 as *const libc::c_char)).is_null())
        as libc::c_int;
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
static mut _cairo_xlib_display_list: *mut cairo_xlib_display_t = 0
    as *const cairo_xlib_display_t as *mut cairo_xlib_display_t;
unsafe extern "C" fn _noop_error_handler(
    mut display: *mut Display,
    mut event: *mut XErrorEvent,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_xlib_display_finish(
    mut abstract_display: *mut libc::c_void,
) {
    let mut display: *mut cairo_xlib_display_t = abstract_display
        as *mut cairo_xlib_display_t;
    let mut dpy: *mut Display = (*display).display;
    _cairo_xlib_display_fini_shm(display);
    if cairo_device_acquire(&mut (*display).base) as u64 == 0 {
        let mut old_handler: cairo_xlib_error_func_t = None;
        XSync(dpy, 0 as libc::c_int);
        old_handler = XSetErrorHandler(
            Some(
                _noop_error_handler
                    as unsafe extern "C" fn(
                        *mut Display,
                        *mut XErrorEvent,
                    ) -> libc::c_int,
            ),
        );
        while cairo_list_is_empty(&mut (*display).fonts) == 0 {
            _cairo_xlib_font_close(
                ({
                    let mut mptr__: *const cairo_list_t = (*display).fonts.next;
                    (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                        as *mut cairo_xlib_font_t
                }),
            );
        }
        while cairo_list_is_empty(&mut (*display).screens) == 0 {
            _cairo_xlib_screen_destroy(
                display,
                ({
                    let mut mptr__: *const cairo_list_t = (*display).screens.next;
                    (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                        as *mut cairo_xlib_screen_t
                }),
            );
        }
        XSync(dpy, 0 as libc::c_int);
        XSetErrorHandler(old_handler);
        cairo_device_release(&mut (*display).base);
    }
}
unsafe extern "C" fn _cairo_xlib_display_destroy(
    mut abstract_display: *mut libc::c_void,
) {
    let mut display: *mut cairo_xlib_display_t = abstract_display
        as *mut cairo_xlib_display_t;
    free(display as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xlib_close_display(
    mut dpy: *mut Display,
    mut codes: *mut XExtCodes,
) -> libc::c_int {
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut prev: *mut *mut cairo_xlib_display_t = 0 as *mut *mut cairo_xlib_display_t;
    let mut next: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    pthread_mutex_lock(&mut _cairo_xlib_display_mutex);
    display = _cairo_xlib_display_list;
    while !display.is_null() {
        if (*display).display == dpy {
            break;
        }
        display = (*display).next;
    }
    pthread_mutex_unlock(&mut _cairo_xlib_display_mutex);
    if display.is_null() {
        return 0 as libc::c_int;
    }
    cairo_device_finish(&mut (*display).base);
    pthread_mutex_lock(&mut _cairo_xlib_display_mutex);
    prev = &mut _cairo_xlib_display_list;
    display = _cairo_xlib_display_list;
    while !display.is_null() {
        next = (*display).next;
        if (*display).display == dpy {
            *prev = next;
            break;
        } else {
            prev = &mut (*display).next;
            display = next;
        }
    }
    pthread_mutex_unlock(&mut _cairo_xlib_display_mutex);
    let ref mut fresh8 = (*display).display;
    *fresh8 = 0 as *mut Display;
    cairo_device_destroy(&mut (*display).base);
    return 0 as libc::c_int;
}
static mut _cairo_xlib_device_backend: cairo_device_backend_t = unsafe {
    {
        let mut init = _cairo_device_backend {
            type_0: CAIRO_DEVICE_TYPE_XLIB,
            lock: None,
            unlock: None,
            flush: None,
            finish: Some(
                _cairo_xlib_display_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            destroy: Some(
                _cairo_xlib_display_destroy
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn _cairo_xlib_display_select_compositor(
    mut display: *mut cairo_xlib_display_t,
) {
    if (*display).render_major > 0 as libc::c_int
        || (*display).render_minor >= 4 as libc::c_int
    {
        let ref mut fresh9 = (*display).compositor;
        *fresh9 = _cairo_xlib_traps_compositor_get();
    } else if (*display).render_major > 0 as libc::c_int
        || (*display).render_minor >= 0 as libc::c_int
    {
        let ref mut fresh10 = (*display).compositor;
        *fresh10 = _cairo_xlib_mask_compositor_get();
    } else {
        let ref mut fresh11 = (*display).compositor;
        *fresh11 = _cairo_xlib_core_compositor_get();
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_device_create(
    mut dpy: *mut Display,
) -> *mut cairo_device_t {
    let mut current_block: u64;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut prev: *mut *mut cairo_xlib_display_t = 0 as *mut *mut cairo_xlib_display_t;
    let mut device: *mut cairo_device_t = 0 as *mut cairo_device_t;
    let mut codes: *mut XExtCodes = 0 as *mut XExtCodes;
    let mut env: *const libc::c_char = 0 as *const libc::c_char;
    pthread_mutex_lock(&mut _cairo_xlib_display_mutex);
    prev = &mut _cairo_xlib_display_list;
    loop {
        display = *prev;
        if display.is_null() {
            current_block = 17407779659766490442;
            break;
        }
        if (*display).display == dpy {
            if prev != &mut _cairo_xlib_display_list as *mut *mut cairo_xlib_display_t {
                *prev = (*display).next;
                let ref mut fresh12 = (*display).next;
                *fresh12 = _cairo_xlib_display_list;
                _cairo_xlib_display_list = display;
            }
            device = cairo_device_reference(&mut (*display).base);
            current_block = 3813301988374497887;
            break;
        } else {
            prev = &mut (**prev).next;
        }
    }
    match current_block {
        17407779659766490442 => {
            display = (if ::std::mem::size_of::<cairo_xlib_display_t>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_xlib_display_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_xlib_display_t;
            if display.is_null() {
                device = _cairo_device_create_in_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                _cairo_device_init(&mut (*display).base, &_cairo_xlib_device_backend);
                let ref mut fresh13 = (*display).display;
                *fresh13 = dpy;
                cairo_list_init(&mut (*display).screens);
                cairo_list_init(&mut (*display).fonts);
                (*display).set_closed(0 as libc::c_int as libc::c_uint);
                let ref mut fresh14 = (*display).render_minor;
                *fresh14 = -(1 as libc::c_int);
                (*display).render_major = *fresh14;
                XRenderQueryVersion(
                    dpy,
                    &mut (*display).render_major,
                    &mut (*display).render_minor,
                );
                env = getenv(b"CAIRO_DEBUG\0" as *const u8 as *const libc::c_char);
                if !env.is_null()
                    && {
                        env = strstr(
                            env,
                            b"xrender-version=\0" as *const u8 as *const libc::c_char,
                        );
                        !env.is_null()
                    }
                {
                    let mut max_render_major: libc::c_int = 0;
                    let mut max_render_minor: libc::c_int = 0;
                    env = env
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 17]>()
                                as libc::c_ulong)
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
                    if max_render_major < (*display).render_major
                        || max_render_major == (*display).render_major
                            && max_render_minor < (*display).render_minor
                    {
                        (*display).render_major = max_render_major;
                        (*display).render_minor = max_render_minor;
                    }
                }
                _cairo_xlib_display_select_compositor(display);
                let ref mut fresh15 = (*display).white;
                *fresh15 = 0 as *mut cairo_surface_t;
                memset(
                    ((*display).alpha).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[*mut cairo_surface_t; 256]>() as libc::c_ulong,
                );
                memset(
                    ((*display).solid).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[*mut cairo_surface_t; 32]>() as libc::c_ulong,
                );
                memset(
                    ((*display).solid_cache).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[uint32_t; 32]>() as libc::c_ulong,
                );
                memset(
                    ((*display).last_solid_cache).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[C2RustUnnamed_33; 2]>() as libc::c_ulong,
                );
                memset(
                    ((*display).cached_xrender_formats).as_mut_ptr()
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[*mut XRenderPictFormat; 6]>() as libc::c_ulong,
                );
                (*display).force_precision = -(1 as libc::c_int);
                _cairo_xlib_display_init_shm(display);
                (*display).set_buggy_gradients(0 as libc::c_int as libc::c_uint);
                (*display).set_buggy_pad_reflect(0 as libc::c_int as libc::c_uint);
                (*display).set_buggy_repeat(0 as libc::c_int as libc::c_uint);
                if _cairo_xlib_vendor_is_xorg(dpy) != 0 {
                    if (*(dpy as _XPrivDisplay)).release >= 60700000 as libc::c_int {
                        if (*(dpy as _XPrivDisplay)).release < 70000000 as libc::c_int {
                            (*display)
                                .set_buggy_repeat(1 as libc::c_int as libc::c_uint);
                        }
                        if (*(dpy as _XPrivDisplay)).release < 70200000 as libc::c_int {
                            (*display)
                                .set_buggy_gradients(1 as libc::c_int as libc::c_uint);
                        }
                        (*display)
                            .set_buggy_pad_reflect(1 as libc::c_int as libc::c_uint);
                    } else {
                        if (*(dpy as _XPrivDisplay)).release < 10400000 as libc::c_int {
                            (*display)
                                .set_buggy_repeat(1 as libc::c_int as libc::c_uint);
                        }
                        if (*(dpy as _XPrivDisplay)).release < 10699000 as libc::c_int {
                            (*display)
                                .set_buggy_pad_reflect(1 as libc::c_int as libc::c_uint);
                        }
                    }
                } else if !(strstr(
                    (*(dpy as _XPrivDisplay)).vendor,
                    b"XFree86\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    if (*(dpy as _XPrivDisplay)).release <= 40500000 as libc::c_int {
                        (*display).set_buggy_repeat(1 as libc::c_int as libc::c_uint);
                    }
                    (*display).set_buggy_gradients(1 as libc::c_int as libc::c_uint);
                    (*display).set_buggy_pad_reflect(1 as libc::c_int as libc::c_uint);
                }
                codes = XAddExtension(dpy);
                if codes.is_null() {
                    device = _cairo_device_create_in_error(CAIRO_STATUS_NO_MEMORY);
                    free(display as *mut libc::c_void);
                } else {
                    XESetCloseDisplay(
                        dpy,
                        (*codes).extension,
                        Some(
                            _cairo_xlib_close_display
                                as unsafe extern "C" fn(
                                    *mut Display,
                                    *mut XExtCodes,
                                ) -> libc::c_int,
                        ),
                    );
                    cairo_device_reference(&mut (*display).base);
                    let ref mut fresh16 = (*display).next;
                    *fresh16 = _cairo_xlib_display_list;
                    _cairo_xlib_display_list = display;
                    device = &mut (*display).base;
                }
            }
        }
        _ => {}
    }
    pthread_mutex_unlock(&mut _cairo_xlib_display_mutex);
    return device;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_acquire(
    mut device: *mut cairo_device_t,
    mut display: *mut *mut cairo_xlib_display_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_device_acquire(device);
    if status as u64 != 0 {
        return status;
    }
    *display = device as *mut cairo_xlib_display_t;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_get_xrender_format_for_pixman(
    mut display: *mut cairo_xlib_display_t,
    mut format: pixman_format_code_t,
) -> *mut XRenderPictFormat {
    let mut dpy: *mut Display = (*display).display;
    let mut tmpl: XRenderPictFormat = XRenderPictFormat {
        id: 0,
        type_0: 0,
        depth: 0,
        direct: XRenderDirectFormat {
            red: 0,
            redMask: 0,
            green: 0,
            greenMask: 0,
            blue: 0,
            blueMask: 0,
            alpha: 0,
            alphaMask: 0,
        },
        colormap: 0,
    };
    let mut mask: libc::c_int = 0;
    if format as libc::c_uint == PIXMAN_rgba_float as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_rgb_float as libc::c_int as libc::c_uint
    {
        return 0 as *mut XRenderPictFormat;
    }
    tmpl
        .depth = ((format as libc::c_uint >> 12 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint))
        .wrapping_add(
            (format as libc::c_uint >> 8 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint),
        )
        .wrapping_add(
            (format as libc::c_uint >> 4 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint),
        )
        .wrapping_add(
            (format as libc::c_uint >> 0 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint),
        ) as libc::c_int;
    mask = (1 as libc::c_int) << 1 as libc::c_int
        | (1 as libc::c_int) << 2 as libc::c_int;
    match format as libc::c_uint >> 16 as libc::c_int
        & 0x3f as libc::c_int as libc::c_uint
    {
        2 => {
            tmpl.type_0 = 1 as libc::c_int;
            tmpl
                .direct
                .alphaMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 12 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            if (format as libc::c_uint >> 12 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint) != 0
            {
                tmpl
                    .direct
                    .alpha = ((format as libc::c_uint >> 8 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))
                    .wrapping_add(
                        (format as libc::c_uint >> 4 as libc::c_int
                            & (((1 as libc::c_int) << 4 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint)
                            << (format as libc::c_uint >> 22 as libc::c_int
                                & 3 as libc::c_int as libc::c_uint),
                    )
                    .wrapping_add(
                        (format as libc::c_uint >> 0 as libc::c_int
                            & (((1 as libc::c_int) << 4 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint)
                            << (format as libc::c_uint >> 22 as libc::c_int
                                & 3 as libc::c_int as libc::c_uint),
                    ) as libc::c_short;
            }
            tmpl
                .direct
                .redMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 8 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .red = ((format as libc::c_uint >> 4 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_add(
                    (format as libc::c_uint >> 0 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                ) as libc::c_short;
            tmpl
                .direct
                .greenMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 4 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .green = ((format as libc::c_uint >> 0 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint)) as libc::c_short;
            tmpl
                .direct
                .blueMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 0 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl.direct.blue = 0 as libc::c_int as libc::c_short;
            mask
                |= (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 7 as libc::c_int
                    | (1 as libc::c_int) << 8 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 9 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int;
        }
        3 => {
            tmpl.type_0 = 1 as libc::c_int;
            tmpl
                .direct
                .alphaMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 12 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            if tmpl.direct.alphaMask != 0 {
                tmpl
                    .direct
                    .alpha = ((format as libc::c_uint >> 0 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))
                    .wrapping_add(
                        (format as libc::c_uint >> 4 as libc::c_int
                            & (((1 as libc::c_int) << 4 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint)
                            << (format as libc::c_uint >> 22 as libc::c_int
                                & 3 as libc::c_int as libc::c_uint),
                    )
                    .wrapping_add(
                        (format as libc::c_uint >> 8 as libc::c_int
                            & (((1 as libc::c_int) << 4 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint)
                            << (format as libc::c_uint >> 22 as libc::c_int
                                & 3 as libc::c_int as libc::c_uint),
                    ) as libc::c_short;
            }
            tmpl
                .direct
                .blueMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 0 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .blue = ((format as libc::c_uint >> 4 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_add(
                    (format as libc::c_uint >> 8 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                ) as libc::c_short;
            tmpl
                .direct
                .greenMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 4 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .green = ((format as libc::c_uint >> 8 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint)) as libc::c_short;
            tmpl
                .direct
                .redMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 8 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl.direct.red = 0 as libc::c_int as libc::c_short;
            mask
                |= (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 7 as libc::c_int
                    | (1 as libc::c_int) << 8 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 9 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int;
        }
        8 => {
            tmpl.type_0 = 1 as libc::c_int;
            tmpl
                .direct
                .blueMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 0 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .blue = ((format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_sub(
                    (format as libc::c_uint >> 0 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                ) as libc::c_short;
            tmpl
                .direct
                .greenMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 4 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .green = ((format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_sub(
                    (format as libc::c_uint >> 0 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                )
                .wrapping_sub(
                    (format as libc::c_uint >> 4 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                ) as libc::c_short;
            tmpl
                .direct
                .redMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 8 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl
                .direct
                .red = ((format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_sub(
                    (format as libc::c_uint >> 0 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                )
                .wrapping_sub(
                    (format as libc::c_uint >> 4 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                )
                .wrapping_sub(
                    (format as libc::c_uint >> 8 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                ) as libc::c_short;
            tmpl
                .direct
                .alphaMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 12 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            tmpl.direct.alpha = 0 as libc::c_int as libc::c_short;
            mask
                |= (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 7 as libc::c_int
                    | (1 as libc::c_int) << 8 as libc::c_int;
            mask
                |= (1 as libc::c_int) << 9 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int;
        }
        1 => {
            tmpl.type_0 = 1 as libc::c_int;
            tmpl.direct.alpha = 0 as libc::c_int as libc::c_short;
            tmpl
                .direct
                .alphaMask = (((1 as libc::c_int)
                << ((format as libc::c_uint >> 12 as libc::c_int
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << (format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))) - 1 as libc::c_int)
                as libc::c_short;
            mask
                |= (1 as libc::c_int) << 9 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int;
        }
        4 | 5 => {
            tmpl.type_0 = 0 as libc::c_int;
            return 0 as *mut XRenderPictFormat;
        }
        _ => {}
    }
    return XRenderFindFormat(dpy, mask as libc::c_ulong, &mut tmpl, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_get_xrender_format(
    mut display: *mut cairo_xlib_display_t,
    mut format: cairo_format_t,
) -> *mut XRenderPictFormat {
    let mut xrender_format: *mut XRenderPictFormat = 0 as *mut XRenderPictFormat;
    xrender_format = (*display).cached_xrender_formats[format as usize];
    if xrender_format.is_null() {
        let mut pict_format: libc::c_int = 5 as libc::c_int;
        let mut current_block_11: u64;
        match format as libc::c_int {
            3 => {
                pict_format = 4 as libc::c_int;
                current_block_11 = 26972500619410423;
            }
            2 => {
                pict_format = 2 as libc::c_int;
                current_block_11 = 26972500619410423;
            }
            1 => {
                pict_format = 1 as libc::c_int;
                current_block_11 = 26972500619410423;
            }
            4 => {
                xrender_format = _cairo_xlib_display_get_xrender_format_for_pixman(
                    display,
                    PIXMAN_r5g6b5,
                );
                current_block_11 = 26972500619410423;
            }
            5 => {
                xrender_format = _cairo_xlib_display_get_xrender_format_for_pixman(
                    display,
                    PIXMAN_x2r10g10b10,
                );
                current_block_11 = 26972500619410423;
            }
            7 => {
                xrender_format = _cairo_xlib_display_get_xrender_format_for_pixman(
                    display,
                    PIXMAN_rgba_float,
                );
                current_block_11 = 26972500619410423;
            }
            6 => {
                xrender_format = _cairo_xlib_display_get_xrender_format_for_pixman(
                    display,
                    PIXMAN_rgb_float,
                );
                current_block_11 = 26972500619410423;
            }
            0 => {
                current_block_11 = 18353277038117392318;
            }
            -1 | _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-xlib-display.c\0" as *const u8
                            as *const libc::c_char,
                        527 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 98],
                            &[libc::c_char; 98],
                        >(
                            b"XRenderPictFormat *_cairo_xlib_display_get_xrender_format(cairo_xlib_display_t *, cairo_format_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                current_block_11 = 18353277038117392318;
            }
        }
        match current_block_11 {
            18353277038117392318 => {
                pict_format = 0 as libc::c_int;
            }
            _ => {}
        }
        if pict_format != 5 as libc::c_int {
            xrender_format = XRenderFindStandardFormat((*display).display, pict_format);
        }
        let ref mut fresh17 = (*display).cached_xrender_formats[format as usize];
        *fresh17 = xrender_format;
    }
    return xrender_format;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_get_screen(
    mut display: *mut cairo_xlib_display_t,
    mut screen: *mut Screen,
) -> *mut cairo_xlib_screen_t {
    let mut info: *mut cairo_xlib_screen_t = 0 as *mut cairo_xlib_screen_t;
    info = ({
        let mut mptr__: *const cairo_list_t = (*display).screens.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_xlib_screen_t
    });
    while &mut (*info).link as *mut cairo_list_t
        != &mut (*display).screens as *mut cairo_list_t
    {
        if (*info).screen == screen {
            if (*display).screens.next != &mut (*info).link as *mut cairo_list_t {
                cairo_list_move(&mut (*info).link, &mut (*display).screens);
            }
            return info;
        }
        info = ({
            let mut mptr__: *const cairo_list_t = (*info).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_xlib_screen_t
        });
    }
    return 0 as *mut cairo_xlib_screen_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_has_repeat(
    mut device: *mut cairo_device_t,
) -> cairo_bool_t {
    return ((*(device as *mut cairo_xlib_display_t)).buggy_repeat() == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_has_reflect(
    mut device: *mut cairo_device_t,
) -> cairo_bool_t {
    return ((*(device as *mut cairo_xlib_display_t)).buggy_pad_reflect() == 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_has_gradients(
    mut device: *mut cairo_device_t,
) -> cairo_bool_t {
    return ((*(device as *mut cairo_xlib_display_t)).buggy_gradients() == 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_device_debug_cap_xrender_version(
    mut device: *mut cairo_device_t,
    mut major_version: libc::c_int,
    mut minor_version: libc::c_int,
) {
    let mut display: *mut cairo_xlib_display_t = device as *mut cairo_xlib_display_t;
    if device.is_null() || (*device).status as libc::c_uint != 0 {
        return;
    }
    if (*(*device).backend).type_0 as libc::c_int
        != CAIRO_DEVICE_TYPE_XLIB as libc::c_int
    {
        return;
    }
    if major_version < (*display).render_major
        || major_version == (*display).render_major
            && minor_version < (*display).render_minor
    {
        (*display).render_major = major_version;
        (*display).render_minor = minor_version;
    }
    _cairo_xlib_display_select_compositor(display);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_device_debug_set_precision(
    mut device: *mut cairo_device_t,
    mut precision: libc::c_int,
) {
    if device.is_null() || (*device).status as libc::c_uint != 0 {
        return;
    }
    if (*(*device).backend).type_0 as libc::c_int
        != CAIRO_DEVICE_TYPE_XLIB as libc::c_int
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
        return;
    }
    (*(device as *mut cairo_xlib_display_t)).force_precision = precision;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_device_debug_get_precision(
    mut device: *mut cairo_device_t,
) -> libc::c_int {
    if device.is_null() || (*device).status as libc::c_uint != 0 {
        return -(1 as libc::c_int);
    }
    if (*(*device).backend).type_0 as libc::c_int
        != CAIRO_DEVICE_TYPE_XLIB as libc::c_int
    {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
        return -(1 as libc::c_int);
    }
    return (*(device as *mut cairo_xlib_display_t)).force_precision;
}
