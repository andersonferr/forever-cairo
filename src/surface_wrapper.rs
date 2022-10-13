use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _cairo_matrix_multiply(
        r: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn _cairo_matrix_transform_bounding_box(
        matrix: *const cairo_matrix_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
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
    fn _cairo_surface_tag(
        surface: *mut cairo_surface_t,
        begin: cairo_bool_t,
        tag_name: *const libc::c_char,
        attributes: *const libc::c_char,
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
    fn _cairo_surface_fill_stroke(
        surface: *mut cairo_surface_t,
        fill_op: cairo_operator_t,
        fill_source: *const cairo_pattern_t,
        fill_rule: cairo_fill_rule_t,
        fill_tolerance: libc::c_double,
        fill_antialias: cairo_antialias_t,
        path: *mut cairo_path_fixed_t,
        stroke_op: cairo_operator_t,
        stroke_source: *const cairo_pattern_t,
        stroke_style: *const cairo_stroke_style_t,
        stroke_ctm: *const cairo_matrix_t,
        stroke_ctm_inverse: *const cairo_matrix_t,
        stroke_tolerance: libc::c_double,
        stroke_antialias: cairo_antialias_t,
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
    fn _cairo_surface_create_scratch(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
        color: *const cairo_color_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_path_fixed_transform(
        path: *mut cairo_path_fixed_t,
        matrix: *const cairo_matrix_t,
    );
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_init_copy(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn cairo_font_options_merge(
        options: *mut cairo_font_options_t,
        other: *const cairo_font_options_t,
    );
    fn cairo_font_options_equal(
        options: *const cairo_font_options_t,
        other: *const cairo_font_options_t,
    ) -> cairo_bool_t;
    fn cairo_scaled_font_create(
        font_face: *mut cairo_font_face_t,
        font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t,
        options: *const cairo_font_options_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_get_font_options(
        surface: *mut cairo_surface_t,
        options: *mut cairo_font_options_t,
    );
    fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
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
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    static __cairo_clip_all: cairo_clip_t;
    fn _cairo_clip_copy(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_clip_transform(
        clip: *mut cairo_clip_t,
        m: *const cairo_matrix_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_clip_intersect_rectangle(
        clip: *mut cairo_clip_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_clip_intersect_clip(
        clip: *mut cairo_clip_t,
        other: *const cairo_clip_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_clip_get_extents(
        clip: *const cairo_clip_t,
    ) -> *const cairo_rectangle_int_t;
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_pattern_init_static_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    );
    fn _cairo_pattern_create_solid(color: *const cairo_color_t) -> *mut cairo_pattern_t;
    fn _cairo_pattern_transform(
        pattern: *mut cairo_pattern_t,
        ctm_inverse: *const cairo_matrix_t,
    );
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
pub type uint16_t = __uint16_t;
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
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_stop {
    pub offset: libc::c_double,
    pub color: cairo_color_stop_t,
}
pub type cairo_gradient_stop_t = _cairo_gradient_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_pattern {
    pub base: cairo_pattern_t,
    pub n_stops: libc::c_uint,
    pub stops_size: libc::c_uint,
    pub stops: *mut cairo_gradient_stop_t,
    pub stops_embedded: [cairo_gradient_stop_t; 2],
}
pub type cairo_gradient_pattern_t = _cairo_gradient_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_linear_pattern {
    pub base: cairo_gradient_pattern_t,
    pub pd1: cairo_point_double_t,
    pub pd2: cairo_point_double_t,
}
pub type cairo_linear_pattern_t = _cairo_linear_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_radial_pattern {
    pub base: cairo_gradient_pattern_t,
    pub cd1: cairo_circle_double_t,
    pub cd2: cairo_circle_double_t,
}
pub type cairo_radial_pattern_t = _cairo_radial_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_gradient_pattern_union_t {
    pub base: cairo_gradient_pattern_t,
    pub linear: cairo_linear_pattern_t,
    pub radial: cairo_radial_pattern_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_patch {
    pub points: [[cairo_point_double_t; 4]; 4],
    pub colors: [cairo_color_t; 4],
}
pub type cairo_mesh_patch_t = _cairo_mesh_patch;
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
pub type cairo_mesh_pattern_t = _cairo_mesh_pattern;
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
pub type cairo_raster_source_pattern_t = _cairo_raster_source_pattern;
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
unsafe extern "C" fn _cairo_matrix_is_translation(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64) as libc::c_int;
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
unsafe extern "C" fn _cairo_unbounded_rectangle_init(
    mut rect: *mut cairo_rectangle_int_t,
) {
    *rect = _cairo_unbounded_rectangle;
}
#[inline]
unsafe extern "C" fn _cairo_clip_is_all_clipped(
    mut clip: *const cairo_clip_t,
) -> cairo_bool_t {
    return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
}
unsafe extern "C" fn _copy_transformed_pattern(
    mut pattern: *mut cairo_pattern_t,
    mut original: *const cairo_pattern_t,
    mut ctm_inverse: *const cairo_matrix_t,
) {
    _cairo_pattern_init_static_copy(pattern, original);
    if _cairo_matrix_is_identity(ctm_inverse) == 0 {
        _cairo_pattern_transform(pattern, ctm_inverse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_acquire_source_image(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    return _cairo_surface_acquire_source_image(
        (*wrapper).target,
        image_out,
        image_extra,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_release_source_image(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    _cairo_surface_release_source_image((*wrapper).target, image, image_extra);
}
unsafe extern "C" fn _cairo_surface_wrapper_get_transform(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut m: *mut cairo_matrix_t,
) {
    cairo_matrix_init_identity(m);
    if _cairo_matrix_is_identity(&mut (*wrapper).transform) == 0 {
        cairo_matrix_multiply(m, &mut (*wrapper).transform, m);
    }
    if _cairo_matrix_is_identity(&mut (*(*wrapper).target).device_transform) == 0 {
        cairo_matrix_multiply(m, &mut (*(*wrapper).target).device_transform, m);
    }
}
unsafe extern "C" fn _cairo_surface_wrapper_get_inverse_transform(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut m: *mut cairo_matrix_t,
) {
    cairo_matrix_init_identity(m);
    if _cairo_matrix_is_identity(&mut (*(*wrapper).target).device_transform_inverse) == 0
    {
        cairo_matrix_multiply(m, &mut (*(*wrapper).target).device_transform_inverse, m);
    }
    if _cairo_matrix_is_identity(&mut (*wrapper).transform) == 0 {
        let mut inv: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        inv = (*wrapper).transform;
        status = cairo_matrix_invert(&mut inv);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-surface-wrapper.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void _cairo_surface_wrapper_get_inverse_transform(cairo_surface_wrapper_t *, cairo_matrix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        cairo_matrix_multiply(m, &mut inv, m);
    }
}
unsafe extern "C" fn _cairo_surface_wrapper_get_clip(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut clip: *const cairo_clip_t,
) -> *mut cairo_clip_t {
    let mut copy: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut m: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    copy = _cairo_clip_copy(clip);
    if (*wrapper).has_extents != 0 {
        copy = _cairo_clip_intersect_rectangle(copy, &mut (*wrapper).extents);
    }
    _cairo_surface_wrapper_get_transform(wrapper, &mut m);
    copy = _cairo_clip_transform(copy, &mut m);
    if !((*wrapper).clip).is_null() {
        copy = _cairo_clip_intersect_clip(copy, (*wrapper).clip);
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_paint(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut dev_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    dev_clip = _cairo_surface_wrapper_get_clip(wrapper, clip);
    if _cairo_clip_is_all_clipped(dev_clip) != 0 {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    if (*source).is_userfont_foreground != 0 && !((*wrapper).foreground_source).is_null()
    {
        source = (*wrapper).foreground_source;
    }
    if (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        _cairo_surface_wrapper_get_transform(wrapper, &mut m);
        status = cairo_matrix_invert(&mut m);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-surface-wrapper.c\0" as *const u8 as *const libc::c_char,
                156 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 136],
                    &[libc::c_char; 136],
                >(
                    b"cairo_status_t _cairo_surface_wrapper_paint(cairo_surface_wrapper_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_clip_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _copy_transformed_pattern(&mut source_copy.base, source, &mut m);
        source = &mut source_copy.base;
    }
    status = _cairo_surface_paint((*wrapper).target, op, source, dev_clip);
    _cairo_clip_destroy(dev_clip);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_mask(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut dev_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    let mut mask_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    dev_clip = _cairo_surface_wrapper_get_clip(wrapper, clip);
    if _cairo_clip_is_all_clipped(dev_clip) != 0 {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    if (*source).is_userfont_foreground != 0 && !((*wrapper).foreground_source).is_null()
    {
        source = (*wrapper).foreground_source;
    }
    if (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        _cairo_surface_wrapper_get_transform(wrapper, &mut m);
        status = cairo_matrix_invert(&mut m);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-surface-wrapper.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 160],
                    &[libc::c_char; 160],
                >(
                    b"cairo_status_t _cairo_surface_wrapper_mask(cairo_surface_wrapper_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_clip_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _copy_transformed_pattern(&mut source_copy.base, source, &mut m);
        source = &mut source_copy.base;
        _copy_transformed_pattern(&mut mask_copy.base, mask, &mut m);
        mask = &mut mask_copy.base;
    }
    status = _cairo_surface_mask((*wrapper).target, op, source, mask, dev_clip);
    _cairo_clip_destroy(dev_clip);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_stroke(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut path_copy: cairo_path_fixed_t = cairo_path_fixed_t {
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
    };
    let mut dev_path: *mut cairo_path_fixed_t = path as *mut cairo_path_fixed_t;
    let mut dev_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut dev_ctm: cairo_matrix_t = *ctm;
    let mut dev_ctm_inverse: cairo_matrix_t = *ctm_inverse;
    let mut source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    dev_clip = _cairo_surface_wrapper_get_clip(wrapper, clip);
    if _cairo_clip_is_all_clipped(dev_clip) != 0 {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    if (*source).is_userfont_foreground != 0 && !((*wrapper).foreground_source).is_null()
    {
        source = (*wrapper).foreground_source;
    }
    if (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        _cairo_surface_wrapper_get_transform(wrapper, &mut m);
        status = _cairo_path_fixed_init_copy(&mut path_copy, dev_path);
        if status as u64 != 0 {
            current_block = 5498835644851925448;
        } else {
            _cairo_path_fixed_transform(&mut path_copy, &mut m);
            dev_path = &mut path_copy;
            cairo_matrix_multiply(&mut dev_ctm, &mut dev_ctm, &mut m);
            status = cairo_matrix_invert(&mut m);
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-surface-wrapper.c\0" as *const u8
                        as *const libc::c_char,
                    256 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 270],
                        &[libc::c_char; 270],
                    >(
                        b"cairo_status_t _cairo_surface_wrapper_stroke(cairo_surface_wrapper_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            cairo_matrix_multiply(&mut dev_ctm_inverse, &mut m, &mut dev_ctm_inverse);
            _copy_transformed_pattern(&mut source_copy.base, source, &mut m);
            source = &mut source_copy.base;
            current_block = 11584701595673473500;
        }
    } else {
        current_block = 11584701595673473500;
    }
    match current_block {
        11584701595673473500 => {
            status = _cairo_surface_stroke(
                (*wrapper).target,
                op,
                source,
                dev_path,
                stroke_style,
                &mut dev_ctm,
                &mut dev_ctm_inverse,
                tolerance,
                antialias,
                dev_clip,
            );
        }
        _ => {}
    }
    if dev_path != path as *mut cairo_path_fixed_t {
        _cairo_path_fixed_fini(dev_path);
    }
    _cairo_clip_destroy(dev_clip);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_fill_stroke(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut fill_op: cairo_operator_t,
    mut fill_source: *const cairo_pattern_t,
    mut fill_rule: cairo_fill_rule_t,
    mut fill_tolerance: libc::c_double,
    mut fill_antialias: cairo_antialias_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_op: cairo_operator_t,
    mut stroke_source: *const cairo_pattern_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut stroke_ctm: *const cairo_matrix_t,
    mut stroke_ctm_inverse: *const cairo_matrix_t,
    mut stroke_tolerance: libc::c_double,
    mut stroke_antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut path_copy: cairo_path_fixed_t = cairo_path_fixed_t {
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
    };
    let mut dev_path: *mut cairo_path_fixed_t = path as *mut cairo_path_fixed_t;
    let mut dev_ctm: cairo_matrix_t = *stroke_ctm;
    let mut dev_ctm_inverse: cairo_matrix_t = *stroke_ctm_inverse;
    let mut dev_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut stroke_source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    let mut fill_source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    dev_clip = _cairo_surface_wrapper_get_clip(wrapper, clip);
    if _cairo_clip_is_all_clipped(dev_clip) != 0 {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    if (*fill_source).is_userfont_foreground != 0
        && !((*wrapper).foreground_source).is_null()
    {
        fill_source = (*wrapper).foreground_source;
    }
    if (*stroke_source).is_userfont_foreground != 0
        && !((*wrapper).foreground_source).is_null()
    {
        stroke_source = (*wrapper).foreground_source;
    }
    if (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        _cairo_surface_wrapper_get_transform(wrapper, &mut m);
        status = _cairo_path_fixed_init_copy(&mut path_copy, dev_path);
        if status as u64 != 0 {
            current_block = 25117249615787885;
        } else {
            _cairo_path_fixed_transform(&mut path_copy, &mut m);
            dev_path = &mut path_copy;
            cairo_matrix_multiply(&mut dev_ctm, &mut dev_ctm, &mut m);
            status = cairo_matrix_invert(&mut m);
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-surface-wrapper.c\0" as *const u8
                        as *const libc::c_char,
                    330 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 364],
                        &[libc::c_char; 364],
                    >(
                        b"cairo_status_t _cairo_surface_wrapper_fill_stroke(cairo_surface_wrapper_t *, cairo_operator_t, const cairo_pattern_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_path_fixed_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            cairo_matrix_multiply(&mut dev_ctm_inverse, &mut m, &mut dev_ctm_inverse);
            _copy_transformed_pattern(
                &mut stroke_source_copy.base,
                stroke_source,
                &mut m,
            );
            stroke_source = &mut stroke_source_copy.base;
            _copy_transformed_pattern(&mut fill_source_copy.base, fill_source, &mut m);
            fill_source = &mut fill_source_copy.base;
            current_block = 17478428563724192186;
        }
    } else {
        current_block = 17478428563724192186;
    }
    match current_block {
        17478428563724192186 => {
            status = _cairo_surface_fill_stroke(
                (*wrapper).target,
                fill_op,
                fill_source,
                fill_rule,
                fill_tolerance,
                fill_antialias,
                dev_path,
                stroke_op,
                stroke_source,
                stroke_style,
                &mut dev_ctm,
                &mut dev_ctm_inverse,
                stroke_tolerance,
                stroke_antialias,
                dev_clip,
            );
        }
        _ => {}
    }
    if dev_path != path as *mut cairo_path_fixed_t {
        _cairo_path_fixed_fini(dev_path);
    }
    _cairo_clip_destroy(dev_clip);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_fill(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut path_copy: cairo_path_fixed_t = cairo_path_fixed_t {
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
    };
    let mut dev_path: *mut cairo_path_fixed_t = path as *mut cairo_path_fixed_t;
    let mut source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    };
    let mut dev_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    dev_clip = _cairo_surface_wrapper_get_clip(wrapper, clip);
    if _cairo_clip_is_all_clipped(dev_clip) != 0 {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    if (*source).is_userfont_foreground != 0 && !((*wrapper).foreground_source).is_null()
    {
        source = (*wrapper).foreground_source;
    }
    if (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        _cairo_surface_wrapper_get_transform(wrapper, &mut m);
        status = _cairo_path_fixed_init_copy(&mut path_copy, dev_path);
        if status as u64 != 0 {
            current_block = 16673538728585695334;
        } else {
            _cairo_path_fixed_transform(&mut path_copy, &mut m);
            dev_path = &mut path_copy;
            status = cairo_matrix_invert(&mut m);
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-surface-wrapper.c\0" as *const u8
                        as *const libc::c_char,
                    396 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 209],
                        &[libc::c_char; 209],
                    >(
                        b"cairo_status_t _cairo_surface_wrapper_fill(cairo_surface_wrapper_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_clip_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            _copy_transformed_pattern(&mut source_copy.base, source, &mut m);
            source = &mut source_copy.base;
            current_block = 8457315219000651999;
        }
    } else {
        current_block = 8457315219000651999;
    }
    match current_block {
        8457315219000651999 => {
            status = _cairo_surface_fill(
                (*wrapper).target,
                op,
                source,
                dev_path,
                fill_rule,
                tolerance,
                antialias,
                dev_clip,
            );
        }
        _ => {}
    }
    if dev_path != path as *mut cairo_path_fixed_t {
        _cairo_path_fixed_fini(dev_path);
    }
    _cairo_clip_destroy(dev_clip);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_show_text_glyphs(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *const cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut clusters: *const cairo_text_cluster_t,
    mut num_clusters: libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut dev_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut stack_glyphs: [cairo_glyph_t; 85] = [cairo_glyph_t {
        index: 0,
        x: 0.,
        y: 0.,
    }; 85];
    let mut dev_glyphs: *mut cairo_glyph_t = stack_glyphs.as_mut_ptr();
    let mut dev_scaled_font: *mut cairo_scaled_font_t = scaled_font;
    let mut source_copy: cairo_pattern_union_t = cairo_pattern_union_t {
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
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    dev_clip = _cairo_surface_wrapper_get_clip(wrapper, clip);
    if _cairo_clip_is_all_clipped(dev_clip) != 0 {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    cairo_surface_get_font_options((*wrapper).target, &mut options);
    cairo_font_options_merge(&mut options, &mut (*scaled_font).options);
    if (*source).is_userfont_foreground != 0 && !((*wrapper).foreground_source).is_null()
    {
        source = (*wrapper).foreground_source;
    }
    if (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        let mut i: libc::c_int = 0;
        _cairo_surface_wrapper_get_transform(wrapper, &mut m);
        if _cairo_matrix_is_translation(&mut m) == 0 {
            let mut ctm: cairo_matrix_t = cairo_matrix_t {
                xx: 0.,
                yx: 0.,
                xy: 0.,
                yy: 0.,
                x0: 0.,
                y0: 0.,
            };
            _cairo_matrix_multiply(&mut ctm, &mut m, &mut (*scaled_font).ctm);
            dev_scaled_font = cairo_scaled_font_create(
                (*scaled_font).font_face,
                &mut (*scaled_font).font_matrix,
                &mut ctm,
                &mut options,
            );
        }
        if num_glyphs
            > (::std::mem::size_of::<[cairo_glyph_t; 85]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                as libc::c_int
        {
            dev_glyphs = _cairo_malloc_ab(
                num_glyphs as size_t,
                ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
            ) as *mut cairo_glyph_t;
            if dev_glyphs.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                current_block = 13405376692158893296;
            } else {
                current_block = 11298138898191919651;
            }
        } else {
            current_block = 11298138898191919651;
        }
        match current_block {
            13405376692158893296 => {}
            _ => {
                i = 0 as libc::c_int;
                while i < num_glyphs {
                    *dev_glyphs.offset(i as isize) = *glyphs.offset(i as isize);
                    cairo_matrix_transform_point(
                        &mut m,
                        &mut (*dev_glyphs.offset(i as isize)).x,
                        &mut (*dev_glyphs.offset(i as isize)).y,
                    );
                    i += 1;
                }
                status = cairo_matrix_invert(&mut m);
                if status as libc::c_uint
                    == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                            as *const libc::c_char,
                        b"../src/cairo-surface-wrapper.c\0" as *const u8
                            as *const libc::c_char,
                        482 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 280],
                            &[libc::c_char; 280],
                        >(
                            b"cairo_status_t _cairo_surface_wrapper_show_text_glyphs(cairo_surface_wrapper_t *, cairo_operator_t, const cairo_pattern_t *, const char *, int, const cairo_glyph_t *, int, const cairo_text_cluster_t *, int, cairo_text_cluster_flags_t, cairo_scaled_font_t *, const cairo_clip_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                _copy_transformed_pattern(&mut source_copy.base, source, &mut m);
                source = &mut source_copy.base;
                current_block = 18435049525520518667;
            }
        }
    } else {
        if cairo_font_options_equal(&mut options, &mut (*scaled_font).options) == 0 {
            dev_scaled_font = cairo_scaled_font_create(
                (*scaled_font).font_face,
                &mut (*scaled_font).font_matrix,
                &mut (*scaled_font).ctm,
                &mut options,
            );
        }
        if num_glyphs
            > (::std::mem::size_of::<[cairo_glyph_t; 85]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                as libc::c_int
        {
            dev_glyphs = _cairo_malloc_ab(
                num_glyphs as size_t,
                ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
            ) as *mut cairo_glyph_t;
            if dev_glyphs.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                current_block = 13405376692158893296;
            } else {
                current_block = 15090052786889560393;
            }
        } else {
            current_block = 15090052786889560393;
        }
        match current_block {
            13405376692158893296 => {}
            _ => {
                memcpy(
                    dev_glyphs as *mut libc::c_void,
                    glyphs as *const libc::c_void,
                    (::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                        .wrapping_mul(num_glyphs as libc::c_ulong),
                );
                current_block = 18435049525520518667;
            }
        }
    }
    match current_block {
        18435049525520518667 => {
            status = _cairo_surface_show_text_glyphs(
                (*wrapper).target,
                op,
                source,
                utf8,
                utf8_len,
                dev_glyphs,
                num_glyphs,
                clusters,
                num_clusters,
                cluster_flags,
                dev_scaled_font,
                dev_clip,
            );
        }
        _ => {}
    }
    _cairo_clip_destroy(dev_clip);
    if dev_glyphs != stack_glyphs.as_mut_ptr() {
        free(dev_glyphs as *mut libc::c_void);
    }
    if dev_scaled_font != scaled_font {
        cairo_scaled_font_destroy(dev_scaled_font);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_tag(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut begin: cairo_bool_t,
    mut tag_name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_status_t {
    if (*(*wrapper).target).status as u64 != 0 {
        return (*(*wrapper).target).status;
    }
    return _cairo_surface_tag((*wrapper).target, begin, tag_name, attributes);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_create_similar(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    return _cairo_surface_create_scratch(
        (*wrapper).target,
        content,
        width,
        height,
        0 as *const cairo_color_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_get_extents(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    if (*wrapper).has_extents != 0 {
        if _cairo_surface_get_extents((*wrapper).target, extents) != 0 {
            _cairo_rectangle_intersect(extents, &mut (*wrapper).extents);
        } else {
            *extents = (*wrapper).extents;
        }
        return 1 as libc::c_int;
    } else {
        return _cairo_surface_get_extents((*wrapper).target, extents)
    };
}
unsafe extern "C" fn _cairo_surface_wrapper_needs_device_transform(
    mut wrapper: *mut cairo_surface_wrapper_t,
) -> cairo_bool_t {
    return ((*wrapper).has_extents != 0
        && (*wrapper).extents.x | (*wrapper).extents.y != 0
        || _cairo_matrix_is_identity(&mut (*wrapper).transform) == 0
        || _cairo_matrix_is_identity(&mut (*(*wrapper).target).device_transform) == 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_intersect_extents(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut extents: *const cairo_rectangle_int_t,
) {
    if (*wrapper).has_extents == 0 {
        (*wrapper).extents = *extents;
        (*wrapper).has_extents = 1 as libc::c_int;
    } else {
        _cairo_rectangle_intersect(&mut (*wrapper).extents, extents);
    }
    (*wrapper).needs_transform = _cairo_surface_wrapper_needs_device_transform(wrapper);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_set_inverse_transform(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut transform: *const cairo_matrix_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if transform.is_null() || _cairo_matrix_is_identity(transform) != 0 {
        cairo_matrix_init_identity(&mut (*wrapper).transform);
        (*wrapper)
            .needs_transform = _cairo_surface_wrapper_needs_device_transform(wrapper);
    } else {
        (*wrapper).transform = *transform;
        status = cairo_matrix_invert(&mut (*wrapper).transform);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-surface-wrapper.c\0" as *const u8 as *const libc::c_char,
                602 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"void _cairo_surface_wrapper_set_inverse_transform(cairo_surface_wrapper_t *, const cairo_matrix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        (*wrapper).needs_transform = 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_set_clip(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut clip: *const cairo_clip_t,
) {
    let ref mut fresh2 = (*wrapper).clip;
    *fresh2 = clip;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_set_foreground_color(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut color: *const cairo_color_t,
) {
    if !color.is_null() {
        let ref mut fresh3 = (*wrapper).foreground_source;
        *fresh3 = _cairo_pattern_create_solid(color);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_get_font_options(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut options: *mut cairo_font_options_t,
) {
    cairo_surface_get_font_options((*wrapper).target, options);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_snapshot(
    mut wrapper: *mut cairo_surface_wrapper_t,
) -> *mut cairo_surface_t {
    if ((*(*(*wrapper).target).backend).snapshot).is_some() {
        return ((*(*(*wrapper).target).backend).snapshot)
            .expect("non-null function pointer")((*wrapper).target as *mut libc::c_void);
    }
    return 0 as *mut cairo_surface_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_has_show_text_glyphs(
    mut wrapper: *mut cairo_surface_wrapper_t,
) -> cairo_bool_t {
    return cairo_surface_has_show_text_glyphs((*wrapper).target);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_init(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut target: *mut cairo_surface_t,
) {
    let ref mut fresh4 = (*wrapper).target;
    *fresh4 = cairo_surface_reference(target);
    cairo_matrix_init_identity(&mut (*wrapper).transform);
    (*wrapper).has_extents = 0 as libc::c_int;
    let ref mut fresh5 = (*wrapper).extents.y;
    *fresh5 = 0 as libc::c_int;
    (*wrapper).extents.x = *fresh5;
    let ref mut fresh6 = (*wrapper).clip;
    *fresh6 = 0 as *const cairo_clip_t;
    let ref mut fresh7 = (*wrapper).foreground_source;
    *fresh7 = 0 as *mut cairo_pattern_t;
    (*wrapper).needs_transform = 0 as libc::c_int;
    if !target.is_null() {
        (*wrapper)
            .needs_transform = (_cairo_matrix_is_identity(
            &mut (*target).device_transform,
        ) == 0) as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_fini(
    mut wrapper: *mut cairo_surface_wrapper_t,
) {
    if !((*wrapper).foreground_source).is_null() {
        cairo_pattern_destroy((*wrapper).foreground_source);
    }
    cairo_surface_destroy((*wrapper).target);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_wrapper_get_target_extents(
    mut wrapper: *mut cairo_surface_wrapper_t,
    mut surface_is_unbounded: cairo_bool_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut clip: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut has_clip: cairo_bool_t = 0 as libc::c_int;
    if surface_is_unbounded == 0 {
        has_clip = _cairo_surface_get_extents((*wrapper).target, &mut clip);
    }
    if !((*wrapper).clip).is_null() {
        if has_clip != 0 {
            if _cairo_rectangle_intersect(
                &mut clip,
                _cairo_clip_get_extents((*wrapper).clip),
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else {
            has_clip = 1 as libc::c_int;
            clip = *_cairo_clip_get_extents((*wrapper).clip);
        }
    }
    if has_clip != 0 && (*wrapper).needs_transform != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        _cairo_surface_wrapper_get_inverse_transform(wrapper, &mut m);
        x1 = clip.x as libc::c_double;
        y1 = clip.y as libc::c_double;
        x2 = (clip.x + clip.width) as libc::c_double;
        y2 = (clip.y + clip.height) as libc::c_double;
        _cairo_matrix_transform_bounding_box(
            &mut m,
            &mut x1,
            &mut y1,
            &mut x2,
            &mut y2,
            0 as *mut cairo_bool_t,
        );
        clip.x = floor(x1) as libc::c_int;
        clip.y = floor(y1) as libc::c_int;
        clip.width = (ceil(x2) - clip.x as libc::c_double) as libc::c_int;
        clip.height = (ceil(y2) - clip.y as libc::c_double) as libc::c_int;
    }
    if has_clip != 0 {
        if (*wrapper).has_extents != 0 {
            *extents = (*wrapper).extents;
            return _cairo_rectangle_intersect(extents, &mut clip);
        } else {
            *extents = clip;
            return 1 as libc::c_int;
        }
    } else if (*wrapper).has_extents != 0 {
        *extents = (*wrapper).extents;
        return 1 as libc::c_int;
    } else {
        _cairo_unbounded_rectangle_init(extents);
        return 1 as libc::c_int;
    };
}
