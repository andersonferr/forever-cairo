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
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_matrix_transform_bounding_box_fixed(
        matrix: *const cairo_matrix_t,
        bbox: *mut cairo_box_t,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_scaled_font_glyph_path(
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *const cairo_glyph_t,
        num_glyphs: libc::c_int,
        path: *mut cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_polygon_to_traps(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        traps: *mut cairo_traps_t,
    ) -> cairo_int_status_t;
    fn cairo_scaled_font_reference(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_flush(surface: *mut cairo_surface_t);
    fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    static _cairo_empty_rectangle: cairo_rectangle_int_t;
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_init_copy(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_stroke_style_fini(style: *mut cairo_stroke_style_t);
    fn _cairo_stroke_style_init_copy(
        style: *mut cairo_stroke_style_t,
        other: *const cairo_stroke_style_t,
    ) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_clip_copy(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_surface_set_error(
        surface: *mut cairo_surface_t,
        status: cairo_int_status_t,
    ) -> cairo_int_status_t;
    fn _cairo_image_analyze_transparency(
        image: *mut cairo_image_surface_t,
    ) -> cairo_image_transparency_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_path_fixed_append(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
        tx: cairo_fixed_t,
        ty: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_clip_equal(
        clip_a: *const cairo_clip_t,
        clip_b: *const cairo_clip_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_equal(
        a: *const cairo_path_fixed_t,
        b: *const cairo_path_fixed_t,
    ) -> cairo_bool_t;
    fn _cairo_box_from_rectangle(
        box_0: *mut cairo_box_t,
        rectangle: *const cairo_rectangle_int_t,
    );
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_surface_attach_snapshot(
        surface: *mut cairo_surface_t,
        snapshot: *mut cairo_surface_t,
        detach_func: cairo_surface_func_t,
    );
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
    fn _cairo_surface_default_source(
        surface: *mut libc::c_void,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_create_with_content(
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_has_snapshot(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_analysis_surface_create(
        target: *mut cairo_surface_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_analysis_surface_set_ctm(
        surface: *mut cairo_surface_t,
        ctm: *const cairo_matrix_t,
    );
    fn _cairo_analysis_surface_get_bounding_box(
        surface: *mut cairo_surface_t,
        bbox: *mut cairo_box_t,
    );
    fn _cairo_null_surface_create(content: cairo_content_t) -> *mut cairo_surface_t;
    fn _cairo_composite_rectangles_init_for_paint(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_mask(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_stroke(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_fill(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_fini(extents: *mut cairo_composite_rectangles_t);
    fn _cairo_composite_rectangles_init_for_glyphs(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clip: *const cairo_clip_t,
        overlap: *mut cairo_bool_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_can_reduce_clip(
        composite: *mut cairo_composite_rectangles_t,
        clip: *mut cairo_clip_t,
    ) -> cairo_bool_t;
    fn _cairo_pattern_init_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_is_clear(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_pattern_is_opaque(
        pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_pattern_is_opaque_solid(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_pattern_init_snapshot(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_raster_source_pattern_acquire(
        abstract_pattern: *const cairo_pattern_t,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_raster_source_pattern_release(
        abstract_pattern: *const cairo_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_surface_wrapper_init(
        wrapper: *mut cairo_surface_wrapper_t,
        target: *mut cairo_surface_t,
    );
    fn _cairo_surface_wrapper_intersect_extents(
        wrapper: *mut cairo_surface_wrapper_t,
        extents: *const cairo_rectangle_int_t,
    );
    fn _cairo_surface_wrapper_set_inverse_transform(
        wrapper: *mut cairo_surface_wrapper_t,
        transform: *const cairo_matrix_t,
    );
    fn _cairo_surface_wrapper_set_clip(
        wrapper: *mut cairo_surface_wrapper_t,
        clip: *const cairo_clip_t,
    );
    fn _cairo_surface_wrapper_set_foreground_color(
        wrapper: *mut cairo_surface_wrapper_t,
        color: *const cairo_color_t,
    );
    fn _cairo_surface_wrapper_fini(wrapper: *mut cairo_surface_wrapper_t);
    fn _cairo_surface_wrapper_paint(
        wrapper: *mut cairo_surface_wrapper_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_mask(
        wrapper: *mut cairo_surface_wrapper_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_stroke(
        wrapper: *mut cairo_surface_wrapper_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_fill_stroke(
        wrapper: *mut cairo_surface_wrapper_t,
        fill_op: cairo_operator_t,
        fill_source: *const cairo_pattern_t,
        fill_rule: cairo_fill_rule_t,
        fill_tolerance: libc::c_double,
        fill_antialias: cairo_antialias_t,
        path: *const cairo_path_fixed_t,
        stroke_op: cairo_operator_t,
        stroke_source: *const cairo_pattern_t,
        stroke_style: *const cairo_stroke_style_t,
        stroke_ctm: *const cairo_matrix_t,
        stroke_ctm_inverse: *const cairo_matrix_t,
        stroke_tolerance: libc::c_double,
        stroke_antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_fill(
        wrapper: *mut cairo_surface_wrapper_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_show_text_glyphs(
        wrapper: *mut cairo_surface_wrapper_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        glyphs: *const cairo_glyph_t,
        num_glyphs: libc::c_int,
        clusters: *const cairo_text_cluster_t,
        num_clusters: libc::c_int,
        cluster_flags: cairo_text_cluster_flags_t,
        scaled_font: *mut cairo_scaled_font_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_tag(
        wrapper: *mut cairo_surface_wrapper_t,
        begin: cairo_bool_t,
        tag_name: *const libc::c_char,
        attributes: *const libc::c_char,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_get_target_extents(
        wrapper: *mut cairo_surface_wrapper_t,
        surface_is_unbounded: cairo_bool_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_traps_init(traps: *mut cairo_traps_t);
    fn _cairo_traps_fini(traps: *mut cairo_traps_t);
    fn _cairo_traps_path(
        traps: *const cairo_traps_t,
        path: *mut cairo_path_fixed_t,
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
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
pub type cairo_recording_surface_t = _cairo_recording_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_recording_surface {
    pub base: cairo_surface_t,
    pub extents_pixels: cairo_rectangle_t,
    pub extents: cairo_rectangle_int_t,
    pub unbounded: cairo_bool_t,
    pub commands: cairo_array_t,
    pub indices: *mut libc::c_uint,
    pub num_indices: libc::c_uint,
    pub optimize_clears: cairo_bool_t,
    pub has_bilevel_alpha: cairo_bool_t,
    pub has_only_op_over: cairo_bool_t,
    pub bbtree: bbtree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bbtree {
    pub extents: cairo_box_t,
    pub left: *mut bbtree,
    pub right: *mut bbtree,
    pub chain: *mut cairo_command_header_t,
}
pub type cairo_command_header_t = _cairo_command_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_header {
    pub type_0: cairo_command_type_t,
    pub region: cairo_recording_region_type_t,
    pub op: cairo_operator_t,
    pub extents: cairo_rectangle_int_t,
    pub clip: *mut cairo_clip_t,
    pub index: libc::c_int,
    pub chain: *mut _cairo_command_header,
}
pub type cairo_recording_region_type_t = libc::c_uint;
pub const CAIRO_RECORDING_REGION_IMAGE_FALLBACK: cairo_recording_region_type_t = 2;
pub const CAIRO_RECORDING_REGION_NATIVE: cairo_recording_region_type_t = 1;
pub const CAIRO_RECORDING_REGION_ALL: cairo_recording_region_type_t = 0;
pub type cairo_command_type_t = libc::c_uint;
pub const CAIRO_COMMAND_TAG: cairo_command_type_t = 5;
pub const CAIRO_COMMAND_SHOW_TEXT_GLYPHS: cairo_command_type_t = 4;
pub const CAIRO_COMMAND_FILL: cairo_command_type_t = 3;
pub const CAIRO_COMMAND_STROKE: cairo_command_type_t = 2;
pub const CAIRO_COMMAND_MASK: cairo_command_type_t = 1;
pub const CAIRO_COMMAND_PAINT: cairo_command_type_t = 0;
pub type cairo_command_t = _cairo_command;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _cairo_command {
    pub header: cairo_command_header_t,
    pub paint: cairo_command_paint_t,
    pub mask: cairo_command_mask_t,
    pub stroke: cairo_command_stroke_t,
    pub fill: cairo_command_fill_t,
    pub show_text_glyphs: cairo_command_show_text_glyphs_t,
    pub tag: cairo_command_tag_t,
}
pub type cairo_command_tag_t = _cairo_command_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_tag {
    pub header: cairo_command_header_t,
    pub begin: cairo_bool_t,
    pub tag_name: *mut libc::c_char,
    pub attributes: *mut libc::c_char,
}
pub type cairo_command_show_text_glyphs_t = _cairo_command_show_text_glyphs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_show_text_glyphs {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub utf8: *mut libc::c_char,
    pub utf8_len: libc::c_int,
    pub glyphs: *mut cairo_glyph_t,
    pub num_glyphs: libc::c_uint,
    pub clusters: *mut cairo_text_cluster_t,
    pub num_clusters: libc::c_int,
    pub cluster_flags: cairo_text_cluster_flags_t,
    pub scaled_font: *mut cairo_scaled_font_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_pattern_union_t {
    pub base: cairo_pattern_t,
    pub solid: cairo_solid_pattern_t,
    pub surface: cairo_surface_pattern_t,
    pub gradient: cairo_gradient_pattern_union_t,
    pub mesh: cairo_mesh_pattern_t,
    pub raster_source: cairo_raster_source_pattern_t,
}
pub type cairo_raster_source_pattern_t = _cairo_raster_source_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_raster_source_pattern {
    pub base: cairo_pattern_t,
    pub content: cairo_content_t,
    pub extents: cairo_rectangle_int_t,
    pub acquire: cairo_raster_source_acquire_func_t,
    pub release: cairo_raster_source_release_func_t,
    pub snapshot: cairo_raster_source_snapshot_func_t,
    pub copy: cairo_raster_source_copy_func_t,
    pub finish: cairo_raster_source_finish_func_t,
    pub user_data: *mut libc::c_void,
}
pub type cairo_raster_source_finish_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> (),
>;
pub type cairo_raster_source_copy_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *const cairo_pattern_t,
    ) -> cairo_status_t,
>;
pub type cairo_raster_source_snapshot_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> cairo_status_t,
>;
pub type cairo_raster_source_release_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
    ) -> (),
>;
pub type cairo_raster_source_acquire_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
        *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t,
>;
pub type cairo_mesh_pattern_t = _cairo_mesh_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_pattern {
    pub base: cairo_pattern_t,
    pub patches: cairo_array_t,
    pub current_patch: *mut cairo_mesh_patch_t,
    pub current_side: libc::c_int,
    pub has_control_point: [cairo_bool_t; 4],
    pub has_color: [cairo_bool_t; 4],
}
pub type cairo_mesh_patch_t = _cairo_mesh_patch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_patch {
    pub points: [[cairo_point_double_t; 4]; 4],
    pub colors: [cairo_color_t; 4],
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_gradient_pattern_union_t {
    pub base: cairo_gradient_pattern_t,
    pub linear: cairo_linear_pattern_t,
    pub radial: cairo_radial_pattern_t,
}
pub type cairo_radial_pattern_t = _cairo_radial_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_radial_pattern {
    pub base: cairo_gradient_pattern_t,
    pub cd1: cairo_circle_double_t,
    pub cd2: cairo_circle_double_t,
}
pub type cairo_circle_double_t = _cairo_circle_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_circle_double {
    pub center: cairo_point_double_t,
    pub radius: libc::c_double,
}
pub type cairo_gradient_pattern_t = _cairo_gradient_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_pattern {
    pub base: cairo_pattern_t,
    pub n_stops: libc::c_uint,
    pub stops_size: libc::c_uint,
    pub stops: *mut cairo_gradient_stop_t,
    pub stops_embedded: [cairo_gradient_stop_t; 2],
}
pub type cairo_gradient_stop_t = _cairo_gradient_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_stop {
    pub offset: libc::c_double,
    pub color: cairo_color_stop_t,
}
pub type cairo_color_stop_t = _cairo_color_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_color_stop {
    pub red: libc::c_double,
    pub green: libc::c_double,
    pub blue: libc::c_double,
    pub alpha: libc::c_double,
    pub red_short: uint16_t,
    pub green_short: uint16_t,
    pub blue_short: uint16_t,
    pub alpha_short: uint16_t,
}
pub type uint16_t = __uint16_t;
pub type cairo_linear_pattern_t = _cairo_linear_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_linear_pattern {
    pub base: cairo_gradient_pattern_t,
    pub pd1: cairo_point_double_t,
    pub pd2: cairo_point_double_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_command_fill_t = _cairo_command_fill;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_fill {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub path: cairo_path_fixed_t,
    pub fill_rule: cairo_fill_rule_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
}
pub type cairo_command_stroke_t = _cairo_command_stroke;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_stroke {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub path: cairo_path_fixed_t,
    pub style: cairo_stroke_style_t,
    pub ctm: cairo_matrix_t,
    pub ctm_inverse: cairo_matrix_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
}
pub type cairo_command_mask_t = _cairo_command_mask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_mask {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub mask: cairo_pattern_union_t,
}
pub type cairo_command_paint_t = _cairo_command_paint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_paint {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
}
pub type cairo_composite_rectangles_t = _cairo_composite_rectangles;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_composite_rectangles {
    pub surface: *mut cairo_surface_t,
    pub op: cairo_operator_t,
    pub source: cairo_rectangle_int_t,
    pub mask: cairo_rectangle_int_t,
    pub destination: cairo_rectangle_int_t,
    pub bounded: cairo_rectangle_int_t,
    pub unbounded: cairo_rectangle_int_t,
    pub is_bounded: uint32_t,
    pub source_sample_area: cairo_rectangle_int_t,
    pub mask_sample_area: cairo_rectangle_int_t,
    pub source_pattern: cairo_pattern_union_t,
    pub mask_pattern: cairo_pattern_union_t,
    pub original_source_pattern: *const cairo_pattern_t,
    pub original_mask_pattern: *const cairo_pattern_t,
    pub clip: *mut cairo_clip_t,
}
pub type cairo_recording_surface_replay_params_t = _cairo_recording_surface_replay_params;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_recording_surface_replay_params {
    pub surface_extents: *const cairo_rectangle_int_t,
    pub surface_transform: *const cairo_matrix_t,
    pub target: *mut cairo_surface_t,
    pub target_clip: *const cairo_clip_t,
    pub surface_is_unbounded: cairo_bool_t,
    pub type_0: cairo_recording_replay_type_t,
    pub region: cairo_recording_region_type_t,
    pub foreground_color: *const cairo_color_t,
}
pub type cairo_recording_replay_type_t = libc::c_uint;
pub const CAIRO_RECORDING_CREATE_REGIONS: cairo_recording_replay_type_t = 1;
pub const CAIRO_RECORDING_REPLAY: cairo_recording_replay_type_t = 0;
pub type cairo_surface_wrapper_t = _cairo_surface_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_wrapper {
    pub target: *mut cairo_surface_t,
    pub transform: cairo_matrix_t,
    pub has_extents: cairo_bool_t,
    pub extents: cairo_rectangle_int_t,
    pub clip: *const cairo_clip_t,
    pub foreground_source: *mut cairo_pattern_t,
    pub needs_transform: cairo_bool_t,
}
pub const CAIRO_IMAGE_HAS_ALPHA: _cairo_image_transparency = 2;
pub type cairo_image_transparency_t = _cairo_image_transparency;
pub type _cairo_image_transparency = libc::c_uint;
pub const CAIRO_IMAGE_UNKNOWN: _cairo_image_transparency = 3;
pub const CAIRO_IMAGE_HAS_BILEVEL_ALPHA: _cairo_image_transparency = 1;
pub const CAIRO_IMAGE_IS_OPAQUE: _cairo_image_transparency = 0;
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_surface_snapshot_t = _cairo_surface_snapshot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_snapshot {
    pub base: cairo_surface_t,
    pub mutex: cairo_mutex_t,
    pub target: *mut cairo_surface_t,
    pub clone: *mut cairo_surface_t,
}
pub const CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT: _cairo_internal_surface_type = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy {
    pub base: cairo_surface_t,
    pub image: *mut cairo_surface_t,
}
pub const CAIRO_INTERNAL_SURFACE_TYPE_NULL: _cairo_internal_surface_type = 4103;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_traps {
    pub status: cairo_status_t,
    pub bounds: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    #[bitfield(name = "maybe_region", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_intersections", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_rectilinear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "is_rectangular", ty = "libc::c_uint", bits = "3..=3")]
    pub maybe_region_has_intersections_is_rectilinear_is_rectangular: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub num_traps: libc::c_int,
    pub traps_size: libc::c_int,
    pub traps: *mut cairo_trapezoid_t,
    pub traps_embedded: [cairo_trapezoid_t; 16],
}
pub type cairo_trapezoid_t = _cairo_trapezoid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_trapezoid {
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub left: cairo_line_t,
    pub right: cairo_line_t,
}
pub type cairo_line_t = _cairo_line;
pub type cairo_traps_t = _cairo_traps;
pub type _cairo_internal_surface_type = libc::c_uint;
pub const CAIRO_INTERNAL_SURFACE_TYPE_QUARTZ_SNAPSHOT: _cairo_internal_surface_type = 4105;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TYPE3_GLYPH: _cairo_internal_surface_type = 4104;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_WRAPPING: _cairo_internal_surface_type = 4102;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_PAGINATED: _cairo_internal_surface_type = 4101;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_FALLBACK: _cairo_internal_surface_type = 4100;
pub const CAIRO_INTERNAL_SURFACE_TYPE_OBSERVER: _cairo_internal_surface_type = 4099;
pub const CAIRO_INTERNAL_SURFACE_TYPE_ANALYSIS: _cairo_internal_surface_type = 4098;
pub const CAIRO_INTERNAL_SURFACE_TYPE_PAGINATED: _cairo_internal_surface_type = 4097;
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
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline]
unsafe extern "C" fn _cairo_rectangle_intersects(
    mut dst: *const cairo_rectangle_int_t,
    mut src: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    return !((*src).x >= (*dst).x + (*dst).width || (*src).x + (*src).width <= (*dst).x
        || (*src).y >= (*dst).y + (*dst).height || (*src).y + (*src).height <= (*dst).y)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_combsort_newgap(mut gap: libc::c_uint) -> libc::c_uint {
    gap = (10 as libc::c_int as libc::c_uint)
        .wrapping_mul(gap)
        .wrapping_div(13 as libc::c_int as libc::c_uint);
    if gap == 9 as libc::c_int as libc::c_uint
        || gap == 10 as libc::c_int as libc::c_uint
    {
        gap = 11 as libc::c_int as libc::c_uint;
    }
    if gap < 1 as libc::c_int as libc::c_uint {
        gap = 1 as libc::c_int as libc::c_uint;
    }
    return gap;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_recording(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_reference(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
        == -(1 as libc::c_int))
    {
        ::std::intrinsics::atomic_xadd(
            &mut (*surface).ref_count.ref_count,
            1 as libc::c_int,
        );
    }
    return surface;
}
#[inline]
unsafe extern "C" fn _cairo_surface_snapshot_get_target(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut snapshot: *mut cairo_surface_snapshot_t = surface
        as *mut cairo_surface_snapshot_t;
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    pthread_mutex_lock(&mut (*snapshot).mutex);
    target = _cairo_surface_reference((*snapshot).target);
    pthread_mutex_unlock(&mut (*snapshot).mutex);
    return target;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_snapshot(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT as libc::c_int as cairo_surface_type_t
            as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_wrapper_has_fill_stroke(
    mut wrapper: *mut cairo_surface_wrapper_t,
) -> cairo_bool_t {
    return ((*(*(*wrapper).target).backend).fill_stroke).is_some() as libc::c_int;
}
unsafe extern "C" fn bbtree_left_or_right(
    mut bbt: *mut bbtree,
    mut box_0: *const cairo_box_t,
) -> libc::c_int {
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    if !((*bbt).left).is_null() {
        let mut e: *mut cairo_box_t = &mut (*(*bbt).left).extents;
        let mut b: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        b.p1.x = if (*e).p1.x < (*box_0).p1.x { (*e).p1.x } else { (*box_0).p1.x };
        b.p1.y = if (*e).p1.y < (*box_0).p1.y { (*e).p1.y } else { (*box_0).p1.y };
        b.p2.x = if (*e).p2.x > (*box_0).p2.x { (*e).p2.x } else { (*box_0).p2.x };
        b.p2.y = if (*e).p2.y > (*box_0).p2.y { (*e).p2.y } else { (*box_0).p2.y };
        left = _cairo_fixed_integer_part(b.p2.x - b.p1.x)
            * _cairo_fixed_integer_part(b.p2.y - b.p1.y);
        left
            -= _cairo_fixed_integer_part((*e).p2.x - (*e).p1.x)
                * _cairo_fixed_integer_part((*e).p2.y - (*e).p1.y);
    } else {
        left = 0 as libc::c_int;
    }
    if !((*bbt).right).is_null() {
        let mut e_0: *mut cairo_box_t = &mut (*(*bbt).right).extents;
        let mut b_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        b_0.p1.x = if (*e_0).p1.x < (*box_0).p1.x { (*e_0).p1.x } else { (*box_0).p1.x };
        b_0.p1.y = if (*e_0).p1.y < (*box_0).p1.y { (*e_0).p1.y } else { (*box_0).p1.y };
        b_0.p2.x = if (*e_0).p2.x > (*box_0).p2.x { (*e_0).p2.x } else { (*box_0).p2.x };
        b_0.p2.y = if (*e_0).p2.y > (*box_0).p2.y { (*e_0).p2.y } else { (*box_0).p2.y };
        right = _cairo_fixed_integer_part(b_0.p2.x - b_0.p1.x)
            * _cairo_fixed_integer_part(b_0.p2.y - b_0.p1.y);
        right
            -= _cairo_fixed_integer_part((*e_0).p2.x - (*e_0).p1.x)
                * _cairo_fixed_integer_part((*e_0).p2.y - (*e_0).p1.y);
    } else {
        right = 0 as libc::c_int;
    }
    return (left <= right) as libc::c_int;
}
unsafe extern "C" fn bbtree_new(
    mut box_0: *const cairo_box_t,
    mut chain: *mut cairo_command_header_t,
) -> *mut bbtree {
    let mut bbt: *mut bbtree = (if ::std::mem::size_of::<bbtree>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<bbtree>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut bbtree;
    if bbt.is_null() {
        return 0 as *mut bbtree;
    }
    (*bbt).extents = *box_0;
    let ref mut fresh2 = (*bbt).right;
    *fresh2 = 0 as *mut bbtree;
    let ref mut fresh3 = (*bbt).left;
    *fresh3 = *fresh2;
    let ref mut fresh4 = (*bbt).chain;
    *fresh4 = chain;
    return bbt;
}
unsafe extern "C" fn bbtree_init(
    mut bbt: *mut bbtree,
    mut header: *mut cairo_command_header_t,
) {
    _cairo_box_from_rectangle(&mut (*bbt).extents, &mut (*header).extents);
    let ref mut fresh5 = (*bbt).chain;
    *fresh5 = header;
}
unsafe extern "C" fn bbtree_add(
    mut bbt: *mut bbtree,
    mut header: *mut cairo_command_header_t,
    mut box_0: *const cairo_box_t,
) -> cairo_status_t {
    if (*box_0).p1.x < (*bbt).extents.p1.x || (*box_0).p1.y < (*bbt).extents.p1.y
        || (*box_0).p2.x > (*bbt).extents.p2.x || (*box_0).p2.y > (*bbt).extents.p2.y
    {
        if !((*bbt).chain).is_null() {
            if bbtree_left_or_right(bbt, &mut (*bbt).extents) != 0 {
                if ((*bbt).left).is_null() {
                    let ref mut fresh6 = (*bbt).left;
                    *fresh6 = bbtree_new(&mut (*bbt).extents, (*bbt).chain);
                    if ((*bbt).left).is_null() {
                        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    }
                } else {
                    bbtree_add((*bbt).left, (*bbt).chain, &mut (*bbt).extents);
                }
            } else if ((*bbt).right).is_null() {
                let ref mut fresh7 = (*bbt).right;
                *fresh7 = bbtree_new(&mut (*bbt).extents, (*bbt).chain);
                if ((*bbt).right).is_null() {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
            } else {
                bbtree_add((*bbt).right, (*bbt).chain, &mut (*bbt).extents);
            }
            let ref mut fresh8 = (*bbt).chain;
            *fresh8 = 0 as *mut cairo_command_header_t;
        }
        (*bbt)
            .extents
            .p1
            .x = if (*bbt).extents.p1.x < (*box_0).p1.x {
            (*bbt).extents.p1.x
        } else {
            (*box_0).p1.x
        };
        (*bbt)
            .extents
            .p1
            .y = if (*bbt).extents.p1.y < (*box_0).p1.y {
            (*bbt).extents.p1.y
        } else {
            (*box_0).p1.y
        };
        (*bbt)
            .extents
            .p2
            .x = if (*bbt).extents.p2.x > (*box_0).p2.x {
            (*bbt).extents.p2.x
        } else {
            (*box_0).p2.x
        };
        (*bbt)
            .extents
            .p2
            .y = if (*bbt).extents.p2.y > (*box_0).p2.y {
            (*bbt).extents.p2.y
        } else {
            (*box_0).p2.y
        };
    }
    if (*box_0).p1.x == (*bbt).extents.p1.x && (*box_0).p1.y == (*bbt).extents.p1.y
        && (*box_0).p2.x == (*bbt).extents.p2.x && (*box_0).p2.y == (*bbt).extents.p2.y
    {
        let mut last: *mut cairo_command_header_t = header;
        while !((*last).chain).is_null() {
            last = (*last).chain;
        }
        let ref mut fresh9 = (*last).chain;
        *fresh9 = (*bbt).chain;
        let ref mut fresh10 = (*bbt).chain;
        *fresh10 = header;
        return CAIRO_STATUS_SUCCESS;
    }
    if bbtree_left_or_right(bbt, box_0) != 0 {
        if ((*bbt).left).is_null() {
            let ref mut fresh11 = (*bbt).left;
            *fresh11 = bbtree_new(box_0, header);
            if ((*bbt).left).is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        } else {
            return bbtree_add((*bbt).left, header, box_0)
        }
    } else if ((*bbt).right).is_null() {
        let ref mut fresh12 = (*bbt).right;
        *fresh12 = bbtree_new(box_0, header);
        if ((*bbt).right).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    } else {
        return bbtree_add((*bbt).right, header, box_0)
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn bbtree_del(mut bbt: *mut bbtree) {
    if !((*bbt).left).is_null() {
        bbtree_del((*bbt).left);
    }
    if !((*bbt).right).is_null() {
        bbtree_del((*bbt).right);
    }
    free(bbt as *mut libc::c_void);
}
unsafe extern "C" fn box_outside(
    mut a: *const cairo_box_t,
    mut b: *const cairo_box_t,
) -> cairo_bool_t {
    return ((*a).p1.x >= (*b).p2.x || (*a).p1.y >= (*b).p2.y || (*a).p2.x <= (*b).p1.x
        || (*a).p2.y <= (*b).p1.y) as libc::c_int;
}
unsafe extern "C" fn bbtree_foreach_mark_visible(
    mut bbt: *mut bbtree,
    mut box_0: *const cairo_box_t,
    mut indices: *mut *mut libc::c_uint,
) {
    let mut chain: *mut cairo_command_header_t = 0 as *mut cairo_command_header_t;
    chain = (*bbt).chain;
    while !chain.is_null() {
        let fresh13 = *indices;
        *indices = (*indices).offset(1);
        *fresh13 = (*chain).index as libc::c_uint;
        chain = (*chain).chain;
    }
    if !((*bbt).left).is_null() && box_outside(box_0, &mut (*(*bbt).left).extents) == 0 {
        bbtree_foreach_mark_visible((*bbt).left, box_0, indices);
    }
    if !((*bbt).right).is_null() && box_outside(box_0, &mut (*(*bbt).right).extents) == 0
    {
        bbtree_foreach_mark_visible((*bbt).right, box_0, indices);
    }
}
#[inline]
unsafe extern "C" fn intcmp(a: libc::c_uint, b: libc::c_uint) -> libc::c_int {
    return a.wrapping_sub(b) as libc::c_int;
}
unsafe extern "C" fn sort_indices(mut base: *mut libc::c_uint, mut nmemb: libc::c_uint) {
    let mut gap: libc::c_uint = nmemb;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut swapped: libc::c_int = 0;
    loop {
        gap = _cairo_combsort_newgap(gap);
        swapped = (gap > 1 as libc::c_int as libc::c_uint) as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while i < nmemb.wrapping_sub(gap) {
            j = i.wrapping_add(gap);
            if intcmp(*base.offset(i as isize), *base.offset(j as isize))
                > 0 as libc::c_int
            {
                let mut tmp: libc::c_uint = 0;
                tmp = *base.offset(i as isize);
                *base.offset(i as isize) = *base.offset(j as isize);
                *base.offset(j as isize) = tmp;
                swapped = 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        if !(swapped != 0) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn sizecmp(
    mut a: libc::c_uint,
    mut b: libc::c_uint,
    mut elements: *mut *mut cairo_command_header_t,
) -> libc::c_int {
    let mut r: *const cairo_rectangle_int_t = 0 as *const cairo_rectangle_int_t;
    r = &mut (**elements.offset(a as isize)).extents;
    a = ((*r).width * (*r).height) as libc::c_uint;
    r = &mut (**elements.offset(b as isize)).extents;
    b = ((*r).width * (*r).height) as libc::c_uint;
    return b.wrapping_sub(a) as libc::c_int;
}
unsafe extern "C" fn sort_commands(
    mut base: *mut libc::c_uint,
    mut nmemb: libc::c_uint,
    mut data: *mut libc::c_void,
) {
    let mut gap: libc::c_uint = nmemb;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut swapped: libc::c_int = 0;
    loop {
        gap = _cairo_combsort_newgap(gap);
        swapped = (gap > 1 as libc::c_int as libc::c_uint) as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while i < nmemb.wrapping_sub(gap) {
            j = i.wrapping_add(gap);
            if sizecmp(
                *base.offset(i as isize),
                *base.offset(j as isize),
                data as *mut *mut cairo_command_header_t,
            ) > 0 as libc::c_int
            {
                let mut tmp: libc::c_uint = 0;
                tmp = *base.offset(i as isize);
                *base.offset(i as isize) = *base.offset(j as isize);
                *base.offset(j as isize) = tmp;
                swapped = 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        if !(swapped != 0) {
            break;
        }
    };
}
unsafe extern "C" fn _cairo_recording_surface_destroy_bbtree(
    mut surface: *mut cairo_recording_surface_t,
) {
    let mut elements: *mut *mut cairo_command_t = 0 as *mut *mut cairo_command_t;
    let mut i: libc::c_int = 0;
    let mut num_elements: libc::c_int = 0;
    if (*surface).bbtree.chain == -(1 as libc::c_int) as *mut cairo_command_header_t {
        return;
    }
    if !((*surface).bbtree.left).is_null() {
        bbtree_del((*surface).bbtree.left);
        let ref mut fresh14 = (*surface).bbtree.left;
        *fresh14 = 0 as *mut bbtree;
    }
    if !((*surface).bbtree.right).is_null() {
        bbtree_del((*surface).bbtree.right);
        let ref mut fresh15 = (*surface).bbtree.right;
        *fresh15 = 0 as *mut bbtree;
    }
    elements = _cairo_array_index(
        &mut (*surface).commands,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut cairo_command_t;
    num_elements = (*surface).commands.num_elements as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elements {
        let ref mut fresh16 = (**elements.offset(i as isize)).header.chain;
        *fresh16 = 0 as *mut _cairo_command_header;
        i += 1;
    }
    let ref mut fresh17 = (*surface).bbtree.chain;
    *fresh17 = -(1 as libc::c_int) as *mut cairo_command_header_t;
}
unsafe extern "C" fn _cairo_recording_surface_create_bbtree(
    mut surface: *mut cairo_recording_surface_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut elements: *mut *mut cairo_command_t = _cairo_array_index(
        &mut (*surface).commands,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut cairo_command_t;
    let mut indices: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    count = (*surface).commands.num_elements;
    if count > (*surface).num_indices {
        free((*surface).indices as *mut libc::c_void);
        let ref mut fresh18 = (*surface).indices;
        *fresh18 = _cairo_malloc_ab(
            count as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_uint;
        if ((*surface).indices).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        (*surface).num_indices = count;
    }
    indices = (*surface).indices;
    i = 0 as libc::c_int as libc::c_uint;
    while i < count {
        *indices.offset(i as isize) = i;
        i = i.wrapping_add(1);
    }
    sort_commands(indices, count, elements as *mut libc::c_void);
    bbtree_init(
        &mut (*surface).bbtree,
        &mut (**elements.offset(*indices.offset(0 as libc::c_int as isize) as isize))
            .header,
    );
    i = 1 as libc::c_int as libc::c_uint;
    loop {
        if !(i < count) {
            current_block = 5601891728916014340;
            break;
        }
        let mut header: *mut cairo_command_header_t = &mut (**elements
            .offset(*indices.offset(i as isize) as isize))
            .header;
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        _cairo_box_from_rectangle(&mut box_0, &mut (*header).extents);
        status = bbtree_add(&mut (*surface).bbtree, header, &mut box_0);
        if status as u64 != 0 {
            current_block = 6993817191820622865;
            break;
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        5601891728916014340 => return CAIRO_STATUS_SUCCESS,
        _ => {
            bbtree_del(&mut (*surface).bbtree);
            return status;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_recording_surface_create(
    mut content: cairo_content_t,
    mut extents: *const cairo_rectangle_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_recording_surface_t = 0
        as *mut cairo_recording_surface_t;
    surface = (if ::std::mem::size_of::<cairo_recording_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_recording_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_recording_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_recording_surface_backend,
        0 as *mut cairo_device_t,
        content,
        1 as libc::c_int,
    );
    (*surface).unbounded = 1 as libc::c_int;
    if !extents.is_null() {
        (*surface).extents_pixels = *extents;
        (*surface).extents.x = floor((*extents).x) as libc::c_int;
        (*surface).extents.y = floor((*extents).y) as libc::c_int;
        (*surface)
            .extents
            .width = (ceil((*extents).x + (*extents).width)
            - (*surface).extents.x as libc::c_double) as libc::c_int;
        (*surface)
            .extents
            .height = (ceil((*extents).y + (*extents).height)
            - (*surface).extents.y as libc::c_double) as libc::c_int;
        (*surface).unbounded = 0 as libc::c_int;
    }
    _cairo_array_init(
        &mut (*surface).commands,
        ::std::mem::size_of::<*mut cairo_command_t>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh19 = (*surface).base;
    (*fresh19).set_is_clear(1 as libc::c_int as libc::c_uint);
    let ref mut fresh20 = (*surface).bbtree.right;
    *fresh20 = 0 as *mut bbtree;
    let ref mut fresh21 = (*surface).bbtree.left;
    *fresh21 = *fresh20;
    let ref mut fresh22 = (*surface).bbtree.chain;
    *fresh22 = -(1 as libc::c_int) as *mut cairo_command_header_t;
    let ref mut fresh23 = (*surface).indices;
    *fresh23 = 0 as *mut libc::c_uint;
    (*surface).num_indices = 0 as libc::c_int as libc::c_uint;
    (*surface).optimize_clears = 1 as libc::c_int;
    (*surface).has_bilevel_alpha = 0 as libc::c_int;
    (*surface).has_only_op_over = 0 as libc::c_int;
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_recording_surface_create_similar(
    mut abstract_surface: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut extents: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    extents.y = 0 as libc::c_int as libc::c_double;
    extents.x = extents.y;
    extents.width = width as libc::c_double;
    extents.height = height as libc::c_double;
    return cairo_recording_surface_create(content, &mut extents);
}
unsafe extern "C" fn _cairo_recording_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut elements: *mut *mut cairo_command_t = 0 as *mut *mut cairo_command_t;
    let mut i: libc::c_int = 0;
    let mut num_elements: libc::c_int = 0;
    num_elements = (*surface).commands.num_elements as libc::c_int;
    elements = _cairo_array_index(
        &mut (*surface).commands,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut cairo_command_t;
    i = 0 as libc::c_int;
    while i < num_elements {
        let mut command: *mut cairo_command_t = *elements.offset(i as isize);
        match (*command).header.type_0 as libc::c_uint {
            0 => {
                _cairo_pattern_fini(&mut (*command).paint.source.base);
            }
            1 => {
                _cairo_pattern_fini(&mut (*command).mask.source.base);
                _cairo_pattern_fini(&mut (*command).mask.mask.base);
            }
            2 => {
                _cairo_pattern_fini(&mut (*command).stroke.source.base);
                _cairo_path_fixed_fini(&mut (*command).stroke.path);
                _cairo_stroke_style_fini(&mut (*command).stroke.style);
            }
            3 => {
                _cairo_pattern_fini(&mut (*command).fill.source.base);
                _cairo_path_fixed_fini(&mut (*command).fill.path);
            }
            4 => {
                _cairo_pattern_fini(&mut (*command).show_text_glyphs.source.base);
                free((*command).show_text_glyphs.utf8 as *mut libc::c_void);
                free((*command).show_text_glyphs.glyphs as *mut libc::c_void);
                free((*command).show_text_glyphs.clusters as *mut libc::c_void);
                cairo_scaled_font_destroy((*command).show_text_glyphs.scaled_font);
            }
            5 => {
                free((*command).tag.tag_name as *mut libc::c_void);
                if (*command).tag.begin != 0 {
                    free((*command).tag.attributes as *mut libc::c_void);
                }
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-recording-surface.c\0" as *const u8
                            as *const libc::c_char,
                        505 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"cairo_status_t _cairo_recording_surface_finish(void *)\0"))
                            .as_ptr(),
                    );
                }
            }
        }
        _cairo_clip_destroy((*command).header.clip);
        free(command as *mut libc::c_void);
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).commands);
    if !((*surface).bbtree.left).is_null() {
        bbtree_del((*surface).bbtree.left);
    }
    if !((*surface).bbtree.right).is_null() {
        bbtree_del((*surface).bbtree.right);
    }
    free((*surface).indices as *mut libc::c_void);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn proxy_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut proxy: *mut proxy = abstract_surface as *mut proxy;
    return _cairo_surface_acquire_source_image((*proxy).image, image_out, image_extra);
}
unsafe extern "C" fn proxy_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    let mut proxy: *mut proxy = abstract_surface as *mut proxy;
    _cairo_surface_release_source_image((*proxy).image, image, image_extra);
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
            source: Some(
                _cairo_surface_default_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                proxy_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                proxy_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
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
    mut image: *mut cairo_surface_t,
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
        (*image).content,
        0 as libc::c_int,
    );
    let ref mut fresh24 = (*proxy).image;
    *fresh24 = image;
    _cairo_surface_attach_snapshot(source, &mut (*proxy).base, None);
    return &mut (*proxy).base;
}
unsafe extern "C" fn detach_proxy(
    mut source: *mut cairo_surface_t,
    mut proxy: *mut cairo_surface_t,
) {
    cairo_surface_finish(proxy);
    cairo_surface_destroy(proxy);
}
unsafe extern "C" fn get_proxy(mut proxy: *mut cairo_surface_t) -> *mut cairo_surface_t {
    return (*(proxy as *mut proxy)).image;
}
unsafe extern "C" fn _cairo_recording_surface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut proxy: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    proxy = _cairo_surface_has_snapshot(
        abstract_surface as *mut cairo_surface_t,
        &proxy_backend,
    );
    if !proxy.is_null() {
        *image_out = cairo_surface_reference(get_proxy(proxy))
            as *mut cairo_image_surface_t;
        *image_extra = 0 as *mut libc::c_void;
        return CAIRO_STATUS_SUCCESS;
    }
    if (*surface).unbounded == 0 {} else {
        __assert_fail(
            b"! surface->unbounded\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-recording-surface.c\0" as *const u8 as *const libc::c_char,
            617 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"cairo_status_t _cairo_recording_surface_acquire_source_image(void *, cairo_image_surface_t **, void **)\0",
            ))
                .as_ptr(),
        );
    }
    image = _cairo_image_surface_create_with_content(
        (*surface).base.content,
        (*surface).extents.width,
        (*surface).extents.height,
    );
    cairo_surface_set_device_offset(
        image,
        -(*surface).extents.x as libc::c_double,
        -(*surface).extents.y as libc::c_double,
    );
    if (*image).status as u64 != 0 {
        return (*image).status;
    }
    cairo_surface_set_device_offset(
        image,
        -(*surface).extents.x as libc::c_double,
        -(*surface).extents.y as libc::c_double,
    );
    proxy = attach_proxy(abstract_surface as *mut cairo_surface_t, image);
    status = _cairo_recording_surface_replay(&mut (*surface).base, image);
    detach_proxy(abstract_surface as *mut cairo_surface_t, proxy);
    if status as u64 != 0 {
        cairo_surface_destroy(image);
        return status;
    }
    *image_out = image as *mut cairo_image_surface_t;
    *image_extra = 0 as *mut libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_recording_surface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    cairo_surface_destroy(&mut (*image).base);
}
unsafe extern "C" fn _command_init(
    mut surface: *mut cairo_recording_surface_t,
    mut command: *mut cairo_command_header_t,
    mut type_0: cairo_command_type_t,
    mut op: cairo_operator_t,
    mut composite: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*command).type_0 = type_0;
    (*command).op = op;
    (*command).region = CAIRO_RECORDING_REGION_ALL;
    (*command)
        .extents = if !composite.is_null() {
        (*composite).unbounded
    } else {
        _cairo_empty_rectangle
    };
    let ref mut fresh25 = (*command).chain;
    *fresh25 = 0 as *mut _cairo_command_header;
    (*command).index = (*surface).commands.num_elements as libc::c_int;
    let ref mut fresh26 = (*command).clip;
    *fresh26 = 0 as *mut cairo_clip_t;
    if !composite.is_null()
        && _cairo_composite_rectangles_can_reduce_clip(composite, (*composite).clip) == 0
    {
        let ref mut fresh27 = (*command).clip;
        *fresh27 = (*composite).clip;
        let ref mut fresh28 = (*composite).clip;
        *fresh28 = 0 as *mut cairo_clip_t;
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_break_self_copy_loop(
    mut surface: *mut cairo_recording_surface_t,
) {
    cairo_surface_flush(&mut (*surface).base);
}
unsafe extern "C" fn _cairo_recording_surface_commit(
    mut surface: *mut cairo_recording_surface_t,
    mut command: *mut cairo_command_header_t,
) -> cairo_status_t {
    _cairo_recording_surface_break_self_copy_loop(surface);
    return _cairo_array_append(
        &mut (*surface).commands,
        &mut command as *mut *mut cairo_command_header_t as *const libc::c_void,
    );
}
unsafe extern "C" fn _cairo_recording_surface_reset(
    mut surface: *mut cairo_recording_surface_t,
) {
    _cairo_recording_surface_finish(surface as *mut libc::c_void);
    let ref mut fresh29 = (*surface).bbtree.right;
    *fresh29 = 0 as *mut bbtree;
    let ref mut fresh30 = (*surface).bbtree.left;
    *fresh30 = *fresh29;
    let ref mut fresh31 = (*surface).bbtree.chain;
    *fresh31 = -(1 as libc::c_int) as *mut cairo_command_header_t;
    let ref mut fresh32 = (*surface).indices;
    *fresh32 = 0 as *mut libc::c_uint;
    (*surface).num_indices = 0 as libc::c_int as libc::c_uint;
    _cairo_array_init(
        &mut (*surface).commands,
        ::std::mem::size_of::<*mut cairo_command_t>() as libc::c_ulong as libc::c_uint,
    );
}
unsafe extern "C" fn _cairo_recording_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut command: *mut cairo_command_paint_t = 0 as *mut cairo_command_paint_t;
    let mut composite: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        && clip.is_null()
    {
        if (*surface).optimize_clears != 0 {
            _cairo_recording_surface_reset(surface);
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    }
    if clip.is_null() && (*surface).optimize_clears != 0
        && (op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                && (((*surface).base).is_clear() as libc::c_int != 0
                    || _cairo_pattern_is_opaque_solid(source) != 0))
    {
        _cairo_recording_surface_reset(surface);
    }
    status = _cairo_composite_rectangles_init_for_paint(
        &mut composite,
        &mut (*surface).base,
        op,
        source,
        clip,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    command = (if ::std::mem::size_of::<cairo_command_paint_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_paint_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_paint_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        status = _command_init(
            surface,
            &mut (*command).header,
            CAIRO_COMMAND_PAINT,
            op,
            &mut composite,
        );
        if !(status as u64 != 0) {
            status = _cairo_pattern_init_snapshot(&mut (*command).source.base, source);
            if !(status as u64 != 0) {
                status = _cairo_recording_surface_commit(
                    surface,
                    &mut (*command).header,
                );
                if status as u64 != 0 {
                    _cairo_pattern_fini(&mut (*command).source.base);
                } else {
                    _cairo_recording_surface_destroy_bbtree(surface);
                    _cairo_composite_rectangles_fini(&mut composite);
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
            }
        }
        _cairo_clip_destroy((*command).header.clip);
        free(command as *mut libc::c_void);
    }
    _cairo_composite_rectangles_fini(&mut composite);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_recording_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut command: *mut cairo_command_mask_t = 0 as *mut cairo_command_mask_t;
    let mut composite: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    status = _cairo_composite_rectangles_init_for_mask(
        &mut composite,
        &mut (*surface).base,
        op,
        source,
        mask,
        clip,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    command = (if ::std::mem::size_of::<cairo_command_mask_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_mask_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_mask_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        status = _command_init(
            surface,
            &mut (*command).header,
            CAIRO_COMMAND_MASK,
            op,
            &mut composite,
        );
        if !(status as u64 != 0) {
            status = _cairo_pattern_init_snapshot(&mut (*command).source.base, source);
            if !(status as u64 != 0) {
                status = _cairo_pattern_init_snapshot(&mut (*command).mask.base, mask);
                if !(status as u64 != 0) {
                    status = _cairo_recording_surface_commit(
                        surface,
                        &mut (*command).header,
                    );
                    if status as u64 != 0 {
                        _cairo_pattern_fini(&mut (*command).mask.base);
                    } else {
                        _cairo_recording_surface_destroy_bbtree(surface);
                        _cairo_composite_rectangles_fini(&mut composite);
                        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                    }
                }
                _cairo_pattern_fini(&mut (*command).source.base);
            }
        }
        _cairo_clip_destroy((*command).header.clip);
        free(command as *mut libc::c_void);
    }
    _cairo_composite_rectangles_fini(&mut composite);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_recording_surface_stroke(
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut command: *mut cairo_command_stroke_t = 0 as *mut cairo_command_stroke_t;
    let mut composite: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    status = _cairo_composite_rectangles_init_for_stroke(
        &mut composite,
        &mut (*surface).base,
        op,
        source,
        path,
        style,
        ctm,
        clip,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    command = (if ::std::mem::size_of::<cairo_command_stroke_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_stroke_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_stroke_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        status = _command_init(
            surface,
            &mut (*command).header,
            CAIRO_COMMAND_STROKE,
            op,
            &mut composite,
        );
        if !(status as u64 != 0) {
            status = _cairo_pattern_init_snapshot(&mut (*command).source.base, source);
            if !(status as u64 != 0) {
                status = _cairo_path_fixed_init_copy(&mut (*command).path, path);
                if !(status as u64 != 0) {
                    status = _cairo_stroke_style_init_copy(&mut (*command).style, style);
                    if !(status as u64 != 0) {
                        (*command).ctm = *ctm;
                        (*command).ctm_inverse = *ctm_inverse;
                        (*command).tolerance = tolerance;
                        (*command).antialias = antialias;
                        status = _cairo_recording_surface_commit(
                            surface,
                            &mut (*command).header,
                        );
                        if status as u64 != 0 {
                            _cairo_stroke_style_fini(&mut (*command).style);
                        } else {
                            _cairo_recording_surface_destroy_bbtree(surface);
                            _cairo_composite_rectangles_fini(&mut composite);
                            return CAIRO_STATUS_SUCCESS as libc::c_int
                                as cairo_int_status_t;
                        }
                    }
                    _cairo_path_fixed_fini(&mut (*command).path);
                }
                _cairo_pattern_fini(&mut (*command).source.base);
            }
        }
        _cairo_clip_destroy((*command).header.clip);
        free(command as *mut libc::c_void);
    }
    _cairo_composite_rectangles_fini(&mut composite);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_recording_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut command: *mut cairo_command_fill_t = 0 as *mut cairo_command_fill_t;
    let mut composite: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    status = _cairo_composite_rectangles_init_for_fill(
        &mut composite,
        &mut (*surface).base,
        op,
        source,
        path,
        clip,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    command = (if ::std::mem::size_of::<cairo_command_fill_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_fill_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_fill_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        status = _command_init(
            surface,
            &mut (*command).header,
            CAIRO_COMMAND_FILL,
            op,
            &mut composite,
        );
        if !(status as u64 != 0) {
            status = _cairo_pattern_init_snapshot(&mut (*command).source.base, source);
            if !(status as u64 != 0) {
                status = _cairo_path_fixed_init_copy(&mut (*command).path, path);
                if !(status as u64 != 0) {
                    (*command).fill_rule = fill_rule;
                    (*command).tolerance = tolerance;
                    (*command).antialias = antialias;
                    status = _cairo_recording_surface_commit(
                        surface,
                        &mut (*command).header,
                    );
                    if status as u64 != 0 {
                        _cairo_path_fixed_fini(&mut (*command).path);
                    } else {
                        _cairo_recording_surface_destroy_bbtree(surface);
                        _cairo_composite_rectangles_fini(&mut composite);
                        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                    }
                }
                _cairo_pattern_fini(&mut (*command).source.base);
            }
        }
        _cairo_clip_destroy((*command).header.clip);
        free(command as *mut libc::c_void);
    }
    _cairo_composite_rectangles_fini(&mut composite);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_recording_surface_has_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_recording_surface_show_text_glyphs(
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
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut command: *mut cairo_command_show_text_glyphs_t = 0
        as *mut cairo_command_show_text_glyphs_t;
    let mut composite: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    status = _cairo_composite_rectangles_init_for_glyphs(
        &mut composite,
        &mut (*surface).base,
        op,
        source,
        scaled_font,
        glyphs,
        num_glyphs,
        clip,
        0 as *mut cairo_bool_t,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    command = (if ::std::mem::size_of::<cairo_command_show_text_glyphs_t>()
        as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
    {
        malloc(
            ::std::mem::size_of::<cairo_command_show_text_glyphs_t>() as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_show_text_glyphs_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        status = _command_init(
            surface,
            &mut (*command).header,
            CAIRO_COMMAND_SHOW_TEXT_GLYPHS,
            op,
            &mut composite,
        );
        if !(status as u64 != 0) {
            status = _cairo_pattern_init_snapshot(&mut (*command).source.base, source);
            if !(status as u64 != 0) {
                let ref mut fresh33 = (*command).utf8;
                *fresh33 = 0 as *mut libc::c_char;
                (*command).utf8_len = utf8_len;
                let ref mut fresh34 = (*command).glyphs;
                *fresh34 = 0 as *mut cairo_glyph_t;
                (*command).num_glyphs = num_glyphs as libc::c_uint;
                let ref mut fresh35 = (*command).clusters;
                *fresh35 = 0 as *mut cairo_text_cluster_t;
                (*command).num_clusters = num_clusters;
                if utf8_len != 0 {
                    let ref mut fresh36 = (*command).utf8;
                    *fresh36 = (if utf8_len != 0 as libc::c_int {
                        malloc(utf8_len as libc::c_ulong)
                    } else {
                        0 as *mut libc::c_void
                    }) as *mut libc::c_char;
                    if ((*command).utf8).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                        current_block = 5688903204127586675;
                    } else {
                        memcpy(
                            (*command).utf8 as *mut libc::c_void,
                            utf8 as *const libc::c_void,
                            utf8_len as libc::c_ulong,
                        );
                        current_block = 11307063007268554308;
                    }
                } else {
                    current_block = 11307063007268554308;
                }
                match current_block {
                    11307063007268554308 => {
                        if num_glyphs != 0 {
                            let ref mut fresh37 = (*command).glyphs;
                            *fresh37 = _cairo_malloc_ab(
                                num_glyphs as size_t,
                                ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
                            ) as *mut cairo_glyph_t;
                            if ((*command).glyphs).is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                current_block = 5688903204127586675;
                            } else {
                                memcpy(
                                    (*command).glyphs as *mut libc::c_void,
                                    glyphs as *const libc::c_void,
                                    (::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                                        .wrapping_mul(num_glyphs as libc::c_ulong),
                                );
                                current_block = 4775909272756257391;
                            }
                        } else {
                            current_block = 4775909272756257391;
                        }
                        match current_block {
                            5688903204127586675 => {}
                            _ => {
                                if num_clusters != 0 {
                                    let ref mut fresh38 = (*command).clusters;
                                    *fresh38 = _cairo_malloc_ab(
                                        num_clusters as size_t,
                                        ::std::mem::size_of::<cairo_text_cluster_t>()
                                            as libc::c_ulong,
                                    ) as *mut cairo_text_cluster_t;
                                    if ((*command).clusters).is_null() {
                                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                        current_block = 5688903204127586675;
                                    } else {
                                        memcpy(
                                            (*command).clusters as *mut libc::c_void,
                                            clusters as *const libc::c_void,
                                            (::std::mem::size_of::<cairo_text_cluster_t>()
                                                as libc::c_ulong)
                                                .wrapping_mul(num_clusters as libc::c_ulong),
                                        );
                                        current_block = 7333393191927787629;
                                    }
                                } else {
                                    current_block = 7333393191927787629;
                                }
                                match current_block {
                                    5688903204127586675 => {}
                                    _ => {
                                        (*command).cluster_flags = cluster_flags;
                                        let ref mut fresh39 = (*command).scaled_font;
                                        *fresh39 = cairo_scaled_font_reference(scaled_font);
                                        status = _cairo_recording_surface_commit(
                                            surface,
                                            &mut (*command).header,
                                        );
                                        if status as u64 != 0 {
                                            cairo_scaled_font_destroy((*command).scaled_font);
                                        } else {
                                            _cairo_composite_rectangles_fini(&mut composite);
                                            return CAIRO_STATUS_SUCCESS as libc::c_int
                                                as cairo_int_status_t;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                free((*command).utf8 as *mut libc::c_void);
                free((*command).glyphs as *mut libc::c_void);
                free((*command).clusters as *mut libc::c_void);
                _cairo_pattern_fini(&mut (*command).source.base);
            }
        }
        _cairo_clip_destroy((*command).header.clip);
        free(command as *mut libc::c_void);
    }
    _cairo_composite_rectangles_fini(&mut composite);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_recording_surface_tag(
    mut abstract_surface: *mut libc::c_void,
    mut begin: cairo_bool_t,
    mut tag_name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    let mut command: *mut cairo_command_tag_t = 0 as *mut cairo_command_tag_t;
    command = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_command_tag_t>() as libc::c_ulong,
    ) as *mut cairo_command_tag_t;
    if command.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _command_init(
        surface,
        &mut (*command).header,
        CAIRO_COMMAND_TAG,
        CAIRO_OPERATOR_SOURCE,
        0 as *mut cairo_composite_rectangles_t,
    );
    if !(status as u64 != 0) {
        (*command).begin = begin;
        let ref mut fresh40 = (*command).tag_name;
        *fresh40 = strdup(tag_name);
        if ((*command).tag_name).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            if begin != 0 {
                if !attributes.is_null() {
                    let ref mut fresh41 = (*command).attributes;
                    *fresh41 = strdup(attributes);
                    if ((*command).attributes).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                        current_block = 149470181576859839;
                    } else {
                        current_block = 5948590327928692120;
                    }
                } else {
                    current_block = 5948590327928692120;
                }
            } else {
                current_block = 5948590327928692120;
            }
            match current_block {
                5948590327928692120 => {
                    status = _cairo_recording_surface_commit(
                        surface,
                        &mut (*command).header,
                    );
                    if !(status as u64 != 0) {
                        _cairo_recording_surface_destroy_bbtree(surface);
                        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                    }
                }
                _ => {}
            }
            free((*command).tag_name as *mut libc::c_void);
            free((*command).attributes as *mut libc::c_void);
        }
    }
    _cairo_clip_destroy((*command).header.clip);
    free(command as *mut libc::c_void);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _command_init_copy(
    mut surface: *mut cairo_recording_surface_t,
    mut dst: *mut cairo_command_header_t,
    mut src: *const cairo_command_header_t,
) {
    (*dst).type_0 = (*src).type_0;
    (*dst).op = (*src).op;
    (*dst).region = CAIRO_RECORDING_REGION_ALL;
    (*dst).extents = (*src).extents;
    let ref mut fresh42 = (*dst).chain;
    *fresh42 = 0 as *mut _cairo_command_header;
    (*dst).index = (*surface).commands.num_elements as libc::c_int;
    let ref mut fresh43 = (*dst).clip;
    *fresh43 = _cairo_clip_copy((*src).clip);
}
unsafe extern "C" fn _cairo_recording_surface_copy__paint(
    mut surface: *mut cairo_recording_surface_t,
    mut src: *const cairo_command_t,
) -> cairo_status_t {
    let mut command: *mut cairo_command_paint_t = 0 as *mut cairo_command_paint_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    command = (if ::std::mem::size_of::<cairo_command_paint_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_paint_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_paint_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _command_init_copy(surface, &mut (*command).header, &(*src).header);
        status = _cairo_pattern_init_copy(
            &mut (*command).source.base,
            &(*src).paint.source.base,
        );
        if !(status as u64 != 0) {
            status = _cairo_recording_surface_commit(surface, &mut (*command).header);
            if status as u64 != 0 {
                _cairo_pattern_fini(&mut (*command).source.base);
            } else {
                return CAIRO_STATUS_SUCCESS
            }
        }
        free(command as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_copy__mask(
    mut surface: *mut cairo_recording_surface_t,
    mut src: *const cairo_command_t,
) -> cairo_status_t {
    let mut command: *mut cairo_command_mask_t = 0 as *mut cairo_command_mask_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    command = (if ::std::mem::size_of::<cairo_command_mask_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_mask_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_mask_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _command_init_copy(surface, &mut (*command).header, &(*src).header);
        status = _cairo_pattern_init_copy(
            &mut (*command).source.base,
            &(*src).mask.source.base,
        );
        if !(status as u64 != 0) {
            status = _cairo_pattern_init_copy(
                &mut (*command).mask.base,
                &(*src).mask.mask.base,
            );
            if !(status as u64 != 0) {
                status = _cairo_recording_surface_commit(
                    surface,
                    &mut (*command).header,
                );
                if status as u64 != 0 {
                    _cairo_pattern_fini(&mut (*command).mask.base);
                } else {
                    return CAIRO_STATUS_SUCCESS
                }
            }
            _cairo_pattern_fini(&mut (*command).source.base);
        }
        free(command as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_copy__stroke(
    mut surface: *mut cairo_recording_surface_t,
    mut src: *const cairo_command_t,
) -> cairo_status_t {
    let mut command: *mut cairo_command_stroke_t = 0 as *mut cairo_command_stroke_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    command = (if ::std::mem::size_of::<cairo_command_stroke_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_stroke_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_stroke_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _command_init_copy(surface, &mut (*command).header, &(*src).header);
        status = _cairo_pattern_init_copy(
            &mut (*command).source.base,
            &(*src).stroke.source.base,
        );
        if !(status as u64 != 0) {
            status = _cairo_path_fixed_init_copy(
                &mut (*command).path,
                &(*src).stroke.path,
            );
            if !(status as u64 != 0) {
                status = _cairo_stroke_style_init_copy(
                    &mut (*command).style,
                    &(*src).stroke.style,
                );
                if !(status as u64 != 0) {
                    (*command).ctm = (*src).stroke.ctm;
                    (*command).ctm_inverse = (*src).stroke.ctm_inverse;
                    (*command).tolerance = (*src).stroke.tolerance;
                    (*command).antialias = (*src).stroke.antialias;
                    status = _cairo_recording_surface_commit(
                        surface,
                        &mut (*command).header,
                    );
                    if status as u64 != 0 {
                        _cairo_stroke_style_fini(&mut (*command).style);
                    } else {
                        return CAIRO_STATUS_SUCCESS
                    }
                }
                _cairo_path_fixed_fini(&mut (*command).path);
            }
            _cairo_pattern_fini(&mut (*command).source.base);
        }
        free(command as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_copy__fill(
    mut surface: *mut cairo_recording_surface_t,
    mut src: *const cairo_command_t,
) -> cairo_status_t {
    let mut command: *mut cairo_command_fill_t = 0 as *mut cairo_command_fill_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    command = (if ::std::mem::size_of::<cairo_command_fill_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_command_fill_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_fill_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _command_init_copy(surface, &mut (*command).header, &(*src).header);
        status = _cairo_pattern_init_copy(
            &mut (*command).source.base,
            &(*src).fill.source.base,
        );
        if !(status as u64 != 0) {
            status = _cairo_path_fixed_init_copy(
                &mut (*command).path,
                &(*src).fill.path,
            );
            if !(status as u64 != 0) {
                (*command).fill_rule = (*src).fill.fill_rule;
                (*command).tolerance = (*src).fill.tolerance;
                (*command).antialias = (*src).fill.antialias;
                status = _cairo_recording_surface_commit(
                    surface,
                    &mut (*command).header,
                );
                if status as u64 != 0 {
                    _cairo_path_fixed_fini(&mut (*command).path);
                } else {
                    return CAIRO_STATUS_SUCCESS
                }
            }
            _cairo_pattern_fini(&mut (*command).source.base);
        }
        free(command as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_copy__glyphs(
    mut surface: *mut cairo_recording_surface_t,
    mut src: *const cairo_command_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut command: *mut cairo_command_show_text_glyphs_t = 0
        as *mut cairo_command_show_text_glyphs_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    command = (if ::std::mem::size_of::<cairo_command_show_text_glyphs_t>()
        as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
    {
        malloc(
            ::std::mem::size_of::<cairo_command_show_text_glyphs_t>() as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_command_show_text_glyphs_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _command_init_copy(surface, &mut (*command).header, &(*src).header);
        status = _cairo_pattern_init_copy(
            &mut (*command).source.base,
            &(*src).show_text_glyphs.source.base,
        );
        if !(status as u64 != 0) {
            let ref mut fresh44 = (*command).utf8;
            *fresh44 = 0 as *mut libc::c_char;
            (*command).utf8_len = (*src).show_text_glyphs.utf8_len;
            let ref mut fresh45 = (*command).glyphs;
            *fresh45 = 0 as *mut cairo_glyph_t;
            (*command).num_glyphs = (*src).show_text_glyphs.num_glyphs;
            let ref mut fresh46 = (*command).clusters;
            *fresh46 = 0 as *mut cairo_text_cluster_t;
            (*command).num_clusters = (*src).show_text_glyphs.num_clusters;
            if (*command).utf8_len != 0 {
                let ref mut fresh47 = (*command).utf8;
                *fresh47 = (if (*command).utf8_len != 0 as libc::c_int {
                    malloc((*command).utf8_len as libc::c_ulong)
                } else {
                    0 as *mut libc::c_void
                }) as *mut libc::c_char;
                if ((*command).utf8).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    current_block = 221213694942880872;
                } else {
                    memcpy(
                        (*command).utf8 as *mut libc::c_void,
                        (*src).show_text_glyphs.utf8 as *const libc::c_void,
                        (*command).utf8_len as libc::c_ulong,
                    );
                    current_block = 8457315219000651999;
                }
            } else {
                current_block = 8457315219000651999;
            }
            match current_block {
                8457315219000651999 => {
                    if (*command).num_glyphs != 0 {
                        let ref mut fresh48 = (*command).glyphs;
                        *fresh48 = _cairo_malloc_ab(
                            (*command).num_glyphs as size_t,
                            ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
                        ) as *mut cairo_glyph_t;
                        if ((*command).glyphs).is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                            current_block = 221213694942880872;
                        } else {
                            memcpy(
                                (*command).glyphs as *mut libc::c_void,
                                (*src).show_text_glyphs.glyphs as *const libc::c_void,
                                (::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                                    .wrapping_mul((*command).num_glyphs as libc::c_ulong),
                            );
                            current_block = 11298138898191919651;
                        }
                    } else {
                        current_block = 11298138898191919651;
                    }
                    match current_block {
                        221213694942880872 => {}
                        _ => {
                            if (*command).num_clusters != 0 {
                                let ref mut fresh49 = (*command).clusters;
                                *fresh49 = _cairo_malloc_ab(
                                    (*command).num_clusters as size_t,
                                    ::std::mem::size_of::<cairo_text_cluster_t>()
                                        as libc::c_ulong,
                                ) as *mut cairo_text_cluster_t;
                                if ((*command).clusters).is_null() {
                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                    current_block = 221213694942880872;
                                } else {
                                    memcpy(
                                        (*command).clusters as *mut libc::c_void,
                                        (*src).show_text_glyphs.clusters as *const libc::c_void,
                                        (::std::mem::size_of::<cairo_text_cluster_t>()
                                            as libc::c_ulong)
                                            .wrapping_mul((*command).num_clusters as libc::c_ulong),
                                    );
                                    current_block = 4761528863920922185;
                                }
                            } else {
                                current_block = 4761528863920922185;
                            }
                            match current_block {
                                221213694942880872 => {}
                                _ => {
                                    (*command)
                                        .cluster_flags = (*src).show_text_glyphs.cluster_flags;
                                    let ref mut fresh50 = (*command).scaled_font;
                                    *fresh50 = cairo_scaled_font_reference(
                                        (*src).show_text_glyphs.scaled_font,
                                    );
                                    status = _cairo_recording_surface_commit(
                                        surface,
                                        &mut (*command).header,
                                    );
                                    if !(status as u64 != 0) {
                                        return CAIRO_STATUS_SUCCESS;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            free((*command).utf8 as *mut libc::c_void);
            free((*command).glyphs as *mut libc::c_void);
            free((*command).clusters as *mut libc::c_void);
            _cairo_pattern_fini(&mut (*command).source.base);
        }
        free(command as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_copy__tag(
    mut surface: *mut cairo_recording_surface_t,
    mut src: *const cairo_command_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut command: *mut cairo_command_tag_t = 0 as *mut cairo_command_tag_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    command = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_command_tag_t>() as libc::c_ulong,
    ) as *mut cairo_command_tag_t;
    if command.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _command_init_copy(surface, &mut (*command).header, &(*src).header);
        (*command).begin = (*src).tag.begin;
        let ref mut fresh51 = (*command).tag_name;
        *fresh51 = strdup((*src).tag.tag_name);
        if ((*command).tag_name).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            if (*src).tag.begin != 0 {
                if !((*src).tag.attributes).is_null() {
                    let ref mut fresh52 = (*command).attributes;
                    *fresh52 = strdup((*src).tag.attributes);
                    if ((*command).attributes).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                        current_block = 4119781943706198705;
                    } else {
                        current_block = 1054647088692577877;
                    }
                } else {
                    current_block = 1054647088692577877;
                }
            } else {
                current_block = 1054647088692577877;
            }
            match current_block {
                4119781943706198705 => {}
                _ => {
                    status = _cairo_recording_surface_commit(
                        surface,
                        &mut (*command).header,
                    );
                    if !(status as u64 != 0) {
                        return CAIRO_STATUS_SUCCESS;
                    }
                }
            }
        }
        free((*command).tag_name as *mut libc::c_void);
        free((*command).attributes as *mut libc::c_void);
        free(command as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_copy(
    mut dst: *mut cairo_recording_surface_t,
    mut src: *mut cairo_recording_surface_t,
) -> cairo_status_t {
    let mut elements: *mut *mut cairo_command_t = 0 as *mut *mut cairo_command_t;
    let mut i: libc::c_int = 0;
    let mut num_elements: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    elements = _cairo_array_index(&mut (*src).commands, 0 as libc::c_int as libc::c_uint)
        as *mut *mut cairo_command_t;
    num_elements = (*src).commands.num_elements as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elements {
        let mut command: *const cairo_command_t = *elements.offset(i as isize);
        match (*command).header.type_0 as libc::c_uint {
            0 => {
                status = _cairo_recording_surface_copy__paint(dst, command);
            }
            1 => {
                status = _cairo_recording_surface_copy__mask(dst, command);
            }
            2 => {
                status = _cairo_recording_surface_copy__stroke(dst, command);
            }
            3 => {
                status = _cairo_recording_surface_copy__fill(dst, command);
            }
            4 => {
                status = _cairo_recording_surface_copy__glyphs(dst, command);
            }
            5 => {
                status = _cairo_recording_surface_copy__tag(dst, command);
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-recording-surface.c\0" as *const u8
                            as *const libc::c_char,
                        1505 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 103],
                            &[libc::c_char; 103],
                        >(
                            b"cairo_status_t _cairo_recording_surface_copy(cairo_recording_surface_t *, cairo_recording_surface_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_recording_surface_snapshot(
    mut abstract_other: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut other: *mut cairo_recording_surface_t = abstract_other
        as *mut cairo_recording_surface_t;
    let mut surface: *mut cairo_recording_surface_t = 0
        as *mut cairo_recording_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    surface = (if ::std::mem::size_of::<cairo_recording_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_recording_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_recording_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_recording_surface_backend,
        0 as *mut cairo_device_t,
        (*other).base.content,
        ((*other).base).is_vector() as cairo_bool_t,
    );
    (*surface).extents_pixels = (*other).extents_pixels;
    (*surface).extents = (*other).extents;
    (*surface).unbounded = (*other).unbounded;
    (*surface).has_bilevel_alpha = (*other).has_bilevel_alpha;
    (*surface).has_only_op_over = (*other).has_only_op_over;
    let ref mut fresh53 = (*surface).base;
    (*fresh53).set_is_clear(((*other).base).is_clear());
    let ref mut fresh54 = (*surface).bbtree.right;
    *fresh54 = 0 as *mut bbtree;
    let ref mut fresh55 = (*surface).bbtree.left;
    *fresh55 = *fresh54;
    let ref mut fresh56 = (*surface).bbtree.chain;
    *fresh56 = -(1 as libc::c_int) as *mut cairo_command_header_t;
    let ref mut fresh57 = (*surface).indices;
    *fresh57 = 0 as *mut libc::c_uint;
    (*surface).num_indices = 0 as libc::c_int as libc::c_uint;
    (*surface).optimize_clears = 1 as libc::c_int;
    (*surface).has_bilevel_alpha = (*other).has_bilevel_alpha;
    (*surface).has_only_op_over = (*other).has_only_op_over;
    _cairo_array_init(
        &mut (*surface).commands,
        ::std::mem::size_of::<*mut cairo_command_t>() as libc::c_ulong as libc::c_uint,
    );
    status = _cairo_recording_surface_copy(surface, other);
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*surface).base);
        return _cairo_surface_create_in_error(status);
    }
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_recording_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_recording_surface_t = abstract_surface
        as *mut cairo_recording_surface_t;
    if (*surface).unbounded != 0 {
        return 0 as libc::c_int;
    }
    *rectangle = (*surface).extents;
    return 1 as libc::c_int;
}
static mut cairo_recording_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_RECORDING,
            finish: Some(
                _cairo_recording_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_recording_surface_create_similar
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
                _cairo_surface_default_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_recording_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_recording_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_recording_surface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: None,
            show_page: None,
            get_extents: Some(
                _cairo_recording_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_recording_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_recording_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_recording_surface_stroke
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
                _cairo_recording_surface_fill
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
                _cairo_recording_surface_has_show_text_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            show_text_glyphs: Some(
                _cairo_recording_surface_show_text_glyphs
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
                _cairo_recording_surface_tag
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
pub unsafe extern "C" fn _cairo_recording_surface_get_path(
    mut abstract_surface: *mut cairo_surface_t,
    mut path: *mut cairo_path_fixed_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_recording_surface_t = 0
        as *mut cairo_recording_surface_t;
    let mut elements: *mut *mut cairo_command_t = 0 as *mut *mut cairo_command_t;
    let mut i: libc::c_int = 0;
    let mut num_elements: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*abstract_surface).status as u64 != 0 {
        return (*abstract_surface).status as cairo_int_status_t;
    }
    surface = abstract_surface as *mut cairo_recording_surface_t;
    status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    num_elements = (*surface).commands.num_elements as libc::c_int;
    elements = _cairo_array_index(
        &mut (*surface).commands,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut cairo_command_t;
    i = 0 as libc::c_int;
    while i < num_elements {
        let mut command: *mut cairo_command_t = *elements.offset(i as isize);
        match (*command).header.type_0 as libc::c_uint {
            0 | 1 => {
                status = CAIRO_INT_STATUS_UNSUPPORTED;
            }
            2 => {
                let mut traps: cairo_traps_t = cairo_traps_t {
                    status: CAIRO_STATUS_SUCCESS,
                    bounds: cairo_box_t {
                        p1: cairo_point_t { x: 0, y: 0 },
                        p2: cairo_point_t { x: 0, y: 0 },
                    },
                    limits: 0 as *const cairo_box_t,
                    num_limits: 0,
                    maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
                    c2rust_padding: [0; 3],
                    num_traps: 0,
                    traps_size: 0,
                    traps: 0 as *mut cairo_trapezoid_t,
                    traps_embedded: [cairo_trapezoid_t {
                        top: 0,
                        bottom: 0,
                        left: cairo_box_t {
                            p1: cairo_point_t { x: 0, y: 0 },
                            p2: cairo_point_t { x: 0, y: 0 },
                        },
                        right: cairo_box_t {
                            p1: cairo_point_t { x: 0, y: 0 },
                            p2: cairo_point_t { x: 0, y: 0 },
                        },
                    }; 16],
                };
                _cairo_traps_init(&mut traps);
                status = _cairo_path_fixed_stroke_polygon_to_traps(
                    &mut (*command).stroke.path,
                    &mut (*command).stroke.style,
                    &mut (*command).stroke.ctm,
                    &mut (*command).stroke.ctm_inverse,
                    (*command).stroke.tolerance,
                    &mut traps,
                );
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    status = _cairo_traps_path(&mut traps, path) as cairo_int_status_t;
                }
                _cairo_traps_fini(&mut traps);
            }
            3 => {
                status = _cairo_path_fixed_append(
                    path,
                    &mut (*command).fill.path,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) as cairo_int_status_t;
            }
            4 => {
                status = _cairo_scaled_font_glyph_path(
                    (*command).show_text_glyphs.scaled_font,
                    (*command).show_text_glyphs.glyphs,
                    (*command).show_text_glyphs.num_glyphs as libc::c_int,
                    path,
                ) as cairo_int_status_t;
            }
            5 => {}
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-recording-surface.c\0" as *const u8
                            as *const libc::c_char,
                        1693 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"cairo_int_status_t _cairo_recording_surface_get_path(cairo_surface_t *, cairo_path_fixed_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        if status as u64 != 0 {
            break;
        }
        i += 1;
    }
    return status;
}
unsafe extern "C" fn _cairo_recording_surface_get_visible_commands(
    mut surface: *mut cairo_recording_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> libc::c_int {
    let mut num_visible: libc::c_uint = 0;
    let mut indices: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if (*surface).commands.num_elements == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    _cairo_box_from_rectangle(&mut box_0, extents);
    if (*surface).bbtree.chain == -(1 as libc::c_int) as *mut cairo_command_header_t {
        _cairo_recording_surface_create_bbtree(surface);
    }
    indices = (*surface).indices;
    bbtree_foreach_mark_visible(&mut (*surface).bbtree, &mut box_0, &mut indices);
    num_visible = indices.offset_from((*surface).indices) as libc::c_long
        as libc::c_uint;
    if num_visible > 1 as libc::c_int as libc::c_uint {
        sort_indices((*surface).indices, num_visible);
    }
    return num_visible as libc::c_int;
}
unsafe extern "C" fn _cairo_recording_surface_merge_source_attributes(
    mut surface: *mut cairo_recording_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
) {
    if op as libc::c_uint != CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        (*surface).has_only_op_over = 0 as libc::c_int;
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        let mut surf_pat: *mut cairo_surface_pattern_t = source
            as *mut cairo_surface_pattern_t;
        let mut surf: *mut cairo_surface_t = (*surf_pat).surface;
        let mut free_me: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        if _cairo_surface_is_snapshot(surf) != 0 {
            surf = _cairo_surface_snapshot_get_target(surf);
            free_me = surf;
        }
        if (*surf).status as u64 != 0 {
            return;
        }
        if (*surf).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
        {
            let mut rec_surf: *mut cairo_recording_surface_t = surf
                as *mut cairo_recording_surface_t;
            if _cairo_recording_surface_has_only_bilevel_alpha(rec_surf) == 0 {
                (*surface).has_bilevel_alpha = 0 as libc::c_int;
            }
            if _cairo_recording_surface_has_only_op_over(rec_surf) == 0 {
                (*surface).has_only_op_over = 0 as libc::c_int;
            }
        } else if (*surf).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        {
            let mut img_surf: *mut cairo_image_surface_t = surf
                as *mut cairo_image_surface_t;
            if _cairo_image_analyze_transparency(img_surf) as libc::c_uint
                == CAIRO_IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
            {
                (*surface).has_bilevel_alpha = 0 as libc::c_int;
            }
        } else if _cairo_pattern_is_clear(source) == 0
            && _cairo_pattern_is_opaque(source, 0 as *const cairo_rectangle_int_t) == 0
        {
            (*surface).has_bilevel_alpha = 0 as libc::c_int;
        }
        cairo_surface_destroy(free_me);
        return;
    } else {
        if (*source).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
        {
            let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
            let mut raster: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
            image = cairo_image_surface_create(
                CAIRO_FORMAT_ARGB32,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            raster = _cairo_raster_source_pattern_acquire(
                source,
                image,
                0 as *const cairo_rectangle_int_t,
            );
            cairo_surface_destroy(image);
            if !raster.is_null() {
                if (*raster).type_0 as libc::c_uint
                    == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
                {
                    if _cairo_image_analyze_transparency(
                        raster as *mut cairo_image_surface_t,
                    ) as libc::c_uint
                        == CAIRO_IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
                    {
                        (*surface).has_bilevel_alpha = 0 as libc::c_int;
                    }
                }
                _cairo_raster_source_pattern_release(source, raster);
                if (*raster).type_0 as libc::c_uint
                    == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
                {
                    return;
                }
            }
        }
    }
    if _cairo_pattern_is_clear(source) == 0
        && _cairo_pattern_is_opaque(source, 0 as *const cairo_rectangle_int_t) == 0
    {
        (*surface).has_bilevel_alpha = 0 as libc::c_int;
    }
}
unsafe extern "C" fn _cairo_recording_surface_replay_internal(
    mut surface: *mut cairo_recording_surface_t,
    mut params: *mut cairo_recording_surface_replay_params_t,
) -> cairo_status_t {
    let mut wrapper: cairo_surface_wrapper_t = cairo_surface_wrapper_t {
        target: 0 as *mut cairo_surface_t,
        transform: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        has_extents: 0,
        extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        clip: 0 as *const cairo_clip_t,
        foreground_source: 0 as *mut cairo_pattern_t,
        needs_transform: 0,
    };
    let mut elements: *mut *mut cairo_command_t = 0 as *mut *mut cairo_command_t;
    let mut replay_all: cairo_bool_t = ((*params).type_0 as libc::c_uint
        == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int as libc::c_uint
        || (*params).region as libc::c_uint
            == CAIRO_RECORDING_REGION_ALL as libc::c_int as libc::c_uint) as libc::c_int;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut use_indices: cairo_bool_t = 0 as libc::c_int;
    let mut r: *const cairo_rectangle_int_t = 0 as *const cairo_rectangle_int_t;
    let mut i: libc::c_uint = 0;
    let mut num_elements: libc::c_uint = 0;
    if (*surface).base.status as u64 != 0 {
        return (*surface).base.status;
    }
    if (*(*params).target).status as u64 != 0 {
        return (*(*params).target).status;
    }
    if ((*surface).base).finished() != 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    if ((*surface).base).is_clear() != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_surface_is_recording(&mut (*surface).base) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_is_recording (&surface->base)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-recording-surface.c\0" as *const u8 as *const libc::c_char,
            1820 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 128],
                &[libc::c_char; 128],
            >(
                b"cairo_status_t _cairo_recording_surface_replay_internal(cairo_recording_surface_t *, cairo_recording_surface_replay_params_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_surface_wrapper_init(&mut wrapper, (*params).target);
    if !((*params).surface_extents).is_null() {
        _cairo_surface_wrapper_intersect_extents(
            &mut wrapper,
            (*params).surface_extents,
        );
    }
    r = &_cairo_unbounded_rectangle;
    if (*surface).unbounded == 0 && (*params).surface_is_unbounded == 0 {
        _cairo_surface_wrapper_intersect_extents(&mut wrapper, &mut (*surface).extents);
        r = &mut (*surface).extents;
    }
    _cairo_surface_wrapper_set_inverse_transform(
        &mut wrapper,
        (*params).surface_transform,
    );
    _cairo_surface_wrapper_set_clip(&mut wrapper, (*params).target_clip);
    _cairo_surface_wrapper_set_foreground_color(
        &mut wrapper,
        (*params).foreground_color,
    );
    if !(_cairo_surface_wrapper_get_target_extents(
        &mut wrapper,
        (*params).surface_is_unbounded,
        &mut extents,
    ) == 0)
    {
        (*surface).has_bilevel_alpha = 1 as libc::c_int;
        (*surface).has_only_op_over = 1 as libc::c_int;
        num_elements = (*surface).commands.num_elements;
        elements = _cairo_array_index(
            &mut (*surface).commands,
            0 as libc::c_int as libc::c_uint,
        ) as *mut *mut cairo_command_t;
        if extents.width < (*r).width || extents.height < (*r).height {
            num_elements = _cairo_recording_surface_get_visible_commands(
                surface,
                &mut extents,
            ) as libc::c_uint;
            use_indices = (num_elements != (*surface).commands.num_elements)
                as libc::c_int;
        }
        let mut current_block_87: u64;
        i = 0 as libc::c_int as libc::c_uint;
        while i < num_elements {
            let mut command: *mut cairo_command_t = *elements
                .offset(
                    (if use_indices != 0 {
                        *((*surface).indices).offset(i as isize)
                    } else {
                        i
                    }) as isize,
                );
            if !(replay_all == 0
                && (*command).header.region as libc::c_uint
                    != (*params).region as libc::c_uint)
            {
                if _cairo_rectangle_intersects(
                    &mut extents,
                    &mut (*command).header.extents,
                ) == 0
                {
                    if (*command).header.type_0 as libc::c_uint
                        != CAIRO_COMMAND_TAG as libc::c_int as libc::c_uint
                    {
                        current_block_87 = 11932355480408055363;
                    } else {
                        current_block_87 = 17184638872671510253;
                    }
                } else {
                    current_block_87 = 17184638872671510253;
                }
                match current_block_87 {
                    11932355480408055363 => {}
                    _ => {
                        match (*command).header.type_0 as libc::c_uint {
                            0 => {
                                status = _cairo_surface_wrapper_paint(
                                    &mut wrapper,
                                    (*command).header.op,
                                    &mut (*command).paint.source.base,
                                    (*command).header.clip,
                                ) as cairo_int_status_t;
                                if (*params).type_0 as libc::c_uint
                                    == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                        as libc::c_uint
                                {
                                    _cairo_recording_surface_merge_source_attributes(
                                        surface,
                                        (*command).header.op,
                                        &mut (*command).paint.source.base,
                                    );
                                }
                            }
                            1 => {
                                status = _cairo_surface_wrapper_mask(
                                    &mut wrapper,
                                    (*command).header.op,
                                    &mut (*command).mask.source.base,
                                    &mut (*command).mask.mask.base,
                                    (*command).header.clip,
                                ) as cairo_int_status_t;
                                if (*params).type_0 as libc::c_uint
                                    == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                        as libc::c_uint
                                {
                                    _cairo_recording_surface_merge_source_attributes(
                                        surface,
                                        (*command).header.op,
                                        &mut (*command).mask.source.base,
                                    );
                                    _cairo_recording_surface_merge_source_attributes(
                                        surface,
                                        (*command).header.op,
                                        &mut (*command).mask.mask.base,
                                    );
                                }
                            }
                            2 => {
                                status = _cairo_surface_wrapper_stroke(
                                    &mut wrapper,
                                    (*command).header.op,
                                    &mut (*command).stroke.source.base,
                                    &mut (*command).stroke.path,
                                    &mut (*command).stroke.style,
                                    &mut (*command).stroke.ctm,
                                    &mut (*command).stroke.ctm_inverse,
                                    (*command).stroke.tolerance,
                                    (*command).stroke.antialias,
                                    (*command).header.clip,
                                ) as cairo_int_status_t;
                                if (*params).type_0 as libc::c_uint
                                    == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                        as libc::c_uint
                                {
                                    _cairo_recording_surface_merge_source_attributes(
                                        surface,
                                        (*command).header.op,
                                        &mut (*command).stroke.source.base,
                                    );
                                }
                            }
                            3 => {
                                status = CAIRO_INT_STATUS_UNSUPPORTED;
                                if _cairo_surface_wrapper_has_fill_stroke(&mut wrapper) != 0
                                {
                                    let mut stroke_command: *mut cairo_command_t = 0
                                        as *mut cairo_command_t;
                                    stroke_command = 0 as *mut cairo_command_t;
                                    if (*params).type_0 as libc::c_uint
                                        != CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                            as libc::c_uint
                                        && i
                                            < num_elements
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    {
                                        stroke_command = *elements
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                            );
                                    }
                                    if !stroke_command.is_null()
                                        && (*params).type_0 as libc::c_uint
                                            == CAIRO_RECORDING_REPLAY as libc::c_int as libc::c_uint
                                        && (*params).region as libc::c_uint
                                            != CAIRO_RECORDING_REGION_ALL as libc::c_int as libc::c_uint
                                    {
                                        if (*stroke_command).header.region as libc::c_uint
                                            != (*params).region as libc::c_uint
                                        {
                                            stroke_command = 0 as *mut cairo_command_t;
                                        }
                                    }
                                    if !stroke_command.is_null()
                                        && (*stroke_command).header.type_0 as libc::c_uint
                                            == CAIRO_COMMAND_STROKE as libc::c_int as libc::c_uint
                                        && _cairo_path_fixed_equal(
                                            &mut (*command).fill.path,
                                            &mut (*stroke_command).stroke.path,
                                        ) != 0
                                        && _cairo_clip_equal(
                                            (*command).header.clip,
                                            (*stroke_command).header.clip,
                                        ) != 0
                                    {
                                        status = _cairo_surface_wrapper_fill_stroke(
                                            &mut wrapper,
                                            (*command).header.op,
                                            &mut (*command).fill.source.base,
                                            (*command).fill.fill_rule,
                                            (*command).fill.tolerance,
                                            (*command).fill.antialias,
                                            &mut (*command).fill.path,
                                            (*stroke_command).header.op,
                                            &mut (*stroke_command).stroke.source.base,
                                            &mut (*stroke_command).stroke.style,
                                            &mut (*stroke_command).stroke.ctm,
                                            &mut (*stroke_command).stroke.ctm_inverse,
                                            (*stroke_command).stroke.tolerance,
                                            (*stroke_command).stroke.antialias,
                                            (*command).header.clip,
                                        ) as cairo_int_status_t;
                                        if (*params).type_0 as libc::c_uint
                                            == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                                as libc::c_uint
                                        {
                                            _cairo_recording_surface_merge_source_attributes(
                                                surface,
                                                (*command).header.op,
                                                &mut (*command).fill.source.base,
                                            );
                                            _cairo_recording_surface_merge_source_attributes(
                                                surface,
                                                (*command).header.op,
                                                &mut (*command).stroke.source.base,
                                            );
                                        }
                                        i = i.wrapping_add(1);
                                    }
                                }
                                if status as libc::c_uint
                                    == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                        as libc::c_uint
                                {
                                    status = _cairo_surface_wrapper_fill(
                                        &mut wrapper,
                                        (*command).header.op,
                                        &mut (*command).fill.source.base,
                                        &mut (*command).fill.path,
                                        (*command).fill.fill_rule,
                                        (*command).fill.tolerance,
                                        (*command).fill.antialias,
                                        (*command).header.clip,
                                    ) as cairo_int_status_t;
                                    if (*params).type_0 as libc::c_uint
                                        == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                            as libc::c_uint
                                    {
                                        _cairo_recording_surface_merge_source_attributes(
                                            surface,
                                            (*command).header.op,
                                            &mut (*command).fill.source.base,
                                        );
                                    }
                                }
                            }
                            4 => {
                                status = _cairo_surface_wrapper_show_text_glyphs(
                                    &mut wrapper,
                                    (*command).header.op,
                                    &mut (*command).show_text_glyphs.source.base,
                                    (*command).show_text_glyphs.utf8,
                                    (*command).show_text_glyphs.utf8_len,
                                    (*command).show_text_glyphs.glyphs,
                                    (*command).show_text_glyphs.num_glyphs as libc::c_int,
                                    (*command).show_text_glyphs.clusters,
                                    (*command).show_text_glyphs.num_clusters,
                                    (*command).show_text_glyphs.cluster_flags,
                                    (*command).show_text_glyphs.scaled_font,
                                    (*command).header.clip,
                                ) as cairo_int_status_t;
                                if (*params).type_0 as libc::c_uint
                                    == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                        as libc::c_uint
                                {
                                    _cairo_recording_surface_merge_source_attributes(
                                        surface,
                                        (*command).header.op,
                                        &mut (*command).show_text_glyphs.source.base,
                                    );
                                }
                            }
                            5 => {
                                status = _cairo_surface_wrapper_tag(
                                    &mut wrapper,
                                    (*command).tag.begin,
                                    (*command).tag.tag_name,
                                    (*command).tag.attributes,
                                ) as cairo_int_status_t;
                            }
                            _ => {
                                if (b"reached\0" as *const u8 as *const libc::c_char)
                                    .is_null()
                                {} else {
                                    __assert_fail(
                                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                                        b"../src/cairo-recording-surface.c\0" as *const u8
                                            as *const libc::c_char,
                                        1999 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 128],
                                            &[libc::c_char; 128],
                                        >(
                                            b"cairo_status_t _cairo_recording_surface_replay_internal(cairo_recording_surface_t *, cairo_recording_surface_replay_params_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            }
                        }
                        if status as libc::c_uint
                            == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int
                                as libc::c_uint
                        {
                            status = CAIRO_INT_STATUS_SUCCESS;
                        }
                        if (*params).type_0 as libc::c_uint
                            == CAIRO_RECORDING_CREATE_REGIONS as libc::c_int
                                as libc::c_uint
                            && (*command).header.region as libc::c_uint
                                != CAIRO_RECORDING_REGION_NATIVE as libc::c_int
                                    as libc::c_uint
                        {
                            if status as libc::c_uint
                                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                            {
                                (*command).header.region = CAIRO_RECORDING_REGION_NATIVE;
                            } else if status as libc::c_uint
                                == CAIRO_INT_STATUS_IMAGE_FALLBACK as libc::c_int
                                    as libc::c_uint
                            {
                                (*command)
                                    .header
                                    .region = CAIRO_RECORDING_REGION_IMAGE_FALLBACK;
                                status = CAIRO_INT_STATUS_SUCCESS;
                            } else if status as libc::c_uint
                                != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                                && (status as libc::c_uint)
                                    < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int
                                        as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"_cairo_int_status_is_error (status)\0" as *const u8
                                        as *const libc::c_char,
                                    b"../src/cairo-recording-surface.c\0" as *const u8
                                        as *const libc::c_char,
                                    2013 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 128],
                                        &[libc::c_char; 128],
                                    >(
                                        b"cairo_status_t _cairo_recording_surface_replay_internal(cairo_recording_surface_t *, cairo_recording_surface_replay_params_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        }
                        if status as u64 != 0 {
                            break;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    _cairo_surface_wrapper_fini(&mut wrapper);
    return _cairo_surface_set_error(&mut (*surface).base, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_replay_one(
    mut surface: *mut cairo_recording_surface_t,
    mut index: libc::c_ulong,
    mut target: *mut cairo_surface_t,
) -> cairo_status_t {
    let mut wrapper: cairo_surface_wrapper_t = cairo_surface_wrapper_t {
        target: 0 as *mut cairo_surface_t,
        transform: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        has_extents: 0,
        extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        clip: 0 as *const cairo_clip_t,
        foreground_source: 0 as *mut cairo_pattern_t,
        needs_transform: 0,
    };
    let mut elements: *mut *mut cairo_command_t = 0 as *mut *mut cairo_command_t;
    let mut command: *mut cairo_command_t = 0 as *mut cairo_command_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).base.status as u64 != 0 {
        return (*surface).base.status;
    }
    if (*target).status as u64 != 0 {
        return (*target).status;
    }
    if ((*surface).base).finished() != 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    if _cairo_surface_is_recording(&mut (*surface).base) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_is_recording (&surface->base)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-recording-surface.c\0" as *const u8 as *const libc::c_char,
            2044 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"cairo_status_t _cairo_recording_surface_replay_one(cairo_recording_surface_t *, unsigned long, cairo_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_surface_wrapper_init(&mut wrapper, target);
    if index > (*surface).commands.num_elements as libc::c_ulong {
        return _cairo_error(CAIRO_STATUS_READ_ERROR);
    }
    elements = _cairo_array_index(
        &mut (*surface).commands,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut cairo_command_t;
    command = *elements.offset(index as isize);
    match (*command).header.type_0 as libc::c_uint {
        0 => {
            status = _cairo_surface_wrapper_paint(
                &mut wrapper,
                (*command).header.op,
                &mut (*command).paint.source.base,
                (*command).header.clip,
            ) as cairo_int_status_t;
        }
        1 => {
            status = _cairo_surface_wrapper_mask(
                &mut wrapper,
                (*command).header.op,
                &mut (*command).mask.source.base,
                &mut (*command).mask.mask.base,
                (*command).header.clip,
            ) as cairo_int_status_t;
        }
        2 => {
            status = _cairo_surface_wrapper_stroke(
                &mut wrapper,
                (*command).header.op,
                &mut (*command).stroke.source.base,
                &mut (*command).stroke.path,
                &mut (*command).stroke.style,
                &mut (*command).stroke.ctm,
                &mut (*command).stroke.ctm_inverse,
                (*command).stroke.tolerance,
                (*command).stroke.antialias,
                (*command).header.clip,
            ) as cairo_int_status_t;
        }
        3 => {
            status = _cairo_surface_wrapper_fill(
                &mut wrapper,
                (*command).header.op,
                &mut (*command).fill.source.base,
                &mut (*command).fill.path,
                (*command).fill.fill_rule,
                (*command).fill.tolerance,
                (*command).fill.antialias,
                (*command).header.clip,
            ) as cairo_int_status_t;
        }
        4 => {
            status = _cairo_surface_wrapper_show_text_glyphs(
                &mut wrapper,
                (*command).header.op,
                &mut (*command).show_text_glyphs.source.base,
                (*command).show_text_glyphs.utf8,
                (*command).show_text_glyphs.utf8_len,
                (*command).show_text_glyphs.glyphs,
                (*command).show_text_glyphs.num_glyphs as libc::c_int,
                (*command).show_text_glyphs.clusters,
                (*command).show_text_glyphs.num_clusters,
                (*command).show_text_glyphs.cluster_flags,
                (*command).show_text_glyphs.scaled_font,
                (*command).header.clip,
            ) as cairo_int_status_t;
        }
        5 => {
            status = _cairo_surface_wrapper_tag(
                &mut wrapper,
                (*command).tag.begin,
                (*command).tag.tag_name,
                (*command).tag.attributes,
            ) as cairo_int_status_t;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-recording-surface.c\0" as *const u8
                        as *const libc::c_char,
                    2117 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 114],
                        &[libc::c_char; 114],
                    >(
                        b"cairo_status_t _cairo_recording_surface_replay_one(cairo_recording_surface_t *, unsigned long, cairo_surface_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    _cairo_surface_wrapper_fini(&mut wrapper);
    return _cairo_surface_set_error(&mut (*surface).base, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_replay(
    mut surface: *mut cairo_surface_t,
    mut target: *mut cairo_surface_t,
) -> cairo_status_t {
    let mut params: cairo_recording_surface_replay_params_t = cairo_recording_surface_replay_params_t {
        surface_extents: 0 as *const cairo_rectangle_int_t,
        surface_transform: 0 as *const cairo_matrix_t,
        target: 0 as *mut cairo_surface_t,
        target_clip: 0 as *const cairo_clip_t,
        surface_is_unbounded: 0,
        type_0: CAIRO_RECORDING_REPLAY,
        region: CAIRO_RECORDING_REGION_ALL,
        foreground_color: 0 as *const cairo_color_t,
    };
    params.surface_extents = 0 as *const cairo_rectangle_int_t;
    params.surface_transform = 0 as *const cairo_matrix_t;
    params.target = target;
    params.target_clip = 0 as *const cairo_clip_t;
    params.surface_is_unbounded = 0 as libc::c_int;
    params.type_0 = CAIRO_RECORDING_REPLAY;
    params.region = CAIRO_RECORDING_REGION_ALL;
    params.foreground_color = 0 as *const cairo_color_t;
    return _cairo_recording_surface_replay_internal(
        surface as *mut cairo_recording_surface_t,
        &mut params,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_replay_with_foreground_color(
    mut surface: *mut cairo_surface_t,
    mut target: *mut cairo_surface_t,
    mut color: *const cairo_color_t,
) -> cairo_status_t {
    let mut params: cairo_recording_surface_replay_params_t = cairo_recording_surface_replay_params_t {
        surface_extents: 0 as *const cairo_rectangle_int_t,
        surface_transform: 0 as *const cairo_matrix_t,
        target: 0 as *mut cairo_surface_t,
        target_clip: 0 as *const cairo_clip_t,
        surface_is_unbounded: 0,
        type_0: CAIRO_RECORDING_REPLAY,
        region: CAIRO_RECORDING_REGION_ALL,
        foreground_color: 0 as *const cairo_color_t,
    };
    params.surface_extents = 0 as *const cairo_rectangle_int_t;
    params.surface_transform = 0 as *const cairo_matrix_t;
    params.target = target;
    params.target_clip = 0 as *const cairo_clip_t;
    params.surface_is_unbounded = 0 as libc::c_int;
    params.type_0 = CAIRO_RECORDING_REPLAY;
    params.region = CAIRO_RECORDING_REGION_ALL;
    params.foreground_color = color;
    return _cairo_recording_surface_replay_internal(
        surface as *mut cairo_recording_surface_t,
        &mut params,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_replay_with_clip(
    mut surface: *mut cairo_surface_t,
    mut surface_transform: *const cairo_matrix_t,
    mut target: *mut cairo_surface_t,
    mut target_clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut params: cairo_recording_surface_replay_params_t = cairo_recording_surface_replay_params_t {
        surface_extents: 0 as *const cairo_rectangle_int_t,
        surface_transform: 0 as *const cairo_matrix_t,
        target: 0 as *mut cairo_surface_t,
        target_clip: 0 as *const cairo_clip_t,
        surface_is_unbounded: 0,
        type_0: CAIRO_RECORDING_REPLAY,
        region: CAIRO_RECORDING_REGION_ALL,
        foreground_color: 0 as *const cairo_color_t,
    };
    params.surface_extents = 0 as *const cairo_rectangle_int_t;
    params.surface_transform = surface_transform;
    params.target = target;
    params.target_clip = target_clip;
    params.surface_is_unbounded = 0 as libc::c_int;
    params.type_0 = CAIRO_RECORDING_REPLAY;
    params.region = CAIRO_RECORDING_REGION_ALL;
    params.foreground_color = 0 as *const cairo_color_t;
    return _cairo_recording_surface_replay_internal(
        surface as *mut cairo_recording_surface_t,
        &mut params,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_replay_and_create_regions(
    mut surface: *mut cairo_surface_t,
    mut surface_transform: *const cairo_matrix_t,
    mut target: *mut cairo_surface_t,
    mut surface_is_unbounded: cairo_bool_t,
) -> cairo_status_t {
    let mut params: cairo_recording_surface_replay_params_t = cairo_recording_surface_replay_params_t {
        surface_extents: 0 as *const cairo_rectangle_int_t,
        surface_transform: 0 as *const cairo_matrix_t,
        target: 0 as *mut cairo_surface_t,
        target_clip: 0 as *const cairo_clip_t,
        surface_is_unbounded: 0,
        type_0: CAIRO_RECORDING_REPLAY,
        region: CAIRO_RECORDING_REGION_ALL,
        foreground_color: 0 as *const cairo_color_t,
    };
    params.surface_extents = 0 as *const cairo_rectangle_int_t;
    params.surface_transform = surface_transform;
    params.target = target;
    params.target_clip = 0 as *const cairo_clip_t;
    params.surface_is_unbounded = surface_is_unbounded;
    params.type_0 = CAIRO_RECORDING_CREATE_REGIONS;
    params.region = CAIRO_RECORDING_REGION_ALL;
    params.foreground_color = 0 as *const cairo_color_t;
    return _cairo_recording_surface_replay_internal(
        surface as *mut cairo_recording_surface_t,
        &mut params,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_replay_region(
    mut surface: *mut cairo_surface_t,
    mut surface_extents: *const cairo_rectangle_int_t,
    mut target: *mut cairo_surface_t,
    mut region: cairo_recording_region_type_t,
) -> cairo_status_t {
    let mut params: cairo_recording_surface_replay_params_t = cairo_recording_surface_replay_params_t {
        surface_extents: 0 as *const cairo_rectangle_int_t,
        surface_transform: 0 as *const cairo_matrix_t,
        target: 0 as *mut cairo_surface_t,
        target_clip: 0 as *const cairo_clip_t,
        surface_is_unbounded: 0,
        type_0: CAIRO_RECORDING_REPLAY,
        region: CAIRO_RECORDING_REGION_ALL,
        foreground_color: 0 as *const cairo_color_t,
    };
    params.surface_extents = surface_extents;
    params.surface_transform = 0 as *const cairo_matrix_t;
    params.target = target;
    params.target_clip = 0 as *const cairo_clip_t;
    params.surface_is_unbounded = 0 as libc::c_int;
    params.type_0 = CAIRO_RECORDING_REPLAY;
    params.region = region;
    params.foreground_color = 0 as *const cairo_color_t;
    return _cairo_recording_surface_replay_internal(
        surface as *mut cairo_recording_surface_t,
        &mut params,
    );
}
unsafe extern "C" fn _recording_surface_get_ink_bbox(
    mut surface: *mut cairo_recording_surface_t,
    mut bbox: *mut cairo_box_t,
    mut transform: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut null_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut analysis_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    null_surface = _cairo_null_surface_create((*surface).base.content);
    analysis_surface = _cairo_analysis_surface_create(null_surface);
    cairo_surface_destroy(null_surface);
    status = (*analysis_surface).status;
    if status as u64 != 0 {
        return status;
    }
    if !transform.is_null() {
        _cairo_analysis_surface_set_ctm(analysis_surface, transform);
    }
    status = _cairo_recording_surface_replay(&mut (*surface).base, analysis_surface);
    _cairo_analysis_surface_get_bounding_box(analysis_surface, bbox);
    cairo_surface_destroy(analysis_surface);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_recording_surface_ink_extents(
    mut surface: *mut cairo_surface_t,
    mut x0: *mut libc::c_double,
    mut y0: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut height: *mut libc::c_double,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut bbox: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    memset(
        &mut bbox as *mut cairo_box_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_box_t>() as libc::c_ulong,
    );
    if (*surface).status as libc::c_uint != 0
        || _cairo_surface_is_recording(surface) == 0
    {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
    } else {
        status = _recording_surface_get_ink_bbox(
            surface as *mut cairo_recording_surface_t,
            &mut bbox,
            0 as *const cairo_matrix_t,
        );
        if status as u64 != 0 {
            status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
                as cairo_status_t;
        }
    }
    if !x0.is_null() {
        *x0 = _cairo_fixed_to_double(bbox.p1.x);
    }
    if !y0.is_null() {
        *y0 = _cairo_fixed_to_double(bbox.p1.y);
    }
    if !width.is_null() {
        *width = _cairo_fixed_to_double(bbox.p2.x - bbox.p1.x);
    }
    if !height.is_null() {
        *height = _cairo_fixed_to_double(bbox.p2.y - bbox.p1.y);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_get_bbox(
    mut surface: *mut cairo_recording_surface_t,
    mut bbox: *mut cairo_box_t,
    mut transform: *const cairo_matrix_t,
) -> cairo_status_t {
    if (*surface).unbounded == 0 {
        _cairo_box_from_rectangle(bbox, &mut (*surface).extents);
        if !transform.is_null() {
            _cairo_matrix_transform_bounding_box_fixed(
                transform,
                bbox,
                0 as *mut cairo_bool_t,
            );
        }
        return CAIRO_STATUS_SUCCESS;
    }
    return _recording_surface_get_ink_bbox(surface, bbox, transform);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_get_ink_bbox(
    mut surface: *mut cairo_recording_surface_t,
    mut bbox: *mut cairo_box_t,
    mut transform: *const cairo_matrix_t,
) -> cairo_status_t {
    return _recording_surface_get_ink_bbox(surface, bbox, transform);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_recording_surface_get_extents(
    mut surface: *mut cairo_surface_t,
    mut extents: *mut cairo_rectangle_t,
) -> cairo_bool_t {
    let mut record: *mut cairo_recording_surface_t = 0 as *mut cairo_recording_surface_t;
    if (*surface).status as libc::c_uint != 0
        || _cairo_surface_is_recording(surface) == 0
    {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    record = surface as *mut cairo_recording_surface_t;
    if (*record).unbounded != 0 {
        return 0 as libc::c_int;
    }
    *extents = (*record).extents_pixels;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_has_only_bilevel_alpha(
    mut surface: *mut cairo_recording_surface_t,
) -> cairo_bool_t {
    return (*surface).has_bilevel_alpha;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_recording_surface_has_only_op_over(
    mut surface: *mut cairo_recording_surface_t,
) -> cairo_bool_t {
    return (*surface).has_only_op_over;
}
