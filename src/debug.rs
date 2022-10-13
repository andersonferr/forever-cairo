use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _cairo_default_context_reset_static_data();
    fn _cairo_image_compositor_reset_static_data();
    fn _cairo_image_reset_static_data();
    fn _cairo_clip_reset_static_data();
    fn _cairo_pattern_reset_static_data();
    fn _cairo_scaled_font_reset_static_data();
    fn _cairo_intern_string_reset_static_data();
    fn _cairo_ft_font_reset_static_data();
    fn _cairo_toy_font_face_reset_static_data();
    fn _cairo_scaled_font_map_destroy();
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_path_fixed_interpret(
        path: *const cairo_path_fixed_t,
        move_to: Option::<cairo_path_fixed_move_to_func_t>,
        line_to: Option::<cairo_path_fixed_line_to_func_t>,
        curve_to: Option::<cairo_path_fixed_curve_to_func_t>,
        close_path: Option::<cairo_path_fixed_close_path_func_t>,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_is_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_int {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type cairo_line_t = _cairo_line;
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
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_debug_reset_static_data() {
    _cairo_scaled_font_map_destroy();
    _cairo_toy_font_face_reset_static_data();
    _cairo_ft_font_reset_static_data();
    _cairo_intern_string_reset_static_data();
    _cairo_scaled_font_reset_static_data();
    _cairo_pattern_reset_static_data();
    _cairo_clip_reset_static_data();
    _cairo_image_reset_static_data();
    _cairo_image_compositor_reset_static_data();
    _cairo_default_context_reset_static_data();
}
unsafe extern "C" fn _print_move_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    fprintf(
        closure as *mut FILE,
        b" %f %f m\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*point).x),
        _cairo_fixed_to_double((*point).y),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _print_line_to(
    mut closure: *mut libc::c_void,
    mut point: *const cairo_point_t,
) -> cairo_status_t {
    fprintf(
        closure as *mut FILE,
        b" %f %f l\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*point).x),
        _cairo_fixed_to_double((*point).y),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _print_curve_to(
    mut closure: *mut libc::c_void,
    mut p1: *const cairo_point_t,
    mut p2: *const cairo_point_t,
    mut p3: *const cairo_point_t,
) -> cairo_status_t {
    fprintf(
        closure as *mut FILE,
        b" %f %f %f %f %f %f c\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*p1).x),
        _cairo_fixed_to_double((*p1).y),
        _cairo_fixed_to_double((*p2).x),
        _cairo_fixed_to_double((*p2).y),
        _cairo_fixed_to_double((*p3).x),
        _cairo_fixed_to_double((*p3).y),
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _print_close(mut closure: *mut libc::c_void) -> cairo_status_t {
    fprintf(closure as *mut FILE, b" h\0" as *const u8 as *const libc::c_char);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_path(
    mut stream: *mut FILE,
    mut path: *const cairo_path_fixed_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    fprintf(
        stream,
        b"path: extents=(%f, %f), (%f, %f)\n\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*path).extents.p1.x),
        _cairo_fixed_to_double((*path).extents.p1.y),
        _cairo_fixed_to_double((*path).extents.p2.x),
        _cairo_fixed_to_double((*path).extents.p2.y),
    );
    status = _cairo_path_fixed_interpret(
        path,
        Some(
            _print_move_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _print_line_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(
            _print_curve_to
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const cairo_point_t,
                    *const cairo_point_t,
                    *const cairo_point_t,
                ) -> cairo_status_t,
        ),
        Some(_print_close as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t),
        stream as *mut libc::c_void,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-debug.c\0" as *const u8 as *const libc::c_char,
            258 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void _cairo_debug_print_path(FILE *, const cairo_path_fixed_t *)\0"))
                .as_ptr(),
        );
    }
    if _cairo_path_fixed_is_box(path, &mut box_0) != 0 {
        fprintf(
            stream,
            b"[box (%d, %d), (%d, %d)]\0" as *const u8 as *const libc::c_char,
            box_0.p1.x,
            box_0.p1.y,
            box_0.p2.x,
            box_0.p2.y,
        );
    }
    fprintf(stream, b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_polygon(
    mut stream: *mut FILE,
    mut polygon: *mut cairo_polygon_t,
) {
    let mut n: libc::c_int = 0;
    fprintf(
        stream,
        b"polygon: extents=(%f, %f), (%f, %f)\n\0" as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double((*polygon).extents.p1.x),
        _cairo_fixed_to_double((*polygon).extents.p1.y),
        _cairo_fixed_to_double((*polygon).extents.p2.x),
        _cairo_fixed_to_double((*polygon).extents.p2.y),
    );
    if (*polygon).num_limits != 0 {
        fprintf(
            stream,
            b"       : limit=(%f, %f), (%f, %f) x %d\n\0" as *const u8
                as *const libc::c_char,
            _cairo_fixed_to_double((*polygon).limit.p1.x),
            _cairo_fixed_to_double((*polygon).limit.p1.y),
            _cairo_fixed_to_double((*polygon).limit.p2.x),
            _cairo_fixed_to_double((*polygon).limit.p2.y),
            (*polygon).num_limits,
        );
    }
    n = 0 as libc::c_int;
    while n < (*polygon).num_edges {
        let mut edge: *mut cairo_edge_t = &mut *((*polygon).edges).offset(n as isize)
            as *mut cairo_edge_t;
        fprintf(
            stream,
            b"  [%d] = [(%f, %f), (%f, %f)], top=%f, bottom=%f, dir=%d\n\0" as *const u8
                as *const libc::c_char,
            n,
            _cairo_fixed_to_double((*edge).line.p1.x),
            _cairo_fixed_to_double((*edge).line.p1.y),
            _cairo_fixed_to_double((*edge).line.p2.x),
            _cairo_fixed_to_double((*edge).line.p2.y),
            _cairo_fixed_to_double((*edge).top),
            _cairo_fixed_to_double((*edge).bottom),
            (*edge).dir,
        );
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_matrix(
    mut file: *mut FILE,
    mut matrix: *const cairo_matrix_t,
) {
    fprintf(
        file,
        b"[%g %g %g %g %g %g]\n\0" as *const u8 as *const libc::c_char,
        (*matrix).xx,
        (*matrix).yx,
        (*matrix).xy,
        (*matrix).yy,
        (*matrix).x0,
        (*matrix).y0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_debug_print_rect(
    mut file: *mut FILE,
    mut rect: *const cairo_rectangle_int_t,
) {
    fprintf(
        file,
        b"x: %d y: %d width: %d height: %d\n\0" as *const u8 as *const libc::c_char,
        (*rect).x,
        (*rect).y,
        (*rect).width,
        (*rect).height,
    );
}
