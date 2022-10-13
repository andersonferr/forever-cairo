use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_slope_compare(
        a: *const cairo_slope_t,
        b: *const cairo_slope_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type cairo_int64_t = int64_t;
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
pub type cairo_hull_t = cairo_hull;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_hull {
    pub point: cairo_point_t,
    pub slope: cairo_slope_t,
    pub discard: libc::c_int,
    pub id: libc::c_int,
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
unsafe extern "C" fn _cairo_slope_init(
    mut slope: *mut cairo_slope_t,
    mut a: *const cairo_point_t,
    mut b: *const cairo_point_t,
) {
    (*slope).dx = (*b).x - (*a).x;
    (*slope).dy = (*b).y - (*a).y;
}
unsafe extern "C" fn _cairo_hull_init(
    mut hull: *mut cairo_hull_t,
    mut vertices: *mut cairo_pen_vertex_t,
    mut num_vertices: libc::c_int,
) {
    let mut p: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut extremum: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut tmp: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut i: libc::c_int = 0;
    extremum = &mut (*vertices.offset(0 as libc::c_int as isize)).point;
    i = 1 as libc::c_int;
    while i < num_vertices {
        p = &mut (*vertices.offset(i as isize)).point;
        if (*p).y < (*extremum).y || (*p).y == (*extremum).y && (*p).x < (*extremum).x {
            extremum = p;
        }
        i += 1;
    }
    tmp = *extremum;
    *extremum = (*vertices.offset(0 as libc::c_int as isize)).point;
    (*vertices.offset(0 as libc::c_int as isize)).point = tmp;
    i = 0 as libc::c_int;
    while i < num_vertices {
        (*hull.offset(i as isize)).point = (*vertices.offset(i as isize)).point;
        _cairo_slope_init(
            &mut (*hull.offset(i as isize)).slope,
            &mut (*hull.offset(0 as libc::c_int as isize)).point,
            &mut (*hull.offset(i as isize)).point,
        );
        (*hull.offset(i as isize)).id = i;
        (*hull.offset(i as isize)).discard = 0 as libc::c_int;
        if i != 0 as libc::c_int
            && (*hull.offset(i as isize)).slope.dx == 0 as libc::c_int
            && (*hull.offset(i as isize)).slope.dy == 0 as libc::c_int
        {
            (*hull.offset(i as isize)).discard = 1 as libc::c_int;
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn _slope_length(mut slope: *mut cairo_slope_t) -> cairo_int64_t {
    return (*slope).dx as int64_t * (*slope).dx as libc::c_long
        + (*slope).dy as int64_t * (*slope).dy as libc::c_long;
}
unsafe extern "C" fn _cairo_hull_vertex_compare(
    mut av: *const libc::c_void,
    mut bv: *const libc::c_void,
) -> libc::c_int {
    let mut a: *mut cairo_hull_t = av as *mut cairo_hull_t;
    let mut b: *mut cairo_hull_t = bv as *mut cairo_hull_t;
    let mut ret: libc::c_int = 0;
    if a == b {
        return 0 as libc::c_int;
    }
    ret = _cairo_slope_compare(&mut (*a).slope, &mut (*b).slope);
    if ret == 0 as libc::c_int {
        let mut cmp: libc::c_int = 0;
        cmp = if _slope_length(&mut (*a).slope) == _slope_length(&mut (*b).slope) {
            0 as libc::c_int
        } else if _slope_length(&mut (*a).slope) < _slope_length(&mut (*b).slope) {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        if cmp < 0 as libc::c_int || cmp == 0 as libc::c_int && (*a).id < (*b).id {
            (*a).discard = 1 as libc::c_int;
            ret = -(1 as libc::c_int);
        } else {
            (*b).discard = 1 as libc::c_int;
            ret = 1 as libc::c_int;
        }
    }
    return ret;
}
unsafe extern "C" fn _cairo_hull_prev_valid(
    mut hull: *mut cairo_hull_t,
    mut num_hull: libc::c_int,
    mut index: libc::c_int,
) -> libc::c_int {
    if index == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    loop {
        index -= 1;
        if !((*hull.offset(index as isize)).discard != 0) {
            break;
        }
    }
    return index;
}
unsafe extern "C" fn _cairo_hull_next_valid(
    mut hull: *mut cairo_hull_t,
    mut num_hull: libc::c_int,
    mut index: libc::c_int,
) -> libc::c_int {
    loop {
        index = (index + 1 as libc::c_int) % num_hull;
        if !((*hull.offset(index as isize)).discard != 0) {
            break;
        }
    }
    return index;
}
unsafe extern "C" fn _cairo_hull_eliminate_concave(
    mut hull: *mut cairo_hull_t,
    mut num_hull: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut slope_ij: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    let mut slope_jk: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
    i = 0 as libc::c_int;
    j = _cairo_hull_next_valid(hull, num_hull, i);
    k = _cairo_hull_next_valid(hull, num_hull, j);
    loop {
        _cairo_slope_init(
            &mut slope_ij,
            &mut (*hull.offset(i as isize)).point,
            &mut (*hull.offset(j as isize)).point,
        );
        _cairo_slope_init(
            &mut slope_jk,
            &mut (*hull.offset(j as isize)).point,
            &mut (*hull.offset(k as isize)).point,
        );
        if _cairo_slope_compare(&mut slope_ij, &mut slope_jk) >= 0 as libc::c_int {
            if i == k {
                return;
            }
            (*hull.offset(j as isize)).discard = 1 as libc::c_int;
            j = i;
            i = _cairo_hull_prev_valid(hull, num_hull, j);
        } else {
            i = j;
            j = k;
            k = _cairo_hull_next_valid(hull, num_hull, j);
        }
        if !(j != 0 as libc::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn _cairo_hull_to_pen(
    mut hull: *mut cairo_hull_t,
    mut vertices: *mut cairo_pen_vertex_t,
    mut num_vertices: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < *num_vertices {
        if !((*hull.offset(i as isize)).discard != 0) {
            let fresh2 = j;
            j = j + 1;
            (*vertices.offset(fresh2 as isize)).point = (*hull.offset(i as isize)).point;
        }
        i += 1;
    }
    *num_vertices = j;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_hull_compute(
    mut vertices: *mut cairo_pen_vertex_t,
    mut num_vertices: *mut libc::c_int,
) -> cairo_status_t {
    let mut hull_stack: [cairo_hull_t; 85] = [cairo_hull_t {
        point: cairo_point_t { x: 0, y: 0 },
        slope: cairo_slope_t { dx: 0, dy: 0 },
        discard: 0,
        id: 0,
    }; 85];
    let mut hull: *mut cairo_hull_t = 0 as *mut cairo_hull_t;
    let mut num_hull: libc::c_int = *num_vertices;
    if num_hull
        > (::std::mem::size_of::<[cairo_hull_t; 85]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_hull_t>() as libc::c_ulong)
            as libc::c_int
    {
        hull = _cairo_malloc_ab(
            num_hull as size_t,
            ::std::mem::size_of::<cairo_hull_t>() as libc::c_ulong,
        ) as *mut cairo_hull_t;
        if hull.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    } else {
        hull = hull_stack.as_mut_ptr();
    }
    _cairo_hull_init(hull, vertices, num_hull);
    qsort(
        hull.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (num_hull - 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<cairo_hull_t>() as libc::c_ulong,
        Some(
            _cairo_hull_vertex_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    _cairo_hull_eliminate_concave(hull, num_hull);
    _cairo_hull_to_pen(hull, vertices, num_vertices);
    if hull != hull_stack.as_mut_ptr() {
        free(hull as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
