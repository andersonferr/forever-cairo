use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type pixman_glyph_cache_t;
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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _cairo_bentley_ottmann_tessellate_traps(
        traps: *mut cairo_traps_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn pixman_blt(
        src_bits: *mut uint32_t,
        dst_bits: *mut uint32_t,
        src_stride: libc::c_int,
        dst_stride: libc::c_int,
        src_bpp: libc::c_int,
        dst_bpp: libc::c_int,
        src_x: libc::c_int,
        src_y: libc::c_int,
        dest_x: libc::c_int,
        dest_y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> pixman_bool_t;
    fn pixman_fill(
        bits: *mut uint32_t,
        stride: libc::c_int,
        bpp: libc::c_int,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        _xor: uint32_t,
    ) -> pixman_bool_t;
    fn pixman_image_create_bits(
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        bits: *mut uint32_t,
        rowstride_bytes: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_image_unref(image: *mut pixman_image_t) -> pixman_bool_t;
    fn pixman_image_set_destroy_function(
        image: *mut pixman_image_t,
        function: pixman_image_destroy_func_t,
        data: *mut libc::c_void,
    );
    fn pixman_image_set_clip_region32(
        image: *mut pixman_image_t,
        region: *mut pixman_region32_t,
    ) -> pixman_bool_t;
    fn pixman_image_get_data(image: *mut pixman_image_t) -> *mut uint32_t;
    fn pixman_image_get_stride(image: *mut pixman_image_t) -> libc::c_int;
    fn pixman_image_composite32(
        op: pixman_op_t,
        src: *mut pixman_image_t,
        mask: *mut pixman_image_t,
        dest: *mut pixman_image_t,
        src_x: int32_t,
        src_y: int32_t,
        mask_x: int32_t,
        mask_y: int32_t,
        dest_x: int32_t,
        dest_y: int32_t,
        width: int32_t,
        height: int32_t,
    );
    fn pixman_glyph_cache_create() -> *mut pixman_glyph_cache_t;
    fn pixman_glyph_cache_destroy(cache: *mut pixman_glyph_cache_t);
    fn pixman_glyph_cache_freeze(cache: *mut pixman_glyph_cache_t);
    fn pixman_glyph_cache_thaw(cache: *mut pixman_glyph_cache_t);
    fn pixman_glyph_cache_lookup(
        cache: *mut pixman_glyph_cache_t,
        font_key: *mut libc::c_void,
        glyph_key: *mut libc::c_void,
    ) -> *const libc::c_void;
    fn pixman_glyph_cache_insert(
        cache: *mut pixman_glyph_cache_t,
        font_key: *mut libc::c_void,
        glyph_key: *mut libc::c_void,
        origin_x: libc::c_int,
        origin_y: libc::c_int,
        glyph_image: *mut pixman_image_t,
    ) -> *const libc::c_void;
    fn pixman_glyph_cache_remove(
        cache: *mut pixman_glyph_cache_t,
        font_key: *mut libc::c_void,
        glyph_key: *mut libc::c_void,
    );
    fn pixman_glyph_get_mask_format(
        cache: *mut pixman_glyph_cache_t,
        n_glyphs: libc::c_int,
        glyphs: *const pixman_glyph_t,
    ) -> pixman_format_code_t;
    fn pixman_composite_glyphs(
        op: pixman_op_t,
        src: *mut pixman_image_t,
        dest: *mut pixman_image_t,
        mask_format: pixman_format_code_t,
        src_x: int32_t,
        src_y: int32_t,
        mask_x: int32_t,
        mask_y: int32_t,
        dest_x: int32_t,
        dest_y: int32_t,
        width: int32_t,
        height: int32_t,
        cache: *mut pixman_glyph_cache_t,
        n_glyphs: libc::c_int,
        glyphs: *const pixman_glyph_t,
    );
    fn pixman_composite_glyphs_no_mask(
        op: pixman_op_t,
        src: *mut pixman_image_t,
        dest: *mut pixman_image_t,
        src_x: int32_t,
        src_y: int32_t,
        dest_x: int32_t,
        dest_y: int32_t,
        cache: *mut pixman_glyph_cache_t,
        n_glyphs: libc::c_int,
        glyphs: *const pixman_glyph_t,
    );
    fn pixman_rasterize_trapezoid(
        image: *mut pixman_image_t,
        trap: *const pixman_trapezoid_t,
        x_off: libc::c_int,
        y_off: libc::c_int,
    );
    fn pixman_add_triangles(
        image: *mut pixman_image_t,
        x_off: int32_t,
        y_off: int32_t,
        n_tris: libc::c_int,
        tris: *const pixman_triangle_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    static mut _cairo_glyph_cache_mutex: cairo_mutex_t;
    fn _cairo_image_source_create_for_pattern(
        dst: *mut cairo_surface_t,
        pattern: *const cairo_pattern_t,
        is_mask: cairo_bool_t,
        extents: *const cairo_rectangle_int_t,
        sample: *const cairo_rectangle_int_t,
        src_x: *mut libc::c_int,
        src_y: *mut libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _pixman_image_for_color(cairo_color: *const cairo_color_t) -> *mut pixman_image_t;
    fn _pixman_image_for_pattern(
        dst: *mut cairo_image_surface_t,
        pattern: *const cairo_pattern_t,
        is_mask: cairo_bool_t,
        extents: *const cairo_rectangle_int_t,
        sample: *const cairo_rectangle_int_t,
        tx: *mut libc::c_int,
        ty: *mut libc::c_int,
    ) -> *mut pixman_image_t;
    fn _cairo_pattern_is_opaque_solid(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_pattern_is_opaque(
        pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_mask_compositor_init(
        compositor: *mut cairo_mask_compositor_t,
        delegate: *const cairo_compositor_t,
    );
    fn _cairo_traps_compositor_init(
        compositor: *mut cairo_traps_compositor_t,
        delegate: *const cairo_compositor_t,
    );
    fn _cairo_shape_mask_compositor_init(
        compositor: *mut cairo_compositor_t,
        delegate: *const cairo_compositor_t,
    );
    static __cairo_no_compositor: cairo_compositor_t;
    fn _cairo_spans_compositor_init(
        compositor: *mut cairo_spans_compositor_t,
        delegate: *const cairo_compositor_t,
    );
    fn _cairo_traps_init(traps: *mut cairo_traps_t);
    fn _cairo_traps_fini(traps: *mut cairo_traps_t);
    fn _cairo_traps_tessellate_convex_quad(
        traps: *mut cairo_traps_t,
        q: *const cairo_point_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type pixman_bool_t = libc::c_int;
pub type pixman_fixed_16_16_t = int32_t;
pub type pixman_fixed_t = pixman_fixed_16_16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_point_fixed {
    pub x: pixman_fixed_t,
    pub y: pixman_fixed_t,
}
pub type pixman_point_fixed_t = pixman_point_fixed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_line_fixed {
    pub p1: pixman_point_fixed_t,
    pub p2: pixman_point_fixed_t,
}
pub type pixman_line_fixed_t = pixman_line_fixed;
pub type pixman_op_t = libc::c_uint;
pub const PIXMAN_OP_HSL_LUMINOSITY: pixman_op_t = 62;
pub const PIXMAN_OP_HSL_COLOR: pixman_op_t = 61;
pub const PIXMAN_OP_HSL_SATURATION: pixman_op_t = 60;
pub const PIXMAN_OP_HSL_HUE: pixman_op_t = 59;
pub const PIXMAN_OP_EXCLUSION: pixman_op_t = 58;
pub const PIXMAN_OP_DIFFERENCE: pixman_op_t = 57;
pub const PIXMAN_OP_SOFT_LIGHT: pixman_op_t = 56;
pub const PIXMAN_OP_HARD_LIGHT: pixman_op_t = 55;
pub const PIXMAN_OP_COLOR_BURN: pixman_op_t = 54;
pub const PIXMAN_OP_COLOR_DODGE: pixman_op_t = 53;
pub const PIXMAN_OP_LIGHTEN: pixman_op_t = 52;
pub const PIXMAN_OP_DARKEN: pixman_op_t = 51;
pub const PIXMAN_OP_OVERLAY: pixman_op_t = 50;
pub const PIXMAN_OP_SCREEN: pixman_op_t = 49;
pub const PIXMAN_OP_MULTIPLY: pixman_op_t = 48;
pub const PIXMAN_OP_CONJOINT_XOR: pixman_op_t = 43;
pub const PIXMAN_OP_CONJOINT_ATOP_REVERSE: pixman_op_t = 42;
pub const PIXMAN_OP_CONJOINT_ATOP: pixman_op_t = 41;
pub const PIXMAN_OP_CONJOINT_OUT_REVERSE: pixman_op_t = 40;
pub const PIXMAN_OP_CONJOINT_OUT: pixman_op_t = 39;
pub const PIXMAN_OP_CONJOINT_IN_REVERSE: pixman_op_t = 38;
pub const PIXMAN_OP_CONJOINT_IN: pixman_op_t = 37;
pub const PIXMAN_OP_CONJOINT_OVER_REVERSE: pixman_op_t = 36;
pub const PIXMAN_OP_CONJOINT_OVER: pixman_op_t = 35;
pub const PIXMAN_OP_CONJOINT_DST: pixman_op_t = 34;
pub const PIXMAN_OP_CONJOINT_SRC: pixman_op_t = 33;
pub const PIXMAN_OP_CONJOINT_CLEAR: pixman_op_t = 32;
pub const PIXMAN_OP_DISJOINT_XOR: pixman_op_t = 27;
pub const PIXMAN_OP_DISJOINT_ATOP_REVERSE: pixman_op_t = 26;
pub const PIXMAN_OP_DISJOINT_ATOP: pixman_op_t = 25;
pub const PIXMAN_OP_DISJOINT_OUT_REVERSE: pixman_op_t = 24;
pub const PIXMAN_OP_DISJOINT_OUT: pixman_op_t = 23;
pub const PIXMAN_OP_DISJOINT_IN_REVERSE: pixman_op_t = 22;
pub const PIXMAN_OP_DISJOINT_IN: pixman_op_t = 21;
pub const PIXMAN_OP_DISJOINT_OVER_REVERSE: pixman_op_t = 20;
pub const PIXMAN_OP_DISJOINT_OVER: pixman_op_t = 19;
pub const PIXMAN_OP_DISJOINT_DST: pixman_op_t = 18;
pub const PIXMAN_OP_DISJOINT_SRC: pixman_op_t = 17;
pub const PIXMAN_OP_DISJOINT_CLEAR: pixman_op_t = 16;
pub const PIXMAN_OP_SATURATE: pixman_op_t = 13;
pub const PIXMAN_OP_ADD: pixman_op_t = 12;
pub const PIXMAN_OP_XOR: pixman_op_t = 11;
pub const PIXMAN_OP_ATOP_REVERSE: pixman_op_t = 10;
pub const PIXMAN_OP_ATOP: pixman_op_t = 9;
pub const PIXMAN_OP_OUT_REVERSE: pixman_op_t = 8;
pub const PIXMAN_OP_OUT: pixman_op_t = 7;
pub const PIXMAN_OP_IN_REVERSE: pixman_op_t = 6;
pub const PIXMAN_OP_IN: pixman_op_t = 5;
pub const PIXMAN_OP_OVER_REVERSE: pixman_op_t = 4;
pub const PIXMAN_OP_OVER: pixman_op_t = 3;
pub const PIXMAN_OP_DST: pixman_op_t = 2;
pub const PIXMAN_OP_SRC: pixman_op_t = 1;
pub const PIXMAN_OP_CLEAR: pixman_op_t = 0;
pub type pixman_image_destroy_func_t = Option::<
    unsafe extern "C" fn(*mut pixman_image_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_glyph_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub glyph: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_trapezoid {
    pub top: pixman_fixed_t,
    pub bottom: pixman_fixed_t,
    pub left: pixman_line_fixed_t,
    pub right: pixman_line_fixed_t,
}
pub type pixman_trapezoid_t = pixman_trapezoid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_triangle {
    pub p1: pixman_point_fixed_t,
    pub p2: pixman_point_fixed_t,
    pub p3: pixman_point_fixed_t,
}
pub type pixman_triangle_t = pixman_triangle;
pub type cairo_fixed_16_16_t = int32_t;
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
pub struct cairo_spans_compositor {
    pub base: cairo_compositor_t,
    pub flags: libc::c_uint,
    pub fill_boxes: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_color_t,
            *mut cairo_boxes_t,
        ) -> cairo_int_status_t,
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
    pub renderer_init: Option::<
        unsafe extern "C" fn(
            *mut cairo_abstract_span_renderer_t,
            *const cairo_composite_rectangles_t,
            cairo_antialias_t,
            cairo_bool_t,
        ) -> cairo_int_status_t,
    >,
    pub renderer_fini: Option::<
        unsafe extern "C" fn(
            *mut cairo_abstract_span_renderer_t,
            cairo_int_status_t,
        ) -> (),
    >,
}
pub type cairo_abstract_span_renderer_t = _cairo_abstract_span_renderer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_abstract_span_renderer {
    pub base: cairo_span_renderer_t,
    pub data: [libc::c_char; 4096],
}
pub type cairo_span_renderer_t = _cairo_span_renderer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_span_renderer {
    pub status: cairo_status_t,
    pub destroy: cairo_destroy_func_t,
    pub render_rows: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            *const cairo_half_open_span_t,
            libc::c_uint,
        ) -> cairo_status_t,
    >,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
}
pub type cairo_half_open_span_t = _cairo_half_open_span;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_half_open_span {
    pub x: int32_t,
    pub coverage: uint8_t,
    pub inverse: uint8_t,
}
pub type cairo_spans_compositor_t = cairo_spans_compositor;
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
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
pub type cairo_image_span_renderer_t = _cairo_image_span_renderer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_image_span_renderer {
    pub base: cairo_span_renderer_t,
    pub composite: *const cairo_composite_rectangles_t,
    pub opacity: libc::c_float,
    pub op: uint8_t,
    pub bpp: libc::c_int,
    pub src: *mut pixman_image_t,
    pub mask: *mut pixman_image_t,
    pub u: C2RustUnnamed_0,
    pub _buf: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub fill: fill,
    pub blit: blit,
    pub composite: composite,
    pub mask: finish,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct finish {
    pub extents: cairo_rectangle_int_t,
    pub src_x: libc::c_int,
    pub src_y: libc::c_int,
    pub stride: ptrdiff_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite {
    pub dst: *mut pixman_image_t,
    pub src_x: libc::c_int,
    pub src_y: libc::c_int,
    pub mask_x: libc::c_int,
    pub mask_y: libc::c_int,
    pub run_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blit {
    pub stride: libc::c_int,
    pub data: *mut uint8_t,
    pub src_stride: libc::c_int,
    pub src_data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fill {
    pub stride: ptrdiff_t,
    pub data: *mut uint8_t,
    pub pixel: uint32_t,
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
unsafe extern "C" fn _cairo_fixed_to_16_16(mut f: cairo_fixed_t) -> cairo_fixed_16_16_t {
    let mut x: cairo_fixed_16_16_t = 0;
    if (f >> 8 as libc::c_int) < -(32767 as libc::c_int) - 1 as libc::c_int {
        x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    } else if f >> 8 as libc::c_int > 32767 as libc::c_int {
        x = 2147483647 as libc::c_int;
    } else {
        x = f << 16 as libc::c_int - 8 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
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
    let fresh2 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh2.0;
    return fresh2.1 as cairo_bool_t;
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
unsafe extern "C" fn to_pixman_image(
    mut s: *mut cairo_surface_t,
) -> *mut pixman_image_t {
    return (*(s as *mut cairo_image_surface_t)).pixman_image;
}
unsafe extern "C" fn acquire(mut abstract_dst: *mut libc::c_void) -> cairo_int_status_t {
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn release(mut abstract_dst: *mut libc::c_void) -> cairo_int_status_t {
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn set_clip_region(
    mut _surface: *mut libc::c_void,
    mut region: *mut cairo_region_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_image_surface_t = _surface as *mut cairo_image_surface_t;
    let mut rgn: *mut pixman_region32_t = if !region.is_null() {
        &mut (*region).rgn
    } else {
        0 as *mut pixman_region32_t
    };
    if pixman_image_set_clip_region32((*surface).pixman_image, rgn) == 0 {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn draw_image_boxes(
    mut _dst: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut boxes: *mut cairo_boxes_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = _dst as *mut cairo_image_surface_t;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let mut b: *mut cairo_box_t = &mut *((*chunk).base).offset(i as isize)
                as *mut cairo_box_t;
            let mut x: libc::c_int = _cairo_fixed_integer_part((*b).p1.x);
            let mut y: libc::c_int = _cairo_fixed_integer_part((*b).p1.y);
            let mut w: libc::c_int = _cairo_fixed_integer_part((*b).p2.x) - x;
            let mut h: libc::c_int = _cairo_fixed_integer_part((*b).p2.y) - y;
            if (*dst).pixman_format as libc::c_uint
                != (*image).pixman_format as libc::c_uint
                || pixman_blt(
                    (*image).data as *mut uint32_t,
                    (*dst).data as *mut uint32_t,
                    ((*image).stride as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                        as libc::c_int,
                    ((*dst).stride as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                        as libc::c_int,
                    (((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << ((*image).pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint)) as libc::c_int,
                    (((*dst).pixman_format as libc::c_uint >> 24 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << ((*dst).pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint)) as libc::c_int,
                    x + dx,
                    y + dy,
                    x,
                    y,
                    w,
                    h,
                ) == 0
            {
                pixman_image_composite32(
                    PIXMAN_OP_SRC,
                    (*image).pixman_image,
                    0 as *mut pixman_image_t,
                    (*dst).pixman_image,
                    x + dx,
                    y + dy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x,
                    y,
                    w,
                    h,
                );
            }
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[inline]
unsafe extern "C" fn color_to_uint32(mut color: *const cairo_color_t) -> uint32_t {
    return ((*color).alpha_short as uint32_t >> 8 as libc::c_int) << 24 as libc::c_int
        | (((*color).red_short as libc::c_int >> 8 as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
        | ((*color).green_short as libc::c_int & 0xff00 as libc::c_int) as libc::c_uint
        | ((*color).blue_short as libc::c_int >> 8 as libc::c_int) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn color_to_pixel(
    mut color: *const cairo_color_t,
    mut format: pixman_format_code_t,
    mut pixel: *mut uint32_t,
) -> cairo_bool_t {
    let mut c: uint32_t = 0;
    if !(format as libc::c_uint == PIXMAN_a8r8g8b8 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_x8r8g8b8 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_a8b8g8r8 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_x8b8g8r8 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_b8g8r8a8 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_b8g8r8x8 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_r5g6b5 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_b5g6r5 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_a8 as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    c = color_to_uint32(color);
    if format as libc::c_uint >> 16 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
        == 3 as libc::c_int as libc::c_uint
    {
        c = (c & 0xff000000 as libc::c_uint) >> 0 as libc::c_int
            | (c & 0xff0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int
            | (c & 0xff00 as libc::c_int as libc::c_uint) >> 0 as libc::c_int
            | (c & 0xff as libc::c_int as libc::c_uint) << 16 as libc::c_int;
    }
    if format as libc::c_uint >> 16 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
        == 8 as libc::c_int as libc::c_uint
    {
        c = (c & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
            | (c & 0xff0000 as libc::c_int as libc::c_uint) >> 8 as libc::c_int
            | (c & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
            | (c & 0xff as libc::c_int as libc::c_uint) << 24 as libc::c_int;
    }
    if format as libc::c_uint == PIXMAN_a8 as libc::c_int as libc::c_uint {
        c = c >> 24 as libc::c_int;
    } else if format as libc::c_uint == PIXMAN_r5g6b5 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PIXMAN_b5g6r5 as libc::c_int as libc::c_uint
    {
        c = c >> 3 as libc::c_int & 0x1f as libc::c_int as libc::c_uint
            | c >> 5 as libc::c_int & 0x7e0 as libc::c_int as libc::c_uint
            | c >> 8 as libc::c_int & 0xf800 as libc::c_int as libc::c_uint;
    }
    *pixel = c;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _pixman_operator(mut op: cairo_operator_t) -> pixman_op_t {
    match op as libc::c_int {
        0 => return PIXMAN_OP_CLEAR,
        1 => return PIXMAN_OP_SRC,
        2 => return PIXMAN_OP_OVER,
        3 => return PIXMAN_OP_IN,
        4 => return PIXMAN_OP_OUT,
        5 => return PIXMAN_OP_ATOP,
        6 => return PIXMAN_OP_DST,
        7 => return PIXMAN_OP_OVER_REVERSE,
        8 => return PIXMAN_OP_IN_REVERSE,
        9 => return PIXMAN_OP_OUT_REVERSE,
        10 => return PIXMAN_OP_ATOP_REVERSE,
        11 => return PIXMAN_OP_XOR,
        12 => return PIXMAN_OP_ADD,
        13 => return PIXMAN_OP_SATURATE,
        14 => return PIXMAN_OP_MULTIPLY,
        15 => return PIXMAN_OP_SCREEN,
        16 => return PIXMAN_OP_OVERLAY,
        17 => return PIXMAN_OP_DARKEN,
        18 => return PIXMAN_OP_LIGHTEN,
        19 => return PIXMAN_OP_COLOR_DODGE,
        20 => return PIXMAN_OP_COLOR_BURN,
        21 => return PIXMAN_OP_HARD_LIGHT,
        22 => return PIXMAN_OP_SOFT_LIGHT,
        23 => return PIXMAN_OP_DIFFERENCE,
        24 => return PIXMAN_OP_EXCLUSION,
        25 => return PIXMAN_OP_HSL_HUE,
        26 => return PIXMAN_OP_HSL_SATURATION,
        27 => return PIXMAN_OP_HSL_COLOR,
        28 => return PIXMAN_OP_HSL_LUMINOSITY,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-image-compositor.c\0" as *const u8
                        as *const libc::c_char,
                    255 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 47],
                        &[libc::c_char; 47],
                    >(b"pixman_op_t _pixman_operator(cairo_operator_t)\0"))
                        .as_ptr(),
                );
            }
            return PIXMAN_OP_OVER;
        }
    };
}
unsafe extern "C" fn __fill_reduces_to_source(
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut dst: *const cairo_image_surface_t,
) -> cairo_bool_t {
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
        && (*color).alpha_short as libc::c_int >= 0xff00 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if ((*dst).base).is_clear() != 0 {
        return (op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint)
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fill_reduces_to_source(
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut dst: *const cairo_image_surface_t,
    mut pixel: *mut uint32_t,
) -> cairo_bool_t {
    if __fill_reduces_to_source(op, color, dst) != 0 {
        return color_to_pixel(color, (*dst).pixman_format, pixel);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fill_rectangles(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut rects: *mut cairo_rectangle_int_t,
    mut num_rects: libc::c_int,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = _dst as *mut cairo_image_surface_t;
    let mut pixel: uint32_t = 0;
    let mut i: libc::c_int = 0;
    if fill_reduces_to_source(op, color, dst, &mut pixel) != 0 {
        i = 0 as libc::c_int;
        while i < num_rects {
            pixman_fill(
                (*dst).data as *mut uint32_t,
                ((*dst).stride as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as libc::c_int,
                (((*dst).pixman_format as libc::c_uint >> 24 as libc::c_int
                    & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << ((*dst).pixman_format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint)) as libc::c_int,
                (*rects.offset(i as isize)).x,
                (*rects.offset(i as isize)).y,
                (*rects.offset(i as isize)).width,
                (*rects.offset(i as isize)).height,
                pixel,
            );
            i += 1;
        }
    } else {
        let mut src: *mut pixman_image_t = _pixman_image_for_color(color);
        if src.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        op = _pixman_operator(op) as cairo_operator_t;
        i = 0 as libc::c_int;
        while i < num_rects {
            pixman_image_composite32(
                op as pixman_op_t,
                src,
                0 as *mut pixman_image_t,
                (*dst).pixman_image,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*rects.offset(i as isize)).x,
                (*rects.offset(i as isize)).y,
                (*rects.offset(i as isize)).width,
                (*rects.offset(i as isize)).height,
            );
            i += 1;
        }
        pixman_image_unref(src);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn fill_boxes(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = _dst as *mut cairo_image_surface_t;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut pixel: uint32_t = 0;
    let mut i: libc::c_int = 0;
    if fill_reduces_to_source(op, color, dst, &mut pixel) != 0 {
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.x,
                );
                let mut y: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.y,
                );
                let mut w: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.x,
                ) - x;
                let mut h: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.y,
                ) - y;
                pixman_fill(
                    (*dst).data as *mut uint32_t,
                    ((*dst).stride as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                        as libc::c_int,
                    (((*dst).pixman_format as libc::c_uint >> 24 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                        << ((*dst).pixman_format as libc::c_uint >> 22 as libc::c_int
                            & 3 as libc::c_int as libc::c_uint)) as libc::c_int,
                    x,
                    y,
                    w,
                    h,
                    pixel,
                );
                i += 1;
            }
            chunk = (*chunk).next;
        }
    } else {
        let mut src: *mut pixman_image_t = _pixman_image_for_color(color);
        if src.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        op = _pixman_operator(op) as cairo_operator_t;
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x1: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.x,
                );
                let mut y1: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.y,
                );
                let mut x2: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.x,
                );
                let mut y2: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.y,
                );
                pixman_image_composite32(
                    op as pixman_op_t,
                    src,
                    0 as *mut pixman_image_t,
                    (*dst).pixman_image,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x1,
                    y1,
                    x2 - x1,
                    y2 - y1,
                );
                i += 1;
            }
            chunk = (*chunk).next;
        }
        pixman_image_unref(src);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut abstract_src: *mut cairo_surface_t,
    mut abstract_mask: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut mask_x: libc::c_int,
    mut mask_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) -> cairo_int_status_t {
    let mut src: *mut cairo_image_source_t = abstract_src as *mut cairo_image_source_t;
    let mut mask: *mut cairo_image_source_t = abstract_mask as *mut cairo_image_source_t;
    if !mask.is_null() {
        pixman_image_composite32(
            _pixman_operator(op),
            (*src).pixman_image,
            (*mask).pixman_image,
            to_pixman_image(_dst as *mut cairo_surface_t),
            src_x,
            src_y,
            mask_x,
            mask_y,
            dst_x,
            dst_y,
            width as int32_t,
            height as int32_t,
        );
    } else {
        pixman_image_composite32(
            _pixman_operator(op),
            (*src).pixman_image,
            0 as *mut pixman_image_t,
            to_pixman_image(_dst as *mut cairo_surface_t),
            src_x,
            src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            dst_x,
            dst_y,
            width as int32_t,
            height as int32_t,
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn lerp(
    mut _dst: *mut libc::c_void,
    mut abstract_src: *mut cairo_surface_t,
    mut abstract_mask: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut mask_x: libc::c_int,
    mut mask_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = _dst as *mut cairo_image_surface_t;
    let mut src: *mut cairo_image_source_t = abstract_src as *mut cairo_image_source_t;
    let mut mask: *mut cairo_image_source_t = abstract_mask as *mut cairo_image_source_t;
    pixman_image_composite32(
        PIXMAN_OP_OUT_REVERSE,
        (*mask).pixman_image,
        0 as *mut pixman_image_t,
        (*dst).pixman_image,
        mask_x,
        mask_y,
        0 as libc::c_int,
        0 as libc::c_int,
        dst_x,
        dst_y,
        width as int32_t,
        height as int32_t,
    );
    pixman_image_composite32(
        PIXMAN_OP_ADD,
        (*src).pixman_image,
        (*mask).pixman_image,
        (*dst).pixman_image,
        src_x,
        src_y,
        mask_x,
        mask_y,
        dst_x,
        dst_y,
        width as int32_t,
        height as int32_t,
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_boxes(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut abstract_src: *mut cairo_surface_t,
    mut abstract_mask: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut mask_x: libc::c_int,
    mut mask_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut boxes: *mut cairo_boxes_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut dst: *mut pixman_image_t = to_pixman_image(_dst as *mut cairo_surface_t);
    let mut src: *mut pixman_image_t = (*(abstract_src as *mut cairo_image_source_t))
        .pixman_image;
    let mut mask: *mut pixman_image_t = if !abstract_mask.is_null() {
        (*(abstract_mask as *mut cairo_image_source_t)).pixman_image
    } else {
        0 as *mut pixman_image_t
    };
    let mut free_src: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    if (*(_dst as *mut cairo_surface_t)).is_clear() as libc::c_int != 0
        && (op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint)
    {
        op = PIXMAN_OP_SRC as libc::c_int as cairo_operator_t;
    } else if !mask.is_null() {
        if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
            src = _pixman_image_for_color(_cairo_stock_color(CAIRO_STOCK_WHITE));
            free_src = src;
            if src.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            op = PIXMAN_OP_OUT_REVERSE as libc::c_int as cairo_operator_t;
        } else if op as libc::c_uint
            == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        {
            return CAIRO_INT_STATUS_UNSUPPORTED
        } else {
            op = _pixman_operator(op) as cairo_operator_t;
        }
    } else {
        op = _pixman_operator(op) as cairo_operator_t;
    }
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let mut x1: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p1.x,
            );
            let mut y1: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p1.y,
            );
            let mut x2: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p2.x,
            );
            let mut y2: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p2.y,
            );
            pixman_image_composite32(
                op as pixman_op_t,
                src,
                mask,
                dst,
                x1 + src_x,
                y1 + src_y,
                x1 + mask_x,
                y1 + mask_y,
                x1 + dst_x,
                y1 + dst_y,
                x2 - x1,
                y2 - y1,
            );
            i += 1;
        }
        chunk = (*chunk).next;
    }
    if !free_src.is_null() {
        pixman_image_unref(free_src);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn line_exceeds_16_16(mut line: *const cairo_line_t) -> cairo_bool_t {
    return ((*line).p1.x <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p1.x >= _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p2.x <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p2.x >= _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p1.y <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p1.y >= _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p2.y <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p2.y >= _cairo_fixed_from_int(32767 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn project_line_x_onto_16_16(
    mut line: *const cairo_line_t,
    mut top: cairo_fixed_t,
    mut bottom: cairo_fixed_t,
    mut out: *mut pixman_line_fixed_t,
) {
    let mut p1: cairo_point_double_t = cairo_point_double_t {
        x: 0.,
        y: 0.,
    };
    let mut p2: cairo_point_double_t = cairo_point_double_t {
        x: 0.,
        y: 0.,
    };
    let mut m: libc::c_double = 0.;
    p1.x = _cairo_fixed_to_double((*line).p1.x);
    p1.y = _cairo_fixed_to_double((*line).p1.y);
    p2.x = _cairo_fixed_to_double((*line).p2.x);
    p2.y = _cairo_fixed_to_double((*line).p2.y);
    m = (p2.x - p1.x) / (p2.y - p1.y);
    (*out)
        .p1
        .x = _cairo_fixed_16_16_from_double(
        p1.x + m * _cairo_fixed_to_double(top - (*line).p1.y),
    );
    (*out)
        .p2
        .x = _cairo_fixed_16_16_from_double(
        p1.x + m * _cairo_fixed_to_double(bottom - (*line).p1.y),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_image_add_traps(
    mut image: *mut pixman_image_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut traps: *mut cairo_traps_t,
) {
    let mut t: *mut cairo_trapezoid_t = (*traps).traps;
    let mut num_traps: libc::c_int = (*traps).num_traps;
    loop {
        let fresh3 = num_traps;
        num_traps = num_traps - 1;
        if !(fresh3 != 0) {
            break;
        }
        let mut trap: pixman_trapezoid_t = pixman_trapezoid_t {
            top: 0,
            bottom: 0,
            left: pixman_line_fixed_t {
                p1: pixman_point_fixed_t { x: 0, y: 0 },
                p2: pixman_point_fixed_t { x: 0, y: 0 },
            },
            right: pixman_line_fixed_t {
                p1: pixman_point_fixed_t { x: 0, y: 0 },
                p2: pixman_point_fixed_t { x: 0, y: 0 },
            },
        };
        trap.top = _cairo_fixed_to_16_16((*t).top);
        trap.bottom = _cairo_fixed_to_16_16((*t).bottom);
        if line_exceeds_16_16(&mut (*t).left) != 0 {
            project_line_x_onto_16_16(
                &mut (*t).left,
                (*t).top,
                (*t).bottom,
                &mut trap.left,
            );
            trap.left.p1.y = trap.top;
            trap.left.p2.y = trap.bottom;
        } else {
            trap.left.p1.x = _cairo_fixed_to_16_16((*t).left.p1.x);
            trap.left.p1.y = _cairo_fixed_to_16_16((*t).left.p1.y);
            trap.left.p2.x = _cairo_fixed_to_16_16((*t).left.p2.x);
            trap.left.p2.y = _cairo_fixed_to_16_16((*t).left.p2.y);
        }
        if line_exceeds_16_16(&mut (*t).right) != 0 {
            project_line_x_onto_16_16(
                &mut (*t).right,
                (*t).top,
                (*t).bottom,
                &mut trap.right,
            );
            trap.right.p1.y = trap.top;
            trap.right.p2.y = trap.bottom;
        } else {
            trap.right.p1.x = _cairo_fixed_to_16_16((*t).right.p1.x);
            trap.right.p1.y = _cairo_fixed_to_16_16((*t).right.p1.y);
            trap.right.p2.x = _cairo_fixed_to_16_16((*t).right.p2.x);
            trap.right.p2.y = _cairo_fixed_to_16_16((*t).right.p2.y);
        }
        pixman_rasterize_trapezoid(image, &mut trap, -dst_x, -dst_y);
        t = t.offset(1);
    };
}
unsafe extern "C" fn composite_traps(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut abstract_src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut antialias: cairo_antialias_t,
    mut traps: *mut cairo_traps_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = _dst as *mut cairo_image_surface_t;
    let mut src: *mut cairo_image_source_t = abstract_src as *mut cairo_image_source_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut mask: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut format: pixman_format_code_t = 0 as pixman_format_code_t;
    status = _cairo_bentley_ottmann_tessellate_traps(traps, CAIRO_FILL_RULE_WINDING)
        as cairo_int_status_t;
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return status;
    }
    format = (if antialias as libc::c_uint
        == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
    {
        PIXMAN_a1 as libc::c_int
    } else {
        PIXMAN_a8 as libc::c_int
    }) as pixman_format_code_t;
    if (*dst).pixman_format as libc::c_uint == format as libc::c_uint
        && (abstract_src.is_null()
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint
                && (*src).is_opaque_solid() as libc::c_int != 0)
    {
        _pixman_image_add_traps((*dst).pixman_image, dst_x, dst_y, traps);
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    mask = pixman_image_create_bits(
        format,
        (*extents).width,
        (*extents).height,
        0 as *mut uint32_t,
        0 as libc::c_int,
    );
    if mask.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _pixman_image_add_traps(mask, (*extents).x, (*extents).y, traps);
    pixman_image_composite32(
        _pixman_operator(op),
        (*src).pixman_image,
        mask,
        (*dst).pixman_image,
        (*extents).x + src_x,
        (*extents).y + src_y,
        0 as libc::c_int,
        0 as libc::c_int,
        (*extents).x - dst_x,
        (*extents).y - dst_y,
        (*extents).width,
        (*extents).height,
    );
    pixman_image_unref(mask);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn set_point(
    mut p: *mut pixman_point_fixed_t,
    mut c: *mut cairo_point_t,
) {
    (*p).x = _cairo_fixed_to_16_16((*c).x);
    (*p).y = _cairo_fixed_to_16_16((*c).y);
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_image_add_tristrip(
    mut image: *mut pixman_image_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut strip: *mut cairo_tristrip_t,
) {
    let mut tri: pixman_triangle_t = pixman_triangle_t {
        p1: pixman_point_fixed_t { x: 0, y: 0 },
        p2: pixman_point_fixed_t { x: 0, y: 0 },
        p3: pixman_point_fixed_t { x: 0, y: 0 },
    };
    let mut p: [*mut pixman_point_fixed_t; 3] = [&mut tri.p1, &mut tri.p2, &mut tri.p3];
    let mut n: libc::c_int = 0;
    set_point(
        p[0 as libc::c_int as usize],
        &mut *((*strip).points).offset(0 as libc::c_int as isize),
    );
    set_point(
        p[1 as libc::c_int as usize],
        &mut *((*strip).points).offset(1 as libc::c_int as isize),
    );
    set_point(
        p[2 as libc::c_int as usize],
        &mut *((*strip).points).offset(2 as libc::c_int as isize),
    );
    pixman_add_triangles(image, -dst_x, -dst_y, 1 as libc::c_int, &mut tri);
    n = 3 as libc::c_int;
    while n < (*strip).num_points {
        set_point(
            p[(n % 3 as libc::c_int) as usize],
            &mut *((*strip).points).offset(n as isize),
        );
        pixman_add_triangles(image, -dst_x, -dst_y, 1 as libc::c_int, &mut tri);
        n += 1;
    }
}
unsafe extern "C" fn composite_tristrip(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut abstract_src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut antialias: cairo_antialias_t,
    mut strip: *mut cairo_tristrip_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = _dst as *mut cairo_image_surface_t;
    let mut src: *mut cairo_image_source_t = abstract_src as *mut cairo_image_source_t;
    let mut mask: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut format: pixman_format_code_t = 0 as pixman_format_code_t;
    if (*strip).num_points < 3 as libc::c_int {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
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
    let mut n: libc::c_int = 0;
    _cairo_traps_init(&mut traps);
    n = 0 as libc::c_int;
    while n < (*strip).num_points {
        let mut p: [cairo_point_t; 4] = [cairo_point_t { x: 0, y: 0 }; 4];
        p[0 as libc::c_int
            as usize] = *((*strip).points).offset(0 as libc::c_int as isize);
        p[1 as libc::c_int
            as usize] = *((*strip).points).offset(1 as libc::c_int as isize);
        p[2 as libc::c_int
            as usize] = *((*strip).points).offset(2 as libc::c_int as isize);
        p[3 as libc::c_int
            as usize] = *((*strip).points).offset(0 as libc::c_int as isize);
        _cairo_traps_tessellate_convex_quad(
            &mut traps,
            p.as_mut_ptr() as *const cairo_point_t,
        );
        n += 1;
    }
    status = composite_traps(
        _dst,
        op,
        abstract_src,
        src_x,
        src_y,
        dst_x,
        dst_y,
        extents,
        antialias,
        &mut traps,
    );
    _cairo_traps_fini(&mut traps);
    return status;
}
unsafe extern "C" fn check_composite_glyphs(
    mut extents: *const cairo_composite_rectangles_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: *mut libc::c_int,
) -> cairo_int_status_t {
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
static mut global_glyph_cache: *mut pixman_glyph_cache_t = 0
    as *const pixman_glyph_cache_t as *mut pixman_glyph_cache_t;
#[inline]
unsafe extern "C" fn get_glyph_cache() -> *mut pixman_glyph_cache_t {
    if global_glyph_cache.is_null() {
        global_glyph_cache = pixman_glyph_cache_create();
    }
    return global_glyph_cache;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_compositor_reset_static_data() {
    pthread_mutex_lock(&mut _cairo_glyph_cache_mutex);
    if !global_glyph_cache.is_null() {
        pixman_glyph_cache_destroy(global_glyph_cache);
    }
    global_glyph_cache = 0 as *mut pixman_glyph_cache_t;
    pthread_mutex_unlock(&mut _cairo_glyph_cache_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_scaled_glyph_fini(
    mut scaled_font: *mut cairo_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
) {
    pthread_mutex_lock(&mut _cairo_glyph_cache_mutex);
    if !global_glyph_cache.is_null() {
        pixman_glyph_cache_remove(
            global_glyph_cache,
            scaled_font as *mut libc::c_void,
            (*scaled_glyph).hash_entry.hash as *mut libc::c_void,
        );
    }
    pthread_mutex_unlock(&mut _cairo_glyph_cache_mutex);
}
unsafe extern "C" fn composite_glyphs(
    mut _dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut _src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut info: *mut cairo_composite_glyphs_info_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut glyph_cache: *mut pixman_glyph_cache_t = 0 as *mut pixman_glyph_cache_t;
    let mut pglyphs_stack: [pixman_glyph_t; 128] = [pixman_glyph_t {
        x: 0,
        y: 0,
        glyph: 0 as *const libc::c_void,
    }; 128];
    let mut pglyphs: *mut pixman_glyph_t = pglyphs_stack.as_mut_ptr();
    let mut pg: *mut pixman_glyph_t = 0 as *mut pixman_glyph_t;
    let mut i: libc::c_int = 0;
    pthread_mutex_lock(&mut _cairo_glyph_cache_mutex);
    glyph_cache = get_glyph_cache();
    if glyph_cache.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    } else {
        pixman_glyph_cache_freeze(glyph_cache);
        if (*info).num_glyphs
            > (::std::mem::size_of::<[pixman_glyph_t; 128]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<pixman_glyph_t>() as libc::c_ulong)
                as libc::c_int
        {
            pglyphs = _cairo_malloc_ab(
                (*info).num_glyphs as size_t,
                ::std::mem::size_of::<pixman_glyph_t>() as libc::c_ulong,
            ) as *mut pixman_glyph_t;
            if pglyphs.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                current_block = 3047497157763451437;
            } else {
                current_block = 12599329904712511516;
            }
        } else {
            current_block = 12599329904712511516;
        }
        match current_block {
            12599329904712511516 => {
                pg = pglyphs;
                i = 0 as libc::c_int;
                loop {
                    if !(i < (*info).num_glyphs) {
                        current_block = 6450636197030046351;
                        break;
                    }
                    let mut index: libc::c_ulong = (*((*info).glyphs).offset(i as isize))
                        .index;
                    let mut glyph: *const libc::c_void = 0 as *const libc::c_void;
                    let mut xphase: libc::c_ulong = 0;
                    let mut yphase: libc::c_ulong = 0;
                    xphase = (floor(
                        4 as libc::c_int as libc::c_double
                            * ((*((*info).glyphs).offset(i as isize)).x + 0.125f64),
                    )
                        - 4 as libc::c_int as libc::c_double
                            * floor((*((*info).glyphs).offset(i as isize)).x + 0.125f64))
                        as libc::c_int as libc::c_ulong;
                    yphase = (floor(
                        4 as libc::c_int as libc::c_double
                            * ((*((*info).glyphs).offset(i as isize)).y + 0.125f64),
                    )
                        - 4 as libc::c_int as libc::c_double
                            * floor((*((*info).glyphs).offset(i as isize)).y + 0.125f64))
                        as libc::c_int as libc::c_ulong;
                    index = index | xphase << 24 as libc::c_int
                        | yphase << 26 as libc::c_int;
                    glyph = pixman_glyph_cache_lookup(
                        glyph_cache,
                        (*info).font as *mut libc::c_void,
                        index as *mut libc::c_void,
                    );
                    if glyph.is_null() {
                        let mut scaled_glyph: *mut cairo_scaled_glyph_t = 0
                            as *mut cairo_scaled_glyph_t;
                        let mut glyph_surface: *mut cairo_image_surface_t = 0
                            as *mut cairo_image_surface_t;
                        pthread_mutex_unlock(&mut _cairo_glyph_cache_mutex);
                        status = _cairo_scaled_glyph_lookup(
                            (*info).font,
                            index,
                            CAIRO_SCALED_GLYPH_INFO_SURFACE,
                            0 as *const cairo_color_t,
                            &mut scaled_glyph,
                        );
                        pthread_mutex_lock(&mut _cairo_glyph_cache_mutex);
                        if status as u64 != 0 {
                            current_block = 3047497157763451437;
                            break;
                        }
                        glyph_surface = (*scaled_glyph).surface;
                        glyph = pixman_glyph_cache_insert(
                            glyph_cache,
                            (*info).font as *mut libc::c_void,
                            index as *mut libc::c_void,
                            (*glyph_surface).base.device_transform.x0 as libc::c_int,
                            (*glyph_surface).base.device_transform.y0 as libc::c_int,
                            (*glyph_surface).pixman_image,
                        );
                        if glyph.is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                as cairo_int_status_t;
                            current_block = 3047497157763451437;
                            break;
                        }
                    }
                    (*pg)
                        .x = floor((*((*info).glyphs).offset(i as isize)).x + 0.125f64)
                        as libc::c_int;
                    (*pg)
                        .y = floor((*((*info).glyphs).offset(i as isize)).y + 0.125f64)
                        as libc::c_int;
                    let ref mut fresh4 = (*pg).glyph;
                    *fresh4 = glyph;
                    pg = pg.offset(1);
                    i += 1;
                }
                match current_block {
                    3047497157763451437 => {}
                    _ => {
                        if (*info).use_mask != 0 {
                            let mut mask_format: pixman_format_code_t = 0
                                as pixman_format_code_t;
                            mask_format = pixman_glyph_get_mask_format(
                                glyph_cache,
                                pg.offset_from(pglyphs) as libc::c_long as libc::c_int,
                                pglyphs,
                            );
                            pixman_composite_glyphs(
                                _pixman_operator(op),
                                (*(_src as *mut cairo_image_source_t)).pixman_image,
                                to_pixman_image(_dst as *mut cairo_surface_t),
                                mask_format,
                                (*info).extents.x + src_x,
                                (*info).extents.y + src_y,
                                (*info).extents.x,
                                (*info).extents.y,
                                (*info).extents.x - dst_x,
                                (*info).extents.y - dst_y,
                                (*info).extents.width,
                                (*info).extents.height,
                                glyph_cache,
                                pg.offset_from(pglyphs) as libc::c_long as libc::c_int,
                                pglyphs,
                            );
                        } else {
                            pixman_composite_glyphs_no_mask(
                                _pixman_operator(op),
                                (*(_src as *mut cairo_image_source_t)).pixman_image,
                                to_pixman_image(_dst as *mut cairo_surface_t),
                                src_x,
                                src_y,
                                -dst_x,
                                -dst_y,
                                glyph_cache,
                                pg.offset_from(pglyphs) as libc::c_long as libc::c_int,
                                pglyphs,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        pixman_glyph_cache_thaw(glyph_cache);
        if pglyphs != pglyphs_stack.as_mut_ptr() {
            free(pglyphs as *mut libc::c_void);
        }
    }
    pthread_mutex_unlock(&mut _cairo_glyph_cache_mutex);
    return status;
}
unsafe extern "C" fn check_composite(
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_traps_compositor_get() -> *const cairo_compositor_t {
    static mut once: cairo_atomic_once_t = 0 as libc::c_int;
    static mut compositor: cairo_traps_compositor_t = cairo_traps_compositor_t {
        base: cairo_compositor_t {
            delegate: 0 as *const cairo_compositor_t,
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            glyphs: None,
        },
        acquire: None,
        release: None,
        set_clip_region: None,
        pattern_to_surface: None,
        draw_image_boxes: None,
        copy_boxes: None,
        fill_boxes: None,
        check_composite: None,
        composite: None,
        lerp: None,
        composite_boxes: None,
        composite_traps: None,
        composite_tristrip: None,
        check_composite_glyphs: None,
        composite_glyphs: None,
    };
    if _cairo_atomic_init_once_enter(&mut once) != 0 {
        _cairo_traps_compositor_init(&mut compositor, &__cairo_no_compositor);
        compositor
            .acquire = Some(
            acquire as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
        );
        compositor
            .release = Some(
            release as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
        );
        compositor
            .set_clip_region = Some(
            set_clip_region
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_region_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .pattern_to_surface = Some(
            _cairo_image_source_create_for_pattern
                as unsafe extern "C" fn(
                    *mut cairo_surface_t,
                    *const cairo_pattern_t,
                    cairo_bool_t,
                    *const cairo_rectangle_int_t,
                    *const cairo_rectangle_int_t,
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> *mut cairo_surface_t,
        );
        compositor
            .draw_image_boxes = Some(
            draw_image_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_image_surface_t,
                    *mut cairo_boxes_t,
                    libc::c_int,
                    libc::c_int,
                ) -> cairo_int_status_t,
        );
        compositor
            .fill_boxes = Some(
            fill_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    cairo_operator_t,
                    *const cairo_color_t,
                    *mut cairo_boxes_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .check_composite = Some(
            check_composite
                as unsafe extern "C" fn(
                    *const cairo_composite_rectangles_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .composite = Some(
            composite
                as unsafe extern "C" fn(
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
        );
        compositor
            .lerp = Some(
            lerp
                as unsafe extern "C" fn(
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
        );
        compositor
            .composite_boxes = Some(
            composite_boxes
                as unsafe extern "C" fn(
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
        );
        compositor
            .composite_traps = Some(
            composite_traps
                as unsafe extern "C" fn(
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
        );
        compositor
            .composite_tristrip = Some(
            composite_tristrip
                as unsafe extern "C" fn(
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
        );
        compositor
            .check_composite_glyphs = Some(
            check_composite_glyphs
                as unsafe extern "C" fn(
                    *const cairo_composite_rectangles_t,
                    *mut cairo_scaled_font_t,
                    *mut cairo_glyph_t,
                    *mut libc::c_int,
                ) -> cairo_int_status_t,
        );
        compositor
            .composite_glyphs = Some(
            composite_glyphs
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    cairo_operator_t,
                    *mut cairo_surface_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *mut cairo_composite_glyphs_info_t,
                ) -> cairo_int_status_t,
        );
        _cairo_atomic_init_once_leave(&mut once);
    }
    return &mut compositor.base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_mask_compositor_get() -> *const cairo_compositor_t {
    static mut once: cairo_atomic_once_t = 0 as libc::c_int;
    static mut compositor: cairo_mask_compositor_t = cairo_mask_compositor_t {
        base: cairo_compositor_t {
            delegate: 0 as *const cairo_compositor_t,
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            glyphs: None,
        },
        acquire: None,
        release: None,
        set_clip_region: None,
        pattern_to_surface: None,
        draw_image_boxes: None,
        copy_boxes: None,
        fill_rectangles: None,
        fill_boxes: None,
        check_composite: None,
        composite: None,
        composite_boxes: None,
        check_composite_glyphs: None,
        composite_glyphs: None,
    };
    if _cairo_atomic_init_once_enter(&mut once) != 0 {
        _cairo_mask_compositor_init(
            &mut compositor,
            _cairo_image_traps_compositor_get(),
        );
        compositor
            .acquire = Some(
            acquire as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
        );
        compositor
            .release = Some(
            release as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
        );
        compositor
            .set_clip_region = Some(
            set_clip_region
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_region_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .pattern_to_surface = Some(
            _cairo_image_source_create_for_pattern
                as unsafe extern "C" fn(
                    *mut cairo_surface_t,
                    *const cairo_pattern_t,
                    cairo_bool_t,
                    *const cairo_rectangle_int_t,
                    *const cairo_rectangle_int_t,
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> *mut cairo_surface_t,
        );
        compositor
            .draw_image_boxes = Some(
            draw_image_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_image_surface_t,
                    *mut cairo_boxes_t,
                    libc::c_int,
                    libc::c_int,
                ) -> cairo_int_status_t,
        );
        compositor
            .fill_rectangles = Some(
            fill_rectangles
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    cairo_operator_t,
                    *const cairo_color_t,
                    *mut cairo_rectangle_int_t,
                    libc::c_int,
                ) -> cairo_int_status_t,
        );
        compositor
            .fill_boxes = Some(
            fill_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    cairo_operator_t,
                    *const cairo_color_t,
                    *mut cairo_boxes_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .check_composite = Some(
            check_composite
                as unsafe extern "C" fn(
                    *const cairo_composite_rectangles_t,
                ) -> cairo_int_status_t,
        );
        compositor
            .composite = Some(
            composite
                as unsafe extern "C" fn(
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
        );
        compositor
            .composite_boxes = Some(
            composite_boxes
                as unsafe extern "C" fn(
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
        );
        compositor
            .check_composite_glyphs = Some(
            check_composite_glyphs
                as unsafe extern "C" fn(
                    *const cairo_composite_rectangles_t,
                    *mut cairo_scaled_font_t,
                    *mut cairo_glyph_t,
                    *mut libc::c_int,
                ) -> cairo_int_status_t,
        );
        compositor
            .composite_glyphs = Some(
            composite_glyphs
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    cairo_operator_t,
                    *mut cairo_surface_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    *mut cairo_composite_glyphs_info_t,
                ) -> cairo_int_status_t,
        );
        _cairo_atomic_init_once_leave(&mut once);
    }
    return &mut compositor.base;
}
unsafe extern "C" fn _cairo_image_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut height: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut mask: *mut uint8_t = 0 as *mut uint8_t;
    let mut row: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: libc::c_int = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    mask = ((*r).u.mask.data)
        .offset(
            ((y - (*r).u.mask.extents.y) as libc::c_long * (*r).u.mask.stride) as isize,
        );
    mask = mask
        .offset(
            ((*spans.offset(0 as libc::c_int as isize)).x - (*r).u.mask.extents.x)
                as isize,
        );
    row = mask;
    loop {
        len = (*spans.offset(1 as libc::c_int as isize)).x
            - (*spans.offset(0 as libc::c_int as isize)).x;
        if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
            let fresh5 = row;
            row = row.offset(1);
            *fresh5 = ((*r).opacity
                * (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
                    as libc::c_float) as uint8_t;
            len -= 1;
            if len != 0 {
                memset(
                    row as *mut libc::c_void,
                    *row.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                    len as libc::c_ulong,
                );
            }
        }
        row = row.offset(len as isize);
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    len = row.offset_from(mask) as libc::c_long as libc::c_int;
    row = mask;
    loop {
        height -= 1;
        if !(height != 0) {
            break;
        }
        mask = mask.offset((*r).u.mask.stride as isize);
        memcpy(
            mask as *mut libc::c_void,
            row as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_image_spans_and_zero(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut height: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut mask: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: libc::c_int = 0;
    mask = (*r).u.mask.data;
    if y > (*r).u.mask.extents.y {
        len = ((y - (*r).u.mask.extents.y) as libc::c_long * (*r).u.mask.stride)
            as libc::c_int;
        memset(mask as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        mask = mask.offset(len as isize);
    }
    (*r).u.mask.extents.y = y + height;
    let ref mut fresh6 = (*r).u.mask.data;
    *fresh6 = mask.offset((height as libc::c_long * (*r).u.mask.stride) as isize);
    if num_spans == 0 as libc::c_int as libc::c_uint {
        memset(
            mask as *mut libc::c_void,
            0 as libc::c_int,
            (height as libc::c_long * (*r).u.mask.stride) as libc::c_ulong,
        );
    } else {
        let mut row: *mut uint8_t = mask;
        if (*spans.offset(0 as libc::c_int as isize)).x != (*r).u.mask.extents.x {
            len = (*spans.offset(0 as libc::c_int as isize)).x - (*r).u.mask.extents.x;
            memset(row as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
            row = row.offset(len as isize);
        }
        loop {
            len = (*spans.offset(1 as libc::c_int as isize)).x
                - (*spans.offset(0 as libc::c_int as isize)).x;
            let fresh7 = row;
            row = row.offset(1);
            *fresh7 = ((*r).opacity
                * (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
                    as libc::c_float) as uint8_t;
            if len > 1 as libc::c_int {
                len -= 1;
                memset(
                    row as *mut libc::c_void,
                    *row.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                    len as libc::c_ulong,
                );
                row = row.offset(len as isize);
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
        if (*spans.offset(0 as libc::c_int as isize)).x
            != (*r).u.mask.extents.x + (*r).u.mask.extents.width
        {
            len = (*r).u.mask.extents.x + (*r).u.mask.extents.width
                - (*spans.offset(0 as libc::c_int as isize)).x;
            memset(row as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        }
        row = mask;
        loop {
            height -= 1;
            if !(height != 0) {
                break;
            }
            mask = mask.offset((*r).u.mask.stride as isize);
            memcpy(
                mask as *mut libc::c_void,
                row as *const libc::c_void,
                (*r).u.mask.extents.width as libc::c_ulong,
            );
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_image_finish_spans_and_zero(
    mut abstract_renderer: *mut libc::c_void,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if (*r).u.mask.extents.y < (*r).u.mask.extents.height {
        memset(
            (*r).u.mask.data as *mut libc::c_void,
            0 as libc::c_int,
            (((*r).u.mask.extents.height - (*r).u.mask.extents.y) as libc::c_long
                * (*r).u.mask.stride) as libc::c_ulong,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill8_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                let mut d: *mut uint8_t = ((*r).u.fill.data)
                    .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                    .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                if len == 1 as libc::c_int {
                    *d = (*r).u.fill.pixel as uint8_t;
                } else {
                    memset(
                        d as *mut libc::c_void,
                        (*r).u.fill.pixel as libc::c_int,
                        len as libc::c_ulong,
                    );
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                loop {
                    let mut len_0: libc::c_int = (*spans
                        .offset(1 as libc::c_int as isize))
                        .x - (*spans.offset(0 as libc::c_int as isize)).x;
                    let mut d_0: *mut uint8_t = ((*r).u.fill.data)
                        .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                        .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                    if len_0 == 1 as libc::c_int {
                        *d_0 = (*r).u.fill.pixel as uint8_t;
                    } else {
                        memset(
                            d_0 as *mut libc::c_void,
                            (*r).u.fill.pixel as libc::c_int,
                            len_0 as libc::c_ulong,
                        );
                    }
                    yy += 1;
                    hh -= 1;
                    if !(hh != 0) {
                        break;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill16_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                let mut d: *mut uint16_t = ((*r).u.fill.data)
                    .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                    .offset(
                        ((*spans.offset(0 as libc::c_int as isize)).x * 2 as libc::c_int)
                            as isize,
                    ) as *mut uint16_t;
                loop {
                    let fresh8 = len;
                    len = len - 1;
                    if !(fresh8 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh9 = d;
                    d = d.offset(1);
                    *fresh9 = (*r).u.fill.pixel as uint16_t;
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                loop {
                    let mut len_0: libc::c_int = (*spans
                        .offset(1 as libc::c_int as isize))
                        .x - (*spans.offset(0 as libc::c_int as isize)).x;
                    let mut d_0: *mut uint16_t = ((*r).u.fill.data)
                        .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x
                                * 2 as libc::c_int) as isize,
                        ) as *mut uint16_t;
                    loop {
                        let fresh10 = len_0;
                        len_0 = len_0 - 1;
                        if !(fresh10 > 0 as libc::c_int) {
                            break;
                        }
                        let fresh11 = d_0;
                        d_0 = d_0.offset(1);
                        *fresh11 = (*r).u.fill.pixel as uint16_t;
                    }
                    yy += 1;
                    hh -= 1;
                    if !(hh != 0) {
                        break;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill32_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                if len > 32 as libc::c_int {
                    pixman_fill(
                        (*r).u.fill.data as *mut uint32_t,
                        ((*r).u.fill.stride as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ) as libc::c_int,
                        (*r).bpp,
                        (*spans.offset(0 as libc::c_int as isize)).x,
                        y,
                        len,
                        1 as libc::c_int,
                        (*r).u.fill.pixel,
                    );
                } else {
                    let mut d: *mut uint32_t = ((*r).u.fill.data)
                        .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x
                                * 4 as libc::c_int) as isize,
                        ) as *mut uint32_t;
                    loop {
                        let fresh12 = len;
                        len = len - 1;
                        if !(fresh12 > 0 as libc::c_int) {
                            break;
                        }
                        let fresh13 = d;
                        d = d.offset(1);
                        *fresh13 = (*r).u.fill.pixel;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                if (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x > 16 as libc::c_int
                {
                    pixman_fill(
                        (*r).u.fill.data as *mut uint32_t,
                        ((*r).u.fill.stride as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ) as libc::c_int,
                        (*r).bpp,
                        (*spans.offset(0 as libc::c_int as isize)).x,
                        y,
                        (*spans.offset(1 as libc::c_int as isize)).x
                            - (*spans.offset(0 as libc::c_int as isize)).x,
                        h,
                        (*r).u.fill.pixel,
                    );
                } else {
                    let mut yy: libc::c_int = y;
                    let mut hh: libc::c_int = h;
                    loop {
                        let mut len_0: libc::c_int = (*spans
                            .offset(1 as libc::c_int as isize))
                            .x - (*spans.offset(0 as libc::c_int as isize)).x;
                        let mut d_0: *mut uint32_t = ((*r).u.fill.data)
                            .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                            .offset(
                                ((*spans.offset(0 as libc::c_int as isize)).x
                                    * 4 as libc::c_int) as isize,
                            ) as *mut uint32_t;
                        loop {
                            let fresh14 = len_0;
                            len_0 = len_0 - 1;
                            if !(fresh14 > 0 as libc::c_int) {
                                break;
                            }
                            let fresh15 = d_0;
                            d_0 = d_0.offset(1);
                            *fresh15 = (*r).u.fill.pixel;
                        }
                        yy += 1;
                        hh -= 1;
                        if !(hh != 0) {
                            break;
                        }
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _blit_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut cpp: libc::c_int = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    cpp = (*r).bpp / 8 as libc::c_int;
    if h == 1 as libc::c_int {
        let mut src: *mut uint8_t = ((*r).u.blit.src_data)
            .offset((y * (*r).u.blit.src_stride) as isize);
        let mut dst: *mut uint8_t = ((*r).u.blit.data)
            .offset((y * (*r).u.blit.stride) as isize);
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut s: *mut libc::c_void = src
                    .offset(
                        ((*spans.offset(0 as libc::c_int as isize)).x * cpp) as isize,
                    ) as *mut libc::c_void;
                let mut d: *mut libc::c_void = dst
                    .offset(
                        ((*spans.offset(0 as libc::c_int as isize)).x * cpp) as isize,
                    ) as *mut libc::c_void;
                let mut len: libc::c_int = ((*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x) * cpp;
                match len {
                    1 => {
                        *(d as *mut uint8_t) = *(s as *mut uint8_t);
                    }
                    2 => {
                        *(d as *mut uint16_t) = *(s as *mut uint16_t);
                    }
                    4 => {
                        *(d as *mut uint32_t) = *(s as *mut uint32_t);
                    }
                    8 => {
                        *(d as *mut uint64_t) = *(s as *mut uint64_t);
                    }
                    _ => {
                        memcpy(d, s, len as libc::c_ulong);
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                loop {
                    let mut src_0: *mut libc::c_void = ((*r).u.blit.src_data)
                        .offset((yy * (*r).u.blit.src_stride) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x * cpp) as isize,
                        ) as *mut libc::c_void;
                    let mut dst_0: *mut libc::c_void = ((*r).u.blit.data)
                        .offset((yy * (*r).u.blit.stride) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x * cpp) as isize,
                        ) as *mut libc::c_void;
                    let mut len_0: libc::c_int = ((*spans
                        .offset(1 as libc::c_int as isize))
                        .x - (*spans.offset(0 as libc::c_int as isize)).x) * cpp;
                    match len_0 {
                        1 => {
                            *(dst_0 as *mut uint8_t) = *(src_0 as *mut uint8_t);
                        }
                        2 => {
                            *(dst_0 as *mut uint16_t) = *(src_0 as *mut uint16_t);
                        }
                        4 => {
                            *(dst_0 as *mut uint32_t) = *(src_0 as *mut uint32_t);
                        }
                        8 => {
                            *(dst_0 as *mut uint64_t) = *(src_0 as *mut uint64_t);
                        }
                        _ => {
                            memcpy(dst_0, src_0, len_0 as libc::c_ulong);
                        }
                    }
                    yy += 1;
                    hh -= 1;
                    if !(hh != 0) {
                        break;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _mono_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    loop {
        if (*spans.offset(0 as libc::c_int as isize)).coverage != 0 {
            pixman_image_composite32(
                (*r).op as pixman_op_t,
                (*r).src,
                0 as *mut pixman_image_t,
                (*r).u.composite.dst,
                (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
                y + (*r).u.composite.src_y,
                0 as libc::c_int,
                0 as libc::c_int,
                (*spans.offset(0 as libc::c_int as isize)).x,
                y,
                (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x,
                h,
            );
        }
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _mono_unbounded_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        pixman_image_composite32(
            PIXMAN_OP_CLEAR,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*(*r).composite).unbounded.x,
            y,
            (*(*r).composite).unbounded.width,
            h,
        );
        (*r).u.composite.mask_y = y + h;
        return CAIRO_STATUS_SUCCESS;
    }
    if y != (*r).u.composite.mask_y {
        pixman_image_composite32(
            PIXMAN_OP_CLEAR,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*(*r).composite).unbounded.x,
            (*r).u.composite.mask_y,
            (*(*r).composite).unbounded.width,
            y - (*r).u.composite.mask_y,
        );
    }
    if (*spans.offset(0 as libc::c_int as isize)).x != (*(*r).composite).unbounded.x {
        pixman_image_composite32(
            PIXMAN_OP_CLEAR,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*(*r).composite).unbounded.x,
            y,
            (*spans.offset(0 as libc::c_int as isize)).x - (*(*r).composite).unbounded.x,
            h,
        );
    }
    loop {
        let mut op: libc::c_int = if (*spans.offset(0 as libc::c_int as isize)).coverage
            as libc::c_int != 0
        {
            (*r).op as libc::c_int
        } else {
            PIXMAN_OP_CLEAR as libc::c_int
        };
        pixman_image_composite32(
            op as pixman_op_t,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*spans.offset(0 as libc::c_int as isize)).x,
            y,
            (*spans.offset(1 as libc::c_int as isize)).x
                - (*spans.offset(0 as libc::c_int as isize)).x,
            h,
        );
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if (*spans.offset(0 as libc::c_int as isize)).x
        != (*(*r).composite).unbounded.x + (*(*r).composite).unbounded.width
    {
        pixman_image_composite32(
            PIXMAN_OP_CLEAR,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*spans.offset(0 as libc::c_int as isize)).x,
            y,
            (*(*r).composite).unbounded.x + (*(*r).composite).unbounded.width
                - (*spans.offset(0 as libc::c_int as isize)).x,
            h,
        );
    }
    (*r).u.composite.mask_y = y + h;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _mono_finish_unbounded_spans(
    mut abstract_renderer: *mut libc::c_void,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if (*r).u.composite.mask_y
        < (*(*r).composite).unbounded.y + (*(*r).composite).unbounded.height
    {
        pixman_image_composite32(
            PIXMAN_OP_CLEAR,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*(*r).composite).unbounded.x + (*r).u.composite.src_x,
            (*r).u.composite.mask_y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*(*r).composite).unbounded.x,
            (*r).u.composite.mask_y,
            (*(*r).composite).unbounded.width,
            (*(*r).composite).unbounded.y + (*(*r).composite).unbounded.height
                - (*r).u.composite.mask_y,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn mono_renderer_init(
    mut r: *mut cairo_image_span_renderer_t,
    mut composite_0: *const cairo_composite_rectangles_t,
    mut antialias: cairo_antialias_t,
    mut needs_clip: cairo_bool_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = (*composite_0).surface
        as *mut cairo_image_surface_t;
    if antialias as libc::c_uint != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if _cairo_pattern_is_opaque_solid(&(*composite_0).mask_pattern.base) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    let ref mut fresh16 = (*r).base.render_rows;
    *fresh16 = None;
    if (*composite_0).source_pattern.base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        let mut color: *const cairo_color_t = 0 as *const cairo_color_t;
        color = &(*composite_0).source_pattern.solid.color;
        if (*composite_0).op as libc::c_uint
            == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        {
            color = _cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
        }
        if fill_reduces_to_source((*composite_0).op, color, dst, &mut (*r).u.fill.pixel)
            != 0
        {
            match ((*dst).pixman_format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << ((*dst).pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint)
            {
                8 => {
                    let ref mut fresh17 = (*r).base.render_rows;
                    *fresh17 = Some(
                        _fill8_spans
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_int,
                                libc::c_int,
                                *const cairo_half_open_span_t,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    );
                }
                16 => {
                    let ref mut fresh18 = (*r).base.render_rows;
                    *fresh18 = Some(
                        _fill16_spans
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_int,
                                libc::c_int,
                                *const cairo_half_open_span_t,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    );
                }
                32 => {
                    let ref mut fresh19 = (*r).base.render_rows;
                    *fresh19 = Some(
                        _fill32_spans
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_int,
                                libc::c_int,
                                *const cairo_half_open_span_t,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    );
                }
                _ => {}
            }
            let ref mut fresh20 = (*r).u.fill.data;
            *fresh20 = (*dst).data;
            (*r).u.fill.stride = (*dst).stride;
        }
    } else if ((*composite_0).op as libc::c_uint
        == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        || (*composite_0).op as libc::c_uint
            == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            && (((*dst).base).is_clear() as libc::c_int != 0
                || (*dst).base.content as libc::c_uint
                    & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint))
        && (*composite_0).source_pattern.base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*(*(*composite_0).source_pattern.surface.surface).backend).type_0
            as libc::c_uint == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        && (*((*composite_0).source_pattern.surface.surface
            as *mut cairo_image_surface_t))
            .format as libc::c_int == (*dst).format as libc::c_int
    {
        let mut src: *mut cairo_image_surface_t = (*composite_0)
            .source_pattern
            .surface
            .surface as *mut cairo_image_surface_t;
        let mut tx: libc::c_int = 0;
        let mut ty: libc::c_int = 0;
        if _cairo_matrix_is_integer_translation(
            &(*composite_0).source_pattern.base.matrix,
            &mut tx,
            &mut ty,
        ) != 0 && (*composite_0).bounded.x + tx >= 0 as libc::c_int
            && (*composite_0).bounded.y + ty >= 0 as libc::c_int
            && (*composite_0).bounded.x + (*composite_0).bounded.width + tx
                <= (*src).width
            && (*composite_0).bounded.y + (*composite_0).bounded.height + ty
                <= (*src).height
        {
            (*r).u.blit.stride = (*dst).stride as libc::c_int;
            let ref mut fresh21 = (*r).u.blit.data;
            *fresh21 = (*dst).data;
            (*r).u.blit.src_stride = (*src).stride as libc::c_int;
            let ref mut fresh22 = (*r).u.blit.src_data;
            *fresh22 = ((*src).data)
                .offset(((*src).stride * ty as libc::c_long) as isize)
                .offset((tx * 4 as libc::c_int) as isize);
            let ref mut fresh23 = (*r).base.render_rows;
            *fresh23 = Some(
                _blit_spans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_half_open_span_t,
                        libc::c_uint,
                    ) -> cairo_status_t,
            );
        }
    }
    if ((*r).base.render_rows).is_none() {
        let ref mut fresh24 = (*r).src;
        *fresh24 = _pixman_image_for_pattern(
            dst,
            &(*composite_0).source_pattern.base,
            0 as libc::c_int,
            &(*composite_0).unbounded,
            &(*composite_0).source_sample_area,
            &mut (*r).u.composite.src_x,
            &mut (*r).u.composite.src_y,
        );
        if ((*r).src).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        let ref mut fresh25 = (*r).u.composite.dst;
        *fresh25 = to_pixman_image((*composite_0).surface);
        (*r).op = _pixman_operator((*composite_0).op) as uint8_t;
        if (*composite_0).is_bounded == 0 as libc::c_int as libc::c_uint {
            let ref mut fresh26 = (*r).base.render_rows;
            *fresh26 = Some(
                _mono_unbounded_spans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_half_open_span_t,
                        libc::c_uint,
                    ) -> cairo_status_t,
            );
            let ref mut fresh27 = (*r).base.finish;
            *fresh27 = Some(
                _mono_finish_unbounded_spans
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            );
            (*r).u.composite.mask_y = (*composite_0).unbounded.y;
        } else {
            let ref mut fresh28 = (*r).base.render_rows;
            *fresh28 = Some(
                _mono_spans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_half_open_span_t,
                        libc::c_uint,
                    ) -> cairo_status_t,
            );
        }
    }
    (*r)
        .bpp = (((*dst).pixman_format as libc::c_uint >> 24 as libc::c_int
        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << ((*dst).pixman_format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    return CAIRO_INT_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn mul8x2_8(mut a: uint32_t, mut b: uint8_t) -> uint32_t {
    let mut t: uint32_t = (a & 0xff00ff as libc::c_int as libc::c_uint)
        .wrapping_mul(b as libc::c_uint)
        .wrapping_add(0x7f007f as libc::c_int as libc::c_uint);
    return t
        .wrapping_add(t >> 8 as libc::c_int & 0xff00ff as libc::c_int as libc::c_uint)
        >> 8 as libc::c_int & 0xff00ff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn add8x2_8x2(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut t: uint32_t = a.wrapping_add(b);
    t
        |= (0x1000100 as libc::c_int as libc::c_uint)
            .wrapping_sub(
                t >> 8 as libc::c_int & 0xff00ff as libc::c_int as libc::c_uint,
            );
    return t & 0xff00ff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn mul8_8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut t: uint16_t = (a as libc::c_int * b as uint16_t as libc::c_int
        + 0x7f as libc::c_int) as uint16_t;
    return ((t as libc::c_int >> 8 as libc::c_int) + t as libc::c_int
        >> 8 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn lerp8x4(
    mut src: uint32_t,
    mut a: uint8_t,
    mut dst: uint32_t,
) -> uint32_t {
    return add8x2_8x2(mul8x2_8(src, a), mul8x2_8(dst, !(a as libc::c_int) as uint8_t))
        | add8x2_8x2(
            mul8x2_8(src >> 8 as libc::c_int, a),
            mul8x2_8(dst >> 8 as libc::c_int, !(a as libc::c_int) as uint8_t),
        ) << 8 as libc::c_int;
}
unsafe extern "C" fn _fill_a8_lerp_opaque_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        let mut d: *mut uint8_t = ((*r).u.fill.data)
            .offset(((*r).u.fill.stride * y as libc::c_long) as isize);
        loop {
            let mut a: uint8_t = (*spans.offset(0 as libc::c_int as isize)).coverage;
            if a != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                if a as libc::c_int == 0xff as libc::c_int {
                    memset(
                        d.offset((*spans.offset(0 as libc::c_int as isize)).x as isize)
                            as *mut libc::c_void,
                        (*r).u.fill.pixel as libc::c_int,
                        len as libc::c_ulong,
                    );
                } else {
                    let mut s: uint8_t = mul8_8(a, (*r).u.fill.pixel as uint8_t);
                    let mut dst: *mut uint8_t = d
                        .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                    a = !(a as libc::c_int) as uint8_t;
                    loop {
                        let fresh29 = len;
                        len = len - 1;
                        if !(fresh29 > 0 as libc::c_int) {
                            break;
                        }
                        let mut t: uint8_t = mul8_8(*dst, a);
                        let fresh30 = dst;
                        dst = dst.offset(1);
                        *fresh30 = (t as libc::c_int + s as libc::c_int) as uint8_t;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            let mut a_0: uint8_t = (*spans.offset(0 as libc::c_int as isize)).coverage;
            if a_0 != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                if a_0 as libc::c_int == 0xff as libc::c_int {
                    loop {
                        let mut len_0: libc::c_int = (*spans
                            .offset(1 as libc::c_int as isize))
                            .x - (*spans.offset(0 as libc::c_int as isize)).x;
                        let mut d_0: *mut uint8_t = ((*r).u.fill.data)
                            .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                            .offset(
                                (*spans.offset(0 as libc::c_int as isize)).x as isize,
                            );
                        memset(
                            d_0 as *mut libc::c_void,
                            (*r).u.fill.pixel as libc::c_int,
                            len_0 as libc::c_ulong,
                        );
                        yy += 1;
                        hh -= 1;
                        if !(hh != 0) {
                            break;
                        }
                    }
                } else {
                    let mut s_0: uint8_t = mul8_8(a_0, (*r).u.fill.pixel as uint8_t);
                    a_0 = !(a_0 as libc::c_int) as uint8_t;
                    loop {
                        let mut len_1: libc::c_int = (*spans
                            .offset(1 as libc::c_int as isize))
                            .x - (*spans.offset(0 as libc::c_int as isize)).x;
                        let mut d_1: *mut uint8_t = ((*r).u.fill.data)
                            .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                            .offset(
                                (*spans.offset(0 as libc::c_int as isize)).x as isize,
                            );
                        loop {
                            let fresh31 = len_1;
                            len_1 = len_1 - 1;
                            if !(fresh31 > 0 as libc::c_int) {
                                break;
                            }
                            let mut t_0: uint8_t = mul8_8(*d_1, a_0);
                            let fresh32 = d_1;
                            d_1 = d_1.offset(1);
                            *fresh32 = (t_0 as libc::c_int + s_0 as libc::c_int)
                                as uint8_t;
                        }
                        yy += 1;
                        hh -= 1;
                        if !(hh != 0) {
                            break;
                        }
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill_xrgb32_lerp_opaque_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        loop {
            let mut a: uint8_t = (*spans.offset(0 as libc::c_int as isize)).coverage;
            if a != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                let mut d: *mut uint32_t = ((*r).u.fill.data)
                    .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                    .offset(
                        ((*spans.offset(0 as libc::c_int as isize)).x * 4 as libc::c_int)
                            as isize,
                    ) as *mut uint32_t;
                if a as libc::c_int == 0xff as libc::c_int {
                    if len > 31 as libc::c_int {
                        pixman_fill(
                            (*r).u.fill.data as *mut uint32_t,
                            ((*r).u.fill.stride as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                                ) as libc::c_int,
                            32 as libc::c_int,
                            (*spans.offset(0 as libc::c_int as isize)).x,
                            y,
                            len,
                            1 as libc::c_int,
                            (*r).u.fill.pixel,
                        );
                    } else {
                        let mut d_0: *mut uint32_t = ((*r).u.fill.data)
                            .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                            .offset(
                                ((*spans.offset(0 as libc::c_int as isize)).x
                                    * 4 as libc::c_int) as isize,
                            ) as *mut uint32_t;
                        loop {
                            let fresh33 = len;
                            len = len - 1;
                            if !(fresh33 > 0 as libc::c_int) {
                                break;
                            }
                            let fresh34 = d_0;
                            d_0 = d_0.offset(1);
                            *fresh34 = (*r).u.fill.pixel;
                        }
                    }
                } else {
                    loop {
                        let fresh35 = len;
                        len = len - 1;
                        if !(fresh35 > 0 as libc::c_int) {
                            break;
                        }
                        *d = lerp8x4((*r).u.fill.pixel, a, *d);
                        d = d.offset(1);
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            let mut a_0: uint8_t = (*spans.offset(0 as libc::c_int as isize)).coverage;
            if a_0 != 0 {
                if a_0 as libc::c_int == 0xff as libc::c_int {
                    if (*spans.offset(1 as libc::c_int as isize)).x
                        - (*spans.offset(0 as libc::c_int as isize)).x
                        > 16 as libc::c_int
                    {
                        pixman_fill(
                            (*r).u.fill.data as *mut uint32_t,
                            ((*r).u.fill.stride as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                                ) as libc::c_int,
                            32 as libc::c_int,
                            (*spans.offset(0 as libc::c_int as isize)).x,
                            y,
                            (*spans.offset(1 as libc::c_int as isize)).x
                                - (*spans.offset(0 as libc::c_int as isize)).x,
                            h,
                            (*r).u.fill.pixel,
                        );
                    } else {
                        let mut yy: libc::c_int = y;
                        let mut hh: libc::c_int = h;
                        loop {
                            let mut len_0: libc::c_int = (*spans
                                .offset(1 as libc::c_int as isize))
                                .x - (*spans.offset(0 as libc::c_int as isize)).x;
                            let mut d_1: *mut uint32_t = ((*r).u.fill.data)
                                .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                                .offset(
                                    ((*spans.offset(0 as libc::c_int as isize)).x
                                        * 4 as libc::c_int) as isize,
                                ) as *mut uint32_t;
                            loop {
                                let fresh36 = len_0;
                                len_0 = len_0 - 1;
                                if !(fresh36 > 0 as libc::c_int) {
                                    break;
                                }
                                let fresh37 = d_1;
                                d_1 = d_1.offset(1);
                                *fresh37 = (*r).u.fill.pixel;
                            }
                            yy += 1;
                            hh -= 1;
                            if !(hh != 0) {
                                break;
                            }
                        }
                    }
                } else {
                    let mut yy_0: libc::c_int = y;
                    let mut hh_0: libc::c_int = h;
                    loop {
                        let mut len_1: libc::c_int = (*spans
                            .offset(1 as libc::c_int as isize))
                            .x - (*spans.offset(0 as libc::c_int as isize)).x;
                        let mut d_2: *mut uint32_t = ((*r).u.fill.data)
                            .offset(((*r).u.fill.stride * yy_0 as libc::c_long) as isize)
                            .offset(
                                ((*spans.offset(0 as libc::c_int as isize)).x
                                    * 4 as libc::c_int) as isize,
                            ) as *mut uint32_t;
                        loop {
                            let fresh38 = len_1;
                            len_1 = len_1 - 1;
                            if !(fresh38 > 0 as libc::c_int) {
                                break;
                            }
                            *d_2 = lerp8x4((*r).u.fill.pixel, a_0, *d_2);
                            d_2 = d_2.offset(1);
                        }
                        yy_0 += 1;
                        hh_0 -= 1;
                        if !(hh_0 != 0) {
                            break;
                        }
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill_a8_lerp_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        loop {
            let mut a: uint8_t = mul8_8(
                (*spans.offset(0 as libc::c_int as isize)).coverage,
                (*r).bpp as uint8_t,
            );
            if a != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                let mut d: *mut uint8_t = ((*r).u.fill.data)
                    .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                    .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                let mut p: uint16_t = (a as uint16_t as libc::c_uint)
                    .wrapping_mul((*r).u.fill.pixel)
                    .wrapping_add(0x7f as libc::c_int as libc::c_uint) as uint16_t;
                let mut ia: uint16_t = !(a as libc::c_int) as uint16_t;
                loop {
                    let fresh39 = len;
                    len = len - 1;
                    if !(fresh39 > 0 as libc::c_int) {
                        break;
                    }
                    let mut t: uint16_t = (*d as libc::c_int * ia as libc::c_int
                        + p as libc::c_int) as uint16_t;
                    let fresh40 = d;
                    d = d.offset(1);
                    *fresh40 = (t as libc::c_int + (t as libc::c_int >> 8 as libc::c_int)
                        >> 8 as libc::c_int) as uint8_t;
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            let mut a_0: uint8_t = mul8_8(
                (*spans.offset(0 as libc::c_int as isize)).coverage,
                (*r).bpp as uint8_t,
            );
            if a_0 != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                let mut p_0: uint16_t = (a_0 as uint16_t as libc::c_uint)
                    .wrapping_mul((*r).u.fill.pixel)
                    .wrapping_add(0x7f as libc::c_int as libc::c_uint) as uint16_t;
                let mut ia_0: uint16_t = !(a_0 as libc::c_int) as uint16_t;
                loop {
                    let mut len_0: libc::c_int = (*spans
                        .offset(1 as libc::c_int as isize))
                        .x - (*spans.offset(0 as libc::c_int as isize)).x;
                    let mut d_0: *mut uint8_t = ((*r).u.fill.data)
                        .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                        .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                    loop {
                        let fresh41 = len_0;
                        len_0 = len_0 - 1;
                        if !(fresh41 > 0 as libc::c_int) {
                            break;
                        }
                        let mut t_0: uint16_t = (*d_0 as libc::c_int
                            * ia_0 as libc::c_int + p_0 as libc::c_int) as uint16_t;
                        let fresh42 = d_0;
                        d_0 = d_0.offset(1);
                        *fresh42 = (t_0 as libc::c_int
                            + (t_0 as libc::c_int >> 8 as libc::c_int)
                            >> 8 as libc::c_int) as uint8_t;
                    }
                    yy += 1;
                    hh -= 1;
                    if !(hh != 0) {
                        break;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _fill_xrgb32_lerp_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        loop {
            let mut a: uint8_t = mul8_8(
                (*spans.offset(0 as libc::c_int as isize)).coverage,
                (*r).bpp as uint8_t,
            );
            if a != 0 {
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                let mut d: *mut uint32_t = ((*r).u.fill.data)
                    .offset(((*r).u.fill.stride * y as libc::c_long) as isize)
                    .offset(
                        ((*spans.offset(0 as libc::c_int as isize)).x * 4 as libc::c_int)
                            as isize,
                    ) as *mut uint32_t;
                loop {
                    let fresh43 = len;
                    len = len - 1;
                    if !(fresh43 > 0 as libc::c_int) {
                        break;
                    }
                    *d = lerp8x4((*r).u.fill.pixel, a, *d);
                    d = d.offset(1);
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            let mut a_0: uint8_t = mul8_8(
                (*spans.offset(0 as libc::c_int as isize)).coverage,
                (*r).bpp as uint8_t,
            );
            if a_0 != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                loop {
                    let mut len_0: libc::c_int = (*spans
                        .offset(1 as libc::c_int as isize))
                        .x - (*spans.offset(0 as libc::c_int as isize)).x;
                    let mut d_0: *mut uint32_t = ((*r).u.fill.data)
                        .offset(((*r).u.fill.stride * yy as libc::c_long) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x
                                * 4 as libc::c_int) as isize,
                        ) as *mut uint32_t;
                    loop {
                        let fresh44 = len_0;
                        len_0 = len_0 - 1;
                        if !(fresh44 > 0 as libc::c_int) {
                            break;
                        }
                        *d_0 = lerp8x4((*r).u.fill.pixel, a_0, *d_0);
                        d_0 = d_0.offset(1);
                    }
                    yy += 1;
                    hh -= 1;
                    if !(hh != 0) {
                        break;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _blit_xrgb32_lerp_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if h == 1 as libc::c_int {
        let mut src: *mut uint8_t = ((*r).u.blit.src_data)
            .offset((y * (*r).u.blit.src_stride) as isize);
        let mut dst: *mut uint8_t = ((*r).u.blit.data)
            .offset((y * (*r).u.blit.stride) as isize);
        loop {
            let mut a: uint8_t = mul8_8(
                (*spans.offset(0 as libc::c_int as isize)).coverage,
                (*r).bpp as uint8_t,
            );
            if a != 0 {
                let mut s: *mut uint32_t = (src as *mut uint32_t)
                    .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                let mut d: *mut uint32_t = (dst as *mut uint32_t)
                    .offset((*spans.offset(0 as libc::c_int as isize)).x as isize);
                let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x;
                if a as libc::c_int == 0xff as libc::c_int {
                    if len == 1 as libc::c_int {
                        *d = *s;
                    } else {
                        memcpy(
                            d as *mut libc::c_void,
                            s as *const libc::c_void,
                            (len * 4 as libc::c_int) as libc::c_ulong,
                        );
                    }
                } else {
                    loop {
                        let fresh45 = len;
                        len = len - 1;
                        if !(fresh45 > 0 as libc::c_int) {
                            break;
                        }
                        *d = lerp8x4(*s, a, *d);
                        s = s.offset(1);
                        d = d.offset(1);
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    } else {
        loop {
            let mut a_0: uint8_t = mul8_8(
                (*spans.offset(0 as libc::c_int as isize)).coverage,
                (*r).bpp as uint8_t,
            );
            if a_0 != 0 {
                let mut yy: libc::c_int = y;
                let mut hh: libc::c_int = h;
                loop {
                    let mut s_0: *mut uint32_t = ((*r).u.blit.src_data)
                        .offset((yy * (*r).u.blit.src_stride) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x
                                * 4 as libc::c_int) as isize,
                        ) as *mut uint32_t;
                    let mut d_0: *mut uint32_t = ((*r).u.blit.data)
                        .offset((yy * (*r).u.blit.stride) as isize)
                        .offset(
                            ((*spans.offset(0 as libc::c_int as isize)).x
                                * 4 as libc::c_int) as isize,
                        ) as *mut uint32_t;
                    let mut len_0: libc::c_int = (*spans
                        .offset(1 as libc::c_int as isize))
                        .x - (*spans.offset(0 as libc::c_int as isize)).x;
                    if a_0 as libc::c_int == 0xff as libc::c_int {
                        if len_0 == 1 as libc::c_int {
                            *d_0 = *s_0;
                        } else {
                            memcpy(
                                d_0 as *mut libc::c_void,
                                s_0 as *const libc::c_void,
                                (len_0 * 4 as libc::c_int) as libc::c_ulong,
                            );
                        }
                    } else {
                        loop {
                            let fresh46 = len_0;
                            len_0 = len_0 - 1;
                            if !(fresh46 > 0 as libc::c_int) {
                                break;
                            }
                            *d_0 = lerp8x4(*s_0, a_0, *d_0);
                            s_0 = s_0.offset(1);
                            d_0 = d_0.offset(1);
                        }
                    }
                    yy += 1;
                    hh -= 1;
                    if !(hh != 0) {
                        break;
                    }
                }
            }
            spans = spans.offset(1);
            num_spans = num_spans.wrapping_sub(1);
            if !(num_spans > 1 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _inplace_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut mask: *mut uint8_t = 0 as *mut uint8_t;
    let mut x0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    if num_spans == 2 as libc::c_int as libc::c_uint
        && (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
            == 0xff as libc::c_int
    {
        pixman_image_composite32(
            (*r).op as pixman_op_t,
            (*r).src,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*spans.offset(0 as libc::c_int as isize)).x,
            y,
            (*spans.offset(1 as libc::c_int as isize)).x
                - (*spans.offset(0 as libc::c_int as isize)).x,
            h,
        );
        return CAIRO_STATUS_SUCCESS;
    }
    mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
    x0 = (*spans.offset(0 as libc::c_int as isize)).x;
    x1 = x0;
    loop {
        let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
            - (*spans.offset(0 as libc::c_int as isize)).x;
        let fresh47 = mask;
        mask = mask.offset(1);
        *fresh47 = (*spans.offset(0 as libc::c_int as isize)).coverage;
        if len > 1 as libc::c_int {
            if len >= (*r).u.composite.run_length
                && (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
                    == 0xff as libc::c_int
            {
                if x1 != x0 {
                    pixman_image_composite32(
                        (*r).op as pixman_op_t,
                        (*r).src,
                        (*r).mask,
                        (*r).u.composite.dst,
                        x0 + (*r).u.composite.src_x,
                        y + (*r).u.composite.src_y,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        x0,
                        y,
                        x1 - x0,
                        h,
                    );
                }
                pixman_image_composite32(
                    (*r).op as pixman_op_t,
                    (*r).src,
                    0 as *mut pixman_image_t,
                    (*r).u.composite.dst,
                    (*spans.offset(0 as libc::c_int as isize)).x
                        + (*r).u.composite.src_x,
                    y + (*r).u.composite.src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*spans.offset(0 as libc::c_int as isize)).x,
                    y,
                    len,
                    h,
                );
                mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
                x0 = (*spans.offset(1 as libc::c_int as isize)).x;
            } else if (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
                == 0 as libc::c_int && x1 - x0 > (*r).u.composite.run_length
            {
                pixman_image_composite32(
                    (*r).op as pixman_op_t,
                    (*r).src,
                    (*r).mask,
                    (*r).u.composite.dst,
                    x0 + (*r).u.composite.src_x,
                    y + (*r).u.composite.src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    x1 - x0,
                    h,
                );
                mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
                x0 = (*spans.offset(1 as libc::c_int as isize)).x;
            } else {
                len -= 1;
                memset(
                    mask as *mut libc::c_void,
                    (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int,
                    len as libc::c_ulong,
                );
                mask = mask.offset(len as isize);
            }
        }
        x1 = (*spans.offset(1 as libc::c_int as isize)).x;
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if x1 != x0 {
        pixman_image_composite32(
            (*r).op as pixman_op_t,
            (*r).src,
            (*r).mask,
            (*r).u.composite.dst,
            x0 + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x0,
            y,
            x1 - x0,
            h,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _inplace_opacity_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut mask: *mut uint8_t = 0 as *mut uint8_t;
    let mut x0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
    x0 = (*spans.offset(0 as libc::c_int as isize)).x;
    x1 = x0;
    loop {
        let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
            - (*spans.offset(0 as libc::c_int as isize)).x;
        let mut m: uint8_t = mul8_8(
            (*spans.offset(0 as libc::c_int as isize)).coverage,
            (*r).bpp as uint8_t,
        );
        let fresh48 = mask;
        mask = mask.offset(1);
        *fresh48 = m;
        if len > 1 as libc::c_int {
            if m as libc::c_int == 0 as libc::c_int
                && x1 - x0 > (*r).u.composite.run_length
            {
                pixman_image_composite32(
                    (*r).op as pixman_op_t,
                    (*r).src,
                    (*r).mask,
                    (*r).u.composite.dst,
                    x0 + (*r).u.composite.src_x,
                    y + (*r).u.composite.src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    x1 - x0,
                    h,
                );
                mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
                x0 = (*spans.offset(1 as libc::c_int as isize)).x;
            } else {
                len -= 1;
                memset(
                    mask as *mut libc::c_void,
                    m as libc::c_int,
                    len as libc::c_ulong,
                );
                mask = mask.offset(len as isize);
            }
        }
        x1 = (*spans.offset(1 as libc::c_int as isize)).x;
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if x1 != x0 {
        pixman_image_composite32(
            (*r).op as pixman_op_t,
            (*r).src,
            (*r).mask,
            (*r).u.composite.dst,
            x0 + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x0,
            y,
            x1 - x0,
            h,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _inplace_src_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut m: *mut uint8_t = 0 as *mut uint8_t;
    let mut base: *mut uint8_t = pixman_image_get_data((*r).mask) as *mut uint8_t;
    let mut x0: libc::c_int = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    x0 = (*spans.offset(0 as libc::c_int as isize)).x;
    m = base;
    loop {
        let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
            - (*spans.offset(0 as libc::c_int as isize)).x;
        if len >= (*r).u.composite.run_length
            && (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
                == 0xff as libc::c_int
        {
            if (*spans.offset(0 as libc::c_int as isize)).x != x0 {
                pixman_image_composite32(
                    PIXMAN_OP_OUT_REVERSE,
                    (*r).mask,
                    0 as *mut pixman_image_t,
                    (*r).u.composite.dst,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    (*spans.offset(0 as libc::c_int as isize)).x - x0,
                    h,
                );
                pixman_image_composite32(
                    PIXMAN_OP_ADD,
                    (*r).src,
                    (*r).mask,
                    (*r).u.composite.dst,
                    x0 + (*r).u.composite.src_x,
                    y + (*r).u.composite.src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    (*spans.offset(0 as libc::c_int as isize)).x - x0,
                    h,
                );
            }
            pixman_image_composite32(
                PIXMAN_OP_SRC,
                (*r).src,
                0 as *mut pixman_image_t,
                (*r).u.composite.dst,
                (*spans.offset(0 as libc::c_int as isize)).x + (*r).u.composite.src_x,
                y + (*r).u.composite.src_y,
                0 as libc::c_int,
                0 as libc::c_int,
                (*spans.offset(0 as libc::c_int as isize)).x,
                y,
                (*spans.offset(1 as libc::c_int as isize)).x
                    - (*spans.offset(0 as libc::c_int as isize)).x,
                h,
            );
            m = base;
            x0 = (*spans.offset(1 as libc::c_int as isize)).x;
        } else if (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int
            == 0 as libc::c_int
        {
            if (*spans.offset(0 as libc::c_int as isize)).x != x0 {
                pixman_image_composite32(
                    PIXMAN_OP_OUT_REVERSE,
                    (*r).mask,
                    0 as *mut pixman_image_t,
                    (*r).u.composite.dst,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    (*spans.offset(0 as libc::c_int as isize)).x - x0,
                    h,
                );
                pixman_image_composite32(
                    PIXMAN_OP_ADD,
                    (*r).src,
                    (*r).mask,
                    (*r).u.composite.dst,
                    x0 + (*r).u.composite.src_x,
                    y + (*r).u.composite.src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    (*spans.offset(0 as libc::c_int as isize)).x - x0,
                    h,
                );
            }
            m = base;
            x0 = (*spans.offset(1 as libc::c_int as isize)).x;
        } else {
            let fresh49 = m;
            m = m.offset(1);
            *fresh49 = (*spans.offset(0 as libc::c_int as isize)).coverage;
            if len > 1 as libc::c_int {
                len -= 1;
                memset(
                    m as *mut libc::c_void,
                    (*spans.offset(0 as libc::c_int as isize)).coverage as libc::c_int,
                    len as libc::c_ulong,
                );
                m = m.offset(len as isize);
            }
        }
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if (*spans.offset(0 as libc::c_int as isize)).x != x0 {
        pixman_image_composite32(
            PIXMAN_OP_OUT_REVERSE,
            (*r).mask,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            x0,
            y,
            (*spans.offset(0 as libc::c_int as isize)).x - x0,
            h,
        );
        pixman_image_composite32(
            PIXMAN_OP_ADD,
            (*r).src,
            (*r).mask,
            (*r).u.composite.dst,
            x0 + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x0,
            y,
            (*spans.offset(0 as libc::c_int as isize)).x - x0,
            h,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _inplace_src_opacity_spans(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut spans: *const cairo_half_open_span_t,
    mut num_spans: libc::c_uint,
) -> cairo_status_t {
    let mut r: *mut cairo_image_span_renderer_t = abstract_renderer
        as *mut cairo_image_span_renderer_t;
    let mut mask: *mut uint8_t = 0 as *mut uint8_t;
    let mut x0: libc::c_int = 0;
    if num_spans == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    x0 = (*spans.offset(0 as libc::c_int as isize)).x;
    mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
    loop {
        let mut len: libc::c_int = (*spans.offset(1 as libc::c_int as isize)).x
            - (*spans.offset(0 as libc::c_int as isize)).x;
        let mut m: uint8_t = mul8_8(
            (*spans.offset(0 as libc::c_int as isize)).coverage,
            (*r).bpp as uint8_t,
        );
        if m as libc::c_int == 0 as libc::c_int {
            if (*spans.offset(0 as libc::c_int as isize)).x != x0 {
                pixman_image_composite32(
                    PIXMAN_OP_OUT_REVERSE,
                    (*r).mask,
                    0 as *mut pixman_image_t,
                    (*r).u.composite.dst,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    (*spans.offset(0 as libc::c_int as isize)).x - x0,
                    h,
                );
                pixman_image_composite32(
                    PIXMAN_OP_ADD,
                    (*r).src,
                    (*r).mask,
                    (*r).u.composite.dst,
                    x0 + (*r).u.composite.src_x,
                    y + (*r).u.composite.src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x0,
                    y,
                    (*spans.offset(0 as libc::c_int as isize)).x - x0,
                    h,
                );
            }
            mask = pixman_image_get_data((*r).mask) as *mut uint8_t;
            x0 = (*spans.offset(1 as libc::c_int as isize)).x;
        } else {
            let fresh50 = mask;
            mask = mask.offset(1);
            *fresh50 = m;
            if len > 1 as libc::c_int {
                len -= 1;
                memset(
                    mask as *mut libc::c_void,
                    m as libc::c_int,
                    len as libc::c_ulong,
                );
                mask = mask.offset(len as isize);
            }
        }
        spans = spans.offset(1);
        num_spans = num_spans.wrapping_sub(1);
        if !(num_spans > 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if (*spans.offset(0 as libc::c_int as isize)).x != x0 {
        pixman_image_composite32(
            PIXMAN_OP_OUT_REVERSE,
            (*r).mask,
            0 as *mut pixman_image_t,
            (*r).u.composite.dst,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            x0,
            y,
            (*spans.offset(0 as libc::c_int as isize)).x - x0,
            h,
        );
        pixman_image_composite32(
            PIXMAN_OP_ADD,
            (*r).src,
            (*r).mask,
            (*r).u.composite.dst,
            x0 + (*r).u.composite.src_x,
            y + (*r).u.composite.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            x0,
            y,
            (*spans.offset(0 as libc::c_int as isize)).x - x0,
            h,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn free_pixels(
    mut image: *mut pixman_image_t,
    mut data: *mut libc::c_void,
) {
    free(data);
}
unsafe extern "C" fn inplace_renderer_init(
    mut r: *mut cairo_image_span_renderer_t,
    mut composite_0: *const cairo_composite_rectangles_t,
    mut antialias: cairo_antialias_t,
    mut needs_clip: cairo_bool_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_image_surface_t = (*composite_0).surface
        as *mut cairo_image_surface_t;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    if (*composite_0).mask_pattern.base.type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    let ref mut fresh51 = (*r).base.render_rows;
    *fresh51 = None;
    (*r)
        .bpp = (*composite_0).mask_pattern.solid.color.alpha_short as libc::c_int
        >> 8 as libc::c_int;
    if (*composite_0).source_pattern.base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        let mut color: *const cairo_color_t = 0 as *const cairo_color_t;
        color = &(*composite_0).source_pattern.solid.color;
        if (*composite_0).op as libc::c_uint
            == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        {
            color = _cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
        }
        if fill_reduces_to_source((*composite_0).op, color, dst, &mut (*r).u.fill.pixel)
            != 0
        {
            if (*r).bpp == 0xff as libc::c_int {
                match (*dst).format as libc::c_int {
                    2 => {
                        let ref mut fresh52 = (*r).base.render_rows;
                        *fresh52 = Some(
                            _fill_a8_lerp_opaque_spans
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    libc::c_int,
                                    libc::c_int,
                                    *const cairo_half_open_span_t,
                                    libc::c_uint,
                                ) -> cairo_status_t,
                        );
                    }
                    1 | 0 => {
                        let ref mut fresh53 = (*r).base.render_rows;
                        *fresh53 = Some(
                            _fill_xrgb32_lerp_opaque_spans
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    libc::c_int,
                                    libc::c_int,
                                    *const cairo_half_open_span_t,
                                    libc::c_uint,
                                ) -> cairo_status_t,
                        );
                    }
                    3 | 4 | 5 | 6 | 7 | -1 | _ => {}
                }
            } else {
                match (*dst).format as libc::c_int {
                    2 => {
                        let ref mut fresh54 = (*r).base.render_rows;
                        *fresh54 = Some(
                            _fill_a8_lerp_spans
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    libc::c_int,
                                    libc::c_int,
                                    *const cairo_half_open_span_t,
                                    libc::c_uint,
                                ) -> cairo_status_t,
                        );
                    }
                    1 | 0 => {
                        let ref mut fresh55 = (*r).base.render_rows;
                        *fresh55 = Some(
                            _fill_xrgb32_lerp_spans
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    libc::c_int,
                                    libc::c_int,
                                    *const cairo_half_open_span_t,
                                    libc::c_uint,
                                ) -> cairo_status_t,
                        );
                    }
                    3 | 4 | 5 | 6 | 7 | -1 | _ => {}
                }
            }
            let ref mut fresh56 = (*r).u.fill.data;
            *fresh56 = (*dst).data;
            (*r).u.fill.stride = (*dst).stride;
        }
    } else if ((*dst).format as libc::c_int == CAIRO_FORMAT_ARGB32 as libc::c_int
        || (*dst).format as libc::c_int == CAIRO_FORMAT_RGB24 as libc::c_int)
        && ((*composite_0).op as libc::c_uint
            == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || (*composite_0).op as libc::c_uint
                == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                && (((*dst).base).is_clear() as libc::c_int != 0
                    || (*dst).base.content as libc::c_uint
                        & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint))
        && (*composite_0).source_pattern.base.type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*(*(*composite_0).source_pattern.surface.surface).backend).type_0
            as libc::c_uint == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        && (*((*composite_0).source_pattern.surface.surface
            as *mut cairo_image_surface_t))
            .format as libc::c_int == (*dst).format as libc::c_int
    {
        let mut src: *mut cairo_image_surface_t = (*composite_0)
            .source_pattern
            .surface
            .surface as *mut cairo_image_surface_t;
        let mut tx: libc::c_int = 0;
        let mut ty: libc::c_int = 0;
        if _cairo_matrix_is_integer_translation(
            &(*composite_0).source_pattern.base.matrix,
            &mut tx,
            &mut ty,
        ) != 0 && (*composite_0).bounded.x + tx >= 0 as libc::c_int
            && (*composite_0).bounded.y + ty >= 0 as libc::c_int
            && (*composite_0).bounded.x + (*composite_0).bounded.width + tx
                <= (*src).width
            && (*composite_0).bounded.y + (*composite_0).bounded.height + ty
                <= (*src).height
        {
            if ((*dst).pixman_format as libc::c_uint >> 24 as libc::c_int
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint)
                << ((*dst).pixman_format as libc::c_uint >> 22 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint)
                == 32 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"PIXMAN_FORMAT_BPP(dst->pixman_format) == 32\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-image-compositor.c\0" as *const u8
                        as *const libc::c_char,
                    2907 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 143],
                        &[libc::c_char; 143],
                    >(
                        b"cairo_int_status_t inplace_renderer_init(cairo_image_span_renderer_t *, const cairo_composite_rectangles_t *, cairo_antialias_t, cairo_bool_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            (*r).u.blit.stride = (*dst).stride as libc::c_int;
            let ref mut fresh57 = (*r).u.blit.data;
            *fresh57 = (*dst).data;
            (*r).u.blit.src_stride = (*src).stride as libc::c_int;
            let ref mut fresh58 = (*r).u.blit.src_data;
            *fresh58 = ((*src).data)
                .offset(((*src).stride * ty as libc::c_long) as isize)
                .offset((tx * 4 as libc::c_int) as isize);
            let ref mut fresh59 = (*r).base.render_rows;
            *fresh59 = Some(
                _blit_xrgb32_lerp_spans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_half_open_span_t,
                        libc::c_uint,
                    ) -> cairo_status_t,
            );
        }
    }
    if ((*r).base.render_rows).is_none() {
        let mut src_0: *const cairo_pattern_t = &(*composite_0).source_pattern.base;
        let mut width: libc::c_uint = 0;
        if (*composite_0).is_bounded == 0 as libc::c_int as libc::c_uint {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        let ref mut fresh60 = (*r).base.render_rows;
        *fresh60 = if (*r).bpp == 0xff as libc::c_int {
            Some(
                _inplace_spans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_half_open_span_t,
                        libc::c_uint,
                    ) -> cairo_status_t,
            )
        } else {
            Some(
                _inplace_opacity_spans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_half_open_span_t,
                        libc::c_uint,
                    ) -> cairo_status_t,
            )
        };
        width = ((*composite_0).bounded.width + 3 as libc::c_int & !(3 as libc::c_int))
            as libc::c_uint;
        (*r).u.composite.run_length = 8 as libc::c_int;
        if (*src_0).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
            || (*src_0).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
        {
            (*r).u.composite.run_length = 256 as libc::c_int;
        }
        if ((*dst).base).is_clear() as libc::c_int != 0
            && ((*composite_0).op as libc::c_uint
                == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
                || (*composite_0).op as libc::c_uint
                    == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                || (*composite_0).op as libc::c_uint
                    == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint)
        {
            (*r).op = PIXMAN_OP_SRC as libc::c_int as uint8_t;
        } else if (*composite_0).op as libc::c_uint
            == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        {
            let ref mut fresh61 = (*r).base.render_rows;
            *fresh61 = if (*r).bpp == 0xff as libc::c_int {
                Some(
                    _inplace_src_spans
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_half_open_span_t,
                            libc::c_uint,
                        ) -> cairo_status_t,
                )
            } else {
                Some(
                    _inplace_src_opacity_spans
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_half_open_span_t,
                            libc::c_uint,
                        ) -> cairo_status_t,
                )
            };
            (*r).u.composite.mask_y = (*(*r).composite).unbounded.y;
            width = ((*composite_0).unbounded.width + 3 as libc::c_int
                & !(3 as libc::c_int)) as libc::c_uint;
        } else if (*composite_0).op as libc::c_uint
            == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
        {
            (*r).op = PIXMAN_OP_OUT_REVERSE as libc::c_int as uint8_t;
            src_0 = 0 as *const cairo_pattern_t;
        } else {
            (*r).op = _pixman_operator((*composite_0).op) as uint8_t;
        }
        let ref mut fresh62 = (*r).src;
        *fresh62 = _pixman_image_for_pattern(
            dst,
            src_0,
            0 as libc::c_int,
            &(*composite_0).bounded,
            &(*composite_0).source_sample_area,
            &mut (*r).u.composite.src_x,
            &mut (*r).u.composite.src_y,
        );
        if ((*r).src).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        buf = ((*r)._buf).as_mut_ptr();
        if width
            > (::std::mem::size_of::<cairo_abstract_span_renderer_t>() as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<cairo_image_span_renderer_t>() as libc::c_ulong,
                ) as libc::c_int as libc::c_uint
        {
            buf = (if width != 0 as libc::c_int as libc::c_uint {
                malloc(width as libc::c_ulong)
            } else {
                0 as *mut libc::c_void
            }) as *mut uint8_t;
            if buf.is_null() {
                pixman_image_unref((*r).src);
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
        let ref mut fresh63 = (*r).mask;
        *fresh63 = pixman_image_create_bits(
            PIXMAN_a8,
            width as libc::c_int,
            (*composite_0).unbounded.height,
            buf as *mut uint32_t,
            0 as libc::c_int,
        );
        if ((*r).mask).is_null() {
            pixman_image_unref((*r).src);
            if buf != ((*r)._buf).as_mut_ptr() {
                free(buf as *mut libc::c_void);
            }
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        if buf != ((*r)._buf).as_mut_ptr() {
            pixman_image_set_destroy_function(
                (*r).mask,
                Some(
                    free_pixels
                        as unsafe extern "C" fn(
                            *mut pixman_image_t,
                            *mut libc::c_void,
                        ) -> (),
                ),
                buf as *mut libc::c_void,
            );
        }
        let ref mut fresh64 = (*r).u.composite.dst;
        *fresh64 = (*dst).pixman_image;
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn span_renderer_init(
    mut _r: *mut cairo_abstract_span_renderer_t,
    mut composite_0: *const cairo_composite_rectangles_t,
    mut antialias: cairo_antialias_t,
    mut needs_clip: cairo_bool_t,
) -> cairo_int_status_t {
    let mut r: *mut cairo_image_span_renderer_t = _r as *mut cairo_image_span_renderer_t;
    let mut dst: *mut cairo_image_surface_t = (*composite_0).surface
        as *mut cairo_image_surface_t;
    let mut source: *const cairo_pattern_t = &(*composite_0).source_pattern.base;
    let mut op: cairo_operator_t = (*composite_0).op;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if needs_clip != 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    let ref mut fresh65 = (*r).composite;
    *fresh65 = composite_0;
    let ref mut fresh66 = (*r).mask;
    *fresh66 = 0 as *mut pixman_image_t;
    let ref mut fresh67 = (*r).src;
    *fresh67 = 0 as *mut pixman_image_t;
    let ref mut fresh68 = (*r).base.finish;
    *fresh68 = None;
    status = mono_renderer_init(r, composite_0, antialias, needs_clip);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = inplace_renderer_init(r, composite_0, antialias, needs_clip);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    (*r).bpp = 0 as libc::c_int;
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
        source = &_cairo_pattern_white.base;
        op = PIXMAN_OP_OUT_REVERSE as libc::c_int as cairo_operator_t;
    } else if ((*dst).base).is_clear() as libc::c_int != 0
        && (op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint)
    {
        op = PIXMAN_OP_SRC as libc::c_int as cairo_operator_t;
    } else if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
    {
        if _cairo_pattern_is_opaque(
            &(*composite_0).source_pattern.base,
            &(*composite_0).source_sample_area,
        ) != 0
        {
            op = PIXMAN_OP_OVER as libc::c_int as cairo_operator_t;
        } else {
            return CAIRO_INT_STATUS_UNSUPPORTED
        }
    } else {
        op = _pixman_operator(op) as cairo_operator_t;
    }
    (*r).op = op as uint8_t;
    let ref mut fresh69 = (*r).src;
    *fresh69 = _pixman_image_for_pattern(
        dst,
        source,
        0 as libc::c_int,
        &(*composite_0).unbounded,
        &(*composite_0).source_sample_area,
        &mut (*r).u.mask.src_x,
        &mut (*r).u.mask.src_y,
    );
    if ((*r).src).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    (*r).opacity = 1.0f64 as libc::c_float;
    if (*composite_0).mask_pattern.base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        (*r).opacity = (*composite_0).mask_pattern.solid.color.alpha as libc::c_float;
    } else {
        let mut mask: *mut pixman_image_t = 0 as *mut pixman_image_t;
        let mut mask_x: libc::c_int = 0;
        let mut mask_y: libc::c_int = 0;
        mask = _pixman_image_for_pattern(
            dst,
            &(*composite_0).mask_pattern.base,
            1 as libc::c_int,
            &(*composite_0).unbounded,
            &(*composite_0).mask_sample_area,
            &mut mask_x,
            &mut mask_y,
        );
        if mask.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        if (*dst).base.content as libc::c_uint
            & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            && _cairo_pattern_is_opaque(source, &(*composite_0).source_sample_area) != 0
        {
            pixman_image_unref((*r).src);
            let ref mut fresh70 = (*r).src;
            *fresh70 = mask;
            (*r).u.mask.src_x = mask_x;
            (*r).u.mask.src_y = mask_y;
            mask = 0 as *mut pixman_image_t;
        }
        if !mask.is_null() {
            pixman_image_unref(mask);
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
    }
    (*r).u.mask.extents = (*composite_0).unbounded;
    (*r)
        .u
        .mask
        .stride = ((*r).u.mask.extents.width + 3 as libc::c_int & !(3 as libc::c_int))
        as ptrdiff_t;
    if (*r).u.mask.extents.height as libc::c_long * (*r).u.mask.stride
        > (::std::mem::size_of::<cairo_abstract_span_renderer_t>() as libc::c_ulong)
            .wrapping_sub(
                ::std::mem::size_of::<cairo_image_span_renderer_t>() as libc::c_ulong,
            ) as libc::c_int as libc::c_long
    {
        let ref mut fresh71 = (*r).mask;
        *fresh71 = pixman_image_create_bits(
            PIXMAN_a8,
            (*r).u.mask.extents.width,
            (*r).u.mask.extents.height,
            0 as *mut uint32_t,
            0 as libc::c_int,
        );
        let ref mut fresh72 = (*r).base.render_rows;
        *fresh72 = Some(
            _cairo_image_spans
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *const cairo_half_open_span_t,
                    libc::c_uint,
                ) -> cairo_status_t,
        );
        let ref mut fresh73 = (*r).base.finish;
        *fresh73 = None;
    } else {
        let ref mut fresh74 = (*r).mask;
        *fresh74 = pixman_image_create_bits(
            PIXMAN_a8,
            (*r).u.mask.extents.width,
            (*r).u.mask.extents.height,
            ((*r)._buf).as_mut_ptr() as *mut uint32_t,
            (*r).u.mask.stride as libc::c_int,
        );
        let ref mut fresh75 = (*r).base.render_rows;
        *fresh75 = Some(
            _cairo_image_spans_and_zero
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *const cairo_half_open_span_t,
                    libc::c_uint,
                ) -> cairo_status_t,
        );
        let ref mut fresh76 = (*r).base.finish;
        *fresh76 = Some(
            _cairo_image_finish_spans_and_zero
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        );
    }
    if ((*r).mask).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh77 = (*r).u.mask.data;
    *fresh77 = pixman_image_get_data((*r).mask) as *mut uint8_t;
    (*r).u.mask.stride = pixman_image_get_stride((*r).mask) as ptrdiff_t;
    (*r).u.mask.extents.height += (*r).u.mask.extents.y;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn span_renderer_fini(
    mut _r: *mut cairo_abstract_span_renderer_t,
    mut status: cairo_int_status_t,
) {
    let mut r: *mut cairo_image_span_renderer_t = _r as *mut cairo_image_span_renderer_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        if ((*r).base.finish).is_some() {
            ((*r).base.finish)
                .expect("non-null function pointer")(r as *mut libc::c_void);
        }
    }
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*r).bpp == 0 as libc::c_int
    {
        let mut composite_0: *const cairo_composite_rectangles_t = (*r).composite;
        pixman_image_composite32(
            (*r).op as pixman_op_t,
            (*r).src,
            (*r).mask,
            to_pixman_image((*composite_0).surface),
            (*composite_0).unbounded.x + (*r).u.mask.src_x,
            (*composite_0).unbounded.y + (*r).u.mask.src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            (*composite_0).unbounded.x,
            (*composite_0).unbounded.y,
            (*composite_0).unbounded.width,
            (*composite_0).unbounded.height,
        );
    }
    if !((*r).src).is_null() {
        pixman_image_unref((*r).src);
    }
    if !((*r).mask).is_null() {
        pixman_image_unref((*r).mask);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_spans_compositor_get() -> *const cairo_compositor_t {
    static mut once: cairo_atomic_once_t = 0 as libc::c_int;
    static mut spans: cairo_spans_compositor_t = cairo_spans_compositor_t {
        base: cairo_compositor_t {
            delegate: 0 as *const cairo_compositor_t,
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            glyphs: None,
        },
        flags: 0,
        fill_boxes: None,
        draw_image_boxes: None,
        copy_boxes: None,
        pattern_to_surface: None,
        composite_boxes: None,
        renderer_init: None,
        renderer_fini: None,
    };
    static mut shape: cairo_compositor_t = cairo_compositor_t {
        delegate: 0 as *const cairo_compositor_t,
        paint: None,
        mask: None,
        stroke: None,
        fill: None,
        glyphs: None,
    };
    if _cairo_atomic_init_once_enter(&mut once) != 0 {
        _cairo_shape_mask_compositor_init(
            &mut shape,
            _cairo_image_traps_compositor_get(),
        );
        shape.glyphs = None;
        _cairo_spans_compositor_init(&mut spans, &mut shape);
        spans.flags = 0 as libc::c_int as libc::c_uint;
        spans
            .fill_boxes = Some(
            fill_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    cairo_operator_t,
                    *const cairo_color_t,
                    *mut cairo_boxes_t,
                ) -> cairo_int_status_t,
        );
        spans
            .draw_image_boxes = Some(
            draw_image_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_image_surface_t,
                    *mut cairo_boxes_t,
                    libc::c_int,
                    libc::c_int,
                ) -> cairo_int_status_t,
        );
        spans
            .pattern_to_surface = Some(
            _cairo_image_source_create_for_pattern
                as unsafe extern "C" fn(
                    *mut cairo_surface_t,
                    *const cairo_pattern_t,
                    cairo_bool_t,
                    *const cairo_rectangle_int_t,
                    *const cairo_rectangle_int_t,
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> *mut cairo_surface_t,
        );
        spans
            .composite_boxes = Some(
            composite_boxes
                as unsafe extern "C" fn(
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
        );
        spans
            .renderer_init = Some(
            span_renderer_init
                as unsafe extern "C" fn(
                    *mut cairo_abstract_span_renderer_t,
                    *const cairo_composite_rectangles_t,
                    cairo_antialias_t,
                    cairo_bool_t,
                ) -> cairo_int_status_t,
        );
        spans
            .renderer_fini = Some(
            span_renderer_fini
                as unsafe extern "C" fn(
                    *mut cairo_abstract_span_renderer_t,
                    cairo_int_status_t,
                ) -> (),
        );
        _cairo_atomic_init_once_leave(&mut once);
    }
    return &mut spans.base;
}
