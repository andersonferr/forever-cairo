use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;
    fn cairo_region_destroy(region: *mut cairo_region_t);
    fn cairo_region_num_rectangles(region: *const cairo_region_t) -> libc::c_int;
    fn cairo_region_get_rectangle(
        region: *const cairo_region_t,
        nth: libc::c_int,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn _cairo_debug_print_path(stream: *mut FILE, path: *const cairo_path_fixed_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_init_for_array(
        boxes: *mut cairo_boxes_t,
        array: *mut cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_path_fixed_translate(
        path: *mut cairo_path_fixed_t,
        offx: cairo_fixed_t,
        offy: cairo_fixed_t,
    );
    fn _cairo_path_fixed_equal(
        a: *const cairo_path_fixed_t,
        b: *const cairo_path_fixed_t,
    ) -> cairo_bool_t;
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_init_copy(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_clip_intersect_rectangle(
        clip: *mut cairo_clip_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_path_fixed_approximate_clip_extents(
        path: *const cairo_path_fixed_t,
        extents: *mut cairo_rectangle_int_t,
    );
    fn _cairo_clip_intersect_rectilinear_path(
        clip: *mut cairo_clip_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        antialias: cairo_antialias_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_clip_intersect_box(
        clip: *mut cairo_clip_t,
        box_0: *const cairo_box_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_path_fixed_is_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_transform(
        path: *mut cairo_path_fixed_t,
        matrix: *const cairo_matrix_t,
    );
    fn _cairo_path_fixed_close_path(path: *mut cairo_path_fixed_t) -> cairo_status_t;
    fn _cairo_path_fixed_line_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_move_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_init(path: *mut cairo_path_fixed_t);
    fn _cairo_clip_intersect_boxes(
        clip: *mut cairo_clip_t,
        boxes: *const cairo_boxes_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    static _cairo_empty_rectangle: cairo_rectangle_int_t;
    fn _cairo_clip_get_region(clip: *const cairo_clip_t) -> *mut cairo_region_t;
    fn _cairo_clip_is_region(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _cairo_clip_contains_rectangle(
        clip: *const cairo_clip_t,
        rect: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _freed_pool_get_search(pool: *mut freed_pool_t) -> *mut libc::c_void;
    fn _freed_pool_put_search(pool: *mut freed_pool_t, ptr: *mut libc::c_void);
    fn _freed_pool_reset(pool: *mut freed_pool_t);
    fn _cairo_gstate_backend_to_user_rectangle(
        gstate: *mut cairo_gstate_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
        is_tight: *mut cairo_bool_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_list {
    pub status: cairo_status_t,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: libc::c_int,
}
pub type cairo_rectangle_list_t = _cairo_rectangle_list;
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
pub type cairo_fixed_unsigned_t = uint32_t;
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
pub type cairo_composite_rectangles_t = _cairo_composite_rectangles;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gstate {
    pub op: cairo_operator_t,
    pub opacity: libc::c_double,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
    pub stroke_style: cairo_stroke_style_t,
    pub fill_rule: cairo_fill_rule_t,
    pub font_face: *mut cairo_font_face_t,
    pub scaled_font: *mut cairo_scaled_font_t,
    pub previous_scaled_font: *mut cairo_scaled_font_t,
    pub font_matrix: cairo_matrix_t,
    pub font_options: cairo_font_options_t,
    pub clip: *mut cairo_clip_t,
    pub target: *mut cairo_surface_t,
    pub parent_target: *mut cairo_surface_t,
    pub original_target: *mut cairo_surface_t,
    pub device_transform_observer: cairo_observer_t,
    pub ctm: cairo_matrix_t,
    pub ctm_inverse: cairo_matrix_t,
    pub source_ctm_inverse: cairo_matrix_t,
    pub is_identity: cairo_bool_t,
    pub source: *mut cairo_pattern_t,
    pub next: *mut _cairo_gstate,
}
pub type cairo_observer_t = _cairo_observer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_observer {
    pub link: cairo_list_t,
    pub callback: Option::<
        unsafe extern "C" fn(*mut cairo_observer_t, *mut libc::c_void) -> (),
    >,
}
pub type cairo_gstate_t = _cairo_gstate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freed_pool_t {
    pub pool: [*mut libc::c_void; 16],
    pub top: cairo_atomic_int_t,
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get_relaxed(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load_relaxed(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_set_relaxed(
    mut x: *mut cairo_atomic_int_t,
    mut val: cairo_atomic_int_t,
) {
    ::std::intrinsics::atomic_store_relaxed(x, val);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_get(
    mut x: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_cmpxchg_impl(
    mut x: *mut *mut libc::c_void,
    mut oldv: *mut libc::c_void,
    mut newv: *mut libc::c_void,
) -> cairo_bool_t {
    let mut expected: *mut libc::c_void = oldv;
    let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh0.0;
    return fresh0.1 as cairo_bool_t;
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_fill_is_empty(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).fill_is_empty() as cairo_bool_t;
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
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh1, fresh2) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh1;
    if fresh2 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_ceil(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return _cairo_fixed_floor(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t,
    );
}
#[inline]
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_round_down(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return _cairo_fixed_floor(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn _cairo_matrix_is_translation(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_clip_is_all_clipped(
    mut clip: *const cairo_clip_t,
) -> cairo_bool_t {
    return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_clip_set_all_clipped(
    mut clip: *mut cairo_clip_t,
) -> *mut cairo_clip_t {
    _cairo_clip_destroy(clip);
    return &__cairo_clip_all as *const cairo_clip_t as *mut cairo_clip_t;
}
#[inline(always)]
unsafe extern "C" fn _atomic_fetch(
    mut slot: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    loop {
        ptr = _cairo_atomic_ptr_get(slot);
        if !(_cairo_atomic_ptr_cmpxchg_impl(slot, ptr, 0 as *mut libc::c_void) == 0) {
            break;
        }
    }
    return ptr;
}
#[inline(always)]
unsafe extern "C" fn _atomic_store(
    mut slot: *mut *mut libc::c_void,
    mut ptr: *mut libc::c_void,
) -> cairo_bool_t {
    return _cairo_atomic_ptr_cmpxchg_impl(slot, 0 as *mut libc::c_void, ptr);
}
#[inline]
unsafe extern "C" fn _freed_pool_get(mut pool: *mut freed_pool_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    i = _cairo_atomic_int_get_relaxed(&mut (*pool).top) - 1 as libc::c_int;
    if i < 0 as libc::c_int {
        i = 0 as libc::c_int;
    }
    ptr = _atomic_fetch(&mut *((*pool).pool).as_mut_ptr().offset(i as isize));
    if !ptr.is_null() {
        _cairo_atomic_int_set_relaxed(&mut (*pool).top, i);
        return ptr;
    }
    return _freed_pool_get_search(pool);
}
#[inline]
unsafe extern "C" fn _freed_pool_put(
    mut pool: *mut freed_pool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = _cairo_atomic_int_get_relaxed(&mut (*pool).top);
    if i
        < (::std::mem::size_of::<[*mut libc::c_void; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            as libc::c_int
        && _atomic_store(&mut *((*pool).pool).as_mut_ptr().offset(i as isize), ptr) != 0
    {
        _cairo_atomic_int_set_relaxed(&mut (*pool).top, i + 1 as libc::c_int);
        return;
    }
    _freed_pool_put_search(pool, ptr);
}
static mut clip_path_pool: freed_pool_t = freed_pool_t {
    pool: [0 as *const libc::c_void as *mut libc::c_void; 16],
    top: 0,
};
static mut clip_pool: freed_pool_t = freed_pool_t {
    pool: [0 as *const libc::c_void as *mut libc::c_void; 16],
    top: 0,
};
#[no_mangle]
pub static mut __cairo_clip_all: cairo_clip_t = cairo_clip_t {
    extents: cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    },
    path: 0 as *const cairo_clip_path_t as *mut cairo_clip_path_t,
    boxes: 0 as *const cairo_box_t as *mut cairo_box_t,
    num_boxes: 0,
    region: 0 as *const cairo_region_t as *mut cairo_region_t,
    is_region: 0,
    embedded_box: cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    },
};
unsafe extern "C" fn _cairo_clip_path_create(
    mut clip: *mut cairo_clip_t,
) -> *mut cairo_clip_path_t {
    let mut clip_path: *mut cairo_clip_path_t = 0 as *mut cairo_clip_path_t;
    clip_path = _freed_pool_get(&mut clip_path_pool) as *mut cairo_clip_path_t;
    if clip_path.is_null() {
        clip_path = (if ::std::mem::size_of::<cairo_clip_path_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_clip_path_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_clip_path_t;
        if clip_path.is_null() {
            return 0 as *mut cairo_clip_path_t;
        }
    }
    (*clip_path).ref_count.ref_count = 1 as libc::c_int;
    let ref mut fresh3 = (*clip_path).prev;
    *fresh3 = (*clip).path;
    let ref mut fresh4 = (*clip).path;
    *fresh4 = clip_path;
    return clip_path;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_path_reference(
    mut clip_path: *mut cairo_clip_path_t,
) -> *mut cairo_clip_path_t {
    if _cairo_atomic_int_get(&mut (*clip_path).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&clip_path->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-clip.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"cairo_clip_path_t *_cairo_clip_path_reference(cairo_clip_path_t *)\0"))
                .as_ptr(),
        );
    }
    ::std::intrinsics::atomic_xadd(
        &mut (*clip_path).ref_count.ref_count,
        1 as libc::c_int,
    );
    return clip_path;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_path_destroy(
    mut clip_path: *mut cairo_clip_path_t,
) {
    if _cairo_atomic_int_get(&mut (*clip_path).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&clip_path->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-clip.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void _cairo_clip_path_destroy(cairo_clip_path_t *)\0"))
                .as_ptr(),
        );
    }
    if !(::std::intrinsics::atomic_xsub(
        &mut (*clip_path).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        return;
    }
    _cairo_path_fixed_fini(&mut (*clip_path).path);
    if !((*clip_path).prev).is_null() {
        _cairo_clip_path_destroy((*clip_path).prev);
    }
    _freed_pool_put(&mut clip_path_pool, clip_path as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_create() -> *mut cairo_clip_t {
    let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    clip = _freed_pool_get(&mut clip_pool) as *mut cairo_clip_t;
    if clip.is_null() {
        clip = (if ::std::mem::size_of::<cairo_clip_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_clip_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_clip_t;
        if clip.is_null() {
            return 0 as *mut cairo_clip_t;
        }
    }
    (*clip).extents = _cairo_unbounded_rectangle;
    let ref mut fresh5 = (*clip).path;
    *fresh5 = 0 as *mut cairo_clip_path_t;
    let ref mut fresh6 = (*clip).boxes;
    *fresh6 = 0 as *mut cairo_box_t;
    (*clip).num_boxes = 0 as libc::c_int;
    let ref mut fresh7 = (*clip).region;
    *fresh7 = 0 as *mut cairo_region_t;
    (*clip).is_region = 0 as libc::c_int;
    return clip;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_destroy(mut clip: *mut cairo_clip_t) {
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return;
    }
    if !((*clip).path).is_null() {
        _cairo_clip_path_destroy((*clip).path);
    }
    if (*clip).boxes != &mut (*clip).embedded_box as *mut cairo_box_t {
        free((*clip).boxes as *mut libc::c_void);
    }
    cairo_region_destroy((*clip).region);
    _freed_pool_put(&mut clip_pool, clip as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_copy(
    mut clip: *const cairo_clip_t,
) -> *mut cairo_clip_t {
    let mut copy: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return clip as *mut cairo_clip_t;
    }
    copy = _cairo_clip_create();
    if !((*clip).path).is_null() {
        let ref mut fresh8 = (*copy).path;
        *fresh8 = _cairo_clip_path_reference((*clip).path);
    }
    if (*clip).num_boxes != 0 {
        if (*clip).num_boxes == 1 as libc::c_int {
            let ref mut fresh9 = (*copy).boxes;
            *fresh9 = &mut (*copy).embedded_box;
        } else {
            let ref mut fresh10 = (*copy).boxes;
            *fresh10 = _cairo_malloc_ab(
                (*clip).num_boxes as size_t,
                ::std::mem::size_of::<cairo_box_t>() as libc::c_ulong,
            ) as *mut cairo_box_t;
            if ((*copy).boxes).is_null() {
                return _cairo_clip_set_all_clipped(copy);
            }
        }
        memcpy(
            (*copy).boxes as *mut libc::c_void,
            (*clip).boxes as *const libc::c_void,
            ((*clip).num_boxes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_box_t>() as libc::c_ulong),
        );
        (*copy).num_boxes = (*clip).num_boxes;
    }
    (*copy).extents = (*clip).extents;
    let ref mut fresh11 = (*copy).region;
    *fresh11 = cairo_region_reference((*clip).region);
    (*copy).is_region = (*clip).is_region;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_copy_path(
    mut clip: *const cairo_clip_t,
) -> *mut cairo_clip_t {
    let mut copy: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return clip as *mut cairo_clip_t;
    }
    if (*clip).num_boxes != 0 {} else {
        __assert_fail(
            b"clip->num_boxes\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-clip.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"cairo_clip_t *_cairo_clip_copy_path(const cairo_clip_t *)\0"))
                .as_ptr(),
        );
    }
    copy = _cairo_clip_create();
    (*copy).extents = (*clip).extents;
    if !((*clip).path).is_null() {
        let ref mut fresh12 = (*copy).path;
        *fresh12 = _cairo_clip_path_reference((*clip).path);
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_copy_region(
    mut clip: *const cairo_clip_t,
) -> *mut cairo_clip_t {
    let mut copy: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut i: libc::c_int = 0;
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return clip as *mut cairo_clip_t;
    }
    if (*clip).num_boxes != 0 {} else {
        __assert_fail(
            b"clip->num_boxes\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-clip.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"cairo_clip_t *_cairo_clip_copy_region(const cairo_clip_t *)\0"))
                .as_ptr(),
        );
    }
    copy = _cairo_clip_create();
    (*copy).extents = (*clip).extents;
    if (*clip).num_boxes == 1 as libc::c_int {
        let ref mut fresh13 = (*copy).boxes;
        *fresh13 = &mut (*copy).embedded_box;
    } else {
        let ref mut fresh14 = (*copy).boxes;
        *fresh14 = _cairo_malloc_ab(
            (*clip).num_boxes as size_t,
            ::std::mem::size_of::<cairo_box_t>() as libc::c_ulong,
        ) as *mut cairo_box_t;
        if ((*copy).boxes).is_null() {
            return _cairo_clip_set_all_clipped(copy);
        }
    }
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        (*((*copy).boxes).offset(i as isize))
            .p1
            .x = _cairo_fixed_floor((*((*clip).boxes).offset(i as isize)).p1.x);
        (*((*copy).boxes).offset(i as isize))
            .p1
            .y = _cairo_fixed_floor((*((*clip).boxes).offset(i as isize)).p1.y);
        (*((*copy).boxes).offset(i as isize))
            .p2
            .x = _cairo_fixed_ceil((*((*clip).boxes).offset(i as isize)).p2.x);
        (*((*copy).boxes).offset(i as isize))
            .p2
            .y = _cairo_fixed_ceil((*((*clip).boxes).offset(i as isize)).p2.y);
        i += 1;
    }
    (*copy).num_boxes = (*clip).num_boxes;
    let ref mut fresh15 = (*copy).region;
    *fresh15 = cairo_region_reference((*clip).region);
    (*copy).is_region = 1 as libc::c_int;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_intersect_path(
    mut clip: *mut cairo_clip_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> *mut cairo_clip_t {
    let mut clip_path: *mut cairo_clip_path_t = 0 as *mut cairo_clip_path_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return clip;
    }
    if _cairo_path_fixed_fill_is_empty(path) != 0 {
        return _cairo_clip_set_all_clipped(clip);
    }
    if _cairo_path_fixed_is_box(path, &mut box_0) != 0 {
        if antialias as libc::c_uint
            == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        {
            box_0.p1.x = _cairo_fixed_round_down(box_0.p1.x);
            box_0.p1.y = _cairo_fixed_round_down(box_0.p1.y);
            box_0.p2.x = _cairo_fixed_round_down(box_0.p2.x);
            box_0.p2.y = _cairo_fixed_round_down(box_0.p2.y);
        }
        return _cairo_clip_intersect_box(clip, &mut box_0);
    }
    if _cairo_path_fixed_fill_is_rectilinear(path) != 0 {
        return _cairo_clip_intersect_rectilinear_path(clip, path, fill_rule, antialias);
    }
    _cairo_path_fixed_approximate_clip_extents(path, &mut extents);
    if extents.width == 0 as libc::c_int || extents.height == 0 as libc::c_int {
        return _cairo_clip_set_all_clipped(clip);
    }
    clip = _cairo_clip_intersect_rectangle(clip, &mut extents);
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return clip;
    }
    clip_path = _cairo_clip_path_create(clip);
    if clip_path.is_null() {
        return _cairo_clip_set_all_clipped(clip);
    }
    status = _cairo_path_fixed_init_copy(&mut (*clip_path).path, path);
    if status as u64 != 0 {
        return _cairo_clip_set_all_clipped(clip);
    }
    (*clip_path).fill_rule = fill_rule;
    (*clip_path).tolerance = tolerance;
    (*clip_path).antialias = antialias;
    if !((*clip).region).is_null() {
        cairo_region_destroy((*clip).region);
        let ref mut fresh16 = (*clip).region;
        *fresh16 = 0 as *mut cairo_region_t;
    }
    (*clip).is_region = 0 as libc::c_int;
    return clip;
}
unsafe extern "C" fn _cairo_clip_intersect_clip_path(
    mut clip: *mut cairo_clip_t,
    mut clip_path: *const cairo_clip_path_t,
) -> *mut cairo_clip_t {
    if !((*clip_path).prev).is_null() {
        clip = _cairo_clip_intersect_clip_path(clip, (*clip_path).prev);
    }
    return _cairo_clip_intersect_path(
        clip,
        &(*clip_path).path,
        (*clip_path).fill_rule,
        (*clip_path).tolerance,
        (*clip_path).antialias,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_intersect_clip(
    mut clip: *mut cairo_clip_t,
    mut other: *const cairo_clip_t,
) -> *mut cairo_clip_t {
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return clip;
    }
    if other.is_null() {
        return clip;
    }
    if clip.is_null() {
        return _cairo_clip_copy(other);
    }
    if _cairo_clip_is_all_clipped(other) != 0 {
        return _cairo_clip_set_all_clipped(clip);
    }
    if _cairo_rectangle_intersect(&mut (*clip).extents, &(*other).extents) == 0 {
        return _cairo_clip_set_all_clipped(clip);
    }
    if (*other).num_boxes != 0 {
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
        _cairo_boxes_init_for_array(&mut boxes, (*other).boxes, (*other).num_boxes);
        clip = _cairo_clip_intersect_boxes(clip, &mut boxes);
    }
    if _cairo_clip_is_all_clipped(clip) == 0 {
        if !((*other).path).is_null() {
            if ((*clip).path).is_null() {
                let ref mut fresh17 = (*clip).path;
                *fresh17 = _cairo_clip_path_reference((*other).path);
            } else {
                clip = _cairo_clip_intersect_clip_path(clip, (*other).path);
            }
        }
    }
    if !((*clip).region).is_null() {
        cairo_region_destroy((*clip).region);
        let ref mut fresh18 = (*clip).region;
        *fresh18 = 0 as *mut cairo_region_t;
    }
    (*clip).is_region = 0 as libc::c_int;
    return clip;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_equal(
    mut clip_a: *const cairo_clip_t,
    mut clip_b: *const cairo_clip_t,
) -> cairo_bool_t {
    let mut cp_a: *const cairo_clip_path_t = 0 as *const cairo_clip_path_t;
    let mut cp_b: *const cairo_clip_path_t = 0 as *const cairo_clip_path_t;
    if clip_a == clip_b {
        return 1 as libc::c_int;
    }
    if clip_a.is_null() || clip_b.is_null() || _cairo_clip_is_all_clipped(clip_a) != 0
        || _cairo_clip_is_all_clipped(clip_b) != 0
    {
        return 0 as libc::c_int;
    }
    if (*clip_a).num_boxes != (*clip_b).num_boxes {
        return 0 as libc::c_int;
    }
    if memcmp(
        (*clip_a).boxes as *const libc::c_void,
        (*clip_b).boxes as *const libc::c_void,
        (::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
            .wrapping_mul((*clip_a).num_boxes as libc::c_ulong),
    ) != 0
    {
        return 0 as libc::c_int;
    }
    cp_a = (*clip_a).path;
    cp_b = (*clip_b).path;
    while !cp_a.is_null() && !cp_b.is_null() {
        if cp_a == cp_b {
            return 1 as libc::c_int;
        }
        if (*cp_a).antialias as libc::c_uint != (*cp_b).antialias as libc::c_uint {
            return 0 as libc::c_int;
        }
        if (*cp_a).tolerance != (*cp_b).tolerance {
            return 0 as libc::c_int;
        }
        if (*cp_a).fill_rule as libc::c_uint != (*cp_b).fill_rule as libc::c_uint {
            return 0 as libc::c_int;
        }
        if _cairo_path_fixed_equal(&(*cp_a).path, &(*cp_b).path) == 0 {
            return 0 as libc::c_int;
        }
        cp_a = (*cp_a).prev;
        cp_b = (*cp_b).prev;
    }
    return (cp_a.is_null() && cp_b.is_null()) as libc::c_int;
}
unsafe extern "C" fn _cairo_clip_path_copy_with_translation(
    mut clip: *mut cairo_clip_t,
    mut other_path: *mut cairo_clip_path_t,
    mut fx: libc::c_int,
    mut fy: libc::c_int,
) -> *mut cairo_clip_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut clip_path: *mut cairo_clip_path_t = 0 as *mut cairo_clip_path_t;
    if !((*other_path).prev).is_null() {
        clip = _cairo_clip_path_copy_with_translation(clip, (*other_path).prev, fx, fy);
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return clip;
    }
    clip_path = _cairo_clip_path_create(clip);
    if clip_path.is_null() {
        return _cairo_clip_set_all_clipped(clip);
    }
    status = _cairo_path_fixed_init_copy(
        &mut (*clip_path).path,
        &mut (*other_path).path,
    );
    if status as u64 != 0 {
        return _cairo_clip_set_all_clipped(clip);
    }
    _cairo_path_fixed_translate(&mut (*clip_path).path, fx, fy);
    (*clip_path).fill_rule = (*other_path).fill_rule;
    (*clip_path).tolerance = (*other_path).tolerance;
    (*clip_path).antialias = (*other_path).antialias;
    return clip;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_translate(
    mut clip: *mut cairo_clip_t,
    mut tx: libc::c_int,
    mut ty: libc::c_int,
) -> *mut cairo_clip_t {
    let mut fx: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut clip_path: *mut cairo_clip_path_t = 0 as *mut cairo_clip_path_t;
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return clip;
    }
    if tx == 0 as libc::c_int && ty == 0 as libc::c_int {
        return clip;
    }
    fx = _cairo_fixed_from_int(tx);
    fy = _cairo_fixed_from_int(ty);
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        let ref mut fresh19 = (*((*clip).boxes).offset(i as isize)).p1.x;
        *fresh19 += fx;
        let ref mut fresh20 = (*((*clip).boxes).offset(i as isize)).p2.x;
        *fresh20 += fx;
        let ref mut fresh21 = (*((*clip).boxes).offset(i as isize)).p1.y;
        *fresh21 += fy;
        let ref mut fresh22 = (*((*clip).boxes).offset(i as isize)).p2.y;
        *fresh22 += fy;
        i += 1;
    }
    (*clip).extents.x += tx;
    (*clip).extents.y += ty;
    if ((*clip).path).is_null() {
        return clip;
    }
    clip_path = (*clip).path;
    let ref mut fresh23 = (*clip).path;
    *fresh23 = 0 as *mut cairo_clip_path_t;
    clip = _cairo_clip_path_copy_with_translation(clip, clip_path, fx, fy);
    _cairo_clip_path_destroy(clip_path);
    return clip;
}
unsafe extern "C" fn _cairo_path_fixed_add_box(
    mut path: *mut cairo_path_fixed_t,
    mut box_0: *const cairo_box_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_path_fixed_move_to(path, (*box_0).p1.x, (*box_0).p1.y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_line_to(path, (*box_0).p2.x, (*box_0).p1.y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_line_to(path, (*box_0).p2.x, (*box_0).p2.y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_line_to(path, (*box_0).p1.x, (*box_0).p2.y);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_path_fixed_close_path(path);
}
unsafe extern "C" fn _cairo_path_fixed_init_from_boxes(
    mut path: *mut cairo_path_fixed_t,
    mut boxes: *const cairo_boxes_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    _cairo_path_fixed_init(path);
    if (*boxes).num_boxes == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    chunk = &(*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            status = _cairo_path_fixed_add_box(
                path,
                &mut *((*chunk).base).offset(i as isize),
            );
            if status as u64 != 0 {
                _cairo_path_fixed_fini(path);
                return status;
            }
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_clip_intersect_clip_path_transformed(
    mut clip: *mut cairo_clip_t,
    mut clip_path: *const cairo_clip_path_t,
    mut m: *const cairo_matrix_t,
) -> *mut cairo_clip_t {
    let mut path: cairo_path_fixed_t = cairo_path_fixed_t {
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
    if !((*clip_path).prev).is_null() {
        clip = _cairo_clip_intersect_clip_path_transformed(clip, (*clip_path).prev, m);
    }
    if _cairo_path_fixed_init_copy(&mut path, &(*clip_path).path) as u64 != 0 {
        return _cairo_clip_set_all_clipped(clip);
    }
    _cairo_path_fixed_transform(&mut path, m);
    clip = _cairo_clip_intersect_path(
        clip,
        &mut path,
        (*clip_path).fill_rule,
        (*clip_path).tolerance,
        (*clip_path).antialias,
    );
    _cairo_path_fixed_fini(&mut path);
    return clip;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_transform(
    mut clip: *mut cairo_clip_t,
    mut m: *const cairo_matrix_t,
) -> *mut cairo_clip_t {
    let mut copy: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return clip;
    }
    if _cairo_matrix_is_translation(m) != 0 {
        return _cairo_clip_translate(
            clip,
            (*m).x0 as libc::c_int,
            (*m).y0 as libc::c_int,
        );
    }
    copy = _cairo_clip_create();
    if (*clip).num_boxes != 0 {
        let mut path: cairo_path_fixed_t = cairo_path_fixed_t {
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
        _cairo_boxes_init_for_array(&mut boxes, (*clip).boxes, (*clip).num_boxes);
        _cairo_path_fixed_init_from_boxes(&mut path, &mut boxes);
        _cairo_path_fixed_transform(&mut path, m);
        copy = _cairo_clip_intersect_path(
            copy,
            &mut path,
            CAIRO_FILL_RULE_WINDING,
            0.1f64,
            CAIRO_ANTIALIAS_DEFAULT,
        );
        _cairo_path_fixed_fini(&mut path);
    }
    if !((*clip).path).is_null() {
        copy = _cairo_clip_intersect_clip_path_transformed(copy, (*clip).path, m);
    }
    _cairo_clip_destroy(clip);
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_copy_with_translation(
    mut clip: *const cairo_clip_t,
    mut tx: libc::c_int,
    mut ty: libc::c_int,
) -> *mut cairo_clip_t {
    let mut copy: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut fx: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if clip.is_null() || _cairo_clip_is_all_clipped(clip) != 0 {
        return clip as *mut cairo_clip_t;
    }
    if tx == 0 as libc::c_int && ty == 0 as libc::c_int {
        return _cairo_clip_copy(clip);
    }
    copy = _cairo_clip_create();
    if copy.is_null() {
        return _cairo_clip_set_all_clipped(copy);
    }
    fx = _cairo_fixed_from_int(tx);
    fy = _cairo_fixed_from_int(ty);
    if (*clip).num_boxes != 0 {
        if (*clip).num_boxes == 1 as libc::c_int {
            let ref mut fresh24 = (*copy).boxes;
            *fresh24 = &mut (*copy).embedded_box;
        } else {
            let ref mut fresh25 = (*copy).boxes;
            *fresh25 = _cairo_malloc_ab(
                (*clip).num_boxes as size_t,
                ::std::mem::size_of::<cairo_box_t>() as libc::c_ulong,
            ) as *mut cairo_box_t;
            if ((*copy).boxes).is_null() {
                return _cairo_clip_set_all_clipped(copy);
            }
        }
        i = 0 as libc::c_int;
        while i < (*clip).num_boxes {
            (*((*copy).boxes).offset(i as isize))
                .p1
                .x = (*((*clip).boxes).offset(i as isize)).p1.x + fx;
            (*((*copy).boxes).offset(i as isize))
                .p2
                .x = (*((*clip).boxes).offset(i as isize)).p2.x + fx;
            (*((*copy).boxes).offset(i as isize))
                .p1
                .y = (*((*clip).boxes).offset(i as isize)).p1.y + fy;
            (*((*copy).boxes).offset(i as isize))
                .p2
                .y = (*((*clip).boxes).offset(i as isize)).p2.y + fy;
            i += 1;
        }
        (*copy).num_boxes = (*clip).num_boxes;
    }
    (*copy).extents = (*clip).extents;
    (*copy).extents.x += tx;
    (*copy).extents.y += ty;
    if ((*clip).path).is_null() {
        return copy;
    }
    return _cairo_clip_path_copy_with_translation(copy, (*clip).path, fx, fy);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_contains_extents(
    mut clip: *const cairo_clip_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut rect: *const cairo_rectangle_int_t = 0 as *const cairo_rectangle_int_t;
    rect = if (*extents).is_bounded != 0 {
        &(*extents).bounded
    } else {
        &(*extents).unbounded
    };
    return _cairo_clip_contains_rectangle(clip, rect);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_clip(
    mut stream: *mut FILE,
    mut clip: *const cairo_clip_t,
) {
    let mut i: libc::c_int = 0;
    if clip.is_null() {
        fprintf(stream, b"no clip\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        fprintf(stream, b"clip: all-clipped\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    fprintf(stream, b"clip:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stream,
        b"  extents: (%d, %d) x (%d, %d), is-region? %d\0" as *const u8
            as *const libc::c_char,
        (*clip).extents.x,
        (*clip).extents.y,
        (*clip).extents.width,
        (*clip).extents.height,
        (*clip).is_region,
    );
    fprintf(
        stream,
        b"  num_boxes = %d\n\0" as *const u8 as *const libc::c_char,
        (*clip).num_boxes,
    );
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        fprintf(
            stream,
            b"  [%d] = (%f, %f), (%f, %f)\n\0" as *const u8 as *const libc::c_char,
            i,
            _cairo_fixed_to_double((*((*clip).boxes).offset(i as isize)).p1.x),
            _cairo_fixed_to_double((*((*clip).boxes).offset(i as isize)).p1.y),
            _cairo_fixed_to_double((*((*clip).boxes).offset(i as isize)).p2.x),
            _cairo_fixed_to_double((*((*clip).boxes).offset(i as isize)).p2.y),
        );
        i += 1;
    }
    if !((*clip).path).is_null() {
        let mut clip_path: *mut cairo_clip_path_t = (*clip).path;
        loop {
            fprintf(
                stream,
                b"path: aa=%d, tolerance=%f, rule=%d: \0" as *const u8
                    as *const libc::c_char,
                (*clip_path).antialias as libc::c_uint,
                (*clip_path).tolerance,
                (*clip_path).fill_rule as libc::c_uint,
            );
            _cairo_debug_print_path(stream, &mut (*clip_path).path);
            fprintf(stream, b"\n\0" as *const u8 as *const libc::c_char);
            clip_path = (*clip_path).prev;
            if clip_path.is_null() {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_get_extents(
    mut clip: *const cairo_clip_t,
) -> *const cairo_rectangle_int_t {
    if clip.is_null() {
        return &_cairo_unbounded_rectangle;
    }
    if _cairo_clip_is_all_clipped(clip) != 0 {
        return &_cairo_empty_rectangle;
    }
    return &(*clip).extents;
}
#[no_mangle]
pub static mut _cairo_rectangles_nil: cairo_rectangle_list_t = {
    let mut init = _cairo_rectangle_list {
        status: CAIRO_STATUS_NO_MEMORY,
        rectangles: 0 as *const cairo_rectangle_t as *mut cairo_rectangle_t,
        num_rectangles: 0 as libc::c_int,
    };
    init
};
static mut _cairo_rectangles_not_representable: cairo_rectangle_list_t = {
    let mut init = _cairo_rectangle_list {
        status: CAIRO_STATUS_CLIP_NOT_REPRESENTABLE,
        rectangles: 0 as *const cairo_rectangle_t as *mut cairo_rectangle_t,
        num_rectangles: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn _cairo_clip_int_rect_to_user(
    mut gstate: *mut cairo_gstate_t,
    mut clip_rect: *mut cairo_rectangle_int_t,
    mut user_rect: *mut cairo_rectangle_t,
) -> cairo_bool_t {
    let mut is_tight: cairo_bool_t = 0;
    let mut x1: libc::c_double = (*clip_rect).x as libc::c_double;
    let mut y1: libc::c_double = (*clip_rect).y as libc::c_double;
    let mut x2: libc::c_double = ((*clip_rect).x + (*clip_rect).width) as libc::c_double;
    let mut y2: libc::c_double = ((*clip_rect).y + (*clip_rect).height)
        as libc::c_double;
    _cairo_gstate_backend_to_user_rectangle(
        gstate,
        &mut x1,
        &mut y1,
        &mut x2,
        &mut y2,
        &mut is_tight,
    );
    (*user_rect).x = x1;
    (*user_rect).y = y1;
    (*user_rect).width = x2 - x1;
    (*user_rect).height = y2 - y1;
    return is_tight;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_rectangle_list_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_rectangle_list_t {
    let mut list: *mut cairo_rectangle_list_t = 0 as *mut cairo_rectangle_list_t;
    if status as libc::c_uint == CAIRO_STATUS_NO_MEMORY as libc::c_int as libc::c_uint {
        return &_cairo_rectangles_nil as *const cairo_rectangle_list_t
            as *mut cairo_rectangle_list_t;
    }
    if status as libc::c_uint
        == CAIRO_STATUS_CLIP_NOT_REPRESENTABLE as libc::c_int as libc::c_uint
    {
        return &_cairo_rectangles_not_representable as *const cairo_rectangle_list_t
            as *mut cairo_rectangle_list_t;
    }
    list = (if ::std::mem::size_of::<cairo_rectangle_list_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_rectangle_list_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_rectangle_list_t;
    if list.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_rectangles_nil as *const cairo_rectangle_list_t
            as *mut cairo_rectangle_list_t;
    }
    (*list).status = status;
    let ref mut fresh26 = (*list).rectangles;
    *fresh26 = 0 as *mut cairo_rectangle_t;
    (*list).num_rectangles = 0 as libc::c_int;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_copy_rectangle_list(
    mut clip: *mut cairo_clip_t,
    mut gstate: *mut cairo_gstate_t,
) -> *mut cairo_rectangle_list_t {
    let mut list: *mut cairo_rectangle_list_t = 0 as *mut cairo_rectangle_list_t;
    let mut rectangles: *mut cairo_rectangle_t = 0 as *mut cairo_rectangle_t;
    let mut region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    let mut n_rects: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if clip.is_null() {
        return _cairo_rectangle_list_create_in_error(
            _cairo_error(CAIRO_STATUS_CLIP_NOT_REPRESENTABLE),
        );
    }
    if !(_cairo_clip_is_all_clipped(clip) != 0) {
        if _cairo_clip_is_region(clip) == 0 {
            return _cairo_rectangle_list_create_in_error(
                _cairo_error(CAIRO_STATUS_CLIP_NOT_REPRESENTABLE),
            );
        }
        region = _cairo_clip_get_region(clip);
        if region.is_null() {
            return _cairo_rectangle_list_create_in_error(
                _cairo_error(CAIRO_STATUS_NO_MEMORY),
            );
        }
        n_rects = cairo_region_num_rectangles(region);
        if n_rects != 0 {
            rectangles = _cairo_malloc_ab(
                n_rects as size_t,
                ::std::mem::size_of::<cairo_rectangle_t>() as libc::c_ulong,
            ) as *mut cairo_rectangle_t;
            if rectangles.is_null() {
                return _cairo_rectangle_list_create_in_error(
                    _cairo_error(CAIRO_STATUS_NO_MEMORY),
                );
            }
            i = 0 as libc::c_int;
            while i < n_rects {
                let mut clip_rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                cairo_region_get_rectangle(region, i, &mut clip_rect);
                if _cairo_clip_int_rect_to_user(
                    gstate,
                    &mut clip_rect,
                    &mut *rectangles.offset(i as isize),
                ) == 0
                {
                    free(rectangles as *mut libc::c_void);
                    return _cairo_rectangle_list_create_in_error(
                        _cairo_error(CAIRO_STATUS_CLIP_NOT_REPRESENTABLE),
                    );
                }
                i += 1;
            }
        }
    }
    list = (if ::std::mem::size_of::<cairo_rectangle_list_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_rectangle_list_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_rectangle_list_t;
    if list.is_null() {
        free(rectangles as *mut libc::c_void);
        return _cairo_rectangle_list_create_in_error(
            _cairo_error(CAIRO_STATUS_NO_MEMORY),
        );
    }
    (*list).status = CAIRO_STATUS_SUCCESS;
    let ref mut fresh27 = (*list).rectangles;
    *fresh27 = rectangles;
    (*list).num_rectangles = n_rects;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_rectangle_list_destroy(
    mut rectangle_list: *mut cairo_rectangle_list_t,
) {
    if rectangle_list.is_null()
        || rectangle_list
            == &_cairo_rectangles_nil as *const cairo_rectangle_list_t
                as *mut cairo_rectangle_list_t
        || rectangle_list
            == &_cairo_rectangles_not_representable as *const cairo_rectangle_list_t
                as *mut cairo_rectangle_list_t
    {
        return;
    }
    free((*rectangle_list).rectangles as *mut libc::c_void);
    free(rectangle_list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_clip_reset_static_data() {
    _freed_pool_reset(&mut clip_path_pool);
    _freed_pool_reset(&mut clip_pool);
}
