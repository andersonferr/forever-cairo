use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    fn cairo_surface_get_font_options(
        surface: *mut cairo_surface_t,
        options: *mut cairo_font_options_t,
    );
    fn cairo_surface_set_device_scale(
        surface: *mut cairo_surface_t,
        x_scale: libc::c_double,
        y_scale: libc::c_double,
    );
    fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn cairo_surface_show_page(surface: *mut cairo_surface_t);
    fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_recording_surface_create(
        content: cairo_content_t,
        extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    fn cairo_matrix_init(
        matrix: *mut cairo_matrix_t,
        xx: libc::c_double,
        yx: libc::c_double,
        xy: libc::c_double,
        yy: libc::c_double,
        x0: libc::c_double,
        y0: libc::c_double,
    );
    fn cairo_region_num_rectangles(region: *const cairo_region_t) -> libc::c_int;
    fn cairo_region_get_rectangle(
        region: *const cairo_region_t,
        nth: libc::c_int,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    fn _cairo_clip_intersect_rectangle(
        clip: *mut cairo_clip_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_surface_set_error(
        surface: *mut cairo_surface_t,
        status: cairo_int_status_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_surface_set_font_options(
        surface: *mut cairo_surface_t,
        options: *mut cairo_font_options_t,
    );
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_mask(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_stroke(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_fill(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_show_text_glyphs(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clusters: *const cairo_text_cluster_t,
        num_clusters: libc::c_int,
        cluster_flags: cairo_text_cluster_flags_t,
        scaled_font: *mut cairo_scaled_font_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_tag(
        surface: *mut cairo_surface_t,
        begin: cairo_bool_t,
        tag_name: *const libc::c_char,
        attributes: *const libc::c_char,
    ) -> cairo_status_t;
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_image_surface_create_with_content(
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_recording_surface_replay(
        surface: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_recording_surface_replay_and_create_regions(
        surface: *mut cairo_surface_t,
        surface_transform: *const cairo_matrix_t,
        target: *mut cairo_surface_t,
        surface_is_unbounded: cairo_bool_t,
    ) -> cairo_status_t;
    fn _cairo_recording_surface_replay_region(
        surface: *mut cairo_surface_t,
        surface_extents: *const cairo_rectangle_int_t,
        target: *mut cairo_surface_t,
        region: cairo_recording_region_type_t,
    ) -> cairo_status_t;
    fn _cairo_analysis_surface_create(
        target: *mut cairo_surface_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_analysis_surface_get_unsupported(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_region_t;
    fn _cairo_analysis_surface_has_supported(
        surface: *mut cairo_surface_t,
    ) -> cairo_bool_t;
    fn _cairo_analysis_surface_has_unsupported(
        surface: *mut cairo_surface_t,
    ) -> cairo_bool_t;
    fn _cairo_analysis_surface_get_bounding_box(
        surface: *mut cairo_surface_t,
        bbox: *mut cairo_box_t,
    );
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
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_paginated_surface_backend {
    pub start_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub set_paginated_mode: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_paginated_mode_t,
        ) -> cairo_int_status_t,
    >,
    pub set_bounding_box: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_box_t) -> cairo_int_status_t,
    >,
    pub set_fallback_images_required: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_bool_t) -> cairo_int_status_t,
    >,
    pub supports_fine_grained_fallbacks: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
    pub requires_thumbnail_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> cairo_bool_t,
    >,
    pub set_thumbnail_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
        ) -> cairo_int_status_t,
    >,
}
pub type cairo_paginated_mode_t = _cairo_paginated_mode;
pub type _cairo_paginated_mode = libc::c_uint;
pub const CAIRO_PAGINATED_MODE_FALLBACK: _cairo_paginated_mode = 2;
pub const CAIRO_PAGINATED_MODE_RENDER: _cairo_paginated_mode = 1;
pub const CAIRO_PAGINATED_MODE_ANALYZE: _cairo_paginated_mode = 0;
pub type cairo_paginated_surface_backend_t = _cairo_paginated_surface_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_subsurface {
    pub base: cairo_surface_t,
    pub extents: cairo_rectangle_int_t,
    pub target: *mut cairo_surface_t,
    pub snapshot: *mut cairo_surface_t,
}
pub type cairo_surface_subsurface_t = _cairo_surface_subsurface;
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
pub type cairo_paginated_surface_t = _cairo_paginated_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_paginated_surface {
    pub base: cairo_surface_t,
    pub target: *mut cairo_surface_t,
    pub content: cairo_content_t,
    pub backend: *const cairo_paginated_surface_backend_t,
    pub recording_surface: *mut cairo_surface_t,
    pub page_num: libc::c_int,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub type cairo_recording_region_type_t = libc::c_uint;
pub const CAIRO_RECORDING_REGION_IMAGE_FALLBACK: cairo_recording_region_type_t = 2;
pub const CAIRO_RECORDING_REGION_NATIVE: cairo_recording_region_type_t = 1;
pub const CAIRO_RECORDING_REGION_ALL: cairo_recording_region_type_t = 0;
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline]
unsafe extern "C" fn _cairo_surface_subsurface_get_target(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    return (*(surface as *mut cairo_surface_subsurface_t)).target;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_subsurface(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn _cairo_paginated_surface_create_similar(
    mut abstract_surface: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut rect: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    rect.y = 0.0f64;
    rect.x = rect.y;
    rect.width = width as libc::c_double;
    rect.height = height as libc::c_double;
    return cairo_recording_surface_create(content, &mut rect);
}
unsafe extern "C" fn _create_recording_surface_for_target(
    mut target: *mut cairo_surface_t,
    mut content: cairo_content_t,
) -> *mut cairo_surface_t {
    let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if _cairo_surface_get_extents(target, &mut rect) != 0 {
        let mut recording_extents: cairo_rectangle_t = cairo_rectangle_t {
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
        };
        recording_extents.x = rect.x as libc::c_double;
        recording_extents.y = rect.y as libc::c_double;
        recording_extents.width = rect.width as libc::c_double;
        recording_extents.height = rect.height as libc::c_double;
        return cairo_recording_surface_create(content, &mut recording_extents);
    } else {
        return cairo_recording_surface_create(content, 0 as *const cairo_rectangle_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_paginated_surface_create(
    mut target: *mut cairo_surface_t,
    mut content: cairo_content_t,
    mut backend: *const cairo_paginated_surface_backend_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_paginated_surface_t = 0
        as *mut cairo_paginated_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    surface = (if ::std::mem::size_of::<cairo_paginated_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_paginated_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_paginated_surface_t;
    if surface.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _cairo_surface_init(
            &mut (*surface).base,
            &cairo_paginated_surface_backend,
            0 as *mut cairo_device_t,
            content,
            (*target).is_vector() as cairo_bool_t,
        );
        (*surface).base.type_0 = (*target).type_0;
        let ref mut fresh0 = (*surface).target;
        *fresh0 = cairo_surface_reference(target);
        (*surface).content = content;
        let ref mut fresh1 = (*surface).backend;
        *fresh1 = backend;
        let ref mut fresh2 = (*surface).recording_surface;
        *fresh2 = _create_recording_surface_for_target(target, content);
        status = (*(*surface).recording_surface).status;
        if status as u64 != 0 {
            cairo_surface_destroy(target);
            free(surface as *mut libc::c_void);
        } else {
            (*surface).page_num = 1 as libc::c_int;
            let ref mut fresh3 = (*surface).base;
            (*fresh3).set_is_clear(1 as libc::c_int as libc::c_uint);
            return &mut (*surface).base;
        }
    }
    return _cairo_surface_create_in_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_is_paginated(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*surface).backend
        == &cairo_paginated_surface_backend as *const cairo_surface_backend_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_paginated_surface_get_target(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut paginated_surface: *mut cairo_paginated_surface_t = 0
        as *mut cairo_paginated_surface_t;
    if _cairo_surface_is_paginated(surface) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_is_paginated (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-paginated-surface.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"cairo_surface_t *_cairo_paginated_surface_get_target(cairo_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    paginated_surface = surface as *mut cairo_paginated_surface_t;
    return (*paginated_surface).target;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_paginated_surface_get_recording(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut paginated_surface: *mut cairo_paginated_surface_t = 0
        as *mut cairo_paginated_surface_t;
    if _cairo_surface_is_paginated(surface) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_is_paginated (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-paginated-surface.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"cairo_surface_t *_cairo_paginated_surface_get_recording(cairo_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    paginated_surface = surface as *mut cairo_paginated_surface_t;
    return (*paginated_surface).recording_surface;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_paginated_surface_set_size(
    mut surface: *mut cairo_surface_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> cairo_status_t {
    let mut paginated_surface: *mut cairo_paginated_surface_t = 0
        as *mut cairo_paginated_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut recording_extents: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    if _cairo_surface_is_paginated(surface) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_is_paginated (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-paginated-surface.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"cairo_status_t _cairo_paginated_surface_set_size(cairo_surface_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    paginated_surface = surface as *mut cairo_paginated_surface_t;
    recording_extents.x = 0 as libc::c_int as libc::c_double;
    recording_extents.y = 0 as libc::c_int as libc::c_double;
    recording_extents.width = width as libc::c_double;
    recording_extents.height = height as libc::c_double;
    cairo_surface_destroy((*paginated_surface).recording_surface);
    let ref mut fresh4 = (*paginated_surface).recording_surface;
    *fresh4 = cairo_recording_surface_create(
        (*paginated_surface).content,
        &mut recording_extents,
    );
    status = (*(*paginated_surface).recording_surface).status;
    if status as u64 != 0 {
        return _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_paginated_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if ((*surface).base).is_clear() == 0 || (*surface).page_num == 1 as libc::c_int {
        status = _cairo_paginated_surface_show_page(surface as *mut libc::c_void)
            as cairo_status_t;
    }
    if _cairo_atomic_int_get(&mut (*(*surface).target).ref_count.ref_count)
        == 1 as libc::c_int
    {
        cairo_surface_finish((*surface).target);
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = cairo_surface_status((*surface).target);
    }
    cairo_surface_destroy((*surface).target);
    cairo_surface_finish((*surface).recording_surface);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = cairo_surface_status((*surface).recording_surface);
    }
    cairo_surface_destroy((*surface).recording_surface);
    return status;
}
unsafe extern "C" fn _cairo_paginated_surface_create_image_surface(
    mut abstract_surface: *mut libc::c_void,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut options: cairo_font_options_t = cairo_font_options_t {
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
    image = _cairo_image_surface_create_with_content((*surface).content, width, height);
    cairo_surface_get_font_options(&mut (*surface).base, &mut options);
    _cairo_surface_set_font_options(image, &mut options);
    return image;
}
unsafe extern "C" fn _cairo_paginated_surface_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_get_source((*surface).target, extents);
}
unsafe extern "C" fn _cairo_paginated_surface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    let mut is_bounded: cairo_bool_t = 0;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    is_bounded = _cairo_surface_get_extents((*surface).target, &mut extents);
    if is_bounded == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    image = _cairo_paginated_surface_create_image_surface(
        surface as *mut libc::c_void,
        extents.width,
        extents.height,
    );
    status = _cairo_recording_surface_replay((*surface).recording_surface, image);
    if status as u64 != 0 {
        cairo_surface_destroy(image);
        return status;
    }
    *image_out = image as *mut cairo_image_surface_t;
    *image_extra = 0 as *mut libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_paginated_surface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    cairo_surface_destroy(&mut (*image).base);
}
unsafe extern "C" fn _paint_thumbnail_image(
    mut surface: *mut cairo_paginated_surface_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> cairo_int_status_t {
    let mut pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
        base: cairo_pattern_t {
            ref_count: cairo_reference_count_t {
                ref_count: 0,
            },
            status: CAIRO_STATUS_SUCCESS,
            user_data: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
            observers: cairo_list_t {
                next: 0 as *const _cairo_list as *mut _cairo_list,
                prev: 0 as *const _cairo_list as *mut _cairo_list,
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
        surface: 0 as *mut cairo_surface_t,
    };
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut x_scale: libc::c_double = 0.;
    let mut y_scale: libc::c_double = 0.;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut opaque: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_surface_get_extents((*surface).target, &mut extents);
    x_scale = width as libc::c_double / extents.width as libc::c_double;
    y_scale = height as libc::c_double / extents.height as libc::c_double;
    image = _cairo_paginated_surface_create_image_surface(
        surface as *mut libc::c_void,
        width,
        height,
    );
    cairo_surface_set_device_scale(image, x_scale, y_scale);
    cairo_surface_set_device_offset(
        image,
        -extents.x as libc::c_double * x_scale,
        -extents.y as libc::c_double * y_scale,
    );
    status = _cairo_recording_surface_replay((*surface).recording_surface, image);
    if !(status as u64 != 0) {
        opaque = cairo_image_surface_create(CAIRO_FORMAT_RGB24, width, height);
        if (*opaque).status as u64 != 0 {
            status = (*opaque).status;
        } else {
            status = _cairo_surface_paint(
                opaque,
                CAIRO_OPERATOR_SOURCE,
                &_cairo_pattern_white.base,
                0 as *const cairo_clip_t,
            );
            if !(status as u64 != 0) {
                _cairo_pattern_init_for_surface(&mut pattern, image);
                pattern.base.filter = CAIRO_FILTER_NEAREST;
                status = _cairo_surface_paint(
                    opaque,
                    CAIRO_OPERATOR_OVER,
                    &mut pattern.base,
                    0 as *const cairo_clip_t,
                );
                _cairo_pattern_fini(&mut pattern.base);
                if !(status as u64 != 0) {
                    status = ((*(*surface).backend).set_thumbnail_image)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*surface).target as *mut libc::c_void,
                        opaque as *mut cairo_image_surface_t,
                    ) as cairo_status_t;
                }
            }
        }
    }
    if !image.is_null() {
        cairo_surface_destroy(image);
    }
    if !opaque.is_null() {
        cairo_surface_destroy(opaque);
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _paint_fallback_image(
    mut surface: *mut cairo_paginated_surface_t,
    mut rect: *mut cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut x_scale: libc::c_double = (*surface).base.x_fallback_resolution
        / (*(*surface).target).x_resolution;
    let mut y_scale: libc::c_double = (*surface).base.y_fallback_resolution
        / (*(*surface).target).y_resolution;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
        base: cairo_pattern_t {
            ref_count: cairo_reference_count_t {
                ref_count: 0,
            },
            status: CAIRO_STATUS_SUCCESS,
            user_data: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
            observers: cairo_list_t {
                next: 0 as *const _cairo_list as *mut _cairo_list,
                prev: 0 as *const _cairo_list as *mut _cairo_list,
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
        surface: 0 as *mut cairo_surface_t,
    };
    let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    x = (*rect).x;
    y = (*rect).y;
    width = (*rect).width;
    height = (*rect).height;
    image = _cairo_paginated_surface_create_image_surface(
        surface as *mut libc::c_void,
        ceil(width as libc::c_double * x_scale) as libc::c_int,
        ceil(height as libc::c_double * y_scale) as libc::c_int,
    );
    cairo_surface_set_device_scale(image, x_scale, y_scale);
    cairo_surface_set_device_offset(
        image,
        -x as libc::c_double * x_scale,
        -y as libc::c_double * y_scale,
    );
    status = _cairo_recording_surface_replay((*surface).recording_surface, image);
    if !(status as u64 != 0) {
        _cairo_pattern_init_for_surface(&mut pattern, image);
        cairo_matrix_init(
            &mut pattern.base.matrix,
            x_scale,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            y_scale,
            -x as libc::c_double * x_scale,
            -y as libc::c_double * y_scale,
        );
        pattern.base.filter = CAIRO_FILTER_NEAREST;
        clip = _cairo_clip_intersect_rectangle(0 as *mut cairo_clip_t, rect);
        status = _cairo_surface_paint(
            (*surface).target,
            CAIRO_OPERATOR_SOURCE,
            &mut pattern.base,
            clip,
        );
        _cairo_clip_destroy(clip);
        _cairo_pattern_fini(&mut pattern.base);
    }
    cairo_surface_destroy(image);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _paint_page(
    mut surface: *mut cairo_paginated_surface_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut analysis: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut has_supported: cairo_bool_t = 0;
    let mut has_page_fallback: cairo_bool_t = 0;
    let mut has_finegrained_fallback: cairo_bool_t = 0;
    if (*(*surface).target).status as u64 != 0 {
        return (*(*surface).target).status as cairo_int_status_t;
    }
    analysis = _cairo_analysis_surface_create((*surface).target);
    if (*analysis).status as u64 != 0 {
        return _cairo_surface_set_error(
            (*surface).target,
            (*analysis).status as cairo_int_status_t,
        );
    }
    status = ((*(*surface).backend).set_paginated_mode)
        .expect(
            "non-null function pointer",
        )((*surface).target as *mut libc::c_void, CAIRO_PAGINATED_MODE_ANALYZE);
    if !(status as u64 != 0) {
        status = _cairo_recording_surface_replay_and_create_regions(
            (*surface).recording_surface,
            0 as *const cairo_matrix_t,
            analysis,
            0 as libc::c_int,
        ) as cairo_int_status_t;
        if !(status as u64 != 0) {
            if (*analysis).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"analysis->status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-paginated-surface.c\0" as *const u8
                        as *const libc::c_char,
                    422 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 60],
                        &[libc::c_char; 60],
                    >(b"cairo_int_status_t _paint_page(cairo_paginated_surface_t *)\0"))
                        .as_ptr(),
                );
            }
            if ((*(*surface).backend).set_bounding_box).is_some() {
                let mut bbox: cairo_box_t = cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                };
                _cairo_analysis_surface_get_bounding_box(analysis, &mut bbox);
                status = ((*(*surface).backend).set_bounding_box)
                    .expect(
                        "non-null function pointer",
                    )((*surface).target as *mut libc::c_void, &mut bbox);
                if status as u64 != 0 {
                    current_block = 13465148123302063353;
                } else {
                    current_block = 9606288038608642794;
                }
            } else {
                current_block = 9606288038608642794;
            }
            match current_block {
                13465148123302063353 => {}
                _ => {
                    if ((*(*surface).backend).set_fallback_images_required).is_some() {
                        let mut has_fallbacks: cairo_bool_t = _cairo_analysis_surface_has_unsupported(
                            analysis,
                        );
                        status = ((*(*surface).backend).set_fallback_images_required)
                            .expect(
                                "non-null function pointer",
                            )((*surface).target as *mut libc::c_void, has_fallbacks);
                        if status as u64 != 0 {
                            current_block = 13465148123302063353;
                        } else {
                            current_block = 12124785117276362961;
                        }
                    } else {
                        current_block = 12124785117276362961;
                    }
                    match current_block {
                        13465148123302063353 => {}
                        _ => {
                            if ((*(*surface).backend).supports_fine_grained_fallbacks)
                                .is_some()
                                && ((*(*surface).backend).supports_fine_grained_fallbacks)
                                    .expect(
                                        "non-null function pointer",
                                    )((*surface).target as *mut libc::c_void) != 0
                            {
                                has_supported = _cairo_analysis_surface_has_supported(
                                    analysis,
                                );
                                has_page_fallback = 0 as libc::c_int;
                                has_finegrained_fallback = _cairo_analysis_surface_has_unsupported(
                                    analysis,
                                );
                            } else {
                                if _cairo_analysis_surface_has_unsupported(analysis) != 0 {
                                    has_supported = 0 as libc::c_int;
                                    has_page_fallback = 1 as libc::c_int;
                                } else {
                                    has_supported = 1 as libc::c_int;
                                    has_page_fallback = 0 as libc::c_int;
                                }
                                has_finegrained_fallback = 0 as libc::c_int;
                            }
                            if has_supported != 0 {
                                status = ((*(*surface).backend).set_paginated_mode)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*surface).target as *mut libc::c_void,
                                    CAIRO_PAGINATED_MODE_RENDER,
                                );
                                if status as u64 != 0 {
                                    current_block = 13465148123302063353;
                                } else {
                                    status = _cairo_recording_surface_replay_region(
                                        (*surface).recording_surface,
                                        0 as *const cairo_rectangle_int_t,
                                        (*surface).target,
                                        CAIRO_RECORDING_REGION_NATIVE,
                                    ) as cairo_int_status_t;
                                    if status as libc::c_uint
                                        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                            as libc::c_uint
                                    {} else {
                                        __assert_fail(
                                            b"status != CAIRO_INT_STATUS_UNSUPPORTED\0" as *const u8
                                                as *const libc::c_char,
                                            b"../src/cairo-paginated-surface.c\0" as *const u8
                                                as *const libc::c_char,
                                            473 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 60],
                                                &[libc::c_char; 60],
                                            >(
                                                b"cairo_int_status_t _paint_page(cairo_paginated_surface_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    if status as u64 != 0 {
                                        current_block = 13465148123302063353;
                                    } else {
                                        current_block = 7659304154607701039;
                                    }
                                }
                            } else {
                                current_block = 7659304154607701039;
                            }
                            match current_block {
                                13465148123302063353 => {}
                                _ => {
                                    if has_page_fallback != 0 {
                                        let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
                                            x: 0,
                                            y: 0,
                                            width: 0,
                                            height: 0,
                                        };
                                        let mut is_bounded: cairo_bool_t = 0;
                                        status = ((*(*surface).backend).set_paginated_mode)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*surface).target as *mut libc::c_void,
                                            CAIRO_PAGINATED_MODE_FALLBACK,
                                        );
                                        if status as u64 != 0 {
                                            current_block = 13465148123302063353;
                                        } else {
                                            is_bounded = _cairo_surface_get_extents(
                                                (*surface).target,
                                                &mut extents,
                                            );
                                            if is_bounded == 0 {
                                                status = CAIRO_INT_STATUS_UNSUPPORTED;
                                                current_block = 13465148123302063353;
                                            } else {
                                                status = _paint_fallback_image(surface, &mut extents);
                                                if status as u64 != 0 {
                                                    current_block = 13465148123302063353;
                                                } else {
                                                    current_block = 1924505913685386279;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 1924505913685386279;
                                    }
                                    match current_block {
                                        13465148123302063353 => {}
                                        _ => {
                                            if has_finegrained_fallback != 0 {
                                                let mut region: *mut cairo_region_t = 0
                                                    as *mut cairo_region_t;
                                                let mut num_rects: libc::c_int = 0;
                                                let mut i: libc::c_int = 0;
                                                status = ((*(*surface).backend).set_paginated_mode)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    (*surface).target as *mut libc::c_void,
                                                    CAIRO_PAGINATED_MODE_FALLBACK,
                                                );
                                                if status as u64 != 0 {
                                                    current_block = 13465148123302063353;
                                                } else {
                                                    region = _cairo_analysis_surface_get_unsupported(analysis);
                                                    num_rects = cairo_region_num_rectangles(region);
                                                    i = 0 as libc::c_int;
                                                    loop {
                                                        if !(i < num_rects) {
                                                            current_block = 3546145585875536353;
                                                            break;
                                                        }
                                                        let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                                                            x: 0,
                                                            y: 0,
                                                            width: 0,
                                                            height: 0,
                                                        };
                                                        cairo_region_get_rectangle(region, i, &mut rect);
                                                        status = _paint_fallback_image(surface, &mut rect);
                                                        if status as u64 != 0 {
                                                            current_block = 13465148123302063353;
                                                            break;
                                                        }
                                                        i += 1;
                                                    }
                                                }
                                            } else {
                                                current_block = 3546145585875536353;
                                            }
                                            match current_block {
                                                13465148123302063353 => {}
                                                _ => {
                                                    if ((*(*surface).backend).requires_thumbnail_image)
                                                        .is_some()
                                                    {
                                                        let mut width: libc::c_int = 0;
                                                        let mut height: libc::c_int = 0;
                                                        if ((*(*surface).backend).requires_thumbnail_image)
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            (*surface).target as *mut libc::c_void,
                                                            &mut width,
                                                            &mut height,
                                                        ) != 0
                                                        {
                                                            _paint_thumbnail_image(surface, width, height);
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
        }
    }
    cairo_surface_destroy(analysis);
    return _cairo_surface_set_error((*surface).target, status);
}
unsafe extern "C" fn _start_page(
    mut surface: *mut cairo_paginated_surface_t,
) -> cairo_status_t {
    if (*(*surface).target).status as u64 != 0 {
        return (*(*surface).target).status;
    }
    if ((*(*surface).backend).start_page).is_none() {
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_surface_set_error(
        (*surface).target,
        ((*(*surface).backend).start_page)
            .expect("non-null function pointer")((*surface).target as *mut libc::c_void),
    ) as cairo_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_copy_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    status = _start_page(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _paint_page(surface) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    let ref mut fresh5 = (*surface).page_num;
    *fresh5 += 1;
    cairo_surface_show_page((*surface).target);
    return cairo_surface_status((*surface).target) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_show_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    status = _start_page(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _paint_page(surface) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    cairo_surface_show_page((*surface).target);
    status = (*(*surface).target).status;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = (*(*surface).recording_surface).status;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if ((*surface).base).finished() == 0 {
        cairo_surface_destroy((*surface).recording_surface);
        let ref mut fresh6 = (*surface).recording_surface;
        *fresh6 = _create_recording_surface_for_target(
            (*surface).target,
            (*surface).content,
        );
        status = (*(*surface).recording_surface).status;
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        let ref mut fresh7 = (*surface).page_num;
        *fresh7 += 1;
        let ref mut fresh8 = (*surface).base;
        (*fresh8).set_is_clear(1 as libc::c_int as libc::c_uint);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_get_extents((*surface).target, rectangle);
}
unsafe extern "C" fn _cairo_paginated_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    cairo_surface_get_font_options((*surface).target, options);
}
unsafe extern "C" fn _cairo_paginated_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_paint((*surface).recording_surface, op, source, clip)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_mask((*surface).recording_surface, op, source, mask, clip)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_stroke(
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
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_stroke(
        (*surface).recording_surface,
        op,
        source,
        path,
        style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_fill(
        (*surface).recording_surface,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_has_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return cairo_surface_has_show_text_glyphs((*surface).target);
}
unsafe extern "C" fn _cairo_paginated_surface_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut clusters: *const cairo_text_cluster_t,
    mut num_clusters: libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_show_text_glyphs(
        (*surface).recording_surface,
        op,
        source,
        utf8,
        utf8_len,
        glyphs,
        num_glyphs,
        clusters,
        num_clusters,
        cluster_flags,
        scaled_font,
        clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_get_supported_mime_types(
    mut abstract_surface: *mut libc::c_void,
) -> *mut *const libc::c_char {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    if ((*(*(*surface).target).backend).get_supported_mime_types).is_some() {
        return ((*(*(*surface).target).backend).get_supported_mime_types)
            .expect("non-null function pointer")((*surface).target as *mut libc::c_void);
    }
    return 0 as *mut *const libc::c_char;
}
unsafe extern "C" fn _cairo_paginated_surface_tag(
    mut abstract_surface: *mut libc::c_void,
    mut begin: cairo_bool_t,
    mut tag_name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_paginated_surface_t = abstract_surface
        as *mut cairo_paginated_surface_t;
    return _cairo_surface_tag((*surface).recording_surface, begin, tag_name, attributes)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_paginated_surface_snapshot(
    mut abstract_other: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut other: *mut cairo_paginated_surface_t = abstract_other
        as *mut cairo_paginated_surface_t;
    return ((*(*(*other).recording_surface).backend).snapshot)
        .expect(
            "non-null function pointer",
        )((*other).recording_surface as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_paginated_context_create(
    mut target: *mut libc::c_void,
) -> *mut cairo_t {
    let mut surface: *mut cairo_paginated_surface_t = target
        as *mut cairo_paginated_surface_t;
    if _cairo_surface_is_subsurface(&mut (*surface).base) != 0 {
        surface = _cairo_surface_subsurface_get_target(&mut (*surface).base)
            as *mut cairo_paginated_surface_t;
    }
    return ((*(*(*surface).recording_surface).backend).create_context)
        .expect("non-null function pointer")(target);
}
static mut cairo_paginated_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_PAGINATED as libc::c_int
                as cairo_surface_type_t,
            finish: Some(
                _cairo_paginated_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_paginated_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_paginated_surface_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: None,
            map_to_image: None,
            unmap_image: None,
            source: Some(
                _cairo_paginated_surface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_paginated_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_paginated_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_paginated_surface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: Some(
                _cairo_paginated_surface_copy_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            show_page: Some(
                _cairo_paginated_surface_show_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            get_extents: Some(
                _cairo_paginated_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_paginated_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_paginated_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_paginated_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_paginated_surface_stroke
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
                _cairo_paginated_surface_fill
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
            show_glyphs: None,
            has_show_text_glyphs: Some(
                _cairo_paginated_surface_has_show_text_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            show_text_glyphs: Some(
                _cairo_paginated_surface_show_text_glyphs
                    as unsafe extern "C" fn(
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
            ),
            get_supported_mime_types: Some(
                _cairo_paginated_surface_get_supported_mime_types
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                    ) -> *mut *const libc::c_char,
            ),
            tag: Some(
                _cairo_paginated_surface_tag
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_bool_t,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> cairo_int_status_t,
            ),
        };
        init
    }
};
