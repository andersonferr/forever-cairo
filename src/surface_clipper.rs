use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_region;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    static __cairo_clip_all: cairo_clip_t;
    fn _cairo_clip_copy(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_clip_equal(
        clip_a: *const cairo_clip_t,
        clip_b: *const cairo_clip_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_init(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
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
    fn _cairo_path_fixed_close_path(path: *mut cairo_path_fixed_t) -> cairo_status_t;
}
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
pub struct _cairo_surface_clipper {
    pub clip: *mut cairo_clip_t,
    pub intersect_clip_path: cairo_surface_clipper_intersect_clip_path_func_t,
}
pub type cairo_surface_clipper_intersect_clip_path_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_surface_clipper_t,
        *mut cairo_path_fixed_t,
        cairo_fill_rule_t,
        libc::c_double,
        cairo_antialias_t,
    ) -> cairo_status_t,
>;
pub type cairo_surface_clipper_t = _cairo_surface_clipper;
#[inline]
unsafe extern "C" fn _cairo_clip_is_all_clipped(
    mut clip: *const cairo_clip_t,
) -> cairo_bool_t {
    return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
}
unsafe extern "C" fn _cairo_path_fixed_add_box(
    mut path: *mut cairo_path_fixed_t,
    mut box_0: *const cairo_box_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_path_fixed_move_to(path, (*box_0).p1.x, (*box_0).p1.y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_line_to(path, (*box_0).p2.x, (*box_0).p1.y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_line_to(path, (*box_0).p2.x, (*box_0).p2.y);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_path_fixed_line_to(path, (*box_0).p1.x, (*box_0).p2.y);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_path_fixed_close_path(path);
}
unsafe extern "C" fn _cairo_surface_clipper_intersect_clip_boxes(
    mut clipper: *mut cairo_surface_clipper_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut path: cairo_path_fixed_t = cairo_path_fixed_t {
        last_move_point: cairo_point_t { x: 0, y: 0 },
        current_point: cairo_point_t { x: 0, y: 0 },
        has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [0; 1],
        c2rust_padding: [0; 3],
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        buf: cairo_path_buf_fixed_t {
            base: cairo_path_buf_t {
                link: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
                },
                num_ops: 0,
                size_ops: 0,
                num_points: 0,
                size_points: 0,
                op: 0 as *mut cairo_path_op_t,
                points: 0 as *mut cairo_point_t,
            },
            op: [0; 27],
            points: [cairo_point_t { x: 0, y: 0 }; 54],
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if (*clip).num_boxes == 0 as libc::c_int {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_path_fixed_init(&mut path);
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        status = _cairo_path_fixed_add_box(
            &mut path,
            &mut *((*clip).boxes).offset(i as isize),
        );
        if status as u64 != 0 {
            _cairo_path_fixed_fini(&mut path);
            return status;
        }
        i += 1;
    }
    status = ((*clipper).intersect_clip_path)
        .expect(
            "non-null function pointer",
        )(clipper, &mut path, CAIRO_FILL_RULE_WINDING, 0.0f64, CAIRO_ANTIALIAS_DEFAULT);
    _cairo_path_fixed_fini(&mut path);
    return status;
}
unsafe extern "C" fn _cairo_surface_clipper_intersect_clip_path_recursive(
    mut clipper: *mut cairo_surface_clipper_t,
    mut clip_path: *mut cairo_clip_path_t,
    mut end: *mut cairo_clip_path_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*clip_path).prev != end {
        status = _cairo_surface_clipper_intersect_clip_path_recursive(
            clipper,
            (*clip_path).prev,
            end,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    return ((*clipper).intersect_clip_path)
        .expect(
            "non-null function pointer",
        )(
        clipper,
        &mut (*clip_path).path,
        (*clip_path).fill_rule,
        (*clip_path).tolerance,
        (*clip_path).antialias,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_clipper_set_clip(
    mut clipper: *mut cairo_surface_clipper_t,
    mut clip: *const cairo_clip_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut incremental: cairo_bool_t = 0 as libc::c_int;
    if _cairo_clip_equal(clip, (*clipper).clip) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if _cairo_clip_is_all_clipped(clip) == 0 {} else {
        __assert_fail(
            b"!_cairo_clip_is_all_clipped (clip)\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-surface-clipper.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_status_t _cairo_surface_clipper_set_clip(cairo_surface_clipper_t *, const cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if !((*clipper).clip).is_null() && !clip.is_null()
        && (*clip).num_boxes == (*(*clipper).clip).num_boxes
        && memcmp(
            (*clip).boxes as *const libc::c_void,
            (*(*clipper).clip).boxes as *const libc::c_void,
            (::std::mem::size_of::<cairo_box_t>() as libc::c_ulong)
                .wrapping_mul((*clip).num_boxes as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        let mut clip_path: *mut cairo_clip_path_t = (*clip).path;
        while !clip_path.is_null() && clip_path != (*(*clipper).clip).path {
            clip_path = (*clip_path).prev;
        }
        if !clip_path.is_null() {
            incremental = 1 as libc::c_int;
            status = _cairo_surface_clipper_intersect_clip_path_recursive(
                clipper,
                (*clip).path,
                (*(*clipper).clip).path,
            );
        }
    }
    _cairo_clip_destroy((*clipper).clip);
    let ref mut fresh0 = (*clipper).clip;
    *fresh0 = _cairo_clip_copy(clip);
    if incremental != 0 {
        return status;
    }
    status = ((*clipper).intersect_clip_path)
        .expect(
            "non-null function pointer",
        )(
        clipper,
        0 as *mut cairo_path_fixed_t,
        CAIRO_FILL_RULE_WINDING,
        0 as libc::c_int as libc::c_double,
        CAIRO_ANTIALIAS_DEFAULT,
    );
    if status as u64 != 0 {
        return status;
    }
    if clip.is_null() {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_surface_clipper_intersect_clip_boxes(clipper, clip);
    if status as u64 != 0 {
        return status;
    }
    if !((*clip).path).is_null() {
        status = _cairo_surface_clipper_intersect_clip_path_recursive(
            clipper,
            (*clip).path,
            0 as *mut cairo_clip_path_t,
        );
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_clipper_init(
    mut clipper: *mut cairo_surface_clipper_t,
    mut func: cairo_surface_clipper_intersect_clip_path_func_t,
) {
    let ref mut fresh1 = (*clipper).clip;
    *fresh1 = 0 as *mut cairo_clip_t;
    let ref mut fresh2 = (*clipper).intersect_clip_path;
    *fresh2 = func;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_surface_clipper_reset(
    mut clipper: *mut cairo_surface_clipper_t,
) {
    _cairo_clip_destroy((*clipper).clip);
    let ref mut fresh3 = (*clipper).clip;
    *fresh3 = 0 as *mut cairo_clip_t;
}
