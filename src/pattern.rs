use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn _cairo_user_data_array_init(array: *mut cairo_user_data_array_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_color_init_rgba(
        color: *mut cairo_color_t,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
        alpha: libc::c_double,
    );
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_user_data_array_fini(array: *mut cairo_user_data_array_t);
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
    fn _cairo_color_double_to_short(d: libc::c_double) -> uint16_t;
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn _cairo_color_get_rgba(
        color: *mut cairo_color_t,
        red: *mut libc::c_double,
        green: *mut libc::c_double,
        blue: *mut libc::c_double,
        alpha: *mut libc::c_double,
    );
    fn cairo_matrix_init_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_hash_bytes(
        hash: uintptr_t,
        bytes: *const libc::c_void,
        length: libc::c_uint,
    ) -> uintptr_t;
    fn _cairo_color_equal(
        color_a: *const cairo_color_t,
        color_b: *const cairo_color_t,
    ) -> cairo_bool_t;
    fn _cairo_color_stop_equal(
        color_a: *const cairo_color_stop_t,
        color_b: *const cairo_color_stop_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_snapshot(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_matrix_transform_bounding_box(
        matrix: *const cairo_matrix_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_matrix_is_pixel_exact(matrix: *const cairo_matrix_t) -> cairo_bool_t;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_append_multiple(
        array: *mut cairo_array_t,
        elements: *const libc::c_void,
        num_elements: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_array_allocate(
        array: *mut cairo_array_t,
        num_elements: libc::c_uint,
        elements: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_index_const(
        array: *const cairo_array_t,
        index: libc::c_uint,
    ) -> *const libc::c_void;
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn _freed_pool_get_search(pool: *mut freed_pool_t) -> *mut libc::c_void;
    fn _freed_pool_put_search(pool: *mut freed_pool_t, ptr: *mut libc::c_void);
    fn _freed_pool_reset(pool: *mut freed_pool_t);
    fn _cairo_path_create_in_error(status: cairo_status_t) -> *mut cairo_path_t;
    fn _cairo_raster_source_pattern_snapshot(
        abstract_pattern: *mut cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_raster_source_pattern_init_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_raster_source_pattern_finish(abstract_pattern: *mut cairo_pattern_t);
    fn _cairo_recording_surface_get_ink_bbox(
        surface: *mut cairo_recording_surface_t,
        bbox: *mut cairo_box_t,
        transform: *const cairo_matrix_t,
    ) -> cairo_status_t;
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
pub type ptrdiff_t = libc::c_long;
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
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
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freed_pool_t {
    pub pool: [*mut libc::c_void; 16],
    pub top: cairo_atomic_int_t,
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
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
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
pub const CAIRO_PATTERN_NOTIFY_MATRIX: C2RustUnnamed_2 = 1;
pub type cairo_pattern_observer_t = _cairo_pattern_observer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pattern_observer {
    pub notify: Option::<
        unsafe extern "C" fn(
            *mut cairo_pattern_observer_t,
            *mut cairo_pattern_t,
            libc::c_uint,
        ) -> (),
    >,
    pub link: cairo_list_t,
}
pub const CAIRO_PATTERN_NOTIFY_EXTEND: C2RustUnnamed_2 = 4;
pub const CAIRO_PATTERN_NOTIFY_FILTER: C2RustUnnamed_2 = 2;
pub type cairo_fixed_unsigned_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CAIRO_PATTERN_NOTIFY_OPACITY: C2RustUnnamed_2 = 9;
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_gradient_pattern_union_t {
    pub base: cairo_gradient_pattern_t,
    pub linear: cairo_linear_pattern_t,
    pub radial: cairo_radial_pattern_t,
}
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
unsafe extern "C" fn _cairo_atomic_int_set_relaxed(
    mut x: *mut cairo_atomic_int_t,
    mut val: cairo_atomic_int_t,
) {
    ::std::intrinsics::atomic_store_relaxed(x, val);
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
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_ptr_get(
    mut x: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get_relaxed(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load_relaxed(x);
}
#[inline]
unsafe extern "C" fn _cairo_restrict_value(
    mut value: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    if value < min {
        return min
    } else if value > max {
        return max
    } else {
        return value
    };
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_cmpxchg_impl(
    mut x: *mut cairo_atomic_int_t,
    mut oldv: cairo_atomic_int_t,
    mut newv: cairo_atomic_int_t,
) -> cairo_bool_t {
    let mut expected: cairo_atomic_int_t = oldv;
    let fresh1 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh1.0;
    return fresh1.1 as cairo_bool_t;
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh4, fresh5) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh4;
    if fresh5 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_unbounded_rectangle_init(
    mut rect: *mut cairo_rectangle_int_t,
) {
    *rect = _cairo_unbounded_rectangle;
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
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
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
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_matrix_is_translation(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64) as libc::c_int;
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
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh6 = (*entry).next;
    *fresh6 = entry;
    let ref mut fresh7 = (*entry).prev;
    *fresh7 = entry;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_recording(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint) as libc::c_int;
}
static mut freed_pattern_pool: [freed_pool_t; 5] = [freed_pool_t {
    pool: [0 as *const libc::c_void as *mut libc::c_void; 16],
    top: 0,
}; 5];
static mut _cairo_pattern_nil: cairo_solid_pattern_t = {
    let mut init = _cairo_solid_pattern {
        base: {
            let mut init = _cairo_pattern {
                ref_count: {
                    let mut init = cairo_reference_count_t {
                        ref_count: -(1 as libc::c_int),
                    };
                    init
                },
                status: CAIRO_STATUS_NO_MEMORY,
                user_data: {
                    let mut init = _cairo_array {
                        size: 0 as libc::c_int as libc::c_uint,
                        num_elements: 0 as libc::c_int as libc::c_uint,
                        element_size: 0 as libc::c_int as libc::c_uint,
                        elements: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                observers: {
                    let mut init = _cairo_list {
                        next: 0 as *const _cairo_list as *mut _cairo_list,
                        prev: 0 as *const _cairo_list as *mut _cairo_list,
                    };
                    init
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_GOOD,
                extend: CAIRO_EXTEND_PAD,
                has_component_alpha: 0 as libc::c_int,
                is_userfont_foreground: 0 as libc::c_int,
                matrix: {
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
                opacity: 1.0f64,
            };
            init
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
    init
};
static mut _cairo_pattern_nil_null_pointer: cairo_solid_pattern_t = {
    let mut init = _cairo_solid_pattern {
        base: {
            let mut init = _cairo_pattern {
                ref_count: {
                    let mut init = cairo_reference_count_t {
                        ref_count: -(1 as libc::c_int),
                    };
                    init
                },
                status: CAIRO_STATUS_NULL_POINTER,
                user_data: {
                    let mut init = _cairo_array {
                        size: 0 as libc::c_int as libc::c_uint,
                        num_elements: 0 as libc::c_int as libc::c_uint,
                        element_size: 0 as libc::c_int as libc::c_uint,
                        elements: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                observers: {
                    let mut init = _cairo_list {
                        next: 0 as *const _cairo_list as *mut _cairo_list,
                        prev: 0 as *const _cairo_list as *mut _cairo_list,
                    };
                    init
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_GOOD,
                extend: CAIRO_EXTEND_PAD,
                has_component_alpha: 0 as libc::c_int,
                is_userfont_foreground: 0 as libc::c_int,
                matrix: {
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
                opacity: 1.0f64,
            };
            init
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
    init
};
#[no_mangle]
pub static mut _cairo_pattern_black: cairo_solid_pattern_t = {
    let mut init = _cairo_solid_pattern {
        base: {
            let mut init = _cairo_pattern {
                ref_count: {
                    let mut init = cairo_reference_count_t {
                        ref_count: -(1 as libc::c_int),
                    };
                    init
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: {
                    let mut init = _cairo_array {
                        size: 0 as libc::c_int as libc::c_uint,
                        num_elements: 0 as libc::c_int as libc::c_uint,
                        element_size: 0 as libc::c_int as libc::c_uint,
                        elements: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                observers: {
                    let mut init = _cairo_list {
                        next: 0 as *const _cairo_list as *mut _cairo_list,
                        prev: 0 as *const _cairo_list as *mut _cairo_list,
                    };
                    init
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_NEAREST,
                extend: CAIRO_EXTEND_REPEAT,
                has_component_alpha: 0 as libc::c_int,
                is_userfont_foreground: 0 as libc::c_int,
                matrix: {
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
                opacity: 1.0f64,
            };
            init
        },
        color: {
            let mut init = _cairo_color {
                red: 0.0f64,
                green: 0.0f64,
                blue: 0.0f64,
                alpha: 1.0f64,
                red_short: 0 as libc::c_int as libc::c_ushort,
                green_short: 0 as libc::c_int as libc::c_ushort,
                blue_short: 0 as libc::c_int as libc::c_ushort,
                alpha_short: 0xffff as libc::c_int as libc::c_ushort,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut _cairo_pattern_clear: cairo_solid_pattern_t = {
    let mut init = _cairo_solid_pattern {
        base: {
            let mut init = _cairo_pattern {
                ref_count: {
                    let mut init = cairo_reference_count_t {
                        ref_count: -(1 as libc::c_int),
                    };
                    init
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: {
                    let mut init = _cairo_array {
                        size: 0 as libc::c_int as libc::c_uint,
                        num_elements: 0 as libc::c_int as libc::c_uint,
                        element_size: 0 as libc::c_int as libc::c_uint,
                        elements: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                observers: {
                    let mut init = _cairo_list {
                        next: 0 as *const _cairo_list as *mut _cairo_list,
                        prev: 0 as *const _cairo_list as *mut _cairo_list,
                    };
                    init
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_NEAREST,
                extend: CAIRO_EXTEND_REPEAT,
                has_component_alpha: 0 as libc::c_int,
                is_userfont_foreground: 0 as libc::c_int,
                matrix: {
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
                opacity: 1.0f64,
            };
            init
        },
        color: {
            let mut init = _cairo_color {
                red: 0.0f64,
                green: 0.0f64,
                blue: 0.0f64,
                alpha: 0.0f64,
                red_short: 0 as libc::c_int as libc::c_ushort,
                green_short: 0 as libc::c_int as libc::c_ushort,
                blue_short: 0 as libc::c_int as libc::c_ushort,
                alpha_short: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut _cairo_pattern_white: cairo_solid_pattern_t = {
    let mut init = _cairo_solid_pattern {
        base: {
            let mut init = _cairo_pattern {
                ref_count: {
                    let mut init = cairo_reference_count_t {
                        ref_count: -(1 as libc::c_int),
                    };
                    init
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: {
                    let mut init = _cairo_array {
                        size: 0 as libc::c_int as libc::c_uint,
                        num_elements: 0 as libc::c_int as libc::c_uint,
                        element_size: 0 as libc::c_int as libc::c_uint,
                        elements: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                observers: {
                    let mut init = _cairo_list {
                        next: 0 as *const _cairo_list as *mut _cairo_list,
                        prev: 0 as *const _cairo_list as *mut _cairo_list,
                    };
                    init
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_NEAREST,
                extend: CAIRO_EXTEND_REPEAT,
                has_component_alpha: 0 as libc::c_int,
                is_userfont_foreground: 0 as libc::c_int,
                matrix: {
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
                opacity: 1.0f64,
            };
            init
        },
        color: {
            let mut init = _cairo_color {
                red: 1.0f64,
                green: 1.0f64,
                blue: 1.0f64,
                alpha: 1.0f64,
                red_short: 0xffff as libc::c_int as libc::c_ushort,
                green_short: 0xffff as libc::c_int as libc::c_ushort,
                blue_short: 0xffff as libc::c_int as libc::c_ushort,
                alpha_short: 0xffff as libc::c_int as libc::c_ushort,
            };
            init
        },
    };
    init
};
unsafe extern "C" fn _cairo_pattern_notify_observers(
    mut pattern: *mut cairo_pattern_t,
    mut flags: libc::c_uint,
) {
    let mut pos: *mut cairo_pattern_observer_t = 0 as *mut cairo_pattern_observer_t;
    pos = ({
        let mut mptr__: *const cairo_list_t = (*pattern).observers.next;
        (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
            as *mut cairo_pattern_observer_t
    });
    while &mut (*pos).link as *mut cairo_list_t
        != &mut (*pattern).observers as *mut cairo_list_t
    {
        ((*pos).notify).expect("non-null function pointer")(pos, pattern, flags);
        pos = ({
            let mut mptr__: *const cairo_list_t = (*pos).link.next;
            (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                as *mut cairo_pattern_observer_t
        });
    }
}
unsafe extern "C" fn _cairo_pattern_set_error(
    mut pattern: *mut cairo_pattern_t,
    mut status: cairo_status_t,
) -> cairo_status_t {
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return status;
    }
    let mut ret__: libc::c_int = 0;
    if (status as libc::c_uint) < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status < CAIRO_STATUS_LAST_STATUS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"cairo_status_t _cairo_pattern_set_error(cairo_pattern_t *, cairo_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    ret__ = _cairo_atomic_int_cmpxchg_impl(
        &mut (*pattern).status as *mut cairo_status_t as *mut cairo_atomic_int_t,
        CAIRO_STATUS_SUCCESS as libc::c_int,
        status as cairo_atomic_int_t,
    );
    return _cairo_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_init(
    mut pattern: *mut cairo_pattern_t,
    mut type_0: cairo_pattern_type_t,
) {
    (*pattern).type_0 = type_0;
    (*pattern).status = CAIRO_STATUS_SUCCESS;
    (*pattern).ref_count.ref_count = 0 as libc::c_int;
    _cairo_user_data_array_init(&mut (*pattern).user_data);
    if type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        (*pattern).extend = CAIRO_EXTEND_NONE;
    } else {
        (*pattern).extend = CAIRO_EXTEND_PAD;
    }
    (*pattern).filter = CAIRO_FILTER_GOOD;
    (*pattern).opacity = 1.0f64;
    (*pattern).has_component_alpha = 0 as libc::c_int;
    (*pattern).is_userfont_foreground = 0 as libc::c_int;
    cairo_matrix_init_identity(&mut (*pattern).matrix);
    cairo_list_init(&mut (*pattern).observers);
}
unsafe extern "C" fn _cairo_gradient_pattern_init_copy(
    mut pattern: *mut cairo_gradient_pattern_t,
    mut other: *const cairo_gradient_pattern_t,
) -> cairo_status_t {
    if (*other).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        let mut dst: *mut cairo_linear_pattern_t = pattern
            as *mut cairo_linear_pattern_t;
        let mut src: *mut cairo_linear_pattern_t = other as *mut cairo_linear_pattern_t;
        *dst = *src;
    } else {
        let mut dst_0: *mut cairo_radial_pattern_t = pattern
            as *mut cairo_radial_pattern_t;
        let mut src_0: *mut cairo_radial_pattern_t = other
            as *mut cairo_radial_pattern_t;
        *dst_0 = *src_0;
    }
    if (*other).stops == ((*other).stops_embedded).as_ptr() as *mut cairo_gradient_stop_t
    {
        let ref mut fresh8 = (*pattern).stops;
        *fresh8 = ((*pattern).stops_embedded).as_mut_ptr();
    } else if !((*other).stops).is_null() {
        let ref mut fresh9 = (*pattern).stops;
        *fresh9 = _cairo_malloc_ab(
            (*other).stops_size as size_t,
            ::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong,
        ) as *mut cairo_gradient_stop_t;
        if ((*pattern).stops).is_null() {
            (*pattern).stops_size = 0 as libc::c_int as libc::c_uint;
            (*pattern).n_stops = 0 as libc::c_int as libc::c_uint;
            return _cairo_pattern_set_error(
                &mut (*pattern).base,
                CAIRO_STATUS_NO_MEMORY,
            );
        }
        memcpy(
            (*pattern).stops as *mut libc::c_void,
            (*other).stops as *const libc::c_void,
            ((*other).n_stops as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong,
                ),
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_mesh_pattern_init_copy(
    mut pattern: *mut cairo_mesh_pattern_t,
    mut other: *const cairo_mesh_pattern_t,
) -> cairo_status_t {
    *pattern = *other;
    _cairo_array_init(
        &mut (*pattern).patches,
        ::std::mem::size_of::<cairo_mesh_patch_t>() as libc::c_ulong as libc::c_uint,
    );
    return _cairo_array_append_multiple(
        &mut (*pattern).patches,
        _cairo_array_index_const(&(*other).patches, 0 as libc::c_int as libc::c_uint),
        _cairo_array_num_elements(&(*other).patches),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_init_copy(
    mut pattern: *mut cairo_pattern_t,
    mut other: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*other).status as u64 != 0 {
        return _cairo_pattern_set_error(pattern, (*other).status);
    }
    match (*other).type_0 as libc::c_uint {
        0 => {
            let mut dst: *mut cairo_solid_pattern_t = pattern
                as *mut cairo_solid_pattern_t;
            let mut src: *mut cairo_solid_pattern_t = other
                as *mut cairo_solid_pattern_t;
            *dst = *src;
        }
        1 => {
            let mut dst_0: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            let mut src_0: *mut cairo_surface_pattern_t = other
                as *mut cairo_surface_pattern_t;
            *dst_0 = *src_0;
            cairo_surface_reference((*dst_0).surface);
        }
        2 | 3 => {
            let mut dst_1: *mut cairo_gradient_pattern_t = pattern
                as *mut cairo_gradient_pattern_t;
            let mut src_1: *mut cairo_gradient_pattern_t = other
                as *mut cairo_gradient_pattern_t;
            (*other).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint;
            status = _cairo_gradient_pattern_init_copy(dst_1, src_1);
            if status as u64 != 0 {
                return status;
            }
        }
        4 => {
            let mut dst_2: *mut cairo_mesh_pattern_t = pattern
                as *mut cairo_mesh_pattern_t;
            let mut src_2: *mut cairo_mesh_pattern_t = other
                as *mut cairo_mesh_pattern_t;
            status = _cairo_mesh_pattern_init_copy(dst_2, src_2);
            if status as u64 != 0 {
                return status;
            }
        }
        5 => {
            status = _cairo_raster_source_pattern_init_copy(pattern, other);
            if status as u64 != 0 {
                return status;
            }
        }
        _ => {}
    }
    (*pattern).ref_count.ref_count = 0 as libc::c_int;
    _cairo_user_data_array_init(&mut (*pattern).user_data);
    cairo_list_init(&mut (*pattern).observers);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_init_static_copy(
    mut pattern: *mut cairo_pattern_t,
    mut other: *const cairo_pattern_t,
) {
    let mut size: libc::c_int = 0;
    if (*other).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"other->status == CAIRO_STATUS_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            377 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"void _cairo_pattern_init_static_copy(cairo_pattern_t *, const cairo_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut current_block_9: u64;
    match (*other).type_0 as libc::c_uint {
        0 => {
            current_block_9 = 17943626125737661905;
        }
        1 => {
            size = ::std::mem::size_of::<cairo_surface_pattern_t>() as libc::c_ulong
                as libc::c_int;
            current_block_9 = 9606288038608642794;
        }
        2 => {
            size = ::std::mem::size_of::<cairo_linear_pattern_t>() as libc::c_ulong
                as libc::c_int;
            current_block_9 = 9606288038608642794;
        }
        3 => {
            size = ::std::mem::size_of::<cairo_radial_pattern_t>() as libc::c_ulong
                as libc::c_int;
            current_block_9 = 9606288038608642794;
        }
        4 => {
            size = ::std::mem::size_of::<cairo_mesh_pattern_t>() as libc::c_ulong
                as libc::c_int;
            current_block_9 = 9606288038608642794;
        }
        5 => {
            size = ::std::mem::size_of::<cairo_raster_source_pattern_t>()
                as libc::c_ulong as libc::c_int;
            current_block_9 = 9606288038608642794;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    381 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 81],
                        &[libc::c_char; 81],
                    >(
                        b"void _cairo_pattern_init_static_copy(cairo_pattern_t *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_9 = 17943626125737661905;
        }
    }
    match current_block_9 {
        17943626125737661905 => {
            size = ::std::mem::size_of::<cairo_solid_pattern_t>() as libc::c_ulong
                as libc::c_int;
        }
        _ => {}
    }
    memcpy(
        pattern as *mut libc::c_void,
        other as *const libc::c_void,
        size as libc::c_ulong,
    );
    (*pattern).ref_count.ref_count = 0 as libc::c_int;
    _cairo_user_data_array_init(&mut (*pattern).user_data);
    cairo_list_init(&mut (*pattern).observers);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_init_snapshot(
    mut pattern: *mut cairo_pattern_t,
    mut other: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_pattern_init_copy(pattern, other);
    if status as u64 != 0 {
        return status;
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        let mut surface_pattern: *mut cairo_surface_pattern_t = pattern
            as *mut cairo_surface_pattern_t;
        let mut surface: *mut cairo_surface_t = (*surface_pattern).surface;
        let ref mut fresh10 = (*surface_pattern).surface;
        *fresh10 = _cairo_surface_snapshot(surface);
        cairo_surface_destroy(surface);
        status = (*(*surface_pattern).surface).status;
    } else if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        status = _cairo_raster_source_pattern_snapshot(pattern);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_fini(mut pattern: *mut cairo_pattern_t) {
    _cairo_user_data_array_fini(&mut (*pattern).user_data);
    match (*pattern).type_0 as libc::c_uint {
        1 => {
            let mut surface_pattern: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            cairo_surface_destroy((*surface_pattern).surface);
        }
        2 | 3 => {
            let mut gradient: *mut cairo_gradient_pattern_t = pattern
                as *mut cairo_gradient_pattern_t;
            if !((*gradient).stops).is_null()
                && (*gradient).stops != ((*gradient).stops_embedded).as_mut_ptr()
            {
                free((*gradient).stops as *mut libc::c_void);
            }
        }
        4 => {
            let mut mesh: *mut cairo_mesh_pattern_t = pattern
                as *mut cairo_mesh_pattern_t;
            _cairo_array_fini(&mut (*mesh).patches);
        }
        5 => {
            _cairo_raster_source_pattern_finish(pattern);
        }
        0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_create_copy(
    mut pattern_out: *mut *mut cairo_pattern_t,
    mut other: *const cairo_pattern_t,
) -> cairo_status_t {
    let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*other).status as u64 != 0 {
        return (*other).status;
    }
    match (*other).type_0 as libc::c_uint {
        0 => {
            pattern = (if ::std::mem::size_of::<cairo_solid_pattern_t>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_solid_pattern_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pattern_t;
        }
        1 => {
            pattern = (if ::std::mem::size_of::<cairo_surface_pattern_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_surface_pattern_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pattern_t;
        }
        2 => {
            pattern = (if ::std::mem::size_of::<cairo_linear_pattern_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_linear_pattern_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pattern_t;
        }
        3 => {
            pattern = (if ::std::mem::size_of::<cairo_radial_pattern_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_radial_pattern_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pattern_t;
        }
        4 => {
            pattern = (if ::std::mem::size_of::<cairo_mesh_pattern_t>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
            {
                malloc(::std::mem::size_of::<cairo_mesh_pattern_t>() as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pattern_t;
        }
        5 => {
            pattern = (if ::std::mem::size_of::<cairo_raster_source_pattern_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(
                    ::std::mem::size_of::<cairo_raster_source_pattern_t>()
                        as libc::c_ulong,
                )
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pattern_t;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    525 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 87],
                        &[libc::c_char; 87],
                    >(
                        b"cairo_status_t _cairo_pattern_create_copy(cairo_pattern_t **, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        }
    }
    if pattern.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _cairo_pattern_init_copy(pattern, other);
    if status as u64 != 0 {
        free(pattern as *mut libc::c_void);
        return status;
    }
    (*pattern).ref_count.ref_count = 1 as libc::c_int;
    *pattern_out = pattern;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_init_solid(
    mut pattern: *mut cairo_solid_pattern_t,
    mut color: *const cairo_color_t,
) {
    _cairo_pattern_init(&mut (*pattern).base, CAIRO_PATTERN_TYPE_SOLID);
    (*pattern).color = *color;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_init_for_surface(
    mut pattern: *mut cairo_surface_pattern_t,
    mut surface: *mut cairo_surface_t,
) {
    if (*surface).status as u64 != 0 {
        _cairo_pattern_init(&mut (*pattern).base, CAIRO_PATTERN_TYPE_SOLID);
        _cairo_pattern_set_error(&mut (*pattern).base, (*surface).status);
        return;
    }
    _cairo_pattern_init(&mut (*pattern).base, CAIRO_PATTERN_TYPE_SURFACE);
    let ref mut fresh11 = (*pattern).surface;
    *fresh11 = cairo_surface_reference(surface);
}
unsafe extern "C" fn _cairo_pattern_init_gradient(
    mut pattern: *mut cairo_gradient_pattern_t,
    mut type_0: cairo_pattern_type_t,
) {
    _cairo_pattern_init(&mut (*pattern).base, type_0);
    (*pattern).n_stops = 0 as libc::c_int as libc::c_uint;
    (*pattern).stops_size = 0 as libc::c_int as libc::c_uint;
    let ref mut fresh12 = (*pattern).stops;
    *fresh12 = 0 as *mut cairo_gradient_stop_t;
}
unsafe extern "C" fn _cairo_pattern_init_linear(
    mut pattern: *mut cairo_linear_pattern_t,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
) {
    _cairo_pattern_init_gradient(&mut (*pattern).base, CAIRO_PATTERN_TYPE_LINEAR);
    (*pattern).pd1.x = x0;
    (*pattern).pd1.y = y0;
    (*pattern).pd2.x = x1;
    (*pattern).pd2.y = y1;
}
unsafe extern "C" fn _cairo_pattern_init_radial(
    mut pattern: *mut cairo_radial_pattern_t,
    mut cx0: libc::c_double,
    mut cy0: libc::c_double,
    mut radius0: libc::c_double,
    mut cx1: libc::c_double,
    mut cy1: libc::c_double,
    mut radius1: libc::c_double,
) {
    _cairo_pattern_init_gradient(&mut (*pattern).base, CAIRO_PATTERN_TYPE_RADIAL);
    (*pattern).cd1.center.x = cx0;
    (*pattern).cd1.center.y = cy0;
    (*pattern).cd1.radius = fabs(radius0);
    (*pattern).cd2.center.x = cx1;
    (*pattern).cd2.center.y = cy1;
    (*pattern).cd2.radius = fabs(radius1);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_create_solid(
    mut color: *const cairo_color_t,
) -> *mut cairo_pattern_t {
    let mut pattern: *mut cairo_solid_pattern_t = 0 as *mut cairo_solid_pattern_t;
    pattern = _freed_pool_get(
        &mut *freed_pattern_pool
            .as_mut_ptr()
            .offset(CAIRO_PATTERN_TYPE_SOLID as libc::c_int as isize),
    ) as *mut cairo_solid_pattern_t;
    if pattern.is_null() {
        pattern = (if ::std::mem::size_of::<cairo_solid_pattern_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_solid_pattern_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_solid_pattern_t;
        if pattern.is_null() {
            let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_pattern_nil as *const cairo_solid_pattern_t
                as *mut cairo_pattern_t;
        }
    }
    _cairo_pattern_init_solid(pattern, color);
    (*pattern).base.ref_count.ref_count = 1 as libc::c_int;
    return &mut (*pattern).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_pattern_t {
    let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
    if status as libc::c_uint == CAIRO_STATUS_NO_MEMORY as libc::c_int as libc::c_uint {
        return &_cairo_pattern_nil.base as *const cairo_pattern_t
            as *mut cairo_pattern_t;
    }
    pattern = _cairo_pattern_create_solid(_cairo_stock_color(CAIRO_STOCK_BLACK));
    if (*pattern).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = _cairo_pattern_set_error(pattern, status);
    }
    return pattern;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_create_rgb(
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
) -> *mut cairo_pattern_t {
    return cairo_pattern_create_rgba(red, green, blue, 1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_create_rgba(
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
    mut alpha: libc::c_double,
) -> *mut cairo_pattern_t {
    let mut color: cairo_color_t = cairo_color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        red_short: 0,
        green_short: 0,
        blue_short: 0,
        alpha_short: 0,
    };
    red = _cairo_restrict_value(red, 0.0f64, 1.0f64);
    green = _cairo_restrict_value(green, 0.0f64, 1.0f64);
    blue = _cairo_restrict_value(blue, 0.0f64, 1.0f64);
    alpha = _cairo_restrict_value(alpha, 0.0f64, 1.0f64);
    _cairo_color_init_rgba(&mut color, red, green, blue, alpha);
    return _cairo_pattern_create_solid(&mut color);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_create_for_surface(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_pattern_t {
    let mut pattern: *mut cairo_surface_pattern_t = 0 as *mut cairo_surface_pattern_t;
    if surface.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NULL_POINTER);
        return &_cairo_pattern_nil_null_pointer as *const cairo_solid_pattern_t
            as *mut cairo_pattern_t;
    }
    if (*surface).status as u64 != 0 {
        return _cairo_pattern_create_in_error((*surface).status);
    }
    pattern = _freed_pool_get(
        &mut *freed_pattern_pool
            .as_mut_ptr()
            .offset(CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as isize),
    ) as *mut cairo_surface_pattern_t;
    if pattern.is_null() {
        pattern = (if ::std::mem::size_of::<cairo_surface_pattern_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_surface_pattern_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_surface_pattern_t;
        if pattern.is_null() {
            let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_pattern_nil.base as *const cairo_pattern_t
                as *mut cairo_pattern_t;
        }
    }
    _cairo_pattern_init_for_surface(pattern, surface);
    (*pattern).base.ref_count.ref_count = 1 as libc::c_int;
    return &mut (*pattern).base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_create_linear(
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
) -> *mut cairo_pattern_t {
    let mut pattern: *mut cairo_linear_pattern_t = 0 as *mut cairo_linear_pattern_t;
    pattern = _freed_pool_get(
        &mut *freed_pattern_pool
            .as_mut_ptr()
            .offset(CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as isize),
    ) as *mut cairo_linear_pattern_t;
    if pattern.is_null() {
        pattern = (if ::std::mem::size_of::<cairo_linear_pattern_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_linear_pattern_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_linear_pattern_t;
        if pattern.is_null() {
            let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_pattern_nil.base as *const cairo_pattern_t
                as *mut cairo_pattern_t;
        }
    }
    _cairo_pattern_init_linear(pattern, x0, y0, x1, y1);
    (*pattern).base.base.ref_count.ref_count = 1 as libc::c_int;
    return &mut (*pattern).base.base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_create_radial(
    mut cx0: libc::c_double,
    mut cy0: libc::c_double,
    mut radius0: libc::c_double,
    mut cx1: libc::c_double,
    mut cy1: libc::c_double,
    mut radius1: libc::c_double,
) -> *mut cairo_pattern_t {
    let mut pattern: *mut cairo_radial_pattern_t = 0 as *mut cairo_radial_pattern_t;
    pattern = _freed_pool_get(
        &mut *freed_pattern_pool
            .as_mut_ptr()
            .offset(CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as isize),
    ) as *mut cairo_radial_pattern_t;
    if pattern.is_null() {
        pattern = (if ::std::mem::size_of::<cairo_radial_pattern_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_radial_pattern_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_radial_pattern_t;
        if pattern.is_null() {
            let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_pattern_nil.base as *const cairo_pattern_t
                as *mut cairo_pattern_t;
        }
    }
    _cairo_pattern_init_radial(pattern, cx0, cy0, radius0, cx1, cy1, radius1);
    (*pattern).base.base.ref_count.ref_count = 1 as libc::c_int;
    return &mut (*pattern).base.base;
}
static mut mesh_path_point_i: [libc::c_int; 12] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
];
static mut mesh_path_point_j: [libc::c_int; 12] = [
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
static mut mesh_control_point_i: [libc::c_int; 4] = [
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
];
static mut mesh_control_point_j: [libc::c_int; 4] = [
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t {
    let mut pattern: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    pattern = _freed_pool_get(
        &mut *freed_pattern_pool
            .as_mut_ptr()
            .offset(CAIRO_PATTERN_TYPE_MESH as libc::c_int as isize),
    ) as *mut cairo_mesh_pattern_t;
    if pattern.is_null() {
        pattern = (if ::std::mem::size_of::<cairo_mesh_pattern_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_mesh_pattern_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_mesh_pattern_t;
        if pattern.is_null() {
            let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_pattern_nil.base as *const cairo_pattern_t
                as *mut cairo_pattern_t;
        }
    }
    _cairo_pattern_init(&mut (*pattern).base, CAIRO_PATTERN_TYPE_MESH);
    _cairo_array_init(
        &mut (*pattern).patches,
        ::std::mem::size_of::<cairo_mesh_patch_t>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh13 = (*pattern).current_patch;
    *fresh13 = 0 as *mut cairo_mesh_patch_t;
    (*pattern).base.ref_count.ref_count = 1 as libc::c_int;
    return &mut (*pattern).base;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_reference(
    mut pattern: *mut cairo_pattern_t,
) -> *mut cairo_pattern_t {
    if pattern.is_null()
        || _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return pattern;
    }
    if _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&pattern->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1070 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"cairo_pattern_t *cairo_pattern_reference(cairo_pattern_t *)\0"))
                .as_ptr(),
        );
    }
    ::std::intrinsics::atomic_xadd(
        &mut (*pattern).ref_count.ref_count,
        1 as libc::c_int,
    );
    return pattern;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_type(
    mut pattern: *mut cairo_pattern_t,
) -> cairo_pattern_type_t {
    return (*pattern).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_status(
    mut pattern: *mut cairo_pattern_t,
) -> cairo_status_t {
    return (*pattern).status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_destroy(mut pattern: *mut cairo_pattern_t) {
    let mut type_0: cairo_pattern_type_t = CAIRO_PATTERN_TYPE_SOLID;
    if pattern.is_null()
        || _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return;
    }
    if _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&pattern->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void cairo_pattern_destroy(cairo_pattern_t *)\0"))
                .as_ptr(),
        );
    }
    if !(::std::intrinsics::atomic_xsub(
        &mut (*pattern).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        return;
    }
    type_0 = (*pattern).type_0;
    _cairo_pattern_fini(pattern);
    if (type_0 as libc::c_uint)
        < (::std::mem::size_of::<[freed_pool_t; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<freed_pool_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        _freed_pool_put(
            &mut *freed_pattern_pool.as_mut_ptr().offset(type_0 as isize),
            pattern as *mut libc::c_void,
        );
    } else {
        free(pattern as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_reference_count(
    mut pattern: *mut cairo_pattern_t,
) -> libc::c_uint {
    if pattern.is_null()
        || _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    return _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_user_data(
    mut pattern: *mut cairo_pattern_t,
    mut key: *const cairo_user_data_key_t,
) -> *mut libc::c_void {
    return _cairo_user_data_array_get_data(&mut (*pattern).user_data, key);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_set_user_data(
    mut pattern: *mut cairo_pattern_t,
    mut key: *const cairo_user_data_key_t,
    mut user_data: *mut libc::c_void,
    mut destroy: cairo_destroy_func_t,
) -> cairo_status_t {
    if _cairo_atomic_int_get(&mut (*pattern).ref_count.ref_count) == -(1 as libc::c_int)
    {
        return (*pattern).status;
    }
    return _cairo_user_data_array_set_data(
        &mut (*pattern).user_data,
        key,
        user_data,
        destroy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_begin_patch(
    mut pattern: *mut cairo_pattern_t,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut current_patch: *mut cairo_mesh_patch_t = 0 as *mut cairo_mesh_patch_t;
    let mut i: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    if !((*mesh).current_patch).is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    status = _cairo_array_allocate(
        &mut (*mesh).patches,
        1 as libc::c_int as libc::c_uint,
        &mut current_patch as *mut *mut cairo_mesh_patch_t as *mut *mut libc::c_void,
    );
    if status as u64 != 0 {
        _cairo_pattern_set_error(pattern, status);
        return;
    }
    let ref mut fresh14 = (*mesh).current_patch;
    *fresh14 = current_patch;
    (*mesh).current_side = -(2 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*mesh).has_control_point[i as usize] = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*mesh).has_color[i as usize] = 0 as libc::c_int;
        i += 1;
    }
}
unsafe extern "C" fn _calc_control_point(
    mut patch: *mut cairo_mesh_patch_t,
    mut control_point: libc::c_int,
) {
    let mut p: [[*mut cairo_point_double_t; 3]; 3] = [[0
        as *mut cairo_point_double_t; 3]; 3];
    let mut cp_i: libc::c_int = 0;
    let mut cp_j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    cp_i = mesh_control_point_i[control_point as usize];
    cp_j = mesh_control_point_j[control_point as usize];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            p[i
                as usize][j
                as usize] = &mut *(*((*patch).points)
                .as_mut_ptr()
                .offset((cp_i ^ i) as isize))
                .as_mut_ptr()
                .offset((cp_j ^ j) as isize) as *mut cairo_point_double_t;
            j += 1;
        }
        i += 1;
    }
    (*p[0 as libc::c_int as usize][0 as libc::c_int as usize])
        .x = (-(4 as libc::c_int) as libc::c_double
        * (*p[1 as libc::c_int as usize][1 as libc::c_int as usize]).x
        + 6 as libc::c_int as libc::c_double
            * ((*p[1 as libc::c_int as usize][0 as libc::c_int as usize]).x
                + (*p[0 as libc::c_int as usize][1 as libc::c_int as usize]).x)
        - 2 as libc::c_int as libc::c_double
            * ((*p[1 as libc::c_int as usize][2 as libc::c_int as usize]).x
                + (*p[2 as libc::c_int as usize][1 as libc::c_int as usize]).x)
        + 3 as libc::c_int as libc::c_double
            * ((*p[2 as libc::c_int as usize][0 as libc::c_int as usize]).x
                + (*p[0 as libc::c_int as usize][2 as libc::c_int as usize]).x)
        - 1 as libc::c_int as libc::c_double
            * (*p[2 as libc::c_int as usize][2 as libc::c_int as usize]).x)
        * (1.0f64 / 9 as libc::c_int as libc::c_double);
    (*p[0 as libc::c_int as usize][0 as libc::c_int as usize])
        .y = (-(4 as libc::c_int) as libc::c_double
        * (*p[1 as libc::c_int as usize][1 as libc::c_int as usize]).y
        + 6 as libc::c_int as libc::c_double
            * ((*p[1 as libc::c_int as usize][0 as libc::c_int as usize]).y
                + (*p[0 as libc::c_int as usize][1 as libc::c_int as usize]).y)
        - 2 as libc::c_int as libc::c_double
            * ((*p[1 as libc::c_int as usize][2 as libc::c_int as usize]).y
                + (*p[2 as libc::c_int as usize][1 as libc::c_int as usize]).y)
        + 3 as libc::c_int as libc::c_double
            * ((*p[2 as libc::c_int as usize][0 as libc::c_int as usize]).y
                + (*p[0 as libc::c_int as usize][2 as libc::c_int as usize]).y)
        - 1 as libc::c_int as libc::c_double
            * (*p[2 as libc::c_int as usize][2 as libc::c_int as usize]).y)
        * (1.0f64 / 9 as libc::c_int as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_end_patch(
    mut pattern: *mut cairo_pattern_t,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    let mut current_patch: *mut cairo_mesh_patch_t = 0 as *mut cairo_mesh_patch_t;
    let mut i: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    current_patch = (*mesh).current_patch;
    if current_patch.is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    if (*mesh).current_side == -(2 as libc::c_int) {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    while (*mesh).current_side < 3 as libc::c_int {
        let mut corner_num: libc::c_int = 0;
        cairo_mesh_pattern_line_to(
            pattern,
            (*current_patch)
                .points[0 as libc::c_int as usize][0 as libc::c_int as usize]
                .x,
            (*current_patch)
                .points[0 as libc::c_int as usize][0 as libc::c_int as usize]
                .y,
        );
        corner_num = (*mesh).current_side + 1 as libc::c_int;
        if corner_num < 4 as libc::c_int && (*mesh).has_color[corner_num as usize] == 0 {
            (*current_patch)
                .colors[corner_num
                as usize] = (*current_patch).colors[0 as libc::c_int as usize];
            (*mesh).has_color[corner_num as usize] = 1 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*mesh).has_control_point[i as usize] == 0 {
            _calc_control_point(current_patch, i);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*mesh).has_color[i as usize] == 0 {
            (*current_patch)
                .colors[i as usize] = *_cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
        }
        i += 1;
    }
    let ref mut fresh15 = (*mesh).current_patch;
    *fresh15 = 0 as *mut cairo_mesh_patch_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_curve_to(
    mut pattern: *mut cairo_pattern_t,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut x2: libc::c_double,
    mut y2: libc::c_double,
    mut x3: libc::c_double,
    mut y3: libc::c_double,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    let mut current_point: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    if ((*mesh).current_patch).is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    if (*mesh).current_side == 3 as libc::c_int {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    if (*mesh).current_side == -(2 as libc::c_int) {
        cairo_mesh_pattern_move_to(pattern, x1, y1);
    }
    if (*mesh).current_side >= -(1 as libc::c_int) {} else {
        __assert_fail(
            b"mesh->current_side >= -1\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1460 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void cairo_mesh_pattern_curve_to(cairo_pattern_t *, double, double, double, double, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    if (*pattern).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"pattern->status == CAIRO_STATUS_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1461 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"void cairo_mesh_pattern_curve_to(cairo_pattern_t *, double, double, double, double, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh16 = (*mesh).current_side;
    *fresh16 += 1;
    current_point = 3 as libc::c_int * (*mesh).current_side;
    current_point += 1;
    i = mesh_path_point_i[current_point as usize];
    j = mesh_path_point_j[current_point as usize];
    (*(*mesh).current_patch).points[i as usize][j as usize].x = x1;
    (*(*mesh).current_patch).points[i as usize][j as usize].y = y1;
    current_point += 1;
    i = mesh_path_point_i[current_point as usize];
    j = mesh_path_point_j[current_point as usize];
    (*(*mesh).current_patch).points[i as usize][j as usize].x = x2;
    (*(*mesh).current_patch).points[i as usize][j as usize].y = y2;
    current_point += 1;
    if current_point < 12 as libc::c_int {
        i = mesh_path_point_i[current_point as usize];
        j = mesh_path_point_j[current_point as usize];
        (*(*mesh).current_patch).points[i as usize][j as usize].x = x3;
        (*(*mesh).current_patch).points[i as usize][j as usize].y = y3;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_line_to(
    mut pattern: *mut cairo_pattern_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    let mut last_point: cairo_point_double_t = cairo_point_double_t {
        x: 0.,
        y: 0.,
    };
    let mut last_point_idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    if ((*mesh).current_patch).is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    if (*mesh).current_side == 3 as libc::c_int {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    if (*mesh).current_side == -(2 as libc::c_int) {
        cairo_mesh_pattern_move_to(pattern, x, y);
        return;
    }
    last_point_idx = 3 as libc::c_int * ((*mesh).current_side + 1 as libc::c_int);
    i = mesh_path_point_i[last_point_idx as usize];
    j = mesh_path_point_j[last_point_idx as usize];
    last_point = (*(*mesh).current_patch).points[i as usize][j as usize];
    cairo_mesh_pattern_curve_to(
        pattern,
        (2 as libc::c_int as libc::c_double * last_point.x + x)
            * (1.0f64 / 3 as libc::c_int as libc::c_double),
        (2 as libc::c_int as libc::c_double * last_point.y + y)
            * (1.0f64 / 3 as libc::c_int as libc::c_double),
        (last_point.x + 2 as libc::c_int as libc::c_double * x)
            * (1.0f64 / 3 as libc::c_int as libc::c_double),
        (last_point.y + 2 as libc::c_int as libc::c_double * y)
            * (1.0f64 / 3 as libc::c_int as libc::c_double),
        x,
        y,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_move_to(
    mut pattern: *mut cairo_pattern_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    if ((*mesh).current_patch).is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    if (*mesh).current_side >= 0 as libc::c_int {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    (*mesh).current_side = -(1 as libc::c_int);
    (*(*mesh).current_patch)
        .points[0 as libc::c_int as usize][0 as libc::c_int as usize]
        .x = x;
    (*(*mesh).current_patch)
        .points[0 as libc::c_int as usize][0 as libc::c_int as usize]
        .y = y;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_set_control_point(
    mut pattern: *mut cairo_pattern_t,
    mut point_num: libc::c_uint,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    if point_num > 3 as libc::c_int as libc::c_uint {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_INDEX);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    if ((*mesh).current_patch).is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    i = mesh_control_point_i[point_num as usize];
    j = mesh_control_point_j[point_num as usize];
    (*(*mesh).current_patch).points[i as usize][j as usize].x = x;
    (*(*mesh).current_patch).points[i as usize][j as usize].y = y;
    (*mesh).has_control_point[point_num as usize] = 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_pattern_gradient_grow(
    mut pattern: *mut cairo_gradient_pattern_t,
) -> cairo_status_t {
    let mut new_stops: *mut cairo_gradient_stop_t = 0 as *mut cairo_gradient_stop_t;
    let mut old_size: libc::c_int = (*pattern).stops_size as libc::c_int;
    let mut embedded_size: libc::c_int = (::std::mem::size_of::<
        [cairo_gradient_stop_t; 2],
    >() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong)
        as libc::c_int;
    let mut new_size: libc::c_int = 2 as libc::c_int
        * (if old_size > 4 as libc::c_int { old_size } else { 4 as libc::c_int });
    if old_size < embedded_size {
        let ref mut fresh17 = (*pattern).stops;
        *fresh17 = ((*pattern).stops_embedded).as_mut_ptr();
        (*pattern).stops_size = embedded_size as libc::c_uint;
        return CAIRO_STATUS_SUCCESS;
    }
    if (*pattern).n_stops <= (*pattern).stops_size {} else {
        __assert_fail(
            b"pattern->n_stops <= pattern->stops_size\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1688 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"cairo_status_t _cairo_pattern_gradient_grow(cairo_gradient_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*pattern).stops == ((*pattern).stops_embedded).as_mut_ptr() {
        new_stops = _cairo_malloc_ab(
            new_size as size_t,
            ::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong,
        ) as *mut cairo_gradient_stop_t;
        if !new_stops.is_null() {
            memcpy(
                new_stops as *mut libc::c_void,
                (*pattern).stops as *const libc::c_void,
                (old_size as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong,
                    ),
            );
        }
    } else {
        new_stops = _cairo_realloc_ab(
            (*pattern).stops as *mut libc::c_void,
            new_size as size_t,
            ::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong,
        ) as *mut cairo_gradient_stop_t;
    }
    if new_stops.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh18 = (*pattern).stops;
    *fresh18 = new_stops;
    (*pattern).stops_size = new_size as libc::c_uint;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_mesh_pattern_set_corner_color(
    mut mesh: *mut cairo_mesh_pattern_t,
    mut corner_num: libc::c_uint,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
    mut alpha: libc::c_double,
) {
    let mut color: *mut cairo_color_t = 0 as *mut cairo_color_t;
    if !((*mesh).current_patch).is_null() {} else {
        __assert_fail(
            b"mesh->current_patch\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1717 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"void _cairo_mesh_pattern_set_corner_color(cairo_mesh_pattern_t *, unsigned int, double, double, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    if corner_num <= 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"corner_num <= 3\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            1718 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"void _cairo_mesh_pattern_set_corner_color(cairo_mesh_pattern_t *, unsigned int, double, double, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    color = &mut *((*(*mesh).current_patch).colors)
        .as_mut_ptr()
        .offset(corner_num as isize) as *mut cairo_color_t;
    (*color).red = red;
    (*color).green = green;
    (*color).blue = blue;
    (*color).alpha = alpha;
    (*color).red_short = _cairo_color_double_to_short(red);
    (*color).green_short = _cairo_color_double_to_short(green);
    (*color).blue_short = _cairo_color_double_to_short(blue);
    (*color).alpha_short = _cairo_color_double_to_short(alpha);
    (*mesh).has_color[corner_num as usize] = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_set_corner_color_rgb(
    mut pattern: *mut cairo_pattern_t,
    mut corner_num: libc::c_uint,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
) {
    cairo_mesh_pattern_set_corner_color_rgba(
        pattern,
        corner_num,
        red,
        green,
        blue,
        1.0f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_set_corner_color_rgba(
    mut pattern: *mut cairo_pattern_t,
    mut corner_num: libc::c_uint,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
    mut alpha: libc::c_double,
) {
    let mut mesh: *mut cairo_mesh_pattern_t = 0 as *mut cairo_mesh_pattern_t;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    if corner_num > 3 as libc::c_int as libc::c_uint {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_INDEX);
        return;
    }
    mesh = pattern as *mut cairo_mesh_pattern_t;
    if ((*mesh).current_patch).is_null() {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_INVALID_MESH_CONSTRUCTION);
        return;
    }
    red = _cairo_restrict_value(red, 0.0f64, 1.0f64);
    green = _cairo_restrict_value(green, 0.0f64, 1.0f64);
    blue = _cairo_restrict_value(blue, 0.0f64, 1.0f64);
    alpha = _cairo_restrict_value(alpha, 0.0f64, 1.0f64);
    _cairo_mesh_pattern_set_corner_color(mesh, corner_num, red, green, blue, alpha);
}
unsafe extern "C" fn _cairo_pattern_add_color_stop(
    mut pattern: *mut cairo_gradient_pattern_t,
    mut offset: libc::c_double,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
    mut alpha: libc::c_double,
) {
    let mut stops: *mut cairo_gradient_stop_t = 0 as *mut cairo_gradient_stop_t;
    let mut i: libc::c_uint = 0;
    if (*pattern).n_stops >= (*pattern).stops_size {
        let mut status: cairo_status_t = _cairo_pattern_gradient_grow(pattern);
        if status as u64 != 0 {
            status = _cairo_pattern_set_error(&mut (*pattern).base, status);
            return;
        }
    }
    stops = (*pattern).stops;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*pattern).n_stops {
        if offset < (*stops.offset(i as isize)).offset {
            memmove(
                &mut *stops
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    as *mut cairo_gradient_stop_t as *mut libc::c_void,
                &mut *stops.offset(i as isize) as *mut cairo_gradient_stop_t
                    as *const libc::c_void,
                (::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong)
                    .wrapping_mul(((*pattern).n_stops).wrapping_sub(i) as libc::c_ulong),
            );
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    (*stops.offset(i as isize)).offset = offset;
    (*stops.offset(i as isize)).color.red = red;
    (*stops.offset(i as isize)).color.green = green;
    (*stops.offset(i as isize)).color.blue = blue;
    (*stops.offset(i as isize)).color.alpha = alpha;
    (*stops.offset(i as isize)).color.red_short = _cairo_color_double_to_short(red);
    (*stops.offset(i as isize)).color.green_short = _cairo_color_double_to_short(green);
    (*stops.offset(i as isize)).color.blue_short = _cairo_color_double_to_short(blue);
    (*stops.offset(i as isize)).color.alpha_short = _cairo_color_double_to_short(alpha);
    let ref mut fresh19 = (*pattern).n_stops;
    *fresh19 = (*fresh19).wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_add_color_stop_rgb(
    mut pattern: *mut cairo_pattern_t,
    mut offset: libc::c_double,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
) {
    cairo_pattern_add_color_stop_rgba(pattern, offset, red, green, blue, 1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_add_color_stop_rgba(
    mut pattern: *mut cairo_pattern_t,
    mut offset: libc::c_double,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
    mut alpha: libc::c_double,
) {
    if (*pattern).status as u64 != 0 {
        return;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        && (*pattern).type_0 as libc::c_uint
            != CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {
        _cairo_pattern_set_error(pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
        return;
    }
    offset = _cairo_restrict_value(offset, 0.0f64, 1.0f64);
    red = _cairo_restrict_value(red, 0.0f64, 1.0f64);
    green = _cairo_restrict_value(green, 0.0f64, 1.0f64);
    blue = _cairo_restrict_value(blue, 0.0f64, 1.0f64);
    alpha = _cairo_restrict_value(alpha, 0.0f64, 1.0f64);
    _cairo_pattern_add_color_stop(
        pattern as *mut cairo_gradient_pattern_t,
        offset,
        red,
        green,
        blue,
        alpha,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_set_matrix(
    mut pattern: *mut cairo_pattern_t,
    mut matrix: *const cairo_matrix_t,
) {
    let mut inverse: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pattern).status as u64 != 0 {
        return;
    }
    if memcmp(
        &mut (*pattern).matrix as *mut cairo_matrix_t as *const libc::c_void,
        matrix as *const libc::c_void,
        ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return;
    }
    (*pattern).matrix = *matrix;
    _cairo_pattern_notify_observers(
        pattern,
        CAIRO_PATTERN_NOTIFY_MATRIX as libc::c_int as libc::c_uint,
    );
    inverse = *matrix;
    status = cairo_matrix_invert(&mut inverse);
    if status as u64 != 0 {
        status = _cairo_pattern_set_error(pattern, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_matrix(
    mut pattern: *mut cairo_pattern_t,
    mut matrix: *mut cairo_matrix_t,
) {
    *matrix = (*pattern).matrix;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_set_filter(
    mut pattern: *mut cairo_pattern_t,
    mut filter: cairo_filter_t,
) {
    if (*pattern).status as u64 != 0 {
        return;
    }
    (*pattern).filter = filter;
    _cairo_pattern_notify_observers(
        pattern,
        CAIRO_PATTERN_NOTIFY_FILTER as libc::c_int as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_filter(
    mut pattern: *mut cairo_pattern_t,
) -> cairo_filter_t {
    return (*pattern).filter;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_set_extend(
    mut pattern: *mut cairo_pattern_t,
    mut extend: cairo_extend_t,
) {
    if (*pattern).status as u64 != 0 {
        return;
    }
    (*pattern).extend = extend;
    _cairo_pattern_notify_observers(
        pattern,
        CAIRO_PATTERN_NOTIFY_EXTEND as libc::c_int as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_extend(
    mut pattern: *mut cairo_pattern_t,
) -> cairo_extend_t {
    return (*pattern).extend;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_pretransform(
    mut pattern: *mut cairo_pattern_t,
    mut ctm: *const cairo_matrix_t,
) {
    if (*pattern).status as u64 != 0 {
        return;
    }
    cairo_matrix_multiply(&mut (*pattern).matrix, &mut (*pattern).matrix, ctm);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_transform(
    mut pattern: *mut cairo_pattern_t,
    mut ctm_inverse: *const cairo_matrix_t,
) {
    if (*pattern).status as u64 != 0 {
        return;
    }
    cairo_matrix_multiply(&mut (*pattern).matrix, ctm_inverse, &mut (*pattern).matrix);
}
unsafe extern "C" fn _linear_pattern_is_degenerate(
    mut linear: *const cairo_linear_pattern_t,
) -> cairo_bool_t {
    return (fabs((*linear).pd1.x - (*linear).pd2.x) < 2.2204460492503131e-16f64
        && fabs((*linear).pd1.y - (*linear).pd2.y) < 2.2204460492503131e-16f64)
        as libc::c_int;
}
unsafe extern "C" fn _radial_pattern_is_degenerate(
    mut radial: *const cairo_radial_pattern_t,
) -> cairo_bool_t {
    return (fabs((*radial).cd1.radius - (*radial).cd2.radius) < 2.2204460492503131e-16f64
        && ((if (*radial).cd1.radius < (*radial).cd2.radius {
            (*radial).cd1.radius
        } else {
            (*radial).cd2.radius
        }) < 2.2204460492503131e-16f64
            || (if fabs((*radial).cd1.center.x - (*radial).cd2.center.x)
                > fabs((*radial).cd1.center.y - (*radial).cd2.center.y)
            {
                fabs((*radial).cd1.center.x - (*radial).cd2.center.x)
            } else {
                fabs((*radial).cd1.center.y - (*radial).cd2.center.y)
            }) < 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64))
        as libc::c_int;
}
unsafe extern "C" fn _cairo_linear_pattern_box_to_parameter(
    mut linear: *const cairo_linear_pattern_t,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut range: *mut libc::c_double,
) {
    let mut t0: libc::c_double = 0.;
    let mut tdx: libc::c_double = 0.;
    let mut tdy: libc::c_double = 0.;
    let mut p1x: libc::c_double = 0.;
    let mut p1y: libc::c_double = 0.;
    let mut pdx: libc::c_double = 0.;
    let mut pdy: libc::c_double = 0.;
    let mut invsqnorm: libc::c_double = 0.;
    if _linear_pattern_is_degenerate(linear) == 0 {} else {
        __assert_fail(
            b"! _linear_pattern_is_degenerate (linear)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2199 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void _cairo_linear_pattern_box_to_parameter(const cairo_linear_pattern_t *, double, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    }
    p1x = (*linear).pd1.x;
    p1y = (*linear).pd1.y;
    pdx = (*linear).pd2.x - p1x;
    pdy = (*linear).pd2.y - p1y;
    invsqnorm = 1.0f64 / (pdx * pdx + pdy * pdy);
    pdx *= invsqnorm;
    pdy *= invsqnorm;
    t0 = (x0 - p1x) * pdx + (y0 - p1y) * pdy;
    tdx = (x1 - x0) * pdx;
    tdy = (y1 - y0) * pdy;
    let ref mut fresh20 = *range.offset(1 as libc::c_int as isize);
    *fresh20 = t0;
    *range.offset(0 as libc::c_int as isize) = *fresh20;
    if tdx < 0 as libc::c_int as libc::c_double {
        *range.offset(0 as libc::c_int as isize) += tdx;
    } else {
        *range.offset(1 as libc::c_int as isize) += tdx;
    }
    if tdy < 0 as libc::c_int as libc::c_double {
        *range.offset(0 as libc::c_int as isize) += tdy;
    } else {
        *range.offset(1 as libc::c_int as isize) += tdy;
    };
}
unsafe extern "C" fn _extend_range(
    mut range: *mut libc::c_double,
    mut value: libc::c_double,
    mut valid: cairo_bool_t,
) -> cairo_bool_t {
    if valid == 0 {
        let ref mut fresh21 = *range.offset(1 as libc::c_int as isize);
        *fresh21 = value;
        *range.offset(0 as libc::c_int as isize) = *fresh21;
    } else if value < *range.offset(0 as libc::c_int as isize) {
        *range.offset(0 as libc::c_int as isize) = value;
    } else if value > *range.offset(1 as libc::c_int as isize) {
        *range.offset(1 as libc::c_int as isize) = value;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_radial_pattern_focus_is_inside(
    mut radial: *const cairo_radial_pattern_t,
) -> cairo_bool_t {
    let mut cx: libc::c_double = 0.;
    let mut cy: libc::c_double = 0.;
    let mut cr: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dr: libc::c_double = 0.;
    cx = (*radial).cd1.center.x;
    cy = (*radial).cd1.center.y;
    cr = (*radial).cd1.radius;
    dx = (*radial).cd2.center.x - cx;
    dy = (*radial).cd2.center.y - cy;
    dr = (*radial).cd2.radius - cr;
    return (dx * dx + dy * dy < dr * dr) as libc::c_int;
}
unsafe extern "C" fn _cairo_radial_pattern_box_to_parameter(
    mut radial: *const cairo_radial_pattern_t,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut tolerance: libc::c_double,
    mut range: *mut libc::c_double,
) {
    let mut cx: libc::c_double = 0.;
    let mut cy: libc::c_double = 0.;
    let mut cr: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dr: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut x_focus: libc::c_double = 0.;
    let mut y_focus: libc::c_double = 0.;
    let mut mindr: libc::c_double = 0.;
    let mut minx: libc::c_double = 0.;
    let mut miny: libc::c_double = 0.;
    let mut maxx: libc::c_double = 0.;
    let mut maxy: libc::c_double = 0.;
    let mut valid: cairo_bool_t = 0;
    if _radial_pattern_is_degenerate(radial) == 0 {} else {
        __assert_fail(
            b"! _radial_pattern_is_degenerate (radial)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2302 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"void _cairo_radial_pattern_box_to_parameter(const cairo_radial_pattern_t *, double, double, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    }
    if x0 < x1 {} else {
        __assert_fail(
            b"x0 < x1\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2303 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"void _cairo_radial_pattern_box_to_parameter(const cairo_radial_pattern_t *, double, double, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    }
    if y0 < y1 {} else {
        __assert_fail(
            b"y0 < y1\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"void _cairo_radial_pattern_box_to_parameter(const cairo_radial_pattern_t *, double, double, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    }
    tolerance = if tolerance > 2.2204460492503131e-16f64 {
        tolerance
    } else {
        2.2204460492503131e-16f64
    };
    let ref mut fresh22 = *range.offset(1 as libc::c_int as isize);
    *fresh22 = 0 as libc::c_int as libc::c_double;
    *range.offset(0 as libc::c_int as isize) = *fresh22;
    valid = 0 as libc::c_int;
    y_focus = 0 as libc::c_int as libc::c_double;
    x_focus = y_focus;
    cx = (*radial).cd1.center.x;
    cy = (*radial).cd1.center.y;
    cr = (*radial).cd1.radius;
    dx = (*radial).cd2.center.x - cx;
    dy = (*radial).cd2.center.y - cy;
    dr = (*radial).cd2.radius - cr;
    x0 -= cx;
    y0 -= cy;
    x1 -= cx;
    y1 -= cy;
    x0 -= 2.2204460492503131e-16f64;
    y0 -= 2.2204460492503131e-16f64;
    x1 += 2.2204460492503131e-16f64;
    y1 += 2.2204460492503131e-16f64;
    minx = x0 - 2.2204460492503131e-16f64;
    miny = y0 - 2.2204460492503131e-16f64;
    maxx = x1 + 2.2204460492503131e-16f64;
    maxy = y1 + 2.2204460492503131e-16f64;
    mindr = -(cr + 2.2204460492503131e-16f64);
    if fabs(dr) >= 2.2204460492503131e-16f64 {
        let mut t_focus: libc::c_double = 0.;
        t_focus = -cr / dr;
        x_focus = t_focus * dx;
        y_focus = t_focus * dy;
        if minx <= x_focus && x_focus <= maxx && miny <= y_focus && y_focus <= maxy {
            valid = _extend_range(range, t_focus, valid);
        }
    }
    if fabs(dx + dr) >= 2.2204460492503131e-16f64 {
        let mut t_edge: libc::c_double = 0.;
        let mut v: libc::c_double = 0.;
        t_edge = (x0 - cr) / (dx + dr);
        v = t_edge * dy;
        if t_edge * dr >= mindr && miny <= v && v <= maxy {
            valid = _extend_range(range, t_edge, valid);
        }
    }
    if fabs(dx - dr) >= 2.2204460492503131e-16f64 {
        let mut t_edge_0: libc::c_double = 0.;
        let mut v_0: libc::c_double = 0.;
        t_edge_0 = (x1 + cr) / (dx - dr);
        v_0 = t_edge_0 * dy;
        if t_edge_0 * dr >= mindr && miny <= v_0 && v_0 <= maxy {
            valid = _extend_range(range, t_edge_0, valid);
        }
    }
    if fabs(dy + dr) >= 2.2204460492503131e-16f64 {
        let mut t_edge_1: libc::c_double = 0.;
        let mut v_1: libc::c_double = 0.;
        t_edge_1 = (y0 - cr) / (dy + dr);
        v_1 = t_edge_1 * dx;
        if t_edge_1 * dr >= mindr && minx <= v_1 && v_1 <= maxx {
            valid = _extend_range(range, t_edge_1, valid);
        }
    }
    if fabs(dy - dr) >= 2.2204460492503131e-16f64 {
        let mut t_edge_2: libc::c_double = 0.;
        let mut v_2: libc::c_double = 0.;
        t_edge_2 = (y1 + cr) / (dy - dr);
        v_2 = t_edge_2 * dx;
        if t_edge_2 * dr >= mindr && minx <= v_2 && v_2 <= maxx {
            valid = _extend_range(range, t_edge_2, valid);
        }
    }
    a = dx * dx + dy * dy - dr * dr;
    if fabs(a) < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64 {
        let mut b: libc::c_double = 0.;
        let mut maxd2: libc::c_double = 0.;
        if fabs(dr) >= 2.2204460492503131e-16f64 {} else {
            __assert_fail(
                b"fabs (dr) >= DBL_EPSILON\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                2479 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 126],
                    &[libc::c_char; 126],
                >(
                    b"void _cairo_radial_pattern_box_to_parameter(const cairo_radial_pattern_t *, double, double, double, double, double, double *)\0",
                ))
                    .as_ptr(),
            );
        }
        maxd2 = 0 as libc::c_int as libc::c_double;
        if fabs(dx) >= 2.2204460492503131e-16f64 {
            let mut v_3: libc::c_double = 0.;
            v_3 = -(y0 * dy + cr * dr) / dx;
            if minx <= v_3 && v_3 <= maxx {
                let mut u: libc::c_double = 0.;
                let mut d2: libc::c_double = 0.;
                u = y0 - y_focus;
                v_3 -= x_focus;
                d2 = u * u + v_3 * v_3;
                if maxd2 < d2 {
                    maxd2 = d2;
                }
            }
        }
        if fabs(dx) >= 2.2204460492503131e-16f64 {
            let mut v_4: libc::c_double = 0.;
            v_4 = -(y1 * dy + cr * dr) / dx;
            if minx <= v_4 && v_4 <= maxx {
                let mut u_0: libc::c_double = 0.;
                let mut d2_0: libc::c_double = 0.;
                u_0 = y1 - y_focus;
                v_4 -= x_focus;
                d2_0 = u_0 * u_0 + v_4 * v_4;
                if maxd2 < d2_0 {
                    maxd2 = d2_0;
                }
            }
        }
        if fabs(dy) >= 2.2204460492503131e-16f64 {
            let mut v_5: libc::c_double = 0.;
            v_5 = -(x0 * dx + cr * dr) / dy;
            if miny <= v_5 && v_5 <= maxy {
                let mut u_1: libc::c_double = 0.;
                let mut d2_1: libc::c_double = 0.;
                u_1 = x0 - x_focus;
                v_5 -= y_focus;
                d2_1 = u_1 * u_1 + v_5 * v_5;
                if maxd2 < d2_1 {
                    maxd2 = d2_1;
                }
            }
        }
        if fabs(dy) >= 2.2204460492503131e-16f64 {
            let mut v_6: libc::c_double = 0.;
            v_6 = -(x1 * dx + cr * dr) / dy;
            if miny <= v_6 && v_6 <= maxy {
                let mut u_2: libc::c_double = 0.;
                let mut d2_2: libc::c_double = 0.;
                u_2 = x1 - x_focus;
                v_6 -= y_focus;
                d2_2 = u_2 * u_2 + v_6 * v_6;
                if maxd2 < d2_2 {
                    maxd2 = d2_2;
                }
            }
        }
        if maxd2 > 0 as libc::c_int as libc::c_double {
            let mut t_limit: libc::c_double = maxd2 + tolerance * tolerance
                - 2 as libc::c_int as libc::c_double * tolerance * cr;
            t_limit /= 2 as libc::c_int as libc::c_double * tolerance * dr;
            valid = _extend_range(range, t_limit, valid);
        }
        b = x0 * dx + y0 * dy + cr * dr;
        if fabs(b) >= 2.2204460492503131e-16f64 {
            let mut t_corner: libc::c_double = 0.;
            let mut x2: libc::c_double = x0 * x0;
            let mut y2: libc::c_double = y0 * y0;
            let mut cr2: libc::c_double = cr * cr;
            let mut c: libc::c_double = x2 + y2 - cr2;
            t_corner = 0.5f64 * c / b;
            if t_corner * dr >= mindr {
                valid = _extend_range(range, t_corner, valid);
            }
        }
        b = x0 * dx + y1 * dy + cr * dr;
        if fabs(b) >= 2.2204460492503131e-16f64 {
            let mut t_corner_0: libc::c_double = 0.;
            let mut x2_0: libc::c_double = x0 * x0;
            let mut y2_0: libc::c_double = y1 * y1;
            let mut cr2_0: libc::c_double = cr * cr;
            let mut c_0: libc::c_double = x2_0 + y2_0 - cr2_0;
            t_corner_0 = 0.5f64 * c_0 / b;
            if t_corner_0 * dr >= mindr {
                valid = _extend_range(range, t_corner_0, valid);
            }
        }
        b = x1 * dx + y0 * dy + cr * dr;
        if fabs(b) >= 2.2204460492503131e-16f64 {
            let mut t_corner_1: libc::c_double = 0.;
            let mut x2_1: libc::c_double = x1 * x1;
            let mut y2_1: libc::c_double = y0 * y0;
            let mut cr2_1: libc::c_double = cr * cr;
            let mut c_1: libc::c_double = x2_1 + y2_1 - cr2_1;
            t_corner_1 = 0.5f64 * c_1 / b;
            if t_corner_1 * dr >= mindr {
                valid = _extend_range(range, t_corner_1, valid);
            }
        }
        b = x1 * dx + y1 * dy + cr * dr;
        if fabs(b) >= 2.2204460492503131e-16f64 {
            let mut t_corner_2: libc::c_double = 0.;
            let mut x2_2: libc::c_double = x1 * x1;
            let mut y2_2: libc::c_double = y1 * y1;
            let mut cr2_2: libc::c_double = cr * cr;
            let mut c_2: libc::c_double = x2_2 + y2_2 - cr2_2;
            t_corner_2 = 0.5f64 * c_2 / b;
            if t_corner_2 * dr >= mindr {
                valid = _extend_range(range, t_corner_2, valid);
            }
        }
    } else {
        let mut inva: libc::c_double = 0.;
        let mut b_0: libc::c_double = 0.;
        let mut c_3: libc::c_double = 0.;
        let mut d: libc::c_double = 0.;
        inva = 1 as libc::c_int as libc::c_double / a;
        b_0 = x0 * dx + y0 * dy + cr * dr;
        c_3 = x0 * x0 + y0 * y0 - cr * cr;
        d = b_0 * b_0 - a * c_3;
        if d >= 0 as libc::c_int as libc::c_double {
            let mut t_corner_3: libc::c_double = 0.;
            d = sqrt(d);
            t_corner_3 = (b_0 + d) * inva;
            if t_corner_3 * dr >= mindr {
                valid = _extend_range(range, t_corner_3, valid);
            }
            t_corner_3 = (b_0 - d) * inva;
            if t_corner_3 * dr >= mindr {
                valid = _extend_range(range, t_corner_3, valid);
            }
        }
        b_0 = x0 * dx + y1 * dy + cr * dr;
        c_3 = x0 * x0 + y1 * y1 - cr * cr;
        d = b_0 * b_0 - a * c_3;
        if d >= 0 as libc::c_int as libc::c_double {
            let mut t_corner_4: libc::c_double = 0.;
            d = sqrt(d);
            t_corner_4 = (b_0 + d) * inva;
            if t_corner_4 * dr >= mindr {
                valid = _extend_range(range, t_corner_4, valid);
            }
            t_corner_4 = (b_0 - d) * inva;
            if t_corner_4 * dr >= mindr {
                valid = _extend_range(range, t_corner_4, valid);
            }
        }
        b_0 = x1 * dx + y0 * dy + cr * dr;
        c_3 = x1 * x1 + y0 * y0 - cr * cr;
        d = b_0 * b_0 - a * c_3;
        if d >= 0 as libc::c_int as libc::c_double {
            let mut t_corner_5: libc::c_double = 0.;
            d = sqrt(d);
            t_corner_5 = (b_0 + d) * inva;
            if t_corner_5 * dr >= mindr {
                valid = _extend_range(range, t_corner_5, valid);
            }
            t_corner_5 = (b_0 - d) * inva;
            if t_corner_5 * dr >= mindr {
                valid = _extend_range(range, t_corner_5, valid);
            }
        }
        b_0 = x1 * dx + y1 * dy + cr * dr;
        c_3 = x1 * x1 + y1 * y1 - cr * cr;
        d = b_0 * b_0 - a * c_3;
        if d >= 0 as libc::c_int as libc::c_double {
            let mut t_corner_6: libc::c_double = 0.;
            d = sqrt(d);
            t_corner_6 = (b_0 + d) * inva;
            if t_corner_6 * dr >= mindr {
                valid = _extend_range(range, t_corner_6, valid);
            }
            t_corner_6 = (b_0 - d) * inva;
            if t_corner_6 * dr >= mindr {
                valid = _extend_range(range, t_corner_6, valid);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_gradient_pattern_box_to_parameter(
    mut gradient: *const cairo_gradient_pattern_t,
    mut x0: libc::c_double,
    mut y0: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut tolerance: libc::c_double,
    mut out_range: *mut libc::c_double,
) {
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*gradient).base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.type == CAIRO_PATTERN_TYPE_LINEAR || gradient->base.type == CAIRO_PATTERN_TYPE_RADIAL\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2657 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 130],
                &[libc::c_char; 130],
            >(
                b"void _cairo_gradient_pattern_box_to_parameter(const cairo_gradient_pattern_t *, double, double, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        _cairo_linear_pattern_box_to_parameter(
            gradient as *mut cairo_linear_pattern_t,
            x0,
            y0,
            x1,
            y1,
            out_range,
        );
    } else {
        _cairo_radial_pattern_box_to_parameter(
            gradient as *mut cairo_radial_pattern_t,
            x0,
            y0,
            x1,
            y1,
            tolerance,
            out_range,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_gradient_pattern_interpolate(
    mut gradient: *const cairo_gradient_pattern_t,
    mut t: libc::c_double,
    mut out_circle: *mut cairo_circle_double_t,
) {
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*gradient).base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.type == CAIRO_PATTERN_TYPE_LINEAR || gradient->base.type == CAIRO_PATTERN_TYPE_RADIAL\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2681 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 108],
                &[libc::c_char; 108],
            >(
                b"void _cairo_gradient_pattern_interpolate(const cairo_gradient_pattern_t *, double, cairo_circle_double_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        let mut linear: *mut cairo_linear_pattern_t = gradient
            as *mut cairo_linear_pattern_t;
        (*out_circle)
            .center
            .x = (*linear).pd1.x * (1 as libc::c_int as libc::c_double - t)
            + (*linear).pd2.x * t;
        (*out_circle)
            .center
            .y = (*linear).pd1.y * (1 as libc::c_int as libc::c_double - t)
            + (*linear).pd2.y * t;
        (*out_circle).radius = 0 as libc::c_int as libc::c_double;
    } else {
        let mut radial: *mut cairo_radial_pattern_t = gradient
            as *mut cairo_radial_pattern_t;
        (*out_circle)
            .center
            .x = (*radial).cd1.center.x * (1 as libc::c_int as libc::c_double - t)
            + (*radial).cd2.center.x * t;
        (*out_circle)
            .center
            .y = (*radial).cd1.center.y * (1 as libc::c_int as libc::c_double - t)
            + (*radial).cd2.center.y * t;
        (*out_circle)
            .radius = (*radial).cd1.radius * (1 as libc::c_int as libc::c_double - t)
            + (*radial).cd2.radius * t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_gradient_pattern_fit_to_range(
    mut gradient: *const cairo_gradient_pattern_t,
    mut max_value: libc::c_double,
    mut out_matrix: *mut cairo_matrix_t,
    mut out_circle: *mut cairo_circle_double_t,
) {
    let mut dim: libc::c_double = 0.;
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*gradient).base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.type == CAIRO_PATTERN_TYPE_LINEAR || gradient->base.type == CAIRO_PATTERN_TYPE_RADIAL\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2721 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"void _cairo_gradient_pattern_fit_to_range(const cairo_gradient_pattern_t *, double, cairo_matrix_t *, cairo_circle_double_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        let mut linear: *mut cairo_linear_pattern_t = gradient
            as *mut cairo_linear_pattern_t;
        (*out_circle.offset(0 as libc::c_int as isize)).center = (*linear).pd1;
        (*out_circle.offset(0 as libc::c_int as isize))
            .radius = 0 as libc::c_int as libc::c_double;
        (*out_circle.offset(1 as libc::c_int as isize)).center = (*linear).pd2;
        (*out_circle.offset(1 as libc::c_int as isize))
            .radius = 0 as libc::c_int as libc::c_double;
        dim = fabs((*linear).pd1.x);
        dim = if dim > fabs((*linear).pd1.y) { dim } else { fabs((*linear).pd1.y) };
        dim = if dim > fabs((*linear).pd2.x) { dim } else { fabs((*linear).pd2.x) };
        dim = if dim > fabs((*linear).pd2.y) { dim } else { fabs((*linear).pd2.y) };
        dim = if dim > fabs((*linear).pd1.x - (*linear).pd2.x) {
            dim
        } else {
            fabs((*linear).pd1.x - (*linear).pd2.x)
        };
        dim = if dim > fabs((*linear).pd1.y - (*linear).pd2.y) {
            dim
        } else {
            fabs((*linear).pd1.y - (*linear).pd2.y)
        };
    } else {
        let mut radial: *mut cairo_radial_pattern_t = gradient
            as *mut cairo_radial_pattern_t;
        *out_circle.offset(0 as libc::c_int as isize) = (*radial).cd1;
        *out_circle.offset(1 as libc::c_int as isize) = (*radial).cd2;
        dim = fabs((*radial).cd1.center.x);
        dim = if dim > fabs((*radial).cd1.center.y) {
            dim
        } else {
            fabs((*radial).cd1.center.y)
        };
        dim = if dim > fabs((*radial).cd1.radius) {
            dim
        } else {
            fabs((*radial).cd1.radius)
        };
        dim = if dim > fabs((*radial).cd2.center.x) {
            dim
        } else {
            fabs((*radial).cd2.center.x)
        };
        dim = if dim > fabs((*radial).cd2.center.y) {
            dim
        } else {
            fabs((*radial).cd2.center.y)
        };
        dim = if dim > fabs((*radial).cd2.radius) {
            dim
        } else {
            fabs((*radial).cd2.radius)
        };
        dim = if dim > fabs((*radial).cd1.center.x - (*radial).cd2.center.x) {
            dim
        } else {
            fabs((*radial).cd1.center.x - (*radial).cd2.center.x)
        };
        dim = if dim > fabs((*radial).cd1.center.y - (*radial).cd2.center.y) {
            dim
        } else {
            fabs((*radial).cd1.center.y - (*radial).cd2.center.y)
        };
        dim = if dim > fabs((*radial).cd1.radius - (*radial).cd2.radius) {
            dim
        } else {
            fabs((*radial).cd1.radius - (*radial).cd2.radius)
        };
    }
    if dim > max_value {
        let mut scale: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        dim = max_value / dim;
        (*out_circle.offset(0 as libc::c_int as isize)).center.x *= dim;
        (*out_circle.offset(0 as libc::c_int as isize)).center.y *= dim;
        (*out_circle.offset(0 as libc::c_int as isize)).radius *= dim;
        (*out_circle.offset(1 as libc::c_int as isize)).center.x *= dim;
        (*out_circle.offset(1 as libc::c_int as isize)).center.y *= dim;
        (*out_circle.offset(1 as libc::c_int as isize)).radius *= dim;
        cairo_matrix_init_scale(&mut scale, dim, dim);
        cairo_matrix_multiply(out_matrix, &(*gradient).base.matrix, &mut scale);
    } else {
        *out_matrix = (*gradient).base.matrix;
    };
}
unsafe extern "C" fn _gradient_is_clear(
    mut gradient: *const cairo_gradient_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut i: libc::c_uint = 0;
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*gradient).base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.type == CAIRO_PATTERN_TYPE_LINEAR || gradient->base.type == CAIRO_PATTERN_TYPE_RADIAL\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2780 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_bool_t _gradient_is_clear(const cairo_gradient_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).n_stops == 0 as libc::c_int as libc::c_uint
        || (*gradient).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            && (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset
                == (*((*gradient).stops)
                    .offset(
                        ((*gradient).n_stops)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ))
                    .offset
    {
        return 1 as libc::c_int;
    }
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {
        if _radial_pattern_is_degenerate(gradient as *mut cairo_radial_pattern_t) != 0 {
            return 1 as libc::c_int;
        }
    } else if (*gradient).base.extend as libc::c_uint
        == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        if _linear_pattern_is_degenerate(gradient as *mut cairo_linear_pattern_t) != 0 {
            return 1 as libc::c_int;
        }
    }
    if !extents.is_null()
        && ((*gradient).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            || (*gradient).base.type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint)
    {
        let mut t: [libc::c_double; 2] = [0.; 2];
        _cairo_gradient_pattern_box_to_parameter(
            gradient,
            (*extents).x as libc::c_double,
            (*extents).y as libc::c_double,
            ((*extents).x + (*extents).width) as libc::c_double,
            ((*extents).y + (*extents).height) as libc::c_double,
            2.2204460492503131e-16f64,
            t.as_mut_ptr(),
        );
        if (*gradient).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            && (t[0 as libc::c_int as usize]
                >= (*((*gradient).stops)
                    .offset(
                        ((*gradient).n_stops)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ))
                    .offset
                || t[1 as libc::c_int as usize]
                    <= (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset)
        {
            return 1 as libc::c_int;
        }
        if t[0 as libc::c_int as usize] == t[1 as libc::c_int as usize] {
            return 1 as libc::c_int;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*gradient).n_stops {
        if !((*((*gradient).stops).offset(i as isize)).color.alpha_short as libc::c_int
            <= 0xff as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _gradient_color_average(
    mut gradient: *const cairo_gradient_pattern_t,
    mut color: *mut cairo_color_t,
) {
    let mut delta0: libc::c_double = 0.;
    let mut delta1: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut i: libc::c_uint = 0;
    let mut start: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut end: libc::c_uint = 0;
    if (*gradient).n_stops > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"gradient->n_stops > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2838 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void _gradient_color_average(const cairo_gradient_pattern_t *, cairo_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).base.extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.extend != CAIRO_EXTEND_NONE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            2839 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void _gradient_color_average(const cairo_gradient_pattern_t *, cairo_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).n_stops == 1 as libc::c_int as libc::c_uint {
        _cairo_color_init_rgba(
            color,
            (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.red,
            (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.green,
            (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.blue,
            (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.alpha,
        );
        return;
    }
    end = ((*gradient).n_stops).wrapping_sub(1 as libc::c_int as libc::c_uint);
    match (*gradient).base.extend as libc::c_uint {
        1 => {
            delta0 = 1.0f64
                + (*((*gradient).stops).offset(1 as libc::c_int as isize)).offset
                - (*((*gradient).stops).offset(end as isize)).offset;
            delta1 = 1.0f64
                + (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset
                - (*((*gradient).stops)
                    .offset(end.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
                    .offset;
        }
        2 => {
            delta0 = (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset
                + (*((*gradient).stops).offset(1 as libc::c_int as isize)).offset;
            delta1 = 2.0f64
                - (*((*gradient).stops)
                    .offset(end.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
                    .offset - (*((*gradient).stops).offset(end as isize)).offset;
        }
        3 => {
            delta1 = 1.0f64;
            delta0 = delta1;
            start = end;
        }
        0 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    2909 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 80],
                        &[libc::c_char; 80],
                    >(
                        b"void _gradient_color_average(const cairo_gradient_pattern_t *, cairo_color_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            _cairo_color_init_rgba(
                color,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
            );
            return;
        }
    }
    r = delta0 * (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.red;
    g = delta0 * (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.green;
    b = delta0 * (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.blue;
    a = delta0 * (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.alpha;
    i = start;
    while i < end {
        let mut delta: libc::c_double = (*((*gradient).stops)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
            .offset
            - (*((*gradient).stops)
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
                .offset;
        r += delta * (*((*gradient).stops).offset(i as isize)).color.red;
        g += delta * (*((*gradient).stops).offset(i as isize)).color.green;
        b += delta * (*((*gradient).stops).offset(i as isize)).color.blue;
        a += delta * (*((*gradient).stops).offset(i as isize)).color.alpha;
        i = i.wrapping_add(1);
    }
    r += delta1 * (*((*gradient).stops).offset(end as isize)).color.red;
    g += delta1 * (*((*gradient).stops).offset(end as isize)).color.green;
    b += delta1 * (*((*gradient).stops).offset(end as isize)).color.blue;
    a += delta1 * (*((*gradient).stops).offset(end as isize)).color.alpha;
    _cairo_color_init_rgba(color, r * 0.5f64, g * 0.5f64, b * 0.5f64, a * 0.5f64);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_alpha_range(
    mut pattern: *const cairo_pattern_t,
    mut out_min: *mut libc::c_double,
    mut out_max: *mut libc::c_double,
) {
    let mut alpha_min: libc::c_double = 0.;
    let mut alpha_max: libc::c_double = 0.;
    let mut current_block_26: u64;
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            let mut solid: *const cairo_solid_pattern_t = pattern
                as *mut cairo_solid_pattern_t;
            alpha_max = (*solid).color.alpha;
            alpha_min = alpha_max;
            current_block_26 = 2604890879466389055;
        }
        2 | 3 => {
            let mut gradient: *const cairo_gradient_pattern_t = pattern
                as *mut cairo_gradient_pattern_t;
            let mut i: libc::c_uint = 0;
            if (*gradient).n_stops >= 1 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"gradient->n_stops >= 1\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    2969 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"void _cairo_pattern_alpha_range(const cairo_pattern_t *, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            alpha_max = (*((*gradient).stops).offset(0 as libc::c_int as isize))
                .color
                .alpha;
            alpha_min = alpha_max;
            i = 1 as libc::c_int as libc::c_uint;
            while i < (*gradient).n_stops {
                if alpha_min > (*((*gradient).stops).offset(i as isize)).color.alpha {
                    alpha_min = (*((*gradient).stops).offset(i as isize)).color.alpha;
                } else if alpha_max
                    < (*((*gradient).stops).offset(i as isize)).color.alpha
                {
                    alpha_max = (*((*gradient).stops).offset(i as isize)).color.alpha;
                }
                i = i.wrapping_add(1);
            }
            current_block_26 = 2604890879466389055;
        }
        4 => {
            let mut mesh: *const cairo_mesh_pattern_t = pattern
                as *const cairo_mesh_pattern_t;
            let mut patch: *const cairo_mesh_patch_t = _cairo_array_index_const(
                &(*mesh).patches,
                0 as libc::c_int as libc::c_uint,
            ) as *const cairo_mesh_patch_t;
            let mut i_0: libc::c_uint = 0;
            let mut j: libc::c_uint = 0;
            let mut n: libc::c_uint = _cairo_array_num_elements(&(*mesh).patches);
            if n >= 1 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"n >= 1\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    2987 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"void _cairo_pattern_alpha_range(const cairo_pattern_t *, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            alpha_max = (*patch.offset(0 as libc::c_int as isize))
                .colors[0 as libc::c_int as usize]
                .alpha;
            alpha_min = alpha_max;
            i_0 = 0 as libc::c_int as libc::c_uint;
            while i_0 < n {
                j = 0 as libc::c_int as libc::c_uint;
                while j < 4 as libc::c_int as libc::c_uint {
                    if (*patch.offset(i_0 as isize)).colors[j as usize].alpha < alpha_min
                    {
                        alpha_min = (*patch.offset(i_0 as isize))
                            .colors[j as usize]
                            .alpha;
                    } else if (*patch.offset(i_0 as isize)).colors[j as usize].alpha
                        > alpha_max
                    {
                        alpha_max = (*patch.offset(i_0 as isize))
                            .colors[j as usize]
                            .alpha;
                    }
                    j = j.wrapping_add(1);
                }
                i_0 = i_0.wrapping_add(1);
            }
            current_block_26 = 2604890879466389055;
        }
        1 | 5 => {
            current_block_26 = 7088001668418101606;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    3003 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"void _cairo_pattern_alpha_range(const cairo_pattern_t *, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_26 = 7088001668418101606;
        }
    }
    match current_block_26 {
        7088001668418101606 => {
            alpha_min = 0 as libc::c_int as libc::c_double;
            alpha_max = 1 as libc::c_int as libc::c_double;
        }
        _ => {}
    }
    if !out_min.is_null() {
        *out_min = alpha_min;
    }
    if !out_max.is_null() {
        *out_max = alpha_max;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mesh_pattern_coord_box(
    mut mesh: *const cairo_mesh_pattern_t,
    mut out_xmin: *mut libc::c_double,
    mut out_ymin: *mut libc::c_double,
    mut out_xmax: *mut libc::c_double,
    mut out_ymax: *mut libc::c_double,
) -> cairo_bool_t {
    let mut patch: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut num_patches: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut x0: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    if ((*mesh).current_patch).is_null() {} else {
        __assert_fail(
            b"mesh->current_patch == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            3046 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"cairo_bool_t _cairo_mesh_pattern_coord_box(const cairo_mesh_pattern_t *, double *, double *, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    num_patches = _cairo_array_num_elements(&(*mesh).patches);
    if num_patches == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    patch = _cairo_array_index_const(&(*mesh).patches, 0 as libc::c_int as libc::c_uint)
        as *const cairo_mesh_patch_t;
    x1 = (*patch).points[0 as libc::c_int as usize][0 as libc::c_int as usize].x;
    x0 = x1;
    y1 = (*patch).points[0 as libc::c_int as usize][0 as libc::c_int as usize].y;
    y0 = y1;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_patches {
        j = 0 as libc::c_int as libc::c_uint;
        while j < 4 as libc::c_int as libc::c_uint {
            k = 0 as libc::c_int as libc::c_uint;
            while k < 4 as libc::c_int as libc::c_uint {
                x0 = if x0 < (*patch.offset(i as isize)).points[j as usize][k as usize].x
                {
                    x0
                } else {
                    (*patch.offset(i as isize)).points[j as usize][k as usize].x
                };
                y0 = if y0 < (*patch.offset(i as isize)).points[j as usize][k as usize].y
                {
                    y0
                } else {
                    (*patch.offset(i as isize)).points[j as usize][k as usize].y
                };
                x1 = if x1 > (*patch.offset(i as isize)).points[j as usize][k as usize].x
                {
                    x1
                } else {
                    (*patch.offset(i as isize)).points[j as usize][k as usize].x
                };
                y1 = if y1 > (*patch.offset(i as isize)).points[j as usize][k as usize].y
                {
                    y1
                } else {
                    (*patch.offset(i as isize)).points[j as usize][k as usize].y
                };
                k = k.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    *out_xmin = x0;
    *out_ymin = y0;
    *out_xmax = x1;
    *out_ymax = y1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_gradient_pattern_is_solid(
    mut gradient: *const cairo_gradient_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut color: *mut cairo_color_t,
) -> cairo_bool_t {
    let mut i: libc::c_uint = 0;
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*gradient).base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.type == CAIRO_PATTERN_TYPE_LINEAR || gradient->base.type == CAIRO_PATTERN_TYPE_RADIAL\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            3096 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 128],
                &[libc::c_char; 128],
            >(
                b"cairo_bool_t _cairo_gradient_pattern_is_solid(const cairo_gradient_pattern_t *, const cairo_rectangle_int_t *, cairo_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        let mut linear: *mut cairo_linear_pattern_t = gradient
            as *mut cairo_linear_pattern_t;
        if _linear_pattern_is_degenerate(linear) != 0 {
            _gradient_color_average(gradient, color);
            return 1 as libc::c_int;
        }
        if (*gradient).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        {
            let mut t: [libc::c_double; 2] = [0.; 2];
            if extents.is_null() {
                return 0 as libc::c_int;
            }
            _cairo_linear_pattern_box_to_parameter(
                linear,
                (*extents).x as libc::c_double,
                (*extents).y as libc::c_double,
                ((*extents).x + (*extents).width) as libc::c_double,
                ((*extents).y + (*extents).height) as libc::c_double,
                t.as_mut_ptr(),
            );
            if t[0 as libc::c_int as usize] < 0.0f64
                || t[1 as libc::c_int as usize] > 1.0f64
            {
                return 0 as libc::c_int;
            }
        }
    } else {
        return 0 as libc::c_int
    }
    i = 1 as libc::c_int as libc::c_uint;
    while i < (*gradient).n_stops {
        if _cairo_color_stop_equal(
            &mut (*((*gradient).stops).offset(0 as libc::c_int as isize)).color,
            &mut (*((*gradient).stops).offset(i as isize)).color,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    _cairo_color_init_rgba(
        color,
        (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.red,
        (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.green,
        (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.blue,
        (*((*gradient).stops).offset(0 as libc::c_int as isize)).color.alpha,
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_is_constant_alpha(
    mut abstract_pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut alpha: *mut libc::c_double,
) -> cairo_bool_t {
    let mut pattern: *const cairo_pattern_union_t = 0 as *const cairo_pattern_union_t;
    let mut color: cairo_color_t = cairo_color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        red_short: 0,
        green_short: 0,
        blue_short: 0,
        alpha_short: 0,
    };
    if _cairo_pattern_is_clear(abstract_pattern) != 0 {
        *alpha = 0.0f64;
        return 1 as libc::c_int;
    }
    if _cairo_pattern_is_opaque(abstract_pattern, extents) != 0 {
        *alpha = 1.0f64;
        return 1 as libc::c_int;
    }
    pattern = abstract_pattern as *mut cairo_pattern_union_t;
    match (*pattern).base.type_0 as libc::c_uint {
        0 => {
            *alpha = (*pattern).solid.color.alpha;
            return 1 as libc::c_int;
        }
        2 | 3 => {
            if _cairo_gradient_pattern_is_solid(
                &(*pattern).gradient.base,
                extents,
                &mut color,
            ) != 0
            {
                *alpha = color.alpha;
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
        1 | 5 | 4 => return 0 as libc::c_int,
        _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            3192 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"cairo_bool_t _cairo_pattern_is_constant_alpha(const cairo_pattern_t *, const cairo_rectangle_int_t *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _mesh_is_clear(
    mut mesh: *const cairo_mesh_pattern_t,
) -> cairo_bool_t {
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut is_valid: cairo_bool_t = 0;
    is_valid = _cairo_mesh_pattern_coord_box(mesh, &mut x1, &mut y1, &mut x2, &mut y2);
    if is_valid == 0 {
        return 1 as libc::c_int;
    }
    if x2 - x1 < 2.2204460492503131e-16f64 || y2 - y1 < 2.2204460492503131e-16f64 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_is_opaque_solid(
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    let mut solid: *mut cairo_solid_pattern_t = 0 as *mut cairo_solid_pattern_t;
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    solid = pattern as *mut cairo_solid_pattern_t;
    return ((*solid).color.alpha_short as libc::c_int >= 0xff00 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn _surface_is_opaque(
    mut pattern: *const cairo_surface_pattern_t,
    mut sample: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*(*pattern).surface).content as libc::c_uint
        & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint != 0
    {
        return 0 as libc::c_int;
    }
    if (*pattern).base.extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if _cairo_surface_get_extents((*pattern).surface, &mut extents) == 0 {
        return 1 as libc::c_int;
    }
    if sample.is_null() {
        return 0 as libc::c_int;
    }
    return _cairo_rectangle_contains_rectangle(&mut extents, sample);
}
unsafe extern "C" fn _raster_source_is_opaque(
    mut pattern: *const cairo_raster_source_pattern_t,
    mut sample: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    if (*pattern).content as libc::c_uint
        & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint != 0
    {
        return 0 as libc::c_int;
    }
    if (*pattern).base.extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if sample.is_null() {
        return 0 as libc::c_int;
    }
    return _cairo_rectangle_contains_rectangle(&(*pattern).extents, sample);
}
unsafe extern "C" fn _surface_is_clear(
    mut pattern: *const cairo_surface_pattern_t,
) -> cairo_bool_t {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if _cairo_surface_get_extents((*pattern).surface, &mut extents) != 0
        && (extents.width == 0 as libc::c_int || extents.height == 0 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    return ((*(*pattern).surface).is_clear() as libc::c_int != 0
        && (*(*pattern).surface).content as libc::c_uint
            & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint != 0) as libc::c_int;
}
unsafe extern "C" fn _raster_source_is_clear(
    mut pattern: *const cairo_raster_source_pattern_t,
) -> cairo_bool_t {
    return ((*pattern).extents.width == 0 as libc::c_int
        || (*pattern).extents.height == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn _gradient_is_opaque(
    mut gradient: *const cairo_gradient_pattern_t,
    mut sample: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut i: libc::c_uint = 0;
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*gradient).base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"gradient->base.type == CAIRO_PATTERN_TYPE_LINEAR || gradient->base.type == CAIRO_PATTERN_TYPE_RADIAL\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            3299 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"cairo_bool_t _gradient_is_opaque(const cairo_gradient_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).n_stops == 0 as libc::c_int as libc::c_uint
        || (*gradient).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            && (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset
                == (*((*gradient).stops)
                    .offset(
                        ((*gradient).n_stops)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ))
                    .offset
    {
        return 0 as libc::c_int;
    }
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        if (*gradient).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        {
            let mut t: [libc::c_double; 2] = [0.; 2];
            let mut linear: *mut cairo_linear_pattern_t = gradient
                as *mut cairo_linear_pattern_t;
            if _linear_pattern_is_degenerate(linear) != 0 {
                return 0 as libc::c_int;
            }
            if sample.is_null() {
                return 0 as libc::c_int;
            }
            _cairo_linear_pattern_box_to_parameter(
                linear,
                (*sample).x as libc::c_double,
                (*sample).y as libc::c_double,
                ((*sample).x + (*sample).width) as libc::c_double,
                ((*sample).y + (*sample).height) as libc::c_double,
                t.as_mut_ptr(),
            );
            if t[0 as libc::c_int as usize] < 0.0f64
                || t[1 as libc::c_int as usize] > 1.0f64
            {
                return 0 as libc::c_int;
            }
        }
    } else {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*gradient).n_stops {
        if !((*((*gradient).stops).offset(i as isize)).color.alpha_short as libc::c_int
            >= 0xff00 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_is_opaque(
    mut abstract_pattern: *const cairo_pattern_t,
    mut sample: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut pattern: *const cairo_pattern_union_t = 0 as *const cairo_pattern_union_t;
    if (*abstract_pattern).has_component_alpha != 0 {
        return 0 as libc::c_int;
    }
    pattern = abstract_pattern as *mut cairo_pattern_union_t;
    match (*pattern).base.type_0 as libc::c_uint {
        0 => return _cairo_pattern_is_opaque_solid(abstract_pattern),
        1 => return _surface_is_opaque(&(*pattern).surface, sample),
        5 => return _raster_source_is_opaque(&(*pattern).raster_source, sample),
        2 | 3 => return _gradient_is_opaque(&(*pattern).gradient.base, sample),
        4 => return 0 as libc::c_int,
        _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            3371 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"cairo_bool_t _cairo_pattern_is_opaque(const cairo_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_is_clear(
    mut abstract_pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    let mut pattern: *const cairo_pattern_union_t = 0 as *const cairo_pattern_union_t;
    if (*abstract_pattern).has_component_alpha != 0 {
        return 0 as libc::c_int;
    }
    pattern = abstract_pattern as *mut cairo_pattern_union_t;
    match (*abstract_pattern).type_0 as libc::c_uint {
        0 => {
            return ((*pattern).solid.color.alpha_short as libc::c_int
                <= 0xff as libc::c_int) as libc::c_int;
        }
        1 => return _surface_is_clear(&(*pattern).surface),
        5 => return _raster_source_is_clear(&(*pattern).raster_source),
        2 | 3 => {
            return _gradient_is_clear(
                &(*pattern).gradient.base,
                0 as *const cairo_rectangle_int_t,
            );
        }
        4 => return _mesh_is_clear(&(*pattern).mesh),
        _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
            3398 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"cairo_bool_t _cairo_pattern_is_clear(const cairo_pattern_t *)\0"))
                .as_ptr(),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn use_bilinear(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut t: libc::c_double,
) -> libc::c_int {
    let mut h: libc::c_double = x * x + y * y;
    if h < 1.0f64 / (0.75f64 * 0.75f64) {
        return 1 as libc::c_int;
    }
    if h > 3.99f64 && h < 4.01f64 && _cairo_fixed_from_double(x * y) == 0
        && _cairo_fixed_is_integer(_cairo_fixed_from_double(t)) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_analyze_filter(
    mut pattern: *const cairo_pattern_t,
) -> cairo_filter_t {
    match (*pattern).filter as libc::c_uint {
        1 | 2 | 4 | 0 => {
            if _cairo_matrix_is_pixel_exact(&(*pattern).matrix) != 0 {
                return CAIRO_FILTER_NEAREST
            } else {
                if (*pattern).filter as libc::c_uint
                    == CAIRO_FILTER_GOOD as libc::c_int as libc::c_uint
                    && use_bilinear(
                        (*pattern).matrix.xx,
                        (*pattern).matrix.xy,
                        (*pattern).matrix.x0,
                    ) != 0
                    && use_bilinear(
                        (*pattern).matrix.yx,
                        (*pattern).matrix.yy,
                        (*pattern).matrix.y0,
                    ) != 0
                {
                    return CAIRO_FILTER_BILINEAR;
                }
            }
        }
        3 | 5 | _ => {}
    }
    return (*pattern).filter;
}
#[inline]
unsafe extern "C" fn _cairo_hypot(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    return hypot(x, y);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_sampled_area(
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *mut cairo_rectangle_int_t,
) {
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut padx: libc::c_double = 0.;
    let mut pady: libc::c_double = 0.;
    if _cairo_matrix_is_identity(&(*pattern).matrix) != 0 {
        *sample = *extents;
        return;
    }
    x1 = (*extents).x as libc::c_double + 0.5f64;
    y1 = (*extents).y as libc::c_double + 0.5f64;
    x2 = x1 + ((*extents).width - 1 as libc::c_int) as libc::c_double;
    y2 = y1 + ((*extents).height - 1 as libc::c_int) as libc::c_double;
    _cairo_matrix_transform_bounding_box(
        &(*pattern).matrix,
        &mut x1,
        &mut y1,
        &mut x2,
        &mut y2,
        0 as *mut cairo_bool_t,
    );
    match (*pattern).filter as libc::c_uint {
        3 | 0 => {
            pady = 0.004f64;
            padx = pady;
        }
        1 => {
            padx = _cairo_hypot((*pattern).matrix.xx, (*pattern).matrix.xy);
            if padx <= 1.0f64 {
                padx = 0.495f64;
            } else if padx >= 16.0f64 {
                padx = 7.92f64;
            } else {
                padx *= 0.495f64;
            }
            pady = _cairo_hypot((*pattern).matrix.yx, (*pattern).matrix.yy);
            if pady <= 1.0f64 {
                pady = 0.495f64;
            } else if pady >= 16.0f64 {
                pady = 7.92f64;
            } else {
                pady *= 0.495f64;
            }
        }
        2 => {
            padx = _cairo_hypot((*pattern).matrix.xx, (*pattern).matrix.xy) * 1.98f64;
            if padx > 7.92f64 {
                padx = 7.92f64;
            }
            pady = _cairo_hypot((*pattern).matrix.yx, (*pattern).matrix.yy) * 1.98f64;
            if pady > 7.92f64 {
                pady = 7.92f64;
            }
        }
        4 | 5 | _ => {
            pady = 0.495f64;
            padx = pady;
        }
    }
    x1 = floor(x1 - padx);
    if x1
        < (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int)
            as libc::c_double
    {
        x1 = (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int)
            as libc::c_double;
    }
    (*sample).x = x1 as libc::c_int;
    y1 = floor(y1 - pady);
    if y1
        < (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int)
            as libc::c_double
    {
        y1 = (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int)
            as libc::c_double;
    }
    (*sample).y = y1 as libc::c_int;
    x2 = floor(x2 + padx) + 1.0f64;
    if x2 > (2147483647 as libc::c_int >> 8 as libc::c_int) as libc::c_double {
        x2 = (2147483647 as libc::c_int >> 8 as libc::c_int) as libc::c_double;
    }
    (*sample).width = (x2 - x1) as libc::c_int;
    y2 = floor(y2 + pady) + 1.0f64;
    if y2 > (2147483647 as libc::c_int >> 8 as libc::c_int) as libc::c_double {
        y2 = (2147483647 as libc::c_int >> 8 as libc::c_int) as libc::c_double;
    }
    (*sample).height = (y2 - y1) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_get_extents(
    mut pattern: *const cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut is_vector: cairo_bool_t,
) {
    let mut current_block: u64;
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut ix1: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut iy1: libc::c_int = 0;
    let mut iy2: libc::c_int = 0;
    let mut round_x: cairo_bool_t = 0 as libc::c_int;
    let mut round_y: cairo_bool_t = 0 as libc::c_int;
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            current_block = 4637608790532360702;
        }
        1 => {
            let mut surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let mut surface_pattern: *const cairo_surface_pattern_t = pattern
                as *const cairo_surface_pattern_t;
            let mut surface: *mut cairo_surface_t = (*surface_pattern).surface;
            if _cairo_surface_get_extents(surface, &mut surface_extents) == 0 {
                current_block = 4637608790532360702;
            } else if surface_extents.width == 0 as libc::c_int
                || surface_extents.height == 0 as libc::c_int
            {
                current_block = 722612367739836663;
            } else if (*pattern).extend as libc::c_uint
                != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            {
                current_block = 4637608790532360702;
            } else {
                x1 = surface_extents.x as libc::c_double;
                y1 = surface_extents.y as libc::c_double;
                x2 = (surface_extents.x + surface_extents.width) as libc::c_double;
                y2 = (surface_extents.y + surface_extents.height) as libc::c_double;
                current_block = 12524008191624889971;
            }
        }
        5 => {
            let mut raster: *const cairo_raster_source_pattern_t = pattern
                as *const cairo_raster_source_pattern_t;
            if (*raster).extents.width == 0 as libc::c_int
                || (*raster).extents.height == 0 as libc::c_int
            {
                current_block = 722612367739836663;
            } else if (*pattern).extend as libc::c_uint
                != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            {
                current_block = 4637608790532360702;
            } else {
                x1 = (*raster).extents.x as libc::c_double;
                y1 = (*raster).extents.y as libc::c_double;
                x2 = ((*raster).extents.x + (*raster).extents.width) as libc::c_double;
                y2 = ((*raster).extents.y + (*raster).extents.height) as libc::c_double;
                current_block = 12524008191624889971;
            }
        }
        3 => {
            let mut radial: *const cairo_radial_pattern_t = pattern
                as *const cairo_radial_pattern_t;
            let mut cx1: libc::c_double = 0.;
            let mut cy1: libc::c_double = 0.;
            let mut cx2: libc::c_double = 0.;
            let mut cy2: libc::c_double = 0.;
            let mut r1: libc::c_double = 0.;
            let mut r2: libc::c_double = 0.;
            if _radial_pattern_is_degenerate(radial) != 0 {
                current_block = 722612367739836663;
            } else if (*pattern).extend as libc::c_uint
                != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            {
                current_block = 4637608790532360702;
            } else {
                cx1 = (*radial).cd1.center.x;
                cy1 = (*radial).cd1.center.y;
                r1 = (*radial).cd1.radius;
                cx2 = (*radial).cd2.center.x;
                cy2 = (*radial).cd2.center.y;
                r2 = (*radial).cd2.radius;
                x1 = if cx1 - r1 < cx2 - r2 { cx1 - r1 } else { cx2 - r2 };
                y1 = if cy1 - r1 < cy2 - r2 { cy1 - r1 } else { cy2 - r2 };
                x2 = if cx1 + r1 > cx2 + r2 { cx1 + r1 } else { cx2 + r2 };
                y2 = if cy1 + r1 > cy2 + r2 { cy1 + r1 } else { cy2 + r2 };
                current_block = 8464383504555462953;
            }
        }
        2 => {
            let mut linear: *const cairo_linear_pattern_t = pattern
                as *const cairo_linear_pattern_t;
            if (*pattern).extend as libc::c_uint
                != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            {
                current_block = 4637608790532360702;
            } else if _linear_pattern_is_degenerate(linear) != 0 {
                current_block = 722612367739836663;
            } else if (*pattern).matrix.xy != 0.0f64 || (*pattern).matrix.yx != 0.0f64 {
                current_block = 4637608790532360702;
            } else {
                if (*linear).pd1.x == (*linear).pd2.x {
                    x1 = -::std::f64::INFINITY;
                    x2 = ::std::f64::INFINITY;
                    y1 = if (*linear).pd1.y < (*linear).pd2.y {
                        (*linear).pd1.y
                    } else {
                        (*linear).pd2.y
                    };
                    y2 = if (*linear).pd1.y > (*linear).pd2.y {
                        (*linear).pd1.y
                    } else {
                        (*linear).pd2.y
                    };
                    current_block = 5684854171168229155;
                } else if (*linear).pd1.y == (*linear).pd2.y {
                    x1 = if (*linear).pd1.x < (*linear).pd2.x {
                        (*linear).pd1.x
                    } else {
                        (*linear).pd2.x
                    };
                    x2 = if (*linear).pd1.x > (*linear).pd2.x {
                        (*linear).pd1.x
                    } else {
                        (*linear).pd2.x
                    };
                    y1 = -::std::f64::INFINITY;
                    y2 = ::std::f64::INFINITY;
                    current_block = 5684854171168229155;
                } else {
                    current_block = 4637608790532360702;
                }
                match current_block {
                    4637608790532360702 => {}
                    _ => {
                        round_y = 1 as libc::c_int;
                        round_x = round_y;
                        current_block = 8464383504555462953;
                    }
                }
            }
        }
        4 => {
            let mut mesh: *const cairo_mesh_pattern_t = pattern
                as *const cairo_mesh_pattern_t;
            if _cairo_mesh_pattern_coord_box(mesh, &mut x1, &mut y1, &mut x2, &mut y2)
                == 0
            {
                current_block = 722612367739836663;
            } else {
                current_block = 8464383504555462953;
            }
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    3766 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void _cairo_pattern_get_extents(const cairo_pattern_t *, cairo_rectangle_int_t *, cairo_bool_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block = 8464383504555462953;
        }
    }
    match current_block {
        4637608790532360702 => {
            _cairo_unbounded_rectangle_init(extents);
            return;
        }
        722612367739836663 => {
            let ref mut fresh23 = (*extents).y;
            *fresh23 = 0 as libc::c_int;
            (*extents).x = *fresh23;
            let ref mut fresh24 = (*extents).height;
            *fresh24 = 0 as libc::c_int;
            (*extents).width = *fresh24;
            return;
        }
        12524008191624889971 => {
            match (*pattern).filter as libc::c_uint {
                3 | 0 => {
                    round_y = 1 as libc::c_int;
                    round_x = round_y;
                    x1 -= 0.004f64;
                    y1 -= 0.004f64;
                    x2 += 0.004f64;
                    y2 += 0.004f64;
                }
                2 => {}
                4 | 5 | 1 | _ => {
                    if _cairo_hypot((*pattern).matrix.xx, (*pattern).matrix.yx) < 1.0f64
                    {
                        x1 -= 0.5f64;
                        x2 += 0.5f64;
                        round_x = 1 as libc::c_int;
                    }
                    if _cairo_hypot((*pattern).matrix.xy, (*pattern).matrix.yy) < 1.0f64
                    {
                        y1 -= 0.5f64;
                        y2 += 0.5f64;
                        round_y = 1 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    if _cairo_matrix_is_translation(&(*pattern).matrix) != 0 {
        x1 -= (*pattern).matrix.x0;
        x2 -= (*pattern).matrix.x0;
        y1 -= (*pattern).matrix.y0;
        y2 -= (*pattern).matrix.y0;
    } else {
        let mut imatrix: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        imatrix = (*pattern).matrix;
        status = cairo_matrix_invert(&mut imatrix);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                3779 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void _cairo_pattern_get_extents(const cairo_pattern_t *, cairo_rectangle_int_t *, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_matrix_transform_bounding_box(
            &mut imatrix,
            &mut x1,
            &mut y1,
            &mut x2,
            &mut y2,
            0 as *mut cairo_bool_t,
        );
    }
    if round_x == 0 {
        x1 -= 0.5f64;
        x2 += 0.5f64;
    }
    if x1
        < (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int)
            as libc::c_double
    {
        ix1 = -(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int;
    } else {
        ix1 = _cairo_lround(x1);
    }
    if x2 > (2147483647 as libc::c_int >> 8 as libc::c_int) as libc::c_double {
        ix2 = 2147483647 as libc::c_int >> 8 as libc::c_int;
    } else {
        ix2 = _cairo_lround(x2);
    }
    (*extents).x = ix1;
    (*extents).width = ix2 - ix1;
    if is_vector != 0 && (*extents).width == 0 as libc::c_int && x1 != x2 {
        (*extents).width += 1 as libc::c_int;
    }
    if round_y == 0 {
        y1 -= 0.5f64;
        y2 += 0.5f64;
    }
    if y1
        < (-(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int)
            as libc::c_double
    {
        iy1 = -(2147483647 as libc::c_int) - 1 as libc::c_int >> 8 as libc::c_int;
    } else {
        iy1 = _cairo_lround(y1);
    }
    if y2 > (2147483647 as libc::c_int >> 8 as libc::c_int) as libc::c_double {
        iy2 = 2147483647 as libc::c_int >> 8 as libc::c_int;
    } else {
        iy2 = _cairo_lround(y2);
    }
    (*extents).y = iy1;
    (*extents).height = iy2 - iy1;
    if is_vector != 0 && (*extents).height == 0 as libc::c_int && y1 != y2 {
        (*extents).height += 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_get_ink_extents(
    mut pattern: *const cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_int_status_t {
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*pattern).extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        let mut surface_pattern: *const cairo_surface_pattern_t = pattern
            as *const cairo_surface_pattern_t;
        let mut surface: *mut cairo_surface_t = (*surface_pattern).surface;
        surface = _cairo_surface_get_source(surface, 0 as *mut cairo_rectangle_int_t);
        if _cairo_surface_is_recording(surface) != 0 {
            let mut imatrix: cairo_matrix_t = cairo_matrix_t {
                xx: 0.,
                yx: 0.,
                xy: 0.,
                yy: 0.,
                x0: 0.,
                y0: 0.,
            };
            let mut box_0: cairo_box_t = cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
            imatrix = (*pattern).matrix;
            status = cairo_matrix_invert(&mut imatrix);
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    3856 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 100],
                        &[libc::c_char; 100],
                    >(
                        b"cairo_int_status_t _cairo_pattern_get_ink_extents(const cairo_pattern_t *, cairo_rectangle_int_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            status = _cairo_recording_surface_get_ink_bbox(
                surface as *mut cairo_recording_surface_t,
                &mut box_0,
                &mut imatrix,
            );
            if status as u64 != 0 {
                return status as cairo_int_status_t;
            }
            _cairo_box_round_to_rectangle(&mut box_0, extents);
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    }
    _cairo_pattern_get_extents(pattern, extents, 1 as libc::c_int);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_solid_pattern_hash(
    mut hash: uintptr_t,
    mut solid: *const cairo_solid_pattern_t,
) -> uintptr_t {
    hash = _cairo_hash_bytes(
        hash,
        &(*solid).color as *const cairo_color_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_color_t>() as libc::c_ulong as libc::c_uint,
    );
    return hash;
}
unsafe extern "C" fn _cairo_gradient_color_stops_hash(
    mut hash: uintptr_t,
    mut gradient: *const cairo_gradient_pattern_t,
) -> uintptr_t {
    let mut n: libc::c_uint = 0;
    hash = _cairo_hash_bytes(
        hash,
        &(*gradient).n_stops as *const libc::c_uint as *const libc::c_void,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_uint,
    );
    n = 0 as libc::c_int as libc::c_uint;
    while n < (*gradient).n_stops {
        hash = _cairo_hash_bytes(
            hash,
            &mut (*((*gradient).stops).offset(n as isize)).offset as *mut libc::c_double
                as *const libc::c_void,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_uint,
        );
        hash = _cairo_hash_bytes(
            hash,
            &mut (*((*gradient).stops).offset(n as isize)).color
                as *mut cairo_color_stop_t as *const libc::c_void,
            ::std::mem::size_of::<cairo_color_stop_t>() as libc::c_ulong as libc::c_uint,
        );
        n = n.wrapping_add(1);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_linear_pattern_hash(
    mut hash: uintptr_t,
    mut linear: *const cairo_linear_pattern_t,
) -> uintptr_t {
    hash = _cairo_hash_bytes(
        hash,
        &(*linear).pd1 as *const cairo_point_double_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_point_double_t>() as libc::c_ulong as libc::c_uint,
    );
    hash = _cairo_hash_bytes(
        hash,
        &(*linear).pd2 as *const cairo_point_double_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_point_double_t>() as libc::c_ulong as libc::c_uint,
    );
    return _cairo_gradient_color_stops_hash(hash, &(*linear).base);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_radial_pattern_hash(
    mut hash: uintptr_t,
    mut radial: *const cairo_radial_pattern_t,
) -> uintptr_t {
    hash = _cairo_hash_bytes(
        hash,
        &(*radial).cd1.center as *const cairo_point_double_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_point_double_t>() as libc::c_ulong as libc::c_uint,
    );
    hash = _cairo_hash_bytes(
        hash,
        &(*radial).cd1.radius as *const libc::c_double as *const libc::c_void,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_uint,
    );
    hash = _cairo_hash_bytes(
        hash,
        &(*radial).cd2.center as *const cairo_point_double_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_point_double_t>() as libc::c_ulong as libc::c_uint,
    );
    hash = _cairo_hash_bytes(
        hash,
        &(*radial).cd2.radius as *const libc::c_double as *const libc::c_void,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_uint,
    );
    return _cairo_gradient_color_stops_hash(hash, &(*radial).base);
}
unsafe extern "C" fn _cairo_mesh_pattern_hash(
    mut hash: uintptr_t,
    mut mesh: *const cairo_mesh_pattern_t,
) -> uintptr_t {
    let mut patch: *const cairo_mesh_patch_t = _cairo_array_index_const(
        &(*mesh).patches,
        0 as libc::c_int as libc::c_uint,
    ) as *const cairo_mesh_patch_t;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = _cairo_array_num_elements(&(*mesh).patches);
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        hash = _cairo_hash_bytes(
            hash,
            patch.offset(i as isize) as *const libc::c_void,
            ::std::mem::size_of::<cairo_mesh_patch_t>() as libc::c_ulong as libc::c_uint,
        );
        i = i.wrapping_add(1);
    }
    return hash;
}
unsafe extern "C" fn _cairo_surface_pattern_hash(
    mut hash: uintptr_t,
    mut surface: *const cairo_surface_pattern_t,
) -> uintptr_t {
    hash ^= (*(*surface).surface).unique_id as libc::c_ulong;
    return hash;
}
unsafe extern "C" fn _cairo_raster_source_pattern_hash(
    mut hash: uintptr_t,
    mut raster: *const cairo_raster_source_pattern_t,
) -> uintptr_t {
    hash ^= (*raster).user_data as uintptr_t;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_hash(
    mut pattern: *const cairo_pattern_t,
) -> uintptr_t {
    let mut hash: uintptr_t = 5381 as libc::c_int as uintptr_t;
    if (*pattern).status as u64 != 0 {
        return 0 as libc::c_int as uintptr_t;
    }
    hash = _cairo_hash_bytes(
        hash,
        &(*pattern).type_0 as *const cairo_pattern_type_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_pattern_type_t>() as libc::c_ulong as libc::c_uint,
    );
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        hash = _cairo_hash_bytes(
            hash,
            &(*pattern).matrix as *const cairo_matrix_t as *const libc::c_void,
            ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong as libc::c_uint,
        );
        hash = _cairo_hash_bytes(
            hash,
            &(*pattern).filter as *const cairo_filter_t as *const libc::c_void,
            ::std::mem::size_of::<cairo_filter_t>() as libc::c_ulong as libc::c_uint,
        );
        hash = _cairo_hash_bytes(
            hash,
            &(*pattern).extend as *const cairo_extend_t as *const libc::c_void,
            ::std::mem::size_of::<cairo_extend_t>() as libc::c_ulong as libc::c_uint,
        );
        hash = _cairo_hash_bytes(
            hash,
            &(*pattern).has_component_alpha as *const cairo_bool_t
                as *const libc::c_void,
            ::std::mem::size_of::<cairo_bool_t>() as libc::c_ulong as libc::c_uint,
        );
    }
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            return _cairo_solid_pattern_hash(hash, pattern as *mut cairo_solid_pattern_t);
        }
        2 => {
            return _cairo_linear_pattern_hash(
                hash,
                pattern as *mut cairo_linear_pattern_t,
            );
        }
        3 => {
            return _cairo_radial_pattern_hash(
                hash,
                pattern as *mut cairo_radial_pattern_t,
            );
        }
        4 => return _cairo_mesh_pattern_hash(hash, pattern as *mut cairo_mesh_pattern_t),
        1 => {
            return _cairo_surface_pattern_hash(
                hash,
                pattern as *mut cairo_surface_pattern_t,
            );
        }
        5 => {
            return _cairo_raster_source_pattern_hash(
                hash,
                pattern as *mut cairo_raster_source_pattern_t,
            );
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    3990 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"uintptr_t _cairo_pattern_hash(const cairo_pattern_t *)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int as uintptr_t;
        }
    };
}
unsafe extern "C" fn _cairo_solid_pattern_equal(
    mut a: *const cairo_solid_pattern_t,
    mut b: *const cairo_solid_pattern_t,
) -> cairo_bool_t {
    return _cairo_color_equal(&(*a).color, &(*b).color);
}
unsafe extern "C" fn _cairo_gradient_color_stops_equal(
    mut a: *const cairo_gradient_pattern_t,
    mut b: *const cairo_gradient_pattern_t,
) -> cairo_bool_t {
    let mut n: libc::c_uint = 0;
    if (*a).n_stops != (*b).n_stops {
        return 0 as libc::c_int;
    }
    n = 0 as libc::c_int as libc::c_uint;
    while n < (*a).n_stops {
        if (*((*a).stops).offset(n as isize)).offset
            != (*((*b).stops).offset(n as isize)).offset
        {
            return 0 as libc::c_int;
        }
        if _cairo_color_stop_equal(
            &mut (*((*a).stops).offset(n as isize)).color,
            &mut (*((*b).stops).offset(n as isize)).color,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        n = n.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_linear_pattern_equal(
    mut a: *const cairo_linear_pattern_t,
    mut b: *const cairo_linear_pattern_t,
) -> cairo_bool_t {
    if (*a).pd1.x != (*b).pd1.x {
        return 0 as libc::c_int;
    }
    if (*a).pd1.y != (*b).pd1.y {
        return 0 as libc::c_int;
    }
    if (*a).pd2.x != (*b).pd2.x {
        return 0 as libc::c_int;
    }
    if (*a).pd2.y != (*b).pd2.y {
        return 0 as libc::c_int;
    }
    return _cairo_gradient_color_stops_equal(&(*a).base, &(*b).base);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_radial_pattern_equal(
    mut a: *const cairo_radial_pattern_t,
    mut b: *const cairo_radial_pattern_t,
) -> cairo_bool_t {
    if (*a).cd1.center.x != (*b).cd1.center.x {
        return 0 as libc::c_int;
    }
    if (*a).cd1.center.y != (*b).cd1.center.y {
        return 0 as libc::c_int;
    }
    if (*a).cd1.radius != (*b).cd1.radius {
        return 0 as libc::c_int;
    }
    if (*a).cd2.center.x != (*b).cd2.center.x {
        return 0 as libc::c_int;
    }
    if (*a).cd2.center.y != (*b).cd2.center.y {
        return 0 as libc::c_int;
    }
    if (*a).cd2.radius != (*b).cd2.radius {
        return 0 as libc::c_int;
    }
    return _cairo_gradient_color_stops_equal(&(*a).base, &(*b).base);
}
unsafe extern "C" fn _cairo_mesh_pattern_equal(
    mut a: *const cairo_mesh_pattern_t,
    mut b: *const cairo_mesh_pattern_t,
) -> cairo_bool_t {
    let mut patch_a: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut patch_b: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut i: libc::c_uint = 0;
    let mut num_patches_a: libc::c_uint = 0;
    let mut num_patches_b: libc::c_uint = 0;
    num_patches_a = _cairo_array_num_elements(&(*a).patches);
    num_patches_b = _cairo_array_num_elements(&(*b).patches);
    if num_patches_a != num_patches_b {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_patches_a {
        patch_a = _cairo_array_index_const(&(*a).patches, i)
            as *const cairo_mesh_patch_t;
        patch_b = _cairo_array_index_const(&(*b).patches, i)
            as *const cairo_mesh_patch_t;
        if memcmp(
            patch_a as *const libc::c_void,
            patch_b as *const libc::c_void,
            ::std::mem::size_of::<cairo_mesh_patch_t>() as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_surface_pattern_equal(
    mut a: *const cairo_surface_pattern_t,
    mut b: *const cairo_surface_pattern_t,
) -> cairo_bool_t {
    return ((*(*a).surface).unique_id == (*(*b).surface).unique_id) as libc::c_int;
}
unsafe extern "C" fn _cairo_raster_source_pattern_equal(
    mut a: *const cairo_raster_source_pattern_t,
    mut b: *const cairo_raster_source_pattern_t,
) -> cairo_bool_t {
    return ((*a).user_data == (*b).user_data) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_equal(
    mut a: *const cairo_pattern_t,
    mut b: *const cairo_pattern_t,
) -> cairo_bool_t {
    if (*a).status as libc::c_uint != 0 || (*b).status as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    if a == b {
        return 1 as libc::c_int;
    }
    if (*a).type_0 as libc::c_uint != (*b).type_0 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*a).has_component_alpha != (*b).has_component_alpha {
        return 0 as libc::c_int;
    }
    if (*a).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        if memcmp(
            &(*a).matrix as *const cairo_matrix_t as *const libc::c_void,
            &(*b).matrix as *const cairo_matrix_t as *const libc::c_void,
            ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
        ) != 0
        {
            return 0 as libc::c_int;
        }
        if (*a).filter as libc::c_uint != (*b).filter as libc::c_uint {
            return 0 as libc::c_int;
        }
        if (*a).extend as libc::c_uint != (*b).extend as libc::c_uint {
            return 0 as libc::c_int;
        }
    }
    match (*a).type_0 as libc::c_uint {
        0 => {
            return _cairo_solid_pattern_equal(
                a as *mut cairo_solid_pattern_t,
                b as *mut cairo_solid_pattern_t,
            );
        }
        2 => {
            return _cairo_linear_pattern_equal(
                a as *mut cairo_linear_pattern_t,
                b as *mut cairo_linear_pattern_t,
            );
        }
        3 => {
            return _cairo_radial_pattern_equal(
                a as *mut cairo_radial_pattern_t,
                b as *mut cairo_radial_pattern_t,
            );
        }
        4 => {
            return _cairo_mesh_pattern_equal(
                a as *mut cairo_mesh_pattern_t,
                b as *mut cairo_mesh_pattern_t,
            );
        }
        1 => {
            return _cairo_surface_pattern_equal(
                a as *mut cairo_surface_pattern_t,
                b as *mut cairo_surface_pattern_t,
            );
        }
        5 => {
            return _cairo_raster_source_pattern_equal(
                a as *mut cairo_raster_source_pattern_t,
                b as *mut cairo_raster_source_pattern_t,
            );
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    4148 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"cairo_bool_t _cairo_pattern_equal(const cairo_pattern_t *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_rgba(
    mut pattern: *mut cairo_pattern_t,
    mut red: *mut libc::c_double,
    mut green: *mut libc::c_double,
    mut blue: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
) -> cairo_status_t {
    let mut solid: *mut cairo_solid_pattern_t = pattern as *mut cairo_solid_pattern_t;
    let mut r0: libc::c_double = 0.;
    let mut g0: libc::c_double = 0.;
    let mut b0: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    _cairo_color_get_rgba(&mut (*solid).color, &mut r0, &mut g0, &mut b0, &mut a0);
    if !red.is_null() {
        *red = r0;
    }
    if !green.is_null() {
        *green = g0;
    }
    if !blue.is_null() {
        *blue = b0;
    }
    if !alpha.is_null() {
        *alpha = a0;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_surface(
    mut pattern: *mut cairo_pattern_t,
    mut surface: *mut *mut cairo_surface_t,
) -> cairo_status_t {
    let mut spat: *mut cairo_surface_pattern_t = pattern as *mut cairo_surface_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if !surface.is_null() {
        *surface = (*spat).surface;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_color_stop_rgba(
    mut pattern: *mut cairo_pattern_t,
    mut index: libc::c_int,
    mut offset: *mut libc::c_double,
    mut red: *mut libc::c_double,
    mut green: *mut libc::c_double,
    mut blue: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
) -> cairo_status_t {
    let mut gradient: *mut cairo_gradient_pattern_t = pattern
        as *mut cairo_gradient_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        && (*pattern).type_0 as libc::c_uint
            != CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if index < 0 as libc::c_int || index as libc::c_uint >= (*gradient).n_stops {
        return _cairo_error(CAIRO_STATUS_INVALID_INDEX);
    }
    if !offset.is_null() {
        *offset = (*((*gradient).stops).offset(index as isize)).offset;
    }
    if !red.is_null() {
        *red = (*((*gradient).stops).offset(index as isize)).color.red;
    }
    if !green.is_null() {
        *green = (*((*gradient).stops).offset(index as isize)).color.green;
    }
    if !blue.is_null() {
        *blue = (*((*gradient).stops).offset(index as isize)).color.blue;
    }
    if !alpha.is_null() {
        *alpha = (*((*gradient).stops).offset(index as isize)).color.alpha;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_color_stop_count(
    mut pattern: *mut cairo_pattern_t,
    mut count: *mut libc::c_int,
) -> cairo_status_t {
    let mut gradient: *mut cairo_gradient_pattern_t = pattern
        as *mut cairo_gradient_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        && (*pattern).type_0 as libc::c_uint
            != CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if !count.is_null() {
        *count = (*gradient).n_stops as libc::c_int;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_linear_points(
    mut pattern: *mut cairo_pattern_t,
    mut x0: *mut libc::c_double,
    mut y0: *mut libc::c_double,
    mut x1: *mut libc::c_double,
    mut y1: *mut libc::c_double,
) -> cairo_status_t {
    let mut linear: *mut cairo_linear_pattern_t = pattern as *mut cairo_linear_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if !x0.is_null() {
        *x0 = (*linear).pd1.x;
    }
    if !y0.is_null() {
        *y0 = (*linear).pd1.y;
    }
    if !x1.is_null() {
        *x1 = (*linear).pd2.x;
    }
    if !y1.is_null() {
        *y1 = (*linear).pd2.y;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pattern_get_radial_circles(
    mut pattern: *mut cairo_pattern_t,
    mut x0: *mut libc::c_double,
    mut y0: *mut libc::c_double,
    mut r0: *mut libc::c_double,
    mut x1: *mut libc::c_double,
    mut y1: *mut libc::c_double,
    mut r1: *mut libc::c_double,
) -> cairo_status_t {
    let mut radial: *mut cairo_radial_pattern_t = pattern as *mut cairo_radial_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if !x0.is_null() {
        *x0 = (*radial).cd1.center.x;
    }
    if !y0.is_null() {
        *y0 = (*radial).cd1.center.y;
    }
    if !r0.is_null() {
        *r0 = (*radial).cd1.radius;
    }
    if !x1.is_null() {
        *x1 = (*radial).cd2.center.x;
    }
    if !y1.is_null() {
        *y1 = (*radial).cd2.center.y;
    }
    if !r1.is_null() {
        *r1 = (*radial).cd2.radius;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_get_patch_count(
    mut pattern: *mut cairo_pattern_t,
    mut count: *mut libc::c_uint,
) -> cairo_status_t {
    let mut mesh: *mut cairo_mesh_pattern_t = pattern as *mut cairo_mesh_pattern_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if !count.is_null() {
        *count = _cairo_array_num_elements(&mut (*mesh).patches);
        if !((*mesh).current_patch).is_null() {
            *count = (*count).wrapping_sub(1 as libc::c_int as libc::c_uint);
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_get_path(
    mut pattern: *mut cairo_pattern_t,
    mut patch_num: libc::c_uint,
) -> *mut cairo_path_t {
    let mut mesh: *mut cairo_mesh_pattern_t = pattern as *mut cairo_mesh_pattern_t;
    let mut patch: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut path: *mut cairo_path_t = 0 as *mut cairo_path_t;
    let mut data: *mut cairo_path_data_t = 0 as *mut cairo_path_data_t;
    let mut patch_count: libc::c_uint = 0;
    let mut l: libc::c_int = 0;
    let mut current_point: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return _cairo_path_create_in_error((*pattern).status);
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return _cairo_path_create_in_error(
            _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH),
        );
    }
    patch_count = _cairo_array_num_elements(&mut (*mesh).patches);
    if !((*mesh).current_patch).is_null() {
        patch_count = patch_count.wrapping_sub(1);
    }
    if patch_num >= patch_count {
        return _cairo_path_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_INDEX));
    }
    patch = _cairo_array_index_const(&mut (*mesh).patches, patch_num)
        as *const cairo_mesh_patch_t;
    path = (if ::std::mem::size_of::<cairo_path_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_path_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_path_t;
    if path.is_null() {
        return _cairo_path_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    (*path).num_data = 18 as libc::c_int;
    let ref mut fresh25 = (*path).data;
    *fresh25 = _cairo_malloc_ab(
        (*path).num_data as size_t,
        ::std::mem::size_of::<cairo_path_data_t>() as libc::c_ulong,
    ) as *mut cairo_path_data_t;
    if ((*path).data).is_null() {
        free(path as *mut libc::c_void);
        return _cairo_path_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    data = (*path).data;
    (*data.offset(0 as libc::c_int as isize)).header.type_0 = CAIRO_PATH_MOVE_TO;
    (*data.offset(0 as libc::c_int as isize)).header.length = 2 as libc::c_int;
    (*data.offset(1 as libc::c_int as isize))
        .point
        .x = (*patch).points[0 as libc::c_int as usize][0 as libc::c_int as usize].x;
    (*data.offset(1 as libc::c_int as isize))
        .point
        .y = (*patch).points[0 as libc::c_int as usize][0 as libc::c_int as usize].y;
    data = data.offset((*data.offset(0 as libc::c_int as isize)).header.length as isize);
    current_point = 0 as libc::c_int;
    l = 0 as libc::c_int;
    while l < 4 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        (*data.offset(0 as libc::c_int as isize)).header.type_0 = CAIRO_PATH_CURVE_TO;
        (*data.offset(0 as libc::c_int as isize)).header.length = 4 as libc::c_int;
        k = 1 as libc::c_int;
        while k < 4 as libc::c_int {
            current_point = (current_point + 1 as libc::c_int) % 12 as libc::c_int;
            i = mesh_path_point_i[current_point as usize];
            j = mesh_path_point_j[current_point as usize];
            (*data.offset(k as isize))
                .point
                .x = (*patch).points[i as usize][j as usize].x;
            (*data.offset(k as isize))
                .point
                .y = (*patch).points[i as usize][j as usize].y;
            k += 1;
        }
        data = data
            .offset((*data.offset(0 as libc::c_int as isize)).header.length as isize);
        l += 1;
    }
    (*path).status = CAIRO_STATUS_SUCCESS;
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_get_corner_color_rgba(
    mut pattern: *mut cairo_pattern_t,
    mut patch_num: libc::c_uint,
    mut corner_num: libc::c_uint,
    mut red: *mut libc::c_double,
    mut green: *mut libc::c_double,
    mut blue: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
) -> cairo_status_t {
    let mut mesh: *mut cairo_mesh_pattern_t = pattern as *mut cairo_mesh_pattern_t;
    let mut patch_count: libc::c_uint = 0;
    let mut patch: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if corner_num > 3 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_INVALID_INDEX);
    }
    patch_count = _cairo_array_num_elements(&mut (*mesh).patches);
    if !((*mesh).current_patch).is_null() {
        patch_count = patch_count.wrapping_sub(1);
    }
    if patch_num >= patch_count {
        return _cairo_error(CAIRO_STATUS_INVALID_INDEX);
    }
    patch = _cairo_array_index_const(&mut (*mesh).patches, patch_num)
        as *const cairo_mesh_patch_t;
    if !red.is_null() {
        *red = (*patch).colors[corner_num as usize].red;
    }
    if !green.is_null() {
        *green = (*patch).colors[corner_num as usize].green;
    }
    if !blue.is_null() {
        *blue = (*patch).colors[corner_num as usize].blue;
    }
    if !alpha.is_null() {
        *alpha = (*patch).colors[corner_num as usize].alpha;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_mesh_pattern_get_control_point(
    mut pattern: *mut cairo_pattern_t,
    mut patch_num: libc::c_uint,
    mut point_num: libc::c_uint,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> cairo_status_t {
    let mut mesh: *mut cairo_mesh_pattern_t = pattern as *mut cairo_mesh_pattern_t;
    let mut patch: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut patch_count: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*pattern).status as u64 != 0 {
        return (*pattern).status;
    }
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
    }
    if point_num > 3 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_INVALID_INDEX);
    }
    patch_count = _cairo_array_num_elements(&mut (*mesh).patches);
    if !((*mesh).current_patch).is_null() {
        patch_count = patch_count.wrapping_sub(1);
    }
    if patch_num >= patch_count {
        return _cairo_error(CAIRO_STATUS_INVALID_INDEX);
    }
    patch = _cairo_array_index_const(&mut (*mesh).patches, patch_num)
        as *const cairo_mesh_patch_t;
    i = mesh_control_point_i[point_num as usize];
    j = mesh_control_point_j[point_num as usize];
    if !x.is_null() {
        *x = (*patch).points[i as usize][j as usize].x;
    }
    if !y.is_null() {
        *y = (*patch).points[i as usize][j as usize].y;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pattern_reset_static_data() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[freed_pool_t; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<freed_pool_t>() as libc::c_ulong)
            as libc::c_int
    {
        _freed_pool_reset(&mut *freed_pattern_pool.as_mut_ptr().offset(i as isize));
        i += 1;
    }
}
unsafe extern "C" fn _cairo_debug_print_surface_pattern(
    mut file: *mut FILE,
    mut pattern: *const cairo_surface_pattern_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    match (*(*pattern).surface).type_0 as libc::c_uint {
        0 => {
            s = b"image\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            s = b"pdf\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"ps\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = b"xlib\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            s = b"xcb\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            s = b"glitz\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            s = b"quartz\0" as *const u8 as *const libc::c_char;
        }
        7 => {
            s = b"win32\0" as *const u8 as *const libc::c_char;
        }
        8 => {
            s = b"beos\0" as *const u8 as *const libc::c_char;
        }
        9 => {
            s = b"directfb\0" as *const u8 as *const libc::c_char;
        }
        10 => {
            s = b"svg\0" as *const u8 as *const libc::c_char;
        }
        11 => {
            s = b"os2\0" as *const u8 as *const libc::c_char;
        }
        12 => {
            s = b"win32_printing\0" as *const u8 as *const libc::c_char;
        }
        13 => {
            s = b"quartz_image\0" as *const u8 as *const libc::c_char;
        }
        14 => {
            s = b"script\0" as *const u8 as *const libc::c_char;
        }
        15 => {
            s = b"qt\0" as *const u8 as *const libc::c_char;
        }
        16 => {
            s = b"recording\0" as *const u8 as *const libc::c_char;
        }
        17 => {
            s = b"vg\0" as *const u8 as *const libc::c_char;
        }
        18 => {
            s = b"gl\0" as *const u8 as *const libc::c_char;
        }
        19 => {
            s = b"drm\0" as *const u8 as *const libc::c_char;
        }
        20 => {
            s = b"tee\0" as *const u8 as *const libc::c_char;
        }
        21 => {
            s = b"xml\0" as *const u8 as *const libc::c_char;
        }
        22 => {
            s = b"skia\0" as *const u8 as *const libc::c_char;
        }
        23 => {
            s = b"subsurface\0" as *const u8 as *const libc::c_char;
        }
        24 => {
            s = b"cogl\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            s = b"invalid\0" as *const u8 as *const libc::c_char;
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    4706 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 81],
                        &[libc::c_char; 81],
                    >(
                        b"void _cairo_debug_print_surface_pattern(FILE *, const cairo_surface_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    fprintf(file, b"  surface type: %s\n\0" as *const u8 as *const libc::c_char, s);
}
unsafe extern "C" fn _cairo_debug_print_raster_source_pattern(
    mut file: *mut FILE,
    mut raster: *const cairo_raster_source_pattern_t,
) {
    fprintf(
        file,
        b"  content: %x, size %dx%d\n\0" as *const u8 as *const libc::c_char,
        (*raster).content as libc::c_uint,
        (*raster).extents.width,
        (*raster).extents.height,
    );
}
unsafe extern "C" fn _cairo_debug_print_linear_pattern(
    mut file: *mut FILE,
    mut pattern: *const cairo_linear_pattern_t,
) {}
unsafe extern "C" fn _cairo_debug_print_radial_pattern(
    mut file: *mut FILE,
    mut pattern: *const cairo_radial_pattern_t,
) {}
unsafe extern "C" fn _cairo_debug_print_mesh_pattern(
    mut file: *mut FILE,
    mut pattern: *const cairo_mesh_pattern_t,
) {}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_pattern(
    mut file: *mut FILE,
    mut pattern: *const cairo_pattern_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            s = b"solid\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            s = b"surface\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"linear\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = b"radial\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            s = b"mesh\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            s = b"raster\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            s = b"invalid\0" as *const u8 as *const libc::c_char;
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    4747 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"void _cairo_debug_print_pattern(FILE *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    fprintf(file, b"pattern: %s\n\0" as *const u8 as *const libc::c_char, s);
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        return;
    }
    match (*pattern).extend as libc::c_uint {
        0 => {
            s = b"none\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            s = b"repeat\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"reflect\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = b"pad\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            s = b"invalid\0" as *const u8 as *const libc::c_char;
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    4759 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"void _cairo_debug_print_pattern(FILE *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    fprintf(file, b"  extend: %s\n\0" as *const u8 as *const libc::c_char, s);
    match (*pattern).filter as libc::c_uint {
        0 => {
            s = b"fast\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            s = b"good\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"best\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = b"nearest\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            s = b"bilinear\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            s = b"gaussian\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            s = b"invalid\0" as *const u8 as *const libc::c_char;
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pattern.c\0" as *const u8 as *const libc::c_char,
                    4770 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"void _cairo_debug_print_pattern(FILE *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    fprintf(file, b"  filter: %s\n\0" as *const u8 as *const libc::c_char, s);
    fprintf(
        file,
        b"  matrix: [%g %g %g %g %g %g]\n\0" as *const u8 as *const libc::c_char,
        (*pattern).matrix.xx,
        (*pattern).matrix.yx,
        (*pattern).matrix.xy,
        (*pattern).matrix.yy,
        (*pattern).matrix.x0,
        (*pattern).matrix.y0,
    );
    match (*pattern).type_0 as libc::c_uint {
        5 => {
            _cairo_debug_print_raster_source_pattern(
                file,
                pattern as *mut cairo_raster_source_pattern_t,
            );
        }
        1 => {
            _cairo_debug_print_surface_pattern(
                file,
                pattern as *mut cairo_surface_pattern_t,
            );
        }
        2 => {
            _cairo_debug_print_linear_pattern(
                file,
                pattern as *mut cairo_linear_pattern_t,
            );
        }
        3 => {
            _cairo_debug_print_radial_pattern(
                file,
                pattern as *mut cairo_radial_pattern_t,
            );
        }
        4 => {
            _cairo_debug_print_mesh_pattern(file, pattern as *mut cairo_mesh_pattern_t);
        }
        0 | _ => {}
    };
}
