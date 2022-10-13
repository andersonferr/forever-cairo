use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t,
        hint_metrics: cairo_hint_metrics_t,
    );
    fn cairo_surface_create_similar_image(
        other: *mut cairo_surface_t,
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn pixman_image_unref(image: *mut pixman_image_t) -> pixman_bool_t;
    fn pixman_image_get_depth(image: *mut pixman_image_t) -> libc::c_int;
    fn pixman_image_get_stride(image: *mut pixman_image_t) -> libc::c_int;
    fn pixman_image_get_height(image: *mut pixman_image_t) -> libc::c_int;
    fn pixman_image_get_width(image: *mut pixman_image_t) -> libc::c_int;
    fn pixman_image_get_data(image: *mut pixman_image_t) -> *mut uint32_t;
    fn _cairo_font_options_set_round_glyph_positions(
        options: *mut cairo_font_options_t,
        round: cairo_round_glyph_positions_t,
    );
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn pixman_image_composite32(
        op: pixman_op_t,
        src: *mut pixman_image_t,
        mask: *mut pixman_image_t,
        dest: *mut pixman_image_t,
        src_x: int32_t,
        src_y: int32_t,
        mask_x: int32_t,
        mask_y: int32_t,
        dest_x: int32_t,
        dest_y: int32_t,
        width: int32_t,
        height: int32_t,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn pixman_image_create_bits(
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        bits: *mut uint32_t,
        rowstride_bytes: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_format_supported_destination(
        format: pixman_format_code_t,
    ) -> pixman_bool_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
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
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_image_spans_compositor_get() -> *const cairo_compositor_t;
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
pub type pixman_bool_t = libc::c_int;
pub const CAIRO_IMAGE_UNKNOWN_COLOR: _cairo_image_color = 3;
pub const CAIRO_IMAGE_UNKNOWN: _cairo_image_transparency = 3;
pub type uint8_t = __uint8_t;
pub type pixman_op_t = libc::c_uint;
pub const PIXMAN_OP_HSL_LUMINOSITY: pixman_op_t = 62;
pub const PIXMAN_OP_HSL_COLOR: pixman_op_t = 61;
pub const PIXMAN_OP_HSL_SATURATION: pixman_op_t = 60;
pub const PIXMAN_OP_HSL_HUE: pixman_op_t = 59;
pub const PIXMAN_OP_EXCLUSION: pixman_op_t = 58;
pub const PIXMAN_OP_DIFFERENCE: pixman_op_t = 57;
pub const PIXMAN_OP_SOFT_LIGHT: pixman_op_t = 56;
pub const PIXMAN_OP_HARD_LIGHT: pixman_op_t = 55;
pub const PIXMAN_OP_COLOR_BURN: pixman_op_t = 54;
pub const PIXMAN_OP_COLOR_DODGE: pixman_op_t = 53;
pub const PIXMAN_OP_LIGHTEN: pixman_op_t = 52;
pub const PIXMAN_OP_DARKEN: pixman_op_t = 51;
pub const PIXMAN_OP_OVERLAY: pixman_op_t = 50;
pub const PIXMAN_OP_SCREEN: pixman_op_t = 49;
pub const PIXMAN_OP_MULTIPLY: pixman_op_t = 48;
pub const PIXMAN_OP_CONJOINT_XOR: pixman_op_t = 43;
pub const PIXMAN_OP_CONJOINT_ATOP_REVERSE: pixman_op_t = 42;
pub const PIXMAN_OP_CONJOINT_ATOP: pixman_op_t = 41;
pub const PIXMAN_OP_CONJOINT_OUT_REVERSE: pixman_op_t = 40;
pub const PIXMAN_OP_CONJOINT_OUT: pixman_op_t = 39;
pub const PIXMAN_OP_CONJOINT_IN_REVERSE: pixman_op_t = 38;
pub const PIXMAN_OP_CONJOINT_IN: pixman_op_t = 37;
pub const PIXMAN_OP_CONJOINT_OVER_REVERSE: pixman_op_t = 36;
pub const PIXMAN_OP_CONJOINT_OVER: pixman_op_t = 35;
pub const PIXMAN_OP_CONJOINT_DST: pixman_op_t = 34;
pub const PIXMAN_OP_CONJOINT_SRC: pixman_op_t = 33;
pub const PIXMAN_OP_CONJOINT_CLEAR: pixman_op_t = 32;
pub const PIXMAN_OP_DISJOINT_XOR: pixman_op_t = 27;
pub const PIXMAN_OP_DISJOINT_ATOP_REVERSE: pixman_op_t = 26;
pub const PIXMAN_OP_DISJOINT_ATOP: pixman_op_t = 25;
pub const PIXMAN_OP_DISJOINT_OUT_REVERSE: pixman_op_t = 24;
pub const PIXMAN_OP_DISJOINT_OUT: pixman_op_t = 23;
pub const PIXMAN_OP_DISJOINT_IN_REVERSE: pixman_op_t = 22;
pub const PIXMAN_OP_DISJOINT_IN: pixman_op_t = 21;
pub const PIXMAN_OP_DISJOINT_OVER_REVERSE: pixman_op_t = 20;
pub const PIXMAN_OP_DISJOINT_OVER: pixman_op_t = 19;
pub const PIXMAN_OP_DISJOINT_DST: pixman_op_t = 18;
pub const PIXMAN_OP_DISJOINT_SRC: pixman_op_t = 17;
pub const PIXMAN_OP_DISJOINT_CLEAR: pixman_op_t = 16;
pub const PIXMAN_OP_SATURATE: pixman_op_t = 13;
pub const PIXMAN_OP_ADD: pixman_op_t = 12;
pub const PIXMAN_OP_XOR: pixman_op_t = 11;
pub const PIXMAN_OP_ATOP_REVERSE: pixman_op_t = 10;
pub const PIXMAN_OP_ATOP: pixman_op_t = 9;
pub const PIXMAN_OP_OUT_REVERSE: pixman_op_t = 8;
pub const PIXMAN_OP_OUT: pixman_op_t = 7;
pub const PIXMAN_OP_IN_REVERSE: pixman_op_t = 6;
pub const PIXMAN_OP_IN: pixman_op_t = 5;
pub const PIXMAN_OP_OVER_REVERSE: pixman_op_t = 4;
pub const PIXMAN_OP_OVER: pixman_op_t = 3;
pub const PIXMAN_OP_DST: pixman_op_t = 2;
pub const PIXMAN_OP_SRC: pixman_op_t = 1;
pub const PIXMAN_OP_CLEAR: pixman_op_t = 0;
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
pub struct _cairo_format_masks {
    pub bpp: libc::c_int,
    pub alpha_mask: libc::c_ulong,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
}
pub type cairo_format_masks_t = _cairo_format_masks;
pub type _cairo_image_transparency = libc::c_uint;
pub const CAIRO_IMAGE_HAS_ALPHA: _cairo_image_transparency = 2;
pub const CAIRO_IMAGE_HAS_BILEVEL_ALPHA: _cairo_image_transparency = 1;
pub const CAIRO_IMAGE_IS_OPAQUE: _cairo_image_transparency = 0;
pub type cairo_image_transparency_t = _cairo_image_transparency;
pub type _cairo_image_color = libc::c_uint;
pub const CAIRO_IMAGE_IS_MONOCHROME: _cairo_image_color = 2;
pub const CAIRO_IMAGE_IS_GRAYSCALE: _cairo_image_color = 1;
pub const CAIRO_IMAGE_IS_COLOR: _cairo_image_color = 0;
pub type cairo_image_color_t = _cairo_image_color;
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
unsafe extern "C" fn _cairo_popcount(mut mask: uint32_t) -> libc::c_int {
    return mask.count_ones() as i32;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_image(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return (!((*surface).backend).is_null()
        && (*(*surface).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_image_surface_set_parent(
    mut image: *mut cairo_image_surface_t,
    mut parent: *mut cairo_surface_t,
) {
    let ref mut fresh2 = (*image).parent;
    *fresh2 = parent;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_snapshot(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT as libc::c_int as cairo_surface_type_t
            as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn _cairo_image_surface_is_size_valid(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> cairo_bool_t {
    return (0 as libc::c_int <= width && width <= 32767 as libc::c_int
        && 0 as libc::c_int <= height && height <= 32767 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_format_from_pixman_format(
    mut pixman_format: pixman_format_code_t,
) -> cairo_format_t {
    match pixman_format as libc::c_uint {
        281756740 => return CAIRO_FORMAT_RGBA128F,
        214631492 => return CAIRO_FORMAT_RGB96F,
        537036936 => return CAIRO_FORMAT_ARGB32,
        537004714 => return CAIRO_FORMAT_RGB30,
        537004168 => return CAIRO_FORMAT_RGB24,
        134316032 => return CAIRO_FORMAT_A8,
        16846848 => return CAIRO_FORMAT_A1,
        268567909 => return CAIRO_FORMAT_RGB16_565,
        537495688 | 537462920 | 537561224 | 537102472 | 537069704 | 402786440 | 402851976
        | 268633445 | 268571989 | 268567893 | 268637525 | 268633429 | 268584004
        | 268567620 | 268649540 | 268633156 | 134349618 | 134415154 | 134357538
        | 134423074 | 134479872 | 134545408 | 134299648 | 67190784 | 67240225 | 67305761
        | 67244305 | 67309841 | 67371008 | 67436544 | 17104896 | 268828672 | 201785344
        | 537397384 | 537430152 | 537078442 | 537070250 | 537012906 | 537003622 | _ => {
            return CAIRO_FORMAT_INVALID;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_content_from_pixman_format(
    mut pixman_format: pixman_format_code_t,
) -> cairo_content_t {
    let mut content: cairo_content_t = 0 as cairo_content_t;
    content = 0 as cairo_content_t;
    if pixman_format as libc::c_uint & 0xfff as libc::c_int as libc::c_uint != 0 {
        content = ::std::mem::transmute::<
            libc::c_uint,
            cairo_content_t,
        >(content as libc::c_uint | CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint);
    }
    if (pixman_format as libc::c_uint >> 12 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (pixman_format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint) != 0
    {
        content = ::std::mem::transmute::<
            libc::c_uint,
            cairo_content_t,
        >(content as libc::c_uint | CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint);
    }
    return content;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_init(
    mut surface: *mut cairo_image_surface_t,
    mut pixman_image: *mut pixman_image_t,
    mut pixman_format: pixman_format_code_t,
) {
    let ref mut fresh3 = (*surface).parent;
    *fresh3 = 0 as *mut cairo_surface_t;
    let ref mut fresh4 = (*surface).pixman_image;
    *fresh4 = pixman_image;
    (*surface).pixman_format = pixman_format;
    (*surface).format = _cairo_format_from_pixman_format(pixman_format);
    let ref mut fresh5 = (*surface).data;
    *fresh5 = pixman_image_get_data(pixman_image) as *mut uint8_t;
    (*surface).set_owns_data(0 as libc::c_int as libc::c_uint);
    (*surface).set_transparency(CAIRO_IMAGE_UNKNOWN as libc::c_int as libc::c_uint);
    (*surface).set_color(CAIRO_IMAGE_UNKNOWN_COLOR as libc::c_int as libc::c_uint);
    (*surface).width = pixman_image_get_width(pixman_image);
    (*surface).height = pixman_image_get_height(pixman_image);
    (*surface).stride = pixman_image_get_stride(pixman_image) as ptrdiff_t;
    (*surface).depth = pixman_image_get_depth(pixman_image);
    let ref mut fresh6 = (*surface).base;
    (*fresh6)
        .set_is_clear(
            ((*surface).width == 0 as libc::c_int
                || (*surface).height == 0 as libc::c_int) as libc::c_int as libc::c_uint,
        );
    let ref mut fresh7 = (*surface).compositor;
    *fresh7 = _cairo_image_spans_compositor_get();
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_create_for_pixman_image(
    mut pixman_image: *mut pixman_image_t,
    mut pixman_format: pixman_format_code_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    surface = (if ::std::mem::size_of::<cairo_image_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_image_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_image_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_image_surface_backend,
        0 as *mut cairo_device_t,
        _cairo_content_from_pixman_format(pixman_format),
        0 as libc::c_int,
    );
    _cairo_image_surface_init(surface, pixman_image, pixman_format);
    return &mut (*surface).base;
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_format_from_masks(
    mut masks: *mut cairo_format_masks_t,
    mut format_ret: *mut pixman_format_code_t,
) -> cairo_bool_t {
    let mut format: pixman_format_code_t = 0 as pixman_format_code_t;
    let mut format_type: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut format_masks: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    a = _cairo_popcount((*masks).alpha_mask as uint32_t);
    r = _cairo_popcount((*masks).red_mask as uint32_t);
    g = _cairo_popcount((*masks).green_mask as uint32_t);
    b = _cairo_popcount((*masks).blue_mask as uint32_t);
    if (*masks).red_mask != 0 {
        if (*masks).red_mask > (*masks).blue_mask {
            format_type = 2 as libc::c_int;
        } else {
            format_type = 3 as libc::c_int;
        }
    } else if (*masks).alpha_mask != 0 {
        format_type = 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    }
    format = ((*masks).bpp << 24 as libc::c_int | format_type << 16 as libc::c_int
        | a << 12 as libc::c_int | r << 8 as libc::c_int | g << 4 as libc::c_int | b)
        as pixman_format_code_t;
    if pixman_format_supported_destination(format) == 0 {
        return 0 as libc::c_int;
    }
    if _pixman_format_to_masks(format, &mut format_masks) == 0
        || (*masks).bpp != format_masks.bpp || (*masks).red_mask != format_masks.red_mask
        || (*masks).green_mask != format_masks.green_mask
        || (*masks).blue_mask != format_masks.blue_mask
    {
        return 0 as libc::c_int;
    }
    *format_ret = format;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_format_to_masks(
    mut format: pixman_format_code_t,
    mut masks: *mut cairo_format_masks_t,
) -> cairo_bool_t {
    let mut a: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    (*masks)
        .bpp = ((format as libc::c_uint >> 24 as libc::c_int
        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    a = ((format as libc::c_uint >> 12 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    r = ((format as libc::c_uint >> 8 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    g = ((format as libc::c_uint >> 4 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    b = ((format as libc::c_uint >> 0 as libc::c_int
        & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    match format as libc::c_uint >> 16 as libc::c_int
        & 0x3f as libc::c_int as libc::c_uint
    {
        2 => {
            (*masks)
                .alpha_mask = ((1 as libc::c_ulong) << a)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << r + g + b;
            (*masks)
                .red_mask = ((1 as libc::c_ulong) << r)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << g + b;
            (*masks)
                .green_mask = ((1 as libc::c_ulong) << g)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << b;
            (*masks)
                .blue_mask = ((1 as libc::c_ulong) << b)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            return 1 as libc::c_int;
        }
        3 => {
            (*masks)
                .alpha_mask = ((1 as libc::c_ulong) << a)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << b + g + r;
            (*masks)
                .blue_mask = ((1 as libc::c_ulong) << b)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << g + r;
            (*masks)
                .green_mask = ((1 as libc::c_ulong) << g)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << r;
            (*masks)
                .red_mask = ((1 as libc::c_ulong) << r)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            return 1 as libc::c_int;
        }
        8 => {
            (*masks)
                .blue_mask = ((1 as libc::c_ulong) << b)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << (*masks).bpp - b;
            (*masks)
                .green_mask = ((1 as libc::c_ulong) << g)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << (*masks).bpp - b - g;
            (*masks)
                .red_mask = ((1 as libc::c_ulong) << r)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                << (*masks).bpp - b - g - r;
            (*masks)
                .alpha_mask = ((1 as libc::c_ulong) << a)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            return 1 as libc::c_int;
        }
        1 => {
            (*masks)
                .alpha_mask = ((1 as libc::c_ulong) << a)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            (*masks).red_mask = 0 as libc::c_int as libc::c_ulong;
            (*masks).green_mask = 0 as libc::c_int as libc::c_ulong;
            (*masks).blue_mask = 0 as libc::c_int as libc::c_ulong;
            return 1 as libc::c_int;
        }
        0 | 4 | 5 | 6 | 7 | _ => {
            (*masks).alpha_mask = 0 as libc::c_int as libc::c_ulong;
            (*masks).red_mask = 0 as libc::c_int as libc::c_ulong;
            (*masks).green_mask = 0 as libc::c_int as libc::c_ulong;
            (*masks).blue_mask = 0 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_format_to_pixman_format_code(
    mut format: cairo_format_t,
) -> pixman_format_code_t {
    let mut ret: pixman_format_code_t = 0 as pixman_format_code_t;
    match format as libc::c_int {
        3 => {
            ret = PIXMAN_a1;
        }
        2 => {
            ret = PIXMAN_a8;
        }
        1 => {
            ret = PIXMAN_x8r8g8b8;
        }
        5 => {
            ret = PIXMAN_x2r10g10b10;
        }
        4 => {
            ret = PIXMAN_r5g6b5;
        }
        6 => {
            ret = PIXMAN_rgb_float;
        }
        7 => {
            ret = PIXMAN_rgba_float;
        }
        0 | -1 | _ => {
            ret = PIXMAN_a8r8g8b8;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_create_with_pixman_format(
    mut data: *mut libc::c_uchar,
    mut pixman_format: pixman_format_code_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut pixman_image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    if _cairo_image_surface_is_size_valid(width, height) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    pixman_image = pixman_image_create_bits(
        pixman_format,
        width,
        height,
        data as *mut uint32_t,
        stride,
    );
    if pixman_image.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    surface = _cairo_image_surface_create_for_pixman_image(pixman_image, pixman_format);
    if (*surface).status as u64 != 0 {
        pixman_image_unref(pixman_image);
        return surface;
    }
    (*surface)
        .set_is_clear(
            (data == 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int
                as libc::c_uint,
        );
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_create(
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    if !(format as libc::c_int >= CAIRO_FORMAT_ARGB32 as libc::c_int
        && format as libc::c_int <= CAIRO_FORMAT_RGBA128F as libc::c_int)
    {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_FORMAT));
    }
    pixman_format = _cairo_format_to_pixman_format_code(format);
    return _cairo_image_surface_create_with_pixman_format(
        0 as *mut libc::c_uchar,
        pixman_format,
        width,
        height,
        -(1 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_create_with_content(
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    return cairo_image_surface_create(
        _cairo_format_from_content(content),
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_format_stride_for_width(
    mut format: cairo_format_t,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut bpp: libc::c_int = 0;
    if !(format as libc::c_int >= CAIRO_FORMAT_ARGB32 as libc::c_int
        && format as libc::c_int <= CAIRO_FORMAT_RGBA128F as libc::c_int)
    {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_INVALID_FORMAT);
        return -(1 as libc::c_int);
    }
    bpp = _cairo_format_bits_per_pixel(format);
    if width as libc::c_uint
        >= ((2147483647 as libc::c_int - 7 as libc::c_int) as libc::c_uint)
            .wrapping_div(bpp as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    return ((((bpp * width + 7 as libc::c_int) / 8 as libc::c_int) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_create_for_data(
    mut data: *mut libc::c_uchar,
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
) -> *mut cairo_surface_t {
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    let mut minstride: libc::c_int = 0;
    if !(format as libc::c_int >= CAIRO_FORMAT_ARGB32 as libc::c_int
        && format as libc::c_int <= CAIRO_FORMAT_RGBA128F as libc::c_int)
    {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_FORMAT));
    }
    if stride as libc::c_ulong
        & (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_STRIDE));
    }
    if _cairo_image_surface_is_size_valid(width, height) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    minstride = cairo_format_stride_for_width(format, width);
    if stride < 0 as libc::c_int {
        if stride > -minstride {
            return _cairo_surface_create_in_error(
                _cairo_error(CAIRO_STATUS_INVALID_STRIDE),
            );
        }
    } else if stride < minstride {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_STRIDE))
    }
    pixman_format = _cairo_format_to_pixman_format_code(format);
    return _cairo_image_surface_create_with_pixman_format(
        data,
        pixman_format,
        width,
        height,
        stride,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_get_data(
    mut surface: *mut cairo_surface_t,
) -> *mut libc::c_uchar {
    let mut image_surface: *mut cairo_image_surface_t = surface
        as *mut cairo_image_surface_t;
    if _cairo_surface_is_image(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as *mut libc::c_uchar;
    }
    return (*image_surface).data;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_get_format(
    mut surface: *mut cairo_surface_t,
) -> cairo_format_t {
    let mut image_surface: *mut cairo_image_surface_t = surface
        as *mut cairo_image_surface_t;
    if _cairo_surface_is_image(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return CAIRO_FORMAT_INVALID;
    }
    return (*image_surface).format;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_get_width(
    mut surface: *mut cairo_surface_t,
) -> libc::c_int {
    let mut image_surface: *mut cairo_image_surface_t = surface
        as *mut cairo_image_surface_t;
    if _cairo_surface_is_image(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    return (*image_surface).width;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_get_height(
    mut surface: *mut cairo_surface_t,
) -> libc::c_int {
    let mut image_surface: *mut cairo_image_surface_t = surface
        as *mut cairo_image_surface_t;
    if _cairo_surface_is_image(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    return (*image_surface).height;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_get_stride(
    mut surface: *mut cairo_surface_t,
) -> libc::c_int {
    let mut image_surface: *mut cairo_image_surface_t = surface
        as *mut cairo_image_surface_t;
    if _cairo_surface_is_image(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    return (*image_surface).stride as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_format_from_content(
    mut content: cairo_content_t,
) -> cairo_format_t {
    match content as libc::c_uint {
        4096 => return CAIRO_FORMAT_RGB24,
        8192 => return CAIRO_FORMAT_A8,
        12288 => return CAIRO_FORMAT_ARGB32,
        _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-image-surface.c\0" as *const u8 as *const libc::c_char,
            698 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"cairo_format_t _cairo_format_from_content(cairo_content_t)\0"))
                .as_ptr(),
        );
    }
    return CAIRO_FORMAT_INVALID;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_content_from_format(
    mut format: cairo_format_t,
) -> cairo_content_t {
    match format as libc::c_int {
        7 | 0 => return CAIRO_CONTENT_COLOR_ALPHA,
        6 | 5 => return CAIRO_CONTENT_COLOR,
        1 => return CAIRO_CONTENT_COLOR,
        4 => return CAIRO_CONTENT_COLOR,
        2 | 3 => return CAIRO_CONTENT_ALPHA,
        -1 | _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-image-surface.c\0" as *const u8 as *const libc::c_char,
            723 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"cairo_content_t _cairo_content_from_format(cairo_format_t)\0"))
                .as_ptr(),
        );
    }
    return CAIRO_CONTENT_COLOR_ALPHA;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_format_bits_per_pixel(
    mut format: cairo_format_t,
) -> libc::c_int {
    match format as libc::c_int {
        7 => return 128 as libc::c_int,
        6 => return 96 as libc::c_int,
        0 | 5 | 1 => return 32 as libc::c_int,
        4 => return 16 as libc::c_int,
        2 => return 8 as libc::c_int,
        3 => return 1 as libc::c_int,
        -1 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-image-surface.c\0" as *const u8
                        as *const libc::c_char,
                    747 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 49],
                        &[libc::c_char; 49],
                    >(b"int _cairo_format_bits_per_pixel(cairo_format_t)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_create_similar(
    mut abstract_other: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut other: *mut cairo_image_surface_t = abstract_other
        as *mut cairo_image_surface_t;
    if _cairo_image_surface_is_size_valid(width, height) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    if content as libc::c_uint == (*other).base.content as libc::c_uint {
        return _cairo_image_surface_create_with_pixman_format(
            0 as *mut libc::c_uchar,
            (*other).pixman_format,
            width,
            height,
            0 as libc::c_int,
        );
    }
    return _cairo_image_surface_create_with_content(content, width, height);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_snapshot(
    mut abstract_surface: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut image: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    let mut clone: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    if (*image).owns_data() as libc::c_int != 0
        && ((*image).base)._finishing() as libc::c_int != 0
    {
        clone = _cairo_image_surface_create_for_pixman_image(
            (*image).pixman_image,
            (*image).pixman_format,
        ) as *mut cairo_image_surface_t;
        if (*clone).base.status as u64 != 0 {
            return &mut (*clone).base;
        }
        let ref mut fresh8 = (*image).pixman_image;
        *fresh8 = 0 as *mut pixman_image_t;
        (*image).set_owns_data(0 as libc::c_int as libc::c_uint);
        (*clone).set_transparency((*image).transparency());
        (*clone).set_color((*image).color());
        (*clone).set_owns_data(1 as libc::c_int as libc::c_uint);
        return &mut (*clone).base;
    }
    clone = _cairo_image_surface_create_with_pixman_format(
        0 as *mut libc::c_uchar,
        (*image).pixman_format,
        (*image).width,
        (*image).height,
        0 as libc::c_int,
    ) as *mut cairo_image_surface_t;
    if (*clone).base.status as u64 != 0 {
        return &mut (*clone).base;
    }
    if (*clone).stride == (*image).stride {
        memcpy(
            (*clone).data as *mut libc::c_void,
            (*image).data as *const libc::c_void,
            ((*clone).stride * (*clone).height as libc::c_long) as libc::c_ulong,
        );
    } else {
        pixman_image_composite32(
            PIXMAN_OP_SRC,
            (*image).pixman_image,
            0 as *mut pixman_image_t,
            (*clone).pixman_image,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            (*image).width,
            (*image).height,
        );
    }
    let ref mut fresh9 = (*clone).base;
    (*fresh9).set_is_clear(0 as libc::c_int as libc::c_uint);
    return &mut (*clone).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_map_to_image(
    mut abstract_other: *mut libc::c_void,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
    let mut other: *mut cairo_image_surface_t = abstract_other
        as *mut cairo_image_surface_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    data = (*other).data;
    data = data.offset(((*extents).y as libc::c_long * (*other).stride) as isize);
    data = data
        .offset(
            ((*extents).x as libc::c_uint)
                .wrapping_mul(
                    ((*other).pixman_format as libc::c_uint >> 24 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << ((*other).pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint),
                )
                .wrapping_div(8 as libc::c_int as libc::c_uint) as isize,
        );
    surface = _cairo_image_surface_create_with_pixman_format(
        data,
        (*other).pixman_format,
        (*extents).width,
        (*extents).height,
        (*other).stride as libc::c_int,
    );
    cairo_surface_set_device_offset(
        surface,
        -(*extents).x as libc::c_double,
        -(*extents).y as libc::c_double,
    );
    return surface as *mut cairo_image_surface_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_unmap_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    cairo_surface_finish(&mut (*image).base);
    cairo_surface_destroy(&mut (*image).base);
    return CAIRO_INT_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    if !((*surface).pixman_image).is_null() {
        pixman_image_unref((*surface).pixman_image);
        let ref mut fresh10 = (*surface).pixman_image;
        *fresh10 = 0 as *mut pixman_image_t;
    }
    if (*surface).owns_data() != 0 {
        free((*surface).data as *mut libc::c_void);
        let ref mut fresh11 = (*surface).data;
        *fresh11 = 0 as *mut libc::c_uchar;
    }
    if !((*surface).parent).is_null() {
        let mut parent: *mut cairo_surface_t = (*surface).parent;
        let ref mut fresh12 = (*surface).parent;
        *fresh12 = 0 as *mut cairo_surface_t;
        cairo_surface_destroy(parent);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_assume_ownership_of_data(
    mut surface: *mut cairo_image_surface_t,
) {
    (*surface).set_owns_data(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    if !extents.is_null() {
        let ref mut fresh13 = (*extents).y;
        *fresh13 = 0 as libc::c_int;
        (*extents).x = *fresh13;
        (*extents).width = (*surface).width;
        (*extents).height = (*surface).height;
    }
    return &mut (*surface).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    *image_out = abstract_surface as *mut cairo_image_surface_t;
    *image_extra = 0 as *mut libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    (*rectangle).x = 0 as libc::c_int;
    (*rectangle).y = 0 as libc::c_int;
    (*rectangle).width = (*surface).width;
    (*rectangle).height = (*surface).height;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    return _cairo_compositor_paint(
        (*surface).compositor,
        &mut (*surface).base,
        op,
        source,
        clip,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    return _cairo_compositor_mask(
        (*surface).compositor,
        &mut (*surface).base,
        op,
        source,
        mask,
        clip,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_stroke(
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
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    return _cairo_compositor_stroke(
        (*surface).compositor,
        &mut (*surface).base,
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    return _cairo_compositor_fill(
        (*surface).compositor,
        &mut (*surface).base,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_image_surface_t = abstract_surface
        as *mut cairo_image_surface_t;
    return _cairo_compositor_glyphs(
        (*surface).compositor,
        &mut (*surface).base,
        op,
        source,
        glyphs,
        num_glyphs,
        scaled_font,
        clip,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    _cairo_font_options_init_default(options);
    cairo_font_options_set_hint_metrics(options, CAIRO_HINT_METRICS_ON);
    _cairo_font_options_set_round_glyph_positions(options, CAIRO_ROUND_GLYPH_POS_ON);
}
#[no_mangle]
pub static mut _cairo_image_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            finish: Some(
                _cairo_image_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_image_surface_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: None,
            map_to_image: Some(
                _cairo_image_surface_map_to_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_rectangle_int_t,
                    ) -> *mut cairo_image_surface_t,
            ),
            unmap_image: Some(
                _cairo_image_surface_unmap_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
            source: Some(
                _cairo_image_surface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_image_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_image_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_image_surface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: None,
            show_page: None,
            get_extents: Some(
                _cairo_image_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_image_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_image_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_image_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_image_surface_stroke
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
                _cairo_image_surface_fill
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
                _cairo_image_surface_glyphs
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
pub unsafe extern "C" fn _cairo_image_surface_coerce(
    mut surface: *mut cairo_image_surface_t,
) -> *mut cairo_image_surface_t {
    return _cairo_image_surface_coerce_to_format(
        surface,
        _cairo_format_from_content((*surface).base.content),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_coerce_to_format(
    mut surface: *mut cairo_image_surface_t,
    mut format: cairo_format_t,
) -> *mut cairo_image_surface_t {
    let mut clone: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = (*surface).base.status;
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status) as *mut cairo_image_surface_t;
    }
    if (*surface).format as libc::c_int == format as libc::c_int {
        return cairo_surface_reference(&mut (*surface).base)
            as *mut cairo_image_surface_t;
    }
    clone = cairo_image_surface_create(format, (*surface).width, (*surface).height)
        as *mut cairo_image_surface_t;
    if (*clone).base.status as u64 != 0 {
        return clone;
    }
    pixman_image_composite32(
        PIXMAN_OP_SRC,
        (*surface).pixman_image,
        0 as *mut pixman_image_t,
        (*clone).pixman_image,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        (*surface).width,
        (*surface).height,
    );
    let ref mut fresh14 = (*clone).base;
    (*fresh14).set_is_clear(0 as libc::c_int as libc::c_uint);
    (*clone).base.device_transform = (*surface).base.device_transform;
    (*clone).base.device_transform_inverse = (*surface).base.device_transform_inverse;
    return clone;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_create_from_image(
    mut other: *mut cairo_image_surface_t,
    mut format: pixman_format_code_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
) -> *mut cairo_image_surface_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    status = (*other).base.status;
    if !(status as u64 != 0) {
        if stride != 0 {
            mem = _cairo_malloc_ab(height as size_t, stride as size_t);
            if mem.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                current_block = 8507267719738525705;
            } else {
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            8507267719738525705 => {}
            _ => {
                image = pixman_image_create_bits(
                    format,
                    width,
                    height,
                    mem as *mut uint32_t,
                    stride,
                );
                if image.is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                } else {
                    surface = _cairo_image_surface_create_for_pixman_image(image, format)
                        as *mut cairo_image_surface_t;
                    if (*surface).base.status as u64 != 0 {
                        status = (*surface).base.status;
                        pixman_image_unref(image);
                    } else {
                        pixman_image_composite32(
                            PIXMAN_OP_SRC,
                            (*other).pixman_image,
                            0 as *mut pixman_image_t,
                            image,
                            x,
                            y,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            width,
                            height,
                        );
                        let ref mut fresh15 = (*surface).base;
                        (*fresh15).set_is_clear(0 as libc::c_int as libc::c_uint);
                        (*surface)
                            .set_owns_data(
                                (mem != 0 as *mut libc::c_void) as libc::c_int
                                    as libc::c_uint,
                            );
                        return surface;
                    }
                }
                free(mem);
            }
        }
    }
    return _cairo_surface_create_in_error(status) as *mut cairo_image_surface_t;
}
unsafe extern "C" fn _cairo_image_compute_transparency(
    mut image: *mut cairo_image_surface_t,
) -> cairo_image_transparency_t {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    if (*image).base.content as libc::c_uint
        & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_IMAGE_IS_OPAQUE;
    }
    if ((*image).base).is_clear() != 0 {
        return CAIRO_IMAGE_HAS_BILEVEL_ALPHA;
    }
    if (*image).base.content as libc::c_uint
        & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        if (*image).format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {
            return CAIRO_IMAGE_HAS_BILEVEL_ALPHA
        } else if (*image).format as libc::c_int == CAIRO_FORMAT_A8 as libc::c_int {
            y = 0 as libc::c_int;
            while y < (*image).height {
                let mut alpha: *mut uint8_t = ((*image).data)
                    .offset((y as libc::c_long * (*image).stride) as isize)
                    as *mut uint8_t;
                x = 0 as libc::c_int;
                while x < (*image).width {
                    if *alpha as libc::c_int > 0 as libc::c_int
                        && (*alpha as libc::c_int) < 255 as libc::c_int
                    {
                        return CAIRO_IMAGE_HAS_ALPHA;
                    }
                    x += 1;
                    alpha = alpha.offset(1);
                }
                y += 1;
            }
            return CAIRO_IMAGE_HAS_BILEVEL_ALPHA;
        } else {
            return CAIRO_IMAGE_HAS_ALPHA
        }
    }
    if (*image).format as libc::c_int == CAIRO_FORMAT_RGB16_565 as libc::c_int {
        return CAIRO_IMAGE_IS_OPAQUE;
    }
    if (*image).format as libc::c_int != CAIRO_FORMAT_ARGB32 as libc::c_int {
        return CAIRO_IMAGE_HAS_ALPHA;
    }
    transparency = CAIRO_IMAGE_IS_OPAQUE;
    y = 0 as libc::c_int;
    while y < (*image).height {
        let mut pixel: *mut uint32_t = ((*image).data)
            .offset((y as libc::c_long * (*image).stride) as isize) as *mut uint32_t;
        x = 0 as libc::c_int;
        while x < (*image).width {
            let mut a: libc::c_int = ((*pixel & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as libc::c_int;
            if a > 0 as libc::c_int && a < 255 as libc::c_int {
                return CAIRO_IMAGE_HAS_ALPHA
            } else {
                if a == 0 as libc::c_int {
                    transparency = CAIRO_IMAGE_HAS_BILEVEL_ALPHA;
                }
            }
            x += 1;
            pixel = pixel.offset(1);
        }
        y += 1;
    }
    return transparency;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_analyze_transparency(
    mut image: *mut cairo_image_surface_t,
) -> cairo_image_transparency_t {
    if _cairo_surface_is_snapshot(&mut (*image).base) != 0 {
        if (*image).transparency() as libc::c_int == CAIRO_IMAGE_UNKNOWN as libc::c_int {
            (*image)
                .set_transparency(
                    _cairo_image_compute_transparency(image) as libc::c_uint,
                );
        }
        return (*image).transparency() as cairo_image_transparency_t;
    }
    return _cairo_image_compute_transparency(image);
}
unsafe extern "C" fn _cairo_image_compute_color(
    mut image: *mut cairo_image_surface_t,
) -> cairo_image_color_t {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: cairo_image_color_t = CAIRO_IMAGE_IS_COLOR;
    if (*image).width == 0 as libc::c_int || (*image).height == 0 as libc::c_int {
        return CAIRO_IMAGE_IS_MONOCHROME;
    }
    if (*image).format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {
        return CAIRO_IMAGE_IS_MONOCHROME;
    }
    if (*image).format as libc::c_int == CAIRO_FORMAT_A8 as libc::c_int {
        return CAIRO_IMAGE_IS_GRAYSCALE;
    }
    if (*image).format as libc::c_int == CAIRO_FORMAT_ARGB32 as libc::c_int {
        color = CAIRO_IMAGE_IS_MONOCHROME;
        y = 0 as libc::c_int;
        while y < (*image).height {
            let mut pixel: *mut uint32_t = ((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize) as *mut uint32_t;
            x = 0 as libc::c_int;
            while x < (*image).width {
                let mut a: libc::c_int = ((*pixel & 0xff000000 as libc::c_uint)
                    >> 24 as libc::c_int) as libc::c_int;
                let mut r: libc::c_int = ((*pixel
                    & 0xff0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int)
                    as libc::c_int;
                let mut g: libc::c_int = ((*pixel
                    & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
                    as libc::c_int;
                let mut b: libc::c_int = (*pixel & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if a == 0 as libc::c_int {
                    b = 0 as libc::c_int;
                    g = b;
                    r = g;
                } else {
                    r = (r * 255 as libc::c_int + a / 2 as libc::c_int) / a;
                    g = (g * 255 as libc::c_int + a / 2 as libc::c_int) / a;
                    b = (b * 255 as libc::c_int + a / 2 as libc::c_int) / a;
                }
                if !(r == g && g == b) {
                    return CAIRO_IMAGE_IS_COLOR
                } else {
                    if r > 0 as libc::c_int && r < 255 as libc::c_int {
                        color = CAIRO_IMAGE_IS_GRAYSCALE;
                    }
                }
                x += 1;
                pixel = pixel.offset(1);
            }
            y += 1;
        }
        return color;
    }
    if (*image).format as libc::c_int == CAIRO_FORMAT_RGB24 as libc::c_int {
        color = CAIRO_IMAGE_IS_MONOCHROME;
        y = 0 as libc::c_int;
        while y < (*image).height {
            let mut pixel_0: *mut uint32_t = ((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize) as *mut uint32_t;
            x = 0 as libc::c_int;
            while x < (*image).width {
                let mut r_0: libc::c_int = ((*pixel_0
                    & 0xff0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int)
                    as libc::c_int;
                let mut g_0: libc::c_int = ((*pixel_0
                    & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
                    as libc::c_int;
                let mut b_0: libc::c_int = (*pixel_0
                    & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
                if !(r_0 == g_0 && g_0 == b_0) {
                    return CAIRO_IMAGE_IS_COLOR
                } else {
                    if r_0 > 0 as libc::c_int && r_0 < 255 as libc::c_int {
                        color = CAIRO_IMAGE_IS_GRAYSCALE;
                    }
                }
                x += 1;
                pixel_0 = pixel_0.offset(1);
            }
            y += 1;
        }
        return color;
    }
    return CAIRO_IMAGE_IS_COLOR;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_analyze_color(
    mut image: *mut cairo_image_surface_t,
) -> cairo_image_color_t {
    if _cairo_surface_is_snapshot(&mut (*image).base) != 0 {
        if (*image).color() as libc::c_int == CAIRO_IMAGE_UNKNOWN_COLOR as libc::c_int {
            (*image).set_color(_cairo_image_compute_color(image) as libc::c_uint);
        }
        return (*image).color() as cairo_image_color_t;
    }
    return _cairo_image_compute_color(image);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_surface_clone_subimage(
    mut surface: *mut cairo_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
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
        surface: 0 as *mut cairo_surface_t,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    image = cairo_surface_create_similar_image(
        surface,
        _cairo_format_from_content((*surface).content),
        (*extents).width,
        (*extents).height,
    );
    if (*image).status as u64 != 0 {
        return image as *mut cairo_image_surface_t;
    }
    cairo_surface_set_device_offset(
        image,
        -(*extents).x as libc::c_double,
        -(*extents).y as libc::c_double,
    );
    _cairo_pattern_init_for_surface(&mut pattern, surface);
    pattern.base.filter = CAIRO_FILTER_NEAREST;
    status = _cairo_surface_paint(
        image,
        CAIRO_OPERATOR_SOURCE,
        &mut pattern.base,
        0 as *const cairo_clip_t,
    );
    _cairo_pattern_fini(&mut pattern.base);
    if status as u64 != 0 {
        cairo_surface_destroy(image);
        return _cairo_surface_create_in_error(status) as *mut cairo_image_surface_t;
    } else {
        _cairo_image_surface_set_parent(
            image as *mut cairo_image_surface_t,
            cairo_surface_reference(surface),
        );
        return image as *mut cairo_image_surface_t;
    };
}
