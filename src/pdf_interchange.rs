use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type _cairo_image_surface;
    pub type _cairo_hash_table;
    pub type _cairo_pattern;
    pub type _cairo_scaled_font_subsets;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn cairo_version_string() -> *const libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_rectangle_int_from_double(
        recti: *mut cairo_rectangle_int_t,
        rectf: *const cairo_rectangle_t,
    );
    fn _cairo_rectangle_union(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    );
    fn _cairo_hash_bytes(
        hash: uintptr_t,
        bytes: *const libc::c_void,
        length: libc::c_uint,
    ) -> uintptr_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn _cairo_tag_get_type(name: *const libc::c_char) -> cairo_tag_type_t;
    fn _cairo_pdf_operators_tag_end(
        pdf_operators: *mut cairo_pdf_operators_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_array_copy_element(
        array: *const cairo_array_t,
        index: libc::c_uint,
        dst: *mut libc::c_void,
    );
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn _cairo_tag_stack_free_elem(elem: *mut cairo_tag_stack_elem_t);
    fn _cairo_tag_parse_link_attributes(
        attributes: *const libc::c_char,
        link_attrs: *mut cairo_link_attrs_t,
    ) -> cairo_int_status_t;
    fn _cairo_tag_parse_dest_attributes(
        attributes: *const libc::c_char,
        dest_attrs: *mut cairo_dest_attrs_t,
    ) -> cairo_int_status_t;
    fn _cairo_tag_stack_init(stack: *mut cairo_tag_stack_t);
    fn _cairo_tag_stack_fini(stack: *mut cairo_tag_stack_t);
    fn _cairo_tag_stack_get_structure_type(
        stack: *mut cairo_tag_stack_t,
    ) -> cairo_tag_stack_structure_type_t;
    fn _cairo_tag_stack_push(
        stack: *mut cairo_tag_stack_t,
        name: *const libc::c_char,
        attributes: *const libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_tag_stack_set_top_data(
        stack: *mut cairo_tag_stack_t,
        data: *mut libc::c_void,
    );
    fn _cairo_tag_stack_pop(
        stack: *mut cairo_tag_stack_t,
        name: *const libc::c_char,
        elem: *mut *mut cairo_tag_stack_elem_t,
    ) -> cairo_int_status_t;
    fn _cairo_tag_error(fmt: *const libc::c_char, _: ...) -> cairo_status_t;
    fn _cairo_pdf_operators_tag_begin(
        pdf_operators: *mut cairo_pdf_operators_t,
        tag_name: *const libc::c_char,
        mcid: libc::c_int,
    ) -> cairo_int_status_t;
    fn _cairo_tag_stack_top_elem(
        stack: *mut cairo_tag_stack_t,
    ) -> *mut cairo_tag_stack_elem_t;
    fn _cairo_utf8_to_pdf_string(
        utf8: *const libc::c_char,
        str_out: *mut *mut libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_surface_new_object(
        surface: *mut cairo_pdf_surface_t,
    ) -> cairo_pdf_resource_t;
    fn _cairo_pdf_surface_update_object(
        surface: *mut cairo_pdf_surface_t,
        resource: cairo_pdf_resource_t,
    );
    fn _cairo_pdf_surface_object_begin(
        surface: *mut cairo_pdf_surface_t,
        resource: cairo_pdf_resource_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_surface_object_end(surface: *mut cairo_pdf_surface_t);
    fn _cairo_output_stream_printf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type cairo_scaled_font_subsets_t = _cairo_scaled_font_subsets;
pub type _cairo_paginated_mode = libc::c_uint;
pub const CAIRO_PAGINATED_MODE_FALLBACK: _cairo_paginated_mode = 2;
pub const CAIRO_PAGINATED_MODE_RENDER: _cairo_paginated_mode = 1;
pub const CAIRO_PAGINATED_MODE_ANALYZE: _cairo_paginated_mode = 0;
pub type cairo_paginated_mode_t = _cairo_paginated_mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_box_double {
    pub p1: cairo_point_double_t,
    pub p2: cairo_point_double_t,
}
pub type cairo_box_double_t = _cairo_box_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type _cairo_pdf_version = libc::c_uint;
pub const CAIRO_PDF_VERSION_1_7: _cairo_pdf_version = 3;
pub const CAIRO_PDF_VERSION_1_6: _cairo_pdf_version = 2;
pub const CAIRO_PDF_VERSION_1_5: _cairo_pdf_version = 1;
pub const CAIRO_PDF_VERSION_1_4: _cairo_pdf_version = 0;
pub type cairo_pdf_version_t = _cairo_pdf_version;
pub type _cairo_pdf_outline_flags = libc::c_uint;
pub const CAIRO_PDF_OUTLINE_FLAG_ITALIC: _cairo_pdf_outline_flags = 4;
pub const CAIRO_PDF_OUTLINE_FLAG_BOLD: _cairo_pdf_outline_flags = 2;
pub const CAIRO_PDF_OUTLINE_FLAG_OPEN: _cairo_pdf_outline_flags = 1;
pub type cairo_pdf_outline_flags_t = _cairo_pdf_outline_flags;
pub type _cairo_pdf_metadata = libc::c_uint;
pub const CAIRO_PDF_METADATA_MOD_DATE: _cairo_pdf_metadata = 6;
pub const CAIRO_PDF_METADATA_CREATE_DATE: _cairo_pdf_metadata = 5;
pub const CAIRO_PDF_METADATA_CREATOR: _cairo_pdf_metadata = 4;
pub const CAIRO_PDF_METADATA_KEYWORDS: _cairo_pdf_metadata = 3;
pub const CAIRO_PDF_METADATA_SUBJECT: _cairo_pdf_metadata = 2;
pub const CAIRO_PDF_METADATA_AUTHOR: _cairo_pdf_metadata = 1;
pub const CAIRO_PDF_METADATA_TITLE: _cairo_pdf_metadata = 0;
pub type cairo_pdf_metadata_t = _cairo_pdf_metadata;
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
pub type cairo_surface_clipper_t = _cairo_surface_clipper;
pub type cairo_pdf_operators_use_font_subset_t = Option::<
    unsafe extern "C" fn(
        libc::c_uint,
        libc::c_uint,
        *mut libc::c_void,
    ) -> cairo_int_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_glyph {
    pub glyph_index: libc::c_uint,
    pub x_position: libc::c_double,
    pub x_advance: libc::c_double,
}
pub type cairo_pdf_glyph_t = _cairo_pdf_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_operators {
    pub stream: *mut cairo_output_stream_t,
    pub cairo_to_pdf: cairo_matrix_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub use_font_subset: cairo_pdf_operators_use_font_subset_t,
    pub use_font_subset_closure: *mut libc::c_void,
    pub ps_output: cairo_bool_t,
    pub use_actual_text: cairo_bool_t,
    pub in_text_object: cairo_bool_t,
    pub is_new_text_object: cairo_bool_t,
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub text_matrix: cairo_matrix_t,
    pub cairo_to_pdftext: cairo_matrix_t,
    pub font_matrix_inverse: cairo_matrix_t,
    pub cur_x: libc::c_double,
    pub cur_y: libc::c_double,
    pub hex_width: libc::c_int,
    pub is_latin: cairo_bool_t,
    pub num_glyphs: libc::c_int,
    pub glyph_buf_x_pos: libc::c_double,
    pub glyphs: [cairo_pdf_glyph_t; 200],
    pub has_line_style: cairo_bool_t,
    pub line_width: libc::c_double,
    pub line_cap: cairo_line_cap_t,
    pub line_join: cairo_line_join_t,
    pub miter_limit: libc::c_double,
    pub has_dashes: cairo_bool_t,
}
pub type cairo_pdf_operators_t = _cairo_pdf_operators;
pub type cairo_tag_link_type_t = libc::c_uint;
pub const TAG_LINK_FILE: cairo_tag_link_type_t = 4;
pub const TAG_LINK_URI: cairo_tag_link_type_t = 3;
pub const TAG_LINK_DEST: cairo_tag_link_type_t = 2;
pub const TAG_LINK_EMPTY: cairo_tag_link_type_t = 1;
pub const TAG_LINK_INVALID: cairo_tag_link_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_link_attrs {
    pub link_type: cairo_tag_link_type_t,
    pub rects: cairo_array_t,
    pub dest: *mut libc::c_char,
    pub uri: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub page: libc::c_int,
    pub has_pos: cairo_bool_t,
    pub pos: cairo_point_double_t,
}
pub type cairo_link_attrs_t = _cairo_link_attrs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_dest_attrs {
    pub name: *mut libc::c_char,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub x_valid: cairo_bool_t,
    pub y_valid: cairo_bool_t,
    pub internal: cairo_bool_t,
}
pub type cairo_dest_attrs_t = _cairo_dest_attrs;
pub type cairo_tag_type_t = libc::c_uint;
pub const TAG_TYPE_DEST: cairo_tag_type_t = 4;
pub const TAG_TYPE_LINK: cairo_tag_type_t = 2;
pub const TAG_TYPE_STRUCTURE: cairo_tag_type_t = 1;
pub const TAG_TYPE_INVALID: cairo_tag_type_t = 0;
pub type _cairo_tag_stack_structure_type = libc::c_uint;
pub const TAG_TREE_TYPE_INVALID: _cairo_tag_stack_structure_type = 4;
pub const TAG_TREE_TYPE_NO_TAGS: _cairo_tag_stack_structure_type = 3;
pub const TAG_TREE_TYPE_LINK_ONLY: _cairo_tag_stack_structure_type = 2;
pub const TAG_TREE_TYPE_STRUCTURE: _cairo_tag_stack_structure_type = 1;
pub const TAG_TREE_TYPE_TAGGED: _cairo_tag_stack_structure_type = 0;
pub type cairo_tag_stack_structure_type_t = _cairo_tag_stack_structure_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tag_stack_elem {
    pub name: *mut libc::c_char,
    pub attributes: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub link: cairo_list_t,
}
pub type cairo_tag_stack_elem_t = _cairo_tag_stack_elem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tag_stack {
    pub list: cairo_list_t,
    pub type_0: cairo_tag_stack_structure_type_t,
    pub size: libc::c_int,
}
pub type cairo_tag_stack_t = _cairo_tag_stack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_resource {
    pub id: libc::c_uint,
}
pub type cairo_pdf_resource_t = _cairo_pdf_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_group_resources {
    pub operators: [cairo_bool_t; 29],
    pub alphas: cairo_array_t,
    pub smasks: cairo_array_t,
    pub patterns: cairo_array_t,
    pub shadings: cairo_array_t,
    pub xobjects: cairo_array_t,
    pub fonts: cairo_array_t,
}
pub type cairo_pdf_group_resources_t = _cairo_pdf_group_resources;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_mcid {
    pub page: libc::c_int,
    pub mcid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag_extents {
    pub extents: cairo_rectangle_int_t,
    pub valid: cairo_bool_t,
    pub link: cairo_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_struct_tree_node {
    pub name: *mut libc::c_char,
    pub res: cairo_pdf_resource_t,
    pub parent: *mut _cairo_pdf_struct_tree_node,
    pub children: cairo_list_t,
    pub mcid: cairo_array_t,
    pub annot_res: cairo_pdf_resource_t,
    pub extents: tag_extents,
    pub link: cairo_list_t,
}
pub type cairo_pdf_struct_tree_node_t = _cairo_pdf_struct_tree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_annotation {
    pub node: *mut cairo_pdf_struct_tree_node_t,
    pub link_attrs: cairo_link_attrs_t,
}
pub type cairo_pdf_annotation_t = _cairo_pdf_annotation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_named_dest {
    pub base: cairo_hash_entry_t,
    pub extents: tag_extents,
    pub attrs: cairo_dest_attrs_t,
    pub page: libc::c_int,
}
pub type cairo_pdf_named_dest_t = _cairo_pdf_named_dest;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_outline_entry {
    pub name: *mut libc::c_char,
    pub link_attrs: cairo_link_attrs_t,
    pub flags: cairo_pdf_outline_flags_t,
    pub res: cairo_pdf_resource_t,
    pub parent: *mut _cairo_pdf_outline_entry,
    pub first_child: *mut _cairo_pdf_outline_entry,
    pub last_child: *mut _cairo_pdf_outline_entry,
    pub next: *mut _cairo_pdf_outline_entry,
    pub prev: *mut _cairo_pdf_outline_entry,
    pub count: libc::c_int,
}
pub type cairo_pdf_outline_entry_t = _cairo_pdf_outline_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_forward_link {
    pub res: cairo_pdf_resource_t,
    pub dest: *mut libc::c_char,
    pub page: libc::c_int,
    pub has_pos: cairo_bool_t,
    pub pos: cairo_point_double_t,
}
pub type cairo_pdf_forward_link_t = _cairo_pdf_forward_link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct docinfo {
    pub title: *mut libc::c_char,
    pub author: *mut libc::c_char,
    pub subject: *mut libc::c_char,
    pub keywords: *mut libc::c_char,
    pub creator: *mut libc::c_char,
    pub create_date: *mut libc::c_char,
    pub mod_date: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metadata {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_interchange {
    pub analysis_tag_stack: cairo_tag_stack_t,
    pub render_tag_stack: cairo_tag_stack_t,
    pub push_data: cairo_array_t,
    pub push_data_index: libc::c_int,
    pub struct_root: *mut cairo_pdf_struct_tree_node_t,
    pub current_node: *mut cairo_pdf_struct_tree_node_t,
    pub begin_page_node: *mut cairo_pdf_struct_tree_node_t,
    pub end_page_node: *mut cairo_pdf_struct_tree_node_t,
    pub parent_tree: cairo_array_t,
    pub mcid_to_tree: cairo_array_t,
    pub annots: cairo_array_t,
    pub parent_tree_res: cairo_pdf_resource_t,
    pub extents_list: cairo_list_t,
    pub named_dests: *mut cairo_hash_table_t,
    pub num_dests: libc::c_int,
    pub sorted_dests: *mut *mut cairo_pdf_named_dest_t,
    pub dests_res: cairo_pdf_resource_t,
    pub annot_page: libc::c_int,
    pub outline: cairo_array_t,
    pub docinfo: docinfo,
    pub custom_metadata: cairo_array_t,
}
pub type cairo_pdf_interchange_t = _cairo_pdf_interchange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_surface {
    pub base: cairo_surface_t,
    pub output: *mut cairo_output_stream_t,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub surface_extents: cairo_rectangle_int_t,
    pub surface_bounded: cairo_bool_t,
    pub cairo_to_pdf: cairo_matrix_t,
    pub in_xobject: cairo_bool_t,
    pub objects: cairo_array_t,
    pub pages: cairo_array_t,
    pub rgb_linear_functions: cairo_array_t,
    pub alpha_linear_functions: cairo_array_t,
    pub page_patterns: cairo_array_t,
    pub page_surfaces: cairo_array_t,
    pub doc_surfaces: cairo_array_t,
    pub all_surfaces: *mut cairo_hash_table_t,
    pub smask_groups: cairo_array_t,
    pub knockout_group: cairo_array_t,
    pub jbig2_global: cairo_array_t,
    pub page_heights: cairo_array_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub fonts: cairo_array_t,
    pub next_available_resource: cairo_pdf_resource_t,
    pub pages_resource: cairo_pdf_resource_t,
    pub struct_tree_root: cairo_pdf_resource_t,
    pub pdf_version: cairo_pdf_version_t,
    pub compress_streams: cairo_bool_t,
    pub content: cairo_pdf_resource_t,
    pub content_resources: cairo_pdf_resource_t,
    pub resources: cairo_pdf_group_resources_t,
    pub has_fallback_images: cairo_bool_t,
    pub header_emitted: cairo_bool_t,
    pub pdf_stream: C2RustUnnamed_1,
    pub group_stream: C2RustUnnamed_0,
    pub object_stream: C2RustUnnamed,
    pub clipper: cairo_surface_clipper_t,
    pub pdf_operators: cairo_pdf_operators_t,
    pub paginated_mode: cairo_paginated_mode_t,
    pub select_pattern_gstate_saved: cairo_bool_t,
    pub force_fallbacks: cairo_bool_t,
    pub current_operator: cairo_operator_t,
    pub current_pattern_is_solid_color: cairo_bool_t,
    pub current_color_is_stroke: cairo_bool_t,
    pub current_color_red: libc::c_double,
    pub current_color_green: libc::c_double,
    pub current_color_blue: libc::c_double,
    pub current_color_alpha: libc::c_double,
    pub interchange: cairo_pdf_interchange_t,
    pub page_parent_tree: libc::c_int,
    pub page_annots: cairo_array_t,
    pub forward_links: cairo_array_t,
    pub tagged: cairo_bool_t,
    pub current_page_label: *mut libc::c_char,
    pub page_labels: cairo_array_t,
    pub outlines_dict_res: cairo_pdf_resource_t,
    pub names_dict_res: cairo_pdf_resource_t,
    pub docinfo_res: cairo_pdf_resource_t,
    pub page_labels_res: cairo_pdf_resource_t,
    pub thumbnail_width: libc::c_int,
    pub thumbnail_height: libc::c_int,
    pub thumbnail_image: *mut cairo_image_surface_t,
    pub paginated_surface: *mut cairo_surface_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub active: cairo_bool_t,
    pub stream: *mut cairo_output_stream_t,
    pub resource: cairo_pdf_resource_t,
    pub objects: cairo_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub active: cairo_bool_t,
    pub stream: *mut cairo_output_stream_t,
    pub mem_stream: *mut cairo_output_stream_t,
    pub old_output: *mut cairo_output_stream_t,
    pub resource: cairo_pdf_resource_t,
    pub bbox: cairo_box_double_t,
    pub is_knockout: cairo_bool_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub active: cairo_bool_t,
    pub self_0: cairo_pdf_resource_t,
    pub length: cairo_pdf_resource_t,
    pub start_offset: libc::c_longlong,
    pub compressed: cairo_bool_t,
    pub old_output: *mut cairo_output_stream_t,
}
pub type cairo_pdf_surface_t = _cairo_pdf_surface;
#[inline]
unsafe extern "C" fn _cairo_isdigit(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh0 = (*entry).next;
    *fresh0 = entry;
    let ref mut fresh1 = (*entry).prev;
    *fresh1 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh2 = (*next).prev;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).next;
    *fresh3 = next;
    let ref mut fresh4 = (*entry).prev;
    *fresh4 = prev;
    let ref mut fresh5 = (*prev).next;
    *fresh5 = entry;
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
    let ref mut fresh6 = (*next).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = next;
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
unsafe extern "C" fn cairo_list_is_singular(
    mut head: *const cairo_list_t,
) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list || (*head).next == (*head).prev)
        as libc::c_int;
}
unsafe extern "C" fn write_rect_to_pdf_quad_points(
    mut stream: *mut cairo_output_stream_t,
    mut rect: *const cairo_rectangle_t,
    mut surface_height: libc::c_double,
) {
    _cairo_output_stream_printf(
        stream,
        b"%f %f %f %f %f %f %f %f\0" as *const u8 as *const libc::c_char,
        (*rect).x,
        surface_height - (*rect).y,
        (*rect).x + (*rect).width,
        surface_height - (*rect).y,
        (*rect).x + (*rect).width,
        surface_height - ((*rect).y + (*rect).height),
        (*rect).x,
        surface_height - ((*rect).y + (*rect).height),
    );
}
unsafe extern "C" fn write_rect_int_to_pdf_bbox(
    mut stream: *mut cairo_output_stream_t,
    mut rect: *const cairo_rectangle_int_t,
    mut surface_height: libc::c_double,
) {
    _cairo_output_stream_printf(
        stream,
        b"%d %f %d %f\0" as *const u8 as *const libc::c_char,
        (*rect).x,
        surface_height - ((*rect).y + (*rect).height) as libc::c_double,
        (*rect).x + (*rect).width,
        surface_height - (*rect).y as libc::c_double,
    );
}
unsafe extern "C" fn add_tree_node(
    mut surface: *mut cairo_pdf_surface_t,
    mut parent: *mut cairo_pdf_struct_tree_node_t,
    mut name: *const libc::c_char,
    mut new_node: *mut *mut cairo_pdf_struct_tree_node_t,
) -> cairo_int_status_t {
    let mut node: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    node = (if ::std::mem::size_of::<cairo_pdf_struct_tree_node_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_pdf_struct_tree_node_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_pdf_struct_tree_node_t;
    if node.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh8 = (*node).name;
    *fresh8 = strdup(name);
    (*node).res = _cairo_pdf_surface_new_object(surface);
    if (*node).res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh9 = (*node).parent;
    *fresh9 = parent;
    cairo_list_init(&mut (*node).children);
    _cairo_array_init(
        &mut (*node).mcid,
        ::std::mem::size_of::<page_mcid>() as libc::c_ulong as libc::c_uint,
    );
    (*node).annot_res.id = 0 as libc::c_int as libc::c_uint;
    (*node).extents.valid = 0 as libc::c_int;
    cairo_list_init(&mut (*node).extents.link);
    cairo_list_add_tail(&mut (*node).link, &mut (*parent).children);
    *new_node = node;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn is_leaf_node(
    mut node: *mut cairo_pdf_struct_tree_node_t,
) -> cairo_bool_t {
    return (!((*node).parent).is_null()
        && cairo_list_is_empty(&mut (*node).children) != 0) as libc::c_int;
}
unsafe extern "C" fn free_node(mut node: *mut cairo_pdf_struct_tree_node_t) {
    let mut child: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    let mut next: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    if node.is_null() {
        return;
    }
    child = ({
        let mut mptr__: *const cairo_list_t = (*node).children.next;
        (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
            as *mut cairo_pdf_struct_tree_node_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*child).link.next;
        (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
            as *mut cairo_pdf_struct_tree_node_t
    });
    while &mut (*child).link as *mut cairo_list_t
        != &mut (*node).children as *mut cairo_list_t
    {
        cairo_list_del(&mut (*child).link);
        free_node(child);
        child = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                as *mut cairo_pdf_struct_tree_node_t
        });
    }
    free((*node).name as *mut libc::c_void);
    _cairo_array_fini(&mut (*node).mcid);
    free(node as *mut libc::c_void);
}
unsafe extern "C" fn add_mcid_to_node(
    mut surface: *mut cairo_pdf_surface_t,
    mut node: *mut cairo_pdf_struct_tree_node_t,
    mut page: libc::c_int,
    mut mcid: *mut libc::c_int,
) -> cairo_status_t {
    let mut mcid_elem: page_mcid = page_mcid { page: 0, mcid: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    status = _cairo_array_append(
        &mut (*ic).mcid_to_tree,
        &mut node as *mut *mut cairo_pdf_struct_tree_node_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    mcid_elem.page = page;
    mcid_elem
        .mcid = (_cairo_array_num_elements(&mut (*ic).mcid_to_tree))
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    *mcid = mcid_elem.mcid;
    return _cairo_array_append(
        &mut (*node).mcid,
        &mut mcid_elem as *mut page_mcid as *const libc::c_void,
    );
}
unsafe extern "C" fn add_annotation(
    mut surface: *mut cairo_pdf_surface_t,
    mut node: *mut cairo_pdf_struct_tree_node_t,
    mut name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut annot: *mut cairo_pdf_annotation_t = 0 as *mut cairo_pdf_annotation_t;
    annot = malloc(::std::mem::size_of::<cairo_pdf_annotation_t>() as libc::c_ulong)
        as *mut cairo_pdf_annotation_t;
    if annot.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_tag_parse_link_attributes(attributes, &mut (*annot).link_attrs);
    if status as u64 != 0 {
        free(annot as *mut libc::c_void);
        return status;
    }
    let ref mut fresh10 = (*annot).node;
    *fresh10 = node;
    status = _cairo_array_append(
        &mut (*ic).annots,
        &mut annot as *mut *mut cairo_pdf_annotation_t as *const libc::c_void,
    ) as cairo_int_status_t;
    return status;
}
unsafe extern "C" fn free_annotation(mut annot: *mut cairo_pdf_annotation_t) {
    _cairo_array_fini(&mut (*annot).link_attrs.rects);
    free((*annot).link_attrs.dest as *mut libc::c_void);
    free((*annot).link_attrs.uri as *mut libc::c_void);
    free((*annot).link_attrs.file as *mut libc::c_void);
    free(annot as *mut libc::c_void);
}
unsafe extern "C" fn cairo_pdf_interchange_clear_annotations(
    mut surface: *mut cairo_pdf_surface_t,
) {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_elems = _cairo_array_num_elements(&mut (*ic).annots) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        let mut annot: *mut cairo_pdf_annotation_t = 0 as *mut cairo_pdf_annotation_t;
        _cairo_array_copy_element(
            &mut (*ic).annots,
            i as libc::c_uint,
            &mut annot as *mut *mut cairo_pdf_annotation_t as *mut libc::c_void,
        );
        free_annotation(annot);
        i += 1;
    }
    _cairo_array_truncate(&mut (*ic).annots, 0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn cairo_pdf_interchange_write_node_object(
    mut surface: *mut cairo_pdf_surface_t,
    mut node: *mut cairo_pdf_struct_tree_node_t,
) -> cairo_int_status_t {
    let mut mcid_elem: *mut page_mcid = 0 as *mut page_mcid;
    let mut i: libc::c_int = 0;
    let mut num_mcid: libc::c_int = 0;
    let mut first_page: libc::c_int = 0;
    let mut page_res: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut child: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_surface_object_begin(surface, (*node).res);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Type /StructElem\n   /S /%s\n   /P %d 0 R\n\0" as *const u8
            as *const libc::c_char,
        (*node).name,
        (*(*node).parent).res.id,
    );
    if cairo_list_is_empty(&mut (*node).children) == 0 {
        if cairo_list_is_singular(&mut (*node).children) != 0
            && (*node).annot_res.id == 0 as libc::c_int as libc::c_uint
        {
            child = ({
                let mut mptr__: *const cairo_list_t = (*node).children.next;
                (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                    as *mut cairo_pdf_struct_tree_node_t
            });
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /K %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*child).res.id,
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /K [ \0" as *const u8 as *const libc::c_char,
            );
            if (*node).annot_res.id != 0 as libc::c_int as libc::c_uint {
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"<< /Type /OBJR /Obj %d 0 R >> \0" as *const u8
                        as *const libc::c_char,
                    (*node).annot_res.id,
                );
            }
            child = ({
                let mut mptr__: *const cairo_list_t = (*node).children.next;
                (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                    as *mut cairo_pdf_struct_tree_node_t
            });
            while &mut (*child).link as *mut cairo_list_t
                != &mut (*node).children as *mut cairo_list_t
            {
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"%d 0 R \0" as *const u8 as *const libc::c_char,
                    (*child).res.id,
                );
                child = ({
                    let mut mptr__: *const cairo_list_t = (*child).link.next;
                    (mptr__ as *mut libc::c_char)
                        .offset(-(112 as libc::c_ulong as isize))
                        as *mut cairo_pdf_struct_tree_node_t
                });
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"]\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        num_mcid = _cairo_array_num_elements(&mut (*node).mcid) as libc::c_int;
        if num_mcid > 0 as libc::c_int {
            mcid_elem = _cairo_array_index(
                &mut (*node).mcid,
                0 as libc::c_int as libc::c_uint,
            ) as *mut page_mcid;
            first_page = (*mcid_elem).page;
            page_res = _cairo_array_index(
                &mut (*surface).pages,
                (first_page - 1 as libc::c_int) as libc::c_uint,
            ) as *mut cairo_pdf_resource_t;
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /Pg %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*page_res).id,
            );
            if num_mcid == 1 as libc::c_int
                && (*node).annot_res.id == 0 as libc::c_int as libc::c_uint
            {
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"   /K %d\n\0" as *const u8 as *const libc::c_char,
                    (*mcid_elem).mcid,
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"   /K [ \0" as *const u8 as *const libc::c_char,
                );
                if (*node).annot_res.id != 0 as libc::c_int as libc::c_uint {
                    _cairo_output_stream_printf(
                        (*surface).object_stream.stream,
                        b"<< /Type /OBJR /Obj %d 0 R >> \0" as *const u8
                            as *const libc::c_char,
                        (*node).annot_res.id,
                    );
                }
                i = 0 as libc::c_int;
                while i < num_mcid {
                    mcid_elem = _cairo_array_index(&mut (*node).mcid, i as libc::c_uint)
                        as *mut page_mcid;
                    page_res = _cairo_array_index(
                        &mut (*surface).pages,
                        ((*mcid_elem).page - 1 as libc::c_int) as libc::c_uint,
                    ) as *mut cairo_pdf_resource_t;
                    if (*mcid_elem).page == first_page {
                        _cairo_output_stream_printf(
                            (*surface).object_stream.stream,
                            b"%d \0" as *const u8 as *const libc::c_char,
                            (*mcid_elem).mcid,
                        );
                    } else {
                        _cairo_output_stream_printf(
                            (*surface).object_stream.stream,
                            b"\n       << /Type /MCR /Pg %d 0 R /MCID %d >> \0"
                                as *const u8 as *const libc::c_char,
                            (*page_res).id,
                            (*mcid_elem).mcid,
                        );
                    }
                    i += 1;
                }
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"]\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return _cairo_output_stream_get_status((*surface).object_stream.stream)
        as cairo_int_status_t;
}
unsafe extern "C" fn init_named_dest_key(mut dest: *mut cairo_pdf_named_dest_t) {
    (*dest)
        .base
        .hash = _cairo_hash_bytes(
        5381 as libc::c_int as uintptr_t,
        (*dest).attrs.name as *const libc::c_void,
        strlen((*dest).attrs.name) as libc::c_uint,
    );
}
unsafe extern "C" fn _named_dest_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_pdf_named_dest_t = key_a as *const cairo_pdf_named_dest_t;
    let mut b: *const cairo_pdf_named_dest_t = key_b as *const cairo_pdf_named_dest_t;
    return (strcmp((*a).attrs.name, (*b).attrs.name) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn _named_dest_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut dest: *mut cairo_pdf_named_dest_t = entry as *mut cairo_pdf_named_dest_t;
    let mut table: *mut cairo_hash_table_t = closure as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(table, &mut (*dest).base);
    free((*dest).attrs.name as *mut libc::c_void);
    free(dest as *mut libc::c_void);
}
unsafe extern "C" fn cairo_pdf_interchange_write_explicit_dest(
    mut surface: *mut cairo_pdf_surface_t,
    mut page: libc::c_int,
    mut has_pos: cairo_bool_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> cairo_int_status_t {
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut height: libc::c_double = 0.;
    _cairo_array_copy_element(
        &mut (*surface).page_heights,
        (page - 1 as libc::c_int) as libc::c_uint,
        &mut height as *mut libc::c_double as *mut libc::c_void,
    );
    _cairo_array_copy_element(
        &mut (*surface).pages,
        (page - 1 as libc::c_int) as libc::c_uint,
        &mut res as *mut cairo_pdf_resource_t as *mut libc::c_void,
    );
    if has_pos != 0 {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"[%d 0 R /XYZ %f %f 0]\n\0" as *const u8 as *const libc::c_char,
            res.id,
            x,
            height - y,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"[%d 0 R /XYZ null null 0]\n\0" as *const u8 as *const libc::c_char,
            res.id,
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_dest(
    mut surface: *mut cairo_pdf_surface_t,
    mut link_attrs: *mut cairo_link_attrs_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut link: *mut cairo_pdf_forward_link_t = 0 as *mut cairo_pdf_forward_link_t;
    let mut link_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    if !((*link_attrs).dest).is_null() {
        let mut key: cairo_pdf_named_dest_t = cairo_pdf_named_dest_t {
            base: cairo_hash_entry_t { hash: 0 },
            extents: tag_extents {
                extents: cairo_rectangle_int_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                },
                valid: 0,
                link: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                },
            },
            attrs: cairo_dest_attrs_t {
                name: 0 as *mut libc::c_char,
                x: 0.,
                y: 0.,
                x_valid: 0,
                y_valid: 0,
                internal: 0,
            },
            page: 0,
        };
        let mut named_dest: *mut cairo_pdf_named_dest_t = 0
            as *mut cairo_pdf_named_dest_t;
        key.attrs.name = (*link_attrs).dest;
        init_named_dest_key(&mut key);
        named_dest = _cairo_hash_table_lookup((*ic).named_dests, &mut key.base)
            as *mut cairo_pdf_named_dest_t;
        if !named_dest.is_null() {
            let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
            if (*named_dest).extents.valid != 0 {
                x = (*named_dest).extents.extents.x as libc::c_double;
                y = (*named_dest).extents.extents.y as libc::c_double;
            }
            if (*named_dest).attrs.x_valid != 0 {
                x = (*named_dest).attrs.x;
            }
            if (*named_dest).attrs.y_valid != 0 {
                y = (*named_dest).attrs.y;
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /Dest \0" as *const u8 as *const libc::c_char,
            );
            status = cairo_pdf_interchange_write_explicit_dest(
                surface,
                (*named_dest).page,
                1 as libc::c_int,
                x,
                y,
            );
            return status;
        }
    }
    if ((*link_attrs).dest).is_null() {
        if (*link_attrs).page < 1 as libc::c_int {
            return _cairo_tag_error(
                b"Link attribute: \"page=%d\" page must be >= 1\0" as *const u8
                    as *const libc::c_char,
                (*link_attrs).page,
            ) as cairo_int_status_t;
        }
        if (*link_attrs).page
            <= _cairo_array_num_elements(&mut (*surface).pages) as libc::c_int
        {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /Dest \0" as *const u8 as *const libc::c_char,
            );
            return cairo_pdf_interchange_write_explicit_dest(
                surface,
                (*link_attrs).page,
                (*link_attrs).has_pos,
                (*link_attrs).pos.x,
                (*link_attrs).pos.y,
            );
        }
    }
    link = (if ::std::mem::size_of::<cairo_pdf_forward_link_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_pdf_forward_link_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_pdf_forward_link_t;
    if link.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    link_res = _cairo_pdf_surface_new_object(surface);
    if link_res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"   /Dest %d 0 R\n\0" as *const u8 as *const libc::c_char,
        link_res.id,
    );
    (*link).res = link_res;
    let ref mut fresh11 = (*link).dest;
    *fresh11 = if !((*link_attrs).dest).is_null() {
        strdup((*link_attrs).dest)
    } else {
        0 as *mut libc::c_char
    };
    (*link).page = (*link_attrs).page;
    (*link).has_pos = (*link_attrs).has_pos;
    (*link).pos = (*link_attrs).pos;
    status = _cairo_array_append(
        &mut (*surface).forward_links,
        link as *const libc::c_void,
    ) as cairo_int_status_t;
    return status;
}
unsafe extern "C" fn _cairo_utf8_to_pdf_utf8_hexstring(
    mut utf8: *const libc::c_char,
    mut str_out: *mut *mut libc::c_char,
) -> cairo_int_status_t {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ascii: cairo_bool_t = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    ascii = 1 as libc::c_int;
    p = utf8 as *mut libc::c_uchar;
    len = 0 as libc::c_int;
    while *p != 0 {
        if (*p as libc::c_int) < 32 as libc::c_int
            || *p as libc::c_int > 126 as libc::c_int
        {
            ascii = 0 as libc::c_int;
        }
        if *p as libc::c_int == '(' as i32 || *p as libc::c_int == ')' as i32
            || *p as libc::c_int == '\\' as i32
        {
            len += 2 as libc::c_int;
        } else {
            len += 1;
        }
        p = p.offset(1);
    }
    if ascii != 0 {
        str = (if len + 3 as libc::c_int != 0 as libc::c_int {
            malloc((len + 3 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if str.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        *str.offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
        p = utf8 as *mut libc::c_uchar;
        i = 1 as libc::c_int;
        while *p != 0 {
            if *p as libc::c_int == '(' as i32 || *p as libc::c_int == ')' as i32
                || *p as libc::c_int == '\\' as i32
            {
                let fresh12 = i;
                i = i + 1;
                *str.offset(fresh12 as isize) = '\\' as i32 as libc::c_char;
            }
            let fresh13 = i;
            i = i + 1;
            *str.offset(fresh13 as isize) = *p as libc::c_char;
            p = p.offset(1);
        }
        let fresh14 = i;
        i = i + 1;
        *str.offset(fresh14 as isize) = ')' as i32 as libc::c_char;
        let fresh15 = i;
        i = i + 1;
        *str.offset(fresh15 as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        str = (if len * 2 as libc::c_int + 3 as libc::c_int != 0 as libc::c_int {
            malloc((len * 2 as libc::c_int + 3 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if str.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        *str.offset(0 as libc::c_int as isize) = '<' as i32 as libc::c_char;
        p = utf8 as *mut libc::c_uchar;
        i = 1 as libc::c_int;
        while *p != 0 {
            if *p as libc::c_int == '\\' as i32 {
                snprintf(
                    str.offset(i as isize),
                    3 as libc::c_int as libc::c_ulong,
                    b"%02x\0" as *const u8 as *const libc::c_char,
                    '\\' as i32,
                );
                i += 2 as libc::c_int;
            }
            snprintf(
                str.offset(i as isize),
                3 as libc::c_int as libc::c_ulong,
                b"%02x\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            );
            i += 2 as libc::c_int;
            p = p.offset(1);
        }
        let fresh16 = i;
        i = i + 1;
        *str.offset(fresh16 as isize) = '>' as i32 as libc::c_char;
        let fresh17 = i;
        i = i + 1;
        *str.offset(fresh17 as isize) = 0 as libc::c_int as libc::c_char;
    }
    *str_out = str;
    return status;
}
unsafe extern "C" fn cairo_pdf_interchange_write_link_action(
    mut surface: *mut cairo_pdf_surface_t,
    mut link_attrs: *mut cairo_link_attrs_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*link_attrs).link_type as libc::c_uint
        == TAG_LINK_DEST as libc::c_int as libc::c_uint
    {
        status = cairo_pdf_interchange_write_dest(surface, link_attrs);
        if status as u64 != 0 {
            return status;
        }
    } else if (*link_attrs).link_type as libc::c_uint
        == TAG_LINK_URI as libc::c_int as libc::c_uint
    {
        status = _cairo_utf8_to_pdf_string((*link_attrs).uri, &mut dest);
        if status as u64 != 0 {
            return status;
        }
        if *dest.offset(0 as libc::c_int as isize) as libc::c_int != '(' as i32 {
            free(dest as *mut libc::c_void);
            return _cairo_tag_error(
                b"Link attribute: \"url=%s\" URI may only contain ASCII characters\0"
                    as *const u8 as *const libc::c_char,
                (*link_attrs).uri,
            ) as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /A <<\n      /Type /Action\n      /S /URI\n      /URI %s\n   >>\n\0"
                as *const u8 as *const libc::c_char,
            dest,
        );
        free(dest as *mut libc::c_void);
    } else if (*link_attrs).link_type as libc::c_uint
        == TAG_LINK_FILE as libc::c_int as libc::c_uint
    {
        status = _cairo_utf8_to_pdf_utf8_hexstring((*link_attrs).file, &mut dest);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /A <<\n      /Type /Action\n      /S /GoToR\n      /F %s\n\0"
                as *const u8 as *const libc::c_char,
            dest,
        );
        free(dest as *mut libc::c_void);
        if (*surface).pdf_version as libc::c_uint
            >= CAIRO_PDF_VERSION_1_7 as libc::c_int as libc::c_uint
        {
            status = _cairo_utf8_to_pdf_string((*link_attrs).file, &mut dest);
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"      /UF %s\n\0" as *const u8 as *const libc::c_char,
                dest,
            );
            free(dest as *mut libc::c_void);
        }
        if !((*link_attrs).dest).is_null() {
            status = _cairo_utf8_to_pdf_string((*link_attrs).dest, &mut dest);
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"      /D %s\n\0" as *const u8 as *const libc::c_char,
                dest,
            );
            free(dest as *mut libc::c_void);
        } else if (*link_attrs).has_pos != 0 {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"      /D [%d /XYZ %f %f 0]\n\0" as *const u8 as *const libc::c_char,
                (*link_attrs).page,
                (*link_attrs).pos.x,
                (*link_attrs).pos.y,
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"      /D [%d /XYZ null null 0]\n\0" as *const u8
                    as *const libc::c_char,
                (*link_attrs).page,
            );
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   >>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_annot(
    mut surface: *mut cairo_pdf_surface_t,
    mut annot: *mut cairo_pdf_annotation_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut node: *mut cairo_pdf_struct_tree_node_t = (*annot).node;
    let mut sp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut num_rects: libc::c_int = 0;
    let mut height: libc::c_double = 0.;
    num_rects = _cairo_array_num_elements(&mut (*annot).link_attrs.rects) as libc::c_int;
    if strcmp((*node).name, b"Link\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        && (*annot).link_attrs.link_type as libc::c_uint
            != TAG_LINK_EMPTY as libc::c_int as libc::c_uint
        && ((*node).extents.valid != 0 || num_rects > 0 as libc::c_int)
    {
        status = _cairo_array_append(
            &mut (*ic).parent_tree,
            &mut (*node).res as *mut cairo_pdf_resource_t as *const libc::c_void,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        sp = (_cairo_array_num_elements(&mut (*ic).parent_tree))
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        (*node).annot_res = _cairo_pdf_surface_new_object(surface);
        if (*node).annot_res.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_array_append(
            &mut (*surface).page_annots,
            &mut (*node).annot_res as *mut cairo_pdf_resource_t as *const libc::c_void,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_object_begin(surface, (*node).annot_res);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"<< /Type /Annot\n   /Subtype /Link\n   /StructParent %d\n\0" as *const u8
                as *const libc::c_char,
            sp,
        );
        height = (*surface).height;
        if num_rects > 0 as libc::c_int {
            let mut bbox_rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /QuadPoints [ \0" as *const u8 as *const libc::c_char,
            );
            i = 0 as libc::c_int;
            while i < num_rects {
                let mut rectf: cairo_rectangle_t = cairo_rectangle_t {
                    x: 0.,
                    y: 0.,
                    width: 0.,
                    height: 0.,
                };
                let mut recti: cairo_rectangle_int_t = cairo_rectangle_int_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                _cairo_array_copy_element(
                    &mut (*annot).link_attrs.rects,
                    i as libc::c_uint,
                    &mut rectf as *mut cairo_rectangle_t as *mut libc::c_void,
                );
                _cairo_rectangle_int_from_double(&mut recti, &mut rectf);
                if i == 0 as libc::c_int {
                    bbox_rect = recti;
                } else {
                    _cairo_rectangle_union(&mut bbox_rect, &mut recti);
                }
                write_rect_to_pdf_quad_points(
                    (*surface).object_stream.stream,
                    &mut rectf,
                    height,
                );
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                i += 1;
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"]\n   /Rect [ \0" as *const u8 as *const libc::c_char,
            );
            write_rect_int_to_pdf_bbox(
                (*surface).object_stream.stream,
                &mut bbox_rect,
                height,
            );
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b" ]\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /Rect [ \0" as *const u8 as *const libc::c_char,
            );
            write_rect_int_to_pdf_bbox(
                (*surface).object_stream.stream,
                &mut (*node).extents.extents,
                height,
            );
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b" ]\n\0" as *const u8 as *const libc::c_char,
            );
        }
        status = cairo_pdf_interchange_write_link_action(
            surface,
            &mut (*annot).link_attrs,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /BS << /W 0 >>>>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_surface_object_end(surface);
        status = _cairo_output_stream_get_status((*surface).object_stream.stream)
            as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn cairo_pdf_interchange_walk_struct_tree(
    mut surface: *mut cairo_pdf_surface_t,
    mut node: *mut cairo_pdf_struct_tree_node_t,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut cairo_pdf_surface_t,
            *mut cairo_pdf_struct_tree_node_t,
        ) -> cairo_int_status_t,
    >,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut child: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    if !((*node).parent).is_null() {
        status = func.expect("non-null function pointer")(surface, node);
        if status as u64 != 0 {
            return status;
        }
    }
    child = ({
        let mut mptr__: *const cairo_list_t = (*node).children.next;
        (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
            as *mut cairo_pdf_struct_tree_node_t
    });
    while &mut (*child).link as *mut cairo_list_t
        != &mut (*node).children as *mut cairo_list_t
    {
        status = cairo_pdf_interchange_walk_struct_tree(surface, child, func);
        if status as u64 != 0 {
            return status;
        }
        child = ({
            let mut mptr__: *const cairo_list_t = (*child).link.next;
            (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                as *mut cairo_pdf_struct_tree_node_t
        });
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_struct_tree(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut child: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if cairo_list_is_empty(&mut (*(*ic).struct_root).children) != 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    (*surface).struct_tree_root = _cairo_pdf_surface_new_object(surface);
    if (*surface).struct_tree_root.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    (*(*ic).struct_root).res = (*surface).struct_tree_root;
    cairo_pdf_interchange_walk_struct_tree(
        surface,
        (*ic).struct_root,
        Some(
            cairo_pdf_interchange_write_node_object
                as unsafe extern "C" fn(
                    *mut cairo_pdf_surface_t,
                    *mut cairo_pdf_struct_tree_node_t,
                ) -> cairo_int_status_t,
        ),
    );
    child = ({
        let mut mptr__: *const cairo_list_t = (*(*ic).struct_root).children.next;
        (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
            as *mut cairo_pdf_struct_tree_node_t
    });
    status = _cairo_pdf_surface_object_begin(surface, (*surface).struct_tree_root);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Type /StructTreeRoot\n   /ParentTree %d 0 R\n\0" as *const u8
            as *const libc::c_char,
        (*ic).parent_tree_res.id,
    );
    if cairo_list_is_singular(&mut (*(*ic).struct_root).children) != 0 {
        child = ({
            let mut mptr__: *const cairo_list_t = (*(*ic).struct_root).children.next;
            (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                as *mut cairo_pdf_struct_tree_node_t
        });
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /K [ %d 0 R ]\n\0" as *const u8 as *const libc::c_char,
            (*child).res.id,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /K [ \0" as *const u8 as *const libc::c_char,
        );
        child = ({
            let mut mptr__: *const cairo_list_t = (*(*ic).struct_root).children.next;
            (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                as *mut cairo_pdf_struct_tree_node_t
        });
        while &mut (*child).link as *mut cairo_list_t
            != &mut (*(*ic).struct_root).children as *mut cairo_list_t
        {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"%d 0 R \0" as *const u8 as *const libc::c_char,
                (*child).res.id,
            );
            child = ({
                let mut mptr__: *const cairo_list_t = (*child).link.next;
                (mptr__ as *mut libc::c_char).offset(-(112 as libc::c_ulong as isize))
                    as *mut cairo_pdf_struct_tree_node_t
            });
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"]\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_page_annots(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    num_elems = _cairo_array_num_elements(&mut (*ic).annots) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        let mut annot: *mut cairo_pdf_annotation_t = 0 as *mut cairo_pdf_annotation_t;
        _cairo_array_copy_element(
            &mut (*ic).annots,
            i as libc::c_uint,
            &mut annot as *mut *mut cairo_pdf_annotation_t as *mut libc::c_void,
        );
        status = cairo_pdf_interchange_write_annot(surface, annot);
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    return status;
}
unsafe extern "C" fn cairo_pdf_interchange_write_page_parent_elems(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut node: *mut cairo_pdf_struct_tree_node_t = 0
        as *mut cairo_pdf_struct_tree_node_t;
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    (*surface).page_parent_tree = -(1 as libc::c_int);
    num_elems = _cairo_array_num_elements(&mut (*ic).mcid_to_tree) as libc::c_int;
    if num_elems > 0 as libc::c_int {
        res = _cairo_pdf_surface_new_object(surface);
        if res.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_pdf_surface_object_begin(surface, res);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"[\n\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < num_elems {
            _cairo_array_copy_element(
                &mut (*ic).mcid_to_tree,
                i as libc::c_uint,
                &mut node as *mut *mut cairo_pdf_struct_tree_node_t as *mut libc::c_void,
            );
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"  %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*node).res.id,
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"]\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_surface_object_end(surface);
        status = _cairo_array_append(
            &mut (*ic).parent_tree,
            &mut res as *mut cairo_pdf_resource_t as *const libc::c_void,
        ) as cairo_int_status_t;
        (*surface)
            .page_parent_tree = (_cairo_array_num_elements(&mut (*ic).parent_tree))
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    return status;
}
unsafe extern "C" fn cairo_pdf_interchange_write_parent_tree(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut res: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    num_elems = _cairo_array_num_elements(&mut (*ic).parent_tree) as libc::c_int;
    if num_elems > 0 as libc::c_int {
        (*ic).parent_tree_res = _cairo_pdf_surface_new_object(surface);
        if (*ic).parent_tree_res.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_pdf_surface_object_begin(surface, (*ic).parent_tree_res);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"<< /Nums [\n\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < num_elems {
            res = _cairo_array_index(&mut (*ic).parent_tree, i as libc::c_uint)
                as *mut cairo_pdf_resource_t;
            if (*res).id != 0 {
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"   %d %d 0 R\n\0" as *const u8 as *const libc::c_char,
                    i,
                    (*res).id,
                );
            }
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"  ]\n>>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_surface_object_end(surface);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_outline(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut outline: *mut cairo_pdf_outline_entry_t = 0
        as *mut cairo_pdf_outline_entry_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    num_elems = _cairo_array_num_elements(&mut (*ic).outline) as libc::c_int;
    if num_elems < 2 as libc::c_int {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    _cairo_array_copy_element(
        &mut (*ic).outline,
        0 as libc::c_int as libc::c_uint,
        &mut outline as *mut *mut cairo_pdf_outline_entry_t as *mut libc::c_void,
    );
    (*outline).res = _cairo_pdf_surface_new_object(surface);
    if (*outline).res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    (*surface).outlines_dict_res = (*outline).res;
    status = _cairo_pdf_surface_object_begin(surface, (*outline).res);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Type /Outlines\n   /First %d 0 R\n   /Last %d 0 R\n   /Count %d\n>>\n\0"
            as *const u8 as *const libc::c_char,
        (*(*outline).first_child).res.id,
        (*(*outline).last_child).res.id,
        (*outline).count,
    );
    _cairo_pdf_surface_object_end(surface);
    i = 1 as libc::c_int;
    while i < num_elems {
        _cairo_array_copy_element(
            &mut (*ic).outline,
            i as libc::c_uint,
            &mut outline as *mut *mut cairo_pdf_outline_entry_t as *mut libc::c_void,
        );
        _cairo_pdf_surface_update_object(surface, (*outline).res);
        status = _cairo_utf8_to_pdf_string((*outline).name, &mut name);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_object_begin(surface, (*outline).res);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"<< /Title %s\n   /Parent %d 0 R\n\0" as *const u8 as *const libc::c_char,
            name,
            (*(*outline).parent).res.id,
        );
        free(name as *mut libc::c_void);
        if !((*outline).prev).is_null() {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /Prev %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*(*outline).prev).res.id,
            );
        }
        if !((*outline).next).is_null() {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /Next %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*(*outline).next).res.id,
            );
        }
        if !((*outline).first_child).is_null() {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /First %d 0 R\n   /Last %d 0 R\n   /Count %d\n\0" as *const u8
                    as *const libc::c_char,
                (*(*outline).first_child).res.id,
                (*(*outline).last_child).res.id,
                (*outline).count,
            );
        }
        if (*outline).flags as u64 != 0 {
            let mut flags: libc::c_int = 0 as libc::c_int;
            if (*outline).flags as libc::c_uint
                & CAIRO_PDF_OUTLINE_FLAG_ITALIC as libc::c_int as libc::c_uint != 0
            {
                flags |= 1 as libc::c_int;
            }
            if (*outline).flags as libc::c_uint
                & CAIRO_PDF_OUTLINE_FLAG_BOLD as libc::c_int as libc::c_uint != 0
            {
                flags |= 2 as libc::c_int;
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /F %d\n\0" as *const u8 as *const libc::c_char,
                flags,
            );
        }
        status = cairo_pdf_interchange_write_link_action(
            surface,
            &mut (*outline).link_attrs,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b">>\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_surface_object_end(surface);
        i += 1;
    }
    return status;
}
unsafe extern "C" fn split_label(
    mut label: *const libc::c_char,
    mut num: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    *num = 0 as libc::c_int;
    len = strlen(label) as libc::c_int;
    if len == 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    i = len;
    while i > 0 as libc::c_int
        && _cairo_isdigit(*label.offset((i - 1 as libc::c_int) as isize) as libc::c_int)
            != 0
    {
        i -= 1;
    }
    while i < len && *label.offset(i as isize) as libc::c_int == '0' as i32 {
        i += 1;
    }
    if i < len {
        sscanf(
            label.offset(i as isize),
            b"%d\0" as *const u8 as *const libc::c_char,
            num,
        );
    }
    if i > 0 as libc::c_int {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = (if i + 1 as libc::c_int != 0 as libc::c_int {
            malloc((i + 1 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if s.is_null() {
            return 0 as *mut libc::c_char;
        }
        memcpy(s as *mut libc::c_void, label as *const libc::c_void, i as libc::c_ulong);
        *s.offset(i as isize) = 0 as libc::c_int as libc::c_char;
        return s;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn strcmp_null(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> cairo_bool_t {
    if !s1.is_null() && !s2.is_null() {
        return (strcmp(s1, s2) == 0 as libc::c_int) as libc::c_int;
    }
    if s1.is_null() && s2.is_null() {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cairo_pdf_interchange_write_forward_links(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut link: *mut cairo_pdf_forward_link_t = 0 as *mut cairo_pdf_forward_link_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut key: cairo_pdf_named_dest_t = cairo_pdf_named_dest_t {
        base: cairo_hash_entry_t { hash: 0 },
        extents: tag_extents {
            extents: cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            valid: 0,
            link: cairo_list_t {
                next: 0 as *mut _cairo_list,
                prev: 0 as *mut _cairo_list,
            },
        },
        attrs: cairo_dest_attrs_t {
            name: 0 as *mut libc::c_char,
            x: 0.,
            y: 0.,
            x_valid: 0,
            y_valid: 0,
            internal: 0,
        },
        page: 0,
    };
    let mut named_dest: *mut cairo_pdf_named_dest_t = 0 as *mut cairo_pdf_named_dest_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    num_elems = _cairo_array_num_elements(&mut (*surface).forward_links) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        link = _cairo_array_index(&mut (*surface).forward_links, i as libc::c_uint)
            as *mut cairo_pdf_forward_link_t;
        if (*link).page > _cairo_array_num_elements(&mut (*surface).pages) as libc::c_int
        {
            return _cairo_tag_error(
                b"Link attribute: \"page=%d\" page exceeds page count (%d)\0"
                    as *const u8 as *const libc::c_char,
                (*link).page,
                _cairo_array_num_elements(&mut (*surface).pages),
            ) as cairo_int_status_t;
        }
        status = _cairo_pdf_surface_object_begin(surface, (*link).res);
        if status as u64 != 0 {
            return status;
        }
        if !((*link).dest).is_null() {
            key.attrs.name = (*link).dest;
            init_named_dest_key(&mut key);
            named_dest = _cairo_hash_table_lookup((*ic).named_dests, &mut key.base)
                as *mut cairo_pdf_named_dest_t;
            if !named_dest.is_null() {
                let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
                let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
                if (*named_dest).extents.valid != 0 {
                    x = (*named_dest).extents.extents.x as libc::c_double;
                    y = (*named_dest).extents.extents.y as libc::c_double;
                }
                if (*named_dest).attrs.x_valid != 0 {
                    x = (*named_dest).attrs.x;
                }
                if (*named_dest).attrs.y_valid != 0 {
                    y = (*named_dest).attrs.y;
                }
                status = cairo_pdf_interchange_write_explicit_dest(
                    surface,
                    (*named_dest).page,
                    1 as libc::c_int,
                    x,
                    y,
                );
            } else {
                return _cairo_tag_error(
                    b"Link to dest=\"%s\" not found\0" as *const u8
                        as *const libc::c_char,
                    (*link).dest,
                ) as cairo_int_status_t
            }
        } else {
            cairo_pdf_interchange_write_explicit_dest(
                surface,
                (*link).page,
                (*link).has_pos,
                (*link).pos.x,
                (*link).pos.y,
            );
        }
        _cairo_pdf_surface_object_end(surface);
        i += 1;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_page_labels(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prev_prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: libc::c_int = 0;
    let mut prev_num: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut has_labels: cairo_bool_t = 0;
    num_elems = _cairo_array_num_elements(&mut (*surface).page_labels) as libc::c_int;
    has_labels = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        _cairo_array_copy_element(
            &mut (*surface).page_labels,
            i as libc::c_uint,
            &mut label as *mut *mut libc::c_char as *mut libc::c_void,
        );
        if !label.is_null() {
            has_labels = 1 as libc::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if has_labels == 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    (*surface).page_labels_res = _cairo_pdf_surface_new_object(surface);
    if (*surface).page_labels_res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_pdf_surface_object_begin(surface, (*surface).page_labels_res);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Nums [\n\0" as *const u8 as *const libc::c_char,
    );
    prefix = 0 as *mut libc::c_char;
    prev_prefix = 0 as *mut libc::c_char;
    num = 0 as libc::c_int;
    prev_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        _cairo_array_copy_element(
            &mut (*surface).page_labels,
            i as libc::c_uint,
            &mut label as *mut *mut libc::c_char as *mut libc::c_void,
        );
        if !label.is_null() {
            prefix = split_label(label, &mut num);
        } else {
            prefix = 0 as *mut libc::c_char;
            num = i + 1 as libc::c_int;
        }
        if strcmp_null(prefix, prev_prefix) == 0 || num != prev_num + 1 as libc::c_int {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   %d << \0" as *const u8 as *const libc::c_char,
                i,
            );
            if num != 0 {
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"/S /D /St %d \0" as *const u8 as *const libc::c_char,
                    num,
                );
            }
            if !prefix.is_null() {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                status = _cairo_utf8_to_pdf_string(prefix, &mut s);
                if status as u64 != 0 {
                    return status;
                }
                _cairo_output_stream_printf(
                    (*surface).object_stream.stream,
                    b"/P %s \0" as *const u8 as *const libc::c_char,
                    s,
                );
                free(s as *mut libc::c_void);
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b">>\n\0" as *const u8 as *const libc::c_char,
            );
        }
        free(prev_prefix as *mut libc::c_void);
        prev_prefix = prefix;
        prefix = 0 as *mut libc::c_char;
        prev_num = num;
        i += 1;
    }
    free(prefix as *mut libc::c_void);
    free(prev_prefix as *mut libc::c_void);
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"  ]\n>>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _collect_external_dest(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut dest: *mut cairo_pdf_named_dest_t = entry as *mut cairo_pdf_named_dest_t;
    let mut surface: *mut cairo_pdf_surface_t = closure as *mut cairo_pdf_surface_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    if (*dest).attrs.internal == 0 {
        let ref mut fresh18 = (*ic).num_dests;
        let fresh19 = *fresh18;
        *fresh18 = *fresh18 + 1;
        let ref mut fresh20 = *((*ic).sorted_dests).offset(fresh19 as isize);
        *fresh20 = dest;
    }
}
unsafe extern "C" fn _dest_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut dest_a: *const *const cairo_pdf_named_dest_t = a
        as *const *const cairo_pdf_named_dest_t;
    let mut dest_b: *const *const cairo_pdf_named_dest_t = b
        as *const *const cairo_pdf_named_dest_t;
    return strcmp((**dest_a).attrs.name, (**dest_b).attrs.name);
}
unsafe extern "C" fn _cairo_pdf_interchange_write_document_dests(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut i: libc::c_int = 0;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*ic).num_dests == 0 as libc::c_int {
        (*ic).dests_res.id = 0 as libc::c_int as libc::c_uint;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    let ref mut fresh21 = (*ic).sorted_dests;
    *fresh21 = calloc(
        (*ic).num_dests as libc::c_ulong,
        ::std::mem::size_of::<*mut cairo_pdf_named_dest_t>() as libc::c_ulong,
    ) as *mut *mut cairo_pdf_named_dest_t;
    if ((*ic).sorted_dests).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    (*ic).num_dests = 0 as libc::c_int;
    _cairo_hash_table_foreach(
        (*ic).named_dests,
        Some(
            _collect_external_dest
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        surface as *mut libc::c_void,
    );
    qsort(
        (*ic).sorted_dests as *mut libc::c_void,
        (*ic).num_dests as size_t,
        ::std::mem::size_of::<*mut cairo_pdf_named_dest_t>() as libc::c_ulong,
        Some(
            _dest_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    (*ic).dests_res = _cairo_pdf_surface_new_object(surface);
    if (*ic).dests_res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_pdf_surface_object_begin(surface, (*ic).dests_res);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Names [\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*ic).num_dests {
        let mut dest: *mut cairo_pdf_named_dest_t = *((*ic).sorted_dests)
            .offset(i as isize);
        let mut page_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
        let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut height: libc::c_double = 0.;
        if !((*dest).attrs.internal != 0) {
            if (*dest).extents.valid != 0 {
                x = (*dest).extents.extents.x as libc::c_double;
                y = (*dest).extents.extents.y as libc::c_double;
            }
            if (*dest).attrs.x_valid != 0 {
                x = (*dest).attrs.x;
            }
            if (*dest).attrs.y_valid != 0 {
                y = (*dest).attrs.y;
            }
            _cairo_array_copy_element(
                &mut (*surface).pages,
                ((*dest).page - 1 as libc::c_int) as libc::c_uint,
                &mut page_res as *mut cairo_pdf_resource_t as *mut libc::c_void,
            );
            _cairo_array_copy_element(
                &mut (*surface).page_heights,
                ((*dest).page - 1 as libc::c_int) as libc::c_uint,
                &mut height as *mut libc::c_double as *mut libc::c_void,
            );
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   (%s) [%d 0 R /XYZ %f %f 0]\n\0" as *const u8 as *const libc::c_char,
                (*dest).attrs.name,
                page_res.id,
                x,
                height - y,
            );
        }
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"  ]\n>>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_names_dict(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_interchange_write_document_dests(surface);
    if status as u64 != 0 {
        return status;
    }
    (*surface).names_dict_res.id = 0 as libc::c_int as libc::c_uint;
    if (*ic).dests_res.id != 0 as libc::c_int as libc::c_uint {
        (*surface).names_dict_res = _cairo_pdf_surface_new_object(surface);
        if (*surface).names_dict_res.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_pdf_surface_object_begin(surface, (*surface).names_dict_res);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"<< /Dests %d 0 R >>\n\0" as *const u8 as *const libc::c_char,
            (*ic).dests_res.id,
        );
        _cairo_pdf_surface_object_end(surface);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_interchange_write_docinfo(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut num_elems: libc::c_uint = 0;
    let mut data: *mut metadata = 0 as *mut metadata;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    (*surface).docinfo_res = _cairo_pdf_surface_new_object(surface);
    if (*surface).docinfo_res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_pdf_surface_object_begin(surface, (*surface).docinfo_res);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Producer (cairo %s (https://cairographics.org))\n\0" as *const u8
            as *const libc::c_char,
        cairo_version_string(),
    );
    if !((*ic).docinfo.title).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Title %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.title,
        );
    }
    if !((*ic).docinfo.author).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Author %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.author,
        );
    }
    if !((*ic).docinfo.subject).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Subject %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.subject,
        );
    }
    if !((*ic).docinfo.keywords).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Keywords %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.keywords,
        );
    }
    if !((*ic).docinfo.creator).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Creator %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.creator,
        );
    }
    if !((*ic).docinfo.create_date).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /CreationDate %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.create_date,
        );
    }
    if !((*ic).docinfo.mod_date).is_null() {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /ModDate %s\n\0" as *const u8 as *const libc::c_char,
            (*ic).docinfo.mod_date,
        );
    }
    num_elems = _cairo_array_num_elements(&mut (*ic).custom_metadata);
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_elems {
        data = _cairo_array_index(&mut (*ic).custom_metadata, i) as *mut metadata;
        if !((*data).value).is_null() {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /\0" as *const u8 as *const libc::c_char,
            );
            p = (*data).name as *mut libc::c_uchar;
            while *p != 0 {
                if (*p as libc::c_int) < 0x21 as libc::c_int
                    || *p as libc::c_int > 0x7e as libc::c_int
                    || *p as libc::c_int == '#' as i32 || *p as libc::c_int == '/' as i32
                {
                    _cairo_output_stream_printf(
                        (*surface).object_stream.stream,
                        b"#%02x\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_int,
                    );
                } else {
                    _cairo_output_stream_printf(
                        (*surface).object_stream.stream,
                        b"%c\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_int,
                    );
                }
                p = p.offset(1);
            }
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b" %s\n\0" as *const u8 as *const libc::c_char,
                (*data).value,
            );
        }
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_interchange_begin_structure_tag(
    mut surface: *mut cairo_pdf_surface_t,
    mut tag_type: cairo_tag_type_t,
    mut name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut page_num: libc::c_int = 0;
    let mut mcid: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = add_tree_node(
            surface,
            (*ic).current_node,
            name,
            &mut (*ic).current_node,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_tag_stack_set_top_data(
            &mut (*ic).analysis_tag_stack,
            (*ic).current_node as *mut libc::c_void,
        );
        if tag_type as libc::c_uint & TAG_TYPE_LINK as libc::c_int as libc::c_uint != 0 {
            status = add_annotation(surface, (*ic).current_node, name, attributes);
            if status as u64 != 0 {
                return status;
            }
            cairo_list_add_tail(
                &mut (*(*ic).current_node).extents.link,
                &mut (*ic).extents_list,
            );
        }
    } else if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        let ref mut fresh22 = (*ic).current_node;
        *fresh22 = (*_cairo_tag_stack_top_elem(&mut (*ic).render_tag_stack)).data
            as *mut cairo_pdf_struct_tree_node_t;
        if !((*ic).current_node).is_null() {} else {
            __assert_fail(
                b"ic->current_node != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-pdf-interchange.c\0" as *const u8 as *const libc::c_char,
                1369 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 131],
                    &[libc::c_char; 131],
                >(
                    b"cairo_int_status_t _cairo_pdf_interchange_begin_structure_tag(cairo_pdf_surface_t *, cairo_tag_type_t, const char *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
        if is_leaf_node((*ic).current_node) != 0 {
            page_num = _cairo_array_num_elements(&mut (*surface).pages) as libc::c_int;
            add_mcid_to_node(surface, (*ic).current_node, page_num, &mut mcid);
            status = _cairo_pdf_operators_tag_begin(
                &mut (*surface).pdf_operators,
                name,
                mcid,
            );
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_interchange_begin_dest_tag(
    mut surface: *mut cairo_pdf_surface_t,
    mut tag_type: cairo_tag_type_t,
    mut name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut dest: *mut cairo_pdf_named_dest_t = 0 as *mut cairo_pdf_named_dest_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        dest = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<cairo_pdf_named_dest_t>() as libc::c_ulong,
        ) as *mut cairo_pdf_named_dest_t;
        if dest.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_tag_parse_dest_attributes(attributes, &mut (*dest).attrs);
        if status as u64 != 0 {
            free(dest as *mut libc::c_void);
            return status;
        }
        (*dest).page = _cairo_array_num_elements(&mut (*surface).pages) as libc::c_int;
        init_named_dest_key(dest);
        status = _cairo_hash_table_insert((*ic).named_dests, &mut (*dest).base)
            as cairo_int_status_t;
        if status as u64 != 0 {
            free((*dest).attrs.name as *mut libc::c_void);
            free(dest as *mut libc::c_void);
            return status;
        }
        _cairo_tag_stack_set_top_data(
            &mut (*ic).analysis_tag_stack,
            dest as *mut libc::c_void,
        );
        cairo_list_add_tail(&mut (*dest).extents.link, &mut (*ic).extents_list);
        let ref mut fresh23 = (*ic).num_dests;
        *fresh23 += 1;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_tag_begin(
    mut surface: *mut cairo_pdf_surface_t,
    mut name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut tag_type: cairo_tag_type_t = TAG_TYPE_INVALID;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_tag_stack_push(&mut (*ic).analysis_tag_stack, name, attributes);
    } else if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        status = _cairo_tag_stack_push(&mut (*ic).render_tag_stack, name, attributes);
        let ref mut fresh24 = (*ic).push_data_index;
        let fresh25 = *fresh24;
        *fresh24 = *fresh24 + 1;
        _cairo_array_copy_element(
            &mut (*ic).push_data,
            fresh25 as libc::c_uint,
            &mut ptr as *mut *mut libc::c_void as *mut libc::c_void,
        );
        _cairo_tag_stack_set_top_data(&mut (*ic).render_tag_stack, ptr);
    }
    if status as u64 != 0 {
        return status;
    }
    tag_type = _cairo_tag_get_type(name);
    if tag_type as libc::c_uint & TAG_TYPE_STRUCTURE as libc::c_int as libc::c_uint != 0
    {
        status = _cairo_pdf_interchange_begin_structure_tag(
            surface,
            tag_type,
            name,
            attributes,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    if tag_type as libc::c_uint & TAG_TYPE_DEST as libc::c_int as libc::c_uint != 0 {
        status = _cairo_pdf_interchange_begin_dest_tag(
            surface,
            tag_type,
            name,
            attributes,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        ptr = (*_cairo_tag_stack_top_elem(&mut (*ic).analysis_tag_stack)).data;
        status = _cairo_array_append(
            &mut (*ic).push_data,
            &mut ptr as *mut *mut libc::c_void as *const libc::c_void,
        ) as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_interchange_end_structure_tag(
    mut surface: *mut cairo_pdf_surface_t,
    mut tag_type: cairo_tag_type_t,
    mut elem: *mut cairo_tag_stack_elem_t,
) -> cairo_int_status_t {
    let mut node: *const cairo_pdf_struct_tree_node_t = 0
        as *const cairo_pdf_struct_tree_node_t;
    let mut tag: *mut tag_extents = 0 as *mut tag_extents;
    let mut next: *mut tag_extents = 0 as *mut tag_extents;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    if !((*elem).data).is_null() {} else {
        __assert_fail(
            b"elem->data != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-interchange.c\0" as *const u8 as *const libc::c_char,
            1473 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"cairo_int_status_t _cairo_pdf_interchange_end_structure_tag(cairo_pdf_surface_t *, cairo_tag_type_t, cairo_tag_stack_elem_t *)\0",
            ))
                .as_ptr(),
        );
    }
    node = (*elem).data as *const cairo_pdf_struct_tree_node_t;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        if tag_type as libc::c_uint & TAG_TYPE_LINK as libc::c_int as libc::c_uint != 0 {
            tag = ({
                let mut mptr__: *const cairo_list_t = (*ic).extents_list.next;
                (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                    as *mut tag_extents
            });
            next = ({
                let mut mptr__: *const cairo_list_t = (*tag).link.next;
                (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                    as *mut tag_extents
            });
            while &mut (*tag).link as *mut cairo_list_t
                != &mut (*ic).extents_list as *mut cairo_list_t
            {
                if tag == &(*node).extents as *const tag_extents as *mut tag_extents {
                    cairo_list_del(&mut (*tag).link);
                    break;
                } else {
                    tag = next;
                    next = ({
                        let mut mptr__: *const cairo_list_t = (*next).link.next;
                        (mptr__ as *mut libc::c_char)
                            .offset(-(24 as libc::c_ulong as isize)) as *mut tag_extents
                    });
                }
            }
        }
    } else if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        if is_leaf_node((*ic).current_node) != 0 {
            status = _cairo_pdf_operators_tag_end(&mut (*surface).pdf_operators);
            if status as u64 != 0 {
                return status;
            }
        }
    }
    let ref mut fresh26 = (*ic).current_node;
    *fresh26 = (*(*ic).current_node).parent;
    if !((*ic).current_node).is_null() {} else {
        __assert_fail(
            b"ic->current_node != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-interchange.c\0" as *const u8 as *const libc::c_char,
            1494 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"cairo_int_status_t _cairo_pdf_interchange_end_structure_tag(cairo_pdf_surface_t *, cairo_tag_type_t, cairo_tag_stack_elem_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_interchange_end_dest_tag(
    mut surface: *mut cairo_pdf_surface_t,
    mut tag_type: cairo_tag_type_t,
    mut elem: *mut cairo_tag_stack_elem_t,
) -> cairo_int_status_t {
    let mut tag: *mut tag_extents = 0 as *mut tag_extents;
    let mut next: *mut tag_extents = 0 as *mut tag_extents;
    let mut dest: *mut cairo_pdf_named_dest_t = 0 as *mut cairo_pdf_named_dest_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        if !((*elem).data).is_null() {} else {
            __assert_fail(
                b"elem->data != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-pdf-interchange.c\0" as *const u8 as *const libc::c_char,
                1509 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"cairo_int_status_t _cairo_pdf_interchange_end_dest_tag(cairo_pdf_surface_t *, cairo_tag_type_t, cairo_tag_stack_elem_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        dest = (*elem).data as *mut cairo_pdf_named_dest_t;
        tag = ({
            let mut mptr__: *const cairo_list_t = (*ic).extents_list.next;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut tag_extents
        });
        next = ({
            let mut mptr__: *const cairo_list_t = (*tag).link.next;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut tag_extents
        });
        while &mut (*tag).link as *mut cairo_list_t
            != &mut (*ic).extents_list as *mut cairo_list_t
        {
            if tag == &mut (*dest).extents as *mut tag_extents {
                cairo_list_del(&mut (*tag).link);
                break;
            } else {
                tag = next;
                next = ({
                    let mut mptr__: *const cairo_list_t = (*next).link.next;
                    (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                        as *mut tag_extents
                });
            }
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_tag_end(
    mut surface: *mut cairo_pdf_surface_t,
    mut name: *const libc::c_char,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut tag_type: cairo_tag_type_t = TAG_TYPE_INVALID;
    let mut elem: *mut cairo_tag_stack_elem_t = 0 as *mut cairo_tag_stack_elem_t;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_tag_stack_pop(&mut (*ic).analysis_tag_stack, name, &mut elem);
    } else if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        status = _cairo_tag_stack_pop(&mut (*ic).render_tag_stack, name, &mut elem);
    }
    if status as u64 != 0 {
        return status;
    }
    tag_type = _cairo_tag_get_type(name);
    if tag_type as libc::c_uint & TAG_TYPE_STRUCTURE as libc::c_int as libc::c_uint != 0
    {
        status = _cairo_pdf_interchange_end_structure_tag(surface, tag_type, elem);
        if status as u64 != 0 {
            current_block = 18221575213305925356;
        } else {
            current_block = 12209867499936983673;
        }
    } else {
        current_block = 12209867499936983673;
    }
    match current_block {
        12209867499936983673 => {
            if tag_type as libc::c_uint & TAG_TYPE_DEST as libc::c_int as libc::c_uint
                != 0
            {
                status = _cairo_pdf_interchange_end_dest_tag(surface, tag_type, elem);
                status as u64 != 0;
            }
        }
        _ => {}
    }
    _cairo_tag_stack_free_elem(elem);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_add_operation_extents(
    mut surface: *mut cairo_pdf_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut tag: *mut tag_extents = 0 as *mut tag_extents;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        tag = ({
            let mut mptr__: *const cairo_list_t = (*ic).extents_list.next;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut tag_extents
        });
        while &mut (*tag).link as *mut cairo_list_t
            != &mut (*ic).extents_list as *mut cairo_list_t
        {
            if (*tag).valid != 0 {
                _cairo_rectangle_union(&mut (*tag).extents, extents);
            } else {
                (*tag).extents = *extents;
                (*tag).valid = 1 as libc::c_int;
            }
            tag = ({
                let mut mptr__: *const cairo_list_t = (*tag).link.next;
                (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                    as *mut tag_extents
            });
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_begin_page_content(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut page_num: libc::c_int = 0;
    let mut mcid: libc::c_int = 0;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        _cairo_array_truncate(&mut (*ic).mcid_to_tree, 0 as libc::c_int as libc::c_uint);
        _cairo_array_truncate(&mut (*ic).push_data, 0 as libc::c_int as libc::c_uint);
        let ref mut fresh27 = (*ic).begin_page_node;
        *fresh27 = (*ic).current_node;
    } else if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        (*ic).push_data_index = 0 as libc::c_int;
        let ref mut fresh28 = (*ic).current_node;
        *fresh28 = (*ic).begin_page_node;
        if !((*ic).end_page_node).is_null() && is_leaf_node((*ic).end_page_node) != 0 {
            page_num = _cairo_array_num_elements(&mut (*surface).pages) as libc::c_int;
            add_mcid_to_node(surface, (*ic).end_page_node, page_num, &mut mcid);
            status = _cairo_pdf_operators_tag_begin(
                &mut (*surface).pdf_operators,
                (*(*ic).end_page_node).name,
                mcid,
            );
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_end_page_content(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        let ref mut fresh29 = (*ic).end_page_node;
        *fresh29 = (*ic).current_node;
        if is_leaf_node((*ic).current_node) != 0 {
            status = _cairo_pdf_operators_tag_end(&mut (*surface).pdf_operators);
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_write_page_objects(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = cairo_pdf_interchange_write_page_annots(surface);
    if status as u64 != 0 {
        return status;
    }
    cairo_pdf_interchange_clear_annotations(surface);
    return cairo_pdf_interchange_write_page_parent_elems(surface);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_write_document_objects(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut tag_type: cairo_tag_stack_structure_type_t = TAG_TREE_TYPE_TAGGED;
    tag_type = _cairo_tag_stack_get_structure_type(&mut (*ic).analysis_tag_stack);
    if tag_type as libc::c_uint == TAG_TREE_TYPE_TAGGED as libc::c_int as libc::c_uint
        || tag_type as libc::c_uint
            == TAG_TREE_TYPE_STRUCTURE as libc::c_int as libc::c_uint
        || tag_type as libc::c_uint
            == TAG_TREE_TYPE_LINK_ONLY as libc::c_int as libc::c_uint
    {
        status = cairo_pdf_interchange_write_parent_tree(surface);
        if status as u64 != 0 {
            return status;
        }
        status = cairo_pdf_interchange_write_struct_tree(surface);
        if status as u64 != 0 {
            return status;
        }
        if tag_type as libc::c_uint
            == TAG_TREE_TYPE_TAGGED as libc::c_int as libc::c_uint
        {
            (*surface).tagged = 1 as libc::c_int;
        }
    }
    status = cairo_pdf_interchange_write_outline(surface);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_pdf_interchange_write_page_labels(surface);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_pdf_interchange_write_forward_links(surface);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_pdf_interchange_write_names_dict(surface);
    if status as u64 != 0 {
        return status;
    }
    status = cairo_pdf_interchange_write_docinfo(surface);
    return status;
}
unsafe extern "C" fn _cairo_pdf_interchange_set_create_date(
    mut surface: *mut cairo_pdf_surface_t,
) {
    let mut utc: time_t = 0;
    let mut local: time_t = 0;
    let mut offset: time_t = 0;
    let mut tm_local: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tm_utc: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut buf: [libc::c_char; 50] = [0; 50];
    let mut buf_size: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    utc = time(0 as *mut time_t);
    localtime_r(&mut utc, &mut tm_local);
    strftime(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
        b"(D:%Y%m%d%H%M%S\0" as *const u8 as *const libc::c_char,
        &mut tm_local,
    );
    gmtime_r(&mut utc, &mut tm_utc);
    tm_utc.tm_isdst = tm_local.tm_isdst;
    local = mktime(&mut tm_utc);
    offset = difftime(utc, local) as time_t;
    if offset == 0 as libc::c_int as libc::c_long {
        strcat(buf.as_mut_ptr(), b"Z\0" as *const u8 as *const libc::c_char);
    } else {
        if offset > 0 as libc::c_int as libc::c_long {
            strcat(buf.as_mut_ptr(), b"+\0" as *const u8 as *const libc::c_char);
        } else {
            strcat(buf.as_mut_ptr(), b"-\0" as *const u8 as *const libc::c_char);
            offset = -offset;
        }
        p = buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
        buf_size = (::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong)
            .wrapping_sub(strlen(buf.as_mut_ptr())) as libc::c_int;
        snprintf(
            p,
            buf_size as libc::c_ulong,
            b"%02d'%02d\0" as *const u8 as *const libc::c_char,
            (offset / 3600 as libc::c_int as libc::c_long) as libc::c_int,
            (offset % 3600 as libc::c_int as libc::c_long) as libc::c_int
                / 60 as libc::c_int,
        );
    }
    strcat(buf.as_mut_ptr(), b")\0" as *const u8 as *const libc::c_char);
    let ref mut fresh30 = (*ic).docinfo.create_date;
    *fresh30 = strdup(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_init(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut outline_root: *mut cairo_pdf_outline_entry_t = 0
        as *mut cairo_pdf_outline_entry_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    _cairo_tag_stack_init(&mut (*ic).analysis_tag_stack);
    _cairo_tag_stack_init(&mut (*ic).render_tag_stack);
    _cairo_array_init(
        &mut (*ic).push_data,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh31 = (*ic).struct_root;
    *fresh31 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_pdf_struct_tree_node_t>() as libc::c_ulong,
    ) as *mut cairo_pdf_struct_tree_node_t;
    if ((*ic).struct_root).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    cairo_list_init(&mut (*(*ic).struct_root).children);
    _cairo_array_init(
        &mut (*(*ic).struct_root).mcid,
        ::std::mem::size_of::<page_mcid>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh32 = (*ic).current_node;
    *fresh32 = (*ic).struct_root;
    let ref mut fresh33 = (*ic).begin_page_node;
    *fresh33 = 0 as *mut cairo_pdf_struct_tree_node_t;
    let ref mut fresh34 = (*ic).end_page_node;
    *fresh34 = 0 as *mut cairo_pdf_struct_tree_node_t;
    _cairo_array_init(
        &mut (*ic).parent_tree,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*ic).mcid_to_tree,
        ::std::mem::size_of::<*mut cairo_pdf_struct_tree_node_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*ic).annots,
        ::std::mem::size_of::<*mut cairo_pdf_annotation_t>() as libc::c_ulong
            as libc::c_uint,
    );
    (*ic).parent_tree_res.id = 0 as libc::c_int as libc::c_uint;
    cairo_list_init(&mut (*ic).extents_list);
    let ref mut fresh35 = (*ic).named_dests;
    *fresh35 = _cairo_hash_table_create(
        Some(
            _named_dest_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if ((*ic).named_dests).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    (*ic).num_dests = 0 as libc::c_int;
    let ref mut fresh36 = (*ic).sorted_dests;
    *fresh36 = 0 as *mut *mut cairo_pdf_named_dest_t;
    (*ic).dests_res.id = 0 as libc::c_int as libc::c_uint;
    _cairo_array_init(
        &mut (*ic).outline,
        ::std::mem::size_of::<*mut cairo_pdf_outline_entry_t>() as libc::c_ulong
            as libc::c_uint,
    );
    outline_root = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_pdf_outline_entry_t>() as libc::c_ulong,
    ) as *mut cairo_pdf_outline_entry_t;
    if outline_root.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    memset(
        &mut (*ic).docinfo as *mut docinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<docinfo>() as libc::c_ulong,
    );
    _cairo_array_init(
        &mut (*ic).custom_metadata,
        ::std::mem::size_of::<metadata>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_pdf_interchange_set_create_date(surface);
    status = _cairo_array_append(
        &mut (*ic).outline,
        &mut outline_root as *mut *mut cairo_pdf_outline_entry_t as *const libc::c_void,
    ) as cairo_int_status_t;
    return status;
}
unsafe extern "C" fn _cairo_pdf_interchange_free_outlines(
    mut surface: *mut cairo_pdf_surface_t,
) {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_elems = _cairo_array_num_elements(&mut (*ic).outline) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        let mut outline: *mut cairo_pdf_outline_entry_t = 0
            as *mut cairo_pdf_outline_entry_t;
        _cairo_array_copy_element(
            &mut (*ic).outline,
            i as libc::c_uint,
            &mut outline as *mut *mut cairo_pdf_outline_entry_t as *mut libc::c_void,
        );
        free((*outline).name as *mut libc::c_void);
        free((*outline).link_attrs.dest as *mut libc::c_void);
        free((*outline).link_attrs.uri as *mut libc::c_void);
        free((*outline).link_attrs.file as *mut libc::c_void);
        free(outline as *mut libc::c_void);
        i += 1;
    }
    _cairo_array_fini(&mut (*ic).outline);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_fini(
    mut surface: *mut cairo_pdf_surface_t,
) {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut i: libc::c_uint = 0;
    let mut num_elems: libc::c_uint = 0;
    let mut data: *mut metadata = 0 as *mut metadata;
    _cairo_tag_stack_fini(&mut (*ic).analysis_tag_stack);
    _cairo_tag_stack_fini(&mut (*ic).render_tag_stack);
    _cairo_array_fini(&mut (*ic).push_data);
    free_node((*ic).struct_root);
    _cairo_array_fini(&mut (*ic).mcid_to_tree);
    cairo_pdf_interchange_clear_annotations(surface);
    _cairo_array_fini(&mut (*ic).annots);
    _cairo_array_fini(&mut (*ic).parent_tree);
    _cairo_hash_table_foreach(
        (*ic).named_dests,
        Some(
            _named_dest_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*ic).named_dests as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*ic).named_dests);
    free((*ic).sorted_dests as *mut libc::c_void);
    _cairo_pdf_interchange_free_outlines(surface);
    free((*ic).docinfo.title as *mut libc::c_void);
    free((*ic).docinfo.author as *mut libc::c_void);
    free((*ic).docinfo.subject as *mut libc::c_void);
    free((*ic).docinfo.keywords as *mut libc::c_void);
    free((*ic).docinfo.creator as *mut libc::c_void);
    free((*ic).docinfo.create_date as *mut libc::c_void);
    free((*ic).docinfo.mod_date as *mut libc::c_void);
    num_elems = _cairo_array_num_elements(&mut (*ic).custom_metadata);
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_elems {
        data = _cairo_array_index(&mut (*ic).custom_metadata, i) as *mut metadata;
        free((*data).name as *mut libc::c_void);
        free((*data).value as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    _cairo_array_fini(&mut (*ic).custom_metadata);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_add_outline(
    mut surface: *mut cairo_pdf_surface_t,
    mut parent_id: libc::c_int,
    mut name: *const libc::c_char,
    mut link_attribs: *const libc::c_char,
    mut flags: cairo_pdf_outline_flags_t,
    mut id: *mut libc::c_int,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut outline: *mut cairo_pdf_outline_entry_t = 0
        as *mut cairo_pdf_outline_entry_t;
    let mut parent: *mut cairo_pdf_outline_entry_t = 0 as *mut cairo_pdf_outline_entry_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if parent_id < 0 as libc::c_int
        || parent_id >= _cairo_array_num_elements(&mut (*ic).outline) as libc::c_int
    {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    outline = (if ::std::mem::size_of::<cairo_pdf_outline_entry_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_pdf_outline_entry_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_pdf_outline_entry_t;
    if outline.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_tag_parse_link_attributes(link_attribs, &mut (*outline).link_attrs);
    if status as u64 != 0 {
        free(outline as *mut libc::c_void);
        return status;
    }
    (*outline).res = _cairo_pdf_surface_new_object(surface);
    if (*outline).res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh37 = (*outline).name;
    *fresh37 = strdup(name);
    (*outline).flags = flags;
    (*outline).count = 0 as libc::c_int;
    _cairo_array_copy_element(
        &mut (*ic).outline,
        parent_id as libc::c_uint,
        &mut parent as *mut *mut cairo_pdf_outline_entry_t as *mut libc::c_void,
    );
    let ref mut fresh38 = (*outline).parent;
    *fresh38 = parent;
    let ref mut fresh39 = (*outline).first_child;
    *fresh39 = 0 as *mut _cairo_pdf_outline_entry;
    let ref mut fresh40 = (*outline).last_child;
    *fresh40 = 0 as *mut _cairo_pdf_outline_entry;
    let ref mut fresh41 = (*outline).next;
    *fresh41 = 0 as *mut _cairo_pdf_outline_entry;
    if !((*parent).last_child).is_null() {
        let ref mut fresh42 = (*(*parent).last_child).next;
        *fresh42 = outline;
        let ref mut fresh43 = (*outline).prev;
        *fresh43 = (*parent).last_child;
        let ref mut fresh44 = (*parent).last_child;
        *fresh44 = outline;
    } else {
        let ref mut fresh45 = (*parent).first_child;
        *fresh45 = outline;
        let ref mut fresh46 = (*parent).last_child;
        *fresh46 = outline;
        let ref mut fresh47 = (*outline).prev;
        *fresh47 = 0 as *mut _cairo_pdf_outline_entry;
    }
    *id = _cairo_array_num_elements(&mut (*ic).outline) as libc::c_int;
    status = _cairo_array_append(
        &mut (*ic).outline,
        &mut outline as *mut *mut cairo_pdf_outline_entry_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    outline = (*outline).parent;
    while !outline.is_null() {
        if (*outline).flags as libc::c_uint
            & CAIRO_PDF_OUTLINE_FLAG_OPEN as libc::c_int as libc::c_uint != 0
        {
            let ref mut fresh48 = (*outline).count;
            *fresh48 += 1;
            outline = (*outline).parent;
        } else {
            let ref mut fresh49 = (*outline).count;
            *fresh49 -= 1;
            break;
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn iso8601_to_pdf_date_string(
    mut iso: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut buf: [libc::c_char; 40] = [0; 40];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    p = iso;
    while *p != 0 {
        if _cairo_isdigit(*p as libc::c_int) == 0 && *p as libc::c_int != '-' as i32
            && *p as libc::c_int != 'T' as i32 && *p as libc::c_int != ':' as i32
            && *p as libc::c_int != 'Z' as i32 && *p as libc::c_int != '+' as i32
        {
            return 0 as *mut libc::c_char;
        }
        p = p.offset(1);
    }
    p = iso;
    strcpy(buf.as_mut_ptr(), b"(\0" as *const u8 as *const libc::c_char);
    if strlen(p) < 4 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    strncat(buf.as_mut_ptr(), p, 4 as libc::c_int as libc::c_ulong);
    p = p.offset(4 as libc::c_int as isize);
    i = 0 as libc::c_int;
    loop {
        if !(i < 5 as libc::c_int) {
            current_block = 15652330335145281839;
            break;
        }
        if strlen(p) < 3 as libc::c_int as libc::c_ulong {
            current_block = 4688994028827908915;
            break;
        }
        strncat(
            buf.as_mut_ptr(),
            p.offset(1 as libc::c_int as isize),
            2 as libc::c_int as libc::c_ulong,
        );
        p = p.offset(3 as libc::c_int as isize);
        i += 1;
    }
    match current_block {
        15652330335145281839 => {
            if !(strlen(p) < 1 as libc::c_int as libc::c_ulong) {
                strncat(buf.as_mut_ptr(), p, 1 as libc::c_int as libc::c_ulong);
                p = p.offset(1 as libc::c_int as isize);
                if !(strlen(p) < 2 as libc::c_int as libc::c_ulong) {
                    strncat(buf.as_mut_ptr(), p, 2 as libc::c_int as libc::c_ulong);
                    strcat(buf.as_mut_ptr(), b"'\0" as *const u8 as *const libc::c_char);
                    p = p.offset(2 as libc::c_int as isize);
                    if !(strlen(p) < 3 as libc::c_int as libc::c_ulong) {
                        strncat(
                            buf.as_mut_ptr(),
                            p.offset(1 as libc::c_int as isize),
                            2 as libc::c_int as libc::c_ulong,
                        );
                        strcat(
                            buf.as_mut_ptr(),
                            b"'\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
        _ => {}
    }
    strcat(buf.as_mut_ptr(), b")\0" as *const u8 as *const libc::c_char);
    return strdup(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_set_metadata(
    mut surface: *mut cairo_pdf_surface_t,
    mut metadata: cairo_pdf_metadata_t,
    mut utf8: *const libc::c_char,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !utf8.is_null() {
        if metadata as libc::c_uint
            == CAIRO_PDF_METADATA_CREATE_DATE as libc::c_int as libc::c_uint
            || metadata as libc::c_uint
                == CAIRO_PDF_METADATA_MOD_DATE as libc::c_int as libc::c_uint
        {
            s = iso8601_to_pdf_date_string(utf8);
        } else {
            status = _cairo_utf8_to_pdf_string(utf8, &mut s) as cairo_status_t;
            if status as u64 != 0 {
                return status as cairo_int_status_t;
            }
        }
    }
    match metadata as libc::c_uint {
        0 => {
            free((*ic).docinfo.title as *mut libc::c_void);
            let ref mut fresh50 = (*ic).docinfo.title;
            *fresh50 = s;
        }
        1 => {
            free((*ic).docinfo.author as *mut libc::c_void);
            let ref mut fresh51 = (*ic).docinfo.author;
            *fresh51 = s;
        }
        2 => {
            free((*ic).docinfo.subject as *mut libc::c_void);
            let ref mut fresh52 = (*ic).docinfo.subject;
            *fresh52 = s;
        }
        3 => {
            free((*ic).docinfo.keywords as *mut libc::c_void);
            let ref mut fresh53 = (*ic).docinfo.keywords;
            *fresh53 = s;
        }
        4 => {
            free((*ic).docinfo.creator as *mut libc::c_void);
            let ref mut fresh54 = (*ic).docinfo.creator;
            *fresh54 = s;
        }
        5 => {
            free((*ic).docinfo.create_date as *mut libc::c_void);
            let ref mut fresh55 = (*ic).docinfo.create_date;
            *fresh55 = s;
        }
        6 => {
            free((*ic).docinfo.mod_date as *mut libc::c_void);
            let ref mut fresh56 = (*ic).docinfo.mod_date;
            *fresh56 = s;
        }
        _ => {}
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
static mut reserved_metadata_names: [*const libc::c_char; 10] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"Title\0" as *const u8 as *const libc::c_char,
    b"Author\0" as *const u8 as *const libc::c_char,
    b"Subject\0" as *const u8 as *const libc::c_char,
    b"Keywords\0" as *const u8 as *const libc::c_char,
    b"Creator\0" as *const u8 as *const libc::c_char,
    b"Producer\0" as *const u8 as *const libc::c_char,
    b"CreationDate\0" as *const u8 as *const libc::c_char,
    b"ModDate\0" as *const u8 as *const libc::c_char,
    b"Trapped\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_interchange_set_custom_metadata(
    mut surface: *mut cairo_pdf_surface_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> cairo_int_status_t {
    let mut ic: *mut cairo_pdf_interchange_t = &mut (*surface).interchange;
    let mut data: *mut metadata = 0 as *mut metadata;
    let mut new_data: metadata = metadata {
        name: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
    };
    let mut i: libc::c_int = 0;
    let mut num_elems: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if name.is_null() {
        return CAIRO_STATUS_NULL_POINTER as libc::c_int as cairo_int_status_t;
    }
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {
        if strcmp(name, reserved_metadata_names[i as usize]) == 0 as libc::c_int {
            return CAIRO_STATUS_INVALID_STRING as libc::c_int as cairo_int_status_t;
        }
        i += 1;
    }
    num_elems = _cairo_array_num_elements(&mut (*ic).custom_metadata) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        data = _cairo_array_index(&mut (*ic).custom_metadata, i as libc::c_uint)
            as *mut metadata;
        if strcmp(name, (*data).name) == 0 as libc::c_int {
            free((*data).value as *mut libc::c_void);
            let ref mut fresh57 = (*data).value;
            *fresh57 = 0 as *mut libc::c_char;
            if !value.is_null() && strlen(value) != 0 {
                status = _cairo_utf8_to_pdf_string(value, &mut s);
                if status as u64 != 0 {
                    return status;
                }
                let ref mut fresh58 = (*data).value;
                *fresh58 = s;
            }
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        i += 1;
    }
    status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    if !value.is_null() && strlen(value) != 0 {
        new_data.name = strdup(name);
        status = _cairo_utf8_to_pdf_string(value, &mut s);
        if status as u64 != 0 {
            return status;
        }
        new_data.value = s;
        status = _cairo_array_append(
            &mut (*ic).custom_metadata,
            &mut new_data as *mut metadata as *const libc::c_void,
        ) as cairo_int_status_t;
    }
    return status;
}
