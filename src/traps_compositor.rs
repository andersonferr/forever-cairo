use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type pixman_image;
    pub type _cairo_hash_table;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn cairo_region_contains_rectangle(
        region: *const cairo_region_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_region_overlap_t;
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
    fn _cairo_boxes_init_from_rectangle(
        boxes: *mut cairo_boxes_t,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_extents(boxes: *const cairo_boxes_t, box_0: *mut cairo_box_t);
    fn _cairo_boxes_intersect(
        a: *const cairo_boxes_t,
        b: *const cairo_boxes_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_rasterise_polygon_to_boxes(
        polygon: *mut cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_fini(boxes: *mut cairo_boxes_t);
    fn _cairo_clip_path_destroy(clip_path: *mut cairo_clip_path_t);
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    static __cairo_clip_all: cairo_clip_t;
    fn _cairo_clip_copy(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_clip_copy_region(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_clip_intersect_boxes(
        clip: *mut cairo_clip_t,
        boxes: *const cairo_boxes_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_clip_combine_with_surface(
        clip: *const cairo_clip_t,
        dst: *mut cairo_surface_t,
        dst_x: libc::c_int,
        dst_y: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_clip_from_boxes(boxes: *const cairo_boxes_t) -> *mut cairo_clip_t;
    fn _cairo_clip_get_region(clip: *const cairo_clip_t) -> *mut cairo_region_t;
    fn _cairo_clip_is_region(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _cairo_clip_get_polygon(
        clip: *const cairo_clip_t,
        polygon: *mut cairo_polygon_t,
        fill_rule: *mut cairo_fill_rule_t,
        antialias: *mut cairo_antialias_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_color_init_rgba(
        color: *mut cairo_color_t,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
        alpha: libc::c_double,
    );
    fn _cairo_path_fixed_fill_to_polygon(
        path: *const cairo_path_fixed_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_fill_rectilinear_to_boxes(
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_to_polygon(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_to_tristrip(
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        strip: *mut cairo_tristrip_t,
    ) -> cairo_int_status_t;
    fn _cairo_path_fixed_stroke_rectilinear_to_boxes(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_int_status_t;
    fn _cairo_path_fixed_stroke_to_traps(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        traps: *mut cairo_traps_t,
    ) -> cairo_int_status_t;
    fn _cairo_path_fixed_stroke_polygon_to_traps(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        traps: *mut cairo_traps_t,
    ) -> cairo_int_status_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_surface_create_scratch(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
        color: *const cairo_color_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_has_device_transform(
        surface: *mut cairo_surface_t,
    ) -> cairo_bool_t;
    fn _cairo_polygon_init_with_clip(
        polygon: *mut cairo_polygon_t,
        clip: *const cairo_clip_t,
    );
    fn _cairo_polygon_fini(polygon: *mut cairo_polygon_t);
    fn _cairo_polygon_intersect(
        a: *mut cairo_polygon_t,
        winding_a: libc::c_int,
        b: *mut cairo_polygon_t,
        winding_b: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn _cairo_bentley_ottmann_tessellate_polygon(
        traps: *mut cairo_traps_t,
        polygon: *const cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_bentley_ottmann_tessellate_boxes(
        in_0: *const cairo_boxes_t,
        fill_rule: cairo_fill_rule_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_init_solid(
        pattern: *mut cairo_solid_pattern_t,
        color: *const cairo_color_t,
    );
    fn _cairo_pattern_is_opaque_solid(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_composite_rectangles_intersect_mask_extents(
        extents: *mut cairo_composite_rectangles_t,
        box_0: *const cairo_box_t,
    ) -> cairo_int_status_t;
    fn _cairo_recording_surface_replay_with_clip(
        surface: *mut cairo_surface_t,
        surface_transform: *const cairo_matrix_t,
        target: *mut cairo_surface_t,
        target_clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_traps_init(traps: *mut cairo_traps_t);
    fn _cairo_traps_init_with_clip(traps: *mut cairo_traps_t, clip: *const cairo_clip_t);
    fn _cairo_traps_init_boxes(
        traps: *mut cairo_traps_t,
        boxes: *const cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_traps_fini(traps: *mut cairo_traps_t);
    fn _cairo_traps_extents(traps: *const cairo_traps_t, extents: *mut cairo_box_t);
    fn _cairo_traps_to_boxes(
        traps: *mut cairo_traps_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_bool_t;
    fn _cairo_rasterise_polygon_to_traps(
        polygon: *mut cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
        antialias: cairo_antialias_t,
        traps: *mut cairo_traps_t,
    ) -> cairo_int_status_t;
    fn _cairo_tristrip_init_with_clip(
        strip: *mut cairo_tristrip_t,
        clip: *const cairo_clip_t,
    );
    fn _cairo_tristrip_extents(
        strip: *const cairo_tristrip_t,
        extents: *mut cairo_box_t,
    );
    fn _cairo_tristrip_fini(strip: *mut cairo_tristrip_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_traps {
    pub status: cairo_status_t,
    pub bounds: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    #[bitfield(name = "maybe_region", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_intersections", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_rectilinear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "is_rectangular", ty = "libc::c_uint", bits = "3..=3")]
    pub maybe_region_has_intersections_is_rectilinear_is_rectangular: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub num_traps: libc::c_int,
    pub traps_size: libc::c_int,
    pub traps: *mut cairo_trapezoid_t,
    pub traps_embedded: [cairo_trapezoid_t; 16],
}
pub type cairo_trapezoid_t = _cairo_trapezoid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_trapezoid {
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub left: cairo_line_t,
    pub right: cairo_line_t,
}
pub type cairo_line_t = _cairo_line;
pub type cairo_traps_t = _cairo_traps;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tristrip {
    pub status: cairo_status_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_points: libc::c_int,
    pub size_points: libc::c_int,
    pub points: *mut cairo_point_t,
    pub points_embedded: [cairo_point_t; 64],
}
pub type cairo_tristrip_t = _cairo_tristrip;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_composite_glyphs_info_t {
    pub font: *mut cairo_scaled_font_t,
    pub glyphs: *mut cairo_glyph_t,
    pub num_glyphs: libc::c_int,
    pub use_mask: cairo_bool_t,
    pub extents: cairo_rectangle_int_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_traps_compositor {
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
    pub lerp: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
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
    pub composite_traps: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *mut cairo_surface_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const cairo_rectangle_int_t,
            cairo_antialias_t,
            *mut cairo_traps_t,
        ) -> cairo_int_status_t,
    >,
    pub composite_tristrip: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *mut cairo_surface_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const cairo_rectangle_int_t,
            cairo_antialias_t,
            *mut cairo_tristrip_t,
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
pub type cairo_traps_compositor_t = cairo_traps_compositor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_edge {
    pub line: cairo_line_t,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
    pub dir: libc::c_int,
}
pub type cairo_edge_t = _cairo_edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_polygon {
    pub status: cairo_status_t,
    pub extents: cairo_box_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_edges: libc::c_int,
    pub edges_size: libc::c_int,
    pub edges: *mut cairo_edge_t,
    pub edges_embedded: [cairo_edge_t; 32],
}
pub type cairo_polygon_t = _cairo_polygon;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
pub const FORCE_CLIP_REGION: C2RustUnnamed = 4;
pub const NEED_CLIP_SURFACE: C2RustUnnamed = 2;
pub const NEED_CLIP_REGION: C2RustUnnamed = 1;
pub type draw_func_t = Option::<
    unsafe extern "C" fn(
        *const cairo_traps_compositor_t,
        *mut cairo_surface_t,
        *mut libc::c_void,
        cairo_operator_t,
        *mut cairo_surface_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const cairo_rectangle_int_t,
        *mut cairo_clip_t,
    ) -> cairo_int_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blt_in {
    pub compositor: *const cairo_traps_compositor_t,
    pub dst: *mut cairo_surface_t,
    pub boxes: cairo_boxes_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_traps_info_t {
    pub traps: cairo_traps_t,
    pub antialias: cairo_antialias_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_tristrip_info_t {
    pub strip: cairo_tristrip_t,
    pub antialias: cairo_antialias_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_mask {
    pub mask: *mut cairo_surface_t,
    pub mask_x: libc::c_int,
    pub mask_y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_box_info {
    pub compositor: *const cairo_traps_compositor_t,
    pub dst: *mut cairo_surface_t,
    pub src: *mut cairo_surface_t,
    pub src_x: libc::c_int,
    pub src_y: libc::c_int,
    pub op: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_opacity_info {
    pub compositor: *const cairo_traps_compositor_t,
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
unsafe extern "C" fn _cairo_fixed_integer_round_down(
    mut f: cairo_fixed_t,
) -> libc::c_int {
    return _cairo_fixed_integer_part(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
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
unsafe extern "C" fn _cairo_rectangle_contains_rectangle(
    mut a: *const cairo_rectangle_int_t,
    mut b: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    return ((*a).x <= (*b).x && (*a).x + (*a).width >= (*b).x + (*b).width
        && (*a).y <= (*b).y && (*a).y + (*a).height >= (*b).y + (*b).height)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
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
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_box_from_integers(
    mut box_0: *mut cairo_box_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    (*box_0).p1.x = _cairo_fixed_from_int(x);
    (*box_0).p1.y = _cairo_fixed_from_int(y);
    (*box_0).p2.x = _cairo_fixed_from_int(x + w);
    (*box_0).p2.y = _cairo_fixed_from_int(y + h);
}
#[inline]
unsafe extern "C" fn _cairo_clip_is_all_clipped(
    mut clip: *const cairo_clip_t,
) -> cairo_bool_t {
    return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
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
#[inline]
unsafe extern "C" fn _cairo_surface_is_recording(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint) as libc::c_int;
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
    if coverage as libc::c_int >= 0xff00 as libc::c_int {
        return;
    }
    _cairo_box_from_integers(
        &mut *((*info).boxes.chunks.base).offset(0 as libc::c_int as isize),
        x as libc::c_int,
        y as libc::c_int,
        w as libc::c_int,
        h as libc::c_int,
    );
    _cairo_color_init_rgba(
        &mut color,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        coverage as libc::c_int as libc::c_double
            / 0xffff as libc::c_int as libc::c_double,
    );
    ((*(*info).compositor).fill_boxes)
        .expect(
            "non-null function pointer",
        )(
        (*info).dst as *mut libc::c_void,
        CAIRO_OPERATOR_IN,
        &mut color,
        &mut (*info).boxes,
    );
}
unsafe extern "C" fn add_rect_with_offset(
    mut boxes: *mut cairo_boxes_t,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    box_0.p1.x = _cairo_fixed_from_int(x1 - dx);
    box_0.p1.y = _cairo_fixed_from_int(y1 - dy);
    box_0.p2.x = _cairo_fixed_from_int(x2 - dx);
    box_0.p2.y = _cairo_fixed_from_int(y2 - dy);
    status = _cairo_boxes_add(boxes, CAIRO_ANTIALIAS_DEFAULT, &mut box_0)
        as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps-compositor.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void add_rect_with_offset(cairo_boxes_t *, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn combine_clip_as_traps(
    mut compositor: *const cairo_traps_compositor_t,
    mut mask: *mut cairo_surface_t,
    mut clip: *const cairo_clip_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
    let mut antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
    let mut traps: cairo_traps_t = cairo_traps_t {
        status: CAIRO_STATUS_SUCCESS,
        bounds: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
        c2rust_padding: [0; 3],
        num_traps: 0,
        traps_size: 0,
        traps: 0 as *mut cairo_trapezoid_t,
        traps_embedded: [cairo_trapezoid_t {
            top: 0,
            bottom: 0,
            left: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            right: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
        }; 16],
    };
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut fixup: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_clip_get_polygon(clip, &mut polygon, &mut fill_rule, &mut antialias);
    if status as u64 != 0 {
        return status;
    }
    _cairo_traps_init(&mut traps);
    status = _cairo_bentley_ottmann_tessellate_polygon(
        &mut traps,
        &mut polygon,
        fill_rule,
    ) as cairo_int_status_t;
    _cairo_polygon_fini(&mut polygon);
    if status as u64 != 0 {
        return status;
    }
    src = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(
        mask,
        0 as *const cairo_pattern_t,
        0 as libc::c_int,
        extents,
        0 as *const cairo_rectangle_int_t,
        &mut src_x,
        &mut src_y,
    );
    if (*src).status as u64 != 0 {
        _cairo_traps_fini(&mut traps);
        return (*src).status as cairo_int_status_t;
    }
    status = ((*compositor).composite_traps)
        .expect(
            "non-null function pointer",
        )(
        mask as *mut libc::c_void,
        CAIRO_OPERATOR_IN,
        src,
        src_x,
        src_y,
        (*extents).x,
        (*extents).y,
        extents,
        antialias,
        &mut traps,
    );
    _cairo_traps_extents(&mut traps, &mut box_0);
    _cairo_box_round_to_rectangle(&mut box_0, &mut fixup);
    _cairo_traps_fini(&mut traps);
    cairo_surface_destroy(src);
    if status as u64 != 0 {
        return status;
    }
    if _cairo_rectangle_intersect(&mut fixup, extents) == 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if fixup.width < (*extents).width || fixup.height < (*extents).height {
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
        _cairo_boxes_init(&mut clear);
        if fixup.y != (*extents).y {
            add_rect_with_offset(
                &mut clear,
                (*extents).x,
                (*extents).y,
                (*extents).x + (*extents).width,
                fixup.y,
                (*extents).x,
                (*extents).y,
            );
        }
        if fixup.x != (*extents).x {
            add_rect_with_offset(
                &mut clear,
                (*extents).x,
                fixup.y,
                fixup.x,
                fixup.y + fixup.height,
                (*extents).x,
                (*extents).y,
            );
        }
        if fixup.x + fixup.width != (*extents).x + (*extents).width {
            add_rect_with_offset(
                &mut clear,
                fixup.x + fixup.width,
                fixup.y,
                (*extents).x + (*extents).width,
                fixup.y + fixup.height,
                (*extents).x,
                (*extents).y,
            );
        }
        if fixup.y + fixup.height != (*extents).y + (*extents).height {
            add_rect_with_offset(
                &mut clear,
                (*extents).x,
                fixup.y + fixup.height,
                (*extents).x + (*extents).width,
                (*extents).y + (*extents).height,
                (*extents).x,
                (*extents).y,
            );
        }
        status = ((*compositor).fill_boxes)
            .expect(
                "non-null function pointer",
            )(
            mask as *mut libc::c_void,
            CAIRO_OPERATOR_CLEAR,
            _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
            &mut clear,
        );
        _cairo_boxes_fini(&mut clear);
    }
    return status;
}
unsafe extern "C" fn __clip_to_surface(
    mut compositor: *const cairo_traps_compositor_t,
    mut composite: *const cairo_composite_rectangles_t,
    mut extents: *const cairo_rectangle_int_t,
    mut surface: *mut *mut cairo_surface_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
    let mut antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
    let mut traps: cairo_traps_t = cairo_traps_t {
        status: CAIRO_STATUS_SUCCESS,
        bounds: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
        c2rust_padding: [0; 3],
        num_traps: 0,
        traps_size: 0,
        traps: 0 as *mut cairo_trapezoid_t,
        traps_embedded: [cairo_trapezoid_t {
            top: 0,
            bottom: 0,
            left: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            right: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
        }; 16],
    };
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
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_clip_get_polygon(
        (*composite).clip,
        &mut polygon,
        &mut fill_rule,
        &mut antialias,
    );
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    _cairo_traps_init(&mut traps);
    status = _cairo_bentley_ottmann_tessellate_polygon(
        &mut traps,
        &mut polygon,
        fill_rule,
    ) as cairo_int_status_t;
    _cairo_polygon_fini(&mut polygon);
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    mask = _cairo_surface_create_scratch(
        (*composite).surface,
        CAIRO_CONTENT_ALPHA,
        (*extents).width,
        (*extents).height,
        0 as *const cairo_color_t,
    );
    if (*mask).status as u64 != 0 {
        _cairo_traps_fini(&mut traps);
        return status as cairo_status_t;
    }
    src = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(
        mask,
        0 as *const cairo_pattern_t,
        0 as libc::c_int,
        extents,
        0 as *const cairo_rectangle_int_t,
        &mut src_x,
        &mut src_y,
    );
    status = (*src).status as cairo_int_status_t;
    if status as u64 != 0 {
        current_block = 970628688140122216;
    } else {
        status = ((*compositor).acquire)
            .expect("non-null function pointer")(mask as *mut libc::c_void);
        if status as u64 != 0 {
            current_block = 970628688140122216;
        } else {
            _cairo_boxes_init_from_rectangle(
                &mut clear,
                0 as libc::c_int,
                0 as libc::c_int,
                (*extents).width,
                (*extents).height,
            );
            status = ((*compositor).fill_boxes)
                .expect(
                    "non-null function pointer",
                )(
                mask as *mut libc::c_void,
                CAIRO_OPERATOR_CLEAR,
                _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
                &mut clear,
            );
            if status as u64 != 0 {
                current_block = 4334262699895247165;
            } else {
                status = ((*compositor).composite_traps)
                    .expect(
                        "non-null function pointer",
                    )(
                    mask as *mut libc::c_void,
                    CAIRO_OPERATOR_ADD,
                    src,
                    src_x,
                    src_y,
                    (*extents).x,
                    (*extents).y,
                    extents,
                    antialias,
                    &mut traps,
                );
                if status as u64 != 0 {
                    current_block = 4334262699895247165;
                } else {
                    ((*compositor).release)
                        .expect("non-null function pointer")(mask as *mut libc::c_void);
                    *surface = mask;
                    current_block = 4769713867740228885;
                }
            }
            match current_block {
                4769713867740228885 => {}
                _ => {
                    ((*compositor).release)
                        .expect("non-null function pointer")(mask as *mut libc::c_void);
                    current_block = 970628688140122216;
                }
            }
        }
    }
    match current_block {
        970628688140122216 => {
            cairo_surface_destroy(mask);
        }
        _ => {}
    }
    cairo_surface_destroy(src);
    _cairo_traps_fini(&mut traps);
    return status as cairo_status_t;
}
unsafe extern "C" fn traps_get_clip_surface(
    mut compositor: *const cairo_traps_compositor_t,
    mut composite: *const cairo_composite_rectangles_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = __clip_to_surface(compositor, composite, extents, &mut surface)
        as cairo_int_status_t;
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        surface = _cairo_surface_create_scratch(
            (*composite).surface,
            CAIRO_CONTENT_ALPHA,
            (*extents).width,
            (*extents).height,
            _cairo_stock_color(CAIRO_STOCK_WHITE),
        );
        if (*surface).status as u64 != 0 {
            return surface;
        }
        status = _cairo_clip_combine_with_surface(
            (*composite).clip,
            surface,
            (*extents).x,
            (*extents).y,
        ) as cairo_int_status_t;
    }
    if status as u64 != 0 {
        cairo_surface_destroy(surface);
        surface = _cairo_surface_create_in_error(status as cairo_status_t);
    }
    return surface;
}
unsafe extern "C" fn blt_unaligned_boxes(
    mut compositor: *const cairo_traps_compositor_t,
    mut surface: *mut cairo_surface_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut boxes: *mut cairo_box_t,
    mut num_boxes: libc::c_int,
) {
    let mut info: blt_in = blt_in {
        compositor: 0 as *const cairo_traps_compositor_t,
        dst: 0 as *mut cairo_surface_t,
        boxes: cairo_boxes_t {
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
        },
    };
    let mut i: libc::c_int = 0;
    info.compositor = compositor;
    info.dst = surface;
    _cairo_boxes_init(&mut info.boxes);
    info.boxes.num_boxes = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_boxes {
        let mut b: *mut cairo_box_t = &mut *boxes.offset(i as isize) as *mut cairo_box_t;
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
                dx,
                dy,
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn create_composite_mask(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut draw_closure: *mut libc::c_void,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> *mut cairo_surface_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
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
    src = ((*compositor).pattern_to_surface)
        .expect(
            "non-null function pointer",
        )(
        surface,
        &_cairo_pattern_white.base,
        0 as libc::c_int,
        &(*extents).bounded,
        &(*extents).bounded,
        &mut src_x,
        &mut src_y,
    );
    if (*src).status as u64 != 0 {
        cairo_surface_destroy(surface);
        return src;
    }
    status = ((*compositor).acquire)
        .expect("non-null function pointer")(surface as *mut libc::c_void);
    if status as u64 != 0 {
        cairo_surface_destroy(src);
        cairo_surface_destroy(surface);
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    if (*surface).is_clear() == 0 {
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
        _cairo_boxes_init_from_rectangle(
            &mut clear,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).bounded.width,
            (*extents).bounded.height,
        );
        status = ((*compositor).fill_boxes)
            .expect(
                "non-null function pointer",
            )(
            surface as *mut libc::c_void,
            CAIRO_OPERATOR_CLEAR,
            _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
            &mut clear,
        );
        if status as u64 != 0 {
            current_block = 2217840156037743612;
        } else {
            (*surface).set_is_clear(1 as libc::c_int as libc::c_uint);
            current_block = 7175849428784450219;
        }
    } else {
        current_block = 7175849428784450219;
    }
    match current_block {
        7175849428784450219 => {
            if mask_func.is_some() {
                status = mask_func
                    .expect(
                        "non-null function pointer",
                    )(
                    compositor,
                    surface,
                    draw_closure,
                    CAIRO_OPERATOR_SOURCE,
                    src,
                    src_x,
                    src_y,
                    (*extents).bounded.x,
                    (*extents).bounded.y,
                    &(*extents).bounded,
                    (*extents).clip,
                );
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
                    current_block = 16378897105462260407;
                } else if status as libc::c_uint
                    != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                {
                    current_block = 2217840156037743612;
                } else {
                    current_block = 7172762164747879670;
                }
            } else {
                current_block = 7172762164747879670;
            }
            match current_block {
                2217840156037743612 => {}
                _ => {
                    match current_block {
                        7172762164747879670 => {
                            status = draw_func
                                .expect(
                                    "non-null function pointer",
                                )(
                                compositor,
                                surface,
                                draw_closure,
                                CAIRO_OPERATOR_ADD,
                                src,
                                src_x,
                                src_y,
                                (*extents).bounded.x,
                                (*extents).bounded.y,
                                &(*extents).bounded,
                                0 as *mut cairo_clip_t,
                            );
                            if status as u64 != 0 {
                                current_block = 2217840156037743612;
                            } else {
                                (*surface).set_is_clear(0 as libc::c_int as libc::c_uint);
                                if !((*(*extents).clip).path).is_null() {
                                    status = combine_clip_as_traps(
                                        compositor,
                                        surface,
                                        (*extents).clip,
                                        &(*extents).bounded,
                                    );
                                    if status as libc::c_uint
                                        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                            as libc::c_uint
                                    {
                                        status = _cairo_clip_combine_with_surface(
                                            (*extents).clip,
                                            surface,
                                            (*extents).bounded.x,
                                            (*extents).bounded.y,
                                        ) as cairo_int_status_t;
                                    }
                                    if status as u64 != 0 {
                                        current_block = 2217840156037743612;
                                    } else {
                                        current_block = 16378897105462260407;
                                    }
                                } else {
                                    if !((*(*extents).clip).boxes).is_null() {
                                        blt_unaligned_boxes(
                                            compositor,
                                            surface,
                                            (*extents).bounded.x,
                                            (*extents).bounded.y,
                                            (*(*extents).clip).boxes,
                                            (*(*extents).clip).num_boxes,
                                        );
                                    }
                                    current_block = 16378897105462260407;
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        2217840156037743612 => {}
                        _ => {
                            ((*compositor).release)
                                .expect(
                                    "non-null function pointer",
                                )(surface as *mut libc::c_void);
                            cairo_surface_destroy(src);
                            return surface;
                        }
                    }
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
        surface = _cairo_surface_create_in_error(status as cairo_status_t);
    }
    cairo_surface_destroy(src);
    return surface;
}
unsafe extern "C" fn clip_and_composite_with_mask(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *const cairo_composite_rectangles_t,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    if !((*mask).is_clear() != 0) {
        if !src.is_null()
            || (*dst).content as libc::c_uint
                != CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
        {
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
    }
    cairo_surface_destroy(mask);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn clip_and_composite_combine(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *const cairo_composite_rectangles_t,
    mut draw_func: draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut tmp: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut clip: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
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
    status = ((*compositor).acquire)
        .expect("non-null function pointer")(tmp as *mut libc::c_void) as cairo_status_t;
    if status as u64 != 0 {
        cairo_surface_destroy(tmp);
        return status;
    }
    ((*compositor).composite)
        .expect(
            "non-null function pointer",
        )(
        tmp as *mut libc::c_void,
        (if (*dst).is_clear() as libc::c_int != 0 {
            CAIRO_OPERATOR_CLEAR as libc::c_int
        } else {
            CAIRO_OPERATOR_SOURCE as libc::c_int
        }) as cairo_operator_t,
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
        src,
        src_x,
        src_y,
        (*extents).bounded.x,
        (*extents).bounded.y,
        &(*extents).bounded,
        0 as *mut cairo_clip_t,
    ) as cairo_status_t;
    if !(status as u64 != 0) {
        clip = traps_get_clip_surface(compositor, extents, &(*extents).bounded);
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
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*extents).bounded.x,
                    (*extents).bounded.y,
                    (*extents).bounded.width as libc::c_uint,
                    (*extents).bounded.height as libc::c_uint,
                );
            } else {
                ((*compositor).lerp)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst as *mut libc::c_void,
                    tmp,
                    clip,
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
            cairo_surface_destroy(clip);
        }
    }
    ((*compositor).release)
        .expect("non-null function pointer")(tmp as *mut libc::c_void);
    cairo_surface_destroy(tmp);
    return status;
}
unsafe extern "C" fn clip_and_composite_source(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    if !((*mask).is_clear() != 0) {
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
            ((*compositor).lerp)
                .expect(
                    "non-null function pointer",
                )(
                dst as *mut libc::c_void,
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
    }
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
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut op: cairo_operator_t = (*extents).op;
    let mut pattern: *const cairo_pattern_t = &mut (*extents).source_pattern.base;
    return ((*dst).is_clear() as libc::c_int != 0
        && (*dst).content as libc::c_uint
            == CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
        && _cairo_pattern_is_opaque_solid(pattern) != 0 && can_reduce_alpha_op(op) != 0)
        as libc::c_int;
}
unsafe extern "C" fn fixup_unbounded_with_mask(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    mask = traps_get_clip_surface(compositor, extents, &(*extents).unbounded);
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
            0 as libc::c_int,
            0 as libc::c_int,
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
            0 as libc::c_int,
            y_0 - (*extents).unbounded.y,
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
            x_1 - (*extents).unbounded.x,
            y_1 - (*extents).unbounded.y,
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
            0 as libc::c_int,
            y_2 - (*extents).unbounded.y,
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
unsafe extern "C" fn add_rect(
    mut boxes: *mut cairo_boxes_t,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
) {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    box_0.p1.x = _cairo_fixed_from_int(x1);
    box_0.p1.y = _cairo_fixed_from_int(y1);
    box_0.p2.x = _cairo_fixed_from_int(x2);
    box_0.p2.y = _cairo_fixed_from_int(y2);
    status = _cairo_boxes_add(boxes, CAIRO_ANTIALIAS_DEFAULT, &mut box_0)
        as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps-compositor.c\0" as *const u8 as *const libc::c_char,
            811 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void add_rect(cairo_boxes_t *, int, int, int, int)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn fixup_unbounded(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut current_block: u64;
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
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*extents).bounded.width == (*extents).unbounded.width
        && (*extents).bounded.height == (*extents).unbounded.height
    {
        return CAIRO_STATUS_SUCCESS;
    }
    if ((*(*extents).clip).path).is_null() {} else {
        __assert_fail(
            b"extents->clip->path == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-traps-compositor.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"cairo_status_t fixup_unbounded(const cairo_traps_compositor_t *, cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
            ))
                .as_ptr(),
        );
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
    if boxes.is_null() {
        if (*extents).bounded.width == 0 as libc::c_int
            || (*extents).bounded.height == 0 as libc::c_int
        {
            current_block = 17448545326861384109;
        } else {
            if (*extents).bounded.y != (*extents).unbounded.y {
                add_rect(
                    &mut clear,
                    (*extents).unbounded.x,
                    (*extents).unbounded.y,
                    (*extents).unbounded.x + (*extents).unbounded.width,
                    (*extents).bounded.y,
                );
            }
            if (*extents).bounded.x != (*extents).unbounded.x {
                add_rect(
                    &mut clear,
                    (*extents).unbounded.x,
                    (*extents).bounded.y,
                    (*extents).bounded.x,
                    (*extents).bounded.y + (*extents).bounded.height,
                );
            }
            if (*extents).bounded.x + (*extents).bounded.width
                != (*extents).unbounded.x + (*extents).unbounded.width
            {
                add_rect(
                    &mut clear,
                    (*extents).bounded.x + (*extents).bounded.width,
                    (*extents).bounded.y,
                    (*extents).unbounded.x + (*extents).unbounded.width,
                    (*extents).bounded.y + (*extents).bounded.height,
                );
            }
            if (*extents).bounded.y + (*extents).bounded.height
                != (*extents).unbounded.y + (*extents).unbounded.height
            {
                add_rect(
                    &mut clear,
                    (*extents).unbounded.x,
                    (*extents).bounded.y + (*extents).bounded.height,
                    (*extents).unbounded.x + (*extents).unbounded.width,
                    (*extents).unbounded.y + (*extents).unbounded.height,
                );
            }
            current_block = 980989089337379490;
        }
    } else if (*boxes).num_boxes != 0 {
        _cairo_boxes_init(&mut tmp);
        if (*boxes).is_pixel_aligned != 0 {} else {
            __assert_fail(
                b"boxes->is_pixel_aligned\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-traps-compositor.c\0" as *const u8 as *const libc::c_char,
                880 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"cairo_status_t fixup_unbounded(const cairo_traps_compositor_t *, cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        status = _cairo_boxes_add(&mut tmp, CAIRO_ANTIALIAS_DEFAULT, &mut box_0)
            as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-traps-compositor.c\0" as *const u8 as *const libc::c_char,
                883 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"cairo_status_t fixup_unbounded(const cairo_traps_compositor_t *, cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
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
        ) as cairo_int_status_t;
        tmp.chunks.next = 0 as *mut _cairo_boxes_chunk;
        if status as u64 != 0 {
            current_block = 12181531763959899098;
        } else {
            current_block = 980989089337379490;
        }
    } else {
        current_block = 17448545326861384109;
    }
    match current_block {
        17448545326861384109 => {
            box_0.p1.x = _cairo_fixed_from_int((*extents).unbounded.x);
            box_0
                .p2
                .x = _cairo_fixed_from_int(
                (*extents).unbounded.x + (*extents).unbounded.width,
            );
            status = _cairo_boxes_add(&mut clear, CAIRO_ANTIALIAS_DEFAULT, &mut box_0)
                as cairo_int_status_t;
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-traps-compositor.c\0" as *const u8
                        as *const libc::c_char,
                    900 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 114],
                        &[libc::c_char; 114],
                    >(
                        b"cairo_status_t fixup_unbounded(const cairo_traps_compositor_t *, cairo_composite_rectangles_t *, cairo_boxes_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block = 980989089337379490;
        }
        _ => {}
    }
    match current_block {
        980989089337379490 => {
            if (*(*extents).clip).num_boxes != 0 {
                _cairo_boxes_init_for_array(
                    &mut tmp,
                    (*(*extents).clip).boxes,
                    (*(*extents).clip).num_boxes,
                );
                status = _cairo_boxes_intersect(&mut clear, &mut tmp, &mut clear)
                    as cairo_int_status_t;
                if status as u64 != 0 {
                    current_block = 12181531763959899098;
                } else {
                    current_block = 721385680381463314;
                }
            } else {
                current_block = 721385680381463314;
            }
            match current_block {
                12181531763959899098 => {}
                _ => {
                    status = ((*compositor).fill_boxes)
                        .expect(
                            "non-null function pointer",
                        )(
                        dst as *mut libc::c_void,
                        CAIRO_OPERATOR_CLEAR,
                        _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
                        &mut clear,
                    );
                }
            }
        }
        _ => {}
    }
    _cairo_boxes_fini(&mut clear);
    return status as cairo_status_t;
}
unsafe extern "C" fn need_bounded_clip(
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*(*extents).clip).num_boxes > 1 as libc::c_int
        || (*extents).mask.width > (*extents).unbounded.width
        || (*extents).mask.height > (*extents).unbounded.height
    {
        flags |= NEED_CLIP_REGION as libc::c_int as libc::c_uint;
    }
    if (*(*extents).clip).num_boxes > 1 as libc::c_int
        || (*extents).mask.width > (*extents).bounded.width
        || (*extents).mask.height > (*extents).bounded.height
    {
        flags |= FORCE_CLIP_REGION as libc::c_int as libc::c_uint;
    }
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
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut draw_func: draw_func_t,
    mut mask_func: draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut need_clip: libc::c_uint,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut op: cairo_operator_t = (*extents).op;
    let mut source: *mut cairo_pattern_t = &mut (*extents).source_pattern.base;
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut src_x: libc::c_int = 0;
    let mut src_y: libc::c_int = 0;
    let mut clip_region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if reduce_alpha_op(extents) != 0 {
        op = CAIRO_OPERATOR_ADD;
        source = 0 as *mut cairo_pattern_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
        op = CAIRO_OPERATOR_DEST_OUT;
        source = 0 as *mut cairo_pattern_t;
    }
    ((*compositor).acquire)
        .expect("non-null function pointer")(dst as *mut libc::c_void);
    if need_clip & NEED_CLIP_REGION as libc::c_int as libc::c_uint != 0 {
        let mut limit: *const cairo_rectangle_int_t = 0 as *const cairo_rectangle_int_t;
        if need_clip & FORCE_CLIP_REGION as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            limit = &mut (*extents).unbounded;
        } else {
            limit = &mut (*extents).destination;
        }
        clip_region = _cairo_clip_get_region((*extents).clip);
        if !clip_region.is_null()
            && cairo_region_contains_rectangle(clip_region, limit) as libc::c_uint
                == CAIRO_REGION_OVERLAP_IN as libc::c_int as libc::c_uint
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
    if (*extents).bounded.width == 0 as libc::c_int
        || (*extents).bounded.height == 0 as libc::c_int
    {
        current_block = 7451321250879711261;
    } else {
        src = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            source,
            0 as libc::c_int,
            &mut (*extents).bounded,
            &mut (*extents).source_sample_area,
            &mut src_x,
            &mut src_y,
        );
        status = (*src).status;
        if status as u64 != 0 {
            current_block = 16606933391642184533;
        } else {
            if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            {
                status = clip_and_composite_source(
                    compositor,
                    dst,
                    draw_func,
                    mask_func,
                    draw_closure,
                    src,
                    src_x,
                    src_y,
                    extents,
                );
            } else if need_clip & NEED_CLIP_SURFACE as libc::c_int as libc::c_uint != 0 {
                if (*extents).is_bounded != 0 {
                    status = clip_and_composite_with_mask(
                        compositor,
                        extents,
                        draw_func,
                        mask_func,
                        draw_closure,
                        op,
                        src,
                        src_x,
                        src_y,
                    );
                } else {
                    status = clip_and_composite_combine(
                        compositor,
                        extents,
                        draw_func,
                        draw_closure,
                        op,
                        src,
                        src_x,
                        src_y,
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
                    src_x,
                    src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    &mut (*extents).bounded,
                    (*extents).clip,
                ) as cairo_status_t;
            }
            cairo_surface_destroy(src);
            current_block = 7451321250879711261;
        }
    }
    match current_block {
        7451321250879711261 => {
            if status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                && (*extents).is_bounded == 0
            {
                if need_clip & NEED_CLIP_SURFACE as libc::c_int as libc::c_uint != 0 {
                    status = fixup_unbounded_with_mask(compositor, extents);
                } else {
                    status = fixup_unbounded(
                        compositor,
                        extents,
                        0 as *mut cairo_boxes_t,
                    );
                }
            }
        }
        _ => {}
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
unsafe extern "C" fn composite_traps(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut info: *mut composite_traps_info_t = closure as *mut composite_traps_info_t;
    return ((*compositor).composite_traps)
        .expect(
            "non-null function pointer",
        )(
        dst as *mut libc::c_void,
        op,
        src,
        src_x - dst_x,
        src_y - dst_y,
        dst_x,
        dst_y,
        extents,
        (*info).antialias,
        &mut (*info).traps,
    );
}
unsafe extern "C" fn composite_tristrip(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut info: *mut composite_tristrip_info_t = closure
        as *mut composite_tristrip_info_t;
    return ((*compositor).composite_tristrip)
        .expect(
            "non-null function pointer",
        )(
        dst as *mut libc::c_void,
        op,
        src,
        src_x - dst_x,
        src_y - dst_y,
        dst_x,
        dst_y,
        extents,
        (*info).antialias,
        &mut (*info).strip,
    );
}
unsafe extern "C" fn is_recording_pattern(
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if (*pattern).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    surface = (*(pattern as *const cairo_surface_pattern_t)).surface;
    surface = _cairo_surface_get_source(surface, 0 as *mut cairo_rectangle_int_t);
    return _cairo_surface_is_recording(surface);
}
unsafe extern "C" fn recording_pattern_get_surface(
    mut pattern: *const cairo_pattern_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    surface = (*(pattern as *const cairo_surface_pattern_t)).surface;
    return _cairo_surface_get_source(surface, 0 as *mut cairo_rectangle_int_t);
}
unsafe extern "C" fn recording_pattern_contains_sample(
    mut pattern: *const cairo_pattern_t,
    mut sample: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_recording_surface_t = 0
        as *mut cairo_recording_surface_t;
    if is_recording_pattern(pattern) == 0 {
        return 0 as libc::c_int;
    }
    if (*pattern).extend as libc::c_uint
        == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    surface = recording_pattern_get_surface(pattern) as *mut cairo_recording_surface_t;
    if (*surface).unbounded != 0 {
        return 1 as libc::c_int;
    }
    return _cairo_rectangle_contains_rectangle(&mut (*surface).extents, sample);
}
unsafe extern "C" fn op_reduces_to_source(
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    if (*extents).op as libc::c_uint
        == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if (*(*extents).surface).is_clear() != 0 {
        return ((*extents).op as libc::c_uint
            == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || (*extents).op as libc::c_uint
                == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn composite_aligned_boxes(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut op: cairo_operator_t = (*extents).op;
    let mut need_clip_mask: cairo_bool_t = (_cairo_clip_is_region((*extents).clip) == 0)
        as libc::c_int;
    let mut op_is_source: cairo_bool_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if need_clip_mask != 0
        && ((*extents).is_bounded == 0
            || (*extents).op as libc::c_uint
                == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    op_is_source = op_reduces_to_source(extents);
    if need_clip_mask == 0 && op_is_source != 0
        && recording_pattern_contains_sample(
            &mut (*extents).source_pattern.base,
            &mut (*extents).source_sample_area,
        ) != 0
    {
        let mut recording_clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
        let mut source: *const cairo_pattern_t = &mut (*extents).source_pattern.base;
        let mut m: *const cairo_matrix_t = 0 as *const cairo_matrix_t;
        let mut matrix: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        if (*dst).is_clear() == 0 {
            status = ((*compositor).acquire)
                .expect("non-null function pointer")(dst as *mut libc::c_void)
                as cairo_status_t;
            if status as u64 != 0 {
                return status;
            }
            status = ((*compositor).fill_boxes)
                .expect(
                    "non-null function pointer",
                )(
                dst as *mut libc::c_void,
                CAIRO_OPERATOR_CLEAR,
                _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
                boxes,
            ) as cairo_status_t;
            ((*compositor).release)
                .expect("non-null function pointer")(dst as *mut libc::c_void);
            if status as u64 != 0 {
                return status;
            }
        }
        m = &(*source).matrix;
        if _cairo_surface_has_device_transform(dst) != 0 {
            cairo_matrix_multiply(
                &mut matrix,
                &(*source).matrix,
                &mut (*dst).device_transform,
            );
            m = &mut matrix;
        }
        recording_clip = _cairo_clip_from_boxes(boxes);
        status = _cairo_recording_surface_replay_with_clip(
            recording_pattern_get_surface(source),
            m,
            dst,
            recording_clip,
        );
        _cairo_clip_destroy(recording_clip);
        return status;
    }
    status = ((*compositor).acquire)
        .expect("non-null function pointer")(dst as *mut libc::c_void) as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    if need_clip_mask == 0
        && (op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            || (*extents).source_pattern.base.type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint)
    {
        let mut color: *const cairo_color_t = 0 as *const cairo_color_t;
        if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
            color = _cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
        } else {
            color = &mut (*(&mut (*extents).source_pattern as *mut cairo_pattern_union_t
                as *mut cairo_solid_pattern_t))
                .color;
            if op_is_source != 0 {
                op = CAIRO_OPERATOR_SOURCE;
            }
        }
        status = ((*compositor).fill_boxes)
            .expect(
                "non-null function pointer",
            )(dst as *mut libc::c_void, op, color, boxes) as cairo_status_t;
    } else {
        let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        let mut mask: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        let mut source_0: *mut cairo_pattern_t = &mut (*extents).source_pattern.base;
        let mut src_x: libc::c_int = 0;
        let mut src_y: libc::c_int = 0;
        let mut mask_x: libc::c_int = 0 as libc::c_int;
        let mut mask_y: libc::c_int = 0 as libc::c_int;
        if need_clip_mask != 0 {
            mask = traps_get_clip_surface(compositor, extents, &mut (*extents).bounded);
            if (*mask).status as u64 != 0 {
                return (*mask).status;
            }
            mask_x = -(*extents).bounded.x;
            mask_y = -(*extents).bounded.y;
            if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            {
                source_0 = 0 as *mut cairo_pattern_t;
                op = CAIRO_OPERATOR_DEST_OUT;
            }
        } else if op_is_source != 0 {
            op = CAIRO_OPERATOR_SOURCE;
        }
        src = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            dst,
            source_0,
            0 as libc::c_int,
            &mut (*extents).bounded,
            &mut (*extents).source_sample_area,
            &mut src_x,
            &mut src_y,
        );
        if (*src).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
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
                &mut (*extents).bounded,
            ) as cairo_status_t;
            cairo_surface_destroy(src);
        } else {
            status = (*src).status;
        }
        cairo_surface_destroy(mask);
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*extents).is_bounded == 0
    {
        status = fixup_unbounded(compositor, extents, boxes);
    }
    ((*compositor).release)
        .expect("non-null function pointer")(dst as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn upload_boxes(
    mut compositor: *const cairo_traps_compositor_t,
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
unsafe extern "C" fn trim_extents_to_traps(
    mut extents: *mut cairo_composite_rectangles_t,
    mut traps: *mut cairo_traps_t,
) -> cairo_int_status_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    _cairo_traps_extents(traps, &mut box_0);
    return _cairo_composite_rectangles_intersect_mask_extents(extents, &mut box_0);
}
unsafe extern "C" fn trim_extents_to_tristrip(
    mut extents: *mut cairo_composite_rectangles_t,
    mut strip: *mut cairo_tristrip_t,
) -> cairo_int_status_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    _cairo_tristrip_extents(strip, &mut box_0);
    return _cairo_composite_rectangles_intersect_mask_extents(extents, &mut box_0);
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
unsafe extern "C" fn boxes_for_traps(
    mut boxes: *mut cairo_boxes_t,
    mut traps: *mut cairo_traps_t,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            let mut t: *const cairo_trapezoid_t = &mut *((*traps).traps)
                .offset(i as isize) as *mut cairo_trapezoid_t;
            if _cairo_fixed_integer_round_down((*t).left.p1.x)
                != _cairo_fixed_integer_round_down((*t).left.p2.x)
                || _cairo_fixed_integer_round_down((*t).right.p1.x)
                    != _cairo_fixed_integer_round_down((*t).right.p2.x)
            {
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            let mut t_0: *const cairo_trapezoid_t = &mut *((*traps).traps)
                .offset(i as isize) as *mut cairo_trapezoid_t;
            if (*t_0).left.p1.x != (*t_0).left.p2.x
                || (*t_0).right.p1.x != (*t_0).right.p2.x
            {
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            i += 1;
        }
    }
    _cairo_boxes_init(boxes);
    let ref mut fresh3 = (*boxes).chunks.base;
    *fresh3 = (*traps).traps as *mut cairo_box_t;
    (*boxes).chunks.size = (*traps).num_traps;
    if antialias as libc::c_uint != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int;
        i = j;
        while i < (*traps).num_traps {
            let mut x1: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).left.p1.x;
            let mut x2: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .right
                .p1
                .x;
            let mut y1: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).top;
            let mut y2: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).bottom;
            if !(x1 == x2 || y1 == y2) {
                (*((*boxes).chunks.base).offset(j as isize)).p1.x = x1;
                (*((*boxes).chunks.base).offset(j as isize)).p1.y = y1;
                (*((*boxes).chunks.base).offset(j as isize)).p2.x = x2;
                (*((*boxes).chunks.base).offset(j as isize)).p2.y = y2;
                j += 1;
                if (*boxes).is_pixel_aligned != 0 {
                    (*boxes)
                        .is_pixel_aligned = (_cairo_fixed_is_integer(x1) != 0
                        && _cairo_fixed_is_integer(y1) != 0
                        && _cairo_fixed_is_integer(x2) != 0
                        && _cairo_fixed_is_integer(y2) != 0) as libc::c_int
                        as libc::c_uint;
                }
            }
            i += 1;
        }
    } else {
        (*boxes).is_pixel_aligned = 1 as libc::c_int as libc::c_uint;
        j = 0 as libc::c_int;
        i = j;
        while i < (*traps).num_traps {
            let mut x1_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .left
                .p1
                .x;
            let mut x2_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .right
                .p1
                .x;
            let mut y1_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).top;
            let mut y2_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).bottom;
            (*((*boxes).chunks.base).offset(j as isize))
                .p1
                .x = _cairo_fixed_round_down(x1_0);
            (*((*boxes).chunks.base).offset(j as isize))
                .p1
                .y = _cairo_fixed_round_down(y1_0);
            (*((*boxes).chunks.base).offset(j as isize))
                .p2
                .x = _cairo_fixed_round_down(x2_0);
            (*((*boxes).chunks.base).offset(j as isize))
                .p2
                .y = _cairo_fixed_round_down(y2_0);
            j
                += ((*((*boxes).chunks.base).offset(j as isize)).p1.x
                    != (*((*boxes).chunks.base).offset(j as isize)).p2.x
                    && (*((*boxes).chunks.base).offset(j as isize)).p1.y
                        != (*((*boxes).chunks.base).offset(j as isize)).p2.y)
                    as libc::c_int;
            i += 1;
        }
    }
    (*boxes).chunks.count = j;
    (*boxes).num_boxes = j;
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn clip_and_composite_polygon(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut polygon: *mut cairo_polygon_t,
    mut antialias: cairo_antialias_t,
    mut fill_rule: cairo_fill_rule_t,
    mut curvy: cairo_bool_t,
) -> cairo_status_t {
    let mut traps: composite_traps_info_t = composite_traps_info_t {
        traps: cairo_traps_t {
            status: CAIRO_STATUS_SUCCESS,
            bounds: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
            c2rust_padding: [0; 3],
            num_traps: 0,
            traps_size: 0,
            traps: 0 as *mut cairo_trapezoid_t,
            traps_embedded: [cairo_trapezoid_t {
                top: 0,
                bottom: 0,
                left: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                right: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
            }; 16],
        },
        antialias: CAIRO_ANTIALIAS_DEFAULT,
    };
    let mut dst: *mut cairo_surface_t = (*extents).surface;
    let mut clip_surface: cairo_bool_t = (_cairo_clip_is_region((*extents).clip) == 0)
        as libc::c_int;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*polygon).num_edges == 0 as libc::c_int {
        status = CAIRO_INT_STATUS_SUCCESS;
        if (*extents).is_bounded == 0 {
            let mut clip_region: *mut cairo_region_t = _cairo_clip_get_region(
                (*extents).clip,
            );
            if !clip_region.is_null()
                && cairo_region_contains_rectangle(
                    clip_region,
                    &mut (*extents).unbounded,
                ) as libc::c_uint
                    == CAIRO_REGION_OVERLAP_IN as libc::c_int as libc::c_uint
            {
                clip_region = 0 as *mut cairo_region_t;
            }
            if !clip_region.is_null() {
                status = ((*compositor).set_clip_region)
                    .expect(
                        "non-null function pointer",
                    )(dst as *mut libc::c_void, clip_region);
                if status as u64 != 0 {
                    return status as cairo_status_t;
                }
            }
            if clip_surface != 0 {
                status = fixup_unbounded_with_mask(compositor, extents)
                    as cairo_int_status_t;
            } else {
                status = fixup_unbounded(compositor, extents, 0 as *mut cairo_boxes_t)
                    as cairo_int_status_t;
            }
            if !clip_region.is_null() {
                ((*compositor).set_clip_region)
                    .expect(
                        "non-null function pointer",
                    )(dst as *mut libc::c_void, 0 as *mut cairo_region_t);
            }
        }
        return status as cairo_status_t;
    }
    if !((*(*extents).clip).path).is_null() && (*extents).is_bounded != 0 {
        let mut clipper: cairo_polygon_t = cairo_polygon_t {
            status: CAIRO_STATUS_SUCCESS,
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_edges: 0,
            edges_size: 0,
            edges: 0 as *mut cairo_edge_t,
            edges_embedded: [cairo_edge_t {
                line: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            }; 32],
        };
        let mut clipper_fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
        let mut clipper_antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
        status = _cairo_clip_get_polygon(
            (*extents).clip,
            &mut clipper,
            &mut clipper_fill_rule,
            &mut clipper_antialias,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            if clipper_antialias as libc::c_uint == antialias as libc::c_uint {
                status = _cairo_polygon_intersect(
                    polygon,
                    fill_rule as libc::c_int,
                    &mut clipper,
                    clipper_fill_rule as libc::c_int,
                ) as cairo_int_status_t;
                if status as libc::c_uint
                    == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    let mut clip: *mut cairo_clip_t = _cairo_clip_copy_region(
                        (*extents).clip,
                    );
                    _cairo_clip_destroy((*extents).clip);
                    let ref mut fresh4 = (*extents).clip;
                    *fresh4 = clip;
                    fill_rule = CAIRO_FILL_RULE_WINDING;
                }
                _cairo_polygon_fini(&mut clipper);
            }
        }
    }
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        && curvy != 0
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
        _cairo_boxes_init(&mut boxes);
        status = _cairo_rasterise_polygon_to_boxes(polygon, fill_rule, &mut boxes)
            as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            if boxes.is_pixel_aligned != 0 {} else {
                __assert_fail(
                    b"boxes.is_pixel_aligned\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-traps-compositor.c\0" as *const u8
                        as *const libc::c_char,
                    1551 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 179],
                        &[libc::c_char; 179],
                    >(
                        b"cairo_status_t clip_and_composite_polygon(const cairo_traps_compositor_t *, cairo_composite_rectangles_t *, cairo_polygon_t *, cairo_antialias_t, cairo_fill_rule_t, cairo_bool_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            status = clip_and_composite_boxes(compositor, extents, &mut boxes)
                as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    _cairo_traps_init(&mut traps.traps);
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        && curvy != 0
    {
        status = _cairo_rasterise_polygon_to_traps(
            polygon,
            fill_rule,
            antialias,
            &mut traps.traps,
        );
    } else {
        status = _cairo_bentley_ottmann_tessellate_polygon(
            &mut traps.traps,
            polygon,
            fill_rule,
        ) as cairo_int_status_t;
    }
    if !(status as u64 != 0) {
        status = trim_extents_to_traps(extents, &mut traps.traps);
        if !(status as u64 != 0) {
            status = CAIRO_INT_STATUS_UNSUPPORTED;
            let mut boxes_0: cairo_boxes_t = cairo_boxes_t {
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
            status = boxes_for_traps(&mut boxes_0, &mut traps.traps, antialias);
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                status = clip_and_composite_boxes(compositor, extents, &mut boxes_0)
                    as cairo_int_status_t;
                if status as libc::c_uint
                    != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"status != CAIRO_INT_STATUS_UNSUPPORTED\0" as *const u8
                            as *const libc::c_char,
                        b"../src/cairo-traps-compositor.c\0" as *const u8
                            as *const libc::c_char,
                        1582 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 179],
                            &[libc::c_char; 179],
                        >(
                            b"cairo_status_t clip_and_composite_polygon(const cairo_traps_compositor_t *, cairo_composite_rectangles_t *, cairo_polygon_t *, cairo_antialias_t, cairo_fill_rule_t, cairo_bool_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
            if status as libc::c_uint
                == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
            {
                let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                if (*extents).is_bounded == 0 {
                    flags |= FORCE_CLIP_REGION as libc::c_int as libc::c_uint;
                }
                traps.antialias = antialias;
                status = clip_and_composite(
                    compositor,
                    extents,
                    Some(
                        composite_traps
                            as unsafe extern "C" fn(
                                *const cairo_traps_compositor_t,
                                *mut cairo_surface_t,
                                *mut libc::c_void,
                                cairo_operator_t,
                                *mut cairo_surface_t,
                                libc::c_int,
                                libc::c_int,
                                libc::c_int,
                                libc::c_int,
                                *const cairo_rectangle_int_t,
                                *mut cairo_clip_t,
                            ) -> cairo_int_status_t,
                    ),
                    None,
                    &mut traps as *mut composite_traps_info_t as *mut libc::c_void,
                    need_unbounded_clip(extents) as libc::c_uint | flags,
                ) as cairo_int_status_t;
            }
        }
    }
    _cairo_traps_fini(&mut traps.traps);
    return status as cairo_status_t;
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
    let mut compositor: *const cairo_traps_compositor_t = (*info).compositor;
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
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut mask: *const cairo_solid_pattern_t = closure as *const cairo_solid_pattern_t;
    let mut info: composite_opacity_info = composite_opacity_info {
        compositor: 0 as *const cairo_traps_compositor_t,
        op: 0,
        dst: 0 as *mut cairo_surface_t,
        src: 0 as *mut cairo_surface_t,
        src_x: 0,
        src_y: 0,
        opacity: 0.,
    };
    let mut i: libc::c_int = 0;
    info.compositor = compositor;
    info.op = op as uint8_t;
    info.dst = dst;
    info.src = src;
    info.src_x = src_x;
    info.src_y = src_y;
    info.opacity = (*mask).color.alpha / 0xffff as libc::c_int as libc::c_double;
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
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_boxes(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut traps: cairo_traps_t = cairo_traps_t {
        status: CAIRO_STATUS_SUCCESS,
        bounds: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
        c2rust_padding: [0; 3],
        num_traps: 0,
        traps_size: 0,
        traps: 0 as *mut cairo_trapezoid_t,
        traps_embedded: [cairo_trapezoid_t {
            top: 0,
            bottom: 0,
            left: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            right: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
        }; 16],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_traps_init_boxes(&mut traps, closure as *const cairo_boxes_t);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = ((*compositor).composite_traps)
        .expect(
            "non-null function pointer",
        )(
        dst as *mut libc::c_void,
        op,
        src,
        src_x - dst_x,
        src_y - dst_y,
        dst_x,
        dst_y,
        extents,
        CAIRO_ANTIALIAS_DEFAULT,
        &mut traps,
    ) as cairo_status_t;
    _cairo_traps_fini(&mut traps);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn clip_and_composite_boxes(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*boxes).num_boxes == 0 as libc::c_int && (*extents).is_bounded != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = trim_extents_to_boxes(extents, boxes);
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if (*boxes).is_pixel_aligned != 0 && ((*(*extents).clip).path).is_null()
        && (*extents).source_pattern.base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (op_reduces_to_source(extents) != 0
            || (*extents).op as libc::c_uint
                == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                && (*(*extents).source_pattern.surface.surface).content as libc::c_uint
                    & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
    {
        status = upload_boxes(compositor, extents, boxes) as cairo_int_status_t;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    if !((*(*extents).clip).path).is_null() && (*extents).is_bounded != 0 {
        let mut polygon: cairo_polygon_t = cairo_polygon_t {
            status: CAIRO_STATUS_SUCCESS,
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_edges: 0,
            edges_size: 0,
            edges: 0 as *mut cairo_edge_t,
            edges_embedded: [cairo_edge_t {
                line: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            }; 32],
        };
        let mut fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
        let mut antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
        let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
        clip = _cairo_clip_copy((*extents).clip);
        clip = _cairo_clip_intersect_boxes(clip, boxes);
        if _cairo_clip_is_all_clipped(clip) != 0 {
            return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
        }
        status = _cairo_clip_get_polygon(
            clip,
            &mut polygon,
            &mut fill_rule,
            &mut antialias,
        );
        _cairo_clip_path_destroy((*clip).path);
        let ref mut fresh5 = (*clip).path;
        *fresh5 = 0 as *mut cairo_clip_path_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            let mut saved_clip: *mut cairo_clip_t = (*extents).clip;
            let ref mut fresh6 = (*extents).clip;
            *fresh6 = clip;
            status = clip_and_composite_polygon(
                compositor,
                extents,
                &mut polygon,
                antialias,
                fill_rule,
                0 as libc::c_int,
            ) as cairo_int_status_t;
            clip = (*extents).clip;
            let ref mut fresh7 = (*extents).clip;
            *fresh7 = saved_clip;
            _cairo_polygon_fini(&mut polygon);
        }
        _cairo_clip_destroy(clip);
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    if (*boxes).is_pixel_aligned != 0 {
        status = composite_aligned_boxes(compositor, extents, boxes)
            as cairo_int_status_t;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    return clip_and_composite(
        compositor,
        extents,
        Some(
            composite_boxes
                as unsafe extern "C" fn(
                    *const cairo_traps_compositor_t,
                    *mut cairo_surface_t,
                    *mut libc::c_void,
                    cairo_operator_t,
                    *mut cairo_surface_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *const cairo_rectangle_int_t,
                    *mut cairo_clip_t,
                ) -> cairo_int_status_t,
        ),
        None,
        boxes as *mut libc::c_void,
        need_unbounded_clip(extents) as libc::c_uint,
    );
}
unsafe extern "C" fn composite_traps_as_boxes(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut info: *mut composite_traps_info_t,
) -> cairo_int_status_t {
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
    if _cairo_traps_to_boxes(&mut (*info).traps, (*info).antialias, &mut boxes) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    return clip_and_composite_boxes(compositor, extents, &mut boxes)
        as cairo_int_status_t;
}
unsafe extern "C" fn clip_and_composite_traps(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut info: *mut composite_traps_info_t,
    mut flags: libc::c_uint,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = trim_extents_to_traps(extents, &mut (*info).traps);
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if flags & FORCE_CLIP_REGION as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        status = composite_traps_as_boxes(compositor, extents, info);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        if (*extents).is_bounded == 0 {
            flags |= FORCE_CLIP_REGION as libc::c_int as libc::c_uint;
        }
        status = clip_and_composite(
            compositor,
            extents,
            Some(
                composite_traps
                    as unsafe extern "C" fn(
                        *const cairo_traps_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *mut cairo_surface_t,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            None,
            info as *mut libc::c_void,
            need_unbounded_clip(extents) as libc::c_uint | flags,
        ) as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn clip_and_composite_tristrip(
    mut compositor: *const cairo_traps_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut info: *mut composite_tristrip_info_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    status = trim_extents_to_tristrip(extents, &mut (*info).strip);
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return status;
    }
    if (*extents).is_bounded == 0 {
        flags |= FORCE_CLIP_REGION as libc::c_int as libc::c_uint;
    }
    status = clip_and_composite(
        compositor,
        extents,
        Some(
            composite_tristrip
                as unsafe extern "C" fn(
                    *const cairo_traps_compositor_t,
                    *mut cairo_surface_t,
                    *mut libc::c_void,
                    cairo_operator_t,
                    *mut cairo_surface_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *const cairo_rectangle_int_t,
                    *mut cairo_clip_t,
                ) -> cairo_int_status_t,
        ),
        None,
        info as *mut libc::c_void,
        need_unbounded_clip(extents) as libc::c_uint | flags,
    ) as cairo_int_status_t;
    return status;
}
unsafe extern "C" fn composite_mask(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut data: *mut composite_mask = closure as *mut composite_mask;
    if !src.is_null() {
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            src,
            (*data).mask,
            (*extents).x + src_x,
            (*extents).y + src_y,
            (*extents).x + (*data).mask_x,
            (*extents).y + (*data).mask_y,
            (*extents).x - dst_x,
            (*extents).y - dst_y,
            (*extents).width as libc::c_uint,
            (*extents).height as libc::c_uint,
        );
    } else {
        ((*compositor).composite)
            .expect(
                "non-null function pointer",
            )(
            dst as *mut libc::c_void,
            op,
            (*data).mask,
            0 as *mut cairo_surface_t,
            (*extents).x + (*data).mask_x,
            (*extents).y + (*data).mask_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*extents).x - dst_x,
            (*extents).y - dst_y,
            (*extents).width as libc::c_uint,
            (*extents).height as libc::c_uint,
        );
    }
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
    let mut compositor: *const cairo_traps_compositor_t = (*info).compositor;
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
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut data: *mut composite_mask = closure as *mut composite_mask;
    let mut info: composite_box_info = composite_box_info {
        compositor: 0 as *const cairo_traps_compositor_t,
        dst: 0 as *mut cairo_surface_t,
        src: 0 as *mut cairo_surface_t,
        src_x: 0,
        src_y: 0,
        op: 0,
    };
    let mut i: libc::c_int = 0;
    info.compositor = compositor;
    info.op = CAIRO_OPERATOR_SOURCE as libc::c_int as uint8_t;
    info.dst = dst;
    info.src = (*data).mask;
    info.src_x = (*data).mask_x;
    info.src_y = (*data).mask_y;
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
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_mask_clip(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut data: *mut composite_mask = closure as *mut composite_mask;
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
    let mut info: composite_traps_info_t = composite_traps_info_t {
        traps: cairo_traps_t {
            status: CAIRO_STATUS_SUCCESS,
            bounds: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
            c2rust_padding: [0; 3],
            num_traps: 0,
            traps_size: 0,
            traps: 0 as *mut cairo_trapezoid_t,
            traps_embedded: [cairo_trapezoid_t {
                top: 0,
                bottom: 0,
                left: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                right: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
            }; 16],
        },
        antialias: CAIRO_ANTIALIAS_DEFAULT,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_clip_get_polygon(
        clip,
        &mut polygon,
        &mut fill_rule,
        &mut info.antialias,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_traps_init(&mut info.traps);
    status = _cairo_bentley_ottmann_tessellate_polygon(
        &mut info.traps,
        &mut polygon,
        fill_rule,
    );
    _cairo_polygon_fini(&mut polygon);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    status = composite_traps(
        compositor,
        dst,
        &mut info as *mut composite_traps_info_t as *mut libc::c_void,
        CAIRO_OPERATOR_SOURCE,
        (*data).mask,
        (*data).mask_x + dst_x,
        (*data).mask_y + dst_y,
        dst_x,
        dst_y,
        extents,
        0 as *mut cairo_clip_t,
    ) as cairo_status_t;
    _cairo_traps_fini(&mut info.traps);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_traps_compositor_paint(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut compositor: *mut cairo_traps_compositor_t = _compositor
        as *mut cairo_traps_compositor_t;
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
unsafe extern "C" fn _cairo_traps_compositor_mask(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_traps_compositor_t = _compositor
        as *mut cairo_traps_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    if (*extents).mask_pattern.base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
        && ((*(*extents).clip).path).is_null()
    {
        status = clip_and_composite(
            compositor,
            extents,
            Some(
                composite_opacity_boxes
                    as unsafe extern "C" fn(
                        *const cairo_traps_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *mut cairo_surface_t,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            Some(
                composite_opacity_boxes
                    as unsafe extern "C" fn(
                        *const cairo_traps_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *mut cairo_surface_t,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            &mut (*extents).mask_pattern as *mut cairo_pattern_union_t
                as *mut libc::c_void,
            need_unbounded_clip(extents) as libc::c_uint,
        ) as cairo_int_status_t;
    } else {
        let mut data: composite_mask = composite_mask {
            mask: 0 as *mut cairo_surface_t,
            mask_x: 0,
            mask_y: 0,
        };
        data
            .mask = ((*compositor).pattern_to_surface)
            .expect(
                "non-null function pointer",
            )(
            (*extents).surface,
            &mut (*extents).mask_pattern.base,
            1 as libc::c_int,
            &mut (*extents).bounded,
            &mut (*extents).mask_sample_area,
            &mut data.mask_x,
            &mut data.mask_y,
        );
        if (*data.mask).status as u64 != 0 {
            return (*data.mask).status as cairo_int_status_t;
        }
        status = clip_and_composite(
            compositor,
            extents,
            Some(
                composite_mask
                    as unsafe extern "C" fn(
                        *const cairo_traps_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *mut cairo_surface_t,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            if !((*(*extents).clip).path).is_null() {
                Some(
                    composite_mask_clip
                        as unsafe extern "C" fn(
                            *const cairo_traps_compositor_t,
                            *mut cairo_surface_t,
                            *mut libc::c_void,
                            cairo_operator_t,
                            *mut cairo_surface_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_rectangle_int_t,
                            *mut cairo_clip_t,
                        ) -> cairo_int_status_t,
                )
            } else {
                Some(
                    composite_mask_clip_boxes
                        as unsafe extern "C" fn(
                            *const cairo_traps_compositor_t,
                            *mut cairo_surface_t,
                            *mut libc::c_void,
                            cairo_operator_t,
                            *mut cairo_surface_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_rectangle_int_t,
                            *mut cairo_clip_t,
                        ) -> cairo_int_status_t,
                )
            },
            &mut data as *mut composite_mask as *mut libc::c_void,
            need_bounded_clip(extents) as libc::c_uint,
        ) as cairo_int_status_t;
        cairo_surface_destroy(data.mask);
    }
    return status;
}
unsafe extern "C" fn _cairo_traps_compositor_stroke(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_traps_compositor_t = _compositor
        as *mut cairo_traps_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
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
        && 0 as libc::c_int != 0 && _cairo_clip_is_region((*extents).clip) != 0
    {
        let mut info: composite_tristrip_info_t = composite_tristrip_info_t {
            strip: cairo_tristrip_t {
                status: CAIRO_STATUS_SUCCESS,
                limits: 0 as *const cairo_box_t,
                num_limits: 0,
                num_points: 0,
                size_points: 0,
                points: 0 as *mut cairo_point_t,
                points_embedded: [cairo_point_t { x: 0, y: 0 }; 64],
            },
            antialias: CAIRO_ANTIALIAS_DEFAULT,
        };
        info.antialias = antialias;
        _cairo_tristrip_init_with_clip(&mut info.strip, (*extents).clip);
        status = _cairo_path_fixed_stroke_to_tristrip(
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            &mut info.strip,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = clip_and_composite_tristrip(compositor, extents, &mut info);
        }
        _cairo_tristrip_fini(&mut info.strip);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        && (*path).has_curve_to() as libc::c_int != 0
        && antialias as libc::c_uint
            == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
    {
        let mut polygon: cairo_polygon_t = cairo_polygon_t {
            status: CAIRO_STATUS_SUCCESS,
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_edges: 0,
            edges_size: 0,
            edges: 0 as *mut cairo_edge_t,
            edges_embedded: [cairo_edge_t {
                line: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            }; 32],
        };
        _cairo_polygon_init_with_clip(&mut polygon, (*extents).clip);
        status = _cairo_path_fixed_stroke_to_polygon(
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            &mut polygon,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = clip_and_composite_polygon(
                compositor,
                extents,
                &mut polygon,
                CAIRO_ANTIALIAS_NONE,
                CAIRO_FILL_RULE_WINDING,
                1 as libc::c_int,
            ) as cairo_int_status_t;
        }
        _cairo_polygon_fini(&mut polygon);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        let mut func: Option::<
            unsafe extern "C" fn(
                *const cairo_path_fixed_t,
                *const cairo_stroke_style_t,
                *const cairo_matrix_t,
                *const cairo_matrix_t,
                libc::c_double,
                *mut cairo_traps_t,
            ) -> cairo_int_status_t,
        > = None;
        let mut info_0: composite_traps_info_t = composite_traps_info_t {
            traps: cairo_traps_t {
                status: CAIRO_STATUS_SUCCESS,
                bounds: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                limits: 0 as *const cairo_box_t,
                num_limits: 0,
                maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
                c2rust_padding: [0; 3],
                num_traps: 0,
                traps_size: 0,
                traps: 0 as *mut cairo_trapezoid_t,
                traps_embedded: [cairo_trapezoid_t {
                    top: 0,
                    bottom: 0,
                    left: cairo_box_t {
                        p1: cairo_point_t { x: 0, y: 0 },
                        p2: cairo_point_t { x: 0, y: 0 },
                    },
                    right: cairo_box_t {
                        p1: cairo_point_t { x: 0, y: 0 },
                        p2: cairo_point_t { x: 0, y: 0 },
                    },
                }; 16],
            },
            antialias: CAIRO_ANTIALIAS_DEFAULT,
        };
        let mut flags: libc::c_uint = 0;
        if antialias as libc::c_uint
            == CAIRO_ANTIALIAS_BEST as libc::c_int as libc::c_uint
            || antialias as libc::c_uint
                == CAIRO_ANTIALIAS_GOOD as libc::c_int as libc::c_uint
        {
            func = Some(
                _cairo_path_fixed_stroke_polygon_to_traps
                    as unsafe extern "C" fn(
                        *const cairo_path_fixed_t,
                        *const cairo_stroke_style_t,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        libc::c_double,
                        *mut cairo_traps_t,
                    ) -> cairo_int_status_t,
            );
            flags = 0 as libc::c_int as libc::c_uint;
        } else {
            func = Some(
                _cairo_path_fixed_stroke_to_traps
                    as unsafe extern "C" fn(
                        *const cairo_path_fixed_t,
                        *const cairo_stroke_style_t,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        libc::c_double,
                        *mut cairo_traps_t,
                    ) -> cairo_int_status_t,
            );
            flags = (need_bounded_clip(extents) & !(NEED_CLIP_SURFACE as libc::c_int))
                as libc::c_uint;
        }
        info_0.antialias = antialias;
        _cairo_traps_init_with_clip(&mut info_0.traps, (*extents).clip);
        status = func
            .expect(
                "non-null function pointer",
            )(path, style, ctm, ctm_inverse, tolerance, &mut info_0.traps);
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = clip_and_composite_traps(compositor, extents, &mut info_0, flags);
        }
        _cairo_traps_fini(&mut info_0.traps);
    }
    return status;
}
unsafe extern "C" fn _cairo_traps_compositor_fill(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_traps_compositor_t = _compositor
        as *mut cairo_traps_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
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
        let mut polygon: cairo_polygon_t = cairo_polygon_t {
            status: CAIRO_STATUS_SUCCESS,
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_edges: 0,
            edges_size: 0,
            edges: 0 as *mut cairo_edge_t,
            edges_embedded: [cairo_edge_t {
                line: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            }; 32],
        };
        _cairo_polygon_init_with_clip(&mut polygon, (*extents).clip);
        status = _cairo_path_fixed_fill_to_polygon(path, tolerance, &mut polygon)
            as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = clip_and_composite_polygon(
                compositor,
                extents,
                &mut polygon,
                antialias,
                fill_rule,
                (*path).has_curve_to() as cairo_bool_t,
            ) as cairo_int_status_t;
        }
        _cairo_polygon_fini(&mut polygon);
    }
    return status;
}
unsafe extern "C" fn composite_glyphs(
    mut compositor: *const cairo_traps_compositor_t,
    mut dst: *mut cairo_surface_t,
    mut closure: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut info: *mut cairo_composite_glyphs_info_t = closure
        as *mut cairo_composite_glyphs_info_t;
    if op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint
        && (*dst).content as libc::c_uint
            & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        (*info).use_mask = 0 as libc::c_int;
    }
    return ((*compositor).composite_glyphs)
        .expect(
            "non-null function pointer",
        )(dst as *mut libc::c_void, op, src, src_x, src_y, dst_x, dst_y, info);
}
unsafe extern "C" fn _cairo_traps_compositor_glyphs(
    mut _compositor: *const cairo_compositor_t,
    mut extents: *mut cairo_composite_rectangles_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut overlap: cairo_bool_t,
) -> cairo_int_status_t {
    let mut compositor: *const cairo_traps_compositor_t = _compositor
        as *mut cairo_traps_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = ((*compositor).check_composite)
        .expect("non-null function pointer")(extents);
    if status as u64 != 0 {
        return status;
    }
    _cairo_scaled_font_freeze_cache(scaled_font);
    status = ((*compositor).check_composite_glyphs)
        .expect(
            "non-null function pointer",
        )(extents, scaled_font, glyphs, &mut num_glyphs);
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        let mut info: cairo_composite_glyphs_info_t = cairo_composite_glyphs_info_t {
            font: 0 as *mut cairo_scaled_font_t,
            glyphs: 0 as *mut cairo_glyph_t,
            num_glyphs: 0,
            use_mask: 0,
            extents: cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        };
        info.font = scaled_font;
        info.glyphs = glyphs;
        info.num_glyphs = num_glyphs;
        info.use_mask = (overlap != 0 || (*extents).is_bounded == 0) as libc::c_int;
        info.extents = (*extents).bounded;
        status = clip_and_composite(
            compositor,
            extents,
            Some(
                composite_glyphs
                    as unsafe extern "C" fn(
                        *const cairo_traps_compositor_t,
                        *mut cairo_surface_t,
                        *mut libc::c_void,
                        cairo_operator_t,
                        *mut cairo_surface_t,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            None,
            &mut info as *mut cairo_composite_glyphs_info_t as *mut libc::c_void,
            (need_bounded_clip(extents) | FORCE_CLIP_REGION as libc::c_int)
                as libc::c_uint,
        ) as cairo_int_status_t;
    }
    _cairo_scaled_font_thaw_cache(scaled_font);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_traps_compositor_init(
    mut compositor: *mut cairo_traps_compositor_t,
    mut delegate: *const cairo_compositor_t,
) {
    let ref mut fresh8 = (*compositor).base.delegate;
    *fresh8 = delegate;
    let ref mut fresh9 = (*compositor).base.paint;
    *fresh9 = Some(
        _cairo_traps_compositor_paint
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
            ) -> cairo_int_status_t,
    );
    let ref mut fresh10 = (*compositor).base.mask;
    *fresh10 = Some(
        _cairo_traps_compositor_mask
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
            ) -> cairo_int_status_t,
    );
    let ref mut fresh11 = (*compositor).base.fill;
    *fresh11 = Some(
        _cairo_traps_compositor_fill
            as unsafe extern "C" fn(
                *const cairo_compositor_t,
                *mut cairo_composite_rectangles_t,
                *const cairo_path_fixed_t,
                cairo_fill_rule_t,
                libc::c_double,
                cairo_antialias_t,
            ) -> cairo_int_status_t,
    );
    let ref mut fresh12 = (*compositor).base.stroke;
    *fresh12 = Some(
        _cairo_traps_compositor_stroke
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
    let ref mut fresh13 = (*compositor).base.glyphs;
    *fresh13 = Some(
        _cairo_traps_compositor_glyphs
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
