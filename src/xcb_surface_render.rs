use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo;
    pub type _cairo_damage;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type xcb_connection_t;
    pub type _cairo_xcb_shm_mem_pool;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _cairo_clip_copy(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _cairo_clip_copy_region(clip: *const cairo_clip_t) -> *mut cairo_clip_t;
    fn _cairo_bentley_ottmann_tessellate_rectilinear_traps(
        traps: *mut cairo_traps_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_bentley_ottmann_tessellate_boxes(
        in_0: *const cairo_boxes_t,
        fill_rule: cairo_fill_rule_t,
        out: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_bentley_ottmann_tessellate_rectangular_traps(
        traps: *mut cairo_traps_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_bentley_ottmann_tessellate_traps(
        traps: *mut cairo_traps_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_bentley_ottmann_tessellate_polygon(
        traps: *mut cairo_traps_t,
        polygon: *const cairo_polygon_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_status_t;
    fn _cairo_matrix_to_pixman_matrix_offset(
        matrix: *const cairo_matrix_t,
        filter: cairo_filter_t,
        xc: libc::c_double,
        yc: libc::c_double,
        out_transform: *mut pixman_transform_t,
        out_x_offset: *mut libc::c_int,
        out_y_offset: *mut libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_matrix_is_pixel_exact(matrix: *const cairo_matrix_t) -> cairo_bool_t;
    fn _cairo_matrix_is_integer_translation(
        matrix: *const cairo_matrix_t,
        itx: *mut libc::c_int,
        ity: *mut libc::c_int,
    ) -> cairo_bool_t;
    fn _cairo_matrix_transform_bounding_box(
        matrix: *const cairo_matrix_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_boxes_init(boxes: *mut cairo_boxes_t);
    fn _cairo_boxes_init_with_clip(boxes: *mut cairo_boxes_t, clip: *mut cairo_clip_t);
    fn _cairo_boxes_init_for_array(
        boxes: *mut cairo_boxes_t,
        array: *mut cairo_box_t,
        num_boxes: libc::c_int,
    );
    fn _cairo_boxes_add(
        boxes: *mut cairo_boxes_t,
        antialias: cairo_antialias_t,
        box_0: *const cairo_box_t,
    ) -> cairo_status_t;
    fn _cairo_boxes_fini(boxes: *mut cairo_boxes_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_clip_path_destroy(clip_path: *mut cairo_clip_path_t);
    fn _cairo_clip_destroy(clip: *mut cairo_clip_t);
    static __cairo_clip_all: cairo_clip_t;
    fn _cairo_clip_intersect_boxes(
        clip: *mut cairo_clip_t,
        boxes: *const cairo_boxes_t,
    ) -> *mut cairo_clip_t;
    fn _cairo_clip_combine_with_surface(
        clip: *const cairo_clip_t,
        dst: *mut cairo_surface_t,
        dst_x: libc::c_int,
        dst_y: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_clip_get_region(clip: *const cairo_clip_t) -> *mut cairo_region_t;
    fn _cairo_clip_is_region(clip: *const cairo_clip_t) -> cairo_bool_t;
    fn _cairo_clip_get_polygon(
        clip: *const cairo_clip_t,
        polygon: *mut cairo_polygon_t,
        fill_rule: *mut cairo_fill_rule_t,
        antialias: *mut cairo_antialias_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_surface_get_source(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_scaled_font_find_private(
        scaled_font: *mut cairo_scaled_font_t,
        key: *const libc::c_void,
    ) -> *mut cairo_scaled_font_private_t;
    fn _cairo_scaled_font_attach_private(
        scaled_font: *mut cairo_scaled_font_t,
        priv_0: *mut cairo_scaled_font_private_t,
        key: *const libc::c_void,
        destroy: Option::<
            unsafe extern "C" fn(
                *mut cairo_scaled_font_private_t,
                *mut cairo_scaled_font_t,
            ) -> (),
        >,
    );
    fn _cairo_scaled_glyph_attach_private(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        priv_0: *mut cairo_scaled_glyph_private_t,
        key: *const libc::c_void,
        destroy: Option::<
            unsafe extern "C" fn(
                *mut cairo_scaled_glyph_private_t,
                *mut cairo_scaled_glyph_t,
                *mut cairo_scaled_font_t,
            ) -> (),
        >,
    );
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_stock_color(stock: cairo_stock_t) -> *const cairo_color_t;
    fn _cairo_color_equal(
        color_a: *const cairo_color_t,
        color_b: *const cairo_color_t,
    ) -> cairo_bool_t;
    fn _cairo_path_fixed_fill_to_polygon(
        path: *const cairo_path_fixed_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_fill_rectilinear_to_boxes(
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_to_polygon(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        polygon: *mut cairo_polygon_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_stroke_rectilinear_to_boxes(
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        antialias: cairo_antialias_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_int_status_t;
    fn _cairo_scaled_font_freeze_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_thaw_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_font_reset_cache(scaled_font: *mut cairo_scaled_font_t);
    fn _cairo_scaled_glyph_set_surface(
        scaled_glyph: *mut cairo_scaled_glyph_t,
        scaled_font: *mut cairo_scaled_font_t,
        surface: *mut cairo_image_surface_t,
    );
    fn _cairo_scaled_glyph_lookup(
        scaled_font: *mut cairo_scaled_font_t,
        index: libc::c_ulong,
        info: cairo_scaled_glyph_info_t,
        foreground_color: *const cairo_color_t,
        scaled_glyph_ret: *mut *mut cairo_scaled_glyph_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_create_scratch(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
        color: *const cairo_color_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_init(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
        device: *mut cairo_device_t,
        content: cairo_content_t,
        is_vector: cairo_bool_t,
    );
    fn _cairo_surface_paint(
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_acquire_source_image(
        surface: *mut cairo_surface_t,
        image_out: *mut *mut cairo_image_surface_t,
        image_extra: *mut *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_surface_release_source_image(
        surface: *mut cairo_surface_t,
        image: *mut cairo_image_surface_t,
        image_extra: *mut libc::c_void,
    );
    fn _cairo_surface_has_snapshot(
        surface: *mut cairo_surface_t,
        backend: *const cairo_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_format_from_content(content: cairo_content_t) -> cairo_format_t;
    fn _cairo_content_from_pixman_format(
        pixman_format: pixman_format_code_t,
    ) -> cairo_content_t;
    fn _cairo_image_surface_coerce(
        surface: *mut cairo_image_surface_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_image_surface_coerce_to_format(
        surface: *mut cairo_image_surface_t,
        format: cairo_format_t,
    ) -> *mut cairo_image_surface_t;
    fn _cairo_polygon_init_with_clip(
        polygon: *mut cairo_polygon_t,
        clip: *const cairo_clip_t,
    );
    fn _cairo_polygon_fini(polygon: *mut cairo_polygon_t);
    fn _cairo_polygon_intersect(
        a: *mut cairo_polygon_t,
        winding_a: libc::c_int,
        b: *mut cairo_polygon_t,
        winding_b: libc::c_int,
    ) -> cairo_status_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_device_destroy(device: *mut cairo_device_t);
    fn cairo_device_release(device: *mut cairo_device_t);
    fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    fn cairo_matrix_init_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn cairo_matrix_translate(
        matrix: *mut cairo_matrix_t,
        tx: libc::c_double,
        ty: libc::c_double,
    );
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn cairo_region_contains_rectangle(
        region: *const cairo_region_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_region_overlap_t;
    fn cairo_region_num_rectangles(region: *const cairo_region_t) -> libc::c_int;
    fn cairo_region_get_rectangle(
        region: *const cairo_region_t,
        nth: libc::c_int,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn xcb_free_pixmap(
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn _cairo_xcb_connection_render_free_glyph_set(
        connection: *mut cairo_xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
    );
    fn _cairo_xcb_connection_render_create_glyph_set(
        connection: *mut cairo_xcb_connection_t,
        id: xcb_render_glyphset_t,
        format: xcb_render_pictformat_t,
    );
    fn _cairo_xcb_connection_render_add_glyphs(
        connection: *mut cairo_xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        num_glyphs: uint32_t,
        glyphs_id: *mut uint32_t,
        glyphs: *mut xcb_render_glyphinfo_t,
        data_len: uint32_t,
        data: *mut uint8_t,
    );
    fn _cairo_xcb_connection_render_free_glyphs(
        connection: *mut cairo_xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        num_glyphs: uint32_t,
        glyphs: *mut xcb_render_glyph_t,
    );
    fn _cairo_xcb_connection_render_composite_glyphs_8(
        connection: *mut cairo_xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: int16_t,
        src_y: int16_t,
        glyphcmds_len: uint32_t,
        glyphcmds: *mut uint8_t,
    );
    fn _cairo_pattern_init_static_copy(
        pattern: *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    );
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_is_opaque_solid(pattern: *const cairo_pattern_t) -> cairo_bool_t;
    fn _cairo_pattern_is_opaque(
        pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_gradient_pattern_fit_to_range(
        gradient: *const cairo_gradient_pattern_t,
        max_value: libc::c_double,
        out_matrix: *mut cairo_matrix_t,
        out_circle: *mut cairo_circle_double_t,
    );
    fn _cairo_radial_pattern_focus_is_inside(
        radial: *const cairo_radial_pattern_t,
    ) -> cairo_bool_t;
    static _cairo_xcb_surface_backend: cairo_surface_backend_t;
    fn _cairo_xcb_connection_render_composite_glyphs_16(
        connection: *mut cairo_xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: int16_t,
        src_y: int16_t,
        glyphcmds_len: uint32_t,
        glyphcmds: *mut uint8_t,
    );
    fn _cairo_xcb_connection_render_composite_glyphs_32(
        connection: *mut cairo_xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: int16_t,
        src_y: int16_t,
        glyphcmds_len: uint32_t,
        glyphcmds: *mut uint8_t,
    );
    fn _cairo_xcb_surface_core_copy_boxes(
        dst: *mut cairo_xcb_surface_t,
        src_pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
        boxes: *const cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_surface_core_fill_boxes(
        dst: *mut cairo_xcb_surface_t,
        color: *const cairo_color_t,
        boxes: *mut cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_connection_render_composite(
        connection: *mut cairo_xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        mask: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        src_x: int16_t,
        src_y: int16_t,
        mask_x: int16_t,
        mask_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    );
    fn _cairo_xcb_screen_get_gc(
        screen: *mut cairo_xcb_screen_t,
        drawable: xcb_drawable_t,
        depth: libc::c_int,
    ) -> xcb_gcontext_t;
    fn _cairo_xcb_screen_put_gc(
        screen: *mut cairo_xcb_screen_t,
        depth: libc::c_int,
        gc: xcb_gcontext_t,
    );
    fn _cairo_xcb_screen_store_linear_picture(
        screen: *mut cairo_xcb_screen_t,
        linear: *const cairo_linear_pattern_t,
        picture: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_screen_lookup_linear_picture(
        screen: *mut cairo_xcb_screen_t,
        linear: *const cairo_linear_pattern_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_xcb_screen_store_radial_picture(
        screen: *mut cairo_xcb_screen_t,
        radial: *const cairo_radial_pattern_t,
        picture: *mut cairo_surface_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_screen_lookup_radial_picture(
        screen: *mut cairo_xcb_screen_t,
        radial: *const cairo_radial_pattern_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_xcb_surface_create_similar_image(
        abstrct_other: *mut libc::c_void,
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_xcb_surface_create_similar(
        abstract_other: *mut libc::c_void,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_xcb_connection_render_create_linear_gradient(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        p1: xcb_render_pointfix_t,
        p2: xcb_render_pointfix_t,
        num_stops: uint32_t,
        stops: *mut xcb_render_fixed_t,
        colors: *mut xcb_render_color_t,
    );
    fn _cairo_xcb_connection_render_create_radial_gradient(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        inner: xcb_render_pointfix_t,
        outer: xcb_render_pointfix_t,
        inner_radius: xcb_render_fixed_t,
        outer_radius: xcb_render_fixed_t,
        num_stops: uint32_t,
        stops: *mut xcb_render_fixed_t,
        colors: *mut xcb_render_color_t,
    );
    fn _cairo_xcb_connection_render_set_picture_clip_rectangles(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        clip_x_origin: int16_t,
        clip_y_origin: int16_t,
        rectangles_len: uint32_t,
        rectangles: *mut xcb_rectangle_t,
    );
    fn _cairo_xcb_connection_render_create_solid_fill(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        color: xcb_render_color_t,
    );
    fn _cairo_xcb_connection_render_trapezoids(
        connection: *mut cairo_xcb_connection_t,
        op: uint8_t,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: int16_t,
        src_y: int16_t,
        traps_len: uint32_t,
        traps: *mut xcb_render_trapezoid_t,
    );
    fn _cairo_xcb_connection_render_change_picture(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: uint32_t,
        value_list: *mut uint32_t,
    );
    fn _cairo_xcb_connection_render_fill_rectangles(
        connection: *mut cairo_xcb_connection_t,
        op: uint8_t,
        dst: xcb_render_picture_t,
        color: xcb_render_color_t,
        num_rects: uint32_t,
        rects: *mut xcb_rectangle_t,
    );
    fn _cairo_xcb_connection_poly_fill_rectangle(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        num_rectangles: uint32_t,
        rectangles: *mut xcb_rectangle_t,
    );
    fn _cairo_xcb_connection_render_set_picture_filter(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        filter_len: uint16_t,
        filter: *mut libc::c_char,
    );
    fn _cairo_xcb_connection_render_set_picture_transform(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        transform: *mut xcb_render_transform_t,
    );
    fn _cairo_xcb_shm_image_create(
        connection: *mut cairo_xcb_connection_t,
        pixman_format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        image_out: *mut *mut cairo_image_surface_t,
        shm_info_out: *mut *mut cairo_xcb_shm_info_t,
    ) -> cairo_status_t;
    fn _cairo_xcb_connection_create_pixmap(
        connection: *mut cairo_xcb_connection_t,
        depth: uint8_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_pixmap_t;
    fn _cairo_xcb_connection_shm_put_image(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        total_width: uint16_t,
        total_height: uint16_t,
        src_x: int16_t,
        src_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        shm: uint32_t,
        offset: uint32_t,
    );
    fn _cairo_xcb_connection_render_create_picture(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: uint32_t,
        value_list: *mut uint32_t,
    );
    fn _cairo_xcb_connection_put_image(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: uint16_t,
        height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        length: uint32_t,
        data: *mut libc::c_void,
    );
    fn _cairo_xcb_connection_put_subimage(
        connection: *mut cairo_xcb_connection_t,
        dst: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        width: uint16_t,
        height: uint16_t,
        cpp: uint16_t,
        stride: libc::c_int,
        dst_x: int16_t,
        dst_y: int16_t,
        depth: uint8_t,
        data: *mut libc::c_void,
    );
    fn _cairo_xcb_connection_render_free_picture(
        connection: *mut cairo_xcb_connection_t,
        picture: xcb_render_picture_t,
    );
    fn _cairo_composite_rectangles_intersect_mask_extents(
        extents: *mut cairo_composite_rectangles_t,
        box_0: *const cairo_box_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_offset_stroke(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        stroke_style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_fill(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        antialias: cairo_antialias_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_offset_glyphs(
        surface: *mut cairo_surface_t,
        x: libc::c_int,
        y: libc::c_int,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_traps_init(traps: *mut cairo_traps_t);
    fn _cairo_traps_init_boxes(
        traps: *mut cairo_traps_t,
        boxes: *const cairo_boxes_t,
    ) -> cairo_status_t;
    fn _cairo_traps_fini(traps: *mut cairo_traps_t);
    fn _cairo_traps_extents(traps: *const cairo_traps_t, extents: *mut cairo_box_t);
    fn _cairo_recording_surface_replay_with_clip(
        surface: *mut cairo_surface_t,
        surface_transform: *const cairo_matrix_t,
        target: *mut cairo_surface_t,
        target_clip: *const cairo_clip_t,
    ) -> cairo_status_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type ptrdiff_t = libc::c_long;
pub type cairo_bool_t = libc::c_int;
pub type cairo_t = _cairo;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_surface {
    pub backend: *const cairo_surface_backend_t,
    pub device: *mut cairo_device_t,
    pub type_0: cairo_surface_type_t,
    pub content: cairo_content_t,
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub unique_id: libc::c_uint,
    pub serial: libc::c_uint,
    pub damage: *mut cairo_damage_t,
    #[bitfield(name = "_finishing", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "finished", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_clear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "has_font_options", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "owns_device", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "is_vector", ty = "libc::c_uint", bits = "5..=5")]
    pub _finishing_finished_is_clear_has_font_options_owns_device_is_vector: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub user_data: cairo_user_data_array_t,
    pub mime_data: cairo_user_data_array_t,
    pub device_transform: cairo_matrix_t,
    pub device_transform_inverse: cairo_matrix_t,
    pub device_transform_observers: cairo_list_t,
    pub x_resolution: libc::c_double,
    pub y_resolution: libc::c_double,
    pub x_fallback_resolution: libc::c_double,
    pub y_fallback_resolution: libc::c_double,
    pub snapshot_of: *mut cairo_surface_t,
    pub snapshot_detach: cairo_surface_func_t,
    pub snapshots: cairo_list_t,
    pub snapshot: cairo_list_t,
    pub font_options: cairo_font_options_t,
}
pub type cairo_font_options_t = _cairo_font_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_font_options {
    pub antialias: cairo_antialias_t,
    pub subpixel_order: cairo_subpixel_order_t,
    pub lcd_filter: cairo_lcd_filter_t,
    pub hint_style: cairo_hint_style_t,
    pub hint_metrics: cairo_hint_metrics_t,
    pub round_glyph_positions: cairo_round_glyph_positions_t,
    pub variations: *mut libc::c_char,
    pub color_mode: cairo_color_mode_t,
    pub palette_index: libc::c_uint,
}
pub type cairo_color_mode_t = _cairo_color_mode;
pub type _cairo_color_mode = libc::c_uint;
pub const CAIRO_COLOR_MODE_COLOR: _cairo_color_mode = 2;
pub const CAIRO_COLOR_MODE_NO_COLOR: _cairo_color_mode = 1;
pub const CAIRO_COLOR_MODE_DEFAULT: _cairo_color_mode = 0;
pub type cairo_round_glyph_positions_t = _cairo_round_glyph_positions;
pub type _cairo_round_glyph_positions = libc::c_uint;
pub const CAIRO_ROUND_GLYPH_POS_OFF: _cairo_round_glyph_positions = 2;
pub const CAIRO_ROUND_GLYPH_POS_ON: _cairo_round_glyph_positions = 1;
pub const CAIRO_ROUND_GLYPH_POS_DEFAULT: _cairo_round_glyph_positions = 0;
pub type cairo_hint_metrics_t = _cairo_hint_metrics;
pub type _cairo_hint_metrics = libc::c_uint;
pub const CAIRO_HINT_METRICS_ON: _cairo_hint_metrics = 2;
pub const CAIRO_HINT_METRICS_OFF: _cairo_hint_metrics = 1;
pub const CAIRO_HINT_METRICS_DEFAULT: _cairo_hint_metrics = 0;
pub type cairo_hint_style_t = _cairo_hint_style;
pub type _cairo_hint_style = libc::c_uint;
pub const CAIRO_HINT_STYLE_FULL: _cairo_hint_style = 4;
pub const CAIRO_HINT_STYLE_MEDIUM: _cairo_hint_style = 3;
pub const CAIRO_HINT_STYLE_SLIGHT: _cairo_hint_style = 2;
pub const CAIRO_HINT_STYLE_NONE: _cairo_hint_style = 1;
pub const CAIRO_HINT_STYLE_DEFAULT: _cairo_hint_style = 0;
pub type cairo_lcd_filter_t = _cairo_lcd_filter;
pub type _cairo_lcd_filter = libc::c_uint;
pub const CAIRO_LCD_FILTER_FIR5: _cairo_lcd_filter = 4;
pub const CAIRO_LCD_FILTER_FIR3: _cairo_lcd_filter = 3;
pub const CAIRO_LCD_FILTER_INTRA_PIXEL: _cairo_lcd_filter = 2;
pub const CAIRO_LCD_FILTER_NONE: _cairo_lcd_filter = 1;
pub const CAIRO_LCD_FILTER_DEFAULT: _cairo_lcd_filter = 0;
pub type cairo_subpixel_order_t = _cairo_subpixel_order;
pub type _cairo_subpixel_order = libc::c_uint;
pub const CAIRO_SUBPIXEL_ORDER_VBGR: _cairo_subpixel_order = 4;
pub const CAIRO_SUBPIXEL_ORDER_VRGB: _cairo_subpixel_order = 3;
pub const CAIRO_SUBPIXEL_ORDER_BGR: _cairo_subpixel_order = 2;
pub const CAIRO_SUBPIXEL_ORDER_RGB: _cairo_subpixel_order = 1;
pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: _cairo_subpixel_order = 0;
pub type cairo_antialias_t = _cairo_antialias;
pub type _cairo_antialias = libc::c_uint;
pub const CAIRO_ANTIALIAS_BEST: _cairo_antialias = 6;
pub const CAIRO_ANTIALIAS_GOOD: _cairo_antialias = 5;
pub const CAIRO_ANTIALIAS_FAST: _cairo_antialias = 4;
pub const CAIRO_ANTIALIAS_SUBPIXEL: _cairo_antialias = 3;
pub const CAIRO_ANTIALIAS_GRAY: _cairo_antialias = 2;
pub const CAIRO_ANTIALIAS_NONE: _cairo_antialias = 1;
pub const CAIRO_ANTIALIAS_DEFAULT: _cairo_antialias = 0;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
pub type cairo_surface_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_surface_t) -> (),
>;
pub type cairo_surface_t = _cairo_surface;
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
pub type cairo_damage_t = _cairo_damage;
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
pub type cairo_content_t = _cairo_content;
pub type _cairo_content = libc::c_uint;
pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;
pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
pub type cairo_surface_type_t = _cairo_surface_type;
pub type _cairo_surface_type = libc::c_uint;
pub const CAIRO_SURFACE_TYPE_COGL: _cairo_surface_type = 24;
pub const CAIRO_SURFACE_TYPE_SUBSURFACE: _cairo_surface_type = 23;
pub const CAIRO_SURFACE_TYPE_SKIA: _cairo_surface_type = 22;
pub const CAIRO_SURFACE_TYPE_XML: _cairo_surface_type = 21;
pub const CAIRO_SURFACE_TYPE_TEE: _cairo_surface_type = 20;
pub const CAIRO_SURFACE_TYPE_DRM: _cairo_surface_type = 19;
pub const CAIRO_SURFACE_TYPE_GL: _cairo_surface_type = 18;
pub const CAIRO_SURFACE_TYPE_VG: _cairo_surface_type = 17;
pub const CAIRO_SURFACE_TYPE_RECORDING: _cairo_surface_type = 16;
pub const CAIRO_SURFACE_TYPE_QT: _cairo_surface_type = 15;
pub const CAIRO_SURFACE_TYPE_SCRIPT: _cairo_surface_type = 14;
pub const CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: _cairo_surface_type = 13;
pub const CAIRO_SURFACE_TYPE_WIN32_PRINTING: _cairo_surface_type = 12;
pub const CAIRO_SURFACE_TYPE_OS2: _cairo_surface_type = 11;
pub const CAIRO_SURFACE_TYPE_SVG: _cairo_surface_type = 10;
pub const CAIRO_SURFACE_TYPE_DIRECTFB: _cairo_surface_type = 9;
pub const CAIRO_SURFACE_TYPE_BEOS: _cairo_surface_type = 8;
pub const CAIRO_SURFACE_TYPE_WIN32: _cairo_surface_type = 7;
pub const CAIRO_SURFACE_TYPE_QUARTZ: _cairo_surface_type = 6;
pub const CAIRO_SURFACE_TYPE_GLITZ: _cairo_surface_type = 5;
pub const CAIRO_SURFACE_TYPE_XCB: _cairo_surface_type = 4;
pub const CAIRO_SURFACE_TYPE_XLIB: _cairo_surface_type = 3;
pub const CAIRO_SURFACE_TYPE_PS: _cairo_surface_type = 2;
pub const CAIRO_SURFACE_TYPE_PDF: _cairo_surface_type = 1;
pub const CAIRO_SURFACE_TYPE_IMAGE: _cairo_surface_type = 0;
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
pub type cairo_surface_backend_t = _cairo_surface_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_backend {
    pub type_0: cairo_surface_type_t,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t>,
    pub create_context: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
    >,
    pub create_similar: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_content_t,
            libc::c_int,
            libc::c_int,
        ) -> *mut cairo_surface_t,
    >,
    pub create_similar_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_format_t,
            libc::c_int,
            libc::c_int,
        ) -> *mut cairo_surface_t,
    >,
    pub map_to_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_rectangle_int_t,
        ) -> *mut cairo_image_surface_t,
    >,
    pub unmap_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
        ) -> cairo_int_status_t,
    >,
    pub source: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_rectangle_int_t,
        ) -> *mut cairo_surface_t,
    >,
    pub acquire_source_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *mut cairo_image_surface_t,
            *mut *mut libc::c_void,
        ) -> cairo_status_t,
    >,
    pub release_source_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
            *mut libc::c_void,
        ) -> (),
    >,
    pub snapshot: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_surface_t,
    >,
    pub copy_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub show_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub get_extents: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_rectangle_int_t,
        ) -> cairo_bool_t,
    >,
    pub get_font_options: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_font_options_t) -> (),
    >,
    pub flush: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> cairo_status_t,
    >,
    pub mark_dirty_rectangle: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> cairo_status_t,
    >,
    pub paint: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub mask: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_pattern_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub stroke: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_path_fixed_t,
            *const cairo_stroke_style_t,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub fill: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_path_fixed_t,
            cairo_fill_rule_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub fill_stroke: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            cairo_fill_rule_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_path_fixed_t,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const cairo_stroke_style_t,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            libc::c_double,
            cairo_antialias_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub show_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *mut cairo_glyph_t,
            libc::c_int,
            *mut cairo_scaled_font_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub has_show_text_glyphs: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
    pub show_text_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_operator_t,
            *const cairo_pattern_t,
            *const libc::c_char,
            libc::c_int,
            *mut cairo_glyph_t,
            libc::c_int,
            *const cairo_text_cluster_t,
            libc::c_int,
            cairo_text_cluster_flags_t,
            *mut cairo_scaled_font_t,
            *const cairo_clip_t,
        ) -> cairo_int_status_t,
    >,
    pub get_supported_mime_types: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut *const libc::c_char,
    >,
    pub tag: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_bool_t,
            *const libc::c_char,
            *const libc::c_char,
        ) -> cairo_int_status_t,
    >,
}
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
pub type cairo_clip_t = _cairo_clip;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip {
    pub extents: cairo_rectangle_int_t,
    pub path: *mut cairo_clip_path_t,
    pub boxes: *mut cairo_box_t,
    pub num_boxes: libc::c_int,
    pub region: *mut cairo_region_t,
    pub is_region: cairo_bool_t,
    pub embedded_box: cairo_box_t,
}
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
pub type cairo_region_t = _cairo_region;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_region {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub rgn: pixman_region32_t,
}
pub type pixman_region32_t = pixman_region32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32 {
    pub extents: pixman_box32_t,
    pub data: *mut pixman_region32_data_t,
}
pub type pixman_region32_data_t = pixman_region32_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_region32_data {
    pub size: libc::c_long,
    pub numRects: libc::c_long,
}
pub type pixman_box32_t = pixman_box32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_box32 {
    pub x1: int32_t,
    pub y1: int32_t,
    pub x2: int32_t,
    pub y2: int32_t,
}
pub type cairo_clip_path_t = _cairo_clip_path;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_clip_path {
    pub ref_count: cairo_reference_count_t,
    pub path: cairo_path_fixed_t,
    pub fill_rule: cairo_fill_rule_t,
    pub tolerance: libc::c_double,
    pub antialias: cairo_antialias_t,
    pub prev: *mut cairo_clip_path_t,
}
pub type cairo_fill_rule_t = _cairo_fill_rule;
pub type _cairo_fill_rule = libc::c_uint;
pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;
pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
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
pub type cairo_scaled_font_t = _cairo_scaled_font;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_scaled_font {
    pub hash_entry: cairo_hash_entry_t,
    pub status: cairo_status_t,
    pub ref_count: cairo_reference_count_t,
    pub user_data: cairo_user_data_array_t,
    pub original_font_face: *mut cairo_font_face_t,
    pub font_face: *mut cairo_font_face_t,
    pub font_matrix: cairo_matrix_t,
    pub ctm: cairo_matrix_t,
    pub options: cairo_font_options_t,
    #[bitfield(name = "placeholder", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "holdover", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "finished", ty = "libc::c_uint", bits = "2..=2")]
    pub placeholder_holdover_finished: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub scale: cairo_matrix_t,
    pub scale_inverse: cairo_matrix_t,
    pub max_scale: libc::c_double,
    pub extents: cairo_font_extents_t,
    pub fs_extents: cairo_font_extents_t,
    pub mutex: cairo_recursive_mutex_t,
    pub glyphs: *mut cairo_hash_table_t,
    pub glyph_pages: cairo_list_t,
    pub cache_frozen: cairo_bool_t,
    pub global_cache_frozen: cairo_bool_t,
    pub recording_surfaces_to_free: cairo_array_t,
    pub dev_privates: cairo_list_t,
    pub backend: *const cairo_scaled_font_backend_t,
    pub link: cairo_list_t,
}
pub type cairo_scaled_font_backend_t = _cairo_scaled_font_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_backend {
    pub type_0: cairo_font_type_t,
    pub fini: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scaled_glyph_init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_scaled_glyph_t,
            cairo_scaled_glyph_info_t,
            *const cairo_color_t,
        ) -> cairo_int_status_t,
    >,
    pub text_to_glyphs: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            *const libc::c_char,
            libc::c_int,
            *mut *mut cairo_glyph_t,
            *mut libc::c_int,
            *mut *mut cairo_text_cluster_t,
            *mut libc::c_int,
            *mut cairo_text_cluster_flags_t,
        ) -> cairo_int_status_t,
    >,
    pub ucs4_to_index: Option::<
        unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_ulong,
    >,
    pub load_truetype_table: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_ulong,
            libc::c_long,
            *mut libc::c_uchar,
            *mut libc::c_ulong,
        ) -> cairo_int_status_t,
    >,
    pub index_to_ucs4: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_ulong,
            *mut uint32_t,
        ) -> cairo_int_status_t,
    >,
    pub is_synthetic: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_bool_t) -> cairo_int_status_t,
    >,
    pub index_to_glyph_name: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *mut libc::c_char,
            libc::c_int,
            libc::c_ulong,
            *mut libc::c_ulong,
        ) -> cairo_int_status_t,
    >,
    pub load_type1_data: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_long,
            *mut libc::c_uchar,
            *mut libc::c_ulong,
        ) -> cairo_int_status_t,
    >,
    pub has_color_glyphs: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
}
pub type uint32_t = __uint32_t;
pub type cairo_text_cluster_flags_t = _cairo_text_cluster_flags;
pub type _cairo_text_cluster_flags = libc::c_uint;
pub const CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: _cairo_text_cluster_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_text_cluster_t {
    pub num_bytes: libc::c_int,
    pub num_glyphs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_glyph_t {
    pub index: libc::c_ulong,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type cairo_color_t = _cairo_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_color {
    pub red: libc::c_double,
    pub green: libc::c_double,
    pub blue: libc::c_double,
    pub alpha: libc::c_double,
    pub red_short: libc::c_ushort,
    pub green_short: libc::c_ushort,
    pub blue_short: libc::c_ushort,
    pub alpha_short: libc::c_ushort,
}
pub type cairo_scaled_glyph_info_t = _cairo_scaled_glyph_info;
pub type _cairo_scaled_glyph_info = libc::c_uint;
pub const CAIRO_SCALED_GLYPH_INFO_COLOR_SURFACE: _cairo_scaled_glyph_info = 16;
pub const CAIRO_SCALED_GLYPH_INFO_RECORDING_SURFACE: _cairo_scaled_glyph_info = 8;
pub const CAIRO_SCALED_GLYPH_INFO_PATH: _cairo_scaled_glyph_info = 4;
pub const CAIRO_SCALED_GLYPH_INFO_SURFACE: _cairo_scaled_glyph_info = 2;
pub const CAIRO_SCALED_GLYPH_INFO_METRICS: _cairo_scaled_glyph_info = 1;
pub type cairo_scaled_glyph_t = _cairo_scaled_glyph;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_scaled_glyph {
    pub hash_entry: cairo_hash_entry_t,
    pub metrics: cairo_text_extents_t,
    pub fs_metrics: cairo_text_extents_t,
    pub bbox: cairo_box_t,
    pub x_advance: int16_t,
    pub y_advance: int16_t,
    pub has_info: libc::c_uint,
    pub surface: *mut cairo_image_surface_t,
    pub path: *mut cairo_path_fixed_t,
    pub recording_surface: *mut cairo_surface_t,
    pub color_surface: *mut cairo_image_surface_t,
    pub dev_private_key: *const libc::c_void,
    pub dev_private: *mut libc::c_void,
    pub dev_privates: cairo_list_t,
    pub foreground_color: cairo_color_t,
    #[bitfield(name = "uses_foreground_color", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "color_glyph_set", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "color_glyph", ty = "libc::c_uint", bits = "2..=2")]
    pub uses_foreground_color_color_glyph_set_color_glyph: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type cairo_image_surface_t = _cairo_image_surface;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_image_surface {
    pub base: cairo_surface_t,
    pub pixman_image: *mut pixman_image_t,
    pub compositor: *const cairo_compositor_t,
    pub parent: *mut cairo_surface_t,
    pub pixman_format: pixman_format_code_t,
    pub format: cairo_format_t,
    pub data: *mut libc::c_uchar,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub stride: ptrdiff_t,
    pub depth: libc::c_int,
    #[bitfield(name = "owns_data", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "transparency", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "color", ty = "libc::c_uint", bits = "3..=4")]
    pub owns_data_transparency_color: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type cairo_format_t = _cairo_format;
pub type _cairo_format = libc::c_int;
pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub type pixman_format_code_t = libc::c_uint;
pub const PIXMAN_yv12: pixman_format_code_t = 201785344;
pub const PIXMAN_yuy2: pixman_format_code_t = 268828672;
pub const PIXMAN_g1: pixman_format_code_t = 17104896;
pub const PIXMAN_a1: pixman_format_code_t = 16846848;
pub const PIXMAN_g4: pixman_format_code_t = 67436544;
pub const PIXMAN_c4: pixman_format_code_t = 67371008;
pub const PIXMAN_a1b1g1r1: pixman_format_code_t = 67309841;
pub const PIXMAN_a1r1g1b1: pixman_format_code_t = 67244305;
pub const PIXMAN_b1g2r1: pixman_format_code_t = 67305761;
pub const PIXMAN_r1g2b1: pixman_format_code_t = 67240225;
pub const PIXMAN_a4: pixman_format_code_t = 67190784;
pub const PIXMAN_x4g4: pixman_format_code_t = 134545408;
pub const PIXMAN_x4c4: pixman_format_code_t = 134479872;
pub const PIXMAN_x4a4: pixman_format_code_t = 134299648;
pub const PIXMAN_g8: pixman_format_code_t = 134545408;
pub const PIXMAN_c8: pixman_format_code_t = 134479872;
pub const PIXMAN_a2b2g2r2: pixman_format_code_t = 134423074;
pub const PIXMAN_a2r2g2b2: pixman_format_code_t = 134357538;
pub const PIXMAN_b2g3r3: pixman_format_code_t = 134415154;
pub const PIXMAN_r3g3b2: pixman_format_code_t = 134349618;
pub const PIXMAN_a8: pixman_format_code_t = 134316032;
pub const PIXMAN_x4b4g4r4: pixman_format_code_t = 268633156;
pub const PIXMAN_a4b4g4r4: pixman_format_code_t = 268649540;
pub const PIXMAN_x4r4g4b4: pixman_format_code_t = 268567620;
pub const PIXMAN_a4r4g4b4: pixman_format_code_t = 268584004;
pub const PIXMAN_x1b5g5r5: pixman_format_code_t = 268633429;
pub const PIXMAN_a1b5g5r5: pixman_format_code_t = 268637525;
pub const PIXMAN_x1r5g5b5: pixman_format_code_t = 268567893;
pub const PIXMAN_a1r5g5b5: pixman_format_code_t = 268571989;
pub const PIXMAN_b5g6r5: pixman_format_code_t = 268633445;
pub const PIXMAN_r5g6b5: pixman_format_code_t = 268567909;
pub const PIXMAN_b8g8r8: pixman_format_code_t = 402851976;
pub const PIXMAN_r8g8b8: pixman_format_code_t = 402786440;
pub const PIXMAN_a8r8g8b8_sRGB: pixman_format_code_t = 537561224;
pub const PIXMAN_a2b10g10r10: pixman_format_code_t = 537078442;
pub const PIXMAN_x2b10g10r10: pixman_format_code_t = 537070250;
pub const PIXMAN_a2r10g10b10: pixman_format_code_t = 537012906;
pub const PIXMAN_x2r10g10b10: pixman_format_code_t = 537004714;
pub const PIXMAN_x14r6g6b6: pixman_format_code_t = 537003622;
pub const PIXMAN_r8g8b8x8: pixman_format_code_t = 537462920;
pub const PIXMAN_r8g8b8a8: pixman_format_code_t = 537495688;
pub const PIXMAN_b8g8r8x8: pixman_format_code_t = 537397384;
pub const PIXMAN_b8g8r8a8: pixman_format_code_t = 537430152;
pub const PIXMAN_x8b8g8r8: pixman_format_code_t = 537069704;
pub const PIXMAN_a8b8g8r8: pixman_format_code_t = 537102472;
pub const PIXMAN_x8r8g8b8: pixman_format_code_t = 537004168;
pub const PIXMAN_a8r8g8b8: pixman_format_code_t = 537036936;
pub const PIXMAN_rgb_float: pixman_format_code_t = 214631492;
pub const PIXMAN_rgba_float: pixman_format_code_t = 281756740;
pub type cairo_compositor_t = cairo_compositor;
pub type pixman_image_t = pixman_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_text_extents_t {
    pub x_bearing: libc::c_double,
    pub y_bearing: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
}
pub type cairo_hash_entry_t = _cairo_hash_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_hash_entry {
    pub hash: uintptr_t,
}
pub type uintptr_t = libc::c_ulong;
pub type cairo_font_type_t = _cairo_font_type;
pub type _cairo_font_type = libc::c_uint;
pub const CAIRO_FONT_TYPE_DWRITE: _cairo_font_type = 5;
pub const CAIRO_FONT_TYPE_USER: _cairo_font_type = 4;
pub const CAIRO_FONT_TYPE_QUARTZ: _cairo_font_type = 3;
pub const CAIRO_FONT_TYPE_WIN32: _cairo_font_type = 2;
pub const CAIRO_FONT_TYPE_FT: _cairo_font_type = 1;
pub const CAIRO_FONT_TYPE_TOY: _cairo_font_type = 0;
pub type cairo_hash_table_t = _cairo_hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_font_extents_t {
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub height: libc::c_double,
    pub max_x_advance: libc::c_double,
    pub max_y_advance: libc::c_double,
}
pub type cairo_font_face_t = _cairo_font_face;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_font_face {
    pub hash_entry: cairo_hash_entry_t,
    pub status: cairo_status_t,
    pub ref_count: cairo_reference_count_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_font_face_backend_t,
}
pub type cairo_font_face_backend_t = _cairo_font_face_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_font_face_backend {
    pub type_0: cairo_font_type_t,
    pub create_for_toy: Option::<
        unsafe extern "C" fn(
            *mut cairo_toy_font_face_t,
            *mut *mut cairo_font_face_t,
        ) -> cairo_status_t,
    >,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t>,
    pub scaled_font_create: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            *const cairo_font_options_t,
            *mut *mut cairo_scaled_font_t,
        ) -> cairo_status_t,
    >,
    pub get_implementation: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const cairo_matrix_t,
            *const cairo_matrix_t,
            *const cairo_font_options_t,
        ) -> *mut cairo_font_face_t,
    >,
}
pub type cairo_toy_font_face_t = _cairo_toy_font_face;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_toy_font_face {
    pub base: cairo_font_face_t,
    pub family: *const libc::c_char,
    pub owns_family: cairo_bool_t,
    pub slant: cairo_font_slant_t,
    pub weight: cairo_font_weight_t,
    pub impl_face: *mut cairo_font_face_t,
}
pub type cairo_font_weight_t = _cairo_font_weight;
pub type _cairo_font_weight = libc::c_uint;
pub const CAIRO_FONT_WEIGHT_BOLD: _cairo_font_weight = 1;
pub const CAIRO_FONT_WEIGHT_NORMAL: _cairo_font_weight = 0;
pub type cairo_font_slant_t = _cairo_font_slant;
pub type _cairo_font_slant = libc::c_uint;
pub const CAIRO_FONT_SLANT_OBLIQUE: _cairo_font_slant = 2;
pub const CAIRO_FONT_SLANT_ITALIC: _cairo_font_slant = 1;
pub const CAIRO_FONT_SLANT_NORMAL: _cairo_font_slant = 0;
pub type cairo_pattern_t = _cairo_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pattern {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub observers: cairo_list_t,
    pub type_0: cairo_pattern_type_t,
    pub filter: cairo_filter_t,
    pub extend: cairo_extend_t,
    pub has_component_alpha: cairo_bool_t,
    pub is_userfont_foreground: cairo_bool_t,
    pub matrix: cairo_matrix_t,
    pub opacity: libc::c_double,
}
pub type cairo_extend_t = _cairo_extend;
pub type _cairo_extend = libc::c_uint;
pub const CAIRO_EXTEND_PAD: _cairo_extend = 3;
pub const CAIRO_EXTEND_REFLECT: _cairo_extend = 2;
pub const CAIRO_EXTEND_REPEAT: _cairo_extend = 1;
pub const CAIRO_EXTEND_NONE: _cairo_extend = 0;
pub type cairo_filter_t = _cairo_filter;
pub type _cairo_filter = libc::c_uint;
pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;
pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
pub type cairo_pattern_type_t = _cairo_pattern_type;
pub type _cairo_pattern_type = libc::c_uint;
pub const CAIRO_PATTERN_TYPE_RASTER_SOURCE: _cairo_pattern_type = 5;
pub const CAIRO_PATTERN_TYPE_MESH: _cairo_pattern_type = 4;
pub const CAIRO_PATTERN_TYPE_RADIAL: _cairo_pattern_type = 3;
pub const CAIRO_PATTERN_TYPE_LINEAR: _cairo_pattern_type = 2;
pub const CAIRO_PATTERN_TYPE_SURFACE: _cairo_pattern_type = 1;
pub const CAIRO_PATTERN_TYPE_SOLID: _cairo_pattern_type = 0;
pub type cairo_operator_t = _cairo_operator;
pub type _cairo_operator = libc::c_uint;
pub const CAIRO_OPERATOR_HSL_LUMINOSITY: _cairo_operator = 28;
pub const CAIRO_OPERATOR_HSL_COLOR: _cairo_operator = 27;
pub const CAIRO_OPERATOR_HSL_SATURATION: _cairo_operator = 26;
pub const CAIRO_OPERATOR_HSL_HUE: _cairo_operator = 25;
pub const CAIRO_OPERATOR_EXCLUSION: _cairo_operator = 24;
pub const CAIRO_OPERATOR_DIFFERENCE: _cairo_operator = 23;
pub const CAIRO_OPERATOR_SOFT_LIGHT: _cairo_operator = 22;
pub const CAIRO_OPERATOR_HARD_LIGHT: _cairo_operator = 21;
pub const CAIRO_OPERATOR_COLOR_BURN: _cairo_operator = 20;
pub const CAIRO_OPERATOR_COLOR_DODGE: _cairo_operator = 19;
pub const CAIRO_OPERATOR_LIGHTEN: _cairo_operator = 18;
pub const CAIRO_OPERATOR_DARKEN: _cairo_operator = 17;
pub const CAIRO_OPERATOR_OVERLAY: _cairo_operator = 16;
pub const CAIRO_OPERATOR_SCREEN: _cairo_operator = 15;
pub const CAIRO_OPERATOR_MULTIPLY: _cairo_operator = 14;
pub const CAIRO_OPERATOR_SATURATE: _cairo_operator = 13;
pub const CAIRO_OPERATOR_ADD: _cairo_operator = 12;
pub const CAIRO_OPERATOR_XOR: _cairo_operator = 11;
pub const CAIRO_OPERATOR_DEST_ATOP: _cairo_operator = 10;
pub const CAIRO_OPERATOR_DEST_OUT: _cairo_operator = 9;
pub const CAIRO_OPERATOR_DEST_IN: _cairo_operator = 8;
pub const CAIRO_OPERATOR_DEST_OVER: _cairo_operator = 7;
pub const CAIRO_OPERATOR_DEST: _cairo_operator = 6;
pub const CAIRO_OPERATOR_ATOP: _cairo_operator = 5;
pub const CAIRO_OPERATOR_OUT: _cairo_operator = 4;
pub const CAIRO_OPERATOR_IN: _cairo_operator = 3;
pub const CAIRO_OPERATOR_OVER: _cairo_operator = 2;
pub const CAIRO_OPERATOR_SOURCE: _cairo_operator = 1;
pub const CAIRO_OPERATOR_CLEAR: _cairo_operator = 0;
pub type cairo_stroke_style_t = _cairo_stroke_style;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_stroke_style {
    pub line_width: libc::c_double,
    pub line_cap: cairo_line_cap_t,
    pub line_join: cairo_line_join_t,
    pub miter_limit: libc::c_double,
    pub dash: *mut libc::c_double,
    pub num_dashes: libc::c_uint,
    pub dash_offset: libc::c_double,
    pub is_hairline: cairo_bool_t,
    pub pre_hairline_line_width: libc::c_double,
}
pub type cairo_line_join_t = _cairo_line_join;
pub type _cairo_line_join = libc::c_uint;
pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;
pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
pub type cairo_line_cap_t = _cairo_line_cap;
pub type _cairo_line_cap = libc::c_uint;
pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;
pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
pub type cairo_destroy_func_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type cairo_raster_source_acquire_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
        *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t,
>;
pub type cairo_raster_source_release_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *mut cairo_surface_t,
    ) -> (),
>;
pub type cairo_raster_source_snapshot_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> cairo_status_t,
>;
pub type cairo_raster_source_copy_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_pattern_t,
        *mut libc::c_void,
        *const cairo_pattern_t,
    ) -> cairo_status_t,
>;
pub type cairo_raster_source_finish_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_pattern_t, *mut libc::c_void) -> (),
>;
pub type _cairo_region_overlap = libc::c_uint;
pub const CAIRO_REGION_OVERLAP_PART: _cairo_region_overlap = 2;
pub const CAIRO_REGION_OVERLAP_OUT: _cairo_region_overlap = 1;
pub const CAIRO_REGION_OVERLAP_IN: _cairo_region_overlap = 0;
pub type cairo_region_overlap_t = _cairo_region_overlap;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type pixman_fixed_16_16_t = int32_t;
pub type pixman_fixed_t = pixman_fixed_16_16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_transform {
    pub matrix: [[pixman_fixed_t; 3]; 3],
}
pub type pixman_transform_t = pixman_transform;
pub type cairo_fixed_16_16_t = int32_t;
pub type cairo_fixed_unsigned_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_t {
    pub status: cairo_status_t,
    pub limit: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    pub num_boxes: libc::c_int,
    pub is_pixel_aligned: libc::c_uint,
    pub chunks: _cairo_boxes_chunk,
    pub tail: *mut _cairo_boxes_chunk,
    pub boxes_embedded: [cairo_box_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_boxes_chunk {
    pub next: *mut _cairo_boxes_chunk,
    pub base: *mut cairo_box_t,
    pub count: libc::c_int,
    pub size: libc::c_int,
}
pub type cairo_boxes_t = _cairo_boxes_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cache {
    pub hash_table: *mut cairo_hash_table_t,
    pub predicate: cairo_cache_predicate_func_t,
    pub entry_destroy: cairo_destroy_func_t,
    pub max_size: libc::c_ulong,
    pub size: libc::c_ulong,
    pub freeze_count: libc::c_int,
}
pub type cairo_cache_predicate_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_cache_t = _cairo_cache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_composite_rectangles {
    pub surface: *mut cairo_surface_t,
    pub op: cairo_operator_t,
    pub source: cairo_rectangle_int_t,
    pub mask: cairo_rectangle_int_t,
    pub destination: cairo_rectangle_int_t,
    pub bounded: cairo_rectangle_int_t,
    pub unbounded: cairo_rectangle_int_t,
    pub is_bounded: uint32_t,
    pub source_sample_area: cairo_rectangle_int_t,
    pub mask_sample_area: cairo_rectangle_int_t,
    pub source_pattern: cairo_pattern_union_t,
    pub mask_pattern: cairo_pattern_union_t,
    pub original_source_pattern: *const cairo_pattern_t,
    pub original_mask_pattern: *const cairo_pattern_t,
    pub clip: *mut cairo_clip_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_pattern_union_t {
    pub base: cairo_pattern_t,
    pub solid: cairo_solid_pattern_t,
    pub surface: cairo_surface_pattern_t,
    pub gradient: cairo_gradient_pattern_union_t,
    pub mesh: cairo_mesh_pattern_t,
    pub raster_source: cairo_raster_source_pattern_t,
}
pub type cairo_raster_source_pattern_t = _cairo_raster_source_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_raster_source_pattern {
    pub base: cairo_pattern_t,
    pub content: cairo_content_t,
    pub extents: cairo_rectangle_int_t,
    pub acquire: cairo_raster_source_acquire_func_t,
    pub release: cairo_raster_source_release_func_t,
    pub snapshot: cairo_raster_source_snapshot_func_t,
    pub copy: cairo_raster_source_copy_func_t,
    pub finish: cairo_raster_source_finish_func_t,
    pub user_data: *mut libc::c_void,
}
pub type cairo_mesh_pattern_t = _cairo_mesh_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_pattern {
    pub base: cairo_pattern_t,
    pub patches: cairo_array_t,
    pub current_patch: *mut cairo_mesh_patch_t,
    pub current_side: libc::c_int,
    pub has_control_point: [cairo_bool_t; 4],
    pub has_color: [cairo_bool_t; 4],
}
pub type cairo_mesh_patch_t = _cairo_mesh_patch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_mesh_patch {
    pub points: [[cairo_point_double_t; 4]; 4],
    pub colors: [cairo_color_t; 4],
}
pub type cairo_point_double_t = _cairo_point_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_double {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_gradient_pattern_union_t {
    pub base: cairo_gradient_pattern_t,
    pub linear: cairo_linear_pattern_t,
    pub radial: cairo_radial_pattern_t,
}
pub type cairo_radial_pattern_t = _cairo_radial_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_radial_pattern {
    pub base: cairo_gradient_pattern_t,
    pub cd1: cairo_circle_double_t,
    pub cd2: cairo_circle_double_t,
}
pub type cairo_circle_double_t = _cairo_circle_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_circle_double {
    pub center: cairo_point_double_t,
    pub radius: libc::c_double,
}
pub type cairo_gradient_pattern_t = _cairo_gradient_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_pattern {
    pub base: cairo_pattern_t,
    pub n_stops: libc::c_uint,
    pub stops_size: libc::c_uint,
    pub stops: *mut cairo_gradient_stop_t,
    pub stops_embedded: [cairo_gradient_stop_t; 2],
}
pub type cairo_gradient_stop_t = _cairo_gradient_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_gradient_stop {
    pub offset: libc::c_double,
    pub color: cairo_color_stop_t,
}
pub type cairo_color_stop_t = _cairo_color_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_color_stop {
    pub red: libc::c_double,
    pub green: libc::c_double,
    pub blue: libc::c_double,
    pub alpha: libc::c_double,
    pub red_short: uint16_t,
    pub green_short: uint16_t,
    pub blue_short: uint16_t,
    pub alpha_short: uint16_t,
}
pub type cairo_linear_pattern_t = _cairo_linear_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_linear_pattern {
    pub base: cairo_gradient_pattern_t,
    pub pd1: cairo_point_double_t,
    pub pd2: cairo_point_double_t,
}
pub type cairo_surface_pattern_t = _cairo_surface_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_pattern {
    pub base: cairo_pattern_t,
    pub surface: *mut cairo_surface_t,
}
pub type cairo_solid_pattern_t = _cairo_solid_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_solid_pattern {
    pub base: cairo_pattern_t,
    pub color: cairo_color_t,
}
pub type cairo_composite_rectangles_t = _cairo_composite_rectangles;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_snapshot {
    pub base: cairo_surface_t,
    pub mutex: cairo_mutex_t,
    pub target: *mut cairo_surface_t,
    pub clone: *mut cairo_surface_t,
}
pub type cairo_mutex_t = cairo_mutex_impl_t;
pub type cairo_mutex_impl_t = pthread_mutex_t;
pub type cairo_surface_snapshot_t = _cairo_surface_snapshot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_subsurface {
    pub base: cairo_surface_t,
    pub extents: cairo_rectangle_int_t,
    pub target: *mut cairo_surface_t,
    pub snapshot: *mut cairo_surface_t,
}
pub type cairo_surface_subsurface_t = _cairo_surface_subsurface;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _cairo_traps {
    pub status: cairo_status_t,
    pub bounds: cairo_box_t,
    pub limits: *const cairo_box_t,
    pub num_limits: libc::c_int,
    #[bitfield(name = "maybe_region", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_intersections", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_rectilinear", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "is_rectangular", ty = "libc::c_uint", bits = "3..=3")]
    pub maybe_region_has_intersections_is_rectilinear_is_rectangular: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub num_traps: libc::c_int,
    pub traps_size: libc::c_int,
    pub traps: *mut cairo_trapezoid_t,
    pub traps_embedded: [cairo_trapezoid_t; 16],
}
pub type cairo_trapezoid_t = _cairo_trapezoid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_trapezoid {
    pub top: cairo_fixed_t,
    pub bottom: cairo_fixed_t,
    pub left: cairo_line_t,
    pub right: cairo_line_t,
}
pub type cairo_line_t = _cairo_line;
pub type cairo_traps_t = _cairo_traps;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_private {
    pub link: cairo_list_t,
    pub key: *const libc::c_void,
    pub destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_font_private_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
}
pub type cairo_scaled_font_private_t = _cairo_scaled_font_private;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_glyph_private {
    pub link: cairo_list_t,
    pub key: *const libc::c_void,
    pub destroy: Option::<
        unsafe extern "C" fn(
            *mut cairo_scaled_glyph_private_t,
            *mut cairo_scaled_glyph_t,
            *mut cairo_scaled_font_t,
        ) -> (),
    >,
}
pub type cairo_scaled_glyph_private_t = _cairo_scaled_glyph_private;
pub type _cairo_internal_surface_type = libc::c_uint;
pub const CAIRO_INTERNAL_SURFACE_TYPE_QUARTZ_SNAPSHOT: _cairo_internal_surface_type = 4105;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TYPE3_GLYPH: _cairo_internal_surface_type = 4104;
pub const CAIRO_INTERNAL_SURFACE_TYPE_NULL: _cairo_internal_surface_type = 4103;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_WRAPPING: _cairo_internal_surface_type = 4102;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_PAGINATED: _cairo_internal_surface_type = 4101;
pub const CAIRO_INTERNAL_SURFACE_TYPE_TEST_FALLBACK: _cairo_internal_surface_type = 4100;
pub const CAIRO_INTERNAL_SURFACE_TYPE_OBSERVER: _cairo_internal_surface_type = 4099;
pub const CAIRO_INTERNAL_SURFACE_TYPE_ANALYSIS: _cairo_internal_surface_type = 4098;
pub const CAIRO_INTERNAL_SURFACE_TYPE_PAGINATED: _cairo_internal_surface_type = 4097;
pub const CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT: _cairo_internal_surface_type = 4096;
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
pub type cairo_stock_t = libc::c_uint;
pub const CAIRO_STOCK_NUM_COLORS: cairo_stock_t = 3;
pub const CAIRO_STOCK_TRANSPARENT: cairo_stock_t = 2;
pub const CAIRO_STOCK_BLACK: cairo_stock_t = 1;
pub const CAIRO_STOCK_WHITE: cairo_stock_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: libc::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_rectangle_t {
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
pub type xcb_image_order_t = libc::c_uint;
pub const XCB_IMAGE_ORDER_MSB_FIRST: xcb_image_order_t = 1;
pub const XCB_IMAGE_ORDER_LSB_FIRST: xcb_image_order_t = 0;
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
pub type xcb_render_pict_op_t = libc::c_uint;
pub const XCB_RENDER_PICT_OP_HSL_LUMINOSITY: xcb_render_pict_op_t = 62;
pub const XCB_RENDER_PICT_OP_HSL_COLOR: xcb_render_pict_op_t = 61;
pub const XCB_RENDER_PICT_OP_HSL_SATURATION: xcb_render_pict_op_t = 60;
pub const XCB_RENDER_PICT_OP_HSL_HUE: xcb_render_pict_op_t = 59;
pub const XCB_RENDER_PICT_OP_EXCLUSION: xcb_render_pict_op_t = 58;
pub const XCB_RENDER_PICT_OP_DIFFERENCE: xcb_render_pict_op_t = 57;
pub const XCB_RENDER_PICT_OP_SOFT_LIGHT: xcb_render_pict_op_t = 56;
pub const XCB_RENDER_PICT_OP_HARD_LIGHT: xcb_render_pict_op_t = 55;
pub const XCB_RENDER_PICT_OP_COLOR_BURN: xcb_render_pict_op_t = 54;
pub const XCB_RENDER_PICT_OP_COLOR_DODGE: xcb_render_pict_op_t = 53;
pub const XCB_RENDER_PICT_OP_LIGHTEN: xcb_render_pict_op_t = 52;
pub const XCB_RENDER_PICT_OP_DARKEN: xcb_render_pict_op_t = 51;
pub const XCB_RENDER_PICT_OP_OVERLAY: xcb_render_pict_op_t = 50;
pub const XCB_RENDER_PICT_OP_SCREEN: xcb_render_pict_op_t = 49;
pub const XCB_RENDER_PICT_OP_MULTIPLY: xcb_render_pict_op_t = 48;
pub const XCB_RENDER_PICT_OP_CONJOINT_XOR: xcb_render_pict_op_t = 43;
pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE: xcb_render_pict_op_t = 42;
pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP: xcb_render_pict_op_t = 41;
pub const XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE: xcb_render_pict_op_t = 40;
pub const XCB_RENDER_PICT_OP_CONJOINT_OUT: xcb_render_pict_op_t = 39;
pub const XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE: xcb_render_pict_op_t = 38;
pub const XCB_RENDER_PICT_OP_CONJOINT_IN: xcb_render_pict_op_t = 37;
pub const XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE: xcb_render_pict_op_t = 36;
pub const XCB_RENDER_PICT_OP_CONJOINT_OVER: xcb_render_pict_op_t = 35;
pub const XCB_RENDER_PICT_OP_CONJOINT_DST: xcb_render_pict_op_t = 34;
pub const XCB_RENDER_PICT_OP_CONJOINT_SRC: xcb_render_pict_op_t = 33;
pub const XCB_RENDER_PICT_OP_CONJOINT_CLEAR: xcb_render_pict_op_t = 32;
pub const XCB_RENDER_PICT_OP_DISJOINT_XOR: xcb_render_pict_op_t = 27;
pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE: xcb_render_pict_op_t = 26;
pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP: xcb_render_pict_op_t = 25;
pub const XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE: xcb_render_pict_op_t = 24;
pub const XCB_RENDER_PICT_OP_DISJOINT_OUT: xcb_render_pict_op_t = 23;
pub const XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE: xcb_render_pict_op_t = 22;
pub const XCB_RENDER_PICT_OP_DISJOINT_IN: xcb_render_pict_op_t = 21;
pub const XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE: xcb_render_pict_op_t = 20;
pub const XCB_RENDER_PICT_OP_DISJOINT_OVER: xcb_render_pict_op_t = 19;
pub const XCB_RENDER_PICT_OP_DISJOINT_DST: xcb_render_pict_op_t = 18;
pub const XCB_RENDER_PICT_OP_DISJOINT_SRC: xcb_render_pict_op_t = 17;
pub const XCB_RENDER_PICT_OP_DISJOINT_CLEAR: xcb_render_pict_op_t = 16;
pub const XCB_RENDER_PICT_OP_SATURATE: xcb_render_pict_op_t = 13;
pub const XCB_RENDER_PICT_OP_ADD: xcb_render_pict_op_t = 12;
pub const XCB_RENDER_PICT_OP_XOR: xcb_render_pict_op_t = 11;
pub const XCB_RENDER_PICT_OP_ATOP_REVERSE: xcb_render_pict_op_t = 10;
pub const XCB_RENDER_PICT_OP_ATOP: xcb_render_pict_op_t = 9;
pub const XCB_RENDER_PICT_OP_OUT_REVERSE: xcb_render_pict_op_t = 8;
pub const XCB_RENDER_PICT_OP_OUT: xcb_render_pict_op_t = 7;
pub const XCB_RENDER_PICT_OP_IN_REVERSE: xcb_render_pict_op_t = 6;
pub const XCB_RENDER_PICT_OP_IN: xcb_render_pict_op_t = 5;
pub const XCB_RENDER_PICT_OP_OVER_REVERSE: xcb_render_pict_op_t = 4;
pub const XCB_RENDER_PICT_OP_OVER: xcb_render_pict_op_t = 3;
pub const XCB_RENDER_PICT_OP_DST: xcb_render_pict_op_t = 2;
pub const XCB_RENDER_PICT_OP_SRC: xcb_render_pict_op_t = 1;
pub const XCB_RENDER_PICT_OP_CLEAR: xcb_render_pict_op_t = 0;
pub type xcb_render_poly_mode_t = libc::c_uint;
pub const XCB_RENDER_POLY_MODE_IMPRECISE: xcb_render_poly_mode_t = 1;
pub const XCB_RENDER_POLY_MODE_PRECISE: xcb_render_poly_mode_t = 0;
pub type xcb_render_cp_t = libc::c_uint;
pub const XCB_RENDER_CP_COMPONENT_ALPHA: xcb_render_cp_t = 4096;
pub const XCB_RENDER_CP_DITHER: xcb_render_cp_t = 2048;
pub const XCB_RENDER_CP_POLY_MODE: xcb_render_cp_t = 1024;
pub const XCB_RENDER_CP_POLY_EDGE: xcb_render_cp_t = 512;
pub const XCB_RENDER_CP_SUBWINDOW_MODE: xcb_render_cp_t = 256;
pub const XCB_RENDER_CP_GRAPHICS_EXPOSURE: xcb_render_cp_t = 128;
pub const XCB_RENDER_CP_CLIP_MASK: xcb_render_cp_t = 64;
pub const XCB_RENDER_CP_CLIP_Y_ORIGIN: xcb_render_cp_t = 32;
pub const XCB_RENDER_CP_CLIP_X_ORIGIN: xcb_render_cp_t = 16;
pub const XCB_RENDER_CP_ALPHA_Y_ORIGIN: xcb_render_cp_t = 8;
pub const XCB_RENDER_CP_ALPHA_X_ORIGIN: xcb_render_cp_t = 4;
pub const XCB_RENDER_CP_ALPHA_MAP: xcb_render_cp_t = 2;
pub const XCB_RENDER_CP_REPEAT: xcb_render_cp_t = 1;
pub type xcb_render_sub_pixel_t = libc::c_uint;
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 5;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 4;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 3;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 2;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 1;
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0;
pub type xcb_render_repeat_t = libc::c_uint;
pub const XCB_RENDER_REPEAT_REFLECT: xcb_render_repeat_t = 3;
pub const XCB_RENDER_REPEAT_PAD: xcb_render_repeat_t = 2;
pub const XCB_RENDER_REPEAT_NORMAL: xcb_render_repeat_t = 1;
pub const XCB_RENDER_REPEAT_NONE: xcb_render_repeat_t = 0;
pub type xcb_render_glyph_t = uint32_t;
pub type xcb_render_glyphset_t = uint32_t;
pub type xcb_render_picture_t = uint32_t;
pub type xcb_render_pictformat_t = uint32_t;
pub type xcb_render_fixed_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_color_t {
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub alpha: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_pointfix_t {
    pub x: xcb_render_fixed_t,
    pub y: xcb_render_fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_linefix_t {
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_trapezoid_t {
    pub top: xcb_render_fixed_t,
    pub bottom: xcb_render_fixed_t,
    pub left: xcb_render_linefix_t,
    pub right: xcb_render_linefix_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_glyphinfo_t {
    pub width: uint16_t,
    pub height: uint16_t,
    pub x: int16_t,
    pub y: int16_t,
    pub x_off: int16_t,
    pub y_off: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_render_transform_t {
    pub matrix11: xcb_render_fixed_t,
    pub matrix12: xcb_render_fixed_t,
    pub matrix13: xcb_render_fixed_t,
    pub matrix21: xcb_render_fixed_t,
    pub matrix22: xcb_render_fixed_t,
    pub matrix23: xcb_render_fixed_t,
    pub matrix31: xcb_render_fixed_t,
    pub matrix32: xcb_render_fixed_t,
    pub matrix33: xcb_render_fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist_node {
    pub next: *mut cairo_freelist_node_t,
}
pub type cairo_freelist_node_t = _cairo_freelist_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_freelist {
    pub first_free_node: *mut cairo_freelist_node_t,
    pub nodesize: libc::c_uint,
}
pub type cairo_freelist_t = _cairo_freelist;
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
pub struct _cairo_xcb_font {
    pub base: cairo_scaled_font_private_t,
    pub scaled_font: *mut cairo_scaled_font_t,
    pub connection: *mut cairo_xcb_connection_t,
    pub glyphset_info: [cairo_xcb_font_glyphset_info_t; 3],
    pub link: cairo_list_t,
}
pub type cairo_xcb_font_glyphset_info_t = _cairo_xcb_font_glyphset_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_font_glyphset_info {
    pub glyphset: xcb_render_glyphset_t,
    pub format: cairo_format_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pending_free_glyphs: *mut cairo_xcb_font_glyphset_free_glyphs_t,
}
pub type cairo_xcb_font_glyphset_free_glyphs_t = _cairo_xcb_font_glyphset_free_glyphs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_font_glyphset_free_glyphs {
    pub glyphset: xcb_render_glyphset_t,
    pub glyph_count: libc::c_int,
    pub glyph_indices: [xcb_render_glyph_t; 128],
}
pub type cairo_xcb_font_t = _cairo_xcb_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_screen {
    pub connection: *mut cairo_xcb_connection_t,
    pub xcb_screen: *mut xcb_screen_t,
    pub subpixel_order: xcb_render_sub_pixel_t,
    pub gc: [xcb_gcontext_t; 4],
    pub gc_depths: [uint8_t; 4],
    pub stock_colors: [*mut cairo_surface_t; 3],
    pub solid_cache: [C2RustUnnamed_0; 16],
    pub solid_cache_size: libc::c_int,
    pub linear_pattern_cache: cairo_cache_t,
    pub radial_pattern_cache: cairo_cache_t,
    pub pattern_cache_entry_freelist: cairo_freelist_t,
    pub link: cairo_list_t,
    pub surfaces: cairo_list_t,
    pub pictures: cairo_list_t,
    pub has_font_options: cairo_bool_t,
    pub font_options: cairo_font_options_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub picture: *mut cairo_surface_t,
    pub color: cairo_color_t,
}
pub type cairo_xcb_screen_t = _cairo_xcb_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_surface {
    pub base: cairo_surface_t,
    pub fallback: *mut cairo_image_surface_t,
    pub fallback_damage: cairo_boxes_t,
    pub connection: *mut cairo_xcb_connection_t,
    pub screen: *mut cairo_xcb_screen_t,
    pub drawable: xcb_drawable_t,
    pub owns_pixmap: cairo_bool_t,
    pub deferred_clear: cairo_bool_t,
    pub deferred_clear_color: cairo_color_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub picture: xcb_render_picture_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pixman_format: pixman_format_code_t,
    pub precision: uint32_t,
    pub link: cairo_list_t,
}
pub type cairo_xcb_surface_t = _cairo_xcb_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xcb_picture {
    pub base: cairo_surface_t,
    pub screen: *mut cairo_xcb_screen_t,
    pub picture: xcb_render_picture_t,
    pub xrender_format: xcb_render_pictformat_t,
    pub pixman_format: pixman_format_code_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub extend: cairo_extend_t,
    pub filter: cairo_filter_t,
    pub has_component_alpha: cairo_bool_t,
    pub transform: xcb_render_transform_t,
    pub x0: libc::c_int,
    pub y0: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub link: cairo_list_t,
}
pub type cairo_xcb_picture_t = _cairo_xcb_picture;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NUM_GLYPHSETS: C2RustUnnamed_1 = 3;
pub const GLYPHSET_INDEX_A1: C2RustUnnamed_1 = 2;
pub const GLYPHSET_INDEX_A8: C2RustUnnamed_1 = 1;
pub const GLYPHSET_INDEX_ARGB32: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CAIRO_XCB_SHM_MASK: C2RustUnnamed_2 = 2147483648;
pub const CAIRO_XCB_RENDER_MASK: C2RustUnnamed_2 = 4095;
pub const CAIRO_XCB_HAS_SHM: C2RustUnnamed_2 = 2147483648;
pub const CAIRO_XCB_RENDER_HAS_FILTER_BEST: C2RustUnnamed_2 = 2048;
pub const CAIRO_XCB_RENDER_HAS_FILTER_GOOD: C2RustUnnamed_2 = 1024;
pub const CAIRO_XCB_RENDER_HAS_GRADIENTS: C2RustUnnamed_2 = 512;
pub const CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT: C2RustUnnamed_2 = 256;
pub const CAIRO_XCB_RENDER_HAS_PDF_OPERATORS: C2RustUnnamed_2 = 128;
pub const CAIRO_XCB_RENDER_HAS_FILTERS: C2RustUnnamed_2 = 64;
pub const CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM: C2RustUnnamed_2 = 32;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS: C2RustUnnamed_2 = 16;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS: C2RustUnnamed_2 = 8;
pub const CAIRO_XCB_RENDER_HAS_COMPOSITE: C2RustUnnamed_2 = 4;
pub const CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES: C2RustUnnamed_2 = 2;
pub const CAIRO_XCB_HAS_RENDER: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_traps_info_t {
    pub traps: cairo_traps_t,
    pub antialias: cairo_antialias_t,
}
pub const NEED_CLIP_SURFACE: C2RustUnnamed_4 = 2;
pub const NEED_CLIP_REGION: C2RustUnnamed_4 = 1;
pub type xcb_draw_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut cairo_xcb_surface_t,
        cairo_operator_t,
        *const cairo_pattern_t,
        libc::c_int,
        libc::c_int,
        *const cairo_rectangle_int_t,
        *mut cairo_clip_t,
    ) -> cairo_int_status_t,
>;
pub const FORCE_CLIP_REGION: C2RustUnnamed_4 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _image_upload_box {
    pub surface: *mut cairo_xcb_surface_t,
    pub image: *mut cairo_image_surface_t,
    pub gc: xcb_gcontext_t,
    pub tx: libc::c_int,
    pub ty: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _image_contains_box {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub tx: libc::c_int,
    pub ty: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_box_info {
    pub dst: *mut cairo_xcb_surface_t,
    pub src: *mut cairo_xcb_picture_t,
    pub op: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_opacity_info {
    pub op: uint8_t,
    pub dst: *mut cairo_xcb_surface_t,
    pub src: *mut cairo_xcb_picture_t,
    pub opacity: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct composite_glyphs_info_t {
    pub font: *mut cairo_scaled_font_t,
    pub glyphs: *mut cairo_xcb_glyph_t,
    pub num_glyphs: libc::c_int,
    pub use_mask: cairo_bool_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cairo_xcb_glyph_t {
    pub d: cairo_glyph_t,
    pub index: libc::c_ulong,
    pub i: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub index: libc::c_ulong,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type cairo_xcb_render_composite_text_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_xcb_connection_t,
        uint8_t,
        xcb_render_picture_t,
        xcb_render_picture_t,
        xcb_render_pictformat_t,
        xcb_render_glyphset_t,
        int16_t,
        int16_t,
        uint32_t,
        *mut uint8_t,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x_glyph_elt_t {
    pub len: uint8_t,
    pub pad1: uint8_t,
    pub pad2: uint16_t,
    pub deltax: int16_t,
    pub deltay: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_xcb_glyph_private_t {
    pub base: cairo_scaled_glyph_private_t,
    pub glyphset: *mut cairo_xcb_font_glyphset_info_t,
}
pub type C2RustUnnamed_4 = libc::c_uint;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_16_16_from_double(
    mut d: libc::c_double,
) -> cairo_fixed_16_16_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
    u.d = d + 103079215104.0f64;
    return u.i[0 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_16_16(mut f: cairo_fixed_t) -> cairo_fixed_16_16_t {
    let mut x: cairo_fixed_16_16_t = 0;
    if (f >> 8 as libc::c_int) < -(32767 as libc::c_int) - 1 as libc::c_int {
        x = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    } else if f >> 8 as libc::c_int > 32767 as libc::c_int {
        x = 2147483647 as libc::c_int;
    } else {
        x = f << 16 as libc::c_int - 8 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_ceil(mut f: cairo_fixed_t) -> libc::c_int {
    if f > 0 as libc::c_int {
        return (f - 1 as libc::c_int >> 8 as libc::c_int) + 1 as libc::c_int
    } else {
        return -((f as cairo_fixed_unsigned_t).wrapping_neg() as cairo_fixed_t
            >> 8 as libc::c_int)
    };
}
#[inline]
unsafe extern "C" fn _cairo_fixed_fractional_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_round_down(
    mut f: cairo_fixed_t,
) -> libc::c_int {
    return _cairo_fixed_integer_part(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
    return f >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_round_down(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return _cairo_fixed_floor(
        f
            + (-(1 as libc::c_int) as cairo_fixed_unsigned_t
                >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
                / 2 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn _cairo_fixed_floor(mut f: cairo_fixed_t) -> cairo_fixed_t {
    return f
        & !((-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t);
}
#[inline]
unsafe extern "C" fn _cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
    return (f
        & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
            >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn _cairo_is_little_endian() -> cairo_bool_t {
    static mut i: libc::c_int = 1 as libc::c_int;
    return (*(&i as *const libc::c_int as *mut libc::c_char) as libc::c_int
        == 0x1 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_fill_is_rectilinear(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    if (*path).fill_is_rectilinear() == 0 {
        return 0 as libc::c_int;
    }
    if (*path).has_current_point() == 0 || (*path).needs_move_to() as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return ((*path).current_point.x == (*path).last_move_point.x
        || (*path).current_point.y == (*path).last_move_point.y) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_path_fixed_stroke_is_rectilinear(
    mut path: *const cairo_path_fixed_t,
) -> cairo_bool_t {
    return (*path).stroke_is_rectilinear() as cairo_bool_t;
}
#[inline]
unsafe extern "C" fn _cairo_rectangle_contains_rectangle(
    mut a: *const cairo_rectangle_int_t,
    mut b: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    return ((*a).x <= (*b).x && (*a).x + (*a).width >= (*b).x + (*b).width
        && (*a).y <= (*b).y && (*a).y + (*a).height >= (*b).y + (*b).height)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_round(mut r: libc::c_double) -> libc::c_double {
    return floor(r + 0.5f64);
}
#[inline]
unsafe extern "C" fn _cairo_lround(mut r: libc::c_double) -> libc::c_int {
    return _cairo_round(r) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_xcb(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return (!((*surface).backend).is_null()
        && (*(*surface).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_XCB as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_reference(
    mut connection: *mut cairo_xcb_connection_t,
) -> *mut cairo_xcb_connection_t {
    return cairo_device_reference(&mut (*connection).device)
        as *mut cairo_xcb_connection_t;
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_acquire(
    mut connection: *mut cairo_xcb_connection_t,
) -> cairo_status_t {
    return cairo_device_acquire(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_release(
    mut connection: *mut cairo_xcb_connection_t,
) {
    cairo_device_release(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_xcb_connection_destroy(
    mut connection: *mut cairo_xcb_connection_t,
) {
    cairo_device_destroy(&mut (*connection).device);
}
#[inline]
unsafe extern "C" fn _cairo_clip_is_all_clipped(
    mut clip: *const cairo_clip_t,
) -> cairo_bool_t {
    return (clip == &__cairo_clip_all as *const cairo_clip_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_clip_steal_boxes(
    mut clip: *mut cairo_clip_t,
    mut boxes: *mut cairo_boxes_t,
) {
    let mut array: *mut cairo_box_t = (*clip).boxes;
    if array == &mut (*clip).embedded_box as *mut cairo_box_t {
        if (*clip).num_boxes == 1 as libc::c_int {} else {
            __assert_fail(
                b"clip->num_boxes == 1\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-clip-inline.h\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"void _cairo_clip_steal_boxes(cairo_clip_t *, cairo_boxes_t *)\0"))
                    .as_ptr(),
            );
        }
        (*boxes).boxes_embedded[0 as libc::c_int as usize] = (*clip).embedded_box;
        array = &mut *((*boxes).boxes_embedded)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cairo_box_t;
    }
    _cairo_boxes_init_for_array(boxes, array, (*clip).num_boxes);
    let ref mut fresh2 = (*clip).boxes;
    *fresh2 = 0 as *mut cairo_box_t;
    (*clip).num_boxes = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_clip_unsteal_boxes(
    mut clip: *mut cairo_clip_t,
    mut boxes: *mut cairo_boxes_t,
) {
    if (*boxes).chunks.base
        == &mut *((*boxes).boxes_embedded).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut cairo_box_t
    {
        if (*boxes).num_boxes == 1 as libc::c_int {} else {
            __assert_fail(
                b"boxes->num_boxes == 1\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-clip-inline.h\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void _cairo_clip_unsteal_boxes(cairo_clip_t *, cairo_boxes_t *)\0"))
                    .as_ptr(),
            );
        }
        (*clip).embedded_box = *(*boxes).chunks.base;
        let ref mut fresh3 = (*clip).boxes;
        *fresh3 = &mut (*clip).embedded_box;
    } else {
        let ref mut fresh4 = (*clip).boxes;
        *fresh4 = (*boxes).chunks.base;
    }
    (*clip).num_boxes = (*boxes).num_boxes;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_image(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return (!((*surface).backend).is_null()
        && (*(*surface).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint) as libc::c_int;
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
unsafe extern "C" fn _cairo_surface_is_snapshot(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_INTERNAL_SURFACE_TYPE_SNAPSHOT as libc::c_int as cairo_surface_type_t
            as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_is_recording(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return ((*(*surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_pattern_get_source(
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
) -> *mut cairo_surface_t {
    return _cairo_surface_get_source((*pattern).surface, extents);
}
#[inline]
unsafe extern "C" fn _picture_to_connection(
    mut picture: *mut cairo_xcb_picture_t,
) -> *mut cairo_xcb_connection_t {
    return (*picture).base.device as *mut cairo_xcb_connection_t;
}
unsafe extern "C" fn hars_petruska_f54_1_random() -> uint32_t {
    static mut x: uint32_t = 0;
    x = (x ^ (x << 5 as libc::c_int | x >> 32 as libc::c_int - 5 as libc::c_int)
        ^ (x << 24 as libc::c_int | x >> 32 as libc::c_int - 24 as libc::c_int))
        .wrapping_add(0x37798849 as libc::c_int as libc::c_uint);
    return x;
}
unsafe extern "C" fn _cairo_xcb_picture_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_xcb_picture_t = abstract_surface
        as *mut cairo_xcb_picture_t;
    let mut connection: *mut cairo_xcb_connection_t = _picture_to_connection(surface);
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_xcb_connection_acquire(connection);
    cairo_list_del(&mut (*surface).link);
    if status as u64 != 0 {
        return status;
    }
    _cairo_xcb_connection_render_free_picture(connection, (*surface).picture);
    _cairo_xcb_connection_release(connection);
    return CAIRO_STATUS_SUCCESS;
}
static mut _cairo_xcb_picture_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_XCB,
            finish: Some(
                _cairo_xcb_picture_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: None,
            create_similar: None,
            create_similar_image: None,
            map_to_image: None,
            unmap_image: None,
            source: None,
            acquire_source_image: None,
            release_source_image: None,
            snapshot: None,
            copy_page: None,
            show_page: None,
            get_extents: None,
            get_font_options: None,
            flush: None,
            mark_dirty_rectangle: None,
            paint: None,
            mask: None,
            stroke: None,
            fill: None,
            fill_stroke: None,
            show_glyphs: None,
            has_show_text_glyphs: None,
            show_text_glyphs: None,
            get_supported_mime_types: None,
            tag: None,
        };
        init
    }
};
static mut identity_transform: xcb_render_transform_t = {
    let mut init = xcb_render_transform_t {
        matrix11: (1 as libc::c_int) << 16 as libc::c_int,
        matrix12: 0 as libc::c_int,
        matrix13: 0 as libc::c_int,
        matrix21: 0 as libc::c_int,
        matrix22: (1 as libc::c_int) << 16 as libc::c_int,
        matrix23: 0 as libc::c_int,
        matrix31: 0 as libc::c_int,
        matrix32: 0 as libc::c_int,
        matrix33: (1 as libc::c_int) << 16 as libc::c_int,
    };
    init
};
unsafe extern "C" fn _cairo_xcb_picture_create(
    mut screen: *mut cairo_xcb_screen_t,
    mut pixman_format: pixman_format_code_t,
    mut xrender_format: xcb_render_pictformat_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut cairo_xcb_picture_t {
    let mut surface: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    surface = (if ::std::mem::size_of::<cairo_xcb_picture_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xcb_picture_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xcb_picture_t;
    if surface.is_null() {
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY))
            as *mut cairo_xcb_picture_t;
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &_cairo_xcb_picture_backend,
        &mut (*(*screen).connection).device,
        _cairo_content_from_pixman_format(pixman_format),
        0 as libc::c_int,
    );
    cairo_list_add(&mut (*surface).link, &mut (*screen).pictures);
    let ref mut fresh13 = (*surface).screen;
    *fresh13 = screen;
    (*surface).picture = xcb_generate_id((*(*screen).connection).xcb_connection);
    (*surface).pixman_format = pixman_format;
    (*surface).xrender_format = xrender_format;
    let ref mut fresh14 = (*surface).y0;
    *fresh14 = 0 as libc::c_int;
    (*surface).x0 = *fresh14;
    let ref mut fresh15 = (*surface).y;
    *fresh15 = 0 as libc::c_int;
    (*surface).x = *fresh15;
    (*surface).width = width;
    (*surface).height = height;
    (*surface).transform = identity_transform;
    (*surface).extend = CAIRO_EXTEND_NONE;
    (*surface).filter = CAIRO_FILTER_NEAREST;
    (*surface).has_component_alpha = 0 as libc::c_int;
    return surface;
}
#[inline]
unsafe extern "C" fn _operator_is_supported(
    mut flags: uint32_t,
    mut op: cairo_operator_t,
) -> cairo_bool_t {
    if op as libc::c_uint <= CAIRO_OPERATOR_SATURATE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    if op as libc::c_uint <= CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int as libc::c_uint
    {
        return (flags
            & CAIRO_XCB_RENDER_HAS_PDF_OPERATORS as libc::c_int as libc::c_uint)
            as cairo_bool_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _render_operator(mut op: cairo_operator_t) -> libc::c_int {
    match op as libc::c_uint {
        0 => return XCB_RENDER_PICT_OP_CLEAR as libc::c_int,
        1 => return XCB_RENDER_PICT_OP_SRC as libc::c_int,
        2 => return XCB_RENDER_PICT_OP_OVER as libc::c_int,
        3 => return XCB_RENDER_PICT_OP_IN as libc::c_int,
        4 => return XCB_RENDER_PICT_OP_OUT as libc::c_int,
        5 => return XCB_RENDER_PICT_OP_ATOP as libc::c_int,
        6 => return XCB_RENDER_PICT_OP_DST as libc::c_int,
        7 => return XCB_RENDER_PICT_OP_OVER_REVERSE as libc::c_int,
        8 => return XCB_RENDER_PICT_OP_IN_REVERSE as libc::c_int,
        9 => return XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int,
        10 => return XCB_RENDER_PICT_OP_ATOP_REVERSE as libc::c_int,
        11 => return XCB_RENDER_PICT_OP_XOR as libc::c_int,
        12 => return XCB_RENDER_PICT_OP_ADD as libc::c_int,
        13 => return XCB_RENDER_PICT_OP_SATURATE as libc::c_int,
        14 => return XCB_RENDER_PICT_OP_MULTIPLY as libc::c_int,
        15 => return XCB_RENDER_PICT_OP_SCREEN as libc::c_int,
        16 => return XCB_RENDER_PICT_OP_OVERLAY as libc::c_int,
        17 => return XCB_RENDER_PICT_OP_DARKEN as libc::c_int,
        18 => return XCB_RENDER_PICT_OP_LIGHTEN as libc::c_int,
        19 => return XCB_RENDER_PICT_OP_COLOR_DODGE as libc::c_int,
        20 => return XCB_RENDER_PICT_OP_COLOR_BURN as libc::c_int,
        21 => return XCB_RENDER_PICT_OP_HARD_LIGHT as libc::c_int,
        22 => return XCB_RENDER_PICT_OP_SOFT_LIGHT as libc::c_int,
        23 => return XCB_RENDER_PICT_OP_DIFFERENCE as libc::c_int,
        24 => return XCB_RENDER_PICT_OP_EXCLUSION as libc::c_int,
        25 => return XCB_RENDER_PICT_OP_HSL_HUE as libc::c_int,
        26 => return XCB_RENDER_PICT_OP_HSL_SATURATION as libc::c_int,
        27 => return XCB_RENDER_PICT_OP_HSL_COLOR as libc::c_int,
        28 => return XCB_RENDER_PICT_OP_HSL_LUMINOSITY as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    210 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"int _render_operator(cairo_operator_t)\0"))
                        .as_ptr(),
                );
            }
            return XCB_RENDER_PICT_OP_OVER as libc::c_int;
        }
    };
}
unsafe extern "C" fn _cairo_xcb_surface_set_clip_region(
    mut surface: *mut cairo_xcb_surface_t,
    mut region: *mut cairo_region_t,
) -> cairo_status_t {
    let mut stack_rects: [xcb_rectangle_t; 256] = [xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 256];
    let mut rects: *mut xcb_rectangle_t = stack_rects.as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut num_rects: libc::c_int = 0;
    num_rects = cairo_region_num_rectangles(region);
    if num_rects
        > (::std::mem::size_of::<[xcb_rectangle_t; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        rects = _cairo_malloc_ab(
            num_rects as size_t,
            ::std::mem::size_of::<xcb_rectangle_t>() as libc::c_ulong,
        ) as *mut xcb_rectangle_t;
        if rects.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    i = 0 as libc::c_int;
    while i < num_rects {
        let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        cairo_region_get_rectangle(region, i, &mut rect);
        (*rects.offset(i as isize)).x = rect.x as int16_t;
        (*rects.offset(i as isize)).y = rect.y as int16_t;
        (*rects.offset(i as isize)).width = rect.width as uint16_t;
        (*rects.offset(i as isize)).height = rect.height as uint16_t;
        i += 1;
    }
    _cairo_xcb_connection_render_set_picture_clip_rectangles(
        (*surface).connection,
        (*surface).picture,
        0 as libc::c_int as int16_t,
        0 as libc::c_int as int16_t,
        num_rects as uint32_t,
        rects,
    );
    if rects != stack_rects.as_mut_ptr() {
        free(rects as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xcb_surface_clear_clip_region(
    mut surface: *mut cairo_xcb_surface_t,
) {
    let mut values: [uint32_t; 1] = [0 as libc::c_long as uint32_t];
    _cairo_xcb_connection_render_change_picture(
        (*surface).connection,
        (*surface).picture,
        XCB_RENDER_CP_CLIP_MASK as libc::c_int as uint32_t,
        values.as_mut_ptr(),
    );
}
unsafe extern "C" fn _cairo_xcb_surface_set_precision(
    mut surface: *mut cairo_xcb_surface_t,
    mut antialias: cairo_antialias_t,
) {
    let mut connection: *mut cairo_xcb_connection_t = (*surface).connection;
    let mut precision: uint32_t = 0;
    if (*connection).force_precision != -(1 as libc::c_int) {
        precision = (*connection).force_precision as uint32_t;
    } else {
        match antialias as libc::c_uint {
            3 | 6 => {
                precision = XCB_RENDER_POLY_MODE_PRECISE as libc::c_int as uint32_t;
            }
            0 | 2 | 1 | 4 | 5 | _ => {
                precision = XCB_RENDER_POLY_MODE_IMPRECISE as libc::c_int as uint32_t;
            }
        }
    }
    if (*surface).precision != precision {
        _cairo_xcb_connection_render_change_picture(
            connection,
            (*surface).picture,
            XCB_RENDER_CP_POLY_MODE as libc::c_int as uint32_t,
            &mut precision,
        );
        (*surface).precision = precision;
    }
}
unsafe extern "C" fn _cairo_xcb_surface_ensure_picture(
    mut surface: *mut cairo_xcb_surface_t,
) {
    if ((*surface).fallback).is_null() {} else {
        __assert_fail(
            b"surface->fallback == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void _cairo_xcb_surface_ensure_picture(cairo_xcb_surface_t *)\0"))
                .as_ptr(),
        );
    }
    if (*surface).picture as libc::c_long == 0 as libc::c_long {
        let mut values: [uint32_t; 1] = [0; 1];
        let mut flags: uint32_t = 0 as libc::c_int as uint32_t;
        if (*surface).precision
            != XCB_RENDER_POLY_MODE_PRECISE as libc::c_int as libc::c_uint
        {
            flags |= XCB_RENDER_CP_POLY_MODE as libc::c_int as libc::c_uint;
            values[0 as libc::c_int as usize] = (*surface).precision;
        }
        (*surface).picture = xcb_generate_id((*(*surface).connection).xcb_connection);
        _cairo_xcb_connection_render_create_picture(
            (*surface).connection,
            (*surface).picture,
            (*surface).drawable,
            (*surface).xrender_format,
            flags,
            values.as_mut_ptr(),
        );
    }
}
unsafe extern "C" fn _picture_from_image(
    mut target: *mut cairo_xcb_surface_t,
    mut format: xcb_render_pictformat_t,
    mut image: *mut cairo_image_surface_t,
    mut shm_info: *mut cairo_xcb_shm_info_t,
) -> *mut cairo_xcb_picture_t {
    let mut pixmap: xcb_pixmap_t = 0;
    let mut gc: xcb_gcontext_t = 0;
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    pixmap = _cairo_xcb_connection_create_pixmap(
        (*target).connection,
        (*image).depth as uint8_t,
        (*target).drawable,
        (*image).width as uint16_t,
        (*image).height as uint16_t,
    );
    gc = _cairo_xcb_screen_get_gc((*target).screen, pixmap, (*image).depth);
    if !shm_info.is_null() {
        _cairo_xcb_connection_shm_put_image(
            (*target).connection,
            pixmap,
            gc,
            (*image).width as uint16_t,
            (*image).height as uint16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*image).width as uint16_t,
            (*image).height as uint16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*image).depth as uint8_t,
            (*shm_info).shm,
            (*shm_info).offset,
        );
    } else {
        let mut len: libc::c_int = 0;
        len = (((((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
            & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint)
            << ((*image).pixman_format as libc::c_uint >> 22 as libc::c_int
                & 3 as libc::c_int as libc::c_uint))
            .wrapping_mul((*image).width as libc::c_uint)
            .wrapping_add(7 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
            as libc::c_int;
        if len as libc::c_long == (*image).stride {
            _cairo_xcb_connection_put_image(
                (*target).connection,
                pixmap,
                gc,
                (*image).width as uint16_t,
                (*image).height as uint16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                (*image).depth as uint8_t,
                (*image).stride as uint32_t,
                (*image).data as *mut libc::c_void,
            );
        } else {
            _cairo_xcb_connection_put_subimage(
                (*target).connection,
                pixmap,
                gc,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                (*image).width as uint16_t,
                (*image).height as uint16_t,
                (((*image).pixman_format as libc::c_uint >> 24 as libc::c_int
                    & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint)
                    << ((*image).pixman_format as libc::c_uint >> 22 as libc::c_int
                        & 3 as libc::c_int as libc::c_uint))
                    .wrapping_div(8 as libc::c_int as libc::c_uint) as uint16_t,
                (*image).stride as libc::c_int,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                (*image).depth as uint8_t,
                (*image).data as *mut libc::c_void,
            );
        }
    }
    _cairo_xcb_screen_put_gc((*target).screen, (*image).depth, gc);
    picture = _cairo_xcb_picture_create(
        (*target).screen,
        (*image).pixman_format,
        format,
        (*image).width,
        (*image).height,
    );
    if (*picture).base.status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        _cairo_xcb_connection_render_create_picture(
            (*target).connection,
            (*picture).picture,
            pixmap,
            format,
            0 as libc::c_int as uint32_t,
            0 as *mut uint32_t,
        );
    }
    xcb_free_pixmap((*(*target).connection).xcb_connection, pixmap);
    return picture;
}
unsafe extern "C" fn _pattern_is_supported(
    mut flags: uint32_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    match (*pattern).extend as libc::c_uint {
        0 | 1 => {}
        3 | 2 => {
            if flags
                & CAIRO_XCB_RENDER_HAS_EXTENDED_REPEAT as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    400 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 70],
                        &[libc::c_char; 70],
                    >(
                        b"cairo_bool_t _pattern_is_supported(uint32_t, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        match (*pattern).filter as libc::c_uint {
            0 | 3 => {
                return (flags
                    & CAIRO_XCB_RENDER_HAS_PICTURE_TRANSFORM as libc::c_int
                        as libc::c_uint != 0
                    || _cairo_matrix_is_integer_translation(
                        &(*pattern).matrix,
                        0 as *mut libc::c_int,
                        0 as *mut libc::c_int,
                    ) != 0) as libc::c_int;
            }
            1 => {
                return (flags
                    & CAIRO_XCB_RENDER_HAS_FILTER_GOOD as libc::c_int as libc::c_uint)
                    as cairo_bool_t;
            }
            2 => {
                return (flags
                    & CAIRO_XCB_RENDER_HAS_FILTER_BEST as libc::c_int as libc::c_uint)
                    as cairo_bool_t;
            }
            4 | 5 | _ => {
                return (flags
                    & CAIRO_XCB_RENDER_HAS_FILTERS as libc::c_int as libc::c_uint)
                    as cairo_bool_t;
            }
        }
    } else if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int
    } else {
        if flags & CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        if (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
            && _cairo_radial_pattern_focus_is_inside(
                pattern as *mut cairo_radial_pattern_t,
            ) == 0
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn _cairo_xcb_picture_set_matrix(
    mut picture: *mut cairo_xcb_picture_t,
    mut matrix: *const cairo_matrix_t,
    mut filter: cairo_filter_t,
    mut xc: libc::c_double,
    mut yc: libc::c_double,
) {
    let mut transform: xcb_render_transform_t = xcb_render_transform_t {
        matrix11: 0,
        matrix12: 0,
        matrix13: 0,
        matrix21: 0,
        matrix22: 0,
        matrix23: 0,
        matrix31: 0,
        matrix32: 0,
        matrix33: 0,
    };
    let mut pixman_transform: *mut pixman_transform_t = 0 as *mut pixman_transform_t;
    let mut ignored: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    pixman_transform = &mut transform as *mut xcb_render_transform_t
        as *mut pixman_transform_t;
    (*picture).x = (*picture).x0;
    (*picture).y = (*picture).y0;
    ignored = _cairo_matrix_to_pixman_matrix_offset(
        matrix,
        filter,
        xc,
        yc,
        pixman_transform,
        &mut (*picture).x,
        &mut (*picture).y,
    ) as cairo_int_status_t;
    if memcmp(
        &mut (*picture).transform as *mut xcb_render_transform_t as *const libc::c_void,
        &mut transform as *mut xcb_render_transform_t as *const libc::c_void,
        ::std::mem::size_of::<xcb_render_transform_t>() as libc::c_ulong,
    ) != 0
    {
        _cairo_xcb_connection_render_set_picture_transform(
            _picture_to_connection(picture),
            (*picture).picture,
            &mut transform,
        );
        (*picture).transform = transform;
    }
}
unsafe extern "C" fn _cairo_xcb_picture_set_filter(
    mut picture: *mut cairo_xcb_picture_t,
    mut filter: cairo_filter_t,
) {
    let mut render_filter: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    if (*picture).filter as libc::c_uint == filter as libc::c_uint {
        return;
    }
    let mut current_block_16: u64;
    match filter as libc::c_uint {
        0 => {
            render_filter = b"fast\0" as *const u8 as *const libc::c_char;
            len = strlen(b"fast\0" as *const u8 as *const libc::c_char) as libc::c_int;
            current_block_16 = 15089075282327824602;
        }
        1 => {
            render_filter = b"good\0" as *const u8 as *const libc::c_char;
            len = strlen(b"good\0" as *const u8 as *const libc::c_char) as libc::c_int;
            current_block_16 = 15089075282327824602;
        }
        2 => {
            render_filter = b"best\0" as *const u8 as *const libc::c_char;
            len = strlen(b"best\0" as *const u8 as *const libc::c_char) as libc::c_int;
            current_block_16 = 15089075282327824602;
        }
        3 => {
            render_filter = b"nearest\0" as *const u8 as *const libc::c_char;
            len = strlen(b"nearest\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            current_block_16 = 15089075282327824602;
        }
        4 => {
            render_filter = b"bilinear\0" as *const u8 as *const libc::c_char;
            len = strlen(b"bilinear\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            current_block_16 = 15089075282327824602;
        }
        5 => {
            current_block_16 = 12291338484763943359;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    510 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"void _cairo_xcb_picture_set_filter(cairo_xcb_picture_t *, cairo_filter_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_16 = 12291338484763943359;
        }
    }
    match current_block_16 {
        12291338484763943359 => {
            render_filter = b"best\0" as *const u8 as *const libc::c_char;
            len = strlen(b"best\0" as *const u8 as *const libc::c_char) as libc::c_int;
        }
        _ => {}
    }
    _cairo_xcb_connection_render_set_picture_filter(
        _picture_to_connection(picture),
        (*picture).picture,
        len as uint16_t,
        render_filter as *mut libc::c_char,
    );
    (*picture).filter = filter;
}
unsafe extern "C" fn _cairo_xcb_picture_set_extend(
    mut picture: *mut cairo_xcb_picture_t,
    mut extend: cairo_extend_t,
) {
    let mut pa: [uint32_t; 1] = [0; 1];
    if (*picture).extend as libc::c_uint == extend as libc::c_uint {
        return;
    }
    let mut current_block_8: u64;
    match extend as libc::c_uint {
        0 => {
            current_block_8 = 14630865847280217768;
        }
        1 => {
            pa[0 as libc::c_int
                as usize] = XCB_RENDER_REPEAT_NORMAL as libc::c_int as uint32_t;
            current_block_8 = 11050875288958768710;
        }
        2 => {
            pa[0 as libc::c_int
                as usize] = XCB_RENDER_REPEAT_REFLECT as libc::c_int as uint32_t;
            current_block_8 = 11050875288958768710;
        }
        3 => {
            pa[0 as libc::c_int
                as usize] = XCB_RENDER_REPEAT_PAD as libc::c_int as uint32_t;
            current_block_8 = 11050875288958768710;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    534 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"void _cairo_xcb_picture_set_extend(cairo_xcb_picture_t *, cairo_extend_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            current_block_8 = 14630865847280217768;
        }
    }
    match current_block_8 {
        14630865847280217768 => {
            pa[0 as libc::c_int
                as usize] = XCB_RENDER_REPEAT_NONE as libc::c_int as uint32_t;
        }
        _ => {}
    }
    _cairo_xcb_connection_render_change_picture(
        _picture_to_connection(picture),
        (*picture).picture,
        XCB_RENDER_CP_REPEAT as libc::c_int as uint32_t,
        pa.as_mut_ptr(),
    );
    (*picture).extend = extend;
}
unsafe extern "C" fn _cairo_xcb_picture_set_component_alpha(
    mut picture: *mut cairo_xcb_picture_t,
    mut ca: cairo_bool_t,
) {
    let mut pa: [uint32_t; 1] = [0; 1];
    if (*picture).has_component_alpha == ca {
        return;
    }
    pa[0 as libc::c_int as usize] = ca as uint32_t;
    _cairo_xcb_connection_render_change_picture(
        _picture_to_connection(picture),
        (*picture).picture,
        XCB_RENDER_CP_COMPONENT_ALPHA as libc::c_int as uint32_t,
        pa.as_mut_ptr(),
    );
    (*picture).has_component_alpha = ca;
}
unsafe extern "C" fn _solid_picture(
    mut target: *mut cairo_xcb_surface_t,
    mut color: *const cairo_color_t,
) -> *mut cairo_xcb_picture_t {
    let mut xcb_color: xcb_render_color_t = xcb_render_color_t {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let mut xrender_format: xcb_render_pictformat_t = 0;
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    xcb_color.red = (*color).red_short;
    xcb_color.green = (*color).green_short;
    xcb_color.blue = (*color).blue_short;
    xcb_color.alpha = (*color).alpha_short;
    xrender_format = (*(*(*target).screen).connection)
        .standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int as usize];
    picture = _cairo_xcb_picture_create(
        (*target).screen,
        PIXMAN_a8r8g8b8,
        xrender_format,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    );
    if (*picture).base.status as u64 != 0 {
        return picture;
    }
    if (*(*target).connection).flags
        & CAIRO_XCB_RENDER_HAS_GRADIENTS as libc::c_int as libc::c_uint != 0
    {
        _cairo_xcb_connection_render_create_solid_fill(
            (*target).connection,
            (*picture).picture,
            xcb_color,
        );
    } else {
        let mut pixmap: xcb_pixmap_t = 0;
        let mut values: [uint32_t; 1] = [
            XCB_RENDER_REPEAT_NORMAL as libc::c_int as uint32_t,
        ];
        pixmap = _cairo_xcb_connection_create_pixmap(
            (*target).connection,
            32 as libc::c_int as uint8_t,
            (*target).drawable,
            1 as libc::c_int as uint16_t,
            1 as libc::c_int as uint16_t,
        );
        _cairo_xcb_connection_render_create_picture(
            (*target).connection,
            (*picture).picture,
            pixmap,
            xrender_format,
            XCB_RENDER_CP_REPEAT as libc::c_int as uint32_t,
            values.as_mut_ptr(),
        );
        if (*(*target).connection).flags
            & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
        {
            let mut rect: xcb_rectangle_t = xcb_rectangle_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            rect.y = 0 as libc::c_int as int16_t;
            rect.x = rect.y;
            rect.height = 1 as libc::c_int as uint16_t;
            rect.width = rect.height;
            _cairo_xcb_connection_render_fill_rectangles(
                _picture_to_connection(picture),
                XCB_RENDER_PICT_OP_SRC as libc::c_int as uint8_t,
                (*picture).picture,
                xcb_color,
                1 as libc::c_int as uint32_t,
                &mut rect,
            );
        } else {
            let mut gc: xcb_gcontext_t = 0;
            let mut pixel: uint32_t = 0;
            gc = _cairo_xcb_screen_get_gc((*target).screen, pixmap, 32 as libc::c_int);
            pixel = ((*color).alpha_short as uint32_t >> 8 as libc::c_int)
                << 24 as libc::c_int
                | (((*color).red_short as libc::c_int >> 8 as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint
                | (((*color).green_short as libc::c_int >> 8 as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint
                | (((*color).blue_short as libc::c_int >> 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_uint;
            _cairo_xcb_connection_put_image(
                (*target).connection,
                pixmap,
                gc,
                1 as libc::c_int as uint16_t,
                1 as libc::c_int as uint16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                32 as libc::c_int as uint8_t,
                4 as libc::c_int as uint32_t,
                &mut pixel as *mut uint32_t as *mut libc::c_void,
            );
            _cairo_xcb_screen_put_gc((*target).screen, 32 as libc::c_int, gc);
        }
        xcb_free_pixmap((*(*target).connection).xcb_connection, pixmap);
    }
    return picture;
}
unsafe extern "C" fn _cairo_xcb_transparent_picture(
    mut target: *mut cairo_xcb_surface_t,
) -> *mut cairo_xcb_picture_t {
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    picture = (*(*target).screen)
        .stock_colors[CAIRO_STOCK_TRANSPARENT as libc::c_int as usize]
        as *mut cairo_xcb_picture_t;
    if picture.is_null() {
        picture = _solid_picture(target, _cairo_stock_color(CAIRO_STOCK_TRANSPARENT));
        let ref mut fresh16 = (*(*target).screen)
            .stock_colors[CAIRO_STOCK_TRANSPARENT as libc::c_int as usize];
        *fresh16 = &mut (*picture).base;
    }
    return cairo_surface_reference(&mut (*picture).base) as *mut cairo_xcb_picture_t;
}
unsafe extern "C" fn _cairo_xcb_black_picture(
    mut target: *mut cairo_xcb_surface_t,
) -> *mut cairo_xcb_picture_t {
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    picture = (*(*target).screen).stock_colors[CAIRO_STOCK_BLACK as libc::c_int as usize]
        as *mut cairo_xcb_picture_t;
    if picture.is_null() {
        picture = _solid_picture(target, _cairo_stock_color(CAIRO_STOCK_BLACK));
        let ref mut fresh17 = (*(*target).screen)
            .stock_colors[CAIRO_STOCK_BLACK as libc::c_int as usize];
        *fresh17 = &mut (*picture).base;
    }
    return cairo_surface_reference(&mut (*picture).base) as *mut cairo_xcb_picture_t;
}
unsafe extern "C" fn _cairo_xcb_white_picture(
    mut target: *mut cairo_xcb_surface_t,
) -> *mut cairo_xcb_picture_t {
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    picture = (*(*target).screen).stock_colors[CAIRO_STOCK_WHITE as libc::c_int as usize]
        as *mut cairo_xcb_picture_t;
    if picture.is_null() {
        picture = _solid_picture(target, _cairo_stock_color(CAIRO_STOCK_WHITE));
        let ref mut fresh18 = (*(*target).screen)
            .stock_colors[CAIRO_STOCK_WHITE as libc::c_int as usize];
        *fresh18 = &mut (*picture).base;
    }
    return cairo_surface_reference(&mut (*picture).base) as *mut cairo_xcb_picture_t;
}
unsafe extern "C" fn _cairo_xcb_solid_picture(
    mut target: *mut cairo_xcb_surface_t,
    mut pattern: *const cairo_solid_pattern_t,
) -> *mut cairo_xcb_picture_t {
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut screen: *mut cairo_xcb_screen_t = 0 as *mut cairo_xcb_screen_t;
    let mut i: libc::c_int = 0;
    let mut n_cached: libc::c_int = 0;
    if (*pattern).color.alpha_short as libc::c_int <= 0xff as libc::c_int {
        return _cairo_xcb_transparent_picture(target);
    }
    if (*pattern).color.alpha_short as libc::c_int >= 0xff00 as libc::c_int {
        if (*pattern).color.red_short as libc::c_int <= 0xff as libc::c_int
            && (*pattern).color.green_short as libc::c_int <= 0xff as libc::c_int
            && (*pattern).color.blue_short as libc::c_int <= 0xff as libc::c_int
        {
            return _cairo_xcb_black_picture(target);
        }
        if (*pattern).color.red_short as libc::c_int >= 0xff00 as libc::c_int
            && (*pattern).color.green_short as libc::c_int >= 0xff00 as libc::c_int
            && (*pattern).color.blue_short as libc::c_int >= 0xff00 as libc::c_int
        {
            return _cairo_xcb_white_picture(target);
        }
    }
    screen = (*target).screen;
    n_cached = (*screen).solid_cache_size;
    i = 0 as libc::c_int;
    while i < n_cached {
        if _cairo_color_equal(
            &mut (*((*screen).solid_cache).as_mut_ptr().offset(i as isize)).color,
            &(*pattern).color,
        ) != 0
        {
            return cairo_surface_reference((*screen).solid_cache[i as usize].picture)
                as *mut cairo_xcb_picture_t;
        }
        i += 1;
    }
    picture = _solid_picture(target, &(*pattern).color);
    if (*picture).base.status as u64 != 0 {
        return picture;
    }
    if (*screen).solid_cache_size
        < (::std::mem::size_of::<[C2RustUnnamed_0; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
            as libc::c_int
    {
        let ref mut fresh19 = (*screen).solid_cache_size;
        let fresh20 = *fresh19;
        *fresh19 = *fresh19 + 1;
        i = fresh20;
    } else {
        i = (hars_petruska_f54_1_random())
            .wrapping_rem(
                (::std::mem::size_of::<[C2RustUnnamed_0; 16]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
                    ) as libc::c_int as libc::c_uint,
            ) as libc::c_int;
        cairo_surface_destroy((*screen).solid_cache[i as usize].picture);
    }
    let ref mut fresh21 = (*screen).solid_cache[i as usize].picture;
    *fresh21 = cairo_surface_reference(&mut (*picture).base);
    (*screen).solid_cache[i as usize].color = (*pattern).color;
    return picture;
}
unsafe extern "C" fn _render_to_picture(
    mut target: *mut cairo_xcb_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_picture_t {
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut shm_info: *mut cairo_xcb_shm_info_t = 0 as *mut cairo_xcb_shm_info_t;
    let mut copy: cairo_pattern_union_t = cairo_pattern_union_t {
        base: cairo_pattern_t {
            ref_count: cairo_reference_count_t {
                ref_count: 0,
            },
            status: CAIRO_STATUS_SUCCESS,
            user_data: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
            observers: cairo_list_t {
                next: 0 as *const _cairo_list as *mut _cairo_list,
                prev: 0 as *const _cairo_list as *mut _cairo_list,
            },
            type_0: CAIRO_PATTERN_TYPE_SOLID,
            filter: CAIRO_FILTER_FAST,
            extend: CAIRO_EXTEND_NONE,
            has_component_alpha: 0,
            is_userfont_foreground: 0,
            matrix: cairo_matrix_t {
                xx: 0.,
                yx: 0.,
                xy: 0.,
                yy: 0.,
                x0: 0.,
                y0: 0.,
            },
            opacity: 0.,
        },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut pixman_format: pixman_format_code_t = 0 as pixman_format_code_t;
    let mut xrender_format: xcb_render_pictformat_t = 0;
    pixman_format = PIXMAN_a8r8g8b8;
    xrender_format = (*(*(*target).screen).connection)
        .standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int as usize];
    status = _cairo_xcb_shm_image_create(
        (*(*target).screen).connection,
        pixman_format,
        (*extents).width,
        (*extents).height,
        &mut image,
        &mut shm_info,
    );
    if status as u64 != 0 {
        return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
    }
    _cairo_pattern_init_static_copy(&mut copy.base, pattern);
    cairo_matrix_translate(
        &mut copy.base.matrix,
        (*extents).x as libc::c_double,
        (*extents).y as libc::c_double,
    );
    status = _cairo_surface_paint(
        &mut (*image).base,
        CAIRO_OPERATOR_SOURCE,
        &mut copy.base,
        0 as *const cairo_clip_t,
    );
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*image).base);
        return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
    }
    picture = _picture_from_image(target, xrender_format, image, shm_info);
    cairo_surface_destroy(&mut (*image).base);
    if (*picture).base.status as u64 != 0 {
        return picture;
    }
    _cairo_xcb_picture_set_component_alpha(picture, (*pattern).has_component_alpha);
    (*picture).x = -(*extents).x;
    (*picture).y = -(*extents).y;
    return picture;
}
unsafe extern "C" fn _gradient_to_xcb(
    mut gradient: *const cairo_gradient_pattern_t,
    mut n_stops: *mut libc::c_uint,
    mut buf: *mut libc::c_char,
    mut buflen: libc::c_uint,
) -> *mut xcb_render_fixed_t {
    let mut stops: *mut xcb_render_fixed_t = 0 as *mut xcb_render_fixed_t;
    let mut colors: *mut xcb_render_color_t = 0 as *mut xcb_render_color_t;
    let mut i: libc::c_uint = 0;
    if (*gradient).n_stops > 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"gradient->n_stops > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            801 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"xcb_render_fixed_t *_gradient_to_xcb(const cairo_gradient_pattern_t *, unsigned int *, char *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    *n_stops = if (*gradient).n_stops > 2 as libc::c_int as libc::c_uint {
        (*gradient).n_stops
    } else {
        2 as libc::c_int as libc::c_uint
    };
    if (*n_stops as libc::c_ulong)
        .wrapping_mul(
            (::std::mem::size_of::<xcb_render_fixed_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<xcb_render_color_t>() as libc::c_ulong,
                ),
        ) < buflen as libc::c_ulong
    {
        stops = buf as *mut xcb_render_fixed_t;
    } else {
        stops = _cairo_malloc_ab(
            *n_stops as size_t,
            (::std::mem::size_of::<xcb_render_fixed_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<xcb_render_color_t>() as libc::c_ulong,
                ),
        ) as *mut xcb_render_fixed_t;
        if stops.is_null() {
            return 0 as *mut xcb_render_fixed_t;
        }
    }
    colors = stops.offset(*n_stops as isize) as *mut xcb_render_color_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*gradient).n_stops {
        *stops
            .offset(
                i as isize,
            ) = _cairo_fixed_16_16_from_double(
            (*((*gradient).stops).offset(i as isize)).offset,
        );
        (*colors.offset(i as isize))
            .red = (*((*gradient).stops).offset(i as isize)).color.red_short;
        (*colors.offset(i as isize))
            .green = (*((*gradient).stops).offset(i as isize)).color.green_short;
        (*colors.offset(i as isize))
            .blue = (*((*gradient).stops).offset(i as isize)).color.blue_short;
        (*colors.offset(i as isize))
            .alpha = (*((*gradient).stops).offset(i as isize)).color.alpha_short;
        i = i.wrapping_add(1);
    }
    if (*gradient).n_stops == 1 as libc::c_int as libc::c_uint {
        *stops
            .offset(
                1 as libc::c_int as isize,
            ) = _cairo_fixed_16_16_from_double(
            (*((*gradient).stops).offset(0 as libc::c_int as isize)).offset,
        );
        (*colors.offset(1 as libc::c_int as isize))
            .red = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .red_short;
        (*colors.offset(1 as libc::c_int as isize))
            .green = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .green_short;
        (*colors.offset(1 as libc::c_int as isize))
            .blue = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .blue_short;
        (*colors.offset(1 as libc::c_int as isize))
            .alpha = (*((*gradient).stops).offset(0 as libc::c_int as isize))
            .color
            .alpha_short;
    }
    return stops;
}
unsafe extern "C" fn _cairo_xcb_linear_picture(
    mut target: *mut cairo_xcb_surface_t,
    mut pattern: *const cairo_linear_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_picture_t {
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut stops: *mut xcb_render_fixed_t = 0 as *mut xcb_render_fixed_t;
    let mut colors: *mut xcb_render_color_t = 0 as *mut xcb_render_color_t;
    let mut p1: xcb_render_pointfix_t = xcb_render_pointfix_t {
        x: 0,
        y: 0,
    };
    let mut p2: xcb_render_pointfix_t = xcb_render_pointfix_t {
        x: 0,
        y: 0,
    };
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut extremes: [cairo_circle_double_t; 2] = [cairo_circle_double_t {
        center: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        radius: 0.,
    }; 2];
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut n_stops: libc::c_uint = 0;
    _cairo_gradient_pattern_fit_to_range(
        &(*pattern).base,
        ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
            >> 1 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
            as libc::c_double,
        &mut matrix,
        extremes.as_mut_ptr(),
    );
    picture = _cairo_xcb_screen_lookup_linear_picture((*target).screen, pattern)
        as *mut cairo_xcb_picture_t;
    if picture.is_null() {
        stops = _gradient_to_xcb(
            &(*pattern).base,
            &mut n_stops,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_uint,
        );
        if stops.is_null() {
            return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY)
                as *mut cairo_xcb_picture_t;
        }
        picture = _cairo_xcb_picture_create(
            (*target).screen,
            (*(*(*target).screen).connection)
                .standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int as usize]
                as pixman_format_code_t,
            PIXMAN_a8r8g8b8 as libc::c_int as xcb_render_pictformat_t,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        );
        if (*picture).base.status as u64 != 0 {
            if stops != buf.as_mut_ptr() as *mut xcb_render_fixed_t {
                free(stops as *mut libc::c_void);
            }
            return picture;
        }
        (*picture).filter = CAIRO_FILTER_GOOD;
        colors = stops.offset(n_stops as isize) as *mut xcb_render_color_t;
        p1
            .x = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.x,
        );
        p1
            .y = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.y,
        );
        p2
            .x = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.x,
        );
        p2
            .y = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.y,
        );
        _cairo_xcb_connection_render_create_linear_gradient(
            (*target).connection,
            (*picture).picture,
            p1,
            p2,
            n_stops,
            stops,
            colors,
        );
        if stops != buf.as_mut_ptr() as *mut xcb_render_fixed_t {
            free(stops as *mut libc::c_void);
        }
        status = _cairo_xcb_screen_store_linear_picture(
            (*target).screen,
            pattern,
            &mut (*picture).base,
        );
        if status as u64 != 0 {
            cairo_surface_destroy(&mut (*picture).base);
            return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
        }
    }
    _cairo_xcb_picture_set_matrix(
        picture,
        &mut matrix,
        (*pattern).base.base.filter,
        (*extents).x as libc::c_double + (*extents).width as libc::c_double / 2.0f64,
        (*extents).y as libc::c_double + (*extents).height as libc::c_double / 2.0f64,
    );
    _cairo_xcb_picture_set_filter(picture, (*pattern).base.base.filter);
    _cairo_xcb_picture_set_extend(picture, (*pattern).base.base.extend);
    _cairo_xcb_picture_set_component_alpha(
        picture,
        (*pattern).base.base.has_component_alpha,
    );
    return picture;
}
unsafe extern "C" fn _cairo_xcb_radial_picture(
    mut target: *mut cairo_xcb_surface_t,
    mut pattern: *const cairo_radial_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_picture_t {
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut stops: *mut xcb_render_fixed_t = 0 as *mut xcb_render_fixed_t;
    let mut colors: *mut xcb_render_color_t = 0 as *mut xcb_render_color_t;
    let mut p1: xcb_render_pointfix_t = xcb_render_pointfix_t {
        x: 0,
        y: 0,
    };
    let mut p2: xcb_render_pointfix_t = xcb_render_pointfix_t {
        x: 0,
        y: 0,
    };
    let mut r1: xcb_render_fixed_t = 0;
    let mut r2: xcb_render_fixed_t = 0;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut extremes: [cairo_circle_double_t; 2] = [cairo_circle_double_t {
        center: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        radius: 0.,
    }; 2];
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut n_stops: libc::c_uint = 0;
    _cairo_gradient_pattern_fit_to_range(
        &(*pattern).base,
        ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
            >> 1 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
            as libc::c_double,
        &mut matrix,
        extremes.as_mut_ptr(),
    );
    picture = _cairo_xcb_screen_lookup_radial_picture((*target).screen, pattern)
        as *mut cairo_xcb_picture_t;
    if picture.is_null() {
        stops = _gradient_to_xcb(
            &(*pattern).base,
            &mut n_stops,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_uint,
        );
        if stops.is_null() {
            return _cairo_surface_create_in_error(CAIRO_STATUS_NO_MEMORY)
                as *mut cairo_xcb_picture_t;
        }
        picture = _cairo_xcb_picture_create(
            (*target).screen,
            (*(*(*target).screen).connection)
                .standard_formats[CAIRO_FORMAT_ARGB32 as libc::c_int as usize]
                as pixman_format_code_t,
            PIXMAN_a8r8g8b8 as libc::c_int as xcb_render_pictformat_t,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        );
        if (*picture).base.status as u64 != 0 {
            if stops != buf.as_mut_ptr() as *mut xcb_render_fixed_t {
                free(stops as *mut libc::c_void);
            }
            return picture;
        }
        (*picture).filter = CAIRO_FILTER_GOOD;
        colors = stops.offset(n_stops as isize) as *mut xcb_render_color_t;
        p1
            .x = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.x,
        );
        p1
            .y = _cairo_fixed_16_16_from_double(
            extremes[0 as libc::c_int as usize].center.y,
        );
        p2
            .x = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.x,
        );
        p2
            .y = _cairo_fixed_16_16_from_double(
            extremes[1 as libc::c_int as usize].center.y,
        );
        r1 = _cairo_fixed_16_16_from_double(extremes[0 as libc::c_int as usize].radius);
        r2 = _cairo_fixed_16_16_from_double(extremes[1 as libc::c_int as usize].radius);
        _cairo_xcb_connection_render_create_radial_gradient(
            (*target).connection,
            (*picture).picture,
            p1,
            p2,
            r1,
            r2,
            n_stops,
            stops,
            colors,
        );
        if stops != buf.as_mut_ptr() as *mut xcb_render_fixed_t {
            free(stops as *mut libc::c_void);
        }
        status = _cairo_xcb_screen_store_radial_picture(
            (*target).screen,
            pattern,
            &mut (*picture).base,
        );
        if status as u64 != 0 {
            cairo_surface_destroy(&mut (*picture).base);
            return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
        }
    }
    _cairo_xcb_picture_set_matrix(
        picture,
        &mut matrix,
        (*pattern).base.base.filter,
        (*extents).x as libc::c_double + (*extents).width as libc::c_double / 2.0f64,
        (*extents).y as libc::c_double + (*extents).height as libc::c_double / 2.0f64,
    );
    _cairo_xcb_picture_set_filter(picture, (*pattern).base.base.filter);
    _cairo_xcb_picture_set_extend(picture, (*pattern).base.base.extend);
    _cairo_xcb_picture_set_component_alpha(
        picture,
        (*pattern).base.base.has_component_alpha,
    );
    return picture;
}
unsafe extern "C" fn _copy_to_picture(
    mut source: *mut cairo_xcb_surface_t,
) -> *mut cairo_xcb_picture_t {
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut values: [uint32_t; 2] = [
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
    ];
    if (*source).deferred_clear != 0 {
        let mut status: cairo_status_t = _cairo_xcb_surface_clear(source);
        if status as u64 != 0 {
            return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
        }
    }
    picture = _cairo_xcb_picture_create(
        (*source).screen,
        (*source).xrender_format as pixman_format_code_t,
        (*source).pixman_format as xcb_render_pictformat_t,
        (*source).width,
        (*source).height,
    );
    if (*picture).base.status as u64 != 0 {
        return picture;
    }
    _cairo_xcb_connection_render_create_picture(
        (*source).connection,
        (*picture).picture,
        (*source).drawable,
        (*source).xrender_format,
        (XCB_RENDER_CP_GRAPHICS_EXPOSURE as libc::c_int
            | XCB_RENDER_CP_SUBWINDOW_MODE as libc::c_int) as uint32_t,
        values.as_mut_ptr(),
    );
    return picture;
}
unsafe extern "C" fn _cairo_xcb_surface_setup_surface_picture(
    mut picture: *mut cairo_xcb_picture_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) {
    let mut filter: cairo_filter_t = CAIRO_FILTER_FAST;
    filter = (*pattern).base.filter;
    if filter as libc::c_uint != CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint
        && _cairo_matrix_is_pixel_exact(&(*pattern).base.matrix) != 0
    {
        filter = CAIRO_FILTER_NEAREST;
    }
    _cairo_xcb_picture_set_filter(picture, filter);
    _cairo_xcb_picture_set_matrix(
        picture,
        &(*pattern).base.matrix,
        filter,
        (*extents).x as libc::c_double + (*extents).width as libc::c_double / 2.0f64,
        (*extents).y as libc::c_double + (*extents).height as libc::c_double / 2.0f64,
    );
    _cairo_xcb_picture_set_extend(picture, (*pattern).base.extend);
    _cairo_xcb_picture_set_component_alpha(picture, (*pattern).base.has_component_alpha);
}
unsafe extern "C" fn record_to_picture(
    mut target: *mut cairo_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_picture_t {
    let mut tmp_pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
        base: cairo_pattern_t {
            ref_count: cairo_reference_count_t {
                ref_count: 0,
            },
            status: CAIRO_STATUS_SUCCESS,
            user_data: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *const libc::c_char as *mut libc::c_char,
            },
            observers: cairo_list_t {
                next: 0 as *const _cairo_list as *mut _cairo_list,
                prev: 0 as *const _cairo_list as *mut _cairo_list,
            },
            type_0: CAIRO_PATTERN_TYPE_SOLID,
            filter: CAIRO_FILTER_FAST,
            extend: CAIRO_EXTEND_NONE,
            has_component_alpha: 0,
            is_userfont_foreground: 0,
            matrix: cairo_matrix_t {
                xx: 0.,
                yx: 0.,
                xy: 0.,
                yy: 0.,
                x0: 0.,
                y0: 0.,
            },
            opacity: 0.,
        },
        surface: 0 as *mut cairo_surface_t,
    };
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut tmp: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut limit: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut extend: cairo_extend_t = CAIRO_EXTEND_NONE;
    source = _cairo_pattern_get_source(pattern, &mut limit);
    if _cairo_surface_is_recording(source) != 0 {} else {
        __assert_fail(
            b"_cairo_surface_is_recording (source)\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            1072 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 122],
                &[libc::c_char; 122],
            >(
                b"cairo_xcb_picture_t *record_to_picture(cairo_surface_t *, const cairo_surface_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if _cairo_matrix_is_identity(&(*pattern).base.matrix) == 0 {
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        matrix = (*pattern).base.matrix;
        status = cairo_matrix_invert(&mut matrix);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                1079 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"cairo_xcb_picture_t *record_to_picture(cairo_surface_t *, const cairo_surface_pattern_t *, const cairo_rectangle_int_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        x1 = limit.x as libc::c_double;
        y1 = limit.y as libc::c_double;
        x2 = (limit.x + limit.width) as libc::c_double;
        y2 = (limit.y + limit.height) as libc::c_double;
        _cairo_matrix_transform_bounding_box(
            &mut matrix,
            &mut x1,
            &mut y1,
            &mut x2,
            &mut y2,
            0 as *mut cairo_bool_t,
        );
        limit.x = floor(x1) as libc::c_int;
        limit.y = floor(y1) as libc::c_int;
        limit.width = (ceil(x2) - limit.x as libc::c_double) as libc::c_int;
        limit.height = (ceil(y2) - limit.y as libc::c_double) as libc::c_int;
    }
    extend = (*pattern).base.extend;
    if _cairo_rectangle_contains_rectangle(&mut limit, extents) != 0 {
        extend = CAIRO_EXTEND_NONE;
    }
    if extend as libc::c_uint == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        && _cairo_rectangle_intersect(&mut limit, extents) == 0
    {
        return _cairo_xcb_transparent_picture(target as *mut cairo_xcb_surface_t);
    }
    tmp = _cairo_surface_create_scratch(
        target,
        (*source).content,
        limit.width,
        limit.height,
        _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
    );
    if (*tmp).status as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        return tmp as *mut cairo_xcb_picture_t;
    }
    cairo_matrix_init_translate(
        &mut matrix,
        limit.x as libc::c_double,
        limit.y as libc::c_double,
    );
    cairo_matrix_multiply(&mut matrix, &mut matrix, &(*pattern).base.matrix);
    status = _cairo_recording_surface_replay_with_clip(
        source,
        &mut matrix,
        tmp,
        0 as *const cairo_clip_t,
    );
    if status as u64 != 0 {
        cairo_surface_destroy(tmp);
        return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
    }
    _cairo_pattern_init_static_copy(&mut tmp_pattern.base, &(*pattern).base);
    tmp_pattern.surface = tmp;
    cairo_matrix_init_translate(
        &mut tmp_pattern.base.matrix,
        -limit.x as libc::c_double,
        -limit.y as libc::c_double,
    );
    picture = _copy_to_picture(tmp as *mut cairo_xcb_surface_t);
    if (*picture).base.status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        _cairo_xcb_surface_setup_surface_picture(picture, &mut tmp_pattern, extents);
    }
    cairo_surface_destroy(tmp);
    return picture;
}
unsafe extern "C" fn _cairo_xcb_surface_picture(
    mut target: *mut cairo_xcb_surface_t,
    mut pattern: *const cairo_surface_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_picture_t {
    let mut source: *mut cairo_surface_t = (*pattern).surface;
    let mut picture: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    picture = _cairo_surface_has_snapshot(source, &_cairo_xcb_picture_backend)
        as *mut cairo_xcb_picture_t;
    if !picture.is_null() {
        if (*picture).screen == (*target).screen {
            picture = cairo_surface_reference(&mut (*picture).base)
                as *mut cairo_xcb_picture_t;
            _cairo_xcb_surface_setup_surface_picture(picture, pattern, extents);
            return picture;
        }
        picture = 0 as *mut cairo_xcb_picture_t;
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_XCB as libc::c_int as libc::c_uint
    {
        if _cairo_surface_is_xcb(source) != 0 {
            let mut xcb: *mut cairo_xcb_surface_t = source as *mut cairo_xcb_surface_t;
            if (*xcb).screen == (*target).screen && ((*xcb).fallback).is_null() {
                picture = _copy_to_picture(source as *mut cairo_xcb_surface_t);
                if (*picture).base.status as u64 != 0 {
                    return picture;
                }
            }
        } else if (*(*source).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
        {
            let mut sub: *mut cairo_surface_subsurface_t = source
                as *mut cairo_surface_subsurface_t;
            let mut xcb_0: *mut cairo_xcb_surface_t = (*sub).target
                as *mut cairo_xcb_surface_t;
            if 0 as libc::c_int != 0 && (*xcb_0).screen == (*target).screen
                && ((*xcb_0).fallback).is_null()
            {
                let mut rect: xcb_rectangle_t = xcb_rectangle_t {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                picture = _copy_to_picture(xcb_0);
                if (*picture).base.status as u64 != 0 {
                    return picture;
                }
                rect.x = (*sub).extents.x as int16_t;
                rect.y = (*sub).extents.y as int16_t;
                rect.width = (*sub).extents.width as uint16_t;
                rect.height = (*sub).extents.height as uint16_t;
                _cairo_xcb_connection_render_set_picture_clip_rectangles(
                    (*xcb_0).connection,
                    (*picture).picture,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    1 as libc::c_int as uint32_t,
                    &mut rect,
                );
                (*picture).x0 = rect.x as libc::c_int;
                (*picture).y0 = rect.y as libc::c_int;
                (*picture).width = rect.width as libc::c_int;
                (*picture).height = rect.height as libc::c_int;
            }
        } else if _cairo_surface_is_snapshot(source) != 0 {
            let mut snap: *mut cairo_surface_snapshot_t = source
                as *mut cairo_surface_snapshot_t;
            let mut xcb_1: *mut cairo_xcb_surface_t = (*snap).target
                as *mut cairo_xcb_surface_t;
            if (*xcb_1).screen == (*target).screen && ((*xcb_1).fallback).is_null() {
                picture = _copy_to_picture(xcb_1);
                if (*picture).base.status as u64 != 0 {
                    return picture;
                }
            }
        }
    } else if (*source).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return record_to_picture(&mut (*target).base, pattern, extents)
    }
    if picture.is_null() {
        let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
        let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
        status = _cairo_surface_acquire_source_image(
            source,
            &mut image,
            &mut image_extra,
        );
        if status as u64 != 0 {
            return _cairo_surface_create_in_error(status) as *mut cairo_xcb_picture_t;
        }
        if (*image).format as libc::c_int != CAIRO_FORMAT_INVALID as libc::c_int {
            let mut format: xcb_render_pictformat_t = 0;
            format = (*(*(*target).screen).connection)
                .standard_formats[(*image).format as usize];
            picture = _picture_from_image(
                target,
                format,
                image,
                0 as *mut cairo_xcb_shm_info_t,
            );
            _cairo_surface_release_source_image(source, image, image_extra);
        } else {
            let mut conv: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
            let mut render_format: xcb_render_pictformat_t = 0;
            conv = _cairo_image_surface_coerce(image);
            _cairo_surface_release_source_image(source, image, image_extra);
            if (*conv).base.status as u64 != 0 {
                return conv as *mut cairo_xcb_picture_t;
            }
            render_format = (*(*(*target).screen).connection)
                .standard_formats[(*conv).format as usize];
            picture = _picture_from_image(
                target,
                render_format,
                conv,
                0 as *mut cairo_xcb_shm_info_t,
            );
            cairo_surface_destroy(&mut (*conv).base);
        }
        if (*picture).base.status as u64 != 0 {
            return picture;
        }
    }
    _cairo_xcb_surface_setup_surface_picture(picture, pattern, extents);
    return picture;
}
unsafe extern "C" fn _cairo_xcb_picture_for_pattern(
    mut target: *mut cairo_xcb_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_picture_t {
    if pattern.is_null() {
        return _cairo_xcb_white_picture(target);
    }
    if _pattern_is_supported((*(*target).connection).flags, pattern) == 0 {
        return _render_to_picture(target, pattern, extents);
    }
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            return _cairo_xcb_solid_picture(
                target,
                pattern as *mut cairo_solid_pattern_t,
            );
        }
        2 => {
            return _cairo_xcb_linear_picture(
                target,
                pattern as *mut cairo_linear_pattern_t,
                extents,
            );
        }
        3 => {
            return _cairo_xcb_radial_picture(
                target,
                pattern as *mut cairo_radial_pattern_t,
                extents,
            );
        }
        1 => {
            return _cairo_xcb_surface_picture(
                target,
                pattern as *mut cairo_surface_pattern_t,
                extents,
            );
        }
        4 | 5 => {}
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    1337 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 131],
                        &[libc::c_char; 131],
                    >(
                        b"cairo_xcb_picture_t *_cairo_xcb_picture_for_pattern(cairo_xcb_surface_t *, const cairo_pattern_t *, const cairo_rectangle_int_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    return _render_to_picture(target, pattern, extents);
}
unsafe extern "C" fn _render_fill_boxes(
    mut abstract_dst: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut color: *const cairo_color_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut dst: *mut cairo_xcb_surface_t = abstract_dst as *mut cairo_xcb_surface_t;
    let mut stack_xrects: [xcb_rectangle_t; 256] = [xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 256];
    let mut xrects: *mut xcb_rectangle_t = stack_xrects.as_mut_ptr();
    let mut render_color: xcb_render_color_t = xcb_render_color_t {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let mut render_op: libc::c_int = _render_operator(op);
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut max_count: libc::c_int = 0;
    render_color.red = (*color).red_short;
    render_color.green = (*color).green_short;
    render_color.blue = (*color).blue_short;
    render_color.alpha = (*color).alpha_short;
    max_count = 0 as libc::c_int;
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        if (*chunk).count > max_count {
            max_count = (*chunk).count;
        }
        chunk = (*chunk).next;
    }
    if max_count
        > (::std::mem::size_of::<[xcb_rectangle_t; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        xrects = _cairo_malloc_ab(
            max_count as size_t,
            ::std::mem::size_of::<xcb_rectangle_t>() as libc::c_ulong,
        ) as *mut xcb_rectangle_t;
        if xrects.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        i = j;
        while i < (*chunk).count {
            let mut x1: libc::c_int = _cairo_fixed_integer_round_down(
                (*((*chunk).base).offset(i as isize)).p1.x,
            );
            let mut y1: libc::c_int = _cairo_fixed_integer_round_down(
                (*((*chunk).base).offset(i as isize)).p1.y,
            );
            let mut x2: libc::c_int = _cairo_fixed_integer_round_down(
                (*((*chunk).base).offset(i as isize)).p2.x,
            );
            let mut y2: libc::c_int = _cairo_fixed_integer_round_down(
                (*((*chunk).base).offset(i as isize)).p2.y,
            );
            if x2 > x1 && y2 > y1 {
                (*xrects.offset(j as isize)).x = x1 as int16_t;
                (*xrects.offset(j as isize)).y = y1 as int16_t;
                (*xrects.offset(j as isize)).width = (x2 - x1) as uint16_t;
                (*xrects.offset(j as isize)).height = (y2 - y1) as uint16_t;
                j += 1;
            }
            i += 1;
        }
        if j != 0 {
            _cairo_xcb_connection_render_fill_rectangles(
                (*dst).connection,
                render_op as uint8_t,
                (*dst).picture,
                render_color,
                j as uint32_t,
                xrects,
            );
        }
        chunk = (*chunk).next;
    }
    if xrects != stack_xrects.as_mut_ptr() {
        free(xrects as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _render_composite_boxes(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut mask_pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut boxes: *const cairo_boxes_t,
) -> cairo_int_status_t {
    let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut mask: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut chunk: *const _cairo_boxes_chunk = 0 as *const _cairo_boxes_chunk;
    let mut stack_boxes: [xcb_rectangle_t; 256] = [xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 256];
    let mut clip_boxes: *mut xcb_rectangle_t = 0 as *mut xcb_rectangle_t;
    let mut stack_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut num_boxes: libc::c_int = 0;
    let mut render_op: libc::c_int = 0;
    render_op = _render_operator(op);
    if src_pattern.is_null() {
        src_pattern = mask_pattern;
        mask_pattern = 0 as *const cairo_pattern_t;
    }
    clip_boxes = stack_boxes.as_mut_ptr();
    if (*boxes).num_boxes
        > (::std::mem::size_of::<[xcb_rectangle_t; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<xcb_rectangle_t>() as libc::c_ulong)
            as libc::c_int
    {
        clip_boxes = _cairo_malloc_ab(
            (*boxes).num_boxes as size_t,
            ::std::mem::size_of::<xcb_rectangle_t>() as libc::c_ulong,
        ) as *mut xcb_rectangle_t;
        if clip_boxes.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    src = _cairo_xcb_picture_for_pattern(dst, src_pattern, extents);
    status = (*src).base.status;
    if !(status as u64 != 0) {
        num_boxes = 0 as libc::c_int;
        chunk = &(*boxes).chunks;
        while !chunk.is_null() {
            let mut box_0: *const cairo_box_t = (*chunk).base;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                let mut x: libc::c_int = _cairo_fixed_integer_round_down(
                    (*box_0.offset(i as isize)).p1.x,
                );
                let mut y: libc::c_int = _cairo_fixed_integer_round_down(
                    (*box_0.offset(i as isize)).p1.y,
                );
                let mut width: libc::c_int = _cairo_fixed_integer_round_down(
                    (*box_0.offset(i as isize)).p2.x,
                ) - x;
                let mut height: libc::c_int = _cairo_fixed_integer_round_down(
                    (*box_0.offset(i as isize)).p2.y,
                ) - y;
                if width != 0 && height != 0 {
                    (*clip_boxes.offset(num_boxes as isize)).x = x as int16_t;
                    (*clip_boxes.offset(num_boxes as isize)).y = y as int16_t;
                    (*clip_boxes.offset(num_boxes as isize)).width = width as uint16_t;
                    (*clip_boxes.offset(num_boxes as isize)).height = height as uint16_t;
                    num_boxes += 1;
                }
                i += 1;
            }
            chunk = (*chunk).next;
        }
        if num_boxes != 0 {
            if num_boxes > 1 as libc::c_int {
                _cairo_xcb_connection_render_set_picture_clip_rectangles(
                    (*dst).connection,
                    (*dst).picture,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    num_boxes as uint32_t,
                    clip_boxes,
                );
            } else {
                stack_extents
                    .x = (*clip_boxes.offset(0 as libc::c_int as isize)).x
                    as libc::c_int;
                stack_extents
                    .y = (*clip_boxes.offset(0 as libc::c_int as isize)).y
                    as libc::c_int;
                stack_extents
                    .width = (*clip_boxes.offset(0 as libc::c_int as isize)).width
                    as libc::c_int;
                stack_extents
                    .height = (*clip_boxes.offset(0 as libc::c_int as isize)).height
                    as libc::c_int;
                extents = &mut stack_extents;
            }
            if !mask_pattern.is_null() {
                mask = _cairo_xcb_picture_for_pattern(dst, mask_pattern, extents);
                status = (*mask).base.status;
                if !(status as u64 != 0) {
                    _cairo_xcb_connection_render_composite(
                        (*dst).connection,
                        render_op as uint8_t,
                        (*src).picture,
                        (*mask).picture,
                        (*dst).picture,
                        ((*src).x + (*extents).x) as int16_t,
                        ((*src).y + (*extents).y) as int16_t,
                        ((*mask).x + (*extents).x) as int16_t,
                        ((*mask).y + (*extents).y) as int16_t,
                        (*extents).x as int16_t,
                        (*extents).y as int16_t,
                        (*extents).width as uint16_t,
                        (*extents).height as uint16_t,
                    );
                    cairo_surface_destroy(&mut (*mask).base);
                }
            } else {
                _cairo_xcb_connection_render_composite(
                    (*dst).connection,
                    render_op as uint8_t,
                    (*src).picture,
                    0 as libc::c_long as xcb_render_picture_t,
                    (*dst).picture,
                    ((*src).x + (*extents).x) as int16_t,
                    ((*src).y + (*extents).y) as int16_t,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    (*extents).x as int16_t,
                    (*extents).y as int16_t,
                    (*extents).width as uint16_t,
                    (*extents).height as uint16_t,
                );
            }
            if num_boxes > 1 as libc::c_int {
                _cairo_xcb_surface_clear_clip_region(dst);
            }
        }
        cairo_surface_destroy(&mut (*src).base);
    }
    if clip_boxes != stack_boxes.as_mut_ptr() {
        free(clip_boxes as *mut libc::c_void);
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _line_exceeds_16_16(mut line: *const cairo_line_t) -> cairo_bool_t {
    return ((*line).p1.x <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p1.x >= _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p2.x <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p2.x >= _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p1.y <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p1.y >= _cairo_fixed_from_int(32767 as libc::c_int)
        || (*line).p2.y <= _cairo_fixed_from_int(-(32768 as libc::c_int))
        || (*line).p2.y >= _cairo_fixed_from_int(32767 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn _project_line_x_onto_16_16(
    mut line: *const cairo_line_t,
    mut top: cairo_fixed_t,
    mut bottom: cairo_fixed_t,
    mut out: *mut xcb_render_linefix_t,
) {
    let mut p1: cairo_point_double_t = cairo_point_double_t {
        x: 0.,
        y: 0.,
    };
    let mut p2: cairo_point_double_t = cairo_point_double_t {
        x: 0.,
        y: 0.,
    };
    let mut m: libc::c_double = 0.;
    p1.x = _cairo_fixed_to_double((*line).p1.x);
    p1.y = _cairo_fixed_to_double((*line).p1.y);
    p2.x = _cairo_fixed_to_double((*line).p2.x);
    p2.y = _cairo_fixed_to_double((*line).p2.y);
    m = (p2.x - p1.x) / (p2.y - p1.y);
    (*out)
        .p1
        .x = _cairo_fixed_16_16_from_double(
        p1.x + m * _cairo_fixed_to_double(top - (*line).p1.y),
    );
    (*out)
        .p2
        .x = _cairo_fixed_16_16_from_double(
        p1.x + m * _cairo_fixed_to_double(bottom - (*line).p1.y),
    );
}
unsafe extern "C" fn _composite_traps(
    mut closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut info: *mut composite_traps_info_t = closure as *mut composite_traps_info_t;
    let mut traps: *const cairo_traps_t = &mut (*info).traps;
    let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
    let mut xrender_format: xcb_render_pictformat_t = 0;
    let mut xtraps: *mut xcb_render_trapezoid_t = 0 as *mut xcb_render_trapezoid_t;
    let mut render_reference_x: libc::c_int = 0;
    let mut render_reference_y: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if (*dst).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(dst);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    src = _cairo_xcb_picture_for_pattern(dst, pattern, extents);
    if (*src).base.status as u64 != 0 {
        return (*src).base.status as cairo_int_status_t;
    }
    if (*info).antialias as libc::c_uint
        == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint
    {
        format = CAIRO_FORMAT_A1;
    } else {
        format = CAIRO_FORMAT_A8;
    }
    xrender_format = (*(*(*dst).screen).connection).standard_formats[format as usize];
    xtraps = (*traps).traps as *mut xcb_render_trapezoid_t;
    i = 0 as libc::c_int;
    while i < (*traps).num_traps {
        let mut t: cairo_trapezoid_t = *((*traps).traps).offset(i as isize);
        (*xtraps.offset(i as isize)).top = _cairo_fixed_to_16_16(t.top);
        let ref mut fresh22 = (*xtraps.offset(i as isize)).top;
        *fresh22 -= dst_y << 16 as libc::c_int;
        (*xtraps.offset(i as isize)).bottom = _cairo_fixed_to_16_16(t.bottom);
        let ref mut fresh23 = (*xtraps.offset(i as isize)).bottom;
        *fresh23 -= dst_y << 16 as libc::c_int;
        if _line_exceeds_16_16(&mut t.left) != 0 {
            _project_line_x_onto_16_16(
                &mut t.left,
                t.top,
                t.bottom,
                &mut (*xtraps.offset(i as isize)).left,
            );
            (*xtraps.offset(i as isize)).left.p1.y = (*xtraps.offset(i as isize)).top;
            (*xtraps.offset(i as isize)).left.p2.y = (*xtraps.offset(i as isize)).bottom;
        } else {
            (*xtraps.offset(i as isize)).left.p1.x = _cairo_fixed_to_16_16(t.left.p1.x);
            (*xtraps.offset(i as isize)).left.p1.y = _cairo_fixed_to_16_16(t.left.p1.y);
            (*xtraps.offset(i as isize)).left.p2.x = _cairo_fixed_to_16_16(t.left.p2.x);
            (*xtraps.offset(i as isize)).left.p2.y = _cairo_fixed_to_16_16(t.left.p2.y);
        }
        let ref mut fresh24 = (*xtraps.offset(i as isize)).left.p1.x;
        *fresh24 -= dst_x << 16 as libc::c_int;
        let ref mut fresh25 = (*xtraps.offset(i as isize)).left.p1.y;
        *fresh25 -= dst_y << 16 as libc::c_int;
        let ref mut fresh26 = (*xtraps.offset(i as isize)).left.p2.x;
        *fresh26 -= dst_x << 16 as libc::c_int;
        let ref mut fresh27 = (*xtraps.offset(i as isize)).left.p2.y;
        *fresh27 -= dst_y << 16 as libc::c_int;
        if _line_exceeds_16_16(&mut t.right) != 0 {
            _project_line_x_onto_16_16(
                &mut t.right,
                t.top,
                t.bottom,
                &mut (*xtraps.offset(i as isize)).right,
            );
            (*xtraps.offset(i as isize)).right.p1.y = (*xtraps.offset(i as isize)).top;
            (*xtraps.offset(i as isize))
                .right
                .p2
                .y = (*xtraps.offset(i as isize)).bottom;
        } else {
            (*xtraps.offset(i as isize))
                .right
                .p1
                .x = _cairo_fixed_to_16_16(t.right.p1.x);
            (*xtraps.offset(i as isize))
                .right
                .p1
                .y = _cairo_fixed_to_16_16(t.right.p1.y);
            (*xtraps.offset(i as isize))
                .right
                .p2
                .x = _cairo_fixed_to_16_16(t.right.p2.x);
            (*xtraps.offset(i as isize))
                .right
                .p2
                .y = _cairo_fixed_to_16_16(t.right.p2.y);
        }
        let ref mut fresh28 = (*xtraps.offset(i as isize)).right.p1.x;
        *fresh28 -= dst_x << 16 as libc::c_int;
        let ref mut fresh29 = (*xtraps.offset(i as isize)).right.p1.y;
        *fresh29 -= dst_y << 16 as libc::c_int;
        let ref mut fresh30 = (*xtraps.offset(i as isize)).right.p2.x;
        *fresh30 -= dst_x << 16 as libc::c_int;
        let ref mut fresh31 = (*xtraps.offset(i as isize)).right.p2.y;
        *fresh31 -= dst_y << 16 as libc::c_int;
        i += 1;
    }
    if (*xtraps.offset(0 as libc::c_int as isize)).left.p1.y
        < (*xtraps.offset(0 as libc::c_int as isize)).left.p2.y
    {
        render_reference_x = (*xtraps.offset(0 as libc::c_int as isize)).left.p1.x
            >> 16 as libc::c_int;
        render_reference_y = (*xtraps.offset(0 as libc::c_int as isize)).left.p1.y
            >> 16 as libc::c_int;
    } else {
        render_reference_x = (*xtraps.offset(0 as libc::c_int as isize)).left.p2.x
            >> 16 as libc::c_int;
        render_reference_y = (*xtraps.offset(0 as libc::c_int as isize)).left.p2.y
            >> 16 as libc::c_int;
    }
    render_reference_x += (*src).x + dst_x;
    render_reference_y += (*src).y + dst_y;
    _cairo_xcb_surface_set_precision(dst, (*info).antialias);
    _cairo_xcb_connection_render_trapezoids(
        (*dst).connection,
        _render_operator(op) as uint8_t,
        (*src).picture,
        (*dst).picture,
        xrender_format,
        render_reference_x as int16_t,
        render_reference_y as int16_t,
        (*traps).num_traps as uint32_t,
        xtraps,
    );
    cairo_surface_destroy(&mut (*src).base);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn get_clip_surface(
    mut clip: *const cairo_clip_t,
    mut target: *mut cairo_xcb_surface_t,
    mut tx: *mut libc::c_int,
    mut ty: *mut libc::c_int,
) -> *mut cairo_xcb_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    surface = _cairo_surface_create_scratch(
        &mut (*target).base,
        CAIRO_CONTENT_ALPHA,
        (*clip).extents.width,
        (*clip).extents.height,
        _cairo_stock_color(CAIRO_STOCK_WHITE),
    );
    if (*surface).status as u64 != 0 {
        return surface as *mut cairo_xcb_surface_t;
    }
    if (*surface).backend
        == &_cairo_xcb_surface_backend as *const cairo_surface_backend_t
    {} else {
        __assert_fail(
            b"surface->backend == &_cairo_xcb_surface_backend\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            1704 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_xcb_surface_t *get_clip_surface(const cairo_clip_t *, cairo_xcb_surface_t *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_clip_combine_with_surface(
        clip,
        surface,
        (*clip).extents.x,
        (*clip).extents.y,
    );
    if status as u64 != 0 {
        cairo_surface_destroy(surface);
        surface = _cairo_surface_create_in_error(status);
    }
    *tx = (*clip).extents.x;
    *ty = (*clip).extents.y;
    return surface as *mut cairo_xcb_surface_t;
}
unsafe extern "C" fn do_unaligned_row(
    mut blt: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            int16_t,
            int16_t,
            int16_t,
            int16_t,
            uint16_t,
        ) -> (),
    >,
    mut closure: *mut libc::c_void,
    mut b: *const cairo_box_t,
    mut tx: libc::c_int,
    mut y: libc::c_int,
    mut h: libc::c_int,
    mut coverage: uint16_t,
) {
    let mut x1: libc::c_int = _cairo_fixed_integer_part((*b).p1.x) - tx;
    let mut x2: libc::c_int = _cairo_fixed_integer_part((*b).p2.x) - tx;
    if x2 > x1 {
        if _cairo_fixed_is_integer((*b).p1.x) == 0 {
            blt
                .expect(
                    "non-null function pointer",
                )(
                closure,
                x1 as int16_t,
                y as int16_t,
                1 as libc::c_int as int16_t,
                h as int16_t,
                (coverage as libc::c_int
                    * (256 as libc::c_int - _cairo_fixed_fractional_part((*b).p1.x)))
                    as uint16_t,
            );
            x1 += 1;
        }
        if x2 > x1 {
            blt
                .expect(
                    "non-null function pointer",
                )(
                closure,
                x1 as int16_t,
                y as int16_t,
                (x2 - x1) as int16_t,
                h as int16_t,
                (((coverage as libc::c_int) << 8 as libc::c_int)
                    - (coverage as libc::c_int >> 8 as libc::c_int)) as uint16_t,
            );
        }
        if _cairo_fixed_is_integer((*b).p2.x) == 0 {
            blt
                .expect(
                    "non-null function pointer",
                )(
                closure,
                x2 as int16_t,
                y as int16_t,
                1 as libc::c_int as int16_t,
                h as int16_t,
                (coverage as libc::c_int * _cairo_fixed_fractional_part((*b).p2.x))
                    as uint16_t,
            );
        }
    } else {
        blt
            .expect(
                "non-null function pointer",
            )(
            closure,
            x1 as int16_t,
            y as int16_t,
            1 as libc::c_int as int16_t,
            h as int16_t,
            (coverage as libc::c_int * ((*b).p2.x - (*b).p1.x)) as uint16_t,
        );
    };
}
unsafe extern "C" fn do_unaligned_box(
    mut blt: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            int16_t,
            int16_t,
            int16_t,
            int16_t,
            uint16_t,
        ) -> (),
    >,
    mut closure: *mut libc::c_void,
    mut b: *const cairo_box_t,
    mut tx: libc::c_int,
    mut ty: libc::c_int,
) {
    let mut y1: libc::c_int = _cairo_fixed_integer_part((*b).p1.y) - ty;
    let mut y2: libc::c_int = _cairo_fixed_integer_part((*b).p2.y) - ty;
    if y2 > y1 {
        if _cairo_fixed_is_integer((*b).p1.y) == 0 {
            do_unaligned_row(
                blt,
                closure,
                b,
                tx,
                y1,
                1 as libc::c_int,
                (256 as libc::c_int - _cairo_fixed_fractional_part((*b).p1.y))
                    as uint16_t,
            );
            y1 += 1;
        }
        if y2 > y1 {
            do_unaligned_row(
                blt,
                closure,
                b,
                tx,
                y1,
                y2 - y1,
                256 as libc::c_int as uint16_t,
            );
        }
        if _cairo_fixed_is_integer((*b).p2.y) == 0 {
            do_unaligned_row(
                blt,
                closure,
                b,
                tx,
                y2,
                1 as libc::c_int,
                _cairo_fixed_fractional_part((*b).p2.y) as uint16_t,
            );
        }
    } else {
        do_unaligned_row(
            blt,
            closure,
            b,
            tx,
            y1,
            1 as libc::c_int,
            ((*b).p2.y - (*b).p1.y) as uint16_t,
        );
    };
}
unsafe extern "C" fn blt_in(
    mut closure: *mut libc::c_void,
    mut x: int16_t,
    mut y: int16_t,
    mut w: int16_t,
    mut h: int16_t,
    mut coverage: uint16_t,
) {
    let mut mask: *mut cairo_xcb_surface_t = closure as *mut cairo_xcb_surface_t;
    let mut color: xcb_render_color_t = xcb_render_color_t {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let mut rect: xcb_rectangle_t = xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if coverage as libc::c_int == 0xffff as libc::c_int {
        return;
    }
    color.blue = 0 as libc::c_int as uint16_t;
    color.green = color.blue;
    color.red = color.green;
    color.alpha = coverage;
    rect.x = x;
    rect.y = y;
    rect.width = w as uint16_t;
    rect.height = h as uint16_t;
    _cairo_xcb_connection_render_fill_rectangles(
        (*mask).connection,
        XCB_RENDER_PICT_OP_IN as libc::c_int as uint8_t,
        (*mask).picture,
        color,
        1 as libc::c_int as uint32_t,
        &mut rect,
    );
}
unsafe extern "C" fn _create_composite_mask(
    mut clip: *mut cairo_clip_t,
    mut draw_func: xcb_draw_func_t,
    mut mask_func: xcb_draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_xcb_surface_t {
    let mut surface: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut need_clip_combine: cairo_bool_t = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    surface = _cairo_xcb_surface_create_similar(
        dst as *mut libc::c_void,
        CAIRO_CONTENT_ALPHA,
        (*extents).width,
        (*extents).height,
    ) as *mut cairo_xcb_surface_t;
    if (*surface).base.status as u64 != 0 {
        return surface;
    }
    _cairo_xcb_surface_ensure_picture(surface);
    (*surface).deferred_clear_color = *_cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
    (*surface).deferred_clear = 1 as libc::c_int;
    let ref mut fresh32 = (*surface).base;
    (*fresh32).set_is_clear(1 as libc::c_int as libc::c_uint);
    if mask_func.is_some() {
        status = mask_func
            .expect(
                "non-null function pointer",
            )(
            draw_closure,
            surface,
            CAIRO_OPERATOR_ADD,
            0 as *const cairo_pattern_t,
            (*extents).x,
            (*extents).y,
            extents,
            clip,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return surface;
        }
    }
    status = draw_func
        .expect(
            "non-null function pointer",
        )(
        draw_closure,
        surface,
        CAIRO_OPERATOR_ADD,
        0 as *const cairo_pattern_t,
        (*extents).x,
        (*extents).y,
        extents,
        0 as *mut cairo_clip_t,
    );
    if status as u64 != 0 {
        cairo_surface_destroy(&mut (*surface).base);
        return _cairo_surface_create_in_error(status as cairo_status_t)
            as *mut cairo_xcb_surface_t;
    }
    if (*(*surface).connection).flags
        & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
    {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*clip).num_boxes {
            let mut b: *mut cairo_box_t = &mut *((*clip).boxes).offset(i as isize)
                as *mut cairo_box_t;
            if _cairo_fixed_is_integer((*b).p1.x) == 0
                || _cairo_fixed_is_integer((*b).p1.y) == 0
                || _cairo_fixed_is_integer((*b).p2.x) == 0
                || _cairo_fixed_is_integer((*b).p2.y) == 0
            {
                do_unaligned_box(
                    Some(
                        blt_in
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                int16_t,
                                int16_t,
                                int16_t,
                                int16_t,
                                uint16_t,
                            ) -> (),
                    ),
                    surface as *mut libc::c_void,
                    b,
                    (*extents).x,
                    (*extents).y,
                );
            }
            i += 1;
        }
        need_clip_combine = ((*clip).path
            != 0 as *mut libc::c_void as *mut cairo_clip_path_t) as libc::c_int;
    } else {
        need_clip_combine = (_cairo_clip_is_region(clip) == 0) as libc::c_int;
    }
    if need_clip_combine != 0 {
        status = _cairo_clip_combine_with_surface(
            clip,
            &mut (*surface).base,
            (*extents).x,
            (*extents).y,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            cairo_surface_destroy(&mut (*surface).base);
            return _cairo_surface_create_in_error(status as cairo_status_t)
                as *mut cairo_xcb_surface_t;
        }
    }
    return surface;
}
unsafe extern "C" fn _clip_and_composite_with_mask(
    mut clip: *mut cairo_clip_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut draw_func: xcb_draw_func_t,
    mut mask_func: xcb_draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut mask: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    mask = _create_composite_mask(
        clip,
        draw_func,
        mask_func,
        draw_closure,
        dst,
        extents,
    );
    if (*mask).base.status as u64 != 0 {
        return (*mask).base.status;
    }
    if !pattern.is_null()
        || (*dst).base.content as libc::c_uint
            != CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
    {
        src = _cairo_xcb_picture_for_pattern(dst, pattern, extents);
        if (*src).base.status as u64 != 0 {
            cairo_surface_destroy(&mut (*mask).base);
            return (*src).base.status;
        }
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            _render_operator(op) as uint8_t,
            (*src).picture,
            (*mask).picture,
            (*dst).picture,
            ((*extents).x + (*src).x) as int16_t,
            ((*extents).y + (*src).y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*extents).x as int16_t,
            (*extents).y as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
        cairo_surface_destroy(&mut (*src).base);
    } else {
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            _render_operator(op) as uint8_t,
            (*mask).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*extents).x as int16_t,
            (*extents).y as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
    }
    cairo_surface_destroy(&mut (*mask).base);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _clip_and_composite_combine(
    mut clip: *mut cairo_clip_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut draw_func: xcb_draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut tmp: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut clip_surface: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut clip_x: libc::c_int = 0 as libc::c_int;
    let mut clip_y: libc::c_int = 0 as libc::c_int;
    let mut clip_picture: xcb_render_picture_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    tmp = _cairo_xcb_surface_create_similar(
        dst as *mut libc::c_void,
        (*dst).base.content,
        (*extents).width,
        (*extents).height,
    ) as *mut cairo_xcb_surface_t;
    if (*tmp).base.status as u64 != 0 {
        return (*tmp).base.status;
    }
    if (*tmp).base.backend
        == &_cairo_xcb_surface_backend as *const cairo_surface_backend_t
    {} else {
        __assert_fail(
            b"tmp->base.backend == &_cairo_xcb_surface_backend\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            1966 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 181],
                &[libc::c_char; 181],
            >(
                b"cairo_status_t _clip_and_composite_combine(cairo_clip_t *, cairo_operator_t, const cairo_pattern_t *, xcb_draw_func_t, void *, cairo_xcb_surface_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_xcb_surface_ensure_picture(tmp);
    if pattern.is_null() {
        status = (Some(draw_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            draw_closure,
            tmp,
            CAIRO_OPERATOR_ADD,
            0 as *const cairo_pattern_t,
            (*extents).x,
            (*extents).y,
            extents,
            0 as *mut cairo_clip_t,
        ) as cairo_status_t;
    } else {
        if ((*dst).base).is_clear() == 0
            || (*(*dst).connection).flags
                & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            _cairo_xcb_connection_render_composite(
                (*dst).connection,
                XCB_RENDER_PICT_OP_SRC as libc::c_int as uint8_t,
                (*dst).picture,
                0 as libc::c_long as xcb_render_picture_t,
                (*tmp).picture,
                (*extents).x as int16_t,
                (*extents).y as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                (*extents).width as uint16_t,
                (*extents).height as uint16_t,
            );
        } else {
            let mut clear: xcb_render_color_t = xcb_render_color_t {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            };
            let mut xrect: xcb_rectangle_t = xcb_rectangle_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            clear.alpha = 0 as libc::c_int as uint16_t;
            clear.blue = clear.alpha;
            clear.green = clear.blue;
            clear.red = clear.green;
            xrect.y = 0 as libc::c_int as int16_t;
            xrect.x = xrect.y;
            xrect.width = (*extents).width as uint16_t;
            xrect.height = (*extents).height as uint16_t;
            _cairo_xcb_connection_render_fill_rectangles(
                (*dst).connection,
                XCB_RENDER_PICT_OP_CLEAR as libc::c_int as uint8_t,
                (*dst).picture,
                clear,
                1 as libc::c_int as uint32_t,
                &mut xrect,
            );
        }
        status = (Some(draw_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            draw_closure,
            tmp,
            op,
            pattern,
            (*extents).x,
            (*extents).y,
            extents,
            0 as *mut cairo_clip_t,
        ) as cairo_status_t;
    }
    if !(status as u64 != 0) {
        clip_surface = get_clip_surface(clip, dst, &mut clip_x, &mut clip_y);
        status = (*clip_surface).base.status;
        if !(status as u64 != 0) {
            if (*clip_surface).base.backend
                == &_cairo_xcb_surface_backend as *const cairo_surface_backend_t
            {} else {
                __assert_fail(
                    b"clip_surface->base.backend == &_cairo_xcb_surface_backend\0"
                        as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    2022 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 181],
                        &[libc::c_char; 181],
                    >(
                        b"cairo_status_t _clip_and_composite_combine(cairo_clip_t *, cairo_operator_t, const cairo_pattern_t *, xcb_draw_func_t, void *, cairo_xcb_surface_t *, const cairo_rectangle_int_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            clip_picture = (*clip_surface).picture;
            if clip_picture as libc::c_long != 0 as libc::c_long {} else {
                __assert_fail(
                    b"clip_picture != XCB_NONE\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-xcb-surface-render.c\0" as *const u8
                        as *const libc::c_char,
                    2024 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 181],
                        &[libc::c_char; 181],
                    >(
                        b"cairo_status_t _clip_and_composite_combine(cairo_clip_t *, cairo_operator_t, const cairo_pattern_t *, xcb_draw_func_t, void *, cairo_xcb_surface_t *, const cairo_rectangle_int_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ((*dst).base).is_clear() != 0 {
                _cairo_xcb_connection_render_composite(
                    (*dst).connection,
                    XCB_RENDER_PICT_OP_SRC as libc::c_int as uint8_t,
                    (*tmp).picture,
                    clip_picture,
                    (*dst).picture,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    (*extents).x as int16_t,
                    (*extents).y as int16_t,
                    (*extents).width as uint16_t,
                    (*extents).height as uint16_t,
                );
            } else {
                _cairo_xcb_connection_render_composite(
                    (*dst).connection,
                    XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int as uint8_t,
                    clip_picture,
                    0 as libc::c_long as xcb_render_picture_t,
                    (*dst).picture,
                    ((*extents).x - clip_x) as int16_t,
                    ((*extents).y - clip_y) as int16_t,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    (*extents).x as int16_t,
                    (*extents).y as int16_t,
                    (*extents).width as uint16_t,
                    (*extents).height as uint16_t,
                );
                _cairo_xcb_connection_render_composite(
                    (*dst).connection,
                    XCB_RENDER_PICT_OP_ADD as libc::c_int as uint8_t,
                    (*tmp).picture,
                    clip_picture,
                    (*dst).picture,
                    0 as libc::c_int as int16_t,
                    0 as libc::c_int as int16_t,
                    ((*extents).x - clip_x) as int16_t,
                    ((*extents).y - clip_y) as int16_t,
                    (*extents).x as int16_t,
                    (*extents).y as int16_t,
                    (*extents).width as uint16_t,
                    (*extents).height as uint16_t,
                );
            }
            cairo_surface_destroy(&mut (*clip_surface).base);
        }
    }
    cairo_surface_destroy(&mut (*tmp).base);
    return status;
}
unsafe extern "C" fn _clip_and_composite_source(
    mut clip: *mut cairo_clip_t,
    mut pattern: *const cairo_pattern_t,
    mut draw_func: xcb_draw_func_t,
    mut mask_func: xcb_draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut mask: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    mask = _create_composite_mask(
        clip,
        draw_func,
        mask_func,
        draw_closure,
        dst,
        extents,
    );
    if (*mask).base.status as u64 != 0 {
        return (*mask).base.status;
    }
    src = _cairo_xcb_picture_for_pattern(dst, pattern, extents);
    if (*src).base.status as u64 != 0 {
        cairo_surface_destroy(&mut (*mask).base);
        return (*src).base.status;
    }
    if ((*dst).base).is_clear() != 0 {
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_SRC as libc::c_int as uint8_t,
            (*src).picture,
            (*mask).picture,
            (*dst).picture,
            ((*extents).x + (*src).x) as int16_t,
            ((*extents).y + (*src).y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*extents).x as int16_t,
            (*extents).y as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
    } else {
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int as uint8_t,
            (*mask).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*extents).x as int16_t,
            (*extents).y as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_ADD as libc::c_int as uint8_t,
            (*src).picture,
            (*mask).picture,
            (*dst).picture,
            ((*extents).x + (*src).x) as int16_t,
            ((*extents).y + (*src).y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            (*extents).x as int16_t,
            (*extents).y as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
    }
    cairo_surface_destroy(&mut (*src).base);
    cairo_surface_destroy(&mut (*mask).base);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn can_reduce_alpha_op(mut op: cairo_operator_t) -> cairo_bool_t {
    let mut iop: libc::c_int = op as libc::c_int;
    match iop {
        2 | 1 | 12 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn reduce_alpha_op(
    mut dst: *mut cairo_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    return ((*dst).is_clear() as libc::c_int != 0
        && (*dst).content as libc::c_uint
            == CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
        && _cairo_pattern_is_opaque_solid(pattern) != 0 && can_reduce_alpha_op(op) != 0)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_xcb_surface_fixup_unbounded(
    mut dst: *mut cairo_xcb_surface_t,
    mut rects: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut xrects: [xcb_rectangle_t; 4] = [xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 4];
    let mut n: libc::c_int = 0;
    if (*rects).bounded.width == (*rects).unbounded.width
        && (*rects).bounded.height == (*rects).unbounded.height
    {
        return CAIRO_STATUS_SUCCESS;
    }
    n = 0 as libc::c_int;
    if (*rects).bounded.width == 0 as libc::c_int
        || (*rects).bounded.height == 0 as libc::c_int
    {
        xrects[n as usize].x = (*rects).unbounded.x as int16_t;
        xrects[n as usize].width = (*rects).unbounded.width as uint16_t;
        xrects[n as usize].y = (*rects).unbounded.y as int16_t;
        xrects[n as usize].height = (*rects).unbounded.height as uint16_t;
        n += 1;
    } else {
        if (*rects).bounded.y != (*rects).unbounded.y {
            xrects[n as usize].x = (*rects).unbounded.x as int16_t;
            xrects[n as usize].width = (*rects).unbounded.width as uint16_t;
            xrects[n as usize].y = (*rects).unbounded.y as int16_t;
            xrects[n as usize]
                .height = ((*rects).bounded.y - (*rects).unbounded.y) as uint16_t;
            n += 1;
        }
        if (*rects).bounded.x != (*rects).unbounded.x {
            xrects[n as usize].x = (*rects).unbounded.x as int16_t;
            xrects[n as usize]
                .width = ((*rects).bounded.x - (*rects).unbounded.x) as uint16_t;
            xrects[n as usize].y = (*rects).bounded.y as int16_t;
            xrects[n as usize].height = (*rects).bounded.height as uint16_t;
            n += 1;
        }
        if (*rects).bounded.x + (*rects).bounded.width
            != (*rects).unbounded.x + (*rects).unbounded.width
        {
            xrects[n as usize]
                .x = ((*rects).bounded.x + (*rects).bounded.width) as int16_t;
            xrects[n as usize]
                .width = ((*rects).unbounded.x + (*rects).unbounded.width
                - xrects[n as usize].x as libc::c_int) as uint16_t;
            xrects[n as usize].y = (*rects).bounded.y as int16_t;
            xrects[n as usize].height = (*rects).bounded.height as uint16_t;
            n += 1;
        }
        if (*rects).bounded.y + (*rects).bounded.height
            != (*rects).unbounded.y + (*rects).unbounded.height
        {
            xrects[n as usize].x = (*rects).unbounded.x as int16_t;
            xrects[n as usize].width = (*rects).unbounded.width as uint16_t;
            xrects[n as usize]
                .y = ((*rects).bounded.y + (*rects).bounded.height) as int16_t;
            xrects[n as usize]
                .height = ((*rects).unbounded.y + (*rects).unbounded.height
                - xrects[n as usize].y as libc::c_int) as uint16_t;
            n += 1;
        }
    }
    if (*(*dst).connection).flags
        & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
    {
        let mut color: xcb_render_color_t = xcb_render_color_t {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        color.red = 0 as libc::c_int as uint16_t;
        color.green = 0 as libc::c_int as uint16_t;
        color.blue = 0 as libc::c_int as uint16_t;
        color.alpha = 0 as libc::c_int as uint16_t;
        _cairo_xcb_connection_render_fill_rectangles(
            (*dst).connection,
            XCB_RENDER_PICT_OP_CLEAR as libc::c_int as uint8_t,
            (*dst).picture,
            color,
            n as uint32_t,
            xrects.as_mut_ptr(),
        );
    } else {
        let mut i: libc::c_int = 0;
        let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
        src = _cairo_xcb_transparent_picture(dst);
        if (*src).base.status as u64 != 0 {
            return (*src).base.status;
        }
        i = 0 as libc::c_int;
        while i < n {
            _cairo_xcb_connection_render_composite(
                (*dst).connection,
                XCB_RENDER_PICT_OP_CLEAR as libc::c_int as uint8_t,
                (*src).picture,
                0 as libc::c_long as xcb_render_picture_t,
                (*dst).picture,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                xrects[i as usize].x,
                xrects[i as usize].y,
                xrects[i as usize].width,
                xrects[i as usize].height,
            );
            i += 1;
        }
        cairo_surface_destroy(&mut (*src).base);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xcb_surface_fixup_unbounded_with_mask(
    mut dst: *mut cairo_xcb_surface_t,
    mut rects: *const cairo_composite_rectangles_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_status_t {
    let mut mask: *mut cairo_xcb_surface_t = 0 as *mut cairo_xcb_surface_t;
    let mut mask_x: libc::c_int = 0 as libc::c_int;
    let mut mask_y: libc::c_int = 0 as libc::c_int;
    mask = get_clip_surface(clip, dst, &mut mask_x, &mut mask_y);
    if (*mask).base.status as u64 != 0 {
        return (*mask).base.status;
    }
    if (*rects).bounded.y != (*rects).unbounded.y {
        let mut x: libc::c_int = (*rects).unbounded.x;
        let mut y: libc::c_int = (*rects).unbounded.y;
        let mut width: libc::c_int = (*rects).unbounded.width;
        let mut height: libc::c_int = (*rects).bounded.y - y;
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int as uint8_t,
            (*mask).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            (x - mask_x) as int16_t,
            (y - mask_y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            x as int16_t,
            y as int16_t,
            width as uint16_t,
            height as uint16_t,
        );
    }
    if (*rects).bounded.x != (*rects).unbounded.x {
        let mut x_0: libc::c_int = (*rects).unbounded.x;
        let mut y_0: libc::c_int = (*rects).bounded.y;
        let mut width_0: libc::c_int = (*rects).bounded.x - x_0;
        let mut height_0: libc::c_int = (*rects).bounded.height;
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int as uint8_t,
            (*mask).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            (x_0 - mask_x) as int16_t,
            (y_0 - mask_y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            x_0 as int16_t,
            y_0 as int16_t,
            width_0 as uint16_t,
            height_0 as uint16_t,
        );
    }
    if (*rects).bounded.x + (*rects).bounded.width
        != (*rects).unbounded.x + (*rects).unbounded.width
    {
        let mut x_1: libc::c_int = (*rects).bounded.x + (*rects).bounded.width;
        let mut y_1: libc::c_int = (*rects).bounded.y;
        let mut width_1: libc::c_int = (*rects).unbounded.x + (*rects).unbounded.width
            - x_1;
        let mut height_1: libc::c_int = (*rects).bounded.height;
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int as uint8_t,
            (*mask).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            (x_1 - mask_x) as int16_t,
            (y_1 - mask_y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            x_1 as int16_t,
            y_1 as int16_t,
            width_1 as uint16_t,
            height_1 as uint16_t,
        );
    }
    if (*rects).bounded.y + (*rects).bounded.height
        != (*rects).unbounded.y + (*rects).unbounded.height
    {
        let mut x_2: libc::c_int = (*rects).unbounded.x;
        let mut y_2: libc::c_int = (*rects).bounded.y + (*rects).bounded.height;
        let mut width_2: libc::c_int = (*rects).unbounded.width;
        let mut height_2: libc::c_int = (*rects).unbounded.y + (*rects).unbounded.height
            - y_2;
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            XCB_RENDER_PICT_OP_OUT_REVERSE as libc::c_int as uint8_t,
            (*mask).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            (x_2 - mask_x) as int16_t,
            (y_2 - mask_y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            x_2 as int16_t,
            y_2 as int16_t,
            width_2 as uint16_t,
            height_2 as uint16_t,
        );
    }
    cairo_surface_destroy(&mut (*mask).base);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xcb_surface_fixup_unbounded_boxes(
    mut dst: *mut cairo_xcb_surface_t,
    mut extents: *const cairo_composite_rectangles_t,
    mut clip: *mut cairo_clip_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut clear: cairo_boxes_t = cairo_boxes_t {
        status: CAIRO_STATUS_SUCCESS,
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_boxes: 0,
        is_pixel_aligned: 0,
        chunks: _cairo_boxes_chunk {
            next: 0 as *mut _cairo_boxes_chunk,
            base: 0 as *mut cairo_box_t,
            count: 0,
            size: 0,
        },
        tail: 0 as *mut _cairo_boxes_chunk,
        boxes_embedded: [cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        }; 32],
    };
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    if (*boxes).num_boxes <= 1 as libc::c_int && clip.is_null() {
        return _cairo_xcb_surface_fixup_unbounded(dst, extents);
    }
    _cairo_boxes_init(&mut clear);
    box_0
        .p1
        .x = _cairo_fixed_from_int((*extents).unbounded.x + (*extents).unbounded.width);
    box_0.p1.y = _cairo_fixed_from_int((*extents).unbounded.y);
    box_0.p2.x = _cairo_fixed_from_int((*extents).unbounded.x);
    box_0
        .p2
        .y = _cairo_fixed_from_int((*extents).unbounded.y + (*extents).unbounded.height);
    if clip.is_null() {
        let mut tmp: cairo_boxes_t = cairo_boxes_t {
            status: CAIRO_STATUS_SUCCESS,
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_boxes: 0,
            is_pixel_aligned: 0,
            chunks: _cairo_boxes_chunk {
                next: 0 as *mut _cairo_boxes_chunk,
                base: 0 as *mut cairo_box_t,
                count: 0,
                size: 0,
            },
            tail: 0 as *mut _cairo_boxes_chunk,
            boxes_embedded: [cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            }; 32],
        };
        _cairo_boxes_init(&mut tmp);
        status = _cairo_boxes_add(&mut tmp, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                2354 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 150],
                    &[libc::c_char; 150],
                >(
                    b"cairo_status_t _cairo_xcb_surface_fixup_unbounded_boxes(cairo_xcb_surface_t *, const cairo_composite_rectangles_t *, cairo_clip_t *, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        tmp.chunks.next = &mut (*boxes).chunks;
        tmp.num_boxes += (*boxes).num_boxes;
        status = _cairo_bentley_ottmann_tessellate_boxes(
            &mut tmp,
            CAIRO_FILL_RULE_WINDING,
            &mut clear,
        );
        tmp.chunks.next = 0 as *mut _cairo_boxes_chunk;
    } else {
        _cairo_boxes_init_with_clip(&mut clear, clip);
        status = _cairo_boxes_add(&mut clear, CAIRO_ANTIALIAS_DEFAULT, &mut box_0);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                2368 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 150],
                    &[libc::c_char; 150],
                >(
                    b"cairo_status_t _cairo_xcb_surface_fixup_unbounded_boxes(cairo_xcb_surface_t *, const cairo_composite_rectangles_t *, cairo_clip_t *, cairo_boxes_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        chunk = &mut (*boxes).chunks;
        while !chunk.is_null() {
            i = 0 as libc::c_int;
            while i < (*chunk).count {
                status = _cairo_boxes_add(
                    &mut clear,
                    CAIRO_ANTIALIAS_DEFAULT,
                    &mut *((*chunk).base).offset(i as isize),
                );
                if status as u64 != 0 {
                    _cairo_boxes_fini(&mut clear);
                    return status;
                }
                i += 1;
            }
            chunk = (*chunk).next;
        }
        status = _cairo_bentley_ottmann_tessellate_boxes(
            &mut clear,
            CAIRO_FILL_RULE_WINDING,
            &mut clear,
        );
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        if (*(*dst).connection).flags
            & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
        {
            status = _render_fill_boxes(
                dst as *mut libc::c_void,
                CAIRO_OPERATOR_CLEAR,
                _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
                &mut clear,
            );
        } else {
            status = _cairo_xcb_surface_core_fill_boxes(
                dst,
                _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
                &mut clear,
            );
        }
    }
    _cairo_boxes_fini(&mut clear);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_surface_clear(
    mut dst: *mut cairo_xcb_surface_t,
) -> cairo_status_t {
    let mut gc: xcb_gcontext_t = 0;
    let mut rect: xcb_rectangle_t = xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_xcb_connection_acquire((*dst).connection);
    if status as u64 != 0 {
        return status;
    }
    rect.y = 0 as libc::c_int as int16_t;
    rect.x = rect.y;
    rect.width = (*dst).width as uint16_t;
    rect.height = (*dst).height as uint16_t;
    if (*(*dst).connection).flags
        & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
    {
        let mut color: xcb_render_color_t = xcb_render_color_t {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        let mut op: uint8_t = 0;
        color.red = (*dst).deferred_clear_color.red_short;
        color.green = (*dst).deferred_clear_color.green_short;
        color.blue = (*dst).deferred_clear_color.blue_short;
        color.alpha = (*dst).deferred_clear_color.alpha_short;
        if color.alpha as libc::c_int == 0 as libc::c_int {
            op = XCB_RENDER_PICT_OP_CLEAR as libc::c_int as uint8_t;
        } else {
            op = XCB_RENDER_PICT_OP_SRC as libc::c_int as uint8_t;
        }
        _cairo_xcb_surface_ensure_picture(dst);
        _cairo_xcb_connection_render_fill_rectangles(
            (*dst).connection,
            op,
            (*dst).picture,
            color,
            1 as libc::c_int as uint32_t,
            &mut rect,
        );
    } else {
        gc = _cairo_xcb_screen_get_gc((*dst).screen, (*dst).drawable, (*dst).depth);
        _cairo_xcb_connection_poly_fill_rectangle(
            (*dst).connection,
            (*dst).drawable,
            gc,
            1 as libc::c_int as uint32_t,
            &mut rect,
        );
        _cairo_xcb_screen_put_gc((*dst).screen, (*dst).depth, gc);
    }
    _cairo_xcb_connection_release((*dst).connection);
    (*dst).deferred_clear = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn need_bounded_clip(
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut flags: libc::c_uint = NEED_CLIP_REGION as libc::c_int as libc::c_uint;
    if _cairo_clip_is_region((*extents).clip) == 0 {
        flags |= NEED_CLIP_SURFACE as libc::c_int as libc::c_uint;
    }
    return flags as cairo_bool_t;
}
unsafe extern "C" fn need_unbounded_clip(
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_bool_t {
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*extents).is_bounded == 0 {
        flags |= NEED_CLIP_REGION as libc::c_int as libc::c_uint;
        if _cairo_clip_is_region((*extents).clip) == 0 {
            flags |= NEED_CLIP_SURFACE as libc::c_int as libc::c_uint;
        }
    }
    if !((*(*extents).clip).path).is_null() {
        flags |= NEED_CLIP_SURFACE as libc::c_int as libc::c_uint;
    }
    return flags as cairo_bool_t;
}
unsafe extern "C" fn _clip_and_composite(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src: *const cairo_pattern_t,
    mut draw_func: xcb_draw_func_t,
    mut mask_func: xcb_draw_func_t,
    mut draw_closure: *mut libc::c_void,
    mut extents: *mut cairo_composite_rectangles_t,
    mut need_clip: libc::c_uint,
) -> cairo_status_t {
    let mut clip_region: *mut cairo_region_t = 0 as *mut cairo_region_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_xcb_connection_acquire((*dst).connection);
    if status as u64 != 0 {
        return status;
    }
    if (*dst).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(dst);
        if status as u64 != 0 {
            _cairo_xcb_connection_release((*dst).connection);
            return status;
        }
    }
    _cairo_xcb_surface_ensure_picture(dst);
    if need_clip & NEED_CLIP_REGION as libc::c_int as libc::c_uint != 0 {
        clip_region = _cairo_clip_get_region((*extents).clip);
        if need_clip & FORCE_CLIP_REGION as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint && !clip_region.is_null()
            && cairo_region_contains_rectangle(clip_region, &mut (*extents).unbounded)
                as libc::c_uint == CAIRO_REGION_OVERLAP_IN as libc::c_int as libc::c_uint
        {
            clip_region = 0 as *mut cairo_region_t;
        }
        if !clip_region.is_null() {
            status = _cairo_xcb_surface_set_clip_region(dst, clip_region);
            if status as u64 != 0 {
                _cairo_xcb_connection_release((*dst).connection);
                return status;
            }
        }
    }
    if reduce_alpha_op(&mut (*dst).base, op, src) != 0 {
        op = CAIRO_OPERATOR_ADD;
        src = 0 as *const cairo_pattern_t;
    }
    if (*extents).bounded.width != 0 as libc::c_int
        && (*extents).bounded.height != 0 as libc::c_int
    {
        if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
            status = _clip_and_composite_source(
                (*extents).clip,
                src,
                draw_func,
                mask_func,
                draw_closure,
                dst,
                &mut (*extents).bounded,
            );
        } else {
            if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            {
                op = CAIRO_OPERATOR_DEST_OUT;
                src = 0 as *const cairo_pattern_t;
            }
            if need_clip & NEED_CLIP_SURFACE as libc::c_int as libc::c_uint != 0 {
                if (*extents).is_bounded != 0 {
                    status = _clip_and_composite_with_mask(
                        (*extents).clip,
                        op,
                        src,
                        draw_func,
                        mask_func,
                        draw_closure,
                        dst,
                        &mut (*extents).bounded,
                    );
                } else {
                    status = _clip_and_composite_combine(
                        (*extents).clip,
                        op,
                        src,
                        draw_func,
                        draw_closure,
                        dst,
                        &mut (*extents).bounded,
                    );
                }
            } else {
                status = draw_func
                    .expect(
                        "non-null function pointer",
                    )(
                    draw_closure,
                    dst,
                    op,
                    src,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    &mut (*extents).bounded,
                    (*extents).clip,
                ) as cairo_status_t;
            }
        }
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*extents).is_bounded == 0
    {
        if need_clip & NEED_CLIP_SURFACE as libc::c_int as libc::c_uint != 0 {
            status = _cairo_xcb_surface_fixup_unbounded_with_mask(
                dst,
                extents,
                (*extents).clip,
            );
        } else {
            status = _cairo_xcb_surface_fixup_unbounded(dst, extents);
        }
    }
    if !clip_region.is_null() {
        _cairo_xcb_surface_clear_clip_region(dst);
    }
    _cairo_xcb_connection_release((*dst).connection);
    return status;
}
unsafe extern "C" fn _core_boxes(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    if (*boxes).is_pixel_aligned == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_clip_is_region((*extents).clip) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
        return _cairo_xcb_surface_core_fill_boxes(
            dst,
            _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
            boxes,
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint {
        if ((*dst).base).is_clear() as libc::c_int != 0
            || _cairo_pattern_is_opaque(src, &(*extents).bounded) != 0
        {
            op = CAIRO_OPERATOR_SOURCE;
        }
    }
    if op as libc::c_uint != CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*src).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        return _cairo_xcb_surface_core_fill_boxes(
            dst,
            &mut (*(src as *mut cairo_solid_pattern_t)).color,
            boxes,
        );
    }
    return _cairo_xcb_surface_core_copy_boxes(dst, src, &(*extents).bounded, boxes);
}
unsafe extern "C" fn _composite_boxes(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
    mut extents: *const cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut clip: *mut cairo_clip_t = (*extents).clip;
    let mut need_clip_mask: cairo_bool_t = (_cairo_clip_is_region(clip) == 0)
        as libc::c_int;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*boxes).is_pixel_aligned == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if need_clip_mask != 0
        && ((*extents).is_bounded == 0
            || op as libc::c_uint
                == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = _cairo_xcb_connection_acquire((*dst).connection);
    if status as u64 != 0 {
        return status;
    }
    _cairo_xcb_surface_ensure_picture(dst);
    if (*(*dst).connection).flags
        & CAIRO_XCB_RENDER_HAS_FILL_RECTANGLES as libc::c_int as libc::c_uint != 0
        && need_clip_mask == 0
        && (op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            || (*src).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint)
    {
        let mut color: *const cairo_color_t = 0 as *const cairo_color_t;
        if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
            color = _cairo_stock_color(CAIRO_STOCK_TRANSPARENT);
        } else {
            color = &mut (*(src as *mut cairo_solid_pattern_t)).color;
        }
        status = _render_fill_boxes(dst as *mut libc::c_void, op, color, boxes);
    } else {
        let mut mask: cairo_surface_pattern_t = cairo_surface_pattern_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *const libc::c_char as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *const _cairo_list as *mut _cairo_list,
                    prev: 0 as *const _cairo_list as *mut _cairo_list,
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_FAST,
                extend: CAIRO_EXTEND_NONE,
                has_component_alpha: 0,
                is_userfont_foreground: 0,
                matrix: cairo_matrix_t {
                    xx: 0.,
                    yx: 0.,
                    xy: 0.,
                    yy: 0.,
                    x0: 0.,
                    y0: 0.,
                },
                opacity: 0.,
            },
            surface: 0 as *mut cairo_surface_t,
        };
        if need_clip_mask != 0 {
            let mut clip_surface: *mut cairo_xcb_surface_t = 0
                as *mut cairo_xcb_surface_t;
            let mut clip_x: libc::c_int = 0 as libc::c_int;
            let mut clip_y: libc::c_int = 0 as libc::c_int;
            clip_surface = get_clip_surface(
                (*extents).clip,
                dst,
                &mut clip_x,
                &mut clip_y,
            );
            if (*clip_surface).base.status as u64 != 0 {
                return (*clip_surface).base.status;
            }
            _cairo_pattern_init_for_surface(&mut mask, &mut (*clip_surface).base);
            mask.base.filter = CAIRO_FILTER_NEAREST;
            cairo_matrix_init_translate(
                &mut mask.base.matrix,
                -clip_x as libc::c_double,
                -clip_y as libc::c_double,
            );
            cairo_surface_destroy(&mut (*clip_surface).base);
            if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            {
                src = 0 as *const cairo_pattern_t;
                op = CAIRO_OPERATOR_DEST_OUT;
            }
        }
        status = _render_composite_boxes(
            dst,
            op,
            src,
            if need_clip_mask != 0 { &mut mask.base } else { 0 as *mut cairo_pattern_t },
            &(*extents).bounded,
            boxes,
        ) as cairo_status_t;
        if need_clip_mask != 0 {
            _cairo_pattern_fini(&mut mask.base);
        }
    }
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (*extents).is_bounded == 0
    {
        status = _cairo_xcb_surface_fixup_unbounded_boxes(dst, extents, clip, boxes);
    }
    _cairo_xcb_connection_release((*dst).connection);
    return status;
}
unsafe extern "C" fn cairo_boxes_for_each_box(
    mut boxes: *mut cairo_boxes_t,
    mut func: Option::<
        unsafe extern "C" fn(*mut cairo_box_t, *mut libc::c_void) -> cairo_bool_t,
    >,
    mut data: *mut libc::c_void,
) -> cairo_bool_t {
    let mut chunk: *mut _cairo_boxes_chunk = 0 as *mut _cairo_boxes_chunk;
    let mut i: libc::c_int = 0;
    chunk = &mut (*boxes).chunks;
    while !chunk.is_null() {
        i = 0 as libc::c_int;
        while i < (*chunk).count {
            if func
                .expect(
                    "non-null function pointer",
                )(&mut *((*chunk).base).offset(i as isize), data) == 0
            {
                return 0 as libc::c_int;
            }
            i += 1;
        }
        chunk = (*chunk).next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn image_contains_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut data: *mut _image_contains_box = closure as *mut _image_contains_box;
    return (_cairo_fixed_integer_part((*box_0).p1.x) + (*data).tx >= 0 as libc::c_int
        && _cairo_fixed_integer_part((*box_0).p1.y) + (*data).ty >= 0 as libc::c_int
        && _cairo_fixed_integer_part((*box_0).p2.x) + (*data).tx <= (*data).width
        && _cairo_fixed_integer_part((*box_0).p2.y) + (*data).ty <= (*data).height)
        as libc::c_int;
}
unsafe extern "C" fn image_upload_box(
    mut box_0: *mut cairo_box_t,
    mut closure: *mut libc::c_void,
) -> cairo_bool_t {
    let mut iub: *const _image_upload_box = closure as *const _image_upload_box;
    let mut x: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.x);
    let mut y: libc::c_int = _cairo_fixed_integer_part((*box_0).p1.y);
    let mut width: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.x - (*box_0).p1.x,
    );
    let mut height: libc::c_int = _cairo_fixed_integer_part(
        (*box_0).p2.y - (*box_0).p1.y,
    );
    let mut bpp: libc::c_int = (((*(*iub).image).pixman_format as libc::c_uint
        >> 24 as libc::c_int
        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        << ((*(*iub).image).pixman_format as libc::c_uint >> 22 as libc::c_int
            & 3 as libc::c_int as libc::c_uint)) as libc::c_int;
    let mut len: libc::c_int = ((((bpp * width + 7 as libc::c_int) / 8 as libc::c_int)
        as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
        as libc::c_int;
    if len as libc::c_long == (*(*iub).image).stride {
        _cairo_xcb_connection_put_image(
            (*(*iub).surface).connection,
            (*(*iub).surface).drawable,
            (*iub).gc,
            width as uint16_t,
            height as uint16_t,
            x as int16_t,
            y as int16_t,
            (*(*iub).image).depth as uint8_t,
            (*(*iub).image).stride as uint32_t,
            ((*(*iub).image).data)
                .offset(
                    ((y + (*iub).ty) as libc::c_long * (*(*iub).image).stride) as isize,
                )
                .offset(((x + (*iub).tx) * bpp / 8 as libc::c_int) as isize)
                as *mut libc::c_void,
        );
    } else {
        _cairo_xcb_connection_put_subimage(
            (*(*iub).surface).connection,
            (*(*iub).surface).drawable,
            (*iub).gc,
            (x + (*iub).tx) as int16_t,
            (y + (*iub).ty) as int16_t,
            width as uint16_t,
            height as uint16_t,
            (bpp / 8 as libc::c_int) as uint16_t,
            (*(*iub).image).stride as libc::c_int,
            x as int16_t,
            y as int16_t,
            (*(*iub).image).depth as uint8_t,
            (*(*iub).image).data as *mut libc::c_void,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _upload_image_inplace(
    mut surface: *mut cairo_xcb_surface_t,
    mut source: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
) -> cairo_status_t {
    let mut pattern: *const cairo_surface_pattern_t = 0
        as *const cairo_surface_pattern_t;
    let mut icb: _image_contains_box = _image_contains_box {
        width: 0,
        height: 0,
        tx: 0,
        ty: 0,
    };
    let mut iub: _image_upload_box = _image_upload_box {
        surface: 0 as *mut cairo_xcb_surface_t,
        image: 0 as *mut cairo_image_surface_t,
        gc: 0,
        tx: 0,
        ty: 0,
    };
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    if (*boxes).is_pixel_aligned == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*source).type_0 as libc::c_uint
        != CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    pattern = source as *const cairo_surface_pattern_t;
    if _cairo_surface_is_image((*pattern).surface) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    let mut snapshot: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    snapshot = _cairo_surface_has_snapshot(
        (*pattern).surface,
        &_cairo_xcb_picture_backend,
    ) as *mut cairo_xcb_picture_t;
    if !snapshot.is_null() {
        if (*snapshot).screen == (*surface).screen {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
    }
    image = (*pattern).surface as *mut cairo_image_surface_t;
    if (*image).format as libc::c_int == CAIRO_FORMAT_INVALID as libc::c_int {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*image).depth != (*surface).depth {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if _cairo_matrix_is_integer_translation(&(*source).matrix, &mut tx, &mut ty) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    icb.width = (*image).width;
    icb.height = (*image).height;
    icb.tx = tx;
    icb.ty = ty;
    if cairo_boxes_for_each_box(
        boxes,
        Some(
            image_contains_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut icb as *mut _image_contains_box as *mut libc::c_void,
    ) == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    if (*surface).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(surface);
        if status as u64 != 0 {
            return status;
        }
    }
    status = _cairo_xcb_connection_acquire((*surface).connection);
    if status as u64 != 0 {
        return status;
    }
    iub.surface = surface;
    iub.image = image;
    iub
        .gc = _cairo_xcb_screen_get_gc(
        (*surface).screen,
        (*surface).drawable,
        (*image).depth,
    );
    iub.tx = tx;
    iub.ty = ty;
    cairo_boxes_for_each_box(
        boxes,
        Some(
            image_upload_box
                as unsafe extern "C" fn(
                    *mut cairo_box_t,
                    *mut libc::c_void,
                ) -> cairo_bool_t,
        ),
        &mut iub as *mut _image_upload_box as *mut libc::c_void,
    );
    _cairo_xcb_screen_put_gc((*surface).screen, (*image).depth, iub.gc);
    _cairo_xcb_connection_release((*surface).connection);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn trim_extents_to_traps(
    mut extents: *mut cairo_composite_rectangles_t,
    mut traps: *mut cairo_traps_t,
) -> cairo_int_status_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    _cairo_traps_extents(traps, &mut box_0);
    return _cairo_composite_rectangles_intersect_mask_extents(extents, &mut box_0);
}
unsafe extern "C" fn _mono_edge_is_vertical(
    mut line: *const cairo_line_t,
) -> cairo_bool_t {
    return (_cairo_fixed_integer_round_down((*line).p1.x)
        == _cairo_fixed_integer_round_down((*line).p2.x)) as libc::c_int;
}
unsafe extern "C" fn _traps_are_pixel_aligned(
    mut traps: *mut cairo_traps_t,
    mut antialias: cairo_antialias_t,
) -> cairo_bool_t {
    let mut i: libc::c_int = 0;
    if antialias as libc::c_uint == CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            if _mono_edge_is_vertical(&mut (*((*traps).traps).offset(i as isize)).left)
                == 0
                || _mono_edge_is_vertical(
                    &mut (*((*traps).traps).offset(i as isize)).right,
                ) == 0
            {
                (*traps).set_maybe_region(0 as libc::c_int as libc::c_uint);
                return 0 as libc::c_int;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*traps).num_traps {
            if (*((*traps).traps).offset(i as isize)).left.p1.x
                != (*((*traps).traps).offset(i as isize)).left.p2.x
                || (*((*traps).traps).offset(i as isize)).right.p1.x
                    != (*((*traps).traps).offset(i as isize)).right.p2.x
                || _cairo_fixed_is_integer((*((*traps).traps).offset(i as isize)).top)
                    == 0
                || _cairo_fixed_is_integer((*((*traps).traps).offset(i as isize)).bottom)
                    == 0
                || _cairo_fixed_is_integer(
                    (*((*traps).traps).offset(i as isize)).left.p1.x,
                ) == 0
                || _cairo_fixed_is_integer(
                    (*((*traps).traps).offset(i as isize)).right.p1.x,
                ) == 0
            {
                (*traps).set_maybe_region(0 as libc::c_int as libc::c_uint);
                return 0 as libc::c_int;
            }
            i += 1;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _boxes_for_traps(
    mut boxes: *mut cairo_boxes_t,
    mut traps: *mut cairo_traps_t,
    mut antialias: cairo_antialias_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    _cairo_boxes_init(boxes);
    let ref mut fresh33 = (*boxes).chunks.base;
    *fresh33 = (*traps).traps as *mut cairo_box_t;
    (*boxes).chunks.size = (*traps).num_traps;
    if antialias as libc::c_uint != CAIRO_ANTIALIAS_NONE as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int;
        i = j;
        while i < (*traps).num_traps {
            let mut x1: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).left.p1.x;
            let mut x2: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .right
                .p1
                .x;
            let mut y1: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).top;
            let mut y2: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).bottom;
            if !(x1 == x2 || y1 == y2) {
                (*((*boxes).chunks.base).offset(j as isize)).p1.x = x1;
                (*((*boxes).chunks.base).offset(j as isize)).p1.y = y1;
                (*((*boxes).chunks.base).offset(j as isize)).p2.x = x2;
                (*((*boxes).chunks.base).offset(j as isize)).p2.y = y2;
                j += 1;
                if (*boxes).is_pixel_aligned != 0 {
                    (*boxes)
                        .is_pixel_aligned = (_cairo_fixed_is_integer(x1) != 0
                        && _cairo_fixed_is_integer(y1) != 0
                        && _cairo_fixed_is_integer(x2) != 0
                        && _cairo_fixed_is_integer(y2) != 0) as libc::c_int
                        as libc::c_uint;
                }
            }
            i += 1;
        }
    } else {
        (*boxes).is_pixel_aligned = 1 as libc::c_int as libc::c_uint;
        j = 0 as libc::c_int;
        i = j;
        while i < (*traps).num_traps {
            let mut x1_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .left
                .p1
                .x;
            let mut x2_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize))
                .right
                .p1
                .x;
            let mut y1_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).top;
            let mut y2_0: cairo_fixed_t = (*((*traps).traps).offset(i as isize)).bottom;
            (*((*boxes).chunks.base).offset(j as isize))
                .p1
                .x = _cairo_fixed_round_down(x1_0);
            (*((*boxes).chunks.base).offset(j as isize))
                .p1
                .y = _cairo_fixed_round_down(y1_0);
            (*((*boxes).chunks.base).offset(j as isize))
                .p2
                .x = _cairo_fixed_round_down(x2_0);
            (*((*boxes).chunks.base).offset(j as isize))
                .p2
                .y = _cairo_fixed_round_down(y2_0);
            j
                += ((*((*boxes).chunks.base).offset(j as isize)).p1.x
                    != (*((*boxes).chunks.base).offset(j as isize)).p2.x
                    && (*((*boxes).chunks.base).offset(j as isize)).p1.y
                        != (*((*boxes).chunks.base).offset(j as isize)).p2.y)
                    as libc::c_int;
            i += 1;
        }
    }
    (*boxes).num_boxes = j;
    (*boxes).chunks.count = j;
}
unsafe extern "C" fn _composite_polygon(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut polygon: *mut cairo_polygon_t,
    mut antialias: cairo_antialias_t,
    mut fill_rule: cairo_fill_rule_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut traps: composite_traps_info_t = composite_traps_info_t {
        traps: cairo_traps_t {
            status: CAIRO_STATUS_SUCCESS,
            bounds: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
            c2rust_padding: [0; 3],
            num_traps: 0,
            traps_size: 0,
            traps: 0 as *mut cairo_trapezoid_t,
            traps_embedded: [cairo_trapezoid_t {
                top: 0,
                bottom: 0,
                left: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                right: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
            }; 16],
        },
        antialias: CAIRO_ANTIALIAS_DEFAULT,
    };
    let mut clip_surface: cairo_bool_t = (_cairo_clip_is_region((*extents).clip) == 0)
        as libc::c_int;
    let mut clip_region: *mut cairo_region_t = _cairo_clip_get_region((*extents).clip);
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*polygon).num_edges == 0 as libc::c_int {
        status = CAIRO_STATUS_SUCCESS;
        if (*extents).is_bounded == 0 {
            if cairo_region_contains_rectangle(clip_region, &mut (*extents).unbounded)
                as libc::c_uint == CAIRO_REGION_OVERLAP_IN as libc::c_int as libc::c_uint
            {
                clip_region = 0 as *mut cairo_region_t;
            }
            if clip_surface == 0 as libc::c_int {
                if !clip_region.is_null() {
                    status = _cairo_xcb_surface_set_clip_region(dst, clip_region);
                    if status as u64 != 0 {
                        return status;
                    }
                }
                status = _cairo_xcb_surface_fixup_unbounded(dst, extents);
                if !clip_region.is_null() {
                    _cairo_xcb_surface_clear_clip_region(dst);
                }
            } else {
                status = _cairo_xcb_surface_fixup_unbounded_with_mask(
                    dst,
                    extents,
                    (*extents).clip,
                );
            }
        }
        return status;
    }
    if !((*(*extents).clip).path).is_null() && (*extents).is_bounded != 0 {
        let mut clipper: cairo_polygon_t = cairo_polygon_t {
            status: CAIRO_STATUS_SUCCESS,
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_edges: 0,
            edges_size: 0,
            edges: 0 as *mut cairo_edge_t,
            edges_embedded: [cairo_edge_t {
                line: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            }; 32],
        };
        let mut clipper_fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
        let mut clipper_antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
        status = _cairo_clip_get_polygon(
            (*extents).clip,
            &mut clipper,
            &mut clipper_fill_rule,
            &mut clipper_antialias,
        ) as cairo_status_t;
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            if clipper_antialias as libc::c_uint == antialias as libc::c_uint {
                status = _cairo_polygon_intersect(
                    polygon,
                    fill_rule as libc::c_int,
                    &mut clipper,
                    clipper_fill_rule as libc::c_int,
                );
                if status as libc::c_uint
                    == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {
                    let mut clip: *mut cairo_clip_t = _cairo_clip_copy_region(
                        (*extents).clip,
                    );
                    _cairo_clip_destroy((*extents).clip);
                    let ref mut fresh34 = (*extents).clip;
                    *fresh34 = clip;
                    fill_rule = CAIRO_FILL_RULE_WINDING;
                }
                _cairo_polygon_fini(&mut clipper);
            }
        }
    }
    _cairo_traps_init(&mut traps.traps);
    status = _cairo_bentley_ottmann_tessellate_polygon(
        &mut traps.traps,
        polygon,
        fill_rule,
    );
    if !(status as u64 != 0) {
        if (traps.traps).has_intersections() != 0 {
            if (traps.traps).is_rectangular() != 0 {
                status = _cairo_bentley_ottmann_tessellate_rectangular_traps(
                    &mut traps.traps,
                    CAIRO_FILL_RULE_WINDING,
                );
            } else if (traps.traps).is_rectilinear() != 0 {
                status = _cairo_bentley_ottmann_tessellate_rectilinear_traps(
                    &mut traps.traps,
                    CAIRO_FILL_RULE_WINDING,
                );
            } else {
                status = _cairo_bentley_ottmann_tessellate_traps(
                    &mut traps.traps,
                    CAIRO_FILL_RULE_WINDING,
                );
            }
            if status as u64 != 0 {
                current_block = 15213290871723755507;
            } else {
                current_block = 14447253356787937536;
            }
        } else {
            current_block = 14447253356787937536;
        }
        match current_block {
            15213290871723755507 => {}
            _ => {
                if (traps.traps).maybe_region() as libc::c_int != 0
                    && _traps_are_pixel_aligned(&mut traps.traps, antialias) != 0
                    && (clip_surface == 0
                        || (*extents).is_bounded != 0
                            && op as libc::c_uint
                                != CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint)
                {
                    let mut boxes: cairo_boxes_t = cairo_boxes_t {
                        status: CAIRO_STATUS_SUCCESS,
                        limit: cairo_box_t {
                            p1: cairo_point_t { x: 0, y: 0 },
                            p2: cairo_point_t { x: 0, y: 0 },
                        },
                        limits: 0 as *const cairo_box_t,
                        num_limits: 0,
                        num_boxes: 0,
                        is_pixel_aligned: 0,
                        chunks: _cairo_boxes_chunk {
                            next: 0 as *mut _cairo_boxes_chunk,
                            base: 0 as *mut cairo_box_t,
                            count: 0,
                            size: 0,
                        },
                        tail: 0 as *mut _cairo_boxes_chunk,
                        boxes_embedded: [cairo_box_t {
                            p1: cairo_point_t { x: 0, y: 0 },
                            p2: cairo_point_t { x: 0, y: 0 },
                        }; 32],
                    };
                    _boxes_for_traps(&mut boxes, &mut traps.traps, antialias);
                    status = _clip_and_composite_boxes(
                        dst,
                        op,
                        source,
                        &mut boxes,
                        extents,
                    );
                } else {
                    traps.antialias = antialias;
                    status = trim_extents_to_traps(extents, &mut traps.traps)
                        as cairo_status_t;
                    if status as libc::c_uint
                        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    {
                        let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                        if (*extents).is_bounded == 0 {
                            flags |= FORCE_CLIP_REGION as libc::c_int as libc::c_uint;
                        }
                        status = _clip_and_composite(
                            dst,
                            op,
                            source,
                            Some(
                                _composite_traps
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                        *mut cairo_xcb_surface_t,
                                        cairo_operator_t,
                                        *const cairo_pattern_t,
                                        libc::c_int,
                                        libc::c_int,
                                        *const cairo_rectangle_int_t,
                                        *mut cairo_clip_t,
                                    ) -> cairo_int_status_t,
                            ),
                            None,
                            &mut traps as *mut composite_traps_info_t
                                as *mut libc::c_void,
                            extents,
                            need_unbounded_clip(extents) as libc::c_uint | flags,
                        );
                    }
                }
            }
        }
    }
    _cairo_traps_fini(&mut traps.traps);
    return status;
}
unsafe extern "C" fn _clip_and_composite_boxes(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src: *const cairo_pattern_t,
    mut boxes: *mut cairo_boxes_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut info: composite_traps_info_t = composite_traps_info_t {
        traps: cairo_traps_t {
            status: CAIRO_STATUS_SUCCESS,
            bounds: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
            c2rust_padding: [0; 3],
            num_traps: 0,
            traps_size: 0,
            traps: 0 as *mut cairo_trapezoid_t,
            traps_embedded: [cairo_trapezoid_t {
                top: 0,
                bottom: 0,
                left: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                right: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
            }; 16],
        },
        antialias: CAIRO_ANTIALIAS_DEFAULT,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*boxes).num_boxes == 0 as libc::c_int && (*extents).is_bounded != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    if (*boxes).is_pixel_aligned != 0 && _cairo_clip_is_region((*extents).clip) != 0
        && (op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || ((*dst).base).is_clear() as libc::c_int != 0
                && (op as libc::c_uint
                    == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
                    || op as libc::c_uint
                        == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint))
    {
        if (*boxes).num_boxes == 1 as libc::c_int
            && (*extents).bounded.width == (*dst).width
            && (*extents).bounded.height == (*dst).height
        {
            op = CAIRO_OPERATOR_SOURCE;
            (*dst).deferred_clear = 0 as libc::c_int;
        }
        status = _upload_image_inplace(dst, src, boxes) as cairo_int_status_t;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    if (*(*dst).connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint != 0
        && !((*(*extents).clip).path).is_null() && (*extents).is_bounded != 0
    {
        let mut polygon: cairo_polygon_t = cairo_polygon_t {
            status: CAIRO_STATUS_SUCCESS,
            extents: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_edges: 0,
            edges_size: 0,
            edges: 0 as *mut cairo_edge_t,
            edges_embedded: [cairo_edge_t {
                line: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                top: 0,
                bottom: 0,
                dir: 0,
            }; 32],
        };
        let mut fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
        let mut antialias: cairo_antialias_t = CAIRO_ANTIALIAS_DEFAULT;
        let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
        clip = _cairo_clip_copy((*extents).clip);
        clip = _cairo_clip_intersect_boxes(clip, boxes);
        if _cairo_clip_is_all_clipped(clip) != 0 {
            return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
        }
        status = _cairo_clip_get_polygon(
            clip,
            &mut polygon,
            &mut fill_rule,
            &mut antialias,
        );
        _cairo_clip_path_destroy((*clip).path);
        let ref mut fresh35 = (*clip).path;
        *fresh35 = 0 as *mut cairo_clip_path_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            let mut saved_clip: *mut cairo_clip_t = (*extents).clip;
            let ref mut fresh36 = (*extents).clip;
            *fresh36 = clip;
            status = _composite_polygon(
                dst,
                op,
                src,
                &mut polygon,
                antialias,
                fill_rule,
                extents,
            ) as cairo_int_status_t;
            clip = (*extents).clip;
            let ref mut fresh37 = (*extents).clip;
            *fresh37 = saved_clip;
            _cairo_polygon_fini(&mut polygon);
        }
        if !clip.is_null() {
            _cairo_clip_destroy(clip);
        }
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    if (*dst).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(dst) as cairo_int_status_t;
        if status as u64 != 0 {
            return status as cairo_status_t;
        }
    }
    if (*boxes).is_pixel_aligned != 0 && _cairo_clip_is_region((*extents).clip) != 0
        && op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
    {
        status = _upload_image_inplace(dst, src, boxes) as cairo_int_status_t;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status as cairo_status_t;
        }
    }
    if (*(*dst).connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return _core_boxes(dst, op, src, boxes, extents);
    }
    status = _composite_boxes(dst, op, src, boxes, extents) as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status as cairo_status_t;
    }
    if (*(*dst).connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    status = _cairo_traps_init_boxes(&mut info.traps, boxes) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    info.antialias = CAIRO_ANTIALIAS_DEFAULT;
    status = trim_extents_to_traps(extents, &mut info.traps);
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = _clip_and_composite(
            dst,
            op,
            src,
            Some(
                _composite_traps
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            None,
            &mut info as *mut composite_traps_info_t as *mut libc::c_void,
            extents,
            need_unbounded_clip(extents) as libc::c_uint,
        ) as cairo_int_status_t;
    }
    _cairo_traps_fini(&mut info.traps);
    return status as cairo_status_t;
}
unsafe extern "C" fn _composite_mask(
    mut closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut mask_pattern: *const cairo_pattern_t = closure as *const cairo_pattern_t;
    let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut mask: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if ((*dst).base).is_clear() != 0 {
        if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint
        {
            op = CAIRO_OPERATOR_SOURCE;
        }
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        && clip.is_null()
    {
        (*dst).deferred_clear = 0 as libc::c_int;
    }
    if (*dst).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(dst);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    if !src_pattern.is_null() {
        src = _cairo_xcb_picture_for_pattern(dst, src_pattern, extents);
        if (*src).base.status as u64 != 0 {
            return (*src).base.status as cairo_int_status_t;
        }
        mask = _cairo_xcb_picture_for_pattern(dst, mask_pattern, extents);
        if (*mask).base.status as u64 != 0 {
            cairo_surface_destroy(&mut (*src).base);
            return (*mask).base.status as cairo_int_status_t;
        }
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            _render_operator(op) as uint8_t,
            (*src).picture,
            (*mask).picture,
            (*dst).picture,
            ((*extents).x + (*src).x) as int16_t,
            ((*extents).y + (*src).y) as int16_t,
            ((*extents).x + (*mask).x) as int16_t,
            ((*extents).y + (*mask).y) as int16_t,
            ((*extents).x - dst_x) as int16_t,
            ((*extents).y - dst_y) as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
        cairo_surface_destroy(&mut (*mask).base);
        cairo_surface_destroy(&mut (*src).base);
    } else {
        src = _cairo_xcb_picture_for_pattern(dst, mask_pattern, extents);
        if (*src).base.status as u64 != 0 {
            return (*src).base.status as cairo_int_status_t;
        }
        _cairo_xcb_connection_render_composite(
            (*dst).connection,
            _render_operator(op) as uint8_t,
            (*src).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*dst).picture,
            ((*extents).x + (*src).x) as int16_t,
            ((*extents).y + (*src).y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            ((*extents).x - dst_x) as int16_t,
            ((*extents).y - dst_y) as int16_t,
            (*extents).width as uint16_t,
            (*extents).height as uint16_t,
        );
        cairo_surface_destroy(&mut (*src).base);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn composite_box(
    mut closure: *mut libc::c_void,
    mut x: int16_t,
    mut y: int16_t,
    mut w: int16_t,
    mut h: int16_t,
    mut coverage: uint16_t,
) {
    let mut info: *mut composite_box_info = closure as *mut composite_box_info;
    if (coverage as libc::c_int) < 0xff00 as libc::c_int {
        let mut mask: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
        let mut color: cairo_color_t = cairo_color_t {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.,
            red_short: 0,
            green_short: 0,
            blue_short: 0,
            alpha_short: 0,
        };
        color.blue_short = 0 as libc::c_int as libc::c_ushort;
        color.green_short = color.blue_short;
        color.red_short = color.green_short;
        color.alpha_short = coverage;
        mask = _solid_picture((*info).dst, &mut color);
        if (*mask).base.status as libc::c_uint
            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            _cairo_xcb_connection_render_composite(
                (*(*info).dst).connection,
                (*info).op,
                (*(*info).src).picture,
                (*mask).picture,
                (*(*info).dst).picture,
                (x as libc::c_int + (*(*info).src).x) as int16_t,
                (y as libc::c_int + (*(*info).src).y) as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                x,
                y,
                w as uint16_t,
                h as uint16_t,
            );
        }
        cairo_surface_destroy(&mut (*mask).base);
    } else {
        _cairo_xcb_connection_render_composite(
            (*(*info).dst).connection,
            (*info).op,
            (*(*info).src).picture,
            0 as libc::c_long as xcb_render_picture_t,
            (*(*info).dst).picture,
            (x as libc::c_int + (*(*info).src).x) as int16_t,
            (y as libc::c_int + (*(*info).src).y) as int16_t,
            0 as libc::c_int as int16_t,
            0 as libc::c_int as int16_t,
            x,
            y,
            w as uint16_t,
            h as uint16_t,
        );
    };
}
unsafe extern "C" fn _composite_mask_clip_boxes(
    mut closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut info: composite_box_info = composite_box_info {
        dst: 0 as *mut cairo_xcb_surface_t,
        src: 0 as *mut cairo_xcb_picture_t,
        op: 0,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if src_pattern.is_null() {} else {
        __assert_fail(
            b"src_pattern == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            3330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"cairo_int_status_t _composite_mask_clip_boxes(void *, cairo_xcb_surface_t *, cairo_operator_t, const cairo_pattern_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"op == CAIRO_OPERATOR_ADD\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            3331 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"cairo_int_status_t _composite_mask_clip_boxes(void *, cairo_xcb_surface_t *, cairo_operator_t, const cairo_pattern_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*dst).base).is_clear() != 0 {} else {
        __assert_fail(
            b"dst->base.is_clear\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            3332 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 177],
                &[libc::c_char; 177],
            >(
                b"cairo_int_status_t _composite_mask_clip_boxes(void *, cairo_xcb_surface_t *, cairo_operator_t, const cairo_pattern_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*clip).num_boxes > 1 as libc::c_int {
        status = _cairo_xcb_surface_clear(dst);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    info.op = XCB_RENDER_PICT_OP_SRC as libc::c_int as uint8_t;
    info.dst = dst;
    info
        .src = _cairo_xcb_picture_for_pattern(
        dst,
        closure as *const cairo_pattern_t,
        extents,
    );
    if (*info.src).base.status as u64 != 0 {
        return (*info.src).base.status as cairo_int_status_t;
    }
    (*info.src).x += dst_x;
    (*info.src).y += dst_y;
    i = 0 as libc::c_int;
    while i < (*clip).num_boxes {
        do_unaligned_box(
            Some(
                composite_box
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        int16_t,
                        int16_t,
                        int16_t,
                        int16_t,
                        uint16_t,
                    ) -> (),
            ),
            &mut info as *mut composite_box_info as *mut libc::c_void,
            &mut *((*clip).boxes).offset(i as isize),
            dst_x,
            dst_y,
        );
        i += 1;
    }
    cairo_surface_destroy(&mut (*info.src).base);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _composite_mask_clip(
    mut closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut mask_pattern: *const cairo_pattern_t = closure as *const cairo_pattern_t;
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut fill_rule: cairo_fill_rule_t = CAIRO_FILL_RULE_WINDING;
    let mut info: composite_traps_info_t = composite_traps_info_t {
        traps: cairo_traps_t {
            status: CAIRO_STATUS_SUCCESS,
            bounds: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
            c2rust_padding: [0; 3],
            num_traps: 0,
            traps_size: 0,
            traps: 0 as *mut cairo_trapezoid_t,
            traps_embedded: [cairo_trapezoid_t {
                top: 0,
                bottom: 0,
                left: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
                right: cairo_box_t {
                    p1: cairo_point_t { x: 0, y: 0 },
                    p2: cairo_point_t { x: 0, y: 0 },
                },
            }; 16],
        },
        antialias: CAIRO_ANTIALIAS_DEFAULT,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if src_pattern.is_null() {} else {
        __assert_fail(
            b"src_pattern == NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            3372 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 171],
                &[libc::c_char; 171],
            >(
                b"cairo_int_status_t _composite_mask_clip(void *, cairo_xcb_surface_t *, cairo_operator_t, const cairo_pattern_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"op == CAIRO_OPERATOR_ADD\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            3373 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 171],
                &[libc::c_char; 171],
            >(
                b"cairo_int_status_t _composite_mask_clip(void *, cairo_xcb_surface_t *, cairo_operator_t, const cairo_pattern_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if ((*dst).base).is_clear() != 0 {} else {
        __assert_fail(
            b"dst->base.is_clear\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            3374 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 171],
                &[libc::c_char; 171],
            >(
                b"cairo_int_status_t _composite_mask_clip(void *, cairo_xcb_surface_t *, cairo_operator_t, const cairo_pattern_t *, int, int, const cairo_rectangle_int_t *, cairo_clip_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_clip_get_polygon(
        clip,
        &mut polygon,
        &mut fill_rule,
        &mut info.antialias,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_traps_init(&mut info.traps);
    status = _cairo_bentley_ottmann_tessellate_polygon(
        &mut info.traps,
        &mut polygon,
        fill_rule,
    );
    _cairo_polygon_fini(&mut polygon);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if (info.traps).has_intersections() != 0 {
        if (info.traps).is_rectangular() != 0 {
            status = _cairo_bentley_ottmann_tessellate_rectangular_traps(
                &mut info.traps,
                CAIRO_FILL_RULE_WINDING,
            );
        } else if (info.traps).is_rectilinear() != 0 {
            status = _cairo_bentley_ottmann_tessellate_rectilinear_traps(
                &mut info.traps,
                CAIRO_FILL_RULE_WINDING,
            );
        } else {
            status = _cairo_bentley_ottmann_tessellate_traps(
                &mut info.traps,
                CAIRO_FILL_RULE_WINDING,
            );
        }
        if status as u64 != 0 {
            _cairo_traps_fini(&mut info.traps);
            return status as cairo_int_status_t;
        }
    }
    status = _composite_traps(
        &mut info as *mut composite_traps_info_t as *mut libc::c_void,
        dst,
        CAIRO_OPERATOR_SOURCE,
        mask_pattern,
        dst_x,
        dst_y,
        extents,
        0 as *mut cairo_clip_t,
    ) as cairo_status_t;
    _cairo_traps_fini(&mut info.traps);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn composite_opacity(
    mut closure: *mut libc::c_void,
    mut x: int16_t,
    mut y: int16_t,
    mut w: int16_t,
    mut h: int16_t,
    mut coverage: uint16_t,
) {
    let mut info: *mut composite_opacity_info = closure as *mut composite_opacity_info;
    let mut mask: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut color: cairo_color_t = cairo_color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        red_short: 0,
        green_short: 0,
        blue_short: 0,
        alpha_short: 0,
    };
    color.blue_short = 0 as libc::c_int as libc::c_ushort;
    color.green_short = color.blue_short;
    color.red_short = color.green_short;
    color
        .alpha_short = ((*info).opacity * coverage as libc::c_int as libc::c_double)
        as libc::c_ushort;
    mask = _solid_picture((*info).dst, &mut color);
    if (*mask).base.status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        if !((*info).src).is_null() {
            _cairo_xcb_connection_render_composite(
                (*(*info).dst).connection,
                (*info).op,
                (*(*info).src).picture,
                (*mask).picture,
                (*(*info).dst).picture,
                (x as libc::c_int + (*(*info).src).x) as int16_t,
                (y as libc::c_int + (*(*info).src).y) as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                x,
                y,
                w as uint16_t,
                h as uint16_t,
            );
        } else {
            _cairo_xcb_connection_render_composite(
                (*(*info).dst).connection,
                (*info).op,
                (*mask).picture,
                0 as libc::c_long as xcb_render_picture_t,
                (*(*info).dst).picture,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                0 as libc::c_int as int16_t,
                x,
                y,
                w as uint16_t,
                h as uint16_t,
            );
        }
    }
    cairo_surface_destroy(&mut (*mask).base);
}
unsafe extern "C" fn _composite_opacity_boxes(
    mut closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src_pattern: *const cairo_pattern_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut mask_pattern: *const cairo_solid_pattern_t = closure
        as *const cairo_solid_pattern_t;
    let mut info: composite_opacity_info = composite_opacity_info {
        op: 0,
        dst: 0 as *mut cairo_xcb_surface_t,
        src: 0 as *mut cairo_xcb_picture_t,
        opacity: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if ((*dst).base).is_clear() != 0 {
        if op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint
        {
            op = CAIRO_OPERATOR_SOURCE;
        }
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        && (clip.is_null()
            || (*clip).extents.width >= (*extents).width
                && (*clip).extents.height >= (*extents).height)
    {
        (*dst).deferred_clear = 0 as libc::c_int;
    }
    if (*dst).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(dst);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    info.op = _render_operator(op) as uint8_t;
    info.dst = dst;
    if !src_pattern.is_null() {
        info.src = _cairo_xcb_picture_for_pattern(dst, src_pattern, extents);
        if (*info.src).base.status as u64 != 0 {
            return (*info.src).base.status as cairo_int_status_t;
        }
    } else {
        info.src = 0 as *mut cairo_xcb_picture_t;
    }
    info.opacity = (*mask_pattern).color.alpha;
    if !clip.is_null() {
        i = 0 as libc::c_int;
        while i < (*clip).num_boxes {
            do_unaligned_box(
                Some(
                    composite_opacity
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            int16_t,
                            int16_t,
                            int16_t,
                            int16_t,
                            uint16_t,
                        ) -> (),
                ),
                &mut info as *mut composite_opacity_info as *mut libc::c_void,
                &mut *((*clip).boxes).offset(i as isize),
                dst_x,
                dst_y,
            );
            i += 1;
        }
    } else {
        composite_opacity(
            &mut info as *mut composite_opacity_info as *mut libc::c_void,
            ((*extents).x - dst_x) as int16_t,
            ((*extents).y - dst_y) as int16_t,
            (*extents).width as int16_t,
            (*extents).height as int16_t,
            0xffff as libc::c_int as uint16_t,
        );
    }
    cairo_surface_destroy(&mut (*info.src).base);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_render_compositor_paint(
    mut compositor: *const cairo_compositor_t,
    mut composite: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*composite).surface
        as *mut cairo_xcb_surface_t;
    let mut op: cairo_operator_t = (*composite).op;
    let mut source: *mut cairo_pattern_t = &mut (*composite).source_pattern.base;
    let mut boxes: cairo_boxes_t = cairo_boxes_t {
        status: CAIRO_STATUS_SUCCESS,
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_boxes: 0,
        is_pixel_aligned: 0,
        chunks: _cairo_boxes_chunk {
            next: 0 as *mut _cairo_boxes_chunk,
            base: 0 as *mut cairo_box_t,
            count: 0,
            size: 0,
        },
        tail: 0 as *mut _cairo_boxes_chunk,
        boxes_embedded: [cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        }; 32],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _operator_is_supported((*(*surface).connection).flags, op) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*(*surface).connection).flags
        & (CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int
            | CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if ((*composite).clip).is_null()
        && (*source).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
        && (op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
            || op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint
            || ((*surface).base).is_clear() as libc::c_int != 0
                && (op as libc::c_uint
                    == CAIRO_OPERATOR_ADD as libc::c_int as libc::c_uint
                    || op as libc::c_uint
                        == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint))
    {
        (*surface).deferred_clear = 1 as libc::c_int;
        (*surface).deferred_clear_color = (*composite).source_pattern.solid.color;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    _cairo_clip_steal_boxes((*composite).clip, &mut boxes);
    status = _clip_and_composite_boxes(surface, op, source, &mut boxes, composite);
    _cairo_clip_unsteal_boxes((*composite).clip, &mut boxes);
    return status as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_render_compositor_mask(
    mut compositor: *const cairo_compositor_t,
    mut composite: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*composite).surface
        as *mut cairo_xcb_surface_t;
    let mut op: cairo_operator_t = (*composite).op;
    let mut source: *mut cairo_pattern_t = &mut (*composite).source_pattern.base;
    let mut mask: *mut cairo_pattern_t = &mut (*composite).mask_pattern.base;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _operator_is_supported((*(*surface).connection).flags, op) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*(*surface).connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*mask).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
        && ((*(*composite).clip).path).is_null()
        && _cairo_clip_is_region((*composite).clip) == 0
    {
        status = _clip_and_composite(
            surface,
            op,
            source,
            Some(
                _composite_opacity_boxes
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            Some(
                _composite_opacity_boxes
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask as *mut libc::c_void,
            composite,
            need_unbounded_clip(composite) as libc::c_uint,
        );
    } else {
        let mut mask_func: xcb_draw_func_t = None;
        if (*(*surface).connection).flags
            & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint
            != 0
        {
            mask_func = if !((*(*composite).clip).path).is_null() {
                Some(
                    _composite_mask_clip
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut cairo_xcb_surface_t,
                            cairo_operator_t,
                            *const cairo_pattern_t,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_rectangle_int_t,
                            *mut cairo_clip_t,
                        ) -> cairo_int_status_t,
                )
            } else {
                Some(
                    _composite_mask_clip_boxes
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut cairo_xcb_surface_t,
                            cairo_operator_t,
                            *const cairo_pattern_t,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_rectangle_int_t,
                            *mut cairo_clip_t,
                        ) -> cairo_int_status_t,
                )
            };
        }
        status = _clip_and_composite(
            surface,
            op,
            source,
            Some(
                _composite_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask_func,
            mask as *mut libc::c_void,
            composite,
            need_bounded_clip(composite) as libc::c_uint,
        );
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_xcb_surface_render_stroke_as_polygon(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_polygon_init_with_clip(&mut polygon, (*extents).clip);
    status = _cairo_path_fixed_stroke_to_polygon(
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        &mut polygon,
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _composite_polygon(
            dst,
            op,
            source,
            &mut polygon,
            antialias,
            CAIRO_FILL_RULE_WINDING,
            extents,
        );
    }
    _cairo_polygon_fini(&mut polygon);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_xcb_surface_render_stroke_via_mask(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    x = (*extents).bounded.x;
    y = (*extents).bounded.y;
    image = _cairo_xcb_surface_create_similar_image(
        dst as *mut libc::c_void,
        CAIRO_FORMAT_A8,
        (*extents).bounded.width,
        (*extents).bounded.height,
    );
    if (*image).status as u64 != 0 {
        return (*image).status;
    }
    clip = _cairo_clip_copy_region((*extents).clip);
    status = _cairo_surface_offset_stroke(
        image,
        x,
        y,
        CAIRO_OPERATOR_ADD,
        &_cairo_pattern_white.base,
        path,
        stroke_style,
        ctm,
        ctm_inverse,
        tolerance,
        antialias,
        clip,
    );
    _cairo_clip_destroy(clip);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        let mut mask: cairo_surface_pattern_t = cairo_surface_pattern_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *const libc::c_char as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *const _cairo_list as *mut _cairo_list,
                    prev: 0 as *const _cairo_list as *mut _cairo_list,
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_FAST,
                extend: CAIRO_EXTEND_NONE,
                has_component_alpha: 0,
                is_userfont_foreground: 0,
                matrix: cairo_matrix_t {
                    xx: 0.,
                    yx: 0.,
                    xy: 0.,
                    yy: 0.,
                    x0: 0.,
                    y0: 0.,
                },
                opacity: 0.,
            },
            surface: 0 as *mut cairo_surface_t,
        };
        _cairo_pattern_init_for_surface(&mut mask, image);
        mask.base.filter = CAIRO_FILTER_NEAREST;
        cairo_matrix_init_translate(
            &mut mask.base.matrix,
            -x as libc::c_double,
            -y as libc::c_double,
        );
        status = _clip_and_composite(
            dst,
            op,
            source,
            Some(
                _composite_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            None,
            &mut mask.base as *mut cairo_pattern_t as *mut libc::c_void,
            extents,
            need_bounded_clip(extents) as libc::c_uint,
        );
        _cairo_pattern_fini(&mut mask.base);
    }
    cairo_surface_finish(image);
    cairo_surface_destroy(image);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_render_compositor_stroke(
    mut compositor: *const cairo_compositor_t,
    mut composite: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*composite).surface
        as *mut cairo_xcb_surface_t;
    let mut op: cairo_operator_t = (*composite).op;
    let mut source: *mut cairo_pattern_t = &mut (*composite).source_pattern.base;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if _operator_is_supported((*(*surface).connection).flags, op) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*(*surface).connection).flags
        & (CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int
            | CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if _cairo_path_fixed_stroke_is_rectilinear(path) != 0 {
        let mut boxes: cairo_boxes_t = cairo_boxes_t {
            status: CAIRO_STATUS_SUCCESS,
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_boxes: 0,
            is_pixel_aligned: 0,
            chunks: _cairo_boxes_chunk {
                next: 0 as *mut _cairo_boxes_chunk,
                base: 0 as *mut cairo_box_t,
                count: 0,
                size: 0,
            },
            tail: 0 as *mut _cairo_boxes_chunk,
            boxes_embedded: [cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            }; 32],
        };
        _cairo_boxes_init_with_clip(&mut boxes, (*composite).clip);
        status = _cairo_path_fixed_stroke_rectilinear_to_boxes(
            path,
            style,
            ctm,
            antialias,
            &mut boxes,
        );
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = _clip_and_composite_boxes(
                surface,
                op,
                source,
                &mut boxes,
                composite,
            ) as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        if (*(*surface).connection).flags
            & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint
            != 0
        {
            status = _cairo_xcb_surface_render_stroke_as_polygon(
                surface,
                op,
                source,
                path,
                style,
                ctm,
                ctm_inverse,
                tolerance,
                antialias,
                composite,
            );
        } else if (*(*surface).connection).flags
            & CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint != 0
        {
            status = _cairo_xcb_surface_render_stroke_via_mask(
                surface,
                op,
                source,
                path,
                style,
                ctm,
                ctm_inverse,
                tolerance,
                antialias,
                composite,
            ) as cairo_int_status_t;
        } else if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                3737 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 248],
                    &[libc::c_char; 248],
                >(
                    b"cairo_int_status_t _cairo_xcb_render_compositor_stroke(const cairo_compositor_t *, cairo_composite_rectangles_t *, const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_xcb_surface_render_fill_as_polygon(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut polygon: cairo_polygon_t = cairo_polygon_t {
        status: CAIRO_STATUS_SUCCESS,
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limit: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        limits: 0 as *const cairo_box_t,
        num_limits: 0,
        num_edges: 0,
        edges_size: 0,
        edges: 0 as *mut cairo_edge_t,
        edges_embedded: [cairo_edge_t {
            line: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            top: 0,
            bottom: 0,
            dir: 0,
        }; 32],
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_polygon_init_with_clip(&mut polygon, (*extents).clip);
    status = _cairo_path_fixed_fill_to_polygon(path, tolerance, &mut polygon);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = _composite_polygon(
            dst,
            op,
            source,
            &mut polygon,
            antialias,
            fill_rule,
            extents,
        );
    }
    _cairo_polygon_fini(&mut polygon);
    return status;
}
unsafe extern "C" fn _cairo_xcb_surface_render_fill_via_mask(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    x = (*extents).bounded.x;
    y = (*extents).bounded.y;
    image = _cairo_xcb_surface_create_similar_image(
        dst as *mut libc::c_void,
        CAIRO_FORMAT_A8,
        (*extents).bounded.width,
        (*extents).bounded.height,
    );
    if (*image).status as u64 != 0 {
        return (*image).status;
    }
    clip = _cairo_clip_copy_region((*extents).clip);
    status = _cairo_surface_offset_fill(
        image,
        x,
        y,
        CAIRO_OPERATOR_ADD,
        &_cairo_pattern_white.base,
        path,
        fill_rule,
        tolerance,
        antialias,
        clip,
    );
    _cairo_clip_destroy(clip);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        let mut mask: cairo_surface_pattern_t = cairo_surface_pattern_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *const libc::c_char as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *const _cairo_list as *mut _cairo_list,
                    prev: 0 as *const _cairo_list as *mut _cairo_list,
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_FAST,
                extend: CAIRO_EXTEND_NONE,
                has_component_alpha: 0,
                is_userfont_foreground: 0,
                matrix: cairo_matrix_t {
                    xx: 0.,
                    yx: 0.,
                    xy: 0.,
                    yy: 0.,
                    x0: 0.,
                    y0: 0.,
                },
                opacity: 0.,
            },
            surface: 0 as *mut cairo_surface_t,
        };
        _cairo_pattern_init_for_surface(&mut mask, image);
        mask.base.filter = CAIRO_FILTER_NEAREST;
        cairo_matrix_init_translate(
            &mut mask.base.matrix,
            -x as libc::c_double,
            -y as libc::c_double,
        );
        status = _clip_and_composite(
            dst,
            op,
            source,
            Some(
                _composite_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            None,
            &mut mask.base as *mut cairo_pattern_t as *mut libc::c_void,
            extents,
            need_bounded_clip(extents) as libc::c_uint,
        );
        _cairo_pattern_fini(&mut mask.base);
    }
    cairo_surface_finish(image);
    cairo_surface_destroy(image);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_render_compositor_fill(
    mut compositor: *const cairo_compositor_t,
    mut composite: *mut cairo_composite_rectangles_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*composite).surface
        as *mut cairo_xcb_surface_t;
    let mut op: cairo_operator_t = (*composite).op;
    let mut source: *mut cairo_pattern_t = &mut (*composite).source_pattern.base;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if _operator_is_supported((*(*surface).connection).flags, op) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*(*surface).connection).flags
        & (CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int
            | CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if _cairo_path_fixed_fill_is_rectilinear(path) != 0 {
        let mut boxes: cairo_boxes_t = cairo_boxes_t {
            status: CAIRO_STATUS_SUCCESS,
            limit: cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            },
            limits: 0 as *const cairo_box_t,
            num_limits: 0,
            num_boxes: 0,
            is_pixel_aligned: 0,
            chunks: _cairo_boxes_chunk {
                next: 0 as *mut _cairo_boxes_chunk,
                base: 0 as *mut cairo_box_t,
                count: 0,
                size: 0,
            },
            tail: 0 as *mut _cairo_boxes_chunk,
            boxes_embedded: [cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            }; 32],
        };
        _cairo_boxes_init_with_clip(&mut boxes, (*composite).clip);
        status = _cairo_path_fixed_fill_rectilinear_to_boxes(
            path,
            fill_rule,
            antialias,
            &mut boxes,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = _clip_and_composite_boxes(
                surface,
                op,
                source,
                &mut boxes,
                composite,
            ) as cairo_int_status_t;
        }
        _cairo_boxes_fini(&mut boxes);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        if (*(*surface).connection).flags
            & CAIRO_XCB_RENDER_HAS_COMPOSITE_TRAPEZOIDS as libc::c_int as libc::c_uint
            != 0
        {
            status = _cairo_xcb_surface_render_fill_as_polygon(
                surface,
                op,
                source,
                path,
                fill_rule,
                tolerance,
                antialias,
                composite,
            ) as cairo_int_status_t;
        } else if (*(*surface).connection).flags
            & CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint != 0
        {
            status = _cairo_xcb_surface_render_fill_via_mask(
                surface,
                op,
                source,
                path,
                fill_rule,
                tolerance,
                antialias,
                composite,
            ) as cairo_int_status_t;
        } else if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
            __assert_fail(
                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                3869 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 187],
                    &[libc::c_char; 187],
                >(
                    b"cairo_int_status_t _cairo_xcb_render_compositor_fill(const cairo_compositor_t *, cairo_composite_rectangles_t *, const cairo_path_fixed_t *, cairo_fill_rule_t, double, cairo_antialias_t)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_xcb_surface_render_glyphs_via_mask(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut extents: *mut cairo_composite_rectangles_t,
) -> cairo_status_t {
    let mut image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut content: cairo_content_t = 0 as cairo_content_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut clip: *mut cairo_clip_t = 0 as *mut cairo_clip_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    content = CAIRO_CONTENT_ALPHA;
    if (*scaled_font).options.antialias as libc::c_uint
        == CAIRO_ANTIALIAS_SUBPIXEL as libc::c_int as libc::c_uint
    {
        content = CAIRO_CONTENT_COLOR_ALPHA;
    }
    x = (*extents).bounded.x;
    y = (*extents).bounded.y;
    image = _cairo_xcb_surface_create_similar_image(
        dst as *mut libc::c_void,
        _cairo_format_from_content(content),
        (*extents).bounded.width,
        (*extents).bounded.height,
    );
    if (*image).status as u64 != 0 {
        return (*image).status;
    }
    clip = _cairo_clip_copy_region((*extents).clip);
    status = _cairo_surface_offset_glyphs(
        image,
        x,
        y,
        CAIRO_OPERATOR_ADD,
        &_cairo_pattern_white.base,
        scaled_font,
        glyphs,
        num_glyphs,
        clip,
    );
    _cairo_clip_destroy(clip);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        let mut mask: cairo_surface_pattern_t = cairo_surface_pattern_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *const libc::c_char as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *const _cairo_list as *mut _cairo_list,
                    prev: 0 as *const _cairo_list as *mut _cairo_list,
                },
                type_0: CAIRO_PATTERN_TYPE_SOLID,
                filter: CAIRO_FILTER_FAST,
                extend: CAIRO_EXTEND_NONE,
                has_component_alpha: 0,
                is_userfont_foreground: 0,
                matrix: cairo_matrix_t {
                    xx: 0.,
                    yx: 0.,
                    xy: 0.,
                    yy: 0.,
                    x0: 0.,
                    y0: 0.,
                },
                opacity: 0.,
            },
            surface: 0 as *mut cairo_surface_t,
        };
        _cairo_pattern_init_for_surface(&mut mask, image);
        mask.base.filter = CAIRO_FILTER_NEAREST;
        if content as libc::c_uint & CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
            != 0
        {
            mask.base.has_component_alpha = 1 as libc::c_int;
        }
        cairo_matrix_init_translate(
            &mut mask.base.matrix,
            -x as libc::c_double,
            -y as libc::c_double,
        );
        status = _clip_and_composite(
            dst,
            op,
            source,
            Some(
                _composite_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_xcb_surface_t,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        libc::c_int,
                        libc::c_int,
                        *const cairo_rectangle_int_t,
                        *mut cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            None,
            &mut mask.base as *mut cairo_pattern_t as *mut libc::c_void,
            extents,
            need_bounded_clip(extents) as libc::c_uint,
        );
        _cairo_pattern_fini(&mut mask.base);
    }
    cairo_surface_finish(image);
    cairo_surface_destroy(image);
    return status;
}
unsafe extern "C" fn _can_composite_glyphs(
    mut dst: *mut cairo_xcb_surface_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: *mut libc::c_int,
) -> cairo_status_t {
    let mut bbox_cache: [cairo_box_t; 64] = [cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    }; 64];
    let mut glyph_cache: [libc::c_ulong; 64] = [0; 64];
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut glyphs_end: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    let mut valid_glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
    let max_glyph_size: libc::c_int = ((*(*dst).connection).maximum_request_length)
        .wrapping_sub(64 as libc::c_int as libc::c_uint) as libc::c_int;
    memset(
        glyph_cache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_ulong; 64]>() as libc::c_ulong,
    );
    glyph_cache[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ulong;
    valid_glyphs = glyphs;
    glyphs_end = glyphs.offset(*num_glyphs as isize);
    while glyphs != glyphs_end {
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        let mut glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        let mut bbox: *mut cairo_box_t = 0 as *mut cairo_box_t;
        let mut width: libc::c_int = 0;
        let mut height: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        g = ((*glyphs).index)
            .wrapping_rem(
                (::std::mem::size_of::<[libc::c_ulong; 64]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                    ) as libc::c_int as libc::c_ulong,
            ) as libc::c_int;
        if glyph_cache[g as usize] != (*glyphs).index {
            status = _cairo_scaled_glyph_lookup(
                scaled_font,
                (*glyphs).index,
                CAIRO_SCALED_GLYPH_INFO_METRICS,
                0 as *const cairo_color_t,
                &mut glyph,
            ) as cairo_status_t;
            if status as u64 != 0 {
                break;
            }
            glyph_cache[g as usize] = (*glyphs).index;
            bbox_cache[g as usize] = (*glyph).bbox;
        }
        bbox = &mut *bbox_cache.as_mut_ptr().offset(g as isize) as *mut cairo_box_t;
        x1 = _cairo_fixed_to_double((*bbox).p1.x);
        y1 = _cairo_fixed_to_double((*bbox).p1.y);
        y2 = _cairo_fixed_to_double((*bbox).p2.y);
        x2 = _cairo_fixed_to_double((*bbox).p2.x);
        if (*glyphs).x + x2 <= (*extents).x as libc::c_double
            || (*glyphs).y + y2 <= (*extents).y as libc::c_double
            || (*glyphs).x + x1 >= ((*extents).x + (*extents).width) as libc::c_double
            || (*glyphs).y + y1 >= ((*extents).y + (*extents).height) as libc::c_double
        {
            *num_glyphs -= 1;
        } else {
            width = _cairo_fixed_integer_ceil((*bbox).p2.x - (*bbox).p1.x);
            height = _cairo_fixed_integer_ceil((*bbox).p2.y - (*bbox).p1.y);
            len = ((((32 as libc::c_int * width + 7 as libc::c_int) / 8 as libc::c_int)
                as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_neg())
                .wrapping_mul(height as libc::c_ulong) as libc::c_int;
            if len >= max_glyph_size {
                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                break;
            } else if (*glyphs).x > 32767 as libc::c_int as libc::c_double
                || (*glyphs).y > 32767 as libc::c_int as libc::c_double
                || ((*glyphs).x - (*extents).x as libc::c_double)
                    < (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                || ((*glyphs).y - (*extents).y as libc::c_double)
                    < (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_double
            {
                status = CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
                break;
            } else {
                if valid_glyphs != glyphs {
                    *valid_glyphs = *glyphs;
                }
                valid_glyphs = valid_glyphs.offset(1);
            }
        }
        glyphs = glyphs.offset(1);
    }
    if valid_glyphs != glyphs {
        while glyphs != glyphs_end {
            *valid_glyphs = *glyphs;
            valid_glyphs = valid_glyphs.offset(1);
            glyphs = glyphs.offset(1);
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_xcb_font_destroy(mut font: *mut cairo_xcb_font_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < NUM_GLYPHSETS as libc::c_int {
        let mut info: *mut cairo_xcb_font_glyphset_info_t = 0
            as *mut cairo_xcb_font_glyphset_info_t;
        info = &mut *((*font).glyphset_info).as_mut_ptr().offset(i as isize)
            as *mut cairo_xcb_font_glyphset_info_t;
        free((*info).pending_free_glyphs as *mut libc::c_void);
        i += 1;
    }
    cairo_list_del(&mut (*font).base.link);
    cairo_list_del(&mut (*font).link);
    _cairo_xcb_connection_destroy((*font).connection);
    free(font as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xcb_font_fini(
    mut abstract_private: *mut cairo_scaled_font_private_t,
    mut scaled_font: *mut cairo_scaled_font_t,
) {
    let mut font_private: *mut cairo_xcb_font_t = abstract_private
        as *mut cairo_xcb_font_t;
    let mut connection: *mut cairo_xcb_connection_t = 0 as *mut cairo_xcb_connection_t;
    let mut have_connection: cairo_bool_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    connection = (*font_private).connection;
    status = _cairo_xcb_connection_acquire(connection);
    have_connection = (status as libc::c_uint
        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < NUM_GLYPHSETS as libc::c_int {
        let mut info: *mut cairo_xcb_font_glyphset_info_t = 0
            as *mut cairo_xcb_font_glyphset_info_t;
        info = &mut *((*font_private).glyphset_info).as_mut_ptr().offset(i as isize)
            as *mut cairo_xcb_font_glyphset_info_t;
        if (*info).glyphset != 0
            && status as libc::c_uint
                == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            _cairo_xcb_connection_render_free_glyph_set(connection, (*info).glyphset);
        }
        i += 1;
    }
    if have_connection != 0 {
        _cairo_xcb_connection_release(connection);
    }
    _cairo_xcb_font_destroy(font_private);
}
unsafe extern "C" fn _cairo_xcb_font_create(
    mut connection: *mut cairo_xcb_connection_t,
    mut font: *mut cairo_scaled_font_t,
) -> *mut cairo_xcb_font_t {
    let mut priv_0: *mut cairo_xcb_font_t = 0 as *mut cairo_xcb_font_t;
    let mut i: libc::c_int = 0;
    priv_0 = (if ::std::mem::size_of::<cairo_xcb_font_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xcb_font_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xcb_font_t;
    if priv_0.is_null() {
        return 0 as *mut cairo_xcb_font_t;
    }
    _cairo_scaled_font_attach_private(
        font,
        &mut (*priv_0).base,
        connection as *const libc::c_void,
        Some(
            _cairo_xcb_font_fini
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_private_t,
                    *mut cairo_scaled_font_t,
                ) -> (),
        ),
    );
    let ref mut fresh38 = (*priv_0).scaled_font;
    *fresh38 = font;
    let ref mut fresh39 = (*priv_0).connection;
    *fresh39 = _cairo_xcb_connection_reference(connection);
    cairo_list_add(&mut (*priv_0).link, &mut (*connection).fonts);
    i = 0 as libc::c_int;
    while i < NUM_GLYPHSETS as libc::c_int {
        let mut info: *mut cairo_xcb_font_glyphset_info_t = &mut *((*priv_0)
            .glyphset_info)
            .as_mut_ptr()
            .offset(i as isize) as *mut cairo_xcb_font_glyphset_info_t;
        match i {
            0 => {
                (*info).format = CAIRO_FORMAT_ARGB32;
            }
            1 => {
                (*info).format = CAIRO_FORMAT_A8;
            }
            2 => {
                (*info).format = CAIRO_FORMAT_A1;
            }
            _ => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-xcb-surface-render.c\0" as *const u8
                            as *const libc::c_char,
                        4161 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 90],
                            &[libc::c_char; 90],
                        >(
                            b"cairo_xcb_font_t *_cairo_xcb_font_create(cairo_xcb_connection_t *, cairo_scaled_font_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
        }
        (*info).xrender_format = 0 as libc::c_int as xcb_render_pictformat_t;
        (*info).glyphset = 0 as libc::c_long as xcb_render_glyphset_t;
        let ref mut fresh40 = (*info).pending_free_glyphs;
        *fresh40 = 0 as *mut cairo_xcb_font_glyphset_free_glyphs_t;
        i += 1;
    }
    return priv_0;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_font_close(mut font: *mut cairo_xcb_font_t) {
    let mut scaled_font: *mut cairo_scaled_font_t = 0 as *mut cairo_scaled_font_t;
    scaled_font = (*font).scaled_font;
    _cairo_scaled_font_reset_cache(scaled_font);
    _cairo_xcb_font_destroy(font);
}
unsafe extern "C" fn _cairo_xcb_render_free_glyphs(
    mut connection: *mut cairo_xcb_connection_t,
    mut to_free: *mut cairo_xcb_font_glyphset_free_glyphs_t,
) {
    _cairo_xcb_connection_render_free_glyphs(
        connection,
        (*to_free).glyphset,
        (*to_free).glyph_count as uint32_t,
        ((*to_free).glyph_indices).as_mut_ptr(),
    );
}
unsafe extern "C" fn _cairo_xcb_get_glyphset_index_for_format(
    mut format: cairo_format_t,
) -> libc::c_int {
    if format as libc::c_int == CAIRO_FORMAT_A8 as libc::c_int {
        return GLYPHSET_INDEX_A8 as libc::c_int;
    }
    if format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {
        return GLYPHSET_INDEX_A1 as libc::c_int;
    }
    if format as libc::c_int == CAIRO_FORMAT_ARGB32 as libc::c_int {} else {
        __assert_fail(
            b"format == CAIRO_FORMAT_ARGB32\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-xcb-surface-render.c\0" as *const u8 as *const libc::c_char,
            4202 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"int _cairo_xcb_get_glyphset_index_for_format(cairo_format_t)\0"))
                .as_ptr(),
        );
    }
    return GLYPHSET_INDEX_ARGB32 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_xcb_font_get(
    mut c: *const cairo_xcb_connection_t,
    mut font: *mut cairo_scaled_font_t,
) -> *mut cairo_xcb_font_t {
    return _cairo_scaled_font_find_private(font, c as *const libc::c_void)
        as *mut cairo_xcb_font_t;
}
unsafe extern "C" fn _cairo_xcb_scaled_font_get_glyphset_info_for_format(
    mut c: *mut cairo_xcb_connection_t,
    mut font: *mut cairo_scaled_font_t,
    mut format: cairo_format_t,
) -> *mut cairo_xcb_font_glyphset_info_t {
    let mut priv_0: *mut cairo_xcb_font_t = 0 as *mut cairo_xcb_font_t;
    let mut info: *mut cairo_xcb_font_glyphset_info_t = 0
        as *mut cairo_xcb_font_glyphset_info_t;
    let mut glyphset_index: libc::c_int = 0;
    glyphset_index = _cairo_xcb_get_glyphset_index_for_format(format);
    priv_0 = _cairo_xcb_font_get(c, font);
    if priv_0.is_null() {
        priv_0 = _cairo_xcb_font_create(c, font);
        if priv_0.is_null() {
            return 0 as *mut cairo_xcb_font_glyphset_info_t;
        }
    }
    info = &mut *((*priv_0).glyphset_info).as_mut_ptr().offset(glyphset_index as isize)
        as *mut cairo_xcb_font_glyphset_info_t;
    if (*info).glyphset as libc::c_long == 0 as libc::c_long {
        (*info).glyphset = xcb_generate_id((*c).xcb_connection);
        (*info).xrender_format = (*c).standard_formats[(*info).format as usize];
        _cairo_xcb_connection_render_create_glyph_set(
            c,
            (*info).glyphset,
            (*info).xrender_format,
        );
    }
    return info;
}
unsafe extern "C" fn _cairo_xcb_glyphset_info_has_pending_free_glyph(
    mut info: *mut cairo_xcb_font_glyphset_info_t,
    mut glyph_index: libc::c_ulong,
) -> cairo_bool_t {
    if !((*info).pending_free_glyphs).is_null() {
        let mut to_free: *mut cairo_xcb_font_glyphset_free_glyphs_t = 0
            as *mut cairo_xcb_font_glyphset_free_glyphs_t;
        let mut i: libc::c_int = 0;
        to_free = (*info).pending_free_glyphs;
        i = 0 as libc::c_int;
        while i < (*to_free).glyph_count {
            if (*to_free).glyph_indices[i as usize] as libc::c_ulong == glyph_index {
                let ref mut fresh41 = (*to_free).glyph_count;
                *fresh41 -= 1;
                memmove(
                    &mut *((*to_free).glyph_indices).as_mut_ptr().offset(i as isize)
                        as *mut xcb_render_glyph_t as *mut libc::c_void,
                    &mut *((*to_free).glyph_indices)
                        .as_mut_ptr()
                        .offset((i + 1 as libc::c_int) as isize)
                        as *mut xcb_render_glyph_t as *const libc::c_void,
                    (((*to_free).glyph_count - i) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<xcb_render_glyph_t>() as libc::c_ulong,
                        ),
                );
                return 1 as libc::c_int;
            }
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_xcb_scaled_font_get_glyphset_info_for_pending_free_glyph(
    mut c: *mut cairo_xcb_connection_t,
    mut font: *mut cairo_scaled_font_t,
    mut glyph_index: libc::c_ulong,
    mut surface: *mut cairo_image_surface_t,
) -> *mut cairo_xcb_font_glyphset_info_t {
    let mut priv_0: *mut cairo_xcb_font_t = 0 as *mut cairo_xcb_font_t;
    let mut i: libc::c_int = 0;
    priv_0 = _cairo_xcb_font_get(c, font);
    if priv_0.is_null() {
        return 0 as *mut cairo_xcb_font_glyphset_info_t;
    }
    if !surface.is_null() {
        i = _cairo_xcb_get_glyphset_index_for_format((*surface).format);
        if _cairo_xcb_glyphset_info_has_pending_free_glyph(
            &mut *((*priv_0).glyphset_info).as_mut_ptr().offset(i as isize),
            glyph_index,
        ) != 0
        {
            return &mut *((*priv_0).glyphset_info).as_mut_ptr().offset(i as isize)
                as *mut cairo_xcb_font_glyphset_info_t;
        }
    } else {
        i = 0 as libc::c_int;
        while i < NUM_GLYPHSETS as libc::c_int {
            if _cairo_xcb_glyphset_info_has_pending_free_glyph(
                &mut *((*priv_0).glyphset_info).as_mut_ptr().offset(i as isize),
                glyph_index,
            ) != 0
            {
                return &mut *((*priv_0).glyphset_info).as_mut_ptr().offset(i as isize)
                    as *mut cairo_xcb_font_glyphset_info_t;
            }
            i += 1;
        }
    }
    return 0 as *mut cairo_xcb_font_glyphset_info_t;
}
unsafe extern "C" fn _cairo_xcb_glyph_fini(
    mut glyph_private: *mut cairo_scaled_glyph_private_t,
    mut glyph: *mut cairo_scaled_glyph_t,
    mut font: *mut cairo_scaled_font_t,
) {
    let mut priv_0: *mut cairo_xcb_glyph_private_t = glyph_private
        as *mut cairo_xcb_glyph_private_t;
    if (*font).finished() == 0 {
        let mut info: *mut cairo_xcb_font_glyphset_info_t = (*priv_0).glyphset;
        let mut to_free: *mut cairo_xcb_font_glyphset_free_glyphs_t = 0
            as *mut cairo_xcb_font_glyphset_free_glyphs_t;
        let mut font_private: *mut cairo_xcb_font_t = 0 as *mut cairo_xcb_font_t;
        font_private = _cairo_xcb_font_get(
            (*glyph_private).key as *const cairo_xcb_connection_t,
            font,
        );
        if !font_private.is_null() {} else {
            __assert_fail(
                b"font_private\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                4326 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 106],
                    &[libc::c_char; 106],
                >(
                    b"void _cairo_xcb_glyph_fini(cairo_scaled_glyph_private_t *, cairo_scaled_glyph_t *, cairo_scaled_font_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        to_free = (*info).pending_free_glyphs;
        if !to_free.is_null()
            && (*to_free).glyph_count
                == (::std::mem::size_of::<[xcb_render_glyph_t; 128]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<xcb_render_glyph_t>() as libc::c_ulong,
                    ) as libc::c_int
        {
            _cairo_xcb_render_free_glyphs((*font_private).connection, to_free);
            let ref mut fresh42 = (*info).pending_free_glyphs;
            *fresh42 = 0 as *mut cairo_xcb_font_glyphset_free_glyphs_t;
            to_free = *fresh42;
        }
        if to_free.is_null() {
            to_free = (if ::std::mem::size_of::<cairo_xcb_font_glyphset_free_glyphs_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(
                    ::std::mem::size_of::<cairo_xcb_font_glyphset_free_glyphs_t>()
                        as libc::c_ulong,
                )
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_xcb_font_glyphset_free_glyphs_t;
            if to_free.is_null() {
                let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                return;
            }
            (*to_free).glyphset = (*info).glyphset;
            (*to_free).glyph_count = 0 as libc::c_int;
            let ref mut fresh43 = (*info).pending_free_glyphs;
            *fresh43 = to_free;
        }
        let ref mut fresh44 = (*to_free).glyph_count;
        let fresh45 = *fresh44;
        *fresh44 = *fresh44 + 1;
        (*to_free)
            .glyph_indices[fresh45
            as usize] = ((*glyph).hash_entry.hash
            & 0xffffff as libc::c_int as libc::c_ulong) as xcb_render_glyph_t;
    }
    cairo_list_del(&mut (*glyph_private).link);
    free(glyph_private as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_xcb_glyph_attach(
    mut c: *mut cairo_xcb_connection_t,
    mut glyph: *mut cairo_scaled_glyph_t,
    mut info: *mut cairo_xcb_font_glyphset_info_t,
) -> cairo_status_t {
    let mut priv_0: *mut cairo_xcb_glyph_private_t = 0 as *mut cairo_xcb_glyph_private_t;
    priv_0 = (if ::std::mem::size_of::<cairo_xcb_glyph_private_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_xcb_glyph_private_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_xcb_glyph_private_t;
    if priv_0.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    _cairo_scaled_glyph_attach_private(
        glyph,
        &mut (*priv_0).base,
        c as *const libc::c_void,
        Some(
            _cairo_xcb_glyph_fini
                as unsafe extern "C" fn(
                    *mut cairo_scaled_glyph_private_t,
                    *mut cairo_scaled_glyph_t,
                    *mut cairo_scaled_font_t,
                ) -> (),
        ),
    );
    let ref mut fresh46 = (*priv_0).glyphset;
    *fresh46 = info;
    let ref mut fresh47 = (*glyph).dev_private;
    *fresh47 = info as *mut libc::c_void;
    let ref mut fresh48 = (*glyph).dev_private_key;
    *fresh48 = c as *const libc::c_void;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_xcb_surface_add_glyph(
    mut connection: *mut cairo_xcb_connection_t,
    mut font: *mut cairo_scaled_font_t,
    mut scaled_glyph_out: *mut *mut cairo_scaled_glyph_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut glyph_info: xcb_render_glyphinfo_t = xcb_render_glyphinfo_t {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        x_off: 0,
        y_off: 0,
    };
    let mut glyph_index: uint32_t = 0;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut scaled_glyph: *mut cairo_scaled_glyph_t = *scaled_glyph_out;
    let mut glyph_surface: *mut cairo_image_surface_t = (*scaled_glyph).surface;
    let mut already_had_glyph_surface: cairo_bool_t = 0;
    let mut info: *mut cairo_xcb_font_glyphset_info_t = 0
        as *mut cairo_xcb_font_glyphset_info_t;
    glyph_index = ((*scaled_glyph).hash_entry.hash
        & 0xffffff as libc::c_int as libc::c_ulong) as uint32_t;
    info = _cairo_xcb_scaled_font_get_glyphset_info_for_pending_free_glyph(
        connection,
        font,
        glyph_index as libc::c_ulong,
        glyph_surface,
    );
    if !info.is_null() {
        return _cairo_xcb_glyph_attach(connection, scaled_glyph, info);
    }
    if glyph_surface.is_null() {
        status = _cairo_scaled_glyph_lookup(
            font,
            glyph_index as libc::c_ulong,
            (CAIRO_SCALED_GLYPH_INFO_METRICS as libc::c_int
                | CAIRO_SCALED_GLYPH_INFO_SURFACE as libc::c_int)
                as cairo_scaled_glyph_info_t,
            0 as *const cairo_color_t,
            scaled_glyph_out,
        ) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        scaled_glyph = *scaled_glyph_out;
        glyph_surface = (*scaled_glyph).surface;
        already_had_glyph_surface = 0 as libc::c_int;
    } else {
        already_had_glyph_surface = 1 as libc::c_int;
    }
    info = _cairo_xcb_scaled_font_get_glyphset_info_for_format(
        connection,
        font,
        (*glyph_surface).format,
    );
    if info.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        if (*glyph_surface).format as libc::c_int != (*info).format as libc::c_int {
            glyph_surface = _cairo_image_surface_coerce_to_format(
                glyph_surface,
                (*info).format,
            );
            status = (*glyph_surface).base.status;
            if status as u64 != 0 {
                current_block = 8181715672544856967;
            } else {
                current_block = 10043043949733653460;
            }
        } else {
            current_block = 10043043949733653460;
        }
        match current_block {
            8181715672544856967 => {}
            _ => {
                glyph_info
                    .x = _cairo_lround((*glyph_surface).base.device_transform.x0)
                    as int16_t;
                glyph_info
                    .y = _cairo_lround((*glyph_surface).base.device_transform.y0)
                    as int16_t;
                glyph_info.width = (*glyph_surface).width as uint16_t;
                glyph_info.height = (*glyph_surface).height as uint16_t;
                glyph_info.x_off = (*scaled_glyph).x_advance;
                glyph_info.y_off = (*scaled_glyph).y_advance;
                data = (*glyph_surface).data;
                match _cairo_xcb_get_glyphset_index_for_format(
                    (*(*scaled_glyph).surface).format,
                ) {
                    2 => {
                        if _cairo_is_little_endian()
                            != ((*(*connection).root).bitmap_format_bit_order
                                as libc::c_int == XCB_IMAGE_ORDER_LSB_FIRST as libc::c_int)
                                as libc::c_int
                        {
                            let mut c: libc::c_int = ((*glyph_surface).stride
                                * (*glyph_surface).height as libc::c_long) as libc::c_int;
                            let mut d: *const uint8_t = 0 as *const uint8_t;
                            let mut new: *mut uint8_t = 0 as *mut uint8_t;
                            let mut n: *mut uint8_t = 0 as *mut uint8_t;
                            if c == 0 as libc::c_int {
                                current_block = 5028470053297453708;
                            } else {
                                new = (if c != 0 as libc::c_int {
                                    malloc(c as libc::c_ulong)
                                } else {
                                    0 as *mut libc::c_void
                                }) as *mut uint8_t;
                                if new.is_null() {
                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                    current_block = 8181715672544856967;
                                } else {
                                    n = new;
                                    d = data;
                                    loop {
                                        let fresh49 = d;
                                        d = d.offset(1);
                                        let mut b: uint8_t = *fresh49;
                                        b = ((b as libc::c_int) << 1 as libc::c_int
                                            & 0xaa as libc::c_int
                                            | b as libc::c_int >> 1 as libc::c_int
                                                & 0x55 as libc::c_int) as uint8_t;
                                        b = ((b as libc::c_int) << 2 as libc::c_int
                                            & 0xcc as libc::c_int
                                            | b as libc::c_int >> 2 as libc::c_int
                                                & 0x33 as libc::c_int) as uint8_t;
                                        b = ((b as libc::c_int) << 4 as libc::c_int
                                            & 0xf0 as libc::c_int
                                            | b as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                                            as uint8_t;
                                        let fresh50 = n;
                                        n = n.offset(1);
                                        *fresh50 = b;
                                        c -= 1;
                                        if !(c != 0) {
                                            break;
                                        }
                                    }
                                    data = new;
                                    current_block = 5028470053297453708;
                                }
                            }
                        } else {
                            current_block = 5028470053297453708;
                        }
                    }
                    1 => {
                        current_block = 5028470053297453708;
                    }
                    0 => {
                        if _cairo_is_little_endian()
                            != ((*(*connection).root).image_byte_order as libc::c_int
                                == XCB_IMAGE_ORDER_LSB_FIRST as libc::c_int) as libc::c_int
                        {
                            let mut c_0: libc::c_uint = ((*glyph_surface).stride
                                * (*glyph_surface).height as libc::c_long
                                / 4 as libc::c_int as libc::c_long) as libc::c_uint;
                            let mut d_0: *const uint32_t = 0 as *const uint32_t;
                            let mut new_0: *mut uint32_t = 0 as *mut uint32_t;
                            let mut n_0: *mut uint32_t = 0 as *mut uint32_t;
                            if c_0 == 0 as libc::c_int as libc::c_uint {
                                current_block = 5028470053297453708;
                            } else {
                                new_0 = (if (4 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(c_0) != 0 as libc::c_int as libc::c_uint
                                {
                                    malloc(
                                        (4 as libc::c_int as libc::c_uint).wrapping_mul(c_0)
                                            as libc::c_ulong,
                                    )
                                } else {
                                    0 as *mut libc::c_void
                                }) as *mut uint32_t;
                                if new_0.is_null() {
                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                                    current_block = 8181715672544856967;
                                } else {
                                    n_0 = new_0;
                                    d_0 = data as *mut uint32_t;
                                    loop {
                                        let fresh51 = n_0;
                                        n_0 = n_0.offset(1);
                                        *fresh51 = __bswap_32(*d_0);
                                        d_0 = d_0.offset(1);
                                        c_0 = c_0.wrapping_sub(1);
                                        if !(c_0 != 0) {
                                            break;
                                        }
                                    }
                                    data = new_0 as *mut uint8_t;
                                    current_block = 5028470053297453708;
                                }
                            }
                        } else {
                            current_block = 5028470053297453708;
                        }
                    }
                    _ => {
                        if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                        {} else {
                            __assert_fail(
                                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                                    as *const libc::c_char,
                                4524 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 118],
                                    &[libc::c_char; 118],
                                >(
                                    b"cairo_status_t _cairo_xcb_surface_add_glyph(cairo_xcb_connection_t *, cairo_scaled_font_t *, cairo_scaled_glyph_t **)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        current_block = 5028470053297453708;
                    }
                }
                match current_block {
                    8181715672544856967 => {}
                    _ => {
                        _cairo_xcb_connection_render_add_glyphs(
                            connection,
                            (*info).glyphset,
                            1 as libc::c_int as uint32_t,
                            &mut glyph_index,
                            &mut glyph_info,
                            ((*glyph_surface).stride
                                * (*glyph_surface).height as libc::c_long) as uint32_t,
                            data,
                        );
                        if data != (*glyph_surface).data {
                            free(data as *mut libc::c_void);
                        }
                        status = _cairo_xcb_glyph_attach(connection, scaled_glyph, info);
                    }
                }
            }
        }
    }
    if glyph_surface != (*scaled_glyph).surface {
        cairo_surface_destroy(&mut (*glyph_surface).base);
    }
    if already_had_glyph_surface == 0 {
        _cairo_scaled_glyph_set_surface(
            scaled_glyph,
            font,
            0 as *mut cairo_image_surface_t,
        );
    }
    return status;
}
unsafe extern "C" fn _emit_glyphs_chunk(
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut src: *mut cairo_xcb_picture_t,
    mut glyphs: *mut cairo_xcb_glyph_t,
    mut num_glyphs: libc::c_int,
    mut width: libc::c_int,
    mut estimated_req_size: libc::c_int,
    mut info: *mut cairo_xcb_font_glyphset_info_t,
    mut mask_format: xcb_render_pictformat_t,
) -> cairo_status_t {
    let mut composite_text_func: cairo_xcb_render_composite_text_func_t = None;
    let mut stack_buf: [uint8_t; 2048] = [0; 2048];
    let mut buf: *mut uint8_t = stack_buf.as_mut_ptr();
    let mut elt: *mut x_glyph_elt_t = 0 as *mut x_glyph_elt_t;
    let mut len: uint32_t = 0;
    let mut i: libc::c_int = 0;
    if estimated_req_size
        > (::std::mem::size_of::<[uint8_t; 2048]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            as libc::c_int
    {
        buf = (if estimated_req_size != 0 as libc::c_int {
            malloc(estimated_req_size as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut uint8_t;
        if buf.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
    }
    len = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while i < num_glyphs {
        if i & 127 as libc::c_int == 0 as libc::c_int
            || (*glyphs.offset(i as isize)).i.x != 0
            || (*glyphs.offset(i as isize)).i.y != 0
        {
            if len & 3 as libc::c_int as libc::c_uint != 0 {
                len = (len as libc::c_uint)
                    .wrapping_add(
                        (4 as libc::c_int as libc::c_uint)
                            .wrapping_sub(len & 3 as libc::c_int as libc::c_uint),
                    ) as uint32_t as uint32_t;
            }
            elt = buf.offset(len as isize) as *mut x_glyph_elt_t;
            (*elt).len = 0 as libc::c_int as uint8_t;
            (*elt).deltax = (*glyphs.offset(i as isize)).i.x as int16_t;
            (*elt).deltay = (*glyphs.offset(i as isize)).i.y as int16_t;
            len = (len as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<x_glyph_elt_t>() as libc::c_ulong)
                as uint32_t as uint32_t;
        }
        match width {
            1 => {
                *buf
                    .offset(
                        len as isize,
                    ) = (*glyphs.offset(i as isize)).index as uint8_t;
            }
            2 => {
                *(buf.offset(len as isize)
                    as *mut uint16_t) = (*glyphs.offset(i as isize)).index as uint16_t;
            }
            4 | _ => {
                *(buf.offset(len as isize)
                    as *mut uint32_t) = (*glyphs.offset(i as isize)).index as uint32_t;
            }
        }
        len = (len as libc::c_uint).wrapping_add(width as libc::c_uint) as uint32_t
            as uint32_t;
        let ref mut fresh52 = (*elt).len;
        *fresh52 = (*fresh52).wrapping_add(1);
        i += 1;
    }
    if len & 3 as libc::c_int as libc::c_uint != 0 {
        len = (len as libc::c_uint)
            .wrapping_add(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_sub(len & 3 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
    }
    match width {
        1 => {
            composite_text_func = Some(
                _cairo_xcb_connection_render_composite_glyphs_8
                    as unsafe extern "C" fn(
                        *mut cairo_xcb_connection_t,
                        uint8_t,
                        xcb_render_picture_t,
                        xcb_render_picture_t,
                        xcb_render_pictformat_t,
                        xcb_render_glyphset_t,
                        int16_t,
                        int16_t,
                        uint32_t,
                        *mut uint8_t,
                    ) -> (),
            );
        }
        2 => {
            composite_text_func = Some(
                _cairo_xcb_connection_render_composite_glyphs_16
                    as unsafe extern "C" fn(
                        *mut cairo_xcb_connection_t,
                        uint8_t,
                        xcb_render_picture_t,
                        xcb_render_picture_t,
                        xcb_render_pictformat_t,
                        xcb_render_glyphset_t,
                        int16_t,
                        int16_t,
                        uint32_t,
                        *mut uint8_t,
                    ) -> (),
            );
        }
        4 | _ => {
            composite_text_func = Some(
                _cairo_xcb_connection_render_composite_glyphs_32
                    as unsafe extern "C" fn(
                        *mut cairo_xcb_connection_t,
                        uint8_t,
                        xcb_render_picture_t,
                        xcb_render_picture_t,
                        xcb_render_pictformat_t,
                        xcb_render_glyphset_t,
                        int16_t,
                        int16_t,
                        uint32_t,
                        *mut uint8_t,
                    ) -> (),
            );
        }
    }
    composite_text_func
        .expect(
            "non-null function pointer",
        )(
        (*dst).connection,
        _render_operator(op) as uint8_t,
        (*src).picture,
        (*dst).picture,
        mask_format,
        (*info).glyphset,
        ((*src).x + (*glyphs.offset(0 as libc::c_int as isize)).i.x) as int16_t,
        ((*src).y + (*glyphs.offset(0 as libc::c_int as isize)).i.y) as int16_t,
        len,
        buf,
    );
    if buf != stack_buf.as_mut_ptr() {
        free(buf as *mut libc::c_void);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _composite_glyphs(
    mut closure: *mut libc::c_void,
    mut dst: *mut cairo_xcb_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut dst_x: libc::c_int,
    mut dst_y: libc::c_int,
    mut extents: *const cairo_rectangle_int_t,
    mut clip: *mut cairo_clip_t,
) -> cairo_int_status_t {
    let mut info: *mut composite_glyphs_info_t = closure as *mut composite_glyphs_info_t;
    let mut glyph_cache: [*mut cairo_scaled_glyph_t; 64] = [0
        as *mut cairo_scaled_glyph_t; 64];
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut x: cairo_fixed_t = 0 as libc::c_int;
    let mut y: cairo_fixed_t = 0 as libc::c_int;
    let mut glyphset_info: *mut cairo_xcb_font_glyphset_info_t = 0
        as *mut cairo_xcb_font_glyphset_info_t;
    let mut this_glyphset_info: *mut cairo_xcb_font_glyphset_info_t = 0
        as *mut cairo_xcb_font_glyphset_info_t;
    let max_request_size: libc::c_uint = ((*(*dst).connection).maximum_request_length)
        .wrapping_sub(64 as libc::c_int as libc::c_uint);
    let mut src: *mut cairo_xcb_picture_t = 0 as *mut cairo_xcb_picture_t;
    let mut max_index: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut width: libc::c_int = 1 as libc::c_int;
    let mut request_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    if (*dst).deferred_clear != 0 {
        status = _cairo_xcb_surface_clear(dst);
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
    }
    src = _cairo_xcb_picture_for_pattern(dst, pattern, extents);
    if (*src).base.status as u64 != 0 {
        return (*src).base.status as cairo_int_status_t;
    }
    memset(
        glyph_cache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut cairo_scaled_glyph_t; 64]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (*info).num_glyphs {
        let mut glyph: *mut cairo_scaled_glyph_t = 0 as *mut cairo_scaled_glyph_t;
        let mut glyph_index: libc::c_ulong = (*((*info).glyphs).offset(i as isize))
            .index;
        let mut cache_index: libc::c_int = glyph_index
            .wrapping_rem(
                (::std::mem::size_of::<[*mut cairo_scaled_glyph_t; 64]>()
                    as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut cairo_scaled_glyph_t>()
                            as libc::c_ulong,
                    ) as libc::c_int as libc::c_ulong,
            ) as libc::c_int;
        let mut old_width: libc::c_int = width;
        let mut this_x: libc::c_int = 0;
        let mut this_y: libc::c_int = 0;
        glyph = glyph_cache[cache_index as usize];
        if glyph.is_null()
            || (*glyph).hash_entry.hash & 0xffffff as libc::c_int as libc::c_ulong
                != glyph_index
        {
            status = _cairo_scaled_glyph_lookup(
                (*info).font,
                glyph_index,
                CAIRO_SCALED_GLYPH_INFO_METRICS,
                0 as *const cairo_color_t,
                &mut glyph,
            ) as cairo_status_t;
            if status as u64 != 0 {
                cairo_surface_destroy(&mut (*src).base);
                return status as cairo_int_status_t;
            }
            if (*glyph).dev_private_key != (*dst).connection as *const libc::c_void {
                status = _cairo_xcb_surface_add_glyph(
                    (*dst).connection,
                    (*info).font,
                    &mut glyph,
                );
                if status as u64 != 0 {
                    cairo_surface_destroy(&mut (*src).base);
                    return status as cairo_int_status_t;
                }
            }
            glyph_cache[cache_index as usize] = glyph;
        }
        this_x = _cairo_lround((*((*info).glyphs).offset(i as isize)).d.x) - dst_x;
        this_y = _cairo_lround((*((*info).glyphs).offset(i as isize)).d.y) - dst_y;
        this_glyphset_info = (*glyph).dev_private as *mut cairo_xcb_font_glyphset_info_t;
        if glyphset_info.is_null() {
            glyphset_info = this_glyphset_info;
        }
        if glyph_index > max_index {
            max_index = glyph_index;
            if max_index >= 65536 as libc::c_int as libc::c_ulong {
                width = 4 as libc::c_int;
            } else if max_index >= 256 as libc::c_int as libc::c_ulong {
                width = 2 as libc::c_int;
            }
            if width != old_width {
                request_size = request_size
                    .wrapping_add(((width - old_width) * i) as libc::c_uint);
            }
        }
        if request_size.wrapping_add(width as libc::c_uint) as libc::c_ulong
            > (max_request_size as libc::c_ulong)
                .wrapping_sub(
                    (::std::mem::size_of::<x_glyph_elt_t>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong),
                ) || this_x - x > 32767 as libc::c_int
            || this_x - x < -(32767 as libc::c_int) - 1 as libc::c_int
            || this_y - y > 32767 as libc::c_int
            || this_y - y < -(32767 as libc::c_int) - 1 as libc::c_int
            || this_glyphset_info != glyphset_info
        {
            status = _emit_glyphs_chunk(
                dst,
                op,
                src,
                (*info).glyphs,
                i,
                old_width,
                request_size as libc::c_int,
                glyphset_info,
                if (*info).use_mask != 0 {
                    (*glyphset_info).xrender_format
                } else {
                    0 as libc::c_int as libc::c_uint
                },
            );
            if status as u64 != 0 {
                cairo_surface_destroy(&mut (*src).base);
                return status as cairo_int_status_t;
            }
            let ref mut fresh53 = (*info).glyphs;
            *fresh53 = (*fresh53).offset(i as isize);
            (*info).num_glyphs -= i;
            i = 0 as libc::c_int;
            max_index = (*((*info).glyphs).offset(0 as libc::c_int as isize)).index;
            width = if max_index < 256 as libc::c_int as libc::c_ulong {
                1 as libc::c_int
            } else if max_index < 65536 as libc::c_int as libc::c_ulong {
                2 as libc::c_int
            } else {
                4 as libc::c_int
            };
            request_size = 0 as libc::c_int as libc::c_uint;
            y = 0 as libc::c_int;
            x = y;
            glyphset_info = this_glyphset_info;
        }
        (*((*info).glyphs).offset(i as isize)).i.x = this_x - x;
        (*((*info).glyphs).offset(i as isize)).i.y = this_y - y;
        if i & 127 as libc::c_int == 0 as libc::c_int
            || (*((*info).glyphs).offset(i as isize)).i.x != 0
            || (*((*info).glyphs).offset(i as isize)).i.y != 0
        {
            request_size = (request_size as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<x_glyph_elt_t>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint as libc::c_uint;
        }
        x = this_x + (*glyph).x_advance as libc::c_int;
        y = this_y + (*glyph).y_advance as libc::c_int;
        request_size = request_size.wrapping_add(width as libc::c_uint);
        i += 1;
    }
    if i != 0 {
        status = _emit_glyphs_chunk(
            dst,
            op,
            src,
            (*info).glyphs,
            i,
            width,
            request_size as libc::c_int,
            glyphset_info,
            if (*info).use_mask != 0 {
                (*glyphset_info).xrender_format
            } else {
                0 as libc::c_int as libc::c_uint
            },
        );
    }
    cairo_surface_destroy(&mut (*src).base);
    return status as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_xcb_render_compositor_glyphs(
    mut compositor: *const cairo_compositor_t,
    mut composite: *mut cairo_composite_rectangles_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut overlap: cairo_bool_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_xcb_surface_t = (*composite).surface
        as *mut cairo_xcb_surface_t;
    let mut op: cairo_operator_t = (*composite).op;
    let mut source: *mut cairo_pattern_t = &mut (*composite).source_pattern.base;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if _operator_is_supported((*(*surface).connection).flags, op) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*(*surface).connection).flags
        & (CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS as libc::c_int
            | CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int) as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = CAIRO_INT_STATUS_UNSUPPORTED;
    if (*(*surface).connection).flags
        & CAIRO_XCB_RENDER_HAS_COMPOSITE_GLYPHS as libc::c_int as libc::c_uint != 0
    {
        _cairo_scaled_font_freeze_cache(scaled_font);
        status = _can_composite_glyphs(
            surface,
            &mut (*composite).bounded,
            scaled_font,
            glyphs,
            &mut num_glyphs,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            let mut info: composite_glyphs_info_t = composite_glyphs_info_t {
                font: 0 as *mut cairo_scaled_font_t,
                glyphs: 0 as *mut cairo_xcb_glyph_t,
                num_glyphs: 0,
                use_mask: 0,
            };
            let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            info.font = scaled_font;
            info.glyphs = glyphs as *mut cairo_xcb_glyph_t;
            info.num_glyphs = num_glyphs;
            info
                .use_mask = (overlap != 0 || (*composite).is_bounded == 0
                || _cairo_clip_is_region((*composite).clip) == 0) as libc::c_int;
            if (*composite).mask.width > (*composite).unbounded.width
                || (*composite).mask.height > (*composite).unbounded.height
            {
                flags |= FORCE_CLIP_REGION as libc::c_int as libc::c_uint;
            }
            status = _clip_and_composite(
                surface,
                op,
                source,
                Some(
                    _composite_glyphs
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut cairo_xcb_surface_t,
                            cairo_operator_t,
                            *const cairo_pattern_t,
                            libc::c_int,
                            libc::c_int,
                            *const cairo_rectangle_int_t,
                            *mut cairo_clip_t,
                        ) -> cairo_int_status_t,
                ),
                None,
                &mut info as *mut composite_glyphs_info_t as *mut libc::c_void,
                composite,
                need_bounded_clip(composite) as libc::c_uint | flags,
            ) as cairo_int_status_t;
        }
        _cairo_scaled_font_thaw_cache(scaled_font);
    }
    if status as libc::c_uint
        == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        if (*(*surface).connection).flags
            & CAIRO_XCB_RENDER_HAS_COMPOSITE as libc::c_int as libc::c_uint != 0
        {} else {
            __assert_fail(
                b"surface->connection->flags & CAIRO_XCB_RENDER_HAS_COMPOSITE\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-xcb-surface-render.c\0" as *const u8
                    as *const libc::c_char,
                4874 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 174],
                    &[libc::c_char; 174],
                >(
                    b"cairo_int_status_t _cairo_xcb_render_compositor_glyphs(const cairo_compositor_t *, cairo_composite_rectangles_t *, cairo_scaled_font_t *, cairo_glyph_t *, int, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        status = _cairo_xcb_surface_render_glyphs_via_mask(
            surface,
            op,
            source,
            scaled_font,
            glyphs,
            num_glyphs,
            composite,
        ) as cairo_int_status_t;
    }
    return status;
}
