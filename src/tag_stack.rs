use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type cairo_bool_t = libc::c_int;
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
pub type cairo_tag_type_t = libc::c_uint;
pub const TAG_TYPE_DEST: cairo_tag_type_t = 4;
pub const TAG_TYPE_LINK: cairo_tag_type_t = 2;
pub const TAG_TYPE_STRUCTURE: cairo_tag_type_t = 1;
pub const TAG_TYPE_INVALID: cairo_tag_type_t = 0;
pub type _cairo_tag_stack_structure_type = libc::c_uint;
pub const TAG_TREE_TYPE_INVALID: _cairo_tag_stack_structure_type = 4;
pub const TAG_TREE_TYPE_NO_TAGS: _cairo_tag_stack_structure_type = 3;
pub const TAG_TREE_TYPE_LINK_ONLY: _cairo_tag_stack_structure_type = 2;
pub const TAG_TREE_TYPE_STRUCTURE: _cairo_tag_stack_structure_type = 1;
pub const TAG_TREE_TYPE_TAGGED: _cairo_tag_stack_structure_type = 0;
pub type cairo_tag_stack_structure_type_t = _cairo_tag_stack_structure_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tag_stack_elem {
    pub name: *mut libc::c_char,
    pub attributes: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub link: cairo_list_t,
}
pub type cairo_tag_stack_elem_t = _cairo_tag_stack_elem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tag_stack {
    pub list: cairo_list_t,
    pub type_0: cairo_tag_stack_structure_type_t,
    pub size: libc::c_int,
}
pub type cairo_tag_stack_t = _cairo_tag_stack;
#[inline]
unsafe extern "C" fn cairo_list_del(mut entry: *mut cairo_list_t) {
    _cairo_list_del(entry);
    cairo_list_init(entry);
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh0 = (*next).prev;
    *fresh0 = prev;
    let ref mut fresh1 = (*prev).next;
    *fresh1 = next;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh2 = (*entry).next;
    *fresh2 = entry;
    let ref mut fresh3 = (*entry).prev;
    *fresh3 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh4 = (*next).prev;
    *fresh4 = entry;
    let ref mut fresh5 = (*entry).next;
    *fresh5 = next;
    let ref mut fresh6 = (*entry).prev;
    *fresh6 = prev;
    let ref mut fresh7 = (*prev).next;
    *fresh7 = entry;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn cairo_list_add_tail(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, (*head).prev, head);
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
static mut _cairo_tag_stack_tagged_pdf_top_level_element_list: [*const libc::c_char; 6] = [
    b"Document\0" as *const u8 as *const libc::c_char,
    b"Part\0" as *const u8 as *const libc::c_char,
    b"Art\0" as *const u8 as *const libc::c_char,
    b"Sect\0" as *const u8 as *const libc::c_char,
    b"Div\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut _cairo_tag_stack_struct_pdf_list: [*const libc::c_char; 50] = [
    b"Document\0" as *const u8 as *const libc::c_char,
    b"Part\0" as *const u8 as *const libc::c_char,
    b"Art\0" as *const u8 as *const libc::c_char,
    b"Sect\0" as *const u8 as *const libc::c_char,
    b"Div\0" as *const u8 as *const libc::c_char,
    b"BlockQuote\0" as *const u8 as *const libc::c_char,
    b"Caption\0" as *const u8 as *const libc::c_char,
    b"TOC\0" as *const u8 as *const libc::c_char,
    b"TOCI\0" as *const u8 as *const libc::c_char,
    b"Index\0" as *const u8 as *const libc::c_char,
    b"NonStruct\0" as *const u8 as *const libc::c_char,
    b"Private\0" as *const u8 as *const libc::c_char,
    b"P\0" as *const u8 as *const libc::c_char,
    b"H\0" as *const u8 as *const libc::c_char,
    b"H1\0" as *const u8 as *const libc::c_char,
    b"H2\0" as *const u8 as *const libc::c_char,
    b"H3\0" as *const u8 as *const libc::c_char,
    b"H4\0" as *const u8 as *const libc::c_char,
    b"H5\0" as *const u8 as *const libc::c_char,
    b"H6\0" as *const u8 as *const libc::c_char,
    b"L\0" as *const u8 as *const libc::c_char,
    b"LI\0" as *const u8 as *const libc::c_char,
    b"Lbl\0" as *const u8 as *const libc::c_char,
    b"LBody\0" as *const u8 as *const libc::c_char,
    b"Table\0" as *const u8 as *const libc::c_char,
    b"TR\0" as *const u8 as *const libc::c_char,
    b"TH\0" as *const u8 as *const libc::c_char,
    b"TD\0" as *const u8 as *const libc::c_char,
    b"THead\0" as *const u8 as *const libc::c_char,
    b"TBody\0" as *const u8 as *const libc::c_char,
    b"TFoot\0" as *const u8 as *const libc::c_char,
    b"Span\0" as *const u8 as *const libc::c_char,
    b"Quote\0" as *const u8 as *const libc::c_char,
    b"Note\0" as *const u8 as *const libc::c_char,
    b"Reference\0" as *const u8 as *const libc::c_char,
    b"BibEntry\0" as *const u8 as *const libc::c_char,
    b"Code\0" as *const u8 as *const libc::c_char,
    b"Link\0" as *const u8 as *const libc::c_char,
    b"Annot\0" as *const u8 as *const libc::c_char,
    b"Ruby\0" as *const u8 as *const libc::c_char,
    b"Warichu\0" as *const u8 as *const libc::c_char,
    b"RB\0" as *const u8 as *const libc::c_char,
    b"RT\0" as *const u8 as *const libc::c_char,
    b"RP\0" as *const u8 as *const libc::c_char,
    b"WT\0" as *const u8 as *const libc::c_char,
    b"WP\0" as *const u8 as *const libc::c_char,
    b"Figure\0" as *const u8 as *const libc::c_char,
    b"Formula\0" as *const u8 as *const libc::c_char,
    b"Form\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut _cairo_tag_stack_cairo_tag_list: [*const libc::c_char; 2] = [
    b"cairo.dest\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_init(mut stack: *mut cairo_tag_stack_t) {
    cairo_list_init(&mut (*stack).list);
    (*stack).type_0 = TAG_TREE_TYPE_NO_TAGS;
    (*stack).size = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_fini(mut stack: *mut cairo_tag_stack_t) {
    while cairo_list_is_empty(&mut (*stack).list) == 0 {
        let mut elem: *mut cairo_tag_stack_elem_t = 0 as *mut cairo_tag_stack_elem_t;
        elem = ({
            let mut mptr__: *const cairo_list_t = (*stack).list.next;
            (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
                as *mut cairo_tag_stack_elem_t
        });
        cairo_list_del(&mut (*elem).link);
        free((*elem).name as *mut libc::c_void);
        free((*elem).attributes as *mut libc::c_void);
        free(elem as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_get_structure_type(
    mut stack: *mut cairo_tag_stack_t,
) -> cairo_tag_stack_structure_type_t {
    return (*stack).type_0;
}
unsafe extern "C" fn name_in_list(
    mut name: *const libc::c_char,
    mut list: *mut *const libc::c_char,
) -> cairo_bool_t {
    if name.is_null() {
        return 0 as libc::c_int;
    }
    while !(*list).is_null() {
        if strcmp(name, *list) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        list = list.offset(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_push(
    mut stack: *mut cairo_tag_stack_t,
    mut name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut elem: *mut cairo_tag_stack_elem_t = 0 as *mut cairo_tag_stack_elem_t;
    if name_in_list(name, _cairo_tag_stack_struct_pdf_list.as_mut_ptr()) == 0
        && name_in_list(name, _cairo_tag_stack_cairo_tag_list.as_mut_ptr()) == 0
    {
        (*stack)
            .type_0 = TAG_TYPE_INVALID as libc::c_int
            as cairo_tag_stack_structure_type_t;
        return _cairo_tag_error(
            b"Invalid tag: %s\0" as *const u8 as *const libc::c_char,
            name,
        ) as cairo_int_status_t;
    }
    if (*stack).type_0 as libc::c_uint
        == TAG_TREE_TYPE_NO_TAGS as libc::c_int as libc::c_uint
    {
        if name_in_list(
            name,
            _cairo_tag_stack_tagged_pdf_top_level_element_list.as_mut_ptr(),
        ) != 0
        {
            (*stack).type_0 = TAG_TREE_TYPE_TAGGED;
        } else if strcmp(name, b"Link\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*stack).type_0 = TAG_TREE_TYPE_LINK_ONLY;
        } else if name_in_list(name, _cairo_tag_stack_struct_pdf_list.as_mut_ptr()) != 0
        {
            (*stack).type_0 = TAG_TREE_TYPE_STRUCTURE;
        }
    } else if (*stack).type_0 as libc::c_uint
        == TAG_TREE_TYPE_LINK_ONLY as libc::c_int as libc::c_uint
        && strcmp(name, b"Link\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        && name_in_list(name, _cairo_tag_stack_struct_pdf_list.as_mut_ptr()) != 0
    {
        (*stack).type_0 = TAG_TREE_TYPE_STRUCTURE;
    }
    elem = (if ::std::mem::size_of::<cairo_tag_stack_elem_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_tag_stack_elem_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_tag_stack_elem_t;
    if elem.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh8 = (*elem).name;
    *fresh8 = strdup(name);
    if ((*elem).name).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if !attributes.is_null() {
        let ref mut fresh9 = (*elem).attributes;
        *fresh9 = strdup(attributes);
        if ((*elem).attributes).is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    } else {
        let ref mut fresh10 = (*elem).attributes;
        *fresh10 = 0 as *mut libc::c_char;
    }
    let ref mut fresh11 = (*elem).data;
    *fresh11 = 0 as *mut libc::c_void;
    cairo_list_add_tail(&mut (*elem).link, &mut (*stack).list);
    let ref mut fresh12 = (*stack).size;
    *fresh12 += 1;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_set_top_data(
    mut stack: *mut cairo_tag_stack_t,
    mut data: *mut libc::c_void,
) {
    let mut top: *mut cairo_tag_stack_elem_t = 0 as *mut cairo_tag_stack_elem_t;
    top = _cairo_tag_stack_top_elem(stack);
    if !top.is_null() {
        let ref mut fresh13 = (*top).data;
        *fresh13 = data;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_pop(
    mut stack: *mut cairo_tag_stack_t,
    mut name: *const libc::c_char,
    mut elem: *mut *mut cairo_tag_stack_elem_t,
) -> cairo_int_status_t {
    let mut top: *mut cairo_tag_stack_elem_t = 0 as *mut cairo_tag_stack_elem_t;
    top = _cairo_tag_stack_top_elem(stack);
    if top.is_null() {
        (*stack)
            .type_0 = TAG_TYPE_INVALID as libc::c_int
            as cairo_tag_stack_structure_type_t;
        return _cairo_tag_error(
            b"cairo_tag_end(\"%s\") no matching begin tag\0" as *const u8
                as *const libc::c_char,
            name,
        ) as cairo_int_status_t;
    }
    cairo_list_del(&mut (*top).link);
    let ref mut fresh14 = (*stack).size;
    *fresh14 -= 1;
    if strcmp((*top).name, name) != 0 as libc::c_int {
        (*stack)
            .type_0 = TAG_TYPE_INVALID as libc::c_int
            as cairo_tag_stack_structure_type_t;
        _cairo_tag_stack_free_elem(top);
        return _cairo_tag_error(
            b"cairo_tag_end(\"%s\") does not matching previous begin tag \"%s\"\0"
                as *const u8 as *const libc::c_char,
            name,
            (*top).name,
        ) as cairo_int_status_t;
    }
    if !elem.is_null() {
        *elem = top;
    } else {
        _cairo_tag_stack_free_elem(top);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_top_elem(
    mut stack: *mut cairo_tag_stack_t,
) -> *mut cairo_tag_stack_elem_t {
    if cairo_list_is_empty(&mut (*stack).list) != 0 {
        return 0 as *mut cairo_tag_stack_elem_t;
    }
    return ({
        let mut mptr__: *const cairo_list_t = (*stack).list.prev;
        (mptr__ as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize))
            as *mut cairo_tag_stack_elem_t
    });
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_stack_free_elem(
    mut elem: *mut cairo_tag_stack_elem_t,
) {
    free((*elem).name as *mut libc::c_void);
    free((*elem).attributes as *mut libc::c_void);
    free(elem as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_get_type(
    mut name: *const libc::c_char,
) -> cairo_tag_type_t {
    if name_in_list(name, _cairo_tag_stack_struct_pdf_list.as_mut_ptr()) == 0
        && name_in_list(name, _cairo_tag_stack_cairo_tag_list.as_mut_ptr()) == 0
    {
        return TAG_TYPE_INVALID;
    }
    if strcmp(name, b"Link\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return (TAG_TYPE_LINK as libc::c_int | TAG_TYPE_STRUCTURE as libc::c_int)
            as cairo_tag_type_t;
    }
    if strcmp(name, b"cairo.dest\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return TAG_TYPE_DEST;
    }
    return TAG_TYPE_STRUCTURE;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_tag_error(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> cairo_status_t {
    let mut ap: ::std::ffi::VaListImpl;
    if !(getenv(b"CAIRO_DEBUG_TAG\0" as *const u8 as *const libc::c_char)).is_null() {
        printf(b"TAG ERROR: \0" as *const u8 as *const libc::c_char);
        ap = args.clone();
        vprintf(fmt, ap.as_va_list());
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return _cairo_error(CAIRO_STATUS_TAG_ERROR);
}
