use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_clear(boxes: *mut cairo_boxes_t);
    fn _cairo_path_fixed_iter_init(
        iter: *mut cairo_path_fixed_iter_t,
        path: *const cairo_path_fixed_t,
    );
    fn _cairo_path_fixed_iter_is_fill_box(
        _iter: *mut cairo_path_fixed_iter_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_iter_at_end(
        iter: *const cairo_path_fixed_iter_t,
    ) -> cairo_bool_t;
    fn _cairo_spline_intersects(
        a: *const cairo_point_t,
        b: *const cairo_point_t,
        c: *const cairo_point_t,
        d: *const cairo_point_t,
        box_0: *const cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_interpret_flat(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
        tolerance: libc::c_double,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_is_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_polygon_add_external_edge(
        polygon: *mut libc::c_void,
        p1: *const cairo_point_t,
        p2: *const cairo_point_t,
    ) -> cairo_status_t;
    fn _cairo_spline_decompose(
        spline: *mut cairo_spline_t,
        tolerance: libc::c_double,
    ) -> cairo_status_t;
    fn _cairo_spline_init(
        spline: *mut cairo_spline_t,
        add_point_func: cairo_spline_add_point_func_t,
        closure: *mut libc::c_void,
        a: *const cairo_point_t,
        b: *const cairo_point_t,
        c: *const cairo_point_t,
        d: *const cairo_point_t,
    ) -> cairo_bool_t;
    fn _cairo_polygon_fini(polygon: *mut cairo_polygon_t);
    fn _cairo_bentley_ottmann_tessellate_rectilinear_polygon_to_boxes(
        polygon: *const cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_polygon_init(
        polygon: *mut cairo_polygon_t,
        boxes: *const cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_bentley_ottmann_tessellate_boxes(
        in_0: *const cairo_boxes_t,
        fill_rule: cairo_fill_rule_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_bentley_ottmann_tessellate_polygon(
        traps: *mut cairo_traps_t,
        polygon: *const cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type cairo_fixed_unsigned_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_t {
    pub status: cairo_status_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_boxes: libc::c_int,
    pub is_pixel_aligned: libc::c_uint,
    pub chunks: _cairo_boxes_chunk,
    pub tail: *mut _cairo_boxes_chunk,
    pub boxes_embedded: [cairo_box_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_chunk {
    pub next: *mut _cairo_boxes_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_boxes_t = _cairo_boxes_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_traps {
    pub status: cairo_status_t,
    pub bounds: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    #[bitfield(name = "maybe_region", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_intersections", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_rectilinear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "is_rectangular", ty = "libc::c_uint", bits = "3..=3")]
    pub maybe_region_has_intersections_is_rectilinear_is_rectangular: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub num_traps: libc::c_int,
    pub traps_size: libc::c_int,
    pub traps: *mut cairo_trapezoid_t,
    pub traps_embedded: [cairo_trapezoid_t; 16],
}
pub type cairo_trapezoid_t = _cairo_trapezoid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_trapezoid {
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub left: cairo_line_t,
    pub right: cairo_line_t,
}
pub type cairo_line_t = _cairo_line;
pub type cairo_traps_t = _cairo_traps;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_slope {
    pub dx: cairo_fixed_t,
    pub dy: cairo_fixed_t,
}
pub type cairo_slope_t = _cairo_slope;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_edge {
    pub line: cairo_line_t,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
    pub dir: libc::c_int,
}
pub type cairo_edge_t = _cairo_edge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_polygon {
    pub status: cairo_status_t,
    pub extents: cairo_box_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_edges: libc::c_int,
    pub edges_size: libc::c_int,
    pub edges: *mut cairo_edge_t,
    pub edges_embedded: [cairo_edge_t; 32],
}
pub type cairo_polygon_t = _cairo_polygon;
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
pub struct _cairo_path_fixed_iter {
    pub first: *const cairo_path_buf_t,
    pub buf: *const cairo_path_buf_t,
    pub n_op: libc::c_uint,
    pub n_point: libc::c_uint,
}
pub type cairo_path_fixed_iter_t = _cairo_path_fixed_iter;
pub type cairo_path_fixed_move_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_line_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_curve_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
    *const cairo_point_t,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_close_path_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> cairo_status_t;
pub type cairo_filler_t = cairo_filler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_filler {
    pub polygon: *mut cairo_polygon_t,
    pub tolerance: libc::c_double,
    pub limit: cairo_box_t,
    pub has_limits: cairo_bool_t,
    pub current_point: cairo_point_t,
    pub last_move_to: cairo_point_t,
}
pub type cairo_filler_ra_t = cairo_filler_rectilinear_aligned;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_filler_rectilinear_aligned {
    pub polygon: *mut cairo_polygon_t,
    pub current_point: cairo_point_t,
    pub last_move_to: cairo_point_t,
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_fill_is_empty(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).fill_is_empty() as cairo_bool_t;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_round_down(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return _cairo_fixed_floor(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
}
unsafe extern "C" fn _cairo_filler_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_t = closure as *mut cairo_filler_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_polygon_add_external_edge(
        (*filler).polygon as *mut libc::c_void,
        &mut (*filler).current_point,
        point,
    );
    (*filler).current_point = *point;
    return status;
}
unsafe extern "C" fn _cairo_filler_add_point(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    return _cairo_filler_line_to(closure, point);
}
unsafe extern "C" fn _cairo_filler_close(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_t = closure as *mut cairo_filler_t;
    return _cairo_filler_line_to(closure, &mut (*filler).last_move_to);
}
unsafe extern "C" fn _cairo_filler_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_t = closure as *mut cairo_filler_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_filler_close(closure);
    if status as u64 != 0 {
        return status;
    }
    (*filler).current_point = *point;
    (*filler).last_move_to = *point;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_filler_curve_to(
    mut closure: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_t = closure as *mut cairo_filler_t;
    let mut spline: cairo_spline_t = cairo_spline_t {
        add_point_func: None,
        closure: 0 as *mut libc::c_void,
        knots: cairo_spline_knots_t {
            a: cairo_point_t { x: 0, y: 0 },
            b: cairo_point_t { x: 0, y: 0 },
            c: cairo_point_t { x: 0, y: 0 },
            d: cairo_point_t { x: 0, y: 0 },
        },
        initial_slope: cairo_slope_t { dx: 0, dy: 0 },
        final_slope: cairo_slope_t { dx: 0, dy: 0 },
        has_point: 0,
        last_point: cairo_point_t { x: 0, y: 0 },
    };
    if (*filler).has_limits != 0 {
        if _cairo_spline_intersects(
            &mut (*filler).current_point,
            p1,
            p2,
            p3,
            &mut (*filler).limit,
        ) == 0
        {
            return _cairo_filler_line_to(filler as *mut libc::c_void, p3);
        }
    }
    if _cairo_spline_init(
        &mut spline,
        Some(
            _cairo_filler_add_point
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        ),
        filler as *mut libc::c_void,
        &mut (*filler).current_point,
        p1,
        p2,
        p3,
    ) == 0
    {
        return _cairo_filler_line_to(closure, p3);
    }
    return _cairo_spline_decompose(&mut spline, (*filler).tolerance);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_fill_to_polygon(
    mut path: *const cairo_path_fixed_t,
    mut tolerance: libc::c_double,
    mut polygon: *mut cairo_polygon_t,
) -> cairo_status_t {
    let mut filler: cairo_filler_t = cairo_filler_t {
        polygon: 0 as *mut cairo_polygon_t,
        tolerance: 0.,
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        has_limits: 0,
        current_point: cairo_point_t { x: 0, y: 0 },
        last_move_to: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    filler.polygon = polygon;
    filler.tolerance = tolerance;
    filler.has_limits = 0 as libc::c_int;
    if (*polygon).num_limits != 0 {
        filler.has_limits = 1 as libc::c_int;
        filler.limit = (*polygon).limit;
    }
    filler.current_point.x = 0 as libc::c_int;
    filler.current_point.y = 0 as libc::c_int;
    filler.last_move_to = filler.current_point;
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _cairo_filler_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_filler_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_filler_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_filler_close
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut filler as *mut cairo_filler_t as *mut libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    return _cairo_filler_close(&mut filler as *mut cairo_filler_t as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_filler_ra_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_ra_t = closure as *mut cairo_filler_ra_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    p.x = _cairo_fixed_round_down((*point).x);
    p.y = _cairo_fixed_round_down((*point).y);
    status = _cairo_polygon_add_external_edge(
        (*filler).polygon as *mut libc::c_void,
        &mut (*filler).current_point,
        &mut p,
    );
    (*filler).current_point = p;
    return status;
}
unsafe extern "C" fn _cairo_filler_ra_close(
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_ra_t = closure as *mut cairo_filler_ra_t;
    return _cairo_filler_ra_line_to(closure, &mut (*filler).last_move_to);
}
unsafe extern "C" fn _cairo_filler_ra_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut filler: *mut cairo_filler_ra_t = closure as *mut cairo_filler_ra_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut p: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    status = _cairo_filler_ra_close(closure);
    if status as u64 != 0 {
        return status;
    }
    p.x = _cairo_fixed_round_down((*point).x);
    p.y = _cairo_fixed_round_down((*point).y);
    (*filler).current_point = p;
    (*filler).last_move_to = p;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_fill_rectilinear_to_polygon(
    mut path: *const cairo_path_fixed_t,
    mut antialias: cairo_antialias_t,
    mut polygon: *mut cairo_polygon_t,
) -> cairo_status_t {
    let mut filler: cairo_filler_ra_t = cairo_filler_ra_t {
        polygon: 0 as *mut cairo_polygon_t,
        current_point: cairo_point_t { x: 0, y: 0 },
        last_move_to: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if antialias as libc::c_uint != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        return _cairo_path_fixed_fill_to_polygon(path, 0.0f64, polygon);
    }
    filler.polygon = polygon;
    filler.current_point.x = 0 as libc::c_int;
    filler.current_point.y = 0 as libc::c_int;
    filler.last_move_to = filler.current_point;
    status = _cairo_path_fixed_interpret_flat(
        path,
        Some(
            _cairo_filler_ra_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_filler_ra_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cairo_filler_ra_close
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut filler as *mut cairo_filler_ra_t as *mut libc::c_void,
        0.0f64,
    );
    if status as u64 != 0 {
        return status;
    }
    return _cairo_filler_ra_close(
        &mut filler as *mut cairo_filler_ra_t as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_fill_to_traps(
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut traps: *mut cairo_traps_t,
) -> cairo_status_t {
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _cairo_path_fixed_fill_is_empty(path) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_polygon_init(&mut polygon, (*traps).limits, (*traps).num_limits);
    status = _cairo_path_fixed_fill_to_polygon(path, tolerance, &mut polygon);
    if !(status as libc::c_uint != 0 || polygon.num_edges == 0 as libc::c_int) {
        status = _cairo_bentley_ottmann_tessellate_polygon(
            traps,
            &mut polygon,
            fill_rule,
        );
    }
    _cairo_polygon_fini(&mut polygon);
    return status;
}
unsafe extern "C" fn _cairo_path_fixed_fill_rectilinear_tessellate_to_boxes(
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut antialias: cairo_antialias_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_polygon_init(&mut polygon, (*boxes).limits, (*boxes).num_limits);
    (*boxes).num_limits = 0 as libc::c_int;
    status = _cairo_path_fixed_fill_rectilinear_to_polygon(
        path,
        antialias,
        &mut polygon,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _cairo_bentley_ottmann_tessellate_rectilinear_polygon_to_boxes(
            &mut polygon,
            fill_rule,
            boxes,
        );
    }
    _cairo_polygon_fini(&mut polygon);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_fill_rectilinear_to_boxes(
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut antialias: cairo_antialias_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut iter: cairo_path_fixed_iter_t = cairo_path_fixed_iter_t {
        first: 0 as *const cairo_path_buf_t,
        buf: 0 as *const cairo_path_buf_t,
        n_op: 0,
        n_point: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if _cairo_path_fixed_is_box(path, &mut box_0) != 0 {
        return _cairo_boxes_add(boxes, antialias, &mut box_0);
    }
    _cairo_path_fixed_iter_init(&mut iter, path);
    while _cairo_path_fixed_iter_is_fill_box(&mut iter, &mut box_0) != 0 {
        if box_0.p1.y == box_0.p2.y || box_0.p1.x == box_0.p2.x {
            continue;
        }
        if box_0.p1.y > box_0.p2.y {
            let mut t: cairo_fixed_t = 0;
            t = box_0.p1.y;
            box_0.p1.y = box_0.p2.y;
            box_0.p2.y = t;
            t = box_0.p1.x;
            box_0.p1.x = box_0.p2.x;
            box_0.p2.x = t;
        }
        status = _cairo_boxes_add(boxes, antialias, &mut box_0);
        if status as u64 != 0 {
            return status;
        }
    }
    if _cairo_path_fixed_iter_at_end(&mut iter) != 0 {
        return _cairo_bentley_ottmann_tessellate_boxes(boxes, fill_rule, boxes);
    }
    _cairo_boxes_clear(boxes);
    return _cairo_path_fixed_fill_rectilinear_tessellate_to_boxes(
        path,
        fill_rule,
        antialias,
        boxes,
    );
}
