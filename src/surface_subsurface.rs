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
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_surface_set_device_scale(
        surface: *mut cairo_surface_t,
        x_scale: libc::c_double,
        y_scale: libc::c_double,
    );
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    fn _cairo_clip_copy(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_clip_intersect_rectangle(
        clip: *mut cairo_clip_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_flush(
        surface: *mut cairo_surface_t,
        flags: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn cairo_matrix_init_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn _cairo_surface_create_scratch(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
        color: *const cairo_color_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_create_with_content(
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_unmap_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_map_to_image(
        surface: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_surface_attach_snapshot(
        surface: *mut cairo_surface_t,
        snapshot: *mut cairo_surface_t,
        detach_func: cairo_surface_func_t,
    );
    fn _cairo_surface_detach_snapshot(snapshot: *mut cairo_surface_t);
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_surface_offset_paint(
        target: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_mask(
        target: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_stroke(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
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
    fn _cairo_surface_offset_fill(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_glyphs(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
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
pub type cairo_surface_subsurface_t = _cairo_surface_subsurface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_subsurface {
    pub base: cairo_surface_t,
    pub extents: cairo_rectangle_int_t,
    pub target: *mut cairo_surface_t,
    pub snapshot: *mut cairo_surface_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
#[inline]
unsafe extern "C" fn _cairo_clip_copy_intersect_rectangle(
    mut clip: *const cairo_clip_t,
    mut r: *const cairo_rectangle_int_t,
) -> *mut cairo_clip_t {
    return _cairo_clip_intersect_rectangle(_cairo_clip_copy(clip), r);
}
unsafe extern "C" fn _cairo_surface_subsurface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    cairo_surface_destroy((*surface).target);
    cairo_surface_destroy((*surface).snapshot);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_surface_subsurface_create_similar(
    mut other: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = other
        as *mut cairo_surface_subsurface_t;
    if ((*(*(*surface).target).backend).create_similar).is_none() {
        return 0 as *mut cairo_surface_t;
    }
    return ((*(*(*surface).target).backend).create_similar)
        .expect(
            "non-null function pointer",
        )((*surface).target as *mut libc::c_void, content, width, height);
}
unsafe extern "C" fn _cairo_surface_subsurface_create_similar_image(
    mut other: *mut libc::c_void,
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = other
        as *mut cairo_surface_subsurface_t;
    if ((*(*(*surface).target).backend).create_similar_image).is_none() {
        return 0 as *mut cairo_surface_t;
    }
    return ((*(*(*surface).target).backend).create_similar_image)
        .expect(
            "non-null function pointer",
        )((*surface).target as *mut libc::c_void, format, width, height);
}
unsafe extern "C" fn _cairo_surface_subsurface_map_to_image(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut target_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    target_extents.x = (*extents).x + (*surface).extents.x;
    target_extents.y = (*extents).y + (*surface).extents.y;
    target_extents.width = (*extents).width;
    target_extents.height = (*extents).height;
    return _cairo_surface_map_to_image((*surface).target, &mut target_extents);
}
unsafe extern "C" fn _cairo_surface_subsurface_unmap_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    return _cairo_surface_unmap_image((*surface).target, image);
}
unsafe extern "C" fn _cairo_surface_subsurface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut rect: cairo_rectangle_int_t = {
        let mut init = _cairo_rectangle_int {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
            width: (*surface).extents.width,
            height: (*surface).extents.height,
        };
        init
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut target_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    target_clip = _cairo_clip_copy_intersect_rectangle(clip, &mut rect);
    status = _cairo_surface_offset_paint(
        (*surface).target,
        -(*surface).extents.x,
        -(*surface).extents.y,
        op,
        source,
        target_clip,
    );
    _cairo_clip_destroy(target_clip);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_subsurface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut rect: cairo_rectangle_int_t = {
        let mut init = _cairo_rectangle_int {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
            width: (*surface).extents.width,
            height: (*surface).extents.height,
        };
        init
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut target_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    target_clip = _cairo_clip_copy_intersect_rectangle(clip, &mut rect);
    status = _cairo_surface_offset_mask(
        (*surface).target,
        -(*surface).extents.x,
        -(*surface).extents.y,
        op,
        source,
        mask,
        target_clip,
    );
    _cairo_clip_destroy(target_clip);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_subsurface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut rect: cairo_rectangle_int_t = {
        let mut init = _cairo_rectangle_int {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
            width: (*surface).extents.width,
            height: (*surface).extents.height,
        };
        init
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut target_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    target_clip = _cairo_clip_copy_intersect_rectangle(clip, &mut rect);
    status = _cairo_surface_offset_fill(
        (*surface).target,
        -(*surface).extents.x,
        -(*surface).extents.y,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        target_clip,
    );
    _cairo_clip_destroy(target_clip);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_subsurface_stroke(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut rect: cairo_rectangle_int_t = {
        let mut init = _cairo_rectangle_int {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
            width: (*surface).extents.width,
            height: (*surface).extents.height,
        };
        init
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut target_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    target_clip = _cairo_clip_copy_intersect_rectangle(clip, &mut rect);
    status = _cairo_surface_offset_stroke(
        (*surface).target,
        -(*surface).extents.x,
        -(*surface).extents.y,
        op,
        source,
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        target_clip,
    );
    _cairo_clip_destroy(target_clip);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_subsurface_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut rect: cairo_rectangle_int_t = {
        let mut init = _cairo_rectangle_int {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
            width: (*surface).extents.width,
            height: (*surface).extents.height,
        };
        init
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut target_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    target_clip = _cairo_clip_copy_intersect_rectangle(clip, &mut rect);
    status = _cairo_surface_offset_glyphs(
        (*surface).target,
        -(*surface).extents.x,
        -(*surface).extents.y,
        op,
        source,
        scaled_font,
        glyphs,
        num_glyphs,
        target_clip,
    );
    _cairo_clip_destroy(target_clip);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_subsurface_flush(
    mut abstract_surface: *mut libc::c_void,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    return _cairo_surface_flush((*surface).target, flags);
}
unsafe extern "C" fn _cairo_surface_subsurface_mark_dirty(
    mut abstract_surface: *mut libc::c_void,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = CAIRO_STATUS_SUCCESS;
    if ((*(*(*surface).target).backend).mark_dirty_rectangle).is_some() {
        let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        rect.x = x;
        rect.y = y;
        rect.width = width;
        rect.height = height;
        extents.y = 0 as libc::c_int;
        extents.x = extents.y;
        extents.width = (*surface).extents.width;
        extents.height = (*surface).extents.height;
        if _cairo_rectangle_intersect(&mut rect, &mut extents) != 0 {
            status = ((*(*(*surface).target).backend).mark_dirty_rectangle)
                .expect(
                    "non-null function pointer",
                )(
                (*surface).target as *mut libc::c_void,
                rect.x + (*surface).extents.x,
                rect.y + (*surface).extents.y,
                rect.width,
                rect.height,
            );
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_surface_subsurface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    (*extents).x = 0 as libc::c_int;
    (*extents).y = 0 as libc::c_int;
    (*extents).width = (*surface).extents.width;
    (*extents).height = (*surface).extents.height;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_surface_subsurface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    if ((*(*(*surface).target).backend).get_font_options).is_some() {
        ((*(*(*surface).target).backend).get_font_options)
            .expect(
                "non-null function pointer",
            )((*surface).target as *mut libc::c_void, options);
    }
}
unsafe extern "C" fn _cairo_surface_subsurface_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    source = _cairo_surface_get_source((*surface).target, extents);
    if !extents.is_null() {
        *extents = (*surface).extents;
    }
    return source;
}
unsafe extern "C" fn _cairo_surface_subsurface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut extra_out: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
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
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    image = _cairo_image_surface_create_with_content(
        (*surface).base.content,
        (*surface).extents.width,
        (*surface).extents.height,
    );
    if (*image).status as u64 != 0 {
        return (*image).status;
    }
    _cairo_pattern_init_for_surface(&mut pattern, (*surface).target);
    cairo_matrix_init_translate(
        &mut pattern.base.matrix,
        (*surface).extents.x as libc::c_double,
        (*surface).extents.y as libc::c_double,
    );
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
        return status;
    }
    *image_out = image as *mut cairo_image_surface_t;
    *extra_out = 0 as *mut libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_surface_subsurface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut abstract_extra: *mut libc::c_void,
) {
    cairo_surface_destroy(&mut (*image).base);
}
unsafe extern "C" fn _cairo_surface_subsurface_snapshot(
    mut abstract_surface: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = abstract_surface
        as *mut cairo_surface_subsurface_t;
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
    let mut clone: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    clone = _cairo_surface_create_scratch(
        (*surface).target,
        (*(*surface).target).content,
        (*surface).extents.width,
        (*surface).extents.height,
        0 as *const cairo_color_t,
    );
    if (*clone).status as u64 != 0 {
        return clone;
    }
    _cairo_pattern_init_for_surface(&mut pattern, (*surface).target);
    cairo_matrix_init_translate(
        &mut pattern.base.matrix,
        (*surface).extents.x as libc::c_double,
        (*surface).extents.y as libc::c_double,
    );
    pattern.base.filter = CAIRO_FILTER_NEAREST;
    status = _cairo_surface_paint(
        clone,
        CAIRO_OPERATOR_SOURCE,
        &mut pattern.base,
        0 as *const cairo_clip_t,
    );
    _cairo_pattern_fini(&mut pattern.base);
    if status as u64 != 0 {
        cairo_surface_destroy(clone);
        clone = _cairo_surface_create_in_error(status);
    }
    return clone;
}
unsafe extern "C" fn _cairo_surface_subsurface_create_context(
    mut target: *mut libc::c_void,
) -> *mut cairo_t {
    let mut surface: *mut cairo_surface_subsurface_t = target
        as *mut cairo_surface_subsurface_t;
    return ((*(*(*surface).target).backend).create_context)
        .expect(
            "non-null function pointer",
        )(&mut (*surface).base as *mut cairo_surface_t as *mut libc::c_void);
}
static mut _cairo_surface_subsurface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_SUBSURFACE,
            finish: Some(
                _cairo_surface_subsurface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_surface_subsurface_create_context
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_surface_subsurface_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: Some(
                _cairo_surface_subsurface_create_similar_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_format_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            map_to_image: Some(
                _cairo_surface_subsurface_map_to_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_rectangle_int_t,
                    ) -> *mut cairo_image_surface_t,
            ),
            unmap_image: Some(
                _cairo_surface_subsurface_unmap_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
            source: Some(
                _cairo_surface_subsurface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_surface_subsurface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_surface_subsurface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_surface_subsurface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: None,
            show_page: None,
            get_extents: Some(
                _cairo_surface_subsurface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_surface_subsurface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: Some(
                _cairo_surface_subsurface_flush
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> cairo_status_t,
            ),
            mark_dirty_rectangle: Some(
                _cairo_surface_subsurface_mark_dirty
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> cairo_status_t,
            ),
            paint: Some(
                _cairo_surface_subsurface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_surface_subsurface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_surface_subsurface_stroke
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
                _cairo_surface_subsurface_fill
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
                _cairo_surface_subsurface_glyphs
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
pub unsafe extern "C" fn cairo_surface_create_for_rectangle(
    mut target: *mut cairo_surface_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = 0
        as *mut cairo_surface_subsurface_t;
    if width < 0 as libc::c_int as libc::c_double
        || height < 0 as libc::c_int as libc::c_double
    {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    if (*target).status as u64 != 0 {
        return _cairo_surface_create_in_error((*target).status);
    }
    if (*target).finished() != 0 {
        return _cairo_surface_create_in_error(
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED),
        );
    }
    surface = (if ::std::mem::size_of::<cairo_surface_subsurface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_surface_subsurface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_surface_subsurface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    x *= (*target).device_transform.xx;
    y *= (*target).device_transform.yy;
    width *= (*target).device_transform.xx;
    height *= (*target).device_transform.yy;
    x += (*target).device_transform.x0;
    y += (*target).device_transform.y0;
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_surface_subsurface_backend,
        0 as *mut cairo_device_t,
        (*target).content,
        (*target).is_vector() as cairo_bool_t,
    );
    (*surface).extents.x = ceil(x) as libc::c_int;
    (*surface).extents.y = ceil(y) as libc::c_int;
    (*surface)
        .extents
        .width = (floor(x + width) - (*surface).extents.x as libc::c_double)
        as libc::c_int;
    (*surface)
        .extents
        .height = (floor(y + height) - (*surface).extents.y as libc::c_double)
        as libc::c_int;
    if (*surface).extents.width | (*surface).extents.height < 0 as libc::c_int {
        let ref mut fresh0 = (*surface).extents.height;
        *fresh0 = 0 as libc::c_int;
        (*surface).extents.width = *fresh0;
    }
    if (*(*target).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
    {
        let mut sub: *mut cairo_surface_subsurface_t = target
            as *mut cairo_surface_subsurface_t;
        (*surface).extents.x += (*sub).extents.x;
        (*surface).extents.y += (*sub).extents.y;
        target = (*sub).target;
    }
    let ref mut fresh1 = (*surface).target;
    *fresh1 = cairo_surface_reference(target);
    (*surface).base.type_0 = (*(*surface).target).type_0;
    let ref mut fresh2 = (*surface).snapshot;
    *fresh2 = 0 as *mut cairo_surface_t;
    cairo_surface_set_device_scale(
        &mut (*surface).base,
        (*target).device_transform.xx,
        (*target).device_transform.yy,
    );
    return &mut (*surface).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_create_for_rectangle_int(
    mut target: *mut cairo_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_subsurface_t = 0
        as *mut cairo_surface_subsurface_t;
    if (*target).status as u64 != 0 {
        return _cairo_surface_create_in_error((*target).status);
    }
    if (*target).finished() != 0 {
        return _cairo_surface_create_in_error(
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED),
        );
    }
    if (*(*target).backend).type_0 as libc::c_uint
        != CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"target->backend->type != CAIRO_SURFACE_TYPE_SUBSURFACE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface-subsurface.c\0" as *const u8 as *const libc::c_char,
            523 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 107],
                &[libc::c_char; 107],
            >(
                b"cairo_surface_t *_cairo_surface_create_for_rectangle_int(cairo_surface_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    surface = (if ::std::mem::size_of::<cairo_surface_subsurface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_surface_subsurface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_surface_subsurface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_surface_subsurface_backend,
        0 as *mut cairo_device_t,
        (*target).content,
        (*target).is_vector() as cairo_bool_t,
    );
    (*surface).extents = *extents;
    let ref mut fresh3 = (*surface).extents.x;
    *fresh3 = (*fresh3 as libc::c_double * (*target).device_transform.xx) as libc::c_int;
    let ref mut fresh4 = (*surface).extents.y;
    *fresh4 = (*fresh4 as libc::c_double * (*target).device_transform.yy) as libc::c_int;
    let ref mut fresh5 = (*surface).extents.width;
    *fresh5 = (*fresh5 as libc::c_double * (*target).device_transform.xx) as libc::c_int;
    let ref mut fresh6 = (*surface).extents.height;
    *fresh6 = (*fresh6 as libc::c_double * (*target).device_transform.yy) as libc::c_int;
    let ref mut fresh7 = (*surface).extents.x;
    *fresh7 = (*fresh7 as libc::c_double + (*target).device_transform.x0) as libc::c_int;
    let ref mut fresh8 = (*surface).extents.y;
    *fresh8 = (*fresh8 as libc::c_double + (*target).device_transform.y0) as libc::c_int;
    let ref mut fresh9 = (*surface).target;
    *fresh9 = cairo_surface_reference(target);
    (*surface).base.type_0 = (*(*surface).target).type_0;
    let ref mut fresh10 = (*surface).snapshot;
    *fresh10 = 0 as *mut cairo_surface_t;
    cairo_surface_set_device_scale(
        &mut (*surface).base,
        (*target).device_transform.xx,
        (*target).device_transform.yy,
    );
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_surface_subsurface_detach_snapshot(
    mut surface: *mut cairo_surface_t,
) {
    let mut ss: *mut cairo_surface_subsurface_t = surface
        as *mut cairo_surface_subsurface_t;
    cairo_surface_destroy((*ss).snapshot);
    let ref mut fresh11 = (*ss).snapshot;
    *fresh11 = 0 as *mut cairo_surface_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_subsurface_set_snapshot(
    mut surface: *mut cairo_surface_t,
    mut snapshot: *mut cairo_surface_t,
) {
    let mut ss: *mut cairo_surface_subsurface_t = surface
        as *mut cairo_surface_subsurface_t;
}