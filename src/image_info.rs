use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type cairo_bool_t = libc::c_int;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_image_info {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub num_components: libc::c_int,
    pub bits_per_component: libc::c_int,
}
pub type cairo_image_info_t = _cairo_image_info;
#[inline]
unsafe extern "C" fn get_unaligned_be32(mut p: *const libc::c_uchar) -> uint32_t {
    return (*p.offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | ((*p.offset(1 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
            as libc::c_uint
        | ((*p.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            as libc::c_uint | *p.offset(3 as libc::c_int as isize) as libc::c_uint;
}
unsafe extern "C" fn _jpeg_skip_segment(
    mut p: *const libc::c_uchar,
) -> *const libc::c_uchar {
    let mut len: libc::c_int = 0;
    p = p.offset(1);
    len = (*p.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *p.offset(1 as libc::c_int as isize) as libc::c_int;
    return p.offset(len as isize);
}
unsafe extern "C" fn _jpeg_extract_info(
    mut info: *mut cairo_image_info_t,
    mut p: *const libc::c_uchar,
) {
    (*info)
        .width = ((*p.offset(6 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int) + *p.offset(7 as libc::c_int as isize) as libc::c_int;
    (*info)
        .height = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int) + *p.offset(5 as libc::c_int as isize) as libc::c_int;
    (*info).num_components = *p.offset(8 as libc::c_int as isize) as libc::c_int;
    (*info).bits_per_component = *p.offset(3 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_info_get_jpeg_info(
    mut info: *mut cairo_image_info_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_ulong,
) -> cairo_int_status_t {
    let mut p: *const libc::c_uchar = data;
    while p.offset(1 as libc::c_int as isize) < data.offset(length as isize) {
        if *p as libc::c_int != 0xff as libc::c_int {
            return CAIRO_INT_STATUS_UNSUPPORTED;
        }
        p = p.offset(1);
        match *p as libc::c_int {
            255 => {
                p = p.offset(1);
            }
            1 | 216 | 217 => {
                p = p.offset(1);
            }
            192 | 193 | 194 | 195 | 197 | 198 | 199 | 201 | 202 | 203 | 205 | 206
            | 207 => {
                if p.offset(8 as libc::c_int as isize) > data.offset(length as isize) {
                    return CAIRO_INT_STATUS_UNSUPPORTED;
                }
                _jpeg_extract_info(info, p);
                return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            }
            _ => {
                if *p as libc::c_int >= 0xd0 as libc::c_int
                    && *p as libc::c_int <= 0xd7 as libc::c_int
                {
                    p = p.offset(1);
                } else {
                    if p.offset(3 as libc::c_int as isize) > data.offset(length as isize)
                    {
                        return CAIRO_INT_STATUS_UNSUPPORTED;
                    }
                    p = _jpeg_skip_segment(p);
                }
            }
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
static mut _jpx_signature: [libc::c_uchar; 12] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn _jpx_next_box(mut p: *const libc::c_uchar) -> *const libc::c_uchar {
    return p.offset(get_unaligned_be32(p) as isize);
}
unsafe extern "C" fn _jpx_get_box_contents(
    mut p: *const libc::c_uchar,
) -> *const libc::c_uchar {
    return p.offset(8 as libc::c_int as isize);
}
unsafe extern "C" fn _jpx_match_box(
    mut p: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
    mut type_0: uint32_t,
) -> cairo_bool_t {
    let mut length: uint32_t = 0;
    if p.offset(8 as libc::c_int as isize) < end {
        length = get_unaligned_be32(p);
        if get_unaligned_be32(p.offset(4 as libc::c_int as isize)) == type_0
            && p.offset(length as isize) < end
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _jpx_find_box(
    mut p: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
    mut type_0: uint32_t,
) -> *const libc::c_uchar {
    while p < end {
        if _jpx_match_box(p, end, type_0) != 0 {
            return p;
        }
        p = _jpx_next_box(p);
    }
    return 0 as *const libc::c_uchar;
}
unsafe extern "C" fn _jpx_extract_info(
    mut p: *const libc::c_uchar,
    mut info: *mut cairo_image_info_t,
) {
    (*info).height = get_unaligned_be32(p) as libc::c_int;
    (*info)
        .width = get_unaligned_be32(p.offset(4 as libc::c_int as isize)) as libc::c_int;
    (*info)
        .num_components = ((*p.offset(8 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int) + *p.offset(9 as libc::c_int as isize) as libc::c_int;
    (*info).bits_per_component = *p.offset(10 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_info_get_jpx_info(
    mut info: *mut cairo_image_info_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_ulong,
) -> cairo_int_status_t {
    let mut p: *const libc::c_uchar = data;
    let mut end: *const libc::c_uchar = data.offset(length as isize);
    if length
        < (::std::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as libc::c_int as libc::c_ulong
        || memcmp(
            p as *const libc::c_void,
            _jpx_signature.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = p
        .offset(
            (::std::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as libc::c_int as isize,
        );
    if _jpx_match_box(p, end, 0x66747970 as libc::c_int as uint32_t) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = _jpx_next_box(p);
    p = _jpx_find_box(p, end, 0x6a703268 as libc::c_int as uint32_t);
    if p.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = _jpx_get_box_contents(p);
    if _jpx_match_box(p, end, 0x69686472 as libc::c_int as uint32_t) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = _jpx_get_box_contents(p);
    _jpx_extract_info(p, info);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
static mut _png_magic: [libc::c_uchar; 8] = [
    137 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_info_get_png_info(
    mut info: *mut cairo_image_info_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_ulong,
) -> cairo_int_status_t {
    let mut p: *const libc::c_uchar = data;
    let mut end: *const libc::c_uchar = data.offset(length as isize);
    if length < 8 as libc::c_int as libc::c_ulong
        || memcmp(
            data as *const libc::c_void,
            _png_magic.as_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = p.offset(8 as libc::c_int as isize);
    if p.offset(13 as libc::c_int as isize).offset(12 as libc::c_int as isize) > end {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = p.offset(4 as libc::c_int as isize);
    if get_unaligned_be32(p) != 0x49484452 as libc::c_int as libc::c_uint {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    p = p.offset(4 as libc::c_int as isize);
    (*info).width = get_unaligned_be32(p) as libc::c_int;
    p = p.offset(4 as libc::c_int as isize);
    (*info).height = get_unaligned_be32(p) as libc::c_int;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _jbig2_find_data_end(
    mut p: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
    mut type_0: libc::c_int,
) -> *const libc::c_uchar {
    let mut end_seq: [libc::c_uchar; 2] = [0; 2];
    let mut mmr: libc::c_int = 0;
    if type_0 == 36 as libc::c_int || type_0 == 38 as libc::c_int
        || type_0 == 39 as libc::c_int
    {
        if p.offset(18 as libc::c_int as isize) < end {
            mmr = *p.offset(17 as libc::c_int as isize) as libc::c_int
                & 0x1 as libc::c_int;
            if mmr != 0 {
                end_seq[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
                end_seq[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            } else {
                end_seq[0 as libc::c_int
                    as usize] = 0xff as libc::c_int as libc::c_uchar;
                end_seq[1 as libc::c_int
                    as usize] = 0xac as libc::c_int as libc::c_uchar;
            }
            p = p.offset(18 as libc::c_int as isize);
            while p < end {
                if *p.offset(0 as libc::c_int as isize) as libc::c_int
                    == end_seq[0 as libc::c_int as usize] as libc::c_int
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                        == end_seq[1 as libc::c_int as usize] as libc::c_int
                {
                    p = p.offset(6 as libc::c_int as isize);
                    if p < end {
                        return p;
                    }
                }
                p = p.offset(1);
            }
        }
    }
    return 0 as *const libc::c_uchar;
}
unsafe extern "C" fn _jbig2_get_next_segment(
    mut p: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
    mut type_0: *mut libc::c_int,
    mut data: *mut *const libc::c_uchar,
    mut data_len: *mut libc::c_ulong,
) -> *const libc::c_uchar {
    let mut seg_num: libc::c_ulong = 0;
    let mut big_page_size: cairo_bool_t = 0;
    let mut num_segs: libc::c_int = 0;
    let mut ref_seg_bytes: libc::c_int = 0;
    let mut referred_size: libc::c_int = 0;
    if p.offset(6 as libc::c_int as isize) >= end {
        return 0 as *const libc::c_uchar;
    }
    seg_num = get_unaligned_be32(p) as libc::c_ulong;
    *type_0 = *p.offset(4 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int;
    big_page_size = (*p.offset(4 as libc::c_int as isize) as libc::c_int
        & 0x40 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    p = p.offset(5 as libc::c_int as isize);
    num_segs = *p.offset(0 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int;
    if num_segs == 7 as libc::c_int {
        num_segs = (get_unaligned_be32(p) & 0x1fffffff as libc::c_int as libc::c_uint)
            as libc::c_int;
        ref_seg_bytes = 4 as libc::c_int
            + (num_segs + 1 as libc::c_int) / 8 as libc::c_int;
    } else {
        ref_seg_bytes = 1 as libc::c_int;
    }
    p = p.offset(ref_seg_bytes as isize);
    if seg_num <= 256 as libc::c_int as libc::c_ulong {
        referred_size = 1 as libc::c_int;
    } else if seg_num <= 65536 as libc::c_int as libc::c_ulong {
        referred_size = 2 as libc::c_int;
    } else {
        referred_size = 4 as libc::c_int;
    }
    p = p.offset((num_segs * referred_size) as isize);
    p = p
        .offset(
            (if big_page_size != 0 { 4 as libc::c_int } else { 1 as libc::c_int })
                as isize,
        );
    if p.offset(4 as libc::c_int as isize) >= end {
        return 0 as *const libc::c_uchar;
    }
    *data_len = get_unaligned_be32(p) as libc::c_ulong;
    p = p.offset(4 as libc::c_int as isize);
    *data = p;
    if *data_len == 0xffffffff as libc::c_uint as libc::c_ulong {
        p = _jbig2_find_data_end(*data, end, *type_0);
        if p.is_null() || p >= end {
            return 0 as *const libc::c_uchar;
        }
        *data_len = p.offset_from(*data) as libc::c_long as libc::c_ulong;
    } else {
        p = p.offset(*data_len as isize);
    }
    if p < end { return p } else { return 0 as *const libc::c_uchar };
}
unsafe extern "C" fn _jbig2_extract_info(
    mut info: *mut cairo_image_info_t,
    mut p: *const libc::c_uchar,
) {
    (*info).width = get_unaligned_be32(p) as libc::c_int;
    (*info)
        .height = get_unaligned_be32(p.offset(4 as libc::c_int as isize)) as libc::c_int;
    (*info).num_components = 1 as libc::c_int;
    (*info).bits_per_component = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_image_info_get_jbig2_info(
    mut info: *mut cairo_image_info_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_ulong,
) -> cairo_int_status_t {
    let mut p: *const libc::c_uchar = data;
    let mut end: *const libc::c_uchar = data.offset(length as isize);
    let mut seg_type: libc::c_int = 0;
    let mut seg_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut seg_data_len: libc::c_ulong = 0;
    while !p.is_null() && p < end {
        p = _jbig2_get_next_segment(
            p,
            end,
            &mut seg_type,
            &mut seg_data,
            &mut seg_data_len,
        );
        if !p.is_null() && seg_type == 48 as libc::c_int
            && seg_data_len > 8 as libc::c_int as libc::c_ulong
        {
            _jbig2_extract_info(info, seg_data);
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
