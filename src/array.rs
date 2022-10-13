use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_user_data_slot_t {
    pub key: *const cairo_user_data_key_t,
    pub user_data: *mut libc::c_void,
    pub destroy: cairo_destroy_func_t,
}
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
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_init(
    mut array: *mut cairo_array_t,
    mut element_size: libc::c_uint,
) {
    (*array).size = 0 as libc::c_int as libc::c_uint;
    (*array).num_elements = 0 as libc::c_int as libc::c_uint;
    (*array).element_size = element_size;
    let ref mut fresh2 = (*array).elements;
    *fresh2 = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_fini(mut array: *mut cairo_array_t) {
    free((*array).elements as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_grow_by(
    mut array: *mut cairo_array_t,
    mut additional: libc::c_uint,
) -> cairo_status_t {
    let mut new_elements: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_size: libc::c_uint = (*array).size;
    let mut required_size: libc::c_uint = ((*array).num_elements)
        .wrapping_add(additional);
    let mut new_size: libc::c_uint = 0;
    if required_size > 2147483647 as libc::c_int as libc::c_uint
        || required_size < (*array).num_elements
    {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    if required_size <= old_size {
        return CAIRO_STATUS_SUCCESS;
    }
    if old_size == 0 as libc::c_int as libc::c_uint {
        new_size = 1 as libc::c_int as libc::c_uint;
    } else {
        new_size = old_size.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    while new_size < required_size {
        new_size = new_size.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    (*array).size = new_size;
    new_elements = _cairo_realloc_ab(
        (*array).elements as *mut libc::c_void,
        (*array).size as size_t,
        (*array).element_size as size_t,
    ) as *mut libc::c_char;
    if new_elements.is_null() {
        (*array).size = old_size;
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    let ref mut fresh3 = (*array).elements;
    *fresh3 = new_elements;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_truncate(
    mut array: *mut cairo_array_t,
    mut num_elements: libc::c_uint,
) {
    if num_elements < (*array).num_elements {
        (*array).num_elements = num_elements;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_index(
    mut array: *mut cairo_array_t,
    mut index: libc::c_uint,
) -> *mut libc::c_void {
    if index == 0 as libc::c_int as libc::c_uint
        && (*array).num_elements == 0 as libc::c_int as libc::c_uint
    {
        return 0 as *mut libc::c_void;
    }
    if index < (*array).num_elements {} else {
        __assert_fail(
            b"index < array->num_elements\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-array.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void *_cairo_array_index(cairo_array_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    return ((*array).elements)
        .offset(
            (index as size_t).wrapping_mul((*array).element_size as libc::c_ulong)
                as isize,
        ) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_index_const(
    mut array: *const cairo_array_t,
    mut index: libc::c_uint,
) -> *const libc::c_void {
    if index == 0 as libc::c_int as libc::c_uint
        && (*array).num_elements == 0 as libc::c_int as libc::c_uint
    {
        return 0 as *const libc::c_void;
    }
    if index < (*array).num_elements {} else {
        __assert_fail(
            b"index < array->num_elements\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-array.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"const void *_cairo_array_index_const(const cairo_array_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    return ((*array).elements)
        .offset(
            (index as size_t).wrapping_mul((*array).element_size as libc::c_ulong)
                as isize,
        ) as *const libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_copy_element(
    mut array: *const cairo_array_t,
    mut index: libc::c_uint,
    mut dst: *mut libc::c_void,
) {
    memcpy(
        dst,
        _cairo_array_index_const(array, index),
        (*array).element_size as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_append(
    mut array: *mut cairo_array_t,
    mut element: *const libc::c_void,
) -> cairo_status_t {
    return _cairo_array_append_multiple(
        array,
        element,
        1 as libc::c_int as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_append_multiple(
    mut array: *mut cairo_array_t,
    mut elements: *const libc::c_void,
    mut num_elements: libc::c_uint,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut dest: *mut libc::c_void = 0 as *mut libc::c_void;
    status = _cairo_array_allocate(array, num_elements, &mut dest);
    if status as u64 != 0 {
        return status;
    }
    memcpy(
        dest,
        elements,
        (num_elements as size_t).wrapping_mul((*array).element_size as libc::c_ulong),
    );
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_allocate(
    mut array: *mut cairo_array_t,
    mut num_elements: libc::c_uint,
    mut elements: *mut *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_array_grow_by(array, num_elements);
    if status as u64 != 0 {
        return status;
    }
    if ((*array).num_elements).wrapping_add(num_elements) <= (*array).size {} else {
        __assert_fail(
            b"array->num_elements + num_elements <= array->size\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-array.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"cairo_status_t _cairo_array_allocate(cairo_array_t *, unsigned int, void **)\0",
            ))
                .as_ptr(),
        );
    }
    *elements = ((*array).elements)
        .offset(
            ((*array).num_elements as size_t)
                .wrapping_mul((*array).element_size as libc::c_ulong) as isize,
        ) as *mut libc::c_void;
    let ref mut fresh4 = (*array).num_elements;
    *fresh4 = (*fresh4).wrapping_add(num_elements);
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_num_elements(
    mut array: *const cairo_array_t,
) -> libc::c_uint {
    return (*array).num_elements;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_size(
    mut array: *const cairo_array_t,
) -> libc::c_uint {
    return (*array).size;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_user_data_array_init(
    mut array: *mut cairo_user_data_array_t,
) {
    _cairo_array_init(
        array,
        ::std::mem::size_of::<cairo_user_data_slot_t>() as libc::c_ulong as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_user_data_array_fini(
    mut array: *mut cairo_user_data_array_t,
) {
    let mut num_slots: libc::c_uint = 0;
    num_slots = (*array).num_elements;
    if num_slots != 0 {
        let mut slots: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
        slots = _cairo_array_index(array, 0 as libc::c_int as libc::c_uint)
            as *mut cairo_user_data_slot_t;
        loop {
            let fresh5 = num_slots;
            num_slots = num_slots.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            let mut s: *mut cairo_user_data_slot_t = &mut *slots
                .offset(num_slots as isize) as *mut cairo_user_data_slot_t;
            if !((*s).user_data).is_null() && ((*s).destroy).is_some() {
                ((*s).destroy).expect("non-null function pointer")((*s).user_data);
            }
        }
    }
    _cairo_array_fini(array);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_user_data_array_get_data(
    mut array: *mut cairo_user_data_array_t,
    mut key: *const cairo_user_data_key_t,
) -> *mut libc::c_void {
    let mut i: libc::c_uint = 0;
    let mut num_slots: libc::c_uint = 0;
    let mut slots: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
    if array.is_null() {
        return 0 as *mut libc::c_void;
    }
    num_slots = (*array).num_elements;
    slots = _cairo_array_index(array, 0 as libc::c_int as libc::c_uint)
        as *mut cairo_user_data_slot_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_slots {
        if (*slots.offset(i as isize)).key == key {
            return (*slots.offset(i as isize)).user_data;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_user_data_array_set_data(
    mut array: *mut cairo_user_data_array_t,
    mut key: *const cairo_user_data_key_t,
    mut user_data: *mut libc::c_void,
    mut destroy: cairo_destroy_func_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut num_slots: libc::c_uint = 0;
    let mut slots: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
    let mut slot: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
    let mut new_slot: cairo_user_data_slot_t = cairo_user_data_slot_t {
        key: 0 as *const cairo_user_data_key_t,
        user_data: 0 as *mut libc::c_void,
        destroy: None,
    };
    if !user_data.is_null() {
        new_slot.key = key;
        new_slot.user_data = user_data;
        new_slot.destroy = destroy;
    } else {
        new_slot.key = 0 as *const cairo_user_data_key_t;
        new_slot.user_data = 0 as *mut libc::c_void;
        new_slot.destroy = None;
    }
    slot = 0 as *mut cairo_user_data_slot_t;
    num_slots = (*array).num_elements;
    slots = _cairo_array_index(array, 0 as libc::c_int as libc::c_uint)
        as *mut cairo_user_data_slot_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_slots {
        if (*slots.offset(i as isize)).key == key {
            slot = &mut *slots.offset(i as isize) as *mut cairo_user_data_slot_t;
            if ((*slot).destroy).is_some() && !((*slot).user_data).is_null() {
                ((*slot).destroy).expect("non-null function pointer")((*slot).user_data);
            }
            break;
        } else {
            if !user_data.is_null() && ((*slots.offset(i as isize)).user_data).is_null()
            {
                slot = &mut *slots.offset(i as isize) as *mut cairo_user_data_slot_t;
            }
            i = i.wrapping_add(1);
        }
    }
    if !slot.is_null() {
        *slot = new_slot;
        return CAIRO_STATUS_SUCCESS;
    }
    if user_data.is_null() {
        return CAIRO_STATUS_SUCCESS;
    }
    status = _cairo_array_append(
        array,
        &mut new_slot as *mut cairo_user_data_slot_t as *const libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_user_data_array_copy(
    mut dst: *mut cairo_user_data_array_t,
    mut src: *const cairo_user_data_array_t,
) -> cairo_status_t {
    if (*dst).num_elements != 0 as libc::c_int as libc::c_uint {
        _cairo_user_data_array_fini(dst);
        _cairo_user_data_array_init(dst);
    }
    if (*src).num_elements == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_array_append_multiple(
        dst,
        _cairo_array_index_const(src, 0 as libc::c_int as libc::c_uint),
        (*src).num_elements,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_user_data_array_foreach(
    mut array: *mut cairo_user_data_array_t,
    mut func: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> (),
    >,
    mut closure: *mut libc::c_void,
) {
    let mut slots: *mut cairo_user_data_slot_t = 0 as *mut cairo_user_data_slot_t;
    let mut i: libc::c_uint = 0;
    let mut num_slots: libc::c_uint = 0;
    num_slots = (*array).num_elements;
    slots = _cairo_array_index(array, 0 as libc::c_int as libc::c_uint)
        as *mut cairo_user_data_slot_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_slots {
        if !((*slots.offset(i as isize)).user_data).is_null() {
            func
                .expect(
                    "non-null function pointer",
                )(
                (*slots.offset(i as isize)).key as *const libc::c_void,
                (*slots.offset(i as isize)).user_data,
                closure,
            );
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_array_sort(
    mut array: *const cairo_array_t,
    mut compar: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    qsort(
        (*array).elements as *mut libc::c_void,
        (*array).num_elements as size_t,
        (*array).element_size as size_t,
        compar,
    );
}
