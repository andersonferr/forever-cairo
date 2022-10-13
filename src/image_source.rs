use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sin(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_pattern_set_matrix(
        pattern: *mut cairo_pattern_t,
        matrix: *const cairo_matrix_t,
    );
    fn cairo_matrix_init_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn cairo_matrix_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn pixman_image_create_solid_fill(
        color: *const pixman_color_t,
    ) -> *mut pixman_image_t;
    fn pixman_image_create_linear_gradient(
        p1: *const pixman_point_fixed_t,
        p2: *const pixman_point_fixed_t,
        stops: *const pixman_gradient_stop_t,
        n_stops: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_image_create_radial_gradient(
        inner: *const pixman_point_fixed_t,
        outer: *const pixman_point_fixed_t,
        inner_radius: pixman_fixed_t,
        outer_radius: pixman_fixed_t,
        stops: *const pixman_gradient_stop_t,
        n_stops: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_image_create_bits(
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        bits: *mut uint32_t,
        rowstride_bytes: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_image_ref(image: *mut pixman_image_t) -> *mut pixman_image_t;
    fn pixman_image_unref(image: *mut pixman_image_t) -> pixman_bool_t;
    fn pixman_image_set_destroy_function(
        image: *mut pixman_image_t,
        function: pixman_image_destroy_func_t,
        data: *mut libc::c_void,
    );
    fn pixman_image_set_transform(
        image: *mut pixman_image_t,
        transform: *const pixman_transform_t,
    ) -> pixman_bool_t;
    fn pixman_image_set_repeat(image: *mut pixman_image_t, repeat: pixman_repeat_t);
    fn pixman_image_set_filter(
        image: *mut pixman_image_t,
        filter: pixman_filter_t,
        filter_params: *const pixman_fixed_t,
        n_filter_params: libc::c_int,
    ) -> pixman_bool_t;
    fn pixman_image_set_component_alpha(
        image: *mut pixman_image_t,
        component_alpha: pixman_bool_t,
    );
    fn pixman_image_get_data(image: *mut pixman_image_t) -> *mut uint32_t;
    fn pixman_image_get_stride(image: *mut pixman_image_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_default_source(
        surface: *mut libc::c_void,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
    fn _cairo_surface_attach_snapshot(
        surface: *mut cairo_surface_t,
        snapshot: *mut cairo_surface_t,
        detach_func: cairo_surface_func_t,
    );
    fn _cairo_surface_has_snapshot(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_create_with_content(
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_matrix_transform_bounding_box(
        matrix: *const cairo_matrix_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_matrix_to_pixman_matrix_offset(
        matrix: *const cairo_matrix_t,
        filter: cairo_filter_t,
        xc: libc::c_double,
        yc: libc::c_double,
        out_transform: *mut pixman_transform_t,
        out_x_offset: *mut libc::c_int,
        out_y_offset: *mut libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_pattern_is_opaque_solid(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_gradient_pattern_fit_to_range(
        gradient: *const cairo_gradient_pattern_t,
        max_value: libc::c_double,
        out_matrix: *mut cairo_matrix_t,
        out_circle: *mut cairo_circle_double_t,
    );
    fn _cairo_pattern_init_static_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    );
    fn _cairo_mesh_pattern_rasterize(
        mesh: *const cairo_mesh_pattern_t,
        data: *mut libc::c_void,
        width: libc::c_int,
        height: libc::c_int,
        stride: libc::c_int,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn _cairo_raster_source_pattern_acquire(
        abstract_pattern: *const cairo_pattern_t,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_raster_source_pattern_release(
        abstract_pattern: *const cairo_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_recording_surface_replay_with_clip(
        surface: *mut cairo_surface_t,
        surface_transform: *const cairo_matrix_t,
        target: *mut cairo_surface_t,
        target_clip: *const cairo_clip_t,
    ) -> cairo_status_t;
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
pub type uint8_t = __uint8_t;
pub type pixman_bool_t = libc::c_int;
pub type pixman_fixed_16_16_t = int32_t;
pub type pixman_fixed_t = pixman_fixed_16_16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_color {
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub alpha: uint16_t,
}
pub type pixman_color_t = pixman_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_point_fixed {
    pub x: pixman_fixed_t,
    pub y: pixman_fixed_t,
}
pub type pixman_point_fixed_t = pixman_point_fixed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_transform {
    pub matrix: [[pixman_fixed_t; 3]; 3],
}
pub type pixman_transform_t = pixman_transform;
pub type pixman_repeat_t = libc::c_uint;
pub const PIXMAN_REPEAT_REFLECT: pixman_repeat_t = 3;
pub const PIXMAN_REPEAT_PAD: pixman_repeat_t = 2;
pub const PIXMAN_REPEAT_NORMAL: pixman_repeat_t = 1;
pub const PIXMAN_REPEAT_NONE: pixman_repeat_t = 0;
pub type pixman_filter_t = libc::c_uint;
pub const PIXMAN_FILTER_SEPARABLE_CONVOLUTION: pixman_filter_t = 6;
pub const PIXMAN_FILTER_CONVOLUTION: pixman_filter_t = 5;
pub const PIXMAN_FILTER_BILINEAR: pixman_filter_t = 4;
pub const PIXMAN_FILTER_NEAREST: pixman_filter_t = 3;
pub const PIXMAN_FILTER_BEST: pixman_filter_t = 2;
pub const PIXMAN_FILTER_GOOD: pixman_filter_t = 1;
pub const PIXMAN_FILTER_FAST: pixman_filter_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_gradient_stop {
    pub x: pixman_fixed_t,
    pub color: pixman_color_t,
}
pub type pixman_gradient_stop_t = pixman_gradient_stop;
pub type pixman_image_destroy_func_t = Option::<
    unsafe extern "C" fn(*mut pixman_image_t, *mut libc::c_void) -> (),
>;
pub type cairo_fixed_16_16_t = int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_subsurface {
    pub base: cairo_surface_t,
    pub extents: cairo_rectangle_int_t,
    pub target: *mut cairo_surface_t,
    pub snapshot: *mut cairo_surface_t,
}
pub type cairo_surface_subsurface_t = _cairo_surface_subsurface;
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
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_image_source {
    pub base: cairo_surface_t,
    pub pixman_image: *mut pixman_image_t,
    #[bitfield(name = "is_opaque_solid", ty = "libc::c_uint", bits = "0..=0")]
    pub is_opaque_solid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type cairo_image_source_t = _cairo_image_source;
pub type kernel_t = libc::c_uint;
pub const KERNEL_TENT: kernel_t = 8;
pub const KERNEL_LANCZOS3_STRETCHED: kernel_t = 7;
pub const KERNEL_LANCZOS3: kernel_t = 6;
pub const KERNEL_CATMULL_ROM: kernel_t = 5;
pub const KERNEL_NOTCH: kernel_t = 4;
pub const KERNEL_MITCHELL: kernel_t = 3;
pub const KERNEL_LINEAR: kernel_t = 2;
pub const KERNEL_BOX: kernel_t = 1;
pub const KERNEL_IMPULSE: kernel_t = 0;
pub type kernel_func_t = Option::<
    unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_info_t {
    pub kernel: kernel_t,
    pub func: kernel_func_t,
    pub width: kernel_width_func_t,
}
pub type kernel_width_func_t = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raster_source_cleanup {
    pub pattern: *const cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
    pub image: *mut cairo_image_surface_t,
    pub image_extra: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acquire_source_cleanup {
    pub surface: *mut cairo_surface_t,
    pub image: *mut cairo_image_surface_t,
    pub image_extra: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy {
    pub base: cairo_surface_t,
    pub image: *mut cairo_surface_t,
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
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
unsafe extern "C" fn _cairo_fixed_16_16_from_double(
    mut d: libc::c_double,
) -> cairo_fixed_16_16_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
    u.d = d + 103079215104.0f64;
    return u.i[0 as libc::c_int as usize];
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
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_pattern_get_source(
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    return _cairo_surface_get_source((*pattern).surface, extents);
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
unsafe extern "C" fn _pixman_transparent_image() -> *mut pixman_image_t {
    return _pixman_image_for_color(_cairo_stock_color(CAIRO_STOCK_TRANSPARENT));
}
unsafe extern "C" fn _pixman_black_image() -> *mut pixman_image_t {
    return _pixman_image_for_color(_cairo_stock_color(CAIRO_STOCK_BLACK));
}
unsafe extern "C" fn _pixman_white_image() -> *mut pixman_image_t {
    return _pixman_image_for_color(_cairo_stock_color(CAIRO_STOCK_WHITE));
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_image_for_color(
    mut cairo_color: *const cairo_color_t,
) -> *mut pixman_image_t {
    let mut color: pixman_color_t = pixman_color_t {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let mut image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    color.red = (*cairo_color).red_short;
    color.green = (*cairo_color).green_short;
    color.blue = (*cairo_color).blue_short;
    color.alpha = (*cairo_color).alpha_short;
    image = pixman_image_create_solid_fill(&mut color);
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_reset_static_data() {}
unsafe extern "C" fn _pixman_image_for_gradient(
    mut pattern: *const cairo_gradient_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut ix: *mut libc::c_int,
    mut iy: *mut libc::c_int,
) -> *mut pixman_image_t {
    let mut pixman_image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut pixman_stops_static: [pixman_gradient_stop_t; 2] = [pixman_gradient_stop_t {
        x: 0,
        color: pixman_color_t {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        },
    }; 2];
    let mut pixman_stops: *mut pixman_gradient_stop_t = pixman_stops_static.as_mut_ptr();
    let mut pixman_transform: pixman_transform_t = pixman_transform_t {
        matrix: [[0; 3]; 3],
    };
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut extremes: [cairo_circle_double_t; 2] = [cairo_circle_double_t {
        center: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        radius: 0.,
    }; 2];
    let mut p1: pixman_point_fixed_t = pixman_point_fixed_t { x: 0, y: 0 };
    let mut p2: pixman_point_fixed_t = pixman_point_fixed_t { x: 0, y: 0 };
    let mut i: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*pattern).n_stops
        > (::std::mem::size_of::<[pixman_gradient_stop_t; 2]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<pixman_gradient_stop_t>() as libc::c_ulong,
            ) as libc::c_int as libc::c_uint
    {
        pixman_stops = _cairo_malloc_ab(
            (*pattern).n_stops as size_t,
            ::std::mem::size_of::<pixman_gradient_stop_t>() as libc::c_ulong,
        ) as *mut pixman_gradient_stop_t;
        if pixman_stops.is_null() {
            return 0 as *mut pixman_image_t;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*pattern).n_stops {
        (*pixman_stops.offset(i as isize))
            .x = _cairo_fixed_16_16_from_double(
            (*((*pattern).stops).offset(i as isize)).offset,
        );
        (*pixman_stops.offset(i as isize))
            .color
            .red = (*((*pattern).stops).offset(i as isize)).color.red_short;
        (*pixman_stops.offset(i as isize))
            .color
            .green = (*((*pattern).stops).offset(i as isize)).color.green_short;
        (*pixman_stops.offset(i as isize))
            .color
            .blue = (*((*pattern).stops).offset(i as isize)).color.blue_short;
        (*pixman_stops.offset(i as isize))
            .color
            .alpha = (*((*pattern).stops).offset(i as isize)).color.alpha_short;
        i = i.wrapping_add(1);
    }
    _cairo_gradient_pattern_fit_to_range(
        pattern,
        ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
            >> 1 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
            as libc::c_double,
        &mut matrix,
        extremes.as_mut_ptr(),
    );
    p1.x = _cairo_fixed_16_16_from_double(extremes[0 as libc::c_int as usize].center.x);
    p1.y = _cairo_fixed_16_16_from_double(extremes[0 as libc::c_int as usize].center.y);
    p2.x = _cairo_fixed_16_16_from_double(extremes[1 as libc::c_int as usize].center.x);
    p2.y = _cairo_fixed_16_16_from_double(extremes[1 as libc::c_int as usize].center.y);
    if (*pattern).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        pixman_image = pixman_image_create_linear_gradient(
            &mut p1,
            &mut p2,
            pixman_stops,
            (*pattern).n_stops as libc::c_int,
        );
    } else {
        let mut r1: pixman_fixed_t = 0;
        let mut r2: pixman_fixed_t = 0;
        r1 = _cairo_fixed_16_16_from_double(extremes[0 as libc::c_int as usize].radius);
        r2 = _cairo_fixed_16_16_from_double(extremes[1 as libc::c_int as usize].radius);
        pixman_image = pixman_image_create_radial_gradient(
            &mut p1,
            &mut p2,
            r1,
            r2,
            pixman_stops,
            (*pattern).n_stops as libc::c_int,
        );
    }
    if pixman_stops != pixman_stops_static.as_mut_ptr() {
        free(pixman_stops as *mut libc::c_void);
    }
    if pixman_image.is_null() {
        return 0 as *mut pixman_image_t;
    }
    *iy = 0 as libc::c_int;
    *ix = *iy;
    status = _cairo_matrix_to_pixman_matrix_offset(
        &mut matrix,
        (*pattern).base.filter,
        (*extents).x as libc::c_double + (*extents).width as libc::c_double / 2.0f64,
        (*extents).y as libc::c_double + (*extents).height as libc::c_double / 2.0f64,
        &mut pixman_transform,
        ix,
        iy,
    ) as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        if status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            || pixman_image_set_transform(pixman_image, &mut pixman_transform) == 0
        {
            pixman_image_unref(pixman_image);
            return 0 as *mut pixman_image_t;
        }
    }
    let mut pixman_repeat: pixman_repeat_t = PIXMAN_REPEAT_NONE;
    match (*pattern).base.extend as libc::c_uint {
        1 => {
            pixman_repeat = PIXMAN_REPEAT_NORMAL;
        }
        2 => {
            pixman_repeat = PIXMAN_REPEAT_REFLECT;
        }
        3 => {
            pixman_repeat = PIXMAN_REPEAT_PAD;
        }
        0 | _ => {
            pixman_repeat = PIXMAN_REPEAT_NONE;
        }
    }
    pixman_image_set_repeat(pixman_image, pixman_repeat);
    return pixman_image;
}
unsafe extern "C" fn _pixman_image_for_mesh(
    mut pattern: *const cairo_mesh_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut tx: *mut libc::c_int,
    mut ty: *mut libc::c_int,
) -> *mut pixman_image_t {
    let mut image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    *tx = -(*extents).x;
    *ty = -(*extents).y;
    width = (*extents).width;
    height = (*extents).height;
    image = pixman_image_create_bits(
        PIXMAN_a8r8g8b8,
        width,
        height,
        0 as *mut uint32_t,
        0 as libc::c_int,
    );
    if image.is_null() {
        return 0 as *mut pixman_image_t;
    }
    _cairo_mesh_pattern_rasterize(
        pattern,
        pixman_image_get_data(image) as *mut libc::c_void,
        width,
        height,
        pixman_image_get_stride(image),
        *tx as libc::c_double,
        *ty as libc::c_double,
    );
    return image;
}
unsafe extern "C" fn _acquire_source_cleanup(
    mut pixman_image: *mut pixman_image_t,
    mut closure: *mut libc::c_void,
) {
    let mut data: *mut acquire_source_cleanup = closure as *mut acquire_source_cleanup;
    _cairo_surface_release_source_image(
        (*data).surface,
        (*data).image,
        (*data).image_extra,
    );
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn _defer_free_cleanup(
    mut pixman_image: *mut pixman_image_t,
    mut closure: *mut libc::c_void,
) {
    cairo_surface_destroy(closure as *mut cairo_surface_t);
}
unsafe extern "C" fn expand_channel(mut v: uint16_t, mut bits: uint32_t) -> uint16_t {
    let mut offset: libc::c_int = (16 as libc::c_int as libc::c_uint).wrapping_sub(bits)
        as libc::c_int;
    while offset > 0 as libc::c_int {
        v = (v as libc::c_int | v as libc::c_int >> bits) as uint16_t;
        offset = (offset as libc::c_uint).wrapping_sub(bits) as libc::c_int
            as libc::c_int;
        bits = (bits as libc::c_uint).wrapping_add(bits) as uint32_t as uint32_t;
    }
    return v;
}
unsafe extern "C" fn _pixel_to_solid(
    mut image: *mut cairo_image_surface_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut pixman_image_t {
    let mut pixel: uint32_t = 0;
    let mut rgba: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut color: pixman_color_t = pixman_color_t {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    match (*image).format as libc::c_int {
        3 => {
            pixel = *(((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize)
                .offset((x / 8 as libc::c_int) as isize) as *mut uint8_t) as uint32_t;
            return if pixel
                & ((1 as libc::c_int) << (x & 7 as libc::c_int)) as libc::c_uint != 0
            {
                _pixman_black_image()
            } else {
                _pixman_transparent_image()
            };
        }
        2 => {
            color
                .alpha = *(((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize)
                .offset(x as isize) as *mut uint8_t) as uint16_t;
            color
                .alpha = (color.alpha as libc::c_int
                | (color.alpha as libc::c_int) << 8 as libc::c_int) as uint16_t;
            if color.alpha as libc::c_int == 0 as libc::c_int {
                return _pixman_transparent_image();
            }
            if color.alpha as libc::c_int == 0xffff as libc::c_int {
                return _pixman_black_image();
            }
            color.blue = 0 as libc::c_int as uint16_t;
            color.green = color.blue;
            color.red = color.green;
            return pixman_image_create_solid_fill(&mut color);
        }
        4 => {
            pixel = *(((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize)
                .offset((2 as libc::c_int * x) as isize) as *mut uint16_t) as uint32_t;
            if pixel == 0 as libc::c_int as libc::c_uint {
                return _pixman_black_image();
            }
            if pixel == 0xffff as libc::c_int as libc::c_uint {
                return _pixman_white_image();
            }
            color.alpha = 0xffff as libc::c_int as uint16_t;
            color
                .red = expand_channel(
                ((pixel >> 11 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
                    << 11 as libc::c_int) as uint16_t,
                5 as libc::c_int as uint32_t,
            );
            color
                .green = expand_channel(
                ((pixel >> 5 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                    << 10 as libc::c_int) as uint16_t,
                6 as libc::c_int as uint32_t,
            );
            color
                .blue = expand_channel(
                ((pixel & 0x1f as libc::c_int as libc::c_uint) << 11 as libc::c_int)
                    as uint16_t,
                5 as libc::c_int as uint32_t,
            );
            return pixman_image_create_solid_fill(&mut color);
        }
        5 => {
            pixel = *(((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize)
                .offset((4 as libc::c_int * x) as isize) as *mut uint32_t);
            pixel &= 0x3fffffff as libc::c_int as libc::c_uint;
            if pixel == 0 as libc::c_int as libc::c_uint {
                return _pixman_black_image();
            }
            if pixel == 0x3fffffff as libc::c_int as libc::c_uint {
                return _pixman_white_image();
            }
            color.alpha = 0xffff as libc::c_int as uint16_t;
            color
                .red = expand_channel(
                (pixel >> 20 as libc::c_int & 0x3fff as libc::c_int as libc::c_uint)
                    as uint16_t,
                10 as libc::c_int as uint32_t,
            );
            color
                .green = expand_channel(
                (pixel >> 10 as libc::c_int & 0x3fff as libc::c_int as libc::c_uint)
                    as uint16_t,
                10 as libc::c_int as uint32_t,
            );
            color
                .blue = expand_channel(
                (pixel & 0x3fff as libc::c_int as libc::c_uint) as uint16_t,
                10 as libc::c_int as uint32_t,
            );
            return pixman_image_create_solid_fill(&mut color);
        }
        0 | 1 => {
            pixel = *(((*image).data)
                .offset((y as libc::c_long * (*image).stride) as isize)
                .offset((4 as libc::c_int * x) as isize) as *mut uint32_t);
            color
                .alpha = (if (*image).format as libc::c_int
                == CAIRO_FORMAT_ARGB32 as libc::c_int
            {
                pixel >> 24 as libc::c_int
                    | pixel >> 16 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
            } else {
                0xffff as libc::c_int as libc::c_uint
            }) as uint16_t;
            if color.alpha as libc::c_int == 0 as libc::c_int {
                return _pixman_transparent_image();
            }
            if pixel == 0xffffffff as libc::c_uint {
                return _pixman_white_image();
            }
            if color.alpha as libc::c_int == 0xffff as libc::c_int
                && pixel & 0xffffff as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
            {
                return _pixman_black_image();
            }
            color
                .red = (pixel >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                | pixel >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint)
                as uint16_t;
            color
                .green = (pixel >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                | pixel & 0xff00 as libc::c_int as libc::c_uint) as uint16_t;
            color
                .blue = (pixel & 0xff as libc::c_int as libc::c_uint
                | pixel << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint)
                as uint16_t;
            return pixman_image_create_solid_fill(&mut color);
        }
        6 | 7 => {
            if (*image).format as libc::c_int == CAIRO_FORMAT_RGBA128F as libc::c_int {
                rgba = &mut *((*image).data)
                    .offset(
                        (y as libc::c_long * (*image).stride
                            + (16 as libc::c_int * x) as libc::c_long) as isize,
                    ) as *mut libc::c_uchar as *mut libc::c_float;
                color
                    .alpha = (65535.0f32 * *rgba.offset(3 as libc::c_int as isize))
                    as uint16_t;
                if color.alpha as libc::c_int == 0 as libc::c_int {
                    return _pixman_transparent_image();
                }
            } else {
                rgba = &mut *((*image).data)
                    .offset(
                        (y as libc::c_long * (*image).stride
                            + (12 as libc::c_int * x) as libc::c_long) as isize,
                    ) as *mut libc::c_uchar as *mut libc::c_float;
                color.alpha = 0xffff as libc::c_int as uint16_t;
            }
            if color.alpha as libc::c_int == 0xffff as libc::c_int
                && *rgba.offset(0 as libc::c_int as isize) == 0.0f32
                && *rgba.offset(1 as libc::c_int as isize) == 0.0f32
                && *rgba.offset(2 as libc::c_int as isize) == 0.0f32
            {
                return _pixman_black_image();
            }
            if color.alpha as libc::c_int == 0xffff as libc::c_int
                && *rgba.offset(0 as libc::c_int as isize) == 1.0f32
                && *rgba.offset(1 as libc::c_int as isize) == 1.0f32
                && *rgba.offset(2 as libc::c_int as isize) == 1.0f32
            {
                return _pixman_white_image();
            }
            color
                .red = (*rgba.offset(0 as libc::c_int as isize) * 65535.0f32)
                as uint16_t;
            color
                .green = (*rgba.offset(1 as libc::c_int as isize) * 65535.0f32)
                as uint16_t;
            color
                .blue = (*rgba.offset(2 as libc::c_int as isize) * 65535.0f32)
                as uint16_t;
            return pixman_image_create_solid_fill(&mut color);
        }
        -1 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
                    466 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 67],
                        &[libc::c_char; 67],
                    >(
                        b"pixman_image_t *_pixel_to_solid(cairo_image_surface_t *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            return 0 as *mut pixman_image_t;
        }
    };
}
unsafe extern "C" fn impulse_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return 1 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn impulse_width(mut r: libc::c_double) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn box_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return if 0.0f64
        > (if (if r < 1.0f64 { r } else { 1.0f64 })
            < (if (r + 1 as libc::c_int as libc::c_double)
                / 2 as libc::c_int as libc::c_double - x
                < (r + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double + x
            {
                (r + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double - x
            } else {
                (r + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double + x
            })
        {
            (if r < 1.0f64 { r } else { 1.0f64 })
        } else {
            (if (r + 1 as libc::c_int as libc::c_double)
                / 2 as libc::c_int as libc::c_double - x
                < (r + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double + x
            {
                (r + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double - x
            } else {
                (r + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double + x
            })
        })
    {
        0.0f64
    } else if (if r < 1.0f64 { r } else { 1.0f64 })
        < (if (r + 1 as libc::c_int as libc::c_double)
            / 2 as libc::c_int as libc::c_double - x
            < (r + 1 as libc::c_int as libc::c_double)
                / 2 as libc::c_int as libc::c_double + x
        {
            (r + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double
                - x
        } else {
            (r + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double
                + x
        })
    {
        if r < 1.0f64 { r } else { 1.0f64 }
    } else if (r + 1 as libc::c_int as libc::c_double)
        / 2 as libc::c_int as libc::c_double - x
        < (r + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double
            + x
    {
        (r + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double - x
    } else {
        (r + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double + x
    };
}
unsafe extern "C" fn box_width(mut r: libc::c_double) -> libc::c_int {
    return (if r < 1.0f64 {
        2 as libc::c_int as libc::c_double
    } else {
        ceil(r + 1 as libc::c_int as libc::c_double)
    }) as libc::c_int;
}
unsafe extern "C" fn linear_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return if 1.0f64 - fabs(x) > 0.0f64 { 1.0f64 - fabs(x) } else { 0.0f64 };
}
unsafe extern "C" fn linear_width(mut r: libc::c_double) -> libc::c_int {
    return 2 as libc::c_int;
}
unsafe extern "C" fn general_cubic(
    mut x: libc::c_double,
    mut r: libc::c_double,
    mut B: libc::c_double,
    mut C: libc::c_double,
) -> libc::c_double {
    let mut ax: libc::c_double = 0.;
    if r < 1.0f64 {
        return general_cubic(
            x * 2 as libc::c_int as libc::c_double - 0.5f64,
            r * 2 as libc::c_int as libc::c_double,
            B,
            C,
        )
            + general_cubic(
                x * 2 as libc::c_int as libc::c_double + 0.5f64,
                r * 2 as libc::c_int as libc::c_double,
                B,
                C,
            );
    }
    ax = fabs(x / r);
    if ax < 1 as libc::c_int as libc::c_double {
        return (((12 as libc::c_int as libc::c_double
            - 9 as libc::c_int as libc::c_double * B
            - 6 as libc::c_int as libc::c_double * C) * ax
            + (-(18 as libc::c_int) as libc::c_double
                + 12 as libc::c_int as libc::c_double * B
                + 6 as libc::c_int as libc::c_double * C)) * ax * ax
            + (6 as libc::c_int as libc::c_double
                - 2 as libc::c_int as libc::c_double * B))
            / 6 as libc::c_int as libc::c_double
    } else if ax < 2 as libc::c_int as libc::c_double {
        return ((((-B - 6 as libc::c_int as libc::c_double * C) * ax
            + (6 as libc::c_int as libc::c_double * B
                + 30 as libc::c_int as libc::c_double * C)) * ax
            + (-(12 as libc::c_int) as libc::c_double * B
                - 48 as libc::c_int as libc::c_double * C)) * ax
            + (8 as libc::c_int as libc::c_double * B
                + 24 as libc::c_int as libc::c_double * C))
            / 6 as libc::c_int as libc::c_double
    } else {
        return 0.0f64
    };
}
unsafe extern "C" fn cubic_width(mut r: libc::c_double) -> libc::c_int {
    return (if 2 as libc::c_int as libc::c_double
        > ceil(r * 4 as libc::c_int as libc::c_double)
    {
        2 as libc::c_int as libc::c_double
    } else {
        ceil(r * 4 as libc::c_int as libc::c_double)
    }) as libc::c_int;
}
unsafe extern "C" fn cubic_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return general_cubic(x, r, 0.0f64, 0.5f64);
}
unsafe extern "C" fn mitchell_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return general_cubic(
        x,
        r,
        1 as libc::c_int as libc::c_double / 3.0f64,
        1 as libc::c_int as libc::c_double / 3.0f64,
    );
}
unsafe extern "C" fn notch_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return general_cubic(x, r, 1.5f64, -0.25f64);
}
unsafe extern "C" fn sinc(mut x: libc::c_double) -> libc::c_double {
    return if x != 0. {
        sin(3.14159265358979323846f64 * x) / (3.14159265358979323846f64 * x)
    } else {
        1.0f64
    };
}
unsafe extern "C" fn lanczos(
    mut x: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    return if fabs(x) < n { sinc(x) * sinc(x * (1.0f64 / n)) } else { 0.0f64 };
}
unsafe extern "C" fn lanczos3_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    if r < 1.0f64 {
        return lanczos3_kernel(
            x * 2 as libc::c_int as libc::c_double - 0.5f64,
            r * 2 as libc::c_int as libc::c_double,
        )
            + lanczos3_kernel(
                x * 2 as libc::c_int as libc::c_double + 0.5f64,
                r * 2 as libc::c_int as libc::c_double,
            )
    } else {
        return lanczos(x / r, 3.0f64)
    };
}
unsafe extern "C" fn lanczos3_width(mut r: libc::c_double) -> libc::c_int {
    return (if 2 as libc::c_int as libc::c_double
        > ceil(r * 6 as libc::c_int as libc::c_double)
    {
        2 as libc::c_int as libc::c_double
    } else {
        ceil(r * 6 as libc::c_int as libc::c_double)
    }) as libc::c_int;
}
unsafe extern "C" fn nice_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    return lanczos3_kernel(x, r * (4.0f64 / 3 as libc::c_int as libc::c_double));
}
unsafe extern "C" fn nice_width(mut r: libc::c_double) -> libc::c_int {
    return (if 2.0f64 > ceil(r * 8 as libc::c_int as libc::c_double) {
        2.0f64
    } else {
        ceil(r * 8 as libc::c_int as libc::c_double)
    }) as libc::c_int;
}
unsafe extern "C" fn tent_kernel(
    mut x: libc::c_double,
    mut r: libc::c_double,
) -> libc::c_double {
    if r < 1.0f64 {
        return box_kernel(x, r)
    } else {
        return if 1.0f64 - fabs(x / r) > 0.0f64 { 1.0f64 - fabs(x / r) } else { 0.0f64 }
    };
}
unsafe extern "C" fn tent_width(mut r: libc::c_double) -> libc::c_int {
    return (if r < 1.0f64 {
        2 as libc::c_int as libc::c_double
    } else {
        ceil(2 as libc::c_int as libc::c_double * r)
    }) as libc::c_int;
}
static mut filters: [filter_info_t; 9] = unsafe {
    [
        {
            let mut init = filter_info_t {
                kernel: KERNEL_IMPULSE,
                func: Some(
                    impulse_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    impulse_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_BOX,
                func: Some(
                    box_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    box_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_LINEAR,
                func: Some(
                    linear_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    linear_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_MITCHELL,
                func: Some(
                    mitchell_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    cubic_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_NOTCH,
                func: Some(
                    notch_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    cubic_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_CATMULL_ROM,
                func: Some(
                    cubic_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    cubic_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_LANCZOS3,
                func: Some(
                    lanczos3_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    lanczos3_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_LANCZOS3_STRETCHED,
                func: Some(
                    nice_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    nice_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = filter_info_t {
                kernel: KERNEL_TENT,
                func: Some(
                    tent_kernel
                        as unsafe extern "C" fn(
                            libc::c_double,
                            libc::c_double,
                        ) -> libc::c_double,
                ),
                width: Some(
                    tent_width as unsafe extern "C" fn(libc::c_double) -> libc::c_int,
                ),
            };
            init
        },
    ]
};
unsafe extern "C" fn get_filter(
    mut filter: kernel_t,
    mut r: libc::c_double,
    mut width: libc::c_int,
    mut subsample: libc::c_int,
    mut out: *mut pixman_fixed_t,
) {
    let mut i: libc::c_int = 0;
    let mut p: *mut pixman_fixed_t = out;
    let mut n_phases: libc::c_int = (1 as libc::c_int) << subsample;
    let mut step: libc::c_double = 1.0f64 / n_phases as libc::c_double;
    let mut func: kernel_func_t = filters[filter as usize].func;
    if width <= 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n_phases {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = ((1 as libc::c_int as uint32_t) << 16 as libc::c_int)
                as pixman_fixed_t;
            i += 1;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < n_phases {
        let mut frac: libc::c_double = (i as libc::c_double + 0.5f64) * step;
        let mut x1: libc::c_double = ceil(
            frac - width as libc::c_double / 2.0f64 - 0.5f64,
        ) - frac + 0.5f64;
        let mut total: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut new_total: pixman_fixed_t = 0 as libc::c_int;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < width {
            let mut v: libc::c_double = func
                .expect("non-null function pointer")(x1 + j as libc::c_double, r);
            total += v;
            *p.offset(j as isize) = (v * 65536.0f64) as pixman_fixed_t;
            j += 1;
        }
        total = 1 as libc::c_int as libc::c_double / total;
        j = 0 as libc::c_int;
        while j < width {
            let ref mut fresh3 = *p.offset(j as isize);
            *fresh3 = (*fresh3 as libc::c_double * total) as pixman_fixed_t;
            new_total += *fresh3;
            j += 1;
        }
        let ref mut fresh4 = *p.offset((width / 2 as libc::c_int) as isize);
        *fresh4
            += ((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
                - new_total;
        p = p.offset(width as isize);
        i += 1;
    }
}
unsafe extern "C" fn create_separable_convolution(
    mut n_values: *mut libc::c_int,
    mut xfilter: kernel_t,
    mut sx: libc::c_double,
    mut yfilter: kernel_t,
    mut sy: libc::c_double,
) -> *mut pixman_fixed_t {
    let mut xwidth: libc::c_int = 0;
    let mut xsubsample: libc::c_int = 0;
    let mut ywidth: libc::c_int = 0;
    let mut ysubsample: libc::c_int = 0;
    let mut size_x: libc::c_int = 0;
    let mut size_y: libc::c_int = 0;
    let mut params: *mut pixman_fixed_t = 0 as *mut pixman_fixed_t;
    xwidth = (filters[xfilter as usize].width).expect("non-null function pointer")(sx);
    xsubsample = 0 as libc::c_int;
    if xwidth > 1 as libc::c_int {
        while sx * ((1 as libc::c_int) << xsubsample) as libc::c_double <= 128.0f64 {
            xsubsample += 1;
        }
    }
    size_x = ((1 as libc::c_int) << xsubsample) * xwidth;
    ywidth = (filters[yfilter as usize].width).expect("non-null function pointer")(sy);
    ysubsample = 0 as libc::c_int;
    if ywidth > 1 as libc::c_int {
        while sy * ((1 as libc::c_int) << ysubsample) as libc::c_double <= 128.0f64 {
            ysubsample += 1;
        }
    }
    size_y = ((1 as libc::c_int) << ysubsample) * ywidth;
    *n_values = 4 as libc::c_int + size_x + size_y;
    params = (if (*n_values as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pixman_fixed_t>() as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(
            (*n_values as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pixman_fixed_t>() as libc::c_ulong),
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut pixman_fixed_t;
    if params.is_null() {
        return 0 as *mut pixman_fixed_t;
    }
    *params
        .offset(
            0 as libc::c_int as isize,
        ) = ((xwidth as uint32_t) << 16 as libc::c_int) as pixman_fixed_t;
    *params
        .offset(
            1 as libc::c_int as isize,
        ) = ((ywidth as uint32_t) << 16 as libc::c_int) as pixman_fixed_t;
    *params
        .offset(
            2 as libc::c_int as isize,
        ) = ((xsubsample as uint32_t) << 16 as libc::c_int) as pixman_fixed_t;
    *params
        .offset(
            3 as libc::c_int as isize,
        ) = ((ysubsample as uint32_t) << 16 as libc::c_int) as pixman_fixed_t;
    get_filter(
        xfilter,
        sx,
        xwidth,
        xsubsample,
        params.offset(4 as libc::c_int as isize),
    );
    get_filter(
        yfilter,
        sy,
        ywidth,
        ysubsample,
        params.offset(4 as libc::c_int as isize).offset(size_x as isize),
    );
    return params;
}
unsafe extern "C" fn _pixman_image_set_properties(
    mut pixman_image: *mut pixman_image_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut ix: *mut libc::c_int,
    mut iy: *mut libc::c_int,
) -> cairo_bool_t {
    let mut pixman_transform: pixman_transform_t = pixman_transform_t {
        matrix: [[0; 3]; 3],
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_matrix_to_pixman_matrix_offset(
        &(*pattern).matrix,
        (*pattern).filter,
        (*extents).x as libc::c_double + (*extents).width as libc::c_double / 2.0f64,
        (*extents).y as libc::c_double + (*extents).height as libc::c_double / 2.0f64,
        &mut pixman_transform,
        ix,
        iy,
    ) as cairo_int_status_t;
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        pixman_image_set_filter(
            pixman_image,
            PIXMAN_FILTER_NEAREST,
            0 as *const pixman_fixed_t,
            0 as libc::c_int,
        );
    } else if status as libc::c_uint
        != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        || pixman_image_set_transform(pixman_image, &mut pixman_transform) == 0
    {
        return 0 as libc::c_int
    } else {
        let mut pixman_filter: pixman_filter_t = PIXMAN_FILTER_FAST;
        let mut kernel: kernel_t = KERNEL_IMPULSE;
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        dx = hypot((*pattern).matrix.xx, (*pattern).matrix.xy);
        dy = hypot((*pattern).matrix.yx, (*pattern).matrix.yy);
        if !(dx < 0x7fff as libc::c_int as libc::c_double) {
            dx = 0x7fff as libc::c_int as libc::c_double;
        }
        if !(dy < 0x7fff as libc::c_int as libc::c_double) {
            dy = 0x7fff as libc::c_int as libc::c_double;
        }
        match (*pattern).filter as libc::c_uint {
            0 => {
                pixman_filter = PIXMAN_FILTER_FAST;
            }
            1 => {
                pixman_filter = PIXMAN_FILTER_SEPARABLE_CONVOLUTION;
                kernel = KERNEL_BOX;
                if dx > 16.0f64 {
                    dx = 16.0f64;
                }
                if dy > 16.0f64 {
                    dy = 16.0f64;
                }
                if dx < 1.0f64 / 0.75f64 {
                    dx = 1.0f64;
                }
                if dy < 1.0f64 / 0.75f64 {
                    dy = 1.0f64;
                }
            }
            2 => {
                pixman_filter = PIXMAN_FILTER_SEPARABLE_CONVOLUTION;
                kernel = KERNEL_CATMULL_ROM;
                if dx > 16.0f64 {
                    dx = 16.0f64;
                    kernel = KERNEL_BOX;
                } else if dx < 1.0f64 {
                    if dx < 1.0f64 / 128 as libc::c_int as libc::c_double {
                        dx = 1.0f64 / 127 as libc::c_int as libc::c_double;
                    } else if dx < 0.5f64 {
                        dx = 1.0f64 / (1.0f64 / dx - 1.0f64);
                    } else {
                        dx = 1.0f64;
                    }
                }
                if dy > 16.0f64 {
                    dy = 16.0f64;
                    kernel = KERNEL_BOX;
                } else if dy < 1.0f64 {
                    if dy < 1.0f64 / 128 as libc::c_int as libc::c_double {
                        dy = 1.0f64 / 127 as libc::c_int as libc::c_double;
                    } else if dy < 0.5f64 {
                        dy = 1.0f64 / (1.0f64 / dy - 1.0f64);
                    } else {
                        dy = 1.0f64;
                    }
                }
            }
            3 => {
                pixman_filter = PIXMAN_FILTER_NEAREST;
            }
            4 => {
                pixman_filter = PIXMAN_FILTER_BILINEAR;
            }
            5 | _ => {
                pixman_filter = PIXMAN_FILTER_BEST;
            }
        }
        if pixman_filter as libc::c_uint
            == PIXMAN_FILTER_SEPARABLE_CONVOLUTION as libc::c_int as libc::c_uint
        {
            let mut n_params: libc::c_int = 0;
            let mut params: *mut pixman_fixed_t = 0 as *mut pixman_fixed_t;
            params = create_separable_convolution(&mut n_params, kernel, dx, kernel, dy);
            pixman_image_set_filter(pixman_image, pixman_filter, params, n_params);
            free(params as *mut libc::c_void);
        } else {
            pixman_image_set_filter(
                pixman_image,
                pixman_filter,
                0 as *const pixman_fixed_t,
                0 as libc::c_int,
            );
        }
    }
    let mut pixman_repeat: pixman_repeat_t = PIXMAN_REPEAT_NONE;
    match (*pattern).extend as libc::c_uint {
        1 => {
            pixman_repeat = PIXMAN_REPEAT_NORMAL;
        }
        2 => {
            pixman_repeat = PIXMAN_REPEAT_REFLECT;
        }
        3 => {
            pixman_repeat = PIXMAN_REPEAT_PAD;
        }
        0 | _ => {
            pixman_repeat = PIXMAN_REPEAT_NONE;
        }
    }
    pixman_image_set_repeat(pixman_image, pixman_repeat);
    if (*pattern).has_component_alpha != 0 {
        pixman_image_set_component_alpha(pixman_image, 1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn proxy_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut proxy: *mut proxy = abstract_surface as *mut proxy;
    return _cairo_surface_acquire_source_image((*proxy).image, image_out, image_extra);
}
unsafe extern "C" fn proxy_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    let mut proxy: *mut proxy = abstract_surface as *mut proxy;
    _cairo_surface_release_source_image((*proxy).image, image, image_extra);
}
unsafe extern "C" fn proxy_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    return CAIRO_STATUS_SUCCESS;
}
static mut proxy_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_INTERNAL_SURFACE_TYPE_NULL as libc::c_int
                as cairo_surface_type_t,
            finish: Some(
                proxy_finish as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: None,
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
            acquire_source_image: Some(
                proxy_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                proxy_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: None,
            copy_page: None,
            show_page: None,
            get_extents: None,
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            fill_stroke: None,
            show_glyphs: None,
            has_show_text_glyphs: None,
            show_text_glyphs: None,
            get_supported_mime_types: None,
            tag: None,
        };
        init
    }
};
unsafe extern "C" fn attach_proxy(
    mut source: *mut cairo_surface_t,
    mut image: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut proxy: *mut proxy = 0 as *mut proxy;
    proxy = (if ::std::mem::size_of::<proxy>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<proxy>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut proxy;
    if proxy.is_null() {
        return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_surface_init(
        &mut (*proxy).base,
        &proxy_backend,
        0 as *mut cairo_device_t,
        (*image).content,
        0 as libc::c_int,
    );
    let ref mut fresh5 = (*proxy).image;
    *fresh5 = image;
    _cairo_surface_attach_snapshot(source, &mut (*proxy).base, None);
    return &mut (*proxy).base;
}
unsafe extern "C" fn detach_proxy(
    mut source: *mut cairo_surface_t,
    mut proxy: *mut cairo_surface_t,
) {
    cairo_surface_finish(proxy);
    cairo_surface_destroy(proxy);
}
unsafe extern "C" fn get_proxy(mut proxy: *mut cairo_surface_t) -> *mut cairo_surface_t {
    return (*(proxy as *mut proxy)).image;
}
unsafe extern "C" fn _pixman_image_for_recording(
    mut dst: *mut cairo_image_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut ix: *mut libc::c_int,
    mut iy: *mut libc::c_int,
) -> *mut pixman_image_t {
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut clone: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut proxy: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut limit: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut src_limit: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut pixman_image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut extend: cairo_extend_t = CAIRO_EXTEND_NONE;
    let mut m: *mut cairo_matrix_t = 0 as *mut cairo_matrix_t;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut sx: libc::c_double = 1.0f64;
    let mut sy: libc::c_double = 1.0f64;
    let mut tx: libc::c_int = 0 as libc::c_int;
    let mut ty: libc::c_int = 0 as libc::c_int;
    *iy = 0 as libc::c_int;
    *ix = *iy;
    source = _cairo_pattern_get_source(pattern, &mut limit);
    src_limit = limit;
    extend = (*pattern).base.extend;
    if _cairo_rectangle_contains_rectangle(&mut limit, sample) != 0 {
        extend = CAIRO_EXTEND_NONE;
    }
    if extend as libc::c_uint == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint {
        if _cairo_rectangle_intersect(&mut limit, sample) == 0 {
            return _pixman_transparent_image();
        }
    }
    if _cairo_matrix_is_identity(&(*pattern).base.matrix) == 0 {
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        matrix = (*pattern).base.matrix;
        status = cairo_matrix_invert(&mut matrix);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
                1172 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"pixman_image_t *_pixman_image_for_recording(cairo_image_surface_t *, const cairo_surface_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, const cairo_rectangle_int_t *, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        x1 = limit.x as libc::c_double;
        y1 = limit.y as libc::c_double;
        x2 = (limit.x + limit.width) as libc::c_double;
        y2 = (limit.y + limit.height) as libc::c_double;
        _cairo_matrix_transform_bounding_box(
            &mut matrix,
            &mut x1,
            &mut y1,
            &mut x2,
            &mut y2,
            0 as *mut cairo_bool_t,
        );
        limit.x = floor(x1) as libc::c_int;
        limit.y = floor(y1) as libc::c_int;
        limit.width = (ceil(x2) - limit.x as libc::c_double) as libc::c_int;
        limit.height = (ceil(y2) - limit.y as libc::c_double) as libc::c_int;
        sx = src_limit.width as libc::c_double / limit.width as libc::c_double;
        sy = src_limit.height as libc::c_double / limit.height as libc::c_double;
    }
    tx = limit.x;
    ty = limit.y;
    proxy = _cairo_surface_has_snapshot(source, &proxy_backend);
    if !proxy.is_null() {
        clone = cairo_surface_reference(get_proxy(proxy));
    } else {
        if is_mask != 0 {
            clone = cairo_image_surface_create(
                CAIRO_FORMAT_A8,
                limit.width,
                limit.height,
            );
        } else if (*dst).base.content as libc::c_uint
            == (*source).content as libc::c_uint
        {
            clone = cairo_image_surface_create((*dst).format, limit.width, limit.height);
        } else {
            clone = _cairo_image_surface_create_with_content(
                (*source).content,
                limit.width,
                limit.height,
            );
        }
        m = 0 as *mut cairo_matrix_t;
        if extend as libc::c_uint == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint {
            matrix = (*pattern).base.matrix;
            if tx | ty != 0 {
                cairo_matrix_translate(
                    &mut matrix,
                    tx as libc::c_double,
                    ty as libc::c_double,
                );
            }
            m = &mut matrix;
        } else {
            cairo_matrix_init_scale(&mut matrix, sx, sy);
            cairo_matrix_translate(
                &mut matrix,
                src_limit.x as libc::c_double / sx,
                src_limit.y as libc::c_double / sy,
            );
            m = &mut matrix;
        }
        proxy = attach_proxy(source, clone);
        status = _cairo_recording_surface_replay_with_clip(
            source,
            m,
            clone,
            0 as *const cairo_clip_t,
        );
        detach_proxy(source, proxy);
        if status as u64 != 0 {
            cairo_surface_destroy(clone);
            return 0 as *mut pixman_image_t;
        }
    }
    pixman_image = pixman_image_ref(
        (*(clone as *mut cairo_image_surface_t)).pixman_image,
    );
    cairo_surface_destroy(clone);
    if extend as libc::c_uint == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint {
        *ix = -limit.x;
        *iy = -limit.y;
    } else {
        let mut tmp_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
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
        _cairo_pattern_init_static_copy(&mut tmp_pattern.base, &(*pattern).base);
        matrix = (*pattern).base.matrix;
        status = cairo_matrix_invert(&mut matrix);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
                1245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"pixman_image_t *_pixman_image_for_recording(cairo_image_surface_t *, const cairo_surface_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, const cairo_rectangle_int_t *, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        cairo_matrix_translate(
            &mut matrix,
            src_limit.x as libc::c_double,
            src_limit.y as libc::c_double,
        );
        cairo_matrix_scale(&mut matrix, sx, sy);
        status = cairo_matrix_invert(&mut matrix);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
                1249 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 192],
                    &[libc::c_char; 192],
                >(
                    b"pixman_image_t *_pixman_image_for_recording(cairo_image_surface_t *, const cairo_surface_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, const cairo_rectangle_int_t *, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        cairo_pattern_set_matrix(&mut tmp_pattern.base, &mut matrix);
        if _pixman_image_set_properties(
            pixman_image,
            &mut tmp_pattern.base,
            extents,
            ix,
            iy,
        ) == 0
        {
            pixman_image_unref(pixman_image);
            pixman_image = 0 as *mut pixman_image_t;
        }
    }
    return pixman_image;
}
unsafe extern "C" fn _pixman_image_for_surface(
    mut dst: *mut cairo_image_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut ix: *mut libc::c_int,
    mut iy: *mut libc::c_int,
) -> *mut pixman_image_t {
    let mut extend: cairo_extend_t = (*pattern).base.extend;
    let mut pixman_image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    *iy = 0 as libc::c_int;
    *ix = *iy;
    pixman_image = 0 as *mut pixman_image_t;
    if (*(*pattern).surface).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return _pixman_image_for_recording(
            dst,
            pattern,
            is_mask,
            extents,
            sample,
            ix,
            iy,
        );
    }
    if (*(*pattern).surface).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        && (is_mask == 0 || (*pattern).base.has_component_alpha == 0
            || (*(*pattern).surface).content as libc::c_uint
                & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint)
    {
        let mut defer_free: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        let mut source: *mut cairo_image_surface_t = (*pattern).surface
            as *mut cairo_image_surface_t;
        let mut type_0: cairo_surface_type_t = CAIRO_SURFACE_TYPE_IMAGE;
        if _cairo_surface_is_snapshot(&mut (*source).base) != 0 {
            defer_free = _cairo_surface_snapshot_get_target(&mut (*source).base);
            source = defer_free as *mut cairo_image_surface_t;
        }
        type_0 = (*(*source).base.backend).type_0;
        if type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        {
            if extend as libc::c_uint != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                && (*sample).x >= 0 as libc::c_int && (*sample).y >= 0 as libc::c_int
                && (*sample).x + (*sample).width <= (*source).width
                && (*sample).y + (*sample).height <= (*source).height
            {
                extend = CAIRO_EXTEND_NONE;
            }
            if (*sample).width == 1 as libc::c_int
                && (*sample).height == 1 as libc::c_int
            {
                if (*sample).x < 0 as libc::c_int || (*sample).y < 0 as libc::c_int
                    || (*sample).x >= (*source).width || (*sample).y >= (*source).height
                {
                    if extend as libc::c_uint
                        == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                    {
                        cairo_surface_destroy(defer_free);
                        return _pixman_transparent_image();
                    }
                } else {
                    pixman_image = _pixel_to_solid(source, (*sample).x, (*sample).y);
                    if !pixman_image.is_null() {
                        cairo_surface_destroy(defer_free);
                        return pixman_image;
                    }
                }
            }
            pixman_image = pixman_image_create_bits(
                (*source).pixman_format,
                (*source).width,
                (*source).height,
                (*source).data as *mut uint32_t,
                (*source).stride as libc::c_int,
            );
            if pixman_image.is_null() {
                cairo_surface_destroy(defer_free);
                return 0 as *mut pixman_image_t;
            }
            if !defer_free.is_null() {
                pixman_image_set_destroy_function(
                    pixman_image,
                    Some(
                        _defer_free_cleanup
                            as unsafe extern "C" fn(
                                *mut pixman_image_t,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    defer_free as *mut libc::c_void,
                );
            }
        } else if type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
        {
            let mut sub: *mut cairo_surface_subsurface_t = 0
                as *mut cairo_surface_subsurface_t;
            let mut is_contained: cairo_bool_t = 0 as libc::c_int;
            sub = source as *mut cairo_surface_subsurface_t;
            source = (*sub).target as *mut cairo_image_surface_t;
            if (*sample).x >= 0 as libc::c_int && (*sample).y >= 0 as libc::c_int
                && (*sample).x + (*sample).width <= (*sub).extents.width
                && (*sample).y + (*sample).height <= (*sub).extents.height
            {
                is_contained = 1 as libc::c_int;
            }
            if (*sample).width == 1 as libc::c_int
                && (*sample).height == 1 as libc::c_int
            {
                if is_contained != 0 {
                    pixman_image = _pixel_to_solid(
                        source,
                        (*sub).extents.x + (*sample).x,
                        (*sub).extents.y + (*sample).y,
                    );
                    if !pixman_image.is_null() {
                        return pixman_image;
                    }
                } else if extend as libc::c_uint
                    == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                {
                    return _pixman_transparent_image()
                }
            }
            if ((*source).pixman_format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << ((*source).pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint)
                >= 8 as libc::c_int as libc::c_uint
            {
                if is_contained != 0 {
                    let mut data: *mut libc::c_void = ((*source).data)
                        .offset(
                            ((*sub).extents.x as libc::c_uint)
                                .wrapping_mul(
                                    ((*source).pixman_format as libc::c_uint
                                        >> 24 as libc::c_int
                                        & (((1 as libc::c_int) << 8 as libc::c_int)
                                            - 1 as libc::c_int) as libc::c_uint)
                                        << ((*source).pixman_format as libc::c_uint
                                            >> 22 as libc::c_int & 3 as libc::c_int as libc::c_uint),
                                )
                                .wrapping_div(8 as libc::c_int as libc::c_uint) as isize,
                        )
                        .offset(
                            ((*sub).extents.y as libc::c_long * (*source).stride)
                                as isize,
                        ) as *mut libc::c_void;
                    pixman_image = pixman_image_create_bits(
                        (*source).pixman_format,
                        (*sub).extents.width,
                        (*sub).extents.height,
                        data as *mut uint32_t,
                        (*source).stride as libc::c_int,
                    );
                    if pixman_image.is_null() {
                        return 0 as *mut pixman_image_t;
                    }
                }
            }
        }
    }
    if pixman_image.is_null() {
        let mut cleanup: *mut acquire_source_cleanup = 0 as *mut acquire_source_cleanup;
        let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
        let mut extra: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_surface_acquire_source_image(
            (*pattern).surface,
            &mut image,
            &mut extra,
        );
        if status as u64 != 0 {
            return 0 as *mut pixman_image_t;
        }
        pixman_image = pixman_image_create_bits(
            (*image).pixman_format,
            (*image).width,
            (*image).height,
            (*image).data as *mut uint32_t,
            (*image).stride as libc::c_int,
        );
        if pixman_image.is_null() {
            _cairo_surface_release_source_image((*pattern).surface, image, extra);
            return 0 as *mut pixman_image_t;
        }
        cleanup = (if ::std::mem::size_of::<acquire_source_cleanup>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<acquire_source_cleanup>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut acquire_source_cleanup;
        if cleanup.is_null() {
            _cairo_surface_release_source_image((*pattern).surface, image, extra);
            pixman_image_unref(pixman_image);
            return 0 as *mut pixman_image_t;
        }
        let ref mut fresh6 = (*cleanup).surface;
        *fresh6 = (*pattern).surface;
        let ref mut fresh7 = (*cleanup).image;
        *fresh7 = image;
        let ref mut fresh8 = (*cleanup).image_extra;
        *fresh8 = extra;
        pixman_image_set_destroy_function(
            pixman_image,
            Some(
                _acquire_source_cleanup
                    as unsafe extern "C" fn(*mut pixman_image_t, *mut libc::c_void) -> (),
            ),
            cleanup as *mut libc::c_void,
        );
    }
    if _pixman_image_set_properties(pixman_image, &(*pattern).base, extents, ix, iy) == 0
    {
        pixman_image_unref(pixman_image);
        pixman_image = 0 as *mut pixman_image_t;
    }
    return pixman_image;
}
unsafe extern "C" fn _raster_source_cleanup(
    mut pixman_image: *mut pixman_image_t,
    mut closure: *mut libc::c_void,
) {
    let mut data: *mut raster_source_cleanup = closure as *mut raster_source_cleanup;
    _cairo_surface_release_source_image(
        (*data).surface,
        (*data).image,
        (*data).image_extra,
    );
    _cairo_raster_source_pattern_release((*data).pattern, (*data).surface);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn _pixman_image_for_raster(
    mut dst: *mut cairo_image_surface_t,
    mut pattern: *const cairo_raster_source_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut ix: *mut libc::c_int,
    mut iy: *mut libc::c_int,
) -> *mut pixman_image_t {
    let mut pixman_image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut cleanup: *mut raster_source_cleanup = 0 as *mut raster_source_cleanup;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    *iy = 0 as libc::c_int;
    *ix = *iy;
    surface = _cairo_raster_source_pattern_acquire(
        &(*pattern).base,
        &mut (*dst).base,
        0 as *const cairo_rectangle_int_t,
    );
    if surface.is_null() || (*surface).status as libc::c_uint != 0 {
        return 0 as *mut pixman_image_t;
    }
    status = _cairo_surface_acquire_source_image(surface, &mut image, &mut extra);
    if status as u64 != 0 {
        _cairo_raster_source_pattern_release(&(*pattern).base, surface);
        return 0 as *mut pixman_image_t;
    }
    if (*image).width == (*pattern).extents.width {} else {
        __assert_fail(
            b"image->width == pattern->extents.width\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
            1514 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 195],
                &[libc::c_char; 195],
            >(
                b"pixman_image_t *_pixman_image_for_raster(cairo_image_surface_t *, const cairo_raster_source_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, const cairo_rectangle_int_t *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*image).height == (*pattern).extents.height {} else {
        __assert_fail(
            b"image->height == pattern->extents.height\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
            1515 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 195],
                &[libc::c_char; 195],
            >(
                b"pixman_image_t *_pixman_image_for_raster(cairo_image_surface_t *, const cairo_raster_source_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, const cairo_rectangle_int_t *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    pixman_image = pixman_image_create_bits(
        (*image).pixman_format,
        (*image).width,
        (*image).height,
        (*image).data as *mut uint32_t,
        (*image).stride as libc::c_int,
    );
    if pixman_image.is_null() {
        _cairo_surface_release_source_image(surface, image, extra);
        _cairo_raster_source_pattern_release(&(*pattern).base, surface);
        return 0 as *mut pixman_image_t;
    }
    cleanup = (if ::std::mem::size_of::<raster_source_cleanup>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<raster_source_cleanup>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut raster_source_cleanup;
    if cleanup.is_null() {
        pixman_image_unref(pixman_image);
        _cairo_surface_release_source_image(surface, image, extra);
        _cairo_raster_source_pattern_release(&(*pattern).base, surface);
        return 0 as *mut pixman_image_t;
    }
    let ref mut fresh9 = (*cleanup).pattern;
    *fresh9 = &(*pattern).base;
    let ref mut fresh10 = (*cleanup).surface;
    *fresh10 = surface;
    let ref mut fresh11 = (*cleanup).image;
    *fresh11 = image;
    let ref mut fresh12 = (*cleanup).image_extra;
    *fresh12 = extra;
    pixman_image_set_destroy_function(
        pixman_image,
        Some(
            _raster_source_cleanup
                as unsafe extern "C" fn(*mut pixman_image_t, *mut libc::c_void) -> (),
        ),
        cleanup as *mut libc::c_void,
    );
    if _pixman_image_set_properties(pixman_image, &(*pattern).base, extents, ix, iy) == 0
    {
        pixman_image_unref(pixman_image);
        pixman_image = 0 as *mut pixman_image_t;
    }
    return pixman_image;
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_image_for_pattern(
    mut dst: *mut cairo_image_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut tx: *mut libc::c_int,
    mut ty: *mut libc::c_int,
) -> *mut pixman_image_t {
    *ty = 0 as libc::c_int;
    *tx = *ty;
    if pattern.is_null() {
        return _pixman_white_image();
    }
    match (*pattern).type_0 as libc::c_uint {
        0 => {}
        3 | 2 => {
            return _pixman_image_for_gradient(
                pattern as *const cairo_gradient_pattern_t,
                extents,
                tx,
                ty,
            );
        }
        4 => {
            return _pixman_image_for_mesh(
                pattern as *const cairo_mesh_pattern_t,
                extents,
                tx,
                ty,
            );
        }
        1 => {
            return _pixman_image_for_surface(
                dst,
                pattern as *const cairo_surface_pattern_t,
                is_mask,
                extents,
                sample,
                tx,
                ty,
            );
        }
        5 => {
            return _pixman_image_for_raster(
                dst,
                pattern as *const cairo_raster_source_pattern_t,
                is_mask,
                extents,
                sample,
                tx,
                ty,
            );
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-image-source.c\0" as *const u8 as *const libc::c_char,
                    1570 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 182],
                        &[libc::c_char; 182],
                    >(
                        b"pixman_image_t *_pixman_image_for_pattern(cairo_image_surface_t *, const cairo_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, const cairo_rectangle_int_t *, int *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    return _pixman_image_for_color(&(*(pattern as *const cairo_solid_pattern_t)).color);
}
unsafe extern "C" fn _cairo_image_source_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut source: *mut cairo_image_source_t = abstract_surface
        as *mut cairo_image_source_t;
    pixman_image_unref((*source).pixman_image);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub static mut _cairo_image_source_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            finish: Some(
                _cairo_image_source_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: None,
            create_similar: None,
            create_similar_image: None,
            map_to_image: None,
            unmap_image: None,
            source: None,
            acquire_source_image: None,
            release_source_image: None,
            snapshot: None,
            copy_page: None,
            show_page: None,
            get_extents: None,
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            fill_stroke: None,
            show_glyphs: None,
            has_show_text_glyphs: None,
            show_text_glyphs: None,
            get_supported_mime_types: None,
            tag: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_source_create_for_pattern(
    mut dst: *mut cairo_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut source: *mut cairo_image_source_t = 0 as *mut cairo_image_source_t;
    source = (if ::std::mem::size_of::<cairo_image_source_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_image_source_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_image_source_t;
    if source.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    let ref mut fresh13 = (*source).pixman_image;
    *fresh13 = _pixman_image_for_pattern(
        dst as *mut cairo_image_surface_t,
        pattern,
        is_mask,
        extents,
        sample,
        src_x,
        src_y,
    );
    if ((*source).pixman_image).is_null() {
        free(source as *mut libc::c_void);
        return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_surface_init(
        &mut (*source).base,
        &_cairo_image_source_backend,
        0 as *mut cairo_device_t,
        CAIRO_CONTENT_COLOR_ALPHA,
        0 as libc::c_int,
    );
    (*source)
        .set_is_opaque_solid(
            (pattern.is_null() || _cairo_pattern_is_opaque_solid(pattern) != 0)
                as libc::c_int as libc::c_uint,
        );
    return &mut (*source).base;
}
