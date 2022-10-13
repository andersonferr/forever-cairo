use ::libc;
extern "C" {
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn _cairo_color_double_to_short(d: libc::c_double) -> uint16_t;
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_transform_point(
        matrix: *const cairo_matrix_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_array_index_const(
        array: *const cairo_array_t,
        index: libc::c_uint,
    ) -> *const libc::c_void;
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
}
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type ptrdiff_t = libc::c_long;
pub type cairo_bool_t = libc::c_int;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
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
pub type cairo_fixed_t = int32_t;
pub type uint32_t = __uint32_t;
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
pub type uint16_t = __uint16_t;
pub type cairo_fixed_16_16_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_patch {
    pub points: [[cairo_point_double_t; 4]; 4],
    pub colors: [cairo_color_t; 4],
}
pub type cairo_mesh_patch_t = _cairo_mesh_patch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_pattern {
    pub base: cairo_pattern_t,
    pub patches: cairo_array_t,
    pub current_patch: *mut cairo_mesh_patch_t,
    pub current_side: libc::c_int,
    pub has_control_point: [cairo_bool_t; 4],
    pub has_color: [cairo_bool_t; 4],
}
pub type cairo_mesh_pattern_t = _cairo_mesh_pattern;
pub const INSIDE: _intersection = -1;
pub const OUTSIDE: _intersection = 0;
pub const PARTIAL: _intersection = 1;
pub type _intersection = libc::c_int;
#[inline]
unsafe extern "C" fn _cairo_fixed_16_16_from_double(
    mut d: libc::c_double,
) -> cairo_fixed_16_16_t {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { d: 0. };
    u.d = d + 103079215104.0f64;
    return u.i[0 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
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
unsafe extern "C" fn sqlen(
    mut p0: cairo_point_double_t,
    mut p1: cairo_point_double_t,
) -> libc::c_double {
    let mut delta: cairo_point_double_t = cairo_point_double_t {
        x: 0.,
        y: 0.,
    };
    delta.x = p0.x - p1.x;
    delta.y = p0.y - p1.y;
    return delta.x * delta.x + delta.y * delta.y;
}
#[inline]
unsafe extern "C" fn _color_delta_to_shifted_short(
    mut from: int32_t,
    mut to: int32_t,
    mut shift: libc::c_int,
) -> int16_t {
    let mut delta: int32_t = to - from;
    if delta >= 0 as libc::c_int {
        return (delta >> shift) as int16_t
    } else {
        return -(-delta >> shift) as int16_t
    };
}
#[inline]
unsafe extern "C" fn sqsteps2shift(mut steps_sq: libc::c_double) -> libc::c_int {
    let mut r: libc::c_int = 0;
    frexp(if 1.0f64 > steps_sq { 1.0f64 } else { steps_sq }, &mut r);
    return r + 1 as libc::c_int >> 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn fd_init(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut w: libc::c_double,
    mut f: *mut libc::c_double,
) {
    *f.offset(0 as libc::c_int as isize) = x;
    *f.offset(1 as libc::c_int as isize) = w - x;
    *f.offset(2 as libc::c_int as isize) = 6.0f64 * (w - 2.0f64 * z + y);
    *f.offset(3 as libc::c_int as isize) = 6.0f64 * (w - 3.0f64 * z + 3.0f64 * y - x);
}
#[inline]
unsafe extern "C" fn fd_down(mut f: *mut libc::c_double) {
    *f.offset(3 as libc::c_int as isize) *= 0.125f64;
    *f
        .offset(
            2 as libc::c_int as isize,
        ) = *f.offset(2 as libc::c_int as isize) * 0.25f64
        - *f.offset(3 as libc::c_int as isize);
    *f
        .offset(
            1 as libc::c_int as isize,
        ) = (*f.offset(1 as libc::c_int as isize) - *f.offset(2 as libc::c_int as isize))
        * 0.5f64;
}
#[inline]
unsafe extern "C" fn fd_fwd(mut f: *mut libc::c_double) {
    *f.offset(0 as libc::c_int as isize) += *f.offset(1 as libc::c_int as isize);
    *f.offset(1 as libc::c_int as isize) += *f.offset(2 as libc::c_int as isize);
    *f.offset(2 as libc::c_int as isize) += *f.offset(3 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn fd_fixed(mut d: *mut libc::c_double, mut i: *mut int32_t) {
    *i
        .offset(
            0 as libc::c_int as isize,
        ) = _cairo_fixed_16_16_from_double(
        (256 as libc::c_int * 2 as libc::c_int) as libc::c_double
            * *d.offset(0 as libc::c_int as isize),
    );
    *i
        .offset(
            1 as libc::c_int as isize,
        ) = _cairo_fixed_16_16_from_double(
        (256 as libc::c_int * 16 as libc::c_int) as libc::c_double
            * *d.offset(1 as libc::c_int as isize),
    );
    *i
        .offset(
            2 as libc::c_int as isize,
        ) = _cairo_fixed_16_16_from_double(
        (256 as libc::c_int * 16 as libc::c_int) as libc::c_double
            * *d.offset(2 as libc::c_int as isize),
    );
    *i
        .offset(
            3 as libc::c_int as isize,
        ) = _cairo_fixed_16_16_from_double(
        (256 as libc::c_int * 16 as libc::c_int) as libc::c_double
            * *d.offset(3 as libc::c_int as isize),
    );
}
#[inline]
unsafe extern "C" fn fd_fixed_fwd(mut f: *mut int32_t) {
    let ref mut fresh0 = *f.offset(0 as libc::c_int as isize);
    *fresh0
        += (*f.offset(1 as libc::c_int as isize) >> 5 as libc::c_int)
            + (*f.offset(1 as libc::c_int as isize) >> 4 as libc::c_int
                & 1 as libc::c_int);
    let ref mut fresh1 = *f.offset(1 as libc::c_int as isize);
    *fresh1 += *f.offset(2 as libc::c_int as isize);
    let ref mut fresh2 = *f.offset(2 as libc::c_int as isize);
    *fresh2 += *f.offset(3 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn bezier_steps_sq(
    mut p: *mut cairo_point_double_t,
) -> libc::c_double {
    let mut tmp: libc::c_double = sqlen(
        *p.offset(0 as libc::c_int as isize),
        *p.offset(1 as libc::c_int as isize),
    );
    tmp = if tmp
        > sqlen(
            *p.offset(2 as libc::c_int as isize),
            *p.offset(3 as libc::c_int as isize),
        )
    {
        tmp
    } else {
        sqlen(*p.offset(2 as libc::c_int as isize), *p.offset(3 as libc::c_int as isize))
    };
    tmp = if tmp
        > sqlen(
            *p.offset(0 as libc::c_int as isize),
            *p.offset(2 as libc::c_int as isize),
        ) * 0.25f64
    {
        tmp
    } else {
        sqlen(*p.offset(0 as libc::c_int as isize), *p.offset(2 as libc::c_int as isize))
            * 0.25f64
    };
    tmp = if tmp
        > sqlen(
            *p.offset(1 as libc::c_int as isize),
            *p.offset(3 as libc::c_int as isize),
        ) * 0.25f64
    {
        tmp
    } else {
        sqlen(*p.offset(1 as libc::c_int as isize), *p.offset(3 as libc::c_int as isize))
            * 0.25f64
    };
    return 18.0f64 * tmp;
}
#[inline]
unsafe extern "C" fn split_bezier_1D(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut w: libc::c_double,
    mut x0: *mut libc::c_double,
    mut y0: *mut libc::c_double,
    mut z0: *mut libc::c_double,
    mut w0: *mut libc::c_double,
    mut x1: *mut libc::c_double,
    mut y1: *mut libc::c_double,
    mut z1: *mut libc::c_double,
    mut w1: *mut libc::c_double,
) {
    let mut tmp: libc::c_double = 0.;
    *x0 = x;
    *w1 = w;
    tmp = 0.5f64 * (y + z);
    *y0 = 0.5f64 * (x + y);
    *z1 = 0.5f64 * (z + w);
    *z0 = 0.5f64 * (*y0 + tmp);
    *y1 = 0.5f64 * (tmp + *z1);
    *x1 = 0.5f64 * (*z0 + *y1);
    *w0 = *x1;
}
unsafe extern "C" fn split_bezier(
    mut p: *mut cairo_point_double_t,
    mut fst_half: *mut cairo_point_double_t,
    mut snd_half: *mut cairo_point_double_t,
) {
    split_bezier_1D(
        (*p.offset(0 as libc::c_int as isize)).x,
        (*p.offset(1 as libc::c_int as isize)).x,
        (*p.offset(2 as libc::c_int as isize)).x,
        (*p.offset(3 as libc::c_int as isize)).x,
        &mut (*fst_half.offset(0 as libc::c_int as isize)).x,
        &mut (*fst_half.offset(1 as libc::c_int as isize)).x,
        &mut (*fst_half.offset(2 as libc::c_int as isize)).x,
        &mut (*fst_half.offset(3 as libc::c_int as isize)).x,
        &mut (*snd_half.offset(0 as libc::c_int as isize)).x,
        &mut (*snd_half.offset(1 as libc::c_int as isize)).x,
        &mut (*snd_half.offset(2 as libc::c_int as isize)).x,
        &mut (*snd_half.offset(3 as libc::c_int as isize)).x,
    );
    split_bezier_1D(
        (*p.offset(0 as libc::c_int as isize)).y,
        (*p.offset(1 as libc::c_int as isize)).y,
        (*p.offset(2 as libc::c_int as isize)).y,
        (*p.offset(3 as libc::c_int as isize)).y,
        &mut (*fst_half.offset(0 as libc::c_int as isize)).y,
        &mut (*fst_half.offset(1 as libc::c_int as isize)).y,
        &mut (*fst_half.offset(2 as libc::c_int as isize)).y,
        &mut (*fst_half.offset(3 as libc::c_int as isize)).y,
        &mut (*snd_half.offset(0 as libc::c_int as isize)).y,
        &mut (*snd_half.offset(1 as libc::c_int as isize)).y,
        &mut (*snd_half.offset(2 as libc::c_int as isize)).y,
        &mut (*snd_half.offset(3 as libc::c_int as isize)).y,
    );
}
#[inline]
unsafe extern "C" fn intersect_interval(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut d: libc::c_double,
) -> libc::c_int {
    if c <= a && b <= d {
        return INSIDE as libc::c_int
    } else if a >= d || b <= c {
        return OUTSIDE as libc::c_int
    } else {
        return PARTIAL as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn draw_pixel(
    mut data: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut r: uint16_t,
    mut g: uint16_t,
    mut b: uint16_t,
    mut a: uint16_t,
) {
    if 0 as libc::c_int <= x && 0 as libc::c_int <= y && x < width && y < height {
        let mut tr: uint32_t = 0;
        let mut tg: uint32_t = 0;
        let mut tb: uint32_t = 0;
        let mut ta: uint32_t = 0;
        ta = a as uint32_t;
        tr = (r as libc::c_uint)
            .wrapping_mul(ta)
            .wrapping_add(0x8000 as libc::c_int as libc::c_uint);
        tg = (g as libc::c_uint)
            .wrapping_mul(ta)
            .wrapping_add(0x8000 as libc::c_int as libc::c_uint);
        tb = (b as libc::c_uint)
            .wrapping_mul(ta)
            .wrapping_add(0x8000 as libc::c_int as libc::c_uint);
        tr = (tr as libc::c_uint).wrapping_add(tr >> 16 as libc::c_int) as uint32_t
            as uint32_t;
        tg = (tg as libc::c_uint).wrapping_add(tg >> 16 as libc::c_int) as uint32_t
            as uint32_t;
        tb = (tb as libc::c_uint).wrapping_add(tb >> 16 as libc::c_int) as uint32_t
            as uint32_t;
        *(data
            .offset((y as libc::c_long * stride as ptrdiff_t) as isize)
            .offset((4 as libc::c_int * x) as isize)
            as *mut uint32_t) = ta << 16 as libc::c_int & 0xff000000 as libc::c_uint
            | tr >> 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
            | tg >> 16 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
            | tb >> 24 as libc::c_int;
    }
}
#[inline]
unsafe extern "C" fn rasterize_bezier_curve(
    mut data: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut ushift: libc::c_int,
    mut dxu: *mut libc::c_double,
    mut dyu: *mut libc::c_double,
    mut r0: uint16_t,
    mut g0: uint16_t,
    mut b0: uint16_t,
    mut a0: uint16_t,
    mut r3: uint16_t,
    mut g3: uint16_t,
    mut b3: uint16_t,
    mut a3: uint16_t,
) {
    let mut xu: [int32_t; 4] = [0; 4];
    let mut yu: [int32_t; 4] = [0; 4];
    let mut x0: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut usteps: libc::c_int = (1 as libc::c_int) << ushift;
    let mut r: uint16_t = r0;
    let mut g: uint16_t = g0;
    let mut b: uint16_t = b0;
    let mut a: uint16_t = a0;
    let mut dr: int16_t = _color_delta_to_shifted_short(
        r0 as int32_t,
        r3 as int32_t,
        ushift,
    );
    let mut dg: int16_t = _color_delta_to_shifted_short(
        g0 as int32_t,
        g3 as int32_t,
        ushift,
    );
    let mut db: int16_t = _color_delta_to_shifted_short(
        b0 as int32_t,
        b3 as int32_t,
        ushift,
    );
    let mut da: int16_t = _color_delta_to_shifted_short(
        a0 as int32_t,
        a3 as int32_t,
        ushift,
    );
    fd_fixed(dxu, xu.as_mut_ptr());
    fd_fixed(dyu, yu.as_mut_ptr());
    x0 = _cairo_fixed_from_double(*dxu.offset(0 as libc::c_int as isize));
    y0 = _cairo_fixed_from_double(*dyu.offset(0 as libc::c_int as isize));
    xu[0 as libc::c_int as usize] = 0 as libc::c_int;
    yu[0 as libc::c_int as usize] = 0 as libc::c_int;
    u = 0 as libc::c_int;
    while u <= usteps {
        let mut x: libc::c_int = _cairo_fixed_integer_floor(
            x0 + (xu[0 as libc::c_int as usize] >> 15 as libc::c_int)
                + (xu[0 as libc::c_int as usize] >> 14 as libc::c_int & 1 as libc::c_int),
        );
        let mut y: libc::c_int = _cairo_fixed_integer_floor(
            y0 + (yu[0 as libc::c_int as usize] >> 15 as libc::c_int)
                + (yu[0 as libc::c_int as usize] >> 14 as libc::c_int & 1 as libc::c_int),
        );
        draw_pixel(data, width, height, stride, x, y, r, g, b, a);
        fd_fixed_fwd(xu.as_mut_ptr());
        fd_fixed_fwd(yu.as_mut_ptr());
        r = (r as libc::c_int + dr as libc::c_int) as uint16_t;
        g = (g as libc::c_int + dg as libc::c_int) as uint16_t;
        b = (b as libc::c_int + db as libc::c_int) as uint16_t;
        a = (a as libc::c_int + da as libc::c_int) as uint16_t;
        u += 1;
    }
}
unsafe extern "C" fn draw_bezier_curve(
    mut data: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut p: *mut cairo_point_double_t,
    mut c0: *mut libc::c_double,
    mut c3: *mut libc::c_double,
) {
    let mut top: libc::c_double = 0.;
    let mut bottom: libc::c_double = 0.;
    let mut left: libc::c_double = 0.;
    let mut right: libc::c_double = 0.;
    let mut steps_sq: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    bottom = (*p.offset(0 as libc::c_int as isize)).y;
    top = bottom;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        top = if top < (*p.offset(i as isize)).y {
            top
        } else {
            (*p.offset(i as isize)).y
        };
        bottom = if bottom > (*p.offset(i as isize)).y {
            bottom
        } else {
            (*p.offset(i as isize)).y
        };
        i += 1;
    }
    v = intersect_interval(
        top,
        bottom,
        0 as libc::c_int as libc::c_double,
        height as libc::c_double,
    );
    if v == OUTSIDE as libc::c_int {
        return;
    }
    right = (*p.offset(0 as libc::c_int as isize)).x;
    left = right;
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        left = if left < (*p.offset(i as isize)).x {
            left
        } else {
            (*p.offset(i as isize)).x
        };
        right = if right > (*p.offset(i as isize)).x {
            right
        } else {
            (*p.offset(i as isize)).x
        };
        i += 1;
    }
    v
        &= intersect_interval(
            left,
            right,
            0 as libc::c_int as libc::c_double,
            width as libc::c_double,
        );
    if v == OUTSIDE as libc::c_int {
        return;
    }
    steps_sq = bezier_steps_sq(p);
    if steps_sq
        >= (if v == INSIDE as libc::c_int {
            256.0f64 * 256.0f64
        } else {
            64.0f64 * 64.0f64
        })
    {
        let mut first: [cairo_point_double_t; 4] = [cairo_point_double_t {
            x: 0.,
            y: 0.,
        }; 4];
        let mut second: [cairo_point_double_t; 4] = [cairo_point_double_t {
            x: 0.,
            y: 0.,
        }; 4];
        let mut midc: [libc::c_double; 4] = [0.; 4];
        split_bezier(p, first.as_mut_ptr(), second.as_mut_ptr());
        midc[0 as libc::c_int
            as usize] = (*c0.offset(0 as libc::c_int as isize)
            + *c3.offset(0 as libc::c_int as isize)) * 0.5f64;
        midc[1 as libc::c_int
            as usize] = (*c0.offset(1 as libc::c_int as isize)
            + *c3.offset(1 as libc::c_int as isize)) * 0.5f64;
        midc[2 as libc::c_int
            as usize] = (*c0.offset(2 as libc::c_int as isize)
            + *c3.offset(2 as libc::c_int as isize)) * 0.5f64;
        midc[3 as libc::c_int
            as usize] = (*c0.offset(3 as libc::c_int as isize)
            + *c3.offset(3 as libc::c_int as isize)) * 0.5f64;
        draw_bezier_curve(
            data,
            width,
            height,
            stride,
            first.as_mut_ptr(),
            c0,
            midc.as_mut_ptr(),
        );
        draw_bezier_curve(
            data,
            width,
            height,
            stride,
            second.as_mut_ptr(),
            midc.as_mut_ptr(),
            c3,
        );
    } else {
        let mut xu: [libc::c_double; 4] = [0.; 4];
        let mut yu: [libc::c_double; 4] = [0.; 4];
        let mut ushift: libc::c_int = sqsteps2shift(steps_sq);
        let mut k: libc::c_int = 0;
        fd_init(
            (*p.offset(0 as libc::c_int as isize)).x,
            (*p.offset(1 as libc::c_int as isize)).x,
            (*p.offset(2 as libc::c_int as isize)).x,
            (*p.offset(3 as libc::c_int as isize)).x,
            xu.as_mut_ptr(),
        );
        fd_init(
            (*p.offset(0 as libc::c_int as isize)).y,
            (*p.offset(1 as libc::c_int as isize)).y,
            (*p.offset(2 as libc::c_int as isize)).y,
            (*p.offset(3 as libc::c_int as isize)).y,
            yu.as_mut_ptr(),
        );
        k = 0 as libc::c_int;
        while k < ushift {
            fd_down(xu.as_mut_ptr());
            fd_down(yu.as_mut_ptr());
            k += 1;
        }
        rasterize_bezier_curve(
            data,
            width,
            height,
            stride,
            ushift,
            xu.as_mut_ptr(),
            yu.as_mut_ptr(),
            _cairo_color_double_to_short(*c0.offset(0 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c0.offset(1 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c0.offset(2 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c0.offset(3 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(0 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(1 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(2 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(3 as libc::c_int as isize)),
        );
        draw_pixel(
            data,
            width,
            height,
            stride,
            _cairo_fixed_integer_floor(
                _cairo_fixed_from_double((*p.offset(3 as libc::c_int as isize)).x),
            ),
            _cairo_fixed_integer_floor(
                _cairo_fixed_from_double((*p.offset(3 as libc::c_int as isize)).y),
            ),
            _cairo_color_double_to_short(*c3.offset(0 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(1 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(2 as libc::c_int as isize)),
            _cairo_color_double_to_short(*c3.offset(3 as libc::c_int as isize)),
        );
    };
}
#[inline]
unsafe extern "C" fn rasterize_bezier_patch(
    mut data: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut vshift: libc::c_int,
    mut p: *mut [cairo_point_double_t; 4],
    mut col: *mut [libc::c_double; 4],
) {
    let mut pv: [[[libc::c_double; 4]; 2]; 4] = [[[0.; 4]; 2]; 4];
    let mut cstart: [libc::c_double; 4] = [0.; 4];
    let mut cend: [libc::c_double; 4] = [0.; 4];
    let mut dcstart: [libc::c_double; 4] = [0.; 4];
    let mut dcend: [libc::c_double; 4] = [0.; 4];
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    v = (1 as libc::c_int) << vshift;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        fd_init(
            (*p.offset(i as isize))[0 as libc::c_int as usize].x,
            (*p.offset(i as isize))[1 as libc::c_int as usize].x,
            (*p.offset(i as isize))[2 as libc::c_int as usize].x,
            (*p.offset(i as isize))[3 as libc::c_int as usize].x,
            (pv[i as usize][0 as libc::c_int as usize]).as_mut_ptr(),
        );
        fd_init(
            (*p.offset(i as isize))[0 as libc::c_int as usize].y,
            (*p.offset(i as isize))[1 as libc::c_int as usize].y,
            (*p.offset(i as isize))[2 as libc::c_int as usize].y,
            (*p.offset(i as isize))[3 as libc::c_int as usize].y,
            (pv[i as usize][1 as libc::c_int as usize]).as_mut_ptr(),
        );
        k = 0 as libc::c_int;
        while k < vshift {
            fd_down((pv[i as usize][0 as libc::c_int as usize]).as_mut_ptr());
            fd_down((pv[i as usize][1 as libc::c_int as usize]).as_mut_ptr());
            k += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        cstart[i as usize] = (*col.offset(0 as libc::c_int as isize))[i as usize];
        cend[i as usize] = (*col.offset(1 as libc::c_int as isize))[i as usize];
        dcstart[i
            as usize] = ((*col.offset(2 as libc::c_int as isize))[i as usize]
            - (*col.offset(0 as libc::c_int as isize))[i as usize])
            / v as libc::c_double;
        dcend[i
            as usize] = ((*col.offset(3 as libc::c_int as isize))[i as usize]
            - (*col.offset(1 as libc::c_int as isize))[i as usize])
            / v as libc::c_double;
        i += 1;
    }
    v += 1;
    loop {
        let fresh3 = v;
        v = v - 1;
        if !(fresh3 != 0) {
            break;
        }
        let mut nodes: [cairo_point_double_t; 4] = [cairo_point_double_t {
            x: 0.,
            y: 0.,
        }; 4];
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            nodes[i as usize]
                .x = pv[i
                as usize][0 as libc::c_int as usize][0 as libc::c_int as usize];
            nodes[i as usize]
                .y = pv[i
                as usize][1 as libc::c_int as usize][0 as libc::c_int as usize];
            i += 1;
        }
        draw_bezier_curve(
            data,
            width,
            height,
            stride,
            nodes.as_mut_ptr(),
            cstart.as_mut_ptr(),
            cend.as_mut_ptr(),
        );
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            fd_fwd((pv[i as usize][0 as libc::c_int as usize]).as_mut_ptr());
            fd_fwd((pv[i as usize][1 as libc::c_int as usize]).as_mut_ptr());
            cstart[i as usize] += dcstart[i as usize];
            cend[i as usize] += dcend[i as usize];
            i += 1;
        }
    };
}
unsafe extern "C" fn draw_bezier_patch(
    mut data: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut p: *mut [cairo_point_double_t; 4],
    mut c: *mut [libc::c_double; 4],
) {
    let mut top: libc::c_double = 0.;
    let mut bottom: libc::c_double = 0.;
    let mut left: libc::c_double = 0.;
    let mut right: libc::c_double = 0.;
    let mut steps_sq: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    bottom = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize].y;
    top = bottom;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            top = if top < (*p.offset(i as isize))[j as usize].y {
                top
            } else {
                (*p.offset(i as isize))[j as usize].y
            };
            bottom = if bottom > (*p.offset(i as isize))[j as usize].y {
                bottom
            } else {
                (*p.offset(i as isize))[j as usize].y
            };
            j += 1;
        }
        i += 1;
    }
    v = intersect_interval(
        top,
        bottom,
        0 as libc::c_int as libc::c_double,
        height as libc::c_double,
    );
    if v == OUTSIDE as libc::c_int {
        return;
    }
    right = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize].x;
    left = right;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            left = if left < (*p.offset(i as isize))[j as usize].x {
                left
            } else {
                (*p.offset(i as isize))[j as usize].x
            };
            right = if right > (*p.offset(i as isize))[j as usize].x {
                right
            } else {
                (*p.offset(i as isize))[j as usize].x
            };
            j += 1;
        }
        i += 1;
    }
    v
        &= intersect_interval(
            left,
            right,
            0 as libc::c_int as libc::c_double,
            width as libc::c_double,
        );
    if v == OUTSIDE as libc::c_int {
        return;
    }
    steps_sq = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        steps_sq = if steps_sq > bezier_steps_sq((*p.offset(i as isize)).as_mut_ptr()) {
            steps_sq
        } else {
            bezier_steps_sq((*p.offset(i as isize)).as_mut_ptr())
        };
        i += 1;
    }
    if steps_sq
        >= (if v == INSIDE as libc::c_int {
            256.0f64 * 256.0f64
        } else {
            64.0f64 * 64.0f64
        })
    {
        let mut first: [[cairo_point_double_t; 4]; 4] = [[cairo_point_double_t {
            x: 0.,
            y: 0.,
        }; 4]; 4];
        let mut second: [[cairo_point_double_t; 4]; 4] = [[cairo_point_double_t {
            x: 0.,
            y: 0.,
        }; 4]; 4];
        let mut subc: [[libc::c_double; 4]; 4] = [[0.; 4]; 4];
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            split_bezier(
                (*p.offset(i as isize)).as_mut_ptr(),
                (first[i as usize]).as_mut_ptr(),
                (second[i as usize]).as_mut_ptr(),
            );
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            subc[0 as libc::c_int
                as usize][i
                as usize] = (*c.offset(0 as libc::c_int as isize))[i as usize];
            subc[1 as libc::c_int
                as usize][i
                as usize] = (*c.offset(1 as libc::c_int as isize))[i as usize];
            subc[2 as libc::c_int
                as usize][i
                as usize] = 0.5f64
                * ((*c.offset(0 as libc::c_int as isize))[i as usize]
                    + (*c.offset(2 as libc::c_int as isize))[i as usize]);
            subc[3 as libc::c_int
                as usize][i
                as usize] = 0.5f64
                * ((*c.offset(1 as libc::c_int as isize))[i as usize]
                    + (*c.offset(3 as libc::c_int as isize))[i as usize]);
            i += 1;
        }
        draw_bezier_patch(
            data,
            width,
            height,
            stride,
            first.as_mut_ptr(),
            subc.as_mut_ptr(),
        );
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            subc[0 as libc::c_int
                as usize][i as usize] = subc[2 as libc::c_int as usize][i as usize];
            subc[1 as libc::c_int
                as usize][i as usize] = subc[3 as libc::c_int as usize][i as usize];
            subc[2 as libc::c_int
                as usize][i
                as usize] = (*c.offset(2 as libc::c_int as isize))[i as usize];
            subc[3 as libc::c_int
                as usize][i
                as usize] = (*c.offset(3 as libc::c_int as isize))[i as usize];
            i += 1;
        }
        draw_bezier_patch(
            data,
            width,
            height,
            stride,
            second.as_mut_ptr(),
            subc.as_mut_ptr(),
        );
    } else {
        rasterize_bezier_patch(
            data,
            width,
            height,
            stride,
            sqsteps2shift(steps_sq),
            p,
            c,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mesh_pattern_rasterize(
    mut mesh: *const cairo_mesh_pattern_t,
    mut data: *mut libc::c_void,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut x_offset: libc::c_double,
    mut y_offset: libc::c_double,
) {
    let mut nodes: [[cairo_point_double_t; 4]; 4] = [[cairo_point_double_t {
        x: 0.,
        y: 0.,
    }; 4]; 4];
    let mut colors: [[libc::c_double; 4]; 4] = [[0.; 4]; 4];
    let mut p2u: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut patch: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut c: *const cairo_color_t = 0 as *const cairo_color_t;
    if (*mesh).base.status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"mesh->base.status == CAIRO_STATUS_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-mesh-pattern-rasterizer.c\0" as *const u8
                as *const libc::c_char,
            895 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"void _cairo_mesh_pattern_rasterize(const cairo_mesh_pattern_t *, void *, int, int, int, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*mesh).current_patch).is_null() {} else {
        __assert_fail(
            b"mesh->current_patch == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mesh-pattern-rasterizer.c\0" as *const u8
                as *const libc::c_char,
            896 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"void _cairo_mesh_pattern_rasterize(const cairo_mesh_pattern_t *, void *, int, int, int, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    p2u = (*mesh).base.matrix;
    status = cairo_matrix_invert(&mut p2u);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mesh-pattern-rasterizer.c\0" as *const u8
                as *const libc::c_char,
            900 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"void _cairo_mesh_pattern_rasterize(const cairo_mesh_pattern_t *, void *, int, int, int, double, double)\0",
            ))
                .as_ptr(),
        );
    }
    n = _cairo_array_num_elements(&(*mesh).patches);
    patch = _cairo_array_index_const(&(*mesh).patches, 0 as libc::c_int as libc::c_uint)
        as *const cairo_mesh_patch_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        j = 0 as libc::c_int as libc::c_uint;
        while j < 4 as libc::c_int as libc::c_uint {
            k = 0 as libc::c_int as libc::c_uint;
            while k < 4 as libc::c_int as libc::c_uint {
                nodes[j as usize][k as usize] = (*patch).points[j as usize][k as usize];
                cairo_matrix_transform_point(
                    &mut p2u,
                    &mut (*(*nodes.as_mut_ptr().offset(j as isize))
                        .as_mut_ptr()
                        .offset(k as isize))
                        .x,
                    &mut (*(*nodes.as_mut_ptr().offset(j as isize))
                        .as_mut_ptr()
                        .offset(k as isize))
                        .y,
                );
                nodes[j as usize][k as usize].x += x_offset;
                nodes[j as usize][k as usize].y += y_offset;
                k = k.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        c = &*((*patch).colors).as_ptr().offset(0 as libc::c_int as isize)
            as *const cairo_color_t;
        colors[0 as libc::c_int as usize][0 as libc::c_int as usize] = (*c).red;
        colors[0 as libc::c_int as usize][1 as libc::c_int as usize] = (*c).green;
        colors[0 as libc::c_int as usize][2 as libc::c_int as usize] = (*c).blue;
        colors[0 as libc::c_int as usize][3 as libc::c_int as usize] = (*c).alpha;
        c = &*((*patch).colors).as_ptr().offset(3 as libc::c_int as isize)
            as *const cairo_color_t;
        colors[1 as libc::c_int as usize][0 as libc::c_int as usize] = (*c).red;
        colors[1 as libc::c_int as usize][1 as libc::c_int as usize] = (*c).green;
        colors[1 as libc::c_int as usize][2 as libc::c_int as usize] = (*c).blue;
        colors[1 as libc::c_int as usize][3 as libc::c_int as usize] = (*c).alpha;
        c = &*((*patch).colors).as_ptr().offset(1 as libc::c_int as isize)
            as *const cairo_color_t;
        colors[2 as libc::c_int as usize][0 as libc::c_int as usize] = (*c).red;
        colors[2 as libc::c_int as usize][1 as libc::c_int as usize] = (*c).green;
        colors[2 as libc::c_int as usize][2 as libc::c_int as usize] = (*c).blue;
        colors[2 as libc::c_int as usize][3 as libc::c_int as usize] = (*c).alpha;
        c = &*((*patch).colors).as_ptr().offset(2 as libc::c_int as isize)
            as *const cairo_color_t;
        colors[3 as libc::c_int as usize][0 as libc::c_int as usize] = (*c).red;
        colors[3 as libc::c_int as usize][1 as libc::c_int as usize] = (*c).green;
        colors[3 as libc::c_int as usize][2 as libc::c_int as usize] = (*c).blue;
        colors[3 as libc::c_int as usize][3 as libc::c_int as usize] = (*c).alpha;
        draw_bezier_patch(
            data as *mut libc::c_uchar,
            width,
            height,
            stride,
            nodes.as_mut_ptr(),
            colors.as_mut_ptr(),
        );
        patch = patch.offset(1);
        i = i.wrapping_add(1);
    }
}
