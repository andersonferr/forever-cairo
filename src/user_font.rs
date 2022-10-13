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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    fn cairo_set_source_rgb(
        cr: *mut cairo_t,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
    );
    fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    fn cairo_glyph_free(glyphs: *mut cairo_glyph_t);
    fn cairo_set_font_size(cr: *mut cairo_t, size: libc::c_double);
    fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t);
    fn _cairo_scaled_font_fini(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_set_metrics(
        scaled_font: *mut cairo_scaled_font_t,
        fs_metrics: *mut cairo_font_extents_t,
    ) -> cairo_status_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_scaled_font_unregister_placeholder_and_lock_font_map(
        scaled_font: *mut cairo_scaled_font_t,
    );
    fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn _cairo_matrix_is_scale_0(matrix: *const cairo_matrix_t) -> cairo_bool_t;
    fn cairo_recording_surface_create(
        content: cairo_content_t,
        extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_scaled_font_register_placeholder_and_unlock_font_map(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> cairo_status_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn cairo_matrix_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn _cairo_matrix_compute_basis_scale_factors(
        matrix: *const cairo_matrix_t,
        sx: *mut libc::c_double,
        sy: *mut libc::c_double,
        x_major: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_set_error(
        scaled_font: *mut cairo_scaled_font_t,
        status: cairo_status_t,
    ) -> cairo_status_t;
    fn cairo_matrix_transform_point(
        matrix: *const cairo_matrix_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn _cairo_path_fixed_create() -> *mut cairo_path_fixed_t;
    fn _cairo_scaled_glyph_set_path(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        path: *mut cairo_path_fixed_t,
    );
    fn _cairo_path_fixed_destroy(path: *mut cairo_path_fixed_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_scaled_glyph_set_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        surface: *mut cairo_image_surface_t,
    );
    fn _cairo_scaled_glyph_set_color_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        surface: *mut cairo_image_surface_t,
        uses_foreground_color: cairo_bool_t,
    );
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
    fn _cairo_scaled_glyph_set_metrics(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        fs_metrics: *mut cairo_text_extents_t,
    );
    fn _cairo_box_to_doubles(
        box_0: *const cairo_box_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
    );
    fn _cairo_scaled_glyph_set_recording_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        recording_surface: *mut cairo_surface_t,
    );
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    fn cairo_pattern_create_rgb(
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
    ) -> *mut cairo_pattern_t;
    fn _cairo_scaled_font_init(
        scaled_font: *mut cairo_scaled_font_t,
        font_face: *mut cairo_font_face_t,
        font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t,
        options: *const cairo_font_options_t,
        backend: *const cairo_scaled_font_backend_t,
    ) -> cairo_status_t;
    fn _cairo_font_face_destroy(abstract_face: *mut libc::c_void) -> cairo_bool_t;
    fn _cairo_font_face_twin_create_for_toy(
        toy_face: *mut cairo_toy_font_face_t,
        font_face: *mut *mut cairo_font_face_t,
    ) -> cairo_status_t;
    fn _cairo_font_face_init(
        font_face: *mut cairo_font_face_t,
        backend: *const cairo_font_face_backend_t,
    );
    static _cairo_font_face_nil: cairo_font_face_t;
    fn _cairo_font_face_set_error(
        font_face: *mut cairo_font_face_t,
        status: cairo_status_t,
    ) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_recording_surface_get_path(
        surface: *mut cairo_surface_t,
        path: *mut cairo_path_fixed_t,
    ) -> cairo_int_status_t;
    fn _cairo_recording_surface_replay(
        surface: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_recording_surface_replay_with_foreground_color(
        surface: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
        color: *const cairo_color_t,
    ) -> cairo_status_t;
    fn _cairo_recording_surface_get_bbox(
        recording: *mut cairo_recording_surface_t,
        bbox: *mut cairo_box_t,
        transform: *const cairo_matrix_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
pub type cairo_user_font_face_t = _cairo_user_font_face;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_font_face {
    pub base: cairo_font_face_t,
    pub immutable: cairo_bool_t,
    pub has_color: cairo_bool_t,
    pub scaled_font_methods: cairo_user_scaled_font_methods_t,
}
pub type cairo_user_scaled_font_methods_t = _cairo_user_scaled_font_methods;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_scaled_font_methods {
    pub init: cairo_user_scaled_font_init_func_t,
    pub render_color_glyph: cairo_user_scaled_font_render_glyph_func_t,
    pub render_glyph: cairo_user_scaled_font_render_glyph_func_t,
    pub unicode_to_glyph: cairo_user_scaled_font_unicode_to_glyph_func_t,
    pub text_to_glyphs: cairo_user_scaled_font_text_to_glyphs_func_t,
}
pub type cairo_user_scaled_font_text_to_glyphs_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        *const libc::c_char,
        libc::c_int,
        *mut *mut cairo_glyph_t,
        *mut libc::c_int,
        *mut *mut cairo_text_cluster_t,
        *mut libc::c_int,
        *mut cairo_text_cluster_flags_t,
    ) -> cairo_status_t,
>;
pub type cairo_user_scaled_font_unicode_to_glyph_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> cairo_status_t,
>;
pub type cairo_user_scaled_font_render_glyph_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        libc::c_ulong,
        *mut cairo_t,
        *mut cairo_text_extents_t,
    ) -> cairo_status_t,
>;
pub type cairo_user_scaled_font_init_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_t,
        *mut cairo_t,
        *mut cairo_font_extents_t,
    ) -> cairo_status_t,
>;
pub type cairo_user_scaled_font_t = _cairo_user_scaled_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_scaled_font {
    pub base: cairo_scaled_font_t,
    pub default_glyph_extents: cairo_text_extents_t,
    pub extent_scale: cairo_matrix_t,
    pub extent_x_scale: libc::c_double,
    pub extent_y_scale: libc::c_double,
    pub snap_x_scale: libc::c_double,
    pub snap_y_scale: libc::c_double,
}
pub type cairo_fixed_unsigned_t = uint32_t;
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
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_ceil(mut f: cairo_fixed_t) -> libc::c_int {
    if f > 0 as libc::c_int {
        return (f - 1 as libc::c_int >> 8 as libc::c_int) + 1 as libc::c_int
    } else {
        return -((f as cairo_fixed_unsigned_t).wrapping_neg() as cairo_fixed_t
            >> 8 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
unsafe extern "C" fn _cairo_user_scaled_font_create_recording_surface(
    mut scaled_font: *const cairo_user_scaled_font_t,
    mut color: cairo_bool_t,
) -> *mut cairo_surface_t {
    let mut content: cairo_content_t = 0 as cairo_content_t;
    if color != 0 {
        content = CAIRO_CONTENT_COLOR_ALPHA;
    } else {
        content = (if (*scaled_font).base.options.antialias as libc::c_uint
            == CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
        {
            CAIRO_CONTENT_COLOR_ALPHA as libc::c_int
        } else {
            CAIRO_CONTENT_ALPHA as libc::c_int
        }) as cairo_content_t;
    }
    return cairo_recording_surface_create(content, 0 as *const cairo_rectangle_t);
}
unsafe extern "C" fn _cairo_user_scaled_font_create_recording_context(
    mut scaled_font: *const cairo_user_scaled_font_t,
    mut recording_surface: *mut cairo_surface_t,
    mut color: cairo_bool_t,
) -> *mut cairo_t {
    let mut cr: *mut cairo_t = 0 as *mut cairo_t;
    cr = cairo_create(recording_surface);
    if _cairo_matrix_is_scale_0(&(*scaled_font).base.scale) == 0 {
        let mut scale: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        scale = (*scaled_font).base.scale;
        scale.y0 = 0.0f64;
        scale.x0 = scale.y0;
        cairo_set_matrix(cr, &mut scale);
    }
    cairo_set_font_size(cr, 1.0f64);
    cairo_set_font_options(cr, &(*scaled_font).base.options);
    if color == 0 {
        cairo_set_source_rgb(cr, 1.0f64, 1.0f64, 1.0f64);
    }
    return cr;
}
unsafe extern "C" fn _cairo_user_scaled_glyph_init_record_glyph(
    mut scaled_font: *mut cairo_user_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
) -> cairo_int_status_t {
    let mut face: *mut cairo_user_font_face_t = (*scaled_font).base.font_face
        as *mut cairo_user_font_face_t;
    let mut extents: cairo_text_extents_t = (*scaled_font).default_glyph_extents;
    let mut recording_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut cr: *mut cairo_t = 0 as *mut cairo_t;
    if ((*face).scaled_font_methods.render_color_glyph).is_none()
        && ((*face).scaled_font_methods.render_glyph).is_none()
    {
        return CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int
            as cairo_int_status_t;
    }
    if _cairo_matrix_is_scale_0(&mut (*scaled_font).base.scale) != 0 {
        recording_surface = _cairo_user_scaled_font_create_recording_surface(
            scaled_font,
            0 as libc::c_int,
        );
        _cairo_scaled_glyph_set_recording_surface(
            scaled_glyph,
            &mut (*scaled_font).base,
            recording_surface,
        );
    } else {
        status = CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int
            as cairo_int_status_t;
        if ((*face).scaled_font_methods.render_color_glyph).is_some() {
            let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
            recording_surface = _cairo_user_scaled_font_create_recording_surface(
                scaled_font,
                1 as libc::c_int,
            );
            cr = _cairo_user_scaled_font_create_recording_context(
                scaled_font,
                recording_surface,
                1 as libc::c_int,
            );
            pattern = cairo_pattern_create_rgb(
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
            );
            (*pattern).is_userfont_foreground = 1 as libc::c_int;
            cairo_set_source(cr, pattern);
            cairo_pattern_destroy(pattern);
            status = ((*face).scaled_font_methods.render_color_glyph)
                .expect(
                    "non-null function pointer",
                )(
                scaled_font as *mut cairo_scaled_font_t,
                (*scaled_glyph).hash_entry.hash
                    & 0xffffff as libc::c_int as libc::c_ulong,
                cr,
                &mut extents,
            ) as cairo_int_status_t;
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = cairo_status(cr) as cairo_int_status_t;
                (*scaled_glyph).set_color_glyph(1 as libc::c_int as libc::c_uint);
                (*scaled_glyph).set_color_glyph_set(1 as libc::c_int as libc::c_uint);
            }
            cairo_destroy(cr);
        }
        if status as libc::c_uint
            == CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int
                as cairo_int_status_t as libc::c_uint
            && ((*face).scaled_font_methods.render_glyph).is_some()
        {
            if !recording_surface.is_null() {
                cairo_surface_destroy(recording_surface);
            }
            recording_surface = _cairo_user_scaled_font_create_recording_surface(
                scaled_font,
                0 as libc::c_int,
            );
            (*recording_surface)
                .device_transform
                .x0 = 0.25f64
                * ((*scaled_glyph).hash_entry.hash >> 24 as libc::c_int
                    & 3 as libc::c_int as libc::c_ulong) as libc::c_int
                    as libc::c_double;
            (*recording_surface)
                .device_transform
                .y0 = 0.25f64
                * ((*scaled_glyph).hash_entry.hash >> 26 as libc::c_int
                    & 3 as libc::c_int as libc::c_ulong) as libc::c_int
                    as libc::c_double;
            cr = _cairo_user_scaled_font_create_recording_context(
                scaled_font,
                recording_surface,
                0 as libc::c_int,
            );
            status = ((*face).scaled_font_methods.render_glyph)
                .expect(
                    "non-null function pointer",
                )(
                scaled_font as *mut cairo_scaled_font_t,
                (*scaled_glyph).hash_entry.hash
                    & 0xffffff as libc::c_int as libc::c_ulong,
                cr,
                &mut extents,
            ) as cairo_int_status_t;
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = cairo_status(cr) as cairo_int_status_t;
                (*scaled_glyph).set_color_glyph(0 as libc::c_int as libc::c_uint);
                (*scaled_glyph).set_color_glyph_set(1 as libc::c_int as libc::c_uint);
            }
            cairo_destroy(cr);
        }
        if status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            if !recording_surface.is_null() {
                cairo_surface_destroy(recording_surface);
            }
            return status;
        }
        _cairo_scaled_glyph_set_recording_surface(
            scaled_glyph,
            &mut (*scaled_font).base,
            recording_surface,
        );
    }
    if extents.width == 0.0f64 {
        let mut bbox: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        let mut x_scale: libc::c_double = 0.;
        let mut y_scale: libc::c_double = 0.;
        status = _cairo_recording_surface_get_bbox(
            recording_surface as *mut cairo_recording_surface_t,
            &mut bbox,
            &mut (*scaled_font).extent_scale,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_box_to_doubles(&mut bbox, &mut x1, &mut y1, &mut x2, &mut y2);
        x_scale = (*scaled_font).extent_x_scale;
        y_scale = (*scaled_font).extent_y_scale;
        extents.x_bearing = x1 * x_scale;
        extents.y_bearing = y1 * y_scale;
        extents.width = (x2 - x1) * x_scale;
        extents.height = (y2 - y1) * y_scale;
    }
    if (*scaled_font).base.options.hint_metrics as libc::c_uint
        != CAIRO_HINT_METRICS_OFF as libc::c_int as libc::c_uint
    {
        extents
            .x_advance = _cairo_lround(extents.x_advance / (*scaled_font).snap_x_scale)
            as libc::c_double * (*scaled_font).snap_x_scale;
        extents
            .y_advance = _cairo_lround(extents.y_advance / (*scaled_font).snap_y_scale)
            as libc::c_double * (*scaled_font).snap_y_scale;
    }
    _cairo_scaled_glyph_set_metrics(
        scaled_glyph,
        &mut (*scaled_font).base,
        &mut extents,
    );
    return status;
}
unsafe extern "C" fn _cairo_user_scaled_glyph_init_surface(
    mut scaled_font: *mut cairo_user_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut info: cairo_scaled_glyph_info_t,
    mut foreground_color: *const cairo_color_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
        || info as libc::c_uint
            == CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"info == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE || info == CAIRO_SCALED_GLYPH_INFO_SURFACE\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-user-font.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 159],
                &[libc::c_char; 159],
            >(
                b"cairo_int_status_t _cairo_user_scaled_glyph_init_surface(cairo_user_scaled_font_t *, cairo_scaled_glyph_t *, cairo_scaled_glyph_info_t, const cairo_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    width = _cairo_fixed_integer_ceil((*scaled_glyph).bbox.p2.x)
        - _cairo_fixed_integer_floor((*scaled_glyph).bbox.p1.x);
    height = _cairo_fixed_integer_ceil((*scaled_glyph).bbox.p2.y)
        - _cairo_fixed_integer_floor((*scaled_glyph).bbox.p1.y);
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
    {
        format = CAIRO_FORMAT_ARGB32;
    } else {
        match (*scaled_font).base.options.antialias as libc::c_uint {
            1 => {
                format = CAIRO_FORMAT_A1;
            }
            6 | 3 => {
                format = CAIRO_FORMAT_ARGB32;
            }
            0 | 4 | 5 | 2 | _ => {
                format = CAIRO_FORMAT_A8;
            }
        }
    }
    surface = cairo_image_surface_create(format, width, height);
    cairo_surface_set_device_offset(
        surface,
        -_cairo_fixed_integer_floor((*scaled_glyph).bbox.p1.x) as libc::c_double,
        -_cairo_fixed_integer_floor((*scaled_glyph).bbox.p1.y) as libc::c_double,
    );
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
    {
        status = _cairo_recording_surface_replay_with_foreground_color(
            (*scaled_glyph).recording_surface,
            surface,
            foreground_color,
        ) as cairo_int_status_t;
    } else {
        status = _cairo_recording_surface_replay(
            (*scaled_glyph).recording_surface,
            surface,
        ) as cairo_int_status_t;
    }
    if status as u64 != 0 {
        cairo_surface_destroy(surface);
        return status;
    }
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
    {
        _cairo_scaled_glyph_set_color_surface(
            scaled_glyph,
            &mut (*scaled_font).base,
            surface as *mut cairo_image_surface_t,
            1 as libc::c_int,
        );
        surface = 0 as *mut cairo_surface_t;
    } else {
        _cairo_scaled_glyph_set_surface(
            scaled_glyph,
            &mut (*scaled_font).base,
            surface as *mut cairo_image_surface_t,
        );
        surface = 0 as *mut cairo_surface_t;
    }
    if !surface.is_null() {
        cairo_surface_destroy(surface);
    }
    return status;
}
unsafe extern "C" fn _cairo_user_scaled_glyph_init(
    mut abstract_font: *mut libc::c_void,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut info: cairo_scaled_glyph_info_t,
    mut foreground_color: *const cairo_color_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut scaled_font: *mut cairo_user_scaled_font_t = abstract_font
        as *mut cairo_user_scaled_font_t;
    if ((*scaled_glyph).recording_surface).is_null() {
        status = _cairo_user_scaled_glyph_init_record_glyph(scaled_font, scaled_glyph);
        if status as u64 != 0 {
            return status;
        }
    }
    if info as libc::c_uint
        & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint != 0
    {
        if (*scaled_glyph).color_glyph() == 0 {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        status = _cairo_user_scaled_glyph_init_surface(
            scaled_font,
            scaled_glyph,
            CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE,
            foreground_color,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    if info as libc::c_uint
        & CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int as libc::c_uint != 0
    {
        status = _cairo_user_scaled_glyph_init_surface(
            scaled_font,
            scaled_glyph,
            CAIRO_SCALED_GLYPH_INFO_SURFACE,
            0 as *const cairo_color_t,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    if info as libc::c_uint & CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int as libc::c_uint
        != 0
    {
        let mut path: *mut cairo_path_fixed_t = _cairo_path_fixed_create();
        if path.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_recording_surface_get_path(
            (*scaled_glyph).recording_surface,
            path,
        );
        if status as u64 != 0 {
            _cairo_path_fixed_destroy(path);
            return status;
        }
        _cairo_scaled_glyph_set_path(scaled_glyph, &mut (*scaled_font).base, path);
    }
    return status;
}
unsafe extern "C" fn _cairo_user_ucs4_to_index(
    mut abstract_font: *mut libc::c_void,
    mut ucs4: uint32_t,
) -> libc::c_ulong {
    let mut scaled_font: *mut cairo_user_scaled_font_t = abstract_font
        as *mut cairo_user_scaled_font_t;
    let mut face: *mut cairo_user_font_face_t = (*scaled_font).base.font_face
        as *mut cairo_user_font_face_t;
    let mut glyph: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut current_block_5: u64;
    if ((*face).scaled_font_methods.unicode_to_glyph).is_some() {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = ((*face).scaled_font_methods.unicode_to_glyph)
            .expect(
                "non-null function pointer",
            )(&mut (*scaled_font).base, ucs4 as libc::c_ulong, &mut glyph);
        if status as libc::c_uint
            == CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int as libc::c_uint
        {
            current_block_5 = 8266534687289315666;
        } else {
            if status as libc::c_uint
                != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = _cairo_scaled_font_set_error(&mut (*scaled_font).base, status);
                glyph = 0 as libc::c_int as libc::c_ulong;
            }
            current_block_5 = 17965632435239708295;
        }
    } else {
        current_block_5 = 8266534687289315666;
    }
    match current_block_5 {
        8266534687289315666 => {
            glyph = ucs4 as libc::c_ulong;
        }
        _ => {}
    }
    return glyph;
}
unsafe extern "C" fn _cairo_user_has_color_glyphs(
    mut abstract_font: *mut libc::c_void,
) -> cairo_bool_t {
    let mut scaled_font: *mut cairo_user_scaled_font_t = abstract_font
        as *mut cairo_user_scaled_font_t;
    let mut face: *mut cairo_user_font_face_t = (*scaled_font).base.font_face
        as *mut cairo_user_font_face_t;
    return (*face).has_color;
}
unsafe extern "C" fn _cairo_user_text_to_glyphs(
    mut abstract_font: *mut libc::c_void,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut *mut cairo_glyph_t,
    mut num_glyphs: *mut libc::c_int,
    mut clusters: *mut *mut cairo_text_cluster_t,
    mut num_clusters: *mut libc::c_int,
    mut cluster_flags: *mut cairo_text_cluster_flags_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_UNSUPPORTED;
    let mut scaled_font: *mut cairo_user_scaled_font_t = abstract_font
        as *mut cairo_user_scaled_font_t;
    let mut face: *mut cairo_user_font_face_t = (*scaled_font).base.font_face
        as *mut cairo_user_font_face_t;
    if ((*face).scaled_font_methods.text_to_glyphs).is_some() {
        let mut i: libc::c_int = 0;
        let mut orig_glyphs: *mut cairo_glyph_t = *glyphs;
        let mut orig_num_glyphs: libc::c_int = *num_glyphs;
        status = ((*face).scaled_font_methods.text_to_glyphs)
            .expect(
                "non-null function pointer",
            )(
            &mut (*scaled_font).base,
            utf8,
            utf8_len,
            glyphs,
            num_glyphs,
            clusters,
            num_clusters,
            cluster_flags,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && status as libc::c_uint
                != CAIRO_INT_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int
                    as libc::c_uint
        {
            return status;
        }
        if status as libc::c_uint
            == CAIRO_INT_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int as libc::c_uint
            || *num_glyphs < 0 as libc::c_int
        {
            if orig_glyphs != *glyphs {
                cairo_glyph_free(*glyphs);
                *glyphs = orig_glyphs;
            }
            *num_glyphs = orig_num_glyphs;
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        i = 0 as libc::c_int;
        while i < *num_glyphs {
            let mut gx: libc::c_double = (*(*glyphs).offset(i as isize)).x;
            let mut gy: libc::c_double = (*(*glyphs).offset(i as isize)).y;
            cairo_matrix_transform_point(
                &mut (*scaled_font).base.font_matrix,
                &mut gx,
                &mut gy,
            );
            (*(*glyphs).offset(i as isize)).x = gx + x;
            (*(*glyphs).offset(i as isize)).y = gy + y;
            i += 1;
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_user_font_face_create_for_toy(
    mut toy_face: *mut cairo_toy_font_face_t,
    mut font_face: *mut *mut cairo_font_face_t,
) -> cairo_status_t {
    return _cairo_font_face_twin_create_for_toy(toy_face, font_face);
}
static mut _cairo_user_scaled_font_backend: cairo_scaled_font_backend_t = unsafe {
    {
        let mut init = _cairo_scaled_font_backend {
            type_0: CAIRO_FONT_TYPE_USER,
            fini: None,
            scaled_glyph_init: Some(
                _cairo_user_scaled_glyph_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_scaled_glyph_t,
                        cairo_scaled_glyph_info_t,
                        *const cairo_color_t,
                    ) -> cairo_int_status_t,
            ),
            text_to_glyphs: Some(
                _cairo_user_text_to_glyphs
                    as unsafe extern "C" fn(
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
            ),
            ucs4_to_index: Some(
                _cairo_user_ucs4_to_index
                    as unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_ulong,
            ),
            load_truetype_table: None,
            index_to_ucs4: None,
            is_synthetic: None,
            index_to_glyph_name: None,
            load_type1_data: None,
            has_color_glyphs: Some(
                _cairo_user_has_color_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
        };
        init
    }
};
unsafe extern "C" fn _cairo_user_font_face_scaled_font_create(
    mut abstract_face: *mut libc::c_void,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
    mut scaled_font: *mut *mut cairo_scaled_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut font_face: *mut cairo_user_font_face_t = abstract_face
        as *mut cairo_user_font_face_t;
    let mut user_scaled_font: *mut cairo_user_scaled_font_t = 0
        as *mut cairo_user_scaled_font_t;
    let mut font_extents: cairo_font_extents_t = {
        let mut init = cairo_font_extents_t {
            ascent: 1.0f64,
            descent: 0.0f64,
            height: 1.0f64,
            max_x_advance: 1.0f64,
            max_y_advance: 0.0f64,
        };
        init
    };
    (*font_face).immutable = 1 as libc::c_int;
    user_scaled_font = (if ::std::mem::size_of::<cairo_user_scaled_font_t>()
        as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_user_scaled_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_user_scaled_font_t;
    if user_scaled_font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _cairo_scaled_font_init(
        &mut (*user_scaled_font).base,
        &mut (*font_face).base,
        font_matrix,
        ctm,
        options,
        &_cairo_user_scaled_font_backend,
    );
    if status as u64 != 0 {
        free(user_scaled_font as *mut libc::c_void);
        return status;
    }
    let mut fixed_scale: libc::c_double = 0.;
    let mut x_scale: libc::c_double = 0.;
    let mut y_scale: libc::c_double = 0.;
    (*user_scaled_font).extent_scale = (*user_scaled_font).base.scale_inverse;
    status = _cairo_matrix_compute_basis_scale_factors(
        &mut (*user_scaled_font).extent_scale,
        &mut x_scale,
        &mut y_scale,
        1 as libc::c_int,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        if x_scale == 0 as libc::c_int as libc::c_double {
            x_scale = 1.0f64;
        }
        if y_scale == 0 as libc::c_int as libc::c_double {
            y_scale = 1.0f64;
        }
        (*user_scaled_font).snap_x_scale = x_scale;
        (*user_scaled_font).snap_y_scale = y_scale;
        fixed_scale = 1024.0f64;
        x_scale /= fixed_scale;
        y_scale /= fixed_scale;
        cairo_matrix_scale(
            &mut (*user_scaled_font).extent_scale,
            1.0f64 / x_scale,
            1.0f64 / y_scale,
        );
        (*user_scaled_font).extent_x_scale = x_scale;
        (*user_scaled_font).extent_y_scale = y_scale;
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && ((*font_face).scaled_font_methods.init).is_some()
    {
        pthread_mutex_lock(&mut (*user_scaled_font).base.mutex);
        status = _cairo_scaled_font_register_placeholder_and_unlock_font_map(
            &mut (*user_scaled_font).base,
        );
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            let mut recording_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
            let mut cr: *mut cairo_t = 0 as *mut cairo_t;
            recording_surface = _cairo_user_scaled_font_create_recording_surface(
                user_scaled_font,
                0 as libc::c_int,
            );
            cr = _cairo_user_scaled_font_create_recording_context(
                user_scaled_font,
                recording_surface,
                0 as libc::c_int,
            );
            cairo_surface_destroy(recording_surface);
            status = ((*font_face).scaled_font_methods.init)
                .expect(
                    "non-null function pointer",
                )(&mut (*user_scaled_font).base, cr, &mut font_extents);
            if status as libc::c_uint
                == CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED as libc::c_int as libc::c_uint
            {
                status = CAIRO_STATUS_SUCCESS;
            }
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = cairo_status(cr);
            }
            cairo_destroy(cr);
            _cairo_scaled_font_unregister_placeholder_and_lock_font_map(
                &mut (*user_scaled_font).base,
            );
        }
        pthread_mutex_unlock(&mut (*user_scaled_font).base.mutex);
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _cairo_scaled_font_set_metrics(
            &mut (*user_scaled_font).base,
            &mut font_extents,
        );
    }
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        _cairo_scaled_font_fini(&mut (*user_scaled_font).base);
        free(user_scaled_font as *mut libc::c_void);
    } else {
        (*user_scaled_font).default_glyph_extents.x_bearing = 0.0f64;
        (*user_scaled_font).default_glyph_extents.y_bearing = -font_extents.ascent;
        (*user_scaled_font).default_glyph_extents.width = 0.0f64;
        (*user_scaled_font)
            .default_glyph_extents
            .height = font_extents.ascent + font_extents.descent;
        (*user_scaled_font).default_glyph_extents.x_advance = font_extents.max_x_advance;
        (*user_scaled_font).default_glyph_extents.y_advance = 0.0f64;
        *scaled_font = &mut (*user_scaled_font).base;
    }
    return status;
}
#[no_mangle]
pub static mut _cairo_user_font_face_backend: cairo_font_face_backend_t = unsafe {
    {
        let mut init = _cairo_font_face_backend {
            type_0: CAIRO_FONT_TYPE_USER,
            create_for_toy: Some(
                _cairo_user_font_face_create_for_toy
                    as unsafe extern "C" fn(
                        *mut cairo_toy_font_face_t,
                        *mut *mut cairo_font_face_t,
                    ) -> cairo_status_t,
            ),
            destroy: Some(
                _cairo_font_face_destroy
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            scaled_font_create: Some(
                _cairo_user_font_face_scaled_font_create
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        *const cairo_font_options_t,
                        *mut *mut cairo_scaled_font_t,
                    ) -> cairo_status_t,
            ),
            get_implementation: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_face_is_user(
    mut font_face: *mut cairo_font_face_t,
) -> cairo_bool_t {
    return ((*font_face).backend
        == &_cairo_user_font_face_backend as *const cairo_font_face_backend_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_create() -> *mut cairo_font_face_t {
    let mut font_face: *mut cairo_user_font_face_t = 0 as *mut cairo_user_font_face_t;
    font_face = (if ::std::mem::size_of::<cairo_user_font_face_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_user_font_face_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_user_font_face_t;
    if font_face.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    _cairo_font_face_init(&mut (*font_face).base, &_cairo_user_font_face_backend);
    (*font_face).immutable = 0 as libc::c_int;
    (*font_face).has_color = 0 as libc::c_int;
    memset(
        &mut (*font_face).scaled_font_methods as *mut cairo_user_scaled_font_methods_t
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_user_scaled_font_methods_t>() as libc::c_ulong,
    );
    return &mut (*font_face).base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_set_init_func(
    mut font_face: *mut cairo_font_face_t,
    mut init_func: cairo_user_scaled_font_init_func_t,
) {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    if (*user_font_face).immutable != 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_USER_FONT_IMMUTABLE) as u64
            != 0
        {
            return;
        }
    }
    let ref mut fresh0 = (*user_font_face).scaled_font_methods.init;
    *fresh0 = init_func;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_set_render_color_glyph_func(
    mut font_face: *mut cairo_font_face_t,
    mut render_glyph_func: cairo_user_scaled_font_render_glyph_func_t,
) {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    if (*user_font_face).immutable != 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_USER_FONT_IMMUTABLE) as u64
            != 0
        {
            return;
        }
    }
    let ref mut fresh1 = (*user_font_face).scaled_font_methods.render_color_glyph;
    *fresh1 = render_glyph_func;
    (*user_font_face)
        .has_color = if render_glyph_func.is_some() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_set_render_glyph_func(
    mut font_face: *mut cairo_font_face_t,
    mut render_glyph_func: cairo_user_scaled_font_render_glyph_func_t,
) {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    if (*user_font_face).immutable != 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_USER_FONT_IMMUTABLE) as u64
            != 0
        {
            return;
        }
    }
    let ref mut fresh2 = (*user_font_face).scaled_font_methods.render_glyph;
    *fresh2 = render_glyph_func;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_set_text_to_glyphs_func(
    mut font_face: *mut cairo_font_face_t,
    mut text_to_glyphs_func: cairo_user_scaled_font_text_to_glyphs_func_t,
) {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    if (*user_font_face).immutable != 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_USER_FONT_IMMUTABLE) as u64
            != 0
        {
            return;
        }
    }
    let ref mut fresh3 = (*user_font_face).scaled_font_methods.text_to_glyphs;
    *fresh3 = text_to_glyphs_func;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_set_unicode_to_glyph_func(
    mut font_face: *mut cairo_font_face_t,
    mut unicode_to_glyph_func: cairo_user_scaled_font_unicode_to_glyph_func_t,
) {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    if (*user_font_face).immutable != 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_USER_FONT_IMMUTABLE) as u64
            != 0
        {
            return;
        }
    }
    let ref mut fresh4 = (*user_font_face).scaled_font_methods.unicode_to_glyph;
    *fresh4 = unicode_to_glyph_func;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_get_init_func(
    mut font_face: *mut cairo_font_face_t,
) -> cairo_user_scaled_font_init_func_t {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return None;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return None;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    return (*user_font_face).scaled_font_methods.init;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_get_render_color_glyph_func(
    mut font_face: *mut cairo_font_face_t,
) -> cairo_user_scaled_font_render_glyph_func_t {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return None;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return None;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    return (*user_font_face).scaled_font_methods.render_color_glyph;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_get_render_glyph_func(
    mut font_face: *mut cairo_font_face_t,
) -> cairo_user_scaled_font_render_glyph_func_t {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return None;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return None;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    return (*user_font_face).scaled_font_methods.render_glyph;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_get_text_to_glyphs_func(
    mut font_face: *mut cairo_font_face_t,
) -> cairo_user_scaled_font_text_to_glyphs_func_t {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return None;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return None;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    return (*user_font_face).scaled_font_methods.text_to_glyphs;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_user_font_face_get_unicode_to_glyph_func(
    mut font_face: *mut cairo_font_face_t,
) -> cairo_user_scaled_font_unicode_to_glyph_func_t {
    let mut user_font_face: *mut cairo_user_font_face_t = 0
        as *mut cairo_user_font_face_t;
    if (*font_face).status as u64 != 0 {
        return None;
    }
    if _cairo_font_face_is_user(font_face) == 0 {
        if _cairo_font_face_set_error(font_face, CAIRO_STATUS_FONT_TYPE_MISMATCH) as u64
            != 0
        {
            return None;
        }
    }
    user_font_face = font_face as *mut cairo_user_font_face_t;
    return (*user_font_face).scaled_font_methods.unicode_to_glyph;
}
