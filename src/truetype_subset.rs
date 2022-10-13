use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    pub type _cairo_pattern;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_ucs4_to_utf8(unicode: uint32_t, utf8: *mut libc::c_char) -> libc::c_int;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_grow_by(
        array: *mut cairo_array_t,
        additional: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_array_truncate(array: *mut cairo_array_t, num_elements: libc::c_uint);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_append_multiple(
        array: *mut cairo_array_t,
        elements: *const libc::c_void,
        num_elements: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_array_allocate(
        array: *mut cairo_array_t,
        num_elements: libc::c_uint,
        elements: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn _cairo_unicode_to_winansi(unicode: libc::c_ulong) -> libc::c_int;
    fn _cairo_escape_ps_name(ps_name: *mut *mut libc::c_char) -> cairo_int_status_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
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
pub type cairo_recursive_mutex_t = cairo_recursive_mutex_impl_t;
pub type cairo_recursive_mutex_impl_t = pthread_mutex_t;
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
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_subset {
    pub scaled_font: *mut cairo_scaled_font_t,
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub glyphs: *mut libc::c_ulong,
    pub utf8: *mut *mut libc::c_char,
    pub glyph_names: *mut *mut libc::c_char,
    pub to_latin_char: *mut libc::c_int,
    pub latin_to_subset_glyph_index: *mut libc::c_ulong,
    pub num_glyphs: libc::c_uint,
    pub is_composite: cairo_bool_t,
    pub is_scaled: cairo_bool_t,
    pub is_latin: cairo_bool_t,
}
pub type cairo_scaled_font_subset_t = _cairo_scaled_font_subset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_truetype_subset {
    pub family_name_utf8: *mut libc::c_char,
    pub ps_name: *mut libc::c_char,
    pub widths: *mut libc::c_double,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub data: *mut libc::c_uchar,
    pub data_length: libc::c_ulong,
    pub string_offsets: *mut libc::c_ulong,
    pub num_string_offsets: libc::c_ulong,
}
pub type cairo_truetype_subset_t = _cairo_truetype_subset;
pub type cairo_truetype_font_t = _cairo_truetype_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_truetype_font {
    pub scaled_font_subset: *mut cairo_scaled_font_subset_t,
    pub truetype_tables: [table_t; 10],
    pub num_tables: libc::c_int,
    pub base: C2RustUnnamed,
    pub glyphs: *mut subset_glyph_t,
    pub backend: *const cairo_scaled_font_backend_t,
    pub num_glyphs: libc::c_uint,
    pub widths: *mut libc::c_int,
    pub checksum_index: libc::c_int,
    pub output: cairo_array_t,
    pub string_offsets: cairo_array_t,
    pub last_offset: libc::c_ulong,
    pub last_boundary: libc::c_ulong,
    pub parent_to_subset: *mut libc::c_int,
    pub status: cairo_status_t,
    pub is_pdf: cairo_bool_t,
}
pub type subset_glyph_t = subset_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subset_glyph {
    pub parent_index: libc::c_int,
    pub location: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub font_name: *mut libc::c_char,
    pub ps_name: *mut libc::c_char,
    pub num_glyphs_in_face: libc::c_int,
    pub x_min: libc::c_long,
    pub y_min: libc::c_long,
    pub x_max: libc::c_long,
    pub y_max: libc::c_long,
    pub ascent: libc::c_long,
    pub descent: libc::c_long,
    pub units_per_em: libc::c_int,
}
pub type table_t = table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table {
    pub tag: libc::c_ulong,
    pub write: Option::<
        unsafe extern "C" fn(*mut cairo_truetype_font_t, libc::c_ulong) -> cairo_status_t,
    >,
    pub pos: libc::c_int,
}
pub type tt_maxp_t = _tt_maxp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_maxp {
    pub version_1: int16_t,
    pub version_2: int16_t,
    pub num_glyphs: uint16_t,
    pub max_points: uint16_t,
    pub max_contours: uint16_t,
    pub max_composite_points: uint16_t,
    pub max_composite_contours: uint16_t,
    pub max_zones: uint16_t,
    pub max_twilight_points: uint16_t,
    pub max_storage: uint16_t,
    pub max_function_defs: uint16_t,
    pub max_instruction_defs: uint16_t,
    pub max_stack_elements: uint16_t,
    pub max_size_of_instructions: uint16_t,
    pub max_component_elements: uint16_t,
    pub max_component_depth: uint16_t,
}
pub type tt_head_t = _tt_head;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_head {
    pub version_1: int16_t,
    pub version_2: int16_t,
    pub revision_1: int16_t,
    pub revision_2: int16_t,
    pub checksum_1: uint16_t,
    pub checksum_2: uint16_t,
    pub magic_1: uint16_t,
    pub magic_2: uint16_t,
    pub flags: uint16_t,
    pub units_per_em: uint16_t,
    pub created_1: int16_t,
    pub created_2: int16_t,
    pub created_3: int16_t,
    pub created_4: int16_t,
    pub modified_1: int16_t,
    pub modified_2: int16_t,
    pub modified_3: int16_t,
    pub modified_4: int16_t,
    pub x_min: int16_t,
    pub y_min: int16_t,
    pub x_max: int16_t,
    pub y_max: int16_t,
    pub mac_style: uint16_t,
    pub lowest_rec_pppem: uint16_t,
    pub font_direction_hint: int16_t,
    pub index_to_loc_format: int16_t,
    pub glyph_data_format: int16_t,
}
pub type tt_hhea_t = _tt_hhea;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_hhea {
    pub version_1: int16_t,
    pub version_2: int16_t,
    pub ascender: int16_t,
    pub descender: int16_t,
    pub line_gap: int16_t,
    pub advance_max_width: uint16_t,
    pub min_left_side_bearing: int16_t,
    pub min_right_side_bearing: int16_t,
    pub x_max_extent: int16_t,
    pub caret_slope_rise: int16_t,
    pub caret_slope_run: int16_t,
    pub reserved: [int16_t; 5],
    pub metric_data_format: int16_t,
    pub num_hmetrics: uint16_t,
}
pub type cmap_unicode_range_t = _cmap_unicode_range;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cmap_unicode_range {
    pub start: libc::c_uint,
    pub end: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub bytes: *mut libc::c_uchar,
    pub short_offsets: *mut uint16_t,
    pub long_offsets: *mut uint32_t,
}
pub type tt_composite_glyph_t = _tt_composite_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_composite_glyph {
    pub flags: uint16_t,
    pub index: uint16_t,
    pub args: [uint16_t; 6],
}
pub type tt_glyph_data_t = _tt_glyph_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_glyph_data {
    pub num_contours: int16_t,
    pub data: [int8_t; 8],
    pub glyph: tt_composite_glyph_t,
}
pub type tt_name_t = _tt_name;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_name {
    pub format: uint16_t,
    pub num_records: uint16_t,
    pub strings_offset: uint16_t,
    pub records: [tt_name_record_t; 1],
}
pub type tt_name_record_t = _tt_name_record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_name_record {
    pub platform: uint16_t,
    pub encoding: uint16_t,
    pub language: uint16_t,
    pub name: uint16_t,
    pub length: uint16_t,
    pub offset: uint16_t,
}
pub type tt_cmap_t = _tt_cmap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_cmap {
    pub version: uint16_t,
    pub num_tables: uint16_t,
    pub index: [tt_cmap_index_t; 1],
}
pub type tt_cmap_index_t = _tt_cmap_index;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_cmap_index {
    pub platform: uint16_t,
    pub encoding: uint16_t,
    pub offset: uint32_t,
}
pub type tt_segment_map_t = _segment_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _segment_map {
    pub format: uint16_t,
    pub length: uint16_t,
    pub version: uint16_t,
    pub segCountX2: uint16_t,
    pub searchRange: uint16_t,
    pub entrySelector: uint16_t,
    pub rangeShift: uint16_t,
    pub endCount: [uint16_t; 1],
}
pub type tt_os2_t = _tt_os2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tt_os2 {
    pub _unused1: [uint16_t; 2],
    pub usWeightClass: uint16_t,
    pub _unused2: [uint16_t; 28],
    pub fsSelection: uint16_t,
    pub _unused3: [uint16_t; 11],
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_cmpxchg_impl(
    mut x: *mut cairo_atomic_int_t,
    mut oldv: cairo_atomic_int_t,
    mut newv: cairo_atomic_int_t,
) -> cairo_bool_t {
    let mut expected: cairo_atomic_int_t = oldv;
    let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh0.0;
    return fresh0.1 as cairo_bool_t;
}
#[inline]
unsafe extern "C" fn cpu_to_be16(mut v: uint16_t) -> uint16_t {
    return ((v as libc::c_int) << 8 as libc::c_int
        | v as libc::c_int >> 8 as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn be16_to_cpu(mut v: uint16_t) -> uint16_t {
    return cpu_to_be16(v);
}
#[inline]
unsafe extern "C" fn cpu_to_be32(mut v: uint32_t) -> uint32_t {
    return v >> 24 as libc::c_int
        | v >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | v << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
        | v << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn be32_to_cpu(mut v: uint32_t) -> uint32_t {
    return cpu_to_be32(v);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab_plus_c(
    mut a: size_t,
    mut size: size_t,
    mut c: size_t,
) -> *mut libc::c_void {
    let mut d: size_t = 0;
    let mut e: size_t = 0;
    let (fresh1, fresh2) = a.overflowing_mul(size);
    *(&mut d as *mut size_t) = fresh1;
    if fresh2 {
        return 0 as *mut libc::c_void;
    }
    let (fresh3, fresh4) = d.overflowing_add(c);
    *(&mut e as *mut size_t) = fresh3;
    if fresh4 {
        return 0 as *mut libc::c_void;
    }
    return if e != 0 as libc::c_int as libc::c_ulong {
        malloc(e)
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn _cairo_truetype_font_set_error(
    mut font: *mut cairo_truetype_font_t,
    mut status: cairo_status_t,
) -> cairo_status_t {
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        || status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    let mut ret__: libc::c_int = 0;
    if (status as libc::c_uint) < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status < CAIRO_STATUS_LAST_STATUS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-truetype-subset.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"cairo_status_t _cairo_truetype_font_set_error(cairo_truetype_font_t *, cairo_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    ret__ = _cairo_atomic_int_cmpxchg_impl(
        &mut (*font).status as *mut cairo_status_t as *mut cairo_atomic_int_t,
        CAIRO_STATUS_SUCCESS as libc::c_int,
        status as cairo_atomic_int_t,
    );
    return _cairo_error(status);
}
unsafe extern "C" fn _cairo_truetype_font_create(
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut is_pdf: cairo_bool_t,
    mut font_return: *mut *mut cairo_truetype_font_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut is_synthetic: cairo_bool_t = 0;
    let mut font: *mut cairo_truetype_font_t = 0 as *mut cairo_truetype_font_t;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut head: tt_head_t = tt_head_t {
        version_1: 0,
        version_2: 0,
        revision_1: 0,
        revision_2: 0,
        checksum_1: 0,
        checksum_2: 0,
        magic_1: 0,
        magic_2: 0,
        flags: 0,
        units_per_em: 0,
        created_1: 0,
        created_2: 0,
        created_3: 0,
        created_4: 0,
        modified_1: 0,
        modified_2: 0,
        modified_3: 0,
        modified_4: 0,
        x_min: 0,
        y_min: 0,
        x_max: 0,
        y_max: 0,
        mac_style: 0,
        lowest_rec_pppem: 0,
        font_direction_hint: 0,
        index_to_loc_format: 0,
        glyph_data_format: 0,
    };
    let mut hhea: tt_hhea_t = tt_hhea_t {
        version_1: 0,
        version_2: 0,
        ascender: 0,
        descender: 0,
        line_gap: 0,
        advance_max_width: 0,
        min_left_side_bearing: 0,
        min_right_side_bearing: 0,
        x_max_extent: 0,
        caret_slope_rise: 0,
        caret_slope_run: 0,
        reserved: [0; 5],
        metric_data_format: 0,
        num_hmetrics: 0,
    };
    let mut maxp: tt_maxp_t = tt_maxp_t {
        version_1: 0,
        version_2: 0,
        num_glyphs: 0,
        max_points: 0,
        max_contours: 0,
        max_composite_points: 0,
        max_composite_contours: 0,
        max_zones: 0,
        max_twilight_points: 0,
        max_storage: 0,
        max_function_defs: 0,
        max_instruction_defs: 0,
        max_stack_elements: 0,
        max_size_of_instructions: 0,
        max_component_elements: 0,
        max_component_depth: 0,
    };
    let mut size: libc::c_ulong = 0;
    backend = (*(*scaled_font_subset).scaled_font).backend;
    if ((*backend).load_truetype_table).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if ((*backend).is_synthetic).is_some() {
        status = ((*backend).is_synthetic)
            .expect(
                "non-null function pointer",
            )((*scaled_font_subset).scaled_font as *mut libc::c_void, &mut is_synthetic)
            as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        if is_synthetic != 0 {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
    }
    size = ::std::mem::size_of::<tt_head_t>() as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('e' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'd' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut head as *mut tt_head_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    size = ::std::mem::size_of::<tt_maxp_t>() as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*scaled_font_subset).scaled_font as *mut libc::c_void,
        (('m' as i32 as uint32_t) << 24 as libc::c_int
            | (('a' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('x' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut maxp as *mut tt_maxp_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    size = ::std::mem::size_of::<tt_hhea_t>() as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('h' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('e' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'a' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut hhea as *mut tt_hhea_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    font = (if ::std::mem::size_of::<cairo_truetype_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_truetype_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_truetype_font_t;
    if font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh5 = (*font).backend;
    *fresh5 = backend;
    (*font).base.num_glyphs_in_face = be16_to_cpu(maxp.num_glyphs) as libc::c_int;
    let ref mut fresh6 = (*font).scaled_font_subset;
    *fresh6 = scaled_font_subset;
    (*font).last_offset = 0 as libc::c_int as libc::c_ulong;
    (*font).last_boundary = 0 as libc::c_int as libc::c_ulong;
    _cairo_array_init(
        &mut (*font).output,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    );
    status = _cairo_array_grow_by(
        &mut (*font).output,
        4096 as libc::c_int as libc::c_uint,
    );
    if !(status as u64 != 0) {
        let ref mut fresh7 = (*font).glyphs;
        *fresh7 = calloc(
            ((*font).base.num_glyphs_in_face + 2 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<subset_glyph_t>() as libc::c_ulong,
        ) as *mut subset_glyph_t;
        if ((*font).glyphs).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            let ref mut fresh8 = (*font).parent_to_subset;
            *fresh8 = calloc(
                ((*font).base.num_glyphs_in_face + 1 as libc::c_int) as libc::c_ulong,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            if ((*font).parent_to_subset).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                (*font).is_pdf = is_pdf;
                (*font).num_glyphs = 0 as libc::c_int as libc::c_uint;
                (*font)
                    .base
                    .x_min = be16_to_cpu(head.x_min as uint16_t) as int16_t
                    as libc::c_long;
                (*font)
                    .base
                    .y_min = be16_to_cpu(head.y_min as uint16_t) as int16_t
                    as libc::c_long;
                (*font)
                    .base
                    .x_max = be16_to_cpu(head.x_max as uint16_t) as int16_t
                    as libc::c_long;
                (*font)
                    .base
                    .y_max = be16_to_cpu(head.y_max as uint16_t) as int16_t
                    as libc::c_long;
                (*font)
                    .base
                    .ascent = be16_to_cpu(hhea.ascender as uint16_t) as int16_t
                    as libc::c_long;
                (*font)
                    .base
                    .descent = be16_to_cpu(hhea.descender as uint16_t) as int16_t
                    as libc::c_long;
                (*font)
                    .base
                    .units_per_em = be16_to_cpu(head.units_per_em) as int16_t
                    as libc::c_int;
                if (*font).base.units_per_em == 0 as libc::c_int {
                    (*font).base.units_per_em = 2048 as libc::c_int;
                }
                let ref mut fresh9 = (*font).base.ps_name;
                *fresh9 = 0 as *mut libc::c_char;
                let ref mut fresh10 = (*font).base.font_name;
                *fresh10 = 0 as *mut libc::c_char;
                status = _cairo_truetype_read_font_name(
                    (*scaled_font_subset).scaled_font,
                    &mut (*font).base.ps_name,
                    &mut (*font).base.font_name,
                ) as cairo_status_t;
                if !(status as libc::c_uint
                    != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    && (status as libc::c_uint)
                        < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint)
                {
                    if ((*font).base.ps_name).is_null() {
                        let ref mut fresh11 = (*font).base.ps_name;
                        *fresh11 = (if 30 as libc::c_int != 0 as libc::c_int {
                            malloc(30 as libc::c_int as libc::c_ulong)
                        } else {
                            0 as *mut libc::c_void
                        }) as *mut libc::c_char;
                        if ((*font).base.ps_name).is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                            current_block = 17700620985501781025;
                        } else {
                            snprintf(
                                (*font).base.ps_name,
                                30 as libc::c_int as libc::c_ulong,
                                b"CairoFont-%u-%u\0" as *const u8 as *const libc::c_char,
                                (*scaled_font_subset).font_id,
                                (*scaled_font_subset).subset_id,
                            );
                            current_block = 13325891313334703151;
                        }
                    } else {
                        current_block = 13325891313334703151;
                    }
                    match current_block {
                        17700620985501781025 => {}
                        _ => {
                            let ref mut fresh12 = (*font).widths;
                            *fresh12 = calloc(
                                ((*font).base.num_glyphs_in_face + 1 as libc::c_int)
                                    as libc::c_ulong,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ) as *mut libc::c_int;
                            if ((*font).widths).is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                            } else {
                                _cairo_array_init(
                                    &mut (*font).string_offsets,
                                    ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
                                        as libc::c_uint,
                                );
                                status = _cairo_array_grow_by(
                                    &mut (*font).string_offsets,
                                    10 as libc::c_int as libc::c_uint,
                                );
                                if status as u64 != 0 {
                                    _cairo_array_fini(&mut (*font).string_offsets);
                                    free((*font).widths as *mut libc::c_void);
                                } else {
                                    (*font).status = CAIRO_STATUS_SUCCESS;
                                    *font_return = font;
                                    return CAIRO_STATUS_SUCCESS;
                                }
                            }
                            free((*font).base.ps_name as *mut libc::c_void);
                        }
                    }
                }
                free((*font).parent_to_subset as *mut libc::c_void);
                free((*font).base.font_name as *mut libc::c_void);
            }
            free((*font).glyphs as *mut libc::c_void);
        }
    }
    _cairo_array_fini(&mut (*font).output);
    free(font as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn cairo_truetype_font_destroy(mut font: *mut cairo_truetype_font_t) {
    _cairo_array_fini(&mut (*font).string_offsets);
    free((*font).widths as *mut libc::c_void);
    free((*font).base.ps_name as *mut libc::c_void);
    free((*font).base.font_name as *mut libc::c_void);
    free((*font).parent_to_subset as *mut libc::c_void);
    free((*font).glyphs as *mut libc::c_void);
    _cairo_array_fini(&mut (*font).output);
    free(font as *mut libc::c_void);
}
unsafe extern "C" fn cairo_truetype_font_allocate_write_buffer(
    mut font: *mut cairo_truetype_font_t,
    mut length: size_t,
    mut buffer: *mut *mut libc::c_uchar,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    status = _cairo_array_allocate(
        &mut (*font).output,
        length as libc::c_uint,
        buffer as *mut *mut libc::c_void,
    );
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_write(
    mut font: *mut cairo_truetype_font_t,
    mut data: *const libc::c_void,
    mut length: size_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return;
    }
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        data,
        length as libc::c_uint,
    );
    if status as u64 != 0 {
        status = _cairo_truetype_font_set_error(font, status);
    }
}
unsafe extern "C" fn cairo_truetype_font_write_be16(
    mut font: *mut cairo_truetype_font_t,
    mut value: uint16_t,
) {
    let mut be16_value: uint16_t = 0;
    if (*font).status as u64 != 0 {
        return;
    }
    be16_value = cpu_to_be16(value);
    cairo_truetype_font_write(
        font,
        &mut be16_value as *mut uint16_t as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn cairo_truetype_font_write_be32(
    mut font: *mut cairo_truetype_font_t,
    mut value: uint32_t,
) {
    let mut be32_value: uint32_t = 0;
    if (*font).status as u64 != 0 {
        return;
    }
    be32_value = cpu_to_be32(value);
    cairo_truetype_font_write(
        font,
        &mut be32_value as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn cairo_truetype_font_align_output(
    mut font: *mut cairo_truetype_font_t,
    mut aligned: *mut libc::c_ulong,
) -> cairo_status_t {
    let mut length: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    let mut padding: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    length = _cairo_array_num_elements(&mut (*font).output) as libc::c_int;
    *aligned = (length + 3 as libc::c_int & !(3 as libc::c_int)) as libc::c_ulong;
    pad = (*aligned).wrapping_sub(length as libc::c_ulong) as libc::c_int;
    if pad != 0 {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = cairo_truetype_font_allocate_write_buffer(
            font,
            pad as size_t,
            &mut padding,
        );
        if status as u64 != 0 {
            return status;
        }
        memset(padding as *mut libc::c_void, 0 as libc::c_int, pad as libc::c_ulong);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_check_boundary(
    mut font: *mut cairo_truetype_font_t,
    mut boundary: libc::c_ulong,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    if boundary.wrapping_sub((*font).last_offset) > 65535 as libc::c_int as libc::c_ulong
    {
        status = _cairo_array_append(
            &mut (*font).string_offsets,
            &mut (*font).last_boundary as *mut libc::c_ulong as *const libc::c_void,
        );
        if status as u64 != 0 {
            return _cairo_truetype_font_set_error(font, status);
        }
        (*font).last_offset = (*font).last_boundary;
    }
    (*font).last_boundary = boundary;
    return CAIRO_STATUS_SUCCESS;
}
static mut winansi_unicode_ranges: [cmap_unicode_range_t; 14] = [
    {
        let mut init = _cmap_unicode_range {
            start: 0x20 as libc::c_int as libc::c_uint,
            end: 0x7f as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0xa0 as libc::c_int as libc::c_uint,
            end: 0xff as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x152 as libc::c_int as libc::c_uint,
            end: 0x153 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x160 as libc::c_int as libc::c_uint,
            end: 0x161 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x178 as libc::c_int as libc::c_uint,
            end: 0x178 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x17d as libc::c_int as libc::c_uint,
            end: 0x17e as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x192 as libc::c_int as libc::c_uint,
            end: 0x192 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x2c6 as libc::c_int as libc::c_uint,
            end: 0x2c6 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x2dc as libc::c_int as libc::c_uint,
            end: 0x2dc as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x2013 as libc::c_int as libc::c_uint,
            end: 0x2026 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x2030 as libc::c_int as libc::c_uint,
            end: 0x2030 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x2039 as libc::c_int as libc::c_uint,
            end: 0x203a as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x20ac as libc::c_int as libc::c_uint,
            end: 0x20ac as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = _cmap_unicode_range {
            start: 0x2122 as libc::c_int as libc::c_uint,
            end: 0x2122 as libc::c_int as libc::c_uint,
        };
        init
    },
];
unsafe extern "C" fn cairo_truetype_font_write_cmap_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    let mut range_offset: libc::c_int = 0;
    let mut num_ranges: libc::c_int = 0;
    let mut entry_selector: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    num_ranges = (::std::mem::size_of::<[cmap_unicode_range_t; 14]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cmap_unicode_range_t>() as libc::c_ulong)
        as libc::c_int;
    length = 16 as libc::c_int + (num_ranges + 1 as libc::c_int) * 8 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_ranges {
        length = (length as libc::c_uint)
            .wrapping_add(
                (winansi_unicode_ranges[i as usize].end)
                    .wrapping_sub(winansi_unicode_ranges[i as usize].start)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint),
            ) as libc::c_int as libc::c_int;
        i += 1;
    }
    entry_selector = 0 as libc::c_int;
    while (1 as libc::c_int) << entry_selector <= num_ranges + 1 as libc::c_int {
        entry_selector += 1;
    }
    entry_selector -= 1;
    cairo_truetype_font_write_be16(font, 0 as libc::c_int as uint16_t);
    cairo_truetype_font_write_be16(font, 1 as libc::c_int as uint16_t);
    cairo_truetype_font_write_be16(font, 3 as libc::c_int as uint16_t);
    cairo_truetype_font_write_be16(font, 1 as libc::c_int as uint16_t);
    cairo_truetype_font_write_be32(font, 12 as libc::c_int as uint32_t);
    cairo_truetype_font_write_be16(font, 4 as libc::c_int as uint16_t);
    cairo_truetype_font_write_be16(font, length as uint16_t);
    cairo_truetype_font_write_be16(font, 0 as libc::c_int as uint16_t);
    cairo_truetype_font_write_be16(
        font,
        (num_ranges * 2 as libc::c_int + 2 as libc::c_int) as uint16_t,
    );
    cairo_truetype_font_write_be16(
        font,
        ((1 as libc::c_int) << entry_selector + 1 as libc::c_int) as uint16_t,
    );
    cairo_truetype_font_write_be16(font, entry_selector as uint16_t);
    cairo_truetype_font_write_be16(
        font,
        (num_ranges * 2 as libc::c_int + 2 as libc::c_int
            - ((1 as libc::c_int) << entry_selector + 1 as libc::c_int)) as uint16_t,
    );
    i = 0 as libc::c_int;
    while i < num_ranges {
        cairo_truetype_font_write_be16(
            font,
            winansi_unicode_ranges[i as usize].end as uint16_t,
        );
        i += 1;
    }
    cairo_truetype_font_write_be16(font, 0xffff as libc::c_int as uint16_t);
    cairo_truetype_font_write_be16(font, 0 as libc::c_int as uint16_t);
    i = 0 as libc::c_int;
    while i < num_ranges {
        cairo_truetype_font_write_be16(
            font,
            winansi_unicode_ranges[i as usize].start as uint16_t,
        );
        i += 1;
    }
    cairo_truetype_font_write_be16(font, 0xffff as libc::c_int as uint16_t);
    i = 0 as libc::c_int;
    while i < num_ranges {
        cairo_truetype_font_write_be16(font, 0 as libc::c_int as uint16_t);
        i += 1;
    }
    cairo_truetype_font_write_be16(font, 1 as libc::c_int as uint16_t);
    range_offset = num_ranges * 2 as libc::c_int + 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_ranges {
        cairo_truetype_font_write_be16(font, range_offset as uint16_t);
        range_offset = (range_offset as libc::c_uint)
            .wrapping_add(
                (winansi_unicode_ranges[i as usize].end)
                    .wrapping_sub(winansi_unicode_ranges[i as usize].start)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint)
                    .wrapping_sub(2 as libc::c_int as libc::c_uint),
            ) as libc::c_int as libc::c_int;
        i += 1;
    }
    cairo_truetype_font_write_be16(font, 0 as libc::c_int as uint16_t);
    i = 0 as libc::c_int;
    while i < num_ranges {
        j = winansi_unicode_ranges[i as usize].start;
        while j
            < (winansi_unicode_ranges[i as usize].end)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            let mut ch: libc::c_int = _cairo_unicode_to_winansi(j as libc::c_ulong);
            let mut glyph: libc::c_int = 0;
            if ch > 0 as libc::c_int {
                glyph = *((*(*font).scaled_font_subset).latin_to_subset_glyph_index)
                    .offset(ch as isize) as libc::c_int;
            } else {
                glyph = 0 as libc::c_int;
            }
            cairo_truetype_font_write_be16(font, glyph as uint16_t);
            j = j.wrapping_add(1);
        }
        i += 1;
    }
    return (*font).status;
}
unsafe extern "C" fn cairo_truetype_font_write_generic_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_ulong = 0;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        tag,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    status = cairo_truetype_font_allocate_write_buffer(font, size, &mut buffer);
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        tag,
        0 as libc::c_int as libc::c_long,
        buffer,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_remap_composite_glyph(
    mut font: *mut cairo_truetype_font_t,
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_ulong,
) -> cairo_status_t {
    let mut glyph_data: *mut tt_glyph_data_t = 0 as *mut tt_glyph_data_t;
    let mut composite_glyph: *mut tt_composite_glyph_t = 0 as *mut tt_composite_glyph_t;
    let mut num_args: libc::c_int = 0;
    let mut has_more_components: libc::c_int = 0;
    let mut flags: libc::c_ushort = 0;
    let mut index: libc::c_ushort = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut end: *mut libc::c_uchar = buffer.offset(size as isize);
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    glyph_data = buffer as *mut tt_glyph_data_t;
    if &mut (*glyph_data).data as *mut [int8_t; 8] as *mut libc::c_uchar >= end {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if be16_to_cpu((*glyph_data).num_contours as uint16_t) as int16_t as libc::c_int
        >= 0 as libc::c_int
    {
        return CAIRO_STATUS_SUCCESS;
    }
    composite_glyph = &mut (*glyph_data).glyph;
    loop {
        if &mut *((*composite_glyph).args).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut uint16_t as *mut libc::c_uchar > end
        {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        flags = be16_to_cpu((*composite_glyph).flags);
        has_more_components = flags as libc::c_int & 0x20 as libc::c_int;
        status = cairo_truetype_font_use_glyph(
            font,
            be16_to_cpu((*composite_glyph).index),
            &mut index,
        );
        if status as u64 != 0 {
            return status;
        }
        (*composite_glyph).index = cpu_to_be16(index);
        num_args = 1 as libc::c_int;
        if flags as libc::c_int & 0x1 as libc::c_int != 0 {
            num_args += 1 as libc::c_int;
        }
        if flags as libc::c_int & 0x8 as libc::c_int != 0 {
            num_args += 1 as libc::c_int;
        } else if flags as libc::c_int & 0x40 as libc::c_int != 0 {
            num_args += 2 as libc::c_int;
        } else if flags as libc::c_int & 0x80 as libc::c_int != 0 {
            num_args += 4 as libc::c_int;
        }
        composite_glyph = &mut *((*composite_glyph).args)
            .as_mut_ptr()
            .offset(num_args as isize) as *mut uint16_t as *mut tt_composite_glyph_t;
        if !(has_more_components != 0) {
            break;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_write_glyf_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut start_offset: libc::c_ulong = 0;
    let mut index: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    let mut next: libc::c_ulong = 0;
    let mut header: tt_head_t = tt_head_t {
        version_1: 0,
        version_2: 0,
        revision_1: 0,
        revision_2: 0,
        checksum_1: 0,
        checksum_2: 0,
        magic_1: 0,
        magic_2: 0,
        flags: 0,
        units_per_em: 0,
        created_1: 0,
        created_2: 0,
        created_3: 0,
        created_4: 0,
        modified_1: 0,
        modified_2: 0,
        modified_3: 0,
        modified_4: 0,
        x_min: 0,
        y_min: 0,
        x_max: 0,
        y_max: 0,
        mac_style: 0,
        lowest_rec_pppem: 0,
        font_direction_hint: 0,
        index_to_loc_format: 0,
        glyph_data_format: 0,
    };
    let mut begin: libc::c_ulong = 0;
    let mut end: libc::c_ulong = 0;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 {
        bytes: 0 as *mut libc::c_uchar,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = ::std::mem::size_of::<tt_head_t>() as libc::c_ulong;
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('e' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'd' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut header as *mut tt_head_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    if be16_to_cpu(header.index_to_loc_format as uint16_t) as libc::c_int
        == 0 as libc::c_int
    {
        size = (::std::mem::size_of::<int16_t>() as libc::c_ulong)
            .wrapping_mul(
                ((*font).base.num_glyphs_in_face + 1 as libc::c_int) as libc::c_ulong,
            );
    } else {
        size = (::std::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(
                ((*font).base.num_glyphs_in_face + 1 as libc::c_int) as libc::c_ulong,
            );
    }
    u
        .bytes = (if size != 0 as libc::c_int as libc::c_ulong {
        malloc(size)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if (u.bytes).is_null() {
        return _cairo_truetype_font_set_error(font, CAIRO_STATUS_NO_MEMORY);
    }
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('l' as i32 as uint32_t) << 24 as libc::c_int
            | (('o' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('c' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'a' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        u.bytes,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        free(u.bytes as *mut libc::c_void);
        return _cairo_truetype_font_set_error(font, status);
    }
    start_offset = _cairo_array_num_elements(&mut (*font).output) as libc::c_ulong;
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        if !(i < (*font).num_glyphs) {
            current_block = 7990025728955927862;
            break;
        }
        index = (*((*font).glyphs).offset(i as isize)).parent_index as libc::c_ulong;
        if be16_to_cpu(header.index_to_loc_format as uint16_t) as libc::c_int
            == 0 as libc::c_int
        {
            begin = (be16_to_cpu(*(u.short_offsets).offset(index as isize))
                as libc::c_int * 2 as libc::c_int) as libc::c_ulong;
            end = (be16_to_cpu(
                *(u.short_offsets)
                    .offset(
                        index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            ) as libc::c_int * 2 as libc::c_int) as libc::c_ulong;
        } else {
            begin = be32_to_cpu(*(u.long_offsets).offset(index as isize))
                as libc::c_ulong;
            end = be32_to_cpu(
                *(u.long_offsets)
                    .offset(
                        index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            ) as libc::c_ulong;
        }
        if end < begin {
            status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
            current_block = 12990696227398993977;
            break;
        } else {
            size = end.wrapping_sub(begin);
            status = cairo_truetype_font_align_output(font, &mut next);
            if status as u64 != 0 {
                current_block = 12990696227398993977;
                break;
            }
            status = cairo_truetype_font_check_boundary(font, next);
            if status as u64 != 0 {
                current_block = 12990696227398993977;
                break;
            }
            (*((*font).glyphs).offset(i as isize))
                .location = next.wrapping_sub(start_offset);
            status = cairo_truetype_font_allocate_write_buffer(font, size, &mut buffer);
            if status as u64 != 0 {
                current_block = 12990696227398993977;
                break;
            }
            if size > 1 as libc::c_int as libc::c_ulong {
                let mut glyph_data: *mut tt_glyph_data_t = 0 as *mut tt_glyph_data_t;
                let mut num_contours: libc::c_int = 0;
                status = ((*(*font).backend).load_truetype_table)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
                    (('g' as i32 as uint32_t) << 24 as libc::c_int
                        | (('l' as i32) << 16 as libc::c_int) as libc::c_uint
                        | (('y' as i32) << 8 as libc::c_int) as libc::c_uint
                        | 'f' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
                    begin as libc::c_long,
                    buffer,
                    &mut size,
                ) as cairo_status_t;
                if status as u64 != 0 {
                    current_block = 12990696227398993977;
                    break;
                }
                glyph_data = buffer as *mut tt_glyph_data_t;
                num_contours = be16_to_cpu((*glyph_data).num_contours as uint16_t)
                    as int16_t as libc::c_int;
                if num_contours < 0 as libc::c_int {
                    status = cairo_truetype_font_remap_composite_glyph(
                        font,
                        buffer,
                        size,
                    );
                    if status as u64 != 0 {
                        current_block = 12990696227398993977;
                        break;
                    }
                } else if num_contours == 0 as libc::c_int {
                    _cairo_array_truncate(
                        &mut (*font).output,
                        (_cairo_array_num_elements(&mut (*font).output) as libc::c_ulong)
                            .wrapping_sub(size) as libc::c_uint,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    }
    match current_block {
        7990025728955927862 => {
            status = cairo_truetype_font_align_output(font, &mut next);
            if !(status as u64 != 0) {
                (*((*font).glyphs).offset(i as isize))
                    .location = next.wrapping_sub(start_offset);
                status = (*font).status;
            }
        }
        _ => {}
    }
    free(u.bytes as *mut libc::c_void);
    return _cairo_truetype_font_set_error(font, status);
}
unsafe extern "C" fn cairo_truetype_font_write_head_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_ulong = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        tag,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    (*font)
        .checksum_index = (_cairo_array_num_elements(&mut (*font).output))
        .wrapping_add(8 as libc::c_int as libc::c_uint) as libc::c_int;
    status = cairo_truetype_font_allocate_write_buffer(font, size, &mut buffer);
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        tag,
        0 as libc::c_int as libc::c_long,
        buffer,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    *(buffer.offset(8 as libc::c_int as isize)
        as *mut uint32_t) = 0 as libc::c_int as uint32_t;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_write_hhea_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut hhea: *mut tt_hhea_t = 0 as *mut tt_hhea_t;
    let mut size: libc::c_ulong = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = ::std::mem::size_of::<tt_hhea_t>() as libc::c_ulong;
    status = cairo_truetype_font_allocate_write_buffer(
        font,
        size,
        &mut hhea as *mut *mut tt_hhea_t as *mut *mut libc::c_uchar,
    );
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        tag,
        0 as libc::c_int as libc::c_long,
        hhea as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    (*hhea).num_hmetrics = cpu_to_be16((*font).num_glyphs as uint16_t);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_write_hmtx_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut size: libc::c_ulong = 0;
    let mut long_entry_size: libc::c_ulong = 0;
    let mut short_entry_size: libc::c_ulong = 0;
    let mut p: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut i: libc::c_uint = 0;
    let mut hhea: tt_hhea_t = tt_hhea_t {
        version_1: 0,
        version_2: 0,
        ascender: 0,
        descender: 0,
        line_gap: 0,
        advance_max_width: 0,
        min_left_side_bearing: 0,
        min_right_side_bearing: 0,
        x_max_extent: 0,
        caret_slope_rise: 0,
        caret_slope_run: 0,
        reserved: [0; 5],
        metric_data_format: 0,
        num_hmetrics: 0,
    };
    let mut num_hmetrics: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = ::std::mem::size_of::<tt_hhea_t>() as libc::c_ulong;
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('h' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('e' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'a' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut hhea as *mut tt_hhea_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    num_hmetrics = be16_to_cpu(hhea.num_hmetrics) as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font).num_glyphs {
        long_entry_size = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int16_t>() as libc::c_ulong);
        short_entry_size = ::std::mem::size_of::<int16_t>() as libc::c_ulong;
        status = cairo_truetype_font_allocate_write_buffer(
            font,
            long_entry_size,
            &mut p as *mut *mut libc::c_short as *mut *mut libc::c_uchar,
        );
        if status as u64 != 0 {
            return _cairo_truetype_font_set_error(font, status);
        }
        if (*((*font).glyphs).offset(i as isize)).parent_index < num_hmetrics {
            status = ((*(*font).backend).load_truetype_table)
                .expect(
                    "non-null function pointer",
                )(
                (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
                (('h' as i32 as uint32_t) << 24 as libc::c_int
                    | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
                    | 'x' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
                ((*((*font).glyphs).offset(i as isize)).parent_index as libc::c_ulong)
                    .wrapping_mul(long_entry_size) as libc::c_long,
                p as *mut libc::c_uchar,
                &mut long_entry_size,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return _cairo_truetype_font_set_error(font, status);
            }
        } else {
            status = ((*(*font).backend).load_truetype_table)
                .expect(
                    "non-null function pointer",
                )(
                (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
                (('h' as i32 as uint32_t) << 24 as libc::c_int
                    | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
                    | 'x' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
                ((num_hmetrics - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(long_entry_size) as libc::c_long,
                p as *mut libc::c_uchar,
                &mut short_entry_size,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return _cairo_truetype_font_set_error(font, status);
            }
            status = ((*(*font).backend).load_truetype_table)
                .expect(
                    "non-null function pointer",
                )(
                (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
                (('h' as i32 as uint32_t) << 24 as libc::c_int
                    | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
                    | 'x' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
                (num_hmetrics as libc::c_ulong)
                    .wrapping_mul(long_entry_size)
                    .wrapping_add(
                        (((*((*font).glyphs).offset(i as isize)).parent_index
                            - num_hmetrics) as libc::c_ulong)
                            .wrapping_mul(short_entry_size),
                    ) as libc::c_long,
                p.offset(1 as libc::c_int as isize) as *mut libc::c_uchar,
                &mut short_entry_size,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return _cairo_truetype_font_set_error(font, status);
            }
        }
        *((*font).widths)
            .offset(
                i as isize,
            ) = be16_to_cpu(*p.offset(0 as libc::c_int as isize) as uint16_t)
            as libc::c_int;
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_write_loca_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut i: libc::c_uint = 0;
    let mut header: tt_head_t = tt_head_t {
        version_1: 0,
        version_2: 0,
        revision_1: 0,
        revision_2: 0,
        checksum_1: 0,
        checksum_2: 0,
        magic_1: 0,
        magic_2: 0,
        flags: 0,
        units_per_em: 0,
        created_1: 0,
        created_2: 0,
        created_3: 0,
        created_4: 0,
        modified_1: 0,
        modified_2: 0,
        modified_3: 0,
        modified_4: 0,
        x_min: 0,
        y_min: 0,
        x_max: 0,
        y_max: 0,
        mac_style: 0,
        lowest_rec_pppem: 0,
        font_direction_hint: 0,
        index_to_loc_format: 0,
        glyph_data_format: 0,
    };
    let mut size: libc::c_ulong = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = ::std::mem::size_of::<tt_head_t>() as libc::c_ulong;
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('e' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'd' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut header as *mut tt_head_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    if be16_to_cpu(header.index_to_loc_format as uint16_t) as libc::c_int
        == 0 as libc::c_int
    {
        i = 0 as libc::c_int as libc::c_uint;
        while i < ((*font).num_glyphs).wrapping_add(1 as libc::c_int as libc::c_uint) {
            cairo_truetype_font_write_be16(
                font,
                ((*((*font).glyphs).offset(i as isize)).location)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong) as uint16_t,
            );
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < ((*font).num_glyphs).wrapping_add(1 as libc::c_int as libc::c_uint) {
            cairo_truetype_font_write_be32(
                font,
                (*((*font).glyphs).offset(i as isize)).location as uint32_t,
            );
            i = i.wrapping_add(1);
        }
    }
    return (*font).status;
}
unsafe extern "C" fn cairo_truetype_font_write_maxp_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
) -> cairo_status_t {
    let mut maxp: *mut tt_maxp_t = 0 as *mut tt_maxp_t;
    let mut size: libc::c_ulong = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    size = ::std::mem::size_of::<tt_maxp_t>() as libc::c_ulong;
    status = cairo_truetype_font_allocate_write_buffer(
        font,
        size,
        &mut maxp as *mut *mut tt_maxp_t as *mut *mut libc::c_uchar,
    );
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        tag,
        0 as libc::c_int as libc::c_long,
        maxp as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    (*maxp).num_glyphs = cpu_to_be16((*font).num_glyphs as uint16_t);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_write_offset_table(
    mut font: *mut cairo_truetype_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut table_buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut table_buffer_length: size_t = 0;
    let mut search_range: libc::c_ushort = 0;
    let mut entry_selector: libc::c_ushort = 0;
    let mut range_shift: libc::c_ushort = 0;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    search_range = 1 as libc::c_int as libc::c_ushort;
    entry_selector = 0 as libc::c_int as libc::c_ushort;
    while search_range as libc::c_int * 2 as libc::c_int <= (*font).num_tables {
        search_range = (search_range as libc::c_int * 2 as libc::c_int)
            as libc::c_ushort;
        entry_selector = entry_selector.wrapping_add(1);
    }
    search_range = (search_range as libc::c_int * 16 as libc::c_int) as libc::c_ushort;
    range_shift = ((*font).num_tables * 16 as libc::c_int - search_range as libc::c_int)
        as libc::c_ushort;
    cairo_truetype_font_write_be32(font, 0x10000 as libc::c_int as uint32_t);
    cairo_truetype_font_write_be16(font, (*font).num_tables as uint16_t);
    cairo_truetype_font_write_be16(font, search_range);
    cairo_truetype_font_write_be16(font, entry_selector);
    cairo_truetype_font_write_be16(font, range_shift);
    table_buffer_length = ((*font).num_tables * 16 as libc::c_int) as size_t;
    status = cairo_truetype_font_allocate_write_buffer(
        font,
        table_buffer_length,
        &mut table_buffer,
    );
    if status as u64 != 0 {
        return _cairo_truetype_font_set_error(font, status);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_calculate_checksum(
    mut font: *mut cairo_truetype_font_t,
    mut start: libc::c_ulong,
    mut end: libc::c_ulong,
) -> uint32_t {
    let mut padded_end: *mut uint32_t = 0 as *mut uint32_t;
    let mut p: *mut uint32_t = 0 as *mut uint32_t;
    let mut checksum: uint32_t = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    checksum = 0 as libc::c_int as uint32_t;
    data = _cairo_array_index(&mut (*font).output, 0 as libc::c_int as libc::c_uint)
        as *mut libc::c_char;
    p = data.offset(start as isize) as *mut uint32_t;
    padded_end = data
        .offset(
            (end.wrapping_add(3 as libc::c_int as libc::c_ulong)
                & !(3 as libc::c_int) as libc::c_ulong) as isize,
        ) as *mut uint32_t;
    while p < padded_end {
        let fresh13 = p;
        p = p.offset(1);
        checksum = (checksum as libc::c_uint).wrapping_add(be32_to_cpu(*fresh13))
            as uint32_t as uint32_t;
    }
    return checksum;
}
unsafe extern "C" fn cairo_truetype_font_update_entry(
    mut font: *mut cairo_truetype_font_t,
    mut index: libc::c_int,
    mut tag: libc::c_ulong,
    mut start: libc::c_ulong,
    mut end: libc::c_ulong,
) {
    let mut entry: *mut uint32_t = 0 as *mut uint32_t;
    entry = _cairo_array_index(
        &mut (*font).output,
        (12 as libc::c_int + 16 as libc::c_int * index) as libc::c_uint,
    ) as *mut uint32_t;
    *entry.offset(0 as libc::c_int as isize) = cpu_to_be32(tag as uint32_t);
    *entry
        .offset(
            1 as libc::c_int as isize,
        ) = cpu_to_be32(cairo_truetype_font_calculate_checksum(font, start, end));
    *entry.offset(2 as libc::c_int as isize) = cpu_to_be32(start as uint32_t);
    *entry
        .offset(
            3 as libc::c_int as isize,
        ) = cpu_to_be32(end.wrapping_sub(start) as uint32_t);
}
unsafe extern "C" fn cairo_truetype_font_generate(
    mut font: *mut cairo_truetype_font_t,
    mut data: *mut *const libc::c_char,
    mut length: *mut libc::c_ulong,
    mut string_offsets: *mut *const libc::c_ulong,
    mut num_strings: *mut libc::c_ulong,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut start: libc::c_ulong = 0;
    let mut end: libc::c_ulong = 0;
    let mut next: libc::c_ulong = 0;
    let mut checksum: uint32_t = 0;
    let mut checksum_location: *mut uint32_t = 0 as *mut uint32_t;
    let mut i: libc::c_int = 0;
    if (*font).status as u64 != 0 {
        return (*font).status;
    }
    status = cairo_truetype_font_write_offset_table(font);
    if !(status as u64 != 0) {
        status = cairo_truetype_font_align_output(font, &mut start);
        if !(status as u64 != 0) {
            end = 0 as libc::c_int as libc::c_ulong;
            i = 0 as libc::c_int;
            loop {
                if !(i < (*font).num_tables) {
                    current_block = 2838571290723028321;
                    break;
                }
                status = ((*font).truetype_tables[i as usize].write)
                    .expect(
                        "non-null function pointer",
                    )(font, (*font).truetype_tables[i as usize].tag);
                if status as u64 != 0 {
                    current_block = 11939525241834974512;
                    break;
                }
                end = _cairo_array_num_elements(&mut (*font).output) as libc::c_ulong;
                status = cairo_truetype_font_align_output(font, &mut next);
                if status as u64 != 0 {
                    current_block = 11939525241834974512;
                    break;
                }
                cairo_truetype_font_update_entry(
                    font,
                    (*font).truetype_tables[i as usize].pos,
                    (*font).truetype_tables[i as usize].tag,
                    start,
                    end,
                );
                status = cairo_truetype_font_check_boundary(font, next);
                if status as u64 != 0 {
                    current_block = 11939525241834974512;
                    break;
                }
                start = next;
                i += 1;
            }
            match current_block {
                11939525241834974512 => {}
                _ => {
                    checksum = (0xb1b0afba as libc::c_uint)
                        .wrapping_sub(
                            cairo_truetype_font_calculate_checksum(
                                font,
                                0 as libc::c_int as libc::c_ulong,
                                end,
                            ),
                        );
                    checksum_location = _cairo_array_index(
                        &mut (*font).output,
                        (*font).checksum_index as libc::c_uint,
                    ) as *mut uint32_t;
                    *checksum_location = cpu_to_be32(checksum);
                    *data = _cairo_array_index(
                        &mut (*font).output,
                        0 as libc::c_int as libc::c_uint,
                    ) as *const libc::c_char;
                    *length = _cairo_array_num_elements(&mut (*font).output)
                        as libc::c_ulong;
                    *num_strings = _cairo_array_num_elements(&mut (*font).string_offsets)
                        as libc::c_ulong;
                    if *num_strings != 0 as libc::c_int as libc::c_ulong {
                        *string_offsets = _cairo_array_index(
                            &mut (*font).string_offsets,
                            0 as libc::c_int as libc::c_uint,
                        ) as *const libc::c_ulong;
                    } else {
                        *string_offsets = 0 as *const libc::c_ulong;
                    }
                }
            }
        }
    }
    return _cairo_truetype_font_set_error(font, status);
}
unsafe extern "C" fn cairo_truetype_font_use_glyph(
    mut font: *mut cairo_truetype_font_t,
    mut glyph: libc::c_ushort,
    mut out: *mut libc::c_ushort,
) -> cairo_status_t {
    if glyph as libc::c_int >= (*font).base.num_glyphs_in_face {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if *((*font).parent_to_subset).offset(glyph as isize) == 0 as libc::c_int {
        *((*font).parent_to_subset)
            .offset(glyph as isize) = (*font).num_glyphs as libc::c_int;
        (*((*font).glyphs).offset((*font).num_glyphs as isize))
            .parent_index = glyph as libc::c_int;
        let ref mut fresh14 = (*font).num_glyphs;
        *fresh14 = (*fresh14).wrapping_add(1);
    }
    *out = *((*font).parent_to_subset).offset(glyph as isize) as libc::c_ushort;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_truetype_font_add_truetype_table(
    mut font: *mut cairo_truetype_font_t,
    mut tag: libc::c_ulong,
    mut write: Option::<
        unsafe extern "C" fn(*mut cairo_truetype_font_t, libc::c_ulong) -> cairo_status_t,
    >,
    mut pos: libc::c_int,
) {
    (*font).truetype_tables[(*font).num_tables as usize].tag = tag;
    let ref mut fresh15 = (*font).truetype_tables[(*font).num_tables as usize].write;
    *fresh15 = write;
    (*font).truetype_tables[(*font).num_tables as usize].pos = pos;
    let ref mut fresh16 = (*font).num_tables;
    *fresh16 += 1;
}
unsafe extern "C" fn cairo_truetype_font_create_truetype_table_list(
    mut font: *mut cairo_truetype_font_t,
) {
    let mut has_cvt: cairo_bool_t = 0 as libc::c_int;
    let mut has_fpgm: cairo_bool_t = 0 as libc::c_int;
    let mut has_prep: cairo_bool_t = 0 as libc::c_int;
    let mut size: libc::c_ulong = 0;
    let mut pos: libc::c_int = 0;
    size = 0 as libc::c_int as libc::c_ulong;
    if ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('c' as i32 as uint32_t) << 24 as libc::c_int
            | (('v' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
            | ' ' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        has_cvt = 1 as libc::c_int;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    if ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('f' as i32 as uint32_t) << 24 as libc::c_int
            | (('p' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('g' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'm' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        has_fpgm = 1 as libc::c_int;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    if ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('p' as i32 as uint32_t) << 24 as libc::c_int
            | (('r' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('e' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        has_prep = 1 as libc::c_int;
    }
    (*font).num_tables = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    if (*font).is_pdf != 0 && (*(*font).scaled_font_subset).is_latin != 0 {
        pos += 1;
    }
    if has_cvt != 0 {
        pos += 1;
    }
    if has_fpgm != 0 {
        pos += 1;
    }
    cairo_truetype_font_add_truetype_table(
        font,
        (('g' as i32 as uint32_t) << 24 as libc::c_int
            | (('l' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('y' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'f' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        Some(
            cairo_truetype_font_write_glyf_table
                as unsafe extern "C" fn(
                    *mut cairo_truetype_font_t,
                    libc::c_ulong,
                ) -> cairo_status_t,
        ),
        pos,
    );
    pos = 0 as libc::c_int;
    if (*font).is_pdf != 0 && (*(*font).scaled_font_subset).is_latin != 0 {
        let fresh17 = pos;
        pos = pos + 1;
        cairo_truetype_font_add_truetype_table(
            font,
            (('c' as i32 as uint32_t) << 24 as libc::c_int
                | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
                | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
                | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
            Some(
                cairo_truetype_font_write_cmap_table
                    as unsafe extern "C" fn(
                        *mut cairo_truetype_font_t,
                        libc::c_ulong,
                    ) -> cairo_status_t,
            ),
            fresh17,
        );
    }
    if has_cvt != 0 {
        let fresh18 = pos;
        pos = pos + 1;
        cairo_truetype_font_add_truetype_table(
            font,
            (('c' as i32 as uint32_t) << 24 as libc::c_int
                | (('v' as i32) << 16 as libc::c_int) as libc::c_uint
                | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
                | ' ' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
            Some(
                cairo_truetype_font_write_generic_table
                    as unsafe extern "C" fn(
                        *mut cairo_truetype_font_t,
                        libc::c_ulong,
                    ) -> cairo_status_t,
            ),
            fresh18,
        );
    }
    if has_fpgm != 0 {
        let fresh19 = pos;
        pos = pos + 1;
        cairo_truetype_font_add_truetype_table(
            font,
            (('f' as i32 as uint32_t) << 24 as libc::c_int
                | (('p' as i32) << 16 as libc::c_int) as libc::c_uint
                | (('g' as i32) << 8 as libc::c_int) as libc::c_uint
                | 'm' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
            Some(
                cairo_truetype_font_write_generic_table
                    as unsafe extern "C" fn(
                        *mut cairo_truetype_font_t,
                        libc::c_ulong,
                    ) -> cairo_status_t,
            ),
            fresh19,
        );
    }
    pos += 1;
    let fresh20 = pos;
    pos = pos + 1;
    cairo_truetype_font_add_truetype_table(
        font,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('e' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'd' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        Some(
            cairo_truetype_font_write_head_table
                as unsafe extern "C" fn(
                    *mut cairo_truetype_font_t,
                    libc::c_ulong,
                ) -> cairo_status_t,
        ),
        fresh20,
    );
    let fresh21 = pos;
    pos = pos + 1;
    cairo_truetype_font_add_truetype_table(
        font,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('h' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('e' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'a' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        Some(
            cairo_truetype_font_write_hhea_table
                as unsafe extern "C" fn(
                    *mut cairo_truetype_font_t,
                    libc::c_ulong,
                ) -> cairo_status_t,
        ),
        fresh21,
    );
    let fresh22 = pos;
    pos = pos + 1;
    cairo_truetype_font_add_truetype_table(
        font,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'x' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        Some(
            cairo_truetype_font_write_hmtx_table
                as unsafe extern "C" fn(
                    *mut cairo_truetype_font_t,
                    libc::c_ulong,
                ) -> cairo_status_t,
        ),
        fresh22,
    );
    let fresh23 = pos;
    pos = pos + 1;
    cairo_truetype_font_add_truetype_table(
        font,
        (('l' as i32 as uint32_t) << 24 as libc::c_int
            | (('o' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('c' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'a' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        Some(
            cairo_truetype_font_write_loca_table
                as unsafe extern "C" fn(
                    *mut cairo_truetype_font_t,
                    libc::c_ulong,
                ) -> cairo_status_t,
        ),
        fresh23,
    );
    let fresh24 = pos;
    pos = pos + 1;
    cairo_truetype_font_add_truetype_table(
        font,
        (('m' as i32 as uint32_t) << 24 as libc::c_int
            | (('a' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('x' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        Some(
            cairo_truetype_font_write_maxp_table
                as unsafe extern "C" fn(
                    *mut cairo_truetype_font_t,
                    libc::c_ulong,
                ) -> cairo_status_t,
        ),
        fresh24,
    );
    if has_prep != 0 {
        cairo_truetype_font_add_truetype_table(
            font,
            (('p' as i32 as uint32_t) << 24 as libc::c_int
                | (('r' as i32) << 16 as libc::c_int) as libc::c_uint
                | (('e' as i32) << 8 as libc::c_int) as libc::c_uint
                | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
            Some(
                cairo_truetype_font_write_generic_table
                    as unsafe extern "C" fn(
                        *mut cairo_truetype_font_t,
                        libc::c_ulong,
                    ) -> cairo_status_t,
            ),
            pos,
        );
    }
}
unsafe extern "C" fn cairo_truetype_subset_init_internal(
    mut truetype_subset: *mut cairo_truetype_subset_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut is_pdf: cairo_bool_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut font: *mut cairo_truetype_font_t = 0 as *mut cairo_truetype_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut offsets_length: libc::c_ulong = 0;
    let mut i: libc::c_uint = 0;
    let mut string_offsets: *const libc::c_ulong = 0 as *const libc::c_ulong;
    let mut num_strings: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    status = _cairo_truetype_font_create(font_subset, is_pdf, &mut font);
    if status as u64 != 0 {
        return status;
    }
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        if !(i < (*(*font).scaled_font_subset).num_glyphs) {
            current_block = 1841672684692190573;
            break;
        }
        let mut parent_glyph: libc::c_ushort = *((*(*font).scaled_font_subset).glyphs)
            .offset(i as isize) as libc::c_ushort;
        status = cairo_truetype_font_use_glyph(font, parent_glyph, &mut parent_glyph);
        if status as u64 != 0 {
            current_block = 16647905829462691112;
            break;
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        1841672684692190573 => {
            cairo_truetype_font_create_truetype_table_list(font);
            status = cairo_truetype_font_generate(
                font,
                &mut data,
                &mut length,
                &mut string_offsets,
                &mut num_strings,
            );
            if !(status as u64 != 0) {
                let ref mut fresh25 = (*truetype_subset).ps_name;
                *fresh25 = strdup((*font).base.ps_name);
                if ((*truetype_subset).ps_name).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                } else {
                    if !((*font).base.font_name).is_null() {
                        let ref mut fresh26 = (*truetype_subset).family_name_utf8;
                        *fresh26 = strdup((*font).base.font_name);
                        if ((*truetype_subset).family_name_utf8).is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                            current_block = 11818362928757487149;
                        } else {
                            current_block = 16203760046146113240;
                        }
                    } else {
                        let ref mut fresh27 = (*truetype_subset).family_name_utf8;
                        *fresh27 = 0 as *mut libc::c_char;
                        current_block = 16203760046146113240;
                    }
                    match current_block {
                        16203760046146113240 => {
                            let ref mut fresh28 = (*truetype_subset).widths;
                            *fresh28 = calloc(
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
                            ) as *mut libc::c_double;
                            if ((*truetype_subset).widths).is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                            } else {
                                i = 0 as libc::c_int as libc::c_uint;
                                while i < (*(*font).scaled_font_subset).num_glyphs {
                                    *((*truetype_subset).widths)
                                        .offset(
                                            i as isize,
                                        ) = *((*font).widths).offset(i as isize) as libc::c_double
                                        / (*font).base.units_per_em as libc::c_double;
                                    i = i.wrapping_add(1);
                                }
                                (*truetype_subset)
                                    .x_min = (*font).base.x_min as libc::c_double
                                    / (*font).base.units_per_em as libc::c_double;
                                (*truetype_subset)
                                    .y_min = (*font).base.y_min as libc::c_double
                                    / (*font).base.units_per_em as libc::c_double;
                                (*truetype_subset)
                                    .x_max = (*font).base.x_max as libc::c_double
                                    / (*font).base.units_per_em as libc::c_double;
                                (*truetype_subset)
                                    .y_max = (*font).base.y_max as libc::c_double
                                    / (*font).base.units_per_em as libc::c_double;
                                (*truetype_subset)
                                    .ascent = (*font).base.ascent as libc::c_double
                                    / (*font).base.units_per_em as libc::c_double;
                                (*truetype_subset)
                                    .descent = (*font).base.descent as libc::c_double
                                    / (*font).base.units_per_em as libc::c_double;
                                if length != 0 {
                                    let ref mut fresh29 = (*truetype_subset).data;
                                    *fresh29 = (if length != 0 as libc::c_int as libc::c_ulong {
                                        malloc(length)
                                    } else {
                                        0 as *mut libc::c_void
                                    }) as *mut libc::c_uchar;
                                    if ((*truetype_subset).data).is_null() {
                                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                        current_block = 13498252046719513127;
                                    } else {
                                        memcpy(
                                            (*truetype_subset).data as *mut libc::c_void,
                                            data as *const libc::c_void,
                                            length,
                                        );
                                        current_block = 2543120759711851213;
                                    }
                                } else {
                                    let ref mut fresh30 = (*truetype_subset).data;
                                    *fresh30 = 0 as *mut libc::c_uchar;
                                    current_block = 2543120759711851213;
                                }
                                match current_block {
                                    2543120759711851213 => {
                                        (*truetype_subset).data_length = length;
                                        if num_strings != 0 {
                                            offsets_length = num_strings
                                                .wrapping_mul(
                                                    ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                                                );
                                            let ref mut fresh31 = (*truetype_subset).string_offsets;
                                            *fresh31 = (if offsets_length
                                                != 0 as libc::c_int as libc::c_ulong
                                            {
                                                malloc(offsets_length)
                                            } else {
                                                0 as *mut libc::c_void
                                            }) as *mut libc::c_ulong;
                                            if ((*truetype_subset).string_offsets).is_null() {
                                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                                free((*truetype_subset).data as *mut libc::c_void);
                                                current_block = 13498252046719513127;
                                            } else {
                                                memcpy(
                                                    (*truetype_subset).string_offsets as *mut libc::c_void,
                                                    string_offsets as *const libc::c_void,
                                                    offsets_length,
                                                );
                                                (*truetype_subset).num_string_offsets = num_strings;
                                                current_block = 13281731871476506071;
                                            }
                                        } else {
                                            let ref mut fresh32 = (*truetype_subset).string_offsets;
                                            *fresh32 = 0 as *mut libc::c_ulong;
                                            (*truetype_subset)
                                                .num_string_offsets = 0 as libc::c_int as libc::c_ulong;
                                            current_block = 13281731871476506071;
                                        }
                                        match current_block {
                                            13498252046719513127 => {}
                                            _ => {
                                                cairo_truetype_font_destroy(font);
                                                return CAIRO_STATUS_SUCCESS;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                free((*truetype_subset).widths as *mut libc::c_void);
                            }
                            free(
                                (*truetype_subset).family_name_utf8 as *mut libc::c_void,
                            );
                        }
                        _ => {}
                    }
                    free((*truetype_subset).ps_name as *mut libc::c_void);
                }
            }
        }
        _ => {}
    }
    cairo_truetype_font_destroy(font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_truetype_subset_init_ps(
    mut truetype_subset: *mut cairo_truetype_subset_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    return cairo_truetype_subset_init_internal(
        truetype_subset,
        font_subset,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_truetype_subset_init_pdf(
    mut truetype_subset: *mut cairo_truetype_subset_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    return cairo_truetype_subset_init_internal(
        truetype_subset,
        font_subset,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_truetype_subset_fini(
    mut subset: *mut cairo_truetype_subset_t,
) {
    free((*subset).ps_name as *mut libc::c_void);
    free((*subset).family_name_utf8 as *mut libc::c_void);
    free((*subset).widths as *mut libc::c_void);
    free((*subset).data as *mut libc::c_void);
    free((*subset).string_offsets as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_truetype_reverse_cmap(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut table_offset: libc::c_ulong,
    mut index: libc::c_ulong,
    mut ucs4: *mut uint32_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut map: *mut tt_segment_map_t = 0 as *mut tt_segment_map_t;
    let mut map_header: tt_segment_map_t = tt_segment_map_t {
        format: 0,
        length: 0,
        version: 0,
        segCountX2: 0,
        searchRange: 0,
        entrySelector: 0,
        rangeShift: 0,
        endCount: [0; 1],
    };
    let mut num_segments: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut size: libc::c_ulong = 0;
    let mut start_code: *mut uint16_t = 0 as *mut uint16_t;
    let mut end_code: *mut uint16_t = 0 as *mut uint16_t;
    let mut delta: *mut uint16_t = 0 as *mut uint16_t;
    let mut range_offset: *mut uint16_t = 0 as *mut uint16_t;
    let mut c: uint16_t = 0;
    backend = (*scaled_font).backend;
    size = 4 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('c' as i32 as uint32_t) << 24 as libc::c_int
            | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        table_offset as libc::c_long,
        &mut map_header as *mut tt_segment_map_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if be16_to_cpu(map_header.format) as libc::c_int != 4 as libc::c_int {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    size = be16_to_cpu(map_header.length) as libc::c_ulong;
    if size < 24 as libc::c_int as libc::c_ulong {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    map = (if size != 0 as libc::c_int as libc::c_ulong {
        malloc(size)
    } else {
        0 as *mut libc::c_void
    }) as *mut tt_segment_map_t;
    if map.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('c' as i32 as uint32_t) << 24 as libc::c_int
            | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        table_offset as libc::c_long,
        map as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if !(status as u64 != 0) {
        num_segments = (be16_to_cpu((*map).segCountX2) as libc::c_int / 2 as libc::c_int)
            as libc::c_uint;
        if size
            < ((8 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (4 as libc::c_int as libc::c_uint).wrapping_mul(num_segments),
                ) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        end_code = ((*map).endCount).as_mut_ptr();
        start_code = &mut *end_code
            .offset(num_segments.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
            as *mut uint16_t;
        delta = &mut *start_code.offset(num_segments as isize) as *mut uint16_t;
        range_offset = &mut *delta.offset(num_segments as isize) as *mut uint16_t;
        i = 0 as libc::c_int as libc::c_uint;
        loop {
            if !(i < num_segments) {
                current_block = 8704759739624374314;
                break;
            }
            let mut start: uint16_t = be16_to_cpu(*start_code.offset(i as isize));
            let mut end: uint16_t = be16_to_cpu(*end_code.offset(i as isize));
            if start as libc::c_int == 0xffff as libc::c_int
                && end as libc::c_int == 0xffff as libc::c_int
            {
                current_block = 8704759739624374314;
                break;
            }
            c = index
                .wrapping_sub(be16_to_cpu(*delta.offset(i as isize)) as libc::c_ulong)
                as uint16_t;
            if *range_offset.offset(i as isize) as libc::c_int == 0 as libc::c_int
                && c as libc::c_int >= start as libc::c_int
                && c as libc::c_int <= end as libc::c_int
            {
                *ucs4 = c as uint32_t;
                current_block = 6025957328182792992;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        match current_block {
            8704759739624374314 => {
                i = 0 as libc::c_int as libc::c_uint;
                's_169: loop {
                    if !(i < num_segments) {
                        current_block = 11793792312832361944;
                        break;
                    }
                    let mut start_0: uint16_t = be16_to_cpu(
                        *start_code.offset(i as isize),
                    );
                    let mut end_0: uint16_t = be16_to_cpu(*end_code.offset(i as isize));
                    if start_0 as libc::c_int == 0xffff as libc::c_int
                        && end_0 as libc::c_int == 0xffff as libc::c_int
                    {
                        current_block = 11793792312832361944;
                        break;
                    }
                    if *range_offset.offset(i as isize) as libc::c_int
                        != 0 as libc::c_int
                    {
                        let mut glyph_ids: *mut uint16_t = (&mut *range_offset
                            .offset(i as isize) as *mut uint16_t)
                            .offset(
                                (be16_to_cpu(*range_offset.offset(i as isize))
                                    as libc::c_int / 2 as libc::c_int) as isize,
                            );
                        let mut range_size: libc::c_int = end_0 as libc::c_int
                            - start_0 as libc::c_int + 1 as libc::c_int;
                        let mut g_id_be: uint16_t = cpu_to_be16(index as uint16_t);
                        let mut j: libc::c_int = 0;
                        if range_size > 0 as libc::c_int {
                            if (glyph_ids as *mut libc::c_char)
                                .offset((2 as libc::c_int * range_size) as isize)
                                > (map as *mut libc::c_char).offset(size as isize)
                            {
                                return CAIRO_INT_STATUS_UNSUPPORTED;
                            }
                            j = 0 as libc::c_int;
                            while j < range_size {
                                if *glyph_ids.offset(j as isize) as libc::c_int
                                    == g_id_be as libc::c_int
                                {
                                    *ucs4 = (start_0 as libc::c_int + j) as uint32_t;
                                    current_block = 6025957328182792992;
                                    break 's_169;
                                } else {
                                    j += 1;
                                }
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
                match current_block {
                    6025957328182792992 => {}
                    _ => {
                        *ucs4 = -(1 as libc::c_int) as uint32_t;
                    }
                }
            }
            _ => {}
        }
        status = CAIRO_STATUS_SUCCESS;
    }
    free(map as *mut libc::c_void);
    return status as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_truetype_index_to_ucs4(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut index: libc::c_ulong,
    mut ucs4: *mut uint32_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_UNSUPPORTED;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut cmap: *mut tt_cmap_t = 0 as *mut tt_cmap_t;
    let mut cmap_header: tt_cmap_t = tt_cmap_t {
        version: 0,
        num_tables: 0,
        index: [tt_cmap_index_t {
            platform: 0,
            encoding: 0,
            offset: 0,
        }; 1],
    };
    let mut num_tables: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_ulong = 0;
    backend = (*scaled_font).backend;
    if ((*backend).load_truetype_table).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    size = 4 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('c' as i32 as uint32_t) << 24 as libc::c_int
            | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut cmap_header as *mut tt_cmap_t as *mut libc::c_uchar,
        &mut size,
    );
    if status as u64 != 0 {
        return status;
    }
    num_tables = be16_to_cpu(cmap_header.num_tables) as libc::c_int;
    size = (4 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (num_tables as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<tt_cmap_index_t>() as libc::c_ulong),
        );
    cmap = _cairo_malloc_ab_plus_c(
        num_tables as size_t,
        ::std::mem::size_of::<tt_cmap_index_t>() as libc::c_ulong,
        4 as libc::c_int as size_t,
    ) as *mut tt_cmap_t;
    if cmap.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('c' as i32 as uint32_t) << 24 as libc::c_int
            | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'p' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        cmap as *mut libc::c_uchar,
        &mut size,
    );
    if !(status as u64 != 0) {
        i = 0 as libc::c_int;
        while i < num_tables {
            if be16_to_cpu((*((*cmap).index).as_mut_ptr().offset(i as isize)).platform)
                as libc::c_int == 3 as libc::c_int
                && be16_to_cpu(
                    (*((*cmap).index).as_mut_ptr().offset(i as isize)).encoding,
                ) as libc::c_int == 1 as libc::c_int
            {
                status = _cairo_truetype_reverse_cmap(
                    scaled_font,
                    be32_to_cpu(
                        (*((*cmap).index).as_mut_ptr().offset(i as isize)).offset,
                    ) as libc::c_ulong,
                    index,
                    ucs4,
                );
                if status as libc::c_uint
                    != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                {
                    break;
                }
            }
            i += 1;
        }
    }
    free(cmap as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn find_name(
    mut name: *mut tt_name_t,
    mut size: libc::c_ulong,
    mut name_id: libc::c_int,
    mut platform: libc::c_int,
    mut encoding: libc::c_int,
    mut language: libc::c_int,
    mut str_out: *mut *mut libc::c_char,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut record: *mut tt_name_record_t = 0 as *mut tt_name_record_t;
    let mut i: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut has_tag: cairo_bool_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    str = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (if (be16_to_cpu((*name).num_records) as libc::c_ulong)
            < size
                .wrapping_div(::std::mem::size_of::<tt_name_record_t>() as libc::c_ulong)
        {
            be16_to_cpu((*name).num_records) as libc::c_ulong
        } else {
            size.wrapping_div(::std::mem::size_of::<tt_name_record_t>() as libc::c_ulong)
        })
    {
        record = &mut *((*name).records).as_mut_ptr().offset(i as isize)
            as *mut tt_name_record_t;
        if be16_to_cpu((*record).name) as libc::c_int == name_id
            && be16_to_cpu((*record).platform) as libc::c_int == platform
            && be16_to_cpu((*record).encoding) as libc::c_int == encoding
            && (language == -(1 as libc::c_int)
                || be16_to_cpu((*record).language) as libc::c_int == language)
        {
            len = be16_to_cpu((*record).length) as libc::c_uint;
            if platform == 3 as libc::c_int
                && len > (127 as libc::c_int * 2 as libc::c_int) as libc::c_uint
            {
                break;
            }
            if len > 127 as libc::c_int as libc::c_uint {
                break;
            }
            let mut offset: uint16_t = (be16_to_cpu((*name).strings_offset)
                as libc::c_int + be16_to_cpu((*record).offset) as libc::c_int)
                as uint16_t;
            if (offset as libc::c_uint).wrapping_add(len) as libc::c_ulong > size {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            str = (if len.wrapping_add(1 as libc::c_int as libc::c_uint)
                != 0 as libc::c_int as libc::c_uint
            {
                malloc(
                    len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
                )
            } else {
                0 as *mut libc::c_void
            }) as *mut libc::c_char;
            if str.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            memcpy(
                str as *mut libc::c_void,
                (name as *mut libc::c_char).offset(offset as libc::c_int as isize)
                    as *const libc::c_void,
                len as libc::c_ulong,
            );
            *str.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if str.is_null() {
        *str_out = 0 as *mut libc::c_char;
        return CAIRO_STATUS_SUCCESS;
    }
    if platform == 3 as libc::c_int {
        let mut size_0: libc::c_int = 0 as libc::c_int;
        let mut utf8: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut u: *mut uint16_t = str as *mut uint16_t;
        let mut u_len: libc::c_uint = len.wrapping_div(2 as libc::c_int as libc::c_uint);
        i = 0 as libc::c_int as libc::c_uint;
        while i < u_len {
            size_0
                += _cairo_ucs4_to_utf8(
                    be16_to_cpu(*u.offset(i as isize)) as uint32_t,
                    0 as *mut libc::c_char,
                );
            i = i.wrapping_add(1);
        }
        utf8 = (if size_0 + 1 as libc::c_int != 0 as libc::c_int {
            malloc((size_0 + 1 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if utf8.is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            current_block = 11720415011864856717;
        } else {
            p = utf8;
            i = 0 as libc::c_int as libc::c_uint;
            while i < u_len {
                p = p
                    .offset(
                        _cairo_ucs4_to_utf8(
                            be16_to_cpu(*u.offset(i as isize)) as uint32_t,
                            p,
                        ) as isize,
                    );
                i = i.wrapping_add(1);
            }
            *p = 0 as libc::c_int as libc::c_char;
            free(str as *mut libc::c_void);
            str = utf8;
            current_block = 12381812505308290051;
        }
    } else {
        if platform == 1 as libc::c_int {
            i = 0 as libc::c_int as libc::c_uint;
            while i < len {
                if *str.offset(i as isize) as libc::c_uchar as libc::c_int
                    > 127 as libc::c_int
                {
                    *str.offset(i as isize) = '_' as i32 as libc::c_char;
                }
                i = i.wrapping_add(1);
            }
        }
        current_block = 12381812505308290051;
    }
    match current_block {
        12381812505308290051 => {
            p = str;
            len = strlen(str) as libc::c_uint;
            has_tag = 0 as libc::c_int;
            if len > 7 as libc::c_int as libc::c_uint
                && *p.offset(6 as libc::c_int as isize) as libc::c_int == '+' as i32
            {
                has_tag = 1 as libc::c_int;
                i = 0 as libc::c_int as libc::c_uint;
                while i < 6 as libc::c_int as libc::c_uint {
                    if (*p.offset(i as isize) as libc::c_int) < 'A' as i32
                        || *p.offset(i as isize) as libc::c_int > 'Z' as i32
                    {
                        has_tag = 0 as libc::c_int;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
            }
            if has_tag != 0 {
                p = (if len.wrapping_sub(6 as libc::c_int as libc::c_uint)
                    != 0 as libc::c_int as libc::c_uint
                {
                    malloc(
                        len.wrapping_sub(6 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    )
                } else {
                    0 as *mut libc::c_void
                }) as *mut libc::c_char;
                if p.is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    current_block = 11720415011864856717;
                } else {
                    memcpy(
                        p as *mut libc::c_void,
                        str.offset(7 as libc::c_int as isize) as *const libc::c_void,
                        len.wrapping_sub(7 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    );
                    *p
                        .offset(
                            len.wrapping_sub(7 as libc::c_int as libc::c_uint) as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                    free(str as *mut libc::c_void);
                    str = p;
                    current_block = 13707613154239713890;
                }
            } else {
                current_block = 13707613154239713890;
            }
            match current_block {
                11720415011864856717 => {}
                _ => {
                    *str_out = str;
                    return CAIRO_STATUS_SUCCESS;
                }
            }
        }
        _ => {}
    }
    free(str as *mut libc::c_void);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_truetype_read_font_name(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut ps_name_out: *mut *mut libc::c_char,
    mut font_name_out: *mut *mut libc::c_char,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut name: *mut tt_name_t = 0 as *mut tt_name_t;
    let mut size: libc::c_ulong = 0;
    let mut ps_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut family_name: *mut libc::c_char = 0 as *mut libc::c_char;
    backend = (*scaled_font).backend;
    if ((*backend).load_truetype_table).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('n' as i32 as uint32_t) << 24 as libc::c_int
            | (('a' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('m' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'e' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    name = (if size != 0 as libc::c_int as libc::c_ulong {
        malloc(size)
    } else {
        0 as *mut libc::c_void
    }) as *mut tt_name_t;
    if name.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('n' as i32 as uint32_t) << 24 as libc::c_int
            | (('a' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('m' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'e' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        name as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if !(status as u64 != 0) {
        status = find_name(
            name,
            size,
            6 as libc::c_int,
            3 as libc::c_int,
            1 as libc::c_int,
            0x409 as libc::c_int,
            &mut ps_name,
        );
        if !(status as u64 != 0) {
            if ps_name.is_null() {
                status = find_name(
                    name,
                    size,
                    6 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    &mut ps_name,
                );
                if status as u64 != 0 {
                    current_block = 4023792313809996375;
                } else {
                    current_block = 2370887241019905314;
                }
            } else {
                current_block = 2370887241019905314;
            }
            match current_block {
                4023792313809996375 => {}
                _ => {
                    status = find_name(
                        name,
                        size,
                        1 as libc::c_int,
                        3 as libc::c_int,
                        1 as libc::c_int,
                        0x409 as libc::c_int,
                        &mut family_name,
                    );
                    if !(status as u64 != 0) {
                        if family_name.is_null() {
                            status = find_name(
                                name,
                                size,
                                1 as libc::c_int,
                                3 as libc::c_int,
                                0 as libc::c_int,
                                0x409 as libc::c_int,
                                &mut family_name,
                            );
                            if status as u64 != 0 {
                                current_block = 4023792313809996375;
                            } else {
                                current_block = 17478428563724192186;
                            }
                        } else {
                            current_block = 17478428563724192186;
                        }
                        match current_block {
                            4023792313809996375 => {}
                            _ => {
                                if family_name.is_null() {
                                    status = find_name(
                                        name,
                                        size,
                                        1 as libc::c_int,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        &mut family_name,
                                    );
                                    if status as u64 != 0 {
                                        current_block = 4023792313809996375;
                                    } else {
                                        current_block = 14648156034262866959;
                                    }
                                } else {
                                    current_block = 14648156034262866959;
                                }
                                match current_block {
                                    4023792313809996375 => {}
                                    _ => {
                                        if family_name.is_null() {
                                            status = find_name(
                                                name,
                                                size,
                                                1 as libc::c_int,
                                                3 as libc::c_int,
                                                1 as libc::c_int,
                                                -(1 as libc::c_int),
                                                &mut family_name,
                                            );
                                            if status as u64 != 0 {
                                                current_block = 4023792313809996375;
                                            } else {
                                                current_block = 5494826135382683477;
                                            }
                                        } else {
                                            current_block = 5494826135382683477;
                                        }
                                        match current_block {
                                            4023792313809996375 => {}
                                            _ => {
                                                status = _cairo_escape_ps_name(&mut ps_name)
                                                    as cairo_status_t;
                                                if !(status as u64 != 0) {
                                                    free(name as *mut libc::c_void);
                                                    *ps_name_out = ps_name;
                                                    *font_name_out = family_name;
                                                    return CAIRO_STATUS_SUCCESS as libc::c_int
                                                        as cairo_int_status_t;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(name as *mut libc::c_void);
    free(ps_name as *mut libc::c_void);
    free(family_name as *mut libc::c_void);
    *ps_name_out = 0 as *mut libc::c_char;
    *font_name_out = 0 as *mut libc::c_char;
    return status as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_truetype_get_style(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut weight: *mut libc::c_int,
    mut bold: *mut cairo_bool_t,
    mut italic: *mut cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut os2: tt_os2_t = tt_os2_t {
        _unused1: [0; 2],
        usWeightClass: 0,
        _unused2: [0; 28],
        fsSelection: 0,
        _unused3: [0; 11],
    };
    let mut size: libc::c_ulong = 0;
    let mut selection: uint16_t = 0;
    backend = (*scaled_font).backend;
    if ((*backend).load_truetype_table).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('O' as i32 as uint32_t) << 24 as libc::c_int
            | (('S' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('/' as i32) << 8 as libc::c_int) as libc::c_uint
            | '2' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if size < ::std::mem::size_of::<tt_os2_t>() as libc::c_ulong {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    size = ::std::mem::size_of::<tt_os2_t>() as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        (('O' as i32 as uint32_t) << 24 as libc::c_int
            | (('S' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('/' as i32) << 8 as libc::c_int) as libc::c_uint
            | '2' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut os2 as *mut tt_os2_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    *weight = be16_to_cpu(os2.usWeightClass) as libc::c_int;
    selection = be16_to_cpu(os2.fsSelection);
    *bold = if selection as libc::c_int & 32 as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    *italic = if selection as libc::c_int & 1 as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
