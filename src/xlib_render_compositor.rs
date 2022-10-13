use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_region;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type _cairo_xlib_shm_display;
    fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_region_num_rectangles(region: *const cairo_region_t) -> libc::c_int;
    fn cairo_region_get_rectangle(
        region: *const cairo_region_t,
        nth: libc::c_int,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_scaled_glyph_set_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        surface: *mut cairo_image_surface_t,
    );
    fn _cairo_scaled_font_reset_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_glyph_attach_private(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        priv_0: *mut cairo_scaled_glyph_private_t,
        key: *const libc::c_void,
        destroy: Option::<
            unsafe extern "C" fn(
                *mut cairo_scaled_glyph_private_t,
                *mut cairo_scaled_glyph_t,
                *mut cairo_scaled_font_t,
            ) -> (),
        >,
    );
    fn _cairo_scaled_font_attach_private(
        scaled_font: *mut cairo_scaled_font_t,
        priv_0: *mut cairo_scaled_font_private_t,
        key: *const libc::c_void,
        destroy: Option::<
            unsafe extern "C" fn(
                *mut cairo_scaled_font_private_t,
                *mut cairo_scaled_font_t,
            ) -> (),
        >,
    );
    fn _cairo_scaled_font_find_private(
        scaled_font: *mut cairo_scaled_font_t,
        key: *const libc::c_void,
    ) -> *mut cairo_scaled_font_private_t;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_extents(boxes: *const cairo_boxes_t, box_0: *mut cairo_box_t);
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn _cairo_xlib_surface_set_precision(
        surface: *mut cairo_xlib_surface_t,
        antialias: cairo_antialias_t,
    );
    fn _cairo_xlib_source_create_for_pattern(
        dst: *mut cairo_surface_t,
        pattern: *const cairo_pattern_t,
        is_mask: cairo_bool_t,
        extents: *const cairo_rectangle_int_t,
        sample: *const cairo_rectangle_int_t,
        src_x: *mut libc::c_int,
        src_y: *mut libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_xlib_surface_get_shm(
        surface: *mut cairo_xlib_surface_t,
        overwrite: cairo_bool_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_xlib_surface_create_shm(
        other: *mut cairo_xlib_surface_t,
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
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
    fn _cairo_xlib_core_fill_rectangles(
        dst: *mut cairo_xlib_surface_t,
        color: *const cairo_color_t,
        num_rects: libc::c_int,
        rects: *mut cairo_rectangle_int_t,
    ) -> cairo_int_status_t;
    fn XMaxRequestSize(_: *mut Display) -> libc::c_long;
    fn XExtendedMaxRequestSize(_: *mut Display) -> libc::c_long;
    fn _cairo_xlib_core_fill_boxes(
        dst: *mut cairo_xlib_surface_t,
        color: *const cairo_color_t,
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
    fn XSetClipMask(_: *mut Display, _: GC, _: Pixmap) -> libc::c_int;
    fn XSetClipRectangles(
        _: *mut Display,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut XRectangle,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn _cairo_xlib_shm_surface_get_pixmap(surface: *mut cairo_surface_t) -> Pixmap;
    fn _cairo_xlib_shm_surface_mark_active(shm: *mut cairo_surface_t);
    fn _cairo_xlib_surface_ensure_picture(surface: *mut cairo_xlib_surface_t);
    fn _cairo_xlib_fallback_compositor_get() -> *const cairo_compositor_t;
    fn _cairo_xlib_screen_put_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        gc: GC,
    );
    fn _cairo_xlib_display_get_xrender_format(
        display: *mut cairo_xlib_display_t,
        format: cairo_format_t,
    ) -> *mut XRenderPictFormat;
    fn _cairo_xlib_display_acquire(
        device: *mut cairo_device_t,
        display: *mut *mut cairo_xlib_display_t,
    ) -> cairo_status_t;
    fn _cairo_xlib_surface_get_gc(
        display: *mut cairo_xlib_display_t,
        surface: *mut cairo_xlib_surface_t,
        gc: *mut GC,
    ) -> cairo_status_t;
    fn XRenderCreateGlyphSet(
        dpy: *mut Display,
        format: *const XRenderPictFormat,
    ) -> GlyphSet;
    fn XRenderComposite(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        mask: Picture,
        dst: Picture,
        src_x: libc::c_int,
        src_y: libc::c_int,
        mask_x: libc::c_int,
        mask_y: libc::c_int,
        dst_x: libc::c_int,
        dst_y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    fn XRenderSetPictureClipRectangles(
        dpy: *mut Display,
        picture: Picture,
        xOrigin: libc::c_int,
        yOrigin: libc::c_int,
        rects: *const XRectangle,
        n: libc::c_int,
    );
    fn XRenderChangePicture(
        dpy: *mut Display,
        picture: Picture,
        valuemask: libc::c_ulong,
        attributes: *const XRenderPictureAttributes,
    );
    fn XRenderCompositeTriStrip(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        dst: Picture,
        maskFormat: *const XRenderPictFormat,
        xSrc: libc::c_int,
        ySrc: libc::c_int,
        points: *const XPointFixed,
        npoint: libc::c_int,
    );
    fn XRenderCompositeTrapezoids(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        dst: Picture,
        maskFormat: *const XRenderPictFormat,
        xSrc: libc::c_int,
        ySrc: libc::c_int,
        traps: *const XTrapezoid,
        ntrap: libc::c_int,
    );
    fn XRenderFillRectangles(
        dpy: *mut Display,
        op: libc::c_int,
        dst: Picture,
        color: *const XRenderColor,
        rectangles: *const XRectangle,
        n_rects: libc::c_int,
    );
    fn XRenderFillRectangle(
        dpy: *mut Display,
        op: libc::c_int,
        dst: Picture,
        color: *const XRenderColor,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    fn XRenderCompositeText32(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        dst: Picture,
        maskFormat: *const XRenderPictFormat,
        xSrc: libc::c_int,
        ySrc: libc::c_int,
        xDst: libc::c_int,
        yDst: libc::c_int,
        elts: *const XGlyphElt32,
        nelt: libc::c_int,
    );
    fn XRenderCompositeText16(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        dst: Picture,
        maskFormat: *const XRenderPictFormat,
        xSrc: libc::c_int,
        ySrc: libc::c_int,
        xDst: libc::c_int,
        yDst: libc::c_int,
        elts: *const XGlyphElt16,
        nelt: libc::c_int,
    );
    fn XRenderCompositeText8(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        dst: Picture,
        maskFormat: *const XRenderPictFormat,
        xSrc: libc::c_int,
        ySrc: libc::c_int,
        xDst: libc::c_int,
        yDst: libc::c_int,
        elts: *const XGlyphElt8,
        nelt: libc::c_int,
    );
    fn XRenderFreeGlyphs(
        dpy: *mut Display,
        glyphset: GlyphSet,
        gids: *const Glyph,
        nglyphs: libc::c_int,
    );
    fn XRenderAddGlyphs(
        dpy: *mut Display,
        glyphset: GlyphSet,
        gids: *const Glyph,
        glyphs: *const XGlyphInfo,
        nglyphs: libc::c_int,
        images: *const libc::c_char,
        nbyte_images: libc::c_int,
    );
    fn XRenderFreeGlyphSet(dpy: *mut Display, glyphset: GlyphSet);
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_mask_compositor_init(
        compositor: *mut cairo_mask_compositor_t,
        delegate: *const cairo_compositor_t,
    );
    fn _cairo_traps_compositor_init(
        compositor: *mut cairo_traps_compositor_t,
        delegate: *const cairo_compositor_t,
    );
    fn _cairo_damage_add_rectangle(
        damage: *mut cairo_damage_t,
        rect: *const cairo_rectangle_int_t,
    ) -> *mut cairo_damage_t;
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
pub type cairo_scaled_font_private_t = _cairo_scaled_font_private;
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
pub type cairo_scaled_glyph_private_t = _cairo_scaled_glyph_private;
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
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
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
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub type _XPrivDisplay = *mut C2RustUnnamed_0;
pub type Glyph = XID;
pub type GlyphSet = XID;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderColor {
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub alpha: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGlyphInfo {
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub xOff: libc::c_short,
    pub yOff: libc::c_short,
}
pub type XGlyphInfo = _XGlyphInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGlyphElt8 {
    pub glyphset: GlyphSet,
    pub chars: *const libc::c_char,
    pub nchars: libc::c_int,
    pub xOff: libc::c_int,
    pub yOff: libc::c_int,
}
pub type XGlyphElt8 = _XGlyphElt8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGlyphElt16 {
    pub glyphset: GlyphSet,
    pub chars: *const libc::c_ushort,
    pub nchars: libc::c_int,
    pub xOff: libc::c_int,
    pub yOff: libc::c_int,
}
pub type XGlyphElt16 = _XGlyphElt16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGlyphElt32 {
    pub glyphset: GlyphSet,
    pub chars: *const libc::c_uint,
    pub nchars: libc::c_int,
    pub xOff: libc::c_int,
    pub yOff: libc::c_int,
}
pub type XGlyphElt32 = _XGlyphElt32;
pub type XFixed = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XPointFixed {
    pub x: XFixed,
    pub y: XFixed,
}
pub type XPointFixed = _XPointFixed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XLineFixed {
    pub p1: XPointFixed,
    pub p2: XPointFixed,
}
pub type XLineFixed = _XLineFixed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XTrapezoid {
    pub top: XFixed,
    pub bottom: XFixed,
    pub left: XLineFixed,
    pub right: XLineFixed,
}
pub type XTrapezoid = _XTrapezoid;
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
    pub last_solid_cache: [C2RustUnnamed_1; 2],
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
pub struct C2RustUnnamed_1 {
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
pub type cairo_xlib_source_t = _cairo_xlib_source;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const NUM_GLYPHSETS: C2RustUnnamed_2 = 3;
pub const GLYPHSET_INDEX_A1: C2RustUnnamed_2 = 2;
pub const GLYPHSET_INDEX_A8: C2RustUnnamed_2 = 1;
pub const GLYPHSET_INDEX_ARGB32: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_font_glyphset {
    pub glyphset: GlyphSet,
    pub format: cairo_format_t,
    pub xrender_format: *mut XRenderPictFormat,
    pub to_free: _cairo_xlib_font_glyphset_free_glyphs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_font_glyphset_free_glyphs {
    pub count: libc::c_int,
    pub indices: [libc::c_ulong; 128],
}
pub type cairo_xlib_font_glyphset_t = _cairo_xlib_font_glyphset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_font {
    pub base: cairo_scaled_font_private_t,
    pub font: *mut cairo_scaled_font_t,
    pub device: *mut cairo_device_t,
    pub link: cairo_list_t,
    pub glyphset: [cairo_xlib_font_glyphset_t; 3],
}
pub type cairo_xlib_font_t = _cairo_xlib_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_xlib_glyph_t {
    pub d: cairo_glyph_t,
    pub index: libc::c_ulong,
    pub i: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub index: libc::c_ulong,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type cairo_xrender_composite_text_func_t = Option::<
    unsafe extern "C" fn(
        *mut Display,
        libc::c_int,
        Picture,
        Picture,
        *const XRenderPictFormat,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const XGlyphElt8,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_xlib_glyph_private_t {
    pub base: cairo_scaled_glyph_private_t,
    pub glyphset: *mut cairo_xlib_font_glyphset_t,
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
unsafe extern "C" fn _cairo_fixed_16_16_floor(
    mut f: cairo_fixed_16_16_t,
) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 16 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 16 as libc::c_int) - 1 as libc::c_int
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
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
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
#[inline(always)]
unsafe extern "C" fn _cairo_is_little_endian() -> cairo_bool_t {
    static mut i: libc::c_int = 1 as libc::c_int;
    return (*(&i as *const libc::c_int as *mut libc::c_char) as libc::c_int
        == 0x1 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh3 = (*entry).next;
    *fresh3 = entry;
    let ref mut fresh4 = (*entry).prev;
    *fresh4 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh5 = (*next).prev;
    *fresh5 = entry;
    let ref mut fresh6 = (*entry).next;
    *fresh6 = next;
    let ref mut fresh7 = (*entry).prev;
    *fresh7 = prev;
    let ref mut fresh8 = (*prev).next;
    *fresh8 = entry;
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
    let ref mut fresh9 = (*next).prev;
    *fresh9 = prev;
    let ref mut fresh10 = (*prev).next;
    *fresh10 = next;
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
unsafe extern "C" fn check_composite(
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut display: *mut cairo_xlib_display_t = (*((*extents).surface
        as *mut cairo_xlib_surface_t))
        .display;
    if !((*extents).op as libc::c_uint
        <= CAIRO_OPERATOR_SATURATE as libc::c_int as libc::c_uint
        || ((*display).render_major > 0 as libc::c_int
            || (*display).render_major == 0 as libc::c_int
                && (*display).render_minor >= 11 as libc::c_int)
            && (*extents).op as libc::c_uint
                <= CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn acquire(mut abstract_dst: *mut libc::c_void) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_xlib_display_acquire((*dst).base.device, &mut (*dst).display)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh11 = (*dst).dpy;
    *fresh11 = (*(*dst).display).display;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn release(mut abstract_dst: *mut libc::c_void) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    cairo_device_release(&mut (*(*dst).display).base);
    let ref mut fresh12 = (*dst).dpy;
    *fresh12 = 0 as *mut Display;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn set_clip_region(
    mut _surface: *mut libc::c_void,
    mut region: *mut cairo_region_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xlib_surface_t = _surface as *mut cairo_xlib_surface_t;
    _cairo_xlib_surface_ensure_picture(surface);
    if !region.is_null() {
        let mut stack_rects: [XRectangle; 256] = [XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 256];
        let mut rects: *mut XRectangle = stack_rects.as_mut_ptr();
        let mut n_rects: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        n_rects = cairo_region_num_rectangles(region);
        if n_rects
            > (::std::mem::size_of::<[XRectangle; 256]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<XRectangle>() as libc::c_ulong)
                as libc::c_int
        {
            rects = _cairo_malloc_ab(
                n_rects as size_t,
                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
            ) as *mut XRectangle;
            if rects.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
        i = 0 as libc::c_int;
        while i < n_rects {
            let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            cairo_region_get_rectangle(region, i, &mut rect);
            (*rects.offset(i as isize)).x = rect.x as libc::c_short;
            (*rects.offset(i as isize)).y = rect.y as libc::c_short;
            (*rects.offset(i as isize)).width = rect.width as libc::c_ushort;
            (*rects.offset(i as isize)).height = rect.height as libc::c_ushort;
            i += 1;
        }
        XRenderSetPictureClipRectangles(
            (*surface).dpy,
            (*surface).picture,
            0 as libc::c_int,
            0 as libc::c_int,
            rects,
            n_rects,
        );
        if rects != stack_rects.as_mut_ptr() {
            free(rects as *mut libc::c_void);
        }
    } else {
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
        pa.clip_mask = 0 as libc::c_long as Pixmap;
        XRenderChangePicture(
            (*surface).dpy,
            (*surface).picture,
            ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong,
            &mut pa,
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn copy_image_boxes(
    mut _dst: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut boxes: *mut cairo_boxes_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = _dst as *mut cairo_xlib_surface_t;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut src: Pixmap = 0;
    let mut gc: GC = 0 as *mut _XGC;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*image).depth == (*dst).depth {} else {
        __assert_fail(
            b"image->depth == dst->depth\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_int_status_t copy_image_boxes(void *, cairo_image_surface_t *, cairo_boxes_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    status = acquire(dst as *mut libc::c_void);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_xlib_surface_get_gc((*dst).display, dst, &mut gc)
        as cairo_int_status_t;
    if status as u64 != 0 {
        release(dst as *mut libc::c_void);
        return status;
    }
    src = _cairo_xlib_shm_surface_get_pixmap(&mut (*image).base);
    if (*boxes).num_boxes == 1 as libc::c_int {
        let mut x1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.x,
        );
        let mut y1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.y,
        );
        let mut x2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.x,
        );
        let mut y2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.y,
        );
        _cairo_xlib_shm_surface_mark_active(&mut (*image).base);
        XCopyArea(
            (*dst).dpy,
            src,
            (*dst).drawable,
            gc,
            x1 + dx,
            y1 + dy,
            (x2 - x1) as libc::c_uint,
            (y2 - y1) as libc::c_uint,
            x1,
            y1,
        );
    } else {
        let mut stack_rects: [XRectangle; 256] = [XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 256];
        let mut rects: *mut XRectangle = stack_rects.as_mut_ptr();
        if (*boxes).num_boxes
            > (::std::mem::size_of::<[XRectangle; 256]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<XRectangle>() as libc::c_ulong)
                as libc::c_int
        {
            rects = _cairo_malloc_ab(
                (*boxes).num_boxes as size_t,
                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
            ) as *mut XRectangle;
            if rects.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
        j = 0 as libc::c_int;
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x1_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.x,
                );
                let mut y1_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.y,
                );
                let mut x2_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.x,
                );
                let mut y2_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.y,
                );
                if x2_0 > x1_0 && y2_0 > y1_0 {
                    (*rects.offset(j as isize)).x = x1_0 as libc::c_short;
                    (*rects.offset(j as isize)).y = y1_0 as libc::c_short;
                    (*rects.offset(j as isize)).width = (x2_0 - x1_0) as libc::c_ushort;
                    (*rects.offset(j as isize)).height = (y2_0 - y1_0) as libc::c_ushort;
                    j += 1;
                }
                i += 1;
            }
            chunk = (*chunk).next;
        }
        XSetClipRectangles(
            (*dst).dpy,
            gc,
            0 as libc::c_int,
            0 as libc::c_int,
            rects,
            j,
            0 as libc::c_int,
        );
        _cairo_xlib_shm_surface_mark_active(&mut (*image).base);
        XCopyArea(
            (*dst).dpy,
            src,
            (*dst).drawable,
            gc,
            0 as libc::c_int,
            0 as libc::c_int,
            (*image).width as libc::c_uint,
            (*image).height as libc::c_uint,
            -dx,
            -dy,
        );
        XSetClipMask((*dst).dpy, gc, 0 as libc::c_long as Pixmap);
        if rects != stack_rects.as_mut_ptr() {
            free(rects as *mut libc::c_void);
        }
    }
    _cairo_xlib_surface_put_gc((*dst).display, dst, gc);
    release(dst as *mut libc::c_void);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn boxes_cover_surface(
    mut boxes: *mut cairo_boxes_t,
    mut surface: *mut cairo_xlib_surface_t,
) -> cairo_bool_t {
    let mut b: *mut cairo_box_t = 0 as *mut cairo_box_t;
    if (*boxes).num_boxes != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    b = &mut *((*boxes).chunks.base).offset(0 as libc::c_int as isize)
        as *mut cairo_box_t;
    if _cairo_fixed_integer_part((*b).p1.x) > 0 as libc::c_int
        || _cairo_fixed_integer_part((*b).p1.y) > 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if _cairo_fixed_integer_part((*b).p2.x) < (*surface).width
        || _cairo_fixed_integer_part((*b).p2.y) < (*surface).height
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn draw_image_boxes(
    mut _dst: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
    mut boxes: *mut cairo_boxes_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut dst: *mut cairo_xlib_surface_t = _dst as *mut cairo_xlib_surface_t;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut shm: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if (*image).base.device == (*dst).base.device {
        if (*image).depth != (*dst).depth {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        if _cairo_xlib_shm_surface_get_pixmap(&mut (*image).base) != 0 {
            return copy_image_boxes(dst as *mut libc::c_void, image, boxes, dx, dy);
        }
        current_block = 14030995180564626706;
    } else {
        if boxes_cover_surface(boxes, dst) != 0 {
            shm = _cairo_xlib_surface_get_shm(dst, 1 as libc::c_int)
                as *mut cairo_image_surface_t;
        }
        if !shm.is_null() {
            chunk = &mut (*boxes).chunks;
            while !chunk.is_null() {
                i = 0 as libc::c_int;
                while i < (*chunk).count {
                    let mut b: *mut cairo_box_t = &mut *((*chunk).base)
                        .offset(i as isize) as *mut cairo_box_t;
                    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                    };
                    r.x = _cairo_fixed_integer_part((*b).p1.x);
                    r.y = _cairo_fixed_integer_part((*b).p1.y);
                    r.width = _cairo_fixed_integer_part((*b).p2.x) - r.x;
                    r.height = _cairo_fixed_integer_part((*b).p2.y) - r.y;
                    if (*shm).pixman_format as libc::c_uint
                        != (*image).pixman_format as libc::c_uint
                        || pixman_blt(
                            (*image).data as *mut uint32_t,
                            (*shm).data as *mut uint32_t,
                            ((*image).stride as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                                ) as libc::c_int,
                            ((*shm).stride as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                                ) as libc::c_int,
                            (((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
                                & (((1 as libc::c_int) << 8 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint)
                                << ((*image).pixman_format as libc::c_uint
                                    >> 22 as libc::c_int & 3 as libc::c_int as libc::c_uint))
                                as libc::c_int,
                            (((*shm).pixman_format as libc::c_uint >> 24 as libc::c_int
                                & (((1 as libc::c_int) << 8 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint)
                                << ((*shm).pixman_format as libc::c_uint
                                    >> 22 as libc::c_int & 3 as libc::c_int as libc::c_uint))
                                as libc::c_int,
                            r.x + dx,
                            r.y + dy,
                            r.x,
                            r.y,
                            r.width,
                            r.height,
                        ) == 0
                    {
                        pixman_image_composite32(
                            PIXMAN_OP_SRC,
                            (*image).pixman_image,
                            0 as *mut pixman_image_t,
                            (*shm).pixman_image,
                            r.x + dx,
                            r.y + dy,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            r.x,
                            r.y,
                            r.width,
                            r.height,
                        );
                    }
                    let ref mut fresh13 = (*shm).base.damage;
                    *fresh13 = _cairo_damage_add_rectangle((*shm).base.damage, &mut r);
                    i += 1;
                }
                chunk = (*chunk).next;
            }
            let ref mut fresh14 = (*dst).base;
            (*fresh14).set_is_clear(0 as libc::c_int as libc::c_uint);
            let ref mut fresh15 = (*dst).fallback;
            *fresh15 += 1;
            let ref mut fresh16 = (*dst).base.serial;
            *fresh16 = (*fresh16).wrapping_add(1);
            return CAIRO_INT_STATUS_NOTHING_TO_DO;
        }
        if (*image).depth == (*dst).depth && !((*(*dst).display).shm).is_null() {
            let mut extents: cairo_box_t = cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            let mut r_0: cairo_rectangle_int_t = cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            _cairo_boxes_extents(boxes, &mut extents);
            _cairo_box_round_to_rectangle(&mut extents, &mut r_0);
            shm = _cairo_xlib_surface_create_shm(
                dst,
                (*image).pixman_format,
                r_0.width,
                r_0.height,
            ) as *mut cairo_image_surface_t;
            if !shm.is_null() {
                let mut tx: libc::c_int = -r_0.x;
                let mut ty: libc::c_int = -r_0.y;
                if (*shm).pixman_format as libc::c_uint
                    == (*image).pixman_format as libc::c_uint
                {} else {
                    __assert_fail(
                        b"shm->pixman_format == image->pixman_format\0" as *const u8
                            as *const libc::c_char,
                        b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                            as *const libc::c_char,
                        319 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 96],
                            &[libc::c_char; 96],
                        >(
                            b"cairo_int_status_t draw_image_boxes(void *, cairo_image_surface_t *, cairo_boxes_t *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                chunk = &mut (*boxes).chunks;
                while !chunk.is_null() {
                    i = 0 as libc::c_int;
                    while i < (*chunk).count {
                        let mut b_0: *mut cairo_box_t = &mut *((*chunk).base)
                            .offset(i as isize) as *mut cairo_box_t;
                        r_0.x = _cairo_fixed_integer_part((*b_0).p1.x);
                        r_0.y = _cairo_fixed_integer_part((*b_0).p1.y);
                        r_0.width = _cairo_fixed_integer_part((*b_0).p2.x) - r_0.x;
                        r_0.height = _cairo_fixed_integer_part((*b_0).p2.y) - r_0.y;
                        if pixman_blt(
                            (*image).data as *mut uint32_t,
                            (*shm).data as *mut uint32_t,
                            ((*image).stride as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                                ) as libc::c_int,
                            ((*shm).stride as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                                ) as libc::c_int,
                            (((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
                                & (((1 as libc::c_int) << 8 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint)
                                << ((*image).pixman_format as libc::c_uint
                                    >> 22 as libc::c_int & 3 as libc::c_int as libc::c_uint))
                                as libc::c_int,
                            (((*shm).pixman_format as libc::c_uint >> 24 as libc::c_int
                                & (((1 as libc::c_int) << 8 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint)
                                << ((*shm).pixman_format as libc::c_uint
                                    >> 22 as libc::c_int & 3 as libc::c_int as libc::c_uint))
                                as libc::c_int,
                            r_0.x + dx,
                            r_0.y + dy,
                            r_0.x + tx,
                            r_0.y + ty,
                            r_0.width,
                            r_0.height,
                        ) == 0
                        {
                            pixman_image_composite32(
                                PIXMAN_OP_SRC,
                                (*image).pixman_image,
                                0 as *mut pixman_image_t,
                                (*shm).pixman_image,
                                r_0.x + dx,
                                r_0.y + dy,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                r_0.x + tx,
                                r_0.y + ty,
                                r_0.width,
                                r_0.height,
                            );
                        }
                        i += 1;
                    }
                    chunk = (*chunk).next;
                }
                dx = tx;
                dy = ty;
                image = shm;
                if _cairo_xlib_shm_surface_get_pixmap(&mut (*image).base) != 0 {
                    status = copy_image_boxes(
                        dst as *mut libc::c_void,
                        image,
                        boxes,
                        dx,
                        dy,
                    );
                    if status as libc::c_uint
                        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                    {
                        current_block = 10126451532405318797;
                    } else {
                        current_block = 14030995180564626706;
                    }
                } else {
                    current_block = 14030995180564626706;
                }
            } else {
                current_block = 14030995180564626706;
            }
        } else {
            current_block = 14030995180564626706;
        }
    }
    match current_block {
        14030995180564626706 => {
            status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            chunk = &mut (*boxes).chunks;
            's_290: while !chunk.is_null() {
                i = 0 as libc::c_int;
                while i < (*chunk).count {
                    let mut b_1: *mut cairo_box_t = &mut *((*chunk).base)
                        .offset(i as isize) as *mut cairo_box_t;
                    let mut x1: libc::c_int = _cairo_fixed_integer_part((*b_1).p1.x);
                    let mut y1: libc::c_int = _cairo_fixed_integer_part((*b_1).p1.y);
                    let mut x2: libc::c_int = _cairo_fixed_integer_part((*b_1).p2.x);
                    let mut y2: libc::c_int = _cairo_fixed_integer_part((*b_1).p2.y);
                    if _cairo_xlib_surface_draw_image(
                        dst,
                        image,
                        x1 + dx,
                        y1 + dy,
                        x2 - x1,
                        y2 - y1,
                        x1,
                        y1,
                    ) as u64 != 0
                    {
                        status = CAIRO_INT_STATUS_UNSUPPORTED;
                        break 's_290;
                    } else {
                        i += 1;
                    }
                }
                chunk = (*chunk).next;
            }
        }
        _ => {}
    }
    cairo_surface_destroy(&mut (*shm).base);
    return status;
}
unsafe extern "C" fn copy_boxes(
    mut _dst: *mut libc::c_void,
    mut _src: *mut cairo_surface_t,
    mut boxes: *mut cairo_boxes_t,
    mut extents: *const cairo_rectangle_int_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = _dst as *mut cairo_xlib_surface_t;
    let mut src: *mut cairo_xlib_surface_t = _src as *mut cairo_xlib_surface_t;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut gc: GC = 0 as *mut _XGC;
    let mut d: Drawable = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if _cairo_xlib_surface_same_screen(dst, src) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*dst).depth != (*src).depth {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = acquire(dst as *mut libc::c_void);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_xlib_surface_get_gc((*dst).display, dst, &mut gc)
        as cairo_int_status_t;
    if status as u64 != 0 {
        release(dst as *mut libc::c_void);
        return status;
    }
    if (*src).fallback != 0 && (*(*(*src).shm).damage).dirty != 0 {
        if src != dst {} else {
            __assert_fail(
                b"src != dst\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                    as *const libc::c_char,
                416 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"cairo_int_status_t copy_boxes(void *, cairo_surface_t *, cairo_boxes_t *, const cairo_rectangle_int_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        d = _cairo_xlib_shm_surface_get_pixmap((*src).shm);
        if d != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"d != 0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                    as *const libc::c_char,
                418 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"cairo_int_status_t copy_boxes(void *, cairo_surface_t *, cairo_boxes_t *, const cairo_rectangle_int_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
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
                gc,
                ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
                &mut gcv,
            );
        }
        d = (*src).drawable;
    }
    if (*boxes).num_boxes == 1 as libc::c_int {
        let mut x1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.x,
        );
        let mut y1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.y,
        );
        let mut x2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.x,
        );
        let mut y2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.y,
        );
        XCopyArea(
            (*dst).dpy,
            d,
            (*dst).drawable,
            gc,
            x1 + dx,
            y1 + dy,
            (x2 - x1) as libc::c_uint,
            (y2 - y1) as libc::c_uint,
            x1,
            y1,
        );
    } else if src == dst || (*src).owns_pixmap == 0 && (*dst).owns_pixmap == 0 {
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x1_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.x,
                );
                let mut y1_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.y,
                );
                let mut x2_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.x,
                );
                let mut y2_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.y,
                );
                XCopyArea(
                    (*dst).dpy,
                    d,
                    (*dst).drawable,
                    gc,
                    x1_0 + dx,
                    y1_0 + dy,
                    (x2_0 - x1_0) as libc::c_uint,
                    (y2_0 - y1_0) as libc::c_uint,
                    x1_0,
                    y1_0,
                );
                i += 1;
            }
            chunk = (*chunk).next;
        }
    } else {
        let mut stack_rects: [XRectangle; 256] = [XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 256];
        let mut rects: *mut XRectangle = stack_rects.as_mut_ptr();
        if (*boxes).num_boxes
            > (::std::mem::size_of::<[XRectangle; 256]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<XRectangle>() as libc::c_ulong)
                as libc::c_int
        {
            rects = _cairo_malloc_ab(
                (*boxes).num_boxes as size_t,
                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
            ) as *mut XRectangle;
            if rects.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
        j = 0 as libc::c_int;
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x1_1: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.x,
                );
                let mut y1_1: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.y,
                );
                let mut x2_1: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.x,
                );
                let mut y2_1: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.y,
                );
                (*rects.offset(j as isize)).x = x1_1 as libc::c_short;
                (*rects.offset(j as isize)).y = y1_1 as libc::c_short;
                (*rects.offset(j as isize)).width = (x2_1 - x1_1) as libc::c_ushort;
                (*rects.offset(j as isize)).height = (y2_1 - y1_1) as libc::c_ushort;
                j += 1;
                i += 1;
            }
            chunk = (*chunk).next;
        }
        if j == (*boxes).num_boxes {} else {
            __assert_fail(
                b"j == boxes->num_boxes\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                    as *const libc::c_char,
                487 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"cairo_int_status_t copy_boxes(void *, cairo_surface_t *, cairo_boxes_t *, const cairo_rectangle_int_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        XSetClipRectangles(
            (*dst).dpy,
            gc,
            0 as libc::c_int,
            0 as libc::c_int,
            rects,
            j,
            0 as libc::c_int,
        );
        XCopyArea(
            (*dst).dpy,
            d,
            (*dst).drawable,
            gc,
            (*extents).x + dx,
            (*extents).y + dy,
            (*extents).width as libc::c_uint,
            (*extents).height as libc::c_uint,
            (*extents).x,
            (*extents).y,
        );
        XSetClipMask((*dst).dpy, gc, 0 as libc::c_long as Pixmap);
        if rects != stack_rects.as_mut_ptr() {
            free(rects as *mut libc::c_void);
        }
    }
    if (*src).fallback != 0 && (*(*(*src).shm).damage).dirty != 0 {
        _cairo_xlib_shm_surface_mark_active((*src).shm);
    } else if (*src).owns_pixmap == 0 {
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
            gc,
            ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
            &mut gcv_0,
        );
    }
    _cairo_xlib_surface_put_gc((*dst).display, dst, gc);
    release(dst as *mut libc::c_void);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _render_operator(mut op: cairo_operator_t) -> libc::c_int {
    match op as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 => return 1 as libc::c_int,
        2 => return 3 as libc::c_int,
        3 => return 5 as libc::c_int,
        4 => return 7 as libc::c_int,
        5 => return 9 as libc::c_int,
        6 => return 2 as libc::c_int,
        7 => return 4 as libc::c_int,
        8 => return 6 as libc::c_int,
        9 => return 8 as libc::c_int,
        10 => return 10 as libc::c_int,
        11 => return 11 as libc::c_int,
        12 => return 12 as libc::c_int,
        13 => return 13 as libc::c_int,
        14 => return 0x30 as libc::c_int,
        15 => return 0x31 as libc::c_int,
        16 => return 0x32 as libc::c_int,
        17 => return 0x33 as libc::c_int,
        18 => return 0x34 as libc::c_int,
        19 => return 0x35 as libc::c_int,
        20 => return 0x36 as libc::c_int,
        21 => return 0x37 as libc::c_int,
        22 => return 0x38 as libc::c_int,
        23 => return 0x39 as libc::c_int,
        24 => return 0x3a as libc::c_int,
        25 => return 0x3b as libc::c_int,
        26 => return 0x3c as libc::c_int,
        27 => return 0x3d as libc::c_int,
        28 => return 0x3e as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                        as *const libc::c_char,
                    585 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"int _render_operator(cairo_operator_t)\0"))
                        .as_ptr(),
                );
            }
            return 3 as libc::c_int;
        }
    };
}
unsafe extern "C" fn fill_reduces_to_source(
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut dst: *mut cairo_xlib_surface_t,
) -> cairo_bool_t {
    if ((*dst).base).is_clear() as libc::c_int != 0
        || (*color).alpha_short as libc::c_int >= 0xff00 as libc::c_int
    {
        if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        if op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint {
            return ((*dst).base.content as libc::c_uint
                & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fill_rectangles(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut rects: *mut cairo_rectangle_int_t,
    mut num_rects: libc::c_int,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut render_color: XRenderColor = XRenderColor {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let mut i: libc::c_int = 0;
    if fill_reduces_to_source(op, color, dst) != 0 {
        op = CAIRO_OPERATOR_SOURCE;
    }
    if !((*(*dst).display).render_major > 0 as libc::c_int
        || (*(*dst).display).render_major == 0 as libc::c_int
            && (*(*dst).display).render_minor >= 1 as libc::c_int)
    {
        let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
        status = CAIRO_INT_STATUS_UNSUPPORTED;
        if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
            status = _cairo_xlib_core_fill_rectangles(dst, color, num_rects, rects);
        }
        return status;
    }
    render_color.red = (*color).red_short;
    render_color.green = (*color).green_short;
    render_color.blue = (*color).blue_short;
    render_color.alpha = (*color).alpha_short;
    _cairo_xlib_surface_ensure_picture(dst);
    if num_rects == 1 as libc::c_int {
        XRenderFillRectangle(
            (*dst).dpy,
            _render_operator(op),
            (*dst).picture,
            &mut render_color,
            (*rects).x,
            (*rects).y,
            (*rects).width as libc::c_uint,
            (*rects).height as libc::c_uint,
        );
    } else {
        let mut stack_xrects: [XRectangle; 256] = [XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 256];
        let mut xrects: *mut XRectangle = stack_xrects.as_mut_ptr();
        if num_rects
            > (::std::mem::size_of::<[XRectangle; 256]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<XRectangle>() as libc::c_ulong)
                as libc::c_int
        {
            xrects = _cairo_malloc_ab(
                num_rects as size_t,
                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
            ) as *mut XRectangle;
            if xrects.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
        i = 0 as libc::c_int;
        while i < num_rects {
            (*xrects.offset(i as isize))
                .x = (*rects.offset(i as isize)).x as libc::c_short;
            (*xrects.offset(i as isize))
                .y = (*rects.offset(i as isize)).y as libc::c_short;
            (*xrects.offset(i as isize))
                .width = (*rects.offset(i as isize)).width as libc::c_ushort;
            (*xrects.offset(i as isize))
                .height = (*rects.offset(i as isize)).height as libc::c_ushort;
            i += 1;
        }
        XRenderFillRectangles(
            (*dst).dpy,
            _render_operator(op),
            (*dst).picture,
            &mut render_color,
            xrects,
            num_rects,
        );
        if xrects != stack_xrects.as_mut_ptr() {
            free(xrects as *mut libc::c_void);
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn fill_boxes(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = abstract_surface
        as *mut cairo_xlib_surface_t;
    let mut render_color: XRenderColor = XRenderColor {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    if fill_reduces_to_source(op, color, dst) != 0 {
        op = CAIRO_OPERATOR_SOURCE;
    }
    if !((*(*dst).display).render_major > 0 as libc::c_int
        || (*(*dst).display).render_major == 0 as libc::c_int
            && (*(*dst).display).render_minor >= 1 as libc::c_int)
    {
        let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
        status = CAIRO_INT_STATUS_UNSUPPORTED;
        if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
            status = _cairo_xlib_core_fill_boxes(dst, color, boxes);
        }
        return status;
    }
    render_color.red = (*color).red_short;
    render_color.green = (*color).green_short;
    render_color.blue = (*color).blue_short;
    render_color.alpha = (*color).alpha_short;
    _cairo_xlib_surface_ensure_picture(dst);
    if (*boxes).num_boxes == 1 as libc::c_int {
        let mut x1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.x,
        );
        let mut y1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.y,
        );
        let mut x2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.x,
        );
        let mut y2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.y,
        );
        XRenderFillRectangle(
            (*dst).dpy,
            _render_operator(op),
            (*dst).picture,
            &mut render_color,
            x1,
            y1,
            (x2 - x1) as libc::c_uint,
            (y2 - y1) as libc::c_uint,
        );
    } else {
        let mut stack_xrects: [XRectangle; 256] = [XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 256];
        let mut xrects: *mut XRectangle = stack_xrects.as_mut_ptr();
        let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        if (*boxes).num_boxes
            > (::std::mem::size_of::<[XRectangle; 256]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<XRectangle>() as libc::c_ulong)
                as libc::c_int
        {
            xrects = _cairo_malloc_ab(
                (*boxes).num_boxes as size_t,
                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
            ) as *mut XRectangle;
            if xrects.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
        j = 0 as libc::c_int;
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x1_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.x,
                );
                let mut y1_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p1.y,
                );
                let mut x2_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.x,
                );
                let mut y2_0: libc::c_int = _cairo_fixed_integer_part(
                    (*((*chunk).base).offset(i as isize)).p2.y,
                );
                (*xrects.offset(j as isize)).x = x1_0 as libc::c_short;
                (*xrects.offset(j as isize)).y = y1_0 as libc::c_short;
                (*xrects.offset(j as isize)).width = (x2_0 - x1_0) as libc::c_ushort;
                (*xrects.offset(j as isize)).height = (y2_0 - y1_0) as libc::c_ushort;
                j += 1;
                i += 1;
            }
            chunk = (*chunk).next;
        }
        XRenderFillRectangles(
            (*dst).dpy,
            _render_operator(op),
            (*dst).picture,
            &mut render_color,
            xrects,
            j,
        );
        if xrects != stack_xrects.as_mut_ptr() {
            free(xrects as *mut libc::c_void);
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite(
    mut abstract_dst: *mut libc::c_void,
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
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    let mut src: *mut cairo_xlib_source_t = abstract_src as *mut cairo_xlib_source_t;
    op = _render_operator(op) as cairo_operator_t;
    _cairo_xlib_surface_ensure_picture(dst);
    if !abstract_mask.is_null() {
        let mut mask: *mut cairo_xlib_source_t = abstract_mask
            as *mut cairo_xlib_source_t;
        XRenderComposite(
            (*dst).dpy,
            op as libc::c_int,
            (*src).picture,
            (*mask).picture,
            (*dst).picture,
            src_x,
            src_y,
            mask_x,
            mask_y,
            dst_x,
            dst_y,
            width,
            height,
        );
    } else {
        XRenderComposite(
            (*dst).dpy,
            op as libc::c_int,
            (*src).picture,
            0 as libc::c_int as Picture,
            (*dst).picture,
            src_x,
            src_y,
            0 as libc::c_int,
            0 as libc::c_int,
            dst_x,
            dst_y,
            width,
            height,
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn lerp(
    mut abstract_dst: *mut libc::c_void,
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
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    let mut src: *mut cairo_xlib_source_t = abstract_src as *mut cairo_xlib_source_t;
    let mut mask: *mut cairo_xlib_source_t = abstract_mask as *mut cairo_xlib_source_t;
    _cairo_xlib_surface_ensure_picture(dst);
    XRenderComposite(
        (*dst).dpy,
        8 as libc::c_int,
        (*mask).picture,
        0 as libc::c_long as Picture,
        (*dst).picture,
        mask_x,
        mask_y,
        0 as libc::c_int,
        0 as libc::c_int,
        dst_x,
        dst_y,
        width,
        height,
    );
    XRenderComposite(
        (*dst).dpy,
        12 as libc::c_int,
        (*src).picture,
        (*mask).picture,
        (*dst).picture,
        src_x,
        src_y,
        mask_x,
        mask_y,
        dst_x,
        dst_y,
        width,
        height,
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_boxes(
    mut abstract_dst: *mut libc::c_void,
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
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    let mut src: Picture = (*(abstract_src as *mut cairo_xlib_source_t)).picture;
    let mut mask: Picture = if !abstract_mask.is_null() {
        (*(abstract_mask as *mut cairo_xlib_source_t)).picture
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut stack_rects: [XRectangle; 256] = [XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 256];
    let mut rects: *mut XRectangle = stack_rects.as_mut_ptr();
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    op = _render_operator(op) as cairo_operator_t;
    _cairo_xlib_surface_ensure_picture(dst);
    if (*boxes).num_boxes == 1 as libc::c_int {
        let mut x1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.x,
        );
        let mut y1: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p1.y,
        );
        let mut x2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.x,
        );
        let mut y2: libc::c_int = _cairo_fixed_integer_part(
            (*((*boxes).chunks.base).offset(0 as libc::c_int as isize)).p2.y,
        );
        XRenderComposite(
            (*dst).dpy,
            op as libc::c_int,
            src,
            mask,
            (*dst).picture,
            x1 + src_x,
            y1 + src_y,
            x1 + mask_x,
            y1 + mask_y,
            x1 - dst_x,
            y1 - dst_y,
            (x2 - x1) as libc::c_uint,
            (y2 - y1) as libc::c_uint,
        );
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if (*boxes).num_boxes
        > (::std::mem::size_of::<[XRectangle; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<XRectangle>() as libc::c_ulong)
            as libc::c_int
    {
        rects = _cairo_malloc_ab(
            (*boxes).num_boxes as size_t,
            ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
        ) as *mut XRectangle;
        if rects.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    j = 0 as libc::c_int;
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            let mut x1_0: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p1.x,
            );
            let mut y1_0: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p1.y,
            );
            let mut x2_0: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p2.x,
            );
            let mut y2_0: libc::c_int = _cairo_fixed_integer_part(
                (*((*chunk).base).offset(i as isize)).p2.y,
            );
            (*rects.offset(j as isize)).x = (x1_0 - dst_x) as libc::c_short;
            (*rects.offset(j as isize)).y = (y1_0 - dst_y) as libc::c_short;
            (*rects.offset(j as isize)).width = (x2_0 - x1_0) as libc::c_ushort;
            (*rects.offset(j as isize)).height = (y2_0 - y1_0) as libc::c_ushort;
            j += 1;
            i += 1;
        }
        chunk = (*chunk).next;
    }
    if j == (*boxes).num_boxes {} else {
        __assert_fail(
            b"j == boxes->num_boxes\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                as *const libc::c_char,
            909 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"cairo_int_status_t composite_boxes(void *, cairo_operator_t, cairo_surface_t *, cairo_surface_t *, int, int, int, int, int, int, cairo_boxes_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    XRenderSetPictureClipRectangles(
        (*dst).dpy,
        (*dst).picture,
        0 as libc::c_int,
        0 as libc::c_int,
        rects,
        j,
    );
    if rects != stack_rects.as_mut_ptr() {
        free(rects as *mut libc::c_void);
    }
    XRenderComposite(
        (*dst).dpy,
        op as libc::c_int,
        src,
        mask,
        (*dst).picture,
        (*extents).x + src_x,
        (*extents).y + src_y,
        (*extents).x + mask_x,
        (*extents).y + mask_y,
        (*extents).x - dst_x,
        (*extents).y - dst_y,
        (*extents).width as libc::c_uint,
        (*extents).height as libc::c_uint,
    );
    set_clip_region(dst as *mut libc::c_void, 0 as *mut cairo_region_t);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_font_close(mut priv_0: *mut cairo_xlib_font_t) {
    let mut display: *mut cairo_xlib_display_t = (*priv_0).base.key
        as *mut cairo_xlib_display_t;
    let mut i: libc::c_int = 0;
    _cairo_scaled_font_reset_cache((*priv_0).font);
    i = 0 as libc::c_int;
    while i < NUM_GLYPHSETS as libc::c_int {
        let mut info: *mut cairo_xlib_font_glyphset_t = 0
            as *mut cairo_xlib_font_glyphset_t;
        info = &mut *((*priv_0).glyphset).as_mut_ptr().offset(i as isize)
            as *mut cairo_xlib_font_glyphset_t;
        if (*info).glyphset != 0 {
            XRenderFreeGlyphSet((*display).display, (*info).glyphset);
        }
        i += 1;
    }
    cairo_list_del(&mut (*priv_0).link);
    cairo_list_del(&mut (*priv_0).base.link);
    free(priv_0 as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xlib_font_fini(
    mut abstract_private: *mut cairo_scaled_font_private_t,
    mut font: *mut cairo_scaled_font_t,
) {
    let mut priv_0: *mut cairo_xlib_font_t = abstract_private as *mut cairo_xlib_font_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut i: libc::c_int = 0;
    cairo_list_del(&mut (*priv_0).base.link);
    cairo_list_del(&mut (*priv_0).link);
    status = _cairo_xlib_display_acquire((*priv_0).device, &mut display);
    if !(status as u64 != 0) {
        i = 0 as libc::c_int;
        while i < NUM_GLYPHSETS as libc::c_int {
            let mut info: *mut cairo_xlib_font_glyphset_t = 0
                as *mut cairo_xlib_font_glyphset_t;
            info = &mut *((*priv_0).glyphset).as_mut_ptr().offset(i as isize)
                as *mut cairo_xlib_font_glyphset_t;
            if (*info).glyphset != 0 {
                XRenderFreeGlyphSet((*display).display, (*info).glyphset);
            }
            i += 1;
        }
        cairo_device_release(&mut (*display).base);
    }
    cairo_device_destroy((*priv_0).device);
    free(priv_0 as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xlib_font_create(
    mut display: *mut cairo_xlib_display_t,
    mut font: *mut cairo_scaled_font_t,
) -> *mut cairo_xlib_font_t {
    let mut priv_0: *mut cairo_xlib_font_t = 0 as *mut cairo_xlib_font_t;
    let mut i: libc::c_int = 0;
    priv_0 = (if ::std::mem::size_of::<cairo_xlib_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_font_t;
    if priv_0.is_null() {
        return 0 as *mut cairo_xlib_font_t;
    }
    _cairo_scaled_font_attach_private(
        font,
        &mut (*priv_0).base,
        display as *const libc::c_void,
        Some(
            _cairo_xlib_font_fini
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_private_t,
                    *mut cairo_scaled_font_t,
                ) -> (),
        ),
    );
    let ref mut fresh17 = (*priv_0).device;
    *fresh17 = cairo_device_reference(&mut (*display).base);
    let ref mut fresh18 = (*priv_0).font;
    *fresh18 = font;
    cairo_list_add(&mut (*priv_0).link, &mut (*display).fonts);
    i = 0 as libc::c_int;
    while i < NUM_GLYPHSETS as libc::c_int {
        let mut info: *mut cairo_xlib_font_glyphset_t = &mut *((*priv_0).glyphset)
            .as_mut_ptr()
            .offset(i as isize) as *mut cairo_xlib_font_glyphset_t;
        match i {
            0 => {
                (*info).format = CAIRO_FORMAT_ARGB32;
            }
            1 => {
                (*info).format = CAIRO_FORMAT_A8;
            }
            2 => {
                (*info).format = CAIRO_FORMAT_A1;
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                            as *const libc::c_char,
                        1009 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 90],
                            &[libc::c_char; 90],
                        >(
                            b"cairo_xlib_font_t *_cairo_xlib_font_create(cairo_xlib_display_t *, cairo_scaled_font_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        let ref mut fresh19 = (*info).xrender_format;
        *fresh19 = 0 as *mut XRenderPictFormat;
        (*info).glyphset = 0 as libc::c_long as GlyphSet;
        (*info).to_free.count = 0 as libc::c_int;
        i += 1;
    }
    return priv_0;
}
unsafe extern "C" fn _cairo_xlib_get_glyphset_index_for_format(
    mut format: cairo_format_t,
) -> libc::c_int {
    if format as libc::c_int == CAIRO_FORMAT_A8 as libc::c_int {
        return GLYPHSET_INDEX_A8 as libc::c_int;
    }
    if format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {
        return GLYPHSET_INDEX_A1 as libc::c_int;
    }
    if format as libc::c_int == CAIRO_FORMAT_ARGB32 as libc::c_int {} else {
        __assert_fail(
            b"format == CAIRO_FORMAT_ARGB32\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                as *const libc::c_char,
            1027 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"int _cairo_xlib_get_glyphset_index_for_format(cairo_format_t)\0"))
                .as_ptr(),
        );
    }
    return GLYPHSET_INDEX_ARGB32 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_xlib_font_get(
    mut display: *const cairo_xlib_display_t,
    mut font: *mut cairo_scaled_font_t,
) -> *mut cairo_xlib_font_t {
    return _cairo_scaled_font_find_private(font, display as *const libc::c_void)
        as *mut cairo_xlib_font_t;
}
unsafe extern "C" fn _cairo_xlib_glyph_fini(
    mut glyph_private: *mut cairo_scaled_glyph_private_t,
    mut glyph: *mut cairo_scaled_glyph_t,
    mut font: *mut cairo_scaled_font_t,
) {
    let mut priv_0: *mut cairo_xlib_glyph_private_t = glyph_private
        as *mut cairo_xlib_glyph_private_t;
    if (*font).finished() == 0 {
        let mut font_private: *mut cairo_xlib_font_t = 0 as *mut cairo_xlib_font_t;
        let mut to_free: *mut _cairo_xlib_font_glyphset_free_glyphs = 0
            as *mut _cairo_xlib_font_glyphset_free_glyphs;
        let mut info: *mut cairo_xlib_font_glyphset_t = 0
            as *mut cairo_xlib_font_glyphset_t;
        font_private = _cairo_xlib_font_get(
            (*glyph_private).key as *const cairo_xlib_display_t,
            font,
        );
        if !font_private.is_null() {} else {
            __assert_fail(
                b"font_private\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                    as *const libc::c_char,
                1058 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 107],
                    &[libc::c_char; 107],
                >(
                    b"void _cairo_xlib_glyph_fini(cairo_scaled_glyph_private_t *, cairo_scaled_glyph_t *, cairo_scaled_font_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        info = (*priv_0).glyphset;
        to_free = &mut (*info).to_free;
        if (*to_free).count
            == (::std::mem::size_of::<[libc::c_ulong; 128]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                as libc::c_int
        {
            let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
            if _cairo_xlib_display_acquire((*font_private).device, &mut display)
                as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                XRenderFreeGlyphs(
                    (*display).display,
                    (*info).glyphset,
                    ((*to_free).indices).as_mut_ptr(),
                    (*to_free).count,
                );
                cairo_device_release(&mut (*display).base);
            }
            (*to_free).count = 0 as libc::c_int;
        }
        let ref mut fresh20 = (*to_free).count;
        let fresh21 = *fresh20;
        *fresh20 = *fresh20 + 1;
        (*to_free).indices[fresh21 as usize] = (*glyph).hash_entry.hash;
    }
    cairo_list_del(&mut (*glyph_private).link);
    free(glyph_private as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xlib_glyph_attach(
    mut display: *mut cairo_xlib_display_t,
    mut glyph: *mut cairo_scaled_glyph_t,
    mut info: *mut cairo_xlib_font_glyphset_t,
) -> cairo_status_t {
    let mut priv_0: *mut cairo_xlib_glyph_private_t = 0
        as *mut cairo_xlib_glyph_private_t;
    priv_0 = (if ::std::mem::size_of::<cairo_xlib_glyph_private_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_glyph_private_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_glyph_private_t;
    if priv_0.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_scaled_glyph_attach_private(
        glyph,
        &mut (*priv_0).base,
        display as *const libc::c_void,
        Some(
            _cairo_xlib_glyph_fini
                as unsafe extern "C" fn(
                    *mut cairo_scaled_glyph_private_t,
                    *mut cairo_scaled_glyph_t,
                    *mut cairo_scaled_font_t,
                ) -> (),
        ),
    );
    let ref mut fresh22 = (*priv_0).glyphset;
    *fresh22 = info;
    let ref mut fresh23 = (*glyph).dev_private;
    *fresh23 = info as *mut libc::c_void;
    let ref mut fresh24 = (*glyph).dev_private_key;
    *fresh24 = display as *const libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xlib_font_get_glyphset_info_for_format(
    mut display: *mut cairo_xlib_display_t,
    mut font: *mut cairo_scaled_font_t,
    mut format: cairo_format_t,
) -> *mut cairo_xlib_font_glyphset_t {
    let mut priv_0: *mut cairo_xlib_font_t = 0 as *mut cairo_xlib_font_t;
    let mut info: *mut cairo_xlib_font_glyphset_t = 0 as *mut cairo_xlib_font_glyphset_t;
    let mut glyphset_index: libc::c_int = 0;
    glyphset_index = _cairo_xlib_get_glyphset_index_for_format(format);
    priv_0 = _cairo_xlib_font_get(display, font);
    if priv_0.is_null() {
        priv_0 = _cairo_xlib_font_create(display, font);
        if priv_0.is_null() {
            return 0 as *mut cairo_xlib_font_glyphset_t;
        }
    }
    info = &mut *((*priv_0).glyphset).as_mut_ptr().offset(glyphset_index as isize)
        as *mut cairo_xlib_font_glyphset_t;
    if (*info).glyphset == 0 as libc::c_long as libc::c_ulong {
        let ref mut fresh25 = (*info).xrender_format;
        *fresh25 = _cairo_xlib_display_get_xrender_format(display, (*info).format);
        (*info)
            .glyphset = XRenderCreateGlyphSet(
            (*display).display,
            (*info).xrender_format,
        );
    }
    return info;
}
unsafe extern "C" fn has_pending_free_glyph(
    mut info: *mut cairo_xlib_font_glyphset_t,
    mut glyph_index: libc::c_ulong,
) -> cairo_bool_t {
    let mut to_free: *mut _cairo_xlib_font_glyphset_free_glyphs = 0
        as *mut _cairo_xlib_font_glyphset_free_glyphs;
    let mut i: libc::c_int = 0;
    to_free = &mut (*info).to_free;
    i = 0 as libc::c_int;
    while i < (*to_free).count {
        if (*to_free).indices[i as usize] == glyph_index {
            let ref mut fresh26 = (*to_free).count;
            *fresh26 -= 1;
            memmove(
                &mut *((*to_free).indices).as_mut_ptr().offset(i as isize)
                    as *mut libc::c_ulong as *mut libc::c_void,
                &mut *((*to_free).indices)
                    .as_mut_ptr()
                    .offset((i + 1 as libc::c_int) as isize) as *mut libc::c_ulong
                    as *const libc::c_void,
                (((*to_free).count - i) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                    ),
            );
            return 1 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_pending_free_glyph(
    mut display: *mut cairo_xlib_display_t,
    mut font: *mut cairo_scaled_font_t,
    mut glyph_index: libc::c_ulong,
    mut surface: *mut cairo_image_surface_t,
) -> *mut cairo_xlib_font_glyphset_t {
    let mut priv_0: *mut cairo_xlib_font_t = 0 as *mut cairo_xlib_font_t;
    let mut i: libc::c_int = 0;
    priv_0 = _cairo_xlib_font_get(display, font);
    if priv_0.is_null() {
        return 0 as *mut cairo_xlib_font_glyphset_t;
    }
    if !surface.is_null() {
        i = _cairo_xlib_get_glyphset_index_for_format((*surface).format);
        if has_pending_free_glyph(
            &mut *((*priv_0).glyphset).as_mut_ptr().offset(i as isize),
            glyph_index,
        ) != 0
        {
            return &mut *((*priv_0).glyphset).as_mut_ptr().offset(i as isize)
                as *mut cairo_xlib_font_glyphset_t;
        }
    } else {
        i = 0 as libc::c_int;
        while i < NUM_GLYPHSETS as libc::c_int {
            if has_pending_free_glyph(
                &mut *((*priv_0).glyphset).as_mut_ptr().offset(i as isize),
                glyph_index,
            ) != 0
            {
                return &mut *((*priv_0).glyphset).as_mut_ptr().offset(i as isize)
                    as *mut cairo_xlib_font_glyphset_t;
            }
            i += 1;
        }
    }
    return 0 as *mut cairo_xlib_font_glyphset_t;
}
unsafe extern "C" fn _cairo_xlib_surface_add_glyph(
    mut display: *mut cairo_xlib_display_t,
    mut font: *mut cairo_scaled_font_t,
    mut pscaled_glyph: *mut *mut cairo_scaled_glyph_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut glyph_info: XGlyphInfo = XGlyphInfo {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        xOff: 0,
        yOff: 0,
    };
    let mut glyph_index: libc::c_ulong = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut glyph: *mut cairo_scaled_glyph_t = *pscaled_glyph;
    let mut glyph_surface: *mut cairo_image_surface_t = (*glyph).surface;
    let mut already_had_glyph_surface: cairo_bool_t = 0;
    let mut info: *mut cairo_xlib_font_glyphset_t = 0 as *mut cairo_xlib_font_glyphset_t;
    glyph_index = (*glyph).hash_entry.hash;
    info = find_pending_free_glyph(display, font, glyph_index, glyph_surface);
    if !info.is_null() {
        return _cairo_xlib_glyph_attach(display, glyph, info);
    }
    if glyph_surface.is_null() {
        status = _cairo_scaled_glyph_lookup(
            font,
            glyph_index,
            (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
                | CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int)
                as cairo_scaled_glyph_info_t,
            0 as *const cairo_color_t,
            pscaled_glyph,
        ) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        glyph = *pscaled_glyph;
        glyph_surface = (*glyph).surface;
        already_had_glyph_surface = 0 as libc::c_int;
    } else {
        already_had_glyph_surface = 1 as libc::c_int;
    }
    info = _cairo_xlib_font_get_glyphset_info_for_format(
        display,
        font,
        (*glyph_surface).format,
    );
    if (*glyph_surface).format as libc::c_int != (*info).format as libc::c_int {
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
        let mut tmp_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        tmp_surface = cairo_image_surface_create(
            (*info).format,
            (*glyph_surface).width,
            (*glyph_surface).height,
        );
        status = (*tmp_surface).status;
        if status as u64 != 0 {
            current_block = 17921409928858268605;
        } else {
            (*tmp_surface).device_transform = (*glyph_surface).base.device_transform;
            (*tmp_surface)
                .device_transform_inverse = (*glyph_surface)
                .base
                .device_transform_inverse;
            _cairo_pattern_init_for_surface(&mut pattern, &mut (*glyph_surface).base);
            status = _cairo_surface_paint(
                tmp_surface,
                CAIRO_OPERATOR_SOURCE,
                &mut pattern.base,
                0 as *const cairo_clip_t,
            );
            _cairo_pattern_fini(&mut pattern.base);
            glyph_surface = tmp_surface as *mut cairo_image_surface_t;
            if status as u64 != 0 {
                current_block = 17921409928858268605;
            } else {
                current_block = 14359455889292382949;
            }
        }
    } else {
        current_block = 14359455889292382949;
    }
    match current_block {
        14359455889292382949 => {
            glyph_info
                .x = _cairo_lround((*glyph_surface).base.device_transform.x0)
                as libc::c_short;
            glyph_info
                .y = _cairo_lround((*glyph_surface).base.device_transform.y0)
                as libc::c_short;
            glyph_info.width = (*glyph_surface).width as libc::c_ushort;
            glyph_info.height = (*glyph_surface).height as libc::c_ushort;
            glyph_info.xOff = (*glyph).x_advance;
            glyph_info.yOff = (*glyph).y_advance;
            data = (*glyph_surface).data;
            match _cairo_xlib_get_glyphset_index_for_format((*(*glyph).surface).format) {
                2 => {
                    if _cairo_is_little_endian()
                        != ((*((*display).display as _XPrivDisplay)).bitmap_bit_order
                            == 0 as libc::c_int) as libc::c_int
                    {
                        let mut c: libc::c_int = ((*glyph_surface).stride
                            * (*glyph_surface).height as libc::c_long) as libc::c_int;
                        let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        let mut new: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        let mut n: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        if c == 0 as libc::c_int {
                            current_block = 12961834331865314435;
                        } else {
                            new = (if c != 0 as libc::c_int {
                                malloc(c as libc::c_ulong)
                            } else {
                                0 as *mut libc::c_void
                            }) as *mut libc::c_uchar;
                            if new.is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                current_block = 17921409928858268605;
                            } else {
                                n = new;
                                d = data;
                                loop {
                                    let fresh27 = d;
                                    d = d.offset(1);
                                    let mut b: libc::c_char = *fresh27 as libc::c_char;
                                    b = ((b as libc::c_int) << 1 as libc::c_int
                                        & 0xaa as libc::c_int
                                        | b as libc::c_int >> 1 as libc::c_int
                                            & 0x55 as libc::c_int) as libc::c_char;
                                    b = ((b as libc::c_int) << 2 as libc::c_int
                                        & 0xcc as libc::c_int
                                        | b as libc::c_int >> 2 as libc::c_int
                                            & 0x33 as libc::c_int) as libc::c_char;
                                    b = ((b as libc::c_int) << 4 as libc::c_int
                                        & 0xf0 as libc::c_int
                                        | b as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                                        as libc::c_char;
                                    let fresh28 = n;
                                    n = n.offset(1);
                                    *fresh28 = b as libc::c_uchar;
                                    c -= 1;
                                    if !(c != 0) {
                                        break;
                                    }
                                }
                                data = new;
                                current_block = 12961834331865314435;
                            }
                        }
                    } else {
                        current_block = 12961834331865314435;
                    }
                }
                1 => {
                    current_block = 12961834331865314435;
                }
                0 => {
                    if _cairo_is_little_endian()
                        != ((*((*display).display as _XPrivDisplay)).byte_order
                            == 0 as libc::c_int) as libc::c_int
                    {
                        let mut c_0: libc::c_uint = ((*glyph_surface).stride
                            * (*glyph_surface).height as libc::c_long
                            / 4 as libc::c_int as libc::c_long) as libc::c_uint;
                        let mut d_0: *const uint32_t = 0 as *const uint32_t;
                        let mut new_0: *mut uint32_t = 0 as *mut uint32_t;
                        let mut n_0: *mut uint32_t = 0 as *mut uint32_t;
                        if c_0 == 0 as libc::c_int as libc::c_uint {
                            current_block = 12961834331865314435;
                        } else {
                            new_0 = (if (4 as libc::c_int as libc::c_uint)
                                .wrapping_mul(c_0) != 0 as libc::c_int as libc::c_uint
                            {
                                malloc(
                                    (4 as libc::c_int as libc::c_uint).wrapping_mul(c_0)
                                        as libc::c_ulong,
                                )
                            } else {
                                0 as *mut libc::c_void
                            }) as *mut uint32_t;
                            if new_0.is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                current_block = 17921409928858268605;
                            } else {
                                n_0 = new_0;
                                d_0 = data as *mut uint32_t;
                                loop {
                                    let fresh29 = n_0;
                                    n_0 = n_0.offset(1);
                                    *fresh29 = __bswap_32(*d_0);
                                    d_0 = d_0.offset(1);
                                    c_0 = c_0.wrapping_sub(1);
                                    if !(c_0 != 0) {
                                        break;
                                    }
                                }
                                data = new_0 as *mut uint8_t;
                                current_block = 12961834331865314435;
                            }
                        }
                    } else {
                        current_block = 12961834331865314435;
                    }
                }
                _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                                as *const libc::c_char,
                            1336 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 117],
                                &[libc::c_char; 117],
                            >(
                                b"cairo_status_t _cairo_xlib_surface_add_glyph(cairo_xlib_display_t *, cairo_scaled_font_t *, cairo_scaled_glyph_t **)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    current_block = 12961834331865314435;
                }
            }
            match current_block {
                17921409928858268605 => {}
                _ => {
                    XRenderAddGlyphs(
                        (*display).display,
                        (*info).glyphset,
                        &mut glyph_index,
                        &mut glyph_info,
                        1 as libc::c_int,
                        data as *mut libc::c_char,
                        ((*glyph_surface).stride
                            * (*glyph_surface).height as libc::c_long) as libc::c_int,
                    );
                    if data != (*glyph_surface).data {
                        free(data as *mut libc::c_void);
                    }
                    status = _cairo_xlib_glyph_attach(display, glyph, info);
                }
            }
        }
        _ => {}
    }
    if glyph_surface != (*glyph).surface {
        cairo_surface_destroy(&mut (*glyph_surface).base);
    }
    if already_had_glyph_surface == 0 {
        _cairo_scaled_glyph_set_surface(glyph, font, 0 as *mut cairo_image_surface_t);
    }
    return status;
}
unsafe extern "C" fn _emit_glyphs_chunk(
    mut display: *mut cairo_xlib_display_t,
    mut dst: *mut cairo_xlib_surface_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut glyphs: *mut cairo_xlib_glyph_t,
    mut num_glyphs: libc::c_int,
    mut font: *mut cairo_scaled_font_t,
    mut use_mask: cairo_bool_t,
    mut op: cairo_operator_t,
    mut src: *mut cairo_xlib_source_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut num_elts: libc::c_int,
    mut width: libc::c_int,
    mut info: *mut cairo_xlib_font_glyphset_t,
) -> cairo_status_t {
    let mut composite_text_func: cairo_xrender_composite_text_func_t = None;
    let mut size: libc::c_int = 0;
    let mut elts: *mut XGlyphElt8 = 0 as *mut XGlyphElt8;
    let mut stack_elts: [XGlyphElt8; 64] = [XGlyphElt8 {
        glyphset: 0,
        chars: 0 as *const libc::c_char,
        nchars: 0,
        xOff: 0,
        yOff: 0,
    }; 64];
    let mut char8: *mut libc::c_char = glyphs as *mut libc::c_char;
    let mut char16: *mut libc::c_ushort = glyphs as *mut libc::c_ushort;
    let mut char32: *mut libc::c_uint = glyphs as *mut libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut nelt: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    match width {
        1 => {
            composite_text_func = Some(
                XRenderCompositeText8
                    as unsafe extern "C" fn(
                        *mut Display,
                        libc::c_int,
                        Picture,
                        Picture,
                        *const XRenderPictFormat,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const XGlyphElt8,
                        libc::c_int,
                    ) -> (),
            );
            size = ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int;
        }
        2 => {
            composite_text_func = ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Display,
                        libc::c_int,
                        Picture,
                        Picture,
                        *const XRenderPictFormat,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const XGlyphElt16,
                        libc::c_int,
                    ) -> (),
                >,
                cairo_xrender_composite_text_func_t,
            >(
                Some(
                    XRenderCompositeText16
                        as unsafe extern "C" fn(
                            *mut Display,
                            libc::c_int,
                            Picture,
                            Picture,
                            *const XRenderPictFormat,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            *const XGlyphElt16,
                            libc::c_int,
                        ) -> (),
                ),
            );
            size = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong
                as libc::c_int;
        }
        4 | _ => {
            composite_text_func = ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Display,
                        libc::c_int,
                        Picture,
                        Picture,
                        *const XRenderPictFormat,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const XGlyphElt32,
                        libc::c_int,
                    ) -> (),
                >,
                cairo_xrender_composite_text_func_t,
            >(
                Some(
                    XRenderCompositeText32
                        as unsafe extern "C" fn(
                            *mut Display,
                            libc::c_int,
                            Picture,
                            Picture,
                            *const XRenderPictFormat,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            *const XGlyphElt32,
                            libc::c_int,
                        ) -> (),
                ),
            );
            size = ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int;
        }
    }
    if num_elts
        <= (::std::mem::size_of::<[XGlyphElt8; 64]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<XGlyphElt8>() as libc::c_ulong)
            as libc::c_int
    {
        elts = stack_elts.as_mut_ptr();
    } else {
        elts = _cairo_malloc_ab(
            num_elts as size_t,
            ::std::mem::size_of::<XGlyphElt8>() as libc::c_ulong,
        ) as *mut XGlyphElt8;
        if elts.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    nelt = 0 as libc::c_int;
    n = 0 as libc::c_int;
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_glyphs {
        if j & 127 as libc::c_int == 0 as libc::c_int
            || (*glyphs.offset(i as isize)).i.x != 0
            || (*glyphs.offset(i as isize)).i.y != 0
        {
            if j != 0 {
                (*elts.offset(nelt as isize)).nchars = n;
                nelt += 1;
                n = 0 as libc::c_int;
            }
            let ref mut fresh30 = (*elts.offset(nelt as isize)).chars;
            *fresh30 = char8.offset((size * j) as isize);
            (*elts.offset(nelt as isize)).glyphset = (*info).glyphset;
            (*elts.offset(nelt as isize)).xOff = (*glyphs.offset(i as isize)).i.x;
            (*elts.offset(nelt as isize)).yOff = (*glyphs.offset(i as isize)).i.y;
        }
        match width {
            1 => {
                *char8
                    .offset(
                        j as isize,
                    ) = (*glyphs.offset(i as isize)).index as libc::c_char;
            }
            2 => {
                *char16
                    .offset(
                        j as isize,
                    ) = (*glyphs.offset(i as isize)).index as libc::c_ushort;
            }
            4 | _ => {
                *char32
                    .offset(
                        j as isize,
                    ) = (*glyphs.offset(i as isize)).index as libc::c_uint;
            }
        }
        n += 1;
        j += 1;
        i += 1;
    }
    if n != 0 {
        (*elts.offset(nelt as isize)).nchars = n;
        nelt += 1;
    }
    if nelt == num_elts {} else {
        __assert_fail(
            b"nelt == num_elts\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-render-compositor.c\0" as *const u8
                as *const libc::c_char,
            1507 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 247],
                &[libc::c_char; 247],
            >(
                b"cairo_status_t _emit_glyphs_chunk(cairo_xlib_display_t *, cairo_xlib_surface_t *, int, int, cairo_xlib_glyph_t *, int, cairo_scaled_font_t *, cairo_bool_t, cairo_operator_t, cairo_xlib_source_t *, int, int, int, int, cairo_xlib_font_glyphset_t *)\0",
            ))
                .as_ptr(),
        );
    }
    composite_text_func
        .expect(
            "non-null function pointer",
        )(
        (*display).display,
        op as libc::c_int,
        (*src).picture,
        (*dst).picture,
        if use_mask != 0 { (*info).xrender_format } else { 0 as *mut XRenderPictFormat },
        src_x + (*elts.offset(0 as libc::c_int as isize)).xOff + dst_x,
        src_y + (*elts.offset(0 as libc::c_int as isize)).yOff + dst_y,
        (*elts.offset(0 as libc::c_int as isize)).xOff,
        (*elts.offset(0 as libc::c_int as isize)).yOff,
        elts,
        nelt,
    );
    if elts != stack_elts.as_mut_ptr() {
        free(elts as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn check_composite_glyphs(
    mut extents: *const cairo_composite_rectangles_t,
    mut font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: *mut libc::c_int,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = (*extents).surface
        as *mut cairo_xlib_surface_t;
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut max_request_size: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if !((*extents).op as libc::c_uint
        <= CAIRO_OPERATOR_SATURATE as libc::c_int as libc::c_uint
        || ((*display).render_major > 0 as libc::c_int
            || (*display).render_major == 0 as libc::c_int
                && (*display).render_minor >= 11 as libc::c_int)
            && (*extents).op as libc::c_uint
                <= CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*extents).bounded.x + (*extents).bounded.width > 32767 as libc::c_int
        || (*extents).bounded.y + (*extents).bounded.height > 32767 as libc::c_int
        || (*extents).bounded.x < -(32767 as libc::c_int) - 1 as libc::c_int
        || (*extents).bounded.y < -(32767 as libc::c_int) - 1 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    size = ceil((*font).max_scale) as libc::c_int;
    size = 4 as libc::c_int * size * size;
    max_request_size = ((if XExtendedMaxRequestSize((*display).display) != 0 {
        XExtendedMaxRequestSize((*display).display)
    } else {
        XMaxRequestSize((*display).display)
    }) * 4 as libc::c_int as libc::c_long - 12 as libc::c_int as libc::c_long
        - 12 as libc::c_int as libc::c_long - 8 as libc::c_int as libc::c_long)
        as libc::c_int;
    if size >= max_request_size {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_glyphs(
    mut surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut _src: *mut cairo_surface_t,
    mut src_x: libc::c_int,
    mut src_y: libc::c_int,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut info: *mut cairo_composite_glyphs_info_t,
) -> cairo_int_status_t {
    let mut dst: *mut cairo_xlib_surface_t = surface as *mut cairo_xlib_surface_t;
    let mut glyphs: *mut cairo_xlib_glyph_t = (*info).glyphs as *mut cairo_xlib_glyph_t;
    let mut src: *mut cairo_xlib_source_t = _src as *mut cairo_xlib_source_t;
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
    let mut x: cairo_fixed_t = dst_x;
    let mut y: cairo_fixed_t = dst_y;
    let mut glyphset: *mut cairo_xlib_font_glyphset_t = 0
        as *mut cairo_xlib_font_glyphset_t;
    let mut this_glyphset_info: *mut cairo_xlib_font_glyphset_t = 0
        as *mut cairo_xlib_font_glyphset_t;
    let mut max_index: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut width: libc::c_int = 1 as libc::c_int;
    let mut num_elts: libc::c_int = 0 as libc::c_int;
    let mut num_out_glyphs: libc::c_int = 0 as libc::c_int;
    let mut num_glyphs: libc::c_int = (*info).num_glyphs;
    let mut max_request_size: libc::c_int = (XMaxRequestSize((*display).display)
        * 4 as libc::c_int as libc::c_long
        - (if 28 as libc::c_int
            > (if 28 as libc::c_int > 28 as libc::c_int {
                28 as libc::c_int
            } else {
                28 as libc::c_int
            })
        {
            28 as libc::c_int
        } else {
            (if 28 as libc::c_int > 28 as libc::c_int {
                28 as libc::c_int
            } else {
                28 as libc::c_int
            })
        }) as libc::c_long) as libc::c_int;
    let mut request_size: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    op = _render_operator(op) as cairo_operator_t;
    _cairo_xlib_surface_ensure_picture(dst);
    i = 0 as libc::c_int;
    while i < num_glyphs {
        let mut xphase: libc::c_ulong = 0;
        let mut yphase: libc::c_ulong = 0;
        let mut this_x: libc::c_int = 0;
        let mut this_y: libc::c_int = 0;
        let mut old_width: libc::c_int = 0;
        xphase = (floor(
            4 as libc::c_int as libc::c_double
                * ((*glyphs.offset(i as isize)).d.x + 0.125f64),
        )
            - 4 as libc::c_int as libc::c_double
                * floor((*glyphs.offset(i as isize)).d.x + 0.125f64)) as libc::c_int
            as libc::c_ulong;
        yphase = (floor(
            4 as libc::c_int as libc::c_double
                * ((*glyphs.offset(i as isize)).d.y + 0.125f64),
        )
            - 4 as libc::c_int as libc::c_double
                * floor((*glyphs.offset(i as isize)).d.y + 0.125f64)) as libc::c_int
            as libc::c_ulong;
        (*glyphs.offset(i as isize)).index
            |= xphase << 24 as libc::c_int | yphase << 26 as libc::c_int;
        status = _cairo_scaled_glyph_lookup(
            (*info).font,
            (*glyphs.offset(i as isize)).index,
            CAIRO_SCALED_GLYPH_INFO_METRICS,
            0 as *const cairo_color_t,
            &mut glyph,
        );
        if status as u64 != 0 {
            return status;
        }
        this_x = floor((*glyphs.offset(i as isize)).d.x + 0.125f64) as libc::c_int;
        this_y = floor((*glyphs.offset(i as isize)).d.y + 0.125f64) as libc::c_int;
        if (*glyph).dev_private_key != display as *const libc::c_void {
            status = _cairo_xlib_surface_add_glyph(display, (*info).font, &mut glyph)
                as cairo_int_status_t;
            if status as u64 != 0 {
                return status;
            }
        }
        this_glyphset_info = (*glyph).dev_private as *mut cairo_xlib_font_glyphset_t;
        if glyphset.is_null() {
            glyphset = this_glyphset_info;
        }
        old_width = width;
        if (*glyphs.offset(i as isize)).index > max_index {
            max_index = (*glyphs.offset(i as isize)).index;
            if max_index >= 65536 as libc::c_int as libc::c_ulong {
                width = 4 as libc::c_int;
            } else if max_index >= 256 as libc::c_int as libc::c_ulong {
                width = 2 as libc::c_int;
            }
            if width != old_width {
                request_size += (width - old_width) * num_out_glyphs;
            }
        }
        if request_size + width
            > max_request_size - (8 as libc::c_int + 4 as libc::c_int)
            || this_x - x > 32767 as libc::c_int
            || this_x - x < -(32767 as libc::c_int) - 1 as libc::c_int
            || this_y - y > 32767 as libc::c_int
            || this_y - y < -(32767 as libc::c_int) - 1 as libc::c_int
            || this_glyphset_info != glyphset
        {
            status = _emit_glyphs_chunk(
                display,
                dst,
                dst_x,
                dst_y,
                glyphs,
                i,
                (*info).font,
                (*info).use_mask,
                op,
                src,
                src_x,
                src_y,
                num_elts,
                old_width,
                glyphset,
            ) as cairo_int_status_t;
            if status as u64 != 0 {
                return status;
            }
            glyphs = glyphs.offset(i as isize);
            num_glyphs -= i;
            i = 0 as libc::c_int;
            max_index = (*glyphs.offset(i as isize)).index;
            width = if max_index < 256 as libc::c_int as libc::c_ulong {
                1 as libc::c_int
            } else if max_index < 65536 as libc::c_int as libc::c_ulong {
                2 as libc::c_int
            } else {
                4 as libc::c_int
            };
            request_size = 0 as libc::c_int;
            num_elts = 0 as libc::c_int;
            num_out_glyphs = 0 as libc::c_int;
            y = 0 as libc::c_int;
            x = y;
            glyphset = this_glyphset_info;
        }
        (*glyphs.offset(i as isize)).i.x = this_x - x;
        (*glyphs.offset(i as isize)).i.y = this_y - y;
        if num_out_glyphs & 127 as libc::c_int == 0 as libc::c_int
            || (*glyphs.offset(i as isize)).i.x != 0
            || (*glyphs.offset(i as isize)).i.y != 0
        {
            num_elts += 1;
            request_size += 8 as libc::c_int + 4 as libc::c_int;
        }
        x = this_x + (*glyph).x_advance as libc::c_int;
        y = this_y + (*glyph).y_advance as libc::c_int;
        num_out_glyphs += 1;
        request_size += width;
        i += 1;
    }
    if num_elts != 0 {
        status = _emit_glyphs_chunk(
            display,
            dst,
            dst_x,
            dst_y,
            glyphs,
            i,
            (*info).font,
            (*info).use_mask,
            op,
            src,
            src_x,
            src_y,
            num_elts,
            width,
            glyphset,
        ) as cairo_int_status_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_mask_compositor_get() -> *const cairo_compositor_t {
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
            _cairo_xlib_fallback_compositor_get(),
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
            _cairo_xlib_source_create_for_pattern
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
            .copy_boxes = Some(
            copy_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_surface_t,
                    *mut cairo_boxes_t,
                    *const cairo_rectangle_int_t,
                    libc::c_int,
                    libc::c_int,
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
unsafe extern "C" fn line_exceeds_16_16(mut line: *const cairo_line_t) -> cairo_bool_t {
    return ((*line).p1.x < _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p1.x > _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p2.x < _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p2.x > _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p1.y < _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p1.y > _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p2.y < _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p2.y > _cairo_fixed_from_int(32767 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn project_line_x_onto_16_16(
    mut line: *const cairo_line_t,
    mut top: cairo_fixed_t,
    mut bottom: cairo_fixed_t,
    mut out: *mut XLineFixed,
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
unsafe extern "C" fn composite_traps(
    mut abstract_dst: *mut libc::c_void,
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
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut src: *mut cairo_xlib_source_t = abstract_src as *mut cairo_xlib_source_t;
    let mut pict_format: *mut XRenderPictFormat = 0 as *mut XRenderPictFormat;
    let mut xtraps_stack: [XTrapezoid; 51] = [XTrapezoid {
        top: 0,
        bottom: 0,
        left: XLineFixed {
            p1: XPointFixed { x: 0, y: 0 },
            p2: XPointFixed { x: 0, y: 0 },
        },
        right: XLineFixed {
            p1: XPointFixed { x: 0, y: 0 },
            p2: XPointFixed { x: 0, y: 0 },
        },
    }; 51];
    let mut xtraps: *mut XTrapezoid = xtraps_stack.as_mut_ptr();
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*traps).num_traps == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if ((*dst).base).is_clear() as libc::c_int != 0
        && (op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint)
    {
        op = CAIRO_OPERATOR_SOURCE;
    }
    pict_format = _cairo_xlib_display_get_xrender_format(
        display,
        (if antialias as libc::c_uint
            == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        {
            CAIRO_FORMAT_A1 as libc::c_int
        } else {
            CAIRO_FORMAT_A8 as libc::c_int
        }) as cairo_format_t,
    );
    if (*traps).num_traps
        > (::std::mem::size_of::<[XTrapezoid; 51]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<XTrapezoid>() as libc::c_ulong)
            as libc::c_int
    {
        xtraps = _cairo_malloc_ab(
            (*traps).num_traps as size_t,
            ::std::mem::size_of::<XTrapezoid>() as libc::c_ulong,
        ) as *mut XTrapezoid;
        if xtraps.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    dx = -dst_x << 16 as libc::c_int;
    dy = -dst_y << 16 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        let mut t: *mut cairo_trapezoid_t = &mut *((*traps).traps).offset(i as isize)
            as *mut cairo_trapezoid_t;
        (*xtraps.offset(i as isize)).top = _cairo_fixed_to_16_16((*t).top) + dy;
        (*xtraps.offset(i as isize)).bottom = _cairo_fixed_to_16_16((*t).bottom) + dy;
        if line_exceeds_16_16(&mut (*t).left) != 0 {
            project_line_x_onto_16_16(
                &mut (*t).left,
                (*t).top,
                (*t).bottom,
                &mut (*xtraps.offset(i as isize)).left,
            );
            let ref mut fresh31 = (*xtraps.offset(i as isize)).left.p1.x;
            *fresh31 += dx;
            let ref mut fresh32 = (*xtraps.offset(i as isize)).left.p2.x;
            *fresh32 += dx;
            (*xtraps.offset(i as isize)).left.p1.y = (*xtraps.offset(i as isize)).top;
            (*xtraps.offset(i as isize)).left.p2.y = (*xtraps.offset(i as isize)).bottom;
        } else {
            (*xtraps.offset(i as isize))
                .left
                .p1
                .x = _cairo_fixed_to_16_16((*t).left.p1.x) + dx;
            (*xtraps.offset(i as isize))
                .left
                .p1
                .y = _cairo_fixed_to_16_16((*t).left.p1.y) + dy;
            (*xtraps.offset(i as isize))
                .left
                .p2
                .x = _cairo_fixed_to_16_16((*t).left.p2.x) + dx;
            (*xtraps.offset(i as isize))
                .left
                .p2
                .y = _cairo_fixed_to_16_16((*t).left.p2.y) + dy;
        }
        if line_exceeds_16_16(&mut (*t).right) != 0 {
            project_line_x_onto_16_16(
                &mut (*t).right,
                (*t).top,
                (*t).bottom,
                &mut (*xtraps.offset(i as isize)).right,
            );
            let ref mut fresh33 = (*xtraps.offset(i as isize)).right.p1.x;
            *fresh33 += dx;
            let ref mut fresh34 = (*xtraps.offset(i as isize)).right.p2.x;
            *fresh34 += dx;
            (*xtraps.offset(i as isize)).right.p1.y = (*xtraps.offset(i as isize)).top;
            (*xtraps.offset(i as isize))
                .right
                .p2
                .y = (*xtraps.offset(i as isize)).bottom;
        } else {
            (*xtraps.offset(i as isize))
                .right
                .p1
                .x = _cairo_fixed_to_16_16((*t).right.p1.x) + dx;
            (*xtraps.offset(i as isize))
                .right
                .p1
                .y = _cairo_fixed_to_16_16((*t).right.p1.y) + dy;
            (*xtraps.offset(i as isize))
                .right
                .p2
                .x = _cairo_fixed_to_16_16((*t).right.p2.x) + dx;
            (*xtraps.offset(i as isize))
                .right
                .p2
                .y = _cairo_fixed_to_16_16((*t).right.p2.y) + dy;
        }
        i += 1;
    }
    if (*xtraps.offset(0 as libc::c_int as isize)).left.p1.y
        < (*xtraps.offset(0 as libc::c_int as isize)).left.p2.y
    {
        src_x
            += _cairo_fixed_16_16_floor(
                (*xtraps.offset(0 as libc::c_int as isize)).left.p1.x,
            );
        src_y
            += _cairo_fixed_16_16_floor(
                (*xtraps.offset(0 as libc::c_int as isize)).left.p1.y,
            );
    } else {
        src_x
            += _cairo_fixed_16_16_floor(
                (*xtraps.offset(0 as libc::c_int as isize)).left.p2.x,
            );
        src_y
            += _cairo_fixed_16_16_floor(
                (*xtraps.offset(0 as libc::c_int as isize)).left.p2.y,
            );
    }
    src_x += dst_x;
    src_y += dst_y;
    _cairo_xlib_surface_ensure_picture(dst);
    _cairo_xlib_surface_set_precision(dst, antialias);
    XRenderCompositeTrapezoids(
        (*dst).dpy,
        _render_operator(op),
        (*src).picture,
        (*dst).picture,
        pict_format,
        src_x,
        src_y,
        xtraps,
        (*traps).num_traps,
    );
    if xtraps != xtraps_stack.as_mut_ptr() {
        free(xtraps as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_tristrip(
    mut abstract_dst: *mut libc::c_void,
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
    let mut dst: *mut cairo_xlib_surface_t = abstract_dst as *mut cairo_xlib_surface_t;
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut src: *mut cairo_xlib_source_t = abstract_src as *mut cairo_xlib_source_t;
    let mut pict_format: *mut XRenderPictFormat = 0 as *mut XRenderPictFormat;
    let mut points_stack: [XPointFixed; 256] = [XPointFixed { x: 0, y: 0 }; 256];
    let mut points: *mut XPointFixed = points_stack.as_mut_ptr();
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    pict_format = _cairo_xlib_display_get_xrender_format(
        display,
        (if antialias as libc::c_uint
            == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        {
            CAIRO_FORMAT_A1 as libc::c_int
        } else {
            CAIRO_FORMAT_A8 as libc::c_int
        }) as cairo_format_t,
    );
    if (*strip).num_points
        > (::std::mem::size_of::<[XPointFixed; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<XPointFixed>() as libc::c_ulong)
            as libc::c_int
    {
        points = _cairo_malloc_ab(
            (*strip).num_points as size_t,
            ::std::mem::size_of::<XPointFixed>() as libc::c_ulong,
        ) as *mut XPointFixed;
        if points.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    dx = -dst_x << 16 as libc::c_int;
    dy = -dst_y << 16 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*strip).num_points {
        let mut p: *mut cairo_point_t = &mut *((*strip).points).offset(i as isize)
            as *mut cairo_point_t;
        (*points.offset(i as isize)).x = _cairo_fixed_to_16_16((*p).x) + dx;
        (*points.offset(i as isize)).y = _cairo_fixed_to_16_16((*p).y) + dy;
        i += 1;
    }
    src_x
        += _cairo_fixed_16_16_floor((*points.offset(0 as libc::c_int as isize)).x)
            + dst_x;
    src_y
        += _cairo_fixed_16_16_floor((*points.offset(0 as libc::c_int as isize)).y)
            + dst_y;
    _cairo_xlib_surface_ensure_picture(dst);
    _cairo_xlib_surface_set_precision(dst, antialias);
    XRenderCompositeTriStrip(
        (*dst).dpy,
        _render_operator(op),
        (*src).picture,
        (*dst).picture,
        pict_format,
        src_x,
        src_y,
        points,
        (*strip).num_points,
    );
    if points != points_stack.as_mut_ptr() {
        free(points as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_traps_compositor_get() -> *const cairo_compositor_t {
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
        _cairo_traps_compositor_init(&mut compositor, _cairo_xlib_mask_compositor_get());
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
            _cairo_xlib_source_create_for_pattern
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
            .copy_boxes = Some(
            copy_boxes
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_surface_t,
                    *mut cairo_boxes_t,
                    *const cairo_rectangle_int_t,
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
