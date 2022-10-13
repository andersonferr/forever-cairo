use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _cairo_scaled_font_subsets;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t,
        antialias: cairo_antialias_t,
    );
    fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t,
        hint_style: cairo_hint_style_t,
    );
    fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t,
        hint_metrics: cairo_hint_metrics_t,
    );
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    fn cairo_surface_write_to_png_stream(
        surface: *mut cairo_surface_t,
        write_func: cairo_write_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn cairo_surface_get_mime_data(
        surface: *mut cairo_surface_t,
        mime_type: *const libc::c_char,
        data: *mut *const libc::c_uchar,
        length: *mut libc::c_ulong,
    );
    fn cairo_surface_set_fallback_resolution(
        surface: *mut cairo_surface_t,
        x_pixels_per_inch: libc::c_double,
        y_pixels_per_inch: libc::c_double,
    );
    fn cairo_surface_show_page(surface: *mut cairo_surface_t);
    fn cairo_pattern_create_for_surface(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_pattern_t;
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
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
    fn _cairo_path_fixed_size(path: *const cairo_path_fixed_t) -> libc::c_ulong;
    fn _cairo_surface_default_source(
        surface: *mut libc::c_void,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_hash_bytes(
        hash: uintptr_t,
        bytes: *const libc::c_void,
        length: libc::c_uint,
    ) -> uintptr_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static _cairo_pattern_clear: cairo_solid_pattern_t;
    static _cairo_pattern_black: cairo_solid_pattern_t;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_color_equal(
        color_a: *const cairo_color_t,
        color_b: *const cairo_color_t,
    ) -> cairo_bool_t;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_font_options_set_round_glyph_positions(
        options: *mut cairo_font_options_t,
        round: cairo_round_glyph_positions_t,
    );
    fn _cairo_path_fixed_init(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_is_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_glyph_path(
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *const cairo_glyph_t,
        num_glyphs: libc::c_int,
        path: *mut cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_surface_acquire_source_image(
        surface: *mut cairo_surface_t,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_surface_release_source_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
    );
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
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_truncate(array: *mut cairo_array_t, num_elements: libc::c_uint);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_image_info_get_jpeg_info(
        info: *mut cairo_image_info_t,
        data: *const libc::c_uchar,
        length: libc::c_ulong,
    ) -> cairo_int_status_t;
    fn _cairo_recording_surface_replay(
        surface: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_create(
        write_func: cairo_write_func_t,
        close_func: cairo_close_func_t,
        closure: *mut libc::c_void,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_destroy(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_write(
        stream: *mut cairo_output_stream_t,
        data: *const libc::c_void,
        length: size_t,
    );
    fn _cairo_output_stream_vprintf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        ap: ::std::ffi::VaList,
    );
    fn _cairo_output_stream_printf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_create_for_filename(
        filename: *const libc::c_char,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_copy(
        base: *mut cairo_output_stream_t,
        dest: *mut cairo_output_stream_t,
    );
    fn _cairo_paginated_surface_create(
        target: *mut cairo_surface_t,
        content: cairo_content_t,
        backend: *const cairo_paginated_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_paginated_surface_get_target(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_paginated_surface_get_recording(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_is_paginated(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn _cairo_scaled_font_subsets_create_scaled() -> *mut cairo_scaled_font_subsets_t;
    fn _cairo_scaled_font_subsets_destroy(
        font_subsets: *mut cairo_scaled_font_subsets_t,
    );
    fn _cairo_scaled_font_subsets_map_glyph(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        scaled_font: *mut cairo_scaled_font_t,
        scaled_font_glyph_index: libc::c_ulong,
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        subset_glyph_ret: *mut cairo_scaled_font_subsets_glyph_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_subsets_foreach_scaled(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        font_subset_callback: cairo_scaled_font_subset_callback_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_subsets_foreach_user(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        font_subset_callback: cairo_scaled_font_subset_callback_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_set_clip(
        clipper: *mut cairo_surface_clipper_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_init(
        clipper: *mut cairo_surface_clipper_t,
        intersect: cairo_surface_clipper_intersect_clip_path_func_t,
    );
    fn _cairo_surface_clipper_reset(clipper: *mut cairo_surface_clipper_t);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
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
pub type cairo_output_stream_t = _cairo_output_stream;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_paginated_surface_backend {
    pub start_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub set_paginated_mode: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_paginated_mode_t,
        ) -> cairo_int_status_t,
    >,
    pub set_bounding_box: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_box_t) -> cairo_int_status_t,
    >,
    pub set_fallback_images_required: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_bool_t) -> cairo_int_status_t,
    >,
    pub supports_fine_grained_fallbacks: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
    pub requires_thumbnail_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> cairo_bool_t,
    >,
    pub set_thumbnail_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
        ) -> cairo_int_status_t,
    >,
}
pub type cairo_paginated_mode_t = _cairo_paginated_mode;
pub type _cairo_paginated_mode = libc::c_uint;
pub const CAIRO_PAGINATED_MODE_FALLBACK: _cairo_paginated_mode = 2;
pub const CAIRO_PAGINATED_MODE_RENDER: _cairo_paginated_mode = 1;
pub const CAIRO_PAGINATED_MODE_ANALYZE: _cairo_paginated_mode = 0;
pub type cairo_paginated_surface_backend_t = _cairo_paginated_surface_backend;
pub type cairo_scaled_font_subsets_t = _cairo_scaled_font_subsets;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_snapshot {
    pub base: cairo_surface_t,
    pub mutex: cairo_mutex_t,
    pub target: *mut cairo_surface_t,
    pub clone: *mut cairo_surface_t,
}
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_surface_snapshot_t = _cairo_surface_snapshot;
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
pub struct _cairo_box_double {
    pub p1: cairo_point_double_t,
    pub p2: cairo_point_double_t,
}
pub type cairo_box_double_t = _cairo_box_double;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_subset {
    pub scaled_font: *mut cairo_scaled_font_t,
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub glyphs: *mut libc::c_ulong,
    pub utf8: *mut *mut libc::c_char,
    pub glyph_names: *mut *mut libc::c_char,
    pub to_latin_char: *mut libc::c_int,
    pub latin_to_subset_glyph_index: *mut libc::c_ulong,
    pub num_glyphs: libc::c_uint,
    pub is_composite: cairo_bool_t,
    pub is_scaled: cairo_bool_t,
    pub is_latin: cairo_bool_t,
}
pub type cairo_scaled_font_subset_t = _cairo_scaled_font_subset;
pub type cairo_path_fixed_move_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_line_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_curve_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
    *const cairo_point_t,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_close_path_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> cairo_status_t;
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type _cairo_svg_version = libc::c_uint;
pub const CAIRO_SVG_VERSION_1_2: _cairo_svg_version = 1;
pub const CAIRO_SVG_VERSION_1_1: _cairo_svg_version = 0;
pub type cairo_svg_version_t = _cairo_svg_version;
pub type _cairo_svg_unit = libc::c_uint;
pub const CAIRO_SVG_UNIT_PERCENT: _cairo_svg_unit = 9;
pub const CAIRO_SVG_UNIT_PC: _cairo_svg_unit = 8;
pub const CAIRO_SVG_UNIT_PT: _cairo_svg_unit = 7;
pub const CAIRO_SVG_UNIT_MM: _cairo_svg_unit = 6;
pub const CAIRO_SVG_UNIT_CM: _cairo_svg_unit = 5;
pub const CAIRO_SVG_UNIT_IN: _cairo_svg_unit = 4;
pub const CAIRO_SVG_UNIT_PX: _cairo_svg_unit = 3;
pub const CAIRO_SVG_UNIT_EX: _cairo_svg_unit = 2;
pub const CAIRO_SVG_UNIT_EM: _cairo_svg_unit = 1;
pub const CAIRO_SVG_UNIT_USER: _cairo_svg_unit = 0;
pub type cairo_svg_unit_t = _cairo_svg_unit;
pub type cairo_svg_document_t = _cairo_svg_document;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_document {
    pub output_stream: *mut cairo_output_stream_t,
    pub refcount: libc::c_ulong,
    pub owner: *mut cairo_surface_t,
    pub finished: cairo_bool_t,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub unit: cairo_svg_unit_t,
    pub xml_node_defs: cairo_svg_stream_t,
    pub xml_node_glyphs: cairo_svg_stream_t,
    pub xml_node_filters: cairo_svg_stream_t,
    pub linear_pattern_id: libc::c_uint,
    pub radial_pattern_id: libc::c_uint,
    pub pattern_id: libc::c_uint,
    pub clip_id: libc::c_uint,
    pub mask_id: libc::c_uint,
    pub compositing_group_id: libc::c_uint,
    pub filter_id: libc::c_uint,
    pub filters_emitted: [cairo_bool_t; 3],
    pub svg_version: cairo_svg_version_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub paints: *mut cairo_hash_table_t,
}
pub type cairo_svg_stream_t = _cairo_svg_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_stream {
    pub status: cairo_status_t,
    pub elements: cairo_array_t,
}
pub type cairo_svg_paint_t = _cairo_svg_paint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_paint {
    pub base: cairo_hash_entry_t,
    pub source_id: libc::c_uint,
    pub paint_elements: cairo_array_t,
    pub box_0: cairo_box_double_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub output_stream: *mut cairo_output_stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub text: C2RustUnnamed,
    pub paint_dependent: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub source_id: libc::c_uint,
    pub type_0: cairo_svg_stream_paint_dependent_element_type,
}
pub type cairo_svg_stream_paint_dependent_element_type = libc::c_uint;
pub const CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION: cairo_svg_stream_paint_dependent_element_type = 3;
pub const CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_TRANSLATION: cairo_svg_stream_paint_dependent_element_type = 2;
pub const CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN: cairo_svg_stream_paint_dependent_element_type = 1;
pub const CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE: cairo_svg_stream_paint_dependent_element_type = 0;
pub type cairo_svg_stream_element_t = _cairo_svg_stream_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_stream_element {
    pub type_0: cairo_svg_stream_element_type,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
pub type cairo_svg_stream_element_type = libc::c_uint;
pub const CAIRO_SVG_STREAM_ELEMENT_TYPE_PAINT_DEPENDENT: cairo_svg_stream_element_type = 1;
pub const CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT: cairo_svg_stream_element_type = 0;
pub type cairo_svg_page_t = _cairo_svg_page;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_page {
    pub xml_node: cairo_svg_stream_t,
}
pub type cairo_svg_surface_t = _cairo_svg_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_surface {
    pub base: cairo_surface_t,
    pub force_fallbacks: cairo_bool_t,
    pub source_id: libc::c_uint,
    pub depth: libc::c_uint,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub surface_bounded: cairo_bool_t,
    pub document: *mut cairo_svg_document_t,
    pub xml_node: cairo_svg_stream_t,
    pub page_set: cairo_array_t,
    pub source_surfaces: *mut cairo_hash_table_t,
    pub clipper: cairo_surface_clipper_t,
    pub current_clipper_stream: *mut cairo_svg_stream_t,
    pub clip_level: libc::c_uint,
    pub transitive_paint_used: cairo_bool_t,
    pub paginated_mode: cairo_paginated_mode_t,
}
pub type cairo_surface_clipper_t = _cairo_surface_clipper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_clipper {
    pub clip: *mut cairo_clip_t,
    pub intersect_clip_path: cairo_surface_clipper_intersect_clip_path_func_t,
}
pub type cairo_surface_clipper_intersect_clip_path_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_surface_clipper_t,
        *mut cairo_path_fixed_t,
        cairo_fill_rule_t,
        libc::c_double,
        cairo_antialias_t,
    ) -> cairo_status_t,
>;
pub type cairo_svg_paint_element_t = _cairo_svg_paint_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_paint_element {
    pub source_id: libc::c_uint,
    pub matrix: cairo_matrix_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svg_path_info_t {
    pub output: *mut cairo_svg_stream_t,
    pub ctm_inverse: *const cairo_matrix_t,
}
pub type cairo_svg_source_surface_t = _cairo_svg_source_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_svg_source_surface {
    pub base: cairo_hash_entry_t,
    pub id: libc::c_uint,
    pub unique_id: *mut libc::c_uchar,
    pub unique_id_length: libc::c_ulong,
    pub transitive_paint_used: cairo_bool_t,
}
pub type cairo_radial_pattern_t = _cairo_radial_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_radial_pattern {
    pub base: cairo_gradient_pattern_t,
    pub cd1: cairo_circle_double_t,
    pub cd2: cairo_circle_double_t,
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
pub type cairo_svg_filter = libc::c_uint;
pub const CAIRO_SVG_FILTER_LUMINOSITY: cairo_svg_filter = 24;
pub const CAIRO_SVG_FILTER_COLOR: cairo_svg_filter = 23;
pub const CAIRO_SVG_FILTER_SATURATION: cairo_svg_filter = 22;
pub const CAIRO_SVG_FILTER_HUE: cairo_svg_filter = 21;
pub const CAIRO_SVG_FILTER_EXCLUSION: cairo_svg_filter = 20;
pub const CAIRO_SVG_FILTER_DIFFERENCE: cairo_svg_filter = 19;
pub const CAIRO_SVG_FILTER_SOFT_LIGHT: cairo_svg_filter = 18;
pub const CAIRO_SVG_FILTER_HARD_LIGHT: cairo_svg_filter = 17;
pub const CAIRO_SVG_FILTER_COLOR_BURN: cairo_svg_filter = 16;
pub const CAIRO_SVG_FILTER_COLOR_DODGE: cairo_svg_filter = 15;
pub const CAIRO_SVG_FILTER_LIGHTEN: cairo_svg_filter = 14;
pub const CAIRO_SVG_FILTER_DARKEN: cairo_svg_filter = 13;
pub const CAIRO_SVG_FILTER_OVERLAY: cairo_svg_filter = 12;
pub const CAIRO_SVG_FILTER_SCREEN: cairo_svg_filter = 11;
pub const CAIRO_SVG_FILTER_MULTIPLY: cairo_svg_filter = 10;
pub const CAIRO_SVG_FILTER_ADD: cairo_svg_filter = 9;
pub const CAIRO_SVG_FILTER_XOR: cairo_svg_filter = 8;
pub const CAIRO_SVG_FILTER_ATOP: cairo_svg_filter = 7;
pub const CAIRO_SVG_FILTER_OUT: cairo_svg_filter = 6;
pub const CAIRO_SVG_FILTER_IN: cairo_svg_filter = 5;
pub const CAIRO_SVG_FILTER_OVER: cairo_svg_filter = 4;
pub const CAIRO_SVG_FILTER_LAST_STATIC_FILTER: cairo_svg_filter = 3;
pub const CAIRO_SVG_FILTER_COLOR_TO_ALPHA: cairo_svg_filter = 2;
pub const CAIRO_SVG_FILTER_REMOVE_COLOR_AND_INVERT_ALPHA: cairo_svg_filter = 1;
pub const CAIRO_SVG_FILTER_REMOVE_COLOR: cairo_svg_filter = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_write_closure_t {
    pub output: *mut cairo_svg_stream_t,
    pub in_mem: libc::c_uint,
    pub trailing: libc::c_uint,
    pub src: [libc::c_uchar; 3],
}
pub type cairo_image_info_t = _cairo_image_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_image_info {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub num_components: libc::c_int,
    pub bits_per_component: libc::c_int,
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
pub type cairo_scaled_font_subsets_glyph_t = _cairo_scaled_font_subsets_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_subsets_glyph {
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub subset_glyph_index: libc::c_uint,
    pub is_scaled: cairo_bool_t,
    pub is_composite: cairo_bool_t,
    pub is_latin: cairo_bool_t,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
    pub utf8_is_mapped: cairo_bool_t,
    pub unicode: uint32_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_gradient_pattern_union_t {
    pub base: cairo_gradient_pattern_t,
    pub linear: cairo_linear_pattern_t,
    pub radial: cairo_radial_pattern_t,
}
pub type cairo_command_mask_t = _cairo_command_mask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_mask {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub mask: cairo_pattern_union_t,
}
pub type cairo_command_t = _cairo_command;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _cairo_command {
    pub header: cairo_command_header_t,
    pub paint: cairo_command_paint_t,
    pub mask: cairo_command_mask_t,
    pub stroke: cairo_command_stroke_t,
    pub fill: cairo_command_fill_t,
    pub show_text_glyphs: cairo_command_show_text_glyphs_t,
    pub tag: cairo_command_tag_t,
}
pub type cairo_command_tag_t = _cairo_command_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_tag {
    pub header: cairo_command_header_t,
    pub begin: cairo_bool_t,
    pub tag_name: *mut libc::c_char,
    pub attributes: *mut libc::c_char,
}
pub type cairo_command_show_text_glyphs_t = _cairo_command_show_text_glyphs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_show_text_glyphs {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub utf8: *mut libc::c_char,
    pub utf8_len: libc::c_int,
    pub glyphs: *mut cairo_glyph_t,
    pub num_glyphs: libc::c_uint,
    pub clusters: *mut cairo_text_cluster_t,
    pub num_clusters: libc::c_int,
    pub cluster_flags: cairo_text_cluster_flags_t,
    pub scaled_font: *mut cairo_scaled_font_t,
}
pub type cairo_command_fill_t = _cairo_command_fill;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_fill {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub path: cairo_path_fixed_t,
    pub fill_rule: cairo_fill_rule_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
}
pub type cairo_command_stroke_t = _cairo_command_stroke;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_stroke {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
    pub path: cairo_path_fixed_t,
    pub style: cairo_stroke_style_t,
    pub ctm: cairo_matrix_t,
    pub ctm_inverse: cairo_matrix_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
}
pub type cairo_command_paint_t = _cairo_command_paint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_paint {
    pub header: cairo_command_header_t,
    pub source: cairo_pattern_union_t,
}
pub type cairo_scaled_font_subset_callback_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_subset_t,
        *mut libc::c_void,
    ) -> cairo_int_status_t,
>;
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
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
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
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
unsafe extern "C" fn _cairo_surface_snapshot_get_target(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut snapshot: *mut cairo_surface_snapshot_t = surface
        as *mut cairo_surface_snapshot_t;
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    pthread_mutex_lock(&mut (*snapshot).mutex);
    target = _cairo_surface_reference((*snapshot).target);
    pthread_mutex_unlock(&mut (*snapshot).mutex);
    return target;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_snapshot(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT as libc::c_int as cairo_surface_type_t
            as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_reference(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
        == -(1 as libc::c_int))
    {
        ::std::intrinsics::atomic_xadd(
            &mut (*surface).ref_count.ref_count,
            1 as libc::c_int,
        );
    }
    return surface;
}
unsafe extern "C" fn _cairo_svg_source_surface_init_key(
    mut source_surface: *mut cairo_svg_source_surface_t,
) {
    if !((*source_surface).unique_id).is_null()
        && (*source_surface).unique_id_length > 0 as libc::c_int as libc::c_ulong
    {
        (*source_surface)
            .base
            .hash = _cairo_hash_bytes(
            5381 as libc::c_int as uintptr_t,
            (*source_surface).unique_id as *const libc::c_void,
            (*source_surface).unique_id_length as libc::c_uint,
        );
    } else {
        (*source_surface).base.hash = (*source_surface).id as uintptr_t;
    };
}
unsafe extern "C" fn _cairo_svg_source_surface_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_svg_source_surface_t = key_a
        as *const cairo_svg_source_surface_t;
    let mut b: *const cairo_svg_source_surface_t = key_b
        as *const cairo_svg_source_surface_t;
    if !((*a).unique_id).is_null() && !((*b).unique_id).is_null()
        && (*a).unique_id_length == (*b).unique_id_length
    {
        return (memcmp(
            (*a).unique_id as *const libc::c_void,
            (*b).unique_id as *const libc::c_void,
            (*a).unique_id_length,
        ) == 0 as libc::c_int) as libc::c_int;
    }
    return ((*a).id == (*b).id) as libc::c_int;
}
unsafe extern "C" fn _cairo_svg_source_surface_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut source_surface: *mut cairo_svg_source_surface_t = entry
        as *mut cairo_svg_source_surface_t;
    let mut patterns: *mut cairo_hash_table_t = closure as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(patterns, &mut (*source_surface).base);
    free((*source_surface).unique_id as *mut libc::c_void);
    free(source_surface as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_svg_paint_init_key(mut paint: *mut cairo_svg_paint_t) {
    (*paint).base.hash = (*paint).source_id as uintptr_t;
}
unsafe extern "C" fn _cairo_svg_paint_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_svg_paint_t = key_a as *const cairo_svg_paint_t;
    let mut b: *const cairo_svg_paint_t = key_b as *const cairo_svg_paint_t;
    return ((*a).source_id == (*b).source_id) as libc::c_int;
}
unsafe extern "C" fn _cairo_svg_paint_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut paint: *mut cairo_svg_paint_t = entry as *mut cairo_svg_paint_t;
    let mut patterns: *mut cairo_hash_table_t = closure as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(patterns, &mut (*paint).base);
    _cairo_array_fini(&mut (*paint).paint_elements);
    free(paint as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_svg_paint_box_add_padding(
    mut box_0: *mut cairo_box_double_t,
) {
    let mut width: libc::c_double = (*box_0).p2.x - (*box_0).p1.x;
    let mut height: libc::c_double = (*box_0).p2.y - (*box_0).p1.y;
    (*box_0).p1.x -= width / 10.0f64;
    (*box_0).p1.y -= height / 10.0f64;
    (*box_0).p2.x += width / 10.0f64;
    (*box_0).p2.y += height / 10.0f64;
}
unsafe extern "C" fn _cairo_svg_stream_create() -> cairo_svg_stream_t {
    let mut svg_stream: cairo_svg_stream_t = cairo_svg_stream_t {
        status: CAIRO_STATUS_SUCCESS,
        elements: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
    };
    svg_stream.status = CAIRO_STATUS_SUCCESS;
    _cairo_array_init(
        &mut svg_stream.elements,
        ::std::mem::size_of::<cairo_svg_stream_element_t>() as libc::c_ulong
            as libc::c_uint,
    );
    return svg_stream;
}
unsafe extern "C" fn _cairo_svg_stream_write(
    mut svg_stream: *mut cairo_svg_stream_t,
    mut data: *const libc::c_void,
    mut length: size_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut last_element: *mut cairo_svg_stream_element_t = 0
        as *mut cairo_svg_stream_element_t;
    if (*svg_stream).elements.num_elements > 0 as libc::c_int as libc::c_uint {
        last_element = _cairo_array_index(
            &mut (*svg_stream).elements,
            ((*svg_stream).elements.num_elements)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_svg_stream_element_t;
    }
    if last_element.is_null()
        || (*last_element).type_0 as libc::c_uint
            != CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT as libc::c_int as libc::c_uint
    {
        let mut element: cairo_svg_stream_element_t = cairo_svg_stream_element_t {
            type_0: CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT,
            c2rust_unnamed: C2RustUnnamed_0 {
                text: C2RustUnnamed {
                    output_stream: 0 as *mut cairo_output_stream_t,
                },
            },
        };
        element.type_0 = CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT;
        element.c2rust_unnamed.text.output_stream = _cairo_memory_stream_create();
        status = _cairo_array_append(
            &mut (*svg_stream).elements,
            &mut element as *mut cairo_svg_stream_element_t as *const libc::c_void,
        );
        if status as u64 != 0 {
            if (*svg_stream).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                (*svg_stream).status = status;
            }
            return;
        }
        last_element = _cairo_array_index(
            &mut (*svg_stream).elements,
            ((*svg_stream).elements.num_elements)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_svg_stream_element_t;
    }
    _cairo_output_stream_write(
        (*last_element).c2rust_unnamed.text.output_stream,
        data,
        length,
    );
}
unsafe extern "C" fn _cairo_svg_stream_printf(
    mut svg_stream: *mut cairo_svg_stream_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut last_element: *mut cairo_svg_stream_element_t = 0
        as *mut cairo_svg_stream_element_t;
    if (*svg_stream).elements.num_elements > 0 as libc::c_int as libc::c_uint {
        last_element = _cairo_array_index(
            &mut (*svg_stream).elements,
            ((*svg_stream).elements.num_elements)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_svg_stream_element_t;
    }
    if last_element.is_null()
        || (*last_element).type_0 as libc::c_uint
            != CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT as libc::c_int as libc::c_uint
    {
        let mut element: cairo_svg_stream_element_t = cairo_svg_stream_element_t {
            type_0: CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT,
            c2rust_unnamed: C2RustUnnamed_0 {
                text: C2RustUnnamed {
                    output_stream: 0 as *mut cairo_output_stream_t,
                },
            },
        };
        element.type_0 = CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT;
        element.c2rust_unnamed.text.output_stream = _cairo_memory_stream_create();
        status = _cairo_array_append(
            &mut (*svg_stream).elements,
            &mut element as *mut cairo_svg_stream_element_t as *const libc::c_void,
        );
        if status as u64 != 0 {
            if (*svg_stream).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                (*svg_stream).status = status;
            }
            return;
        }
        last_element = _cairo_array_index(
            &mut (*svg_stream).elements,
            ((*svg_stream).elements.num_elements)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_svg_stream_element_t;
    }
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    _cairo_output_stream_vprintf(
        (*last_element).c2rust_unnamed.text.output_stream,
        fmt,
        ap.as_va_list(),
    );
}
unsafe extern "C" fn _cairo_svg_stream_append_paint_dependent(
    mut svg_stream: *mut cairo_svg_stream_t,
    mut source_id: libc::c_uint,
    mut type_0: cairo_svg_stream_paint_dependent_element_type,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut element: cairo_svg_stream_element_t = cairo_svg_stream_element_t {
        type_0: CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT,
        c2rust_unnamed: C2RustUnnamed_0 {
            text: C2RustUnnamed {
                output_stream: 0 as *mut cairo_output_stream_t,
            },
        },
    };
    element.type_0 = CAIRO_SVG_STREAM_ELEMENT_TYPE_PAINT_DEPENDENT;
    element.c2rust_unnamed.paint_dependent.source_id = source_id;
    element.c2rust_unnamed.paint_dependent.type_0 = type_0;
    status = _cairo_array_append(
        &mut (*svg_stream).elements,
        &mut element as *mut cairo_svg_stream_element_t as *const libc::c_void,
    );
    if (*svg_stream).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        (*svg_stream).status = status;
    }
}
unsafe extern "C" fn _cairo_svg_stream_copy(
    mut from: *mut cairo_svg_stream_t,
    mut to: *mut cairo_svg_stream_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*from).status as u64 != 0 {
        if (*to).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            (*to).status = (*from).status;
        }
        return;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*from).elements.num_elements {
        let mut element: *mut cairo_svg_stream_element_t = _cairo_array_index(
            &mut (*from).elements,
            i,
        ) as *mut cairo_svg_stream_element_t;
        let mut element_copy: cairo_svg_stream_element_t = *element;
        if (*element).type_0 as libc::c_uint
            == CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT as libc::c_int as libc::c_uint
        {
            element_copy
                .c2rust_unnamed
                .text
                .output_stream = _cairo_memory_stream_create();
            _cairo_memory_stream_copy(
                (*element).c2rust_unnamed.text.output_stream,
                element_copy.c2rust_unnamed.text.output_stream,
            );
            if (*to).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                (*to).status = (*(*element).c2rust_unnamed.text.output_stream).status;
            }
        }
        status = _cairo_array_append(
            &mut (*to).elements,
            &mut element_copy as *mut cairo_svg_stream_element_t as *const libc::c_void,
        );
        if status as u64 != 0 {
            if (*to).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                (*to).status = status;
            }
            return;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn _cairo_svg_stream_copy_to_output_stream(
    mut from: *mut cairo_svg_stream_t,
    mut to: *mut cairo_output_stream_t,
    mut paints: *mut cairo_hash_table_t,
) {
    if (*from).status as u64 != 0 {
        if (*to).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            (*to).status = (*from).status;
        }
        return;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*from).elements.num_elements {
        let mut element: *mut cairo_svg_stream_element_t = _cairo_array_index(
            &mut (*from).elements,
            i,
        ) as *mut cairo_svg_stream_element_t;
        if (*element).type_0 as libc::c_uint
            == CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT as libc::c_int as libc::c_uint
        {
            _cairo_memory_stream_copy((*element).c2rust_unnamed.text.output_stream, to);
        }
        if (*element).type_0 as libc::c_uint
            == CAIRO_SVG_STREAM_ELEMENT_TYPE_PAINT_DEPENDENT as libc::c_int
                as libc::c_uint
        {
            let mut paint_key: cairo_svg_paint_t = cairo_svg_paint_t {
                base: cairo_hash_entry_t { hash: 0 },
                source_id: 0,
                paint_elements: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *const libc::c_char as *mut libc::c_char,
                },
                box_0: cairo_box_double_t {
                    p1: cairo_point_double_t {
                        x: 0.,
                        y: 0.,
                    },
                    p2: cairo_point_double_t {
                        x: 0.,
                        y: 0.,
                    },
                },
            };
            paint_key.source_id = (*element).c2rust_unnamed.paint_dependent.source_id;
            _cairo_svg_paint_init_key(&mut paint_key);
            let mut found_paint_entry: *mut cairo_svg_paint_t = _cairo_hash_table_lookup(
                paints,
                &mut paint_key.base,
            ) as *mut cairo_svg_paint_t;
            if !found_paint_entry.is_null() {} else {
                __assert_fail(
                    b"found_paint_entry\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    347 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 114],
                        &[libc::c_char; 114],
                    >(
                        b"void _cairo_svg_stream_copy_to_output_stream(cairo_svg_stream_t *, cairo_output_stream_t *, cairo_hash_table_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            match (*element).c2rust_unnamed.paint_dependent.type_0 as libc::c_uint {
                0 => {
                    _cairo_output_stream_printf(
                        to,
                        b" x=\"%f\" y=\"%f\" width=\"%f\" height=\"%f\"\0" as *const u8
                            as *const libc::c_char,
                        (*found_paint_entry).box_0.p1.x,
                        (*found_paint_entry).box_0.p1.y,
                        (*found_paint_entry).box_0.p2.x
                            - (*found_paint_entry).box_0.p1.x,
                        (*found_paint_entry).box_0.p2.y - (*found_paint_entry).box_0.p1.y,
                    );
                }
                1 => {
                    _cairo_output_stream_printf(
                        to,
                        b" x=\"0\" y=\"0\" width=\"%f\" height=\"%f\"\0" as *const u8
                            as *const libc::c_char,
                        (*found_paint_entry).box_0.p2.x
                            - (*found_paint_entry).box_0.p1.x,
                        (*found_paint_entry).box_0.p2.y - (*found_paint_entry).box_0.p1.y,
                    );
                }
                2 => {
                    _cairo_output_stream_printf(
                        to,
                        b" transform=\"translate(%f, %f)\"\0" as *const u8
                            as *const libc::c_char,
                        (*found_paint_entry).box_0.p1.x,
                        (*found_paint_entry).box_0.p1.y,
                    );
                }
                3 => {
                    _cairo_output_stream_printf(
                        to,
                        b" transform=\"translate(%f, %f)\"\0" as *const u8
                            as *const libc::c_char,
                        -(*found_paint_entry).box_0.p1.x,
                        -(*found_paint_entry).box_0.p1.y,
                    );
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn _cairo_svg_stream_destroy(
    mut svg_stream: *mut cairo_svg_stream_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = (*svg_stream).status;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*svg_stream).elements.num_elements {
        let mut element: *mut cairo_svg_stream_element_t = _cairo_array_index(
            &mut (*svg_stream).elements,
            i,
        ) as *mut cairo_svg_stream_element_t;
        if (*element).type_0 as libc::c_uint
            == CAIRO_SVG_STREAM_ELEMENT_TYPE_TEXT as libc::c_int as libc::c_uint
        {
            let mut element_status: cairo_status_t = _cairo_output_stream_destroy(
                (*element).c2rust_unnamed.text.output_stream,
            );
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = element_status;
            }
        }
        i = i.wrapping_add(1);
    }
    _cairo_array_fini(&mut (*svg_stream).elements);
    return status;
}
static mut invalid_pattern_id: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
static mut _cairo_svg_versions: [cairo_svg_version_t; 2] = [
    CAIRO_SVG_VERSION_1_1,
    CAIRO_SVG_VERSION_1_2,
];
static mut _cairo_svg_supported_mime_types: [*const libc::c_char; 5] = [
    b"image/jpeg\0" as *const u8 as *const libc::c_char,
    b"image/png\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.uuid\0" as *const u8 as *const libc::c_char,
    b"text/x-uri\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut _cairo_svg_version_strings: [*const libc::c_char; 2] = [
    b"SVG 1.1\0" as *const u8 as *const libc::c_char,
    b"SVG 1.2\0" as *const u8 as *const libc::c_char,
];
static mut _cairo_svg_unit_strings: [*const libc::c_char; 10] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"em\0" as *const u8 as *const libc::c_char,
    b"ex\0" as *const u8 as *const libc::c_char,
    b"px\0" as *const u8 as *const libc::c_char,
    b"in\0" as *const u8 as *const libc::c_char,
    b"cm\0" as *const u8 as *const libc::c_char,
    b"mm\0" as *const u8 as *const libc::c_char,
    b"pt\0" as *const u8 as *const libc::c_char,
    b"pc\0" as *const u8 as *const libc::c_char,
    b"%\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_surface_create_for_stream(
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> *mut cairo_surface_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    stream = _cairo_output_stream_create(write_func, None, closure);
    if _cairo_output_stream_get_status(stream) as u64 != 0 {
        return _cairo_surface_create_in_error(_cairo_output_stream_destroy(stream));
    }
    return _cairo_svg_surface_create_for_stream_internal(
        stream,
        width,
        height,
        CAIRO_SVG_VERSION_1_1,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_surface_create(
    mut filename: *const libc::c_char,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> *mut cairo_surface_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    stream = _cairo_output_stream_create_for_filename(filename);
    if _cairo_output_stream_get_status(stream) as u64 != 0 {
        return _cairo_surface_create_in_error(_cairo_output_stream_destroy(stream));
    }
    return _cairo_svg_surface_create_for_stream_internal(
        stream,
        width,
        height,
        CAIRO_SVG_VERSION_1_1,
    );
}
unsafe extern "C" fn _cairo_surface_is_svg(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*surface).backend
        == &cairo_svg_surface_backend as *const cairo_surface_backend_t) as libc::c_int;
}
unsafe extern "C" fn _extract_svg_surface(
    mut surface: *mut cairo_surface_t,
    mut svg_surface: *mut *mut cairo_svg_surface_t,
) -> cairo_bool_t {
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    if _cairo_surface_is_paginated(surface) == 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        );
        return 0 as libc::c_int;
    }
    target = _cairo_paginated_surface_get_target(surface);
    if (*target).status as u64 != 0 {
        _cairo_surface_set_error(surface, (*target).status as cairo_int_status_t);
        return 0 as libc::c_int;
    }
    if (*target).finished() != 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return 0 as libc::c_int;
    }
    if _cairo_surface_is_svg(target) == 0 {
        _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        );
        return 0 as libc::c_int;
    }
    *svg_surface = target as *mut cairo_svg_surface_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_surface_restrict_to_version(
    mut abstract_surface: *mut cairo_surface_t,
    mut version: cairo_svg_version_t,
) {
    let mut surface: *mut cairo_svg_surface_t = 0 as *mut cairo_svg_surface_t;
    if _extract_svg_surface(abstract_surface, &mut surface) == 0 {
        return;
    }
    if (version as libc::c_uint)
        < (::std::mem::size_of::<[cairo_svg_version_t; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_svg_version_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        (*(*surface).document).svg_version = version;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_get_versions(
    mut versions: *mut *const cairo_svg_version_t,
    mut num_versions: *mut libc::c_int,
) {
    if !versions.is_null() {
        *versions = _cairo_svg_versions.as_ptr();
    }
    if !num_versions.is_null() {
        *num_versions = (::std::mem::size_of::<[cairo_svg_version_t; 2]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_svg_version_t>() as libc::c_ulong)
            as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_version_to_string(
    mut version: cairo_svg_version_t,
) -> *const libc::c_char {
    if version as libc::c_uint
        >= (::std::mem::size_of::<[cairo_svg_version_t; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_svg_version_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        return 0 as *const libc::c_char;
    }
    return _cairo_svg_version_strings[version as usize];
}
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_surface_set_document_unit(
    mut abstract_surface: *mut cairo_surface_t,
    mut unit: cairo_svg_unit_t,
) {
    let mut surface: *mut cairo_svg_surface_t = 0 as *mut cairo_svg_surface_t;
    if _extract_svg_surface(abstract_surface, &mut surface) == 0 {
        return;
    }
    if unit as libc::c_uint <= CAIRO_SVG_UNIT_PERCENT as libc::c_int as libc::c_uint {
        (*(*surface).document).unit = unit;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_svg_surface_get_document_unit(
    mut abstract_surface: *mut cairo_surface_t,
) -> cairo_svg_unit_t {
    let mut surface: *mut cairo_svg_surface_t = 0 as *mut cairo_svg_surface_t;
    if _extract_svg_surface(abstract_surface, &mut surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return CAIRO_SVG_UNIT_USER;
    }
    return (*(*surface).document).unit;
}
unsafe extern "C" fn _cairo_svg_paint_compute(
    mut document: *mut cairo_svg_document_t,
    mut paint: *mut cairo_svg_paint_t,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*paint).paint_elements.num_elements {
        let mut paint_element: *mut cairo_svg_paint_element_t = _cairo_array_index(
            &mut (*paint).paint_elements,
            i,
        ) as *mut cairo_svg_paint_element_t;
        let mut paint_key: cairo_svg_paint_t = cairo_svg_paint_t {
            base: cairo_hash_entry_t { hash: 0 },
            source_id: 0,
            paint_elements: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
            box_0: cairo_box_double_t {
                p1: cairo_point_double_t {
                    x: 0.,
                    y: 0.,
                },
                p2: cairo_point_double_t {
                    x: 0.,
                    y: 0.,
                },
            },
        };
        paint_key.source_id = (*paint_element).source_id;
        _cairo_svg_paint_init_key(&mut paint_key);
        let mut found_paint_entry: *mut cairo_svg_paint_t = _cairo_hash_table_lookup(
            (*document).paints,
            &mut paint_key.base,
        ) as *mut cairo_svg_paint_t;
        if !found_paint_entry.is_null() {} else {
            __assert_fail(
                b"found_paint_entry\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                872 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void _cairo_svg_paint_compute(cairo_svg_document_t *, cairo_svg_paint_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_svg_paint_compute(document, found_paint_entry);
        let mut box_0: cairo_box_double_t = (*found_paint_entry).box_0;
        _cairo_matrix_transform_bounding_box(
            &mut (*paint_element).matrix,
            &mut box_0.p1.x,
            &mut box_0.p1.y,
            &mut box_0.p2.x,
            &mut box_0.p2.y,
            0 as *mut cairo_bool_t,
        );
        _cairo_svg_paint_box_add_padding(&mut box_0);
        if i == 0 as libc::c_int as libc::c_uint {
            (*paint).box_0 = box_0;
        } else {
            (*paint)
                .box_0
                .p1
                .x = if (*paint).box_0.p1.x < box_0.p1.x {
                (*paint).box_0.p1.x
            } else {
                box_0.p1.x
            };
            (*paint)
                .box_0
                .p1
                .y = if (*paint).box_0.p1.y < box_0.p1.y {
                (*paint).box_0.p1.y
            } else {
                box_0.p1.y
            };
            (*paint)
                .box_0
                .p2
                .x = if (*paint).box_0.p2.x > box_0.p2.x {
                (*paint).box_0.p2.x
            } else {
                box_0.p2.x
            };
            (*paint)
                .box_0
                .p2
                .y = if (*paint).box_0.p2.y > box_0.p2.y {
                (*paint).box_0.p2.y
            } else {
                box_0.p2.y
            };
        }
        i = i.wrapping_add(1);
    }
    _cairo_array_truncate(
        &mut (*paint).paint_elements,
        0 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn _cairo_svg_paint_compute_func(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut paint: *mut cairo_svg_paint_t = entry as *mut cairo_svg_paint_t;
    let mut document: *mut cairo_svg_document_t = closure as *mut cairo_svg_document_t;
    _cairo_svg_paint_compute(document, paint);
}
unsafe extern "C" fn _cairo_svg_surface_add_source_surface(
    mut surface: *mut cairo_svg_surface_t,
    mut source_surface: *mut cairo_surface_t,
    mut is_new: *mut cairo_bool_t,
    mut result_source_surface: *mut *mut cairo_svg_source_surface_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut source_surface_key: cairo_svg_source_surface_t = cairo_svg_source_surface_t {
        base: cairo_hash_entry_t { hash: 0 },
        id: 0,
        unique_id: 0 as *mut libc::c_uchar,
        unique_id_length: 0,
        transitive_paint_used: 0,
    };
    source_surface_key.id = (*source_surface).unique_id;
    cairo_surface_get_mime_data(
        source_surface,
        b"application/x-cairo.uuid\0" as *const u8 as *const libc::c_char,
        &mut source_surface_key.unique_id as *mut *mut libc::c_uchar
            as *mut *const libc::c_uchar,
        &mut source_surface_key.unique_id_length,
    );
    _cairo_svg_source_surface_init_key(&mut source_surface_key);
    let mut found_source_surface_entry: *mut cairo_svg_source_surface_t = _cairo_hash_table_lookup(
        (*surface).source_surfaces,
        &mut source_surface_key.base,
    ) as *mut cairo_svg_source_surface_t;
    if !found_source_surface_entry.is_null() {
        *is_new = 0 as libc::c_int;
        *result_source_surface = found_source_surface_entry;
        return CAIRO_STATUS_SUCCESS;
    }
    let mut unique_id: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut unique_id_length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !(source_surface_key.unique_id).is_null()
        && source_surface_key.unique_id_length > 0 as libc::c_int as libc::c_ulong
    {
        unique_id = (if source_surface_key.unique_id_length
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(source_surface_key.unique_id_length)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_uchar;
        if unique_id.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        unique_id_length = source_surface_key.unique_id_length;
        memcpy(
            unique_id as *mut libc::c_void,
            source_surface_key.unique_id as *const libc::c_void,
            unique_id_length,
        );
    } else {
        unique_id = 0 as *mut libc::c_uchar;
        unique_id_length = 0 as libc::c_int as libc::c_ulong;
    }
    let mut source_surface_entry: *mut cairo_svg_source_surface_t = malloc(
        ::std::mem::size_of::<cairo_svg_source_surface_t>() as libc::c_ulong,
    ) as *mut cairo_svg_source_surface_t;
    if source_surface_entry.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        (*source_surface_entry).id = source_surface_key.id;
        (*source_surface_entry).unique_id_length = unique_id_length;
        let ref mut fresh2 = (*source_surface_entry).unique_id;
        *fresh2 = unique_id;
        _cairo_svg_source_surface_init_key(source_surface_entry);
        status = _cairo_hash_table_insert(
            (*surface).source_surfaces,
            &mut (*source_surface_entry).base,
        );
        if !(status as u64 != 0) {
            *is_new = 1 as libc::c_int;
            *result_source_surface = source_surface_entry;
            return CAIRO_STATUS_SUCCESS;
        }
    }
    free(unique_id as *mut libc::c_void);
    free(source_surface_entry as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn _cairo_svg_surface_cliprect_covers_surface(
    mut surface: *mut cairo_svg_surface_t,
    mut path: *mut cairo_path_fixed_t,
) -> cairo_bool_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    return ((*surface).surface_bounded != 0
        && _cairo_path_fixed_is_box(path, &mut box_0) != 0
        && box_0.p1.x <= 0 as libc::c_int && box_0.p1.y <= 0 as libc::c_int
        && _cairo_fixed_to_double(box_0.p2.x) >= (*surface).width
        && _cairo_fixed_to_double(box_0.p2.y) >= (*surface).height) as libc::c_int;
}
unsafe extern "C" fn _cairo_svg_surface_clipper_intersect_clip_path(
    mut clipper: *mut cairo_surface_clipper_t,
    mut path: *mut cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_status_t {
    let mut surface: *mut cairo_svg_surface_t = ({
        let mut mptr__: *const cairo_surface_clipper_t = clipper;
        (mptr__ as *mut libc::c_char).offset(-(448 as libc::c_ulong as isize))
            as *mut cairo_svg_surface_t
    });
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    if path.is_null() {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*surface).clip_level {
            _cairo_svg_stream_printf(
                (*surface).current_clipper_stream,
                b"</g>\n\0" as *const u8 as *const libc::c_char,
            );
            i = i.wrapping_add(1);
        }
        (*surface).clip_level = 0 as libc::c_int as libc::c_uint;
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_svg_surface_cliprect_covers_surface(surface, path) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<clipPath id=\"clip-%d\">\n\0" as *const u8 as *const libc::c_char,
        (*document).clip_id,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<path clip-rule=\"%s\"\0" as *const u8 as *const libc::c_char,
        if fill_rule as libc::c_uint
            == CAIRO_FILL_RULE_EVEN_ODD as libc::c_int as libc::c_uint
        {
            b"evenodd\0" as *const u8 as *const libc::c_char
        } else {
            b"nonzero\0" as *const u8 as *const libc::c_char
        },
    );
    _cairo_svg_surface_emit_path(
        &mut (*document).xml_node_defs,
        path,
        0 as *const cairo_matrix_t,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"/>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</clipPath>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        (*surface).current_clipper_stream,
        b"<g clip-path=\"url(#clip-%d)\">\n\0" as *const u8 as *const libc::c_char,
        (*document).clip_id,
    );
    let ref mut fresh3 = (*document).clip_id;
    *fresh3 = (*fresh3).wrapping_add(1);
    let ref mut fresh4 = (*surface).clip_level;
    *fresh4 = (*fresh4).wrapping_add(1);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_reset_clip(
    mut surface: *mut cairo_svg_surface_t,
) {
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    if !((*surface).current_clipper_stream).is_null() {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*surface).clip_level {
            _cairo_svg_stream_printf(
                (*surface).current_clipper_stream,
                b"</g>\n\0" as *const u8 as *const libc::c_char,
            );
            i = i.wrapping_add(1);
        }
    }
    (*surface).clip_level = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn _cairo_svg_surface_set_clip(
    mut surface: *mut cairo_svg_surface_t,
    mut clipper_stream: *mut cairo_svg_stream_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    if (*surface).current_clipper_stream != clipper_stream {
        _cairo_svg_surface_reset_clip(surface);
        let ref mut fresh5 = (*surface).current_clipper_stream;
        *fresh5 = clipper_stream;
    }
    return _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip);
}
unsafe extern "C" fn _cairo_svg_surface_create_for_document(
    mut document: *mut cairo_svg_document_t,
    mut content: cairo_content_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
    mut bounded: cairo_bool_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_svg_surface_t = 0 as *mut cairo_svg_surface_t;
    let mut paginated: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    surface = (if ::std::mem::size_of::<cairo_svg_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_svg_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_svg_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_svg_surface_backend,
        0 as *mut cairo_device_t,
        content,
        1 as libc::c_int,
    );
    (*surface).source_id = (*surface).base.unique_id;
    (*surface).depth = 0 as libc::c_int as libc::c_uint;
    (*surface).width = width;
    (*surface).height = height;
    (*surface).surface_bounded = bounded;
    let ref mut fresh6 = (*surface).document;
    *fresh6 = _cairo_svg_document_reference(document);
    (*surface).xml_node = _cairo_svg_stream_create();
    _cairo_array_init(
        &mut (*surface).page_set,
        ::std::mem::size_of::<cairo_svg_page_t>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh7 = (*surface).source_surfaces;
    *fresh7 = _cairo_hash_table_create(
        Some(
            _cairo_svg_source_surface_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if ((*surface).source_surfaces).is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _cairo_surface_clipper_init(
            &mut (*surface).clipper,
            Some(
                _cairo_svg_surface_clipper_intersect_clip_path
                    as unsafe extern "C" fn(
                        *mut cairo_surface_clipper_t,
                        *mut cairo_path_fixed_t,
                        cairo_fill_rule_t,
                        libc::c_double,
                        cairo_antialias_t,
                    ) -> cairo_status_t,
            ),
        );
        let ref mut fresh8 = (*surface).current_clipper_stream;
        *fresh8 = 0 as *mut cairo_svg_stream_t;
        (*surface).clip_level = 0 as libc::c_int as libc::c_uint;
        (*surface).transitive_paint_used = 0 as libc::c_int;
        (*surface).paginated_mode = CAIRO_PAGINATED_MODE_ANALYZE;
        (*surface).force_fallbacks = 0 as libc::c_int;
        paginated = _cairo_paginated_surface_create(
            &mut (*surface).base,
            (*surface).base.content,
            &cairo_svg_surface_paginated_backend,
        );
        status = (*paginated).status;
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            cairo_surface_destroy(&mut (*surface).base);
            return paginated;
        }
    }
    _cairo_svg_stream_destroy(&mut (*surface).xml_node);
    _cairo_svg_document_destroy(document);
    free(surface as *mut libc::c_void);
    return _cairo_surface_create_in_error(status);
}
unsafe extern "C" fn _cairo_svg_surface_create_for_stream_internal(
    mut stream: *mut cairo_output_stream_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
    mut version: cairo_svg_version_t,
) -> *mut cairo_surface_t {
    let mut document: *mut cairo_svg_document_t = 0 as *mut cairo_svg_document_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_svg_document_create(stream, width, height, version, &mut document);
    if status as u64 != 0 {
        surface = _cairo_surface_create_in_error(status);
        status = _cairo_output_stream_destroy(stream);
        return surface;
    }
    surface = _cairo_svg_surface_create_for_document(
        document,
        CAIRO_CONTENT_COLOR_ALPHA,
        width,
        height,
        1 as libc::c_int,
    );
    if (*surface).status as u64 != 0 {
        return surface;
    }
    let ref mut fresh9 = (*document).owner;
    *fresh9 = surface;
    status = _cairo_svg_document_destroy(document);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            1151 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"cairo_surface_t *_cairo_svg_surface_create_for_stream_internal(cairo_output_stream_t *, double, double, cairo_svg_version_t)\0",
            ))
                .as_ptr(),
        );
    }
    return surface;
}
unsafe extern "C" fn _cairo_svg_surface_store_page(
    mut surface: *mut cairo_svg_surface_t,
) -> *mut cairo_svg_page_t {
    _cairo_svg_surface_reset_clip(surface);
    let mut page: cairo_svg_page_t = cairo_svg_page_t {
        xml_node: cairo_svg_stream_t {
            status: CAIRO_STATUS_SUCCESS,
            elements: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
        },
    };
    page.xml_node = (*surface).xml_node;
    if _cairo_array_append(
        &mut (*surface).page_set,
        &mut page as *mut cairo_svg_page_t as *const libc::c_void,
    ) as u64 != 0
    {
        return 0 as *mut cairo_svg_page_t;
    }
    (*surface).xml_node = _cairo_svg_stream_create();
    return _cairo_array_index(
        &mut (*surface).page_set,
        ((*surface).page_set.num_elements).wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) as *mut cairo_svg_page_t;
}
unsafe extern "C" fn _cairo_svg_surface_copy_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    let mut page: *mut cairo_svg_page_t = _cairo_svg_surface_store_page(surface);
    if page.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_svg_stream_copy(&mut (*page).xml_node, &mut (*surface).xml_node);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_show_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    let mut page: *mut cairo_svg_page_t = _cairo_svg_surface_store_page(surface);
    if page.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_emit_transform(
    mut output: *mut cairo_svg_stream_t,
    mut attribute_name: *const libc::c_char,
    mut object_matrix: *const cairo_matrix_t,
    mut parent_matrix: *const cairo_matrix_t,
) {
    let mut matrix: cairo_matrix_t = *object_matrix;
    if !parent_matrix.is_null() {
        cairo_matrix_multiply(&mut matrix, &mut matrix, parent_matrix);
    }
    if _cairo_matrix_is_identity(&mut matrix) == 0 {
        _cairo_svg_stream_printf(
            output,
            b" %s=\"matrix(%f, %f, %f, %f, %f, %f)\"\0" as *const u8
                as *const libc::c_char,
            attribute_name,
            matrix.xx,
            matrix.yx,
            matrix.xy,
            matrix.yy,
            matrix.x0,
            matrix.y0,
        );
    }
}
unsafe extern "C" fn _cairo_svg_path_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut info: *mut svg_path_info_t = closure as *mut svg_path_info_t;
    let mut x: libc::c_double = _cairo_fixed_to_double((*point).x);
    let mut y: libc::c_double = _cairo_fixed_to_double((*point).y);
    if !((*info).ctm_inverse).is_null() {
        cairo_matrix_transform_point((*info).ctm_inverse, &mut x, &mut y);
    }
    _cairo_svg_stream_printf(
        (*info).output,
        b"M %f %f \0" as *const u8 as *const libc::c_char,
        x,
        y,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_path_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut info: *mut svg_path_info_t = closure as *mut svg_path_info_t;
    let mut x: libc::c_double = _cairo_fixed_to_double((*point).x);
    let mut y: libc::c_double = _cairo_fixed_to_double((*point).y);
    if !((*info).ctm_inverse).is_null() {
        cairo_matrix_transform_point((*info).ctm_inverse, &mut x, &mut y);
    }
    _cairo_svg_stream_printf(
        (*info).output,
        b"L %f %f \0" as *const u8 as *const libc::c_char,
        x,
        y,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_path_curve_to(
    mut closure: *mut libc::c_void,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_status_t {
    let mut info: *mut svg_path_info_t = closure as *mut svg_path_info_t;
    let mut bx: libc::c_double = _cairo_fixed_to_double((*b).x);
    let mut by: libc::c_double = _cairo_fixed_to_double((*b).y);
    let mut cx: libc::c_double = _cairo_fixed_to_double((*c).x);
    let mut cy: libc::c_double = _cairo_fixed_to_double((*c).y);
    let mut dx: libc::c_double = _cairo_fixed_to_double((*d).x);
    let mut dy: libc::c_double = _cairo_fixed_to_double((*d).y);
    if !((*info).ctm_inverse).is_null() {
        cairo_matrix_transform_point((*info).ctm_inverse, &mut bx, &mut by);
        cairo_matrix_transform_point((*info).ctm_inverse, &mut cx, &mut cy);
        cairo_matrix_transform_point((*info).ctm_inverse, &mut dx, &mut dy);
    }
    _cairo_svg_stream_printf(
        (*info).output,
        b"C %f %f %f %f %f %f \0" as *const u8 as *const libc::c_char,
        bx,
        by,
        cx,
        cy,
        dx,
        dy,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_path_close_path(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut info: *mut svg_path_info_t = closure as *mut svg_path_info_t;
    _cairo_svg_stream_printf(
        (*info).output,
        b"Z \0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_path(
    mut output: *mut cairo_svg_stream_t,
    mut path: *const cairo_path_fixed_t,
    mut ctm_inverse: *const cairo_matrix_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut info: svg_path_info_t = svg_path_info_t {
        output: 0 as *mut cairo_svg_stream_t,
        ctm_inverse: 0 as *const cairo_matrix_t,
    };
    _cairo_svg_stream_printf(output, b" d=\"\0" as *const u8 as *const libc::c_char);
    info.output = output;
    info.ctm_inverse = ctm_inverse;
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _cairo_svg_path_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_svg_path_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_svg_path_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_svg_path_close_path
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut info as *mut svg_path_info_t as *mut libc::c_void,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 108],
                &[libc::c_char; 108],
            >(
                b"void _cairo_svg_surface_emit_path(cairo_svg_stream_t *, const cairo_path_fixed_t *, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_svg_stream_printf(output, b"\"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn _cairo_svg_document_emit_outline_glyph_data(
    mut document: *mut cairo_svg_document_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyph_index: libc::c_ulong,
) -> cairo_int_status_t {
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_scaled_glyph_lookup(
        scaled_font,
        glyph_index,
        (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
            | CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int) as cairo_scaled_glyph_info_t,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    );
    if status as u64 != 0 {
        return status;
    }
    if _cairo_path_fixed_size((*scaled_glyph).path) != 0 as libc::c_int as libc::c_ulong
    {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
            b"<path\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_surface_emit_path(
            &mut (*document).xml_node_glyphs,
            (*scaled_glyph).path,
            0 as *const cairo_matrix_t,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
            b"/>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return status;
}
unsafe extern "C" fn _cairo_svg_document_emit_bitmap_glyph_data(
    mut document: *mut cairo_svg_document_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyph_index: libc::c_ulong,
) -> cairo_int_status_t {
    let mut source_id: libc::c_uint = 0;
    let mut temporary_stream: cairo_svg_stream_t = cairo_svg_stream_t {
        status: CAIRO_STATUS_SUCCESS,
        elements: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
    };
    let mut mask_id: libc::c_uint = 0;
    let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
    let mut paint_entry: *mut cairo_svg_paint_t = 0 as *mut cairo_svg_paint_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    status = _cairo_scaled_glyph_lookup(
        scaled_font,
        glyph_index,
        (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
            | CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int)
            as cairo_scaled_glyph_info_t,
        0 as *const cairo_color_t,
        &mut scaled_glyph,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    let mut use_recording_surface: cairo_bool_t = ((*scaled_glyph).has_info
        & CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut glyph_matrix: cairo_matrix_t = (*(*scaled_glyph).surface)
        .base
        .device_transform_inverse;
    let mut glyph_image_surface: *mut cairo_image_surface_t = (*scaled_glyph).surface;
    let mut extracted_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut extracted_image: *mut cairo_image_surface_t = 0
        as *mut cairo_image_surface_t;
    let mut extracted_image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    if use_recording_surface != 0 {
        let mut recording_surface: *mut cairo_recording_surface_t = (*scaled_glyph)
            .recording_surface as *mut cairo_recording_surface_t;
        if (*recording_surface).commands.num_elements == 1 as libc::c_int as libc::c_uint
        {
            let mut command: *mut cairo_command_t = *(_cairo_array_index(
                &mut (*recording_surface).commands,
                0 as libc::c_int as libc::c_uint,
            ) as *mut *mut cairo_command_t);
            if (*command).header.type_0 as libc::c_uint
                == CAIRO_COMMAND_MASK as libc::c_int as libc::c_uint
                && (*command).header.op as libc::c_uint
                    == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                && ((*command).header.clip).is_null()
                && (*command).mask.source.base.type_0 as libc::c_uint
                    == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
                && _cairo_color_equal(
                    &mut (*command).mask.source.solid.color,
                    _cairo_stock_color(CAIRO_STOCK_BLACK),
                ) != 0
                && (*command).mask.mask.base.extend as libc::c_uint
                    == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                && (*command).mask.mask.base.type_0 as libc::c_uint
                    == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
                && (*(*command).mask.mask.surface.surface).type_0 as libc::c_uint
                    == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
            {
                extracted_surface = (*command).mask.mask.surface.surface;
                if _cairo_surface_acquire_source_image(
                    extracted_surface,
                    &mut extracted_image,
                    &mut extracted_image_extra,
                ) as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    if (*extracted_image).format as libc::c_int
                        == CAIRO_FORMAT_A1 as libc::c_int
                        || (*extracted_image).format as libc::c_int
                            == CAIRO_FORMAT_A8 as libc::c_int
                    {
                        use_recording_surface = 0 as libc::c_int;
                        glyph_image_surface = extracted_image;
                        glyph_matrix = (*command).mask.mask.base.matrix;
                        status = cairo_matrix_invert(&mut glyph_matrix);
                        if status as libc::c_uint
                            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                                    as *const libc::c_char,
                                b"../src/cairo-svg-surface.c\0" as *const u8
                                    as *const libc::c_char,
                                1395 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 124],
                                    &[libc::c_char; 124],
                                >(
                                    b"cairo_int_status_t _cairo_svg_document_emit_bitmap_glyph_data(cairo_svg_document_t *, cairo_scaled_font_t *, unsigned long)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    }
                }
            }
        }
    }
    let mut paginated_surface: *mut cairo_surface_t = _cairo_svg_surface_create_for_document(
        document,
        CAIRO_CONTENT_COLOR_ALPHA,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int,
    );
    let mut svg_surface: *mut cairo_svg_surface_t = _cairo_paginated_surface_get_target(
        paginated_surface,
    ) as *mut cairo_svg_surface_t;
    status = (*paginated_surface).status;
    if !(status as u64 != 0) {
        source_id = (*svg_surface).base.unique_id;
        cairo_surface_set_fallback_resolution(
            paginated_surface,
            (*(*document).owner).x_fallback_resolution,
            (*(*document).owner).y_fallback_resolution,
        );
        temporary_stream = _cairo_svg_stream_create();
        let ref mut fresh10 = (*document).mask_id;
        let fresh11 = *fresh10;
        *fresh10 = (*fresh10).wrapping_add(1);
        mask_id = fresh11;
        _cairo_svg_stream_printf(
            &mut temporary_stream as *mut cairo_svg_stream_t,
            b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
            mask_id,
        );
        pattern = cairo_pattern_create_for_surface(
            if use_recording_surface != 0 {
                (*scaled_glyph).recording_surface
            } else {
                &mut (*glyph_image_surface).base
            },
        );
        _cairo_svg_surface_emit_composite_pattern(
            &mut temporary_stream,
            svg_surface,
            pattern as *mut cairo_surface_pattern_t,
            invalid_pattern_id,
            0 as *const cairo_matrix_t,
        );
        cairo_pattern_destroy(pattern);
        _cairo_svg_stream_printf(
            &mut temporary_stream as *mut cairo_svg_stream_t,
            b"</mask>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_copy(&mut temporary_stream, &mut (*document).xml_node_defs);
        status = _cairo_svg_stream_destroy(&mut temporary_stream);
        if !(status as u64 != 0) {
            (*svg_surface).transitive_paint_used = 1 as libc::c_int;
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
                b"<rect\0" as *const u8 as *const libc::c_char,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*document).xml_node_glyphs,
                source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE,
            );
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
                b" mask=\"url(#mask-%d)\"\0" as *const u8 as *const libc::c_char,
                mask_id,
            );
            if use_recording_surface == 0 {
                _cairo_svg_surface_emit_transform(
                    &mut (*document).xml_node_glyphs,
                    b"transform\0" as *const u8 as *const libc::c_char,
                    &mut glyph_matrix,
                    0 as *const cairo_matrix_t,
                );
            }
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
                b"/>\n\0" as *const u8 as *const libc::c_char,
            );
            paint_entry = malloc(
                ::std::mem::size_of::<cairo_svg_paint_t>() as libc::c_ulong,
            ) as *mut cairo_svg_paint_t;
            if paint_entry.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                (*paint_entry).source_id = source_id;
                (*paint_entry).box_0.p1.x = 0 as libc::c_int as libc::c_double;
                (*paint_entry).box_0.p1.y = 0 as libc::c_int as libc::c_double;
                (*paint_entry)
                    .box_0
                    .p2
                    .x = (*glyph_image_surface).width as libc::c_double;
                (*paint_entry)
                    .box_0
                    .p2
                    .y = (*glyph_image_surface).height as libc::c_double;
                if use_recording_surface != 0 {
                    _cairo_matrix_transform_bounding_box(
                        &mut glyph_matrix,
                        &mut (*paint_entry).box_0.p1.x,
                        &mut (*paint_entry).box_0.p1.y,
                        &mut (*paint_entry).box_0.p2.x,
                        &mut (*paint_entry).box_0.p2.y,
                        0 as *mut cairo_bool_t,
                    );
                }
                _cairo_svg_paint_box_add_padding(&mut (*paint_entry).box_0);
                _cairo_array_init(
                    &mut (*paint_entry).paint_elements,
                    ::std::mem::size_of::<cairo_svg_paint_element_t>() as libc::c_ulong
                        as libc::c_uint,
                );
                _cairo_svg_paint_init_key(paint_entry);
                status = _cairo_hash_table_insert(
                    (*document).paints,
                    &mut (*paint_entry).base,
                );
                status as u64 != 0;
            }
        }
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = cairo_surface_status(paginated_surface);
    }
    cairo_surface_destroy(paginated_surface);
    if !extracted_image.is_null() {
        _cairo_surface_release_source_image(
            extracted_surface,
            extracted_image,
            extracted_image_extra,
        );
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_document_emit_glyph(
    mut document: *mut cairo_svg_document_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_font_glyph_index: libc::c_ulong,
    mut font_id: libc::c_uint,
    mut subset_glyph_index: libc::c_uint,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
        b"<g id=\"glyph-%d-%d\">\n\0" as *const u8 as *const libc::c_char,
        font_id,
        subset_glyph_index,
    );
    status = _cairo_svg_document_emit_outline_glyph_data(
        document,
        scaled_font,
        scaled_font_glyph_index,
    );
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        status = _cairo_svg_document_emit_bitmap_glyph_data(
            document,
            scaled_font,
            scaled_font_glyph_index,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_glyphs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_document_emit_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut document: *mut cairo_svg_document_t = closure as *mut cairo_svg_document_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    _cairo_scaled_font_freeze_cache((*font_subset).scaled_font);
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        status = _cairo_svg_document_emit_glyph(
            document,
            (*font_subset).scaled_font,
            *((*font_subset).glyphs).offset(i as isize),
            (*font_subset).font_id,
            i,
        );
        if status as u64 != 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    _cairo_scaled_font_thaw_cache((*font_subset).scaled_font);
    return status;
}
unsafe extern "C" fn _cairo_svg_document_emit_font_subsets(
    mut document: *mut cairo_svg_document_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_scaled_font_subsets_foreach_scaled(
        (*document).font_subsets,
        Some(
            _cairo_svg_document_emit_font_subset
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_subset_t,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        document as *mut libc::c_void,
    );
    if !(status as u64 != 0) {
        status = _cairo_scaled_font_subsets_foreach_user(
            (*document).font_subsets,
            Some(
                _cairo_svg_document_emit_font_subset
                    as unsafe extern "C" fn(
                        *mut cairo_scaled_font_subset_t,
                        *mut libc::c_void,
                    ) -> cairo_int_status_t,
            ),
            document as *mut libc::c_void,
        );
    }
    _cairo_scaled_font_subsets_destroy((*document).font_subsets);
    let ref mut fresh12 = (*document).font_subsets;
    *fresh12 = 0 as *mut cairo_scaled_font_subsets_t;
    return status;
}
unsafe extern "C" fn _cairo_svg_surface_are_operation_and_pattern_supported(
    mut surface: *mut cairo_svg_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    if (*surface).force_fallbacks != 0 {
        return 0 as libc::c_int;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SATURATE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVERLAY as libc::c_int as libc::c_uint
        || op as libc::c_uint
            == CAIRO_OPERATOR_COLOR_DODGE as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_COLOR_BURN as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_HARD_LIGHT as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_SOFT_LIGHT as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_DIFFERENCE as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_EXCLUSION as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_HSL_HUE as libc::c_int as libc::c_uint
        || op as libc::c_uint
            == CAIRO_OPERATOR_HSL_SATURATION as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_HSL_COLOR as libc::c_int as libc::c_uint
        || op as libc::c_uint
            == CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        if (*(*(pattern as *mut cairo_surface_pattern_t)).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
            && (*surface).depth > 1000 as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        if (*pattern).extend as libc::c_uint
            != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
            && (*pattern).extend as libc::c_uint
                != CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
    {
        let mut radial_pattern: *mut cairo_radial_pattern_t = pattern
            as *mut cairo_radial_pattern_t;
        let mut max_radius: libc::c_double = 0.;
        if (*radial_pattern).cd1.radius > (*radial_pattern).cd2.radius {
            max_radius = (*radial_pattern).cd1.radius;
        } else {
            max_radius = (*radial_pattern).cd2.radius;
        }
        let mut c1: cairo_point_double_t = (*radial_pattern).cd1.center;
        let mut c2: cairo_point_double_t = (*radial_pattern).cd2.center;
        if (c1.x - c2.x) * (c1.x - c2.x) + (c1.y - c2.y) * (c1.y - c2.y)
            >= max_radius * max_radius
        {
            return 0 as libc::c_int;
        }
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_svg_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut final_status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    if _cairo_paginated_surface_get_target((*(*surface).document).owner)
        == &mut (*surface).base as *mut cairo_surface_t
    {
        final_status = _cairo_svg_document_finish((*surface).document);
    } else {
        final_status = CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_svg_stream_destroy(&mut (*surface).xml_node);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*surface).page_set.num_elements {
        let mut page: *mut cairo_svg_page_t = _cairo_array_index(
            &mut (*surface).page_set,
            i,
        ) as *mut cairo_svg_page_t;
        status = _cairo_svg_stream_destroy(&mut (*page).xml_node);
        if final_status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            final_status = status;
        }
        i = i.wrapping_add(1);
    }
    _cairo_array_fini(&mut (*surface).page_set);
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    _cairo_hash_table_foreach(
        (*surface).source_surfaces,
        Some(
            _cairo_svg_source_surface_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*surface).source_surfaces as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*surface).source_surfaces);
    status = _cairo_svg_document_destroy((*surface).document);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    return final_status;
}
unsafe extern "C" fn _cairo_svg_surface_emit_static_filter(
    mut document: *mut cairo_svg_document_t,
    mut filter: cairo_svg_filter,
) -> *const libc::c_char {
    if (*document).filters_emitted[filter as usize] == 0 {
        (*document).filters_emitted[filter as usize] = 1 as libc::c_int;
        if filter as libc::c_uint
            == CAIRO_SVG_FILTER_REMOVE_COLOR as libc::c_int as libc::c_uint
        {
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-remove-color\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feColorMatrix color-interpolation-filters=\"sRGB\" values=\"0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 1 0\" />\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if filter as libc::c_uint
            == CAIRO_SVG_FILTER_REMOVE_COLOR_AND_INVERT_ALPHA as libc::c_int
                as libc::c_uint
        {
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-remove-color-and-invert-alpha\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feColorMatrix color-interpolation-filters=\"sRGB\" values=\"0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 -1 1\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if filter as libc::c_uint
            == CAIRO_SVG_FILTER_COLOR_TO_ALPHA as libc::c_int as libc::c_uint
        {
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-color-to-alpha\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feColorMatrix color-interpolation-filters=\"sRGB\" values=\"0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0.2126 0.7152 0.0722 0 0\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if filter as libc::c_uint
        == CAIRO_SVG_FILTER_REMOVE_COLOR as libc::c_int as libc::c_uint
    {
        return b"remove-color\0" as *const u8 as *const libc::c_char
    } else {
        if filter as libc::c_uint
            == CAIRO_SVG_FILTER_REMOVE_COLOR_AND_INVERT_ALPHA as libc::c_int
                as libc::c_uint
        {
            return b"remove-color-and-invert-alpha\0" as *const u8 as *const libc::c_char
        } else {
            if filter as libc::c_uint
                == CAIRO_SVG_FILTER_COLOR_TO_ALPHA as libc::c_int as libc::c_uint
            {
                return b"color-to-alpha\0" as *const u8 as *const libc::c_char
            } else {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-svg-surface.c\0" as *const u8
                            as *const libc::c_char,
                        1727 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 97],
                            &[libc::c_char; 97],
                        >(
                            b"const char *_cairo_svg_surface_emit_static_filter(cairo_svg_document_t *, enum cairo_svg_filter)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn _cairo_svg_surface_emit_parametric_filter(
    mut surface: *mut cairo_svg_surface_t,
    mut filter: cairo_svg_filter,
    mut source_compositing_group_id: libc::c_uint,
    mut destination_compositing_group_id: libc::c_uint,
) -> libc::c_uint {
    let ref mut fresh13 = (*(*surface).document).filter_id;
    let fresh14 = *fresh13;
    *fresh13 = (*fresh13).wrapping_add(1);
    let mut filter_id: libc::c_uint = fresh14;
    match filter as libc::c_uint {
        4 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feComposite in=\"source\" in2=\"destination\" operator=\"over\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        5 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feComposite in=\"source\" in2=\"destination\" operator=\"in\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        6 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feComposite in=\"source\" in2=\"destination\" operator=\"out\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        7 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feComposite in=\"source\" in2=\"destination\" operator=\"atop\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        8 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feComposite in=\"source\" in2=\"destination\" operator=\"xor\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        9 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feComposite in=\"source\" in2=\"destination\" operator=\"arithmetic\" k1=\"0\" k2=\"1\" k3=\"1\" k4=\"0\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        10 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"multiply\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        11 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"screen\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        12 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"overlay\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        13 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"darken\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        14 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"lighten\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        15 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"color-dodge\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        16 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"color-burn\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        17 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"hard-light\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        18 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"soft-light\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        19 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"difference\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        20 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"exclusion\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        21 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"hue\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        22 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"saturation\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        23 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"color\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        24 => {
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"<filter id=\"filter-%d\" x=\"0%%\" y=\"0%%\" width=\"100%%\" height=\"100%%\">\n<feImage xlink:href=\"#compositing-group-%d\" result=\"source\"\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feImage xlink:href=\"#compositing-group-%d\" result=\"destination\"\0"
                    as *const u8 as *const libc::c_char,
                destination_compositing_group_id,
            );
            _cairo_svg_stream_append_paint_dependent(
                &mut (*(*surface).document).xml_node_filters,
                (*surface).source_id,
                CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
            );
            _cairo_svg_stream_printf(
                &mut (*(*surface).document).xml_node_filters as *mut cairo_svg_stream_t,
                b"/>\n<feBlend in=\"source\" in2=\"destination\" mode=\"luminosity\" color-interpolation-filters=\"sRGB\"/>\n</filter>\n\0"
                    as *const u8 as *const libc::c_char,
                filter_id,
                source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        0 | 1 | 2 | 3 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    1884 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 129],
                        &[libc::c_char; 129],
                    >(
                        b"unsigned int _cairo_svg_surface_emit_parametric_filter(cairo_svg_surface_t *, enum cairo_svg_filter, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    return filter_id;
}
static mut base64_table: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &[libc::c_char; 64],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
};
unsafe extern "C" fn base64_write_func(
    mut closure: *mut libc::c_void,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut info: *mut base64_write_closure_t = closure as *mut base64_write_closure_t;
    let mut i: libc::c_uint = 0;
    let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    src = ((*info).src).as_mut_ptr();
    if ((*info).in_mem).wrapping_add(length) < 3 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as libc::c_uint;
        while i < length {
            let fresh15 = data;
            data = data.offset(1);
            *src.offset(i.wrapping_add((*info).in_mem) as isize) = *fresh15;
            i = i.wrapping_add(1);
        }
        let ref mut fresh16 = (*info).in_mem;
        *fresh16 = (*fresh16).wrapping_add(length);
        return CAIRO_STATUS_SUCCESS;
    }
    loop {
        let mut dst: [libc::c_uchar; 4] = [0; 4];
        i = (*info).in_mem;
        while i < 3 as libc::c_int as libc::c_uint {
            let fresh17 = data;
            data = data.offset(1);
            *src.offset(i as isize) = *fresh17;
            length = length.wrapping_sub(1);
            i = i.wrapping_add(1);
        }
        (*info).in_mem = 0 as libc::c_int as libc::c_uint;
        dst[0 as libc::c_int
            as usize] = base64_table[(*src.offset(0 as libc::c_int as isize)
            as libc::c_int >> 2 as libc::c_int) as usize] as libc::c_uchar;
        dst[1 as libc::c_int
            as usize] = base64_table[((*src.offset(0 as libc::c_int as isize)
            as libc::c_int & 0x3 as libc::c_int) << 4 as libc::c_int
            | *src.offset(1 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int)
            as usize] as libc::c_uchar;
        dst[2 as libc::c_int
            as usize] = base64_table[((*src.offset(1 as libc::c_int as isize)
            as libc::c_int & 0xf as libc::c_int) << 2 as libc::c_int
            | *src.offset(2 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int)
            as usize] as libc::c_uchar;
        dst[3 as libc::c_int
            as usize] = base64_table[(*src.offset(2 as libc::c_int as isize)
            as libc::c_int & 0xfc as libc::c_int >> 2 as libc::c_int) as usize]
            as libc::c_uchar;
        let mut current_block_20: u64;
        match (*info).trailing {
            2 => {
                dst[2 as libc::c_int as usize] = '=' as i32 as libc::c_uchar;
                current_block_20 = 3533913109686437874;
            }
            1 => {
                current_block_20 = 3533913109686437874;
            }
            _ => {
                current_block_20 = 10043043949733653460;
            }
        }
        match current_block_20 {
            3533913109686437874 => {
                dst[3 as libc::c_int as usize] = '=' as i32 as libc::c_uchar;
            }
            _ => {}
        }
        _cairo_svg_stream_write(
            (*info).output,
            dst.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        if !(length >= 3 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let fresh18 = data;
        data = data.offset(1);
        *src.offset(i as isize) = *fresh18;
        i = i.wrapping_add(1);
    }
    (*info).in_mem = length;
    return (*(*info).output).status;
}
unsafe extern "C" fn _cairo_surface_base64_encode_jpeg(
    mut surface: *mut cairo_surface_t,
    mut output: *mut cairo_svg_stream_t,
) -> cairo_int_status_t {
    let mut mime_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let mut image_info: cairo_image_info_t = cairo_image_info_t {
        width: 0,
        height: 0,
        num_components: 0,
        bits_per_component: 0,
    };
    let mut info: base64_write_closure_t = base64_write_closure_t {
        output: 0 as *mut cairo_svg_stream_t,
        in_mem: 0,
        trailing: 0,
        src: [0; 3],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    cairo_surface_get_mime_data(
        surface,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_image_info_get_jpeg_info(
        &mut image_info,
        mime_data,
        mime_data_length,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if image_info.num_components == 4 as libc::c_int {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    _cairo_svg_stream_printf(
        output,
        b"data:image/jpeg;base64,\0" as *const u8 as *const libc::c_char,
    );
    info.output = output;
    info.in_mem = 0 as libc::c_int as libc::c_uint;
    info.trailing = 0 as libc::c_int as libc::c_uint;
    status = base64_write_func(
        &mut info as *mut base64_write_closure_t as *mut libc::c_void,
        mime_data,
        mime_data_length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if info.in_mem > 0 as libc::c_int as libc::c_uint {
        memset(
            (info.src).as_mut_ptr().offset(info.in_mem as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (3 as libc::c_int as libc::c_uint).wrapping_sub(info.in_mem) as libc::c_ulong,
        );
        info.trailing = (3 as libc::c_int as libc::c_uint).wrapping_sub(info.in_mem);
        info.in_mem = 3 as libc::c_int as libc::c_uint;
        status = base64_write_func(
            &mut info as *mut base64_write_closure_t as *mut libc::c_void,
            0 as *const libc::c_uchar,
            0 as libc::c_int as libc::c_uint,
        );
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_base64_encode_png(
    mut surface: *mut cairo_surface_t,
    mut output: *mut cairo_svg_stream_t,
) -> cairo_int_status_t {
    let mut mime_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let mut info: base64_write_closure_t = base64_write_closure_t {
        output: 0 as *mut cairo_svg_stream_t,
        in_mem: 0,
        trailing: 0,
        src: [0; 3],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    cairo_surface_get_mime_data(
        surface,
        b"image/png\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if (*surface).status as u64 != 0 {
        return (*surface).status as cairo_int_status_t;
    }
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    _cairo_svg_stream_printf(
        output,
        b"data:image/png;base64,\0" as *const u8 as *const libc::c_char,
    );
    info.output = output;
    info.in_mem = 0 as libc::c_int as libc::c_uint;
    info.trailing = 0 as libc::c_int as libc::c_uint;
    status = base64_write_func(
        &mut info as *mut base64_write_closure_t as *mut libc::c_void,
        mime_data,
        mime_data_length as libc::c_uint,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if info.in_mem > 0 as libc::c_int as libc::c_uint {
        memset(
            (info.src).as_mut_ptr().offset(info.in_mem as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (3 as libc::c_int as libc::c_uint).wrapping_sub(info.in_mem) as libc::c_ulong,
        );
        info.trailing = (3 as libc::c_int as libc::c_uint).wrapping_sub(info.in_mem);
        info.in_mem = 3 as libc::c_int as libc::c_uint;
        status = base64_write_func(
            &mut info as *mut base64_write_closure_t as *mut libc::c_void,
            0 as *const libc::c_uchar,
            0 as libc::c_int as libc::c_uint,
        );
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_surface_base64_encode(
    mut surface: *mut cairo_surface_t,
    mut output: *mut cairo_svg_stream_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut info: base64_write_closure_t = base64_write_closure_t {
        output: 0 as *mut cairo_svg_stream_t,
        in_mem: 0,
        trailing: 0,
        src: [0; 3],
    };
    status = _cairo_surface_base64_encode_jpeg(surface, output);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_surface_base64_encode_png(surface, output);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    info.output = output;
    info.in_mem = 0 as libc::c_int as libc::c_uint;
    info.trailing = 0 as libc::c_int as libc::c_uint;
    _cairo_svg_stream_printf(
        info.output,
        b"data:image/png;base64,\0" as *const u8 as *const libc::c_char,
    );
    status = cairo_surface_write_to_png_stream(
        surface,
        Some(
            base64_write_func
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        &mut info as *mut base64_write_closure_t as *mut libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if info.in_mem > 0 as libc::c_int as libc::c_uint {
        memset(
            (info.src).as_mut_ptr().offset(info.in_mem as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (3 as libc::c_int as libc::c_uint).wrapping_sub(info.in_mem) as libc::c_ulong,
        );
        info.trailing = (3 as libc::c_int as libc::c_uint).wrapping_sub(info.in_mem);
        info.in_mem = 3 as libc::c_int as libc::c_uint;
        status = base64_write_func(
            &mut info as *mut base64_write_closure_t as *mut libc::c_void,
            0 as *const libc::c_uchar,
            0 as libc::c_int as libc::c_uint,
        ) as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn _cairo_svg_surface_emit_attr_value(
    mut stream: *mut cairo_svg_stream_t,
    mut value: *const libc::c_uchar,
    mut length: libc::c_uint,
) {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut q: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: libc::c_uint = 0;
    p = value;
    q = p;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        if *p as libc::c_int == '&' as i32 || *p as libc::c_int == '"' as i32 {
            if p != q {
                _cairo_svg_stream_write(
                    stream,
                    q as *const libc::c_void,
                    p.offset_from(q) as libc::c_long as size_t,
                );
                q = p.offset(1 as libc::c_int as isize);
            }
            if *p as libc::c_int == '&' as i32 {
                _cairo_svg_stream_printf(
                    stream,
                    b"&amp;\0" as *const u8 as *const libc::c_char,
                );
            } else {
                _cairo_svg_stream_printf(
                    stream,
                    b"&quot;\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        i = i.wrapping_add(1);
        p = p.offset(1);
    }
    if p != q {
        _cairo_svg_stream_write(
            stream,
            q as *const libc::c_void,
            p.offset_from(q) as libc::c_long as size_t,
        );
    }
}
unsafe extern "C" fn _cairo_svg_surface_emit_surface(
    mut document: *mut cairo_svg_document_t,
    mut surface: *mut cairo_surface_t,
    mut source_id: libc::c_uint,
) -> cairo_status_t {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut is_bounded: cairo_bool_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut uri: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut uri_len: libc::c_ulong = 0;
    is_bounded = _cairo_surface_get_extents(surface, &mut extents);
    if is_bounded != 0 {} else {
        __assert_fail(
            b"is_bounded\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            2118 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"cairo_status_t _cairo_svg_surface_emit_surface(cairo_svg_document_t *, cairo_surface_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<image id=\"source-%d\" x=\"%d\" y=\"%d\" width=\"%d\" height=\"%d\"\0"
            as *const u8 as *const libc::c_char,
        source_id,
        extents.x,
        extents.y,
        extents.width,
        extents.height,
    );
    if extents.width != 0 as libc::c_int && extents.height != 0 as libc::c_int {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b" xlink:href=\"\0" as *const u8 as *const libc::c_char,
        );
        cairo_surface_get_mime_data(
            surface,
            b"text/x-uri\0" as *const u8 as *const libc::c_char,
            &mut uri,
            &mut uri_len,
        );
        if !uri.is_null() {
            _cairo_svg_surface_emit_attr_value(
                &mut (*document).xml_node_defs,
                uri,
                uri_len as libc::c_uint,
            );
        } else {
            status = _cairo_surface_base64_encode(
                surface,
                &mut (*document).xml_node_defs,
            ) as cairo_status_t;
            if status as u64 != 0 {
                return status;
            }
        }
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"\"\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"/>\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_composite_surface_pattern(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_surface_pattern_t,
    mut pattern_id: libc::c_uint,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut p2u: cairo_matrix_t = (*pattern).base.matrix;
    status = cairo_matrix_invert(&mut p2u);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            2159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 175],
                &[libc::c_char; 175],
            >(
                b"cairo_status_t _cairo_svg_surface_emit_composite_surface_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut is_new: cairo_bool_t = 0;
    let mut source_surface: *mut cairo_svg_source_surface_t = 0
        as *mut cairo_svg_source_surface_t;
    status = _cairo_svg_surface_add_source_surface(
        surface,
        (*pattern).surface,
        &mut is_new,
        &mut source_surface,
    );
    if status as u64 != 0 {
        return status;
    }
    let mut source_id: libc::c_uint = (*source_surface).id;
    if is_new != 0 {
        status = _cairo_svg_surface_emit_surface(
            (*surface).document,
            (*pattern).surface,
            source_id,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    if pattern_id != invalid_pattern_id {
        let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut is_bounded: cairo_bool_t = 0;
        is_bounded = _cairo_surface_get_extents((*pattern).surface, &mut extents);
        if is_bounded != 0 {} else {
            __assert_fail(
                b"is_bounded\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                2186 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 175],
                    &[libc::c_char; 175],
                >(
                    b"cairo_status_t _cairo_svg_surface_emit_composite_surface_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_svg_stream_printf(
            output,
            b"<pattern id=\"pattern-%d\" patternUnits=\"userSpaceOnUse\" x=\"%d\" y=\"%d\" width=\"%d\" height=\"%d\" viewBox=\"%d %d %d %d\"\0"
                as *const u8 as *const libc::c_char,
            pattern_id,
            extents.x,
            extents.y,
            extents.width,
            extents.height,
            extents.x,
            extents.y,
            extents.width,
            extents.height,
        );
        _cairo_svg_surface_emit_transform(
            output,
            b"patternTransform\0" as *const u8 as *const libc::c_char,
            &mut p2u,
            parent_matrix,
        );
        _cairo_svg_stream_printf(output, b">\n\0" as *const u8 as *const libc::c_char);
    }
    _cairo_svg_stream_printf(
        output,
        b"<use xlink:href=\"#source-%d\"\0" as *const u8 as *const libc::c_char,
        source_id,
    );
    if (*(*pattern).surface).content as libc::c_uint
        == CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
    {
        let mut can_skip_filter: cairo_bool_t = 0 as libc::c_int;
        if !((*(*pattern).surface).backend).is_null()
            && (*(*(*pattern).surface).backend).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
            && ((*((*pattern).surface as *mut cairo_image_surface_t)).format
                as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int
                || (*((*pattern).surface as *mut cairo_image_surface_t)).format
                    as libc::c_int == CAIRO_FORMAT_A8 as libc::c_int)
        {
            can_skip_filter = 1 as libc::c_int;
        }
        if can_skip_filter == 0 {
            _cairo_svg_stream_printf(
                output,
                b" filter=\"url(#filter-%s)\"\0" as *const u8 as *const libc::c_char,
                _cairo_svg_surface_emit_static_filter(
                    (*surface).document,
                    CAIRO_SVG_FILTER_COLOR_TO_ALPHA,
                ),
            );
        }
    }
    if pattern_id == invalid_pattern_id {
        _cairo_svg_surface_emit_transform(
            output,
            b"transform\0" as *const u8 as *const libc::c_char,
            &mut p2u,
            parent_matrix,
        );
    }
    _cairo_svg_stream_printf(output, b"/>\n\0" as *const u8 as *const libc::c_char);
    if pattern_id != invalid_pattern_id {
        _cairo_svg_stream_printf(
            output,
            b"</pattern>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_recording_surface(
    mut surface: *mut cairo_svg_surface_t,
    mut source: *mut cairo_recording_surface_t,
    mut source_id: libc::c_uint,
    mut transitive_paint_used: *mut cairo_bool_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    let mut paginated_surface: *mut cairo_surface_t = _cairo_svg_surface_create_for_document(
        document,
        (*source).base.content,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int,
    );
    let mut svg_surface: *mut cairo_svg_surface_t = _cairo_paginated_surface_get_target(
        paginated_surface,
    ) as *mut cairo_svg_surface_t;
    if (*paginated_surface).status as u64 != 0 {
        return (*paginated_surface).status;
    }
    (*svg_surface).source_id = source_id;
    (*svg_surface)
        .depth = ((*surface).depth).wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut bounded: cairo_bool_t = _cairo_surface_get_extents(
        &mut (*source).base,
        &mut extents,
    );
    cairo_surface_set_fallback_resolution(
        paginated_surface,
        (*(*document).owner).x_fallback_resolution,
        (*(*document).owner).y_fallback_resolution,
    );
    if (*source).base.content as libc::c_uint
        == CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
    {
        _cairo_svg_surface_emit_paint(
            &mut (*svg_surface).xml_node,
            svg_surface,
            &_cairo_pattern_black.base,
            0 as libc::c_int,
        );
    }
    status = _cairo_recording_surface_replay(&mut (*source).base, paginated_surface);
    if status as u64 != 0 {
        cairo_surface_destroy(paginated_surface);
        return status;
    }
    cairo_surface_show_page(paginated_surface);
    status = cairo_surface_status(paginated_surface);
    if status as u64 != 0 {
        cairo_surface_destroy(paginated_surface);
        return status;
    }
    let mut clip_id: libc::c_uint = 0;
    if bounded != 0 {
        let ref mut fresh19 = (*document).clip_id;
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_add(1);
        clip_id = fresh20;
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<clipPath id=\"clip-%d\">\n<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"%d\"/>\n</clipPath>\n\0"
                as *const u8 as *const libc::c_char,
            clip_id,
            extents.x,
            extents.y,
            extents.width,
            extents.height,
        );
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g id=\"source-%d\"\0" as *const u8 as *const libc::c_char,
        source_id,
    );
    if bounded != 0 {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b" clip-path=\"url(#clip-%d)\"\0" as *const u8 as *const libc::c_char,
            clip_id,
        );
    }
    if (*source).base.content as libc::c_uint
        == CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
    {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b" filter=\"url(#filter-%s)\"\0" as *const u8 as *const libc::c_char,
            _cairo_svg_surface_emit_static_filter(
                document,
                CAIRO_SVG_FILTER_REMOVE_COLOR,
            ),
        );
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    if (*svg_surface).xml_node.elements.num_elements > 0 as libc::c_int as libc::c_uint {
        let mut page: *mut cairo_svg_page_t = _cairo_svg_surface_store_page(svg_surface);
        if page.is_null() {
            cairo_surface_destroy(paginated_surface);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    if (*svg_surface).page_set.num_elements > 0 as libc::c_int as libc::c_uint {
        let mut page_0: *mut cairo_svg_page_t = _cairo_array_index(
            &mut (*svg_surface).page_set,
            ((*svg_surface).page_set.num_elements)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_svg_page_t;
        _cairo_svg_stream_copy(&mut (*page_0).xml_node, &mut (*document).xml_node_defs);
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    *transitive_paint_used = (*svg_surface).transitive_paint_used;
    status = cairo_surface_status(paginated_surface);
    cairo_surface_destroy(paginated_surface);
    return status;
}
unsafe extern "C" fn _cairo_svg_surface_to_recording_surface(
    mut pattern: *const cairo_surface_pattern_t,
) -> *mut cairo_recording_surface_t {
    let mut surface: *mut cairo_surface_t = (*pattern).surface;
    if _cairo_surface_is_paginated(surface) != 0 {
        surface = _cairo_paginated_surface_get_recording(surface);
    }
    if _cairo_surface_is_snapshot(surface) != 0 {
        surface = _cairo_surface_snapshot_get_target(surface);
    }
    return surface as *mut cairo_recording_surface_t;
}
unsafe extern "C" fn _cairo_svg_surface_svg_pattern_should_be_used(
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    return ((*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*pattern).extend as libc::c_uint
            == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
        && _cairo_surface_get_extents(
            (*(pattern as *mut cairo_surface_pattern_t)).surface,
            &mut extents,
        ) != 0) as libc::c_int;
}
unsafe extern "C" fn _cairo_svg_surface_svg_clip_or_svg_mask_should_be_used(
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    return ((*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && _cairo_svg_surface_svg_pattern_should_be_used(pattern) == 0) as libc::c_int;
}
unsafe extern "C" fn _cairo_svg_surface_emit_composite_recording_pattern(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_surface_pattern_t,
    mut pattern_id: libc::c_uint,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    let mut p2u: cairo_matrix_t = (*pattern).base.matrix;
    status = cairo_matrix_invert(&mut p2u);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            2379 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"cairo_status_t _cairo_svg_surface_emit_composite_recording_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut is_new: cairo_bool_t = 0;
    let mut source_surface: *mut cairo_svg_source_surface_t = 0
        as *mut cairo_svg_source_surface_t;
    status = _cairo_svg_surface_add_source_surface(
        surface,
        (*pattern).surface,
        &mut is_new,
        &mut source_surface,
    );
    if status as u64 != 0 {
        return status;
    }
    let mut source_id: libc::c_uint = (*source_surface).id;
    let mut recording_surface: *mut cairo_recording_surface_t = _cairo_svg_surface_to_recording_surface(
        pattern,
    );
    if is_new != 0 {
        status = _cairo_svg_surface_emit_recording_surface(
            surface,
            recording_surface,
            source_id,
            &mut (*source_surface).transitive_paint_used,
        );
        if status as u64 != 0 {
            return status;
        }
        if (*source_surface).transitive_paint_used != 0 {
            let mut paint_entry: *mut cairo_svg_paint_t = malloc(
                ::std::mem::size_of::<cairo_svg_paint_t>() as libc::c_ulong,
            ) as *mut cairo_svg_paint_t;
            if paint_entry.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            (*paint_entry).source_id = source_id;
            _cairo_array_init(
                &mut (*paint_entry).paint_elements,
                ::std::mem::size_of::<cairo_svg_paint_element_t>() as libc::c_ulong
                    as libc::c_uint,
            );
            _cairo_svg_paint_init_key(paint_entry);
            status = _cairo_hash_table_insert(
                (*document).paints,
                &mut (*paint_entry).base,
            );
            if status as u64 != 0 {
                return status;
            }
        }
    }
    if (*source_surface).transitive_paint_used != 0 {
        let mut paint_key: cairo_svg_paint_t = cairo_svg_paint_t {
            base: cairo_hash_entry_t { hash: 0 },
            source_id: 0,
            paint_elements: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
            box_0: cairo_box_double_t {
                p1: cairo_point_double_t {
                    x: 0.,
                    y: 0.,
                },
                p2: cairo_point_double_t {
                    x: 0.,
                    y: 0.,
                },
            },
        };
        paint_key.source_id = source_id;
        _cairo_svg_paint_init_key(&mut paint_key);
        let mut found_paint_entry: *mut cairo_svg_paint_t = _cairo_hash_table_lookup(
            (*document).paints,
            &mut paint_key.base,
        ) as *mut cairo_svg_paint_t;
        if !found_paint_entry.is_null() {} else {
            __assert_fail(
                b"found_paint_entry\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                2424 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 177],
                    &[libc::c_char; 177],
                >(
                    b"cairo_status_t _cairo_svg_surface_emit_composite_recording_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        let mut paint_element: cairo_svg_paint_element_t = cairo_svg_paint_element_t {
            source_id: 0,
            matrix: cairo_matrix_t {
                xx: 0.,
                yx: 0.,
                xy: 0.,
                yy: 0.,
                x0: 0.,
                y0: 0.,
            },
        };
        paint_element.source_id = (*surface).source_id;
        paint_element.matrix = (*pattern).base.matrix;
        if !parent_matrix.is_null() {
            let mut parent_matrix_inverse: cairo_matrix_t = *parent_matrix;
            status = cairo_matrix_invert(&mut parent_matrix_inverse);
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    2433 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 177],
                        &[libc::c_char; 177],
                    >(
                        b"cairo_status_t _cairo_svg_surface_emit_composite_recording_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            cairo_matrix_multiply(
                &mut paint_element.matrix,
                &mut parent_matrix_inverse,
                &mut paint_element.matrix,
            );
        }
        status = _cairo_array_append(
            &mut (*found_paint_entry).paint_elements,
            &mut paint_element as *mut cairo_svg_paint_element_t as *const libc::c_void,
        );
        if status as u64 != 0 {
            return status;
        }
        (*surface).transitive_paint_used = 1 as libc::c_int;
    }
    if pattern_id != invalid_pattern_id {
        if (*recording_surface).unbounded == 0 {} else {
            __assert_fail(
                b"!recording_surface->unbounded\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                2445 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 177],
                    &[libc::c_char; 177],
                >(
                    b"cairo_status_t _cairo_svg_surface_emit_composite_recording_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        _cairo_svg_stream_printf(
            output,
            b"<pattern id=\"pattern-%d\" patternUnits=\"userSpaceOnUse\" x=\"%f\" y=\"%f\" width=\"%f\" height=\"%f\" viewBox=\"%f %f %f %f\"\0"
                as *const u8 as *const libc::c_char,
            pattern_id,
            (*recording_surface).extents_pixels.x,
            (*recording_surface).extents_pixels.y,
            (*recording_surface).extents_pixels.width,
            (*recording_surface).extents_pixels.height,
            (*recording_surface).extents_pixels.x,
            (*recording_surface).extents_pixels.y,
            (*recording_surface).extents_pixels.width,
            (*recording_surface).extents_pixels.height,
        );
        _cairo_svg_surface_emit_transform(
            output,
            b"patternTransform\0" as *const u8 as *const libc::c_char,
            &mut p2u,
            parent_matrix,
        );
        _cairo_svg_stream_printf(output, b">\n\0" as *const u8 as *const libc::c_char);
    }
    _cairo_svg_stream_printf(
        output,
        b"<use xlink:href=\"#source-%d\"\0" as *const u8 as *const libc::c_char,
        source_id,
    );
    if pattern_id == invalid_pattern_id {
        _cairo_svg_surface_emit_transform(
            output,
            b"transform\0" as *const u8 as *const libc::c_char,
            &mut p2u,
            parent_matrix,
        );
    }
    _cairo_svg_stream_printf(output, b"/>\n\0" as *const u8 as *const libc::c_char);
    if pattern_id != invalid_pattern_id {
        _cairo_svg_stream_printf(
            output,
            b"</pattern>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_composite_pattern(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_surface_pattern_t,
    mut pattern_id: libc::c_uint,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    if pattern_id != invalid_pattern_id {
        if _cairo_svg_surface_svg_pattern_should_be_used(&mut (*pattern).base) != 0
        {} else {
            __assert_fail(
                b"_cairo_svg_surface_svg_pattern_should_be_used (&pattern->base)\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                2489 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 167],
                    &[libc::c_char; 167],
                >(
                    b"cairo_status_t _cairo_svg_surface_emit_composite_pattern(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_surface_pattern_t *, unsigned int, const cairo_matrix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*(*pattern).surface).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return _cairo_svg_surface_emit_composite_recording_pattern(
            output,
            surface,
            pattern,
            pattern_id,
            parent_matrix,
        )
    } else {
        return _cairo_svg_surface_emit_composite_surface_pattern(
            output,
            surface,
            pattern,
            pattern_id,
            parent_matrix,
        )
    };
}
unsafe extern "C" fn _cairo_svg_surface_emit_solid_pattern(
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_solid_pattern_t,
    mut output: *mut cairo_svg_stream_t,
    mut is_stroke: cairo_bool_t,
) -> cairo_status_t {
    _cairo_svg_stream_printf(
        output,
        if is_stroke != 0 {
            b" stroke=\"rgb(%f%%, %f%%, %f%%)\" stroke-opacity=\"%f\"\0" as *const u8
                as *const libc::c_char
        } else {
            b" fill=\"rgb(%f%%, %f%%, %f%%)\" fill-opacity=\"%f\"\0" as *const u8
                as *const libc::c_char
        },
        (*pattern).color.red * 100.0f64,
        (*pattern).color.green * 100.0f64,
        (*pattern).color.blue * 100.0f64,
        (*pattern).color.alpha,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_surface_pattern(
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_surface_pattern_t,
    mut output: *mut cairo_svg_stream_t,
    mut is_stroke: cairo_bool_t,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let ref mut fresh21 = (*document).pattern_id;
    let fresh22 = *fresh21;
    *fresh21 = (*fresh21).wrapping_add(1);
    let mut pattern_id: libc::c_uint = fresh22;
    status = _cairo_svg_surface_emit_composite_pattern(
        &mut (*document).xml_node_defs,
        surface,
        pattern,
        pattern_id,
        parent_matrix,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_svg_stream_printf(
        output,
        if is_stroke != 0 {
            b" stroke=\"url(#pattern-%d)\"\0" as *const u8 as *const libc::c_char
        } else {
            b" fill=\"url(#pattern-%d)\"\0" as *const u8 as *const libc::c_char
        },
        pattern_id,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_pattern_stops(
    mut output: *mut cairo_svg_stream_t,
    mut pattern: *const cairo_gradient_pattern_t,
    mut start_offset: libc::c_double,
    mut reverse_stops: cairo_bool_t,
    mut emulate_reflect: cairo_bool_t,
) -> cairo_status_t {
    let mut stops: *mut cairo_gradient_stop_t = 0 as *mut cairo_gradient_stop_t;
    let mut n_stops: libc::c_uint = 0;
    if (*pattern).n_stops < 1 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*pattern).n_stops == 1 as libc::c_int as libc::c_uint {
        _cairo_svg_stream_printf(
            output,
            b"<stop offset=\"%f\" stop-color=\"rgb(%f%%, %f%%, %f%%)\" stop-opacity=\"%f\"/>\n\0"
                as *const u8 as *const libc::c_char,
            (*((*pattern).stops).offset(0 as libc::c_int as isize)).offset,
            (*((*pattern).stops).offset(0 as libc::c_int as isize)).color.red * 100.0f64,
            (*((*pattern).stops).offset(0 as libc::c_int as isize)).color.green
                * 100.0f64,
            (*((*pattern).stops).offset(0 as libc::c_int as isize)).color.blue
                * 100.0f64,
            (*((*pattern).stops).offset(0 as libc::c_int as isize)).color.alpha,
        );
        return CAIRO_STATUS_SUCCESS;
    }
    if emulate_reflect != 0 || reverse_stops != 0 {
        n_stops = if emulate_reflect != 0 {
            ((*pattern).n_stops)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_sub(2 as libc::c_int as libc::c_uint)
        } else {
            (*pattern).n_stops
        };
        stops = _cairo_malloc_ab(
            n_stops as size_t,
            ::std::mem::size_of::<cairo_gradient_stop_t>() as libc::c_ulong,
        ) as *mut cairo_gradient_stop_t;
        if stops.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*pattern).n_stops {
            if reverse_stops != 0 {
                *stops
                    .offset(
                        i as isize,
                    ) = *((*pattern).stops)
                    .offset(
                        ((*pattern).n_stops)
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    );
                (*stops.offset(i as isize))
                    .offset = 1.0f64 - (*stops.offset(i as isize)).offset;
            } else {
                *stops.offset(i as isize) = *((*pattern).stops).offset(i as isize);
            }
            if emulate_reflect != 0 {
                (*stops.offset(i as isize)).offset *= 0.5f64;
                if i > 0 as libc::c_int as libc::c_uint
                    && i
                        < ((*pattern).n_stops)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    if reverse_stops != 0 {
                        *stops
                            .offset(
                                i
                                    .wrapping_add((*pattern).n_stops)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ) = *((*pattern).stops).offset(i as isize);
                        (*stops
                            .offset(
                                i
                                    .wrapping_add((*pattern).n_stops)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ))
                            .offset = 0.5f64
                            + 0.5f64
                                * (*stops
                                    .offset(
                                        i
                                            .wrapping_add((*pattern).n_stops)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                    ))
                                    .offset;
                    } else {
                        *stops
                            .offset(
                                i
                                    .wrapping_add((*pattern).n_stops)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ) = *((*pattern).stops)
                            .offset(
                                ((*pattern).n_stops)
                                    .wrapping_sub(i)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            );
                        (*stops
                            .offset(
                                i
                                    .wrapping_add((*pattern).n_stops)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ))
                            .offset = 1.0f64
                            - 0.5f64
                                * (*stops
                                    .offset(
                                        i
                                            .wrapping_add((*pattern).n_stops)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                    ))
                                    .offset;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        n_stops = (*pattern).n_stops;
        stops = (*pattern).stops;
    }
    if start_offset >= 0.0f64 {
        let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i_0 < n_stops {
            _cairo_svg_stream_printf(
                output,
                b"<stop offset=\"%f\" stop-color=\"rgb(%f%%, %f%%, %f%%)\" stop-opacity=\"%f\"/>\n\0"
                    as *const u8 as *const libc::c_char,
                start_offset
                    + (1.0f64 - start_offset) * (*stops.offset(i_0 as isize)).offset,
                (*stops.offset(i_0 as isize)).color.red * 100.0f64,
                (*stops.offset(i_0 as isize)).color.green * 100.0f64,
                (*stops.offset(i_0 as isize)).color.blue * 100.0f64,
                (*stops.offset(i_0 as isize)).color.alpha,
            );
            i_0 = i_0.wrapping_add(1);
        }
    } else {
        let mut found: cairo_bool_t = 0 as libc::c_int;
        let mut offset_index: libc::c_uint = 0;
        let mut offset_color_start: cairo_color_stop_t = cairo_color_stop_t {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.,
            red_short: 0,
            green_short: 0,
            blue_short: 0,
            alpha_short: 0,
        };
        let mut offset_color_stop: cairo_color_stop_t = cairo_color_stop_t {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.,
            red_short: 0,
            green_short: 0,
            blue_short: 0,
            alpha_short: 0,
        };
        let mut i_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i_1 <= n_stops {
            let mut x1: libc::c_double = if i_1 == n_stops {
                (*stops.offset(0 as libc::c_int as isize)).offset
                    + 1 as libc::c_int as libc::c_double
            } else {
                (*stops.offset(i_1 as isize)).offset
            };
            let mut color1: *mut cairo_color_stop_t = if i_1 == n_stops {
                &mut (*stops.offset(0 as libc::c_int as isize)).color
            } else {
                &mut (*stops.offset(i_1 as isize)).color
            };
            if x1 >= -start_offset {
                if i_1 > 0 as libc::c_int as libc::c_uint {
                    let mut x0: libc::c_double = (*stops
                        .offset(
                            i_1.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ))
                        .offset;
                    let mut color0: *mut cairo_color_stop_t = &mut (*stops
                        .offset(
                            i_1.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ))
                        .color;
                    if x0 != x1 {
                        offset_color_start
                            .red = (*color0).red
                            + ((*color1).red - (*color0).red) * (-start_offset - x0)
                                / (x1 - x0);
                        offset_color_start
                            .green = (*color0).green
                            + ((*color1).green - (*color0).green) * (-start_offset - x0)
                                / (x1 - x0);
                        offset_color_start
                            .blue = (*color0).blue
                            + ((*color1).blue - (*color0).blue) * (-start_offset - x0)
                                / (x1 - x0);
                        offset_color_start
                            .alpha = (*color0).alpha
                            + ((*color1).alpha - (*color0).alpha) * (-start_offset - x0)
                                / (x1 - x0);
                        offset_color_stop = offset_color_start;
                    } else {
                        offset_color_stop = (*stops
                            .offset(
                                i_1.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ))
                            .color;
                        offset_color_start = (*stops.offset(i_1 as isize)).color;
                    }
                } else {
                    offset_color_start = (*stops.offset(i_1 as isize)).color;
                    offset_color_stop = offset_color_start;
                }
                offset_index = i_1;
                found = 1 as libc::c_int;
                break;
            } else {
                i_1 = i_1.wrapping_add(1);
            }
        }
        if found == 0 {
            offset_index = n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint);
            offset_color_start = (*stops.offset(offset_index as isize)).color;
            offset_color_stop = offset_color_start;
        }
        _cairo_svg_stream_printf(
            output,
            b"<stop offset=\"0\" stop-color=\"rgb(%f%%, %f%%, %f%%)\" stop-opacity=\"%f\"/>\n\0"
                as *const u8 as *const libc::c_char,
            offset_color_start.red * 100.0f64,
            offset_color_start.green * 100.0f64,
            offset_color_start.blue * 100.0f64,
            offset_color_start.alpha,
        );
        let mut i_2: libc::c_uint = offset_index;
        while i_2 < n_stops {
            _cairo_svg_stream_printf(
                output,
                b"<stop offset=\"%f\" stop-color=\"rgb(%f%%, %f%%, %f%%)\" stop-opacity=\"%f\"/>\n\0"
                    as *const u8 as *const libc::c_char,
                (*stops.offset(i_2 as isize)).offset + start_offset,
                (*stops.offset(i_2 as isize)).color.red * 100.0f64,
                (*stops.offset(i_2 as isize)).color.green * 100.0f64,
                (*stops.offset(i_2 as isize)).color.blue * 100.0f64,
                (*stops.offset(i_2 as isize)).color.alpha,
            );
            i_2 = i_2.wrapping_add(1);
        }
        let mut i_3: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i_3 < offset_index {
            _cairo_svg_stream_printf(
                output,
                b"<stop offset=\"%f\" stop-color=\"rgb(%f%%, %f%%, %f%%)\" stop-opacity=\"%f\"/>\n\0"
                    as *const u8 as *const libc::c_char,
                1.0f64 + (*stops.offset(i_3 as isize)).offset + start_offset,
                (*stops.offset(i_3 as isize)).color.red * 100.0f64,
                (*stops.offset(i_3 as isize)).color.green * 100.0f64,
                (*stops.offset(i_3 as isize)).color.blue * 100.0f64,
                (*stops.offset(i_3 as isize)).color.alpha,
            );
            i_3 = i_3.wrapping_add(1);
        }
        _cairo_svg_stream_printf(
            output,
            b"<stop offset=\"1\" stop-color=\"rgb(%f%%, %f%%, %f%%)\" stop-opacity=\"%f\"/>\n\0"
                as *const u8 as *const libc::c_char,
            offset_color_stop.red * 100.0f64,
            offset_color_stop.green * 100.0f64,
            offset_color_stop.blue * 100.0f64,
            offset_color_stop.alpha,
        );
    }
    if reverse_stops != 0 || emulate_reflect != 0 {
        free(stops as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_pattern_extend(
    mut output: *mut cairo_svg_stream_t,
    mut pattern: *mut cairo_pattern_t,
) {
    match (*pattern).extend as libc::c_uint {
        1 => {
            _cairo_svg_stream_printf(
                output,
                b" spreadMethod=\"repeat\"\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            _cairo_svg_stream_printf(
                output,
                b" spreadMethod=\"reflect\"\0" as *const u8 as *const libc::c_char,
            );
        }
        0 | 3 | _ => {}
    };
}
unsafe extern "C" fn _cairo_svg_surface_emit_linear_pattern(
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_linear_pattern_t,
    mut output: *mut cairo_svg_stream_t,
    mut is_stroke: cairo_bool_t,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    let mut p2u: cairo_matrix_t = (*pattern).base.base.matrix;
    status = cairo_matrix_invert(&mut p2u);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            2741 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 163],
                &[libc::c_char; 163],
            >(
                b"cairo_status_t _cairo_svg_surface_emit_linear_pattern(cairo_svg_surface_t *, cairo_linear_pattern_t *, cairo_svg_stream_t *, cairo_bool_t, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh23 = (*document).linear_pattern_id;
    let fresh24 = *fresh23;
    *fresh23 = (*fresh23).wrapping_add(1);
    let mut linear_pattern_id: libc::c_uint = fresh24;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<linearGradient id=\"linear-pattern-%d\" gradientUnits=\"userSpaceOnUse\" x1=\"%f\" y1=\"%f\" x2=\"%f\" y2=\"%f\"\0"
            as *const u8 as *const libc::c_char,
        linear_pattern_id,
        (*pattern).pd1.x,
        (*pattern).pd1.y,
        (*pattern).pd2.x,
        (*pattern).pd2.y,
    );
    _cairo_svg_surface_emit_pattern_extend(
        &mut (*document).xml_node_defs,
        &mut (*pattern).base.base,
    );
    _cairo_svg_surface_emit_transform(
        &mut (*document).xml_node_defs,
        b"gradientTransform\0" as *const u8 as *const libc::c_char,
        &mut p2u,
        parent_matrix,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_svg_surface_emit_pattern_stops(
        &mut (*document).xml_node_defs,
        &mut (*pattern).base,
        0.0f64,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</linearGradient>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        output,
        if is_stroke != 0 {
            b" stroke=\"url(#linear-pattern-%d)\"\0" as *const u8 as *const libc::c_char
        } else {
            b" fill=\"url(#linear-pattern-%d)\"\0" as *const u8 as *const libc::c_char
        },
        linear_pattern_id,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_radial_pattern(
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *mut cairo_radial_pattern_t,
    mut output: *mut cairo_svg_stream_t,
    mut is_stroke: cairo_bool_t,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    let mut extend: cairo_extend_t = (*pattern).base.base.extend;
    let mut reverse_stops: cairo_bool_t = 0;
    let mut c0: *mut cairo_circle_double_t = 0 as *mut cairo_circle_double_t;
    let mut c1: *mut cairo_circle_double_t = 0 as *mut cairo_circle_double_t;
    if (*pattern).cd1.radius < (*pattern).cd2.radius {
        c0 = &mut (*pattern).cd1;
        c1 = &mut (*pattern).cd2;
        reverse_stops = 0 as libc::c_int;
    } else {
        c0 = &mut (*pattern).cd2;
        c1 = &mut (*pattern).cd1;
        reverse_stops = 1 as libc::c_int;
    }
    let mut x0: libc::c_double = (*c0).center.x;
    let mut y0: libc::c_double = (*c0).center.y;
    let mut r0: libc::c_double = (*c0).radius;
    let mut x1: libc::c_double = (*c1).center.x;
    let mut y1: libc::c_double = (*c1).center.y;
    let mut r1: libc::c_double = (*c1).radius;
    let mut p2u: cairo_matrix_t = (*pattern).base.base.matrix;
    status = cairo_matrix_invert(&mut p2u);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
            2810 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 163],
                &[libc::c_char; 163],
            >(
                b"cairo_status_t _cairo_svg_surface_emit_radial_pattern(cairo_svg_surface_t *, cairo_radial_pattern_t *, cairo_svg_stream_t *, cairo_bool_t, const cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh25 = (*document).radial_pattern_id;
    let fresh26 = *fresh25;
    *fresh25 = (*fresh25).wrapping_add(1);
    let mut radial_pattern_id: libc::c_uint = fresh26;
    let mut start_offset: libc::c_double = 0.;
    let mut emulate_reflect: cairo_bool_t = 0 as libc::c_int;
    let mut fx: libc::c_double = (r1 * x0 - r0 * x1) / (r1 - r0);
    let mut fy: libc::c_double = (r1 * y0 - r0 * y1) / (r1 - r0);
    if (extend as libc::c_uint == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
        || extend as libc::c_uint == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint)
        && r0 > 0.0f64
    {
        let mut r_org: libc::c_double = r1;
        if extend as libc::c_uint == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
        {
            r1 = 2.0f64 * r1 - r0;
            emulate_reflect = 1 as libc::c_int;
        }
        start_offset = fmod(r1, r1 - r0) / (r1 - r0) - 1.0f64;
        let mut r: libc::c_double = r1 - r0;
        let mut x: libc::c_double = r * (x1 - fx) / r_org + fx;
        let mut y: libc::c_double = r * (y1 - fy) / r_org + fy;
        x1 = x;
        y1 = y;
        r1 = r;
        r0 = 0.0f64;
    } else {
        start_offset = r0 / r1;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<radialGradient id=\"radial-pattern-%d\" gradientUnits=\"userSpaceOnUse\" cx=\"%f\" cy=\"%f\" fx=\"%f\" fy=\"%f\" r=\"%f\"\0"
            as *const u8 as *const libc::c_char,
        radial_pattern_id,
        x1,
        y1,
        fx,
        fy,
        r1,
    );
    if emulate_reflect != 0 {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b" spreadMethod=\"repeat\"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_svg_surface_emit_pattern_extend(
            &mut (*document).xml_node_defs,
            &mut (*pattern).base.base,
        );
    }
    _cairo_svg_surface_emit_transform(
        &mut (*document).xml_node_defs,
        b"gradientTransform\0" as *const u8 as *const libc::c_char,
        &mut p2u,
        parent_matrix,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    if extend as libc::c_uint == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<stop offset=\"0\" stop-color=\"rgb(0%%, 0%%, 0%%)\" stop-opacity=\"0\"/>\n\0"
                as *const u8 as *const libc::c_char,
        );
        if r0 != 0.0f64 {
            _cairo_svg_stream_printf(
                &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
                b"<stop offset=\"%f\" stop-color=\"rgb(0%%, 0%%, 0%%)\" stop-opacity=\"0\"/>\n\0"
                    as *const u8 as *const libc::c_char,
                r0 / r1,
            );
        }
    }
    status = _cairo_svg_surface_emit_pattern_stops(
        &mut (*document).xml_node_defs,
        &mut (*pattern).base,
        start_offset,
        reverse_stops,
        emulate_reflect,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*pattern).base.base.extend as libc::c_uint
        == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<stop offset=\"1\" stop-color=\"rgb(0%%, 0%%, 0%%)\" stop-opacity=\"0\"/>\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</radialGradient>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        output,
        if is_stroke != 0 {
            b" stroke=\"url(#radial-pattern-%d)\"\0" as *const u8 as *const libc::c_char
        } else {
            b" fill=\"url(#radial-pattern-%d)\"\0" as *const u8 as *const libc::c_char
        },
        radial_pattern_id,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_emit_pattern(
    mut surface: *mut cairo_svg_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut output: *mut cairo_svg_stream_t,
    mut is_stroke: cairo_bool_t,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            return _cairo_svg_surface_emit_solid_pattern(
                surface,
                pattern as *mut cairo_solid_pattern_t,
                output,
                is_stroke,
            );
        }
        1 => {
            return _cairo_svg_surface_emit_surface_pattern(
                surface,
                pattern as *mut cairo_surface_pattern_t,
                output,
                is_stroke,
                parent_matrix,
            );
        }
        2 => {
            return _cairo_svg_surface_emit_linear_pattern(
                surface,
                pattern as *mut cairo_linear_pattern_t,
                output,
                is_stroke,
                parent_matrix,
            );
        }
        3 => {
            return _cairo_svg_surface_emit_radial_pattern(
                surface,
                pattern as *mut cairo_radial_pattern_t,
                output,
                is_stroke,
                parent_matrix,
            );
        }
        4 | 5 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    2940 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 155],
                        &[libc::c_char; 155],
                    >(
                        b"cairo_status_t _cairo_svg_surface_emit_pattern(cairo_svg_surface_t *, const cairo_pattern_t *, cairo_svg_stream_t *, cairo_bool_t, const cairo_matrix_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        _ => {}
    }
    return _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH);
}
unsafe extern "C" fn _cairo_svg_surface_emit_fill_style(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut fill_rule: cairo_fill_rule_t,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    _cairo_svg_stream_printf(
        output,
        b" fill-rule=\"%s\"\0" as *const u8 as *const libc::c_char,
        if fill_rule as libc::c_uint
            == CAIRO_FILL_RULE_EVEN_ODD as libc::c_int as libc::c_uint
        {
            b"evenodd\0" as *const u8 as *const libc::c_char
        } else {
            b"nonzero\0" as *const u8 as *const libc::c_char
        },
    );
    return _cairo_svg_surface_emit_pattern(
        surface,
        source,
        output,
        0 as libc::c_int,
        parent_matrix,
    );
}
unsafe extern "C" fn _cairo_svg_surface_emit_stroke_style(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut parent_matrix: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut line_cap: *const libc::c_char = 0 as *const libc::c_char;
    let mut line_join: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    match (*stroke_style).line_cap as libc::c_uint {
        0 => {
            line_cap = b"butt\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            line_cap = b"round\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            line_cap = b"square\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    2980 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 176],
                        &[libc::c_char; 176],
                    >(
                        b"cairo_status_t _cairo_svg_surface_emit_stroke_style(cairo_svg_stream_t *, cairo_svg_surface_t *, const cairo_pattern_t *, const cairo_stroke_style_t *, const cairo_matrix_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    match (*stroke_style).line_join as libc::c_uint {
        0 => {
            line_join = b"miter\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            line_join = b"round\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            line_join = b"bevel\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    2994 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 176],
                        &[libc::c_char; 176],
                    >(
                        b"cairo_status_t _cairo_svg_surface_emit_stroke_style(cairo_svg_stream_t *, cairo_svg_surface_t *, const cairo_pattern_t *, const cairo_stroke_style_t *, const cairo_matrix_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    if (*stroke_style).is_hairline != 0 {
        _cairo_svg_stream_printf(
            output,
            b" stroke-width=\"1px\" stroke-linecap=\"%s\" stroke-linejoin=\"%s\" style=\"vector-effect: non-scaling-stroke\"\0"
                as *const u8 as *const libc::c_char,
            line_cap,
            line_join,
        );
    } else {
        _cairo_svg_stream_printf(
            output,
            b" stroke-width=\"%f\" stroke-linecap=\"%s\" stroke-linejoin=\"%s\"\0"
                as *const u8 as *const libc::c_char,
            (*stroke_style).line_width,
            line_cap,
            line_join,
        );
    }
    status = _cairo_svg_surface_emit_pattern(
        surface,
        source,
        output,
        1 as libc::c_int,
        parent_matrix,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*stroke_style).num_dashes > 0 as libc::c_int as libc::c_uint {
        _cairo_svg_stream_printf(
            output,
            b" stroke-dasharray=\"\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*stroke_style).num_dashes {
            _cairo_svg_stream_printf(
                output,
                b"%f\0" as *const u8 as *const libc::c_char,
                *((*stroke_style).dash).offset(i as isize),
            );
            _cairo_svg_stream_printf(
                output,
                if i.wrapping_add(1 as libc::c_int as libc::c_uint)
                    < (*stroke_style).num_dashes
                {
                    b" \0" as *const u8 as *const libc::c_char
                } else {
                    b"\"\0" as *const u8 as *const libc::c_char
                },
            );
            i = i.wrapping_add(1);
        }
        if (*stroke_style).dash_offset != 0.0f64 {
            _cairo_svg_stream_printf(
                output,
                b" stroke-dashoffset=\"%f\"\0" as *const u8 as *const libc::c_char,
                (*stroke_style).dash_offset,
            );
        }
    }
    _cairo_svg_stream_printf(
        output,
        b" stroke-miterlimit=\"%f\"\0" as *const u8 as *const libc::c_char,
        (*stroke_style).miter_limit,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    (*rectangle).x = 0 as libc::c_int;
    (*rectangle).y = 0 as libc::c_int;
    (*rectangle).width = ceil((*surface).width) as libc::c_int;
    (*rectangle).height = ceil((*surface).height) as libc::c_int;
    return (*surface).surface_bounded;
}
unsafe extern "C" fn _cairo_svg_surface_emit_paint(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut at_origin: cairo_bool_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _cairo_svg_surface_svg_clip_or_svg_mask_should_be_used(source) != 0 {
        return _cairo_svg_surface_emit_composite_pattern(
            output,
            surface,
            source as *mut cairo_surface_pattern_t,
            invalid_pattern_id,
            0 as *const cairo_matrix_t,
        );
    }
    (*surface).transitive_paint_used = 1 as libc::c_int;
    _cairo_svg_stream_printf(output, b"<rect\0" as *const u8 as *const libc::c_char);
    if at_origin != 0 {
        _cairo_svg_stream_append_paint_dependent(
            output,
            (*surface).source_id,
            CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE_AT_ORIGIN,
        );
    } else {
        _cairo_svg_stream_append_paint_dependent(
            output,
            (*surface).source_id,
            CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_RECTANGLE,
        );
    }
    status = _cairo_svg_surface_emit_pattern(
        surface,
        source,
        output,
        0 as libc::c_int,
        0 as *const cairo_matrix_t,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_svg_stream_printf(output, b"/>\n\0" as *const u8 as *const libc::c_char);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_surface_do_operator(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut op: cairo_operator_t,
    mut clip: *const cairo_clip_t,
    mut mask_stream: *mut cairo_svg_stream_t,
    mut source_stream: *mut cairo_svg_stream_t,
    mut destination_stream: *mut cairo_svg_stream_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    if (*surface).base.content as libc::c_uint
        == CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
        && (op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_IN as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_OUT as libc::c_int as libc::c_uint
            || op as libc::c_uint
                == CAIRO_OPERATOR_DEST_IN as libc::c_int as libc::c_uint
            || op as libc::c_uint
                == CAIRO_OPERATOR_DEST_OUT as libc::c_int as libc::c_uint
            || op as libc::c_uint
                == CAIRO_OPERATOR_DEST_ATOP as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_XOR as libc::c_int as libc::c_uint)
    {
        _cairo_svg_surface_emit_paint(
            output,
            surface,
            &_cairo_pattern_black.base,
            0 as libc::c_int,
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
        status = _cairo_svg_stream_destroy(source_stream);
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(destination_stream);
            _cairo_svg_stream_destroy(mask_stream);
            return status as cairo_int_status_t;
        }
        let mut empty_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        return _cairo_svg_surface_do_operator(
            output,
            surface,
            CAIRO_OPERATOR_SOURCE,
            clip,
            mask_stream,
            &mut empty_stream,
            destination_stream,
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
        let ref mut fresh27 = (*document).compositing_group_id;
        let fresh28 = *fresh27;
        *fresh27 = (*fresh27).wrapping_add(1);
        let mut lerp_compositing_group_id: libc::c_uint = fresh28;
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<g id=\"compositing-group-%d\"\0" as *const u8 as *const libc::c_char,
            lerp_compositing_group_id,
        );
        _cairo_svg_stream_append_paint_dependent(
            &mut (*document).xml_node_defs,
            (*surface).source_id,
            CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b">\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_surface_emit_paint(
            &mut (*document).xml_node_defs,
            surface,
            &_cairo_pattern_clear.base,
            0 as libc::c_int,
        );
        status = _cairo_svg_surface_set_clip(
            surface,
            &mut (*document).xml_node_defs,
            clip,
        );
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(destination_stream);
            _cairo_svg_stream_destroy(source_stream);
            _cairo_svg_stream_destroy(mask_stream);
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_copy(mask_stream, &mut (*document).xml_node_defs);
        status = _cairo_svg_stream_destroy(mask_stream);
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(destination_stream);
            _cairo_svg_stream_destroy(source_stream);
            return status as cairo_int_status_t;
        }
        _cairo_svg_surface_reset_clip(surface);
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh29 = (*document).mask_id;
        let fresh30 = *fresh29;
        *fresh29 = (*fresh29).wrapping_add(1);
        let mut positive_lerp_mask_id: libc::c_uint = fresh30;
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
            positive_lerp_mask_id,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<use xlink:href=\"#compositing-group-%d\"/>\n\0" as *const u8
                as *const libc::c_char,
            lerp_compositing_group_id,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</mask>\n\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh31 = (*document).mask_id;
        let fresh32 = *fresh31;
        *fresh31 = (*fresh31).wrapping_add(1);
        let mut negative_lerp_mask_id: libc::c_uint = fresh32;
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
            negative_lerp_mask_id,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<use xlink:href=\"#compositing-group-%d\" filter=\"url(#filter-%s)\"/>\n\0"
                as *const u8 as *const libc::c_char,
            lerp_compositing_group_id,
            _cairo_svg_surface_emit_static_filter(
                document,
                CAIRO_SVG_FILTER_REMOVE_COLOR_AND_INVERT_ALPHA,
            ),
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</mask>\n\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh33 = (*document).compositing_group_id;
        let fresh34 = *fresh33;
        *fresh33 = (*fresh33).wrapping_add(1);
        let mut lerped_source_compositing_group_id: libc::c_uint = fresh34;
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<g id=\"compositing-group-%d\" mask=\"url(#mask-%d)\">\n\0" as *const u8
                as *const libc::c_char,
            lerped_source_compositing_group_id,
            positive_lerp_mask_id,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<g\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_append_paint_dependent(
            &mut (*document).xml_node_defs,
            (*surface).source_id,
            CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b">\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_copy(source_stream, &mut (*document).xml_node_defs);
        status = _cairo_svg_stream_destroy(source_stream);
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(destination_stream);
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh35 = (*document).compositing_group_id;
        let fresh36 = *fresh35;
        *fresh35 = (*fresh35).wrapping_add(1);
        let mut lerped_destination_compositing_group_id: libc::c_uint = fresh36;
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<g id=\"compositing-group-%d\" mask=\"url(#mask-%d)\">\n\0" as *const u8
                as *const libc::c_char,
            lerped_destination_compositing_group_id,
            negative_lerp_mask_id,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<g\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_append_paint_dependent(
            &mut (*document).xml_node_defs,
            (*surface).source_id,
            CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b">\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_copy(destination_stream, &mut (*document).xml_node_defs);
        status = _cairo_svg_stream_destroy(destination_stream);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_printf(
            &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_printf(
            &mut (*surface).xml_node as *mut cairo_svg_stream_t,
            b"<g filter=\"url(#filter-%d)\"\0" as *const u8 as *const libc::c_char,
            _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_ADD,
                lerped_source_compositing_group_id,
                lerped_destination_compositing_group_id,
            ),
        );
        _cairo_svg_stream_append_paint_dependent(
            &mut (*surface).xml_node,
            (*surface).source_id,
            CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_TRANSLATION,
        );
        _cairo_svg_stream_printf(
            &mut (*surface).xml_node as *mut cairo_svg_stream_t,
            b">\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_svg_surface_emit_paint(
            &mut (*surface).xml_node,
            surface,
            &_cairo_pattern_black.base,
            1 as libc::c_int,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_printf(
            &mut (*surface).xml_node as *mut cairo_svg_stream_t,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_DEST as libc::c_int as libc::c_uint {
        _cairo_svg_stream_copy(destination_stream, &mut (*surface).xml_node);
        status = _cairo_svg_stream_destroy(destination_stream);
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(source_stream);
            _cairo_svg_stream_destroy(mask_stream);
            return status as cairo_int_status_t;
        }
        status = _cairo_svg_stream_destroy(source_stream);
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(source_stream);
            return status as cairo_int_status_t;
        }
        status = _cairo_svg_stream_destroy(source_stream);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    let ref mut fresh37 = (*document).compositing_group_id;
    let fresh38 = *fresh37;
    *fresh37 = (*fresh37).wrapping_add(1);
    let mut lerp_compositing_group_id_0: libc::c_uint = fresh38;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g id=\"compositing-group-%d\"\0" as *const u8 as *const libc::c_char,
        lerp_compositing_group_id_0,
    );
    _cairo_svg_stream_append_paint_dependent(
        &mut (*document).xml_node_defs,
        (*surface).source_id,
        CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_surface_emit_paint(
        &mut (*document).xml_node_defs,
        surface,
        &_cairo_pattern_clear.base,
        0 as libc::c_int,
    );
    status = _cairo_svg_surface_set_clip(surface, &mut (*document).xml_node_defs, clip);
    if status as u64 != 0 {
        _cairo_svg_stream_destroy(destination_stream);
        _cairo_svg_stream_destroy(source_stream);
        _cairo_svg_stream_destroy(mask_stream);
        return status as cairo_int_status_t;
    }
    status = _cairo_svg_surface_emit_paint(
        &mut (*document).xml_node_defs,
        surface,
        &_cairo_pattern_white.base,
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        _cairo_svg_stream_destroy(destination_stream);
        _cairo_svg_stream_destroy(source_stream);
        _cairo_svg_stream_destroy(mask_stream);
        return status as cairo_int_status_t;
    }
    _cairo_svg_surface_reset_clip(surface);
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh39 = (*document).mask_id;
    let fresh40 = *fresh39;
    *fresh39 = (*fresh39).wrapping_add(1);
    let mut positive_lerp_mask_id_0: libc::c_uint = fresh40;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
        positive_lerp_mask_id_0,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<use xlink:href=\"#compositing-group-%d\"/>\n\0" as *const u8
            as *const libc::c_char,
        lerp_compositing_group_id_0,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</mask>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh41 = (*document).mask_id;
    let fresh42 = *fresh41;
    *fresh41 = (*fresh41).wrapping_add(1);
    let mut negative_lerp_mask_id_0: libc::c_uint = fresh42;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
        negative_lerp_mask_id_0,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<use xlink:href=\"#compositing-group-%d\" filter=\"url(#filter-%s)\"/>\n\0"
            as *const u8 as *const libc::c_char,
        lerp_compositing_group_id_0,
        _cairo_svg_surface_emit_static_filter(
            document,
            CAIRO_SVG_FILTER_REMOVE_COLOR_AND_INVERT_ALPHA,
        ),
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</mask>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh43 = (*document).mask_id;
    let fresh44 = *fresh43;
    *fresh43 = (*fresh43).wrapping_add(1);
    let mut mask_mask_id: libc::c_uint = fresh44;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
        mask_mask_id,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_append_paint_dependent(
        &mut (*document).xml_node_defs,
        (*surface).source_id,
        CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_copy(mask_stream, &mut (*document).xml_node_defs);
    status = _cairo_svg_stream_destroy(mask_stream);
    if status as u64 != 0 {
        _cairo_svg_stream_destroy(source_stream);
        _cairo_svg_stream_destroy(destination_stream);
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</mask>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh45 = (*document).compositing_group_id;
    let fresh46 = *fresh45;
    *fresh45 = (*fresh45).wrapping_add(1);
    let mut masked_source_compositing_group_id: libc::c_uint = fresh46;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g id=\"compositing-group-%d\" mask=\"url(#mask-%d)\">\n\0" as *const u8
            as *const libc::c_char,
        masked_source_compositing_group_id,
        mask_mask_id,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_append_paint_dependent(
        &mut (*document).xml_node_defs,
        (*surface).source_id,
        CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_copy(source_stream, &mut (*document).xml_node_defs);
    status = _cairo_svg_stream_destroy(source_stream);
    if status as u64 != 0 {
        _cairo_svg_stream_destroy(destination_stream);
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh47 = (*document).compositing_group_id;
    let fresh48 = *fresh47;
    *fresh47 = (*fresh47).wrapping_add(1);
    let mut destination_compositing_group_id: libc::c_uint = fresh48;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g id=\"compositing-group-%d\"\0" as *const u8 as *const libc::c_char,
        destination_compositing_group_id,
    );
    _cairo_svg_stream_append_paint_dependent(
        &mut (*document).xml_node_defs,
        (*surface).source_id,
        CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_INVERSE_TRANSLATION,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_copy(destination_stream, &mut (*document).xml_node_defs);
    status = _cairo_svg_stream_destroy(destination_stream);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh49 = (*document).compositing_group_id;
    let fresh50 = *fresh49;
    *fresh49 = (*fresh49).wrapping_add(1);
    let mut lerped_operation_compositing_group_id: libc::c_uint = fresh50;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g id=\"compositing-group-%d\"\0" as *const u8 as *const libc::c_char,
        lerped_operation_compositing_group_id,
    );
    let mut filter_id: libc::c_uint = 0;
    let mut current_block_187: u64;
    match op as libc::c_uint {
        0 | 1 | 2 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    3419 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 201],
                        &[libc::c_char; 201],
                    >(
                        b"cairo_int_status_t _cairo_svg_surface_do_operator(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_operator_t, const cairo_clip_t *, cairo_svg_stream_t *, cairo_svg_stream_t *, cairo_svg_stream_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_187 = 12223342419967878574;
        }
        3 => {
            current_block_187 = 12223342419967878574;
        }
        4 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_OUT,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        5 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_ATOP,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        6 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    3439 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 201],
                        &[libc::c_char; 201],
                    >(
                        b"cairo_int_status_t _cairo_svg_surface_do_operator(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_operator_t, const cairo_clip_t *, cairo_svg_stream_t *, cairo_svg_stream_t *, cairo_svg_stream_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_187 = 3068239094627381543;
        }
        7 => {
            current_block_187 = 3068239094627381543;
        }
        8 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_IN,
                destination_compositing_group_id,
                masked_source_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        9 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_OUT,
                destination_compositing_group_id,
                masked_source_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        10 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_ATOP,
                destination_compositing_group_id,
                masked_source_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        11 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_XOR,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        12 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_ADD,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        13 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    3477 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 201],
                        &[libc::c_char; 201],
                    >(
                        b"cairo_int_status_t _cairo_svg_surface_do_operator(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_operator_t, const cairo_clip_t *, cairo_svg_stream_t *, cairo_svg_stream_t *, cairo_svg_stream_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_187 = 4784651426634242263;
        }
        14 => {
            current_block_187 = 4784651426634242263;
        }
        15 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_SCREEN,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        16 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_OVERLAY,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        17 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_DARKEN,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        18 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_LIGHTEN,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        19 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_COLOR_DODGE,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        20 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_COLOR_BURN,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        21 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_HARD_LIGHT,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        22 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_SOFT_LIGHT,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        23 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_DIFFERENCE,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        24 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_EXCLUSION,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        25 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_HUE,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        26 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_SATURATION,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        27 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_COLOR,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        28 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_LUMINOSITY,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
            current_block_187 = 17322559809113443968;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-svg-surface.c\0" as *const u8 as *const libc::c_char,
                    3569 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 201],
                        &[libc::c_char; 201],
                    >(
                        b"cairo_int_status_t _cairo_svg_surface_do_operator(cairo_svg_stream_t *, cairo_svg_surface_t *, cairo_operator_t, const cairo_clip_t *, cairo_svg_stream_t *, cairo_svg_stream_t *, cairo_svg_stream_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_187 = 17322559809113443968;
        }
    }
    match current_block_187 {
        12223342419967878574 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_IN,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        3068239094627381543 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_OVER,
                destination_compositing_group_id,
                masked_source_compositing_group_id,
            );
        }
        4784651426634242263 => {
            filter_id = _cairo_svg_surface_emit_parametric_filter(
                surface,
                CAIRO_SVG_FILTER_MULTIPLY,
                masked_source_compositing_group_id,
                destination_compositing_group_id,
            );
        }
        _ => {}
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b" filter=\"url(#filter-%d)\" mask=\"url(#mask-%d)\">\n\0" as *const u8
            as *const libc::c_char,
        filter_id,
        positive_lerp_mask_id_0,
    );
    status = _cairo_svg_surface_emit_paint(
        &mut (*document).xml_node_defs,
        surface,
        &_cairo_pattern_black.base,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh51 = (*document).compositing_group_id;
    let fresh52 = *fresh51;
    *fresh51 = (*fresh51).wrapping_add(1);
    let mut lerped_destination_compositing_group_id_0: libc::c_uint = fresh52;
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<g id=\"compositing-group-%d\" mask=\"url(#mask-%d)\">\n\0" as *const u8
            as *const libc::c_char,
        lerped_destination_compositing_group_id_0,
        negative_lerp_mask_id_0,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"<use xlink:href=\"#compositing-group-%d\"/>\n\0" as *const u8
            as *const libc::c_char,
        destination_compositing_group_id,
    );
    _cairo_svg_stream_printf(
        &mut (*document).xml_node_defs as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        &mut (*surface).xml_node as *mut cairo_svg_stream_t,
        b"<g filter=\"url(#filter-%d)\"\0" as *const u8 as *const libc::c_char,
        _cairo_svg_surface_emit_parametric_filter(
            surface,
            CAIRO_SVG_FILTER_ADD,
            lerped_operation_compositing_group_id,
            lerped_destination_compositing_group_id_0,
        ),
    );
    _cairo_svg_stream_append_paint_dependent(
        &mut (*surface).xml_node,
        (*surface).source_id,
        CAIRO_SVG_STREAM_PAINT_DEPENDENT_ELEMENT_TYPE_TRANSLATION,
    );
    _cairo_svg_stream_printf(
        &mut (*surface).xml_node as *mut cairo_svg_stream_t,
        b">\n\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_svg_surface_emit_paint(
        &mut (*surface).xml_node,
        surface,
        &_cairo_pattern_black.base,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut (*surface).xml_node as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_paint_impl(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
) -> cairo_int_status_t {
    return _cairo_svg_surface_emit_paint(output, surface, source, 0 as libc::c_int)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    if (op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint)
        && clip.is_null()
    {
        match (*surface).paginated_mode as libc::c_uint {
            0 => return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t,
            1 => {
                status = _cairo_svg_stream_destroy(&mut (*surface).xml_node);
                if status as u64 != 0 {
                    return status as cairo_int_status_t;
                }
                (*surface).xml_node = _cairo_svg_stream_create();
                if op as libc::c_uint
                    == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
                {
                    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
                }
            }
            2 => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-svg-surface.c\0" as *const u8
                            as *const libc::c_char,
                        3684 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 117],
                            &[libc::c_char; 117],
                        >(
                            b"cairo_int_status_t _cairo_svg_surface_paint(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_clip_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
            _ => {}
        }
    } else if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return (if _cairo_svg_surface_are_operation_and_pattern_supported(
            surface,
            op,
            source,
        ) != 0
        {
            CAIRO_STATUS_SUCCESS as libc::c_int
        } else {
            CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        }) as cairo_int_status_t
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        status = _cairo_svg_surface_set_clip(surface, &mut (*surface).xml_node, clip);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        return _cairo_svg_surface_paint_impl(&mut (*surface).xml_node, surface, source);
    } else {
        _cairo_svg_surface_reset_clip(surface);
        let mut mask_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_paint_impl(
            &mut mask_stream,
            surface,
            &_cairo_pattern_white.base,
        ) as cairo_status_t;
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut source_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_emit_paint(
            &mut source_stream,
            surface,
            source,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut source_stream);
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut destination_stream: cairo_svg_stream_t = (*surface).xml_node;
        (*surface).xml_node = _cairo_svg_stream_create();
        return _cairo_svg_surface_do_operator(
            &mut (*surface).xml_node,
            surface,
            op,
            clip,
            &mut mask_stream,
            &mut source_stream,
            &mut destination_stream,
        );
    };
}
unsafe extern "C" fn _cairo_svg_surface_mask_impl(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    let mut temporary_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
    let ref mut fresh53 = (*document).mask_id;
    let fresh54 = *fresh53;
    *fresh53 = (*fresh53).wrapping_add(1);
    let mut mask_id: libc::c_uint = fresh54;
    _cairo_svg_stream_printf(
        &mut temporary_stream as *mut cairo_svg_stream_t,
        b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
        mask_id,
    );
    _cairo_svg_stream_printf(
        &mut temporary_stream as *mut cairo_svg_stream_t,
        b"<g filter=\"url(#filter-%s)\">\n\0" as *const u8 as *const libc::c_char,
        _cairo_svg_surface_emit_static_filter(document, CAIRO_SVG_FILTER_REMOVE_COLOR),
    );
    status = _cairo_svg_surface_emit_paint(
        &mut temporary_stream,
        surface,
        mask,
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        _cairo_svg_stream_destroy(&mut temporary_stream);
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut temporary_stream as *mut cairo_svg_stream_t,
        b"</g>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_printf(
        &mut temporary_stream as *mut cairo_svg_stream_t,
        b"</mask>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_svg_stream_copy(&mut temporary_stream, &mut (*document).xml_node_defs);
    status = _cairo_svg_stream_destroy(&mut temporary_stream);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        output,
        b"<g mask=\"url(#mask-%d)\">\n\0" as *const u8 as *const libc::c_char,
        mask_id,
    );
    status = _cairo_svg_surface_emit_paint(output, surface, source, 0 as libc::c_int);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(output, b"</g>\n\0" as *const u8 as *const libc::c_char);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return (if _cairo_svg_surface_are_operation_and_pattern_supported(
            surface,
            op,
            source,
        ) != 0
            && _cairo_svg_surface_are_operation_and_pattern_supported(surface, op, mask)
                != 0
        {
            CAIRO_STATUS_SUCCESS as libc::c_int
        } else {
            CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        }) as cairo_int_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        status = _cairo_svg_surface_set_clip(surface, &mut (*surface).xml_node, clip);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        return _cairo_svg_surface_mask_impl(
            &mut (*surface).xml_node,
            surface,
            source,
            mask,
        );
    } else {
        _cairo_svg_surface_reset_clip(surface);
        let mut mask_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_mask_impl(
            &mut mask_stream,
            surface,
            &_cairo_pattern_white.base,
            mask,
        ) as cairo_status_t;
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut source_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_emit_paint(
            &mut source_stream,
            surface,
            source,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut source_stream);
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut destination_stream: cairo_svg_stream_t = (*surface).xml_node;
        (*surface).xml_node = _cairo_svg_stream_create();
        return _cairo_svg_surface_do_operator(
            &mut (*surface).xml_node,
            surface,
            op,
            clip,
            &mut mask_stream,
            &mut source_stream,
            &mut destination_stream,
        );
    };
}
unsafe extern "C" fn _cairo_svg_surface_stroke_impl(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut svg_clip_or_svg_mask_should_be_used: cairo_bool_t = _cairo_svg_surface_svg_clip_or_svg_mask_should_be_used(
        source,
    );
    let mut mask_id: libc::c_uint = 0;
    let mut output_stream: *mut cairo_svg_stream_t = output;
    if svg_clip_or_svg_mask_should_be_used != 0 {
        let ref mut fresh55 = (*(*surface).document).mask_id;
        let fresh56 = *fresh55;
        *fresh55 = (*fresh55).wrapping_add(1);
        mask_id = fresh56;
        output_stream = &mut (*(*surface).document).xml_node_defs;
        _cairo_svg_stream_printf(
            output_stream,
            b"<mask id=\"mask-%d\">\n\0" as *const u8 as *const libc::c_char,
            mask_id,
        );
    }
    _cairo_svg_stream_printf(
        output_stream,
        b"<path fill=\"none\"\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_svg_surface_emit_stroke_style(
        output_stream,
        surface,
        if svg_clip_or_svg_mask_should_be_used != 0 {
            &_cairo_pattern_white.base
        } else {
            source
        },
        stroke_style,
        ctm_inverse,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_surface_emit_path(output_stream, path, ctm_inverse);
    _cairo_svg_surface_emit_transform(
        output_stream,
        b"transform\0" as *const u8 as *const libc::c_char,
        ctm,
        0 as *const cairo_matrix_t,
    );
    _cairo_svg_stream_printf(
        output_stream,
        b"/>\n\0" as *const u8 as *const libc::c_char,
    );
    if svg_clip_or_svg_mask_should_be_used != 0 {
        _cairo_svg_stream_printf(
            output_stream,
            b"</mask>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_printf(
            output,
            b"<g mask=\"url(#mask-%d)\">\n\0" as *const u8 as *const libc::c_char,
            mask_id,
        );
        status = _cairo_svg_surface_emit_composite_pattern(
            output,
            surface,
            source as *mut cairo_surface_pattern_t,
            invalid_pattern_id,
            0 as *const cairo_matrix_t,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_printf(
            output,
            b"</g>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_stroke(
    mut abstract_dst: *mut libc::c_void,
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
    let mut surface: *mut cairo_svg_surface_t = abstract_dst as *mut cairo_svg_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return (if _cairo_svg_surface_are_operation_and_pattern_supported(
            surface,
            op,
            source,
        ) != 0
        {
            CAIRO_STATUS_SUCCESS as libc::c_int
        } else {
            CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        }) as cairo_int_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        status = _cairo_svg_surface_set_clip(surface, &mut (*surface).xml_node, clip);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        return _cairo_svg_surface_stroke_impl(
            &mut (*surface).xml_node,
            surface,
            source,
            path,
            stroke_style,
            ctm,
            ctm_inverse,
            tolerance,
            antialias,
        );
    } else {
        _cairo_svg_surface_reset_clip(surface);
        let mut mask_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_stroke_impl(
            &mut mask_stream,
            surface,
            &_cairo_pattern_white.base,
            path,
            stroke_style,
            ctm,
            ctm_inverse,
            tolerance,
            antialias,
        ) as cairo_status_t;
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut source_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_emit_paint(
            &mut source_stream,
            surface,
            source,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut source_stream);
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut destination_stream: cairo_svg_stream_t = (*surface).xml_node;
        (*surface).xml_node = _cairo_svg_stream_create();
        return _cairo_svg_surface_do_operator(
            &mut (*surface).xml_node,
            surface,
            op,
            clip,
            &mut mask_stream,
            &mut source_stream,
            &mut destination_stream,
        );
    };
}
unsafe extern "C" fn _cairo_svg_surface_fill_impl(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _cairo_svg_surface_svg_clip_or_svg_mask_should_be_used(source) != 0 {
        _cairo_svg_stream_printf(
            &mut (*(*surface).document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<clipPath id=\"clip-%d\">\n\0" as *const u8 as *const libc::c_char,
            (*(*surface).document).clip_id,
        );
        _cairo_svg_stream_printf(
            &mut (*(*surface).document).xml_node_defs as *mut cairo_svg_stream_t,
            b"<path clip-rule=\"%s\"\0" as *const u8 as *const libc::c_char,
            if fill_rule as libc::c_uint
                == CAIRO_FILL_RULE_EVEN_ODD as libc::c_int as libc::c_uint
            {
                b"evenodd\0" as *const u8 as *const libc::c_char
            } else {
                b"nonzero\0" as *const u8 as *const libc::c_char
            },
        );
        _cairo_svg_surface_emit_path(
            &mut (*(*surface).document).xml_node_defs,
            path,
            0 as *const cairo_matrix_t,
        );
        _cairo_svg_stream_printf(
            &mut (*(*surface).document).xml_node_defs as *mut cairo_svg_stream_t,
            b"/>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_printf(
            &mut (*(*surface).document).xml_node_defs as *mut cairo_svg_stream_t,
            b"</clipPath>\n\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh57 = (*(*surface).document).clip_id;
        let fresh58 = *fresh57;
        *fresh57 = (*fresh57).wrapping_add(1);
        _cairo_svg_stream_printf(
            output,
            b"<g clip-path=\"url(#clip-%d)\">\n\0" as *const u8 as *const libc::c_char,
            fresh58,
        );
        status = _cairo_svg_surface_emit_composite_pattern(
            output,
            surface,
            source as *mut cairo_surface_pattern_t,
            invalid_pattern_id,
            0 as *const cairo_matrix_t,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_printf(output, b"</g>\0" as *const u8 as *const libc::c_char);
    } else {
        _cairo_svg_stream_printf(output, b"<path\0" as *const u8 as *const libc::c_char);
        status = _cairo_svg_surface_emit_fill_style(
            output,
            surface,
            source,
            fill_rule,
            0 as *const cairo_matrix_t,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_svg_surface_emit_path(output, path, 0 as *const cairo_matrix_t);
        _cairo_svg_stream_printf(output, b"/>\n\0" as *const u8 as *const libc::c_char);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return (if _cairo_svg_surface_are_operation_and_pattern_supported(
            surface,
            op,
            source,
        ) != 0
        {
            CAIRO_STATUS_SUCCESS as libc::c_int
        } else {
            CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        }) as cairo_int_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        status = _cairo_svg_surface_set_clip(surface, &mut (*surface).xml_node, clip);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        return _cairo_svg_surface_fill_impl(
            &mut (*surface).xml_node,
            surface,
            source,
            path,
            fill_rule,
            tolerance,
            antialias,
        );
    } else {
        _cairo_svg_surface_reset_clip(surface);
        let mut mask_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_fill_impl(
            &mut mask_stream,
            surface,
            &_cairo_pattern_white.base,
            path,
            fill_rule,
            tolerance,
            antialias,
        ) as cairo_status_t;
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut source_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_emit_paint(
            &mut source_stream,
            surface,
            source,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut source_stream);
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status as cairo_int_status_t;
        }
        let mut destination_stream: cairo_svg_stream_t = (*surface).xml_node;
        (*surface).xml_node = _cairo_svg_stream_create();
        return _cairo_svg_surface_do_operator(
            &mut (*surface).xml_node,
            surface,
            op,
            clip,
            &mut mask_stream,
            &mut source_stream,
            &mut destination_stream,
        );
    };
}
unsafe extern "C" fn _cairo_svg_surface_fill_stroke(
    mut abstract_surface: *mut libc::c_void,
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
) -> cairo_int_status_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _cairo_svg_surface_svg_clip_or_svg_mask_should_be_used(fill_source) != 0
        || _cairo_svg_surface_svg_clip_or_svg_mask_should_be_used(stroke_source) != 0
        || fill_op as libc::c_uint != CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
        || stroke_op as libc::c_uint
            != CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return (if _cairo_svg_surface_are_operation_and_pattern_supported(
            surface,
            fill_op,
            fill_source,
        ) != 0
            && _cairo_svg_surface_are_operation_and_pattern_supported(
                surface,
                stroke_op,
                stroke_source,
            ) != 0
        {
            CAIRO_STATUS_SUCCESS as libc::c_int
        } else {
            CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        }) as cairo_int_status_t;
    }
    status = _cairo_svg_surface_set_clip(surface, &mut (*surface).xml_node, clip);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_stream_printf(
        &mut (*surface).xml_node as *mut cairo_svg_stream_t,
        b"<path\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_svg_surface_emit_fill_style(
        &mut (*surface).xml_node,
        surface,
        fill_source,
        fill_rule,
        stroke_ctm_inverse,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = _cairo_svg_surface_emit_stroke_style(
        &mut (*surface).xml_node,
        surface,
        stroke_source,
        stroke_style,
        stroke_ctm_inverse,
    );
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_svg_surface_emit_path(&mut (*surface).xml_node, path, stroke_ctm_inverse);
    _cairo_svg_surface_emit_transform(
        &mut (*surface).xml_node,
        b"transform\0" as *const u8 as *const libc::c_char,
        stroke_ctm,
        0 as *const cairo_matrix_t,
    );
    _cairo_svg_stream_printf(
        &mut (*surface).xml_node as *mut cairo_svg_stream_t,
        b"/>\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_show_glyphs_impl(
    mut output: *mut cairo_svg_stream_t,
    mut surface: *mut cairo_svg_surface_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut document: *mut cairo_svg_document_t = (*surface).document;
    if num_glyphs <= 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if !((*source).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint)
    {
        _cairo_svg_stream_printf(output, b"<g\0" as *const u8 as *const libc::c_char);
        status = _cairo_svg_surface_emit_pattern(
            surface,
            source,
            output,
            0 as libc::c_int,
            0 as *const cairo_matrix_t,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_svg_stream_printf(output, b">\n\0" as *const u8 as *const libc::c_char);
        let mut i: libc::c_int = 0 as libc::c_int;
        loop {
            if !(i < num_glyphs) {
                current_block = 4495394744059808450;
                break;
            }
            let mut subset_glyph: cairo_scaled_font_subsets_glyph_t = cairo_scaled_font_subsets_glyph_t {
                font_id: 0,
                subset_id: 0,
                subset_glyph_index: 0,
                is_scaled: 0,
                is_composite: 0,
                is_latin: 0,
                x_advance: 0.,
                y_advance: 0.,
                utf8_is_mapped: 0,
                unicode: 0,
            };
            status = _cairo_scaled_font_subsets_map_glyph(
                (*document).font_subsets,
                scaled_font,
                (*glyphs.offset(i as isize)).index,
                0 as *const libc::c_char,
                0 as libc::c_int,
                &mut subset_glyph,
            );
            if status as cairo_int_status_t as libc::c_uint
                == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
            {
                _cairo_svg_stream_printf(
                    output,
                    b"</g>\n\0" as *const u8 as *const libc::c_char,
                );
                glyphs = glyphs.offset(i as isize);
                num_glyphs -= i;
                current_block = 11194104282611034094;
                break;
            } else {
                if status as u64 != 0 {
                    return status as cairo_int_status_t;
                }
                _cairo_svg_stream_printf(
                    output,
                    b"<use xlink:href=\"#glyph-%d-%d\" x=\"%f\" y=\"%f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    subset_glyph.font_id,
                    subset_glyph.subset_glyph_index,
                    (*glyphs.offset(i as isize)).x,
                    (*glyphs.offset(i as isize)).y,
                );
                i += 1;
            }
        }
        match current_block {
            11194104282611034094 => {}
            _ => {
                _cairo_svg_stream_printf(
                    output,
                    b"</g>\n\0" as *const u8 as *const libc::c_char,
                );
                return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            }
        }
    }
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
                    next: 0 as *const _cairo_list as *mut _cairo_list,
                    prev: 0 as *const _cairo_list as *mut _cairo_list,
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
    _cairo_path_fixed_init(&mut path);
    status = _cairo_scaled_font_glyph_path(scaled_font, glyphs, num_glyphs, &mut path);
    if status as u64 != 0 {
        _cairo_path_fixed_fini(&mut path);
        return status as cairo_int_status_t;
    }
    status = _cairo_svg_surface_fill_impl(
        output,
        surface,
        source,
        &mut path,
        CAIRO_FILL_RULE_WINDING,
        0.0f64,
        CAIRO_ANTIALIAS_DEFAULT,
    ) as cairo_status_t;
    _cairo_path_fixed_fini(&mut path);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_show_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return (if _cairo_svg_surface_are_operation_and_pattern_supported(
            surface,
            op,
            source,
        ) != 0
        {
            CAIRO_STATUS_SUCCESS as libc::c_int
        } else {
            CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        }) as cairo_int_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        status = _cairo_svg_surface_set_clip(surface, &mut (*surface).xml_node, clip)
            as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        return _cairo_svg_surface_show_glyphs_impl(
            &mut (*surface).xml_node,
            surface,
            source,
            glyphs,
            num_glyphs,
            scaled_font,
        );
    } else {
        _cairo_svg_surface_reset_clip(surface);
        let mut mask_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_show_glyphs_impl(
            &mut mask_stream,
            surface,
            &_cairo_pattern_white.base,
            glyphs,
            num_glyphs,
            scaled_font,
        );
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status;
        }
        let mut source_stream: cairo_svg_stream_t = _cairo_svg_stream_create();
        status = _cairo_svg_surface_emit_paint(
            &mut source_stream,
            surface,
            source,
            0 as libc::c_int,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            _cairo_svg_stream_destroy(&mut source_stream);
            _cairo_svg_stream_destroy(&mut mask_stream);
            return status;
        }
        let mut destination_stream: cairo_svg_stream_t = (*surface).xml_node;
        (*surface).xml_node = _cairo_svg_stream_create();
        return _cairo_svg_surface_do_operator(
            &mut (*surface).xml_node,
            surface,
            op,
            clip,
            &mut mask_stream,
            &mut source_stream,
            &mut destination_stream,
        );
    };
}
unsafe extern "C" fn _cairo_svg_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    _cairo_font_options_init_default(options);
    cairo_font_options_set_hint_style(options, CAIRO_HINT_STYLE_NONE);
    cairo_font_options_set_hint_metrics(options, CAIRO_HINT_METRICS_OFF);
    cairo_font_options_set_antialias(options, CAIRO_ANTIALIAS_GRAY);
    _cairo_font_options_set_round_glyph_positions(options, CAIRO_ROUND_GLYPH_POS_OFF);
}
unsafe extern "C" fn _cairo_svg_surface_get_supported_mime_types(
    mut abstract_surface: *mut libc::c_void,
) -> *mut *const libc::c_char {
    return _cairo_svg_supported_mime_types.as_mut_ptr();
}
static mut cairo_svg_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_SVG,
            finish: Some(
                _cairo_svg_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: None,
            create_similar_image: None,
            map_to_image: None,
            unmap_image: None,
            source: Some(
                _cairo_surface_default_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: None,
            release_source_image: None,
            snapshot: None,
            copy_page: Some(
                _cairo_svg_surface_copy_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            show_page: Some(
                _cairo_svg_surface_show_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            get_extents: Some(
                _cairo_svg_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_svg_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_svg_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_svg_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_svg_surface_stroke
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
                _cairo_svg_surface_fill
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
            fill_stroke: Some(
                _cairo_svg_surface_fill_stroke
                    as unsafe extern "C" fn(
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
            ),
            show_glyphs: Some(
                _cairo_svg_surface_show_glyphs
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
            get_supported_mime_types: Some(
                _cairo_svg_surface_get_supported_mime_types
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                    ) -> *mut *const libc::c_char,
            ),
            tag: None,
        };
        init
    }
};
unsafe extern "C" fn _cairo_svg_document_create(
    mut output_stream: *mut cairo_output_stream_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
    mut version: cairo_svg_version_t,
    mut document_out: *mut *mut cairo_svg_document_t,
) -> cairo_status_t {
    let mut document: *mut cairo_svg_document_t = 0 as *mut cairo_svg_document_t;
    if (*output_stream).status as u64 != 0 {
        return (*output_stream).status;
    }
    document = (if ::std::mem::size_of::<cairo_svg_document_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_svg_document_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_svg_document_t;
    if document.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh59 = (*document).output_stream;
    *fresh59 = output_stream;
    (*document).refcount = 1 as libc::c_int as libc::c_ulong;
    let ref mut fresh60 = (*document).owner;
    *fresh60 = 0 as *mut cairo_surface_t;
    (*document).finished = 0 as libc::c_int;
    (*document).width = width;
    (*document).height = height;
    (*document).unit = CAIRO_SVG_UNIT_USER;
    (*document).xml_node_defs = _cairo_svg_stream_create();
    (*document).xml_node_glyphs = _cairo_svg_stream_create();
    (*document).xml_node_filters = _cairo_svg_stream_create();
    (*document).linear_pattern_id = 0 as libc::c_int as libc::c_uint;
    (*document).radial_pattern_id = 0 as libc::c_int as libc::c_uint;
    (*document).pattern_id = 0 as libc::c_int as libc::c_uint;
    (*document).clip_id = 0 as libc::c_int as libc::c_uint;
    (*document).mask_id = 0 as libc::c_int as libc::c_uint;
    (*document).compositing_group_id = 0 as libc::c_int as libc::c_uint;
    (*document).filter_id = 0 as libc::c_int as libc::c_uint;
    let mut filter: cairo_svg_filter = CAIRO_SVG_FILTER_REMOVE_COLOR;
    while (filter as libc::c_uint)
        < CAIRO_SVG_FILTER_LAST_STATIC_FILTER as libc::c_int as libc::c_uint
    {
        (*document).filters_emitted[filter as usize] = 0 as libc::c_int;
        filter += 1;
    }
    (*document).svg_version = version;
    let ref mut fresh61 = (*document).font_subsets;
    *fresh61 = _cairo_scaled_font_subsets_create_scaled();
    if ((*document).font_subsets).is_null() {
        _cairo_svg_stream_destroy(&mut (*document).xml_node_defs);
        _cairo_svg_stream_destroy(&mut (*document).xml_node_glyphs);
        _cairo_svg_stream_destroy(&mut (*document).xml_node_filters);
        free(document as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh62 = (*document).paints;
    *fresh62 = _cairo_hash_table_create(
        Some(
            _cairo_svg_paint_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if ((*document).paints).is_null() {
        _cairo_svg_stream_destroy(&mut (*document).xml_node_defs);
        _cairo_svg_stream_destroy(&mut (*document).xml_node_glyphs);
        _cairo_svg_stream_destroy(&mut (*document).xml_node_filters);
        _cairo_scaled_font_subsets_destroy((*document).font_subsets);
        free(document as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    *document_out = document;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_svg_document_reference(
    mut document: *mut cairo_svg_document_t,
) -> *mut cairo_svg_document_t {
    let ref mut fresh63 = (*document).refcount;
    *fresh63 = (*fresh63).wrapping_add(1);
    return document;
}
unsafe extern "C" fn _cairo_svg_document_destroy(
    mut document: *mut cairo_svg_document_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let ref mut fresh64 = (*document).refcount;
    *fresh64 = (*fresh64).wrapping_sub(1);
    if (*document).refcount > 0 as libc::c_int as libc::c_ulong {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_svg_document_finish(document);
    free(document as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn _cairo_svg_document_finish(
    mut document: *mut cairo_svg_document_t,
) -> cairo_status_t {
    if (*document).finished != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    (*document).finished = 1 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut final_status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut output: *mut cairo_output_stream_t = (*document).output_stream;
    _cairo_output_stream_printf(
        output,
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"%f%s\" height=\"%f%s\" viewBox=\"0 0 %f %f\">\n\0"
            as *const u8 as *const libc::c_char,
        (*document).width,
        _cairo_svg_unit_strings[(*document).unit as usize],
        (*document).height,
        _cairo_svg_unit_strings[(*document).unit as usize],
        (*document).width,
        (*document).height,
    );
    status = _cairo_svg_document_emit_font_subsets(document);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    let mut surface: *mut cairo_svg_surface_t = 0 as *mut cairo_svg_surface_t;
    if !((*document).owner).is_null() {
        surface = _cairo_paginated_surface_get_target((*document).owner)
            as *mut cairo_svg_surface_t;
        if (*surface).xml_node.elements.num_elements > 0 as libc::c_int as libc::c_uint {
            let mut page: *mut cairo_svg_page_t = _cairo_svg_surface_store_page(surface);
            if final_status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint && page.is_null()
            {
                final_status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
        if (*surface).transitive_paint_used != 0 {
            let mut paint_entry: *mut cairo_svg_paint_t = malloc(
                ::std::mem::size_of::<cairo_svg_paint_t>() as libc::c_ulong,
            ) as *mut cairo_svg_paint_t;
            if paint_entry.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            (*paint_entry).source_id = (*surface).source_id;
            (*paint_entry).box_0.p1.x = 0 as libc::c_int as libc::c_double;
            (*paint_entry).box_0.p1.y = 0 as libc::c_int as libc::c_double;
            (*paint_entry).box_0.p2.x = (*document).width;
            (*paint_entry).box_0.p2.y = (*document).height;
            _cairo_svg_paint_box_add_padding(&mut (*paint_entry).box_0);
            _cairo_array_init(
                &mut (*paint_entry).paint_elements,
                ::std::mem::size_of::<cairo_svg_paint_element_t>() as libc::c_ulong
                    as libc::c_uint,
            );
            _cairo_svg_paint_init_key(paint_entry);
            status = _cairo_hash_table_insert(
                (*document).paints,
                &mut (*paint_entry).base,
            );
            if status as u64 != 0 {
                return status;
            }
        }
    }
    _cairo_hash_table_foreach(
        (*document).paints,
        Some(
            _cairo_svg_paint_compute_func
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        document as *mut libc::c_void,
    );
    if (*document).xml_node_filters.elements.num_elements
        > 0 as libc::c_int as libc::c_uint
        || (*document).xml_node_glyphs.elements.num_elements
            > 0 as libc::c_int as libc::c_uint
        || (*document).xml_node_defs.elements.num_elements
            > 0 as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            output,
            b"<defs>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_svg_stream_copy_to_output_stream(
            &mut (*document).xml_node_filters,
            output,
            (*document).paints,
        );
        if (*document).xml_node_glyphs.elements.num_elements
            > 0 as libc::c_int as libc::c_uint
        {
            _cairo_output_stream_printf(
                output,
                b"<g>\n\0" as *const u8 as *const libc::c_char,
            );
            _cairo_svg_stream_copy_to_output_stream(
                &mut (*document).xml_node_glyphs,
                output,
                (*document).paints,
            );
            _cairo_output_stream_printf(
                output,
                b"</g>\n\0" as *const u8 as *const libc::c_char,
            );
        }
        _cairo_svg_stream_copy_to_output_stream(
            &mut (*document).xml_node_defs,
            output,
            (*document).paints,
        );
        _cairo_output_stream_printf(
            output,
            b"</defs>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*document).owner).is_null() {
        if (*surface).page_set.num_elements == 1 as libc::c_int as libc::c_uint {
            let mut page_0: *mut cairo_svg_page_t = _cairo_array_index(
                &mut (*surface).page_set,
                0 as libc::c_int as libc::c_uint,
            ) as *mut cairo_svg_page_t;
            _cairo_svg_stream_copy_to_output_stream(
                &mut (*page_0).xml_node,
                output,
                (*document).paints,
            );
        } else if (*surface).page_set.num_elements > 1 as libc::c_int as libc::c_uint {
            _cairo_output_stream_printf(
                output,
                b"<pageSet>\n\0" as *const u8 as *const libc::c_char,
            );
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i < (*surface).page_set.num_elements {
                let mut page_1: *mut cairo_svg_page_t = _cairo_array_index(
                    &mut (*surface).page_set,
                    i,
                ) as *mut cairo_svg_page_t;
                _cairo_output_stream_printf(
                    output,
                    b"<page>\n\0" as *const u8 as *const libc::c_char,
                );
                _cairo_svg_stream_copy_to_output_stream(
                    &mut (*page_1).xml_node,
                    output,
                    (*document).paints,
                );
                _cairo_output_stream_printf(
                    output,
                    b"</page>\n\0" as *const u8 as *const libc::c_char,
                );
                i = i.wrapping_add(1);
            }
            _cairo_output_stream_printf(
                output,
                b"</pageSet>\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    _cairo_output_stream_printf(
        output,
        b"</svg>\n\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_svg_stream_destroy(&mut (*document).xml_node_defs);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    status = _cairo_svg_stream_destroy(&mut (*document).xml_node_glyphs);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    status = _cairo_svg_stream_destroy(&mut (*document).xml_node_filters);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    _cairo_hash_table_foreach(
        (*document).paints,
        Some(
            _cairo_svg_paint_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*document).paints as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*document).paints);
    status = _cairo_output_stream_destroy(output);
    if final_status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        final_status = status;
    }
    return final_status;
}
unsafe extern "C" fn _cairo_svg_surface_set_paginated_mode(
    mut abstract_surface: *mut libc::c_void,
    mut paginated_mode: cairo_paginated_mode_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_svg_surface_t = abstract_surface
        as *mut cairo_svg_surface_t;
    (*surface).paginated_mode = paginated_mode;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_svg_surface_supports_fine_grained_fallbacks(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
static mut cairo_svg_surface_paginated_backend: cairo_paginated_surface_backend_t = unsafe {
    {
        let mut init = _cairo_paginated_surface_backend {
            start_page: None,
            set_paginated_mode: Some(
                _cairo_svg_surface_set_paginated_mode
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_paginated_mode_t,
                    ) -> cairo_int_status_t,
            ),
            set_bounding_box: None,
            set_fallback_images_required: None,
            supports_fine_grained_fallbacks: Some(
                _cairo_svg_surface_supports_fine_grained_fallbacks
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            requires_thumbnail_image: None,
            set_thumbnail_image: None,
        };
        init
    }
};
