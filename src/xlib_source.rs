use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _XGC;
    pub type _XDisplay;
    pub type _cairo_xlib_shm_display;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_matrix_init_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_flush(
        surface: *mut cairo_surface_t,
        flags: libc::c_uint,
    ) -> cairo_status_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_surface_create_scratch(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
        color: *const cairo_color_t,
    ) -> *mut cairo_surface_t;
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
        source_0: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_map_to_image(
        surface: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_surface_unmap_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn _cairo_matrix_to_pixman_matrix_offset(
        matrix: *const cairo_matrix_t,
        filter: cairo_filter_t,
        xc: libc::c_double,
        yc: libc::c_double,
        out_transform: *mut pixman_transform_t,
        out_x_offset: *mut libc::c_int,
        out_y_offset: *mut libc::c_int,
    ) -> cairo_status_t;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;
    fn _cairo_xlib_shm_surface_mark_active(shm: *mut cairo_surface_t);
    fn _cairo_xlib_shm_surface_get_pixmap(surface: *mut cairo_surface_t) -> Pixmap;
    fn _cairo_xlib_shm_surface_get_xrender_format(
        surface: *mut cairo_surface_t,
    ) -> *mut XRenderPictFormat;
    fn _cairo_xlib_surface_put_shm(
        surface: *mut cairo_xlib_surface_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_xlib_surface_ensure_picture(surface: *mut cairo_xlib_surface_t);
    fn _cairo_xlib_screen_put_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        gc: GC,
    );
    fn _cairo_xlib_screen_get_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        drawable: Drawable,
    ) -> GC;
    fn _cairo_xlib_display_get_xrender_format(
        display: *mut cairo_xlib_display_t,
        format: cairo_format_t,
    ) -> *mut XRenderPictFormat;
    fn XRenderCreateRadialGradient(
        dpy: *mut Display,
        gradient: *const XRadialGradient,
        stops: *const XFixed,
        colors: *const XRenderColor,
        nstops: libc::c_int,
    ) -> Picture;
    fn XRenderCreateLinearGradient(
        dpy: *mut Display,
        gradient: *const XLinearGradient,
        stops: *const XFixed,
        colors: *const XRenderColor,
        nstops: libc::c_int,
    ) -> Picture;
    fn XRenderCreateSolidFill(dpy: *mut Display, color: *const XRenderColor) -> Picture;
    fn XRenderSetPictureFilter(
        dpy: *mut Display,
        picture: Picture,
        filter: *const libc::c_char,
        params: *mut XFixed,
        nparams: libc::c_int,
    );
    fn XRenderFillRectangles(
        dpy: *mut Display,
        op: libc::c_int,
        dst: Picture,
        color: *const XRenderColor,
        rectangles: *const XRectangle,
        n_rects: libc::c_int,
    );
    fn XRenderFreePicture(dpy: *mut Display, picture: Picture);
    fn XRenderSetPictureTransform(
        dpy: *mut Display,
        picture: Picture,
        transform: *mut XTransform,
    );
    fn XRenderChangePicture(
        dpy: *mut Display,
        picture: Picture,
        valuemask: libc::c_ulong,
        attributes: *const XRenderPictureAttributes,
    );
    fn XRenderCreatePicture(
        dpy: *mut Display,
        drawable: Drawable,
        format: *const XRenderPictFormat,
        valuemask: libc::c_ulong,
        attributes: *const XRenderPictureAttributes,
    ) -> Picture;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XChangeGC(
        _: *mut Display,
        _: GC,
        _: libc::c_ulong,
        _: *mut XGCValues,
    ) -> libc::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    fn _cairo_paginated_surface_get_recording(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_is_paginated(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn _cairo_pattern_init_static_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    );
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_gradient_pattern_fit_to_range(
        gradient: *const cairo_gradient_pattern_t,
        max_value: libc::c_double,
        out_matrix: *mut cairo_matrix_t,
        out_circle: *mut cairo_circle_double_t,
    );
    fn _cairo_radial_pattern_focus_is_inside(
        radial: *const cairo_radial_pattern_t,
    ) -> cairo_bool_t;
    fn _cairo_recording_surface_replay_with_clip(
        surface: *mut cairo_surface_t,
        surface_transform: *const cairo_matrix_t,
        target: *mut cairo_surface_t,
        target_clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_paint(
        target: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source_0: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_subsurface_set_snapshot(
        surface: *mut cairo_surface_t,
        snapshot: *mut cairo_surface_t,
    );
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type pixman_fixed_16_16_t = int32_t;
pub type pixman_fixed_t = pixman_fixed_16_16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_transform {
    pub matrix: [[pixman_fixed_t; 3]; 3],
}
pub type pixman_transform_t = pixman_transform;
pub type cairo_fixed_16_16_t = int32_t;
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
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
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
pub struct _XCircle {
    pub x: XFixed,
    pub y: XFixed,
    pub radius: XFixed,
}
pub type XCircle = _XCircle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XTransform {
    pub matrix: [[XFixed; 3]; 3],
}
pub type XTransform = _XTransform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XLinearGradient {
    pub p1: XPointFixed,
    pub p2: XPointFixed,
}
pub type XLinearGradient = _XLinearGradient;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRadialGradient {
    pub inner: XCircle,
    pub outer: XCircle,
}
pub type XRadialGradient = _XRadialGradient;
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
pub struct _cairo_xlib_proxy {
    pub source: _cairo_xlib_source,
    pub owner: *mut cairo_surface_t,
}
pub type cairo_xlib_proxy_t = _cairo_xlib_proxy;
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
pub type cairo_radial_pattern_t = _cairo_radial_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_radial_pattern {
    pub base: cairo_gradient_pattern_t,
    pub cd1: cairo_circle_double_t,
    pub cd2: cairo_circle_double_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline]
unsafe extern "C" fn _cairo_matrix_is_translation(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_16_16_from_double(
    mut d: libc::c_double,
) -> cairo_fixed_16_16_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
    u.d = d + 103079215104.0f64;
    return u.i[0 as libc::c_int as usize];
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
unsafe extern "C" fn _cairo_xlib_surface_same_screen(
    mut dst: *mut cairo_xlib_surface_t,
    mut src: *mut cairo_xlib_surface_t,
) -> cairo_bool_t {
    return ((*dst).screen == (*src).screen) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_image(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return (!((*surface).backend).is_null()
        && (*(*surface).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_pattern_get_source(
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    return _cairo_surface_get_source((*pattern).surface, extents);
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
#[inline]
unsafe extern "C" fn _cairo_surface_is_snapshot(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT as libc::c_int as cairo_surface_type_t
            as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_subsurface(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn unwrap_source(
    mut pattern: *const cairo_surface_pattern_t,
) -> *mut cairo_xlib_surface_t {
    let mut limits: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    return _cairo_pattern_get_source(pattern, &mut limits) as *mut cairo_xlib_surface_t;
}
unsafe extern "C" fn _cairo_xlib_source_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut source_0: *mut cairo_xlib_source_t = abstract_surface
        as *mut cairo_xlib_source_t;
    XRenderFreePicture((*source_0).dpy, (*source_0).picture);
    if (*source_0).pixmap != 0 {
        XFreePixmap((*source_0).dpy, (*source_0).pixmap);
    }
    return CAIRO_STATUS_SUCCESS;
}
static mut cairo_xlib_source_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_XLIB,
            finish: Some(
                _cairo_xlib_source_finish
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
unsafe extern "C" fn _cairo_xlib_proxy_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut proxy: *mut cairo_xlib_proxy_t = abstract_surface as *mut cairo_xlib_proxy_t;
    _cairo_xlib_shm_surface_mark_active((*proxy).owner);
    XRenderFreePicture((*proxy).source.dpy, (*proxy).source.picture);
    if (*proxy).source.pixmap != 0 {
        XFreePixmap((*proxy).source.dpy, (*proxy).source.pixmap);
    }
    cairo_surface_destroy((*proxy).owner);
    return CAIRO_STATUS_SUCCESS;
}
static mut cairo_xlib_proxy_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_XLIB,
            finish: Some(
                _cairo_xlib_proxy_finish
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
unsafe extern "C" fn source(
    mut dst: *mut cairo_xlib_surface_t,
    mut picture: Picture,
    mut pixmap: Pixmap,
) -> *mut cairo_surface_t {
    let mut source_0: *mut cairo_xlib_source_t = 0 as *mut cairo_xlib_source_t;
    if picture == 0 as libc::c_long as libc::c_ulong {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    source_0 = (if ::std::mem::size_of::<cairo_xlib_source_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_source_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_source_t;
    if source_0.is_null() {
        XRenderFreePicture((*(*dst).display).display, picture);
        if pixmap != 0 {
            XFreePixmap((*(*dst).display).display, pixmap);
        }
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*source_0).base,
        &cairo_xlib_source_backend,
        0 as *mut cairo_device_t,
        CAIRO_CONTENT_COLOR_ALPHA,
        0 as libc::c_int,
    );
    (*source_0).picture = picture;
    (*source_0).pixmap = pixmap;
    let ref mut fresh2 = (*source_0).dpy;
    *fresh2 = (*(*dst).display).display;
    return &mut (*source_0).base;
}
unsafe extern "C" fn hars_petruska_f54_1_random() -> uint32_t {
    static mut x: uint32_t = 0;
    x = (x ^ (x << 5 as libc::c_int | x >> 32 as libc::c_int - 5 as libc::c_int)
        ^ (x << 24 as libc::c_int | x >> 32 as libc::c_int - 24 as libc::c_int))
        .wrapping_add(0x37798849 as libc::c_int as libc::c_uint);
    return x;
}
static mut identity: XTransform = {
    let mut init = _XTransform {
        matrix: [
            [
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ],
            [
                0 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
            ],
            [0 as libc::c_int, 0 as libc::c_int, (1 as libc::c_int) << 16 as libc::c_int],
        ],
    };
    init
};
unsafe extern "C" fn picture_set_matrix(
    mut display: *mut cairo_xlib_display_t,
    mut picture: Picture,
    mut matrix: *const cairo_matrix_t,
    mut filter: cairo_filter_t,
    mut xc: libc::c_double,
    mut yc: libc::c_double,
    mut x_offset: *mut libc::c_int,
    mut y_offset: *mut libc::c_int,
) -> cairo_bool_t {
    let mut xtransform: XTransform = XTransform { matrix: [[0; 3]; 3] };
    let mut pixman_transform: *mut pixman_transform_t = 0 as *mut pixman_transform_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    pixman_transform = &mut xtransform as *mut XTransform as *mut pixman_transform_t;
    status = _cairo_matrix_to_pixman_matrix_offset(
        matrix,
        filter,
        xc,
        yc,
        pixman_transform,
        x_offset,
        y_offset,
    ) as cairo_int_status_t;
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if memcmp(
        &mut xtransform as *mut XTransform as *const libc::c_void,
        &identity as *const XTransform as *const libc::c_void,
        ::std::mem::size_of::<XTransform>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if !((*display).render_major > 0 as libc::c_int
        || (*display).render_major == 0 as libc::c_int
            && (*display).render_minor >= 6 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    XRenderSetPictureTransform((*display).display, picture, &mut xtransform);
    return 1 as libc::c_int;
}
unsafe extern "C" fn picture_set_filter(
    mut dpy: *mut Display,
    mut picture: Picture,
    mut filter: cairo_filter_t,
) -> cairo_status_t {
    let mut render_filter: *const libc::c_char = 0 as *const libc::c_char;
    match filter as libc::c_uint {
        0 => {
            render_filter = b"fast\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            render_filter = b"good\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            render_filter = b"best\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            render_filter = b"nearest\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            render_filter = b"bilinear\0" as *const u8 as *const libc::c_char;
        }
        5 | _ => {
            render_filter = b"best\0" as *const u8 as *const libc::c_char;
        }
    }
    XRenderSetPictureFilter(
        dpy,
        picture,
        render_filter as *mut libc::c_char,
        0 as *mut XFixed,
        0 as libc::c_int,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn extend_to_repeat(mut extend: cairo_extend_t) -> libc::c_int {
    match extend as libc::c_uint {
        0 => {}
        1 => return 1 as libc::c_int,
        2 => return 3 as libc::c_int,
        3 => return 2 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xlib-source.c\0" as *const u8 as *const libc::c_char,
                    231 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"int extend_to_repeat(cairo_extend_t)\0"))
                        .as_ptr(),
                );
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn picture_set_properties(
    mut display: *mut cairo_xlib_display_t,
    mut picture: Picture,
    mut pattern: *const cairo_pattern_t,
    mut matrix: *const cairo_matrix_t,
    mut extents: *const cairo_rectangle_int_t,
    mut x_off: *mut libc::c_int,
    mut y_off: *mut libc::c_int,
) -> cairo_bool_t {
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
    if picture_set_matrix(
        display,
        picture,
        matrix,
        (*pattern).filter,
        ((*extents).x + (*extents).width / 2 as libc::c_int) as libc::c_double,
        ((*extents).y + (*extents).height / 2 as libc::c_int) as libc::c_double,
        x_off,
        y_off,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    picture_set_filter((*display).display, picture, (*pattern).filter);
    if (*pattern).has_component_alpha != 0 {
        pa.component_alpha = 1 as libc::c_int;
        mask |= (1 as libc::c_int) << 12 as libc::c_int;
    }
    if (*pattern).extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        pa.repeat = extend_to_repeat((*pattern).extend);
        mask |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if mask != 0 {
        XRenderChangePicture(
            (*display).display,
            picture,
            mask as libc::c_ulong,
            &mut pa,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn render_pattern(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut dpy: *mut Display = (*(*dst).display).display;
    let mut src: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut map_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    src = _cairo_surface_create_scratch(
        &mut (*dst).base,
        (if is_mask != 0 {
            CAIRO_CONTENT_ALPHA as libc::c_int
        } else {
            CAIRO_CONTENT_COLOR_ALPHA as libc::c_int
        }) as cairo_content_t,
        (*extents).width,
        (*extents).height,
        0 as *const cairo_color_t,
    ) as *mut cairo_xlib_surface_t;
    if (*src).base.type_0 as libc::c_uint
        != CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
    {
        cairo_surface_destroy(&mut (*src).base);
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    map_extents = *extents;
    map_extents.y = 0 as libc::c_int;
    map_extents.x = map_extents.y;
    image = _cairo_surface_map_to_image(&mut (*src).base, &mut map_extents);
    status = _cairo_surface_offset_paint(
        &mut (*image).base,
        (*extents).x,
        (*extents).y,
        CAIRO_OPERATOR_SOURCE,
        pattern,
        0 as *const cairo_clip_t,
    );
    status = _cairo_surface_unmap_image(&mut (*src).base, image) as cairo_status_t;
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*src).base);
        return _cairo_surface_create_in_error(status);
    }
    status = _cairo_xlib_surface_put_shm(src) as cairo_status_t;
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*src).base);
        return _cairo_surface_create_in_error(status);
    }
    (*src)
        .picture = XRenderCreatePicture(
        dpy,
        (*src).drawable,
        (*src).xrender_format,
        0 as libc::c_int as libc::c_ulong,
        0 as *const XRenderPictureAttributes,
    );
    *src_x = -(*extents).x;
    *src_y = -(*extents).y;
    return &mut (*src).base;
}
unsafe extern "C" fn gradient_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut gradient: *const cairo_gradient_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut matrix: cairo_matrix_t = (*gradient).base.matrix;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut extremes: [cairo_circle_double_t; 2] = [cairo_circle_double_t {
        center: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        radius: 0.,
    }; 2];
    let mut stops: *mut XFixed = 0 as *mut XFixed;
    let mut colors: *mut XRenderColor = 0 as *mut XRenderColor;
    let mut picture: Picture = 0;
    let mut i: libc::c_uint = 0;
    let mut n_stops: libc::c_uint = 0;
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
        && _cairo_radial_pattern_focus_is_inside(gradient as *mut cairo_radial_pattern_t)
            == 0
    {
        return render_pattern(dst, &(*gradient).base, is_mask, extents, src_x, src_y);
    }
    if (*gradient).n_stops > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"gradient->n_stops > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-source.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 150],
                &[libc::c_char; 150],
            >(
                b"cairo_surface_t *gradient_source(cairo_xlib_surface_t *, const cairo_gradient_pattern_t *, cairo_bool_t, const cairo_rectangle_int_t *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    n_stops = if (*gradient).n_stops > 2 as libc::c_int as libc::c_uint {
        (*gradient).n_stops
    } else {
        2 as libc::c_int as libc::c_uint
    };
    if (n_stops as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<XFixed>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<XRenderColor>() as libc::c_ulong),
            )
    {
        stops = buf.as_mut_ptr() as *mut XFixed;
    } else {
        stops = _cairo_malloc_ab(
            n_stops as size_t,
            (::std::mem::size_of::<XFixed>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<XRenderColor>() as libc::c_ulong),
        ) as *mut XFixed;
        if stops.is_null() {
            return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
        }
    }
    colors = stops.offset(n_stops as isize) as *mut XRenderColor;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*gradient).n_stops {
        *stops
            .offset(
                i as isize,
            ) = _cairo_fixed_16_16_from_double(
            (*((*gradient).stops).offset(i as isize)).offset,
        );
        (*colors.offset(i as isize))
            .red = (*((*gradient).stops).offset(i as isize)).color.red_short;
        (*colors.offset(i as isize))
            .green = (*((*gradient).stops).offset(i as isize)).color.green_short;
        (*colors.offset(i as isize))
            .blue = (*((*gradient).stops).offset(i as isize)).color.blue_short;
        (*colors.offset(i as isize))
            .alpha = (*((*gradient).stops).offset(i as isize)).color.alpha_short;
        i = i.wrapping_add(1);
    }
    if (*gradient).n_stops == 1 as libc::c_int as libc::c_uint {
        *stops
            .offset(
                1 as libc::c_int as isize,
            ) = _cairo_fixed_16_16_from_double(
            (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset,
        );
        (*colors.offset(1 as libc::c_int as isize))
            .red = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .red_short;
        (*colors.offset(1 as libc::c_int as isize))
            .green = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .green_short;
        (*colors.offset(1 as libc::c_int as isize))
            .blue = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .blue_short;
        (*colors.offset(1 as libc::c_int as isize))
            .alpha = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .alpha_short;
    }
    _cairo_gradient_pattern_fit_to_range(
        gradient,
        ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
            >> 1 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
            as libc::c_double,
        &mut matrix,
        extremes.as_mut_ptr(),
    );
    if (*gradient).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        let mut grad: XLinearGradient = XLinearGradient {
            p1: XPointFixed { x: 0, y: 0 },
            p2: XPointFixed { x: 0, y: 0 },
        };
        grad
            .p1
            .x = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.x,
        );
        grad
            .p1
            .y = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.y,
        );
        grad
            .p2
            .x = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.x,
        );
        grad
            .p2
            .y = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.y,
        );
        picture = XRenderCreateLinearGradient(
            (*display).display,
            &mut grad,
            stops,
            colors,
            n_stops as libc::c_int,
        );
    } else {
        let mut grad_0: XRadialGradient = XRadialGradient {
            inner: XCircle { x: 0, y: 0, radius: 0 },
            outer: XCircle { x: 0, y: 0, radius: 0 },
        };
        grad_0
            .inner
            .x = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.x,
        );
        grad_0
            .inner
            .y = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.y,
        );
        grad_0
            .inner
            .radius = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].radius,
        );
        grad_0
            .outer
            .x = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.x,
        );
        grad_0
            .outer
            .y = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.y,
        );
        grad_0
            .outer
            .radius = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].radius,
        );
        picture = XRenderCreateRadialGradient(
            (*display).display,
            &mut grad_0,
            stops,
            colors,
            n_stops as libc::c_int,
        );
    }
    if stops != buf.as_mut_ptr() as *mut XFixed {
        free(stops as *mut libc::c_void);
    }
    *src_y = 0 as libc::c_int;
    *src_x = *src_y;
    if picture_set_properties(
        display,
        picture,
        &(*gradient).base,
        &(*gradient).base.matrix,
        extents,
        src_x,
        src_y,
    ) == 0
    {
        XRenderFreePicture((*display).display, picture);
        return render_pattern(dst, &(*gradient).base, is_mask, extents, src_x, src_y);
    }
    return source(dst, picture, 0 as libc::c_long as Pixmap);
}
unsafe extern "C" fn color_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
) -> *mut cairo_surface_t {
    let mut dpy: *mut Display = (*(*dst).display).display;
    let mut xcolor: XRenderColor = XRenderColor {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let mut picture: Picture = 0;
    let mut pixmap: Pixmap = 0 as libc::c_long as Pixmap;
    xcolor.red = (*color).red_short;
    xcolor.green = (*color).green_short;
    xcolor.blue = (*color).blue_short;
    xcolor.alpha = (*color).alpha_short;
    if (*(*dst).display).render_major > 0 as libc::c_int
        || (*(*dst).display).render_major == 0 as libc::c_int
            && (*(*dst).display).render_minor >= 10 as libc::c_int
    {
        picture = XRenderCreateSolidFill(dpy, &mut xcolor);
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
        let mut mask: libc::c_int = 0 as libc::c_int;
        pa.repeat = 1 as libc::c_int;
        mask |= (1 as libc::c_int) << 0 as libc::c_int;
        pixmap = XCreatePixmap(
            dpy,
            (*dst).drawable,
            1 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
            32 as libc::c_int as libc::c_uint,
        );
        picture = XRenderCreatePicture(
            dpy,
            pixmap,
            _cairo_xlib_display_get_xrender_format((*dst).display, CAIRO_FORMAT_ARGB32),
            mask as libc::c_ulong,
            &mut pa,
        );
        if (*(*dst).display).render_major > 0 as libc::c_int
            || (*(*dst).display).render_major == 0 as libc::c_int
                && (*(*dst).display).render_minor >= 1 as libc::c_int
        {
            let mut r: XRectangle = {
                let mut init = XRectangle {
                    x: 0 as libc::c_int as libc::c_short,
                    y: 0 as libc::c_int as libc::c_short,
                    width: 1 as libc::c_int as libc::c_ushort,
                    height: 1 as libc::c_int as libc::c_ushort,
                };
                init
            };
            XRenderFillRectangles(
                dpy,
                1 as libc::c_int,
                picture,
                &mut xcolor,
                &mut r,
                1 as libc::c_int,
            );
        } else {
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
            let mut gc: GC = 0 as *mut _XGC;
            gc = _cairo_xlib_screen_get_gc(
                (*dst).display,
                (*dst).screen,
                32 as libc::c_int,
                pixmap,
            );
            if gc.is_null() {
                XFreePixmap(dpy, pixmap);
                return _cairo_surface_create_in_error(
                    _cairo_error(CAIRO_STATUS_NO_MEMORY),
                );
            }
            gcv.foreground = 0 as libc::c_int as libc::c_ulong;
            gcv.foreground
                |= (((*color).alpha_short as uint32_t >> 8 as libc::c_int)
                    << 24 as libc::c_int) as libc::c_ulong;
            gcv.foreground
                |= (((*color).red_short as libc::c_int >> 8 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_ulong;
            gcv.foreground
                |= (((*color).green_short as libc::c_int >> 8 as libc::c_int)
                    << 8 as libc::c_int) as libc::c_ulong;
            gcv.foreground
                |= (((*color).blue_short as libc::c_int >> 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_ulong;
            gcv.fill_style = 0 as libc::c_int;
            XChangeGC(
                dpy,
                gc,
                ((1 as libc::c_long) << 8 as libc::c_int
                    | (1 as libc::c_long) << 2 as libc::c_int) as libc::c_ulong,
                &mut gcv,
            );
            XFillRectangle(
                dpy,
                pixmap,
                gc,
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            );
            _cairo_xlib_screen_put_gc(
                (*dst).display,
                (*dst).screen,
                32 as libc::c_int,
                gc,
            );
        }
    }
    return source(dst, picture, pixmap);
}
unsafe extern "C" fn alpha_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut alpha: uint8_t,
) -> *mut cairo_surface_t {
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    if ((*display).alpha[alpha as usize]).is_null() {
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
        color.blue_short = 0 as libc::c_int as libc::c_ushort;
        color.green_short = color.blue_short;
        color.red_short = color.green_short;
        color
            .alpha_short = ((alpha as libc::c_int) << 8 as libc::c_int
            | alpha as libc::c_int) as libc::c_ushort;
        let ref mut fresh3 = (*display).alpha[alpha as usize];
        *fresh3 = color_source(dst, &mut color);
    }
    return cairo_surface_reference((*display).alpha[alpha as usize]);
}
unsafe extern "C" fn white_source(
    mut dst: *mut cairo_xlib_surface_t,
) -> *mut cairo_surface_t {
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    if ((*display).white).is_null() {
        let ref mut fresh4 = (*display).white;
        *fresh4 = color_source(dst, _cairo_stock_color(CAIRO_STOCK_WHITE));
    }
    return cairo_surface_reference((*display).white);
}
unsafe extern "C" fn opaque_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
) -> *mut cairo_surface_t {
    let mut current_block: u64;
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut pixel: uint32_t = 0xff000000 as libc::c_uint
        | (((*color).red_short as libc::c_int >> 8 as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
        | (((*color).green_short as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int)
            as libc::c_uint
        | (((*color).blue_short as libc::c_int >> 8 as libc::c_int) << 0 as libc::c_int)
            as libc::c_uint;
    let mut i: libc::c_int = 0;
    if (*display).last_solid_cache[0 as libc::c_int as usize].color == pixel {
        return cairo_surface_reference(
            (*display)
                .solid[(*display).last_solid_cache[0 as libc::c_int as usize].index
                as usize],
        );
    }
    i = 0 as libc::c_int;
    loop {
        if !(i < 16 as libc::c_int) {
            current_block = 11006700562992250127;
            break;
        }
        if (*display).solid_cache[i as usize] == pixel {
            current_block = 16647855912173822573;
            break;
        }
        i += 1;
    }
    match current_block {
        11006700562992250127 => {
            i = (hars_petruska_f54_1_random())
                .wrapping_rem(16 as libc::c_int as libc::c_uint) as libc::c_int;
            cairo_surface_destroy((*display).solid[i as usize]);
            let ref mut fresh5 = (*display).solid[i as usize];
            *fresh5 = color_source(dst, color);
            (*display).solid_cache[i as usize] = pixel;
        }
        _ => {}
    }
    (*display).last_solid_cache[0 as libc::c_int as usize].color = pixel;
    (*display).last_solid_cache[0 as libc::c_int as usize].index = i;
    return cairo_surface_reference((*display).solid[i as usize]);
}
unsafe extern "C" fn transparent_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
) -> *mut cairo_surface_t {
    let mut current_block: u64;
    let mut display: *mut cairo_xlib_display_t = (*dst).display;
    let mut pixel: uint32_t = ((*color).alpha_short as uint32_t >> 8 as libc::c_int)
        << 24 as libc::c_int
        | (((*color).red_short as libc::c_int >> 8 as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
        | (((*color).green_short as libc::c_int >> 8 as libc::c_int) << 8 as libc::c_int)
            as libc::c_uint
        | (((*color).blue_short as libc::c_int >> 8 as libc::c_int) << 0 as libc::c_int)
            as libc::c_uint;
    let mut i: libc::c_int = 0;
    if (*display).last_solid_cache[1 as libc::c_int as usize].color == pixel {
        if !((*display)
            .solid[(*display).last_solid_cache[1 as libc::c_int as usize].index
            as usize])
            .is_null()
        {} else {
            __assert_fail(
                b"display->solid[display->last_solid_cache[1].index]\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-xlib-source.c\0" as *const u8 as *const libc::c_char,
                577 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"cairo_surface_t *transparent_source(cairo_xlib_surface_t *, const cairo_color_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        return cairo_surface_reference(
            (*display)
                .solid[(*display).last_solid_cache[1 as libc::c_int as usize].index
                as usize],
        );
    }
    i = 16 as libc::c_int;
    loop {
        if !(i < 32 as libc::c_int) {
            current_block = 13513818773234778473;
            break;
        }
        if (*display).solid_cache[i as usize] == pixel {
            current_block = 7976325610921902275;
            break;
        }
        i += 1;
    }
    match current_block {
        13513818773234778473 => {
            i = (16 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (hars_petruska_f54_1_random())
                        .wrapping_rem(16 as libc::c_int as libc::c_uint),
                ) as libc::c_int;
            cairo_surface_destroy((*display).solid[i as usize]);
            let ref mut fresh6 = (*display).solid[i as usize];
            *fresh6 = color_source(dst, color);
            (*display).solid_cache[i as usize] = pixel;
        }
        _ => {}
    }
    (*display).last_solid_cache[1 as libc::c_int as usize].color = pixel;
    (*display).last_solid_cache[1 as libc::c_int as usize].index = i;
    if !((*display).solid[i as usize]).is_null() {} else {
        __assert_fail(
            b"display->solid[i]\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-source.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"cairo_surface_t *transparent_source(cairo_xlib_surface_t *, const cairo_color_t *)\0",
            ))
                .as_ptr(),
        );
    }
    return cairo_surface_reference((*display).solid[i as usize]);
}
unsafe extern "C" fn solid_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut color: *const cairo_color_t,
) -> *mut cairo_surface_t {
    if (*color).red_short as libc::c_int | (*color).green_short as libc::c_int
        | (*color).blue_short as libc::c_int <= 0xff as libc::c_int
    {
        return alpha_source(
            dst,
            ((*color).alpha_short as libc::c_int >> 8 as libc::c_int) as uint8_t,
        );
    }
    if (*color).alpha_short as libc::c_int >= 0xff00 as libc::c_int {
        if (*color).red_short as libc::c_int >= 0xff00 as libc::c_int
            && (*color).green_short as libc::c_int >= 0xff00 as libc::c_int
            && (*color).blue_short as libc::c_int >= 0xff00 as libc::c_int
        {
            return white_source(dst);
        }
        return opaque_source(dst, color);
    } else {
        return transparent_source(dst, color)
    };
}
unsafe extern "C" fn init_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut src: *mut cairo_xlib_surface_t,
) -> *mut cairo_xlib_source_t {
    let mut dpy: *mut Display = (*(*dst).display).display;
    let mut source_0: *mut cairo_xlib_source_t = &mut (*src).embedded_source;
    if (*source_0).picture == 0 as libc::c_long as libc::c_ulong {
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
        _cairo_surface_init(
            &mut (*source_0).base,
            &cairo_xlib_source_backend,
            0 as *mut cairo_device_t,
            CAIRO_CONTENT_COLOR_ALPHA,
            0 as libc::c_int,
        );
        pa.subwindow_mode = 1 as libc::c_int;
        (*source_0)
            .picture = XRenderCreatePicture(
            dpy,
            (*src).drawable,
            (*src).xrender_format,
            ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
            &mut pa,
        );
        (*source_0).set_has_component_alpha(0 as libc::c_int as libc::c_uint);
        (*source_0).set_has_matrix(0 as libc::c_int as libc::c_uint);
        (*source_0).set_filter(CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint);
        (*source_0).set_extend(CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint);
    }
    return cairo_surface_reference(&mut (*source_0).base) as *mut cairo_xlib_source_t;
}
unsafe extern "C" fn embedded_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
    mut source_0: *mut cairo_xlib_source_t,
) -> *mut cairo_surface_t {
    let mut dpy: *mut Display = (*(*dst).display).display;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut xtransform: XTransform = XTransform { matrix: [[0; 3]; 3] };
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
    let mut mask: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    status = _cairo_matrix_to_pixman_matrix_offset(
        &(*pattern).base.matrix,
        (*pattern).base.filter,
        ((*extents).x + (*extents).width / 2 as libc::c_int) as libc::c_double,
        ((*extents).y + (*extents).height / 2 as libc::c_int) as libc::c_double,
        &mut xtransform as *mut XTransform as *mut pixman_transform_t,
        src_x,
        src_y,
    ) as cairo_int_status_t;
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        if (*source_0).has_matrix() != 0 {
            (*source_0).set_has_matrix(0 as libc::c_int as libc::c_uint);
            memcpy(
                &mut xtransform as *mut XTransform as *mut libc::c_void,
                &identity as *const XTransform as *const libc::c_void,
                ::std::mem::size_of::<XTransform>() as libc::c_ulong,
            );
            status = CAIRO_INT_STATUS_SUCCESS;
        }
    } else {
        (*source_0).set_has_matrix(1 as libc::c_int as libc::c_uint);
    }
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        XRenderSetPictureTransform(dpy, (*source_0).picture, &mut xtransform);
    }
    if (*source_0).filter() != (*pattern).base.filter as libc::c_uint {
        picture_set_filter(dpy, (*source_0).picture, (*pattern).base.filter);
        (*source_0).set_filter((*pattern).base.filter as libc::c_uint);
    }
    if (*source_0).has_component_alpha() as libc::c_int
        != (*pattern).base.has_component_alpha
    {
        pa.component_alpha = (*pattern).base.has_component_alpha;
        mask |= ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
        (*source_0)
            .set_has_component_alpha(
                (*pattern).base.has_component_alpha as libc::c_uint,
            );
    }
    if (*source_0).extend() != (*pattern).base.extend as libc::c_uint {
        pa.repeat = extend_to_repeat((*pattern).base.extend);
        mask |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*source_0).set_extend((*pattern).base.extend as libc::c_uint);
    }
    if mask != 0 {
        XRenderChangePicture(dpy, (*source_0).picture, mask as libc::c_ulong, &mut pa);
    }
    return &mut (*source_0).base;
}
unsafe extern "C" fn subsurface_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut sub: *mut cairo_surface_subsurface_t = 0 as *mut cairo_surface_subsurface_t;
    let mut src: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut source_0: *mut cairo_xlib_source_t = 0 as *mut cairo_xlib_source_t;
    let mut dpy: *mut Display = (*(*dst).display).display;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut local_pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
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
    let mut xtransform: XTransform = XTransform { matrix: [[0; 3]; 3] };
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
    let mut mask: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    sub = (*pattern).surface as *mut cairo_surface_subsurface_t;
    if (*sample).x >= 0 as libc::c_int && (*sample).y >= 0 as libc::c_int
        && (*sample).x + (*sample).width <= (*sub).extents.width
        && (*sample).y + (*sample).height <= (*sub).extents.height
    {
        src = (*sub).target as *mut cairo_xlib_surface_t;
        status = _cairo_surface_flush(&mut (*src).base, 0 as libc::c_int as libc::c_uint)
            as cairo_int_status_t;
        if status as u64 != 0 {
            return _cairo_surface_create_in_error(status as cairo_status_t);
        }
        if (*pattern).base.filter as libc::c_uint
            == CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint
            && _cairo_matrix_is_translation(&(*pattern).base.matrix) != 0
        {
            *src_x = (*src_x as libc::c_double
                + ((*pattern).base.matrix.x0 + (*sub).extents.x as libc::c_double))
                as libc::c_int;
            *src_y = (*src_y as libc::c_double
                + ((*pattern).base.matrix.y0 + (*sub).extents.y as libc::c_double))
                as libc::c_int;
            _cairo_xlib_surface_ensure_picture(src);
            return cairo_surface_reference(&mut (*src).base);
        } else {
            let mut local_pattern_0: cairo_surface_pattern_t = *pattern;
            local_pattern_0.base.matrix.x0 += (*sub).extents.x as libc::c_double;
            local_pattern_0.base.matrix.y0 += (*sub).extents.y as libc::c_double;
            local_pattern_0.base.extend = CAIRO_EXTEND_NONE;
            return embedded_source(
                dst,
                &mut local_pattern_0,
                extents,
                src_x,
                src_y,
                init_source(dst, src),
            );
        }
    }
    if !((*sub).snapshot).is_null()
        && (*(*sub).snapshot).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
    {
        src = cairo_surface_reference((*sub).snapshot) as *mut cairo_xlib_surface_t;
        source_0 = &mut (*src).embedded_source;
    } else {
        src = _cairo_surface_create_scratch(
            &mut (*dst).base,
            (*sub).base.content,
            (*sub).extents.width,
            (*sub).extents.height,
            0 as *const cairo_color_t,
        ) as *mut cairo_xlib_surface_t;
        if (*src).base.type_0 as libc::c_uint
            != CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
        {
            cairo_surface_destroy(&mut (*src).base);
            return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY);
        }
        _cairo_pattern_init_for_surface(&mut local_pattern, (*sub).target);
        cairo_matrix_init_translate(
            &mut local_pattern.base.matrix,
            (*sub).extents.x as libc::c_double,
            (*sub).extents.y as libc::c_double,
        );
        local_pattern.base.filter = CAIRO_FILTER_NEAREST;
        status = _cairo_surface_paint(
            &mut (*src).base,
            CAIRO_OPERATOR_SOURCE,
            &mut local_pattern.base,
            0 as *const cairo_clip_t,
        ) as cairo_int_status_t;
        _cairo_pattern_fini(&mut local_pattern.base);
        if status as u64 != 0 {
            cairo_surface_destroy(&mut (*src).base);
            return _cairo_surface_create_in_error(status as cairo_status_t);
        }
        _cairo_xlib_surface_ensure_picture(src);
        _cairo_surface_subsurface_set_snapshot(&mut (*sub).base, &mut (*src).base);
        source_0 = &mut (*src).embedded_source;
        (*source_0).set_has_component_alpha(0 as libc::c_int as libc::c_uint);
        (*source_0).set_has_matrix(0 as libc::c_int as libc::c_uint);
        (*source_0).set_filter(CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint);
        (*source_0).set_extend(CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint);
    }
    status = _cairo_matrix_to_pixman_matrix_offset(
        &(*pattern).base.matrix,
        (*pattern).base.filter,
        ((*extents).x + (*extents).width / 2 as libc::c_int) as libc::c_double,
        ((*extents).y + (*extents).height / 2 as libc::c_int) as libc::c_double,
        &mut xtransform as *mut XTransform as *mut pixman_transform_t,
        src_x,
        src_y,
    ) as cairo_int_status_t;
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        if (*source_0).has_matrix() != 0 {
            (*source_0).set_has_matrix(0 as libc::c_int as libc::c_uint);
            memcpy(
                &mut xtransform as *mut XTransform as *mut libc::c_void,
                &identity as *const XTransform as *const libc::c_void,
                ::std::mem::size_of::<XTransform>() as libc::c_ulong,
            );
            status = CAIRO_INT_STATUS_SUCCESS;
        }
    } else {
        (*source_0).set_has_matrix(1 as libc::c_int as libc::c_uint);
    }
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        XRenderSetPictureTransform(dpy, (*src).picture, &mut xtransform);
    }
    if (*source_0).filter() != (*pattern).base.filter as libc::c_uint {
        picture_set_filter(dpy, (*src).picture, (*pattern).base.filter);
        (*source_0).set_filter((*pattern).base.filter as libc::c_uint);
    }
    if (*source_0).has_component_alpha() as libc::c_int
        != (*pattern).base.has_component_alpha
    {
        pa.component_alpha = (*pattern).base.has_component_alpha;
        mask |= ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
        (*source_0)
            .set_has_component_alpha(
                (*pattern).base.has_component_alpha as libc::c_uint,
            );
    }
    if (*source_0).extend() != (*pattern).base.extend as libc::c_uint {
        pa.repeat = extend_to_repeat((*pattern).base.extend);
        mask |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*source_0).set_extend((*pattern).base.extend as libc::c_uint);
    }
    if mask != 0 {
        XRenderChangePicture(dpy, (*src).picture, mask as libc::c_ulong, &mut pa);
    }
    return &mut (*src).base;
}
unsafe extern "C" fn native_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut src: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if _cairo_surface_is_subsurface((*pattern).surface) != 0 {
        return subsurface_source(dst, pattern, is_mask, extents, sample, src_x, src_y);
    }
    src = unwrap_source(pattern);
    status = _cairo_surface_flush(&mut (*src).base, 0 as libc::c_int as libc::c_uint)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status as cairo_status_t);
    }
    if (*pattern).base.filter as libc::c_uint
        == CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint
        && (*sample).x >= 0 as libc::c_int && (*sample).y >= 0 as libc::c_int
        && (*sample).x + (*sample).width <= (*src).width
        && (*sample).y + (*sample).height <= (*src).height
        && _cairo_matrix_is_translation(&(*pattern).base.matrix) != 0
    {
        *src_x = (*src_x as libc::c_double + (*pattern).base.matrix.x0) as libc::c_int;
        *src_y = (*src_y as libc::c_double + (*pattern).base.matrix.y0) as libc::c_int;
        _cairo_xlib_surface_ensure_picture(src);
        return cairo_surface_reference(&mut (*src).base);
    }
    return embedded_source(dst, pattern, extents, src_x, src_y, init_source(dst, src));
}
unsafe extern "C" fn recording_pattern_get_surface(
    mut pattern: *const cairo_pattern_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    surface = (*(pattern as *const cairo_surface_pattern_t)).surface;
    if _cairo_surface_is_paginated(surface) != 0 {
        return cairo_surface_reference(_cairo_paginated_surface_get_recording(surface));
    }
    if _cairo_surface_is_snapshot(surface) != 0 {
        return _cairo_surface_snapshot_get_target(surface);
    }
    return cairo_surface_reference(surface);
}
unsafe extern "C" fn record_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut src: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut recording: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut m: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut upload: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut limit: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    upload = *sample;
    if _cairo_surface_get_extents((*pattern).surface, &mut limit) != 0
        && _cairo_rectangle_intersect(&mut upload, &mut limit) == 0
    {
        if (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        {
            return alpha_source(dst, 0 as libc::c_int as uint8_t);
        }
        upload = limit;
    }
    src = _cairo_surface_create_scratch(
        &mut (*dst).base,
        (*(*pattern).surface).content,
        upload.width,
        upload.height,
        0 as *const cairo_color_t,
    ) as *mut cairo_xlib_surface_t;
    if (*src).base.type_0 as libc::c_uint
        != CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
    {
        cairo_surface_destroy(&mut (*src).base);
        return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY);
    }
    cairo_matrix_init_translate(
        &mut matrix,
        upload.x as libc::c_double,
        upload.y as libc::c_double,
    );
    recording = recording_pattern_get_surface(&(*pattern).base);
    status = _cairo_recording_surface_replay_with_clip(
        recording,
        &mut matrix,
        &mut (*src).base,
        0 as *const cairo_clip_t,
    );
    cairo_surface_destroy(recording);
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*src).base);
        return _cairo_surface_create_in_error(status);
    }
    matrix = (*pattern).base.matrix;
    if upload.x | upload.y != 0 {
        cairo_matrix_init_translate(
            &mut m,
            -upload.x as libc::c_double,
            -upload.y as libc::c_double,
        );
        cairo_matrix_multiply(&mut matrix, &mut matrix, &mut m);
    }
    _cairo_xlib_surface_ensure_picture(src);
    if picture_set_properties(
        (*src).display,
        (*src).picture,
        &(*pattern).base,
        &mut matrix,
        extents,
        src_x,
        src_y,
    ) == 0
    {
        cairo_surface_destroy(&mut (*src).base);
        return render_pattern(dst, &(*pattern).base, is_mask, extents, src_x, src_y);
    }
    return &mut (*src).base;
}
unsafe extern "C" fn surface_source(
    mut dst: *mut cairo_xlib_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut src: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut xsrc: *mut cairo_xlib_surface_t = 0 as *mut cairo_xlib_surface_t;
    let mut local_pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut upload: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut limit: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    src = (*pattern).surface;
    if (*src).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        && (*src).device == (*dst).base.device
        && _cairo_xlib_shm_surface_get_pixmap(src) != 0
    {
        let mut proxy: *mut cairo_xlib_proxy_t = 0 as *mut cairo_xlib_proxy_t;
        proxy = (if ::std::mem::size_of::<cairo_xlib_proxy_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_xlib_proxy_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_xlib_proxy_t;
        if proxy.is_null() {
            return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY);
        }
        _cairo_surface_init(
            &mut (*proxy).source.base,
            &cairo_xlib_proxy_backend,
            (*dst).base.device,
            (*src).content,
            (*src).is_vector() as cairo_bool_t,
        );
        let ref mut fresh7 = (*proxy).source.dpy;
        *fresh7 = (*(*dst).display).display;
        (*proxy)
            .source
            .picture = XRenderCreatePicture(
            (*proxy).source.dpy,
            _cairo_xlib_shm_surface_get_pixmap(src),
            _cairo_xlib_shm_surface_get_xrender_format(src),
            0 as libc::c_int as libc::c_ulong,
            0 as *const XRenderPictureAttributes,
        );
        (*proxy).source.pixmap = 0 as libc::c_long as Pixmap;
        let ref mut fresh8 = (*proxy).source;
        (*fresh8).set_has_component_alpha(0 as libc::c_int as libc::c_uint);
        let ref mut fresh9 = (*proxy).source;
        (*fresh9).set_has_matrix(0 as libc::c_int as libc::c_uint);
        let ref mut fresh10 = (*proxy).source;
        (*fresh10).set_filter(CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint);
        let ref mut fresh11 = (*proxy).source;
        (*fresh11).set_extend(CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint);
        let ref mut fresh12 = (*proxy).owner;
        *fresh12 = cairo_surface_reference(src);
        return embedded_source(
            dst,
            pattern,
            extents,
            src_x,
            src_y,
            &mut (*proxy).source,
        );
    }
    upload = *sample;
    if _cairo_surface_get_extents((*pattern).surface, &mut limit) != 0 {
        if (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        {
            if _cairo_rectangle_intersect(&mut upload, &mut limit) == 0 {
                return alpha_source(dst, 0 as libc::c_int as uint8_t);
            }
        } else if (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        {
            if _cairo_rectangle_intersect(&mut upload, &mut limit) == 0 {
                upload = limit;
            }
        } else if upload.x < limit.x || upload.x + upload.width > limit.x + limit.width
            || upload.y < limit.y || upload.y + upload.height > limit.y + limit.height
        {
            upload = limit;
        }
    }
    xsrc = _cairo_surface_create_scratch(
        &mut (*dst).base,
        (*src).content,
        upload.width,
        upload.height,
        0 as *const cairo_color_t,
    ) as *mut cairo_xlib_surface_t;
    if (*xsrc).base.type_0 as libc::c_uint
        != CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
    {
        cairo_surface_destroy(src);
        cairo_surface_destroy(&mut (*xsrc).base);
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    if _cairo_surface_is_image(src) != 0 {
        status = _cairo_xlib_surface_draw_image(
            xsrc,
            src as *mut cairo_image_surface_t,
            upload.x,
            upload.y,
            upload.width,
            upload.height,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    } else {
        let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
        let mut map_extents: cairo_rectangle_int_t = {
            let mut init = _cairo_rectangle_int {
                x: 0 as libc::c_int,
                y: 0 as libc::c_int,
                width: upload.width,
                height: upload.height,
            };
            init
        };
        image = _cairo_surface_map_to_image(&mut (*xsrc).base, &mut map_extents);
        _cairo_pattern_init_for_surface(&mut local_pattern, (*pattern).surface);
        cairo_matrix_init_translate(
            &mut local_pattern.base.matrix,
            upload.x as libc::c_double,
            upload.y as libc::c_double,
        );
        status = _cairo_surface_paint(
            &mut (*image).base,
            CAIRO_OPERATOR_SOURCE,
            &mut local_pattern.base,
            0 as *const cairo_clip_t,
        );
        _cairo_pattern_fini(&mut local_pattern.base);
        status = _cairo_surface_unmap_image(&mut (*xsrc).base, image) as cairo_status_t;
        if status as u64 != 0 {
            cairo_surface_destroy(&mut (*xsrc).base);
            return _cairo_surface_create_in_error(status);
        }
        status = _cairo_xlib_surface_put_shm(xsrc) as cairo_status_t;
        if status as u64 != 0 {
            cairo_surface_destroy(&mut (*xsrc).base);
            return _cairo_surface_create_in_error(status);
        }
    }
    _cairo_pattern_init_static_copy(&mut local_pattern.base, &(*pattern).base);
    if upload.x | upload.y != 0 {
        let mut m: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        cairo_matrix_init_translate(
            &mut m,
            -upload.x as libc::c_double,
            -upload.y as libc::c_double,
        );
        cairo_matrix_multiply(
            &mut local_pattern.base.matrix,
            &mut local_pattern.base.matrix,
            &mut m,
        );
    }
    *src_y = 0 as libc::c_int;
    *src_x = *src_y;
    _cairo_xlib_surface_ensure_picture(xsrc);
    if picture_set_properties(
        (*xsrc).display,
        (*xsrc).picture,
        &mut local_pattern.base,
        &mut local_pattern.base.matrix,
        extents,
        src_x,
        src_y,
    ) == 0
    {
        cairo_surface_destroy(&mut (*xsrc).base);
        return render_pattern(dst, &(*pattern).base, is_mask, extents, src_x, src_y);
    }
    return &mut (*xsrc).base;
}
unsafe extern "C" fn pattern_is_supported(
    mut display: *mut cairo_xlib_display_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*display).buggy_pad_reflect() != 0 {
        if (*pattern).extend as libc::c_uint
            == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
            || (*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
    }
    if (*display).buggy_gradients() != 0 {
        if (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
            || (*pattern).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
    }
    match (*pattern).filter as libc::c_uint {
        0 | 3 => {
            return ((*display).render_major > 0 as libc::c_int
                || (*display).render_major == 0 as libc::c_int
                    && (*display).render_minor >= 6 as libc::c_int
                || _cairo_matrix_is_integer_translation(
                    &(*pattern).matrix,
                    0 as *mut libc::c_int,
                    0 as *mut libc::c_int,
                ) != 0) as libc::c_int;
        }
        1 => return 0 as libc::c_int,
        2 => return 0 as libc::c_int,
        4 | 5 | _ => {
            return ((*display).render_major > 0 as libc::c_int
                || (*display).render_major == 0 as libc::c_int
                    && (*display).render_minor >= 6 as libc::c_int) as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_source_create_for_pattern(
    mut _dst: *mut cairo_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut is_mask: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut sample: *const cairo_rectangle_int_t,
    mut src_x: *mut libc::c_int,
    mut src_y: *mut libc::c_int,
) -> *mut cairo_surface_t {
    let mut dst: *mut cairo_xlib_surface_t = _dst as *mut cairo_xlib_surface_t;
    *src_y = 0 as libc::c_int;
    *src_x = *src_y;
    if pattern.is_null()
        || (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        if pattern.is_null() {
            pattern = &_cairo_pattern_white.base;
        }
        return solid_source(dst, &mut (*(pattern as *mut cairo_solid_pattern_t)).color);
    }
    if pattern_is_supported((*dst).display, pattern) != 0 {
        if (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        {
            let mut spattern: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            if (*(*spattern).surface).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_XLIB as libc::c_int as libc::c_uint
                && _cairo_xlib_surface_same_screen(dst, unwrap_source(spattern)) != 0
            {
                return native_source(
                    dst,
                    spattern,
                    is_mask,
                    extents,
                    sample,
                    src_x,
                    src_y,
                );
            }
            if (*(*spattern).surface).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
            {
                return record_source(
                    dst,
                    spattern,
                    is_mask,
                    extents,
                    sample,
                    src_x,
                    src_y,
                );
            }
            return surface_source(dst, spattern, is_mask, extents, sample, src_x, src_y);
        }
        if (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
            || (*pattern).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
        {
            let mut gpattern: *mut cairo_gradient_pattern_t = pattern
                as *mut cairo_gradient_pattern_t;
            return gradient_source(dst, gpattern, is_mask, extents, src_x, src_y);
        }
    }
    return render_pattern(dst, pattern, is_mask, extents, src_x, src_y);
}
