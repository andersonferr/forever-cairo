use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _cairo_pattern;
    pub type _XErrorThreadInfo;
    pub type _X11XCBPrivate;
    pub type _XtransConnInfo;
    pub type _XkbInfoRec;
    pub type _XIMFilter;
    pub type _XContextDB;
    pub type _XDisplayAtoms;
    pub type _XKeytrans;
    pub type _XLockInfo;
    pub type _XrmHashBucketRec;
    pub type _XPrivate;
    fn pixman_image_create_bits(
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        bits: *mut uint32_t,
        rowstride_bytes: libc::c_int,
    ) -> *mut pixman_image_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_content_from_pixman_format(
        pixman_format: pixman_format_code_t,
    ) -> cairo_content_t;
    fn _cairo_format_to_pixman_format_code(
        format: cairo_format_t,
    ) -> pixman_format_code_t;
    fn _pixman_format_to_masks(
        pixman_format: pixman_format_code_t,
        masks: *mut cairo_format_masks_t,
    ) -> cairo_bool_t;
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
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
    fn XInitImage(_: *mut XImage) -> libc::c_int;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_uint,
        _: *mut Visual,
        _: libc::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XLockDisplay(_: *mut Display);
    fn XUnlockDisplay(_: *mut Display);
    fn XInitExtension(_: *mut Display, _: *const libc::c_char) -> *mut XExtCodes;
    fn XNextRequest(_: *mut Display) -> libc::c_ulong;
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
    fn XDestroyWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn _pixman_format_for_xlib_surface(
        surface: *mut cairo_xlib_surface_t,
    ) -> pixman_format_code_t;
    fn _cairo_xlib_screen_put_gc(
        display: *mut cairo_xlib_display_t,
        info: *mut cairo_xlib_screen_t,
        depth: libc::c_int,
        gc: GC,
    );
    fn _cairo_xlib_display_get_xrender_format_for_pixman(
        display: *mut cairo_xlib_display_t,
        format: pixman_format_code_t,
    ) -> *mut XRenderPictFormat;
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
    fn XEventsQueued(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_long,
        _: *mut XEvent,
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
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn _cairo_damage_create() -> *mut cairo_damage_t;
    fn _cairo_damage_create_in_error(status: cairo_status_t) -> *mut cairo_damage_t;
    fn _cairo_damage_add_rectangle(
        damage: *mut cairo_damage_t,
        rect: *const cairo_rectangle_int_t,
    ) -> *mut cairo_damage_t;
    fn _cairo_damage_reduce(damage: *mut cairo_damage_t) -> *mut cairo_damage_t;
    fn _cairo_damage_destroy(damage: *mut cairo_damage_t);
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_image_surface_paint(
        abstract_surface: *mut libc::c_void,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_image_surface_mask(
        abstract_surface: *mut libc::c_void,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_image_surface_stroke(
        abstract_surface: *mut libc::c_void,
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
    fn _cairo_image_surface_fill(
        abstract_surface: *mut libc::c_void,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_image_surface_glyphs(
        abstract_surface: *mut libc::c_void,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        scaled_font: *mut cairo_scaled_font_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_image_surface_init(
        surface: *mut cairo_image_surface_t,
        pixman_image: *mut pixman_image_t,
        pixman_format: pixman_format_code_t,
    );
    fn _cairo_image_surface_create_similar(
        abstract_other: *mut libc::c_void,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_map_to_image(
        abstract_other: *mut libc::c_void,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_image_surface_unmap_image(
        abstract_surface: *mut libc::c_void,
        image: *mut cairo_image_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_image_surface_source(
        abstract_surface: *mut libc::c_void,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_acquire_source_image(
        abstract_surface: *mut libc::c_void,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_image_surface_release_source_image(
        abstract_surface: *mut libc::c_void,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
    );
    fn _cairo_image_surface_snapshot(
        abstract_surface: *mut libc::c_void,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_surface_get_extents(
        abstract_surface: *mut libc::c_void,
        rectangle: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_image_surface_get_font_options(
        abstract_surface: *mut libc::c_void,
        options: *mut cairo_font_options_t,
    );
    fn _cairo_image_surface_finish(
        abstract_surface: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_mempool_init(
        pool: *mut cairo_mempool_t,
        base: *mut libc::c_void,
        bytes: size_t,
        min_bits: libc::c_int,
        num_sizes: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_mempool_alloc(
        pi: *mut cairo_mempool_t,
        bytes: size_t,
    ) -> *mut libc::c_void;
    fn _cairo_mempool_free(pi: *mut cairo_mempool_t, storage: *mut libc::c_void);
    fn _cairo_mempool_fini(pool: *mut cairo_mempool_t);
    fn _XReadEvents(_: *mut Display);
    fn XShmQueryExtension(_: *mut Display) -> libc::c_int;
    fn XShmQueryVersion(
        _: *mut Display,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XShmAttach(_: *mut Display, _: *mut XShmSegmentInfo) -> libc::c_int;
    fn XShmDetach(_: *mut Display, _: *mut XShmSegmentInfo) -> libc::c_int;
    fn XShmCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: *mut libc::c_char,
        _: *mut XShmSegmentInfo,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;
    fn shmctl(
        __shmid: libc::c_int,
        __cmd: libc::c_int,
        __buf: *mut shmid_ds,
    ) -> libc::c_int;
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
    fn shmat(
        __shmid: libc::c_int,
        __shmaddr: *const libc::c_void,
        __shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    fn shmdt(__shmaddr: *const libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_ulong_t = libc::c_ulong;
pub type key_t = __key_t;
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
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
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
pub struct XExtCodes {
    pub extension: libc::c_int,
    pub major_opcode: libc::c_int,
    pub first_event: libc::c_int,
    pub first_error: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGC {
    pub ext_data: *mut XExtData,
    pub gid: GContext,
    pub rects: libc::c_int,
    pub dashes: libc::c_int,
    pub dirty: libc::c_ulong,
    pub values: XGCValues,
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
pub struct _XDisplay {
    pub ext_data: *mut XExtData,
    pub free_funcs: *mut _XFreeFuncs,
    pub fd: libc::c_int,
    pub conn_checker: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub resource_base: XID,
    pub resource_mask: XID,
    pub resource_id: XID,
    pub resource_shift: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub vnumber: libc::c_int,
    pub release: libc::c_int,
    pub head: *mut _XSQEvent,
    pub tail: *mut _XSQEvent,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub last_req: *mut libc::c_char,
    pub buffer: *mut libc::c_char,
    pub bufptr: *mut libc::c_char,
    pub bufmax: *mut libc::c_char,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub synchandler: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub keysyms: *mut KeySym,
    pub modifiermap: *mut XModifierKeymap,
    pub keysyms_per_keycode: libc::c_int,
    pub xdefaults: *mut libc::c_char,
    pub scratch_buffer: *mut libc::c_char,
    pub scratch_length: libc::c_ulong,
    pub ext_number: libc::c_int,
    pub ext_procs: *mut _XExten,
    pub event_vec: [Option::<
        unsafe extern "C" fn(*mut Display, *mut XEvent, *mut xEvent) -> libc::c_int,
    >; 128],
    pub wire_vec: [Option::<
        unsafe extern "C" fn(*mut Display, *mut XEvent, *mut xEvent) -> libc::c_int,
    >; 128],
    pub lock_meaning: KeySym,
    pub lock: *mut _XLockInfo,
    pub async_handlers: *mut _XInternalAsync,
    pub bigreq_size: libc::c_ulong,
    pub lock_fns: *mut _XLockPtrs,
    pub idlist_alloc: Option::<
        unsafe extern "C" fn(*mut Display, *mut XID, libc::c_int) -> (),
    >,
    pub key_bindings: *mut _XKeytrans,
    pub cursor_font: Font,
    pub atoms: *mut _XDisplayAtoms,
    pub mode_switch: libc::c_uint,
    pub num_lock: libc::c_uint,
    pub context_db: *mut _XContextDB,
    pub error_vec: *mut Option::<
        unsafe extern "C" fn(*mut Display, *mut XErrorEvent, *mut xError) -> libc::c_int,
    >,
    pub cms: C2RustUnnamed_31,
    pub im_filters: *mut _XIMFilter,
    pub qfree: *mut _XSQEvent,
    pub next_event_serial_num: libc::c_ulong,
    pub flushes: *mut _XExten,
    pub im_fd_info: *mut _XConnectionInfo,
    pub im_fd_length: libc::c_int,
    pub conn_watchers: *mut _XConnWatchInfo,
    pub watcher_count: libc::c_int,
    pub filedes: XPointer,
    pub savedsynchandler: Option::<unsafe extern "C" fn(*mut Display) -> libc::c_int>,
    pub resource_max: XID,
    pub xcmisc_opcode: libc::c_int,
    pub xkb_info: *mut _XkbInfoRec,
    pub trans_conn: *mut _XtransConnInfo,
    pub xcb: *mut _X11XCBPrivate,
    pub next_cookie: libc::c_uint,
    pub generic_event_vec: [Option::<
        unsafe extern "C" fn(
            *mut Display,
            *mut XGenericEventCookie,
            *mut xEvent,
        ) -> libc::c_int,
    >; 128],
    pub generic_event_copy_vec: [Option::<
        unsafe extern "C" fn(
            *mut Display,
            *mut XGenericEventCookie,
            *mut XGenericEventCookie,
        ) -> libc::c_int,
    >; 128],
    pub cookiejar: *mut libc::c_void,
    pub error_threads: *mut _XErrorThreadInfo,
    pub exit_handler: XIOErrorExitHandler,
    pub exit_handler_data: *mut libc::c_void,
}
pub type XIOErrorExitHandler = Option::<
    unsafe extern "C" fn(*mut Display, *mut libc::c_void) -> (),
>;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
pub type xEvent = _xEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xEvent {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: C2RustUnnamed_29,
    pub keyButtonPointer: C2RustUnnamed_28,
    pub enterLeave: C2RustUnnamed_27,
    pub focus: C2RustUnnamed_26,
    pub expose: C2RustUnnamed_25,
    pub graphicsExposure: C2RustUnnamed_24,
    pub noExposure: C2RustUnnamed_23,
    pub visibility: C2RustUnnamed_22,
    pub createNotify: C2RustUnnamed_21,
    pub destroyNotify: C2RustUnnamed_20,
    pub unmapNotify: C2RustUnnamed_19,
    pub mapNotify: C2RustUnnamed_18,
    pub mapRequest: C2RustUnnamed_17,
    pub reparent: C2RustUnnamed_16,
    pub configureNotify: C2RustUnnamed_15,
    pub configureRequest: C2RustUnnamed_14,
    pub gravity: C2RustUnnamed_13,
    pub resizeRequest: C2RustUnnamed_12,
    pub circulate: C2RustUnnamed_11,
    pub property: C2RustUnnamed_10,
    pub selectionClear: C2RustUnnamed_9,
    pub selectionRequest: C2RustUnnamed_8,
    pub selectionNotify: C2RustUnnamed_7,
    pub colormap: C2RustUnnamed_6,
    pub mappingNotify: C2RustUnnamed_5,
    pub clientMessage: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub l: C2RustUnnamed_4,
    pub s: C2RustUnnamed_3,
    pub b: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_0: CARD32,
    pub bytes: [INT8; 20],
}
pub type INT8 = libc::c_schar;
pub type CARD32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub type_0: CARD32,
    pub shorts0: INT16,
    pub shorts1: INT16,
    pub shorts2: INT16,
    pub shorts3: INT16,
    pub shorts4: INT16,
    pub shorts5: INT16,
    pub shorts6: INT16,
    pub shorts7: INT16,
    pub shorts8: INT16,
    pub shorts9: INT16,
}
pub type INT16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub type_0: CARD32,
    pub longs0: INT32,
    pub longs1: INT32,
    pub longs2: INT32,
    pub longs3: INT32,
    pub longs4: INT32,
}
pub type INT32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub pad00: CARD32,
    pub request: CARD8,
    pub firstKeyCode: CARD8,
    pub count: CARD8,
    pub pad1: BYTE,
}
pub type BYTE = CARD8;
pub type CARD8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub colormap: CARD32,
    pub new: BOOL,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
}
pub type BOOL = CARD8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub owner: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
    pub time: CARD32,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: CARD16,
}
pub type CARD16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub place: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub width: CARD16,
    pub height: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub sibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub valueMask: CARD16,
    pub pad1: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub aboveSibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub fromConfigure: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub state: CARD8,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub minorEvent: CARD16,
    pub majorEvent: BYTE,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub minorEvent: CARD16,
    pub count: CARD16,
    pub majorEvent: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub count: CARD16,
    pub pad2: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub mode: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub mode: BYTE,
    pub flags: BYTE,
}
pub type KeyButMask = CARD16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub sameScreen: BOOL,
    pub pad1: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub type_0: BYTE,
    pub detail: BYTE,
    pub sequenceNumber: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnWatchInfo {
    pub fn_0: XConnectionWatchProc,
    pub client_data: XPointer,
    pub next: *mut _XConnWatchInfo,
}
pub type XConnectionWatchProc = Option::<
    unsafe extern "C" fn(
        *mut Display,
        XPointer,
        libc::c_int,
        libc::c_int,
        *mut XPointer,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnectionInfo {
    pub fd: libc::c_int,
    pub read_callback: _XInternalConnectionProc,
    pub call_data: XPointer,
    pub watch_data: *mut XPointer,
    pub next: *mut _XConnectionInfo,
}
pub type _XInternalConnectionProc = Option::<
    unsafe extern "C" fn(*mut Display, libc::c_int, XPointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExten {
    pub next: *mut _XExten,
    pub codes: XExtCodes,
    pub create_GC: CreateGCType,
    pub copy_GC: CopyGCType,
    pub flush_GC: FlushGCType,
    pub free_GC: FreeGCType,
    pub create_Font: CreateFontType,
    pub free_Font: FreeFontType,
    pub close_display: CloseDisplayType,
    pub error: ErrorType,
    pub error_string: ErrorStringType,
    pub name: *mut libc::c_char,
    pub error_values: PrintErrorType,
    pub before_flush: BeforeFlushType,
    pub next_flush: *mut _XExten,
}
pub type BeforeFlushType = Option::<
    unsafe extern "C" fn(
        *mut Display,
        *mut XExtCodes,
        *const libc::c_char,
        libc::c_long,
    ) -> (),
>;
pub type PrintErrorType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent, *mut libc::c_void) -> (),
>;
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
pub type ErrorStringType = Option::<
    unsafe extern "C" fn(
        *mut Display,
        libc::c_int,
        *mut XExtCodes,
        *mut libc::c_char,
        libc::c_int,
    ) -> *mut libc::c_char,
>;
pub type ErrorType = Option::<
    unsafe extern "C" fn(
        *mut Display,
        *mut xError,
        *mut XExtCodes,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xError {
    pub type_0: BYTE,
    pub errorCode: BYTE,
    pub sequenceNumber: CARD16,
    pub resourceID: CARD32,
    pub minorCode: CARD16,
    pub majorCode: CARD8,
    pub pad1: BYTE,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
pub type CloseDisplayType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XExtCodes) -> libc::c_int,
>;
pub type FreeFontType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XFontStruct, *mut XExtCodes) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: libc::c_uint,
    pub min_char_or_byte2: libc::c_uint,
    pub max_char_or_byte2: libc::c_uint,
    pub min_byte1: libc::c_uint,
    pub max_byte1: libc::c_uint,
    pub all_chars_exist: libc::c_int,
    pub default_char: libc::c_uint,
    pub n_properties: libc::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: libc::c_short,
    pub rbearing: libc::c_short,
    pub width: libc::c_short,
    pub ascent: libc::c_short,
    pub descent: libc::c_short,
    pub attributes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: libc::c_ulong,
}
pub type CreateFontType = Option::<
    unsafe extern "C" fn(*mut Display, *mut XFontStruct, *mut XExtCodes) -> libc::c_int,
>;
pub type FreeGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
pub type FlushGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
pub type CopyGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
pub type CreateGCType = Option::<
    unsafe extern "C" fn(*mut Display, GC, *mut XExtCodes) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XSQEvent {
    pub next: *mut _XSQEvent,
    pub event: XEvent,
    pub qserial_num: libc::c_ulong,
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub defaultCCCs: XPointer,
    pub clientCmaps: XPointer,
    pub perVisualIntensityMaps: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XLockPtrs {
    pub lock_display: Option::<unsafe extern "C" fn(*mut Display) -> ()>,
    pub unlock_display: Option::<unsafe extern "C" fn(*mut Display) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XInternalAsync {
    pub next: *mut _XInternalAsync,
    pub handler: Option::<
        unsafe extern "C" fn(
            *mut Display,
            *mut xReply,
            *mut libc::c_char,
            libc::c_int,
            XPointer,
        ) -> libc::c_int,
    >,
    pub data: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union xReply {
    pub generic: xGenericReply,
    pub geom: xGetGeometryReply,
    pub tree: xQueryTreeReply,
    pub atom: xInternAtomReply,
    pub atomName: xGetAtomNameReply,
    pub property: xGetPropertyReply,
    pub listProperties: xListPropertiesReply,
    pub selection: xGetSelectionOwnerReply,
    pub grabPointer: xGrabPointerReply,
    pub grabKeyboard: xGrabKeyboardReply,
    pub pointer: xQueryPointerReply,
    pub motionEvents: xGetMotionEventsReply,
    pub coords: xTranslateCoordsReply,
    pub inputFocus: xGetInputFocusReply,
    pub textExtents: xQueryTextExtentsReply,
    pub fonts: xListFontsReply,
    pub fontPath: xGetFontPathReply,
    pub image: xGetImageReply,
    pub colormaps: xListInstalledColormapsReply,
    pub allocColor: xAllocColorReply,
    pub allocNamedColor: xAllocNamedColorReply,
    pub colorCells: xAllocColorCellsReply,
    pub colorPlanes: xAllocColorPlanesReply,
    pub colors: xQueryColorsReply,
    pub lookupColor: xLookupColorReply,
    pub bestSize: xQueryBestSizeReply,
    pub extension: xQueryExtensionReply,
    pub extensions: xListExtensionsReply,
    pub setModifierMapping: xSetModifierMappingReply,
    pub getModifierMapping: xGetModifierMappingReply,
    pub setPointerMapping: xSetPointerMappingReply,
    pub getKeyboardMapping: xGetKeyboardMappingReply,
    pub getPointerMapping: xGetPointerMappingReply,
    pub pointerControl: xGetPointerControlReply,
    pub screenSaver: xGetScreenSaverReply,
    pub hosts: xListHostsReply,
    pub error: xError,
    pub event: xEvent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListHostsReply {
    pub type_0: BYTE,
    pub enabled: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nHosts: CARD16,
    pub pad1: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetScreenSaverReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub timeout: CARD16,
    pub interval: CARD16,
    pub preferBlanking: BOOL,
    pub allowExposures: BOOL,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerControlReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub accelNumerator: CARD16,
    pub accelDenominator: CARD16,
    pub threshold: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerMappingReply {
    pub type_0: BYTE,
    pub nElts: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetKeyboardMappingReply {
    pub type_0: BYTE,
    pub keySymsPerKeyCode: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
pub type xSetMappingReply = xSetPointerMappingReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xSetPointerMappingReply {
    pub type_0: BYTE,
    pub success: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetModifierMappingReply {
    pub type_0: BYTE,
    pub numKeyPerModifier: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
pub type xSetModifierMappingReply = xSetMappingReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListExtensionsReply {
    pub type_0: BYTE,
    pub nExtensions: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryExtensionReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub present: BOOL,
    pub major_opcode: CARD8,
    pub first_event: CARD8,
    pub first_error: CARD8,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryBestSizeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub width: CARD16,
    pub height: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xLookupColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryColorsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColors: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorPlanesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub pad2: CARD16,
    pub redMask: CARD32,
    pub greenMask: CARD32,
    pub blueMask: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorCellsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub nMasks: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocNamedColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pixel: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub red: CARD16,
    pub green: CARD16,
    pub blue: CARD16,
    pub pad2: CARD16,
    pub pixel: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListInstalledColormapsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColormaps: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetImageReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub visual: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetFontPathReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPaths: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListFontsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nFonts: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTextExtentsReply {
    pub type_0: BYTE,
    pub drawDirection: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub fontAscent: INT16,
    pub fontDescent: INT16,
    pub overallAscent: INT16,
    pub overallDescent: INT16,
    pub overallWidth: INT32,
    pub overallLeft: INT32,
    pub overallRight: INT32,
    pub pad: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetInputFocusReply {
    pub type_0: BYTE,
    pub revertTo: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub focus: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xTranslateCoordsReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub child: CARD32,
    pub dstX: INT16,
    pub dstY: INT16,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetMotionEventsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nEvents: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryPointerReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub winX: INT16,
    pub winY: INT16,
    pub mask: CARD16,
    pub pad1: CARD16,
    pub pad: CARD32,
}
pub type xGrabPointerReply = xGrabKeyboardReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGrabKeyboardReply {
    pub type_0: BYTE,
    pub status: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetSelectionOwnerReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub owner: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListPropertiesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nProperties: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPropertyReply {
    pub type_0: BYTE,
    pub format: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub propertyType: CARD32,
    pub bytesAfter: CARD32,
    pub nItems: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetAtomNameReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nameLength: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xInternAtomReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub atom: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTreeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub parent: CARD32,
    pub nChildren: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetGeometryReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub pad1: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGenericReply {
    pub type_0: BYTE,
    pub data1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub data00: CARD32,
    pub data01: CARD32,
    pub data02: CARD32,
    pub data03: CARD32,
    pub data04: CARD32,
    pub data05: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: libc::c_int,
    pub modifiermap: *mut KeyCode,
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
pub struct _XFreeFuncs {
    pub atoms: FreeFuncType,
    pub modifiermap: FreeModmapType,
    pub key_bindings: FreeFuncType,
    pub context_db: FreeFuncType,
    pub defaultCCCs: FreeFuncType,
    pub clientCmaps: FreeFuncType,
    pub intensityMaps: FreeFuncType,
    pub im_filters: FreeFuncType,
    pub xkb: FreeFuncType,
}
pub type FreeFuncType = Option::<unsafe extern "C" fn(*mut Display) -> ()>;
pub type FreeModmapType = Option::<
    unsafe extern "C" fn(*mut XModifierKeymap) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
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
pub type _XPrivDisplay = *mut C2RustUnnamed_32;
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
    pub last_solid_cache: [C2RustUnnamed_33; 2],
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
pub struct C2RustUnnamed_33 {
    pub color: uint32_t,
    pub index: libc::c_int,
}
pub type cairo_xlib_shm_display_t = _cairo_xlib_shm_display;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_shm_display {
    pub has_pixmaps: libc::c_int,
    pub opcode: libc::c_int,
    pub event: libc::c_int,
    pub window: Window,
    pub last_request: libc::c_ulong,
    pub last_event: libc::c_ulong,
    pub surfaces: cairo_list_t,
    pub pool: cairo_list_t,
    pub info: pqueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pqueue {
    pub size: libc::c_int,
    pub max_size: libc::c_int,
    pub elements: *mut *mut cairo_xlib_shm_info_t,
}
pub type cairo_xlib_shm_info_t = _cairo_xlib_shm_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_shm_info {
    pub last_request: libc::c_ulong,
    pub mem: *mut libc::c_void,
    pub size: size_t,
    pub pool: *mut cairo_xlib_shm_t,
}
pub type cairo_xlib_shm_t = _cairo_xlib_shm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_shm {
    pub mem: cairo_mempool_t,
    pub shm: XShmSegmentInfo,
    pub attached: libc::c_ulong,
    pub link: cairo_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XShmSegmentInfo {
    pub shmseg: ShmSeg,
    pub shmid: libc::c_int,
    pub shmaddr: *mut libc::c_char,
    pub readOnly: libc::c_int,
}
pub type ShmSeg = libc::c_ulong;
pub type cairo_mempool_t = _cairo_mempool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mempool {
    pub base: *mut libc::c_char,
    pub blocks: *mut _cairo_memblock,
    pub free: [cairo_list_t; 32],
    pub map: *mut libc::c_uchar,
    pub num_blocks: libc::c_uint,
    pub min_bits: libc::c_int,
    pub num_sizes: libc::c_int,
    pub max_free_bits: libc::c_int,
    pub free_bytes: size_t,
    pub max_bytes: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_memblock {
    pub bits: libc::c_int,
    pub link: cairo_list_t,
}
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
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: __time_t,
    pub shm_dtime: __time_t,
    pub shm_ctime: __time_t,
    pub shm_cpid: __pid_t,
    pub shm_lpid: __pid_t,
    pub shm_nattch: shmatt_t,
    pub __glibc_reserved5: __syscall_ulong_t,
    pub __glibc_reserved6: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
    pub __key: __key_t,
    pub uid: __uid_t,
    pub gid: __gid_t,
    pub cuid: __uid_t,
    pub cgid: __gid_t,
    pub mode: __mode_t,
    pub __seq: libc::c_ushort,
    pub __pad2: libc::c_ushort,
    pub __glibc_reserved1: __syscall_ulong_t,
    pub __glibc_reserved2: __syscall_ulong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XShmCompletionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
    pub shmseg: ShmSeg,
    pub offset: libc::c_ulong,
}
pub type cairo_xlib_shm_surface_t = _cairo_xlib_shm_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_shm_surface {
    pub image: cairo_image_surface_t,
    pub link: cairo_list_t,
    pub info: *mut cairo_xlib_shm_info_t,
    pub pixmap: Pixmap,
    pub active: libc::c_ulong,
    pub idle: libc::c_int,
}
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
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
unsafe extern "C" fn _cairo_xlib_vendor_is_xorg(mut dpy: *mut Display) -> cairo_bool_t {
    let vendor: *const libc::c_char = (*(dpy as _XPrivDisplay)).vendor;
    return (!(strstr(vendor, b"X.Org\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(vendor, b"Xorg\0" as *const u8 as *const libc::c_char)).is_null())
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh4 = (*entry).next;
    *fresh4 = entry;
    let ref mut fresh5 = (*entry).prev;
    *fresh5 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh6 = (*next).prev;
    *fresh6 = entry;
    let ref mut fresh7 = (*entry).next;
    *fresh7 = next;
    let ref mut fresh8 = (*entry).prev;
    *fresh8 = prev;
    let ref mut fresh9 = (*prev).next;
    *fresh9 = entry;
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
    let ref mut fresh10 = (*next).prev;
    *fresh10 = prev;
    let ref mut fresh11 = (*prev).next;
    *fresh11 = next;
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
unsafe extern "C" fn seqno_passed(
    mut a: libc::c_ulong,
    mut b: libc::c_ulong,
) -> cairo_bool_t {
    return (b.wrapping_sub(a) as libc::c_long >= 0 as libc::c_int as libc::c_long)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn seqno_before(
    mut a: libc::c_ulong,
    mut b: libc::c_ulong,
) -> cairo_bool_t {
    return (b.wrapping_sub(a) as libc::c_long > 0 as libc::c_int as libc::c_long)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn seqno_after(
    mut a: libc::c_ulong,
    mut b: libc::c_ulong,
) -> cairo_bool_t {
    return (a.wrapping_sub(b) as libc::c_long > 0 as libc::c_int as libc::c_long)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _pqueue_init(mut pq: *mut pqueue) -> cairo_status_t {
    (*pq).max_size = 32 as libc::c_int;
    (*pq).size = 0 as libc::c_int;
    let ref mut fresh12 = (*pq).elements;
    *fresh12 = _cairo_malloc_ab(
        (*pq).max_size as size_t,
        ::std::mem::size_of::<*mut cairo_xlib_shm_info_t>() as libc::c_ulong,
    ) as *mut *mut cairo_xlib_shm_info_t;
    if ((*pq).elements).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh13 = *((*pq).elements).offset(1 as libc::c_int as isize);
    *fresh13 = 0 as *mut cairo_xlib_shm_info_t;
    return CAIRO_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn _pqueue_fini(mut pq: *mut pqueue) {
    free((*pq).elements as *mut libc::c_void);
}
unsafe extern "C" fn _pqueue_grow(mut pq: *mut pqueue) -> cairo_status_t {
    let mut new_elements: *mut *mut cairo_xlib_shm_info_t = 0
        as *mut *mut cairo_xlib_shm_info_t;
    new_elements = _cairo_realloc_ab(
        (*pq).elements as *mut libc::c_void,
        (2 as libc::c_int * (*pq).max_size) as size_t,
        ::std::mem::size_of::<*mut cairo_xlib_shm_info_t>() as libc::c_ulong,
    ) as *mut *mut cairo_xlib_shm_info_t;
    if new_elements.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh14 = (*pq).elements;
    *fresh14 = new_elements;
    (*pq).max_size *= 2 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _pqueue_shrink(mut pq: *mut pqueue, mut min_size: libc::c_int) {
    let mut new_elements: *mut *mut cairo_xlib_shm_info_t = 0
        as *mut *mut cairo_xlib_shm_info_t;
    if min_size > (*pq).max_size {
        return;
    }
    new_elements = _cairo_realloc_ab(
        (*pq).elements as *mut libc::c_void,
        min_size as size_t,
        ::std::mem::size_of::<*mut cairo_xlib_shm_info_t>() as libc::c_ulong,
    ) as *mut *mut cairo_xlib_shm_info_t;
    if new_elements.is_null() {
        return;
    }
    let ref mut fresh15 = (*pq).elements;
    *fresh15 = new_elements;
    (*pq).max_size = min_size;
}
#[inline]
unsafe extern "C" fn _pqueue_push(
    mut pq: *mut pqueue,
    mut info: *mut cairo_xlib_shm_info_t,
) -> cairo_status_t {
    let mut elements: *mut *mut cairo_xlib_shm_info_t = 0
        as *mut *mut cairo_xlib_shm_info_t;
    let mut i: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    if (*pq).size + 1 as libc::c_int == (*pq).max_size {
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _pqueue_grow(pq);
        if status as u64 != 0 {
            return status;
        }
    }
    elements = (*pq).elements;
    let ref mut fresh16 = (*pq).size;
    *fresh16 += 1;
    i = *fresh16;
    while i != 1 as libc::c_int
        && {
            parent = i >> 1 as libc::c_int;
            (*info).last_request < (**elements.offset(parent as isize)).last_request
        }
    {
        let ref mut fresh17 = *elements.offset(i as isize);
        *fresh17 = *elements.offset(parent as isize);
        i = parent;
    }
    let ref mut fresh18 = *elements.offset(i as isize);
    *fresh18 = info;
    return CAIRO_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn _pqueue_pop(mut pq: *mut pqueue) {
    let mut elements: *mut *mut cairo_xlib_shm_info_t = (*pq).elements;
    let mut tail: *mut cairo_xlib_shm_info_t = 0 as *mut cairo_xlib_shm_info_t;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let ref mut fresh19 = (*pq).size;
    let fresh20 = *fresh19;
    *fresh19 = *fresh19 - 1;
    tail = *elements.offset(fresh20 as isize);
    if (*pq).size == 0 as libc::c_int {
        let ref mut fresh21 = *elements.offset(1 as libc::c_int as isize);
        *fresh21 = 0 as *mut cairo_xlib_shm_info_t;
        _pqueue_shrink(pq, 32 as libc::c_int);
        return;
    }
    i = 1 as libc::c_int;
    loop {
        child = i << 1 as libc::c_int;
        if !(child <= (*pq).size) {
            break;
        }
        if child != (*pq).size
            && (**elements.offset((child + 1 as libc::c_int) as isize)).last_request
                < (**elements.offset(child as isize)).last_request
        {
            child += 1;
        }
        if (**elements.offset(child as isize)).last_request >= (*tail).last_request {
            break;
        }
        let ref mut fresh22 = *elements.offset(i as isize);
        *fresh22 = *elements.offset(child as isize);
        i = child;
    }
    let ref mut fresh23 = *elements.offset(i as isize);
    *fresh23 = tail;
}
static mut _x_error_occurred: cairo_bool_t = 0;
unsafe extern "C" fn _check_error_handler(
    mut display: *mut Display,
    mut event: *mut XErrorEvent,
) -> libc::c_int {
    _x_error_occurred = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn can_use_shm(
    mut dpy: *mut Display,
    mut has_pixmap: *mut libc::c_int,
) -> cairo_bool_t {
    let mut shm: XShmSegmentInfo = XShmSegmentInfo {
        shmseg: 0,
        shmid: 0,
        shmaddr: 0 as *mut libc::c_char,
        readOnly: 0,
    };
    let mut old_handler: Option::<
        unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
    > = None;
    let mut success: libc::c_int = 0;
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    if XShmQueryExtension(dpy) == 0 {
        return 0 as libc::c_int;
    }
    XShmQueryVersion(dpy, &mut major, &mut minor, has_pixmap);
    shm
        .shmid = shmget(
        0 as libc::c_int,
        0x1000 as libc::c_int as size_t,
        0o1000 as libc::c_int | 0o600 as libc::c_int,
    );
    if shm.shmid == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    shm.readOnly = 0 as libc::c_int;
    shm
        .shmaddr = shmat(shm.shmid, 0 as *const libc::c_void, 0 as libc::c_int)
        as *mut libc::c_char;
    if shm.shmaddr == -(1 as libc::c_int) as *mut libc::c_char {
        shmctl(shm.shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
        return 0 as libc::c_int;
    }
    _x_error_occurred = 0 as libc::c_int;
    XLockDisplay(dpy);
    XSync(dpy, 0 as libc::c_int);
    old_handler = XSetErrorHandler(
        Some(
            _check_error_handler
                as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
        ),
    );
    success = XShmAttach(dpy, &mut shm);
    if success != 0 {
        XShmDetach(dpy, &mut shm);
    }
    XSync(dpy, 0 as libc::c_int);
    XSetErrorHandler(old_handler);
    XUnlockDisplay(dpy);
    shmctl(shm.shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
    shmdt(shm.shmaddr as *const libc::c_void);
    return (success != 0 && _x_error_occurred == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn peek_display(mut device: *mut cairo_device_t) -> *mut Display {
    return (*(device as *mut cairo_xlib_display_t)).display;
}
#[inline]
unsafe extern "C" fn peek_processed(mut device: *mut cairo_device_t) -> libc::c_ulong {
    return (*(peek_display(device) as _XPrivDisplay)).last_request_read;
}
unsafe extern "C" fn _cairo_xlib_display_shm_pool_destroy(
    mut display: *mut cairo_xlib_display_t,
    mut pool: *mut cairo_xlib_shm_t,
) {
    shmdt((*pool).shm.shmaddr as *const libc::c_void);
    if !((*display).display).is_null() {
        XShmDetach((*display).display, &mut (*pool).shm);
    }
    _cairo_mempool_fini(&mut (*pool).mem);
    cairo_list_del(&mut (*pool).link);
    free(pool as *mut libc::c_void);
}
unsafe extern "C" fn send_event(
    mut display: *mut cairo_xlib_display_t,
    mut info: *mut cairo_xlib_shm_info_t,
    mut seqno: libc::c_ulong,
) {
    let mut ev: XShmCompletionEvent = XShmCompletionEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: 0 as *mut Display,
        drawable: 0,
        major_code: 0,
        minor_code: 0,
        shmseg: 0,
        offset: 0,
    };
    if seqno_after(seqno, (*(*display).shm).last_event) == 0 {
        return;
    }
    ev.type_0 = (*(*display).shm).event;
    ev.send_event = 1 as libc::c_int;
    ev.serial = XNextRequest((*display).display);
    ev.drawable = (*(*display).shm).window;
    ev.major_code = (*(*display).shm).opcode;
    ev.minor_code = 3 as libc::c_int;
    ev.shmseg = (*(*info).pool).shm.shmid as ShmSeg;
    ev
        .offset = ((*info).mem as *mut libc::c_char)
        .offset_from((*(*info).pool).shm.shmaddr) as libc::c_long as libc::c_ulong;
    XSendEvent(
        (*display).display,
        ev.drawable,
        0 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        &mut ev as *mut XShmCompletionEvent as *mut XEvent,
    );
    (*(*display).shm).last_event = ev.serial;
}
unsafe extern "C" fn _cairo_xlib_display_sync(mut display: *mut cairo_xlib_display_t) {
    let mut info: *mut cairo_xlib_shm_info_t = 0 as *mut cairo_xlib_shm_info_t;
    let mut pq: *mut pqueue = &mut (*(*display).shm).info;
    XSync((*display).display, 0 as libc::c_int);
    loop {
        info = *((*pq).elements).offset(1 as libc::c_int as isize);
        if info.is_null() {
            break;
        }
        _cairo_mempool_free(&mut (*(*info).pool).mem, (*info).mem);
        _pqueue_pop(&mut (*(*display).shm).info);
        free(info as *mut libc::c_void);
    };
}
unsafe extern "C" fn _cairo_xlib_shm_info_cleanup(
    mut display: *mut cairo_xlib_display_t,
) {
    let mut info: *mut cairo_xlib_shm_info_t = 0 as *mut cairo_xlib_shm_info_t;
    let mut dpy: *mut Display = (*display).display;
    let mut pq: *mut pqueue = &mut (*(*display).shm).info;
    let mut processed: libc::c_ulong = 0;
    if (*((*pq).elements).offset(1 as libc::c_int as isize)).is_null() {
        return;
    }
    XEventsQueued(dpy, 1 as libc::c_int);
    processed = (*(dpy as _XPrivDisplay)).last_request_read;
    info = *((*pq).elements).offset(1 as libc::c_int as isize);
    loop {
        if seqno_passed((*info).last_request, processed) == 0 {
            send_event(display, info, (*(*display).shm).last_request);
            return;
        }
        _cairo_mempool_free(&mut (*(*info).pool).mem, (*info).mem);
        _pqueue_pop(&mut (*(*display).shm).info);
        free(info as *mut libc::c_void);
        info = *((*pq).elements).offset(1 as libc::c_int as isize);
        if info.is_null() {
            break;
        }
    };
}
unsafe extern "C" fn _cairo_xlib_shm_info_find(
    mut display: *mut cairo_xlib_display_t,
    mut size: size_t,
    mut ptr: *mut *mut libc::c_void,
    mut last_request: *mut libc::c_ulong,
) -> *mut cairo_xlib_shm_t {
    let mut info: *mut cairo_xlib_shm_info_t = 0 as *mut cairo_xlib_shm_info_t;
    let mut pq: *mut pqueue = &mut (*(*display).shm).info;
    if (*((*pq).elements).offset(1 as libc::c_int as isize)).is_null() {
        return 0 as *mut cairo_xlib_shm_t;
    }
    info = *((*pq).elements).offset(1 as libc::c_int as isize);
    loop {
        let mut pool: *mut cairo_xlib_shm_t = (*info).pool;
        *last_request = (*info).last_request;
        _pqueue_pop(&mut (*(*display).shm).info);
        _cairo_mempool_free(&mut (*pool).mem, (*info).mem);
        free(info as *mut libc::c_void);
        if (*pool).mem.free_bytes >= size {
            let mut mem: *mut libc::c_void = _cairo_mempool_alloc(
                &mut (*pool).mem,
                size,
            );
            if !mem.is_null() {
                *ptr = mem;
                return pool;
            }
        }
        info = *((*pq).elements).offset(1 as libc::c_int as isize);
        if info.is_null() {
            break;
        }
    }
    return 0 as *mut cairo_xlib_shm_t;
}
unsafe extern "C" fn _cairo_xlib_shm_pool_find(
    mut display: *mut cairo_xlib_display_t,
    mut size: size_t,
    mut ptr: *mut *mut libc::c_void,
) -> *mut cairo_xlib_shm_t {
    let mut pool: *mut cairo_xlib_shm_t = 0 as *mut cairo_xlib_shm_t;
    pool = ({
        let mut mptr__: *const cairo_list_t = (*(*display).shm).pool.next;
        (mptr__ as *mut libc::c_char).offset(-(608 as libc::c_ulong as isize))
            as *mut cairo_xlib_shm_t
    });
    while &mut (*pool).link as *mut cairo_list_t
        != &mut (*(*display).shm).pool as *mut cairo_list_t
    {
        if (*pool).mem.free_bytes >= size {
            let mut mem: *mut libc::c_void = _cairo_mempool_alloc(
                &mut (*pool).mem,
                size,
            );
            if !mem.is_null() {
                *ptr = mem;
                return pool;
            }
        }
        pool = ({
            let mut mptr__: *const cairo_list_t = (*pool).link.next;
            (mptr__ as *mut libc::c_char).offset(-(608 as libc::c_ulong as isize))
                as *mut cairo_xlib_shm_t
        });
    }
    return 0 as *mut cairo_xlib_shm_t;
}
unsafe extern "C" fn _cairo_xlib_shm_pool_cleanup(
    mut display: *mut cairo_xlib_display_t,
) {
    let mut pool: *mut cairo_xlib_shm_t = 0 as *mut cairo_xlib_shm_t;
    let mut next: *mut cairo_xlib_shm_t = 0 as *mut cairo_xlib_shm_t;
    let mut processed: libc::c_ulong = 0;
    processed = (*((*display).display as _XPrivDisplay)).last_request_read;
    pool = ({
        let mut mptr__: *const cairo_list_t = (*(*display).shm).pool.next;
        (mptr__ as *mut libc::c_char).offset(-(608 as libc::c_ulong as isize))
            as *mut cairo_xlib_shm_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*pool).link.next;
        (mptr__ as *mut libc::c_char).offset(-(608 as libc::c_ulong as isize))
            as *mut cairo_xlib_shm_t
    });
    while &mut (*pool).link as *mut cairo_list_t
        != &mut (*(*display).shm).pool as *mut cairo_list_t
    {
        if seqno_passed((*pool).attached, processed) == 0 {
            break;
        }
        if (*pool).mem.free_bytes == (*pool).mem.max_bytes {
            _cairo_xlib_display_shm_pool_destroy(display, pool);
        }
        pool = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(608 as libc::c_ulong as isize))
                as *mut cairo_xlib_shm_t
        });
    }
}
unsafe extern "C" fn _cairo_xlib_shm_pool_create(
    mut display: *mut cairo_xlib_display_t,
    mut size: size_t,
    mut ptr: *mut *mut libc::c_void,
) -> *mut cairo_xlib_shm_t {
    let mut dpy: *mut Display = (*display).display;
    let mut pool: *mut cairo_xlib_shm_t = 0 as *mut cairo_xlib_shm_t;
    let mut bytes: size_t = 0;
    let mut maxbits: size_t = 16 as libc::c_int as size_t;
    let mut minbits: size_t = 8 as libc::c_int as size_t;
    let mut success: libc::c_int = 0;
    pool = (if ::std::mem::size_of::<cairo_xlib_shm_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_shm_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_shm_t;
    if pool.is_null() {
        return 0 as *mut cairo_xlib_shm_t;
    }
    bytes = ((1 as libc::c_int) << maxbits) as size_t;
    while bytes <= size {
        bytes <<= 1 as libc::c_int;
        maxbits = maxbits.wrapping_add(1);
    }
    bytes <<= 3 as libc::c_int;
    minbits = (minbits as libc::c_ulong)
        .wrapping_add(
            maxbits
                .wrapping_sub(16 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    (*pool)
        .shm
        .shmid = shmget(
        0 as libc::c_int,
        bytes,
        0o1000 as libc::c_int | 0o600 as libc::c_int,
    );
    while (*pool).shm.shmid == -(1 as libc::c_int)
        && bytes >= (2 as libc::c_int as libc::c_ulong).wrapping_mul(size)
    {
        bytes >>= 1 as libc::c_int;
        (*pool)
            .shm
            .shmid = shmget(
            0 as libc::c_int,
            bytes,
            0o1000 as libc::c_int | 0o600 as libc::c_int,
        );
    }
    if !((*pool).shm.shmid == -(1 as libc::c_int)) {
        (*pool).shm.readOnly = 0 as libc::c_int;
        let ref mut fresh24 = (*pool).shm.shmaddr;
        *fresh24 = shmat((*pool).shm.shmid, 0 as *const libc::c_void, 0 as libc::c_int)
            as *mut libc::c_char;
        if (*pool).shm.shmaddr == -(1 as libc::c_int) as *mut libc::c_char {
            shmctl((*pool).shm.shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
        } else {
            (*pool).attached = XNextRequest(dpy);
            success = XShmAttach(dpy, &mut (*pool).shm);
            shmctl((*pool).shm.shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
            if !(success == 0) {
                if _cairo_mempool_init(
                    &mut (*pool).mem,
                    (*pool).shm.shmaddr as *mut libc::c_void,
                    bytes,
                    minbits as libc::c_int,
                    maxbits
                        .wrapping_sub(minbits)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                ) as u64 != 0
                {
                    XShmDetach(dpy, &mut (*pool).shm);
                } else {
                    cairo_list_add(&mut (*pool).link, &mut (*(*display).shm).pool);
                    *ptr = _cairo_mempool_alloc(&mut (*pool).mem, size);
                    if !(*ptr).is_null() {} else {
                        __assert_fail(
                            b"*ptr != NULL\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-xlib-surface-shm.c\0" as *const u8
                                as *const libc::c_char,
                            622 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 87],
                                &[libc::c_char; 87],
                            >(
                                b"cairo_xlib_shm_t *_cairo_xlib_shm_pool_create(cairo_xlib_display_t *, size_t, void **)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    return pool;
                }
            }
            shmdt((*pool).shm.shmaddr as *const libc::c_void);
        }
    }
    free(pool as *mut libc::c_void);
    return 0 as *mut cairo_xlib_shm_t;
}
unsafe extern "C" fn _cairo_xlib_shm_info_create(
    mut display: *mut cairo_xlib_display_t,
    mut size: size_t,
    mut will_sync: cairo_bool_t,
) -> *mut cairo_xlib_shm_info_t {
    let mut info: *mut cairo_xlib_shm_info_t = 0 as *mut cairo_xlib_shm_info_t;
    let mut pool: *mut cairo_xlib_shm_t = 0 as *mut cairo_xlib_shm_t;
    let mut last_request: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    _cairo_xlib_shm_info_cleanup(display);
    pool = _cairo_xlib_shm_pool_find(display, size, &mut mem);
    _cairo_xlib_shm_pool_cleanup(display);
    if pool.is_null() && will_sync != 0 {
        pool = _cairo_xlib_shm_info_find(display, size, &mut mem, &mut last_request);
    }
    if pool.is_null() {
        pool = _cairo_xlib_shm_pool_create(display, size, &mut mem);
    }
    if pool.is_null() {
        return 0 as *mut cairo_xlib_shm_info_t;
    }
    if !mem.is_null() {} else {
        __assert_fail(
            b"mem != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_xlib_shm_info_t *_cairo_xlib_shm_info_create(cairo_xlib_display_t *, size_t, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    info = (if ::std::mem::size_of::<cairo_xlib_shm_info_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_shm_info_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_shm_info_t;
    if info.is_null() {
        _cairo_mempool_free(&mut (*pool).mem, mem);
        return 0 as *mut cairo_xlib_shm_info_t;
    }
    let ref mut fresh25 = (*info).pool;
    *fresh25 = pool;
    let ref mut fresh26 = (*info).mem;
    *fresh26 = mem;
    (*info).size = size;
    (*info).last_request = last_request;
    return info;
}
unsafe extern "C" fn _cairo_xlib_shm_surface_flush(
    mut abstract_surface: *mut libc::c_void,
    mut flags: libc::c_uint,
) -> cairo_status_t {
    let mut shm: *mut cairo_xlib_shm_surface_t = abstract_surface
        as *mut cairo_xlib_shm_surface_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut dpy: *mut Display = 0 as *mut Display;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*shm).active == 0 as libc::c_int as libc::c_ulong {
        return CAIRO_STATUS_SUCCESS;
    }
    if ((*shm).image.base)._finishing() != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if seqno_passed((*shm).active, peek_processed((*shm).image.base.device)) != 0 {
        (*shm).active = 0 as libc::c_int as libc::c_ulong;
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_xlib_display_acquire((*shm).image.base.device, &mut display);
    if status as u64 != 0 {
        return status;
    }
    send_event(display, (*shm).info, (*shm).active);
    dpy = (*display).display;
    XEventsQueued(dpy, 1 as libc::c_int);
    while seqno_passed((*shm).active, (*(dpy as _XPrivDisplay)).last_request_read) == 0 {
        if !((*dpy).lock_fns).is_null() {
            (Some(((*(*dpy).lock_fns).lock_display).expect("non-null function pointer")))
                .expect("non-null function pointer")(dpy);
        }
        _XReadEvents(dpy);
        if !((*dpy).lock_fns).is_null() {
            (Some(
                ((*(*dpy).lock_fns).unlock_display).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(dpy);
        }
    }
    cairo_device_release(&mut (*display).base);
    (*shm).active = 0 as libc::c_int as libc::c_ulong;
    return CAIRO_STATUS_SUCCESS;
}
#[inline]
unsafe extern "C" fn active(
    mut shm: *mut cairo_xlib_shm_surface_t,
    mut dpy: *mut Display,
) -> cairo_bool_t {
    return ((*shm).active != 0
        && seqno_passed((*shm).active, (*(dpy as _XPrivDisplay)).last_request_read) == 0)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_xlib_shm_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut shm: *mut cairo_xlib_shm_surface_t = abstract_surface
        as *mut cairo_xlib_shm_surface_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !((*shm).image.base.damage).is_null() {
        _cairo_damage_destroy((*shm).image.base.damage);
        let ref mut fresh27 = (*shm).image.base.damage;
        *fresh27 = _cairo_damage_create_in_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    status = _cairo_xlib_display_acquire((*shm).image.base.device, &mut display);
    if status as u64 != 0 {
        return status;
    }
    if (*shm).pixmap != 0 {
        XFreePixmap((*display).display, (*shm).pixmap);
    }
    if active(shm, (*display).display) != 0 {
        (*(*shm).info).last_request = (*shm).active;
        _pqueue_push(&mut (*(*display).shm).info, (*shm).info);
        if seqno_before((*(*display).shm).last_request, (*shm).active) != 0 {
            (*(*display).shm).last_request = (*shm).active;
        }
    } else {
        _cairo_mempool_free(&mut (*(*(*shm).info).pool).mem, (*(*shm).info).mem);
        free((*shm).info as *mut libc::c_void);
        _cairo_xlib_shm_pool_cleanup(display);
    }
    cairo_list_del(&mut (*shm).link);
    cairo_device_release(&mut (*display).base);
    return _cairo_image_surface_finish(abstract_surface);
}
static mut cairo_xlib_shm_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_IMAGE,
            finish: Some(
                _cairo_xlib_shm_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: Some(
                _cairo_image_surface_create_similar
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_content_t,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut cairo_surface_t,
            ),
            create_similar_image: None,
            map_to_image: Some(
                _cairo_image_surface_map_to_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_rectangle_int_t,
                    ) -> *mut cairo_image_surface_t,
            ),
            unmap_image: Some(
                _cairo_image_surface_unmap_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
            source: Some(
                _cairo_image_surface_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: Some(
                _cairo_image_surface_acquire_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut cairo_image_surface_t,
                        *mut *mut libc::c_void,
                    ) -> cairo_status_t,
            ),
            release_source_image: Some(
                _cairo_image_surface_release_source_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            snapshot: Some(
                _cairo_image_surface_snapshot
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
            ),
            copy_page: None,
            show_page: None,
            get_extents: Some(
                _cairo_image_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_image_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: Some(
                _cairo_xlib_shm_surface_flush
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> cairo_status_t,
            ),
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_image_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_image_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_image_surface_stroke
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
                _cairo_image_surface_fill
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
                _cairo_image_surface_glyphs
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
unsafe extern "C" fn has_shm(mut surface: *mut cairo_xlib_surface_t) -> cairo_bool_t {
    let mut display: *mut cairo_xlib_display_t = (*surface).base.device
        as *mut cairo_xlib_display_t;
    return ((*display).shm != 0 as *mut libc::c_void as *mut cairo_xlib_shm_display_t)
        as libc::c_int;
}
unsafe extern "C" fn has_shm_pixmaps(
    mut surface: *mut cairo_xlib_surface_t,
) -> libc::c_int {
    let mut display: *mut cairo_xlib_display_t = (*surface).base.device
        as *mut cairo_xlib_display_t;
    if ((*display).shm).is_null() {
        return 0 as libc::c_int;
    }
    return (*(*display).shm).has_pixmaps;
}
unsafe extern "C" fn _cairo_xlib_shm_surface_create(
    mut other: *mut cairo_xlib_surface_t,
    mut format: pixman_format_code_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut will_sync: cairo_bool_t,
    mut create_pixmap: libc::c_int,
) -> *mut cairo_xlib_shm_surface_t {
    let mut shm: *mut cairo_xlib_shm_surface_t = 0 as *mut cairo_xlib_shm_surface_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut image: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut stride: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if width > 32767 as libc::c_int || height > 32767 as libc::c_int {
        return 0 as *mut cairo_xlib_shm_surface_t;
    }
    stride = ((((format as libc::c_uint >> 24 as libc::c_int
        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << (format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint))
        .wrapping_mul(width as libc::c_uint)
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
        as libc::c_int;
    size = stride * height;
    if size < (1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int {
        return 0 as *mut cairo_xlib_shm_surface_t;
    }
    shm = (if ::std::mem::size_of::<cairo_xlib_shm_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_shm_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_shm_surface_t;
    if shm.is_null() {
        return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY)
            as *mut cairo_xlib_shm_surface_t;
    }
    _cairo_surface_init(
        &mut (*shm).image.base,
        &cairo_xlib_shm_surface_backend,
        (*other).base.device,
        _cairo_content_from_pixman_format(format),
        0 as libc::c_int,
    );
    if !(_cairo_xlib_display_acquire((*other).base.device, &mut display) as u64 != 0) {
        let ref mut fresh28 = (*shm).info;
        *fresh28 = _cairo_xlib_shm_info_create(display, size as size_t, will_sync);
        if !((*shm).info).is_null() {
            image = pixman_image_create_bits(
                format,
                width,
                height,
                (*(*shm).info).mem as *mut uint32_t,
                stride,
            );
            if image.is_null() {
                _cairo_mempool_free(&mut (*(*(*shm).info).pool).mem, (*(*shm).info).mem);
                free((*shm).info as *mut libc::c_void);
            } else {
                _cairo_image_surface_init(&mut (*shm).image, image, format);
                (*shm).pixmap = 0 as libc::c_int as Pixmap;
                if create_pixmap != 0 && size >= create_pixmap {
                    (*shm)
                        .pixmap = XShmCreatePixmap(
                        (*display).display,
                        (*other).drawable,
                        (*(*shm).info).mem as *mut libc::c_char,
                        &mut (*(*(*shm).info).pool).shm,
                        (*shm).image.width as libc::c_uint,
                        (*shm).image.height as libc::c_uint,
                        (*shm).image.depth as libc::c_uint,
                    );
                }
                (*shm).active = (*(*shm).info).last_request;
                (*shm).idle = -(5 as libc::c_int);
                if (*shm).active == 0 as libc::c_int as libc::c_ulong || will_sync != 0
                {} else {
                    __assert_fail(
                        b"shm->active == 0 || will_sync\0" as *const u8
                            as *const libc::c_char,
                        b"../src/cairo-xlib-surface-shm.c\0" as *const u8
                            as *const libc::c_char,
                        860 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 132],
                            &[libc::c_char; 132],
                        >(
                            b"cairo_xlib_shm_surface_t *_cairo_xlib_shm_surface_create(cairo_xlib_surface_t *, pixman_format_code_t, int, int, cairo_bool_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                cairo_list_add(&mut (*shm).link, &mut (*(*display).shm).surfaces);
                cairo_device_release(&mut (*display).base);
                return shm;
            }
        }
        cairo_device_release(&mut (*display).base);
    }
    free(shm as *mut libc::c_void);
    return 0 as *mut cairo_xlib_shm_surface_t;
}
unsafe extern "C" fn _cairo_xlib_surface_update_shm(
    mut surface: *mut cairo_xlib_surface_t,
) {
    let mut shm: *mut cairo_xlib_shm_surface_t = (*surface).shm
        as *mut cairo_xlib_shm_surface_t;
    let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
    let mut damage: *mut cairo_damage_t = 0 as *mut cairo_damage_t;
    let mut gc: GC = 0 as *mut _XGC;
    damage = _cairo_damage_reduce((*surface).base.damage);
    let ref mut fresh29 = (*surface).base.damage;
    *fresh29 = _cairo_damage_create();
    if !(_cairo_xlib_display_acquire((*surface).base.device, &mut display) as u64 != 0) {
        if !(_cairo_xlib_surface_get_gc(display, surface, &mut gc) as u64 != 0) {
            if (*surface).owns_pixmap == 0 {
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
            }
            if !((*damage).region).is_null() {
                let mut stack_rects: [XRectangle; 256] = [XRectangle {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                }; 256];
                let mut rects: *mut XRectangle = stack_rects.as_mut_ptr();
                let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                let mut n_rects: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                n_rects = cairo_region_num_rectangles((*damage).region);
                if !(n_rects == 0 as libc::c_int) {
                    if n_rects == 1 as libc::c_int {
                        cairo_region_get_rectangle(
                            (*damage).region,
                            0 as libc::c_int,
                            &mut r,
                        );
                        XCopyArea(
                            (*display).display,
                            (*surface).drawable,
                            (*shm).pixmap,
                            gc,
                            r.x,
                            r.y,
                            r.width as libc::c_uint,
                            r.height as libc::c_uint,
                            r.x,
                            r.y,
                        );
                    } else {
                        if n_rects
                            > (::std::mem::size_of::<[XRectangle; 256]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                                ) as libc::c_int
                        {
                            rects = _cairo_malloc_ab(
                                n_rects as size_t,
                                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                            ) as *mut XRectangle;
                            if rects.is_null() {
                                rects = stack_rects.as_mut_ptr();
                                n_rects = (::std::mem::size_of::<[XRectangle; 256]>()
                                    as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                                    ) as libc::c_int;
                            }
                        }
                        i = 0 as libc::c_int;
                        while i < n_rects {
                            cairo_region_get_rectangle((*damage).region, i, &mut r);
                            (*rects.offset(i as isize)).x = r.x as libc::c_short;
                            (*rects.offset(i as isize)).y = r.y as libc::c_short;
                            (*rects.offset(i as isize))
                                .width = r.width as libc::c_ushort;
                            (*rects.offset(i as isize))
                                .height = r.height as libc::c_ushort;
                            i += 1;
                        }
                        XSetClipRectangles(
                            (*display).display,
                            gc,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            rects,
                            i,
                            3 as libc::c_int,
                        );
                        XCopyArea(
                            (*display).display,
                            (*surface).drawable,
                            (*shm).pixmap,
                            gc,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            (*shm).image.width as libc::c_uint,
                            (*shm).image.height as libc::c_uint,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        );
                        if (*damage).status as libc::c_uint
                            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                            && !((*damage).region).is_null()
                        {
                            XSetClipMask(
                                (*display).display,
                                gc,
                                0 as libc::c_long as Pixmap,
                            );
                        }
                    }
                }
            } else {
                XCopyArea(
                    (*display).display,
                    (*surface).drawable,
                    (*shm).pixmap,
                    gc,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*shm).image.width as libc::c_uint,
                    (*shm).image.height as libc::c_uint,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            if (*surface).owns_pixmap == 0 {
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
                    (*display).display,
                    gc,
                    ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong,
                    &mut gcv_0,
                );
            }
            _cairo_xlib_display_sync(display);
            (*shm).active = 0 as libc::c_int as libc::c_ulong;
            let ref mut fresh30 = (*shm).idle;
            *fresh30 -= 1;
            _cairo_xlib_surface_put_gc(display, surface, gc);
        }
        cairo_device_release(&mut (*display).base);
    }
    _cairo_damage_destroy(damage);
}
unsafe extern "C" fn _cairo_xlib_surface_clear_shm(
    mut surface: *mut cairo_xlib_surface_t,
) {
    let mut shm: *mut cairo_xlib_shm_surface_t = (*surface).shm
        as *mut cairo_xlib_shm_surface_t;
    if (*shm).active == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"shm->active == 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
            975 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void _cairo_xlib_surface_clear_shm(cairo_xlib_surface_t *)\0"))
                .as_ptr(),
        );
    }
    _cairo_damage_destroy((*surface).base.damage);
    let ref mut fresh31 = (*surface).base.damage;
    *fresh31 = _cairo_damage_create();
    memset(
        (*shm).image.data as *mut libc::c_void,
        0 as libc::c_int,
        ((*shm).image.stride * (*shm).image.height as libc::c_long) as libc::c_ulong,
    );
    let ref mut fresh32 = (*shm).image.base;
    (*fresh32).set_is_clear(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn inc_idle(mut surface: *mut cairo_surface_t) {
    let mut shm: *mut cairo_xlib_shm_surface_t = surface
        as *mut cairo_xlib_shm_surface_t;
    let ref mut fresh33 = (*shm).idle;
    *fresh33 += 1;
}
unsafe extern "C" fn dec_idle(mut surface: *mut cairo_surface_t) {
    let mut shm: *mut cairo_xlib_shm_surface_t = surface
        as *mut cairo_xlib_shm_surface_t;
    let ref mut fresh34 = (*shm).idle;
    *fresh34 -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_get_shm(
    mut surface: *mut cairo_xlib_surface_t,
    mut overwrite: cairo_bool_t,
) -> *mut cairo_surface_t {
    if (*surface).fallback != 0 {
        if !((*surface).base.damage).is_null() {} else {
            __assert_fail(
                b"surface->base.damage\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
                1001 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"cairo_surface_t *_cairo_xlib_surface_get_shm(cairo_xlib_surface_t *, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if !((*surface).shm).is_null() {} else {
            __assert_fail(
                b"surface->shm\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
                1002 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"cairo_surface_t *_cairo_xlib_surface_get_shm(cairo_xlib_surface_t *, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if !((*(*surface).shm).damage).is_null() {} else {
            __assert_fail(
                b"surface->shm->damage\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
                1003 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"cairo_surface_t *_cairo_xlib_surface_get_shm(cairo_xlib_surface_t *, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        if ((*surface).shm).is_null() {
            let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
            let mut will_sync: cairo_bool_t = 0;
            if has_shm_pixmaps(surface) == 0 {
                return 0 as *mut cairo_surface_t;
            }
            if (*surface).width | (*surface).height < 32 as libc::c_int {
                return 0 as *mut cairo_surface_t;
            }
            pixman_format = _pixman_format_for_xlib_surface(surface);
            if pixman_format as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                return 0 as *mut cairo_surface_t;
            }
            will_sync = (((*surface).base).is_clear() == 0 && overwrite == 0)
                as libc::c_int;
            let ref mut fresh35 = (*surface).shm;
            *fresh35 = &mut (*(_cairo_xlib_shm_surface_create
                as unsafe extern "C" fn(
                    *mut cairo_xlib_surface_t,
                    pixman_format_code_t,
                    libc::c_int,
                    libc::c_int,
                    cairo_bool_t,
                    libc::c_int,
                ) -> *mut cairo_xlib_shm_surface_t)(
                surface,
                pixman_format,
                (*surface).width,
                (*surface).height,
                will_sync,
                1 as libc::c_int,
            ))
                .image
                .base;
            if ((*surface).shm).is_null() {
                return 0 as *mut cairo_surface_t;
            }
            if ((*surface).base.damage).is_null() {} else {
                __assert_fail(
                    b"surface->base.damage == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-xlib-surface-shm.c\0" as *const u8
                        as *const libc::c_char,
                    1030 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"cairo_surface_t *_cairo_xlib_surface_get_shm(cairo_xlib_surface_t *, cairo_bool_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (*surface).base.serial != 0 || (*surface).owns_pixmap == 0 {
                let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                rect.y = 0 as libc::c_int;
                rect.x = rect.y;
                rect.width = (*surface).width;
                rect.height = (*surface).height;
                let ref mut fresh36 = (*surface).base.damage;
                *fresh36 = _cairo_damage_add_rectangle(
                    0 as *mut cairo_damage_t,
                    &mut rect,
                );
            } else {
                let ref mut fresh37 = (*surface).base.damage;
                *fresh37 = _cairo_damage_create();
            }
            let ref mut fresh38 = (*(*surface).shm).damage;
            *fresh38 = _cairo_damage_create();
        }
        if overwrite != 0 {
            _cairo_damage_destroy((*surface).base.damage);
            let ref mut fresh39 = (*surface).base.damage;
            *fresh39 = _cairo_damage_create();
        }
        if ((*surface).base).is_clear() == 0 && (*(*surface).base.damage).dirty != 0 {
            _cairo_xlib_surface_update_shm(surface);
        }
        _cairo_xlib_shm_surface_flush(
            (*surface).shm as *mut libc::c_void,
            1 as libc::c_int as libc::c_uint,
        );
        if ((*surface).base).is_clear() as libc::c_int != 0
            && (*(*surface).base.damage).dirty != 0
        {
            _cairo_xlib_surface_clear_shm(surface);
        }
    }
    dec_idle((*surface).shm);
    return (*surface).shm;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_put_shm(
    mut surface: *mut cairo_xlib_surface_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).fallback == 0 {
        if !((*surface).shm).is_null() {
            inc_idle((*surface).shm);
        }
        return CAIRO_INT_STATUS_SUCCESS;
    }
    if (*(*(*surface).shm).damage).dirty != 0 {
        let mut shm: *mut cairo_xlib_shm_surface_t = (*surface).shm
            as *mut cairo_xlib_shm_surface_t;
        let mut display: *mut cairo_xlib_display_t = 0 as *mut cairo_xlib_display_t;
        let mut damage: *mut cairo_damage_t = 0 as *mut cairo_damage_t;
        let mut gc: GC = 0 as *mut _XGC;
        status = _cairo_xlib_display_acquire((*surface).base.device, &mut display)
            as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        damage = _cairo_damage_reduce((*shm).image.base.damage);
        let ref mut fresh40 = (*shm).image.base.damage;
        *fresh40 = _cairo_damage_create();
        if (*damage).status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && !((*damage).region).is_null()
        {
            let mut stack_rects: [XRectangle; 256] = [XRectangle {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }; 256];
            let mut rects: *mut XRectangle = stack_rects.as_mut_ptr();
            let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let mut n_rects: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            n_rects = cairo_region_num_rectangles((*damage).region);
            if !(n_rects == 0 as libc::c_int) {
                status = _cairo_xlib_surface_get_gc(display, surface, &mut gc)
                    as cairo_int_status_t;
                if !(status as u64 != 0) {
                    if n_rects == 1 as libc::c_int {
                        cairo_region_get_rectangle(
                            (*damage).region,
                            0 as libc::c_int,
                            &mut r,
                        );
                        _cairo_xlib_shm_surface_mark_active((*surface).shm);
                        XCopyArea(
                            (*display).display,
                            (*shm).pixmap,
                            (*surface).drawable,
                            gc,
                            r.x,
                            r.y,
                            r.width as libc::c_uint,
                            r.height as libc::c_uint,
                            r.x,
                            r.y,
                        );
                        current_block = 4567019141635105728;
                    } else {
                        if n_rects
                            > (::std::mem::size_of::<[XRectangle; 256]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                                ) as libc::c_int
                        {
                            rects = _cairo_malloc_ab(
                                n_rects as size_t,
                                ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                            ) as *mut XRectangle;
                            if rects.is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                    as cairo_int_status_t;
                                _cairo_xlib_surface_put_gc(display, surface, gc);
                                current_block = 210880132257114028;
                            } else {
                                current_block = 5689316957504528238;
                            }
                        } else {
                            current_block = 5689316957504528238;
                        }
                        match current_block {
                            210880132257114028 => {}
                            _ => {
                                i = 0 as libc::c_int;
                                while i < n_rects {
                                    cairo_region_get_rectangle((*damage).region, i, &mut r);
                                    (*rects.offset(i as isize)).x = r.x as libc::c_short;
                                    (*rects.offset(i as isize)).y = r.y as libc::c_short;
                                    (*rects.offset(i as isize))
                                        .width = r.width as libc::c_ushort;
                                    (*rects.offset(i as isize))
                                        .height = r.height as libc::c_ushort;
                                    i += 1;
                                }
                                XSetClipRectangles(
                                    (*display).display,
                                    gc,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    rects,
                                    i,
                                    3 as libc::c_int,
                                );
                                _cairo_xlib_shm_surface_mark_active((*surface).shm);
                                XCopyArea(
                                    (*display).display,
                                    (*shm).pixmap,
                                    (*surface).drawable,
                                    gc,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    (*shm).image.width as libc::c_uint,
                                    (*shm).image.height as libc::c_uint,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                );
                                if (*damage).status as libc::c_uint
                                    == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                                    && !((*damage).region).is_null()
                                {
                                    XSetClipMask(
                                        (*display).display,
                                        gc,
                                        0 as libc::c_long as Pixmap,
                                    );
                                }
                                current_block = 4567019141635105728;
                            }
                        }
                    }
                    match current_block {
                        210880132257114028 => {}
                        _ => {
                            _cairo_xlib_surface_put_gc(display, surface, gc);
                        }
                    }
                }
            }
        }
        _cairo_damage_destroy(damage);
        cairo_device_release(&mut (*display).base);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_create_shm(
    mut other: *mut cairo_xlib_surface_t,
    mut format: pixman_format_code_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    surface = 0 as *mut cairo_surface_t;
    if has_shm(other) != 0 {
        surface = &mut (*(_cairo_xlib_shm_surface_create
            as unsafe extern "C" fn(
                *mut cairo_xlib_surface_t,
                pixman_format_code_t,
                libc::c_int,
                libc::c_int,
                cairo_bool_t,
                libc::c_int,
            ) -> *mut cairo_xlib_shm_surface_t)(
            other,
            format,
            width,
            height,
            0 as libc::c_int,
            (has_shm_pixmaps
                as unsafe extern "C" fn(*mut cairo_xlib_surface_t) -> libc::c_int)(other),
        ))
            .image
            .base;
    }
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_create_shm__image(
    mut surface: *mut cairo_xlib_surface_t,
    mut format: pixman_format_code_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    if has_shm(surface) == 0 {
        return 0 as *mut cairo_surface_t;
    }
    return &mut (*(_cairo_xlib_shm_surface_create
        as unsafe extern "C" fn(
            *mut cairo_xlib_surface_t,
            pixman_format_code_t,
            libc::c_int,
            libc::c_int,
            cairo_bool_t,
            libc::c_int,
        ) -> *mut cairo_xlib_shm_surface_t)(
        surface,
        format,
        width,
        height,
        0 as libc::c_int,
        0 as libc::c_int,
    ))
        .image
        .base;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_surface_create_similar_shm(
    mut other: *mut libc::c_void,
    mut format: cairo_format_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    surface = _cairo_xlib_surface_create_shm(
        other as *mut cairo_xlib_surface_t,
        _cairo_format_to_pixman_format_code(format),
        width,
        height,
    );
    if !surface.is_null() {
        if (*surface).is_clear() == 0 {
            let mut shm: *mut cairo_xlib_shm_surface_t = surface
                as *mut cairo_xlib_shm_surface_t;
            if (*shm).active == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"shm->active == 0\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xlib-surface-shm.c\0" as *const u8
                        as *const libc::c_char,
                    1193 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 90],
                        &[libc::c_char; 90],
                    >(
                        b"cairo_surface_t *_cairo_xlib_surface_create_similar_shm(void *, cairo_format_t, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            memset(
                (*shm).image.data as *mut libc::c_void,
                0 as libc::c_int,
                ((*shm).image.stride * (*shm).image.height as libc::c_long)
                    as libc::c_ulong,
            );
            let ref mut fresh41 = (*shm).image.base;
            (*fresh41).set_is_clear(1 as libc::c_int as libc::c_uint);
        }
    } else {
        surface = cairo_image_surface_create(format, width, height);
    }
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_mark_active(
    mut _shm: *mut cairo_surface_t,
) {
    let mut shm: *mut cairo_xlib_shm_surface_t = _shm as *mut cairo_xlib_shm_surface_t;
    let mut display: *mut cairo_xlib_display_t = (*_shm).device
        as *mut cairo_xlib_display_t;
    (*shm).active = XNextRequest((*display).display);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_get_ximage(
    mut surface: *mut cairo_surface_t,
    mut ximage: *mut XImage,
) {
    let mut shm: *mut cairo_xlib_shm_surface_t = surface
        as *mut cairo_xlib_shm_surface_t;
    let mut native_byte_order: libc::c_int = if _cairo_is_little_endian() != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut image_masks: cairo_format_masks_t = cairo_format_masks_t {
        bpp: 0,
        alpha_mask: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
    };
    let mut ret: libc::c_int = 0;
    ret = _pixman_format_to_masks((*shm).image.pixman_format, &mut image_masks);
    if ret != 0 {} else {
        __assert_fail(
            b"ret\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
            1222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void _cairo_xlib_shm_surface_get_ximage(cairo_surface_t *, XImage *)\0"))
                .as_ptr(),
        );
    }
    (*ximage).width = (*shm).image.width;
    (*ximage).height = (*shm).image.height;
    (*ximage).format = 2 as libc::c_int;
    let ref mut fresh42 = (*ximage).data;
    *fresh42 = (*shm).image.data as *mut libc::c_char;
    let ref mut fresh43 = (*ximage).obdata;
    *fresh43 = &mut (*(*(*shm).info).pool).shm as *mut XShmSegmentInfo
        as *mut libc::c_char;
    (*ximage).byte_order = native_byte_order;
    (*ximage).bitmap_unit = 32 as libc::c_int;
    (*ximage).bitmap_bit_order = native_byte_order;
    (*ximage).bitmap_pad = 32 as libc::c_int;
    (*ximage).depth = (*shm).image.depth;
    (*ximage).bytes_per_line = (*shm).image.stride as libc::c_int;
    (*ximage).bits_per_pixel = image_masks.bpp;
    (*ximage).red_mask = image_masks.red_mask;
    (*ximage).green_mask = image_masks.green_mask;
    (*ximage).blue_mask = image_masks.blue_mask;
    (*ximage).xoffset = 0 as libc::c_int;
    ret = XInitImage(ximage);
    if ret != 0 as libc::c_int {} else {
        __assert_fail(
            b"ret != 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xlib-surface-shm.c\0" as *const u8 as *const libc::c_char,
            1242 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void _cairo_xlib_shm_surface_get_ximage(cairo_surface_t *, XImage *)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_get_obdata(
    mut surface: *mut cairo_surface_t,
) -> *mut libc::c_void {
    let mut display: *mut cairo_xlib_display_t = (*surface).device
        as *mut cairo_xlib_display_t;
    let mut shm: *mut cairo_xlib_shm_surface_t = surface
        as *mut cairo_xlib_shm_surface_t;
    let ref mut fresh44 = (*shm).active;
    *fresh44 = XNextRequest((*display).display);
    (*(*display).shm).last_event = *fresh44;
    return &mut (*(*(*shm).info).pool).shm as *mut XShmSegmentInfo as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_get_pixmap(
    mut surface: *mut cairo_surface_t,
) -> Pixmap {
    let mut shm: *mut cairo_xlib_shm_surface_t = 0 as *mut cairo_xlib_shm_surface_t;
    shm = surface as *mut cairo_xlib_shm_surface_t;
    return (*shm).pixmap;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_get_xrender_format(
    mut surface: *mut cairo_surface_t,
) -> *mut XRenderPictFormat {
    let mut shm: *mut cairo_xlib_shm_surface_t = 0 as *mut cairo_xlib_shm_surface_t;
    shm = surface as *mut cairo_xlib_shm_surface_t;
    if (*shm).image.format as libc::c_int != CAIRO_FORMAT_INVALID as libc::c_int {
        return _cairo_xlib_display_get_xrender_format(
            (*surface).device as *mut cairo_xlib_display_t,
            (*shm).image.format,
        );
    }
    return _cairo_xlib_display_get_xrender_format_for_pixman(
        (*surface).device as *mut cairo_xlib_display_t,
        (*shm).image.pixman_format,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_is_active(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    let mut shm: *mut cairo_xlib_shm_surface_t = 0 as *mut cairo_xlib_shm_surface_t;
    shm = surface as *mut cairo_xlib_shm_surface_t;
    if (*shm).active == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if seqno_passed((*shm).active, peek_processed((*shm).image.base.device)) != 0 {
        (*shm).active = 0 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_shm_surface_is_idle(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    let mut shm: *mut cairo_xlib_shm_surface_t = 0 as *mut cairo_xlib_shm_surface_t;
    shm = surface as *mut cairo_xlib_shm_surface_t;
    return ((*shm).idle > 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn has_broken_send_shm_event(
    mut display: *mut cairo_xlib_display_t,
    mut shm: *mut cairo_xlib_shm_display_t,
) -> cairo_bool_t {
    let mut dpy: *mut Display = (*display).display;
    let mut old_handler: Option::<
        unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
    > = None;
    let mut ev: XShmCompletionEvent = XShmCompletionEvent {
        type_0: 0,
        serial: 0,
        send_event: 0,
        display: 0 as *mut Display,
        drawable: 0,
        major_code: 0,
        minor_code: 0,
        shmseg: 0,
        offset: 0,
    };
    let mut info: XShmSegmentInfo = XShmSegmentInfo {
        shmseg: 0,
        shmid: 0,
        shmaddr: 0 as *mut libc::c_char,
        readOnly: 0,
    };
    info
        .shmid = shmget(
        0 as libc::c_int,
        0x1000 as libc::c_int as size_t,
        0o1000 as libc::c_int | 0o600 as libc::c_int,
    );
    if info.shmid == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    info.readOnly = 0 as libc::c_int;
    info
        .shmaddr = shmat(info.shmid, 0 as *const libc::c_void, 0 as libc::c_int)
        as *mut libc::c_char;
    if info.shmaddr == -(1 as libc::c_int) as *mut libc::c_char {
        shmctl(info.shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
        return 1 as libc::c_int;
    }
    ev.type_0 = (*shm).event;
    ev.send_event = 1 as libc::c_int;
    ev.serial = 1 as libc::c_int as libc::c_ulong;
    ev.drawable = (*shm).window;
    ev.major_code = (*shm).opcode;
    ev.minor_code = 3 as libc::c_int;
    ev.shmseg = info.shmid as ShmSeg;
    ev.offset = 0 as libc::c_int as libc::c_ulong;
    _x_error_occurred = 0 as libc::c_int;
    XLockDisplay(dpy);
    XSync(dpy, 0 as libc::c_int);
    old_handler = XSetErrorHandler(
        Some(
            _check_error_handler
                as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
        ),
    );
    XShmAttach(dpy, &mut info);
    XSendEvent(
        dpy,
        ev.drawable,
        0 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        &mut ev as *mut XShmCompletionEvent as *mut XEvent,
    );
    XShmDetach(dpy, &mut info);
    XSync(dpy, 0 as libc::c_int);
    XSetErrorHandler(old_handler);
    XUnlockDisplay(dpy);
    shmctl(info.shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
    shmdt(info.shmaddr as *const libc::c_void);
    return _x_error_occurred;
}
unsafe extern "C" fn xorg_has_buggy_send_shm_completion_event(
    mut display: *mut cairo_xlib_display_t,
    mut shm: *mut cairo_xlib_shm_display_t,
) -> cairo_bool_t {
    let mut dpy: *mut Display = (*display).display;
    if _cairo_xlib_vendor_is_xorg(dpy) != 0
        && (*(dpy as _XPrivDisplay)).release
            < 1 as libc::c_int * 10000000 as libc::c_int
                + 11 as libc::c_int * 100000 as libc::c_int
                + 0 as libc::c_int * 1000 as libc::c_int + 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return has_broken_send_shm_event(display, shm);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_init_shm(
    mut display: *mut cairo_xlib_display_t,
) {
    let mut shm: *mut cairo_xlib_shm_display_t = 0 as *mut cairo_xlib_shm_display_t;
    let mut attr: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut codes: *mut XExtCodes = 0 as *mut XExtCodes;
    let mut has_pixmap: libc::c_int = 0;
    let mut scr: libc::c_int = 0;
    let ref mut fresh45 = (*display).shm;
    *fresh45 = 0 as *mut cairo_xlib_shm_display_t;
    if can_use_shm((*display).display, &mut has_pixmap) == 0 {
        return;
    }
    shm = (if ::std::mem::size_of::<cairo_xlib_shm_display_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_shm_display_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_shm_display_t;
    if shm.is_null() {
        return;
    }
    codes = XInitExtension(
        (*display).display,
        b"MIT-SHM\0" as *const u8 as *const libc::c_char,
    );
    if codes.is_null() {
        free(shm as *mut libc::c_void);
        return;
    }
    (*shm).opcode = (*codes).major_opcode;
    (*shm).event = (*codes).first_event;
    if _pqueue_init(&mut (*shm).info) as u64 != 0 {
        free(shm as *mut libc::c_void);
        return;
    }
    scr = (*((*display).display as _XPrivDisplay)).default_screen;
    attr.override_redirect = 1 as libc::c_int;
    (*shm)
        .window = XCreateWindow(
        (*display).display,
        (*((*((*display).display as _XPrivDisplay)).screens)
            .offset((*((*display).display as _XPrivDisplay)).default_screen as isize))
            .root,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        (*((*((*display).display as _XPrivDisplay)).screens).offset(scr as isize))
            .root_depth,
        1 as libc::c_int as libc::c_uint,
        (*((*((*display).display as _XPrivDisplay)).screens).offset(scr as isize))
            .root_visual,
        ((1 as libc::c_long) << 9 as libc::c_int) as libc::c_ulong,
        &mut attr,
    );
    (*shm).last_event = 0 as libc::c_int as libc::c_ulong;
    (*shm).last_request = 0 as libc::c_int as libc::c_ulong;
    if xorg_has_buggy_send_shm_completion_event(display, shm) != 0 {
        has_pixmap = 0 as libc::c_int;
    }
    (*shm)
        .has_pixmaps = if has_pixmap != 0 {
        4096 as libc::c_int
    } else {
        0 as libc::c_int
    };
    cairo_list_init(&mut (*shm).pool);
    cairo_list_init(&mut (*shm).surfaces);
    let ref mut fresh46 = (*display).shm;
    *fresh46 = shm;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_display_fini_shm(
    mut display: *mut cairo_xlib_display_t,
) {
    let mut shm: *mut cairo_xlib_shm_display_t = (*display).shm;
    if shm.is_null() {
        return;
    }
    while cairo_list_is_empty(&mut (*shm).surfaces) == 0 {
        cairo_surface_finish(
            &mut (*({
                let mut mptr__: *const cairo_list_t = (*shm).surfaces.next;
                (mptr__ as *mut libc::c_char).offset(-(400 as libc::c_ulong as isize))
                    as *mut cairo_xlib_shm_surface_t
            }))
                .image
                .base,
        );
    }
    _pqueue_fini(&mut (*shm).info);
    while cairo_list_is_empty(&mut (*shm).pool) == 0 {
        let mut pool: *mut cairo_xlib_shm_t = 0 as *mut cairo_xlib_shm_t;
        pool = ({
            let mut mptr__: *const cairo_list_t = (*shm).pool.next;
            (mptr__ as *mut libc::c_char).offset(-(608 as libc::c_ulong as isize))
                as *mut cairo_xlib_shm_t
        });
        _cairo_xlib_display_shm_pool_destroy(display, pool);
    }
    if !((*display).display).is_null() {
        XDestroyWindow((*display).display, (*shm).window);
    }
    free(shm as *mut libc::c_void);
    let ref mut fresh47 = (*display).shm;
    *fresh47 = 0 as *mut cairo_xlib_shm_display_t;
}
