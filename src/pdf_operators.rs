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
    pub type _cairo_scaled_font_subsets;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_matrix_init_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn cairo_matrix_transform_distance(
        matrix: *const cairo_matrix_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn cairo_matrix_transform_point(
        matrix: *const cairo_matrix_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_is_rectangle(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_utf8_to_utf16(
        str: *const libc::c_char,
        len: libc::c_int,
        result: *mut *mut uint16_t,
        items_written: *mut libc::c_int,
    ) -> cairo_status_t;
    static _cairo_output_stream_nil: cairo_output_stream_t;
    fn _cairo_output_stream_init(
        stream: *mut cairo_output_stream_t,
        write_func: cairo_output_stream_write_func_t,
        flush_func: cairo_output_stream_flush_func_t,
        close_func: cairo_output_stream_close_func_t,
    );
    fn _cairo_output_stream_create_in_error(
        status: cairo_status_t,
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
    fn _cairo_output_stream_print_matrix(
        stream: *mut cairo_output_stream_t,
        matrix: *const cairo_matrix_t,
    );
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_subsets_map_glyph(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        scaled_font: *mut cairo_scaled_font_t,
        scaled_font_glyph_index: libc::c_ulong,
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        subset_glyph_ret: *mut cairo_scaled_font_subsets_glyph_t,
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
pub type cairo_scaled_font_subsets_t = _cairo_scaled_font_subsets;
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
pub type cairo_pdf_operators_use_font_subset_t = Option::<
    unsafe extern "C" fn(
        libc::c_uint,
        libc::c_uint,
        *mut libc::c_void,
    ) -> cairo_int_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_glyph {
    pub glyph_index: libc::c_uint,
    pub x_position: libc::c_double,
    pub x_advance: libc::c_double,
}
pub type cairo_pdf_glyph_t = _cairo_pdf_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_operators {
    pub stream: *mut cairo_output_stream_t,
    pub cairo_to_pdf: cairo_matrix_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub use_font_subset: cairo_pdf_operators_use_font_subset_t,
    pub use_font_subset_closure: *mut libc::c_void,
    pub ps_output: cairo_bool_t,
    pub use_actual_text: cairo_bool_t,
    pub in_text_object: cairo_bool_t,
    pub is_new_text_object: cairo_bool_t,
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub text_matrix: cairo_matrix_t,
    pub cairo_to_pdftext: cairo_matrix_t,
    pub font_matrix_inverse: cairo_matrix_t,
    pub cur_x: libc::c_double,
    pub cur_y: libc::c_double,
    pub hex_width: libc::c_int,
    pub is_latin: cairo_bool_t,
    pub num_glyphs: libc::c_int,
    pub glyph_buf_x_pos: libc::c_double,
    pub glyphs: [cairo_pdf_glyph_t; 200],
    pub has_line_style: cairo_bool_t,
    pub line_width: libc::c_double,
    pub line_cap: cairo_line_cap_t,
    pub line_join: cairo_line_join_t,
    pub miter_limit: libc::c_double,
    pub has_dashes: cairo_bool_t,
}
pub type cairo_pdf_operators_t = _cairo_pdf_operators;
pub type word_wrap_stream_t = _word_wrap_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _word_wrap_stream {
    pub base: cairo_output_stream_t,
    pub output: *mut cairo_output_stream_t,
    pub max_column: libc::c_int,
    pub ps_output: cairo_bool_t,
    pub column: libc::c_int,
    pub state: cairo_word_wrap_state_t,
    pub in_escape: cairo_bool_t,
    pub escape_digits: libc::c_int,
}
pub type cairo_word_wrap_state_t = _cairo_word_wrap_state;
pub type _cairo_word_wrap_state = libc::c_uint;
pub const WRAP_STATE_HEXSTRING: _cairo_word_wrap_state = 3;
pub const WRAP_STATE_STRING: _cairo_word_wrap_state = 2;
pub const WRAP_STATE_WORD: _cairo_word_wrap_state = 1;
pub const WRAP_STATE_DELIMITER: _cairo_word_wrap_state = 0;
pub type pdf_path_info_t = _pdf_path_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pdf_path_info {
    pub output: *mut cairo_output_stream_t,
    pub path_transform: *mut cairo_matrix_t,
    pub line_cap: cairo_line_cap_t,
    pub last_move_to_point: cairo_point_t,
    pub has_sub_path: cairo_bool_t,
}
pub type cairo_scaled_font_subsets_glyph_t = _cairo_scaled_font_subsets_glyph;
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
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
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
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_abc(
    mut a: size_t,
    mut b: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let mut d: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(b);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    let (fresh4, fresh5) = c.overflowing_mul(size);
    *(&mut d as *mut size_t) = fresh4;
    if fresh5 {
        return 0 as *mut libc::c_void;
    }
    return if d != 0 as libc::c_int as libc::c_ulong {
        malloc(d)
    } else {
        0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_init(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut stream: *mut cairo_output_stream_t,
    mut cairo_to_pdf: *mut cairo_matrix_t,
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut ps: cairo_bool_t,
) {
    let ref mut fresh6 = (*pdf_operators).stream;
    *fresh6 = stream;
    (*pdf_operators).cairo_to_pdf = *cairo_to_pdf;
    let ref mut fresh7 = (*pdf_operators).font_subsets;
    *fresh7 = font_subsets;
    (*pdf_operators).ps_output = ps;
    let ref mut fresh8 = (*pdf_operators).use_font_subset;
    *fresh8 = None;
    let ref mut fresh9 = (*pdf_operators).use_font_subset_closure;
    *fresh9 = 0 as *mut libc::c_void;
    (*pdf_operators).in_text_object = 0 as libc::c_int;
    (*pdf_operators).num_glyphs = 0 as libc::c_int;
    (*pdf_operators).has_line_style = 0 as libc::c_int;
    (*pdf_operators).use_actual_text = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_fini(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_status_t {
    return _cairo_pdf_operators_flush(pdf_operators);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_set_font_subsets_callback(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut use_font_subset: cairo_pdf_operators_use_font_subset_t,
    mut closure: *mut libc::c_void,
) {
    let ref mut fresh10 = (*pdf_operators).use_font_subset;
    *fresh10 = use_font_subset;
    let ref mut fresh11 = (*pdf_operators).use_font_subset_closure;
    *fresh11 = closure;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_set_stream(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut stream: *mut cairo_output_stream_t,
) {
    let ref mut fresh12 = (*pdf_operators).stream;
    *fresh12 = stream;
    (*pdf_operators).has_line_style = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_set_cairo_to_pdf_matrix(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut cairo_to_pdf: *mut cairo_matrix_t,
) {
    (*pdf_operators).cairo_to_pdf = *cairo_to_pdf;
    (*pdf_operators).has_line_style = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_enable_actual_text(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut enable: cairo_bool_t,
) {
    (*pdf_operators).use_actual_text = enable;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_flush(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pdf_operators).in_text_object != 0 {
        status = _cairo_pdf_operators_end_text(pdf_operators);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_reset(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) {
    (*pdf_operators).has_line_style = 0 as libc::c_int;
}
unsafe extern "C" fn _word_wrap_stream_count_word_up_to(
    mut stream: *mut word_wrap_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_uchar = data;
    let mut count: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh13 = length;
        length = length - 1;
        if !(fresh13 != 0) {
            break;
        }
        if _cairo_isspace(*s as libc::c_int) != 0 || *s as libc::c_int == '<' as i32
            || *s as libc::c_int == '(' as i32
        {
            (*stream).state = WRAP_STATE_DELIMITER;
            break;
        } else {
            count += 1;
            let ref mut fresh14 = (*stream).column;
            *fresh14 += 1;
            s = s.offset(1);
        }
    }
    if count != 0 {
        _cairo_output_stream_write(
            (*stream).output,
            data as *const libc::c_void,
            count as size_t,
        );
    }
    return count;
}
unsafe extern "C" fn _word_wrap_stream_count_hexstring_up_to(
    mut stream: *mut word_wrap_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_uchar = data;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut newline: cairo_bool_t = 0 as libc::c_int;
    loop {
        let fresh15 = length;
        length = length - 1;
        if !(fresh15 != 0) {
            break;
        }
        count += 1;
        let ref mut fresh16 = (*stream).column;
        *fresh16 += 1;
        if *s as libc::c_int == '>' as i32 {
            (*stream).state = WRAP_STATE_DELIMITER;
            break;
        } else if (*stream).column > (*stream).max_column {
            newline = 1 as libc::c_int;
            break;
        } else {
            s = s.offset(1);
        }
    }
    if count != 0 {
        _cairo_output_stream_write(
            (*stream).output,
            data as *const libc::c_void,
            count as size_t,
        );
    }
    if newline != 0 {
        _cairo_output_stream_printf(
            (*stream).output,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
        (*stream).column = 0 as libc::c_int;
    }
    return count;
}
unsafe extern "C" fn _word_wrap_stream_count_string_up_to(
    mut stream: *mut word_wrap_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_uchar = data;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut newline: cairo_bool_t = 0 as libc::c_int;
    loop {
        let fresh17 = length;
        length = length - 1;
        if !(fresh17 != 0) {
            break;
        }
        count += 1;
        let ref mut fresh18 = (*stream).column;
        *fresh18 += 1;
        if (*stream).in_escape == 0 {
            if *s as libc::c_int == ')' as i32 {
                (*stream).state = WRAP_STATE_DELIMITER;
                break;
            } else if *s as libc::c_int == '\\' as i32 {
                (*stream).in_escape = 1 as libc::c_int;
                (*stream).escape_digits = 0 as libc::c_int;
            } else if (*stream).ps_output != 0 && (*stream).column > (*stream).max_column
            {
                newline = 1 as libc::c_int;
                break;
            }
        } else if _cairo_isdigit(*s as libc::c_int) == 0
            || {
                let ref mut fresh19 = (*stream).escape_digits;
                *fresh19 += 1;
                *fresh19 == 3 as libc::c_int
            }
        {
            (*stream).in_escape = 0 as libc::c_int;
        }
        s = s.offset(1);
    }
    if count != 0 {
        _cairo_output_stream_write(
            (*stream).output,
            data as *const libc::c_void,
            count as size_t,
        );
    }
    if newline != 0 {
        _cairo_output_stream_printf(
            (*stream).output,
            b"\\\n\0" as *const u8 as *const libc::c_char,
        );
        (*stream).column = 0 as libc::c_int;
    }
    return count;
}
unsafe extern "C" fn _word_wrap_stream_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut word_wrap_stream_t = base as *mut word_wrap_stream_t;
    let mut count: libc::c_int = 0;
    while length != 0 {
        match (*stream).state as libc::c_uint {
            1 => {
                count = _word_wrap_stream_count_word_up_to(
                    stream,
                    data,
                    length as libc::c_int,
                );
            }
            3 => {
                count = _word_wrap_stream_count_hexstring_up_to(
                    stream,
                    data,
                    length as libc::c_int,
                );
            }
            2 => {
                count = _word_wrap_stream_count_string_up_to(
                    stream,
                    data,
                    length as libc::c_int,
                );
            }
            0 => {
                count = 1 as libc::c_int;
                let ref mut fresh20 = (*stream).column;
                *fresh20 += 1;
                if *data as libc::c_int == '\n' as i32
                    || (*stream).column >= (*stream).max_column
                {
                    _cairo_output_stream_printf(
                        (*stream).output,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                    (*stream).column = 0 as libc::c_int;
                }
                if *data as libc::c_int == '<' as i32 {
                    (*stream).state = WRAP_STATE_HEXSTRING;
                } else if *data as libc::c_int == '(' as i32 {
                    (*stream).state = WRAP_STATE_STRING;
                } else if _cairo_isspace(*data as libc::c_int) == 0 {
                    (*stream).state = WRAP_STATE_WORD;
                }
                if *data as libc::c_int != '\n' as i32 {
                    _cairo_output_stream_write(
                        (*stream).output,
                        data as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                }
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-pdf-operators.c\0" as *const u8
                            as *const libc::c_char,
                        335 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"cairo_status_t _word_wrap_stream_write(cairo_output_stream_t *, const unsigned char *, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                count = length as libc::c_int;
            }
        }
        data = data.offset(count as isize);
        length = length.wrapping_sub(count as libc::c_uint);
    }
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _word_wrap_stream_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut word_wrap_stream_t = base as *mut word_wrap_stream_t;
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _word_wrap_stream_create(
    mut output: *mut cairo_output_stream_t,
    mut ps: cairo_bool_t,
    mut max_column: libc::c_int,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut word_wrap_stream_t = 0 as *mut word_wrap_stream_t;
    if (*output).status as u64 != 0 {
        return _cairo_output_stream_create_in_error((*output).status);
    }
    stream = (if ::std::mem::size_of::<word_wrap_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<word_wrap_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut word_wrap_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            _word_wrap_stream_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            _word_wrap_stream_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh21 = (*stream).output;
    *fresh21 = output;
    (*stream).max_column = max_column;
    (*stream).ps_output = ps;
    (*stream).column = 0 as libc::c_int;
    (*stream).state = WRAP_STATE_DELIMITER;
    (*stream).in_escape = 0 as libc::c_int;
    (*stream).escape_digits = 0 as libc::c_int;
    return &mut (*stream).base;
}
unsafe extern "C" fn _cairo_pdf_path_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut info: *mut pdf_path_info_t = closure as *mut pdf_path_info_t;
    let mut x: libc::c_double = _cairo_fixed_to_double((*point).x);
    let mut y: libc::c_double = _cairo_fixed_to_double((*point).y);
    (*info).last_move_to_point = *point;
    (*info).has_sub_path = 0 as libc::c_int;
    cairo_matrix_transform_point((*info).path_transform, &mut x, &mut y);
    _cairo_output_stream_printf(
        (*info).output,
        b"%g %g m \0" as *const u8 as *const libc::c_char,
        x,
        y,
    );
    return _cairo_output_stream_get_status((*info).output);
}
unsafe extern "C" fn _cairo_pdf_path_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut info: *mut pdf_path_info_t = closure as *mut pdf_path_info_t;
    let mut x: libc::c_double = _cairo_fixed_to_double((*point).x);
    let mut y: libc::c_double = _cairo_fixed_to_double((*point).y);
    if (*info).line_cap as libc::c_uint
        != CAIRO_LINE_CAP_ROUND as libc::c_int as libc::c_uint
        && (*info).has_sub_path == 0 && (*point).x == (*info).last_move_to_point.x
        && (*point).y == (*info).last_move_to_point.y
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*info).has_sub_path = 1 as libc::c_int;
    cairo_matrix_transform_point((*info).path_transform, &mut x, &mut y);
    _cairo_output_stream_printf(
        (*info).output,
        b"%g %g l \0" as *const u8 as *const libc::c_char,
        x,
        y,
    );
    return _cairo_output_stream_get_status((*info).output);
}
unsafe extern "C" fn _cairo_pdf_path_curve_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_status_t {
    let mut info: *mut pdf_path_info_t = closure as *mut pdf_path_info_t;
    let mut bx: libc::c_double = _cairo_fixed_to_double((*b).x);
    let mut by: libc::c_double = _cairo_fixed_to_double((*b).y);
    let mut cx: libc::c_double = _cairo_fixed_to_double((*c).x);
    let mut cy: libc::c_double = _cairo_fixed_to_double((*c).y);
    let mut dx: libc::c_double = _cairo_fixed_to_double((*d).x);
    let mut dy: libc::c_double = _cairo_fixed_to_double((*d).y);
    (*info).has_sub_path = 1 as libc::c_int;
    cairo_matrix_transform_point((*info).path_transform, &mut bx, &mut by);
    cairo_matrix_transform_point((*info).path_transform, &mut cx, &mut cy);
    cairo_matrix_transform_point((*info).path_transform, &mut dx, &mut dy);
    _cairo_output_stream_printf(
        (*info).output,
        b"%g %g %g %g %g %g c \0" as *const u8 as *const libc::c_char,
        bx,
        by,
        cx,
        cy,
        dx,
        dy,
    );
    return _cairo_output_stream_get_status((*info).output);
}
unsafe extern "C" fn _cairo_pdf_path_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut info: *mut pdf_path_info_t = closure as *mut pdf_path_info_t;
    if (*info).line_cap as libc::c_uint
        != CAIRO_LINE_CAP_ROUND as libc::c_int as libc::c_uint
        && (*info).has_sub_path == 0
    {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_output_stream_printf(
        (*info).output,
        b"h\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*info).output);
}
unsafe extern "C" fn _cairo_pdf_path_rectangle(
    mut info: *mut pdf_path_info_t,
    mut box_0: *mut cairo_box_t,
) -> cairo_status_t {
    let mut x1: libc::c_double = _cairo_fixed_to_double((*box_0).p1.x);
    let mut y1: libc::c_double = _cairo_fixed_to_double((*box_0).p1.y);
    let mut x2: libc::c_double = _cairo_fixed_to_double((*box_0).p2.x);
    let mut y2: libc::c_double = _cairo_fixed_to_double((*box_0).p2.y);
    cairo_matrix_transform_point((*info).path_transform, &mut x1, &mut y1);
    cairo_matrix_transform_point((*info).path_transform, &mut x2, &mut y2);
    _cairo_output_stream_printf(
        (*info).output,
        b"%g %g %g %g re \0" as *const u8 as *const libc::c_char,
        x1,
        y1,
        x2 - x1,
        y2 - y1,
    );
    return _cairo_output_stream_get_status((*info).output);
}
unsafe extern "C" fn _cairo_pdf_operators_emit_path(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut path: *const cairo_path_fixed_t,
    mut path_transform: *mut cairo_matrix_t,
    mut line_cap: cairo_line_cap_t,
) -> cairo_status_t {
    let mut word_wrap: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut info: pdf_path_info_t = pdf_path_info_t {
        output: 0 as *mut cairo_output_stream_t,
        path_transform: 0 as *mut cairo_matrix_t,
        line_cap: CAIRO_LINE_CAP_BUTT,
        last_move_to_point: cairo_point_t { x: 0, y: 0 },
        has_sub_path: 0,
    };
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    word_wrap = _word_wrap_stream_create(
        (*pdf_operators).stream,
        (*pdf_operators).ps_output,
        72 as libc::c_int,
    );
    status = _cairo_output_stream_get_status(word_wrap);
    if status as u64 != 0 {
        return _cairo_output_stream_destroy(word_wrap);
    }
    info.output = word_wrap;
    info.path_transform = path_transform;
    info.line_cap = line_cap;
    if _cairo_path_fixed_is_rectangle(path, &mut box_0) != 0
        && ((*path_transform).xx == 0 as libc::c_int as libc::c_double
            && (*path_transform).yy == 0 as libc::c_int as libc::c_double
            || (*path_transform).xy == 0 as libc::c_int as libc::c_double
                && (*path_transform).yx == 0 as libc::c_int as libc::c_double)
    {
        status = _cairo_pdf_path_rectangle(&mut info, &mut box_0);
    } else {
        status = _cairo_path_fixed_interpret(
            path,
            Some(
                _cairo_pdf_path_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cairo_pdf_path_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cairo_pdf_path_curve_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                        *const cairo_point_t,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cairo_pdf_path_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut info as *mut pdf_path_info_t as *mut libc::c_void,
        );
    }
    status2 = _cairo_output_stream_destroy(word_wrap);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_clip(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
) -> cairo_int_status_t {
    let mut pdf_operator: *const libc::c_char = 0 as *const libc::c_char;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pdf_operators).in_text_object != 0 {
        status = _cairo_pdf_operators_end_text(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    if (*path).has_current_point() == 0 {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"0 0 m \0" as *const u8 as *const libc::c_char,
        );
    } else {
        status = _cairo_pdf_operators_emit_path(
            pdf_operators,
            path,
            &mut (*pdf_operators).cairo_to_pdf,
            CAIRO_LINE_CAP_ROUND,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    let mut current_block_16: u64;
    match fill_rule as libc::c_uint {
        0 => {
            current_block_16 = 15861556936773649323;
        }
        1 => {
            pdf_operator = b"W*\0" as *const u8 as *const libc::c_char;
            current_block_16 = 7175849428784450219;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-operators.c\0" as *const u8
                        as *const libc::c_char,
                    566 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 117],
                        &[libc::c_char; 117],
                    >(
                        b"cairo_int_status_t _cairo_pdf_operators_clip(cairo_pdf_operators_t *, const cairo_path_fixed_t *, cairo_fill_rule_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_16 = 15861556936773649323;
        }
    }
    match current_block_16 {
        15861556936773649323 => {
            pdf_operator = b"W\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"%s n\n\0" as *const u8 as *const libc::c_char,
        pdf_operator,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_line_cap(mut cap: cairo_line_cap_t) -> libc::c_int {
    match cap as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 => return 1 as libc::c_int,
        2 => return 2 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-operators.c\0" as *const u8
                        as *const libc::c_char,
                    593 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"int _cairo_pdf_line_cap(cairo_line_cap_t)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn _cairo_pdf_line_join(mut join: cairo_line_join_t) -> libc::c_int {
    match join as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 => return 1 as libc::c_int,
        2 => return 2 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-operators.c\0" as *const u8
                        as *const libc::c_char,
                    609 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"int _cairo_pdf_line_join(cairo_line_join_t)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_emit_stroke_style(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut style: *const cairo_stroke_style_t,
    mut scale: libc::c_double,
) -> cairo_int_status_t {
    let mut dash: *mut libc::c_double = (*style).dash;
    let mut num_dashes: libc::c_int = (*style).num_dashes as libc::c_int;
    let mut dash_offset: libc::c_double = (*style).dash_offset;
    let mut line_width: libc::c_double = (*style).line_width * scale;
    if num_dashes != 0
        && (*style).line_cap as libc::c_uint
            == CAIRO_LINE_CAP_BUTT as libc::c_int as libc::c_uint
    {
        let mut i: libc::c_int = 0;
        if num_dashes % 2 as libc::c_int != 0 {
            dash = _cairo_malloc_abc(
                num_dashes as size_t,
                2 as libc::c_int as size_t,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            ) as *mut libc::c_double;
            if dash.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            memcpy(
                dash as *mut libc::c_void,
                (*style).dash as *const libc::c_void,
                (num_dashes as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            memcpy(
                dash.offset(num_dashes as isize) as *mut libc::c_void,
                (*style).dash as *const libc::c_void,
                (num_dashes as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            num_dashes *= 2 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < num_dashes {
            if *dash.offset(i as isize) == 0.0f64 {
                if dash == (*style).dash {
                    dash = _cairo_malloc_ab(
                        num_dashes as size_t,
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ) as *mut libc::c_double;
                    if dash.is_null() {
                        return _cairo_error(CAIRO_STATUS_NO_MEMORY)
                            as cairo_int_status_t;
                    }
                    memcpy(
                        dash as *mut libc::c_void,
                        (*style).dash as *const libc::c_void,
                        (num_dashes as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            ),
                    );
                }
                if i == 0 as libc::c_int {
                    let mut last_two: [libc::c_double; 2] = [0.; 2];
                    if num_dashes == 2 as libc::c_int {
                        free(dash as *mut libc::c_void);
                        return CAIRO_INT_STATUS_NOTHING_TO_DO;
                    }
                    memcpy(
                        last_two.as_mut_ptr() as *mut libc::c_void,
                        dash
                            .offset(num_dashes as isize)
                            .offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong,
                    );
                    memmove(
                        dash.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                        dash as *const libc::c_void,
                        ((num_dashes - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            ),
                    );
                    memcpy(
                        dash as *mut libc::c_void,
                        last_two.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong,
                    );
                    dash_offset
                        += *dash.offset(0 as libc::c_int as isize)
                            + *dash.offset(1 as libc::c_int as isize);
                    i = 2 as libc::c_int;
                }
                *dash.offset((i - 1 as libc::c_int) as isize)
                    += *dash.offset((i + 1 as libc::c_int) as isize);
                num_dashes -= 2 as libc::c_int;
                memmove(
                    dash.offset(i as isize) as *mut libc::c_void,
                    dash.offset(i as isize).offset(2 as libc::c_int as isize)
                        as *const libc::c_void,
                    ((num_dashes - i) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                );
                if i == 2 as libc::c_int {
                    i = -(2 as libc::c_int);
                }
            }
            i += 2 as libc::c_int;
        }
    }
    if (*pdf_operators).has_line_style == 0 || (*pdf_operators).line_width != line_width
    {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"%f w\n\0" as *const u8 as *const libc::c_char,
            line_width,
        );
        (*pdf_operators).line_width = line_width;
    }
    if (*pdf_operators).has_line_style == 0
        || (*pdf_operators).line_cap as libc::c_uint != (*style).line_cap as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"%d J\n\0" as *const u8 as *const libc::c_char,
            _cairo_pdf_line_cap((*style).line_cap),
        );
        (*pdf_operators).line_cap = (*style).line_cap;
    }
    if (*pdf_operators).has_line_style == 0
        || (*pdf_operators).line_join as libc::c_uint
            != (*style).line_join as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"%d j\n\0" as *const u8 as *const libc::c_char,
            _cairo_pdf_line_join((*style).line_join),
        );
        (*pdf_operators).line_join = (*style).line_join;
    }
    if num_dashes != 0 {
        let mut d: libc::c_int = 0;
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"[\0" as *const u8 as *const libc::c_char,
        );
        d = 0 as libc::c_int;
        while d < num_dashes {
            _cairo_output_stream_printf(
                (*pdf_operators).stream,
                b" %f\0" as *const u8 as *const libc::c_char,
                *dash.offset(d as isize) * scale,
            );
            d += 1;
        }
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"] %f d\n\0" as *const u8 as *const libc::c_char,
            dash_offset * scale,
        );
        (*pdf_operators).has_dashes = 1 as libc::c_int;
    } else if (*pdf_operators).has_line_style == 0 || (*pdf_operators).has_dashes != 0 {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"[] 0.0 d\n\0" as *const u8 as *const libc::c_char,
        );
        (*pdf_operators).has_dashes = 0 as libc::c_int;
    }
    if dash != (*style).dash {
        free(dash as *mut libc::c_void);
    }
    if (*pdf_operators).has_line_style == 0
        || (*pdf_operators).miter_limit != (*style).miter_limit
    {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"%f M \0" as *const u8 as *const libc::c_char,
            if (*style).miter_limit < 1.0f64 { 1.0f64 } else { (*style).miter_limit },
        );
        (*pdf_operators).miter_limit = (*style).miter_limit;
    }
    (*pdf_operators).has_line_style = 1 as libc::c_int;
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_matrix_factor_out_scale(
    mut m: *mut cairo_matrix_t,
    mut scale: *mut libc::c_double,
) {
    let mut s: libc::c_double = 0.;
    s = fabs((*m).xx);
    if fabs((*m).xy) > s {
        s = fabs((*m).xy);
    }
    if fabs((*m).yx) > s {
        s = fabs((*m).yx);
    }
    if fabs((*m).yy) > s {
        s = fabs((*m).yy);
    }
    *scale = s;
    s = 1.0f64 / s;
    cairo_matrix_scale(m, s, s);
}
unsafe extern "C" fn _cairo_pdf_operators_emit_stroke(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut pdf_operator: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut m: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut path_transform: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut has_ctm: cairo_bool_t = 1 as libc::c_int;
    let mut scale: libc::c_double = 1.0f64;
    if (*pdf_operators).in_text_object != 0 {
        status = _cairo_pdf_operators_end_text(pdf_operators) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
    }
    if fabs((*ctm).xx) == 1.0f64 && fabs((*ctm).yy) == 1.0f64
        && fabs((*ctm).xy) == 0.0f64 && fabs((*ctm).yx) == 0.0f64
    {
        has_ctm = 0 as libc::c_int;
    }
    if has_ctm != 0 {
        m = *ctm;
        m.x0 = 0.0f64;
        m.y0 = 0.0f64;
        _cairo_matrix_factor_out_scale(&mut m, &mut scale);
        path_transform = m;
        status = cairo_matrix_invert(&mut path_transform) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        cairo_matrix_multiply(&mut m, &mut m, &mut (*pdf_operators).cairo_to_pdf);
    }
    status = _cairo_pdf_operators_emit_stroke_style(pdf_operators, style, scale);
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if status as u64 != 0 {
        return status;
    }
    if has_ctm != 0 {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b"q \0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*pdf_operators).stream, &mut m);
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b" cm\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        path_transform = (*pdf_operators).cairo_to_pdf;
    }
    status = _cairo_pdf_operators_emit_path(
        pdf_operators,
        path,
        &mut path_transform,
        (*style).line_cap,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"%s\0" as *const u8 as *const libc::c_char,
        pdf_operator,
    );
    if has_ctm != 0 {
        _cairo_output_stream_printf(
            (*pdf_operators).stream,
            b" Q\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_stroke(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
) -> cairo_int_status_t {
    return _cairo_pdf_operators_emit_stroke(
        pdf_operators,
        path,
        style,
        ctm,
        ctm_inverse,
        b"S\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_fill(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
) -> cairo_int_status_t {
    let mut pdf_operator: *const libc::c_char = 0 as *const libc::c_char;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pdf_operators).in_text_object != 0 {
        status = _cairo_pdf_operators_end_text(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    status = _cairo_pdf_operators_emit_path(
        pdf_operators,
        path,
        &mut (*pdf_operators).cairo_to_pdf,
        CAIRO_LINE_CAP_ROUND,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    let mut current_block_12: u64;
    match fill_rule as libc::c_uint {
        0 => {
            current_block_12 = 3758502364500251198;
        }
        1 => {
            pdf_operator = b"f*\0" as *const u8 as *const libc::c_char;
            current_block_12 = 17407779659766490442;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-operators.c\0" as *const u8
                        as *const libc::c_char,
                    899 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 117],
                        &[libc::c_char; 117],
                    >(
                        b"cairo_int_status_t _cairo_pdf_operators_fill(cairo_pdf_operators_t *, const cairo_path_fixed_t *, cairo_fill_rule_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_12 = 3758502364500251198;
        }
    }
    match current_block_12 {
        3758502364500251198 => {
            pdf_operator = b"f\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        pdf_operator,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_fill_stroke(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
) -> cairo_int_status_t {
    let mut operator: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block_4: u64;
    match fill_rule as libc::c_uint {
        0 => {
            current_block_4 = 15748462753081132835;
        }
        1 => {
            operator = b"B*\0" as *const u8 as *const libc::c_char;
            current_block_4 = 14523784380283086299;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-operators.c\0" as *const u8
                        as *const libc::c_char,
                    927 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 202],
                        &[libc::c_char; 202],
                    >(
                        b"cairo_int_status_t _cairo_pdf_operators_fill_stroke(cairo_pdf_operators_t *, const cairo_path_fixed_t *, cairo_fill_rule_t, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_4 = 15748462753081132835;
        }
    }
    match current_block_4 {
        15748462753081132835 => {
            operator = b"B\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    return _cairo_pdf_operators_emit_stroke(
        pdf_operators,
        path,
        style,
        ctm,
        ctm_inverse,
        operator,
    );
}
unsafe extern "C" fn _cairo_pdf_operators_emit_glyph_index(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut stream: *mut cairo_output_stream_t,
    mut glyph: libc::c_uint,
) {
    if (*pdf_operators).is_latin != 0 {
        if glyph == '(' as i32 as libc::c_uint || glyph == ')' as i32 as libc::c_uint
            || glyph == '\\' as i32 as libc::c_uint
        {
            _cairo_output_stream_printf(
                stream,
                b"\\%c\0" as *const u8 as *const libc::c_char,
                glyph,
            );
        } else if glyph >= 0x20 as libc::c_int as libc::c_uint
            && glyph <= 0x7e as libc::c_int as libc::c_uint
        {
            _cairo_output_stream_printf(
                stream,
                b"%c\0" as *const u8 as *const libc::c_char,
                glyph,
            );
        } else {
            _cairo_output_stream_printf(
                stream,
                b"\\%03o\0" as *const u8 as *const libc::c_char,
                glyph,
            );
        }
    } else {
        _cairo_output_stream_printf(
            stream,
            b"%0*x\0" as *const u8 as *const libc::c_char,
            (*pdf_operators).hex_width,
            glyph,
        );
    };
}
unsafe extern "C" fn _cairo_pdf_operators_emit_glyph_string(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut i: libc::c_int = 0;
    _cairo_output_stream_printf(
        stream,
        b"%s\0" as *const u8 as *const libc::c_char,
        if (*pdf_operators).is_latin != 0 {
            b"(\0" as *const u8 as *const libc::c_char
        } else {
            b"<\0" as *const u8 as *const libc::c_char
        },
    );
    i = 0 as libc::c_int;
    while i < (*pdf_operators).num_glyphs {
        _cairo_pdf_operators_emit_glyph_index(
            pdf_operators,
            stream,
            (*pdf_operators).glyphs[i as usize].glyph_index,
        );
        (*pdf_operators).cur_x += (*pdf_operators).glyphs[i as usize].x_advance;
        i += 1;
    }
    _cairo_output_stream_printf(
        stream,
        b"%sTj\n\0" as *const u8 as *const libc::c_char,
        if (*pdf_operators).is_latin != 0 {
            b")\0" as *const u8 as *const libc::c_char
        } else {
            b">\0" as *const u8 as *const libc::c_char
        },
    );
    return _cairo_output_stream_get_status(stream);
}
unsafe extern "C" fn _cairo_pdf_operators_emit_glyph_string_with_positioning(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut stream: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut i: libc::c_int = 0;
    _cairo_output_stream_printf(
        stream,
        b"[%s\0" as *const u8 as *const libc::c_char,
        if (*pdf_operators).is_latin != 0 {
            b"(\0" as *const u8 as *const libc::c_char
        } else {
            b"<\0" as *const u8 as *const libc::c_char
        },
    );
    i = 0 as libc::c_int;
    while i < (*pdf_operators).num_glyphs {
        if (*pdf_operators).glyphs[i as usize].x_position != (*pdf_operators).cur_x {
            let mut delta: libc::c_double = (*pdf_operators)
                .glyphs[i as usize]
                .x_position - (*pdf_operators).cur_x;
            let mut rounded_delta: libc::c_int = 0;
            delta = -1000.0f64 * delta;
            rounded_delta = _cairo_lround(delta);
            if abs(rounded_delta) < 3 as libc::c_int {
                rounded_delta = 0 as libc::c_int;
            }
            if rounded_delta != 0 as libc::c_int {
                if (*pdf_operators).is_latin != 0 {
                    _cairo_output_stream_printf(
                        stream,
                        b")%d(\0" as *const u8 as *const libc::c_char,
                        rounded_delta,
                    );
                } else {
                    _cairo_output_stream_printf(
                        stream,
                        b">%d<\0" as *const u8 as *const libc::c_char,
                        rounded_delta,
                    );
                }
            }
            delta = rounded_delta as libc::c_double / -1000.0f64;
            (*pdf_operators).cur_x += delta;
        }
        _cairo_pdf_operators_emit_glyph_index(
            pdf_operators,
            stream,
            (*pdf_operators).glyphs[i as usize].glyph_index,
        );
        (*pdf_operators).cur_x += (*pdf_operators).glyphs[i as usize].x_advance;
        i += 1;
    }
    _cairo_output_stream_printf(
        stream,
        b"%s]TJ\n\0" as *const u8 as *const libc::c_char,
        if (*pdf_operators).is_latin != 0 {
            b")\0" as *const u8 as *const libc::c_char
        } else {
            b">\0" as *const u8 as *const libc::c_char
        },
    );
    return _cairo_output_stream_get_status(stream);
}
unsafe extern "C" fn _cairo_pdf_operators_flush_glyphs(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_status_t {
    let mut word_wrap_stream: *mut cairo_output_stream_t = 0
        as *mut cairo_output_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    if (*pdf_operators).num_glyphs == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    word_wrap_stream = _word_wrap_stream_create(
        (*pdf_operators).stream,
        (*pdf_operators).ps_output,
        72 as libc::c_int,
    );
    status = _cairo_output_stream_get_status(word_wrap_stream);
    if status as u64 != 0 {
        return _cairo_output_stream_destroy(word_wrap_stream);
    }
    x = (*pdf_operators).cur_x;
    i = 0 as libc::c_int;
    while i < (*pdf_operators).num_glyphs {
        if fabs((*pdf_operators).glyphs[i as usize].x_position - x) > 0.001f64 {
            break;
        }
        x += (*pdf_operators).glyphs[i as usize].x_advance;
        i += 1;
    }
    if i == (*pdf_operators).num_glyphs {
        status = _cairo_pdf_operators_emit_glyph_string(pdf_operators, word_wrap_stream);
    } else {
        status = _cairo_pdf_operators_emit_glyph_string_with_positioning(
            pdf_operators,
            word_wrap_stream,
        );
    }
    (*pdf_operators).num_glyphs = 0 as libc::c_int;
    (*pdf_operators).glyph_buf_x_pos = (*pdf_operators).cur_x;
    status2 = _cairo_output_stream_destroy(word_wrap_stream);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_operators_add_glyph(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut glyph: *mut cairo_scaled_font_subsets_glyph_t,
    mut x_position: libc::c_double,
) -> cairo_status_t {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    x = (*glyph).x_advance;
    y = (*glyph).y_advance;
    if (*glyph).is_scaled != 0 {
        cairo_matrix_transform_distance(
            &mut (*pdf_operators).font_matrix_inverse,
            &mut x,
            &mut y,
        );
    }
    (*pdf_operators)
        .glyphs[(*pdf_operators).num_glyphs as usize]
        .x_position = x_position;
    (*pdf_operators)
        .glyphs[(*pdf_operators).num_glyphs as usize]
        .glyph_index = (*glyph).subset_glyph_index;
    (*pdf_operators).glyphs[(*pdf_operators).num_glyphs as usize].x_advance = x;
    (*pdf_operators).glyph_buf_x_pos += x;
    let ref mut fresh22 = (*pdf_operators).num_glyphs;
    *fresh22 += 1;
    if (*pdf_operators).num_glyphs == 200 as libc::c_int {
        return _cairo_pdf_operators_flush_glyphs(pdf_operators);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_pdf_operators_set_text_matrix(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut matrix: *mut cairo_matrix_t,
) -> cairo_status_t {
    let mut inverse: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    inverse = *matrix;
    status = cairo_matrix_invert(&mut inverse);
    if status as u64 != 0 {
        return status;
    }
    (*pdf_operators).text_matrix = *matrix;
    (*pdf_operators).cur_x = 0 as libc::c_int as libc::c_double;
    (*pdf_operators).cur_y = 0 as libc::c_int as libc::c_double;
    (*pdf_operators).glyph_buf_x_pos = 0 as libc::c_int as libc::c_double;
    _cairo_output_stream_print_matrix(
        (*pdf_operators).stream,
        &mut (*pdf_operators).text_matrix,
    );
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b" Tm\n\0" as *const u8 as *const libc::c_char,
    );
    (*pdf_operators).cairo_to_pdftext = *matrix;
    status = cairo_matrix_invert(&mut (*pdf_operators).cairo_to_pdftext);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-operators.c\0" as *const u8 as *const libc::c_char,
            1134 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"cairo_status_t _cairo_pdf_operators_set_text_matrix(cairo_pdf_operators_t *, cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(
        &mut (*pdf_operators).cairo_to_pdftext,
        &mut (*pdf_operators).cairo_to_pdf,
        &mut (*pdf_operators).cairo_to_pdftext,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream);
}
unsafe extern "C" fn _cairo_pdf_operators_set_text_position(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> cairo_status_t {
    let mut translate: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut inverse: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    inverse = (*pdf_operators).text_matrix;
    status = cairo_matrix_invert(&mut inverse);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-operators.c\0" as *const u8 as *const libc::c_char,
            1164 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"cairo_status_t _cairo_pdf_operators_set_text_position(cairo_pdf_operators_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    (*pdf_operators).text_matrix.x0 = x;
    (*pdf_operators).text_matrix.y0 = y;
    cairo_matrix_multiply(
        &mut translate,
        &mut (*pdf_operators).text_matrix,
        &mut inverse,
    );
    if fabs(translate.x0) < 1e-6f64 {
        translate.x0 = 0.0f64;
    }
    if fabs(translate.y0) < 1e-6f64 {
        translate.y0 = 0.0f64;
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"%f %f Td\n\0" as *const u8 as *const libc::c_char,
        translate.x0,
        translate.y0,
    );
    (*pdf_operators).cur_x = 0 as libc::c_int as libc::c_double;
    (*pdf_operators).cur_y = 0 as libc::c_int as libc::c_double;
    (*pdf_operators).glyph_buf_x_pos = 0 as libc::c_int as libc::c_double;
    (*pdf_operators).cairo_to_pdftext = (*pdf_operators).text_matrix;
    status = cairo_matrix_invert(&mut (*pdf_operators).cairo_to_pdftext);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-operators.c\0" as *const u8 as *const libc::c_char,
            1182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"cairo_status_t _cairo_pdf_operators_set_text_position(cairo_pdf_operators_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(
        &mut (*pdf_operators).cairo_to_pdftext,
        &mut (*pdf_operators).cairo_to_pdf,
        &mut (*pdf_operators).cairo_to_pdftext,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream);
}
unsafe extern "C" fn _cairo_pdf_operators_set_font_subset(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut subset_glyph: *mut cairo_scaled_font_subsets_glyph_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"/f-%d-%d 1 Tf\n\0" as *const u8 as *const libc::c_char,
        (*subset_glyph).font_id,
        (*subset_glyph).subset_id,
    );
    if ((*pdf_operators).use_font_subset).is_some() {
        status = ((*pdf_operators).use_font_subset)
            .expect(
                "non-null function pointer",
            )(
            (*subset_glyph).font_id,
            (*subset_glyph).subset_id,
            (*pdf_operators).use_font_subset_closure,
        ) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
    }
    (*pdf_operators).font_id = (*subset_glyph).font_id;
    (*pdf_operators).subset_id = (*subset_glyph).subset_id;
    (*pdf_operators).is_latin = (*subset_glyph).is_latin;
    if (*subset_glyph).is_composite != 0 {
        (*pdf_operators).hex_width = 4 as libc::c_int;
    } else {
        (*pdf_operators).hex_width = 2 as libc::c_int;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_pdf_operators_begin_text(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_status_t {
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"BT\n\0" as *const u8 as *const libc::c_char,
    );
    (*pdf_operators).in_text_object = 1 as libc::c_int;
    (*pdf_operators).num_glyphs = 0 as libc::c_int;
    (*pdf_operators).glyph_buf_x_pos = 0 as libc::c_int as libc::c_double;
    return _cairo_output_stream_get_status((*pdf_operators).stream);
}
unsafe extern "C" fn _cairo_pdf_operators_end_text(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_pdf_operators_flush_glyphs(pdf_operators);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"ET\n\0" as *const u8 as *const libc::c_char,
    );
    (*pdf_operators).in_text_object = 0 as libc::c_int;
    return _cairo_output_stream_get_status((*pdf_operators).stream);
}
unsafe extern "C" fn _cairo_matrix_scale_equal(
    mut a: *mut cairo_matrix_t,
    mut b: *mut cairo_matrix_t,
) -> cairo_bool_t {
    return ((*a).xx == (*b).xx && (*a).xy == (*b).xy && (*a).yx == (*b).yx
        && (*a).yy == (*b).yy) as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_operators_begin_actualtext(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
) -> cairo_status_t {
    let mut utf16: *mut uint16_t = 0 as *mut uint16_t;
    let mut utf16_len: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"/Span << /ActualText <feff\0" as *const u8 as *const libc::c_char,
    );
    if utf8_len != 0 {
        status = _cairo_utf8_to_utf16(utf8, utf8_len, &mut utf16, &mut utf16_len);
        if status as u64 != 0 {
            return status;
        }
        i = 0 as libc::c_int;
        while i < utf16_len {
            _cairo_output_stream_printf(
                (*pdf_operators).stream,
                b"%04x\0" as *const u8 as *const libc::c_char,
                *utf16.offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
        free(utf16 as *mut libc::c_void);
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"> >> BDC\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream);
}
unsafe extern "C" fn _cairo_pdf_operators_end_actualtext(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_status_t {
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"EMC\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream);
}
unsafe extern "C" fn _cairo_pdf_operators_emit_glyph(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut glyph: *mut cairo_glyph_t,
    mut subset_glyph: *mut cairo_scaled_font_subsets_glyph_t,
) -> cairo_status_t {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pdf_operators).is_new_text_object != 0
        || (*pdf_operators).font_id != (*subset_glyph).font_id
        || (*pdf_operators).subset_id != (*subset_glyph).subset_id
    {
        status = _cairo_pdf_operators_flush_glyphs(pdf_operators);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_operators_set_font_subset(pdf_operators, subset_glyph);
        if status as u64 != 0 {
            return status;
        }
        (*pdf_operators).is_new_text_object = 0 as libc::c_int;
    }
    x = (*glyph).x;
    y = (*glyph).y;
    cairo_matrix_transform_point(&mut (*pdf_operators).cairo_to_pdftext, &mut x, &mut y);
    if fabs(x - (*pdf_operators).glyph_buf_x_pos) > 10 as libc::c_int as libc::c_double
        || fabs(y - (*pdf_operators).cur_y) > 0.001f64
    {
        status = _cairo_pdf_operators_flush_glyphs(pdf_operators);
        if status as u64 != 0 {
            return status;
        }
        x = (*glyph).x;
        y = (*glyph).y;
        cairo_matrix_transform_point(&mut (*pdf_operators).cairo_to_pdf, &mut x, &mut y);
        status = _cairo_pdf_operators_set_text_position(pdf_operators, x, y);
        if status as u64 != 0 {
            return status;
        }
        x = 0.0f64;
        y = 0.0f64;
    }
    status = _cairo_pdf_operators_add_glyph(pdf_operators, subset_glyph, x);
    return status;
}
unsafe extern "C" fn _cairo_pdf_operators_emit_cluster(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_int_status_t {
    let mut subset_glyph: cairo_scaled_font_subsets_glyph_t = cairo_scaled_font_subsets_glyph_t {
        font_id: 0,
        subset_id: 0,
        subset_glyph_index: 0,
        is_scaled: 0,
        is_composite: 0,
        is_latin: 0,
        x_advance: 0.,
        y_advance: 0.,
        utf8_is_mapped: 0,
        unicode: 0,
    };
    let mut cur_glyph: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if num_glyphs == 1 as libc::c_int && utf8_len != 0 as libc::c_int {
        status = _cairo_scaled_font_subsets_map_glyph(
            (*pdf_operators).font_subsets,
            scaled_font,
            (*glyphs).index,
            utf8,
            utf8_len,
            &mut subset_glyph,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        if subset_glyph.utf8_is_mapped != 0 || utf8_len < 0 as libc::c_int {
            status = _cairo_pdf_operators_emit_glyph(
                pdf_operators,
                glyphs,
                &mut subset_glyph,
            );
            if status as u64 != 0 {
                return status as cairo_int_status_t;
            }
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    }
    if (*pdf_operators).use_actual_text != 0 {
        status = _cairo_pdf_operators_flush_glyphs(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        status = _cairo_pdf_operators_begin_actualtext(pdf_operators, utf8, utf8_len);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    if cluster_flags as libc::c_uint
        & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint != 0
    {
        cur_glyph = glyphs
            .offset(num_glyphs as isize)
            .offset(-(1 as libc::c_int as isize));
    } else {
        cur_glyph = glyphs;
    }
    i = 0 as libc::c_int;
    while i < num_glyphs {
        status = _cairo_scaled_font_subsets_map_glyph(
            (*pdf_operators).font_subsets,
            scaled_font,
            (*cur_glyph).index,
            0 as *const libc::c_char,
            -(1 as libc::c_int),
            &mut subset_glyph,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        status = _cairo_pdf_operators_emit_glyph(
            pdf_operators,
            cur_glyph,
            &mut subset_glyph,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        if cluster_flags as libc::c_uint
            & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint != 0
        {
            cur_glyph = cur_glyph.offset(-1);
        } else {
            cur_glyph = cur_glyph.offset(1);
        }
        i += 1;
    }
    if (*pdf_operators).use_actual_text != 0 {
        status = _cairo_pdf_operators_flush_glyphs(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        status = _cairo_pdf_operators_end_actualtext(pdf_operators);
    }
    return status as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_show_text_glyphs(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut clusters: *const cairo_text_cluster_t,
    mut num_clusters: libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut text_matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut invert_y_axis: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut cur_text: *const libc::c_char = 0 as *const libc::c_char;
    let mut cur_glyph: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    (*pdf_operators).font_matrix_inverse = (*scaled_font).font_matrix;
    status = cairo_matrix_invert(&mut (*pdf_operators).font_matrix_inverse);
    if status as libc::c_uint
        == CAIRO_STATUS_INVALID_MATRIX as libc::c_int as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-operators.c\0" as *const u8 as *const libc::c_char,
            1480 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 209],
                &[libc::c_char; 209],
            >(
                b"cairo_int_status_t _cairo_pdf_operators_show_text_glyphs(cairo_pdf_operators_t *, const char *, int, cairo_glyph_t *, int, const cairo_text_cluster_t *, int, cairo_text_cluster_flags_t, cairo_scaled_font_t *)\0",
            ))
                .as_ptr(),
        );
    }
    (*pdf_operators).is_new_text_object = 0 as libc::c_int;
    if (*pdf_operators).in_text_object == 0 as libc::c_int {
        status = _cairo_pdf_operators_begin_text(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        (*pdf_operators).is_new_text_object = 1 as libc::c_int;
    }
    cairo_matrix_init_scale(
        &mut invert_y_axis,
        1 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
    );
    text_matrix = (*scaled_font).scale;
    cairo_matrix_multiply(&mut text_matrix, &mut invert_y_axis, &mut text_matrix);
    if (*pdf_operators).is_new_text_object != 0
        || _cairo_matrix_scale_equal(&mut (*pdf_operators).text_matrix, &mut text_matrix)
            == 0
    {
        status = _cairo_pdf_operators_flush_glyphs(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        x = (*glyphs.offset(0 as libc::c_int as isize)).x;
        y = (*glyphs.offset(0 as libc::c_int as isize)).y;
        cairo_matrix_transform_point(&mut (*pdf_operators).cairo_to_pdf, &mut x, &mut y);
        text_matrix.x0 = x;
        text_matrix.y0 = y;
        status = _cairo_pdf_operators_set_text_matrix(pdf_operators, &mut text_matrix);
        if status as libc::c_uint
            == CAIRO_STATUS_INVALID_MATRIX as libc::c_int as libc::c_uint
        {
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    if num_clusters > 0 as libc::c_int {
        cur_text = utf8;
        if cluster_flags as libc::c_uint
            & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint != 0
        {
            cur_glyph = glyphs.offset(num_glyphs as isize);
        } else {
            cur_glyph = glyphs;
        }
        i = 0 as libc::c_int;
        while i < num_clusters {
            if cluster_flags as libc::c_uint
                & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint != 0
            {
                cur_glyph = cur_glyph
                    .offset(-((*clusters.offset(i as isize)).num_glyphs as isize));
            }
            status = _cairo_pdf_operators_emit_cluster(
                pdf_operators,
                cur_text,
                (*clusters.offset(i as isize)).num_bytes,
                cur_glyph,
                (*clusters.offset(i as isize)).num_glyphs,
                cluster_flags,
                scaled_font,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return status as cairo_int_status_t;
            }
            cur_text = cur_text
                .offset((*clusters.offset(i as isize)).num_bytes as isize);
            if cluster_flags as libc::c_uint
                & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint == 0
            {
                cur_glyph = cur_glyph
                    .offset((*clusters.offset(i as isize)).num_glyphs as isize);
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < num_glyphs {
            status = _cairo_pdf_operators_emit_cluster(
                pdf_operators,
                0 as *const libc::c_char,
                -(1 as libc::c_int),
                &mut *glyphs.offset(i as isize),
                1 as libc::c_int,
                0 as cairo_text_cluster_flags_t,
                scaled_font,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return status as cairo_int_status_t;
            }
            i += 1;
        }
    }
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_tag_begin(
    mut pdf_operators: *mut cairo_pdf_operators_t,
    mut tag_name: *const libc::c_char,
    mut mcid: libc::c_int,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pdf_operators).in_text_object != 0 {
        status = _cairo_pdf_operators_end_text(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"/%s << /MCID %d >> BDC\n\0" as *const u8 as *const libc::c_char,
        tag_name,
        mcid,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_operators_tag_end(
    mut pdf_operators: *mut cairo_pdf_operators_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pdf_operators).in_text_object != 0 {
        status = _cairo_pdf_operators_end_text(pdf_operators);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    _cairo_output_stream_printf(
        (*pdf_operators).stream,
        b"EMC\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*pdf_operators).stream)
        as cairo_int_status_t;
}
