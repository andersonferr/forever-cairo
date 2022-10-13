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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t,
        hint_style: cairo_hint_style_t,
    );
    fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t,
        hint_metrics: cairo_hint_metrics_t,
    );
    fn cairo_scaled_font_create(
        font_face: *mut cairo_font_face_t,
        font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t,
        options: *const cairo_font_options_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_reference(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_scaled_font_get_font_face(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_font_face_t;
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
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
    fn _cairo_utf8_to_ucs4(
        str: *const libc::c_char,
        len: libc::c_int,
        result: *mut *mut uint32_t,
        items_written: *mut libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_ucs4_to_utf8(unicode: uint32_t, utf8: *mut libc::c_char) -> libc::c_int;
    fn _cairo_utf8_to_utf16(
        str: *const libc::c_char,
        len: libc::c_int,
        result: *mut *mut uint16_t,
        items_written: *mut libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_cff_scaled_font_is_cid_cff(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> cairo_bool_t;
    fn _cairo_winansi_to_glyphname(glyph: libc::c_int) -> *const libc::c_char;
    fn _cairo_type1_scaled_font_is_type1(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> cairo_bool_t;
    fn _cairo_truetype_index_to_ucs4(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        ucs4: *mut uint32_t,
    ) -> cairo_int_status_t;
    fn _cairo_font_face_is_user(font_face: *mut cairo_font_face_t) -> cairo_bool_t;
}
pub type size_t = libc::c_ulong;
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
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_subsets {
    pub type_0: cairo_subsets_type_t,
    pub use_latin_subset: cairo_bool_t,
    pub max_glyphs_per_unscaled_subset_used: libc::c_int,
    pub unscaled_sub_fonts: *mut cairo_hash_table_t,
    pub unscaled_sub_fonts_list: *mut cairo_sub_font_t,
    pub unscaled_sub_fonts_list_end: *mut cairo_sub_font_t,
    pub max_glyphs_per_scaled_subset_used: libc::c_int,
    pub scaled_sub_fonts: *mut cairo_hash_table_t,
    pub scaled_sub_fonts_list: *mut cairo_sub_font_t,
    pub scaled_sub_fonts_list_end: *mut cairo_sub_font_t,
    pub num_sub_fonts: libc::c_int,
}
pub type cairo_sub_font_t = _cairo_sub_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_sub_font {
    pub base: cairo_hash_entry_t,
    pub is_scaled: cairo_bool_t,
    pub is_composite: cairo_bool_t,
    pub is_user: cairo_bool_t,
    pub use_latin_subset: cairo_bool_t,
    pub reserve_notdef: cairo_bool_t,
    pub parent: *mut cairo_scaled_font_subsets_t,
    pub scaled_font: *mut cairo_scaled_font_t,
    pub font_id: libc::c_uint,
    pub current_subset: libc::c_int,
    pub num_glyphs_in_current_subset: libc::c_int,
    pub num_glyphs_in_latin_subset: libc::c_int,
    pub max_glyphs_per_subset: libc::c_int,
    pub latin_char_map: [libc::c_char; 256],
    pub sub_font_glyphs: *mut cairo_hash_table_t,
    pub next: *mut _cairo_sub_font,
}
pub type cairo_scaled_font_subsets_t = _cairo_scaled_font_subsets;
pub type cairo_subsets_type_t = libc::c_uint;
pub const CAIRO_SUBSETS_COMPOSITE: cairo_subsets_type_t = 2;
pub const CAIRO_SUBSETS_SIMPLE: cairo_subsets_type_t = 1;
pub const CAIRO_SUBSETS_SCALED: cairo_subsets_type_t = 0;
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
pub struct _cairo_scaled_font_subsets_glyph {
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub subset_glyph_index: libc::c_uint,
    pub is_scaled: cairo_bool_t,
    pub is_composite: cairo_bool_t,
    pub is_latin: cairo_bool_t,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
    pub utf8_is_mapped: cairo_bool_t,
    pub unicode: uint32_t,
}
pub type cairo_scaled_font_subsets_glyph_t = _cairo_scaled_font_subsets_glyph;
pub type cairo_sub_font_glyph_t = _cairo_sub_font_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_sub_font_glyph {
    pub base: cairo_hash_entry_t,
    pub subset_id: libc::c_uint,
    pub subset_glyph_index: libc::c_uint,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
    pub is_latin: cairo_bool_t,
    pub latin_character: libc::c_int,
    pub is_mapped: cairo_bool_t,
    pub unicode: uint32_t,
    pub utf8: *mut libc::c_char,
    pub utf8_len: libc::c_int,
}
pub type cairo_scaled_font_subset_callback_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_subset_t,
        *mut libc::c_void,
    ) -> cairo_int_status_t,
>;
pub type cairo_subsets_foreach_type_t = libc::c_uint;
pub const CAIRO_SUBSETS_FOREACH_USER: cairo_subsets_foreach_type_t = 2;
pub const CAIRO_SUBSETS_FOREACH_SCALED: cairo_subsets_foreach_type_t = 1;
pub const CAIRO_SUBSETS_FOREACH_UNSCALED: cairo_subsets_foreach_type_t = 0;
pub type cairo_sub_font_collection_t = _cairo_sub_font_collection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_sub_font_collection {
    pub glyphs: *mut libc::c_ulong,
    pub utf8: *mut *mut libc::c_char,
    pub glyphs_size: libc::c_uint,
    pub to_latin_char: *mut libc::c_int,
    pub latin_to_subset_glyph_index: *mut libc::c_ulong,
    pub max_glyph: libc::c_uint,
    pub num_glyphs: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub status: cairo_status_t,
    pub font_subset_callback: cairo_scaled_font_subset_callback_func_t,
    pub font_subset_callback_closure: *mut libc::c_void,
}
pub type cairo_string_entry_t = _cairo_string_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_string_entry {
    pub base: cairo_hash_entry_t,
    pub string: *mut libc::c_char,
}
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
unsafe extern "C" fn _cairo_sub_font_glyph_init_key(
    mut sub_font_glyph: *mut cairo_sub_font_glyph_t,
    mut scaled_font_glyph_index: libc::c_ulong,
) {
    (*sub_font_glyph).base.hash = scaled_font_glyph_index;
}
unsafe extern "C" fn _cairo_sub_font_glyph_create(
    mut scaled_font_glyph_index: libc::c_ulong,
    mut subset_id: libc::c_uint,
    mut subset_glyph_index: libc::c_uint,
    mut x_advance: libc::c_double,
    mut y_advance: libc::c_double,
    mut latin_character: libc::c_int,
    mut unicode: uint32_t,
    mut utf8: *mut libc::c_char,
    mut utf8_len: libc::c_int,
) -> *mut cairo_sub_font_glyph_t {
    let mut sub_font_glyph: *mut cairo_sub_font_glyph_t = 0
        as *mut cairo_sub_font_glyph_t;
    sub_font_glyph = (if ::std::mem::size_of::<cairo_sub_font_glyph_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_sub_font_glyph_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_sub_font_glyph_t;
    if sub_font_glyph.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_sub_font_glyph_t;
    }
    _cairo_sub_font_glyph_init_key(sub_font_glyph, scaled_font_glyph_index);
    (*sub_font_glyph).subset_id = subset_id;
    (*sub_font_glyph).subset_glyph_index = subset_glyph_index;
    (*sub_font_glyph).x_advance = x_advance;
    (*sub_font_glyph).y_advance = y_advance;
    (*sub_font_glyph).is_latin = (latin_character >= 0 as libc::c_int) as libc::c_int;
    (*sub_font_glyph).latin_character = latin_character;
    (*sub_font_glyph).is_mapped = 0 as libc::c_int;
    (*sub_font_glyph).unicode = unicode;
    let ref mut fresh2 = (*sub_font_glyph).utf8;
    *fresh2 = utf8;
    (*sub_font_glyph).utf8_len = utf8_len;
    return sub_font_glyph;
}
unsafe extern "C" fn _cairo_sub_font_glyph_destroy(
    mut sub_font_glyph: *mut cairo_sub_font_glyph_t,
) {
    free((*sub_font_glyph).utf8 as *mut libc::c_void);
    free(sub_font_glyph as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_sub_font_glyph_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut sub_font_glyph: *mut cairo_sub_font_glyph_t = entry
        as *mut cairo_sub_font_glyph_t;
    let mut sub_font_glyphs: *mut cairo_hash_table_t = closure
        as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(sub_font_glyphs, &mut (*sub_font_glyph).base);
    _cairo_sub_font_glyph_destroy(sub_font_glyph);
}
unsafe extern "C" fn _cairo_sub_font_glyph_collect(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut sub_font_glyph: *mut cairo_sub_font_glyph_t = entry
        as *mut cairo_sub_font_glyph_t;
    let mut collection: *mut cairo_sub_font_collection_t = closure
        as *mut cairo_sub_font_collection_t;
    let mut scaled_font_glyph_index: libc::c_ulong = 0;
    let mut subset_glyph_index: libc::c_uint = 0;
    if (*sub_font_glyph).subset_id != (*collection).subset_id {
        return;
    }
    scaled_font_glyph_index = (*sub_font_glyph).base.hash;
    subset_glyph_index = (*sub_font_glyph).subset_glyph_index;
    if subset_glyph_index < (*collection).glyphs_size {} else {
        __assert_fail(
            b"subset_glyph_index < collection->glyphs_size\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-scaled-font-subsets.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void _cairo_sub_font_glyph_collect(void *, void *)\0"))
                .as_ptr(),
        );
    }
    *((*collection).glyphs)
        .offset(subset_glyph_index as isize) = scaled_font_glyph_index;
    let ref mut fresh3 = *((*collection).utf8).offset(subset_glyph_index as isize);
    *fresh3 = (*sub_font_glyph).utf8;
    *((*collection).to_latin_char)
        .offset(subset_glyph_index as isize) = (*sub_font_glyph).latin_character;
    if (*sub_font_glyph).is_latin != 0 {
        *((*collection).latin_to_subset_glyph_index)
            .offset(
                (*sub_font_glyph).latin_character as isize,
            ) = subset_glyph_index as libc::c_ulong;
    }
    if subset_glyph_index > (*collection).max_glyph {
        (*collection).max_glyph = subset_glyph_index;
    }
    let ref mut fresh4 = (*collection).num_glyphs;
    *fresh4 = (*fresh4).wrapping_add(1);
}
unsafe extern "C" fn _cairo_sub_fonts_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut sub_font_a: *const cairo_sub_font_t = key_a as *const cairo_sub_font_t;
    let mut sub_font_b: *const cairo_sub_font_t = key_b as *const cairo_sub_font_t;
    let mut a: *mut cairo_scaled_font_t = (*sub_font_a).scaled_font;
    let mut b: *mut cairo_scaled_font_t = (*sub_font_b).scaled_font;
    if (*sub_font_a).is_scaled != 0 {
        return (a == b) as libc::c_int
    } else {
        return ((*a).font_face == (*b).font_face
            || (*a).original_font_face == (*b).original_font_face) as libc::c_int
    };
}
unsafe extern "C" fn _cairo_sub_font_init_key(
    mut sub_font: *mut cairo_sub_font_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    if (*sub_font).is_scaled != 0 {
        (*sub_font).base.hash = scaled_font as uintptr_t;
        let ref mut fresh5 = (*sub_font).scaled_font;
        *fresh5 = scaled_font;
    } else {
        (*sub_font).base.hash = (*scaled_font).font_face as uintptr_t;
        let ref mut fresh6 = (*sub_font).scaled_font;
        *fresh6 = scaled_font;
    };
}
unsafe extern "C" fn _cairo_sub_font_create(
    mut parent: *mut cairo_scaled_font_subsets_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_id: libc::c_uint,
    mut max_glyphs_per_subset: libc::c_int,
    mut is_scaled: cairo_bool_t,
    mut is_composite: cairo_bool_t,
    mut sub_font_out: *mut *mut cairo_sub_font_t,
) -> cairo_status_t {
    let mut sub_font: *mut cairo_sub_font_t = 0 as *mut cairo_sub_font_t;
    let mut i: libc::c_int = 0;
    sub_font = (if ::std::mem::size_of::<cairo_sub_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_sub_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_sub_font_t;
    if sub_font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*sub_font).is_scaled = is_scaled;
    (*sub_font).is_composite = is_composite;
    (*sub_font).is_user = _cairo_font_face_is_user((*scaled_font).font_face);
    (*sub_font).reserve_notdef = ((*sub_font).is_user == 0) as libc::c_int;
    _cairo_sub_font_init_key(sub_font, scaled_font);
    let ref mut fresh7 = (*sub_font).parent;
    *fresh7 = parent;
    let ref mut fresh8 = (*sub_font).scaled_font;
    *fresh8 = scaled_font;
    (*sub_font).font_id = font_id;
    (*sub_font).use_latin_subset = (*parent).use_latin_subset;
    if (*sub_font).is_user != 0 || (*sub_font).is_scaled != 0
        || _cairo_cff_scaled_font_is_cid_cff(scaled_font) != 0
    {
        (*sub_font).use_latin_subset = 0 as libc::c_int;
    }
    if (*sub_font).use_latin_subset != 0 {
        (*sub_font).current_subset = 1 as libc::c_int;
    } else {
        (*sub_font).current_subset = 0 as libc::c_int;
    }
    (*sub_font).num_glyphs_in_current_subset = 0 as libc::c_int;
    (*sub_font).num_glyphs_in_latin_subset = 0 as libc::c_int;
    (*sub_font).max_glyphs_per_subset = max_glyphs_per_subset;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*sub_font).latin_char_map[i as usize] = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    let ref mut fresh9 = (*sub_font).sub_font_glyphs;
    *fresh9 = _cairo_hash_table_create(None);
    if ((*sub_font).sub_font_glyphs).is_null() {
        free(sub_font as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh10 = (*sub_font).next;
    *fresh10 = 0 as *mut _cairo_sub_font;
    *sub_font_out = sub_font;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_sub_font_destroy(mut sub_font: *mut cairo_sub_font_t) {
    _cairo_hash_table_foreach(
        (*sub_font).sub_font_glyphs,
        Some(
            _cairo_sub_font_glyph_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*sub_font).sub_font_glyphs as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*sub_font).sub_font_glyphs);
    cairo_scaled_font_destroy((*sub_font).scaled_font);
    free(sub_font as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_sub_font_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut sub_font: *mut cairo_sub_font_t = entry as *mut cairo_sub_font_t;
    let mut sub_fonts: *mut cairo_hash_table_t = closure as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(sub_fonts, &mut (*sub_font).base);
    _cairo_sub_font_destroy(sub_font);
}
static mut _winansi_0x80_to_0x9f: [libc::c_uint; 32] = [
    0x20ac as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x201a as libc::c_int as libc::c_uint,
    0x192 as libc::c_int as libc::c_uint,
    0x201e as libc::c_int as libc::c_uint,
    0x2026 as libc::c_int as libc::c_uint,
    0x2020 as libc::c_int as libc::c_uint,
    0x2021 as libc::c_int as libc::c_uint,
    0x2c6 as libc::c_int as libc::c_uint,
    0x2030 as libc::c_int as libc::c_uint,
    0x160 as libc::c_int as libc::c_uint,
    0x2039 as libc::c_int as libc::c_uint,
    0x152 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x17d as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x2018 as libc::c_int as libc::c_uint,
    0x2019 as libc::c_int as libc::c_uint,
    0x201c as libc::c_int as libc::c_uint,
    0x201d as libc::c_int as libc::c_uint,
    0x2022 as libc::c_int as libc::c_uint,
    0x2013 as libc::c_int as libc::c_uint,
    0x2014 as libc::c_int as libc::c_uint,
    0x2dc as libc::c_int as libc::c_uint,
    0x2122 as libc::c_int as libc::c_uint,
    0x161 as libc::c_int as libc::c_uint,
    0x203a as libc::c_int as libc::c_uint,
    0x153 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0x17e as libc::c_int as libc::c_uint,
    0x178 as libc::c_int as libc::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_unicode_to_winansi(
    mut uni: libc::c_ulong,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if uni >= 0x20 as libc::c_int as libc::c_ulong
        && uni <= 0x7e as libc::c_int as libc::c_ulong
        || uni >= 0xa1 as libc::c_int as libc::c_ulong
            && uni <= 0xff as libc::c_int as libc::c_ulong
            && uni != 0xad as libc::c_int as libc::c_ulong
        || uni == 0 as libc::c_int as libc::c_ulong
    {
        return uni as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if _winansi_0x80_to_0x9f[i as usize] as libc::c_ulong == uni {
            return i + 0x80 as libc::c_int;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn _cairo_sub_font_glyph_lookup_unicode(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_font_glyph_index: libc::c_ulong,
    mut unicode_out: *mut uint32_t,
    mut utf8_out: *mut *mut libc::c_char,
    mut utf8_len_out: *mut libc::c_int,
) -> cairo_status_t {
    let mut unicode: uint32_t = 0;
    let mut buf: [libc::c_char; 8] = [0; 8];
    let mut len: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    unicode = -(1 as libc::c_int) as uint32_t;
    status = _cairo_truetype_index_to_ucs4(
        scaled_font,
        scaled_font_glyph_index,
        &mut unicode,
    ) as cairo_status_t;
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status;
    }
    if unicode == -(1 as libc::c_int) as uint32_t
        && ((*(*scaled_font).backend).index_to_ucs4).is_some()
    {
        status = ((*(*scaled_font).backend).index_to_ucs4)
            .expect(
                "non-null function pointer",
            )(scaled_font as *mut libc::c_void, scaled_font_glyph_index, &mut unicode)
            as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
    }
    *unicode_out = unicode;
    *utf8_out = 0 as *mut libc::c_char;
    *utf8_len_out = 0 as libc::c_int;
    if unicode != -(1 as libc::c_int) as uint32_t {
        len = _cairo_ucs4_to_utf8(unicode, buf.as_mut_ptr());
        if len > 0 as libc::c_int {
            *utf8_out = (if len + 1 as libc::c_int != 0 as libc::c_int {
                malloc((len + 1 as libc::c_int) as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut libc::c_char;
            if (*utf8_out).is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            memcpy(
                *utf8_out as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                len as libc::c_ulong,
            );
            *(*utf8_out).offset(len as isize) = 0 as libc::c_int as libc::c_char;
            *utf8_len_out = len;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_sub_font_glyph_map_to_unicode(
    mut sub_font_glyph: *mut cairo_sub_font_glyph_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut is_mapped: *mut cairo_bool_t,
) -> cairo_status_t {
    *is_mapped = 0 as libc::c_int;
    if utf8_len < 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    if !utf8.is_null() && utf8_len != 0 as libc::c_int
        && *utf8.offset((utf8_len - 1 as libc::c_int) as isize) as libc::c_int
            == '\0' as i32
    {
        utf8_len -= 1;
    }
    if !utf8.is_null() && utf8_len != 0 as libc::c_int {
        if !((*sub_font_glyph).utf8).is_null() {
            if utf8_len == (*sub_font_glyph).utf8_len
                && memcmp(
                    utf8 as *const libc::c_void,
                    (*sub_font_glyph).utf8 as *const libc::c_void,
                    utf8_len as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                *is_mapped = 1 as libc::c_int;
            }
        } else {
            let ref mut fresh11 = (*sub_font_glyph).utf8;
            *fresh11 = (if utf8_len + 1 as libc::c_int != 0 as libc::c_int {
                malloc((utf8_len + 1 as libc::c_int) as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut libc::c_char;
            if ((*sub_font_glyph).utf8).is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            memcpy(
                (*sub_font_glyph).utf8 as *mut libc::c_void,
                utf8 as *const libc::c_void,
                utf8_len as libc::c_ulong,
            );
            *((*sub_font_glyph).utf8)
                .offset(utf8_len as isize) = 0 as libc::c_int as libc::c_char;
            (*sub_font_glyph).utf8_len = utf8_len;
            *is_mapped = 1 as libc::c_int;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_sub_font_lookup_glyph(
    mut sub_font: *mut cairo_sub_font_t,
    mut scaled_font_glyph_index: libc::c_ulong,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut subset_glyph: *mut cairo_scaled_font_subsets_glyph_t,
) -> cairo_int_status_t {
    let mut key: cairo_sub_font_glyph_t = cairo_sub_font_glyph_t {
        base: cairo_hash_entry_t { hash: 0 },
        subset_id: 0,
        subset_glyph_index: 0,
        x_advance: 0.,
        y_advance: 0.,
        is_latin: 0,
        latin_character: 0,
        is_mapped: 0,
        unicode: 0,
        utf8: 0 as *mut libc::c_char,
        utf8_len: 0,
    };
    let mut sub_font_glyph: *mut cairo_sub_font_glyph_t = 0
        as *mut cairo_sub_font_glyph_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    _cairo_sub_font_glyph_init_key(&mut key, scaled_font_glyph_index);
    sub_font_glyph = _cairo_hash_table_lookup((*sub_font).sub_font_glyphs, &mut key.base)
        as *mut cairo_sub_font_glyph_t;
    if !sub_font_glyph.is_null() {
        (*subset_glyph).font_id = (*sub_font).font_id;
        (*subset_glyph).subset_id = (*sub_font_glyph).subset_id;
        if (*sub_font_glyph).is_latin != 0 {
            (*subset_glyph)
                .subset_glyph_index = (*sub_font_glyph).latin_character as libc::c_uint;
        } else {
            (*subset_glyph).subset_glyph_index = (*sub_font_glyph).subset_glyph_index;
        }
        (*subset_glyph).is_scaled = (*sub_font).is_scaled;
        (*subset_glyph).is_composite = (*sub_font).is_composite;
        (*subset_glyph).is_latin = (*sub_font_glyph).is_latin;
        (*subset_glyph).x_advance = (*sub_font_glyph).x_advance;
        (*subset_glyph).y_advance = (*sub_font_glyph).y_advance;
        status = _cairo_sub_font_glyph_map_to_unicode(
            sub_font_glyph,
            utf8,
            utf8_len,
            &mut (*subset_glyph).utf8_is_mapped,
        ) as cairo_int_status_t;
        (*subset_glyph).unicode = (*sub_font_glyph).unicode;
        return status;
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
unsafe extern "C" fn _cairo_sub_font_add_glyph(
    mut sub_font: *mut cairo_sub_font_t,
    mut scaled_font_glyph_index: libc::c_ulong,
    mut is_latin: cairo_bool_t,
    mut latin_character: libc::c_int,
    mut unicode: uint32_t,
    mut utf8: *mut libc::c_char,
    mut utf8_len: libc::c_int,
    mut sub_font_glyph_out: *mut *mut cairo_sub_font_glyph_t,
) -> cairo_status_t {
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut sub_font_glyph: *mut cairo_sub_font_glyph_t = 0
        as *mut cairo_sub_font_glyph_t;
    let mut num_glyphs_in_subset_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x_advance: libc::c_double = 0.;
    let mut y_advance: libc::c_double = 0.;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    _cairo_scaled_font_freeze_cache((*sub_font).scaled_font);
    status = _cairo_scaled_glyph_lookup(
        (*sub_font).scaled_font,
        scaled_font_glyph_index,
        CAIRO_SCALED_GLYPH_INFO_METRICS,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    );
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status != CAIRO_INT_STATUS_UNSUPPORTED\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-scaled-font-subsets.c\0" as *const u8 as *const libc::c_char,
            518 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 145],
                &[libc::c_char; 145],
            >(
                b"cairo_status_t _cairo_sub_font_add_glyph(cairo_sub_font_t *, unsigned long, cairo_bool_t, int, uint32_t, char *, int, cairo_sub_font_glyph_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if status as u64 != 0 {
        _cairo_scaled_font_thaw_cache((*sub_font).scaled_font);
        return status as cairo_status_t;
    }
    x_advance = (*scaled_glyph).metrics.x_advance;
    y_advance = (*scaled_glyph).metrics.y_advance;
    _cairo_scaled_font_thaw_cache((*sub_font).scaled_font);
    if is_latin == 0
        && (*sub_font).num_glyphs_in_current_subset == (*sub_font).max_glyphs_per_subset
    {
        let ref mut fresh12 = (*sub_font).current_subset;
        *fresh12 += 1;
        (*sub_font).num_glyphs_in_current_subset = 0 as libc::c_int;
    }
    if is_latin != 0 {
        num_glyphs_in_subset_ptr = &mut (*sub_font).num_glyphs_in_latin_subset;
    } else {
        num_glyphs_in_subset_ptr = &mut (*sub_font).num_glyphs_in_current_subset;
    }
    if *num_glyphs_in_subset_ptr == 0 as libc::c_int && (*sub_font).reserve_notdef != 0 {
        *num_glyphs_in_subset_ptr += 1;
    }
    sub_font_glyph = _cairo_sub_font_glyph_create(
        scaled_font_glyph_index,
        (if is_latin != 0 { 0 as libc::c_int } else { (*sub_font).current_subset })
            as libc::c_uint,
        *num_glyphs_in_subset_ptr as libc::c_uint,
        x_advance,
        y_advance,
        if is_latin != 0 { latin_character } else { -(1 as libc::c_int) },
        unicode,
        utf8,
        utf8_len,
    );
    if sub_font_glyph.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _cairo_hash_table_insert(
        (*sub_font).sub_font_glyphs,
        &mut (*sub_font_glyph).base,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        _cairo_sub_font_glyph_destroy(sub_font_glyph);
        return status as cairo_status_t;
    }
    *num_glyphs_in_subset_ptr += 1;
    if (*sub_font).is_scaled != 0 {
        if *num_glyphs_in_subset_ptr
            > (*(*sub_font).parent).max_glyphs_per_scaled_subset_used
        {
            (*(*sub_font).parent)
                .max_glyphs_per_scaled_subset_used = *num_glyphs_in_subset_ptr;
        }
    } else if *num_glyphs_in_subset_ptr
        > (*(*sub_font).parent).max_glyphs_per_unscaled_subset_used
    {
        (*(*sub_font).parent)
            .max_glyphs_per_unscaled_subset_used = *num_glyphs_in_subset_ptr;
    }
    *sub_font_glyph_out = sub_font_glyph;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_sub_font_map_glyph(
    mut sub_font: *mut cairo_sub_font_t,
    mut scaled_font_glyph_index: libc::c_ulong,
    mut text_utf8: *const libc::c_char,
    mut text_utf8_len: libc::c_int,
    mut subset_glyph: *mut cairo_scaled_font_subsets_glyph_t,
) -> cairo_status_t {
    let mut key: cairo_sub_font_glyph_t = cairo_sub_font_glyph_t {
        base: cairo_hash_entry_t { hash: 0 },
        subset_id: 0,
        subset_glyph_index: 0,
        x_advance: 0.,
        y_advance: 0.,
        is_latin: 0,
        latin_character: 0,
        is_mapped: 0,
        unicode: 0,
        utf8: 0 as *mut libc::c_char,
        utf8_len: 0,
    };
    let mut sub_font_glyph: *mut cairo_sub_font_glyph_t = 0
        as *mut cairo_sub_font_glyph_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_sub_font_glyph_init_key(&mut key, scaled_font_glyph_index);
    sub_font_glyph = _cairo_hash_table_lookup((*sub_font).sub_font_glyphs, &mut key.base)
        as *mut cairo_sub_font_glyph_t;
    if sub_font_glyph.is_null() {
        let mut font_unicode: uint32_t = 0;
        let mut font_utf8: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut font_utf8_len: libc::c_int = 0;
        let mut is_latin: cairo_bool_t = 0;
        let mut latin_character: libc::c_int = 0;
        status = _cairo_sub_font_glyph_lookup_unicode(
            (*sub_font).scaled_font,
            scaled_font_glyph_index,
            &mut font_unicode,
            &mut font_utf8,
            &mut font_utf8_len,
        );
        if status as u64 != 0 {
            return status;
        }
        if !text_utf8.is_null() && text_utf8_len > 0 as libc::c_int {
            let mut ucs4: *mut uint32_t = 0 as *mut uint32_t;
            let mut ucs4_len: libc::c_int = 0;
            status = _cairo_utf8_to_ucs4(
                text_utf8,
                text_utf8_len,
                &mut ucs4,
                &mut ucs4_len,
            );
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                if ucs4_len == 1 as libc::c_int {
                    font_unicode = *ucs4.offset(0 as libc::c_int as isize);
                    free(font_utf8 as *mut libc::c_void);
                    font_utf8 = (if text_utf8_len + 1 as libc::c_int != 0 as libc::c_int
                    {
                        malloc((text_utf8_len + 1 as libc::c_int) as libc::c_ulong)
                    } else {
                        0 as *mut libc::c_void
                    }) as *mut libc::c_char;
                    if font_utf8.is_null() {
                        free(ucs4 as *mut libc::c_void);
                        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    }
                    memcpy(
                        font_utf8 as *mut libc::c_void,
                        text_utf8 as *const libc::c_void,
                        text_utf8_len as libc::c_ulong,
                    );
                    *font_utf8
                        .offset(
                            text_utf8_len as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                    font_utf8_len = text_utf8_len;
                }
                free(ucs4 as *mut libc::c_void);
            }
        }
        is_latin = 0 as libc::c_int;
        latin_character = -(1 as libc::c_int);
        if (*sub_font).use_latin_subset != 0
            && _cairo_font_face_is_user((*(*sub_font).scaled_font).font_face) == 0
        {
            latin_character = _cairo_unicode_to_winansi(font_unicode as libc::c_ulong);
            if latin_character > 0 as libc::c_int {
                if (*sub_font).latin_char_map[latin_character as usize] == 0 {
                    (*sub_font)
                        .latin_char_map[latin_character
                        as usize] = 1 as libc::c_int as libc::c_char;
                    is_latin = 1 as libc::c_int;
                }
            }
        }
        status = _cairo_sub_font_add_glyph(
            sub_font,
            scaled_font_glyph_index,
            is_latin,
            latin_character,
            font_unicode,
            font_utf8,
            font_utf8_len,
            &mut sub_font_glyph,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    (*subset_glyph).font_id = (*sub_font).font_id;
    (*subset_glyph).subset_id = (*sub_font_glyph).subset_id;
    if (*sub_font_glyph).is_latin != 0 {
        (*subset_glyph)
            .subset_glyph_index = (*sub_font_glyph).latin_character as libc::c_uint;
    } else {
        (*subset_glyph).subset_glyph_index = (*sub_font_glyph).subset_glyph_index;
    }
    (*subset_glyph).is_scaled = (*sub_font).is_scaled;
    (*subset_glyph).is_composite = (*sub_font).is_composite;
    (*subset_glyph).is_latin = (*sub_font_glyph).is_latin;
    (*subset_glyph).x_advance = (*sub_font_glyph).x_advance;
    (*subset_glyph).y_advance = (*sub_font_glyph).y_advance;
    status = _cairo_sub_font_glyph_map_to_unicode(
        sub_font_glyph,
        text_utf8,
        text_utf8_len,
        &mut (*subset_glyph).utf8_is_mapped,
    );
    (*subset_glyph).unicode = (*sub_font_glyph).unicode;
    return status;
}
unsafe extern "C" fn _cairo_sub_font_collect(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut sub_font: *mut cairo_sub_font_t = entry as *mut cairo_sub_font_t;
    let mut collection: *mut cairo_sub_font_collection_t = closure
        as *mut cairo_sub_font_collection_t;
    let mut subset: cairo_scaled_font_subset_t = cairo_scaled_font_subset_t {
        scaled_font: 0 as *mut cairo_scaled_font_t,
        font_id: 0,
        subset_id: 0,
        glyphs: 0 as *mut libc::c_ulong,
        utf8: 0 as *mut *mut libc::c_char,
        glyph_names: 0 as *mut *mut libc::c_char,
        to_latin_char: 0 as *mut libc::c_int,
        latin_to_subset_glyph_index: 0 as *mut libc::c_ulong,
        num_glyphs: 0,
        is_composite: 0,
        is_scaled: 0,
        is_latin: 0,
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    if (*collection).status as u64 != 0 {
        return;
    }
    (*collection).status = (*(*sub_font).scaled_font).status;
    if (*collection).status as u64 != 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i <= (*sub_font).current_subset {
        (*collection).subset_id = i as libc::c_uint;
        (*collection).num_glyphs = 0 as libc::c_int as libc::c_uint;
        (*collection).max_glyph = 0 as libc::c_int as libc::c_uint;
        memset(
            (*collection).latin_to_subset_glyph_index as *mut libc::c_void,
            0 as libc::c_int,
            (256 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
        );
        if (*sub_font).reserve_notdef != 0 {
            *((*collection).glyphs)
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_ulong;
            let ref mut fresh13 = *((*collection).utf8)
                .offset(0 as libc::c_int as isize);
            *fresh13 = 0 as *mut libc::c_char;
            *((*collection).to_latin_char)
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
            *((*collection).latin_to_subset_glyph_index)
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_ulong;
            let ref mut fresh14 = (*collection).num_glyphs;
            *fresh14 = (*fresh14).wrapping_add(1);
        }
        _cairo_hash_table_foreach(
            (*sub_font).sub_font_glyphs,
            Some(
                _cairo_sub_font_glyph_collect
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            collection as *mut libc::c_void,
        );
        if (*collection).status as u64 != 0 {
            break;
        }
        if !((*collection).num_glyphs == 0 as libc::c_int as libc::c_uint) {
            if !((*sub_font).reserve_notdef != 0
                && (*collection).num_glyphs == 1 as libc::c_int as libc::c_uint)
            {
                if (*collection).num_glyphs
                    == ((*collection).max_glyph)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                {} else {
                    __assert_fail(
                        b"collection->num_glyphs == collection->max_glyph + 1\0"
                            as *const u8 as *const libc::c_char,
                        b"../src/cairo-scaled-font-subsets.c\0" as *const u8
                            as *const libc::c_char,
                        720 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 45],
                            &[libc::c_char; 45],
                        >(b"void _cairo_sub_font_collect(void *, void *)\0"))
                            .as_ptr(),
                    );
                }
                subset.scaled_font = (*sub_font).scaled_font;
                subset.is_composite = (*sub_font).is_composite;
                subset.is_scaled = (*sub_font).is_scaled;
                subset.font_id = (*sub_font).font_id;
                subset.subset_id = i as libc::c_uint;
                subset.glyphs = (*collection).glyphs;
                subset.utf8 = (*collection).utf8;
                subset.num_glyphs = (*collection).num_glyphs;
                subset.glyph_names = 0 as *mut *mut libc::c_char;
                subset.is_latin = 0 as libc::c_int;
                if (*sub_font).use_latin_subset != 0 && i == 0 as libc::c_int {
                    subset.is_latin = 1 as libc::c_int;
                    subset.to_latin_char = (*collection).to_latin_char;
                    subset
                        .latin_to_subset_glyph_index = (*collection)
                        .latin_to_subset_glyph_index;
                } else {
                    subset.to_latin_char = 0 as *mut libc::c_int;
                    subset.latin_to_subset_glyph_index = 0 as *mut libc::c_ulong;
                }
                (*collection)
                    .status = ((*collection).font_subset_callback)
                    .expect(
                        "non-null function pointer",
                    )(&mut subset, (*collection).font_subset_callback_closure)
                    as cairo_status_t;
                if !(subset.glyph_names).is_null() {
                    j = 0 as libc::c_int as libc::c_uint;
                    while j < (*collection).num_glyphs {
                        free(
                            *(subset.glyph_names).offset(j as isize) as *mut libc::c_void,
                        );
                        j = j.wrapping_add(1);
                    }
                    free(subset.glyph_names as *mut libc::c_void);
                }
                if (*collection).status as u64 != 0 {
                    break;
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn _cairo_scaled_font_subsets_create_internal(
    mut type_0: cairo_subsets_type_t,
) -> *mut cairo_scaled_font_subsets_t {
    let mut subsets: *mut cairo_scaled_font_subsets_t = 0
        as *mut cairo_scaled_font_subsets_t;
    subsets = (if ::std::mem::size_of::<cairo_scaled_font_subsets_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_scaled_font_subsets_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_scaled_font_subsets_t;
    if subsets.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_scaled_font_subsets_t;
    }
    (*subsets).type_0 = type_0;
    (*subsets).use_latin_subset = 0 as libc::c_int;
    (*subsets).max_glyphs_per_unscaled_subset_used = 0 as libc::c_int;
    (*subsets).max_glyphs_per_scaled_subset_used = 0 as libc::c_int;
    (*subsets).num_sub_fonts = 0 as libc::c_int;
    let ref mut fresh15 = (*subsets).unscaled_sub_fonts;
    *fresh15 = _cairo_hash_table_create(
        Some(
            _cairo_sub_fonts_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if ((*subsets).unscaled_sub_fonts).is_null() {
        free(subsets as *mut libc::c_void);
        return 0 as *mut cairo_scaled_font_subsets_t;
    }
    let ref mut fresh16 = (*subsets).unscaled_sub_fonts_list;
    *fresh16 = 0 as *mut cairo_sub_font_t;
    let ref mut fresh17 = (*subsets).unscaled_sub_fonts_list_end;
    *fresh17 = 0 as *mut cairo_sub_font_t;
    let ref mut fresh18 = (*subsets).scaled_sub_fonts;
    *fresh18 = _cairo_hash_table_create(
        Some(
            _cairo_sub_fonts_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if ((*subsets).scaled_sub_fonts).is_null() {
        _cairo_hash_table_destroy((*subsets).unscaled_sub_fonts);
        free(subsets as *mut libc::c_void);
        return 0 as *mut cairo_scaled_font_subsets_t;
    }
    let ref mut fresh19 = (*subsets).scaled_sub_fonts_list;
    *fresh19 = 0 as *mut cairo_sub_font_t;
    let ref mut fresh20 = (*subsets).scaled_sub_fonts_list_end;
    *fresh20 = 0 as *mut cairo_sub_font_t;
    return subsets;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_create_scaled() -> *mut cairo_scaled_font_subsets_t {
    return _cairo_scaled_font_subsets_create_internal(CAIRO_SUBSETS_SCALED);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_create_simple() -> *mut cairo_scaled_font_subsets_t {
    return _cairo_scaled_font_subsets_create_internal(CAIRO_SUBSETS_SIMPLE);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_create_composite() -> *mut cairo_scaled_font_subsets_t {
    return _cairo_scaled_font_subsets_create_internal(CAIRO_SUBSETS_COMPOSITE);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_destroy(
    mut subsets: *mut cairo_scaled_font_subsets_t,
) {
    _cairo_hash_table_foreach(
        (*subsets).scaled_sub_fonts,
        Some(
            _cairo_sub_font_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*subsets).scaled_sub_fonts as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*subsets).scaled_sub_fonts);
    _cairo_hash_table_foreach(
        (*subsets).unscaled_sub_fonts,
        Some(
            _cairo_sub_font_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*subsets).unscaled_sub_fonts as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*subsets).unscaled_sub_fonts);
    free(subsets as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_enable_latin_subset(
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut use_latin: cairo_bool_t,
) {
    (*font_subsets).use_latin_subset = use_latin;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_map_glyph(
    mut subsets: *mut cairo_scaled_font_subsets_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_font_glyph_index: libc::c_ulong,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut subset_glyph: *mut cairo_scaled_font_subsets_glyph_t,
) -> cairo_status_t {
    let mut key: cairo_sub_font_t = cairo_sub_font_t {
        base: cairo_hash_entry_t { hash: 0 },
        is_scaled: 0,
        is_composite: 0,
        is_user: 0,
        use_latin_subset: 0,
        reserve_notdef: 0,
        parent: 0 as *mut cairo_scaled_font_subsets_t,
        scaled_font: 0 as *mut cairo_scaled_font_t,
        font_id: 0,
        current_subset: 0,
        num_glyphs_in_current_subset: 0,
        num_glyphs_in_latin_subset: 0,
        max_glyphs_per_subset: 0,
        latin_char_map: [0; 256],
        sub_font_glyphs: 0 as *mut cairo_hash_table_t,
        next: 0 as *mut _cairo_sub_font,
    };
    let mut sub_font: *mut cairo_sub_font_t = 0 as *mut cairo_sub_font_t;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    let mut identity: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut font_options: cairo_font_options_t = cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    };
    let mut unscaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut max_glyphs: libc::c_int = 0;
    let mut type1_font: cairo_bool_t = 0;
    if (*subsets).type_0 as libc::c_uint
        != CAIRO_SUBSETS_SCALED as libc::c_int as libc::c_uint
    {
        key.is_scaled = 0 as libc::c_int;
        _cairo_sub_font_init_key(&mut key, scaled_font);
        sub_font = _cairo_hash_table_lookup((*subsets).unscaled_sub_fonts, &mut key.base)
            as *mut cairo_sub_font_t;
        if !sub_font.is_null() {
            status = _cairo_sub_font_lookup_glyph(
                sub_font,
                scaled_font_glyph_index,
                utf8,
                utf8_len,
                subset_glyph,
            );
            if status as libc::c_uint
                != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
            {
                return status as cairo_status_t;
            }
        }
    }
    key.is_scaled = 1 as libc::c_int;
    _cairo_sub_font_init_key(&mut key, scaled_font);
    sub_font = _cairo_hash_table_lookup((*subsets).scaled_sub_fonts, &mut key.base)
        as *mut cairo_sub_font_t;
    if !sub_font.is_null() {
        status = _cairo_sub_font_lookup_glyph(
            sub_font,
            scaled_font_glyph_index,
            utf8,
            utf8_len,
            subset_glyph,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    if scaled_font_glyph_index == 0 as libc::c_int as libc::c_ulong
        || _cairo_font_face_is_user((*scaled_font).font_face) != 0
    {
        status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    } else {
        _cairo_scaled_font_freeze_cache(scaled_font);
        status = _cairo_scaled_glyph_lookup(
            scaled_font,
            scaled_font_glyph_index,
            CAIRO_SCALED_GLYPH_INFO_PATH,
            0 as *const cairo_color_t,
            &mut scaled_glyph,
        );
        _cairo_scaled_font_thaw_cache(scaled_font);
    }
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status as cairo_status_t;
    }
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*subsets).type_0 as libc::c_uint
            != CAIRO_SUBSETS_SCALED as libc::c_int as libc::c_uint
        && _cairo_font_face_is_user((*scaled_font).font_face) == 0
    {
        key.is_scaled = 0 as libc::c_int;
        _cairo_sub_font_init_key(&mut key, scaled_font);
        sub_font = _cairo_hash_table_lookup((*subsets).unscaled_sub_fonts, &mut key.base)
            as *mut cairo_sub_font_t;
        if sub_font.is_null() {
            font_face = cairo_scaled_font_get_font_face(scaled_font);
            cairo_matrix_init_identity(&mut identity);
            _cairo_font_options_init_default(&mut font_options);
            cairo_font_options_set_hint_style(&mut font_options, CAIRO_HINT_STYLE_NONE);
            cairo_font_options_set_hint_metrics(
                &mut font_options,
                CAIRO_HINT_METRICS_OFF,
            );
            unscaled_font = cairo_scaled_font_create(
                font_face,
                &mut identity,
                &mut identity,
                &mut font_options,
            );
            if (*unscaled_font).status as u64 != 0 {
                return (*unscaled_font).status;
            }
            (*subset_glyph).is_scaled = 0 as libc::c_int;
            type1_font = _cairo_type1_scaled_font_is_type1(unscaled_font);
            if (*subsets).type_0 as libc::c_uint
                == CAIRO_SUBSETS_COMPOSITE as libc::c_int as libc::c_uint
                && type1_font == 0
            {
                max_glyphs = 65536 as libc::c_int;
                (*subset_glyph).is_composite = 1 as libc::c_int;
            } else {
                max_glyphs = 256 as libc::c_int;
                (*subset_glyph).is_composite = 0 as libc::c_int;
            }
            status = _cairo_sub_font_create(
                subsets,
                unscaled_font,
                (*subsets).num_sub_fonts as libc::c_uint,
                max_glyphs,
                (*subset_glyph).is_scaled,
                (*subset_glyph).is_composite,
                &mut sub_font,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                cairo_scaled_font_destroy(unscaled_font);
                return status as cairo_status_t;
            }
            status = _cairo_hash_table_insert(
                (*subsets).unscaled_sub_fonts,
                &mut (*sub_font).base,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                _cairo_sub_font_destroy(sub_font);
                return status as cairo_status_t;
            }
            if ((*subsets).unscaled_sub_fonts_list).is_null() {
                let ref mut fresh21 = (*subsets).unscaled_sub_fonts_list;
                *fresh21 = sub_font;
            } else {
                let ref mut fresh22 = (*(*subsets).unscaled_sub_fonts_list_end).next;
                *fresh22 = sub_font;
            }
            let ref mut fresh23 = (*subsets).unscaled_sub_fonts_list_end;
            *fresh23 = sub_font;
            let ref mut fresh24 = (*subsets).num_sub_fonts;
            *fresh24 += 1;
        }
    } else {
        key.is_scaled = 1 as libc::c_int;
        _cairo_sub_font_init_key(&mut key, scaled_font);
        sub_font = _cairo_hash_table_lookup((*subsets).scaled_sub_fonts, &mut key.base)
            as *mut cairo_sub_font_t;
        if sub_font.is_null() {
            (*subset_glyph).is_scaled = 1 as libc::c_int;
            (*subset_glyph).is_composite = 0 as libc::c_int;
            if (*subsets).type_0 as libc::c_uint
                == CAIRO_SUBSETS_SCALED as libc::c_int as libc::c_uint
            {
                max_glyphs = 2147483647 as libc::c_int;
            } else {
                max_glyphs = 256 as libc::c_int;
            }
            status = _cairo_sub_font_create(
                subsets,
                cairo_scaled_font_reference(scaled_font),
                (*subsets).num_sub_fonts as libc::c_uint,
                max_glyphs,
                (*subset_glyph).is_scaled,
                (*subset_glyph).is_composite,
                &mut sub_font,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                cairo_scaled_font_destroy(scaled_font);
                return status as cairo_status_t;
            }
            status = _cairo_hash_table_insert(
                (*subsets).scaled_sub_fonts,
                &mut (*sub_font).base,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                _cairo_sub_font_destroy(sub_font);
                return status as cairo_status_t;
            }
            if ((*subsets).scaled_sub_fonts_list).is_null() {
                let ref mut fresh25 = (*subsets).scaled_sub_fonts_list;
                *fresh25 = sub_font;
            } else {
                let ref mut fresh26 = (*(*subsets).scaled_sub_fonts_list_end).next;
                *fresh26 = sub_font;
            }
            let ref mut fresh27 = (*subsets).scaled_sub_fonts_list_end;
            *fresh27 = sub_font;
            let ref mut fresh28 = (*subsets).num_sub_fonts;
            *fresh28 += 1;
        }
    }
    return _cairo_sub_font_map_glyph(
        sub_font,
        scaled_font_glyph_index,
        utf8,
        utf8_len,
        subset_glyph,
    );
}
unsafe extern "C" fn _cairo_scaled_font_subsets_foreach_internal(
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut font_subset_callback: cairo_scaled_font_subset_callback_func_t,
    mut closure: *mut libc::c_void,
    mut type_0: cairo_subsets_foreach_type_t,
) -> cairo_status_t {
    let mut collection: cairo_sub_font_collection_t = cairo_sub_font_collection_t {
        glyphs: 0 as *mut libc::c_ulong,
        utf8: 0 as *mut *mut libc::c_char,
        glyphs_size: 0,
        to_latin_char: 0 as *mut libc::c_int,
        latin_to_subset_glyph_index: 0 as *mut libc::c_ulong,
        max_glyph: 0,
        num_glyphs: 0,
        subset_id: 0,
        status: CAIRO_STATUS_SUCCESS,
        font_subset_callback: None,
        font_subset_callback_closure: 0 as *mut libc::c_void,
    };
    let mut sub_font: *mut cairo_sub_font_t = 0 as *mut cairo_sub_font_t;
    let mut is_scaled: cairo_bool_t = 0;
    let mut is_user: cairo_bool_t = 0;
    is_scaled = 0 as libc::c_int;
    is_user = 0 as libc::c_int;
    if type_0 as libc::c_uint
        == CAIRO_SUBSETS_FOREACH_USER as libc::c_int as libc::c_uint
    {
        is_user = 1 as libc::c_int;
    }
    if type_0 as libc::c_uint
        == CAIRO_SUBSETS_FOREACH_SCALED as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint
            == CAIRO_SUBSETS_FOREACH_USER as libc::c_int as libc::c_uint
    {
        is_scaled = 1 as libc::c_int;
    }
    if is_scaled != 0 {
        collection
            .glyphs_size = (*font_subsets).max_glyphs_per_scaled_subset_used
            as libc::c_uint;
    } else {
        collection
            .glyphs_size = (*font_subsets).max_glyphs_per_unscaled_subset_used
            as libc::c_uint;
    }
    if collection.glyphs_size == 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    collection
        .glyphs = _cairo_malloc_ab(
        collection.glyphs_size as size_t,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) as *mut libc::c_ulong;
    collection
        .utf8 = _cairo_malloc_ab(
        collection.glyphs_size as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    collection
        .to_latin_char = _cairo_malloc_ab(
        collection.glyphs_size as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    collection
        .latin_to_subset_glyph_index = _cairo_malloc_ab(
        256 as libc::c_int as size_t,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) as *mut libc::c_ulong;
    if (collection.glyphs).is_null() || (collection.utf8).is_null()
        || (collection.to_latin_char).is_null()
        || (collection.latin_to_subset_glyph_index).is_null()
    {
        free(collection.glyphs as *mut libc::c_void);
        free(collection.utf8 as *mut libc::c_void);
        free(collection.to_latin_char as *mut libc::c_void);
        free(collection.latin_to_subset_glyph_index as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    collection.font_subset_callback = font_subset_callback;
    collection.font_subset_callback_closure = closure;
    collection.status = CAIRO_STATUS_SUCCESS;
    if is_scaled != 0 {
        sub_font = (*font_subsets).scaled_sub_fonts_list;
    } else {
        sub_font = (*font_subsets).unscaled_sub_fonts_list;
    }
    while !sub_font.is_null() {
        if (*sub_font).is_user == is_user {
            _cairo_sub_font_collect(
                sub_font as *mut libc::c_void,
                &mut collection as *mut cairo_sub_font_collection_t as *mut libc::c_void,
            );
        }
        sub_font = (*sub_font).next;
    }
    free(collection.utf8 as *mut libc::c_void);
    free(collection.glyphs as *mut libc::c_void);
    free(collection.to_latin_char as *mut libc::c_void);
    free(collection.latin_to_subset_glyph_index as *mut libc::c_void);
    return collection.status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_foreach_scaled(
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut font_subset_callback: cairo_scaled_font_subset_callback_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    return _cairo_scaled_font_subsets_foreach_internal(
        font_subsets,
        font_subset_callback,
        closure,
        CAIRO_SUBSETS_FOREACH_SCALED,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_foreach_unscaled(
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut font_subset_callback: cairo_scaled_font_subset_callback_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    return _cairo_scaled_font_subsets_foreach_internal(
        font_subsets,
        font_subset_callback,
        closure,
        CAIRO_SUBSETS_FOREACH_UNSCALED,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subsets_foreach_user(
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut font_subset_callback: cairo_scaled_font_subset_callback_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    return _cairo_scaled_font_subsets_foreach_internal(
        font_subsets,
        font_subset_callback,
        closure,
        CAIRO_SUBSETS_FOREACH_USER,
    );
}
unsafe extern "C" fn _cairo_string_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_string_entry_t = key_a as *const cairo_string_entry_t;
    let mut b: *const cairo_string_entry_t = key_b as *const cairo_string_entry_t;
    if strcmp((*a).string, (*b).string) == 0 as libc::c_int {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn _cairo_string_init_key(
    mut key: *mut cairo_string_entry_t,
    mut s: *mut libc::c_char,
) {
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < strlen(s) {
        sum = sum.wrapping_add(*s.offset(i as isize) as libc::c_ulong);
        i = i.wrapping_add(1);
    }
    (*key).base.hash = sum;
    let ref mut fresh29 = (*key).string;
    *fresh29 = s;
}
unsafe extern "C" fn create_string_entry(
    mut s: *mut libc::c_char,
    mut entry: *mut *mut cairo_string_entry_t,
) -> cairo_status_t {
    *entry = (if ::std::mem::size_of::<cairo_string_entry_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_string_entry_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_string_entry_t;
    if (*entry).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_string_init_key(*entry, s);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _pluck_entry(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    _cairo_hash_table_remove(
        closure as *mut cairo_hash_table_t,
        entry as *mut cairo_hash_entry_t,
    );
    free(entry);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_subset_create_glyph_names(
    mut subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut names: *mut cairo_hash_table_t = 0 as *mut cairo_hash_table_t;
    let mut key: cairo_string_entry_t = cairo_string_entry_t {
        base: cairo_hash_entry_t { hash: 0 },
        string: 0 as *mut libc::c_char,
    };
    let mut entry: *mut cairo_string_entry_t = 0 as *mut cairo_string_entry_t;
    let mut buf: [libc::c_char; 30] = [0; 30];
    let mut utf8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut utf16: *mut uint16_t = 0 as *mut uint16_t;
    let mut utf16_len: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    names = _cairo_hash_table_create(
        Some(
            _cairo_string_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if names.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh30 = (*subset).glyph_names;
    *fresh30 = calloc(
        (*subset).num_glyphs as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if ((*subset).glyph_names).is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        if (*subset).is_scaled == 0 {
            let ref mut fresh31 = *((*subset).glyph_names)
                .offset(0 as libc::c_int as isize);
            *fresh31 = strdup(b".notdef\0" as *const u8 as *const libc::c_char);
            if (*((*subset).glyph_names).offset(0 as libc::c_int as isize)).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                current_block = 16952125948255566013;
            } else {
                status = create_string_entry(
                    *((*subset).glyph_names).offset(0 as libc::c_int as isize),
                    &mut entry,
                );
                if status as u64 != 0 {
                    current_block = 16952125948255566013;
                } else {
                    status = _cairo_hash_table_insert(names, &mut (*entry).base);
                    if status as u64 != 0 {
                        free(entry as *mut libc::c_void);
                        current_block = 16952125948255566013;
                    } else {
                        i = i.wrapping_add(1);
                        current_block = 14401909646449704462;
                    }
                }
            }
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            16952125948255566013 => {}
            _ => {
                while i < (*subset).num_glyphs {
                    utf8 = *((*subset).utf8).offset(i as isize);
                    utf16 = 0 as *mut uint16_t;
                    utf16_len = 0 as libc::c_int;
                    if !utf8.is_null() && *utf8 as libc::c_int != 0 {
                        status = _cairo_utf8_to_utf16(
                            utf8,
                            -(1 as libc::c_int),
                            &mut utf16,
                            &mut utf16_len,
                        );
                        if status as libc::c_uint
                            == CAIRO_STATUS_INVALID_STRING as libc::c_int as libc::c_uint
                        {
                            utf16 = 0 as *mut uint16_t;
                            utf16_len = 0 as libc::c_int;
                        } else if status as u64 != 0 {
                            break;
                        }
                    }
                    if utf16_len == 1 as libc::c_int {
                        let mut ch: libc::c_int = _cairo_unicode_to_winansi(
                            *utf16.offset(0 as libc::c_int as isize) as libc::c_ulong,
                        );
                        if ch > 0 as libc::c_int
                            && !(_cairo_winansi_to_glyphname(ch)).is_null()
                        {
                            strncpy(
                                buf.as_mut_ptr(),
                                _cairo_winansi_to_glyphname(ch),
                                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                            );
                            buf[(::std::mem::size_of::<[libc::c_char; 30]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as usize] = '\0' as i32 as libc::c_char;
                        } else {
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 30]>()
                                    as libc::c_ulong,
                                b"uni%04X\0" as *const u8 as *const libc::c_char,
                                *utf16.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                        _cairo_string_init_key(&mut key, buf.as_mut_ptr());
                        entry = _cairo_hash_table_lookup(names, &mut key.base)
                            as *mut cairo_string_entry_t;
                        if !entry.is_null() {
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 30]>()
                                    as libc::c_ulong,
                                b"g%d\0" as *const u8 as *const libc::c_char,
                                i,
                            );
                        }
                    } else {
                        snprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                            b"g%d\0" as *const u8 as *const libc::c_char,
                            i,
                        );
                    }
                    free(utf16 as *mut libc::c_void);
                    let ref mut fresh32 = *((*subset).glyph_names).offset(i as isize);
                    *fresh32 = strdup(buf.as_mut_ptr());
                    if (*((*subset).glyph_names).offset(i as isize)).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                        break;
                    } else {
                        status = create_string_entry(
                            *((*subset).glyph_names).offset(i as isize),
                            &mut entry,
                        );
                        if status as u64 != 0 {
                            break;
                        }
                        status = _cairo_hash_table_insert(names, &mut (*entry).base);
                        if status as u64 != 0 {
                            free(entry as *mut libc::c_void);
                            break;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                }
            }
        }
    }
    _cairo_hash_table_foreach(
        names,
        Some(
            _pluck_entry
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        names as *mut libc::c_void,
    );
    _cairo_hash_table_destroy(names);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if !((*subset).glyph_names).is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*subset).num_glyphs {
            free(*((*subset).glyph_names).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*subset).glyph_names as *mut libc::c_void);
        let ref mut fresh33 = (*subset).glyph_names;
        *fresh33 = 0 as *mut *mut libc::c_char;
    }
    return status as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_escape_ps_name(
    mut ps_name: *mut *mut libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !(*ps_name).is_null() {
        static mut reserved: *const libc::c_char = b"()<>[]{}/%#\\\0" as *const u8
            as *const libc::c_char;
        let mut buf: [libc::c_char; 128] = [0; 128];
        let mut src: *mut libc::c_char = *ps_name;
        let mut dst: *mut libc::c_char = buf.as_mut_ptr();
        while *src as libc::c_int != 0
            && dst < buf.as_mut_ptr().offset(127 as libc::c_int as isize)
        {
            let mut c: libc::c_uchar = *src as libc::c_uchar;
            if (c as libc::c_int) < 0x21 as libc::c_int
                || c as libc::c_int > 0x7e as libc::c_int
                || !(strchr(reserved, c as libc::c_int)).is_null()
            {
                if dst.offset(4 as libc::c_int as isize)
                    > buf.as_mut_ptr().offset(127 as libc::c_int as isize)
                {
                    break;
                }
                snprintf(
                    dst,
                    4 as libc::c_int as libc::c_ulong,
                    b"#%02X\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
                src = src.offset(1);
                dst = dst.offset(3 as libc::c_int as isize);
            } else {
                let fresh34 = src;
                src = src.offset(1);
                let fresh35 = dst;
                dst = dst.offset(1);
                *fresh35 = *fresh34;
            }
        }
        *dst = 0 as libc::c_int as libc::c_char;
        free(*ps_name as *mut libc::c_void);
        *ps_name = strdup(buf.as_mut_ptr());
        if (*ps_name).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    return status as cairo_int_status_t;
}
