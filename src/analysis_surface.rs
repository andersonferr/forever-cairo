use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_region_contains_rectangle(
        region: *const cairo_region_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_region_overlap_t;
    fn cairo_region_union(
        dst: *mut cairo_region_t,
        other: *const cairo_region_t,
    ) -> cairo_status_t;
    fn cairo_region_union_rectangle(
        dst: *mut cairo_region_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_clip_get_extents(
        clip: *const cairo_clip_t,
    ) -> *const cairo_rectangle_int_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_box_from_rectangle(
        box_0: *mut cairo_box_t,
        rectangle: *const cairo_rectangle_int_t,
    );
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_operator_bounded_by_mask(op: cairo_operator_t) -> cairo_bool_t;
    fn _cairo_operator_bounded_by_source(op: cairo_operator_t) -> cairo_bool_t;
    fn _cairo_path_fixed_fill_extents(
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        extents: *mut cairo_rectangle_int_t,
    );
    fn _cairo_path_fixed_stroke_extents(
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_glyph_device_extents(
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *const cairo_glyph_t,
        num_glyphs: libc::c_int,
        extents: *mut cairo_rectangle_int_t,
        overlap: *mut cairo_bool_t,
    ) -> cairo_status_t;
    fn _cairo_surface_has_mime_image(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_surface_attach_snapshot(
        surface: *mut cairo_surface_t,
        snapshot: *mut cairo_surface_t,
        detach_func: cairo_surface_func_t,
    );
    fn _cairo_surface_has_snapshot(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_matrix_transform_bounding_box_fixed(
        matrix: *const cairo_matrix_t,
        bbox: *mut cairo_box_t,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn _cairo_pattern_get_extents(
        pattern: *const cairo_pattern_t,
        extents: *mut cairo_rectangle_int_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_recording_surface_replay_and_create_regions(
        surface: *mut cairo_surface_t,
        surface_transform: *const cairo_matrix_t,
        target: *mut cairo_surface_t,
        surface_is_unbounded: cairo_bool_t,
    ) -> cairo_status_t;
    fn _cairo_region_init(region: *mut cairo_region_t);
    fn _cairo_region_fini(region: *mut cairo_region_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_region {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub rgn: pixman_region32_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
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
pub type _cairo_region_overlap = libc::c_uint;
pub const CAIRO_REGION_OVERLAP_PART: _cairo_region_overlap = 2;
pub const CAIRO_REGION_OVERLAP_OUT: _cairo_region_overlap = 1;
pub const CAIRO_REGION_OVERLAP_IN: _cairo_region_overlap = 0;
pub type cairo_region_overlap_t = _cairo_region_overlap;
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
pub struct cairo_analysis_surface_t {
    pub base: cairo_surface_t,
    pub target: *mut cairo_surface_t,
    pub first_op: cairo_bool_t,
    pub has_supported: cairo_bool_t,
    pub has_unsupported: cairo_bool_t,
    pub supported_region: cairo_region_t,
    pub fallback_region: cairo_region_t,
    pub page_bbox: cairo_box_t,
    pub has_ctm: cairo_bool_t,
    pub ctm: cairo_matrix_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy {
    pub base: cairo_surface_t,
    pub target: *mut cairo_surface_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
#[inline]
unsafe extern "C" fn _cairo_unbounded_rectangle_init(
    mut rect: *mut cairo_rectangle_int_t,
) {
    *rect = _cairo_unbounded_rectangle;
}
#[inline]
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_box_add_box(
    mut box_0: *mut cairo_box_t,
    mut add: *const cairo_box_t,
) {
    if (*add).p1.x < (*box_0).p1.x {
        (*box_0).p1.x = (*add).p1.x;
    }
    if (*add).p2.x > (*box_0).p2.x {
        (*box_0).p2.x = (*add).p2.x;
    }
    if (*add).p1.y < (*box_0).p1.y {
        (*box_0).p1.y = (*add).p1.y;
    }
    if (*add).p2.y > (*box_0).p2.y {
        (*box_0).p2.y = (*add).p2.y;
    }
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_recording(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_merge_status(
    mut status_a: cairo_int_status_t,
    mut status_b: cairo_int_status_t,
) -> cairo_int_status_t {
    if !(status_a as libc::c_uint
        != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status_a as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"! _cairo_int_status_is_error (status_a)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-analysis-surface.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_int_status_t _cairo_analysis_surface_merge_status(cairo_int_status_t, cairo_int_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    if !(status_b as libc::c_uint
        != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status_b as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"! _cairo_int_status_is_error (status_b)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-analysis-surface.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_int_status_t _cairo_analysis_surface_merge_status(cairo_int_status_t, cairo_int_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    if status_a as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        || status_b as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if status_a as libc::c_uint
        == CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int as libc::c_uint
        || status_b as libc::c_uint
            == CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_IMAGE_FALLBACK;
    }
    if status_a as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
        || status_b as libc::c_uint
            == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
                as libc::c_uint
    {
        return CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN;
    }
    if status_a as libc::c_uint
        == CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY as libc::c_int as libc::c_uint
        || status_b as libc::c_uint
            == CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY;
    }
    if status_a as libc::c_uint
        == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && status_b as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status_a == CAIRO_INT_STATUS_SUCCESS && status_b == CAIRO_INT_STATUS_SUCCESS\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-analysis-surface.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_int_status_t _cairo_analysis_surface_merge_status(cairo_int_status_t, cairo_int_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn proxy_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    return CAIRO_STATUS_SUCCESS;
}
static mut proxy_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_NULL as libc::c_int
                as cairo_surface_type_t,
            finish: Some(
                proxy_finish as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: None,
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
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            fill_stroke: None,
            show_glyphs: None,
            has_show_text_glyphs: None,
            show_text_glyphs: None,
            get_supported_mime_types: None,
            tag: None,
        };
        init
    }
};
unsafe extern "C" fn attach_proxy(
    mut source: *mut cairo_surface_t,
    mut target: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut proxy: *mut proxy = 0 as *mut proxy;
    proxy = (if ::std::mem::size_of::<proxy>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<proxy>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut proxy;
    if proxy.is_null() {
        return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_surface_init(
        &mut (*proxy).base,
        &proxy_backend,
        0 as *mut cairo_device_t,
        (*target).content,
        (*target).is_vector() as cairo_bool_t,
    );
    let ref mut fresh0 = (*proxy).target;
    *fresh0 = target;
    _cairo_surface_attach_snapshot(source, &mut (*proxy).base, None);
    return &mut (*proxy).base;
}
unsafe extern "C" fn detach_proxy(mut proxy: *mut cairo_surface_t) {
    cairo_surface_finish(proxy);
    cairo_surface_destroy(proxy);
}
unsafe extern "C" fn _add_operation(
    mut surface: *mut cairo_analysis_surface_t,
    mut rect: *mut cairo_rectangle_int_t,
    mut backend_status: cairo_int_status_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut bbox: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if (*rect).width == 0 as libc::c_int || (*rect).height == 0 as libc::c_int {
        if backend_status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            || backend_status as libc::c_uint
                == CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY as libc::c_int as libc::c_uint
            || backend_status as libc::c_uint
                == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
        {
            return CAIRO_INT_STATUS_SUCCESS
        } else {
            return CAIRO_INT_STATUS_IMAGE_FALLBACK
        }
    }
    _cairo_box_from_rectangle(&mut bbox, rect);
    if (*surface).has_ctm != 0 {
        let mut tx: libc::c_int = 0;
        let mut ty: libc::c_int = 0;
        if _cairo_matrix_is_integer_translation(&mut (*surface).ctm, &mut tx, &mut ty)
            != 0
        {
            (*rect).x += tx;
            (*rect).y += ty;
            tx = _cairo_fixed_from_int(tx);
            bbox.p1.x += tx;
            bbox.p2.x += tx;
            ty = _cairo_fixed_from_int(ty);
            bbox.p1.y += ty;
            bbox.p2.y += ty;
        } else {
            _cairo_matrix_transform_bounding_box_fixed(
                &mut (*surface).ctm,
                &mut bbox,
                0 as *mut cairo_bool_t,
            );
            if bbox.p1.x == bbox.p2.x || bbox.p1.y == bbox.p2.y {
                if backend_status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    || backend_status as libc::c_uint
                        == CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY as libc::c_int
                            as libc::c_uint
                    || backend_status as libc::c_uint
                        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
                {
                    return CAIRO_INT_STATUS_SUCCESS
                } else {
                    return CAIRO_INT_STATUS_IMAGE_FALLBACK
                }
            }
            _cairo_box_round_to_rectangle(&mut bbox, rect);
        }
    }
    if (*surface).first_op != 0 {
        (*surface).first_op = 0 as libc::c_int;
        (*surface).page_bbox = bbox;
    } else {
        _cairo_box_add_box(&mut (*surface).page_bbox, &mut bbox);
    }
    if cairo_region_contains_rectangle(&mut (*surface).fallback_region, rect)
        as libc::c_uint == CAIRO_REGION_OVERLAP_IN as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_IMAGE_FALLBACK;
    }
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY as libc::c_int as libc::c_uint
    {
        if cairo_region_contains_rectangle(&mut (*surface).supported_region, rect)
            as libc::c_uint == CAIRO_REGION_OVERLAP_OUT as libc::c_int as libc::c_uint
        {
            backend_status = CAIRO_INT_STATUS_SUCCESS;
        }
    }
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        (*surface).has_supported = 1 as libc::c_int;
        return cairo_region_union_rectangle(&mut (*surface).supported_region, rect)
            as cairo_int_status_t;
    }
    (*surface).has_unsupported = 1 as libc::c_int;
    status = cairo_region_union_rectangle(&mut (*surface).fallback_region, rect)
        as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_IMAGE_FALLBACK
    } else {
        return status
    };
}
unsafe extern "C" fn _analyze_recording_surface_pattern(
    mut surface: *mut cairo_analysis_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface_pattern: *const cairo_surface_pattern_t = 0
        as *const cairo_surface_pattern_t;
    let mut tmp: *mut cairo_analysis_surface_t = 0 as *mut cairo_analysis_surface_t;
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut proxy: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut analysis_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut surface_is_unbounded: cairo_bool_t = 0;
    let mut unused: cairo_bool_t = 0;
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"pattern->type == CAIRO_PATTERN_TYPE_SURFACE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-analysis-surface.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 132],
                &[libc::c_char; 132],
            >(
                b"cairo_int_status_t _analyze_recording_surface_pattern(cairo_analysis_surface_t *, const cairo_pattern_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    surface_pattern = pattern as *const cairo_surface_pattern_t;
    if (*(*surface_pattern).surface).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"surface_pattern->surface->type == CAIRO_SURFACE_TYPE_RECORDING\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-analysis-surface.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 132],
                &[libc::c_char; 132],
            >(
                b"cairo_int_status_t _analyze_recording_surface_pattern(cairo_analysis_surface_t *, const cairo_pattern_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    source = (*surface_pattern).surface;
    proxy = _cairo_surface_has_snapshot(source, &proxy_backend);
    if !proxy.is_null() {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    tmp = _cairo_analysis_surface_create((*surface).target)
        as *mut cairo_analysis_surface_t;
    if (*tmp).base.status as u64 != 0 {
        status = (*tmp).base.status as cairo_int_status_t;
    } else {
        proxy = attach_proxy(source, &mut (*tmp).base);
        p2d = (*pattern).matrix;
        status = cairo_matrix_invert(&mut p2d) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-analysis-surface.c\0" as *const u8 as *const libc::c_char,
                292 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 132],
                    &[libc::c_char; 132],
                >(
                    b"cairo_int_status_t _analyze_recording_surface_pattern(cairo_analysis_surface_t *, const cairo_pattern_t *, cairo_rectangle_int_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_analysis_surface_set_ctm(&mut (*tmp).base, &mut p2d);
        source = _cairo_surface_get_source(source, 0 as *mut cairo_rectangle_int_t);
        surface_is_unbounded = ((*pattern).extend as libc::c_uint
            == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
            || (*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint) as libc::c_int;
        status = _cairo_recording_surface_replay_and_create_regions(
            source,
            &(*pattern).matrix,
            &mut (*tmp).base,
            surface_is_unbounded,
        ) as cairo_int_status_t;
        if !(status as u64 != 0) {
            if (*source).content as libc::c_uint
                & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint == 0
                || _cairo_surface_has_mime_image(source) != 0
            {
                let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                if _cairo_surface_get_extents(source, &mut rect) != 0 {
                    let mut bbox: cairo_box_t = cairo_box_t {
                        p1: cairo_point_t { x: 0, y: 0 },
                        p2: cairo_point_t { x: 0, y: 0 },
                    };
                    _cairo_box_from_rectangle(&mut bbox, &mut rect);
                    _cairo_matrix_transform_bounding_box_fixed(
                        &mut p2d,
                        &mut bbox,
                        0 as *mut cairo_bool_t,
                    );
                    _cairo_box_round_to_rectangle(&mut bbox, &mut rect);
                    status = _add_operation(tmp, &mut rect, CAIRO_INT_STATUS_SUCCESS);
                    if status as libc::c_uint
                        == CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int as libc::c_uint
                    {
                        status = CAIRO_INT_STATUS_SUCCESS;
                    }
                    if status as u64 != 0 {
                        current_block = 11524960444988004923;
                    } else {
                        current_block = 18377268871191777778;
                    }
                } else {
                    current_block = 18377268871191777778;
                }
            } else {
                current_block = 18377268871191777778;
            }
            match current_block {
                11524960444988004923 => {}
                _ => {
                    if (*tmp).has_supported != 0 {
                        (*surface).has_supported = 1 as libc::c_int;
                        unused = cairo_region_union(
                            &mut (*surface).supported_region,
                            &mut (*tmp).supported_region,
                        ) as cairo_bool_t;
                    }
                    if (*tmp).has_unsupported != 0 {
                        (*surface).has_unsupported = 1 as libc::c_int;
                        unused = cairo_region_union(
                            &mut (*surface).fallback_region,
                            &mut (*tmp).fallback_region,
                        ) as cairo_bool_t;
                    }
                    analysis_status = (if (*tmp).has_unsupported != 0 {
                        CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int
                    } else {
                        CAIRO_INT_STATUS_SUCCESS as libc::c_int
                    }) as cairo_int_status_t;
                    if (*pattern).extend as libc::c_uint
                        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                    {
                        _cairo_unbounded_rectangle_init(extents);
                    } else {
                        status = cairo_matrix_invert(&mut (*tmp).ctm)
                            as cairo_int_status_t;
                        _cairo_matrix_transform_bounding_box_fixed(
                            &mut (*tmp).ctm,
                            &mut (*tmp).page_bbox,
                            0 as *mut cairo_bool_t,
                        );
                        _cairo_box_round_to_rectangle(&mut (*tmp).page_bbox, extents);
                    }
                }
            }
        }
        detach_proxy(proxy);
    }
    cairo_surface_destroy(&mut (*tmp).base);
    if status as u64 != 0 { return status } else { return analysis_status };
}
unsafe extern "C" fn _cairo_analysis_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    _cairo_region_fini(&mut (*surface).supported_region);
    _cairo_region_fini(&mut (*surface).fallback_region);
    cairo_surface_destroy((*surface).target);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_analysis_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    return _cairo_surface_get_extents((*surface).target, rectangle);
}
unsafe extern "C" fn _rectangle_intersect_clip(
    mut extents: *mut cairo_rectangle_int_t,
    mut clip: *const cairo_clip_t,
) {
    if !clip.is_null() {
        _cairo_rectangle_intersect(extents, _cairo_clip_get_extents(clip));
    }
}
unsafe extern "C" fn _cairo_analysis_surface_operation_extents(
    mut surface: *mut cairo_analysis_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
    mut extents: *mut cairo_rectangle_int_t,
) {
    let mut is_empty: cairo_bool_t = 0;
    is_empty = _cairo_surface_get_extents(&mut (*surface).base, extents);
    if _cairo_operator_bounded_by_source(op) != 0 {
        let mut source_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        _cairo_pattern_get_extents(
            source,
            &mut source_extents,
            (*(*surface).target).is_vector() as cairo_bool_t,
        );
        _cairo_rectangle_intersect(extents, &mut source_extents);
    }
    _rectangle_intersect_clip(extents, clip);
}
unsafe extern "C" fn _cairo_analysis_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ((*(*(*surface).target).backend).paint).is_none() {
        backend_status = CAIRO_INT_STATUS_UNSUPPORTED;
    } else {
        backend_status = ((*(*(*surface).target).backend).paint)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, op, source, clip);
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    }
    _cairo_analysis_surface_operation_extents(surface, op, source, clip, &mut extents);
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
    {
        let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        backend_status = _analyze_recording_surface_pattern(
            surface,
            source,
            &mut rec_extents,
        );
        _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
    }
    return _add_operation(surface, &mut extents, backend_status);
}
unsafe extern "C" fn _cairo_analysis_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ((*(*(*surface).target).backend).mask).is_none() {
        backend_status = CAIRO_INT_STATUS_UNSUPPORTED;
    } else {
        backend_status = ((*(*(*surface).target).backend).mask)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, op, source, mask, clip);
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    }
    _cairo_analysis_surface_operation_extents(surface, op, source, clip, &mut extents);
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
    {
        let mut backend_source_status: cairo_int_status_t = CAIRO_STATUS_SUCCESS
            as libc::c_int as cairo_int_status_t;
        let mut backend_mask_status: cairo_int_status_t = CAIRO_STATUS_SUCCESS
            as libc::c_int as cairo_int_status_t;
        let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        if (*source).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        {
            let mut src_surface: *mut cairo_surface_t = (*(source
                as *mut cairo_surface_pattern_t))
                .surface;
            src_surface = _cairo_surface_get_source(
                src_surface,
                0 as *mut cairo_rectangle_int_t,
            );
            if _cairo_surface_is_recording(src_surface) != 0 {
                backend_source_status = _analyze_recording_surface_pattern(
                    surface,
                    source,
                    &mut rec_extents,
                );
                if backend_source_status as libc::c_uint
                    != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    && (backend_source_status as libc::c_uint)
                        < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
                {
                    return backend_source_status;
                }
                _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
            }
        }
        if (*mask).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        {
            let mut mask_surface: *mut cairo_surface_t = (*(mask
                as *mut cairo_surface_pattern_t))
                .surface;
            mask_surface = _cairo_surface_get_source(
                mask_surface,
                0 as *mut cairo_rectangle_int_t,
            );
            if _cairo_surface_is_recording(mask_surface) != 0 {
                backend_mask_status = _analyze_recording_surface_pattern(
                    surface,
                    mask,
                    &mut rec_extents,
                );
                if backend_mask_status as libc::c_uint
                    != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    && (backend_mask_status as libc::c_uint)
                        < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
                {
                    return backend_mask_status;
                }
                _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
            }
        }
        backend_status = _cairo_analysis_surface_merge_status(
            backend_source_status,
            backend_mask_status,
        );
    }
    if _cairo_operator_bounded_by_mask(op) != 0 {
        let mut mask_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        _cairo_pattern_get_extents(
            mask,
            &mut mask_extents,
            (*(*surface).target).is_vector() as cairo_bool_t,
        );
        _cairo_rectangle_intersect(&mut extents, &mut mask_extents);
    }
    return _add_operation(surface, &mut extents, backend_status);
}
unsafe extern "C" fn _cairo_analysis_surface_stroke(
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
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ((*(*(*surface).target).backend).stroke).is_none() {
        backend_status = CAIRO_INT_STATUS_UNSUPPORTED;
    } else {
        backend_status = ((*(*(*surface).target).backend).stroke)
            .expect(
                "non-null function pointer",
            )(
            (*surface).target as *mut libc::c_void,
            op,
            source,
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            antialias,
            clip,
        );
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    }
    _cairo_analysis_surface_operation_extents(surface, op, source, clip, &mut extents);
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
    {
        let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        backend_status = _analyze_recording_surface_pattern(
            surface,
            source,
            &mut rec_extents,
        );
        _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
    }
    if _cairo_operator_bounded_by_mask(op) != 0 {
        let mut mask_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
        status = _cairo_path_fixed_stroke_extents(
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            &mut mask_extents,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_rectangle_intersect(&mut extents, &mut mask_extents);
    }
    return _add_operation(surface, &mut extents, backend_status);
}
unsafe extern "C" fn _cairo_analysis_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ((*(*(*surface).target).backend).fill).is_none() {
        backend_status = CAIRO_INT_STATUS_UNSUPPORTED;
    } else {
        backend_status = ((*(*(*surface).target).backend).fill)
            .expect(
                "non-null function pointer",
            )(
            (*surface).target as *mut libc::c_void,
            op,
            source,
            path,
            fill_rule,
            tolerance,
            antialias,
            clip,
        );
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    }
    _cairo_analysis_surface_operation_extents(surface, op, source, clip, &mut extents);
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
    {
        let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        backend_status = _analyze_recording_surface_pattern(
            surface,
            source,
            &mut rec_extents,
        );
        _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
    }
    if _cairo_operator_bounded_by_mask(op) != 0 {
        let mut mask_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        _cairo_path_fixed_fill_extents(path, fill_rule, tolerance, &mut mask_extents);
        _cairo_rectangle_intersect(&mut extents, &mut mask_extents);
    }
    return _add_operation(surface, &mut extents, backend_status);
}
unsafe extern "C" fn _cairo_analysis_surface_show_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut glyph_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ((*(*(*surface).target).backend).show_glyphs).is_some() {
        backend_status = ((*(*(*surface).target).backend).show_glyphs)
            .expect(
                "non-null function pointer",
            )(
            (*surface).target as *mut libc::c_void,
            op,
            source,
            glyphs,
            num_glyphs,
            scaled_font,
            clip,
        );
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    } else if ((*(*(*surface).target).backend).show_text_glyphs).is_some() {
        backend_status = ((*(*(*surface).target).backend).show_text_glyphs)
            .expect(
                "non-null function pointer",
            )(
            (*surface).target as *mut libc::c_void,
            op,
            source,
            0 as *const libc::c_char,
            0 as libc::c_int,
            glyphs,
            num_glyphs,
            0 as *const cairo_text_cluster_t,
            0 as libc::c_int,
            0 as cairo_text_cluster_flags_t,
            scaled_font,
            clip,
        );
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    } else {
        backend_status = CAIRO_INT_STATUS_UNSUPPORTED;
    }
    _cairo_analysis_surface_operation_extents(surface, op, source, clip, &mut extents);
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
    {
        let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        backend_status = _analyze_recording_surface_pattern(
            surface,
            source,
            &mut rec_extents,
        );
        _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
    }
    if _cairo_operator_bounded_by_mask(op) != 0 {
        status = _cairo_scaled_font_glyph_device_extents(
            scaled_font,
            glyphs,
            num_glyphs,
            &mut glyph_extents,
            0 as *mut cairo_bool_t,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_rectangle_intersect(&mut extents, &mut glyph_extents);
    }
    return _add_operation(surface, &mut extents, backend_status);
}
unsafe extern "C" fn _cairo_analysis_surface_has_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    return cairo_surface_has_show_text_glyphs((*surface).target);
}
unsafe extern "C" fn _cairo_analysis_surface_show_text_glyphs(
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
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut glyph_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    backend_status = CAIRO_INT_STATUS_UNSUPPORTED;
    if ((*(*(*surface).target).backend).show_text_glyphs).is_some() {
        backend_status = ((*(*(*surface).target).backend).show_text_glyphs)
            .expect(
                "non-null function pointer",
            )(
            (*surface).target as *mut libc::c_void,
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
        );
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    }
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        && ((*(*(*surface).target).backend).show_glyphs).is_some()
    {
        backend_status = ((*(*(*surface).target).backend).show_glyphs)
            .expect(
                "non-null function pointer",
            )(
            (*surface).target as *mut libc::c_void,
            op,
            source,
            glyphs,
            num_glyphs,
            scaled_font,
            clip,
        );
        if backend_status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (backend_status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
        {
            return backend_status;
        }
    }
    _cairo_analysis_surface_operation_extents(surface, op, source, clip, &mut extents);
    if backend_status as libc::c_uint
        == CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN as libc::c_int
            as libc::c_uint
    {
        let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        backend_status = _analyze_recording_surface_pattern(
            surface,
            source,
            &mut rec_extents,
        );
        _cairo_rectangle_intersect(&mut extents, &mut rec_extents);
    }
    if _cairo_operator_bounded_by_mask(op) != 0 {
        status = _cairo_scaled_font_glyph_device_extents(
            scaled_font,
            glyphs,
            num_glyphs,
            &mut glyph_extents,
            0 as *mut cairo_bool_t,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_rectangle_intersect(&mut extents, &mut glyph_extents);
    }
    return _add_operation(surface, &mut extents, backend_status);
}
unsafe extern "C" fn _cairo_analysis_surface_tag(
    mut abstract_surface: *mut libc::c_void,
    mut begin: cairo_bool_t,
    mut tag_name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    let mut backend_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    backend_status = CAIRO_INT_STATUS_SUCCESS;
    if ((*(*(*surface).target).backend).tag).is_some() {
        backend_status = ((*(*(*surface).target).backend).tag)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, begin, tag_name, attributes);
        if backend_status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            (*surface).has_supported = 1 as libc::c_int;
        }
    }
    return backend_status;
}
static mut cairo_analysis_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_ANALYSIS as libc::c_int
                as cairo_surface_type_t,
            finish: Some(
                _cairo_analysis_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: None,
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
            get_extents: Some(
                _cairo_analysis_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_analysis_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_analysis_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_analysis_surface_stroke
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
                _cairo_analysis_surface_fill
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
                _cairo_analysis_surface_show_glyphs
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
            has_show_text_glyphs: Some(
                _cairo_analysis_surface_has_show_text_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            show_text_glyphs: Some(
                _cairo_analysis_surface_show_text_glyphs
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
            get_supported_mime_types: None,
            tag: Some(
                _cairo_analysis_surface_tag
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_create(
    mut target: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_analysis_surface_t = 0 as *mut cairo_analysis_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = (*target).status;
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    surface = (if ::std::mem::size_of::<cairo_analysis_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_analysis_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_analysis_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_analysis_surface_backend,
        0 as *mut cairo_device_t,
        CAIRO_CONTENT_COLOR_ALPHA,
        (*target).is_vector() as cairo_bool_t,
    );
    cairo_matrix_init_identity(&mut (*surface).ctm);
    (*surface).has_ctm = 0 as libc::c_int;
    let ref mut fresh1 = (*surface).target;
    *fresh1 = cairo_surface_reference(target);
    (*surface).first_op = 1 as libc::c_int;
    (*surface).has_supported = 0 as libc::c_int;
    (*surface).has_unsupported = 0 as libc::c_int;
    _cairo_region_init(&mut (*surface).supported_region);
    _cairo_region_init(&mut (*surface).fallback_region);
    (*surface).page_bbox.p1.x = 0 as libc::c_int;
    (*surface).page_bbox.p1.y = 0 as libc::c_int;
    (*surface).page_bbox.p2.x = 0 as libc::c_int;
    (*surface).page_bbox.p2.y = 0 as libc::c_int;
    return &mut (*surface).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_set_ctm(
    mut abstract_surface: *mut cairo_surface_t,
    mut ctm: *const cairo_matrix_t,
) {
    let mut surface: *mut cairo_analysis_surface_t = 0 as *mut cairo_analysis_surface_t;
    if (*abstract_surface).status as u64 != 0 {
        return;
    }
    surface = abstract_surface as *mut cairo_analysis_surface_t;
    (*surface).ctm = *ctm;
    (*surface)
        .has_ctm = (_cairo_matrix_is_identity(&mut (*surface).ctm) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_get_ctm(
    mut abstract_surface: *mut cairo_surface_t,
    mut ctm: *mut cairo_matrix_t,
) {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    *ctm = (*surface).ctm;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_get_supported(
    mut abstract_surface: *mut cairo_surface_t,
) -> *mut cairo_region_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    return &mut (*surface).supported_region;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_get_unsupported(
    mut abstract_surface: *mut cairo_surface_t,
) -> *mut cairo_region_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    return &mut (*surface).fallback_region;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_has_supported(
    mut abstract_surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    return (*surface).has_supported;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_has_unsupported(
    mut abstract_surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    return (*surface).has_unsupported;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_analysis_surface_get_bounding_box(
    mut abstract_surface: *mut cairo_surface_t,
    mut bbox: *mut cairo_box_t,
) {
    let mut surface: *mut cairo_analysis_surface_t = abstract_surface
        as *mut cairo_analysis_surface_t;
    *bbox = (*surface).page_bbox;
}
unsafe extern "C" fn _paint_return_success(
    mut surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _mask_return_success(
    mut surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _stroke_return_success(
    mut surface: *mut libc::c_void,
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
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill_return_success(
    mut surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _show_glyphs_return_success(
    mut surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    return CAIRO_INT_STATUS_SUCCESS;
}
static mut cairo_null_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_NULL as libc::c_int
                as cairo_surface_type_t,
            finish: None,
            create_context: None,
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
                _paint_return_success
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _mask_return_success
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _stroke_return_success
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
                _fill_return_success
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
                _show_glyphs_return_success
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_null_surface_create(
    mut content: cairo_content_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    surface = (if ::std::mem::size_of::<cairo_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        surface,
        &cairo_null_surface_backend,
        0 as *mut cairo_device_t,
        content,
        1 as libc::c_int,
    );
    return surface;
}
