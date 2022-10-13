// #[derive(Copy, Clone)]
//
// pub struct _cairo {
//     pub ref_count: cairo_reference_count_t,
//     pub status: cairo_status_t,
//     pub user_data: cairo_user_data_array_t,
//     pub backend: *const cairo_backend_t,
// }

// pub type cairo_point_t = _cairo_point;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_point {
//     pub x: cairo_fixed_t,
//     pub y: cairo_fixed_t,
// }
// pub type cairo_fixed_t = int32_t;
// pub type cairo_region_t = _cairo_region;
// pub type cairo_clip_path_t = _cairo_clip_path;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_clip_path {
//     pub ref_count: cairo_reference_count_t,
//     pub path: cairo_path_fixed_t,
//     pub fill_rule: cairo_fill_rule_t,
//     pub tolerance: libc::c_double,
//     pub antialias: cairo_antialias_t,
//     pub prev: *mut cairo_clip_path_t,
// }
// pub type cairo_fill_rule_t = _cairo_fill_rule;
// pub type _cairo_fill_rule = libc::c_uint;
// pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;
// pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
// pub type cairo_path_fixed_t = _cairo_path_fixed;
// #[derive(Copy, Clone, BitfieldStruct)]
//
// pub struct _cairo_path_fixed {
//     pub last_move_point: cairo_point_t,
//     pub current_point: cairo_point_t,
//     #[bitfield(name = "has_current_point", ty = "libc::c_uint", bits = "0..=0")]
//     #[bitfield(name = "needs_move_to", ty = "libc::c_uint", bits = "1..=1")]
//     #[bitfield(name = "has_extents", ty = "libc::c_uint", bits = "2..=2")]
//     #[bitfield(name = "has_curve_to", ty = "libc::c_uint", bits = "3..=3")]
//     #[bitfield(name = "stroke_is_rectilinear", ty = "libc::c_uint", bits = "4..=4")]
//     #[bitfield(name = "fill_is_rectilinear", ty = "libc::c_uint", bits = "5..=5")]
//     #[bitfield(name = "fill_maybe_region", ty = "libc::c_uint", bits = "6..=6")]
//     #[bitfield(name = "fill_is_empty", ty = "libc::c_uint", bits = "7..=7")]
//     pub has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [u8; 1],
//     #[bitfield(padding)]
//     pub c2rust_padding: [u8; 3],
//     pub extents: cairo_box_t,
//     pub buf: cairo_path_buf_fixed_t,
// }
// pub type cairo_path_buf_fixed_t = _cairo_path_buf_fixed;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_path_buf_fixed {
//     pub base: cairo_path_buf_t,
//     pub op: [cairo_path_op_t; 27],
//     pub points: [cairo_point_t; 54],
// }
// pub type cairo_path_op_t = libc::c_char;
// pub type cairo_path_buf_t = _cairo_path_buf;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_path_buf {
//     pub link: cairo_list_t,
//     pub num_ops: libc::c_uint,
//     pub size_ops: libc::c_uint,
//     pub num_points: libc::c_uint,
//     pub size_points: libc::c_uint,
//     pub op: *mut cairo_path_op_t,
//     pub points: *mut cairo_point_t,
// }
// pub type cairo_rectangle_int_t = _cairo_rectangle_int;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_rectangle_int {
//     pub x: libc::c_int,
//     pub y: libc::c_int,
//     pub width: libc::c_int,
//     pub height: libc::c_int,
// }
// pub type cairo_pattern_t = _cairo_pattern;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_pattern {
//     pub ref_count: cairo_reference_count_t,
//     pub status: cairo_status_t,
//     pub user_data: cairo_user_data_array_t,
//     pub observers: cairo_list_t,
//     pub type_0: cairo_pattern_type_t,
//     pub filter: cairo_filter_t,
//     pub extend: cairo_extend_t,
//     pub has_component_alpha: cairo_bool_t,
//     pub is_userfont_foreground: cairo_bool_t,
//     pub matrix: cairo_matrix_t,
//     pub opacity: libc::c_double,
// }
// pub type cairo_extend_t = _cairo_extend;
// pub type _cairo_extend = libc::c_uint;
// pub const CAIRO_EXTEND_PAD: _cairo_extend = 3;
// pub const CAIRO_EXTEND_REFLECT: _cairo_extend = 2;
// pub const CAIRO_EXTEND_REPEAT: _cairo_extend = 1;
// pub const CAIRO_EXTEND_NONE: _cairo_extend = 0;
// pub type cairo_filter_t = _cairo_filter;
// pub type _cairo_filter = libc::c_uint;
// pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;
// pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
// pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
// pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
// pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
// pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
// pub type cairo_pattern_type_t = _cairo_pattern_type;
// pub type _cairo_pattern_type = libc::c_uint;
// pub const CAIRO_PATTERN_TYPE_RASTER_SOURCE: _cairo_pattern_type = 5;
// pub const CAIRO_PATTERN_TYPE_MESH: _cairo_pattern_type = 4;
// pub const CAIRO_PATTERN_TYPE_RADIAL: _cairo_pattern_type = 3;
// pub const CAIRO_PATTERN_TYPE_LINEAR: _cairo_pattern_type = 2;
// pub const CAIRO_PATTERN_TYPE_SURFACE: _cairo_pattern_type = 1;
// pub const CAIRO_PATTERN_TYPE_SOLID: _cairo_pattern_type = 0;
// pub type cairo_operator_t = _cairo_operator;
// pub type _cairo_operator = libc::c_uint;
// pub const CAIRO_OPERATOR_HSL_LUMINOSITY: _cairo_operator = 28;
// pub const CAIRO_OPERATOR_HSL_COLOR: _cairo_operator = 27;
// pub const CAIRO_OPERATOR_HSL_SATURATION: _cairo_operator = 26;
// pub const CAIRO_OPERATOR_HSL_HUE: _cairo_operator = 25;
// pub const CAIRO_OPERATOR_EXCLUSION: _cairo_operator = 24;
// pub const CAIRO_OPERATOR_DIFFERENCE: _cairo_operator = 23;
// pub const CAIRO_OPERATOR_SOFT_LIGHT: _cairo_operator = 22;
// pub const CAIRO_OPERATOR_HARD_LIGHT: _cairo_operator = 21;
// pub const CAIRO_OPERATOR_COLOR_BURN: _cairo_operator = 20;
// pub const CAIRO_OPERATOR_COLOR_DODGE: _cairo_operator = 19;
// pub const CAIRO_OPERATOR_LIGHTEN: _cairo_operator = 18;
// pub const CAIRO_OPERATOR_DARKEN: _cairo_operator = 17;
// pub const CAIRO_OPERATOR_OVERLAY: _cairo_operator = 16;
// pub const CAIRO_OPERATOR_SCREEN: _cairo_operator = 15;
// pub const CAIRO_OPERATOR_MULTIPLY: _cairo_operator = 14;
// pub const CAIRO_OPERATOR_SATURATE: _cairo_operator = 13;
// pub const CAIRO_OPERATOR_ADD: _cairo_operator = 12;
// pub const CAIRO_OPERATOR_XOR: _cairo_operator = 11;
// pub const CAIRO_OPERATOR_DEST_ATOP: _cairo_operator = 10;
// pub const CAIRO_OPERATOR_DEST_OUT: _cairo_operator = 9;
// pub const CAIRO_OPERATOR_DEST_IN: _cairo_operator = 8;
// pub const CAIRO_OPERATOR_DEST_OVER: _cairo_operator = 7;
// pub const CAIRO_OPERATOR_DEST: _cairo_operator = 6;
// pub const CAIRO_OPERATOR_ATOP: _cairo_operator = 5;
// pub const CAIRO_OPERATOR_OUT: _cairo_operator = 4;
// pub const CAIRO_OPERATOR_IN: _cairo_operator = 3;
// pub const CAIRO_OPERATOR_OVER: _cairo_operator = 2;
// pub const CAIRO_OPERATOR_SOURCE: _cairo_operator = 1;
// pub const CAIRO_OPERATOR_CLEAR: _cairo_operator = 0;
// pub type cairo_stroke_style_t = _cairo_stroke_style;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_stroke_style {
//     pub line_width: libc::c_double,
//     pub line_cap: cairo_line_cap_t,
//     pub line_join: cairo_line_join_t,
//     pub miter_limit: libc::c_double,
//     pub dash: *mut libc::c_double,
//     pub num_dashes: libc::c_uint,
//     pub dash_offset: libc::c_double,
//     pub is_hairline: cairo_bool_t,
//     pub pre_hairline_line_width: libc::c_double,
// }
// pub type cairo_line_join_t = _cairo_line_join;
// pub type _cairo_line_join = libc::c_uint;
// pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;
// pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
// pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
// pub type cairo_line_cap_t = _cairo_line_cap;
// pub type _cairo_line_cap = libc::c_uint;
// pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;
// pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
// pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
// pub type cairo_format_t = _cairo_format;
// pub type _cairo_format = libc::c_int;
// pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
// pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
// pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
// pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
// pub const CAIRO_FORMAT_A1: _cairo_format = 3;
// pub const CAIRO_FORMAT_A8: _cairo_format = 2;
// pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
// pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
// pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
// pub type cairo_t = _cairo;
// pub type cairo_hash_entry_t = _cairo_hash_entry;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_hash_entry {
//     pub hash: uintptr_t,
// }
// pub type uintptr_t = libc::c_ulong;
// pub type cairo_font_type_t = _cairo_font_type;
// pub type _cairo_font_type = libc::c_uint;
// pub const CAIRO_FONT_TYPE_DWRITE: _cairo_font_type = 5;
// pub const CAIRO_FONT_TYPE_USER: _cairo_font_type = 4;
// pub const CAIRO_FONT_TYPE_QUARTZ: _cairo_font_type = 3;
// pub const CAIRO_FONT_TYPE_WIN32: _cairo_font_type = 2;
// pub const CAIRO_FONT_TYPE_FT: _cairo_font_type = 1;
// pub const CAIRO_FONT_TYPE_TOY: _cairo_font_type = 0;
// pub type cairo_hash_table_t = _cairo_hash_table;
// pub type cairo_recursive_mutex_t = cairo_recursive_mutex_impl_t;
// pub type cairo_recursive_mutex_impl_t = pthread_mutex_t;
// pub type cairo_font_face_t = _cairo_font_face;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_font_face {
//     pub hash_entry: cairo_hash_entry_t,
//     pub status: cairo_status_t,
//     pub ref_count: cairo_reference_count_t,
//     pub user_data: cairo_user_data_array_t,
//     pub backend: *const cairo_font_face_backend_t,
// }
// pub type cairo_font_face_backend_t = _cairo_font_face_backend;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_font_face_backend {
//     pub type_0: cairo_font_type_t,
//     pub create_for_toy: Option::<
//          fn(
//             *mut cairo_toy_font_face_t,
//             *mut *mut cairo_font_face_t,
//         ) -> cairo_status_t,
//     >,
//     pub destroy: Option::< fn(*mut libc::c_void) -> cairo_bool_t>,
//     pub scaled_font_create: Option::<
//          fn(
//             *mut libc::c_void,
//             *const cairo_matrix_t,
//             *const cairo_matrix_t,
//             *const cairo_font_options_t,
//             *mut *mut cairo_scaled_font_t,
//         ) -> cairo_status_t,
//     >,
//     pub get_implementation: Option::<
//          fn(
//             *mut libc::c_void,
//             *const cairo_matrix_t,
//             *const cairo_matrix_t,
//             *const cairo_font_options_t,
//         ) -> *mut cairo_font_face_t,
//     >,
// }
// pub type cairo_toy_font_face_t = _cairo_toy_font_face;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_toy_font_face {
//     pub base: cairo_font_face_t,
//     pub family: *const libc::c_char,
//     pub owns_family: cairo_bool_t,
//     pub slant: cairo_font_slant_t,
//     pub weight: cairo_font_weight_t,
//     pub impl_face: *mut cairo_font_face_t,
// }
// pub type cairo_font_weight_t = _cairo_font_weight;
// pub type _cairo_font_weight = libc::c_uint;
// pub const CAIRO_FONT_WEIGHT_BOLD: _cairo_font_weight = 1;
// pub const CAIRO_FONT_WEIGHT_NORMAL: _cairo_font_weight = 0;
// pub type cairo_font_slant_t = _cairo_font_slant;
// pub type _cairo_font_slant = libc::c_uint;
// pub const CAIRO_FONT_SLANT_OBLIQUE: _cairo_font_slant = 2;
// pub const CAIRO_FONT_SLANT_ITALIC: _cairo_font_slant = 1;
// pub const CAIRO_FONT_SLANT_NORMAL: _cairo_font_slant = 0;
// pub type cairo_rectangle_list_t = _cairo_rectangle_list;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_rectangle_list {
//     pub status: cairo_status_t,
//     pub rectangles: *mut cairo_rectangle_t,
//     pub num_rectangles: libc::c_int,
// }
// pub type cairo_rectangle_t = _cairo_rectangle;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_rectangle {
//     pub x: libc::c_double,
//     pub y: libc::c_double,
//     pub width: libc::c_double,
//     pub height: libc::c_double,
// }
// pub type cairo_path_t = cairo_path;
// #[derive(Copy, Clone)]
//
// pub struct cairo_path {
//     pub status: cairo_status_t,
//     pub data: *mut cairo_path_data_t,
//     pub num_data: libc::c_int,
// }
// pub type cairo_path_data_t = _cairo_path_data_t;
// #[derive(Copy, Clone)]
//
// pub union _cairo_path_data_t {
//     pub header: C2RustUnnamed_0,
//     pub point: C2RustUnnamed,
// }
// #[derive(Copy, Clone)]
//
// pub struct C2RustUnnamed {
//     pub x: libc::c_double,
//     pub y: libc::c_double,
// }
// #[derive(Copy, Clone)]
//
// pub struct C2RustUnnamed_0 {
//     pub type_0: cairo_path_data_type_t,
//     pub length: libc::c_int,
// }
// pub type cairo_path_data_type_t = _cairo_path_data_type;
// pub type _cairo_path_data_type = libc::c_uint;
// pub const CAIRO_PATH_CLOSE_PATH: _cairo_path_data_type = 3;
// pub const CAIRO_PATH_CURVE_TO: _cairo_path_data_type = 2;
// pub const CAIRO_PATH_LINE_TO: _cairo_path_data_type = 1;
// pub const CAIRO_PATH_MOVE_TO: _cairo_path_data_type = 0;
// pub type cairo_backend_type_t = _cairo_backend_type;
// pub type _cairo_backend_type = libc::c_uint;
// pub const CAIRO_TYPE_SKIA: _cairo_backend_type = 1;
// pub const CAIRO_TYPE_DEFAULT: _cairo_backend_type = 0;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_gstate {
//     pub op: cairo_operator_t,
//     pub opacity: libc::c_double,
//     pub tolerance: libc::c_double,
//     pub antialias: cairo_antialias_t,
//     pub stroke_style: cairo_stroke_style_t,
//     pub fill_rule: cairo_fill_rule_t,
//     pub font_face: *mut cairo_font_face_t,
//     pub scaled_font: *mut cairo_scaled_font_t,
//     pub previous_scaled_font: *mut cairo_scaled_font_t,
//     pub font_matrix: cairo_matrix_t,
//     pub font_options: cairo_font_options_t,
//     pub clip: *mut cairo_clip_t,
//     pub target: *mut cairo_surface_t,
//     pub parent_target: *mut cairo_surface_t,
//     pub original_target: *mut cairo_surface_t,
//     pub device_transform_observer: cairo_observer_t,
//     pub ctm: cairo_matrix_t,
//     pub ctm_inverse: cairo_matrix_t,
//     pub source_ctm_inverse: cairo_matrix_t,
//     pub is_identity: cairo_bool_t,
//     pub source: *mut cairo_pattern_t,
//     pub next: *mut _cairo_gstate,
// }
// pub type cairo_observer_t = _cairo_observer;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_observer {
//     pub link: cairo_list_t,
//     pub callback: Option::<
//          fn(*mut cairo_observer_t, *mut libc::c_void) -> (),
//     >,
// }
// pub type cairo_gstate_t = _cairo_gstate;
// #[derive(Copy, Clone)]
//
// pub struct _cairo_solid_pattern {
//     pub base: cairo_pattern_t,
//     pub color: cairo_color_t,
// }
// pub type cairo_solid_pattern_t = _cairo_solid_pattern;
// pub type cairo_stock_t = libc::c_uint;
// pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
// pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
// pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
// pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
// #[derive(Copy, Clone)]
//
// pub struct freed_pool_t {
//     pub pool: [*mut libc::c_void; 16],
//     pub top: cairo_atomic_int_t,
// }
// #[derive(Copy, Clone)]
//
// pub union C2RustUnnamed_1 {
//     pub d: libc::c_double,
//     pub i: [int32_t; 2],
// }

// pub type cairo_default_context_t = _cairo_default_context;
// #[inline]
//  fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
//     return f as libc::c_double
//         / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
// }
// #[inline]
//  fn _cairo_fixed_from_double_clamped(
//     mut d: libc::c_double,
//     mut tolerance: libc::c_double,
// ) -> cairo_fixed_t {
//     if d
//         > 2147483647 as libc::c_int as libc::c_double
//             / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double - tolerance
//     {
//         d = 2147483647 as libc::c_int as libc::c_double
//             / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double - tolerance;
//     } else if d
//         < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
//             / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + tolerance
//     {
//         d = (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
//             / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + tolerance;
//     }
//     return _cairo_fixed_from_double(d);
// }
// #[inline]
//  fn _cairo_fixed_from_double(mut d: libc::c_double) -> cairo_fixed_t {
//     let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { d: 0. };
//     u
//         .d = d
//         + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
//             as libc::c_double * 1.5f64;
//     return u.i[0 as libc::c_int as usize];
// }
// #[inline]
//  fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
//     return i << 8 as libc::c_int;
// }
// #[inline(always)]
//  fn _cairo_atomic_int_get_relaxed(
//     mut x: *mut cairo_atomic_int_t,
// ) -> cairo_atomic_int_t {
//     return ::std::intrinsics::atomic_load_relaxed(x);
// }
// #[inline(always)]
//  fn _cairo_atomic_int_set_relaxed(
//     mut x: *mut cairo_atomic_int_t,
//     mut val: cairo_atomic_int_t,
// ) {
//     ::std::intrinsics::atomic_store_relaxed(x, val);
// }
// #[inline(always)]
//  fn _cairo_atomic_ptr_get(
//     mut x: *mut *mut libc::c_void,
// ) -> *mut libc::c_void {
//     return ::std::intrinsics::atomic_load(x);
// }
// #[inline(always)]
//  fn _cairo_atomic_ptr_cmpxchg_impl(
//     mut x: *mut *mut libc::c_void,
//     mut oldv: *mut libc::c_void,
//     mut newv: *mut libc::c_void,
// ) -> cairo_bool_t {
//     let mut expected: *mut libc::c_void = oldv;
//     let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
//     *&mut expected = fresh0.0;
//     return fresh0.1 as cairo_bool_t;
// }
// #[inline]
//  fn _cairo_restrict_value(
//     mut value: libc::c_double,
//     mut min: libc::c_double,
//     mut max: libc::c_double,
// ) -> libc::c_double {
//     if value < min {
//         return min
//     } else if value > max {
//         return max
//     } else {
//         return value
//     };
// }
// #[inline]
//  fn _cairo_clip_is_all_clipped(
//     mut clip: *const cairo_clip_t,
// ) -> cairo_bool_t {
//     return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
// }
// #[inline]
//  fn _cairo_gstate_user_to_backend(
//     mut gstate: *mut cairo_gstate_t,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     if (*gstate).is_identity == 0 {
//         _do_cairo_gstate_user_to_backend(gstate, x, y);
//     }
// }
// #[inline]
//  fn _cairo_gstate_user_to_backend_distance(
//     mut gstate: *mut cairo_gstate_t,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     if (*gstate).is_identity == 0 {
//         _do_cairo_gstate_user_to_backend_distance(gstate, x, y);
//     }
// }
// #[inline]
//  fn _cairo_gstate_backend_to_user(
//     mut gstate: *mut cairo_gstate_t,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     if (*gstate).is_identity == 0 {
//         _do_cairo_gstate_backend_to_user(gstate, x, y);
//     }
// }
// #[inline]
//  fn _cairo_gstate_backend_to_user_distance(
//     mut gstate: *mut cairo_gstate_t,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     if (*gstate).is_identity == 0 {
//         _do_cairo_gstate_backend_to_user_distance(gstate, x, y);
//     }
// }
// #[inline(always)]
//  fn _atomic_fetch(
//     mut slot: *mut *mut libc::c_void,
// ) -> *mut libc::c_void {
//     let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
//     loop {
//         ptr = _cairo_atomic_ptr_get(slot);
//         if !(_cairo_atomic_ptr_cmpxchg_impl(slot, ptr, 0 as *mut libc::c_void) == 0) {
//             break;
//         }
//     }
//     return ptr;
// }
// #[inline(always)]
//  fn _atomic_store(
//     mut slot: *mut *mut libc::c_void,
//     mut ptr: *mut libc::c_void,
// ) -> cairo_bool_t {
//     return _cairo_atomic_ptr_cmpxchg_impl(slot, 0 as *mut libc::c_void, ptr);
// }
// #[inline]
//  fn _freed_pool_get(mut pool: *mut freed_pool_t) -> *mut libc::c_void {
//     let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
//     let mut i: libc::c_int = 0;
//     i = _cairo_atomic_int_get_relaxed(&mut (*pool).top) - 1 as libc::c_int;
//     if i < 0 as libc::c_int {
//         i = 0 as libc::c_int;
//     }
//     ptr = _atomic_fetch(&mut *((*pool).pool).as_mut_ptr().offset(i as isize));
//     if !ptr.is_null() {
//         _cairo_atomic_int_set_relaxed(&mut (*pool).top, i);
//         return ptr;
//     }
//     return _freed_pool_get_search(pool);
// }
// #[inline]
//  fn _freed_pool_put(
//     mut pool: *mut freed_pool_t,
//     mut ptr: *mut libc::c_void,
// ) {
//     let mut i: libc::c_int = 0;
//     i = _cairo_atomic_int_get_relaxed(&mut (*pool).top);
//     if i
//         < (::std::mem::size_of::<[*mut libc::c_void; 16]>() as libc::c_ulong)
//             .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
//             as libc::c_int
//         && _atomic_store(&mut *((*pool).pool).as_mut_ptr().offset(i as isize), ptr) != 0
//     {
//         _cairo_atomic_int_set_relaxed(&mut (*pool).top, i + 1 as libc::c_int);
//         return;
//     }
//     _freed_pool_put_search(pool, ptr);
// }
// static mut context_pool: freed_pool_t = freed_pool_t {
//     pool: [0 as *const libc::c_void as *mut libc::c_void; 16],
//     top: 0,
// };

// pub  fn _cairo_default_context_reset_static_data() {
//     _freed_pool_reset(&mut context_pool);
// }

//  fn _cairo_default_context_destroy(mut abstract_cr: *mut libc::c_void) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_default_context_fini(cr);
//     cr.base.status = CAIRO_STATUS_NULL_POINTER;
//     _freed_pool_put(&mut context_pool, cr as *mut libc::c_void);
// }
//  fn _cairo_default_context_get_original_target(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_surface_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_original_target(cr.gstate);
// }
//  fn _cairo_default_context_get_current_target(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_surface_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_target(cr.gstate);
// }
//  fn _cairo_default_context_save(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_save(&mut cr.gstate, &mut cr.gstate_freelist);
// }
//  fn _cairo_default_context_restore(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     if _cairo_gstate_is_group(cr.gstate) != 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_RESTORE);
//     }
//     return _cairo_gstate_restore(&mut cr.gstate, &mut cr.gstate_freelist);
// }
//  fn _cairo_default_context_push_group(
//     mut abstract_cr: *mut libc::c_void,
//     mut content: cairo_content_t,
// ) -> cairo_status_t {
//     let mut current_block: u64;
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut group_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
//     let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     clip = _cairo_gstate_get_clip(cr.gstate);
//     if _cairo_clip_is_all_clipped(clip) != 0 {
//         group_surface = cairo_image_surface_create(
//             CAIRO_FORMAT_ARGB32,
//             0 as libc::c_int,
//             0 as libc::c_int,
//         );
//         status = (*group_surface).status;
//         if status as u64 != 0 {
//             current_block = 12624786710253696582;
//         } else {
//             current_block = 4068382217303356765;
//         }
//     } else {
//         let mut parent_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
//         let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
//             x: 0,
//             y: 0,
//             width: 0,
//             height: 0,
//         };
//         let mut bounded: cairo_bool_t = 0;
//         let mut is_empty: cairo_bool_t = 0;
//         parent_surface = _cairo_gstate_get_target(cr.gstate);
//         if (*parent_surface).status as u64 != 0 {
//             return (*parent_surface).status;
//         }
//         if (*parent_surface).finished() != 0 {
//             return _cairo_error(CAIRO_STATUS_SURFACE_FINISHED);
//         }
//         bounded = _cairo_surface_get_extents(parent_surface, &mut extents);
//         if !clip.is_null() {
//             is_empty = _cairo_rectangle_intersect(
//                 &mut extents,
//                 _cairo_clip_get_extents(clip),
//             );
//         }
//         if bounded == 0 {
//             group_surface = cairo_recording_surface_create(
//                 content,
//                 0 as *const cairo_rectangle_t,
//             );
//             extents.y = 0 as libc::c_int;
//             extents.x = extents.y;
//         } else {
//             group_surface = _cairo_surface_create_scratch(
//                 parent_surface,
//                 content,
//                 extents.width,
//                 extents.height,
//                 _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
//             );
//         }
//         status = (*group_surface).status;
//         if status as u64 != 0 {
//             current_block = 12624786710253696582;
//         } else {
//             cairo_surface_set_device_offset(
//                 group_surface,
//                 (*parent_surface).device_transform.x0 - extents.x as libc::c_double,
//                 (*parent_surface).device_transform.y0 - extents.y as libc::c_double,
//             );
//             cairo_surface_set_device_scale(
//                 group_surface,
//                 (*parent_surface).device_transform.xx,
//                 (*parent_surface).device_transform.yy,
//             );
//             _cairo_path_fixed_translate(
//                 (cr.path).as_mut_ptr(),
//                 _cairo_fixed_from_int(-extents.x),
//                 _cairo_fixed_from_int(-extents.y),
//             );
//             current_block = 4068382217303356765;
//         }
//     }
//     match current_block {
//         4068382217303356765 => {
//             status = _cairo_gstate_save(&mut cr.gstate, &mut cr.gstate_freelist);
//             if !(status as u64 != 0) {
//                 status = _cairo_gstate_redirect_target(cr.gstate, group_surface);
//             }
//         }
//         _ => {}
//     }
//     cairo_surface_destroy(group_surface);
//     return status;
// }
//  fn _cairo_default_context_pop_group(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_pattern_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut group_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
//     let mut group_pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
//     let mut parent_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
//     let mut group_matrix: cairo_matrix_t = cairo_matrix_t {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     if _cairo_gstate_is_group(cr.gstate) == 0 {
//         return _cairo_pattern_create_in_error(CAIRO_STATUS_INVALID_POP_GROUP);
//     }
//     group_surface = _cairo_gstate_get_target(cr.gstate);
//     group_surface = cairo_surface_reference(group_surface);
//     status = _cairo_gstate_restore(&mut cr.gstate, &mut cr.gstate_freelist);
//     if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
//     {} else {
//         __assert_fail(
//             b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo-default-context.c\0" as *const u8 as *const libc::c_char,
//             234 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<
//                 &[u8; 58],
//                 &[libc::c_char; 58],
//             >(b"cairo_pattern_t *_cairo_default_context_pop_group(void *)\0"))
//                 .as_ptr(),
//         );
//     }
//     parent_surface = _cairo_gstate_get_target(cr.gstate);
//     group_pattern = cairo_pattern_create_for_surface(group_surface);
//     status = (*group_pattern).status;
//     if !(status as u64 != 0) {
//         _cairo_gstate_get_matrix(cr.gstate, &mut group_matrix);
//         cairo_pattern_set_matrix(group_pattern, &mut group_matrix);
//         _cairo_path_fixed_translate(
//             (cr.path).as_mut_ptr(),
//             _cairo_fixed_from_int(
//                 ((*parent_surface).device_transform.x0
//                     - (*group_surface).device_transform.x0) as libc::c_int,
//             ),
//             _cairo_fixed_from_int(
//                 ((*parent_surface).device_transform.y0
//                     - (*group_surface).device_transform.y0) as libc::c_int,
//             ),
//         );
//     }
//     cairo_surface_destroy(group_surface);
//     return group_pattern;
// }
//  fn _cairo_default_context_set_source(
//     mut abstract_cr: *mut libc::c_void,
//     mut source: *mut cairo_pattern_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_source(cr.gstate, source);
// }
//  fn _current_source_matches_solid(
//     mut pattern: *const cairo_pattern_t,
//     mut red: libc::c_double,
//     mut green: libc::c_double,
//     mut blue: libc::c_double,
//     mut alpha: libc::c_double,
// ) -> cairo_bool_t {
//     let mut color: cairo_color_t = cairo_color_t {
//         red: 0.,
//         green: 0.,
//         blue: 0.,
//         alpha: 0.,
//         red_short: 0,
//         green_short: 0,
//         blue_short: 0,
//         alpha_short: 0,
//     };
//     if (*pattern).type_0 as libc::c_uint
//         != CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
//     {
//         return 0 as libc::c_int;
//     }
//     red = _cairo_restrict_value(red, 0.0f64, 1.0f64);
//     green = _cairo_restrict_value(green, 0.0f64, 1.0f64);
//     blue = _cairo_restrict_value(blue, 0.0f64, 1.0f64);
//     alpha = _cairo_restrict_value(alpha, 0.0f64, 1.0f64);
//     _cairo_color_init_rgba(&mut color, red, green, blue, alpha);
//     return _cairo_color_equal(
//         &mut color,
//         &mut (*(pattern as *mut cairo_solid_pattern_t)).color,
//     );
// }
//  fn _cairo_default_context_set_source_rgba(
//     mut abstract_cr: *mut libc::c_void,
//     mut red: libc::c_double,
//     mut green: libc::c_double,
//     mut blue: libc::c_double,
//     mut alpha: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     if _current_source_matches_solid((*cr.gstate).source, red, green, blue, alpha)
//         != 0
//     {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     _cairo_default_context_set_source(
//         cr as *mut libc::c_void,
//         &_cairo_pattern_black as *const cairo_solid_pattern_t as *mut cairo_pattern_t,
//     );
//     pattern = cairo_pattern_create_rgba(red, green, blue, alpha);
//     if (*pattern).status as u64 != 0 {
//         return (*pattern).status;
//     }
//     status = _cairo_default_context_set_source(cr as *mut libc::c_void, pattern);
//     cairo_pattern_destroy(pattern);
//     return status;
// }
//  fn _cairo_default_context_set_source_surface(
//     mut abstract_cr: *mut libc::c_void,
//     mut surface: *mut cairo_surface_t,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut pattern: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
//     let mut matrix: cairo_matrix_t = cairo_matrix_t {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     _cairo_default_context_set_source(
//         cr as *mut libc::c_void,
//         &_cairo_pattern_black as *const cairo_solid_pattern_t as *mut cairo_pattern_t,
//     );
//     pattern = cairo_pattern_create_for_surface(surface);
//     if (*pattern).status as u64 != 0 {
//         status = (*pattern).status;
//         cairo_pattern_destroy(pattern);
//         return status;
//     }
//     cairo_matrix_init_translate(&mut matrix, -x, -y);
//     cairo_pattern_set_matrix(pattern, &mut matrix);
//     status = _cairo_default_context_set_source(cr as *mut libc::c_void, pattern);
//     cairo_pattern_destroy(pattern);
//     return status;
// }
//  fn _cairo_default_context_get_source(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_pattern_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_source(cr.gstate);
// }
//  fn _cairo_default_context_set_tolerance(
//     mut abstract_cr: *mut libc::c_void,
//     mut tolerance: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     if tolerance < _cairo_fixed_to_double(1 as libc::c_int) {
//         tolerance = _cairo_fixed_to_double(1 as libc::c_int);
//     }
//     return _cairo_gstate_set_tolerance(cr.gstate, tolerance);
// }
//  fn _cairo_default_context_set_operator(
//     mut abstract_cr: *mut libc::c_void,
//     mut op: cairo_operator_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_operator(cr.gstate, op);
// }
//  fn _cairo_default_context_set_opacity(
//     mut abstract_cr: *mut libc::c_void,
//     mut opacity: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_opacity(cr.gstate, opacity);
// }
//  fn _cairo_default_context_set_antialias(
//     mut abstract_cr: *mut libc::c_void,
//     mut antialias: cairo_antialias_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_antialias(cr.gstate, antialias);
// }
//  fn _cairo_default_context_set_fill_rule(
//     mut abstract_cr: *mut libc::c_void,
//     mut fill_rule: cairo_fill_rule_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_fill_rule(cr.gstate, fill_rule);
// }
//  fn _cairo_default_context_set_line_width(
//     mut abstract_cr: *mut libc::c_void,
//     mut line_width: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_line_width(cr.gstate, line_width);
// }
//  fn _cairo_default_context_set_hairline(
//     mut abstract_cr: *mut libc::c_void,
//     mut set_hairline: cairo_bool_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_hairline(cr.gstate, set_hairline);
// }
//  fn _cairo_default_context_set_line_cap(
//     mut abstract_cr: *mut libc::c_void,
//     mut line_cap: cairo_line_cap_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_line_cap(cr.gstate, line_cap);
// }
//  fn _cairo_default_context_set_line_join(
//     mut abstract_cr: *mut libc::c_void,
//     mut line_join: cairo_line_join_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_line_join(cr.gstate, line_join);
// }
//  fn _cairo_default_context_set_dash(
//     mut abstract_cr: *mut libc::c_void,
//     mut dashes: *const libc::c_double,
//     mut num_dashes: libc::c_int,
//     mut offset: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_dash(cr.gstate, dashes, num_dashes, offset);
// }
//  fn _cairo_default_context_set_miter_limit(
//     mut abstract_cr: *mut libc::c_void,
//     mut limit: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_miter_limit(cr.gstate, limit);
// }
//  fn _cairo_default_context_get_antialias(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_antialias_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_antialias(cr.gstate);
// }
//  fn _cairo_default_context_get_dash(
//     mut abstract_cr: *mut libc::c_void,
//     mut dashes: *mut libc::c_double,
//     mut num_dashes: *mut libc::c_int,
//     mut offset: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_get_dash(cr.gstate, dashes, num_dashes, offset);
// }
//  fn _cairo_default_context_get_fill_rule(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_fill_rule_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_fill_rule(cr.gstate);
// }
//  fn _cairo_default_context_get_line_width(
//     mut abstract_cr: *mut libc::c_void,
// ) -> libc::c_double {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_line_width(cr.gstate);
// }
//  fn _cairo_default_context_get_hairline(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_bool_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_hairline(cr.gstate);
// }
//  fn _cairo_default_context_get_line_cap(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_line_cap_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_line_cap(cr.gstate);
// }
//  fn _cairo_default_context_get_line_join(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_line_join_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_line_join(cr.gstate);
// }
//  fn _cairo_default_context_get_miter_limit(
//     mut abstract_cr: *mut libc::c_void,
// ) -> libc::c_double {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_miter_limit(cr.gstate);
// }
//  fn _cairo_default_context_get_operator(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_operator_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_operator(cr.gstate);
// }
//  fn _cairo_default_context_get_opacity(
//     mut abstract_cr: *mut libc::c_void,
// ) -> libc::c_double {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_opacity(cr.gstate);
// }
//  fn _cairo_default_context_get_tolerance(
//     mut abstract_cr: *mut libc::c_void,
// ) -> libc::c_double {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_tolerance(cr.gstate);
// }
//  fn _cairo_default_context_translate(
//     mut abstract_cr: *mut libc::c_void,
//     mut tx: libc::c_double,
//     mut ty: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_translate(cr.gstate, tx, ty);
// }
//  fn _cairo_default_context_scale(
//     mut abstract_cr: *mut libc::c_void,
//     mut sx: libc::c_double,
//     mut sy: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_scale(cr.gstate, sx, sy);
// }
//  fn _cairo_default_context_rotate(
//     mut abstract_cr: *mut libc::c_void,
//     mut theta: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_rotate(cr.gstate, theta);
// }
//  fn _cairo_default_context_transform(
//     mut abstract_cr: *mut libc::c_void,
//     mut matrix: *const cairo_matrix_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_transform(cr.gstate, matrix);
// }
//  fn _cairo_default_context_set_matrix(
//     mut abstract_cr: *mut libc::c_void,
//     mut matrix: *const cairo_matrix_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_matrix(cr.gstate, matrix);
// }
//  fn _cairo_default_context_set_identity_matrix(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_identity_matrix(cr.gstate);
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_get_matrix(
//     mut abstract_cr: *mut libc::c_void,
//     mut matrix: *mut cairo_matrix_t,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_get_matrix(cr.gstate, matrix);
// }
//  fn _cairo_default_context_user_to_device(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_user_to_device(cr.gstate, x, y);
// }
//  fn _cairo_default_context_user_to_device_distance(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx: *mut libc::c_double,
//     mut dy: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_user_to_device_distance(cr.gstate, dx, dy);
// }
//  fn _cairo_default_context_device_to_user(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_device_to_user(cr.gstate, x, y);
// }
//  fn _cairo_default_context_device_to_user_distance(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx: *mut libc::c_double,
//     mut dy: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_device_to_user_distance(cr.gstate, dx, dy);
// }
//  fn _cairo_default_context_backend_to_user(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_backend_to_user(cr.gstate, x, y);
// }
//  fn _cairo_default_context_backend_to_user_distance(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx: *mut libc::c_double,
//     mut dy: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_backend_to_user_distance(cr.gstate, dx, dy);
// }
//  fn _cairo_default_context_user_to_backend(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_user_to_backend(cr.gstate, x, y);
// }
//  fn _cairo_default_context_user_to_backend_distance(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx: *mut libc::c_double,
//     mut dy: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_user_to_backend_distance(cr.gstate, dx, dy);
// }
//  fn _cairo_default_context_new_path(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_path_fixed_fini((cr.path).as_mut_ptr());
//     _cairo_path_fixed_init((cr.path).as_mut_ptr());
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_new_sub_path(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_path_fixed_new_sub_path((cr.path).as_mut_ptr());
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_move_to(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut x_fixed: cairo_fixed_t = 0;
//     let mut y_fixed: cairo_fixed_t = 0;
//     let mut width: libc::c_double = 0.;
//     _cairo_gstate_user_to_backend(cr.gstate, &mut x, &mut y);
//     width = _cairo_gstate_get_line_width(cr.gstate);
//     x_fixed = _cairo_fixed_from_double_clamped(x, width);
//     y_fixed = _cairo_fixed_from_double_clamped(y, width);
//     return _cairo_path_fixed_move_to((cr.path).as_mut_ptr(), x_fixed, y_fixed);
// }
//  fn _cairo_default_context_line_to(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut x_fixed: cairo_fixed_t = 0;
//     let mut y_fixed: cairo_fixed_t = 0;
//     let mut width: libc::c_double = 0.;
//     _cairo_gstate_user_to_backend(cr.gstate, &mut x, &mut y);
//     width = _cairo_gstate_get_line_width(cr.gstate);
//     x_fixed = _cairo_fixed_from_double_clamped(x, width);
//     y_fixed = _cairo_fixed_from_double_clamped(y, width);
//     return _cairo_path_fixed_line_to((cr.path).as_mut_ptr(), x_fixed, y_fixed);
// }
//  fn _cairo_default_context_curve_to(
//     mut abstract_cr: *mut libc::c_void,
//     mut x1: libc::c_double,
//     mut y1: libc::c_double,
//     mut x2: libc::c_double,
//     mut y2: libc::c_double,
//     mut x3: libc::c_double,
//     mut y3: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut x1_fixed: cairo_fixed_t = 0;
//     let mut y1_fixed: cairo_fixed_t = 0;
//     let mut x2_fixed: cairo_fixed_t = 0;
//     let mut y2_fixed: cairo_fixed_t = 0;
//     let mut x3_fixed: cairo_fixed_t = 0;
//     let mut y3_fixed: cairo_fixed_t = 0;
//     let mut width: libc::c_double = 0.;
//     _cairo_gstate_user_to_backend(cr.gstate, &mut x1, &mut y1);
//     _cairo_gstate_user_to_backend(cr.gstate, &mut x2, &mut y2);
//     _cairo_gstate_user_to_backend(cr.gstate, &mut x3, &mut y3);
//     width = _cairo_gstate_get_line_width(cr.gstate);
//     x1_fixed = _cairo_fixed_from_double_clamped(x1, width);
//     y1_fixed = _cairo_fixed_from_double_clamped(y1, width);
//     x2_fixed = _cairo_fixed_from_double_clamped(x2, width);
//     y2_fixed = _cairo_fixed_from_double_clamped(y2, width);
//     x3_fixed = _cairo_fixed_from_double_clamped(x3, width);
//     y3_fixed = _cairo_fixed_from_double_clamped(y3, width);
//     return _cairo_path_fixed_curve_to(
//         (cr.path).as_mut_ptr(),
//         x1_fixed,
//         y1_fixed,
//         x2_fixed,
//         y2_fixed,
//         x3_fixed,
//         y3_fixed,
//     );
// }
//  fn _cairo_default_context_arc(
//     mut abstract_cr: *mut libc::c_void,
//     mut xc: libc::c_double,
//     mut yc: libc::c_double,
//     mut radius: libc::c_double,
//     mut angle1: libc::c_double,
//     mut angle2: libc::c_double,
//     mut forward: cairo_bool_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     if radius <= 0.0f64 {
//         let mut x_fixed: cairo_fixed_t = 0;
//         let mut y_fixed: cairo_fixed_t = 0;
//         _cairo_gstate_user_to_backend(cr.gstate, &mut xc, &mut yc);
//         x_fixed = _cairo_fixed_from_double(xc);
//         y_fixed = _cairo_fixed_from_double(yc);
//         status = _cairo_path_fixed_line_to((cr.path).as_mut_ptr(), x_fixed, y_fixed);
//         if status as u64 != 0 {
//             return status;
//         }
//         status = _cairo_path_fixed_line_to((cr.path).as_mut_ptr(), x_fixed, y_fixed);
//         if status as u64 != 0 {
//             return status;
//         }
//         return CAIRO_STATUS_SUCCESS;
//     }
//     status = _cairo_default_context_line_to(
//         cr as *mut libc::c_void,
//         xc + radius * cos(angle1),
//         yc + radius * sin(angle1),
//     );
//     if status as u64 != 0 {
//         return status;
//     }
//     if forward != 0 {
//         _cairo_arc_path(&mut cr.base, xc, yc, radius, angle1, angle2);
//     } else {
//         _cairo_arc_path_negative(&mut cr.base, xc, yc, radius, angle1, angle2);
//     }
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_rel_move_to(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx: libc::c_double,
//     mut dy: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut dx_fixed: cairo_fixed_t = 0;
//     let mut dy_fixed: cairo_fixed_t = 0;
//     _cairo_gstate_user_to_backend_distance(cr.gstate, &mut dx, &mut dy);
//     dx_fixed = _cairo_fixed_from_double(dx);
//     dy_fixed = _cairo_fixed_from_double(dy);
//     return _cairo_path_fixed_rel_move_to((cr.path).as_mut_ptr(), dx_fixed, dy_fixed);
// }
//  fn _cairo_default_context_rel_line_to(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx: libc::c_double,
//     mut dy: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut dx_fixed: cairo_fixed_t = 0;
//     let mut dy_fixed: cairo_fixed_t = 0;
//     _cairo_gstate_user_to_backend_distance(cr.gstate, &mut dx, &mut dy);
//     dx_fixed = _cairo_fixed_from_double(dx);
//     dy_fixed = _cairo_fixed_from_double(dy);
//     return _cairo_path_fixed_rel_line_to((cr.path).as_mut_ptr(), dx_fixed, dy_fixed);
// }
//  fn _cairo_default_context_rel_curve_to(
//     mut abstract_cr: *mut libc::c_void,
//     mut dx1: libc::c_double,
//     mut dy1: libc::c_double,
//     mut dx2: libc::c_double,
//     mut dy2: libc::c_double,
//     mut dx3: libc::c_double,
//     mut dy3: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut dx1_fixed: cairo_fixed_t = 0;
//     let mut dy1_fixed: cairo_fixed_t = 0;
//     let mut dx2_fixed: cairo_fixed_t = 0;
//     let mut dy2_fixed: cairo_fixed_t = 0;
//     let mut dx3_fixed: cairo_fixed_t = 0;
//     let mut dy3_fixed: cairo_fixed_t = 0;
//     _cairo_gstate_user_to_backend_distance(cr.gstate, &mut dx1, &mut dy1);
//     _cairo_gstate_user_to_backend_distance(cr.gstate, &mut dx2, &mut dy2);
//     _cairo_gstate_user_to_backend_distance(cr.gstate, &mut dx3, &mut dy3);
//     dx1_fixed = _cairo_fixed_from_double(dx1);
//     dy1_fixed = _cairo_fixed_from_double(dy1);
//     dx2_fixed = _cairo_fixed_from_double(dx2);
//     dy2_fixed = _cairo_fixed_from_double(dy2);
//     dx3_fixed = _cairo_fixed_from_double(dx3);
//     dy3_fixed = _cairo_fixed_from_double(dy3);
//     return _cairo_path_fixed_rel_curve_to(
//         (cr.path).as_mut_ptr(),
//         dx1_fixed,
//         dy1_fixed,
//         dx2_fixed,
//         dy2_fixed,
//         dx3_fixed,
//         dy3_fixed,
//     );
// }
//  fn _cairo_default_context_close_path(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_path_fixed_close_path((cr.path).as_mut_ptr());
// }
//  fn _cairo_default_context_rectangle(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
//     mut width: libc::c_double,
//     mut height: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_default_context_move_to(cr as *mut libc::c_void, x, y);
//     if status as u64 != 0 {
//         return status;
//     }
//     status = _cairo_default_context_rel_line_to(
//         cr as *mut libc::c_void,
//         width,
//         0 as libc::c_int as libc::c_double,
//     );
//     if status as u64 != 0 {
//         return status;
//     }
//     status = _cairo_default_context_rel_line_to(
//         cr as *mut libc::c_void,
//         0 as libc::c_int as libc::c_double,
//         height,
//     );
//     if status as u64 != 0 {
//         return status;
//     }
//     status = _cairo_default_context_rel_line_to(
//         cr as *mut libc::c_void,
//         -width,
//         0 as libc::c_int as libc::c_double,
//     );
//     if status as u64 != 0 {
//         return status;
//     }
//     return _cairo_default_context_close_path(cr as *mut libc::c_void);
// }
//  fn _cairo_default_context_path_extents(
//     mut abstract_cr: *mut libc::c_void,
//     mut x1: *mut libc::c_double,
//     mut y1: *mut libc::c_double,
//     mut x2: *mut libc::c_double,
//     mut y2: *mut libc::c_double,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_path_extents(cr.gstate, (cr.path).as_mut_ptr(), x1, y1, x2, y2);
// }
//  fn _cairo_default_context_has_current_point(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_bool_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return (*(cr.path).as_mut_ptr()).has_current_point() as cairo_bool_t;
// }
//  fn _cairo_default_context_get_current_point(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: *mut libc::c_double,
//     mut y: *mut libc::c_double,
// ) -> cairo_bool_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut x_fixed: cairo_fixed_t = 0;
//     let mut y_fixed: cairo_fixed_t = 0;
//     if _cairo_path_fixed_get_current_point(
//         (cr.path).as_mut_ptr(),
//         &mut x_fixed,
//         &mut y_fixed,
//     ) != 0
//     {
//         *x = _cairo_fixed_to_double(x_fixed);
//         *y = _cairo_fixed_to_double(y_fixed);
//         _cairo_gstate_backend_to_user(cr.gstate, x, y);
//         return 1 as libc::c_int;
//     } else {
//         return 0 as libc::c_int
//     };
// }
//  fn _cairo_default_context_copy_path(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_path_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_path_create((cr.path).as_mut_ptr(), &mut cr.base);
// }
//  fn _cairo_default_context_copy_path_flat(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_path_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_path_create_flat((cr.path).as_mut_ptr(), &mut cr.base);
// }
//  fn _cairo_default_context_append_path(
//     mut abstract_cr: *mut libc::c_void,
//     mut path: *const cairo_path_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_path_append_to_context(path, &mut cr.base);
// }
//  fn _cairo_default_context_paint(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_paint(cr.gstate);
// }
//  fn _cairo_default_context_paint_with_alpha(
//     mut abstract_cr: *mut libc::c_void,
//     mut alpha: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut pattern: cairo_solid_pattern_t = cairo_solid_pattern_t {
//         base: cairo_pattern_t {
//             ref_count: cairo_reference_count_t {
//                 ref_count: 0,
//             },
//             status: CAIRO_STATUS_SUCCESS,
//             user_data: cairo_user_data_array_t {
//                 size: 0,
//                 num_elements: 0,
//                 element_size: 0,
//                 elements: 0 as *const libc::c_char as *mut libc::c_char,
//             },
//             observers: cairo_list_t {
//                 next: 0 as *const _cairo_list as *mut _cairo_list,
//                 prev: 0 as *const _cairo_list as *mut _cairo_list,
//             },
//             type_0: CAIRO_PATTERN_TYPE_SOLID,
//             filter: CAIRO_FILTER_FAST,
//             extend: CAIRO_EXTEND_NONE,
//             has_component_alpha: 0,
//             is_userfont_foreground: 0,
//             matrix: cairo_matrix_t {
//                 xx: 0.,
//                 yx: 0.,
//                 xy: 0.,
//                 yy: 0.,
//                 x0: 0.,
//                 y0: 0.,
//             },
//             opacity: 0.,
//         },
//         color: cairo_color_t {
//             red: 0.,
//             green: 0.,
//             blue: 0.,
//             alpha: 0.,
//             red_short: 0,
//             green_short: 0,
//             blue_short: 0,
//             alpha_short: 0,
//         },
//     };
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     let mut color: cairo_color_t = cairo_color_t {
//         red: 0.,
//         green: 0.,
//         blue: 0.,
//         alpha: 0.,
//         red_short: 0,
//         green_short: 0,
//         blue_short: 0,
//         alpha_short: 0,
//     };
//     if alpha
//         >= 0xff00 as libc::c_int as libc::c_double
//             / 0xffff as libc::c_int as libc::c_double
//     {
//         return _cairo_gstate_paint(cr.gstate);
//     }
//     if alpha <= 0.0f64 && _cairo_operator_bounded_by_mask((*cr.gstate).op) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     _cairo_color_init_rgba(&mut color, 0.0f64, 0.0f64, 0.0f64, alpha);
//     _cairo_pattern_init_solid(&mut pattern, &mut color);
//     status = _cairo_gstate_mask(cr.gstate, &mut pattern.base);
//     _cairo_pattern_fini(&mut pattern.base);
//     return status;
// }
//  fn _cairo_default_context_mask(
//     mut abstract_cr: *mut libc::c_void,
//     mut mask: *mut cairo_pattern_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_mask(cr.gstate, mask);
// }
//  fn _cairo_default_context_stroke_preserve(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_stroke(cr.gstate, (cr.path).as_mut_ptr());
// }
//  fn _cairo_default_context_stroke(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_stroke(cr.gstate, (cr.path).as_mut_ptr());
//     if status as u64 != 0 {
//         return status;
//     }
//     return _cairo_default_context_new_path(cr as *mut libc::c_void);
// }
//  fn _cairo_default_context_in_stroke(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
//     mut inside: *mut cairo_bool_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_in_stroke(
//         cr.gstate,
//         (cr.path).as_mut_ptr(),
//         x,
//         y,
//         inside,
//     );
// }
//  fn _cairo_default_context_stroke_extents(
//     mut abstract_cr: *mut libc::c_void,
//     mut x1: *mut libc::c_double,
//     mut y1: *mut libc::c_double,
//     mut x2: *mut libc::c_double,
//     mut y2: *mut libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_stroke_extents(
//         cr.gstate,
//         (cr.path).as_mut_ptr(),
//         x1,
//         y1,
//         x2,
//         y2,
//     );
// }
//  fn _cairo_default_context_fill_preserve(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_fill(cr.gstate, (cr.path).as_mut_ptr());
// }
//  fn _cairo_default_context_fill(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_fill(cr.gstate, (cr.path).as_mut_ptr());
//     if status as u64 != 0 {
//         return status;
//     }
//     return _cairo_default_context_new_path(cr as *mut libc::c_void);
// }
//  fn _cairo_default_context_in_fill(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
//     mut inside: *mut cairo_bool_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     *inside = _cairo_gstate_in_fill(cr.gstate, (cr.path).as_mut_ptr(), x, y);
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_fill_extents(
//     mut abstract_cr: *mut libc::c_void,
//     mut x1: *mut libc::c_double,
//     mut y1: *mut libc::c_double,
//     mut x2: *mut libc::c_double,
//     mut y2: *mut libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_fill_extents(
//         cr.gstate,
//         (cr.path).as_mut_ptr(),
//         x1,
//         y1,
//         x2,
//         y2,
//     );
// }
//  fn _cairo_default_context_clip_preserve(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_clip(cr.gstate, (cr.path).as_mut_ptr());
// }
//  fn _cairo_default_context_clip(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_clip(cr.gstate, (cr.path).as_mut_ptr());
//     if status as u64 != 0 {
//         return status;
//     }
//     return _cairo_default_context_new_path(cr as *mut libc::c_void);
// }
//  fn _cairo_default_context_in_clip(
//     mut abstract_cr: *mut libc::c_void,
//     mut x: libc::c_double,
//     mut y: libc::c_double,
//     mut inside: *mut cairo_bool_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     *inside = _cairo_gstate_in_clip(cr.gstate, x, y);
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_reset_clip(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_reset_clip(cr.gstate);
// }
//  fn _cairo_default_context_clip_extents(
//     mut abstract_cr: *mut libc::c_void,
//     mut x1: *mut libc::c_double,
//     mut y1: *mut libc::c_double,
//     mut x2: *mut libc::c_double,
//     mut y2: *mut libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     if _cairo_gstate_clip_extents(cr.gstate, x1, y1, x2, y2) == 0 {
//         *x1 = -::std::f32::INFINITY as libc::c_double;
//         *y1 = -::std::f32::INFINITY as libc::c_double;
//         *x2 = ::std::f32::INFINITY as libc::c_double;
//         *y2 = ::std::f32::INFINITY as libc::c_double;
//     }
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_copy_clip_rectangle_list(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_rectangle_list_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_copy_clip_rectangle_list(cr.gstate);
// }
//  fn _cairo_default_context_copy_page(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_copy_page(cr.gstate);
// }
//  fn _cairo_default_context_tag_begin(
//     mut abstract_cr: *mut libc::c_void,
//     mut tag_name: *const libc::c_char,
//     mut attributes: *const libc::c_char,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_tag_begin(cr.gstate, tag_name, attributes);
// }
//  fn _cairo_default_context_tag_end(
//     mut abstract_cr: *mut libc::c_void,
//     mut tag_name: *const libc::c_char,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_tag_end(cr.gstate, tag_name);
// }
//  fn _cairo_default_context_show_page(
//     mut abstract_cr: *mut libc::c_void,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_show_page(cr.gstate);
// }
//  fn _cairo_default_context_set_font_face(
//     mut abstract_cr: *mut libc::c_void,
//     mut font_face: *mut cairo_font_face_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_font_face(cr.gstate, font_face);
// }
//  fn _cairo_default_context_get_font_face(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_font_face_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_get_font_face(cr.gstate, &mut font_face);
//     if status as u64 != 0 {
//         let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
//         return &_cairo_font_face_nil as *const cairo_font_face_t
//             as *mut cairo_font_face_t;
//     }
//     return font_face;
// }
//  fn _cairo_default_context_font_extents(
//     mut abstract_cr: *mut libc::c_void,
//     mut extents: *mut cairo_font_extents_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_get_font_extents(cr.gstate, extents);
// }
//  fn _cairo_default_context_set_font_size(
//     mut abstract_cr: *mut libc::c_void,
//     mut size: libc::c_double,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_font_size(cr.gstate, size);
// }
//  fn _cairo_default_context_set_font_matrix(
//     mut abstract_cr: *mut libc::c_void,
//     mut matrix: *const cairo_matrix_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_set_font_matrix(cr.gstate, matrix);
// }
//  fn _cairo_default_context_get_font_matrix(
//     mut abstract_cr: *mut libc::c_void,
//     mut matrix: *mut cairo_matrix_t,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_get_font_matrix(cr.gstate, matrix);
// }
//  fn _cairo_default_context_set_font_options(
//     mut abstract_cr: *mut libc::c_void,
//     mut options: *const cairo_font_options_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_set_font_options(cr.gstate, options);
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_get_font_options(
//     mut abstract_cr: *mut libc::c_void,
//     mut options: *mut cairo_font_options_t,
// ) {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     _cairo_gstate_get_font_options(cr.gstate, options);
// }
//  fn _cairo_default_context_set_scaled_font(
//     mut abstract_cr: *mut libc::c_void,
//     mut scaled_font: *mut cairo_scaled_font_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut was_previous: cairo_bool_t = 0;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     if scaled_font == (*cr.gstate).scaled_font {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     was_previous = (scaled_font == (*cr.gstate).previous_scaled_font) as libc::c_int;
//     status = _cairo_gstate_set_font_face(cr.gstate, (*scaled_font).font_face);
//     if status as u64 != 0 {
//         return status;
//     }
//     status = _cairo_gstate_set_font_matrix(
//         cr.gstate,
//         &mut (*scaled_font).font_matrix,
//     );
//     if status as u64 != 0 {
//         return status;
//     }
//     _cairo_gstate_set_font_options(cr.gstate, &mut (*scaled_font).options);
//     if was_previous != 0 {
//         let ref mut fresh3 = (*cr.gstate).scaled_font;
//         *fresh3 = cairo_scaled_font_reference(scaled_font);
//     }
//     return CAIRO_STATUS_SUCCESS;
// }
//  fn _cairo_default_context_get_scaled_font(
//     mut abstract_cr: *mut libc::c_void,
// ) -> *mut cairo_scaled_font_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_get_scaled_font(cr.gstate, &mut scaled_font);
//     if status as u64 != 0 {
//         return _cairo_scaled_font_create_in_error(status);
//     }
//     return scaled_font;
// }
//  fn _cairo_default_context_glyphs(
//     mut abstract_cr: *mut libc::c_void,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
//     mut info: *mut cairo_glyph_text_info_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_show_text_glyphs(cr.gstate, glyphs, num_glyphs, info);
// }
//  fn _cairo_default_context_glyph_path(
//     mut abstract_cr: *mut libc::c_void,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_glyph_path(
//         cr.gstate,
//         glyphs,
//         num_glyphs,
//         (cr.path).as_mut_ptr(),
//     );
// }
//  fn _cairo_default_context_glyph_extents(
//     mut abstract_cr: *mut libc::c_void,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
//     mut extents: *mut cairo_text_extents_t,
// ) -> cairo_status_t {
//     let mut cr: *mut cairo_default_context_t = abstract_cr
//         as *mut cairo_default_context_t;
//     return _cairo_gstate_glyph_extents(cr.gstate, glyphs, num_glyphs, extents);
// }
// static mut _cairo_default_context_backend: cairo_backend_t = unsafe {
//     {
//         let mut init = _cairo_backend {
//             type_0: CAIRO_TYPE_DEFAULT,
//             destroy: Some(
//                 _cairo_default_context_destroy
//                     as  fn(*mut libc::c_void) -> (),
//             ),
//             get_original_target: Some(
//                 _cairo_default_context_get_original_target
//                     as  fn(*mut libc::c_void) -> *mut cairo_surface_t,
//             ),
//             get_current_target: Some(
//                 _cairo_default_context_get_current_target
//                     as  fn(*mut libc::c_void) -> *mut cairo_surface_t,
//             ),
//             save: Some(
//                 _cairo_default_context_save
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             restore: Some(
//                 _cairo_default_context_restore
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             push_group: Some(
//                 _cairo_default_context_push_group
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_content_t,
//                     ) -> cairo_status_t,
//             ),
//             pop_group: Some(
//                 _cairo_default_context_pop_group
//                     as  fn(*mut libc::c_void) -> *mut cairo_pattern_t,
//             ),
//             set_source_rgba: Some(
//                 _cairo_default_context_set_source_rgba
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_source_surface: Some(
//                 _cairo_default_context_set_source_surface
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_surface_t,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_source: Some(
//                 _cairo_default_context_set_source
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_pattern_t,
//                     ) -> cairo_status_t,
//             ),
//             get_source: Some(
//                 _cairo_default_context_get_source
//                     as  fn(*mut libc::c_void) -> *mut cairo_pattern_t,
//             ),
//             set_antialias: Some(
//                 _cairo_default_context_set_antialias
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_antialias_t,
//                     ) -> cairo_status_t,
//             ),
//             set_dash: Some(
//                 _cairo_default_context_set_dash
//                     as  fn(
//                         *mut libc::c_void,
//                         *const libc::c_double,
//                         libc::c_int,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_fill_rule: Some(
//                 _cairo_default_context_set_fill_rule
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_fill_rule_t,
//                     ) -> cairo_status_t,
//             ),
//             set_line_cap: Some(
//                 _cairo_default_context_set_line_cap
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_line_cap_t,
//                     ) -> cairo_status_t,
//             ),
//             set_line_join: Some(
//                 _cairo_default_context_set_line_join
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_line_join_t,
//                     ) -> cairo_status_t,
//             ),
//             set_line_width: Some(
//                 _cairo_default_context_set_line_width
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_hairline: Some(
//                 _cairo_default_context_set_hairline
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_bool_t,
//                     ) -> cairo_status_t,
//             ),
//             set_miter_limit: Some(
//                 _cairo_default_context_set_miter_limit
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_opacity: Some(
//                 _cairo_default_context_set_opacity
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_operator: Some(
//                 _cairo_default_context_set_operator
//                     as  fn(
//                         *mut libc::c_void,
//                         cairo_operator_t,
//                     ) -> cairo_status_t,
//             ),
//             set_tolerance: Some(
//                 _cairo_default_context_set_tolerance
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             get_antialias: Some(
//                 _cairo_default_context_get_antialias
//                     as  fn(*mut libc::c_void) -> cairo_antialias_t,
//             ),
//             get_dash: Some(
//                 _cairo_default_context_get_dash
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_int,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             get_fill_rule: Some(
//                 _cairo_default_context_get_fill_rule
//                     as  fn(*mut libc::c_void) -> cairo_fill_rule_t,
//             ),
//             get_line_cap: Some(
//                 _cairo_default_context_get_line_cap
//                     as  fn(*mut libc::c_void) -> cairo_line_cap_t,
//             ),
//             get_line_join: Some(
//                 _cairo_default_context_get_line_join
//                     as  fn(*mut libc::c_void) -> cairo_line_join_t,
//             ),
//             get_line_width: Some(
//                 _cairo_default_context_get_line_width
//                     as  fn(*mut libc::c_void) -> libc::c_double,
//             ),
//             get_hairline: Some(
//                 _cairo_default_context_get_hairline
//                     as  fn(*mut libc::c_void) -> cairo_bool_t,
//             ),
//             get_miter_limit: Some(
//                 _cairo_default_context_get_miter_limit
//                     as  fn(*mut libc::c_void) -> libc::c_double,
//             ),
//             get_opacity: Some(
//                 _cairo_default_context_get_opacity
//                     as  fn(*mut libc::c_void) -> libc::c_double,
//             ),
//             get_operator: Some(
//                 _cairo_default_context_get_operator
//                     as  fn(*mut libc::c_void) -> cairo_operator_t,
//             ),
//             get_tolerance: Some(
//                 _cairo_default_context_get_tolerance
//                     as  fn(*mut libc::c_void) -> libc::c_double,
//             ),
//             translate: Some(
//                 _cairo_default_context_translate
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             scale: Some(
//                 _cairo_default_context_scale
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             rotate: Some(
//                 _cairo_default_context_rotate
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             transform: Some(
//                 _cairo_default_context_transform
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_matrix_t,
//                     ) -> cairo_status_t,
//             ),
//             set_matrix: Some(
//                 _cairo_default_context_set_matrix
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_matrix_t,
//                     ) -> cairo_status_t,
//             ),
//             set_identity_matrix: Some(
//                 _cairo_default_context_set_identity_matrix
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             get_matrix: Some(
//                 _cairo_default_context_get_matrix
//                     as  fn(*mut libc::c_void, *mut cairo_matrix_t) -> (),
//             ),
//             user_to_device: Some(
//                 _cairo_default_context_user_to_device
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             user_to_device_distance: Some(
//                 _cairo_default_context_user_to_device_distance
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             device_to_user: Some(
//                 _cairo_default_context_device_to_user
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             device_to_user_distance: Some(
//                 _cairo_default_context_device_to_user_distance
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             user_to_backend: Some(
//                 _cairo_default_context_user_to_backend
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             user_to_backend_distance: Some(
//                 _cairo_default_context_user_to_backend_distance
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             backend_to_user: Some(
//                 _cairo_default_context_backend_to_user
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             backend_to_user_distance: Some(
//                 _cairo_default_context_backend_to_user_distance
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             new_path: Some(
//                 _cairo_default_context_new_path
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             new_sub_path: Some(
//                 _cairo_default_context_new_sub_path
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             move_to: Some(
//                 _cairo_default_context_move_to
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             rel_move_to: Some(
//                 _cairo_default_context_rel_move_to
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             line_to: Some(
//                 _cairo_default_context_line_to
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             rel_line_to: Some(
//                 _cairo_default_context_rel_line_to
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             curve_to: Some(
//                 _cairo_default_context_curve_to
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             rel_curve_to: Some(
//                 _cairo_default_context_rel_curve_to
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             arc_to: None,
//             rel_arc_to: None,
//             close_path: Some(
//                 _cairo_default_context_close_path
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             arc: Some(
//                 _cairo_default_context_arc
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         cairo_bool_t,
//                     ) -> cairo_status_t,
//             ),
//             rectangle: Some(
//                 _cairo_default_context_rectangle
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             path_extents: Some(
//                 _cairo_default_context_path_extents
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> (),
//             ),
//             has_current_point: Some(
//                 _cairo_default_context_has_current_point
//                     as  fn(*mut libc::c_void) -> cairo_bool_t,
//             ),
//             get_current_point: Some(
//                 _cairo_default_context_get_current_point
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> cairo_bool_t,
//             ),
//             copy_path: Some(
//                 _cairo_default_context_copy_path
//                     as  fn(*mut libc::c_void) -> *mut cairo_path_t,
//             ),
//             copy_path_flat: Some(
//                 _cairo_default_context_copy_path_flat
//                     as  fn(*mut libc::c_void) -> *mut cairo_path_t,
//             ),
//             append_path: Some(
//                 _cairo_default_context_append_path
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_path_t,
//                     ) -> cairo_status_t,
//             ),
//             stroke_to_path: None,
//             clip: Some(
//                 _cairo_default_context_clip
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             clip_preserve: Some(
//                 _cairo_default_context_clip_preserve
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             in_clip: Some(
//                 _cairo_default_context_in_clip
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         *mut cairo_bool_t,
//                     ) -> cairo_status_t,
//             ),
//             clip_extents: Some(
//                 _cairo_default_context_clip_extents
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             reset_clip: Some(
//                 _cairo_default_context_reset_clip
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             clip_copy_rectangle_list: Some(
//                 _cairo_default_context_copy_clip_rectangle_list
//                     as  fn(
//                         *mut libc::c_void,
//                     ) -> *mut cairo_rectangle_list_t,
//             ),
//             paint: Some(
//                 _cairo_default_context_paint
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             paint_with_alpha: Some(
//                 _cairo_default_context_paint_with_alpha
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             mask: Some(
//                 _cairo_default_context_mask
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_pattern_t,
//                     ) -> cairo_status_t,
//             ),
//             stroke: Some(
//                 _cairo_default_context_stroke
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             stroke_preserve: Some(
//                 _cairo_default_context_stroke_preserve
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             in_stroke: Some(
//                 _cairo_default_context_in_stroke
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         *mut cairo_bool_t,
//                     ) -> cairo_status_t,
//             ),
//             stroke_extents: Some(
//                 _cairo_default_context_stroke_extents
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             fill: Some(
//                 _cairo_default_context_fill
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             fill_preserve: Some(
//                 _cairo_default_context_fill_preserve
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             in_fill: Some(
//                 _cairo_default_context_in_fill
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                         libc::c_double,
//                         *mut cairo_bool_t,
//                     ) -> cairo_status_t,
//             ),
//             fill_extents: Some(
//                 _cairo_default_context_fill_extents
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                         *mut libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_font_face: Some(
//                 _cairo_default_context_set_font_face
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_font_face_t,
//                     ) -> cairo_status_t,
//             ),
//             get_font_face: Some(
//                 _cairo_default_context_get_font_face
//                     as  fn(*mut libc::c_void) -> *mut cairo_font_face_t,
//             ),
//             set_font_size: Some(
//                 _cairo_default_context_set_font_size
//                     as  fn(
//                         *mut libc::c_void,
//                         libc::c_double,
//                     ) -> cairo_status_t,
//             ),
//             set_font_matrix: Some(
//                 _cairo_default_context_set_font_matrix
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_matrix_t,
//                     ) -> cairo_status_t,
//             ),
//             get_font_matrix: Some(
//                 _cairo_default_context_get_font_matrix
//                     as  fn(*mut libc::c_void, *mut cairo_matrix_t) -> (),
//             ),
//             set_font_options: Some(
//                 _cairo_default_context_set_font_options
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_font_options_t,
//                     ) -> cairo_status_t,
//             ),
//             get_font_options: Some(
//                 _cairo_default_context_get_font_options
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_font_options_t,
//                     ) -> (),
//             ),
//             set_scaled_font: Some(
//                 _cairo_default_context_set_scaled_font
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_scaled_font_t,
//                     ) -> cairo_status_t,
//             ),
//             get_scaled_font: Some(
//                 _cairo_default_context_get_scaled_font
//                     as  fn(
//                         *mut libc::c_void,
//                     ) -> *mut cairo_scaled_font_t,
//             ),
//             font_extents: Some(
//                 _cairo_default_context_font_extents
//                     as  fn(
//                         *mut libc::c_void,
//                         *mut cairo_font_extents_t,
//                     ) -> cairo_status_t,
//             ),
//             glyphs: Some(
//                 _cairo_default_context_glyphs
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_glyph_t,
//                         libc::c_int,
//                         *mut cairo_glyph_text_info_t,
//                     ) -> cairo_status_t,
//             ),
//             glyph_path: Some(
//                 _cairo_default_context_glyph_path
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_glyph_t,
//                         libc::c_int,
//                     ) -> cairo_status_t,
//             ),
//             glyph_extents: Some(
//                 _cairo_default_context_glyph_extents
//                     as  fn(
//                         *mut libc::c_void,
//                         *const cairo_glyph_t,
//                         libc::c_int,
//                         *mut cairo_text_extents_t,
//                     ) -> cairo_status_t,
//             ),
//             copy_page: Some(
//                 _cairo_default_context_copy_page
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             show_page: Some(
//                 _cairo_default_context_show_page
//                     as  fn(*mut libc::c_void) -> cairo_status_t,
//             ),
//             tag_begin: Some(
//                 _cairo_default_context_tag_begin
//                     as  fn(
//                         *mut libc::c_void,
//                         *const libc::c_char,
//                         *const libc::c_char,
//                     ) -> cairo_status_t,
//             ),
//             tag_end: Some(
//                 _cairo_default_context_tag_end
//                     as  fn(
//                         *mut libc::c_void,
//                         *const libc::c_char,
//                     ) -> cairo_status_t,
//             ),
//         };
//         init
//     }
// };

#[derive(Copy, Clone)]
pub struct DefaultContext {
    pub base: Cairo,
    pub gstate: *mut cairo_gstate_t,
    pub gstate_tail: [cairo_gstate_t; 2],
    pub gstate_freelist: *mut cairo_gstate_t,
    pub path: [cairo_path_fixed_t; 1],
}

pub fn _cairo_default_context_create(mut target: *mut libc::c_void) -> *mut cairo_t {
    let mut cr: *mut cairo_default_context_t = 0 as *mut cairo_default_context_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    cr = _freed_pool_get(&mut context_pool) as *mut cairo_default_context_t;
    if cr.is_null() {
        cr = (if ::std::mem::size_of::<cairo_default_context_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_default_context_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_default_context_t;
        if cr.is_null() {
            return _cairo_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
        }
    }
    status = _cairo_default_context_init(cr, target);
    if status as u64 != 0 {
        _freed_pool_put(&mut context_pool, cr as *mut libc::c_void);
        return _cairo_create_in_error(status);
    }
    return &mut cr.base;
}

pub fn _cairo_default_context_init(
    mut cr: *mut cairo_default_context_t,
    mut target: *mut libc::c_void,
) -> cairo_status_t {
    _cairo_init(&mut cr.base, &_cairo_default_context_backend);
    _cairo_path_fixed_init((cr.path).as_mut_ptr());
    let ref mut fresh4 = cr.gstate;
    *fresh4 = &mut *(cr.gstate_tail)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut cairo_gstate_t;
    let ref mut fresh5 = cr.gstate_freelist;
    *fresh5 = &mut *(cr.gstate_tail)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) as *mut cairo_gstate_t;
    let ref mut fresh6 = cr.gstate_tail[1 as libc::c_int as usize].next;
    *fresh6 = 0 as *mut _cairo_gstate;
    return _cairo_gstate_init(cr.gstate, target as *mut cairo_surface_t);
}

pub fn _cairo_default_context_fini(mut cr: *mut cairo_default_context_t) {
    while cr.gstate
        != &mut *(cr.gstate_tail)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cairo_gstate_t
    {
        if _cairo_gstate_restore(&mut cr.gstate, &mut cr.gstate_freelist) as u64 != 0 {
            break;
        }
    }
    _cairo_gstate_fini(cr.gstate);
    let ref mut fresh1 = cr.gstate_freelist;
    *fresh1 = (*cr.gstate_freelist).next;
    while !(cr.gstate_freelist).is_null() {
        let mut gstate: *mut cairo_gstate_t = cr.gstate_freelist;
        let ref mut fresh2 = cr.gstate_freelist;
        *fresh2 = (*gstate).next;
        free(gstate as *mut libc::c_void);
    }
    _cairo_path_fixed_fini((cr.path).as_mut_ptr());
    _cairo_fini(&mut cr.base);
}

use crate::cairo::Backend;

impl Backend for DefaultContext {

}