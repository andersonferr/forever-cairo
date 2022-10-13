use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type FT_LibraryRec_;
    pub type _FcConfig;
    pub type _FcPattern;
    pub type FT_Face_InternalRec_;
    pub type FT_DriverRec_;
    pub type FT_Size_InternalRec_;
    pub type FT_Slot_InternalRec_;
    pub type FT_SubGlyphRec_;
    pub type _FcRange;
    pub type _FcLangSet;
    pub type _FcCharSet;
    fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
    fn cairo_font_options_equal(
        options: *const cairo_font_options_t,
        other: *const cairo_font_options_t,
    ) -> cairo_bool_t;
    fn cairo_font_face_reference(
        font_face: *mut cairo_font_face_t,
    ) -> *mut cairo_font_face_t;
    fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: libc::c_double,
        y_offset: libc::c_double,
    );
    fn cairo_surface_get_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: *mut libc::c_double,
        y_offset: *mut libc::c_double,
    );
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
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
    fn cairo_pattern_set_matrix(
        pattern: *mut cairo_pattern_t,
        matrix: *const cairo_matrix_t,
    );
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _cairo_scaled_font_set_error(
        scaled_font: *mut cairo_scaled_font_t,
        status: cairo_status_t,
    ) -> cairo_status_t;
    static _cairo_font_face_nil_file_not_found: cairo_font_face_t;
    fn _cairo_scaled_font_init(
        scaled_font: *mut cairo_scaled_font_t,
        font_face: *mut cairo_font_face_t,
        font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t,
        options: *const cairo_font_options_t,
        backend: *const cairo_scaled_font_backend_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_glyph_set_metrics(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        fs_metrics: *mut cairo_text_extents_t,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_image_surface_assume_ownership_of_data(
        surface: *mut cairo_image_surface_t,
    );
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_glyph_set_color_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        surface: *mut cairo_image_surface_t,
        uses_foreground_color: cairo_bool_t,
    );
    fn _cairo_scaled_glyph_set_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        surface: *mut cairo_image_surface_t,
    );
    fn _cairo_strtod(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn _cairo_path_fixed_create() -> *mut cairo_path_fixed_t;
    fn _cairo_path_fixed_move_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_line_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_get_current_point(
        path: *mut cairo_path_fixed_t,
        x: *mut cairo_fixed_t,
        y: *mut cairo_fixed_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_curve_to(
        path: *mut cairo_path_fixed_t,
        x0: cairo_fixed_t,
        y0: cairo_fixed_t,
        x1: cairo_fixed_t,
        y1: cairo_fixed_t,
        x2: cairo_fixed_t,
        y2: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_close_path(path: *mut cairo_path_fixed_t) -> cairo_status_t;
    fn _cairo_path_fixed_destroy(path: *mut cairo_path_fixed_t);
    fn _cairo_scaled_glyph_set_path(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        path: *mut cairo_path_fixed_t,
    );
    fn _cairo_hash_table_random_entry(
        hash_table: *mut cairo_hash_table_t,
        predicate: cairo_hash_predicate_func_t,
    ) -> *mut libc::c_void;
    fn _cairo_scaled_font_set_metrics(
        scaled_font: *mut cairo_scaled_font_t,
        fs_metrics: *mut cairo_font_extents_t,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_create_in_error(
        status: cairo_status_t,
    ) -> *mut cairo_scaled_font_t;
    fn _cairo_matrix_compute_basis_scale_factors(
        matrix: *const cairo_matrix_t,
        sx: *mut libc::c_double,
        sy: *mut libc::c_double,
        x_major: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_matrix_get_affine(
        matrix: *const cairo_matrix_t,
        xx: *mut libc::c_double,
        yx: *mut libc::c_double,
        xy: *mut libc::c_double,
        yy: *mut libc::c_double,
        x0: *mut libc::c_double,
        y0: *mut libc::c_double,
    );
    fn _cairo_font_face_twin_create_fallback() -> *mut cairo_font_face_t;
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_unscaled_font_init(
        font: *mut cairo_unscaled_font_t,
        backend: *const cairo_unscaled_font_backend_t,
    );
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    static _cairo_font_face_nil: cairo_font_face_t;
    fn _cairo_unscaled_font_reference(
        font: *mut cairo_unscaled_font_t,
    ) -> *mut cairo_unscaled_font_t;
    fn _cairo_font_options_init_copy(
        options: *mut cairo_font_options_t,
        other: *const cairo_font_options_t,
    );
    fn _cairo_font_face_init(
        font_face: *mut cairo_font_face_t,
        backend: *const cairo_font_face_backend_t,
    );
    fn _cairo_font_options_fini(options: *mut cairo_font_options_t);
    fn _cairo_unscaled_font_destroy(font: *mut cairo_unscaled_font_t);
    static mut _cairo_ft_unscaled_font_map_mutex: cairo_mutex_t;
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_hash_string(c: *const libc::c_char) -> uintptr_t;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pixman_image_get_format(image: *mut pixman_image_t) -> pixman_format_code_t;
    fn pixman_image_get_component_alpha(image: *mut pixman_image_t) -> pixman_bool_t;
    fn pixman_image_set_component_alpha(
        image: *mut pixman_image_t,
        component_alpha: pixman_bool_t,
    );
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn cairo_matrix_init(
        matrix: *mut cairo_matrix_t,
        xx: libc::c_double,
        yx: libc::c_double,
        xy: libc::c_double,
        yy: libc::c_double,
        x0: libc::c_double,
        y0: libc::c_double,
    );
    fn cairo_matrix_init_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn cairo_matrix_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn cairo_matrix_transform_point(
        matrix: *const cairo_matrix_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn FT_Set_Transform(face: FT_Face, matrix: *mut FT_Matrix, delta: *mut FT_Vector);
    fn FT_Set_Char_Size(
        face: FT_Face,
        char_width: FT_F26Dot6,
        char_height: FT_F26Dot6,
        horz_resolution: FT_UInt,
        vert_resolution: FT_UInt,
    ) -> FT_Error;
    fn FT_Load_Glyph(
        face: FT_Face,
        glyph_index: FT_UInt,
        load_flags: FT_Int32,
    ) -> FT_Error;
    fn FT_Vector_Transform(vector: *mut FT_Vector, matrix: *const FT_Matrix);
    fn FT_Get_First_Char(face: FT_Face, agindex: *mut FT_UInt) -> FT_ULong;
    fn FT_Get_Next_Char(
        face: FT_Face,
        char_code: FT_ULong,
        agindex: *mut FT_UInt,
    ) -> FT_ULong;
    fn FT_Get_Glyph_Name(
        face: FT_Face,
        glyph_index: FT_UInt,
        buffer: FT_Pointer,
        buffer_max: FT_UInt,
    ) -> FT_Error;
    fn FT_New_Face(
        library: FT_Library,
        filepathname: *const libc::c_char,
        face_index: FT_Long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    fn FT_Init_FreeType(alibrary: *mut FT_Library) -> FT_Error;
    fn FT_Done_Face(face: FT_Face) -> FT_Error;
    fn FT_Render_Glyph(slot: FT_GlyphSlot, render_mode: FT_Render_Mode) -> FT_Error;
    fn FT_Done_FreeType(library: FT_Library) -> FT_Error;
    fn FcPatternAddString(
        p: *mut FcPattern,
        object: *const libc::c_char,
        s: *const FcChar8,
    ) -> FcBool;
    fn FcConfigGetCurrent() -> *mut FcConfig;
    fn FcPatternDestroy(p: *mut FcPattern);
    fn FcPatternGetString(
        p: *const FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        s: *mut *mut FcChar8,
    ) -> FcResult;
    fn FcPatternGetBool(
        p: *const FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        b: *mut FcBool,
    ) -> FcResult;
    fn FcPatternGetInteger(
        p: *const FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        i: *mut libc::c_int,
    ) -> FcResult;
    fn FcFontMatch(
        config: *mut FcConfig,
        p: *mut FcPattern,
        result: *mut FcResult,
    ) -> *mut FcPattern;
    fn FcDefaultSubstitute(pattern: *mut FcPattern);
    fn FcPatternAddInteger(
        p: *mut FcPattern,
        object: *const libc::c_char,
        i: libc::c_int,
    ) -> FcBool;
    fn FcPatternGet(
        p: *const FcPattern,
        object: *const libc::c_char,
        id: libc::c_int,
        v: *mut FcValue,
    ) -> FcResult;
    fn FcPatternAddBool(
        p: *mut FcPattern,
        object: *const libc::c_char,
        b: FcBool,
    ) -> FcBool;
    fn FcPatternDel(p: *mut FcPattern, object: *const libc::c_char) -> FcBool;
    fn FcConfigSubstitute(
        config: *mut FcConfig,
        p: *mut FcPattern,
        kind: FcMatchKind,
    ) -> FcBool;
    fn FcPatternAddDouble(
        p: *mut FcPattern,
        object: *const libc::c_char,
        d: libc::c_double,
    ) -> FcBool;
    fn FcPatternDuplicate(p: *const FcPattern) -> *mut FcPattern;
    fn FcInitBringUptoDate() -> FcBool;
    fn FcPatternCreate() -> *mut FcPattern;
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn FcFreeTypeCharIndex(face: FT_Face, ucs4: FcChar32) -> FT_UInt;
    fn FcPatternGetFTFace(
        p: *const FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        f: *mut FT_Face,
    ) -> FcResult;
    fn FT_Outline_Decompose(
        outline: *mut FT_Outline,
        func_interface: *const FT_Outline_Funcs,
        user: *mut libc::c_void,
    ) -> FT_Error;
    fn FT_Outline_Get_CBox(outline: *const FT_Outline, acbox: *mut FT_BBox);
    fn FT_Outline_Translate(
        outline: *const FT_Outline,
        xOffset: FT_Pos,
        yOffset: FT_Pos,
    );
    fn FT_Outline_Transform(outline: *const FT_Outline, matrix: *const FT_Matrix);
    fn FT_Palette_Data_Get(face: FT_Face, apalette: *mut FT_Palette_Data) -> FT_Error;
    fn FT_Palette_Select(
        face: FT_Face,
        palette_index: FT_UShort,
        apalette: *mut *mut FT_Color,
    ) -> FT_Error;
    fn FT_Palette_Set_Foreground_Color(
        face: FT_Face,
        foreground_color: FT_Color,
    ) -> FT_Error;
    fn FT_Get_Color_Glyph_Layer(
        face: FT_Face,
        base_glyph: FT_UInt,
        aglyph_index: *mut FT_UInt,
        acolor_index: *mut FT_UInt,
        iterator: *mut FT_LayerIterator,
    ) -> FT_Bool;
    fn FT_Bitmap_New(abitmap: *mut FT_Bitmap);
    fn FT_Bitmap_Convert(
        library: FT_Library,
        source: *const FT_Bitmap,
        target: *mut FT_Bitmap,
        alignment: FT_Int,
    ) -> FT_Error;
    fn FT_Bitmap_Done(library: FT_Library, bitmap: *mut FT_Bitmap) -> FT_Error;
    fn FT_Load_Sfnt_Table(
        face: FT_Face,
        tag: FT_ULong,
        offset: FT_Long,
        buffer: *mut FT_Byte,
        length: *mut FT_ULong,
    ) -> FT_Error;
    fn FT_Get_X11_Font_Format(face: FT_Face) -> *const libc::c_char;
    fn FT_Get_Var_Blend_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *mut FT_Fixed,
    ) -> FT_Error;
    fn FT_Set_Var_Design_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *mut FT_Fixed,
    ) -> FT_Error;
    fn FT_Done_MM_Var(library: FT_Library, amaster: *mut FT_MM_Var) -> FT_Error;
    fn FT_Get_Var_Design_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *mut FT_Fixed,
    ) -> FT_Error;
    fn FT_Get_MM_Var(face: FT_Face, amaster: *mut *mut FT_MM_Var) -> FT_Error;
    fn FT_GlyphSlot_Embolden(slot: FT_GlyphSlot);
    fn FT_GlyphSlot_Oblique(slot: FT_GlyphSlot);
    fn FT_Library_SetLcdFilter(library: FT_Library, filter: FT_LcdFilter) -> FT_Error;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type pixman_bool_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_unscaled_font_backend {
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t>,
}
pub type cairo_unscaled_font_backend_t = _cairo_unscaled_font_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_unscaled_font {
    pub hash_entry: cairo_hash_entry_t,
    pub ref_count: cairo_reference_count_t,
    pub backend: *const cairo_unscaled_font_backend_t,
}
pub type cairo_unscaled_font_t = _cairo_unscaled_font;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_ft_unscaled_font_map_t = _cairo_ft_unscaled_font_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ft_unscaled_font_map {
    pub hash_table: *mut cairo_hash_table_t,
    pub ft_library: FT_Library,
    pub num_open_faces: libc::c_int,
}
pub type FT_Library = *mut FT_LibraryRec_;
pub type FT_Error = libc::c_int;
pub type cairo_ft_unscaled_font_t = _cairo_ft_unscaled_font;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_ft_unscaled_font {
    pub base: cairo_unscaled_font_t,
    pub from_face: cairo_bool_t,
    pub face: FT_Face,
    pub filename: *mut libc::c_char,
    pub id: libc::c_int,
    pub have_scale: cairo_bool_t,
    pub current_scale: cairo_matrix_t,
    pub x_scale: libc::c_double,
    pub y_scale: libc::c_double,
    pub have_shape: cairo_bool_t,
    pub current_shape: cairo_matrix_t,
    pub Current_Shape: FT_Matrix,
    #[bitfield(name = "have_color_set", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "have_color", ty = "libc::c_uint", bits = "1..=1")]
    pub have_color_set_have_color: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub variations: *mut FT_Fixed,
    pub num_palettes: libc::c_uint,
    pub mutex: cairo_mutex_t,
    pub lock_count: libc::c_int,
    pub faces: *mut cairo_ft_font_face_t,
}
pub type cairo_ft_font_face_t = _cairo_ft_font_face;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ft_font_face {
    pub base: cairo_font_face_t,
    pub unscaled: *mut cairo_ft_unscaled_font_t,
    pub ft_options: cairo_ft_options_t,
    pub next: *mut cairo_ft_font_face_t,
    pub pattern: *mut FcPattern,
    pub resolved_font_face: *mut cairo_font_face_t,
    pub resolved_config: *mut FcConfig,
}
pub type FcConfig = _FcConfig;
pub type FcPattern = _FcPattern;
pub type cairo_ft_options_t = _cairo_ft_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ft_options {
    pub base: cairo_font_options_t,
    pub load_flags: libc::c_uint,
    pub synth_flags: libc::c_uint,
}
pub type FT_Fixed = libc::c_long;
pub type FT_Matrix = FT_Matrix_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Matrix_ {
    pub xx: FT_Fixed,
    pub xy: FT_Fixed,
    pub yx: FT_Fixed,
    pub yy: FT_Fixed,
}
pub type FT_Face = *mut FT_FaceRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_FaceRec_ {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,
    pub face_flags: FT_Long,
    pub style_flags: FT_Long,
    pub num_glyphs: FT_Long,
    pub family_name: *mut FT_String,
    pub style_name: *mut FT_String,
    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *mut FT_Bitmap_Size,
    pub num_charmaps: FT_Int,
    pub charmaps: *mut FT_CharMap,
    pub generic: FT_Generic,
    pub bbox: FT_BBox,
    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,
    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,
    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,
    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,
    pub sizes_list: FT_ListRec,
    pub autohint: FT_Generic,
    pub extensions: *mut libc::c_void,
    pub internal: FT_Face_Internal,
}
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
pub type FT_Generic = FT_Generic_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Generic_ {
    pub data: *mut libc::c_void,
    pub finalizer: FT_Generic_Finalizer,
}
pub type FT_Generic_Finalizer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type FT_ListRec = FT_ListRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListRec_ {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}
pub type FT_ListNode = *mut FT_ListNodeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListNodeRec_ {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *mut libc::c_void,
}
pub type FT_Stream = *mut FT_StreamRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_StreamRec_ {
    pub base: *mut libc::c_uchar,
    pub size: libc::c_ulong,
    pub pos: libc::c_ulong,
    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,
    pub memory: FT_Memory,
    pub cursor: *mut libc::c_uchar,
    pub limit: *mut libc::c_uchar,
}
pub type FT_Memory = *mut FT_MemoryRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_MemoryRec_ {
    pub user: *mut libc::c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}
pub type FT_Realloc_Func = Option::<
    unsafe extern "C" fn(
        FT_Memory,
        libc::c_long,
        libc::c_long,
        *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type FT_Free_Func = Option::<
    unsafe extern "C" fn(FT_Memory, *mut libc::c_void) -> (),
>;
pub type FT_Alloc_Func = Option::<
    unsafe extern "C" fn(FT_Memory, libc::c_long) -> *mut libc::c_void,
>;
pub type FT_Stream_CloseFunc = Option::<unsafe extern "C" fn(FT_Stream) -> ()>;
pub type FT_Stream_IoFunc = Option::<
    unsafe extern "C" fn(
        FT_Stream,
        libc::c_ulong,
        *mut libc::c_uchar,
        libc::c_ulong,
    ) -> libc::c_ulong,
>;
pub type FT_StreamDesc = FT_StreamDesc_;
#[derive(Copy, Clone)]
#[repr(C)]
pub union FT_StreamDesc_ {
    pub value: libc::c_long,
    pub pointer: *mut libc::c_void,
}
pub type FT_Driver = *mut FT_DriverRec_;
pub type FT_CharMap = *mut FT_CharMapRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_CharMapRec_ {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}
pub type FT_UShort = libc::c_ushort;
pub type FT_Encoding = FT_Encoding_;
pub type FT_Encoding_ = libc::c_uint;
pub const FT_ENCODING_APPLE_ROMAN: FT_Encoding_ = 1634889070;
pub const FT_ENCODING_OLD_LATIN_2: FT_Encoding_ = 1818326066;
pub const FT_ENCODING_ADOBE_LATIN_1: FT_Encoding_ = 1818326065;
pub const FT_ENCODING_ADOBE_CUSTOM: FT_Encoding_ = 1094992451;
pub const FT_ENCODING_ADOBE_EXPERT: FT_Encoding_ = 1094992453;
pub const FT_ENCODING_ADOBE_STANDARD: FT_Encoding_ = 1094995778;
pub const FT_ENCODING_MS_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_MS_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_MS_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_MS_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_MS_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_PRC: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_UNICODE: FT_Encoding_ = 1970170211;
pub const FT_ENCODING_MS_SYMBOL: FT_Encoding_ = 1937337698;
pub const FT_ENCODING_NONE: FT_Encoding_ = 0;
pub type FT_Size = *mut FT_SizeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_SizeRec_ {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}
pub type FT_Size_Internal = *mut FT_Size_InternalRec_;
pub type FT_Size_Metrics = FT_Size_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Size_Metrics_ {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,
    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,
    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos,
}
pub type FT_Pos = libc::c_long;
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_GlyphSlotRec_ {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub glyph_index: FT_UInt,
    pub generic: FT_Generic,
    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,
    pub format: FT_Glyph_Format,
    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,
    pub outline: FT_Outline,
    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,
    pub control_data: *mut libc::c_void,
    pub control_len: libc::c_long,
    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,
    pub other: *mut libc::c_void,
    pub internal: FT_Slot_Internal,
}
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec_;
pub type FT_SubGlyph = *mut FT_SubGlyphRec_;
pub type FT_UInt = libc::c_uint;
pub type FT_Outline = FT_Outline_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Outline_ {
    pub n_contours: libc::c_short,
    pub n_points: libc::c_short,
    pub points: *mut FT_Vector,
    pub tags: *mut libc::c_char,
    pub contours: *mut libc::c_short,
    pub flags: libc::c_int,
}
pub type FT_Vector = FT_Vector_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Vector_ {
    pub x: FT_Pos,
    pub y: FT_Pos,
}
pub type FT_Int = libc::c_int;
pub type FT_Bitmap = FT_Bitmap_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_ {
    pub rows: libc::c_uint,
    pub width: libc::c_uint,
    pub pitch: libc::c_int,
    pub buffer: *mut libc::c_uchar,
    pub num_grays: libc::c_ushort,
    pub pixel_mode: libc::c_uchar,
    pub palette_mode: libc::c_uchar,
    pub palette: *mut libc::c_void,
}
pub type FT_Glyph_Format = FT_Glyph_Format_;
pub type FT_Glyph_Format_ = libc::c_uint;
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format_ = 1886154612;
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format_ = 1869968492;
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format_ = 1651078259;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format_ = 1668246896;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format_ = 0;
pub type FT_Glyph_Metrics = FT_Glyph_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Glyph_Metrics_ {
    pub width: FT_Pos,
    pub height: FT_Pos,
    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,
    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}
pub type FT_Short = libc::c_short;
pub type FT_BBox = FT_BBox_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_BBox_ {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}
pub type FT_Bitmap_Size = FT_Bitmap_Size_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_Size_ {
    pub height: FT_Short,
    pub width: FT_Short,
    pub size: FT_Pos,
    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}
pub type FT_String = libc::c_char;
pub type FT_Long = libc::c_long;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub const FcResultMatch: _FcResult = 0;
pub type FcResult = _FcResult;
pub type _FcResult = libc::c_uint;
pub const FcResultOutOfMemory: _FcResult = 4;
pub const FcResultNoId: _FcResult = 3;
pub const FcResultTypeMismatch: _FcResult = 2;
pub const FcResultNoMatch: _FcResult = 1;
pub type FcChar8 = libc::c_uchar;
pub const CAIRO_FT_SYNTHESIZE_BOLD: C2RustUnnamed_2 = 1;
pub type FcBool = libc::c_int;
pub type FT_MM_Var = FT_MM_Var_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_MM_Var_ {
    pub num_axis: FT_UInt,
    pub num_designs: FT_UInt,
    pub num_namedstyles: FT_UInt,
    pub axis: *mut FT_Var_Axis,
    pub namedstyle: *mut FT_Var_Named_Style,
}
pub type FT_Var_Named_Style = FT_Var_Named_Style_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Var_Named_Style_ {
    pub coords: *mut FT_Fixed,
    pub strid: FT_UInt,
    pub psid: FT_UInt,
}
pub type FT_Var_Axis = FT_Var_Axis_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Var_Axis_ {
    pub name: *mut FT_String,
    pub minimum: FT_Fixed,
    pub def: FT_Fixed,
    pub maximum: FT_Fixed,
    pub tag: FT_ULong,
    pub strid: FT_UInt,
}
pub type FT_ULong = libc::c_ulong;
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type FcValue = _FcValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FcValue {
    pub type_0: FcType,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: *const FcChar8,
    pub i: libc::c_int,
    pub b: FcBool,
    pub d: libc::c_double,
    pub m: *const FcMatrix,
    pub c: *const FcCharSet,
    pub f: *mut libc::c_void,
    pub l: *const FcLangSet,
    pub r: *const FcRange,
}
pub type FcRange = _FcRange;
pub type FcLangSet = _FcLangSet;
pub type FcCharSet = _FcCharSet;
pub type FcMatrix = _FcMatrix;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FcMatrix {
    pub xx: libc::c_double,
    pub xy: libc::c_double,
    pub yx: libc::c_double,
    pub yy: libc::c_double,
}
pub type FcType = _FcType;
pub type _FcType = libc::c_int;
pub const FcTypeRange: _FcType = 9;
pub const FcTypeLangSet: _FcType = 8;
pub const FcTypeFTFace: _FcType = 7;
pub const FcTypeCharSet: _FcType = 6;
pub const FcTypeMatrix: _FcType = 5;
pub const FcTypeBool: _FcType = 4;
pub const FcTypeString: _FcType = 3;
pub const FcTypeDouble: _FcType = 2;
pub const FcTypeInteger: _FcType = 1;
pub const FcTypeVoid: _FcType = 0;
pub const FcTypeUnknown: _FcType = -1;
pub type FcMatchKind = _FcMatchKind;
pub type _FcMatchKind = libc::c_uint;
pub const FcMatchKindBegin: _FcMatchKind = 0;
pub const FcMatchKindEnd: _FcMatchKind = 3;
pub const FcMatchScan: _FcMatchKind = 2;
pub const FcMatchFont: _FcMatchKind = 1;
pub const FcMatchPattern: _FcMatchKind = 0;
pub type cairo_ft_font_transform_t = _cairo_ft_font_transform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ft_font_transform {
    pub x_scale: libc::c_double,
    pub y_scale: libc::c_double,
    pub shape: [[libc::c_double; 2]; 2],
}
pub type cairo_ft_scaled_font_t = _cairo_ft_scaled_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ft_scaled_font {
    pub base: cairo_scaled_font_t,
    pub unscaled: *mut cairo_ft_unscaled_font_t,
    pub ft_options: cairo_ft_options_t,
}
pub const FT_Err_Out_Of_Memory: C2RustUnnamed_1 = 64;
pub type cairo_hash_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
pub const FT_Err_Ok: C2RustUnnamed_1 = 0;
pub type FT_Pointer = *mut libc::c_void;
pub type FT_Byte = libc::c_uchar;
pub type FcChar32 = libc::c_uint;
pub type FT_Outline_Funcs = FT_Outline_Funcs_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Outline_Funcs_ {
    pub move_to: FT_Outline_MoveToFunc,
    pub line_to: FT_Outline_LineToFunc,
    pub conic_to: FT_Outline_ConicToFunc,
    pub cubic_to: FT_Outline_CubicToFunc,
    pub shift: libc::c_int,
    pub delta: FT_Pos,
}
pub type FT_Outline_CubicToFunc = Option::<
    unsafe extern "C" fn(
        *const FT_Vector,
        *const FT_Vector,
        *const FT_Vector,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type FT_Outline_ConicToFunc = Option::<
    unsafe extern "C" fn(
        *const FT_Vector,
        *const FT_Vector,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type FT_Outline_LineToFunc = Option::<
    unsafe extern "C" fn(*const FT_Vector, *mut libc::c_void) -> libc::c_int,
>;
pub type FT_Outline_MoveToFunc = Option::<
    unsafe extern "C" fn(*const FT_Vector, *mut libc::c_void) -> libc::c_int,
>;
pub const CAIRO_FT_SYNTHESIZE_OBLIQUE: C2RustUnnamed_2 = 2;
pub type FT_Int32 = libc::c_int;
pub type FT_Tag = FT_UInt32;
pub type FT_UInt32 = libc::c_uint;
pub type FT_F26Dot6 = libc::c_long;
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub const FT_PIXEL_MODE_GRAY4: FT_Pixel_Mode_ = 4;
pub const FT_PIXEL_MODE_GRAY2: FT_Pixel_Mode_ = 3;
pub const FT_PIXEL_MODE_BGRA: FT_Pixel_Mode_ = 7;
pub const FT_PIXEL_MODE_GRAY: FT_Pixel_Mode_ = 2;
pub const FT_PIXEL_MODE_LCD_V: FT_Pixel_Mode_ = 6;
pub const FT_PIXEL_MODE_LCD: FT_Pixel_Mode_ = 5;
pub const FT_PIXEL_MODE_MONO: FT_Pixel_Mode_ = 1;
pub type FT_Render_Mode = FT_Render_Mode_;
pub type FT_Render_Mode_ = libc::c_uint;
pub const FT_RENDER_MODE_MAX: FT_Render_Mode_ = 6;
pub const FT_RENDER_MODE_SDF: FT_Render_Mode_ = 5;
pub const FT_RENDER_MODE_LCD_V: FT_Render_Mode_ = 4;
pub const FT_RENDER_MODE_LCD: FT_Render_Mode_ = 3;
pub const FT_RENDER_MODE_MONO: FT_Render_Mode_ = 2;
pub const FT_RENDER_MODE_LIGHT: FT_Render_Mode_ = 1;
pub const FT_RENDER_MODE_NORMAL: FT_Render_Mode_ = 0;
pub type FT_LcdFilter = FT_LcdFilter_;
pub type FT_LcdFilter_ = libc::c_uint;
pub const FT_LCD_FILTER_MAX: FT_LcdFilter_ = 17;
pub const FT_LCD_FILTER_LEGACY_0: FT_LcdFilter_ = 16;
pub const FT_LCD_FILTER_LEGACY1: FT_LcdFilter_ = 3;
pub const FT_LCD_FILTER_LIGHT_0: FT_LcdFilter_ = 2;
pub const FT_LCD_FILTER_DEFAULT_0: FT_LcdFilter_ = 1;
pub const FT_LCD_FILTER_NONE_0: FT_LcdFilter_ = 0;
pub type FT_Color = FT_Color_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Color_ {
    pub blue: FT_Byte,
    pub green: FT_Byte,
    pub red: FT_Byte,
    pub alpha: FT_Byte,
}
pub type FT_Palette_Data = FT_Palette_Data_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Palette_Data_ {
    pub num_palettes: FT_UShort,
    pub palette_name_ids: *const FT_UShort,
    pub palette_flags: *const FT_UShort,
    pub num_palette_entries: FT_UShort,
    pub palette_entry_name_ids: *const FT_UShort,
}
pub type FT_Bool = libc::c_uchar;
pub type FT_LayerIterator = FT_LayerIterator_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_LayerIterator_ {
    pub num_layers: FT_UInt,
    pub layer: FT_UInt,
    pub p: *mut FT_Byte,
}
pub type FT_Pixel_Mode_ = libc::c_uint;
pub const FT_PIXEL_MODE_MAX: FT_Pixel_Mode_ = 8;
pub const FT_PIXEL_MODE_NONE: FT_Pixel_Mode_ = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FT_Err_Max: C2RustUnnamed_1 = 187;
pub const FT_Err_Corrupted_Font_Glyphs: C2RustUnnamed_1 = 186;
pub const FT_Err_Corrupted_Font_Header: C2RustUnnamed_1 = 185;
pub const FT_Err_Bbx_Too_Big: C2RustUnnamed_1 = 184;
pub const FT_Err_Missing_Bbx_Field: C2RustUnnamed_1 = 183;
pub const FT_Err_Missing_Encoding_Field: C2RustUnnamed_1 = 182;
pub const FT_Err_Missing_Startchar_Field: C2RustUnnamed_1 = 181;
pub const FT_Err_Missing_Chars_Field: C2RustUnnamed_1 = 180;
pub const FT_Err_Missing_Fontboundingbox_Field: C2RustUnnamed_1 = 179;
pub const FT_Err_Missing_Size_Field: C2RustUnnamed_1 = 178;
pub const FT_Err_Missing_Font_Field: C2RustUnnamed_1 = 177;
pub const FT_Err_Missing_Startfont_Field: C2RustUnnamed_1 = 176;
pub const FT_Err_Glyph_Too_Big: C2RustUnnamed_1 = 164;
pub const FT_Err_No_Unicode_Glyph_Name: C2RustUnnamed_1 = 163;
pub const FT_Err_Ignore: C2RustUnnamed_1 = 162;
pub const FT_Err_Stack_Underflow: C2RustUnnamed_1 = 161;
pub const FT_Err_Syntax_Error: C2RustUnnamed_1 = 160;
pub const FT_Err_Missing_Bitmap: C2RustUnnamed_1 = 157;
pub const FT_Err_DEF_In_Glyf_Bytecode: C2RustUnnamed_1 = 156;
pub const FT_Err_Invalid_Post_Table: C2RustUnnamed_1 = 155;
pub const FT_Err_Invalid_Post_Table_Format: C2RustUnnamed_1 = 154;
pub const FT_Err_Could_Not_Find_Context: C2RustUnnamed_1 = 153;
pub const FT_Err_Invalid_Vert_Metrics: C2RustUnnamed_1 = 152;
pub const FT_Err_Invalid_PPem: C2RustUnnamed_1 = 151;
pub const FT_Err_Invalid_CharMap_Format: C2RustUnnamed_1 = 150;
pub const FT_Err_Invalid_Horiz_Metrics: C2RustUnnamed_1 = 149;
pub const FT_Err_Post_Table_Missing: C2RustUnnamed_1 = 148;
pub const FT_Err_Hmtx_Table_Missing: C2RustUnnamed_1 = 147;
pub const FT_Err_CMap_Table_Missing: C2RustUnnamed_1 = 146;
pub const FT_Err_Name_Table_Missing: C2RustUnnamed_1 = 145;
pub const FT_Err_Locations_Missing: C2RustUnnamed_1 = 144;
pub const FT_Err_Horiz_Header_Missing: C2RustUnnamed_1 = 143;
pub const FT_Err_Table_Missing: C2RustUnnamed_1 = 142;
pub const FT_Err_Too_Many_Instruction_Defs: C2RustUnnamed_1 = 141;
pub const FT_Err_Too_Many_Function_Defs: C2RustUnnamed_1 = 140;
pub const FT_Err_Execution_Too_Long: C2RustUnnamed_1 = 139;
pub const FT_Err_Invalid_CodeRange: C2RustUnnamed_1 = 138;
pub const FT_Err_Nested_DEFS: C2RustUnnamed_1 = 137;
pub const FT_Err_ENDF_In_Exec_Stream: C2RustUnnamed_1 = 136;
pub const FT_Err_Debug_OpCode: C2RustUnnamed_1 = 135;
pub const FT_Err_Invalid_Reference: C2RustUnnamed_1 = 134;
pub const FT_Err_Divide_By_Zero: C2RustUnnamed_1 = 133;
pub const FT_Err_Bad_Argument: C2RustUnnamed_1 = 132;
pub const FT_Err_Code_Overflow: C2RustUnnamed_1 = 131;
pub const FT_Err_Stack_Overflow: C2RustUnnamed_1 = 130;
pub const FT_Err_Too_Few_Arguments: C2RustUnnamed_1 = 129;
pub const FT_Err_Invalid_Opcode: C2RustUnnamed_1 = 128;
pub const FT_Err_Too_Many_Caches: C2RustUnnamed_1 = 112;
pub const FT_Err_Raster_Negative_Height: C2RustUnnamed_1 = 99;
pub const FT_Err_Raster_Overflow: C2RustUnnamed_1 = 98;
pub const FT_Err_Raster_Corrupted: C2RustUnnamed_1 = 97;
pub const FT_Err_Raster_Uninitialized: C2RustUnnamed_1 = 96;
pub const FT_Err_Invalid_Frame_Read: C2RustUnnamed_1 = 88;
pub const FT_Err_Nested_Frame_Access: C2RustUnnamed_1 = 87;
pub const FT_Err_Invalid_Frame_Operation: C2RustUnnamed_1 = 86;
pub const FT_Err_Invalid_Stream_Operation: C2RustUnnamed_1 = 85;
pub const FT_Err_Invalid_Stream_Read: C2RustUnnamed_1 = 84;
pub const FT_Err_Invalid_Stream_Skip: C2RustUnnamed_1 = 83;
pub const FT_Err_Invalid_Stream_Seek: C2RustUnnamed_1 = 82;
pub const FT_Err_Cannot_Open_Stream: C2RustUnnamed_1 = 81;
pub const FT_Err_Unlisted_Object: C2RustUnnamed_1 = 65;
pub const FT_Err_Too_Many_Extensions: C2RustUnnamed_1 = 49;
pub const FT_Err_Too_Many_Drivers: C2RustUnnamed_1 = 48;
pub const FT_Err_Invalid_Stream_Handle: C2RustUnnamed_1 = 40;
pub const FT_Err_Invalid_Cache_Handle: C2RustUnnamed_1 = 39;
pub const FT_Err_Invalid_CharMap_Handle: C2RustUnnamed_1 = 38;
pub const FT_Err_Invalid_Slot_Handle: C2RustUnnamed_1 = 37;
pub const FT_Err_Invalid_Size_Handle: C2RustUnnamed_1 = 36;
pub const FT_Err_Invalid_Face_Handle: C2RustUnnamed_1 = 35;
pub const FT_Err_Invalid_Driver_Handle: C2RustUnnamed_1 = 34;
pub const FT_Err_Invalid_Library_Handle: C2RustUnnamed_1 = 33;
pub const FT_Err_Invalid_Handle: C2RustUnnamed_1 = 32;
pub const FT_Err_Invalid_Pixel_Size: C2RustUnnamed_1 = 23;
pub const FT_Err_Too_Many_Hints: C2RustUnnamed_1 = 22;
pub const FT_Err_Invalid_Composite: C2RustUnnamed_1 = 21;
pub const FT_Err_Invalid_Outline: C2RustUnnamed_1 = 20;
pub const FT_Err_Cannot_Render_Glyph: C2RustUnnamed_1 = 19;
pub const FT_Err_Invalid_Glyph_Format: C2RustUnnamed_1 = 18;
pub const FT_Err_Invalid_Character_Code: C2RustUnnamed_1 = 17;
pub const FT_Err_Invalid_Glyph_Index: C2RustUnnamed_1 = 16;
pub const FT_Err_Missing_Property: C2RustUnnamed_1 = 12;
pub const FT_Err_Missing_Module: C2RustUnnamed_1 = 11;
pub const FT_Err_Array_Too_Large: C2RustUnnamed_1 = 10;
pub const FT_Err_Invalid_Offset: C2RustUnnamed_1 = 9;
pub const FT_Err_Invalid_Table: C2RustUnnamed_1 = 8;
pub const FT_Err_Unimplemented_Feature: C2RustUnnamed_1 = 7;
pub const FT_Err_Invalid_Argument: C2RustUnnamed_1 = 6;
pub const FT_Err_Lower_Module_Version: C2RustUnnamed_1 = 5;
pub const FT_Err_Invalid_Version: C2RustUnnamed_1 = 4;
pub const FT_Err_Invalid_File_Format: C2RustUnnamed_1 = 3;
pub const FT_Err_Unknown_File_Format: C2RustUnnamed_1 = 2;
pub const FT_Err_Cannot_Open_Resource: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_26_6(mut i: uint32_t) -> cairo_fixed_t {
    return (i << 8 as libc::c_int - 6 as libc::c_int) as cairo_fixed_t;
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
unsafe extern "C" fn _cairo_isspace(mut c: libc::c_int) -> libc::c_int {
    return (c == 0x20 as libc::c_int
        || c >= 0x9 as libc::c_int && c <= 0xd as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn be32_to_cpu(mut v: uint32_t) -> uint32_t {
    return cpu_to_be32(v);
}
#[inline]
unsafe extern "C" fn cpu_to_be32(mut v: uint32_t) -> uint32_t {
    return v >> 24 as libc::c_int
        | v >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | v << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
        | v << 24 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn _cairo_is_little_endian() -> cairo_bool_t {
    static mut i: libc::c_int = 1 as libc::c_int;
    return (*(&i as *const libc::c_int as *mut libc::c_char) as libc::c_int
        == 0x1 as libc::c_int) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
unsafe extern "C" fn _cairo_ft_options_init_copy(
    mut options: *mut cairo_ft_options_t,
    mut other: *const cairo_ft_options_t,
) {
    _cairo_font_options_init_copy(&mut (*options).base, &(*other).base);
    (*options).load_flags = (*other).load_flags;
    (*options).synth_flags = (*other).synth_flags;
}
unsafe extern "C" fn _cairo_ft_options_fini(mut options: *mut cairo_ft_options_t) {
    _cairo_font_options_fini(&mut (*options).base);
}
unsafe extern "C" fn _ft_to_cairo_error(mut error: FT_Error) -> cairo_status_t {
    match error {
        64 => return CAIRO_STATUS_NO_MEMORY,
        _ => return CAIRO_STATUS_FREETYPE_ERROR,
    };
}
static mut cairo_ft_unscaled_font_map: *mut cairo_ft_unscaled_font_map_t = 0
    as *const cairo_ft_unscaled_font_map_t as *mut cairo_ft_unscaled_font_map_t;
unsafe extern "C" fn _font_map_release_face_lock_held(
    mut font_map: *mut cairo_ft_unscaled_font_map_t,
    mut unscaled: *mut cairo_ft_unscaled_font_t,
) {
    if !((*unscaled).face).is_null() {
        FT_Done_Face((*unscaled).face);
        let ref mut fresh2 = (*unscaled).face;
        *fresh2 = 0 as FT_Face;
        (*unscaled).have_scale = 0 as libc::c_int;
        let ref mut fresh3 = (*font_map).num_open_faces;
        *fresh3 -= 1;
    }
}
unsafe extern "C" fn _cairo_ft_unscaled_font_map_create() -> cairo_status_t {
    let mut font_map: *mut cairo_ft_unscaled_font_map_t = 0
        as *mut cairo_ft_unscaled_font_map_t;
    if cairo_ft_unscaled_font_map.is_null() {} else {
        __assert_fail(
            b"cairo_ft_unscaled_font_map == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"cairo_status_t _cairo_ft_unscaled_font_map_create(void)\0"))
                .as_ptr(),
        );
    }
    font_map = (if ::std::mem::size_of::<cairo_ft_unscaled_font_map_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_ft_unscaled_font_map_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_ft_unscaled_font_map_t;
    if font_map.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh4 = (*font_map).hash_table;
    *fresh4 = _cairo_hash_table_create(
        Some(
            _cairo_ft_unscaled_font_keys_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if !((*font_map).hash_table).is_null() {
        if !(FT_Init_FreeType(&mut (*font_map).ft_library) != 0) {
            (*font_map).num_open_faces = 0 as libc::c_int;
            cairo_ft_unscaled_font_map = font_map;
            return CAIRO_STATUS_SUCCESS;
        }
    }
    if !((*font_map).hash_table).is_null() {
        _cairo_hash_table_destroy((*font_map).hash_table);
    }
    free(font_map as *mut libc::c_void);
    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
}
unsafe extern "C" fn _cairo_ft_unscaled_font_map_pluck_entry(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut unscaled: *mut cairo_ft_unscaled_font_t = entry
        as *mut cairo_ft_unscaled_font_t;
    let mut font_map: *mut cairo_ft_unscaled_font_map_t = closure
        as *mut cairo_ft_unscaled_font_map_t;
    _cairo_hash_table_remove((*font_map).hash_table, &mut (*unscaled).base.hash_entry);
    if (*unscaled).from_face == 0 {
        _font_map_release_face_lock_held(font_map, unscaled);
    }
    _cairo_ft_unscaled_font_fini(unscaled);
    free(unscaled as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_ft_unscaled_font_map_destroy() {
    let mut font_map: *mut cairo_ft_unscaled_font_map_t = 0
        as *mut cairo_ft_unscaled_font_map_t;
    pthread_mutex_lock(&mut _cairo_ft_unscaled_font_map_mutex);
    font_map = cairo_ft_unscaled_font_map;
    cairo_ft_unscaled_font_map = 0 as *mut cairo_ft_unscaled_font_map_t;
    pthread_mutex_unlock(&mut _cairo_ft_unscaled_font_map_mutex);
    if !font_map.is_null() {
        _cairo_hash_table_foreach(
            (*font_map).hash_table,
            Some(
                _cairo_ft_unscaled_font_map_pluck_entry
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            font_map as *mut libc::c_void,
        );
        if (*font_map).num_open_faces == 0 as libc::c_int {} else {
            __assert_fail(
                b"font_map->num_open_faces == 0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void _cairo_ft_unscaled_font_map_destroy(void)\0"))
                    .as_ptr(),
            );
        }
        FT_Done_FreeType((*font_map).ft_library);
        _cairo_hash_table_destroy((*font_map).hash_table);
        free(font_map as *mut libc::c_void);
    }
}
unsafe extern "C" fn _cairo_ft_unscaled_font_map_lock() -> *mut cairo_ft_unscaled_font_map_t {
    pthread_mutex_lock(&mut _cairo_ft_unscaled_font_map_mutex);
    if cairo_ft_unscaled_font_map.is_null() {
        if _cairo_ft_unscaled_font_map_create() as u64 != 0 {
            pthread_mutex_unlock(&mut _cairo_ft_unscaled_font_map_mutex);
            return 0 as *mut cairo_ft_unscaled_font_map_t;
        }
    }
    return cairo_ft_unscaled_font_map;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_map_unlock() {
    pthread_mutex_unlock(&mut _cairo_ft_unscaled_font_map_mutex);
}
unsafe extern "C" fn _cairo_ft_unscaled_font_init_key(
    mut key: *mut cairo_ft_unscaled_font_t,
    mut from_face: cairo_bool_t,
    mut filename: *mut libc::c_char,
    mut id: libc::c_int,
    mut face: FT_Face,
) {
    let mut hash: uintptr_t = 0;
    (*key).from_face = from_face;
    let ref mut fresh5 = (*key).filename;
    *fresh5 = filename;
    (*key).id = id;
    let ref mut fresh6 = (*key).face;
    *fresh6 = face;
    hash = _cairo_hash_string(filename);
    hash = (hash as libc::c_ulong)
        .wrapping_add(
            (id as uintptr_t).wrapping_mul(1607 as libc::c_int as libc::c_ulong),
        ) as uintptr_t as uintptr_t;
    hash = (hash as libc::c_ulong)
        .wrapping_add(
            (face as uintptr_t).wrapping_mul(2137 as libc::c_int as libc::c_ulong),
        ) as uintptr_t as uintptr_t;
    (*key).base.hash_entry.hash = hash;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_init(
    mut unscaled: *mut cairo_ft_unscaled_font_t,
    mut from_face: cairo_bool_t,
    mut filename: *const libc::c_char,
    mut id: libc::c_int,
    mut face: FT_Face,
) -> cairo_status_t {
    _cairo_unscaled_font_init(&mut (*unscaled).base, &cairo_ft_unscaled_font_backend);
    let ref mut fresh7 = (*unscaled).variations;
    *fresh7 = 0 as *mut FT_Fixed;
    if from_face != 0 {
        (*unscaled).from_face = 1 as libc::c_int;
        _cairo_ft_unscaled_font_init_key(
            unscaled,
            1 as libc::c_int,
            0 as *mut libc::c_char,
            id,
            face,
        );
        (*unscaled)
            .set_have_color(
                (((*face).face_flags & (1 as libc::c_long) << 14 as libc::c_int != 0)
                    as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_uint,
            );
        (*unscaled).set_have_color_set(1 as libc::c_int as libc::c_uint);
        let mut ft_mm_var: *mut FT_MM_Var = 0 as *mut FT_MM_Var;
        if 0 as libc::c_int == FT_Get_MM_Var(face, &mut ft_mm_var) {
            let ref mut fresh8 = (*unscaled).variations;
            *fresh8 = calloc(
                (*ft_mm_var).num_axis as libc::c_ulong,
                ::std::mem::size_of::<FT_Fixed>() as libc::c_ulong,
            ) as *mut FT_Fixed;
            if !((*unscaled).variations).is_null() {
                FT_Get_Var_Design_Coordinates(
                    face,
                    (*ft_mm_var).num_axis,
                    (*unscaled).variations,
                );
            }
            FT_Done_MM_Var((*(*face).glyph).library, ft_mm_var);
        }
    } else {
        let mut filename_copy: *mut libc::c_char = 0 as *mut libc::c_char;
        (*unscaled).from_face = 0 as libc::c_int;
        let ref mut fresh9 = (*unscaled).face;
        *fresh9 = 0 as FT_Face;
        filename_copy = strdup(filename);
        if filename_copy.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        _cairo_ft_unscaled_font_init_key(
            unscaled,
            0 as libc::c_int,
            filename_copy,
            id,
            0 as FT_Face,
        );
        (*unscaled).set_have_color_set(0 as libc::c_int as libc::c_uint);
    }
    (*unscaled).have_scale = 0 as libc::c_int;
    let mut _tmp_mutex: cairo_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *mut __pthread_internal_list,
                        __next: 0 as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    memcpy(
        &mut (*unscaled).mutex as *mut cairo_mutex_t as *mut libc::c_void,
        &mut _tmp_mutex as *mut cairo_mutex_t as *const libc::c_void,
        ::std::mem::size_of::<cairo_mutex_t>() as libc::c_ulong,
    );
    (*unscaled).lock_count = 0 as libc::c_int;
    let ref mut fresh10 = (*unscaled).faces;
    *fresh10 = 0 as *mut cairo_ft_font_face_t;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_fini(
    mut unscaled: *mut cairo_ft_unscaled_font_t,
) {
    if ((*unscaled).face).is_null() {} else {
        __assert_fail(
            b"unscaled->face == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void _cairo_ft_unscaled_font_fini(cairo_ft_unscaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    free((*unscaled).filename as *mut libc::c_void);
    let ref mut fresh11 = (*unscaled).filename;
    *fresh11 = 0 as *mut libc::c_char;
    free((*unscaled).variations as *mut libc::c_void);
    pthread_mutex_destroy(&mut (*unscaled).mutex);
}
unsafe extern "C" fn _cairo_ft_unscaled_font_keys_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> libc::c_int {
    let mut unscaled_a: *const cairo_ft_unscaled_font_t = key_a
        as *const cairo_ft_unscaled_font_t;
    let mut unscaled_b: *const cairo_ft_unscaled_font_t = key_b
        as *const cairo_ft_unscaled_font_t;
    if (*unscaled_a).id == (*unscaled_b).id
        && (*unscaled_a).from_face == (*unscaled_b).from_face
    {
        if (*unscaled_a).from_face != 0 {
            return ((*unscaled_a).face == (*unscaled_b).face) as libc::c_int;
        }
        if ((*unscaled_a).filename).is_null() && ((*unscaled_b).filename).is_null() {
            return 1 as libc::c_int
        } else if ((*unscaled_a).filename).is_null()
            || ((*unscaled_b).filename).is_null()
        {
            return 0 as libc::c_int
        } else {
            return (strcmp((*unscaled_a).filename, (*unscaled_b).filename)
                == 0 as libc::c_int) as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_create_internal(
    mut from_face: cairo_bool_t,
    mut filename: *mut libc::c_char,
    mut id: libc::c_int,
    mut font_face: FT_Face,
    mut out: *mut *mut cairo_ft_unscaled_font_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut key: cairo_ft_unscaled_font_t = cairo_ft_unscaled_font_t {
        base: cairo_unscaled_font_t {
            hash_entry: cairo_hash_entry_t { hash: 0 },
            ref_count: cairo_reference_count_t {
                ref_count: 0,
            },
            backend: 0 as *const cairo_unscaled_font_backend_t,
        },
        from_face: 0,
        face: 0 as *mut FT_FaceRec_,
        filename: 0 as *mut libc::c_char,
        id: 0,
        have_scale: 0,
        current_scale: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        x_scale: 0.,
        y_scale: 0.,
        have_shape: 0,
        current_shape: cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        },
        Current_Shape: FT_Matrix {
            xx: 0,
            xy: 0,
            yx: 0,
            yy: 0,
        },
        have_color_set_have_color: [0; 1],
        c2rust_padding: [0; 7],
        variations: 0 as *mut FT_Fixed,
        num_palettes: 0,
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                },
            },
        },
        lock_count: 0,
        faces: 0 as *mut cairo_ft_font_face_t,
    };
    let mut unscaled: *mut cairo_ft_unscaled_font_t = 0 as *mut cairo_ft_unscaled_font_t;
    let mut font_map: *mut cairo_ft_unscaled_font_map_t = 0
        as *mut cairo_ft_unscaled_font_map_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    font_map = _cairo_ft_unscaled_font_map_lock();
    if font_map.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_ft_unscaled_font_init_key(&mut key, from_face, filename, id, font_face);
    unscaled = _cairo_hash_table_lookup((*font_map).hash_table, &mut key.base.hash_entry)
        as *mut cairo_ft_unscaled_font_t;
    if !unscaled.is_null() {
        _cairo_unscaled_font_reference(&mut (*unscaled).base);
    } else {
        unscaled = (if ::std::mem::size_of::<cairo_ft_unscaled_font_t>() as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
        {
            malloc(::std::mem::size_of::<cairo_ft_unscaled_font_t>() as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut cairo_ft_unscaled_font_t;
        if unscaled.is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            current_block = 11897628446687629572;
        } else {
            status = _cairo_ft_unscaled_font_init(
                unscaled,
                from_face,
                filename,
                id,
                font_face,
            );
            if status as u64 != 0 {
                current_block = 17276706029082809800;
            } else {
                if (*unscaled).base.hash_entry.hash == key.base.hash_entry.hash {} else {
                    __assert_fail(
                        b"unscaled->base.hash_entry.hash == key.base.hash_entry.hash\0"
                            as *const u8 as *const libc::c_char,
                        b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
                        589 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 120],
                            &[libc::c_char; 120],
                        >(
                            b"cairo_status_t _cairo_ft_unscaled_font_create_internal(cairo_bool_t, char *, int, FT_Face, cairo_ft_unscaled_font_t **)\0",
                        ))
                            .as_ptr(),
                    );
                }
                status = _cairo_hash_table_insert(
                    (*font_map).hash_table,
                    &mut (*unscaled).base.hash_entry,
                );
                if status as u64 != 0 {
                    _cairo_ft_unscaled_font_fini(unscaled);
                    current_block = 17276706029082809800;
                } else {
                    current_block = 6747398143295752582;
                }
            }
            match current_block {
                6747398143295752582 => {}
                _ => {
                    free(unscaled as *mut libc::c_void);
                    current_block = 11897628446687629572;
                }
            }
        }
        match current_block {
            6747398143295752582 => {}
            _ => {
                _cairo_ft_unscaled_font_map_unlock();
                return status;
            }
        }
    }
    _cairo_ft_unscaled_font_map_unlock();
    *out = unscaled;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_create_for_pattern(
    mut pattern: *mut FcPattern,
    mut out: *mut *mut cairo_ft_unscaled_font_t,
) -> cairo_status_t {
    let mut font_face: FT_Face = 0 as FT_Face;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: libc::c_int = 0 as libc::c_int;
    let mut ret: FcResult = FcResultMatch;
    ret = FcPatternGetFTFace(
        pattern,
        b"ftface\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut font_face,
    );
    if !(ret as libc::c_uint == FcResultMatch as libc::c_int as libc::c_uint) {
        if ret as libc::c_uint == FcResultOutOfMemory as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        ret = FcPatternGetString(
            pattern,
            b"file\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut filename as *mut *mut libc::c_char as *mut *mut FcChar8,
        );
        if ret as libc::c_uint == FcResultOutOfMemory as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        if ret as libc::c_uint == FcResultMatch as libc::c_int as libc::c_uint {
            if access(filename, 4 as libc::c_int) == 0 as libc::c_int {
                ret = FcPatternGetInteger(
                    pattern,
                    b"index\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    &mut id,
                );
                if ret as libc::c_uint
                    == FcResultOutOfMemory as libc::c_int as libc::c_uint
                {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
            } else {
                return _cairo_error(CAIRO_STATUS_FILE_NOT_FOUND)
            }
        } else {
            *out = 0 as *mut cairo_ft_unscaled_font_t;
            return CAIRO_STATUS_SUCCESS;
        }
    }
    return _cairo_ft_unscaled_font_create_internal(
        (font_face != 0 as *mut libc::c_void as FT_Face) as libc::c_int,
        filename,
        id,
        font_face,
        out,
    );
}
unsafe extern "C" fn _cairo_ft_unscaled_font_create_from_face(
    mut face: FT_Face,
    mut out: *mut *mut cairo_ft_unscaled_font_t,
) -> cairo_status_t {
    return _cairo_ft_unscaled_font_create_internal(
        1 as libc::c_int,
        0 as *mut libc::c_char,
        (*face).face_index as libc::c_int,
        face,
        out,
    );
}
unsafe extern "C" fn _cairo_ft_unscaled_font_destroy(
    mut abstract_font: *mut libc::c_void,
) -> cairo_bool_t {
    let mut unscaled: *mut cairo_ft_unscaled_font_t = abstract_font
        as *mut cairo_ft_unscaled_font_t;
    let mut font_map: *mut cairo_ft_unscaled_font_map_t = 0
        as *mut cairo_ft_unscaled_font_map_t;
    font_map = _cairo_ft_unscaled_font_map_lock();
    if !font_map.is_null() {} else {
        __assert_fail(
            b"font_map != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            667 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"cairo_bool_t _cairo_ft_unscaled_font_destroy(void *)\0"))
                .as_ptr(),
        );
    }
    if !(::std::intrinsics::atomic_xsub(
        &mut (*unscaled).base.ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        _cairo_ft_unscaled_font_map_unlock();
        return 0 as libc::c_int;
    }
    _cairo_hash_table_remove((*font_map).hash_table, &mut (*unscaled).base.hash_entry);
    if (*unscaled).from_face != 0 {
        if !((*unscaled).faces).is_null() && ((*(*unscaled).faces).unscaled).is_null() {
            if ((*(*unscaled).faces).next).is_null() {} else {
                __assert_fail(
                    b"unscaled->faces->next == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
                    683 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 53],
                        &[libc::c_char; 53],
                    >(b"cairo_bool_t _cairo_ft_unscaled_font_destroy(void *)\0"))
                        .as_ptr(),
                );
            }
            cairo_font_face_destroy(&mut (*(*unscaled).faces).base);
        }
    } else {
        _font_map_release_face_lock_held(font_map, unscaled);
    }
    let ref mut fresh12 = (*unscaled).face;
    *fresh12 = 0 as FT_Face;
    _cairo_ft_unscaled_font_map_unlock();
    _cairo_ft_unscaled_font_fini(unscaled);
    return 1 as libc::c_int;
}
unsafe extern "C" fn _has_unlocked_face(mut entry: *const libc::c_void) -> cairo_bool_t {
    let mut unscaled: *const cairo_ft_unscaled_font_t = entry
        as *const cairo_ft_unscaled_font_t;
    return ((*unscaled).from_face == 0 && (*unscaled).lock_count == 0 as libc::c_int
        && !((*unscaled).face).is_null()) as libc::c_int;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_lock_face(
    mut unscaled: *mut cairo_ft_unscaled_font_t,
) -> FT_Face {
    let mut font_map: *mut cairo_ft_unscaled_font_map_t = 0
        as *mut cairo_ft_unscaled_font_map_t;
    let mut face: FT_Face = 0 as FT_Face;
    let mut error: FT_Error = 0;
    pthread_mutex_lock(&mut (*unscaled).mutex);
    let ref mut fresh13 = (*unscaled).lock_count;
    *fresh13 += 1;
    if !((*unscaled).face).is_null() {
        return (*unscaled).face;
    }
    if (*unscaled).from_face == 0 {} else {
        __assert_fail(
            b"!unscaled->from_face\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            726 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"FT_Face _cairo_ft_unscaled_font_lock_face(cairo_ft_unscaled_font_t *)\0",
            ))
                .as_ptr(),
        );
    }
    font_map = _cairo_ft_unscaled_font_map_lock();
    if !font_map.is_null() {} else {
        __assert_fail(
            b"font_map != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            730 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"FT_Face _cairo_ft_unscaled_font_lock_face(cairo_ft_unscaled_font_t *)\0",
            ))
                .as_ptr(),
        );
    }
    while (*font_map).num_open_faces >= 10 as libc::c_int {
        let mut entry: *mut cairo_ft_unscaled_font_t = 0
            as *mut cairo_ft_unscaled_font_t;
        entry = _cairo_hash_table_random_entry(
            (*font_map).hash_table,
            Some(
                _has_unlocked_face
                    as unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
            ),
        ) as *mut cairo_ft_unscaled_font_t;
        if entry.is_null() {
            break;
        }
        _font_map_release_face_lock_held(font_map, entry);
    }
    _cairo_ft_unscaled_font_map_unlock();
    error = FT_New_Face(
        (*font_map).ft_library,
        (*unscaled).filename,
        (*unscaled).id as FT_Long,
        &mut face,
    );
    if error != 0 {
        let ref mut fresh14 = (*unscaled).lock_count;
        *fresh14 -= 1;
        pthread_mutex_unlock(&mut (*unscaled).mutex);
        let mut status__: cairo_status_t = _cairo_error(_ft_to_cairo_error(error));
        return 0 as FT_Face;
    }
    let ref mut fresh15 = (*unscaled).face;
    *fresh15 = face;
    (*unscaled)
        .set_have_color(
            (((*face).face_flags & (1 as libc::c_long) << 14 as libc::c_int != 0)
                as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_uint,
        );
    (*unscaled).set_have_color_set(1 as libc::c_int as libc::c_uint);
    let ref mut fresh16 = (*font_map).num_open_faces;
    *fresh16 += 1;
    return face;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_unlock_face(
    mut unscaled: *mut cairo_ft_unscaled_font_t,
) {
    if (*unscaled).lock_count > 0 as libc::c_int {} else {
        __assert_fail(
            b"unscaled->lock_count > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            774 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void _cairo_ft_unscaled_font_unlock_face(cairo_ft_unscaled_font_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh17 = (*unscaled).lock_count;
    *fresh17 -= 1;
    pthread_mutex_unlock(&mut (*unscaled).mutex);
}
unsafe extern "C" fn _compute_transform(
    mut sf: *mut cairo_ft_font_transform_t,
    mut scale: *mut cairo_matrix_t,
    mut unscaled: *mut cairo_ft_unscaled_font_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut x_scale: libc::c_double = 0.;
    let mut y_scale: libc::c_double = 0.;
    let mut normalized: cairo_matrix_t = *scale;
    status = _cairo_matrix_compute_basis_scale_factors(
        scale,
        &mut x_scale,
        &mut y_scale,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    if x_scale < 1.0f64 {
        x_scale = 1.0f64;
    }
    if y_scale < 1.0f64 {
        y_scale = 1.0f64;
    }
    if !unscaled.is_null()
        && (*(*unscaled).face).face_flags & (1 as libc::c_long) << 0 as libc::c_int
            == 0 as libc::c_int as libc::c_long
    {
        let mut min_distance: libc::c_double = 1.7976931348623157e+308f64;
        let mut magnify: cairo_bool_t = 1 as libc::c_int;
        let mut i: libc::c_int = 0;
        let mut best_x_size: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut best_y_size: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < (*(*unscaled).face).num_fixed_sizes {
            let mut x_size: libc::c_double = (*((*(*unscaled).face).available_sizes)
                .offset(i as isize))
                .x_ppem as libc::c_double / 64.0f64;
            let mut y_size: libc::c_double = (*((*(*unscaled).face).available_sizes)
                .offset(i as isize))
                .y_ppem as libc::c_double / 64.0f64;
            let mut distance: libc::c_double = y_size - y_scale;
            if magnify != 0 && distance >= 0 as libc::c_int as libc::c_double
                || fabs(distance) <= min_distance
            {
                magnify = (distance < 0 as libc::c_int as libc::c_double) as libc::c_int;
                min_distance = fabs(distance);
                best_x_size = x_size;
                best_y_size = y_size;
            }
            i += 1;
        }
        x_scale = best_x_size;
        y_scale = best_y_size;
    }
    (*sf).x_scale = x_scale;
    (*sf).y_scale = y_scale;
    cairo_matrix_scale(&mut normalized, 1.0f64 / x_scale, 1.0f64 / y_scale);
    _cairo_matrix_get_affine(
        &mut normalized,
        &mut *(*((*sf).shape).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *(*((*sf).shape).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        &mut *(*((*sf).shape).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        &mut *(*((*sf).shape).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ft_unscaled_font_set_scale(
    mut unscaled: *mut cairo_ft_unscaled_font_t,
    mut scale: *mut cairo_matrix_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut sf: cairo_ft_font_transform_t = cairo_ft_font_transform_t {
        x_scale: 0.,
        y_scale: 0.,
        shape: [[0.; 2]; 2],
    };
    let mut mat: FT_Matrix = FT_Matrix {
        xx: 0,
        xy: 0,
        yx: 0,
        yy: 0,
    };
    let mut error: FT_Error = 0;
    if !((*unscaled).face).is_null() {} else {
        __assert_fail(
            b"unscaled->face != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            869 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"cairo_status_t _cairo_ft_unscaled_font_set_scale(cairo_ft_unscaled_font_t *, cairo_matrix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*unscaled).have_scale != 0 && (*scale).xx == (*unscaled).current_scale.xx
        && (*scale).yx == (*unscaled).current_scale.yx
        && (*scale).xy == (*unscaled).current_scale.xy
        && (*scale).yy == (*unscaled).current_scale.yy
    {
        return CAIRO_STATUS_SUCCESS;
    }
    (*unscaled).have_scale = 1 as libc::c_int;
    (*unscaled).current_scale = *scale;
    status = _compute_transform(&mut sf, scale, unscaled);
    if status as u64 != 0 {
        return status;
    }
    (*unscaled).x_scale = sf.x_scale;
    (*unscaled).y_scale = sf.y_scale;
    mat
        .xx = (sf.shape[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * 65536.0f64) as FT_Fixed;
    mat
        .yx = -((sf.shape[0 as libc::c_int as usize][1 as libc::c_int as usize]
        * 65536.0f64) as FT_Fixed);
    mat
        .xy = -((sf.shape[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * 65536.0f64) as FT_Fixed);
    mat
        .yy = (sf.shape[1 as libc::c_int as usize][1 as libc::c_int as usize]
        * 65536.0f64) as FT_Fixed;
    (*unscaled)
        .have_shape = (mat.xx != 0x10000 as libc::c_int as libc::c_long
        || mat.yx != 0 as libc::c_int as libc::c_long
        || mat.xy != 0 as libc::c_int as libc::c_long
        || mat.yy != 0x10000 as libc::c_int as libc::c_long) as libc::c_int;
    (*unscaled).Current_Shape = mat;
    cairo_matrix_init(
        &mut (*unscaled).current_shape,
        sf.shape[0 as libc::c_int as usize][0 as libc::c_int as usize],
        sf.shape[0 as libc::c_int as usize][1 as libc::c_int as usize],
        sf.shape[1 as libc::c_int as usize][0 as libc::c_int as usize],
        sf.shape[1 as libc::c_int as usize][1 as libc::c_int as usize],
        0.0f64,
        0.0f64,
    );
    FT_Set_Transform((*unscaled).face, &mut mat, 0 as *mut FT_Vector);
    error = FT_Set_Char_Size(
        (*unscaled).face,
        (sf.x_scale * 64.0f64 + 0.5f64) as FT_F26Dot6,
        (sf.y_scale * 64.0f64 + 0.5f64) as FT_F26Dot6,
        0 as libc::c_int as FT_UInt,
        0 as libc::c_int as FT_UInt,
    );
    if error != 0 {
        return _cairo_error(_ft_to_cairo_error(error));
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _compute_xrender_bitmap_size(
    mut target: *mut FT_Bitmap,
    mut slot: FT_GlyphSlot,
    mut mode: FT_Render_Mode,
) -> libc::c_int {
    let mut ftbit: *mut FT_Bitmap = 0 as *mut FT_Bitmap;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    if (*slot).format as libc::c_uint
        != FT_GLYPH_FORMAT_BITMAP as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    ftbit = &mut (*slot).bitmap;
    width = (*ftbit).width as libc::c_int;
    height = (*ftbit).rows as libc::c_int;
    pitch = width + 3 as libc::c_int & !(3 as libc::c_int);
    let mut current_block_19: u64;
    match (*ftbit).pixel_mode as libc::c_int {
        1 => {
            if mode as libc::c_uint == FT_RENDER_MODE_MONO as libc::c_int as libc::c_uint
            {
                pitch = (width + 31 as libc::c_int & !(31 as libc::c_int))
                    >> 3 as libc::c_int;
                current_block_19 = 11194104282611034094;
            } else {
                current_block_19 = 5352616937031334073;
            }
        }
        2 => {
            current_block_19 = 5352616937031334073;
        }
        5 => {
            if mode as libc::c_uint != FT_RENDER_MODE_LCD as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            width /= 3 as libc::c_int;
            pitch = width * 4 as libc::c_int;
            current_block_19 = 11194104282611034094;
        }
        6 => {
            if mode as libc::c_uint
                != FT_RENDER_MODE_LCD_V as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            height /= 3 as libc::c_int;
            pitch = width * 4 as libc::c_int;
            current_block_19 = 11194104282611034094;
        }
        7 => {
            pitch = width * 4 as libc::c_int;
            current_block_19 = 11194104282611034094;
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block_19 {
        5352616937031334073 => {
            if mode as libc::c_uint == FT_RENDER_MODE_LCD as libc::c_int as libc::c_uint
                || mode as libc::c_uint
                    == FT_RENDER_MODE_LCD_V as libc::c_int as libc::c_uint
            {
                pitch = width * 4 as libc::c_int;
            }
        }
        _ => {}
    }
    (*target).width = width as libc::c_uint;
    (*target).rows = height as libc::c_uint;
    (*target).pitch = pitch;
    let ref mut fresh18 = (*target).buffer;
    *fresh18 = 0 as *mut libc::c_uchar;
    return pitch * height;
}
unsafe extern "C" fn _fill_xrender_bitmap(
    mut target: *mut FT_Bitmap,
    mut slot: FT_GlyphSlot,
    mut mode: FT_Render_Mode,
    mut bgr: libc::c_int,
) {
    let mut ftbit: *mut FT_Bitmap = &mut (*slot).bitmap;
    let mut srcLine: *mut libc::c_uchar = (*ftbit).buffer;
    let mut dstLine: *mut libc::c_uchar = (*target).buffer;
    let mut src_pitch: libc::c_int = (*ftbit).pitch;
    let mut width: libc::c_int = (*target).width as libc::c_int;
    let mut height: libc::c_int = (*target).rows as libc::c_int;
    let mut pitch: libc::c_int = (*target).pitch;
    let mut subpixel: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    subpixel = (mode as libc::c_uint == FT_RENDER_MODE_LCD as libc::c_int as libc::c_uint
        || mode as libc::c_uint == FT_RENDER_MODE_LCD_V as libc::c_int as libc::c_uint)
        as libc::c_int;
    if src_pitch < 0 as libc::c_int {
        srcLine = srcLine
            .offset(
                -((src_pitch as libc::c_uint)
                    .wrapping_mul(
                        ((*ftbit).rows).wrapping_sub(1 as libc::c_int as libc::c_uint),
                    ) as isize),
            );
    }
    (*target).pixel_mode = (*ftbit).pixel_mode;
    match (*ftbit).pixel_mode as libc::c_int {
        1 => {
            if subpixel != 0 {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x: libc::c_int = 0;
                    x = 0 as libc::c_int;
                    while x < width {
                        if *srcLine.offset((x >> 3 as libc::c_int) as isize)
                            as libc::c_int
                            & 0x80 as libc::c_int >> (x & 7 as libc::c_int) != 0
                        {
                            *(dstLine as *mut libc::c_uint)
                                .offset(x as isize) = 0xffffffff as libc::c_uint;
                        }
                        x += 1;
                    }
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
                (*target).pixel_mode = FT_PIXEL_MODE_LCD as libc::c_int as libc::c_uchar;
            } else if mode as libc::c_uint
                == FT_RENDER_MODE_NORMAL as libc::c_int as libc::c_uint
            {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x_0: libc::c_int = 0;
                    x_0 = 0 as libc::c_int;
                    while x_0 < width {
                        if *srcLine.offset((x_0 >> 3 as libc::c_int) as isize)
                            as libc::c_int
                            & 0x80 as libc::c_int >> (x_0 & 7 as libc::c_int) != 0
                        {
                            *dstLine
                                .offset(
                                    x_0 as isize,
                                ) = 0xff as libc::c_int as libc::c_uchar;
                        }
                        x_0 += 1;
                    }
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
                (*target)
                    .pixel_mode = FT_PIXEL_MODE_GRAY as libc::c_int as libc::c_uchar;
            } else {
                let mut bytes: libc::c_int = width + 7 as libc::c_int
                    >> 3 as libc::c_int;
                h = height;
                while h > 0 as libc::c_int {
                    memcpy(
                        dstLine as *mut libc::c_void,
                        srcLine as *const libc::c_void,
                        bytes as libc::c_ulong,
                    );
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
            }
        }
        2 => {
            if subpixel != 0 {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x_1: libc::c_int = 0;
                    let mut dst: *mut libc::c_uint = dstLine as *mut libc::c_uint;
                    x_1 = 0 as libc::c_int;
                    while x_1 < width {
                        let mut pix: libc::c_uint = *srcLine.offset(x_1 as isize)
                            as libc::c_uint;
                        pix |= pix << 8 as libc::c_int;
                        pix |= pix << 16 as libc::c_int;
                        *dst.offset(x_1 as isize) = pix;
                        x_1 += 1;
                    }
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
                (*target).pixel_mode = FT_PIXEL_MODE_LCD as libc::c_int as libc::c_uchar;
            } else {
                h = height;
                while h > 0 as libc::c_int {
                    memcpy(
                        dstLine as *mut libc::c_void,
                        srcLine as *const libc::c_void,
                        width as libc::c_ulong,
                    );
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
            }
        }
        5 => {
            if bgr == 0 {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x_2: libc::c_int = 0;
                    let mut src: *mut libc::c_uchar = srcLine;
                    let mut dst_0: *mut libc::c_uint = dstLine as *mut libc::c_uint;
                    x_2 = 0 as libc::c_int;
                    while x_2 < width {
                        let mut pix_0: libc::c_uint = 0;
                        pix_0 = (*src.offset(0 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int
                            | (*src.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int
                            | *src.offset(2 as libc::c_int as isize) as libc::c_uint
                            | (*src.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int;
                        *dst_0.offset(x_2 as isize) = pix_0;
                        x_2 += 1;
                        src = src.offset(3 as libc::c_int as isize);
                    }
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
            } else {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x_3: libc::c_int = 0;
                    let mut src_0: *mut libc::c_uchar = srcLine;
                    let mut dst_1: *mut libc::c_uint = dstLine as *mut libc::c_uint;
                    x_3 = 0 as libc::c_int;
                    while x_3 < width {
                        let mut pix_1: libc::c_uint = 0;
                        pix_1 = (*src_0.offset(2 as libc::c_int as isize)
                            as libc::c_uint) << 16 as libc::c_int
                            | (*src_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int
                            | *src_0.offset(0 as libc::c_int as isize) as libc::c_uint
                            | (*src_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int;
                        *dst_1.offset(x_3 as isize) = pix_1;
                        x_3 += 1;
                        src_0 = src_0.offset(3 as libc::c_int as isize);
                    }
                    h -= 1;
                    srcLine = srcLine.offset(src_pitch as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
            }
        }
        6 => {
            if bgr == 0 {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x_4: libc::c_int = 0;
                    let mut src_1: *mut libc::c_uchar = srcLine;
                    let mut dst_2: *mut libc::c_uint = dstLine as *mut libc::c_uint;
                    x_4 = 0 as libc::c_int;
                    while x_4 < width {
                        let mut pix_2: libc::c_uint = 0;
                        pix_2 = (*src_1.offset(0 as libc::c_int as isize)
                            as libc::c_uint) << 16 as libc::c_int
                            | (*src_1.offset(src_pitch as isize) as libc::c_uint)
                                << 8 as libc::c_int
                            | *src_1.offset((src_pitch * 2 as libc::c_int) as isize)
                                as libc::c_uint
                            | (*src_1.offset(src_pitch as isize) as libc::c_uint)
                                << 24 as libc::c_int;
                        *dst_2.offset(x_4 as isize) = pix_2;
                        x_4 += 1;
                        src_1 = src_1.offset(1 as libc::c_int as isize);
                    }
                    h -= 1;
                    srcLine = srcLine.offset((3 as libc::c_int * src_pitch) as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
            } else {
                h = height;
                while h > 0 as libc::c_int {
                    let mut x_5: libc::c_int = 0;
                    let mut src_2: *mut libc::c_uchar = srcLine;
                    let mut dst_3: *mut libc::c_uint = dstLine as *mut libc::c_uint;
                    x_5 = 0 as libc::c_int;
                    while x_5 < width {
                        let mut pix_3: libc::c_uint = 0;
                        pix_3 = (*src_2.offset((src_pitch * 2 as libc::c_int) as isize)
                            as libc::c_uint) << 16 as libc::c_int
                            | (*src_2.offset(src_pitch as isize) as libc::c_uint)
                                << 8 as libc::c_int
                            | *src_2.offset(0 as libc::c_int as isize) as libc::c_uint
                            | (*src_2.offset(src_pitch as isize) as libc::c_uint)
                                << 24 as libc::c_int;
                        *dst_3.offset(x_5 as isize) = pix_3;
                        x_5 += 1;
                        src_2 = src_2.offset(1 as libc::c_int as isize);
                    }
                    h -= 1;
                    srcLine = srcLine.offset((3 as libc::c_int * src_pitch) as isize);
                    dstLine = dstLine.offset(pitch as isize);
                }
            }
        }
        7 => {
            h = height;
            while h > 0 as libc::c_int {
                memcpy(
                    dstLine as *mut libc::c_void,
                    srcLine as *const libc::c_void,
                    (width as size_t).wrapping_mul(4 as libc::c_int as libc::c_ulong),
                );
                h -= 1;
                srcLine = srcLine.offset(src_pitch as isize);
                dstLine = dstLine.offset(pitch as isize);
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
                1203 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void _fill_xrender_bitmap(FT_Bitmap *, FT_GlyphSlot, FT_Render_Mode, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn _get_bitmap_surface(
    mut bitmap: *mut FT_Bitmap,
    mut library: FT_Library,
    mut own_buffer: cairo_bool_t,
    mut font_options: *mut cairo_font_options_t,
    mut surface: *mut *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut format: libc::c_int = CAIRO_FORMAT_A8 as libc::c_int;
    let mut stride: libc::c_int = 0;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut component_alpha: cairo_bool_t = 0 as libc::c_int;
    width = (*bitmap).width;
    height = (*bitmap).rows;
    if width == 0 as libc::c_int as libc::c_uint
        || height == 0 as libc::c_int as libc::c_uint
    {
        *surface = cairo_image_surface_create_for_data(
            0 as *mut libc::c_uchar,
            format as cairo_format_t,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) as *mut cairo_image_surface_t;
        return (**surface).base.status;
    }
    let mut current_block_89: u64;
    match (*bitmap).pixel_mode as libc::c_int {
        1 => {
            stride = ((width.wrapping_add(31 as libc::c_int as libc::c_uint)
                & !(31 as libc::c_int) as libc::c_uint) >> 3 as libc::c_int)
                as libc::c_int;
            if own_buffer != 0 {
                data = (*bitmap).buffer;
                if stride == (*bitmap).pitch {} else {
                    __assert_fail(
                        b"stride == bitmap->pitch\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
                        1238 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 124],
                            &[libc::c_char; 124],
                        >(
                            b"cairo_status_t _get_bitmap_surface(FT_Bitmap *, FT_Library, cairo_bool_t, cairo_font_options_t *, cairo_image_surface_t **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            } else {
                data = _cairo_malloc_ab(height as size_t, stride as size_t)
                    as *mut libc::c_uchar;
                if data.is_null() {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
                if stride == (*bitmap).pitch {
                    memcpy(
                        data as *mut libc::c_void,
                        (*bitmap).buffer as *const libc::c_void,
                        (stride as size_t).wrapping_mul(height as libc::c_ulong),
                    );
                } else {
                    let mut i: libc::c_int = 0;
                    let mut source: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut dest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    source = (*bitmap).buffer;
                    dest = data;
                    i = height as libc::c_int;
                    while i != 0 {
                        memcpy(
                            dest as *mut libc::c_void,
                            source as *const libc::c_void,
                            (*bitmap).pitch as libc::c_ulong,
                        );
                        memset(
                            dest.offset((*bitmap).pitch as isize) as *mut libc::c_void,
                            '\0' as i32,
                            (stride - (*bitmap).pitch) as libc::c_ulong,
                        );
                        source = source.offset((*bitmap).pitch as isize);
                        dest = dest.offset(stride as isize);
                        i -= 1;
                    }
                }
            }
            let mut d: *mut uint8_t = data;
            let mut count: libc::c_int = (stride as libc::c_uint).wrapping_mul(height)
                as libc::c_int;
            loop {
                let fresh19 = count;
                count = count - 1;
                if !(fresh19 != 0) {
                    break;
                }
                *d = (((*d as libc::c_ulong).wrapping_mul(0x802 as libc::c_ulong)
                    & 0x22110 as libc::c_ulong
                    | (*d as libc::c_ulong).wrapping_mul(0x8020 as libc::c_ulong)
                        & 0x88440 as libc::c_ulong)
                    .wrapping_mul(0x10101 as libc::c_ulong) >> 16 as libc::c_int)
                    as uint8_t;
                d = d.offset(1);
            }
            format = CAIRO_FORMAT_A1 as libc::c_int;
            current_block_89 = 10153752038087260855;
        }
        5 | 6 | 2 => {
            if (*font_options).antialias as libc::c_uint
                != CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
                || (*bitmap).pixel_mode as libc::c_int
                    == FT_PIXEL_MODE_GRAY as libc::c_int
            {
                stride = (*bitmap).pitch;
                if stride & 3 as libc::c_int != 0 {
                    if own_buffer == 0 {} else {
                        __assert_fail(
                            b"!own_buffer\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-ft-font.c\0" as *const u8
                                as *const libc::c_char,
                            1287 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 124],
                                &[libc::c_char; 124],
                            >(
                                b"cairo_status_t _get_bitmap_surface(FT_Bitmap *, FT_Library, cairo_bool_t, cairo_font_options_t *, cairo_image_surface_t **)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    current_block_89 = 2760422882351422010;
                } else {
                    if own_buffer != 0 {
                        data = (*bitmap).buffer;
                    } else {
                        data = _cairo_malloc_ab(height as size_t, stride as size_t)
                            as *mut libc::c_uchar;
                        if data.is_null() {
                            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                        }
                        memcpy(
                            data as *mut libc::c_void,
                            (*bitmap).buffer as *const libc::c_void,
                            (stride as size_t).wrapping_mul(height as libc::c_ulong),
                        );
                    }
                    format = CAIRO_FORMAT_A8 as libc::c_int;
                    current_block_89 = 10153752038087260855;
                }
            } else {
                data = (*bitmap).buffer;
                stride = (*bitmap).pitch;
                format = CAIRO_FORMAT_ARGB32 as libc::c_int;
                component_alpha = 1 as libc::c_int;
                current_block_89 = 10153752038087260855;
            }
        }
        7 => {
            stride = width.wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_int;
            if own_buffer != 0 {
                data = (*bitmap).buffer;
            } else {
                data = _cairo_malloc_ab(height as size_t, stride as size_t)
                    as *mut libc::c_uchar;
                if data.is_null() {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
                memcpy(
                    data as *mut libc::c_void,
                    (*bitmap).buffer as *const libc::c_void,
                    (stride as size_t).wrapping_mul(height as libc::c_ulong),
                );
            }
            if _cairo_is_little_endian() == 0 {
                let mut i_0: libc::c_uint = 0;
                let mut count_0: libc::c_uint = height.wrapping_mul(width);
                let mut p: *mut uint32_t = data as *mut uint32_t;
                i_0 = 0 as libc::c_int as libc::c_uint;
                while i_0 < count_0 {
                    *p.offset(i_0 as isize) = be32_to_cpu(*p.offset(i_0 as isize));
                    i_0 = i_0.wrapping_add(1);
                }
            }
            format = CAIRO_FORMAT_ARGB32 as libc::c_int;
            current_block_89 = 10153752038087260855;
        }
        3 | 4 => {
            current_block_89 = 2760422882351422010;
        }
        _ => {
            current_block_89 = 6935398789478259777;
        }
    }
    match current_block_89 {
        2760422882351422010 => {
            if own_buffer == 0 && !library.is_null() {
                let mut tmp: FT_Bitmap = FT_Bitmap {
                    rows: 0,
                    width: 0,
                    pitch: 0,
                    buffer: 0 as *mut libc::c_uchar,
                    num_grays: 0,
                    pixel_mode: 0,
                    palette_mode: 0,
                    palette: 0 as *mut libc::c_void,
                };
                let mut align: FT_Int = 0;
                let mut error: FT_Error = 0;
                format = CAIRO_FORMAT_A8 as libc::c_int;
                align = cairo_format_stride_for_width(
                    format as cairo_format_t,
                    (*bitmap).width as libc::c_int,
                );
                FT_Bitmap_New(&mut tmp);
                error = FT_Bitmap_Convert(library, bitmap, &mut tmp, align);
                if error != 0 {
                    return _cairo_error(_ft_to_cairo_error(error));
                }
                FT_Bitmap_Done(library, bitmap);
                *bitmap = tmp;
                stride = (*bitmap).pitch;
                data = _cairo_malloc_ab(height as size_t, stride as size_t)
                    as *mut libc::c_uchar;
                if data.is_null() {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
                if (*bitmap).num_grays as libc::c_int != 256 as libc::c_int {
                    let mut x: libc::c_uint = 0;
                    let mut y: libc::c_uint = 0;
                    let mut mul: libc::c_uint = (255 as libc::c_int
                        / ((*bitmap).num_grays as libc::c_int - 1 as libc::c_int))
                        as libc::c_uint;
                    let mut p_0: *mut FT_Byte = (*bitmap).buffer;
                    y = 0 as libc::c_int as libc::c_uint;
                    while y < height {
                        x = 0 as libc::c_int as libc::c_uint;
                        while x < width {
                            let ref mut fresh20 = *p_0.offset(x as isize);
                            *fresh20 = (*fresh20 as libc::c_uint).wrapping_mul(mul)
                                as FT_Byte as FT_Byte;
                            x = x.wrapping_add(1);
                        }
                        p_0 = p_0.offset((*bitmap).pitch as isize);
                        y = y.wrapping_add(1);
                    }
                }
                memcpy(
                    data as *mut libc::c_void,
                    (*bitmap).buffer as *const libc::c_void,
                    (stride as size_t).wrapping_mul(height as libc::c_ulong),
                );
                current_block_89 = 10153752038087260855;
            } else {
                current_block_89 = 6935398789478259777;
            }
        }
        _ => {}
    }
    match current_block_89 {
        10153752038087260855 => {}
        _ => {
            if own_buffer != 0 {
                free((*bitmap).buffer as *mut libc::c_void);
            }
            return _cairo_error(CAIRO_STATUS_INVALID_FORMAT);
        }
    }
    image = cairo_image_surface_create_for_data(
        data,
        format as cairo_format_t,
        width as libc::c_int,
        height as libc::c_int,
        stride,
    ) as *mut cairo_image_surface_t;
    *surface = image;
    if (*image).base.status as u64 != 0 {
        free(data as *mut libc::c_void);
        return (**surface).base.status;
    }
    if component_alpha != 0 {
        pixman_image_set_component_alpha((*image).pixman_image, 1 as libc::c_int);
    }
    _cairo_image_surface_assume_ownership_of_data(image);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _render_glyph_outline(
    mut face: FT_Face,
    mut font_options: *mut cairo_font_options_t,
    mut surface: *mut *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut rgba: libc::c_int = 0 as libc::c_int;
    let mut lcd_filter: libc::c_int = 1 as libc::c_int;
    let mut glyphslot: FT_GlyphSlot = (*face).glyph;
    let mut outline: *mut FT_Outline = &mut (*glyphslot).outline;
    let mut bitmap: FT_Bitmap = FT_Bitmap {
        rows: 0,
        width: 0,
        pitch: 0,
        buffer: 0 as *mut libc::c_uchar,
        num_grays: 0,
        pixel_mode: 0,
        palette_mode: 0,
        palette: 0 as *mut libc::c_void,
    };
    let mut cbox: FT_BBox = FT_BBox {
        xMin: 0,
        yMin: 0,
        xMax: 0,
        yMax: 0,
    };
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut error: FT_Error = 0;
    let mut library: FT_Library = (*glyphslot).library;
    let mut render_mode: FT_Render_Mode = FT_RENDER_MODE_NORMAL;
    match (*font_options).antialias as libc::c_uint {
        1 => {
            render_mode = FT_RENDER_MODE_MONO;
        }
        3 | 6 => {
            match (*font_options).subpixel_order as libc::c_uint {
                0 | 1 | 2 => {
                    render_mode = FT_RENDER_MODE_LCD;
                }
                3 | 4 => {
                    render_mode = FT_RENDER_MODE_LCD_V;
                }
                _ => {}
            }
            match (*font_options).lcd_filter as libc::c_uint {
                1 => {
                    lcd_filter = 0 as libc::c_int;
                }
                2 => {
                    lcd_filter = 16 as libc::c_int;
                }
                3 => {
                    lcd_filter = 2 as libc::c_int;
                }
                0 | 4 => {
                    lcd_filter = 1 as libc::c_int;
                }
                _ => {}
            }
        }
        0 | 2 | 5 | 4 => {
            render_mode = FT_RENDER_MODE_NORMAL;
        }
        _ => {}
    }
    FT_Outline_Get_CBox(outline, &mut cbox);
    cbox.xMin &= -(64 as libc::c_int) as libc::c_long;
    cbox.yMin &= -(64 as libc::c_int) as libc::c_long;
    cbox
        .xMax = cbox.xMax + 63 as libc::c_int as libc::c_long
        & -(64 as libc::c_int) as libc::c_long;
    cbox
        .yMax = cbox.yMax + 63 as libc::c_int as libc::c_long
        & -(64 as libc::c_int) as libc::c_long;
    width = (cbox.xMax - cbox.xMin >> 6 as libc::c_int) as libc::c_uint;
    height = (cbox.yMax - cbox.yMin >> 6 as libc::c_int) as libc::c_uint;
    if width.wrapping_mul(height) == 0 as libc::c_int as libc::c_uint {
        let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
        match render_mode as libc::c_uint {
            2 => {
                format = CAIRO_FORMAT_A1;
            }
            3 | 4 => {
                format = CAIRO_FORMAT_ARGB32;
            }
            1 | 0 | 6 | 5 | _ => {
                format = CAIRO_FORMAT_A8;
            }
        }
        *surface = cairo_image_surface_create_for_data(
            0 as *mut libc::c_uchar,
            format,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) as *mut cairo_image_surface_t;
        pixman_image_set_component_alpha((**surface).pixman_image, 1 as libc::c_int);
        if (**surface).base.status as u64 != 0 {
            return (**surface).base.status;
        }
    } else {
        let mut bitmap_size: libc::c_int = 0;
        match render_mode as libc::c_uint {
            3 => {
                if (*font_options).subpixel_order as libc::c_uint
                    == CAIRO_SUBPIXEL_ORDER_BGR as libc::c_int as libc::c_uint
                {
                    rgba = 2 as libc::c_int;
                } else {
                    rgba = 1 as libc::c_int;
                }
            }
            4 => {
                if (*font_options).subpixel_order as libc::c_uint
                    == CAIRO_SUBPIXEL_ORDER_VBGR as libc::c_int as libc::c_uint
                {
                    rgba = 4 as libc::c_int;
                } else {
                    rgba = 3 as libc::c_int;
                }
            }
            2 | 1 | 0 | 6 | 5 | _ => {}
        }
        FT_Library_SetLcdFilter(library, lcd_filter as FT_LcdFilter);
        error = FT_Render_Glyph((*face).glyph, render_mode);
        FT_Library_SetLcdFilter(library, FT_LCD_FILTER_NONE_0);
        if error != 0 {
            return _cairo_error(_ft_to_cairo_error(error));
        }
        bitmap_size = _compute_xrender_bitmap_size(
            &mut bitmap,
            (*face).glyph,
            render_mode,
        );
        if bitmap_size < 0 as libc::c_int {
            return _cairo_error(CAIRO_STATUS_INVALID_FORMAT);
        }
        bitmap
            .buffer = calloc(
            1 as libc::c_int as libc::c_ulong,
            bitmap_size as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if (bitmap.buffer).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        _fill_xrender_bitmap(
            &mut bitmap,
            (*face).glyph,
            render_mode,
            (rgba == 2 as libc::c_int || rgba == 4 as libc::c_int) as libc::c_int,
        );
        status = _get_bitmap_surface(
            &mut bitmap,
            0 as FT_Library,
            1 as libc::c_int,
            font_options,
            surface,
        );
        if status as u64 != 0 {
            return status;
        }
        cairo_surface_set_device_offset(
            &mut (**surface).base,
            -(*glyphslot).bitmap_left as libc::c_double,
            (*glyphslot).bitmap_top as libc::c_double,
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _render_glyph_bitmap(
    mut face: FT_Face,
    mut font_options: *mut cairo_font_options_t,
    mut surface: *mut *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut glyphslot: FT_GlyphSlot = (*face).glyph;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut error: FT_Error = 0;
    error = FT_Render_Glyph(glyphslot, FT_RENDER_MODE_NORMAL);
    if error == FT_Err_Out_Of_Memory as libc::c_int {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _get_bitmap_surface(
        &mut (*glyphslot).bitmap,
        (*glyphslot).library,
        0 as libc::c_int,
        font_options,
        surface,
    );
    if status as u64 != 0 {
        return status;
    }
    cairo_surface_set_device_offset(
        &mut (**surface).base,
        -(*glyphslot).bitmap_left as libc::c_double,
        (*glyphslot).bitmap_top as libc::c_double,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _transform_glyph_bitmap(
    mut shape: *mut cairo_matrix_t,
    mut surface: *mut *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut original_to_transformed: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut transformed_to_original: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut old_image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut x: [libc::c_double; 4] = [0.; 4];
    let mut y: [libc::c_double; 4] = [0.; 4];
    let mut origin_x: libc::c_double = 0.;
    let mut origin_y: libc::c_double = 0.;
    let mut orig_width: libc::c_int = 0;
    let mut orig_height: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x_min: libc::c_int = 0;
    let mut y_min: libc::c_int = 0;
    let mut x_max: libc::c_int = 0;
    let mut y_max: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
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
                elements: 0 as *const libc::c_char as *mut libc::c_char,
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
    original_to_transformed = *shape;
    cairo_surface_get_device_offset(&mut (**surface).base, &mut origin_x, &mut origin_y);
    orig_width = (**surface).width;
    orig_height = (**surface).height;
    cairo_matrix_translate(&mut original_to_transformed, -origin_x, -origin_y);
    x[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    y[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    x[1 as libc::c_int as usize] = orig_width as libc::c_double;
    y[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    x[2 as libc::c_int as usize] = orig_width as libc::c_double;
    y[2 as libc::c_int as usize] = orig_height as libc::c_double;
    x[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    y[3 as libc::c_int as usize] = orig_height as libc::c_double;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        cairo_matrix_transform_point(
            &mut original_to_transformed,
            &mut *x.as_mut_ptr().offset(i as isize),
            &mut *y.as_mut_ptr().offset(i as isize),
        );
        i += 1;
    }
    x_min = floor(x[0 as libc::c_int as usize]) as libc::c_int;
    y_min = floor(y[0 as libc::c_int as usize]) as libc::c_int;
    x_max = ceil(x[0 as libc::c_int as usize]) as libc::c_int;
    y_max = ceil(y[0 as libc::c_int as usize]) as libc::c_int;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        if x[i as usize] < x_min as libc::c_double {
            x_min = floor(x[i as usize]) as libc::c_int;
        } else if x[i as usize] > x_max as libc::c_double {
            x_max = ceil(x[i as usize]) as libc::c_int;
        }
        if y[i as usize] < y_min as libc::c_double {
            y_min = floor(y[i as usize]) as libc::c_int;
        } else if y[i as usize] > y_max as libc::c_double {
            y_max = ceil(y[i as usize]) as libc::c_int;
        }
        i += 1;
    }
    original_to_transformed.x0 -= x_min as libc::c_double;
    original_to_transformed.y0 -= y_min as libc::c_double;
    width = x_max - x_min;
    height = y_max - y_min;
    transformed_to_original = original_to_transformed;
    status = cairo_matrix_invert(&mut transformed_to_original);
    if status as u64 != 0 {
        return status;
    }
    if (**surface).format as libc::c_int == CAIRO_FORMAT_ARGB32 as libc::c_int
        && pixman_image_get_component_alpha((**surface).pixman_image) == 0
    {
        image = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, width, height);
    } else {
        image = cairo_image_surface_create(CAIRO_FORMAT_A8, width, height);
    }
    if (*image).status as u64 != 0 {
        return (*image).status;
    }
    _cairo_pattern_init_for_surface(&mut pattern, &mut (**surface).base);
    cairo_pattern_set_matrix(&mut pattern.base, &mut transformed_to_original);
    status = _cairo_surface_paint(
        image,
        CAIRO_OPERATOR_SOURCE,
        &mut pattern.base,
        0 as *const cairo_clip_t,
    );
    _cairo_pattern_fini(&mut pattern.base);
    if status as u64 != 0 {
        cairo_surface_destroy(image);
        return status;
    }
    cairo_matrix_transform_point(
        &mut original_to_transformed,
        &mut origin_x,
        &mut origin_y,
    );
    old_image = *surface;
    *surface = image as *mut cairo_image_surface_t;
    cairo_surface_destroy(&mut (*old_image).base);
    cairo_surface_set_device_offset(
        &mut (**surface).base,
        _cairo_lround(origin_x) as libc::c_double,
        _cairo_lround(origin_y) as libc::c_double,
    );
    return CAIRO_STATUS_SUCCESS;
}
static mut cairo_ft_unscaled_font_backend: cairo_unscaled_font_backend_t = unsafe {
    {
        let mut init = _cairo_unscaled_font_backend {
            destroy: Some(
                _cairo_ft_unscaled_font_destroy
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
        };
        init
    }
};
unsafe extern "C" fn _get_pattern_ft_options(
    mut pattern: *mut FcPattern,
    mut ret: *mut cairo_ft_options_t,
) {
    let mut antialias: FcBool = 0;
    let mut vertical_layout: FcBool = 0;
    let mut hinting: FcBool = 0;
    let mut autohint: FcBool = 0;
    let mut bitmap: FcBool = 0;
    let mut embolden: FcBool = 0;
    let mut ft_options: cairo_ft_options_t = cairo_ft_options_t {
        base: cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        },
        load_flags: 0,
        synth_flags: 0,
    };
    let mut rgba: libc::c_int = 0;
    let mut hintstyle: libc::c_int = 0;
    let mut variations: *mut libc::c_char = 0 as *mut libc::c_char;
    _cairo_font_options_init_default(&mut ft_options.base);
    ft_options.load_flags = 0 as libc::c_int as libc::c_uint;
    ft_options.synth_flags = 0 as libc::c_int as libc::c_uint;
    if FcPatternGetBool(
        pattern,
        b"embeddedbitmap\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut bitmap,
    ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
    {
        bitmap = 0 as libc::c_int;
    }
    if FcPatternGetBool(
        pattern,
        b"antialias\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut antialias,
    ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
    {
        antialias = 1 as libc::c_int;
    }
    if antialias != 0 {
        let mut subpixel_order: cairo_subpixel_order_t = CAIRO_SUBPIXEL_ORDER_DEFAULT;
        let mut lcd_filter: libc::c_int = 0;
        if FcPatternGetBool(
            pattern,
            b"hinting\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut hinting,
        ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
        {
            hinting = 1 as libc::c_int;
        }
        if FcPatternGetInteger(
            pattern,
            b"rgba\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut rgba,
        ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
        {
            rgba = 0 as libc::c_int;
        }
        match rgba {
            1 => {
                subpixel_order = CAIRO_SUBPIXEL_ORDER_RGB;
            }
            2 => {
                subpixel_order = CAIRO_SUBPIXEL_ORDER_BGR;
            }
            3 => {
                subpixel_order = CAIRO_SUBPIXEL_ORDER_VRGB;
            }
            4 => {
                subpixel_order = CAIRO_SUBPIXEL_ORDER_VBGR;
            }
            0 | 5 | _ => {
                subpixel_order = CAIRO_SUBPIXEL_ORDER_DEFAULT;
            }
        }
        if subpixel_order as libc::c_uint
            != CAIRO_SUBPIXEL_ORDER_DEFAULT as libc::c_int as libc::c_uint
        {
            ft_options.base.subpixel_order = subpixel_order;
            ft_options.base.antialias = CAIRO_ANTIALIAS_SUBPIXEL;
        }
        if FcPatternGetInteger(
            pattern,
            b"lcdfilter\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut lcd_filter,
        ) as libc::c_uint == FcResultMatch as libc::c_int as libc::c_uint
        {
            match lcd_filter {
                0 => {
                    ft_options.base.lcd_filter = CAIRO_LCD_FILTER_NONE;
                }
                1 => {
                    ft_options.base.lcd_filter = CAIRO_LCD_FILTER_FIR5;
                }
                2 => {
                    ft_options.base.lcd_filter = CAIRO_LCD_FILTER_FIR3;
                }
                3 => {
                    ft_options.base.lcd_filter = CAIRO_LCD_FILTER_INTRA_PIXEL;
                }
                _ => {}
            }
        }
        if FcPatternGetInteger(
            pattern,
            b"hintstyle\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut hintstyle,
        ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
        {
            hintstyle = 3 as libc::c_int;
        }
        if hinting == 0 {
            hintstyle = 0 as libc::c_int;
        }
        match hintstyle {
            0 => {
                ft_options.base.hint_style = CAIRO_HINT_STYLE_NONE;
            }
            1 => {
                ft_options.base.hint_style = CAIRO_HINT_STYLE_SLIGHT;
            }
            3 => {
                ft_options.base.hint_style = CAIRO_HINT_STYLE_FULL;
            }
            2 | _ => {
                ft_options.base.hint_style = CAIRO_HINT_STYLE_MEDIUM;
            }
        }
        if ft_options.base.hint_style as libc::c_uint
            == CAIRO_HINT_STYLE_NONE as libc::c_int as libc::c_uint
        {
            bitmap = 0 as libc::c_int;
        }
        if bitmap == 0 {
            ft_options
                .load_flags = (ft_options.load_flags as libc::c_long
                | (1 as libc::c_long) << 3 as libc::c_int) as libc::c_uint;
        }
    } else {
        ft_options.base.antialias = CAIRO_ANTIALIAS_NONE;
    }
    if FcPatternGetBool(
        pattern,
        b"autohint\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut autohint,
    ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
    {
        autohint = 0 as libc::c_int;
    }
    if autohint != 0 {
        ft_options
            .load_flags = (ft_options.load_flags as libc::c_long
            | (1 as libc::c_long) << 5 as libc::c_int) as libc::c_uint;
    }
    if FcPatternGetBool(
        pattern,
        b"verticallayout\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut vertical_layout,
    ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
    {
        vertical_layout = 0 as libc::c_int;
    }
    if vertical_layout != 0 {
        ft_options
            .load_flags = (ft_options.load_flags as libc::c_long
            | (1 as libc::c_long) << 4 as libc::c_int) as libc::c_uint;
    }
    if FcPatternGetBool(
        pattern,
        b"embolden\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut embolden,
    ) as libc::c_uint != FcResultMatch as libc::c_int as libc::c_uint
    {
        embolden = 0 as libc::c_int;
    }
    if embolden != 0 {
        ft_options.synth_flags
            |= CAIRO_FT_SYNTHESIZE_BOLD as libc::c_int as libc::c_uint;
    }
    if FcPatternGetString(
        pattern,
        b"fontvariations\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        &mut variations as *mut *mut libc::c_char as *mut *mut FcChar8,
    ) as libc::c_uint == FcResultMatch as libc::c_int as libc::c_uint
    {
        ft_options.base.variations = strdup(variations);
    }
    *ret = ft_options;
}
unsafe extern "C" fn _cairo_ft_options_merge(
    mut options: *mut cairo_ft_options_t,
    mut other: *mut cairo_ft_options_t,
) {
    let mut load_flags: libc::c_int = (*other).load_flags as libc::c_int;
    let mut load_target: libc::c_int = (FT_RENDER_MODE_NORMAL as libc::c_int
        & 15 as libc::c_int) << 16 as libc::c_int;
    load_flags
        &= !(((((*other).load_flags >> 16 as libc::c_int
            & 15 as libc::c_int as libc::c_uint) as FT_Render_Mode as libc::c_uint
            & 15 as libc::c_int as libc::c_uint) as FT_Int32) << 16 as libc::c_int);
    if load_flags as libc::c_long & (1 as libc::c_long) << 1 as libc::c_int != 0 {
        (*other).base.hint_style = CAIRO_HINT_STYLE_NONE;
    }
    if (*other).base.antialias as libc::c_uint
        == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
        || (*options).base.antialias as libc::c_uint
            == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
    {
        (*options).base.antialias = CAIRO_ANTIALIAS_NONE;
        (*options).base.subpixel_order = CAIRO_SUBPIXEL_ORDER_DEFAULT;
    }
    if (*other).base.antialias as libc::c_uint
        == CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
        && (*options).base.antialias as libc::c_uint
            == CAIRO_ANTIALIAS_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).base.antialias = CAIRO_ANTIALIAS_SUBPIXEL;
        (*options).base.subpixel_order = (*other).base.subpixel_order;
    }
    if (*options).base.hint_style as libc::c_uint
        == CAIRO_HINT_STYLE_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).base.hint_style = (*other).base.hint_style;
    }
    if (*other).base.hint_style as libc::c_uint
        == CAIRO_HINT_STYLE_NONE as libc::c_int as libc::c_uint
    {
        (*options).base.hint_style = CAIRO_HINT_STYLE_NONE;
    }
    if (*options).base.lcd_filter as libc::c_uint
        == CAIRO_LCD_FILTER_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).base.lcd_filter = (*other).base.lcd_filter;
    }
    if (*other).base.lcd_filter as libc::c_uint
        == CAIRO_LCD_FILTER_NONE as libc::c_int as libc::c_uint
    {
        (*options).base.lcd_filter = CAIRO_LCD_FILTER_NONE;
    }
    if (*options).base.antialias as libc::c_uint
        == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
    {
        if (*options).base.hint_style as libc::c_uint
            == CAIRO_HINT_STYLE_NONE as libc::c_int as libc::c_uint
        {
            load_flags = (load_flags as libc::c_long
                | (1 as libc::c_long) << 1 as libc::c_int) as libc::c_int;
        } else {
            load_target = (FT_RENDER_MODE_MONO as libc::c_int & 15 as libc::c_int)
                << 16 as libc::c_int;
        }
        load_flags = (load_flags as libc::c_long
            | (1 as libc::c_long) << 12 as libc::c_int) as libc::c_int;
    } else {
        match (*options).base.hint_style as libc::c_uint {
            1 => {
                load_flags = (load_flags as libc::c_long
                    | (1 as libc::c_long) << 1 as libc::c_int) as libc::c_int;
            }
            2 => {
                load_target = (FT_RENDER_MODE_LIGHT as libc::c_int & 15 as libc::c_int)
                    << 16 as libc::c_int;
            }
            4 | 0 => {
                if (*options).base.antialias as libc::c_uint
                    == CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
                {
                    match (*options).base.subpixel_order as libc::c_uint {
                        0 | 1 | 2 => {
                            load_target = (FT_RENDER_MODE_LCD as libc::c_int
                                & 15 as libc::c_int) << 16 as libc::c_int;
                        }
                        3 | 4 => {
                            load_target = (FT_RENDER_MODE_LCD_V as libc::c_int
                                & 15 as libc::c_int) << 16 as libc::c_int;
                        }
                        _ => {}
                    }
                }
            }
            3 | _ => {}
        }
    }
    if !((*other).base.variations).is_null() {
        if !((*options).base.variations).is_null() {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            p = malloc(
                (strlen((*other).base.variations))
                    .wrapping_add(strlen((*options).base.variations))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            strcat(p, (*other).base.variations);
            strcat(p, b",\0" as *const u8 as *const libc::c_char);
            strcat(p, (*options).base.variations);
            free((*options).base.variations as *mut libc::c_void);
            let ref mut fresh21 = (*options).base.variations;
            *fresh21 = p;
        } else {
            let ref mut fresh22 = (*options).base.variations;
            *fresh22 = strdup((*other).base.variations);
        }
    }
    (*options).load_flags = (load_flags | load_target) as libc::c_uint;
    (*options).synth_flags = (*other).synth_flags;
}
unsafe extern "C" fn _cairo_ft_font_face_scaled_font_create(
    mut abstract_font_face: *mut libc::c_void,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
    mut font_out: *mut *mut cairo_scaled_font_t,
) -> cairo_status_t {
    let mut font_face: *mut cairo_ft_font_face_t = abstract_font_face
        as *mut cairo_ft_font_face_t;
    let mut scaled_font: *mut cairo_ft_scaled_font_t = 0 as *mut cairo_ft_scaled_font_t;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut metrics: *mut FT_Size_Metrics = 0 as *mut FT_Size_Metrics;
    let mut fs_metrics: cairo_font_extents_t = cairo_font_extents_t {
        ascent: 0.,
        descent: 0.,
        height: 0.,
        max_x_advance: 0.,
        max_y_advance: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = 0 as *mut cairo_ft_unscaled_font_t;
    if !((*font_face).unscaled).is_null() {} else {
        __assert_fail(
            b"font_face->unscaled\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            2049 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 164],
                &[libc::c_char; 164],
            >(
                b"cairo_status_t _cairo_ft_font_face_scaled_font_create(void *, const cairo_matrix_t *, const cairo_matrix_t *, const cairo_font_options_t *, cairo_scaled_font_t **)\0",
            ))
                .as_ptr(),
        );
    }
    face = _cairo_ft_unscaled_font_lock_face((*font_face).unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    scaled_font = (if ::std::mem::size_of::<cairo_ft_scaled_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_ft_scaled_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_ft_scaled_font_t;
    if scaled_font.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        unscaled = (*font_face).unscaled;
        let ref mut fresh23 = (*scaled_font).unscaled;
        *fresh23 = unscaled;
        _cairo_unscaled_font_reference(&mut (*unscaled).base);
        _cairo_font_options_init_copy(&mut (*scaled_font).ft_options.base, options);
        _cairo_ft_options_merge(
            &mut (*scaled_font).ft_options,
            &mut (*font_face).ft_options,
        );
        status = _cairo_scaled_font_init(
            &mut (*scaled_font).base,
            &mut (*font_face).base,
            font_matrix,
            ctm,
            options,
            &_cairo_ft_scaled_font_backend,
        );
        if !(status as u64 != 0) {
            status = _cairo_ft_unscaled_font_set_scale(
                unscaled,
                &mut (*scaled_font).base.scale,
            );
            if status as u64 != 0 {
                _cairo_ft_unscaled_font_unlock_face(unscaled);
                _cairo_unscaled_font_destroy(&mut (*unscaled).base);
                free(scaled_font as *mut libc::c_void);
                return status;
            }
            metrics = &mut (*(*face).size).metrics;
            if (*scaled_font).base.options.hint_metrics as libc::c_uint
                != CAIRO_HINT_METRICS_OFF as libc::c_int as libc::c_uint
                || (*face).units_per_EM as libc::c_int == 0 as libc::c_int
            {
                let mut x_factor: libc::c_double = 0.;
                let mut y_factor: libc::c_double = 0.;
                if (*unscaled).x_scale == 0 as libc::c_int as libc::c_double {
                    x_factor = 0 as libc::c_int as libc::c_double;
                } else {
                    x_factor = 1 as libc::c_int as libc::c_double / (*unscaled).x_scale;
                }
                if (*unscaled).y_scale == 0 as libc::c_int as libc::c_double {
                    y_factor = 0 as libc::c_int as libc::c_double;
                } else {
                    y_factor = 1 as libc::c_int as libc::c_double / (*unscaled).y_scale;
                }
                fs_metrics
                    .ascent = (*metrics).ascender as libc::c_double / 64.0f64 * y_factor;
                fs_metrics
                    .descent = -(*metrics).descender as libc::c_double / 64.0f64
                    * y_factor;
                fs_metrics
                    .height = (*metrics).height as libc::c_double / 64.0f64 * y_factor;
                if _cairo_ft_scaled_font_is_vertical(&mut (*scaled_font).base) == 0 {
                    fs_metrics
                        .max_x_advance = (*metrics).max_advance as libc::c_double
                        / 64.0f64 * x_factor;
                    fs_metrics.max_y_advance = 0 as libc::c_int as libc::c_double;
                } else {
                    fs_metrics.max_x_advance = 0 as libc::c_int as libc::c_double;
                    fs_metrics
                        .max_y_advance = (*metrics).max_advance as libc::c_double
                        / 64.0f64 * y_factor;
                }
            } else {
                let mut scale: libc::c_double = (*face).units_per_EM as libc::c_double;
                fs_metrics
                    .ascent = (*face).ascender as libc::c_int as libc::c_double / scale;
                fs_metrics
                    .descent = -((*face).descender as libc::c_int) as libc::c_double
                    / scale;
                fs_metrics
                    .height = (*face).height as libc::c_int as libc::c_double / scale;
                if _cairo_ft_scaled_font_is_vertical(&mut (*scaled_font).base) == 0 {
                    fs_metrics
                        .max_x_advance = (*face).max_advance_width as libc::c_int
                        as libc::c_double / scale;
                    fs_metrics.max_y_advance = 0 as libc::c_int as libc::c_double;
                } else {
                    fs_metrics.max_x_advance = 0 as libc::c_int as libc::c_double;
                    fs_metrics
                        .max_y_advance = (*face).max_advance_height as libc::c_int
                        as libc::c_double / scale;
                }
            }
            status = _cairo_scaled_font_set_metrics(
                &mut (*scaled_font).base,
                &mut fs_metrics,
            );
            if !(status as u64 != 0) {
                _cairo_ft_unscaled_font_unlock_face(unscaled);
                *font_out = &mut (*scaled_font).base;
                return CAIRO_STATUS_SUCCESS;
            }
        }
        _cairo_unscaled_font_destroy(&mut (*unscaled).base);
        free(scaled_font as *mut libc::c_void);
    }
    _cairo_ft_unscaled_font_unlock_face((*font_face).unscaled);
    *font_out = _cairo_scaled_font_create_in_error(status);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scaled_font_is_ft(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_bool_t {
    return ((*scaled_font).backend
        == &_cairo_ft_scaled_font_backend as *const cairo_scaled_font_backend_t)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_ft_scaled_font_fini(mut abstract_font: *mut libc::c_void) {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    if scaled_font.is_null() {
        return;
    }
    _cairo_unscaled_font_destroy(&mut (*(*scaled_font).unscaled).base);
}
unsafe extern "C" fn _move_to(
    mut to: *mut FT_Vector,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut path: *mut cairo_path_fixed_t = closure as *mut cairo_path_fixed_t;
    let mut x: cairo_fixed_t = 0;
    let mut y: cairo_fixed_t = 0;
    x = _cairo_fixed_from_26_6((*to).x as uint32_t);
    y = _cairo_fixed_from_26_6((*to).y as uint32_t);
    if _cairo_path_fixed_close_path(path) as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if _cairo_path_fixed_move_to(path, x, y) as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _line_to(
    mut to: *mut FT_Vector,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut path: *mut cairo_path_fixed_t = closure as *mut cairo_path_fixed_t;
    let mut x: cairo_fixed_t = 0;
    let mut y: cairo_fixed_t = 0;
    x = _cairo_fixed_from_26_6((*to).x as uint32_t);
    y = _cairo_fixed_from_26_6((*to).y as uint32_t);
    if _cairo_path_fixed_line_to(path, x, y) as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _conic_to(
    mut control: *mut FT_Vector,
    mut to: *mut FT_Vector,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut path: *mut cairo_path_fixed_t = closure as *mut cairo_path_fixed_t;
    let mut x0: cairo_fixed_t = 0;
    let mut y0: cairo_fixed_t = 0;
    let mut x1: cairo_fixed_t = 0;
    let mut y1: cairo_fixed_t = 0;
    let mut x2: cairo_fixed_t = 0;
    let mut y2: cairo_fixed_t = 0;
    let mut x3: cairo_fixed_t = 0;
    let mut y3: cairo_fixed_t = 0;
    let mut conic: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    if _cairo_path_fixed_get_current_point(path, &mut x0, &mut y0) == 0 {
        return 1 as libc::c_int;
    }
    conic.x = _cairo_fixed_from_26_6((*control).x as uint32_t);
    conic.y = _cairo_fixed_from_26_6((*control).y as uint32_t);
    x3 = _cairo_fixed_from_26_6((*to).x as uint32_t);
    y3 = _cairo_fixed_from_26_6((*to).y as uint32_t);
    x1 = (x0 as libc::c_double + 2.0f64 / 3.0f64 * (conic.x - x0) as libc::c_double)
        as cairo_fixed_t;
    y1 = (y0 as libc::c_double + 2.0f64 / 3.0f64 * (conic.y - y0) as libc::c_double)
        as cairo_fixed_t;
    x2 = (x3 as libc::c_double + 2.0f64 / 3.0f64 * (conic.x - x3) as libc::c_double)
        as cairo_fixed_t;
    y2 = (y3 as libc::c_double + 2.0f64 / 3.0f64 * (conic.y - y3) as libc::c_double)
        as cairo_fixed_t;
    if _cairo_path_fixed_curve_to(path, x1, y1, x2, y2, x3, y3) as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cubic_to(
    mut control1: *mut FT_Vector,
    mut control2: *mut FT_Vector,
    mut to: *mut FT_Vector,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut path: *mut cairo_path_fixed_t = closure as *mut cairo_path_fixed_t;
    let mut x0: cairo_fixed_t = 0;
    let mut y0: cairo_fixed_t = 0;
    let mut x1: cairo_fixed_t = 0;
    let mut y1: cairo_fixed_t = 0;
    let mut x2: cairo_fixed_t = 0;
    let mut y2: cairo_fixed_t = 0;
    x0 = _cairo_fixed_from_26_6((*control1).x as uint32_t);
    y0 = _cairo_fixed_from_26_6((*control1).y as uint32_t);
    x1 = _cairo_fixed_from_26_6((*control2).x as uint32_t);
    y1 = _cairo_fixed_from_26_6((*control2).y as uint32_t);
    x2 = _cairo_fixed_from_26_6((*to).x as uint32_t);
    y2 = _cairo_fixed_from_26_6((*to).y as uint32_t);
    if _cairo_path_fixed_curve_to(path, x0, y0, x1, y1, x2, y2) as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _decompose_glyph_outline(
    mut face: FT_Face,
    mut options: *mut cairo_font_options_t,
    mut pathp: *mut *mut cairo_path_fixed_t,
) -> cairo_status_t {
    static mut outline_funcs: FT_Outline_Funcs = unsafe {
        {
            let mut init = FT_Outline_Funcs_ {
                move_to: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FT_Vector,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    FT_Outline_MoveToFunc,
                >(
                    Some(
                        _move_to
                            as unsafe extern "C" fn(
                                *mut FT_Vector,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                ),
                line_to: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FT_Vector,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    FT_Outline_LineToFunc,
                >(
                    Some(
                        _line_to
                            as unsafe extern "C" fn(
                                *mut FT_Vector,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                ),
                conic_to: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FT_Vector,
                            *mut FT_Vector,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    FT_Outline_ConicToFunc,
                >(
                    Some(
                        _conic_to
                            as unsafe extern "C" fn(
                                *mut FT_Vector,
                                *mut FT_Vector,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                ),
                cubic_to: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FT_Vector,
                            *mut FT_Vector,
                            *mut FT_Vector,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    FT_Outline_CubicToFunc,
                >(
                    Some(
                        _cubic_to
                            as unsafe extern "C" fn(
                                *mut FT_Vector,
                                *mut FT_Vector,
                                *mut FT_Vector,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                ),
                shift: 0 as libc::c_int,
                delta: 0 as libc::c_int as FT_Pos,
            };
            init
        }
    };
    static mut invert_y: FT_Matrix = {
        let mut init = FT_Matrix_ {
            xx: (1.0f64 * 65536.0f64) as FT_Fixed,
            xy: 0 as libc::c_int as FT_Fixed,
            yx: 0 as libc::c_int as FT_Fixed,
            yy: (-1.0f64 * 65536.0f64) as FT_Fixed,
        };
        init
    };
    let mut glyph: FT_GlyphSlot = 0 as *mut FT_GlyphSlotRec_;
    let mut path: *mut cairo_path_fixed_t = 0 as *mut cairo_path_fixed_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    path = _cairo_path_fixed_create();
    if path.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    glyph = (*face).glyph;
    FT_Outline_Transform(&mut (*glyph).outline, &invert_y);
    if FT_Outline_Decompose(
        &mut (*glyph).outline,
        &outline_funcs,
        path as *mut libc::c_void,
    ) != 0
    {
        _cairo_path_fixed_destroy(path);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    status = _cairo_path_fixed_close_path(path);
    if status as u64 != 0 {
        _cairo_path_fixed_destroy(path);
        return status;
    }
    *pathp = path;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ft_scaled_glyph_vertical_layout_bearing_fix(
    mut abstract_font: *mut libc::c_void,
    mut glyph: FT_GlyphSlot,
) {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut vector: FT_Vector = FT_Vector { x: 0, y: 0 };
    vector.x = (*glyph).metrics.vertBearingX - (*glyph).metrics.horiBearingX;
    vector.y = -(*glyph).metrics.vertBearingY - (*glyph).metrics.horiBearingY;
    if (*glyph).format as libc::c_uint
        == FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint
    {
        FT_Vector_Transform(&mut vector, &mut (*(*scaled_font).unscaled).Current_Shape);
        FT_Outline_Translate(&mut (*glyph).outline, vector.x, vector.y);
    } else if (*glyph).format as libc::c_uint
        == FT_GLYPH_FORMAT_BITMAP as libc::c_int as libc::c_uint
    {
        let ref mut fresh24 = (*glyph).bitmap_left;
        *fresh24 = (*fresh24 as libc::c_long
            + vector.x / 64 as libc::c_int as libc::c_long) as FT_Int;
        let ref mut fresh25 = (*glyph).bitmap_top;
        *fresh25 = (*fresh25 as libc::c_long
            + vector.y / 64 as libc::c_int as libc::c_long) as FT_Int;
    }
}
unsafe extern "C" fn cairo_ft_apply_variations(
    mut face: FT_Face,
    mut scaled_font: *mut cairo_ft_scaled_font_t,
) {
    let mut current_block: u64;
    let mut ft_mm_var: *mut FT_MM_Var = 0 as *mut FT_MM_Var;
    let mut ret: FT_Error = 0;
    let mut instance_id: libc::c_uint = ((*(*scaled_font).unscaled).id
        >> 16 as libc::c_int) as libc::c_uint;
    ret = FT_Get_MM_Var(face, &mut ft_mm_var);
    if ret == 0 as libc::c_int {
        let mut current_coords: *mut FT_Fixed = 0 as *mut FT_Fixed;
        let mut coords: *mut FT_Fixed = 0 as *mut FT_Fixed;
        let mut i: libc::c_uint = 0;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        coords = malloc(
            (::std::mem::size_of::<FT_Fixed>() as libc::c_ulong)
                .wrapping_mul((*ft_mm_var).num_axis as libc::c_ulong),
        ) as *mut FT_Fixed;
        if !((*(*scaled_font).unscaled).variations).is_null() {
            memcpy(
                coords as *mut libc::c_void,
                (*(*scaled_font).unscaled).variations as *const libc::c_void,
                ((*ft_mm_var).num_axis as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<FT_Fixed>() as libc::c_ulong),
            );
        } else if instance_id != 0 && instance_id <= (*ft_mm_var).num_namedstyles {
            let mut instance: *mut FT_Var_Named_Style = &mut *((*ft_mm_var).namedstyle)
                .offset(
                    instance_id.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as *mut FT_Var_Named_Style;
            memcpy(
                coords as *mut libc::c_void,
                (*instance).coords as *const libc::c_void,
                ((*ft_mm_var).num_axis as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<FT_Fixed>() as libc::c_ulong),
            );
        } else {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*ft_mm_var).num_axis {
                *coords
                    .offset(i as isize) = (*((*ft_mm_var).axis).offset(i as isize)).def;
                i = i.wrapping_add(1);
            }
        }
        p = (*scaled_font).ft_options.base.variations;
        while !p.is_null() && *p as libc::c_int != 0 {
            let mut start: *const libc::c_char = 0 as *const libc::c_char;
            let mut end: *const libc::c_char = 0 as *const libc::c_char;
            let mut end2: *const libc::c_char = 0 as *const libc::c_char;
            let mut tag: FT_ULong = 0;
            let mut value: libc::c_double = 0.;
            while _cairo_isspace(*p as libc::c_int) != 0 {
                p = p.offset(1);
            }
            start = p;
            end = strchr(p, ',' as i32);
            if !(!end.is_null()
                && (end.offset_from(p) as libc::c_long)
                    < 6 as libc::c_int as libc::c_long)
            {
                tag = ((*p.offset(0 as libc::c_int as isize) as libc::c_uchar as FT_Tag)
                    << 24 as libc::c_int
                    | (*p.offset(1 as libc::c_int as isize) as libc::c_uchar as FT_Tag)
                        << 16 as libc::c_int
                    | (*p.offset(2 as libc::c_int as isize) as libc::c_uchar as FT_Tag)
                        << 8 as libc::c_int
                    | *p.offset(3 as libc::c_int as isize) as libc::c_uchar as FT_Tag)
                    as FT_ULong;
                p = p.offset(4 as libc::c_int as isize);
                while _cairo_isspace(*p as libc::c_int) != 0 {
                    p = p.offset(1);
                }
                if *p as libc::c_int == '=' as i32 {
                    p = p.offset(1);
                }
                if !((p.offset_from(start) as libc::c_long)
                    < 5 as libc::c_int as libc::c_long)
                {
                    value = _cairo_strtod(
                        p,
                        &mut end2 as *mut *const libc::c_char as *mut *mut libc::c_char,
                    );
                    while !end2.is_null() && _cairo_isspace(*end2 as libc::c_int) != 0 {
                        end2 = end2.offset(1);
                    }
                    if !(!end2.is_null()
                        && (*end2 as libc::c_int != ',' as i32
                            && *end2 as libc::c_int != '\0' as i32))
                    {
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < (*ft_mm_var).num_axis {
                            if (*((*ft_mm_var).axis).offset(i as isize)).tag == tag {
                                *coords
                                    .offset(
                                        i as isize,
                                    ) = (value * 65536 as libc::c_int as libc::c_double)
                                    as FT_Fixed;
                                break;
                            } else {
                                i = i.wrapping_add(1);
                            }
                        }
                    }
                }
            }
            p = if !end.is_null() {
                end.offset(1 as libc::c_int as isize)
            } else {
                0 as *const libc::c_char
            };
        }
        current_coords = malloc(
            (::std::mem::size_of::<FT_Fixed>() as libc::c_ulong)
                .wrapping_mul((*ft_mm_var).num_axis as libc::c_ulong),
        ) as *mut FT_Fixed;
        ret = FT_Get_Var_Design_Coordinates(face, (*ft_mm_var).num_axis, current_coords);
        if ret == 0 as libc::c_int {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*ft_mm_var).num_axis {
                if *coords.offset(i as isize) != *current_coords.offset(i as isize) {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i == (*ft_mm_var).num_axis {
                current_block = 1907845719370243911;
            } else {
                current_block = 7990025728955927862;
            }
        } else {
            current_block = 7990025728955927862;
        }
        match current_block {
            7990025728955927862 => {
                FT_Set_Var_Design_Coordinates(face, (*ft_mm_var).num_axis, coords);
            }
            _ => {}
        }
        free(coords as *mut libc::c_void);
        free(current_coords as *mut libc::c_void);
        FT_Done_MM_Var((*(*face).glyph).library, ft_mm_var);
    }
}
unsafe extern "C" fn _cairo_ft_scaled_glyph_load_glyph(
    mut scaled_font: *mut cairo_ft_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut face: FT_Face,
    mut load_flags: libc::c_int,
    mut use_em_size: cairo_bool_t,
    mut vertical_layout: cairo_bool_t,
) -> cairo_int_status_t {
    let mut error: FT_Error = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if use_em_size != 0 {
        let mut em_size: cairo_matrix_t = cairo_matrix_t {
            xx: 0.,
            yx: 0.,
            xy: 0.,
            yy: 0.,
            x0: 0.,
            y0: 0.,
        };
        cairo_matrix_init_scale(
            &mut em_size,
            (*face).units_per_EM as libc::c_double,
            (*face).units_per_EM as libc::c_double,
        );
        status = _cairo_ft_unscaled_font_set_scale(
            (*scaled_font).unscaled,
            &mut em_size,
        );
    } else {
        status = _cairo_ft_unscaled_font_set_scale(
            (*scaled_font).unscaled,
            &mut (*scaled_font).base.scale,
        );
    }
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    cairo_ft_apply_variations(face, scaled_font);
    error = FT_Load_Glyph(
        face,
        ((*scaled_glyph).hash_entry.hash & 0xffffff as libc::c_int as libc::c_ulong)
            as FT_UInt,
        load_flags,
    );
    if error == FT_Err_Out_Of_Memory as libc::c_int {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if (*scaled_font).ft_options.synth_flags
        & CAIRO_FT_SYNTHESIZE_BOLD as libc::c_int as libc::c_uint != 0
    {
        FT_GlyphSlot_Embolden((*face).glyph);
    }
    if (*scaled_font).ft_options.synth_flags
        & CAIRO_FT_SYNTHESIZE_OBLIQUE as libc::c_int as libc::c_uint != 0
    {
        FT_GlyphSlot_Oblique((*face).glyph);
    }
    if vertical_layout != 0 {
        _cairo_ft_scaled_glyph_vertical_layout_bearing_fix(
            scaled_font as *mut libc::c_void,
            (*face).glyph,
        );
    }
    if (*(*face).glyph).format as libc::c_uint
        == FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint
    {
        let mut xshift: FT_Pos = 0;
        let mut yshift: FT_Pos = 0;
        xshift = ((((*scaled_glyph).hash_entry.hash >> 24 as libc::c_int
            & 3 as libc::c_int as libc::c_ulong) as libc::c_int) << 4 as libc::c_int)
            as FT_Pos;
        yshift = ((((*scaled_glyph).hash_entry.hash >> 26 as libc::c_int
            & 3 as libc::c_int as libc::c_ulong) as libc::c_int) << 4 as libc::c_int)
            as FT_Pos;
        FT_Outline_Translate(&mut (*(*face).glyph).outline, xshift, -yshift);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ft_scaled_glyph_init_surface(
    mut scaled_font: *mut cairo_ft_scaled_font_t,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut info: cairo_scaled_glyph_info_t,
    mut face: FT_Face,
    mut foreground_color: *const cairo_color_t,
    mut vertical_layout: cairo_bool_t,
    mut load_flags: libc::c_int,
) -> cairo_int_status_t {
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut glyph: FT_GlyphSlot = 0 as *mut FT_GlyphSlotRec_;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut uses_foreground_color: cairo_bool_t = 0 as libc::c_int;
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
        || info as libc::c_uint
            == CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"info == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE || info == CAIRO_SCALED_GLYPH_INFO_SURFACE\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            2505 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 183],
                &[libc::c_char; 183],
            >(
                b"cairo_int_status_t _cairo_ft_scaled_glyph_init_surface(cairo_ft_scaled_font_t *, cairo_scaled_glyph_t *, cairo_scaled_glyph_info_t, FT_Face, const cairo_color_t *, cairo_bool_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
    {
        if (*unscaled).have_color() == 0 {
            (*scaled_glyph).set_color_glyph(0 as libc::c_int as libc::c_uint);
            (*scaled_glyph).set_color_glyph_set(1 as libc::c_int as libc::c_uint);
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        let mut iterator: FT_LayerIterator = FT_LayerIterator {
            num_layers: 0,
            layer: 0,
            p: 0 as *mut FT_Byte,
        };
        let mut layer_glyph_index: FT_UInt = 0;
        let mut layer_color_index: FT_UInt = 0;
        let mut color: FT_Color = FT_Color {
            blue: 0,
            green: 0,
            red: 0,
            alpha: 0,
        };
        let mut palette_data: FT_Palette_Data = FT_Palette_Data {
            num_palettes: 0,
            palette_name_ids: 0 as *const FT_UShort,
            palette_flags: 0 as *const FT_UShort,
            num_palette_entries: 0,
            palette_entry_name_ids: 0 as *const FT_UShort,
        };
        iterator.p = 0 as *mut FT_Byte;
        while FT_Get_Color_Glyph_Layer(
            face,
            ((*scaled_glyph).hash_entry.hash & 0xffffff as libc::c_int as libc::c_ulong)
                as FT_UInt,
            &mut layer_glyph_index,
            &mut layer_color_index,
            &mut iterator,
        ) != 0
        {
            if !(layer_color_index == 0xffff as libc::c_int as libc::c_uint) {
                continue;
            }
            uses_foreground_color = 1 as libc::c_int;
            break;
        }
        if uses_foreground_color != 0 {
            color
                .red = ((*foreground_color).red * 0xff as libc::c_int as libc::c_double)
                as FT_Byte;
            color
                .green = ((*foreground_color).green
                * 0xff as libc::c_int as libc::c_double) as FT_Byte;
            color
                .blue = ((*foreground_color).blue
                * 0xff as libc::c_int as libc::c_double) as FT_Byte;
            color
                .alpha = ((*foreground_color).alpha
                * 0xff as libc::c_int as libc::c_double) as FT_Byte;
            FT_Palette_Set_Foreground_Color(face, color);
        }
        if FT_Palette_Data_Get(face, &mut palette_data) == 0 as libc::c_int
            && palette_data.num_palettes as libc::c_int > 0 as libc::c_int
        {
            let mut palette_index: FT_UShort = 0 as libc::c_int as FT_UShort;
            if (*scaled_font).base.options.palette_index
                < palette_data.num_palettes as libc::c_uint
            {
                palette_index = (*scaled_font).base.options.palette_index as FT_UShort;
            }
            FT_Palette_Select(face, palette_index, 0 as *mut *mut FT_Color);
        }
        load_flags = (load_flags as libc::c_long
            & !((1 as libc::c_long) << 12 as libc::c_int)) as libc::c_int;
        load_flags
            &= !((((load_flags >> 16 as libc::c_int & 15 as libc::c_int)
                as FT_Render_Mode as libc::c_uint & 15 as libc::c_int as libc::c_uint)
                as FT_Int32) << 16 as libc::c_int);
        load_flags
            |= (FT_RENDER_MODE_NORMAL as libc::c_int & 15 as libc::c_int)
                << 16 as libc::c_int;
        load_flags = (load_flags as libc::c_long
            | (1 as libc::c_long) << 20 as libc::c_int) as libc::c_int;
    } else {
        load_flags = (load_flags as libc::c_long
            & !((1 as libc::c_long) << 20 as libc::c_int)) as libc::c_int;
    }
    status = _cairo_ft_scaled_glyph_load_glyph(
        scaled_font,
        scaled_glyph,
        face,
        load_flags,
        0 as libc::c_int,
        vertical_layout,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    glyph = (*face).glyph;
    if (*glyph).format as libc::c_uint
        == FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint
    {
        status = _render_glyph_outline(
            face,
            &mut (*scaled_font).ft_options.base,
            &mut surface,
        );
    } else {
        status = _render_glyph_bitmap(
            face,
            &mut (*scaled_font).ft_options.base,
            &mut surface,
        );
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (*unscaled).have_shape != 0
        {
            status = _transform_glyph_bitmap(
                &mut (*unscaled).current_shape,
                &mut surface,
            );
            if status as u64 != 0 {
                cairo_surface_destroy(&mut (*surface).base);
            }
        }
    }
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if info as libc::c_uint
        == CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
    {
        if pixman_image_get_format((*surface).pixman_image) as libc::c_uint
            == PIXMAN_a8r8g8b8 as libc::c_int as libc::c_uint
            && pixman_image_get_component_alpha((*surface).pixman_image) == 0
        {
            _cairo_scaled_glyph_set_color_surface(
                scaled_glyph,
                &mut (*scaled_font).base,
                surface,
                uses_foreground_color,
            );
            (*scaled_glyph).set_color_glyph(1 as libc::c_int as libc::c_uint);
        } else {
            _cairo_scaled_glyph_set_surface(
                scaled_glyph,
                &mut (*scaled_font).base,
                surface,
            );
            (*scaled_glyph).set_color_glyph(0 as libc::c_int as libc::c_uint);
            status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        (*scaled_glyph).set_color_glyph_set(1 as libc::c_int as libc::c_uint);
    } else {
        _cairo_scaled_glyph_set_surface(scaled_glyph, &mut (*scaled_font).base, surface);
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ft_scaled_glyph_init(
    mut abstract_font: *mut libc::c_void,
    mut scaled_glyph: *mut cairo_scaled_glyph_t,
    mut info: cairo_scaled_glyph_info_t,
    mut foreground_color: *const cairo_color_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut fs_metrics: cairo_text_extents_t = cairo_text_extents_t {
        x_bearing: 0.,
        y_bearing: 0.,
        width: 0.,
        height: 0.,
        x_advance: 0.,
        y_advance: 0.,
    };
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut glyph: FT_GlyphSlot = 0 as *mut FT_GlyphSlotRec_;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut load_flags: libc::c_int = (*scaled_font).ft_options.load_flags
        as libc::c_int;
    let mut metrics: *mut FT_Glyph_Metrics = 0 as *mut FT_Glyph_Metrics;
    let mut x_factor: libc::c_double = 0.;
    let mut y_factor: libc::c_double = 0.;
    let mut vertical_layout: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut scaled_glyph_loaded: cairo_bool_t = 0 as libc::c_int;
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    load_flags = (load_flags as libc::c_long | (1 as libc::c_long) << 9 as libc::c_int)
        as libc::c_int;
    if info as libc::c_uint & CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && info as libc::c_uint
            & (CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int
                | CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int) as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        load_flags = (load_flags as libc::c_long
            | (1 as libc::c_long) << 3 as libc::c_int) as libc::c_int;
    }
    if load_flags as libc::c_long & (1 as libc::c_long) << 4 as libc::c_int != 0 {
        load_flags = (load_flags as libc::c_long
            & !((1 as libc::c_long) << 4 as libc::c_int)) as libc::c_int;
        vertical_layout = 1 as libc::c_int;
    }
    if info as libc::c_uint
        & CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int as libc::c_uint != 0
    {
        let mut hint_metrics: cairo_bool_t = ((*scaled_font).base.options.hint_metrics
            as libc::c_uint != CAIRO_HINT_METRICS_OFF as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut color_flag: libc::c_int = 0 as libc::c_int;
        if (*unscaled).have_color() as libc::c_int != 0
            && (*scaled_font).base.options.color_mode as libc::c_uint
                != CAIRO_COLOR_MODE_NO_COLOR as libc::c_int as libc::c_uint
        {
            color_flag = ((1 as libc::c_long) << 20 as libc::c_int) as libc::c_int;
        }
        status = _cairo_ft_scaled_glyph_load_glyph(
            scaled_font,
            scaled_glyph,
            face,
            load_flags | color_flag,
            (hint_metrics == 0) as libc::c_int,
            vertical_layout,
        ) as cairo_status_t;
        if status as u64 != 0 {
            current_block = 32816393318278798;
        } else {
            glyph = (*face).glyph;
            scaled_glyph_loaded = hint_metrics;
            metrics = &mut (*glyph).metrics;
            if (*unscaled).x_scale == 0 as libc::c_int as libc::c_double {
                x_factor = 0 as libc::c_int as libc::c_double;
            } else {
                x_factor = 1 as libc::c_int as libc::c_double / (*unscaled).x_scale;
            }
            if (*unscaled).y_scale == 0 as libc::c_int as libc::c_double {
                y_factor = 0 as libc::c_int as libc::c_double;
            } else {
                y_factor = 1 as libc::c_int as libc::c_double / (*unscaled).y_scale;
            }
            if hint_metrics != 0
                && load_flags as libc::c_long & (1 as libc::c_long) << 1 as libc::c_int
                    != 0
            {
                let mut x1: FT_Pos = 0;
                let mut x2: FT_Pos = 0;
                let mut y1: FT_Pos = 0;
                let mut y2: FT_Pos = 0;
                let mut advance: FT_Pos = 0;
                if vertical_layout == 0 {
                    x1 = (*metrics).horiBearingX & -(64 as libc::c_int) as libc::c_long;
                    x2 = (*metrics).horiBearingX + (*metrics).width
                        + 63 as libc::c_int as libc::c_long
                        & -(64 as libc::c_int) as libc::c_long;
                    y1 = -(*metrics).horiBearingY & -(64 as libc::c_int) as libc::c_long;
                    y2 = -(*metrics).horiBearingY + (*metrics).height
                        + 63 as libc::c_int as libc::c_long
                        & -(64 as libc::c_int) as libc::c_long;
                    advance = (*metrics).horiAdvance + 32 as libc::c_int as libc::c_long
                        & -(64 as libc::c_int) as libc::c_long;
                    fs_metrics.x_bearing = x1 as libc::c_double / 64.0f64 * x_factor;
                    fs_metrics.y_bearing = y1 as libc::c_double / 64.0f64 * y_factor;
                    fs_metrics.width = (x2 - x1) as libc::c_double / 64.0f64 * x_factor;
                    fs_metrics.height = (y2 - y1) as libc::c_double / 64.0f64 * y_factor;
                    fs_metrics
                        .x_advance = advance as libc::c_double / 64.0f64 * x_factor;
                    fs_metrics.y_advance = 0 as libc::c_int as libc::c_double;
                } else {
                    x1 = (*metrics).vertBearingX & -(64 as libc::c_int) as libc::c_long;
                    x2 = (*metrics).vertBearingX + (*metrics).width
                        + 63 as libc::c_int as libc::c_long
                        & -(64 as libc::c_int) as libc::c_long;
                    y1 = (*metrics).vertBearingY & -(64 as libc::c_int) as libc::c_long;
                    y2 = (*metrics).vertBearingY + (*metrics).height
                        + 63 as libc::c_int as libc::c_long
                        & -(64 as libc::c_int) as libc::c_long;
                    advance = (*metrics).vertAdvance + 32 as libc::c_int as libc::c_long
                        & -(64 as libc::c_int) as libc::c_long;
                    fs_metrics.x_bearing = x1 as libc::c_double / 64.0f64 * x_factor;
                    fs_metrics.y_bearing = y1 as libc::c_double / 64.0f64 * y_factor;
                    fs_metrics.width = (x2 - x1) as libc::c_double / 64.0f64 * x_factor;
                    fs_metrics.height = (y2 - y1) as libc::c_double / 64.0f64 * y_factor;
                    fs_metrics.x_advance = 0 as libc::c_int as libc::c_double;
                    fs_metrics
                        .y_advance = advance as libc::c_double / 64.0f64 * y_factor;
                }
            } else {
                fs_metrics
                    .width = (*metrics).width as libc::c_double / 64.0f64 * x_factor;
                fs_metrics
                    .height = (*metrics).height as libc::c_double / 64.0f64 * y_factor;
                if vertical_layout == 0 {
                    fs_metrics
                        .x_bearing = (*metrics).horiBearingX as libc::c_double / 64.0f64
                        * x_factor;
                    fs_metrics
                        .y_bearing = -(*metrics).horiBearingY as libc::c_double / 64.0f64
                        * y_factor;
                    if hint_metrics != 0
                        || (*glyph).format as libc::c_uint
                            != FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint
                    {
                        fs_metrics
                            .x_advance = (*metrics).horiAdvance as libc::c_double
                            / 64.0f64 * x_factor;
                    } else {
                        fs_metrics
                            .x_advance = (*glyph).linearHoriAdvance as libc::c_double
                            / 65536.0f64 * x_factor;
                    }
                    fs_metrics.y_advance = 0 as libc::c_int as libc::c_double * y_factor;
                } else {
                    fs_metrics
                        .x_bearing = (*metrics).vertBearingX as libc::c_double / 64.0f64
                        * x_factor;
                    fs_metrics
                        .y_bearing = (*metrics).vertBearingY as libc::c_double / 64.0f64
                        * y_factor;
                    fs_metrics.x_advance = 0 as libc::c_int as libc::c_double * x_factor;
                    if hint_metrics != 0
                        || (*glyph).format as libc::c_uint
                            != FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint
                    {
                        fs_metrics
                            .y_advance = (*metrics).vertAdvance as libc::c_double
                            / 64.0f64 * y_factor;
                    } else {
                        fs_metrics
                            .y_advance = (*glyph).linearVertAdvance as libc::c_double
                            / 65536.0f64 * y_factor;
                    }
                }
            }
            _cairo_scaled_glyph_set_metrics(
                scaled_glyph,
                &mut (*scaled_font).base,
                &mut fs_metrics,
            );
            current_block = 13853033528615664019;
        }
    } else {
        current_block = 13853033528615664019;
    }
    match current_block {
        13853033528615664019 => {
            if info as libc::c_uint
                & CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int as libc::c_uint
                != 0
            {
                status = _cairo_ft_scaled_glyph_init_surface(
                    scaled_font,
                    scaled_glyph,
                    CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE,
                    face,
                    foreground_color,
                    vertical_layout,
                    load_flags,
                ) as cairo_status_t;
                if status as u64 != 0 {
                    current_block = 32816393318278798;
                } else {
                    current_block = 5873035170358615968;
                }
            } else {
                current_block = 5873035170358615968;
            }
            match current_block {
                32816393318278798 => {}
                _ => {
                    if info as libc::c_uint
                        & CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int as libc::c_uint
                        != 0
                    {
                        status = _cairo_ft_scaled_glyph_init_surface(
                            scaled_font,
                            scaled_glyph,
                            CAIRO_SCALED_GLYPH_INFO_SURFACE,
                            face,
                            0 as *const cairo_color_t,
                            vertical_layout,
                            load_flags,
                        ) as cairo_status_t;
                        if status as u64 != 0 {
                            current_block = 32816393318278798;
                        } else {
                            current_block = 479107131381816815;
                        }
                    } else {
                        current_block = 479107131381816815;
                    }
                    match current_block {
                        32816393318278798 => {}
                        _ => {
                            if info as libc::c_uint
                                & CAIRO_SCALED_GLYPH_INFO_PATH as libc::c_int
                                    as libc::c_uint != 0
                            {
                                let mut path: *mut cairo_path_fixed_t = 0
                                    as *mut cairo_path_fixed_t;
                                if info as libc::c_uint
                                    & (CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int
                                        | CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE as libc::c_int)
                                        as libc::c_uint != 0 as libc::c_int as libc::c_uint
                                {
                                    scaled_glyph_loaded = 0 as libc::c_int;
                                    load_flags = (load_flags as libc::c_long
                                        | (1 as libc::c_long) << 3 as libc::c_int) as libc::c_int;
                                }
                                if scaled_glyph_loaded == 0 {
                                    status = _cairo_ft_scaled_glyph_load_glyph(
                                        scaled_font,
                                        scaled_glyph,
                                        face,
                                        load_flags,
                                        0 as libc::c_int,
                                        vertical_layout,
                                    ) as cairo_status_t;
                                    if status as u64 != 0 {
                                        current_block = 32816393318278798;
                                    } else {
                                        glyph = (*face).glyph;
                                        current_block = 5005389895767293342;
                                    }
                                } else {
                                    current_block = 5005389895767293342;
                                }
                                match current_block {
                                    32816393318278798 => {}
                                    _ => {
                                        if (*glyph).format as libc::c_uint
                                            == FT_GLYPH_FORMAT_OUTLINE as libc::c_int as libc::c_uint
                                        {
                                            status = _decompose_glyph_outline(
                                                face,
                                                &mut (*scaled_font).ft_options.base,
                                                &mut path,
                                            );
                                        } else {
                                            status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                                as cairo_status_t;
                                        }
                                        if !(status as u64 != 0) {
                                            _cairo_scaled_glyph_set_path(
                                                scaled_glyph,
                                                &mut (*scaled_font).base,
                                                path,
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
        _ => {}
    }
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ft_ucs4_to_index(
    mut abstract_font: *mut libc::c_void,
    mut ucs4: uint32_t,
) -> libc::c_ulong {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut index: FT_UInt = 0;
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return 0 as libc::c_int as libc::c_ulong;
    }
    index = FcFreeTypeCharIndex(face, ucs4);
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    return index as libc::c_ulong;
}
unsafe extern "C" fn _cairo_ft_load_truetype_table(
    mut abstract_font: *mut libc::c_void,
    mut tag: libc::c_ulong,
    mut offset: libc::c_long,
    mut buffer: *mut libc::c_uchar,
    mut length: *mut libc::c_ulong,
) -> cairo_int_status_t {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut status: cairo_status_t = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
        as cairo_status_t;
    if !length.is_null() {} else {
        __assert_fail(
            b"length != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            2890 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"cairo_int_status_t _cairo_ft_load_truetype_table(void *, unsigned long, long, unsigned char *, unsigned long *)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_ft_scaled_font_is_vertical(&mut (*scaled_font).base) != 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if (*face).face_flags & (1 as libc::c_long) << 3 as libc::c_int != 0 {
        if buffer.is_null() {
            *length = 0 as libc::c_int as libc::c_ulong;
        }
        if FT_Load_Sfnt_Table(face, tag, offset, buffer, length) == 0 as libc::c_int {
            status = CAIRO_STATUS_SUCCESS;
        }
    }
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ft_index_to_ucs4(
    mut abstract_font: *mut libc::c_void,
    mut index: libc::c_ulong,
    mut ucs4: *mut uint32_t,
) -> cairo_int_status_t {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut charcode: FT_ULong = 0;
    let mut gindex: FT_UInt = 0;
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    *ucs4 = -(1 as libc::c_int) as uint32_t;
    charcode = FT_Get_First_Char(face, &mut gindex);
    while gindex != 0 as libc::c_int as libc::c_uint {
        if gindex as libc::c_ulong == index {
            *ucs4 = charcode as uint32_t;
            break;
        } else {
            charcode = FT_Get_Next_Char(face, charcode, &mut gindex);
        }
    }
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ft_is_synthetic(
    mut abstract_font: *mut libc::c_void,
    mut is_synthetic: *mut cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut error: FT_Error = 0;
    if (*scaled_font).ft_options.synth_flags != 0 as libc::c_int as libc::c_uint {
        *is_synthetic = 1 as libc::c_int;
        return status;
    }
    *is_synthetic = 0 as libc::c_int;
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if (*face).face_flags & (1 as libc::c_long) << 8 as libc::c_int != 0 {
        let mut mm_var: *mut FT_MM_Var = 0 as *mut FT_MM_Var;
        let mut coords: *mut FT_Fixed = 0 as *mut FT_Fixed;
        let mut num_axis: libc::c_int = 0;
        *is_synthetic = 1 as libc::c_int;
        error = FT_Get_MM_Var(face, &mut mm_var);
        if error != 0 {
            status = _cairo_error(_ft_to_cairo_error(error)) as cairo_int_status_t;
        } else {
            num_axis = (*mm_var).num_axis as libc::c_int;
            coords = _cairo_malloc_ab(
                num_axis as size_t,
                ::std::mem::size_of::<FT_Fixed>() as libc::c_ulong,
            ) as *mut FT_Fixed;
            if coords.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            } else {
                let mut i: libc::c_int = 0;
                FT_Get_Var_Blend_Coordinates(face, num_axis as FT_UInt, coords);
                *is_synthetic = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < num_axis {
                    if *coords.offset(i as isize) != 0 {
                        *is_synthetic = 1 as libc::c_int;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        free(coords as *mut libc::c_void);
        FT_Done_MM_Var((*(*face).glyph).library, mm_var);
    }
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    return status;
}
unsafe extern "C" fn _cairo_index_to_glyph_name(
    mut abstract_font: *mut libc::c_void,
    mut glyph_names: *mut *mut libc::c_char,
    mut num_glyph_names: libc::c_int,
    mut glyph_index: libc::c_ulong,
    mut glyph_array_index: *mut libc::c_ulong,
) -> cairo_int_status_t {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut error: FT_Error = 0;
    let mut i: libc::c_int = 0;
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    error = FT_Get_Glyph_Name(
        face,
        glyph_index as FT_UInt,
        buffer.as_mut_ptr() as FT_Pointer,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as FT_UInt,
    );
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    if error != FT_Err_Ok as libc::c_int {
        if error == FT_Err_Out_Of_Memory as libc::c_int {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (glyph_index as libc::c_long) < num_glyph_names as libc::c_long
        && strcmp(*glyph_names.offset(glyph_index as isize), buffer.as_mut_ptr())
            == 0 as libc::c_int
    {
        *glyph_array_index = glyph_index;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    i = 0 as libc::c_int;
    while i < num_glyph_names {
        if strcmp(*glyph_names.offset(i as isize), buffer.as_mut_ptr())
            == 0 as libc::c_int
        {
            *glyph_array_index = i as libc::c_ulong;
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        i += 1;
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
unsafe extern "C" fn _ft_is_type1(mut face: FT_Face) -> cairo_bool_t {
    let mut font_format: *const libc::c_char = FT_Get_X11_Font_Format(face);
    if !font_format.is_null()
        && (strcmp(font_format, b"Type 1\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(font_format, b"CFF\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_ft_load_type1_data(
    mut abstract_font: *mut libc::c_void,
    mut offset: libc::c_long,
    mut buffer: *mut libc::c_uchar,
    mut length: *mut libc::c_ulong,
) -> cairo_int_status_t {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*scaled_font).unscaled;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut available_length: libc::c_ulong = 0;
    let mut ret: libc::c_ulong = 0;
    if !length.is_null() {} else {
        __assert_fail(
            b"length != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
            3108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"cairo_int_status_t _cairo_ft_load_type1_data(void *, long, unsigned char *, unsigned long *)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_ft_scaled_font_is_vertical(&mut (*scaled_font).base) != 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    face = _cairo_ft_unscaled_font_lock_face(unscaled);
    if face.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if (*face).face_flags & (1 as libc::c_long) << 3 as libc::c_int != 0 {
        status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    } else if _ft_is_type1(face) == 0 {
        status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    } else {
        available_length = if ((*(*face).stream).size)
            .wrapping_sub(offset as libc::c_ulong) > 0 as libc::c_int as libc::c_ulong
        {
            ((*(*face).stream).size).wrapping_sub(offset as libc::c_ulong)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if buffer.is_null() {
            *length = available_length;
        } else if *length > available_length {
            status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        } else if ((*(*face).stream).read).is_some() {
            ret = (Some(((*(*face).stream).read).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*face).stream, offset as libc::c_ulong, buffer, *length);
            if ret != *length {
                status = _cairo_error(CAIRO_STATUS_READ_ERROR);
            }
        } else {
            memcpy(
                buffer as *mut libc::c_void,
                ((*(*face).stream).base).offset(offset as isize) as *const libc::c_void,
                *length,
            );
        }
    }
    _cairo_ft_unscaled_font_unlock_face(unscaled);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ft_has_color_glyphs(
    mut scaled: *mut libc::c_void,
) -> cairo_bool_t {
    let mut unscaled: *mut cairo_ft_unscaled_font_t = (*(scaled
        as *mut cairo_ft_scaled_font_t))
        .unscaled;
    if (*unscaled).have_color_set() == 0 {
        let mut face: FT_Face = 0 as *mut FT_FaceRec_;
        face = _cairo_ft_unscaled_font_lock_face(unscaled);
        if face.is_null() {
            return 0 as libc::c_int;
        }
        _cairo_ft_unscaled_font_unlock_face(unscaled);
    }
    return (*unscaled).have_color() as cairo_bool_t;
}
static mut _cairo_ft_scaled_font_backend: cairo_scaled_font_backend_t = unsafe {
    {
        let mut init = _cairo_scaled_font_backend {
            type_0: CAIRO_FONT_TYPE_FT,
            fini: Some(
                _cairo_ft_scaled_font_fini
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            scaled_glyph_init: Some(
                _cairo_ft_scaled_glyph_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_scaled_glyph_t,
                        cairo_scaled_glyph_info_t,
                        *const cairo_color_t,
                    ) -> cairo_int_status_t,
            ),
            text_to_glyphs: None,
            ucs4_to_index: Some(
                _cairo_ft_ucs4_to_index
                    as unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_ulong,
            ),
            load_truetype_table: Some(
                _cairo_ft_load_truetype_table
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_ulong,
                        libc::c_long,
                        *mut libc::c_uchar,
                        *mut libc::c_ulong,
                    ) -> cairo_int_status_t,
            ),
            index_to_ucs4: Some(
                _cairo_ft_index_to_ucs4
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_ulong,
                        *mut uint32_t,
                    ) -> cairo_int_status_t,
            ),
            is_synthetic: Some(
                _cairo_ft_is_synthetic
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_bool_t,
                    ) -> cairo_int_status_t,
            ),
            index_to_glyph_name: Some(
                _cairo_index_to_glyph_name
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        libc::c_ulong,
                        *mut libc::c_ulong,
                    ) -> cairo_int_status_t,
            ),
            load_type1_data: Some(
                _cairo_ft_load_type1_data
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_long,
                        *mut libc::c_uchar,
                        *mut libc::c_ulong,
                    ) -> cairo_int_status_t,
            ),
            has_color_glyphs: Some(
                _cairo_ft_has_color_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
        };
        init
    }
};
unsafe extern "C" fn _cairo_ft_font_face_create_for_toy(
    mut toy_face: *mut cairo_toy_font_face_t,
    mut font_face_out: *mut *mut cairo_font_face_t,
) -> cairo_status_t {
    let mut font_face: *mut cairo_font_face_t = &_cairo_font_face_nil
        as *const cairo_font_face_t as *mut cairo_font_face_t;
    let mut pattern: *mut FcPattern = 0 as *mut FcPattern;
    let mut fcslant: libc::c_int = 0;
    let mut fcweight: libc::c_int = 0;
    pattern = FcPatternCreate();
    if pattern.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return (*font_face).status;
    }
    if FcPatternAddString(
        pattern,
        b"family\0" as *const u8 as *const libc::c_char,
        (*toy_face).family as *mut libc::c_uchar,
    ) == 0
    {
        let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        match (*toy_face).slant as libc::c_uint {
            1 => {
                fcslant = 100 as libc::c_int;
            }
            2 => {
                fcslant = 110 as libc::c_int;
            }
            0 | _ => {
                fcslant = 0 as libc::c_int;
            }
        }
        if FcPatternAddInteger(
            pattern,
            b"slant\0" as *const u8 as *const libc::c_char,
            fcslant,
        ) == 0
        {
            let mut status___1: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            match (*toy_face).weight as libc::c_uint {
                1 => {
                    fcweight = 200 as libc::c_int;
                }
                0 | _ => {
                    fcweight = 100 as libc::c_int;
                }
            }
            if FcPatternAddInteger(
                pattern,
                b"weight\0" as *const u8 as *const libc::c_char,
                fcweight,
            ) == 0
            {
                let mut status___2: cairo_status_t = _cairo_error(
                    CAIRO_STATUS_NO_MEMORY,
                );
            } else {
                font_face = _cairo_ft_font_face_create_for_pattern(pattern);
            }
        }
    }
    FcPatternDestroy(pattern);
    *font_face_out = font_face;
    return (*font_face).status;
}
unsafe extern "C" fn _cairo_ft_font_face_destroy(
    mut abstract_face: *mut libc::c_void,
) -> cairo_bool_t {
    let mut font_face: *mut cairo_ft_font_face_t = abstract_face
        as *mut cairo_ft_font_face_t;
    if !((*font_face).unscaled).is_null() && (*(*font_face).unscaled).from_face != 0
        && ((*font_face).next).is_null() && (*(*font_face).unscaled).faces == font_face
        && _cairo_atomic_int_get(&mut (*(*font_face).unscaled).base.ref_count.ref_count)
            > 1 as libc::c_int
    {
        _cairo_unscaled_font_destroy(&mut (*(*font_face).unscaled).base);
        let ref mut fresh26 = (*font_face).unscaled;
        *fresh26 = 0 as *mut cairo_ft_unscaled_font_t;
        return 0 as libc::c_int;
    }
    if !((*font_face).unscaled).is_null() {
        let mut tmp_face: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
        let mut last_face: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
        tmp_face = (*(*font_face).unscaled).faces;
        while !tmp_face.is_null() {
            if tmp_face == font_face {
                if !last_face.is_null() {
                    let ref mut fresh27 = (*last_face).next;
                    *fresh27 = (*tmp_face).next;
                } else {
                    let ref mut fresh28 = (*(*font_face).unscaled).faces;
                    *fresh28 = (*tmp_face).next;
                }
            }
            last_face = tmp_face;
            tmp_face = (*tmp_face).next;
        }
        _cairo_unscaled_font_destroy(&mut (*(*font_face).unscaled).base);
        let ref mut fresh29 = (*font_face).unscaled;
        *fresh29 = 0 as *mut cairo_ft_unscaled_font_t;
    }
    _cairo_ft_options_fini(&mut (*font_face).ft_options);
    if !((*font_face).pattern).is_null() {
        FcPatternDestroy((*font_face).pattern);
        cairo_font_face_destroy((*font_face).resolved_font_face);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_ft_font_face_get_implementation(
    mut abstract_face: *mut libc::c_void,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut options: *const cairo_font_options_t,
) -> *mut cairo_font_face_t {
    let mut font_face: *mut cairo_ft_font_face_t = abstract_face
        as *mut cairo_ft_font_face_t;
    if !((*font_face).pattern).is_null() {
        let mut resolved: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
        resolved = (*font_face).resolved_font_face;
        if !resolved.is_null() {
            if FcInitBringUptoDate() == 0 {
                let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                return &_cairo_font_face_nil as *const cairo_font_face_t
                    as *mut cairo_font_face_t;
            }
            if (*font_face).resolved_config == FcConfigGetCurrent() {
                return cairo_font_face_reference(resolved);
            }
            cairo_font_face_destroy(resolved);
            let ref mut fresh30 = (*font_face).resolved_font_face;
            *fresh30 = 0 as *mut cairo_font_face_t;
        }
        resolved = _cairo_ft_resolve_pattern(
            (*font_face).pattern,
            font_matrix,
            ctm,
            options,
        );
        if (*resolved).status as u64 != 0 {
            return resolved;
        }
        let ref mut fresh31 = (*font_face).resolved_font_face;
        *fresh31 = cairo_font_face_reference(resolved);
        let ref mut fresh32 = (*font_face).resolved_config;
        *fresh32 = FcConfigGetCurrent();
        return resolved;
    }
    return abstract_face as *mut cairo_font_face_t;
}
#[no_mangle]
pub static mut _cairo_ft_font_face_backend: cairo_font_face_backend_t = unsafe {
    {
        let mut init = _cairo_font_face_backend {
            type_0: CAIRO_FONT_TYPE_FT,
            create_for_toy: Some(
                _cairo_ft_font_face_create_for_toy
                    as unsafe extern "C" fn(
                        *mut cairo_toy_font_face_t,
                        *mut *mut cairo_font_face_t,
                    ) -> cairo_status_t,
            ),
            destroy: Some(
                _cairo_ft_font_face_destroy
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            scaled_font_create: Some(
                _cairo_ft_font_face_scaled_font_create
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        *const cairo_font_options_t,
                        *mut *mut cairo_scaled_font_t,
                    ) -> cairo_status_t,
            ),
            get_implementation: Some(
                _cairo_ft_font_face_get_implementation
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const cairo_matrix_t,
                        *const cairo_matrix_t,
                        *const cairo_font_options_t,
                    ) -> *mut cairo_font_face_t,
            ),
        };
        init
    }
};
unsafe extern "C" fn _cairo_ft_font_face_create_for_pattern(
    mut pattern: *mut FcPattern,
) -> *mut cairo_font_face_t {
    let mut font_face: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
    font_face = (if ::std::mem::size_of::<cairo_ft_font_face_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_ft_font_face_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_ft_font_face_t;
    if font_face.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    let ref mut fresh33 = (*font_face).unscaled;
    *fresh33 = 0 as *mut cairo_ft_unscaled_font_t;
    _get_pattern_ft_options(pattern, &mut (*font_face).ft_options);
    let ref mut fresh34 = (*font_face).next;
    *fresh34 = 0 as *mut cairo_ft_font_face_t;
    let ref mut fresh35 = (*font_face).pattern;
    *fresh35 = FcPatternDuplicate(pattern);
    if ((*font_face).pattern).is_null() {
        free(font_face as *mut libc::c_void);
        let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    let ref mut fresh36 = (*font_face).resolved_font_face;
    *fresh36 = 0 as *mut cairo_font_face_t;
    let ref mut fresh37 = (*font_face).resolved_config;
    *fresh37 = 0 as *mut FcConfig;
    _cairo_font_face_init(&mut (*font_face).base, &_cairo_ft_font_face_backend);
    return &mut (*font_face).base;
}
unsafe extern "C" fn _cairo_ft_font_face_create(
    mut unscaled: *mut cairo_ft_unscaled_font_t,
    mut ft_options: *mut cairo_ft_options_t,
) -> *mut cairo_font_face_t {
    let mut font_face: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
    let mut prev_font_face: *mut *mut cairo_ft_font_face_t = 0
        as *mut *mut cairo_ft_font_face_t;
    font_face = (*unscaled).faces;
    prev_font_face = &mut (*unscaled).faces;
    while !font_face.is_null() {
        if (*font_face).ft_options.load_flags == (*ft_options).load_flags
            && (*font_face).ft_options.synth_flags == (*ft_options).synth_flags
            && cairo_font_options_equal(
                &mut (*font_face).ft_options.base,
                &mut (*ft_options).base,
            ) != 0
        {
            if (*font_face).base.status as u64 != 0 {
                *prev_font_face = (*font_face).next;
                break;
            } else if ((*font_face).unscaled).is_null() {
                let ref mut fresh38 = (*font_face).unscaled;
                *fresh38 = unscaled;
                _cairo_unscaled_font_reference(&mut (*unscaled).base);
                return &mut (*font_face).base;
            } else {
                return cairo_font_face_reference(&mut (*font_face).base)
            }
        } else {
            prev_font_face = &mut (*font_face).next;
            font_face = (*font_face).next;
        }
    }
    font_face = (if ::std::mem::size_of::<cairo_ft_font_face_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_ft_font_face_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_ft_font_face_t;
    if font_face.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    let ref mut fresh39 = (*font_face).unscaled;
    *fresh39 = unscaled;
    _cairo_unscaled_font_reference(&mut (*unscaled).base);
    _cairo_ft_options_init_copy(&mut (*font_face).ft_options, ft_options);
    if !((*unscaled).faces).is_null() && ((*(*unscaled).faces).unscaled).is_null() {
        if (*unscaled).from_face != 0 && ((*(*unscaled).faces).next).is_null() {} else {
            __assert_fail(
                b"unscaled->from_face && unscaled->faces->next == NULL\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-ft-font.c\0" as *const u8 as *const libc::c_char,
                3479 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"cairo_font_face_t *_cairo_ft_font_face_create(cairo_ft_unscaled_font_t *, cairo_ft_options_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        cairo_font_face_destroy(&mut (*(*unscaled).faces).base);
        let ref mut fresh40 = (*unscaled).faces;
        *fresh40 = 0 as *mut cairo_ft_font_face_t;
    }
    let ref mut fresh41 = (*font_face).next;
    *fresh41 = (*unscaled).faces;
    let ref mut fresh42 = (*unscaled).faces;
    *fresh42 = font_face;
    let ref mut fresh43 = (*font_face).pattern;
    *fresh43 = 0 as *mut FcPattern;
    _cairo_font_face_init(&mut (*font_face).base, &_cairo_ft_font_face_backend);
    return &mut (*font_face).base;
}
unsafe extern "C" fn _cairo_ft_font_options_substitute(
    mut options: *const cairo_font_options_t,
    mut pattern: *mut FcPattern,
) -> cairo_status_t {
    let mut v: FcValue = FcValue {
        type_0: FcTypeVoid,
        u: C2RustUnnamed_0 {
            s: 0 as *const FcChar8,
        },
    };
    if (*options).antialias as libc::c_uint
        != CAIRO_ANTIALIAS_DEFAULT as libc::c_int as libc::c_uint
    {
        if FcPatternGet(
            pattern,
            b"antialias\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut v,
        ) as libc::c_uint == FcResultNoMatch as libc::c_int as libc::c_uint
        {
            if FcPatternAddBool(
                pattern,
                b"antialias\0" as *const u8 as *const libc::c_char,
                ((*options).antialias as libc::c_uint
                    != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint)
                    as libc::c_int,
            ) == 0
            {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            if (*options).antialias as libc::c_uint
                != CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
            {
                FcPatternDel(pattern, b"rgba\0" as *const u8 as *const libc::c_char);
                if FcPatternAddInteger(
                    pattern,
                    b"rgba\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) == 0
                {
                    return _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
            }
        }
    }
    if (*options).antialias as libc::c_uint
        != CAIRO_ANTIALIAS_DEFAULT as libc::c_int as libc::c_uint
    {
        if FcPatternGet(
            pattern,
            b"rgba\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut v,
        ) as libc::c_uint == FcResultNoMatch as libc::c_int as libc::c_uint
        {
            let mut rgba: libc::c_int = 0;
            if (*options).antialias as libc::c_uint
                == CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
            {
                match (*options).subpixel_order as libc::c_uint {
                    2 => {
                        rgba = 2 as libc::c_int;
                    }
                    3 => {
                        rgba = 3 as libc::c_int;
                    }
                    4 => {
                        rgba = 4 as libc::c_int;
                    }
                    0 | 1 | _ => {
                        rgba = 1 as libc::c_int;
                    }
                }
            } else {
                rgba = 5 as libc::c_int;
            }
            if FcPatternAddInteger(
                pattern,
                b"rgba\0" as *const u8 as *const libc::c_char,
                rgba,
            ) == 0
            {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
    }
    if (*options).lcd_filter as libc::c_uint
        != CAIRO_LCD_FILTER_DEFAULT as libc::c_int as libc::c_uint
    {
        if FcPatternGet(
            pattern,
            b"lcdfilter\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut v,
        ) as libc::c_uint == FcResultNoMatch as libc::c_int as libc::c_uint
        {
            let mut lcd_filter: libc::c_int = 0;
            match (*options).lcd_filter as libc::c_uint {
                1 => {
                    lcd_filter = 0 as libc::c_int;
                }
                2 => {
                    lcd_filter = 16 as libc::c_int;
                }
                3 => {
                    lcd_filter = 2 as libc::c_int;
                }
                0 | 4 | _ => {
                    lcd_filter = 1 as libc::c_int;
                }
            }
            if FcPatternAddInteger(
                pattern,
                b"lcdfilter\0" as *const u8 as *const libc::c_char,
                lcd_filter,
            ) == 0
            {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
    }
    if (*options).hint_style as libc::c_uint
        != CAIRO_HINT_STYLE_DEFAULT as libc::c_int as libc::c_uint
    {
        if FcPatternGet(
            pattern,
            b"hinting\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut v,
        ) as libc::c_uint == FcResultNoMatch as libc::c_int as libc::c_uint
        {
            if FcPatternAddBool(
                pattern,
                b"hinting\0" as *const u8 as *const libc::c_char,
                ((*options).hint_style as libc::c_uint
                    != CAIRO_HINT_STYLE_NONE as libc::c_int as libc::c_uint)
                    as libc::c_int,
            ) == 0
            {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
        if FcPatternGet(
            pattern,
            b"hintstyle\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            &mut v,
        ) as libc::c_uint == FcResultNoMatch as libc::c_int as libc::c_uint
        {
            let mut hint_style: libc::c_int = 0;
            match (*options).hint_style as libc::c_uint {
                1 => {
                    hint_style = 0 as libc::c_int;
                }
                2 => {
                    hint_style = 1 as libc::c_int;
                }
                3 => {
                    hint_style = 2 as libc::c_int;
                }
                4 | 0 | _ => {
                    hint_style = 3 as libc::c_int;
                }
            }
            if FcPatternAddInteger(
                pattern,
                b"hintstyle\0" as *const u8 as *const libc::c_char,
                hint_style,
            ) == 0
            {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_font_options_substitute(
    mut options: *const cairo_font_options_t,
    mut pattern: *mut FcPattern,
) {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return;
    }
    _cairo_ft_font_options_substitute(options, pattern);
}
unsafe extern "C" fn _cairo_ft_resolve_pattern(
    mut pattern: *mut FcPattern,
    mut font_matrix: *const cairo_matrix_t,
    mut ctm: *const cairo_matrix_t,
    mut font_options: *const cairo_font_options_t,
) -> *mut cairo_font_face_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut scale: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut resolved: *mut FcPattern = 0 as *mut FcPattern;
    let mut sf: cairo_ft_font_transform_t = cairo_ft_font_transform_t {
        x_scale: 0.,
        y_scale: 0.,
        shape: [[0.; 2]; 2],
    };
    let mut result: FcResult = FcResultMatch;
    let mut unscaled: *mut cairo_ft_unscaled_font_t = 0 as *mut cairo_ft_unscaled_font_t;
    let mut ft_options: cairo_ft_options_t = cairo_ft_options_t {
        base: cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        },
        load_flags: 0,
        synth_flags: 0,
    };
    let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    scale = *ctm;
    scale.y0 = 0 as libc::c_int as libc::c_double;
    scale.x0 = scale.y0;
    cairo_matrix_multiply(&mut scale, font_matrix, &mut scale);
    status = _compute_transform(&mut sf, &mut scale, 0 as *mut cairo_ft_unscaled_font_t);
    if status as u64 != 0 {
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    pattern = FcPatternDuplicate(pattern);
    if pattern.is_null() {
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    if FcPatternAddDouble(
        pattern,
        b"pixelsize\0" as *const u8 as *const libc::c_char,
        sf.y_scale,
    ) == 0
    {
        font_face = &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    } else if FcConfigSubstitute(0 as *mut FcConfig, pattern, FcMatchPattern) == 0 {
        font_face = &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    } else {
        status = _cairo_ft_font_options_substitute(font_options, pattern);
        if status as u64 != 0 {
            font_face = &_cairo_font_face_nil as *const cairo_font_face_t
                as *mut cairo_font_face_t;
        } else {
            FcDefaultSubstitute(pattern);
            status = _cairo_ft_unscaled_font_create_for_pattern(pattern, &mut unscaled);
            if status as u64 != 0 {
                font_face = &_cairo_font_face_nil as *const cairo_font_face_t
                    as *mut cairo_font_face_t;
            } else {
                if unscaled.is_null() {
                    resolved = FcFontMatch(0 as *mut FcConfig, pattern, &mut result);
                    if resolved.is_null() {
                        font_face = _cairo_font_face_twin_create_fallback();
                        current_block = 937408387176680137;
                    } else {
                        status = _cairo_ft_unscaled_font_create_for_pattern(
                            resolved,
                            &mut unscaled,
                        );
                        if status as libc::c_uint != 0 || unscaled.is_null() {
                            font_face = &_cairo_font_face_nil as *const cairo_font_face_t
                                as *mut cairo_font_face_t;
                            current_block = 14207774521742130516;
                        } else {
                            current_block = 1118134448028020070;
                        }
                    }
                } else {
                    resolved = pattern;
                    current_block = 1118134448028020070;
                }
                match current_block {
                    937408387176680137 => {}
                    _ => {
                        match current_block {
                            1118134448028020070 => {
                                _get_pattern_ft_options(resolved, &mut ft_options);
                                font_face = _cairo_ft_font_face_create(
                                    unscaled,
                                    &mut ft_options,
                                );
                                _cairo_ft_options_fini(&mut ft_options);
                                _cairo_unscaled_font_destroy(&mut (*unscaled).base);
                            }
                            _ => {}
                        }
                        if resolved != pattern {
                            FcPatternDestroy(resolved);
                        }
                    }
                }
            }
        }
    }
    FcPatternDestroy(pattern);
    return font_face;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_font_face_create_for_pattern(
    mut pattern: *mut FcPattern,
) -> *mut cairo_font_face_t {
    let mut unscaled: *mut cairo_ft_unscaled_font_t = 0 as *mut cairo_ft_unscaled_font_t;
    let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    let mut ft_options: cairo_ft_options_t = cairo_ft_options_t {
        base: cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        },
        load_flags: 0,
        synth_flags: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_ft_unscaled_font_create_for_pattern(pattern, &mut unscaled);
    if status as u64 != 0 {
        if status as libc::c_uint
            == CAIRO_STATUS_FILE_NOT_FOUND as libc::c_int as libc::c_uint
        {
            return &_cairo_font_face_nil_file_not_found as *const cairo_font_face_t
                as *mut cairo_font_face_t
        } else {
            return &_cairo_font_face_nil as *const cairo_font_face_t
                as *mut cairo_font_face_t
        }
    }
    if unscaled.is_null() {
        return _cairo_ft_font_face_create_for_pattern(pattern);
    }
    _get_pattern_ft_options(pattern, &mut ft_options);
    font_face = _cairo_ft_font_face_create(unscaled, &mut ft_options);
    _cairo_ft_options_fini(&mut ft_options);
    _cairo_unscaled_font_destroy(&mut (*unscaled).base);
    return font_face;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_font_face_create_for_ft_face(
    mut face: FT_Face,
    mut load_flags: libc::c_int,
) -> *mut cairo_font_face_t {
    let mut unscaled: *mut cairo_ft_unscaled_font_t = 0 as *mut cairo_ft_unscaled_font_t;
    let mut font_face: *mut cairo_font_face_t = 0 as *mut cairo_font_face_t;
    let mut ft_options: cairo_ft_options_t = cairo_ft_options_t {
        base: cairo_font_options_t {
            antialias: CAIRO_ANTIALIAS_DEFAULT,
            subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
            lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
            hint_style: CAIRO_HINT_STYLE_DEFAULT,
            hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
            round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
            variations: 0 as *mut libc::c_char,
            color_mode: CAIRO_COLOR_MODE_DEFAULT,
            palette_index: 0,
        },
        load_flags: 0,
        synth_flags: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_ft_unscaled_font_create_from_face(face, &mut unscaled);
    if status as u64 != 0 {
        return &_cairo_font_face_nil as *const cairo_font_face_t
            as *mut cairo_font_face_t;
    }
    ft_options.load_flags = load_flags as libc::c_uint;
    ft_options.synth_flags = 0 as libc::c_int as libc::c_uint;
    _cairo_font_options_init_default(&mut ft_options.base);
    font_face = _cairo_ft_font_face_create(unscaled, &mut ft_options);
    _cairo_unscaled_font_destroy(&mut (*unscaled).base);
    return font_face;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_font_face_set_synthesize(
    mut font_face: *mut cairo_font_face_t,
    mut synth_flags: libc::c_uint,
) {
    let mut ft: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
    if (*(*font_face).backend).type_0 as libc::c_uint
        != CAIRO_FONT_TYPE_FT as libc::c_int as libc::c_uint
    {
        return;
    }
    ft = font_face as *mut cairo_ft_font_face_t;
    (*ft).ft_options.synth_flags |= synth_flags;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_font_face_unset_synthesize(
    mut font_face: *mut cairo_font_face_t,
    mut synth_flags: libc::c_uint,
) {
    let mut ft: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
    if (*(*font_face).backend).type_0 as libc::c_uint
        != CAIRO_FONT_TYPE_FT as libc::c_int as libc::c_uint
    {
        return;
    }
    ft = font_face as *mut cairo_ft_font_face_t;
    (*ft).ft_options.synth_flags &= !synth_flags;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_font_face_get_synthesize(
    mut font_face: *mut cairo_font_face_t,
) -> libc::c_uint {
    let mut ft: *mut cairo_ft_font_face_t = 0 as *mut cairo_ft_font_face_t;
    if (*(*font_face).backend).type_0 as libc::c_uint
        != CAIRO_FONT_TYPE_FT as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    ft = font_face as *mut cairo_ft_font_face_t;
    return (*ft).ft_options.synth_flags;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_scaled_font_lock_face(
    mut abstract_font: *mut cairo_scaled_font_t,
) -> FT_Face {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    let mut face: FT_Face = 0 as *mut FT_FaceRec_;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _cairo_scaled_font_is_ft(abstract_font) == 0 {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_FONT_TYPE_MISMATCH);
        return 0 as FT_Face;
    }
    if (*scaled_font).base.status as u64 != 0 {
        return 0 as FT_Face;
    }
    face = _cairo_ft_unscaled_font_lock_face((*scaled_font).unscaled);
    if face.is_null() {
        status = _cairo_scaled_font_set_error(
            &mut (*scaled_font).base,
            CAIRO_STATUS_NO_MEMORY,
        );
        return 0 as FT_Face;
    }
    status = _cairo_ft_unscaled_font_set_scale(
        (*scaled_font).unscaled,
        &mut (*scaled_font).base.scale,
    );
    if status as u64 != 0 {
        _cairo_ft_unscaled_font_unlock_face((*scaled_font).unscaled);
        status = _cairo_scaled_font_set_error(&mut (*scaled_font).base, status);
        return 0 as FT_Face;
    }
    cairo_ft_apply_variations(face, scaled_font);
    pthread_mutex_unlock(&mut (*(*scaled_font).unscaled).mutex);
    return face;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ft_scaled_font_unlock_face(
    mut abstract_font: *mut cairo_scaled_font_t,
) {
    let mut scaled_font: *mut cairo_ft_scaled_font_t = abstract_font
        as *mut cairo_ft_scaled_font_t;
    if _cairo_scaled_font_is_ft(abstract_font) == 0 {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_FONT_TYPE_MISMATCH);
        return;
    }
    if (*scaled_font).base.status as u64 != 0 {
        return;
    }
    pthread_mutex_lock(&mut (*(*scaled_font).unscaled).mutex);
    _cairo_ft_unscaled_font_unlock_face((*scaled_font).unscaled);
}
unsafe extern "C" fn _cairo_ft_scaled_font_is_vertical(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> cairo_bool_t {
    let mut ft_scaled_font: *mut cairo_ft_scaled_font_t = 0
        as *mut cairo_ft_scaled_font_t;
    if _cairo_scaled_font_is_ft(scaled_font) == 0 {
        return 0 as libc::c_int;
    }
    ft_scaled_font = scaled_font as *mut cairo_ft_scaled_font_t;
    if (*ft_scaled_font).ft_options.load_flags as libc::c_long
        & (1 as libc::c_long) << 4 as libc::c_int != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_ft_scaled_font_get_load_flags(
    mut scaled_font: *mut cairo_scaled_font_t,
) -> libc::c_uint {
    let mut ft_scaled_font: *mut cairo_ft_scaled_font_t = 0
        as *mut cairo_ft_scaled_font_t;
    if _cairo_scaled_font_is_ft(scaled_font) == 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    ft_scaled_font = scaled_font as *mut cairo_ft_scaled_font_t;
    return (*ft_scaled_font).ft_options.load_flags;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_ft_font_reset_static_data() {
    _cairo_ft_unscaled_font_map_destroy();
}
