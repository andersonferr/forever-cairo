use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_device_flush(device: *mut cairo_device_t);
    fn cairo_device_finish(device: *mut cairo_device_t);
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn cairo_recording_surface_create(
        content: cairo_content_t,
        extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_clip_is_polygon(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _cairo_clip_is_region(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_map_to_image(
        surface: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_surface_unmap_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
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
    fn _cairo_surface_mask(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_flush(
        surface: *mut cairo_surface_t,
        flags: libc::c_uint,
    ) -> cairo_status_t;
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
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn _cairo_image_surface_create_with_content(
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_time_to_s(t: cairo_time_t) -> libc::c_double;
    fn _cairo_time_get() -> cairo_time_t;
    fn _cairo_recording_surface_replay_one(
        surface: *mut cairo_recording_surface_t,
        index: libc::c_ulong,
        target: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_device_init(
        device: *mut cairo_device_t,
        backend: *const cairo_device_backend_t,
    );
    fn _cairo_device_create_in_error(status: cairo_status_t) -> *mut cairo_device_t;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
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
    fn _cairo_composite_rectangles_fini(extents: *mut cairo_composite_rectangles_t);
    fn _cairo_output_stream_create(
        write_func: cairo_write_func_t,
        close_func: cairo_close_func_t,
        closure: *mut libc::c_void,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_destroy(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_printf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _cairo_script_context_create_internal(
        stream: *mut cairo_output_stream_t,
    ) -> *mut cairo_device_t;
    fn _cairo_script_context_attach_snapshots(
        device: *mut cairo_device_t,
        enable: cairo_bool_t,
    );
    fn cairo_script_surface_create(
        script: *mut cairo_device_t,
        content: cairo_content_t,
        width: libc::c_double,
        height: libc::c_double,
    ) -> *mut cairo_surface_t;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type cairo_surface_observer_mode_t = libc::c_uint;
pub const CAIRO_SURFACE_OBSERVER_RECORD_OPERATIONS: cairo_surface_observer_mode_t = 1;
pub const CAIRO_SURFACE_OBSERVER_NORMAL: cairo_surface_observer_mode_t = 0;
pub type cairo_surface_observer_t = _cairo_surface_observer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_observer {
    pub base: cairo_surface_t,
    pub target: *mut cairo_surface_t,
    pub log: cairo_observation_t,
    pub paint_callbacks: cairo_list_t,
    pub mask_callbacks: cairo_list_t,
    pub fill_callbacks: cairo_list_t,
    pub stroke_callbacks: cairo_list_t,
    pub glyphs_callbacks: cairo_list_t,
    pub flush_callbacks: cairo_list_t,
    pub finish_callbacks: cairo_list_t,
}
pub type cairo_observation_t = _cairo_observation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_observation {
    pub num_surfaces: libc::c_int,
    pub num_contexts: libc::c_int,
    pub num_sources_acquired: libc::c_int,
    pub paint: paint,
    pub mask: mask,
    pub fill: fill,
    pub stroke: stroke,
    pub glyphs: glyphs,
    pub timings: cairo_array_t,
    pub record: *mut cairo_recording_surface_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyphs {
    pub elapsed: cairo_time_t,
    pub count: libc::c_uint,
    pub extents: extents,
    pub operators: [libc::c_uint; 29],
    pub source: pattern,
    pub clip: clip,
    pub noop: libc::c_uint,
    pub slowest: cairo_observation_record_t,
}
pub type cairo_observation_record_t = _cairo_observation_record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_observation_record {
    pub target_content: cairo_content_t,
    pub target_width: libc::c_int,
    pub target_height: libc::c_int,
    pub index: libc::c_int,
    pub op: cairo_operator_t,
    pub source: libc::c_int,
    pub mask: libc::c_int,
    pub num_glyphs: libc::c_int,
    pub path: libc::c_int,
    pub fill_rule: libc::c_int,
    pub tolerance: libc::c_double,
    pub antialias: libc::c_int,
    pub clip: libc::c_int,
    pub elapsed: cairo_time_t,
}
pub type cairo_time_t = cairo_int64_t;
pub type cairo_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clip {
    pub type_0: [libc::c_uint; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern {
    pub type_0: [libc::c_uint; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extents {
    pub area: stat,
    pub bounded: libc::c_uint,
    pub unbounded: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub sum: libc::c_double,
    pub sum_sq: libc::c_double,
    pub count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stroke {
    pub elapsed: cairo_time_t,
    pub count: libc::c_uint,
    pub extents: extents,
    pub operators: [libc::c_uint; 29],
    pub caps: [libc::c_uint; 3],
    pub joins: [libc::c_uint; 3],
    pub antialias: [libc::c_uint; 7],
    pub source: pattern,
    pub path: path,
    pub line_width: stat,
    pub clip: clip,
    pub noop: libc::c_uint,
    pub slowest: cairo_observation_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path {
    pub type_0: [libc::c_uint; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fill {
    pub elapsed: cairo_time_t,
    pub count: libc::c_uint,
    pub extents: extents,
    pub operators: [libc::c_uint; 29],
    pub source: pattern,
    pub path: path,
    pub antialias: [libc::c_uint; 7],
    pub fill_rule: [libc::c_uint; 2],
    pub clip: clip,
    pub noop: libc::c_uint,
    pub slowest: cairo_observation_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mask {
    pub elapsed: cairo_time_t,
    pub count: libc::c_uint,
    pub extents: extents,
    pub operators: [libc::c_uint; 29],
    pub source: pattern,
    pub mask: pattern,
    pub clip: clip,
    pub noop: libc::c_uint,
    pub slowest: cairo_observation_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paint {
    pub elapsed: cairo_time_t,
    pub count: libc::c_uint,
    pub extents: extents,
    pub operators: [libc::c_uint; 29],
    pub source: pattern,
    pub clip: clip,
    pub noop: libc::c_uint,
    pub slowest: cairo_observation_record_t,
}
pub type cairo_device_observer_t = _cairo_device_observer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device_observer {
    pub base: cairo_device_t,
    pub target: *mut cairo_device_t,
    pub log: cairo_observation_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_list {
    pub link: cairo_list_t,
    pub func: cairo_surface_observer_callback_t,
    pub data: *mut libc::c_void,
}
pub type cairo_surface_observer_callback_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_surface_t,
        *mut cairo_surface_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
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
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_surface_subsurface_t = _cairo_surface_subsurface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_subsurface {
    pub base: cairo_surface_t,
    pub extents: cairo_rectangle_int_t,
    pub target: *mut cairo_surface_t,
    pub snapshot: *mut cairo_surface_t,
}
pub const CAIRO_INTERNAL_SURFACE_TYPE_OBSERVER: _cairo_internal_surface_type = 4099;
pub const CAIRO_INTERNAL_DEVICE_TYPE_OBSERVER: _cairo_internal_device_type = 4096;
pub type cairo_output_stream_t = _cairo_output_stream;
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
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
pub type _cairo_internal_surface_type = libc::c_uint;
pub const CAIRO_INTERNAL_SURFACE_TYPE_QUARTZ_SNAPSHOT: _cairo_internal_surface_type = 4105;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TYPE3_GLYPH: _cairo_internal_surface_type = 4104;
pub const CAIRO_INTERNAL_SURFACE_TYPE_NULL: _cairo_internal_surface_type = 4103;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_WRAPPING: _cairo_internal_surface_type = 4102;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_PAGINATED: _cairo_internal_surface_type = 4101;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_FALLBACK: _cairo_internal_surface_type = 4100;
pub const CAIRO_INTERNAL_SURFACE_TYPE_ANALYSIS: _cairo_internal_surface_type = 4098;
pub const CAIRO_INTERNAL_SURFACE_TYPE_PAGINATED: _cairo_internal_surface_type = 4097;
pub const CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT: _cairo_internal_surface_type = 4096;
pub type _cairo_internal_device_type = libc::c_uint;
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
#[inline]
unsafe extern "C" fn _cairo_path_fixed_stroke_is_rectilinear(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).stroke_is_rectilinear() as cairo_bool_t;
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
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline(always)]
unsafe extern "C" fn _cairo_time_get_delta(mut t: cairo_time_t) -> cairo_time_t {
    let mut now: cairo_time_t = 0;
    now = _cairo_time_get();
    return now - t;
}
#[inline(always)]
unsafe extern "C" fn _cairo_time_to_ns(mut t: cairo_time_t) -> libc::c_double {
    return 1.0e9f64 * _cairo_time_to_s(t);
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_observer(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_INTERNAL_SURFACE_TYPE_OBSERVER as libc::c_int as cairo_surface_type_t
            as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_device_is_observer(
    mut device: *mut cairo_device_t,
) -> cairo_bool_t {
    return ((*(*device).backend).type_0 as libc::c_int
        == CAIRO_INTERNAL_DEVICE_TYPE_OBSERVER as libc::c_int as cairo_device_type_t
            as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn init_stats(mut s: *mut stat) {
    (*s).min = ::std::f64::INFINITY;
    (*s).max = -::std::f64::INFINITY;
}
unsafe extern "C" fn init_extents(mut e: *mut extents) {
    init_stats(&mut (*e).area);
}
unsafe extern "C" fn init_pattern(mut p: *mut pattern) {}
unsafe extern "C" fn init_path(mut p: *mut path) {}
unsafe extern "C" fn init_clip(mut c: *mut clip) {}
unsafe extern "C" fn init_paint(mut p: *mut paint) {
    init_extents(&mut (*p).extents);
    init_pattern(&mut (*p).source);
    init_clip(&mut (*p).clip);
}
unsafe extern "C" fn init_mask(mut m: *mut mask) {
    init_extents(&mut (*m).extents);
    init_pattern(&mut (*m).source);
    init_pattern(&mut (*m).mask);
    init_clip(&mut (*m).clip);
}
unsafe extern "C" fn init_fill(mut f: *mut fill) {
    init_extents(&mut (*f).extents);
    init_pattern(&mut (*f).source);
    init_path(&mut (*f).path);
    init_clip(&mut (*f).clip);
}
unsafe extern "C" fn init_stroke(mut s: *mut stroke) {
    init_extents(&mut (*s).extents);
    init_pattern(&mut (*s).source);
    init_path(&mut (*s).path);
    init_clip(&mut (*s).clip);
}
unsafe extern "C" fn init_glyphs(mut g: *mut glyphs) {
    init_extents(&mut (*g).extents);
    init_pattern(&mut (*g).source);
    init_clip(&mut (*g).clip);
}
unsafe extern "C" fn log_init(
    mut log: *mut cairo_observation_t,
    mut record: cairo_bool_t,
) -> cairo_status_t {
    memset(
        log as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_observation_t>() as libc::c_ulong,
    );
    init_paint(&mut (*log).paint);
    init_mask(&mut (*log).mask);
    init_fill(&mut (*log).fill);
    init_stroke(&mut (*log).stroke);
    init_glyphs(&mut (*log).glyphs);
    _cairo_array_init(
        &mut (*log).timings,
        ::std::mem::size_of::<cairo_observation_record_t>() as libc::c_ulong
            as libc::c_uint,
    );
    if record != 0 {
        let ref mut fresh8 = (*log).record;
        *fresh8 = cairo_recording_surface_create(
            CAIRO_CONTENT_COLOR_ALPHA,
            0 as *const cairo_rectangle_t,
        ) as *mut cairo_recording_surface_t;
        if (*(*log).record).base.status as u64 != 0 {
            return (*(*log).record).base.status;
        }
        (*(*log).record).optimize_clears = 0 as libc::c_int;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn log_fini(mut log: *mut cairo_observation_t) {
    _cairo_array_fini(&mut (*log).timings);
    cairo_surface_destroy(&mut (*(*log).record).base);
}
unsafe extern "C" fn get_pattern_surface(
    mut pattern: *const cairo_pattern_t,
) -> *mut cairo_surface_t {
    return (*(pattern as *mut cairo_surface_pattern_t)).surface;
}
unsafe extern "C" fn classify_pattern(
    mut pattern: *const cairo_pattern_t,
    mut target: *const cairo_surface_t,
) -> libc::c_int {
    let mut classify: libc::c_int = 0;
    match (*pattern).type_0 as libc::c_uint {
        1 => {
            if (*get_pattern_surface(pattern)).type_0 as libc::c_uint
                == (*target).type_0 as libc::c_uint
            {
                classify = 0 as libc::c_int;
            } else if (*get_pattern_surface(pattern)).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
            {
                classify = 1 as libc::c_int;
            } else {
                classify = 2 as libc::c_int;
            }
        }
        2 => {
            classify = 4 as libc::c_int;
        }
        3 => {
            classify = 5 as libc::c_int;
        }
        4 => {
            classify = 6 as libc::c_int;
        }
        5 => {
            classify = 7 as libc::c_int;
        }
        0 | _ => {
            classify = 3 as libc::c_int;
        }
    }
    return classify;
}
unsafe extern "C" fn add_pattern(
    mut stats: *mut pattern,
    mut pattern: *const cairo_pattern_t,
    mut target: *const cairo_surface_t,
) {
    let ref mut fresh9 = (*stats).type_0[classify_pattern(pattern, target) as usize];
    *fresh9 = (*fresh9).wrapping_add(1);
}
unsafe extern "C" fn classify_path(
    mut path: *const cairo_path_fixed_t,
    mut is_fill: cairo_bool_t,
) -> libc::c_int {
    let mut classify: libc::c_int = 0;
    classify = -(1 as libc::c_int);
    if is_fill != 0 {
        if (*path).fill_is_empty() != 0 {
            classify = 0 as libc::c_int;
        } else if _cairo_path_fixed_fill_is_rectilinear(path) != 0 {
            classify = if (*path).fill_maybe_region() as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                2 as libc::c_int
            };
        }
    } else if _cairo_path_fixed_stroke_is_rectilinear(path) != 0 {
        classify = 2 as libc::c_int;
    }
    if classify == -(1 as libc::c_int) {
        classify = 3 as libc::c_int
            + ((*path).has_curve_to() as libc::c_int != 0 as libc::c_int) as libc::c_int;
    }
    return classify;
}
unsafe extern "C" fn add_path(
    mut stats: *mut path,
    mut path: *const cairo_path_fixed_t,
    mut is_fill: cairo_bool_t,
) {
    let ref mut fresh10 = (*stats).type_0[classify_path(path, is_fill) as usize];
    *fresh10 = (*fresh10).wrapping_add(1);
}
unsafe extern "C" fn classify_clip(mut clip: *const cairo_clip_t) -> libc::c_int {
    let mut classify: libc::c_int = 0;
    if clip.is_null() {
        classify = 0 as libc::c_int;
    } else if _cairo_clip_is_region(clip) != 0 {
        classify = 1 as libc::c_int;
    } else if ((*clip).path).is_null() {
        classify = 2 as libc::c_int;
    } else if ((*(*clip).path).prev).is_null() {
        classify = 3 as libc::c_int;
    } else if _cairo_clip_is_polygon(clip) != 0 {
        classify = 4 as libc::c_int;
    } else {
        classify = 5 as libc::c_int;
    }
    return classify;
}
unsafe extern "C" fn add_clip(mut stats: *mut clip, mut clip: *const cairo_clip_t) {
    let ref mut fresh11 = (*stats).type_0[classify_clip(clip) as usize];
    *fresh11 = (*fresh11).wrapping_add(1);
}
unsafe extern "C" fn stats_add(mut s: *mut stat, mut v: libc::c_double) {
    if v < (*s).min {
        (*s).min = v;
    }
    if v > (*s).max {
        (*s).max = v;
    }
    (*s).sum += v;
    (*s).sum_sq += v * v;
    let ref mut fresh12 = (*s).count;
    *fresh12 = (*fresh12).wrapping_add(1);
}
unsafe extern "C" fn add_extents(
    mut stats: *mut extents,
    mut extents: *const cairo_composite_rectangles_t,
) {
    let mut r: *const cairo_rectangle_int_t = if (*extents).is_bounded != 0 {
        &(*extents).bounded
    } else {
        &(*extents).unbounded
    };
    stats_add(&mut (*stats).area, ((*r).width * (*r).height) as libc::c_double);
    let ref mut fresh13 = (*stats).bounded;
    *fresh13 = (*fresh13)
        .wrapping_add(
            ((*extents).is_bounded != 0 as libc::c_int as libc::c_uint) as libc::c_int
                as libc::c_uint,
        );
    let ref mut fresh14 = (*stats).unbounded;
    *fresh14 = (*fresh14)
        .wrapping_add(
            ((*extents).is_bounded == 0 as libc::c_int as libc::c_uint) as libc::c_int
                as libc::c_uint,
        );
}
unsafe extern "C" fn _cairo_device_observer_lock(mut _device: *mut libc::c_void) {
    let mut device: *mut cairo_device_observer_t = _device
        as *mut cairo_device_observer_t;
    let mut ignored: cairo_status_t = CAIRO_STATUS_SUCCESS;
    ignored = cairo_device_acquire((*device).target);
}
unsafe extern "C" fn _cairo_device_observer_unlock(mut _device: *mut libc::c_void) {
    let mut device: *mut cairo_device_observer_t = _device
        as *mut cairo_device_observer_t;
    cairo_device_release((*device).target);
}
unsafe extern "C" fn _cairo_device_observer_flush(
    mut _device: *mut libc::c_void,
) -> cairo_status_t {
    let mut device: *mut cairo_device_observer_t = _device
        as *mut cairo_device_observer_t;
    if ((*device).target).is_null() {
        return CAIRO_STATUS_SUCCESS;
    }
    cairo_device_flush((*device).target);
    return (*(*device).target).status;
}
unsafe extern "C" fn _cairo_device_observer_finish(mut _device: *mut libc::c_void) {
    let mut device: *mut cairo_device_observer_t = _device
        as *mut cairo_device_observer_t;
    log_fini(&mut (*device).log);
    cairo_device_finish((*device).target);
}
unsafe extern "C" fn _cairo_device_observer_destroy(mut _device: *mut libc::c_void) {
    let mut device: *mut cairo_device_observer_t = _device
        as *mut cairo_device_observer_t;
    cairo_device_destroy((*device).target);
    free(device as *mut libc::c_void);
}
static mut _cairo_device_observer_backend: cairo_device_backend_t = unsafe {
    {
        let mut init = _cairo_device_backend {
            type_0: CAIRO_INTERNAL_DEVICE_TYPE_OBSERVER as libc::c_int
                as cairo_device_type_t,
            lock: Some(
                _cairo_device_observer_lock
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            unlock: Some(
                _cairo_device_observer_unlock
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            flush: Some(
                _cairo_device_observer_flush
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            finish: Some(
                _cairo_device_observer_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            destroy: Some(
                _cairo_device_observer_destroy
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn _cairo_device_create_observer_internal(
    mut target: *mut cairo_device_t,
    mut record: cairo_bool_t,
) -> *mut cairo_device_t {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    device = (if ::std::mem::size_of::<cairo_device_observer_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_device_observer_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_device_observer_t;
    if device.is_null() {
        return _cairo_device_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_device_init(&mut (*device).base, &_cairo_device_observer_backend);
    status = log_init(&mut (*device).log, record);
    if status as u64 != 0 {
        free(device as *mut libc::c_void);
        return _cairo_device_create_in_error(status);
    }
    let ref mut fresh15 = (*device).target;
    *fresh15 = cairo_device_reference(target);
    return &mut (*device).base;
}
unsafe extern "C" fn to_device(
    mut suface: *mut cairo_surface_observer_t,
) -> *mut cairo_device_observer_t {
    return (*suface).base.device as *mut cairo_device_observer_t;
}
unsafe extern "C" fn _cairo_surface_create_observer_internal(
    mut device: *mut cairo_device_t,
    mut target: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    surface = (if ::std::mem::size_of::<cairo_surface_observer_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_surface_observer_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_surface_observer_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_surface_observer_backend,
        device,
        (*target).content,
        (*target).is_vector() as cairo_bool_t,
    );
    status = log_init(
        &mut (*surface).log,
        ((*(device as *mut cairo_device_observer_t)).log.record
            != 0 as *mut libc::c_void as *mut cairo_recording_surface_t) as libc::c_int,
    );
    if status as u64 != 0 {
        free(surface as *mut libc::c_void);
        return _cairo_surface_create_in_error(status);
    }
    let ref mut fresh16 = (*surface).target;
    *fresh16 = cairo_surface_reference(target);
    (*surface).base.type_0 = (*(*surface).target).type_0;
    let ref mut fresh17 = (*surface).base;
    (*fresh17).set_is_clear((*(*surface).target).is_clear());
    cairo_list_init(&mut (*surface).paint_callbacks);
    cairo_list_init(&mut (*surface).mask_callbacks);
    cairo_list_init(&mut (*surface).fill_callbacks);
    cairo_list_init(&mut (*surface).stroke_callbacks);
    cairo_list_init(&mut (*surface).glyphs_callbacks);
    cairo_list_init(&mut (*surface).flush_callbacks);
    cairo_list_init(&mut (*surface).finish_callbacks);
    let ref mut fresh18 = (*surface).log.num_surfaces;
    *fresh18 += 1;
    let ref mut fresh19 = (*to_device(surface)).log.num_surfaces;
    *fresh19 += 1;
    return &mut (*surface).base;
}
#[inline]
unsafe extern "C" fn do_callbacks(
    mut surface: *mut cairo_surface_observer_t,
    mut head: *mut cairo_list_t,
) {
    let mut cb: *mut callback_list = 0 as *mut callback_list;
    cb = ({
        let mut mptr__: *const cairo_list_t = (*head).next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut callback_list
    });
    while &mut (*cb).link as *mut cairo_list_t != head {
        ((*cb).func)
            .expect(
                "non-null function pointer",
            )(&mut (*surface).base, (*surface).target, (*cb).data);
        cb = ({
            let mut mptr__: *const cairo_list_t = (*cb).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut callback_list
        });
    }
}
unsafe extern "C" fn _cairo_surface_observer_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    do_callbacks(surface, &mut (*surface).finish_callbacks);
    cairo_surface_destroy((*surface).target);
    log_fini(&mut (*surface).log);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_surface_observer_create_similar(
    mut abstract_other: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut other: *mut cairo_surface_observer_t = abstract_other
        as *mut cairo_surface_observer_t;
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    target = 0 as *mut cairo_surface_t;
    if ((*(*(*other).target).backend).create_similar).is_some() {
        target = ((*(*(*other).target).backend).create_similar)
            .expect(
                "non-null function pointer",
            )((*other).target as *mut libc::c_void, content, width, height);
    }
    if target.is_null() {
        target = _cairo_image_surface_create_with_content(content, width, height);
    }
    surface = _cairo_surface_create_observer_internal((*other).base.device, target);
    cairo_surface_destroy(target);
    return surface;
}
unsafe extern "C" fn _cairo_surface_observer_create_similar_image(
    mut other: *mut libc::c_void,
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_observer_t = other
        as *mut cairo_surface_observer_t;
    if ((*(*(*surface).target).backend).create_similar_image).is_some() {
        return ((*(*(*surface).target).backend).create_similar_image)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, format, width, height);
    }
    return 0 as *mut cairo_surface_t;
}
unsafe extern "C" fn _cairo_surface_observer_map_to_image(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    return _cairo_surface_map_to_image((*surface).target, extents);
}
unsafe extern "C" fn _cairo_surface_observer_unmap_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    return _cairo_surface_unmap_image((*surface).target, image);
}
unsafe extern "C" fn record_target(
    mut r: *mut cairo_observation_record_t,
    mut target: *mut cairo_surface_t,
) {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    (*r).target_content = (*target).content;
    if _cairo_surface_get_extents(target, &mut extents) != 0 {
        (*r).target_width = extents.width;
        (*r).target_height = extents.height;
    } else {
        (*r).target_width = -(1 as libc::c_int);
        (*r).target_height = -(1 as libc::c_int);
    };
}
unsafe extern "C" fn record_paint(
    mut r: *mut cairo_observation_record_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) -> *mut cairo_observation_record_t {
    record_target(r, target);
    (*r).op = op;
    (*r).source = classify_pattern(source, target);
    (*r).mask = -(1 as libc::c_int);
    (*r).num_glyphs = -(1 as libc::c_int);
    (*r).path = -(1 as libc::c_int);
    (*r).fill_rule = -(1 as libc::c_int);
    (*r).tolerance = -(1 as libc::c_int) as libc::c_double;
    (*r).antialias = -(1 as libc::c_int);
    (*r).clip = classify_clip(clip);
    (*r).elapsed = elapsed;
    return r;
}
unsafe extern "C" fn record_mask(
    mut r: *mut cairo_observation_record_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) -> *mut cairo_observation_record_t {
    record_target(r, target);
    (*r).op = op;
    (*r).source = classify_pattern(source, target);
    (*r).mask = classify_pattern(mask, target);
    (*r).num_glyphs = -(1 as libc::c_int);
    (*r).path = -(1 as libc::c_int);
    (*r).fill_rule = -(1 as libc::c_int);
    (*r).tolerance = -(1 as libc::c_int) as libc::c_double;
    (*r).antialias = -(1 as libc::c_int);
    (*r).clip = classify_clip(clip);
    (*r).elapsed = elapsed;
    return r;
}
unsafe extern "C" fn record_fill(
    mut r: *mut cairo_observation_record_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) -> *mut cairo_observation_record_t {
    record_target(r, target);
    (*r).op = op;
    (*r).source = classify_pattern(source, target);
    (*r).mask = -(1 as libc::c_int);
    (*r).num_glyphs = -(1 as libc::c_int);
    (*r).path = classify_path(path, 1 as libc::c_int);
    (*r).fill_rule = fill_rule as libc::c_int;
    (*r).tolerance = tolerance;
    (*r).antialias = antialias as libc::c_int;
    (*r).clip = classify_clip(clip);
    (*r).elapsed = elapsed;
    return r;
}
unsafe extern "C" fn record_stroke(
    mut r: *mut cairo_observation_record_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) -> *mut cairo_observation_record_t {
    record_target(r, target);
    (*r).op = op;
    (*r).source = classify_pattern(source, target);
    (*r).mask = -(1 as libc::c_int);
    (*r).num_glyphs = -(1 as libc::c_int);
    (*r).path = classify_path(path, 0 as libc::c_int);
    (*r).fill_rule = -(1 as libc::c_int);
    (*r).tolerance = tolerance;
    (*r).antialias = antialias as libc::c_int;
    (*r).clip = classify_clip(clip);
    (*r).elapsed = elapsed;
    return r;
}
unsafe extern "C" fn record_glyphs(
    mut r: *mut cairo_observation_record_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) -> *mut cairo_observation_record_t {
    record_target(r, target);
    (*r).op = op;
    (*r).source = classify_pattern(source, target);
    (*r).mask = -(1 as libc::c_int);
    (*r).path = -(1 as libc::c_int);
    (*r).num_glyphs = num_glyphs;
    (*r).fill_rule = -(1 as libc::c_int);
    (*r).tolerance = -(1 as libc::c_int) as libc::c_double;
    (*r).antialias = -(1 as libc::c_int);
    (*r).clip = classify_clip(clip);
    (*r).elapsed = elapsed;
    return r;
}
unsafe extern "C" fn add_record(
    mut log: *mut cairo_observation_t,
    mut r: *mut cairo_observation_record_t,
) {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    (*r)
        .index = (if !((*log).record).is_null() {
        (*(*log).record).commands.num_elements
    } else {
        0 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    status = _cairo_array_append(&mut (*log).timings, r as *const libc::c_void)
        as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void add_record(cairo_observation_t *, cairo_observation_record_t *)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn _cairo_surface_sync(
    mut target: *mut cairo_surface_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    extents.x = x;
    extents.y = y;
    extents.width = 1 as libc::c_int;
    extents.height = 1 as libc::c_int;
    _cairo_surface_unmap_image(
        target,
        _cairo_surface_map_to_image(target, &mut extents),
    );
}
unsafe extern "C" fn midpt(
    mut extents: *const cairo_composite_rectangles_t,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
) {
    *x = (*extents).bounded.x + (*extents).bounded.width / 2 as libc::c_int;
    *y = (*extents).bounded.y + (*extents).bounded.height / 2 as libc::c_int;
}
unsafe extern "C" fn add_record_paint(
    mut log: *mut cairo_observation_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) {
    let mut record: cairo_observation_record_t = cairo_observation_record_t {
        target_content: 0 as cairo_content_t,
        target_width: 0,
        target_height: 0,
        index: 0,
        op: CAIRO_OPERATOR_CLEAR,
        source: 0,
        mask: 0,
        num_glyphs: 0,
        path: 0,
        fill_rule: 0,
        tolerance: 0.,
        antialias: 0,
        clip: 0,
        elapsed: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    add_record(log, record_paint(&mut record, target, op, source, clip, elapsed));
    if !((*log).record).is_null() {
        status = ((*(*(*log).record).base.backend).paint)
            .expect(
                "non-null function pointer",
            )(
            &mut (*(*log).record).base as *mut cairo_surface_t as *mut libc::c_void,
            op,
            source,
            clip,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
                699 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 143],
                    &[libc::c_char; 143],
                >(
                    b"void add_record_paint(cairo_observation_t *, cairo_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_clip_t *, cairo_time_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*log).paint.slowest.elapsed < elapsed {
        (*log).paint.slowest = record;
    }
    (*log).paint.elapsed = (*log).paint.elapsed + elapsed;
}
unsafe extern "C" fn _cairo_surface_observer_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut device: *mut cairo_device_observer_t = to_device(surface);
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut t: cairo_time_t = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let ref mut fresh20 = (*surface).log.paint.count;
    *fresh20 = (*fresh20).wrapping_add(1);
    let ref mut fresh21 = (*surface).log.paint.operators[op as usize];
    *fresh21 = (*fresh21).wrapping_add(1);
    add_pattern(&mut (*surface).log.paint.source, source, (*surface).target);
    add_clip(&mut (*surface).log.paint.clip, clip);
    let ref mut fresh22 = (*device).log.paint.count;
    *fresh22 = (*fresh22).wrapping_add(1);
    let ref mut fresh23 = (*device).log.paint.operators[op as usize];
    *fresh23 = (*fresh23).wrapping_add(1);
    add_pattern(&mut (*device).log.paint.source, source, (*surface).target);
    add_clip(&mut (*device).log.paint.clip, clip);
    status = _cairo_composite_rectangles_init_for_paint(
        &mut composite,
        (*surface).target,
        op,
        source,
        clip,
    );
    if status as u64 != 0 {
        let ref mut fresh24 = (*surface).log.paint.noop;
        *fresh24 = (*fresh24).wrapping_add(1);
        let ref mut fresh25 = (*device).log.paint.noop;
        *fresh25 = (*fresh25).wrapping_add(1);
        return status;
    }
    midpt(&mut composite, &mut x, &mut y);
    add_extents(&mut (*surface).log.paint.extents, &mut composite);
    add_extents(&mut (*device).log.paint.extents, &mut composite);
    _cairo_composite_rectangles_fini(&mut composite);
    t = _cairo_time_get();
    status = _cairo_surface_paint((*surface).target, op, source, clip)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_surface_sync((*surface).target, x, y);
    t = _cairo_time_get_delta(t);
    add_record_paint(&mut (*surface).log, (*surface).target, op, source, clip, t);
    add_record_paint(&mut (*device).log, (*surface).target, op, source, clip, t);
    do_callbacks(surface, &mut (*surface).paint_callbacks);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn add_record_mask(
    mut log: *mut cairo_observation_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) {
    let mut record: cairo_observation_record_t = cairo_observation_record_t {
        target_content: 0 as cairo_content_t,
        target_width: 0,
        target_height: 0,
        index: 0,
        op: CAIRO_OPERATOR_CLEAR,
        source: 0,
        mask: 0,
        num_glyphs: 0,
        path: 0,
        fill_rule: 0,
        tolerance: 0.,
        antialias: 0,
        clip: 0,
        elapsed: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    add_record(log, record_mask(&mut record, target, op, source, mask, clip, elapsed));
    if !((*log).record).is_null() {
        status = ((*(*(*log).record).base.backend).mask)
            .expect(
                "non-null function pointer",
            )(
            &mut (*(*log).record).base as *mut cairo_surface_t as *mut libc::c_void,
            op,
            source,
            mask,
            clip,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
                784 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 167],
                    &[libc::c_char; 167],
                >(
                    b"void add_record_mask(cairo_observation_t *, cairo_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_clip_t *, cairo_time_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*log).mask.slowest.elapsed < elapsed {
        (*log).mask.slowest = record;
    }
    (*log).mask.elapsed = (*log).mask.elapsed + elapsed;
}
unsafe extern "C" fn _cairo_surface_observer_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut device: *mut cairo_device_observer_t = to_device(surface);
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut t: cairo_time_t = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let ref mut fresh26 = (*surface).log.mask.count;
    *fresh26 = (*fresh26).wrapping_add(1);
    let ref mut fresh27 = (*surface).log.mask.operators[op as usize];
    *fresh27 = (*fresh27).wrapping_add(1);
    add_pattern(&mut (*surface).log.mask.source, source, (*surface).target);
    add_pattern(&mut (*surface).log.mask.mask, mask, (*surface).target);
    add_clip(&mut (*surface).log.mask.clip, clip);
    let ref mut fresh28 = (*device).log.mask.count;
    *fresh28 = (*fresh28).wrapping_add(1);
    let ref mut fresh29 = (*device).log.mask.operators[op as usize];
    *fresh29 = (*fresh29).wrapping_add(1);
    add_pattern(&mut (*device).log.mask.source, source, (*surface).target);
    add_pattern(&mut (*device).log.mask.mask, mask, (*surface).target);
    add_clip(&mut (*device).log.mask.clip, clip);
    status = _cairo_composite_rectangles_init_for_mask(
        &mut composite,
        (*surface).target,
        op,
        source,
        mask,
        clip,
    );
    if status as u64 != 0 {
        let ref mut fresh30 = (*surface).log.mask.noop;
        *fresh30 = (*fresh30).wrapping_add(1);
        let ref mut fresh31 = (*device).log.mask.noop;
        *fresh31 = (*fresh31).wrapping_add(1);
        return status;
    }
    midpt(&mut composite, &mut x, &mut y);
    add_extents(&mut (*surface).log.mask.extents, &mut composite);
    add_extents(&mut (*device).log.mask.extents, &mut composite);
    _cairo_composite_rectangles_fini(&mut composite);
    t = _cairo_time_get();
    status = _cairo_surface_mask((*surface).target, op, source, mask, clip)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_surface_sync((*surface).target, x, y);
    t = _cairo_time_get_delta(t);
    add_record_mask(&mut (*surface).log, (*surface).target, op, source, mask, clip, t);
    add_record_mask(&mut (*device).log, (*surface).target, op, source, mask, clip, t);
    do_callbacks(surface, &mut (*surface).mask_callbacks);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn add_record_fill(
    mut log: *mut cairo_observation_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) {
    let mut record: cairo_observation_record_t = cairo_observation_record_t {
        target_content: 0 as cairo_content_t,
        target_width: 0,
        target_height: 0,
        index: 0,
        op: CAIRO_OPERATOR_CLEAR,
        source: 0,
        mask: 0,
        num_glyphs: 0,
        path: 0,
        fill_rule: 0,
        tolerance: 0.,
        antialias: 0,
        clip: 0,
        elapsed: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    add_record(
        log,
        record_fill(
            &mut record,
            target,
            op,
            source,
            path,
            fill_rule,
            tolerance,
            antialias,
            clip,
            elapsed,
        ),
    );
    if !((*log).record).is_null() {
        status = ((*(*(*log).record).base.backend).fill)
            .expect(
                "non-null function pointer",
            )(
            &mut (*(*log).record).base as *mut cairo_surface_t as *mut libc::c_void,
            op,
            source,
            path,
            fill_rule,
            tolerance,
            antialias,
            clip,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
                883 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 216],
                    &[libc::c_char; 216],
                >(
                    b"void add_record_fill(cairo_observation_t *, cairo_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_clip_t *, cairo_time_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*log).fill.slowest.elapsed < elapsed {
        (*log).fill.slowest = record;
    }
    (*log).fill.elapsed = (*log).fill.elapsed + elapsed;
}
unsafe extern "C" fn _cairo_surface_observer_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut device: *mut cairo_device_observer_t = to_device(surface);
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut t: cairo_time_t = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let ref mut fresh32 = (*surface).log.fill.count;
    *fresh32 = (*fresh32).wrapping_add(1);
    let ref mut fresh33 = (*surface).log.fill.operators[op as usize];
    *fresh33 = (*fresh33).wrapping_add(1);
    let ref mut fresh34 = (*surface).log.fill.fill_rule[fill_rule as usize];
    *fresh34 = (*fresh34).wrapping_add(1);
    let ref mut fresh35 = (*surface).log.fill.antialias[antialias as usize];
    *fresh35 = (*fresh35).wrapping_add(1);
    add_pattern(&mut (*surface).log.fill.source, source, (*surface).target);
    add_path(&mut (*surface).log.fill.path, path, 1 as libc::c_int);
    add_clip(&mut (*surface).log.fill.clip, clip);
    let ref mut fresh36 = (*device).log.fill.count;
    *fresh36 = (*fresh36).wrapping_add(1);
    let ref mut fresh37 = (*device).log.fill.operators[op as usize];
    *fresh37 = (*fresh37).wrapping_add(1);
    let ref mut fresh38 = (*device).log.fill.fill_rule[fill_rule as usize];
    *fresh38 = (*fresh38).wrapping_add(1);
    let ref mut fresh39 = (*device).log.fill.antialias[antialias as usize];
    *fresh39 = (*fresh39).wrapping_add(1);
    add_pattern(&mut (*device).log.fill.source, source, (*surface).target);
    add_path(&mut (*device).log.fill.path, path, 1 as libc::c_int);
    add_clip(&mut (*device).log.fill.clip, clip);
    status = _cairo_composite_rectangles_init_for_fill(
        &mut composite,
        (*surface).target,
        op,
        source,
        path,
        clip,
    );
    if status as u64 != 0 {
        let ref mut fresh40 = (*surface).log.fill.noop;
        *fresh40 = (*fresh40).wrapping_add(1);
        let ref mut fresh41 = (*device).log.fill.noop;
        *fresh41 = (*fresh41).wrapping_add(1);
        return status;
    }
    midpt(&mut composite, &mut x, &mut y);
    add_extents(&mut (*surface).log.fill.extents, &mut composite);
    add_extents(&mut (*device).log.fill.extents, &mut composite);
    _cairo_composite_rectangles_fini(&mut composite);
    t = _cairo_time_get();
    status = _cairo_surface_fill(
        (*surface).target,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_surface_sync((*surface).target, x, y);
    t = _cairo_time_get_delta(t);
    add_record_fill(
        &mut (*surface).log,
        (*surface).target,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
        t,
    );
    add_record_fill(
        &mut (*device).log,
        (*surface).target,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
        t,
    );
    do_callbacks(surface, &mut (*surface).fill_callbacks);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn add_record_stroke(
    mut log: *mut cairo_observation_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) {
    let mut record: cairo_observation_record_t = cairo_observation_record_t {
        target_content: 0 as cairo_content_t,
        target_width: 0,
        target_height: 0,
        index: 0,
        op: CAIRO_OPERATOR_CLEAR,
        source: 0,
        mask: 0,
        num_glyphs: 0,
        path: 0,
        fill_rule: 0,
        tolerance: 0.,
        antialias: 0,
        clip: 0,
        elapsed: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    add_record(
        log,
        record_stroke(
            &mut record,
            target,
            op,
            source,
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            antialias,
            clip,
            elapsed,
        ),
    );
    if !((*log).record).is_null() {
        status = ((*(*(*log).record).base.backend).stroke)
            .expect(
                "non-null function pointer",
            )(
            &mut (*(*log).record).base as *mut cairo_surface_t as *mut libc::c_void,
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
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
                996 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 277],
                    &[libc::c_char; 277],
                >(
                    b"void add_record_stroke(cairo_observation_t *, cairo_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *, cairo_time_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*log).stroke.slowest.elapsed < elapsed {
        (*log).stroke.slowest = record;
    }
    (*log).stroke.elapsed = (*log).stroke.elapsed + elapsed;
}
unsafe extern "C" fn _cairo_surface_observer_stroke(
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
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut device: *mut cairo_device_observer_t = to_device(surface);
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut t: cairo_time_t = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let ref mut fresh42 = (*surface).log.stroke.count;
    *fresh42 = (*fresh42).wrapping_add(1);
    let ref mut fresh43 = (*surface).log.stroke.operators[op as usize];
    *fresh43 = (*fresh43).wrapping_add(1);
    let ref mut fresh44 = (*surface).log.stroke.antialias[antialias as usize];
    *fresh44 = (*fresh44).wrapping_add(1);
    let ref mut fresh45 = (*surface).log.stroke.caps[(*style).line_cap as usize];
    *fresh45 = (*fresh45).wrapping_add(1);
    let ref mut fresh46 = (*surface).log.stroke.joins[(*style).line_join as usize];
    *fresh46 = (*fresh46).wrapping_add(1);
    add_pattern(&mut (*surface).log.stroke.source, source, (*surface).target);
    add_path(&mut (*surface).log.stroke.path, path, 0 as libc::c_int);
    add_clip(&mut (*surface).log.stroke.clip, clip);
    let ref mut fresh47 = (*device).log.stroke.count;
    *fresh47 = (*fresh47).wrapping_add(1);
    let ref mut fresh48 = (*device).log.stroke.operators[op as usize];
    *fresh48 = (*fresh48).wrapping_add(1);
    let ref mut fresh49 = (*device).log.stroke.antialias[antialias as usize];
    *fresh49 = (*fresh49).wrapping_add(1);
    let ref mut fresh50 = (*device).log.stroke.caps[(*style).line_cap as usize];
    *fresh50 = (*fresh50).wrapping_add(1);
    let ref mut fresh51 = (*device).log.stroke.joins[(*style).line_join as usize];
    *fresh51 = (*fresh51).wrapping_add(1);
    add_pattern(&mut (*device).log.stroke.source, source, (*surface).target);
    add_path(&mut (*device).log.stroke.path, path, 0 as libc::c_int);
    add_clip(&mut (*device).log.stroke.clip, clip);
    status = _cairo_composite_rectangles_init_for_stroke(
        &mut composite,
        (*surface).target,
        op,
        source,
        path,
        style,
        ctm,
        clip,
    );
    if status as u64 != 0 {
        let ref mut fresh52 = (*surface).log.stroke.noop;
        *fresh52 = (*fresh52).wrapping_add(1);
        let ref mut fresh53 = (*device).log.stroke.noop;
        *fresh53 = (*fresh53).wrapping_add(1);
        return status;
    }
    midpt(&mut composite, &mut x, &mut y);
    add_extents(&mut (*surface).log.stroke.extents, &mut composite);
    add_extents(&mut (*device).log.stroke.extents, &mut composite);
    _cairo_composite_rectangles_fini(&mut composite);
    t = _cairo_time_get();
    status = _cairo_surface_stroke(
        (*surface).target,
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
    if status as u64 != 0 {
        return status;
    }
    _cairo_surface_sync((*surface).target, x, y);
    t = _cairo_time_get_delta(t);
    add_record_stroke(
        &mut (*surface).log,
        (*surface).target,
        op,
        source,
        path,
        style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        clip,
        t,
    );
    add_record_stroke(
        &mut (*device).log,
        (*surface).target,
        op,
        source,
        path,
        style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        clip,
        t,
    );
    do_callbacks(surface, &mut (*surface).stroke_callbacks);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn add_record_glyphs(
    mut log: *mut cairo_observation_t,
    mut target: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
    mut elapsed: cairo_time_t,
) {
    let mut record: cairo_observation_record_t = cairo_observation_record_t {
        target_content: 0 as cairo_content_t,
        target_width: 0,
        target_height: 0,
        index: 0,
        op: CAIRO_OPERATOR_CLEAR,
        source: 0,
        mask: 0,
        num_glyphs: 0,
        path: 0,
        fill_rule: 0,
        tolerance: 0.,
        antialias: 0,
        clip: 0,
        elapsed: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    add_record(
        log,
        record_glyphs(
            &mut record,
            target,
            op,
            source,
            glyphs,
            num_glyphs,
            scaled_font,
            clip,
            elapsed,
        ),
    );
    if !((*log).record).is_null() {
        status = ((*(*(*log).record).base.backend).show_text_glyphs)
            .expect(
                "non-null function pointer",
            )(
            &mut (*(*log).record).base as *mut cairo_surface_t as *mut libc::c_void,
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
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
                1115 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 189],
                    &[libc::c_char; 189],
                >(
                    b"void add_record_glyphs(cairo_observation_t *, cairo_surface_t *, cairo_operator_t, const cairo_pattern_t *, cairo_glyph_t *, int, cairo_scaled_font_t *, const cairo_clip_t *, cairo_time_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*log).glyphs.slowest.elapsed < elapsed {
        (*log).glyphs.slowest = record;
    }
    (*log).glyphs.elapsed = (*log).glyphs.elapsed + elapsed;
}
unsafe extern "C" fn _cairo_surface_observer_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut device: *mut cairo_device_observer_t = to_device(surface);
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut dev_glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    let mut t: cairo_time_t = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let ref mut fresh54 = (*surface).log.glyphs.count;
    *fresh54 = (*fresh54).wrapping_add(1);
    let ref mut fresh55 = (*surface).log.glyphs.operators[op as usize];
    *fresh55 = (*fresh55).wrapping_add(1);
    add_pattern(&mut (*surface).log.glyphs.source, source, (*surface).target);
    add_clip(&mut (*surface).log.glyphs.clip, clip);
    let ref mut fresh56 = (*device).log.glyphs.count;
    *fresh56 = (*fresh56).wrapping_add(1);
    let ref mut fresh57 = (*device).log.glyphs.operators[op as usize];
    *fresh57 = (*fresh57).wrapping_add(1);
    add_pattern(&mut (*device).log.glyphs.source, source, (*surface).target);
    add_clip(&mut (*device).log.glyphs.clip, clip);
    status = _cairo_composite_rectangles_init_for_glyphs(
        &mut composite,
        (*surface).target,
        op,
        source,
        scaled_font,
        glyphs,
        num_glyphs,
        clip,
        0 as *mut cairo_bool_t,
    );
    if status as u64 != 0 {
        let ref mut fresh58 = (*surface).log.glyphs.noop;
        *fresh58 = (*fresh58).wrapping_add(1);
        let ref mut fresh59 = (*device).log.glyphs.noop;
        *fresh59 = (*fresh59).wrapping_add(1);
        return status;
    }
    midpt(&mut composite, &mut x, &mut y);
    add_extents(&mut (*surface).log.glyphs.extents, &mut composite);
    add_extents(&mut (*device).log.glyphs.extents, &mut composite);
    _cairo_composite_rectangles_fini(&mut composite);
    dev_glyphs = _cairo_malloc_ab(
        num_glyphs as size_t,
        ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
    ) as *mut cairo_glyph_t;
    if dev_glyphs.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    memcpy(
        dev_glyphs as *mut libc::c_void,
        glyphs as *const libc::c_void,
        (num_glyphs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong),
    );
    t = _cairo_time_get();
    status = _cairo_surface_show_text_glyphs(
        (*surface).target,
        op,
        source,
        0 as *const libc::c_char,
        0 as libc::c_int,
        dev_glyphs,
        num_glyphs,
        0 as *const cairo_text_cluster_t,
        0 as libc::c_int,
        0 as cairo_text_cluster_flags_t,
        scaled_font,
        clip,
    ) as cairo_int_status_t;
    free(dev_glyphs as *mut libc::c_void);
    if status as u64 != 0 {
        return status;
    }
    _cairo_surface_sync((*surface).target, x, y);
    t = _cairo_time_get_delta(t);
    add_record_glyphs(
        &mut (*surface).log,
        (*surface).target,
        op,
        source,
        glyphs,
        num_glyphs,
        scaled_font,
        clip,
        t,
    );
    add_record_glyphs(
        &mut (*device).log,
        (*surface).target,
        op,
        source,
        glyphs,
        num_glyphs,
        scaled_font,
        clip,
        t,
    );
    do_callbacks(surface, &mut (*surface).glyphs_callbacks);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_observer_flush(
    mut abstract_surface: *mut libc::c_void,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    do_callbacks(surface, &mut (*surface).flush_callbacks);
    return _cairo_surface_flush((*surface).target, flags);
}
unsafe extern "C" fn _cairo_surface_observer_mark_dirty(
    mut abstract_surface: *mut libc::c_void,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = CAIRO_STATUS_SUCCESS;
    if ((*(*(*surface).target).backend).mark_dirty_rectangle).is_some() {
        status = ((*(*(*surface).target).backend).mark_dirty_rectangle)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, x, y, width, height);
    }
    return status;
}
unsafe extern "C" fn _cairo_surface_observer_copy_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = CAIRO_STATUS_SUCCESS;
    if ((*(*(*surface).target).backend).copy_page).is_some() {
        status = ((*(*(*surface).target).backend).copy_page)
            .expect("non-null function pointer")((*surface).target as *mut libc::c_void)
            as cairo_status_t;
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_observer_show_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = CAIRO_STATUS_SUCCESS;
    if ((*(*(*surface).target).backend).show_page).is_some() {
        status = ((*(*(*surface).target).backend).show_page)
            .expect("non-null function pointer")((*surface).target as *mut libc::c_void)
            as cairo_status_t;
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_observer_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    return _cairo_surface_get_extents((*surface).target, extents);
}
unsafe extern "C" fn _cairo_surface_observer_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    if ((*(*(*surface).target).backend).get_font_options).is_some() {
        ((*(*(*surface).target).backend).get_font_options)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, options);
    }
}
unsafe extern "C" fn _cairo_surface_observer_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    return _cairo_surface_get_source((*surface).target, extents);
}
unsafe extern "C" fn _cairo_surface_observer_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    let ref mut fresh60 = (*surface).log.num_sources_acquired;
    *fresh60 += 1;
    let ref mut fresh61 = (*to_device(surface)).log.num_sources_acquired;
    *fresh61 += 1;
    return _cairo_surface_acquire_source_image(
        (*surface).target,
        image_out,
        image_extra,
    );
}
unsafe extern "C" fn _cairo_surface_observer_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    _cairo_surface_release_source_image((*surface).target, image, image_extra);
}
unsafe extern "C" fn _cairo_surface_observer_snapshot(
    mut abstract_surface: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_observer_t = abstract_surface
        as *mut cairo_surface_observer_t;
    if ((*(*(*surface).target).backend).snapshot).is_some() {
        return ((*(*(*surface).target).backend).snapshot)
            .expect("non-null function pointer")((*surface).target as *mut libc::c_void);
    }
    return 0 as *mut cairo_surface_t;
}
unsafe extern "C" fn _cairo_surface_observer_create_context(
    mut target: *mut libc::c_void,
) -> *mut cairo_t {
    let mut surface: *mut cairo_surface_observer_t = target
        as *mut cairo_surface_observer_t;
    if _cairo_surface_is_subsurface(&mut (*surface).base) != 0 {
        surface = _cairo_surface_subsurface_get_target(&mut (*surface).base)
            as *mut cairo_surface_observer_t;
    }
    let ref mut fresh62 = (*surface).log.num_contexts;
    *fresh62 += 1;
    let ref mut fresh63 = (*to_device(surface)).log.num_contexts;
    *fresh63 += 1;
    return ((*(*(*surface).target).backend).create_context)
        .expect("non-null function pointer")(target);
}
static mut _cairo_surface_observer_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_OBSERVER as libc::c_int
                as cairo_surface_type_t,
            finish: Some(
                _cairo_surface_observer_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_surface_observer_create_context
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_surface_observer_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: Some(
                _cairo_surface_observer_create_similar_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_format_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            map_to_image: Some(
                _cairo_surface_observer_map_to_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_rectangle_int_t,
                    ) -> *mut cairo_image_surface_t,
            ),
            unmap_image: Some(
                _cairo_surface_observer_unmap_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
            source: Some(
                _cairo_surface_observer_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_surface_observer_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_surface_observer_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_surface_observer_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: Some(
                _cairo_surface_observer_copy_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            show_page: Some(
                _cairo_surface_observer_show_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            get_extents: Some(
                _cairo_surface_observer_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_surface_observer_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: Some(
                _cairo_surface_observer_flush
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> cairo_status_t,
            ),
            mark_dirty_rectangle: Some(
                _cairo_surface_observer_mark_dirty
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> cairo_status_t,
            ),
            paint: Some(
                _cairo_surface_observer_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_surface_observer_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_surface_observer_stroke
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
                _cairo_surface_observer_fill
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
                _cairo_surface_observer_glyphs
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
pub unsafe extern "C" fn cairo_surface_create_observer(
    mut target: *mut cairo_surface_t,
    mut mode: cairo_surface_observer_mode_t,
) -> *mut cairo_surface_t {
    let mut device: *mut cairo_device_t = 0 as *mut cairo_device_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut record: cairo_bool_t = 0;
    if (*target).status as u64 != 0 {
        return _cairo_surface_create_in_error((*target).status);
    }
    if (*target).finished() != 0 {
        return _cairo_surface_create_in_error(
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED),
        );
    }
    record = (mode as libc::c_uint
        & CAIRO_SURFACE_OBSERVER_RECORD_OPERATIONS as libc::c_int as libc::c_uint)
        as cairo_bool_t;
    device = _cairo_device_create_observer_internal((*target).device, record);
    if (*device).status as u64 != 0 {
        return _cairo_surface_create_in_error((*device).status);
    }
    surface = _cairo_surface_create_observer_internal(device, target);
    cairo_device_destroy(device);
    return surface;
}
unsafe extern "C" fn _cairo_surface_observer_add_callback(
    mut head: *mut cairo_list_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut cb: *mut callback_list = 0 as *mut callback_list;
    cb = (if ::std::mem::size_of::<callback_list>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<callback_list>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut callback_list;
    if cb.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    cairo_list_add(&mut (*cb).link, head);
    let ref mut fresh64 = (*cb).func;
    *fresh64 = func;
    let ref mut fresh65 = (*cb).data;
    *fresh65 = data;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_paint_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).paint_callbacks,
        func,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_mask_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).mask_callbacks,
        func,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_fill_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).fill_callbacks,
        func,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_stroke_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).stroke_callbacks,
        func,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_glyphs_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).glyphs_callbacks,
        func,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_flush_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).flush_callbacks,
        func,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_add_finish_callback(
    mut abstract_surface: *mut cairo_surface_t,
    mut func: cairo_surface_observer_callback_t,
    mut data: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_surface_observer_add_callback(
        &mut (*surface).finish_callbacks,
        func,
        data,
    );
}
unsafe extern "C" fn print_extents(
    mut stream: *mut cairo_output_stream_t,
    mut e: *const extents,
) {
    _cairo_output_stream_printf(
        stream,
        b"  extents: total %g, avg %g [unbounded %d]\n\0" as *const u8
            as *const libc::c_char,
        (*e).area.sum,
        (*e).area.sum / (*e).area.count as libc::c_double,
        (*e).unbounded,
    );
}
#[inline]
unsafe extern "C" fn ordercmp(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut array: *const libc::c_uint,
) -> libc::c_int {
    return (*array.offset(b as isize)).wrapping_sub(*array.offset(a as isize))
        as libc::c_int;
}
unsafe extern "C" fn sort_order(
    mut base: *mut libc::c_int,
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
            if ordercmp(
                *base.offset(i as isize),
                *base.offset(j as isize),
                data as *const libc::c_uint,
            ) > 0 as libc::c_int
            {
                let mut tmp: libc::c_int = 0;
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
unsafe extern "C" fn print_array(
    mut stream: *mut cairo_output_stream_t,
    mut array: *const libc::c_uint,
    mut names: *mut *const libc::c_char,
    mut count: libc::c_int,
) {
    let mut order: [libc::c_int; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if count
        < (::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int
    {} else {
        __assert_fail(
            b"count < ARRAY_LENGTH (order)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
            1584 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void print_array(cairo_output_stream_t *, const unsigned int *, const char **, int)\0",
            ))
                .as_ptr(),
        );
    }
    j = 0 as libc::c_int;
    i = j;
    while i < count {
        if *array.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            let fresh66 = j;
            j = j + 1;
            order[fresh66 as usize] = i;
        }
        i += 1;
    }
    sort_order(order.as_mut_ptr(), j as libc::c_uint, array as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < j {
        _cairo_output_stream_printf(
            stream,
            b" %d %s%s\0" as *const u8 as *const libc::c_char,
            *array.offset(order[i as usize] as isize),
            *names.offset(order[i as usize] as isize),
            if i < j - 1 as libc::c_int {
                b",\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
    }
}
static mut operator_names: [*const libc::c_char; 29] = [
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
unsafe extern "C" fn print_operators(
    mut stream: *mut cairo_output_stream_t,
    mut array: *mut libc::c_uint,
) {
    _cairo_output_stream_printf(stream, b"  op:\0" as *const u8 as *const libc::c_char);
    print_array(
        stream,
        array,
        operator_names.as_mut_ptr(),
        CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int + 1 as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut fill_rule_names: [*const libc::c_char; 2] = [
    b"non-zero\0" as *const u8 as *const libc::c_char,
    b"even-odd\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_fill_rule(
    mut stream: *mut cairo_output_stream_t,
    mut array: *mut libc::c_uint,
) {
    _cairo_output_stream_printf(
        stream,
        b"  fill rule:\0" as *const u8 as *const libc::c_char,
    );
    print_array(
        stream,
        array,
        fill_rule_names.as_mut_ptr(),
        (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut cap_names: [*const libc::c_char; 3] = [
    b"butt\0" as *const u8 as *const libc::c_char,
    b"round\0" as *const u8 as *const libc::c_char,
    b"square\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_line_caps(
    mut stream: *mut cairo_output_stream_t,
    mut array: *mut libc::c_uint,
) {
    _cairo_output_stream_printf(
        stream,
        b"  caps:\0" as *const u8 as *const libc::c_char,
    );
    print_array(
        stream,
        array,
        cap_names.as_mut_ptr(),
        CAIRO_LINE_CAP_SQUARE as libc::c_int + 1 as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut join_names: [*const libc::c_char; 3] = [
    b"miter\0" as *const u8 as *const libc::c_char,
    b"round\0" as *const u8 as *const libc::c_char,
    b"bevel\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_line_joins(
    mut stream: *mut cairo_output_stream_t,
    mut array: *mut libc::c_uint,
) {
    _cairo_output_stream_printf(
        stream,
        b"  joins:\0" as *const u8 as *const libc::c_char,
    );
    print_array(
        stream,
        array,
        join_names.as_mut_ptr(),
        CAIRO_LINE_JOIN_BEVEL as libc::c_int + 1 as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut antialias_names: [*const libc::c_char; 7] = [
    b"default\0" as *const u8 as *const libc::c_char,
    b"none\0" as *const u8 as *const libc::c_char,
    b"gray\0" as *const u8 as *const libc::c_char,
    b"subpixel\0" as *const u8 as *const libc::c_char,
    b"fast\0" as *const u8 as *const libc::c_char,
    b"good\0" as *const u8 as *const libc::c_char,
    b"best\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_antialias(
    mut stream: *mut cairo_output_stream_t,
    mut array: *mut libc::c_uint,
) {
    _cairo_output_stream_printf(
        stream,
        b"  antialias:\0" as *const u8 as *const libc::c_char,
    );
    print_array(
        stream,
        array,
        antialias_names.as_mut_ptr(),
        CAIRO_ANTIALIAS_BEST as libc::c_int + 1 as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut pattern_names: [*const libc::c_char; 8] = [
    b"native\0" as *const u8 as *const libc::c_char,
    b"record\0" as *const u8 as *const libc::c_char,
    b"other surface\0" as *const u8 as *const libc::c_char,
    b"solid\0" as *const u8 as *const libc::c_char,
    b"linear\0" as *const u8 as *const libc::c_char,
    b"radial\0" as *const u8 as *const libc::c_char,
    b"mesh\0" as *const u8 as *const libc::c_char,
    b"raster\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_pattern(
    mut stream: *mut cairo_output_stream_t,
    mut name: *const libc::c_char,
    mut p: *const pattern,
) {
    _cairo_output_stream_printf(
        stream,
        b"  %s:\0" as *const u8 as *const libc::c_char,
        name,
    );
    print_array(
        stream,
        ((*p).type_0).as_ptr(),
        pattern_names.as_mut_ptr(),
        (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut path_names: [*const libc::c_char; 5] = [
    b"empty\0" as *const u8 as *const libc::c_char,
    b"pixel-aligned\0" as *const u8 as *const libc::c_char,
    b"rectliinear\0" as *const u8 as *const libc::c_char,
    b"straight\0" as *const u8 as *const libc::c_char,
    b"curved\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_path(
    mut stream: *mut cairo_output_stream_t,
    mut p: *const path,
) {
    _cairo_output_stream_printf(
        stream,
        b"  path:\0" as *const u8 as *const libc::c_char,
    );
    print_array(
        stream,
        ((*p).type_0).as_ptr(),
        path_names.as_mut_ptr(),
        (::std::mem::size_of::<[*const libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut clip_names: [*const libc::c_char; 6] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"region\0" as *const u8 as *const libc::c_char,
    b"boxes\0" as *const u8 as *const libc::c_char,
    b"single path\0" as *const u8 as *const libc::c_char,
    b"polygon\0" as *const u8 as *const libc::c_char,
    b"general\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn print_clip(
    mut stream: *mut cairo_output_stream_t,
    mut c: *const clip,
) {
    _cairo_output_stream_printf(
        stream,
        b"  clip:\0" as *const u8 as *const libc::c_char,
    );
    print_array(
        stream,
        ((*c).type_0).as_ptr(),
        clip_names.as_mut_ptr(),
        (::std::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_record(
    mut stream: *mut cairo_output_stream_t,
    mut r: *mut cairo_observation_record_t,
) {
    _cairo_output_stream_printf(
        stream,
        b"  op: %s\n\0" as *const u8 as *const libc::c_char,
        operator_names[(*r).op as usize],
    );
    _cairo_output_stream_printf(
        stream,
        b"  source: %s\n\0" as *const u8 as *const libc::c_char,
        pattern_names[(*r).source as usize],
    );
    if (*r).mask != -(1 as libc::c_int) {
        _cairo_output_stream_printf(
            stream,
            b"  mask: %s\n\0" as *const u8 as *const libc::c_char,
            pattern_names[(*r).mask as usize],
        );
    }
    if (*r).num_glyphs != -(1 as libc::c_int) {
        _cairo_output_stream_printf(
            stream,
            b"  num_glyphs: %d\n\0" as *const u8 as *const libc::c_char,
            (*r).num_glyphs,
        );
    }
    if (*r).path != -(1 as libc::c_int) {
        _cairo_output_stream_printf(
            stream,
            b"  path: %s\n\0" as *const u8 as *const libc::c_char,
            path_names[(*r).path as usize],
        );
    }
    if (*r).fill_rule != -(1 as libc::c_int) {
        _cairo_output_stream_printf(
            stream,
            b"  fill rule: %s\n\0" as *const u8 as *const libc::c_char,
            fill_rule_names[(*r).fill_rule as usize],
        );
    }
    if (*r).antialias != -(1 as libc::c_int) {
        _cairo_output_stream_printf(
            stream,
            b"  antialias: %s\n\0" as *const u8 as *const libc::c_char,
            antialias_names[(*r).antialias as usize],
        );
    }
    _cairo_output_stream_printf(
        stream,
        b"  clip: %s\n\0" as *const u8 as *const libc::c_char,
        clip_names[(*r).clip as usize],
    );
    _cairo_output_stream_printf(
        stream,
        b"  elapsed: %f ns\n\0" as *const u8 as *const libc::c_char,
        _cairo_time_to_ns((*r).elapsed),
    );
}
unsafe extern "C" fn percent(
    mut a: cairo_time_t,
    mut b: cairo_time_t,
) -> libc::c_double {
    return _cairo_round(
        _cairo_time_to_s(a) * 1000 as libc::c_int as libc::c_double / _cairo_time_to_s(b),
    ) / 10 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn replay_record(
    mut log: *mut cairo_observation_t,
    mut r: *mut cairo_observation_record_t,
    mut script: *mut cairo_device_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if ((*log).record).is_null() || script.is_null() {
        return 0 as libc::c_int;
    }
    surface = cairo_script_surface_create(
        script,
        (*r).target_content,
        (*r).target_width as libc::c_double,
        (*r).target_height as libc::c_double,
    );
    status = _cairo_recording_surface_replay_one(
        (*log).record,
        (*r).index as libc::c_ulong,
        surface,
    ) as cairo_int_status_t;
    cairo_surface_destroy(surface);
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface-observer.c\0" as *const u8 as *const libc::c_char,
            1801 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"cairo_bool_t replay_record(cairo_observation_t *, cairo_observation_record_t *, cairo_device_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_observation_total_elapsed(
    mut log: *mut cairo_observation_t,
) -> cairo_time_t {
    let mut total: cairo_time_t = 0;
    total = (*log).paint.elapsed;
    total = total + (*log).mask.elapsed;
    total = total + (*log).fill.elapsed;
    total = total + (*log).stroke.elapsed;
    total = total + (*log).glyphs.elapsed;
    return total;
}
unsafe extern "C" fn _cairo_observation_print(
    mut stream: *mut cairo_output_stream_t,
    mut log: *mut cairo_observation_t,
) {
    let mut script: *mut cairo_device_t = 0 as *mut cairo_device_t;
    let mut total: cairo_time_t = 0;
    script = _cairo_script_context_create_internal(stream);
    _cairo_script_context_attach_snapshots(script, 0 as libc::c_int);
    total = _cairo_observation_total_elapsed(log);
    _cairo_output_stream_printf(
        stream,
        b"elapsed: %f\n\0" as *const u8 as *const libc::c_char,
        _cairo_time_to_ns(total),
    );
    _cairo_output_stream_printf(
        stream,
        b"surfaces: %d\n\0" as *const u8 as *const libc::c_char,
        (*log).num_surfaces,
    );
    _cairo_output_stream_printf(
        stream,
        b"contexts: %d\n\0" as *const u8 as *const libc::c_char,
        (*log).num_contexts,
    );
    _cairo_output_stream_printf(
        stream,
        b"sources acquired: %d\n\0" as *const u8 as *const libc::c_char,
        (*log).num_sources_acquired,
    );
    _cairo_output_stream_printf(
        stream,
        b"paint: count %d [no-op %d], elapsed %f [%f%%]\n\0" as *const u8
            as *const libc::c_char,
        (*log).paint.count,
        (*log).paint.noop,
        _cairo_time_to_ns((*log).paint.elapsed),
        percent((*log).paint.elapsed, total),
    );
    if (*log).paint.count != 0 {
        print_extents(stream, &mut (*log).paint.extents);
        print_operators(stream, ((*log).paint.operators).as_mut_ptr());
        print_pattern(
            stream,
            b"source\0" as *const u8 as *const libc::c_char,
            &mut (*log).paint.source,
        );
        print_clip(stream, &mut (*log).paint.clip);
        _cairo_output_stream_printf(
            stream,
            b"slowest paint: %f%%\n\0" as *const u8 as *const libc::c_char,
            percent((*log).paint.slowest.elapsed, (*log).paint.elapsed),
        );
        print_record(stream, &mut (*log).paint.slowest);
        _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
        if replay_record(log, &mut (*log).paint.slowest, script) != 0 {
            _cairo_output_stream_printf(
                stream,
                b"\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    _cairo_output_stream_printf(
        stream,
        b"mask: count %d [no-op %d], elapsed %f [%f%%]\n\0" as *const u8
            as *const libc::c_char,
        (*log).mask.count,
        (*log).mask.noop,
        _cairo_time_to_ns((*log).mask.elapsed),
        percent((*log).mask.elapsed, total),
    );
    if (*log).mask.count != 0 {
        print_extents(stream, &mut (*log).mask.extents);
        print_operators(stream, ((*log).mask.operators).as_mut_ptr());
        print_pattern(
            stream,
            b"source\0" as *const u8 as *const libc::c_char,
            &mut (*log).mask.source,
        );
        print_pattern(
            stream,
            b"mask\0" as *const u8 as *const libc::c_char,
            &mut (*log).mask.mask,
        );
        print_clip(stream, &mut (*log).mask.clip);
        _cairo_output_stream_printf(
            stream,
            b"slowest mask: %f%%\n\0" as *const u8 as *const libc::c_char,
            percent((*log).mask.slowest.elapsed, (*log).mask.elapsed),
        );
        print_record(stream, &mut (*log).mask.slowest);
        _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
        if replay_record(log, &mut (*log).mask.slowest, script) != 0 {
            _cairo_output_stream_printf(
                stream,
                b"\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    _cairo_output_stream_printf(
        stream,
        b"fill: count %d [no-op %d], elaspsed %f [%f%%]\n\0" as *const u8
            as *const libc::c_char,
        (*log).fill.count,
        (*log).fill.noop,
        _cairo_time_to_ns((*log).fill.elapsed),
        percent((*log).fill.elapsed, total),
    );
    if (*log).fill.count != 0 {
        print_extents(stream, &mut (*log).fill.extents);
        print_operators(stream, ((*log).fill.operators).as_mut_ptr());
        print_pattern(
            stream,
            b"source\0" as *const u8 as *const libc::c_char,
            &mut (*log).fill.source,
        );
        print_path(stream, &mut (*log).fill.path);
        print_fill_rule(stream, ((*log).fill.fill_rule).as_mut_ptr());
        print_antialias(stream, ((*log).fill.antialias).as_mut_ptr());
        print_clip(stream, &mut (*log).fill.clip);
        _cairo_output_stream_printf(
            stream,
            b"slowest fill: %f%%\n\0" as *const u8 as *const libc::c_char,
            percent((*log).fill.slowest.elapsed, (*log).fill.elapsed),
        );
        print_record(stream, &mut (*log).fill.slowest);
        _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
        if replay_record(log, &mut (*log).fill.slowest, script) != 0 {
            _cairo_output_stream_printf(
                stream,
                b"\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    _cairo_output_stream_printf(
        stream,
        b"stroke: count %d [no-op %d], elapsed %f [%f%%]\n\0" as *const u8
            as *const libc::c_char,
        (*log).stroke.count,
        (*log).stroke.noop,
        _cairo_time_to_ns((*log).stroke.elapsed),
        percent((*log).stroke.elapsed, total),
    );
    if (*log).stroke.count != 0 {
        print_extents(stream, &mut (*log).stroke.extents);
        print_operators(stream, ((*log).stroke.operators).as_mut_ptr());
        print_pattern(
            stream,
            b"source\0" as *const u8 as *const libc::c_char,
            &mut (*log).stroke.source,
        );
        print_path(stream, &mut (*log).stroke.path);
        print_antialias(stream, ((*log).stroke.antialias).as_mut_ptr());
        print_line_caps(stream, ((*log).stroke.caps).as_mut_ptr());
        print_line_joins(stream, ((*log).stroke.joins).as_mut_ptr());
        print_clip(stream, &mut (*log).stroke.clip);
        _cairo_output_stream_printf(
            stream,
            b"slowest stroke: %f%%\n\0" as *const u8 as *const libc::c_char,
            percent((*log).stroke.slowest.elapsed, (*log).stroke.elapsed),
        );
        print_record(stream, &mut (*log).stroke.slowest);
        _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
        if replay_record(log, &mut (*log).stroke.slowest, script) != 0 {
            _cairo_output_stream_printf(
                stream,
                b"\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    _cairo_output_stream_printf(
        stream,
        b"glyphs: count %d [no-op %d], elasped %f [%f%%]\n\0" as *const u8
            as *const libc::c_char,
        (*log).glyphs.count,
        (*log).glyphs.noop,
        _cairo_time_to_ns((*log).glyphs.elapsed),
        percent((*log).glyphs.elapsed, total),
    );
    if (*log).glyphs.count != 0 {
        print_extents(stream, &mut (*log).glyphs.extents);
        print_operators(stream, ((*log).glyphs.operators).as_mut_ptr());
        print_pattern(
            stream,
            b"source\0" as *const u8 as *const libc::c_char,
            &mut (*log).glyphs.source,
        );
        print_clip(stream, &mut (*log).glyphs.clip);
        _cairo_output_stream_printf(
            stream,
            b"slowest glyphs: %f%%\n\0" as *const u8 as *const libc::c_char,
            percent((*log).glyphs.slowest.elapsed, (*log).glyphs.elapsed),
        );
        print_record(stream, &mut (*log).glyphs.slowest);
        _cairo_output_stream_printf(stream, b"\n\0" as *const u8 as *const libc::c_char);
        if replay_record(log, &mut (*log).glyphs.slowest, script) != 0 {
            _cairo_output_stream_printf(
                stream,
                b"\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    cairo_device_destroy(script);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_print(
    mut abstract_surface: *mut cairo_surface_t,
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if (*abstract_surface).status as u64 != 0 {
        return (*abstract_surface).status;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH);
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    stream = _cairo_output_stream_create(write_func, None, closure);
    _cairo_observation_print(stream, &mut (*surface).log);
    return _cairo_output_stream_destroy(stream);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_observer_elapsed(
    mut abstract_surface: *mut cairo_surface_t,
) -> libc::c_double {
    let mut surface: *mut cairo_surface_observer_t = 0 as *mut cairo_surface_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_surface).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_surface_is_observer(abstract_surface) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    surface = abstract_surface as *mut cairo_surface_observer_t;
    return _cairo_time_to_ns(_cairo_observation_total_elapsed(&mut (*surface).log));
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_print(
    mut abstract_device: *mut cairo_device_t,
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if (*abstract_device).status as u64 != 0 {
        return (*abstract_device).status;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return _cairo_error(CAIRO_STATUS_DEVICE_TYPE_MISMATCH);
    }
    device = abstract_device as *mut cairo_device_observer_t;
    stream = _cairo_output_stream_create(write_func, None, closure);
    _cairo_observation_print(stream, &mut (*device).log);
    return _cairo_output_stream_destroy(stream);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_elapsed(
    mut abstract_device: *mut cairo_device_t,
) -> libc::c_double {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_device).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    device = abstract_device as *mut cairo_device_observer_t;
    return _cairo_time_to_ns(_cairo_observation_total_elapsed(&mut (*device).log));
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_paint_elapsed(
    mut abstract_device: *mut cairo_device_t,
) -> libc::c_double {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_device).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    device = abstract_device as *mut cairo_device_observer_t;
    return _cairo_time_to_ns((*device).log.paint.elapsed);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_mask_elapsed(
    mut abstract_device: *mut cairo_device_t,
) -> libc::c_double {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_device).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    device = abstract_device as *mut cairo_device_observer_t;
    return _cairo_time_to_ns((*device).log.mask.elapsed);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_fill_elapsed(
    mut abstract_device: *mut cairo_device_t,
) -> libc::c_double {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_device).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    device = abstract_device as *mut cairo_device_observer_t;
    return _cairo_time_to_ns((*device).log.fill.elapsed);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_stroke_elapsed(
    mut abstract_device: *mut cairo_device_t,
) -> libc::c_double {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_device).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    device = abstract_device as *mut cairo_device_observer_t;
    return _cairo_time_to_ns((*device).log.stroke.elapsed);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_observer_glyphs_elapsed(
    mut abstract_device: *mut cairo_device_t,
) -> libc::c_double {
    let mut device: *mut cairo_device_observer_t = 0 as *mut cairo_device_observer_t;
    if _cairo_atomic_int_get(&mut (*abstract_device).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    if _cairo_device_is_observer(abstract_device) == 0 {
        return -(1 as libc::c_int) as libc::c_double;
    }
    device = abstract_device as *mut cairo_device_observer_t;
    return _cairo_time_to_ns((*device).log.glyphs.elapsed);
}
