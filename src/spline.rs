use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type cairo_bool_t = libc::c_int;
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
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
pub type cairo_spline_add_point_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const cairo_point_t,
        *const cairo_slope_t,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_spline_knots {
    pub a: cairo_point_t,
    pub b: cairo_point_t,
    pub c: cairo_point_t,
    pub d: cairo_point_t,
}
pub type cairo_spline_knots_t = _cairo_spline_knots;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_spline {
    pub add_point_func: cairo_spline_add_point_func_t,
    pub closure: *mut libc::c_void,
    pub knots: cairo_spline_knots_t,
    pub initial_slope: cairo_slope_t,
    pub final_slope: cairo_slope_t,
    pub has_point: cairo_bool_t,
    pub last_point: cairo_point_t,
}
pub type cairo_spline_t = _cairo_spline;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_double(mut d: libc::c_double) -> cairo_fixed_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
    u
        .d = d
        + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
            as libc::c_double * 1.5f64;
    return u.i[0 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_box_add_point(
    mut box_0: *mut cairo_box_t,
    mut point: *const cairo_point_t,
) {
    if (*point).x < (*box_0).p1.x {
        (*box_0).p1.x = (*point).x;
    } else if (*point).x > (*box_0).p2.x {
        (*box_0).p2.x = (*point).x;
    }
    if (*point).y < (*box_0).p1.y {
        (*box_0).p1.y = (*point).y;
    } else if (*point).y > (*box_0).p2.y {
        (*box_0).p2.y = (*point).y;
    }
}
#[inline]
unsafe extern "C" fn _cairo_box_contains_point(
    mut box_0: *const cairo_box_t,
    mut point: *const cairo_point_t,
) -> cairo_bool_t {
    return ((*box_0).p1.x <= (*point).x && (*point).x <= (*box_0).p2.x
        && (*box_0).p1.y <= (*point).y && (*point).y <= (*box_0).p2.y) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_slope_init(
    mut slope: *mut cairo_slope_t,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
) {
    (*slope).dx = (*b).x - (*a).x;
    (*slope).dy = (*b).y - (*a).y;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_spline_intersects(
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
    mut box_0: *const cairo_box_t,
) -> cairo_bool_t {
    let mut bounds: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if _cairo_box_contains_point(box_0, a) != 0
        || _cairo_box_contains_point(box_0, b) != 0
        || _cairo_box_contains_point(box_0, c) != 0
        || _cairo_box_contains_point(box_0, d) != 0
    {
        return 1 as libc::c_int;
    }
    bounds.p1 = *a;
    bounds.p2 = bounds.p1;
    _cairo_box_add_point(&mut bounds, b);
    _cairo_box_add_point(&mut bounds, c);
    _cairo_box_add_point(&mut bounds, d);
    if bounds.p2.x <= (*box_0).p1.x || bounds.p1.x >= (*box_0).p2.x
        || bounds.p2.y <= (*box_0).p1.y || bounds.p1.y >= (*box_0).p2.y
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_spline_init(
    mut spline: *mut cairo_spline_t,
    mut add_point_func: cairo_spline_add_point_func_t,
    mut closure: *mut libc::c_void,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
    mut c: *const cairo_point_t,
    mut d: *const cairo_point_t,
) -> cairo_bool_t {
    if (*a).x == (*b).x && (*a).y == (*b).y && (*c).x == (*d).x && (*c).y == (*d).y {
        return 0 as libc::c_int;
    }
    let ref mut fresh0 = (*spline).add_point_func;
    *fresh0 = add_point_func;
    let ref mut fresh1 = (*spline).closure;
    *fresh1 = closure;
    (*spline).knots.a = *a;
    (*spline).knots.b = *b;
    (*spline).knots.c = *c;
    (*spline).knots.d = *d;
    if (*a).x != (*b).x || (*a).y != (*b).y {
        _cairo_slope_init(
            &mut (*spline).initial_slope,
            &mut (*spline).knots.a,
            &mut (*spline).knots.b,
        );
    } else if (*a).x != (*c).x || (*a).y != (*c).y {
        _cairo_slope_init(
            &mut (*spline).initial_slope,
            &mut (*spline).knots.a,
            &mut (*spline).knots.c,
        );
    } else if (*a).x != (*d).x || (*a).y != (*d).y {
        _cairo_slope_init(
            &mut (*spline).initial_slope,
            &mut (*spline).knots.a,
            &mut (*spline).knots.d,
        );
    } else {
        return 0 as libc::c_int
    }
    if (*c).x != (*d).x || (*c).y != (*d).y {
        _cairo_slope_init(
            &mut (*spline).final_slope,
            &mut (*spline).knots.c,
            &mut (*spline).knots.d,
        );
    } else if (*b).x != (*d).x || (*b).y != (*d).y {
        _cairo_slope_init(
            &mut (*spline).final_slope,
            &mut (*spline).knots.b,
            &mut (*spline).knots.d,
        );
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_spline_add_point(
    mut spline: *mut cairo_spline_t,
    mut point: *const cairo_point_t,
    mut knot: *const cairo_point_t,
) -> cairo_status_t {
    let mut prev: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut slope: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    prev = &mut (*spline).last_point;
    if (*prev).x == (*point).x && (*prev).y == (*point).y {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_slope_init(&mut slope, point, knot);
    (*spline).last_point = *point;
    return ((*spline).add_point_func)
        .expect("non-null function pointer")((*spline).closure, point, &mut slope);
}
unsafe extern "C" fn _lerp_half(
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
    mut result: *mut cairo_point_t,
) {
    (*result).x = (*a).x + ((*b).x - (*a).x >> 1 as libc::c_int);
    (*result).y = (*a).y + ((*b).y - (*a).y >> 1 as libc::c_int);
}
unsafe extern "C" fn _de_casteljau(
    mut s1: *mut cairo_spline_knots_t,
    mut s2: *mut cairo_spline_knots_t,
) {
    let mut ab: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut bc: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut cd: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut abbc: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut bccd: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut final_0: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    _lerp_half(&mut (*s1).a, &mut (*s1).b, &mut ab);
    _lerp_half(&mut (*s1).b, &mut (*s1).c, &mut bc);
    _lerp_half(&mut (*s1).c, &mut (*s1).d, &mut cd);
    _lerp_half(&mut ab, &mut bc, &mut abbc);
    _lerp_half(&mut bc, &mut cd, &mut bccd);
    _lerp_half(&mut abbc, &mut bccd, &mut final_0);
    (*s2).a = final_0;
    (*s2).b = bccd;
    (*s2).c = cd;
    (*s2).d = (*s1).d;
    (*s1).b = ab;
    (*s1).c = abbc;
    (*s1).d = final_0;
}
unsafe extern "C" fn _cairo_spline_error_squared(
    mut knots: *const cairo_spline_knots_t,
) -> libc::c_double {
    let mut bdx: libc::c_double = 0.;
    let mut bdy: libc::c_double = 0.;
    let mut berr: libc::c_double = 0.;
    let mut cdx: libc::c_double = 0.;
    let mut cdy: libc::c_double = 0.;
    let mut cerr: libc::c_double = 0.;
    bdx = _cairo_fixed_to_double((*knots).b.x - (*knots).a.x);
    bdy = _cairo_fixed_to_double((*knots).b.y - (*knots).a.y);
    cdx = _cairo_fixed_to_double((*knots).c.x - (*knots).a.x);
    cdy = _cairo_fixed_to_double((*knots).c.y - (*knots).a.y);
    if (*knots).a.x != (*knots).d.x || (*knots).a.y != (*knots).d.y {
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        let mut u: libc::c_double = 0.;
        let mut v: libc::c_double = 0.;
        dx = _cairo_fixed_to_double((*knots).d.x - (*knots).a.x);
        dy = _cairo_fixed_to_double((*knots).d.y - (*knots).a.y);
        v = dx * dx + dy * dy;
        u = bdx * dx + bdy * dy;
        if !(u <= 0 as libc::c_int as libc::c_double) {
            if u >= v {
                bdx -= dx;
                bdy -= dy;
            } else {
                bdx -= u / v * dx;
                bdy -= u / v * dy;
            }
        }
        u = cdx * dx + cdy * dy;
        if !(u <= 0 as libc::c_int as libc::c_double) {
            if u >= v {
                cdx -= dx;
                cdy -= dy;
            } else {
                cdx -= u / v * dx;
                cdy -= u / v * dy;
            }
        }
    }
    berr = bdx * bdx + bdy * bdy;
    cerr = cdx * cdx + cdy * cdy;
    if berr > cerr { return berr } else { return cerr };
}
unsafe extern "C" fn _cairo_spline_decompose_into(
    mut s1: *mut cairo_spline_knots_t,
    mut tolerance_squared: libc::c_double,
    mut result: *mut cairo_spline_t,
) -> cairo_status_t {
    let mut s2: cairo_spline_knots_t = cairo_spline_knots_t {
        a: cairo_point_t { x: 0, y: 0 },
        b: cairo_point_t { x: 0, y: 0 },
        c: cairo_point_t { x: 0, y: 0 },
        d: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _cairo_spline_error_squared(s1) < tolerance_squared {
        return _cairo_spline_add_point(result, &mut (*s1).a, &mut (*s1).b);
    }
    _de_casteljau(s1, &mut s2);
    status = _cairo_spline_decompose_into(s1, tolerance_squared, result);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_spline_decompose_into(&mut s2, tolerance_squared, result);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_spline_decompose(
    mut spline: *mut cairo_spline_t,
    mut tolerance: libc::c_double,
) -> cairo_status_t {
    let mut s1: cairo_spline_knots_t = cairo_spline_knots_t {
        a: cairo_point_t { x: 0, y: 0 },
        b: cairo_point_t { x: 0, y: 0 },
        c: cairo_point_t { x: 0, y: 0 },
        d: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    s1 = (*spline).knots;
    (*spline).last_point = s1.a;
    status = _cairo_spline_decompose_into(&mut s1, tolerance * tolerance, spline);
    if status as u64 != 0 {
        return status;
    }
    return ((*spline).add_point_func)
        .expect(
            "non-null function pointer",
        )((*spline).closure, &mut (*spline).knots.d, &mut (*spline).final_slope);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_spline_bound(
    mut add_point_func: cairo_spline_add_point_func_t,
    mut closure: *mut libc::c_void,
    mut p0: *const cairo_point_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut x3: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut y3: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut t: [libc::c_double; 4] = [0.; 4];
    let mut t_num: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    x0 = _cairo_fixed_to_double((*p0).x);
    y0 = _cairo_fixed_to_double((*p0).y);
    x1 = _cairo_fixed_to_double((*p1).x);
    y1 = _cairo_fixed_to_double((*p1).y);
    x2 = _cairo_fixed_to_double((*p2).x);
    y2 = _cairo_fixed_to_double((*p2).y);
    x3 = _cairo_fixed_to_double((*p3).x);
    y3 = _cairo_fixed_to_double((*p3).y);
    a = -x0 + 3 as libc::c_int as libc::c_double * x1
        - 3 as libc::c_int as libc::c_double * x2 + x3;
    b = x0 - 2 as libc::c_int as libc::c_double * x1 + x2;
    c = -x0 + x1;
    if a == 0 as libc::c_int as libc::c_double {
        if b != 0 as libc::c_int as libc::c_double {
            let mut _t0: libc::c_double = -c / (2 as libc::c_int as libc::c_double * b);
            if (0 as libc::c_int as libc::c_double) < _t0
                && _t0 < 1 as libc::c_int as libc::c_double
            {
                let fresh2 = t_num;
                t_num = t_num + 1;
                t[fresh2 as usize] = _t0;
            }
        }
    } else {
        let mut b2: libc::c_double = b * b;
        let mut delta: libc::c_double = b2 - a * c;
        if delta > 0 as libc::c_int as libc::c_double {
            let mut feasible: cairo_bool_t = 0;
            let mut _2ab: libc::c_double = 2 as libc::c_int as libc::c_double * a * b;
            if _2ab >= 0 as libc::c_int as libc::c_double {
                feasible = (delta > b2 && delta < a * a + b2 + _2ab) as libc::c_int;
            } else if -b / a >= 1 as libc::c_int as libc::c_double {
                feasible = (delta < b2 && delta > a * a + b2 + _2ab) as libc::c_int;
            } else {
                feasible = (delta < b2 || delta < a * a + b2 + _2ab) as libc::c_int;
            }
            if feasible != 0 {
                let mut sqrt_delta: libc::c_double = sqrt(delta);
                let mut _t0_0: libc::c_double = (-b - sqrt_delta) / a;
                if (0 as libc::c_int as libc::c_double) < _t0_0
                    && _t0_0 < 1 as libc::c_int as libc::c_double
                {
                    let fresh3 = t_num;
                    t_num = t_num + 1;
                    t[fresh3 as usize] = _t0_0;
                }
                let mut _t0_1: libc::c_double = (-b + sqrt_delta) / a;
                if (0 as libc::c_int as libc::c_double) < _t0_1
                    && _t0_1 < 1 as libc::c_int as libc::c_double
                {
                    let fresh4 = t_num;
                    t_num = t_num + 1;
                    t[fresh4 as usize] = _t0_1;
                }
            }
        } else if delta == 0 as libc::c_int as libc::c_double {
            let mut _t0_2: libc::c_double = -b / a;
            if (0 as libc::c_int as libc::c_double) < _t0_2
                && _t0_2 < 1 as libc::c_int as libc::c_double
            {
                let fresh5 = t_num;
                t_num = t_num + 1;
                t[fresh5 as usize] = _t0_2;
            }
        }
    }
    a = -y0 + 3 as libc::c_int as libc::c_double * y1
        - 3 as libc::c_int as libc::c_double * y2 + y3;
    b = y0 - 2 as libc::c_int as libc::c_double * y1 + y2;
    c = -y0 + y1;
    if a == 0 as libc::c_int as libc::c_double {
        if b != 0 as libc::c_int as libc::c_double {
            let mut _t0_3: libc::c_double = -c
                / (2 as libc::c_int as libc::c_double * b);
            if (0 as libc::c_int as libc::c_double) < _t0_3
                && _t0_3 < 1 as libc::c_int as libc::c_double
            {
                let fresh6 = t_num;
                t_num = t_num + 1;
                t[fresh6 as usize] = _t0_3;
            }
        }
    } else {
        let mut b2_0: libc::c_double = b * b;
        let mut delta_0: libc::c_double = b2_0 - a * c;
        if delta_0 > 0 as libc::c_int as libc::c_double {
            let mut feasible_0: cairo_bool_t = 0;
            let mut _2ab_0: libc::c_double = 2 as libc::c_int as libc::c_double * a * b;
            if _2ab_0 >= 0 as libc::c_int as libc::c_double {
                feasible_0 = (delta_0 > b2_0 && delta_0 < a * a + b2_0 + _2ab_0)
                    as libc::c_int;
            } else if -b / a >= 1 as libc::c_int as libc::c_double {
                feasible_0 = (delta_0 < b2_0 && delta_0 > a * a + b2_0 + _2ab_0)
                    as libc::c_int;
            } else {
                feasible_0 = (delta_0 < b2_0 || delta_0 < a * a + b2_0 + _2ab_0)
                    as libc::c_int;
            }
            if feasible_0 != 0 {
                let mut sqrt_delta_0: libc::c_double = sqrt(delta_0);
                let mut _t0_4: libc::c_double = (-b - sqrt_delta_0) / a;
                if (0 as libc::c_int as libc::c_double) < _t0_4
                    && _t0_4 < 1 as libc::c_int as libc::c_double
                {
                    let fresh7 = t_num;
                    t_num = t_num + 1;
                    t[fresh7 as usize] = _t0_4;
                }
                let mut _t0_5: libc::c_double = (-b + sqrt_delta_0) / a;
                if (0 as libc::c_int as libc::c_double) < _t0_5
                    && _t0_5 < 1 as libc::c_int as libc::c_double
                {
                    let fresh8 = t_num;
                    t_num = t_num + 1;
                    t[fresh8 as usize] = _t0_5;
                }
            }
        } else if delta_0 == 0 as libc::c_int as libc::c_double {
            let mut _t0_6: libc::c_double = -b / a;
            if (0 as libc::c_int as libc::c_double) < _t0_6
                && _t0_6 < 1 as libc::c_int as libc::c_double
            {
                let fresh9 = t_num;
                t_num = t_num + 1;
                t[fresh9 as usize] = _t0_6;
            }
        }
    }
    status = add_point_func
        .expect("non-null function pointer")(closure, p0, 0 as *const cairo_slope_t);
    if status as u64 != 0 {
        return status;
    }
    i = 0 as libc::c_int;
    while i < t_num {
        let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
        let mut x: libc::c_double = 0.;
        let mut y: libc::c_double = 0.;
        let mut t_1_0: libc::c_double = 0.;
        let mut t_0_1: libc::c_double = 0.;
        let mut t_2_0: libc::c_double = 0.;
        let mut t_0_2: libc::c_double = 0.;
        let mut t_3_0: libc::c_double = 0.;
        let mut t_2_1_3: libc::c_double = 0.;
        let mut t_1_2_3: libc::c_double = 0.;
        let mut t_0_3: libc::c_double = 0.;
        t_1_0 = t[i as usize];
        t_0_1 = 1 as libc::c_int as libc::c_double - t_1_0;
        t_2_0 = t_1_0 * t_1_0;
        t_0_2 = t_0_1 * t_0_1;
        t_3_0 = t_2_0 * t_1_0;
        t_2_1_3 = t_2_0 * t_0_1 * 3 as libc::c_int as libc::c_double;
        t_1_2_3 = t_1_0 * t_0_2 * 3 as libc::c_int as libc::c_double;
        t_0_3 = t_0_1 * t_0_2;
        x = x0 * t_0_3 + x1 * t_1_2_3 + x2 * t_2_1_3 + x3 * t_3_0;
        y = y0 * t_0_3 + y1 * t_1_2_3 + y2 * t_2_1_3 + y3 * t_3_0;
        p.x = _cairo_fixed_from_double(x);
        p.y = _cairo_fixed_from_double(y);
        status = add_point_func
            .expect(
                "non-null function pointer",
            )(closure, &mut p, 0 as *const cairo_slope_t);
        if status as u64 != 0 {
            return status;
        }
        i += 1;
    }
    return add_point_func
        .expect("non-null function pointer")(closure, p3, 0 as *const cairo_slope_t);
}
