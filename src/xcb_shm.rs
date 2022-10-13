use ::libc;
extern "C" {
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
    fn _cairo_xcb_connection_shm_attach(
        connection: *mut cairo_xcb_connection_t,
        id: uint32_t,
        readonly: cairo_bool_t,
    ) -> uint32_t;
    fn _cairo_xcb_connection_shm_detach(
        connection: *mut cairo_xcb_connection_t,
        segment: uint32_t,
    );
    fn xcb_poll_for_reply(
        c: *mut xcb_connection_t,
        request: libc::c_uint,
        reply: *mut *mut libc::c_void,
        error: *mut *mut xcb_generic_error_t,
    ) -> libc::c_int;
    fn xcb_wait_for_reply(
        c: *mut xcb_connection_t,
        request: libc::c_uint,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut libc::c_void;
    fn _cairo_freepool_alloc_from_new_pool(
        freepool: *mut cairo_freepool_t,
    ) -> *mut libc::c_void;
    fn xcb_discard_reply(c: *mut xcb_connection_t, sequence: libc::c_uint);
    fn _cairo_mempool_init(
        pool: *mut cairo_mempool_t,
        base: *mut libc::c_void,
        bytes: size_t,
        min_bits: libc::c_int,
        num_sizes: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_mempool_alloc(
        pi: *mut cairo_mempool_t,
        bytes: size_t,
    ) -> *mut libc::c_void;
    fn _cairo_mempool_free(pi: *mut cairo_mempool_t, storage: *mut libc::c_void);
    fn _cairo_mempool_fini(pool: *mut cairo_mempool_t);
    fn shmat(
        __shmid: libc::c_int,
        __shmaddr: *const libc::c_void,
        __shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    fn shmdt(__shmaddr: *const libc::c_void) -> libc::c_int;
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
    fn shmctl(
        __shmid: libc::c_int,
        __cmd: libc::c_int,
        __buf: *mut shmid_ds,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_ulong_t = libc::c_ulong;
pub type key_t = __key_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type cairo_bool_t = libc::c_int;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
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
pub type cairo_hash_table_t = _cairo_hash_table;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_mutex_t = cairo_mutex_impl_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: uint8_t,
    pub revert_to: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub focus: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
}
pub type xcb_render_sub_pixel_t = libc::c_uint;
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 5;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 4;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 3;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 2;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 1;
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0;
pub type xcb_render_pictformat_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_pool {
    pub next: *mut cairo_freelist_pool_t,
    pub size: libc::c_uint,
    pub rem: libc::c_uint,
    pub data: *mut uint8_t,
}
pub type cairo_freelist_pool_t = _cairo_freelist_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freepool {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub pools: *mut cairo_freelist_pool_t,
    pub freepools: *mut cairo_freelist_pool_t,
    pub nodesize: libc::c_uint,
    pub embedded_pool: cairo_freelist_pool_t,
    pub embedded_data: [uint8_t; 1000],
}
pub type cairo_freepool_t = _cairo_freepool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_connection {
    pub device: cairo_device_t,
    pub xcb_connection: *mut xcb_connection_t,
    pub standard_formats: [xcb_render_pictformat_t; 5],
    pub xrender_formats: *mut cairo_hash_table_t,
    pub visual_to_xrender_format: *mut cairo_hash_table_t,
    pub maximum_request_length: libc::c_uint,
    pub flags: libc::c_uint,
    pub original_flags: libc::c_uint,
    pub force_precision: libc::c_int,
    pub root: *const xcb_setup_t,
    pub render: *const xcb_query_extension_reply_t,
    pub shm: *const xcb_query_extension_reply_t,
    pub subpixel_orders: *mut xcb_render_sub_pixel_t,
    pub shm_mutex: cairo_mutex_t,
    pub shm_pools: cairo_list_t,
    pub shm_pending: cairo_list_t,
    pub shm_info_freelist: cairo_freepool_t,
    pub screens_mutex: cairo_mutex_t,
    pub screens: cairo_list_t,
    pub fonts: cairo_list_t,
    pub link: cairo_list_t,
}
pub type cairo_xcb_connection_t = _cairo_xcb_connection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_shm_mem_pool {
    pub shmid: libc::c_int,
    pub shmseg: uint32_t,
    pub shm: *mut libc::c_void,
    pub mem: cairo_mempool_t,
    pub link: cairo_list_t,
}
pub type cairo_mempool_t = _cairo_mempool;
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
pub type cairo_xcb_shm_mem_pool_t = _cairo_xcb_shm_mem_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_shm_info {
    pub connection: *mut cairo_xcb_connection_t,
    pub shm: uint32_t,
    pub offset: uint32_t,
    pub size: size_t,
    pub mem: *mut libc::c_void,
    pub pool: *mut cairo_xcb_shm_mem_pool_t,
    pub sync: xcb_get_input_focus_cookie_t,
    pub pending: cairo_list_t,
}
pub type cairo_xcb_shm_info_t = _cairo_xcb_shm_info;
pub type C2RustUnnamed = libc::c_uint;
pub const CAIRO_XCB_SHM_MASK: C2RustUnnamed = 2147483648;
pub const CAIRO_XCB_RENDER_MASK: C2RustUnnamed = 4095;
pub const CAIRO_XCB_HAS_SHM: C2RustUnnamed = 2147483648;
pub const CAIRO_XCB_RENDER_HAS_FILTER_BEST: C2RustUnnamed = 2048;
pub const CAIRO_XCB_RENDER_HAS_FILTER_GOOD: C2RustUnnamed = 1024;
pub const CAIRO_XCB_RENDER_HAS_GRADIENTS: C2RustUnnamed = 512;
pub const CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT: C2RustUnnamed = 256;
pub const CAIRO_XCB_RENDER_HAS_PDF_OPERATORS: C2RustUnnamed = 128;
pub const CAIRO_XCB_RENDER_HAS_FILTERS: C2RustUnnamed = 64;
pub const CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM: C2RustUnnamed = 32;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS: C2RustUnnamed = 16;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS: C2RustUnnamed = 8;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE: C2RustUnnamed = 4;
pub const CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES: C2RustUnnamed = 2;
pub const CAIRO_XCB_HAS_RENDER: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: __time_t,
    pub shm_dtime: __time_t,
    pub shm_ctime: __time_t,
    pub shm_cpid: __pid_t,
    pub shm_lpid: __pid_t,
    pub shm_nattch: shmatt_t,
    pub __glibc_reserved5: __syscall_ulong_t,
    pub __glibc_reserved6: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
    pub __key: __key_t,
    pub uid: __uid_t,
    pub gid: __gid_t,
    pub cuid: __uid_t,
    pub cgid: __gid_t,
    pub mode: __mode_t,
    pub __seq: libc::c_ushort,
    pub __pad2: libc::c_ushort,
    pub __glibc_reserved1: __syscall_ulong_t,
    pub __glibc_reserved2: __syscall_ulong_t,
}
pub type shm_wait_type_t = libc::c_uint;
pub const PENDING_POLL: shm_wait_type_t = 1;
pub const PENDING_WAIT: shm_wait_type_t = 0;
#[inline]
unsafe extern "C" fn _cairo_freepool_free(
    mut freepool: *mut cairo_freepool_t,
    mut ptr: *mut libc::c_void,
) {
    let mut node: *mut cairo_freelist_node_t = ptr as *mut cairo_freelist_node_t;
    let ref mut fresh0 = (*node).next;
    *fresh0 = (*freepool).first_free_node;
    let ref mut fresh1 = (*freepool).first_free_node;
    *fresh1 = node;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_alloc(
    mut freepool: *mut cairo_freepool_t,
) -> *mut libc::c_void {
    let mut node: *mut cairo_freelist_node_t = 0 as *mut cairo_freelist_node_t;
    node = (*freepool).first_free_node;
    if node.is_null() {
        return _cairo_freepool_alloc_from_pool(freepool);
    }
    let ref mut fresh2 = (*freepool).first_free_node;
    *fresh2 = (*node).next;
    return node as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn _cairo_freepool_alloc_from_pool(
    mut freepool: *mut cairo_freepool_t,
) -> *mut libc::c_void {
    let mut pool: *mut cairo_freelist_pool_t = 0 as *mut cairo_freelist_pool_t;
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    pool = (*freepool).pools;
    if (*freepool).nodesize > (*pool).rem {
        return _cairo_freepool_alloc_from_new_pool(freepool);
    }
    ptr = (*pool).data;
    let ref mut fresh3 = (*pool).data;
    *fresh3 = (*fresh3).offset((*freepool).nodesize as isize);
    let ref mut fresh4 = (*pool).rem;
    *fresh4 = (*fresh4).wrapping_sub((*freepool).nodesize);
    return ptr as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn cairo_list_init(mut entry: *mut cairo_list_t) {
    let ref mut fresh5 = (*entry).next;
    *fresh5 = entry;
    let ref mut fresh6 = (*entry).prev;
    *fresh6 = entry;
}
#[inline]
unsafe extern "C" fn __cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut prev: *mut cairo_list_t,
    mut next: *mut cairo_list_t,
) {
    let ref mut fresh7 = (*next).prev;
    *fresh7 = entry;
    let ref mut fresh8 = (*entry).next;
    *fresh8 = next;
    let ref mut fresh9 = (*entry).prev;
    *fresh9 = prev;
    let ref mut fresh10 = (*prev).next;
    *fresh10 = entry;
}
#[inline]
unsafe extern "C" fn cairo_list_add(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_add(entry, head, (*head).next);
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
    let ref mut fresh11 = (*next).prev;
    *fresh11 = prev;
    let ref mut fresh12 = (*prev).next;
    *fresh12 = next;
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
unsafe extern "C" fn cairo_list_move(
    mut entry: *mut cairo_list_t,
    mut head: *mut cairo_list_t,
) {
    __cairo_list_del((*entry).prev, (*entry).next);
    __cairo_list_add(entry, head, (*head).next);
}
#[inline]
unsafe extern "C" fn cairo_list_is_empty(mut head: *const cairo_list_t) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list) as libc::c_int;
}
#[inline]
unsafe extern "C" fn cairo_list_is_singular(
    mut head: *const cairo_list_t,
) -> cairo_bool_t {
    return ((*head).next == head as *mut _cairo_list || (*head).next == (*head).prev)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_xcb_shm_mem_pool_destroy(
    mut pool: *mut cairo_xcb_shm_mem_pool_t,
) {
    cairo_list_del(&mut (*pool).link);
    shmdt((*pool).shm);
    _cairo_mempool_fini(&mut (*pool).mem);
    free(pool as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xcb_shm_info_finalize(
    mut shm_info: *mut cairo_xcb_shm_info_t,
) {
    let mut connection: *mut cairo_xcb_connection_t = (*shm_info).connection;
    _cairo_mempool_free(&mut (*(*shm_info).pool).mem, (*shm_info).mem);
    _cairo_freepool_free(
        &mut (*connection).shm_info_freelist,
        shm_info as *mut libc::c_void,
    );
    if cairo_list_is_singular(&mut (*connection).shm_pools) == 0 {
        let mut pool: *mut cairo_xcb_shm_mem_pool_t = 0 as *mut cairo_xcb_shm_mem_pool_t;
        let mut next: *mut cairo_xcb_shm_mem_pool_t = 0 as *mut cairo_xcb_shm_mem_pool_t;
        let mut head: cairo_list_t = cairo_list_t {
            next: 0 as *mut _cairo_list,
            prev: 0 as *mut _cairo_list,
        };
        cairo_list_init(&mut head);
        cairo_list_move((*connection).shm_pools.next, &mut head);
        pool = ({
            let mut mptr__: *const cairo_list_t = (*connection).shm_pools.next;
            (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
                as *mut cairo_xcb_shm_mem_pool_t
        });
        next = ({
            let mut mptr__: *const cairo_list_t = (*pool).link.next;
            (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
                as *mut cairo_xcb_shm_mem_pool_t
        });
        while &mut (*pool).link as *mut cairo_list_t
            != &mut (*connection).shm_pools as *mut cairo_list_t
        {
            if (*pool).mem.free_bytes == (*pool).mem.max_bytes {
                _cairo_xcb_connection_shm_detach(connection, (*pool).shmseg);
                _cairo_xcb_shm_mem_pool_destroy(pool);
            }
            pool = next;
            next = ({
                let mut mptr__: *const cairo_list_t = (*next).link.next;
                (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
                    as *mut cairo_xcb_shm_mem_pool_t
            });
        }
        cairo_list_move(head.next, &mut (*connection).shm_pools);
    }
}
unsafe extern "C" fn _cairo_xcb_shm_process_pending(
    mut connection: *mut cairo_xcb_connection_t,
    mut wait: shm_wait_type_t,
) {
    let mut info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut next: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut reply: *mut xcb_get_input_focus_reply_t = 0
        as *mut xcb_get_input_focus_reply_t;
    info = ({
        let mut mptr__: *const cairo_list_t = (*connection).shm_pending.next;
        (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
            as *mut cairo_xcb_shm_info_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*info).pending.next;
        (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
            as *mut cairo_xcb_shm_info_t
    });
    while &mut (*info).pending as *mut cairo_list_t
        != &mut (*connection).shm_pending as *mut cairo_list_t
    {
        match wait as libc::c_uint {
            0 => {
                reply = xcb_wait_for_reply(
                    (*connection).xcb_connection,
                    (*info).sync.sequence,
                    0 as *mut *mut xcb_generic_error_t,
                ) as *mut xcb_get_input_focus_reply_t;
            }
            1 => {
                if xcb_poll_for_reply(
                    (*connection).xcb_connection,
                    (*info).sync.sequence,
                    &mut reply as *mut *mut xcb_get_input_focus_reply_t
                        as *mut *mut libc::c_void,
                    0 as *mut *mut xcb_generic_error_t,
                ) == 0
                {
                    return;
                }
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-xcb-shm.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 79],
                            &[libc::c_char; 79],
                        >(
                            b"void _cairo_xcb_shm_process_pending(cairo_xcb_connection_t *, shm_wait_type_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                reply = 0 as *mut xcb_get_input_focus_reply_t;
            }
        }
        free(reply as *mut libc::c_void);
        cairo_list_del(&mut (*info).pending);
        _cairo_xcb_shm_info_finalize(info);
        info = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).pending.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut cairo_xcb_shm_info_t
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_allocate_shm_info(
    mut connection: *mut cairo_xcb_connection_t,
    mut size: size_t,
    mut might_reuse: cairo_bool_t,
    mut shm_info_out: *mut *mut cairo_xcb_shm_info_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut pool: *mut cairo_xcb_shm_mem_pool_t = 0 as *mut cairo_xcb_shm_mem_pool_t;
    let mut next: *mut cairo_xcb_shm_mem_pool_t = 0 as *mut cairo_xcb_shm_mem_pool_t;
    let mut bytes: size_t = 0;
    let mut maxbits: size_t = 16 as libc::c_int as size_t;
    let mut minbits: size_t = 8 as libc::c_int as size_t;
    let mut shm_allocated: size_t = 0 as libc::c_int as size_t;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*connection).flags & CAIRO_XCB_HAS_SHM as libc::c_uint != 0 {} else {
        __assert_fail(
            b"connection->flags & CAIRO_XCB_HAS_SHM\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-shm.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 132],
                &[libc::c_char; 132],
            >(
                b"cairo_int_status_t _cairo_xcb_connection_allocate_shm_info(cairo_xcb_connection_t *, size_t, cairo_bool_t, cairo_xcb_shm_info_t **)\0",
            ))
                .as_ptr(),
        );
    }
    pthread_mutex_lock(&mut (*connection).shm_mutex);
    _cairo_xcb_shm_process_pending(connection, PENDING_POLL);
    if might_reuse != 0 {
        shm_info = ({
            let mut mptr__: *const cairo_list_t = (*connection).shm_pending.next;
            (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                as *mut cairo_xcb_shm_info_t
        });
        while &mut (*shm_info).pending as *mut cairo_list_t
            != &mut (*connection).shm_pending as *mut cairo_list_t
        {
            if (*shm_info).size >= size {
                cairo_list_del(&mut (*shm_info).pending);
                pthread_mutex_unlock(&mut (*connection).shm_mutex);
                xcb_discard_reply(
                    (*connection).xcb_connection,
                    (*shm_info).sync.sequence,
                );
                (*shm_info).sync.sequence = 0 as libc::c_long as libc::c_uint;
                *shm_info_out = shm_info;
                return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            }
            shm_info = ({
                let mut mptr__: *const cairo_list_t = (*shm_info).pending.next;
                (mptr__ as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize))
                    as *mut cairo_xcb_shm_info_t
            });
        }
    }
    pool = ({
        let mut mptr__: *const cairo_list_t = (*connection).shm_pools.next;
        (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
            as *mut cairo_xcb_shm_mem_pool_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*pool).link.next;
        (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
            as *mut cairo_xcb_shm_mem_pool_t
    });
    loop {
        if !(&mut (*pool).link as *mut cairo_list_t
            != &mut (*connection).shm_pools as *mut cairo_list_t)
        {
            current_block = 8693738493027456495;
            break;
        }
        if (*pool).mem.free_bytes > size {
            mem = _cairo_mempool_alloc(&mut (*pool).mem, size);
            if !mem.is_null() {
                cairo_list_move(&mut (*pool).link, &mut (*connection).shm_pools);
                current_block = 14854073251454018400;
                break;
            }
        }
        if (*pool).mem.free_bytes == (*pool).mem.max_bytes {
            _cairo_xcb_connection_shm_detach(connection, (*pool).shmseg);
            _cairo_xcb_shm_mem_pool_destroy(pool);
        } else {
            shm_allocated = (shm_allocated as libc::c_ulong)
                .wrapping_add((*pool).mem.max_bytes) as size_t as size_t;
        }
        pool = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
                as *mut cairo_xcb_shm_mem_pool_t
        });
    }
    match current_block {
        8693738493027456495 => {
            if shm_allocated
                >= (16 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                    as libc::c_ulong
            {
                pthread_mutex_unlock(&mut (*connection).shm_mutex);
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            pool = (if ::std::mem::size_of::<cairo_xcb_shm_mem_pool_t>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
            {
                malloc(
                    ::std::mem::size_of::<cairo_xcb_shm_mem_pool_t>() as libc::c_ulong,
                )
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_xcb_shm_mem_pool_t;
            if pool.is_null() {
                pthread_mutex_unlock(&mut (*connection).shm_mutex);
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            bytes = ((1 as libc::c_int) << maxbits) as size_t;
            while bytes <= size {
                bytes <<= 1 as libc::c_int;
                maxbits = maxbits.wrapping_add(1);
            }
            bytes <<= 3 as libc::c_int;
            loop {
                (*pool)
                    .shmid = shmget(
                    0 as libc::c_int,
                    bytes,
                    0o1000 as libc::c_int | 0o600 as libc::c_int,
                );
                if (*pool).shmid != -(1 as libc::c_int) {
                    break;
                }
                bytes >>= 1 as libc::c_int;
                if *__errno_location() != 22 as libc::c_int || bytes < size {
                    break;
                }
            }
            if (*pool).shmid == -(1 as libc::c_int) {
                let mut err: libc::c_int = *__errno_location();
                if !(err == 22 as libc::c_int || err == 12 as libc::c_int) {
                    (*connection).flags &= !(CAIRO_XCB_HAS_SHM as libc::c_uint);
                }
                free(pool as *mut libc::c_void);
                pthread_mutex_unlock(&mut (*connection).shm_mutex);
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            let ref mut fresh13 = (*pool).shm;
            *fresh13 = shmat((*pool).shmid, 0 as *const libc::c_void, 0 as libc::c_int);
            if (*pool).shm
                == -(1 as libc::c_int) as *mut libc::c_char as *mut libc::c_void
            {
                shmctl((*pool).shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
                free(pool as *mut libc::c_void);
                pthread_mutex_unlock(&mut (*connection).shm_mutex);
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            status = _cairo_mempool_init(
                &mut (*pool).mem,
                (*pool).shm,
                bytes,
                minbits as libc::c_int,
                maxbits
                    .wrapping_sub(minbits)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            );
            if status as u64 != 0 {
                shmdt((*pool).shm);
                free(pool as *mut libc::c_void);
                pthread_mutex_unlock(&mut (*connection).shm_mutex);
                return status as cairo_int_status_t;
            }
            (*pool)
                .shmseg = _cairo_xcb_connection_shm_attach(
                connection,
                (*pool).shmid as uint32_t,
                0 as libc::c_int,
            );
            shmctl((*pool).shmid, 0 as libc::c_int, 0 as *mut shmid_ds);
            cairo_list_add(&mut (*pool).link, &mut (*connection).shm_pools);
            mem = _cairo_mempool_alloc(&mut (*pool).mem, size);
        }
        _ => {}
    }
    shm_info = _cairo_freepool_alloc(&mut (*connection).shm_info_freelist)
        as *mut cairo_xcb_shm_info_t;
    if shm_info.is_null() {
        _cairo_mempool_free(&mut (*pool).mem, mem);
        pthread_mutex_unlock(&mut (*connection).shm_mutex);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    let ref mut fresh14 = (*shm_info).connection;
    *fresh14 = connection;
    let ref mut fresh15 = (*shm_info).pool;
    *fresh15 = pool;
    (*shm_info).shm = (*pool).shmseg;
    (*shm_info).size = size;
    (*shm_info)
        .offset = (mem as *mut libc::c_char)
        .offset_from((*pool).shm as *mut libc::c_char) as libc::c_long as uint32_t;
    let ref mut fresh16 = (*shm_info).mem;
    *fresh16 = mem;
    (*shm_info).sync.sequence = 0 as libc::c_long as libc::c_uint;
    pool = ({
        let mut mptr__: *const cairo_list_t = (*connection).shm_pools.next;
        (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
            as *mut cairo_xcb_shm_mem_pool_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*pool).link.next;
        (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
            as *mut cairo_xcb_shm_mem_pool_t
    });
    while &mut (*pool).link as *mut cairo_list_t
        != &mut (*connection).shm_pools as *mut cairo_list_t
    {
        if (*pool).mem.free_bytes == (*pool).mem.max_bytes {
            _cairo_xcb_connection_shm_detach(connection, (*pool).shmseg);
            _cairo_xcb_shm_mem_pool_destroy(pool);
        }
        pool = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
                as *mut cairo_xcb_shm_mem_pool_t
        });
    }
    pthread_mutex_unlock(&mut (*connection).shm_mutex);
    *shm_info_out = shm_info;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_shm_info_destroy(
    mut shm_info: *mut cairo_xcb_shm_info_t,
) {
    let mut connection: *mut cairo_xcb_connection_t = (*shm_info).connection;
    pthread_mutex_lock(&mut (*connection).shm_mutex);
    if (*shm_info).sync.sequence as libc::c_long == 0 as libc::c_long {} else {
        __assert_fail(
            b"shm_info->sync.sequence == XCB_NONE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-shm.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void _cairo_xcb_shm_info_destroy(cairo_xcb_shm_info_t *)\0"))
                .as_ptr(),
        );
    }
    (*shm_info).sync = xcb_get_input_focus((*connection).xcb_connection);
    cairo_list_init(&mut (*shm_info).pending);
    cairo_list_add_tail(&mut (*shm_info).pending, &mut (*connection).shm_pending);
    pthread_mutex_unlock(&mut (*connection).shm_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_shm_mem_pools_flush(
    mut connection: *mut cairo_xcb_connection_t,
) {
    pthread_mutex_lock(&mut (*connection).shm_mutex);
    _cairo_xcb_shm_process_pending(connection, PENDING_WAIT);
    pthread_mutex_unlock(&mut (*connection).shm_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_connection_shm_mem_pools_fini(
    mut connection: *mut cairo_xcb_connection_t,
) {
    if cairo_list_is_empty(&mut (*connection).shm_pending) != 0 {} else {
        __assert_fail(
            b"cairo_list_is_empty (&connection->shm_pending)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-shm.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void _cairo_xcb_connection_shm_mem_pools_fini(cairo_xcb_connection_t *)\0",
            ))
                .as_ptr(),
        );
    }
    while cairo_list_is_empty(&mut (*connection).shm_pools) == 0 {
        _cairo_xcb_shm_mem_pool_destroy(
            ({
                let mut mptr__: *const cairo_list_t = (*connection).shm_pools.next;
                (mptr__ as *mut libc::c_char).offset(-(584 as libc::c_ulong as isize))
                    as *mut cairo_xcb_shm_mem_pool_t
            }),
        );
    }
}
