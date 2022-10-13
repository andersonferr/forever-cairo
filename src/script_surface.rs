use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn cairo_font_options_equal(
        options: *const cairo_font_options_t,
        other: *const cairo_font_options_t,
    ) -> cairo_bool_t;
    fn cairo_scaled_font_get_font_matrix(
        scaled_font: *mut cairo_scaled_font_t,
        font_matrix: *mut cairo_matrix_t,
    );
    fn cairo_scaled_font_get_ctm(
        scaled_font: *mut cairo_scaled_font_t,
        ctm: *mut cairo_matrix_t,
    );
    fn cairo_scaled_font_get_font_options(
        scaled_font: *mut cairo_scaled_font_t,
        options: *mut cairo_font_options_t,
    );
    fn cairo_path_destroy(path: *mut cairo_path_t);
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_get_mime_data(
        surface: *mut cairo_surface_t,
        mime_type: *const libc::c_char,
        data: *mut *const libc::c_uchar,
        length: *mut libc::c_ulong,
    );
    fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn cairo_mesh_pattern_get_patch_count(
        pattern: *mut cairo_pattern_t,
        count: *mut libc::c_uint,
    ) -> cairo_status_t;
    fn cairo_mesh_pattern_get_path(
        pattern: *mut cairo_pattern_t,
        patch_num: libc::c_uint,
    ) -> *mut cairo_path_t;
    fn cairo_mesh_pattern_get_corner_color_rgba(
        pattern: *mut cairo_pattern_t,
        patch_num: libc::c_uint,
        corner_num: libc::c_uint,
        red: *mut libc::c_double,
        green: *mut libc::c_double,
        blue: *mut libc::c_double,
        alpha: *mut libc::c_double,
    ) -> cairo_status_t;
    fn cairo_mesh_pattern_get_control_point(
        pattern: *mut cairo_pattern_t,
        patch_num: libc::c_uint,
        point_num: libc::c_uint,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    ) -> cairo_status_t;
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
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
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_init(boxes: *mut cairo_boxes_t);
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_fini(boxes: *mut cairo_boxes_t);
    fn _cairo_path_fixed_equal(
        a: *const cairo_path_fixed_t,
        b: *const cairo_path_fixed_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_iter_init(
        iter: *mut cairo_path_fixed_iter_t,
        path: *const cairo_path_fixed_t,
    );
    fn _cairo_path_fixed_iter_is_fill_box(
        _iter: *mut cairo_path_fixed_iter_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_iter_at_end(
        iter: *const cairo_path_fixed_iter_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_scaled_font_find_private(
        scaled_font: *mut cairo_scaled_font_t,
        key: *const libc::c_void,
    ) -> *mut cairo_scaled_font_private_t;
    fn _cairo_scaled_font_attach_private(
        scaled_font: *mut cairo_scaled_font_t,
        priv_0: *mut cairo_scaled_font_private_t,
        key: *const libc::c_void,
        destroy: Option::<
            unsafe extern "C" fn(
                *mut cairo_scaled_font_private_t,
                *mut cairo_scaled_font_t,
            ) -> (),
        >,
    );
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_path_fixed_init(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_init_copy(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_is_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_is_rectangle(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_stroke_style_init(style: *mut cairo_stroke_style_t);
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_surface_acquire_source_image(
        surface: *mut cairo_surface_t,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_surface_release_source_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
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
    fn _cairo_image_surface_coerce(
        surface: *mut cairo_image_surface_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_output_stream_create(
        write_func: cairo_write_func_t,
        close_func: cairo_close_func_t,
        closure: *mut libc::c_void,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_flush(stream: *mut cairo_output_stream_t) -> cairo_status_t;
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
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_create_for_filename(
        filename: *const libc::c_char,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_base85_stream_create(
        output: *mut cairo_output_stream_t,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_deflate_stream_create(
        output: *mut cairo_output_stream_t,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_device_create_in_error(status: cairo_status_t) -> *mut cairo_device_t;
    fn _cairo_device_init(
        device: *mut cairo_device_t,
        backend: *const cairo_device_backend_t,
    );
    fn _cairo_pattern_init_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_init_solid(
        pattern: *mut cairo_solid_pattern_t,
        color: *const cairo_color_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_equal(
        a: *const cairo_pattern_t,
        b: *const cairo_pattern_t,
    ) -> cairo_bool_t;
    fn _cairo_raster_source_pattern_acquire(
        abstract_pattern: *const cairo_pattern_t,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_raster_source_pattern_release(
        abstract_pattern: *const cairo_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_recording_surface_replay(
        surface: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_set_clip(
        clipper: *mut cairo_surface_clipper_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_init(
        clipper: *mut cairo_surface_clipper_t,
        intersect: cairo_surface_clipper_intersect_clip_path_func_t,
    );
    fn _cairo_surface_clipper_reset(clipper: *mut cairo_surface_clipper_t);
    fn _cairo_surface_wrapper_init(
        wrapper: *mut cairo_surface_wrapper_t,
        target: *mut cairo_surface_t,
    );
    fn _cairo_surface_wrapper_fini(wrapper: *mut cairo_surface_wrapper_t);
    fn _cairo_surface_wrapper_acquire_source_image(
        wrapper: *mut cairo_surface_wrapper_t,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_surface_wrapper_release_source_image(
        wrapper: *mut cairo_surface_wrapper_t,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
    );
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
    fn _cairo_surface_wrapper_create_similar(
        wrapper: *mut cairo_surface_wrapper_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_wrapper_get_extents(
        wrapper: *mut cairo_surface_wrapper_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_wrapper_snapshot(
        wrapper: *mut cairo_surface_wrapper_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_ft_scaled_font_get_load_flags(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_device_backend_t,
    pub mutex: cairo_recursive_mutex_t,
    pub mutex_depth: libc::c_uint,
    pub finished: cairo_bool_t,
}
pub type cairo_recursive_mutex_t = cairo_recursive_mutex_impl_t;
pub type cairo_recursive_mutex_impl_t = pthread_mutex_t;
pub type cairo_device_backend_t = _cairo_device_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device_backend {
    pub type_0: cairo_device_type_t,
    pub lock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub unlock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type cairo_device_type_t = _cairo_device_type;
pub type _cairo_device_type = libc::c_int;
pub const CAIRO_DEVICE_TYPE_INVALID: _cairo_device_type = -1;
pub const CAIRO_DEVICE_TYPE_WIN32: _cairo_device_type = 7;
pub const CAIRO_DEVICE_TYPE_COGL: _cairo_device_type = 6;
pub const CAIRO_DEVICE_TYPE_XML: _cairo_device_type = 5;
pub const CAIRO_DEVICE_TYPE_XLIB: _cairo_device_type = 4;
pub const CAIRO_DEVICE_TYPE_XCB: _cairo_device_type = 3;
pub const CAIRO_DEVICE_TYPE_SCRIPT: _cairo_device_type = 2;
pub const CAIRO_DEVICE_TYPE_GL: _cairo_device_type = 1;
pub const CAIRO_DEVICE_TYPE_DRM: _cairo_device_type = 0;
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
pub type cairo_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
pub type _cairo_path_data_type = libc::c_uint;
pub const CAIRO_PATH_CLOSE_PATH: _cairo_path_data_type = 3;
pub const CAIRO_PATH_CURVE_TO: _cairo_path_data_type = 2;
pub const CAIRO_PATH_LINE_TO: _cairo_path_data_type = 1;
pub const CAIRO_PATH_MOVE_TO: _cairo_path_data_type = 0;
pub type cairo_path_data_type_t = _cairo_path_data_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _cairo_path_data_t {
    pub header: C2RustUnnamed_0,
    pub point: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub type_0: cairo_path_data_type_t,
    pub length: libc::c_int,
}
pub type cairo_path_data_t = _cairo_path_data_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_path {
    pub status: cairo_status_t,
    pub data: *mut cairo_path_data_t,
    pub num_data: libc::c_int,
}
pub type cairo_path_t = cairo_path;
pub type cairo_raster_source_acquire_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
        *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t,
>;
pub type cairo_raster_source_release_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
    ) -> (),
>;
pub type cairo_raster_source_snapshot_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> cairo_status_t,
>;
pub type cairo_raster_source_copy_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *const cairo_pattern_t,
    ) -> cairo_status_t,
>;
pub type cairo_raster_source_finish_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> (),
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_t {
    pub status: cairo_status_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_boxes: libc::c_int,
    pub is_pixel_aligned: libc::c_uint,
    pub chunks: _cairo_boxes_chunk,
    pub tail: *mut _cairo_boxes_chunk,
    pub boxes_embedded: [cairo_box_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_chunk {
    pub next: *mut _cairo_boxes_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_boxes_t = _cairo_boxes_t;
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
pub type cairo_color_stop_t = _cairo_color_stop;
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
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_snapshot {
    pub base: cairo_surface_t,
    pub mutex: cairo_mutex_t,
    pub target: *mut cairo_surface_t,
    pub clone: *mut cairo_surface_t,
}
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_surface_snapshot_t = _cairo_surface_snapshot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_subsurface {
    pub base: cairo_surface_t,
    pub extents: cairo_rectangle_int_t,
    pub target: *mut cairo_surface_t,
    pub snapshot: *mut cairo_surface_t,
}
pub type cairo_surface_subsurface_t = _cairo_surface_subsurface;
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
pub type cairo_surface_wrapper_t = _cairo_surface_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_private {
    pub link: cairo_list_t,
    pub key: *const libc::c_void,
    pub destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_font_private_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
}
pub type cairo_scaled_font_private_t = _cairo_scaled_font_private;
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
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_circle_double {
    pub center: cairo_point_double_t,
    pub radius: libc::c_double,
}
pub type cairo_circle_double_t = _cairo_circle_double;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_fixed_iter {
    pub first: *const cairo_path_buf_t,
    pub buf: *const cairo_path_buf_t,
    pub n_op: libc::c_uint,
    pub n_point: libc::c_uint,
}
pub type cairo_path_fixed_iter_t = _cairo_path_fixed_iter;
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
pub union C2RustUnnamed_1 {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
pub type cairo_script_mode_t = libc::c_uint;
pub const CAIRO_SCRIPT_MODE_BINARY: cairo_script_mode_t = 1;
pub const CAIRO_SCRIPT_MODE_ASCII: cairo_script_mode_t = 0;
pub type cairo_script_context_t = _cairo_script_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_script_context {
    pub base: cairo_device_t,
    pub active: libc::c_int,
    pub attach_snapshots: libc::c_int,
    pub owns_stream: cairo_bool_t,
    pub stream: *mut cairo_output_stream_t,
    pub mode: cairo_script_mode_t,
    pub surface_id: _bitmap,
    pub font_id: _bitmap,
    pub operands: cairo_list_t,
    pub deferred: cairo_list_t,
    pub fonts: cairo_list_t,
    pub defines: cairo_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bitmap {
    pub min: libc::c_ulong,
    pub count: libc::c_ulong,
    pub map: [libc::c_uint; 64],
    pub next: *mut _bitmap,
}
pub type cairo_script_font_t = _cairo_script_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_script_font {
    pub base: cairo_scaled_font_private_t,
    pub has_sfnt: cairo_bool_t,
    pub id: libc::c_ulong,
    pub subset_glyph_index: libc::c_ulong,
    pub link: cairo_list_t,
    pub parent: *mut cairo_scaled_font_t,
}
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
pub type cairo_script_surface_t = _cairo_script_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_script_surface {
    pub base: cairo_surface_t,
    pub wrapper: cairo_surface_wrapper_t,
    pub clipper: cairo_surface_clipper_t,
    pub operand: operand_t,
    pub emitted: cairo_bool_t,
    pub defined: cairo_bool_t,
    pub active: cairo_bool_t,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub cr: cairo_script_implicit_context_t,
}
pub type cairo_script_implicit_context_t = _cairo_script_implicit_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_script_implicit_context {
    pub current_operator: cairo_operator_t,
    pub current_fill_rule: cairo_fill_rule_t,
    pub current_tolerance: libc::c_double,
    pub current_antialias: cairo_antialias_t,
    pub current_style: cairo_stroke_style_t,
    pub current_source: cairo_pattern_union_t,
    pub current_ctm: cairo_matrix_t,
    pub current_stroke_matrix: cairo_matrix_t,
    pub current_font_matrix: cairo_matrix_t,
    pub current_font_options: cairo_font_options_t,
    pub current_scaled_font: *mut cairo_scaled_font_t,
    pub current_path: cairo_path_fixed_t,
    pub has_clip: cairo_bool_t,
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
pub type operand_t = _operand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _operand {
    pub type_0: C2RustUnnamed_2,
    pub link: cairo_list_t,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DEFERRED: C2RustUnnamed_2 = 1;
pub const SURFACE: C2RustUnnamed_2 = 0;
pub type cairo_surface_clipper_t = _cairo_surface_clipper;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferred_finish {
    pub link: cairo_list_t,
    pub operand: operand_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct script_snapshot {
    pub base: cairo_surface_t,
}
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
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline]
unsafe extern "C" fn _cairo_isprint(mut c: libc::c_int) -> libc::c_int {
    return (c >= 0x20 as libc::c_int && c <= 0x7e as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_fill_is_rectilinear(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    if (*path).fill_is_rectilinear() == 0 {
        return 0 as libc::c_int;
    }
    if (*path).has_current_point() == 0 || (*path).needs_move_to() as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return ((*path).current_point.x == (*path).last_move_point.x
        || (*path).current_point.y == (*path).last_move_point.y) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_fill_maybe_region(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    if (*path).fill_maybe_region() == 0 {
        return 0 as libc::c_int;
    }
    if (*path).has_current_point() == 0 || (*path).needs_move_to() as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return ((*path).current_point.x == (*path).last_move_point.x
        || (*path).current_point.y == (*path).last_move_point.y) as libc::c_int;
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
unsafe extern "C" fn _cairo_fixed_from_double(mut d: libc::c_double) -> cairo_fixed_t {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { d: 0. };
    u
        .d = d
        + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
            as libc::c_double * 1.5f64;
    return u.i[0 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh2 = (*entry).next;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).prev;
    *fresh3 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh4 = (*next).prev;
    *fresh4 = entry;
    let ref mut fresh5 = (*entry).next;
    *fresh5 = next;
    let ref mut fresh6 = (*entry).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh8 = (*next).prev;
    *fresh8 = prev;
    let ref mut fresh9 = (*prev).next;
    *fresh9 = next;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn cairo_list_del(mut entry: *mut cairo_list_t) {
    _cairo_list_del(entry);
    cairo_list_init(entry);
}
#[inline]
unsafe extern "C" fn cairo_list_move(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_del((*entry).prev, (*entry).next);
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn cairo_list_move_tail(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_del((*entry).prev, (*entry).next);
    __cairo_list_add(entry, (*head).prev, head);
}
#[inline]
unsafe extern "C" fn cairo_list_swap(
    mut entry: *mut cairo_list_t,
    mut other: *mut cairo_list_t,
) {
    __cairo_list_add(entry, (*other).prev, (*other).next);
    cairo_list_init(other);
}
#[inline]
unsafe extern "C" fn cairo_list_is_first(
    mut entry: *const cairo_list_t,
    mut head: *const cairo_list_t,
) -> cairo_bool_t {
    return ((*entry).prev == head as *mut _cairo_list) as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_recording_surface_get_bounds(
    mut surface: *mut cairo_surface_t,
    mut extents: *mut cairo_rectangle_t,
) -> cairo_bool_t {
    let mut recording: *mut cairo_recording_surface_t = surface
        as *mut cairo_recording_surface_t;
    if (*recording).unbounded != 0 {
        return 0 as libc::c_int;
    }
    *extents = (*recording).extents_pixels;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_recording(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_snapshot_is_reused(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return (_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
        > 2 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn _cairo_surface_wrapper_is_active(
    mut wrapper: *mut cairo_surface_wrapper_t,
) -> cairo_bool_t {
    return ((*wrapper).target != 0 as *mut cairo_surface_t) as libc::c_int;
}
unsafe extern "C" fn _bitmap_release_id(mut b: *mut _bitmap, mut token: libc::c_ulong) {
    let mut prev: *mut *mut _bitmap = 0 as *mut *mut _bitmap;
    loop {
        if token
            < ((*b).min)
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_uint; 64]>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut bit: libc::c_uint = 0;
            let mut elem: libc::c_uint = 0;
            token = token.wrapping_sub((*b).min);
            elem = token
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint;
            bit = token
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint;
            (*b).map[elem as usize] &= !((1 as libc::c_int) << bit) as libc::c_uint;
            let ref mut fresh10 = (*b).count;
            *fresh10 = (*fresh10).wrapping_sub(1);
            if *fresh10 == 0 && !prev.is_null() {
                *prev = (*b).next;
                free(b as *mut libc::c_void);
            }
            return;
        }
        prev = &mut (*b).next;
        b = (*b).next;
        if b.is_null() {
            break;
        }
    };
}
unsafe extern "C" fn _bitmap_next_id(
    mut b: *mut _bitmap,
    mut id: *mut libc::c_ulong,
) -> cairo_status_t {
    let mut bb: *mut _bitmap = 0 as *mut _bitmap;
    let mut prev: *mut *mut _bitmap = 0 as *mut *mut _bitmap;
    let mut min: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while !((*b).min != min) {
        if (*b).count
            < (::std::mem::size_of::<[libc::c_uint; 64]>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            let mut n: libc::c_uint = 0;
            let mut m: libc::c_uint = 0;
            let mut bit: libc::c_uint = 0;
            n = 0 as libc::c_int as libc::c_uint;
            while n
                < (::std::mem::size_of::<[libc::c_uint; 64]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    as libc::c_int as libc::c_uint
            {
                if !((*b).map[n as usize] == -(1 as libc::c_int) as libc::c_uint) {
                    m = 0 as libc::c_int as libc::c_uint;
                    bit = 1 as libc::c_int as libc::c_uint;
                    while (m as libc::c_ulong)
                        < (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    {
                        if (*b).map[n as usize] & bit == 0 as libc::c_int as libc::c_uint
                        {
                            (*b).map[n as usize] |= bit;
                            let ref mut fresh11 = (*b).count;
                            *fresh11 = (*fresh11).wrapping_add(1);
                            *id = (n as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                )
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_add(m as libc::c_ulong)
                                .wrapping_add((*b).min);
                            return CAIRO_STATUS_SUCCESS;
                        }
                        m = m.wrapping_add(1);
                        bit <<= 1 as libc::c_int;
                    }
                }
                n = n.wrapping_add(1);
            }
        }
        min = min
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_uint; 64]>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            );
        prev = &mut (*b).next;
        b = (*b).next;
        if b.is_null() {
            break;
        }
    }
    if !prev.is_null() {} else {
        __assert_fail(
            b"prev != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"cairo_status_t _bitmap_next_id(struct _bitmap *, unsigned long *)\0"))
                .as_ptr(),
        );
    }
    bb = (if ::std::mem::size_of::<_bitmap>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<_bitmap>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut _bitmap;
    if bb.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    *prev = bb;
    let ref mut fresh12 = (*bb).next;
    *fresh12 = b;
    (*bb).min = min;
    (*bb).count = 1 as libc::c_int as libc::c_ulong;
    (*bb).map[0 as libc::c_int as usize] = 0x1 as libc::c_int as libc::c_uint;
    memset(
        ((*bb).map).as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<[libc::c_uint; 64]>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    *id = min;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _bitmap_fini(mut b: *mut _bitmap) {
    while !b.is_null() {
        let mut next: *mut _bitmap = (*b).next;
        free(b as *mut libc::c_void);
        b = next;
    }
}
unsafe extern "C" fn _direction_to_string(
    mut backward: cairo_bool_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 2] = [
        b"FORWARD\0" as *const u8 as *const libc::c_char,
        b"BACKWARD\0" as *const u8 as *const libc::c_char,
    ];
    if backward
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {} else {
        __assert_fail(
            b"backward < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"const char *_direction_to_string(cairo_bool_t)\0"))
                .as_ptr(),
        );
    }
    return names[backward as usize];
}
unsafe extern "C" fn _operator_to_string(
    mut op: cairo_operator_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 29] = [
        b"CLEAR\0" as *const u8 as *const libc::c_char,
        b"SOURCE\0" as *const u8 as *const libc::c_char,
        b"OVER\0" as *const u8 as *const libc::c_char,
        b"IN\0" as *const u8 as *const libc::c_char,
        b"OUT\0" as *const u8 as *const libc::c_char,
        b"ATOP\0" as *const u8 as *const libc::c_char,
        b"DEST\0" as *const u8 as *const libc::c_char,
        b"DEST_OVER\0" as *const u8 as *const libc::c_char,
        b"DEST_IN\0" as *const u8 as *const libc::c_char,
        b"DEST_OUT\0" as *const u8 as *const libc::c_char,
        b"DEST_ATOP\0" as *const u8 as *const libc::c_char,
        b"XOR\0" as *const u8 as *const libc::c_char,
        b"ADD\0" as *const u8 as *const libc::c_char,
        b"SATURATE\0" as *const u8 as *const libc::c_char,
        b"MULTIPLY\0" as *const u8 as *const libc::c_char,
        b"SCREEN\0" as *const u8 as *const libc::c_char,
        b"OVERLAY\0" as *const u8 as *const libc::c_char,
        b"DARKEN\0" as *const u8 as *const libc::c_char,
        b"LIGHTEN\0" as *const u8 as *const libc::c_char,
        b"DODGE\0" as *const u8 as *const libc::c_char,
        b"BURN\0" as *const u8 as *const libc::c_char,
        b"HARD_LIGHT\0" as *const u8 as *const libc::c_char,
        b"SOFT_LIGHT\0" as *const u8 as *const libc::c_char,
        b"DIFFERENCE\0" as *const u8 as *const libc::c_char,
        b"EXCLUSION\0" as *const u8 as *const libc::c_char,
        b"HSL_HUE\0" as *const u8 as *const libc::c_char,
        b"HSL_SATURATION\0" as *const u8 as *const libc::c_char,
        b"HSL_COLOR\0" as *const u8 as *const libc::c_char,
        b"HSL_LUMINOSITY\0" as *const u8 as *const libc::c_char,
    ];
    if (op as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"op < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"const char *_operator_to_string(cairo_operator_t)\0"))
                .as_ptr(),
        );
    }
    return names[op as usize];
}
unsafe extern "C" fn _extend_to_string(
    mut extend: cairo_extend_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 4] = [
        b"EXTEND_NONE\0" as *const u8 as *const libc::c_char,
        b"EXTEND_REPEAT\0" as *const u8 as *const libc::c_char,
        b"EXTEND_REFLECT\0" as *const u8 as *const libc::c_char,
        b"EXTEND_PAD\0" as *const u8 as *const libc::c_char,
    ];
    if (extend as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"extend < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            354 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"const char *_extend_to_string(cairo_extend_t)\0"))
                .as_ptr(),
        );
    }
    return names[extend as usize];
}
unsafe extern "C" fn _filter_to_string(
    mut filter: cairo_filter_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 6] = [
        b"FILTER_FAST\0" as *const u8 as *const libc::c_char,
        b"FILTER_GOOD\0" as *const u8 as *const libc::c_char,
        b"FILTER_BEST\0" as *const u8 as *const libc::c_char,
        b"FILTER_NEAREST\0" as *const u8 as *const libc::c_char,
        b"FILTER_BILINEAR\0" as *const u8 as *const libc::c_char,
        b"FILTER_GAUSSIAN\0" as *const u8 as *const libc::c_char,
    ];
    if (filter as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"filter < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            369 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"const char *_filter_to_string(cairo_filter_t)\0"))
                .as_ptr(),
        );
    }
    return names[filter as usize];
}
unsafe extern "C" fn _fill_rule_to_string(
    mut rule: cairo_fill_rule_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 2] = [
        b"WINDING\0" as *const u8 as *const libc::c_char,
        b"EVEN_ODD\0" as *const u8 as *const libc::c_char,
    ];
    if (rule as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"rule < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"const char *_fill_rule_to_string(cairo_fill_rule_t)\0"))
                .as_ptr(),
        );
    }
    return names[rule as usize];
}
unsafe extern "C" fn _antialias_to_string(
    mut antialias: cairo_antialias_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 7] = [
        b"ANTIALIAS_DEFAULT\0" as *const u8 as *const libc::c_char,
        b"ANTIALIAS_NONE\0" as *const u8 as *const libc::c_char,
        b"ANTIALIAS_GRAY\0" as *const u8 as *const libc::c_char,
        b"ANTIALIAS_SUBPIXEL\0" as *const u8 as *const libc::c_char,
        b"ANTIALIAS_FAST\0" as *const u8 as *const libc::c_char,
        b"ANTIALIAS_GOOD\0" as *const u8 as *const libc::c_char,
        b"ANTIALIAS_BEST\0" as *const u8 as *const libc::c_char,
    ];
    if (antialias as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"antialias < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"const char *_antialias_to_string(cairo_antialias_t)\0"))
                .as_ptr(),
        );
    }
    return names[antialias as usize];
}
unsafe extern "C" fn _line_cap_to_string(
    mut line_cap: cairo_line_cap_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 3] = [
        b"LINE_CAP_BUTT\0" as *const u8 as *const libc::c_char,
        b"LINE_CAP_ROUND\0" as *const u8 as *const libc::c_char,
        b"LINE_CAP_SQUARE\0" as *const u8 as *const libc::c_char,
    ];
    if (line_cap as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"line_cap < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"const char *_line_cap_to_string(cairo_line_cap_t)\0"))
                .as_ptr(),
        );
    }
    return names[line_cap as usize];
}
unsafe extern "C" fn _line_join_to_string(
    mut line_join: cairo_line_join_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 3] = [
        b"LINE_JOIN_MITER\0" as *const u8 as *const libc::c_char,
        b"LINE_JOIN_ROUND\0" as *const u8 as *const libc::c_char,
        b"LINE_JOIN_BEVEL\0" as *const u8 as *const libc::c_char,
    ];
    if (line_join as libc::c_uint)
        < (::std::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"line_join < ARRAY_LENGTH (names)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"const char *_line_join_to_string(cairo_line_join_t)\0"))
                .as_ptr(),
        );
    }
    return names[line_join as usize];
}
#[inline]
unsafe extern "C" fn to_context(
    mut surface: *mut cairo_script_surface_t,
) -> *mut cairo_script_context_t {
    return (*surface).base.device as *mut cairo_script_context_t;
}
unsafe extern "C" fn target_is_active(
    mut surface: *mut cairo_script_surface_t,
) -> cairo_bool_t {
    return cairo_list_is_first(
        &mut (*surface).operand.link,
        &mut (*(to_context
            as unsafe extern "C" fn(
                *mut cairo_script_surface_t,
            ) -> *mut cairo_script_context_t)(surface))
            .operands,
    );
}
unsafe extern "C" fn target_push(mut surface: *mut cairo_script_surface_t) {
    cairo_list_move(
        &mut (*surface).operand.link,
        &mut (*(to_context
            as unsafe extern "C" fn(
                *mut cairo_script_surface_t,
            ) -> *mut cairo_script_context_t)(surface))
            .operands,
    );
}
unsafe extern "C" fn target_depth(
    mut surface: *mut cairo_script_surface_t,
) -> libc::c_int {
    let mut link: *mut cairo_list_t = 0 as *mut cairo_list_t;
    let mut depth: libc::c_int = 0 as libc::c_int;
    link = (*to_context(surface)).operands.next;
    while link
        != &mut (*(to_context
            as unsafe extern "C" fn(
                *mut cairo_script_surface_t,
            ) -> *mut cairo_script_context_t)(surface))
            .operands as *mut cairo_list_t
    {
        if link == &mut (*surface).operand.link as *mut cairo_list_t {
            break;
        }
        depth += 1;
        link = (*link).next;
    }
    return depth;
}
unsafe extern "C" fn _get_target(mut surface: *mut cairo_script_surface_t) {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    if target_is_active(surface) != 0 {
        _cairo_output_stream_write(
            (*ctx).stream,
            b"dup \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"dup \0" as *const u8 as *const libc::c_char),
        );
        return;
    }
    if (*surface).defined != 0 {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"s%u \0" as *const u8 as *const libc::c_char,
            (*surface).base.unique_id,
        );
    } else {
        let mut depth: libc::c_int = target_depth(surface);
        if cairo_list_is_empty(&mut (*surface).operand.link) == 0 {} else {
            __assert_fail(
                b"! cairo_list_is_empty (&surface->operand.link)\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                474 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void _get_target(cairo_script_surface_t *)\0"))
                    .as_ptr(),
            );
        }
        if target_is_active(surface) == 0 {} else {
            __assert_fail(
                b"! target_is_active (surface)\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                475 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void _get_target(cairo_script_surface_t *)\0"))
                    .as_ptr(),
            );
        }
        if (*ctx).active != 0 {
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"%d index \0" as *const u8 as *const libc::c_char,
                depth,
            );
            _cairo_output_stream_write(
                (*ctx).stream,
                b"/target get exch pop \0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                strlen(b"/target get exch pop \0" as *const u8 as *const libc::c_char),
            );
        } else {
            if depth == 1 as libc::c_int {
                _cairo_output_stream_write(
                    (*ctx).stream,
                    b"exch \0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    strlen(b"exch \0" as *const u8 as *const libc::c_char),
                );
            } else {
                _cairo_output_stream_printf(
                    (*ctx).stream,
                    b"%d -1 roll \0" as *const u8 as *const libc::c_char,
                    depth,
                );
            }
            target_push(surface);
            _cairo_output_stream_write(
                (*ctx).stream,
                b"dup \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"dup \0" as *const u8 as *const libc::c_char),
            );
        }
    };
}
unsafe extern "C" fn _content_to_string(
    mut content: cairo_content_t,
) -> *const libc::c_char {
    match content as libc::c_uint {
        8192 => return b"ALPHA\0" as *const u8 as *const libc::c_char,
        4096 => return b"COLOR\0" as *const u8 as *const libc::c_char,
        12288 | _ => return b"COLOR_ALPHA\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn _emit_surface(
    mut surface: *mut cairo_script_surface_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"<< /content //%s\0" as *const u8 as *const libc::c_char,
        _content_to_string((*surface).base.content),
    );
    if (*surface).width != -(1 as libc::c_int) as libc::c_double
        && (*surface).height != -(1 as libc::c_int) as libc::c_double
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /width %f /height %f\0" as *const u8 as *const libc::c_char,
            (*surface).width,
            (*surface).height,
        );
    }
    if (*surface).base.x_fallback_resolution != 300.0f64
        || (*surface).base.y_fallback_resolution != 300.0f64
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /fallback-resolution [%f %f]\0" as *const u8 as *const libc::c_char,
            (*surface).base.x_fallback_resolution,
            (*surface).base.y_fallback_resolution,
        );
    }
    (*surface).base.device_transform.x0 != 0.0f64
        || (*surface).base.device_transform.y0 != 0.0f64;
    _cairo_output_stream_write(
        (*ctx).stream,
        b" >> surface context\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        strlen(b" >> surface context\n\0" as *const u8 as *const libc::c_char),
    );
    (*surface).emitted = 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_context(
    mut surface: *mut cairo_script_surface_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    if target_is_active(surface) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    while cairo_list_is_empty(&mut (*ctx).operands) == 0 {
        let mut op: *mut operand_t = 0 as *mut operand_t;
        let mut old: *mut cairo_script_surface_t = 0 as *mut cairo_script_surface_t;
        op = ({
            let mut mptr__: *const cairo_list_t = (*ctx).operands.next;
            (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                as *mut operand_t
        });
        if (*op).type_0 as libc::c_uint == DEFERRED as libc::c_int as libc::c_uint {
            break;
        }
        old = ({
            let mut mptr__: *const operand_t = op;
            (mptr__ as *mut libc::c_char).offset(-(456 as libc::c_ulong as isize))
                as *mut cairo_script_surface_t
        });
        if old == surface {
            break;
        }
        if (*old).active != 0 {
            break;
        }
        if (*old).defined == 0 {
            if (*old).emitted != 0 {} else {
                __assert_fail(
                    b"old->emitted\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-script-surface.c\0" as *const u8
                        as *const libc::c_char,
                    572 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"cairo_status_t _emit_context(cairo_script_surface_t *)\0"))
                        .as_ptr(),
                );
            }
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"/target get /s%u exch def pop\n\0" as *const u8 as *const libc::c_char,
                (*old).base.unique_id,
            );
            (*old).defined = 1 as libc::c_int;
        } else {
            _cairo_output_stream_write(
                (*ctx).stream,
                b"pop\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"pop\n\0" as *const u8 as *const libc::c_char),
            );
        }
        cairo_list_del(&mut (*old).operand.link);
    }
    if target_is_active(surface) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*surface).emitted == 0 {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _emit_surface(surface);
        if status as u64 != 0 {
            return status;
        }
    } else if cairo_list_is_empty(&mut (*surface).operand.link) != 0 {
        if (*surface).defined != 0 {} else {
            __assert_fail(
                b"surface->defined\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                594 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"cairo_status_t _emit_context(cairo_script_surface_t *)\0"))
                    .as_ptr(),
            );
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"s%u context\n\0" as *const u8 as *const libc::c_char,
            (*surface).base.unique_id,
        );
        _cairo_script_implicit_context_reset(&mut (*surface).cr);
        _cairo_surface_clipper_reset(&mut (*surface).clipper);
    } else {
        let mut depth: libc::c_int = target_depth(surface);
        if depth == 1 as libc::c_int {
            _cairo_output_stream_write(
                (*ctx).stream,
                b"exch\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"exch\n\0" as *const u8 as *const libc::c_char),
            );
        } else {
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"%d -1 roll\n\0" as *const u8 as *const libc::c_char,
                depth,
            );
        }
    }
    target_push(surface);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_operator(
    mut surface: *mut cairo_script_surface_t,
    mut op: cairo_operator_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            619 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"cairo_status_t _emit_operator(cairo_script_surface_t *, cairo_operator_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).cr.current_operator as libc::c_uint == op as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_operator = op;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"//%s set-operator\n\0" as *const u8 as *const libc::c_char,
        _operator_to_string(op),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_fill_rule(
    mut surface: *mut cairo_script_surface_t,
    mut fill_rule: cairo_fill_rule_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            636 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"cairo_status_t _emit_fill_rule(cairo_script_surface_t *, cairo_fill_rule_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).cr.current_fill_rule as libc::c_uint == fill_rule as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_fill_rule = fill_rule;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"//%s set-fill-rule\n\0" as *const u8 as *const libc::c_char,
        _fill_rule_to_string(fill_rule),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_tolerance(
    mut surface: *mut cairo_script_surface_t,
    mut tolerance: libc::c_double,
    mut force: cairo_bool_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"cairo_status_t _emit_tolerance(cairo_script_surface_t *, double, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (force == 0 || fabs(tolerance - 0.1f64) < 1e-5f64)
        && (*surface).cr.current_tolerance == tolerance
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_tolerance = tolerance;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"%f set-tolerance\n\0" as *const u8 as *const libc::c_char,
        tolerance,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_antialias(
    mut surface: *mut cairo_script_surface_t,
    mut antialias: cairo_antialias_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            675 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"cairo_status_t _emit_antialias(cairo_script_surface_t *, cairo_antialias_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).cr.current_antialias as libc::c_uint == antialias as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_antialias = antialias;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"//%s set-antialias\n\0" as *const u8 as *const libc::c_char,
        _antialias_to_string(antialias),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_line_width(
    mut surface: *mut cairo_script_surface_t,
    mut line_width: libc::c_double,
    mut force: cairo_bool_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            694 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"cairo_status_t _emit_line_width(cairo_script_surface_t *, double, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (force == 0 || fabs(line_width - 2.0f64) < 1e-5f64)
        && (*surface).cr.current_style.line_width == line_width
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_style.line_width = line_width;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"%f set-line-width\n\0" as *const u8 as *const libc::c_char,
        line_width,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_hairline(
    mut surface: *mut cairo_script_surface_t,
    mut set_hairline: cairo_bool_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            714 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"cairo_status_t _emit_hairline(cairo_script_surface_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).cr.current_style.is_hairline == set_hairline {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_style.is_hairline = set_hairline;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"%d set-hairline\n\0" as *const u8 as *const libc::c_char,
        set_hairline,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_line_cap(
    mut surface: *mut cairo_script_surface_t,
    mut line_cap: cairo_line_cap_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            733 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"cairo_status_t _emit_line_cap(cairo_script_surface_t *, cairo_line_cap_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).cr.current_style.line_cap as libc::c_uint == line_cap as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_style.line_cap = line_cap;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"//%s set-line-cap\n\0" as *const u8 as *const libc::c_char,
        _line_cap_to_string(line_cap),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_line_join(
    mut surface: *mut cairo_script_surface_t,
    mut line_join: cairo_line_join_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            750 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"cairo_status_t _emit_line_join(cairo_script_surface_t *, cairo_line_join_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).cr.current_style.line_join as libc::c_uint == line_join as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_style.line_join = line_join;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"//%s set-line-join\n\0" as *const u8 as *const libc::c_char,
        _line_join_to_string(line_join),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_miter_limit(
    mut surface: *mut cairo_script_surface_t,
    mut miter_limit: libc::c_double,
    mut force: cairo_bool_t,
) -> cairo_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            768 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"cairo_status_t _emit_miter_limit(cairo_script_surface_t *, double, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (force == 0 || fabs(miter_limit - 10.0f64) < 1e-5f64)
        && (*surface).cr.current_style.miter_limit == miter_limit
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_style.miter_limit = miter_limit;
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"%f set-miter-limit\n\0" as *const u8 as *const libc::c_char,
        miter_limit,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _dashes_equal(
    mut a: *const libc::c_double,
    mut b: *const libc::c_double,
    mut num_dashes: libc::c_int,
) -> cairo_bool_t {
    loop {
        let fresh13 = num_dashes;
        num_dashes = num_dashes - 1;
        if !(fresh13 != 0) {
            break;
        }
        if fabs(*a - *b) > 1e-5f64 {
            return 0 as libc::c_int;
        }
        a = a.offset(1);
        b = b.offset(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _emit_dash(
    mut surface: *mut cairo_script_surface_t,
    mut dash: *const libc::c_double,
    mut num_dashes: libc::c_uint,
    mut offset: libc::c_double,
    mut force: cairo_bool_t,
) -> cairo_status_t {
    let mut n: libc::c_uint = 0;
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            806 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"cairo_status_t _emit_dash(cairo_script_surface_t *, const double *, unsigned int, double, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if force != 0 && num_dashes == 0 as libc::c_int as libc::c_uint
        && (*surface).cr.current_style.num_dashes == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS;
    }
    if force == 0
        && ((*surface).cr.current_style.num_dashes == num_dashes
            && (num_dashes == 0 as libc::c_int as libc::c_uint
                || fabs((*surface).cr.current_style.dash_offset - offset) < 1e-5f64
                    && _dashes_equal(
                        (*surface).cr.current_style.dash,
                        dash,
                        num_dashes as libc::c_int,
                    ) != 0))
    {
        return CAIRO_STATUS_SUCCESS;
    }
    if num_dashes != 0 {
        let ref mut fresh14 = (*surface).cr.current_style.dash;
        *fresh14 = _cairo_realloc_ab(
            (*surface).cr.current_style.dash as *mut libc::c_void,
            num_dashes as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if ((*surface).cr.current_style.dash).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        memcpy(
            (*surface).cr.current_style.dash as *mut libc::c_void,
            dash as *const libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(num_dashes as libc::c_ulong),
        );
    } else {
        free((*surface).cr.current_style.dash as *mut libc::c_void);
        let ref mut fresh15 = (*surface).cr.current_style.dash;
        *fresh15 = 0 as *mut libc::c_double;
    }
    (*surface).cr.current_style.num_dashes = num_dashes;
    (*surface).cr.current_style.dash_offset = offset;
    _cairo_output_stream_write(
        (*to_context(surface)).stream,
        b"[\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"[\0" as *const u8 as *const libc::c_char),
    );
    n = 0 as libc::c_int as libc::c_uint;
    while n < num_dashes {
        _cairo_output_stream_printf(
            (*to_context(surface)).stream,
            b"%f\0" as *const u8 as *const libc::c_char,
            *dash.offset(n as isize),
        );
        if n < num_dashes.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            _cairo_output_stream_write(
                (*to_context(surface)).stream,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b" \0" as *const u8 as *const libc::c_char),
            );
        }
        n = n.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"] %f set-dash\n\0" as *const u8 as *const libc::c_char,
        offset,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_stroke_style(
    mut surface: *mut cairo_script_surface_t,
    mut style: *const cairo_stroke_style_t,
    mut force: cairo_bool_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            861 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"cairo_status_t _emit_stroke_style(cairo_script_surface_t *, const cairo_stroke_style_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    status = _emit_line_width(surface, (*style).line_width, force);
    if status as u64 != 0 {
        return status;
    }
    status = _emit_line_cap(surface, (*style).line_cap);
    if status as u64 != 0 {
        return status;
    }
    status = _emit_line_join(surface, (*style).line_join);
    if status as u64 != 0 {
        return status;
    }
    status = _emit_miter_limit(surface, (*style).miter_limit, force);
    if status as u64 != 0 {
        return status;
    }
    status = _emit_hairline(surface, (*style).is_hairline);
    if status as u64 != 0 {
        return status;
    }
    status = _emit_dash(
        surface,
        (*style).dash,
        (*style).num_dashes,
        (*style).dash_offset,
        force,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _format_to_string(
    mut format: cairo_format_t,
) -> *const libc::c_char {
    match format as libc::c_int {
        7 => return b"RGBA128F\0" as *const u8 as *const libc::c_char,
        6 => return b"RGB96F\0" as *const u8 as *const libc::c_char,
        0 => return b"ARGB32\0" as *const u8 as *const libc::c_char,
        5 => return b"RGB30\0" as *const u8 as *const libc::c_char,
        1 => return b"RGB24\0" as *const u8 as *const libc::c_char,
        4 => return b"RGB16_565\0" as *const u8 as *const libc::c_char,
        2 => return b"A8\0" as *const u8 as *const libc::c_char,
        3 => return b"A1\0" as *const u8 as *const libc::c_char,
        -1 => return b"INVALID\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            906 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"const char *_format_to_string(cairo_format_t)\0"))
                .as_ptr(),
        );
    }
    return b"INVALID\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn _emit_solid_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut solid: *mut cairo_solid_pattern_t = pattern as *mut cairo_solid_pattern_t;
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    if !((*solid).color.alpha_short as libc::c_int >= 0xff00 as libc::c_int) {
        if (*surface).base.content as libc::c_uint
            & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint == 0
            || ((*solid).color.red_short as libc::c_int == 0 as libc::c_int
                || (*solid).color.red_short as libc::c_int == 0xffff as libc::c_int)
                && ((*solid).color.green_short as libc::c_int == 0 as libc::c_int
                    || (*solid).color.green_short as libc::c_int
                        == 0xffff as libc::c_int)
                && ((*solid).color.blue_short as libc::c_int == 0 as libc::c_int
                    || (*solid).color.blue_short as libc::c_int == 0xffff as libc::c_int)
        {
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"%f a\0" as *const u8 as *const libc::c_char,
                (*solid).color.alpha,
            );
        } else {
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"%f %f %f %f rgba\0" as *const u8 as *const libc::c_char,
                (*solid).color.red,
                (*solid).color.green,
                (*solid).color.blue,
                (*solid).color.alpha,
            );
        }
    } else if (*solid).color.red_short as libc::c_int
        == (*solid).color.green_short as libc::c_int
        && (*solid).color.red_short as libc::c_int
            == (*solid).color.blue_short as libc::c_int
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"%f g\0" as *const u8 as *const libc::c_char,
            (*solid).color.red,
        );
    } else {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"%f %f %f rgb\0" as *const u8 as *const libc::c_char,
            (*solid).color.red,
            (*solid).color.green,
            (*solid).color.blue,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_gradient_color_stops(
    mut gradient: *mut cairo_gradient_pattern_t,
    mut output: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n < (*gradient).n_stops {
        _cairo_output_stream_printf(
            output,
            b"\n  %f %f %f %f %f add-color-stop\0" as *const u8 as *const libc::c_char,
            (*((*gradient).stops).offset(n as isize)).offset,
            (*((*gradient).stops).offset(n as isize)).color.red,
            (*((*gradient).stops).offset(n as isize)).color.green,
            (*((*gradient).stops).offset(n as isize)).color.blue,
            (*((*gradient).stops).offset(n as isize)).color.alpha,
        );
        n = n.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_linear_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut linear: *mut cairo_linear_pattern_t = 0 as *mut cairo_linear_pattern_t;
    linear = pattern as *mut cairo_linear_pattern_t;
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"%f %f %f %f linear\0" as *const u8 as *const libc::c_char,
        (*linear).pd1.x,
        (*linear).pd1.y,
        (*linear).pd2.x,
        (*linear).pd2.y,
    );
    return _emit_gradient_color_stops(&mut (*linear).base, (*ctx).stream);
}
unsafe extern "C" fn _emit_radial_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut radial: *mut cairo_radial_pattern_t = 0 as *mut cairo_radial_pattern_t;
    radial = pattern as *mut cairo_radial_pattern_t;
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"%f %f %f %f %f %f radial\0" as *const u8 as *const libc::c_char,
        (*radial).cd1.center.x,
        (*radial).cd1.center.y,
        (*radial).cd1.radius,
        (*radial).cd2.center.x,
        (*radial).cd2.center.y,
        (*radial).cd2.radius,
    );
    return _emit_gradient_color_stops(&mut (*radial).base, (*ctx).stream);
}
unsafe extern "C" fn _emit_mesh_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut mesh: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    mesh = pattern as *mut cairo_pattern_t;
    status = cairo_mesh_pattern_get_patch_count(mesh, &mut n);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"mesh\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        let mut path: *mut cairo_path_t = 0 as *mut cairo_path_t;
        let mut data: *mut cairo_path_data_t = 0 as *mut cairo_path_data_t;
        let mut j: libc::c_int = 0;
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"\n  begin-patch\0" as *const u8 as *const libc::c_char,
        );
        path = cairo_mesh_pattern_get_path(mesh, i);
        if (*path).status as u64 != 0 {
            return (*path).status;
        }
        j = 0 as libc::c_int;
        while j < (*path).num_data {
            data = &mut *((*path).data).offset(j as isize) as *mut cairo_path_data_t;
            match (*data).header.type_0 as libc::c_uint {
                0 => {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"\n  %f %f m\0" as *const u8 as *const libc::c_char,
                        (*data.offset(1 as libc::c_int as isize)).point.x,
                        (*data.offset(1 as libc::c_int as isize)).point.y,
                    );
                }
                1 => {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"\n  %f %f l\0" as *const u8 as *const libc::c_char,
                        (*data.offset(1 as libc::c_int as isize)).point.x,
                        (*data.offset(1 as libc::c_int as isize)).point.y,
                    );
                }
                2 => {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"\n  %f %f %f %f %f %f c\0" as *const u8 as *const libc::c_char,
                        (*data.offset(1 as libc::c_int as isize)).point.x,
                        (*data.offset(1 as libc::c_int as isize)).point.y,
                        (*data.offset(2 as libc::c_int as isize)).point.x,
                        (*data.offset(2 as libc::c_int as isize)).point.y,
                        (*data.offset(3 as libc::c_int as isize)).point.x,
                        (*data.offset(3 as libc::c_int as isize)).point.y,
                    );
                }
                3 | _ => {}
            }
            j += (*data.offset(0 as libc::c_int as isize)).header.length;
        }
        cairo_path_destroy(path);
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let mut x: libc::c_double = 0.;
            let mut y: libc::c_double = 0.;
            status = cairo_mesh_pattern_get_control_point(
                mesh,
                i,
                j as libc::c_uint,
                &mut x,
                &mut y,
            );
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"\n  %d %f %f set-control-point\0" as *const u8 as *const libc::c_char,
                j,
                x,
                y,
            );
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let mut r: libc::c_double = 0.;
            let mut g: libc::c_double = 0.;
            let mut b: libc::c_double = 0.;
            let mut a: libc::c_double = 0.;
            status = cairo_mesh_pattern_get_corner_color_rgba(
                mesh,
                i,
                j as libc::c_uint,
                &mut r,
                &mut g,
                &mut b,
                &mut a,
            );
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"\n  %d %f %f %f %f set-corner-color\0" as *const u8
                    as *const libc::c_char,
                j,
                r,
                g,
                b,
                a,
            );
            j += 1;
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"\n  end-patch\0" as *const u8 as *const libc::c_char,
        );
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn script_snapshot_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    return CAIRO_STATUS_SUCCESS;
}
static mut script_snapshot_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_SCRIPT,
            finish: Some(
                script_snapshot_finish
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
unsafe extern "C" fn detach_snapshot(mut abstract_surface: *mut cairo_surface_t) {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"/s%d undef\n\0" as *const u8 as *const libc::c_char,
        (*surface).base.unique_id,
    );
}
unsafe extern "C" fn attach_snapshot(
    mut ctx: *mut cairo_script_context_t,
    mut source: *mut cairo_surface_t,
) {
    let mut surface: *mut script_snapshot = 0 as *mut script_snapshot;
    if (*ctx).attach_snapshots == 0 {
        return;
    }
    surface = (if ::std::mem::size_of::<script_snapshot>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<script_snapshot>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut script_snapshot;
    if surface.is_null() {
        return;
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &script_snapshot_backend,
        &mut (*ctx).base,
        (*source).content,
        (*source).is_vector() as cairo_bool_t,
    );
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"dup /s%d exch def \0" as *const u8 as *const libc::c_char,
        (*surface).base.unique_id,
    );
    _cairo_surface_attach_snapshot(
        source,
        &mut (*surface).base,
        Some(detach_snapshot as unsafe extern "C" fn(*mut cairo_surface_t) -> ()),
    );
    cairo_surface_destroy(&mut (*surface).base);
}
unsafe extern "C" fn _emit_recording_surface_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut source: *mut cairo_recording_surface_t,
) -> cairo_status_t {
    let mut old_cr: cairo_script_implicit_context_t = cairo_script_implicit_context_t {
        current_operator: CAIRO_OPERATOR_CLEAR,
        current_fill_rule: CAIRO_FILL_RULE_WINDING,
        current_tolerance: 0.,
        current_antialias: CAIRO_ANTIALIAS_DEFAULT,
        current_style: cairo_stroke_style_t {
            line_width: 0.,
            line_cap: CAIRO_LINE_CAP_BUTT,
            line_join: CAIRO_LINE_JOIN_MITER,
            miter_limit: 0.,
            dash: 0 as *mut libc::c_double,
            num_dashes: 0,
            dash_offset: 0.,
            is_hairline: 0,
            pre_hairline_line_width: 0.,
        },
        current_source: cairo_pattern_union_t {
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
        current_ctm: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        current_stroke_matrix: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        current_font_matrix: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        current_font_options: cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        },
        current_scaled_font: 0 as *mut cairo_scaled_font_t,
        current_path: cairo_path_fixed_t {
            last_move_point: cairo_point_t { x: 0, y: 0 },
            current_point: cairo_point_t { x: 0, y: 0 },
            has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [0; 1],
            c2rust_padding: [0; 3],
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            buf: cairo_path_buf_fixed_t {
                base: cairo_path_buf_t {
                    link: cairo_list_t {
                        next: 0 as *mut _cairo_list,
                        prev: 0 as *mut _cairo_list,
                    },
                    num_ops: 0,
                    size_ops: 0,
                    num_points: 0,
                    size_points: 0,
                    op: 0 as *mut cairo_path_op_t,
                    points: 0 as *mut cairo_point_t,
                },
                op: [0; 27],
                points: [cairo_point_t { x: 0, y: 0 }; 54],
            },
        },
        has_clip: 0,
    };
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut similar: *mut cairo_script_surface_t = 0 as *mut cairo_script_surface_t;
    let mut snapshot: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut r: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    let mut extents: *mut cairo_rectangle_t = 0 as *mut cairo_rectangle_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    snapshot = _cairo_surface_has_snapshot(
        &mut (*source).base,
        &script_snapshot_backend,
    );
    if !snapshot.is_null() {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"s%d\0" as *const u8 as *const libc::c_char,
            (*snapshot).unique_id,
        );
        return CAIRO_INT_STATUS_SUCCESS as libc::c_int as cairo_status_t;
    }
    extents = 0 as *mut cairo_rectangle_t;
    if _cairo_recording_surface_get_bounds(&mut (*source).base, &mut r) != 0 {
        extents = &mut r;
    }
    similar = _cairo_script_surface_create_internal(
        ctx,
        (*source).base.content,
        extents,
        0 as *mut cairo_surface_t,
    );
    if (*similar).base.status as u64 != 0 {
        return (*similar).base.status;
    }
    let ref mut fresh16 = (*similar).base;
    (*fresh16).set_is_clear(1 as libc::c_int as libc::c_uint);
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"//%s \0" as *const u8 as *const libc::c_char,
        _content_to_string((*source).base.content),
    );
    if !extents.is_null() {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"[%f %f %f %f]\0" as *const u8 as *const libc::c_char,
            (*extents).x,
            (*extents).y,
            (*extents).width,
            (*extents).height,
        );
    } else {
        _cairo_output_stream_write(
            (*ctx).stream,
            b"[]\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"[]\0" as *const u8 as *const libc::c_char),
        );
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b" record\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b" record\n\0" as *const u8 as *const libc::c_char),
    );
    attach_snapshot(ctx, &mut (*source).base);
    _cairo_output_stream_write(
        (*ctx).stream,
        b"dup context\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"dup context\n\0" as *const u8 as *const libc::c_char),
    );
    target_push(similar);
    (*similar).emitted = 1 as libc::c_int;
    old_cr = (*surface).cr;
    _cairo_script_implicit_context_init(&mut (*surface).cr);
    status = _cairo_recording_surface_replay(&mut (*source).base, &mut (*similar).base);
    (*surface).cr = old_cr;
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*similar).base);
        return status;
    }
    cairo_list_del(&mut (*similar).operand.link);
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"cairo_status_t _emit_recording_surface_pattern(cairo_script_surface_t *, cairo_recording_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"pop \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"pop \0" as *const u8 as *const libc::c_char),
    );
    cairo_surface_destroy(&mut (*similar).base);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_script_surface_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut source: *mut cairo_script_surface_t,
) -> cairo_status_t {
    _get_target(source);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _write_image_surface(
    mut output: *mut cairo_output_stream_t,
    mut image: *const cairo_image_surface_t,
) -> cairo_status_t {
    let mut row: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut stride: ptrdiff_t = 0;
    let mut row_stack: [uint8_t; 2048] = [0; 2048];
    let mut rowdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    stride = (*image).stride;
    width = (*image).width;
    data = (*image).data;
    if stride
        > (::std::mem::size_of::<[uint8_t; 2048]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            as libc::c_int as libc::c_long
    {
        rowdata = (if stride != 0 as libc::c_int as libc::c_long {
            malloc(stride as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut uint8_t;
        if rowdata.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    } else {
        rowdata = row_stack.as_mut_ptr();
    }
    match (*image).format as libc::c_int {
        3 => {
            row = (*image).height;
            loop {
                let fresh17 = row;
                row = row - 1;
                if !(fresh17 != 0) {
                    break;
                }
                let mut col: libc::c_int = 0;
                col = 0 as libc::c_int;
                while col < (width + 7 as libc::c_int) / 8 as libc::c_int {
                    *rowdata
                        .offset(
                            col as isize,
                        ) = (((*data.offset(col as isize) as libc::c_ulong)
                        .wrapping_mul(0x802 as libc::c_ulong) & 0x22110 as libc::c_ulong
                        | (*data.offset(col as isize) as libc::c_ulong)
                            .wrapping_mul(0x8020 as libc::c_ulong)
                            & 0x88440 as libc::c_ulong)
                        .wrapping_mul(0x10101 as libc::c_ulong) >> 16 as libc::c_int)
                        as uint8_t;
                    col += 1;
                }
                _cairo_output_stream_write(
                    output,
                    rowdata as *const libc::c_void,
                    ((width + 7 as libc::c_int) / 8 as libc::c_int) as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        2 => {
            row = (*image).height;
            loop {
                let fresh18 = row;
                row = row - 1;
                if !(fresh18 != 0) {
                    break;
                }
                _cairo_output_stream_write(
                    output,
                    data as *const libc::c_void,
                    width as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        4 => {
            row = (*image).height;
            loop {
                let fresh19 = row;
                row = row - 1;
                if !(fresh19 != 0) {
                    break;
                }
                let mut src: *mut uint16_t = data as *mut uint16_t;
                let mut dst: *mut uint16_t = rowdata as *mut uint16_t;
                let mut col_0: libc::c_int = 0;
                col_0 = 0 as libc::c_int;
                while col_0 < width {
                    *dst
                        .offset(
                            col_0 as isize,
                        ) = __bswap_16(*src.offset(col_0 as isize));
                    col_0 += 1;
                }
                _cairo_output_stream_write(
                    output,
                    rowdata as *const libc::c_void,
                    (2 as libc::c_int * width) as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        1 => {
            row = (*image).height;
            loop {
                let fresh20 = row;
                row = row - 1;
                if !(fresh20 != 0) {
                    break;
                }
                let mut src_0: *mut uint8_t = data;
                let mut col_1: libc::c_int = 0;
                col_1 = 0 as libc::c_int;
                while col_1 < width {
                    let fresh21 = src_0;
                    src_0 = src_0.offset(1);
                    *rowdata
                        .offset(
                            (3 as libc::c_int * col_1 + 2 as libc::c_int) as isize,
                        ) = *fresh21;
                    let fresh22 = src_0;
                    src_0 = src_0.offset(1);
                    *rowdata
                        .offset(
                            (3 as libc::c_int * col_1 + 1 as libc::c_int) as isize,
                        ) = *fresh22;
                    let fresh23 = src_0;
                    src_0 = src_0.offset(1);
                    *rowdata
                        .offset(
                            (3 as libc::c_int * col_1 + 0 as libc::c_int) as isize,
                        ) = *fresh23;
                    src_0 = src_0.offset(1);
                    col_1 += 1;
                }
                _cairo_output_stream_write(
                    output,
                    rowdata as *const libc::c_void,
                    (3 as libc::c_int * width) as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        5 | 0 => {
            row = (*image).height;
            loop {
                let fresh24 = row;
                row = row - 1;
                if !(fresh24 != 0) {
                    break;
                }
                let mut src_1: *mut uint32_t = data as *mut uint32_t;
                let mut dst_0: *mut uint32_t = rowdata as *mut uint32_t;
                let mut col_2: libc::c_int = 0;
                col_2 = 0 as libc::c_int;
                while col_2 < width {
                    *dst_0
                        .offset(
                            col_2 as isize,
                        ) = __bswap_32(*src_1.offset(col_2 as isize));
                    col_2 += 1;
                }
                _cairo_output_stream_write(
                    output,
                    rowdata as *const libc::c_void,
                    (4 as libc::c_int * width) as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        6 => {
            row = (*image).height;
            loop {
                let fresh25 = row;
                row = row - 1;
                if !(fresh25 != 0) {
                    break;
                }
                _cairo_output_stream_write(
                    output,
                    data as *const libc::c_void,
                    (12 as libc::c_int * width) as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        7 => {
            row = (*image).height;
            loop {
                let fresh26 = row;
                row = row - 1;
                if !(fresh26 != 0) {
                    break;
                }
                _cairo_output_stream_write(
                    output,
                    data as *const libc::c_void,
                    (16 as libc::c_int * width) as size_t,
                );
                data = data.offset(stride as isize);
            }
        }
        -1 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-script-surface.c\0" as *const u8
                        as *const libc::c_char,
                    1356 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 92],
                        &[libc::c_char; 92],
                    >(
                        b"cairo_status_t _write_image_surface(cairo_output_stream_t *, const cairo_image_surface_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    if rowdata != row_stack.as_mut_ptr() {
        free(rowdata as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_png_surface(
    mut surface: *mut cairo_script_surface_t,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut base85_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut mime_data: *const uint8_t = 0 as *const uint8_t;
    let mut mime_data_length: libc::c_ulong = 0;
    cairo_surface_get_mime_data(
        &mut (*image).base,
        b"image/png\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"<< /width %d /height %d /format //%s /mime-type (image/png) /source <~\0"
            as *const u8 as *const libc::c_char,
        (*image).width,
        (*image).height,
        _format_to_string((*image).format),
    );
    base85_stream = _cairo_base85_stream_create((*ctx).stream);
    _cairo_output_stream_write(
        base85_stream,
        mime_data as *const libc::c_void,
        mime_data_length,
    );
    status = _cairo_output_stream_destroy(base85_stream);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"~> >> image \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"~> >> image \0" as *const u8 as *const libc::c_char),
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _emit_image_surface(
    mut surface: *mut cairo_script_surface_t,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut base85_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut zlib_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut status2: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut snapshot: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut mime_data: *const uint8_t = 0 as *const uint8_t;
    let mut mime_data_length: libc::c_ulong = 0;
    snapshot = _cairo_surface_has_snapshot(&mut (*image).base, &script_snapshot_backend);
    if !snapshot.is_null() {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"s%u \0" as *const u8 as *const libc::c_char,
            (*snapshot).unique_id,
        );
        return CAIRO_INT_STATUS_SUCCESS;
    }
    status = _emit_png_surface(surface, image);
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status
    } else {
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            let mut clone: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
            let mut len: uint32_t = 0;
            if (*image).format as libc::c_int == CAIRO_FORMAT_INVALID as libc::c_int {
                clone = _cairo_image_surface_coerce(image);
            } else {
                clone = cairo_surface_reference(&mut (*image).base)
                    as *mut cairo_image_surface_t;
            }
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"<< /width %d /height %d /format //%s /source \0" as *const u8
                    as *const libc::c_char,
                (*clone).width,
                (*clone).height,
                _format_to_string((*clone).format),
            );
            match (*clone).format as libc::c_int {
                3 => {
                    len = (((*clone).width + 7 as libc::c_int) / 8 as libc::c_int)
                        as uint32_t;
                }
                2 => {
                    len = (*clone).width as uint32_t;
                }
                4 => {
                    len = ((*clone).width * 2 as libc::c_int) as uint32_t;
                }
                1 => {
                    len = ((*clone).width * 3 as libc::c_int) as uint32_t;
                }
                5 | 0 => {
                    len = ((*clone).width * 4 as libc::c_int) as uint32_t;
                }
                6 => {
                    len = ((*clone).width * 12 as libc::c_int) as uint32_t;
                }
                7 => {
                    len = ((*clone).width * 16 as libc::c_int) as uint32_t;
                }
                -1 | _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-script-surface.c\0" as *const u8
                                as *const libc::c_char,
                            1468 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 90],
                                &[libc::c_char; 90],
                            >(
                                b"cairo_int_status_t _emit_image_surface(cairo_script_surface_t *, cairo_image_surface_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    len = 0 as libc::c_int as uint32_t;
                }
            }
            len = (len as libc::c_uint).wrapping_mul((*clone).height as libc::c_uint)
                as uint32_t as uint32_t;
            if len > 24 as libc::c_int as libc::c_uint {
                _cairo_output_stream_write(
                    (*ctx).stream,
                    b"<|\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"<|\0" as *const u8 as *const libc::c_char),
                );
                base85_stream = _cairo_base85_stream_create((*ctx).stream);
                len = __bswap_32(len);
                _cairo_output_stream_write(
                    base85_stream,
                    &mut len as *mut uint32_t as *const libc::c_void,
                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                );
                zlib_stream = _cairo_deflate_stream_create(base85_stream);
                status = _write_image_surface(zlib_stream, clone) as cairo_int_status_t;
                status2 = _cairo_output_stream_destroy(zlib_stream)
                    as cairo_int_status_t;
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    status = status2;
                }
                status2 = _cairo_output_stream_destroy(base85_stream)
                    as cairo_int_status_t;
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    status = status2;
                }
                if status as u64 != 0 {
                    return status;
                }
            } else {
                _cairo_output_stream_write(
                    (*ctx).stream,
                    b"<~\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"<~\0" as *const u8 as *const libc::c_char),
                );
                base85_stream = _cairo_base85_stream_create((*ctx).stream);
                status = _write_image_surface(base85_stream, clone)
                    as cairo_int_status_t;
                status2 = _cairo_output_stream_destroy(base85_stream)
                    as cairo_int_status_t;
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    status = status2;
                }
                if status as u64 != 0 {
                    return status;
                }
            }
            _cairo_output_stream_write(
                (*ctx).stream,
                b"~> >> image \0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                strlen(b"~> >> image \0" as *const u8 as *const libc::c_char),
            );
            cairo_surface_destroy(&mut (*clone).base);
        }
    }
    cairo_surface_get_mime_data(
        &mut (*image).base,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if !mime_data.is_null() {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"\n  (%s) <~\0" as *const u8 as *const libc::c_char,
            b"image/jpeg\0" as *const u8 as *const libc::c_char,
        );
        base85_stream = _cairo_base85_stream_create((*ctx).stream);
        _cairo_output_stream_write(
            base85_stream,
            mime_data as *const libc::c_void,
            mime_data_length,
        );
        status = _cairo_output_stream_destroy(base85_stream) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_write(
            (*ctx).stream,
            b"~> set-mime-data\n\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            strlen(b"~> set-mime-data\n\0" as *const u8 as *const libc::c_char),
        );
    }
    cairo_surface_get_mime_data(
        &mut (*image).base,
        b"image/jp2\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if !mime_data.is_null() {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"\n  (%s) <~\0" as *const u8 as *const libc::c_char,
            b"image/jp2\0" as *const u8 as *const libc::c_char,
        );
        base85_stream = _cairo_base85_stream_create((*ctx).stream);
        _cairo_output_stream_write(
            base85_stream,
            mime_data as *const libc::c_void,
            mime_data_length,
        );
        status = _cairo_output_stream_destroy(base85_stream) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_write(
            (*ctx).stream,
            b"~> set-mime-data\n\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            strlen(b"~> set-mime-data\n\0" as *const u8 as *const libc::c_char),
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_image_surface_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut source: *mut cairo_surface_t,
) -> cairo_int_status_t {
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut extra: *mut libc::c_void = 0 as *mut libc::c_void;
    status = _cairo_surface_acquire_source_image(source, &mut image, &mut extra);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _emit_image_surface(surface, image) as cairo_status_t;
        _cairo_surface_release_source_image(source, image, extra);
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _emit_subsurface_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut sub: *mut cairo_surface_subsurface_t,
) -> cairo_int_status_t {
    let mut source: *mut cairo_surface_t = (*sub).target;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    match (*(*source).backend).type_0 as libc::c_int {
        16 => {
            status = _emit_recording_surface_pattern(
                surface,
                source as *mut cairo_recording_surface_t,
            ) as cairo_int_status_t;
        }
        14 => {
            status = _emit_script_surface_pattern(
                surface,
                source as *mut cairo_script_surface_t,
            ) as cairo_int_status_t;
        }
        _ => {
            status = _emit_image_surface_pattern(surface, source);
        }
    }
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*to_context(surface)).stream,
        b"%d %d %d %d subsurface \0" as *const u8 as *const libc::c_char,
        (*sub).extents.x,
        (*sub).extents.y,
        (*sub).extents.width,
        (*sub).extents.height,
    );
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_surface_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_int_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut surface_pattern: *mut cairo_surface_pattern_t = 0
        as *mut cairo_surface_pattern_t;
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut snapshot: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut free_me: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut take_snapshot: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    surface_pattern = pattern as *mut cairo_surface_pattern_t;
    source = (*surface_pattern).surface;
    if _cairo_surface_is_snapshot(source) != 0 {
        snapshot = _cairo_surface_has_snapshot(source, &script_snapshot_backend);
        if !snapshot.is_null() {
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"s%d pattern \0" as *const u8 as *const libc::c_char,
                (*snapshot).unique_id,
            );
            return CAIRO_INT_STATUS_SUCCESS;
        }
        if _cairo_surface_snapshot_is_reused(source) != 0 {
            take_snapshot = source;
        }
        source = _cairo_surface_snapshot_get_target(source);
        free_me = source;
    }
    match (*(*source).backend).type_0 as libc::c_int {
        16 => {
            status = _emit_recording_surface_pattern(
                surface,
                source as *mut cairo_recording_surface_t,
            ) as cairo_int_status_t;
        }
        14 => {
            status = _emit_script_surface_pattern(
                surface,
                source as *mut cairo_script_surface_t,
            ) as cairo_int_status_t;
        }
        23 => {
            status = _emit_subsurface_pattern(
                surface,
                source as *mut cairo_surface_subsurface_t,
            );
        }
        _ => {
            status = _emit_image_surface_pattern(surface, source);
        }
    }
    cairo_surface_destroy(free_me);
    if status as u64 != 0 {
        return status;
    }
    if !take_snapshot.is_null() {
        attach_snapshot(ctx, take_snapshot);
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"pattern\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"pattern\0" as *const u8 as *const libc::c_char),
    );
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_raster_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_int_status_t {
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    source = _cairo_raster_source_pattern_acquire(
        pattern,
        &mut (*surface).base,
        0 as *const cairo_rectangle_int_t,
    );
    if source.is_null() {
        if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                1653 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"cairo_int_status_t _emit_raster_pattern(cairo_script_surface_t *, const cairo_pattern_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*source).status as u64 != 0 {
        return (*source).status as cairo_int_status_t;
    }
    status = _emit_image_surface_pattern(surface, source);
    _cairo_raster_source_pattern_release(pattern, source);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_write(
        (*to_context(surface)).stream,
        b"pattern\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"pattern\0" as *const u8 as *const libc::c_char),
    );
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_pattern(
    mut surface: *mut cairo_script_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_int_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut is_default_extend: cairo_bool_t = 0;
    let mut need_newline: cairo_bool_t = 1 as libc::c_int;
    match (*pattern).type_0 as libc::c_uint {
        0 => return _emit_solid_pattern(surface, pattern) as cairo_int_status_t,
        2 => {
            status = _emit_linear_pattern(surface, pattern) as cairo_int_status_t;
            is_default_extend = ((*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint) as libc::c_int;
        }
        3 => {
            status = _emit_radial_pattern(surface, pattern) as cairo_int_status_t;
            is_default_extend = ((*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint) as libc::c_int;
        }
        4 => {
            status = _emit_mesh_pattern(surface, pattern) as cairo_int_status_t;
            is_default_extend = 1 as libc::c_int;
        }
        1 => {
            status = _emit_surface_pattern(surface, pattern);
            is_default_extend = ((*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint) as libc::c_int;
        }
        5 => {
            status = _emit_raster_pattern(surface, pattern);
            is_default_extend = ((*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint) as libc::c_int;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-script-surface.c\0" as *const u8
                        as *const libc::c_char,
                    1704 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"cairo_int_status_t _emit_pattern(cairo_script_surface_t *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            status = CAIRO_INT_STATUS_UNSUPPORTED;
        }
    }
    if status as u64 != 0 {
        return status;
    }
    if _cairo_matrix_is_identity(&(*pattern).matrix) == 0 {
        if need_newline != 0 {
            _cairo_output_stream_write(
                (*ctx).stream,
                b"\n \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"\n \0" as *const u8 as *const libc::c_char),
            );
            need_newline = 0 as libc::c_int;
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" [%f %f %f %f %f %f] set-matrix\n \0" as *const u8 as *const libc::c_char,
            (*pattern).matrix.xx,
            (*pattern).matrix.yx,
            (*pattern).matrix.xy,
            (*pattern).matrix.yy,
            (*pattern).matrix.x0,
            (*pattern).matrix.y0,
        );
    }
    if (*pattern).filter as libc::c_uint
        != CAIRO_FILTER_GOOD as libc::c_int as libc::c_uint
    {
        if need_newline != 0 {
            _cairo_output_stream_write(
                (*ctx).stream,
                b"\n \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"\n \0" as *const u8 as *const libc::c_char),
            );
            need_newline = 0 as libc::c_int;
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" //%s set-filter\n \0" as *const u8 as *const libc::c_char,
            _filter_to_string((*pattern).filter),
        );
    }
    if is_default_extend == 0 {
        if need_newline != 0 {
            _cairo_output_stream_write(
                (*ctx).stream,
                b"\n \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"\n \0" as *const u8 as *const libc::c_char),
            );
            need_newline = 0 as libc::c_int;
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" //%s set-extend\n \0" as *const u8 as *const libc::c_char,
            _extend_to_string((*pattern).extend),
        );
    }
    if need_newline != 0 {
        _cairo_output_stream_write(
            (*ctx).stream,
            b"\n \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"\n \0" as *const u8 as *const libc::c_char),
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_identity(
    mut surface: *mut cairo_script_surface_t,
    mut matrix_updated: *mut cairo_bool_t,
) -> cairo_int_status_t {
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1755 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"cairo_int_status_t _emit_identity(cairo_script_surface_t *, cairo_bool_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_matrix_is_identity(&mut (*surface).cr.current_ctm) != 0 {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    _cairo_output_stream_write(
        (*to_context(surface)).stream,
        b"identity set-matrix\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        strlen(b"identity set-matrix\n\0" as *const u8 as *const libc::c_char),
    );
    *matrix_updated = 1 as libc::c_int;
    cairo_matrix_init_identity(&mut (*surface).cr.current_ctm);
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_source(
    mut surface: *mut cairo_script_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
) -> cairo_int_status_t {
    let mut matrix_updated: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1777 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"cairo_int_status_t _emit_source(cairo_script_surface_t *, cairo_operator_t, const cairo_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    if _cairo_pattern_equal(&mut (*surface).cr.current_source.base, source) != 0 {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    _cairo_pattern_fini(&mut (*surface).cr.current_source.base);
    status = _cairo_pattern_init_copy(&mut (*surface).cr.current_source.base, source)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _emit_identity(surface, &mut matrix_updated);
    if status as u64 != 0 {
        return status;
    }
    status = _emit_pattern(surface, source);
    if status as u64 != 0 {
        return status;
    }
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1801 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"cairo_int_status_t _emit_source(cairo_script_surface_t *, cairo_operator_t, const cairo_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_output_stream_write(
        (*to_context(surface)).stream,
        b" set-source\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b" set-source\n\0" as *const u8 as *const libc::c_char),
    );
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _path_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    _cairo_output_stream_printf(
        closure as *mut cairo_output_stream_t,
        b" %f %f m\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*point).x),
        _cairo_fixed_to_double((*point).y),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _path_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    _cairo_output_stream_printf(
        closure as *mut cairo_output_stream_t,
        b" %f %f l\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*point).x),
        _cairo_fixed_to_double((*point).y),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _path_curve_to(
    mut closure: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    _cairo_output_stream_printf(
        closure as *mut cairo_output_stream_t,
        b" %f %f %f %f %f %f c\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*p1).x),
        _cairo_fixed_to_double((*p1).y),
        _cairo_fixed_to_double((*p2).x),
        _cairo_fixed_to_double((*p2).y),
        _cairo_fixed_to_double((*p3).x),
        _cairo_fixed_to_double((*p3).y),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _path_close(mut closure: *mut libc::c_void) -> cairo_status_t {
    _cairo_output_stream_printf(
        closure as *mut cairo_output_stream_t,
        b" h\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_path_boxes(
    mut surface: *mut cairo_script_surface_t,
    mut path: *const cairo_path_fixed_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut iter: cairo_path_fixed_iter_t = cairo_path_fixed_iter_t {
        first: 0 as *const cairo_path_buf_t,
        buf: 0 as *const cairo_path_buf_t,
        n_op: 0,
        n_point: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut boxes: cairo_boxes_t = cairo_boxes_t {
        status: CAIRO_STATUS_SUCCESS,
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_boxes: 0,
        is_pixel_aligned: 0,
        chunks: _cairo_boxes_chunk {
            next: 0 as *mut _cairo_boxes_chunk,
            base: 0 as *mut cairo_box_t,
            count: 0,
            size: 0,
        },
        tail: 0 as *mut _cairo_boxes_chunk,
        boxes_embedded: [cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        }; 32],
    };
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut i: libc::c_int = 0;
    _cairo_boxes_init(&mut boxes);
    _cairo_path_fixed_iter_init(&mut iter, path);
    while _cairo_path_fixed_iter_is_fill_box(&mut iter, &mut box_0) != 0 {
        if box_0.p1.y == box_0.p2.y || box_0.p1.x == box_0.p2.x {
            continue;
        }
        status = _cairo_boxes_add(&mut boxes, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        if status as u64 != 0 {
            _cairo_boxes_fini(&mut boxes);
            return status;
        }
    }
    if _cairo_path_fixed_iter_at_end(&mut iter) == 0 {
        _cairo_boxes_fini(&mut boxes);
        return CAIRO_STATUS_INVALID_PATH_DATA;
    }
    chunk = &mut boxes.chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let mut b: *const cairo_box_t = &mut *((*chunk).base).offset(i as isize)
                as *mut cairo_box_t;
            let mut x1: libc::c_double = _cairo_fixed_to_double((*b).p1.x);
            let mut y1: libc::c_double = _cairo_fixed_to_double((*b).p1.y);
            let mut x2: libc::c_double = _cairo_fixed_to_double((*b).p2.x);
            let mut y2: libc::c_double = _cairo_fixed_to_double((*b).p2.y);
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"\n  %f %f %f %f rectangle\0" as *const u8 as *const libc::c_char,
                x1,
                y1,
                x2 - x1,
                y2 - y1,
            );
            i += 1;
        }
        chunk = (*chunk).next;
    }
    _cairo_boxes_fini(&mut boxes);
    return status;
}
unsafe extern "C" fn _emit_path(
    mut surface: *mut cairo_script_surface_t,
    mut path: *const cairo_path_fixed_t,
    mut is_fill: cairo_bool_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1915 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"cairo_status_t _emit_path(cairo_script_surface_t *, const cairo_path_fixed_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_matrix_is_identity(&mut (*surface).cr.current_ctm) != 0 {} else {
        __assert_fail(
            b"_cairo_matrix_is_identity (&surface->cr.current_ctm)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1916 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"cairo_status_t _emit_path(cairo_script_surface_t *, const cairo_path_fixed_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_path_fixed_equal(&mut (*surface).cr.current_path, path) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_path_fixed_fini(&mut (*surface).cr.current_path);
    _cairo_output_stream_write(
        (*ctx).stream,
        b"n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"n\0" as *const u8 as *const libc::c_char),
    );
    if path.is_null() {
        _cairo_path_fixed_init(&mut (*surface).cr.current_path);
        _cairo_output_stream_write(
            (*ctx).stream,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"\n\0" as *const u8 as *const libc::c_char),
        );
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_path_fixed_init_copy(&mut (*surface).cr.current_path, path)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if _cairo_path_fixed_is_rectangle(path, &mut box_0) != 0 {
        let mut x1: libc::c_double = _cairo_fixed_to_double(box_0.p1.x);
        let mut y1: libc::c_double = _cairo_fixed_to_double(box_0.p1.y);
        let mut x2: libc::c_double = _cairo_fixed_to_double(box_0.p2.x);
        let mut y2: libc::c_double = _cairo_fixed_to_double(box_0.p2.y);
        if x1 > -(9999 as libc::c_int) as libc::c_double {} else {
            __assert_fail(
                b"x1 > -9999\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                1942 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"cairo_status_t _emit_path(cairo_script_surface_t *, const cairo_path_fixed_t *, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" %f %f %f %f rectangle\0" as *const u8 as *const libc::c_char,
            x1,
            y1,
            x2 - x1,
            y2 - y1,
        );
        status = CAIRO_INT_STATUS_SUCCESS;
    } else if is_fill != 0 && _cairo_path_fixed_fill_is_rectilinear(path) != 0 {
        status = _emit_path_boxes(surface, path) as cairo_int_status_t;
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        status = _cairo_path_fixed_interpret(
            path,
            Some(
                _path_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _path_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _path_curve_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                        *const cairo_point_t,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _path_close as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            (*ctx).stream as *mut libc::c_void,
        ) as cairo_int_status_t;
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"\n\0" as *const u8 as *const libc::c_char),
    );
    return status as cairo_status_t;
}
unsafe extern "C" fn _scaling_matrix_equal(
    mut a: *const cairo_matrix_t,
    mut b: *const cairo_matrix_t,
) -> cairo_bool_t {
    return (fabs((*a).xx - (*b).xx) < 1e-5f64 && fabs((*a).xy - (*b).xy) < 1e-5f64
        && fabs((*a).yx - (*b).yx) < 1e-5f64 && fabs((*a).yy - (*b).yy) < 1e-5f64)
        as libc::c_int;
}
unsafe extern "C" fn _emit_scaling_matrix(
    mut surface: *mut cairo_script_surface_t,
    mut ctm: *const cairo_matrix_t,
    mut matrix_updated: *mut cairo_bool_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut was_identity: cairo_bool_t = 0;
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            1982 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"cairo_status_t _emit_scaling_matrix(cairo_script_surface_t *, const cairo_matrix_t *, cairo_bool_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if _scaling_matrix_equal(&mut (*surface).cr.current_ctm, ctm) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    was_identity = _cairo_matrix_is_identity(&mut (*surface).cr.current_ctm);
    *matrix_updated = 1 as libc::c_int;
    (*surface).cr.current_ctm = *ctm;
    (*surface).cr.current_ctm.x0 = 0.0f64;
    (*surface).cr.current_ctm.y0 = 0.0f64;
    if _cairo_matrix_is_identity(&mut (*surface).cr.current_ctm) != 0 {
        _cairo_output_stream_write(
            (*ctx).stream,
            b"identity set-matrix\n\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            strlen(b"identity set-matrix\n\0" as *const u8 as *const libc::c_char),
        );
    } else if was_identity != 0 && fabs((*ctm).yx) < 1e-5f64 && fabs((*ctm).xy) < 1e-5f64
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"%f %f scale\n\0" as *const u8 as *const libc::c_char,
            (*ctm).xx,
            (*ctm).yy,
        );
    } else {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"[%f %f %f %f 0 0] set-matrix\n\0" as *const u8 as *const libc::c_char,
            (*ctm).xx,
            (*ctm).yx,
            (*ctm).xy,
            (*ctm).yy,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_font_matrix(
    mut surface: *mut cairo_script_surface_t,
    mut font_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    if target_is_active(surface) != 0 {} else {
        __assert_fail(
            b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            2016 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"cairo_status_t _emit_font_matrix(cairo_script_surface_t *, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if memcmp(
        &mut (*surface).cr.current_font_matrix as *mut cairo_matrix_t
            as *const libc::c_void,
        font_matrix as *const libc::c_void,
        ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).cr.current_font_matrix = *font_matrix;
    if _cairo_matrix_is_identity(font_matrix) != 0 {
        _cairo_output_stream_write(
            (*ctx).stream,
            b"identity set-font-matrix\n\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            strlen(b"identity set-font-matrix\n\0" as *const u8 as *const libc::c_char),
        );
    } else {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"[%f %f %f %f %f %f] set-font-matrix\n\0" as *const u8
                as *const libc::c_char,
            (*font_matrix).xx,
            (*font_matrix).yx,
            (*font_matrix).xy,
            (*font_matrix).yy,
            (*font_matrix).x0,
            (*font_matrix).y0,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_script_surface_create_similar(
    mut abstract_surface: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_script_surface_t = 0 as *mut cairo_script_surface_t;
    let mut other: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut passthrough: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut ctx: *mut cairo_script_context_t = 0 as *mut cairo_script_context_t;
    let mut extents: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    ctx = to_context(other);
    status = cairo_device_acquire(&mut (*ctx).base);
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    if (*other).emitted == 0 {
        status = _emit_surface(other);
        if status as u64 != 0 {
            cairo_device_release(&mut (*ctx).base);
            return _cairo_surface_create_in_error(status);
        }
        target_push(other);
    }
    if _cairo_surface_wrapper_is_active(&mut (*other).wrapper) != 0 {
        passthrough = _cairo_surface_wrapper_create_similar(
            &mut (*other).wrapper,
            content,
            width,
            height,
        );
        if (*passthrough).status as u64 != 0 {
            cairo_device_release(&mut (*ctx).base);
            return passthrough;
        }
    }
    extents.y = 0 as libc::c_int as libc::c_double;
    extents.x = extents.y;
    extents.width = width as libc::c_double;
    extents.height = height as libc::c_double;
    surface = _cairo_script_surface_create_internal(
        ctx,
        content,
        &mut extents,
        passthrough,
    );
    cairo_surface_destroy(passthrough);
    if (*surface).base.status as u64 != 0 {
        cairo_device_release(&mut (*ctx).base);
        return &mut (*surface).base;
    }
    _get_target(other);
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"%u %u //%s similar dup /s%u exch def context\n\0" as *const u8
            as *const libc::c_char,
        width,
        height,
        _content_to_string(content),
        (*surface).base.unique_id,
    );
    (*surface).emitted = 1 as libc::c_int;
    (*surface).defined = 1 as libc::c_int;
    let ref mut fresh27 = (*surface).base;
    (*fresh27).set_is_clear(1 as libc::c_int as libc::c_uint);
    target_push(surface);
    cairo_device_release(&mut (*ctx).base);
    return &mut (*surface).base;
}
unsafe extern "C" fn _device_flush(
    mut abstract_device: *mut libc::c_void,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = abstract_device
        as *mut cairo_script_context_t;
    return _cairo_output_stream_flush((*ctx).stream);
}
unsafe extern "C" fn _device_destroy(mut abstract_device: *mut libc::c_void) {
    let mut ctx: *mut cairo_script_context_t = abstract_device
        as *mut cairo_script_context_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    while cairo_list_is_empty(&mut (*ctx).fonts) == 0 {
        let mut font: *mut cairo_script_font_t = 0 as *mut cairo_script_font_t;
        font = ({
            let mut mptr__: *const cairo_list_t = (*ctx).fonts.next;
            (mptr__ as *mut libc::c_char).offset(-(56 as libc::c_ulong as isize))
                as *mut cairo_script_font_t
        });
        cairo_list_del(&mut (*font).base.link);
        cairo_list_del(&mut (*font).link);
        free(font as *mut libc::c_void);
    }
    _bitmap_fini((*ctx).surface_id.next);
    _bitmap_fini((*ctx).font_id.next);
    if (*ctx).owns_stream != 0 {
        status = _cairo_output_stream_destroy((*ctx).stream);
    }
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_script_surface_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    if !extents.is_null() {
        let ref mut fresh28 = (*extents).y;
        *fresh28 = 0 as libc::c_int;
        (*extents).x = *fresh28;
        (*extents).width = (*surface).width as libc::c_int;
        (*extents).height = (*surface).height as libc::c_int;
    }
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_script_surface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper) != 0 {
        return _cairo_surface_wrapper_acquire_source_image(
            &mut (*surface).wrapper,
            image_out,
            image_extra,
        );
    }
    return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
}
unsafe extern "C" fn _cairo_script_surface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_wrapper_is_active (&surface->wrapper)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            2177 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void _cairo_script_surface_release_source_image(void *, cairo_image_surface_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_surface_wrapper_release_source_image(
        &mut (*surface).wrapper,
        image,
        image_extra,
    );
}
unsafe extern "C" fn _cairo_script_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_surface_wrapper_fini(&mut (*surface).wrapper);
    free((*surface).cr.current_style.dash as *mut libc::c_void);
    let ref mut fresh29 = (*surface).cr.current_style.dash;
    *fresh29 = 0 as *mut libc::c_double;
    _cairo_pattern_fini(&mut (*surface).cr.current_source.base);
    _cairo_path_fixed_fini(&mut (*surface).cr.current_path);
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    status = cairo_device_acquire(&mut (*ctx).base);
    if status as u64 != 0 {
        return status;
    }
    if (*surface).emitted != 0 {
        if (*surface).active == 0 {} else {
            __assert_fail(
                b"! surface->active\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                2204 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"cairo_status_t _cairo_script_surface_finish(void *)\0"))
                    .as_ptr(),
            );
        }
        if cairo_list_is_empty(&mut (*surface).operand.link) == 0 {
            if (*ctx).active == 0 {
                if target_is_active(surface) != 0 {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"pop\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    let mut depth: libc::c_int = target_depth(surface);
                    if depth == 1 as libc::c_int {
                        _cairo_output_stream_printf(
                            (*ctx).stream,
                            b"exch pop\n\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        _cairo_output_stream_printf(
                            (*ctx).stream,
                            b"%d -1 roll pop\n\0" as *const u8 as *const libc::c_char,
                            depth,
                        );
                    }
                }
                cairo_list_del(&mut (*surface).operand.link);
            } else {
                let mut link: *mut deferred_finish = (if ::std::mem::size_of::<
                    deferred_finish,
                >() as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
                {
                    malloc(::std::mem::size_of::<deferred_finish>() as libc::c_ulong)
                } else {
                    0 as *mut libc::c_void
                }) as *mut deferred_finish;
                if link.is_null() {
                    status2 = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    if status as libc::c_uint
                        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    {
                        status = status2;
                    }
                    cairo_list_del(&mut (*surface).operand.link);
                } else {
                    (*link).operand.type_0 = DEFERRED;
                    cairo_list_swap(
                        &mut (*link).operand.link,
                        &mut (*surface).operand.link,
                    );
                    cairo_list_add(&mut (*link).link, &mut (*ctx).deferred);
                }
            }
        }
        if (*surface).defined != 0 {
            _cairo_output_stream_printf(
                (*ctx).stream,
                b"/s%u undef\n\0" as *const u8 as *const libc::c_char,
                (*surface).base.unique_id,
            );
        }
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _cairo_output_stream_flush((*to_context(surface)).stream);
    }
    cairo_device_release(&mut (*ctx).base);
    return status;
}
unsafe extern "C" fn _cairo_script_surface_copy_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_device_acquire((*surface).base.device);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _emit_context(surface);
    if !(status as u64 != 0) {
        _cairo_output_stream_write(
            (*to_context(surface)).stream,
            b"copy-page\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"copy-page\n\0" as *const u8 as *const libc::c_char),
        );
    }
    cairo_device_release((*surface).base.device);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_show_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_device_acquire((*surface).base.device);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _emit_context(surface);
    if !(status as u64 != 0) {
        _cairo_output_stream_write(
            (*to_context(surface)).stream,
            b"show-page\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"show-page\n\0" as *const u8 as *const libc::c_char),
        );
    }
    cairo_device_release((*surface).base.device);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_clipper_intersect_clip_path(
    mut clipper: *mut cairo_surface_clipper_t,
    mut path: *mut cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_status_t {
    let mut surface: *mut cairo_script_surface_t = ({
        let mut mptr__: *const cairo_surface_clipper_t = clipper;
        (mptr__ as *mut libc::c_char).offset(-(440 as libc::c_ulong as isize))
            as *mut cairo_script_surface_t
    });
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut matrix_updated: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    status = _emit_context(surface);
    if status as u64 != 0 {
        return status;
    }
    if path.is_null() {
        if (*surface).cr.has_clip != 0 {
            _cairo_output_stream_write(
                (*ctx).stream,
                b"reset-clip\n\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                strlen(b"reset-clip\n\0" as *const u8 as *const libc::c_char),
            );
            (*surface).cr.has_clip = 0 as libc::c_int;
        }
        return CAIRO_STATUS_SUCCESS;
    }
    if (*surface).width >= 0 as libc::c_int as libc::c_double
        && (*surface).height >= 0 as libc::c_int as libc::c_double
        && _cairo_path_fixed_is_box(path, &mut box_0) != 0
    {
        if box_0.p1.x <= 0 as libc::c_int && box_0.p1.y <= 0 as libc::c_int
            && box_0.p2.x >= _cairo_fixed_from_double((*surface).width)
            && box_0.p2.y >= _cairo_fixed_from_double((*surface).height)
        {
            return CAIRO_STATUS_SUCCESS;
        }
    }
    status = _emit_identity(surface, &mut matrix_updated) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _emit_fill_rule(surface, fill_rule);
    if status as u64 != 0 {
        return status;
    }
    if (*path).has_curve_to() != 0 {
        status = _emit_tolerance(surface, tolerance, matrix_updated);
        if status as u64 != 0 {
            return status;
        }
    }
    if _cairo_path_fixed_fill_maybe_region(path) == 0 {
        status = _emit_antialias(surface, antialias);
        if status as u64 != 0 {
            return status;
        }
    }
    status = _emit_path(surface, path, 1 as libc::c_int);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"clip+\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"clip+\n\0" as *const u8 as *const libc::c_char),
    );
    (*surface).cr.has_clip = 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn active(mut surface: *mut cairo_script_surface_t) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_device_acquire((*surface).base.device);
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh30 = (*surface).active;
    let fresh31 = *fresh30;
    *fresh30 = *fresh30 + 1;
    if fresh31 == 0 as libc::c_int {
        let ref mut fresh32 = (*to_context(surface)).active;
        *fresh32 += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn inactive(mut surface: *mut cairo_script_surface_t) {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut sorted: cairo_list_t = cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    };
    if (*surface).active > 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->active > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
            2386 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void inactive(cairo_script_surface_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh33 = (*surface).active;
    *fresh33 -= 1;
    if !(*fresh33 != 0) {
        if (*ctx).active > 0 as libc::c_int {} else {
            __assert_fail(
                b"ctx->active > 0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                2390 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void inactive(cairo_script_surface_t *)\0"))
                    .as_ptr(),
            );
        }
        let ref mut fresh34 = (*ctx).active;
        *fresh34 -= 1;
        if !(*fresh34 != 0) {
            cairo_list_init(&mut sorted);
            while cairo_list_is_empty(&mut (*ctx).deferred) == 0 {
                let mut df: *mut deferred_finish = 0 as *mut deferred_finish;
                let mut operand: *mut cairo_list_t = 0 as *mut cairo_list_t;
                let mut depth: libc::c_int = 0;
                df = ({
                    let mut mptr__: *const cairo_list_t = (*ctx).deferred.next;
                    (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                        as *mut deferred_finish
                });
                depth = 0 as libc::c_int;
                operand = (*ctx).operands.next;
                while operand != &mut (*ctx).operands as *mut cairo_list_t {
                    if operand == &mut (*df).operand.link as *mut cairo_list_t {
                        break;
                    }
                    depth += 1;
                    operand = (*operand).next;
                }
                (*df).operand.type_0 = depth as C2RustUnnamed_2;
                if cairo_list_is_empty(&mut sorted) != 0 {
                    cairo_list_move(&mut (*df).link, &mut sorted);
                } else {
                    let mut pos: *mut deferred_finish = 0 as *mut deferred_finish;
                    pos = ({
                        let mut mptr__: *const cairo_list_t = sorted.next;
                        (mptr__ as *mut libc::c_char)
                            .offset(-(0 as libc::c_ulong as isize))
                            as *mut deferred_finish
                    });
                    while &mut (*pos).link as *mut cairo_list_t
                        != &mut sorted as *mut cairo_list_t
                    {
                        if ((*df).operand.type_0 as libc::c_uint)
                            < (*pos).operand.type_0 as libc::c_uint
                        {
                            break;
                        }
                        pos = ({
                            let mut mptr__: *const cairo_list_t = (*pos).link.next;
                            (mptr__ as *mut libc::c_char)
                                .offset(-(0 as libc::c_ulong as isize))
                                as *mut deferred_finish
                        });
                    }
                    cairo_list_move_tail(&mut (*df).link, &mut (*pos).link);
                }
            }
            while cairo_list_is_empty(&mut sorted) == 0 {
                let mut df_0: *mut deferred_finish = 0 as *mut deferred_finish;
                let mut operand_0: *mut cairo_list_t = 0 as *mut cairo_list_t;
                let mut depth_0: libc::c_int = 0;
                df_0 = ({
                    let mut mptr__: *const cairo_list_t = sorted.next;
                    (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                        as *mut deferred_finish
                });
                depth_0 = 0 as libc::c_int;
                operand_0 = (*ctx).operands.next;
                while operand_0 != &mut (*ctx).operands as *mut cairo_list_t {
                    if operand_0 == &mut (*df_0).operand.link as *mut cairo_list_t {
                        break;
                    }
                    depth_0 += 1;
                    operand_0 = (*operand_0).next;
                }
                if depth_0 == 0 as libc::c_int {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"pop\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if depth_0 == 1 as libc::c_int {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"exch pop\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"%d -1 roll pop\n\0" as *const u8 as *const libc::c_char,
                        depth_0,
                    );
                }
                cairo_list_del(&mut (*df_0).operand.link);
                cairo_list_del(&mut (*df_0).link);
                free(df_0 as *mut libc::c_void);
            }
        }
    }
    cairo_device_release((*surface).base.device);
}
unsafe extern "C" fn _cairo_script_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = active(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
    if !(status as u64 != 0) {
        status = _emit_context(surface);
        if !(status as u64 != 0) {
            status = _emit_source(surface, op, source) as cairo_status_t;
            if !(status as u64 != 0) {
                status = _emit_operator(surface, op);
                if !(status as u64 != 0) {
                    _cairo_output_stream_write(
                        (*to_context(surface)).stream,
                        b"paint\n\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        strlen(b"paint\n\0" as *const u8 as *const libc::c_char),
                    );
                    inactive(surface);
                    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper) != 0 {
                        return _cairo_surface_wrapper_paint(
                            &mut (*surface).wrapper,
                            op,
                            source,
                            clip,
                        ) as cairo_int_status_t;
                    }
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
            }
        }
    }
    inactive(surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = active(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
    if !(status as u64 != 0) {
        status = _emit_context(surface);
        if !(status as u64 != 0) {
            status = _emit_source(surface, op, source) as cairo_status_t;
            if !(status as u64 != 0) {
                status = _emit_operator(surface, op);
                if !(status as u64 != 0) {
                    if _cairo_pattern_equal(source, mask) != 0 {
                        _cairo_output_stream_write(
                            (*to_context(surface)).stream,
                            b"/source get\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            strlen(b"/source get\0" as *const u8 as *const libc::c_char),
                        );
                        current_block = 5143058163439228106;
                    } else {
                        status = _emit_pattern(surface, mask) as cairo_status_t;
                        if status as u64 != 0 {
                            current_block = 6878358182522111461;
                        } else {
                            current_block = 5143058163439228106;
                        }
                    }
                    match current_block {
                        6878358182522111461 => {}
                        _ => {
                            if (*surface).cr.current_operator as libc::c_uint
                                == op as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"surface->cr.current_operator == op\0" as *const u8
                                        as *const libc::c_char,
                                    b"../src/cairo-script-surface.c\0" as *const u8
                                        as *const libc::c_char,
                                    2550 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 144],
                                        &[libc::c_char; 144],
                                    >(
                                        b"cairo_int_status_t _cairo_script_surface_mask(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_clip_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            _cairo_output_stream_write(
                                (*to_context(surface)).stream,
                                b" mask\n\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                strlen(b" mask\n\0" as *const u8 as *const libc::c_char),
                            );
                            inactive(surface);
                            if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper)
                                != 0
                            {
                                return _cairo_surface_wrapper_mask(
                                    &mut (*surface).wrapper,
                                    op,
                                    source,
                                    mask,
                                    clip,
                                ) as cairo_int_status_t;
                            }
                            return CAIRO_STATUS_SUCCESS as libc::c_int
                                as cairo_int_status_t;
                        }
                    }
                }
            }
        }
    }
    inactive(surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_stroke(
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
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut matrix_updated: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = active(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
    if !(status as u64 != 0) {
        status = _emit_context(surface);
        if !(status as u64 != 0) {
            status = _emit_identity(surface, &mut matrix_updated) as cairo_status_t;
            if !(status as u64 != 0) {
                status = _emit_path(surface, path, 0 as libc::c_int);
                if !(status as u64 != 0) {
                    status = _emit_source(surface, op, source) as cairo_status_t;
                    if !(status as u64 != 0) {
                        status = _emit_scaling_matrix(surface, ctm, &mut matrix_updated);
                        if !(status as u64 != 0) {
                            status = _emit_operator(surface, op);
                            if !(status as u64 != 0) {
                                if _scaling_matrix_equal(
                                    &mut (*surface).cr.current_ctm,
                                    &mut (*surface).cr.current_stroke_matrix,
                                ) != 0
                                {
                                    matrix_updated = 0 as libc::c_int;
                                } else {
                                    matrix_updated = 1 as libc::c_int;
                                    (*surface)
                                        .cr
                                        .current_stroke_matrix = (*surface).cr.current_ctm;
                                }
                                status = _emit_stroke_style(surface, style, matrix_updated);
                                if !(status as u64 != 0) {
                                    status = _emit_tolerance(
                                        surface,
                                        tolerance,
                                        matrix_updated,
                                    );
                                    if !(status as u64 != 0) {
                                        status = _emit_antialias(surface, antialias);
                                        if !(status as u64 != 0) {
                                            _cairo_output_stream_write(
                                                (*to_context(surface)).stream,
                                                b"stroke+\n\0" as *const u8 as *const libc::c_char
                                                    as *const libc::c_void,
                                                strlen(b"stroke+\n\0" as *const u8 as *const libc::c_char),
                                            );
                                            inactive(surface);
                                            if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper)
                                                != 0
                                            {
                                                return _cairo_surface_wrapper_stroke(
                                                    &mut (*surface).wrapper,
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
                                            return CAIRO_STATUS_SUCCESS as libc::c_int
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
    inactive(surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut matrix_updated: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    status = active(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
    if !(status as u64 != 0) {
        status = _emit_context(surface);
        if !(status as u64 != 0) {
            status = _emit_identity(surface, &mut matrix_updated) as cairo_status_t;
            if !(status as u64 != 0) {
                status = _emit_source(surface, op, source) as cairo_status_t;
                if !(status as u64 != 0) {
                    if _cairo_path_fixed_is_box(path, &mut box_0) == 0 {
                        status = _emit_fill_rule(surface, fill_rule);
                        if status as u64 != 0 {
                            current_block = 8359353012777144023;
                        } else {
                            current_block = 8831408221741692167;
                        }
                    } else {
                        current_block = 8831408221741692167;
                    }
                    match current_block {
                        8359353012777144023 => {}
                        _ => {
                            if (*path).has_curve_to() != 0 {
                                status = _emit_tolerance(
                                    surface,
                                    tolerance,
                                    matrix_updated,
                                );
                                if status as u64 != 0 {
                                    current_block = 8359353012777144023;
                                } else {
                                    current_block = 2838571290723028321;
                                }
                            } else {
                                current_block = 2838571290723028321;
                            }
                            match current_block {
                                8359353012777144023 => {}
                                _ => {
                                    if _cairo_path_fixed_fill_maybe_region(path) == 0 {
                                        status = _emit_antialias(surface, antialias);
                                        if status as u64 != 0 {
                                            current_block = 8359353012777144023;
                                        } else {
                                            current_block = 11307063007268554308;
                                        }
                                    } else {
                                        current_block = 11307063007268554308;
                                    }
                                    match current_block {
                                        8359353012777144023 => {}
                                        _ => {
                                            status = _emit_path(surface, path, 1 as libc::c_int);
                                            if !(status as u64 != 0) {
                                                status = _emit_operator(surface, op);
                                                if !(status as u64 != 0) {
                                                    _cairo_output_stream_write(
                                                        (*to_context(surface)).stream,
                                                        b"fill+\n\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        strlen(b"fill+\n\0" as *const u8 as *const libc::c_char),
                                                    );
                                                    inactive(surface);
                                                    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper)
                                                        != 0
                                                    {
                                                        return _cairo_surface_wrapper_fill(
                                                            &mut (*surface).wrapper,
                                                            op,
                                                            source,
                                                            path,
                                                            fill_rule,
                                                            tolerance,
                                                            antialias,
                                                            clip,
                                                        ) as cairo_int_status_t;
                                                    }
                                                    return CAIRO_STATUS_SUCCESS as libc::c_int
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
        }
    }
    inactive(surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_snapshot(
    mut abstract_surface: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper) != 0 {
        return _cairo_surface_wrapper_snapshot(&mut (*surface).wrapper);
    }
    return 0 as *mut cairo_surface_t;
}
unsafe extern "C" fn _cairo_script_surface_has_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
unsafe extern "C" fn _subpixel_order_to_string(
    mut subpixel_order: cairo_subpixel_order_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 5] = [
        b"SUBPIXEL_ORDER_DEFAULT\0" as *const u8 as *const libc::c_char,
        b"SUBPIXEL_ORDER_RGB\0" as *const u8 as *const libc::c_char,
        b"SUBPIXEL_ORDER_BGR\0" as *const u8 as *const libc::c_char,
        b"SUBPIXEL_ORDER_VRGB\0" as *const u8 as *const libc::c_char,
        b"SUBPIXEL_ORDER_VBGR\0" as *const u8 as *const libc::c_char,
    ];
    return names[subpixel_order as usize];
}
unsafe extern "C" fn _hint_style_to_string(
    mut hint_style: cairo_hint_style_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 5] = [
        b"HINT_STYLE_DEFAULT\0" as *const u8 as *const libc::c_char,
        b"HINT_STYLE_NONE\0" as *const u8 as *const libc::c_char,
        b"HINT_STYLE_SLIGHT\0" as *const u8 as *const libc::c_char,
        b"HINT_STYLE_MEDIUM\0" as *const u8 as *const libc::c_char,
        b"HINT_STYLE_FULL\0" as *const u8 as *const libc::c_char,
    ];
    return names[hint_style as usize];
}
unsafe extern "C" fn _hint_metrics_to_string(
    mut hint_metrics: cairo_hint_metrics_t,
) -> *const libc::c_char {
    static mut names: [*const libc::c_char; 3] = [
        b"HINT_METRICS_DEFAULT\0" as *const u8 as *const libc::c_char,
        b"HINT_METRICS_OFF\0" as *const u8 as *const libc::c_char,
        b"HINT_METRICS_ON\0" as *const u8 as *const libc::c_char,
    ];
    return names[hint_metrics as usize];
}
unsafe extern "C" fn _emit_font_options(
    mut surface: *mut cairo_script_surface_t,
    mut font_options: *mut cairo_font_options_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    if cairo_font_options_equal(&mut (*surface).cr.current_font_options, font_options)
        != 0
    {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"<<\0" as *const u8 as *const libc::c_char,
    );
    if (*font_options).antialias as libc::c_uint
        != (*surface).cr.current_font_options.antialias as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /antialias //%s\0" as *const u8 as *const libc::c_char,
            _antialias_to_string((*font_options).antialias),
        );
    }
    if (*font_options).subpixel_order as libc::c_uint
        != (*surface).cr.current_font_options.subpixel_order as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /subpixel-order //%s\0" as *const u8 as *const libc::c_char,
            _subpixel_order_to_string((*font_options).subpixel_order),
        );
    }
    if (*font_options).hint_style as libc::c_uint
        != (*surface).cr.current_font_options.hint_style as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /hint-style //%s\0" as *const u8 as *const libc::c_char,
            _hint_style_to_string((*font_options).hint_style),
        );
    }
    if (*font_options).hint_metrics as libc::c_uint
        != (*surface).cr.current_font_options.hint_metrics as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /hint-metrics //%s\0" as *const u8 as *const libc::c_char,
            _hint_metrics_to_string((*font_options).hint_metrics),
        );
    }
    _cairo_output_stream_printf(
        (*ctx).stream,
        b" >> set-font-options\n\0" as *const u8 as *const libc::c_char,
    );
    (*surface).cr.current_font_options = *font_options;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_script_scaled_font_fini(
    mut abstract_private: *mut cairo_scaled_font_private_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    let mut priv_0: *mut cairo_script_font_t = abstract_private
        as *mut cairo_script_font_t;
    let mut ctx: *mut cairo_script_context_t = (*abstract_private).key
        as *mut cairo_script_context_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_device_acquire(&mut (*ctx).base);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"/f%lu undef /sf%lu undef\n\0" as *const u8 as *const libc::c_char,
            (*priv_0).id,
            (*priv_0).id,
        );
        _bitmap_release_id(&mut (*ctx).font_id, (*priv_0).id);
        cairo_device_release(&mut (*ctx).base);
    }
    cairo_list_del(&mut (*priv_0).link);
    cairo_list_del(&mut (*priv_0).base.link);
    free(priv_0 as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_script_font_get(
    mut ctx: *mut cairo_script_context_t,
    mut font: *mut cairo_scaled_font_t,
) -> *mut cairo_script_font_t {
    return _cairo_scaled_font_find_private(font, ctx as *const libc::c_void)
        as *mut cairo_script_font_t;
}
unsafe extern "C" fn _cairo_script_font_id(
    mut ctx: *mut cairo_script_context_t,
    mut font: *mut cairo_scaled_font_t,
) -> libc::c_ulong {
    return (*_cairo_script_font_get(ctx, font)).id;
}
unsafe extern "C" fn _emit_type42_font(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut backend: *const cairo_scaled_font_backend_t = 0
        as *const cairo_scaled_font_backend_t;
    let mut base85_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut zlib_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut size: libc::c_ulong = 0;
    let mut load_flags: libc::c_uint = 0;
    let mut len: uint32_t = 0;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    backend = (*scaled_font).backend;
    if ((*backend).load_truetype_table).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    size = 0 as libc::c_int as libc::c_ulong;
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_uchar,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    buf = (if size != 0 as libc::c_int as libc::c_ulong {
        malloc(size)
    } else {
        0 as *mut libc::c_void
    }) as *mut uint8_t;
    if buf.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = ((*backend).load_truetype_table)
        .expect(
            "non-null function pointer",
        )(
        scaled_font as *mut libc::c_void,
        0 as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_long,
        buf,
        &mut size,
    ) as cairo_status_t;
    if status as u64 != 0 {
        free(buf as *mut libc::c_void);
        return status;
    }
    load_flags = _cairo_ft_scaled_font_get_load_flags(scaled_font);
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"<< /type 42 /index 0 /flags %d /source <|\0" as *const u8
            as *const libc::c_char,
        load_flags,
    );
    base85_stream = _cairo_base85_stream_create((*ctx).stream);
    len = __bswap_32(size as __uint32_t);
    _cairo_output_stream_write(
        base85_stream,
        &mut len as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    zlib_stream = _cairo_deflate_stream_create(base85_stream);
    _cairo_output_stream_write(zlib_stream, buf as *const libc::c_void, size);
    free(buf as *mut libc::c_void);
    status2 = _cairo_output_stream_destroy(zlib_stream);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    status2 = _cairo_output_stream_destroy(base85_stream);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"~> >> font dup /f%lu exch def set-font-face\0" as *const u8
            as *const libc::c_char,
        _cairo_script_font_id(ctx, scaled_font),
    );
    return status;
}
unsafe extern "C" fn _emit_scaled_font_init(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_out: *mut *mut cairo_script_font_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut font_private: *mut cairo_script_font_t = 0 as *mut cairo_script_font_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    font_private = (if ::std::mem::size_of::<cairo_script_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_script_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_script_font_t;
    if font_private.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_scaled_font_attach_private(
        scaled_font,
        &mut (*font_private).base,
        ctx as *const libc::c_void,
        Some(
            _cairo_script_scaled_font_fini
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_private_t,
                    *mut cairo_scaled_font_t,
                ) -> (),
        ),
    );
    let ref mut fresh35 = (*font_private).parent;
    *fresh35 = scaled_font;
    (*font_private).subset_glyph_index = 0 as libc::c_int as libc::c_ulong;
    (*font_private).has_sfnt = 1 as libc::c_int;
    cairo_list_add(&mut (*font_private).link, &mut (*ctx).fonts);
    status = _bitmap_next_id(&mut (*ctx).font_id, &mut (*font_private).id)
        as cairo_int_status_t;
    if status as u64 != 0 {
        free(font_private as *mut libc::c_void);
        return status as cairo_status_t;
    }
    status = _emit_context(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        free(font_private as *mut libc::c_void);
        return status as cairo_status_t;
    }
    status = _emit_type42_font(surface, scaled_font) as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        *font_out = font_private;
        return status as cairo_status_t;
    }
    (*font_private).has_sfnt = 0 as libc::c_int;
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"dict\n  /type 3 set\n  /metrics [%f %f %f %f %f] set\n  /glyphs array set\n  font dup /f%lu exch def set-font-face\0"
            as *const u8 as *const libc::c_char,
        (*scaled_font).fs_extents.ascent,
        (*scaled_font).fs_extents.descent,
        (*scaled_font).fs_extents.height,
        (*scaled_font).fs_extents.max_x_advance,
        (*scaled_font).fs_extents.max_y_advance,
        (*font_private).id,
    );
    *font_out = font_private;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_scaled_font(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
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
    let mut matrix_updated: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut font_private: *mut cairo_script_font_t = 0 as *mut cairo_script_font_t;
    cairo_scaled_font_get_ctm(scaled_font, &mut matrix);
    status = _emit_scaling_matrix(surface, &mut matrix, &mut matrix_updated);
    if status as u64 != 0 {
        return status;
    }
    if matrix_updated == 0 && (*surface).cr.current_scaled_font == scaled_font {
        return CAIRO_STATUS_SUCCESS;
    }
    let ref mut fresh36 = (*surface).cr.current_scaled_font;
    *fresh36 = scaled_font;
    font_private = _cairo_script_font_get(ctx, scaled_font);
    if font_private.is_null() {
        cairo_scaled_font_get_font_matrix(scaled_font, &mut matrix);
        status = _emit_font_matrix(surface, &mut matrix);
        if status as u64 != 0 {
            return status;
        }
        cairo_scaled_font_get_font_options(scaled_font, &mut options);
        status = _emit_font_options(surface, &mut options);
        if status as u64 != 0 {
            return status;
        }
        status = _emit_scaled_font_init(surface, scaled_font, &mut font_private);
        if status as u64 != 0 {
            return status;
        }
        if target_is_active(surface) != 0 {} else {
            __assert_fail(
                b"target_is_active (surface)\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-script-surface.c\0" as *const u8 as *const libc::c_char,
                3046 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"cairo_status_t _emit_scaled_font(cairo_script_surface_t *, cairo_scaled_font_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b" /scaled-font get /sf%lu exch def\n\0" as *const u8 as *const libc::c_char,
            (*font_private).id,
        );
    } else {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"sf%lu set-scaled-font\n\0" as *const u8 as *const libc::c_char,
            (*font_private).id,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_scaled_glyph_vector(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_private: *mut cairo_script_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut old_cr: cairo_script_implicit_context_t = cairo_script_implicit_context_t {
        current_operator: CAIRO_OPERATOR_CLEAR,
        current_fill_rule: CAIRO_FILL_RULE_WINDING,
        current_tolerance: 0.,
        current_antialias: CAIRO_ANTIALIAS_DEFAULT,
        current_style: cairo_stroke_style_t {
            line_width: 0.,
            line_cap: CAIRO_LINE_CAP_BUTT,
            line_join: CAIRO_LINE_JOIN_MITER,
            miter_limit: 0.,
            dash: 0 as *mut libc::c_double,
            num_dashes: 0,
            dash_offset: 0.,
            is_hairline: 0,
            pre_hairline_line_width: 0.,
        },
        current_source: cairo_pattern_union_t {
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
        current_ctm: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        current_stroke_matrix: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        current_font_matrix: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        current_font_options: cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        },
        current_scaled_font: 0 as *mut cairo_scaled_font_t,
        current_path: cairo_path_fixed_t {
            last_move_point: cairo_point_t { x: 0, y: 0 },
            current_point: cairo_point_t { x: 0, y: 0 },
            has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [0; 1],
            c2rust_padding: [0; 3],
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            buf: cairo_path_buf_fixed_t {
                base: cairo_path_buf_t {
                    link: cairo_list_t {
                        next: 0 as *mut _cairo_list,
                        prev: 0 as *mut _cairo_list,
                    },
                    num_ops: 0,
                    size_ops: 0,
                    num_points: 0,
                    size_points: 0,
                    op: 0 as *mut cairo_path_op_t,
                    points: 0 as *mut cairo_point_t,
                },
                op: [0; 27],
                points: [cairo_point_t { x: 0, y: 0 }; 54],
            },
        },
        has_clip: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut index: libc::c_ulong = 0;
    let ref mut fresh37 = (*font_private).subset_glyph_index;
    *fresh37 = (*fresh37).wrapping_add(1);
    index = *fresh37;
    let ref mut fresh38 = (*scaled_glyph).dev_private_key;
    *fresh38 = ctx as *const libc::c_void;
    let ref mut fresh39 = (*scaled_glyph).dev_private;
    *fresh39 = index as *mut libc::c_void;
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"%lu <<\n  /metrics [%f %f %f %f %f %f]\n  /render {\n\0" as *const u8
            as *const libc::c_char,
        index,
        (*scaled_glyph).fs_metrics.x_bearing,
        (*scaled_glyph).fs_metrics.y_bearing,
        (*scaled_glyph).fs_metrics.width,
        (*scaled_glyph).fs_metrics.height,
        (*scaled_glyph).fs_metrics.x_advance,
        (*scaled_glyph).fs_metrics.y_advance,
    );
    if _cairo_matrix_is_identity(&mut (*scaled_font).scale_inverse) == 0 {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"[%f %f %f %f %f %f] transform\n\0" as *const u8 as *const libc::c_char,
            (*scaled_font).scale_inverse.xx,
            (*scaled_font).scale_inverse.yx,
            (*scaled_font).scale_inverse.xy,
            (*scaled_font).scale_inverse.yy,
            (*scaled_font).scale_inverse.x0,
            (*scaled_font).scale_inverse.y0,
        );
    }
    old_cr = (*surface).cr;
    _cairo_script_implicit_context_init(&mut (*surface).cr);
    status = _cairo_recording_surface_replay(
        (*scaled_glyph).recording_surface,
        &mut (*surface).base,
    );
    (*surface).cr = old_cr;
    _cairo_output_stream_write(
        (*ctx).stream,
        b"} >> set\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"} >> set\n\0" as *const u8 as *const libc::c_char),
    );
    return status;
}
unsafe extern "C" fn _emit_scaled_glyph_bitmap(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_private: *mut cairo_script_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut index: libc::c_ulong = 0;
    let ref mut fresh40 = (*font_private).subset_glyph_index;
    *fresh40 = (*fresh40).wrapping_add(1);
    index = *fresh40;
    let ref mut fresh41 = (*scaled_glyph).dev_private_key;
    *fresh41 = ctx as *const libc::c_void;
    let ref mut fresh42 = (*scaled_glyph).dev_private;
    *fresh42 = index as *mut libc::c_void;
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"%lu <<\n  /metrics [%f %f %f %f %f %f]\n  /render {\n%f %f translate\n\0"
            as *const u8 as *const libc::c_char,
        index,
        (*scaled_glyph).fs_metrics.x_bearing,
        (*scaled_glyph).fs_metrics.y_bearing,
        (*scaled_glyph).fs_metrics.width,
        (*scaled_glyph).fs_metrics.height,
        (*scaled_glyph).fs_metrics.x_advance,
        (*scaled_glyph).fs_metrics.y_advance,
        (*scaled_glyph).fs_metrics.x_bearing,
        (*scaled_glyph).fs_metrics.y_bearing,
    );
    status = _emit_image_surface(surface, (*scaled_glyph).surface) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"pattern \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"pattern \0" as *const u8 as *const libc::c_char),
    );
    if _cairo_matrix_is_identity(&mut (*scaled_font).font_matrix) == 0 {
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"\n  [%f %f %f %f %f %f] set-matrix\n\0" as *const u8
                as *const libc::c_char,
            (*scaled_font).font_matrix.xx,
            (*scaled_font).font_matrix.yx,
            (*scaled_font).font_matrix.xy,
            (*scaled_font).font_matrix.yy,
            (*scaled_font).font_matrix.x0,
            (*scaled_font).font_matrix.y0,
        );
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b"mask\n} >> set\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"mask\n} >> set\n\0" as *const u8 as *const libc::c_char),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_scaled_glyph_prologue(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    _cairo_output_stream_printf(
        (*ctx).stream,
        b"f%lu /glyphs get\n\0" as *const u8 as *const libc::c_char,
        _cairo_script_font_id(ctx, scaled_font),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _emit_scaled_glyphs(
    mut surface: *mut cairo_script_surface_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_uint,
) -> cairo_status_t {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut font_private: *mut cairo_script_font_t = 0 as *mut cairo_script_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut n: libc::c_uint = 0;
    let mut have_glyph_prologue: cairo_bool_t = 0 as libc::c_int;
    if num_glyphs == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    font_private = _cairo_script_font_get(ctx, scaled_font);
    if (*font_private).has_sfnt != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_scaled_font_freeze_cache(scaled_font);
    n = 0 as libc::c_int as libc::c_uint;
    while n < num_glyphs {
        let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        status = _cairo_scaled_glyph_lookup(
            scaled_font,
            (*glyphs.offset(n as isize)).index,
            CAIRO_SCALED_GLYPH_INFO_METRICS,
            0 as *const cairo_color_t,
            &mut scaled_glyph,
        ) as cairo_status_t;
        if status as u64 != 0 {
            break;
        }
        if !((*scaled_glyph).dev_private_key == ctx as *const libc::c_void) {
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                (*glyphs.offset(n as isize)).index,
                CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE,
                0 as *const cairo_color_t,
                &mut scaled_glyph,
            ) as cairo_status_t;
            if status as libc::c_uint
                != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                && (status as libc::c_uint)
                    < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
            {
                break;
            }
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                if have_glyph_prologue == 0 {
                    status = _emit_scaled_glyph_prologue(surface, scaled_font);
                    if status as u64 != 0 {
                        break;
                    }
                    have_glyph_prologue = 1 as libc::c_int;
                }
                status = _emit_scaled_glyph_vector(
                    surface,
                    scaled_font,
                    font_private,
                    scaled_glyph,
                );
                if status as u64 != 0 {
                    break;
                }
            } else {
                status = _cairo_scaled_glyph_lookup(
                    scaled_font,
                    (*glyphs.offset(n as isize)).index,
                    CAIRO_SCALED_GLYPH_INFO_SURFACE,
                    0 as *const cairo_color_t,
                    &mut scaled_glyph,
                ) as cairo_status_t;
                if status as libc::c_uint
                    != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    && (status as libc::c_uint)
                        < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
                {
                    break;
                }
                if status as libc::c_uint
                    == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    if have_glyph_prologue == 0 {
                        status = _emit_scaled_glyph_prologue(surface, scaled_font);
                        if status as u64 != 0 {
                            break;
                        }
                        have_glyph_prologue = 1 as libc::c_int;
                    }
                    status = _emit_scaled_glyph_bitmap(
                        surface,
                        scaled_font,
                        font_private,
                        scaled_glyph,
                    );
                    if status as u64 != 0 {
                        break;
                    }
                }
            }
        }
        n = n.wrapping_add(1);
    }
    _cairo_scaled_font_thaw_cache(scaled_font);
    if have_glyph_prologue != 0 {
        _cairo_output_stream_write(
            (*to_context(surface)).stream,
            b"pop pop\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"pop pop\n\0" as *const u8 as *const libc::c_char),
        );
    }
    return status;
}
unsafe extern "C" fn to_octal(
    mut value: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) {
    loop {
        size = size.wrapping_sub(1);
        *buf
            .offset(
                size as isize,
            ) = ('0' as i32 + (value & 7 as libc::c_int)) as libc::c_char;
        value >>= 3 as libc::c_int;
        if !(size != 0) {
            break;
        }
    };
}
unsafe extern "C" fn _emit_string_literal(
    mut surface: *mut cairo_script_surface_t,
    mut utf8: *const libc::c_char,
    mut len: libc::c_int,
) {
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut c: libc::c_char = 0;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    _cairo_output_stream_write(
        (*ctx).stream,
        b"(\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"(\0" as *const u8 as *const libc::c_char),
    );
    if utf8.is_null() {
        end = utf8;
    } else {
        if len < 0 as libc::c_int {
            len = strlen(utf8) as libc::c_int;
        }
        end = utf8.offset(len as isize);
    }
    while utf8 < end {
        let fresh43 = utf8;
        utf8 = utf8.offset(1);
        c = *fresh43;
        match c as libc::c_int {
            10 => {
                c = 'n' as i32 as libc::c_char;
            }
            13 => {
                c = 'r' as i32 as libc::c_char;
            }
            9 => {
                c = 't' as i32 as libc::c_char;
            }
            8 => {
                c = 'b' as i32 as libc::c_char;
            }
            12 => {
                c = 'f' as i32 as libc::c_char;
            }
            92 | 40 | 41 => {}
            _ => {
                if _cairo_isprint(c as libc::c_int) != 0 {
                    _cairo_output_stream_printf(
                        (*ctx).stream,
                        b"%c\0" as *const u8 as *const libc::c_char,
                        c as libc::c_int,
                    );
                } else {
                    let mut buf: [libc::c_char; 4] = [
                        '\\' as i32 as libc::c_char,
                        0,
                        0,
                        0,
                    ];
                    to_octal(
                        c as libc::c_int,
                        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                        3 as libc::c_int as size_t,
                    );
                    _cairo_output_stream_write(
                        (*ctx).stream,
                        buf.as_mut_ptr() as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    );
                }
                continue;
            }
        }
        _cairo_output_stream_printf(
            (*ctx).stream,
            b"\\%c\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
        );
    }
    _cairo_output_stream_write(
        (*ctx).stream,
        b")\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b")\0" as *const u8 as *const libc::c_char),
    );
}
unsafe extern "C" fn _cairo_script_surface_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut clusters: *const cairo_text_cluster_t,
    mut num_clusters: libc::c_int,
    mut backward: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    let mut ctx: *mut cairo_script_context_t = to_context(surface);
    let mut font_private: *mut cairo_script_font_t = 0 as *mut cairo_script_font_t;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut ix: libc::c_double = 0.;
    let mut iy: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    let mut base85_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    status = active(surface);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
    if !(status as u64 != 0) {
        status = _emit_context(surface);
        if !(status as u64 != 0) {
            status = _emit_source(surface, op, source) as cairo_status_t;
            if !(status as u64 != 0) {
                status = _emit_scaled_font(surface, scaled_font);
                if !(status as u64 != 0) {
                    status = _emit_operator(surface, op);
                    if !(status as u64 != 0) {
                        status = _emit_scaled_glyphs(
                            surface,
                            scaled_font,
                            glyphs,
                            num_glyphs as libc::c_uint,
                        );
                        if !(status as u64 != 0) {
                            if !utf8.is_null() && !clusters.is_null() {
                                _emit_string_literal(surface, utf8, utf8_len);
                                _cairo_output_stream_write(
                                    (*ctx).stream,
                                    b" \0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    strlen(b" \0" as *const u8 as *const libc::c_char),
                                );
                            }
                            matrix = (*surface).cr.current_ctm;
                            status = cairo_matrix_invert(&mut matrix);
                            if status as libc::c_uint
                                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                                        as *const libc::c_char,
                                    b"../src/cairo-script-surface.c\0" as *const u8
                                        as *const libc::c_char,
                                    3394 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 258],
                                        &[libc::c_char; 258],
                                    >(
                                        b"cairo_int_status_t _cairo_script_surface_show_text_glyphs(void *, cairo_operator_t, const cairo_pattern_t *, const char *, int, cairo_glyph_t *, int, const cairo_text_cluster_t *, int, cairo_text_cluster_flags_t, cairo_scaled_font_t *, const cairo_clip_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            x = (*glyphs.offset(0 as libc::c_int as isize)).x;
                            ix = x;
                            y = (*glyphs.offset(0 as libc::c_int as isize)).y;
                            iy = y;
                            cairo_matrix_transform_point(&mut matrix, &mut ix, &mut iy);
                            ix -= (*scaled_font).font_matrix.x0;
                            iy -= (*scaled_font).font_matrix.y0;
                            _cairo_scaled_font_freeze_cache(scaled_font);
                            font_private = _cairo_script_font_get(ctx, scaled_font);
                            _cairo_output_stream_printf(
                                (*ctx).stream,
                                b"[%f %f \0" as *const u8 as *const libc::c_char,
                                ix,
                                iy,
                            );
                            n = 0 as libc::c_int;
                            loop {
                                if !(n < num_glyphs) {
                                    current_block = 2480299350034459858;
                                    break;
                                }
                                if (*font_private).has_sfnt != 0 {
                                    if (*glyphs.offset(n as isize)).index
                                        > 256 as libc::c_int as libc::c_ulong
                                    {
                                        current_block = 2480299350034459858;
                                        break;
                                    }
                                } else {
                                    status = _cairo_scaled_glyph_lookup(
                                        scaled_font,
                                        (*glyphs.offset(n as isize)).index,
                                        CAIRO_SCALED_GLYPH_INFO_METRICS,
                                        0 as *const cairo_color_t,
                                        &mut scaled_glyph,
                                    ) as cairo_status_t;
                                    if status as u64 != 0 {
                                        _cairo_scaled_font_thaw_cache(scaled_font);
                                        current_block = 9649627425948382823;
                                        break;
                                    } else if (*scaled_glyph).dev_private as uintptr_t
                                        > 256 as libc::c_int as libc::c_ulong
                                    {
                                        current_block = 2480299350034459858;
                                        break;
                                    }
                                }
                                n += 1;
                            }
                            match current_block {
                                9649627425948382823 => {}
                                _ => {
                                    if n == num_glyphs {
                                        _cairo_output_stream_write(
                                            (*ctx).stream,
                                            b"<~\0" as *const u8 as *const libc::c_char
                                                as *const libc::c_void,
                                            strlen(b"<~\0" as *const u8 as *const libc::c_char),
                                        );
                                        base85_stream = _cairo_base85_stream_create((*ctx).stream);
                                    } else {
                                        _cairo_output_stream_write(
                                            (*ctx).stream,
                                            b"[\0" as *const u8 as *const libc::c_char
                                                as *const libc::c_void,
                                            strlen(b"[\0" as *const u8 as *const libc::c_char),
                                        );
                                    }
                                    n = 0 as libc::c_int;
                                    loop {
                                        if !(n < num_glyphs) {
                                            current_block = 8723848109087415604;
                                            break;
                                        }
                                        let mut dx: libc::c_double = 0.;
                                        let mut dy: libc::c_double = 0.;
                                        status = _cairo_scaled_glyph_lookup(
                                            scaled_font,
                                            (*glyphs.offset(n as isize)).index,
                                            CAIRO_SCALED_GLYPH_INFO_METRICS,
                                            0 as *const cairo_color_t,
                                            &mut scaled_glyph,
                                        ) as cairo_status_t;
                                        if status as u64 != 0 {
                                            _cairo_scaled_font_thaw_cache(scaled_font);
                                            current_block = 9649627425948382823;
                                            break;
                                        } else {
                                            if fabs((*glyphs.offset(n as isize)).x - x) > 1e-5f64
                                                || fabs((*glyphs.offset(n as isize)).y - y) > 1e-5f64
                                            {
                                                if fabs((*glyphs.offset(n as isize)).y - y) < 1e-5f64 {
                                                    if !base85_stream.is_null() {
                                                        status = _cairo_output_stream_destroy(base85_stream);
                                                        if status as u64 != 0 {
                                                            base85_stream = 0 as *mut cairo_output_stream_t;
                                                            current_block = 8723848109087415604;
                                                            break;
                                                        } else {
                                                            _cairo_output_stream_printf(
                                                                (*ctx).stream,
                                                                b"~> %f <~\0" as *const u8 as *const libc::c_char,
                                                                (*glyphs.offset(n as isize)).x - x,
                                                            );
                                                            base85_stream = _cairo_base85_stream_create((*ctx).stream);
                                                        }
                                                    } else {
                                                        _cairo_output_stream_printf(
                                                            (*ctx).stream,
                                                            b" ] %f [ \0" as *const u8 as *const libc::c_char,
                                                            (*glyphs.offset(n as isize)).x - x,
                                                        );
                                                    }
                                                    x = (*glyphs.offset(n as isize)).x;
                                                } else {
                                                    x = (*glyphs.offset(n as isize)).x;
                                                    ix = x;
                                                    y = (*glyphs.offset(n as isize)).y;
                                                    iy = y;
                                                    cairo_matrix_transform_point(&mut matrix, &mut ix, &mut iy);
                                                    ix -= (*scaled_font).font_matrix.x0;
                                                    iy -= (*scaled_font).font_matrix.y0;
                                                    if !base85_stream.is_null() {
                                                        status = _cairo_output_stream_destroy(base85_stream);
                                                        if status as u64 != 0 {
                                                            base85_stream = 0 as *mut cairo_output_stream_t;
                                                            current_block = 8723848109087415604;
                                                            break;
                                                        } else {
                                                            _cairo_output_stream_printf(
                                                                (*ctx).stream,
                                                                b"~> %f %f <~\0" as *const u8 as *const libc::c_char,
                                                                ix,
                                                                iy,
                                                            );
                                                            base85_stream = _cairo_base85_stream_create((*ctx).stream);
                                                        }
                                                    } else {
                                                        _cairo_output_stream_printf(
                                                            (*ctx).stream,
                                                            b" ] %f %f [ \0" as *const u8 as *const libc::c_char,
                                                            ix,
                                                            iy,
                                                        );
                                                    }
                                                }
                                            }
                                            if !base85_stream.is_null() {
                                                let mut c: uint8_t = 0;
                                                if (*font_private).has_sfnt != 0 {
                                                    c = (*glyphs.offset(n as isize)).index as uint8_t;
                                                } else {
                                                    c = (*scaled_glyph).dev_private as uintptr_t as uint8_t;
                                                }
                                                _cairo_output_stream_write(
                                                    base85_stream,
                                                    &mut c as *mut uint8_t as *const libc::c_void,
                                                    1 as libc::c_int as size_t,
                                                );
                                            } else if (*font_private).has_sfnt != 0 {
                                                _cairo_output_stream_printf(
                                                    (*ctx).stream,
                                                    b" %lu\0" as *const u8 as *const libc::c_char,
                                                    (*glyphs.offset(n as isize)).index,
                                                );
                                            } else {
                                                _cairo_output_stream_printf(
                                                    (*ctx).stream,
                                                    b" %lu\0" as *const u8 as *const libc::c_char,
                                                    (*scaled_glyph).dev_private as uintptr_t,
                                                );
                                            }
                                            dx = (*scaled_glyph).metrics.x_advance;
                                            dy = (*scaled_glyph).metrics.y_advance;
                                            cairo_matrix_transform_distance(
                                                &mut (*scaled_font).ctm,
                                                &mut dx,
                                                &mut dy,
                                            );
                                            x += dx;
                                            y += dy;
                                            n += 1;
                                        }
                                    }
                                    match current_block {
                                        9649627425948382823 => {}
                                        _ => {
                                            _cairo_scaled_font_thaw_cache(scaled_font);
                                            if !base85_stream.is_null() {
                                                let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
                                                status2 = _cairo_output_stream_destroy(base85_stream);
                                                if status as libc::c_uint
                                                    == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                                                {
                                                    status = status2;
                                                }
                                                _cairo_output_stream_printf(
                                                    (*ctx).stream,
                                                    b"~>\0" as *const u8 as *const libc::c_char,
                                                );
                                            } else {
                                                _cairo_output_stream_write(
                                                    (*ctx).stream,
                                                    b" ]\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    strlen(b" ]\0" as *const u8 as *const libc::c_char),
                                                );
                                            }
                                            if status as u64 != 0 {
                                                return status as cairo_int_status_t;
                                            }
                                            if !utf8.is_null() && !clusters.is_null() {
                                                n = 0 as libc::c_int;
                                                while n < num_clusters {
                                                    if (*clusters.offset(n as isize)).num_bytes
                                                        > 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                        || (*clusters.offset(n as isize)).num_glyphs
                                                            > 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                                    {
                                                        break;
                                                    }
                                                    n += 1;
                                                }
                                                if n < num_clusters {
                                                    _cairo_output_stream_write(
                                                        (*ctx).stream,
                                                        b"] [ \0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        strlen(b"] [ \0" as *const u8 as *const libc::c_char),
                                                    );
                                                    n = 0 as libc::c_int;
                                                    while n < num_clusters {
                                                        _cairo_output_stream_printf(
                                                            (*ctx).stream,
                                                            b"%d %d \0" as *const u8 as *const libc::c_char,
                                                            (*clusters.offset(n as isize)).num_bytes,
                                                            (*clusters.offset(n as isize)).num_glyphs,
                                                        );
                                                        n += 1;
                                                    }
                                                    _cairo_output_stream_write(
                                                        (*ctx).stream,
                                                        b"]\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        strlen(b"]\0" as *const u8 as *const libc::c_char),
                                                    );
                                                    current_block = 17336970397495664729;
                                                } else {
                                                    _cairo_output_stream_write(
                                                        (*ctx).stream,
                                                        b"] <~\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        strlen(b"] <~\0" as *const u8 as *const libc::c_char),
                                                    );
                                                    base85_stream = _cairo_base85_stream_create((*ctx).stream);
                                                    n = 0 as libc::c_int;
                                                    while n < num_clusters {
                                                        let mut c_0: [uint8_t; 2] = [0; 2];
                                                        c_0[0 as libc::c_int
                                                            as usize] = (*clusters.offset(n as isize)).num_bytes
                                                            as uint8_t;
                                                        c_0[1 as libc::c_int
                                                            as usize] = (*clusters.offset(n as isize)).num_glyphs
                                                            as uint8_t;
                                                        _cairo_output_stream_write(
                                                            base85_stream,
                                                            c_0.as_mut_ptr() as *const libc::c_void,
                                                            2 as libc::c_int as size_t,
                                                        );
                                                        n += 1;
                                                    }
                                                    status = _cairo_output_stream_destroy(base85_stream);
                                                    if status as u64 != 0 {
                                                        current_block = 9649627425948382823;
                                                    } else {
                                                        _cairo_output_stream_write(
                                                            (*ctx).stream,
                                                            b"~>\0" as *const u8 as *const libc::c_char
                                                                as *const libc::c_void,
                                                            strlen(b"~>\0" as *const u8 as *const libc::c_char),
                                                        );
                                                        current_block = 17336970397495664729;
                                                    }
                                                }
                                                match current_block {
                                                    9649627425948382823 => {}
                                                    _ => {
                                                        _cairo_output_stream_printf(
                                                            (*ctx).stream,
                                                            b" //%s show-text-glyphs\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            _direction_to_string(backward as cairo_bool_t),
                                                        );
                                                        current_block = 17392506108461345148;
                                                    }
                                                }
                                            } else {
                                                _cairo_output_stream_write(
                                                    (*ctx).stream,
                                                    b"] show-glyphs\n\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    strlen(
                                                        b"] show-glyphs\n\0" as *const u8 as *const libc::c_char,
                                                    ),
                                                );
                                                current_block = 17392506108461345148;
                                            }
                                            match current_block {
                                                9649627425948382823 => {}
                                                _ => {
                                                    inactive(surface);
                                                    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper)
                                                        != 0
                                                    {
                                                        return _cairo_surface_wrapper_show_text_glyphs(
                                                            &mut (*surface).wrapper,
                                                            op,
                                                            source,
                                                            utf8,
                                                            utf8_len,
                                                            glyphs,
                                                            num_glyphs,
                                                            clusters,
                                                            num_clusters,
                                                            backward,
                                                            scaled_font,
                                                            clip,
                                                        ) as cairo_int_status_t;
                                                    }
                                                    return CAIRO_STATUS_SUCCESS as libc::c_int
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
        }
    }
    inactive(surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_script_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_script_surface_t = abstract_surface
        as *mut cairo_script_surface_t;
    if _cairo_surface_wrapper_is_active(&mut (*surface).wrapper) != 0 {
        return _cairo_surface_wrapper_get_extents(&mut (*surface).wrapper, rectangle);
    }
    if (*surface).width < 0 as libc::c_int as libc::c_double
        || (*surface).height < 0 as libc::c_int as libc::c_double
    {
        return 0 as libc::c_int;
    }
    (*rectangle).x = 0 as libc::c_int;
    (*rectangle).y = 0 as libc::c_int;
    (*rectangle).width = (*surface).width as libc::c_int;
    (*rectangle).height = (*surface).height as libc::c_int;
    return 1 as libc::c_int;
}
static mut _cairo_script_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_SCRIPT,
            finish: Some(
                _cairo_script_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_script_surface_create_similar
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
                _cairo_script_surface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_script_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_script_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_script_surface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: Some(
                _cairo_script_surface_copy_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            show_page: Some(
                _cairo_script_surface_show_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            get_extents: Some(
                _cairo_script_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_script_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_script_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_script_surface_stroke
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
                _cairo_script_surface_fill
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
                _cairo_script_surface_has_show_text_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            show_text_glyphs: Some(
                _cairo_script_surface_show_text_glyphs
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
            tag: None,
        };
        init
    }
};
unsafe extern "C" fn _cairo_script_implicit_context_init(
    mut cr: *mut cairo_script_implicit_context_t,
) {
    (*cr).current_operator = CAIRO_OPERATOR_OVER;
    (*cr).current_fill_rule = CAIRO_FILL_RULE_WINDING;
    (*cr).current_tolerance = 0.1f64;
    (*cr).current_antialias = CAIRO_ANTIALIAS_DEFAULT;
    _cairo_stroke_style_init(&mut (*cr).current_style);
    _cairo_pattern_init_solid(
        &mut (*cr).current_source.solid,
        _cairo_stock_color(CAIRO_STOCK_BLACK),
    );
    _cairo_path_fixed_init(&mut (*cr).current_path);
    cairo_matrix_init_identity(&mut (*cr).current_ctm);
    cairo_matrix_init_identity(&mut (*cr).current_stroke_matrix);
    cairo_matrix_init_identity(&mut (*cr).current_font_matrix);
    _cairo_font_options_init_default(&mut (*cr).current_font_options);
    let ref mut fresh44 = (*cr).current_scaled_font;
    *fresh44 = 0 as *mut cairo_scaled_font_t;
    (*cr).has_clip = 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_script_implicit_context_reset(
    mut cr: *mut cairo_script_implicit_context_t,
) {
    free((*cr).current_style.dash as *mut libc::c_void);
    let ref mut fresh45 = (*cr).current_style.dash;
    *fresh45 = 0 as *mut libc::c_double;
    _cairo_pattern_fini(&mut (*cr).current_source.base);
    _cairo_path_fixed_fini(&mut (*cr).current_path);
    _cairo_script_implicit_context_init(cr);
}
unsafe extern "C" fn _cairo_script_surface_create_internal(
    mut ctx: *mut cairo_script_context_t,
    mut content: cairo_content_t,
    mut extents: *mut cairo_rectangle_t,
    mut passthrough: *mut cairo_surface_t,
) -> *mut cairo_script_surface_t {
    let mut surface: *mut cairo_script_surface_t = 0 as *mut cairo_script_surface_t;
    if ctx.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NULL_POINTER))
            as *mut cairo_script_surface_t;
    }
    surface = (if ::std::mem::size_of::<cairo_script_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_script_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_script_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY))
            as *mut cairo_script_surface_t;
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_script_surface_backend,
        &mut (*ctx).base,
        content,
        1 as libc::c_int,
    );
    _cairo_surface_wrapper_init(&mut (*surface).wrapper, passthrough);
    _cairo_surface_clipper_init(
        &mut (*surface).clipper,
        Some(
            _cairo_script_surface_clipper_intersect_clip_path
                as unsafe extern "C" fn(
                    *mut cairo_surface_clipper_t,
                    *mut cairo_path_fixed_t,
                    cairo_fill_rule_t,
                    libc::c_double,
                    cairo_antialias_t,
                ) -> cairo_status_t,
        ),
    );
    let ref mut fresh46 = (*surface).height;
    *fresh46 = -(1 as libc::c_int) as libc::c_double;
    (*surface).width = *fresh46;
    if !extents.is_null() {
        (*surface).width = (*extents).width;
        (*surface).height = (*extents).height;
        cairo_surface_set_device_offset(
            &mut (*surface).base,
            -(*extents).x,
            -(*extents).y,
        );
    }
    (*surface).emitted = 0 as libc::c_int;
    (*surface).defined = 0 as libc::c_int;
    (*surface).active = 0 as libc::c_int;
    (*surface).operand.type_0 = SURFACE;
    cairo_list_init(&mut (*surface).operand.link);
    _cairo_script_implicit_context_init(&mut (*surface).cr);
    return surface;
}
static mut _cairo_script_device_backend: cairo_device_backend_t = unsafe {
    {
        let mut init = _cairo_device_backend {
            type_0: CAIRO_DEVICE_TYPE_SCRIPT,
            lock: None,
            unlock: None,
            flush: Some(
                _device_flush
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            finish: None,
            destroy: Some(
                _device_destroy as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_script_context_create_internal(
    mut stream: *mut cairo_output_stream_t,
) -> *mut cairo_device_t {
    let mut ctx: *mut cairo_script_context_t = 0 as *mut cairo_script_context_t;
    ctx = (if ::std::mem::size_of::<cairo_script_context_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_script_context_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_script_context_t;
    if ctx.is_null() {
        return _cairo_device_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_script_context_t>() as libc::c_ulong,
    );
    _cairo_device_init(&mut (*ctx).base, &_cairo_script_device_backend);
    cairo_list_init(&mut (*ctx).operands);
    cairo_list_init(&mut (*ctx).deferred);
    let ref mut fresh47 = (*ctx).stream;
    *fresh47 = stream;
    (*ctx).mode = CAIRO_SCRIPT_MODE_ASCII;
    cairo_list_init(&mut (*ctx).fonts);
    cairo_list_init(&mut (*ctx).defines);
    (*ctx).attach_snapshots = 1 as libc::c_int;
    return &mut (*ctx).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_script_context_attach_snapshots(
    mut device: *mut cairo_device_t,
    mut enable: cairo_bool_t,
) {
    let mut ctx: *mut cairo_script_context_t = 0 as *mut cairo_script_context_t;
    ctx = device as *mut cairo_script_context_t;
    (*ctx).attach_snapshots = enable;
}
unsafe extern "C" fn _cairo_script_context_create(
    mut stream: *mut cairo_output_stream_t,
) -> *mut cairo_device_t {
    let mut ctx: *mut cairo_script_context_t = 0 as *mut cairo_script_context_t;
    ctx = _cairo_script_context_create_internal(stream) as *mut cairo_script_context_t;
    if (*ctx).base.status as u64 != 0 {
        return &mut (*ctx).base;
    }
    (*ctx).owns_stream = 1 as libc::c_int;
    _cairo_output_stream_write(
        (*ctx).stream,
        b"%!CairoScript\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"%!CairoScript\n\0" as *const u8 as *const libc::c_char),
    );
    return &mut (*ctx).base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_create(
    mut filename: *const libc::c_char,
) -> *mut cairo_device_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    stream = _cairo_output_stream_create_for_filename(filename);
    status = _cairo_output_stream_get_status(stream);
    if status as u64 != 0 {
        return _cairo_device_create_in_error(status);
    }
    return _cairo_script_context_create(stream);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_create_for_stream(
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
) -> *mut cairo_device_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    stream = _cairo_output_stream_create(write_func, None, closure);
    status = _cairo_output_stream_get_status(stream);
    if status as u64 != 0 {
        return _cairo_device_create_in_error(status);
    }
    return _cairo_script_context_create(stream);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_write_comment(
    mut script: *mut cairo_device_t,
    mut comment: *const libc::c_char,
    mut len: libc::c_int,
) {
    let mut context: *mut cairo_script_context_t = script as *mut cairo_script_context_t;
    if len < 0 as libc::c_int {
        len = strlen(comment) as libc::c_int;
    }
    _cairo_output_stream_write(
        (*context).stream,
        b"% \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"% \0" as *const u8 as *const libc::c_char),
    );
    _cairo_output_stream_write(
        (*context).stream,
        comment as *const libc::c_void,
        len as size_t,
    );
    _cairo_output_stream_write(
        (*context).stream,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"\n\0" as *const u8 as *const libc::c_char),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_set_mode(
    mut script: *mut cairo_device_t,
    mut mode: cairo_script_mode_t,
) {
    let mut context: *mut cairo_script_context_t = script as *mut cairo_script_context_t;
    (*context).mode = mode;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_get_mode(
    mut script: *mut cairo_device_t,
) -> cairo_script_mode_t {
    let mut context: *mut cairo_script_context_t = script as *mut cairo_script_context_t;
    return (*context).mode;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_surface_create(
    mut script: *mut cairo_device_t,
    mut content: cairo_content_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> *mut cairo_surface_t {
    let mut extents: *mut cairo_rectangle_t = 0 as *mut cairo_rectangle_t;
    let mut r: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    if (*(*script).backend).type_0 as libc::c_int
        != CAIRO_DEVICE_TYPE_SCRIPT as libc::c_int
    {
        return _cairo_surface_create_in_error(CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
    }
    if (*script).status as u64 != 0 {
        return _cairo_surface_create_in_error((*script).status);
    }
    extents = 0 as *mut cairo_rectangle_t;
    if width > 0 as libc::c_int as libc::c_double
        && height > 0 as libc::c_int as libc::c_double
    {
        r.y = 0 as libc::c_int as libc::c_double;
        r.x = r.y;
        r.width = width;
        r.height = height;
        extents = &mut r;
    }
    return &mut (*(_cairo_script_surface_create_internal
        as unsafe extern "C" fn(
            *mut cairo_script_context_t,
            cairo_content_t,
            *mut cairo_rectangle_t,
            *mut cairo_surface_t,
        ) -> *mut cairo_script_surface_t)(
        script as *mut cairo_script_context_t,
        content,
        extents,
        0 as *mut cairo_surface_t,
    ))
        .base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_surface_create_for_target(
    mut script: *mut cairo_device_t,
    mut target: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut rect: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    let mut r: *mut cairo_rectangle_t = 0 as *mut cairo_rectangle_t;
    if (*(*script).backend).type_0 as libc::c_int
        != CAIRO_DEVICE_TYPE_SCRIPT as libc::c_int
    {
        return _cairo_surface_create_in_error(CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
    }
    if (*script).status as u64 != 0 {
        return _cairo_surface_create_in_error((*script).status);
    }
    if (*target).status as u64 != 0 {
        return _cairo_surface_create_in_error((*target).status);
    }
    r = 0 as *mut cairo_rectangle_t;
    if _cairo_surface_get_extents(target, &mut extents) != 0 {
        rect.y = 0 as libc::c_int as libc::c_double;
        rect.x = rect.y;
        rect.width = extents.width as libc::c_double;
        rect.height = extents.height as libc::c_double;
        r = &mut rect;
    }
    return &mut (*(_cairo_script_surface_create_internal
        as unsafe extern "C" fn(
            *mut cairo_script_context_t,
            cairo_content_t,
            *mut cairo_rectangle_t,
            *mut cairo_surface_t,
        ) -> *mut cairo_script_surface_t)(
        script as *mut cairo_script_context_t,
        (*target).content,
        r,
        target,
    ))
        .base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_script_from_recording_surface(
    mut script: *mut cairo_device_t,
    mut recording_surface: *mut cairo_surface_t,
) -> cairo_status_t {
    let mut r: cairo_rectangle_t = cairo_rectangle_t {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };
    let mut extents: *mut cairo_rectangle_t = 0 as *mut cairo_rectangle_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*(*script).backend).type_0 as libc::c_int
        != CAIRO_DEVICE_TYPE_SCRIPT as libc::c_int
    {
        return _cairo_error(CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
    }
    if (*script).status as u64 != 0 {
        return _cairo_error((*script).status);
    }
    if (*recording_surface).status as u64 != 0 {
        return (*recording_surface).status;
    }
    if _cairo_surface_is_recording(recording_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    extents = 0 as *mut cairo_rectangle_t;
    if _cairo_recording_surface_get_bounds(recording_surface, &mut r) != 0 {
        extents = &mut r;
    }
    surface = &mut (*(_cairo_script_surface_create_internal
        as unsafe extern "C" fn(
            *mut cairo_script_context_t,
            cairo_content_t,
            *mut cairo_rectangle_t,
            *mut cairo_surface_t,
        ) -> *mut cairo_script_surface_t)(
        script as *mut cairo_script_context_t,
        (*recording_surface).content,
        extents,
        0 as *mut cairo_surface_t,
    ))
        .base;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    status = _cairo_recording_surface_replay(recording_surface, surface);
    cairo_surface_destroy(surface);
    return status;
}
