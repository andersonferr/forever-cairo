use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_image_surface;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type _cairo_pattern;
    pub type _cairo_hash_table;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn _cairo_path_fixed_interpret_flat(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
        tolerance: libc::c_double,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn cairo_move_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);
    fn cairo_line_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);
    fn cairo_curve_to(
        cr: *mut cairo_t,
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
        x3: libc::c_double,
        y3: libc::c_double,
    );
    fn cairo_close_path(cr: *mut cairo_t);
    fn cairo_get_tolerance(cr: *mut cairo_t) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_backend {
    pub type_0: cairo_backend_type_t,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub get_original_target: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
    >,
    pub get_current_target: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
    >,
    pub save: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub restore: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub push_group: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_content_t) -> cairo_status_t,
    >,
    pub pop_group: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_pattern_t,
    >,
    pub set_source_rgba: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub set_source_surface: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_surface_t,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub set_source: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_pattern_t) -> cairo_status_t,
    >,
    pub get_source: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_pattern_t,
    >,
    pub set_antialias: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_antialias_t) -> cairo_status_t,
    >,
    pub set_dash: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            libc::c_int,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub set_fill_rule: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_fill_rule_t) -> cairo_status_t,
    >,
    pub set_line_cap: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_line_cap_t) -> cairo_status_t,
    >,
    pub set_line_join: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_line_join_t) -> cairo_status_t,
    >,
    pub set_line_width: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub set_hairline: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_bool_t) -> cairo_status_t,
    >,
    pub set_miter_limit: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub set_opacity: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub set_operator: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_operator_t) -> cairo_status_t,
    >,
    pub set_tolerance: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub get_antialias: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_antialias_t,
    >,
    pub get_dash: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> (),
    >,
    pub get_fill_rule: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_fill_rule_t,
    >,
    pub get_line_cap: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_line_cap_t,
    >,
    pub get_line_join: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_line_join_t,
    >,
    pub get_line_width: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
    >,
    pub get_hairline: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t>,
    pub get_miter_limit: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
    >,
    pub get_opacity: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
    pub get_operator: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_operator_t,
    >,
    pub get_tolerance: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
    >,
    pub translate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub scale: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub rotate: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub transform: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_matrix_t) -> cairo_status_t,
    >,
    pub set_matrix: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_matrix_t) -> cairo_status_t,
    >,
    pub set_identity_matrix: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
    >,
    pub get_matrix: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_matrix_t) -> (),
    >,
    pub user_to_device: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub user_to_device_distance: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub device_to_user: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub device_to_user_distance: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub user_to_backend: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub user_to_backend_distance: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub backend_to_user: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub backend_to_user_distance: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub new_path: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub new_sub_path: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
    >,
    pub move_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub rel_move_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub line_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub rel_line_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub curve_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub rel_curve_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub arc_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub rel_arc_to: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub close_path: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub arc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            cairo_bool_t,
        ) -> cairo_status_t,
    >,
    pub rectangle: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> cairo_status_t,
    >,
    pub path_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub has_current_point: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
    pub get_current_point: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> cairo_bool_t,
    >,
    pub copy_path: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_path_t,
    >,
    pub copy_path_flat: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_path_t,
    >,
    pub append_path: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_path_t) -> cairo_status_t,
    >,
    pub stroke_to_path: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
    >,
    pub clip: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub clip_preserve: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
    >,
    pub in_clip: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            *mut cairo_bool_t,
        ) -> cairo_status_t,
    >,
    pub clip_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> cairo_status_t,
    >,
    pub reset_clip: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub clip_copy_rectangle_list: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_rectangle_list_t,
    >,
    pub paint: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub paint_with_alpha: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub mask: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_pattern_t) -> cairo_status_t,
    >,
    pub stroke: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub stroke_preserve: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
    >,
    pub in_stroke: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            *mut cairo_bool_t,
        ) -> cairo_status_t,
    >,
    pub stroke_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> cairo_status_t,
    >,
    pub fill: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub fill_preserve: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
    >,
    pub in_fill: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            *mut cairo_bool_t,
        ) -> cairo_status_t,
    >,
    pub fill_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> cairo_status_t,
    >,
    pub set_font_face: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_font_face_t) -> cairo_status_t,
    >,
    pub get_font_face: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_font_face_t,
    >,
    pub set_font_size: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> cairo_status_t,
    >,
    pub set_font_matrix: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const cairo_matrix_t) -> cairo_status_t,
    >,
    pub get_font_matrix: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_matrix_t) -> (),
    >,
    pub set_font_options: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_font_options_t,
        ) -> cairo_status_t,
    >,
    pub get_font_options: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_font_options_t) -> (),
    >,
    pub set_scaled_font: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_scaled_font_t,
        ) -> cairo_status_t,
    >,
    pub get_scaled_font: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_scaled_font_t,
    >,
    pub font_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_font_extents_t,
        ) -> cairo_status_t,
    >,
    pub glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_glyph_t,
            libc::c_int,
            *mut cairo_glyph_text_info_t,
        ) -> cairo_status_t,
    >,
    pub glyph_path: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_glyph_t,
            libc::c_int,
        ) -> cairo_status_t,
    >,
    pub glyph_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_glyph_t,
            libc::c_int,
            *mut cairo_text_extents_t,
        ) -> cairo_status_t,
    >,
    pub copy_page: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub show_page: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub tag_begin: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
        ) -> cairo_status_t,
    >,
    pub tag_end: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> cairo_status_t,
    >,
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
pub struct cairo_text_extents_t {
    pub x_bearing: libc::c_double,
    pub y_bearing: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_glyph_t {
    pub index: libc::c_ulong,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_glyph_text_info_t = _cairo_glyph_text_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_glyph_text_info {
    pub utf8: *const libc::c_char,
    pub utf8_len: libc::c_int,
    pub clusters: *const cairo_text_cluster_t,
    pub num_clusters: libc::c_int,
    pub cluster_flags: cairo_text_cluster_flags_t,
}
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
pub struct cairo_font_extents_t {
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub height: libc::c_double,
    pub max_x_advance: libc::c_double,
    pub max_y_advance: libc::c_double,
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
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
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
pub type uint32_t = __uint32_t;
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
pub type cairo_surface_t = _cairo_surface;
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
pub type cairo_surface_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_surface_t) -> (),
>;
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
pub type cairo_t = _cairo;
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
pub type cairo_rectangle_list_t = _cairo_rectangle_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_list {
    pub status: cairo_status_t,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: libc::c_int,
}
pub type cairo_rectangle_t = _cairo_rectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_path_t = cairo_path;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_path {
    pub status: cairo_status_t,
    pub data: *mut cairo_path_data_t,
    pub num_data: libc::c_int,
}
pub type cairo_path_data_t = _cairo_path_data_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _cairo_path_data_t {
    pub header: C2RustUnnamed_0,
    pub point: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub type_0: cairo_path_data_type_t,
    pub length: libc::c_int,
}
pub type cairo_path_data_type_t = _cairo_path_data_type;
pub type _cairo_path_data_type = libc::c_uint;
pub const CAIRO_PATH_CLOSE_PATH: _cairo_path_data_type = 3;
pub const CAIRO_PATH_CURVE_TO: _cairo_path_data_type = 2;
pub const CAIRO_PATH_LINE_TO: _cairo_path_data_type = 1;
pub const CAIRO_PATH_MOVE_TO: _cairo_path_data_type = 0;
pub type cairo_backend_type_t = _cairo_backend_type;
pub type _cairo_backend_type = libc::c_uint;
pub const CAIRO_TYPE_SKIA: _cairo_backend_type = 1;
pub const CAIRO_TYPE_DEFAULT: _cairo_backend_type = 0;
pub type cairo_path_fixed_move_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_line_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_curve_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
    *const cairo_point_t,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_close_path_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> cairo_status_t;
pub type cpp_t = cairo_path_populate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_path_populate {
    pub data: *mut cairo_path_data_t,
    pub cr: *mut cairo_t,
}
pub type cpc_t = cairo_path_count;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_path_count {
    pub count: libc::c_int,
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
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_backend_to_user(
    mut cr: *mut cairo_t,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    ((*(*cr).backend).backend_to_user)
        .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
}
static mut _cairo_path_nil: cairo_path_t = {
    let mut init = cairo_path {
        status: CAIRO_STATUS_NO_MEMORY,
        data: 0 as *const cairo_path_data_t as *mut cairo_path_data_t,
        num_data: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn _cpc_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpc: *mut cpc_t = closure as *mut cpc_t;
    (*cpc).count += 2 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cpc_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpc: *mut cpc_t = closure as *mut cpc_t;
    (*cpc).count += 2 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cpc_curve_to(
    mut closure: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpc: *mut cpc_t = closure as *mut cpc_t;
    (*cpc).count += 4 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cpc_close_path(mut closure: *mut libc::c_void) -> cairo_status_t {
    let mut cpc: *mut cpc_t = closure as *mut cpc_t;
    (*cpc).count += 1 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_count(
    mut path: *mut cairo_path_t,
    mut path_fixed: *mut cairo_path_fixed_t,
    mut tolerance: libc::c_double,
    mut flatten: cairo_bool_t,
) -> libc::c_int {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut cpc: cpc_t = cpc_t { count: 0 };
    cpc.count = 0 as libc::c_int;
    if flatten != 0 {
        status = _cairo_path_fixed_interpret_flat(
            path_fixed,
            Some(
                _cpc_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpc_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpc_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut cpc as *mut cpc_t as *mut libc::c_void,
            tolerance,
        );
    } else {
        status = _cairo_path_fixed_interpret(
            path_fixed,
            Some(
                _cpc_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpc_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpc_curve_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                        *const cairo_point_t,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpc_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut cpc as *mut cpc_t as *mut libc::c_void,
        );
    }
    if status as u64 != 0 {
        return -(1 as libc::c_int);
    }
    return cpc.count;
}
unsafe extern "C" fn _cpp_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpp: *mut cpp_t = closure as *mut cpp_t;
    let mut data: *mut cairo_path_data_t = (*cpp).data;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    x = _cairo_fixed_to_double((*point).x);
    y = _cairo_fixed_to_double((*point).y);
    _cairo_backend_to_user((*cpp).cr, &mut x, &mut y);
    (*data).header.type_0 = CAIRO_PATH_MOVE_TO;
    (*data).header.length = 2 as libc::c_int;
    (*data.offset(1 as libc::c_int as isize)).point.x = x;
    (*data.offset(1 as libc::c_int as isize)).point.y = y;
    let ref mut fresh2 = (*cpp).data;
    *fresh2 = (*fresh2).offset((*data).header.length as isize);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cpp_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpp: *mut cpp_t = closure as *mut cpp_t;
    let mut data: *mut cairo_path_data_t = (*cpp).data;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    x = _cairo_fixed_to_double((*point).x);
    y = _cairo_fixed_to_double((*point).y);
    _cairo_backend_to_user((*cpp).cr, &mut x, &mut y);
    (*data).header.type_0 = CAIRO_PATH_LINE_TO;
    (*data).header.length = 2 as libc::c_int;
    (*data.offset(1 as libc::c_int as isize)).point.x = x;
    (*data.offset(1 as libc::c_int as isize)).point.y = y;
    let ref mut fresh3 = (*cpp).data;
    *fresh3 = (*fresh3).offset((*data).header.length as isize);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cpp_curve_to(
    mut closure: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpp: *mut cpp_t = closure as *mut cpp_t;
    let mut data: *mut cairo_path_data_t = (*cpp).data;
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut x3: libc::c_double = 0.;
    let mut y3: libc::c_double = 0.;
    x1 = _cairo_fixed_to_double((*p1).x);
    y1 = _cairo_fixed_to_double((*p1).y);
    _cairo_backend_to_user((*cpp).cr, &mut x1, &mut y1);
    x2 = _cairo_fixed_to_double((*p2).x);
    y2 = _cairo_fixed_to_double((*p2).y);
    _cairo_backend_to_user((*cpp).cr, &mut x2, &mut y2);
    x3 = _cairo_fixed_to_double((*p3).x);
    y3 = _cairo_fixed_to_double((*p3).y);
    _cairo_backend_to_user((*cpp).cr, &mut x3, &mut y3);
    (*data).header.type_0 = CAIRO_PATH_CURVE_TO;
    (*data).header.length = 4 as libc::c_int;
    (*data.offset(1 as libc::c_int as isize)).point.x = x1;
    (*data.offset(1 as libc::c_int as isize)).point.y = y1;
    (*data.offset(2 as libc::c_int as isize)).point.x = x2;
    (*data.offset(2 as libc::c_int as isize)).point.y = y2;
    (*data.offset(3 as libc::c_int as isize)).point.x = x3;
    (*data.offset(3 as libc::c_int as isize)).point.y = y3;
    let ref mut fresh4 = (*cpp).data;
    *fresh4 = (*fresh4).offset((*data).header.length as isize);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cpp_close_path(mut closure: *mut libc::c_void) -> cairo_status_t {
    let mut cpp: *mut cpp_t = closure as *mut cpp_t;
    let mut data: *mut cairo_path_data_t = (*cpp).data;
    (*data).header.type_0 = CAIRO_PATH_CLOSE_PATH;
    (*data).header.length = 1 as libc::c_int;
    let ref mut fresh5 = (*cpp).data;
    *fresh5 = (*fresh5).offset((*data).header.length as isize);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_populate(
    mut path: *mut cairo_path_t,
    mut path_fixed: *mut cairo_path_fixed_t,
    mut cr: *mut cairo_t,
    mut flatten: cairo_bool_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut cpp: cpp_t = cpp_t {
        data: 0 as *mut cairo_path_data_t,
        cr: 0 as *mut cairo_t,
    };
    cpp.data = (*path).data;
    cpp.cr = cr;
    if flatten != 0 {
        status = _cairo_path_fixed_interpret_flat(
            path_fixed,
            Some(
                _cpp_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpp_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpp_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut cpp as *mut cpp_t as *mut libc::c_void,
            cairo_get_tolerance(cr),
        );
    } else {
        status = _cairo_path_fixed_interpret(
            path_fixed,
            Some(
                _cpp_move_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpp_line_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpp_curve_to
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_point_t,
                        *const cairo_point_t,
                        *const cairo_point_t,
                    ) -> cairo_status_t,
            ),
            Some(
                _cpp_close_path
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            &mut cpp as *mut cpp_t as *mut libc::c_void,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    if (cpp.data).offset_from((*path).data) as libc::c_long
        == (*path).num_data as libc::c_long
    {} else {
        __assert_fail(
            b"cpp.data - path->data == path->num_data\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-path.c\0" as *const u8 as *const libc::c_char,
            283 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"cairo_status_t _cairo_path_populate(cairo_path_t *, cairo_path_fixed_t *, cairo_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_path_t {
    let mut path: *mut cairo_path_t = 0 as *mut cairo_path_t;
    if status as libc::c_uint == CAIRO_STATUS_NO_MEMORY as libc::c_int as libc::c_uint {
        return &_cairo_path_nil as *const cairo_path_t as *mut cairo_path_t;
    }
    path = (if ::std::mem::size_of::<cairo_path_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_path_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_path_t;
    if path.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_path_nil as *const cairo_path_t as *mut cairo_path_t;
    }
    (*path).num_data = 0 as libc::c_int;
    let ref mut fresh6 = (*path).data;
    *fresh6 = 0 as *mut cairo_path_data_t;
    (*path).status = status;
    return path;
}
unsafe extern "C" fn _cairo_path_create_internal(
    mut path_fixed: *mut cairo_path_fixed_t,
    mut cr: *mut cairo_t,
    mut flatten: cairo_bool_t,
) -> *mut cairo_path_t {
    let mut path: *mut cairo_path_t = 0 as *mut cairo_path_t;
    path = (if ::std::mem::size_of::<cairo_path_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_path_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_path_t;
    if path.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_path_nil as *const cairo_path_t as *mut cairo_path_t;
    }
    (*path)
        .num_data = _cairo_path_count(
        path,
        path_fixed,
        cairo_get_tolerance(cr),
        flatten,
    );
    if (*path).num_data < 0 as libc::c_int {
        free(path as *mut libc::c_void);
        return &_cairo_path_nil as *const cairo_path_t as *mut cairo_path_t;
    }
    if (*path).num_data != 0 {
        let ref mut fresh7 = (*path).data;
        *fresh7 = _cairo_malloc_ab(
            (*path).num_data as size_t,
            ::std::mem::size_of::<cairo_path_data_t>() as libc::c_ulong,
        ) as *mut cairo_path_data_t;
        if ((*path).data).is_null() {
            free(path as *mut libc::c_void);
            let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            return &_cairo_path_nil as *const cairo_path_t as *mut cairo_path_t;
        }
        (*path).status = _cairo_path_populate(path, path_fixed, cr, flatten);
    } else {
        let ref mut fresh8 = (*path).data;
        *fresh8 = 0 as *mut cairo_path_data_t;
        (*path).status = CAIRO_STATUS_SUCCESS;
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_path_destroy(mut path: *mut cairo_path_t) {
    if path.is_null()
        || path == &_cairo_path_nil as *const cairo_path_t as *mut cairo_path_t
    {
        return;
    }
    free((*path).data as *mut libc::c_void);
    free(path as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_create(
    mut path: *mut cairo_path_fixed_t,
    mut cr: *mut cairo_t,
) -> *mut cairo_path_t {
    return _cairo_path_create_internal(path, cr, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_create_flat(
    mut path: *mut cairo_path_fixed_t,
    mut cr: *mut cairo_t,
) -> *mut cairo_path_t {
    return _cairo_path_create_internal(path, cr, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_append_to_context(
    mut path: *const cairo_path_t,
    mut cr: *mut cairo_t,
) -> cairo_status_t {
    let mut p: *const cairo_path_data_t = 0 as *const cairo_path_data_t;
    let mut end: *const cairo_path_data_t = 0 as *const cairo_path_data_t;
    end = &mut *((*path).data).offset((*path).num_data as isize)
        as *mut cairo_path_data_t;
    p = &mut *((*path).data).offset(0 as libc::c_int as isize) as *mut cairo_path_data_t;
    while p < end {
        match (*p).header.type_0 as libc::c_uint {
            0 => {
                if (*p).header.length < 2 as libc::c_int {
                    return _cairo_error(CAIRO_STATUS_INVALID_PATH_DATA);
                }
                cairo_move_to(
                    cr,
                    (*p.offset(1 as libc::c_int as isize)).point.x,
                    (*p.offset(1 as libc::c_int as isize)).point.y,
                );
            }
            1 => {
                if (*p).header.length < 2 as libc::c_int {
                    return _cairo_error(CAIRO_STATUS_INVALID_PATH_DATA);
                }
                cairo_line_to(
                    cr,
                    (*p.offset(1 as libc::c_int as isize)).point.x,
                    (*p.offset(1 as libc::c_int as isize)).point.y,
                );
            }
            2 => {
                if (*p).header.length < 4 as libc::c_int {
                    return _cairo_error(CAIRO_STATUS_INVALID_PATH_DATA);
                }
                cairo_curve_to(
                    cr,
                    (*p.offset(1 as libc::c_int as isize)).point.x,
                    (*p.offset(1 as libc::c_int as isize)).point.y,
                    (*p.offset(2 as libc::c_int as isize)).point.x,
                    (*p.offset(2 as libc::c_int as isize)).point.y,
                    (*p.offset(3 as libc::c_int as isize)).point.x,
                    (*p.offset(3 as libc::c_int as isize)).point.y,
                );
            }
            3 => {
                if (*p).header.length < 1 as libc::c_int {
                    return _cairo_error(CAIRO_STATUS_INVALID_PATH_DATA);
                }
                cairo_close_path(cr);
            }
            _ => return _cairo_error(CAIRO_STATUS_INVALID_PATH_DATA),
        }
        if (*cr).status as u64 != 0 {
            return (*cr).status;
        }
        p = p.offset((*p).header.length as isize);
    }
    return CAIRO_STATUS_SUCCESS;
}
