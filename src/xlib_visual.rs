use ::libc;
extern "C" {
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn XAllocColor(_: *mut Display, _: Colormap, _: *mut XColor) -> libc::c_int;
    fn XQueryColors(
        _: *mut Display,
        _: Colormap,
        _: *mut XColor,
        _: libc::c_int,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type int8_t = __int8_t;
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
pub type uint8_t = __uint8_t;
pub type XID = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Window = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xlib_visual_info {
    pub link: cairo_list_t,
    pub visualid: VisualID,
    pub colors: [C2RustUnnamed_0; 256],
    pub cube_to_pseudocolor: [[[uint8_t; 6]; 6]; 6],
    pub field8_to_cube: [uint8_t; 256],
    pub dither8_to_cube: [int8_t; 256],
    pub gray8_to_pseudocolor: [uint8_t; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub a: uint8_t,
    pub r: uint8_t,
    pub g: uint8_t,
    pub b: uint8_t,
}
pub type cairo_xlib_visual_info_t = _cairo_xlib_visual_info;
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh0 = (*entry).next;
    *fresh0 = entry;
    let ref mut fresh1 = (*entry).prev;
    *fresh1 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_del(
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh2 = (*next).prev;
    *fresh2 = prev;
    let ref mut fresh3 = (*prev).next;
    *fresh3 = next;
}
#[inline]
unsafe extern "C" fn _cairo_list_del(mut entry: *mut cairo_list_t) {
    __cairo_list_del((*entry).prev, (*entry).next);
}
#[inline]
unsafe extern "C" fn _color_distance(
    mut r1: libc::c_ushort,
    mut g1: libc::c_ushort,
    mut b1: libc::c_ushort,
    mut r2: libc::c_ushort,
    mut g2: libc::c_ushort,
    mut b2: libc::c_ushort,
) -> libc::c_int {
    r1 = (r1 as libc::c_int >> 8 as libc::c_int) as libc::c_ushort;
    g1 = (g1 as libc::c_int >> 8 as libc::c_int) as libc::c_ushort;
    b1 = (b1 as libc::c_int >> 8 as libc::c_int) as libc::c_ushort;
    r2 = (r2 as libc::c_int >> 8 as libc::c_int) as libc::c_ushort;
    g2 = (g2 as libc::c_int >> 8 as libc::c_int) as libc::c_ushort;
    b2 = (b2 as libc::c_int >> 8 as libc::c_int) as libc::c_ushort;
    return (r2 as libc::c_int - r1 as libc::c_int)
        * (r2 as libc::c_int - r1 as libc::c_int)
        + (g2 as libc::c_int - g1 as libc::c_int)
            * (g2 as libc::c_int - g1 as libc::c_int)
        + (b2 as libc::c_int - b1 as libc::c_int)
            * (b2 as libc::c_int - b1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_visual_info_create(
    mut dpy: *mut Display,
    mut screen: libc::c_int,
    mut visualid: VisualID,
    mut out: *mut *mut cairo_xlib_visual_info_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut info: *mut cairo_xlib_visual_info_t = 0 as *mut cairo_xlib_visual_info_t;
    let mut colormap: Colormap = (*((*(dpy as _XPrivDisplay)).screens)
        .offset(screen as isize))
        .cmap;
    let mut color: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    let mut gray: libc::c_int = 0;
    let mut red: libc::c_int = 0;
    let mut green: libc::c_int = 0;
    let mut blue: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut distance: libc::c_int = 0;
    let mut min_distance: libc::c_int = 0 as libc::c_int;
    let mut colors: [XColor; 256] = [XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    }; 256];
    let mut cube_index_to_short: [libc::c_ushort; 6] = [0; 6];
    let mut ramp_index_to_short: [libc::c_ushort; 16] = [0; 16];
    let mut gray_to_pseudocolor: [libc::c_uchar; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        cube_index_to_short[i
            as usize] = ((0xffff as libc::c_int * i
            + (6 as libc::c_int - 1 as libc::c_int >> 1 as libc::c_int))
            / (6 as libc::c_int - 1 as libc::c_int)) as libc::c_ushort;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        ramp_index_to_short[i
            as usize] = ((0xffff as libc::c_int * i
            + (16 as libc::c_int - 1 as libc::c_int >> 1 as libc::c_int))
            / (16 as libc::c_int - 1 as libc::c_int)) as libc::c_ushort;
        i += 1;
    }
    info = (if ::std::mem::size_of::<cairo_xlib_visual_info_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xlib_visual_info_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xlib_visual_info_t;
    if info.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    cairo_list_init(&mut (*info).link);
    (*info).visualid = visualid;
    gray = 0 as libc::c_int;
    loop {
        if !(gray < 16 as libc::c_int) {
            current_block = 26972500619410423;
            break;
        }
        color.blue = ramp_index_to_short[gray as usize];
        color.green = color.blue;
        color.red = color.green;
        if XAllocColor(dpy, colormap, &mut color) == 0 {
            current_block = 14316396439181454654;
            break;
        }
        gray += 1;
    }
    match current_block {
        26972500619410423 => {
            red = 0 as libc::c_int;
            's_104: while red < 6 as libc::c_int {
                green = 0 as libc::c_int;
                while green < 6 as libc::c_int {
                    blue = 0 as libc::c_int;
                    while blue < 6 as libc::c_int {
                        color.red = cube_index_to_short[red as usize];
                        color.green = cube_index_to_short[green as usize];
                        color.blue = cube_index_to_short[blue as usize];
                        color.pixel = 0 as libc::c_int as libc::c_ulong;
                        color.flags = 0 as libc::c_int as libc::c_char;
                        color.pad = 0 as libc::c_int as libc::c_char;
                        if XAllocColor(dpy, colormap, &mut color) == 0 {
                            break 's_104;
                        }
                        blue += 1;
                    }
                    green += 1;
                }
                red += 1;
            }
        }
        _ => {}
    }
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[XColor; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<XColor>() as libc::c_ulong)
            as libc::c_int
    {
        colors[i as usize].pixel = i as libc::c_ulong;
        i += 1;
    }
    XQueryColors(
        dpy,
        colormap,
        colors.as_mut_ptr(),
        (::std::mem::size_of::<[XColor; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<XColor>() as libc::c_ulong)
            as libc::c_int,
    );
    gray = 0 as libc::c_int;
    while gray < 16 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            distance = _color_distance(
                ramp_index_to_short[gray as usize],
                ramp_index_to_short[gray as usize],
                ramp_index_to_short[gray as usize],
                colors[i as usize].red,
                colors[i as usize].green,
                colors[i as usize].blue,
            );
            if i == 0 as libc::c_int || distance < min_distance {
                gray_to_pseudocolor[gray
                    as usize] = colors[i as usize].pixel as libc::c_uchar;
                min_distance = distance;
                if min_distance == 0 {
                    break;
                }
            }
            i += 1;
        }
        gray += 1;
    }
    red = 0 as libc::c_int;
    while red < 6 as libc::c_int {
        green = 0 as libc::c_int;
        while green < 6 as libc::c_int {
            blue = 0 as libc::c_int;
            while blue < 6 as libc::c_int {
                i = 0 as libc::c_int;
                while i < 256 as libc::c_int {
                    distance = _color_distance(
                        cube_index_to_short[red as usize],
                        cube_index_to_short[green as usize],
                        cube_index_to_short[blue as usize],
                        colors[i as usize].red,
                        colors[i as usize].green,
                        colors[i as usize].blue,
                    );
                    if i == 0 as libc::c_int || distance < min_distance {
                        (*info)
                            .cube_to_pseudocolor[red
                            as usize][green
                            as usize][blue
                            as usize] = colors[i as usize].pixel as uint8_t;
                        min_distance = distance;
                        if min_distance == 0 {
                            break;
                        }
                    }
                    i += 1;
                }
                blue += 1;
            }
            green += 1;
        }
        red += 1;
    }
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if j < 6 as libc::c_int - 1 as libc::c_int
            && (i << 8 as libc::c_int) + i
                - cube_index_to_short[j as usize] as libc::c_int
                > cube_index_to_short[(j + 1 as libc::c_int) as usize] as libc::c_int
                    - ((i << 8 as libc::c_int) + i)
        {
            j += 1;
        }
        (*info).field8_to_cube[i as usize] = j as uint8_t;
        (*info)
            .dither8_to_cube[i
            as usize] = ((i - 128 as libc::c_int)
            / (6 as libc::c_int - 1 as libc::c_int)) as int8_t;
        i += 1;
    }
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if j < 16 as libc::c_int - 1 as libc::c_int
            && (i << 8 as libc::c_int) + i
                - ramp_index_to_short[j as usize] as libc::c_int
                > ramp_index_to_short[(j + 1 as libc::c_int) as usize] as libc::c_int
                    - ((i << 8 as libc::c_int) + i)
        {
            j += 1;
        }
        (*info).gray8_to_pseudocolor[i as usize] = gray_to_pseudocolor[j as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*info).colors[i as usize].a = 0xff as libc::c_int as uint8_t;
        (*info)
            .colors[i as usize]
            .r = (colors[i as usize].red as libc::c_int >> 8 as libc::c_int) as uint8_t;
        (*info)
            .colors[i as usize]
            .g = (colors[i as usize].green as libc::c_int >> 8 as libc::c_int)
            as uint8_t;
        (*info)
            .colors[i as usize]
            .b = (colors[i as usize].blue as libc::c_int >> 8 as libc::c_int) as uint8_t;
        i += 1;
    }
    *out = info;
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xlib_visual_info_destroy(
    mut info: *mut cairo_xlib_visual_info_t,
) {
    _cairo_list_del(&mut (*info).link);
    free(info as *mut libc::c_void);
}
