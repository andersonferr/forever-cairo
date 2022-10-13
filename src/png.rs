use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _cairo_pattern;
    pub type png_struct_def;
    pub type png_info_def;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_release_source_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
    );
    fn time(__timer: *mut time_t) -> time_t;
    fn _cairo_image_analyze_transparency(
        image: *mut cairo_image_surface_t,
    ) -> cairo_image_transparency_t;
    fn _cairo_image_surface_coerce(
        surface: *mut cairo_image_surface_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_surface_acquire_source_image(
        surface: *mut cairo_surface_t,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_fopen(
        filename: *const libc::c_char,
        mode: *const libc::c_char,
        file_out: *mut *mut FILE,
    ) -> cairo_status_t;
    fn cairo_surface_set_mime_data(
        surface: *mut cairo_surface_t,
        mime_type: *const libc::c_char,
        data: *const libc::c_uchar,
        length: libc::c_ulong,
        destroy: cairo_destroy_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn cairo_format_stride_for_width(
        format: cairo_format_t,
        width: libc::c_int,
    ) -> libc::c_int;
    fn cairo_image_surface_create_for_data(
        data: *mut libc::c_uchar,
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
        stride: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_image_surface_assume_ownership_of_data(
        surface: *mut cairo_image_surface_t,
    );
    fn _cairo_output_stream_destroy(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_write(
        stream: *mut cairo_output_stream_t,
        data: *const libc::c_void,
        length: size_t,
    );
    fn _cairo_memory_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_destroy(
        abstract_stream: *mut cairo_output_stream_t,
        data_out: *mut *mut libc::c_uchar,
        length_out: *mut libc::c_ulong,
    ) -> cairo_status_t;
    fn __errno_location() -> *mut libc::c_int;
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_create_write_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_set_longjmp_fn(
        png_ptr: png_structrp,
        longjmp_fn: png_longjmp_ptr,
        jmp_buf_size: size_t,
    ) -> *mut jmp_buf;
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    fn png_write_info(png_ptr: png_structrp, info_ptr: png_const_inforp);
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_convert_from_time_t(ptime: png_timep, ttime: time_t);
    fn png_set_expand_gray_1_2_4_to_8(png_ptr: png_structrp);
    fn png_set_palette_to_rgb(png_ptr: png_structrp);
    fn png_set_tRNS_to_alpha(png_ptr: png_structrp);
    fn png_set_gray_to_rgb(png_ptr: png_structrp);
    fn png_set_filler(png_ptr: png_structrp, filler: png_uint_32, flags: libc::c_int);
    fn png_set_swap(png_ptr: png_structrp);
    fn png_set_packing(png_ptr: png_structrp);
    fn png_set_packswap(png_ptr: png_structrp);
    fn png_set_interlace_handling(png_ptr: png_structrp) -> libc::c_int;
    fn png_read_update_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    fn png_write_image(png_ptr: png_structrp, image: png_bytepp);
    fn png_write_end(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_read_end(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    fn png_destroy_write_struct(png_ptr_ptr: png_structpp, info_ptr_ptr: png_infopp);
    fn png_get_error_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_set_write_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        write_data_fn: png_rw_ptr,
        output_flush_fn: png_flush_ptr,
    );
    fn png_set_read_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        read_data_fn: png_rw_ptr,
    );
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_set_read_user_transform_fn(
        png_ptr: png_structrp,
        read_user_transform_fn: png_user_transform_ptr,
    );
    fn png_set_write_user_transform_fn(
        png_ptr: png_structrp,
        write_user_transform_fn: png_user_transform_ptr,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_set_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: png_const_color_16p,
    );
    fn png_get_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        width: *mut png_uint_32,
        height: *mut png_uint_32,
        bit_depth: *mut libc::c_int,
        color_type: *mut libc::c_int,
        interlace_method: *mut libc::c_int,
        compression_method: *mut libc::c_int,
        filter_method: *mut libc::c_int,
    ) -> png_uint_32;
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: libc::c_int,
        color_type: libc::c_int,
        interlace_method: libc::c_int,
        compression_method: libc::c_int,
        filter_method: libc::c_int,
    );
    fn png_set_tIME(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        mod_time: png_const_timep,
    );
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type cairo_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
pub type cairo_read_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
pub type png_size_t = size_t;
pub type png_bytep = *mut png_byte;
pub type png_byte = libc::c_uchar;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_const_charp = *const libc::c_char;
pub type png_const_structrp = *const png_struct;
pub type png_voidp = *mut libc::c_void;
pub type png_rw_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_bytep, size_t) -> (),
>;
pub type png_info = png_info_def;
pub type png_infopp = *mut *mut png_info;
pub type png_structpp = *mut *mut png_struct;
pub type png_inforp = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_bytepp = *mut *mut png_byte;
pub type png_uint_32 = libc::c_uint;
pub type png_row_infop = *mut png_row_info;
pub type png_row_info = png_row_info_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_row_info_struct {
    pub width: png_uint_32,
    pub rowbytes: size_t,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
}
pub type uint8_t = __uint8_t;
pub type png_user_transform_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_row_infop, png_bytep) -> (),
>;
pub type png_const_inforp = *const png_info;
pub type png_const_timep = *const png_time;
pub type png_time = png_time_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_time_struct {
    pub year: png_uint_16,
    pub month: png_byte,
    pub day: png_byte,
    pub hour: png_byte,
    pub minute: png_byte,
    pub second: png_byte,
}
pub type png_uint_16 = libc::c_ushort;
pub type png_timep = *mut png_time;
pub type png_const_color_16p = *const png_color_16;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub const CAIRO_IMAGE_IS_OPAQUE: _cairo_image_transparency = 0;
pub type cairo_image_transparency_t = _cairo_image_transparency;
pub type _cairo_image_transparency = libc::c_uint;
pub const CAIRO_IMAGE_UNKNOWN: _cairo_image_transparency = 3;
pub const CAIRO_IMAGE_HAS_ALPHA: _cairo_image_transparency = 2;
pub const CAIRO_IMAGE_HAS_BILEVEL_ALPHA: _cairo_image_transparency = 1;
pub type png_flush_ptr = Option::<unsafe extern "C" fn(png_structp) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type png_longjmp_ptr = Option::<
    unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> (),
>;
pub type png_infop = *mut png_info;
pub type png_error_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_const_charp) -> (),
>;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_write_closure_t {
    pub write_func: cairo_write_func_t,
    pub closure: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_read_closure_t {
    pub read_func: cairo_read_func_t,
    pub closure: *mut libc::c_void,
    pub png_data: *mut cairo_output_stream_t,
}
pub type cairo_output_stream_t = _cairo_output_stream;
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
unsafe extern "C" fn unpremultiply_data(
    mut png: png_structp,
    mut row_info: png_row_infop,
    mut data: png_bytep,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*row_info).rowbytes {
        let mut b: *mut uint8_t = &mut *data.offset(i as isize) as *mut png_byte;
        let mut pixel: uint32_t = 0;
        let mut alpha: uint8_t = 0;
        memcpy(
            &mut pixel as *mut uint32_t as *mut libc::c_void,
            b as *const libc::c_void,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        alpha = ((pixel & 0xff000000 as libc::c_uint) >> 24 as libc::c_int) as uint8_t;
        if alpha as libc::c_int == 0 as libc::c_int {
            let ref mut fresh2 = *b.offset(3 as libc::c_int as isize);
            *fresh2 = 0 as libc::c_int as uint8_t;
            let ref mut fresh3 = *b.offset(2 as libc::c_int as isize);
            *fresh3 = *fresh2;
            let ref mut fresh4 = *b.offset(1 as libc::c_int as isize);
            *fresh4 = *fresh3;
            *b.offset(0 as libc::c_int as isize) = *fresh4;
        } else {
            *b
                .offset(
                    0 as libc::c_int as isize,
                ) = ((pixel & 0xff0000 as libc::c_int as libc::c_uint)
                >> 16 as libc::c_int)
                .wrapping_mul(255 as libc::c_int as libc::c_uint)
                .wrapping_add((alpha as libc::c_int / 2 as libc::c_int) as libc::c_uint)
                .wrapping_div(alpha as libc::c_uint) as uint8_t;
            *b
                .offset(
                    1 as libc::c_int as isize,
                ) = ((pixel & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
                .wrapping_mul(255 as libc::c_int as libc::c_uint)
                .wrapping_add((alpha as libc::c_int / 2 as libc::c_int) as libc::c_uint)
                .wrapping_div(alpha as libc::c_uint) as uint8_t;
            *b
                .offset(
                    2 as libc::c_int as isize,
                ) = ((pixel & 0xff as libc::c_int as libc::c_uint) >> 0 as libc::c_int)
                .wrapping_mul(255 as libc::c_int as libc::c_uint)
                .wrapping_add((alpha as libc::c_int / 2 as libc::c_int) as libc::c_uint)
                .wrapping_div(alpha as libc::c_uint) as uint8_t;
            *b.offset(3 as libc::c_int as isize) = alpha;
        }
        i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn f_to_u16(mut val: libc::c_float) -> uint16_t {
    if val < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as uint16_t
    } else if val > 1 as libc::c_int as libc::c_float {
        return 65535 as libc::c_int as uint16_t
    } else {
        return (val * 65535.0f32) as uint16_t
    };
}
unsafe extern "C" fn unpremultiply_float(
    mut f: *mut libc::c_float,
    mut d16: *mut uint16_t,
    mut width: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < width {
        let mut r: libc::c_float = 0.;
        let mut g: libc::c_float = 0.;
        let mut b: libc::c_float = 0.;
        let mut a: libc::c_float = 0.;
        let fresh5 = f;
        f = f.offset(1);
        r = *fresh5;
        let fresh6 = f;
        f = f.offset(1);
        g = *fresh6;
        let fresh7 = f;
        f = f.offset(1);
        b = *fresh7;
        let fresh8 = f;
        f = f.offset(1);
        a = *fresh8;
        if a > 0 as libc::c_int as libc::c_float {
            let fresh9 = d16;
            d16 = d16.offset(1);
            *fresh9 = f_to_u16(r / a);
            let fresh10 = d16;
            d16 = d16.offset(1);
            *fresh10 = f_to_u16(g / a);
            let fresh11 = d16;
            d16 = d16.offset(1);
            *fresh11 = f_to_u16(b / a);
            let fresh12 = d16;
            d16 = d16.offset(1);
            *fresh12 = f_to_u16(a);
        } else {
            let fresh13 = d16;
            d16 = d16.offset(1);
            *fresh13 = 0 as libc::c_int as uint16_t;
            let fresh14 = d16;
            d16 = d16.offset(1);
            *fresh14 = 0 as libc::c_int as uint16_t;
            let fresh15 = d16;
            d16 = d16.offset(1);
            *fresh15 = 0 as libc::c_int as uint16_t;
            let fresh16 = d16;
            d16 = d16.offset(1);
            *fresh16 = 0 as libc::c_int as uint16_t;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn premultiply_float(
    mut f: *mut libc::c_float,
    mut d16: *const uint16_t,
    mut width: libc::c_uint,
) {
    let mut i: libc::c_uint = width;
    loop {
        let fresh17 = i;
        i = i.wrapping_sub(1);
        if !(fresh17 != 0) {
            break;
        }
        let mut a: libc::c_float = *d16
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) as libc::c_int as libc::c_float / 65535.0f32;
        *f
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) = a;
        *f
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) = *d16
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) as libc::c_float / 65535.0f32 * a;
        *f
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = *d16
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) as libc::c_float / 65535.0f32 * a;
        *f
            .offset(
                i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize,
            ) = *d16.offset(i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize)
            as libc::c_float / 65535.0f32 * a;
    };
}
unsafe extern "C" fn convert_u16_to_float(
    mut f: *mut libc::c_float,
    mut d16: *const uint16_t,
    mut width: libc::c_uint,
) {
    let mut i: libc::c_uint = width;
    loop {
        let fresh18 = i;
        i = i.wrapping_sub(1);
        if !(fresh18 != 0) {
            break;
        }
        *f
            .offset(
                i
                    .wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) = *d16
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) as libc::c_float / 65535.0f32;
        *f
            .offset(
                i
                    .wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = *d16
            .offset(
                i
                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) as libc::c_float / 65535.0f32;
        *f
            .offset(
                i.wrapping_mul(3 as libc::c_int as libc::c_uint) as isize,
            ) = *d16.offset(i.wrapping_mul(4 as libc::c_int as libc::c_uint) as isize)
            as libc::c_float / 65535.0f32;
    };
}
unsafe extern "C" fn convert_float_to_u16(
    mut f: *mut libc::c_float,
    mut d16: *mut uint16_t,
    mut width: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < width {
        let fresh19 = f;
        f = f.offset(1);
        let fresh20 = d16;
        d16 = d16.offset(1);
        *fresh20 = f_to_u16(*fresh19);
        let fresh21 = f;
        f = f.offset(1);
        let fresh22 = d16;
        d16 = d16.offset(1);
        *fresh22 = f_to_u16(*fresh21);
        let fresh23 = f;
        f = f.offset(1);
        let fresh24 = d16;
        d16 = d16.offset(1);
        *fresh24 = f_to_u16(*fresh23);
        let fresh25 = d16;
        d16 = d16.offset(1);
        *fresh25 = 0 as libc::c_int as uint16_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn convert_data_to_bytes(
    mut png: png_structp,
    mut row_info: png_row_infop,
    mut data: png_bytep,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*row_info).rowbytes {
        let mut b: *mut uint8_t = &mut *data.offset(i as isize) as *mut png_byte;
        let mut pixel: uint32_t = 0;
        memcpy(
            &mut pixel as *mut uint32_t as *mut libc::c_void,
            b as *const libc::c_void,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        *b
            .offset(
                0 as libc::c_int as isize,
            ) = ((pixel & 0xff0000 as libc::c_int as libc::c_uint) >> 16 as libc::c_int)
            as uint8_t;
        *b
            .offset(
                1 as libc::c_int as isize,
            ) = ((pixel & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
            as uint8_t;
        *b
            .offset(
                2 as libc::c_int as isize,
            ) = ((pixel & 0xff as libc::c_int as libc::c_uint) >> 0 as libc::c_int)
            as uint8_t;
        *b.offset(3 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
        i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn png_simple_error_callback(
    mut png: png_structp,
    mut error_msg: png_const_charp,
) {
    let mut error: *mut cairo_status_t = png_get_error_ptr(png as *const png_struct)
        as *mut cairo_status_t;
    if *error as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        *error = _cairo_error(CAIRO_STATUS_PNG_ERROR);
    }
    longjmp(
        (*png_set_longjmp_fn(
            png,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !>,
                png_longjmp_ptr,
            >(
                Some(
                    longjmp as unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                ),
            ),
            ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
        ))
            .as_mut_ptr(),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn png_simple_warning_callback(
    mut png: png_structp,
    mut error_msg: png_const_charp,
) {}
unsafe extern "C" fn png_simple_output_flush_fn(mut png_ptr: png_structp) {}
unsafe extern "C" fn write_png(
    mut surface: *mut cairo_surface_t,
    mut write_func: png_rw_ptr,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut clone: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut png: *mut png_struct = 0 as *mut png_struct;
    let mut info: *mut png_info = 0 as *mut png_info;
    let mut rows: *mut *mut png_byte = 0 as *mut *mut png_byte;
    let mut white: png_color_16 = png_color_16 {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    };
    let mut png_color_type: libc::c_int = 0;
    let mut bpc: libc::c_int = 0;
    let mut u16_copy: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    status = _cairo_surface_acquire_source_image(surface, &mut image, &mut image_extra)
        as cairo_int_status_t;
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH)
    } else {
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
    }
    if (*image).width == 0 as libc::c_int || (*image).height == 0 as libc::c_int {
        status = _cairo_error(CAIRO_STATUS_WRITE_ERROR) as cairo_int_status_t;
    } else {
        if (*image).format as libc::c_int == CAIRO_FORMAT_RGB96F as libc::c_int
            || (*image).format as libc::c_int == CAIRO_FORMAT_RGBA128F as libc::c_int
        {
            ::std::ptr::write_volatile(
                &mut u16_copy as *mut *mut libc::c_uchar,
                _cairo_malloc_ab(
                    ((*image).width * 8 as libc::c_int) as size_t,
                    (*image).height as size_t,
                ) as *mut libc::c_uchar,
            );
            if u16_copy.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                current_block = 16465730530612695416;
            } else {
                ::std::ptr::write_volatile(
                    &mut clone as *mut *mut cairo_image_surface_t,
                    cairo_surface_reference(&mut (*image).base)
                        as *mut cairo_image_surface_t,
                );
                current_block = 2668756484064249700;
            }
        } else {
            ::std::ptr::write_volatile(
                &mut clone as *mut *mut cairo_image_surface_t,
                _cairo_image_surface_coerce(image),
            );
            status = (*clone).base.status as cairo_int_status_t;
            current_block = 2668756484064249700;
        }
        match current_block {
            16465730530612695416 => {}
            _ => {
                if !(status as u64 != 0) {
                    ::std::ptr::write_volatile(
                        &mut rows as *mut *mut *mut png_byte,
                        _cairo_malloc_ab(
                            (*clone).height as size_t,
                            ::std::mem::size_of::<*mut png_byte>() as libc::c_ulong,
                        ) as *mut *mut png_byte,
                    );
                    if rows.is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                            as cairo_int_status_t;
                    } else {
                        if u16_copy.is_null() {
                            i = 0 as libc::c_int;
                            while i < (*clone).height {
                                let ref mut fresh26 = *rows.offset(i as isize);
                                *fresh26 = ((*clone).data as *mut png_byte)
                                    .offset((i as libc::c_long * (*clone).stride) as isize);
                                i += 1;
                            }
                        } else {
                            i = 0 as libc::c_int;
                            while i < (*clone).height {
                                let mut float_line: *mut libc::c_float = &mut *((*clone)
                                    .data)
                                    .offset((i as libc::c_long * (*clone).stride) as isize)
                                    as *mut libc::c_uchar as *mut libc::c_float;
                                let mut u16_line: *mut uint16_t = &mut *u16_copy
                                    .offset((i * (*clone).width * 8 as libc::c_int) as isize)
                                    as *mut libc::c_uchar as *mut uint16_t;
                                if (*image).format as libc::c_int
                                    == CAIRO_FORMAT_RGBA128F as libc::c_int
                                {
                                    unpremultiply_float(
                                        float_line,
                                        u16_line,
                                        (*clone).width as libc::c_uint,
                                    );
                                } else {
                                    convert_float_to_u16(
                                        float_line,
                                        u16_line,
                                        (*clone).width as libc::c_uint,
                                    );
                                }
                                let ref mut fresh27 = *rows.offset(i as isize);
                                *fresh27 = u16_line as *mut png_byte;
                                i += 1;
                            }
                        }
                        png = png_create_write_struct(
                            b"1.6.37\0" as *const u8 as *const libc::c_char,
                            &mut status as *mut cairo_int_status_t as png_voidp,
                            Some(
                                png_simple_error_callback
                                    as unsafe extern "C" fn(png_structp, png_const_charp) -> (),
                            ),
                            Some(
                                png_simple_warning_callback
                                    as unsafe extern "C" fn(png_structp, png_const_charp) -> (),
                            ),
                        );
                        if png.is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                as cairo_int_status_t;
                        } else {
                            info = png_create_info_struct(png);
                            if info.is_null() {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                    as cairo_int_status_t;
                            } else if !(_setjmp(
                                (*png_set_longjmp_fn(
                                    png,
                                    ::std::mem::transmute::<
                                        Option::<
                                            unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                                        >,
                                        png_longjmp_ptr,
                                    >(
                                        Some(
                                            longjmp
                                                as unsafe extern "C" fn(
                                                    *mut __jmp_buf_tag,
                                                    libc::c_int,
                                                ) -> !,
                                        ),
                                    ),
                                    ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
                                ))
                                    .as_mut_ptr(),
                            ) != 0)
                            {
                                png_set_write_fn(
                                    png,
                                    closure,
                                    write_func,
                                    Some(
                                        png_simple_output_flush_fn
                                            as unsafe extern "C" fn(png_structp) -> (),
                                    ),
                                );
                                match (*clone).format as libc::c_int {
                                    0 => {
                                        bpc = 8 as libc::c_int;
                                        if _cairo_image_analyze_transparency(clone) as libc::c_uint
                                            == CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
                                        {
                                            png_color_type = 2 as libc::c_int;
                                        } else {
                                            png_color_type = 2 as libc::c_int | 4 as libc::c_int;
                                        }
                                        current_block = 12961834331865314435;
                                    }
                                    5 => {
                                        bpc = 10 as libc::c_int;
                                        png_color_type = 2 as libc::c_int;
                                        current_block = 12961834331865314435;
                                    }
                                    1 => {
                                        bpc = 8 as libc::c_int;
                                        png_color_type = 2 as libc::c_int;
                                        current_block = 12961834331865314435;
                                    }
                                    2 => {
                                        bpc = 8 as libc::c_int;
                                        png_color_type = 0 as libc::c_int;
                                        current_block = 12961834331865314435;
                                    }
                                    3 => {
                                        bpc = 1 as libc::c_int;
                                        png_color_type = 0 as libc::c_int;
                                        png_set_packswap(png);
                                        current_block = 12961834331865314435;
                                    }
                                    6 => {
                                        bpc = 16 as libc::c_int;
                                        png_color_type = 2 as libc::c_int;
                                        current_block = 12961834331865314435;
                                    }
                                    7 => {
                                        bpc = 16 as libc::c_int;
                                        png_color_type = 2 as libc::c_int | 4 as libc::c_int;
                                        current_block = 12961834331865314435;
                                    }
                                    -1 | 4 | _ => {
                                        status = _cairo_error(CAIRO_STATUS_INVALID_FORMAT)
                                            as cairo_int_status_t;
                                        current_block = 14562744850471458065;
                                    }
                                }
                                match current_block {
                                    14562744850471458065 => {}
                                    _ => {
                                        png_set_IHDR(
                                            png,
                                            info,
                                            (*clone).width as png_uint_32,
                                            (*clone).height as png_uint_32,
                                            bpc,
                                            png_color_type,
                                            0 as libc::c_int,
                                            0 as libc::c_int,
                                            0 as libc::c_int,
                                        );
                                        white
                                            .gray = (((1 as libc::c_int) << bpc) - 1 as libc::c_int)
                                            as png_uint_16;
                                        white.green = white.gray;
                                        white.blue = white.green;
                                        white.red = white.blue;
                                        png_set_bKGD(
                                            png,
                                            info,
                                            &mut white as *mut png_color_16 as png_const_color_16p,
                                        );
                                        png_write_info(png, info);
                                        png_set_swap(png);
                                        if png_color_type == 2 as libc::c_int | 4 as libc::c_int {
                                            if (*clone).format as libc::c_int
                                                != CAIRO_FORMAT_RGBA128F as libc::c_int
                                            {
                                                png_set_write_user_transform_fn(
                                                    png,
                                                    Some(
                                                        unpremultiply_data
                                                            as unsafe extern "C" fn(
                                                                png_structp,
                                                                png_row_infop,
                                                                png_bytep,
                                                            ) -> (),
                                                    ),
                                                );
                                            }
                                        } else if png_color_type == 2 as libc::c_int {
                                            if (*clone).format as libc::c_int
                                                != CAIRO_FORMAT_RGB96F as libc::c_int
                                            {
                                                png_set_write_user_transform_fn(
                                                    png,
                                                    Some(
                                                        convert_data_to_bytes
                                                            as unsafe extern "C" fn(
                                                                png_structp,
                                                                png_row_infop,
                                                                png_bytep,
                                                            ) -> (),
                                                    ),
                                                );
                                            }
                                            png_set_filler(
                                                png,
                                                0 as libc::c_int as png_uint_32,
                                                1 as libc::c_int,
                                            );
                                        }
                                        png_write_image(png, rows);
                                        png_write_end(png, info);
                                    }
                                }
                            }
                            png_destroy_write_struct(&mut png, &mut info);
                        }
                        free(rows as *mut libc::c_void);
                    }
                    cairo_surface_destroy(&mut (*clone).base);
                    free(u16_copy as *mut libc::c_void);
                }
            }
        }
    }
    _cairo_surface_release_source_image(surface, image, image_extra);
    return status as cairo_status_t;
}
unsafe extern "C" fn stdio_write_func(
    mut png: png_structp,
    mut data: png_bytep,
    mut size: png_size_t,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = png_get_io_ptr(png as *const png_struct) as *mut FILE;
    while size != 0 {
        let mut ret: size_t = fwrite(
            data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            size,
            fp,
        );
        size = (size as libc::c_ulong).wrapping_sub(ret) as png_size_t as png_size_t;
        data = data.offset(ret as isize);
        if size != 0 && ferror(fp) != 0 {
            let mut error: *mut cairo_status_t = png_get_error_ptr(
                png as *const png_struct,
            ) as *mut cairo_status_t;
            if *error as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                *error = _cairo_error(CAIRO_STATUS_WRITE_ERROR);
            }
            png_error(png as *const png_struct, 0 as png_const_charp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_write_to_png(
    mut surface: *mut cairo_surface_t,
    mut filename: *const libc::c_char,
) -> cairo_status_t {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    status = _cairo_fopen(
        filename,
        b"wb\0" as *const u8 as *const libc::c_char,
        &mut fp,
    );
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return _cairo_error(status);
    }
    if fp.is_null() {
        match *__errno_location() {
            12 => return _cairo_error(CAIRO_STATUS_NO_MEMORY),
            _ => return _cairo_error(CAIRO_STATUS_WRITE_ERROR),
        }
    }
    status = write_png(
        surface,
        Some(
            stdio_write_func
                as unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> (),
        ),
        fp as *mut libc::c_void,
    );
    if fclose(fp) != 0
        && status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = _cairo_error(CAIRO_STATUS_WRITE_ERROR);
    }
    return status;
}
unsafe extern "C" fn stream_write_func(
    mut png: png_structp,
    mut data: png_bytep,
    mut size: png_size_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut png_closure: *mut png_write_closure_t = 0 as *mut png_write_closure_t;
    png_closure = png_get_io_ptr(png as *const png_struct) as *mut png_write_closure_t;
    status = ((*png_closure).write_func)
        .expect(
            "non-null function pointer",
        )((*png_closure).closure, data as *const libc::c_uchar, size as libc::c_uint);
    if status as u64 != 0 {
        let mut error: *mut cairo_status_t = png_get_error_ptr(png as *const png_struct)
            as *mut cairo_status_t;
        if *error as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            *error = status;
        }
        png_error(png as *const png_struct, 0 as png_const_charp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_surface_write_to_png_stream(
    mut surface: *mut cairo_surface_t,
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut png_closure: png_write_closure_t = png_write_closure_t {
        write_func: None,
        closure: 0 as *mut libc::c_void,
    };
    if (*surface).status as u64 != 0 {
        return (*surface).status;
    }
    if (*surface).finished() != 0 {
        return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
    }
    png_closure.write_func = write_func;
    png_closure.closure = closure;
    return write_png(
        surface,
        Some(
            stream_write_func
                as unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> (),
        ),
        &mut png_closure as *mut png_write_closure_t as *mut libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn multiply_alpha(
    mut alpha: libc::c_int,
    mut color: libc::c_int,
) -> libc::c_int {
    let mut temp: libc::c_int = alpha * color + 0x80 as libc::c_int;
    return temp + (temp >> 8 as libc::c_int) >> 8 as libc::c_int;
}
unsafe extern "C" fn premultiply_data(
    mut png: png_structp,
    mut row_info: png_row_infop,
    mut data: png_bytep,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*row_info).rowbytes {
        let mut base: *mut uint8_t = &mut *data.offset(i as isize) as *mut png_byte;
        let mut alpha: uint8_t = *base.offset(3 as libc::c_int as isize);
        let mut p: uint32_t = 0;
        if alpha as libc::c_int == 0 as libc::c_int {
            p = 0 as libc::c_int as uint32_t;
        } else {
            let mut red: uint8_t = *base.offset(0 as libc::c_int as isize);
            let mut green: uint8_t = *base.offset(1 as libc::c_int as isize);
            let mut blue: uint8_t = *base.offset(2 as libc::c_int as isize);
            if alpha as libc::c_int != 0xff as libc::c_int {
                red = multiply_alpha(alpha as libc::c_int, red as libc::c_int)
                    as uint8_t;
                green = multiply_alpha(alpha as libc::c_int, green as libc::c_int)
                    as uint8_t;
                blue = multiply_alpha(alpha as libc::c_int, blue as libc::c_int)
                    as uint8_t;
            }
            p = (alpha as uint32_t) << 24 as libc::c_int
                | ((red as libc::c_int) << 16 as libc::c_int) as libc::c_uint
                | ((green as libc::c_int) << 8 as libc::c_int) as libc::c_uint
                | ((blue as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        }
        memcpy(
            base as *mut libc::c_void,
            &mut p as *mut uint32_t as *const libc::c_void,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn convert_bytes_to_data(
    mut png: png_structp,
    mut row_info: png_row_infop,
    mut data: png_bytep,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*row_info).rowbytes {
        let mut base: *mut uint8_t = &mut *data.offset(i as isize) as *mut png_byte;
        let mut red: uint8_t = *base.offset(0 as libc::c_int as isize);
        let mut green: uint8_t = *base.offset(1 as libc::c_int as isize);
        let mut blue: uint8_t = *base.offset(2 as libc::c_int as isize);
        let mut pixel: uint32_t = 0;
        pixel = (0xff as libc::c_uint) << 24 as libc::c_int
            | ((red as libc::c_int) << 16 as libc::c_int) as libc::c_uint
            | ((green as libc::c_int) << 8 as libc::c_int) as libc::c_uint
            | ((blue as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        memcpy(
            base as *mut libc::c_void,
            &mut pixel as *mut uint32_t as *const libc::c_void,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn stdio_read_func(
    mut closure: *mut libc::c_void,
    mut data: *mut libc::c_uchar,
    mut size: libc::c_uint,
) -> cairo_status_t {
    let mut file: *mut FILE = closure as *mut FILE;
    while size != 0 {
        let mut ret: size_t = 0;
        ret = fread(
            data as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            size as libc::c_ulong,
            file,
        );
        size = (size as libc::c_ulong).wrapping_sub(ret) as libc::c_uint as libc::c_uint;
        data = data.offset(ret as isize);
        if size != 0 && (feof(file) != 0 || ferror(file) != 0) {
            return _cairo_error(CAIRO_STATUS_READ_ERROR);
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn stream_read_func(
    mut png: png_structp,
    mut data: png_bytep,
    mut size: png_size_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut png_closure: *mut png_read_closure_t = 0 as *mut png_read_closure_t;
    png_closure = png_get_io_ptr(png as *const png_struct) as *mut png_read_closure_t;
    status = ((*png_closure).read_func)
        .expect(
            "non-null function pointer",
        )((*png_closure).closure, data, size as libc::c_uint);
    if status as u64 != 0 {
        let mut error: *mut cairo_status_t = png_get_error_ptr(png as *const png_struct)
            as *mut cairo_status_t;
        if *error as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            *error = status;
        }
        png_error(png as *const png_struct, 0 as png_const_charp);
    }
    _cairo_output_stream_write(
        (*png_closure).png_data,
        data as *const libc::c_void,
        size,
    );
}
unsafe extern "C" fn read_png(
    mut png_closure: *mut png_read_closure_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut png: *mut png_struct = 0 as *mut png_struct;
    let mut info: *mut png_info = 0 as *mut png_info;
    let mut data: *mut png_byte = 0 as *mut png_byte;
    let mut row_pointers: *mut *mut png_byte = 0 as *mut *mut png_byte;
    let mut png_width: png_uint_32 = 0;
    let mut png_height: png_uint_32 = 0;
    let mut depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut interlace: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut mime_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let ref mut fresh28 = (*png_closure).png_data;
    *fresh28 = _cairo_memory_stream_create();
    png = png_create_read_struct(
        b"1.6.37\0" as *const u8 as *const libc::c_char,
        &mut status as *mut cairo_status_t as png_voidp,
        Some(
            png_simple_error_callback
                as unsafe extern "C" fn(png_structp, png_const_charp) -> (),
        ),
        Some(
            png_simple_warning_callback
                as unsafe extern "C" fn(png_structp, png_const_charp) -> (),
        ),
    );
    if png.is_null() {
        ::std::ptr::write_volatile(
            &mut surface as *mut *mut cairo_surface_t,
            _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY)),
        );
    } else {
        info = png_create_info_struct(png);
        if info.is_null() {
            ::std::ptr::write_volatile(
                &mut surface as *mut *mut cairo_surface_t,
                _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY)),
            );
        } else {
            png_set_read_fn(
                png,
                png_closure as png_voidp,
                Some(
                    stream_read_func
                        as unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> (),
                ),
            );
            status = CAIRO_STATUS_SUCCESS;
            if _setjmp(
                (*png_set_longjmp_fn(
                    png,
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                        >,
                        png_longjmp_ptr,
                    >(
                        Some(
                            longjmp
                                as unsafe extern "C" fn(
                                    *mut __jmp_buf_tag,
                                    libc::c_int,
                                ) -> !,
                        ),
                    ),
                    ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
                ))
                    .as_mut_ptr(),
            ) != 0
            {
                ::std::ptr::write_volatile(
                    &mut surface as *mut *mut cairo_surface_t,
                    _cairo_surface_create_in_error(status),
                );
            } else {
                png_read_info(png, info);
                png_set_swap(png);
                png_get_IHDR(
                    png,
                    info,
                    &mut png_width,
                    &mut png_height,
                    &mut depth,
                    &mut color_type,
                    &mut interlace,
                    0 as *mut libc::c_int,
                    0 as *mut libc::c_int,
                );
                if status as u64 != 0 {
                    ::std::ptr::write_volatile(
                        &mut surface as *mut *mut cairo_surface_t,
                        _cairo_surface_create_in_error(status),
                    );
                } else {
                    if color_type == 2 as libc::c_int | 1 as libc::c_int {
                        png_set_palette_to_rgb(png);
                    }
                    if color_type == 0 as libc::c_int {
                        png_set_expand_gray_1_2_4_to_8(png);
                    }
                    if png_get_valid(png, info, 0x10 as libc::c_uint) != 0 {
                        png_set_tRNS_to_alpha(png);
                    }
                    if depth < 8 as libc::c_int {
                        png_set_packing(png);
                    }
                    if color_type == 0 as libc::c_int || color_type == 4 as libc::c_int {
                        png_set_gray_to_rgb(png);
                    }
                    if interlace != 0 as libc::c_int {
                        png_set_interlace_handling(png);
                    }
                    png_set_filler(
                        png,
                        0xff as libc::c_int as png_uint_32,
                        1 as libc::c_int,
                    );
                    png_read_update_info(png, info);
                    png_get_IHDR(
                        png,
                        info,
                        &mut png_width,
                        &mut png_height,
                        &mut depth,
                        &mut color_type,
                        &mut interlace,
                        0 as *mut libc::c_int,
                        0 as *mut libc::c_int,
                    );
                    if depth != 8 as libc::c_int && depth != 16 as libc::c_int
                        || !(color_type == 2 as libc::c_int
                            || color_type == 2 as libc::c_int | 4 as libc::c_int)
                    {
                        ::std::ptr::write_volatile(
                            &mut surface as *mut *mut cairo_surface_t,
                            _cairo_surface_create_in_error(
                                _cairo_error(CAIRO_STATUS_READ_ERROR),
                            ),
                        );
                    } else {
                        let mut current_block_44: u64;
                        match color_type {
                            6 => {
                                current_block_44 = 11290756061803968246;
                            }
                            2 => {
                                if depth == 8 as libc::c_int {
                                    format = CAIRO_FORMAT_RGB24;
                                    png_set_read_user_transform_fn(
                                        png,
                                        Some(
                                            convert_bytes_to_data
                                                as unsafe extern "C" fn(
                                                    png_structp,
                                                    png_row_infop,
                                                    png_bytep,
                                                ) -> (),
                                        ),
                                    );
                                } else {
                                    format = CAIRO_FORMAT_RGB96F;
                                }
                                current_block_44 = 9859671972921157070;
                            }
                            _ => {
                                if (b"reached\0" as *const u8 as *const libc::c_char)
                                    .is_null()
                                {} else {
                                    __assert_fail(
                                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                                        b"../src/cairo-png.c\0" as *const u8 as *const libc::c_char,
                                        776 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 55],
                                            &[libc::c_char; 55],
                                        >(
                                            b"cairo_surface_t *read_png(struct png_read_closure_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                current_block_44 = 11290756061803968246;
                            }
                        }
                        match current_block_44 {
                            11290756061803968246 => {
                                if depth == 8 as libc::c_int {
                                    format = CAIRO_FORMAT_ARGB32;
                                    png_set_read_user_transform_fn(
                                        png,
                                        Some(
                                            premultiply_data
                                                as unsafe extern "C" fn(
                                                    png_structp,
                                                    png_row_infop,
                                                    png_bytep,
                                                ) -> (),
                                        ),
                                    );
                                } else {
                                    format = CAIRO_FORMAT_RGBA128F;
                                }
                            }
                            _ => {}
                        }
                        stride = cairo_format_stride_for_width(
                            format,
                            png_width as libc::c_int,
                        );
                        if stride < 0 as libc::c_int {
                            ::std::ptr::write_volatile(
                                &mut surface as *mut *mut cairo_surface_t,
                                _cairo_surface_create_in_error(
                                    _cairo_error(CAIRO_STATUS_INVALID_STRIDE),
                                ),
                            );
                        } else {
                            ::std::ptr::write_volatile(
                                &mut data as *mut *mut png_byte,
                                _cairo_malloc_ab(png_height as size_t, stride as size_t)
                                    as *mut png_byte,
                            );
                            if data.is_null() {
                                ::std::ptr::write_volatile(
                                    &mut surface as *mut *mut cairo_surface_t,
                                    _cairo_surface_create_in_error(
                                        _cairo_error(CAIRO_STATUS_NO_MEMORY),
                                    ),
                                );
                            } else {
                                ::std::ptr::write_volatile(
                                    &mut row_pointers as *mut *mut *mut png_byte,
                                    _cairo_malloc_ab(
                                        png_height as size_t,
                                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                    ) as *mut *mut png_byte,
                                );
                                if row_pointers.is_null() {
                                    ::std::ptr::write_volatile(
                                        &mut surface as *mut *mut cairo_surface_t,
                                        _cairo_surface_create_in_error(
                                            _cairo_error(CAIRO_STATUS_NO_MEMORY),
                                        ),
                                    );
                                } else {
                                    i = 0 as libc::c_int as libc::c_uint;
                                    while i < png_height {
                                        let ref mut fresh29 = *row_pointers.offset(i as isize);
                                        *fresh29 = &mut *data
                                            .offset((i as libc::c_long * stride as ptrdiff_t) as isize)
                                            as *mut png_byte;
                                        i = i.wrapping_add(1);
                                    }
                                    png_read_image(png, row_pointers);
                                    png_read_end(png, info);
                                    if status as u64 != 0 {
                                        ::std::ptr::write_volatile(
                                            &mut surface as *mut *mut cairo_surface_t,
                                            _cairo_surface_create_in_error(status),
                                        );
                                    } else {
                                        if format as libc::c_int
                                            == CAIRO_FORMAT_RGBA128F as libc::c_int
                                        {
                                            i = png_height;
                                            loop {
                                                let fresh30 = i;
                                                i = i.wrapping_sub(1);
                                                if !(fresh30 != 0) {
                                                    break;
                                                }
                                                let mut float_line: *mut libc::c_float = *row_pointers
                                                    .offset(i as isize) as *mut libc::c_float;
                                                let mut u16_line: *mut uint16_t = *row_pointers
                                                    .offset(i as isize) as *mut uint16_t;
                                                premultiply_float(float_line, u16_line, png_width);
                                            }
                                        } else if format as libc::c_int
                                            == CAIRO_FORMAT_RGB96F as libc::c_int
                                        {
                                            i = png_height;
                                            loop {
                                                let fresh31 = i;
                                                i = i.wrapping_sub(1);
                                                if !(fresh31 != 0) {
                                                    break;
                                                }
                                                let mut float_line_0: *mut libc::c_float = *row_pointers
                                                    .offset(i as isize) as *mut libc::c_float;
                                                let mut u16_line_0: *mut uint16_t = *row_pointers
                                                    .offset(i as isize) as *mut uint16_t;
                                                convert_u16_to_float(float_line_0, u16_line_0, png_width);
                                            }
                                        }
                                        ::std::ptr::write_volatile(
                                            &mut surface as *mut *mut cairo_surface_t,
                                            cairo_image_surface_create_for_data(
                                                data,
                                                format,
                                                png_width as libc::c_int,
                                                png_height as libc::c_int,
                                                stride,
                                            ),
                                        );
                                        if !((*surface).status as u64 != 0) {
                                            _cairo_image_surface_assume_ownership_of_data(
                                                surface as *mut cairo_image_surface_t,
                                            );
                                            ::std::ptr::write_volatile(
                                                &mut data as *mut *mut png_byte,
                                                0 as *mut png_byte,
                                            );
                                            status = _cairo_memory_stream_destroy(
                                                (*png_closure).png_data,
                                                &mut mime_data,
                                                &mut mime_data_length,
                                            );
                                            let ref mut fresh32 = (*png_closure).png_data;
                                            *fresh32 = 0 as *mut cairo_output_stream_t;
                                            if status as u64 != 0 {
                                                cairo_surface_destroy(surface);
                                                ::std::ptr::write_volatile(
                                                    &mut surface as *mut *mut cairo_surface_t,
                                                    _cairo_surface_create_in_error(status),
                                                );
                                            } else {
                                                status = cairo_surface_set_mime_data(
                                                    surface,
                                                    b"image/png\0" as *const u8 as *const libc::c_char,
                                                    mime_data,
                                                    mime_data_length,
                                                    Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                                                    mime_data as *mut libc::c_void,
                                                );
                                                if status as u64 != 0 {
                                                    free(mime_data as *mut libc::c_void);
                                                    cairo_surface_destroy(surface);
                                                    ::std::ptr::write_volatile(
                                                        &mut surface as *mut *mut cairo_surface_t,
                                                        _cairo_surface_create_in_error(status),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(row_pointers as *mut libc::c_void);
    free(data as *mut libc::c_void);
    if !png.is_null() {
        png_destroy_read_struct(&mut png, &mut info, 0 as png_infopp);
    }
    if !((*png_closure).png_data).is_null() {
        let mut status_ignored: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status_ignored = _cairo_output_stream_destroy((*png_closure).png_data);
    }
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_create_from_png(
    mut filename: *const libc::c_char,
) -> *mut cairo_surface_t {
    let mut png_closure: png_read_closure_t = png_read_closure_t {
        read_func: None,
        closure: 0 as *mut libc::c_void,
        png_data: 0 as *mut cairo_output_stream_t,
    };
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_fopen(
        filename,
        b"rb\0" as *const u8 as *const libc::c_char,
        &mut png_closure.closure as *mut *mut libc::c_void as *mut *mut FILE,
    );
    if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return _cairo_surface_create_in_error(status);
    }
    if (png_closure.closure).is_null() {
        match *__errno_location() {
            12 => {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            2 => {
                status = _cairo_error(CAIRO_STATUS_FILE_NOT_FOUND);
            }
            _ => {
                status = _cairo_error(CAIRO_STATUS_READ_ERROR);
            }
        }
        return _cairo_surface_create_in_error(status);
    }
    png_closure
        .read_func = Some(
        stdio_read_func
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                libc::c_uint,
            ) -> cairo_status_t,
    );
    surface = read_png(&mut png_closure);
    fclose(png_closure.closure as *mut FILE);
    return surface;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_image_surface_create_from_png_stream(
    mut read_func: cairo_read_func_t,
    mut closure: *mut libc::c_void,
) -> *mut cairo_surface_t {
    let mut png_closure: png_read_closure_t = png_read_closure_t {
        read_func: None,
        closure: 0 as *mut libc::c_void,
        png_data: 0 as *mut cairo_output_stream_t,
    };
    png_closure.read_func = read_func;
    png_closure.closure = closure;
    return read_png(&mut png_closure);
}
