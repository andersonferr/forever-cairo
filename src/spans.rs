use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_half_open_span {
    pub x: int32_t,
    pub coverage: uint8_t,
    pub inverse: uint8_t,
}
pub type cairo_half_open_span_t = _cairo_half_open_span;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_span_renderer {
    pub status: cairo_status_t,
    pub destroy: cairo_destroy_func_t,
    pub render_rows: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            *const cairo_half_open_span_t,
            libc::c_uint,
        ) -> cairo_status_t,
    >,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
}
pub type cairo_span_renderer_t = _cairo_span_renderer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scan_converter {
    pub destroy: cairo_destroy_func_t,
    pub generate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_span_renderer_t,
        ) -> cairo_status_t,
    >,
    pub status: cairo_status_t,
}
pub type cairo_scan_converter_t = _cairo_scan_converter;
unsafe extern "C" fn _cairo_nil_destroy(mut abstract_0: *mut libc::c_void) {}
unsafe extern "C" fn _cairo_nil_scan_converter_generate(
    mut abstract_converter: *mut libc::c_void,
    mut renderer: *mut cairo_span_renderer_t,
) -> cairo_status_t {
    return _cairo_scan_converter_status(abstract_converter);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scan_converter_status(
    mut abstract_converter: *mut libc::c_void,
) -> cairo_status_t {
    let mut converter: *mut cairo_scan_converter_t = abstract_converter
        as *mut cairo_scan_converter_t;
    return (*converter).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scan_converter_set_error(
    mut abstract_converter: *mut libc::c_void,
    mut error: cairo_status_t,
) -> cairo_status_t {
    let mut converter: *mut cairo_scan_converter_t = abstract_converter
        as *mut cairo_scan_converter_t;
    if error as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-spans.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"cairo_status_t _cairo_scan_converter_set_error(void *, cairo_status_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*converter).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        let ref mut fresh0 = (*converter).generate;
        *fresh0 = Some(
            _cairo_nil_scan_converter_generate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cairo_span_renderer_t,
                ) -> cairo_status_t,
        );
        (*converter).status = error;
    }
    return (*converter).status;
}
unsafe extern "C" fn _cairo_nil_scan_converter_init(
    mut converter: *mut cairo_scan_converter_t,
    mut status: cairo_status_t,
) {
    let ref mut fresh1 = (*converter).destroy;
    *fresh1 = Some(_cairo_nil_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*converter).status = CAIRO_STATUS_SUCCESS;
    status = _cairo_scan_converter_set_error(converter as *mut libc::c_void, status);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_scan_converter_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_scan_converter_t {
    match status as libc::c_uint {
        0 | 44 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-spans.c\0" as *const u8 as *const libc::c_char,
                    91 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"cairo_scan_converter_t *_cairo_scan_converter_create_in_error(cairo_status_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        2 => {
            static mut nil: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil, status);
            return &mut nil;
        }
        3 => {
            static mut nil_0: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_0, status);
            return &mut nil_0;
        }
        4 => {
            static mut nil_1: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_1, status);
            return &mut nil_1;
        }
        5 => {
            static mut nil_2: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_2, status);
            return &mut nil_2;
        }
        6 => {
            static mut nil_3: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_3, status);
            return &mut nil_3;
        }
        7 => {
            static mut nil_4: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_4, status);
            return &mut nil_4;
        }
        8 => {
            static mut nil_5: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_5, status);
            return &mut nil_5;
        }
        9 => {
            static mut nil_6: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_6, status);
            return &mut nil_6;
        }
        10 => {
            static mut nil_7: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_7, status);
            return &mut nil_7;
        }
        11 => {
            static mut nil_8: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_8, status);
            return &mut nil_8;
        }
        12 => {
            static mut nil_9: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_9, status);
            return &mut nil_9;
        }
        13 => {
            static mut nil_10: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_10, status);
            return &mut nil_10;
        }
        14 => {
            static mut nil_11: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_11, status);
            return &mut nil_11;
        }
        15 => {
            static mut nil_12: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_12, status);
            return &mut nil_12;
        }
        16 => {
            static mut nil_13: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_13, status);
            return &mut nil_13;
        }
        17 => {
            static mut nil_14: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_14, status);
            return &mut nil_14;
        }
        18 => {
            static mut nil_15: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_15, status);
            return &mut nil_15;
        }
        19 => {
            static mut nil_16: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_16, status);
            return &mut nil_16;
        }
        20 => {
            static mut nil_17: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_17, status);
            return &mut nil_17;
        }
        21 => {
            static mut nil_18: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_18, status);
            return &mut nil_18;
        }
        22 => {
            static mut nil_19: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_19, status);
            return &mut nil_19;
        }
        23 => {
            static mut nil_20: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_20, status);
            return &mut nil_20;
        }
        24 => {
            static mut nil_21: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_21, status);
            return &mut nil_21;
        }
        25 => {
            static mut nil_22: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_22, status);
            return &mut nil_22;
        }
        26 => {
            static mut nil_23: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_23, status);
            return &mut nil_23;
        }
        27 => {
            static mut nil_24: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_24, status);
            return &mut nil_24;
        }
        28 => {
            static mut nil_25: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_25, status);
            return &mut nil_25;
        }
        29 => {
            static mut nil_26: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_26, status);
            return &mut nil_26;
        }
        30 => {
            static mut nil_27: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_27, status);
            return &mut nil_27;
        }
        31 => {
            static mut nil_28: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_28, status);
            return &mut nil_28;
        }
        1 => {
            static mut nil_29: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_29, status);
            return &mut nil_29;
        }
        32 => {
            static mut nil_30: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_30, status);
            return &mut nil_30;
        }
        33 => {
            static mut nil_31: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_31, status);
            return &mut nil_31;
        }
        34 => {
            static mut nil_32: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_32, status);
            return &mut nil_32;
        }
        35 => {
            static mut nil_33: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_33, status);
            return &mut nil_33;
        }
        36 => {
            static mut nil_34: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_34, status);
            return &mut nil_34;
        }
        37 => {
            static mut nil_35: cairo_scan_converter_t = cairo_scan_converter_t {
                destroy: None,
                generate: None,
                status: CAIRO_STATUS_SUCCESS,
            };
            _cairo_nil_scan_converter_init(&mut nil_35, status);
            return &mut nil_35;
        }
        38 | 39 | 40 | 41 | 42 | 43 | _ => {}
    }
    status = CAIRO_STATUS_NO_MEMORY;
    static mut nil_36: cairo_scan_converter_t = cairo_scan_converter_t {
        destroy: None,
        generate: None,
        status: CAIRO_STATUS_SUCCESS,
    };
    _cairo_nil_scan_converter_init(&mut nil_36, status);
    return &mut nil_36;
}
unsafe extern "C" fn _cairo_nil_span_renderer_render_rows(
    mut abstract_renderer: *mut libc::c_void,
    mut y: libc::c_int,
    mut height: libc::c_int,
    mut coverages: *const cairo_half_open_span_t,
    mut num_coverages: libc::c_uint,
) -> cairo_status_t {
    return _cairo_span_renderer_status(abstract_renderer);
}
unsafe extern "C" fn _cairo_nil_span_renderer_finish(
    mut abstract_renderer: *mut libc::c_void,
) -> cairo_status_t {
    return _cairo_span_renderer_status(abstract_renderer);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_span_renderer_status(
    mut abstract_renderer: *mut libc::c_void,
) -> cairo_status_t {
    let mut renderer: *mut cairo_span_renderer_t = abstract_renderer
        as *mut cairo_span_renderer_t;
    return (*renderer).status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_span_renderer_set_error(
    mut abstract_renderer: *mut libc::c_void,
    mut error: cairo_status_t,
) -> cairo_status_t {
    let mut renderer: *mut cairo_span_renderer_t = abstract_renderer
        as *mut cairo_span_renderer_t;
    if error as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-spans.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"cairo_status_t _cairo_span_renderer_set_error(void *, cairo_status_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    if (*renderer).status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        let ref mut fresh2 = (*renderer).render_rows;
        *fresh2 = Some(
            _cairo_nil_span_renderer_render_rows
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *const cairo_half_open_span_t,
                    libc::c_uint,
                ) -> cairo_status_t,
        );
        let ref mut fresh3 = (*renderer).finish;
        *fresh3 = Some(
            _cairo_nil_span_renderer_finish
                as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
        );
        (*renderer).status = error;
    }
    return (*renderer).status;
}
unsafe extern "C" fn _cairo_nil_span_renderer_init(
    mut renderer: *mut cairo_span_renderer_t,
    mut status: cairo_status_t,
) {
    let ref mut fresh4 = (*renderer).destroy;
    *fresh4 = Some(_cairo_nil_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*renderer).status = CAIRO_STATUS_SUCCESS;
    status = _cairo_span_renderer_set_error(renderer as *mut libc::c_void, status);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_span_renderer_create_in_error(
    mut status: cairo_status_t,
) -> *mut cairo_span_renderer_t {
    match status as libc::c_uint {
        0 | 44 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-spans.c\0" as *const u8 as *const libc::c_char,
                    209 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 76],
                        &[libc::c_char; 76],
                    >(
                        b"cairo_span_renderer_t *_cairo_span_renderer_create_in_error(cairo_status_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        2 => {
            static mut nil: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil, status);
            return &mut nil;
        }
        3 => {
            static mut nil_0: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_0, status);
            return &mut nil_0;
        }
        4 => {
            static mut nil_1: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_1, status);
            return &mut nil_1;
        }
        5 => {
            static mut nil_2: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_2, status);
            return &mut nil_2;
        }
        6 => {
            static mut nil_3: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_3, status);
            return &mut nil_3;
        }
        7 => {
            static mut nil_4: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_4, status);
            return &mut nil_4;
        }
        8 => {
            static mut nil_5: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_5, status);
            return &mut nil_5;
        }
        9 => {
            static mut nil_6: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_6, status);
            return &mut nil_6;
        }
        10 => {
            static mut nil_7: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_7, status);
            return &mut nil_7;
        }
        11 => {
            static mut nil_8: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_8, status);
            return &mut nil_8;
        }
        12 => {
            static mut nil_9: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_9, status);
            return &mut nil_9;
        }
        13 => {
            static mut nil_10: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_10, status);
            return &mut nil_10;
        }
        14 => {
            static mut nil_11: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_11, status);
            return &mut nil_11;
        }
        15 => {
            static mut nil_12: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_12, status);
            return &mut nil_12;
        }
        16 => {
            static mut nil_13: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_13, status);
            return &mut nil_13;
        }
        17 => {
            static mut nil_14: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_14, status);
            return &mut nil_14;
        }
        18 => {
            static mut nil_15: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_15, status);
            return &mut nil_15;
        }
        19 => {
            static mut nil_16: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_16, status);
            return &mut nil_16;
        }
        20 => {
            static mut nil_17: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_17, status);
            return &mut nil_17;
        }
        21 => {
            static mut nil_18: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_18, status);
            return &mut nil_18;
        }
        22 => {
            static mut nil_19: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_19, status);
            return &mut nil_19;
        }
        23 => {
            static mut nil_20: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_20, status);
            return &mut nil_20;
        }
        24 => {
            static mut nil_21: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_21, status);
            return &mut nil_21;
        }
        25 => {
            static mut nil_22: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_22, status);
            return &mut nil_22;
        }
        26 => {
            static mut nil_23: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_23, status);
            return &mut nil_23;
        }
        27 => {
            static mut nil_24: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_24, status);
            return &mut nil_24;
        }
        28 => {
            static mut nil_25: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_25, status);
            return &mut nil_25;
        }
        29 => {
            static mut nil_26: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_26, status);
            return &mut nil_26;
        }
        30 => {
            static mut nil_27: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_27, status);
            return &mut nil_27;
        }
        31 => {
            static mut nil_28: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_28, status);
            return &mut nil_28;
        }
        1 => {
            static mut nil_29: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_29, status);
            return &mut nil_29;
        }
        32 => {
            static mut nil_30: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_30, status);
            return &mut nil_30;
        }
        33 => {
            static mut nil_31: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_31, status);
            return &mut nil_31;
        }
        34 => {
            static mut nil_32: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_32, status);
            return &mut nil_32;
        }
        35 => {
            static mut nil_33: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_33, status);
            return &mut nil_33;
        }
        36 => {
            static mut nil_34: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_34, status);
            return &mut nil_34;
        }
        37 => {
            static mut nil_35: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_35, status);
            return &mut nil_35;
        }
        38 => {
            static mut nil_36: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_36, status);
            return &mut nil_36;
        }
        39 => {
            static mut nil_37: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_37, status);
            return &mut nil_37;
        }
        40 => {
            static mut nil_38: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_38, status);
            return &mut nil_38;
        }
        41 => {
            static mut nil_39: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_39, status);
            return &mut nil_39;
        }
        42 => {
            static mut nil_40: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_40, status);
            return &mut nil_40;
        }
        43 => {
            static mut nil_41: cairo_span_renderer_t = cairo_span_renderer_t {
                status: CAIRO_STATUS_SUCCESS,
                destroy: None,
                render_rows: None,
                finish: None,
            };
            _cairo_nil_span_renderer_init(&mut nil_41, status);
            return &mut nil_41;
        }
        _ => {}
    }
    status = CAIRO_STATUS_NO_MEMORY;
    static mut nil_42: cairo_span_renderer_t = cairo_span_renderer_t {
        status: CAIRO_STATUS_SUCCESS,
        destroy: None,
        render_rows: None,
        finish: None,
    };
    _cairo_nil_span_renderer_init(&mut nil_42, status);
    return &mut nil_42;
}
