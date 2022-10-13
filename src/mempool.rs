use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
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
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mempool {
    pub base: *mut libc::c_char,
    pub blocks: *mut _cairo_memblock,
    pub free: [cairo_list_t; 32],
    pub map: *mut libc::c_uchar,
    pub num_blocks: libc::c_uint,
    pub min_bits: libc::c_int,
    pub num_sizes: libc::c_int,
    pub max_free_bits: libc::c_int,
    pub free_bytes: size_t,
    pub max_bytes: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_memblock {
    pub bits: libc::c_int,
    pub link: cairo_list_t,
}
pub type cairo_mempool_t = _cairo_mempool;
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
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
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
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
unsafe extern "C" fn clear_bits(
    mut pool: *mut cairo_mempool_t,
    mut first: size_t,
    mut last: size_t,
) {
    let mut i: size_t = 0;
    let mut n: size_t = last;
    let mut first_full: size_t = first.wrapping_add(7 as libc::c_int as libc::c_ulong)
        & !(7 as libc::c_int) as libc::c_ulong;
    let mut past_full: size_t = last & !(7 as libc::c_int) as libc::c_ulong;
    let mut bytes: size_t = 0;
    if n > first_full {
        n = first_full;
    }
    i = first;
    while i < n {
        let ref mut fresh8 = *((*pool).map).offset((i >> 3 as libc::c_int) as isize);
        *fresh8 = (*fresh8 as libc::c_int
            & !(128 as libc::c_int >> (i & 7 as libc::c_int as libc::c_ulong)))
            as libc::c_uchar;
        i = i.wrapping_add(1);
    }
    if past_full > first_full {
        bytes = past_full.wrapping_sub(first_full);
        bytes = bytes >> 3 as libc::c_int;
        memset(
            ((*pool).map).offset((first_full >> 3 as libc::c_int) as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            bytes,
        );
    }
    if past_full < n {
        past_full = n;
    }
    i = past_full;
    while i < last {
        let ref mut fresh9 = *((*pool).map).offset((i >> 3 as libc::c_int) as isize);
        *fresh9 = (*fresh9 as libc::c_int
            & !(128 as libc::c_int >> (i & 7 as libc::c_int as libc::c_ulong)))
            as libc::c_uchar;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn free_bits(
    mut pool: *mut cairo_mempool_t,
    mut start: size_t,
    mut bits: libc::c_int,
    mut clear: cairo_bool_t,
) {
    let mut block: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    if clear != 0 {
        clear_bits(
            pool,
            start,
            start.wrapping_add((1 as libc::c_int as size_t) << bits),
        );
    }
    block = ((*pool).blocks).offset(start as isize);
    (*block).bits = bits;
    cairo_list_add(
        &mut (*block).link,
        &mut *((*pool).free).as_mut_ptr().offset(bits as isize),
    );
    let ref mut fresh10 = (*pool).free_bytes;
    *fresh10 = (*fresh10 as libc::c_ulong)
        .wrapping_add((1 as libc::c_int as size_t) << bits + (*pool).min_bits) as size_t
        as size_t;
    if bits > (*pool).max_free_bits {
        (*pool).max_free_bits = bits;
    }
}
unsafe extern "C" fn free_blocks(
    mut pool: *mut cairo_mempool_t,
    mut first: size_t,
    mut last: size_t,
    mut clear: cairo_bool_t,
) {
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut bits: libc::c_int = 0 as libc::c_int;
    i = first;
    len = 1 as libc::c_int as size_t;
    while i < last {
        while bits < (*pool).num_sizes - 1 as libc::c_int {
            let mut next_bits: size_t = (bits + 1 as libc::c_int) as size_t;
            let mut next_len: size_t = len << 1 as libc::c_int;
            if i.wrapping_add(next_bits) > last {
                break;
            } else {
                if i & next_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0 {
                    break;
                }
                bits = next_bits as libc::c_int;
                len = next_len;
            }
        }
        while !(i.wrapping_add(len) <= last
            && i & len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong)
        {
            bits -= 1;
            len >>= 1 as libc::c_int;
            if !(len != 0) {
                break;
            }
        }
        if len == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        free_bits(pool, i, bits, clear);
        i = (i as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    }
}
unsafe extern "C" fn get_buddy(
    mut pool: *mut cairo_mempool_t,
    mut offset: size_t,
    mut bits: libc::c_int,
) -> *mut _cairo_memblock {
    let mut block: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    if offset.wrapping_add((1 as libc::c_int as size_t) << bits)
        >= (*pool).num_blocks as libc::c_ulong
    {
        return 0 as *mut _cairo_memblock;
    }
    if *((*pool).map)
        .offset(
            (offset
                .wrapping_add((1 as libc::c_int as size_t) << bits)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int)
                as isize,
        ) as libc::c_int
        & 128 as libc::c_int
            >> (offset
                .wrapping_add((1 as libc::c_int as size_t) << bits)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & 7 as libc::c_int as libc::c_ulong) != 0
    {
        return 0 as *mut _cairo_memblock;
    }
    block = ((*pool).blocks).offset(offset as isize);
    if (*block).bits != bits {
        return 0 as *mut _cairo_memblock;
    }
    return block;
}
unsafe extern "C" fn merge_buddies(
    mut pool: *mut cairo_mempool_t,
    mut block: *mut _cairo_memblock,
    mut max_bits: libc::c_int,
) {
    let mut block_offset: size_t = block.offset_from((*pool).blocks) as libc::c_long
        as size_t;
    let mut bits: libc::c_int = (*block).bits;
    while bits < max_bits - 1 as libc::c_int {
        let mut buddy_offset: size_t = block_offset
            ^ (1 as libc::c_int as size_t) << bits;
        block = get_buddy(pool, buddy_offset, bits);
        if block.is_null() {
            break;
        }
        cairo_list_del(&mut (*block).link);
        if buddy_offset < block_offset {
            block_offset = buddy_offset;
        }
        bits += 1;
    }
    block = ((*pool).blocks).offset(block_offset as isize);
    (*block).bits = bits;
    cairo_list_add(
        &mut (*block).link,
        &mut *((*pool).free).as_mut_ptr().offset(bits as isize),
    );
    if bits > (*pool).max_free_bits {
        (*pool).max_free_bits = bits;
    }
}
unsafe extern "C" fn merge_bits(
    mut pool: *mut cairo_mempool_t,
    mut max_bits: libc::c_int,
) -> libc::c_int {
    let mut block: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    let mut buddy: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    let mut next: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    let mut bits: libc::c_int = 0;
    bits = 0 as libc::c_int;
    while bits < max_bits - 1 as libc::c_int {
        block = ({
            let mut mptr__: *const cairo_list_t = (*pool).free[bits as usize].next;
            (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                as *mut _cairo_memblock
        });
        next = ({
            let mut mptr__: *const cairo_list_t = (*block).link.next;
            (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                as *mut _cairo_memblock
        });
        while &mut (*block).link as *mut cairo_list_t
            != &mut *((*pool).free).as_mut_ptr().offset(bits as isize)
                as *mut cairo_list_t
        {
            let mut buddy_offset: size_t = block.offset_from((*pool).blocks)
                as libc::c_long as libc::c_ulong ^ (1 as libc::c_int as size_t) << bits;
            buddy = get_buddy(pool, buddy_offset, bits);
            if !buddy.is_null() {
                if buddy == next {
                    next = ({
                        let mut mptr__: *const cairo_list_t = (*buddy).link.next;
                        (mptr__ as *mut libc::c_char)
                            .offset(-(8 as libc::c_ulong as isize))
                            as *mut _cairo_memblock
                    });
                }
                cairo_list_del(&mut (*block).link);
                merge_buddies(pool, block, max_bits);
            }
            block = next;
            next = ({
                let mut mptr__: *const cairo_list_t = (*next).link.next;
                (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                    as *mut _cairo_memblock
            });
        }
        bits += 1;
    }
    return (*pool).max_free_bits;
}
unsafe extern "C" fn buddy_malloc(
    mut pool: *mut cairo_mempool_t,
    mut bits: libc::c_int,
) -> *mut libc::c_void {
    let mut past: size_t = 0;
    let mut offset: size_t = 0;
    let mut block: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    let mut b: libc::c_int = 0;
    if bits > (*pool).max_free_bits && bits > merge_bits(pool, bits) {
        return 0 as *mut libc::c_void;
    }
    block = 0 as *mut _cairo_memblock;
    b = bits;
    while b <= (*pool).max_free_bits {
        if cairo_list_is_empty(&mut *((*pool).free).as_mut_ptr().offset(b as isize)) == 0
        {
            block = ({
                let mut mptr__: *const cairo_list_t = (*pool).free[b as usize].next;
                (mptr__ as *mut libc::c_char).offset(-(8 as libc::c_ulong as isize))
                    as *mut _cairo_memblock
            });
            break;
        } else {
            b += 1;
        }
    }
    if !block.is_null() {} else {
        __assert_fail(
            b"block != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-mempool.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void *buddy_malloc(cairo_mempool_t *, int)\0"))
                .as_ptr(),
        );
    }
    cairo_list_del(&mut (*block).link);
    while cairo_list_is_empty(
        &mut *((*pool).free).as_mut_ptr().offset((*pool).max_free_bits as isize),
    ) != 0
    {
        let ref mut fresh11 = (*pool).max_free_bits;
        *fresh11 -= 1;
        if *fresh11 == -(1 as libc::c_int) {
            break;
        }
    }
    offset = block.offset_from((*pool).blocks) as libc::c_long as size_t;
    past = offset.wrapping_add((1 as libc::c_int as size_t) << bits);
    let ref mut fresh12 = *((*pool).map)
        .offset(
            (past.wrapping_sub(1 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int)
                as isize,
        );
    *fresh12 = (*fresh12 as libc::c_int
        | 128 as libc::c_int
            >> (past.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & 7 as libc::c_int as libc::c_ulong)) as libc::c_uchar;
    (*block).bits = bits;
    let ref mut fresh13 = (*pool).free_bytes;
    *fresh13 = (*fresh13 as libc::c_ulong)
        .wrapping_sub((1 as libc::c_int as size_t) << b + (*pool).min_bits) as size_t
        as size_t;
    free_blocks(
        pool,
        past,
        offset.wrapping_add((1 as libc::c_int as size_t) << b),
        0 as libc::c_int,
    );
    return ((*pool).base)
        .offset(
            ((block.offset_from((*pool).blocks) as libc::c_long) << (*pool).min_bits)
                as isize,
        ) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mempool_init(
    mut pool: *mut cairo_mempool_t,
    mut base: *mut libc::c_void,
    mut bytes: size_t,
    mut min_bits: libc::c_int,
    mut num_sizes: libc::c_int,
) -> cairo_status_t {
    let mut tmp: uintptr_t = 0;
    let mut num_blocks: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    tmp = base as uintptr_t
        & ((1 as libc::c_int as size_t) << min_bits)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if tmp != 0 {
        tmp = ((1 as libc::c_int as size_t) << min_bits).wrapping_sub(tmp);
        base = (base as *mut libc::c_char).offset(tmp as isize) as *mut libc::c_void;
        bytes = (bytes as libc::c_ulong).wrapping_sub(tmp) as size_t as size_t;
    }
    if base as uintptr_t
        & ((1 as libc::c_int as size_t) << min_bits)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(((uintptr_t) base) & ((((size_t) 1) << min_bits) - 1)) == 0\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-mempool.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"cairo_status_t _cairo_mempool_init(cairo_mempool_t *, void *, size_t, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    if num_sizes
        < (::std::mem::size_of::<[cairo_list_t; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_list_t>() as libc::c_ulong)
            as libc::c_int
    {} else {
        __assert_fail(
            b"num_sizes < ARRAY_LENGTH (pool->free)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-mempool.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"cairo_status_t _cairo_mempool_init(cairo_mempool_t *, void *, size_t, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh14 = (*pool).base;
    *fresh14 = base as *mut libc::c_char;
    (*pool).free_bytes = 0 as libc::c_int as size_t;
    (*pool).max_bytes = bytes;
    (*pool).max_free_bits = -(1 as libc::c_int);
    num_blocks = (bytes >> min_bits) as libc::c_int;
    let ref mut fresh15 = (*pool).blocks;
    *fresh15 = calloc(
        num_blocks as libc::c_ulong,
        ::std::mem::size_of::<_cairo_memblock>() as libc::c_ulong,
    ) as *mut _cairo_memblock;
    if ((*pool).blocks).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    (*pool).num_blocks = num_blocks as libc::c_uint;
    (*pool).min_bits = min_bits;
    (*pool).num_sizes = num_sizes;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[cairo_list_t; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_list_t>() as libc::c_ulong)
            as libc::c_int
    {
        cairo_list_init(&mut *((*pool).free).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    let ref mut fresh16 = (*pool).map;
    *fresh16 = (if num_blocks + 7 as libc::c_int >> 3 as libc::c_int != 0 as libc::c_int
    {
        malloc((num_blocks + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if ((*pool).map).is_null() {
        free((*pool).blocks as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    memset(
        (*pool).map as *mut libc::c_void,
        -(1 as libc::c_int),
        (num_blocks + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong,
    );
    clear_bits(pool, 0 as libc::c_int as size_t, num_blocks as size_t);
    free_blocks(
        pool,
        0 as libc::c_int as size_t,
        num_blocks as size_t,
        1 as libc::c_int,
    );
    return CAIRO_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mempool_alloc(
    mut pool: *mut cairo_mempool_t,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let mut size: size_t = 0;
    let mut bits: libc::c_int = 0;
    size = (1 as libc::c_int as size_t) << (*pool).min_bits;
    bits = 0 as libc::c_int;
    while size < bytes {
        size <<= 1 as libc::c_int;
        bits += 1;
    }
    if bits >= (*pool).num_sizes {
        return 0 as *mut libc::c_void;
    }
    return buddy_malloc(pool, bits);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mempool_free(
    mut pool: *mut cairo_mempool_t,
    mut storage: *mut libc::c_void,
) {
    let mut block_offset: size_t = 0;
    let mut block: *mut _cairo_memblock = 0 as *mut _cairo_memblock;
    block_offset = ((storage as *mut libc::c_char).offset_from((*pool).base)
        as libc::c_long >> (*pool).min_bits) as size_t;
    block = ((*pool).blocks).offset(block_offset as isize);
    let ref mut fresh17 = *((*pool).map)
        .offset(
            (block_offset
                .wrapping_add(
                    ((1 as libc::c_int as size_t) << (*block).bits)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) >> 3 as libc::c_int) as isize,
        );
    *fresh17 = (*fresh17 as libc::c_int
        & !(128 as libc::c_int
            >> (block_offset
                .wrapping_add(
                    ((1 as libc::c_int as size_t) << (*block).bits)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) & 7 as libc::c_int as libc::c_ulong))) as libc::c_uchar;
    let ref mut fresh18 = (*pool).free_bytes;
    *fresh18 = (*fresh18 as libc::c_ulong)
        .wrapping_add((1 as libc::c_int as size_t) << (*block).bits + (*pool).min_bits)
        as size_t as size_t;
    merge_buddies(pool, block, (*pool).num_sizes);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_mempool_fini(mut pool: *mut cairo_mempool_t) {
    free((*pool).map as *mut libc::c_void);
    free((*pool).blocks as *mut libc::c_void);
}
