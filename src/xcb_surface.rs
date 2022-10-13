use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    pub type _cairo_xcb_shm_mem_pool;
    fn _cairo_boxes_init(boxes: *mut cairo_boxes_t);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
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
    fn pixman_image_set_destroy_function(
        image: *mut pixman_image_t,
        function: pixman_image_destroy_func_t,
        data: *mut libc::c_void,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_boxes_clear(boxes: *mut cairo_boxes_t);
    fn _cairo_boxes_fini(boxes: *mut cairo_boxes_t);
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_user_data_array_get_data(
        array: *mut cairo_user_data_array_t,
        key: *const cairo_user_data_key_t,
    ) -> *mut libc::c_void;
    fn _cairo_user_data_array_set_data(
        array: *mut cairo_user_data_array_t,
        key: *const cairo_user_data_key_t,
        user_data: *mut libc::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
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
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_map_to_image(
        surface: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_surface_unmap_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_surface_attach_snapshot(
        surface: *mut cairo_surface_t,
        snapshot: *mut cairo_surface_t,
        detach_func: cairo_surface_func_t,
    );
    fn _cairo_surface_has_snapshot(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_begin_modification(
        surface: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_format_from_content(content: cairo_content_t) -> cairo_format_t;
    fn _cairo_content_from_pixman_format(
        pixman_format: pixman_format_code_t,
    ) -> cairo_content_t;
    fn _cairo_format_to_pixman_format_code(
        format: cairo_format_t,
    ) -> pixman_format_code_t;
    fn _pixman_format_from_masks(
        masks: *mut cairo_format_masks_t,
        format_ret: *mut pixman_format_code_t,
    ) -> cairo_bool_t;
    fn _cairo_image_surface_create_with_pixman_format(
        data: *mut libc::c_uchar,
        pixman_format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        stride: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_bentley_ottmann_tessellate_boxes(
        in_0: *const cairo_boxes_t,
        fill_rule: cairo_fill_rule_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);
    fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;
    fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    fn xcb_screen_allowed_depths_iterator(
        R: *const xcb_screen_t,
    ) -> xcb_depth_iterator_t;
    fn xcb_screen_next(i: *mut xcb_screen_iterator_t);
    fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
    fn xcb_free_pixmap(
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
    fn xcb_get_image_data(R: *const xcb_get_image_reply_t) -> *mut uint8_t;
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> libc::c_int;
    fn _cairo_xcb_connection_create_pixmap(
        connection: *mut cairo_xcb_connection_t,
        depth: uint8_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_pixmap_t;
    fn _cairo_xcb_render_compositor_glyphs(
        compositor: *const cairo_compositor_t,
        extents: *mut cairo_composite_rectangles_t,
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        overlap: cairo_bool_t,
    ) -> cairo_int_status_t;
    fn _cairo_xcb_render_compositor_fill(
        compositor: *const cairo_compositor_t,
        extents: *mut cairo_composite_rectangles_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
    ) -> cairo_int_status_t;
    fn _cairo_xcb_render_compositor_stroke(
        compositor: *const cairo_compositor_t,
        extents: *mut cairo_composite_rectangles_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
    ) -> cairo_int_status_t;
    fn _cairo_xcb_render_compositor_mask(
        compositor: *const cairo_compositor_t,
        extents: *mut cairo_composite_rectangles_t,
    ) -> cairo_int_status_t;
    fn _cairo_xcb_render_compositor_paint(
        compositor: *const cairo_compositor_t,
        extents: *mut cairo_composite_rectangles_t,
    ) -> cairo_int_status_t;
    fn _cairo_xcb_screen_put_gc(
        screen: *mut cairo_xcb_screen_t,
        depth: libc::c_int,
        gc: xcb_gcontext_t,
    );
    fn _cairo_xcb_screen_get_gc(
        screen: *mut cairo_xcb_screen_t,
        drawable: xcb_drawable_t,
        depth: libc::c_int,
    ) -> xcb_gcontext_t;
    fn _cairo_xcb_shm_info_destroy(shm_info: *mut cairo_xcb_shm_info_t);
    fn _cairo_xcb_connection_allocate_shm_info(
        display: *mut cairo_xcb_connection_t,
        size: size_t,
        might_reuse: cairo_bool_t,
        shm_info_out: *mut *mut cairo_xcb_shm_info_t,
    ) -> cairo_int_status_t;
    fn _cairo_pattern_init_solid(
        pattern: *mut cairo_solid_pattern_t,
        color: *const cairo_color_t,
    );
    fn _cairo_xcb_surface_clear(dst: *mut cairo_xcb_surface_t) -> cairo_status_t;
    fn _cairo_xcb_screen_get_font_options(
        screen: *mut cairo_xcb_screen_t,
    ) -> *mut cairo_font_options_t;
    fn _cairo_xcb_connection_get_xrender_format_for_visual(
        connection: *mut cairo_xcb_connection_t,
        visual: xcb_visualid_t,
    ) -> xcb_render_pictformat_t;
    fn _cairo_xcb_screen_get(
        connection: *mut xcb_connection_t,
        screen: *mut xcb_screen_t,
    ) -> *mut cairo_xcb_screen_t;
    fn _cairo_xcb_shm_image_create(
        connection: *mut cairo_xcb_connection_t,
        pixman_format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        image_out: *mut *mut cairo_image_surface_t,
        shm_info_out: *mut *mut cairo_xcb_shm_info_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_connection_put_image(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        length: uint32_t,
        data: *mut libc::c_void,
    );
    fn _cairo_xcb_connection_shm_put_image(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        total_width: uint16_t,
        total_height: uint16_t,
        src_x: int16_t,
        src_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        shm: uint32_t,
        offset: uint32_t,
    );
    fn _cairo_xcb_connection_put_subimage(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        cpp: uint16_t,
        stride: libc::c_int,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        data: *mut libc::c_void,
    );
    fn _cairo_xcb_connection_render_free_picture(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
    );
    fn _cairo_xcb_connection_get_image(
        connection: *mut cairo_xcb_connection_t,
        src: xcb_drawable_t,
        src_x: int16_t,
        src_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> *mut xcb_get_image_reply_t;
    fn _cairo_xcb_connection_shm_get_image(
        connection: *mut cairo_xcb_connection_t,
        src: xcb_drawable_t,
        src_x: int16_t,
        src_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        shmseg: uint32_t,
        offset: uint32_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_connection_copy_area(
        connection: *mut cairo_xcb_connection_t,
        src: xcb_drawable_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    );
    fn _cairo_composite_rectangles_add_to_damage(
        composite: *mut cairo_composite_rectangles_t,
        damage: *mut cairo_boxes_t,
    ) -> cairo_int_status_t;
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    static _cairo_image_surface_backend: cairo_surface_backend_t;
    static __cairo_no_compositor: cairo_compositor_t;
    fn _cairo_compositor_paint(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_mask(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_stroke(
        compositor: *const cairo_compositor_t,
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
    ) -> cairo_int_status_t;
    fn _cairo_compositor_fill(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_glyphs(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        scaled_font: *mut cairo_scaled_font_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_compositor {
    pub delegate: *const cairo_compositor_t,
    pub paint: Option::<
        unsafe extern "C" fn(
            *const cairo_compositor_t,
            *mut cairo_composite_rectangles_t,
        ) -> cairo_int_status_t,
    >,
    pub mask: Option::<
        unsafe extern "C" fn(
            *const cairo_compositor_t,
            *mut cairo_composite_rectangles_t,
        ) -> cairo_int_status_t,
    >,
    pub stroke: Option::<
        unsafe extern "C" fn(
            *const cairo_compositor_t,
            *mut cairo_composite_rectangles_t,
            *const cairo_path_fixed_t,
            *const cairo_stroke_style_t,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            libc::c_double,
            cairo_antialias_t,
        ) -> cairo_int_status_t,
    >,
    pub fill: Option::<
        unsafe extern "C" fn(
            *const cairo_compositor_t,
            *mut cairo_composite_rectangles_t,
            *const cairo_path_fixed_t,
            cairo_fill_rule_t,
            libc::c_double,
            cairo_antialias_t,
        ) -> cairo_int_status_t,
    >,
    pub glyphs: Option::<
        unsafe extern "C" fn(
            *const cairo_compositor_t,
            *mut cairo_composite_rectangles_t,
            *mut cairo_scaled_font_t,
            *mut cairo_glyph_t,
            libc::c_int,
            cairo_bool_t,
        ) -> cairo_int_status_t,
    >,
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub type uint8_t = __uint8_t;
pub type pixman_image_destroy_func_t = Option::<
    unsafe extern "C" fn(*mut pixman_image_t, *mut libc::c_void) -> (),
>;
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
pub struct _cairo_cache {
    pub hash_table: *mut cairo_hash_table_t,
    pub predicate: cairo_cache_predicate_func_t,
    pub entry_destroy: cairo_destroy_func_t,
    pub max_size: libc::c_ulong,
    pub size: libc::c_ulong,
    pub freeze_count: libc::c_int,
}
pub type cairo_cache_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_cache_t = _cairo_cache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_format_masks {
    pub bpp: libc::c_int,
    pub alpha_mask: libc::c_ulong,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
}
pub type cairo_format_masks_t = _cairo_format_masks;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: uint8_t,
    pub bits_per_rgb_value: uint8_t,
    pub colormap_entries: uint16_t,
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad0: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub visuals_len: uint16_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: libc::c_int,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub pad0: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}
pub type xcb_render_poly_mode_t = libc::c_uint;
pub const XCB_RENDER_POLY_MODE_IMPRECISE: xcb_render_poly_mode_t = 1;
pub const XCB_RENDER_POLY_MODE_PRECISE: xcb_render_poly_mode_t = 0;
pub type xcb_render_sub_pixel_t = libc::c_uint;
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 5;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 4;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 3;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 2;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 1;
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0;
pub type xcb_render_picture_t = uint32_t;
pub type xcb_render_pictformat_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_directformat_t {
    pub red_shift: uint16_t,
    pub red_mask: uint16_t,
    pub green_shift: uint16_t,
    pub green_mask: uint16_t,
    pub blue_shift: uint16_t,
    pub blue_mask: uint16_t,
    pub alpha_shift: uint16_t,
    pub alpha_mask: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pictforminfo_t {
    pub id: xcb_render_pictformat_t,
    pub type_0: uint8_t,
    pub depth: uint8_t,
    pub pad0: [uint8_t; 2],
    pub direct: xcb_render_directformat_t,
    pub colormap: xcb_colormap_t,
}
pub type cairo_xcb_screen_t = _cairo_xcb_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_screen {
    pub connection: *mut cairo_xcb_connection_t,
    pub xcb_screen: *mut xcb_screen_t,
    pub subpixel_order: xcb_render_sub_pixel_t,
    pub gc: [xcb_gcontext_t; 4],
    pub gc_depths: [uint8_t; 4],
    pub stock_colors: [*mut cairo_surface_t; 3],
    pub solid_cache: [C2RustUnnamed; 16],
    pub solid_cache_size: libc::c_int,
    pub linear_pattern_cache: cairo_cache_t,
    pub radial_pattern_cache: cairo_cache_t,
    pub pattern_cache_entry_freelist: cairo_freelist_t,
    pub link: cairo_list_t,
    pub surfaces: cairo_list_t,
    pub pictures: cairo_list_t,
    pub has_font_options: cairo_bool_t,
    pub font_options: cairo_font_options_t,
}
pub type cairo_freelist_t = _cairo_freelist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub nodesize: libc::c_uint,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub picture: *mut cairo_surface_t,
    pub color: cairo_color_t,
}
pub type cairo_xcb_connection_t = _cairo_xcb_connection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_connection {
    pub device: cairo_device_t,
    pub xcb_connection: *mut xcb_connection_t,
    pub standard_formats: [xcb_render_pictformat_t; 5],
    pub xrender_formats: *mut cairo_hash_table_t,
    pub visual_to_xrender_format: *mut cairo_hash_table_t,
    pub maximum_request_length: libc::c_uint,
    pub flags: libc::c_uint,
    pub original_flags: libc::c_uint,
    pub force_precision: libc::c_int,
    pub root: *const xcb_setup_t,
    pub render: *const xcb_query_extension_reply_t,
    pub shm: *const xcb_query_extension_reply_t,
    pub subpixel_orders: *mut xcb_render_sub_pixel_t,
    pub shm_mutex: cairo_mutex_t,
    pub shm_pools: cairo_list_t,
    pub shm_pending: cairo_list_t,
    pub shm_info_freelist: cairo_freepool_t,
    pub screens_mutex: cairo_mutex_t,
    pub screens: cairo_list_t,
    pub fonts: cairo_list_t,
    pub link: cairo_list_t,
}
pub type cairo_freepool_t = _cairo_freepool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freepool {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub pools: *mut cairo_freelist_pool_t,
    pub freepools: *mut cairo_freelist_pool_t,
    pub nodesize: libc::c_uint,
    pub embedded_pool: cairo_freelist_pool_t,
    pub embedded_data: [uint8_t; 1000],
}
pub type cairo_freelist_pool_t = _cairo_freelist_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_pool {
    pub next: *mut cairo_freelist_pool_t,
    pub size: libc::c_uint,
    pub rem: libc::c_uint,
    pub data: *mut uint8_t,
}
pub type cairo_xcb_surface_t = _cairo_xcb_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_surface {
    pub base: cairo_surface_t,
    pub fallback: *mut cairo_image_surface_t,
    pub fallback_damage: cairo_boxes_t,
    pub connection: *mut cairo_xcb_connection_t,
    pub screen: *mut cairo_xcb_screen_t,
    pub drawable: xcb_drawable_t,
    pub owns_pixmap: cairo_bool_t,
    pub deferred_clear: cairo_bool_t,
    pub deferred_clear_color: cairo_color_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub picture: xcb_render_picture_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pixman_format: pixman_format_code_t,
    pub precision: uint32_t,
    pub link: cairo_list_t,
}
pub type cairo_xcb_shm_info_t = _cairo_xcb_shm_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_shm_info {
    pub connection: *mut cairo_xcb_connection_t,
    pub shm: uint32_t,
    pub offset: uint32_t,
    pub size: size_t,
    pub mem: *mut libc::c_void,
    pub pool: *mut cairo_xcb_shm_mem_pool_t,
    pub sync: xcb_get_input_focus_cookie_t,
    pub pending: cairo_list_t,
}
pub type cairo_xcb_shm_mem_pool_t = _cairo_xcb_shm_mem_pool;
pub const CAIRO_XCB_HAS_SHM: C2RustUnnamed_0 = 2147483648;
pub const CAIRO_XCB_HAS_RENDER: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const CAIRO_XCB_SHM_MASK: C2RustUnnamed_0 = 2147483648;
pub const CAIRO_XCB_RENDER_MASK: C2RustUnnamed_0 = 4095;
pub const CAIRO_XCB_RENDER_HAS_FILTER_BEST: C2RustUnnamed_0 = 2048;
pub const CAIRO_XCB_RENDER_HAS_FILTER_GOOD: C2RustUnnamed_0 = 1024;
pub const CAIRO_XCB_RENDER_HAS_GRADIENTS: C2RustUnnamed_0 = 512;
pub const CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT: C2RustUnnamed_0 = 256;
pub const CAIRO_XCB_RENDER_HAS_PDF_OPERATORS: C2RustUnnamed_0 = 128;
pub const CAIRO_XCB_RENDER_HAS_FILTERS: C2RustUnnamed_0 = 64;
pub const CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM: C2RustUnnamed_0 = 32;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS: C2RustUnnamed_0 = 16;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS: C2RustUnnamed_0 = 8;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE: C2RustUnnamed_0 = 4;
pub const CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES: C2RustUnnamed_0 = 2;
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_reference(
    mut connection: *mut cairo_xcb_connection_t,
) -> *mut cairo_xcb_connection_t {
    return cairo_device_reference(&mut (*connection).device)
        as *mut cairo_xcb_connection_t;
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_release(
    mut connection: *mut cairo_xcb_connection_t,
) {
    cairo_device_release(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_acquire(
    mut connection: *mut cairo_xcb_connection_t,
) -> cairo_status_t {
    return cairo_device_acquire(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_destroy(
    mut connection: *mut cairo_xcb_connection_t,
) {
    cairo_device_destroy(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_xcb(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return (!((*surface).backend).is_null()
        && (*(*surface).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_XCB as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_image_surface_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_image_surface_t {
    return _cairo_surface_create_in_error(status) as *mut cairo_image_surface_t;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh0 = (*entry).next;
    *fresh0 = entry;
    let ref mut fresh1 = (*entry).prev;
    *fresh1 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh2 = (*next).prev;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).next;
    *fresh3 = next;
    let ref mut fresh4 = (*entry).prev;
    *fresh4 = prev;
    let ref mut fresh5 = (*prev).next;
    *fresh5 = entry;
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
    let ref mut fresh6 = (*next).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = next;
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_surface_create_similar(
    mut abstract_other: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut other: *mut cairo_xcb_surface_t = abstract_other as *mut cairo_xcb_surface_t;
    let mut surface: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut connection: *mut cairo_xcb_connection_t = 0 as *mut cairo_xcb_connection_t;
    let mut pixmap: xcb_pixmap_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int
        || width <= 0 as libc::c_int || height <= 0 as libc::c_int
    {
        return cairo_image_surface_create(
            _cairo_format_from_content(content),
            width,
            height,
        );
    }
    if (*(*other).connection).flags & CAIRO_XCB_HAS_RENDER as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return _cairo_xcb_surface_create_similar_image(
            other as *mut libc::c_void,
            _cairo_format_from_content(content),
            width,
            height,
        );
    }
    connection = (*other).connection;
    status = _cairo_xcb_connection_acquire(connection);
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    if content as libc::c_uint == (*other).base.content as libc::c_uint {
        pixmap = _cairo_xcb_connection_create_pixmap(
            connection,
            (*other).depth as uint8_t,
            (*other).drawable,
            width as uint16_t,
            height as uint16_t,
        );
        surface = _cairo_xcb_surface_create_internal(
            (*other).screen,
            pixmap,
            1 as libc::c_int,
            (*other).pixman_format,
            (*other).xrender_format,
            width,
            height,
        ) as *mut cairo_xcb_surface_t;
    } else {
        let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
        let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
        let mut current_block_19: u64;
        match content as libc::c_uint {
            8192 => {
                pixman_format = PIXMAN_a8;
                format = CAIRO_FORMAT_A8;
                current_block_19 = 7172762164747879670;
            }
            4096 => {
                pixman_format = PIXMAN_x8r8g8b8;
                format = CAIRO_FORMAT_RGB24;
                current_block_19 = 7172762164747879670;
            }
            12288 => {
                current_block_19 = 1880261343578478836;
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-xcb-surface.c\0" as *const u8
                            as *const libc::c_char,
                        137 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 86],
                            &[libc::c_char; 86],
                        >(
                            b"cairo_surface_t *_cairo_xcb_surface_create_similar(void *, cairo_content_t, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                current_block_19 = 1880261343578478836;
            }
        }
        match current_block_19 {
            1880261343578478836 => {
                pixman_format = PIXMAN_a8r8g8b8;
                format = CAIRO_FORMAT_ARGB32;
            }
            _ => {}
        }
        pixmap = _cairo_xcb_connection_create_pixmap(
            connection,
            ((pixman_format as libc::c_uint >> 12 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_add(
                    (pixman_format as libc::c_uint >> 8 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                )
                .wrapping_add(
                    (pixman_format as libc::c_uint >> 4 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                )
                .wrapping_add(
                    (pixman_format as libc::c_uint >> 0 as libc::c_int
                        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << (pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                ) as uint8_t,
            (*other).drawable,
            width as uint16_t,
            height as uint16_t,
        );
        surface = _cairo_xcb_surface_create_internal(
            (*other).screen,
            pixmap,
            1 as libc::c_int,
            pixman_format,
            (*connection).standard_formats[format as usize],
            width,
            height,
        ) as *mut cairo_xcb_surface_t;
    }
    if (*surface).base.status as u64 != 0 {
        xcb_free_pixmap((*connection).xcb_connection, pixmap);
    }
    _cairo_xcb_connection_release(connection);
    return &mut (*surface).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_surface_create_similar_image(
    mut abstract_other: *mut libc::c_void,
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut other: *mut cairo_xcb_surface_t = abstract_other as *mut cairo_xcb_surface_t;
    let mut connection: *mut cairo_xcb_connection_t = (*other).connection;
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int
        || width <= 0 as libc::c_int || height <= 0 as libc::c_int
    {
        return 0 as *mut cairo_surface_t;
    }
    pixman_format = _cairo_format_to_pixman_format_code(format);
    status = _cairo_xcb_shm_image_create(
        connection,
        pixman_format,
        width,
        height,
        &mut image,
        &mut shm_info,
    );
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    if ((*image).base).is_clear() == 0 {
        memset(
            (*image).data as *mut libc::c_void,
            0 as libc::c_int,
            ((*image).stride * (*image).height as libc::c_long) as libc::c_ulong,
        );
        let ref mut fresh8 = (*image).base;
        (*fresh8).set_is_clear(1 as libc::c_int as libc::c_uint);
    }
    return &mut (*image).base;
}
unsafe extern "C" fn _cairo_xcb_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !((*surface).fallback).is_null() {
        cairo_surface_finish(&mut (*(*surface).fallback).base);
        cairo_surface_destroy(&mut (*(*surface).fallback).base);
    }
    _cairo_boxes_fini(&mut (*surface).fallback_damage);
    cairo_list_del(&mut (*surface).link);
    status = _cairo_xcb_connection_acquire((*surface).connection);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        if (*surface).picture as libc::c_long != 0 as libc::c_long {
            _cairo_xcb_connection_render_free_picture(
                (*surface).connection,
                (*surface).picture,
            );
        }
        if (*surface).owns_pixmap != 0 {
            xcb_free_pixmap(
                (*(*surface).connection).xcb_connection,
                (*surface).drawable,
            );
        }
        _cairo_xcb_connection_release((*surface).connection);
    }
    _cairo_xcb_connection_destroy((*surface).connection);
    return status;
}
unsafe extern "C" fn _destroy_image(
    mut image: *mut pixman_image_t,
    mut data: *mut libc::c_void,
) {
    free(data);
}
unsafe extern "C" fn _cairo_xcb_surface_create_shm_image(
    mut connection: *mut cairo_xcb_connection_t,
    mut pixman_format: pixman_format_code_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut might_reuse: cairo_bool_t,
    mut shm_info_out: *mut *mut cairo_xcb_shm_info_t,
) -> *mut cairo_surface_t {
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut stride: size_t = 0;
    *shm_info_out = 0 as *mut cairo_xcb_shm_info_t;
    stride = (((pixman_format as libc::c_uint >> 24 as libc::c_int
        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (pixman_format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint))
        .wrapping_mul(width as libc::c_uint)
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg();
    status = _cairo_xcb_connection_allocate_shm_info(
        connection,
        stride.wrapping_mul(height as libc::c_ulong),
        might_reuse,
        &mut shm_info,
    );
    if status as u64 != 0 {
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return 0 as *mut cairo_surface_t;
        }
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    image = _cairo_image_surface_create_with_pixman_format(
        (*shm_info).mem as *mut libc::c_uchar,
        pixman_format,
        width,
        height,
        stride as libc::c_int,
    );
    if (*image).status as u64 != 0 {
        _cairo_xcb_shm_info_destroy(shm_info);
        return image;
    }
    status = _cairo_user_data_array_set_data(
        &mut (*image).user_data,
        connection as *const cairo_user_data_key_t,
        shm_info as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cairo_xcb_shm_info_t) -> ()>,
            cairo_destroy_func_t,
        >(
            Some(
                _cairo_xcb_shm_info_destroy
                    as unsafe extern "C" fn(*mut cairo_xcb_shm_info_t) -> (),
            ),
        ),
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        cairo_surface_destroy(image);
        _cairo_xcb_shm_info_destroy(shm_info);
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    *shm_info_out = shm_info;
    return image;
}
unsafe extern "C" fn _get_shm_image(
    mut surface: *mut cairo_xcb_surface_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*(*surface).connection).flags & CAIRO_XCB_HAS_SHM as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return 0 as *mut cairo_surface_t;
    }
    image = _cairo_xcb_surface_create_shm_image(
        (*surface).connection,
        (*surface).pixman_format,
        width,
        height,
        1 as libc::c_int,
        &mut shm_info,
    );
    if !(image.is_null() || (*image).status as libc::c_uint != 0) {
        status = _cairo_xcb_connection_shm_get_image(
            (*surface).connection,
            (*surface).drawable,
            x as int16_t,
            y as int16_t,
            width as uint16_t,
            height as uint16_t,
            (*shm_info).shm,
            (*shm_info).offset,
        );
        if status as u64 != 0 {
            cairo_surface_destroy(image);
            image = _cairo_surface_create_in_error(status);
        }
    }
    return image;
}
unsafe extern "C" fn _get_image(
    mut surface: *mut cairo_xcb_surface_t,
    mut use_shm: cairo_bool_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut connection: *mut cairo_xcb_connection_t = 0 as *mut cairo_xcb_connection_t;
    let mut reply: *mut xcb_get_image_reply_t = 0 as *mut xcb_get_image_reply_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if ((*surface).fallback).is_null() {} else {
        __assert_fail(
            b"surface->fallback == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            340 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"cairo_surface_t *_get_image(cairo_xcb_surface_t *, cairo_bool_t, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if x >= 0 as libc::c_int {} else {
        __assert_fail(
            b"x >= 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"cairo_surface_t *_get_image(cairo_xcb_surface_t *, cairo_bool_t, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if y >= 0 as libc::c_int {} else {
        __assert_fail(
            b"y >= 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            342 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"cairo_surface_t *_get_image(cairo_xcb_surface_t *, cairo_bool_t, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if x + width <= (*surface).width {} else {
        __assert_fail(
            b"x + width <= surface->width\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"cairo_surface_t *_get_image(cairo_xcb_surface_t *, cairo_bool_t, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if y + height <= (*surface).height {} else {
        __assert_fail(
            b"y + height <= surface->height\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"cairo_surface_t *_get_image(cairo_xcb_surface_t *, cairo_bool_t, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).deferred_clear != 0 {
        image = _cairo_image_surface_create_with_pixman_format(
            0 as *mut libc::c_uchar,
            (*surface).pixman_format,
            width,
            height,
            0 as libc::c_int,
        );
        if (*surface).deferred_clear_color.alpha_short as libc::c_int
            > 0xff as libc::c_int
        {
            let mut solid: cairo_solid_pattern_t = cairo_solid_pattern_t {
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
                color: cairo_color_t {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                    red_short: 0,
                    green_short: 0,
                    blue_short: 0,
                    alpha_short: 0,
                },
            };
            _cairo_pattern_init_solid(&mut solid, &mut (*surface).deferred_clear_color);
            status = _cairo_surface_paint(
                image,
                CAIRO_OPERATOR_SOURCE,
                &mut solid.base,
                0 as *const cairo_clip_t,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                cairo_surface_destroy(image);
                image = _cairo_surface_create_in_error(status as cairo_status_t);
            }
        }
        return image;
    }
    connection = (*surface).connection;
    status = _cairo_xcb_connection_acquire(connection) as cairo_int_status_t;
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    if use_shm != 0 {
        image = _get_shm_image(surface, x, y, width, height);
        if !image.is_null() {
            if (*image).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                _cairo_xcb_connection_release(connection);
                return image;
            }
            cairo_surface_destroy(image);
        }
    }
    reply = _cairo_xcb_connection_get_image(
        connection,
        (*surface).drawable,
        x as int16_t,
        y as int16_t,
        width as uint16_t,
        height as uint16_t,
    );
    if reply.is_null() && (*surface).owns_pixmap == 0 {
        let mut pixmap: xcb_pixmap_t = 0;
        let mut gc: xcb_gcontext_t = 0;
        gc = _cairo_xcb_screen_get_gc(
            (*surface).screen,
            (*surface).drawable,
            (*surface).depth,
        );
        pixmap = _cairo_xcb_connection_create_pixmap(
            connection,
            (*surface).depth as uint8_t,
            (*surface).drawable,
            width as uint16_t,
            height as uint16_t,
        );
        _cairo_xcb_connection_copy_area(
            connection,
            (*surface).drawable,
            pixmap,
            gc,
            x as int16_t,
            y as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            width as uint16_t,
            height as uint16_t,
        );
        _cairo_xcb_screen_put_gc((*surface).screen, (*surface).depth, gc);
        reply = _cairo_xcb_connection_get_image(
            connection,
            pixmap,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            width as uint16_t,
            height as uint16_t,
        );
        xcb_free_pixmap((*connection).xcb_connection, pixmap);
    }
    if reply.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    } else {
        if (*reply).depth as libc::c_int == (*surface).depth {} else {
            __assert_fail(
                b"reply->depth == surface->depth\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
                436 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"cairo_surface_t *_get_image(cairo_xcb_surface_t *, cairo_bool_t, int, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        image = _cairo_image_surface_create_with_pixman_format(
            xcb_get_image_data(reply),
            (*surface).pixman_format,
            width,
            height,
            (((((*surface).pixman_format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << ((*surface).pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_mul(width as libc::c_uint)
                .wrapping_add(7 as libc::c_int as libc::c_uint)
                .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
                as libc::c_int,
        );
        status = (*image).status as cairo_int_status_t;
        if status as u64 != 0 {
            free(reply as *mut libc::c_void);
        } else {
            pixman_image_set_destroy_function(
                (*(image as *mut cairo_image_surface_t)).pixman_image,
                Some(
                    _destroy_image
                        as unsafe extern "C" fn(
                            *mut pixman_image_t,
                            *mut libc::c_void,
                        ) -> (),
                ),
                reply as *mut libc::c_void,
            );
            _cairo_xcb_connection_release(connection);
            return image;
        }
    }
    _cairo_xcb_connection_release(connection);
    return _cairo_surface_create_in_error(status as cairo_status_t);
}
unsafe extern "C" fn _cairo_xcb_surface_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    if !extents.is_null() {
        let ref mut fresh9 = (*extents).y;
        *fresh9 = 0 as libc::c_int;
        (*extents).x = *fresh9;
        (*extents).width = (*surface).width;
        (*extents).height = (*surface).height;
    }
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_xcb_surface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if !((*surface).fallback).is_null() {
        image = cairo_surface_reference(&mut (*(*surface).fallback).base);
    } else {
        image = _cairo_surface_has_snapshot(
            &mut (*surface).base,
            &_cairo_image_surface_backend,
        );
        if !image.is_null() {
            image = cairo_surface_reference(image);
        } else {
            image = _get_image(
                surface,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*surface).width,
                (*surface).height,
            );
            if (*image).status as u64 != 0 {
                return (*image).status;
            }
            _cairo_surface_attach_snapshot(&mut (*surface).base, image, None);
        }
    }
    *image_out = image as *mut cairo_image_surface_t;
    *image_extra = 0 as *mut libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xcb_surface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    cairo_surface_destroy(&mut (*image).base);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    let ref mut fresh10 = (*extents).y;
    *fresh10 = 0 as libc::c_int;
    (*extents).x = *fresh10;
    (*extents).width = (*surface).width;
    (*extents).height = (*surface).height;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_xcb_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    *options = *_cairo_xcb_screen_get_font_options((*surface).screen);
}
unsafe extern "C" fn _put_shm_image(
    mut surface: *mut cairo_xcb_surface_t,
    mut gc: xcb_gcontext_t,
    mut image: *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    shm_info = _cairo_user_data_array_get_data(
        &mut (*image).base.user_data,
        (*surface).connection as *const cairo_user_data_key_t,
    ) as *mut cairo_xcb_shm_info_t;
    if shm_info.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    _cairo_xcb_connection_shm_put_image(
        (*surface).connection,
        (*surface).drawable,
        gc,
        (*surface).width as uint16_t,
        (*surface).height as uint16_t,
        0 as libc::c_int as int16_t,
        0 as libc::c_int as int16_t,
        (*image).width as uint16_t,
        (*image).height as uint16_t,
        (*image).base.device_transform_inverse.x0 as int16_t,
        (*image).base.device_transform_inverse.y0 as int16_t,
        (*image).depth as uint8_t,
        (*shm_info).shm,
        (*shm_info).offset,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _put_image(
    mut surface: *mut cairo_xcb_surface_t,
    mut image: *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_xcb_connection_acquire((*surface).connection) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if (*image).pixman_format as libc::c_uint == (*surface).pixman_format as libc::c_uint
    {
        let mut gc: xcb_gcontext_t = 0;
        if (*image).depth == (*surface).depth {} else {
            __assert_fail(
                b"image->depth == surface->depth\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
                584 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"cairo_status_t _put_image(cairo_xcb_surface_t *, cairo_image_surface_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*image).stride
            == (((((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << ((*image).pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint))
                .wrapping_mul((*image).width as libc::c_uint)
                .wrapping_add(7 as libc::c_int as libc::c_uint)
                .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
                as libc::c_int as libc::c_long
        {} else {
            __assert_fail(
                b"image->stride == (int) CAIRO_STRIDE_FOR_WIDTH_BPP (image->width, PIXMAN_FORMAT_BPP (image->pixman_format))\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
                585 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"cairo_status_t _put_image(cairo_xcb_surface_t *, cairo_image_surface_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        gc = _cairo_xcb_screen_get_gc(
            (*surface).screen,
            (*surface).drawable,
            (*surface).depth,
        );
        status = _put_shm_image(surface, gc, image) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            _cairo_xcb_connection_put_image(
                (*surface).connection,
                (*surface).drawable,
                gc,
                (*image).width as uint16_t,
                (*image).height as uint16_t,
                (*image).base.device_transform_inverse.x0 as int16_t,
                (*image).base.device_transform_inverse.y0 as int16_t,
                (*image).depth as uint8_t,
                (*image).stride as uint32_t,
                (*image).data as *mut libc::c_void,
            );
            status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        _cairo_xcb_screen_put_gc((*surface).screen, (*surface).depth, gc);
    } else if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            606 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"cairo_status_t _put_image(cairo_xcb_surface_t *, cairo_image_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_xcb_connection_release((*surface).connection);
    return status as cairo_status_t;
}
unsafe extern "C" fn _put_shm_image_boxes(
    mut surface: *mut cairo_xcb_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut gc: xcb_gcontext_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    shm_info = _cairo_user_data_array_get_data(
        &mut (*image).base.user_data,
        (*surface).connection as *const cairo_user_data_key_t,
    ) as *mut cairo_xcb_shm_info_t;
    if !shm_info.is_null() {
        let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut b: *mut cairo_box_t = &mut *((*chunk).base).offset(i as isize)
                    as *mut cairo_box_t;
                let mut x: libc::c_int = _cairo_fixed_integer_part((*b).p1.x);
                let mut y: libc::c_int = _cairo_fixed_integer_part((*b).p1.y);
                let mut width: libc::c_int = _cairo_fixed_integer_part(
                    (*b).p2.x - (*b).p1.x,
                );
                let mut height: libc::c_int = _cairo_fixed_integer_part(
                    (*b).p2.y - (*b).p1.y,
                );
                _cairo_xcb_connection_shm_put_image(
                    (*surface).connection,
                    (*surface).drawable,
                    gc,
                    (*surface).width as uint16_t,
                    (*surface).height as uint16_t,
                    x as int16_t,
                    y as int16_t,
                    width as uint16_t,
                    height as uint16_t,
                    x as int16_t,
                    y as int16_t,
                    (*image).depth as uint8_t,
                    (*shm_info).shm,
                    (*shm_info).offset,
                );
                i += 1;
            }
            chunk = (*chunk).next;
        }
        return CAIRO_INT_STATUS_SUCCESS;
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
unsafe extern "C" fn _put_image_boxes(
    mut surface: *mut cairo_xcb_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut gc: xcb_gcontext_t = 0;
    if (*boxes).num_boxes == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_xcb_connection_acquire((*surface).connection) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if (*image).pixman_format as libc::c_uint == (*surface).pixman_format as libc::c_uint
    {} else {
        __assert_fail(
            b"image->pixman_format == surface->pixman_format\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            673 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_status_t _put_image_boxes(cairo_xcb_surface_t *, cairo_image_surface_t *, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*image).depth == (*surface).depth {} else {
        __assert_fail(
            b"image->depth == surface->depth\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_status_t _put_image_boxes(cairo_xcb_surface_t *, cairo_image_surface_t *, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*image).stride
        == (((((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
            & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint)
            << ((*image).pixman_format as libc::c_uint >> 22 as libc::c_int
                & 3 as libc::c_int as libc::c_uint))
            .wrapping_mul((*image).width as libc::c_uint)
            .wrapping_add(7 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
            as libc::c_int as libc::c_long
    {} else {
        __assert_fail(
            b"image->stride == (int) CAIRO_STRIDE_FOR_WIDTH_BPP (image->width, PIXMAN_FORMAT_BPP (image->pixman_format))\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface.c\0" as *const u8 as *const libc::c_char,
            675 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_status_t _put_image_boxes(cairo_xcb_surface_t *, cairo_image_surface_t *, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
    }
    gc = _cairo_xcb_screen_get_gc(
        (*surface).screen,
        (*surface).drawable,
        (*surface).depth,
    );
    status = _put_shm_image_boxes(surface, image, gc, boxes);
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut b: *mut cairo_box_t = &mut *((*chunk).base).offset(i as isize)
                    as *mut cairo_box_t;
                let mut x: libc::c_int = _cairo_fixed_integer_part((*b).p1.x);
                let mut y: libc::c_int = _cairo_fixed_integer_part((*b).p1.y);
                let mut width: libc::c_int = _cairo_fixed_integer_part(
                    (*b).p2.x - (*b).p1.x,
                );
                let mut height: libc::c_int = _cairo_fixed_integer_part(
                    (*b).p2.y - (*b).p1.y,
                );
                _cairo_xcb_connection_put_subimage(
                    (*surface).connection,
                    (*surface).drawable,
                    gc,
                    x as int16_t,
                    y as int16_t,
                    width as uint16_t,
                    height as uint16_t,
                    (((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << ((*image).pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint))
                        .wrapping_div(8 as libc::c_int as libc::c_uint) as uint16_t,
                    (*image).stride as libc::c_int,
                    x as int16_t,
                    y as int16_t,
                    (*image).depth as uint8_t,
                    (*image).data as *mut libc::c_void,
                );
                i += 1;
            }
            chunk = (*chunk).next;
        }
        status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    _cairo_xcb_screen_put_gc((*surface).screen, (*surface).depth, gc);
    _cairo_xcb_connection_release((*surface).connection);
    return status as cairo_status_t;
}
unsafe extern "C" fn _cairo_xcb_surface_flush(
    mut abstract_surface: *mut libc::c_void,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if flags != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if ((*surface).fallback).is_null() {
        status = CAIRO_STATUS_SUCCESS;
        if ((*surface).base).finished() == 0 && (*surface).deferred_clear != 0 {
            status = _cairo_xcb_surface_clear(surface);
        }
        return status;
    }
    status = (*surface).base.status;
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (((*surface).base)._finishing() == 0 || (*surface).owns_pixmap == 0)
    {
        status = cairo_surface_status(&mut (*(*surface).fallback).base);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = _cairo_bentley_ottmann_tessellate_boxes(
                &mut (*surface).fallback_damage,
                CAIRO_FILL_RULE_WINDING,
                &mut (*surface).fallback_damage,
            );
        }
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = _put_image_boxes(
                surface,
                (*surface).fallback,
                &mut (*surface).fallback_damage,
            );
        }
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && ((*surface).base)._finishing() == 0
        {
            _cairo_surface_attach_snapshot(
                &mut (*surface).base,
                &mut (*(*surface).fallback).base,
                Some(
                    cairo_surface_finish
                        as unsafe extern "C" fn(*mut cairo_surface_t) -> (),
                ),
            );
        }
    }
    _cairo_boxes_clear(&mut (*surface).fallback_damage);
    cairo_surface_destroy(&mut (*(*surface).fallback).base);
    let ref mut fresh11 = (*surface).fallback;
    *fresh11 = 0 as *mut cairo_image_surface_t;
    return status;
}
unsafe extern "C" fn _cairo_xcb_surface_map_to_image(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !((*surface).fallback).is_null() {
        return _cairo_surface_map_to_image(&mut (*(*surface).fallback).base, extents);
    }
    image = _get_image(
        surface,
        1 as libc::c_int,
        (*extents).x,
        (*extents).y,
        (*extents).width,
        (*extents).height,
    );
    status = cairo_surface_status(image);
    if status as u64 != 0 {
        cairo_surface_destroy(image);
        return _cairo_image_surface_create_in_error(status);
    }
    if (*surface).deferred_clear != 0
        && !((*extents).width == (*surface).width
            && (*extents).height == (*surface).height)
    {
        status = _cairo_xcb_surface_clear(surface);
        if status as u64 != 0 {
            cairo_surface_destroy(image);
            return _cairo_image_surface_create_in_error(status);
        }
    }
    (*surface).deferred_clear = 0 as libc::c_int;
    cairo_surface_set_device_offset(
        image,
        -(*extents).x as libc::c_double,
        -(*extents).y as libc::c_double,
    );
    return image as *mut cairo_image_surface_t;
}
unsafe extern "C" fn _cairo_xcb_surface_unmap(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = abstract_surface
        as *mut cairo_xcb_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if !((*surface).fallback).is_null() {
        return _cairo_surface_unmap_image(&mut (*(*surface).fallback).base, image);
    }
    status = _put_image(abstract_surface as *mut cairo_xcb_surface_t, image)
        as cairo_int_status_t;
    cairo_surface_finish(&mut (*image).base);
    cairo_surface_destroy(&mut (*image).base);
    return status;
}
unsafe extern "C" fn _cairo_xcb_surface_fallback(
    mut surface: *mut cairo_xcb_surface_t,
    mut composite: *mut cairo_composite_rectangles_t,
) -> *mut cairo_surface_t {
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_composite_rectangles_add_to_damage(
        composite,
        &mut (*surface).fallback_damage,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    if !((*surface).fallback).is_null() {
        return &mut (*(*surface).fallback).base;
    }
    image = _get_image(
        surface,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        (*surface).width,
        (*surface).height,
    ) as *mut cairo_image_surface_t;
    if (*image).base.status as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return &mut (*image).base;
    }
    (*surface).deferred_clear = 0 as libc::c_int;
    let ref mut fresh12 = (*surface).fallback;
    *fresh12 = image;
    return &mut (*(*surface).fallback).base;
}
unsafe extern "C" fn _cairo_xcb_fallback_compositor_paint(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*extents).surface
        as *mut cairo_xcb_surface_t;
    let mut fallback: *mut cairo_surface_t = _cairo_xcb_surface_fallback(
        surface,
        extents,
    );
    return _cairo_surface_paint(
        fallback,
        (*extents).op,
        &mut (*extents).source_pattern.base,
        (*extents).clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_xcb_fallback_compositor_mask(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*extents).surface
        as *mut cairo_xcb_surface_t;
    let mut fallback: *mut cairo_surface_t = _cairo_xcb_surface_fallback(
        surface,
        extents,
    );
    return _cairo_surface_mask(
        fallback,
        (*extents).op,
        &mut (*extents).source_pattern.base,
        &mut (*extents).mask_pattern.base,
        (*extents).clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_xcb_fallback_compositor_stroke(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*extents).surface
        as *mut cairo_xcb_surface_t;
    let mut fallback: *mut cairo_surface_t = _cairo_xcb_surface_fallback(
        surface,
        extents,
    );
    return _cairo_surface_stroke(
        fallback,
        (*extents).op,
        &mut (*extents).source_pattern.base,
        path,
        style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        (*extents).clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_xcb_fallback_compositor_fill(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*extents).surface
        as *mut cairo_xcb_surface_t;
    let mut fallback: *mut cairo_surface_t = _cairo_xcb_surface_fallback(
        surface,
        extents,
    );
    return _cairo_surface_fill(
        fallback,
        (*extents).op,
        &mut (*extents).source_pattern.base,
        path,
        fill_rule,
        tolerance,
        antialias,
        (*extents).clip,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_xcb_fallback_compositor_glyphs(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut overlap: cairo_bool_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*extents).surface
        as *mut cairo_xcb_surface_t;
    let mut fallback: *mut cairo_surface_t = _cairo_xcb_surface_fallback(
        surface,
        extents,
    );
    return _cairo_surface_show_text_glyphs(
        fallback,
        (*extents).op,
        &mut (*extents).source_pattern.base,
        0 as *const libc::c_char,
        0 as libc::c_int,
        glyphs,
        num_glyphs,
        0 as *const cairo_text_cluster_t,
        0 as libc::c_int,
        0 as cairo_text_cluster_flags_t,
        scaled_font,
        (*extents).clip,
    ) as cairo_int_status_t;
}
static mut _cairo_xcb_fallback_compositor: cairo_compositor_t = unsafe {
    {
        let mut init = cairo_compositor {
            delegate: &__cairo_no_compositor as *const cairo_compositor_t,
            paint: Some(
                _cairo_xcb_fallback_compositor_paint
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_xcb_fallback_compositor_mask
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_xcb_fallback_compositor_stroke
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                        *const cairo_path_fixed_t,
                        *const cairo_stroke_style_t,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        libc::c_double,
                        cairo_antialias_t,
                    ) -> cairo_int_status_t,
            ),
            fill: Some(
                _cairo_xcb_fallback_compositor_fill
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                        *const cairo_path_fixed_t,
                        cairo_fill_rule_t,
                        libc::c_double,
                        cairo_antialias_t,
                    ) -> cairo_int_status_t,
            ),
            glyphs: Some(
                _cairo_xcb_fallback_compositor_glyphs
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                        *mut cairo_scaled_font_t,
                        *mut cairo_glyph_t,
                        libc::c_int,
                        cairo_bool_t,
                    ) -> cairo_int_status_t,
            ),
        };
        init
    }
};
static mut _cairo_xcb_render_compositor: cairo_compositor_t = unsafe {
    {
        let mut init = cairo_compositor {
            delegate: &_cairo_xcb_fallback_compositor as *const cairo_compositor_t,
            paint: Some(
                _cairo_xcb_render_compositor_paint
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_xcb_render_compositor_mask
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_xcb_render_compositor_stroke
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                        *const cairo_path_fixed_t,
                        *const cairo_stroke_style_t,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        libc::c_double,
                        cairo_antialias_t,
                    ) -> cairo_int_status_t,
            ),
            fill: Some(
                _cairo_xcb_render_compositor_fill
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                        *const cairo_path_fixed_t,
                        cairo_fill_rule_t,
                        libc::c_double,
                        cairo_antialias_t,
                    ) -> cairo_int_status_t,
            ),
            glyphs: Some(
                _cairo_xcb_render_compositor_glyphs
                    as unsafe extern "C" fn(
                        *const cairo_compositor_t,
                        *mut cairo_composite_rectangles_t,
                        *mut cairo_scaled_font_t,
                        *mut cairo_glyph_t,
                        libc::c_int,
                        cairo_bool_t,
                    ) -> cairo_int_status_t,
            ),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn get_compositor(
    mut s: *mut *mut cairo_surface_t,
) -> *const cairo_compositor_t {
    let mut surface: *mut cairo_xcb_surface_t = *s as *mut cairo_xcb_surface_t;
    if !((*surface).fallback).is_null() {
        *s = &mut (*(*surface).fallback).base;
        return (*(*s as *mut cairo_image_surface_t)).compositor;
    }
    return &_cairo_xcb_render_compositor;
}
unsafe extern "C" fn _cairo_xcb_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_t = abstract_surface as *mut cairo_surface_t;
    let mut compositor: *const cairo_compositor_t = get_compositor(&mut surface);
    return _cairo_compositor_paint(compositor, surface, op, source, clip);
}
unsafe extern "C" fn _cairo_xcb_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_t = abstract_surface as *mut cairo_surface_t;
    let mut compositor: *const cairo_compositor_t = get_compositor(&mut surface);
    return _cairo_compositor_mask(compositor, surface, op, source, mask, clip);
}
unsafe extern "C" fn _cairo_xcb_surface_stroke(
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
    let mut surface: *mut cairo_surface_t = abstract_surface as *mut cairo_surface_t;
    let mut compositor: *const cairo_compositor_t = get_compositor(&mut surface);
    return _cairo_compositor_stroke(
        compositor,
        surface,
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
}
unsafe extern "C" fn _cairo_xcb_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_t = abstract_surface as *mut cairo_surface_t;
    let mut compositor: *const cairo_compositor_t = get_compositor(&mut surface);
    return _cairo_compositor_fill(
        compositor,
        surface,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    );
}
unsafe extern "C" fn _cairo_xcb_surface_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_t = abstract_surface as *mut cairo_surface_t;
    let mut compositor: *const cairo_compositor_t = get_compositor(&mut surface);
    return _cairo_compositor_glyphs(
        compositor,
        surface,
        op,
        source,
        glyphs,
        num_glyphs,
        scaled_font,
        clip,
    );
}
#[no_mangle]
pub static mut _cairo_xcb_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_XCB,
            finish: Some(
                _cairo_xcb_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_xcb_surface_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: Some(
                _cairo_xcb_surface_create_similar_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_format_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            map_to_image: Some(
                _cairo_xcb_surface_map_to_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_rectangle_int_t,
                    ) -> *mut cairo_image_surface_t,
            ),
            unmap_image: Some(
                _cairo_xcb_surface_unmap
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
            source: Some(
                _cairo_xcb_surface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_xcb_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_xcb_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: None,
            copy_page: None,
            show_page: None,
            get_extents: Some(
                _cairo_xcb_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_xcb_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: Some(
                _cairo_xcb_surface_flush
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> cairo_status_t,
            ),
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_xcb_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_xcb_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_xcb_surface_stroke
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
                _cairo_xcb_surface_fill
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
                _cairo_xcb_surface_glyphs
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
pub unsafe extern "C" fn _cairo_xcb_surface_create_internal(
    mut screen: *mut cairo_xcb_screen_t,
    mut drawable: xcb_drawable_t,
    mut owns_pixmap: cairo_bool_t,
    mut pixman_format: pixman_format_code_t,
    mut xrender_format: xcb_render_pictformat_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    surface = (if ::std::mem::size_of::<cairo_xcb_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xcb_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xcb_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_xcb_surface_backend,
        &mut (*(*screen).connection).device,
        _cairo_content_from_pixman_format(pixman_format),
        0 as libc::c_int,
    );
    let ref mut fresh13 = (*surface).connection;
    *fresh13 = _cairo_xcb_connection_reference((*screen).connection);
    let ref mut fresh14 = (*surface).screen;
    *fresh14 = screen;
    cairo_list_add(&mut (*surface).link, &mut (*screen).surfaces);
    (*surface).drawable = drawable;
    (*surface).owns_pixmap = owns_pixmap;
    (*surface).deferred_clear = 0 as libc::c_int;
    (*surface).deferred_clear_color = *_cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
    (*surface).width = width;
    (*surface).height = height;
    (*surface)
        .depth = ((pixman_format as libc::c_uint >> 12 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (pixman_format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint))
        .wrapping_add(
            (pixman_format as libc::c_uint >> 8 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint),
        )
        .wrapping_add(
            (pixman_format as libc::c_uint >> 4 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint),
        )
        .wrapping_add(
            (pixman_format as libc::c_uint >> 0 as libc::c_int
                & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << (pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint),
        ) as libc::c_int;
    (*surface).picture = 0 as libc::c_long as xcb_render_picture_t;
    if (*(*screen).connection).force_precision != -(1 as libc::c_int) {
        (*surface).precision = (*(*screen).connection).force_precision as uint32_t;
    } else {
        (*surface).precision = XCB_RENDER_POLY_MODE_IMPRECISE as libc::c_int as uint32_t;
    }
    (*surface).pixman_format = pixman_format;
    (*surface).xrender_format = xrender_format;
    let ref mut fresh15 = (*surface).fallback;
    *fresh15 = 0 as *mut cairo_image_surface_t;
    _cairo_boxes_init(&mut (*surface).fallback_damage);
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_xcb_screen_from_visual(
    mut connection: *mut xcb_connection_t,
    mut visual: *mut xcb_visualtype_t,
    mut depth: *mut libc::c_int,
) -> *mut xcb_screen_t {
    let mut d: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: 0 as *mut xcb_depth_t,
        rem: 0,
        index: 0,
    };
    let mut s: xcb_screen_iterator_t = xcb_screen_iterator_t {
        data: 0 as *mut xcb_screen_t,
        rem: 0,
        index: 0,
    };
    s = xcb_setup_roots_iterator(xcb_get_setup(connection));
    while s.rem != 0 {
        if (*s.data).root_visual == (*visual).visual_id {
            *depth = (*s.data).root_depth as libc::c_int;
            return s.data;
        }
        d = xcb_screen_allowed_depths_iterator(s.data);
        while d.rem != 0 {
            let mut v: xcb_visualtype_iterator_t = xcb_depth_visuals_iterator(d.data);
            while v.rem != 0 {
                if (*v.data).visual_id == (*visual).visual_id {
                    *depth = (*d.data).depth as libc::c_int;
                    return s.data;
                }
                xcb_visualtype_next(&mut v);
            }
            xcb_depth_next(&mut d);
        }
        xcb_screen_next(&mut s);
    }
    return 0 as *mut xcb_screen_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_surface_create(
    mut connection: *mut xcb_connection_t,
    mut drawable: xcb_drawable_t,
    mut visual: *mut xcb_visualtype_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut screen: *mut cairo_xcb_screen_t = 0 as *mut cairo_xcb_screen_t;
    let mut xcb_screen: *mut xcb_screen_t = 0 as *mut xcb_screen_t;
    let mut image_masks: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    let mut xrender_format: xcb_render_pictformat_t = 0;
    let mut depth: libc::c_int = 0;
    if xcb_connection_has_error(connection) != 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_WRITE_ERROR));
    }
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    xcb_screen = _cairo_xcb_screen_from_visual(connection, visual, &mut depth);
    if xcb_screen.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_VISUAL));
    }
    image_masks.alpha_mask = 0 as libc::c_int as libc::c_ulong;
    image_masks.red_mask = (*visual).red_mask as libc::c_ulong;
    image_masks.green_mask = (*visual).green_mask as libc::c_ulong;
    image_masks.blue_mask = (*visual).blue_mask as libc::c_ulong;
    if depth == 32 as libc::c_int {
        image_masks
            .alpha_mask = (0xffffffff as libc::c_uint
            & !((*visual).red_mask | (*visual).green_mask | (*visual).blue_mask))
            as libc::c_ulong;
    }
    if depth > 16 as libc::c_int {
        image_masks.bpp = 32 as libc::c_int;
    } else if depth > 8 as libc::c_int {
        image_masks.bpp = 16 as libc::c_int;
    } else if depth > 1 as libc::c_int {
        image_masks.bpp = 8 as libc::c_int;
    } else {
        image_masks.bpp = 1 as libc::c_int;
    }
    if _pixman_format_from_masks(&mut image_masks, &mut pixman_format) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_FORMAT));
    }
    screen = _cairo_xcb_screen_get(connection, xcb_screen);
    if screen.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    xrender_format = _cairo_xcb_connection_get_xrender_format_for_visual(
        (*screen).connection,
        (*visual).visual_id,
    );
    return _cairo_xcb_surface_create_internal(
        screen,
        drawable,
        0 as libc::c_int,
        pixman_format,
        xrender_format,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_surface_create_for_bitmap(
    mut connection: *mut xcb_connection_t,
    mut screen: *mut xcb_screen_t,
    mut bitmap: xcb_pixmap_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut cairo_xcb_screen: *mut cairo_xcb_screen_t = 0 as *mut cairo_xcb_screen_t;
    if xcb_connection_has_error(connection) != 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_WRITE_ERROR));
    }
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    cairo_xcb_screen = _cairo_xcb_screen_get(connection, screen);
    if cairo_xcb_screen.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    return _cairo_xcb_surface_create_internal(
        cairo_xcb_screen,
        bitmap,
        0 as libc::c_int,
        PIXMAN_a1,
        (*(*cairo_xcb_screen).connection)
            .standard_formats[CAIRO_FORMAT_A1 as libc::c_int as usize],
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_surface_create_with_xrender_format(
    mut connection: *mut xcb_connection_t,
    mut screen: *mut xcb_screen_t,
    mut drawable: xcb_drawable_t,
    mut format: *mut xcb_render_pictforminfo_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut cairo_xcb_screen: *mut cairo_xcb_screen_t = 0 as *mut cairo_xcb_screen_t;
    let mut image_masks: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    if xcb_connection_has_error(connection) != 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_WRITE_ERROR));
    }
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    image_masks
        .alpha_mask = ((*format).direct.alpha_mask as libc::c_ulong)
        << (*format).direct.alpha_shift as libc::c_int;
    image_masks
        .red_mask = ((*format).direct.red_mask as libc::c_ulong)
        << (*format).direct.red_shift as libc::c_int;
    image_masks
        .green_mask = ((*format).direct.green_mask as libc::c_ulong)
        << (*format).direct.green_shift as libc::c_int;
    image_masks
        .blue_mask = ((*format).direct.blue_mask as libc::c_ulong)
        << (*format).direct.blue_shift as libc::c_int;
    if (*format).depth as libc::c_int > 16 as libc::c_int {
        image_masks.bpp = 32 as libc::c_int;
    } else if (*format).depth as libc::c_int > 8 as libc::c_int {
        image_masks.bpp = 16 as libc::c_int;
    } else if (*format).depth as libc::c_int > 1 as libc::c_int {
        image_masks.bpp = 8 as libc::c_int;
    } else {
        image_masks.bpp = 1 as libc::c_int;
    }
    if _pixman_format_from_masks(&mut image_masks, &mut pixman_format) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_FORMAT));
    }
    cairo_xcb_screen = _cairo_xcb_screen_get(connection, screen);
    if cairo_xcb_screen.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    return _cairo_xcb_surface_create_internal(
        cairo_xcb_screen,
        drawable,
        0 as libc::c_int,
        pixman_format,
        (*format).id,
        width,
        height,
    );
}
unsafe extern "C" fn _drawable_changed(mut surface: *mut cairo_xcb_surface_t) {
    _cairo_surface_set_error(
        &mut (*surface).base,
        _cairo_surface_begin_modification(&mut (*surface).base) as cairo_int_status_t,
    );
    _cairo_boxes_clear(&mut (*surface).fallback_damage);
    cairo_surface_destroy(&mut (*(*surface).fallback).base);
    (*surface).deferred_clear = 0 as libc::c_int;
    let ref mut fresh16 = (*surface).fallback;
    *fresh16 = 0 as *mut cairo_image_surface_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_surface_set_size(
    mut abstract_surface: *mut cairo_surface_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut surface: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    if (*abstract_surface).status as u64 != 0 {
        return;
    }
    if (*abstract_surface).finished() != 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    if _cairo_surface_is_xcb(abstract_surface) == 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        );
        return;
    }
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int
        || width <= 0 as libc::c_int || height <= 0 as libc::c_int
    {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_INVALID_SIZE) as cairo_int_status_t,
        );
        return;
    }
    surface = abstract_surface as *mut cairo_xcb_surface_t;
    _drawable_changed(surface);
    (*surface).width = width;
    (*surface).height = height;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xcb_surface_set_drawable(
    mut abstract_surface: *mut cairo_surface_t,
    mut drawable: xcb_drawable_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut surface: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    if (*abstract_surface).status as u64 != 0 {
        return;
    }
    if (*abstract_surface).finished() != 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    if _cairo_surface_is_xcb(abstract_surface) == 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        );
        return;
    }
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int
        || width <= 0 as libc::c_int || height <= 0 as libc::c_int
    {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_INVALID_SIZE) as cairo_int_status_t,
        );
        return;
    }
    surface = abstract_surface as *mut cairo_xcb_surface_t;
    if (*surface).owns_pixmap != 0 {
        return;
    }
    _drawable_changed(surface);
    if (*surface).drawable != drawable {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_xcb_connection_acquire((*surface).connection);
        if status as u64 != 0 {
            return;
        }
        if (*surface).picture as libc::c_long != 0 as libc::c_long {
            _cairo_xcb_connection_render_free_picture(
                (*surface).connection,
                (*surface).picture,
            );
            (*surface).picture = 0 as libc::c_long as xcb_render_picture_t;
        }
        _cairo_xcb_connection_release((*surface).connection);
        (*surface).drawable = drawable;
    }
    (*surface).width = width;
    (*surface).height = height;
}
