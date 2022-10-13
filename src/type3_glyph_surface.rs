use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _cairo_scaled_font_subsets;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn _cairo_matrix_transform_bounding_box_fixed(
        matrix: *const cairo_matrix_t,
        bbox: *mut cairo_box_t,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_image_surface_coerce_to_format(
        surface: *mut cairo_image_surface_t,
        format: cairo_format_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_surface_release_source_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
    );
    fn _cairo_surface_acquire_source_image(
        surface: *mut cairo_surface_t,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_scaled_font_create(
        font_face: *mut cairo_font_face_t,
        font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t,
        options: *const cairo_font_options_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_matrix_init(
        matrix: *mut cairo_matrix_t,
        xx: libc::c_double,
        yx: libc::c_double,
        xy: libc::c_double,
        yy: libc::c_double,
        x0: libc::c_double,
        y0: libc::c_double,
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_clipper_set_clip(
        clipper: *mut cairo_surface_clipper_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_init(
        clipper: *mut cairo_surface_clipper_t,
        intersect: cairo_surface_clipper_intersect_clip_path_func_t,
    );
    fn _cairo_pdf_operators_flush(
        pdf_operators: *mut cairo_pdf_operators_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_operators_init(
        pdf_operators: *mut cairo_pdf_operators_t,
        stream: *mut cairo_output_stream_t,
        cairo_to_pdf: *mut cairo_matrix_t,
        font_subsets: *mut cairo_scaled_font_subsets_t,
        ps: cairo_bool_t,
    );
    fn _cairo_pdf_operators_fini(
        pdf_operators: *mut cairo_pdf_operators_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_operators_set_font_subsets_callback(
        pdf_operators: *mut cairo_pdf_operators_t,
        use_font_subset: cairo_pdf_operators_use_font_subset_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_pdf_operators_set_stream(
        pdf_operators: *mut cairo_pdf_operators_t,
        stream: *mut cairo_output_stream_t,
    );
    fn _cairo_pdf_operators_clip(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_stroke(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_fill(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_show_text_glyphs(
        pdf_operators: *mut cairo_pdf_operators_t,
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clusters: *const cairo_text_cluster_t,
        num_clusters: libc::c_int,
        cluster_flags: cairo_text_cluster_flags_t,
        scaled_font: *mut cairo_scaled_font_t,
    ) -> cairo_int_status_t;
    fn _cairo_output_stream_destroy(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_printf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _cairo_memory_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_copy(
        base: *mut cairo_output_stream_t,
        dest: *mut cairo_output_stream_t,
    );
    fn _cairo_null_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_recording_surface_replay(
        surface: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
}
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
pub type ptrdiff_t = libc::c_long;
pub type cairo_bool_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_backend_t,
}
pub type cairo_backend_t = _cairo_backend;
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
pub type cairo_damage_t = _cairo_damage;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_image_surface {
    pub base: cairo_surface_t,
    pub pixman_image: *mut pixman_image_t,
    pub compositor: *const cairo_compositor_t,
    pub parent: *mut cairo_surface_t,
    pub pixman_format: pixman_format_code_t,
    pub format: cairo_format_t,
    pub data: *mut libc::c_uchar,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub stride: ptrdiff_t,
    pub depth: libc::c_int,
    #[bitfield(name = "owns_data", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "transparency", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "color", ty = "libc::c_uint", bits = "3..=4")]
    pub owns_data_transparency_color: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
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
pub type cairo_compositor_t = cairo_compositor;
pub type pixman_image_t = pixman_image;
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
pub type _cairo_internal_surface_type = libc::c_uint;
pub const CAIRO_INTERNAL_SURFACE_TYPE_QUARTZ_SNAPSHOT: _cairo_internal_surface_type = 4105;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TYPE3_GLYPH: _cairo_internal_surface_type = 4104;
pub const CAIRO_INTERNAL_SURFACE_TYPE_NULL: _cairo_internal_surface_type = 4103;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_WRAPPING: _cairo_internal_surface_type = 4102;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_PAGINATED: _cairo_internal_surface_type = 4101;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_FALLBACK: _cairo_internal_surface_type = 4100;
pub const CAIRO_INTERNAL_SURFACE_TYPE_OBSERVER: _cairo_internal_surface_type = 4099;
pub const CAIRO_INTERNAL_SURFACE_TYPE_ANALYSIS: _cairo_internal_surface_type = 4098;
pub const CAIRO_INTERNAL_SURFACE_TYPE_PAGINATED: _cairo_internal_surface_type = 4097;
pub const CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT: _cairo_internal_surface_type = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_clipper {
    pub clip: *mut cairo_clip_t,
    pub intersect_clip_path: cairo_surface_clipper_intersect_clip_path_func_t,
}
pub type cairo_surface_clipper_intersect_clip_path_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_surface_clipper_t,
        *mut cairo_path_fixed_t,
        cairo_fill_rule_t,
        libc::c_double,
        cairo_antialias_t,
    ) -> cairo_status_t,
>;
pub type cairo_surface_clipper_t = _cairo_surface_clipper;
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
pub type cairo_type3_glyph_surface_emit_image_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_image_surface_t,
        *mut cairo_output_stream_t,
    ) -> cairo_int_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_type3_glyph_surface {
    pub base: cairo_surface_t,
    pub scaled_font: *mut cairo_scaled_font_t,
    pub stream: *mut cairo_output_stream_t,
    pub pdf_operators: cairo_pdf_operators_t,
    pub cairo_to_pdf: cairo_matrix_t,
    pub emit_image: cairo_type3_glyph_surface_emit_image_t,
    pub clipper: cairo_surface_clipper_t,
}
pub type cairo_type3_glyph_surface_t = cairo_type3_glyph_surface;
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
unsafe extern "C" fn _cairo_type3_glyph_surface_clipper_intersect_clip_path(
    mut clipper: *mut cairo_surface_clipper_t,
    mut path: *mut cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = ({
        let mut mptr__: *const cairo_surface_clipper_t = clipper;
        (mptr__ as *mut libc::c_char).offset(-(5536 as libc::c_ulong as isize))
            as *mut cairo_type3_glyph_surface_t
    });
    if path.is_null() {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"Q q\n\0" as *const u8 as *const libc::c_char,
        );
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_pdf_operators_clip(&mut (*surface).pdf_operators, path, fill_rule)
        as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type3_glyph_surface_create(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut stream: *mut cairo_output_stream_t,
    mut emit_image: cairo_type3_glyph_surface_emit_image_t,
    mut font_subsets: *mut cairo_scaled_font_subsets_t,
    mut ps: cairo_bool_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = 0
        as *mut cairo_type3_glyph_surface_t;
    if !stream.is_null() && (*stream).status as libc::c_uint != 0 {
        return _cairo_surface_create_in_error((*stream).status);
    }
    surface = (if ::std::mem::size_of::<cairo_type3_glyph_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_type3_glyph_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_type3_glyph_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_type3_glyph_surface_backend,
        0 as *mut cairo_device_t,
        CAIRO_CONTENT_COLOR_ALPHA,
        1 as libc::c_int,
    );
    let ref mut fresh0 = (*surface).scaled_font;
    *fresh0 = scaled_font;
    let ref mut fresh1 = (*surface).stream;
    *fresh1 = stream;
    let ref mut fresh2 = (*surface).emit_image;
    *fresh2 = emit_image;
    (*surface).cairo_to_pdf = (*scaled_font).scale_inverse;
    _cairo_pdf_operators_init(
        &mut (*surface).pdf_operators,
        (*surface).stream,
        &mut (*surface).cairo_to_pdf,
        font_subsets,
        ps,
    );
    _cairo_surface_clipper_init(
        &mut (*surface).clipper,
        Some(
            _cairo_type3_glyph_surface_clipper_intersect_clip_path
                as unsafe extern "C" fn(
                    *mut cairo_surface_clipper_t,
                    *mut cairo_path_fixed_t,
                    cairo_fill_rule_t,
                    libc::c_double,
                    cairo_antialias_t,
                ) -> cairo_status_t,
        ),
    );
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_type3_glyph_surface_emit_image(
    mut surface: *mut cairo_type3_glyph_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut image_matrix: *mut cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    image = _cairo_image_surface_coerce_to_format(image, CAIRO_FORMAT_A1);
    status = (*image).base.status;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"q %f %f %f %f %f %f cm\n\0" as *const u8 as *const libc::c_char,
        (*image_matrix).xx,
        (*image_matrix).xy,
        (*image_matrix).yx,
        (*image_matrix).yy,
        (*image_matrix).x0,
        (*image_matrix).y0,
    );
    status = ((*surface).emit_image)
        .expect("non-null function pointer")(image, (*surface).stream) as cairo_status_t;
    cairo_surface_destroy(&mut (*image).base);
    _cairo_output_stream_printf(
        (*surface).stream,
        b"Q\n\0" as *const u8 as *const libc::c_char,
    );
    return status;
}
unsafe extern "C" fn _cairo_type3_glyph_surface_emit_image_pattern(
    mut surface: *mut cairo_type3_glyph_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut pattern_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut mat: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut upside_down: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*image).width == 0 as libc::c_int || (*image).height == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    mat = *pattern_matrix;
    status = cairo_matrix_invert(&mut mat);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-type3-glyph-surface.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 141],
                &[libc::c_char; 141],
            >(
                b"cairo_status_t _cairo_type3_glyph_surface_emit_image_pattern(cairo_type3_glyph_surface_t *, cairo_image_surface_t *, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(&mut mat, &mut mat, &mut (*surface).cairo_to_pdf);
    cairo_matrix_scale(
        &mut mat,
        (*image).width as libc::c_double,
        (*image).height as libc::c_double,
    );
    cairo_matrix_init(
        &mut upside_down,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    );
    cairo_matrix_multiply(&mut mat, &mut upside_down, &mut mat);
    return _cairo_type3_glyph_surface_emit_image(surface, image, &mut mat);
}
unsafe extern "C" fn _cairo_type3_glyph_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    return _cairo_pdf_operators_fini(&mut (*surface).pdf_operators);
}
unsafe extern "C" fn _cairo_type3_glyph_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    let mut pattern: *const cairo_surface_pattern_t = 0
        as *const cairo_surface_pattern_t;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*source).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_IMAGE_FALLBACK;
    }
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    pattern = source as *const cairo_surface_pattern_t;
    status = _cairo_surface_acquire_source_image(
        (*pattern).surface,
        &mut image,
        &mut image_extra,
    );
    if !(status as u64 != 0) {
        status = _cairo_type3_glyph_surface_emit_image_pattern(
            surface,
            image,
            &(*pattern).base.matrix,
        );
    }
    _cairo_surface_release_source_image((*pattern).surface, image, image_extra);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_type3_glyph_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    return _cairo_type3_glyph_surface_paint(abstract_surface, op, mask, clip);
}
unsafe extern "C" fn _cairo_type3_glyph_surface_stroke(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    return _cairo_pdf_operators_stroke(
        &mut (*surface).pdf_operators,
        path,
        style,
        ctm,
        ctm_inverse,
    );
}
unsafe extern "C" fn _cairo_type3_glyph_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    return _cairo_pdf_operators_fill(&mut (*surface).pdf_operators, path, fill_rule);
}
unsafe extern "C" fn _cairo_type3_glyph_surface_show_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut new_ctm: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    cairo_matrix_multiply(
        &mut new_ctm,
        &mut (*surface).cairo_to_pdf,
        &mut (*scaled_font).ctm,
    );
    font = cairo_scaled_font_create(
        (*scaled_font).font_face,
        &mut (*scaled_font).font_matrix,
        &mut new_ctm,
        &mut (*scaled_font).options,
    );
    if (*font).status as u64 != 0 {
        return (*font).status as cairo_int_status_t;
    }
    status = _cairo_pdf_operators_show_text_glyphs(
        &mut (*surface).pdf_operators,
        0 as *const libc::c_char,
        0 as libc::c_int,
        glyphs,
        num_glyphs,
        0 as *const cairo_text_cluster_t,
        0 as libc::c_int,
        0 as cairo_text_cluster_flags_t,
        font,
    );
    cairo_scaled_font_destroy(font);
    return status;
}
static mut cairo_type3_glyph_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_TYPE3_GLYPH as libc::c_int
                as cairo_surface_type_t,
            finish: Some(
                _cairo_type3_glyph_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: None,
            create_similar_image: None,
            map_to_image: None,
            unmap_image: None,
            source: None,
            acquire_source_image: None,
            release_source_image: None,
            snapshot: None,
            copy_page: None,
            show_page: None,
            get_extents: None,
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_type3_glyph_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_type3_glyph_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_type3_glyph_surface_stroke
                    as unsafe extern "C" fn(
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
            ),
            fill: Some(
                _cairo_type3_glyph_surface_fill
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_path_fixed_t,
                        cairo_fill_rule_t,
                        libc::c_double,
                        cairo_antialias_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            fill_stroke: None,
            show_glyphs: Some(
                _cairo_type3_glyph_surface_show_glyphs
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *mut cairo_glyph_t,
                        libc::c_int,
                        *mut cairo_scaled_font_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            has_show_text_glyphs: None,
            show_text_glyphs: None,
            get_supported_mime_types: None,
            tag: None,
        };
        init
    }
};
unsafe extern "C" fn _cairo_type3_glyph_surface_set_stream(
    mut surface: *mut cairo_type3_glyph_surface_t,
    mut stream: *mut cairo_output_stream_t,
) {
    let ref mut fresh3 = (*surface).stream;
    *fresh3 = stream;
    _cairo_pdf_operators_set_stream(&mut (*surface).pdf_operators, stream);
}
unsafe extern "C" fn _cairo_type3_glyph_surface_emit_fallback_image(
    mut surface: *mut cairo_type3_glyph_surface_t,
    mut glyph_index: libc::c_ulong,
) -> cairo_status_t {
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut mat: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    status = _cairo_scaled_glyph_lookup(
        (*surface).scaled_font,
        glyph_index,
        (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
            | CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int)
            as cairo_scaled_glyph_info_t,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    image = (*scaled_glyph).surface;
    if (*image).width == 0 as libc::c_int || (*image).height == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    x = _cairo_fixed_to_double((*scaled_glyph).bbox.p1.x);
    y = _cairo_fixed_to_double((*scaled_glyph).bbox.p2.y);
    cairo_matrix_init(
        &mut mat,
        (*image).width as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(*image).height as libc::c_double,
        x,
        y,
    );
    cairo_matrix_multiply(
        &mut mat,
        &mut mat,
        &mut (*(*surface).scaled_font).scale_inverse,
    );
    return _cairo_type3_glyph_surface_emit_image(surface, image, &mut mat);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type3_glyph_surface_set_font_subsets_callback(
    mut abstract_surface: *mut libc::c_void,
    mut use_font_subset: cairo_pdf_operators_use_font_subset_t,
    mut closure: *mut libc::c_void,
) {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    if (*surface).base.status as u64 != 0 {
        return;
    }
    _cairo_pdf_operators_set_font_subsets_callback(
        &mut (*surface).pdf_operators,
        use_font_subset,
        closure,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type3_glyph_surface_analyze_glyph(
    mut abstract_surface: *mut libc::c_void,
    mut glyph_index: libc::c_ulong,
) -> cairo_status_t {
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut status2: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut null_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    if (*surface).base.status as u64 != 0 {
        return (*surface).base.status;
    }
    null_stream = _cairo_null_stream_create();
    if (*null_stream).status as u64 != 0 {
        return (*null_stream).status;
    }
    _cairo_type3_glyph_surface_set_stream(surface, null_stream);
    _cairo_scaled_font_freeze_cache((*surface).scaled_font);
    status = _cairo_scaled_glyph_lookup(
        (*surface).scaled_font,
        glyph_index,
        CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    );
    if !(status as libc::c_uint
        != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint)
    {
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            status = CAIRO_INT_STATUS_SUCCESS;
        } else {
            status = _cairo_recording_surface_replay(
                (*scaled_glyph).recording_surface,
                &mut (*surface).base,
            ) as cairo_int_status_t;
            if !(status as u64 != 0) {
                status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                    as cairo_int_status_t;
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int as libc::c_uint
                {
                    status = CAIRO_INT_STATUS_SUCCESS;
                }
            }
        }
    }
    _cairo_scaled_font_thaw_cache((*surface).scaled_font);
    status2 = _cairo_output_stream_destroy(null_stream) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = status2;
    }
    return status as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_type3_glyph_surface_emit_glyph(
    mut abstract_surface: *mut libc::c_void,
    mut stream: *mut cairo_output_stream_t,
    mut glyph_index: libc::c_ulong,
    mut bbox: *mut cairo_box_t,
    mut width: *mut libc::c_double,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_type3_glyph_surface_t = abstract_surface
        as *mut cairo_type3_glyph_surface_t;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut status2: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut x_advance: libc::c_double = 0.;
    let mut y_advance: libc::c_double = 0.;
    let mut font_matrix_inverse: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    if (*surface).base.status as u64 != 0 {
        return (*surface).base.status;
    }
    _cairo_type3_glyph_surface_set_stream(surface, stream);
    _cairo_scaled_font_freeze_cache((*surface).scaled_font);
    status = _cairo_scaled_glyph_lookup(
        (*surface).scaled_font,
        glyph_index,
        (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
            | CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE as libc::c_int)
            as cairo_scaled_glyph_info_t,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    );
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        status = _cairo_scaled_glyph_lookup(
            (*surface).scaled_font,
            glyph_index,
            CAIRO_SCALED_GLYPH_INFO_METRICS,
            0 as *const cairo_color_t,
            &mut scaled_glyph,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = CAIRO_INT_STATUS_IMAGE_FALLBACK;
        }
    }
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        _cairo_scaled_font_thaw_cache((*surface).scaled_font);
        return status as cairo_status_t;
    }
    x_advance = (*scaled_glyph).metrics.x_advance;
    y_advance = (*scaled_glyph).metrics.y_advance;
    font_matrix_inverse = (*(*surface).scaled_font).font_matrix;
    status2 = cairo_matrix_invert(&mut font_matrix_inverse) as cairo_int_status_t;
    if status2 as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status2 == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-type3-glyph-surface.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"cairo_status_t _cairo_type3_glyph_surface_emit_glyph(void *, cairo_output_stream_t *, unsigned long, cairo_box_t *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_transform_distance(
        &mut font_matrix_inverse,
        &mut x_advance,
        &mut y_advance,
    );
    *width = x_advance;
    *bbox = (*scaled_glyph).bbox;
    _cairo_matrix_transform_bounding_box_fixed(
        &mut (*(*surface).scaled_font).scale_inverse,
        bbox,
        0 as *mut cairo_bool_t,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"%f 0 %f %f %f %f d1\n\0" as *const u8 as *const libc::c_char,
        x_advance,
        _cairo_fixed_to_double((*bbox).p1.x),
        _cairo_fixed_to_double((*bbox).p1.y),
        _cairo_fixed_to_double((*bbox).p2.x),
        _cairo_fixed_to_double((*bbox).p2.y),
    );
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        let mut mem_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
        mem_stream = _cairo_memory_stream_create();
        status = (*mem_stream).status as cairo_int_status_t;
        if status as u64 != 0 {
            current_block = 5490365314095407341;
        } else {
            _cairo_type3_glyph_surface_set_stream(surface, mem_stream);
            _cairo_output_stream_printf(
                (*surface).stream,
                b"q\n\0" as *const u8 as *const libc::c_char,
            );
            status = _cairo_recording_surface_replay(
                (*scaled_glyph).recording_surface,
                &mut (*surface).base,
            ) as cairo_int_status_t;
            status2 = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                as cairo_int_status_t;
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = status2;
            }
            _cairo_output_stream_printf(
                (*surface).stream,
                b"Q\n\0" as *const u8 as *const libc::c_char,
            );
            _cairo_type3_glyph_surface_set_stream(surface, stream);
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                _cairo_memory_stream_copy(mem_stream, stream);
            }
            status2 = _cairo_output_stream_destroy(mem_stream) as cairo_int_status_t;
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = status2;
            }
            current_block = 3123434771885419771;
        }
    } else {
        current_block = 3123434771885419771;
    }
    match current_block {
        3123434771885419771 => {
            if status as libc::c_uint
                == CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int as libc::c_uint
            {
                status = _cairo_type3_glyph_surface_emit_fallback_image(
                    surface,
                    glyph_index,
                ) as cairo_int_status_t;
            }
        }
        _ => {}
    }
    _cairo_scaled_font_thaw_cache((*surface).scaled_font);
    return status as cairo_status_t;
}
