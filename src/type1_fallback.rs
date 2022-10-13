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
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_scaled_font_get_font_face(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_font_face_t;
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn cairo_matrix_init_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
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
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn _cairo_array_size(array: *const cairo_array_t) -> libc::c_uint;
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
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
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
pub type cairo_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
pub type uint16_t = __uint16_t;
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
pub type cairo_path_fixed_move_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_line_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_curve_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
    *const cairo_point_t,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_close_path_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> cairo_status_t;
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
pub type cairo_type1_font_t = _cairo_type1_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_type1_font {
    pub widths: *mut libc::c_int,
    pub scaled_font_subset: *mut cairo_scaled_font_subset_t,
    pub type1_scaled_font: *mut cairo_scaled_font_t,
    pub contents: cairo_array_t,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub data: *const libc::c_char,
    pub header_size: libc::c_ulong,
    pub data_size: libc::c_ulong,
    pub trailer_size: libc::c_ulong,
    pub bbox_position: libc::c_int,
    pub bbox_max_chars: libc::c_int,
    pub output: *mut cairo_output_stream_t,
    pub eexec_key: libc::c_ushort,
    pub hex_encode: cairo_bool_t,
    pub hex_column: libc::c_int,
}
pub type cairo_charstring_type_t = libc::c_uint;
pub const CAIRO_CHARSTRING_TYPE2: cairo_charstring_type_t = 1;
pub const CAIRO_CHARSTRING_TYPE1: cairo_charstring_type_t = 0;
pub type t1_path_info_t = _ps_path_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ps_path_info {
    pub data: *mut cairo_array_t,
    pub current_x: libc::c_int,
    pub current_y: libc::c_int,
    pub type_0: cairo_charstring_type_t,
}
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
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
pub type cairo_type2_charstrings_t = _cairo_type2_charstrings;
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
unsafe extern "C" fn cairo_type1_font_create(
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut subset_return: *mut *mut cairo_type1_font_t,
    mut hex_encode: cairo_bool_t,
) -> cairo_status_t {
    let mut font: *mut cairo_type1_font_t = 0 as *mut cairo_type1_font_t;
    let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    let mut font_matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut ctm: cairo_matrix_t = cairo_matrix_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    font = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_type1_font_t>() as libc::c_ulong,
    ) as *mut cairo_type1_font_t;
    if font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh0 = (*font).widths;
    *fresh0 = calloc(
        (*scaled_font_subset).num_glyphs as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*font).widths).is_null() {
        free(font as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh1 = (*font).scaled_font_subset;
    *fresh1 = scaled_font_subset;
    (*font).hex_encode = hex_encode;
    font_face = cairo_scaled_font_get_font_face((*scaled_font_subset).scaled_font);
    cairo_matrix_init_scale(
        &mut font_matrix,
        1000 as libc::c_int as libc::c_double,
        -(1000 as libc::c_int) as libc::c_double,
    );
    cairo_matrix_init_identity(&mut ctm);
    _cairo_font_options_init_default(&mut font_options);
    cairo_font_options_set_hint_style(&mut font_options, CAIRO_HINT_STYLE_NONE);
    cairo_font_options_set_hint_metrics(&mut font_options, CAIRO_HINT_METRICS_OFF);
    let ref mut fresh2 = (*font).type1_scaled_font;
    *fresh2 = cairo_scaled_font_create(
        font_face,
        &mut font_matrix,
        &mut ctm,
        &mut font_options,
    );
    status = (*(*font).type1_scaled_font).status;
    if status as u64 != 0 {
        free((*font).widths as *mut libc::c_void);
        free(font as *mut libc::c_void);
        return status;
    } else {
        _cairo_array_init(
            &mut (*font).contents,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as libc::c_uint,
        );
        let ref mut fresh3 = (*font).output;
        *fresh3 = 0 as *mut cairo_output_stream_t;
        *subset_return = font;
        return CAIRO_STATUS_SUCCESS;
    };
}
unsafe extern "C" fn charstring_encode_command(
    mut data: *mut cairo_array_t,
    mut command: libc::c_int,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut orig_size: libc::c_uint = 0;
    let mut buf: [libc::c_uchar; 5] = [0; 5];
    let mut p: *mut libc::c_uchar = buf.as_mut_ptr();
    if command & 0xff00 as libc::c_int != 0 {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = (command >> 8 as libc::c_int) as libc::c_uchar;
    }
    let fresh5 = p;
    p = p.offset(1);
    *fresh5 = (command & 0xff as libc::c_int) as libc::c_uchar;
    orig_size = _cairo_array_size(data);
    status = _cairo_array_append_multiple(
        data,
        buf.as_mut_ptr() as *const libc::c_void,
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_uint,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-type1-fallback.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void charstring_encode_command(cairo_array_t *, int)\0"))
                .as_ptr(),
        );
    }
    if _cairo_array_size(data) == orig_size {} else {
        __assert_fail(
            b"_cairo_array_size (data) == orig_size\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-type1-fallback.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void charstring_encode_command(cairo_array_t *, int)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn charstring_encode_integer(
    mut data: *mut cairo_array_t,
    mut i: libc::c_int,
    mut type_0: cairo_charstring_type_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut orig_size: libc::c_uint = 0;
    let mut buf: [libc::c_uchar; 10] = [0; 10];
    let mut p: *mut libc::c_uchar = buf.as_mut_ptr();
    if i >= -(107 as libc::c_int) && i <= 107 as libc::c_int {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = (i + 139 as libc::c_int) as libc::c_uchar;
    } else if i >= 108 as libc::c_int && i <= 1131 as libc::c_int {
        i -= 108 as libc::c_int;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ((i >> 8 as libc::c_int) + 247 as libc::c_int) as libc::c_uchar;
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = (i & 0xff as libc::c_int) as libc::c_uchar;
    } else if i >= -(1131 as libc::c_int) && i <= -(108 as libc::c_int) {
        i = -i - 108 as libc::c_int;
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = ((i >> 8 as libc::c_int) + 251 as libc::c_int) as libc::c_uchar;
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = (i & 0xff as libc::c_int) as libc::c_uchar;
    } else if type_0 as libc::c_uint
        == CAIRO_CHARSTRING_TYPE1 as libc::c_int as libc::c_uint
    {
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = 0xff as libc::c_int as libc::c_uchar;
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = (i >> 24 as libc::c_int) as libc::c_uchar;
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = (i >> 16 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = (i >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = (i & 0xff as libc::c_int) as libc::c_uchar;
    } else {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = 0xff as libc::c_int as libc::c_uchar;
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = (i >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = (i & 0xff as libc::c_int) as libc::c_uchar;
        let fresh19 = p;
        p = p.offset(1);
        *fresh19 = 0 as libc::c_int as libc::c_uchar;
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 = 0 as libc::c_int as libc::c_uchar;
    }
    orig_size = _cairo_array_size(data);
    status = _cairo_array_append_multiple(
        data,
        buf.as_mut_ptr() as *const libc::c_void,
        p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_uint,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-type1-fallback.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void charstring_encode_integer(cairo_array_t *, int, cairo_charstring_type_t)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_array_size(data) == orig_size {} else {
        __assert_fail(
            b"_cairo_array_size (data) == orig_size\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-type1-fallback.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void charstring_encode_integer(cairo_array_t *, int, cairo_charstring_type_t)\0",
            ))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn _charstring_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut path_info: *mut t1_path_info_t = closure as *mut t1_path_info_t;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_array_grow_by((*path_info).data, 12 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        return status;
    }
    dx = _cairo_fixed_integer_part((*point).x) - (*path_info).current_x;
    dy = _cairo_fixed_integer_part((*point).y) - (*path_info).current_y;
    charstring_encode_integer((*path_info).data, dx, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dy, (*path_info).type_0);
    (*path_info).current_x += dx;
    (*path_info).current_y += dy;
    charstring_encode_command((*path_info).data, 0x15 as libc::c_int);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _charstring_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut path_info: *mut t1_path_info_t = closure as *mut t1_path_info_t;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_array_grow_by((*path_info).data, 12 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        return status;
    }
    dx = _cairo_fixed_integer_part((*point).x) - (*path_info).current_x;
    dy = _cairo_fixed_integer_part((*point).y) - (*path_info).current_y;
    charstring_encode_integer((*path_info).data, dx, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dy, (*path_info).type_0);
    (*path_info).current_x += dx;
    (*path_info).current_y += dy;
    charstring_encode_command((*path_info).data, 0x5 as libc::c_int);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _charstring_curve_to(
    mut closure: *mut libc::c_void,
    mut point1: *const cairo_point_t,
    mut point2: *const cairo_point_t,
    mut point3: *const cairo_point_t,
) -> cairo_status_t {
    let mut path_info: *mut t1_path_info_t = closure as *mut t1_path_info_t;
    let mut dx1: libc::c_int = 0;
    let mut dy1: libc::c_int = 0;
    let mut dx2: libc::c_int = 0;
    let mut dy2: libc::c_int = 0;
    let mut dx3: libc::c_int = 0;
    let mut dy3: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_array_grow_by((*path_info).data, 32 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        return status;
    }
    dx1 = _cairo_fixed_integer_part((*point1).x) - (*path_info).current_x;
    dy1 = _cairo_fixed_integer_part((*point1).y) - (*path_info).current_y;
    dx2 = _cairo_fixed_integer_part((*point2).x) - (*path_info).current_x - dx1;
    dy2 = _cairo_fixed_integer_part((*point2).y) - (*path_info).current_y - dy1;
    dx3 = _cairo_fixed_integer_part((*point3).x) - (*path_info).current_x - dx1 - dx2;
    dy3 = _cairo_fixed_integer_part((*point3).y) - (*path_info).current_y - dy1 - dy2;
    charstring_encode_integer((*path_info).data, dx1, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dy1, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dx2, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dy2, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dx3, (*path_info).type_0);
    charstring_encode_integer((*path_info).data, dy3, (*path_info).type_0);
    (*path_info).current_x += dx1 + dx2 + dx3;
    (*path_info).current_y += dy1 + dy2 + dy3;
    charstring_encode_command((*path_info).data, 0x8 as libc::c_int);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _charstring_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut path_info: *mut t1_path_info_t = closure as *mut t1_path_info_t;
    if (*path_info).type_0 as libc::c_uint
        == CAIRO_CHARSTRING_TYPE2 as libc::c_int as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_array_grow_by((*path_info).data, 2 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        return status;
    }
    charstring_encode_command((*path_info).data, 0x9 as libc::c_int);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn charstring_encrypt(mut data: *mut cairo_array_t) {
    let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: uint16_t = 0;
    let mut p: uint16_t = 0;
    let mut r: uint16_t = 0;
    r = 4330 as libc::c_int as libc::c_ushort;
    d = _cairo_array_index(data, 0 as libc::c_int as libc::c_uint) as *mut libc::c_uchar;
    end = d.offset(_cairo_array_num_elements(data) as isize);
    while d < end {
        p = *d as uint16_t;
        c = (p as libc::c_int ^ r as libc::c_int >> 8 as libc::c_int) as uint16_t;
        r = ((c as libc::c_int + r as libc::c_int)
            * 52845 as libc::c_int as libc::c_ushort as libc::c_int
            + 22719 as libc::c_int as libc::c_ushort as libc::c_int) as uint16_t;
        let fresh21 = d;
        d = d.offset(1);
        *fresh21 = c as libc::c_uchar;
    }
}
unsafe extern "C" fn cairo_type1_font_create_charstring(
    mut font: *mut cairo_type1_font_t,
    mut subset_index: libc::c_int,
    mut glyph_index: libc::c_int,
    mut type_0: cairo_charstring_type_t,
    mut data: *mut cairo_array_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut path_info: t1_path_info_t = t1_path_info_t {
        data: 0 as *mut cairo_array_t,
        current_x: 0,
        current_y: 0,
        type_0: CAIRO_CHARSTRING_TYPE1,
    };
    let mut metrics: *mut cairo_text_extents_t = 0 as *mut cairo_text_extents_t;
    let mut emit_path: cairo_bool_t = 1 as libc::c_int;
    status = _cairo_scaled_glyph_lookup(
        (*font).type1_scaled_font,
        glyph_index as libc::c_ulong,
        (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
            | CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int) as cairo_scaled_glyph_info_t,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    );
    if glyph_index == 0 as libc::c_int
        && status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        emit_path = 0 as libc::c_int;
        status = _cairo_scaled_glyph_lookup(
            (*font).type1_scaled_font,
            glyph_index as libc::c_ulong,
            CAIRO_SCALED_GLYPH_INFO_METRICS,
            0 as *const cairo_color_t,
            &mut scaled_glyph,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    metrics = &mut (*scaled_glyph).metrics;
    if subset_index == 0 as libc::c_int {
        (*font).x_min = (*metrics).x_bearing;
        (*font).y_min = (*metrics).y_bearing;
        (*font).x_max = (*metrics).x_bearing + (*metrics).width;
        (*font).y_max = (*metrics).y_bearing + (*metrics).height;
    } else {
        if (*metrics).x_bearing < (*font).x_min {
            (*font).x_min = (*metrics).x_bearing;
        }
        if (*metrics).y_bearing < (*font).y_min {
            (*font).y_min = (*metrics).y_bearing;
        }
        if (*metrics).x_bearing + (*metrics).width > (*font).x_max {
            (*font).x_max = (*metrics).x_bearing + (*metrics).width;
        }
        if (*metrics).y_bearing + (*metrics).height > (*font).y_max {
            (*font).y_max = (*metrics).y_bearing + (*metrics).height;
        }
    }
    *((*font).widths)
        .offset(subset_index as isize) = (*metrics).x_advance as libc::c_int;
    status = _cairo_array_grow_by(data, 30 as libc::c_int as libc::c_uint)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if type_0 as libc::c_uint == CAIRO_CHARSTRING_TYPE1 as libc::c_int as libc::c_uint {
        charstring_encode_integer(
            data,
            (*scaled_glyph).metrics.x_bearing as libc::c_int,
            type_0,
        );
        charstring_encode_integer(
            data,
            (*scaled_glyph).metrics.y_bearing as libc::c_int,
            type_0,
        );
        charstring_encode_integer(
            data,
            (*scaled_glyph).metrics.x_advance as libc::c_int,
            type_0,
        );
        charstring_encode_integer(
            data,
            (*scaled_glyph).metrics.y_advance as libc::c_int,
            type_0,
        );
        charstring_encode_command(data, 0xc07 as libc::c_int);
        path_info.current_x = (*scaled_glyph).metrics.x_bearing as libc::c_int;
        path_info.current_y = (*scaled_glyph).metrics.y_bearing as libc::c_int;
    } else {
        charstring_encode_integer(
            data,
            (*scaled_glyph).metrics.x_advance as libc::c_int,
            type_0,
        );
        path_info.current_x = 0 as libc::c_int;
        path_info.current_y = 0 as libc::c_int;
    }
    path_info.data = data;
    path_info.type_0 = type_0;
    if emit_path != 0 {
        status = _cairo_path_fixed_interpret(
            (*scaled_glyph).path,
            Some(
                _charstring_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _charstring_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _charstring_curve_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                        *const cairo_point_t,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _charstring_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut path_info as *mut t1_path_info_t as *mut libc::c_void,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
    }
    status = _cairo_array_grow_by(data, 1 as libc::c_int as libc::c_uint)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    charstring_encode_command(path_info.data, 0xe as libc::c_int);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_type1_font_write_charstrings(
    mut font: *mut cairo_type1_font_t,
    mut encrypted_output: *mut cairo_output_stream_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut zeros: [libc::c_uchar; 4] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    let mut data: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    let mut i: libc::c_uint = 0;
    let mut length: libc::c_int = 0;
    _cairo_array_init(
        &mut data,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as libc::c_uint,
    );
    status = _cairo_array_grow_by(&mut data, 1024 as libc::c_int as libc::c_uint);
    if !(status as u64 != 0) {
        _cairo_output_stream_printf(
            encrypted_output,
            b"2 index /CharStrings %d dict dup begin\n\0" as *const u8
                as *const libc::c_char,
            ((*(*font).scaled_font_subset).num_glyphs)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        _cairo_scaled_font_freeze_cache((*font).type1_scaled_font);
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*(*font).scaled_font_subset).num_glyphs {
            _cairo_array_truncate(&mut data, 0 as libc::c_int as libc::c_uint);
            status = _cairo_array_append_multiple(
                &mut data,
                zeros.as_mut_ptr() as *const libc::c_void,
                4 as libc::c_int as libc::c_uint,
            );
            if status as u64 != 0 {
                break;
            }
            status = cairo_type1_font_create_charstring(
                font,
                i as libc::c_int,
                *((*(*font).scaled_font_subset).glyphs).offset(i as isize)
                    as libc::c_int,
                CAIRO_CHARSTRING_TYPE1,
                &mut data,
            ) as cairo_status_t;
            if status as u64 != 0 {
                break;
            }
            charstring_encrypt(&mut data);
            length = _cairo_array_num_elements(&mut data) as libc::c_int;
            if !((*(*font).scaled_font_subset).glyph_names).is_null() {
                _cairo_output_stream_printf(
                    encrypted_output,
                    b"/%s %d RD \0" as *const u8 as *const libc::c_char,
                    *((*(*font).scaled_font_subset).glyph_names).offset(i as isize),
                    length,
                );
            } else if i == 0 as libc::c_int as libc::c_uint {
                _cairo_output_stream_printf(
                    encrypted_output,
                    b"/.notdef %d RD \0" as *const u8 as *const libc::c_char,
                    length,
                );
            } else {
                _cairo_output_stream_printf(
                    encrypted_output,
                    b"/g%d %d RD \0" as *const u8 as *const libc::c_char,
                    i,
                    length,
                );
            }
            _cairo_output_stream_write(
                encrypted_output,
                _cairo_array_index(&mut data, 0 as libc::c_int as libc::c_uint),
                length as size_t,
            );
            _cairo_output_stream_printf(
                encrypted_output,
                b" ND\n\0" as *const u8 as *const libc::c_char,
            );
            i = i.wrapping_add(1);
        }
        _cairo_scaled_font_thaw_cache((*font).type1_scaled_font);
    }
    _cairo_array_fini(&mut data);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn cairo_type1_font_write_header(
    mut font: *mut cairo_type1_font_t,
    mut name: *const libc::c_char,
) {
    let mut i: libc::c_uint = 0;
    let spaces: [libc::c_char; 50] = *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"                                                  ");
    _cairo_output_stream_printf(
        (*font).output,
        b"%%!FontType1-1.1 %s 1.0\n11 dict begin\n/FontName /%s def\n/PaintType 0 def\n/FontType 1 def\n/FontMatrix [0.001 0 0 0.001 0 0] readonly def\n\0"
            as *const u8 as *const libc::c_char,
        name,
        name,
    );
    (*font).bbox_max_chars = 50 as libc::c_int;
    _cairo_output_stream_printf(
        (*font).output,
        b"/FontBBox {\0" as *const u8 as *const libc::c_char,
    );
    (*font)
        .bbox_position = _cairo_output_stream_get_position((*font).output)
        as libc::c_int;
    _cairo_output_stream_write(
        (*font).output,
        spaces.as_ptr() as *const libc::c_void,
        (*font).bbox_max_chars as size_t,
    );
    _cairo_output_stream_printf(
        (*font).output,
        b"} readonly def\n/Encoding 256 array\n0 1 255 {1 index exch /.notdef put} for\n\0"
            as *const u8 as *const libc::c_char,
    );
    if (*(*font).scaled_font_subset).is_latin != 0 {
        i = 1 as libc::c_int as libc::c_uint;
        while i < 256 as libc::c_int as libc::c_uint {
            let mut subset_glyph: libc::c_int = *((*(*font).scaled_font_subset)
                .latin_to_subset_glyph_index)
                .offset(i as isize) as libc::c_int;
            if subset_glyph > 0 as libc::c_int {
                if !((*(*font).scaled_font_subset).glyph_names).is_null() {
                    _cairo_output_stream_printf(
                        (*font).output,
                        b"dup %d /%s put\n\0" as *const u8 as *const libc::c_char,
                        i,
                        *((*(*font).scaled_font_subset).glyph_names)
                            .offset(subset_glyph as isize),
                    );
                } else {
                    _cairo_output_stream_printf(
                        (*font).output,
                        b"dup %d /g%d put\n\0" as *const u8 as *const libc::c_char,
                        i,
                        subset_glyph,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 1 as libc::c_int as libc::c_uint;
        while i < (*(*font).scaled_font_subset).num_glyphs {
            if !((*(*font).scaled_font_subset).glyph_names).is_null() {
                _cairo_output_stream_printf(
                    (*font).output,
                    b"dup %d /%s put\n\0" as *const u8 as *const libc::c_char,
                    i,
                    *((*(*font).scaled_font_subset).glyph_names).offset(i as isize),
                );
            } else {
                _cairo_output_stream_printf(
                    (*font).output,
                    b"dup %d /g%d put\n\0" as *const u8 as *const libc::c_char,
                    i,
                    i,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    _cairo_output_stream_printf(
        (*font).output,
        b"readonly def\ncurrentdict end\ncurrentfile eexec\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn cairo_type1_write_stream_encrypted(
    mut closure: *mut libc::c_void,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut in_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut c: uint16_t = 0;
    let mut p: uint16_t = 0;
    static mut hex_digits: [libc::c_char; 16] = unsafe {
        *::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"0123456789abcdef")
    };
    let mut digits: [libc::c_char; 3] = [0; 3];
    let mut font: *mut cairo_type1_font_t = closure as *mut cairo_type1_font_t;
    in_0 = data;
    end = data.offset(length as isize);
    while in_0 < end {
        let fresh22 = in_0;
        in_0 = in_0.offset(1);
        p = *fresh22 as uint16_t;
        c = (p as libc::c_int ^ (*font).eexec_key as libc::c_int >> 8 as libc::c_int)
            as uint16_t;
        (*font)
            .eexec_key = ((c as libc::c_int + (*font).eexec_key as libc::c_int)
            * 52845 as libc::c_int as libc::c_ushort as libc::c_int
            + 22719 as libc::c_int as libc::c_ushort as libc::c_int) as libc::c_ushort;
        if (*font).hex_encode != 0 {
            digits[0 as libc::c_int
                as usize] = hex_digits[(c as libc::c_int >> 4 as libc::c_int) as usize];
            digits[1 as libc::c_int
                as usize] = hex_digits[(c as libc::c_int & 0xf as libc::c_int) as usize];
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
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_type1_font_write_private_dict(
    mut font: *mut cairo_type1_font_t,
    mut name: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut encrypted_output: *mut cairo_output_stream_t = 0
        as *mut cairo_output_stream_t;
    (*font).eexec_key = 55665 as libc::c_int as libc::c_ushort;
    (*font).hex_column = 0 as libc::c_int;
    encrypted_output = _cairo_output_stream_create(
        Some(
            cairo_type1_write_stream_encrypted
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        font as *mut libc::c_void,
    );
    if _cairo_output_stream_get_status(encrypted_output) as u64 != 0 {
        return _cairo_output_stream_destroy(encrypted_output) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        encrypted_output,
        b"    dup /Private 9 dict dup begin\n/RD {string currentfile exch readstring pop} bind executeonly def\n/ND {noaccess def} executeonly def\n/NP {noaccess put} executeonly def\n/BlueValues [] def\n/MinFeature {16 16} def\n/lenIV 4 def\n/password 5839 def\n\0"
            as *const u8 as *const libc::c_char,
    );
    status = cairo_type1_font_write_charstrings(font, encrypted_output);
    if !(status as u64 != 0) {
        _cairo_output_stream_printf(
            encrypted_output,
            b"end\nend\nreadonly put\nnoaccess put\ndup /FontName get exch definefont pop\nmark currentfile closefile\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    status2 = _cairo_output_stream_destroy(encrypted_output);
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = status2 as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn cairo_type1_font_write_trailer(mut font: *mut cairo_type1_font_t) {
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
    _cairo_output_stream_printf(
        (*font).output,
        b"cleartomark\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn cairo_type1_write_stream(
    mut closure: *mut libc::c_void,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut font: *mut cairo_type1_font_t = closure as *mut cairo_type1_font_t;
    return _cairo_array_append_multiple(
        &mut (*font).contents,
        data as *const libc::c_void,
        length,
    );
}
unsafe extern "C" fn cairo_type1_font_write(
    mut font: *mut cairo_type1_font_t,
    mut name: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    cairo_type1_font_write_header(font, name);
    (*font)
        .header_size = _cairo_output_stream_get_position((*font).output)
        as libc::c_ulong;
    status = cairo_type1_font_write_private_dict(font, name);
    if status as u64 != 0 {
        return status;
    }
    (*font)
        .data_size = (_cairo_output_stream_get_position((*font).output)
        as libc::c_ulonglong)
        .wrapping_sub((*font).header_size as libc::c_ulonglong) as libc::c_ulong;
    cairo_type1_font_write_trailer(font);
    (*font)
        .trailer_size = (_cairo_output_stream_get_position((*font).output)
        as libc::c_ulonglong)
        .wrapping_sub((*font).header_size as libc::c_ulonglong)
        .wrapping_sub((*font).data_size as libc::c_ulonglong) as libc::c_ulong;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_type1_font_generate(
    mut font: *mut cairo_type1_font_t,
    mut name: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_array_grow_by(
        &mut (*font).contents,
        4096 as libc::c_int as libc::c_uint,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh23 = (*font).output;
    *fresh23 = _cairo_output_stream_create(
        Some(
            cairo_type1_write_stream
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        font as *mut libc::c_void,
    );
    if _cairo_output_stream_get_status((*font).output) as u64 != 0 {
        return _cairo_output_stream_destroy((*font).output) as cairo_int_status_t;
    }
    status = cairo_type1_font_write(font, name);
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh24 = (*font).data;
    *fresh24 = _cairo_array_index(
        &mut (*font).contents,
        0 as libc::c_int as libc::c_uint,
    ) as *const libc::c_char;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_type1_font_destroy(
    mut font: *mut cairo_type1_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    free((*font).widths as *mut libc::c_void);
    cairo_scaled_font_destroy((*font).type1_scaled_font);
    _cairo_array_fini(&mut (*font).contents);
    if !((*font).output).is_null() {
        status = _cairo_output_stream_destroy((*font).output);
    }
    free(font as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn _cairo_type1_fallback_init_internal(
    mut type1_subset: *mut cairo_type1_subset_t,
    mut name: *const libc::c_char,
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
    mut hex_encode: cairo_bool_t,
) -> cairo_status_t {
    let mut font: *mut cairo_type1_font_t = 0 as *mut cairo_type1_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut length: libc::c_ulong = 0;
    let mut i: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    status = cairo_type1_font_create(scaled_font_subset, &mut font, hex_encode);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_type1_font_generate(font, name) as cairo_status_t;
    if !(status as u64 != 0) {
        let ref mut fresh25 = (*type1_subset).base_font;
        *fresh25 = strdup(name);
        if ((*type1_subset).base_font).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            let ref mut fresh26 = (*type1_subset).widths;
            *fresh26 = calloc(
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
            ) as *mut libc::c_double;
            if ((*type1_subset).widths).is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*font).scaled_font_subset).num_glyphs {
                    *((*type1_subset).widths)
                        .offset(
                            i as isize,
                        ) = *((*font).widths).offset(i as isize) as libc::c_double
                        / 1000 as libc::c_int as libc::c_double;
                    i = i.wrapping_add(1);
                }
                (*type1_subset)
                    .x_min = (*font).x_min / 1000 as libc::c_int as libc::c_double;
                (*type1_subset)
                    .y_min = (*font).y_min / 1000 as libc::c_int as libc::c_double;
                (*type1_subset)
                    .x_max = (*font).x_max / 1000 as libc::c_int as libc::c_double;
                (*type1_subset)
                    .y_max = (*font).y_max / 1000 as libc::c_int as libc::c_double;
                (*type1_subset)
                    .ascent = (*font).y_max / 1000 as libc::c_int as libc::c_double;
                (*type1_subset)
                    .descent = (*font).y_min / 1000 as libc::c_int as libc::c_double;
                length = ((*font).header_size)
                    .wrapping_add((*font).data_size)
                    .wrapping_add((*font).trailer_size);
                let ref mut fresh27 = (*type1_subset).data;
                *fresh27 = (if length != 0 as libc::c_int as libc::c_ulong {
                    malloc(length)
                } else {
                    0 as *mut libc::c_void
                }) as *mut libc::c_char;
                if ((*type1_subset).data).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    free((*type1_subset).widths as *mut libc::c_void);
                } else {
                    memcpy(
                        (*type1_subset).data as *mut libc::c_void,
                        _cairo_array_index(
                            &mut (*font).contents,
                            0 as libc::c_int as libc::c_uint,
                        ),
                        length,
                    );
                    len = snprintf(
                        ((*type1_subset).data).offset((*font).bbox_position as isize),
                        (*font).bbox_max_chars as libc::c_ulong,
                        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
                        (*font).x_min as libc::c_int,
                        (*font).y_min as libc::c_int,
                        (*font).x_max as libc::c_int,
                        (*font).y_max as libc::c_int,
                    ) as libc::c_uint;
                    *((*type1_subset).data)
                        .offset(
                            ((*font).bbox_position as libc::c_uint).wrapping_add(len)
                                as isize,
                        ) = ' ' as i32 as libc::c_char;
                    (*type1_subset).header_length = (*font).header_size;
                    (*type1_subset).data_length = (*font).data_size;
                    (*type1_subset).trailer_length = (*font).trailer_size;
                    return cairo_type1_font_destroy(font);
                }
            }
            free((*type1_subset).base_font as *mut libc::c_void);
        }
    }
    cairo_type1_font_destroy(font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type1_fallback_init_binary(
    mut type1_subset: *mut cairo_type1_subset_t,
    mut name: *const libc::c_char,
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    return _cairo_type1_fallback_init_internal(
        type1_subset,
        name,
        scaled_font_subset,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type1_fallback_init_hex(
    mut type1_subset: *mut cairo_type1_subset_t,
    mut name: *const libc::c_char,
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    return _cairo_type1_fallback_init_internal(
        type1_subset,
        name,
        scaled_font_subset,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type1_fallback_fini(
    mut subset: *mut cairo_type1_subset_t,
) {
    free((*subset).base_font as *mut libc::c_void);
    free((*subset).widths as *mut libc::c_void);
    free((*subset).data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type2_charstrings_init(
    mut type2_subset: *mut cairo_type2_charstrings_t,
    mut scaled_font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut font: *mut cairo_type1_font_t = 0 as *mut cairo_type1_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut charstring: cairo_array_t = cairo_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *mut libc::c_char,
    };
    status = cairo_type1_font_create(scaled_font_subset, &mut font, 0 as libc::c_int);
    if status as u64 != 0 {
        return status;
    }
    _cairo_array_init(
        &mut (*type2_subset).charstrings,
        ::std::mem::size_of::<cairo_array_t>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh28 = (*type2_subset).widths;
    *fresh28 = calloc(
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*(*font).scaled_font_subset).num_glyphs as libc::c_ulong,
    ) as *mut libc::c_int;
    if ((*type2_subset).widths).is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _cairo_scaled_font_freeze_cache((*font).type1_scaled_font);
        i = 0 as libc::c_int as libc::c_uint;
        loop {
            if !(i < (*(*font).scaled_font_subset).num_glyphs) {
                current_block = 13242334135786603907;
                break;
            }
            _cairo_array_init(
                &mut charstring,
                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as libc::c_uint,
            );
            status = _cairo_array_grow_by(
                &mut charstring,
                32 as libc::c_int as libc::c_uint,
            );
            if status as u64 != 0 {
                current_block = 910549767253072953;
                break;
            }
            status = cairo_type1_font_create_charstring(
                font,
                i as libc::c_int,
                *((*(*font).scaled_font_subset).glyphs).offset(i as isize)
                    as libc::c_int,
                CAIRO_CHARSTRING_TYPE2,
                &mut charstring,
            ) as cairo_status_t;
            if status as u64 != 0 {
                current_block = 910549767253072953;
                break;
            }
            status = _cairo_array_append(
                &mut (*type2_subset).charstrings,
                &mut charstring as *mut cairo_array_t as *const libc::c_void,
            );
            if status as u64 != 0 {
                current_block = 910549767253072953;
                break;
            }
            i = i.wrapping_add(1);
        }
        match current_block {
            910549767253072953 => {
                _cairo_scaled_font_thaw_cache((*font).type1_scaled_font);
                _cairo_array_fini(&mut charstring);
                _cairo_type2_charstrings_fini(type2_subset);
            }
            _ => {
                _cairo_scaled_font_thaw_cache((*font).type1_scaled_font);
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*font).scaled_font_subset).num_glyphs {
                    *((*type2_subset).widths)
                        .offset(i as isize) = *((*font).widths).offset(i as isize);
                    i = i.wrapping_add(1);
                }
                (*type2_subset).x_min = (*font).x_min as libc::c_int as libc::c_long;
                (*type2_subset).y_min = (*font).y_min as libc::c_int as libc::c_long;
                (*type2_subset).x_max = (*font).x_max as libc::c_int as libc::c_long;
                (*type2_subset).y_max = (*font).y_max as libc::c_int as libc::c_long;
                (*type2_subset).ascent = (*font).y_max as libc::c_int as libc::c_long;
                (*type2_subset).descent = (*font).y_min as libc::c_int as libc::c_long;
                return cairo_type1_font_destroy(font);
            }
        }
    }
    cairo_type1_font_destroy(font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type2_charstrings_fini(
    mut type2_subset: *mut cairo_type2_charstrings_t,
) {
    let mut i: libc::c_uint = 0;
    let mut num_charstrings: libc::c_uint = 0;
    let mut charstring: *mut cairo_array_t = 0 as *mut cairo_array_t;
    num_charstrings = _cairo_array_num_elements(&mut (*type2_subset).charstrings);
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_charstrings {
        charstring = _cairo_array_index(&mut (*type2_subset).charstrings, i)
            as *mut cairo_array_t;
        _cairo_array_fini(charstring);
        i = i.wrapping_add(1);
    }
    _cairo_array_fini(&mut (*type2_subset).charstrings);
    free((*type2_subset).widths as *mut libc::c_void);
}
