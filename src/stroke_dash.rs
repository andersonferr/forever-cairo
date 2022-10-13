use ::libc;
pub type cairo_bool_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_stroker_dash {
    pub dashed: cairo_bool_t,
    pub dash_index: libc::c_uint,
    pub dash_on: cairo_bool_t,
    pub dash_starts_on: cairo_bool_t,
    pub dash_remain: libc::c_double,
    pub dash_offset: libc::c_double,
    pub dashes: *const libc::c_double,
    pub num_dashes: libc::c_uint,
}
pub type cairo_stroker_dash_t = _cairo_stroker_dash;
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroker_dash_start(mut dash: *mut cairo_stroker_dash_t) {
    let mut offset: libc::c_double = 0.;
    let mut on: cairo_bool_t = 1 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*dash).dashed == 0 {
        return;
    }
    offset = (*dash).dash_offset;
    while offset > 0.0f64 && offset >= *((*dash).dashes).offset(i as isize) {
        offset -= *((*dash).dashes).offset(i as isize);
        on = (on == 0) as libc::c_int;
        i = i.wrapping_add(1);
        if i == (*dash).num_dashes {
            i = 0 as libc::c_int as libc::c_uint;
        }
    }
    (*dash).dash_index = i;
    let ref mut fresh0 = (*dash).dash_starts_on;
    *fresh0 = on;
    (*dash).dash_on = *fresh0;
    (*dash).dash_remain = *((*dash).dashes).offset(i as isize) - offset;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroker_dash_step(
    mut dash: *mut cairo_stroker_dash_t,
    mut step: libc::c_double,
) {
    (*dash).dash_remain -= step;
    if (*dash).dash_remain
        < 1.0f64
            / (2 as libc::c_int as libc::c_double
                * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double)
    {
        let ref mut fresh1 = (*dash).dash_index;
        *fresh1 = (*fresh1).wrapping_add(1);
        if *fresh1 == (*dash).num_dashes {
            (*dash).dash_index = 0 as libc::c_int as libc::c_uint;
        }
        (*dash).dash_on = ((*dash).dash_on == 0) as libc::c_int;
        (*dash).dash_remain += *((*dash).dashes).offset((*dash).dash_index as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_stroker_dash_init(
    mut dash: *mut cairo_stroker_dash_t,
    mut style: *const cairo_stroke_style_t,
) {
    (*dash)
        .dashed = ((*style).dash != 0 as *mut libc::c_void as *mut libc::c_double)
        as libc::c_int;
    if (*dash).dashed == 0 {
        return;
    }
    let ref mut fresh2 = (*dash).dashes;
    *fresh2 = (*style).dash;
    (*dash).num_dashes = (*style).num_dashes;
    (*dash).dash_offset = (*style).dash_offset;
    _cairo_stroker_dash_start(dash);
}
