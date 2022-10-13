// use ::c2rust_bitfields;
//     pub type _cairo_image_surface;
//     pub type _cairo_damage;
//     pub type _cairo_device;
//     pub type _cairo_region;
//     pub type _cairo_hash_table;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
//     fn fmod(_: f64, _: f64) -> f64;
//     fn _cairo_error(status: Result<()>) -> Result<()>;
//     fn _cairo_user_data_array_get_data(
//         array: *mut cairo_user_data_array_t,
//         key: *const cairo_user_data_key_t,
//     ) -> *mut libc::c_void;
//     fn _cairo_user_data_array_set_data(
//         array: *mut cairo_user_data_array_t,
//         key: *const cairo_user_data_key_t,
//         user_data: *mut libc::c_void,
//         destroy: cairo_destroy_func_t,
//     ) -> Result<()>;
//     fn _cairo_rectangle_list_create_in_error(
//         status: Result<()>,
//     ) -> *mut cairo_rectangle_list_t;
//     fn cairo_glyph_free(glyphs: *mut cairo_glyph_t);
//     fn cairo_text_cluster_free(clusters: *mut cairo_text_cluster_t);
//     fn cairo_font_options_status(options: *mut cairo_font_options_t) -> Result<()>;
//     fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
//     static _cairo_font_face_nil: cairo_font_face_t;
//     fn _cairo_scaled_font_create_in_error(
//         status: Result<()>,
//     ) -> *mut cairo_scaled_font_t;
//     fn _cairo_surface_create_in_error(status: Result<()>) -> *mut Surface;
//     fn _cairo_utf8_to_ucs4(
//         str: *const libc::c_char,
//         len: isize,
//         result: *mut *mut uint32_t,
//         items_written: *mut isize,
//     ) -> Result<()>;
//     fn _cairo_validate_text_clusters(
//         utf8: *const libc::c_char,
//         utf8_len: isize,
//         glyphs: *const cairo_glyph_t,
//         num_glyphs: isize,
//         clusters: *const cairo_text_cluster_t,
//         num_clusters: isize,
//         cluster_flags: cairo_text_cluster_flags_t,
//     ) -> Result<()>;
//     fn __assert_fail(
//         __assertion: *const libc::c_char,
//         __file: *const libc::c_char,
//         __line: usize,
//         __function: *const libc::c_char,
//     ) -> !;
//     fn _cairo_user_data_array_init(array: *mut cairo_user_data_array_t);
//     fn _cairo_user_data_array_fini(array: *mut cairo_user_data_array_t);
//     fn cairo_pattern_destroy(pattern: *mut Pattern);
//     fn cairo_pattern_create_for_surface(
//         surface: *mut Surface,
//     ) -> *mut Pattern;
//     fn cairo_toy_font_face_create(
//         family: *const libc::c_char,
//         slant: cairo_font_slant_t,
//         weight: cairo_font_weight_t,
//     ) -> *mut cairo_font_face_t;
//     fn cairo_scaled_font_text_to_glyphs(
//         scaled_font: *mut cairo_scaled_font_t,
//         x: f64,
//         y: f64,
//         utf8: *const libc::c_char,
//         utf8_len: isize,
//         glyphs: *mut *mut cairo_glyph_t,
//         num_glyphs: *mut isize,
//         clusters: *mut *mut cairo_text_cluster_t,
//         num_clusters: *mut isize,
//         cluster_flags: *mut cairo_text_cluster_flags_t,
//     ) -> Result<()>;
//     fn cairo_surface_has_show_text_glyphs(surface: *mut Surface) -> cairo_bool_t;
//     fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
//     fn cairo_matrix_init_translate(
//         matrix: *mut cairo_matrix_t,
//         tx: f64,
//         ty: f64,
//     );
//     fn cairo_pattern_set_matrix(
//         pattern: *mut Pattern,
//         matrix: &Matrix,
//     );
//     fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
//     fn _cairo_path_create_in_error(status: Result<()>) -> *mut cairo_path_t;
//     fn _cairo_pattern_create_in_error(status: Result<()>) -> *mut Pattern;

// pub type __int16_t = libc::c_short;
// pub type __int32_t = isize;
// pub type __uint32_t = usize;
// pub type int16_t = i16;
// pub type int32_t = i32;

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo {
//     pub ref_count: cairo_reference_count_t,
//     pub status: Result<()>,
//     pub user_data: cairo_user_data_array_t,
//     pub backend: *const cairo_backend_t,
// }

// pub trait Backend {
//     fn get_type() -> BackendType;

//     fn get_original_target(&mut self) -> &mut Surface;
//     fn get_current_target(&mut self) -> &mut Surface;

//     fn save(&mut self) -> Result<()>;
//     fn restore(&mut self) -> Result<()>;

//     fn push_group(&mut self, content: Content) -> Result<()>;
//     fn pop_group(&mut self) -> *mut Pattern;

//     fn set_source_rgba(&mut self, red: f64, green: f64, blue: f64, alpha: f64) -> Result<()>;
//     fn set_source_surface(&mut self, surface: *mut Surface, x: f64, y: f64) -> Result<()>;
//     fn set_source(&mut self, _: *mut Pattern) -> Result<()>;
//     fn get_source(&mut self) -> *mut Pattern;

//     fn set_antialias(&mut self, _: cairo_antialias_t) -> Result<()>;
//     fn set_dash(&mut self, _: *const f64, _: isize, _: f64) -> Result<()>;
//     fn set_fill_rule(&mut self, _: cairo_fill_rule_t) -> Result<()>;
//     fn set_line_cap(&mut self, _: cairo_line_cap_t) -> Result<()>;
//     fn set_line_join(&mut self, _: cairo_line_join_t) -> Result<()>;
//     fn set_line_width(&mut self, _: f64) -> Result<()>;
//     fn set_hairline(&mut self, _: cairo_bool_t) -> Result<()>;
//     fn set_miter_limit(&mut self, _: f64) -> Result<()>;
//     fn set_opacity(&mut self, _: f64) -> Result<()>;
//     fn set_operator(&mut self, _: cairo_operator_t) -> Result<()>;
//     fn set_tolerance(&mut self, _: f64) -> Result<()>;

//     fn get_antialias(&mut self) -> cairo_antialias_t;
//     fn get_dash(&mut self, _: *mut f64, _: *mut isize, _: *mut f64) -> ();
//     fn get_fill_rule(&mut self) -> cairo_fill_rule_t;
//     fn get_line_cap(&mut self) -> cairo_line_cap_t;
//     fn get_line_join(&mut self) -> cairo_line_join_t;
//     fn get_line_width(&mut self) -> f64;
//     fn get_hairline(&mut self) -> cairo_bool_t;
//     fn get_miter_limit(&mut self) -> f64;
//     fn get_opacity(&mut self) -> f64;
//     fn get_operator(&mut self) -> cairo_operator_t;
//     fn get_tolerance(&mut self) -> f64;

//     fn translate(&mut self, _: f64, _: f64) -> Result<()>;
//     fn scale(&mut self, _: f64, _: f64) -> Result<()>;
//     fn rotate(&mut self, _: f64) -> Result<()>;
//     fn transform(&mut self, _: &Matrix) -> Result<()>;
//     fn set_matrix(&mut self, _: &Matrix) -> Result<()>;
//     fn set_identity_matrix(&mut self) -> Result<()>;
//     fn get_matrix(&mut self, _: &Matrix) -> ();

//     fn user_to_device(&mut self, _: *mut f64, _: *mut f64) -> ();
//     fn user_to_device_distance(&mut self, _: *mut f64, _: *mut f64) -> ();
//     fn device_to_user(&mut self, _: *mut f64, _: *mut f64) -> ();
//     fn device_to_user_distance(&mut self, _: *mut f64, _: *mut f64) -> ();

//     fn user_to_backend(&mut self, _: *mut f64, _: *mut f64) -> ();
//     fn user_to_backend_distance(&mut self, _: *mut f64, _: *mut f64) -> ();
//     fn backend_to_user(&mut self, _: *mut f64, _: *mut f64) -> ();
//     fn backend_to_user_distance(&mut self, _: *mut f64, _: *mut f64) -> ();

//     fn new_path(&mut self) -> Result<()>;
//     fn new_sub_path(&mut self) -> Result<()>;
//     fn move_to(&mut self, _: f64, _: f64) -> Result<()>;
//     fn rel_move_to(&mut self, _: f64, _: f64) -> Result<()>;
//     fn line_to(&mut self, _: f64, _: f64) -> Result<()>;
//     fn rel_line_to(&mut self, _: f64, _: f64) -> Result<()>;
//     fn curve_to(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64, _: f64) -> Result<()>;
//     fn rel_curve_to(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64, _: f64) -> Result<()>;
//     fn arc_to(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64) -> Result<()>;
//     fn rel_arc_to(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64) -> Result<()>;
//     fn close_path(&mut self) -> Result<()>;

//     fn arc(&mut self, _: f64, _: f64, _: f64, _: f64, _: f64, _: cairo_bool_t) -> Result<()>;
//     fn rectangle(&mut self, _: f64, _: f64, _: f64, _: f64) -> Result<()>;

//     fn path_extents(&mut self, _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64) -> ();
//     fn has_current_point(&mut self) -> cairo_bool_t;
//     fn get_current_point(&mut self, _: *mut f64, _: *mut f64) -> cairo_bool_t;

//     fn copy_path(&mut self) -> *mut cairo_path_t;
//     fn copy_path_flat(&mut self) -> *mut cairo_path_t;
//     fn append_path(&mut self, _: *const cairo_path_t) -> Result<()>;

//     fn stroke_to_path(&mut self) -> Result<()>;

//     fn clip(&mut self) -> Result<()>;
//     fn clip_preserve(&mut self) -> Result<()>;
//     fn in_clip(&mut self, _: f64, _: f64, _: *mut cairo_bool_t) -> Result<()>;
//     fn clip_extents(&mut self, _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64) -> Result<()>;
//     fn reset_clip(&mut self) -> Result<()>;
//     fn clip_copy_rectangle_list(&mut self) -> *mut cairo_rectangle_list_t;

//     fn paint(&mut self) -> Result<()>;
//     fn paint_with_alpha(&mut self, _: f64) -> Result<()>;
//     fn mask(&mut self, _: *mut Pattern) -> Result<()>;

//     fn stroke(&mut self) -> Result<()>;
//     fn stroke_preserve(&mut self) -> Result<()>;
//     fn in_stroke(&mut self, _: f64, _: f64, _: *mut cairo_bool_t) -> Result<()>;
//     fn stroke_extents(&mut self, _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64) -> Result<()>;

//     fn fill(&mut self) -> Result<()>;
//     fn fill_preserve(&mut self) -> Result<()>;
//     fn in_fill(&mut self, _: f64, _: f64, _: *mut cairo_bool_t) -> Result<()>;
//     fn fill_extents(&mut self, _: *mut f64, _: *mut f64, _: *mut f64, _: *mut f64) -> Result<()>;

//     fn set_font_face(&mut self, _: *mut cairo_font_face_t) -> Result<()>;
//     fn get_font_face(&mut self) -> *mut cairo_font_face_t;
//     fn set_font_size(&mut self, _: f64) -> Result<()>;
//     fn set_font_matrix(&mut self, _: &Matrix) -> Result<()>;
//     fn get_font_matrix(&mut self, _: *mut cairo_matrix_t) -> ();
//     fn set_font_options(&mut self, _: *const cairo_font_options_t) -> Result<()>;
//     fn get_font_options(&mut self, _: *mut cairo_font_options_t) -> ();
//     fn set_scaled_font(&mut self, _: *mut cairo_scaled_font_t) -> Result<()>;
//     fn get_scaled_font(&mut self) -> *mut cairo_scaled_font_t;
//     fn font_extents(&mut self, _: *mut cairo_font_extents_t) -> Result<()>;

//     fn glyphs(
//         &mut self,
//         _: *const cairo_glyph_t,
//         _: isize,
//         _: *mut cairo_glyph_text_info_t,
//     ) -> Result<()>;
//     fn glyph_path(&mut self, _: *const cairo_glyph_t, _: isize) -> Result<()>;
//     fn glyph_extents(
//         &mut self,
//         _: *const cairo_glyph_t,
//         _: isize,
//         _: *mut cairo_text_extents_t,
//     ) -> Result<()>;

//     fn copy_page(&mut self) -> Result<()>;
//     fn show_page(&mut self) -> Result<()>;

//     fn tag_begin(&mut self, _: *const libc::c_char, _: *const libc::c_char) -> Result<()>;
//     fn tag_end(&mut self, _: *const libc::c_char) -> Result<()>;
// }

pub type Result<T> = std::result::Result<T, crate::error::Error>;

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct cairo_text_extents_t {
//     pub x_bearing: f64,
//     pub y_bearing: f64,
//     pub width: f64,
//     pub height: f64,
//     pub x_advance: f64,
//     pub y_advance: f64,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct cairo_glyph_t {
//     pub index: libc::c_ulong,
//     pub x: f64,
//     pub y: f64,
// }
// pub type cairo_glyph_text_info_t = _cairo_glyph_text_info;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_glyph_text_info {
//     pub utf8: *const libc::c_char,
//     pub utf8_len: isize,
//     pub clusters: *const cairo_text_cluster_t,
//     pub num_clusters: isize,
//     pub cluster_flags: cairo_text_cluster_flags_t,
// }
// pub type cairo_text_cluster_flags_t = _cairo_text_cluster_flags;
// pub type _cairo_text_cluster_flags = usize;
// pub const CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: _cairo_text_cluster_flags = 1;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct cairo_text_cluster_t {
//     pub num_bytes: isize,
//     pub num_glyphs: isize,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct cairo_font_extents_t {
//     pub ascent: f64,
//     pub descent: f64,
//     pub height: f64,
//     pub max_x_advance: f64,
//     pub max_y_advance: f64,
// }
// pub type cairo_scaled_font_t = _cairo_scaled_font;
// #[derive(Copy, Clone, BitfieldStruct)]
// #[repr(C)]
// pub struct _cairo_scaled_font {
//     pub hash_entry: cairo_hash_entry_t,
//     pub status: Result<()>,
//     pub ref_count: cairo_reference_count_t,
//     pub user_data: cairo_user_data_array_t,
//     pub original_font_face: *mut cairo_font_face_t,
//     pub font_face: *mut cairo_font_face_t,
//     pub font_matrix: cairo_matrix_t,
//     pub ctm: cairo_matrix_t,
//     pub options: cairo_font_options_t,
//     #[bitfield(name = "placeholder", ty = "usize", bits = "0..=0")]
//     #[bitfield(name = "holdover", ty = "usize", bits = "1..=1")]
//     #[bitfield(name = "finished", ty = "usize", bits = "2..=2")]
//     pub placeholder_holdover_finished: [u8; 1],
//     #[bitfield(padding)]
//     pub c2rust_padding: [u8; 7],
//     pub scale: cairo_matrix_t,
//     pub scale_inverse: cairo_matrix_t,
//     pub max_scale: f64,
//     pub extents: cairo_font_extents_t,
//     pub fs_extents: cairo_font_extents_t,
//     pub mutex: cairo_recursive_mutex_t,
//     pub glyphs: *mut cairo_hash_table_t,
//     pub glyph_pages: cairo_list_t,
//     pub cache_frozen: cairo_bool_t,
//     pub global_cache_frozen: cairo_bool_t,
//     pub recording_surfaces_to_free: cairo_array_t,
//     pub dev_privates: cairo_list_t,
//     pub backend: *const cairo_scaled_font_backend_t,
//     pub link: cairo_list_t,
// }
// pub type cairo_list_t = _cairo_list;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_list {
//     pub next: *mut _cairo_list,
//     pub prev: *mut _cairo_list,
// }
// pub type cairo_scaled_font_backend_t = _cairo_scaled_font_backend;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_scaled_font_backend {
//     pub type_0: cairo_font_type_t,
//     pub fini: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
//     pub scaled_glyph_init: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut cairo_scaled_glyph_t,
//             cairo_scaled_glyph_info_t,
//             *const cairo_color_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub text_to_glyphs: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             f64,
//             f64,
//             *const libc::c_char,
//             isize,
//             *mut *mut cairo_glyph_t,
//             *mut isize,
//             *mut *mut cairo_text_cluster_t,
//             *mut isize,
//             *mut cairo_text_cluster_flags_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub ucs4_to_index: Option::<
//         unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_ulong,
//     >,
//     pub load_truetype_table: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             libc::c_ulong,
//             libc::c_long,
//             *mut libc::c_uchar,
//             *mut libc::c_ulong,
//         ) -> cairo_int_status_t,
//     >,
//     pub index_to_ucs4: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             libc::c_ulong,
//             *mut uint32_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub is_synthetic: Option::<
//         unsafe extern "C" fn(*mut libc::c_void, *mut cairo_bool_t) -> cairo_int_status_t,
//     >,
//     pub index_to_glyph_name: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut *mut libc::c_char,
//             isize,
//             libc::c_ulong,
//             *mut libc::c_ulong,
//         ) -> cairo_int_status_t,
//     >,
//     pub load_type1_data: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             libc::c_long,
//             *mut libc::c_uchar,
//             *mut libc::c_ulong,
//         ) -> cairo_int_status_t,
//     >,
//     pub has_color_glyphs: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
//     >,
// }
// pub type cairo_int_status_t = _cairo_int_status;
// pub type _cairo_int_status = usize;
// pub const CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN: _cairo_int_status = 105;
// pub const CAIRO_INT_STATUS_IMAGE_FALLBACK: _cairo_int_status = 104;
// pub const CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY: _cairo_int_status = 103;
// pub const CAIRO_INT_STATUS_NOTHING_TO_DO: _cairo_int_status = 102;
// pub const CAIRO_INT_STATUS_DEGENERATE: _cairo_int_status = 101;
// pub const CAIRO_INT_STATUS_UNSUPPORTED: _cairo_int_status = 100;
// pub const CAIRO_INT_STATUS_LAST_STATUS: _cairo_int_status = 44;
// pub const CAIRO_INT_STATUS_DWRITE_ERROR: _cairo_int_status = 43;
// pub const CAIRO_INT_STATUS_TAG_ERROR: _cairo_int_status = 42;
// pub const CAIRO_INT_STATUS_WIN32_GDI_ERROR: _cairo_int_status = 41;
// pub const CAIRO_INT_STATUS_FREETYPE_ERROR: _cairo_int_status = 40;
// pub const CAIRO_INT_STATUS_PNG_ERROR: _cairo_int_status = 39;
// pub const CAIRO_INT_STATUS_JBIG2_GLOBAL_MISSING: _cairo_int_status = 38;
// pub const CAIRO_INT_STATUS_DEVICE_FINISHED: _cairo_int_status = 37;
// pub const CAIRO_INT_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_int_status = 36;
// pub const CAIRO_INT_STATUS_DEVICE_ERROR: _cairo_int_status = 35;
// pub const CAIRO_INT_STATUS_DEVICE_TYPE_MISMATCH: _cairo_int_status = 34;
// pub const CAIRO_INT_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_int_status = 33;
// pub const CAIRO_INT_STATUS_INVALID_SIZE: _cairo_int_status = 32;
// pub const CAIRO_INT_STATUS_INVALID_WEIGHT: _cairo_int_status = 31;
// pub const CAIRO_INT_STATUS_INVALID_SLANT: _cairo_int_status = 30;
// pub const CAIRO_INT_STATUS_INVALID_CLUSTERS: _cairo_int_status = 29;
// pub const CAIRO_INT_STATUS_NEGATIVE_COUNT: _cairo_int_status = 28;
// pub const CAIRO_INT_STATUS_USER_FONT_ERROR: _cairo_int_status = 27;
// pub const CAIRO_INT_STATUS_USER_FONT_IMMUTABLE: _cairo_int_status = 26;
// pub const CAIRO_INT_STATUS_FONT_TYPE_MISMATCH: _cairo_int_status = 25;
// pub const CAIRO_INT_STATUS_INVALID_STRIDE: _cairo_int_status = 24;
// pub const CAIRO_INT_STATUS_TEMP_FILE_ERROR: _cairo_int_status = 23;
// pub const CAIRO_INT_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_int_status = 22;
// pub const CAIRO_INT_STATUS_INVALID_INDEX: _cairo_int_status = 21;
// pub const CAIRO_INT_STATUS_INVALID_DSC_COMMENT: _cairo_int_status = 20;
// pub const CAIRO_INT_STATUS_INVALID_DASH: _cairo_int_status = 19;
// pub const CAIRO_INT_STATUS_FILE_NOT_FOUND: _cairo_int_status = 18;
// pub const CAIRO_INT_STATUS_INVALID_VISUAL: _cairo_int_status = 17;
// pub const CAIRO_INT_STATUS_INVALID_FORMAT: _cairo_int_status = 16;
// pub const CAIRO_INT_STATUS_INVALID_CONTENT: _cairo_int_status = 15;
// pub const CAIRO_INT_STATUS_PATTERN_TYPE_MISMATCH: _cairo_int_status = 14;
// pub const CAIRO_INT_STATUS_SURFACE_TYPE_MISMATCH: _cairo_int_status = 13;
// pub const CAIRO_INT_STATUS_SURFACE_FINISHED: _cairo_int_status = 12;
// pub const CAIRO_INT_STATUS_WRITE_ERROR: _cairo_int_status = 11;
// pub const CAIRO_INT_STATUS_READ_ERROR: _cairo_int_status = 10;
// pub const CAIRO_INT_STATUS_INVALID_PATH_DATA: _cairo_int_status = 9;
// pub const CAIRO_INT_STATUS_INVALID_STRING: _cairo_int_status = 8;
// pub const CAIRO_INT_STATUS_NULL_POINTER: _cairo_int_status = 7;
// pub const CAIRO_INT_STATUS_INVALID_STATUS: _cairo_int_status = 6;
// pub const CAIRO_INT_STATUS_INVALID_MATRIX: _cairo_int_status = 5;
// pub const CAIRO_INT_STATUS_NO_CURRENT_POINT: _cairo_int_status = 4;
// pub const CAIRO_INT_STATUS_INVALID_POP_GROUP: _cairo_int_status = 3;
// pub const CAIRO_INT_STATUS_INVALID_RESTORE: _cairo_int_status = 2;
// pub const CAIRO_INT_STATUS_NO_MEMORY: _cairo_int_status = 1;
// pub const CAIRO_INT_STATUS_SUCCESS: _cairo_int_status = 0;
// pub type uint32_t = __uint32_t;
// pub type cairo_color_t = _cairo_color;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_color {
//     pub red: f64,
//     pub green: f64,
//     pub blue: f64,
//     pub alpha: f64,
//     pub red_short: libc::c_ushort,
//     pub green_short: libc::c_ushort,
//     pub blue_short: libc::c_ushort,
//     pub alpha_short: libc::c_ushort,
// }
// pub type cairo_scaled_glyph_info_t = _cairo_scaled_glyph_info;
// pub type _cairo_scaled_glyph_info = usize;
// pub const CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE: _cairo_scaled_glyph_info = 16;
// pub const CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE: _cairo_scaled_glyph_info = 8;
// pub const CAIRO_SCALED_GLYPH_INFO_PATH: _cairo_scaled_glyph_info = 4;
// pub const CAIRO_SCALED_GLYPH_INFO_SURFACE: _cairo_scaled_glyph_info = 2;
// pub const CAIRO_SCALED_GLYPH_INFO_METRICS: _cairo_scaled_glyph_info = 1;
// pub type cairo_scaled_glyph_t = _cairo_scaled_glyph;
// #[derive(Copy, Clone, BitfieldStruct)]
// #[repr(C)]
// pub struct _cairo_scaled_glyph {
//     pub hash_entry: cairo_hash_entry_t,
//     pub metrics: cairo_text_extents_t,
//     pub fs_metrics: cairo_text_extents_t,
//     pub bbox: cairo_box_t,
//     pub x_advance: int16_t,
//     pub y_advance: int16_t,
//     pub has_info: usize,
//     pub surface: *mut cairo_image_surface_t,
//     pub path: *mut cairo_path_fixed_t,
//     pub recording_surface: *mut Surface,
//     pub color_surface: *mut cairo_image_surface_t,
//     pub dev_private_key: *const libc::c_void,
//     pub dev_private: *mut libc::c_void,
//     pub dev_privates: cairo_list_t,
//     pub foreground_color: cairo_color_t,
//     #[bitfield(name = "uses_foreground_color", ty = "usize", bits = "0..=0")]
//     #[bitfield(name = "color_glyph_set", ty = "usize", bits = "1..=1")]
//     #[bitfield(name = "color_glyph", ty = "usize", bits = "2..=2")]
//     pub uses_foreground_color_color_glyph_set_color_glyph: [u8; 1],
//     #[bitfield(padding)]
//     pub c2rust_padding: [u8; 7],
// }
// pub type cairo_image_surface_t = _cairo_image_surface;
// pub type Surface = _cairo_surface;
// #[derive(Copy, Clone, BitfieldStruct)]
// #[repr(C)]
// pub struct _cairo_surface {
//     pub backend: *const cairo_surface_backend_t,
//     pub device: *mut cairo_device_t,
//     pub type_0: Surfaceype_t,
//     pub content: cairo_content_t,
//     pub ref_count: cairo_reference_count_t,
//     pub status: Result<()>,
//     pub unique_id: usize,
//     pub serial: usize,
//     pub damage: *mut cairo_damage_t,
//     #[bitfield(name = "_finishing", ty = "usize", bits = "0..=0")]
//     #[bitfield(name = "finished", ty = "usize", bits = "1..=1")]
//     #[bitfield(name = "is_clear", ty = "usize", bits = "2..=2")]
//     #[bitfield(name = "has_font_options", ty = "usize", bits = "3..=3")]
//     #[bitfield(name = "owns_device", ty = "usize", bits = "4..=4")]
//     #[bitfield(name = "is_vector", ty = "usize", bits = "5..=5")]
//     pub _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [u8; 1],
//     #[bitfield(padding)]
//     pub c2rust_padding: [u8; 7],
//     pub user_data: cairo_user_data_array_t,
//     pub mime_data: cairo_user_data_array_t,
//     pub device_transform: cairo_matrix_t,
//     pub device_transform_inverse: cairo_matrix_t,
//     pub device_transform_observers: cairo_list_t,
//     pub x_resolution: f64,
//     pub y_resolution: f64,
//     pub x_fallback_resolution: f64,
//     pub y_fallback_resolution: f64,
//     pub snapshot_of: *mut Surface,
//     pub snapshot_detach: cairo_surface_func_t,
//     pub snapshots: cairo_list_t,
//     pub snapshot: cairo_list_t,
//     pub font_options: cairo_font_options_t,
// }
// pub type cairo_font_options_t = _cairo_font_options;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_font_options {
//     pub antialias: cairo_antialias_t,
//     pub subpixel_order: cairo_subpixel_order_t,
//     pub lcd_filter: cairo_lcd_filter_t,
//     pub hint_style: cairo_hint_style_t,
//     pub hint_metrics: cairo_hint_metrics_t,
//     pub round_glyph_positions: cairo_round_glyph_positions_t,
//     pub variations: *mut libc::c_char,
//     pub color_mode: cairo_color_mode_t,
//     pub palette_index: usize,
// }
// pub type cairo_color_mode_t = _cairo_color_mode;
// pub type _cairo_color_mode = usize;
// pub const CAIRO_COLOR_MODE_COLOR: _cairo_color_mode = 2;
// pub const CAIRO_COLOR_MODE_NO_COLOR: _cairo_color_mode = 1;
// pub const CAIRO_COLOR_MODE_DEFAULT: _cairo_color_mode = 0;
// pub type cairo_round_glyph_positions_t = _cairo_round_glyph_positions;
// pub type _cairo_round_glyph_positions = usize;
// pub const CAIRO_ROUND_GLYPH_POS_OFF: _cairo_round_glyph_positions = 2;
// pub const CAIRO_ROUND_GLYPH_POS_ON: _cairo_round_glyph_positions = 1;
// pub const CAIRO_ROUND_GLYPH_POS_DEFAULT: _cairo_round_glyph_positions = 0;
// pub type cairo_hint_metrics_t = _cairo_hint_metrics;
// pub type _cairo_hint_metrics = usize;
// pub const CAIRO_HINT_METRICS_ON: _cairo_hint_metrics = 2;
// pub const CAIRO_HINT_METRICS_OFF: _cairo_hint_metrics = 1;
// pub const CAIRO_HINT_METRICS_DEFAULT: _cairo_hint_metrics = 0;
// pub type cairo_hint_style_t = _cairo_hint_style;
// pub type _cairo_hint_style = usize;
// pub const CAIRO_HINT_STYLE_FULL: _cairo_hint_style = 4;
// pub const CAIRO_HINT_STYLE_MEDIUM: _cairo_hint_style = 3;
// pub const CAIRO_HINT_STYLE_SLIGHT: _cairo_hint_style = 2;
// pub const CAIRO_HINT_STYLE_NONE: _cairo_hint_style = 1;
// pub const CAIRO_HINT_STYLE_DEFAULT: _cairo_hint_style = 0;
// pub type cairo_lcd_filter_t = _cairo_lcd_filter;
// pub type _cairo_lcd_filter = usize;
// pub const CAIRO_LCD_FILTER_FIR5: _cairo_lcd_filter = 4;
// pub const CAIRO_LCD_FILTER_FIR3: _cairo_lcd_filter = 3;
// pub const CAIRO_LCD_FILTER_INTRA_PIXEL: _cairo_lcd_filter = 2;
// pub const CAIRO_LCD_FILTER_NONE: _cairo_lcd_filter = 1;
// pub const CAIRO_LCD_FILTER_DEFAULT: _cairo_lcd_filter = 0;
// pub type cairo_subpixel_order_t = _cairo_subpixel_order;
// pub type _cairo_subpixel_order = usize;
// pub const CAIRO_SUBPIXEL_ORDER_VBGR: _cairo_subpixel_order = 4;
// pub const CAIRO_SUBPIXEL_ORDER_VRGB: _cairo_subpixel_order = 3;
// pub const CAIRO_SUBPIXEL_ORDER_BGR: _cairo_subpixel_order = 2;
// pub const CAIRO_SUBPIXEL_ORDER_RGB: _cairo_subpixel_order = 1;
// pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: _cairo_subpixel_order = 0;
// pub type cairo_antialias_t = _cairo_antialias;
// pub type _cairo_antialias = usize;

#[derive(Clone, Copy)]
pub enum Antialias {
    DEFAULT = 0,
    NONE = 1,
    GRAY = 2,
    SUBPIXEL = 3,
    FAST = 4,
    GOOD = 5,
    BEST = 6,
}

// pub type cairo_surface_func_t = Option::<
//     unsafe extern "C" fn(*mut Surface) -> (),
// >;
// pub type cairo_matrix_t = _cairo_matrix;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_matrix {
//     pub xx: f64,
//     pub yx: f64,
//     pub xy: f64,
//     pub yy: f64,
//     pub x0: f64,
//     pub y0: f64,
// }
// pub type cairo_user_data_array_t = cairo_array_t;
// pub type cairo_array_t = _cairo_array;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_array {
//     pub size: usize,
//     pub num_elements: usize,
//     pub element_size: usize,
//     pub elements: *mut libc::c_char,
// }
// pub type cairo_damage_t = _cairo_damage;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct cairo_reference_count_t {
//     pub ref_count: cairo_atomic_int_t,
// }
// pub type cairo_atomic_int_t = isize;
// pub type cairo_content_t = _cairo_content;
// pub type _cairo_content = usize;
// pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;
// pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
// pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
// pub type Surfaceype_t = _Surfaceype;
// pub type _Surfaceype = usize;
// pub const SurfaceYPE_COGL: _Surfaceype = 24;
// pub const SurfaceYPE_SUBSURFACE: _Surfaceype = 23;
// pub const SurfaceYPE_SKIA: _Surfaceype = 22;
// pub const CAIRO_SURFACE_TYPE_XML: _cairo_surface_type = 21;
// pub const CAIRO_SURFACE_TYPE_TEE: _cairo_surface_type = 20;
// pub const CAIRO_SURFACE_TYPE_DRM: _cairo_surface_type = 19;
// pub const CAIRO_SURFACE_TYPE_GL: _cairo_surface_type = 18;
// pub const CAIRO_SURFACE_TYPE_VG: _cairo_surface_type = 17;
// pub const CAIRO_SURFACE_TYPE_RECORDING: _cairo_surface_type = 16;
// pub const CAIRO_SURFACE_TYPE_QT: _cairo_surface_type = 15;
// pub const CAIRO_SURFACE_TYPE_SCRIPT: _cairo_surface_type = 14;
// pub const CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: _cairo_surface_type = 13;
// pub const CAIRO_SURFACE_TYPE_WIN32_PRINTING: _cairo_surface_type = 12;
// pub const CAIRO_SURFACE_TYPE_OS2: _cairo_surface_type = 11;
// pub const CAIRO_SURFACE_TYPE_SVG: _cairo_surface_type = 10;
// pub const CAIRO_SURFACE_TYPE_DIRECTFB: _cairo_surface_type = 9;
// pub const CAIRO_SURFACE_TYPE_BEOS: _cairo_surface_type = 8;
// pub const CAIRO_SURFACE_TYPE_WIN32: _cairo_surface_type = 7;
// pub const CAIRO_SURFACE_TYPE_QUARTZ: _cairo_surface_type = 6;
// pub const CAIRO_SURFACE_TYPE_GLITZ: _cairo_surface_type = 5;
// pub const CAIRO_SURFACE_TYPE_XCB: _cairo_surface_type = 4;
// pub const CAIRO_SURFACE_TYPE_XLIB: _cairo_surface_type = 3;
// pub const CAIRO_SURFACE_TYPE_PS: _cairo_surface_type = 2;
// pub const CAIRO_SURFACE_TYPE_PDF: _cairo_surface_type = 1;
// pub const CAIRO_SURFACE_TYPE_IMAGE: _cairo_surface_type = 0;
// pub type cairo_device_t = _cairo_device;
// pub type cairo_surface_backend_t = _cairo_surface_backend;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_surface_backend {
//     pub type_0: cairo_surface_type_t,
//     pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> Result<()>>,
//     pub create_context: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
//     >,
//     pub create_similar: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_content_t,
//             isize,
//             isize,
//         ) -> *mut Surface,
//     >,
//     pub create_similar_image: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_format_t,
//             isize,
//             isize,
//         ) -> *mut Surface,
//     >,
//     pub map_to_image: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *const cairo_rectangle_int_t,
//         ) -> *mut cairo_image_surface_t,
//     >,
//     pub unmap_image: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut cairo_image_surface_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub source: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut cairo_rectangle_int_t,
//         ) -> *mut Surface,
//     >,
//     pub acquire_source_image: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut *mut cairo_image_surface_t,
//             *mut *mut libc::c_void,
//         ) -> Result<()>,
//     >,
//     pub release_source_image: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut cairo_image_surface_t,
//             *mut libc::c_void,
//         ) -> (),
//     >,
//     pub snapshot: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> *mut Surface,
//     >,
//     pub copy_page: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
//     >,
//     pub show_page: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
//     >,
//     pub get_extents: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             *mut cairo_rectangle_int_t,
//         ) -> cairo_bool_t,
//     >,
//     pub get_font_options: Option::<
//         unsafe extern "C" fn(*mut libc::c_void, *mut cairo_font_options_t) -> (),
//     >,
//     pub flush: Option::<
//         unsafe extern "C" fn(*mut libc::c_void, usize) -> Result<()>,
//     >,
//     pub mark_dirty_rectangle: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             isize,
//             isize,
//             isize,
//             isize,
//         ) -> Result<()>,
//     >,
//     pub paint: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub mask: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             *const Pattern,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub stroke: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             *const cairo_path_fixed_t,
//             *const cairo_stroke_style_t,
//             &Matrix,
//             &Matrix,
//             f64,
//             cairo_antialias_t,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub fill: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             *const cairo_path_fixed_t,
//             cairo_fill_rule_t,
//             f64,
//             cairo_antialias_t,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub fill_stroke: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             cairo_fill_rule_t,
//             f64,
//             cairo_antialias_t,
//             *const cairo_path_fixed_t,
//             cairo_operator_t,
//             *const Pattern,
//             *const cairo_stroke_style_t,
//             &Matrix,
//             &Matrix,
//             f64,
//             cairo_antialias_t,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub show_glyphs: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             *mut cairo_glyph_t,
//             isize,
//             *mut cairo_scaled_font_t,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub has_show_text_glyphs: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
//     >,
//     pub show_text_glyphs: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_operator_t,
//             *const Pattern,
//             *const libc::c_char,
//             isize,
//             *mut cairo_glyph_t,
//             isize,
//             *const cairo_text_cluster_t,
//             isize,
//             cairo_text_cluster_flags_t,
//             *mut cairo_scaled_font_t,
//             *const cairo_clip_t,
//         ) -> cairo_int_status_t,
//     >,
//     pub get_supported_mime_types: Option::<
//         unsafe extern "C" fn(*mut libc::c_void) -> *mut *const libc::c_char,
//     >,
//     pub tag: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             cairo_bool_t,
//             *const libc::c_char,
//             *const libc::c_char,
//         ) -> cairo_int_status_t,
//     >,
// }
// pub type cairo_clip_t = _cairo_clip;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_clip {
//     pub extents: cairo_rectangle_int_t,
//     pub path: *mut cairo_clip_path_t,
//     pub boxes: *mut cairo_box_t,
//     pub num_boxes: isize,
//     pub region: *mut cairo_region_t,
//     pub is_region: cairo_bool_t,
//     pub embedded_box: cairo_box_t,
// }
// pub type cairo_box_t = _cairo_line;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_line {
//     pub p1: cairo_point_t,
//     pub p2: cairo_point_t,
// }
// pub type cairo_point_t = _cairo_point;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_point {
//     pub x: cairo_fixed_t,
//     pub y: cairo_fixed_t,
// }
// pub type cairo_fixed_t = int32_t;
// pub type cairo_region_t = _cairo_region;
// pub type cairo_clip_path_t = _cairo_clip_path;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_clip_path {
//     pub ref_count: cairo_reference_count_t,
//     pub path: cairo_path_fixed_t,
//     pub fill_rule: cairo_fill_rule_t,
//     pub tolerance: f64,
//     pub antialias: cairo_antialias_t,
//     pub prev: *mut cairo_clip_path_t,
// }

pub enum FillRule {
    Winding,
    EvenOdd,
}

// pub type cairo_path_fixed_t = _cairo_path_fixed;
// #[derive(Copy, Clone, BitfieldStruct)]
// #[repr(C)]
// pub struct _cairo_path_fixed {
//     pub last_move_point: cairo_point_t,
//     pub current_point: cairo_point_t,
//     #[bitfield(name = "has_current_point", ty = "usize", bits = "0..=0")]
//     #[bitfield(name = "needs_move_to", ty = "usize", bits = "1..=1")]
//     #[bitfield(name = "has_extents", ty = "usize", bits = "2..=2")]
//     #[bitfield(name = "has_curve_to", ty = "usize", bits = "3..=3")]
//     #[bitfield(name = "stroke_is_rectilinear", ty = "usize", bits = "4..=4")]
//     #[bitfield(name = "fill_is_rectilinear", ty = "usize", bits = "5..=5")]
//     #[bitfield(name = "fill_maybe_region", ty = "usize", bits = "6..=6")]
//     #[bitfield(name = "fill_is_empty", ty = "usize", bits = "7..=7")]
//     pub has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [u8; 1],
//     #[bitfield(padding)]
//     pub c2rust_padding: [u8; 3],
//     pub extents: cairo_box_t,
//     pub buf: cairo_path_buf_fixed_t,
// }
// pub type cairo_path_buf_fixed_t = _cairo_path_buf_fixed;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_path_buf_fixed {
//     pub base: cairo_path_buf_t,
//     pub op: [cairo_path_op_t; 27],
//     pub points: [cairo_point_t; 54],
// }
// pub type cairo_path_op_t = libc::c_char;
// pub type cairo_path_buf_t = _cairo_path_buf;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_path_buf {
//     pub link: cairo_list_t,
//     pub num_ops: usize,
//     pub size_ops: usize,
//     pub num_points: usize,
//     pub size_points: usize,
//     pub op: *mut cairo_path_op_t,
//     pub points: *mut cairo_point_t,
// }
// pub type cairo_rectangle_int_t = _cairo_rectangle_int;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_rectangle_int {
//     pub x: isize,
//     pub y: isize,
//     pub width: isize,
//     pub height: isize,
// }
// pub type Pattern = _cairo_pattern;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_pattern {
//     pub ref_count: cairo_reference_count_t,
//     pub status: Result<()>,
//     pub user_data: cairo_user_data_array_t,
//     pub observers: cairo_list_t,
//     pub type_0: cairo_pattern_type_t,
//     pub filter: cairo_filter_t,
//     pub extend: cairo_extend_t,
//     pub has_component_alpha: cairo_bool_t,
//     pub is_userfont_foreground: cairo_bool_t,
//     pub matrix: cairo_matrix_t,
//     pub opacity: f64,
// }
// pub type cairo_extend_t = _cairo_extend;
// pub type _cairo_extend = usize;
// pub const CAIRO_EXTEND_PAD: _cairo_extend = 3;
// pub const CAIRO_EXTEND_REFLECT: _cairo_extend = 2;
// pub const CAIRO_EXTEND_REPEAT: _cairo_extend = 1;
// pub const CAIRO_EXTEND_NONE: _cairo_extend = 0;
// pub type cairo_filter_t = _cairo_filter;
// pub type _cairo_filter = usize;
// pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;
// pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
// pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
// pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
// pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
// pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
// pub type cairo_pattern_type_t = _cairo_pattern_type;
// pub type _cairo_pattern_type = usize;
// pub const CAIRO_PATTERN_TYPE_RASTER_SOURCE: _cairo_pattern_type = 5;
// pub const CAIRO_PATTERN_TYPE_MESH: _cairo_pattern_type = 4;
// pub const CAIRO_PATTERN_TYPE_RADIAL: _cairo_pattern_type = 3;
// pub const CAIRO_PATTERN_TYPE_LINEAR: _cairo_pattern_type = 2;
// pub const CAIRO_PATTERN_TYPE_SURFACE: _cairo_pattern_type = 1;
// pub const CAIRO_PATTERN_TYPE_SOLID: _cairo_pattern_type = 0;
// pub type cairo_operator_t = _cairo_operator;

pub enum Operator {
    Clear,
    Source,
    Over,
    In,
    Out,
    Atop,
    Dest,
    DestOver,
    DestIn,
    DestOut,
    DestAtop,
    Xor,
    Add,
    Saturate,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    HslHue,
    HslSaturation,
    HslColor,
    HslLuminosity,
}

// pub type cairo_stroke_style_t = _cairo_stroke_style;
// #[derive(Copy, Clone)]
// #[repr(C)]

pub struct StrokeStyle {
    // pub line_width: f64,
    // pub line_cap: cairo_line_cap_t,
    // pub line_join: cairo_line_join_t,
    // pub miter_limit: f64,
    // pub dash: *mut f64,
    // pub num_dashes: usize,
    // pub dash_offset: f64,
    // pub is_hairline: cairo_bool_t,
    // pub pre_hairline_line_width: f64,
}

// pub type cairo_line_join_t = _cairo_line_join;
// pub type _cairo_line_join = usize;
// pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;
// pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
// pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
// pub type cairo_line_cap_t = _cairo_line_cap;
// pub type _cairo_line_cap = usize;
// pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;
// pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
// pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
// pub type cairo_format_t = _cairo_format;
// pub type _cairo_format = isize;
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
// #[repr(C)]
// pub struct _cairo_hash_entry {
//     pub hash: uintptr_t,
// }
// pub type uintptr_t = libc::c_ulong;
// pub type cairo_font_type_t = _cairo_font_type;
// pub type _cairo_font_type = usize;
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
// #[repr(C)]
// pub struct _cairo_font_face {
//     pub hash_entry: cairo_hash_entry_t,
//     pub status: Result<()>,
//     pub ref_count: cairo_reference_count_t,
//     pub user_data: cairo_user_data_array_t,
//     pub backend: *const cairo_font_face_backend_t,
// }
// pub type cairo_font_face_backend_t = _cairo_font_face_backend;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_font_face_backend {
//     pub type_0: cairo_font_type_t,
//     pub create_for_toy: Option::<
//         unsafe extern "C" fn(
//             *mut cairo_toy_font_face_t,
//             *mut *mut cairo_font_face_t,
//         ) -> Result<()>,
//     >,
//     pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t>,
//     pub scaled_font_create: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             &Matrix,
//             &Matrix,
//             *const cairo_font_options_t,
//             *mut *mut cairo_scaled_font_t,
//         ) -> Result<()>,
//     >,
//     pub get_implementation: Option::<
//         unsafe extern "C" fn(
//             *mut libc::c_void,
//             &Matrix,
//             &Matrix,
//             *const cairo_font_options_t,
//         ) -> *mut cairo_font_face_t,
//     >,
// }
// pub type cairo_toy_font_face_t = _cairo_toy_font_face;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_toy_font_face {
//     pub base: cairo_font_face_t,
//     pub family: *const libc::c_char,
//     pub owns_family: cairo_bool_t,
//     pub slant: cairo_font_slant_t,
//     pub weight: cairo_font_weight_t,
//     pub impl_face: *mut cairo_font_face_t,
// }
// pub type cairo_font_weight_t = _cairo_font_weight;
// pub type _cairo_font_weight = usize;
// pub const CAIRO_FONT_WEIGHT_BOLD: _cairo_font_weight = 1;
// pub const CAIRO_FONT_WEIGHT_NORMAL: _cairo_font_weight = 0;
// pub type cairo_font_slant_t = _cairo_font_slant;
// pub type _cairo_font_slant = usize;
// pub const CAIRO_FONT_SLANT_OBLIQUE: _cairo_font_slant = 2;
// pub const CAIRO_FONT_SLANT_ITALIC: _cairo_font_slant = 1;
// pub const CAIRO_FONT_SLANT_NORMAL: _cairo_font_slant = 0;
// pub type cairo_rectangle_list_t = _cairo_rectangle_list;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_rectangle_list {
//     pub status: Result<()>,
//     pub rectangles: *mut cairo_rectangle_t,
//     pub num_rectangles: isize,
// }
// pub type cairo_rectangle_t = _cairo_rectangle;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_rectangle {
//     pub x: f64,
//     pub y: f64,
//     pub width: f64,
//     pub height: f64,
// }
// pub type cairo_path_t = cairo_path;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct cairo_path {
//     pub status: Result<()>,
//     pub data: *mut cairo_path_data_t,
//     pub num_data: isize,
// }
// pub type cairo_path_data_t = _cairo_path_data_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union _cairo_path_data_t {
//     pub header: C2RustUnnamed_0,
//     pub point: C2RustUnnamed,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed {
//     pub x: f64,
//     pub y: f64,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_0 {
//     pub type_0: cairo_path_data_type_t,
//     pub length: isize,
// }
// pub type cairo_path_data_type_t = _cairo_path_data_type;
// pub type _cairo_path_data_type = usize;
// pub const CAIRO_PATH_CLOSE_PATH: _cairo_path_data_type = 3;
// pub const CAIRO_PATH_CURVE_TO: _cairo_path_data_type = 2;
// pub const CAIRO_PATH_LINE_TO: _cairo_path_data_type = 1;
// pub const CAIRO_PATH_MOVE_TO: _cairo_path_data_type = 0;
// pub type cairo_backend_type_t = _cairo_backend_type;
// pub type _cairo_backend_type = usize;
// pub const CAIRO_TYPE_SKIA: _cairo_backend_type = 1;
// pub const CAIRO_TYPE_DEFAULT: _cairo_backend_type = 0;
// pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_user_data_key {
//     pub unused: isize,
// }
// pub type cairo_user_data_key_t = _cairo_user_data_key;
// #[inline(always)]
// unsafe extern "C" fn _cairo_atomic_int_get(
//     mut x: *mut cairo_atomic_int_t,
// ) -> cairo_atomic_int_t {
//     return ::std::intrinsics::atomic_load(x);
// }
// #[inline(always)]
// unsafe extern "C" fn _cairo_atomic_int_cmpxchg_impl(
//     mut x: *mut cairo_atomic_int_t,
//     mut oldv: cairo_atomic_int_t,
//     mut newv: cairo_atomic_int_t,
// ) -> cairo_bool_t {
//     let mut expected: cairo_atomic_int_t = oldv;
//     let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
//     *&mut expected = fresh0.0;
//     return fresh0.1 as cairo_bool_t;
// }
// static mut _cairo_nil: [cairo_t; 43] = [
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_NO_MEMORY,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_RESTORE,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_POP_GROUP,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_NO_CURRENT_POINT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_MATRIX,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_STATUS,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_NULL_POINTER,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_STRING,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_PATH_DATA,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_READ_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_WRITE_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_SURFACE_FINISHED,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_PATTERN_TYPE_MISMATCH,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_CONTENT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_FORMAT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_VISUAL,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_FILE_NOT_FOUND,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_DASH,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_DSC_COMMENT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_INDEX,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_CLIP_NOT_REPRESENTABLE,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: Result<()>EMP_FILE_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_STRIDE,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_FONT_TYPE_MISMATCH,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_USER_FONT_IMMUTABLE,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_USER_FONT_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_NEGATIVE_COUNT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_CLUSTERS,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_SLANT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_WEIGHT,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_SIZE,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_DEVICE_TYPE_MISMATCH,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_DEVICE_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_INVALID_MESH_CONSTRUCTION,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_DEVICE_FINISHED,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_JBIG2_GLOBAL_MISSING,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_PNG_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_FREETYPE_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_WIN32_GDI_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: Result<()>AG_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
//     {
//         let mut init = _cairo {
//             ref_count: {
//                 let mut init = cairo_reference_count_t {
//                     ref_count: -(1 as isize),
//                 };
//                 init
//             },
//             status: CAIRO_STATUS_DWRITE_ERROR,
//             user_data: {
//                 let mut init = _cairo_array {
//                     size: 0 as isize as usize,
//                     num_elements: 0 as isize as usize,
//                     element_size: 0 as isize as usize,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 };
//                 init
//             },
//             backend: 0 as *const cairo_backend_t,
//         };
//         init
//     },
// ];
// unsafe extern "C" fn _cairo_set_error(mut cr: *mut cairo_t, mut status: Result<()>) {
//     let mut ret__: isize = 0;
//     if (_cairo_error(status) as usize)
//         < CAIRO_STATUS_LAST_STATUS as isize as usize
//     {} else {
//         __assert_fail(
//             b"_cairo_error (status) < CAIRO_STATUS_LAST_STATUS\0" as *const u8
//                 as *const libc::c_char,
//             b"../src/cairo.c\0" as *const u8 as *const libc::c_char,
//             400 as isize as usize,
//             (*::std::mem::transmute::<
//                 &[u8; 49],
//                 &[libc::c_char; 49],
//             >(b"void _cairo_set_error(cairo_t *, Result<()>)\0"))
//                 .as_ptr(),
//         );
//     }
//     ret__ = _cairo_atomic_int_cmpxchg_impl(
//         &mut (*cr).status as *mut Result<()> as *mut cairo_atomic_int_t,
//         CAIRO_STATUS_SUCCESS as isize,
//         _cairo_error(status) as cairo_atomic_int_t,
//     );
// }

// pub unsafe extern "C" fn _cairo_create_in_error(
//     mut status: Result<()>,
// ) -> *mut cairo_t {
//     let mut cr: *mut cairo_t = 0 as *mut cairo_t;
//     if status as usize != CAIRO_STATUS_SUCCESS as isize as usize
//     {} else {
//         __assert_fail(
//             b"status != CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo.c\0" as *const u8 as *const libc::c_char,
//             408 as isize as usize,
//             (*::std::mem::transmute::<
//                 &[u8; 48],
//                 &[libc::c_char; 48],
//             >(b"cairo_t *_cairo_create_in_error(Result<()>)\0"))
//                 .as_ptr(),
//         );
//     }
//     cr = &*_cairo_nil
//         .as_ptr()
//         .offset(
//             (status as usize)
//                 .wrapping_sub(CAIRO_STATUS_NO_MEMORY as isize as usize)
//                 as isize,
//         ) as *const cairo_t as *mut cairo_t;
//     if status as usize == (*cr).status as usize {} else {
//         __assert_fail(
//             b"status == cr->status\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo.c\0" as *const u8 as *const libc::c_char,
//             411 as isize as usize,
//             (*::std::mem::transmute::<
//                 &[u8; 48],
//                 &[libc::c_char; 48],
//             >(b"cairo_t *_cairo_create_in_error(Result<()>)\0"))
//                 .as_ptr(),
//         );
//     }
//     return cr;
// }

// pub unsafe extern "C" fn cairo_create(target: &mut Surface) -> *mut cairo_t {
//     if target.is_null() {
//         return _cairo_create_in_error(_cairo_error(CAIRO_STATUS_NULL_POINTER));
//     }
//     if (*target).status as u64 != 0 {
//         return _cairo_create_in_error((*target).status);
//     }
//     if (*target).finished() != 0 {
//         return _cairo_create_in_error(_cairo_error(CAIRO_STATUS_SURFACE_FINISHED));
//     }
//     if ((*(*target).backend).create_context).is_none() {
//         return _cairo_create_in_error(_cairo_error(CAIRO_STATUS_WRITE_ERROR));
//     }
//     return ((*(*target).backend).create_context).expect("non-null function pointer")(
//         target as *mut libc::c_void,
//     );
// }

// pub unsafe extern "C" fn _cairo_init(
//     mut cr: *mut cairo_t,
//     mut backend: *const cairo_backend_t,
// ) {
//     (*cr).ref_count.ref_count = 1 as isize;
//     (*cr).status = CAIRO_STATUS_SUCCESS;
//     _cairo_user_data_array_init(&mut (*cr).user_data);
//     let ref mut fresh1 = (*cr).backend;
//     *fresh1 = backend;
// }

// pub unsafe extern "C" fn cairo_reference(mut cr: *mut cairo_t) -> *mut cairo_t {
//     if cr.is_null()
//         || _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) == -(1 as isize)
//     {
//         return cr;
//     }
//     if _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) > 0 as isize {} else {
//         __assert_fail(
//             b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&cr->ref_count)\0" as *const u8
//                 as *const libc::c_char,
//             b"../src/cairo.c\0" as *const u8 as *const libc::c_char,
//             494 as isize as usize,
//             (*::std::mem::transmute::<
//                 &[u8; 36],
//                 &[libc::c_char; 36],
//             >(b"cairo_t *cairo_reference(cairo_t *)\0"))
//                 .as_ptr(),
//         );
//     }
//     ::std::intrinsics::atomic_xadd(&mut (*cr).ref_count.ref_count, 1 as isize);
//     return cr;
// }

// pub unsafe extern "C" fn _cairo_fini(mut cr: *mut cairo_t) {
//     _cairo_user_data_array_fini(&mut (*cr).user_data);
// }

// pub unsafe extern "C" fn cairo_destroy(mut cr: *mut cairo_t) {
//     if cr.is_null()
//         || _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) == -(1 as isize)
//     {
//         return;
//     }
//     if _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) > 0 as isize {} else {
//         __assert_fail(
//             b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&cr->ref_count)\0" as *const u8
//                 as *const libc::c_char,
//             b"../src/cairo.c\0" as *const u8 as *const libc::c_char,
//             523 as isize as usize,
//             (*::std::mem::transmute::<
//                 &[u8; 30],
//                 &[libc::c_char; 30],
//             >(b"void cairo_destroy(cairo_t *)\0"))
//                 .as_ptr(),
//         );
//     }
//     if !(::std::intrinsics::atomic_xsub(
//         &mut (*cr).ref_count.ref_count as *mut cairo_atomic_int_t,
//         1 as isize,
//     ) == 1 as isize)
//     {
//         return;
//     }
//     ((*(*cr).backend).destroy)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_user_data(
//     mut cr: *mut cairo_t,
//     mut key: *const cairo_user_data_key_t,
// ) -> *mut libc::c_void {
//     return _cairo_user_data_array_get_data(&mut (*cr).user_data, key);
// }

// pub unsafe extern "C" fn cairo_set_user_data(
//     mut cr: *mut cairo_t,
//     mut key: *const cairo_user_data_key_t,
//     mut user_data: *mut libc::c_void,
//     mut destroy: cairo_destroy_func_t,
// ) -> Result<()> {
//     if _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) == -(1 as isize) {
//         return (*cr).status;
//     }
//     return _cairo_user_data_array_set_data(
//         &mut (*cr).user_data,
//         key,
//         user_data,
//         destroy,
//     );
// }

// pub unsafe extern "C" fn cairo_get_reference_count(
//     mut cr: *mut cairo_t,
// ) -> usize {
//     if cr.is_null()
//         || _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) == -(1 as isize)
//     {
//         return 0 as isize as usize;
//     }
//     return _cairo_atomic_int_get(&mut (*cr).ref_count.ref_count) as usize;
// }

// pub unsafe extern "C" fn cairo_save(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).save)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_restore(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).restore)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_push_group(mut cr: *mut cairo_t) {
//     cairo_push_group_with_content(cr, CAIRO_CONTENT_COLOR_ALPHA);
// }

// pub unsafe extern "C" fn cairo_push_group_with_content(
//     mut cr: *mut cairo_t,
//     mut content: cairo_content_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).push_group)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, content);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_pop_group(mut cr: *mut cairo_t) -> *mut Pattern {
//     let mut group_pattern: *mut Pattern = 0 as *mut Pattern;
//     if (*cr).status as u64 != 0 {
//         return _cairo_pattern_create_in_error((*cr).status);
//     }
//     group_pattern = ((*(*cr).backend).pop_group)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if (*group_pattern).status as u64 != 0 {
//         _cairo_set_error(cr, (*group_pattern).status);
//     }
//     return group_pattern;
// }

// pub unsafe extern "C" fn cairo_pop_group_to_source(mut cr: *mut cairo_t) {
//     let mut group_pattern: *mut Pattern = 0 as *mut Pattern;
//     group_pattern = cairo_pop_group(cr);
//     cairo_set_source(cr, group_pattern);
//     cairo_pattern_destroy(group_pattern);
// }

// pub unsafe extern "C" fn cairo_set_operator(
//     mut cr: *mut cairo_t,
//     mut op: cairo_operator_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_operator)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, op);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_source_rgb(
//     mut cr: *mut cairo_t,
//     mut red: f64,
//     mut green: f64,
//     mut blue: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_source_rgba)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, red, green, blue, 1.0f64);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_source_rgba(
//     mut cr: *mut cairo_t,
//     mut red: f64,
//     mut green: f64,
//     mut blue: f64,
//     mut alpha: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_source_rgba)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, red, green, blue, alpha);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_source_surface(
//     mut cr: *mut cairo_t,
//     mut surface: *mut Surface,
//     mut x: f64,
//     mut y: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if surface.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     status = ((*(*cr).backend).set_source_surface)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, surface, x, y);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_source(
//     mut cr: *mut cairo_t,
//     mut source: *mut Pattern,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if source.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     if (*source).status as u64 != 0 {
//         _cairo_set_error(cr, (*source).status);
//         return;
//     }
//     status = ((*(*cr).backend).set_source)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, source);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_source(mut cr: *mut cairo_t) -> *mut Pattern {
//     if (*cr).status as u64 != 0 {
//         return _cairo_pattern_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).get_source)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_set_tolerance(
//     mut cr: *mut cairo_t,
//     mut tolerance: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_tolerance)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, tolerance);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_antialias(
//     mut cr: *mut cairo_t,
//     mut antialias: cairo_antialias_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_antialias)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, antialias);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_fill_rule(
//     mut cr: *mut cairo_t,
//     mut fill_rule: cairo_fill_rule_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_fill_rule)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, fill_rule);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_line_width(
//     mut cr: *mut cairo_t,
//     mut width: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if width < 0.0f64 {
//         width = 0.0f64;
//     }
//     status = ((*(*cr).backend).set_line_width)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, width);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_hairline(
//     mut cr: *mut cairo_t,
//     mut set_hairline: cairo_bool_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_hairline)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, set_hairline);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_line_cap(
//     mut cr: *mut cairo_t,
//     mut line_cap: cairo_line_cap_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_line_cap)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, line_cap);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_line_join(
//     mut cr: *mut cairo_t,
//     mut line_join: cairo_line_join_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_line_join)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, line_join);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_dash(
//     mut cr: *mut cairo_t,
//     mut dashes: *const f64,
//     mut num_dashes: isize,
//     mut offset: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_dash)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, dashes, num_dashes, offset);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_dash_count(mut cr: *mut cairo_t) -> isize {
//     let mut num_dashes: isize = 0;
//     if (*cr).status as u64 != 0 {
//         return 0 as isize;
//     }
//     ((*(*cr).backend).get_dash)
//         .expect(
//             "non-null function pointer",
//         )(
//         cr as *mut libc::c_void,
//         0 as *mut f64,
//         &mut num_dashes,
//         0 as *mut f64,
//     );
//     return num_dashes;
// }

// pub unsafe extern "C" fn cairo_get_dash(
//     mut cr: *mut cairo_t,
//     mut dashes: *mut f64,
//     mut offset: *mut f64,
// ) {
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     ((*(*cr).backend).get_dash)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, dashes, 0 as *mut isize, offset);
// }

// pub unsafe extern "C" fn cairo_set_miter_limit(
//     mut cr: *mut cairo_t,
//     mut limit: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_miter_limit)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, limit);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_translate(
//     mut cr: *mut cairo_t,
//     mut tx: f64,
//     mut ty: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).translate)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, tx, ty);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_scale(
//     mut cr: *mut cairo_t,
//     mut sx: f64,
//     mut sy: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).scale)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, sx, sy);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_rotate(mut cr: *mut cairo_t, mut angle: f64) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).rotate)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, angle);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_transform(
//     mut cr: *mut cairo_t,
//     mut matrix: &Matrix,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).transform)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, matrix);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_matrix(
//     mut cr: *mut cairo_t,
//     mut matrix: &Matrix,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_matrix)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, matrix);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_identity_matrix(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_identity_matrix)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_user_to_device(
//     mut cr: *mut cairo_t,
//     mut x: *mut f64,
//     mut y: *mut f64,
// ) {
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     ((*(*cr).backend).user_to_device)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
// }

// pub unsafe extern "C" fn cairo_user_to_device_distance(
//     mut cr: *mut cairo_t,
//     mut dx: *mut f64,
//     mut dy: *mut f64,
// ) {
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     ((*(*cr).backend).user_to_device_distance)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, dx, dy);
// }

// pub unsafe extern "C" fn cairo_device_to_user(
//     mut cr: *mut cairo_t,
//     mut x: *mut f64,
//     mut y: *mut f64,
// ) {
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     ((*(*cr).backend).device_to_user)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
// }

// pub unsafe extern "C" fn cairo_device_to_user_distance(
//     mut cr: *mut cairo_t,
//     mut dx: *mut f64,
//     mut dy: *mut f64,
// ) {
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     ((*(*cr).backend).device_to_user_distance)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, dx, dy);
// }

// pub unsafe extern "C" fn cairo_new_path(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).new_path)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_new_sub_path(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).new_sub_path)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_move_to(
//     mut cr: *mut cairo_t,
//     mut x: f64,
//     mut y: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).move_to)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_line_to(
//     mut cr: *mut cairo_t,
//     mut x: f64,
//     mut y: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).line_to)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_curve_to(
//     mut cr: *mut cairo_t,
//     mut x1: f64,
//     mut y1: f64,
//     mut x2: f64,
//     mut y2: f64,
//     mut x3: f64,
//     mut y3: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).curve_to)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, x1, y1, x2, y2, x3, y3);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_arc(
//     mut cr: *mut cairo_t,
//     mut xc: f64,
//     mut yc: f64,
//     mut radius: f64,
//     mut angle1: f64,
//     mut angle2: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if angle2 < angle1 {
//         angle2 = fmod(
//             angle2 - angle1,
//             2 as isize as f64 * 3.14159265358979323846f64,
//         );
//         if angle2 < 0 as isize as f64 {
//             angle2 += 2 as isize as f64 * 3.14159265358979323846f64;
//         }
//         angle2 += angle1;
//     }
//     status = ((*(*cr).backend).arc)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, xc, yc, radius, angle1, angle2, 1 as isize);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_arc_negative(
//     mut cr: *mut cairo_t,
//     mut xc: f64,
//     mut yc: f64,
//     mut radius: f64,
//     mut angle1: f64,
//     mut angle2: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if angle2 > angle1 {
//         angle2 = fmod(
//             angle2 - angle1,
//             2 as isize as f64 * 3.14159265358979323846f64,
//         );
//         if angle2 > 0 as isize as f64 {
//             angle2 -= 2 as isize as f64 * 3.14159265358979323846f64;
//         }
//         angle2 += angle1;
//     }
//     status = ((*(*cr).backend).arc)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, xc, yc, radius, angle1, angle2, 0 as isize);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_rel_move_to(
//     mut cr: *mut cairo_t,
//     mut dx: f64,
//     mut dy: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).rel_move_to)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, dx, dy);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_rel_line_to(
//     mut cr: *mut cairo_t,
//     mut dx: f64,
//     mut dy: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).rel_line_to)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, dx, dy);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_rel_curve_to(
//     mut cr: *mut cairo_t,
//     mut dx1: f64,
//     mut dy1: f64,
//     mut dx2: f64,
//     mut dy2: f64,
//     mut dx3: f64,
//     mut dy3: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).rel_curve_to)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, dx1, dy1, dx2, dy2, dx3, dy3);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_rectangle(
//     mut cr: *mut cairo_t,
//     mut x: f64,
//     mut y: f64,
//     mut width: f64,
//     mut height: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).rectangle)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, x, y, width, height);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_close_path(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).close_path)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_path_extents(
//     mut cr: *mut cairo_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) {
//     if (*cr).status as u64 != 0 {
//         if !x1.is_null() {
//             *x1 = 0.0f64;
//         }
//         if !y1.is_null() {
//             *y1 = 0.0f64;
//         }
//         if !x2.is_null() {
//             *x2 = 0.0f64;
//         }
//         if !y2.is_null() {
//             *y2 = 0.0f64;
//         }
//         return;
//     }
//     ((*(*cr).backend).path_extents)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x1, y1, x2, y2);
// }

// pub unsafe extern "C" fn cairo_paint(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).paint)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_paint_with_alpha(
//     mut cr: *mut cairo_t,
//     mut alpha: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).paint_with_alpha)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, alpha);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_mask(
//     mut cr: *mut cairo_t,
//     mut pattern: *mut Pattern,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if pattern.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     if (*pattern).status as u64 != 0 {
//         _cairo_set_error(cr, (*pattern).status);
//         return;
//     }
//     status = ((*(*cr).backend).mask)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, pattern);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_mask_surface(
//     mut cr: *mut cairo_t,
//     mut surface: *mut Surface,
//     mut surface_x: f64,
//     mut surface_y: f64,
// ) {
//     let mut pattern: *mut Pattern = 0 as *mut Pattern;
//     let mut matrix: cairo_matrix_t = cairo_matrix_t {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     pattern = cairo_pattern_create_for_surface(surface);
//     cairo_matrix_init_translate(&mut matrix, -surface_x, -surface_y);
//     cairo_pattern_set_matrix(pattern, &mut matrix);
//     cairo_mask(cr, pattern);
//     cairo_pattern_destroy(pattern);
// }

// pub unsafe extern "C" fn cairo_stroke(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).stroke)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_stroke_preserve(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).stroke_preserve)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_fill(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).fill)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_fill_preserve(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).fill_preserve)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_copy_page(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).copy_page)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_show_page(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).show_page)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_in_stroke(
//     mut cr: *mut cairo_t,
//     mut x: f64,
//     mut y: f64,
// ) -> cairo_bool_t {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     let mut inside: cairo_bool_t = 0 as isize;
//     if (*cr).status as u64 != 0 {
//         return 0 as isize;
//     }
//     status = ((*(*cr).backend).in_stroke)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y, &mut inside);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
//     return inside;
// }

// pub unsafe extern "C" fn cairo_in_fill(
//     mut cr: *mut cairo_t,
//     mut x: f64,
//     mut y: f64,
// ) -> cairo_bool_t {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     let mut inside: cairo_bool_t = 0 as isize;
//     if (*cr).status as u64 != 0 {
//         return 0 as isize;
//     }
//     status = ((*(*cr).backend).in_fill)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y, &mut inside);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
//     return inside;
// }

// pub unsafe extern "C" fn cairo_stroke_extents(
//     mut cr: *mut cairo_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         if !x1.is_null() {
//             *x1 = 0.0f64;
//         }
//         if !y1.is_null() {
//             *y1 = 0.0f64;
//         }
//         if !x2.is_null() {
//             *x2 = 0.0f64;
//         }
//         if !y2.is_null() {
//             *y2 = 0.0f64;
//         }
//         return;
//     }
//     status = ((*(*cr).backend).stroke_extents)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x1, y1, x2, y2);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_fill_extents(
//     mut cr: *mut cairo_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         if !x1.is_null() {
//             *x1 = 0.0f64;
//         }
//         if !y1.is_null() {
//             *y1 = 0.0f64;
//         }
//         if !x2.is_null() {
//             *x2 = 0.0f64;
//         }
//         if !y2.is_null() {
//             *y2 = 0.0f64;
//         }
//         return;
//     }
//     status = ((*(*cr).backend).fill_extents)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x1, y1, x2, y2);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_clip(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).clip)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_clip_preserve(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).clip_preserve)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_reset_clip(mut cr: *mut cairo_t) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).reset_clip)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_clip_extents(
//     mut cr: *mut cairo_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if !x1.is_null() {
//         *x1 = 0.0f64;
//     }
//     if !y1.is_null() {
//         *y1 = 0.0f64;
//     }
//     if !x2.is_null() {
//         *x2 = 0.0f64;
//     }
//     if !y2.is_null() {
//         *y2 = 0.0f64;
//     }
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).clip_extents)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x1, y1, x2, y2);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_in_clip(
//     mut cr: *mut cairo_t,
//     mut x: f64,
//     mut y: f64,
// ) -> cairo_bool_t {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     let mut inside: cairo_bool_t = 0 as isize;
//     if (*cr).status as u64 != 0 {
//         return 0 as isize;
//     }
//     status = ((*(*cr).backend).in_clip)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, x, y, &mut inside);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
//     return inside;
// }

// pub unsafe extern "C" fn cairo_copy_clip_rectangle_list(
//     mut cr: *mut cairo_t,
// ) -> *mut cairo_rectangle_list_t {
//     if (*cr).status as u64 != 0 {
//         return _cairo_rectangle_list_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).clip_copy_rectangle_list)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_tag_begin(
//     mut cr: *mut cairo_t,
//     mut tag_name: *const libc::c_char,
//     mut attributes: *const libc::c_char,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).tag_begin)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, tag_name, attributes);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_tag_end(
//     mut cr: *mut cairo_t,
//     mut tag_name: *const libc::c_char,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).tag_end)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, tag_name);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_select_font_face(
//     mut cr: *mut cairo_t,
//     mut family: *const libc::c_char,
//     mut slant: cairo_font_slant_t,
//     mut weight: cairo_font_weight_t,
// ) {
//     let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     font_face = cairo_toy_font_face_create(family, slant, weight);
//     if (*font_face).status as u64 != 0 {
//         _cairo_set_error(cr, (*font_face).status);
//         return;
//     }
//     status = ((*(*cr).backend).set_font_face)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, font_face);
//     cairo_font_face_destroy(font_face);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_font_extents(
//     mut cr: *mut cairo_t,
//     mut extents: *mut cairo_font_extents_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     (*extents).ascent = 0.0f64;
//     (*extents).descent = 0.0f64;
//     (*extents).height = 0.0f64;
//     (*extents).max_x_advance = 0.0f64;
//     (*extents).max_y_advance = 0.0f64;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).font_extents)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, extents);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_font_face(
//     mut cr: *mut cairo_t,
//     mut font_face: *mut cairo_font_face_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_font_face)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, font_face);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_font_face(
//     mut cr: *mut cairo_t,
// ) -> *mut cairo_font_face_t {
//     if (*cr).status as u64 != 0 {
//         return &_cairo_font_face_nil as *const cairo_font_face_t
//             as *mut cairo_font_face_t;
//     }
//     return ((*(*cr).backend).get_font_face)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_set_font_size(
//     mut cr: *mut cairo_t,
//     mut size: f64,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_font_size)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, size);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_set_font_matrix(
//     mut cr: *mut cairo_t,
//     mut matrix: &Matrix,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = ((*(*cr).backend).set_font_matrix)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, matrix);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_font_matrix(
//     mut cr: *mut cairo_t,
//     mut matrix: *mut cairo_matrix_t,
// ) {
//     if (*cr).status as u64 != 0 {
//         cairo_matrix_init_identity(matrix);
//         return;
//     }
//     ((*(*cr).backend).get_font_matrix)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, matrix);
// }

// pub unsafe extern "C" fn cairo_set_font_options(
//     mut cr: *mut cairo_t,
//     mut options: *const cairo_font_options_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     status = cairo_font_options_status(options as *mut cairo_font_options_t);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//         return;
//     }
//     status = ((*(*cr).backend).set_font_options)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, options);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_font_options(
//     mut cr: *mut cairo_t,
//     mut options: *mut cairo_font_options_t,
// ) {
//     if cairo_font_options_status(options) as u64 != 0 {
//         return;
//     }
//     if (*cr).status as u64 != 0 {
//         _cairo_font_options_init_default(options);
//         return;
//     }
//     ((*(*cr).backend).get_font_options)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, options);
// }

// pub unsafe extern "C" fn cairo_set_scaled_font(
//     mut cr: *mut cairo_t,
//     mut scaled_font: *const cairo_scaled_font_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if scaled_font.is_null() {
//         _cairo_set_error(cr, _cairo_error(CAIRO_STATUS_NULL_POINTER));
//         return;
//     }
//     status = (*scaled_font).status;
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//         return;
//     }
//     status = ((*(*cr).backend).set_scaled_font)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, scaled_font as *mut cairo_scaled_font_t);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_scaled_font(
//     mut cr: *mut cairo_t,
// ) -> *mut cairo_scaled_font_t {
//     if (*cr).status as u64 != 0 {
//         return _cairo_scaled_font_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).get_scaled_font)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_text_extents(
//     mut cr: *mut cairo_t,
//     mut utf8: *const libc::c_char,
//     mut extents: *mut cairo_text_extents_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
//     let mut glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut num_glyphs: isize = 0 as isize;
//     let mut x: f64 = 0.;
//     let mut y: f64 = 0.;
//     (*extents).x_bearing = 0.0f64;
//     (*extents).y_bearing = 0.0f64;
//     (*extents).width = 0.0f64;
//     (*extents).height = 0.0f64;
//     (*extents).x_advance = 0.0f64;
//     (*extents).y_advance = 0.0f64;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if utf8.is_null() {
//         return;
//     }
//     scaled_font = cairo_get_scaled_font(cr);
//     if (*scaled_font).status as u64 != 0 {
//         _cairo_set_error(cr, (*scaled_font).status);
//         return;
//     }
//     cairo_get_current_point(cr, &mut x, &mut y);
//     status = cairo_scaled_font_text_to_glyphs(
//         scaled_font,
//         x,
//         y,
//         utf8,
//         -(1 as isize),
//         &mut glyphs,
//         &mut num_glyphs,
//         0 as *mut *mut cairo_text_cluster_t,
//         0 as *mut isize,
//         0 as *mut cairo_text_cluster_flags_t,
//     );
//     if status as usize == CAIRO_STATUS_SUCCESS as isize as usize {
//         status = ((*(*cr).backend).glyph_extents)
//             .expect(
//                 "non-null function pointer",
//             )(cr as *mut libc::c_void, glyphs, num_glyphs, extents);
//     }
//     cairo_glyph_free(glyphs);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_glyph_extents(
//     mut cr: *mut cairo_t,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: isize,
//     mut extents: *mut cairo_text_extents_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     (*extents).x_bearing = 0.0f64;
//     (*extents).y_bearing = 0.0f64;
//     (*extents).width = 0.0f64;
//     (*extents).height = 0.0f64;
//     (*extents).x_advance = 0.0f64;
//     (*extents).y_advance = 0.0f64;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if num_glyphs == 0 as isize {
//         return;
//     }
//     if num_glyphs < 0 as isize {
//         _cairo_set_error(cr, CAIRO_STATUS_NEGATIVE_COUNT);
//         return;
//     }
//     if glyphs.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     status = ((*(*cr).backend).glyph_extents)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, glyphs, num_glyphs, extents);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_show_text(
//     mut cr: *mut cairo_t,
//     mut utf8: *const libc::c_char,
// ) {
//     let mut extents: cairo_text_extents_t = cairo_text_extents_t {
//         x_bearing: 0.,
//         y_bearing: 0.,
//         width: 0.,
//         height: 0.,
//         x_advance: 0.,
//         y_advance: 0.,
//     };
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     let mut glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut last_glyph: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut clusters: *mut cairo_text_cluster_t = 0 as *mut cairo_text_cluster_t;
//     let mut utf8_len: isize = 0;
//     let mut num_glyphs: isize = 0;
//     let mut num_clusters: isize = 0;
//     let mut cluster_flags: cairo_text_cluster_flags_t = 0 as cairo_text_cluster_flags_t;
//     let mut x: f64 = 0.;
//     let mut y: f64 = 0.;
//     let mut has_show_text_glyphs: cairo_bool_t = 0;
//     let mut stack_glyphs: [cairo_glyph_t; 85] = [cairo_glyph_t {
//         index: 0,
//         x: 0.,
//         y: 0.,
//     }; 85];
//     let mut stack_clusters: [cairo_text_cluster_t; 256] = [cairo_text_cluster_t {
//         num_bytes: 0,
//         num_glyphs: 0,
//     }; 256];
//     let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
//     let mut info: cairo_glyph_text_info_t = cairo_glyph_text_info_t {
//         utf8: 0 as *const libc::c_char,
//         utf8_len: 0,
//         clusters: 0 as *const cairo_text_cluster_t,
//         num_clusters: 0,
//         cluster_flags: 0 as cairo_text_cluster_flags_t,
//     };
//     let mut i: *mut cairo_glyph_text_info_t = 0 as *mut cairo_glyph_text_info_t;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if utf8.is_null() {
//         return;
//     }
//     scaled_font = cairo_get_scaled_font(cr);
//     if (*scaled_font).status as u64 != 0 {
//         _cairo_set_error(cr, (*scaled_font).status);
//         return;
//     }
//     utf8_len = strlen(utf8) as isize;
//     has_show_text_glyphs = cairo_surface_has_show_text_glyphs(cairo_get_target(cr));
//     glyphs = stack_glyphs.as_mut_ptr();
//     num_glyphs = (::std::mem::size_of::<[cairo_glyph_t; 85]>() as libc::c_ulong)
//         .wrapping_div(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
//         as isize;
//     if has_show_text_glyphs != 0 {
//         clusters = stack_clusters.as_mut_ptr();
//         num_clusters = (::std::mem::size_of::<[cairo_text_cluster_t; 256]>()
//             as libc::c_ulong)
//             .wrapping_div(::std::mem::size_of::<cairo_text_cluster_t>() as libc::c_ulong)
//             as isize;
//     } else {
//         clusters = 0 as *mut cairo_text_cluster_t;
//         num_clusters = 0 as isize;
//     }
//     cairo_get_current_point(cr, &mut x, &mut y);
//     status = cairo_scaled_font_text_to_glyphs(
//         scaled_font,
//         x,
//         y,
//         utf8,
//         utf8_len,
//         &mut glyphs,
//         &mut num_glyphs,
//         if has_show_text_glyphs != 0 {
//             &mut clusters
//         } else {
//             0 as *mut *mut cairo_text_cluster_t
//         },
//         &mut num_clusters,
//         &mut cluster_flags,
//     );
//     if !(status as u64 != 0) {
//         if num_glyphs == 0 as isize {
//             return;
//         }
//         i = 0 as *mut cairo_glyph_text_info_t;
//         if has_show_text_glyphs != 0 {
//             info.utf8 = utf8;
//             info.utf8_len = utf8_len;
//             info.clusters = clusters;
//             info.num_clusters = num_clusters;
//             info.cluster_flags = cluster_flags;
//             i = &mut info;
//         }
//         status = ((*(*cr).backend).glyphs)
//             .expect(
//                 "non-null function pointer",
//             )(cr as *mut libc::c_void, glyphs, num_glyphs, i);
//         if !(status as u64 != 0) {
//             last_glyph = &mut *glyphs.offset((num_glyphs - 1 as isize) as isize)
//                 as *mut cairo_glyph_t;
//             status = ((*(*cr).backend).glyph_extents)
//                 .expect(
//                     "non-null function pointer",
//                 )(cr as *mut libc::c_void, last_glyph, 1 as isize, &mut extents);
//             if !(status as u64 != 0) {
//                 x = (*last_glyph).x + extents.x_advance;
//                 y = (*last_glyph).y + extents.y_advance;
//                 ((*(*cr).backend).move_to)
//                     .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
//             }
//         }
//     }
//     if glyphs != stack_glyphs.as_mut_ptr() {
//         cairo_glyph_free(glyphs);
//     }
//     if clusters != stack_clusters.as_mut_ptr() {
//         cairo_text_cluster_free(clusters);
//     }
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_show_glyphs(
//     mut cr: *mut cairo_t,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: isize,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if num_glyphs == 0 as isize {
//         return;
//     }
//     if num_glyphs < 0 as isize {
//         _cairo_set_error(cr, CAIRO_STATUS_NEGATIVE_COUNT);
//         return;
//     }
//     if glyphs.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     status = ((*(*cr).backend).glyphs)
//         .expect(
//             "non-null function pointer",
//         )(
//         cr as *mut libc::c_void,
//         glyphs,
//         num_glyphs,
//         0 as *mut cairo_glyph_text_info_t,
//     );
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_show_text_glyphs(
//     mut cr: *mut cairo_t,
//     mut utf8: *const libc::c_char,
//     mut utf8_len: isize,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: isize,
//     mut clusters: *const cairo_text_cluster_t,
//     mut num_clusters: isize,
//     mut cluster_flags: cairo_text_cluster_flags_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if utf8.is_null() && utf8_len == -(1 as isize) {
//         utf8_len = 0 as isize;
//     }
//     if num_glyphs != 0 && glyphs.is_null() || utf8_len != 0 && utf8.is_null()
//         || num_clusters != 0 && clusters.is_null()
//     {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     if utf8_len == -(1 as isize) {
//         utf8_len = strlen(utf8) as isize;
//     }
//     if num_glyphs < 0 as isize || utf8_len < 0 as isize
//         || num_clusters < 0 as isize
//     {
//         _cairo_set_error(cr, CAIRO_STATUS_NEGATIVE_COUNT);
//         return;
//     }
//     if num_glyphs == 0 as isize && utf8_len == 0 as isize {
//         return;
//     }
//     if !utf8.is_null() {
//         status = _cairo_validate_text_clusters(
//             utf8,
//             utf8_len,
//             glyphs,
//             num_glyphs,
//             clusters,
//             num_clusters,
//             cluster_flags,
//         );
//         if status as usize
//             == CAIRO_STATUS_INVALID_CLUSTERS as isize as usize
//         {
//             let mut status2: Result<()> = CAIRO_STATUS_SUCCESS;
//             status2 = _cairo_utf8_to_ucs4(
//                 utf8,
//                 utf8_len,
//                 0 as *mut *mut uint32_t,
//                 0 as *mut isize,
//             );
//             if status2 as u64 != 0 {
//                 status = status2;
//             }
//         } else {
//             let mut info: cairo_glyph_text_info_t = cairo_glyph_text_info_t {
//                 utf8: 0 as *const libc::c_char,
//                 utf8_len: 0,
//                 clusters: 0 as *const cairo_text_cluster_t,
//                 num_clusters: 0,
//                 cluster_flags: 0 as cairo_text_cluster_flags_t,
//             };
//             info.utf8 = utf8;
//             info.utf8_len = utf8_len;
//             info.clusters = clusters;
//             info.num_clusters = num_clusters;
//             info.cluster_flags = cluster_flags;
//             status = ((*(*cr).backend).glyphs)
//                 .expect(
//                     "non-null function pointer",
//                 )(cr as *mut libc::c_void, glyphs, num_glyphs, &mut info);
//         }
//     } else {
//         status = ((*(*cr).backend).glyphs)
//             .expect(
//                 "non-null function pointer",
//             )(
//             cr as *mut libc::c_void,
//             glyphs,
//             num_glyphs,
//             0 as *mut cairo_glyph_text_info_t,
//         );
//     }
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_text_path(
//     mut cr: *mut cairo_t,
//     mut utf8: *const libc::c_char,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     let mut extents: cairo_text_extents_t = cairo_text_extents_t {
//         x_bearing: 0.,
//         y_bearing: 0.,
//         width: 0.,
//         height: 0.,
//         x_advance: 0.,
//         y_advance: 0.,
//     };
//     let mut stack_glyphs: [cairo_glyph_t; 85] = [cairo_glyph_t {
//         index: 0,
//         x: 0.,
//         y: 0.,
//     }; 85];
//     let mut glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut last_glyph: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
//     let mut num_glyphs: isize = 0;
//     let mut x: f64 = 0.;
//     let mut y: f64 = 0.;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if utf8.is_null() {
//         return;
//     }
//     glyphs = stack_glyphs.as_mut_ptr();
//     num_glyphs = (::std::mem::size_of::<[cairo_glyph_t; 85]>() as libc::c_ulong)
//         .wrapping_div(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
//         as isize;
//     scaled_font = cairo_get_scaled_font(cr);
//     if (*scaled_font).status as u64 != 0 {
//         _cairo_set_error(cr, (*scaled_font).status);
//         return;
//     }
//     cairo_get_current_point(cr, &mut x, &mut y);
//     status = cairo_scaled_font_text_to_glyphs(
//         scaled_font,
//         x,
//         y,
//         utf8,
//         -(1 as isize),
//         &mut glyphs,
//         &mut num_glyphs,
//         0 as *mut *mut cairo_text_cluster_t,
//         0 as *mut isize,
//         0 as *mut cairo_text_cluster_flags_t,
//     );
//     if num_glyphs == 0 as isize {
//         return;
//     }
//     status = ((*(*cr).backend).glyph_path)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, glyphs, num_glyphs);
//     if !(status as u64 != 0) {
//         last_glyph = &mut *glyphs.offset((num_glyphs - 1 as isize) as isize)
//             as *mut cairo_glyph_t;
//         status = ((*(*cr).backend).glyph_extents)
//             .expect(
//                 "non-null function pointer",
//             )(cr as *mut libc::c_void, last_glyph, 1 as isize, &mut extents);
//         if !(status as u64 != 0) {
//             x = (*last_glyph).x + extents.x_advance;
//             y = (*last_glyph).y + extents.y_advance;
//             ((*(*cr).backend).move_to)
//                 .expect("non-null function pointer")(cr as *mut libc::c_void, x, y);
//         }
//     }
//     if glyphs != stack_glyphs.as_mut_ptr() {
//         cairo_glyph_free(glyphs);
//     }
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_glyph_path(
//     mut cr: *mut cairo_t,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: isize,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if num_glyphs == 0 as isize {
//         return;
//     }
//     if num_glyphs < 0 as isize {
//         _cairo_set_error(cr, CAIRO_STATUS_NEGATIVE_COUNT);
//         return;
//     }
//     if glyphs.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     status = ((*(*cr).backend).glyph_path)
//         .expect(
//             "non-null function pointer",
//         )(cr as *mut libc::c_void, glyphs, num_glyphs);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// pub unsafe extern "C" fn cairo_get_operator(mut cr: *mut cairo_t) -> cairo_operator_t {
//     if (*cr).status as u64 != 0 {
//         return CAIRO_OPERATOR_OVER;
//     }
//     return ((*(*cr).backend).get_operator)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_tolerance(mut cr: *mut cairo_t) -> f64 {
//     if (*cr).status as u64 != 0 {
//         return 0.1f64;
//     }
//     return ((*(*cr).backend).get_tolerance)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_antialias(mut cr: *mut cairo_t) -> cairo_antialias_t {
//     if (*cr).status as u64 != 0 {
//         return CAIRO_ANTIALIAS_DEFAULT;
//     }
//     return ((*(*cr).backend).get_antialias)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_has_current_point(mut cr: *mut cairo_t) -> cairo_bool_t {
//     if (*cr).status as u64 != 0 {
//         return 0 as isize;
//     }
//     return ((*(*cr).backend).has_current_point)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_current_point(
//     mut cr: *mut cairo_t,
//     mut x_ret: *mut f64,
//     mut y_ret: *mut f64,
// ) {
//     let mut x: f64 = 0.;
//     let mut y: f64 = 0.;
//     y = 0 as isize as f64;
//     x = y;
//     if (*cr).status as usize
//         == CAIRO_STATUS_SUCCESS as isize as usize
//         && ((*(*cr).backend).has_current_point)
//             .expect("non-null function pointer")(cr as *mut libc::c_void) != 0
//     {
//         ((*(*cr).backend).get_current_point)
//             .expect(
//                 "non-null function pointer",
//             )(cr as *mut libc::c_void, &mut x, &mut y);
//     }
//     if !x_ret.is_null() {
//         *x_ret = x;
//     }
//     if !y_ret.is_null() {
//         *y_ret = y;
//     }
// }

// pub unsafe extern "C" fn cairo_get_fill_rule(mut cr: *mut cairo_t) -> cairo_fill_rule_t {
//     if (*cr).status as u64 != 0 {
//         return CAIRO_FILL_RULE_WINDING;
//     }
//     return ((*(*cr).backend).get_fill_rule)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_line_width(mut cr: *mut cairo_t) -> f64 {
//     if (*cr).status as u64 != 0 {
//         return 2.0f64;
//     }
//     return ((*(*cr).backend).get_line_width)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_hairline(mut cr: *mut cairo_t) -> cairo_bool_t {
//     if (*cr).status as u64 != 0 {
//         return 0 as isize;
//     }
//     return ((*(*cr).backend).get_hairline)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_line_cap(mut cr: *mut cairo_t) -> cairo_line_cap_t {
//     if (*cr).status as u64 != 0 {
//         return CAIRO_LINE_CAP_BUTT;
//     }
//     return ((*(*cr).backend).get_line_cap)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_line_join(mut cr: *mut cairo_t) -> cairo_line_join_t {
//     if (*cr).status as u64 != 0 {
//         return CAIRO_LINE_JOIN_MITER;
//     }
//     return ((*(*cr).backend).get_line_join)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_miter_limit(mut cr: *mut cairo_t) -> f64 {
//     if (*cr).status as u64 != 0 {
//         return 10.0f64;
//     }
//     return ((*(*cr).backend).get_miter_limit)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_matrix(
//     mut cr: *mut cairo_t,
//     mut matrix: *mut cairo_matrix_t,
// ) {
//     if (*cr).status as u64 != 0 {
//         cairo_matrix_init_identity(matrix);
//         return;
//     }
//     ((*(*cr).backend).get_matrix)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, matrix);
// }

// pub unsafe extern "C" fn cairo_get_target(mut cr: *mut cairo_t) -> *mut Surface {
//     if (*cr).status as u64 != 0 {
//         return _cairo_surface_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).get_original_target)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_get_group_target(
//     mut cr: *mut cairo_t,
// ) -> *mut Surface {
//     if (*cr).status as u64 != 0 {
//         return _cairo_surface_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).get_current_target)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_copy_path(mut cr: *mut cairo_t) -> *mut cairo_path_t {
//     if (*cr).status as u64 != 0 {
//         return _cairo_path_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).copy_path)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_copy_path_flat(
//     mut cr: *mut cairo_t,
// ) -> *mut cairo_path_t {
//     if (*cr).status as u64 != 0 {
//         return _cairo_path_create_in_error((*cr).status);
//     }
//     return ((*(*cr).backend).copy_path_flat)
//         .expect("non-null function pointer")(cr as *mut libc::c_void);
// }

// pub unsafe extern "C" fn cairo_append_path(
//     mut cr: *mut cairo_t,
//     mut path: *const cairo_path_t,
// ) {
//     let mut status: Result<()> = CAIRO_STATUS_SUCCESS;
//     if (*cr).status as u64 != 0 {
//         return;
//     }
//     if path.is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     if (*path).status as u64 != 0 {
//         if (*path).status as usize
//             > CAIRO_STATUS_SUCCESS as isize as usize
//             && (*path).status as usize
//                 <= CAIRO_STATUS_LAST_STATUS as isize as usize
//         {
//             _cairo_set_error(cr, (*path).status);
//         } else {
//             _cairo_set_error(cr, CAIRO_STATUS_INVALID_STATUS);
//         }
//         return;
//     }
//     if (*path).num_data == 0 as isize {
//         return;
//     }
//     if ((*path).data).is_null() {
//         _cairo_set_error(cr, CAIRO_STATUS_NULL_POINTER);
//         return;
//     }
//     status = ((*(*cr).backend).append_path)
//         .expect("non-null function pointer")(cr as *mut libc::c_void, path);
//     if status as u64 != 0 {
//         _cairo_set_error(cr, status);
//     }
// }

// impl Cairo {
//     pub fn status(&self) -> Result<()> {
//         return self.status;
//     }
// }
