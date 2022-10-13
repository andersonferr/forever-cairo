use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn cairo_surface_create_similar_image(
        other: *mut cairo_surface_t,
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_matrix_init_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn cairo_region_contains_rectangle(
        region: *const cairo_region_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_region_overlap_t;
    fn pixman_region32_rectangles(
        region: *mut pixman_region32_t,
        n_rects: *mut libc::c_int,
    ) -> *mut pixman_box32_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_boxes_init(boxes: *mut cairo_boxes_t);
    fn _cairo_boxes_init_with_clip(boxes: *mut cairo_boxes_t, clip: *mut cairo_clip_t);
    fn _cairo_boxes_init_for_array(
        boxes: *mut cairo_boxes_t,
        array: *mut cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_boxes_limit(
        boxes: *mut cairo_boxes_t,
        limits: *const cairo_box_t,
        num_limits: libc::c_int,
    );
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_extents(boxes: *const cairo_boxes_t, box_0: *mut cairo_box_t);
    fn _cairo_boxes_fini(boxes: *mut cairo_boxes_t);
    fn _cairo_clip_get_image(
        clip: *const cairo_clip_t,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_clip_combine_with_surface(
        clip: *const cairo_clip_t,
        dst: *mut cairo_surface_t,
        dst_x: libc::c_int,
        dst_y: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_clip_get_region(clip: *const cairo_clip_t) -> *mut cairo_region_t;
    fn _cairo_clip_is_region(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _cairo_int_surface_create_in_error(
        status: cairo_int_status_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_color_init_rgba(
        color: *mut cairo_color_t,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
        alpha: libc::c_double,
    );
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
    fn _cairo_surface_create_scratch(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
        color: *const cairo_color_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_mask(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn _cairo_bentley_ottmann_tessellate_boxes(
        in_0: *const cairo_boxes_t,
        fill_rule: cairo_fill_rule_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_composite_rectangles_can_reduce_clip(
        composite: *mut cairo_composite_rectangles_t,
        clip: *mut cairo_clip_t,
    ) -> cairo_bool_t;
    fn _cairo_composite_rectangles_intersect_mask_extents(
        extents: *mut cairo_composite_rectangles_t,
        box_0: *const cairo_box_t,
    ) -> cairo_int_status_t;
    fn _cairo_pattern_is_opaque_solid(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_init_solid(
        pattern: *mut cairo_solid_pattern_t,
        color: *const cairo_color_t,
    );
    fn _cairo_surface_offset_stroke(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_fill(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_glyphs(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
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
pub type _cairo_region_overlap = libc::c_uint;
pub const CAIRO_REGION_OVERLAP_PART: _cairo_region_overlap = 2;
pub const CAIRO_REGION_OVERLAP_OUT: _cairo_region_overlap = 1;
pub const CAIRO_REGION_OVERLAP_IN: _cairo_region_overlap = 0;
pub type cairo_region_overlap_t = _cairo_region_overlap;
pub type uint8_t = __uint8_t;
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
pub struct cairo_mask_compositor {
    pub base: cairo_compositor_t,
    pub acquire: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t>,
    pub release: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t>,
    pub set_clip_region: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_region_t,
        ) -> cairo_int_status_t,
    >,
    pub pattern_to_surface: Option::<
        unsafe extern "C" fn(
            *mut cairo_surface_t,
            *const cairo_pattern_t,
            cairo_bool_t,
            *const cairo_rectangle_int_t,
            *const cairo_rectangle_int_t,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> *mut cairo_surface_t,
    >,
    pub draw_image_boxes: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
            *mut cairo_boxes_t,
            libc::c_int,
            libc::c_int,
        ) -> cairo_int_status_t,
    >,
    pub copy_boxes: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_surface_t,
            *mut cairo_boxes_t,
            *const cairo_rectangle_int_t,
            libc::c_int,
            libc::c_int,
        ) -> cairo_int_status_t,
    >,
    pub fill_rectangles: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_color_t,
            *mut cairo_rectangle_int_t,
            libc::c_int,
        ) -> cairo_int_status_t,
    >,
    pub fill_boxes: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_color_t,
            *mut cairo_boxes_t,
        ) -> cairo_int_status_t,
    >,
    pub check_composite: Option::<
        unsafe extern "C" fn(*const cairo_composite_rectangles_t) -> cairo_int_status_t,
    >,
    pub composite: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *mut cairo_surface_t,
            *mut cairo_surface_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_uint,
            libc::c_uint,
        ) -> cairo_int_status_t,
    >,
    pub composite_boxes: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *mut cairo_surface_t,
            *mut cairo_surface_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut cairo_boxes_t,
            *const cairo_rectangle_int_t,
        ) -> cairo_int_status_t,
    >,
    pub check_composite_glyphs: Option::<
        unsafe extern "C" fn(
            *const cairo_composite_rectangles_t,
            *mut cairo_scaled_font_t,
            *mut cairo_glyph_t,
            *mut libc::c_int,
        ) -> cairo_int_status_t,
    >,
    pub composite_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *mut cairo_surface_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut cairo_composite_glyphs_info_t,
        ) -> cairo_int_status_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_composite_glyphs_info_t {
    pub font: *mut cairo_scaled_font_t,
    pub glyphs: *mut cairo_glyph_t,
    pub num_glyphs: libc::c_int,
    pub use_mask: cairo_bool_t,
    pub extents: cairo_rectangle_int_t,
}
pub type cairo_mask_compositor_t = cairo_mask_compositor;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
pub const NEED_CLIP_REGION: C2RustUnnamed = 1;
pub const NEED_CLIP_SURFACE: C2RustUnnamed = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_box_info {
    pub compositor: *const cairo_mask_compositor_t,
    pub dst: *mut cairo_surface_t,
    pub src: *mut cairo_surface_t,
    pub src_x: libc::c_int,
    pub src_y: libc::c_int,
    pub op: uint8_t,
}
pub type draw_func_t = Option::<
    unsafe extern "C" fn(
        *const cairo_mask_compositor_t,
        *mut cairo_surface_t,
        *mut libc::c_void,
        cairo_operator_t,
        *const cairo_pattern_t,
        *const cairo_rectangle_int_t,
        libc::c_int,
        libc::c_int,
        *const cairo_rectangle_int_t,
        *mut cairo_clip_t,
    ) -> cairo_int_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blt_in {
    pub compositor: *const cairo_mask_compositor_t,
    pub dst: *mut cairo_surface_t,
}
pub const FORCE_CLIP_REGION: C2RustUnnamed = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_opacity_info {
    pub compositor: *const cairo_mask_compositor_t,
    pub op: uint8_t,
    pub dst: *mut cairo_surface_t,
    pub src: *mut cairo_surface_t,
    pub src_x: libc::c_int,
    pub src_y: libc::c_int,
    pub opacity: libc::c_double,
}
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn _cairo_fixed_fractional_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
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
    let ref mut fresh0 = (*clip).boxes;
    *fresh0 = 0 as *mut cairo_box_t;
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
        let ref mut fresh1 = (*clip).boxes;
        *fresh1 = &mut (*clip).embedded_box;
    } else {
        let ref mut fresh2 = (*clip).boxes;
        *fresh2 = (*boxes).chunks.base;
    }
    (*clip).num_boxes = (*boxes).num_boxes;
}
#[inline]
unsafe extern "C" fn _cairo_pattern_get_source(
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    return _cairo_surface_get_source((*pattern).surface, extents);
}
unsafe extern "C" fn do_unaligned_row(
    mut blt: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            int16_t,
            int16_t,
            int16_t,
            int16_t,
            uint16_t,
        ) -> (),
    >,
    mut closure: *mut libc::c_void,
    mut b: *const cairo_box_t,
    mut tx: libc::c_int,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut coverage: uint16_t,
) {
    let mut x1: libc::c_int = _cairo_fixed_integer_part((*b).p1.x) - tx;
    let mut x2: libc::c_int = _cairo_fixed_integer_part((*b).p2.x) - tx;
    if x2 > x1 {
        if _cairo_fixed_is_integer((*b).p1.x) == 0 {
            blt
                .expect(
                    "non-null function pointer",
                )(
                closure,
                x1 as int16_t,
                y as int16_t,
                1 as libc::c_int as int16_t,
                h as int16_t,
                (coverage as libc::c_int
                    * (256 as libc::c_int - _cairo_fixed_fractional_part((*b).p1.x)))
                    as uint16_t,
            );
            x1 += 1;
        }
        if x2 > x1 {
            blt
                .expect(
                    "non-null function pointer",
                )(
                closure,
                x1 as int16_t,
                y as int16_t,
                (x2 - x1) as int16_t,
                h as int16_t,
                (((coverage as libc::c_int) << 8 as libc::c_int)
                    - (coverage as libc::c_int >> 8 as libc::c_int)) as uint16_t,
            );
        }
        if _cairo_fixed_is_integer((*b).p2.x) == 0 {
            blt
                .expect(
                    "non-null function pointer",
                )(
                closure,
                x2 as int16_t,
                y as int16_t,
                1 as libc::c_int as int16_t,
                h as int16_t,
                (coverage as libc::c_int * _cairo_fixed_fractional_part((*b).p2.x))
                    as uint16_t,
            );
        }
    } else {
        blt
            .expect(
                "non-null function pointer",
            )(
            closure,
            x1 as int16_t,
            y as int16_t,
            1 as libc::c_int as int16_t,
            h as int16_t,
            (coverage as libc::c_int * ((*b).p2.x - (*b).p1.x)) as uint16_t,
        );
    };
}
unsafe extern "C" fn do_unaligned_box(
    mut blt: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            int16_t,
            int16_t,
            int16_t,
            int16_t,
            uint16_t,
        ) -> (),
    >,
    mut closure: *mut libc::c_void,
    mut b: *const cairo_box_t,
    mut tx: libc::c_int,
    mut ty: libc::c_int,
) {
    let mut y1: libc::c_int = _cairo_fixed_integer_part((*b).p1.y) - ty;
    let mut y2: libc::c_int = _cairo_fixed_integer_part((*b).p2.y) - ty;
    if y2 > y1 {
        if _cairo_fixed_is_integer((*b).p1.y) == 0 {
            do_unaligned_row(
                blt,
                closure,
                b,
                tx,
                y1,
                1 as libc::c_int,
                (256 as libc::c_int - _cairo_fixed_fractional_part((*b).p1.y))
                    as uint16_t,
            );
            y1 += 1;
        }
        if y2 > y1 {
            do_unaligned_row(
                blt,
                closure,
                b,
                tx,
                y1,
                y2 - y1,
                256 as libc::c_int as uint16_t,
            );
        }
        if _cairo_fixed_is_integer((*b).p2.y) == 0 {
            do_unaligned_row(
                blt,
                closure,
                b,
                tx,
                y2,
                1 as libc::c_int,
                _cairo_fixed_fractional_part((*b).p2.y) as uint16_t,
            );
        }
    } else {
        do_unaligned_row(
            blt,
            closure,
            b,
            tx,
            y1,
            1 as libc::c_int,
            ((*b).p2.y - (*b).p1.y) as uint16_t,
        );
    };
}
unsafe extern "C" fn blt_in(
    mut closure: *mut libc::c_void,
    mut x: int16_t,
    mut y: int16_t,
    mut w: int16_t,
    mut h: int16_t,
    mut coverage: uint16_t,
) {
    let mut info: *mut blt_in = closure as *mut blt_in;
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
    let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if coverage as libc::c_int == 0xffff as libc::c_int {
        return;
    }
    rect.x = x as libc::c_int;
    rect.y = y as libc::c_int;
    rect.width = w as libc::c_int;
    rect.height = h as libc::c_int;
    _cairo_color_init_rgba(
        &mut color,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        coverage as libc::c_int as libc::c_double
            / 0xffff as libc::c_int as libc::c_double,
    );
    ((*(*info).compositor).fill_rectangles)
        .expect(
            "non-null function pointer",
        )(
        (*info).dst as *mut libc::c_void,
        CAIRO_OPERATOR_IN,
        &mut color,
        &mut rect,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn create_composite_mask(
    mut compositor: *const cairo_mask_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut draw_closure: *mut libc::c_void,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> *mut cairo_surface_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut info: blt_in = blt_in {
        compositor: 0 as *const cairo_mask_compositor_t,
        dst: 0 as *mut cairo_surface_t,
    };
    let mut i: libc::c_int = 0;
    surface = _cairo_surface_create_scratch(
        dst,
        CAIRO_CONTENT_ALPHA,
        (*extents).bounded.width,
        (*extents).bounded.height,
        0 as *const cairo_color_t,
    );
    if (*surface).status as u64 != 0 {
        return surface;
    }
    status = ((*compositor).acquire)
        .expect("non-null function pointer")(surface as *mut libc::c_void);
    if status as u64 != 0 {
        cairo_surface_destroy(surface);
        return _cairo_int_surface_create_in_error(status);
    }
    if (*surface).is_clear() == 0 {
        let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        rect.y = 0 as libc::c_int;
        rect.x = rect.y;
        rect.width = (*extents).bounded.width;
        rect.height = (*extents).bounded.height;
        status = ((*compositor).fill_rectangles)
            .expect(
                "non-null function pointer",
            )(
            surface as *mut libc::c_void,
            CAIRO_OPERATOR_CLEAR,
            _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
            &mut rect,
            1 as libc::c_int,
        );
        if status as u64 != 0 {
            current_block = 14112124086624267904;
        } else {
            current_block = 7976072742316086414;
        }
    } else {
        current_block = 7976072742316086414;
    }
    match current_block {
        7976072742316086414 => {
            if mask_func.is_some() {
                status = mask_func
                    .expect(
                        "non-null function pointer",
                    )(
                    compositor,
                    surface,
                    draw_closure,
                    CAIRO_OPERATOR_SOURCE,
                    0 as *const cairo_pattern_t,
                    0 as *const cairo_rectangle_int_t,
                    (*extents).bounded.x,
                    (*extents).bounded.y,
                    &(*extents).bounded,
                    (*extents).clip,
                );
                if status as libc::c_uint
                    != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                {
                    current_block = 17978592414670926110;
                } else {
                    current_block = 6057473163062296781;
                }
            } else {
                current_block = 6057473163062296781;
            }
            match current_block {
                6057473163062296781 => {
                    status = draw_func
                        .expect(
                            "non-null function pointer",
                        )(
                        compositor,
                        surface,
                        draw_closure,
                        CAIRO_OPERATOR_ADD,
                        0 as *const cairo_pattern_t,
                        0 as *const cairo_rectangle_int_t,
                        (*extents).bounded.x,
                        (*extents).bounded.y,
                        &(*extents).bounded,
                        0 as *mut cairo_clip_t,
                    );
                    if status as u64 != 0 {
                        current_block = 14112124086624267904;
                    } else {
                        info.compositor = compositor;
                        info.dst = surface;
                        i = 0 as libc::c_int;
                        while i < (*(*extents).clip).num_boxes {
                            let mut b: *mut cairo_box_t = &mut *((*(*extents).clip)
                                .boxes)
                                .offset(i as isize) as *mut cairo_box_t;
                            if _cairo_fixed_is_integer((*b).p1.x) == 0
                                || _cairo_fixed_is_integer((*b).p1.y) == 0
                                || _cairo_fixed_is_integer((*b).p2.x) == 0
                                || _cairo_fixed_is_integer((*b).p2.y) == 0
                            {
                                do_unaligned_box(
                                    Some(
                                        blt_in
                                            as unsafe extern "C" fn(
                                                *mut libc::c_void,
                                                int16_t,
                                                int16_t,
                                                int16_t,
                                                int16_t,
                                                uint16_t,
                                            ) -> (),
                                    ),
                                    &mut info as *mut blt_in as *mut libc::c_void,
                                    b,
                                    (*extents).bounded.x,
                                    (*extents).bounded.y,
                                );
                            }
                            i += 1;
                        }
                        if !((*(*extents).clip).path).is_null() {
                            status = _cairo_clip_combine_with_surface(
                                (*extents).clip,
                                surface,
                                (*extents).bounded.x,
                                (*extents).bounded.y,
                            ) as cairo_int_status_t;
                            if status as u64 != 0 {
                                current_block = 14112124086624267904;
                            } else {
                                current_block = 17978592414670926110;
                            }
                        } else {
                            current_block = 17978592414670926110;
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                14112124086624267904 => {}
                _ => {
                    ((*compositor).release)
                        .expect(
                            "non-null function pointer",
                        )(surface as *mut libc::c_void);
                    (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
                    return surface;
                }
            }
        }
        _ => {}
    }
    ((*compositor).release)
        .expect("non-null function pointer")(surface as *mut libc::c_void);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        cairo_surface_destroy(surface);
        surface = _cairo_int_surface_create_in_error(status);
    }
    return surface;
}
unsafe extern "C" fn clip_and_composite_with_mask(
    mut compositor: *const cairo_mask_compositor_t,
    mut draw_closure: *mut libc::c_void,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut op: cairo_operator_t,
    mut pattern: *mut cairo_pattern_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
    mask = create_composite_mask(
        compositor,
        dst,
        draw_closure,
        draw_func,
        mask_func,
        extents,
    );
    if (*mask).status as u64 != 0 {
        return (*mask).status;
    }
    if !pattern.is_null()
        || (*dst).content as libc::c_uint
            != CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
    {
        src = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            &(*extents).source_pattern.base,
            0 as libc::c_int,
            &(*extents).bounded,
            &(*extents).source_sample_area,
            &mut src_x,
            &mut src_y,
        );
        if (*src).status as u64 != 0 {
            cairo_surface_destroy(mask);
            return (*src).status;
        }
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            src,
            mask,
            (*extents).bounded.x + src_x,
            (*extents).bounded.y + src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).bounded.x,
            (*extents).bounded.y,
            (*extents).bounded.width as libc::c_uint,
            (*extents).bounded.height as libc::c_uint,
        );
        cairo_surface_destroy(src);
    } else {
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            mask,
            0 as *mut cairo_surface_t,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).bounded.x,
            (*extents).bounded.y,
            (*extents).bounded.width as libc::c_uint,
            (*extents).bounded.height as libc::c_uint,
        );
    }
    cairo_surface_destroy(mask);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn get_clip_source(
    mut compositor: *const cairo_mask_compositor_t,
    mut clip: *mut cairo_clip_t,
    mut dst: *mut cairo_surface_t,
    mut bounds: *const cairo_rectangle_int_t,
    mut out_x: *mut libc::c_int,
    mut out_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
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
    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    surface = _cairo_clip_get_image(clip, dst, bounds);
    if (*surface).status as u64 != 0 {
        return surface;
    }
    _cairo_pattern_init_for_surface(&mut pattern, surface);
    pattern.base.filter = CAIRO_FILTER_NEAREST;
    cairo_surface_destroy(surface);
    r.y = 0 as libc::c_int;
    r.x = r.y;
    r.width = (*bounds).width;
    r.height = (*bounds).height;
    surface = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(dst, &mut pattern.base, 1 as libc::c_int, &mut r, &mut r, out_x, out_y);
    _cairo_pattern_fini(&mut pattern.base);
    *out_x += -(*bounds).x;
    *out_y += -(*bounds).y;
    return surface;
}
unsafe extern "C" fn clip_and_composite_combine(
    mut compositor: *const cairo_mask_compositor_t,
    mut draw_closure: *mut libc::c_void,
    mut draw_func: draw_func_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut tmp: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut clip: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut clip_x: libc::c_int = 0;
    let mut clip_y: libc::c_int = 0;
    tmp = _cairo_surface_create_scratch(
        dst,
        (*dst).content,
        (*extents).bounded.width,
        (*extents).bounded.height,
        0 as *const cairo_color_t,
    );
    if (*tmp).status as u64 != 0 {
        return (*tmp).status;
    }
    ((*compositor).composite)
        .expect(
            "non-null function pointer",
        )(
        tmp as *mut libc::c_void,
        CAIRO_OPERATOR_SOURCE,
        dst,
        0 as *mut cairo_surface_t,
        (*extents).bounded.x,
        (*extents).bounded.y,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        (*extents).bounded.width as libc::c_uint,
        (*extents).bounded.height as libc::c_uint,
    );
    status = draw_func
        .expect(
            "non-null function pointer",
        )(
        compositor,
        tmp,
        draw_closure,
        op,
        pattern,
        &(*extents).source_sample_area,
        (*extents).bounded.x,
        (*extents).bounded.y,
        &(*extents).bounded,
        0 as *mut cairo_clip_t,
    ) as cairo_status_t;
    if !(status as u64 != 0) {
        clip = get_clip_source(
            compositor,
            (*extents).clip,
            dst,
            &(*extents).bounded,
            &mut clip_x,
            &mut clip_y,
        );
        status = (*clip).status;
        if !(status as u64 != 0) {
            if (*dst).is_clear() != 0 {
                ((*compositor).composite)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst as *mut libc::c_void,
                    CAIRO_OPERATOR_SOURCE,
                    tmp,
                    clip,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    clip_x,
                    clip_y,
                    (*extents).bounded.x,
                    (*extents).bounded.y,
                    (*extents).bounded.width as libc::c_uint,
                    (*extents).bounded.height as libc::c_uint,
                );
            } else {
                ((*compositor).composite)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst as *mut libc::c_void,
                    CAIRO_OPERATOR_DEST_OUT,
                    clip,
                    0 as *mut cairo_surface_t,
                    clip_x,
                    clip_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*extents).bounded.x,
                    (*extents).bounded.y,
                    (*extents).bounded.width as libc::c_uint,
                    (*extents).bounded.height as libc::c_uint,
                );
                ((*compositor).composite)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst as *mut libc::c_void,
                    CAIRO_OPERATOR_ADD,
                    tmp,
                    clip,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    clip_x,
                    clip_y,
                    (*extents).bounded.x,
                    (*extents).bounded.y,
                    (*extents).bounded.width as libc::c_uint,
                    (*extents).bounded.height as libc::c_uint,
                );
            }
            cairo_surface_destroy(clip);
        }
    }
    cairo_surface_destroy(tmp);
    return status;
}
unsafe extern "C" fn clip_and_composite_source(
    mut compositor: *const cairo_mask_compositor_t,
    mut draw_closure: *mut libc::c_void,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut pattern: *mut cairo_pattern_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
    mask = create_composite_mask(
        compositor,
        dst,
        draw_closure,
        draw_func,
        mask_func,
        extents,
    );
    if (*mask).status as u64 != 0 {
        return (*mask).status;
    }
    src = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(
        dst,
        pattern,
        0 as libc::c_int,
        &(*extents).bounded,
        &(*extents).source_sample_area,
        &mut src_x,
        &mut src_y,
    );
    if (*src).status as u64 != 0 {
        cairo_surface_destroy(mask);
        return (*src).status;
    }
    if (*dst).is_clear() != 0 {
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_SOURCE,
            src,
            mask,
            (*extents).bounded.x + src_x,
            (*extents).bounded.y + src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).bounded.x,
            (*extents).bounded.y,
            (*extents).bounded.width as libc::c_uint,
            (*extents).bounded.height as libc::c_uint,
        );
    } else {
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_DEST_OUT,
            mask,
            0 as *mut cairo_surface_t,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).bounded.x,
            (*extents).bounded.y,
            (*extents).bounded.width as libc::c_uint,
            (*extents).bounded.height as libc::c_uint,
        );
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_ADD,
            src,
            mask,
            (*extents).bounded.x + src_x,
            (*extents).bounded.y + src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).bounded.x,
            (*extents).bounded.y,
            (*extents).bounded.width as libc::c_uint,
            (*extents).bounded.height as libc::c_uint,
        );
    }
    cairo_surface_destroy(src);
    cairo_surface_destroy(mask);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn can_reduce_alpha_op(mut op: cairo_operator_t) -> cairo_bool_t {
    let mut iop: libc::c_int = op as libc::c_int;
    match iop {
        2 | 1 | 12 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn reduce_alpha_op(
    mut dst: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    return ((*dst).is_clear() as libc::c_int != 0
        && (*dst).content as libc::c_uint
            == CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
        && _cairo_pattern_is_opaque_solid(pattern) != 0 && can_reduce_alpha_op(op) != 0)
        as libc::c_int;
}
unsafe extern "C" fn fixup_unbounded(
    mut compositor: *const cairo_mask_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut rects: [cairo_rectangle_int_t; 4] = [cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 4];
    let mut n: libc::c_int = 0;
    if (*extents).bounded.width == (*extents).unbounded.width
        && (*extents).bounded.height == (*extents).unbounded.height
    {
        return CAIRO_STATUS_SUCCESS;
    }
    n = 0 as libc::c_int;
    if (*extents).bounded.width == 0 as libc::c_int
        || (*extents).bounded.height == 0 as libc::c_int
    {
        rects[n as usize].x = (*extents).unbounded.x;
        rects[n as usize].width = (*extents).unbounded.width;
        rects[n as usize].y = (*extents).unbounded.y;
        rects[n as usize].height = (*extents).unbounded.height;
        n += 1;
    } else {
        if (*extents).bounded.y != (*extents).unbounded.y {
            rects[n as usize].x = (*extents).unbounded.x;
            rects[n as usize].width = (*extents).unbounded.width;
            rects[n as usize].y = (*extents).unbounded.y;
            rects[n as usize].height = (*extents).bounded.y - (*extents).unbounded.y;
            n += 1;
        }
        if (*extents).bounded.x != (*extents).unbounded.x {
            rects[n as usize].x = (*extents).unbounded.x;
            rects[n as usize].width = (*extents).bounded.x - (*extents).unbounded.x;
            rects[n as usize].y = (*extents).bounded.y;
            rects[n as usize].height = (*extents).bounded.height;
            n += 1;
        }
        if (*extents).bounded.x + (*extents).bounded.width
            != (*extents).unbounded.x + (*extents).unbounded.width
        {
            rects[n as usize].x = (*extents).bounded.x + (*extents).bounded.width;
            rects[n as usize]
                .width = (*extents).unbounded.x + (*extents).unbounded.width
                - rects[n as usize].x;
            rects[n as usize].y = (*extents).bounded.y;
            rects[n as usize].height = (*extents).bounded.height;
            n += 1;
        }
        if (*extents).bounded.y + (*extents).bounded.height
            != (*extents).unbounded.y + (*extents).unbounded.height
        {
            rects[n as usize].x = (*extents).unbounded.x;
            rects[n as usize].width = (*extents).unbounded.width;
            rects[n as usize].y = (*extents).bounded.y + (*extents).bounded.height;
            rects[n as usize]
                .height = (*extents).unbounded.y + (*extents).unbounded.height
                - rects[n as usize].y;
            n += 1;
        }
    }
    return ((*compositor).fill_rectangles)
        .expect(
            "non-null function pointer",
        )(
        dst as *mut libc::c_void,
        CAIRO_OPERATOR_CLEAR,
        _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
        rects.as_mut_ptr(),
        n,
    ) as cairo_status_t;
}
unsafe extern "C" fn fixup_unbounded_with_mask(
    mut compositor: *const cairo_mask_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut mask_x: libc::c_int = 0;
    let mut mask_y: libc::c_int = 0;
    mask = get_clip_source(
        compositor,
        (*extents).clip,
        dst,
        &(*extents).unbounded,
        &mut mask_x,
        &mut mask_y,
    );
    if (*mask).status as u64 != 0 {
        return (*mask).status;
    }
    if (*extents).bounded.y != (*extents).unbounded.y {
        let mut x: libc::c_int = (*extents).unbounded.x;
        let mut y: libc::c_int = (*extents).unbounded.y;
        let mut width: libc::c_int = (*extents).unbounded.width;
        let mut height: libc::c_int = (*extents).bounded.y - y;
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_DEST_OUT,
            mask,
            0 as *mut cairo_surface_t,
            x + mask_x,
            y + mask_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x,
            y,
            width as libc::c_uint,
            height as libc::c_uint,
        );
    }
    if (*extents).bounded.x != (*extents).unbounded.x {
        let mut x_0: libc::c_int = (*extents).unbounded.x;
        let mut y_0: libc::c_int = (*extents).bounded.y;
        let mut width_0: libc::c_int = (*extents).bounded.x - x_0;
        let mut height_0: libc::c_int = (*extents).bounded.height;
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_DEST_OUT,
            mask,
            0 as *mut cairo_surface_t,
            x_0 + mask_x,
            y_0 + mask_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x_0,
            y_0,
            width_0 as libc::c_uint,
            height_0 as libc::c_uint,
        );
    }
    if (*extents).bounded.x + (*extents).bounded.width
        != (*extents).unbounded.x + (*extents).unbounded.width
    {
        let mut x_1: libc::c_int = (*extents).bounded.x + (*extents).bounded.width;
        let mut y_1: libc::c_int = (*extents).bounded.y;
        let mut width_1: libc::c_int = (*extents).unbounded.x
            + (*extents).unbounded.width - x_1;
        let mut height_1: libc::c_int = (*extents).bounded.height;
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_DEST_OUT,
            mask,
            0 as *mut cairo_surface_t,
            x_1 + mask_x,
            y_1 + mask_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x_1,
            y_1,
            width_1 as libc::c_uint,
            height_1 as libc::c_uint,
        );
    }
    if (*extents).bounded.y + (*extents).bounded.height
        != (*extents).unbounded.y + (*extents).unbounded.height
    {
        let mut x_2: libc::c_int = (*extents).unbounded.x;
        let mut y_2: libc::c_int = (*extents).bounded.y + (*extents).bounded.height;
        let mut width_2: libc::c_int = (*extents).unbounded.width;
        let mut height_2: libc::c_int = (*extents).unbounded.y
            + (*extents).unbounded.height - y_2;
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_DEST_OUT,
            mask,
            0 as *mut cairo_surface_t,
            x_2 + mask_x,
            y_2 + mask_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x_2,
            y_2,
            width_2 as libc::c_uint,
            height_2 as libc::c_uint,
        );
    }
    cairo_surface_destroy(mask);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn fixup_unbounded_boxes(
    mut compositor: *const cairo_mask_compositor_t,
    mut extents: *const cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut clear: cairo_boxes_t = cairo_boxes_t {
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
    let mut clip_region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    if (*boxes).is_pixel_aligned != 0 {} else {
        __assert_fail(
            b"boxes->is_pixel_aligned\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mask-compositor.c\0" as *const u8 as *const libc::c_char,
            638 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"cairo_status_t fixup_unbounded_boxes(const cairo_mask_compositor_t *, const cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
    }
    clip_region = 0 as *mut cairo_region_t;
    if _cairo_clip_is_region((*extents).clip) != 0
        && {
            clip_region = _cairo_clip_get_region((*extents).clip);
            !clip_region.is_null()
        }
        && cairo_region_contains_rectangle(clip_region, &(*extents).bounded)
            as libc::c_uint == CAIRO_REGION_OVERLAP_IN as libc::c_int as libc::c_uint
    {
        clip_region = 0 as *mut cairo_region_t;
    }
    if (*boxes).num_boxes <= 1 as libc::c_int && clip_region.is_null() {
        return fixup_unbounded(compositor, dst, extents);
    }
    _cairo_boxes_init(&mut clear);
    box_0
        .p1
        .x = _cairo_fixed_from_int((*extents).unbounded.x + (*extents).unbounded.width);
    box_0.p1.y = _cairo_fixed_from_int((*extents).unbounded.y);
    box_0.p2.x = _cairo_fixed_from_int((*extents).unbounded.x);
    box_0
        .p2
        .y = _cairo_fixed_from_int((*extents).unbounded.y + (*extents).unbounded.height);
    if clip_region.is_null() {
        let mut tmp: cairo_boxes_t = cairo_boxes_t {
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
        _cairo_boxes_init(&mut tmp);
        status = _cairo_boxes_add(&mut tmp, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-mask-compositor.c\0" as *const u8 as *const libc::c_char,
                664 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"cairo_status_t fixup_unbounded_boxes(const cairo_mask_compositor_t *, const cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        tmp.chunks.next = &mut (*boxes).chunks;
        tmp.num_boxes += (*boxes).num_boxes;
        status = _cairo_bentley_ottmann_tessellate_boxes(
            &mut tmp,
            CAIRO_FILL_RULE_WINDING,
            &mut clear,
        );
        tmp.chunks.next = 0 as *mut _cairo_boxes_chunk;
    } else {
        let mut pbox: *mut pixman_box32_t = 0 as *mut pixman_box32_t;
        pbox = pixman_region32_rectangles(&mut (*clip_region).rgn, &mut i);
        _cairo_boxes_limit(&mut clear, pbox as *mut cairo_box_t, i);
        status = _cairo_boxes_add(&mut clear, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-mask-compositor.c\0" as *const u8 as *const libc::c_char,
                681 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"cairo_status_t fixup_unbounded_boxes(const cairo_mask_compositor_t *, const cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                status = _cairo_boxes_add(
                    &mut clear,
                    CAIRO_ANTIALIAS_DEFAULT,
                    &mut *((*chunk).base).offset(i as isize),
                );
                if status as u64 != 0 {
                    _cairo_boxes_fini(&mut clear);
                    return status;
                }
                i += 1;
            }
            chunk = (*chunk).next;
        }
        status = _cairo_bentley_ottmann_tessellate_boxes(
            &mut clear,
            CAIRO_FILL_RULE_WINDING,
            &mut clear,
        );
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = ((*compositor).fill_boxes)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            CAIRO_OPERATOR_CLEAR,
            _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
            &mut clear,
        ) as cairo_status_t;
    }
    _cairo_boxes_fini(&mut clear);
    return status;
}
unsafe extern "C" fn need_bounded_clip(
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut flags: libc::c_uint = NEED_CLIP_REGION as libc::c_int as libc::c_uint;
    if _cairo_clip_is_region((*extents).clip) == 0 {
        flags |= NEED_CLIP_SURFACE as libc::c_int as libc::c_uint;
    }
    return flags as cairo_bool_t;
}
unsafe extern "C" fn need_unbounded_clip(
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*extents).is_bounded == 0 {
        flags |= NEED_CLIP_REGION as libc::c_int as libc::c_uint;
        if _cairo_clip_is_region((*extents).clip) == 0 {
            flags |= NEED_CLIP_SURFACE as libc::c_int as libc::c_uint;
        }
    }
    if !((*(*extents).clip).path).is_null() {
        flags |= NEED_CLIP_SURFACE as libc::c_int as libc::c_uint;
    }
    return flags as cairo_bool_t;
}
unsafe extern "C" fn clip_and_composite(
    mut compositor: *const cairo_mask_compositor_t,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut extents: *mut cairo_composite_rectangles_t,
    mut need_clip: libc::c_uint,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut op: cairo_operator_t = (*extents).op;
    let mut src: *mut cairo_pattern_t = &mut (*extents).source_pattern.base;
    let mut clip_region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    ((*compositor).acquire)
        .expect("non-null function pointer")(dst as *mut libc::c_void);
    if need_clip & NEED_CLIP_REGION as libc::c_int as libc::c_uint != 0 {
        clip_region = _cairo_clip_get_region((*extents).clip);
        if need_clip & FORCE_CLIP_REGION as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            && _cairo_composite_rectangles_can_reduce_clip(extents, (*extents).clip) != 0
        {
            clip_region = 0 as *mut cairo_region_t;
        }
        if !clip_region.is_null() {
            status = ((*compositor).set_clip_region)
                .expect(
                    "non-null function pointer",
                )(dst as *mut libc::c_void, clip_region) as cairo_status_t;
            if status as u64 != 0 {
                ((*compositor).release)
                    .expect("non-null function pointer")(dst as *mut libc::c_void);
                return status;
            }
        }
    }
    if reduce_alpha_op(dst, op, &mut (*extents).source_pattern.base) != 0 {
        op = CAIRO_OPERATOR_ADD;
        src = 0 as *mut cairo_pattern_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
        status = clip_and_composite_source(
            compositor,
            draw_closure,
            draw_func,
            mask_func,
            src,
            extents,
        );
    } else {
        if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
            op = CAIRO_OPERATOR_DEST_OUT;
            src = 0 as *mut cairo_pattern_t;
        }
        if need_clip & NEED_CLIP_SURFACE as libc::c_int as libc::c_uint != 0 {
            if (*extents).is_bounded != 0 {
                status = clip_and_composite_with_mask(
                    compositor,
                    draw_closure,
                    draw_func,
                    mask_func,
                    op,
                    src,
                    extents,
                );
            } else {
                status = clip_and_composite_combine(
                    compositor,
                    draw_closure,
                    draw_func,
                    op,
                    src,
                    extents,
                );
            }
        } else {
            status = draw_func
                .expect(
                    "non-null function pointer",
                )(
                compositor,
                dst,
                draw_closure,
                op,
                src,
                &mut (*extents).source_sample_area,
                0 as libc::c_int,
                0 as libc::c_int,
                &mut (*extents).bounded,
                (*extents).clip,
            ) as cairo_status_t;
        }
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*extents).is_bounded == 0
    {
        if need_clip & NEED_CLIP_SURFACE as libc::c_int as libc::c_uint != 0 {
            status = fixup_unbounded_with_mask(compositor, dst, extents);
        } else {
            status = fixup_unbounded(compositor, dst, extents);
        }
    }
    if !clip_region.is_null() {
        ((*compositor).set_clip_region)
            .expect(
                "non-null function pointer",
            )(dst as *mut libc::c_void, 0 as *mut cairo_region_t);
    }
    ((*compositor).release)
        .expect("non-null function pointer")(dst as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn trim_extents_to_boxes(
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    _cairo_boxes_extents(boxes, &mut box_0);
    return _cairo_composite_rectangles_intersect_mask_extents(extents, &mut box_0);
}
unsafe extern "C" fn upload_boxes(
    mut compositor: *const cairo_mask_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut source: *const cairo_pattern_t = &mut (*extents).source_pattern.base;
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut limit: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    src = _cairo_pattern_get_source(source as *mut cairo_surface_pattern_t, &mut limit);
    if !((*src).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        || (*src).type_0 as libc::c_uint == (*dst).type_0 as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_matrix_is_integer_translation(&(*source).matrix, &mut tx, &mut ty) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*extents).bounded.x + tx < limit.x || (*extents).bounded.y + ty < limit.y {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*extents).bounded.x + (*extents).bounded.width + tx > limit.x + limit.width
        || (*extents).bounded.y + (*extents).bounded.height + ty > limit.y + limit.height
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    tx += limit.x;
    ty += limit.y;
    if (*src).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
    {
        status = ((*compositor).draw_image_boxes)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            src as *mut cairo_image_surface_t,
            boxes,
            tx,
            ty,
        );
    } else {
        status = ((*compositor).copy_boxes)
            .expect(
                "non-null function pointer",
            )(dst as *mut libc::c_void, src, boxes, &mut (*extents).bounded, tx, ty);
    }
    return status as cairo_status_t;
}
unsafe extern "C" fn composite_boxes(
    mut compositor: *const cairo_mask_compositor_t,
    mut extents: *const cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut op: cairo_operator_t = (*extents).op;
    let mut source: *const cairo_pattern_t = &(*extents).source_pattern.base;
    let mut need_clip_mask: cairo_bool_t = ((*(*extents).clip).path
        != 0 as *mut libc::c_void as *mut cairo_clip_path_t) as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if need_clip_mask != 0
        && ((*extents).is_bounded == 0
            || op as libc::c_uint
                == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = ((*compositor).acquire)
        .expect("non-null function pointer")(dst as *mut libc::c_void) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    if need_clip_mask == 0
        && (*source).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        let mut color: *const cairo_color_t = 0 as *const cairo_color_t;
        color = &mut (*(source as *mut cairo_solid_pattern_t)).color;
        status = ((*compositor).fill_boxes)
            .expect(
                "non-null function pointer",
            )(dst as *mut libc::c_void, op, color, boxes) as cairo_status_t;
    } else {
        let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        let mut src_x: libc::c_int = 0;
        let mut src_y: libc::c_int = 0;
        let mut mask_x: libc::c_int = 0 as libc::c_int;
        let mut mask_y: libc::c_int = 0 as libc::c_int;
        if need_clip_mask != 0 {
            mask = get_clip_source(
                compositor,
                (*extents).clip,
                dst,
                &(*extents).bounded,
                &mut mask_x,
                &mut mask_y,
            );
            if (*mask).status as u64 != 0 {
                return (*mask).status;
            }
            if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            {
                source = 0 as *const cairo_pattern_t;
                op = CAIRO_OPERATOR_DEST_OUT;
            }
        }
        if !source.is_null() || mask.is_null() {
            src = ((*compositor).pattern_to_surface)
                .expect(
                    "non-null function pointer",
                )(
                dst,
                source,
                0 as libc::c_int,
                &(*extents).bounded,
                &(*extents).source_sample_area,
                &mut src_x,
                &mut src_y,
            );
        } else {
            src = mask;
            src_x = mask_x;
            src_y = mask_y;
            mask = 0 as *mut cairo_surface_t;
        }
        status = ((*compositor).composite_boxes)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            src,
            mask,
            src_x,
            src_y,
            mask_x,
            mask_y,
            0 as libc::c_int,
            0 as libc::c_int,
            boxes,
            &(*extents).bounded,
        ) as cairo_status_t;
        cairo_surface_destroy(src);
        cairo_surface_destroy(mask);
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*extents).is_bounded == 0
    {
        status = fixup_unbounded_boxes(compositor, extents, boxes);
    }
    ((*compositor).release)
        .expect("non-null function pointer")(dst as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn clip_and_composite_boxes(
    mut compositor: *const cairo_mask_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*boxes).num_boxes == 0 as libc::c_int {
        if (*extents).is_bounded != 0 {
            return CAIRO_STATUS_SUCCESS;
        }
        return fixup_unbounded_boxes(compositor, extents, boxes);
    }
    if (*boxes).is_pixel_aligned == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = trim_extents_to_boxes(extents, boxes);
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if (*extents).source_pattern.base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && ((*(*extents).clip).path).is_null()
        && ((*extents).op as libc::c_uint
            == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || (*dst).is_clear() as libc::c_int != 0
                && ((*extents).op as libc::c_uint
                    == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                    || (*extents).op as libc::c_uint
                        == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint))
    {
        status = upload_boxes(compositor, extents, boxes) as cairo_int_status_t;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    return composite_boxes(compositor, extents, boxes);
}
unsafe extern "C" fn _cairo_mask_compositor_paint(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut compositor: *mut cairo_mask_compositor_t = _compositor
        as *mut cairo_mask_compositor_t;
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    _cairo_clip_steal_boxes((*extents).clip, &mut boxes);
    status = clip_and_composite_boxes(compositor, extents, &mut boxes)
        as cairo_int_status_t;
    _cairo_clip_unsteal_boxes((*extents).clip, &mut boxes);
    return status;
}
unsafe extern "C" fn composite_opacity(
    mut closure: *mut libc::c_void,
    mut x: int16_t,
    mut y: int16_t,
    mut w: int16_t,
    mut h: int16_t,
    mut coverage: uint16_t,
) {
    let mut info: *mut composite_opacity_info = closure as *mut composite_opacity_info;
    let mut compositor: *const cairo_mask_compositor_t = (*info).compositor;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut mask_x: libc::c_int = 0;
    let mut mask_y: libc::c_int = 0;
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
    let mut solid: cairo_solid_pattern_t = cairo_solid_pattern_t {
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
    _cairo_color_init_rgba(
        &mut color,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        (*info).opacity * coverage as libc::c_int as libc::c_double,
    );
    _cairo_pattern_init_solid(&mut solid, &mut color);
    mask = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(
        (*info).dst,
        &mut solid.base,
        1 as libc::c_int,
        &_cairo_unbounded_rectangle,
        &_cairo_unbounded_rectangle,
        &mut mask_x,
        &mut mask_y,
    );
    if (*mask).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        if !((*info).src).is_null() {
            ((*compositor).composite)
                .expect(
                    "non-null function pointer",
                )(
                (*info).dst as *mut libc::c_void,
                (*info).op as cairo_operator_t,
                (*info).src,
                mask,
                x as libc::c_int + (*info).src_x,
                y as libc::c_int + (*info).src_y,
                mask_x,
                mask_y,
                x as libc::c_int,
                y as libc::c_int,
                w as libc::c_uint,
                h as libc::c_uint,
            );
        } else {
            ((*compositor).composite)
                .expect(
                    "non-null function pointer",
                )(
                (*info).dst as *mut libc::c_void,
                (*info).op as cairo_operator_t,
                mask,
                0 as *mut cairo_surface_t,
                mask_x,
                mask_y,
                0 as libc::c_int,
                0 as libc::c_int,
                x as libc::c_int,
                y as libc::c_int,
                w as libc::c_uint,
                h as libc::c_uint,
            );
        }
    }
    cairo_surface_destroy(mask);
}
unsafe extern "C" fn composite_opacity_boxes(
    mut compositor: *const cairo_mask_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut src_sample: *const cairo_rectangle_int_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut mask_pattern: *const cairo_solid_pattern_t = closure
        as *const cairo_solid_pattern_t;
    let mut info: composite_opacity_info = composite_opacity_info {
        compositor: 0 as *const cairo_mask_compositor_t,
        op: 0,
        dst: 0 as *mut cairo_surface_t,
        src: 0 as *mut cairo_surface_t,
        src_x: 0,
        src_y: 0,
        opacity: 0.,
    };
    let mut i: libc::c_int = 0;
    if !clip.is_null() {} else {
        __assert_fail(
            b"clip\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mask-compositor.c\0" as *const u8 as *const libc::c_char,
            1069 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 234],
                &[libc::c_char; 234],
            >(
                b"cairo_int_status_t composite_opacity_boxes(const cairo_mask_compositor_t *, cairo_surface_t *, void *, cairo_operator_t, const cairo_pattern_t *, const cairo_rectangle_int_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    info.compositor = compositor;
    info.op = op as uint8_t;
    info.dst = dst;
    if !src_pattern.is_null() {
        info
            .src = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            src_pattern,
            0 as libc::c_int,
            extents,
            src_sample,
            &mut info.src_x,
            &mut info.src_y,
        );
        if (*info.src).status as u64 != 0 {
            return (*info.src).status as cairo_int_status_t;
        }
    } else {
        info.src = 0 as *mut cairo_surface_t;
    }
    info.opacity = (*mask_pattern).color.alpha / 0xffff as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        do_unaligned_box(
            Some(
                composite_opacity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        int16_t,
                        int16_t,
                        int16_t,
                        int16_t,
                        uint16_t,
                    ) -> (),
            ),
            &mut info as *mut composite_opacity_info as *mut libc::c_void,
            &mut *((*clip).boxes).offset(i as isize),
            dst_x,
            dst_y,
        );
        i += 1;
    }
    cairo_surface_destroy(info.src);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_box(
    mut closure: *mut libc::c_void,
    mut x: int16_t,
    mut y: int16_t,
    mut w: int16_t,
    mut h: int16_t,
    mut coverage: uint16_t,
) {
    let mut info: *mut composite_box_info = closure as *mut composite_box_info;
    let mut compositor: *const cairo_mask_compositor_t = (*info).compositor;
    if !(coverage as libc::c_int >= 0xff00 as libc::c_int) {
        let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
        let mut solid: cairo_solid_pattern_t = cairo_solid_pattern_t {
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
        let mut mask_x: libc::c_int = 0;
        let mut mask_y: libc::c_int = 0;
        _cairo_color_init_rgba(
            &mut color,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            coverage as libc::c_int as libc::c_double
                / 0xffff as libc::c_int as libc::c_double,
        );
        _cairo_pattern_init_solid(&mut solid, &mut color);
        mask = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            (*info).dst,
            &mut solid.base,
            0 as libc::c_int,
            &_cairo_unbounded_rectangle,
            &_cairo_unbounded_rectangle,
            &mut mask_x,
            &mut mask_y,
        );
        if (*mask).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            ((*compositor).composite)
                .expect(
                    "non-null function pointer",
                )(
                (*info).dst as *mut libc::c_void,
                (*info).op as cairo_operator_t,
                (*info).src,
                mask,
                x as libc::c_int + (*info).src_x,
                y as libc::c_int + (*info).src_y,
                mask_x,
                mask_y,
                x as libc::c_int,
                y as libc::c_int,
                w as libc::c_uint,
                h as libc::c_uint,
            );
        }
        cairo_surface_destroy(mask);
    } else {
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            (*info).dst as *mut libc::c_void,
            (*info).op as cairo_operator_t,
            (*info).src,
            0 as *mut cairo_surface_t,
            x as libc::c_int + (*info).src_x,
            y as libc::c_int + (*info).src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x as libc::c_int,
            y as libc::c_int,
            w as libc::c_uint,
            h as libc::c_uint,
        );
    };
}
unsafe extern "C" fn composite_mask_clip_boxes(
    mut compositor: *const cairo_mask_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut src_sample: *const cairo_rectangle_int_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut composite: *mut cairo_composite_rectangles_t = closure
        as *mut cairo_composite_rectangles_t;
    let mut info: composite_box_info = composite_box_info {
        compositor: 0 as *const cairo_mask_compositor_t,
        dst: 0 as *mut cairo_surface_t,
        src: 0 as *mut cairo_surface_t,
        src_x: 0,
        src_y: 0,
        op: 0,
    };
    let mut i: libc::c_int = 0;
    if src_pattern.is_null() {} else {
        __assert_fail(
            b"src_pattern == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mask-compositor.c\0" as *const u8 as *const libc::c_char,
            1159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 236],
                &[libc::c_char; 236],
            >(
                b"cairo_int_status_t composite_mask_clip_boxes(const cairo_mask_compositor_t *, cairo_surface_t *, void *, cairo_operator_t, const cairo_pattern_t *, const cairo_rectangle_int_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"op == CAIRO_OPERATOR_SOURCE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mask-compositor.c\0" as *const u8 as *const libc::c_char,
            1160 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 236],
                &[libc::c_char; 236],
            >(
                b"cairo_int_status_t composite_mask_clip_boxes(const cairo_mask_compositor_t *, cairo_surface_t *, void *, cairo_operator_t, const cairo_pattern_t *, const cairo_rectangle_int_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    info.compositor = compositor;
    info.op = CAIRO_OPERATOR_SOURCE as libc::c_int as uint8_t;
    info.dst = dst;
    info
        .src = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(
        dst,
        &mut (*composite).mask_pattern.base,
        0 as libc::c_int,
        extents,
        &mut (*composite).mask_sample_area,
        &mut info.src_x,
        &mut info.src_y,
    );
    if (*info.src).status as u64 != 0 {
        return (*info.src).status as cairo_int_status_t;
    }
    info.src_x += dst_x;
    info.src_y += dst_y;
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        do_unaligned_box(
            Some(
                composite_box
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        int16_t,
                        int16_t,
                        int16_t,
                        int16_t,
                        uint16_t,
                    ) -> (),
            ),
            &mut info as *mut composite_box_info as *mut libc::c_void,
            &mut *((*clip).boxes).offset(i as isize),
            dst_x,
            dst_y,
        );
        i += 1;
    }
    cairo_surface_destroy(info.src);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_mask(
    mut compositor: *const cairo_mask_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut src_sample: *const cairo_rectangle_int_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut composite: *mut cairo_composite_rectangles_t = closure
        as *mut cairo_composite_rectangles_t;
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
    let mut mask_x: libc::c_int = 0;
    let mut mask_y: libc::c_int = 0;
    if !src_pattern.is_null() {
        src = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            src_pattern,
            0 as libc::c_int,
            extents,
            src_sample,
            &mut src_x,
            &mut src_y,
        );
        if (*src).status as u64 != 0 {
            return (*src).status as cairo_int_status_t;
        }
        mask = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            &mut (*composite).mask_pattern.base,
            1 as libc::c_int,
            extents,
            &mut (*composite).mask_sample_area,
            &mut mask_x,
            &mut mask_y,
        );
        if (*mask).status as u64 != 0 {
            cairo_surface_destroy(src);
            return (*mask).status as cairo_int_status_t;
        }
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            src,
            mask,
            (*extents).x + src_x,
            (*extents).y + src_y,
            (*extents).x + mask_x,
            (*extents).y + mask_y,
            (*extents).x - dst_x,
            (*extents).y - dst_y,
            (*extents).width as libc::c_uint,
            (*extents).height as libc::c_uint,
        );
        cairo_surface_destroy(mask);
        cairo_surface_destroy(src);
    } else {
        src = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            &mut (*composite).mask_pattern.base,
            0 as libc::c_int,
            extents,
            &mut (*composite).mask_sample_area,
            &mut src_x,
            &mut src_y,
        );
        if (*src).status as u64 != 0 {
            return (*src).status as cairo_int_status_t;
        }
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            src,
            0 as *mut cairo_surface_t,
            (*extents).x + src_x,
            (*extents).y + src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).x - dst_x,
            (*extents).y - dst_y,
            (*extents).width as libc::c_uint,
            (*extents).height as libc::c_uint,
        );
        cairo_surface_destroy(src);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_mask_compositor_mask(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_mask_compositor_t = _compositor
        as *mut cairo_mask_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_UNSUPPORTED;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    if (*extents).mask_pattern.base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
        && ((*(*extents).clip).path).is_null()
        && _cairo_clip_is_region((*extents).clip) != 0
    {
        status = clip_and_composite(
            compositor,
            Some(
                composite_opacity_boxes
                    as unsafe extern "C" fn(
                        *const cairo_mask_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_rectangle_int_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            Some(
                composite_opacity_boxes
                    as unsafe extern "C" fn(
                        *const cairo_mask_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_rectangle_int_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            &mut (*extents).mask_pattern.solid as *mut cairo_solid_pattern_t
                as *mut libc::c_void,
            extents,
            need_unbounded_clip(extents) as libc::c_uint,
        ) as cairo_int_status_t;
    } else {
        status = clip_and_composite(
            compositor,
            Some(
                composite_mask
                    as unsafe extern "C" fn(
                        *const cairo_mask_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_rectangle_int_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            if ((*(*extents).clip).path).is_null() {
                Some(
                    composite_mask_clip_boxes
                        as unsafe extern "C" fn(
                            *const cairo_mask_compositor_t,
                            *mut cairo_surface_t,
                            *mut libc::c_void,
                            cairo_operator_t,
                            *const cairo_pattern_t,
                            *const cairo_rectangle_int_t,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_rectangle_int_t,
                            *mut cairo_clip_t,
                        ) -> cairo_int_status_t,
                )
            } else {
                None
            },
            extents as *mut libc::c_void,
            extents,
            need_bounded_clip(extents) as libc::c_uint,
        ) as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn _cairo_mask_compositor_stroke(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_mask_compositor_t = _compositor
        as *mut cairo_mask_compositor_t;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_UNSUPPORTED;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    if _cairo_path_fixed_stroke_is_rectilinear(path) != 0 {
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
            status = clip_and_composite_boxes(compositor, extents, &mut boxes)
                as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        mask = cairo_surface_create_similar_image(
            (*extents).surface,
            CAIRO_FORMAT_A8,
            (*extents).bounded.width,
            (*extents).bounded.height,
        );
        if (*mask).status as u64 != 0 {
            return (*mask).status as cairo_int_status_t;
        }
        status = _cairo_surface_offset_stroke(
            mask,
            (*extents).bounded.x,
            (*extents).bounded.y,
            CAIRO_OPERATOR_ADD,
            &_cairo_pattern_white.base,
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            antialias,
            (*extents).clip,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            cairo_surface_destroy(mask);
            return status;
        }
        _cairo_pattern_init_for_surface(&mut pattern, mask);
        cairo_surface_destroy(mask);
        cairo_matrix_init_translate(
            &mut pattern.base.matrix,
            -(*extents).bounded.x as libc::c_double,
            -(*extents).bounded.y as libc::c_double,
        );
        pattern.base.filter = CAIRO_FILTER_NEAREST;
        pattern.base.extend = CAIRO_EXTEND_NONE;
        status = _cairo_surface_mask(
            (*extents).surface,
            (*extents).op,
            &mut (*extents).source_pattern.base,
            &mut pattern.base,
            (*extents).clip,
        ) as cairo_int_status_t;
        _cairo_pattern_fini(&mut pattern.base);
    }
    return status;
}
unsafe extern "C" fn _cairo_mask_compositor_fill(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_mask_compositor_t = _compositor
        as *mut cairo_mask_compositor_t;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_UNSUPPORTED;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    if _cairo_path_fixed_fill_is_rectilinear(path) != 0 {
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
            status = clip_and_composite_boxes(compositor, extents, &mut boxes)
                as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        mask = cairo_surface_create_similar_image(
            (*extents).surface,
            CAIRO_FORMAT_A8,
            (*extents).bounded.width,
            (*extents).bounded.height,
        );
        if (*mask).status as u64 != 0 {
            return (*mask).status as cairo_int_status_t;
        }
        status = _cairo_surface_offset_fill(
            mask,
            (*extents).bounded.x,
            (*extents).bounded.y,
            CAIRO_OPERATOR_ADD,
            &_cairo_pattern_white.base,
            path,
            fill_rule,
            tolerance,
            antialias,
            (*extents).clip,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            cairo_surface_destroy(mask);
            return status;
        }
        _cairo_pattern_init_for_surface(&mut pattern, mask);
        cairo_surface_destroy(mask);
        cairo_matrix_init_translate(
            &mut pattern.base.matrix,
            -(*extents).bounded.x as libc::c_double,
            -(*extents).bounded.y as libc::c_double,
        );
        pattern.base.filter = CAIRO_FILTER_NEAREST;
        pattern.base.extend = CAIRO_EXTEND_NONE;
        status = _cairo_surface_mask(
            (*extents).surface,
            (*extents).op,
            &mut (*extents).source_pattern.base,
            &mut pattern.base,
            (*extents).clip,
        ) as cairo_int_status_t;
        _cairo_pattern_fini(&mut pattern.base);
    }
    return status;
}
unsafe extern "C" fn _cairo_mask_compositor_glyphs(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut overlap: cairo_bool_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_mask_compositor_t = _compositor
        as *mut cairo_mask_compositor_t;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    mask = cairo_surface_create_similar_image(
        (*extents).surface,
        CAIRO_FORMAT_A8,
        (*extents).bounded.width,
        (*extents).bounded.height,
    );
    if (*mask).status as u64 != 0 {
        return (*mask).status as cairo_int_status_t;
    }
    status = _cairo_surface_offset_glyphs(
        mask,
        (*extents).bounded.x,
        (*extents).bounded.y,
        CAIRO_OPERATOR_ADD,
        &_cairo_pattern_white.base,
        scaled_font,
        glyphs,
        num_glyphs,
        (*extents).clip,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        cairo_surface_destroy(mask);
        return status;
    }
    _cairo_pattern_init_for_surface(&mut pattern, mask);
    cairo_surface_destroy(mask);
    cairo_matrix_init_translate(
        &mut pattern.base.matrix,
        -(*extents).bounded.x as libc::c_double,
        -(*extents).bounded.y as libc::c_double,
    );
    pattern.base.filter = CAIRO_FILTER_NEAREST;
    pattern.base.extend = CAIRO_EXTEND_NONE;
    status = _cairo_surface_mask(
        (*extents).surface,
        (*extents).op,
        &mut (*extents).source_pattern.base,
        &mut pattern.base,
        (*extents).clip,
    ) as cairo_int_status_t;
    _cairo_pattern_fini(&mut pattern.base);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mask_compositor_init(
    mut compositor: *mut cairo_mask_compositor_t,
    mut delegate: *const cairo_compositor_t,
) {
    let ref mut fresh3 = (*compositor).base.delegate;
    *fresh3 = delegate;
    let ref mut fresh4 = (*compositor).base.paint;
    *fresh4 = Some(
        _cairo_mask_compositor_paint
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
            ) -> cairo_int_status_t,
    );
    let ref mut fresh5 = (*compositor).base.mask;
    *fresh5 = Some(
        _cairo_mask_compositor_mask
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
            ) -> cairo_int_status_t,
    );
    let ref mut fresh6 = (*compositor).base.fill;
    *fresh6 = Some(
        _cairo_mask_compositor_fill
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
                *const cairo_path_fixed_t,
                cairo_fill_rule_t,
                libc::c_double,
                cairo_antialias_t,
            ) -> cairo_int_status_t,
    );
    let ref mut fresh7 = (*compositor).base.stroke;
    *fresh7 = Some(
        _cairo_mask_compositor_stroke
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
    let ref mut fresh8 = (*compositor).base.glyphs;
    *fresh8 = Some(
        _cairo_mask_compositor_glyphs
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
                *mut cairo_scaled_font_t,
                *mut cairo_glyph_t,
                libc::c_int,
                cairo_bool_t,
            ) -> cairo_int_status_t,
    );
}
