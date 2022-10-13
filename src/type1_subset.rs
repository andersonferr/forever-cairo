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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_get_locale_decimal_point() -> *const libc::c_char;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_grow_by(
        array: *mut cairo_array_t,
        additional: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
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
    fn _cairo_ps_standard_encoding_to_glyphname(
        glyph: libc::c_int,
    ) -> *const libc::c_char;
    fn _cairo_winansi_to_glyphname(glyph: libc::c_int) -> *const libc::c_char;
    fn _cairo_escape_ps_name(ps_name: *mut *mut libc::c_char) -> cairo_int_status_t;
    fn _cairo_output_stream_create(
        write_func: cairo_write_func_t,
        close_func: cairo_close_func_t,
        closure: *mut libc::c_void,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_destroy(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_write(
        stream: *mut cairo_output_stream_t,
        data: *const libc::c_void,
        length: size_t,
    );
    fn _cairo_output_stream_printf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _cairo_output_stream_get_position(
        stream: *mut cairo_output_stream_t,
    ) -> libc::c_longlong;
}
pub type size_t = libc::c_ulong;
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
pub struct _cairo_type1_subset {
    pub base_font: *mut libc::c_char,
    pub widths: *mut libc::c_double,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub data: *mut libc::c_char,
    pub header_length: libc::c_ulong,
    pub data_length: libc::c_ulong,
    pub trailer_length: libc::c_ulong,
}
pub type cairo_type1_subset_t = _cairo_type1_subset;
pub type cairo_type1_font_subset_t = _cairo_type1_font_subset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_type1_font_subset {
    pub scaled_font_subset: *mut cairo_scaled_font_subset_t,
    pub base: C2RustUnnamed_2,
    pub num_glyphs: libc::c_int,
    pub glyphs_array: cairo_array_t,
    pub glyphs: *mut glyph_data_t,
    pub glyph_names_array: cairo_array_t,
    pub glyph_names: *mut *mut libc::c_char,
    pub num_subrs: libc::c_int,
    pub subset_subrs: cairo_bool_t,
    pub subrs: *mut C2RustUnnamed_1,
    pub scaled_subset_index_to_glyphs: *mut libc::c_int,
    pub type1_subset_index_to_glyphs: *mut libc::c_int,
    pub output: *mut cairo_output_stream_t,
    pub contents: cairo_array_t,
    pub rd: *const libc::c_char,
    pub nd: *const libc::c_char,
    pub np: *const libc::c_char,
    pub lenIV: libc::c_int,
    pub type1_data: *mut libc::c_char,
    pub type1_length: libc::c_uint,
    pub type1_end: *mut libc::c_char,
    pub header_segment: *mut libc::c_char,
    pub header_segment_size: libc::c_uint,
    pub eexec_segment: *mut libc::c_char,
    pub eexec_segment_size: libc::c_uint,
    pub eexec_segment_is_ascii: cairo_bool_t,
    pub cleartext: *mut libc::c_char,
    pub cleartext_end: *mut libc::c_char,
    pub header_size: libc::c_uint,
    pub eexec_key: libc::c_ushort,
    pub hex_encode: cairo_bool_t,
    pub hex_column: libc::c_int,
    pub build_stack: C2RustUnnamed_0,
    pub ps_stack: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub stack: [libc::c_int; 24],
    pub sp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub stack: [libc::c_double; 24],
    pub sp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub subr_string: *const libc::c_char,
    pub subr_length: libc::c_int,
    pub np: *const libc::c_char,
    pub np_length: libc::c_int,
    pub used: cairo_bool_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyph_data_t {
    pub subset_index: libc::c_int,
    pub width: libc::c_double,
    pub encrypted_charstring: *const libc::c_char,
    pub encrypted_charstring_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub font_id: libc::c_uint,
    pub base_font: *mut libc::c_char,
    pub num_glyphs: libc::c_uint,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub units_per_em: libc::c_double,
    pub data: *const libc::c_char,
    pub header_size: libc::c_ulong,
    pub data_size: libc::c_ulong,
    pub trailer_size: libc::c_ulong,
}
pub type glyph_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_type1_font_subset_t,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
    ) -> cairo_status_t,
>;
pub type subr_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_type1_font_subset_t,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
    ) -> cairo_status_t,
>;
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
#[inline]
unsafe extern "C" fn _cairo_isspace(mut c: libc::c_int) -> libc::c_int {
    return (c == 0x20 as libc::c_int
        || c >= 0x9 as libc::c_int && c <= 0xd as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_isdigit(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_isxdigit(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32 || c >= 'a' as i32 && c <= 'f' as i32
        || c >= 'A' as i32 && c <= 'F' as i32) as libc::c_int;
}
unsafe extern "C" fn _cairo_type1_font_subset_init(
    mut font: *mut cairo_type1_font_subset_t,
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut hex_encode: cairo_bool_t,
) -> cairo_status_t {
    memset(
        font as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_type1_font_subset_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*font).scaled_font_subset;
    *fresh0 = scaled_font_subset;
    _cairo_array_init(
        &mut (*font).glyphs_array,
        ::std::mem::size_of::<glyph_data_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*font).glyph_names_array,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh1 = (*font).scaled_subset_index_to_glyphs;
    *fresh1 = calloc(
        (*scaled_font_subset).num_glyphs as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).scaled_subset_index_to_glyphs).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh2 = (*font).type1_subset_index_to_glyphs;
    *fresh2 = 0 as *mut libc::c_int;
    (*font).base.num_glyphs = 0 as libc::c_int as libc::c_uint;
    (*font).num_subrs = 0 as libc::c_int;
    (*font).subset_subrs = 1 as libc::c_int;
    let ref mut fresh3 = (*font).subrs;
    *fresh3 = 0 as *mut C2RustUnnamed_1;
    (*font).hex_encode = hex_encode;
    (*font).num_glyphs = 0 as libc::c_int;
    _cairo_array_init(
        &mut (*font).contents,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_use_glyph(
    mut font: *mut cairo_type1_font_subset_t,
    mut glyph: libc::c_int,
) {
    if (*((*font).glyphs).offset(glyph as isize)).subset_index >= 0 as libc::c_int {
        return;
    }
    (*((*font).glyphs).offset(glyph as isize)).subset_index = (*font).num_glyphs;
    *((*font).type1_subset_index_to_glyphs).offset((*font).num_glyphs as isize) = glyph;
    let ref mut fresh4 = (*font).num_glyphs;
    *fresh4 += 1;
}
unsafe extern "C" fn is_ps_delimiter(mut c: libc::c_int) -> cairo_bool_t {
    static mut delimiters: [libc::c_char; 15] = unsafe {
        *::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"()[]{}<>/% \t\r\n\0")
    };
    return (strchr(delimiters.as_ptr(), c)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
unsafe extern "C" fn find_token(
    mut buffer: *const libc::c_char,
    mut end: *const libc::c_char,
    mut token: *const libc::c_char,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    if buffer.is_null() {
        return 0 as *const libc::c_char;
    }
    length = strlen(token) as libc::c_int;
    i = 0 as libc::c_int;
    while buffer.offset(i as isize)
        < end.offset(-(length as isize)).offset(1 as libc::c_int as isize)
    {
        if memcmp(
            buffer.offset(i as isize) as *const libc::c_void,
            token as *const libc::c_void,
            length as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if (i == 0 as libc::c_int
                || *token.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                || is_ps_delimiter(
                    *buffer.offset((i - 1 as libc::c_int) as isize) as libc::c_int,
                ) != 0)
                && (buffer.offset(i as isize) == end.offset(-(length as isize))
                    || is_ps_delimiter(
                        *buffer.offset((i + length) as isize) as libc::c_int,
                    ) != 0)
            {
                return buffer.offset(i as isize);
            }
        }
        i += 1;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn cairo_type1_font_subset_find_segments(
    mut font: *mut cairo_type1_font_subset_t,
) -> cairo_status_t {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut eexec_token: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    p = (*font).type1_data as *mut libc::c_uchar;
    let ref mut fresh5 = (*font).type1_end;
    *fresh5 = ((*font).type1_data).offset((*font).type1_length as isize);
    if (*font).type1_length >= 2 as libc::c_int as libc::c_uint
        && *p.offset(0 as libc::c_int as isize) as libc::c_int == 0x80 as libc::c_int
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 0x1 as libc::c_int
    {
        if (*font).type1_end < p.offset(6 as libc::c_int as isize) as *mut libc::c_char {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        (*font)
            .header_segment_size = (*p.offset(2 as libc::c_int as isize) as libc::c_int
            | (*p.offset(3 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | (*p.offset(4 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
            | (*p.offset(5 as libc::c_int as isize) as libc::c_uint)
                << 24 as libc::c_int;
        let ref mut fresh6 = (*font).header_segment;
        *fresh6 = (p as *mut libc::c_char).offset(6 as libc::c_int as isize);
        p = p
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_add((*font).header_segment_size) as isize,
            );
        if (*font).type1_end < p.offset(6 as libc::c_int as isize) as *mut libc::c_char {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        (*font)
            .eexec_segment_size = (*p.offset(2 as libc::c_int as isize) as libc::c_int
            | (*p.offset(3 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | (*p.offset(4 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
            | (*p.offset(5 as libc::c_int as isize) as libc::c_uint)
                << 24 as libc::c_int;
        let ref mut fresh7 = (*font).eexec_segment;
        *fresh7 = (p as *mut libc::c_char).offset(6 as libc::c_int as isize);
        (*font)
            .eexec_segment_is_ascii = (*p.offset(1 as libc::c_int as isize)
            as libc::c_int == 1 as libc::c_int) as libc::c_int;
        p = p
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_add((*font).eexec_segment_size) as isize,
            );
        while (*font).type1_end
            >= p.offset(6 as libc::c_int as isize) as *mut libc::c_char
            && *p.offset(1 as libc::c_int as isize) as libc::c_int != 0x3 as libc::c_int
        {
            size = (*p.offset(2 as libc::c_int as isize) as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int
                | (*p.offset(4 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint
                | (*p.offset(5 as libc::c_int as isize) as libc::c_uint)
                    << 24 as libc::c_int;
            if (*font).type1_end
                < p.offset(6 as libc::c_int as isize).offset(size as isize)
                    as *mut libc::c_char
            {
                return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
            }
            p = p.offset((6 as libc::c_int as libc::c_uint).wrapping_add(size) as isize);
        }
        let ref mut fresh8 = (*font).type1_end;
        *fresh8 = p as *mut libc::c_char;
    } else {
        eexec_token = find_token(
            p as *mut libc::c_char,
            (*font).type1_end,
            b"eexec\0" as *const u8 as *const libc::c_char,
        );
        if eexec_token.is_null() {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        (*font)
            .header_segment_size = (eexec_token.offset_from(p as *mut libc::c_char)
            as libc::c_long as libc::c_ulong)
            .wrapping_add(strlen(b"eexec\n\0" as *const u8 as *const libc::c_char))
            as libc::c_uint;
        let ref mut fresh9 = (*font).header_segment;
        *fresh9 = p as *mut libc::c_char;
        (*font)
            .eexec_segment_size = ((*font).type1_length)
            .wrapping_sub((*font).header_segment_size);
        let ref mut fresh10 = (*font).eexec_segment;
        *fresh10 = (p as *mut libc::c_char).offset((*font).header_segment_size as isize);
        (*font).eexec_segment_is_ascii = 1 as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 4 as libc::c_int as libc::c_uint {
            if _cairo_isxdigit(
                *((*font).eexec_segment).offset(i as isize) as libc::c_int,
            ) == 0
            {
                (*font).eexec_segment_is_ascii = 0 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_erase_dict_key(
    mut font: *mut cairo_type1_font_subset_t,
    mut key: *const libc::c_char,
) {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut segment_end: *const libc::c_char = 0 as *const libc::c_char;
    segment_end = ((*font).header_segment).offset((*font).header_segment_size as isize);
    start = (*font).header_segment;
    loop {
        start = find_token(start, segment_end, key);
        if !start.is_null() {
            p = start.offset(strlen(key) as isize);
            while p < segment_end
                && (_cairo_isspace(*p as libc::c_int) != 0
                    || _cairo_isdigit(*p as libc::c_int) != 0
                    || *p as libc::c_int == '[' as i32
                    || *p as libc::c_int == ']' as i32)
            {
                p = p.offset(1);
            }
            if p.offset(3 as libc::c_int as isize) < segment_end
                && memcmp(
                    p as *const libc::c_void,
                    b"def\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                memset(
                    start as *mut libc::c_char as *mut libc::c_void,
                    ' ' as i32,
                    p.offset(3 as libc::c_int as isize).offset_from(start)
                        as libc::c_long as libc::c_ulong,
                );
            }
            start = start.offset(strlen(key) as isize);
        }
        if start.is_null() {
            break;
        }
    };
}
unsafe extern "C" fn cairo_type1_font_subset_get_matrix(
    mut font: *mut cairo_type1_font_subset_t,
    mut name: *const libc::c_char,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
    mut d: *mut libc::c_double,
) -> cairo_status_t {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut segment_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut s_max: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut decimal_point: *const libc::c_char = 0 as *const libc::c_char;
    let mut decimal_point_len: libc::c_int = 0;
    decimal_point = _cairo_get_locale_decimal_point();
    decimal_point_len = strlen(decimal_point) as libc::c_int;
    if decimal_point_len != 0 as libc::c_int {} else {
        __assert_fail(
            b"decimal_point_len != 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-type1-subset.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 133],
                &[libc::c_char; 133],
            >(
                b"cairo_status_t cairo_type1_font_subset_get_matrix(cairo_type1_font_subset_t *, const char *, double *, double *, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    segment_end = ((*font).header_segment).offset((*font).header_segment_size as isize);
    start = find_token((*font).header_segment, segment_end, name);
    if start.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    end = find_token(start, segment_end, b"def\0" as *const u8 as *const libc::c_char);
    if end.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    s_max = (end.offset_from(start) as libc::c_long
        + (5 as libc::c_int * decimal_point_len) as libc::c_long
        + 1 as libc::c_int as libc::c_long) as libc::c_int;
    s = (if s_max != 0 as libc::c_int {
        malloc(s_max as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    if s.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while (i as libc::c_long) < end.offset_from(start) as libc::c_long
        && j < s_max - decimal_point_len
    {
        if *start.offset(i as isize) as libc::c_int == '.' as i32 {
            strncpy(
                s.offset(j as isize),
                decimal_point,
                (decimal_point_len + 1 as libc::c_int) as libc::c_ulong,
            );
            i += 1;
            j += decimal_point_len;
        } else {
            let fresh11 = i;
            i = i + 1;
            let fresh12 = j;
            j = j + 1;
            *s.offset(fresh12 as isize) = *start.offset(fresh11 as isize);
        }
    }
    *s.offset(j as isize) = 0 as libc::c_int as libc::c_char;
    start = strpbrk(s, b"{[\0" as *const u8 as *const libc::c_char);
    if start.is_null() {
        free(s as *mut libc::c_void);
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    start = start.offset(1);
    ret = 0 as libc::c_int;
    if *start != 0 {
        ret = sscanf(
            start,
            b"%lf %lf %lf %lf\0" as *const u8 as *const libc::c_char,
            a,
            b,
            c,
            d,
        );
    }
    free(s as *mut libc::c_void);
    if ret != 4 as libc::c_int {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_get_bbox(
    mut font: *mut cairo_type1_font_subset_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut x_min: libc::c_double = 0.;
    let mut y_min: libc::c_double = 0.;
    let mut x_max: libc::c_double = 0.;
    let mut y_max: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut yx: libc::c_double = 0.;
    let mut xy: libc::c_double = 0.;
    let mut yy: libc::c_double = 0.;
    status = cairo_type1_font_subset_get_matrix(
        font,
        b"/FontBBox\0" as *const u8 as *const libc::c_char,
        &mut x_min,
        &mut y_min,
        &mut x_max,
        &mut y_max,
    );
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_get_matrix(
        font,
        b"/FontMatrix\0" as *const u8 as *const libc::c_char,
        &mut xx,
        &mut yx,
        &mut xy,
        &mut yy,
    );
    if status as u64 != 0 {
        return status;
    }
    if yy == 0.0f64 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    (*font).base.units_per_em = 1.0f64 / yy;
    if xx != yy || yx != 0.0f64 || xy != 0.0f64 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    (*font).base.x_min = x_min / (*font).base.units_per_em;
    (*font).base.y_min = y_min / (*font).base.units_per_em;
    (*font).base.x_max = x_max / (*font).base.units_per_em;
    (*font).base.y_max = y_max / (*font).base.units_per_em;
    (*font).base.ascent = (*font).base.y_max;
    (*font).base.descent = (*font).base.y_min;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_get_fontname(
    mut font: *mut cairo_type1_font_subset_t,
) -> cairo_status_t {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut segment_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    segment_end = ((*font).header_segment).offset((*font).header_segment_size as isize);
    start = find_token(
        (*font).header_segment,
        segment_end,
        b"/FontName\0" as *const u8 as *const libc::c_char,
    );
    if start.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    start = start
        .offset(strlen(b"/FontName\0" as *const u8 as *const libc::c_char) as isize);
    end = find_token(start, segment_end, b"def\0" as *const u8 as *const libc::c_char);
    if end.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    while end > start
        && _cairo_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int) != 0
    {
        end = end.offset(-1);
    }
    s = (if end.offset_from(start) as libc::c_long + 1 as libc::c_int as libc::c_long
        != 0 as libc::c_int as libc::c_long
    {
        malloc(
            (end.offset_from(start) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    if s.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    strncpy(s, start, end.offset_from(start) as libc::c_long as libc::c_ulong);
    *s
        .offset(
            end.offset_from(start) as libc::c_long as isize,
        ) = 0 as libc::c_int as libc::c_char;
    start = strchr(s, '/' as i32);
    let fresh13 = start;
    start = start.offset(1);
    if fresh13.is_null() || start.is_null() {
        free(s as *mut libc::c_void);
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if strlen(start) > 7 as libc::c_int as libc::c_ulong
        && *start.offset(6 as libc::c_int as isize) as libc::c_int == '+' as i32
    {
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if (*start.offset(i as isize) as libc::c_int) < 'A' as i32
                || *start.offset(i as isize) as libc::c_int > 'Z' as i32
            {
                break;
            }
            i += 1;
        }
        if i == 6 as libc::c_int {
            start = start.offset(7 as libc::c_int as isize);
        }
    }
    let ref mut fresh14 = (*font).base.base_font;
    *fresh14 = strdup(start);
    free(s as *mut libc::c_void);
    if ((*font).base.base_font).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _cairo_escape_ps_name(&mut (*font).base.base_font) as cairo_status_t;
    return status;
}
unsafe extern "C" fn cairo_type1_font_subset_write_header(
    mut font: *mut cairo_type1_font_subset_t,
    mut name: *const libc::c_char,
) -> cairo_status_t {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut segment_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut glyph: libc::c_int = 0;
    cairo_type1_font_erase_dict_key(
        font,
        b"/UniqueID\0" as *const u8 as *const libc::c_char,
    );
    cairo_type1_font_erase_dict_key(
        font,
        b"/XUID\0" as *const u8 as *const libc::c_char,
    );
    segment_end = ((*font).header_segment).offset((*font).header_segment_size as isize);
    end = (*font).header_segment;
    start = find_token(
        (*font).header_segment,
        segment_end,
        b"/UniqueID\0" as *const u8 as *const libc::c_char,
    );
    if !start.is_null() {
        start = start.offset(9 as libc::c_int as isize);
        while start < segment_end && _cairo_isspace(*start as libc::c_int) != 0 {
            start = start.offset(1);
        }
        if start.offset(5 as libc::c_int as isize) < segment_end
            && memcmp(
                start as *const libc::c_void,
                b"known\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            _cairo_output_stream_write(
                (*font).output,
                (*font).header_segment as *const libc::c_void,
                start
                    .offset(5 as libc::c_int as isize)
                    .offset_from((*font).header_segment) as libc::c_long as size_t,
            );
            _cairo_output_stream_printf(
                (*font).output,
                b" pop false \0" as *const u8 as *const libc::c_char,
            );
            end = start.offset(5 as libc::c_int as isize);
        }
    }
    start = find_token(
        end,
        segment_end,
        b"/FontName\0" as *const u8 as *const libc::c_char,
    );
    if start.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    _cairo_output_stream_write(
        (*font).output,
        end as *const libc::c_void,
        start.offset_from(end) as libc::c_long as size_t,
    );
    _cairo_output_stream_printf(
        (*font).output,
        b"/FontName /%s def\0" as *const u8 as *const libc::c_char,
        name,
    );
    end = find_token(start, segment_end, b"def\0" as *const u8 as *const libc::c_char);
    if end.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    end = end.offset(3 as libc::c_int as isize);
    start = find_token(
        end,
        segment_end,
        b"/Encoding\0" as *const u8 as *const libc::c_char,
    );
    if start.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    _cairo_output_stream_write(
        (*font).output,
        end as *const libc::c_void,
        start.offset_from(end) as libc::c_long as size_t,
    );
    _cairo_output_stream_printf(
        (*font).output,
        b"/Encoding 256 array\n0 1 255 {1 index exch /.notdef put} for\n\0" as *const u8
            as *const libc::c_char,
    );
    if (*(*font).scaled_font_subset).is_latin != 0 {
        i = 1 as libc::c_int as libc::c_uint;
        while i < 256 as libc::c_int as libc::c_uint {
            let mut subset_glyph: libc::c_int = *((*(*font).scaled_font_subset)
                .latin_to_subset_glyph_index)
                .offset(i as isize) as libc::c_int;
            if subset_glyph > 0 as libc::c_int {
                _cairo_output_stream_printf(
                    (*font).output,
                    b"dup %d /%s put\n\0" as *const u8 as *const libc::c_char,
                    i,
                    _cairo_winansi_to_glyphname(i as libc::c_int),
                );
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 1 as libc::c_int as libc::c_uint;
        while i < (*(*font).scaled_font_subset).num_glyphs {
            glyph = *((*font).scaled_subset_index_to_glyphs).offset(i as isize);
            _cairo_output_stream_printf(
                (*font).output,
                b"dup %d /%s put\n\0" as *const u8 as *const libc::c_char,
                i,
                *((*font).glyph_names).offset(glyph as isize),
            );
            i = i.wrapping_add(1);
        }
    }
    _cairo_output_stream_printf(
        (*font).output,
        b"readonly def\0" as *const u8 as *const libc::c_char,
    );
    end = find_token(start, segment_end, b"def\0" as *const u8 as *const libc::c_char);
    if end.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    end = end.offset(3 as libc::c_int as isize);
    if !(find_token(
        end,
        segment_end,
        b"/Encoding\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    _cairo_output_stream_write(
        (*font).output,
        end as *const libc::c_void,
        segment_end.offset_from(end) as libc::c_long as size_t,
    );
    return (*(*font).output).status;
}
unsafe extern "C" fn hex_to_int(mut ch: libc::c_int) -> libc::c_int {
    if ch <= '9' as i32 {
        return ch - '0' as i32
    } else if ch <= 'F' as i32 {
        return ch - 'A' as i32 + 10 as libc::c_int
    } else {
        return ch - 'a' as i32 + 10 as libc::c_int
    };
}
unsafe extern "C" fn cairo_type1_font_subset_write_encrypted(
    mut font: *mut cairo_type1_font_subset_t,
    mut data: *const libc::c_char,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut in_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut c: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    static mut hex_digits: [libc::c_char; 16] = unsafe {
        *::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"0123456789abcdef")
    };
    let mut digits: [libc::c_char; 3] = [0; 3];
    in_0 = data as *const libc::c_uchar;
    end = (data as *const libc::c_uchar).offset(length as isize);
    while in_0 < end {
        let fresh15 = in_0;
        in_0 = in_0.offset(1);
        p = *fresh15 as libc::c_int;
        c = p ^ (*font).eexec_key as libc::c_int >> 8 as libc::c_int;
        (*font)
            .eexec_key = ((c + (*font).eexec_key as libc::c_int)
            * 52845 as libc::c_int as libc::c_ushort as libc::c_int
            + 22719 as libc::c_int as libc::c_ushort as libc::c_int) as libc::c_ushort;
        if (*font).hex_encode != 0 {
            digits[0 as libc::c_int
                as usize] = hex_digits[(c >> 4 as libc::c_int) as usize];
            digits[1 as libc::c_int
                as usize] = hex_digits[(c & 0xf as libc::c_int) as usize];
            digits[2 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
            (*font).hex_column += 2 as libc::c_int;
            if (*font).hex_column == 78 as libc::c_int {
                _cairo_output_stream_write(
                    (*font).output,
                    digits.as_mut_ptr() as *const libc::c_void,
                    3 as libc::c_int as size_t,
                );
                (*font).hex_column = 0 as libc::c_int;
            } else {
                _cairo_output_stream_write(
                    (*font).output,
                    digits.as_mut_ptr() as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
        } else {
            digits[0 as libc::c_int as usize] = c as libc::c_char;
            _cairo_output_stream_write(
                (*font).output,
                digits.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    return (*(*font).output).status;
}
unsafe extern "C" fn cairo_type1_font_subset_decrypt_eexec_segment(
    mut font: *mut cairo_type1_font_subset_t,
) -> cairo_status_t {
    let mut r: libc::c_ushort = 55665 as libc::c_int as libc::c_ushort;
    let mut in_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    in_0 = (*font).eexec_segment as *mut libc::c_uchar;
    end = in_0.offset((*font).eexec_segment_size as isize);
    let ref mut fresh16 = (*font).cleartext;
    *fresh16 = (if ((*font).eexec_segment_size)
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        != 0 as libc::c_int as libc::c_uint
    {
        malloc(
            ((*font).eexec_segment_size).wrapping_add(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    if ((*font).cleartext).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    out = (*font).cleartext;
    while in_0 < end {
        if (*font).eexec_segment_is_ascii != 0 {
            let fresh17 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh17 as libc::c_int;
            if _cairo_isspace(c) != 0 {
                continue;
            }
            let fresh18 = in_0;
            in_0 = in_0.offset(1);
            c = hex_to_int(c) << 4 as libc::c_int | hex_to_int(*fresh18 as libc::c_int);
        } else {
            let fresh19 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh19 as libc::c_int;
        }
        p = c ^ r as libc::c_int >> 8 as libc::c_int;
        r = ((c + r as libc::c_int)
            * 52845 as libc::c_int as libc::c_ushort as libc::c_int
            + 22719 as libc::c_int as libc::c_ushort as libc::c_int) as libc::c_ushort;
        let fresh20 = out;
        out = out.offset(1);
        *fresh20 = p as libc::c_char;
    }
    let ref mut fresh21 = (*font).cleartext_end;
    *fresh21 = out;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint && i < (*font).eexec_segment_size {
        *((*font).cleartext).offset(i as isize) = ' ' as i32 as libc::c_char;
        i = i.wrapping_add(1);
    }
    *((*font).cleartext)
        .offset((*font).eexec_segment_size as isize) = 0 as libc::c_int as libc::c_char;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn skip_token(
    mut p: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *const libc::c_char {
    while p < end && _cairo_isspace(*p as libc::c_int) != 0 {
        p = p.offset(1);
    }
    while p < end && _cairo_isspace(*p as libc::c_int) == 0 {
        p = p.offset(1);
    }
    if p == end {
        return 0 as *const libc::c_char;
    }
    return p;
}
unsafe extern "C" fn cairo_type1_font_subset_decrypt_charstring(
    mut in_0: *const libc::c_uchar,
    mut size: libc::c_int,
    mut out: *mut libc::c_uchar,
) {
    let mut r: libc::c_ushort = 4330 as libc::c_int as libc::c_ushort;
    let mut c: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let fresh22 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh22 as libc::c_int;
        p = c ^ r as libc::c_int >> 8 as libc::c_int;
        r = ((c + r as libc::c_int)
            * 52845 as libc::c_int as libc::c_ushort as libc::c_int
            + 22719 as libc::c_int as libc::c_ushort as libc::c_int) as libc::c_ushort;
        let fresh23 = out;
        out = out.offset(1);
        *fresh23 = p as libc::c_uchar;
        i += 1;
    }
}
unsafe extern "C" fn cairo_type1_font_subset_decode_integer(
    mut p: *const libc::c_uchar,
    mut integer: *mut libc::c_int,
) -> *const libc::c_uchar {
    if *p as libc::c_int <= 246 as libc::c_int {
        let fresh24 = p;
        p = p.offset(1);
        *integer = *fresh24 as libc::c_int - 139 as libc::c_int;
    } else if *p as libc::c_int <= 250 as libc::c_int {
        *integer = (*p.offset(0 as libc::c_int as isize) as libc::c_int
            - 247 as libc::c_int) * 256 as libc::c_int
            + *p.offset(1 as libc::c_int as isize) as libc::c_int + 108 as libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
    } else if *p as libc::c_int <= 254 as libc::c_int {
        *integer = -(*p.offset(0 as libc::c_int as isize) as libc::c_int
            - 251 as libc::c_int) * 256 as libc::c_int
            - *p.offset(1 as libc::c_int as isize) as libc::c_int - 108 as libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
    } else {
        *integer = ((*p.offset(1 as libc::c_int as isize) as uint32_t)
            << 24 as libc::c_int
            | ((*p.offset(2 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int) as libc::c_uint
            | ((*p.offset(3 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
                as libc::c_uint | *p.offset(4 as libc::c_int as isize) as libc::c_uint)
            as libc::c_int;
        p = p.offset(5 as libc::c_int as isize);
    }
    return p;
}
unsafe extern "C" fn use_standard_encoding_glyph(
    mut font: *mut cairo_type1_font_subset_t,
    mut index: libc::c_int,
) -> cairo_status_t {
    let mut glyph_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    if index < 0 as libc::c_int || index > 255 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    glyph_name = _cairo_ps_standard_encoding_to_glyphname(index);
    if glyph_name.is_null() {
        return CAIRO_STATUS_SUCCESS;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font).base.num_glyphs {
        if !(*((*font).glyph_names).offset(i as isize)).is_null()
            && strcmp(*((*font).glyph_names).offset(i as isize), glyph_name)
                == 0 as libc::c_int
        {
            cairo_type1_font_subset_use_glyph(font, i as libc::c_int);
            return CAIRO_STATUS_SUCCESS;
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
}
unsafe extern "C" fn cairo_type1_font_subset_parse_charstring(
    mut font: *mut cairo_type1_font_subset_t,
    mut glyph: libc::c_int,
    mut encrypted_charstring: *const libc::c_char,
    mut encrypted_charstring_length: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut charstring: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut command: libc::c_int = 0;
    charstring = (if encrypted_charstring_length != 0 as libc::c_int {
        malloc(encrypted_charstring_length as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if charstring.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    cairo_type1_font_subset_decrypt_charstring(
        encrypted_charstring as *const libc::c_uchar,
        encrypted_charstring_length,
        charstring,
    );
    end = charstring.offset(encrypted_charstring_length as isize);
    p = charstring.offset((*font).lenIV as isize);
    status = CAIRO_STATUS_SUCCESS;
    while p < end {
        if (*p as libc::c_int) < 32 as libc::c_int {
            let fresh25 = p;
            p = p.offset(1);
            command = *fresh25 as libc::c_int;
            match command {
                10 => {
                    if (*font).subset_subrs != 0
                        && (*font).build_stack.sp > 0 as libc::c_int
                    {
                        let mut int_val: libc::c_double = 0.;
                        let ref mut fresh26 = (*font).build_stack.sp;
                        *fresh26 -= 1;
                        if modf(
                            (*font).build_stack.stack[*fresh26 as usize],
                            &mut int_val,
                        ) == 0.0f64
                        {
                            let mut subr_num: libc::c_int = int_val as libc::c_int;
                            if subr_num >= 0 as libc::c_int
                                && subr_num < (*font).num_subrs
                            {
                                (*((*font).subrs).offset(subr_num as isize))
                                    .used = 1 as libc::c_int;
                                status = cairo_type1_font_subset_parse_charstring(
                                    font,
                                    glyph,
                                    (*((*font).subrs).offset(subr_num as isize)).subr_string,
                                    (*((*font).subrs).offset(subr_num as isize)).subr_length,
                                );
                                continue;
                            }
                        }
                    }
                    (*font).subset_subrs = 0 as libc::c_int;
                }
                13 => {
                    if (*font).build_stack.sp < 2 as libc::c_int {
                        status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                            as cairo_status_t;
                        break;
                    } else {
                        (*((*font).glyphs).offset(glyph as isize))
                            .width = (*font).build_stack.stack[1 as libc::c_int as usize]
                            / (*font).base.units_per_em;
                        (*font).build_stack.sp = 0 as libc::c_int;
                    }
                }
                12 => {
                    let fresh27 = p;
                    p = p.offset(1);
                    command = command << 8 as libc::c_int | *fresh27 as libc::c_int;
                    match command {
                        3078 => {
                            if (*font).build_stack.sp < 5 as libc::c_int {
                                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                    as cairo_status_t;
                                break;
                            } else {
                                status = use_standard_encoding_glyph(
                                    font,
                                    (*font).build_stack.stack[3 as libc::c_int as usize]
                                        as libc::c_int,
                                );
                                if status as u64 != 0 {
                                    break;
                                }
                                status = use_standard_encoding_glyph(
                                    font,
                                    (*font).build_stack.stack[4 as libc::c_int as usize]
                                        as libc::c_int,
                                );
                                if status as u64 != 0 {
                                    break;
                                }
                                (*font).build_stack.sp = 0 as libc::c_int;
                            }
                        }
                        3079 => {
                            if (*font).build_stack.sp < 4 as libc::c_int {
                                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                    as cairo_status_t;
                                break;
                            } else {
                                (*((*font).glyphs).offset(glyph as isize))
                                    .width = (*font)
                                    .build_stack
                                    .stack[2 as libc::c_int as usize]
                                    / (*font).base.units_per_em;
                                (*font).build_stack.sp = 0 as libc::c_int;
                            }
                        }
                        3084 => {
                            if (*font).build_stack.sp < 2 as libc::c_int {
                                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                    as cairo_status_t;
                                break;
                            } else {
                                let mut num1: libc::c_double = (*font)
                                    .build_stack
                                    .stack[((*font).build_stack.sp - 2 as libc::c_int)
                                    as usize];
                                let mut num2: libc::c_double = (*font)
                                    .build_stack
                                    .stack[((*font).build_stack.sp - 1 as libc::c_int)
                                    as usize];
                                let ref mut fresh28 = (*font).build_stack.sp;
                                *fresh28 -= 1;
                                if num2 == 0.0f64 {
                                    status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                        as cairo_status_t;
                                    break;
                                } else {
                                    (*font)
                                        .build_stack
                                        .stack[((*font).build_stack.sp - 1 as libc::c_int)
                                        as usize] = num1 / num2;
                                }
                            }
                        }
                        3088 => {
                            if (*font).build_stack.sp < 1 as libc::c_int {
                                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                    as cairo_status_t;
                                break;
                            } else {
                                let ref mut fresh29 = (*font).build_stack.sp;
                                *fresh29 -= 1;
                                (*font).ps_stack.sp = 0 as libc::c_int;
                                while (*font).build_stack.sp != 0 {
                                    let ref mut fresh30 = (*font).build_stack.sp;
                                    *fresh30 -= 1;
                                    let ref mut fresh31 = (*font).ps_stack.sp;
                                    let fresh32 = *fresh31;
                                    *fresh31 = *fresh31 + 1;
                                    (*font)
                                        .ps_stack
                                        .stack[fresh32
                                        as usize] = (*font).build_stack.stack[*fresh30 as usize]
                                        as libc::c_int;
                                }
                            }
                        }
                        3089 => {
                            if (*font).ps_stack.sp < 1 as libc::c_int {
                                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                    as cairo_status_t;
                                break;
                            } else {
                                let ref mut fresh33 = (*font).ps_stack.sp;
                                *fresh33 -= 1;
                                let ref mut fresh34 = (*font).build_stack.sp;
                                let fresh35 = *fresh34;
                                *fresh34 = *fresh34 + 1;
                                (*font)
                                    .build_stack
                                    .stack[fresh35
                                    as usize] = (*font).ps_stack.stack[*fresh33 as usize]
                                    as libc::c_double;
                            }
                        }
                        3072 | 3073 | 3074 | 3105 | _ => {
                            (*font).build_stack.sp = 0 as libc::c_int;
                        }
                    }
                }
                1 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 21 | 22 | 30 | 31 | 11 | 14 | _ => {
                    (*font).build_stack.sp = 0 as libc::c_int;
                }
            }
        } else if (*font).build_stack.sp < 24 as libc::c_int {
            let mut val: libc::c_int = 0;
            p = cairo_type1_font_subset_decode_integer(p, &mut val);
            let ref mut fresh36 = (*font).build_stack.sp;
            let fresh37 = *fresh36;
            *fresh36 = *fresh36 + 1;
            (*font).build_stack.stack[fresh37 as usize] = val as libc::c_double;
        } else {
            status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
            break;
        }
    }
    free(charstring as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn cairo_type1_font_subset_build_subr_list(
    mut font: *mut cairo_type1_font_subset_t,
    mut subr_number: libc::c_int,
    mut encrypted_charstring: *const libc::c_char,
    mut encrypted_charstring_length: libc::c_int,
    mut np: *const libc::c_char,
    mut np_length: libc::c_int,
) -> cairo_status_t {
    let ref mut fresh38 = (*((*font).subrs).offset(subr_number as isize)).subr_string;
    *fresh38 = encrypted_charstring;
    (*((*font).subrs).offset(subr_number as isize))
        .subr_length = encrypted_charstring_length;
    let ref mut fresh39 = (*((*font).subrs).offset(subr_number as isize)).np;
    *fresh39 = np;
    (*((*font).subrs).offset(subr_number as isize)).np_length = np_length;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn write_used_subrs(
    mut font: *mut cairo_type1_font_subset_t,
    mut subr_number: libc::c_int,
    mut subr_string: *const libc::c_char,
    mut subr_string_length: libc::c_int,
    mut np: *const libc::c_char,
    mut np_length: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut length: libc::c_int = 0;
    if (*((*font).subrs).offset(subr_number as isize)).used == 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    length = snprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"dup %d %d %s \0" as *const u8 as *const libc::c_char,
        subr_number,
        subr_string_length,
        (*font).rd,
    );
    status = cairo_type1_font_subset_write_encrypted(
        font,
        buffer.as_mut_ptr(),
        length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_write_encrypted(
        font,
        subr_string,
        subr_string_length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    if !np.is_null() {
        status = cairo_type1_font_subset_write_encrypted(
            font,
            np,
            np_length as libc::c_uint,
        );
    } else {
        length = snprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (*font).np,
        );
        status = cairo_type1_font_subset_write_encrypted(
            font,
            buffer.as_mut_ptr(),
            length as libc::c_uint,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_for_each_subr(
    mut font: *mut cairo_type1_font_subset_t,
    mut array_start: *const libc::c_char,
    mut cleartext_end: *const libc::c_char,
    mut func: subr_func_t,
    mut array_end: *mut *const libc::c_char,
) -> cairo_status_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut subr_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subr_num: libc::c_int = 0;
    let mut subr_length: libc::c_int = 0;
    let mut np: *const libc::c_char = 0 as *const libc::c_char;
    let mut np_length: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    p = array_start;
    while p.offset(3 as libc::c_int as isize) < cleartext_end
        && strncmp(
            p,
            b"dup\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        p = skip_token(p, cleartext_end);
        subr_num = strtol(p, &mut end, 10 as libc::c_int) as libc::c_int;
        if p == end as *const libc::c_char {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        if subr_num < 0 as libc::c_int || subr_num >= (*font).num_subrs {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        p = end;
        subr_length = strtol(p, &mut end, 10 as libc::c_int) as libc::c_int;
        if p == end as *const libc::c_char {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        subr_string = (skip_token(end, cleartext_end)).offset(1 as libc::c_int as isize);
        np = 0 as *const libc::c_char;
        np_length = 0 as libc::c_int;
        p = skip_token(subr_string.offset(subr_length as isize), cleartext_end);
        while p < cleartext_end && _cairo_isspace(*p as libc::c_int) != 0 {
            p = p.offset(1);
        }
        if p.offset(3 as libc::c_int as isize) < cleartext_end
            && strncmp(
                p,
                b"put\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            p = skip_token(p, cleartext_end);
            while p < cleartext_end && _cairo_isspace(*p as libc::c_int) != 0 {
                p = p.offset(1);
            }
            np = subr_string.offset(subr_length as isize);
            np_length = p.offset_from(np) as libc::c_long as libc::c_int;
        }
        status = func
            .expect(
                "non-null function pointer",
            )(font, subr_num, subr_string, subr_length, np, np_length);
        if status as u64 != 0 {
            return status;
        }
    }
    *array_end = p as *mut libc::c_char;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_build_glyph_list(
    mut font: *mut cairo_type1_font_subset_t,
    mut glyph_number: libc::c_int,
    mut name: *const libc::c_char,
    mut name_length: libc::c_int,
    mut encrypted_charstring: *const libc::c_char,
    mut encrypted_charstring_length: libc::c_int,
) -> cairo_status_t {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut glyph: glyph_data_t = glyph_data_t {
        subset_index: 0,
        width: 0.,
        encrypted_charstring: 0 as *const libc::c_char,
        encrypted_charstring_length: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    s = (if name_length + 1 as libc::c_int != 0 as libc::c_int {
        malloc((name_length + 1 as libc::c_int) as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    if s.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    strncpy(s, name, name_length as libc::c_ulong);
    *s.offset(name_length as isize) = 0 as libc::c_int as libc::c_char;
    status = _cairo_array_append(
        &mut (*font).glyph_names_array,
        &mut s as *mut *mut libc::c_char as *const libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    glyph.subset_index = -(1 as libc::c_int);
    glyph.width = 0 as libc::c_int as libc::c_double;
    glyph.encrypted_charstring = encrypted_charstring;
    glyph.encrypted_charstring_length = encrypted_charstring_length;
    status = _cairo_array_append(
        &mut (*font).glyphs_array,
        &mut glyph as *mut glyph_data_t as *const libc::c_void,
    );
    return status;
}
unsafe extern "C" fn write_used_glyphs(
    mut font: *mut cairo_type1_font_subset_t,
    mut glyph_number: libc::c_int,
    mut name: *const libc::c_char,
    mut name_length: libc::c_int,
    mut charstring: *const libc::c_char,
    mut charstring_length: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut length: libc::c_int = 0;
    let mut subset_id: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut wa_name: *const libc::c_char = 0 as *const libc::c_char;
    if (*((*font).glyphs).offset(glyph_number as isize)).subset_index < 0 as libc::c_int
    {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*(*font).scaled_font_subset).is_latin != 0 {
        subset_id = (*((*font).glyphs).offset(glyph_number as isize)).subset_index
            as libc::c_uint;
        if subset_id > 0 as libc::c_int as libc::c_uint
            && subset_id < (*(*font).scaled_font_subset).num_glyphs
        {
            ch = *((*(*font).scaled_font_subset).to_latin_char)
                .offset(subset_id as isize);
            wa_name = _cairo_winansi_to_glyphname(ch);
            if !wa_name.is_null() {
                name = wa_name;
                name_length = strlen(name) as libc::c_int;
            }
        }
    }
    length = snprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"/%.*s %d %s \0" as *const u8 as *const libc::c_char,
        name_length,
        name,
        charstring_length,
        (*font).rd,
    );
    status = cairo_type1_font_subset_write_encrypted(
        font,
        buffer.as_mut_ptr(),
        length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_write_encrypted(
        font,
        charstring,
        charstring_length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    length = snprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        (*font).nd,
    );
    status = cairo_type1_font_subset_write_encrypted(
        font,
        buffer.as_mut_ptr(),
        length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_for_each_glyph(
    mut font: *mut cairo_type1_font_subset_t,
    mut dict_start: *const libc::c_char,
    mut dict_end: *const libc::c_char,
    mut func: glyph_func_t,
    mut dict_out: *mut *const libc::c_char,
) -> cairo_status_t {
    let mut charstring_length: libc::c_int = 0;
    let mut name_length: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut charstring: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut glyph_count: libc::c_int = 0;
    p = dict_start;
    glyph_count = 0 as libc::c_int;
    while *p as libc::c_int == '/' as i32 {
        name = p.offset(1 as libc::c_int as isize);
        p = skip_token(p, dict_end);
        name_length = p.offset_from(name) as libc::c_long as libc::c_int;
        charstring_length = strtol(p, &mut end, 10 as libc::c_int) as libc::c_int;
        if p == end as *const libc::c_char {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        charstring = (skip_token(end, dict_end)).offset(1 as libc::c_int as isize);
        p = skip_token(charstring.offset(charstring_length as isize), dict_end);
        while p < dict_end && _cairo_isspace(*p as libc::c_int) != 0 {
            p = p.offset(1);
        }
        if p == dict_end {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        let fresh40 = glyph_count;
        glyph_count = glyph_count + 1;
        status = func
            .expect(
                "non-null function pointer",
            )(font, fresh40, name, name_length, charstring, charstring_length);
        if status as u64 != 0 {
            return status;
        }
    }
    *dict_out = p;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_write_private_dict(
    mut font: *mut cairo_type1_font_subset_t,
    mut name: *const libc::c_char,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut subrs: *const libc::c_char = 0 as *const libc::c_char;
    let mut charstrings: *const libc::c_char = 0 as *const libc::c_char;
    let mut array_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut array_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut dict_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut dict_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut lenIV_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut lenIV_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut closefile_token: *const libc::c_char = 0 as *const libc::c_char;
    let mut buffer: [libc::c_char; 32] = [0; 32];
    let mut lenIV_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subr_count_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut glyph_count_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut lenIV: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut i: libc::c_uint = 0;
    let mut glyph: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*font).lenIV = 4 as libc::c_int;
    lenIV_start = find_token(
        (*font).cleartext,
        (*font).cleartext_end,
        b"/lenIV\0" as *const u8 as *const libc::c_char,
    );
    if !lenIV_start.is_null() {
        lenIV_start = lenIV_start.offset(6 as libc::c_int as isize);
        lenIV_end = find_token(
            lenIV_start,
            (*font).cleartext_end,
            b"def\0" as *const u8 as *const libc::c_char,
        );
        if lenIV_end.is_null() {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        lenIV_str = (if lenIV_end.offset_from(lenIV_start) as libc::c_long
            + 1 as libc::c_int as libc::c_long != 0 as libc::c_int as libc::c_long
        {
            malloc(
                (lenIV_end.offset_from(lenIV_start) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            )
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if lenIV_str.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        strncpy(
            lenIV_str,
            lenIV_start,
            lenIV_end.offset_from(lenIV_start) as libc::c_long as libc::c_ulong,
        );
        *lenIV_str
            .offset(
                lenIV_end.offset_from(lenIV_start) as libc::c_long as isize,
            ) = 0 as libc::c_int as libc::c_char;
        ret = sscanf(
            lenIV_str,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut lenIV as *mut libc::c_int,
        );
        free(lenIV_str as *mut libc::c_void);
        if ret <= 0 as libc::c_int {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        if lenIV < 0 as libc::c_int {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        (*font).lenIV = lenIV;
    }
    subrs = find_token(
        (*font).cleartext,
        (*font).cleartext_end,
        b"/Subrs\0" as *const u8 as *const libc::c_char,
    );
    if subrs.is_null() {
        (*font).subset_subrs = 0 as libc::c_int;
        p = (*font).cleartext;
        array_start = 0 as *const libc::c_char;
    } else {
        p = subrs
            .offset(strlen(b"/Subrs\0" as *const u8 as *const libc::c_char) as isize);
        (*font)
            .num_subrs = strtol(p, &mut subr_count_end, 10 as libc::c_int)
            as libc::c_int;
        if subr_count_end == p as *mut libc::c_char {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        if (*font).num_subrs <= 0 as libc::c_int {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        let ref mut fresh41 = (*font).subrs;
        *fresh41 = calloc(
            (*font).num_subrs as libc::c_ulong,
            ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
        ) as *mut C2RustUnnamed_1;
        if ((*font).subrs).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        array_start = find_token(
            subr_count_end,
            (*font).cleartext_end,
            b"dup\0" as *const u8 as *const libc::c_char,
        );
        if array_start.is_null() {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        status = cairo_type1_font_for_each_subr(
            font,
            array_start,
            (*font).cleartext_end,
            Some(
                cairo_type1_font_subset_build_subr_list
                    as unsafe extern "C" fn(
                        *mut cairo_type1_font_subset_t,
                        libc::c_int,
                        *const libc::c_char,
                        libc::c_int,
                        *const libc::c_char,
                        libc::c_int,
                    ) -> cairo_status_t,
            ),
            &mut array_end,
        );
        if status as u64 != 0 {
            return status;
        }
        p = array_end;
    }
    charstrings = find_token(
        p,
        (*font).cleartext_end,
        b"/CharStrings\0" as *const u8 as *const libc::c_char,
    );
    if charstrings.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    p = charstrings
        .offset(strlen(b"/CharStrings\0" as *const u8 as *const libc::c_char) as isize);
    strtol(p, &mut glyph_count_end, 10 as libc::c_int);
    if p == glyph_count_end as *const libc::c_char {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    p = glyph_count_end;
    while p < (*font).cleartext_end as *const libc::c_char {
        if *p as libc::c_int == '/' as i32 {
            break;
        }
        p = p.offset(1);
    }
    if p == (*font).cleartext_end as *const libc::c_char {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    dict_start = p;
    status = cairo_type1_font_subset_for_each_glyph(
        font,
        dict_start,
        (*font).cleartext_end,
        Some(
            cairo_type1_font_subset_build_glyph_list
                as unsafe extern "C" fn(
                    *mut cairo_type1_font_subset_t,
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                ) -> cairo_status_t,
        ),
        &mut dict_end,
    );
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh42 = (*font).glyphs;
    *fresh42 = _cairo_array_index(
        &mut (*font).glyphs_array,
        0 as libc::c_int as libc::c_uint,
    ) as *mut glyph_data_t;
    let ref mut fresh43 = (*font).glyph_names;
    *fresh43 = _cairo_array_index(
        &mut (*font).glyph_names_array,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut libc::c_char;
    (*font).base.num_glyphs = _cairo_array_num_elements(&mut (*font).glyphs_array);
    let ref mut fresh44 = (*font).type1_subset_index_to_glyphs;
    *fresh44 = calloc(
        (*font).base.num_glyphs as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).type1_subset_index_to_glyphs).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    backend = (*(*(*font).scaled_font_subset).scaled_font).backend;
    if ((*backend).index_to_glyph_name).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        let mut index: libc::c_ulong = 0;
        status = ((*backend).index_to_glyph_name)
            .expect(
                "non-null function pointer",
            )(
            (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
            (*font).glyph_names,
            (*font).base.num_glyphs as libc::c_int,
            *((*(*font).scaled_font_subset).glyphs).offset(i as isize),
            &mut index,
        ) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        cairo_type1_font_subset_use_glyph(font, index as libc::c_int);
        *((*font).scaled_subset_index_to_glyphs)
            .offset(i as isize) = index as libc::c_int;
        i = i.wrapping_add(1);
    }
    j = 0 as libc::c_int;
    while j < (*font).num_glyphs {
        glyph = *((*font).type1_subset_index_to_glyphs).offset(j as isize);
        (*font).build_stack.sp = 0 as libc::c_int;
        (*font).ps_stack.sp = 0 as libc::c_int;
        status = cairo_type1_font_subset_parse_charstring(
            font,
            glyph,
            (*((*font).glyphs).offset(glyph as isize)).encrypted_charstring,
            (*((*font).glyphs).offset(glyph as isize)).encrypted_charstring_length,
        );
        if status as u64 != 0 {
            return status;
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j
        < (if (*font).num_subrs < 5 as libc::c_int {
            (*font).num_subrs
        } else {
            5 as libc::c_int
        })
    {
        (*((*font).subrs).offset(j as isize)).used = 1 as libc::c_int;
        j += 1;
    }
    closefile_token = find_token(
        dict_end,
        (*font).cleartext_end,
        b"closefile\0" as *const u8 as *const libc::c_char,
    );
    if closefile_token.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = cairo_type1_font_subset_write_header(font, name);
    if status as u64 != 0 {
        return status;
    }
    (*font)
        .base
        .header_size = _cairo_output_stream_get_position((*font).output)
        as libc::c_ulong;
    if (*font).subset_subrs != 0 {
        status = cairo_type1_font_subset_write_encrypted(
            font,
            (*font).cleartext,
            array_start.offset_from((*font).cleartext) as libc::c_long as libc::c_uint,
        );
        if status as u64 != 0 {
            return status;
        }
        status = cairo_type1_font_for_each_subr(
            font,
            array_start,
            (*font).cleartext_end,
            Some(
                write_used_subrs
                    as unsafe extern "C" fn(
                        *mut cairo_type1_font_subset_t,
                        libc::c_int,
                        *const libc::c_char,
                        libc::c_int,
                        *const libc::c_char,
                        libc::c_int,
                    ) -> cairo_status_t,
            ),
            &mut p,
        );
        if status as u64 != 0 {
            return status;
        }
    } else {
        p = (*font).cleartext;
    }
    status = cairo_type1_font_subset_write_encrypted(
        font,
        p,
        charstrings.offset_from(p) as libc::c_long as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    length = snprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"/CharStrings %d\0" as *const u8 as *const libc::c_char,
        (*font).num_glyphs,
    );
    status = cairo_type1_font_subset_write_encrypted(
        font,
        buffer.as_mut_ptr(),
        length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_write_encrypted(
        font,
        glyph_count_end,
        dict_start.offset_from(glyph_count_end) as libc::c_long as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_for_each_glyph(
        font,
        dict_start,
        (*font).cleartext_end,
        Some(
            write_used_glyphs
                as unsafe extern "C" fn(
                    *mut cairo_type1_font_subset_t,
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                ) -> cairo_status_t,
        ),
        &mut p,
    );
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_write_encrypted(
        font,
        p,
        (closefile_token.offset_from(p) as libc::c_long as libc::c_ulong)
            .wrapping_add(strlen(b"closefile\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*font).hex_encode != 0 {
        _cairo_output_stream_write(
            (*font).output,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_subset_write_trailer(
    mut font: *mut cairo_type1_font_subset_t,
) -> cairo_status_t {
    let mut cleartomark_token: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    static mut zeros: [libc::c_char; 65] = unsafe {
        *::std::mem::transmute::<
            &[u8; 65],
            &[libc::c_char; 65],
        >(b"0000000000000000000000000000000000000000000000000000000000000000\n")
    };
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        _cairo_output_stream_write(
            (*font).output,
            zeros.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
        );
        i += 1;
    }
    cleartomark_token = find_token(
        (*font).type1_data,
        (*font).type1_end,
        b"cleartomark\0" as *const u8 as *const libc::c_char,
    );
    if !cleartomark_token.is_null() {
        _cairo_output_stream_write(
            (*font).output,
            cleartomark_token as *const libc::c_void,
            ((*font).type1_end).offset_from(cleartomark_token) as libc::c_long as size_t,
        );
        if *((*font).type1_end).offset(-(1 as libc::c_int as isize)) as libc::c_int
            != '\n' as i32
        {
            _cairo_output_stream_printf(
                (*font).output,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*font).eexec_segment_is_ascii == 0 {
        _cairo_output_stream_printf(
            (*font).output,
            b"cleartomark\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t
    }
    _cairo_output_stream_printf(
        (*font).output,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn type1_font_write(
    mut closure: *mut libc::c_void,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut font: *mut cairo_type1_font_subset_t = closure
        as *mut cairo_type1_font_subset_t;
    return _cairo_array_append_multiple(
        &mut (*font).contents,
        data as *const libc::c_void,
        length,
    );
}
unsafe extern "C" fn cairo_type1_font_subset_write(
    mut font: *mut cairo_type1_font_subset_t,
    mut name: *const libc::c_char,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_type1_font_subset_find_segments(font);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_decrypt_eexec_segment(font);
    if status as u64 != 0 {
        return status;
    }
    if !(find_token(
        (*font).cleartext,
        (*font).cleartext_end,
        b"/-|\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        let ref mut fresh45 = (*font).rd;
        *fresh45 = b"-|\0" as *const u8 as *const libc::c_char;
        let ref mut fresh46 = (*font).nd;
        *fresh46 = b"|-\0" as *const u8 as *const libc::c_char;
        let ref mut fresh47 = (*font).np;
        *fresh47 = b"|\0" as *const u8 as *const libc::c_char;
    } else if !(find_token(
        (*font).cleartext,
        (*font).cleartext_end,
        b"/RD\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        let ref mut fresh48 = (*font).rd;
        *fresh48 = b"RD\0" as *const u8 as *const libc::c_char;
        let ref mut fresh49 = (*font).nd;
        *fresh49 = b"ND\0" as *const u8 as *const libc::c_char;
        let ref mut fresh50 = (*font).np;
        *fresh50 = b"NP\0" as *const u8 as *const libc::c_char;
    } else {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t
    }
    (*font).eexec_key = 55665 as libc::c_int as libc::c_ushort;
    (*font).hex_column = 0 as libc::c_int;
    status = cairo_type1_font_subset_get_bbox(font);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_get_fontname(font);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_write_private_dict(font, name);
    if status as u64 != 0 {
        return status;
    }
    (*font)
        .base
        .data_size = (_cairo_output_stream_get_position((*font).output)
        as libc::c_ulonglong)
        .wrapping_sub((*font).base.header_size as libc::c_ulonglong) as libc::c_ulong;
    status = cairo_type1_font_subset_write_trailer(font);
    if status as u64 != 0 {
        return status;
    }
    (*font)
        .base
        .trailer_size = (_cairo_output_stream_get_position((*font).output)
        as libc::c_ulonglong)
        .wrapping_sub((*font).base.header_size as libc::c_ulonglong)
        .wrapping_sub((*font).base.data_size as libc::c_ulonglong) as libc::c_ulong;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn check_fontdata_is_type1(
    mut data: *const libc::c_uchar,
    mut length: libc::c_long,
) -> cairo_bool_t {
    if length > 2 as libc::c_int as libc::c_long
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x80 as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if length > 2 as libc::c_int as libc::c_long
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == '!' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cairo_type1_font_subset_generate(
    mut abstract_font: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> cairo_status_t {
    let mut font: *mut cairo_type1_font_subset_t = abstract_font
        as *mut cairo_type1_font_subset_t;
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut data_length: libc::c_ulong = 0;
    scaled_font = (*(*font).scaled_font_subset).scaled_font;
    if ((*(*scaled_font).backend).load_type1_data).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = ((*(*scaled_font).backend).load_type1_data)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    (*font).type1_length = data_length as libc::c_uint;
    let ref mut fresh51 = (*font).type1_data;
    *fresh51 = (if (*font).type1_length != 0 as libc::c_int as libc::c_uint {
        malloc((*font).type1_length as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    if ((*font).type1_data).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = ((*(*scaled_font).backend).load_type1_data)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_long,
        (*font).type1_data as *mut libc::c_uchar,
        &mut data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    if check_fontdata_is_type1(
        (*font).type1_data as *mut libc::c_uchar,
        data_length as libc::c_long,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = _cairo_array_grow_by(
        &mut (*font).contents,
        4096 as libc::c_int as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh52 = (*font).output;
    *fresh52 = _cairo_output_stream_create(
        Some(
            type1_font_write
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        font as *mut libc::c_void,
    );
    status = (*(*font).output).status;
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_write(font, name);
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh53 = (*font).base.data;
    *fresh53 = _cairo_array_index(
        &mut (*font).contents,
        0 as libc::c_int as libc::c_uint,
    ) as *const libc::c_char;
    return status;
}
unsafe extern "C" fn _cairo_type1_font_subset_fini(
    mut font: *mut cairo_type1_font_subset_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    _cairo_array_fini(&mut (*font).contents);
    free((*font).type1_data as *mut libc::c_void);
    i = 0 as libc::c_int as libc::c_uint;
    while i < _cairo_array_num_elements(&mut (*font).glyph_names_array) {
        let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        s = _cairo_array_index(&mut (*font).glyph_names_array, i)
            as *mut *mut libc::c_char;
        free(*s as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    _cairo_array_fini(&mut (*font).glyph_names_array);
    _cairo_array_fini(&mut (*font).glyphs_array);
    free((*font).subrs as *mut libc::c_void);
    if !((*font).output).is_null() {
        status = _cairo_output_stream_destroy((*font).output);
    }
    free((*font).base.base_font as *mut libc::c_void);
    free((*font).scaled_subset_index_to_glyphs as *mut libc::c_void);
    free((*font).type1_subset_index_to_glyphs as *mut libc::c_void);
    free((*font).cleartext as *mut libc::c_void);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type1_subset_init(
    mut type1_subset: *mut cairo_type1_subset_t,
    mut name: *const libc::c_char,
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut hex_encode: cairo_bool_t,
) -> cairo_status_t {
    let mut font: cairo_type1_font_subset_t = cairo_type1_font_subset_t {
        scaled_font_subset: 0 as *mut cairo_scaled_font_subset_t,
        base: C2RustUnnamed_2 {
            font_id: 0,
            base_font: 0 as *mut libc::c_char,
            num_glyphs: 0,
            x_min: 0.,
            y_min: 0.,
            x_max: 0.,
            y_max: 0.,
            ascent: 0.,
            descent: 0.,
            units_per_em: 0.,
            data: 0 as *const libc::c_char,
            header_size: 0,
            data_size: 0,
            trailer_size: 0,
        },
        num_glyphs: 0,
        glyphs_array: cairo_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *mut libc::c_char,
        },
        glyphs: 0 as *mut glyph_data_t,
        glyph_names_array: cairo_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *mut libc::c_char,
        },
        glyph_names: 0 as *mut *mut libc::c_char,
        num_subrs: 0,
        subset_subrs: 0,
        subrs: 0 as *mut C2RustUnnamed_1,
        scaled_subset_index_to_glyphs: 0 as *mut libc::c_int,
        type1_subset_index_to_glyphs: 0 as *mut libc::c_int,
        output: 0 as *mut cairo_output_stream_t,
        contents: cairo_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *mut libc::c_char,
        },
        rd: 0 as *const libc::c_char,
        nd: 0 as *const libc::c_char,
        np: 0 as *const libc::c_char,
        lenIV: 0,
        type1_data: 0 as *mut libc::c_char,
        type1_length: 0,
        type1_end: 0 as *mut libc::c_char,
        header_segment: 0 as *mut libc::c_char,
        header_segment_size: 0,
        eexec_segment: 0 as *mut libc::c_char,
        eexec_segment_size: 0,
        eexec_segment_is_ascii: 0,
        cleartext: 0 as *mut libc::c_char,
        cleartext_end: 0 as *mut libc::c_char,
        header_size: 0,
        eexec_key: 0,
        hex_encode: 0,
        hex_column: 0,
        build_stack: C2RustUnnamed_0 {
            stack: [0.; 24],
            sp: 0,
        },
        ps_stack: C2RustUnnamed {
            stack: [0; 24],
            sp: 0,
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut is_synthetic: cairo_bool_t = 0;
    let mut length: libc::c_ulong = 0;
    let mut i: libc::c_uint = 0;
    let mut buf: [libc::c_char; 30] = [0; 30];
    let mut glyph: libc::c_int = 0;
    if ((*(*(*scaled_font_subset).scaled_font).backend).is_synthetic).is_some() {
        status = ((*(*(*scaled_font_subset).scaled_font).backend).is_synthetic)
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
    status = _cairo_type1_font_subset_init(&mut font, scaled_font_subset, hex_encode);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_subset_generate(
        &mut font as *mut cairo_type1_font_subset_t as *mut libc::c_void,
        name,
    );
    if !(status as u64 != 0) {
        if !(font.base.base_font).is_null() {
            let ref mut fresh54 = (*type1_subset).base_font;
            *fresh54 = strdup(font.base.base_font);
        } else {
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                b"CairoFont-%u-%u\0" as *const u8 as *const libc::c_char,
                (*scaled_font_subset).font_id,
                (*scaled_font_subset).subset_id,
            );
            let ref mut fresh55 = (*type1_subset).base_font;
            *fresh55 = strdup(buf.as_mut_ptr());
        }
        if !((*type1_subset).base_font).is_null() {
            let ref mut fresh56 = (*type1_subset).widths;
            *fresh56 = calloc(
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                (*scaled_font_subset).num_glyphs as libc::c_ulong,
            ) as *mut libc::c_double;
            if !((*type1_subset).widths).is_null() {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*font.scaled_font_subset).num_glyphs {
                    glyph = *(font.scaled_subset_index_to_glyphs).offset(i as isize);
                    *((*type1_subset).widths)
                        .offset(
                            i as isize,
                        ) = (*(font.glyphs).offset(glyph as isize)).width;
                    i = i.wrapping_add(1);
                }
                (*type1_subset).x_min = font.base.x_min;
                (*type1_subset).y_min = font.base.y_min;
                (*type1_subset).x_max = font.base.x_max;
                (*type1_subset).y_max = font.base.y_max;
                (*type1_subset).ascent = font.base.ascent;
                (*type1_subset).descent = font.base.descent;
                length = (font.base.header_size)
                    .wrapping_add(font.base.data_size)
                    .wrapping_add(font.base.trailer_size);
                let ref mut fresh57 = (*type1_subset).data;
                *fresh57 = (if length != 0 as libc::c_int as libc::c_ulong {
                    malloc(length)
                } else {
                    0 as *mut libc::c_void
                }) as *mut libc::c_char;
                if ((*type1_subset).data).is_null() {
                    free((*type1_subset).widths as *mut libc::c_void);
                } else {
                    memcpy(
                        (*type1_subset).data as *mut libc::c_void,
                        _cairo_array_index(
                            &mut font.contents,
                            0 as libc::c_int as libc::c_uint,
                        ),
                        length,
                    );
                    (*type1_subset).header_length = font.base.header_size;
                    (*type1_subset).data_length = font.base.data_size;
                    (*type1_subset).trailer_length = font.base.trailer_size;
                    return _cairo_type1_font_subset_fini(&mut font);
                }
            }
            free((*type1_subset).base_font as *mut libc::c_void);
        }
    }
    _cairo_type1_font_subset_fini(&mut font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type1_subset_fini(
    mut subset: *mut cairo_type1_subset_t,
) {
    free((*subset).base_font as *mut libc::c_void);
    free((*subset).widths as *mut libc::c_void);
    free((*subset).data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type1_scaled_font_is_type1(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_bool_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut length: libc::c_ulong = 0;
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    if ((*(*scaled_font).backend).load_type1_data).is_none() {
        return 0 as libc::c_int;
    }
    status = ((*(*scaled_font).backend).load_type1_data)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return 0 as libc::c_int;
    }
    if length > ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong {
        length = ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong;
    }
    status = ((*(*scaled_font).backend).load_type1_data)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_long,
        buf.as_mut_ptr(),
        &mut length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return 0 as libc::c_int;
    }
    return check_fontdata_is_type1(buf.as_mut_ptr(), length as libc::c_long);
}
