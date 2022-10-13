use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_user_data_array_fini(array: *mut cairo_user_data_array_t);
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_user_data_array_get_data(
        array: *mut cairo_user_data_array_t,
        key: *const cairo_user_data_key_t,
    ) -> *mut libc::c_void;
    fn _cairo_user_data_array_set_data(
        array: *mut cairo_user_data_array_t,
        key: *const cairo_user_data_key_t,
        user_data: *mut libc::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_user_data_array_init(array: *mut cairo_user_data_array_t);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type cairo_bool_t = libc::c_int;
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
pub type cairo_device_t = _cairo_device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_device_backend_t,
    pub mutex: cairo_recursive_mutex_t,
    pub mutex_depth: libc::c_uint,
    pub finished: cairo_bool_t,
}
pub type cairo_recursive_mutex_t = cairo_recursive_mutex_impl_t;
pub type cairo_recursive_mutex_impl_t = pthread_mutex_t;
pub type cairo_device_backend_t = _cairo_device_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_device_backend {
    pub type_0: cairo_device_type_t,
    pub lock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub unlock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type cairo_device_type_t = _cairo_device_type;
pub type _cairo_device_type = libc::c_int;
pub const CAIRO_DEVICE_TYPE_INVALID: _cairo_device_type = -1;
pub const CAIRO_DEVICE_TYPE_WIN32: _cairo_device_type = 7;
pub const CAIRO_DEVICE_TYPE_COGL: _cairo_device_type = 6;
pub const CAIRO_DEVICE_TYPE_XML: _cairo_device_type = 5;
pub const CAIRO_DEVICE_TYPE_XLIB: _cairo_device_type = 4;
pub const CAIRO_DEVICE_TYPE_XCB: _cairo_device_type = 3;
pub const CAIRO_DEVICE_TYPE_SCRIPT: _cairo_device_type = 2;
pub const CAIRO_DEVICE_TYPE_GL: _cairo_device_type = 1;
pub const CAIRO_DEVICE_TYPE_DRM: _cairo_device_type = 0;
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: libc::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_cmpxchg_impl(
    mut x: *mut cairo_atomic_int_t,
    mut oldv: cairo_atomic_int_t,
    mut newv: cairo_atomic_int_t,
) -> cairo_bool_t {
    let mut expected: cairo_atomic_int_t = oldv;
    let fresh0 = ::std::intrinsics::atomic_cxchg(x, *&mut expected, newv);
    *&mut expected = fresh0.0;
    return fresh0.1 as cairo_bool_t;
}
static mut _nil_device: cairo_device_t = {
    let mut init = _cairo_device {
        ref_count: {
            let mut init = cairo_reference_count_t {
                ref_count: -(1 as libc::c_int),
            };
            init
        },
        status: CAIRO_STATUS_NO_MEMORY,
        user_data: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
        backend: 0 as *const cairo_device_backend_t,
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                },
            },
        },
        mutex_depth: 0,
        finished: 0,
    };
    init
};
static mut _mismatch_device: cairo_device_t = {
    let mut init = _cairo_device {
        ref_count: {
            let mut init = cairo_reference_count_t {
                ref_count: -(1 as libc::c_int),
            };
            init
        },
        status: CAIRO_STATUS_DEVICE_TYPE_MISMATCH,
        user_data: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
        backend: 0 as *const cairo_device_backend_t,
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                },
            },
        },
        mutex_depth: 0,
        finished: 0,
    };
    init
};
static mut _invalid_device: cairo_device_t = {
    let mut init = _cairo_device {
        ref_count: {
            let mut init = cairo_reference_count_t {
                ref_count: -(1 as libc::c_int),
            };
            init
        },
        status: CAIRO_STATUS_DEVICE_ERROR,
        user_data: cairo_user_data_array_t {
            size: 0,
            num_elements: 0,
            element_size: 0,
            elements: 0 as *const libc::c_char as *mut libc::c_char,
        },
        backend: 0 as *const cairo_device_backend_t,
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                },
            },
        },
        mutex_depth: 0,
        finished: 0,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn _cairo_device_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_device_t {
    match status as libc::c_uint {
        1 => return &_nil_device as *const cairo_device_t as *mut cairo_device_t,
        35 => return &_invalid_device as *const cairo_device_t as *mut cairo_device_t,
        34 => return &_mismatch_device as *const cairo_device_t as *mut cairo_device_t,
        0 | 44 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-device.c\0" as *const u8 as *const libc::c_char,
                    125 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"cairo_device_t *_cairo_device_create_in_error(cairo_status_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        13 | 6 | 16 | 17 | 10 | 11 | 18 | 23 | 24 | 32 | 2 | 3 | 4 | 5 | 7 | 8 | 9 | 12
        | 14 | 19 | 20 | 21 | 22 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 33 | 15 | 36 | 37
        | 38 | 39 | 40 | 41 | 42 | 43 | _ => {}
    }
    let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    return &_nil_device as *const cairo_device_t as *mut cairo_device_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_device_init(
    mut device: *mut cairo_device_t,
    mut backend: *const cairo_device_backend_t,
) {
    (*device).ref_count.ref_count = 1 as libc::c_int;
    (*device).status = CAIRO_STATUS_SUCCESS;
    let ref mut fresh1 = (*device).backend;
    *fresh1 = backend;
    let mut attr: pthread_mutexattr_t = pthread_mutexattr_t {
        __size: [0; 4],
    };
    pthread_mutexattr_init(&mut attr);
    pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE as libc::c_int);
    pthread_mutex_init(&mut (*device).mutex, &mut attr);
    pthread_mutexattr_destroy(&mut attr);
    (*device).mutex_depth = 0 as libc::c_int as libc::c_uint;
    (*device).finished = 0 as libc::c_int;
    _cairo_user_data_array_init(&mut (*device).user_data);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_reference(
    mut device: *mut cairo_device_t,
) -> *mut cairo_device_t {
    if device.is_null()
        || _cairo_atomic_int_get(&mut (*device).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return device;
    }
    if _cairo_atomic_int_get(&mut (*device).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&device->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-device.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"cairo_device_t *cairo_device_reference(cairo_device_t *)\0"))
                .as_ptr(),
        );
    }
    ::std::intrinsics::atomic_xadd(&mut (*device).ref_count.ref_count, 1 as libc::c_int);
    return device;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_status(
    mut device: *mut cairo_device_t,
) -> cairo_status_t {
    if device.is_null() {
        return CAIRO_STATUS_NULL_POINTER;
    }
    return (*device).status;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_flush(mut device: *mut cairo_device_t) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if device.is_null() || (*device).status as libc::c_uint != 0 {
        return;
    }
    if (*device).finished != 0 {
        return;
    }
    if ((*(*device).backend).flush).is_some() {
        status = ((*(*device).backend).flush)
            .expect("non-null function pointer")(device as *mut libc::c_void);
        if status as u64 != 0 {
            status = _cairo_device_set_error(device, status);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_finish(mut device: *mut cairo_device_t) {
    if device.is_null()
        || _cairo_atomic_int_get(&mut (*device).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return;
    }
    if (*device).finished != 0 {
        return;
    }
    cairo_device_flush(device);
    if ((*(*device).backend).finish).is_some() {
        ((*(*device).backend).finish)
            .expect("non-null function pointer")(device as *mut libc::c_void);
    }
    (*device).finished = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_destroy(mut device: *mut cairo_device_t) {
    let mut user_data: cairo_user_data_array_t = cairo_user_data_array_t {
        size: 0,
        num_elements: 0,
        element_size: 0,
        elements: 0 as *const libc::c_char as *mut libc::c_char,
    };
    if device.is_null()
        || _cairo_atomic_int_get(&mut (*device).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return;
    }
    if _cairo_atomic_int_get(&mut (*device).ref_count.ref_count) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"CAIRO_REFERENCE_COUNT_HAS_REFERENCE (&device->ref_count)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-device.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void cairo_device_destroy(cairo_device_t *)\0"))
                .as_ptr(),
        );
    }
    if !(::std::intrinsics::atomic_xsub(
        &mut (*device).ref_count.ref_count as *mut cairo_atomic_int_t,
        1 as libc::c_int,
    ) == 1 as libc::c_int)
    {
        return;
    }
    cairo_device_finish(device);
    if (*device).mutex_depth == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"device->mutex_depth == 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-device.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void cairo_device_destroy(cairo_device_t *)\0"))
                .as_ptr(),
        );
    }
    pthread_mutex_destroy(&mut (*device).mutex);
    user_data = (*device).user_data;
    ((*(*device).backend).destroy)
        .expect("non-null function pointer")(device as *mut libc::c_void);
    _cairo_user_data_array_fini(&mut user_data);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_get_type(
    mut device: *mut cairo_device_t,
) -> cairo_device_type_t {
    if device.is_null()
        || _cairo_atomic_int_get(&mut (*device).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return CAIRO_DEVICE_TYPE_INVALID;
    }
    return (*(*device).backend).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_acquire(
    mut device: *mut cairo_device_t,
) -> cairo_status_t {
    if device.is_null() {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*device).status as u64 != 0 {
        return (*device).status;
    }
    if (*device).finished != 0 {
        return _cairo_device_set_error(device, CAIRO_STATUS_DEVICE_FINISHED);
    }
    pthread_mutex_lock(&mut (*device).mutex);
    let ref mut fresh2 = (*device).mutex_depth;
    let fresh3 = *fresh2;
    *fresh2 = (*fresh2).wrapping_add(1);
    if fresh3 == 0 as libc::c_int as libc::c_uint {
        if ((*(*device).backend).lock).is_some() {
            ((*(*device).backend).lock)
                .expect("non-null function pointer")(device as *mut libc::c_void);
        }
    }
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_release(mut device: *mut cairo_device_t) {
    if device.is_null() {
        return;
    }
    if (*device).mutex_depth > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"device->mutex_depth > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-device.c\0" as *const u8 as *const libc::c_char,
            451 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void cairo_device_release(cairo_device_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh4 = (*device).mutex_depth;
    *fresh4 = (*fresh4).wrapping_sub(1);
    if *fresh4 == 0 as libc::c_int as libc::c_uint {
        if ((*(*device).backend).unlock).is_some() {
            ((*(*device).backend).unlock)
                .expect("non-null function pointer")(device as *mut libc::c_void);
        }
    }
    pthread_mutex_unlock(&mut (*device).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_device_set_error(
    mut device: *mut cairo_device_t,
    mut status: cairo_status_t,
) -> cairo_status_t {
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    let mut ret__: libc::c_int = 0;
    if (status as libc::c_uint) < CAIRO_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status < CAIRO_STATUS_LAST_STATUS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-device.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"cairo_status_t _cairo_device_set_error(cairo_device_t *, cairo_status_t)\0",
            ))
                .as_ptr(),
        );
    }
    ret__ = _cairo_atomic_int_cmpxchg_impl(
        &mut (*device).status as *mut cairo_status_t as *mut cairo_atomic_int_t,
        CAIRO_STATUS_SUCCESS as libc::c_int,
        status as cairo_atomic_int_t,
    );
    return _cairo_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_get_reference_count(
    mut device: *mut cairo_device_t,
) -> libc::c_uint {
    if device.is_null()
        || _cairo_atomic_int_get(&mut (*device).ref_count.ref_count)
            == -(1 as libc::c_int)
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    return _cairo_atomic_int_get(&mut (*device).ref_count.ref_count) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_get_user_data(
    mut device: *mut cairo_device_t,
    mut key: *const cairo_user_data_key_t,
) -> *mut libc::c_void {
    return _cairo_user_data_array_get_data(&mut (*device).user_data, key);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_device_set_user_data(
    mut device: *mut cairo_device_t,
    mut key: *const cairo_user_data_key_t,
    mut user_data: *mut libc::c_void,
    mut destroy: cairo_destroy_func_t,
) -> cairo_status_t {
    if _cairo_atomic_int_get(&mut (*device).ref_count.ref_count) == -(1 as libc::c_int) {
        return (*device).status;
    }
    return _cairo_user_data_array_set_data(
        &mut (*device).user_data,
        key,
        user_data,
        destroy,
    );
}
