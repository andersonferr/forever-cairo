use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type _cairo_xlib_shm_display;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_flush(surface: *mut cairo_surface_t);
    fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut libc::c_uchar;
    fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> libc::c_int;
    fn cairo_matrix_init_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn pixman_image_create_bits(
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        bits: *mut uint32_t,
        rowstride_bytes: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_image_unref(image: *mut pixman_image_t) -> pixman_bool_t;
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_flush(
        surface: *mut cairo_surface_t,
        flags: libc::c_uint,
    ) -> cairo_status_t;
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
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_format_from_content(content: cairo_content_t) -> cairo_format_t;
    fn _pixman_format_from_masks(
        masks: *mut cairo_format_masks_t,
        format_ret: *mut pixman_format_code_t,
    ) -> cairo_bool_t;
    fn _pixman_format_to_masks(
        pixman_format: pixman_format_code_t,
        masks: *mut cairo_format_masks_t,
    ) -> cairo_bool_t;
    fn _cairo_image_surface_create_with_pixman_format(
        data: *mut libc::c_uchar,
        pixman_format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        stride: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_assume_ownership_of_data(
        surface: *mut cairo_image_surface_t,
    );
    fn XInitImage(_: *mut XImage) -> libc::c_int;
    fn XGetImage(
        _: *mut Display,
        _: Drawable,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_ulong,
        _: libc::c_int,
    ) -> *mut XImage;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;
    fn XMaxRequestSize(_: *mut Display) -> libc::c_long;
    fn XExtendedMaxRequestSize(_: *mut Display) -> libc::c_long;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
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
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XRenderFindVisualFormat(
        dpy: *mut Display,
        visual: *const Visual,
    ) -> *mut XRenderPictFormat;
    fn XRenderCreatePicture(
        dpy: *mut Display,
        drawable: Drawable,
        format: *const XRenderPictFormat,
        valuemask: libc::c_ulong,
        attributes: *const XRenderPictureAttributes,
    ) -> Picture;
    fn XRenderChangePicture(
        dpy: *mut Display,
        picture: Picture,
        valuemask: libc::c_ulong,
        attributes: *const XRenderPictureAttributes,
    );
    fn XRenderFreePicture(dpy: *mut Display, picture: Picture);
    fn _cairo_xlib_screen_get_visual_info(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        visual: *mut Visual,
        out: *mut *mut cairo_xlib_visual_info_t,
    ) -> cairo_status_t;
    fn _cairo_xlib_screen_get_font_options(
        info: *mut cairo_xlib_screen_t,
    ) -> *mut cairo_font_options_t;
    fn _cairo_xlib_screen_put_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        gc: GC,
    );
    fn _cairo_xlib_screen_get(
        dpy: *mut Display,
        screen: *mut Screen,
        out: *mut *mut cairo_xlib_screen_t,
    ) -> cairo_status_t;
    fn _cairo_xlib_display_get_xrender_format(
        display: *mut cairo_xlib_display_t,
        format: cairo_format_t,
    ) -> *mut XRenderPictFormat;
    fn _cairo_xlib_screen_get_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        drawable: Drawable,
    ) -> GC;
    fn _cairo_xlib_display_acquire(
        device: *mut cairo_device_t,
        display: *mut *mut cairo_xlib_display_t,
    ) -> cairo_status_t;
    fn _cairo_xlib_surface_create_shm__image(
        surface: *mut cairo_xlib_surface_t,
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_xlib_shm_surface_get_ximage(
        surface: *mut cairo_surface_t,
        ximage: *mut XImage,
    );
    fn _cairo_xlib_shm_surface_get_obdata(
        surface: *mut cairo_surface_t,
    ) -> *mut libc::c_void;
    fn _cairo_xlib_surface_create_similar_shm(
        surface: *mut libc::c_void,
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_xlib_surface_get_shm(
        surface: *mut cairo_xlib_surface_t,
        overwrite: cairo_bool_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_xlib_surface_put_shm(
        surface: *mut cairo_xlib_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_xlib_shm_surface_is_idle(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn _cairo_xlib_shm_surface_is_active(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_compositor_paint(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_mask(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_stroke(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_fill(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_compositor_glyphs(
        compositor: *const cairo_compositor_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        scaled_font: *mut cairo_scaled_font_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_damage_add_rectangle(
        damage: *mut cairo_damage_t,
        rect: *const cairo_rectangle_int_t,
    ) -> *mut cairo_damage_t;
    fn _cairo_damage_destroy(damage: *mut cairo_damage_t);
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_image_surface_map_to_image(
        abstract_other: *mut libc::c_void,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_image_surface_unmap_image(
        abstract_surface: *mut libc::c_void,
        image: *mut cairo_image_surface_t,
    ) -> cairo_int_status_t;
    fn XShmPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XShmGetImage(
        _: *mut Display,
        _: Drawable,
        _: *mut XImage,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_damage {
    pub status: cairo_status_t,
    pub region: *mut cairo_region_t,
    pub dirty: libc::c_int,
    pub remain: libc::c_int,
    pub chunks: _cairo_damage_chunk,
    pub tail: *mut _cairo_damage_chunk,
    pub boxes: [cairo_box_t; 32],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_damage_chunk {
    pub next: *mut _cairo_damage_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
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
pub type Atom = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub xoffset: libc::c_int,
    pub format: libc::c_int,
    pub data: *mut libc::c_char,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub depth: libc::c_int,
    pub bytes_per_line: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option::<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
            libc::c_uint,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option::<unsafe extern "C" fn(*mut _XImage) -> libc::c_int>,
    pub get_pixel: Option::<
        unsafe extern "C" fn(*mut _XImage, libc::c_int, libc::c_int) -> libc::c_ulong,
    >,
    pub put_pixel: Option::<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub sub_image: Option::<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_uint,
            libc::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option::<
        unsafe extern "C" fn(*mut _XImage, libc::c_long) -> libc::c_int,
    >,
}
pub type XImage = _XImage;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
pub type XErrorHandler = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRenderPictureAttributes {
    pub repeat: libc::c_int,
    pub alpha_map: Picture,
    pub alpha_x_origin: libc::c_int,
    pub alpha_y_origin: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub graphics_exposures: libc::c_int,
    pub subwindow_mode: libc::c_int,
    pub poly_edge: libc::c_int,
    pub poly_mode: libc::c_int,
    pub dither: Atom,
    pub component_alpha: libc::c_int,
}
pub type XRenderPictureAttributes = _XRenderPictureAttributes;
pub type cairo_xlib_screen_t = _cairo_xlib_screen;
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
pub type cairo_xlib_surface_t = _cairo_xlib_surface;
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
pub type cairo_xlib_display_t = _cairo_xlib_display;
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
    pub last_solid_cache: [C2RustUnnamed_0; 2],
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
pub struct C2RustUnnamed_0 {
    pub color: uint32_t,
    pub index: libc::c_int,
}
pub type cairo_xlib_shm_display_t = _cairo_xlib_shm_display;
pub type cairo_xlib_visual_info_t = _cairo_xlib_visual_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_visual_info {
    pub link: cairo_list_t,
    pub visualid: VisualID,
    pub colors: [C2RustUnnamed_1; 256],
    pub cube_to_pseudocolor: [[[uint8_t; 6]; 6]; 6],
    pub field8_to_cube: [uint8_t; 256],
    pub dither8_to_cube: [int8_t; 256],
    pub gray8_to_pseudocolor: [uint8_t; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub a: uint8_t,
    pub r: uint8_t,
    pub g: uint8_t,
    pub b: uint8_t,
}
pub type cairo_xlib_error_func_t = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
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
unsafe extern "C" fn _cairo_popcount(mut mask: uint32_t) -> libc::c_int {
    return mask.count_ones() as i32;
}
#[inline(always)]
unsafe extern "C" fn _cairo_is_little_endian() -> cairo_bool_t {
    static mut i: libc::c_int = 1 as libc::c_int;
    return (*(&i as *const libc::c_int as *mut libc::c_char) as libc::c_int
        == 0x1 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh2 = (*entry).next;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).prev;
    *fresh3 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh4 = (*next).prev;
    *fresh4 = entry;
    let ref mut fresh5 = (*entry).next;
    *fresh5 = next;
    let ref mut fresh6 = (*entry).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh8 = (*next).prev;
    *fresh8 = prev;
    let ref mut fresh9 = (*prev).next;
    *fresh9 = next;
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
unsafe extern "C" fn _visual_for_xrender_format(
    mut screen: *mut Screen,
    mut xrender_format: *mut XRenderPictFormat,
) -> *mut Visual {
    let mut d: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    d = 0 as libc::c_int;
    while d < (*screen).ndepths {
        let mut d_info: *mut Depth = &mut *((*screen).depths).offset(d as isize)
            as *mut Depth;
        if !((*d_info).depth != (*xrender_format).depth) {
            let mut current_block_3: u64;
            v = 0 as libc::c_int;
            while v < (*d_info).nvisuals {
                let mut visual: *mut Visual = &mut *((*d_info).visuals)
                    .offset(v as isize) as *mut Visual;
                match (*visual).class {
                    4 => {
                        if (*xrender_format).type_0 != 1 as libc::c_int {
                            current_block_3 = 7095457783677275021;
                        } else {
                            current_block_3 = 3512920355445576850;
                        }
                    }
                    5 => {
                        current_block_3 = 7095457783677275021;
                    }
                    0 | 1 | 2 | 3 => {
                        if (*xrender_format).type_0 != 0 as libc::c_int {
                            current_block_3 = 7095457783677275021;
                        } else {
                            current_block_3 = 3512920355445576850;
                        }
                    }
                    _ => {
                        current_block_3 = 3512920355445576850;
                    }
                }
                match current_block_3 {
                    3512920355445576850 => {
                        if xrender_format
                            == XRenderFindVisualFormat((*screen).display, visual)
                        {
                            return visual;
                        }
                    }
                    _ => {}
                }
                v += 1;
            }
        }
        d += 1;
    }
    return 0 as *mut Visual;
}
unsafe extern "C" fn _xrender_format_to_content(
    mut xrender_format: *mut XRenderPictFormat,
) -> cairo_content_t {
    let mut content: cairo_content_t = 0 as cairo_content_t;
    if xrender_format.is_null() {
        return CAIRO_CONTENT_COLOR;
    }
    content = 0 as cairo_content_t;
    if (*xrender_format).direct.alphaMask != 0 {
        content = ::std::mem::transmute::<
            libc::c_uint,
            cairo_content_t,
        >(content as libc::c_uint | CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint);
    }
    if (*xrender_format).direct.redMask as libc::c_int
        | (*xrender_format).direct.greenMask as libc::c_int
        | (*xrender_format).direct.blueMask as libc::c_int != 0
    {
        content = ::std::mem::transmute::<
            libc::c_uint,
            cairo_content_t,
        >(content as libc::c_uint | CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint);
    }
    return content;
}
unsafe extern "C" fn _cairo_xlib_surface_create_similar(
    mut abstract_src: *mut libc::c_void,
    mut content: cairo_content_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut src: *mut cairo_xlib_surface_t = abstract_src as *mut cairo_xlib_surface_t;
    let mut xrender_format: *mut XRenderPictFormat = 0 as *mut XRenderPictFormat;
    let mut surface: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut pix: Pixmap = 0;
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int {
        return 0 as *mut cairo_surface_t;
    }
    if width == 0 as libc::c_int || height == 0 as libc::c_int {
        return 0 as *mut cairo_surface_t;
    }
    if _cairo_xlib_display_acquire((*src).base.device, &mut display) as u64 != 0 {
        return 0 as *mut cairo_surface_t;
    }
    xrender_format = 0 as *mut XRenderPictFormat;
    if !((*src).xrender_format).is_null()
        && _xrender_format_to_content((*src).xrender_format) as libc::c_uint
            == content as libc::c_uint
    {
        xrender_format = (*src).xrender_format;
    }
    if xrender_format.is_null() {
        xrender_format = _cairo_xlib_display_get_xrender_format(
            display,
            _cairo_format_from_content(content),
        );
    }
    if !xrender_format.is_null() {
        let mut visual: *mut Visual = 0 as *mut Visual;
        pix = XCreatePixmap(
            (*display).display,
            (*src).drawable,
            width as libc::c_uint,
            height as libc::c_uint,
            (*xrender_format).depth as libc::c_uint,
        );
        if xrender_format == (*src).xrender_format {
            visual = (*src).visual;
        } else {
            visual = _visual_for_xrender_format((*(*src).screen).screen, xrender_format);
        }
        surface = _cairo_xlib_surface_create_internal(
            (*src).screen,
            pix,
            visual,
            xrender_format,
            width,
            height,
            (*xrender_format).depth,
        ) as *mut cairo_xlib_surface_t;
    } else {
        let mut screen: *mut Screen = (*(*src).screen).screen;
        let mut depth: libc::c_int = 0;
        if content as libc::c_uint != CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
        {
            cairo_device_release(&mut (*display).base);
            return 0 as *mut cairo_surface_t;
        }
        depth = (*screen).root_depth;
        pix = XCreatePixmap(
            (*display).display,
            (*screen).root,
            (if width <= 0 as libc::c_int { 1 as libc::c_int } else { width })
                as libc::c_uint,
            (if height <= 0 as libc::c_int { 1 as libc::c_int } else { height })
                as libc::c_uint,
            depth as libc::c_uint,
        );
        surface = _cairo_xlib_surface_create_internal(
            (*src).screen,
            pix,
            (*screen).root_visual,
            0 as *mut XRenderPictFormat,
            width,
            height,
            depth,
        ) as *mut cairo_xlib_surface_t;
    }
    if (*surface).base.status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        (*surface).owns_pixmap = 1 as libc::c_int;
    } else {
        XFreePixmap((*display).display, pix);
    }
    cairo_device_release(&mut (*display).base);
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_xlib_surface_discard_shm(
    mut surface: *mut cairo_xlib_surface_t,
) {
    if ((*surface).shm).is_null() {
        return;
    }
    if (*surface).owns_pixmap == 0 {
        cairo_surface_flush((*surface).shm);
    }
    cairo_surface_finish((*surface).shm);
    cairo_surface_destroy((*surface).shm);
    let ref mut fresh10 = (*surface).shm;
    *fresh10 = 0 as *mut cairo_surface_t;
    _cairo_damage_destroy((*surface).base.damage);
    let ref mut fresh11 = (*surface).base.damage;
    *fresh11 = 0 as *mut cairo_damage_t;
    (*surface).fallback = 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_xlib_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    cairo_list_del(&mut (*surface).link);
    status = _cairo_xlib_display_acquire((*surface).base.device, &mut display);
    if status as u64 != 0 {
        return status;
    }
    if (*surface).embedded_source.picture != 0 {
        XRenderFreePicture((*display).display, (*surface).embedded_source.picture);
    }
    if (*surface).picture != 0 {
        XRenderFreePicture((*display).display, (*surface).picture);
    }
    _cairo_xlib_surface_discard_shm(surface);
    if (*surface).owns_pixmap != 0 {
        XFreePixmap((*display).display, (*surface).drawable);
    }
    cairo_device_release(&mut (*display).base);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_get_gc(
    mut display: *mut cairo_xlib_display_t,
    mut surface: *mut cairo_xlib_surface_t,
    mut gc: *mut GC,
) -> cairo_status_t {
    *gc = _cairo_xlib_screen_get_gc(
        display,
        (*surface).screen,
        (*surface).depth,
        (*surface).drawable,
    );
    if (*gc).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _noop_error_handler(
    mut display: *mut Display,
    mut event: *mut XErrorEvent,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn _swap_ximage_2bytes(mut ximage: *mut XImage) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: *mut libc::c_char = (*ximage).data;
    j = (*ximage).height;
    while j != 0 {
        let mut p: *mut uint16_t = line as *mut uint16_t;
        i = (*ximage).width;
        while i != 0 {
            *p = __bswap_16(*p);
            p = p.offset(1);
            i -= 1;
        }
        line = line.offset((*ximage).bytes_per_line as isize);
        j -= 1;
    }
}
unsafe extern "C" fn _swap_ximage_3bytes(mut ximage: *mut XImage) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: *mut libc::c_char = (*ximage).data;
    j = (*ximage).height;
    while j != 0 {
        let mut p: *mut uint8_t = line as *mut uint8_t;
        i = (*ximage).width;
        while i != 0 {
            let mut tmp: uint8_t = 0;
            tmp = *p.offset(2 as libc::c_int as isize);
            *p.offset(2 as libc::c_int as isize) = *p.offset(0 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) = tmp;
            p = p.offset(3 as libc::c_int as isize);
            i -= 1;
        }
        line = line.offset((*ximage).bytes_per_line as isize);
        j -= 1;
    }
}
unsafe extern "C" fn _swap_ximage_4bytes(mut ximage: *mut XImage) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: *mut libc::c_char = (*ximage).data;
    j = (*ximage).height;
    while j != 0 {
        let mut p: *mut uint32_t = line as *mut uint32_t;
        i = (*ximage).width;
        while i != 0 {
            *p = __bswap_32(*p);
            p = p.offset(1);
            i -= 1;
        }
        line = line.offset((*ximage).bytes_per_line as isize);
        j -= 1;
    }
}
unsafe extern "C" fn _swap_ximage_nibbles(mut ximage: *mut XImage) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: *mut libc::c_char = (*ximage).data;
    j = (*ximage).height;
    while j != 0 {
        let mut p: *mut uint8_t = line as *mut uint8_t;
        i = ((*ximage).width + 1 as libc::c_int) / 2 as libc::c_int;
        while i != 0 {
            *p = (*p as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int
                | (*p as libc::c_int) << 4 as libc::c_int & !(0xf as libc::c_int))
                as uint8_t;
            p = p.offset(1);
            i -= 1;
        }
        line = line.offset((*ximage).bytes_per_line as isize);
        j -= 1;
    }
}
unsafe extern "C" fn _swap_ximage_bits(mut ximage: *mut XImage) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: *mut libc::c_char = (*ximage).data;
    let mut unit: libc::c_int = (*ximage).bitmap_unit;
    let mut line_bytes: libc::c_int = ((*ximage).width + unit - 1 as libc::c_int
        & !(unit - 1 as libc::c_int)) / 8 as libc::c_int;
    j = (*ximage).height;
    while j != 0 {
        let mut p: *mut libc::c_char = line;
        i = line_bytes;
        while i != 0 {
            let mut b: libc::c_char = *p;
            b = ((b as libc::c_int) << 1 as libc::c_int & 0xaa as libc::c_int
                | b as libc::c_int >> 1 as libc::c_int & 0x55 as libc::c_int)
                as libc::c_char;
            b = ((b as libc::c_int) << 2 as libc::c_int & 0xcc as libc::c_int
                | b as libc::c_int >> 2 as libc::c_int & 0x33 as libc::c_int)
                as libc::c_char;
            b = ((b as libc::c_int) << 4 as libc::c_int & 0xf0 as libc::c_int
                | b as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                as libc::c_char;
            *p = b;
            p = p.offset(1);
            i -= 1;
        }
        line = line.offset((*ximage).bytes_per_line as isize);
        j -= 1;
    }
}
unsafe extern "C" fn _swap_ximage_to_native(mut ximage: *mut XImage) {
    let mut unit_bytes: libc::c_int = 0 as libc::c_int;
    let mut native_byte_order: libc::c_int = if _cairo_is_little_endian() != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if (*ximage).bits_per_pixel == 1 as libc::c_int
        && (*ximage).bitmap_bit_order != native_byte_order
    {
        _swap_ximage_bits(ximage);
        if (*ximage).bitmap_bit_order == (*ximage).byte_order {
            return;
        }
    }
    if (*ximage).byte_order == native_byte_order {
        return;
    }
    let mut current_block_12: u64;
    match (*ximage).bits_per_pixel {
        1 => {
            unit_bytes = (*ximage).bitmap_unit / 8 as libc::c_int;
            current_block_12 = 15904375183555213903;
        }
        4 => {
            _swap_ximage_nibbles(ximage);
            current_block_12 = 13144111597262374312;
        }
        8 | 16 | 20 | 24 | 28 | 30 | 32 => {
            current_block_12 = 13144111597262374312;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                    557 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 38],
                        &[libc::c_char; 38],
                    >(b"void _swap_ximage_to_native(XImage *)\0"))
                        .as_ptr(),
                );
            }
            current_block_12 = 15904375183555213903;
        }
    }
    match current_block_12 {
        13144111597262374312 => {
            unit_bytes = ((*ximage).bits_per_pixel + 7 as libc::c_int)
                / 8 as libc::c_int;
        }
        _ => {}
    }
    match unit_bytes {
        1 => {}
        2 => {
            _swap_ximage_2bytes(ximage);
        }
        3 => {
            _swap_ximage_3bytes(ximage);
        }
        4 => {
            _swap_ximage_4bytes(ximage);
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                    573 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 38],
                        &[libc::c_char; 38],
                    >(b"void _swap_ximage_to_native(XImage *)\0"))
                        .as_ptr(),
                );
            }
        }
    };
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
#[inline]
unsafe extern "C" fn _resize_field(
    mut field: uint32_t,
    mut width: libc::c_int,
    mut new_width: libc::c_int,
) -> uint32_t {
    if width == 0 as libc::c_int {
        return 0 as libc::c_int as uint32_t;
    }
    if width >= new_width {
        return field >> width - new_width
    } else {
        let mut result: uint32_t = field << new_width - width;
        while width < new_width {
            result |= result >> width;
            width <<= 1 as libc::c_int;
        }
        return result;
    };
}
#[inline]
unsafe extern "C" fn _adjust_field(
    mut field: uint32_t,
    mut adjustment: libc::c_int,
) -> uint32_t {
    return (if (255 as libc::c_int)
        < (if 0 as libc::c_int > field as libc::c_int + adjustment {
            0 as libc::c_int
        } else {
            field as libc::c_int + adjustment
        })
    {
        255 as libc::c_int
    } else if 0 as libc::c_int > field as libc::c_int + adjustment {
        0 as libc::c_int
    } else {
        field as libc::c_int + adjustment
    }) as uint32_t;
}
#[inline]
unsafe extern "C" fn _field_to_8(
    mut field: uint32_t,
    mut width: libc::c_int,
    mut shift: libc::c_int,
) -> uint32_t {
    return _resize_field(field >> shift, width, 8 as libc::c_int);
}
#[inline]
unsafe extern "C" fn _field_to_8_undither(
    mut field: uint32_t,
    mut width: libc::c_int,
    mut shift: libc::c_int,
    mut dither_adjustment: libc::c_int,
) -> uint32_t {
    return _adjust_field(_field_to_8(field, width, shift), -dither_adjustment >> width);
}
#[inline]
unsafe extern "C" fn _field_from_8(
    mut field: uint32_t,
    mut width: libc::c_int,
    mut shift: libc::c_int,
) -> uint32_t {
    return _resize_field(field, 8 as libc::c_int, width) << shift;
}
#[inline]
unsafe extern "C" fn _field_from_8_dither(
    mut field: uint32_t,
    mut width: libc::c_int,
    mut shift: libc::c_int,
    mut dither_adjustment: int8_t,
) -> uint32_t {
    return _field_from_8(
        _adjust_field(field, dither_adjustment as libc::c_int >> width),
        width,
        shift,
    );
}
#[inline]
unsafe extern "C" fn _pseudocolor_from_rgb888_dither(
    mut visual_info: *mut cairo_xlib_visual_info_t,
    mut r: uint32_t,
    mut g: uint32_t,
    mut b: uint32_t,
    mut dither_adjustment: int8_t,
) -> uint32_t {
    if r == g && g == b {
        dither_adjustment = (dither_adjustment as libc::c_int / 16 as libc::c_int)
            as int8_t;
        return (*visual_info)
            .gray8_to_pseudocolor[_adjust_field(r, dither_adjustment as libc::c_int)
            as usize] as uint32_t;
    } else {
        dither_adjustment = (*visual_info)
            .dither8_to_cube[(dither_adjustment as libc::c_int + 128 as libc::c_int)
            as usize];
        return (*visual_info)
            .cube_to_pseudocolor[(*visual_info)
            .field8_to_cube[_adjust_field(r, dither_adjustment as libc::c_int) as usize]
            as usize][(*visual_info)
            .field8_to_cube[_adjust_field(g, dither_adjustment as libc::c_int) as usize]
            as usize][(*visual_info)
            .field8_to_cube[_adjust_field(b, dither_adjustment as libc::c_int) as usize]
            as usize] as uint32_t;
    };
}
#[inline]
unsafe extern "C" fn _pseudocolor_to_rgb888(
    mut visual_info: *mut cairo_xlib_visual_info_t,
    mut pixel: uint32_t,
) -> uint32_t {
    let mut r: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut b: uint32_t = 0;
    pixel &= 0xff as libc::c_int as libc::c_uint;
    r = (*visual_info).colors[pixel as usize].r as uint32_t;
    g = (*visual_info).colors[pixel as usize].g as uint32_t;
    b = (*visual_info).colors[pixel as usize].b as uint32_t;
    return r << 16 as libc::c_int | g << 8 as libc::c_int | b;
}
static mut dither_pattern: [[int8_t; 4]; 4] = [
    [
        (-(8 as libc::c_int) * 16 as libc::c_int) as int8_t,
        (0 as libc::c_int * 16 as libc::c_int) as int8_t,
        (-(6 as libc::c_int) * 16 as libc::c_int) as int8_t,
        (2 as libc::c_int * 16 as libc::c_int) as int8_t,
    ],
    [
        (4 as libc::c_int * 16 as libc::c_int) as int8_t,
        (-(4 as libc::c_int) * 16 as libc::c_int) as int8_t,
        (6 as libc::c_int * 16 as libc::c_int) as int8_t,
        (-(2 as libc::c_int) * 16 as libc::c_int) as int8_t,
    ],
    [
        (-(5 as libc::c_int) * 16 as libc::c_int) as int8_t,
        (4 as libc::c_int * 16 as libc::c_int) as int8_t,
        (-(7 as libc::c_int) * 16 as libc::c_int) as int8_t,
        (1 as libc::c_int * 16 as libc::c_int) as int8_t,
    ],
    [
        (7 as libc::c_int * 16 as libc::c_int) as int8_t,
        (-(1 as libc::c_int) * 16 as libc::c_int) as int8_t,
        (5 as libc::c_int * 16 as libc::c_int) as int8_t,
        (-(3 as libc::c_int) * 16 as libc::c_int) as int8_t,
    ],
];
unsafe extern "C" fn bits_per_pixel(
    mut surface: *mut cairo_xlib_surface_t,
) -> libc::c_int {
    if (*surface).depth > 16 as libc::c_int {
        return 32 as libc::c_int
    } else if (*surface).depth > 8 as libc::c_int {
        return 16 as libc::c_int
    } else if (*surface).depth > 1 as libc::c_int {
        return 8 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn _pixman_format_for_xlib_surface(
    mut surface: *mut cairo_xlib_surface_t,
) -> pixman_format_code_t {
    let mut masks: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    let mut format: pixman_format_code_t = 0 as pixman_format_code_t;
    masks.bpp = bits_per_pixel(surface);
    masks.alpha_mask = (*surface).a_mask as libc::c_ulong;
    masks.red_mask = (*surface).r_mask as libc::c_ulong;
    masks.green_mask = (*surface).g_mask as libc::c_ulong;
    masks.blue_mask = (*surface).b_mask as libc::c_ulong;
    if _pixman_format_from_masks(&mut masks, &mut format) == 0 {
        return 0 as pixman_format_code_t;
    }
    return format;
}
unsafe extern "C" fn _get_image_surface(
    mut surface: *mut cairo_xlib_surface_t,
    mut extents: *const cairo_rectangle_int_t,
    mut try_shm: libc::c_int,
) -> *mut cairo_surface_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut ximage: *mut XImage = 0 as *mut XImage;
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    if (*extents).x >= 0 as libc::c_int {} else {
        __assert_fail(
            b"extents->x >= 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
            730 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_surface_t *_get_image_surface(cairo_xlib_surface_t *, const cairo_rectangle_int_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*extents).y >= 0 as libc::c_int {} else {
        __assert_fail(
            b"extents->y >= 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
            731 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_surface_t *_get_image_surface(cairo_xlib_surface_t *, const cairo_rectangle_int_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*extents).x + (*extents).width <= (*surface).width {} else {
        __assert_fail(
            b"extents->x + extents->width <= surface->width\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
            732 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_surface_t *_get_image_surface(cairo_xlib_surface_t *, const cairo_rectangle_int_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if (*extents).y + (*extents).height <= (*surface).height {} else {
        __assert_fail(
            b"extents->y + extents->height <= surface->height\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
            733 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_surface_t *_get_image_surface(cairo_xlib_surface_t *, const cairo_rectangle_int_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*surface).base).is_clear() as libc::c_int != 0
        || (*surface).base.serial == 0 as libc::c_int as libc::c_uint
            && (*surface).owns_pixmap != 0
    {
        pixman_format = _pixman_format_for_xlib_surface(surface);
        if pixman_format as u64 != 0 {
            return _cairo_image_surface_create_with_pixman_format(
                0 as *mut libc::c_uchar,
                pixman_format,
                (*extents).width,
                (*extents).height,
                0 as libc::c_int,
            );
        }
    }
    if !((*surface).shm).is_null() {
        let mut src: *mut cairo_image_surface_t = (*surface).shm
            as *mut cairo_image_surface_t;
        let mut dst: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
            surface: 0 as *mut cairo_surface_t,
        };
        dst = cairo_image_surface_create(
            (*src).format,
            (*extents).width,
            (*extents).height,
        );
        if (*dst).status as u64 != 0 {
            return dst;
        }
        _cairo_pattern_init_for_surface(&mut pattern, &mut (*src).base);
        cairo_matrix_init_translate(
            &mut pattern.base.matrix,
            (*extents).x as libc::c_double,
            (*extents).y as libc::c_double,
        );
        status = _cairo_surface_paint(
            dst,
            CAIRO_OPERATOR_SOURCE,
            &mut pattern.base,
            0 as *const cairo_clip_t,
        ) as cairo_int_status_t;
        _cairo_pattern_fini(&mut pattern.base);
        if status as u64 != 0 {
            cairo_surface_destroy(dst);
            dst = _cairo_surface_create_in_error(status as cairo_status_t);
        }
        return dst;
    }
    status = _cairo_xlib_display_acquire((*surface).base.device, &mut display)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    pixman_format = _pixman_format_for_xlib_surface(surface);
    if try_shm != 0 && pixman_format as libc::c_uint != 0 {
        image = _cairo_xlib_surface_create_shm__image(
            surface,
            pixman_format,
            (*extents).width,
            (*extents).height,
        ) as *mut cairo_image_surface_t;
        if !image.is_null()
            && (*image).base.status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            let mut old_handler: cairo_xlib_error_func_t = None;
            let mut shm_image: XImage = XImage {
                width: 0,
                height: 0,
                xoffset: 0,
                format: 0,
                data: 0 as *mut libc::c_char,
                byte_order: 0,
                bitmap_unit: 0,
                bitmap_bit_order: 0,
                bitmap_pad: 0,
                depth: 0,
                bytes_per_line: 0,
                bits_per_pixel: 0,
                red_mask: 0,
                green_mask: 0,
                blue_mask: 0,
                obdata: 0 as *mut libc::c_char,
                f: funcs {
                    create_image: None,
                    destroy_image: None,
                    get_pixel: None,
                    put_pixel: None,
                    sub_image: None,
                    add_pixel: None,
                },
            };
            let mut success: libc::c_int = 0;
            _cairo_xlib_shm_surface_get_ximage(&mut (*image).base, &mut shm_image);
            XSync((*display).display, 0 as libc::c_int);
            old_handler = XSetErrorHandler(
                Some(
                    _noop_error_handler
                        as unsafe extern "C" fn(
                            *mut Display,
                            *mut XErrorEvent,
                        ) -> libc::c_int,
                ),
            );
            success = XShmGetImage(
                (*display).display,
                (*surface).drawable,
                &mut shm_image,
                (*extents).x,
                (*extents).y,
                !(0 as libc::c_long) as libc::c_ulong,
            );
            XSetErrorHandler(old_handler);
            if success != 0 {
                cairo_device_release(&mut (*display).base);
                return &mut (*image).base;
            }
            cairo_surface_destroy(&mut (*image).base);
            image = 0 as *mut cairo_image_surface_t;
        }
    }
    if (*surface).use_pixmap == 0 as libc::c_int {
        let mut old_handler_0: cairo_xlib_error_func_t = None;
        XSync((*display).display, 0 as libc::c_int);
        old_handler_0 = XSetErrorHandler(
            Some(
                _noop_error_handler
                    as unsafe extern "C" fn(
                        *mut Display,
                        *mut XErrorEvent,
                    ) -> libc::c_int,
            ),
        );
        ximage = XGetImage(
            (*display).display,
            (*surface).drawable,
            (*extents).x,
            (*extents).y,
            (*extents).width as libc::c_uint,
            (*extents).height as libc::c_uint,
            !(0 as libc::c_long) as libc::c_ulong,
            2 as libc::c_int,
        );
        XSetErrorHandler(old_handler_0);
        if ximage.is_null() {
            (*surface).use_pixmap = 20 as libc::c_int;
        }
    } else {
        let ref mut fresh12 = (*surface).use_pixmap;
        *fresh12 -= 1;
        ximage = 0 as *mut XImage;
    }
    if ximage.is_null() {
        let mut pixmap: Pixmap = 0;
        let mut gc: GC = 0 as *mut _XGC;
        status = _cairo_xlib_surface_get_gc(display, surface, &mut gc)
            as cairo_int_status_t;
        if status as u64 != 0 {
            current_block = 787884903692094323;
        } else {
            pixmap = XCreatePixmap(
                (*display).display,
                (*surface).drawable,
                (*extents).width as libc::c_uint,
                (*extents).height as libc::c_uint,
                (*surface).depth as libc::c_uint,
            );
            if pixmap != 0 {
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
                    (*display).display,
                    gc,
                    ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
                    &mut gcv,
                );
                XCopyArea(
                    (*display).display,
                    (*surface).drawable,
                    pixmap,
                    gc,
                    (*extents).x,
                    (*extents).y,
                    (*extents).width as libc::c_uint,
                    (*extents).height as libc::c_uint,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                gcv.subwindow_mode = 0 as libc::c_int;
                XChangeGC(
                    (*display).display,
                    gc,
                    ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
                    &mut gcv,
                );
                ximage = XGetImage(
                    (*display).display,
                    pixmap,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*extents).width as libc::c_uint,
                    (*extents).height as libc::c_uint,
                    !(0 as libc::c_long) as libc::c_ulong,
                    2 as libc::c_int,
                );
                XFreePixmap((*display).display, pixmap);
            }
            _cairo_xlib_surface_put_gc(display, surface, gc);
            if ximage.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                current_block = 787884903692094323;
            } else {
                current_block = 13484060386966298149;
            }
        }
    } else {
        current_block = 13484060386966298149;
    }
    match current_block {
        13484060386966298149 => {
            _swap_ximage_to_native(ximage);
            if pixman_format as libc::c_uint != 0
                && (*ximage).bitmap_unit == 32 as libc::c_int
                && (*ximage).bitmap_pad == 32 as libc::c_int
                && (((*surface).visual).is_null()
                    || (*(*surface).visual).class == 4 as libc::c_int)
            {
                image = _cairo_image_surface_create_with_pixman_format(
                    (*ximage).data as *mut libc::c_uchar,
                    pixman_format,
                    (*ximage).width,
                    (*ximage).height,
                    (*ximage).bytes_per_line,
                ) as *mut cairo_image_surface_t;
                status = (*image).base.status as cairo_int_status_t;
                if !(status as u64 != 0) {
                    _cairo_image_surface_assume_ownership_of_data(image);
                    let ref mut fresh13 = (*ximage).data;
                    *fresh13 = 0 as *mut libc::c_char;
                }
            } else {
                let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
                let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut row: *mut uint32_t = 0 as *mut uint32_t;
                let mut in_pixel: uint32_t = 0;
                let mut out_pixel: uint32_t = 0;
                let mut rowstride: libc::c_uint = 0;
                let mut a_mask: uint32_t = 0 as libc::c_int as uint32_t;
                let mut r_mask: uint32_t = 0 as libc::c_int as uint32_t;
                let mut g_mask: uint32_t = 0 as libc::c_int as uint32_t;
                let mut b_mask: uint32_t = 0 as libc::c_int as uint32_t;
                let mut a_width: libc::c_int = 0 as libc::c_int;
                let mut r_width: libc::c_int = 0 as libc::c_int;
                let mut g_width: libc::c_int = 0 as libc::c_int;
                let mut b_width: libc::c_int = 0 as libc::c_int;
                let mut a_shift: libc::c_int = 0 as libc::c_int;
                let mut r_shift: libc::c_int = 0 as libc::c_int;
                let mut g_shift: libc::c_int = 0 as libc::c_int;
                let mut b_shift: libc::c_int = 0 as libc::c_int;
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut x0: libc::c_int = 0;
                let mut y0: libc::c_int = 0;
                let mut x_off: libc::c_int = 0;
                let mut y_off: libc::c_int = 0;
                let mut visual_info: *mut cairo_xlib_visual_info_t = 0
                    as *mut cairo_xlib_visual_info_t;
                if ((*surface).visual).is_null()
                    || (*(*surface).visual).class == 4 as libc::c_int
                {
                    let mut has_alpha: cairo_bool_t = 0;
                    let mut has_color: cairo_bool_t = 0;
                    has_alpha = (*surface).a_mask as cairo_bool_t;
                    has_color = ((*surface).r_mask != 0 || (*surface).g_mask != 0
                        || (*surface).b_mask != 0) as libc::c_int;
                    if has_color != 0 {
                        if has_alpha != 0 {
                            format = CAIRO_FORMAT_ARGB32;
                        } else {
                            format = CAIRO_FORMAT_RGB24;
                        }
                    } else {
                        format = CAIRO_FORMAT_ARGB32;
                    }
                    a_mask = (*surface).a_mask;
                    r_mask = (*surface).r_mask;
                    g_mask = (*surface).g_mask;
                    b_mask = (*surface).b_mask;
                    _characterize_field(a_mask, &mut a_width, &mut a_shift);
                    _characterize_field(r_mask, &mut r_width, &mut r_shift);
                    _characterize_field(g_mask, &mut g_width, &mut g_shift);
                    _characterize_field(b_mask, &mut b_width, &mut b_shift);
                    current_block = 2652804691515851435;
                } else {
                    format = CAIRO_FORMAT_RGB24;
                    status = _cairo_xlib_screen_get_visual_info(
                        display,
                        (*surface).screen,
                        (*surface).visual,
                        &mut visual_info,
                    ) as cairo_int_status_t;
                    if status as u64 != 0 {
                        current_block = 787884903692094323;
                    } else {
                        current_block = 2652804691515851435;
                    }
                }
                match current_block {
                    787884903692094323 => {}
                    _ => {
                        image = cairo_image_surface_create(
                            format,
                            (*ximage).width,
                            (*ximage).height,
                        ) as *mut cairo_image_surface_t;
                        status = (*image).base.status as cairo_int_status_t;
                        if !(status as u64 != 0) {
                            data = cairo_image_surface_get_data(&mut (*image).base);
                            rowstride = (cairo_image_surface_get_stride(
                                &mut (*image).base,
                            ) >> 2 as libc::c_int) as libc::c_uint;
                            row = data as *mut uint32_t;
                            x0 = ((*extents).x as libc::c_double
                                + (*surface).base.device_transform.x0) as libc::c_int;
                            y0 = ((*extents).y as libc::c_double
                                + (*surface).base.device_transform.y0) as libc::c_int;
                            y = 0 as libc::c_int;
                            y_off = y0
                                % (::std::mem::size_of::<[[int8_t; 4]; 4]>()
                                    as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong,
                                    ) as libc::c_int;
                            while y < (*ximage).height {
                                let mut dither_row: *const int8_t = (dither_pattern[y_off
                                    as usize])
                                    .as_ptr();
                                x = 0 as libc::c_int;
                                x_off = x0
                                    % (::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                                        ) as libc::c_int;
                                while x < (*ximage).width {
                                    let mut dither_adjustment: libc::c_int = *dither_row
                                        .offset(x_off as isize) as libc::c_int;
                                    in_pixel = (Some(
                                        ((*ximage).f.get_pixel).expect("non-null function pointer"),
                                    ))
                                        .expect("non-null function pointer")(ximage, x, y)
                                        as uint32_t;
                                    if visual_info.is_null() {
                                        out_pixel = _field_to_8(in_pixel & a_mask, a_width, a_shift)
                                            << 24 as libc::c_int
                                            | _field_to_8_undither(
                                                in_pixel & r_mask,
                                                r_width,
                                                r_shift,
                                                dither_adjustment,
                                            ) << 16 as libc::c_int
                                            | _field_to_8_undither(
                                                in_pixel & g_mask,
                                                g_width,
                                                g_shift,
                                                dither_adjustment,
                                            ) << 8 as libc::c_int
                                            | _field_to_8_undither(
                                                in_pixel & b_mask,
                                                b_width,
                                                b_shift,
                                                dither_adjustment,
                                            );
                                    } else {
                                        out_pixel = _pseudocolor_to_rgb888(visual_info, in_pixel);
                                    }
                                    *row.offset(x as isize) = out_pixel;
                                    x += 1;
                                    x_off = (x_off + 1 as libc::c_int)
                                        % (::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                                            ) as libc::c_int;
                                }
                                row = row.offset(rowstride as isize);
                                y += 1;
                                y_off = (y_off + 1 as libc::c_int)
                                    % (::std::mem::size_of::<[[int8_t; 4]; 4]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong,
                                        ) as libc::c_int;
                            }
                            cairo_surface_mark_dirty(&mut (*image).base);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !ximage.is_null() {
        (Some(((*ximage).f.destroy_image).expect("non-null function pointer")))
            .expect("non-null function pointer")(ximage);
    }
    cairo_device_release(&mut (*display).base);
    if status as u64 != 0 {
        if !image.is_null() {
            cairo_surface_destroy(&mut (*image).base);
        }
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    return &mut (*image).base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_set_precision(
    mut surface: *mut cairo_xlib_surface_t,
    mut antialias: cairo_antialias_t,
) {
    let mut display: *mut cairo_xlib_display_t = (*surface).display;
    let mut precision: libc::c_int = 0;
    if (*display).force_precision != -(1 as libc::c_int) {
        precision = (*display).force_precision;
    } else {
        match antialias as libc::c_uint {
            6 | 3 => {
                precision = 0 as libc::c_int;
            }
            0 | 2 | 1 | 4 | 5 | _ => {
                precision = 1 as libc::c_int;
            }
        }
    }
    if (*surface).precision != precision {
        let mut pa: XRenderPictureAttributes = XRenderPictureAttributes {
            repeat: 0,
            alpha_map: 0,
            alpha_x_origin: 0,
            alpha_y_origin: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            graphics_exposures: 0,
            subwindow_mode: 0,
            poly_edge: 0,
            poly_mode: 0,
            dither: 0,
            component_alpha: 0,
        };
        pa.poly_mode = precision;
        XRenderChangePicture(
            (*display).display,
            (*surface).picture,
            ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong,
            &mut pa,
        );
        (*surface).precision = precision;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_ensure_picture(
    mut surface: *mut cairo_xlib_surface_t,
) {
    let mut display: *mut cairo_xlib_display_t = (*surface).display;
    let mut pa: XRenderPictureAttributes = XRenderPictureAttributes {
        repeat: 0,
        alpha_map: 0,
        alpha_x_origin: 0,
        alpha_y_origin: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        graphics_exposures: 0,
        subwindow_mode: 0,
        poly_edge: 0,
        poly_mode: 0,
        dither: 0,
        component_alpha: 0,
    };
    let mut mask: libc::c_int = 0 as libc::c_int;
    if (*surface).picture != 0 {
        return;
    }
    if (*display).force_precision != -(1 as libc::c_int) {
        pa.poly_mode = (*display).force_precision;
    } else {
        pa.poly_mode = 1 as libc::c_int;
    }
    if pa.poly_mode != 0 {
        mask |= (1 as libc::c_int) << 10 as libc::c_int;
    }
    (*surface).precision = pa.poly_mode;
    (*surface)
        .picture = XRenderCreatePicture(
        (*display).display,
        (*surface).drawable,
        (*surface).xrender_format,
        mask as libc::c_ulong,
        &mut pa,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_draw_image(
    mut surface: *mut cairo_xlib_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut ximage: XImage = XImage {
        width: 0,
        height: 0,
        xoffset: 0,
        format: 0,
        data: 0 as *mut libc::c_char,
        byte_order: 0,
        bitmap_unit: 0,
        bitmap_bit_order: 0,
        bitmap_pad: 0,
        depth: 0,
        bytes_per_line: 0,
        bits_per_pixel: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
        obdata: 0 as *mut libc::c_char,
        f: funcs {
            create_image: None,
            destroy_image: None,
            get_pixel: None,
            put_pixel: None,
            sub_image: None,
            add_pixel: None,
        },
    };
    let mut image_masks: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    let mut native_byte_order: libc::c_int = if _cairo_is_little_endian() != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut shm_image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut pixman_image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut own_data: cairo_bool_t = 0 as libc::c_int;
    let mut is_rgb_image: cairo_bool_t = 0;
    let mut gc: GC = 0 as *mut _XGC;
    ximage.width = (*image).width;
    ximage.height = (*image).height;
    ximage.format = 2 as libc::c_int;
    ximage.byte_order = native_byte_order;
    ximage.bitmap_unit = 32 as libc::c_int;
    ximage.bitmap_bit_order = native_byte_order;
    ximage.bitmap_pad = 32 as libc::c_int;
    ximage.depth = (*surface).depth;
    ximage.red_mask = (*surface).r_mask as libc::c_ulong;
    ximage.green_mask = (*surface).g_mask as libc::c_ulong;
    ximage.blue_mask = (*surface).b_mask as libc::c_ulong;
    ximage.xoffset = 0 as libc::c_int;
    ximage.obdata = 0 as XPointer;
    status = _cairo_xlib_display_acquire((*surface).base.device, &mut display);
    if status as u64 != 0 {
        return status;
    }
    is_rgb_image = _pixman_format_to_masks((*image).pixman_format, &mut image_masks);
    if is_rgb_image != 0
        && (image_masks.alpha_mask == (*surface).a_mask as libc::c_ulong
            || (*surface).a_mask == 0 as libc::c_int as libc::c_uint)
        && (image_masks.red_mask == (*surface).r_mask as libc::c_ulong
            || (*surface).r_mask == 0 as libc::c_int as libc::c_uint)
        && (image_masks.green_mask == (*surface).g_mask as libc::c_ulong
            || (*surface).g_mask == 0 as libc::c_int as libc::c_uint)
        && (image_masks.blue_mask == (*surface).b_mask as libc::c_ulong
            || (*surface).b_mask == 0 as libc::c_int as libc::c_uint)
    {
        let mut ret: libc::c_int = 0;
        ximage.bits_per_pixel = image_masks.bpp;
        ximage.bytes_per_line = (*image).stride as libc::c_int;
        ximage.data = (*image).data as *mut libc::c_char;
        if (*image).base.device != (*surface).base.device {
            let mut max_request_size: libc::c_int = XExtendedMaxRequestSize(
                (*display).display,
            ) as libc::c_int;
            if max_request_size == 0 as libc::c_int {
                max_request_size = XMaxRequestSize((*display).display) as libc::c_int;
            }
            if max_request_size > 8192 as libc::c_int {
                max_request_size = 8192 as libc::c_int;
            }
            if width * height * 4 as libc::c_int > max_request_size {
                shm_image = _cairo_xlib_surface_create_shm__image(
                    surface,
                    (*image).pixman_format,
                    width,
                    height,
                );
                if !shm_image.is_null()
                    && (*shm_image).status as libc::c_uint
                        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    let mut clone: *mut cairo_image_surface_t = shm_image
                        as *mut cairo_image_surface_t;
                    pixman_image_composite32(
                        PIXMAN_OP_SRC,
                        (*image).pixman_image,
                        0 as *mut pixman_image_t,
                        (*clone).pixman_image,
                        src_x,
                        src_y,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        width,
                        height,
                    );
                    ximage
                        .obdata = _cairo_xlib_shm_surface_get_obdata(shm_image)
                        as XPointer;
                    ximage.data = (*clone).data as *mut libc::c_char;
                    ximage.bytes_per_line = (*clone).stride as libc::c_int;
                    ximage.width = width;
                    ximage.height = height;
                    src_y = 0 as libc::c_int;
                    src_x = src_y;
                }
            }
        } else {
            ximage
                .obdata = _cairo_xlib_shm_surface_get_obdata(&mut (*image).base)
                as XPointer;
        }
        ret = XInitImage(&mut ximage);
        if ret != 0 as libc::c_int {} else {
            __assert_fail(
                b"ret != 0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"cairo_status_t _cairo_xlib_surface_draw_image(cairo_xlib_surface_t *, cairo_image_surface_t *, int, int, int, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        current_block = 7244994750255146185;
    } else if ((*surface).visual).is_null()
        || (*(*surface).visual).class == 4 as libc::c_int
    {
        let mut intermediate_format: pixman_format_code_t = 0 as pixman_format_code_t;
        let mut ret_0: libc::c_int = 0;
        image_masks.alpha_mask = (*surface).a_mask as libc::c_ulong;
        image_masks.red_mask = (*surface).r_mask as libc::c_ulong;
        image_masks.green_mask = (*surface).g_mask as libc::c_ulong;
        image_masks.blue_mask = (*surface).b_mask as libc::c_ulong;
        image_masks.bpp = bits_per_pixel(surface);
        ret_0 = _pixman_format_from_masks(&mut image_masks, &mut intermediate_format);
        if ret_0 != 0 {} else {
            __assert_fail(
                b"ret\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1176 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"cairo_status_t _cairo_xlib_surface_draw_image(cairo_xlib_surface_t *, cairo_image_surface_t *, int, int, int, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        shm_image = _cairo_xlib_surface_create_shm__image(
            surface,
            intermediate_format,
            width,
            height,
        );
        if !shm_image.is_null()
            && (*shm_image).status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            let mut clone_0: *mut cairo_image_surface_t = shm_image
                as *mut cairo_image_surface_t;
            pixman_image_composite32(
                PIXMAN_OP_SRC,
                (*image).pixman_image,
                0 as *mut pixman_image_t,
                (*clone_0).pixman_image,
                src_x,
                src_y,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                width,
                height,
            );
            ximage.data = (*clone_0).data as *mut libc::c_char;
            ximage
                .obdata = _cairo_xlib_shm_surface_get_obdata(&mut (*clone_0).base)
                as XPointer;
            ximage.bytes_per_line = (*clone_0).stride as libc::c_int;
            current_block = 13125627826496529465;
        } else {
            pixman_image = pixman_image_create_bits(
                intermediate_format,
                width,
                height,
                0 as *mut uint32_t,
                0 as libc::c_int,
            );
            if pixman_image.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                current_block = 16361256604139641254;
            } else {
                pixman_image_composite32(
                    PIXMAN_OP_SRC,
                    (*image).pixman_image,
                    0 as *mut pixman_image_t,
                    pixman_image,
                    src_x,
                    src_y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    width,
                    height,
                );
                ximage.data = pixman_image_get_data(pixman_image) as *mut libc::c_char;
                ximage.bytes_per_line = pixman_image_get_stride(pixman_image);
                current_block = 13125627826496529465;
            }
        }
        match current_block {
            16361256604139641254 => {}
            _ => {
                ximage.width = width;
                ximage.height = height;
                ximage.bits_per_pixel = image_masks.bpp;
                ret_0 = XInitImage(&mut ximage);
                if ret_0 != 0 as libc::c_int {} else {
                    __assert_fail(
                        b"ret != 0\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-xlib-surface.c\0" as *const u8
                            as *const libc::c_char,
                        1222 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 125],
                            &[libc::c_char; 125],
                        >(
                            b"cairo_status_t _cairo_xlib_surface_draw_image(cairo_xlib_surface_t *, cairo_image_surface_t *, int, int, int, int, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                src_y = 0 as libc::c_int;
                src_x = src_y;
                current_block = 7244994750255146185;
            }
        }
    } else {
        let mut stride: libc::c_uint = 0;
        let mut rowstride: libc::c_uint = 0;
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut x0: libc::c_int = 0;
        let mut y0: libc::c_int = 0;
        let mut x_off: libc::c_int = 0;
        let mut y_off: libc::c_int = 0;
        let mut in_pixel: uint32_t = 0;
        let mut out_pixel: uint32_t = 0;
        let mut row: *mut uint32_t = 0 as *mut uint32_t;
        let mut i_a_width: libc::c_int = 0 as libc::c_int;
        let mut i_r_width: libc::c_int = 0 as libc::c_int;
        let mut i_g_width: libc::c_int = 0 as libc::c_int;
        let mut i_b_width: libc::c_int = 0 as libc::c_int;
        let mut i_a_shift: libc::c_int = 0 as libc::c_int;
        let mut i_r_shift: libc::c_int = 0 as libc::c_int;
        let mut i_g_shift: libc::c_int = 0 as libc::c_int;
        let mut i_b_shift: libc::c_int = 0 as libc::c_int;
        let mut o_a_width: libc::c_int = 0 as libc::c_int;
        let mut o_r_width: libc::c_int = 0 as libc::c_int;
        let mut o_g_width: libc::c_int = 0 as libc::c_int;
        let mut o_b_width: libc::c_int = 0 as libc::c_int;
        let mut o_a_shift: libc::c_int = 0 as libc::c_int;
        let mut o_r_shift: libc::c_int = 0 as libc::c_int;
        let mut o_g_shift: libc::c_int = 0 as libc::c_int;
        let mut o_b_shift: libc::c_int = 0 as libc::c_int;
        let mut visual_info: *mut cairo_xlib_visual_info_t = 0
            as *mut cairo_xlib_visual_info_t;
        let mut true_color: cairo_bool_t = 0;
        let mut ret_1: libc::c_int = 0;
        ximage.bits_per_pixel = bits_per_pixel(surface);
        stride = ((((ximage.bits_per_pixel * ximage.width + 7 as libc::c_int)
            / 8 as libc::c_int) as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
            as libc::c_uint;
        ximage.bytes_per_line = stride as libc::c_int;
        ximage
            .data = _cairo_malloc_ab(stride as size_t, ximage.height as size_t)
            as *mut libc::c_char;
        if (ximage.data).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            current_block = 16361256604139641254;
        } else {
            own_data = 1 as libc::c_int;
            ret_1 = XInitImage(&mut ximage);
            if ret_1 != 0 as libc::c_int {} else {
                __assert_fail(
                    b"ret != 0\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                    1252 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 125],
                        &[libc::c_char; 125],
                    >(
                        b"cairo_status_t _cairo_xlib_surface_draw_image(cairo_xlib_surface_t *, cairo_image_surface_t *, int, int, int, int, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            _characterize_field(
                image_masks.alpha_mask as uint32_t,
                &mut i_a_width,
                &mut i_a_shift,
            );
            _characterize_field(
                image_masks.red_mask as uint32_t,
                &mut i_r_width,
                &mut i_r_shift,
            );
            _characterize_field(
                image_masks.green_mask as uint32_t,
                &mut i_g_width,
                &mut i_g_shift,
            );
            _characterize_field(
                image_masks.blue_mask as uint32_t,
                &mut i_b_width,
                &mut i_b_shift,
            );
            true_color = (((*surface).visual).is_null()
                || (*(*surface).visual).class == 4 as libc::c_int) as libc::c_int;
            if true_color != 0 {
                _characterize_field((*surface).a_mask, &mut o_a_width, &mut o_a_shift);
                _characterize_field((*surface).r_mask, &mut o_r_width, &mut o_r_shift);
                _characterize_field((*surface).g_mask, &mut o_g_width, &mut o_g_shift);
                _characterize_field((*surface).b_mask, &mut o_b_width, &mut o_b_shift);
                current_block = 2467484839200770573;
            } else {
                status = _cairo_xlib_screen_get_visual_info(
                    display,
                    (*surface).screen,
                    (*surface).visual,
                    &mut visual_info,
                );
                if status as u64 != 0 {
                    current_block = 16361256604139641254;
                } else {
                    current_block = 2467484839200770573;
                }
            }
            match current_block {
                16361256604139641254 => {}
                _ => {
                    rowstride = ((*image).stride >> 2 as libc::c_int) as libc::c_uint;
                    row = (*image).data as *mut uint32_t;
                    x0 = (dst_x as libc::c_double + (*surface).base.device_transform.x0)
                        as libc::c_int;
                    y0 = (dst_y as libc::c_double + (*surface).base.device_transform.y0)
                        as libc::c_int;
                    y = 0 as libc::c_int;
                    y_off = y0
                        % (::std::mem::size_of::<[[int8_t; 4]; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong,
                            ) as libc::c_int;
                    while y < ximage.height {
                        let mut dither_row: *const int8_t = (dither_pattern[y_off
                            as usize])
                            .as_ptr();
                        x = 0 as libc::c_int;
                        x_off = x0
                            % (::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                                ) as libc::c_int;
                        while x < ximage.width {
                            let mut dither_adjustment: libc::c_int = *dither_row
                                .offset(x_off as isize) as libc::c_int;
                            let mut a: libc::c_int = 0;
                            let mut r: libc::c_int = 0;
                            let mut g: libc::c_int = 0;
                            let mut b: libc::c_int = 0;
                            if image_masks.bpp == 1 as libc::c_int {
                                in_pixel = (*(row as *mut uint8_t)
                                    .offset((x / 8 as libc::c_int) as isize) as libc::c_int
                                    & (1 as libc::c_int) << (x & 7 as libc::c_int) != 0)
                                    as libc::c_int as uint32_t;
                            } else if image_masks.bpp <= 8 as libc::c_int {
                                in_pixel = *(row as *mut uint8_t).offset(x as isize)
                                    as uint32_t;
                            } else if image_masks.bpp <= 16 as libc::c_int {
                                in_pixel = *(row as *mut uint16_t).offset(x as isize)
                                    as uint32_t;
                            } else if image_masks.bpp <= 24 as libc::c_int {
                                in_pixel = (*(row as *mut uint8_t)
                                    .offset((3 as libc::c_int * x) as isize) as libc::c_int
                                    | (*(row as *mut uint8_t)
                                        .offset((3 as libc::c_int * x + 1 as libc::c_int) as isize)
                                        as libc::c_int) << 8 as libc::c_int
                                    | (*(row as *mut uint8_t)
                                        .offset((3 as libc::c_int * x + 2 as libc::c_int) as isize)
                                        as libc::c_int) << 16 as libc::c_int) as uint32_t;
                            } else {
                                in_pixel = *row.offset(x as isize);
                            }
                            if image_masks.alpha_mask
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                a = 0xff as libc::c_int;
                            } else {
                                a = _field_to_8(
                                    (in_pixel as libc::c_ulong & image_masks.alpha_mask)
                                        as uint32_t,
                                    i_a_width,
                                    i_a_shift,
                                ) as libc::c_int;
                            }
                            r = _field_to_8(
                                (in_pixel as libc::c_ulong & image_masks.red_mask)
                                    as uint32_t,
                                i_r_width,
                                i_r_shift,
                            ) as libc::c_int;
                            g = _field_to_8(
                                (in_pixel as libc::c_ulong & image_masks.green_mask)
                                    as uint32_t,
                                i_g_width,
                                i_g_shift,
                            ) as libc::c_int;
                            b = _field_to_8(
                                (in_pixel as libc::c_ulong & image_masks.blue_mask)
                                    as uint32_t,
                                i_b_width,
                                i_b_shift,
                            ) as libc::c_int;
                            if true_color != 0 {
                                out_pixel = _field_from_8(
                                    a as uint32_t,
                                    o_a_width,
                                    o_a_shift,
                                )
                                    | _field_from_8_dither(
                                        r as uint32_t,
                                        o_r_width,
                                        o_r_shift,
                                        dither_adjustment as int8_t,
                                    )
                                    | _field_from_8_dither(
                                        g as uint32_t,
                                        o_g_width,
                                        o_g_shift,
                                        dither_adjustment as int8_t,
                                    )
                                    | _field_from_8_dither(
                                        b as uint32_t,
                                        o_b_width,
                                        o_b_shift,
                                        dither_adjustment as int8_t,
                                    );
                            } else {
                                out_pixel = _pseudocolor_from_rgb888_dither(
                                    visual_info,
                                    r as uint32_t,
                                    g as uint32_t,
                                    b as uint32_t,
                                    dither_adjustment as int8_t,
                                );
                            }
                            (Some(
                                (ximage.f.put_pixel).expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(&mut ximage, x, y, out_pixel as libc::c_ulong);
                            x += 1;
                            x_off = (x_off + 1 as libc::c_int)
                                % (::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                                    ) as libc::c_int;
                        }
                        row = row.offset(rowstride as isize);
                        y += 1;
                        y_off = (y_off + 1 as libc::c_int)
                            % (::std::mem::size_of::<[[int8_t; 4]; 4]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<[int8_t; 4]>() as libc::c_ulong,
                                ) as libc::c_int;
                    }
                    current_block = 7244994750255146185;
                }
            }
        }
    }
    match current_block {
        7244994750255146185 => {
            status = _cairo_xlib_surface_get_gc(display, surface, &mut gc);
            if !(status as u64 != 0) {
                if !(ximage.obdata).is_null() {
                    XShmPutImage(
                        (*display).display,
                        (*surface).drawable,
                        gc,
                        &mut ximage,
                        src_x,
                        src_y,
                        dst_x,
                        dst_y,
                        width as libc::c_uint,
                        height as libc::c_uint,
                        1 as libc::c_int,
                    );
                } else {
                    XPutImage(
                        (*display).display,
                        (*surface).drawable,
                        gc,
                        &mut ximage,
                        src_x,
                        src_y,
                        dst_x,
                        dst_y,
                        width as libc::c_uint,
                        height as libc::c_uint,
                    );
                }
                _cairo_xlib_surface_put_gc(display, surface, gc);
            }
        }
        _ => {}
    }
    cairo_device_release(&mut (*display).base);
    if own_data != 0 {
        free(ximage.data as *mut libc::c_void);
    }
    if !shm_image.is_null() {
        cairo_surface_destroy(shm_image);
    }
    if !pixman_image.is_null() {
        pixman_image_unref(pixman_image);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xlib_surface_source(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if !extents.is_null() {
        let ref mut fresh14 = (*extents).y;
        *fresh14 = 0 as libc::c_int;
        (*extents).x = *fresh14;
        (*extents).width = (*surface).width;
        (*extents).height = (*surface).height;
    }
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_xlib_surface_acquire_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image_out: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    *image_extra = 0 as *mut libc::c_void;
    *image_out = _cairo_xlib_surface_get_shm(
        abstract_surface as *mut cairo_xlib_surface_t,
        0 as libc::c_int,
    ) as *mut cairo_image_surface_t;
    if !(*image_out).is_null() {
        return (**image_out).base.status;
    }
    extents.y = 0 as libc::c_int;
    extents.x = extents.y;
    extents.width = (*surface).width;
    extents.height = (*surface).height;
    *image_out = _get_image_surface(surface, &mut extents, 1 as libc::c_int)
        as *mut cairo_image_surface_t;
    return (**image_out).base.status;
}
unsafe extern "C" fn _cairo_xlib_surface_snapshot(
    mut abstract_surface: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    extents.y = 0 as libc::c_int;
    extents.x = extents.y;
    extents.width = (*surface).width;
    extents.height = (*surface).height;
    return _get_image_surface(surface, &mut extents, 0 as libc::c_int);
}
unsafe extern "C" fn _cairo_xlib_surface_release_source_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if &mut (*image).base as *mut cairo_surface_t == (*surface).shm {
        return;
    }
    cairo_surface_destroy(&mut (*image).base);
}
unsafe extern "C" fn _cairo_xlib_surface_map_to_image(
    mut abstract_surface: *mut libc::c_void,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_image_surface_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    image = _cairo_xlib_surface_get_shm(
        abstract_surface as *mut cairo_xlib_surface_t,
        0 as libc::c_int,
    );
    if !image.is_null() {
        if !((*surface).base.damage).is_null() {} else {
            __assert_fail(
                b"surface->base.damage\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1438 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"cairo_image_surface_t *_cairo_xlib_surface_map_to_image(void *, const cairo_rectangle_int_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh15 = (*surface).fallback;
        *fresh15 += 1;
        return _cairo_image_surface_map_to_image(image as *mut libc::c_void, extents);
    }
    image = _get_image_surface(
        abstract_surface as *mut cairo_xlib_surface_t,
        extents,
        1 as libc::c_int,
    );
    cairo_surface_set_device_offset(
        image,
        -(*extents).x as libc::c_double,
        -(*extents).y as libc::c_double,
    );
    return image as *mut cairo_image_surface_t;
}
unsafe extern "C" fn _cairo_xlib_surface_unmap_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if !((*surface).shm).is_null() {
        let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        if (*surface).fallback != 0 {} else {
            __assert_fail(
                b"surface->fallback\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1459 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"cairo_int_status_t _cairo_xlib_surface_unmap_image(void *, cairo_image_surface_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        if !((*surface).base.damage).is_null() {} else {
            __assert_fail(
                b"surface->base.damage\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1460 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"cairo_int_status_t _cairo_xlib_surface_unmap_image(void *, cairo_image_surface_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        r.x = (*image).base.device_transform_inverse.x0 as libc::c_int;
        r.y = (*image).base.device_transform_inverse.y0 as libc::c_int;
        r.width = (*image).width;
        r.height = (*image).height;
        let ref mut fresh16 = (*(*surface).shm).damage;
        *fresh16 = _cairo_damage_add_rectangle((*(*surface).shm).damage, &mut r);
        return _cairo_image_surface_unmap_image(
            (*surface).shm as *mut libc::c_void,
            image,
        );
    }
    status = _cairo_xlib_surface_draw_image(
        abstract_surface as *mut cairo_xlib_surface_t,
        image,
        0 as libc::c_int,
        0 as libc::c_int,
        (*image).width,
        (*image).height,
        (*image).base.device_transform_inverse.x0 as libc::c_int,
        (*image).base.device_transform_inverse.y0 as libc::c_int,
    ) as cairo_int_status_t;
    cairo_surface_finish(&mut (*image).base);
    cairo_surface_destroy(&mut (*image).base);
    return status;
}
unsafe extern "C" fn _cairo_xlib_surface_flush(
    mut abstract_surface: *mut libc::c_void,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if flags != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_xlib_surface_put_shm(surface);
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    (*surface).fallback >>= 1 as libc::c_int;
    if !((*surface).shm).is_null()
        && _cairo_xlib_shm_surface_is_idle((*surface).shm) != 0
    {
        _cairo_xlib_surface_discard_shm(surface);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xlib_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    (*rectangle).x = 0 as libc::c_int;
    (*rectangle).y = 0 as libc::c_int;
    (*rectangle).width = (*surface).width;
    (*rectangle).height = (*surface).height;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_xlib_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    *options = *_cairo_xlib_screen_get_font_options((*surface).screen);
}
#[inline]
unsafe extern "C" fn get_compositor(
    mut surface: *mut *mut cairo_xlib_surface_t,
    mut compositor: *mut *const cairo_compositor_t,
) -> cairo_int_status_t {
    let mut s: *mut cairo_xlib_surface_t = *surface;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*s).fallback != 0 {
        if !((*s).base.damage).is_null() {} else {
            __assert_fail(
                b"s->base.damage != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1540 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"cairo_int_status_t get_compositor(cairo_xlib_surface_t **, const cairo_compositor_t **)\0",
                ))
                    .as_ptr(),
            );
        }
        if !((*s).shm).is_null() {} else {
            __assert_fail(
                b"s->shm != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1541 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"cairo_int_status_t get_compositor(cairo_xlib_surface_t **, const cairo_compositor_t **)\0",
                ))
                    .as_ptr(),
            );
        }
        if !((*(*s).shm).damage).is_null() {} else {
            __assert_fail(
                b"s->shm->damage != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface.c\0" as *const u8 as *const libc::c_char,
                1542 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"cairo_int_status_t get_compositor(cairo_xlib_surface_t **, const cairo_compositor_t **)\0",
                ))
                    .as_ptr(),
            );
        }
        if _cairo_xlib_shm_surface_is_active((*s).shm) == 0 {
            *surface = (*s).shm as *mut cairo_xlib_surface_t;
            *compositor = (*((*s).shm as *mut cairo_image_surface_t)).compositor;
            let ref mut fresh17 = (*s).fallback;
            *fresh17 += 1;
        } else {
            status = _cairo_xlib_surface_put_shm(s);
            (*s).fallback = 0 as libc::c_int;
            *compositor = (*s).compositor;
        }
    } else {
        *compositor = (*s).compositor;
    }
    return status;
}
unsafe extern "C" fn _cairo_xlib_surface_paint(
    mut _surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = _surface as *mut cairo_xlib_surface_t;
    let mut compositor: *const cairo_compositor_t = 0 as *const cairo_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = get_compositor(&mut surface, &mut compositor);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_compositor_paint(compositor, &mut (*surface).base, op, source, clip);
}
unsafe extern "C" fn _cairo_xlib_surface_mask(
    mut _surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = _surface as *mut cairo_xlib_surface_t;
    let mut compositor: *const cairo_compositor_t = 0 as *const cairo_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = get_compositor(&mut surface, &mut compositor);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_compositor_mask(
        compositor,
        &mut (*surface).base,
        op,
        source,
        mask,
        clip,
    );
}
unsafe extern "C" fn _cairo_xlib_surface_stroke(
    mut _surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = _surface as *mut cairo_xlib_surface_t;
    let mut compositor: *const cairo_compositor_t = 0 as *const cairo_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = get_compositor(&mut surface, &mut compositor);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_compositor_stroke(
        compositor,
        &mut (*surface).base,
        op,
        source,
        path,
        style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        clip,
    );
}
unsafe extern "C" fn _cairo_xlib_surface_fill(
    mut _surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = _surface as *mut cairo_xlib_surface_t;
    let mut compositor: *const cairo_compositor_t = 0 as *const cairo_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = get_compositor(&mut surface, &mut compositor);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_compositor_fill(
        compositor,
        &mut (*surface).base,
        op,
        source,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    );
}
unsafe extern "C" fn _cairo_xlib_surface_glyphs(
    mut _surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = _surface as *mut cairo_xlib_surface_t;
    let mut compositor: *const cairo_compositor_t = 0 as *const cairo_compositor_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = get_compositor(&mut surface, &mut compositor);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_compositor_glyphs(
        compositor,
        &mut (*surface).base,
        op,
        source,
        glyphs,
        num_glyphs,
        scaled_font,
        clip,
    );
}
static mut cairo_xlib_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_XLIB,
            finish: Some(
                _cairo_xlib_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_xlib_surface_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: Some(
                _cairo_xlib_surface_create_similar_shm
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_format_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            map_to_image: Some(
                _cairo_xlib_surface_map_to_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_rectangle_int_t,
                    ) -> *mut cairo_image_surface_t,
            ),
            unmap_image: Some(
                _cairo_xlib_surface_unmap_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
            source: Some(
                _cairo_xlib_surface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_xlib_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_xlib_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_xlib_surface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: None,
            show_page: None,
            get_extents: Some(
                _cairo_xlib_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_xlib_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: Some(
                _cairo_xlib_surface_flush
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> cairo_status_t,
            ),
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_xlib_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_xlib_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_xlib_surface_stroke
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
                _cairo_xlib_surface_fill
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
            fill_stroke: None,
            show_glyphs: Some(
                _cairo_xlib_surface_glyphs
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
            get_supported_mime_types: None,
            tag: None,
        };
        init
    }
};
unsafe extern "C" fn _cairo_surface_is_xlib(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*surface).backend
        == &cairo_xlib_surface_backend as *const cairo_surface_backend_t) as libc::c_int;
}
unsafe extern "C" fn _cairo_xlib_surface_create_internal(
    mut screen: *mut cairo_xlib_screen_t,
    mut drawable: Drawable,
    mut visual: *mut Visual,
    mut xrender_format: *mut XRenderPictFormat,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut depth: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if depth == 0 as libc::c_int {
        let mut current_block_10: u64;
        if !xrender_format.is_null() {
            depth = (*xrender_format).depth;
            current_block_10 = 12147880666119273379;
        } else if !visual.is_null() {
            let mut scr: *mut Screen = (*screen).screen;
            if visual == (*scr).root_visual {
                depth = (*scr).root_depth;
                current_block_10 = 12147880666119273379;
            } else {
                let mut j: libc::c_int = 0;
                let mut k: libc::c_int = 0;
                depth = 0 as libc::c_int;
                j = 0 as libc::c_int;
                's_54: loop {
                    if !(j < (*scr).ndepths) {
                        current_block_10 = 12147880666119273379;
                        break;
                    }
                    let mut d: *mut Depth = &mut *((*scr).depths).offset(j as isize)
                        as *mut Depth;
                    k = 0 as libc::c_int;
                    while k < (*d).nvisuals {
                        if &mut *((*d).visuals).offset(k as isize) as *mut Visual
                            == visual
                        {
                            depth = (*d).depth;
                            current_block_10 = 10652014663920648156;
                            break 's_54;
                        } else {
                            k += 1;
                        }
                    }
                    j += 1;
                }
            }
        } else {
            current_block_10 = 12147880666119273379;
        }
        match current_block_10 {
            12147880666119273379 => {
                if depth == 0 as libc::c_int {
                    return _cairo_surface_create_in_error(
                        _cairo_error(CAIRO_STATUS_INVALID_VISUAL),
                    );
                }
            }
            _ => {}
        }
    }
    surface = (if ::std::mem::size_of::<cairo_xlib_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_surface_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    status = _cairo_xlib_display_acquire((*screen).device, &mut display);
    if status as u64 != 0 {
        free(surface as *mut libc::c_void);
        return _cairo_surface_create_in_error(_cairo_error(status));
    }
    let ref mut fresh18 = (*surface).display;
    *fresh18 = display;
    if (*display).render_major > 0 as libc::c_int
        || (*display).render_major == 0 as libc::c_int
            && (*display).render_minor >= 0 as libc::c_int
    {
        if xrender_format.is_null() {
            if !visual.is_null() {
                xrender_format = XRenderFindVisualFormat((*display).display, visual);
            } else if depth == 1 as libc::c_int {
                xrender_format = _cairo_xlib_display_get_xrender_format(
                    display,
                    CAIRO_FORMAT_A1,
                );
            }
        }
    }
    cairo_device_release(&mut (*display).base);
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_xlib_surface_backend,
        (*screen).device,
        _xrender_format_to_content(xrender_format),
        0 as libc::c_int,
    );
    let ref mut fresh19 = (*surface).screen;
    *fresh19 = screen;
    let ref mut fresh20 = (*surface).compositor;
    *fresh20 = (*display).compositor;
    let ref mut fresh21 = (*surface).shm;
    *fresh21 = 0 as *mut cairo_surface_t;
    (*surface).fallback = 0 as libc::c_int;
    (*surface).drawable = drawable;
    (*surface).owns_pixmap = 0 as libc::c_int;
    (*surface).use_pixmap = 0 as libc::c_int;
    (*surface).width = width;
    (*surface).height = height;
    (*surface).picture = 0 as libc::c_long as Picture;
    (*surface).precision = 0 as libc::c_int;
    (*surface).embedded_source.picture = 0 as libc::c_long as Picture;
    let ref mut fresh22 = (*surface).visual;
    *fresh22 = visual;
    let ref mut fresh23 = (*surface).xrender_format;
    *fresh23 = xrender_format;
    (*surface).depth = depth;
    if !xrender_format.is_null() {
        (*surface)
            .a_mask = (((*(*surface).xrender_format).direct.alphaMask as libc::c_ulong)
            << (*(*surface).xrender_format).direct.alpha as libc::c_int) as uint32_t;
        (*surface)
            .r_mask = (((*(*surface).xrender_format).direct.redMask as libc::c_ulong)
            << (*(*surface).xrender_format).direct.red as libc::c_int) as uint32_t;
        (*surface)
            .g_mask = (((*(*surface).xrender_format).direct.greenMask as libc::c_ulong)
            << (*(*surface).xrender_format).direct.green as libc::c_int) as uint32_t;
        (*surface)
            .b_mask = (((*(*surface).xrender_format).direct.blueMask as libc::c_ulong)
            << (*(*surface).xrender_format).direct.blue as libc::c_int) as uint32_t;
    } else if !visual.is_null() {
        (*surface).a_mask = 0 as libc::c_int as uint32_t;
        (*surface).r_mask = (*visual).red_mask as uint32_t;
        (*surface).g_mask = (*visual).green_mask as uint32_t;
        (*surface).b_mask = (*visual).blue_mask as uint32_t;
    } else {
        if depth < 32 as libc::c_int {
            (*surface)
                .a_mask = (((1 as libc::c_int) << depth) - 1 as libc::c_int) as uint32_t;
        } else {
            (*surface).a_mask = 0xffffffff as libc::c_uint;
        }
        (*surface).r_mask = 0 as libc::c_int as uint32_t;
        (*surface).g_mask = 0 as libc::c_int as uint32_t;
        (*surface).b_mask = 0 as libc::c_int as uint32_t;
    }
    cairo_list_add(&mut (*surface).link, &mut (*screen).surfaces);
    return &mut (*surface).base;
}
unsafe extern "C" fn _cairo_xlib_screen_from_visual(
    mut dpy: *mut Display,
    mut visual: *mut Visual,
) -> *mut Screen {
    let mut s: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < (*(dpy as _XPrivDisplay)).nscreens {
        let mut screen: *mut Screen = 0 as *mut Screen;
        screen = &mut *((*(dpy as _XPrivDisplay)).screens).offset(s as isize)
            as *mut Screen;
        if visual == (*screen).root_visual {
            return screen;
        }
        d = 0 as libc::c_int;
        while d < (*screen).ndepths {
            let mut depth: *mut Depth = 0 as *mut Depth;
            depth = &mut *((*screen).depths).offset(d as isize) as *mut Depth;
            v = 0 as libc::c_int;
            while v < (*depth).nvisuals {
                if visual == &mut *((*depth).visuals).offset(v as isize) as *mut Visual {
                    return screen;
                }
                v += 1;
            }
            d += 1;
        }
        s += 1;
    }
    return 0 as *mut Screen;
}
unsafe extern "C" fn valid_size(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> cairo_bool_t {
    return (width >= 0 as libc::c_int && width <= 32767 as libc::c_int
        && height >= 0 as libc::c_int && height <= 32767 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_create(
    mut dpy: *mut Display,
    mut drawable: Drawable,
    mut visual: *mut Visual,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut scr: *mut Screen = 0 as *mut Screen;
    let mut screen: *mut cairo_xlib_screen_t = 0 as *mut cairo_xlib_screen_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if valid_size(width, height) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    scr = _cairo_xlib_screen_from_visual(dpy, visual);
    if scr.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_VISUAL));
    }
    status = _cairo_xlib_screen_get(dpy, scr, &mut screen);
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    return _cairo_xlib_surface_create_internal(
        screen,
        drawable,
        visual,
        0 as *mut XRenderPictFormat,
        width,
        height,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_create_for_bitmap(
    mut dpy: *mut Display,
    mut bitmap: Pixmap,
    mut scr: *mut Screen,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut screen: *mut cairo_xlib_screen_t = 0 as *mut cairo_xlib_screen_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if valid_size(width, height) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    status = _cairo_xlib_screen_get(dpy, scr, &mut screen);
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    return _cairo_xlib_surface_create_internal(
        screen,
        bitmap,
        0 as *mut Visual,
        0 as *mut XRenderPictFormat,
        width,
        height,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_create_with_xrender_format(
    mut dpy: *mut Display,
    mut drawable: Drawable,
    mut scr: *mut Screen,
    mut format: *mut XRenderPictFormat,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut screen: *mut cairo_xlib_screen_t = 0 as *mut cairo_xlib_screen_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if valid_size(width, height) == 0 {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_INVALID_SIZE));
    }
    status = _cairo_xlib_screen_get(dpy, scr, &mut screen);
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status);
    }
    return _cairo_xlib_surface_create_internal(
        screen,
        drawable,
        _visual_for_xrender_format(scr, format),
        format,
        width,
        height,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_xrender_format(
    mut surface: *mut cairo_surface_t,
) -> *mut XRenderPictFormat {
    let mut xlib_surface: *mut cairo_xlib_surface_t = surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as *mut XRenderPictFormat;
    }
    return (*xlib_surface).xrender_format;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_set_size(
    mut abstract_surface: *mut cairo_surface_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*abstract_surface).status as u64 != 0 {
        return;
    }
    if (*abstract_surface).finished() != 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        );
        return;
    }
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        );
        return;
    }
    if (*surface).width == width && (*surface).height == height {
        return;
    }
    if valid_size(width, height) == 0 {
        _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_INVALID_SIZE) as cairo_int_status_t,
        );
        return;
    }
    status = _cairo_surface_flush(abstract_surface, 0 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        _cairo_surface_set_error(abstract_surface, status as cairo_int_status_t);
        return;
    }
    _cairo_xlib_surface_discard_shm(surface);
    (*surface).width = width;
    (*surface).height = height;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_set_drawable(
    mut abstract_surface: *mut cairo_surface_t,
    mut drawable: Drawable,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*abstract_surface).status as u64 != 0 {
        return;
    }
    if (*abstract_surface).finished() != 0 {
        status = _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
        return;
    }
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        status = _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        ) as cairo_status_t;
        return;
    }
    if valid_size(width, height) == 0 {
        status = _cairo_surface_set_error(
            abstract_surface,
            _cairo_error(CAIRO_STATUS_INVALID_SIZE) as cairo_int_status_t,
        ) as cairo_status_t;
        return;
    }
    if (*surface).owns_pixmap != 0 {
        return;
    }
    status = _cairo_surface_flush(abstract_surface, 0 as libc::c_int as libc::c_uint);
    if status as u64 != 0 {
        _cairo_surface_set_error(abstract_surface, status as cairo_int_status_t);
        return;
    }
    if (*surface).drawable != drawable {
        let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
        status = _cairo_xlib_display_acquire((*surface).base.device, &mut display);
        if status as u64 != 0 {
            return;
        }
        if (*surface).picture != 0 as libc::c_long as libc::c_ulong {
            XRenderFreePicture((*display).display, (*surface).picture);
            if status as u64 != 0 {
                status = _cairo_surface_set_error(
                    &mut (*surface).base,
                    status as cairo_int_status_t,
                ) as cairo_status_t;
                return;
            }
            (*surface).picture = 0 as libc::c_long as Picture;
        }
        cairo_device_release(&mut (*display).base);
        (*surface).drawable = drawable;
    }
    if (*surface).width != width || (*surface).height != height {
        _cairo_xlib_surface_discard_shm(surface);
        (*surface).width = width;
        (*surface).height = height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_display(
    mut abstract_surface: *mut cairo_surface_t,
) -> *mut Display {
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as *mut Display;
    }
    return (*((*abstract_surface).device as *mut cairo_xlib_display_t)).display;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_drawable(
    mut abstract_surface: *mut cairo_surface_t,
) -> Drawable {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int as Drawable;
    }
    return (*surface).drawable;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_screen(
    mut abstract_surface: *mut cairo_surface_t,
) -> *mut Screen {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as *mut Screen;
    }
    return (*(*surface).screen).screen;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_visual(
    mut surface: *mut cairo_surface_t,
) -> *mut Visual {
    let mut xlib_surface: *mut cairo_xlib_surface_t = surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as *mut Visual;
    }
    return (*xlib_surface).visual;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_depth(
    mut abstract_surface: *mut cairo_surface_t,
) -> libc::c_int {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    return (*surface).depth;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_width(
    mut abstract_surface: *mut cairo_surface_t,
) -> libc::c_int {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    return (*surface).width;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_xlib_surface_get_height(
    mut abstract_surface: *mut cairo_surface_t,
) -> libc::c_int {
    let mut surface: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    if _cairo_surface_is_xlib(abstract_surface) == 0 {
        let mut status__: cairo_status_t = _cairo_error(
            CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
        );
        return 0 as libc::c_int;
    }
    return (*surface).height;
}
