use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
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
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
    fn cairo_scaled_font_create(
        font_face: *mut cairo_font_face_t,
        font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t,
        options: *const cairo_font_options_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> cairo_status_t;
    fn cairo_scaled_font_get_font_face(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_font_face_t;
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
    fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_user_data_array_fini(array: *mut cairo_user_data_array_t);
    fn _cairo_user_data_array_init(array: *mut cairo_user_data_array_t);
    static __cairo_clip_all: cairo_clip_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_observers_notify(observers: *mut cairo_list_t, arg: *mut libc::c_void);
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn _cairo_font_options_init_copy(
        options: *mut cairo_font_options_t,
        other: *const cairo_font_options_t,
    );
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_format_from_content(content: cairo_content_t) -> cairo_format_t;
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    fn _cairo_clip_intersect_rectangle(
        clip: *mut cairo_clip_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_clip_t;
    fn cairo_matrix_init_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
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
    fn _cairo_intern_string(
        str_inout: *mut *const libc::c_char,
        len: libc::c_int,
    ) -> cairo_status_t;
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn cairo_pattern_create_for_surface(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_pattern_t;
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    fn cairo_pattern_set_matrix(
        pattern: *mut cairo_pattern_t,
        matrix: *const cairo_matrix_t,
    );
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn cairo_matrix_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_user_data_array_copy(
        dst: *mut cairo_user_data_array_t,
        src: *const cairo_user_data_array_t,
    ) -> cairo_status_t;
    fn _cairo_user_data_array_foreach(
        array: *mut cairo_user_data_array_t,
        func: Option::<
            unsafe extern "C" fn(
                *const libc::c_void,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> (),
        >,
        closure: *mut libc::c_void,
    );
    fn _cairo_scaled_font_has_color_glyphs(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> cairo_bool_t;
    fn _cairo_operator_bounded_by_mask(op: cairo_operator_t) -> cairo_bool_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_set_error(
        scaled_font: *mut cairo_scaled_font_t,
        status: cairo_status_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_damage_add_box(
        damage: *mut cairo_damage_t,
        box_0: *const cairo_box_t,
    ) -> *mut cairo_damage_t;
    fn _cairo_damage_destroy(damage: *mut cairo_damage_t);
    fn _cairo_device_create_in_error(status: cairo_status_t) -> *mut cairo_device_t;
    fn _cairo_image_surface_clone_subimage(
        surface: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_pattern_init_solid(
        pattern: *mut cairo_solid_pattern_t,
        color: *const cairo_color_t,
    );
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_is_clear(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_damage {
    pub status: cairo_status_t,
    pub region: *mut cairo_region_t,
    pub dirty: libc::c_int,
    pub remain: libc::c_int,
    pub chunks: _cairo_damage_chunk,
    pub tail: *mut _cairo_damage_chunk,
    pub boxes: [cairo_box_t; 32],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_damage_chunk {
    pub next: *mut _cairo_damage_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
pub type cairo_mime_data_t = _cairo_mime_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mime_data {
    pub ref_count: cairo_reference_count_t,
    pub data: *mut libc::c_uchar,
    pub length: libc::c_ulong,
    pub destroy: cairo_destroy_func_t,
    pub closure: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_user_data_slot_t {
    pub key: *const cairo_user_data_key_t,
    pub user_data: *mut libc::c_void,
    pub destroy: cairo_destroy_func_t,
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_cmpxchg_impl(
    mut x: *mut cairo_atomic_int_t,
    mut oldv: cairo_atomic_int_t,
    mut newv: cairo_atomic_int_t,
) -> cairo_bool_t {
    let mut expected: cairo_atomic_int_t = oldv;
    let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh0.0;
    return fresh0.1 as cairo_bool_t;
}
#[inline]
unsafe extern "C" fn _cairo_rectangle_contains_rectangle(
    mut a: *const cairo_rectangle_int_t,
    mut b: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    return ((*a).x <= (*b).x && (*a).x + (*a).width >= (*b).x + (*b).width
        && (*a).y <= (*b).y && (*a).y + (*a).height >= (*b).y + (*b).height)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_unbounded_rectangle_init(
    mut rect: *mut cairo_rectangle_int_t,
) {
    *rect = _cairo_unbounded_rectangle;
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
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
unsafe extern "C" fn _cairo_clip_is_all_clipped(
    mut clip: *const cairo_clip_t,
) -> cairo_bool_t {
    return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh1 = (*entry).next;
    *fresh1 = entry;
    let ref mut fresh2 = (*entry).prev;
    *fresh2 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh3 = (*next).prev;
    *fresh3 = entry;
    let ref mut fresh4 = (*entry).next;
    *fresh4 = next;
    let ref mut fresh5 = (*entry).prev;
    *fresh5 = prev;
    let ref mut fresh6 = (*prev).next;
    *fresh6 = entry;
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
    let ref mut fresh7 = (*next).prev;
    *fresh7 = prev;
    let ref mut fresh8 = (*prev).next;
    *fresh8 = next;
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
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_image_surface_is_clone(
    mut image: *mut cairo_image_surface_t,
) -> cairo_bool_t {
    return ((*image).parent != 0 as *mut libc::c_void as *mut cairo_surface_t)
        as libc::c_int;
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
unsafe extern "C" fn __cairo_surface_flush(
    mut surface: *mut cairo_surface_t,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if ((*(*surface).backend).flush).is_some() {
        status = ((*(*surface).backend).flush)
            .expect("non-null function pointer")(surface as *mut libc::c_void, flags);
    }
    return status;
}
static mut _cairo_surface_nil: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_surface_type_mismatch: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_invalid_status: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_invalid_content: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_invalid_format: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_invalid_visual: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_file_not_found: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_temp_file_error: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_read_error: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_write_error: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_invalid_stride: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_invalid_size: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_device_type_mismatch: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_device_error: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_unsupported: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
static mut _cairo_surface_nil_nothing_to_do: cairo_surface_t = cairo_surface_t {
    backend: 0 as *const cairo_surface_backend_t,
    device: 0 as *const cairo_device_t as *mut cairo_device_t,
    type_0: CAIRO_SURFACE_TYPE_IMAGE,
    content: 0 as cairo_content_t,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    status: CAIRO_STATUS_SUCCESS,
    unique_id: 0,
    serial: 0,
    damage: 0 as *const cairo_damage_t as *mut cairo_damage_t,
    _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
    c2rust_padding: [0; 7],
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    mime_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    device_transform: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    device_transform_observers: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    x_resolution: 0.,
    y_resolution: 0.,
    x_fallback_resolution: 0.,
    y_fallback_resolution: 0.,
    snapshot_of: 0 as *const cairo_surface_t as *mut cairo_surface_t,
    snapshot_detach: None,
    snapshots: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    snapshot: cairo_list_t {
        next: 0 as *const _cairo_list as *mut _cairo_list,
        prev: 0 as *const _cairo_list as *mut _cairo_list,
    },
    font_options: cairo_font_options_t {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0,
    },
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_set_error(
    mut surface: *mut cairo_surface_t,
    mut status: cairo_int_status_t,
) -> cairo_int_status_t {
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        status = CAIRO_INT_STATUS_SUCCESS;
    }
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        || status as libc::c_uint
            >= CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status;
    }
    let mut ret__: libc::c_int = 0;
    if (status as cairo_status_t as libc::c_uint)
        < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"(cairo_status_t)status < CAIRO_STATUS_LAST_STATUS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"cairo_int_status_t _cairo_surface_set_error(cairo_surface_t *, cairo_int_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    ret__ = _cairo_atomic_int_cmpxchg_impl(
        &mut (*surface).status as *mut cairo_status_t as *mut cairo_atomic_int_t,
        CAIRO_STATUS_SUCCESS as libc::c_int,
        status as cairo_status_t as cairo_atomic_int_t,
    );
    return _cairo_error(status as cairo_status_t) as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_type(
    mut surface: *mut cairo_surface_t,
) -> cairo_surface_type_t {
    return (*surface).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_content(
    mut surface: *mut cairo_surface_t,
) -> cairo_content_t {
    return (*surface).content;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_status(
    mut surface: *mut cairo_surface_t,
) -> cairo_status_t {
    return (*surface).status;
}
unsafe extern "C" fn _cairo_surface_allocate_unique_id() -> libc::c_uint {
    static mut unique_id: cairo_atomic_int_t = 0;
    let mut old: cairo_atomic_int_t = 0;
    let mut id: cairo_atomic_int_t = 0;
    loop {
        old = _cairo_atomic_int_get(&mut unique_id);
        id = old + 1 as libc::c_int;
        if id == 0 as libc::c_int {
            id = 1 as libc::c_int;
        }
        if !(_cairo_atomic_int_cmpxchg_impl(
            &mut unique_id as *mut cairo_atomic_int_t,
            old,
            id,
        ) == 0)
        {
            break;
        }
    }
    return id as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_device(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_device_t {
    if (*surface).status as u64 != 0 {
        return _cairo_device_create_in_error((*surface).status);
    }
    return (*surface).device;
}
unsafe extern "C" fn _cairo_surface_has_snapshots(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return (cairo_list_is_empty(&mut (*surface).snapshots) == 0) as libc::c_int;
}
unsafe extern "C" fn _cairo_surface_has_mime_data(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*surface).mime_data.num_elements != 0 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_surface_detach_mime_data(mut surface: *mut cairo_surface_t) {
    if _cairo_surface_has_mime_data(surface) == 0 {
        return;
    }
    _cairo_user_data_array_fini(&mut (*surface).mime_data);
    _cairo_user_data_array_init(&mut (*surface).mime_data);
}
unsafe extern "C" fn _cairo_surface_detach_snapshots(mut surface: *mut cairo_surface_t) {
    while _cairo_surface_has_snapshots(surface) != 0 {
        _cairo_surface_detach_snapshot(
            ({
                let mut mptr__: *const cairo_list_t = (*surface).snapshots.next;
                (mptr__ as *mut libc::c_char).offset(-(280 as libc::c_ulong as isize))
                    as *mut cairo_surface_t
            }),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_detach_snapshot(
    mut snapshot: *mut cairo_surface_t,
) {
    if !((*snapshot).snapshot_of).is_null() {} else {
        __assert_fail(
            b"snapshot->snapshot_of != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void _cairo_surface_detach_snapshot(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh9 = (*snapshot).snapshot_of;
    *fresh9 = 0 as *mut cairo_surface_t;
    cairo_list_del(&mut (*snapshot).snapshot);
    if ((*snapshot).snapshot_detach).is_some() {
        ((*snapshot).snapshot_detach).expect("non-null function pointer")(snapshot);
    }
    cairo_surface_destroy(snapshot);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_attach_snapshot(
    mut surface: *mut cairo_surface_t,
    mut snapshot: *mut cairo_surface_t,
    mut detach_func: cairo_surface_func_t,
) {
    if surface != snapshot {} else {
        __assert_fail(
            b"surface != snapshot\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void _cairo_surface_attach_snapshot(cairo_surface_t *, cairo_surface_t *, cairo_surface_func_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*snapshot).snapshot_of != surface {} else {
        __assert_fail(
            b"snapshot->snapshot_of != surface\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void _cairo_surface_attach_snapshot(cairo_surface_t *, cairo_surface_t *, cairo_surface_func_t)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_surface_reference(snapshot);
    if !((*snapshot).snapshot_of).is_null() {
        _cairo_surface_detach_snapshot(snapshot);
    }
    let ref mut fresh10 = (*snapshot).snapshot_of;
    *fresh10 = surface;
    let ref mut fresh11 = (*snapshot).snapshot_detach;
    *fresh11 = detach_func;
    cairo_list_add(&mut (*snapshot).snapshot, &mut (*surface).snapshots);
    if _cairo_surface_has_snapshot(surface, (*snapshot).backend) == snapshot {} else {
        __assert_fail(
            b"_cairo_surface_has_snapshot (surface, snapshot->backend) == snapshot\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void _cairo_surface_attach_snapshot(cairo_surface_t *, cairo_surface_t *, cairo_surface_func_t)\0",
            ))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_has_snapshot(
    mut surface: *mut cairo_surface_t,
    mut backend: *const cairo_surface_backend_t,
) -> *mut cairo_surface_t {
    let mut snapshot: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    snapshot = ({
        let mut mptr__: *const cairo_list_t = (*surface).snapshots.next;
        (mptr__ as *mut libc::c_char).offset(-(280 as libc::c_ulong as isize))
            as *mut cairo_surface_t
    });
    while &mut (*snapshot).snapshot as *mut cairo_list_t
        != &mut (*surface).snapshots as *mut cairo_list_t
    {
        if (*snapshot).backend == backend {
            return snapshot;
        }
        snapshot = ({
            let mut mptr__: *const cairo_list_t = (*snapshot).snapshot.next;
            (mptr__ as *mut libc::c_char).offset(-(280 as libc::c_ulong as isize))
                as *mut cairo_surface_t
        });
    }
    return 0 as *mut cairo_surface_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_begin_modification(
    mut surface: *mut cairo_surface_t,
) -> cairo_status_t {
    if (*surface).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"surface->status == CAIRO_STATUS_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"cairo_status_t _cairo_surface_begin_modification(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if (*surface).finished() == 0 {} else {
        __assert_fail(
            b"! surface->finished\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"cairo_status_t _cairo_surface_begin_modification(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    return _cairo_surface_flush(surface, 1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_init(
    mut surface: *mut cairo_surface_t,
    mut backend: *const cairo_surface_backend_t,
    mut device: *mut cairo_device_t,
    mut content: cairo_content_t,
    mut is_vector: cairo_bool_t,
) {
    let ref mut fresh12 = (*surface).backend;
    *fresh12 = backend;
    let ref mut fresh13 = (*surface).device;
    *fresh13 = cairo_device_reference(device);
    (*surface).content = content;
    (*surface).type_0 = (*backend).type_0;
    (*surface).set_is_vector(is_vector as libc::c_uint);
    (*surface).ref_count.ref_count = 1 as libc::c_int;
    (*surface).status = CAIRO_STATUS_SUCCESS;
    (*surface).unique_id = _cairo_surface_allocate_unique_id();
    (*surface).set_finished(0 as libc::c_int as libc::c_uint);
    (*surface).set__finishing(0 as libc::c_int as libc::c_uint);
    (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
    (*surface).serial = 0 as libc::c_int as libc::c_uint;
    let ref mut fresh14 = (*surface).damage;
    *fresh14 = 0 as *mut cairo_damage_t;
    (*surface)
        .set_owns_device(
            (device != 0 as *mut libc::c_void as *mut cairo_device_t) as libc::c_int
                as libc::c_uint,
        );
    _cairo_user_data_array_init(&mut (*surface).user_data);
    _cairo_user_data_array_init(&mut (*surface).mime_data);
    cairo_matrix_init_identity(&mut (*surface).device_transform);
    cairo_matrix_init_identity(&mut (*surface).device_transform_inverse);
    cairo_list_init(&mut (*surface).device_transform_observers);
    (*surface).x_resolution = 72.0f64;
    (*surface).y_resolution = 72.0f64;
    (*surface).x_fallback_resolution = 300.0f64;
    (*surface).y_fallback_resolution = 300.0f64;
    cairo_list_init(&mut (*surface).snapshots);
    let ref mut fresh15 = (*surface).snapshot_of;
    *fresh15 = 0 as *mut cairo_surface_t;
    (*surface).set_has_font_options(0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn _cairo_surface_copy_similar_properties(
    mut surface: *mut cairo_surface_t,
    mut other: *mut cairo_surface_t,
) {
    if (*other).has_font_options() as libc::c_int != 0
        || (*other).backend != (*surface).backend
    {
        let mut options: cairo_font_options_t = cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *const libc::c_char as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        };
        cairo_surface_get_font_options(other, &mut options);
        _cairo_surface_set_font_options(surface, &mut options);
    }
    cairo_surface_set_fallback_resolution(
        surface,
        (*other).x_fallback_resolution,
        (*other).y_fallback_resolution,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_create_similar(
    mut other: *mut cairo_surface_t,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut pattern: cairo_solid_pattern_t = cairo_solid_pattern_t {
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
    if (*other).status as u64 != 0 {
        return _cairo_surface_create_in_error((*other).status);
    }
    if (*other).finished() != 0 {
        return _cairo_surface_create_in_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    if width < 0 as libc::c_int || height < 0 as libc::c_int {
        return _cairo_surface_create_in_error(CAIRO_STATUS_INVALID_SIZE);
    }
    if !(content as libc::c_uint != 0
        && content as libc::c_uint
            & !(CAIRO_CONTENT_COLOR as libc::c_int | CAIRO_CONTENT_ALPHA as libc::c_int
                | CAIRO_CONTENT_COLOR_ALPHA as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
    {
        return _cairo_surface_create_in_error(CAIRO_STATUS_INVALID_CONTENT);
    }
    width = (width as libc::c_double * (*other).device_transform.xx) as libc::c_int;
    height = (height as libc::c_double * (*other).device_transform.yy) as libc::c_int;
    surface = 0 as *mut cairo_surface_t;
    if ((*(*other).backend).create_similar).is_some() {
        surface = ((*(*other).backend).create_similar)
            .expect(
                "non-null function pointer",
            )(other as *mut libc::c_void, content, width, height);
    }
    if surface.is_null() {
        surface = cairo_surface_create_similar_image(
            other,
            _cairo_format_from_content(content),
            width,
            height,
        );
    }
    if (*surface).status as u64 != 0 {
        return surface;
    }
    _cairo_surface_copy_similar_properties(surface, other);
    cairo_surface_set_device_scale(
        surface,
        (*other).device_transform.xx,
        (*other).device_transform.yy,
    );
    if (*surface).status as u64 != 0 {
        return surface;
    }
    _cairo_pattern_init_solid(&mut pattern, _cairo_stock_color(CAIRO_STOCK_TRANSPARENT));
    status = _cairo_surface_paint(
        surface,
        CAIRO_OPERATOR_CLEAR,
        &mut pattern.base,
        0 as *const cairo_clip_t,
    );
    if status as u64 != 0 {
        cairo_surface_destroy(surface);
        surface = _cairo_surface_create_in_error(status);
    }
    if (*surface).is_clear() != 0 {} else {
        __assert_fail(
            b"surface->is_clear\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"cairo_surface_t *cairo_surface_create_similar(cairo_surface_t *, cairo_content_t, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_create_similar_image(
    mut other: *mut cairo_surface_t,
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if (*other).status as u64 != 0 {
        return _cairo_surface_create_in_error((*other).status);
    }
    if (*other).finished() != 0 {
        return _cairo_surface_create_in_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    if width < 0 as libc::c_int || height < 0 as libc::c_int {
        return _cairo_surface_create_in_error(CAIRO_STATUS_INVALID_SIZE);
    }
    if !(format as libc::c_int >= CAIRO_FORMAT_ARGB32 as libc::c_int
        && format as libc::c_int <= CAIRO_FORMAT_RGBA128F as libc::c_int)
    {
        return _cairo_surface_create_in_error(CAIRO_STATUS_INVALID_FORMAT);
    }
    image = 0 as *mut cairo_surface_t;
    if ((*(*other).backend).create_similar_image).is_some() {
        image = ((*(*other).backend).create_similar_image)
            .expect(
                "non-null function pointer",
            )(other as *mut libc::c_void, format, width, height);
    }
    if image.is_null() {
        image = cairo_image_surface_create(format, width, height);
    }
    if (*image).is_clear() != 0 {} else {
        __assert_fail(
            b"image->is_clear\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            600 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_surface_t *cairo_surface_create_similar_image(cairo_surface_t *, cairo_format_t, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_map_to_image(
    mut surface: *mut cairo_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    if !extents.is_null() {} else {
        __assert_fail(
            b"extents != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            642 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"cairo_image_surface_t *_cairo_surface_map_to_image(cairo_surface_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*(*surface).backend).map_to_image).is_some() {
        image = ((*(*surface).backend).map_to_image)
            .expect("non-null function pointer")(surface as *mut libc::c_void, extents);
    }
    if image.is_null() {
        image = _cairo_image_surface_clone_subimage(surface, extents);
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_unmap_image(
    mut surface: *mut cairo_surface_t,
    mut image: *mut cairo_image_surface_t,
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
    let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*image).base.status as u64 != 0 {
        status = (*image).base.status as cairo_int_status_t;
    } else if (*image).base.serial == 0 as libc::c_int as libc::c_uint {
        status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    } else {
        if ((*(*surface).backend).unmap_image).is_some()
            && _cairo_image_surface_is_clone(image) == 0
        {
            status = ((*(*surface).backend).unmap_image)
                .expect(
                    "non-null function pointer",
                )(surface as *mut libc::c_void, image);
            if status as libc::c_uint
                != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
            {
                return status;
            }
        }
        _cairo_pattern_init_for_surface(&mut pattern, &mut (*image).base);
        pattern.base.filter = CAIRO_FILTER_NEAREST;
        cairo_matrix_init_translate(
            &mut pattern.base.matrix,
            (*image).base.device_transform.x0,
            (*image).base.device_transform.y0,
        );
        extents.x = (*image).base.device_transform_inverse.x0 as libc::c_int;
        extents.y = (*image).base.device_transform_inverse.y0 as libc::c_int;
        extents.width = (*image).width;
        extents.height = (*image).height;
        clip = _cairo_clip_intersect_rectangle(0 as *mut cairo_clip_t, &mut extents);
        status = _cairo_surface_paint(
            surface,
            CAIRO_OPERATOR_SOURCE,
            &mut pattern.base,
            clip,
        ) as cairo_int_status_t;
        _cairo_pattern_fini(&mut pattern.base);
        _cairo_clip_destroy(clip);
    }
    cairo_surface_finish(&mut (*image).base);
    cairo_surface_destroy(&mut (*image).base);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_map_to_image(
    mut surface: *mut cairo_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return _cairo_surface_create_in_error((*surface).status);
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_create_in_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    if extents.is_null() {
        if ((*(*surface).backend).get_extents)
            .expect("non-null function pointer")(surface as *mut libc::c_void, &mut rect)
            == 0
        {
            return _cairo_surface_create_in_error(CAIRO_STATUS_INVALID_SIZE);
        }
        extents = &mut rect;
    } else {
        let mut surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        if ((*(*surface).backend).get_extents)
            .expect(
                "non-null function pointer",
            )(surface as *mut libc::c_void, &mut surface_extents) != 0
        {
            if _cairo_rectangle_contains_rectangle(&mut surface_extents, extents) == 0 {
                return _cairo_surface_create_in_error(CAIRO_STATUS_INVALID_SIZE);
            }
        }
    }
    image = _cairo_surface_map_to_image(surface, extents);
    status = (*image).base.status;
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*image).base);
        return _cairo_surface_create_in_error(status);
    }
    if (*image).format as libc::c_int == CAIRO_FORMAT_INVALID as libc::c_int {
        cairo_surface_destroy(&mut (*image).base);
        image = _cairo_image_surface_clone_subimage(surface, extents);
    }
    return &mut (*image).base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_unmap_image(
    mut surface: *mut cairo_surface_t,
    mut image: *mut cairo_surface_t,
) {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    if (*surface).status as u64 != 0 {
        status = (*surface).status as cairo_int_status_t;
    } else if (*surface).finished() != 0 {
        status = _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t;
    } else if (*image).status as u64 != 0 {
        status = (*image).status as cairo_int_status_t;
    } else if (*image).finished() != 0 {
        status = _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t;
    } else if _cairo_surface_is_image(image) == 0 {
        status = _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t;
    } else {
        status = _cairo_surface_unmap_image(
            surface,
            image as *mut cairo_image_surface_t,
        );
        if status as u64 != 0 {
            _cairo_surface_set_error(surface, status);
        }
        return;
    }
    _cairo_surface_set_error(surface, status);
    cairo_surface_finish(image);
    cairo_surface_destroy(image);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_create_scratch(
    mut other: *mut cairo_surface_t,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut color: *const cairo_color_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut pattern: cairo_solid_pattern_t = cairo_solid_pattern_t {
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
    if (*other).status as u64 != 0 {
        return _cairo_surface_create_in_error((*other).status);
    }
    surface = 0 as *mut cairo_surface_t;
    if ((*(*other).backend).create_similar).is_some() {
        surface = ((*(*other).backend).create_similar)
            .expect(
                "non-null function pointer",
            )(other as *mut libc::c_void, content, width, height);
    }
    if surface.is_null() {
        surface = cairo_surface_create_similar_image(
            other,
            _cairo_format_from_content(content),
            width,
            height,
        );
    }
    if (*surface).status as u64 != 0 {
        return surface;
    }
    _cairo_surface_copy_similar_properties(surface, other);
    if (*surface).status as u64 != 0 {
        return surface;
    }
    if !color.is_null() {
        _cairo_pattern_init_solid(&mut pattern, color);
        status = _cairo_surface_paint(
            surface,
            (if color == _cairo_stock_color(CAIRO_STOCK_TRANSPARENT) {
                CAIRO_OPERATOR_CLEAR as libc::c_int
            } else {
                CAIRO_OPERATOR_SOURCE as libc::c_int
            }) as cairo_operator_t,
            &mut pattern.base,
            0 as *const cairo_clip_t,
        );
        if status as u64 != 0 {
            cairo_surface_destroy(surface);
            surface = _cairo_surface_create_in_error(status);
        }
    }
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_reference(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    if surface.is_null()
        || _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return surface;
    }
    if _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&surface->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            930 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"cairo_surface_t *cairo_surface_reference(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    ::std::intrinsics::atomic_xadd(
        &mut (*surface).ref_count.ref_count,
        1 as libc::c_int,
    );
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_destroy(mut surface: *mut cairo_surface_t) {
    if surface.is_null()
        || _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return;
    }
    if _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&surface->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            955 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void cairo_surface_destroy(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if !(::std::intrinsics::atomic_xsub(
        &mut (*surface).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            960 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void cairo_surface_destroy(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if (*surface).finished() == 0 {
        _cairo_surface_finish_snapshots(surface);
        if _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) != 0 {
            return;
        }
        _cairo_surface_finish(surface);
    }
    if !((*surface).damage).is_null() {
        _cairo_damage_destroy((*surface).damage);
    }
    _cairo_user_data_array_fini(&mut (*surface).user_data);
    _cairo_user_data_array_fini(&mut (*surface).mime_data);
    if (*surface).owns_device() != 0 {
        cairo_device_destroy((*surface).device);
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            982 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void cairo_surface_destroy(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if _cairo_surface_has_snapshots(surface) == 0 {} else {
        __assert_fail(
            b"! _cairo_surface_has_snapshots (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            983 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void cairo_surface_destroy(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int)
    {} else {
        __assert_fail(
            b"! CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&surface->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            985 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void cairo_surface_destroy(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    free(surface as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_reference_count(
    mut surface: *mut cairo_surface_t,
) -> libc::c_uint {
    if surface.is_null()
        || _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    return _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) as libc::c_uint;
}
unsafe extern "C" fn _cairo_surface_finish_snapshots(mut surface: *mut cairo_surface_t) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*surface).set__finishing(1 as libc::c_int as libc::c_uint);
    status = _cairo_surface_flush(surface, 0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn _cairo_surface_finish(mut surface: *mut cairo_surface_t) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if ((*(*surface).backend).finish).is_some() {
        status = ((*(*surface).backend).finish)
            .expect("non-null function pointer")(surface as *mut libc::c_void);
        if status as u64 != 0 {
            _cairo_surface_set_error(surface, status as cairo_int_status_t);
        }
    }
    (*surface).set_finished(1 as libc::c_int as libc::c_uint);
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1037 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void _cairo_surface_finish(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if _cairo_surface_has_snapshots(surface) == 0 {} else {
        __assert_fail(
            b"!_cairo_surface_has_snapshots (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1038 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void _cairo_surface_finish(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_finish(mut surface: *mut cairo_surface_t) {
    if surface.is_null() {
        return;
    }
    if _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) == -(1 as libc::c_int)
    {
        return;
    }
    if (*surface).finished() != 0 {
        return;
    }
    cairo_surface_reference(surface);
    _cairo_surface_finish_snapshots(surface);
    _cairo_surface_finish(surface);
    cairo_surface_destroy(surface);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_release_device_reference(
    mut surface: *mut cairo_surface_t,
) {
    if (*surface).owns_device() != 0 {} else {
        __assert_fail(
            b"surface->owns_device\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void _cairo_surface_release_device_reference(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    cairo_device_destroy((*surface).device);
    (*surface).set_owns_device(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_user_data(
    mut surface: *mut cairo_surface_t,
    mut key: *const cairo_user_data_key_t,
) -> *mut libc::c_void {
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int) {
        return 0 as *mut libc::c_void;
    }
    return _cairo_user_data_array_get_data(&mut (*surface).user_data, key);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_set_user_data(
    mut surface: *mut cairo_surface_t,
    mut key: *const cairo_user_data_key_t,
    mut user_data: *mut libc::c_void,
    mut destroy: cairo_destroy_func_t,
) -> cairo_status_t {
    if _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) == -(1 as libc::c_int)
    {
        return (*surface).status;
    }
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int) {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    return _cairo_user_data_array_set_data(
        &mut (*surface).user_data,
        key,
        user_data,
        destroy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_mime_data(
    mut surface: *mut cairo_surface_t,
    mut mime_type: *const libc::c_char,
    mut data: *mut *const libc::c_uchar,
    mut length: *mut libc::c_ulong,
) {
    let mut slots: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
    let mut i: libc::c_int = 0;
    let mut num_slots: libc::c_int = 0;
    *data = 0 as *const libc::c_uchar;
    *length = 0 as libc::c_int as libc::c_ulong;
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int) {
        return;
    }
    num_slots = (*surface).mime_data.num_elements as libc::c_int;
    slots = _cairo_array_index(
        &mut (*surface).mime_data,
        0 as libc::c_int as libc::c_uint,
    ) as *mut cairo_user_data_slot_t;
    i = 0 as libc::c_int;
    while i < num_slots {
        if !((*slots.offset(i as isize)).key).is_null()
            && strcmp((*slots.offset(i as isize)).key as *mut libc::c_char, mime_type)
                == 0 as libc::c_int
        {
            let mut mime_data: *mut cairo_mime_data_t = (*slots.offset(i as isize))
                .user_data as *mut cairo_mime_data_t;
            *data = (*mime_data).data;
            *length = (*mime_data).length;
            return;
        }
        i += 1;
    }
}
unsafe extern "C" fn _cairo_mime_data_destroy(mut ptr: *mut libc::c_void) {
    let mut mime_data: *mut cairo_mime_data_t = ptr as *mut cairo_mime_data_t;
    if !(::std::intrinsics::atomic_xsub(
        &mut (*mime_data).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        return;
    }
    if ((*mime_data).destroy).is_some() && !((*mime_data).closure).is_null() {
        ((*mime_data).destroy).expect("non-null function pointer")((*mime_data).closure);
    }
    free(mime_data as *mut libc::c_void);
}
static mut _cairo_surface_image_mime_types: [*const libc::c_char; 5] = [
    b"image/jpeg\0" as *const u8 as *const libc::c_char,
    b"image/png\0" as *const u8 as *const libc::c_char,
    b"image/jp2\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.jbig2\0" as *const u8 as *const libc::c_char,
    b"image/g3fax\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_has_mime_image(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    let mut slots: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num_slots: libc::c_int = 0;
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int) {
        return 0 as libc::c_int;
    }
    num_slots = (*surface).mime_data.num_elements as libc::c_int;
    slots = _cairo_array_index(
        &mut (*surface).mime_data,
        0 as libc::c_int as libc::c_uint,
    ) as *mut cairo_user_data_slot_t;
    i = 0 as libc::c_int;
    while i < num_slots {
        if !((*slots.offset(i as isize)).key).is_null() {
            j = 0 as libc::c_int;
            while j
                < (::std::mem::size_of::<[*const libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ) as libc::c_int
            {
                if strcmp(
                    (*slots.offset(i as isize)).key as *mut libc::c_char,
                    _cairo_surface_image_mime_types[j as usize],
                ) == 0 as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                j += 1;
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_set_mime_data(
    mut surface: *mut cairo_surface_t,
    mut mime_type: *const libc::c_char,
    mut data: *const libc::c_uchar,
    mut length: libc::c_ulong,
    mut destroy: cairo_destroy_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut mime_data: *mut cairo_mime_data_t = 0 as *mut cairo_mime_data_t;
    if _cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) == -(1 as libc::c_int)
    {
        return (*surface).status;
    }
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count) > 0 as libc::c_int) {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    status = _cairo_intern_string(&mut mime_type, -(1 as libc::c_int));
    if status as u64 != 0 {
        return _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
    if !data.is_null() {
        mime_data = (if ::std::mem::size_of::<cairo_mime_data_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_mime_data_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_mime_data_t;
        if mime_data.is_null() {
            return _cairo_surface_set_error(
                surface,
                _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t,
            ) as cairo_status_t;
        }
        (*mime_data).ref_count.ref_count = 1 as libc::c_int;
        let ref mut fresh16 = (*mime_data).data;
        *fresh16 = data as *mut libc::c_uchar;
        (*mime_data).length = length;
        let ref mut fresh17 = (*mime_data).destroy;
        *fresh17 = destroy;
        let ref mut fresh18 = (*mime_data).closure;
        *fresh18 = closure;
    } else {
        mime_data = 0 as *mut cairo_mime_data_t;
    }
    status = _cairo_user_data_array_set_data(
        &mut (*surface).mime_data,
        mime_type as *mut cairo_user_data_key_t,
        mime_data as *mut libc::c_void,
        Some(_cairo_mime_data_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if status as u64 != 0 {
        free(mime_data as *mut libc::c_void);
        return _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
    (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_supports_mime_type(
    mut surface: *mut cairo_surface_t,
    mut mime_type: *const libc::c_char,
) -> cairo_bool_t {
    let mut types: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if (*surface).status as u64 != 0 {
        return 0 as libc::c_int;
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return 0 as libc::c_int;
    }
    if ((*(*surface).backend).get_supported_mime_types).is_some() {
        types = ((*(*surface).backend).get_supported_mime_types)
            .expect("non-null function pointer")(surface as *mut libc::c_void);
        if !types.is_null() {
            while !(*types).is_null() {
                if strcmp(*types, mime_type) == 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
                types = types.offset(1);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_mime_data_reference(
    mut key: *const libc::c_void,
    mut elt: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut mime_data: *mut cairo_mime_data_t = elt as *mut cairo_mime_data_t;
    ::std::intrinsics::atomic_xadd(
        &mut (*mime_data).ref_count.ref_count,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_copy_mime_data(
    mut dst: *mut cairo_surface_t,
    mut src: *mut cairo_surface_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*dst).status as u64 != 0 {
        return (*dst).status;
    }
    if (*src).status as u64 != 0 {
        return _cairo_surface_set_error(dst, (*src).status as cairo_int_status_t)
            as cairo_status_t;
    }
    status = _cairo_user_data_array_copy(&mut (*dst).mime_data, &mut (*src).mime_data);
    if status as u64 != 0 {
        return _cairo_surface_set_error(dst, status as cairo_int_status_t)
            as cairo_status_t;
    }
    _cairo_user_data_array_foreach(
        &mut (*dst).mime_data,
        Some(
            _cairo_mime_data_reference
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    (*dst).set_is_clear(0 as libc::c_int as libc::c_uint);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_set_font_options(
    mut surface: *mut cairo_surface_t,
    mut options: *mut cairo_font_options_t,
) {
    if (*surface).status as u64 != 0 {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1567 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void _cairo_surface_set_font_options(cairo_surface_t *, cairo_font_options_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    if !options.is_null() {
        (*surface).set_has_font_options(1 as libc::c_int as libc::c_uint);
        _cairo_font_options_init_copy(&mut (*surface).font_options, options);
    } else {
        (*surface).set_has_font_options(0 as libc::c_int as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_font_options(
    mut surface: *mut cairo_surface_t,
    mut options: *mut cairo_font_options_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    if (*surface).status as u64 != 0 {
        _cairo_font_options_init_default(options);
        return;
    }
    if (*surface).has_font_options() == 0 {
        (*surface).set_has_font_options(1 as libc::c_int as libc::c_uint);
        _cairo_font_options_init_default(&mut (*surface).font_options);
        if (*surface).finished() == 0
            && ((*(*surface).backend).get_font_options).is_some()
        {
            ((*(*surface).backend).get_font_options)
                .expect(
                    "non-null function pointer",
                )(surface as *mut libc::c_void, &mut (*surface).font_options);
        }
    }
    _cairo_font_options_init_copy(options, &mut (*surface).font_options);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_flush(
    mut surface: *mut cairo_surface_t,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    _cairo_surface_detach_snapshots(surface);
    if !((*surface).snapshot_of).is_null() {
        _cairo_surface_detach_snapshot(surface);
    }
    _cairo_surface_detach_mime_data(surface);
    return __cairo_surface_flush(surface, flags);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_flush(mut surface: *mut cairo_surface_t) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return;
    }
    if (*surface).finished() != 0 {
        return;
    }
    status = _cairo_surface_flush(surface, 0 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        _cairo_surface_set_error(surface, status as cairo_int_status_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_mark_dirty(mut surface: *mut cairo_surface_t) {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*surface).status as u64 != 0 {
        return;
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    _cairo_surface_get_extents(surface, &mut extents);
    cairo_surface_mark_dirty_rectangle(
        surface,
        extents.x,
        extents.y,
        extents.width,
        extents.height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_mark_dirty_rectangle(
    mut surface: *mut cairo_surface_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1723 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void cairo_surface_mark_dirty_rectangle(cairo_surface_t *, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    if _cairo_surface_has_snapshots(surface) == 0 {} else {
        __assert_fail(
            b"! _cairo_surface_has_snapshots (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1733 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void cairo_surface_mark_dirty_rectangle(cairo_surface_t *, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_surface_has_mime_data(surface) == 0 {} else {
        __assert_fail(
            b"! _cairo_surface_has_mime_data (surface)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1734 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void cairo_surface_mark_dirty_rectangle(cairo_surface_t *, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
    let ref mut fresh19 = (*surface).serial;
    *fresh19 = (*fresh19).wrapping_add(1);
    if !((*surface).damage).is_null() {
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        box_0.p1.x = x;
        box_0.p1.y = y;
        box_0.p2.x = x + width;
        box_0.p2.y = y + height;
        let ref mut fresh20 = (*surface).damage;
        *fresh20 = _cairo_damage_add_box((*surface).damage, &mut box_0);
    }
    if ((*(*surface).backend).mark_dirty_rectangle).is_some() {
        status = ((*(*surface).backend).mark_dirty_rectangle)
            .expect(
                "non-null function pointer",
            )(
            surface as *mut libc::c_void,
            (x as libc::c_double + (*surface).device_transform.x0) as libc::c_int,
            (y as libc::c_double + (*surface).device_transform.y0) as libc::c_int,
            width,
            height,
        );
        if status as u64 != 0 {
            _cairo_surface_set_error(surface, status as cairo_int_status_t);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_set_device_scale(
    mut surface: *mut cairo_surface_t,
    mut x_scale: libc::c_double,
    mut y_scale: libc::c_double,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1796 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void cairo_surface_set_device_scale(cairo_surface_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    status = _cairo_surface_begin_modification(surface);
    if status as u64 != 0 {
        _cairo_surface_set_error(surface, status as cairo_int_status_t);
        return;
    }
    (*surface).device_transform.xx = x_scale;
    (*surface).device_transform.yy = y_scale;
    (*surface).device_transform.xy = 0.0f64;
    (*surface).device_transform.yx = 0.0f64;
    (*surface).device_transform_inverse = (*surface).device_transform;
    status = cairo_matrix_invert(&mut (*surface).device_transform_inverse);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1817 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void cairo_surface_set_device_scale(cairo_surface_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_observers_notify(
        &mut (*surface).device_transform_observers,
        surface as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_device_scale(
    mut surface: *mut cairo_surface_t,
    mut x_scale: *mut libc::c_double,
    mut y_scale: *mut libc::c_double,
) {
    if !x_scale.is_null() {
        *x_scale = (*surface).device_transform.xx;
    }
    if !y_scale.is_null() {
        *y_scale = (*surface).device_transform.yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_set_device_offset(
    mut surface: *mut cairo_surface_t,
    mut x_offset: libc::c_double,
    mut y_offset: libc::c_double,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1876 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void cairo_surface_set_device_offset(cairo_surface_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    status = _cairo_surface_begin_modification(surface);
    if status as u64 != 0 {
        _cairo_surface_set_error(surface, status as cairo_int_status_t);
        return;
    }
    (*surface).device_transform.x0 = x_offset;
    (*surface).device_transform.y0 = y_offset;
    (*surface).device_transform_inverse = (*surface).device_transform;
    status = cairo_matrix_invert(&mut (*surface).device_transform_inverse);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1895 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void cairo_surface_set_device_offset(cairo_surface_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_observers_notify(
        &mut (*surface).device_transform_observers,
        surface as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_device_offset(
    mut surface: *mut cairo_surface_t,
    mut x_offset: *mut libc::c_double,
    mut y_offset: *mut libc::c_double,
) {
    if !x_offset.is_null() {
        *x_offset = (*surface).device_transform.x0;
    }
    if !y_offset.is_null() {
        *y_offset = (*surface).device_transform.y0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_set_fallback_resolution(
    mut surface: *mut cairo_surface_t,
    mut x_pixels_per_inch: libc::c_double,
    mut y_pixels_per_inch: libc::c_double,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            1967 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void cairo_surface_set_fallback_resolution(cairo_surface_t *, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    if x_pixels_per_inch <= 0 as libc::c_int as libc::c_double
        || y_pixels_per_inch <= 0 as libc::c_int as libc::c_double
    {
        _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_INVALID_MATRIX as libc::c_int as cairo_int_status_t,
        );
        return;
    }
    status = _cairo_surface_begin_modification(surface);
    if status as u64 != 0 {
        _cairo_surface_set_error(surface, status as cairo_int_status_t);
        return;
    }
    (*surface).x_fallback_resolution = x_pixels_per_inch;
    (*surface).y_fallback_resolution = y_pixels_per_inch;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_get_fallback_resolution(
    mut surface: *mut cairo_surface_t,
    mut x_pixels_per_inch: *mut libc::c_double,
    mut y_pixels_per_inch: *mut libc::c_double,
) {
    if !x_pixels_per_inch.is_null() {
        *x_pixels_per_inch = (*surface).x_fallback_resolution;
    }
    if !y_pixels_per_inch.is_null() {
        *y_pixels_per_inch = (*surface).y_fallback_resolution;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_has_device_transform(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return (_cairo_matrix_is_identity(&mut (*surface).device_transform) == 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_acquire_source_image(
    mut surface: *mut cairo_surface_t,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() == 0 {} else {
        __assert_fail(
            b"!surface->finished\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            2049 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"cairo_status_t _cairo_surface_acquire_source_image(cairo_surface_t *, cairo_image_surface_t **, void **)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*(*surface).backend).acquire_source_image).is_none() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = ((*(*surface).backend).acquire_source_image)
        .expect(
            "non-null function pointer",
        )(surface as *mut libc::c_void, image_out, image_extra);
    if status as u64 != 0 {
        return _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_default_acquire_source_image(
    mut _surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_surface_t = _surface as *mut cairo_surface_t;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if ((*(*surface).backend).get_extents)
        .expect("non-null function pointer")(surface as *mut libc::c_void, &mut extents)
        == 0
    {
        return _cairo_error(CAIRO_STATUS_INVALID_SIZE);
    }
    *image_out = _cairo_surface_map_to_image(surface, &mut extents);
    *image_extra = 0 as *mut libc::c_void;
    return (**image_out).base.status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_release_source_image(
    mut surface: *mut cairo_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    if (*surface).finished() == 0 {} else {
        __assert_fail(
            b"!surface->finished\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            2092 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"void _cairo_surface_release_source_image(cairo_surface_t *, cairo_image_surface_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*(*surface).backend).release_source_image).is_some() {
        ((*(*surface).backend).release_source_image)
            .expect(
                "non-null function pointer",
            )(surface as *mut libc::c_void, image, image_extra);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_default_release_source_image(
    mut surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    let mut ignored: cairo_status_t = CAIRO_STATUS_SUCCESS;
    ignored = _cairo_surface_unmap_image(surface as *mut cairo_surface_t, image)
        as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_get_source(
    mut surface: *mut cairo_surface_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    if ((*(*surface).backend).source).is_some() {} else {
        __assert_fail(
            b"surface->backend->source\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            2114 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"cairo_surface_t *_cairo_surface_get_source(cairo_surface_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return ((*(*surface).backend).source)
        .expect("non-null function pointer")(surface as *mut libc::c_void, extents);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_default_source(
    mut surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    if !extents.is_null() {
        _cairo_surface_get_extents(surface as *mut cairo_surface_t, extents);
    }
    return surface as *mut cairo_surface_t;
}
unsafe extern "C" fn _pattern_has_error(
    mut pattern: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut spattern: *const cairo_surface_pattern_t = 0
        as *const cairo_surface_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS;
    }
    spattern = pattern as *const cairo_surface_pattern_t;
    if (*(*spattern).surface).status as u64 != 0 {
        return (*(*spattern).surface).status;
    }
    if (*(*spattern).surface).finished() != 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn nothing_to_do(
    mut surface: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
) -> cairo_bool_t {
    if _cairo_pattern_is_clear(source) != 0 {
        if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int;
        }
        if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
            op = CAIRO_OPERATOR_CLEAR;
        }
    }
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        && (*surface).is_clear() as libc::c_int != 0
    {
        return 1 as libc::c_int;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_ATOP as libc::c_int as libc::c_uint
        && (*surface).content as libc::c_uint
            & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_paint(
    mut surface: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut is_clear: cairo_bool_t = 0;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _pattern_has_error(source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if nothing_to_do(surface, op, source) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_surface_begin_modification(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = ((*(*surface).backend).paint)
        .expect(
            "non-null function pointer",
        )(surface as *mut libc::c_void, op, source, clip);
    is_clear = (op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        && clip.is_null()) as libc::c_int;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint || is_clear != 0
    {
        (*surface).set_is_clear(is_clear as libc::c_uint);
        let ref mut fresh21 = (*surface).serial;
        *fresh21 = (*fresh21).wrapping_add(1);
    }
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_mask(
    mut surface: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_pattern_is_clear(mask) != 0 && _cairo_operator_bounded_by_mask(op) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _pattern_has_error(source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = _pattern_has_error(mask) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if nothing_to_do(surface, op, source) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_surface_begin_modification(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = ((*(*surface).backend).mask)
        .expect(
            "non-null function pointer",
        )(surface as *mut libc::c_void, op, source, mask, clip);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
        let ref mut fresh22 = (*surface).serial;
        *fresh22 = (*fresh22).wrapping_add(1);
    }
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_fill_stroke(
    mut surface: *mut cairo_surface_t,
    mut fill_op: cairo_operator_t,
    mut fill_source: *const cairo_pattern_t,
    mut fill_rule: cairo_fill_rule_t,
    mut fill_tolerance: libc::c_double,
    mut fill_antialias: cairo_antialias_t,
    mut path: *mut cairo_path_fixed_t,
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*surface).is_clear() as libc::c_int != 0
        && fill_op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        && stroke_op as libc::c_uint
            == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
    {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _pattern_has_error(fill_source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = _pattern_has_error(stroke_source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = _cairo_surface_begin_modification(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if ((*(*surface).backend).fill_stroke).is_some() {
        let mut dev_ctm: cairo_matrix_t = *stroke_ctm;
        let mut dev_ctm_inverse: cairo_matrix_t = *stroke_ctm_inverse;
        status = ((*(*surface).backend).fill_stroke)
            .expect(
                "non-null function pointer",
            )(
            surface as *mut libc::c_void,
            fill_op,
            fill_source,
            fill_rule,
            fill_tolerance,
            fill_antialias,
            path,
            stroke_op,
            stroke_source,
            stroke_style,
            &mut dev_ctm,
            &mut dev_ctm_inverse,
            stroke_tolerance,
            stroke_antialias,
            clip,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            current_block = 8222956241070022110;
        } else {
            current_block = 26972500619410423;
        }
    } else {
        current_block = 26972500619410423;
    }
    match current_block {
        26972500619410423 => {
            status = _cairo_surface_fill(
                surface,
                fill_op,
                fill_source,
                path,
                fill_rule,
                fill_tolerance,
                fill_antialias,
                clip,
            ) as cairo_int_status_t;
            if !(status as u64 != 0) {
                status = _cairo_surface_stroke(
                    surface,
                    stroke_op,
                    stroke_source,
                    path,
                    stroke_style,
                    stroke_ctm,
                    stroke_ctm_inverse,
                    stroke_tolerance,
                    stroke_antialias,
                    clip,
                ) as cairo_int_status_t;
                status as u64 != 0;
            }
        }
        _ => {}
    }
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
        let ref mut fresh23 = (*surface).serial;
        *fresh23 = (*fresh23).wrapping_add(1);
    }
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_stroke(
    mut surface: *mut cairo_surface_t,
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _pattern_has_error(source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if nothing_to_do(surface, op, source) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_surface_begin_modification(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = ((*(*surface).backend).stroke)
        .expect(
            "non-null function pointer",
        )(
        surface as *mut libc::c_void,
        op,
        source,
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        clip,
    );
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
        let ref mut fresh24 = (*surface).serial;
        *fresh24 = (*fresh24).wrapping_add(1);
    }
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_fill(
    mut surface: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _pattern_has_error(source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if nothing_to_do(surface, op, source) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_surface_begin_modification(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = ((*(*surface).backend).fill)
        .expect(
            "non-null function pointer",
        )(
        surface as *mut libc::c_void,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    );
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
        let ref mut fresh25 = (*surface).serial;
        *fresh25 = (*fresh25).wrapping_add(1);
    }
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_copy_page(mut surface: *mut cairo_surface_t) {
    if (*surface).status as u64 != 0 {
        return;
    }
    if ((*surface).snapshot_of).is_null() {} else {
        __assert_fail(
            b"surface->snapshot_of == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            2456 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void cairo_surface_copy_page(cairo_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_SURFACE_FINISHED as libc::c_int as cairo_int_status_t,
        );
        return;
    }
    if ((*(*surface).backend).copy_page).is_none() {
        return;
    }
    _cairo_surface_set_error(
        surface,
        ((*(*surface).backend).copy_page)
            .expect("non-null function pointer")(surface as *mut libc::c_void),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_show_page(mut surface: *mut cairo_surface_t) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return;
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_SURFACE_FINISHED as libc::c_int as cairo_int_status_t,
        );
        return;
    }
    status = _cairo_surface_begin_modification(surface);
    if status as u64 != 0 {
        _cairo_surface_set_error(surface, status as cairo_int_status_t);
        return;
    }
    if ((*(*surface).backend).show_page).is_none() {
        return;
    }
    _cairo_surface_set_error(
        surface,
        ((*(*surface).backend).show_page)
            .expect("non-null function pointer")(surface as *mut libc::c_void),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_get_extents(
    mut surface: *mut cairo_surface_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut bounded: cairo_bool_t = 0;
    if !((*surface).status as u64 != 0) {
        if (*surface).finished() != 0 {
            _cairo_surface_set_error(
                surface,
                CAIRO_STATUS_SURFACE_FINISHED as libc::c_int as cairo_int_status_t,
            );
        } else {
            bounded = 0 as libc::c_int;
            if ((*(*surface).backend).get_extents).is_some() {
                bounded = ((*(*surface).backend).get_extents)
                    .expect(
                        "non-null function pointer",
                    )(surface as *mut libc::c_void, extents);
            }
            if bounded == 0 {
                _cairo_unbounded_rectangle_init(extents);
            }
            return bounded;
        }
    }
    let ref mut fresh26 = (*extents).y;
    *fresh26 = 0 as libc::c_int;
    (*extents).x = *fresh26;
    let ref mut fresh27 = (*extents).height;
    *fresh27 = 0 as libc::c_int;
    (*extents).width = *fresh27;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_has_show_text_glyphs(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    if (*surface).status as u64 != 0 {
        return 0 as libc::c_int;
    }
    if (*surface).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_SURFACE_FINISHED as libc::c_int as cairo_int_status_t,
        );
        return 0 as libc::c_int;
    }
    if ((*(*surface).backend).has_show_text_glyphs).is_some() {
        return ((*(*surface).backend).has_show_text_glyphs)
            .expect("non-null function pointer")(surface as *mut libc::c_void)
    } else {
        return ((*(*surface).backend).show_text_glyphs).is_some() as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ensure_scaled_glyph(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut foreground_color: *mut cairo_color_t,
    mut glyph_cache: *mut *mut cairo_scaled_glyph_t,
    mut glyph: *mut cairo_glyph_t,
    mut scaled_glyph: *mut *mut cairo_scaled_glyph_t,
) -> cairo_int_status_t {
    let mut cache_index: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    cache_index = ((*glyph).index).wrapping_rem(64 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    *scaled_glyph = *glyph_cache.offset(cache_index as isize);
    if (*scaled_glyph).is_null()
        || (**scaled_glyph).hash_entry.hash & 0xffffff as libc::c_int as libc::c_ulong
            != (*glyph).index
    {
        status = _cairo_scaled_glyph_lookup(
            scaled_font,
            (*glyph).index,
            CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE,
            foreground_color,
            scaled_glyph,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                (*glyph).index,
                CAIRO_SCALED_GLYPH_INFO_SURFACE,
                0 as *const cairo_color_t,
                scaled_glyph,
            );
        }
        if status as u64 != 0 {
            status = _cairo_scaled_font_set_error(scaled_font, status as cairo_status_t)
                as cairo_int_status_t;
        }
        let ref mut fresh28 = *glyph_cache.offset(cache_index as isize);
        *fresh28 = *scaled_glyph;
    }
    return status;
}
#[inline]
unsafe extern "C" fn composite_one_color_glyph(
    mut surface: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
    mut glyph: *mut cairo_glyph_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut x_scale: libc::c_double,
    mut y_scale: libc::c_double,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut glyph_surface: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut has_color: libc::c_int = 0;
    status = CAIRO_INT_STATUS_SUCCESS;
    has_color = ((*scaled_glyph).has_info
        & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint)
        as libc::c_int;
    if has_color != 0 {
        glyph_surface = (*scaled_glyph).color_surface;
    } else {
        glyph_surface = (*scaled_glyph).surface;
    }
    if (*glyph_surface).width != 0 && (*glyph_surface).height != 0 {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        x = _cairo_lround(
            (*glyph).x * x_scale - (*glyph_surface).base.device_transform.x0,
        );
        y = _cairo_lround(
            (*glyph).y * y_scale - (*glyph_surface).base.device_transform.y0,
        );
        pattern = cairo_pattern_create_for_surface(
            glyph_surface as *mut cairo_surface_t,
        );
        cairo_matrix_init_translate(
            &mut matrix,
            -x as libc::c_double,
            -y as libc::c_double,
        );
        cairo_matrix_scale(&mut matrix, x_scale, y_scale);
        cairo_pattern_set_matrix(pattern, &mut matrix);
        if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            || has_color == 0
        {
            status = _cairo_surface_mask(surface, op, pattern, pattern, clip)
                as cairo_int_status_t;
        } else {
            status = _cairo_surface_paint(surface, op, pattern, clip)
                as cairo_int_status_t;
        }
        cairo_pattern_destroy(pattern);
    }
    return status;
}
unsafe extern "C" fn composite_color_glyphs(
    mut surface: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut utf8: *mut libc::c_char,
    mut utf8_len: *mut libc::c_int,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: *mut libc::c_int,
    mut clusters: *mut cairo_text_cluster_t,
    mut num_clusters: *mut libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut remaining_clusters: libc::c_int = 0 as libc::c_int;
    let mut remaining_glyphs: libc::c_int = 0 as libc::c_int;
    let mut remaining_bytes: libc::c_int = 0 as libc::c_int;
    let mut glyph_pos: libc::c_int = 0 as libc::c_int;
    let mut byte_pos: libc::c_int = 0 as libc::c_int;
    let mut gp: libc::c_int = 0;
    let mut glyph_cache: [*mut cairo_scaled_glyph_t; 64] = [0
        as *mut cairo_scaled_glyph_t; 64];
    let mut foreground_color: *mut cairo_color_t = 0 as *mut cairo_color_t;
    let mut x_scale: libc::c_double = 1.0f64;
    let mut y_scale: libc::c_double = 1.0f64;
    if (*surface).is_vector() != 0 {
        let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
        let mut font_matrix: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        let mut ctm: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        let mut font_options: cairo_font_options_t = cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *const libc::c_char as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        };
        x_scale = (*surface).x_fallback_resolution / (*surface).x_resolution;
        y_scale = (*surface).y_fallback_resolution / (*surface).y_resolution;
        font_face = cairo_scaled_font_get_font_face(scaled_font);
        cairo_scaled_font_get_font_matrix(scaled_font, &mut font_matrix);
        cairo_scaled_font_get_ctm(scaled_font, &mut ctm);
        cairo_scaled_font_get_font_options(scaled_font, &mut font_options);
        cairo_matrix_scale(&mut ctm, x_scale, y_scale);
        scaled_font = cairo_scaled_font_create(
            font_face,
            &mut font_matrix,
            &mut ctm,
            &mut font_options,
        );
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        foreground_color = &mut (*(source as *mut cairo_solid_pattern_t)).color;
    }
    memset(
        glyph_cache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut cairo_scaled_glyph_t; 64]>() as libc::c_ulong,
    );
    status = CAIRO_INT_STATUS_SUCCESS;
    _cairo_scaled_font_freeze_cache(scaled_font);
    if !clusters.is_null() {
        if cluster_flags as libc::c_uint
            & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint != 0
        {
            glyph_pos = *num_glyphs - 1 as libc::c_int;
        }
        i = 0 as libc::c_int;
        's_131: loop {
            if !(i < *num_clusters) {
                current_block = 17075014677070940716;
                break;
            }
            let mut skip_cluster: cairo_bool_t = 1 as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*clusters.offset(i as isize)).num_glyphs {
                if cluster_flags as libc::c_uint
                    & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
                    != 0
                {
                    gp = glyph_pos - j;
                } else {
                    gp = glyph_pos + j;
                }
                status = ensure_scaled_glyph(
                    scaled_font,
                    foreground_color,
                    glyph_cache.as_mut_ptr(),
                    &mut *glyphs.offset(gp as isize),
                    &mut scaled_glyph,
                );
                if status as u64 != 0 {
                    current_block = 8942991032556948500;
                    break 's_131;
                }
                if (*scaled_glyph).has_info
                    & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int
                        as libc::c_uint != 0 as libc::c_int as libc::c_uint
                {
                    skip_cluster = 0 as libc::c_int;
                    break;
                } else {
                    j += 1;
                }
            }
            if skip_cluster != 0 {
                memmove(
                    utf8.offset(remaining_bytes as isize) as *mut libc::c_void,
                    utf8.offset(byte_pos as isize) as *const libc::c_void,
                    (*clusters.offset(i as isize)).num_bytes as libc::c_ulong,
                );
                remaining_bytes += (*clusters.offset(i as isize)).num_bytes;
                byte_pos += (*clusters.offset(i as isize)).num_bytes;
                j = 0 as libc::c_int;
                while j < (*clusters.offset(i as isize)).num_glyphs {
                    if cluster_flags as libc::c_uint
                        & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
                        != 0
                    {
                        let fresh29 = glyph_pos;
                        glyph_pos = glyph_pos - 1;
                        *glyphs
                            .offset(
                                (*num_glyphs - 1 as libc::c_int - remaining_glyphs) as isize,
                            ) = *glyphs.offset(fresh29 as isize);
                    } else {
                        let fresh30 = glyph_pos;
                        glyph_pos = glyph_pos + 1;
                        *glyphs
                            .offset(
                                remaining_glyphs as isize,
                            ) = *glyphs.offset(fresh30 as isize);
                    }
                    j += 1;
                    remaining_glyphs += 1;
                }
                let fresh31 = remaining_clusters;
                remaining_clusters = remaining_clusters + 1;
                *clusters.offset(fresh31 as isize) = *clusters.offset(i as isize);
            } else {
                j = 0 as libc::c_int;
                while j < (*clusters.offset(i as isize)).num_glyphs {
                    if cluster_flags as libc::c_uint
                        & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
                        != 0
                    {
                        gp = glyph_pos - j;
                    } else {
                        gp = glyph_pos + j;
                    }
                    status = ensure_scaled_glyph(
                        scaled_font,
                        foreground_color,
                        glyph_cache.as_mut_ptr(),
                        &mut *glyphs.offset(gp as isize),
                        &mut scaled_glyph,
                    );
                    if status as u64 != 0 {
                        current_block = 8942991032556948500;
                        break 's_131;
                    }
                    status = composite_one_color_glyph(
                        surface,
                        op,
                        source,
                        clip,
                        &mut *glyphs.offset(gp as isize),
                        scaled_glyph,
                        x_scale,
                        y_scale,
                    );
                    if status as libc::c_uint != 0
                        && status as libc::c_uint
                            != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int
                                as libc::c_uint
                    {
                        current_block = 8942991032556948500;
                        break 's_131;
                    }
                    j += 1;
                }
                if cluster_flags as libc::c_uint
                    & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
                    != 0
                {
                    glyph_pos -= (*clusters.offset(i as isize)).num_glyphs;
                } else {
                    glyph_pos += (*clusters.offset(i as isize)).num_glyphs;
                }
                byte_pos += (*clusters.offset(i as isize)).num_bytes;
            }
            i += 1;
        }
        match current_block {
            8942991032556948500 => {}
            _ => {
                if cluster_flags as libc::c_uint
                    & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
                    != 0
                {
                    memmove(
                        utf8 as *mut libc::c_void,
                        utf8
                            .offset(*utf8_len as isize)
                            .offset(-(remaining_bytes as isize)) as *const libc::c_void,
                        remaining_bytes as libc::c_ulong,
                    );
                    memmove(
                        glyphs as *mut libc::c_void,
                        glyphs.offset((*num_glyphs - remaining_glyphs) as isize)
                            as *const libc::c_void,
                        (::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                            .wrapping_mul(remaining_glyphs as libc::c_ulong),
                    );
                }
                *utf8_len = remaining_bytes;
                *num_glyphs = remaining_glyphs;
                *num_clusters = remaining_clusters;
            }
        }
    } else {
        glyph_pos = 0 as libc::c_int;
        loop {
            if !(glyph_pos < *num_glyphs) {
                current_block = 1352918242886884122;
                break;
            }
            status = ensure_scaled_glyph(
                scaled_font,
                foreground_color,
                glyph_cache.as_mut_ptr(),
                &mut *glyphs.offset(glyph_pos as isize),
                &mut scaled_glyph,
            );
            if status as u64 != 0 {
                current_block = 8942991032556948500;
                break;
            }
            if (*scaled_glyph).has_info
                & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                let fresh32 = remaining_glyphs;
                remaining_glyphs = remaining_glyphs + 1;
                *glyphs.offset(fresh32 as isize) = *glyphs.offset(glyph_pos as isize);
            } else {
                status = composite_one_color_glyph(
                    surface,
                    op,
                    source,
                    clip,
                    &mut *glyphs.offset(glyph_pos as isize),
                    scaled_glyph,
                    x_scale,
                    y_scale,
                );
                if status as libc::c_uint != 0
                    && status as libc::c_uint
                        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
                {
                    current_block = 8942991032556948500;
                    break;
                }
            }
            glyph_pos += 1;
        }
        match current_block {
            8942991032556948500 => {}
            _ => {
                *num_glyphs = remaining_glyphs;
            }
        }
    }
    _cairo_scaled_font_thaw_cache(scaled_font);
    if (*surface).is_vector() != 0 {
        cairo_scaled_font_destroy(scaled_font);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_show_text_glyphs(
    mut surface: *mut cairo_surface_t,
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
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut utf8_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if num_glyphs == 0 as libc::c_int && utf8_len == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _pattern_has_error(source) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = cairo_scaled_font_status(scaled_font) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if nothing_to_do(surface, op, source) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_surface_begin_modification(surface) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if _cairo_scaled_font_has_color_glyphs(scaled_font) != 0
        && (*scaled_font).options.color_mode as libc::c_uint
            != CAIRO_COLOR_MODE_NO_COLOR as libc::c_int as libc::c_uint
    {
        utf8_copy = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(utf8_len as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            utf8_copy as *mut libc::c_void,
            utf8 as *const libc::c_void,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(utf8_len as libc::c_ulong),
        );
        utf8 = utf8_copy;
        status = composite_color_glyphs(
            surface,
            op,
            source,
            utf8 as *mut libc::c_char,
            &mut utf8_len,
            glyphs,
            &mut num_glyphs,
            clusters as *mut cairo_text_cluster_t,
            &mut num_clusters,
            cluster_flags,
            scaled_font,
            clip,
        );
        if status as libc::c_uint != 0
            && status as libc::c_uint
                != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
        {
            current_block = 2673379966659510649;
        } else if num_glyphs == 0 as libc::c_int {
            current_block = 2673379966659510649;
        } else {
            current_block = 15768484401365413375;
        }
    } else {
        utf8_copy = 0 as *mut libc::c_char;
        current_block = 15768484401365413375;
    }
    match current_block {
        15768484401365413375 => {
            if !clusters.is_null() {
                status = CAIRO_INT_STATUS_UNSUPPORTED;
                if ((*(*surface).backend).show_text_glyphs).is_some() {
                    status = ((*(*surface).backend).show_text_glyphs)
                        .expect(
                            "non-null function pointer",
                        )(
                        surface as *mut libc::c_void,
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
                }
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                    && ((*(*surface).backend).show_glyphs).is_some()
                {
                    status = ((*(*surface).backend).show_glyphs)
                        .expect(
                            "non-null function pointer",
                        )(
                        surface as *mut libc::c_void,
                        op,
                        source,
                        glyphs,
                        num_glyphs,
                        scaled_font,
                        clip,
                    );
                }
            } else if ((*(*surface).backend).show_glyphs).is_some() {
                status = ((*(*surface).backend).show_glyphs)
                    .expect(
                        "non-null function pointer",
                    )(
                    surface as *mut libc::c_void,
                    op,
                    source,
                    glyphs,
                    num_glyphs,
                    scaled_font,
                    clip,
                );
            } else if ((*(*surface).backend).show_text_glyphs).is_some() {
                status = ((*(*surface).backend).show_text_glyphs)
                    .expect(
                        "non-null function pointer",
                    )(
                    surface as *mut libc::c_void,
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
            }
        }
        _ => {}
    }
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
        let ref mut fresh33 = (*surface).serial;
        *fresh33 = (*fresh33).wrapping_add(1);
    }
    if !utf8_copy.is_null() {
        free(utf8_copy as *mut libc::c_void);
    }
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_tag(
    mut surface: *mut cairo_surface_t,
    mut begin: cairo_bool_t,
    mut tag_name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
    }
    if ((*(*surface).backend).tag).is_none() {
        return CAIRO_STATUS_SUCCESS;
    }
    status = ((*(*surface).backend).tag)
        .expect(
            "non-null function pointer",
        )(surface as *mut libc::c_void, begin, tag_name, attributes);
    (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
    return _cairo_surface_set_error(surface, status) as cairo_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_set_resolution(
    mut surface: *mut cairo_surface_t,
    mut x_res: libc::c_double,
    mut y_res: libc::c_double,
) {
    if (*surface).status as u64 != 0 {
        return;
    }
    (*surface).x_resolution = x_res;
    (*surface).y_resolution = y_res;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_surface_t {
    if (status as libc::c_uint) < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status < CAIRO_STATUS_LAST_STATUS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
            3050 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"cairo_surface_t *_cairo_surface_create_in_error(cairo_status_t)\0"))
                .as_ptr(),
        );
    }
    match status as libc::c_uint {
        1 => return &_cairo_surface_nil as *const cairo_surface_t as *mut cairo_surface_t,
        13 => {
            return &_cairo_surface_nil_surface_type_mismatch as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        6 => {
            return &_cairo_surface_nil_invalid_status as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        15 => {
            return &_cairo_surface_nil_invalid_content as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        16 => {
            return &_cairo_surface_nil_invalid_format as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        17 => {
            return &_cairo_surface_nil_invalid_visual as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        10 => {
            return &_cairo_surface_nil_read_error as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        11 => {
            return &_cairo_surface_nil_write_error as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        18 => {
            return &_cairo_surface_nil_file_not_found as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        23 => {
            return &_cairo_surface_nil_temp_file_error as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        24 => {
            return &_cairo_surface_nil_invalid_stride as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        32 => {
            return &_cairo_surface_nil_invalid_size as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        34 => {
            return &_cairo_surface_nil_device_type_mismatch as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        35 => {
            return &_cairo_surface_nil_device_error as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        0 | 44 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-surface.c\0" as *const u8 as *const libc::c_char,
                    3082 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 64],
                        &[libc::c_char; 64],
                    >(
                        b"cairo_surface_t *_cairo_surface_create_in_error(cairo_status_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        2 | 3 | 4 | 5 | 7 | 8 | 9 | 12 | 14 | 19 | 20 | 21 | 22 | 25 | 26 | 27 | 28 | 29
        | 30 | 31 | 33 | 36 | 37 | 38 | 39 | 40 | 41 | 43 | 42 | _ => {}
    }
    let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    return &_cairo_surface_nil as *const cairo_surface_t as *mut cairo_surface_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_int_surface_create_in_error(
    mut status: cairo_int_status_t,
) -> *mut cairo_surface_t {
    if (status as libc::c_uint)
        < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    match status as libc::c_int {
        100 => {
            return &_cairo_surface_nil_unsupported as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        102 => {
            return &_cairo_surface_nil_nothing_to_do as *const cairo_surface_t
                as *mut cairo_surface_t;
        }
        _ => {
            let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_surface_nil as *const cairo_surface_t as *mut cairo_surface_t;
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    _cairo_surface_nil = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_NO_MEMORY,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_surface_type_mismatch = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_invalid_status = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_INVALID_STATUS,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_invalid_content = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_INVALID_CONTENT,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_invalid_format = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_INVALID_FORMAT,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_invalid_visual = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_INVALID_VISUAL,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_file_not_found = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_FILE_NOT_FOUND,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_temp_file_error = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_TEMP_FILE_ERROR,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_read_error = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_READ_ERROR,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_write_error = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_WRITE_ERROR,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_invalid_stride = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_INVALID_STRIDE,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_invalid_size = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_INVALID_SIZE,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_device_type_mismatch = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_DEVICE_TYPE_MISMATCH,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_device_error = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_STATUS_DEVICE_ERROR,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_unsupported = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
    _cairo_surface_nil_nothing_to_do = {
        let mut init = _cairo_surface {
            _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [0; 1],
            c2rust_padding: [0; 7],
            backend: 0 as *const cairo_surface_backend_t,
            device: 0 as *mut cairo_device_t,
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            content: CAIRO_CONTENT_COLOR,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            status: CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t,
            unique_id: 0 as libc::c_int as libc::c_uint,
            serial: 0 as libc::c_int as libc::c_uint,
            damage: 0 as *mut cairo_damage_t,
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            mime_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            device_transform: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0.0f64,
                    y0: 0.0f64,
                };
                init
            },
            device_transform_observers: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            x_resolution: 0.0f64,
            y_resolution: 0.0f64,
            x_fallback_resolution: 0.0f64,
            y_fallback_resolution: 0.0f64,
            snapshot_of: 0 as *mut cairo_surface_t,
            snapshot_detach: None,
            snapshots: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            snapshot: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            font_options: {
                let mut init = _cairo_font_options {
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
                init
            },
        };
        init.set__finishing(0 as libc::c_int as libc::c_uint);
        init.set_finished(0 as libc::c_int as libc::c_uint);
        init.set_is_clear(1 as libc::c_int as libc::c_uint);
        init.set_has_font_options(0 as libc::c_int as libc::c_uint);
        init.set_owns_device(0 as libc::c_int as libc::c_uint);
        init.set_is_vector(0 as libc::c_int as libc::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
