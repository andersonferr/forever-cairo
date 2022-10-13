use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_region;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type cairo_bool_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tristrip {
    pub status: cairo_status_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_points: libc::c_int,
    pub size_points: libc::c_int,
    pub points: *mut cairo_point_t,
    pub points_embedded: [cairo_point_t; 64],
}
pub type cairo_tristrip_t = _cairo_tristrip;
pub type cairo_line_t = _cairo_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_trapezoid {
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub left: cairo_line_t,
    pub right: cairo_line_t,
}
pub type cairo_trapezoid_t = _cairo_trapezoid;
#[inline(always)]
unsafe extern "C" fn _cairo_realloc_ab(
    mut ptr: *mut libc::c_void,
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, c);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh2, fresh3) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_init(mut strip: *mut cairo_tristrip_t) {
    (*strip).status = CAIRO_STATUS_SUCCESS;
    (*strip).num_limits = 0 as libc::c_int;
    (*strip).num_points = 0 as libc::c_int;
    (*strip)
        .size_points = (::std::mem::size_of::<[cairo_point_t; 64]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong)
        as libc::c_int;
    let ref mut fresh4 = (*strip).points;
    *fresh4 = ((*strip).points_embedded).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_fini(mut strip: *mut cairo_tristrip_t) {
    if (*strip).points != ((*strip).points_embedded).as_mut_ptr() {
        free((*strip).points as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_limit(
    mut strip: *mut cairo_tristrip_t,
    mut limits: *const cairo_box_t,
    mut num_limits: libc::c_int,
) {
    let ref mut fresh5 = (*strip).limits;
    *fresh5 = limits;
    (*strip).num_limits = num_limits;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_init_with_clip(
    mut strip: *mut cairo_tristrip_t,
    mut clip: *const cairo_clip_t,
) {
    _cairo_tristrip_init(strip);
    if !clip.is_null() {
        _cairo_tristrip_limit(strip, (*clip).boxes, (*clip).num_boxes);
    }
}
unsafe extern "C" fn _cairo_tristrip_grow(
    mut strip: *mut cairo_tristrip_t,
) -> cairo_bool_t {
    let mut points: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut new_size: libc::c_int = 4 as libc::c_int * (*strip).size_points;
    if (*strip).points == ((*strip).points_embedded).as_mut_ptr() {
        points = _cairo_malloc_ab(
            new_size as size_t,
            ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
        ) as *mut cairo_point_t;
        if !points.is_null() {
            memcpy(
                points as *mut libc::c_void,
                (*strip).points as *const libc::c_void,
                ::std::mem::size_of::<[cairo_point_t; 64]>() as libc::c_ulong,
            );
        }
    } else {
        points = _cairo_realloc_ab(
            (*strip).points as *mut libc::c_void,
            new_size as size_t,
            ::std::mem::size_of::<cairo_trapezoid_t>() as libc::c_ulong,
        ) as *mut cairo_point_t;
    }
    if points.is_null() {
        (*strip).status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as libc::c_int;
    }
    let ref mut fresh6 = (*strip).points;
    *fresh6 = points;
    (*strip).size_points = new_size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_add_point(
    mut strip: *mut cairo_tristrip_t,
    mut p: *const cairo_point_t,
) {
    if (*strip).num_points == (*strip).size_points {
        if _cairo_tristrip_grow(strip) == 0 {
            return;
        }
    }
    let ref mut fresh7 = (*strip).num_points;
    let fresh8 = *fresh7;
    *fresh7 = *fresh7 + 1;
    *((*strip).points).offset(fresh8 as isize) = *p;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_move_to(
    mut strip: *mut cairo_tristrip_t,
    mut p: *const cairo_point_t,
) {
    if (*strip).num_points == 0 as libc::c_int {
        return;
    }
    _cairo_tristrip_add_point(
        strip,
        &mut *((*strip).points).offset(((*strip).num_points - 1 as libc::c_int) as isize),
    );
    _cairo_tristrip_add_point(strip, p);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_translate(
    mut strip: *mut cairo_tristrip_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut xoff: cairo_fixed_t = 0;
    let mut yoff: cairo_fixed_t = 0;
    let mut p: *mut cairo_point_t = 0 as *mut cairo_point_t;
    let mut i: libc::c_int = 0;
    xoff = _cairo_fixed_from_int(x);
    yoff = _cairo_fixed_from_int(y);
    i = 0 as libc::c_int;
    p = (*strip).points;
    while i < (*strip).num_points {
        let ref mut fresh9 = (*p).x;
        *fresh9 += xoff;
        let ref mut fresh10 = (*p).y;
        *fresh10 += yoff;
        i += 1;
        p = p.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tristrip_extents(
    mut strip: *const cairo_tristrip_t,
    mut extents: *mut cairo_box_t,
) {
    let mut i: libc::c_int = 0;
    if (*strip).num_points == 0 as libc::c_int {
        let ref mut fresh11 = (*extents).p1.y;
        *fresh11 = 0 as libc::c_int;
        (*extents).p1.x = *fresh11;
        let ref mut fresh12 = (*extents).p2.y;
        *fresh12 = 0 as libc::c_int;
        (*extents).p2.x = *fresh12;
        return;
    }
    let ref mut fresh13 = (*extents).p1;
    *fresh13 = *((*strip).points).offset(0 as libc::c_int as isize);
    (*extents).p2 = *fresh13;
    i = 1 as libc::c_int;
    while i < (*strip).num_points {
        let mut p: *const cairo_point_t = &mut *((*strip).points).offset(i as isize)
            as *mut cairo_point_t;
        if (*p).x < (*extents).p1.x {
            (*extents).p1.x = (*p).x;
        } else if (*p).x > (*extents).p2.x {
            (*extents).p2.x = (*p).x;
        }
        if (*p).y < (*extents).p1.y {
            (*extents).p1.y = (*p).y;
        } else if (*p).y > (*extents).p2.y {
            (*extents).p2.y = (*p).y;
        }
        i += 1;
    }
}
