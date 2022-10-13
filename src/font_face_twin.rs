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
    fn free(_: *mut libc::c_void);
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_tolerance(cr: *mut cairo_t, tolerance: libc::c_double);
    fn cairo_set_line_width(cr: *mut cairo_t, width: libc::c_double);
    fn cairo_set_line_cap(cr: *mut cairo_t, line_cap: cairo_line_cap_t);
    fn cairo_set_line_join(cr: *mut cairo_t, line_join: cairo_line_join_t);
    fn cairo_translate(cr: *mut cairo_t, tx: libc::c_double, ty: libc::c_double);
    fn cairo_scale(cr: *mut cairo_t, sx: libc::c_double, sy: libc::c_double);
    fn cairo_transform(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    fn cairo_user_to_device_distance(
        cr: *mut cairo_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn cairo_move_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);
    fn cairo_line_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);
    fn cairo_curve_to(
        cr: *mut cairo_t,
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
        x3: libc::c_double,
        y3: libc::c_double,
    );
    fn cairo_close_path(cr: *mut cairo_t);
    fn cairo_stroke(cr: *mut cairo_t);
    fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    fn cairo_font_face_get_user_data(
        font_face: *mut cairo_font_face_t,
        key: *const cairo_user_data_key_t,
    ) -> *mut libc::c_void;
    fn cairo_font_face_set_user_data(
        font_face: *mut cairo_font_face_t,
        key: *const cairo_user_data_key_t,
        user_data: *mut libc::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    fn cairo_scaled_font_get_user_data(
        scaled_font: *mut cairo_scaled_font_t,
        key: *const cairo_user_data_key_t,
    ) -> *mut libc::c_void;
    fn cairo_scaled_font_set_user_data(
        scaled_font: *mut cairo_scaled_font_t,
        key: *const cairo_user_data_key_t,
        user_data: *mut libc::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    fn cairo_scaled_font_get_font_face(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_font_face_t;
    fn cairo_user_font_face_create() -> *mut cairo_font_face_t;
    fn cairo_user_font_face_set_init_func(
        font_face: *mut cairo_font_face_t,
        init_func: cairo_user_scaled_font_init_func_t,
    );
    fn cairo_user_font_face_set_render_glyph_func(
        font_face: *mut cairo_font_face_t,
        render_glyph_func: cairo_user_scaled_font_render_glyph_func_t,
    );
    fn cairo_user_font_face_set_unicode_to_glyph_func(
        font_face: *mut cairo_font_face_t,
        unicode_to_glyph_func: cairo_user_scaled_font_unicode_to_glyph_func_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    static _cairo_font_face_nil: cairo_font_face_t;
    static _cairo_twin_charmap: [uint16_t; 128];
    static _cairo_twin_outlines: [int8_t; 0];
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub type cairo_user_scaled_font_init_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        *mut cairo_t,
        *mut cairo_font_extents_t,
    ) -> cairo_status_t,
>;
pub type cairo_user_scaled_font_render_glyph_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        libc::c_ulong,
        *mut cairo_t,
        *mut cairo_text_extents_t,
    ) -> cairo_status_t,
>;
pub type cairo_user_scaled_font_unicode_to_glyph_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> cairo_status_t,
>;
pub type uint16_t = __uint16_t;
pub type twin_face_properties_t = _twin_face_properties;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _twin_face_properties {
    pub slant: cairo_font_slant_t,
    pub weight: twin_face_weight_t,
    pub stretch: twin_face_stretch_t,
    pub monospace: cairo_bool_t,
    pub smallcaps: cairo_bool_t,
}
pub type twin_face_stretch_t = libc::c_uint;
pub const TWIN_STRETCH_ULTRA_EXPANDED: twin_face_stretch_t = 8;
pub const TWIN_STRETCH_EXTRA_EXPANDED: twin_face_stretch_t = 7;
pub const TWIN_STRETCH_EXPANDED: twin_face_stretch_t = 6;
pub const TWIN_STRETCH_SEMI_EXPANDED: twin_face_stretch_t = 5;
pub const TWIN_STRETCH_NORMAL: twin_face_stretch_t = 4;
pub const TWIN_STRETCH_SEMI_CONDENSED: twin_face_stretch_t = 3;
pub const TWIN_STRETCH_CONDENSED: twin_face_stretch_t = 2;
pub const TWIN_STRETCH_EXTRA_CONDENSED: twin_face_stretch_t = 1;
pub const TWIN_STRETCH_ULTRA_CONDENSED: twin_face_stretch_t = 0;
pub type twin_face_weight_t = libc::c_uint;
pub const TWIN_WEIGHT_ULTRAHEAVY: twin_face_weight_t = 1000;
pub const TWIN_WEIGHT_HEAVY: twin_face_weight_t = 900;
pub const TWIN_WEIGHT_ULTRABOLD: twin_face_weight_t = 800;
pub const TWIN_WEIGHT_BOLD: twin_face_weight_t = 700;
pub const TWIN_WEIGHT_SEMIBOLD: twin_face_weight_t = 600;
pub const TWIN_WEIGHT_MEDIUM: twin_face_weight_t = 500;
pub const TWIN_WEIGHT_NORMAL: twin_face_weight_t = 400;
pub const TWIN_WEIGHT_BOOK: twin_face_weight_t = 380;
pub const TWIN_WEIGHT_LIGHT: twin_face_weight_t = 300;
pub const TWIN_WEIGHT_ULTRALIGHT: twin_face_weight_t = 200;
pub const TWIN_WEIGHT_THIN: twin_face_weight_t = 100;
pub type twin_scaled_properties_t = _twin_scaled_properties;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _twin_scaled_properties {
    pub face_props: *mut twin_face_properties_t,
    pub snap: cairo_bool_t,
    pub weight: libc::c_double,
    pub penx: libc::c_double,
    pub peny: libc::c_double,
    pub marginl: libc::c_double,
    pub marginr: libc::c_double,
    pub stretch: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct twin_snap_info_t {
    pub n_snap_x: libc::c_int,
    pub snap_x: [int8_t; 4],
    pub snapped_x: [libc::c_double; 4],
    pub n_snap_y: libc::c_int,
    pub snap_y: [int8_t; 7],
    pub snapped_y: [libc::c_double; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FieldMap {
    pub value: libc::c_int,
    pub str_0: [libc::c_char; 16],
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
static mut twin_properties_key: cairo_user_data_key_t = cairo_user_data_key_t {
    unused: 0,
};
static mut slant_map: [FieldMap; 4] = unsafe {
    [
        {
            let mut init = FieldMap {
                value: CAIRO_FONT_SLANT_NORMAL as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: CAIRO_FONT_SLANT_NORMAL as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Roman\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: CAIRO_FONT_SLANT_OBLIQUE as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Oblique\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: CAIRO_FONT_SLANT_ITALIC as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Italic\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
    ]
};
static mut smallcaps_map: [FieldMap; 2] = unsafe {
    [
        {
            let mut init = FieldMap {
                value: 0 as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: 1 as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Small-Caps\0\0\0\0\0\0"),
            };
            init
        },
    ]
};
static mut weight_map: [FieldMap; 19] = unsafe {
    [
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_THIN as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Thin\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRALIGHT as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Ultra-Light\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRALIGHT as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Extra-Light\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_LIGHT as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Light\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_BOOK as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Book\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_NORMAL as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_NORMAL as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Regular\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_MEDIUM as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Medium\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_SEMIBOLD as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Semi-Bold\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_SEMIBOLD as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Demi-Bold\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_BOLD as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Bold\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRABOLD as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Ultra-Bold\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRABOLD as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Extra-Bold\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_HEAVY as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Heavy\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_HEAVY as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Black\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRAHEAVY as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Ultra-Heavy\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRAHEAVY as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Extra-Heavy\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRAHEAVY as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Ultra-Black\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_WEIGHT_ULTRAHEAVY as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Extra-Black\0\0\0\0\0"),
            };
            init
        },
    ]
};
static mut stretch_map: [FieldMap; 9] = unsafe {
    [
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_ULTRA_CONDENSED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Ultra-Condensed\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_EXTRA_CONDENSED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Extra-Condensed\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_CONDENSED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Condensed\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_SEMI_CONDENSED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Semi-Condensed\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_NORMAL as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_SEMI_EXPANDED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Semi-Expanded\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_EXPANDED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Expanded\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_EXTRA_EXPANDED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Extra-Expanded\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: TWIN_STRETCH_ULTRA_EXPANDED as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Ultra-Expanded\0\0"),
            };
            init
        },
    ]
};
static mut monospace_map: [FieldMap; 3] = unsafe {
    [
        {
            let mut init = FieldMap {
                value: 0 as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: 1 as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Mono\0\0\0\0\0\0\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = FieldMap {
                value: 1 as libc::c_int,
                str_0: *::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"Monospace\0\0\0\0\0\0\0"),
            };
            init
        },
    ]
};
unsafe extern "C" fn field_matches(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut len: libc::c_int,
) -> cairo_bool_t {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    while len != 0 && *s1 as libc::c_int != 0 && *s2 as libc::c_int != 0 {
        c1 = if *s1 as libc::c_int >= 'A' as i32 && *s1 as libc::c_int <= 'Z' as i32 {
            *s1 as libc::c_int - 'A' as i32 + 'a' as i32
        } else {
            *s1 as libc::c_int
        };
        c2 = if *s2 as libc::c_int >= 'A' as i32 && *s2 as libc::c_int <= 'Z' as i32 {
            *s2 as libc::c_int - 'A' as i32 + 'a' as i32
        } else {
            *s2 as libc::c_int
        };
        if c1 != c2 {
            if c1 == '-' as i32 {
                s1 = s1.offset(1);
            } else {
                return 0 as libc::c_int
            }
        } else {
            s1 = s1.offset(1);
            s2 = s2.offset(1);
            len -= 1;
        }
    }
    return (len == 0 as libc::c_int && *s1 as libc::c_int == '\0' as i32) as libc::c_int;
}
unsafe extern "C" fn parse_int(
    mut word: *const libc::c_char,
    mut wordlen: size_t,
    mut out: *mut libc::c_int,
) -> cairo_bool_t {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_long = strtol(word, &mut end, 10 as libc::c_int);
    let mut i: libc::c_int = val as libc::c_int;
    if end != word as *mut libc::c_char
        && end == word.offset(wordlen as isize) as *mut libc::c_char
        && val >= 0 as libc::c_int as libc::c_long && val == i as libc::c_long
    {
        if !out.is_null() {
            *out = i;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_field(
    mut what: *const libc::c_char,
    mut map: *const FieldMap,
    mut n_elements: libc::c_int,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut val: *mut libc::c_int,
) -> cairo_bool_t {
    let mut i: libc::c_int = 0;
    let mut had_prefix: cairo_bool_t = 0 as libc::c_int;
    if !what.is_null() {
        i = strlen(what) as libc::c_int;
        if len > i && 0 as libc::c_int == strncmp(what, str, i as libc::c_ulong)
            && *str.offset(i as isize) as libc::c_int == '=' as i32
        {
            str = str.offset((i + 1 as libc::c_int) as isize);
            len -= i + 1 as libc::c_int;
            had_prefix = 1 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while i < n_elements {
        if (*map.offset(i as isize)).str_0[0 as libc::c_int as usize] as libc::c_int != 0
            && field_matches(((*map.offset(i as isize)).str_0).as_ptr(), str, len) != 0
        {
            if !val.is_null() {
                *val = (*map.offset(i as isize)).value;
            }
            return 1 as libc::c_int;
        }
        i += 1;
    }
    if what.is_null() || had_prefix != 0 {
        return parse_int(str, len as size_t, val);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_field(
    mut props: *mut twin_face_properties_t,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) {
    if field_matches(b"Normal\0" as *const u8 as *const libc::c_char, str, len) != 0 {
        return;
    }
    if find_field(
        b"weight\0" as *const u8 as *const libc::c_char,
        weight_map.as_ptr(),
        (::std::mem::size_of::<[FieldMap; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<FieldMap>() as libc::c_ulong)
            as libc::c_int,
        str,
        len,
        &mut (*props).weight as *mut twin_face_weight_t as *mut libc::c_void
            as *mut libc::c_int,
    ) != 0
    {
        return;
    }
    if find_field(
        b"slant\0" as *const u8 as *const libc::c_char,
        slant_map.as_ptr(),
        (::std::mem::size_of::<[FieldMap; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<FieldMap>() as libc::c_ulong)
            as libc::c_int,
        str,
        len,
        &mut (*props).slant as *mut cairo_font_slant_t as *mut libc::c_void
            as *mut libc::c_int,
    ) != 0
    {
        return;
    }
    if find_field(
        b"stretch\0" as *const u8 as *const libc::c_char,
        stretch_map.as_ptr(),
        (::std::mem::size_of::<[FieldMap; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<FieldMap>() as libc::c_ulong)
            as libc::c_int,
        str,
        len,
        &mut (*props).stretch as *mut twin_face_stretch_t as *mut libc::c_void
            as *mut libc::c_int,
    ) != 0
    {
        return;
    }
    if find_field(
        b"smallcaps\0" as *const u8 as *const libc::c_char,
        smallcaps_map.as_ptr(),
        (::std::mem::size_of::<[FieldMap; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<FieldMap>() as libc::c_ulong)
            as libc::c_int,
        str,
        len,
        &mut (*props).smallcaps as *mut cairo_bool_t as *mut libc::c_void
            as *mut libc::c_int,
    ) != 0
    {
        return;
    }
    if find_field(
        b"monospace\0" as *const u8 as *const libc::c_char,
        monospace_map.as_ptr(),
        (::std::mem::size_of::<[FieldMap; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<FieldMap>() as libc::c_ulong)
            as libc::c_int,
        str,
        len,
        &mut (*props).monospace as *mut cairo_bool_t as *mut libc::c_void
            as *mut libc::c_int,
    ) != 0
    {
        return;
    }
}
unsafe extern "C" fn face_props_parse(
    mut props: *mut twin_face_properties_t,
    mut s: *const libc::c_char,
) {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    end = s;
    start = end;
    while *end != 0 {
        if !(*end as libc::c_int != ' ' as i32 && *end as libc::c_int != ':' as i32) {
            if start < end {
                parse_field(
                    props,
                    start,
                    end.offset_from(start) as libc::c_long as libc::c_int,
                );
            }
            start = end.offset(1 as libc::c_int as isize);
        }
        end = end.offset(1);
    }
    if start < end {
        parse_field(props, start, end.offset_from(start) as libc::c_long as libc::c_int);
    }
}
unsafe extern "C" fn twin_font_face_create_properties(
    mut twin_face: *mut cairo_font_face_t,
) -> *mut twin_face_properties_t {
    let mut props: *mut twin_face_properties_t = 0 as *mut twin_face_properties_t;
    props = (if ::std::mem::size_of::<twin_face_properties_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<twin_face_properties_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut twin_face_properties_t;
    if props.is_null() {
        return 0 as *mut twin_face_properties_t;
    }
    (*props).stretch = TWIN_STRETCH_NORMAL;
    (*props).slant = CAIRO_FONT_SLANT_NORMAL;
    (*props).weight = TWIN_WEIGHT_NORMAL;
    (*props).monospace = 0 as libc::c_int;
    (*props).smallcaps = 0 as libc::c_int;
    if cairo_font_face_set_user_data(
        twin_face,
        &mut twin_properties_key,
        props as *mut libc::c_void,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    ) as u64 != 0
    {
        free(props as *mut libc::c_void);
        return 0 as *mut twin_face_properties_t;
    }
    return props;
}
unsafe extern "C" fn twin_font_face_set_properties_from_toy(
    mut twin_face: *mut cairo_font_face_t,
    mut toy_face: *mut cairo_toy_font_face_t,
) -> cairo_status_t {
    let mut props: *mut twin_face_properties_t = 0 as *mut twin_face_properties_t;
    props = twin_font_face_create_properties(twin_face);
    if props.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*props).slant = (*toy_face).slant;
    (*props)
        .weight = (if (*toy_face).weight as libc::c_uint
        == CAIRO_FONT_WEIGHT_NORMAL as libc::c_int as libc::c_uint
    {
        TWIN_WEIGHT_NORMAL as libc::c_int
    } else {
        TWIN_WEIGHT_BOLD as libc::c_int
    }) as twin_face_weight_t;
    face_props_parse(props, (*toy_face).family);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn compute_hinting_scale(
    mut cr: *mut cairo_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut scale: *mut libc::c_double,
    mut inv: *mut libc::c_double,
) {
    cairo_user_to_device_distance(cr, &mut x, &mut y);
    *scale = if x == 0 as libc::c_int as libc::c_double {
        y
    } else if y == 0 as libc::c_int as libc::c_double {
        x
    } else {
        sqrt(x * x + y * y)
    };
    *inv = 1 as libc::c_int as libc::c_double / *scale;
}
unsafe extern "C" fn compute_hinting_scales(
    mut cr: *mut cairo_t,
    mut x_scale: *mut libc::c_double,
    mut x_scale_inv: *mut libc::c_double,
    mut y_scale: *mut libc::c_double,
    mut y_scale_inv: *mut libc::c_double,
) {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    x = 1 as libc::c_int as libc::c_double;
    y = 0 as libc::c_int as libc::c_double;
    compute_hinting_scale(cr, x, y, x_scale, x_scale_inv);
    x = 0 as libc::c_int as libc::c_double;
    y = 1 as libc::c_int as libc::c_double;
    compute_hinting_scale(cr, x, y, y_scale, y_scale_inv);
}
unsafe extern "C" fn twin_hint_pen_and_margins(
    mut cr: *mut cairo_t,
    mut penx: *mut libc::c_double,
    mut peny: *mut libc::c_double,
    mut marginl: *mut libc::c_double,
    mut marginr: *mut libc::c_double,
) {
    let mut x_scale: libc::c_double = 0.;
    let mut x_scale_inv: libc::c_double = 0.;
    let mut y_scale: libc::c_double = 0.;
    let mut y_scale_inv: libc::c_double = 0.;
    let mut margin: libc::c_double = 0.;
    compute_hinting_scales(
        cr,
        &mut x_scale,
        &mut x_scale_inv,
        &mut y_scale,
        &mut y_scale_inv,
    );
    *penx = _cairo_round(*penx * x_scale) * x_scale_inv;
    if *penx < x_scale_inv {
        *penx = x_scale_inv;
    }
    *peny = _cairo_round(*peny * y_scale) * y_scale_inv;
    if *peny < y_scale_inv {
        *peny = y_scale_inv;
    }
    margin = *marginl + *marginr;
    *marginl = _cairo_round(*marginl * x_scale) * x_scale_inv;
    if *marginl < x_scale_inv {
        *marginl = x_scale_inv;
    }
    *marginr = margin - *marginl;
    if *marginr < 0 as libc::c_int as libc::c_double {
        *marginr = 0 as libc::c_int as libc::c_double;
    }
    *marginr = _cairo_round(*marginr * x_scale) * x_scale_inv;
}
unsafe extern "C" fn twin_scaled_font_compute_properties(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut cr: *mut cairo_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut props: *mut twin_scaled_properties_t = 0 as *mut twin_scaled_properties_t;
    props = (if ::std::mem::size_of::<twin_scaled_properties_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<twin_scaled_properties_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut twin_scaled_properties_t;
    if props.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh0 = (*props).face_props;
    *fresh0 = cairo_font_face_get_user_data(
        cairo_scaled_font_get_font_face(scaled_font),
        &mut twin_properties_key,
    ) as *mut twin_face_properties_t;
    (*props)
        .snap = ((*scaled_font).options.hint_style as libc::c_uint
        > CAIRO_HINT_STYLE_NONE as libc::c_int as libc::c_uint) as libc::c_int;
    (*props)
        .weight = (*(*props).face_props).weight as libc::c_uint as libc::c_double
        * (4 as libc::c_int as libc::c_double / 72.0f64
            / TWIN_WEIGHT_NORMAL as libc::c_int as libc::c_double);
    let ref mut fresh1 = (*props).peny;
    *fresh1 = (*props).weight;
    (*props).penx = *fresh1;
    let ref mut fresh2 = (*props).marginr;
    *fresh2 = 4 as libc::c_int as libc::c_double / 72.0f64;
    (*props).marginl = *fresh2;
    if (*scaled_font).options.hint_style as libc::c_uint
        > CAIRO_HINT_STYLE_SLIGHT as libc::c_int as libc::c_uint
    {
        twin_hint_pen_and_margins(
            cr,
            &mut (*props).penx,
            &mut (*props).peny,
            &mut (*props).marginl,
            &mut (*props).marginr,
        );
    }
    (*props)
        .stretch = 1 as libc::c_int as libc::c_double
        + 0.1f64
            * ((*(*props).face_props).stretch as libc::c_int
                - TWIN_STRETCH_NORMAL as libc::c_int) as libc::c_double;
    status = cairo_scaled_font_set_user_data(
        scaled_font,
        &mut twin_properties_key,
        props as *mut libc::c_void,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if status as u64 != 0 {
        free(props as *mut libc::c_void);
        return status;
    } else {
        return CAIRO_STATUS_SUCCESS
    };
}
unsafe extern "C" fn twin_scaled_font_init(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut cr: *mut cairo_t,
    mut metrics: *mut cairo_font_extents_t,
) -> cairo_status_t {
    (*metrics).ascent = 54 as libc::c_int as libc::c_double / 72.0f64;
    (*metrics).descent = 1 as libc::c_int as libc::c_double - (*metrics).ascent;
    return twin_scaled_font_compute_properties(scaled_font, cr);
}
unsafe extern "C" fn twin_compute_snap(
    mut cr: *mut cairo_t,
    mut info: *mut twin_snap_info_t,
    mut b: *const libc::c_schar,
) {
    let mut s: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut snap: *const libc::c_schar = 0 as *const libc::c_schar;
    let mut x_scale: libc::c_double = 0.;
    let mut x_scale_inv: libc::c_double = 0.;
    let mut y_scale: libc::c_double = 0.;
    let mut y_scale_inv: libc::c_double = 0.;
    compute_hinting_scales(
        cr,
        &mut x_scale,
        &mut x_scale_inv,
        &mut y_scale,
        &mut y_scale_inv,
    );
    snap = &*b.offset(6 as libc::c_int as isize) as *const libc::c_schar;
    n = *b.offset(4 as libc::c_int as isize) as libc::c_int;
    (*info).n_snap_x = n;
    if n <= 4 as libc::c_int {} else {
        __assert_fail(
            b"n <= TWIN_GLYPH_MAX_SNAP_X\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-font-face-twin.c\0" as *const u8 as *const libc::c_char,
            510 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void twin_compute_snap(cairo_t *, twin_snap_info_t *, const signed char *)\0",
            ))
                .as_ptr(),
        );
    }
    s = 0 as libc::c_int;
    while s < n {
        (*info).snap_x[s as usize] = *snap.offset(s as isize);
        (*info)
            .snapped_x[s
            as usize] = _cairo_round(
            *snap.offset(s as isize) as libc::c_int as libc::c_double / 72.0f64 * x_scale,
        ) * x_scale_inv;
        s += 1;
    }
    snap = (&*b.offset(6 as libc::c_int as isize) as *const libc::c_schar)
        .offset(*b.offset(4 as libc::c_int as isize) as libc::c_int as isize);
    n = *b.offset(5 as libc::c_int as isize) as libc::c_int;
    (*info).n_snap_y = n;
    if n <= 7 as libc::c_int {} else {
        __assert_fail(
            b"n <= TWIN_GLYPH_MAX_SNAP_Y\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-font-face-twin.c\0" as *const u8 as *const libc::c_char,
            519 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void twin_compute_snap(cairo_t *, twin_snap_info_t *, const signed char *)\0",
            ))
                .as_ptr(),
        );
    }
    s = 0 as libc::c_int;
    while s < n {
        (*info).snap_y[s as usize] = *snap.offset(s as isize);
        (*info)
            .snapped_y[s
            as usize] = _cairo_round(
            *snap.offset(s as isize) as libc::c_int as libc::c_double / 72.0f64 * y_scale,
        ) * y_scale_inv;
        s += 1;
    }
}
unsafe extern "C" fn twin_snap(
    mut v: int8_t,
    mut n: libc::c_int,
    mut snap: *mut int8_t,
    mut snapped: *mut libc::c_double,
) -> libc::c_double {
    let mut s: libc::c_int = 0;
    if n == 0 {
        return v as libc::c_int as libc::c_double / 72.0f64;
    }
    if *snap.offset(0 as libc::c_int as isize) as libc::c_int == v as libc::c_int {
        return *snapped.offset(0 as libc::c_int as isize);
    }
    s = 0 as libc::c_int;
    while s < n - 1 as libc::c_int {
        if *snap.offset((s + 1 as libc::c_int) as isize) as libc::c_int
            == v as libc::c_int
        {
            return *snapped.offset((s + 1 as libc::c_int) as isize);
        }
        if *snap.offset(s as isize) as libc::c_int <= v as libc::c_int
            && v as libc::c_int
                <= *snap.offset((s + 1 as libc::c_int) as isize) as libc::c_int
        {
            let mut before: libc::c_int = *snap.offset(s as isize) as libc::c_int;
            let mut after: libc::c_int = *snap.offset((s + 1 as libc::c_int) as isize)
                as libc::c_int;
            let mut dist: libc::c_int = after - before;
            let mut snap_before: libc::c_double = *snapped.offset(s as isize);
            let mut snap_after: libc::c_double = *snapped
                .offset((s + 1 as libc::c_int) as isize);
            let mut dist_before: libc::c_double = (v as libc::c_int - before)
                as libc::c_double;
            return snap_before
                + (snap_after - snap_before) * dist_before / dist as libc::c_double;
        }
        s += 1;
    }
    return v as libc::c_int as libc::c_double / 72.0f64;
}
unsafe extern "C" fn twin_scaled_font_render_glyph(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyph: libc::c_ulong,
    mut cr: *mut cairo_t,
    mut metrics: *mut cairo_text_extents_t,
) -> cairo_status_t {
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut x3: libc::c_double = 0.;
    let mut y3: libc::c_double = 0.;
    let mut marginl: libc::c_double = 0.;
    let mut props: *mut twin_scaled_properties_t = 0 as *mut twin_scaled_properties_t;
    let mut info: twin_snap_info_t = twin_snap_info_t {
        n_snap_x: 0,
        snap_x: [0; 4],
        snapped_x: [0.; 4],
        n_snap_y: 0,
        snap_y: [0; 7],
        snapped_y: [0.; 7],
    };
    let mut b: *const int8_t = 0 as *const int8_t;
    let mut g: *const int8_t = 0 as *const int8_t;
    let mut w: int8_t = 0;
    let mut gw: libc::c_double = 0.;
    props = cairo_scaled_font_get_user_data(scaled_font, &mut twin_properties_key)
        as *mut twin_scaled_properties_t;
    cairo_save(cr);
    cairo_translate(cr, (*props).penx * 0.5f64, -(*props).peny * 0.5f64);
    if (*(*props).face_props).smallcaps != 0 && glyph >= 'a' as i32 as libc::c_ulong
        && glyph <= 'z' as i32 as libc::c_ulong
    {
        glyph = glyph.wrapping_add(('A' as i32 - 'a' as i32) as libc::c_ulong);
        cairo_scale(
            cr,
            1 as libc::c_int as libc::c_double,
            28.0f64 / 42 as libc::c_int as libc::c_double,
        );
    }
    if (*(*props).face_props).slant as libc::c_uint
        != CAIRO_FONT_SLANT_NORMAL as libc::c_int as libc::c_uint
    {
        let mut shear: cairo_matrix_t = {
            let mut init = _cairo_matrix {
                xx: 1 as libc::c_int as libc::c_double,
                yx: 0 as libc::c_int as libc::c_double,
                xy: -0.2f64,
                yy: 1 as libc::c_int as libc::c_double,
                x0: 0 as libc::c_int as libc::c_double,
                y0: 0 as libc::c_int as libc::c_double,
            };
            init
        };
        cairo_transform(cr, &mut shear);
    }
    b = _cairo_twin_outlines
        .as_ptr()
        .offset(
            _cairo_twin_charmap[(if glyph
                >= (::std::mem::size_of::<[uint16_t; 128]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                glyph
            }) as usize] as libc::c_int as isize,
        );
    g = (&*b.offset(6 as libc::c_int as isize) as *const int8_t)
        .offset(*b.offset(4 as libc::c_int as isize) as libc::c_int as isize)
        .offset(*b.offset(5 as libc::c_int as isize) as libc::c_int as isize);
    w = *b.offset(1 as libc::c_int as isize);
    gw = w as libc::c_int as libc::c_double / 72.0f64;
    marginl = (*props).marginl;
    if (*(*props).face_props).monospace != 0 {
        let mut monow: libc::c_double = 24 as libc::c_int as libc::c_double / 72.0f64;
        let mut extra: libc::c_double = (*props).penx + (*props).marginl
            + (*props).marginr;
        cairo_scale(
            cr,
            (monow + extra) / (gw + extra),
            1 as libc::c_int as libc::c_double,
        );
        gw = monow;
        let mut x: libc::c_double = 0.;
        let mut y: libc::c_double = 0.;
        let mut x_scale: libc::c_double = 0.;
        let mut x_scale_inv: libc::c_double = 0.;
        x = 1 as libc::c_int as libc::c_double;
        y = 0 as libc::c_int as libc::c_double;
        compute_hinting_scale(cr, x, y, &mut x_scale, &mut x_scale_inv);
        marginl = _cairo_round(marginl * x_scale) * x_scale_inv;
    }
    cairo_translate(cr, marginl, 0 as libc::c_int as libc::c_double);
    cairo_scale(cr, (*props).stretch, 1 as libc::c_int as libc::c_double);
    if (*props).snap != 0 {
        twin_compute_snap(cr, &mut info, b);
    } else {
        info.n_snap_y = 0 as libc::c_int;
        info.n_snap_x = info.n_snap_y;
    }
    (*metrics)
        .x_advance = gw * (*props).stretch + (*props).penx + (*props).marginl
        + (*props).marginr;
    let mut current_block_53: u64;
    loop {
        let fresh3 = g;
        g = g.offset(1);
        match *fresh3 as libc::c_int {
            77 => {
                cairo_close_path(cr);
                current_block_53 = 11055007047570490168;
            }
            109 => {
                current_block_53 = 11055007047570490168;
            }
            76 => {
                cairo_close_path(cr);
                current_block_53 = 1977384903651761240;
            }
            108 => {
                current_block_53 = 1977384903651761240;
            }
            67 => {
                cairo_close_path(cr);
                current_block_53 = 17690084694202169520;
            }
            99 => {
                current_block_53 = 17690084694202169520;
            }
            69 => {
                cairo_close_path(cr);
                current_block_53 = 12348143141253470563;
            }
            101 => {
                current_block_53 = 12348143141253470563;
            }
            88 => {
                continue;
            }
            _ => {
                break;
            }
        }
        match current_block_53 {
            11055007047570490168 => {
                let fresh4 = g;
                g = g.offset(1);
                x1 = twin_snap(
                    *fresh4,
                    info.n_snap_x,
                    (info.snap_x).as_mut_ptr(),
                    (info.snapped_x).as_mut_ptr(),
                );
                let fresh5 = g;
                g = g.offset(1);
                y1 = twin_snap(
                    *fresh5,
                    info.n_snap_y,
                    (info.snap_y).as_mut_ptr(),
                    (info.snapped_y).as_mut_ptr(),
                );
                cairo_move_to(cr, x1, y1);
            }
            17690084694202169520 => {
                let fresh8 = g;
                g = g.offset(1);
                x1 = twin_snap(
                    *fresh8,
                    info.n_snap_x,
                    (info.snap_x).as_mut_ptr(),
                    (info.snapped_x).as_mut_ptr(),
                );
                let fresh9 = g;
                g = g.offset(1);
                y1 = twin_snap(
                    *fresh9,
                    info.n_snap_y,
                    (info.snap_y).as_mut_ptr(),
                    (info.snapped_y).as_mut_ptr(),
                );
                let fresh10 = g;
                g = g.offset(1);
                x2 = twin_snap(
                    *fresh10,
                    info.n_snap_x,
                    (info.snap_x).as_mut_ptr(),
                    (info.snapped_x).as_mut_ptr(),
                );
                let fresh11 = g;
                g = g.offset(1);
                y2 = twin_snap(
                    *fresh11,
                    info.n_snap_y,
                    (info.snap_y).as_mut_ptr(),
                    (info.snapped_y).as_mut_ptr(),
                );
                let fresh12 = g;
                g = g.offset(1);
                x3 = twin_snap(
                    *fresh12,
                    info.n_snap_x,
                    (info.snap_x).as_mut_ptr(),
                    (info.snapped_x).as_mut_ptr(),
                );
                let fresh13 = g;
                g = g.offset(1);
                y3 = twin_snap(
                    *fresh13,
                    info.n_snap_y,
                    (info.snap_y).as_mut_ptr(),
                    (info.snapped_y).as_mut_ptr(),
                );
                cairo_curve_to(cr, x1, y1, x2, y2, x3, y3);
            }
            1977384903651761240 => {
                let fresh6 = g;
                g = g.offset(1);
                x1 = twin_snap(
                    *fresh6,
                    info.n_snap_x,
                    (info.snap_x).as_mut_ptr(),
                    (info.snapped_x).as_mut_ptr(),
                );
                let fresh7 = g;
                g = g.offset(1);
                y1 = twin_snap(
                    *fresh7,
                    info.n_snap_y,
                    (info.snap_y).as_mut_ptr(),
                    (info.snapped_y).as_mut_ptr(),
                );
                cairo_line_to(cr, x1, y1);
            }
            _ => {
                cairo_restore(cr);
                cairo_set_tolerance(cr, 0.01f64);
                cairo_set_line_join(cr, CAIRO_LINE_JOIN_ROUND);
                cairo_set_line_cap(cr, CAIRO_LINE_CAP_ROUND);
                cairo_set_line_width(cr, 1 as libc::c_int as libc::c_double);
                cairo_scale(cr, (*props).penx, (*props).peny);
                cairo_stroke(cr);
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn twin_scaled_font_unicode_to_glyph(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut unicode: libc::c_ulong,
    mut glyph: *mut libc::c_ulong,
) -> cairo_status_t {
    if unicode
        < (::std::mem::size_of::<[uint16_t; 128]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            as libc::c_int as libc::c_ulong
    {
        *glyph = unicode;
    } else {
        *glyph = 0 as libc::c_int as libc::c_ulong;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_font_face_twin_create_internal() -> *mut cairo_font_face_t {
    let mut twin_font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    twin_font_face = cairo_user_font_face_create();
    cairo_user_font_face_set_init_func(
        twin_font_face,
        Some(
            twin_scaled_font_init
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_t,
                    *mut cairo_t,
                    *mut cairo_font_extents_t,
                ) -> cairo_status_t,
        ),
    );
    cairo_user_font_face_set_render_glyph_func(
        twin_font_face,
        Some(
            twin_scaled_font_render_glyph
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_t,
                    libc::c_ulong,
                    *mut cairo_t,
                    *mut cairo_text_extents_t,
                ) -> cairo_status_t,
        ),
    );
    cairo_user_font_face_set_unicode_to_glyph_func(
        twin_font_face,
        Some(
            twin_scaled_font_unicode_to_glyph
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_t,
                    libc::c_ulong,
                    *mut libc::c_ulong,
                ) -> cairo_status_t,
        ),
    );
    return twin_font_face;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_face_twin_create_fallback() -> *mut cairo_font_face_t {
    let mut twin_font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    twin_font_face = _cairo_font_face_twin_create_internal();
    if (twin_font_face_create_properties(twin_font_face)).is_null() {
        cairo_font_face_destroy(twin_font_face);
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    return twin_font_face;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_face_twin_create_for_toy(
    mut toy_face: *mut cairo_toy_font_face_t,
    mut font_face: *mut *mut cairo_font_face_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut twin_font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    twin_font_face = _cairo_font_face_twin_create_internal();
    status = twin_font_face_set_properties_from_toy(twin_font_face, toy_face);
    if status as u64 != 0 {
        cairo_font_face_destroy(twin_font_face);
        return status;
    }
    *font_face = twin_font_face;
    return CAIRO_STATUS_SUCCESS;
}
