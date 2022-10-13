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
    fn free(_: *mut libc::c_void);
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _cairo_image_surface_coerce_to_format(
        surface: *mut cairo_image_surface_t,
        format: cairo_format_t,
    ) -> *mut cairo_image_surface_t;
    fn cairo_glyph_allocate(num_glyphs: libc::c_int) -> *mut cairo_glyph_t;
    fn cairo_glyph_free(glyphs: *mut cairo_glyph_t);
    fn cairo_text_cluster_allocate(
        num_clusters: libc::c_int,
    ) -> *mut cairo_text_cluster_t;
    fn cairo_text_cluster_free(clusters: *mut cairo_text_cluster_t);
    fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
    fn cairo_font_options_equal(
        options: *const cairo_font_options_t,
        other: *const cairo_font_options_t,
    ) -> cairo_bool_t;
    fn cairo_font_options_hash(options: *const cairo_font_options_t) -> libc::c_ulong;
    fn cairo_font_face_reference(
        font_face: *mut cairo_font_face_t,
    ) -> *mut cairo_font_face_t;
    fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    static mut _cairo_scaled_font_error_mutex: cairo_mutex_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_user_data_array_fini(array: *mut cairo_user_data_array_t);
    fn _cairo_matrix_compute_basis_scale_factors(
        matrix: *const cairo_matrix_t,
        sx: *mut libc::c_double,
        sy: *mut libc::c_double,
        x_major: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_matrix_is_scale_0(matrix: *const cairo_matrix_t) -> cairo_bool_t;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_path_fixed_destroy(path: *mut cairo_path_fixed_t);
    fn _cairo_image_scaled_glyph_fini(
        scaled_font: *mut cairo_scaled_font_t,
        scaled_glyph: *mut cairo_scaled_glyph_t,
    );
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    static mut _cairo_scaled_glyph_page_cache_mutex: cairo_mutex_t;
    static mut _cairo_scaled_font_map_mutex: cairo_mutex_t;
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn _cairo_font_face_set_error(
        font_face: *mut cairo_font_face_t,
        status: cairo_status_t,
    ) -> cairo_status_t;
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_path_fixed_close_path(path: *mut cairo_path_fixed_t) -> cairo_status_t;
    fn _cairo_font_options_init_copy(
        options: *mut cairo_font_options_t,
        other: *const cairo_font_options_t,
    );
    fn _cairo_matrix_compute_determinant(
        matrix: *const cairo_matrix_t,
    ) -> libc::c_double;
    fn _cairo_path_fixed_rel_line_to(
        path: *mut cairo_path_fixed_t,
        dx: cairo_fixed_t,
        dy: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_user_data_array_get_data(
        array: *mut cairo_user_data_array_t,
        key: *const cairo_user_data_key_t,
    ) -> *mut libc::c_void;
    fn _cairo_path_fixed_move_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_user_data_array_set_data(
        array: *mut cairo_user_data_array_t,
        key: *const cairo_user_data_key_t,
        user_data: *mut libc::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    fn _cairo_cache_thaw(cache: *mut cairo_cache_t);
    fn _cairo_color_equal(
        color_a: *const cairo_color_t,
        color_b: *const cairo_color_t,
    ) -> cairo_bool_t;
    fn _cairo_cache_remove(cache: *mut cairo_cache_t, entry: *mut cairo_cache_entry_t);
    fn _cairo_font_options_get_round_glyph_positions(
        options: *const cairo_font_options_t,
    ) -> cairo_round_glyph_positions_t;
    fn _cairo_cache_insert(
        cache: *mut cairo_cache_t,
        entry: *mut cairo_cache_entry_t,
    ) -> cairo_status_t;
    fn _cairo_cache_freeze(cache: *mut cairo_cache_t);
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_cache_init(
        cache: *mut cairo_cache_t,
        keys_equal: cairo_cache_keys_equal_func_t,
        predicate: cairo_cache_predicate_func_t,
        entry_destroy: cairo_destroy_func_t,
        max_size: libc::c_ulong,
    ) -> cairo_status_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_utf8_get_char_validated(
        p: *const libc::c_char,
        unicode: *mut uint32_t,
    ) -> libc::c_int;
    fn _cairo_validate_text_clusters(
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        glyphs: *const cairo_glyph_t,
        num_glyphs: libc::c_int,
        clusters: *const cairo_text_cluster_t,
        num_clusters: libc::c_int,
        cluster_flags: cairo_text_cluster_flags_t,
    ) -> cairo_status_t;
    fn _cairo_utf8_to_ucs4(
        str: *const libc::c_char,
        len: libc::c_int,
        result: *mut *mut uint32_t,
        items_written: *mut libc::c_int,
    ) -> cairo_status_t;
    static _cairo_font_face_nil: cairo_font_face_t;
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn cairo_surface_get_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: *mut libc::c_double,
        y_offset: *mut libc::c_double,
    );
    fn cairo_matrix_init(
        matrix: *mut cairo_matrix_t,
        xx: libc::c_double,
        yx: libc::c_double,
        xy: libc::c_double,
        yy: libc::c_double,
        x0: libc::c_double,
        y0: libc::c_double,
    );
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_cache_fini(cache: *mut cairo_cache_t);
    fn _cairo_path_fixed_append(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
        tx: cairo_fixed_t,
        ty: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn _cairo_user_data_array_init(array: *mut cairo_user_data_array_t);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_truncate(array: *mut cairo_array_t, num_elements: libc::c_uint);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_copy_element(
        array: *const cairo_array_t,
        index: libc::c_uint,
        dst: *mut libc::c_void,
    );
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type cairo_scaled_font_private_t = _cairo_scaled_font_private;
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
pub type cairo_scaled_glyph_page_t = _cairo_scaled_glyph_page;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_glyph_page {
    pub cache_entry: cairo_cache_entry_t,
    pub scaled_font: *mut cairo_scaled_font_t,
    pub link: cairo_list_t,
    pub num_glyphs: libc::c_uint,
    pub glyphs: [cairo_scaled_glyph_t; 32],
}
pub type cairo_cache_entry_t = _cairo_cache_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cache_entry {
    pub hash: uintptr_t,
    pub size: libc::c_ulong,
}
pub type cairo_scaled_glyph_private_t = _cairo_scaled_glyph_private;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_glyph_private {
    pub link: cairo_list_t,
    pub key: *const libc::c_void,
    pub destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_glyph_private_t,
            *mut cairo_scaled_glyph_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
}
pub type cairo_cache_t = _cairo_cache;
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
pub type cairo_scaled_font_map_t = _cairo_scaled_font_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_map {
    pub mru_scaled_font: *mut cairo_scaled_font_t,
    pub hash_table: *mut cairo_hash_table_t,
    pub holdovers: [*mut cairo_scaled_font_t; 256],
    pub num_holdovers: libc::c_int,
}
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type uint64_t = __uint64_t;
pub type uint8_t = __uint8_t;
pub type cairo_cache_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyph_lut_elt {
    pub index: libc::c_ulong,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_double(mut d: libc::c_double) -> cairo_fixed_t {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { d: 0. };
    u
        .d = d
        + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
            as libc::c_double * 1.5f64;
    return u.i[0 as libc::c_int as usize];
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
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
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
unsafe extern "C" fn cairo_list_add_tail(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, (*head).prev, head);
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
unsafe extern "C" fn cairo_list_move(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_del((*entry).prev, (*entry).next);
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
static mut cairo_scaled_glyph_page_cache: cairo_cache_t = cairo_cache_t {
    hash_table: 0 as *const cairo_hash_table_t as *mut cairo_hash_table_t,
    predicate: None,
    entry_destroy: None,
    max_size: 0,
    size: 0,
    freeze_count: 0,
};
unsafe extern "C" fn _cairo_scaled_glyph_fini(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
) {
    while cairo_list_is_empty(&mut (*scaled_glyph).dev_privates) == 0 {
        let mut private: *mut cairo_scaled_glyph_private_t = ({
            let mut mptr__: *const cairo_list_t = (*scaled_glyph).dev_privates.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_scaled_glyph_private_t
        });
        ((*private).destroy)
            .expect("non-null function pointer")(private, scaled_glyph, scaled_font);
    }
    _cairo_image_scaled_glyph_fini(scaled_font, scaled_glyph);
    if !((*scaled_glyph).surface).is_null() {
        cairo_surface_destroy(&mut (*(*scaled_glyph).surface).base);
    }
    if !((*scaled_glyph).path).is_null() {
        _cairo_path_fixed_destroy((*scaled_glyph).path);
    }
    if !((*scaled_glyph).recording_surface).is_null() {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_array_append(
            &mut (*scaled_font).recording_surfaces_to_free,
            &mut (*scaled_glyph).recording_surface as *mut *mut cairo_surface_t
                as *const libc::c_void,
        );
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
                229 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"void _cairo_scaled_glyph_fini(cairo_scaled_font_t *, cairo_scaled_glyph_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if !((*scaled_glyph).color_surface).is_null() {
        cairo_surface_destroy(&mut (*(*scaled_glyph).color_surface).base);
    }
}
static mut _cairo_scaled_font_nil: cairo_scaled_font_t = cairo_scaled_font_t {
    hash_entry: cairo_hash_entry_t { hash: 0 },
    status: CAIRO_STATUS_SUCCESS,
    ref_count: cairo_reference_count_t {
        ref_count: 0,
    },
    user_data: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    original_font_face: 0 as *const cairo_font_face_t as *mut cairo_font_face_t,
    font_face: 0 as *const cairo_font_face_t as *mut cairo_font_face_t,
    font_matrix: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    ctm: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    options: cairo_font_options_t {
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
    placeholder_holdover_finished: [0; 1],
    c2rust_padding: [0; 7],
    scale: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    scale_inverse: cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    },
    max_scale: 0.,
    extents: cairo_font_extents_t {
        ascent: 0.,
        descent: 0.,
        height: 0.,
        max_x_advance: 0.,
        max_y_advance: 0.,
    },
    fs_extents: cairo_font_extents_t {
        ascent: 0.,
        descent: 0.,
        height: 0.,
        max_x_advance: 0.,
        max_y_advance: 0.,
    },
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    glyphs: 0 as *const cairo_hash_table_t as *mut cairo_hash_table_t,
    glyph_pages: cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    },
    cache_frozen: 0,
    global_cache_frozen: 0,
    recording_surfaces_to_free: cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    },
    dev_privates: cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    },
    backend: 0 as *const cairo_scaled_font_backend_t,
    link: cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    },
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_set_error(
    mut scaled_font: *mut cairo_scaled_font_t,
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
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"cairo_status_t _cairo_scaled_font_set_error(cairo_scaled_font_t *, cairo_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    ret__ = _cairo_atomic_int_cmpxchg_impl(
        &mut (*scaled_font).status as *mut cairo_status_t as *mut cairo_atomic_int_t,
        CAIRO_STATUS_SUCCESS as libc::c_int,
        status as cairo_atomic_int_t,
    );
    return _cairo_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_type(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_font_type_t {
    if _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return CAIRO_FONT_TYPE_TOY;
    }
    return (*(*scaled_font).backend).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_status(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_status_t {
    return (*scaled_font).status;
}
static mut cairo_scaled_font_map: *mut cairo_scaled_font_map_t = 0
    as *const cairo_scaled_font_map_t as *mut cairo_scaled_font_map_t;
unsafe extern "C" fn _cairo_scaled_font_map_lock() -> *mut cairo_scaled_font_map_t {
    let mut current_block: u64;
    pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
    if cairo_scaled_font_map.is_null() {
        cairo_scaled_font_map = (if ::std::mem::size_of::<cairo_scaled_font_map_t>()
            as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_scaled_font_map_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_scaled_font_map_t;
        if cairo_scaled_font_map.is_null() {
            current_block = 13034768034686864816;
        } else {
            let ref mut fresh9 = (*cairo_scaled_font_map).mru_scaled_font;
            *fresh9 = 0 as *mut cairo_scaled_font_t;
            let ref mut fresh10 = (*cairo_scaled_font_map).hash_table;
            *fresh10 = _cairo_hash_table_create(
                Some(
                    _cairo_scaled_font_keys_equal
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if ((*cairo_scaled_font_map).hash_table).is_null() {
                free(cairo_scaled_font_map as *mut libc::c_void);
                cairo_scaled_font_map = 0 as *mut cairo_scaled_font_map_t;
                current_block = 13034768034686864816;
            } else {
                (*cairo_scaled_font_map).num_holdovers = 0 as libc::c_int;
                current_block = 6937071982253665452;
            }
        }
        match current_block {
            6937071982253665452 => {}
            _ => {
                pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
                let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                return 0 as *mut cairo_scaled_font_map_t;
            }
        }
    }
    return cairo_scaled_font_map;
}
unsafe extern "C" fn _cairo_scaled_font_map_unlock() {
    pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_map_destroy() {
    let mut font_map: *mut cairo_scaled_font_map_t = 0 as *mut cairo_scaled_font_map_t;
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
    font_map = cairo_scaled_font_map;
    if !font_map.is_null() {
        scaled_font = (*font_map).mru_scaled_font;
        if !scaled_font.is_null() {
            pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
            cairo_scaled_font_destroy(scaled_font);
            pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
        }
        while (*font_map).num_holdovers != 0 {
            scaled_font = (*font_map)
                .holdovers[((*font_map).num_holdovers - 1 as libc::c_int) as usize];
            if !(_cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
                > 0 as libc::c_int)
            {} else {
                __assert_fail(
                    b"! CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&scaled_font->ref_count)\0"
                        as *const u8 as *const libc::c_char,
                    b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
                    437 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"void _cairo_scaled_font_map_destroy(void)\0"))
                        .as_ptr(),
                );
            }
            _cairo_hash_table_remove(
                (*font_map).hash_table,
                &mut (*scaled_font).hash_entry,
            );
            let ref mut fresh11 = (*font_map).num_holdovers;
            *fresh11 -= 1;
            _cairo_scaled_font_fini(scaled_font);
            free(scaled_font as *mut libc::c_void);
        }
        _cairo_hash_table_destroy((*font_map).hash_table);
        free(cairo_scaled_font_map as *mut libc::c_void);
        cairo_scaled_font_map = 0 as *mut cairo_scaled_font_map_t;
    }
    pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
}
unsafe extern "C" fn _cairo_scaled_glyph_page_destroy(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut page: *mut cairo_scaled_glyph_page_t,
) {
    let mut n: libc::c_uint = 0;
    if (*scaled_font).cache_frozen == 0 {} else {
        __assert_fail(
            b"!scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void _cairo_scaled_glyph_page_destroy(cairo_scaled_font_t *, cairo_scaled_glyph_page_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*scaled_font).global_cache_frozen == 0 {} else {
        __assert_fail(
            b"!scaled_font->global_cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            468 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void _cairo_scaled_glyph_page_destroy(cairo_scaled_font_t *, cairo_scaled_glyph_page_t *)\0",
            ))
                .as_ptr(),
        );
    }
    n = 0 as libc::c_int as libc::c_uint;
    while n < (*page).num_glyphs {
        _cairo_hash_table_remove(
            (*scaled_font).glyphs,
            &mut (*((*page).glyphs).as_mut_ptr().offset(n as isize)).hash_entry,
        );
        _cairo_scaled_glyph_fini(
            scaled_font,
            &mut *((*page).glyphs).as_mut_ptr().offset(n as isize),
        );
        n = n.wrapping_add(1);
    }
    cairo_list_del(&mut (*page).link);
    free(page as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_scaled_glyph_page_pluck(mut closure: *mut libc::c_void) {
    let mut page: *mut cairo_scaled_glyph_page_t = closure
        as *mut cairo_scaled_glyph_page_t;
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    if cairo_list_is_empty(&mut (*page).link) == 0 {} else {
        __assert_fail(
            b"! cairo_list_is_empty (&page->link)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void _cairo_scaled_glyph_page_pluck(void *)\0"))
                .as_ptr(),
        );
    }
    scaled_font = (*page).scaled_font;
    _cairo_scaled_glyph_page_destroy(scaled_font, page);
    pthread_mutex_unlock(&mut (*scaled_font).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_register_placeholder_and_unlock_font_map(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut placeholder_scaled_font: *mut cairo_scaled_font_t = 0
        as *mut cairo_scaled_font_t;
    status = (*scaled_font).status;
    if status as u64 != 0 {
        return status;
    }
    placeholder_scaled_font = (if ::std::mem::size_of::<cairo_scaled_font_t>()
        as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_scaled_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_scaled_font_t;
    if placeholder_scaled_font.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _cairo_scaled_font_init(
        placeholder_scaled_font,
        (*scaled_font).font_face,
        &mut (*scaled_font).font_matrix,
        &mut (*scaled_font).ctm,
        &mut (*scaled_font).options,
        0 as *const cairo_scaled_font_backend_t,
    );
    if !(status as u64 != 0) {
        (*placeholder_scaled_font).set_placeholder(1 as libc::c_int as libc::c_uint);
        (*placeholder_scaled_font)
            .hash_entry
            .hash = _cairo_scaled_font_compute_hash(placeholder_scaled_font);
        status = _cairo_hash_table_insert(
            (*cairo_scaled_font_map).hash_table,
            &mut (*placeholder_scaled_font).hash_entry,
        );
        if status as u64 != 0 {
            _cairo_scaled_font_fini_internal(placeholder_scaled_font);
        } else {
            pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
            pthread_mutex_lock(&mut (*placeholder_scaled_font).mutex);
            return CAIRO_STATUS_SUCCESS;
        }
    }
    free(placeholder_scaled_font as *mut libc::c_void);
    return _cairo_scaled_font_set_error(scaled_font, status);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_unregister_placeholder_and_lock_font_map(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    let mut placeholder_scaled_font: *mut cairo_scaled_font_t = 0
        as *mut cairo_scaled_font_t;
    pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
    (*scaled_font).hash_entry.hash = _cairo_scaled_font_compute_hash(scaled_font);
    placeholder_scaled_font = _cairo_hash_table_lookup(
        (*cairo_scaled_font_map).hash_table,
        &mut (*scaled_font).hash_entry,
    ) as *mut cairo_scaled_font_t;
    if !placeholder_scaled_font.is_null() {} else {
        __assert_fail(
            b"placeholder_scaled_font != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            572 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"void _cairo_scaled_font_unregister_placeholder_and_lock_font_map(cairo_scaled_font_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*placeholder_scaled_font).placeholder() != 0 {} else {
        __assert_fail(
            b"placeholder_scaled_font->placeholder\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            573 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"void _cairo_scaled_font_unregister_placeholder_and_lock_font_map(cairo_scaled_font_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_hash_table_remove(
        (*cairo_scaled_font_map).hash_table,
        &mut (*placeholder_scaled_font).hash_entry,
    );
    pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
    pthread_mutex_unlock(&mut (*placeholder_scaled_font).mutex);
    cairo_scaled_font_destroy(placeholder_scaled_font);
    pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
}
unsafe extern "C" fn _cairo_scaled_font_placeholder_wait_for_creation_to_finish(
    mut placeholder_scaled_font: *mut cairo_scaled_font_t,
) {
    cairo_scaled_font_reference(placeholder_scaled_font);
    pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
    pthread_mutex_lock(&mut (*placeholder_scaled_font).mutex);
    pthread_mutex_unlock(&mut (*placeholder_scaled_font).mutex);
    cairo_scaled_font_destroy(placeholder_scaled_font);
    pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
}
unsafe extern "C" fn _hash_matrix_fnv(
    mut matrix: *const cairo_matrix_t,
    mut hval: uint64_t,
) -> uint64_t {
    let mut buffer: *const uint8_t = matrix as *const uint8_t;
    let mut len: libc::c_int = ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong
        as libc::c_int;
    loop {
        hval = (hval as libc::c_ulong)
            .wrapping_mul(0x100000001b3 as libc::c_long as uint64_t) as uint64_t
            as uint64_t;
        let fresh12 = buffer;
        buffer = buffer.offset(1);
        hval ^= *fresh12 as libc::c_ulong;
        len -= 1;
        if !(len != 0) {
            break;
        }
    }
    return hval;
}
unsafe extern "C" fn _hash_mix_bits(mut hash: uint64_t) -> uint64_t {
    hash = (hash as libc::c_ulong).wrapping_add(hash << 12 as libc::c_int) as uint64_t
        as uint64_t;
    hash ^= hash >> 7 as libc::c_int;
    hash = (hash as libc::c_ulong).wrapping_add(hash << 3 as libc::c_int) as uint64_t
        as uint64_t;
    hash ^= hash >> 17 as libc::c_int;
    hash = (hash as libc::c_ulong).wrapping_add(hash << 5 as libc::c_int) as uint64_t
        as uint64_t;
    return hash;
}
unsafe extern "C" fn _cairo_scaled_font_compute_hash(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> uintptr_t {
    let mut hash: uint64_t = 0xcbf29ce484222325 as libc::c_ulong;
    hash = _hash_matrix_fnv(&mut (*scaled_font).font_matrix, hash);
    hash = _hash_matrix_fnv(&mut (*scaled_font).ctm, hash);
    hash = _hash_mix_bits(hash);
    hash ^= (*scaled_font).original_font_face as uintptr_t;
    hash ^= cairo_font_options_hash(&mut (*scaled_font).options);
    hash = _hash_mix_bits(hash);
    if hash != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"hash != ZOMBIE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            655 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"uintptr_t _cairo_scaled_font_compute_hash(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    return hash;
}
unsafe extern "C" fn _cairo_scaled_font_init_key(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_face: *mut cairo_font_face_t,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
) {
    (*scaled_font).status = CAIRO_STATUS_SUCCESS;
    (*scaled_font).set_placeholder(0 as libc::c_int as libc::c_uint);
    let ref mut fresh13 = (*scaled_font).font_face;
    *fresh13 = font_face;
    let ref mut fresh14 = (*scaled_font).original_font_face;
    *fresh14 = font_face;
    (*scaled_font).font_matrix = *font_matrix;
    (*scaled_font).ctm = *ctm;
    (*scaled_font).ctm.x0 = 0.0f64;
    (*scaled_font).ctm.y0 = 0.0f64;
    _cairo_font_options_init_copy(&mut (*scaled_font).options, options);
    (*scaled_font).hash_entry.hash = _cairo_scaled_font_compute_hash(scaled_font);
}
unsafe extern "C" fn _cairo_scaled_font_keys_equal(
    mut abstract_key_a: *const libc::c_void,
    mut abstract_key_b: *const libc::c_void,
) -> libc::c_int {
    let mut key_a: *const cairo_scaled_font_t = abstract_key_a
        as *const cairo_scaled_font_t;
    let mut key_b: *const cairo_scaled_font_t = abstract_key_b
        as *const cairo_scaled_font_t;
    return ((*key_a).original_font_face == (*key_b).original_font_face
        && memcmp(
            &(*key_a).font_matrix.xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            &(*key_b).font_matrix.xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
        ) == 0 as libc::c_int
        && memcmp(
            &(*key_a).ctm.xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            &(*key_b).ctm.xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
        ) == 0 as libc::c_int
        && cairo_font_options_equal(&(*key_a).options, &(*key_b).options) != 0)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_scaled_font_matches(
    mut scaled_font: *const cairo_scaled_font_t,
    mut font_face: *const cairo_font_face_t,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
) -> cairo_bool_t {
    return ((*scaled_font).original_font_face == font_face as *mut cairo_font_face_t
        && memcmp(
            &(*scaled_font).font_matrix.xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            &(*font_matrix).xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
        ) == 0 as libc::c_int
        && memcmp(
            &(*scaled_font).ctm.xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            &(*ctm).xx as *const libc::c_double as *mut libc::c_uchar
                as *const libc::c_void,
            ::std::mem::size_of::<cairo_matrix_t>() as libc::c_ulong,
        ) == 0 as libc::c_int
        && cairo_font_options_equal(&(*scaled_font).options, options) != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_init(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_face: *mut cairo_font_face_t,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
    mut backend: *const cairo_scaled_font_backend_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = cairo_font_options_status(options as *mut cairo_font_options_t);
    if status as u64 != 0 {
        return status;
    }
    (*scaled_font).status = CAIRO_STATUS_SUCCESS;
    (*scaled_font).set_placeholder(0 as libc::c_int as libc::c_uint);
    let ref mut fresh15 = (*scaled_font).font_face;
    *fresh15 = font_face;
    let ref mut fresh16 = (*scaled_font).original_font_face;
    *fresh16 = font_face;
    (*scaled_font).font_matrix = *font_matrix;
    (*scaled_font).ctm = *ctm;
    (*scaled_font).ctm.x0 = 0.0f64;
    (*scaled_font).ctm.y0 = 0.0f64;
    _cairo_font_options_init_copy(&mut (*scaled_font).options, options);
    cairo_matrix_multiply(
        &mut (*scaled_font).scale,
        &mut (*scaled_font).font_matrix,
        &mut (*scaled_font).ctm,
    );
    (*scaled_font)
        .max_scale = if fabs((*scaled_font).scale.xx) + fabs((*scaled_font).scale.xy)
        > fabs((*scaled_font).scale.yx) + fabs((*scaled_font).scale.yy)
    {
        fabs((*scaled_font).scale.xx) + fabs((*scaled_font).scale.xy)
    } else {
        fabs((*scaled_font).scale.yx) + fabs((*scaled_font).scale.yy)
    };
    (*scaled_font).scale_inverse = (*scaled_font).scale;
    status = cairo_matrix_invert(&mut (*scaled_font).scale_inverse);
    if status as u64 != 0 {
        if _cairo_matrix_is_scale_0(&mut (*scaled_font).scale) != 0 {
            cairo_matrix_init(
                &mut (*scaled_font).scale_inverse,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                -(*scaled_font).scale.x0,
                -(*scaled_font).scale.y0,
            );
        } else {
            return status
        }
    }
    let ref mut fresh17 = (*scaled_font).glyphs;
    *fresh17 = _cairo_hash_table_create(None);
    if ((*scaled_font).glyphs).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    cairo_list_init(&mut (*scaled_font).glyph_pages);
    (*scaled_font).cache_frozen = 0 as libc::c_int;
    (*scaled_font).global_cache_frozen = 0 as libc::c_int;
    _cairo_array_init(
        &mut (*scaled_font).recording_surfaces_to_free,
        ::std::mem::size_of::<*mut cairo_surface_t>() as libc::c_ulong as libc::c_uint,
    );
    (*scaled_font).set_holdover(0 as libc::c_int as libc::c_uint);
    (*scaled_font).set_finished(0 as libc::c_int as libc::c_uint);
    (*scaled_font).ref_count.ref_count = 1 as libc::c_int;
    _cairo_user_data_array_init(&mut (*scaled_font).user_data);
    cairo_font_face_reference(font_face);
    let ref mut fresh18 = (*scaled_font).original_font_face;
    *fresh18 = 0 as *mut cairo_font_face_t;
    let mut attr: pthread_mutexattr_t = pthread_mutexattr_t {
        __size: [0; 4],
    };
    pthread_mutexattr_init(&mut attr);
    pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE as libc::c_int);
    pthread_mutex_init(&mut (*scaled_font).mutex, &mut attr);
    pthread_mutexattr_destroy(&mut attr);
    cairo_list_init(&mut (*scaled_font).dev_privates);
    let ref mut fresh19 = (*scaled_font).backend;
    *fresh19 = backend;
    cairo_list_init(&mut (*scaled_font).link);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_scaled_font_free_recording_surfaces(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    let mut num_recording_surfaces: libc::c_int = 0;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    num_recording_surfaces = _cairo_array_num_elements(
        &mut (*scaled_font).recording_surfaces_to_free,
    ) as libc::c_int;
    if num_recording_surfaces > 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < num_recording_surfaces {
            _cairo_array_copy_element(
                &mut (*scaled_font).recording_surfaces_to_free,
                i as libc::c_uint,
                &mut surface as *mut *mut cairo_surface_t as *mut libc::c_void,
            );
            cairo_surface_finish(surface);
            cairo_surface_destroy(surface);
            i += 1;
        }
        _cairo_array_truncate(
            &mut (*scaled_font).recording_surfaces_to_free,
            0 as libc::c_int as libc::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_freeze_cache(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    if (*scaled_font).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"scaled_font->status == CAIRO_STATUS_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            821 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void _cairo_scaled_font_freeze_cache(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    pthread_mutex_lock(&mut (*scaled_font).mutex);
    (*scaled_font).cache_frozen = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_thaw_cache(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    if (*scaled_font).cache_frozen != 0 {} else {
        __assert_fail(
            b"scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            830 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void _cairo_scaled_font_thaw_cache(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    if (*scaled_font).global_cache_frozen != 0 {
        pthread_mutex_lock(&mut _cairo_scaled_glyph_page_cache_mutex);
        _cairo_cache_thaw(&mut cairo_scaled_glyph_page_cache);
        pthread_mutex_unlock(&mut _cairo_scaled_glyph_page_cache_mutex);
        (*scaled_font).global_cache_frozen = 0 as libc::c_int;
    }
    _cairo_scaled_font_free_recording_surfaces(scaled_font);
    (*scaled_font).cache_frozen = 0 as libc::c_int;
    pthread_mutex_unlock(&mut (*scaled_font).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_reset_cache(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    let mut page: *mut cairo_scaled_glyph_page_t = 0 as *mut cairo_scaled_glyph_page_t;
    pthread_mutex_lock(&mut (*scaled_font).mutex);
    if (*scaled_font).cache_frozen == 0 {} else {
        __assert_fail(
            b"! scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            851 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void _cairo_scaled_font_reset_cache(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    if (*scaled_font).global_cache_frozen == 0 {} else {
        __assert_fail(
            b"! scaled_font->global_cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            852 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void _cairo_scaled_font_reset_cache(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    pthread_mutex_lock(&mut _cairo_scaled_glyph_page_cache_mutex);
    page = ({
        let mut mptr__: *const cairo_list_t = (*scaled_font).glyph_pages.next;
        (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
            as *mut cairo_scaled_glyph_page_t
    });
    while &mut (*page).link as *mut cairo_list_t
        != &mut (*scaled_font).glyph_pages as *mut cairo_list_t
    {
        cairo_scaled_glyph_page_cache
            .size = (cairo_scaled_glyph_page_cache.size)
            .wrapping_sub((*page).cache_entry.size);
        _cairo_hash_table_remove(
            cairo_scaled_glyph_page_cache.hash_table,
            &mut (*page).cache_entry as *mut cairo_cache_entry_t
                as *mut cairo_hash_entry_t,
        );
        page = ({
            let mut mptr__: *const cairo_list_t = (*page).link.next;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut cairo_scaled_glyph_page_t
        });
    }
    pthread_mutex_unlock(&mut _cairo_scaled_glyph_page_cache_mutex);
    while cairo_list_is_empty(&mut (*scaled_font).glyph_pages) == 0 {
        page = ({
            let mut mptr__: *const cairo_list_t = (*scaled_font).glyph_pages.next;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut cairo_scaled_glyph_page_t
        });
        _cairo_scaled_glyph_page_destroy(scaled_font, page);
    }
    pthread_mutex_unlock(&mut (*scaled_font).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_set_metrics(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut fs_metrics: *mut cairo_font_extents_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut font_scale_x: libc::c_double = 0.;
    let mut font_scale_y: libc::c_double = 0.;
    (*scaled_font).fs_extents = *fs_metrics;
    status = _cairo_matrix_compute_basis_scale_factors(
        &mut (*scaled_font).font_matrix,
        &mut font_scale_x,
        &mut font_scale_y,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    (*scaled_font).extents.ascent = (*fs_metrics).ascent * font_scale_y;
    (*scaled_font).extents.descent = (*fs_metrics).descent * font_scale_y;
    (*scaled_font).extents.height = (*fs_metrics).height * font_scale_y;
    (*scaled_font).extents.max_x_advance = (*fs_metrics).max_x_advance * font_scale_x;
    (*scaled_font).extents.max_y_advance = (*fs_metrics).max_y_advance * font_scale_y;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_scaled_font_fini_internal(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    if (*scaled_font).cache_frozen == 0 {} else {
        __assert_fail(
            b"! scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            912 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void _cairo_scaled_font_fini_internal(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    if (*scaled_font).global_cache_frozen == 0 {} else {
        __assert_fail(
            b"! scaled_font->global_cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            913 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void _cairo_scaled_font_fini_internal(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    (*scaled_font).set_finished(1 as libc::c_int as libc::c_uint);
    _cairo_scaled_font_reset_cache(scaled_font);
    _cairo_hash_table_destroy((*scaled_font).glyphs);
    cairo_font_face_destroy((*scaled_font).font_face);
    cairo_font_face_destroy((*scaled_font).original_font_face);
    _cairo_scaled_font_free_recording_surfaces(scaled_font);
    _cairo_array_fini(&mut (*scaled_font).recording_surfaces_to_free);
    pthread_mutex_destroy(&mut (*scaled_font).mutex);
    while cairo_list_is_empty(&mut (*scaled_font).dev_privates) == 0 {
        let mut private: *mut cairo_scaled_font_private_t = ({
            let mut mptr__: *const cairo_list_t = (*scaled_font).dev_privates.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_scaled_font_private_t
        });
        ((*private).destroy).expect("non-null function pointer")(private, scaled_font);
    }
    if !((*scaled_font).backend).is_null() && ((*(*scaled_font).backend).fini).is_some()
    {
        ((*(*scaled_font).backend).fini)
            .expect("non-null function pointer")(scaled_font as *mut libc::c_void);
    }
    _cairo_user_data_array_fini(&mut (*scaled_font).user_data);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_fini(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    pthread_mutex_unlock(&mut _cairo_scaled_font_map_mutex);
    _cairo_scaled_font_fini_internal(scaled_font);
    pthread_mutex_lock(&mut _cairo_scaled_font_map_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_attach_private(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut private: *mut cairo_scaled_font_private_t,
    mut key: *const libc::c_void,
    mut destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_font_private_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
) {
    let ref mut fresh20 = (*private).key;
    *fresh20 = key;
    let ref mut fresh21 = (*private).destroy;
    *fresh21 = destroy;
    cairo_list_add(&mut (*private).link, &mut (*scaled_font).dev_privates);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_find_private(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut key: *const libc::c_void,
) -> *mut cairo_scaled_font_private_t {
    let mut priv_0: *mut cairo_scaled_font_private_t = 0
        as *mut cairo_scaled_font_private_t;
    priv_0 = ({
        let mut mptr__: *const cairo_list_t = (*scaled_font).dev_privates.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_scaled_font_private_t
    });
    while &mut (*priv_0).link as *mut cairo_list_t
        != &mut (*scaled_font).dev_privates as *mut cairo_list_t
    {
        if (*priv_0).key == key {
            if (*priv_0).link.prev
                != &mut (*scaled_font).dev_privates as *mut cairo_list_t
            {
                cairo_list_move(&mut (*priv_0).link, &mut (*scaled_font).dev_privates);
            }
            return priv_0;
        }
        priv_0 = ({
            let mut mptr__: *const cairo_list_t = (*priv_0).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_scaled_font_private_t
        });
    }
    return 0 as *mut cairo_scaled_font_private_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_attach_private(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut private: *mut cairo_scaled_glyph_private_t,
    mut key: *const libc::c_void,
    mut destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_glyph_private_t,
            *mut cairo_scaled_glyph_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
) {
    let ref mut fresh22 = (*private).key;
    *fresh22 = key;
    let ref mut fresh23 = (*private).destroy;
    *fresh23 = destroy;
    cairo_list_add(&mut (*private).link, &mut (*scaled_glyph).dev_privates);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_find_private(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut key: *const libc::c_void,
) -> *mut cairo_scaled_glyph_private_t {
    let mut priv_0: *mut cairo_scaled_glyph_private_t = 0
        as *mut cairo_scaled_glyph_private_t;
    priv_0 = ({
        let mut mptr__: *const cairo_list_t = (*scaled_glyph).dev_privates.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_scaled_glyph_private_t
    });
    while &mut (*priv_0).link as *mut cairo_list_t
        != &mut (*scaled_glyph).dev_privates as *mut cairo_list_t
    {
        if (*priv_0).key == key {
            if (*priv_0).link.prev
                != &mut (*scaled_glyph).dev_privates as *mut cairo_list_t
            {
                cairo_list_move(&mut (*priv_0).link, &mut (*scaled_glyph).dev_privates);
            }
            return priv_0;
        }
        priv_0 = ({
            let mut mptr__: *const cairo_list_t = (*priv_0).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_scaled_glyph_private_t
        });
    }
    return 0 as *mut cairo_scaled_glyph_private_t;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_create(
    mut font_face: *mut cairo_font_face_t,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
) -> *mut cairo_scaled_font_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut font_map: *mut cairo_scaled_font_map_t = 0 as *mut cairo_scaled_font_map_t;
    let mut original_font_face: *mut cairo_font_face_t = font_face;
    let mut key: cairo_scaled_font_t = cairo_scaled_font_t {
        hash_entry: cairo_hash_entry_t { hash: 0 },
        status: CAIRO_STATUS_SUCCESS,
        ref_count: cairo_reference_count_t {
            ref_count: 0,
        },
        user_data: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
        original_font_face: 0 as *const cairo_font_face_t as *mut cairo_font_face_t,
        font_face: 0 as *const cairo_font_face_t as *mut cairo_font_face_t,
        font_matrix: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        ctm: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        options: cairo_font_options_t {
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
        placeholder_holdover_finished: [0; 1],
        c2rust_padding: [0; 7],
        scale: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        scale_inverse: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        max_scale: 0.,
        extents: cairo_font_extents_t {
            ascent: 0.,
            descent: 0.,
            height: 0.,
            max_x_advance: 0.,
            max_y_advance: 0.,
        },
        fs_extents: cairo_font_extents_t {
            ascent: 0.,
            descent: 0.,
            height: 0.,
            max_x_advance: 0.,
            max_y_advance: 0.,
        },
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                },
            },
        },
        glyphs: 0 as *const cairo_hash_table_t as *mut cairo_hash_table_t,
        glyph_pages: cairo_list_t {
            next: 0 as *mut _cairo_list,
            prev: 0 as *mut _cairo_list,
        },
        cache_frozen: 0,
        global_cache_frozen: 0,
        recording_surfaces_to_free: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
        dev_privates: cairo_list_t {
            next: 0 as *mut _cairo_list,
            prev: 0 as *mut _cairo_list,
        },
        backend: 0 as *const cairo_scaled_font_backend_t,
        link: cairo_list_t {
            next: 0 as *mut _cairo_list,
            prev: 0 as *mut _cairo_list,
        },
    };
    let mut old: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut dead: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut det: libc::c_double = 0.;
    status = (*font_face).status;
    if status as u64 != 0 {
        return _cairo_scaled_font_create_in_error(status);
    }
    det = _cairo_matrix_compute_determinant(font_matrix);
    if det.is_finite() as i32 == 0 {
        return _cairo_scaled_font_create_in_error(
            _cairo_error(CAIRO_STATUS_INVALID_MATRIX),
        );
    }
    det = _cairo_matrix_compute_determinant(ctm);
    if det.is_finite() as i32 == 0 {
        return _cairo_scaled_font_create_in_error(
            _cairo_error(CAIRO_STATUS_INVALID_MATRIX),
        );
    }
    status = cairo_font_options_status(options as *mut cairo_font_options_t);
    if status as u64 != 0 {
        return _cairo_scaled_font_create_in_error(status);
    }
    font_map = _cairo_scaled_font_map_lock();
    if font_map.is_null() {
        return _cairo_scaled_font_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    scaled_font = (*font_map).mru_scaled_font;
    if !scaled_font.is_null()
        && _cairo_scaled_font_matches(scaled_font, font_face, font_matrix, ctm, options)
            != 0
    {
        if (*scaled_font).hash_entry.hash != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"scaled_font->hash_entry.hash != ZOMBIE\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
                1076 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 145],
                    &[libc::c_char; 145],
                >(
                    b"cairo_scaled_font_t *cairo_scaled_font_create(cairo_font_face_t *, const cairo_matrix_t *, const cairo_matrix_t *, const cairo_font_options_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*scaled_font).placeholder() == 0 {} else {
            __assert_fail(
                b"! scaled_font->placeholder\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
                1077 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 145],
                    &[libc::c_char; 145],
                >(
                    b"cairo_scaled_font_t *cairo_scaled_font_create(cairo_font_face_t *, const cairo_matrix_t *, const cairo_matrix_t *, const cairo_font_options_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*scaled_font).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            ::std::intrinsics::atomic_xadd(
                &mut (*scaled_font).ref_count.ref_count,
                1 as libc::c_int,
            );
            _cairo_scaled_font_map_unlock();
            return scaled_font;
        }
        _cairo_hash_table_remove((*font_map).hash_table, &mut (*scaled_font).hash_entry);
        (*scaled_font).hash_entry.hash = 0 as libc::c_int as uintptr_t;
        dead = scaled_font;
        let ref mut fresh24 = (*font_map).mru_scaled_font;
        *fresh24 = 0 as *mut cairo_scaled_font_t;
    }
    _cairo_scaled_font_init_key(&mut key, font_face, font_matrix, ctm, options);
    loop {
        scaled_font = _cairo_hash_table_lookup(
            (*font_map).hash_table,
            &mut key.hash_entry,
        ) as *mut cairo_scaled_font_t;
        if scaled_font.is_null() {
            break;
        }
        if (*scaled_font).placeholder() == 0 {
            break;
        }
        _cairo_scaled_font_placeholder_wait_for_creation_to_finish(scaled_font);
    }
    if !scaled_font.is_null() {
        if !(_cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
            > 0 as libc::c_int)
        {
            if (*scaled_font).holdover() != 0 {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < (*font_map).num_holdovers {
                    if (*font_map).holdovers[i as usize] == scaled_font {
                        let ref mut fresh25 = (*font_map).num_holdovers;
                        *fresh25 -= 1;
                        memmove(
                            &mut *((*font_map).holdovers).as_mut_ptr().offset(i as isize)
                                as *mut *mut cairo_scaled_font_t as *mut libc::c_void,
                            &mut *((*font_map).holdovers)
                                .as_mut_ptr()
                                .offset((i + 1 as libc::c_int) as isize)
                                as *mut *mut cairo_scaled_font_t as *const libc::c_void,
                            (((*font_map).num_holdovers - i) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut cairo_scaled_font_t>()
                                        as libc::c_ulong,
                                ),
                        );
                        break;
                    } else {
                        i += 1;
                    }
                }
                (*scaled_font).set_holdover(0 as libc::c_int as libc::c_uint);
            }
            (*scaled_font).status = CAIRO_STATUS_SUCCESS;
        }
        if (*scaled_font).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            old = (*font_map).mru_scaled_font;
            let ref mut fresh26 = (*font_map).mru_scaled_font;
            *fresh26 = scaled_font;
            ::std::intrinsics::atomic_xadd(
                &mut (*scaled_font).ref_count.ref_count,
                1 as libc::c_int,
            );
            ::std::intrinsics::atomic_xadd(
                &mut (*scaled_font).ref_count.ref_count,
                1 as libc::c_int,
            );
            _cairo_scaled_font_map_unlock();
            cairo_scaled_font_destroy(old);
            if font_face != original_font_face {
                cairo_font_face_destroy(font_face);
            }
            return scaled_font;
        }
        _cairo_hash_table_remove((*font_map).hash_table, &mut (*scaled_font).hash_entry);
        (*scaled_font).hash_entry.hash = 0 as libc::c_int as uintptr_t;
    }
    if ((*(*font_face).backend).get_implementation).is_some() {
        font_face = ((*(*font_face).backend).get_implementation)
            .expect(
                "non-null function pointer",
            )(font_face as *mut libc::c_void, font_matrix, ctm, options);
        if (*font_face).status as u64 != 0 {
            _cairo_scaled_font_map_unlock();
            return _cairo_scaled_font_create_in_error((*font_face).status);
        }
    }
    status = ((*(*font_face).backend).scaled_font_create)
        .expect(
            "non-null function pointer",
        )(font_face as *mut libc::c_void, font_matrix, ctm, options, &mut scaled_font);
    if status as u64 != 0 {
        status = _cairo_font_face_set_error(font_face, status);
        _cairo_scaled_font_map_unlock();
        if font_face != original_font_face {
            cairo_font_face_destroy(font_face);
        }
        if !dead.is_null() {
            cairo_scaled_font_destroy(dead);
        }
        return _cairo_scaled_font_create_in_error(status);
    }
    if (*scaled_font).status as u64 != 0 {
        _cairo_scaled_font_map_unlock();
        if font_face != original_font_face {
            cairo_font_face_destroy(font_face);
        }
        if !dead.is_null() {
            cairo_scaled_font_destroy(dead);
        }
        return scaled_font;
    }
    if (*scaled_font).font_face == font_face {} else {
        __assert_fail(
            b"scaled_font->font_face == font_face\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 145],
                &[libc::c_char; 145],
            >(
                b"cairo_scaled_font_t *cairo_scaled_font_create(cairo_font_face_t *, const cairo_matrix_t *, const cairo_matrix_t *, const cairo_font_options_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*scaled_font).cache_frozen == 0 {} else {
        __assert_fail(
            b"! scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1208 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 145],
                &[libc::c_char; 145],
            >(
                b"cairo_scaled_font_t *cairo_scaled_font_create(cairo_font_face_t *, const cairo_matrix_t *, const cairo_matrix_t *, const cairo_font_options_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*scaled_font).global_cache_frozen == 0 {} else {
        __assert_fail(
            b"! scaled_font->global_cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 145],
                &[libc::c_char; 145],
            >(
                b"cairo_scaled_font_t *cairo_scaled_font_create(cairo_font_face_t *, const cairo_matrix_t *, const cairo_matrix_t *, const cairo_font_options_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh27 = (*scaled_font).original_font_face;
    *fresh27 = cairo_font_face_reference(original_font_face);
    (*scaled_font).hash_entry.hash = _cairo_scaled_font_compute_hash(scaled_font);
    status = _cairo_hash_table_insert(
        (*font_map).hash_table,
        &mut (*scaled_font).hash_entry,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        old = (*font_map).mru_scaled_font;
        let ref mut fresh28 = (*font_map).mru_scaled_font;
        *fresh28 = scaled_font;
        ::std::intrinsics::atomic_xadd(
            &mut (*scaled_font).ref_count.ref_count,
            1 as libc::c_int,
        );
    }
    _cairo_scaled_font_map_unlock();
    cairo_scaled_font_destroy(old);
    if font_face != original_font_face {
        cairo_font_face_destroy(font_face);
    }
    if !dead.is_null() {
        cairo_scaled_font_destroy(dead);
    }
    if status as u64 != 0 {
        _cairo_scaled_font_fini_internal(scaled_font);
        free(scaled_font as *mut libc::c_void);
        return _cairo_scaled_font_create_in_error(status);
    }
    return scaled_font;
}
static mut _cairo_scaled_font_nil_objects: [*mut cairo_scaled_font_t; 45] = [0
    as *const cairo_scaled_font_t as *mut cairo_scaled_font_t; 45];
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_scaled_font_t {
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status != CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1254 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"cairo_scaled_font_t *_cairo_scaled_font_create_in_error(cairo_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    if status as libc::c_uint == CAIRO_STATUS_NO_MEMORY as libc::c_int as libc::c_uint {
        return &_cairo_scaled_font_nil as *const cairo_scaled_font_t
            as *mut cairo_scaled_font_t;
    }
    pthread_mutex_lock(&mut _cairo_scaled_font_error_mutex);
    scaled_font = _cairo_scaled_font_nil_objects[status as usize];
    if scaled_font.is_null() {
        scaled_font = (if ::std::mem::size_of::<cairo_scaled_font_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_scaled_font_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_scaled_font_t;
        if scaled_font.is_null() {
            pthread_mutex_unlock(&mut _cairo_scaled_font_error_mutex);
            let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_scaled_font_nil as *const cairo_scaled_font_t
                as *mut cairo_scaled_font_t;
        }
        *scaled_font = _cairo_scaled_font_nil;
        (*scaled_font).status = status;
        _cairo_scaled_font_nil_objects[status as usize] = scaled_font;
    }
    pthread_mutex_unlock(&mut _cairo_scaled_font_error_mutex);
    return scaled_font;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_reset_static_data() {
    let mut status: libc::c_int = 0;
    pthread_mutex_lock(&mut _cairo_scaled_font_error_mutex);
    status = CAIRO_STATUS_SUCCESS as libc::c_int;
    while status <= CAIRO_STATUS_LAST_STATUS as libc::c_int {
        free(_cairo_scaled_font_nil_objects[status as usize] as *mut libc::c_void);
        _cairo_scaled_font_nil_objects[status as usize] = 0 as *mut cairo_scaled_font_t;
        status += 1;
    }
    pthread_mutex_unlock(&mut _cairo_scaled_font_error_mutex);
    pthread_mutex_lock(&mut _cairo_scaled_glyph_page_cache_mutex);
    if !(cairo_scaled_glyph_page_cache.hash_table).is_null() {
        _cairo_cache_fini(&mut cairo_scaled_glyph_page_cache);
        cairo_scaled_glyph_page_cache.hash_table = 0 as *mut cairo_hash_table_t;
    }
    pthread_mutex_unlock(&mut _cairo_scaled_glyph_page_cache_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_reference(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> *mut cairo_scaled_font_t {
    if scaled_font.is_null()
        || _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return scaled_font;
    }
    if _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&scaled_font->ref_count)\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1324 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"cairo_scaled_font_t *cairo_scaled_font_reference(cairo_scaled_font_t *)\0",
            ))
                .as_ptr(),
        );
    }
    ::std::intrinsics::atomic_xadd(
        &mut (*scaled_font).ref_count.ref_count,
        1 as libc::c_int,
    );
    return scaled_font;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_destroy(
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    let mut lru: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    let mut font_map: *mut cairo_scaled_font_map_t = 0 as *mut cairo_scaled_font_map_t;
    if scaled_font.is_null()
        || _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return;
    }
    if _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&scaled_font->ref_count)\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1354 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void cairo_scaled_font_destroy(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    font_map = _cairo_scaled_font_map_lock();
    if !font_map.is_null() {} else {
        __assert_fail(
            b"font_map != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            1357 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void cairo_scaled_font_destroy(cairo_scaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    if ::std::intrinsics::atomic_xsub(
        &mut (*scaled_font).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {
        if (*scaled_font).cache_frozen == 0 {} else {
            __assert_fail(
                b"! scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
                1362 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void cairo_scaled_font_destroy(cairo_scaled_font_t *)\0"))
                    .as_ptr(),
            );
        }
        if (*scaled_font).global_cache_frozen == 0 {} else {
            __assert_fail(
                b"! scaled_font->global_cache_frozen\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
                1363 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void cairo_scaled_font_destroy(cairo_scaled_font_t *)\0"))
                    .as_ptr(),
            );
        }
        if !(_cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
            > 0 as libc::c_int)
        {
            if (*scaled_font).placeholder() == 0
                && (*scaled_font).hash_entry.hash != 0 as libc::c_int as libc::c_ulong
            {
                if !((*scaled_font).holdover() != 0) {
                    if (*font_map).num_holdovers == 256 as libc::c_int {
                        lru = (*font_map).holdovers[0 as libc::c_int as usize];
                        if !(_cairo_atomic_int_get(&mut (*lru).ref_count.ref_count)
                            > 0 as libc::c_int)
                        {} else {
                            __assert_fail(
                                b"! CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&lru->ref_count)\0"
                                    as *const u8 as *const libc::c_char,
                                b"../src/cairo-scaled-font.c\0" as *const u8
                                    as *const libc::c_char,
                                1383 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 54],
                                    &[libc::c_char; 54],
                                >(
                                    b"void cairo_scaled_font_destroy(cairo_scaled_font_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        _cairo_hash_table_remove(
                            (*font_map).hash_table,
                            &mut (*lru).hash_entry,
                        );
                        let ref mut fresh29 = (*font_map).num_holdovers;
                        *fresh29 -= 1;
                        memmove(
                            &mut *((*font_map).holdovers)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize)
                                as *mut *mut cairo_scaled_font_t as *mut libc::c_void,
                            &mut *((*font_map).holdovers)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize)
                                as *mut *mut cairo_scaled_font_t as *const libc::c_void,
                            ((*font_map).num_holdovers as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut cairo_scaled_font_t>()
                                        as libc::c_ulong,
                                ),
                        );
                    }
                    let ref mut fresh30 = (*font_map).num_holdovers;
                    let fresh31 = *fresh30;
                    *fresh30 = *fresh30 + 1;
                    let ref mut fresh32 = (*font_map).holdovers[fresh31 as usize];
                    *fresh32 = scaled_font;
                    (*scaled_font).set_holdover(1 as libc::c_int as libc::c_uint);
                }
            } else {
                lru = scaled_font;
            }
        }
    }
    _cairo_scaled_font_map_unlock();
    if !lru.is_null() {
        _cairo_scaled_font_fini_internal(lru);
        free(lru as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_reference_count(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> libc::c_uint {
    if scaled_font.is_null()
        || _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    return _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_user_data(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut key: *const cairo_user_data_key_t,
) -> *mut libc::c_void {
    return _cairo_user_data_array_get_data(&mut (*scaled_font).user_data, key);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_set_user_data(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut key: *const cairo_user_data_key_t,
    mut user_data: *mut libc::c_void,
    mut destroy: cairo_destroy_func_t,
) -> cairo_status_t {
    if _cairo_atomic_int_get(&mut (*scaled_font).ref_count.ref_count)
        == -(1 as libc::c_int)
    {
        return (*scaled_font).status;
    }
    return _cairo_user_data_array_set_data(
        &mut (*scaled_font).user_data,
        key,
        user_data,
        destroy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_extents(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut extents: *mut cairo_font_extents_t,
) {
    if (*scaled_font).status as u64 != 0 {
        (*extents).ascent = 0.0f64;
        (*extents).descent = 0.0f64;
        (*extents).height = 0.0f64;
        (*extents).max_x_advance = 0.0f64;
        (*extents).max_y_advance = 0.0f64;
        return;
    }
    *extents = (*scaled_font).extents;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_text_extents(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut utf8: *const libc::c_char,
    mut extents: *mut cairo_text_extents_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    let mut num_glyphs: libc::c_int = 0;
    if !((*scaled_font).status as u64 != 0) {
        if !utf8.is_null() {
            status = cairo_scaled_font_text_to_glyphs(
                scaled_font,
                0.0f64,
                0.0f64,
                utf8,
                -(1 as libc::c_int),
                &mut glyphs,
                &mut num_glyphs,
                0 as *mut *mut cairo_text_cluster_t,
                0 as *mut libc::c_int,
                0 as *mut cairo_text_cluster_flags_t,
            );
            if status as u64 != 0 {
                status = _cairo_scaled_font_set_error(scaled_font, status);
            } else {
                cairo_scaled_font_glyph_extents(
                    scaled_font,
                    glyphs,
                    num_glyphs,
                    extents,
                );
                free(glyphs as *mut libc::c_void);
                return;
            }
        }
    }
    (*extents).x_bearing = 0.0f64;
    (*extents).y_bearing = 0.0f64;
    (*extents).width = 0.0f64;
    (*extents).height = 0.0f64;
    (*extents).x_advance = 0.0f64;
    (*extents).y_advance = 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_glyph_extents(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *const cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut extents: *mut cairo_text_extents_t,
) {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    let mut min_x: libc::c_double = 0.0f64;
    let mut min_y: libc::c_double = 0.0f64;
    let mut max_x: libc::c_double = 0.0f64;
    let mut max_y: libc::c_double = 0.0f64;
    let mut visible: cairo_bool_t = 0 as libc::c_int;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    (*extents).x_bearing = 0.0f64;
    (*extents).y_bearing = 0.0f64;
    (*extents).width = 0.0f64;
    (*extents).height = 0.0f64;
    (*extents).x_advance = 0.0f64;
    (*extents).y_advance = 0.0f64;
    if !((*scaled_font).status as u64 != 0) {
        if !(num_glyphs == 0 as libc::c_int) {
            if num_glyphs < 0 as libc::c_int {
                let mut status__: cairo_status_t = _cairo_error(
                    CAIRO_STATUS_NEGATIVE_COUNT,
                );
            } else if glyphs.is_null() {
                let mut status___0: cairo_status_t = _cairo_error(
                    CAIRO_STATUS_NULL_POINTER,
                );
            } else {
                _cairo_scaled_font_freeze_cache(scaled_font);
                i = 0 as libc::c_int;
                loop {
                    if !(i < num_glyphs) {
                        current_block = 6174974146017752131;
                        break;
                    }
                    let mut left: libc::c_double = 0.;
                    let mut top: libc::c_double = 0.;
                    let mut right: libc::c_double = 0.;
                    let mut bottom: libc::c_double = 0.;
                    status = _cairo_scaled_glyph_lookup(
                        scaled_font,
                        (*glyphs.offset(i as isize)).index,
                        CAIRO_SCALED_GLYPH_INFO_METRICS,
                        0 as *const cairo_color_t,
                        &mut scaled_glyph,
                    ) as cairo_status_t;
                    if status as u64 != 0 {
                        status = _cairo_scaled_font_set_error(scaled_font, status);
                        current_block = 5027559831149889427;
                        break;
                    } else {
                        if !((*scaled_glyph).metrics.width
                            == 0 as libc::c_int as libc::c_double
                            || (*scaled_glyph).metrics.height
                                == 0 as libc::c_int as libc::c_double)
                        {
                            left = (*scaled_glyph).metrics.x_bearing
                                + (*glyphs.offset(i as isize)).x;
                            right = left + (*scaled_glyph).metrics.width;
                            top = (*scaled_glyph).metrics.y_bearing
                                + (*glyphs.offset(i as isize)).y;
                            bottom = top + (*scaled_glyph).metrics.height;
                            if visible == 0 {
                                visible = 1 as libc::c_int;
                                min_x = left;
                                max_x = right;
                                min_y = top;
                                max_y = bottom;
                            } else {
                                if left < min_x {
                                    min_x = left;
                                }
                                if right > max_x {
                                    max_x = right;
                                }
                                if top < min_y {
                                    min_y = top;
                                }
                                if bottom > max_y {
                                    max_y = bottom;
                                }
                            }
                        }
                        i += 1;
                    }
                }
                match current_block {
                    6174974146017752131 => {
                        if visible != 0 {
                            (*extents)
                                .x_bearing = min_x
                                - (*glyphs.offset(0 as libc::c_int as isize)).x;
                            (*extents)
                                .y_bearing = min_y
                                - (*glyphs.offset(0 as libc::c_int as isize)).y;
                            (*extents).width = max_x - min_x;
                            (*extents).height = max_y - min_y;
                        } else {
                            (*extents).x_bearing = 0.0f64;
                            (*extents).y_bearing = 0.0f64;
                            (*extents).width = 0.0f64;
                            (*extents).height = 0.0f64;
                        }
                        if num_glyphs != 0 {
                            let mut x0: libc::c_double = 0.;
                            let mut y0: libc::c_double = 0.;
                            let mut x1: libc::c_double = 0.;
                            let mut y1: libc::c_double = 0.;
                            x0 = (*glyphs.offset(0 as libc::c_int as isize)).x;
                            y0 = (*glyphs.offset(0 as libc::c_int as isize)).y;
                            x1 = (*glyphs
                                .offset((num_glyphs - 1 as libc::c_int) as isize))
                                .x + (*scaled_glyph).metrics.x_advance;
                            y1 = (*glyphs
                                .offset((num_glyphs - 1 as libc::c_int) as isize))
                                .y + (*scaled_glyph).metrics.y_advance;
                            (*extents).x_advance = x1 - x0;
                            (*extents).y_advance = y1 - y0;
                        } else {
                            (*extents).x_advance = 0.0f64;
                            (*extents).y_advance = 0.0f64;
                        }
                    }
                    _ => {}
                }
                _cairo_scaled_font_thaw_cache(scaled_font);
                return;
            }
        }
    }
    (*extents).x_bearing = 0.0f64;
    (*extents).y_bearing = 0.0f64;
    (*extents).width = 0.0f64;
    (*extents).height = 0.0f64;
    (*extents).x_advance = 0.0f64;
    (*extents).y_advance = 0.0f64;
}
unsafe extern "C" fn cairo_scaled_font_text_to_glyphs_internal_cached(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut utf8: *const libc::c_char,
    mut glyphs: *mut cairo_glyph_t,
    mut clusters: *mut *mut cairo_text_cluster_t,
    mut num_chars: libc::c_int,
) -> cairo_status_t {
    let mut glyph_lut: [glyph_lut_elt; 64] = [glyph_lut_elt {
        index: 0,
        x_advance: 0.,
        y_advance: 0.,
    }; 64];
    let mut glyph_lut_unicode: [uint32_t; 64] = [0; 64];
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        glyph_lut_unicode[i as usize] = !(0 as libc::c_uint);
        i += 1;
    }
    p = utf8;
    i = 0 as libc::c_int;
    while i < num_chars {
        let mut idx: libc::c_int = 0;
        let mut num_bytes: libc::c_int = 0;
        let mut unicode: uint32_t = 0;
        let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        let mut glyph_slot: *mut glyph_lut_elt = 0 as *mut glyph_lut_elt;
        num_bytes = _cairo_utf8_get_char_validated(p, &mut unicode);
        p = p.offset(num_bytes as isize);
        (*glyphs.offset(i as isize)).x = x;
        (*glyphs.offset(i as isize)).y = y;
        idx = unicode
            .wrapping_rem(
                (::std::mem::size_of::<[glyph_lut_elt; 64]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<glyph_lut_elt>() as libc::c_ulong,
                    ) as libc::c_int as libc::c_uint,
            ) as libc::c_int;
        glyph_slot = &mut *glyph_lut.as_mut_ptr().offset(idx as isize)
            as *mut glyph_lut_elt;
        if glyph_lut_unicode[idx as usize] == unicode {
            (*glyphs.offset(i as isize)).index = (*glyph_slot).index;
            x += (*glyph_slot).x_advance;
            y += (*glyph_slot).y_advance;
        } else {
            let mut g: libc::c_ulong = 0;
            g = ((*(*scaled_font).backend).ucs4_to_index)
                .expect(
                    "non-null function pointer",
                )(scaled_font as *mut libc::c_void, unicode);
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                g,
                CAIRO_SCALED_GLYPH_INFO_METRICS,
                0 as *const cairo_color_t,
                &mut scaled_glyph,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return status;
            }
            x += (*scaled_glyph).metrics.x_advance;
            y += (*scaled_glyph).metrics.y_advance;
            glyph_lut_unicode[idx as usize] = unicode;
            (*glyph_slot).index = g;
            (*glyph_slot).x_advance = (*scaled_glyph).metrics.x_advance;
            (*glyph_slot).y_advance = (*scaled_glyph).metrics.y_advance;
            (*glyphs.offset(i as isize)).index = g;
        }
        if !clusters.is_null() {
            (*(*clusters).offset(i as isize)).num_bytes = num_bytes;
            (*(*clusters).offset(i as isize)).num_glyphs = 1 as libc::c_int;
        }
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn cairo_scaled_font_text_to_glyphs_internal_uncached(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut utf8: *const libc::c_char,
    mut glyphs: *mut cairo_glyph_t,
    mut clusters: *mut *mut cairo_text_cluster_t,
    mut num_chars: libc::c_int,
) -> cairo_status_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    p = utf8;
    i = 0 as libc::c_int;
    while i < num_chars {
        let mut g: libc::c_ulong = 0;
        let mut num_bytes: libc::c_int = 0;
        let mut unicode: uint32_t = 0;
        let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        num_bytes = _cairo_utf8_get_char_validated(p, &mut unicode);
        p = p.offset(num_bytes as isize);
        (*glyphs.offset(i as isize)).x = x;
        (*glyphs.offset(i as isize)).y = y;
        g = ((*(*scaled_font).backend).ucs4_to_index)
            .expect(
                "non-null function pointer",
            )(scaled_font as *mut libc::c_void, unicode);
        if num_chars > 1 as libc::c_int {
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                g,
                CAIRO_SCALED_GLYPH_INFO_METRICS,
                0 as *const cairo_color_t,
                &mut scaled_glyph,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return status;
            }
            x += (*scaled_glyph).metrics.x_advance;
            y += (*scaled_glyph).metrics.y_advance;
        }
        (*glyphs.offset(i as isize)).index = g;
        if !clusters.is_null() {
            (*(*clusters).offset(i as isize)).num_bytes = num_bytes;
            (*(*clusters).offset(i as isize)).num_glyphs = 1 as libc::c_int;
        }
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_text_to_glyphs(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut *mut cairo_glyph_t,
    mut num_glyphs: *mut libc::c_int,
    mut clusters: *mut *mut cairo_text_cluster_t,
    mut num_clusters: *mut libc::c_int,
    mut cluster_flags: *mut cairo_text_cluster_flags_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut num_chars: libc::c_int = 0 as libc::c_int;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut orig_glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    let mut orig_clusters: *mut cairo_text_cluster_t = 0 as *mut cairo_text_cluster_t;
    status = (*scaled_font).status as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if glyphs.is_null() || num_glyphs.is_null() {
        status = _cairo_error(CAIRO_STATUS_NULL_POINTER) as cairo_int_status_t;
    } else {
        if utf8.is_null() && utf8_len == -(1 as libc::c_int) {
            utf8_len = 0 as libc::c_int;
        }
        if utf8_len != 0 && utf8.is_null()
            || !clusters.is_null() && num_clusters.is_null()
            || !clusters.is_null() && cluster_flags.is_null()
        {
            status = _cairo_error(CAIRO_STATUS_NULL_POINTER) as cairo_int_status_t;
        } else {
            if utf8_len == -(1 as libc::c_int) {
                utf8_len = strlen(utf8) as libc::c_int;
            }
            if !glyphs.is_null() && (*glyphs).is_null() {
                *num_glyphs = 0 as libc::c_int;
            }
            if !clusters.is_null() && (*clusters).is_null() {
                *num_clusters = 0 as libc::c_int;
            }
            if clusters.is_null() && !num_clusters.is_null() {
                num_clusters = 0 as *mut libc::c_int;
            }
            if !cluster_flags.is_null() {
                *cluster_flags = 0 as cairo_text_cluster_flags_t;
            }
            if clusters.is_null() && !cluster_flags.is_null() {
                cluster_flags = 0 as *mut cairo_text_cluster_flags_t;
            }
            if utf8_len < 0 as libc::c_int || *num_glyphs < 0 as libc::c_int
                || !num_clusters.is_null() && *num_clusters < 0 as libc::c_int
            {
                status = _cairo_error(CAIRO_STATUS_NEGATIVE_COUNT) as cairo_int_status_t;
            } else if utf8_len == 0 as libc::c_int {
                status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            } else {
                status = _cairo_utf8_to_ucs4(
                    utf8,
                    utf8_len,
                    0 as *mut *mut uint32_t,
                    &mut num_chars,
                ) as cairo_int_status_t;
                if !(status as u64 != 0) {
                    _cairo_scaled_font_freeze_cache(scaled_font);
                    orig_glyphs = *glyphs;
                    orig_clusters = if !clusters.is_null() {
                        *clusters
                    } else {
                        0 as *mut cairo_text_cluster_t
                    };
                    if ((*(*scaled_font).backend).text_to_glyphs).is_some() {
                        status = ((*(*scaled_font).backend).text_to_glyphs)
                            .expect(
                                "non-null function pointer",
                            )(
                            scaled_font as *mut libc::c_void,
                            x,
                            y,
                            utf8,
                            utf8_len,
                            glyphs,
                            num_glyphs,
                            clusters,
                            num_clusters,
                            cluster_flags,
                        );
                        if status as libc::c_uint
                            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                as libc::c_uint
                        {
                            if status as libc::c_uint
                                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                            {
                                if *num_glyphs < 0 as libc::c_int {
                                    status = _cairo_error(CAIRO_STATUS_NEGATIVE_COUNT)
                                        as cairo_int_status_t;
                                } else if *num_glyphs != 0 as libc::c_int
                                    && (*glyphs).is_null()
                                {
                                    status = _cairo_error(CAIRO_STATUS_NULL_POINTER)
                                        as cairo_int_status_t;
                                } else if !clusters.is_null() {
                                    if *num_clusters < 0 as libc::c_int {
                                        status = _cairo_error(CAIRO_STATUS_NEGATIVE_COUNT)
                                            as cairo_int_status_t;
                                    } else if *num_clusters != 0 as libc::c_int
                                        && (*clusters).is_null()
                                    {
                                        status = _cairo_error(CAIRO_STATUS_NULL_POINTER)
                                            as cairo_int_status_t;
                                    } else {
                                        status = _cairo_validate_text_clusters(
                                            utf8,
                                            utf8_len,
                                            *glyphs,
                                            *num_glyphs,
                                            *clusters,
                                            *num_clusters,
                                            *cluster_flags,
                                        ) as cairo_int_status_t;
                                    }
                                }
                                current_block = 13351606174237626008;
                            } else {
                                current_block = 13351606174237626008;
                            }
                        } else {
                            current_block = 2290177392965769716;
                        }
                    } else {
                        current_block = 2290177392965769716;
                    }
                    match current_block {
                        2290177392965769716 => {
                            if *num_glyphs < num_chars {
                                *glyphs = cairo_glyph_allocate(num_chars);
                                if (*glyphs).is_null() {
                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                        as cairo_int_status_t;
                                    current_block = 13351606174237626008;
                                } else {
                                    current_block = 9859671972921157070;
                                }
                            } else {
                                current_block = 9859671972921157070;
                            }
                            match current_block {
                                13351606174237626008 => {}
                                _ => {
                                    *num_glyphs = num_chars;
                                    if !clusters.is_null() {
                                        if *num_clusters < num_chars {
                                            *clusters = cairo_text_cluster_allocate(num_chars);
                                            if (*clusters).is_null() {
                                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                    as cairo_int_status_t;
                                                current_block = 13351606174237626008;
                                            } else {
                                                current_block = 307447392441238883;
                                            }
                                        } else {
                                            current_block = 307447392441238883;
                                        }
                                        match current_block {
                                            13351606174237626008 => {}
                                            _ => {
                                                *num_clusters = num_chars;
                                                current_block = 15970011996474399071;
                                            }
                                        }
                                    } else {
                                        current_block = 15970011996474399071;
                                    }
                                    match current_block {
                                        13351606174237626008 => {}
                                        _ => {
                                            if num_chars > 16 as libc::c_int {
                                                status = cairo_scaled_font_text_to_glyphs_internal_cached(
                                                    scaled_font,
                                                    x,
                                                    y,
                                                    utf8,
                                                    *glyphs,
                                                    clusters,
                                                    num_chars,
                                                ) as cairo_int_status_t;
                                            } else {
                                                status = cairo_scaled_font_text_to_glyphs_internal_uncached(
                                                    scaled_font,
                                                    x,
                                                    y,
                                                    utf8,
                                                    *glyphs,
                                                    clusters,
                                                    num_chars,
                                                ) as cairo_int_status_t;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    _cairo_scaled_font_thaw_cache(scaled_font);
                    if status as u64 != 0 {
                        *num_glyphs = 0 as libc::c_int;
                        if *glyphs != orig_glyphs {
                            cairo_glyph_free(*glyphs);
                            *glyphs = orig_glyphs;
                        }
                        if !clusters.is_null() {
                            *num_clusters = 0 as libc::c_int;
                            if *clusters != orig_clusters {
                                cairo_text_cluster_free(*clusters);
                                *clusters = orig_clusters;
                            }
                        }
                    }
                    return _cairo_scaled_font_set_error(
                        scaled_font,
                        status as cairo_status_t,
                    );
                }
            }
        }
    }
    if !num_glyphs.is_null() {
        *num_glyphs = 0 as libc::c_int;
    }
    if !num_clusters.is_null() {
        *num_clusters = 0 as libc::c_int;
    }
    return status as cairo_status_t;
}
#[inline]
unsafe extern "C" fn _range_contains_glyph(
    mut extents: *const cairo_box_t,
    mut left: cairo_fixed_t,
    mut top: cairo_fixed_t,
    mut right: cairo_fixed_t,
    mut bottom: cairo_fixed_t,
) -> cairo_bool_t {
    if left == right || top == bottom {
        return 0 as libc::c_int;
    }
    return (right > (*extents).p1.x && left < (*extents).p2.x && bottom > (*extents).p1.y
        && top < (*extents).p2.y) as libc::c_int;
}
unsafe extern "C" fn _cairo_scaled_font_single_glyph_device_extents(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyph: *const cairo_glyph_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_scaled_font_freeze_cache(scaled_font);
    status = _cairo_scaled_glyph_lookup(
        scaled_font,
        (*glyph).index,
        CAIRO_SCALED_GLYPH_INFO_METRICS,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    ) as cairo_status_t;
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        let mut round_xy: cairo_bool_t = (_cairo_font_options_get_round_glyph_positions(
            &mut (*scaled_font).options,
        ) as libc::c_uint == CAIRO_ROUND_GLYPH_POS_ON as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        let mut v: cairo_fixed_t = 0;
        if round_xy != 0 {
            v = _cairo_fixed_from_int(_cairo_lround((*glyph).x));
        } else {
            v = _cairo_fixed_from_double((*glyph).x);
        }
        box_0.p1.x = v + (*scaled_glyph).bbox.p1.x;
        box_0.p2.x = v + (*scaled_glyph).bbox.p2.x;
        if round_xy != 0 {
            v = _cairo_fixed_from_int(_cairo_lround((*glyph).y));
        } else {
            v = _cairo_fixed_from_double((*glyph).y);
        }
        box_0.p1.y = v + (*scaled_glyph).bbox.p1.y;
        box_0.p2.y = v + (*scaled_glyph).bbox.p2.y;
        _cairo_box_round_to_rectangle(&mut box_0, extents);
    }
    _cairo_scaled_font_thaw_cache(scaled_font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_glyph_device_extents(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *const cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut extents: *mut cairo_rectangle_int_t,
    mut overlap_out: *mut cairo_bool_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = {
        let mut init = _cairo_line {
            p1: {
                let mut init = _cairo_point {
                    x: 2147483647 as libc::c_int,
                    y: 2147483647 as libc::c_int,
                };
                init
            },
            p2: {
                let mut init = _cairo_point {
                    x: -(2147483647 as libc::c_int) - 1 as libc::c_int,
                    y: -(2147483647 as libc::c_int) - 1 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut glyph_cache: [*mut cairo_scaled_glyph_t; 64] = [0
        as *mut cairo_scaled_glyph_t; 64];
    let mut overlap: cairo_bool_t = if !overlap_out.is_null() {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut round_glyph_positions: cairo_round_glyph_positions_t = _cairo_font_options_get_round_glyph_positions(
        &mut (*scaled_font).options,
    );
    let mut i: libc::c_int = 0;
    if (*scaled_font).status as u64 != 0 {
        return (*scaled_font).status;
    }
    if num_glyphs == 1 as libc::c_int {
        if !overlap_out.is_null() {
            *overlap_out = 0 as libc::c_int;
        }
        return _cairo_scaled_font_single_glyph_device_extents(
            scaled_font,
            glyphs,
            extents,
        );
    }
    _cairo_scaled_font_freeze_cache(scaled_font);
    memset(
        glyph_cache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut cairo_scaled_glyph_t; 64]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < num_glyphs {
        let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        let mut x: cairo_fixed_t = 0;
        let mut y: cairo_fixed_t = 0;
        let mut x1: cairo_fixed_t = 0;
        let mut y1: cairo_fixed_t = 0;
        let mut x2: cairo_fixed_t = 0;
        let mut y2: cairo_fixed_t = 0;
        let mut cache_index: libc::c_int = ((*glyphs.offset(i as isize)).index)
            .wrapping_rem(
                (::std::mem::size_of::<[*mut cairo_scaled_glyph_t; 64]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut cairo_scaled_glyph_t>()
                            as libc::c_ulong,
                    ) as libc::c_int as libc::c_ulong,
            ) as libc::c_int;
        scaled_glyph = glyph_cache[cache_index as usize];
        if scaled_glyph.is_null()
            || (*scaled_glyph).hash_entry.hash & 0xffffff as libc::c_int as libc::c_ulong
                != (*glyphs.offset(i as isize)).index
        {
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                (*glyphs.offset(i as isize)).index,
                CAIRO_SCALED_GLYPH_INFO_METRICS,
                0 as *const cairo_color_t,
                &mut scaled_glyph,
            ) as cairo_status_t;
            if status as u64 != 0 {
                break;
            }
            glyph_cache[cache_index as usize] = scaled_glyph;
        }
        if round_glyph_positions as libc::c_uint
            == CAIRO_ROUND_GLYPH_POS_ON as libc::c_int as libc::c_uint
        {
            x = _cairo_fixed_from_int(_cairo_lround((*glyphs.offset(i as isize)).x));
        } else {
            x = _cairo_fixed_from_double((*glyphs.offset(i as isize)).x);
        }
        x1 = x + (*scaled_glyph).bbox.p1.x;
        x2 = x + (*scaled_glyph).bbox.p2.x;
        if round_glyph_positions as libc::c_uint
            == CAIRO_ROUND_GLYPH_POS_ON as libc::c_int as libc::c_uint
        {
            y = _cairo_fixed_from_int(_cairo_lround((*glyphs.offset(i as isize)).y));
        } else {
            y = _cairo_fixed_from_double((*glyphs.offset(i as isize)).y);
        }
        y1 = y + (*scaled_glyph).bbox.p1.y;
        y2 = y + (*scaled_glyph).bbox.p2.y;
        if overlap == 0 as libc::c_int {
            overlap = _range_contains_glyph(&mut box_0, x1, y1, x2, y2);
        }
        if x1 < box_0.p1.x {
            box_0.p1.x = x1;
        }
        if x2 > box_0.p2.x {
            box_0.p2.x = x2;
        }
        if y1 < box_0.p1.y {
            box_0.p1.y = y1;
        }
        if y2 > box_0.p2.y {
            box_0.p2.y = y2;
        }
        i += 1;
    }
    _cairo_scaled_font_thaw_cache(scaled_font);
    if status as u64 != 0 {
        return _cairo_scaled_font_set_error(scaled_font, status);
    }
    if box_0.p1.x < box_0.p2.x {
        _cairo_box_round_to_rectangle(&mut box_0, extents);
    } else {
        let ref mut fresh33 = (*extents).y;
        *fresh33 = 0 as libc::c_int;
        (*extents).x = *fresh33;
        let ref mut fresh34 = (*extents).height;
        *fresh34 = 0 as libc::c_int;
        (*extents).width = *fresh34;
    }
    if !overlap_out.is_null() {
        *overlap_out = overlap;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_glyph_approximate_extents(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *const cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut extents: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut pad: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if (*scaled_font).fs_extents.max_x_advance == 0 as libc::c_int as libc::c_double
        || (*scaled_font).fs_extents.height == 0 as libc::c_int as libc::c_double
        || (*scaled_font).max_scale == 0 as libc::c_int as libc::c_double
    {
        return 0 as libc::c_int;
    }
    if num_glyphs != 0 {} else {
        __assert_fail(
            b"num_glyphs\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            2351 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"cairo_bool_t _cairo_scaled_font_glyph_approximate_extents(cairo_scaled_font_t *, const cairo_glyph_t *, int, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    x1 = (*glyphs.offset(0 as libc::c_int as isize)).x;
    x0 = x1;
    y1 = (*glyphs.offset(0 as libc::c_int as isize)).y;
    y0 = y1;
    i = 1 as libc::c_int;
    while i < num_glyphs {
        let mut g: libc::c_double = 0.;
        g = (*glyphs.offset(i as isize)).x;
        if g < x0 {
            x0 = g;
        }
        if g > x1 {
            x1 = g;
        }
        g = (*glyphs.offset(i as isize)).y;
        if g < y0 {
            y0 = g;
        }
        if g > y1 {
            y1 = g;
        }
        i += 1;
    }
    pad = if (*scaled_font).fs_extents.max_x_advance > (*scaled_font).fs_extents.height {
        (*scaled_font).fs_extents.max_x_advance
    } else {
        (*scaled_font).fs_extents.height
    };
    pad *= (*scaled_font).max_scale;
    (*extents).x = floor(x0 - pad) as libc::c_int;
    (*extents).width = (ceil(x1 + pad) - (*extents).x as libc::c_double) as libc::c_int;
    (*extents).y = floor(y0 - pad) as libc::c_int;
    (*extents).height = (ceil(y1 + pad) - (*extents).y as libc::c_double) as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _add_unit_rectangle_to_path(
    mut path: *mut cairo_path_fixed_t,
    mut x: cairo_fixed_t,
    mut y: cairo_fixed_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_path_fixed_move_to(path, x, y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_rel_line_to(
        path,
        _cairo_fixed_from_int(1 as libc::c_int),
        _cairo_fixed_from_int(0 as libc::c_int),
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_rel_line_to(
        path,
        _cairo_fixed_from_int(0 as libc::c_int),
        _cairo_fixed_from_int(1 as libc::c_int),
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_rel_line_to(
        path,
        _cairo_fixed_from_int(-(1 as libc::c_int)),
        _cairo_fixed_from_int(0 as libc::c_int),
    );
    if status as u64 != 0 {
        return status;
    }
    return _cairo_path_fixed_close_path(path);
}
unsafe extern "C" fn _trace_mask_to_path(
    mut mask: *mut cairo_image_surface_t,
    mut path: *mut cairo_path_fixed_t,
    mut tx: libc::c_double,
    mut ty: libc::c_double,
) -> cairo_status_t {
    let mut row: *const uint8_t = 0 as *const uint8_t;
    let mut rows: libc::c_int = 0;
    let mut cols: libc::c_int = 0;
    let mut bytes_per_row: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut xoff: libc::c_double = 0.;
    let mut yoff: libc::c_double = 0.;
    let mut x0: cairo_fixed_t = 0;
    let mut y0: cairo_fixed_t = 0;
    let mut px: cairo_fixed_t = 0;
    let mut py: cairo_fixed_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    mask = _cairo_image_surface_coerce_to_format(mask, CAIRO_FORMAT_A1);
    status = (*mask).base.status;
    if status as u64 != 0 {
        return status;
    }
    cairo_surface_get_device_offset(&mut (*mask).base, &mut xoff, &mut yoff);
    x0 = _cairo_fixed_from_double(tx - xoff);
    y0 = _cairo_fixed_from_double(ty - yoff);
    bytes_per_row = ((*mask).width + 7 as libc::c_int) / 8 as libc::c_int;
    row = (*mask).data;
    y = 0 as libc::c_int;
    rows = (*mask).height;
    's_60: loop {
        let fresh35 = rows;
        rows = rows - 1;
        if !(fresh35 != 0) {
            break;
        }
        let mut byte_ptr: *const uint8_t = row;
        x = 0 as libc::c_int;
        py = _cairo_fixed_from_int(y);
        cols = bytes_per_row;
        loop {
            let fresh36 = cols;
            cols = cols - 1;
            if !(fresh36 != 0) {
                break;
            }
            let fresh37 = byte_ptr;
            byte_ptr = byte_ptr.offset(1);
            let mut byte: uint8_t = *fresh37;
            if byte as libc::c_int == 0 as libc::c_int {
                x += 8 as libc::c_int;
            } else {
                byte = (((byte as libc::c_ulong).wrapping_mul(0x802 as libc::c_ulong)
                    & 0x22110 as libc::c_ulong
                    | (byte as libc::c_ulong).wrapping_mul(0x8020 as libc::c_ulong)
                        & 0x88440 as libc::c_ulong)
                    .wrapping_mul(0x10101 as libc::c_ulong) >> 16 as libc::c_int)
                    as uint8_t;
                bit = (1 as libc::c_int) << 7 as libc::c_int;
                while bit != 0 && x < (*mask).width {
                    if byte as libc::c_int & bit != 0 {
                        px = _cairo_fixed_from_int(x);
                        status = _add_unit_rectangle_to_path(path, px + x0, py + y0);
                        if status as u64 != 0 {
                            break 's_60;
                        }
                    }
                    bit >>= 1 as libc::c_int;
                    x += 1;
                }
            }
        }
        row = row.offset((*mask).stride as isize);
        y += 1;
    }
    cairo_surface_destroy(&mut (*mask).base);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_glyph_path(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *const cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut path: *mut cairo_path_fixed_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    status = (*scaled_font).status as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    _cairo_scaled_font_freeze_cache(scaled_font);
    i = 0 as libc::c_int;
    while i < num_glyphs {
        let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        status = _cairo_scaled_glyph_lookup(
            scaled_font,
            (*glyphs.offset(i as isize)).index,
            CAIRO_SCALED_GLYPH_INFO_PATH,
            0 as *const cairo_color_t,
            &mut scaled_glyph,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = _cairo_path_fixed_append(
                path,
                (*scaled_glyph).path,
                _cairo_fixed_from_double((*glyphs.offset(i as isize)).x),
                _cairo_fixed_from_double((*glyphs.offset(i as isize)).y),
            ) as cairo_int_status_t;
        } else if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                (*glyphs.offset(i as isize)).index,
                CAIRO_SCALED_GLYPH_INFO_SURFACE,
                0 as *const cairo_color_t,
                &mut scaled_glyph,
            );
            if status as u64 != 0 {
                break;
            }
            status = _trace_mask_to_path(
                (*scaled_glyph).surface,
                path,
                (*glyphs.offset(i as isize)).x,
                (*glyphs.offset(i as isize)).y,
            ) as cairo_int_status_t;
        }
        if status as u64 != 0 {
            break;
        }
        i += 1;
    }
    _cairo_scaled_font_thaw_cache(scaled_font);
    return _cairo_scaled_font_set_error(scaled_font, status as cairo_status_t);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_set_metrics(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut fs_metrics: *mut cairo_text_extents_t,
) {
    let mut first: cairo_bool_t = 1 as libc::c_int;
    let mut hm: libc::c_double = 0.;
    let mut wm: libc::c_double = 0.;
    let mut min_user_x: libc::c_double = 0.0f64;
    let mut max_user_x: libc::c_double = 0.0f64;
    let mut min_user_y: libc::c_double = 0.0f64;
    let mut max_user_y: libc::c_double = 0.0f64;
    let mut min_device_x: libc::c_double = 0.0f64;
    let mut max_device_x: libc::c_double = 0.0f64;
    let mut min_device_y: libc::c_double = 0.0f64;
    let mut max_device_y: libc::c_double = 0.0f64;
    let mut device_x_advance: libc::c_double = 0.;
    let mut device_y_advance: libc::c_double = 0.;
    (*scaled_glyph).fs_metrics = *fs_metrics;
    hm = 0.0f64;
    while hm <= 1.0f64 {
        wm = 0.0f64;
        while wm <= 1.0f64 {
            let mut x: libc::c_double = 0.;
            let mut y: libc::c_double = 0.;
            x = (*fs_metrics).x_bearing + (*fs_metrics).width * wm;
            y = (*fs_metrics).y_bearing + (*fs_metrics).height * hm;
            cairo_matrix_transform_point(
                &mut (*scaled_font).font_matrix,
                &mut x,
                &mut y,
            );
            if first != 0 {
                max_user_x = x;
                min_user_x = max_user_x;
                max_user_y = y;
                min_user_y = max_user_y;
            } else {
                if x < min_user_x {
                    min_user_x = x;
                }
                if x > max_user_x {
                    max_user_x = x;
                }
                if y < min_user_y {
                    min_user_y = y;
                }
                if y > max_user_y {
                    max_user_y = y;
                }
            }
            x = (*fs_metrics).x_bearing + (*fs_metrics).width * wm;
            y = (*fs_metrics).y_bearing + (*fs_metrics).height * hm;
            cairo_matrix_transform_distance(&mut (*scaled_font).scale, &mut x, &mut y);
            if first != 0 {
                max_device_x = x;
                min_device_x = max_device_x;
                max_device_y = y;
                min_device_y = max_device_y;
            } else {
                if x < min_device_x {
                    min_device_x = x;
                }
                if x > max_device_x {
                    max_device_x = x;
                }
                if y < min_device_y {
                    min_device_y = y;
                }
                if y > max_device_y {
                    max_device_y = y;
                }
            }
            first = 0 as libc::c_int;
            wm += 1.0f64;
        }
        hm += 1.0f64;
    }
    (*scaled_glyph).metrics.x_bearing = min_user_x;
    (*scaled_glyph).metrics.y_bearing = min_user_y;
    (*scaled_glyph).metrics.width = max_user_x - min_user_x;
    (*scaled_glyph).metrics.height = max_user_y - min_user_y;
    (*scaled_glyph).metrics.x_advance = (*fs_metrics).x_advance;
    (*scaled_glyph).metrics.y_advance = (*fs_metrics).y_advance;
    cairo_matrix_transform_distance(
        &mut (*scaled_font).font_matrix,
        &mut (*scaled_glyph).metrics.x_advance,
        &mut (*scaled_glyph).metrics.y_advance,
    );
    device_x_advance = (*fs_metrics).x_advance;
    device_y_advance = (*fs_metrics).y_advance;
    cairo_matrix_transform_distance(
        &mut (*scaled_font).scale,
        &mut device_x_advance,
        &mut device_y_advance,
    );
    (*scaled_glyph).bbox.p1.x = _cairo_fixed_from_double(min_device_x);
    (*scaled_glyph).bbox.p1.y = _cairo_fixed_from_double(min_device_y);
    (*scaled_glyph).bbox.p2.x = _cairo_fixed_from_double(max_device_x);
    (*scaled_glyph).bbox.p2.y = _cairo_fixed_from_double(max_device_y);
    (*scaled_glyph).x_advance = _cairo_lround(device_x_advance) as int16_t;
    (*scaled_glyph).y_advance = _cairo_lround(device_y_advance) as int16_t;
    (*scaled_glyph).has_info
        |= CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_set_surface(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut surface: *mut cairo_image_surface_t,
) {
    if !((*scaled_glyph).surface).is_null() {
        cairo_surface_destroy(&mut (*(*scaled_glyph).surface).base);
    }
    let ref mut fresh38 = (*scaled_glyph).surface;
    *fresh38 = surface;
    if !surface.is_null() {
        (*scaled_glyph).has_info
            |= CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int as libc::c_uint;
    } else {
        (*scaled_glyph).has_info
            &= !(CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int) as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_set_path(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut path: *mut cairo_path_fixed_t,
) {
    if !((*scaled_glyph).path).is_null() {
        _cairo_path_fixed_destroy((*scaled_glyph).path);
    }
    let ref mut fresh39 = (*scaled_glyph).path;
    *fresh39 = path;
    if !path.is_null() {
        (*scaled_glyph).has_info
            |= CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int as libc::c_uint;
    } else {
        (*scaled_glyph).has_info
            &= !(CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int) as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_set_recording_surface(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut recording_surface: *mut cairo_surface_t,
) {
    if !((*scaled_glyph).recording_surface).is_null() {
        cairo_surface_finish((*scaled_glyph).recording_surface);
        cairo_surface_destroy((*scaled_glyph).recording_surface);
    }
    let ref mut fresh40 = (*scaled_glyph).recording_surface;
    *fresh40 = recording_surface;
    if !recording_surface.is_null() {
        (*scaled_glyph).has_info
            |= CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE as libc::c_int as libc::c_uint;
    } else {
        (*scaled_glyph).has_info
            &= !(CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE as libc::c_int)
                as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_set_color_surface(
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut surface: *mut cairo_image_surface_t,
    mut uses_foreground_color: cairo_bool_t,
) {
    if !((*scaled_glyph).color_surface).is_null() {
        cairo_surface_destroy(&mut (*(*scaled_glyph).color_surface).base);
    }
    let ref mut fresh41 = (*scaled_glyph).color_surface;
    *fresh41 = surface;
    (*scaled_glyph).set_uses_foreground_color(uses_foreground_color as libc::c_uint);
    if !surface.is_null() {
        (*scaled_glyph).has_info
            |= CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint;
    } else {
        (*scaled_glyph).has_info
            &= !(CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int) as libc::c_uint;
    };
}
unsafe extern "C" fn _cairo_scaled_glyph_page_can_remove(
    mut closure: *const libc::c_void,
) -> cairo_bool_t {
    let mut page: *const cairo_scaled_glyph_page_t = closure
        as *const cairo_scaled_glyph_page_t;
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    scaled_font = (*page).scaled_font;
    if !(pthread_mutex_trylock(&mut (*scaled_font).mutex) == 0 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if (*scaled_font).cache_frozen != 0 as libc::c_int {
        pthread_mutex_unlock(&mut (*scaled_font).mutex);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_scaled_font_allocate_glyph(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_glyph: *mut *mut cairo_scaled_glyph_t,
) -> cairo_status_t {
    let mut page: *mut cairo_scaled_glyph_page_t = 0 as *mut cairo_scaled_glyph_page_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*scaled_font).cache_frozen != 0 {} else {
        __assert_fail(
            b"scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            2725 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_status_t _cairo_scaled_font_allocate_glyph(cairo_scaled_font_t *, cairo_scaled_glyph_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if cairo_list_is_empty(&mut (*scaled_font).glyph_pages) == 0 {
        page = ({
            let mut mptr__: *const cairo_list_t = (*scaled_font).glyph_pages.prev;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut cairo_scaled_glyph_page_t
        });
        if (*page).num_glyphs < 32 as libc::c_int as libc::c_uint {
            let ref mut fresh42 = (*page).num_glyphs;
            let fresh43 = *fresh42;
            *fresh42 = (*fresh42).wrapping_add(1);
            *scaled_glyph = &mut *((*page).glyphs).as_mut_ptr().offset(fresh43 as isize)
                as *mut cairo_scaled_glyph_t;
            return CAIRO_STATUS_SUCCESS;
        }
    }
    page = (if ::std::mem::size_of::<cairo_scaled_glyph_page_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_scaled_glyph_page_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_scaled_glyph_page_t;
    if page.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*page).cache_entry.hash = scaled_font as uintptr_t;
    let ref mut fresh44 = (*page).scaled_font;
    *fresh44 = scaled_font;
    (*page).cache_entry.size = 1 as libc::c_int as libc::c_ulong;
    (*page).num_glyphs = 0 as libc::c_int as libc::c_uint;
    pthread_mutex_lock(&mut _cairo_scaled_glyph_page_cache_mutex);
    if (*scaled_font).global_cache_frozen == 0 as libc::c_int {
        if (cairo_scaled_glyph_page_cache.hash_table).is_null() {
            status = _cairo_cache_init(
                &mut cairo_scaled_glyph_page_cache,
                None,
                Some(
                    _cairo_scaled_glyph_page_can_remove
                        as unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
                ),
                Some(
                    _cairo_scaled_glyph_page_pluck
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                512 as libc::c_int as libc::c_ulong,
            );
            if status as u64 != 0 {
                pthread_mutex_unlock(&mut _cairo_scaled_glyph_page_cache_mutex);
                free(page as *mut libc::c_void);
                return status;
            }
        }
        _cairo_cache_freeze(&mut cairo_scaled_glyph_page_cache);
        (*scaled_font).global_cache_frozen = 1 as libc::c_int;
    }
    status = _cairo_cache_insert(
        &mut cairo_scaled_glyph_page_cache,
        &mut (*page).cache_entry,
    );
    pthread_mutex_unlock(&mut _cairo_scaled_glyph_page_cache_mutex);
    if status as u64 != 0 {
        free(page as *mut libc::c_void);
        return status;
    }
    cairo_list_add_tail(&mut (*page).link, &mut (*scaled_font).glyph_pages);
    let ref mut fresh45 = (*page).num_glyphs;
    let fresh46 = *fresh45;
    *fresh45 = (*fresh45).wrapping_add(1);
    *scaled_glyph = &mut *((*page).glyphs).as_mut_ptr().offset(fresh46 as isize)
        as *mut cairo_scaled_glyph_t;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_scaled_font_free_last_glyph(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
) {
    let mut page: *mut cairo_scaled_glyph_page_t = 0 as *mut cairo_scaled_glyph_page_t;
    if (*scaled_font).cache_frozen != 0 {} else {
        __assert_fail(
            b"scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            2786 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"void _cairo_scaled_font_free_last_glyph(cairo_scaled_font_t *, cairo_scaled_glyph_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if cairo_list_is_empty(&mut (*scaled_font).glyph_pages) == 0 {} else {
        __assert_fail(
            b"! cairo_list_is_empty (&scaled_font->glyph_pages)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            2787 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"void _cairo_scaled_font_free_last_glyph(cairo_scaled_font_t *, cairo_scaled_glyph_t *)\0",
            ))
                .as_ptr(),
        );
    }
    page = ({
        let mut mptr__: *const cairo_list_t = (*scaled_font).glyph_pages.prev;
        (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
            as *mut cairo_scaled_glyph_page_t
    });
    if scaled_glyph
        == &mut *((*page).glyphs)
            .as_mut_ptr()
            .offset(
                ((*page).num_glyphs).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) as *mut cairo_scaled_glyph_t
    {} else {
        __assert_fail(
            b"scaled_glyph == &page->glyphs[page->num_glyphs-1]\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            2791 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 87],
                &[libc::c_char; 87],
            >(
                b"void _cairo_scaled_font_free_last_glyph(cairo_scaled_font_t *, cairo_scaled_glyph_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_scaled_glyph_fini(scaled_font, scaled_glyph);
    let ref mut fresh47 = (*page).num_glyphs;
    *fresh47 = (*fresh47).wrapping_sub(1);
    if *fresh47 == 0 as libc::c_int as libc::c_uint {
        _cairo_scaled_font_thaw_cache(scaled_font);
        pthread_mutex_lock(&mut (*scaled_font).mutex);
        pthread_mutex_lock(&mut _cairo_scaled_glyph_page_cache_mutex);
        cairo_scaled_glyph_page_cache.entry_destroy = None;
        _cairo_cache_remove(
            &mut cairo_scaled_glyph_page_cache,
            &mut (*page).cache_entry,
        );
        _cairo_scaled_glyph_page_destroy(scaled_font, page);
        cairo_scaled_glyph_page_cache
            .entry_destroy = Some(
            _cairo_scaled_glyph_page_pluck
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        pthread_mutex_unlock(&mut _cairo_scaled_glyph_page_cache_mutex);
        pthread_mutex_unlock(&mut (*scaled_font).mutex);
        _cairo_scaled_font_freeze_cache(scaled_font);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_glyph_lookup(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut index: libc::c_ulong,
    mut info: cairo_scaled_glyph_info_t,
    mut foreground_color: *const cairo_color_t,
    mut scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut need_info: cairo_scaled_glyph_info_t = 0 as cairo_scaled_glyph_info_t;
    let mut key: cairo_hash_entry_t = cairo_hash_entry_t { hash: 0 };
    *scaled_glyph_ret = 0 as *mut cairo_scaled_glyph_t;
    if (*scaled_font).status as u64 != 0 {
        return (*scaled_font).status as cairo_int_status_t;
    }
    if (*scaled_font).cache_frozen != 0 {} else {
        __assert_fail(
            b"scaled_font->cache_frozen\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-scaled-font.c\0" as *const u8 as *const libc::c_char,
            2862 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 159],
                &[libc::c_char; 159],
            >(
                b"cairo_int_status_t _cairo_scaled_glyph_lookup(cairo_scaled_font_t *, unsigned long, cairo_scaled_glyph_info_t, const cairo_color_t *, cairo_scaled_glyph_t **)\0",
            ))
                .as_ptr(),
        );
    }
    if foreground_color.is_null() {
        foreground_color = _cairo_stock_color(CAIRO_STOCK_BLACK);
    }
    key.hash = index;
    scaled_glyph = _cairo_hash_table_lookup((*scaled_font).glyphs, &mut key)
        as *mut cairo_scaled_glyph_t;
    if scaled_glyph.is_null() {
        status = _cairo_scaled_font_allocate_glyph(scaled_font, &mut scaled_glyph)
            as cairo_int_status_t;
        if status as u64 != 0 {
            current_block = 11446856665554274158;
        } else {
            memset(
                scaled_glyph as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<cairo_scaled_glyph_t>() as libc::c_ulong,
            );
            (*scaled_glyph).hash_entry.hash = index;
            cairo_list_init(&mut (*scaled_glyph).dev_privates);
            status = ((*(*scaled_font).backend).scaled_glyph_init)
                .expect(
                    "non-null function pointer",
                )(
                scaled_font as *mut libc::c_void,
                scaled_glyph,
                (info as libc::c_uint
                    | CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int as libc::c_uint)
                    as cairo_scaled_glyph_info_t,
                foreground_color,
            );
            if status as u64 != 0 {
                _cairo_scaled_font_free_last_glyph(scaled_font, scaled_glyph);
                current_block = 11446856665554274158;
            } else {
                status = _cairo_hash_table_insert(
                    (*scaled_font).glyphs,
                    &mut (*scaled_glyph).hash_entry,
                ) as cairo_int_status_t;
                if status as u64 != 0 {
                    _cairo_scaled_font_free_last_glyph(scaled_font, scaled_glyph);
                    current_block = 11446856665554274158;
                } else {
                    current_block = 11194104282611034094;
                }
            }
        }
    } else {
        current_block = 11194104282611034094;
    }
    match current_block {
        11194104282611034094 => {
            need_info = (info as libc::c_uint & !(*scaled_glyph).has_info)
                as cairo_scaled_glyph_info_t;
            if need_info as libc::c_uint
                & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
                != 0 && (*scaled_glyph).color_glyph_set() as libc::c_int != 0
                && (*scaled_glyph).color_glyph() == 0
            {
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            if info as libc::c_uint
                & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
                != 0 && (*scaled_glyph).uses_foreground_color() as libc::c_int != 0
                && _cairo_color_equal(
                    foreground_color,
                    &mut (*scaled_glyph).foreground_color,
                ) == 0
            {
                need_info = ::std::mem::transmute::<
                    libc::c_uint,
                    cairo_scaled_glyph_info_t,
                >(
                    need_info as libc::c_uint
                        | CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int
                            as libc::c_uint,
                );
            }
            if need_info as u64 != 0 {
                status = ((*(*scaled_font).backend).scaled_glyph_init)
                    .expect(
                        "non-null function pointer",
                    )(
                    scaled_font as *mut libc::c_void,
                    scaled_glyph,
                    need_info,
                    foreground_color,
                );
                if status as u64 != 0 {
                    current_block = 11446856665554274158;
                } else {
                    if info as libc::c_uint & !(*scaled_glyph).has_info != 0 {
                        return CAIRO_INT_STATUS_UNSUPPORTED;
                    }
                    current_block = 9853141518545631134;
                }
            } else {
                current_block = 9853141518545631134;
            }
            match current_block {
                11446856665554274158 => {}
                _ => {
                    *scaled_glyph_ret = scaled_glyph;
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
            }
        }
        _ => {}
    }
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        status = _cairo_scaled_font_set_error(scaled_font, status as cairo_status_t)
            as cairo_int_status_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_get_max_scale(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> libc::c_double {
    return (*scaled_font).max_scale;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_font_face(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> *mut cairo_font_face_t {
    if (*scaled_font).status as u64 != 0 {
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    if !((*scaled_font).original_font_face).is_null() {
        return (*scaled_font).original_font_face;
    }
    return (*scaled_font).font_face;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_font_matrix(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut font_matrix: *mut cairo_matrix_t,
) {
    if (*scaled_font).status as u64 != 0 {
        cairo_matrix_init_identity(font_matrix);
        return;
    }
    *font_matrix = (*scaled_font).font_matrix;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_ctm(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut ctm: *mut cairo_matrix_t,
) {
    if (*scaled_font).status as u64 != 0 {
        cairo_matrix_init_identity(ctm);
        return;
    }
    *ctm = (*scaled_font).ctm;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_scale_matrix(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scale_matrix: *mut cairo_matrix_t,
) {
    if (*scaled_font).status as u64 != 0 {
        cairo_matrix_init_identity(scale_matrix);
        return;
    }
    *scale_matrix = (*scaled_font).scale;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_scaled_font_get_font_options(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut options: *mut cairo_font_options_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    if (*scaled_font).status as u64 != 0 {
        _cairo_font_options_init_default(options);
        return;
    }
    _cairo_font_options_init_copy(options, &mut (*scaled_font).options);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_has_color_glyphs(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_bool_t {
    if !((*scaled_font).backend).is_null()
        && ((*(*scaled_font).backend).has_color_glyphs).is_some()
    {
        return ((*(*scaled_font).backend).has_color_glyphs)
            .expect("non-null function pointer")(scaled_font as *mut libc::c_void)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn run_static_initializers() {
    _cairo_scaled_font_nil = {
        let mut init = _cairo_scaled_font {
            placeholder_holdover_finished: [0; 1],
            c2rust_padding: [0; 7],
            hash_entry: {
                let mut init = _cairo_hash_entry {
                    hash: 0 as libc::c_int as uintptr_t,
                };
                init
            },
            status: CAIRO_STATUS_NO_MEMORY,
            ref_count: {
                let mut init = cairo_reference_count_t {
                    ref_count: -(1 as libc::c_int),
                };
                init
            },
            user_data: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: 0 as libc::c_int as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            original_font_face: 0 as *mut cairo_font_face_t,
            font_face: 0 as *mut cairo_font_face_t,
            font_matrix: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0 as libc::c_int as libc::c_double,
                    y0: 0 as libc::c_int as libc::c_double,
                };
                init
            },
            ctm: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0 as libc::c_int as libc::c_double,
                    y0: 0 as libc::c_int as libc::c_double,
                };
                init
            },
            options: {
                let mut init = _cairo_font_options {
                    antialias: CAIRO_ANTIALIAS_DEFAULT,
                    subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
                    lcd_filter: CAIRO_HINT_STYLE_DEFAULT as libc::c_int
                        as cairo_lcd_filter_t,
                    hint_style: CAIRO_HINT_METRICS_DEFAULT as libc::c_int
                        as cairo_hint_style_t,
                    hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
                    round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
                    variations: 0 as *mut libc::c_char,
                    color_mode: CAIRO_COLOR_MODE_DEFAULT,
                    palette_index: 0,
                };
                init
            },
            scale: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0 as libc::c_int as libc::c_double,
                    y0: 0 as libc::c_int as libc::c_double,
                };
                init
            },
            scale_inverse: {
                let mut init = _cairo_matrix {
                    xx: 1.0f64,
                    yx: 0.0f64,
                    xy: 0.0f64,
                    yy: 1.0f64,
                    x0: 0 as libc::c_int as libc::c_double,
                    y0: 0 as libc::c_int as libc::c_double,
                };
                init
            },
            max_scale: 1.0f64,
            extents: {
                let mut init = cairo_font_extents_t {
                    ascent: 0.0f64,
                    descent: 0.0f64,
                    height: 0.0f64,
                    max_x_advance: 0.0f64,
                    max_y_advance: 0.0f64,
                };
                init
            },
            fs_extents: {
                let mut init = cairo_font_extents_t {
                    ascent: 0.0f64,
                    descent: 0.0f64,
                    height: 0.0f64,
                    max_x_advance: 0.0f64,
                    max_y_advance: 0.0f64,
                };
                init
            },
            mutex: pthread_mutex_t {
                __data: {
                    let mut init = __pthread_mutex_s {
                        __lock: 0 as libc::c_int,
                        __count: 0 as libc::c_int as libc::c_uint,
                        __owner: 0 as libc::c_int,
                        __nusers: 0 as libc::c_int as libc::c_uint,
                        __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                        __spins: 0 as libc::c_int as libc::c_short,
                        __elision: 0 as libc::c_int as libc::c_short,
                        __list: {
                            let mut init = __pthread_internal_list {
                                __prev: 0 as *mut __pthread_internal_list,
                                __next: 0 as *mut __pthread_internal_list,
                            };
                            init
                        },
                    };
                    init
                },
            },
            glyphs: 0 as *mut cairo_hash_table_t,
            glyph_pages: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            cache_frozen: 0 as libc::c_int,
            global_cache_frozen: 0 as libc::c_int,
            recording_surfaces_to_free: {
                let mut init = _cairo_array {
                    size: 0 as libc::c_int as libc::c_uint,
                    num_elements: 0 as libc::c_int as libc::c_uint,
                    element_size: ::std::mem::size_of::<*mut cairo_surface_t>()
                        as libc::c_ulong as libc::c_uint,
                    elements: 0 as *mut libc::c_char,
                };
                init
            },
            dev_privates: {
                let mut init = _cairo_list {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                };
                init
            },
            backend: 0 as *const cairo_scaled_font_backend_t,
            link: cairo_list_t {
                next: 0 as *mut _cairo_list,
                prev: 0 as *mut _cairo_list,
            },
        };
        init.set_placeholder(0 as libc::c_int as libc::c_uint);
        init.set_holdover(0 as libc::c_int as libc::c_uint);
        init.set_finished(1 as libc::c_int as libc::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
