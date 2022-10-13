use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn cairo_matrix_transform_distance(
        matrix: *const cairo_matrix_t,
        dx: *mut libc::c_double,
        dy: *mut libc::c_double,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_hull_compute(
        vertices: *mut cairo_pen_vertex_t,
        num_vertices: *mut libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_matrix_transformed_circle_major_axis(
        matrix: *const cairo_matrix_t,
        radius: libc::c_double,
    ) -> libc::c_double;
    fn _cairo_matrix_compute_determinant(
        matrix: *const cairo_matrix_t,
    ) -> libc::c_double;
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pen_vertex {
    pub point: cairo_point_t,
    pub slope_ccw: cairo_slope_t,
    pub slope_cw: cairo_slope_t,
}
pub type cairo_pen_vertex_t = _cairo_pen_vertex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pen {
    pub radius: libc::c_double,
    pub tolerance: libc::c_double,
    pub num_vertices: libc::c_int,
    pub vertices: *mut cairo_pen_vertex_t,
    pub vertices_embedded: [cairo_pen_vertex_t; 32],
}
pub type cairo_pen_t = _cairo_pen;
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
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
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
pub unsafe extern "C" fn _cairo_pen_init(
    mut pen: *mut cairo_pen_t,
    mut radius: libc::c_double,
    mut tolerance: libc::c_double,
    mut ctm: *const cairo_matrix_t,
) -> cairo_status_t {
    let mut i: libc::c_int = 0;
    let mut reflect: libc::c_int = 0;
    (*pen).radius = radius;
    (*pen).tolerance = tolerance;
    reflect = (_cairo_matrix_compute_determinant(ctm) < 0.0f64) as libc::c_int;
    (*pen).num_vertices = _cairo_pen_vertices_needed(tolerance, radius, ctm);
    if (*pen).num_vertices
        > (::std::mem::size_of::<[cairo_pen_vertex_t; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong)
            as libc::c_int
    {
        let ref mut fresh4 = (*pen).vertices;
        *fresh4 = _cairo_malloc_ab(
            (*pen).num_vertices as size_t,
            ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
        ) as *mut cairo_pen_vertex_t;
        if ((*pen).vertices).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    } else {
        let ref mut fresh5 = (*pen).vertices;
        *fresh5 = ((*pen).vertices_embedded).as_mut_ptr();
    }
    i = 0 as libc::c_int;
    while i < (*pen).num_vertices {
        let mut v: *mut cairo_pen_vertex_t = &mut *((*pen).vertices).offset(i as isize)
            as *mut cairo_pen_vertex_t;
        let mut theta: libc::c_double = 2 as libc::c_int as libc::c_double
            * 3.14159265358979323846f64 * i as libc::c_double
            / (*pen).num_vertices as libc::c_double;
        let mut dx: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        if reflect != 0 {
            theta = -theta;
        }
        dx = radius * cos(theta);
        dy = radius * sin(theta);
        cairo_matrix_transform_distance(ctm, &mut dx, &mut dy);
        (*v).point.x = _cairo_fixed_from_double(dx);
        (*v).point.y = _cairo_fixed_from_double(dy);
        i += 1;
    }
    _cairo_pen_compute_slopes(pen);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_fini(mut pen: *mut cairo_pen_t) {
    if (*pen).vertices != ((*pen).vertices_embedded).as_mut_ptr() {
        free((*pen).vertices as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_init_copy(
    mut pen: *mut cairo_pen_t,
    mut other: *const cairo_pen_t,
) -> cairo_status_t {
    *pen = *other;
    let ref mut fresh6 = (*pen).vertices;
    *fresh6 = ((*pen).vertices_embedded).as_mut_ptr();
    if (*pen).num_vertices != 0 {
        if (*pen).num_vertices
            > (::std::mem::size_of::<[cairo_pen_vertex_t; 32]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
                ) as libc::c_int
        {
            let ref mut fresh7 = (*pen).vertices;
            *fresh7 = _cairo_malloc_ab(
                (*pen).num_vertices as size_t,
                ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
            ) as *mut cairo_pen_vertex_t;
            if ((*pen).vertices).is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
        memcpy(
            (*pen).vertices as *mut libc::c_void,
            (*other).vertices as *const libc::c_void,
            ((*pen).num_vertices as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
                ),
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_add_points(
    mut pen: *mut cairo_pen_t,
    mut point: *mut cairo_point_t,
    mut num_points: libc::c_int,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut num_vertices: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_vertices = (*pen).num_vertices + num_points;
    if num_vertices
        > (::std::mem::size_of::<[cairo_pen_vertex_t; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong)
            as libc::c_int || (*pen).vertices != ((*pen).vertices_embedded).as_mut_ptr()
    {
        let mut vertices: *mut cairo_pen_vertex_t = 0 as *mut cairo_pen_vertex_t;
        if (*pen).vertices == ((*pen).vertices_embedded).as_mut_ptr() {
            vertices = _cairo_malloc_ab(
                num_vertices as size_t,
                ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
            ) as *mut cairo_pen_vertex_t;
            if vertices.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            memcpy(
                vertices as *mut libc::c_void,
                (*pen).vertices as *const libc::c_void,
                ((*pen).num_vertices as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
                    ),
            );
        } else {
            vertices = _cairo_realloc_ab(
                (*pen).vertices as *mut libc::c_void,
                num_vertices as size_t,
                ::std::mem::size_of::<cairo_pen_vertex_t>() as libc::c_ulong,
            ) as *mut cairo_pen_vertex_t;
            if vertices.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
        }
        let ref mut fresh8 = (*pen).vertices;
        *fresh8 = vertices;
    }
    (*pen).num_vertices = num_vertices;
    i = 0 as libc::c_int;
    while i < num_points {
        (*((*pen).vertices).offset(((*pen).num_vertices - num_points + i) as isize))
            .point = *point.offset(i as isize);
        i += 1;
    }
    status = _cairo_hull_compute((*pen).vertices, &mut (*pen).num_vertices);
    if status as u64 != 0 {
        return status;
    }
    _cairo_pen_compute_slopes(pen);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_vertices_needed(
    mut tolerance: libc::c_double,
    mut radius: libc::c_double,
    mut matrix: *const cairo_matrix_t,
) -> libc::c_int {
    let mut major_axis: libc::c_double = _cairo_matrix_transformed_circle_major_axis(
        matrix,
        radius,
    );
    let mut num_vertices: libc::c_int = 0;
    if tolerance >= 4 as libc::c_int as libc::c_double * major_axis {
        num_vertices = 1 as libc::c_int;
    } else if tolerance >= major_axis {
        num_vertices = 4 as libc::c_int;
    } else {
        num_vertices = ceil(
            2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                / acos(1 as libc::c_int as libc::c_double - tolerance / major_axis),
        ) as libc::c_int;
        if num_vertices % 2 as libc::c_int != 0 {
            num_vertices += 1;
        }
        if num_vertices < 4 as libc::c_int {
            num_vertices = 4 as libc::c_int;
        }
    }
    return num_vertices;
}
unsafe extern "C" fn _cairo_pen_compute_slopes(mut pen: *mut cairo_pen_t) {
    let mut i: libc::c_int = 0;
    let mut i_prev: libc::c_int = 0;
    let mut prev: *mut cairo_pen_vertex_t = 0 as *mut cairo_pen_vertex_t;
    let mut v: *mut cairo_pen_vertex_t = 0 as *mut cairo_pen_vertex_t;
    let mut next: *mut cairo_pen_vertex_t = 0 as *mut cairo_pen_vertex_t;
    i = 0 as libc::c_int;
    i_prev = (*pen).num_vertices - 1 as libc::c_int;
    while i < (*pen).num_vertices {
        prev = &mut *((*pen).vertices).offset(i_prev as isize)
            as *mut cairo_pen_vertex_t;
        v = &mut *((*pen).vertices).offset(i as isize) as *mut cairo_pen_vertex_t;
        next = &mut *((*pen).vertices)
            .offset(((i + 1 as libc::c_int) % (*pen).num_vertices) as isize)
            as *mut cairo_pen_vertex_t;
        _cairo_slope_init(&mut (*v).slope_cw, &mut (*prev).point, &mut (*v).point);
        _cairo_slope_init(&mut (*v).slope_ccw, &mut (*v).point, &mut (*next).point);
        let fresh9 = i;
        i = i + 1;
        i_prev = fresh9;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_find_active_cw_vertex_index(
    mut pen: *const cairo_pen_t,
    mut slope: *const cairo_slope_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*pen).num_vertices {
        if _cairo_slope_compare(
            slope,
            &mut (*((*pen).vertices).offset(i as isize)).slope_ccw,
        ) < 0 as libc::c_int
            && _cairo_slope_compare(
                slope,
                &mut (*((*pen).vertices).offset(i as isize)).slope_cw,
            ) >= 0 as libc::c_int
        {
            break;
        }
        i += 1;
    }
    if i == (*pen).num_vertices {
        i = 0 as libc::c_int;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_find_active_ccw_vertex_index(
    mut pen: *const cairo_pen_t,
    mut slope: *const cairo_slope_t,
) -> libc::c_int {
    let mut slope_reverse: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut i: libc::c_int = 0;
    slope_reverse = *slope;
    slope_reverse.dx = -slope_reverse.dx;
    slope_reverse.dy = -slope_reverse.dy;
    i = (*pen).num_vertices - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if _cairo_slope_compare(
            &mut (*((*pen).vertices).offset(i as isize)).slope_ccw,
            &mut slope_reverse,
        ) >= 0 as libc::c_int
            && _cairo_slope_compare(
                &mut (*((*pen).vertices).offset(i as isize)).slope_cw,
                &mut slope_reverse,
            ) < 0 as libc::c_int
        {
            break;
        }
        i -= 1;
    }
    if i < 0 as libc::c_int {
        i = (*pen).num_vertices - 1 as libc::c_int;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_find_active_cw_vertices(
    mut pen: *const cairo_pen_t,
    mut in_0: *const cairo_slope_t,
    mut out: *const cairo_slope_t,
    mut start: *mut libc::c_int,
    mut stop: *mut libc::c_int,
) {
    let mut lo: libc::c_int = 0 as libc::c_int;
    let mut hi: libc::c_int = (*pen).num_vertices;
    let mut i: libc::c_int = 0;
    i = lo + hi >> 1 as libc::c_int;
    loop {
        if _cairo_slope_compare(
            &mut (*((*pen).vertices).offset(i as isize)).slope_cw,
            in_0,
        ) < 0 as libc::c_int
        {
            lo = i;
        } else {
            hi = i;
        }
        i = lo + hi >> 1 as libc::c_int;
        if !(hi - lo > 1 as libc::c_int) {
            break;
        }
    }
    if _cairo_slope_compare(&mut (*((*pen).vertices).offset(i as isize)).slope_cw, in_0)
        < 0 as libc::c_int
    {
        i += 1;
        if i == (*pen).num_vertices {
            i = 0 as libc::c_int;
        }
    }
    *start = i;
    if _cairo_slope_compare(out, &mut (*((*pen).vertices).offset(i as isize)).slope_ccw)
        >= 0 as libc::c_int
    {
        lo = i;
        hi = i + (*pen).num_vertices;
        i = lo + hi >> 1 as libc::c_int;
        loop {
            let mut j: libc::c_int = i;
            if j >= (*pen).num_vertices {
                j -= (*pen).num_vertices;
            }
            if _cairo_slope_compare(
                &mut (*((*pen).vertices).offset(j as isize)).slope_cw,
                out,
            ) > 0 as libc::c_int
            {
                hi = i;
            } else {
                lo = i;
            }
            i = lo + hi >> 1 as libc::c_int;
            if !(hi - lo > 1 as libc::c_int) {
                break;
            }
        }
        if i >= (*pen).num_vertices {
            i -= (*pen).num_vertices;
        }
    }
    *stop = i;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pen_find_active_ccw_vertices(
    mut pen: *const cairo_pen_t,
    mut in_0: *const cairo_slope_t,
    mut out: *const cairo_slope_t,
    mut start: *mut libc::c_int,
    mut stop: *mut libc::c_int,
) {
    let mut lo: libc::c_int = 0 as libc::c_int;
    let mut hi: libc::c_int = (*pen).num_vertices;
    let mut i: libc::c_int = 0;
    i = lo + hi >> 1 as libc::c_int;
    loop {
        if _cairo_slope_compare(
            in_0,
            &mut (*((*pen).vertices).offset(i as isize)).slope_ccw,
        ) < 0 as libc::c_int
        {
            lo = i;
        } else {
            hi = i;
        }
        i = lo + hi >> 1 as libc::c_int;
        if !(hi - lo > 1 as libc::c_int) {
            break;
        }
    }
    if _cairo_slope_compare(in_0, &mut (*((*pen).vertices).offset(i as isize)).slope_ccw)
        < 0 as libc::c_int
    {
        i += 1;
        if i == (*pen).num_vertices {
            i = 0 as libc::c_int;
        }
    }
    *start = i;
    if _cairo_slope_compare(&mut (*((*pen).vertices).offset(i as isize)).slope_cw, out)
        <= 0 as libc::c_int
    {
        lo = i;
        hi = i + (*pen).num_vertices;
        i = lo + hi >> 1 as libc::c_int;
        loop {
            let mut j: libc::c_int = i;
            if j >= (*pen).num_vertices {
                j -= (*pen).num_vertices;
            }
            if _cairo_slope_compare(
                out,
                &mut (*((*pen).vertices).offset(j as isize)).slope_ccw,
            ) > 0 as libc::c_int
            {
                hi = i;
            } else {
                lo = i;
            }
            i = lo + hi >> 1 as libc::c_int;
            if !(hi - lo > 1 as libc::c_int) {
                break;
            }
        }
        if i >= (*pen).num_vertices {
            i -= (*pen).num_vertices;
        }
    }
    *stop = i;
}
