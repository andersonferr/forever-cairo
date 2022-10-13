use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_string_hash(str: *const libc::c_char, len: libc::c_int) -> libc::c_ulong;
}
pub type cairo_bool_t = libc::c_int;
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
static mut _cairo_font_options_nil: cairo_font_options_t = {
    let mut init = _cairo_font_options {
        antialias: CAIRO_ANTIALIAS_DEFAULT,
        subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
        lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
        hint_style: CAIRO_HINT_STYLE_DEFAULT,
        hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
        round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
        variations: 0 as *const libc::c_char as *mut libc::c_char,
        color_mode: CAIRO_COLOR_MODE_DEFAULT,
        palette_index: 0 as libc::c_int as libc::c_uint,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_init_default(
    mut options: *mut cairo_font_options_t,
) {
    (*options).antialias = CAIRO_ANTIALIAS_DEFAULT;
    (*options).subpixel_order = CAIRO_SUBPIXEL_ORDER_DEFAULT;
    (*options).lcd_filter = CAIRO_LCD_FILTER_DEFAULT;
    (*options).hint_style = CAIRO_HINT_STYLE_DEFAULT;
    (*options).hint_metrics = CAIRO_HINT_METRICS_DEFAULT;
    (*options).round_glyph_positions = CAIRO_ROUND_GLYPH_POS_DEFAULT;
    let ref mut fresh0 = (*options).variations;
    *fresh0 = 0 as *mut libc::c_char;
    (*options).color_mode = CAIRO_COLOR_MODE_DEFAULT;
    (*options).palette_index = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_init_copy(
    mut options: *mut cairo_font_options_t,
    mut other: *const cairo_font_options_t,
) {
    (*options).antialias = (*other).antialias;
    (*options).subpixel_order = (*other).subpixel_order;
    (*options).lcd_filter = (*other).lcd_filter;
    (*options).hint_style = (*other).hint_style;
    (*options).hint_metrics = (*other).hint_metrics;
    (*options).round_glyph_positions = (*other).round_glyph_positions;
    let ref mut fresh1 = (*options).variations;
    *fresh1 = if !((*other).variations).is_null() {
        strdup((*other).variations)
    } else {
        0 as *mut libc::c_char
    };
    (*options).color_mode = (*other).color_mode;
    (*options).palette_index = (*other).palette_index;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_create() -> *mut cairo_font_options_t {
    let mut options: *mut cairo_font_options_t = 0 as *mut cairo_font_options_t;
    options = (if ::std::mem::size_of::<cairo_font_options_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_font_options_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_font_options_t;
    if options.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_font_options_nil as *const cairo_font_options_t
            as *mut cairo_font_options_t;
    }
    _cairo_font_options_init_default(options);
    return options;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_copy(
    mut original: *const cairo_font_options_t,
) -> *mut cairo_font_options_t {
    let mut options: *mut cairo_font_options_t = 0 as *mut cairo_font_options_t;
    if cairo_font_options_status(original as *mut cairo_font_options_t) as u64 != 0 {
        return &_cairo_font_options_nil as *const cairo_font_options_t
            as *mut cairo_font_options_t;
    }
    options = (if ::std::mem::size_of::<cairo_font_options_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_font_options_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_font_options_t;
    if options.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_font_options_nil as *const cairo_font_options_t
            as *mut cairo_font_options_t;
    }
    _cairo_font_options_init_copy(options, original);
    return options;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_fini(
    mut options: *mut cairo_font_options_t,
) {
    free((*options).variations as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_destroy(
    mut options: *mut cairo_font_options_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    _cairo_font_options_fini(options);
    free(options as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_status(
    mut options: *mut cairo_font_options_t,
) -> cairo_status_t {
    if options.is_null() {
        return CAIRO_STATUS_NULL_POINTER
    } else if options
        == &_cairo_font_options_nil as *const cairo_font_options_t
            as *mut cairo_font_options_t
    {
        return CAIRO_STATUS_NO_MEMORY
    } else {
        return CAIRO_STATUS_SUCCESS
    };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_merge(
    mut options: *mut cairo_font_options_t,
    mut other: *const cairo_font_options_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    if cairo_font_options_status(other as *mut cairo_font_options_t) as u64 != 0 {
        return;
    }
    if (*other).antialias as libc::c_uint
        != CAIRO_ANTIALIAS_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).antialias = (*other).antialias;
    }
    if (*other).subpixel_order as libc::c_uint
        != CAIRO_SUBPIXEL_ORDER_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).subpixel_order = (*other).subpixel_order;
    }
    if (*other).lcd_filter as libc::c_uint
        != CAIRO_LCD_FILTER_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).lcd_filter = (*other).lcd_filter;
    }
    if (*other).hint_style as libc::c_uint
        != CAIRO_HINT_STYLE_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).hint_style = (*other).hint_style;
    }
    if (*other).hint_metrics as libc::c_uint
        != CAIRO_HINT_METRICS_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).hint_metrics = (*other).hint_metrics;
    }
    if (*other).round_glyph_positions as libc::c_uint
        != CAIRO_ROUND_GLYPH_POS_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).round_glyph_positions = (*other).round_glyph_positions;
    }
    if !((*other).variations).is_null() {
        if !((*options).variations).is_null() {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            p = malloc(
                (strlen((*other).variations))
                    .wrapping_add(strlen((*options).variations))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            strcat(p, (*options).variations);
            strcat(p, b",\0" as *const u8 as *const libc::c_char);
            strcat(p, (*other).variations);
            free((*options).variations as *mut libc::c_void);
            let ref mut fresh2 = (*options).variations;
            *fresh2 = p;
        } else {
            let ref mut fresh3 = (*options).variations;
            *fresh3 = strdup((*other).variations);
        }
    }
    if (*other).color_mode as libc::c_uint
        != CAIRO_COLOR_MODE_DEFAULT as libc::c_int as libc::c_uint
    {
        (*options).color_mode = (*other).color_mode;
    }
    if (*other).palette_index != 0 as libc::c_int as libc::c_uint {
        (*options).palette_index = (*other).palette_index;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_equal(
    mut options: *const cairo_font_options_t,
    mut other: *const cairo_font_options_t,
) -> cairo_bool_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return 0 as libc::c_int;
    }
    if cairo_font_options_status(other as *mut cairo_font_options_t) as u64 != 0 {
        return 0 as libc::c_int;
    }
    if options == other {
        return 1 as libc::c_int;
    }
    return ((*options).antialias as libc::c_uint == (*other).antialias as libc::c_uint
        && (*options).subpixel_order as libc::c_uint
            == (*other).subpixel_order as libc::c_uint
        && (*options).lcd_filter as libc::c_uint == (*other).lcd_filter as libc::c_uint
        && (*options).hint_style as libc::c_uint == (*other).hint_style as libc::c_uint
        && (*options).hint_metrics as libc::c_uint
            == (*other).hint_metrics as libc::c_uint
        && (*options).round_glyph_positions as libc::c_uint
            == (*other).round_glyph_positions as libc::c_uint
        && (((*options).variations).is_null() && ((*other).variations).is_null()
            || !((*options).variations).is_null() && !((*other).variations).is_null()
                && strcmp((*options).variations, (*other).variations)
                    == 0 as libc::c_int)
        && (*options).color_mode as libc::c_uint == (*other).color_mode as libc::c_uint
        && (*options).palette_index == (*other).palette_index) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_hash(
    mut options: *const cairo_font_options_t,
) -> libc::c_ulong {
    let mut hash: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        options = &_cairo_font_options_nil;
    }
    if !((*options).variations).is_null() {
        hash = _cairo_string_hash(
            (*options).variations,
            strlen((*options).variations) as libc::c_int,
        );
    }
    hash ^= (*options).palette_index as libc::c_ulong;
    return ((*options).antialias as libc::c_uint
        | ((*options).subpixel_order as libc::c_uint) << 4 as libc::c_int
        | ((*options).lcd_filter as libc::c_uint) << 8 as libc::c_int
        | ((*options).hint_style as libc::c_uint) << 12 as libc::c_int
        | ((*options).hint_metrics as libc::c_uint) << 16 as libc::c_int
        | ((*options).color_mode as libc::c_uint) << 20 as libc::c_int) as libc::c_ulong
        ^ hash;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_antialias(
    mut options: *mut cairo_font_options_t,
    mut antialias: cairo_antialias_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).antialias = antialias;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_antialias(
    mut options: *const cairo_font_options_t,
) -> cairo_antialias_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_ANTIALIAS_DEFAULT;
    }
    return (*options).antialias;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_subpixel_order(
    mut options: *mut cairo_font_options_t,
    mut subpixel_order: cairo_subpixel_order_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).subpixel_order = subpixel_order;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_subpixel_order(
    mut options: *const cairo_font_options_t,
) -> cairo_subpixel_order_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_SUBPIXEL_ORDER_DEFAULT;
    }
    return (*options).subpixel_order;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_set_lcd_filter(
    mut options: *mut cairo_font_options_t,
    mut lcd_filter: cairo_lcd_filter_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).lcd_filter = lcd_filter;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_get_lcd_filter(
    mut options: *const cairo_font_options_t,
) -> cairo_lcd_filter_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_LCD_FILTER_DEFAULT;
    }
    return (*options).lcd_filter;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_set_round_glyph_positions(
    mut options: *mut cairo_font_options_t,
    mut round: cairo_round_glyph_positions_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).round_glyph_positions = round;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_font_options_get_round_glyph_positions(
    mut options: *const cairo_font_options_t,
) -> cairo_round_glyph_positions_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_ROUND_GLYPH_POS_DEFAULT;
    }
    return (*options).round_glyph_positions;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_hint_style(
    mut options: *mut cairo_font_options_t,
    mut hint_style: cairo_hint_style_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).hint_style = hint_style;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_hint_style(
    mut options: *const cairo_font_options_t,
) -> cairo_hint_style_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_HINT_STYLE_DEFAULT;
    }
    return (*options).hint_style;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_hint_metrics(
    mut options: *mut cairo_font_options_t,
    mut hint_metrics: cairo_hint_metrics_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).hint_metrics = hint_metrics;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_hint_metrics(
    mut options: *const cairo_font_options_t,
) -> cairo_hint_metrics_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_HINT_METRICS_DEFAULT;
    }
    return (*options).hint_metrics;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_variations(
    mut options: *mut cairo_font_options_t,
    mut variations: *const libc::c_char,
) {
    let mut tmp: *mut libc::c_char = if !variations.is_null() {
        strdup(variations)
    } else {
        0 as *mut libc::c_char
    };
    free((*options).variations as *mut libc::c_void);
    let ref mut fresh4 = (*options).variations;
    *fresh4 = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_variations(
    mut options: *mut cairo_font_options_t,
) -> *const libc::c_char {
    return (*options).variations;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_color_mode(
    mut options: *mut cairo_font_options_t,
    mut color_mode: cairo_color_mode_t,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).color_mode = color_mode;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_color_mode(
    mut options: *const cairo_font_options_t,
) -> cairo_color_mode_t {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return CAIRO_COLOR_MODE_DEFAULT;
    }
    return (*options).color_mode;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_set_color_palette(
    mut options: *mut cairo_font_options_t,
    mut palette_index: libc::c_uint,
) {
    if cairo_font_options_status(options) as u64 != 0 {
        return;
    }
    (*options).palette_index = palette_index;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_font_options_get_color_palette(
    mut options: *const cairo_font_options_t,
) -> libc::c_uint {
    if cairo_font_options_status(options as *mut cairo_font_options_t) as u64 != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    return (*options).palette_index;
}
