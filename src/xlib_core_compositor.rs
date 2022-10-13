use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _XGC;
    pub type _XDisplay;
    pub type _cairo_xlib_shm_display;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_surface_create_similar_image(
        other: *mut cairo_surface_t,
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_boxes_init_with_clip(boxes: *mut cairo_boxes_t, clip: *mut cairo_clip_t);
    fn _cairo_boxes_init_for_array(
        boxes: *mut cairo_boxes_t,
        array: *mut cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_boxes_for_each_box(
        boxes: *mut cairo_boxes_t,
        func: Option::<
            unsafe extern "C" fn(*mut cairo_box_t, *mut libc::c_void) -> cairo_bool_t,
        >,
        data: *mut libc::c_void,
    ) -> cairo_bool_t;
    fn _cairo_boxes_fini(boxes: *mut cairo_boxes_t);
    fn _cairo_clip_is_region(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _pixman_format_to_masks(
        pixman_format: pixman_format_code_t,
        masks: *mut cairo_format_masks_t,
    ) -> cairo_bool_t;
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_fill_rectilinear_to_boxes(
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_rectilinear_to_boxes(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_int_status_t;
    fn XChangeGC(
        _: *mut Display,
        _: GC,
        _: libc::c_ulong,
        _: *mut XGCValues,
    ) -> libc::c_int;
    fn XCopyArea(
        _: *mut Display,
        _: Drawable,
        _: Drawable,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn _cairo_xlib_fallback_compositor_get() -> *const cairo_compositor_t;
    fn _cairo_xlib_surface_draw_image(
        surface: *mut cairo_xlib_surface_t,
        image: *mut cairo_image_surface_t,
        src_x: libc::c_int,
        src_y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        dst_x: libc::c_int,
        dst_y: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_xlib_screen_put_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        gc: GC,
    );
    fn _cairo_xlib_display_acquire(
        device: *mut cairo_device_t,
        display: *mut *mut cairo_xlib_display_t,
    ) -> cairo_status_t;
    fn _cairo_xlib_surface_get_gc(
        display: *mut cairo_xlib_display_t,
        surface: *mut cairo_xlib_surface_t,
        gc: *mut GC,
    ) -> cairo_status_t;
    fn _cairo_pattern_is_opaque(
        pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_offset_paint(
        target: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
}
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
pub type cairo_atomic_once_t = cairo_atomic_int_t;
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
pub struct _cairo_format_masks {
    pub bpp: libc::c_int,
    pub alpha_mask: libc::c_ulong,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
}
pub type cairo_format_masks_t = _cairo_format_masks;
pub type XID = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
pub type Display = _XDisplay;
pub type Picture = XID;
pub type PictFormat = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: libc::c_short,
    pub redMask: libc::c_short,
    pub green: libc::c_short,
    pub greenMask: libc::c_short,
    pub blue: libc::c_short,
    pub blueMask: libc::c_short,
    pub alpha: libc::c_short,
    pub alphaMask: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_0: libc::c_int,
    pub depth: libc::c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_xlib_display {
    pub base: cairo_device_t,
    pub next: *mut cairo_xlib_display_t,
    pub display: *mut Display,
    pub screens: cairo_list_t,
    pub fonts: cairo_list_t,
    pub shm: *mut cairo_xlib_shm_display_t,
    pub compositor: *const cairo_compositor_t,
    pub render_major: libc::c_int,
    pub render_minor: libc::c_int,
    pub cached_xrender_formats: [*mut XRenderPictFormat; 6],
    pub force_precision: libc::c_int,
    pub white: *mut cairo_surface_t,
    pub alpha: [*mut cairo_surface_t; 256],
    pub solid: [*mut cairo_surface_t; 32],
    pub solid_cache: [uint32_t; 32],
    pub last_solid_cache: [C2RustUnnamed; 2],
    #[bitfield(name = "buggy_gradients", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "buggy_pad_reflect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "buggy_repeat", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "closed", ty = "libc::c_uint", bits = "3..=3")]
    pub buggy_gradients_buggy_pad_reflect_buggy_repeat_closed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub color: uint32_t,
    pub index: libc::c_int,
}
pub type cairo_xlib_shm_display_t = _cairo_xlib_shm_display;
pub type cairo_xlib_display_t = _cairo_xlib_display;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_screen {
    pub link: cairo_list_t,
    pub device: *mut cairo_device_t,
    pub screen: *mut Screen,
    pub surfaces: cairo_list_t,
    pub has_font_options: cairo_bool_t,
    pub font_options: cairo_font_options_t,
    pub gc: [GC; 4],
    pub gc_depths: [uint8_t; 4],
    pub visuals: cairo_list_t,
}
pub type cairo_xlib_screen_t = _cairo_xlib_screen;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_xlib_source {
    pub base: cairo_surface_t,
    pub picture: Picture,
    pub pixmap: Pixmap,
    pub dpy: *mut Display,
    #[bitfield(name = "filter", ty = "libc::c_uint", bits = "0..=2")]
    #[bitfield(name = "extend", ty = "libc::c_uint", bits = "3..=5")]
    #[bitfield(name = "has_matrix", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "has_component_alpha", ty = "libc::c_uint", bits = "7..=7")]
    pub filter_extend_has_matrix_has_component_alpha: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_surface {
    pub base: cairo_surface_t,
    pub picture: Picture,
    pub drawable: Drawable,
    pub compositor: *const cairo_compositor_t,
    pub shm: *mut cairo_surface_t,
    pub fallback: libc::c_int,
    pub display: *mut cairo_xlib_display_t,
    pub screen: *mut cairo_xlib_screen_t,
    pub link: cairo_list_t,
    pub dpy: *mut Display,
    pub owns_pixmap: cairo_bool_t,
    pub visual: *mut Visual,
    pub use_pixmap: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub precision: libc::c_int,
    pub xrender_format: *mut XRenderPictFormat,
    pub a_mask: uint32_t,
    pub r_mask: uint32_t,
    pub g_mask: uint32_t,
    pub b_mask: uint32_t,
    pub embedded_source: _cairo_xlib_source,
}
pub type cairo_xlib_surface_t = _cairo_xlib_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fallback_box {
    pub dst: *mut cairo_xlib_surface_t,
    pub format: cairo_format_t,
    pub pattern: *const cairo_pattern_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _box_data {
    pub dpy: *mut Display,
    pub dst: *mut cairo_xlib_surface_t,
    pub src: *mut cairo_surface_t,
    pub gc: GC,
    pub tx: libc::c_int,
    pub ty: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fill_box {
    pub dpy: *mut Display,
    pub drawable: Drawable,
    pub gc: GC,
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
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_init_once_enter(
    mut once: *mut cairo_atomic_once_t,
) -> cairo_bool_t {
    if _cairo_atomic_int_get(once) == 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if _cairo_atomic_int_cmpxchg_impl(once, 0 as libc::c_int, 1 as libc::c_int) != 0 {
        return 1 as libc::c_int;
    }
    while _cairo_atomic_int_get(once) != 2 as libc::c_int {}
    return 0 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_init_once_leave(mut once: *mut cairo_atomic_once_t) {
    if _cairo_atomic_int_cmpxchg_impl(once, 1 as libc::c_int, 2 as libc::c_int) == 0 {
        if 0 as libc::c_int != 0
            && !(b"incorrect use of _cairo_atomic_init_once API (once != CAIRO_ATOMIC_ONCE_INITIALIZING)\0"
                as *const u8 as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"0 && \"incorrect use of _cairo_atomic_init_once API (once != CAIRO_ATOMIC_ONCE_INITIALIZING)\"\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-atomic-private.h\0" as *const u8 as *const libc::c_char,
                478 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void _cairo_atomic_init_once_leave(cairo_atomic_once_t *)\0"))
                    .as_ptr(),
            );
        }
    }
}
#[inline]
unsafe extern "C" fn _cairo_popcount(mut mask: uint32_t) -> libc::c_int {
    return mask.count_ones() as i32;
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
#[inline]
unsafe extern "C" fn _cairo_path_fixed_stroke_is_rectilinear(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).stroke_is_rectilinear() as cairo_bool_t;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_xlib_surface_same_screen(
    mut dst: *mut cairo_xlib_surface_t,
    mut src: *mut cairo_xlib_surface_t,
) -> cairo_bool_t {
    return ((*dst).screen == (*src).screen) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_xlib_surface_put_gc(
    mut display: *mut cairo_xlib_display_t,
    mut surface: *mut cairo_xlib_surface_t,
    mut gc: GC,
) {
    _cairo_xlib_screen_put_gc(display, (*surface).screen, (*surface).depth, gc);
}
#[inline]
unsafe extern "C" fn _cairo_clip_steal_boxes(
    mut clip: *mut cairo_clip_t,
    mut boxes: *mut cairo_boxes_t,
) {
    let mut array: *mut cairo_box_t = (*clip).boxes;
    if array == &mut (*clip).embedded_box as *mut cairo_box_t {
        if (*clip).num_boxes == 1 as libc::c_int {} else {
            __assert_fail(
                b"clip->num_boxes == 1\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-clip-inline.h\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"void _cairo_clip_steal_boxes(cairo_clip_t *, cairo_boxes_t *)\0"))
                    .as_ptr(),
            );
        }
        (*boxes).boxes_embedded[0 as libc::c_int as usize] = (*clip).embedded_box;
        array = &mut *((*boxes).boxes_embedded)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cairo_box_t;
    }
    _cairo_boxes_init_for_array(boxes, array, (*clip).num_boxes);
    let ref mut fresh1 = (*clip).boxes;
    *fresh1 = 0 as *mut cairo_box_t;
    (*clip).num_boxes = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_clip_unsteal_boxes(
    mut clip: *mut cairo_clip_t,
    mut boxes: *mut cairo_boxes_t,
) {
    if (*boxes).chunks.base
        == &mut *((*boxes).boxes_embedded).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut cairo_box_t
    {
        if (*boxes).num_boxes == 1 as libc::c_int {} else {
            __assert_fail(
                b"boxes->num_boxes == 1\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-clip-inline.h\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void _cairo_clip_unsteal_boxes(cairo_clip_t *, cairo_boxes_t *)\0"))
                    .as_ptr(),
            );
        }
        (*clip).embedded_box = *(*boxes).chunks.base;
        let ref mut fresh2 = (*clip).boxes;
        *fresh2 = &mut (*clip).embedded_box;
    } else {
        let ref mut fresh3 = (*clip).boxes;
        *fresh3 = (*boxes).chunks.base;
    }
    (*clip).num_boxes = (*boxes).num_boxes;
}
unsafe extern "C" fn acquire(mut abstract_dst: *mut libc::c_void) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    return _cairo_xlib_display_acquire((*dst).base.device, &mut (*dst).display)
        as cairo_int_status_t;
}
unsafe extern "C" fn release(mut abstract_dst: *mut libc::c_void) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    cairo_device_release(&mut (*(*dst).display).base);
    let ref mut fresh4 = (*dst).display;
    *fresh4 = 0 as *mut cairo_xlib_display_t;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn fill_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut data: *mut _fill_box = closure as *mut _fill_box;
    let mut x: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.x);
    let mut y: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.y);
    let mut width: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.x - (*box_0).p1.x,
    );
    let mut height: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.y - (*box_0).p1.y,
    );
    XFillRectangle(
        (*data).dpy,
        (*data).drawable,
        (*data).gc,
        x,
        y,
        width as libc::c_uint,
        height as libc::c_uint,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn _characterize_field(
    mut mask: uint32_t,
    mut width: *mut libc::c_int,
    mut shift: *mut libc::c_int,
) {
    *width = _cairo_popcount(mask);
    *shift = _cairo_popcount(mask.wrapping_sub(1 as libc::c_int as libc::c_uint) & !mask)
        & 31 as libc::c_int;
}
unsafe extern "C" fn color_to_pixel(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
) -> uint32_t {
    let mut rgba: uint32_t = 0 as libc::c_int as uint32_t;
    let mut width: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    _characterize_field((*dst).a_mask, &mut width, &mut shift);
    rgba
        |= ((*color).alpha_short as libc::c_int >> 16 as libc::c_int - width << shift)
            as libc::c_uint;
    _characterize_field((*dst).r_mask, &mut width, &mut shift);
    rgba
        |= ((*color).red_short as libc::c_int >> 16 as libc::c_int - width << shift)
            as libc::c_uint;
    _characterize_field((*dst).g_mask, &mut width, &mut shift);
    rgba
        |= ((*color).green_short as libc::c_int >> 16 as libc::c_int - width << shift)
            as libc::c_uint;
    _characterize_field((*dst).b_mask, &mut width, &mut shift);
    rgba
        |= ((*color).blue_short as libc::c_int >> 16 as libc::c_int - width << shift)
            as libc::c_uint;
    return rgba;
}
unsafe extern "C" fn _fill_box_init(
    mut fb: *mut _fill_box,
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_xlib_surface_get_gc((*dst).display, dst, &mut (*fb).gc)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh5 = (*fb).dpy;
    *fresh5 = (*(*dst).display).display;
    (*fb).drawable = (*dst).drawable;
    if !(!((*dst).visual).is_null() && (*(*dst).visual).class != 4 as libc::c_int
        && 0 as libc::c_int != 0)
    {
        let mut gcv: XGCValues = XGCValues {
            function: 0,
            plane_mask: 0,
            foreground: 0,
            background: 0,
            line_width: 0,
            line_style: 0,
            cap_style: 0,
            join_style: 0,
            fill_style: 0,
            fill_rule: 0,
            arc_mode: 0,
            tile: 0,
            stipple: 0,
            ts_x_origin: 0,
            ts_y_origin: 0,
            font: 0,
            subwindow_mode: 0,
            graphics_exposures: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            dash_offset: 0,
            dashes: 0,
        };
        gcv.foreground = color_to_pixel(dst, color) as libc::c_ulong;
        gcv.fill_style = 0 as libc::c_int;
        XChangeGC(
            (*fb).dpy,
            (*fb).gc,
            ((1 as libc::c_long) << 8 as libc::c_int
                | (1 as libc::c_long) << 2 as libc::c_int) as libc::c_ulong,
            &mut gcv,
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill_box_fini(
    mut fb: *mut _fill_box,
    mut dst: *mut cairo_xlib_surface_t,
) {
    _cairo_xlib_surface_put_gc((*dst).display, dst, (*fb).gc);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_core_fill_boxes(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut fb: _fill_box = _fill_box {
        dpy: 0 as *mut Display,
        drawable: 0,
        gc: 0 as *mut _XGC,
    };
    status = _fill_box_init(&mut fb, dst, color);
    if status as u64 != 0 {
        return status;
    }
    _cairo_boxes_for_each_box(
        boxes,
        Some(
            fill_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut fb as *mut _fill_box as *mut libc::c_void,
    );
    _fill_box_fini(&mut fb, dst);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_core_fill_rectangles(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
    mut num_rects: libc::c_int,
    mut rects: *mut cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut fb: _fill_box = _fill_box {
        dpy: 0 as *mut Display,
        drawable: 0,
        gc: 0 as *mut _XGC,
    };
    let mut i: libc::c_int = 0;
    status = _fill_box_init(&mut fb, dst, color);
    if status as u64 != 0 {
        return status;
    }
    i = 0 as libc::c_int;
    while i < num_rects {
        XFillRectangle(
            fb.dpy,
            fb.drawable,
            fb.gc,
            (*rects.offset(i as isize)).x,
            (*rects.offset(i as isize)).y,
            (*rects.offset(i as isize)).width as libc::c_uint,
            (*rects.offset(i as isize)).height as libc::c_uint,
        );
        i += 1;
    }
    _fill_box_fini(&mut fb, dst);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn fallback_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut data: *mut _fallback_box = closure as *mut _fallback_box;
    let mut x: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.x);
    let mut y: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.y);
    let mut width: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.x - (*box_0).p1.x,
    );
    let mut height: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.y - (*box_0).p1.y,
    );
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    image = cairo_surface_create_similar_image(
        &mut (*(*data).dst).base,
        (*data).format,
        width,
        height,
    );
    status = _cairo_surface_offset_paint(
        image,
        x,
        y,
        CAIRO_OPERATOR_SOURCE,
        (*data).pattern,
        0 as *const cairo_clip_t,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _cairo_xlib_surface_draw_image(
            (*data).dst,
            image as *mut cairo_image_surface_t,
            0 as libc::c_int,
            0 as libc::c_int,
            width,
            height,
            x,
            y,
        );
    }
    cairo_surface_destroy(image);
    return (status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn fallback_boxes(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut fb: _fallback_box = _fallback_box {
        dst: 0 as *mut cairo_xlib_surface_t,
        format: CAIRO_FORMAT_ARGB32,
        pattern: 0 as *const cairo_pattern_t,
    };
    match (*dst).depth {
        8 => {
            fb.format = CAIRO_FORMAT_A8;
        }
        16 => {
            fb.format = CAIRO_FORMAT_RGB16_565;
        }
        24 => {
            fb.format = CAIRO_FORMAT_RGB24;
        }
        30 => {
            fb.format = CAIRO_FORMAT_RGB30;
        }
        32 => {
            fb.format = CAIRO_FORMAT_ARGB32;
        }
        _ => return CAIRO_INT_STATUS_UNSUPPORTED,
    }
    fb.dst = dst;
    fb.pattern = pattern;
    if _cairo_boxes_for_each_box(
        boxes,
        Some(
            fallback_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut fb as *mut _fallback_box as *mut libc::c_void,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn render_boxes(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    if (*pattern).filter as libc::c_uint
        != CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint
    {
        return fallback_boxes(dst, pattern, boxes);
    }
    match (*pattern).extend as libc::c_uint {
        1 => return fallback_boxes(dst, pattern, boxes),
        0 | 2 | 3 | _ => return fallback_boxes(dst, pattern, boxes),
    };
}
unsafe extern "C" fn source_contains_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut data: *mut _box_data = closure as *mut _box_data;
    return (_cairo_fixed_integer_part((*box_0).p1.x) + (*data).tx >= 0 as libc::c_int
        && _cairo_fixed_integer_part((*box_0).p1.y) + (*data).ty >= 0 as libc::c_int
        && _cairo_fixed_integer_part((*box_0).p2.x) + (*data).tx <= (*data).width
        && _cairo_fixed_integer_part((*box_0).p2.y) + (*data).ty <= (*data).height)
        as libc::c_int;
}
unsafe extern "C" fn image_upload_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut iub: *const _box_data = closure as *const _box_data;
    let mut x: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.x);
    let mut y: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.y);
    let mut width: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.x - (*box_0).p1.x,
    );
    let mut height: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.y - (*box_0).p1.y,
    );
    return (_cairo_xlib_surface_draw_image(
        (*iub).dst,
        (*iub).src as *mut cairo_image_surface_t,
        x + (*iub).tx,
        y + (*iub).ty,
        width,
        height,
        x,
        y,
    ) as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn surface_matches_image_format(
    mut surface: *mut cairo_xlib_surface_t,
    mut image: *mut cairo_image_surface_t,
) -> cairo_bool_t {
    let mut format: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    return (_pixman_format_to_masks((*image).pixman_format, &mut format) != 0
        && (format.alpha_mask == (*surface).a_mask as libc::c_ulong
            || (*surface).a_mask == 0 as libc::c_int as libc::c_uint)
        && (format.red_mask == (*surface).r_mask as libc::c_ulong
            || (*surface).r_mask == 0 as libc::c_int as libc::c_uint)
        && (format.green_mask == (*surface).g_mask as libc::c_ulong
            || (*surface).g_mask == 0 as libc::c_int as libc::c_uint)
        && (format.blue_mask == (*surface).b_mask as libc::c_ulong
            || (*surface).b_mask == 0 as libc::c_int as libc::c_uint)) as libc::c_int;
}
unsafe extern "C" fn upload_image_inplace(
    mut dst: *mut cairo_xlib_surface_t,
    mut source: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut pattern: *const cairo_surface_pattern_t = 0
        as *const cairo_surface_pattern_t;
    let mut iub: _box_data = _box_data {
        dpy: 0 as *mut Display,
        dst: 0 as *mut cairo_xlib_surface_t,
        src: 0 as *mut cairo_surface_t,
        gc: 0 as *mut _XGC,
        tx: 0,
        ty: 0,
        width: 0,
        height: 0,
    };
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    if (*source).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    pattern = source as *const cairo_surface_pattern_t;
    if (*(*pattern).surface).type_0 as libc::c_uint
        != CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    image = (*pattern).surface as *mut cairo_image_surface_t;
    if (*image).format as libc::c_int == CAIRO_FORMAT_INVALID as libc::c_int {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*image).depth != (*dst).depth {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if surface_matches_image_format(dst, image) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_matrix_is_integer_translation(&(*source).matrix, &mut iub.tx, &mut iub.ty)
        == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    iub.dst = dst;
    iub.src = &mut (*image).base;
    iub.width = (*image).width;
    iub.height = (*image).height;
    if _cairo_boxes_for_each_box(
        boxes,
        Some(
            source_contains_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut iub as *mut _box_data as *mut libc::c_void,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_boxes_for_each_box(
        boxes,
        Some(
            image_upload_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut iub as *mut _box_data as *mut libc::c_void,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn copy_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut cb: *const _box_data = closure as *const _box_data;
    let mut x: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.x);
    let mut y: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.y);
    let mut width: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.x - (*box_0).p1.x,
    );
    let mut height: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.y - (*box_0).p1.y,
    );
    XCopyArea(
        (*cb).dpy,
        (*((*cb).src as *mut cairo_xlib_surface_t)).drawable,
        (*(*cb).dst).drawable,
        (*cb).gc,
        x + (*cb).tx,
        y + (*cb).ty,
        width as libc::c_uint,
        height as libc::c_uint,
        x,
        y,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn copy_boxes(
    mut dst: *mut cairo_xlib_surface_t,
    mut source: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut pattern: *const cairo_surface_pattern_t = 0
        as *const cairo_surface_pattern_t;
    let mut cb: _box_data = _box_data {
        dpy: 0 as *mut Display,
        dst: 0 as *mut cairo_xlib_surface_t,
        src: 0 as *mut cairo_surface_t,
        gc: 0 as *mut _XGC,
        tx: 0,
        ty: 0,
        width: 0,
        height: 0,
    };
    let mut src: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*source).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    pattern = source as *const cairo_surface_pattern_t;
    if (*(*(*pattern).surface).backend).type_0 as libc::c_uint
        != CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    src = (*pattern).surface as *mut cairo_xlib_surface_t;
    if (*src).depth != (*dst).depth {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*src).owns_pixmap == 0 && (*dst).owns_pixmap == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_xlib_surface_same_screen(dst, src) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_matrix_is_integer_translation(&(*source).matrix, &mut cb.tx, &mut cb.ty)
        == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    cb.dpy = (*(*dst).display).display;
    cb.dst = dst;
    cb.src = &mut (*src).base;
    cb.width = (*src).width;
    cb.height = (*src).height;
    if _cairo_boxes_for_each_box(
        boxes,
        Some(
            source_contains_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut cb as *mut _box_data as *mut libc::c_void,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = _cairo_xlib_surface_get_gc((*dst).display, dst, &mut cb.gc);
    if status as u64 != 0 {
        return status;
    }
    if (*src).owns_pixmap == 0 {
        let mut gcv: XGCValues = XGCValues {
            function: 0,
            plane_mask: 0,
            foreground: 0,
            background: 0,
            line_width: 0,
            line_style: 0,
            cap_style: 0,
            join_style: 0,
            fill_style: 0,
            fill_rule: 0,
            arc_mode: 0,
            tile: 0,
            stipple: 0,
            ts_x_origin: 0,
            ts_y_origin: 0,
            font: 0,
            subwindow_mode: 0,
            graphics_exposures: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            dash_offset: 0,
            dashes: 0,
        };
        gcv.subwindow_mode = 1 as libc::c_int;
        XChangeGC(
            (*(*dst).display).display,
            cb.gc,
            ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
            &mut gcv,
        );
    }
    status = CAIRO_STATUS_SUCCESS;
    if _cairo_boxes_for_each_box(
        boxes,
        Some(
            copy_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut cb as *mut _box_data as *mut libc::c_void,
    ) == 0
    {
        status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*src).owns_pixmap == 0 {
        let mut gcv_0: XGCValues = XGCValues {
            function: 0,
            plane_mask: 0,
            foreground: 0,
            background: 0,
            line_width: 0,
            line_style: 0,
            cap_style: 0,
            join_style: 0,
            fill_style: 0,
            fill_rule: 0,
            arc_mode: 0,
            tile: 0,
            stipple: 0,
            ts_x_origin: 0,
            ts_y_origin: 0,
            font: 0,
            subwindow_mode: 0,
            graphics_exposures: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            dash_offset: 0,
            dashes: 0,
        };
        gcv_0.subwindow_mode = 0 as libc::c_int;
        XChangeGC(
            (*(*dst).display).display,
            cb.gc,
            ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
            &mut gcv_0,
        );
    }
    _cairo_xlib_surface_put_gc((*dst).display, dst, cb.gc);
    return status;
}
unsafe extern "C" fn draw_boxes(
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_xlib_surface_t = (*extents).surface
        as *mut cairo_xlib_surface_t;
    let mut op: cairo_operator_t = (*extents).op;
    let mut src: *const cairo_pattern_t = &mut (*extents).source_pattern.base;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*boxes).num_boxes == 0 as libc::c_int && (*extents).is_bounded != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*boxes).is_pixel_aligned == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
        op = CAIRO_OPERATOR_SOURCE;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
        && _cairo_pattern_is_opaque(src, &mut (*extents).bounded) != 0
    {
        op = CAIRO_OPERATOR_SOURCE;
    }
    if ((*dst).base).is_clear() as libc::c_int != 0
        && op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
    {
        op = CAIRO_OPERATOR_SOURCE;
    }
    if op as libc::c_uint != CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = acquire(dst as *mut libc::c_void);
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if (*src).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        status = _cairo_xlib_core_fill_boxes(
            dst,
            &mut (*(src as *mut cairo_solid_pattern_t)).color,
            boxes,
        );
    } else {
        status = upload_image_inplace(dst, src, boxes) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            status = copy_boxes(dst, src, boxes) as cairo_int_status_t;
        }
        if status as libc::c_uint
            == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            status = render_boxes(dst, src, boxes);
        }
    }
    release(dst as *mut libc::c_void);
    return status as cairo_status_t;
}
unsafe extern "C" fn _cairo_xlib_core_compositor_paint(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if _cairo_clip_is_region((*extents).clip) != 0 {
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
        _cairo_clip_steal_boxes((*extents).clip, &mut boxes);
        status = draw_boxes(extents, &mut boxes) as cairo_int_status_t;
        _cairo_clip_unsteal_boxes((*extents).clip, &mut boxes);
    }
    return status;
}
unsafe extern "C" fn _cairo_xlib_core_compositor_stroke(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if ((*(*extents).clip).path).is_null()
        && _cairo_path_fixed_stroke_is_rectilinear(path) != 0
    {
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
        _cairo_boxes_init_with_clip(&mut boxes, (*extents).clip);
        status = _cairo_path_fixed_stroke_rectilinear_to_boxes(
            path,
            style,
            ctm,
            antialias,
            &mut boxes,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = draw_boxes(extents, &mut boxes) as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
    }
    return status;
}
unsafe extern "C" fn _cairo_xlib_core_compositor_fill(
    mut compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if ((*(*extents).clip).path).is_null()
        && _cairo_path_fixed_fill_is_rectilinear(path) != 0
    {
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
        _cairo_boxes_init_with_clip(&mut boxes, (*extents).clip);
        status = _cairo_path_fixed_fill_rectilinear_to_boxes(
            path,
            fill_rule,
            antialias,
            &mut boxes,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = draw_boxes(extents, &mut boxes) as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_core_compositor_get() -> *const cairo_compositor_t {
    static mut once: cairo_atomic_once_t = 0 as libc::c_int;
    static mut compositor: cairo_compositor_t = cairo_compositor_t {
        delegate: 0 as *const cairo_compositor_t,
        paint: None,
        mask: None,
        stroke: None,
        fill: None,
        glyphs: None,
    };
    if _cairo_atomic_init_once_enter(&mut once) != 0 {
        compositor.delegate = _cairo_xlib_fallback_compositor_get();
        compositor
            .paint = Some(
            _cairo_xlib_core_compositor_paint
                as unsafe extern "C" fn(
                    *const cairo_compositor_t,
                    *mut cairo_composite_rectangles_t,
                ) -> cairo_int_status_t,
        );
        compositor.mask = None;
        compositor
            .fill = Some(
            _cairo_xlib_core_compositor_fill
                as unsafe extern "C" fn(
                    *const cairo_compositor_t,
                    *mut cairo_composite_rectangles_t,
                    *const cairo_path_fixed_t,
                    cairo_fill_rule_t,
                    libc::c_double,
                    cairo_antialias_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .stroke = Some(
            _cairo_xlib_core_compositor_stroke
                as unsafe extern "C" fn(
                    *const cairo_compositor_t,
                    *mut cairo_composite_rectangles_t,
                    *const cairo_path_fixed_t,
                    *const cairo_stroke_style_t,
                    *const cairo_matrix_t,
                    *const cairo_matrix_t,
                    libc::c_double,
                    cairo_antialias_t,
                ) -> cairo_int_status_t,
        );
        compositor.glyphs = None;
        _cairo_atomic_init_once_leave(&mut once);
    }
    return &mut compositor;
}
