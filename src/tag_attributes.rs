use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _cairo_strtod(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_copy_element(
        array: *const cairo_array_t,
        index: libc::c_uint,
        dst: *mut libc::c_void,
    );
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn _cairo_tag_error(fmt: *const libc::c_char, _: ...) -> cairo_status_t;
}
pub type cairo_bool_t = libc::c_int;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
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
pub type cairo_int_status_t = _cairo_int_status;
pub type _cairo_int_status = libc::c_uint;
pub const CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN: _cairo_int_status = 105;
pub const CAIRO_INT_STATUS_IMAGE_FALLBACK: _cairo_int_status = 104;
pub const CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY: _cairo_int_status = 103;
pub const CAIRO_INT_STATUS_NOTHING_TO_DO: _cairo_int_status = 102;
pub const CAIRO_INT_STATUS_DEGENERATE: _cairo_int_status = 101;
pub const CAIRO_INT_STATUS_UNSUPPORTED: _cairo_int_status = 100;
pub const CAIRO_INT_STATUS_LAST_STATUS: _cairo_int_status = 44;
pub const CAIRO_INT_STATUS_DWRITE_ERROR: _cairo_int_status = 43;
pub const CAIRO_INT_STATUS_TAG_ERROR: _cairo_int_status = 42;
pub const CAIRO_INT_STATUS_WIN32_GDI_ERROR: _cairo_int_status = 41;
pub const CAIRO_INT_STATUS_FREETYPE_ERROR: _cairo_int_status = 40;
pub const CAIRO_INT_STATUS_PNG_ERROR: _cairo_int_status = 39;
pub const CAIRO_INT_STATUS_JBIG2_GLOBAL_MISSING: _cairo_int_status = 38;
pub const CAIRO_INT_STATUS_DEVICE_FINISHED: _cairo_int_status = 37;
pub const CAIRO_INT_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_int_status = 36;
pub const CAIRO_INT_STATUS_DEVICE_ERROR: _cairo_int_status = 35;
pub const CAIRO_INT_STATUS_DEVICE_TYPE_MISMATCH: _cairo_int_status = 34;
pub const CAIRO_INT_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_int_status = 33;
pub const CAIRO_INT_STATUS_INVALID_SIZE: _cairo_int_status = 32;
pub const CAIRO_INT_STATUS_INVALID_WEIGHT: _cairo_int_status = 31;
pub const CAIRO_INT_STATUS_INVALID_SLANT: _cairo_int_status = 30;
pub const CAIRO_INT_STATUS_INVALID_CLUSTERS: _cairo_int_status = 29;
pub const CAIRO_INT_STATUS_NEGATIVE_COUNT: _cairo_int_status = 28;
pub const CAIRO_INT_STATUS_USER_FONT_ERROR: _cairo_int_status = 27;
pub const CAIRO_INT_STATUS_USER_FONT_IMMUTABLE: _cairo_int_status = 26;
pub const CAIRO_INT_STATUS_FONT_TYPE_MISMATCH: _cairo_int_status = 25;
pub const CAIRO_INT_STATUS_INVALID_STRIDE: _cairo_int_status = 24;
pub const CAIRO_INT_STATUS_TEMP_FILE_ERROR: _cairo_int_status = 23;
pub const CAIRO_INT_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_int_status = 22;
pub const CAIRO_INT_STATUS_INVALID_INDEX: _cairo_int_status = 21;
pub const CAIRO_INT_STATUS_INVALID_DSC_COMMENT: _cairo_int_status = 20;
pub const CAIRO_INT_STATUS_INVALID_DASH: _cairo_int_status = 19;
pub const CAIRO_INT_STATUS_FILE_NOT_FOUND: _cairo_int_status = 18;
pub const CAIRO_INT_STATUS_INVALID_VISUAL: _cairo_int_status = 17;
pub const CAIRO_INT_STATUS_INVALID_FORMAT: _cairo_int_status = 16;
pub const CAIRO_INT_STATUS_INVALID_CONTENT: _cairo_int_status = 15;
pub const CAIRO_INT_STATUS_PATTERN_TYPE_MISMATCH: _cairo_int_status = 14;
pub const CAIRO_INT_STATUS_SURFACE_TYPE_MISMATCH: _cairo_int_status = 13;
pub const CAIRO_INT_STATUS_SURFACE_FINISHED: _cairo_int_status = 12;
pub const CAIRO_INT_STATUS_WRITE_ERROR: _cairo_int_status = 11;
pub const CAIRO_INT_STATUS_READ_ERROR: _cairo_int_status = 10;
pub const CAIRO_INT_STATUS_INVALID_PATH_DATA: _cairo_int_status = 9;
pub const CAIRO_INT_STATUS_INVALID_STRING: _cairo_int_status = 8;
pub const CAIRO_INT_STATUS_NULL_POINTER: _cairo_int_status = 7;
pub const CAIRO_INT_STATUS_INVALID_STATUS: _cairo_int_status = 6;
pub const CAIRO_INT_STATUS_INVALID_MATRIX: _cairo_int_status = 5;
pub const CAIRO_INT_STATUS_NO_CURRENT_POINT: _cairo_int_status = 4;
pub const CAIRO_INT_STATUS_INVALID_POP_GROUP: _cairo_int_status = 3;
pub const CAIRO_INT_STATUS_INVALID_RESTORE: _cairo_int_status = 2;
pub const CAIRO_INT_STATUS_NO_MEMORY: _cairo_int_status = 1;
pub const CAIRO_INT_STATUS_SUCCESS: _cairo_int_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_box_double {
    pub p1: cairo_point_double_t,
    pub p2: cairo_point_double_t,
}
pub type cairo_box_double_t = _cairo_box_double;
pub type cairo_tag_link_type_t = libc::c_uint;
pub const TAG_LINK_FILE: cairo_tag_link_type_t = 4;
pub const TAG_LINK_URI: cairo_tag_link_type_t = 3;
pub const TAG_LINK_DEST: cairo_tag_link_type_t = 2;
pub const TAG_LINK_EMPTY: cairo_tag_link_type_t = 1;
pub const TAG_LINK_INVALID: cairo_tag_link_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_link_attrs {
    pub link_type: cairo_tag_link_type_t,
    pub rects: cairo_array_t,
    pub dest: *mut libc::c_char,
    pub uri: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub page: libc::c_int,
    pub has_pos: cairo_bool_t,
    pub pos: cairo_point_double_t,
}
pub type cairo_link_attrs_t = _cairo_link_attrs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_dest_attrs {
    pub name: *mut libc::c_char,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub x_valid: cairo_bool_t,
    pub y_valid: cairo_bool_t,
    pub internal: cairo_bool_t,
}
pub type cairo_dest_attrs_t = _cairo_dest_attrs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ccitt_params {
    pub columns: libc::c_int,
    pub rows: libc::c_int,
    pub k: libc::c_int,
    pub end_of_line: cairo_bool_t,
    pub encoded_byte_align: cairo_bool_t,
    pub end_of_block: cairo_bool_t,
    pub black_is_1: cairo_bool_t,
    pub damaged_rows_before_error: libc::c_int,
}
pub type cairo_ccitt_params_t = _cairo_ccitt_params;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_eps_params {
    pub bbox: cairo_box_double_t,
}
pub type cairo_eps_params_t = _cairo_eps_params;
pub type attribute_t = _attribute;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _attribute {
    pub name: *mut libc::c_char,
    pub type_0: attribute_type_t,
    pub array_len: libc::c_int,
    pub scalar: attrib_val_t,
    pub array: cairo_array_t,
    pub link: cairo_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union attrib_val_t {
    pub b: cairo_bool_t,
    pub i: libc::c_int,
    pub f: libc::c_double,
    pub s: *mut libc::c_char,
}
pub type attribute_type_t = libc::c_uint;
pub const ATTRIBUTE_STRING: attribute_type_t = 3;
pub const ATTRIBUTE_FLOAT: attribute_type_t = 2;
pub const ATTRIBUTE_INT: attribute_type_t = 1;
pub const ATTRIBUTE_BOOL: attribute_type_t = 0;
pub type attribute_spec_t = _attribute_spec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _attribute_spec {
    pub name: *const libc::c_char,
    pub type_0: attribute_type_t,
    pub array_size: libc::c_int,
}
#[inline]
unsafe extern "C" fn _cairo_isalpha(mut c: libc::c_int) -> libc::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_isdigit(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_isspace(mut c: libc::c_int) -> libc::c_int {
    return (c == 0x20 as libc::c_int
        || c >= 0x9 as libc::c_int && c <= 0xd as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh0 = (*entry).next;
    *fresh0 = entry;
    let ref mut fresh1 = (*entry).prev;
    *fresh1 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh2 = (*next).prev;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).next;
    *fresh3 = next;
    let ref mut fresh4 = (*entry).prev;
    *fresh4 = prev;
    let ref mut fresh5 = (*prev).next;
    *fresh5 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add_tail(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, (*head).prev, head);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh6 = (*next).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = next;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn cairo_list_del(mut entry: *mut cairo_list_t) {
    _cairo_list_del(entry);
    cairo_list_init(entry);
}
static mut _dest_attrib_spec: [attribute_spec_t; 5] = [
    {
        let mut init = _attribute_spec {
            name: b"name\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_STRING,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"x\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_FLOAT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"y\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_FLOAT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"internal\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: 0 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
];
static mut _link_attrib_spec: [attribute_spec_t; 7] = [
    {
        let mut init = _attribute_spec {
            name: b"rect\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_FLOAT,
            array_size: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"dest\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_STRING,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"uri\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_STRING,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"file\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_STRING,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"page\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_INT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"pos\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_FLOAT,
            array_size: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: 0 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
];
static mut _ccitt_params_spec: [attribute_spec_t; 9] = [
    {
        let mut init = _attribute_spec {
            name: b"Columns\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_INT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"Rows\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_INT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"K\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_INT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"EndOfLine\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"EncodedByteAlign\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"EndOfBlock\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"BlackIs1\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: b"DamagedRowsBeforeError\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_INT,
            array_size: 0,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: 0 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
];
static mut _eps_params_spec: [attribute_spec_t; 2] = [
    {
        let mut init = _attribute_spec {
            name: b"bbox\0" as *const u8 as *const libc::c_char,
            type_0: ATTRIBUTE_FLOAT,
            array_size: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = _attribute_spec {
            name: 0 as *const libc::c_char,
            type_0: ATTRIBUTE_BOOL,
            array_size: 0,
        };
        init
    },
];
unsafe extern "C" fn skip_space(mut p: *const libc::c_char) -> *const libc::c_char {
    while _cairo_isspace(*p as libc::c_int) != 0 {
        p = p.offset(1);
    }
    return p;
}
unsafe extern "C" fn parse_bool(
    mut p: *const libc::c_char,
    mut b: *mut cairo_bool_t,
) -> *const libc::c_char {
    if *p as libc::c_int == '1' as i32 {
        *b = 1 as libc::c_int;
        return p.offset(1 as libc::c_int as isize);
    } else {
        if *p as libc::c_int == '0' as i32 {
            *b = 0 as libc::c_int;
            return p.offset(1 as libc::c_int as isize);
        } else {
            if strcmp(p, b"true\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                *b = 1 as libc::c_int;
                return p.offset(4 as libc::c_int as isize);
            } else {
                if strcmp(p, b"false\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    *b = 0 as libc::c_int;
                    return p.offset(5 as libc::c_int as isize);
                }
            }
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn parse_int(
    mut p: *const libc::c_char,
    mut i: *mut libc::c_int,
) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    if sscanf(
        p,
        b"%d%n\0" as *const u8 as *const libc::c_char,
        i,
        &mut n as *mut libc::c_int,
    ) > 0 as libc::c_int
    {
        return p.offset(n as isize);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn parse_float(
    mut p: *const libc::c_char,
    mut d: *mut libc::c_double,
) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    let mut start: *const libc::c_char = p;
    let mut has_decimal_point: cairo_bool_t = 0 as libc::c_int;
    while *p != 0 {
        if *p as libc::c_int == '.' as i32 || *p as libc::c_int == ']' as i32
            || _cairo_isspace(*p as libc::c_int) != 0
        {
            break;
        }
        p = p.offset(1);
    }
    if *p as libc::c_int == '.' as i32 {
        has_decimal_point = 1 as libc::c_int;
    }
    if has_decimal_point != 0 {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        *d = _cairo_strtod(start, &mut end);
        if !end.is_null() && end != start as *mut libc::c_char {
            return end;
        }
    } else if sscanf(
        start,
        b"%lf%n\0" as *const u8 as *const libc::c_char,
        d,
        &mut n as *mut libc::c_int,
    ) > 0 as libc::c_int
    {
        return start.offset(n as isize)
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn decode_string(
    mut p: *const libc::c_char,
    mut len: *mut libc::c_int,
    mut s: *mut libc::c_char,
) -> *const libc::c_char {
    if *p as libc::c_int != '\'' as i32 {
        return 0 as *const libc::c_char;
    }
    p = p.offset(1);
    if *p == 0 {
        return 0 as *const libc::c_char;
    }
    *len = 0 as libc::c_int;
    while *p != 0 {
        if *p as libc::c_int == '\\' as i32 {
            p = p.offset(1);
            if *p != 0 {
                if !s.is_null() {
                    let fresh8 = s;
                    s = s.offset(1);
                    *fresh8 = *p;
                }
                p = p.offset(1);
                *len += 1;
            }
        } else if *p as libc::c_int == '\'' as i32 {
            return p.offset(1 as libc::c_int as isize)
        } else {
            if !s.is_null() {
                let fresh9 = s;
                s = s.offset(1);
                *fresh9 = *p;
            }
            p = p.offset(1);
            *len += 1;
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn parse_string(
    mut p: *const libc::c_char,
    mut s: *mut *mut libc::c_char,
) -> *const libc::c_char {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    end = decode_string(p, &mut len, 0 as *mut libc::c_char);
    if end.is_null() {
        return 0 as *const libc::c_char;
    }
    *s = (if len + 1 as libc::c_int != 0 as libc::c_int {
        malloc((len + 1 as libc::c_int) as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    decode_string(p, &mut len, *s);
    *(*s).offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return end;
}
unsafe extern "C" fn parse_scalar(
    mut p: *const libc::c_char,
    mut type_0: attribute_type_t,
    mut scalar: *mut attrib_val_t,
) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        0 => return parse_bool(p, &mut (*scalar).b),
        1 => return parse_int(p, &mut (*scalar).i),
        2 => return parse_float(p, &mut (*scalar).f),
        3 => return parse_string(p, &mut (*scalar).s),
        _ => {}
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn parse_array(
    mut attributes: *const libc::c_char,
    mut p: *const libc::c_char,
    mut type_0: attribute_type_t,
    mut array: *mut cairo_array_t,
    mut end: *mut *const libc::c_char,
) -> cairo_int_status_t {
    let mut val: attrib_val_t = attrib_val_t { b: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    p = skip_space(p);
    if !(*p == 0) {
        let fresh10 = p;
        p = p.offset(1);
        if !(*fresh10 as libc::c_int != '[' as i32) {
            loop {
                p = skip_space(p);
                if *p == 0 {
                    break;
                }
                if *p as libc::c_int == ']' as i32 {
                    *end = p.offset(1 as libc::c_int as isize);
                    return CAIRO_INT_STATUS_SUCCESS;
                }
                p = parse_scalar(p, type_0, &mut val);
                if p.is_null() {
                    break;
                }
                status = _cairo_array_append(
                    array,
                    &mut val as *mut attrib_val_t as *const libc::c_void,
                ) as cairo_int_status_t;
                if status as u64 != 0 {
                    return status;
                }
            }
        }
    }
    return _cairo_tag_error(
        b"while parsing attributes: \"%s\". Error parsing array\0" as *const u8
            as *const libc::c_char,
        attributes,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn parse_name(
    mut attributes: *const libc::c_char,
    mut p: *const libc::c_char,
    mut end: *mut *const libc::c_char,
    mut s: *mut *mut libc::c_char,
) -> cairo_int_status_t {
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if _cairo_isalpha(*p as libc::c_int) == 0 {
        return _cairo_tag_error(
            b"while parsing attributes: \"%s\". Error parsing name. \"%s\" does not start with an alphabetic character\0"
                as *const u8 as *const libc::c_char,
            attributes,
            p,
        ) as cairo_int_status_t;
    }
    p2 = p;
    while _cairo_isalpha(*p2 as libc::c_int) != 0
        || _cairo_isdigit(*p2 as libc::c_int) != 0
    {
        p2 = p2.offset(1);
    }
    len = p2.offset_from(p) as libc::c_long as libc::c_int;
    name = (if len + 1 as libc::c_int != 0 as libc::c_int {
        malloc((len + 1 as libc::c_int) as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    if name.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    memcpy(name as *mut libc::c_void, p as *const libc::c_void, len as libc::c_ulong);
    *name.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    *s = name;
    *end = p2;
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn parse_attributes(
    mut attributes: *const libc::c_char,
    mut attrib_def: *mut attribute_spec_t,
    mut list: *mut cairo_list_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut def: *mut attribute_spec_t = 0 as *mut attribute_spec_t;
    let mut attrib: *mut attribute_t = 0 as *mut attribute_t;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut p: *const libc::c_char = attributes;
    if p.is_null() {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    loop {
        if !(*p != 0) {
            current_block = 1847472278776910194;
            break;
        }
        p = skip_space(p);
        if *p == 0 {
            current_block = 1847472278776910194;
            break;
        }
        status = parse_name(attributes, p, &mut p, &mut name);
        if status as u64 != 0 {
            return status;
        }
        def = attrib_def;
        while !((*def).name).is_null() {
            if strcmp(name, (*def).name) == 0 as libc::c_int {
                break;
            }
            def = def.offset(1);
        }
        if ((*def).name).is_null() {
            status = _cairo_tag_error(
                b"while parsing attributes: \"%s\". Unknown attribute name \"%s\"\0"
                    as *const u8 as *const libc::c_char,
                attributes,
                name,
            ) as cairo_int_status_t;
            current_block = 548054451518336535;
            break;
        } else {
            attrib = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<attribute_t>() as libc::c_ulong,
            ) as *mut attribute_t;
            if attrib.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
                current_block = 548054451518336535;
                break;
            } else {
                let ref mut fresh11 = (*attrib).name;
                *fresh11 = name;
                (*attrib).type_0 = (*def).type_0;
                _cairo_array_init(
                    &mut (*attrib).array,
                    ::std::mem::size_of::<attrib_val_t>() as libc::c_ulong
                        as libc::c_uint,
                );
                p = skip_space(p);
                if (*def).type_0 as libc::c_uint
                    == ATTRIBUTE_BOOL as libc::c_int as libc::c_uint
                    && *p as libc::c_int != '=' as i32
                {
                    (*attrib).scalar.b = 1 as libc::c_int;
                } else {
                    let fresh12 = p;
                    p = p.offset(1);
                    if *fresh12 as libc::c_int != '=' as i32 {
                        status = _cairo_tag_error(
                            b"while parsing attributes: \"%s\". Expected '=' after \"%s\"\0"
                                as *const u8 as *const libc::c_char,
                            attributes,
                            name,
                        ) as cairo_int_status_t;
                        current_block = 18029475797723969734;
                        break;
                    } else if (*def).array_size == 0 as libc::c_int {
                        let mut s: *const libc::c_char = p;
                        p = parse_scalar(p, (*def).type_0, &mut (*attrib).scalar);
                        if p.is_null() {
                            status = _cairo_tag_error(
                                b"while parsing attributes: \"%s\". Error parsing \"%s\"\0"
                                    as *const u8 as *const libc::c_char,
                                attributes,
                                s,
                            ) as cairo_int_status_t;
                            current_block = 18029475797723969734;
                            break;
                        } else {
                            (*attrib).array_len = 0 as libc::c_int;
                        }
                    } else {
                        status = parse_array(
                            attributes,
                            p,
                            (*def).type_0,
                            &mut (*attrib).array,
                            &mut p,
                        );
                        if status as u64 != 0 {
                            current_block = 18029475797723969734;
                            break;
                        }
                        (*attrib)
                            .array_len = _cairo_array_num_elements(&mut (*attrib).array)
                            as libc::c_int;
                        if (*def).array_size > 0 as libc::c_int
                            && (*attrib).array_len != (*def).array_size
                        {
                            status = _cairo_tag_error(
                                b"while parsing attributes: \"%s\". Expected %d elements in array. Found %d\0"
                                    as *const u8 as *const libc::c_char,
                                attributes,
                                (*def).array_size,
                                (*attrib).array_len,
                            ) as cairo_int_status_t;
                            current_block = 18029475797723969734;
                            break;
                        }
                    }
                }
                cairo_list_add_tail(&mut (*attrib).link, list);
            }
        }
    }
    match current_block {
        18029475797723969734 => {
            _cairo_array_fini(&mut (*attrib).array);
            if (*attrib).type_0 as libc::c_uint
                == ATTRIBUTE_STRING as libc::c_int as libc::c_uint
            {
                free((*attrib).scalar.s as *mut libc::c_void);
            }
            free(attrib as *mut libc::c_void);
        }
        1847472278776910194 => return CAIRO_INT_STATUS_SUCCESS,
        _ => {}
    }
    free(name as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn free_attributes_list(mut list: *mut cairo_list_t) {
    let mut attr: *mut attribute_t = 0 as *mut attribute_t;
    let mut next: *mut attribute_t = 0 as *mut attribute_t;
    attr = ({
        let mut mptr__: *const cairo_list_t = (*list).next;
        (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
            as *mut attribute_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*attr).link.next;
        (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
            as *mut attribute_t
    });
    while &mut (*attr).link as *mut cairo_list_t != list {
        cairo_list_del(&mut (*attr).link);
        free((*attr).name as *mut libc::c_void);
        _cairo_array_fini(&mut (*attr).array);
        if (*attr).type_0 as libc::c_uint
            == ATTRIBUTE_STRING as libc::c_int as libc::c_uint
        {
            free((*attr).scalar.s as *mut libc::c_void);
        }
        free(attr as *mut libc::c_void);
        attr = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut attribute_t
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_parse_link_attributes(
    mut attributes: *const libc::c_char,
    mut link_attrs: *mut cairo_link_attrs_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut list: cairo_list_t = cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut attr: *mut attribute_t = 0 as *mut attribute_t;
    let mut val: attrib_val_t = attrib_val_t { b: 0 };
    let mut has_rect: cairo_bool_t = 0 as libc::c_int;
    let mut invalid_combination: cairo_bool_t = 0 as libc::c_int;
    cairo_list_init(&mut list);
    status = parse_attributes(attributes, _link_attrib_spec.as_mut_ptr(), &mut list);
    if status as u64 != 0 {
        return status;
    }
    memset(
        link_attrs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_link_attrs_t>() as libc::c_ulong,
    );
    _cairo_array_init(
        &mut (*link_attrs).rects,
        ::std::mem::size_of::<cairo_rectangle_t>() as libc::c_ulong as libc::c_uint,
    );
    attr = ({
        let mut mptr__: *const cairo_list_t = list.next;
        (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
            as *mut attribute_t
    });
    's_45: loop {
        if !(&mut (*attr).link as *mut cairo_list_t != &mut list as *mut cairo_list_t) {
            current_block = 10150597327160359210;
            break;
        }
        if strcmp((*attr).name, b"dest\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let ref mut fresh13 = (*link_attrs).dest;
            *fresh13 = strdup((*attr).scalar.s);
        } else if strcmp((*attr).name, b"page\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*link_attrs).page = (*attr).scalar.i;
            if (*link_attrs).page < 1 as libc::c_int {
                status = _cairo_tag_error(
                    b"Link attributes: \"%s\" page must be >= 1\0" as *const u8
                        as *const libc::c_char,
                    attributes,
                ) as cairo_int_status_t;
                current_block = 54277198025380432;
                break;
            }
        } else if strcmp((*attr).name, b"pos\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            _cairo_array_copy_element(
                &mut (*attr).array,
                0 as libc::c_int as libc::c_uint,
                &mut val as *mut attrib_val_t as *mut libc::c_void,
            );
            (*link_attrs).pos.x = val.f;
            _cairo_array_copy_element(
                &mut (*attr).array,
                1 as libc::c_int as libc::c_uint,
                &mut val as *mut attrib_val_t as *mut libc::c_void,
            );
            (*link_attrs).pos.y = val.f;
            (*link_attrs).has_pos = 1 as libc::c_int;
        } else if strcmp((*attr).name, b"uri\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let ref mut fresh14 = (*link_attrs).uri;
            *fresh14 = strdup((*attr).scalar.s);
        } else if strcmp((*attr).name, b"file\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let ref mut fresh15 = (*link_attrs).file;
            *fresh15 = strdup((*attr).scalar.s);
        } else if strcmp((*attr).name, b"rect\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let mut rect: cairo_rectangle_t = cairo_rectangle_t {
                x: 0.,
                y: 0.,
                width: 0.,
                height: 0.,
            };
            let mut i: libc::c_int = 0;
            let mut num_elem: libc::c_int = _cairo_array_num_elements(&mut (*attr).array)
                as libc::c_int;
            if num_elem == 0 as libc::c_int
                || num_elem % 4 as libc::c_int != 0 as libc::c_int
            {
                status = _cairo_tag_error(
                    b"Link attributes: \"%s\" rect array size must be multiple of 4\0"
                        as *const u8 as *const libc::c_char,
                    attributes,
                ) as cairo_int_status_t;
                current_block = 54277198025380432;
                break;
            } else {
                i = 0 as libc::c_int;
                while i < num_elem {
                    _cairo_array_copy_element(
                        &mut (*attr).array,
                        i as libc::c_uint,
                        &mut val as *mut attrib_val_t as *mut libc::c_void,
                    );
                    rect.x = val.f;
                    _cairo_array_copy_element(
                        &mut (*attr).array,
                        (i + 1 as libc::c_int) as libc::c_uint,
                        &mut val as *mut attrib_val_t as *mut libc::c_void,
                    );
                    rect.y = val.f;
                    _cairo_array_copy_element(
                        &mut (*attr).array,
                        (i + 2 as libc::c_int) as libc::c_uint,
                        &mut val as *mut attrib_val_t as *mut libc::c_void,
                    );
                    rect.width = val.f;
                    _cairo_array_copy_element(
                        &mut (*attr).array,
                        (i + 3 as libc::c_int) as libc::c_uint,
                        &mut val as *mut attrib_val_t as *mut libc::c_void,
                    );
                    rect.height = val.f;
                    status = _cairo_array_append(
                        &mut (*link_attrs).rects,
                        &mut rect as *mut cairo_rectangle_t as *const libc::c_void,
                    ) as cairo_int_status_t;
                    if status as u64 != 0 {
                        current_block = 54277198025380432;
                        break 's_45;
                    }
                    i += 4 as libc::c_int;
                }
                has_rect = 1 as libc::c_int;
            }
        }
        attr = ({
            let mut mptr__: *const cairo_list_t = (*attr).link.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut attribute_t
        });
    }
    match current_block {
        10150597327160359210 => {
            if !((*link_attrs).uri).is_null() {
                (*link_attrs).link_type = TAG_LINK_URI;
                if !((*link_attrs).dest).is_null() || (*link_attrs).page != 0
                    || (*link_attrs).has_pos != 0 || !((*link_attrs).file).is_null()
                {
                    invalid_combination = 1 as libc::c_int;
                }
            } else if !((*link_attrs).file).is_null() {
                (*link_attrs).link_type = TAG_LINK_FILE;
                if !((*link_attrs).uri).is_null() {
                    invalid_combination = 1 as libc::c_int;
                } else if !((*link_attrs).dest).is_null()
                    && ((*link_attrs).page != 0 || (*link_attrs).has_pos != 0)
                {
                    invalid_combination = 1 as libc::c_int;
                }
            } else if !((*link_attrs).dest).is_null() {
                (*link_attrs).link_type = TAG_LINK_DEST;
                if !((*link_attrs).uri).is_null() || (*link_attrs).page != 0
                    || (*link_attrs).has_pos != 0
                {
                    invalid_combination = 1 as libc::c_int;
                }
            } else if (*link_attrs).page != 0 {
                (*link_attrs).link_type = TAG_LINK_DEST;
                if !((*link_attrs).uri).is_null() || !((*link_attrs).dest).is_null() {
                    invalid_combination = 1 as libc::c_int;
                }
            } else {
                (*link_attrs).link_type = TAG_LINK_EMPTY;
                if (*link_attrs).has_pos != 0 {
                    invalid_combination = 1 as libc::c_int;
                }
            }
            if invalid_combination != 0 {
                status = _cairo_tag_error(
                    b"Link attributes: \"%s\" invalid combination of attributes\0"
                        as *const u8 as *const libc::c_char,
                    attributes,
                ) as cairo_int_status_t;
            }
        }
        _ => {}
    }
    free_attributes_list(&mut list);
    if status as u64 != 0 {
        free((*link_attrs).dest as *mut libc::c_void);
        free((*link_attrs).uri as *mut libc::c_void);
        free((*link_attrs).file as *mut libc::c_void);
        _cairo_array_fini(&mut (*link_attrs).rects);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_parse_dest_attributes(
    mut attributes: *const libc::c_char,
    mut dest_attrs: *mut cairo_dest_attrs_t,
) -> cairo_int_status_t {
    let mut list: cairo_list_t = cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut attr: *mut attribute_t = 0 as *mut attribute_t;
    memset(
        dest_attrs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cairo_dest_attrs_t>() as libc::c_ulong,
    );
    cairo_list_init(&mut list);
    status = parse_attributes(attributes, _dest_attrib_spec.as_mut_ptr(), &mut list);
    if !(status as u64 != 0) {
        attr = ({
            let mut mptr__: *const cairo_list_t = list.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut attribute_t
        });
        while &mut (*attr).link as *mut cairo_list_t != &mut list as *mut cairo_list_t {
            if strcmp((*attr).name, b"name\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let ref mut fresh16 = (*dest_attrs).name;
                *fresh16 = strdup((*attr).scalar.s);
            } else if strcmp((*attr).name, b"x\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*dest_attrs).x = (*attr).scalar.f;
                (*dest_attrs).x_valid = 1 as libc::c_int;
            } else if strcmp((*attr).name, b"y\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*dest_attrs).y = (*attr).scalar.f;
                (*dest_attrs).y_valid = 1 as libc::c_int;
            } else if strcmp(
                (*attr).name,
                b"internal\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*dest_attrs).internal = (*attr).scalar.b;
            }
            attr = ({
                let mut mptr__: *const cairo_list_t = (*attr).link.next;
                (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                    as *mut attribute_t
            });
        }
        if ((*dest_attrs).name).is_null() {
            status = _cairo_tag_error(
                b"Destination attributes: \"%s\" missing name attribute\0" as *const u8
                    as *const libc::c_char,
                attributes,
            ) as cairo_int_status_t;
        }
    }
    free_attributes_list(&mut list);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_parse_ccitt_params(
    mut attributes: *const libc::c_char,
    mut ccitt_params: *mut cairo_ccitt_params_t,
) -> cairo_int_status_t {
    let mut list: cairo_list_t = cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut attr: *mut attribute_t = 0 as *mut attribute_t;
    (*ccitt_params).columns = -(1 as libc::c_int);
    (*ccitt_params).rows = -(1 as libc::c_int);
    (*ccitt_params).k = 0 as libc::c_int;
    (*ccitt_params).end_of_line = 0 as libc::c_int;
    (*ccitt_params).encoded_byte_align = 0 as libc::c_int;
    (*ccitt_params).end_of_block = 1 as libc::c_int;
    (*ccitt_params).black_is_1 = 0 as libc::c_int;
    (*ccitt_params).damaged_rows_before_error = 0 as libc::c_int;
    cairo_list_init(&mut list);
    status = parse_attributes(attributes, _ccitt_params_spec.as_mut_ptr(), &mut list);
    if !(status as u64 != 0) {
        attr = ({
            let mut mptr__: *const cairo_list_t = list.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut attribute_t
        });
        while &mut (*attr).link as *mut cairo_list_t != &mut list as *mut cairo_list_t {
            if strcmp((*attr).name, b"Columns\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*ccitt_params).columns = (*attr).scalar.i;
            } else if strcmp((*attr).name, b"Rows\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*ccitt_params).rows = (*attr).scalar.i;
            } else if strcmp((*attr).name, b"K\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*ccitt_params).k = (*attr).scalar.i;
            } else if strcmp(
                (*attr).name,
                b"EndOfLine\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*ccitt_params).end_of_line = (*attr).scalar.b;
            } else if strcmp(
                (*attr).name,
                b"EncodedByteAlign\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*ccitt_params).encoded_byte_align = (*attr).scalar.b;
            } else if strcmp(
                (*attr).name,
                b"EndOfBlock\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*ccitt_params).end_of_block = (*attr).scalar.b;
            } else if strcmp(
                (*attr).name,
                b"BlackIs1\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*ccitt_params).black_is_1 = (*attr).scalar.b;
            } else if strcmp(
                (*attr).name,
                b"DamagedRowsBeforeError\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*ccitt_params).damaged_rows_before_error = (*attr).scalar.b;
            }
            attr = ({
                let mut mptr__: *const cairo_list_t = (*attr).link.next;
                (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                    as *mut attribute_t
            });
        }
    }
    free_attributes_list(&mut list);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_parse_eps_params(
    mut attributes: *const libc::c_char,
    mut eps_params: *mut cairo_eps_params_t,
) -> cairo_int_status_t {
    let mut list: cairo_list_t = cairo_list_t {
        next: 0 as *mut _cairo_list,
        prev: 0 as *mut _cairo_list,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut attr: *mut attribute_t = 0 as *mut attribute_t;
    let mut val: attrib_val_t = attrib_val_t { b: 0 };
    cairo_list_init(&mut list);
    status = parse_attributes(attributes, _eps_params_spec.as_mut_ptr(), &mut list);
    if !(status as u64 != 0) {
        attr = ({
            let mut mptr__: *const cairo_list_t = list.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut attribute_t
        });
        while &mut (*attr).link as *mut cairo_list_t != &mut list as *mut cairo_list_t {
            if strcmp((*attr).name, b"bbox\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                _cairo_array_copy_element(
                    &mut (*attr).array,
                    0 as libc::c_int as libc::c_uint,
                    &mut val as *mut attrib_val_t as *mut libc::c_void,
                );
                (*eps_params).bbox.p1.x = val.f;
                _cairo_array_copy_element(
                    &mut (*attr).array,
                    1 as libc::c_int as libc::c_uint,
                    &mut val as *mut attrib_val_t as *mut libc::c_void,
                );
                (*eps_params).bbox.p1.y = val.f;
                _cairo_array_copy_element(
                    &mut (*attr).array,
                    2 as libc::c_int as libc::c_uint,
                    &mut val as *mut attrib_val_t as *mut libc::c_void,
                );
                (*eps_params).bbox.p2.x = val.f;
                _cairo_array_copy_element(
                    &mut (*attr).array,
                    3 as libc::c_int as libc::c_uint,
                    &mut val as *mut attrib_val_t as *mut libc::c_void,
                );
                (*eps_params).bbox.p2.y = val.f;
            }
            attr = ({
                let mut mptr__: *const cairo_list_t = (*attr).link.next;
                (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                    as *mut attribute_t
            });
        }
    }
    free_attributes_list(&mut list);
    return status;
}
