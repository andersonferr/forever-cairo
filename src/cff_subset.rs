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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_strtod(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
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
    fn _cairo_type2_charstrings_init(
        charstrings: *mut cairo_type2_charstrings_t,
        font_subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_status_t;
    fn _cairo_type2_charstrings_fini(charstrings: *mut cairo_type2_charstrings_t);
    fn _cairo_truetype_read_font_name(
        scaled_font: *mut cairo_scaled_font_t,
        ps_name: *mut *mut libc::c_char,
        font_name: *mut *mut libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_escape_ps_name(ps_name: *mut *mut libc::c_char) -> cairo_int_status_t;
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
pub type uint8_t = __uint8_t;
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
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cff_subset {
    pub family_name_utf8: *mut libc::c_char,
    pub ps_name: *mut libc::c_char,
    pub widths: *mut libc::c_double,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub data: *mut libc::c_char,
    pub data_length: libc::c_ulong,
}
pub type cairo_cff_subset_t = _cairo_cff_subset;
pub type cairo_cff_font_t = _cairo_cff_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cff_font {
    pub scaled_font_subset: *mut cairo_scaled_font_subset_t,
    pub backend: *const cairo_scaled_font_backend_t,
    pub data: *mut libc::c_uchar,
    pub data_length: libc::c_ulong,
    pub current_ptr: *mut libc::c_uchar,
    pub data_end: *mut libc::c_uchar,
    pub header: *mut cff_header_t,
    pub font_name: *mut libc::c_char,
    pub ps_name: *mut libc::c_char,
    pub top_dict: *mut cairo_hash_table_t,
    pub private_dict: *mut cairo_hash_table_t,
    pub strings_index: cairo_array_t,
    pub charstrings_index: cairo_array_t,
    pub global_sub_index: cairo_array_t,
    pub local_sub_index: cairo_array_t,
    pub charset: *mut libc::c_uchar,
    pub num_glyphs: libc::c_int,
    pub is_cid: cairo_bool_t,
    pub is_opentype: cairo_bool_t,
    pub units_per_em: libc::c_int,
    pub global_sub_bias: libc::c_int,
    pub local_sub_bias: libc::c_int,
    pub default_width: libc::c_double,
    pub nominal_width: libc::c_double,
    pub fdselect: *mut libc::c_int,
    pub num_fontdicts: libc::c_uint,
    pub fd_dict: *mut *mut cairo_hash_table_t,
    pub fd_private_dict: *mut *mut cairo_hash_table_t,
    pub fd_local_sub_index: *mut cairo_array_t,
    pub fd_local_sub_bias: *mut libc::c_int,
    pub fd_default_width: *mut libc::c_double,
    pub fd_nominal_width: *mut libc::c_double,
    pub subset_font_name: *mut libc::c_char,
    pub charstrings_subset_index: cairo_array_t,
    pub strings_subset_index: cairo_array_t,
    pub euro_sid: libc::c_int,
    pub fdselect_subset: *mut libc::c_int,
    pub num_subset_fontdicts: libc::c_uint,
    pub fd_subset_map: *mut libc::c_int,
    pub private_dict_offset: *mut libc::c_int,
    pub subset_subroutines: cairo_bool_t,
    pub global_subs_used: *mut cairo_bool_t,
    pub local_subs_used: *mut cairo_bool_t,
    pub fd_local_subs_used: *mut *mut cairo_bool_t,
    pub output: cairo_array_t,
    pub widths: *mut libc::c_int,
    pub x_min: libc::c_int,
    pub y_min: libc::c_int,
    pub x_max: libc::c_int,
    pub y_max: libc::c_int,
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
    pub type2_stack_size: libc::c_int,
    pub type2_stack_top_value: libc::c_int,
    pub type2_stack_top_is_int: cairo_bool_t,
    pub type2_num_hints: libc::c_int,
    pub type2_hintmask_bytes: libc::c_int,
    pub type2_nesting_level: libc::c_int,
    pub type2_seen_first_int: cairo_bool_t,
    pub type2_find_width: cairo_bool_t,
    pub type2_found_width: cairo_bool_t,
    pub type2_width: libc::c_int,
    pub type2_has_path: cairo_bool_t,
}
pub type cff_header_t = _cff_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cff_header {
    pub major: uint8_t,
    pub minor: uint8_t,
    pub header_size: uint8_t,
    pub offset_size: uint8_t,
}
pub type cff_index_element_t = _cff_index_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cff_index_element {
    pub is_copy: cairo_bool_t,
    pub data: *mut libc::c_uchar,
    pub length: libc::c_int,
}
pub type cff_dict_operator_t = _cff_dict_operator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cff_dict_operator {
    pub base: cairo_hash_entry_t,
    pub operator: libc::c_ushort,
    pub operand: *mut libc::c_uchar,
    pub operand_length: libc::c_int,
    pub operand_offset: libc::c_int,
}
pub type font_write_t = Option::<
    unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
>;
pub type dict_write_info_t = _dict_write_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dict_write_info {
    pub output: *mut cairo_array_t,
    pub status: cairo_status_t,
}
pub type font_read_t = Option::<
    unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_int_status_t,
>;
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
pub type cairo_type2_charstrings_t = _cairo_type2_charstrings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_type2_charstrings {
    pub widths: *mut libc::c_int,
    pub x_min: libc::c_long,
    pub y_min: libc::c_long,
    pub x_max: libc::c_long,
    pub y_max: libc::c_long,
    pub ascent: libc::c_long,
    pub descent: libc::c_long,
    pub charstrings: cairo_array_t,
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
unsafe extern "C" fn get_unaligned_be16(mut p: *const libc::c_uchar) -> uint16_t {
    return ((*p.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *p.offset(1 as libc::c_int as isize) as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn put_unaligned_be32(mut v: uint32_t, mut p: *mut libc::c_uchar) {
    *p
        .offset(
            0 as libc::c_int as isize,
        ) = (v >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *p
        .offset(
            1 as libc::c_int as isize,
        ) = (v >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *p
        .offset(
            2 as libc::c_int as isize,
        ) = (v >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *p
        .offset(
            3 as libc::c_int as isize,
        ) = (v & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
unsafe extern "C" fn encode_integer_max(
    mut p: *mut libc::c_uchar,
    mut i: libc::c_int,
) -> *mut libc::c_uchar {
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = 29 as libc::c_int as libc::c_uchar;
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = (i >> 24 as libc::c_int) as libc::c_uchar;
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = (i >> 16 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = (i >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = (i & 0xff as libc::c_int) as libc::c_uchar;
    return p;
}
unsafe extern "C" fn encode_integer(
    mut p: *mut libc::c_uchar,
    mut i: libc::c_int,
) -> *mut libc::c_uchar {
    if i >= -(107 as libc::c_int) && i <= 107 as libc::c_int {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = (i + 139 as libc::c_int) as libc::c_uchar;
    } else if i >= 108 as libc::c_int && i <= 1131 as libc::c_int {
        i -= 108 as libc::c_int;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ((i >> 8 as libc::c_int) + 247 as libc::c_int) as libc::c_uchar;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = (i & 0xff as libc::c_int) as libc::c_uchar;
    } else if i >= -(1131 as libc::c_int) && i <= -(108 as libc::c_int) {
        i = -i - 108 as libc::c_int;
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = ((i >> 8 as libc::c_int) + 251 as libc::c_int) as libc::c_uchar;
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = (i & 0xff as libc::c_int) as libc::c_uchar;
    } else if i >= -(32768 as libc::c_int) && i <= 32767 as libc::c_int {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = 28 as libc::c_int as libc::c_uchar;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = (i >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = (i & 0xff as libc::c_int) as libc::c_uchar;
    } else {
        p = encode_integer_max(p, i);
    }
    return p;
}
unsafe extern "C" fn decode_integer(
    mut p: *mut libc::c_uchar,
    mut integer: *mut libc::c_int,
) -> *mut libc::c_uchar {
    if *p as libc::c_int == 28 as libc::c_int {
        *integer = ((*p.offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *p.offset(2 as libc::c_int as isize) as libc::c_int)
            as int16_t as libc::c_int;
        p = p.offset(3 as libc::c_int as isize);
    } else if *p as libc::c_int == 29 as libc::c_int {
        *integer = ((*p.offset(1 as libc::c_int as isize) as uint32_t)
            << 24 as libc::c_int
            | ((*p.offset(2 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int) as libc::c_uint
            | ((*p.offset(3 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
                as libc::c_uint | *p.offset(4 as libc::c_int as isize) as libc::c_uint)
            as int32_t;
        p = p.offset(5 as libc::c_int as isize);
    } else if *p as libc::c_int >= 32 as libc::c_int
        && *p as libc::c_int <= 246 as libc::c_int
    {
        let fresh13 = p;
        p = p.offset(1);
        *integer = *fresh13 as libc::c_int - 139 as libc::c_int;
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
        *integer = 0 as libc::c_int;
        p = p.offset(1 as libc::c_int as isize);
    }
    return p;
}
unsafe extern "C" fn decode_nibble(
    mut n: libc::c_int,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    match n {
        10 => {
            let fresh14 = buf;
            buf = buf.offset(1);
            *fresh14 = '.' as i32 as libc::c_char;
        }
        11 => {
            let fresh15 = buf;
            buf = buf.offset(1);
            *fresh15 = 'E' as i32 as libc::c_char;
        }
        12 => {
            let fresh16 = buf;
            buf = buf.offset(1);
            *fresh16 = 'E' as i32 as libc::c_char;
            let fresh17 = buf;
            buf = buf.offset(1);
            *fresh17 = '-' as i32 as libc::c_char;
        }
        13 => {
            let fresh18 = buf;
            buf = buf.offset(1);
            *fresh18 = '-' as i32 as libc::c_char;
        }
        14 => {
            let fresh19 = buf;
            buf = buf.offset(1);
            *fresh19 = '-' as i32 as libc::c_char;
        }
        15 => {}
        _ => {
            let fresh20 = buf;
            buf = buf.offset(1);
            *fresh20 = ('0' as i32 + n) as libc::c_char;
        }
    }
    return buf;
}
unsafe extern "C" fn decode_real(
    mut p: *mut libc::c_uchar,
    mut real: *mut libc::c_double,
) -> *mut libc::c_uchar {
    let mut buffer: [libc::c_char; 100] = [0; 100];
    let mut buf: *mut libc::c_char = buffer.as_mut_ptr();
    let mut buf_end: *mut libc::c_char = buffer
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as isize);
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    p = p.offset(1);
    while buf.offset(2 as libc::c_int as isize) < buf_end {
        n = *p as libc::c_int >> 4 as libc::c_int;
        buf = decode_nibble(n, buf);
        n = *p as libc::c_int & 0xf as libc::c_int;
        buf = decode_nibble(n, buf);
        if *p as libc::c_int & 0xf as libc::c_int == 0xf as libc::c_int {
            p = p.offset(1);
            break;
        } else {
            p = p.offset(1);
        }
    }
    *buf = 0 as libc::c_int as libc::c_char;
    *real = _cairo_strtod(buffer.as_mut_ptr(), &mut end);
    return p;
}
unsafe extern "C" fn decode_number(
    mut p: *mut libc::c_uchar,
    mut number: *mut libc::c_double,
) -> *mut libc::c_uchar {
    if *p as libc::c_int == 30 as libc::c_int {
        p = decode_real(p, number);
    } else {
        let mut i: libc::c_int = 0;
        p = decode_integer(p, &mut i);
        *number = i as libc::c_double;
    }
    return p;
}
unsafe extern "C" fn decode_operator(
    mut p: *mut libc::c_uchar,
    mut operator: *mut libc::c_ushort,
) -> *mut libc::c_uchar {
    let mut op: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let fresh21 = p;
    p = p.offset(1);
    op = *fresh21 as libc::c_ushort;
    if op as libc::c_int == 12 as libc::c_int {
        op = ((op as libc::c_int) << 8 as libc::c_int) as libc::c_ushort;
        let fresh22 = p;
        p = p.offset(1);
        op = (op as libc::c_int | *fresh22 as libc::c_int) as libc::c_ushort;
    }
    *operator = op;
    return p;
}
unsafe extern "C" fn operand_length(mut p: *mut libc::c_uchar) -> libc::c_int {
    let mut begin: *mut libc::c_uchar = p;
    if *p as libc::c_int == 28 as libc::c_int {
        return 3 as libc::c_int;
    }
    if *p as libc::c_int == 29 as libc::c_int {
        return 5 as libc::c_int;
    }
    if *p as libc::c_int >= 32 as libc::c_int && *p as libc::c_int <= 246 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if *p as libc::c_int >= 247 as libc::c_int && *p as libc::c_int <= 254 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if *p as libc::c_int == 30 as libc::c_int {
        while *p as libc::c_int & 0xf as libc::c_int != 0xf as libc::c_int {
            p = p.offset(1);
        }
        return (p.offset_from(begin) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn encode_index_offset(
    mut p: *mut libc::c_uchar,
    mut offset_size: libc::c_int,
    mut offset: libc::c_ulong,
) -> *mut libc::c_uchar {
    loop {
        offset_size -= 1;
        if !(offset_size >= 0 as libc::c_int) {
            break;
        }
        *p
            .offset(
                offset_size as isize,
            ) = (offset & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        offset >>= 8 as libc::c_int;
    }
    return p.offset(offset_size as isize);
}
unsafe extern "C" fn decode_index_offset(
    mut p: *mut libc::c_uchar,
    mut off_size: libc::c_int,
) -> libc::c_ulong {
    let mut offset: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    loop {
        let fresh23 = off_size;
        off_size = off_size - 1;
        if !(fresh23 > 0 as libc::c_int) {
            break;
        }
        let fresh24 = p;
        p = p.offset(1);
        offset = offset
            .wrapping_mul(256 as libc::c_int as libc::c_ulong)
            .wrapping_add(*fresh24 as libc::c_ulong);
    }
    return offset;
}
unsafe extern "C" fn cff_index_init(mut index: *mut cairo_array_t) {
    _cairo_array_init(
        index,
        ::std::mem::size_of::<cff_index_element_t>() as libc::c_ulong as libc::c_uint,
    );
}
unsafe extern "C" fn cff_index_read(
    mut index: *mut cairo_array_t,
    mut ptr: *mut *mut libc::c_uchar,
    mut end_ptr: *mut libc::c_uchar,
) -> cairo_int_status_t {
    let mut element: cff_index_element_t = cff_index_element_t {
        is_copy: 0,
        data: 0 as *mut libc::c_uchar,
        length: 0,
    };
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut offset_size: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0 as libc::c_int;
    p = *ptr;
    if p.offset(2 as libc::c_int as isize) > end_ptr {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    count = get_unaligned_be16(p) as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    if count > 0 as libc::c_int {
        let fresh25 = p;
        p = p.offset(1);
        offset_size = *fresh25 as libc::c_int;
        if p.offset(((count + 1 as libc::c_int) * offset_size) as isize) > end_ptr {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        data = p
            .offset((offset_size * (count + 1 as libc::c_int)) as isize)
            .offset(-(1 as libc::c_int as isize));
        start = decode_index_offset(p, offset_size) as libc::c_int;
        p = p.offset(offset_size as isize);
        i = 0 as libc::c_int;
        while i < count {
            end = decode_index_offset(p, offset_size) as libc::c_int;
            p = p.offset(offset_size as isize);
            if p > end_ptr {
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            element.length = end - start;
            element.is_copy = 0 as libc::c_int;
            element.data = data.offset(start as isize);
            status = _cairo_array_append(
                index,
                &mut element as *mut cff_index_element_t as *const libc::c_void,
            );
            if status as u64 != 0 {
                return status as cairo_int_status_t;
            }
            start = end;
            i += 1;
        }
        p = data.offset(end as isize);
    }
    *ptr = p;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cff_index_write(
    mut index: *mut cairo_array_t,
    mut output: *mut cairo_array_t,
) -> cairo_status_t {
    let mut offset_size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut num_elem: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut count: uint16_t = 0;
    let mut buf: [libc::c_uchar; 5] = [0; 5];
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    num_elem = _cairo_array_num_elements(index) as libc::c_int;
    count = cpu_to_be16(num_elem as uint16_t);
    status = _cairo_array_append_multiple(
        output,
        &mut count as *mut uint16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    if num_elem == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    offset = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elem {
        element = _cairo_array_index(index, i as libc::c_uint)
            as *mut cff_index_element_t;
        offset += (*element).length;
        i += 1;
    }
    if offset < 0x100 as libc::c_int {
        offset_size = 1 as libc::c_int;
    } else if offset < 0x10000 as libc::c_int {
        offset_size = 2 as libc::c_int;
    } else if offset < 0x1000000 as libc::c_int {
        offset_size = 3 as libc::c_int;
    } else {
        offset_size = 4 as libc::c_int;
    }
    buf[0 as libc::c_int as usize] = offset_size as libc::c_uchar;
    status = _cairo_array_append(output, buf.as_mut_ptr() as *const libc::c_void);
    if status as u64 != 0 {
        return status;
    }
    offset = 1 as libc::c_int;
    encode_index_offset(buf.as_mut_ptr(), offset_size, offset as libc::c_ulong);
    status = _cairo_array_append_multiple(
        output,
        buf.as_mut_ptr() as *const libc::c_void,
        offset_size as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    i = 0 as libc::c_int;
    while i < num_elem {
        element = _cairo_array_index(index, i as libc::c_uint)
            as *mut cff_index_element_t;
        offset += (*element).length;
        encode_index_offset(buf.as_mut_ptr(), offset_size, offset as libc::c_ulong);
        status = _cairo_array_append_multiple(
            output,
            buf.as_mut_ptr() as *const libc::c_void,
            offset_size as libc::c_uint,
        );
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < num_elem {
        element = _cairo_array_index(index, i as libc::c_uint)
            as *mut cff_index_element_t;
        if (*element).length > 0 as libc::c_int {
            status = _cairo_array_append_multiple(
                output,
                (*element).data as *const libc::c_void,
                (*element).length as libc::c_uint,
            );
        }
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cff_index_set_object(
    mut index: *mut cairo_array_t,
    mut obj_index: libc::c_int,
    mut object: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    element = _cairo_array_index(index, obj_index as libc::c_uint)
        as *mut cff_index_element_t;
    if (*element).is_copy != 0 {
        free((*element).data as *mut libc::c_void);
    }
    let ref mut fresh26 = (*element).data;
    *fresh26 = object;
    (*element).length = length;
    (*element).is_copy = 0 as libc::c_int;
}
unsafe extern "C" fn cff_index_append(
    mut index: *mut cairo_array_t,
    mut object: *mut libc::c_uchar,
    mut length: libc::c_int,
) -> cairo_status_t {
    let mut element: cff_index_element_t = cff_index_element_t {
        is_copy: 0,
        data: 0 as *mut libc::c_uchar,
        length: 0,
    };
    element.length = length;
    element.is_copy = 0 as libc::c_int;
    element.data = object;
    return _cairo_array_append(
        index,
        &mut element as *mut cff_index_element_t as *const libc::c_void,
    );
}
unsafe extern "C" fn cff_index_append_copy(
    mut index: *mut cairo_array_t,
    mut object: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut element: cff_index_element_t = cff_index_element_t {
        is_copy: 0,
        data: 0 as *mut libc::c_uchar,
        length: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    element.length = length as libc::c_int;
    element.is_copy = 1 as libc::c_int;
    element
        .data = (if element.length != 0 as libc::c_int {
        malloc(element.length as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if (element.data).is_null() && length != 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    memcpy(
        element.data as *mut libc::c_void,
        object as *const libc::c_void,
        element.length as libc::c_ulong,
    );
    status = _cairo_array_append(
        index,
        &mut element as *mut cff_index_element_t as *const libc::c_void,
    );
    if status as u64 != 0 {
        free(element.data as *mut libc::c_void);
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cff_index_fini(mut index: *mut cairo_array_t) {
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < _cairo_array_num_elements(index) {
        element = _cairo_array_index(index, i) as *mut cff_index_element_t;
        if (*element).is_copy != 0 && !((*element).data).is_null() {
            free((*element).data as *mut libc::c_void);
        }
        i = i.wrapping_add(1);
    }
    _cairo_array_fini(index);
}
unsafe extern "C" fn _cairo_cff_dict_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut op_a: *const cff_dict_operator_t = key_a as *const cff_dict_operator_t;
    let mut op_b: *const cff_dict_operator_t = key_b as *const cff_dict_operator_t;
    return ((*op_a).operator as libc::c_int == (*op_b).operator as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn cff_dict_init(
    mut dict: *mut *mut cairo_hash_table_t,
) -> cairo_status_t {
    *dict = _cairo_hash_table_create(
        Some(
            _cairo_cff_dict_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if (*dict).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_dict_init_key(
    mut key: *mut cff_dict_operator_t,
    mut operator: libc::c_int,
) {
    (*key).base.hash = operator as libc::c_ulong;
    (*key).operator = operator as libc::c_ushort;
}
unsafe extern "C" fn cff_dict_create_operator(
    mut operator: libc::c_int,
    mut operand: *mut libc::c_uchar,
    mut size: libc::c_int,
    mut out: *mut *mut cff_dict_operator_t,
) -> cairo_status_t {
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    op = (if ::std::mem::size_of::<cff_dict_operator_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cff_dict_operator_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cff_dict_operator_t;
    if op.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_dict_init_key(op, operator);
    if size != 0 as libc::c_int {
        let ref mut fresh27 = (*op).operand;
        *fresh27 = (if size != 0 as libc::c_int {
            malloc(size as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_uchar;
        if ((*op).operand).is_null() {
            free(op as *mut libc::c_void);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        memcpy(
            (*op).operand as *mut libc::c_void,
            operand as *const libc::c_void,
            size as libc::c_ulong,
        );
    } else {
        let ref mut fresh28 = (*op).operand;
        *fresh28 = 0 as *mut libc::c_uchar;
        if operator != 0x6 as libc::c_int && operator != 0x7 as libc::c_int
            && operator != 0x8 as libc::c_int && operator != 0x9 as libc::c_int
            && operator != 0xc0c as libc::c_int && operator != 0xc0d as libc::c_int
        {
            free(op as *mut libc::c_void);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    (*op).operand_length = size;
    (*op).operand_offset = -(1 as libc::c_int);
    *out = op;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cff_dict_read(
    mut dict: *mut cairo_hash_table_t,
    mut p: *mut libc::c_uchar,
    mut dict_size: libc::c_int,
) -> cairo_status_t {
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut operands: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    let mut operator: libc::c_ushort = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut size: libc::c_int = 0;
    end = p.offset(dict_size as isize);
    _cairo_array_init(&mut operands, 1 as libc::c_int as libc::c_uint);
    while p < end {
        size = operand_length(p);
        if size != 0 as libc::c_int {
            status = _cairo_array_append_multiple(
                &mut operands,
                p as *const libc::c_void,
                size as libc::c_uint,
            );
            if status as u64 != 0 {
                break;
            }
            p = p.offset(size as isize);
        } else {
            p = decode_operator(p, &mut operator);
            status = cff_dict_create_operator(
                operator as libc::c_int,
                _cairo_array_index(&mut operands, 0 as libc::c_int as libc::c_uint)
                    as *mut libc::c_uchar,
                _cairo_array_num_elements(&mut operands) as libc::c_int,
                &mut op,
            );
            if status as u64 != 0 {
                break;
            }
            status = _cairo_hash_table_insert(dict, &mut (*op).base);
            if status as u64 != 0 {
                break;
            }
            _cairo_array_truncate(&mut operands, 0 as libc::c_int as libc::c_uint);
        }
    }
    _cairo_array_fini(&mut operands);
    return status;
}
unsafe extern "C" fn cff_dict_remove(
    mut dict: *mut cairo_hash_table_t,
    mut operator: libc::c_ushort,
) {
    let mut key: cff_dict_operator_t = cff_dict_operator_t {
        base: cairo_hash_entry_t { hash: 0 },
        operator: 0,
        operand: 0 as *mut libc::c_uchar,
        operand_length: 0,
        operand_offset: 0,
    };
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    _cairo_dict_init_key(&mut key, operator as libc::c_int);
    op = _cairo_hash_table_lookup(dict, &mut key.base) as *mut cff_dict_operator_t;
    if !op.is_null() {
        free((*op).operand as *mut libc::c_void);
        _cairo_hash_table_remove(dict, op as *mut cairo_hash_entry_t);
        free(op as *mut libc::c_void);
    }
}
unsafe extern "C" fn cff_dict_get_operands(
    mut dict: *mut cairo_hash_table_t,
    mut operator: libc::c_ushort,
    mut size: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut key: cff_dict_operator_t = cff_dict_operator_t {
        base: cairo_hash_entry_t { hash: 0 },
        operator: 0,
        operand: 0 as *mut libc::c_uchar,
        operand_length: 0,
        operand_offset: 0,
    };
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    _cairo_dict_init_key(&mut key, operator as libc::c_int);
    op = _cairo_hash_table_lookup(dict, &mut key.base) as *mut cff_dict_operator_t;
    if !op.is_null() {
        *size = (*op).operand_length;
        return (*op).operand;
    }
    return 0 as *mut libc::c_uchar;
}
unsafe extern "C" fn cff_dict_set_operands(
    mut dict: *mut cairo_hash_table_t,
    mut operator: libc::c_ushort,
    mut operand: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> cairo_status_t {
    let mut key: cff_dict_operator_t = cff_dict_operator_t {
        base: cairo_hash_entry_t { hash: 0 },
        operator: 0,
        operand: 0 as *mut libc::c_uchar,
        operand_length: 0,
        operand_offset: 0,
    };
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_dict_init_key(&mut key, operator as libc::c_int);
    op = _cairo_hash_table_lookup(dict, &mut key.base) as *mut cff_dict_operator_t;
    if !op.is_null() {
        free((*op).operand as *mut libc::c_void);
        let ref mut fresh29 = (*op).operand;
        *fresh29 = (if size != 0 as libc::c_int {
            malloc(size as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_uchar;
        if ((*op).operand).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        memcpy(
            (*op).operand as *mut libc::c_void,
            operand as *const libc::c_void,
            size as libc::c_ulong,
        );
        (*op).operand_length = size;
    } else {
        status = cff_dict_create_operator(
            operator as libc::c_int,
            operand,
            size,
            &mut op,
        );
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_hash_table_insert(dict, &mut (*op).base);
        if status as u64 != 0 {
            return status;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cff_dict_get_location(
    mut dict: *mut cairo_hash_table_t,
    mut operator: libc::c_ushort,
    mut size: *mut libc::c_int,
) -> libc::c_int {
    let mut key: cff_dict_operator_t = cff_dict_operator_t {
        base: cairo_hash_entry_t { hash: 0 },
        operator: 0,
        operand: 0 as *mut libc::c_uchar,
        operand_length: 0,
        operand_offset: 0,
    };
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    _cairo_dict_init_key(&mut key, operator as libc::c_int);
    op = _cairo_hash_table_lookup(dict, &mut key.base) as *mut cff_dict_operator_t;
    if !op.is_null() {
        *size = (*op).operand_length;
        return (*op).operand_offset;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cairo_dict_write_operator(
    mut op: *mut cff_dict_operator_t,
    mut write_info: *mut dict_write_info_t,
) {
    let mut data: libc::c_uchar = 0;
    (*op)
        .operand_offset = _cairo_array_num_elements((*write_info).output) as libc::c_int;
    (*write_info)
        .status = _cairo_array_append_multiple(
        (*write_info).output,
        (*op).operand as *const libc::c_void,
        (*op).operand_length as libc::c_uint,
    );
    if (*write_info).status as u64 != 0 {
        return;
    }
    if (*op).operator as libc::c_int & 0xff00 as libc::c_int != 0 {
        data = ((*op).operator as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        (*write_info)
            .status = _cairo_array_append(
            (*write_info).output,
            &mut data as *mut libc::c_uchar as *const libc::c_void,
        );
        if (*write_info).status as u64 != 0 {
            return;
        }
    }
    data = ((*op).operator as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    (*write_info)
        .status = _cairo_array_append(
        (*write_info).output,
        &mut data as *mut libc::c_uchar as *const libc::c_void,
    );
}
unsafe extern "C" fn _cairo_dict_collect(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut write_info: *mut dict_write_info_t = closure as *mut dict_write_info_t;
    let mut op: *mut cff_dict_operator_t = entry as *mut cff_dict_operator_t;
    if (*write_info).status as u64 != 0 {
        return;
    }
    if (*op).operator as libc::c_int != 0xc1e as libc::c_int {
        cairo_dict_write_operator(op, write_info);
    }
}
unsafe extern "C" fn cff_dict_write(
    mut dict: *mut cairo_hash_table_t,
    mut output: *mut cairo_array_t,
) -> cairo_status_t {
    let mut write_info: dict_write_info_t = dict_write_info_t {
        output: 0 as *mut cairo_array_t,
        status: CAIRO_STATUS_SUCCESS,
    };
    let mut key: cff_dict_operator_t = cff_dict_operator_t {
        base: cairo_hash_entry_t { hash: 0 },
        operator: 0,
        operand: 0 as *mut libc::c_uchar,
        operand_length: 0,
        operand_offset: 0,
    };
    let mut op: *mut cff_dict_operator_t = 0 as *mut cff_dict_operator_t;
    write_info.output = output;
    write_info.status = CAIRO_STATUS_SUCCESS;
    _cairo_dict_init_key(&mut key, 0xc1e as libc::c_int);
    op = _cairo_hash_table_lookup(dict, &mut key.base) as *mut cff_dict_operator_t;
    if !op.is_null() {
        cairo_dict_write_operator(op, &mut write_info);
    }
    _cairo_hash_table_foreach(
        dict,
        Some(
            _cairo_dict_collect
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        &mut write_info as *mut dict_write_info_t as *mut libc::c_void,
    );
    return write_info.status;
}
unsafe extern "C" fn _cff_dict_entry_pluck(
    mut _entry: *mut libc::c_void,
    mut dict: *mut libc::c_void,
) {
    let mut entry: *mut cff_dict_operator_t = _entry as *mut cff_dict_operator_t;
    _cairo_hash_table_remove(dict as *mut cairo_hash_table_t, &mut (*entry).base);
    free((*entry).operand as *mut libc::c_void);
    free(entry as *mut libc::c_void);
}
unsafe extern "C" fn cff_dict_fini(mut dict: *mut cairo_hash_table_t) {
    _cairo_hash_table_foreach(
        dict,
        Some(
            _cff_dict_entry_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        dict as *mut libc::c_void,
    );
    _cairo_hash_table_destroy(dict);
}
unsafe extern "C" fn cairo_cff_font_read_header(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    if (*font).data_length < ::std::mem::size_of::<cff_header_t>() as libc::c_ulong {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    let ref mut fresh30 = (*font).header;
    *fresh30 = (*font).data as *mut cff_header_t;
    let ref mut fresh31 = (*font).current_ptr;
    *fresh31 = ((*font).data)
        .offset((*(*font).header).header_size as libc::c_int as isize);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_read_name(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut index: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    cff_index_init(&mut index);
    status = cff_index_read(&mut index, &mut (*font).current_ptr, (*font).data_end);
    if (*font).is_opentype == 0 {
        element = _cairo_array_index(&mut index, 0 as libc::c_int as libc::c_uint)
            as *mut cff_index_element_t;
        p = (*element).data;
        len = (*element).length;
        if len > 7 as libc::c_int
            && *p.offset(6 as libc::c_int as isize) as libc::c_int == '+' as i32
        {
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                if (*p.offset(i as isize) as libc::c_int) < 'A' as i32
                    || *p.offset(i as isize) as libc::c_int > 'Z' as i32
                {
                    break;
                }
                i += 1;
            }
            if i == 6 as libc::c_int {
                p = p.offset(7 as libc::c_int as isize);
                len -= 7 as libc::c_int;
            }
        }
        let ref mut fresh32 = (*font).ps_name;
        *fresh32 = (if len + 1 as libc::c_int != 0 as libc::c_int {
            malloc((len + 1 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if ((*font).ps_name).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        memcpy(
            (*font).ps_name as *mut libc::c_void,
            p as *const libc::c_void,
            len as libc::c_ulong,
        );
        *((*font).ps_name).offset(len as isize) = 0 as libc::c_int as libc::c_char;
        status = _cairo_escape_ps_name(&mut (*font).ps_name);
    }
    cff_index_fini(&mut index);
    return status;
}
unsafe extern "C" fn cairo_cff_font_read_private_dict(
    mut font: *mut cairo_cff_font_t,
    mut private_dict: *mut cairo_hash_table_t,
    mut local_sub_index: *mut cairo_array_t,
    mut local_sub_bias: *mut libc::c_int,
    mut local_subs_used: *mut *mut cairo_bool_t,
    mut default_width: *mut libc::c_double,
    mut nominal_width: *mut libc::c_double,
    mut ptr: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut buf: [libc::c_uchar; 10] = [0; 10];
    let mut end_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut operand: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut num_subs: libc::c_int = 0;
    status = cff_dict_read(private_dict, ptr, size) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    operand = cff_dict_get_operands(
        private_dict,
        0x13 as libc::c_int as libc::c_ushort,
        &mut i,
    );
    if !operand.is_null() {
        decode_integer(operand, &mut offset);
        p = ptr.offset(offset as isize);
        status = cff_index_read(local_sub_index, &mut p, (*font).data_end);
        if status as u64 != 0 {
            return status;
        }
        end_buf = encode_integer_max(buf.as_mut_ptr(), 0 as libc::c_int);
        status = cff_dict_set_operands(
            private_dict,
            0x13 as libc::c_int as libc::c_ushort,
            buf.as_mut_ptr(),
            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
    }
    *default_width = 0 as libc::c_int as libc::c_double;
    operand = cff_dict_get_operands(
        private_dict,
        0x14 as libc::c_int as libc::c_ushort,
        &mut i,
    );
    if !operand.is_null() {
        decode_number(operand, default_width);
    }
    *nominal_width = 0 as libc::c_int as libc::c_double;
    operand = cff_dict_get_operands(
        private_dict,
        0x15 as libc::c_int as libc::c_ushort,
        &mut i,
    );
    if !operand.is_null() {
        decode_number(operand, nominal_width);
    }
    num_subs = _cairo_array_num_elements(local_sub_index) as libc::c_int;
    *local_subs_used = calloc(
        num_subs as libc::c_ulong,
        ::std::mem::size_of::<cairo_bool_t>() as libc::c_ulong,
    ) as *mut cairo_bool_t;
    if (*local_subs_used).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if num_subs < 1240 as libc::c_int {
        *local_sub_bias = 107 as libc::c_int;
    } else if num_subs < 33900 as libc::c_int {
        *local_sub_bias = 1131 as libc::c_int;
    } else {
        *local_sub_bias = 32768 as libc::c_int;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_read_fdselect(
    mut font: *mut cairo_cff_font_t,
    mut p: *mut libc::c_uchar,
) -> cairo_int_status_t {
    let mut type_0: libc::c_int = 0;
    let mut num_ranges: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let ref mut fresh33 = (*font).fdselect;
    *fresh33 = calloc(
        (*font).num_glyphs as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).fdselect).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let fresh34 = p;
    p = p.offset(1);
    type_0 = *fresh34 as libc::c_int;
    if type_0 == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*font).num_glyphs {
            let fresh35 = p;
            p = p.offset(1);
            *((*font).fdselect).offset(i as isize) = *fresh35 as libc::c_int;
            i += 1;
        }
    } else if type_0 == 3 as libc::c_int {
        num_ranges = get_unaligned_be16(p) as libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < num_ranges {
            first = get_unaligned_be16(p) as libc::c_int;
            p = p.offset(2 as libc::c_int as isize);
            let fresh36 = p;
            p = p.offset(1);
            fd = *fresh36 as libc::c_int;
            last = get_unaligned_be16(p) as libc::c_int;
            if last > (*font).num_glyphs {
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            j = first;
            while j < last {
                *((*font).fdselect).offset(j as isize) = fd;
                j += 1;
            }
            i += 1;
        }
    } else {
        return CAIRO_INT_STATUS_UNSUPPORTED
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_read_cid_fontdict(
    mut font: *mut cairo_cff_font_t,
    mut ptr: *mut libc::c_uchar,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut index: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut i: libc::c_uint = 0;
    let mut size: libc::c_int = 0;
    let mut operand: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut buf: [libc::c_uchar; 100] = [0; 100];
    let mut end_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    cff_index_init(&mut index);
    status = cff_index_read(&mut index, &mut ptr, (*font).data_end);
    if !(status as u64 != 0) {
        (*font).num_fontdicts = _cairo_array_num_elements(&mut index);
        let ref mut fresh37 = (*font).fd_dict;
        *fresh37 = calloc(
            ::std::mem::size_of::<*mut cairo_hash_table_t>() as libc::c_ulong,
            (*font).num_fontdicts as libc::c_ulong,
        ) as *mut *mut cairo_hash_table_t;
        if ((*font).fd_dict).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        } else {
            let ref mut fresh38 = (*font).fd_private_dict;
            *fresh38 = calloc(
                ::std::mem::size_of::<*mut cairo_hash_table_t>() as libc::c_ulong,
                (*font).num_fontdicts as libc::c_ulong,
            ) as *mut *mut cairo_hash_table_t;
            if ((*font).fd_private_dict).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            } else {
                let ref mut fresh39 = (*font).fd_local_sub_index;
                *fresh39 = calloc(
                    ::std::mem::size_of::<cairo_array_t>() as libc::c_ulong,
                    (*font).num_fontdicts as libc::c_ulong,
                ) as *mut cairo_array_t;
                if ((*font).fd_local_sub_index).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                } else {
                    let ref mut fresh40 = (*font).fd_local_sub_bias;
                    *fresh40 = calloc(
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        (*font).num_fontdicts as libc::c_ulong,
                    ) as *mut libc::c_int;
                    if ((*font).fd_local_sub_bias).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                            as cairo_int_status_t;
                    } else {
                        let ref mut fresh41 = (*font).fd_local_subs_used;
                        *fresh41 = calloc(
                            ::std::mem::size_of::<*mut cairo_bool_t>() as libc::c_ulong,
                            (*font).num_fontdicts as libc::c_ulong,
                        ) as *mut *mut cairo_bool_t;
                        if ((*font).fd_local_subs_used).is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                as cairo_int_status_t;
                        } else {
                            let ref mut fresh42 = (*font).fd_default_width;
                            *fresh42 = calloc(
                                (*font).num_fontdicts as libc::c_ulong,
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            ) as *mut libc::c_double;
                            if ((*font).fd_default_width).is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                    as cairo_int_status_t;
                            } else {
                                let ref mut fresh43 = (*font).fd_nominal_width;
                                *fresh43 = calloc(
                                    (*font).num_fontdicts as libc::c_ulong,
                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                ) as *mut libc::c_double;
                                if ((*font).fd_nominal_width).is_null() {
                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                        as cairo_int_status_t;
                                } else {
                                    i = 0 as libc::c_int as libc::c_uint;
                                    loop {
                                        if !(i < (*font).num_fontdicts) {
                                            current_block = 2290177392965769716;
                                            break;
                                        }
                                        status = cff_dict_init(
                                            &mut *((*font).fd_dict).offset(i as isize),
                                        ) as cairo_int_status_t;
                                        if status as u64 != 0 {
                                            current_block = 5149889255931805157;
                                            break;
                                        }
                                        element = _cairo_array_index(&mut index, i)
                                            as *mut cff_index_element_t;
                                        status = cff_dict_read(
                                            *((*font).fd_dict).offset(i as isize),
                                            (*element).data,
                                            (*element).length,
                                        ) as cairo_int_status_t;
                                        if status as u64 != 0 {
                                            current_block = 5149889255931805157;
                                            break;
                                        }
                                        operand = cff_dict_get_operands(
                                            *((*font).fd_dict).offset(i as isize),
                                            0x12 as libc::c_int as libc::c_ushort,
                                            &mut size,
                                        );
                                        if operand.is_null() {
                                            status = CAIRO_INT_STATUS_UNSUPPORTED;
                                            current_block = 5149889255931805157;
                                            break;
                                        } else {
                                            operand = decode_integer(operand, &mut size);
                                            decode_integer(operand, &mut offset);
                                            status = cff_dict_init(
                                                &mut *((*font).fd_private_dict).offset(i as isize),
                                            ) as cairo_int_status_t;
                                            if status as u64 != 0 {
                                                current_block = 5149889255931805157;
                                                break;
                                            }
                                            cff_index_init(
                                                &mut *((*font).fd_local_sub_index).offset(i as isize),
                                            );
                                            status = cairo_cff_font_read_private_dict(
                                                font,
                                                *((*font).fd_private_dict).offset(i as isize),
                                                &mut *((*font).fd_local_sub_index).offset(i as isize),
                                                &mut *((*font).fd_local_sub_bias).offset(i as isize),
                                                &mut *((*font).fd_local_subs_used).offset(i as isize),
                                                &mut *((*font).fd_default_width).offset(i as isize),
                                                &mut *((*font).fd_nominal_width).offset(i as isize),
                                                ((*font).data).offset(offset as isize),
                                                size,
                                            );
                                            if status as u64 != 0 {
                                                current_block = 5149889255931805157;
                                                break;
                                            }
                                            end_buf = encode_integer_max(
                                                buf.as_mut_ptr(),
                                                0 as libc::c_int,
                                            );
                                            end_buf = encode_integer_max(end_buf, 0 as libc::c_int);
                                            status = cff_dict_set_operands(
                                                *((*font).fd_dict).offset(i as isize),
                                                0x12 as libc::c_int as libc::c_ushort,
                                                buf.as_mut_ptr(),
                                                end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                                    as libc::c_int,
                                            ) as cairo_int_status_t;
                                            if status as u64 != 0 {
                                                current_block = 5149889255931805157;
                                                break;
                                            }
                                            i = i.wrapping_add(1);
                                        }
                                    }
                                    match current_block {
                                        5149889255931805157 => {}
                                        _ => {
                                            status = CAIRO_STATUS_SUCCESS as libc::c_int
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
    cff_index_fini(&mut index);
    return status;
}
unsafe extern "C" fn cairo_cff_font_read_font_metrics(
    mut font: *mut cairo_cff_font_t,
    mut top_dict: *mut cairo_hash_table_t,
) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_int = 0;
    let mut x_min: libc::c_double = 0.;
    let mut y_min: libc::c_double = 0.;
    let mut x_max: libc::c_double = 0.;
    let mut y_max: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut yx: libc::c_double = 0.;
    let mut xy: libc::c_double = 0.;
    let mut yy: libc::c_double = 0.;
    x_min = 0.0f64;
    y_min = 0.0f64;
    x_max = 0.0f64;
    y_max = 0.0f64;
    p = cff_dict_get_operands(
        (*font).top_dict,
        0x5 as libc::c_int as libc::c_ushort,
        &mut size,
    );
    if !p.is_null() {
        end = p.offset(size as isize);
        if p < end {
            p = decode_number(p, &mut x_min);
        }
        if p < end {
            p = decode_number(p, &mut y_min);
        }
        if p < end {
            p = decode_number(p, &mut x_max);
        }
        if p < end {
            p = decode_number(p, &mut y_max);
        }
    }
    (*font).x_min = floor(x_min) as libc::c_int;
    (*font).y_min = floor(y_min) as libc::c_int;
    (*font).x_max = floor(x_max) as libc::c_int;
    (*font).y_max = floor(y_max) as libc::c_int;
    (*font).ascent = (*font).y_max;
    (*font).descent = (*font).y_min;
    xx = 0.001f64;
    yx = 0.0f64;
    xy = 0.0f64;
    yy = 0.001f64;
    p = cff_dict_get_operands(
        (*font).top_dict,
        0xc07 as libc::c_int as libc::c_ushort,
        &mut size,
    );
    if !p.is_null() {
        end = p.offset(size as isize);
        if p < end {
            p = decode_number(p, &mut xx);
        }
        if p < end {
            p = decode_number(p, &mut yx);
        }
        if p < end {
            p = decode_number(p, &mut xy);
        }
        if p < end {
            p = decode_number(p, &mut yy);
        }
    }
    (*font).units_per_em = _cairo_round(1.0f64 / fabs(yy)) as libc::c_int;
}
unsafe extern "C" fn cairo_cff_font_read_top_dict(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut index: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut buf: [libc::c_uchar; 20] = [0; 20];
    let mut end_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut operand: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    cff_index_init(&mut index);
    status = cff_index_read(&mut index, &mut (*font).current_ptr, (*font).data_end);
    if !(status as u64 != 0) {
        element = _cairo_array_index(&mut index, 0 as libc::c_int as libc::c_uint)
            as *mut cff_index_element_t;
        status = cff_dict_read((*font).top_dict, (*element).data, (*element).length)
            as cairo_int_status_t;
        if !(status as u64 != 0) {
            if !(cff_dict_get_operands(
                (*font).top_dict,
                0xc1e as libc::c_int as libc::c_ushort,
                &mut size,
            ))
                .is_null()
            {
                (*font).is_cid = 1 as libc::c_int;
            } else {
                (*font).is_cid = 0 as libc::c_int;
            }
            operand = cff_dict_get_operands(
                (*font).top_dict,
                0x11 as libc::c_int as libc::c_ushort,
                &mut size,
            );
            decode_integer(operand, &mut offset);
            p = ((*font).data).offset(offset as isize);
            status = cff_index_read(
                &mut (*font).charstrings_index,
                &mut p,
                (*font).data_end,
            );
            if !(status as u64 != 0) {
                (*font)
                    .num_glyphs = _cairo_array_num_elements(
                    &mut (*font).charstrings_index,
                ) as libc::c_int;
                if (*font).is_cid != 0 {
                    operand = cff_dict_get_operands(
                        (*font).top_dict,
                        0xf as libc::c_int as libc::c_ushort,
                        &mut size,
                    );
                    if operand.is_null() {
                        return CAIRO_INT_STATUS_UNSUPPORTED;
                    }
                    decode_integer(operand, &mut offset);
                    let ref mut fresh44 = (*font).charset;
                    *fresh44 = ((*font).data).offset(offset as isize);
                    if (*font).charset >= (*font).data_end {
                        return CAIRO_INT_STATUS_UNSUPPORTED;
                    }
                }
                if (*font).is_opentype == 0 {
                    cairo_cff_font_read_font_metrics(font, (*font).top_dict);
                }
                if (*font).is_cid != 0 {
                    operand = cff_dict_get_operands(
                        (*font).top_dict,
                        0xc25 as libc::c_int as libc::c_ushort,
                        &mut size,
                    );
                    decode_integer(operand, &mut offset);
                    status = cairo_cff_font_read_fdselect(
                        font,
                        ((*font).data).offset(offset as isize),
                    );
                    if status as u64 != 0 {
                        current_block = 7364782865091432669;
                    } else {
                        operand = cff_dict_get_operands(
                            (*font).top_dict,
                            0xc24 as libc::c_int as libc::c_ushort,
                            &mut size,
                        );
                        decode_integer(operand, &mut offset);
                        status = cairo_cff_font_read_cid_fontdict(
                            font,
                            ((*font).data).offset(offset as isize),
                        );
                        if status as u64 != 0 {
                            current_block = 7364782865091432669;
                        } else {
                            current_block = 4567019141635105728;
                        }
                    }
                } else {
                    operand = cff_dict_get_operands(
                        (*font).top_dict,
                        0x12 as libc::c_int as libc::c_ushort,
                        &mut size,
                    );
                    operand = decode_integer(operand, &mut size);
                    decode_integer(operand, &mut offset);
                    status = cairo_cff_font_read_private_dict(
                        font,
                        (*font).private_dict,
                        &mut (*font).local_sub_index,
                        &mut (*font).local_sub_bias,
                        &mut (*font).local_subs_used,
                        &mut (*font).default_width,
                        &mut (*font).nominal_width,
                        ((*font).data).offset(offset as isize),
                        size,
                    );
                    if status as u64 != 0 {
                        current_block = 7364782865091432669;
                    } else {
                        current_block = 4567019141635105728;
                    }
                }
                match current_block {
                    7364782865091432669 => {}
                    _ => {
                        end_buf = encode_integer_max(buf.as_mut_ptr(), 0 as libc::c_int);
                        status = cff_dict_set_operands(
                            (*font).top_dict,
                            0x11 as libc::c_int as libc::c_ushort,
                            buf.as_mut_ptr(),
                            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                as libc::c_int,
                        ) as cairo_int_status_t;
                        if !(status as u64 != 0) {
                            status = cff_dict_set_operands(
                                (*font).top_dict,
                                0xf as libc::c_int as libc::c_ushort,
                                buf.as_mut_ptr(),
                                end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                    as libc::c_int,
                            ) as cairo_int_status_t;
                            if !(status as u64 != 0) {
                                if (*(*font).scaled_font_subset).is_latin != 0 {
                                    status = cff_dict_set_operands(
                                        (*font).top_dict,
                                        0x10 as libc::c_int as libc::c_ushort,
                                        buf.as_mut_ptr(),
                                        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                            as libc::c_int,
                                    ) as cairo_int_status_t;
                                    if status as u64 != 0 {
                                        current_block = 7364782865091432669;
                                    } else {
                                        end_buf = encode_integer_max(end_buf, 0 as libc::c_int);
                                        cff_dict_set_operands(
                                            (*font).top_dict,
                                            0x12 as libc::c_int as libc::c_ushort,
                                            buf.as_mut_ptr(),
                                            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                                as libc::c_int,
                                        );
                                        if status as u64 != 0 {
                                            current_block = 7364782865091432669;
                                        } else {
                                            current_block = 4741994311446740739;
                                        }
                                    }
                                } else {
                                    status = cff_dict_set_operands(
                                        (*font).top_dict,
                                        0xc25 as libc::c_int as libc::c_ushort,
                                        buf.as_mut_ptr(),
                                        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                            as libc::c_int,
                                    ) as cairo_int_status_t;
                                    if status as u64 != 0 {
                                        current_block = 7364782865091432669;
                                    } else {
                                        status = cff_dict_set_operands(
                                            (*font).top_dict,
                                            0xc24 as libc::c_int as libc::c_ushort,
                                            buf.as_mut_ptr(),
                                            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long
                                                as libc::c_int,
                                        ) as cairo_int_status_t;
                                        if status as u64 != 0 {
                                            current_block = 7364782865091432669;
                                        } else {
                                            cff_dict_remove(
                                                (*font).top_dict,
                                                0x10 as libc::c_int as libc::c_ushort,
                                            );
                                            cff_dict_remove(
                                                (*font).top_dict,
                                                0x12 as libc::c_int as libc::c_ushort,
                                            );
                                            current_block = 4741994311446740739;
                                        }
                                    }
                                }
                                match current_block {
                                    7364782865091432669 => {}
                                    _ => {
                                        cff_dict_remove(
                                            (*font).top_dict,
                                            0xd as libc::c_int as libc::c_ushort,
                                        );
                                        cff_dict_remove(
                                            (*font).top_dict,
                                            0xe as libc::c_int as libc::c_ushort,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cff_index_fini(&mut index);
    return status;
}
unsafe extern "C" fn cairo_cff_font_read_strings(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    return cff_index_read(
        &mut (*font).strings_index,
        &mut (*font).current_ptr,
        (*font).data_end,
    );
}
unsafe extern "C" fn cairo_cff_font_read_global_subroutines(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut num_subs: libc::c_int = 0;
    status = cff_index_read(
        &mut (*font).global_sub_index,
        &mut (*font).current_ptr,
        (*font).data_end,
    );
    if status as u64 != 0 {
        return status;
    }
    num_subs = _cairo_array_num_elements(&mut (*font).global_sub_index) as libc::c_int;
    let ref mut fresh45 = (*font).global_subs_used;
    *fresh45 = calloc(
        num_subs as libc::c_ulong,
        ::std::mem::size_of::<cairo_bool_t>() as libc::c_ulong,
    ) as *mut cairo_bool_t;
    if ((*font).global_subs_used).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if num_subs < 1240 as libc::c_int {
        (*font).global_sub_bias = 107 as libc::c_int;
    } else if num_subs < 33900 as libc::c_int {
        (*font).global_sub_bias = 1131 as libc::c_int;
    } else {
        (*font).global_sub_bias = 32768 as libc::c_int;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
static mut font_read_funcs: [font_read_t; 5] = unsafe {
    [
        Some(
            cairo_cff_font_read_header
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_int_status_t,
        ),
        Some(
            cairo_cff_font_read_name
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_int_status_t,
        ),
        Some(
            cairo_cff_font_read_top_dict
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_int_status_t,
        ),
        Some(
            cairo_cff_font_read_strings
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_int_status_t,
        ),
        Some(
            cairo_cff_font_read_global_subroutines
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_int_status_t,
        ),
    ]
};
unsafe extern "C" fn cairo_cff_font_read_font(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (::std::mem::size_of::<[font_read_t; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<font_read_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        status = (font_read_funcs[i as usize]).expect("non-null function pointer")(font);
        if status as u64 != 0 {
            return status;
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_set_ros_strings(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut buf: [libc::c_uchar; 30] = [0; 30];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sid1: libc::c_int = 0;
    let mut sid2: libc::c_int = 0;
    let mut registry: *const libc::c_char = b"Adobe\0" as *const u8
        as *const libc::c_char;
    let mut ordering: *const libc::c_char = b"Identity\0" as *const u8
        as *const libc::c_char;
    sid1 = (391 as libc::c_int as libc::c_uint)
        .wrapping_add(_cairo_array_num_elements(&mut (*font).strings_subset_index))
        as libc::c_int;
    status = cff_index_append_copy(
        &mut (*font).strings_subset_index,
        registry as *mut libc::c_uchar,
        strlen(registry) as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    sid2 = (391 as libc::c_int as libc::c_uint)
        .wrapping_add(_cairo_array_num_elements(&mut (*font).strings_subset_index))
        as libc::c_int;
    status = cff_index_append_copy(
        &mut (*font).strings_subset_index,
        ordering as *mut libc::c_uchar,
        strlen(ordering) as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    p = encode_integer(buf.as_mut_ptr(), sid1);
    p = encode_integer(p, sid2);
    p = encode_integer(p, 0 as libc::c_int);
    status = cff_dict_set_operands(
        (*font).top_dict,
        0xc1e as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    p = encode_integer(
        buf.as_mut_ptr(),
        (*(*font).scaled_font_subset).num_glyphs as libc::c_int,
    );
    status = cff_dict_set_operands(
        (*font).top_dict,
        0xc22 as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_subset_dict_string(
    mut font: *mut cairo_cff_font_t,
    mut dict: *mut cairo_hash_table_t,
    mut operator: libc::c_int,
) -> cairo_status_t {
    let mut size: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sid: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 100] = [0; 100];
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    p = cff_dict_get_operands(dict, operator as libc::c_ushort, &mut size);
    if p.is_null() {
        return CAIRO_STATUS_SUCCESS;
    }
    decode_integer(p, &mut sid);
    if sid < 391 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    element = _cairo_array_index(
        &mut (*font).strings_index,
        (sid - 391 as libc::c_int) as libc::c_uint,
    ) as *mut cff_index_element_t;
    sid = (391 as libc::c_int as libc::c_uint)
        .wrapping_add(_cairo_array_num_elements(&mut (*font).strings_subset_index))
        as libc::c_int;
    status = cff_index_append(
        &mut (*font).strings_subset_index,
        (*element).data,
        (*element).length,
    );
    if status as u64 != 0 {
        return status;
    }
    p = encode_integer(buf.as_mut_ptr(), sid);
    status = cff_dict_set_operands(
        dict,
        operator as libc::c_ushort,
        buf.as_mut_ptr(),
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
static mut dict_strings: [libc::c_int; 9] = [
    0 as libc::c_int,
    0x1 as libc::c_int,
    0xc00 as libc::c_int,
    0x2 as libc::c_int,
    0x3 as libc::c_int,
    0x4 as libc::c_int,
    0xc15 as libc::c_int,
    0xc16 as libc::c_int,
    0xc26 as libc::c_int,
];
unsafe extern "C" fn cairo_cff_font_subset_dict_strings(
    mut font: *mut cairo_cff_font_t,
    mut dict: *mut cairo_hash_table_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (::std::mem::size_of::<[libc::c_int; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        status = cairo_cff_font_subset_dict_string(font, dict, dict_strings[i as usize]);
        if status as u64 != 0 {
            return status;
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn type2_decode_integer(
    mut p: *mut libc::c_uchar,
    mut integer: *mut libc::c_int,
) -> *mut libc::c_uchar {
    if *p as libc::c_int == 28 as libc::c_int {
        *integer = (*p.offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *p.offset(2 as libc::c_int as isize) as libc::c_int;
        p = p.offset(3 as libc::c_int as isize);
    } else if *p as libc::c_int <= 246 as libc::c_int {
        let fresh46 = p;
        p = p.offset(1);
        *integer = *fresh46 as libc::c_int - 139 as libc::c_int;
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
        *integer = ((*p.offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *p.offset(2 as libc::c_int as isize) as libc::c_int)
            as int16_t as libc::c_int;
        p = p.offset(5 as libc::c_int as isize);
    }
    return p;
}
unsafe extern "C" fn cairo_cff_parse_charstring(
    mut font: *mut cairo_cff_font_t,
    mut charstring: *mut libc::c_uchar,
    mut length: libc::c_int,
    mut glyph_id: libc::c_int,
    mut need_width: cairo_bool_t,
) -> cairo_status_t {
    let mut p: *mut libc::c_uchar = charstring;
    let mut end: *mut libc::c_uchar = charstring.offset(length as isize);
    let mut integer: libc::c_int = 0;
    let mut hint_bytes: libc::c_int = 0;
    let mut sub_num: libc::c_int = 0;
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut fd: libc::c_int = 0;
    while p < end {
        if *p as libc::c_int == 28 as libc::c_int
            || *p as libc::c_int >= 32 as libc::c_int
        {
            p = type2_decode_integer(p, &mut integer);
            let ref mut fresh47 = (*font).type2_stack_size;
            *fresh47 += 1;
            (*font).type2_stack_top_value = integer;
            (*font).type2_stack_top_is_int = 1 as libc::c_int;
            if (*font).type2_seen_first_int == 0 {
                (*font).type2_width = integer;
                (*font).type2_seen_first_int = 1 as libc::c_int;
            }
        } else if *p as libc::c_int == 0x1 as libc::c_int
            || *p as libc::c_int == 0x3 as libc::c_int
            || *p as libc::c_int == 0x12 as libc::c_int
            || *p as libc::c_int == 0x17 as libc::c_int
        {
            (*font).type2_stack_top_is_int = 0 as libc::c_int;
            (*font).type2_num_hints += (*font).type2_stack_size / 2 as libc::c_int;
            if (*font).type2_find_width != 0
                && (*font).type2_stack_size % 2 as libc::c_int != 0
            {
                (*font).type2_found_width = 1 as libc::c_int;
            }
            (*font).type2_stack_size = 0 as libc::c_int;
            (*font).type2_find_width = 0 as libc::c_int;
            p = p.offset(1);
        } else if *p as libc::c_int == 0x13 as libc::c_int
            || *p as libc::c_int == 0x14 as libc::c_int
        {
            if (*font).type2_hintmask_bytes == 0 as libc::c_int {
                (*font).type2_stack_top_is_int = 0 as libc::c_int;
                (*font).type2_num_hints += (*font).type2_stack_size / 2 as libc::c_int;
                if (*font).type2_find_width != 0
                    && (*font).type2_stack_size % 2 as libc::c_int != 0
                {
                    (*font).type2_found_width = 1 as libc::c_int;
                }
                (*font).type2_stack_size = 0 as libc::c_int;
                (*font).type2_find_width = 0 as libc::c_int;
                (*font)
                    .type2_hintmask_bytes = ((*font).type2_num_hints + 7 as libc::c_int)
                    / 8 as libc::c_int;
            }
            hint_bytes = (*font).type2_hintmask_bytes;
            p = p.offset(1);
            p = p.offset(hint_bytes as isize);
        } else if *p as libc::c_int == 0x15 as libc::c_int {
            if (*font).type2_find_width != 0
                && (*font).type2_stack_size > 2 as libc::c_int
            {
                (*font).type2_found_width = 1 as libc::c_int;
            }
            (*font).type2_stack_size = 0 as libc::c_int;
            (*font).type2_find_width = 0 as libc::c_int;
            (*font).type2_has_path = 1 as libc::c_int;
            p = p.offset(1);
        } else if *p as libc::c_int == 0x16 as libc::c_int
            || *p as libc::c_int == 0x4 as libc::c_int
        {
            if (*font).type2_find_width != 0
                && (*font).type2_stack_size > 1 as libc::c_int
            {
                (*font).type2_found_width = 1 as libc::c_int;
            }
            (*font).type2_stack_size = 0 as libc::c_int;
            (*font).type2_find_width = 0 as libc::c_int;
            (*font).type2_has_path = 1 as libc::c_int;
            p = p.offset(1);
        } else if *p as libc::c_int == 0xe as libc::c_int {
            if (*font).type2_has_path == 0 && (*font).type2_stack_size > 3 as libc::c_int
            {
                return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
            }
            if (*font).type2_find_width != 0
                && (*font).type2_stack_size > 0 as libc::c_int
            {
                (*font).type2_found_width = 1 as libc::c_int;
            }
            return CAIRO_STATUS_SUCCESS;
        } else {
            if *p as libc::c_int == 0xa as libc::c_int {
                if (*font).type2_stack_top_is_int == 0 {
                    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                }
                let ref mut fresh48 = (*font).type2_nesting_level;
                *fresh48 += 1;
                if *fresh48 > 10 as libc::c_int {
                    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                }
                p = p.offset(1);
                (*font).type2_stack_top_is_int = 0 as libc::c_int;
                let ref mut fresh49 = (*font).type2_stack_size;
                *fresh49 -= 1;
                if (*font).type2_find_width != 0
                    && (*font).type2_stack_size == 0 as libc::c_int
                {
                    (*font).type2_seen_first_int = 0 as libc::c_int;
                }
                if (*font).is_cid != 0 {
                    fd = *((*font).fdselect).offset(glyph_id as isize);
                    sub_num = (*font).type2_stack_top_value
                        + *((*font).fd_local_sub_bias).offset(fd as isize);
                    if sub_num
                        >= _cairo_array_num_elements(
                            &mut *((*font).fd_local_sub_index).offset(fd as isize),
                        ) as libc::c_int
                    {
                        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                            as cairo_status_t;
                    }
                    element = _cairo_array_index(
                        &mut *((*font).fd_local_sub_index).offset(fd as isize),
                        sub_num as libc::c_uint,
                    ) as *mut cff_index_element_t;
                    if *(*((*font).fd_local_subs_used).offset(fd as isize))
                        .offset(sub_num as isize) == 0
                    {
                        *(*((*font).fd_local_subs_used).offset(fd as isize))
                            .offset(sub_num as isize) = 1 as libc::c_int;
                        cairo_cff_parse_charstring(
                            font,
                            (*element).data,
                            (*element).length,
                            glyph_id,
                            need_width,
                        );
                    }
                } else {
                    sub_num = (*font).type2_stack_top_value + (*font).local_sub_bias;
                    if sub_num
                        >= _cairo_array_num_elements(&mut (*font).local_sub_index)
                            as libc::c_int
                    {
                        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                            as cairo_status_t;
                    }
                    element = _cairo_array_index(
                        &mut (*font).local_sub_index,
                        sub_num as libc::c_uint,
                    ) as *mut cff_index_element_t;
                    if *((*font).local_subs_used).offset(sub_num as isize) == 0
                        || need_width != 0 && (*font).type2_found_width == 0
                    {
                        *((*font).local_subs_used)
                            .offset(sub_num as isize) = 1 as libc::c_int;
                        cairo_cff_parse_charstring(
                            font,
                            (*element).data,
                            (*element).length,
                            glyph_id,
                            need_width,
                        );
                    }
                }
                let ref mut fresh50 = (*font).type2_nesting_level;
                *fresh50 -= 1;
            } else if *p as libc::c_int == 0x1d as libc::c_int {
                if (*font).type2_stack_top_is_int == 0 {
                    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                }
                let ref mut fresh51 = (*font).type2_nesting_level;
                *fresh51 += 1;
                if *fresh51 > 10 as libc::c_int {
                    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                }
                p = p.offset(1);
                let ref mut fresh52 = (*font).type2_stack_size;
                *fresh52 -= 1;
                (*font).type2_stack_top_is_int = 0 as libc::c_int;
                if (*font).type2_find_width != 0
                    && (*font).type2_stack_size == 0 as libc::c_int
                {
                    (*font).type2_seen_first_int = 0 as libc::c_int;
                }
                sub_num = (*font).type2_stack_top_value + (*font).global_sub_bias;
                if sub_num
                    >= _cairo_array_num_elements(&mut (*font).global_sub_index)
                        as libc::c_int
                {
                    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                }
                element = _cairo_array_index(
                    &mut (*font).global_sub_index,
                    sub_num as libc::c_uint,
                ) as *mut cff_index_element_t;
                if *((*font).global_subs_used).offset(sub_num as isize) == 0
                    || need_width != 0 && (*font).type2_found_width == 0
                {
                    *((*font).global_subs_used)
                        .offset(sub_num as isize) = 1 as libc::c_int;
                    cairo_cff_parse_charstring(
                        font,
                        (*element).data,
                        (*element).length,
                        glyph_id,
                        need_width,
                    );
                }
                let ref mut fresh53 = (*font).type2_nesting_level;
                *fresh53 -= 1;
            } else if *p as libc::c_int == 12 as libc::c_int {
                if need_width != 0 && (*font).type2_find_width != 0 {
                    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                }
                p = p.offset(2 as libc::c_int as isize);
                (*font).type2_stack_top_is_int = 0 as libc::c_int;
            } else {
                p = p.offset(1);
                (*font).type2_stack_top_is_int = 0 as libc::c_int;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_find_width_and_subroutines_used(
    mut font: *mut cairo_cff_font_t,
    mut charstring: *mut libc::c_uchar,
    mut length: libc::c_int,
    mut glyph_id: libc::c_int,
    mut subset_id: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut width: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    (*font).type2_stack_size = 0 as libc::c_int;
    (*font).type2_stack_top_value = 0 as libc::c_int;
    (*font).type2_stack_top_is_int = 0 as libc::c_int;
    (*font).type2_num_hints = 0 as libc::c_int;
    (*font).type2_hintmask_bytes = 0 as libc::c_int;
    (*font).type2_nesting_level = 0 as libc::c_int;
    (*font).type2_seen_first_int = 0 as libc::c_int;
    (*font).type2_find_width = 1 as libc::c_int;
    (*font).type2_found_width = 0 as libc::c_int;
    (*font).type2_width = 0 as libc::c_int;
    (*font).type2_has_path = 0 as libc::c_int;
    status = cairo_cff_parse_charstring(
        font,
        charstring,
        length,
        glyph_id,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*font).is_opentype == 0 {
        if (*font).is_cid != 0 {
            fd = *((*font).fdselect).offset(glyph_id as isize);
            if (*font).type2_found_width != 0 {
                width = (*((*font).fd_nominal_width).offset(fd as isize)
                    + (*font).type2_width as libc::c_double) as libc::c_int;
            } else {
                width = *((*font).fd_default_width).offset(fd as isize) as libc::c_int;
            }
        } else if (*font).type2_found_width != 0 {
            width = ((*font).nominal_width + (*font).type2_width as libc::c_double)
                as libc::c_int;
        } else {
            width = (*font).default_width as libc::c_int;
        }
        *((*font).widths).offset(subset_id as isize) = width;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_get_gid_for_cid(
    mut font: *mut cairo_cff_font_t,
    mut cid: libc::c_ulong,
    mut gid: *mut libc::c_ulong,
) -> cairo_int_status_t {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut first_gid: libc::c_ulong = 0;
    let mut first_cid: libc::c_ulong = 0;
    let mut num_left: libc::c_int = 0;
    let mut c: libc::c_ulong = 0;
    let mut g: libc::c_ulong = 0;
    if cid == 0 as libc::c_int as libc::c_ulong {
        *gid = 0 as libc::c_int as libc::c_ulong;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    match *((*font).charset).offset(0 as libc::c_int as isize) as libc::c_int {
        0 => {
            p = ((*font).charset).offset(1 as libc::c_int as isize);
            g = 1 as libc::c_int as libc::c_ulong;
            while g <= (*font).num_glyphs as libc::c_uint as libc::c_ulong
                && p < (*font).data_end
            {
                c = get_unaligned_be16(p) as libc::c_ulong;
                if c == cid {
                    *gid = g;
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
                g = g.wrapping_add(1);
                p = p.offset(2 as libc::c_int as isize);
            }
        }
        1 => {
            first_gid = 1 as libc::c_int as libc::c_ulong;
            p = ((*font).charset).offset(1 as libc::c_int as isize);
            while first_gid <= (*font).num_glyphs as libc::c_uint as libc::c_ulong
                && p.offset(2 as libc::c_int as isize) < (*font).data_end
            {
                first_cid = get_unaligned_be16(p) as libc::c_ulong;
                num_left = *p.offset(2 as libc::c_int as isize) as libc::c_int;
                if cid >= first_cid
                    && cid <= first_cid.wrapping_add(num_left as libc::c_ulong)
                {
                    *gid = first_gid.wrapping_add(cid).wrapping_sub(first_cid);
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
                first_gid = first_gid
                    .wrapping_add((num_left + 1 as libc::c_int) as libc::c_ulong);
                p = p.offset(3 as libc::c_int as isize);
            }
        }
        2 => {
            first_gid = 1 as libc::c_int as libc::c_ulong;
            p = ((*font).charset).offset(1 as libc::c_int as isize);
            while first_gid <= (*font).num_glyphs as libc::c_uint as libc::c_ulong
                && p.offset(3 as libc::c_int as isize) < (*font).data_end
            {
                first_cid = get_unaligned_be16(p) as libc::c_ulong;
                num_left = get_unaligned_be16(p.offset(2 as libc::c_int as isize))
                    as libc::c_int;
                if cid >= first_cid
                    && cid <= first_cid.wrapping_add(num_left as libc::c_ulong)
                {
                    *gid = first_gid.wrapping_add(cid).wrapping_sub(first_cid);
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
                first_gid = first_gid
                    .wrapping_add((num_left + 1 as libc::c_int) as libc::c_ulong);
                p = p.offset(4 as libc::c_int as isize);
            }
        }
        _ => {}
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
unsafe extern "C" fn cairo_cff_font_subset_charstrings_and_subroutines(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut i: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut glyph: libc::c_ulong = 0;
    let mut cid: libc::c_ulong = 0;
    (*font).subset_subroutines = 1 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        if (*font).is_cid != 0 && (*font).is_opentype == 0 {
            cid = *((*(*font).scaled_font_subset).glyphs).offset(i as isize);
            status = cairo_cff_font_get_gid_for_cid(font, cid, &mut glyph);
            if status as u64 != 0 {
                return status;
            }
        } else {
            glyph = *((*(*font).scaled_font_subset).glyphs).offset(i as isize);
        }
        element = _cairo_array_index(
            &mut (*font).charstrings_index,
            glyph as libc::c_uint,
        ) as *mut cff_index_element_t;
        status = cff_index_append(
            &mut (*font).charstrings_subset_index,
            (*element).data,
            (*element).length,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        if (*font).subset_subroutines != 0 {
            status = cairo_cff_find_width_and_subroutines_used(
                font,
                (*element).data,
                (*element).length,
                glyph as libc::c_int,
                i as libc::c_int,
            ) as cairo_int_status_t;
            if status as libc::c_uint
                == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
            {
                (*font).subset_subroutines = 0 as libc::c_int;
                if (*font).is_opentype == 0 {
                    return status;
                }
            } else if status as u64 != 0 {
                return status
            }
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_subset_fontdict(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut i: libc::c_uint = 0;
    let mut fd: libc::c_int = 0;
    let mut reverse_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cid: libc::c_ulong = 0;
    let mut gid: libc::c_ulong = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let ref mut fresh54 = (*font).fdselect_subset;
    *fresh54 = calloc(
        (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).fdselect_subset).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh55 = (*font).fd_subset_map;
    *fresh55 = calloc(
        (*font).num_fontdicts as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).fd_subset_map).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh56 = (*font).private_dict_offset;
    *fresh56 = calloc(
        (*font).num_fontdicts as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).private_dict_offset).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    reverse_map = calloc(
        (*font).num_fontdicts as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if reverse_map.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font).num_fontdicts {
        *reverse_map.offset(i as isize) = -(1 as libc::c_int);
        i = i.wrapping_add(1);
    }
    (*font).num_subset_fontdicts = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        if (*font).is_opentype != 0 {
            gid = *((*(*font).scaled_font_subset).glyphs).offset(i as isize);
        } else {
            cid = *((*(*font).scaled_font_subset).glyphs).offset(i as isize);
            status = cairo_cff_font_get_gid_for_cid(font, cid, &mut gid);
            if status as u64 != 0 {
                free(reverse_map as *mut libc::c_void);
                return status as cairo_status_t;
            }
        }
        fd = *((*font).fdselect).offset(gid as isize);
        if *reverse_map.offset(fd as isize) < 0 as libc::c_int {
            *((*font).fd_subset_map).offset((*font).num_subset_fontdicts as isize) = fd;
            let ref mut fresh57 = (*font).num_subset_fontdicts;
            let fresh58 = *fresh57;
            *fresh57 = (*fresh57).wrapping_add(1);
            *reverse_map.offset(fd as isize) = fresh58 as libc::c_int;
        }
        *((*font).fdselect_subset).offset(i as isize) = *reverse_map.offset(fd as isize);
        i = i.wrapping_add(1);
    }
    free(reverse_map as *mut libc::c_void);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_create_cid_fontdict(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut buf: [libc::c_uchar; 100] = [0; 100];
    let mut end_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*font).num_fontdicts = 1 as libc::c_int as libc::c_uint;
    let ref mut fresh59 = (*font).fd_dict;
    *fresh59 = (if ::std::mem::size_of::<*mut cairo_hash_table_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<*mut cairo_hash_table_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut *mut cairo_hash_table_t;
    if ((*font).fd_dict).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    if cff_dict_init(&mut *((*font).fd_dict).offset(0 as libc::c_int as isize)) as u64
        != 0
    {
        free((*font).fd_dict as *mut libc::c_void);
        let ref mut fresh60 = (*font).fd_dict;
        *fresh60 = 0 as *mut *mut cairo_hash_table_t;
        (*font).num_fontdicts = 0 as libc::c_int as libc::c_uint;
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh61 = (*font).fd_subset_map;
    *fresh61 = (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_int;
    if ((*font).fd_subset_map).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh62 = (*font).private_dict_offset;
    *fresh62 = (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_int;
    if ((*font).private_dict_offset).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    *((*font).fd_subset_map).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    (*font).num_subset_fontdicts = 1 as libc::c_int as libc::c_uint;
    end_buf = encode_integer_max(buf.as_mut_ptr(), 0 as libc::c_int);
    end_buf = encode_integer_max(end_buf, 0 as libc::c_int);
    status = cff_dict_set_operands(
        *((*font).fd_dict).offset(0 as libc::c_int as isize),
        0x12 as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_subset_strings(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    status = cairo_cff_font_subset_dict_strings(font, (*font).top_dict);
    if status as u64 != 0 {
        return status;
    }
    if (*font).is_cid != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font).num_subset_fontdicts {
            status = cairo_cff_font_subset_dict_strings(
                font,
                *((*font).fd_dict)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
            );
            if status as u64 != 0 {
                return status;
            }
            status = cairo_cff_font_subset_dict_strings(
                font,
                *((*font).fd_private_dict)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
            );
            if status as u64 != 0 {
                return status;
            }
            i = i.wrapping_add(1);
        }
    } else {
        status = cairo_cff_font_subset_dict_strings(font, (*font).private_dict);
    }
    return status;
}
unsafe extern "C" fn cairo_cff_font_add_euro_charset_string(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut euro: *const libc::c_char = b"Euro\0" as *const u8 as *const libc::c_char;
    i = 1 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        ch = *((*(*font).scaled_font_subset).to_latin_char).offset(i as isize);
        if ch == 128 as libc::c_int {
            (*font)
                .euro_sid = (391 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    _cairo_array_num_elements(&mut (*font).strings_subset_index),
                ) as libc::c_int;
            status = cff_index_append_copy(
                &mut (*font).strings_subset_index,
                euro as *mut libc::c_uchar,
                strlen(euro) as libc::c_uint,
            );
            return status;
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_subset_font(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*(*font).scaled_font_subset).is_latin == 0 {
        status = cairo_cff_font_set_ros_strings(font);
        if status as u64 != 0 {
            return status;
        }
    }
    status = cairo_cff_font_subset_charstrings_and_subroutines(font) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    if (*(*font).scaled_font_subset).is_latin == 0 {
        if (*font).is_cid != 0 {
            status = cairo_cff_font_subset_fontdict(font);
        } else {
            status = cairo_cff_font_create_cid_fontdict(font);
        }
        if status as u64 != 0 {
            return status;
        }
    } else {
        let ref mut fresh63 = (*font).private_dict_offset;
        *fresh63 = (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_int;
        if ((*font).private_dict_offset).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    status = cairo_cff_font_subset_strings(font);
    if status as u64 != 0 {
        return status;
    }
    if (*(*font).scaled_font_subset).is_latin != 0 {
        status = cairo_cff_font_add_euro_charset_string(font);
    }
    return status;
}
unsafe extern "C" fn cairo_cff_font_set_topdict_operator_to_cur_pos(
    mut font: *mut cairo_cff_font_t,
    mut operator: libc::c_int,
) {
    let mut cur_pos: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 10] = [0; 10];
    let mut buf_end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut op_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    cur_pos = _cairo_array_num_elements(&mut (*font).output) as libc::c_int;
    buf_end = encode_integer_max(buf.as_mut_ptr(), cur_pos);
    offset = cff_dict_get_location(
        (*font).top_dict,
        operator as libc::c_ushort,
        &mut size,
    );
    if offset > 0 as libc::c_int {} else {
        __assert_fail(
            b"offset > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
            2032 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"void cairo_cff_font_set_topdict_operator_to_cur_pos(cairo_cff_font_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    op_ptr = _cairo_array_index(&mut (*font).output, offset as libc::c_uint)
        as *mut libc::c_uchar;
    memcpy(
        op_ptr as *mut libc::c_void,
        buf.as_mut_ptr() as *const libc::c_void,
        buf_end.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong,
    );
}
unsafe extern "C" fn cairo_cff_font_write_header(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    return _cairo_array_append_multiple(
        &mut (*font).output,
        (*font).header as *const libc::c_void,
        (*(*font).header).header_size as libc::c_uint,
    );
}
unsafe extern "C" fn cairo_cff_font_write_name(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut index: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    cff_index_init(&mut index);
    status = cff_index_append_copy(
        &mut index,
        (*font).ps_name as *mut libc::c_uchar,
        strlen((*font).ps_name) as libc::c_uint,
    );
    if !(status as u64 != 0) {
        status = cff_index_write(&mut index, &mut (*font).output);
        status as u64 != 0;
    }
    cff_index_fini(&mut index);
    return status;
}
unsafe extern "C" fn cairo_cff_font_write_top_dict(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut count: uint16_t = 0;
    let mut buf: [libc::c_uchar; 10] = [0; 10];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset_index: libc::c_int = 0;
    let mut dict_start: libc::c_int = 0;
    let mut dict_size: libc::c_int = 0;
    let mut offset_size: libc::c_int = 4 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    count = cpu_to_be16(1 as libc::c_int as uint16_t);
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        &mut count as *mut uint16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    buf[0 as libc::c_int as usize] = offset_size as libc::c_uchar;
    status = _cairo_array_append(
        &mut (*font).output,
        buf.as_mut_ptr() as *const libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    encode_index_offset(
        buf.as_mut_ptr(),
        offset_size,
        1 as libc::c_int as libc::c_ulong,
    );
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        buf.as_mut_ptr() as *const libc::c_void,
        offset_size as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    offset_index = _cairo_array_num_elements(&mut (*font).output) as libc::c_int;
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        buf.as_mut_ptr() as *const libc::c_void,
        offset_size as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    dict_start = _cairo_array_num_elements(&mut (*font).output) as libc::c_int;
    status = cff_dict_write((*font).top_dict, &mut (*font).output);
    if status as u64 != 0 {
        return status;
    }
    dict_size = (_cairo_array_num_elements(&mut (*font).output))
        .wrapping_sub(dict_start as libc::c_uint) as libc::c_int;
    encode_index_offset(
        buf.as_mut_ptr(),
        offset_size,
        (dict_size + 1 as libc::c_int) as libc::c_ulong,
    );
    p = _cairo_array_index(&mut (*font).output, offset_index as libc::c_uint)
        as *mut libc::c_uchar;
    memcpy(
        p as *mut libc::c_void,
        buf.as_mut_ptr() as *const libc::c_void,
        offset_size as libc::c_ulong,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_strings(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    return cff_index_write(&mut (*font).strings_subset_index, &mut (*font).output);
}
unsafe extern "C" fn cairo_cff_font_write_global_subrs(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut i: libc::c_uint = 0;
    let mut return_op: libc::c_uchar = 0xb as libc::c_int as libc::c_uchar;
    if (*font).subset_subroutines != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i < _cairo_array_num_elements(&mut (*font).global_sub_index) {
            if *((*font).global_subs_used).offset(i as isize) == 0 {
                cff_index_set_object(
                    &mut (*font).global_sub_index,
                    i as libc::c_int,
                    &mut return_op,
                    1 as libc::c_int,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    return cff_index_write(&mut (*font).global_sub_index, &mut (*font).output);
}
unsafe extern "C" fn cairo_cff_font_write_encoding(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut buf: [libc::c_uchar; 2] = [0; 2];
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    cairo_cff_font_set_topdict_operator_to_cur_pos(font, 0x10 as libc::c_int);
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    buf[1 as libc::c_int
        as usize] = ((*(*font).scaled_font_subset).num_glyphs)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_uchar;
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        buf.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_uint,
    );
    if status as u64 != 0 {
        return status;
    }
    i = 1 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        let mut ch: libc::c_uchar = *((*(*font).scaled_font_subset).to_latin_char)
            .offset(i as isize) as libc::c_uchar;
        status = _cairo_array_append(
            &mut (*font).output,
            &mut ch as *mut libc::c_uchar as *const libc::c_void,
        );
        if status as u64 != 0 {
            return status;
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_fdselect(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut data: libc::c_uchar = 0;
    let mut i: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    cairo_cff_font_set_topdict_operator_to_cur_pos(font, 0xc25 as libc::c_int);
    if (*font).is_cid != 0 {
        data = 0 as libc::c_int as libc::c_uchar;
        status = _cairo_array_append(
            &mut (*font).output,
            &mut data as *mut libc::c_uchar as *const libc::c_void,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*(*font).scaled_font_subset).num_glyphs {
            data = *((*font).fdselect_subset).offset(i as isize) as libc::c_uchar;
            status = _cairo_array_append(
                &mut (*font).output,
                &mut data as *mut libc::c_uchar as *const libc::c_void,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                return status as cairo_status_t;
            }
            i = i.wrapping_add(1);
        }
    } else {
        let mut byte: libc::c_uchar = 0;
        let mut word: uint16_t = 0;
        status = _cairo_array_grow_by(
            &mut (*font).output,
            9 as libc::c_int as libc::c_uint,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
        byte = 3 as libc::c_int as libc::c_uchar;
        status = _cairo_array_append(
            &mut (*font).output,
            &mut byte as *mut libc::c_uchar as *const libc::c_void,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2194 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"cairo_status_t cairo_cff_font_write_fdselect(cairo_cff_font_t *)\0"))
                    .as_ptr(),
            );
        }
        word = cpu_to_be16(1 as libc::c_int as uint16_t);
        status = _cairo_array_append_multiple(
            &mut (*font).output,
            &mut word as *mut uint16_t as *const libc::c_void,
            2 as libc::c_int as libc::c_uint,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2198 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"cairo_status_t cairo_cff_font_write_fdselect(cairo_cff_font_t *)\0"))
                    .as_ptr(),
            );
        }
        word = cpu_to_be16(0 as libc::c_int as uint16_t);
        status = _cairo_array_append_multiple(
            &mut (*font).output,
            &mut word as *mut uint16_t as *const libc::c_void,
            2 as libc::c_int as libc::c_uint,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2202 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"cairo_status_t cairo_cff_font_write_fdselect(cairo_cff_font_t *)\0"))
                    .as_ptr(),
            );
        }
        byte = 0 as libc::c_int as libc::c_uchar;
        status = _cairo_array_append(
            &mut (*font).output,
            &mut byte as *mut libc::c_uchar as *const libc::c_void,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2206 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"cairo_status_t cairo_cff_font_write_fdselect(cairo_cff_font_t *)\0"))
                    .as_ptr(),
            );
        }
        word = cpu_to_be16((*(*font).scaled_font_subset).num_glyphs as uint16_t);
        status = _cairo_array_append_multiple(
            &mut (*font).output,
            &mut word as *mut uint16_t as *const libc::c_void,
            2 as libc::c_int as libc::c_uint,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2210 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"cairo_status_t cairo_cff_font_write_fdselect(cairo_cff_font_t *)\0"))
                    .as_ptr(),
            );
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
static mut winansi_to_cff_std_string: [libc::c_int; 128] = [
    0 as libc::c_int,
    0 as libc::c_int,
    117 as libc::c_int,
    101 as libc::c_int,
    118 as libc::c_int,
    121 as libc::c_int,
    112 as libc::c_int,
    113 as libc::c_int,
    126 as libc::c_int,
    122 as libc::c_int,
    192 as libc::c_int,
    107 as libc::c_int,
    142 as libc::c_int,
    0 as libc::c_int,
    199 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    65 as libc::c_int,
    8 as libc::c_int,
    105 as libc::c_int,
    119 as libc::c_int,
    116 as libc::c_int,
    111 as libc::c_int,
    137 as libc::c_int,
    127 as libc::c_int,
    153 as libc::c_int,
    221 as libc::c_int,
    108 as libc::c_int,
    148 as libc::c_int,
    0 as libc::c_int,
    228 as libc::c_int,
    198 as libc::c_int,
    0 as libc::c_int,
    96 as libc::c_int,
    97 as libc::c_int,
    98 as libc::c_int,
    103 as libc::c_int,
    100 as libc::c_int,
    160 as libc::c_int,
    102 as libc::c_int,
    131 as libc::c_int,
    170 as libc::c_int,
    139 as libc::c_int,
    106 as libc::c_int,
    151 as libc::c_int,
    0 as libc::c_int,
    165 as libc::c_int,
    128 as libc::c_int,
    161 as libc::c_int,
    156 as libc::c_int,
    164 as libc::c_int,
    169 as libc::c_int,
    125 as libc::c_int,
    152 as libc::c_int,
    115 as libc::c_int,
    114 as libc::c_int,
    133 as libc::c_int,
    150 as libc::c_int,
    143 as libc::c_int,
    120 as libc::c_int,
    158 as libc::c_int,
    155 as libc::c_int,
    163 as libc::c_int,
    123 as libc::c_int,
    174 as libc::c_int,
    171 as libc::c_int,
    172 as libc::c_int,
    176 as libc::c_int,
    173 as libc::c_int,
    175 as libc::c_int,
    138 as libc::c_int,
    177 as libc::c_int,
    181 as libc::c_int,
    178 as libc::c_int,
    179 as libc::c_int,
    180 as libc::c_int,
    185 as libc::c_int,
    182 as libc::c_int,
    183 as libc::c_int,
    184 as libc::c_int,
    154 as libc::c_int,
    186 as libc::c_int,
    190 as libc::c_int,
    187 as libc::c_int,
    188 as libc::c_int,
    191 as libc::c_int,
    189 as libc::c_int,
    168 as libc::c_int,
    141 as libc::c_int,
    196 as libc::c_int,
    193 as libc::c_int,
    194 as libc::c_int,
    195 as libc::c_int,
    197 as libc::c_int,
    157 as libc::c_int,
    149 as libc::c_int,
    203 as libc::c_int,
    200 as libc::c_int,
    201 as libc::c_int,
    205 as libc::c_int,
    202 as libc::c_int,
    204 as libc::c_int,
    144 as libc::c_int,
    206 as libc::c_int,
    210 as libc::c_int,
    207 as libc::c_int,
    208 as libc::c_int,
    209 as libc::c_int,
    214 as libc::c_int,
    211 as libc::c_int,
    212 as libc::c_int,
    213 as libc::c_int,
    167 as libc::c_int,
    215 as libc::c_int,
    219 as libc::c_int,
    216 as libc::c_int,
    217 as libc::c_int,
    220 as libc::c_int,
    218 as libc::c_int,
    159 as libc::c_int,
    147 as libc::c_int,
    225 as libc::c_int,
    222 as libc::c_int,
    223 as libc::c_int,
    224 as libc::c_int,
    226 as libc::c_int,
    162 as libc::c_int,
    227 as libc::c_int,
];
unsafe extern "C" fn cairo_cff_font_get_sid_for_winansi_char(
    mut font: *mut cairo_cff_font_t,
    mut ch: libc::c_int,
) -> libc::c_int {
    let mut sid: libc::c_int = 0;
    if ch == 39 as libc::c_int {
        sid = 104 as libc::c_int;
    } else if ch == 96 as libc::c_int {
        sid = 124 as libc::c_int;
    } else if ch >= 32 as libc::c_int && ch <= 126 as libc::c_int {
        sid = ch - 31 as libc::c_int;
    } else if ch == 128 as libc::c_int {
        if (*font).euro_sid >= 391 as libc::c_int {} else {
            __assert_fail(
                b"font->euro_sid >= NUM_STD_STRINGS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2259 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"int cairo_cff_font_get_sid_for_winansi_char(cairo_cff_font_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        sid = (*font).euro_sid;
    } else if ch >= 128 as libc::c_int && ch <= 255 as libc::c_int {
        sid = winansi_to_cff_std_string[(ch - 128 as libc::c_int) as usize];
    } else {
        sid = 0 as libc::c_int;
    }
    return sid;
}
unsafe extern "C" fn cairo_cff_font_write_type1_charset(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut format: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut sid: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut sid_be16: uint16_t = 0;
    cairo_cff_font_set_topdict_operator_to_cur_pos(font, 0xf as libc::c_int);
    status = _cairo_array_append(
        &mut (*font).output,
        &mut format as *mut libc::c_uchar as *const libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    i = 1 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        ch = *((*(*font).scaled_font_subset).to_latin_char).offset(i as isize);
        sid = cairo_cff_font_get_sid_for_winansi_char(font, ch);
        if status as u64 != 0 {
            return status;
        }
        sid_be16 = cpu_to_be16(sid as uint16_t);
        status = _cairo_array_append_multiple(
            &mut (*font).output,
            &mut sid_be16 as *mut uint16_t as *const libc::c_void,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong as libc::c_uint,
        );
        if status as u64 != 0 {
            return status;
        }
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_cid_charset(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut byte: libc::c_uchar = 0;
    let mut word: uint16_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    cairo_cff_font_set_topdict_operator_to_cur_pos(font, 0xf as libc::c_int);
    status = _cairo_array_grow_by(&mut (*font).output, 5 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        return status;
    }
    byte = 2 as libc::c_int as libc::c_uchar;
    status = _cairo_array_append(
        &mut (*font).output,
        &mut byte as *mut libc::c_uchar as *const libc::c_void,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
            2315 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"cairo_status_t cairo_cff_font_write_cid_charset(cairo_cff_font_t *)\0"))
                .as_ptr(),
        );
    }
    word = cpu_to_be16(1 as libc::c_int as uint16_t);
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        &mut word as *mut uint16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_uint,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
            2319 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"cairo_status_t cairo_cff_font_write_cid_charset(cairo_cff_font_t *)\0"))
                .as_ptr(),
        );
    }
    word = cpu_to_be16(
        ((*(*font).scaled_font_subset).num_glyphs)
            .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint16_t,
    );
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        &mut word as *mut uint16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_uint,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
            2323 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"cairo_status_t cairo_cff_font_write_cid_charset(cairo_cff_font_t *)\0"))
                .as_ptr(),
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_charstrings(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    cairo_cff_font_set_topdict_operator_to_cur_pos(font, 0x11 as libc::c_int);
    return cff_index_write(&mut (*font).charstrings_subset_index, &mut (*font).output);
}
unsafe extern "C" fn cairo_cff_font_write_cid_fontdict(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut i: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut offset_array: libc::c_uint = 0;
    let mut offset_array_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset_base: libc::c_int = 0;
    let mut count: uint16_t = 0;
    let mut offset_size: uint8_t = 4 as libc::c_int as uint8_t;
    cairo_cff_font_set_topdict_operator_to_cur_pos(font, 0xc24 as libc::c_int);
    count = cpu_to_be16((*font).num_subset_fontdicts as uint16_t);
    status = _cairo_array_append_multiple(
        &mut (*font).output,
        &mut count as *mut uint16_t as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong as libc::c_uint,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = _cairo_array_append(
        &mut (*font).output,
        &mut offset_size as *mut uint8_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    offset_array = _cairo_array_num_elements(&mut (*font).output);
    status = _cairo_array_allocate(
        &mut (*font).output,
        ((*font).num_subset_fontdicts)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(offset_size as libc::c_uint),
        &mut offset_array_ptr as *mut *mut libc::c_uchar as *mut *mut libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    offset_base = (_cairo_array_num_elements(&mut (*font).output))
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    put_unaligned_be32(1 as libc::c_int as uint32_t, offset_array_ptr);
    offset_array = (offset_array as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong) as libc::c_uint
        as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font).num_subset_fontdicts {
        status = cff_dict_write(
            *((*font).fd_dict)
                .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
            &mut (*font).output,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
        offset_array_ptr = _cairo_array_index(&mut (*font).output, offset_array)
            as *mut libc::c_uchar;
        put_unaligned_be32(
            (_cairo_array_num_elements(&mut (*font).output))
                .wrapping_sub(offset_base as libc::c_uint),
            offset_array_ptr,
        );
        offset_array = (offset_array as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint;
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_private_dict(
    mut font: *mut cairo_cff_font_t,
    mut dict_num: libc::c_int,
    mut parent_dict: *mut cairo_hash_table_t,
    mut private_dict: *mut cairo_hash_table_t,
) -> cairo_status_t {
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 10] = [0; 10];
    let mut buf_end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    *((*font).private_dict_offset)
        .offset(
            dict_num as isize,
        ) = _cairo_array_num_elements(&mut (*font).output) as libc::c_int;
    status = cff_dict_write(private_dict, &mut (*font).output);
    if status as u64 != 0 {
        return status;
    }
    size = (_cairo_array_num_elements(&mut (*font).output))
        .wrapping_sub(
            *((*font).private_dict_offset).offset(dict_num as isize) as libc::c_uint,
        ) as libc::c_int;
    buf_end = encode_integer_max(buf.as_mut_ptr(), size);
    buf_end = encode_integer_max(
        buf_end,
        *((*font).private_dict_offset).offset(dict_num as isize),
    );
    offset = cff_dict_get_location(
        parent_dict,
        0x12 as libc::c_int as libc::c_ushort,
        &mut size,
    );
    if offset > 0 as libc::c_int {} else {
        __assert_fail(
            b"offset > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
            2404 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"cairo_status_t cairo_cff_font_write_private_dict(cairo_cff_font_t *, int, cairo_hash_table_t *, cairo_hash_table_t *)\0",
            ))
                .as_ptr(),
        );
    }
    p = _cairo_array_index(&mut (*font).output, offset as libc::c_uint)
        as *mut libc::c_uchar;
    memcpy(
        p as *mut libc::c_void,
        buf.as_mut_ptr() as *const libc::c_void,
        buf_end.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_local_sub(
    mut font: *mut cairo_cff_font_t,
    mut dict_num: libc::c_int,
    mut private_dict: *mut cairo_hash_table_t,
    mut local_sub_index: *mut cairo_array_t,
    mut local_subs_used: *mut cairo_bool_t,
) -> cairo_status_t {
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 10] = [0; 10];
    let mut buf_end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut return_op: libc::c_uchar = 0xb as libc::c_int as libc::c_uchar;
    if _cairo_array_num_elements(local_sub_index) > 0 as libc::c_int as libc::c_uint {
        offset = (_cairo_array_num_elements(&mut (*font).output))
            .wrapping_sub(
                *((*font).private_dict_offset).offset(dict_num as isize) as libc::c_uint,
            ) as libc::c_int;
        buf_end = encode_integer_max(buf.as_mut_ptr(), offset);
        offset = cff_dict_get_location(
            private_dict,
            0x13 as libc::c_int as libc::c_ushort,
            &mut size,
        );
        if offset > 0 as libc::c_int {} else {
            __assert_fail(
                b"offset > 0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-cff-subset.c\0" as *const u8 as *const libc::c_char,
                2434 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 126],
                    &[libc::c_char; 126],
                >(
                    b"cairo_status_t cairo_cff_font_write_local_sub(cairo_cff_font_t *, int, cairo_hash_table_t *, cairo_array_t *, cairo_bool_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        p = _cairo_array_index(&mut (*font).output, offset as libc::c_uint)
            as *mut libc::c_uchar;
        memcpy(
            p as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            buf_end.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong,
        );
        if (*font).subset_subroutines != 0 {
            i = 0 as libc::c_int as libc::c_uint;
            while i < _cairo_array_num_elements(local_sub_index) {
                if *local_subs_used.offset(i as isize) == 0 {
                    cff_index_set_object(
                        local_sub_index,
                        i as libc::c_int,
                        &mut return_op,
                        1 as libc::c_int,
                    );
                }
                i = i.wrapping_add(1);
            }
        }
        status = cff_index_write(local_sub_index, &mut (*font).output);
        if status as u64 != 0 {
            return status;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_cid_private_dict_and_local_sub(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut i: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*font).is_cid != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font).num_subset_fontdicts {
            status = cairo_cff_font_write_private_dict(
                font,
                i as libc::c_int,
                *((*font).fd_dict)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
                *((*font).fd_private_dict)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                return status as cairo_status_t;
            }
            i = i.wrapping_add(1);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font).num_subset_fontdicts {
            status = cairo_cff_font_write_local_sub(
                font,
                i as libc::c_int,
                *((*font).fd_private_dict)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
                &mut *((*font).fd_local_sub_index)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
                *((*font).fd_local_subs_used)
                    .offset(*((*font).fd_subset_map).offset(i as isize) as isize),
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                return status as cairo_status_t;
            }
            i = i.wrapping_add(1);
        }
    } else {
        status = cairo_cff_font_write_private_dict(
            font,
            0 as libc::c_int,
            *((*font).fd_dict).offset(0 as libc::c_int as isize),
            (*font).private_dict,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
        status = cairo_cff_font_write_local_sub(
            font,
            0 as libc::c_int,
            (*font).private_dict,
            &mut (*font).local_sub_index,
            (*font).local_subs_used,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_write_type1_private_dict_and_local_sub(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = cairo_cff_font_write_private_dict(
        font,
        0 as libc::c_int,
        (*font).top_dict,
        (*font).private_dict,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = cairo_cff_font_write_local_sub(
        font,
        0 as libc::c_int,
        (*font).private_dict,
        &mut (*font).local_sub_index,
        (*font).local_subs_used,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    return CAIRO_STATUS_SUCCESS;
}
static mut font_write_cid_funcs: [font_write_t; 10] = unsafe {
    [
        Some(
            cairo_cff_font_write_header
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_name
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_top_dict
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_strings
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_global_subrs
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_cid_charset
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_fdselect
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_charstrings
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_cid_fontdict
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_cid_private_dict_and_local_sub
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
    ]
};
static mut font_write_type1_funcs: [font_write_t; 9] = unsafe {
    [
        Some(
            cairo_cff_font_write_header
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_name
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_top_dict
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_strings
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_global_subrs
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_encoding
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_type1_charset
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_charstrings
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
        Some(
            cairo_cff_font_write_type1_private_dict_and_local_sub
                as unsafe extern "C" fn(*mut cairo_cff_font_t) -> cairo_status_t,
        ),
    ]
};
unsafe extern "C" fn cairo_cff_font_write_subset(
    mut font: *mut cairo_cff_font_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    if (*(*font).scaled_font_subset).is_latin != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i
            < (::std::mem::size_of::<[font_write_t; 9]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<font_write_t>() as libc::c_ulong)
                as libc::c_int as libc::c_uint
        {
            status = (font_write_type1_funcs[i as usize])
                .expect("non-null function pointer")(font) as cairo_int_status_t;
            if status as u64 != 0 {
                return status as cairo_status_t;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i
            < (::std::mem::size_of::<[font_write_t; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<font_write_t>() as libc::c_ulong)
                as libc::c_int as libc::c_uint
        {
            status = (font_write_cid_funcs[i as usize])
                .expect("non-null function pointer")(font) as cairo_int_status_t;
            if status as u64 != 0 {
                return status as cairo_status_t;
            }
            i = i.wrapping_add(1);
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_cff_font_generate(
    mut font: *mut cairo_cff_font_t,
    mut data: *mut *const libc::c_char,
    mut length: *mut libc::c_ulong,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = cairo_cff_font_read_font(font);
    if status as u64 != 0 {
        return status;
    }
    if ((*font).ps_name).is_null() {
        let ref mut fresh64 = (*font).ps_name;
        *fresh64 = (if 30 as libc::c_int != 0 as libc::c_int {
            malloc(30 as libc::c_int as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if ((*font).ps_name).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        snprintf(
            (*font).ps_name,
            30 as libc::c_int as libc::c_ulong,
            b"CairoFont-%u-%u\0" as *const u8 as *const libc::c_char,
            (*(*font).scaled_font_subset).font_id,
            (*(*font).scaled_font_subset).subset_id,
        );
    }
    status = cairo_cff_font_subset_font(font) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = cairo_cff_font_write_subset(font) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    *data = _cairo_array_index(&mut (*font).output, 0 as libc::c_int as libc::c_uint)
        as *const libc::c_char;
    *length = _cairo_array_num_elements(&mut (*font).output) as libc::c_ulong;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_create_set_widths(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut size: libc::c_ulong = 0;
    let mut long_entry_size: libc::c_ulong = 0;
    let mut short_entry_size: libc::c_ulong = 0;
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
    let mut short_entry: uint16_t = 0;
    let mut glyph_index: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
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
    );
    if status as u64 != 0 {
        return status;
    }
    num_hmetrics = be16_to_cpu(hhea.num_hmetrics) as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        glyph_index = *((*(*font).scaled_font_subset).glyphs).offset(i as isize)
            as libc::c_int;
        long_entry_size = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int16_t>() as libc::c_ulong);
        short_entry_size = ::std::mem::size_of::<int16_t>() as libc::c_ulong;
        if glyph_index < num_hmetrics {
            status = ((*(*font).backend).load_truetype_table)
                .expect(
                    "non-null function pointer",
                )(
                (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
                (('h' as i32 as uint32_t) << 24 as libc::c_int
                    | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
                    | 'x' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
                (glyph_index as libc::c_ulong).wrapping_mul(long_entry_size)
                    as libc::c_long,
                &mut short_entry as *mut uint16_t as *mut libc::c_uchar,
                &mut short_entry_size,
            );
            if status as u64 != 0 {
                return status;
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
                &mut short_entry as *mut uint16_t as *mut libc::c_uchar,
                &mut short_entry_size,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        *((*font).widths).offset(i as isize) = be16_to_cpu(short_entry) as libc::c_int;
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn check_fontdata_is_cff(
    mut data: *const libc::c_uchar,
    mut length: libc::c_long,
) -> cairo_bool_t {
    let mut header: *mut cff_header_t = 0 as *mut cff_header_t;
    if length < ::std::mem::size_of::<cff_header_t>() as libc::c_ulong as libc::c_long {
        return 0 as libc::c_int;
    }
    header = data as *mut cff_header_t;
    if (*header).major as libc::c_int == 1 as libc::c_int
        && (*header).minor as libc::c_int == 0 as libc::c_int
        && (*header).header_size as libc::c_int == 4 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_cff_font_load_opentype_cff(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut backend: *const cairo_scaled_font_backend_t = (*font).backend;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
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
    let mut size: libc::c_ulong = 0;
    let mut data_length: libc::c_ulong = 0;
    if ((*backend).load_truetype_table).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    data_length = 0 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('C' as i32 as uint32_t) << 24 as libc::c_int
            | (('F' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('F' as i32) << 8 as libc::c_int) as libc::c_uint
            | ' ' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    size = ::std::mem::size_of::<tt_head_t>() as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('e' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('a' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'd' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        &mut head as *mut tt_head_t as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    size = ::std::mem::size_of::<tt_hhea_t>() as libc::c_ulong;
    status = ((*backend).load_truetype_table)
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
        return status as cairo_int_status_t;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('h' as i32 as uint32_t) << 24 as libc::c_int
            | (('m' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('t' as i32) << 8 as libc::c_int) as libc::c_uint
            | 'x' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    (*font).x_min = be16_to_cpu(head.x_min as uint16_t) as int16_t as libc::c_int;
    (*font).y_min = be16_to_cpu(head.y_min as uint16_t) as int16_t as libc::c_int;
    (*font).x_max = be16_to_cpu(head.x_max as uint16_t) as int16_t as libc::c_int;
    (*font).y_max = be16_to_cpu(head.y_max as uint16_t) as int16_t as libc::c_int;
    (*font).ascent = be16_to_cpu(hhea.ascender as uint16_t) as int16_t as libc::c_int;
    (*font).descent = be16_to_cpu(hhea.descender as uint16_t) as int16_t as libc::c_int;
    (*font).units_per_em = be16_to_cpu(head.units_per_em) as int16_t as libc::c_int;
    if (*font).units_per_em == 0 as libc::c_int {
        (*font).units_per_em = 1000 as libc::c_int;
    }
    let ref mut fresh65 = (*font).font_name;
    *fresh65 = 0 as *mut libc::c_char;
    status = _cairo_truetype_read_font_name(
        (*(*font).scaled_font_subset).scaled_font,
        &mut (*font).ps_name,
        &mut (*font).font_name,
    ) as cairo_status_t;
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status as cairo_int_status_t;
    }
    (*font).is_opentype = 1 as libc::c_int;
    (*font).data_length = data_length;
    let ref mut fresh66 = (*font).data;
    *fresh66 = (if data_length != 0 as libc::c_int as libc::c_ulong {
        malloc(data_length)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if ((*font).data).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = ((*(*font).backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        (('C' as i32 as uint32_t) << 24 as libc::c_int
            | (('F' as i32) << 16 as libc::c_int) as libc::c_uint
            | (('F' as i32) << 8 as libc::c_int) as libc::c_uint
            | ' ' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        (*font).data,
        &mut (*font).data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if check_fontdata_is_cff((*font).data, data_length as libc::c_long) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_cff_font_load_cff(
    mut font: *mut cairo_cff_font_t,
) -> cairo_int_status_t {
    let mut backend: *const cairo_scaled_font_backend_t = (*font).backend;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut data_length: libc::c_ulong = 0;
    if ((*backend).load_type1_data).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    data_length = 0 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_type1_data)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    let ref mut fresh67 = (*font).font_name;
    *fresh67 = 0 as *mut libc::c_char;
    (*font).is_opentype = 0 as libc::c_int;
    (*font).data_length = data_length;
    let ref mut fresh68 = (*font).data;
    *fresh68 = (if data_length != 0 as libc::c_int as libc::c_ulong {
        malloc(data_length)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if ((*font).data).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = ((*(*font).backend).load_type1_data)
        .expect(
            "non-null function pointer",
        )(
        (*(*font).scaled_font_subset).scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_long,
        (*font).data,
        &mut (*font).data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if check_fontdata_is_cff((*font).data, data_length as libc::c_long) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_cff_font_create(
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut font_return: *mut *mut cairo_cff_font_t,
    mut subset_name: *const libc::c_char,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut is_synthetic: cairo_bool_t = 0;
    let mut font: *mut cairo_cff_font_t = 0 as *mut cairo_cff_font_t;
    backend = (*(*scaled_font_subset).scaled_font).backend;
    if ((*backend).is_synthetic).is_some() {
        status = ((*backend).is_synthetic)
            .expect(
                "non-null function pointer",
            )((*scaled_font_subset).scaled_font as *mut libc::c_void, &mut is_synthetic);
        if status as u64 != 0 {
            return status;
        }
        if is_synthetic != 0 {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
    }
    font = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_cff_font_t>() as libc::c_ulong,
    ) as *mut cairo_cff_font_t;
    if font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh69 = (*font).backend;
    *fresh69 = backend;
    let ref mut fresh70 = (*font).scaled_font_subset;
    *fresh70 = scaled_font_subset;
    status = _cairo_cff_font_load_opentype_cff(font);
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        status = _cairo_cff_font_load_cff(font);
    }
    if !(status as u64 != 0) {
        let ref mut fresh71 = (*font).data_end;
        *fresh71 = ((*font).data).offset((*font).data_length as isize);
        _cairo_array_init(
            &mut (*font).output,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
        );
        status = _cairo_array_grow_by(
            &mut (*font).output,
            4096 as libc::c_int as libc::c_uint,
        ) as cairo_int_status_t;
        if !(status as u64 != 0) {
            let ref mut fresh72 = (*font).subset_font_name;
            *fresh72 = strdup(subset_name);
            if ((*font).subset_font_name).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            } else {
                let ref mut fresh73 = (*font).widths;
                *fresh73 = calloc(
                    (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                if ((*font).widths).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                } else {
                    if (*font).is_opentype != 0 {
                        status = cairo_cff_font_create_set_widths(font);
                        if status as u64 != 0 {
                            current_block = 17498650053426571896;
                        } else {
                            current_block = 11459959175219260272;
                        }
                    } else {
                        current_block = 11459959175219260272;
                    }
                    match current_block {
                        11459959175219260272 => {
                            status = cff_dict_init(&mut (*font).top_dict)
                                as cairo_int_status_t;
                            if !(status as u64 != 0) {
                                status = cff_dict_init(&mut (*font).private_dict)
                                    as cairo_int_status_t;
                                if status as u64 != 0 {
                                    _cairo_hash_table_destroy((*font).top_dict);
                                } else {
                                    cff_index_init(&mut (*font).strings_index);
                                    cff_index_init(&mut (*font).charstrings_index);
                                    cff_index_init(&mut (*font).global_sub_index);
                                    cff_index_init(&mut (*font).local_sub_index);
                                    cff_index_init(&mut (*font).charstrings_subset_index);
                                    cff_index_init(&mut (*font).strings_subset_index);
                                    (*font).euro_sid = 0 as libc::c_int;
                                    let ref mut fresh74 = (*font).fdselect;
                                    *fresh74 = 0 as *mut libc::c_int;
                                    let ref mut fresh75 = (*font).fd_dict;
                                    *fresh75 = 0 as *mut *mut cairo_hash_table_t;
                                    let ref mut fresh76 = (*font).fd_private_dict;
                                    *fresh76 = 0 as *mut *mut cairo_hash_table_t;
                                    let ref mut fresh77 = (*font).fd_local_sub_index;
                                    *fresh77 = 0 as *mut cairo_array_t;
                                    let ref mut fresh78 = (*font).fd_local_sub_bias;
                                    *fresh78 = 0 as *mut libc::c_int;
                                    let ref mut fresh79 = (*font).fdselect_subset;
                                    *fresh79 = 0 as *mut libc::c_int;
                                    let ref mut fresh80 = (*font).fd_subset_map;
                                    *fresh80 = 0 as *mut libc::c_int;
                                    let ref mut fresh81 = (*font).private_dict_offset;
                                    *fresh81 = 0 as *mut libc::c_int;
                                    let ref mut fresh82 = (*font).global_subs_used;
                                    *fresh82 = 0 as *mut cairo_bool_t;
                                    let ref mut fresh83 = (*font).local_subs_used;
                                    *fresh83 = 0 as *mut cairo_bool_t;
                                    let ref mut fresh84 = (*font).fd_local_subs_used;
                                    *fresh84 = 0 as *mut *mut cairo_bool_t;
                                    *font_return = font;
                                    return CAIRO_STATUS_SUCCESS as libc::c_int
                                        as cairo_int_status_t;
                                }
                            }
                        }
                        _ => {}
                    }
                    free((*font).widths as *mut libc::c_void);
                }
                free((*font).subset_font_name as *mut libc::c_void);
            }
        }
        free((*font).ps_name as *mut libc::c_void);
        _cairo_array_fini(&mut (*font).output);
    }
    free((*font).data as *mut libc::c_void);
    free((*font).font_name as *mut libc::c_void);
    free(font as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn cairo_cff_font_destroy(mut font: *mut cairo_cff_font_t) {
    let mut i: libc::c_uint = 0;
    free((*font).widths as *mut libc::c_void);
    free((*font).font_name as *mut libc::c_void);
    free((*font).ps_name as *mut libc::c_void);
    free((*font).subset_font_name as *mut libc::c_void);
    _cairo_array_fini(&mut (*font).output);
    cff_dict_fini((*font).top_dict);
    cff_dict_fini((*font).private_dict);
    cff_index_fini(&mut (*font).strings_index);
    cff_index_fini(&mut (*font).charstrings_index);
    cff_index_fini(&mut (*font).global_sub_index);
    cff_index_fini(&mut (*font).local_sub_index);
    cff_index_fini(&mut (*font).charstrings_subset_index);
    cff_index_fini(&mut (*font).strings_subset_index);
    if !((*font).fd_dict).is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font).num_fontdicts {
            if !(*((*font).fd_dict).offset(i as isize)).is_null() {
                cff_dict_fini(*((*font).fd_dict).offset(i as isize));
            }
            i = i.wrapping_add(1);
        }
        free((*font).fd_dict as *mut libc::c_void);
    }
    free((*font).global_subs_used as *mut libc::c_void);
    free((*font).local_subs_used as *mut libc::c_void);
    free((*font).fd_subset_map as *mut libc::c_void);
    free((*font).private_dict_offset as *mut libc::c_void);
    if (*font).is_cid != 0 {
        free((*font).fdselect as *mut libc::c_void);
        free((*font).fdselect_subset as *mut libc::c_void);
        if !((*font).fd_private_dict).is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*font).num_fontdicts {
                if !(*((*font).fd_private_dict).offset(i as isize)).is_null() {
                    cff_dict_fini(*((*font).fd_private_dict).offset(i as isize));
                }
                i = i.wrapping_add(1);
            }
            free((*font).fd_private_dict as *mut libc::c_void);
        }
        if !((*font).fd_local_sub_index).is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*font).num_fontdicts {
                cff_index_fini(&mut *((*font).fd_local_sub_index).offset(i as isize));
                i = i.wrapping_add(1);
            }
            free((*font).fd_local_sub_index as *mut libc::c_void);
        }
        free((*font).fd_local_sub_bias as *mut libc::c_void);
        if !((*font).fd_local_subs_used).is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*font).num_fontdicts {
                free(
                    *((*font).fd_local_subs_used).offset(i as isize) as *mut libc::c_void,
                );
                i = i.wrapping_add(1);
            }
            free((*font).fd_local_subs_used as *mut libc::c_void);
        }
        free((*font).fd_default_width as *mut libc::c_void);
        free((*font).fd_nominal_width as *mut libc::c_void);
    }
    free((*font).data as *mut libc::c_void);
    free(font as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cff_subset_init(
    mut cff_subset: *mut cairo_cff_subset_t,
    mut subset_name: *const libc::c_char,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut font: *mut cairo_cff_font_t = 0 as *mut cairo_cff_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_uint = 0;
    status = _cairo_cff_font_create(font_subset, &mut font, subset_name)
        as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = cairo_cff_font_generate(font, &mut data, &mut length) as cairo_status_t;
    if !(status as u64 != 0) {
        let ref mut fresh85 = (*cff_subset).ps_name;
        *fresh85 = strdup((*font).ps_name);
        if ((*cff_subset).ps_name).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            if !((*font).font_name).is_null() {
                let ref mut fresh86 = (*cff_subset).family_name_utf8;
                *fresh86 = strdup((*font).font_name);
                if ((*cff_subset).family_name_utf8).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    current_block = 1265706858151309666;
                } else {
                    current_block = 15652330335145281839;
                }
            } else {
                let ref mut fresh87 = (*cff_subset).family_name_utf8;
                *fresh87 = 0 as *mut libc::c_char;
                current_block = 15652330335145281839;
            }
            match current_block {
                15652330335145281839 => {
                    let ref mut fresh88 = (*cff_subset).widths;
                    *fresh88 = calloc(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
                    ) as *mut libc::c_double;
                    if ((*cff_subset).widths).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    } else {
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < (*(*font).scaled_font_subset).num_glyphs {
                            *((*cff_subset).widths)
                                .offset(
                                    i as isize,
                                ) = *((*font).widths).offset(i as isize) as libc::c_double
                                / (*font).units_per_em as libc::c_double;
                            i = i.wrapping_add(1);
                        }
                        (*cff_subset)
                            .x_min = (*font).x_min as libc::c_double
                            / (*font).units_per_em as libc::c_double;
                        (*cff_subset)
                            .y_min = (*font).y_min as libc::c_double
                            / (*font).units_per_em as libc::c_double;
                        (*cff_subset)
                            .x_max = (*font).x_max as libc::c_double
                            / (*font).units_per_em as libc::c_double;
                        (*cff_subset)
                            .y_max = (*font).y_max as libc::c_double
                            / (*font).units_per_em as libc::c_double;
                        (*cff_subset)
                            .ascent = (*font).ascent as libc::c_double
                            / (*font).units_per_em as libc::c_double;
                        (*cff_subset)
                            .descent = (*font).descent as libc::c_double
                            / (*font).units_per_em as libc::c_double;
                        let ref mut fresh89 = (*cff_subset).data;
                        *fresh89 = (if length != 0 as libc::c_int as libc::c_ulong {
                            malloc(length)
                        } else {
                            0 as *mut libc::c_void
                        }) as *mut libc::c_char;
                        if ((*cff_subset).data).is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                            free((*cff_subset).widths as *mut libc::c_void);
                        } else {
                            memcpy(
                                (*cff_subset).data as *mut libc::c_void,
                                data as *const libc::c_void,
                                length,
                            );
                            (*cff_subset).data_length = length;
                            cairo_cff_font_destroy(font);
                            return CAIRO_STATUS_SUCCESS;
                        }
                    }
                    free((*cff_subset).family_name_utf8 as *mut libc::c_void);
                }
                _ => {}
            }
            free((*cff_subset).ps_name as *mut libc::c_void);
        }
    }
    cairo_cff_font_destroy(font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cff_subset_fini(mut subset: *mut cairo_cff_subset_t) {
    free((*subset).ps_name as *mut libc::c_void);
    free((*subset).family_name_utf8 as *mut libc::c_void);
    free((*subset).widths as *mut libc::c_void);
    free((*subset).data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cff_scaled_font_is_cid_cff(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_bool_t {
    let mut current_block: u64;
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_length: libc::c_ulong = 0;
    let mut current_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut header: *mut cff_header_t = 0 as *mut cff_header_t;
    let mut element: *mut cff_index_element_t = 0 as *mut cff_index_element_t;
    let mut top_dict: *mut cairo_hash_table_t = 0 as *mut cairo_hash_table_t;
    let mut index: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    let mut size: libc::c_int = 0;
    let mut is_cid: cairo_bool_t = 0 as libc::c_int;
    backend = (*scaled_font).backend;
    data = 0 as *mut libc::c_uchar;
    data_length = 0 as libc::c_int as libc::c_ulong;
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if ((*backend).load_truetype_table).is_some()
        && {
            status = ((*backend).load_truetype_table)
                .expect(
                    "non-null function pointer",
                )(
                scaled_font as *mut libc::c_void,
                (('C' as i32 as uint32_t) << 24 as libc::c_int
                    | (('F' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (('F' as i32) << 8 as libc::c_int) as libc::c_uint
                    | ' ' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
                0 as libc::c_int as libc::c_long,
                0 as *mut libc::c_uchar,
                &mut data_length,
            );
            status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        }
    {
        data = (if data_length != 0 as libc::c_int as libc::c_ulong {
            malloc(data_length)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_uchar;
        if data.is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            return 0 as libc::c_int;
        }
        status = ((*backend).load_truetype_table)
            .expect(
                "non-null function pointer",
            )(
            scaled_font as *mut libc::c_void,
            (('C' as i32 as uint32_t) << 24 as libc::c_int
                | (('F' as i32) << 16 as libc::c_int) as libc::c_uint
                | (('F' as i32) << 8 as libc::c_int) as libc::c_uint
                | ' ' as i32 as libc::c_uint) as libc::c_int as libc::c_ulong,
            0 as libc::c_int as libc::c_long,
            data,
            &mut data_length,
        );
        if status as u64 != 0 {
            current_block = 17041026465945031016;
        } else {
            current_block = 8457315219000651999;
        }
    } else {
        current_block = 8457315219000651999;
    }
    match current_block {
        8457315219000651999 => {
            if status as libc::c_uint
                == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                && ((*backend).load_type1_data).is_some()
                && {
                    status = ((*backend).load_type1_data)
                        .expect(
                            "non-null function pointer",
                        )(
                        scaled_font as *mut libc::c_void,
                        0 as libc::c_int as libc::c_long,
                        0 as *mut libc::c_uchar,
                        &mut data_length,
                    );
                    status as libc::c_uint
                        == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                }
            {
                data = (if data_length != 0 as libc::c_int as libc::c_ulong {
                    malloc(data_length)
                } else {
                    0 as *mut libc::c_void
                }) as *mut libc::c_uchar;
                if data.is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                    return 0 as libc::c_int;
                }
                status = ((*backend).load_type1_data)
                    .expect(
                        "non-null function pointer",
                    )(
                    scaled_font as *mut libc::c_void,
                    0 as libc::c_int as libc::c_long,
                    data,
                    &mut data_length,
                );
                if status as u64 != 0 {
                    current_block = 17041026465945031016;
                } else {
                    current_block = 16203760046146113240;
                }
            } else {
                current_block = 16203760046146113240;
            }
            match current_block {
                17041026465945031016 => {}
                _ => {
                    if !(status as u64 != 0) {
                        if !(check_fontdata_is_cff(data, data_length as libc::c_long)
                            == 0)
                        {
                            data_end = data.offset(data_length as isize);
                            if !(data_length
                                < ::std::mem::size_of::<cff_header_t>() as libc::c_ulong)
                            {
                                header = data as *mut cff_header_t;
                                current_ptr = data
                                    .offset((*header).header_size as libc::c_int as isize);
                                cff_index_init(&mut index);
                                status = cff_index_read(
                                    &mut index,
                                    &mut current_ptr,
                                    data_end,
                                );
                                cff_index_fini(&mut index);
                                if !(status as u64 != 0) {
                                    cff_index_init(&mut index);
                                    status = cff_index_read(
                                        &mut index,
                                        &mut current_ptr,
                                        data_end,
                                    );
                                    if !(status as u64 != 0) {
                                        status = cff_dict_init(&mut top_dict) as cairo_int_status_t;
                                        if !(status as u64 != 0) {
                                            element = _cairo_array_index(
                                                &mut index,
                                                0 as libc::c_int as libc::c_uint,
                                            ) as *mut cff_index_element_t;
                                            status = cff_dict_read(
                                                top_dict,
                                                (*element).data,
                                                (*element).length,
                                            ) as cairo_int_status_t;
                                            if !(status as u64 != 0) {
                                                if !(cff_dict_get_operands(
                                                    top_dict,
                                                    0xc1e as libc::c_int as libc::c_ushort,
                                                    &mut size,
                                                ))
                                                    .is_null()
                                                {
                                                    is_cid = 1 as libc::c_int;
                                                }
                                            }
                                            cff_dict_fini(top_dict);
                                        }
                                    }
                                    cff_index_fini(&mut index);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free(data as *mut libc::c_void);
    return is_cid;
}
unsafe extern "C" fn _cairo_cff_font_fallback_create(
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut font_return: *mut *mut cairo_cff_font_t,
    mut subset_name: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut font: *mut cairo_cff_font_t = 0 as *mut cairo_cff_font_t;
    font = (if ::std::mem::size_of::<cairo_cff_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_cff_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_cff_font_t;
    if font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh90 = (*font).backend;
    *fresh90 = 0 as *const cairo_scaled_font_backend_t;
    let ref mut fresh91 = (*font).scaled_font_subset;
    *fresh91 = scaled_font_subset;
    _cairo_array_init(
        &mut (*font).output,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    );
    status = _cairo_array_grow_by(
        &mut (*font).output,
        4096 as libc::c_int as libc::c_uint,
    );
    if !(status as u64 != 0) {
        let ref mut fresh92 = (*font).subset_font_name;
        *fresh92 = strdup(subset_name);
        if ((*font).subset_font_name).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            let ref mut fresh93 = (*font).ps_name;
            *fresh93 = strdup(subset_name);
            if ((*font).ps_name).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                let ref mut fresh94 = (*font).font_name;
                *fresh94 = 0 as *mut libc::c_char;
                (*font).x_min = 0 as libc::c_int;
                (*font).y_min = 0 as libc::c_int;
                (*font).x_max = 0 as libc::c_int;
                (*font).y_max = 0 as libc::c_int;
                (*font).ascent = 0 as libc::c_int;
                (*font).descent = 0 as libc::c_int;
                let ref mut fresh95 = (*font).widths;
                *fresh95 = calloc(
                    (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                if ((*font).widths).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                } else {
                    (*font).data_length = 0 as libc::c_int as libc::c_ulong;
                    let ref mut fresh96 = (*font).data;
                    *fresh96 = 0 as *mut libc::c_uchar;
                    let ref mut fresh97 = (*font).data_end;
                    *fresh97 = 0 as *mut libc::c_uchar;
                    status = cff_dict_init(&mut (*font).top_dict);
                    if !(status as u64 != 0) {
                        status = cff_dict_init(&mut (*font).private_dict);
                        if status as u64 != 0 {
                            _cairo_hash_table_destroy((*font).top_dict);
                        } else {
                            cff_index_init(&mut (*font).strings_index);
                            cff_index_init(&mut (*font).charstrings_index);
                            cff_index_init(&mut (*font).global_sub_index);
                            cff_index_init(&mut (*font).local_sub_index);
                            cff_index_init(&mut (*font).charstrings_subset_index);
                            cff_index_init(&mut (*font).strings_subset_index);
                            let ref mut fresh98 = (*font).global_subs_used;
                            *fresh98 = 0 as *mut cairo_bool_t;
                            let ref mut fresh99 = (*font).local_subs_used;
                            *fresh99 = 0 as *mut cairo_bool_t;
                            (*font).subset_subroutines = 0 as libc::c_int;
                            let ref mut fresh100 = (*font).fdselect;
                            *fresh100 = 0 as *mut libc::c_int;
                            let ref mut fresh101 = (*font).fd_dict;
                            *fresh101 = 0 as *mut *mut cairo_hash_table_t;
                            let ref mut fresh102 = (*font).fd_private_dict;
                            *fresh102 = 0 as *mut *mut cairo_hash_table_t;
                            let ref mut fresh103 = (*font).fd_local_sub_index;
                            *fresh103 = 0 as *mut cairo_array_t;
                            let ref mut fresh104 = (*font).fdselect_subset;
                            *fresh104 = 0 as *mut libc::c_int;
                            let ref mut fresh105 = (*font).fd_subset_map;
                            *fresh105 = 0 as *mut libc::c_int;
                            let ref mut fresh106 = (*font).private_dict_offset;
                            *fresh106 = 0 as *mut libc::c_int;
                            *font_return = font;
                            return CAIRO_STATUS_SUCCESS as libc::c_int
                                as cairo_int_status_t;
                        }
                    }
                    free((*font).widths as *mut libc::c_void);
                }
                free((*font).font_name as *mut libc::c_void);
                free((*font).ps_name as *mut libc::c_void);
            }
            free((*font).subset_font_name as *mut libc::c_void);
        }
    }
    _cairo_array_fini(&mut (*font).output);
    free(font as *mut libc::c_void);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn cairo_cff_font_fallback_generate(
    mut font: *mut cairo_cff_font_t,
    mut type2_subset: *mut cairo_type2_charstrings_t,
    mut data: *mut *const libc::c_char,
    mut length: *mut libc::c_ulong,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut header: cff_header_t = cff_header_t {
        major: 0,
        minor: 0,
        header_size: 0,
        offset_size: 0,
    };
    let mut charstring: *mut cairo_array_t = 0 as *mut cairo_array_t;
    let mut buf: [libc::c_uchar; 40] = [0; 40];
    let mut end_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end_buf2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut sid: libc::c_int = 0;
    header.major = 1 as libc::c_int as uint8_t;
    header.minor = 0 as libc::c_int as uint8_t;
    header.header_size = 4 as libc::c_int as uint8_t;
    header.offset_size = 4 as libc::c_int as uint8_t;
    let ref mut fresh107 = (*font).header;
    *fresh107 = &mut header;
    (*font).is_cid = 0 as libc::c_int;
    snprintf(
        buf.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong,
        b"CairoFont-%u-%u\0" as *const u8 as *const libc::c_char,
        (*(*font).scaled_font_subset).font_id,
        (*(*font).scaled_font_subset).subset_id,
    );
    sid = (391 as libc::c_int as libc::c_uint)
        .wrapping_add(_cairo_array_num_elements(&mut (*font).strings_subset_index))
        as libc::c_int;
    status = cff_index_append_copy(
        &mut (*font).strings_subset_index,
        buf.as_mut_ptr(),
        strlen(buf.as_mut_ptr() as *mut libc::c_char) as libc::c_uint,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    end_buf = encode_integer(buf.as_mut_ptr(), sid);
    status = cff_dict_set_operands(
        (*font).top_dict,
        0x2 as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = cff_dict_set_operands(
        (*font).top_dict,
        0x3 as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    end_buf = encode_integer(buf.as_mut_ptr(), (*type2_subset).x_min as libc::c_int);
    end_buf = encode_integer(end_buf, (*type2_subset).y_min as libc::c_int);
    end_buf = encode_integer(end_buf, (*type2_subset).x_max as libc::c_int);
    end_buf = encode_integer(end_buf, (*type2_subset).y_max as libc::c_int);
    status = cff_dict_set_operands(
        (*font).top_dict,
        0x5 as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    end_buf = encode_integer_max(buf.as_mut_ptr(), 0 as libc::c_int);
    status = cff_dict_set_operands(
        (*font).top_dict,
        0x11 as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if (*(*font).scaled_font_subset).is_latin != 0 {
        status = cff_dict_set_operands(
            (*font).top_dict,
            0x10 as libc::c_int as libc::c_ushort,
            buf.as_mut_ptr(),
            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        end_buf2 = encode_integer_max(end_buf, 0 as libc::c_int);
        cff_dict_set_operands(
            (*font).top_dict,
            0x12 as libc::c_int as libc::c_ushort,
            buf.as_mut_ptr(),
            end_buf2.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
    } else {
        status = cff_dict_set_operands(
            (*font).top_dict,
            0xc25 as libc::c_int as libc::c_ushort,
            buf.as_mut_ptr(),
            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        status = cff_dict_set_operands(
            (*font).top_dict,
            0xc24 as libc::c_int as libc::c_ushort,
            buf.as_mut_ptr(),
            end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
    }
    status = cff_dict_set_operands(
        (*font).top_dict,
        0xf as libc::c_int as libc::c_ushort,
        buf.as_mut_ptr(),
        end_buf.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if (*(*font).scaled_font_subset).is_latin == 0 {
        status = cairo_cff_font_set_ros_strings(font) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        status = cairo_cff_font_create_cid_fontdict(font) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
    } else {
        let ref mut fresh108 = (*font).private_dict_offset;
        *fresh108 = (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_int;
        if ((*font).private_dict_offset).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*(*font).scaled_font_subset).num_glyphs {
        charstring = _cairo_array_index(&mut (*type2_subset).charstrings, i)
            as *mut cairo_array_t;
        status = cff_index_append(
            &mut (*font).charstrings_subset_index,
            _cairo_array_index(charstring, 0 as libc::c_int as libc::c_uint)
                as *mut libc::c_uchar,
            _cairo_array_num_elements(charstring) as libc::c_int,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        i = i.wrapping_add(1);
    }
    if (*(*font).scaled_font_subset).is_latin != 0 {
        status = cairo_cff_font_add_euro_charset_string(font) as cairo_int_status_t;
    }
    status = cairo_cff_font_write_subset(font) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    *data = _cairo_array_index(&mut (*font).output, 0 as libc::c_int as libc::c_uint)
        as *const libc::c_char;
    *length = _cairo_array_num_elements(&mut (*font).output) as libc::c_ulong;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cff_fallback_init(
    mut cff_subset: *mut cairo_cff_subset_t,
    mut subset_name: *const libc::c_char,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    let mut font: *mut cairo_cff_font_t = 0 as *mut cairo_cff_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_uint = 0;
    let mut type2_subset: cairo_type2_charstrings_t = cairo_type2_charstrings_t {
        widths: 0 as *mut libc::c_int,
        x_min: 0,
        y_min: 0,
        x_max: 0,
        y_max: 0,
        ascent: 0,
        descent: 0,
        charstrings: cairo_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *mut libc::c_char,
        },
    };
    status = _cairo_cff_font_fallback_create(font_subset, &mut font, subset_name)
        as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_type2_charstrings_init(&mut type2_subset, font_subset);
    if !(status as u64 != 0) {
        status = cairo_cff_font_fallback_generate(
            font,
            &mut type2_subset,
            &mut data,
            &mut length,
        ) as cairo_status_t;
        if !(status as u64 != 0) {
            let ref mut fresh109 = (*cff_subset).family_name_utf8;
            *fresh109 = 0 as *mut libc::c_char;
            let ref mut fresh110 = (*cff_subset).ps_name;
            *fresh110 = strdup((*font).ps_name);
            if ((*cff_subset).ps_name).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                let ref mut fresh111 = (*cff_subset).widths;
                *fresh111 = calloc(
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
                ) as *mut libc::c_double;
                if ((*cff_subset).widths).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                } else {
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < (*(*font).scaled_font_subset).num_glyphs {
                        *((*cff_subset).widths)
                            .offset(
                                i as isize,
                            ) = *(type2_subset.widths).offset(i as isize)
                            as libc::c_double / 1000 as libc::c_int as libc::c_double;
                        i = i.wrapping_add(1);
                    }
                    (*cff_subset)
                        .x_min = type2_subset.x_min as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    (*cff_subset)
                        .y_min = type2_subset.y_min as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    (*cff_subset)
                        .x_max = type2_subset.x_max as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    (*cff_subset)
                        .y_max = type2_subset.y_max as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    (*cff_subset)
                        .ascent = type2_subset.y_max as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    (*cff_subset)
                        .descent = type2_subset.y_min as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    let ref mut fresh112 = (*cff_subset).data;
                    *fresh112 = (if length != 0 as libc::c_int as libc::c_ulong {
                        malloc(length)
                    } else {
                        0 as *mut libc::c_void
                    }) as *mut libc::c_char;
                    if ((*cff_subset).data).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                        free((*cff_subset).widths as *mut libc::c_void);
                    } else {
                        memcpy(
                            (*cff_subset).data as *mut libc::c_void,
                            data as *const libc::c_void,
                            length,
                        );
                        (*cff_subset).data_length = length;
                        _cairo_type2_charstrings_fini(&mut type2_subset);
                        cairo_cff_font_destroy(font);
                        return CAIRO_STATUS_SUCCESS;
                    }
                }
                free((*cff_subset).ps_name as *mut libc::c_void);
            }
        }
        _cairo_type2_charstrings_fini(&mut type2_subset);
    }
    cairo_cff_font_destroy(font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_cff_fallback_fini(mut subset: *mut cairo_cff_subset_t) {
    free((*subset).ps_name as *mut libc::c_void);
    free((*subset).widths as *mut libc::c_void);
    free((*subset).data as *mut libc::c_void);
}
