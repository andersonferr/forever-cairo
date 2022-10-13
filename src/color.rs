use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint16_t = libc::c_ushort;
pub type cairo_bool_t = libc::c_int;
pub type cairo_content_t = _cairo_content;
pub type _cairo_content = libc::c_uint;
pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;
pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
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
pub type uint16_t = __uint16_t;
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
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
static mut cairo_color_white: cairo_color_t = {
    let mut init = _cairo_color {
        red: 1.0f64,
        green: 1.0f64,
        blue: 1.0f64,
        alpha: 1.0f64,
        red_short: 0xffff as libc::c_int as libc::c_ushort,
        green_short: 0xffff as libc::c_int as libc::c_ushort,
        blue_short: 0xffff as libc::c_int as libc::c_ushort,
        alpha_short: 0xffff as libc::c_int as libc::c_ushort,
    };
    init
};
static mut cairo_color_black: cairo_color_t = {
    let mut init = _cairo_color {
        red: 0.0f64,
        green: 0.0f64,
        blue: 0.0f64,
        alpha: 1.0f64,
        red_short: 0 as libc::c_int as libc::c_ushort,
        green_short: 0 as libc::c_int as libc::c_ushort,
        blue_short: 0 as libc::c_int as libc::c_ushort,
        alpha_short: 0xffff as libc::c_int as libc::c_ushort,
    };
    init
};
static mut cairo_color_transparent: cairo_color_t = {
    let mut init = _cairo_color {
        red: 0.0f64,
        green: 0.0f64,
        blue: 0.0f64,
        alpha: 0.0f64,
        red_short: 0 as libc::c_int as libc::c_ushort,
        green_short: 0 as libc::c_int as libc::c_ushort,
        blue_short: 0 as libc::c_int as libc::c_ushort,
        alpha_short: 0 as libc::c_int as libc::c_ushort,
    };
    init
};
static mut cairo_color_magenta: cairo_color_t = {
    let mut init = _cairo_color {
        red: 1.0f64,
        green: 0.0f64,
        blue: 1.0f64,
        alpha: 1.0f64,
        red_short: 0xffff as libc::c_int as libc::c_ushort,
        green_short: 0 as libc::c_int as libc::c_ushort,
        blue_short: 0xffff as libc::c_int as libc::c_ushort,
        alpha_short: 0xffff as libc::c_int as libc::c_ushort,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_stock_color(
    mut stock: cairo_stock_t,
) -> *const cairo_color_t {
    match stock as libc::c_uint {
        0 => return &cairo_color_white,
        1 => return &cairo_color_black,
        2 => return &cairo_color_transparent,
        3 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-color.c\0" as *const u8 as *const libc::c_char,
                    73 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"const cairo_color_t *_cairo_stock_color(cairo_stock_t)\0"))
                        .as_ptr(),
                );
            }
            return &cairo_color_magenta;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_double_to_short(
    mut d: libc::c_double,
) -> uint16_t {
    return (d * 65535.0f64 + 0.5f64) as uint16_t;
}
unsafe extern "C" fn _cairo_color_compute_shorts(mut color: *mut cairo_color_t) {
    (*color).red_short = _cairo_color_double_to_short((*color).red * (*color).alpha);
    (*color).green_short = _cairo_color_double_to_short((*color).green * (*color).alpha);
    (*color).blue_short = _cairo_color_double_to_short((*color).blue * (*color).alpha);
    (*color).alpha_short = _cairo_color_double_to_short((*color).alpha);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_init_rgba(
    mut color: *mut cairo_color_t,
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
    mut alpha: libc::c_double,
) {
    (*color).red = red;
    (*color).green = green;
    (*color).blue = blue;
    (*color).alpha = alpha;
    _cairo_color_compute_shorts(color);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_multiply_alpha(
    mut color: *mut cairo_color_t,
    mut alpha: libc::c_double,
) {
    (*color).alpha *= alpha;
    _cairo_color_compute_shorts(color);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_get_rgba(
    mut color: *mut cairo_color_t,
    mut red: *mut libc::c_double,
    mut green: *mut libc::c_double,
    mut blue: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
) {
    *red = (*color).red;
    *green = (*color).green;
    *blue = (*color).blue;
    *alpha = (*color).alpha;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_get_rgba_premultiplied(
    mut color: *mut cairo_color_t,
    mut red: *mut libc::c_double,
    mut green: *mut libc::c_double,
    mut blue: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
) {
    *red = (*color).red * (*color).alpha;
    *green = (*color).green * (*color).alpha;
    *blue = (*color).blue * (*color).alpha;
    *alpha = (*color).alpha;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_equal(
    mut color_a: *const cairo_color_t,
    mut color_b: *const cairo_color_t,
) -> cairo_bool_t {
    if color_a == color_b {
        return 1 as libc::c_int;
    }
    if (*color_a).alpha_short as libc::c_int != (*color_b).alpha_short as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*color_a).alpha_short as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return ((*color_a).red_short as libc::c_int == (*color_b).red_short as libc::c_int
        && (*color_a).green_short as libc::c_int == (*color_b).green_short as libc::c_int
        && (*color_a).blue_short as libc::c_int == (*color_b).blue_short as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_stop_equal(
    mut color_a: *const cairo_color_stop_t,
    mut color_b: *const cairo_color_stop_t,
) -> cairo_bool_t {
    if color_a == color_b {
        return 1 as libc::c_int;
    }
    return ((*color_a).alpha_short as libc::c_int
        == (*color_b).alpha_short as libc::c_int
        && (*color_a).red_short as libc::c_int == (*color_b).red_short as libc::c_int
        && (*color_a).green_short as libc::c_int == (*color_b).green_short as libc::c_int
        && (*color_a).blue_short as libc::c_int == (*color_b).blue_short as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_color_get_content(
    mut color: *const cairo_color_t,
) -> cairo_content_t {
    if (*color).alpha_short as libc::c_int >= 0xff00 as libc::c_int {
        return CAIRO_CONTENT_COLOR;
    }
    if (*color).red_short as libc::c_int == 0 as libc::c_int
        && (*color).green_short as libc::c_int == 0 as libc::c_int
        && (*color).blue_short as libc::c_int == 0 as libc::c_int
    {
        return CAIRO_CONTENT_ALPHA;
    }
    return CAIRO_CONTENT_COLOR_ALPHA;
}
