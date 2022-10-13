use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn cairo_matrix_transform_point(
        matrix: *const cairo_matrix_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_box_add_curve_to(
        extents: *mut cairo_box_t,
        a: *const cairo_point_t,
        b: *const cairo_point_t,
        c: *const cairo_point_t,
        d: *const cairo_point_t,
    );
    fn _cairo_hash_bytes(
        hash: uintptr_t,
        bytes: *const libc::c_void,
        length: libc::c_uint,
    ) -> uintptr_t;
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
    fn _cairo_path_bounder_extents(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_matrix_transform_bounding_box_fixed(
        matrix: *const cairo_matrix_t,
        bbox: *mut cairo_box_t,
        is_tight: *mut cairo_bool_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type uintptr_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type cairo_int64_t = int64_t;
pub type cairo_fixed_unsigned_t = uint32_t;
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
pub type cairo_path_op = libc::c_uint;
pub const CAIRO_PATH_OP_CLOSE_PATH: cairo_path_op = 3;
pub const CAIRO_PATH_OP_CURVE_TO: cairo_path_op = 2;
pub const CAIRO_PATH_OP_LINE_TO: cairo_path_op = 1;
pub const CAIRO_PATH_OP_MOVE_TO: cairo_path_op = 0;
pub type cairo_path_fixed_append_closure_t = _cairo_path_fixed_append_closure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_fixed_append_closure {
    pub offset: cairo_point_t,
    pub path: *mut cairo_path_fixed_t,
}
pub type cairo_path_fixed_close_path_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> cairo_status_t;
pub type cairo_path_fixed_curve_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
    *const cairo_point_t,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_line_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
pub type cairo_path_fixed_move_to_func_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const cairo_point_t,
) -> cairo_status_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_path_fixed_iter {
    pub first: *const cairo_path_buf_t,
    pub buf: *const cairo_path_buf_t,
    pub n_op: libc::c_uint,
    pub n_point: libc::c_uint,
}
pub type cairo_path_fixed_iter_t = _cairo_path_fixed_iter;
pub type cpf_t = cairo_path_flattener;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_path_flattener {
    pub tolerance: libc::c_double,
    pub current_point: cairo_point_t,
    pub move_to: Option::<cairo_path_fixed_move_to_func_t>,
    pub line_to: Option::<cairo_path_fixed_line_to_func_t>,
    pub close_path: Option::<cairo_path_fixed_close_path_func_t>,
    pub closure: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab_plus_c(
    mut a: size_t,
    mut size: size_t,
    mut c: size_t,
) -> *mut libc::c_void {
    let mut d: size_t = 0;
    let mut e: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut d as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    let (fresh2, fresh3) = d.overflowing_add(c);
    *(&mut e as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return if e != 0 as libc::c_int as libc::c_ulong {
        malloc(e)
    } else {
        0 as *mut libc::c_void
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
unsafe extern "C" fn _cairo_fixed_mul(
    mut a: cairo_fixed_t,
    mut b: cairo_fixed_t,
) -> cairo_fixed_t {
    let mut temp: cairo_int64_t = a as int64_t * b as libc::c_long;
    return (temp as uint64_t >> 8 as libc::c_int) as int64_t as int32_t;
}
#[inline]
unsafe extern "C" fn _cairo_box_set(
    mut box_0: *mut cairo_box_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) {
    (*box_0).p1 = *p1;
    (*box_0).p2 = *p2;
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
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh4 = (*entry).next;
    *fresh4 = entry;
    let ref mut fresh5 = (*entry).prev;
    *fresh5 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh6 = (*next).prev;
    *fresh6 = entry;
    let ref mut fresh7 = (*entry).next;
    *fresh7 = next;
    let ref mut fresh8 = (*entry).prev;
    *fresh8 = prev;
    let ref mut fresh9 = (*prev).next;
    *fresh9 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add_tail(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, (*head).prev, head);
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
#[inline]
unsafe extern "C" fn _cairo_slope_equal(
    mut a: *const cairo_slope_t,
    mut b: *const cairo_slope_t,
) -> cairo_bool_t {
    return ((*a).dy as int64_t * (*b).dx as libc::c_long
        == (*b).dy as int64_t * (*a).dx as libc::c_long) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_slope_backwards(
    mut a: *const cairo_slope_t,
    mut b: *const cairo_slope_t,
) -> cairo_bool_t {
    return (((*a).dx as int64_t * (*b).dx as libc::c_long
        + (*a).dy as int64_t * (*b).dy as libc::c_long)
        < 0 as libc::c_int as libc::c_long) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_init(mut path: *mut cairo_path_fixed_t) {
    cairo_list_init(&mut (*path).buf.base.link);
    (*path).buf.base.num_ops = 0 as libc::c_int as libc::c_uint;
    (*path).buf.base.num_points = 0 as libc::c_int as libc::c_uint;
    (*path)
        .buf
        .base
        .size_ops = (::std::mem::size_of::<[cairo_path_op_t; 27]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong)
        as libc::c_int as libc::c_uint;
    (*path)
        .buf
        .base
        .size_points = (::std::mem::size_of::<[cairo_point_t; 54]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong)
        as libc::c_int as libc::c_uint;
    let ref mut fresh10 = (*path).buf.base.op;
    *fresh10 = ((*path).buf.op).as_mut_ptr();
    let ref mut fresh11 = (*path).buf.base.points;
    *fresh11 = ((*path).buf.points).as_mut_ptr();
    (*path).current_point.x = 0 as libc::c_int;
    (*path).current_point.y = 0 as libc::c_int;
    (*path).last_move_point = (*path).current_point;
    (*path).set_has_current_point(0 as libc::c_int as libc::c_uint);
    (*path).set_needs_move_to(1 as libc::c_int as libc::c_uint);
    (*path).set_has_extents(0 as libc::c_int as libc::c_uint);
    (*path).set_has_curve_to(0 as libc::c_int as libc::c_uint);
    (*path).set_stroke_is_rectilinear(1 as libc::c_int as libc::c_uint);
    (*path).set_fill_is_rectilinear(1 as libc::c_int as libc::c_uint);
    (*path).set_fill_maybe_region(1 as libc::c_int as libc::c_uint);
    (*path).set_fill_is_empty(1 as libc::c_int as libc::c_uint);
    let ref mut fresh12 = (*path).extents.p1.y;
    *fresh12 = 0 as libc::c_int;
    (*path).extents.p1.x = *fresh12;
    let ref mut fresh13 = (*path).extents.p2.y;
    *fresh13 = 0 as libc::c_int;
    (*path).extents.p2.x = *fresh13;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_init_copy(
    mut path: *mut cairo_path_fixed_t,
    mut other: *const cairo_path_fixed_t,
) -> cairo_status_t {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    let mut other_buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    let mut num_points: libc::c_uint = 0;
    let mut num_ops: libc::c_uint = 0;
    cairo_list_init(&mut (*path).buf.base.link);
    let ref mut fresh14 = (*path).buf.base.op;
    *fresh14 = ((*path).buf.op).as_mut_ptr();
    let ref mut fresh15 = (*path).buf.base.points;
    *fresh15 = ((*path).buf.points).as_mut_ptr();
    (*path)
        .buf
        .base
        .size_ops = (::std::mem::size_of::<[cairo_path_op_t; 27]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong)
        as libc::c_int as libc::c_uint;
    (*path)
        .buf
        .base
        .size_points = (::std::mem::size_of::<[cairo_point_t; 54]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong)
        as libc::c_int as libc::c_uint;
    (*path).current_point = (*other).current_point;
    (*path).last_move_point = (*other).last_move_point;
    (*path).set_has_current_point((*other).has_current_point());
    (*path).set_needs_move_to((*other).needs_move_to());
    (*path).set_has_extents((*other).has_extents());
    (*path).set_has_curve_to((*other).has_curve_to());
    (*path).set_stroke_is_rectilinear((*other).stroke_is_rectilinear());
    (*path).set_fill_is_rectilinear((*other).fill_is_rectilinear());
    (*path).set_fill_maybe_region((*other).fill_maybe_region());
    (*path).set_fill_is_empty((*other).fill_is_empty());
    (*path).extents = (*other).extents;
    (*path).buf.base.num_ops = (*other).buf.base.num_ops;
    (*path).buf.base.num_points = (*other).buf.base.num_points;
    memcpy(
        ((*path).buf.op).as_mut_ptr() as *mut libc::c_void,
        (*other).buf.base.op as *const libc::c_void,
        ((*other).buf.base.num_ops as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong),
    );
    memcpy(
        ((*path).buf.points).as_mut_ptr() as *mut libc::c_void,
        ((*other).buf.points).as_ptr() as *const libc::c_void,
        ((*other).buf.base.num_points as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong),
    );
    num_ops = 0 as libc::c_int as libc::c_uint;
    num_points = num_ops;
    other_buf = ({
        let mut mptr__: *const cairo_list_t = (*other).buf.base.link.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_path_buf_t
    });
    while other_buf
        != &(*other).buf.base as *const cairo_path_buf_t as *mut cairo_path_buf_t
    {
        num_ops = num_ops.wrapping_add((*other_buf).num_ops);
        num_points = num_points.wrapping_add((*other_buf).num_points);
        other_buf = ({
            let mut mptr__: *const cairo_list_t = (*other_buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
    }
    if num_ops != 0 {
        buf = _cairo_path_buf_create(num_ops as libc::c_int, num_points as libc::c_int);
        if buf.is_null() {
            _cairo_path_fixed_fini(path);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        other_buf = ({
            let mut mptr__: *const cairo_list_t = (*other).buf.base.link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        while other_buf
            != &(*other).buf.base as *const cairo_path_buf_t as *mut cairo_path_buf_t
        {
            memcpy(
                ((*buf).op).offset((*buf).num_ops as isize) as *mut libc::c_void,
                (*other_buf).op as *const libc::c_void,
                ((*other_buf).num_ops as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong,
                    ),
            );
            let ref mut fresh16 = (*buf).num_ops;
            *fresh16 = (*fresh16).wrapping_add((*other_buf).num_ops);
            memcpy(
                ((*buf).points).offset((*buf).num_points as isize) as *mut libc::c_void,
                (*other_buf).points as *const libc::c_void,
                ((*other_buf).num_points as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
                    ),
            );
            let ref mut fresh17 = (*buf).num_points;
            *fresh17 = (*fresh17).wrapping_add((*other_buf).num_points);
            other_buf = ({
                let mut mptr__: *const cairo_list_t = (*other_buf).link.next;
                (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                    as *mut cairo_path_buf_t
            });
        }
        _cairo_path_fixed_add_buf(path, buf);
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_hash(
    mut path: *const cairo_path_fixed_t,
) -> uintptr_t {
    let mut hash: uintptr_t = 5381 as libc::c_int as uintptr_t;
    let mut buf: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    let mut count: libc::c_uint = 0;
    count = 0 as libc::c_int as libc::c_uint;
    buf = &(*path).buf.base;
    loop {
        hash = _cairo_hash_bytes(
            hash,
            (*buf).op as *const libc::c_void,
            ((*buf).num_ops as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong)
                as libc::c_uint,
        );
        count = count.wrapping_add((*buf).num_ops);
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &(*path).buf.base as *const cairo_path_buf_t) {
            break;
        }
    }
    hash = _cairo_hash_bytes(
        hash,
        &mut count as *mut libc::c_uint as *const libc::c_void,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_uint,
    );
    count = 0 as libc::c_int as libc::c_uint;
    buf = &(*path).buf.base;
    loop {
        hash = _cairo_hash_bytes(
            hash,
            (*buf).points as *const libc::c_void,
            ((*buf).num_points as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong)
                as libc::c_uint,
        );
        count = count.wrapping_add((*buf).num_points);
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &(*path).buf.base as *const cairo_path_buf_t) {
            break;
        }
    }
    hash = _cairo_hash_bytes(
        hash,
        &mut count as *mut libc::c_uint as *const libc::c_void,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_uint,
    );
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_size(
    mut path: *const cairo_path_fixed_t,
) -> libc::c_ulong {
    let mut buf: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    let mut num_points: libc::c_int = 0;
    let mut num_ops: libc::c_int = 0;
    num_points = 0 as libc::c_int;
    num_ops = num_points;
    buf = &(*path).buf.base;
    loop {
        num_ops = (num_ops as libc::c_uint).wrapping_add((*buf).num_ops) as libc::c_int
            as libc::c_int;
        num_points = (num_points as libc::c_uint).wrapping_add((*buf).num_points)
            as libc::c_int as libc::c_int;
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &(*path).buf.base as *const cairo_path_buf_t) {
            break;
        }
    }
    return (num_ops as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong)
        .wrapping_add(
            (num_points as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong),
        );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_equal(
    mut a: *const cairo_path_fixed_t,
    mut b: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    let mut buf_a: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    let mut buf_b: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    let mut ops_a: *const cairo_path_op_t = 0 as *const cairo_path_op_t;
    let mut ops_b: *const cairo_path_op_t = 0 as *const cairo_path_op_t;
    let mut points_a: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut points_b: *const cairo_point_t = 0 as *const cairo_point_t;
    let mut num_points_a: libc::c_int = 0;
    let mut num_ops_a: libc::c_int = 0;
    let mut num_points_b: libc::c_int = 0;
    let mut num_ops_b: libc::c_int = 0;
    if a == b {
        return 1 as libc::c_int;
    }
    if (*a).has_curve_to() as libc::c_int != (*b).has_curve_to() as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*a).extents.p1.x != (*b).extents.p1.x || (*a).extents.p1.y != (*b).extents.p1.y
        || (*a).extents.p2.x != (*b).extents.p2.x
        || (*a).extents.p2.y != (*b).extents.p2.y
    {
        return 0 as libc::c_int;
    }
    num_points_a = 0 as libc::c_int;
    num_ops_a = num_points_a;
    buf_a = &(*a).buf.base;
    loop {
        num_ops_a = (num_ops_a as libc::c_uint).wrapping_add((*buf_a).num_ops)
            as libc::c_int as libc::c_int;
        num_points_a = (num_points_a as libc::c_uint).wrapping_add((*buf_a).num_points)
            as libc::c_int as libc::c_int;
        buf_a = ({
            let mut mptr__: *const cairo_list_t = (*buf_a).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf_a != &(*a).buf.base as *const cairo_path_buf_t) {
            break;
        }
    }
    num_points_b = 0 as libc::c_int;
    num_ops_b = num_points_b;
    buf_b = &(*b).buf.base;
    loop {
        num_ops_b = (num_ops_b as libc::c_uint).wrapping_add((*buf_b).num_ops)
            as libc::c_int as libc::c_int;
        num_points_b = (num_points_b as libc::c_uint).wrapping_add((*buf_b).num_points)
            as libc::c_int as libc::c_int;
        buf_b = ({
            let mut mptr__: *const cairo_list_t = (*buf_b).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf_b != &(*b).buf.base as *const cairo_path_buf_t) {
            break;
        }
    }
    if num_ops_a == 0 as libc::c_int && num_ops_b == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if num_ops_a != num_ops_b || num_points_a != num_points_b {
        return 0 as libc::c_int;
    }
    buf_a = &(*a).buf.base;
    num_points_a = (*buf_a).num_points as libc::c_int;
    num_ops_a = (*buf_a).num_ops as libc::c_int;
    ops_a = (*buf_a).op;
    points_a = (*buf_a).points;
    buf_b = &(*b).buf.base;
    num_points_b = (*buf_b).num_points as libc::c_int;
    num_ops_b = (*buf_b).num_ops as libc::c_int;
    ops_b = (*buf_b).op;
    points_b = (*buf_b).points;
    loop {
        let mut num_ops: libc::c_int = if num_ops_a < num_ops_b {
            num_ops_a
        } else {
            num_ops_b
        };
        let mut num_points: libc::c_int = if num_points_a < num_points_b {
            num_points_a
        } else {
            num_points_b
        };
        if memcmp(
            ops_a as *const libc::c_void,
            ops_b as *const libc::c_void,
            (num_ops as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_path_op_t>() as libc::c_ulong),
        ) != 0
        {
            return 0 as libc::c_int;
        }
        if memcmp(
            points_a as *const libc::c_void,
            points_b as *const libc::c_void,
            (num_points as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cairo_point_t>() as libc::c_ulong),
        ) != 0
        {
            return 0 as libc::c_int;
        }
        num_ops_a -= num_ops;
        ops_a = ops_a.offset(num_ops as isize);
        num_points_a -= num_points;
        points_a = points_a.offset(num_points as isize);
        if num_ops_a == 0 as libc::c_int || num_points_a == 0 as libc::c_int {
            if num_ops_a != 0 || num_points_a != 0 {
                return 0 as libc::c_int;
            }
            buf_a = ({
                let mut mptr__: *const cairo_list_t = (*buf_a).link.next;
                (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                    as *mut cairo_path_buf_t
            });
            if buf_a == &(*a).buf.base as *const cairo_path_buf_t {
                break;
            }
            num_points_a = (*buf_a).num_points as libc::c_int;
            num_ops_a = (*buf_a).num_ops as libc::c_int;
            ops_a = (*buf_a).op;
            points_a = (*buf_a).points;
        }
        num_ops_b -= num_ops;
        ops_b = ops_b.offset(num_ops as isize);
        num_points_b -= num_points;
        points_b = points_b.offset(num_points as isize);
        if !(num_ops_b == 0 as libc::c_int || num_points_b == 0 as libc::c_int) {
            continue;
        }
        if num_ops_b != 0 || num_points_b != 0 {
            return 0 as libc::c_int;
        }
        buf_b = ({
            let mut mptr__: *const cairo_list_t = (*buf_b).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if buf_b == &(*b).buf.base as *const cairo_path_buf_t {
            break;
        }
        num_points_b = (*buf_b).num_points as libc::c_int;
        num_ops_b = (*buf_b).num_ops as libc::c_int;
        ops_b = (*buf_b).op;
        points_b = (*buf_b).points;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_create() -> *mut cairo_path_fixed_t {
    let mut path: *mut cairo_path_fixed_t = 0 as *mut cairo_path_fixed_t;
    path = (if ::std::mem::size_of::<cairo_path_fixed_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_path_fixed_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_path_fixed_t;
    if path.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_path_fixed_t;
    }
    _cairo_path_fixed_init(path);
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_fini(mut path: *mut cairo_path_fixed_t) {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    buf = ({
        let mut mptr__: *const cairo_list_t = (*path).buf.base.link.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_path_buf_t
    });
    while buf != &mut (*path).buf.base as *mut cairo_path_buf_t {
        let mut this: *mut cairo_path_buf_t = buf;
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        _cairo_path_buf_destroy(this);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_destroy(mut path: *mut cairo_path_fixed_t) {
    _cairo_path_fixed_fini(path);
    free(path as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_path_fixed_last_op(
    mut path: *mut cairo_path_fixed_t,
) -> cairo_path_op_t {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    buf = ({
        let mut mptr__: *const cairo_list_t = (*path).buf.base.link.prev;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_path_buf_t
    });
    if (*buf).num_ops != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"buf->num_ops != 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-path-fixed.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"cairo_path_op_t _cairo_path_fixed_last_op(cairo_path_fixed_t *)\0"))
                .as_ptr(),
        );
    }
    return *((*buf).op)
        .offset(
            ((*buf).num_ops).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        );
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_penultimate_point(
    mut path: *mut cairo_path_fixed_t,
) -> *const cairo_point_t {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    buf = ({
        let mut mptr__: *const cairo_list_t = (*path).buf.base.link.prev;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_path_buf_t
    });
    if (*buf).num_points >= 2 as libc::c_int as libc::c_uint {
        return &mut *((*buf).points)
            .offset(
                ((*buf).num_points).wrapping_sub(2 as libc::c_int as libc::c_uint)
                    as isize,
            ) as *mut cairo_point_t
    } else {
        let mut prev_buf: *mut cairo_path_buf_t = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.prev;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if (*prev_buf).num_points
            >= (2 as libc::c_int as libc::c_uint).wrapping_sub((*buf).num_points)
        {} else {
            __assert_fail(
                b"prev_buf->num_points >= 2 - buf->num_points\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-path-fixed.c\0" as *const u8 as *const libc::c_char,
                382 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"const cairo_point_t *_cairo_path_fixed_penultimate_point(cairo_path_fixed_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        return &mut *((*prev_buf).points)
            .offset(
                ((*prev_buf).num_points)
                    .wrapping_sub(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_sub((*buf).num_points),
                    ) as isize,
            ) as *mut cairo_point_t;
    };
}
unsafe extern "C" fn _cairo_path_fixed_drop_line_to(mut path: *mut cairo_path_fixed_t) {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    if _cairo_path_fixed_last_op(path) as libc::c_int
        == CAIRO_PATH_OP_LINE_TO as libc::c_int
    {} else {
        __assert_fail(
            b"_cairo_path_fixed_last_op (path) == CAIRO_PATH_OP_LINE_TO\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-path-fixed.c\0" as *const u8 as *const libc::c_char,
            392 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void _cairo_path_fixed_drop_line_to(cairo_path_fixed_t *)\0"))
                .as_ptr(),
        );
    }
    buf = ({
        let mut mptr__: *const cairo_list_t = (*path).buf.base.link.prev;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_path_buf_t
    });
    let ref mut fresh18 = (*buf).num_points;
    *fresh18 = (*fresh18).wrapping_sub(1);
    let ref mut fresh19 = (*buf).num_ops;
    *fresh19 = (*fresh19).wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_move_to(
    mut path: *mut cairo_path_fixed_t,
    mut x: cairo_fixed_t,
    mut y: cairo_fixed_t,
) -> cairo_status_t {
    _cairo_path_fixed_new_sub_path(path);
    (*path).set_has_current_point(1 as libc::c_int as libc::c_uint);
    (*path).current_point.x = x;
    (*path).current_point.y = y;
    (*path).last_move_point = (*path).current_point;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_fixed_move_to_apply(
    mut path: *mut cairo_path_fixed_t,
) -> cairo_status_t {
    if (*path).needs_move_to() == 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    (*path).set_needs_move_to(0 as libc::c_int as libc::c_uint);
    if (*path).has_extents() != 0 {
        _cairo_box_add_point(&mut (*path).extents, &mut (*path).current_point);
    } else {
        _cairo_box_set(
            &mut (*path).extents,
            &mut (*path).current_point,
            &mut (*path).current_point,
        );
        (*path).set_has_extents(1 as libc::c_int as libc::c_uint);
    }
    if (*path).fill_maybe_region() != 0 {
        (*path)
            .set_fill_maybe_region(
                (_cairo_fixed_is_integer((*path).current_point.x) != 0
                    && _cairo_fixed_is_integer((*path).current_point.y) != 0)
                    as libc::c_int as libc::c_uint,
            );
    }
    (*path).last_move_point = (*path).current_point;
    return _cairo_path_fixed_add(
        path,
        CAIRO_PATH_OP_MOVE_TO as libc::c_int as cairo_path_op_t,
        &mut (*path).current_point,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_new_sub_path(
    mut path: *mut cairo_path_fixed_t,
) {
    if (*path).needs_move_to() == 0 {
        if (*path).fill_is_rectilinear() != 0 {
            (*path)
                .set_fill_is_rectilinear(
                    ((*path).current_point.x == (*path).last_move_point.x
                        || (*path).current_point.y == (*path).last_move_point.y)
                        as libc::c_int as libc::c_uint,
                );
            (*path)
                .set_fill_maybe_region(
                    (*path).fill_maybe_region()
                        & (*path).fill_is_rectilinear() as libc::c_int as libc::c_uint,
                );
        }
        (*path).set_needs_move_to(1 as libc::c_int as libc::c_uint);
    }
    (*path).set_has_current_point(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_rel_move_to(
    mut path: *mut cairo_path_fixed_t,
    mut dx: cairo_fixed_t,
    mut dy: cairo_fixed_t,
) -> cairo_status_t {
    if (*path).has_current_point() == 0 {
        return _cairo_error(CAIRO_STATUS_NO_CURRENT_POINT);
    }
    return _cairo_path_fixed_move_to(
        path,
        (*path).current_point.x + dx,
        (*path).current_point.y + dy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_line_to(
    mut path: *mut cairo_path_fixed_t,
    mut x: cairo_fixed_t,
    mut y: cairo_fixed_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut point: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    point.x = x;
    point.y = y;
    if (*path).has_current_point() == 0 {
        return _cairo_path_fixed_move_to(path, point.x, point.y);
    }
    status = _cairo_path_fixed_move_to_apply(path);
    if status as u64 != 0 {
        return status;
    }
    if _cairo_path_fixed_last_op(path) as libc::c_int
        != CAIRO_PATH_OP_MOVE_TO as libc::c_int
    {
        if x == (*path).current_point.x && y == (*path).current_point.y {
            return CAIRO_STATUS_SUCCESS;
        }
    }
    if _cairo_path_fixed_last_op(path) as libc::c_int
        == CAIRO_PATH_OP_LINE_TO as libc::c_int
    {
        let mut p: *const cairo_point_t = 0 as *const cairo_point_t;
        p = _cairo_path_fixed_penultimate_point(path);
        if (*p).x == (*path).current_point.x && (*p).y == (*path).current_point.y {
            _cairo_path_fixed_drop_line_to(path);
        } else {
            let mut prev: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            let mut self_0: cairo_slope_t = cairo_slope_t { dx: 0, dy: 0 };
            _cairo_slope_init(&mut prev, p, &mut (*path).current_point);
            _cairo_slope_init(&mut self_0, &mut (*path).current_point, &mut point);
            if _cairo_slope_equal(&mut prev, &mut self_0) != 0
                && _cairo_slope_backwards(&mut prev, &mut self_0) == 0
            {
                _cairo_path_fixed_drop_line_to(path);
            }
        }
    }
    if (*path).stroke_is_rectilinear() != 0 {
        (*path)
            .set_stroke_is_rectilinear(
                ((*path).current_point.x == x || (*path).current_point.y == y)
                    as libc::c_int as libc::c_uint,
            );
        (*path)
            .set_fill_is_rectilinear(
                (*path).fill_is_rectilinear()
                    & (*path).stroke_is_rectilinear() as libc::c_int as libc::c_uint,
            );
        (*path)
            .set_fill_maybe_region(
                (*path).fill_maybe_region()
                    & (*path).fill_is_rectilinear() as libc::c_int as libc::c_uint,
            );
        if (*path).fill_maybe_region() != 0 {
            (*path)
                .set_fill_maybe_region(
                    (_cairo_fixed_is_integer(x) != 0 && _cairo_fixed_is_integer(y) != 0)
                        as libc::c_int as libc::c_uint,
                );
        }
        if (*path).fill_is_empty() != 0 {
            (*path)
                .set_fill_is_empty(
                    ((*path).current_point.x == x && (*path).current_point.y == y)
                        as libc::c_int as libc::c_uint,
                );
        }
    }
    (*path).current_point = point;
    _cairo_box_add_point(&mut (*path).extents, &mut point);
    return _cairo_path_fixed_add(
        path,
        CAIRO_PATH_OP_LINE_TO as libc::c_int as cairo_path_op_t,
        &mut point,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_rel_line_to(
    mut path: *mut cairo_path_fixed_t,
    mut dx: cairo_fixed_t,
    mut dy: cairo_fixed_t,
) -> cairo_status_t {
    if (*path).has_current_point() == 0 {
        return _cairo_error(CAIRO_STATUS_NO_CURRENT_POINT);
    }
    return _cairo_path_fixed_line_to(
        path,
        (*path).current_point.x + dx,
        (*path).current_point.y + dy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_curve_to(
    mut path: *mut cairo_path_fixed_t,
    mut x0: cairo_fixed_t,
    mut y0: cairo_fixed_t,
    mut x1: cairo_fixed_t,
    mut y1: cairo_fixed_t,
    mut x2: cairo_fixed_t,
    mut y2: cairo_fixed_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut point: [cairo_point_t; 3] = [cairo_point_t { x: 0, y: 0 }; 3];
    if (*path).current_point.x == x2 && (*path).current_point.y == y2 {
        if x1 == x2 && x0 == x2 && y1 == y2 && y0 == y2 {
            return _cairo_path_fixed_line_to(path, x2, y2);
        }
    }
    if (*path).has_current_point() == 0 {
        status = _cairo_path_fixed_move_to(path, x0, y0);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-path-fixed.c\0" as *const u8 as *const libc::c_char,
                591 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 154],
                    &[libc::c_char; 154],
                >(
                    b"cairo_status_t _cairo_path_fixed_curve_to(cairo_path_fixed_t *, cairo_fixed_t, cairo_fixed_t, cairo_fixed_t, cairo_fixed_t, cairo_fixed_t, cairo_fixed_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    status = _cairo_path_fixed_move_to_apply(path);
    if status as u64 != 0 {
        return status;
    }
    if _cairo_path_fixed_last_op(path) as libc::c_int
        == CAIRO_PATH_OP_LINE_TO as libc::c_int
    {
        let mut p: *const cairo_point_t = 0 as *const cairo_point_t;
        p = _cairo_path_fixed_penultimate_point(path);
        if (*p).x == (*path).current_point.x && (*p).y == (*path).current_point.y {
            _cairo_path_fixed_drop_line_to(path);
        }
    }
    point[0 as libc::c_int as usize].x = x0;
    point[0 as libc::c_int as usize].y = y0;
    point[1 as libc::c_int as usize].x = x1;
    point[1 as libc::c_int as usize].y = y1;
    point[2 as libc::c_int as usize].x = x2;
    point[2 as libc::c_int as usize].y = y2;
    _cairo_box_add_curve_to(
        &mut (*path).extents,
        &mut (*path).current_point,
        &mut *point.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *point.as_mut_ptr().offset(1 as libc::c_int as isize),
        &mut *point.as_mut_ptr().offset(2 as libc::c_int as isize),
    );
    (*path).current_point = point[2 as libc::c_int as usize];
    (*path).set_has_curve_to(1 as libc::c_int as libc::c_uint);
    (*path).set_stroke_is_rectilinear(0 as libc::c_int as libc::c_uint);
    (*path).set_fill_is_rectilinear(0 as libc::c_int as libc::c_uint);
    (*path).set_fill_maybe_region(0 as libc::c_int as libc::c_uint);
    (*path).set_fill_is_empty(0 as libc::c_int as libc::c_uint);
    return _cairo_path_fixed_add(
        path,
        CAIRO_PATH_OP_CURVE_TO as libc::c_int as cairo_path_op_t,
        point.as_mut_ptr(),
        3 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_rel_curve_to(
    mut path: *mut cairo_path_fixed_t,
    mut dx0: cairo_fixed_t,
    mut dy0: cairo_fixed_t,
    mut dx1: cairo_fixed_t,
    mut dy1: cairo_fixed_t,
    mut dx2: cairo_fixed_t,
    mut dy2: cairo_fixed_t,
) -> cairo_status_t {
    if (*path).has_current_point() == 0 {
        return _cairo_error(CAIRO_STATUS_NO_CURRENT_POINT);
    }
    return _cairo_path_fixed_curve_to(
        path,
        (*path).current_point.x + dx0,
        (*path).current_point.y + dy0,
        (*path).current_point.x + dx1,
        (*path).current_point.y + dy1,
        (*path).current_point.x + dx2,
        (*path).current_point.y + dy2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_close_path(
    mut path: *mut cairo_path_fixed_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*path).has_current_point() == 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_path_fixed_line_to(
        path,
        (*path).last_move_point.x,
        (*path).last_move_point.y,
    );
    if status as u64 != 0 {
        return status;
    }
    if _cairo_path_fixed_last_op(path) as libc::c_int
        == CAIRO_PATH_OP_LINE_TO as libc::c_int
    {
        _cairo_path_fixed_drop_line_to(path);
    }
    (*path).set_needs_move_to(1 as libc::c_int as libc::c_uint);
    return _cairo_path_fixed_add(
        path,
        CAIRO_PATH_OP_CLOSE_PATH as libc::c_int as cairo_path_op_t,
        0 as *const cairo_point_t,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_get_current_point(
    mut path: *mut cairo_path_fixed_t,
    mut x: *mut cairo_fixed_t,
    mut y: *mut cairo_fixed_t,
) -> cairo_bool_t {
    if (*path).has_current_point() == 0 {
        return 0 as libc::c_int;
    }
    *x = (*path).current_point.x;
    *y = (*path).current_point.y;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_path_fixed_add(
    mut path: *mut cairo_path_fixed_t,
    mut op: cairo_path_op_t,
    mut points: *const cairo_point_t,
    mut num_points: libc::c_int,
) -> cairo_status_t {
    let mut buf: *mut cairo_path_buf_t = ({
        let mut mptr__: *const cairo_list_t = (*path).buf.base.link.prev;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_path_buf_t
    });
    if ((*buf).num_ops).wrapping_add(1 as libc::c_int as libc::c_uint) > (*buf).size_ops
        || ((*buf).num_points).wrapping_add(num_points as libc::c_uint)
            > (*buf).size_points
    {
        buf = _cairo_path_buf_create(
            ((*buf).num_ops).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as libc::c_int,
            ((*buf).num_points).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as libc::c_int,
        );
        if buf.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        _cairo_path_fixed_add_buf(path, buf);
    }
    _cairo_path_buf_add_op(buf, op);
    _cairo_path_buf_add_points(buf, points, num_points);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_path_fixed_add_buf(
    mut path: *mut cairo_path_fixed_t,
    mut buf: *mut cairo_path_buf_t,
) {
    cairo_list_add_tail(&mut (*buf).link, &mut (*path).buf.base.link);
}
unsafe extern "C" fn _cairo_path_buf_create(
    mut size_ops: libc::c_int,
    mut size_points: libc::c_int,
) -> *mut cairo_path_buf_t {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    size_ops = (size_ops as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_sub(
                    (::std::mem::size_of::<cairo_path_buf_t>() as libc::c_ulong)
                        .wrapping_add(size_ops as libc::c_ulong)
                        .wrapping_rem(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                ),
        ) as libc::c_int as libc::c_int;
    buf = _cairo_malloc_ab_plus_c(
        size_points as size_t,
        ::std::mem::size_of::<cairo_point_t>() as libc::c_ulong,
        (size_ops as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<cairo_path_buf_t>() as libc::c_ulong),
    ) as *mut cairo_path_buf_t;
    if !buf.is_null() {
        (*buf).num_ops = 0 as libc::c_int as libc::c_uint;
        (*buf).num_points = 0 as libc::c_int as libc::c_uint;
        (*buf).size_ops = size_ops as libc::c_uint;
        (*buf).size_points = size_points as libc::c_uint;
        let ref mut fresh20 = (*buf).op;
        *fresh20 = buf.offset(1 as libc::c_int as isize) as *mut cairo_path_op_t;
        let ref mut fresh21 = (*buf).points;
        *fresh21 = ((*buf).op).offset(size_ops as isize) as *mut cairo_point_t;
    }
    return buf;
}
unsafe extern "C" fn _cairo_path_buf_destroy(mut buf: *mut cairo_path_buf_t) {
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_path_buf_add_op(
    mut buf: *mut cairo_path_buf_t,
    mut op: cairo_path_op_t,
) {
    let ref mut fresh22 = (*buf).num_ops;
    let fresh23 = *fresh22;
    *fresh22 = (*fresh22).wrapping_add(1);
    *((*buf).op).offset(fresh23 as isize) = op;
}
unsafe extern "C" fn _cairo_path_buf_add_points(
    mut buf: *mut cairo_path_buf_t,
    mut points: *const cairo_point_t,
    mut num_points: libc::c_int,
) {
    if num_points == 0 as libc::c_int {
        return;
    }
    memcpy(
        ((*buf).points).offset((*buf).num_points as isize) as *mut libc::c_void,
        points as *const libc::c_void,
        (::std::mem::size_of::<cairo_point_t>() as libc::c_ulong)
            .wrapping_mul(num_points as libc::c_ulong),
    );
    let ref mut fresh24 = (*buf).num_points;
    *fresh24 = (*fresh24).wrapping_add(num_points as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_interpret(
    mut path: *const cairo_path_fixed_t,
    mut move_to: Option::<cairo_path_fixed_move_to_func_t>,
    mut line_to: Option::<cairo_path_fixed_line_to_func_t>,
    mut curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
    mut close_path: Option::<cairo_path_fixed_close_path_func_t>,
    mut closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut buf: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    buf = &(*path).buf.base;
    loop {
        let mut points: *const cairo_point_t = (*buf).points;
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*buf).num_ops {
            let mut current_block_11: u64;
            match *((*buf).op).offset(i as isize) as libc::c_int {
                0 => {
                    status = (Some(move_to.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(closure, &*points.offset(0 as libc::c_int as isize));
                    points = points.offset(1 as libc::c_int as isize);
                    current_block_11 = 7175849428784450219;
                }
                1 => {
                    status = (Some(line_to.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(closure, &*points.offset(0 as libc::c_int as isize));
                    points = points.offset(1 as libc::c_int as isize);
                    current_block_11 = 7175849428784450219;
                }
                2 => {
                    status = (Some(curve_to.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        closure,
                        &*points.offset(0 as libc::c_int as isize),
                        &*points.offset(1 as libc::c_int as isize),
                        &*points.offset(2 as libc::c_int as isize),
                    );
                    points = points.offset(3 as libc::c_int as isize);
                    current_block_11 = 7175849428784450219;
                }
                3 => {
                    current_block_11 = 3378096811297028624;
                }
                _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-path-fixed.c\0" as *const u8
                                as *const libc::c_char,
                            839 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 223],
                                &[libc::c_char; 223],
                            >(
                                b"cairo_status_t _cairo_path_fixed_interpret(const cairo_path_fixed_t *, cairo_path_fixed_move_to_func_t *, cairo_path_fixed_line_to_func_t *, cairo_path_fixed_curve_to_func_t *, cairo_path_fixed_close_path_func_t *, void *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    current_block_11 = 3378096811297028624;
                }
            }
            match current_block_11 {
                3378096811297028624 => {
                    status = (Some(close_path.expect("non-null function pointer")))
                        .expect("non-null function pointer")(closure);
                }
                _ => {}
            }
            if status as u64 != 0 {
                return status;
            }
            i = i.wrapping_add(1);
        }
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &(*path).buf.base as *const cairo_path_buf_t) {
            break;
        }
    }
    if (*path).needs_move_to() as libc::c_int != 0
        && (*path).has_current_point() as libc::c_int != 0
    {
        return (Some(move_to.expect("non-null function pointer")))
            .expect("non-null function pointer")(closure, &(*path).current_point);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _append_move_to(
    mut abstract_closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut closure: *mut cairo_path_fixed_append_closure_t = abstract_closure
        as *mut cairo_path_fixed_append_closure_t;
    return _cairo_path_fixed_move_to(
        (*closure).path,
        (*point).x + (*closure).offset.x,
        (*point).y + (*closure).offset.y,
    );
}
unsafe extern "C" fn _append_line_to(
    mut abstract_closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut closure: *mut cairo_path_fixed_append_closure_t = abstract_closure
        as *mut cairo_path_fixed_append_closure_t;
    return _cairo_path_fixed_line_to(
        (*closure).path,
        (*point).x + (*closure).offset.x,
        (*point).y + (*closure).offset.y,
    );
}
unsafe extern "C" fn _append_curve_to(
    mut abstract_closure: *mut libc::c_void,
    mut p0: *const cairo_point_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) -> cairo_status_t {
    let mut closure: *mut cairo_path_fixed_append_closure_t = abstract_closure
        as *mut cairo_path_fixed_append_closure_t;
    return _cairo_path_fixed_curve_to(
        (*closure).path,
        (*p0).x + (*closure).offset.x,
        (*p0).y + (*closure).offset.y,
        (*p1).x + (*closure).offset.x,
        (*p1).y + (*closure).offset.y,
        (*p2).x + (*closure).offset.x,
        (*p2).y + (*closure).offset.y,
    );
}
unsafe extern "C" fn _append_close_path(
    mut abstract_closure: *mut libc::c_void,
) -> cairo_status_t {
    let mut closure: *mut cairo_path_fixed_append_closure_t = abstract_closure
        as *mut cairo_path_fixed_append_closure_t;
    return _cairo_path_fixed_close_path((*closure).path);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_append(
    mut path: *mut cairo_path_fixed_t,
    mut other: *const cairo_path_fixed_t,
    mut tx: cairo_fixed_t,
    mut ty: cairo_fixed_t,
) -> cairo_status_t {
    let mut closure: cairo_path_fixed_append_closure_t = cairo_path_fixed_append_closure_t {
        offset: cairo_point_t { x: 0, y: 0 },
        path: 0 as *mut cairo_path_fixed_t,
    };
    closure.path = path;
    closure.offset.x = tx;
    closure.offset.y = ty;
    return _cairo_path_fixed_interpret(
        other,
        Some(
            _append_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _append_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _append_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _append_close_path
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut closure as *mut cairo_path_fixed_append_closure_t as *mut libc::c_void,
    );
}
unsafe extern "C" fn _cairo_path_fixed_offset_and_scale(
    mut path: *mut cairo_path_fixed_t,
    mut offx: cairo_fixed_t,
    mut offy: cairo_fixed_t,
    mut scalex: cairo_fixed_t,
    mut scaley: cairo_fixed_t,
) {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    let mut i: libc::c_uint = 0;
    if scalex == (1 as libc::c_int) << 8 as libc::c_int
        && scaley == (1 as libc::c_int) << 8 as libc::c_int
    {
        _cairo_path_fixed_translate(path, offx, offy);
        return;
    }
    (*path)
        .last_move_point
        .x = _cairo_fixed_mul(scalex, (*path).last_move_point.x) + offx;
    (*path)
        .last_move_point
        .y = _cairo_fixed_mul(scaley, (*path).last_move_point.y) + offy;
    (*path).current_point.x = _cairo_fixed_mul(scalex, (*path).current_point.x) + offx;
    (*path).current_point.y = _cairo_fixed_mul(scaley, (*path).current_point.y) + offy;
    (*path).set_fill_maybe_region(1 as libc::c_int as libc::c_uint);
    buf = &mut (*path).buf.base;
    loop {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*buf).num_points {
            if scalex != (1 as libc::c_int) << 8 as libc::c_int {
                (*((*buf).points).offset(i as isize))
                    .x = _cairo_fixed_mul(
                    (*((*buf).points).offset(i as isize)).x,
                    scalex,
                );
            }
            let ref mut fresh25 = (*((*buf).points).offset(i as isize)).x;
            *fresh25 += offx;
            if scaley != (1 as libc::c_int) << 8 as libc::c_int {
                (*((*buf).points).offset(i as isize))
                    .y = _cairo_fixed_mul(
                    (*((*buf).points).offset(i as isize)).y,
                    scaley,
                );
            }
            let ref mut fresh26 = (*((*buf).points).offset(i as isize)).y;
            *fresh26 += offy;
            if (*path).fill_maybe_region() != 0 {
                (*path)
                    .set_fill_maybe_region(
                        (_cairo_fixed_is_integer((*((*buf).points).offset(i as isize)).x)
                            != 0
                            && _cairo_fixed_is_integer(
                                (*((*buf).points).offset(i as isize)).y,
                            ) != 0) as libc::c_int as libc::c_uint,
                    );
            }
            i = i.wrapping_add(1);
        }
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &mut (*path).buf.base as *mut cairo_path_buf_t) {
            break;
        }
    }
    (*path)
        .set_fill_maybe_region(
            (*path).fill_maybe_region()
                & (*path).fill_is_rectilinear() as libc::c_int as libc::c_uint,
        );
    (*path).extents.p1.x = _cairo_fixed_mul(scalex, (*path).extents.p1.x) + offx;
    (*path).extents.p2.x = _cairo_fixed_mul(scalex, (*path).extents.p2.x) + offx;
    if scalex < 0 as libc::c_int {
        let mut t: cairo_fixed_t = (*path).extents.p1.x;
        (*path).extents.p1.x = (*path).extents.p2.x;
        (*path).extents.p2.x = t;
    }
    (*path).extents.p1.y = _cairo_fixed_mul(scaley, (*path).extents.p1.y) + offy;
    (*path).extents.p2.y = _cairo_fixed_mul(scaley, (*path).extents.p2.y) + offy;
    if scaley < 0 as libc::c_int {
        let mut t_0: cairo_fixed_t = (*path).extents.p1.y;
        (*path).extents.p1.y = (*path).extents.p2.y;
        (*path).extents.p2.y = t_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_translate(
    mut path: *mut cairo_path_fixed_t,
    mut offx: cairo_fixed_t,
    mut offy: cairo_fixed_t,
) {
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    let mut i: libc::c_uint = 0;
    if offx == 0 as libc::c_int && offy == 0 as libc::c_int {
        return;
    }
    let ref mut fresh27 = (*path).last_move_point.x;
    *fresh27 += offx;
    let ref mut fresh28 = (*path).last_move_point.y;
    *fresh28 += offy;
    let ref mut fresh29 = (*path).current_point.x;
    *fresh29 += offx;
    let ref mut fresh30 = (*path).current_point.y;
    *fresh30 += offy;
    (*path).set_fill_maybe_region(1 as libc::c_int as libc::c_uint);
    buf = &mut (*path).buf.base;
    loop {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*buf).num_points {
            let ref mut fresh31 = (*((*buf).points).offset(i as isize)).x;
            *fresh31 += offx;
            let ref mut fresh32 = (*((*buf).points).offset(i as isize)).y;
            *fresh32 += offy;
            if (*path).fill_maybe_region() != 0 {
                (*path)
                    .set_fill_maybe_region(
                        (_cairo_fixed_is_integer((*((*buf).points).offset(i as isize)).x)
                            != 0
                            && _cairo_fixed_is_integer(
                                (*((*buf).points).offset(i as isize)).y,
                            ) != 0) as libc::c_int as libc::c_uint,
                    );
            }
            i = i.wrapping_add(1);
        }
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &mut (*path).buf.base as *mut cairo_path_buf_t) {
            break;
        }
    }
    (*path)
        .set_fill_maybe_region(
            (*path).fill_maybe_region()
                & (*path).fill_is_rectilinear() as libc::c_int as libc::c_uint,
        );
    let ref mut fresh33 = (*path).extents.p1.x;
    *fresh33 += offx;
    let ref mut fresh34 = (*path).extents.p1.y;
    *fresh34 += offy;
    let ref mut fresh35 = (*path).extents.p2.x;
    *fresh35 += offx;
    let ref mut fresh36 = (*path).extents.p2.y;
    *fresh36 += offy;
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_transform_point(
    mut p: *mut cairo_point_t,
    mut matrix: *const cairo_matrix_t,
) {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    dx = _cairo_fixed_to_double((*p).x);
    dy = _cairo_fixed_to_double((*p).y);
    cairo_matrix_transform_point(matrix, &mut dx, &mut dy);
    (*p).x = _cairo_fixed_from_double(dx);
    (*p).y = _cairo_fixed_from_double(dy);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_transform(
    mut path: *mut cairo_path_fixed_t,
    mut matrix: *const cairo_matrix_t,
) {
    let mut extents: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut point: cairo_point_t = cairo_point_t { x: 0, y: 0 };
    let mut buf: *mut cairo_path_buf_t = 0 as *mut cairo_path_buf_t;
    let mut i: libc::c_uint = 0;
    if (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64 {
        _cairo_path_fixed_offset_and_scale(
            path,
            _cairo_fixed_from_double((*matrix).x0),
            _cairo_fixed_from_double((*matrix).y0),
            _cairo_fixed_from_double((*matrix).xx),
            _cairo_fixed_from_double((*matrix).yy),
        );
        return;
    }
    _cairo_path_fixed_transform_point(&mut (*path).last_move_point, matrix);
    _cairo_path_fixed_transform_point(&mut (*path).current_point, matrix);
    buf = &mut (*path).buf.base;
    if (*buf).num_points == 0 as libc::c_int as libc::c_uint {
        return;
    }
    extents = (*path).extents;
    point = *((*buf).points).offset(0 as libc::c_int as isize);
    _cairo_path_fixed_transform_point(&mut point, matrix);
    _cairo_box_set(&mut (*path).extents, &mut point, &mut point);
    buf = &mut (*path).buf.base;
    loop {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*buf).num_points {
            _cairo_path_fixed_transform_point(
                &mut *((*buf).points).offset(i as isize),
                matrix,
            );
            _cairo_box_add_point(
                &mut (*path).extents,
                &mut *((*buf).points).offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
        buf = ({
            let mut mptr__: *const cairo_list_t = (*buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if !(buf != &mut (*path).buf.base as *mut cairo_path_buf_t) {
            break;
        }
    }
    if (*path).has_curve_to() != 0 {
        let mut is_tight: cairo_bool_t = 0;
        _cairo_matrix_transform_bounding_box_fixed(matrix, &mut extents, &mut is_tight);
        if is_tight == 0 {
            let mut has_extents: cairo_bool_t = 0;
            has_extents = _cairo_path_bounder_extents(path, &mut extents);
            if has_extents != 0 {} else {
                __assert_fail(
                    b"has_extents\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-path-fixed.c\0" as *const u8 as *const libc::c_char,
                    1093 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 79],
                        &[libc::c_char; 79],
                    >(
                        b"void _cairo_path_fixed_transform(cairo_path_fixed_t *, const cairo_matrix_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        (*path).extents = extents;
    }
    (*path).set_stroke_is_rectilinear(0 as libc::c_int as libc::c_uint);
    (*path).set_fill_is_rectilinear(0 as libc::c_int as libc::c_uint);
    (*path).set_fill_is_empty(0 as libc::c_int as libc::c_uint);
    (*path).set_fill_maybe_region(0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn _cpf_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpf: *mut cpf_t = closure as *mut cpf_t;
    (*cpf).current_point = *point;
    return ((*cpf).move_to).expect("non-null function pointer")((*cpf).closure, point);
}
unsafe extern "C" fn _cpf_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpf: *mut cpf_t = closure as *mut cpf_t;
    (*cpf).current_point = *point;
    return ((*cpf).line_to).expect("non-null function pointer")((*cpf).closure, point);
}
unsafe extern "C" fn _cpf_add_point(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
    mut tangent: *const cairo_slope_t,
) -> cairo_status_t {
    return _cpf_line_to(closure, point);
}
unsafe extern "C" fn _cpf_curve_to(
    mut closure: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    let mut cpf: *mut cpf_t = closure as *mut cpf_t;
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
    let mut p0: *mut cairo_point_t = &mut (*cpf).current_point;
    if _cairo_spline_init(
        &mut spline,
        Some(
            _cpf_add_point
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_slope_t,
                ) -> cairo_status_t,
        ),
        cpf as *mut libc::c_void,
        p0,
        p1,
        p2,
        p3,
    ) == 0
    {
        return _cpf_line_to(closure, p3);
    }
    (*cpf).current_point = *p3;
    return _cairo_spline_decompose(&mut spline, (*cpf).tolerance);
}
unsafe extern "C" fn _cpf_close_path(mut closure: *mut libc::c_void) -> cairo_status_t {
    let mut cpf: *mut cpf_t = closure as *mut cpf_t;
    return ((*cpf).close_path).expect("non-null function pointer")((*cpf).closure);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_interpret_flat(
    mut path: *const cairo_path_fixed_t,
    mut move_to: Option::<cairo_path_fixed_move_to_func_t>,
    mut line_to: Option::<cairo_path_fixed_line_to_func_t>,
    mut close_path: Option::<cairo_path_fixed_close_path_func_t>,
    mut closure: *mut libc::c_void,
    mut tolerance: libc::c_double,
) -> cairo_status_t {
    let mut flattener: cpf_t = cpf_t {
        tolerance: 0.,
        current_point: cairo_point_t { x: 0, y: 0 },
        move_to: None,
        line_to: None,
        close_path: None,
        closure: 0 as *mut libc::c_void,
    };
    if (*path).has_curve_to() == 0 {
        return _cairo_path_fixed_interpret(
            path,
            move_to,
            line_to,
            None,
            close_path,
            closure,
        );
    }
    flattener.tolerance = tolerance;
    flattener.move_to = move_to;
    flattener.line_to = line_to;
    flattener.close_path = close_path;
    flattener.closure = closure;
    return _cairo_path_fixed_interpret(
        path,
        Some(
            _cpf_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cpf_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cpf_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _cpf_close_path as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        ),
        &mut flattener as *mut cpf_t as *mut libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn _canonical_box(
    mut box_0: *mut cairo_box_t,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
) {
    if (*p1).x <= (*p2).x {
        (*box_0).p1.x = (*p1).x;
        (*box_0).p2.x = (*p2).x;
    } else {
        (*box_0).p1.x = (*p2).x;
        (*box_0).p2.x = (*p1).x;
    }
    if (*p1).y <= (*p2).y {
        (*box_0).p1.y = (*p1).y;
        (*box_0).p2.y = (*p2).y;
    } else {
        (*box_0).p1.y = (*p2).y;
        (*box_0).p2.y = (*p1).y;
    };
}
#[inline]
unsafe extern "C" fn _path_is_quad(mut path: *const cairo_path_fixed_t) -> cairo_bool_t {
    let mut buf: *const cairo_path_buf_t = &(*path).buf.base;
    if (*buf).num_ops < 4 as libc::c_int as libc::c_uint
        || (*buf).num_ops > 6 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if *((*buf).op).offset(0 as libc::c_int as isize) as libc::c_int
        != CAIRO_PATH_OP_MOVE_TO as libc::c_int
        || *((*buf).op).offset(1 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_LINE_TO as libc::c_int
        || *((*buf).op).offset(2 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_LINE_TO as libc::c_int
        || *((*buf).op).offset(3 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_LINE_TO as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*buf).num_ops > 4 as libc::c_int as libc::c_uint {
        if *((*buf).op).offset(4 as libc::c_int as isize) as libc::c_int
            == CAIRO_PATH_OP_LINE_TO as libc::c_int
        {
            if (*((*buf).points).offset(4 as libc::c_int as isize)).x
                != (*((*buf).points).offset(0 as libc::c_int as isize)).x
                || (*((*buf).points).offset(4 as libc::c_int as isize)).y
                    != (*((*buf).points).offset(0 as libc::c_int as isize)).y
            {
                return 0 as libc::c_int;
            }
        } else if *((*buf).op).offset(4 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_CLOSE_PATH as libc::c_int
        {
            return 0 as libc::c_int
        }
        if (*buf).num_ops == 6 as libc::c_int as libc::c_uint {
            if *((*buf).op).offset(5 as libc::c_int as isize) as libc::c_int
                != CAIRO_PATH_OP_MOVE_TO as libc::c_int
                && *((*buf).op).offset(5 as libc::c_int as isize) as libc::c_int
                    != CAIRO_PATH_OP_CLOSE_PATH as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _points_form_rect(
    mut points: *const cairo_point_t,
) -> cairo_bool_t {
    if (*points.offset(0 as libc::c_int as isize)).y
        == (*points.offset(1 as libc::c_int as isize)).y
        && (*points.offset(1 as libc::c_int as isize)).x
            == (*points.offset(2 as libc::c_int as isize)).x
        && (*points.offset(2 as libc::c_int as isize)).y
            == (*points.offset(3 as libc::c_int as isize)).y
        && (*points.offset(3 as libc::c_int as isize)).x
            == (*points.offset(0 as libc::c_int as isize)).x
    {
        return 1 as libc::c_int;
    }
    if (*points.offset(0 as libc::c_int as isize)).x
        == (*points.offset(1 as libc::c_int as isize)).x
        && (*points.offset(1 as libc::c_int as isize)).y
            == (*points.offset(2 as libc::c_int as isize)).y
        && (*points.offset(2 as libc::c_int as isize)).x
            == (*points.offset(3 as libc::c_int as isize)).x
        && (*points.offset(3 as libc::c_int as isize)).y
            == (*points.offset(0 as libc::c_int as isize)).y
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_is_box(
    mut path: *const cairo_path_fixed_t,
    mut box_0: *mut cairo_box_t,
) -> cairo_bool_t {
    let mut buf: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    if (*path).fill_is_rectilinear() == 0 {
        return 0 as libc::c_int;
    }
    if _path_is_quad(path) == 0 {
        return 0 as libc::c_int;
    }
    buf = &(*path).buf.base;
    if _points_form_rect((*buf).points) != 0 {
        _canonical_box(
            box_0,
            &mut *((*buf).points).offset(0 as libc::c_int as isize),
            &mut *((*buf).points).offset(2 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _lines_intersect_or_are_coincident(
    mut a: cairo_point_t,
    mut b: cairo_point_t,
    mut c: cairo_point_t,
    mut d: cairo_point_t,
) -> cairo_bool_t {
    let mut numerator_a: cairo_int64_t = 0;
    let mut numerator_b: cairo_int64_t = 0;
    let mut denominator: cairo_int64_t = 0;
    let mut denominator_negative: cairo_bool_t = 0;
    denominator = (d.y - c.y) as int64_t * (b.x - a.x) as libc::c_long
        - (d.x - c.x) as int64_t * (b.y - a.y) as libc::c_long;
    numerator_a = (d.x - c.x) as int64_t * (a.y - c.y) as libc::c_long
        - (d.y - c.y) as int64_t * (a.x - c.x) as libc::c_long;
    numerator_b = (b.x - a.x) as int64_t * (a.y - c.y) as libc::c_long
        - (b.y - a.y) as int64_t * (a.x - c.x) as libc::c_long;
    if denominator == 0 as libc::c_int as libc::c_long {
        if numerator_a == 0 as libc::c_int as libc::c_long
            && numerator_b == 0 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    denominator_negative = (denominator < 0 as libc::c_int as libc::c_long)
        as libc::c_int;
    if (numerator_a < 0 as libc::c_int as libc::c_long) as libc::c_int
        ^ denominator_negative != 0
    {
        return 0 as libc::c_int;
    }
    if (numerator_b < 0 as libc::c_int as libc::c_long) as libc::c_int
        ^ denominator_negative != 0
    {
        return 0 as libc::c_int;
    }
    if numerator_a == 0 as libc::c_int as libc::c_long
        || numerator_b == 0 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    if denominator_negative == 0 {
        if !(numerator_a < denominator) || !(numerator_b < denominator) {
            return 0 as libc::c_int;
        }
    } else if !(denominator < numerator_a) || !(denominator < numerator_b) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_is_simple_quad(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    let mut points: *const cairo_point_t = 0 as *const cairo_point_t;
    if _path_is_quad(path) == 0 {
        return 0 as libc::c_int;
    }
    points = (*path).buf.base.points;
    if _points_form_rect(points) != 0 {
        return 1 as libc::c_int;
    }
    if _lines_intersect_or_are_coincident(
        *points.offset(0 as libc::c_int as isize),
        *points.offset(1 as libc::c_int as isize),
        *points.offset(3 as libc::c_int as isize),
        *points.offset(2 as libc::c_int as isize),
    ) != 0
    {
        return 0 as libc::c_int;
    }
    if _lines_intersect_or_are_coincident(
        *points.offset(0 as libc::c_int as isize),
        *points.offset(3 as libc::c_int as isize),
        *points.offset(1 as libc::c_int as isize),
        *points.offset(2 as libc::c_int as isize),
    ) != 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_is_stroke_box(
    mut path: *const cairo_path_fixed_t,
    mut box_0: *mut cairo_box_t,
) -> cairo_bool_t {
    let mut buf: *const cairo_path_buf_t = &(*path).buf.base;
    if (*path).fill_is_rectilinear() == 0 {
        return 0 as libc::c_int;
    }
    if (*buf).num_ops != 5 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if *((*buf).op).offset(0 as libc::c_int as isize) as libc::c_int
        != CAIRO_PATH_OP_MOVE_TO as libc::c_int
        || *((*buf).op).offset(1 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_LINE_TO as libc::c_int
        || *((*buf).op).offset(2 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_LINE_TO as libc::c_int
        || *((*buf).op).offset(3 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_LINE_TO as libc::c_int
        || *((*buf).op).offset(4 as libc::c_int as isize) as libc::c_int
            != CAIRO_PATH_OP_CLOSE_PATH as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*((*buf).points).offset(0 as libc::c_int as isize)).y
        == (*((*buf).points).offset(1 as libc::c_int as isize)).y
        && (*((*buf).points).offset(1 as libc::c_int as isize)).x
            == (*((*buf).points).offset(2 as libc::c_int as isize)).x
        && (*((*buf).points).offset(2 as libc::c_int as isize)).y
            == (*((*buf).points).offset(3 as libc::c_int as isize)).y
        && (*((*buf).points).offset(3 as libc::c_int as isize)).x
            == (*((*buf).points).offset(0 as libc::c_int as isize)).x
    {
        _canonical_box(
            box_0,
            &mut *((*buf).points).offset(0 as libc::c_int as isize),
            &mut *((*buf).points).offset(2 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    if (*((*buf).points).offset(0 as libc::c_int as isize)).x
        == (*((*buf).points).offset(1 as libc::c_int as isize)).x
        && (*((*buf).points).offset(1 as libc::c_int as isize)).y
            == (*((*buf).points).offset(2 as libc::c_int as isize)).y
        && (*((*buf).points).offset(2 as libc::c_int as isize)).x
            == (*((*buf).points).offset(3 as libc::c_int as isize)).x
        && (*((*buf).points).offset(3 as libc::c_int as isize)).y
            == (*((*buf).points).offset(0 as libc::c_int as isize)).y
    {
        _canonical_box(
            box_0,
            &mut *((*buf).points).offset(0 as libc::c_int as isize),
            &mut *((*buf).points).offset(2 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_is_rectangle(
    mut path: *const cairo_path_fixed_t,
    mut box_0: *mut cairo_box_t,
) -> cairo_bool_t {
    let mut buf: *const cairo_path_buf_t = 0 as *const cairo_path_buf_t;
    if _cairo_path_fixed_is_box(path, box_0) == 0 {
        return 0 as libc::c_int;
    }
    buf = &(*path).buf.base;
    if (*buf).num_ops > 4 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_iter_init(
    mut iter: *mut cairo_path_fixed_iter_t,
    mut path: *const cairo_path_fixed_t,
) {
    let ref mut fresh37 = (*iter).buf;
    *fresh37 = &(*path).buf.base;
    let ref mut fresh38 = (*iter).first;
    *fresh38 = *fresh37;
    (*iter).n_op = 0 as libc::c_int as libc::c_uint;
    (*iter).n_point = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn _cairo_path_fixed_iter_next_op(
    mut iter: *mut cairo_path_fixed_iter_t,
) -> cairo_bool_t {
    let ref mut fresh39 = (*iter).n_op;
    *fresh39 = (*fresh39).wrapping_add(1);
    if *fresh39 >= (*(*iter).buf).num_ops {
        let ref mut fresh40 = (*iter).buf;
        *fresh40 = ({
            let mut mptr__: *const cairo_list_t = (*(*iter).buf).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_path_buf_t
        });
        if (*iter).buf == (*iter).first {
            let ref mut fresh41 = (*iter).buf;
            *fresh41 = 0 as *const cairo_path_buf_t;
            return 0 as libc::c_int;
        }
        (*iter).n_op = 0 as libc::c_int as libc::c_uint;
        (*iter).n_point = 0 as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_iter_is_fill_box(
    mut _iter: *mut cairo_path_fixed_iter_t,
    mut box_0: *mut cairo_box_t,
) -> cairo_bool_t {
    let mut points: [cairo_point_t; 5] = [cairo_point_t { x: 0, y: 0 }; 5];
    let mut iter: cairo_path_fixed_iter_t = cairo_path_fixed_iter_t {
        first: 0 as *const cairo_path_buf_t,
        buf: 0 as *const cairo_path_buf_t,
        n_op: 0,
        n_point: 0,
    };
    if ((*_iter).buf).is_null() {
        return 0 as libc::c_int;
    }
    iter = *_iter;
    if iter.n_op == (*iter.buf).num_ops && _cairo_path_fixed_iter_next_op(&mut iter) == 0
    {
        return 0 as libc::c_int;
    }
    if *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int
        != CAIRO_PATH_OP_MOVE_TO as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let fresh42 = iter.n_point;
    iter.n_point = (iter.n_point).wrapping_add(1);
    points[0 as libc::c_int as usize] = *((*iter.buf).points).offset(fresh42 as isize);
    if _cairo_path_fixed_iter_next_op(&mut iter) == 0 {
        return 0 as libc::c_int;
    }
    if *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int
        != CAIRO_PATH_OP_LINE_TO as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let fresh43 = iter.n_point;
    iter.n_point = (iter.n_point).wrapping_add(1);
    points[1 as libc::c_int as usize] = *((*iter.buf).points).offset(fresh43 as isize);
    if _cairo_path_fixed_iter_next_op(&mut iter) == 0 {
        return 0 as libc::c_int;
    }
    's_99: {
        match *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int {
            3 => {
                _cairo_path_fixed_iter_next_op(&mut iter);
            }
            0 => {}
            1 => {
                break 's_99;
            }
            _ => return 0 as libc::c_int,
        }
        let ref mut fresh44 = (*box_0).p2;
        *fresh44 = points[0 as libc::c_int as usize];
        (*box_0).p1 = *fresh44;
        *_iter = iter;
        return 1 as libc::c_int;
    }
    let fresh45 = iter.n_point;
    iter.n_point = (iter.n_point).wrapping_add(1);
    points[2 as libc::c_int as usize] = *((*iter.buf).points).offset(fresh45 as isize);
    if _cairo_path_fixed_iter_next_op(&mut iter) == 0 {
        return 0 as libc::c_int;
    }
    if *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int
        != CAIRO_PATH_OP_LINE_TO as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let fresh46 = iter.n_point;
    iter.n_point = (iter.n_point).wrapping_add(1);
    points[3 as libc::c_int as usize] = *((*iter.buf).points).offset(fresh46 as isize);
    if !(_cairo_path_fixed_iter_next_op(&mut iter) == 0) {
        if *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int
            == CAIRO_PATH_OP_LINE_TO as libc::c_int
        {
            let fresh47 = iter.n_point;
            iter.n_point = (iter.n_point).wrapping_add(1);
            points[4 as libc::c_int
                as usize] = *((*iter.buf).points).offset(fresh47 as isize);
            if points[4 as libc::c_int as usize].x != points[0 as libc::c_int as usize].x
                || points[4 as libc::c_int as usize].y
                    != points[0 as libc::c_int as usize].y
            {
                return 0 as libc::c_int;
            }
            _cairo_path_fixed_iter_next_op(&mut iter);
        } else if *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int
            == CAIRO_PATH_OP_CLOSE_PATH as libc::c_int
        {
            _cairo_path_fixed_iter_next_op(&mut iter);
        } else if *((*iter.buf).op).offset(iter.n_op as isize) as libc::c_int
            == CAIRO_PATH_OP_MOVE_TO as libc::c_int
        {} else {
            return 0 as libc::c_int
        }
    }
    if points[0 as libc::c_int as usize].y == points[1 as libc::c_int as usize].y
        && points[1 as libc::c_int as usize].x == points[2 as libc::c_int as usize].x
        && points[2 as libc::c_int as usize].y == points[3 as libc::c_int as usize].y
        && points[3 as libc::c_int as usize].x == points[0 as libc::c_int as usize].x
    {
        (*box_0).p1 = points[0 as libc::c_int as usize];
        (*box_0).p2 = points[2 as libc::c_int as usize];
        *_iter = iter;
        return 1 as libc::c_int;
    }
    if points[0 as libc::c_int as usize].x == points[1 as libc::c_int as usize].x
        && points[1 as libc::c_int as usize].y == points[2 as libc::c_int as usize].y
        && points[2 as libc::c_int as usize].x == points[3 as libc::c_int as usize].x
        && points[3 as libc::c_int as usize].y == points[0 as libc::c_int as usize].y
    {
        (*box_0).p1 = points[1 as libc::c_int as usize];
        (*box_0).p2 = points[3 as libc::c_int as usize];
        *_iter = iter;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_path_fixed_iter_at_end(
    mut iter: *const cairo_path_fixed_iter_t,
) -> cairo_bool_t {
    if ((*iter).buf).is_null() {
        return 1 as libc::c_int;
    }
    return ((*iter).n_op == (*(*iter).buf).num_ops) as libc::c_int;
}
