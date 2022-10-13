use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_color_double_to_short(d: libc::c_double) -> uint16_t;
    fn _cairo_mesh_pattern_coord_box(
        mesh: *const cairo_mesh_pattern_t,
        out_xmin: *mut libc::c_double,
        out_ymin: *mut libc::c_double,
        out_xmax: *mut libc::c_double,
        out_ymax: *mut libc::c_double,
    ) -> cairo_bool_t;
    fn _cairo_array_index_const(
        array: *const cairo_array_t,
        index: libc::c_uint,
    ) -> *const libc::c_void;
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_shading {
    pub shading_type: libc::c_int,
    pub bits_per_coordinate: libc::c_int,
    pub bits_per_component: libc::c_int,
    pub bits_per_flag: libc::c_int,
    pub decode_array: *mut libc::c_double,
    pub decode_array_length: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub data_length: libc::c_ulong,
}
pub type cairo_pdf_shading_t = _cairo_pdf_shading;
#[inline]
unsafe extern "C" fn _cairo_restrict_value(
    mut value: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    if value < min {
        return min
    } else if value > max {
        return max
    } else {
        return value
    };
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
unsafe extern "C" fn encode_coordinate(
    mut p: *mut libc::c_uchar,
    mut c: libc::c_double,
) -> *mut libc::c_uchar {
    let mut f: uint32_t = 0;
    f = c as uint32_t;
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = (f >> 24 as libc::c_int) as libc::c_uchar;
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = (f >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = (f >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    let fresh5 = p;
    p = p.offset(1);
    *fresh5 = (f & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    return p;
}
unsafe extern "C" fn encode_point(
    mut p: *mut libc::c_uchar,
    mut point: *const cairo_point_double_t,
) -> *mut libc::c_uchar {
    p = encode_coordinate(p, (*point).x);
    p = encode_coordinate(p, (*point).y);
    return p;
}
unsafe extern "C" fn encode_color_component(
    mut p: *mut libc::c_uchar,
    mut color: libc::c_double,
) -> *mut libc::c_uchar {
    let mut c: uint16_t = 0;
    c = _cairo_color_double_to_short(color);
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = (c as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = (c as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    return p;
}
unsafe extern "C" fn encode_color(
    mut p: *mut libc::c_uchar,
    mut color: *const cairo_color_t,
) -> *mut libc::c_uchar {
    p = encode_color_component(p, (*color).red);
    p = encode_color_component(p, (*color).green);
    p = encode_color_component(p, (*color).blue);
    return p;
}
unsafe extern "C" fn encode_alpha(
    mut p: *mut libc::c_uchar,
    mut color: *const cairo_color_t,
) -> *mut libc::c_uchar {
    p = encode_color_component(p, (*color).alpha);
    return p;
}
unsafe extern "C" fn _cairo_pdf_shading_generate_decode_array(
    mut shading: *mut cairo_pdf_shading_t,
    mut mesh: *const cairo_mesh_pattern_t,
    mut is_alpha: cairo_bool_t,
) -> cairo_status_t {
    let mut num_color_components: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut is_valid: cairo_bool_t = 0;
    if is_alpha != 0 {
        num_color_components = 1 as libc::c_int as libc::c_uint;
    } else {
        num_color_components = 3 as libc::c_int as libc::c_uint;
    }
    (*shading)
        .decode_array_length = (4 as libc::c_int as libc::c_uint)
        .wrapping_add(
            num_color_components.wrapping_mul(2 as libc::c_int as libc::c_uint),
        ) as libc::c_int;
    let ref mut fresh8 = (*shading).decode_array;
    *fresh8 = _cairo_malloc_ab(
        (*shading).decode_array_length as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if ((*shading).decode_array).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    is_valid = _cairo_mesh_pattern_coord_box(
        mesh,
        &mut *((*shading).decode_array).offset(0 as libc::c_int as isize),
        &mut *((*shading).decode_array).offset(2 as libc::c_int as isize),
        &mut *((*shading).decode_array).offset(1 as libc::c_int as isize),
        &mut *((*shading).decode_array).offset(3 as libc::c_int as isize),
    );
    if is_valid != 0 {} else {
        __assert_fail(
            b"is_valid\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-shading.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"cairo_status_t _cairo_pdf_shading_generate_decode_array(cairo_pdf_shading_t *, const cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if *((*shading).decode_array).offset(1 as libc::c_int as isize)
        - *((*shading).decode_array).offset(0 as libc::c_int as isize)
        >= 2.2204460492503131e-16f64
    {} else {
        __assert_fail(
            b"shading->decode_array[1] - shading->decode_array[0] >= DBL_EPSILON\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-shading.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"cairo_status_t _cairo_pdf_shading_generate_decode_array(cairo_pdf_shading_t *, const cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if *((*shading).decode_array).offset(3 as libc::c_int as isize)
        - *((*shading).decode_array).offset(2 as libc::c_int as isize)
        >= 2.2204460492503131e-16f64
    {} else {
        __assert_fail(
            b"shading->decode_array[3] - shading->decode_array[2] >= DBL_EPSILON\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-shading.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"cairo_status_t _cairo_pdf_shading_generate_decode_array(cairo_pdf_shading_t *, const cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_color_components {
        *((*shading).decode_array)
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_add((2 as libc::c_int as libc::c_uint).wrapping_mul(i))
                    as isize,
            ) = 0 as libc::c_int as libc::c_double;
        *((*shading).decode_array)
            .offset(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_add((2 as libc::c_int as libc::c_uint).wrapping_mul(i))
                    as isize,
            ) = 1 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
    }
    return CAIRO_STATUS_SUCCESS;
}
static mut pdf_points_order_i: [libc::c_int; 16] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
];
static mut pdf_points_order_j: [libc::c_int; 16] = [
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
];
unsafe extern "C" fn _cairo_pdf_shading_generate_data(
    mut shading: *mut cairo_pdf_shading_t,
    mut mesh: *const cairo_mesh_pattern_t,
    mut is_alpha: cairo_bool_t,
) -> cairo_status_t {
    let mut patch: *const cairo_mesh_patch_t = 0 as *const cairo_mesh_patch_t;
    let mut x_off: libc::c_double = 0.;
    let mut y_off: libc::c_double = 0.;
    let mut x_scale: libc::c_double = 0.;
    let mut y_scale: libc::c_double = 0.;
    let mut num_patches: libc::c_uint = 0;
    let mut num_color_components: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    if is_alpha != 0 {
        num_color_components = 1 as libc::c_int as libc::c_uint;
    } else {
        num_color_components = 3 as libc::c_int as libc::c_uint;
    }
    num_patches = _cairo_array_num_elements(&(*mesh).patches);
    patch = _cairo_array_index_const(&(*mesh).patches, 0 as libc::c_int as libc::c_uint)
        as *const cairo_mesh_patch_t;
    (*shading)
        .data_length = num_patches
        .wrapping_mul(
            ((1 as libc::c_int + 16 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int)
                as libc::c_uint)
                .wrapping_add(
                    ((4 as libc::c_int * 2 as libc::c_int) as libc::c_uint)
                        .wrapping_mul(num_color_components),
                ),
        ) as libc::c_ulong;
    let ref mut fresh9 = (*shading).data;
    *fresh9 = (if (*shading).data_length != 0 as libc::c_int as libc::c_ulong {
        malloc((*shading).data_length)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if ((*shading).data).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    x_off = *((*shading).decode_array).offset(0 as libc::c_int as isize);
    y_off = *((*shading).decode_array).offset(2 as libc::c_int as isize);
    x_scale = 4294967295 as libc::c_uint as libc::c_double
        / (*((*shading).decode_array).offset(1 as libc::c_int as isize) - x_off);
    y_scale = 4294967295 as libc::c_uint as libc::c_double
        / (*((*shading).decode_array).offset(3 as libc::c_int as isize) - y_off);
    p = (*shading).data;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_patches {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = 0 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as libc::c_uint;
        while j < 16 as libc::c_int as libc::c_uint {
            let mut point: cairo_point_double_t = cairo_point_double_t {
                x: 0.,
                y: 0.,
            };
            let mut pi: libc::c_int = 0;
            let mut pj: libc::c_int = 0;
            pi = pdf_points_order_i[j as usize];
            pj = pdf_points_order_j[j as usize];
            point = (*patch.offset(i as isize)).points[pi as usize][pj as usize];
            point.x -= x_off;
            point.y -= y_off;
            point.x *= x_scale;
            point.y *= y_scale;
            point
                .x = _cairo_restrict_value(
                point.x,
                0 as libc::c_int as libc::c_double,
                4294967295 as libc::c_uint as libc::c_double,
            );
            point
                .y = _cairo_restrict_value(
                point.y,
                0 as libc::c_int as libc::c_double,
                4294967295 as libc::c_uint as libc::c_double,
            );
            p = encode_point(p, &mut point);
            j = j.wrapping_add(1);
        }
        j = 0 as libc::c_int as libc::c_uint;
        while j < 4 as libc::c_int as libc::c_uint {
            if is_alpha != 0 {
                p = encode_alpha(
                    p,
                    &*((*patch.offset(i as isize)).colors).as_ptr().offset(j as isize),
                );
            } else {
                p = encode_color(
                    p,
                    &*((*patch.offset(i as isize)).colors).as_ptr().offset(j as isize),
                );
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if p == ((*shading).data).offset((*shading).data_length as isize) {} else {
        __assert_fail(
            b"p == shading->data + shading->data_length\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-shading.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"cairo_status_t _cairo_pdf_shading_generate_data(cairo_pdf_shading_t *, const cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_pdf_shading_init(
    mut shading: *mut cairo_pdf_shading_t,
    mut mesh: *const cairo_mesh_pattern_t,
    mut is_alpha: cairo_bool_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*mesh).base.status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"mesh->base.status == CAIRO_STATUS_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-shading.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 106],
                &[libc::c_char; 106],
            >(
                b"cairo_status_t _cairo_pdf_shading_init(cairo_pdf_shading_t *, const cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*mesh).current_patch).is_null() {} else {
        __assert_fail(
            b"mesh->current_patch == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-shading.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 106],
                &[libc::c_char; 106],
            >(
                b"cairo_status_t _cairo_pdf_shading_init(cairo_pdf_shading_t *, const cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    (*shading).shading_type = 7 as libc::c_int;
    (*shading).bits_per_coordinate = 32 as libc::c_int;
    (*shading).bits_per_component = 16 as libc::c_int;
    (*shading).bits_per_flag = 8 as libc::c_int;
    let ref mut fresh11 = (*shading).decode_array;
    *fresh11 = 0 as *mut libc::c_double;
    let ref mut fresh12 = (*shading).data;
    *fresh12 = 0 as *mut libc::c_uchar;
    status = _cairo_pdf_shading_generate_decode_array(shading, mesh, is_alpha);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_pdf_shading_generate_data(shading, mesh, is_alpha);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_shading_init_color(
    mut shading: *mut cairo_pdf_shading_t,
    mut pattern: *const cairo_mesh_pattern_t,
) -> cairo_status_t {
    return _cairo_pdf_shading_init(shading, pattern, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_shading_init_alpha(
    mut shading: *mut cairo_pdf_shading_t,
    mut pattern: *const cairo_mesh_pattern_t,
) -> cairo_status_t {
    return _cairo_pdf_shading_init(shading, pattern, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_shading_fini(mut shading: *mut cairo_pdf_shading_t) {
    free((*shading).data as *mut libc::c_void);
    free((*shading).decode_array as *mut libc::c_void);
}
