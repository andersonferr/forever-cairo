use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _cairo_scaled_font_subsets;
    fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t,
        antialias: cairo_antialias_t,
    );
    fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t,
        hint_style: cairo_hint_style_t,
    );
    fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t,
        hint_metrics: cairo_hint_metrics_t,
    );
    fn cairo_scaled_font_reference(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_scaled_font_t;
    fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_get_mime_data(
        surface: *mut cairo_surface_t,
        mime_type: *const libc::c_char,
        data: *mut *const libc::c_uchar,
        length: *mut libc::c_ulong,
    );
    fn cairo_surface_get_device_offset(
        surface: *mut cairo_surface_t,
        x_offset: *mut libc::c_double,
        y_offset: *mut libc::c_double,
    );
    fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> cairo_extend_t;
    fn cairo_matrix_init(
        matrix: *mut cairo_matrix_t,
        xx: libc::c_double,
        yx: libc::c_double,
        xy: libc::c_double,
        yy: libc::c_double,
        x0: libc::c_double,
        y0: libc::c_double,
    );
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
    fn cairo_matrix_scale(
        matrix: *mut cairo_matrix_t,
        sx: libc::c_double,
        sy: libc::c_double,
    );
    fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t,
        a: *const cairo_matrix_t,
        b: *const cairo_matrix_t,
    );
    fn cairo_matrix_transform_point(
        matrix: *const cairo_matrix_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_hash_table_lookup(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    ) -> *mut libc::c_void;
    fn _cairo_hash_table_destroy(hash_table: *mut cairo_hash_table_t);
    fn _cairo_hash_table_create(
        keys_equal: cairo_hash_keys_equal_func_t,
    ) -> *mut cairo_hash_table_t;
    fn _cairo_error(status: cairo_status_t) -> cairo_status_t;
    fn _cairo_surface_default_source(
        surface: *mut libc::c_void,
        extents: *mut cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_create_in_error(status: cairo_status_t) -> *mut cairo_surface_t;
    fn _cairo_box_to_doubles(
        box_0: *const cairo_box_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
    );
    fn _cairo_box_from_rectangle(
        box_0: *mut cairo_box_t,
        rectangle: *const cairo_rectangle_int_t,
    );
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
    static _cairo_unbounded_rectangle: cairo_rectangle_int_t;
    fn _cairo_rectangle_intersect(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_rectangle_union(
        dst: *mut cairo_rectangle_int_t,
        src: *const cairo_rectangle_int_t,
    );
    fn _cairo_hash_bytes(
        hash: uintptr_t,
        bytes: *const libc::c_void,
        length: libc::c_uint,
    ) -> uintptr_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_font_options_set_round_glyph_positions(
        options: *mut cairo_font_options_t,
        round: cairo_round_glyph_positions_t,
    );
    fn _cairo_path_fixed_init_copy(
        path: *mut cairo_path_fixed_t,
        other: *const cairo_path_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_fill_extents(
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        tolerance: libc::c_double,
        extents: *mut cairo_rectangle_int_t,
    );
    fn _cairo_path_fixed_stroke_extents(
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
        tolerance: libc::c_double,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_is_box(
        path: *const cairo_path_fixed_t,
        box_0: *mut cairo_box_t,
    ) -> cairo_bool_t;
    fn _cairo_surface_set_error(
        surface: *mut cairo_surface_t,
        status: cairo_int_status_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_surface_get_extents(
        surface: *mut cairo_surface_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_image_surface_create_with_content(
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn _cairo_image_analyze_transparency(
        image: *mut cairo_image_surface_t,
    ) -> cairo_image_transparency_t;
    fn _cairo_image_analyze_color(
        image: *mut cairo_image_surface_t,
    ) -> cairo_image_color_t;
    fn _cairo_matrix_transform_bounding_box(
        matrix: *const cairo_matrix_t,
        x1: *mut libc::c_double,
        y1: *mut libc::c_double,
        x2: *mut libc::c_double,
        y2: *mut libc::c_double,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_matrix_transform_bounding_box_fixed(
        matrix: *const cairo_matrix_t,
        bbox: *mut cairo_box_t,
        is_tight: *mut cairo_bool_t,
    );
    fn _cairo_matrix_compute_determinant(
        matrix: *const cairo_matrix_t,
    ) -> libc::c_double;
    fn _cairo_matrix_transformed_circle_major_axis(
        matrix: *const cairo_matrix_t,
        radius: libc::c_double,
    ) -> libc::c_double;
    fn _cairo_utf8_to_utf16(
        str: *const libc::c_char,
        len: libc::c_int,
        result: *mut *mut uint16_t,
        items_written: *mut libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_pdf_operators_init(
        pdf_operators: *mut cairo_pdf_operators_t,
        stream: *mut cairo_output_stream_t,
        cairo_to_pdf: *mut cairo_matrix_t,
        font_subsets: *mut cairo_scaled_font_subsets_t,
        ps: cairo_bool_t,
    );
    fn _cairo_pdf_operators_fini(
        pdf_operators: *mut cairo_pdf_operators_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_operators_set_font_subsets_callback(
        pdf_operators: *mut cairo_pdf_operators_t,
        use_font_subset: cairo_pdf_operators_use_font_subset_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_tag_parse_ccitt_params(
        attributes: *const libc::c_char,
        dest_attrs: *mut cairo_ccitt_params_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_stroke(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
    ) -> cairo_int_status_t;
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_pdf_operators_reset(pdf_operators: *mut cairo_pdf_operators_t);
    fn _cairo_pdf_operators_set_stream(
        pdf_operators: *mut cairo_pdf_operators_t,
        stream: *mut cairo_output_stream_t,
    );
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_pdf_interchange_begin_page_content(
        surface: *mut cairo_pdf_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_clip(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_int_status_t;
    fn _cairo_array_truncate(array: *mut cairo_array_t, num_elements: libc::c_uint);
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_pdf_interchange_init(
        surface: *mut cairo_pdf_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_enable_actual_text(
        pdf_operators: *mut cairo_pdf_operators_t,
        enable: cairo_bool_t,
    );
    fn _cairo_pdf_operators_flush(
        pdf_operators: *mut cairo_pdf_operators_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_interchange_set_custom_metadata(
        surface: *mut cairo_pdf_surface_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_array_copy_element(
        array: *const cairo_array_t,
        index: libc::c_uint,
        dst: *mut libc::c_void,
    );
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
    fn _cairo_pdf_interchange_set_metadata(
        surface: *mut cairo_pdf_surface_t,
        metadata: cairo_pdf_metadata_t,
        utf8: *const libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_array_sort(
        array: *const cairo_array_t,
        compar: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn _cairo_pdf_interchange_add_outline(
        surface: *mut cairo_pdf_surface_t,
        parent_id: libc::c_int,
        name: *const libc::c_char,
        dest: *const libc::c_char,
        flags: cairo_pdf_outline_flags_t,
        id: *mut libc::c_int,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_tag_end(
        surface: *mut cairo_pdf_surface_t,
        name: *const libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_tag_begin(
        surface: *mut cairo_pdf_surface_t,
        name: *const libc::c_char,
        attributes: *const libc::c_char,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_write_page_objects(
        surface: *mut cairo_pdf_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_end_page_content(
        surface: *mut cairo_pdf_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_fini(surface: *mut cairo_pdf_surface_t);
    fn _cairo_pdf_operators_show_text_glyphs(
        pdf_operators: *mut cairo_pdf_operators_t,
        utf8: *const libc::c_char,
        utf8_len: libc::c_int,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clusters: *const cairo_text_cluster_t,
        num_clusters: libc::c_int,
        cluster_flags: cairo_text_cluster_flags_t,
        scaled_font: *mut cairo_scaled_font_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_write_document_objects(
        surface: *mut cairo_pdf_surface_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_fill(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_fill_stroke(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_interchange_add_operation_extents(
        surface: *mut cairo_pdf_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> cairo_int_status_t;
    fn _cairo_surface_clipper_set_clip(
        clipper: *mut cairo_surface_clipper_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_init(
        clipper: *mut cairo_surface_clipper_t,
        intersect: cairo_surface_clipper_intersect_clip_path_func_t,
    );
    fn _cairo_surface_clipper_reset(clipper: *mut cairo_surface_clipper_t);
    fn _cairo_pattern_create_copy(
        pattern: *mut *mut cairo_pattern_t,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_is_opaque(
        pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
    ) -> cairo_bool_t;
    fn _cairo_pattern_is_constant_alpha(
        abstract_pattern: *const cairo_pattern_t,
        extents: *const cairo_rectangle_int_t,
        alpha: *mut libc::c_double,
    ) -> cairo_bool_t;
    fn _cairo_gradient_pattern_box_to_parameter(
        gradient: *const cairo_gradient_pattern_t,
        x0: libc::c_double,
        y0: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
        tolerance: libc::c_double,
        out_range: *mut libc::c_double,
    );
    fn _cairo_gradient_pattern_interpolate(
        gradient: *const cairo_gradient_pattern_t,
        t: libc::c_double,
        out_circle: *mut cairo_circle_double_t,
    );
    fn _cairo_pattern_alpha_range(
        pattern: *const cairo_pattern_t,
        out_min: *mut libc::c_double,
        out_max: *mut libc::c_double,
    );
    fn _cairo_pattern_get_ink_extents(
        pattern: *const cairo_pattern_t,
        extents: *mut cairo_rectangle_int_t,
    ) -> cairo_int_status_t;
    fn _cairo_raster_source_pattern_release(
        abstract_pattern: *const cairo_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_pdf_shading_init_color(
        shading: *mut cairo_pdf_shading_t,
        pattern: *const cairo_mesh_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_shading_init_alpha(
        shading: *mut cairo_pdf_shading_t,
        pattern: *const cairo_mesh_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_shading_fini(shading: *mut cairo_pdf_shading_t);
    fn _cairo_raster_source_pattern_acquire(
        abstract_pattern: *const cairo_pattern_t,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_analysis_surface_merge_status(
        status_a: cairo_int_status_t,
        status_b: cairo_int_status_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_paint(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_mask(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        mask: *const cairo_pattern_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_stroke(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_fill(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        path: *const cairo_path_fixed_t,
        clip: *const cairo_clip_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_init_for_glyphs(
        extents: *mut cairo_composite_rectangles_t,
        surface: *mut cairo_surface_t,
        op: cairo_operator_t,
        source: *const cairo_pattern_t,
        scaled_font: *mut cairo_scaled_font_t,
        glyphs: *mut cairo_glyph_t,
        num_glyphs: libc::c_int,
        clip: *const cairo_clip_t,
        overlap: *mut cairo_bool_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_intersect_source_extents(
        extents: *mut cairo_composite_rectangles_t,
        box_0: *const cairo_box_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_intersect_mask_extents(
        extents: *mut cairo_composite_rectangles_t,
        box_0: *const cairo_box_t,
    ) -> cairo_int_status_t;
    fn _cairo_composite_rectangles_can_reduce_clip(
        composite: *mut cairo_composite_rectangles_t,
        clip: *mut cairo_clip_t,
    ) -> cairo_bool_t;
    fn _cairo_composite_rectangles_fini(extents: *mut cairo_composite_rectangles_t);
    fn _cairo_default_context_create(target: *mut libc::c_void) -> *mut cairo_t;
    fn _cairo_image_info_get_jpeg_info(
        info: *mut cairo_image_info_t,
        data: *const libc::c_uchar,
        length: libc::c_ulong,
    ) -> cairo_int_status_t;
    fn _cairo_image_info_get_jpx_info(
        info: *mut cairo_image_info_t,
        data: *const libc::c_uchar,
        length: libc::c_ulong,
    ) -> cairo_int_status_t;
    fn _cairo_image_info_get_jbig2_info(
        info: *mut cairo_image_info_t,
        data: *const libc::c_uchar,
        length: libc::c_ulong,
    ) -> cairo_int_status_t;
    fn _cairo_recording_surface_replay_region(
        surface: *mut cairo_surface_t,
        surface_extents: *const cairo_rectangle_int_t,
        target: *mut cairo_surface_t,
        region: cairo_recording_region_type_t,
    ) -> cairo_status_t;
    fn _cairo_recording_surface_get_ink_bbox(
        surface: *mut cairo_recording_surface_t,
        bbox: *mut cairo_box_t,
        transform: *const cairo_matrix_t,
    ) -> cairo_status_t;
    fn _cairo_recording_surface_has_only_bilevel_alpha(
        surface: *mut cairo_recording_surface_t,
    ) -> cairo_bool_t;
    fn _cairo_recording_surface_has_only_op_over(
        surface: *mut cairo_recording_surface_t,
    ) -> cairo_bool_t;
    fn _cairo_output_stream_create(
        write_func: cairo_write_func_t,
        close_func: cairo_close_func_t,
        closure: *mut libc::c_void,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_destroy(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_write(
        stream: *mut cairo_output_stream_t,
        data: *const libc::c_void,
        length: size_t,
    );
    fn _cairo_output_stream_vprintf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        ap: ::std::ffi::VaList,
    );
    fn _cairo_output_stream_printf(
        stream: *mut cairo_output_stream_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _cairo_output_stream_print_matrix(
        stream: *mut cairo_output_stream_t,
        matrix: *const cairo_matrix_t,
    );
    fn _cairo_output_stream_get_position(
        stream: *mut cairo_output_stream_t,
    ) -> libc::c_longlong;
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_create_for_filename(
        filename: *const libc::c_char,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_copy(
        base: *mut cairo_output_stream_t,
        dest: *mut cairo_output_stream_t,
    );
    fn _cairo_memory_stream_length(stream: *mut cairo_output_stream_t) -> libc::c_int;
    fn _cairo_null_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_deflate_stream_create(
        output: *mut cairo_output_stream_t,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_paginated_surface_create(
        target: *mut cairo_surface_t,
        content: cairo_content_t,
        backend: *const cairo_paginated_surface_backend_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_paginated_surface_get_target(
        surface: *mut cairo_surface_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_surface_is_paginated(surface: *mut cairo_surface_t) -> cairo_bool_t;
    fn _cairo_paginated_surface_set_size(
        surface: *mut cairo_surface_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_subsets_create_composite() -> *mut cairo_scaled_font_subsets_t;
    fn _cairo_scaled_font_subsets_destroy(
        font_subsets: *mut cairo_scaled_font_subsets_t,
    );
    fn _cairo_scaled_font_subsets_enable_latin_subset(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        use_latin: cairo_bool_t,
    );
    fn _cairo_scaled_font_subsets_foreach_scaled(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        font_subset_callback: cairo_scaled_font_subset_callback_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_subsets_foreach_unscaled(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        font_subset_callback: cairo_scaled_font_subset_callback_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_scaled_font_subsets_foreach_user(
        font_subsets: *mut cairo_scaled_font_subsets_t,
        font_subset_callback: cairo_scaled_font_subset_callback_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_cff_subset_init(
        cff_subset: *mut cairo_cff_subset_t,
        name: *const libc::c_char,
        font_subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_status_t;
    fn _cairo_cff_subset_fini(cff_subset: *mut cairo_cff_subset_t);
    fn _cairo_cff_fallback_init(
        cff_subset: *mut cairo_cff_subset_t,
        name: *const libc::c_char,
        font_subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_status_t;
    fn _cairo_cff_fallback_fini(cff_subset: *mut cairo_cff_subset_t);
    fn _cairo_truetype_subset_init_pdf(
        truetype_subset: *mut cairo_truetype_subset_t,
        font_subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_status_t;
    fn _cairo_truetype_subset_fini(truetype_subset: *mut cairo_truetype_subset_t);
    fn _cairo_type1_subset_init(
        type_subset: *mut cairo_type1_subset_t,
        name: *const libc::c_char,
        font_subset: *mut cairo_scaled_font_subset_t,
        hex_encode: cairo_bool_t,
    ) -> cairo_status_t;
    fn _cairo_type1_subset_fini(subset: *mut cairo_type1_subset_t);
    fn _cairo_type1_fallback_init_binary(
        type_subset: *mut cairo_type1_subset_t,
        name: *const libc::c_char,
        font_subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_status_t;
    fn _cairo_type1_fallback_fini(subset: *mut cairo_type1_subset_t);
    fn _cairo_type3_glyph_surface_create(
        scaled_font: *mut cairo_scaled_font_t,
        stream: *mut cairo_output_stream_t,
        emit_image: cairo_type3_glyph_surface_emit_image_t,
        font_subsets: *mut cairo_scaled_font_subsets_t,
        ps_output: cairo_bool_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_type3_glyph_surface_set_font_subsets_callback(
        abstract_surface: *mut libc::c_void,
        use_font_subset: cairo_pdf_operators_use_font_subset_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_type3_glyph_surface_analyze_glyph(
        abstract_surface: *mut libc::c_void,
        glyph_index: libc::c_ulong,
    ) -> cairo_status_t;
    fn _cairo_type3_glyph_surface_emit_glyph(
        abstract_surface: *mut libc::c_void,
        stream: *mut cairo_output_stream_t,
        glyph_index: libc::c_ulong,
        bbox: *mut cairo_box_t,
        width: *mut libc::c_double,
    ) -> cairo_status_t;
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
pub type va_list = __builtin_va_list;
pub type ptrdiff_t = libc::c_long;
pub type cairo_bool_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo {
    pub ref_count: cairo_reference_count_t,
    pub status: cairo_status_t,
    pub user_data: cairo_user_data_array_t,
    pub backend: *const cairo_backend_t,
}
pub type cairo_backend_t = _cairo_backend;
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
pub type cairo_damage_t = _cairo_damage;
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
pub type cairo_recursive_mutex_t = cairo_recursive_mutex_impl_t;
pub type cairo_recursive_mutex_impl_t = pthread_mutex_t;
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
pub type cairo_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type cairo_rectangle_t = _cairo_rectangle;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type cairo_fixed_unsigned_t = uint32_t;
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
pub struct _cairo_output_stream {
    pub write_func: cairo_output_stream_write_func_t,
    pub flush_func: cairo_output_stream_flush_func_t,
    pub close_func: cairo_output_stream_close_func_t,
    pub position: libc::c_longlong,
    pub status: cairo_status_t,
    pub closed: cairo_bool_t,
}
pub type cairo_output_stream_close_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
>;
pub type cairo_output_stream_t = _cairo_output_stream;
pub type cairo_output_stream_flush_func_t = Option::<
    unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
>;
pub type cairo_output_stream_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_output_stream_t,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_paginated_surface_backend {
    pub start_page: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
    >,
    pub set_paginated_mode: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            cairo_paginated_mode_t,
        ) -> cairo_int_status_t,
    >,
    pub set_bounding_box: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut cairo_box_t) -> cairo_int_status_t,
    >,
    pub set_fallback_images_required: Option::<
        unsafe extern "C" fn(*mut libc::c_void, cairo_bool_t) -> cairo_int_status_t,
    >,
    pub supports_fine_grained_fallbacks: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
    >,
    pub requires_thumbnail_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> cairo_bool_t,
    >,
    pub set_thumbnail_image: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut cairo_image_surface_t,
        ) -> cairo_int_status_t,
    >,
}
pub type cairo_paginated_mode_t = _cairo_paginated_mode;
pub type _cairo_paginated_mode = libc::c_uint;
pub const CAIRO_PAGINATED_MODE_FALLBACK: _cairo_paginated_mode = 2;
pub const CAIRO_PAGINATED_MODE_RENDER: _cairo_paginated_mode = 1;
pub const CAIRO_PAGINATED_MODE_ANALYZE: _cairo_paginated_mode = 0;
pub type cairo_paginated_surface_backend_t = _cairo_paginated_surface_backend;
pub type cairo_scaled_font_subsets_t = _cairo_scaled_font_subsets;
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
pub struct _cairo_box_double {
    pub p1: cairo_point_double_t,
    pub p2: cairo_point_double_t,
}
pub type cairo_box_double_t = _cairo_box_double;
pub type _cairo_image_transparency = libc::c_uint;
pub const CAIRO_IMAGE_UNKNOWN: _cairo_image_transparency = 3;
pub const CAIRO_IMAGE_HAS_ALPHA: _cairo_image_transparency = 2;
pub const CAIRO_IMAGE_HAS_BILEVEL_ALPHA: _cairo_image_transparency = 1;
pub const CAIRO_IMAGE_IS_OPAQUE: _cairo_image_transparency = 0;
pub type cairo_image_transparency_t = _cairo_image_transparency;
pub type _cairo_image_color = libc::c_uint;
pub const CAIRO_IMAGE_UNKNOWN_COLOR: _cairo_image_color = 3;
pub const CAIRO_IMAGE_IS_MONOCHROME: _cairo_image_color = 2;
pub const CAIRO_IMAGE_IS_GRAYSCALE: _cairo_image_color = 1;
pub const CAIRO_IMAGE_IS_COLOR: _cairo_image_color = 0;
pub type cairo_image_color_t = _cairo_image_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_scaled_font_subset {
    pub scaled_font: *mut cairo_scaled_font_t,
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub glyphs: *mut libc::c_ulong,
    pub utf8: *mut *mut libc::c_char,
    pub glyph_names: *mut *mut libc::c_char,
    pub to_latin_char: *mut libc::c_int,
    pub latin_to_subset_glyph_index: *mut libc::c_ulong,
    pub num_glyphs: libc::c_uint,
    pub is_composite: cairo_bool_t,
    pub is_scaled: cairo_bool_t,
    pub is_latin: cairo_bool_t,
}
pub type cairo_scaled_font_subset_t = _cairo_scaled_font_subset;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: libc::c_double,
    pub i: [int32_t; 2],
}
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type _cairo_pdf_version = libc::c_uint;
pub const CAIRO_PDF_VERSION_1_7: _cairo_pdf_version = 3;
pub const CAIRO_PDF_VERSION_1_6: _cairo_pdf_version = 2;
pub const CAIRO_PDF_VERSION_1_5: _cairo_pdf_version = 1;
pub const CAIRO_PDF_VERSION_1_4: _cairo_pdf_version = 0;
pub type cairo_pdf_version_t = _cairo_pdf_version;
pub type cairo_pdf_surface_t = _cairo_pdf_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_surface {
    pub base: cairo_surface_t,
    pub output: *mut cairo_output_stream_t,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub surface_extents: cairo_rectangle_int_t,
    pub surface_bounded: cairo_bool_t,
    pub cairo_to_pdf: cairo_matrix_t,
    pub in_xobject: cairo_bool_t,
    pub objects: cairo_array_t,
    pub pages: cairo_array_t,
    pub rgb_linear_functions: cairo_array_t,
    pub alpha_linear_functions: cairo_array_t,
    pub page_patterns: cairo_array_t,
    pub page_surfaces: cairo_array_t,
    pub doc_surfaces: cairo_array_t,
    pub all_surfaces: *mut cairo_hash_table_t,
    pub smask_groups: cairo_array_t,
    pub knockout_group: cairo_array_t,
    pub jbig2_global: cairo_array_t,
    pub page_heights: cairo_array_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub fonts: cairo_array_t,
    pub next_available_resource: cairo_pdf_resource_t,
    pub pages_resource: cairo_pdf_resource_t,
    pub struct_tree_root: cairo_pdf_resource_t,
    pub pdf_version: cairo_pdf_version_t,
    pub compress_streams: cairo_bool_t,
    pub content: cairo_pdf_resource_t,
    pub content_resources: cairo_pdf_resource_t,
    pub resources: cairo_pdf_group_resources_t,
    pub has_fallback_images: cairo_bool_t,
    pub header_emitted: cairo_bool_t,
    pub pdf_stream: C2RustUnnamed_2,
    pub group_stream: C2RustUnnamed_1,
    pub object_stream: C2RustUnnamed_0,
    pub clipper: cairo_surface_clipper_t,
    pub pdf_operators: cairo_pdf_operators_t,
    pub paginated_mode: cairo_paginated_mode_t,
    pub select_pattern_gstate_saved: cairo_bool_t,
    pub force_fallbacks: cairo_bool_t,
    pub current_operator: cairo_operator_t,
    pub current_pattern_is_solid_color: cairo_bool_t,
    pub current_color_is_stroke: cairo_bool_t,
    pub current_color_red: libc::c_double,
    pub current_color_green: libc::c_double,
    pub current_color_blue: libc::c_double,
    pub current_color_alpha: libc::c_double,
    pub interchange: cairo_pdf_interchange_t,
    pub page_parent_tree: libc::c_int,
    pub page_annots: cairo_array_t,
    pub forward_links: cairo_array_t,
    pub tagged: cairo_bool_t,
    pub current_page_label: *mut libc::c_char,
    pub page_labels: cairo_array_t,
    pub outlines_dict_res: cairo_pdf_resource_t,
    pub names_dict_res: cairo_pdf_resource_t,
    pub docinfo_res: cairo_pdf_resource_t,
    pub page_labels_res: cairo_pdf_resource_t,
    pub thumbnail_width: libc::c_int,
    pub thumbnail_height: libc::c_int,
    pub thumbnail_image: *mut cairo_image_surface_t,
    pub paginated_surface: *mut cairo_surface_t,
}
pub type cairo_pdf_resource_t = _cairo_pdf_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_resource {
    pub id: libc::c_uint,
}
pub type cairo_pdf_interchange_t = _cairo_pdf_interchange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_interchange {
    pub analysis_tag_stack: cairo_tag_stack_t,
    pub render_tag_stack: cairo_tag_stack_t,
    pub push_data: cairo_array_t,
    pub push_data_index: libc::c_int,
    pub struct_root: *mut cairo_pdf_struct_tree_node_t,
    pub current_node: *mut cairo_pdf_struct_tree_node_t,
    pub begin_page_node: *mut cairo_pdf_struct_tree_node_t,
    pub end_page_node: *mut cairo_pdf_struct_tree_node_t,
    pub parent_tree: cairo_array_t,
    pub mcid_to_tree: cairo_array_t,
    pub annots: cairo_array_t,
    pub parent_tree_res: cairo_pdf_resource_t,
    pub extents_list: cairo_list_t,
    pub named_dests: *mut cairo_hash_table_t,
    pub num_dests: libc::c_int,
    pub sorted_dests: *mut *mut cairo_pdf_named_dest_t,
    pub dests_res: cairo_pdf_resource_t,
    pub annot_page: libc::c_int,
    pub outline: cairo_array_t,
    pub docinfo: docinfo,
    pub custom_metadata: cairo_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct docinfo {
    pub title: *mut libc::c_char,
    pub author: *mut libc::c_char,
    pub subject: *mut libc::c_char,
    pub keywords: *mut libc::c_char,
    pub creator: *mut libc::c_char,
    pub create_date: *mut libc::c_char,
    pub mod_date: *mut libc::c_char,
}
pub type cairo_pdf_named_dest_t = _cairo_pdf_named_dest;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_named_dest {
    pub base: cairo_hash_entry_t,
    pub extents: tag_extents,
    pub attrs: cairo_dest_attrs_t,
    pub page: libc::c_int,
}
pub type cairo_dest_attrs_t = _cairo_dest_attrs;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag_extents {
    pub extents: cairo_rectangle_int_t,
    pub valid: cairo_bool_t,
    pub link: cairo_list_t,
}
pub type cairo_pdf_struct_tree_node_t = _cairo_pdf_struct_tree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_struct_tree_node {
    pub name: *mut libc::c_char,
    pub res: cairo_pdf_resource_t,
    pub parent: *mut _cairo_pdf_struct_tree_node,
    pub children: cairo_list_t,
    pub mcid: cairo_array_t,
    pub annot_res: cairo_pdf_resource_t,
    pub extents: tag_extents,
    pub link: cairo_list_t,
}
pub type cairo_tag_stack_t = _cairo_tag_stack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_tag_stack {
    pub list: cairo_list_t,
    pub type_0: cairo_tag_stack_structure_type_t,
    pub size: libc::c_int,
}
pub type cairo_tag_stack_structure_type_t = _cairo_tag_stack_structure_type;
pub type _cairo_tag_stack_structure_type = libc::c_uint;
pub const TAG_TREE_TYPE_INVALID: _cairo_tag_stack_structure_type = 4;
pub const TAG_TREE_TYPE_NO_TAGS: _cairo_tag_stack_structure_type = 3;
pub const TAG_TREE_TYPE_LINK_ONLY: _cairo_tag_stack_structure_type = 2;
pub const TAG_TREE_TYPE_STRUCTURE: _cairo_tag_stack_structure_type = 1;
pub const TAG_TREE_TYPE_TAGGED: _cairo_tag_stack_structure_type = 0;
pub type cairo_pdf_operators_t = _cairo_pdf_operators;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_operators {
    pub stream: *mut cairo_output_stream_t,
    pub cairo_to_pdf: cairo_matrix_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub use_font_subset: cairo_pdf_operators_use_font_subset_t,
    pub use_font_subset_closure: *mut libc::c_void,
    pub ps_output: cairo_bool_t,
    pub use_actual_text: cairo_bool_t,
    pub in_text_object: cairo_bool_t,
    pub is_new_text_object: cairo_bool_t,
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub text_matrix: cairo_matrix_t,
    pub cairo_to_pdftext: cairo_matrix_t,
    pub font_matrix_inverse: cairo_matrix_t,
    pub cur_x: libc::c_double,
    pub cur_y: libc::c_double,
    pub hex_width: libc::c_int,
    pub is_latin: cairo_bool_t,
    pub num_glyphs: libc::c_int,
    pub glyph_buf_x_pos: libc::c_double,
    pub glyphs: [cairo_pdf_glyph_t; 200],
    pub has_line_style: cairo_bool_t,
    pub line_width: libc::c_double,
    pub line_cap: cairo_line_cap_t,
    pub line_join: cairo_line_join_t,
    pub miter_limit: libc::c_double,
    pub has_dashes: cairo_bool_t,
}
pub type cairo_pdf_glyph_t = _cairo_pdf_glyph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_glyph {
    pub glyph_index: libc::c_uint,
    pub x_position: libc::c_double,
    pub x_advance: libc::c_double,
}
pub type cairo_pdf_operators_use_font_subset_t = Option::<
    unsafe extern "C" fn(
        libc::c_uint,
        libc::c_uint,
        *mut libc::c_void,
    ) -> cairo_int_status_t,
>;
pub type cairo_surface_clipper_t = _cairo_surface_clipper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_surface_clipper {
    pub clip: *mut cairo_clip_t,
    pub intersect_clip_path: cairo_surface_clipper_intersect_clip_path_func_t,
}
pub type cairo_surface_clipper_intersect_clip_path_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_surface_clipper_t,
        *mut cairo_path_fixed_t,
        cairo_fill_rule_t,
        libc::c_double,
        cairo_antialias_t,
    ) -> cairo_status_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub active: cairo_bool_t,
    pub stream: *mut cairo_output_stream_t,
    pub resource: cairo_pdf_resource_t,
    pub objects: cairo_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub active: cairo_bool_t,
    pub stream: *mut cairo_output_stream_t,
    pub mem_stream: *mut cairo_output_stream_t,
    pub old_output: *mut cairo_output_stream_t,
    pub resource: cairo_pdf_resource_t,
    pub bbox: cairo_box_double_t,
    pub is_knockout: cairo_bool_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub active: cairo_bool_t,
    pub self_0: cairo_pdf_resource_t,
    pub length: cairo_pdf_resource_t,
    pub start_offset: libc::c_longlong,
    pub compressed: cairo_bool_t,
    pub old_output: *mut cairo_output_stream_t,
}
pub type cairo_pdf_group_resources_t = _cairo_pdf_group_resources;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_group_resources {
    pub operators: [cairo_bool_t; 29],
    pub alphas: cairo_array_t,
    pub smasks: cairo_array_t,
    pub patterns: cairo_array_t,
    pub shadings: cairo_array_t,
    pub xobjects: cairo_array_t,
    pub fonts: cairo_array_t,
}
pub type cairo_pdf_object_t = _cairo_pdf_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_object {
    pub type_0: cairo_pdf_object_type_t,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub offset: libc::c_longlong,
    pub compressed_obj: compressed_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compressed_obj {
    pub xref_stream: cairo_pdf_resource_t,
    pub index: libc::c_int,
}
pub type cairo_pdf_object_type_t = libc::c_uint;
pub const PDF_OBJECT_COMPRESSED: cairo_pdf_object_type_t = 2;
pub const PDF_OBJECT_UNCOMPRESSED: cairo_pdf_object_type_t = 1;
pub const PDF_OBJECT_FREE: cairo_pdf_object_type_t = 0;
pub type cairo_pdf_forward_link_t = _cairo_pdf_forward_link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_forward_link {
    pub res: cairo_pdf_resource_t,
    pub dest: *mut libc::c_char,
    pub page: libc::c_int,
    pub has_pos: cairo_bool_t,
    pub pos: cairo_point_double_t,
}
pub type cairo_pdf_font_t = _cairo_pdf_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_font {
    pub font_id: libc::c_uint,
    pub subset_id: libc::c_uint,
    pub subset_resource: cairo_pdf_resource_t,
}
pub type cairo_xref_stream_object_t = _cairo_xref_stream_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_xref_stream_object {
    pub resource: cairo_pdf_resource_t,
    pub offset: libc::c_longlong,
}
pub type cairo_pdf_source_surface_entry_t = _cairo_pdf_source_surface_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_source_surface_entry {
    pub base: cairo_hash_entry_t,
    pub id: libc::c_uint,
    pub unique_id: *mut libc::c_uchar,
    pub unique_id_length: libc::c_ulong,
    pub operator: cairo_operator_t,
    pub interpolate: cairo_bool_t,
    pub stencil_mask: cairo_bool_t,
    pub smask: cairo_bool_t,
    pub need_transp_group: cairo_bool_t,
    pub surface_res: cairo_pdf_resource_t,
    pub smask_res: cairo_pdf_resource_t,
    pub emit_image: cairo_bool_t,
    pub bounded: cairo_bool_t,
    pub extents: cairo_rectangle_int_t,
    pub required_extents: cairo_rectangle_int_t,
}
pub type cairo_pdf_jbig2_global_t = _cairo_pdf_jbig2_global;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_jbig2_global {
    pub id: *mut libc::c_uchar,
    pub id_length: libc::c_ulong,
    pub res: cairo_pdf_resource_t,
    pub emitted: cairo_bool_t,
}
pub type cairo_pdf_source_surface_t = _cairo_pdf_source_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_source_surface {
    pub type_0: cairo_pattern_type_t,
    pub surface: *mut cairo_surface_t,
    pub raster_pattern: *mut cairo_pattern_t,
    pub hash_entry: *mut cairo_pdf_source_surface_entry_t,
}
pub type cairo_pdf_pattern_t = _cairo_pdf_pattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_pattern {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub extents: cairo_rectangle_int_t,
    pub pattern: *mut cairo_pattern_t,
    pub pattern_res: cairo_pdf_resource_t,
    pub gstate_res: cairo_pdf_resource_t,
    pub operator: cairo_operator_t,
    pub is_shading: cairo_bool_t,
    pub inverted_y_axis: cairo_bool_t,
}
pub type cairo_pdf_smask_group_t = _cairo_pdf_smask_group;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_smask_group {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub extents: cairo_rectangle_int_t,
    pub group_res: cairo_pdf_resource_t,
    pub operation: cairo_pdf_operation_t,
    pub source: *mut cairo_pattern_t,
    pub source_res: cairo_pdf_resource_t,
    pub mask: *mut cairo_pattern_t,
    pub path: cairo_path_fixed_t,
    pub fill_rule: cairo_fill_rule_t,
    pub style: cairo_stroke_style_t,
    pub ctm: cairo_matrix_t,
    pub ctm_inverse: cairo_matrix_t,
    pub utf8: *mut libc::c_char,
    pub utf8_len: libc::c_int,
    pub glyphs: *mut cairo_glyph_t,
    pub num_glyphs: libc::c_int,
    pub clusters: *mut cairo_text_cluster_t,
    pub num_clusters: libc::c_int,
    pub cluster_flags: cairo_bool_t,
    pub scaled_font: *mut cairo_scaled_font_t,
}
pub type cairo_pdf_operation_t = _cairo_pdf_operation;
pub type _cairo_pdf_operation = libc::c_uint;
pub const PDF_SHOW_GLYPHS: _cairo_pdf_operation = 4;
pub const PDF_STROKE: _cairo_pdf_operation = 3;
pub const PDF_FILL: _cairo_pdf_operation = 2;
pub const PDF_MASK: _cairo_pdf_operation = 1;
pub const PDF_PAINT: _cairo_pdf_operation = 0;
pub type cairo_pdf_alpha_linear_function_t = _cairo_pdf_alpha_linear_function;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_alpha_linear_function {
    pub resource: cairo_pdf_resource_t,
    pub alpha1: libc::c_double,
    pub alpha2: libc::c_double,
}
pub type cairo_pdf_rgb_linear_function_t = _cairo_pdf_rgb_linear_function;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_rgb_linear_function {
    pub resource: cairo_pdf_resource_t,
    pub color1: [libc::c_double; 3],
    pub color2: [libc::c_double; 3],
}
pub type cairo_recording_region_type_t = libc::c_uint;
pub const CAIRO_RECORDING_REGION_IMAGE_FALLBACK: cairo_recording_region_type_t = 2;
pub const CAIRO_RECORDING_REGION_NATIVE: cairo_recording_region_type_t = 1;
pub const CAIRO_RECORDING_REGION_ALL: cairo_recording_region_type_t = 0;
pub type cairo_recording_surface_t = _cairo_recording_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_recording_surface {
    pub base: cairo_surface_t,
    pub extents_pixels: cairo_rectangle_t,
    pub extents: cairo_rectangle_int_t,
    pub unbounded: cairo_bool_t,
    pub commands: cairo_array_t,
    pub indices: *mut libc::c_uint,
    pub num_indices: libc::c_uint,
    pub optimize_clears: cairo_bool_t,
    pub has_bilevel_alpha: cairo_bool_t,
    pub has_only_op_over: cairo_bool_t,
    pub bbtree: bbtree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bbtree {
    pub extents: cairo_box_t,
    pub left: *mut bbtree,
    pub right: *mut bbtree,
    pub chain: *mut cairo_command_header_t,
}
pub type cairo_command_header_t = _cairo_command_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_command_header {
    pub type_0: cairo_command_type_t,
    pub region: cairo_recording_region_type_t,
    pub op: cairo_operator_t,
    pub extents: cairo_rectangle_int_t,
    pub clip: *mut cairo_clip_t,
    pub index: libc::c_int,
    pub chain: *mut _cairo_command_header,
}
pub type cairo_command_type_t = libc::c_uint;
pub const CAIRO_COMMAND_TAG: cairo_command_type_t = 5;
pub const CAIRO_COMMAND_SHOW_TEXT_GLYPHS: cairo_command_type_t = 4;
pub const CAIRO_COMMAND_FILL: cairo_command_type_t = 3;
pub const CAIRO_COMMAND_STROKE: cairo_command_type_t = 2;
pub const CAIRO_COMMAND_MASK: cairo_command_type_t = 1;
pub const CAIRO_COMMAND_PAINT: cairo_command_type_t = 0;
pub type cairo_ccitt_params_t = _cairo_ccitt_params;
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
pub type cairo_image_info_t = _cairo_image_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_image_info {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub num_components: libc::c_int,
    pub bits_per_component: libc::c_int,
}
pub type cairo_pdf_shading_t = _cairo_pdf_shading;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_shading {
    pub shading_type: libc::c_int,
    pub bits_per_coordinate: libc::c_int,
    pub bits_per_component: libc::c_int,
    pub bits_per_flag: libc::c_int,
    pub decode_array: *mut libc::c_double,
    pub decode_array_length: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub data_length: libc::c_ulong,
}
pub type cairo_pdf_color_stop_t = _cairo_pdf_color_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_pdf_color_stop {
    pub offset: libc::c_double,
    pub color: [libc::c_double; 4],
    pub resource: cairo_pdf_resource_t,
}
pub type cairo_type3_glyph_surface_emit_image_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_image_surface_t,
        *mut cairo_output_stream_t,
    ) -> cairo_int_status_t,
>;
pub type cairo_scaled_font_subset_callback_func_t = Option::<
    unsafe extern "C" fn(
        *mut cairo_scaled_font_subset_t,
        *mut libc::c_void,
    ) -> cairo_int_status_t,
>;
pub type cairo_type1_subset_t = _cairo_type1_subset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_type1_subset {
    pub base_font: *mut libc::c_char,
    pub widths: *mut libc::c_double,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub data: *mut libc::c_char,
    pub header_length: libc::c_ulong,
    pub data_length: libc::c_ulong,
    pub trailer_length: libc::c_ulong,
}
pub type cairo_cff_subset_t = _cairo_cff_subset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_cff_subset {
    pub family_name_utf8: *mut libc::c_char,
    pub ps_name: *mut libc::c_char,
    pub widths: *mut libc::c_double,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub data: *mut libc::c_char,
    pub data_length: libc::c_ulong,
}
pub type cairo_truetype_subset_t = _cairo_truetype_subset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_truetype_subset {
    pub family_name_utf8: *mut libc::c_char,
    pub ps_name: *mut libc::c_char,
    pub widths: *mut libc::c_double,
    pub x_min: libc::c_double,
    pub y_min: libc::c_double,
    pub x_max: libc::c_double,
    pub y_max: libc::c_double,
    pub ascent: libc::c_double,
    pub descent: libc::c_double,
    pub data: *mut libc::c_uchar,
    pub data_length: libc::c_ulong,
    pub string_offsets: *mut libc::c_ulong,
    pub num_string_offsets: libc::c_ulong,
}
pub type cairo_close_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
>;
pub type _cairo_pdf_outline_flags = libc::c_uint;
pub const CAIRO_PDF_OUTLINE_FLAG_ITALIC: _cairo_pdf_outline_flags = 4;
pub const CAIRO_PDF_OUTLINE_FLAG_BOLD: _cairo_pdf_outline_flags = 2;
pub const CAIRO_PDF_OUTLINE_FLAG_OPEN: _cairo_pdf_outline_flags = 1;
pub type cairo_pdf_outline_flags_t = _cairo_pdf_outline_flags;
pub type _cairo_pdf_metadata = libc::c_uint;
pub const CAIRO_PDF_METADATA_MOD_DATE: _cairo_pdf_metadata = 6;
pub const CAIRO_PDF_METADATA_CREATE_DATE: _cairo_pdf_metadata = 5;
pub const CAIRO_PDF_METADATA_CREATOR: _cairo_pdf_metadata = 4;
pub const CAIRO_PDF_METADATA_KEYWORDS: _cairo_pdf_metadata = 3;
pub const CAIRO_PDF_METADATA_SUBJECT: _cairo_pdf_metadata = 2;
pub const CAIRO_PDF_METADATA_AUTHOR: _cairo_pdf_metadata = 1;
pub const CAIRO_PDF_METADATA_TITLE: _cairo_pdf_metadata = 0;
pub type cairo_pdf_metadata_t = _cairo_pdf_metadata;
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_abc(
    mut a: size_t,
    mut b: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let mut d: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(b);
    *(&mut c as *mut size_t) = fresh0;
    if fresh1 {
        return 0 as *mut libc::c_void;
    }
    let (fresh2, fresh3) = c.overflowing_mul(size);
    *(&mut d as *mut size_t) = fresh2;
    if fresh3 {
        return 0 as *mut libc::c_void;
    }
    return if d != 0 as libc::c_int as libc::c_ulong {
        malloc(d)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline(always)]
unsafe extern "C" fn _cairo_malloc_ab(
    mut a: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut c: size_t = 0;
    let (fresh4, fresh5) = a.overflowing_mul(size);
    *(&mut c as *mut size_t) = fresh4;
    if fresh5 {
        return 0 as *mut libc::c_void;
    }
    return if c != 0 as libc::c_int as libc::c_ulong {
        malloc(c)
    } else {
        0 as *mut libc::c_void
    };
}
#[inline]
unsafe extern "C" fn _cairo_unbounded_rectangle_init(
    mut rect: *mut cairo_rectangle_int_t,
) {
    *rect = _cairo_unbounded_rectangle;
}
#[inline]
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_double(mut d: libc::c_double) -> cairo_fixed_t {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
    u
        .d = d
        + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
            as libc::c_double * 1.5f64;
    return u.i[0 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn _cairo_fixed_to_double(mut f: cairo_fixed_t) -> libc::c_double {
    return f as libc::c_double
        / ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_integer_floor(mut f: cairo_fixed_t) -> libc::c_int {
    if f >= 0 as libc::c_int {
        return f >> 8 as libc::c_int
    } else {
        return -(-f - 1 as libc::c_int >> 8 as libc::c_int) - 1 as libc::c_int
    };
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
unsafe extern "C" fn _cairo_surface_is_image(
    mut surface: *const cairo_surface_t,
) -> cairo_bool_t {
    return (!((*surface).backend).is_null()
        && (*(*surface).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_surface_reference(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    if !(_cairo_atomic_int_get(&mut (*surface).ref_count.ref_count)
        == -(1 as libc::c_int))
    {
        ::std::intrinsics::atomic_xadd(
            &mut (*surface).ref_count.ref_count,
            1 as libc::c_int,
        );
    }
    return surface;
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
unsafe extern "C" fn _cairo_surface_snapshot_get_target(
    mut surface: *mut cairo_surface_t,
) -> *mut cairo_surface_t {
    let mut snapshot: *mut cairo_surface_snapshot_t = surface
        as *mut cairo_surface_snapshot_t;
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    pthread_mutex_lock(&mut (*snapshot).mutex);
    target = _cairo_surface_reference((*snapshot).target);
    pthread_mutex_unlock(&mut (*snapshot).mutex);
    return target;
}
static mut _cairo_pdf_versions: [cairo_pdf_version_t; 4] = [
    CAIRO_PDF_VERSION_1_4,
    CAIRO_PDF_VERSION_1_5,
    CAIRO_PDF_VERSION_1_6,
    CAIRO_PDF_VERSION_1_7,
];
static mut _cairo_pdf_version_strings: [*const libc::c_char; 4] = [
    b"PDF 1.4\0" as *const u8 as *const libc::c_char,
    b"PDF 1.5\0" as *const u8 as *const libc::c_char,
    b"PDF 1.6\0" as *const u8 as *const libc::c_char,
    b"PDF 1.7\0" as *const u8 as *const libc::c_char,
];
static mut _cairo_pdf_supported_mime_types: [*const libc::c_char; 9] = [
    b"image/jpeg\0" as *const u8 as *const libc::c_char,
    b"image/jp2\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.uuid\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.jbig2\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.jbig2-global\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.jbig2-global-id\0" as *const u8 as *const libc::c_char,
    b"image/g3fax\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.ccitt.params\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_surface_new_object(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_pdf_resource_t {
    let mut resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut object: cairo_pdf_object_t = cairo_pdf_object_t {
        type_0: PDF_OBJECT_FREE,
        u: C2RustUnnamed_3 { offset: 0 },
    };
    object.type_0 = PDF_OBJECT_UNCOMPRESSED;
    object.u.offset = _cairo_output_stream_get_position((*surface).output);
    status = _cairo_array_append(
        &mut (*surface).objects,
        &mut object as *mut cairo_pdf_object_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        resource.id = 0 as libc::c_int as libc::c_uint;
        return resource;
    }
    resource = (*surface).next_available_resource;
    let ref mut fresh6 = (*surface).next_available_resource.id;
    *fresh6 = (*fresh6).wrapping_add(1);
    return resource;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_surface_update_object(
    mut surface: *mut cairo_pdf_surface_t,
    mut resource: cairo_pdf_resource_t,
) {
    let mut object: *mut cairo_pdf_object_t = 0 as *mut cairo_pdf_object_t;
    object = _cairo_array_index(
        &mut (*surface).objects,
        (resource.id).wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) as *mut cairo_pdf_object_t;
    (*object).u.offset = _cairo_output_stream_get_position((*surface).output);
}
unsafe extern "C" fn _cairo_pdf_surface_set_size_internal(
    mut surface: *mut cairo_pdf_surface_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
) {
    (*surface).width = width;
    (*surface).height = height;
    (*surface).surface_extents.x = 0 as libc::c_int;
    (*surface).surface_extents.y = 0 as libc::c_int;
    (*surface).surface_extents.width = ceil((*surface).width) as libc::c_int;
    (*surface).surface_extents.height = ceil((*surface).height) as libc::c_int;
}
unsafe extern "C" fn _path_covers_bbox(
    mut surface: *mut cairo_pdf_surface_t,
    mut path: *mut cairo_path_fixed_t,
) -> cairo_bool_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    return (_cairo_path_fixed_is_box(path, &mut box_0) != 0
        && box_0.p1.x <= 0 as libc::c_int && box_0.p1.y <= 0 as libc::c_int
        && box_0.p2.x >= _cairo_fixed_from_double((*surface).width)
        && box_0.p2.y >= _cairo_fixed_from_double((*surface).height)) as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_surface_clipper_intersect_clip_path(
    mut clipper: *mut cairo_surface_clipper_t,
    mut path: *mut cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_status_t {
    let mut surface: *mut cairo_pdf_surface_t = ({
        let mut mptr__: *const cairo_surface_clipper_t = clipper;
        (mptr__ as *mut libc::c_char).offset(-(1216 as libc::c_ulong as isize))
            as *mut cairo_pdf_surface_t
    });
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if path.is_null() {
        _cairo_output_stream_printf(
            (*surface).output,
            b"Q q\n\0" as *const u8 as *const libc::c_char,
        );
        (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
        (*surface).current_operator = CAIRO_OPERATOR_OVER;
        _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
        return CAIRO_STATUS_SUCCESS;
    }
    if _path_covers_bbox(surface, path) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_pdf_operators_clip(&mut (*surface).pdf_operators, path, fill_rule)
        as cairo_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_create_for_stream_internal(
    mut output: *mut cairo_output_stream_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status_ignored: cairo_status_t = CAIRO_STATUS_SUCCESS;
    surface = (if ::std::mem::size_of::<cairo_pdf_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_pdf_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_pdf_surface_t;
    if surface.is_null() {
        status = _cairo_output_stream_destroy(output);
        return _cairo_surface_create_in_error(_cairo_error(CAIRO_STATUS_NO_MEMORY));
    }
    _cairo_surface_init(
        &mut (*surface).base,
        &cairo_pdf_surface_backend,
        0 as *mut cairo_device_t,
        CAIRO_CONTENT_COLOR_ALPHA,
        1 as libc::c_int,
    );
    let ref mut fresh7 = (*surface).output;
    *fresh7 = output;
    (*surface).width = width;
    (*surface).height = height;
    cairo_matrix_init(
        &mut (*surface).cairo_to_pdf,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    (*surface).in_xobject = 0 as libc::c_int;
    (*surface).surface_extents.x = 0 as libc::c_int;
    (*surface).surface_extents.y = 0 as libc::c_int;
    (*surface).surface_extents.width = ceil((*surface).width) as libc::c_int;
    (*surface).surface_extents.height = ceil((*surface).height) as libc::c_int;
    (*surface).surface_bounded = 1 as libc::c_int;
    _cairo_array_init(
        &mut (*surface).objects,
        ::std::mem::size_of::<cairo_pdf_object_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).pages,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).rgb_linear_functions,
        ::std::mem::size_of::<cairo_pdf_rgb_linear_function_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).alpha_linear_functions,
        ::std::mem::size_of::<cairo_pdf_alpha_linear_function_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).fonts,
        ::std::mem::size_of::<cairo_pdf_font_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).smask_groups,
        ::std::mem::size_of::<*mut cairo_pdf_smask_group_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).knockout_group,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).page_patterns,
        ::std::mem::size_of::<cairo_pdf_pattern_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).page_surfaces,
        ::std::mem::size_of::<cairo_pdf_source_surface_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).doc_surfaces,
        ::std::mem::size_of::<cairo_pdf_source_surface_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).jbig2_global,
        ::std::mem::size_of::<cairo_pdf_jbig2_global_t>() as libc::c_ulong
            as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*surface).page_heights,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_uint,
    );
    let ref mut fresh8 = (*surface).all_surfaces;
    *fresh8 = _cairo_hash_table_create(
        Some(
            _cairo_pdf_source_surface_equal
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> cairo_bool_t,
        ),
    );
    if ((*surface).all_surfaces).is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _cairo_pdf_group_resources_init(&mut (*surface).resources);
        let ref mut fresh9 = (*surface).font_subsets;
        *fresh9 = _cairo_scaled_font_subsets_create_composite();
        if ((*surface).font_subsets).is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        } else {
            _cairo_scaled_font_subsets_enable_latin_subset(
                (*surface).font_subsets,
                1 as libc::c_int,
            );
            (*surface).next_available_resource.id = 1 as libc::c_int as libc::c_uint;
            (*surface).pages_resource = _cairo_pdf_surface_new_object(surface);
            if (*surface).pages_resource.id == 0 as libc::c_int as libc::c_uint {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                (*surface).struct_tree_root.id = 0 as libc::c_int as libc::c_uint;
                (*surface).pdf_version = CAIRO_PDF_VERSION_1_7;
                (*surface).compress_streams = 1 as libc::c_int;
                (*surface).pdf_stream.active = 0 as libc::c_int;
                let ref mut fresh10 = (*surface).pdf_stream.old_output;
                *fresh10 = 0 as *mut cairo_output_stream_t;
                (*surface).group_stream.active = 0 as libc::c_int;
                let ref mut fresh11 = (*surface).group_stream.stream;
                *fresh11 = 0 as *mut cairo_output_stream_t;
                let ref mut fresh12 = (*surface).group_stream.mem_stream;
                *fresh12 = 0 as *mut cairo_output_stream_t;
                (*surface).object_stream.active = 0 as libc::c_int;
                let ref mut fresh13 = (*surface).object_stream.stream;
                *fresh13 = 0 as *mut cairo_output_stream_t;
                _cairo_array_init(
                    &mut (*surface).object_stream.objects,
                    ::std::mem::size_of::<cairo_xref_stream_object_t>() as libc::c_ulong
                        as libc::c_uint,
                );
                (*surface).paginated_mode = CAIRO_PAGINATED_MODE_ANALYZE;
                (*surface).force_fallbacks = 0 as libc::c_int;
                (*surface).select_pattern_gstate_saved = 0 as libc::c_int;
                (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
                (*surface).current_operator = CAIRO_OPERATOR_OVER;
                (*surface).header_emitted = 0 as libc::c_int;
                _cairo_surface_clipper_init(
                    &mut (*surface).clipper,
                    Some(
                        _cairo_pdf_surface_clipper_intersect_clip_path
                            as unsafe extern "C" fn(
                                *mut cairo_surface_clipper_t,
                                *mut cairo_path_fixed_t,
                                cairo_fill_rule_t,
                                libc::c_double,
                                cairo_antialias_t,
                            ) -> cairo_status_t,
                    ),
                );
                _cairo_pdf_operators_init(
                    &mut (*surface).pdf_operators,
                    (*surface).output,
                    &mut (*surface).cairo_to_pdf,
                    (*surface).font_subsets,
                    0 as libc::c_int,
                );
                _cairo_pdf_operators_set_font_subsets_callback(
                    &mut (*surface).pdf_operators,
                    Some(
                        _cairo_pdf_surface_add_font
                            as unsafe extern "C" fn(
                                libc::c_uint,
                                libc::c_uint,
                                *mut libc::c_void,
                            ) -> cairo_int_status_t,
                    ),
                    surface as *mut libc::c_void,
                );
                _cairo_pdf_operators_enable_actual_text(
                    &mut (*surface).pdf_operators,
                    1 as libc::c_int,
                );
                status = _cairo_pdf_interchange_init(surface) as cairo_status_t;
                if !(status as u64 != 0) {
                    (*surface).page_parent_tree = -(1 as libc::c_int);
                    _cairo_array_init(
                        &mut (*surface).page_annots,
                        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    _cairo_array_init(
                        &mut (*surface).forward_links,
                        ::std::mem::size_of::<cairo_pdf_forward_link_t>()
                            as libc::c_ulong as libc::c_uint,
                    );
                    (*surface).tagged = 0 as libc::c_int;
                    let ref mut fresh14 = (*surface).current_page_label;
                    *fresh14 = 0 as *mut libc::c_char;
                    _cairo_array_init(
                        &mut (*surface).page_labels,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    (*surface).outlines_dict_res.id = 0 as libc::c_int as libc::c_uint;
                    (*surface).names_dict_res.id = 0 as libc::c_int as libc::c_uint;
                    (*surface).docinfo_res.id = 0 as libc::c_int as libc::c_uint;
                    (*surface).page_labels_res.id = 0 as libc::c_int as libc::c_uint;
                    (*surface).thumbnail_width = 0 as libc::c_int;
                    (*surface).thumbnail_height = 0 as libc::c_int;
                    let ref mut fresh15 = (*surface).thumbnail_image;
                    *fresh15 = 0 as *mut cairo_image_surface_t;
                    if !(getenv(
                        b"CAIRO_DEBUG_PDF\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                    {
                        (*surface).compress_streams = 0 as libc::c_int;
                    }
                    let ref mut fresh16 = (*surface).paginated_surface;
                    *fresh16 = _cairo_paginated_surface_create(
                        &mut (*surface).base,
                        CAIRO_CONTENT_COLOR_ALPHA,
                        &cairo_pdf_surface_paginated_backend,
                    );
                    status = (*(*surface).paginated_surface).status;
                    if status as libc::c_uint
                        == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    {
                        cairo_surface_destroy(&mut (*surface).base);
                        return (*surface).paginated_surface;
                    }
                }
            }
            _cairo_scaled_font_subsets_destroy((*surface).font_subsets);
        }
        _cairo_hash_table_destroy((*surface).all_surfaces);
    }
    _cairo_array_fini(&mut (*surface).objects);
    free(surface as *mut libc::c_void);
    status_ignored = _cairo_output_stream_destroy(output);
    return _cairo_surface_create_in_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_create_for_stream(
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
    mut width_in_points: libc::c_double,
    mut height_in_points: libc::c_double,
) -> *mut cairo_surface_t {
    let mut output: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    output = _cairo_output_stream_create(write_func, None, closure);
    if _cairo_output_stream_get_status(output) as u64 != 0 {
        return _cairo_surface_create_in_error(_cairo_output_stream_destroy(output));
    }
    return _cairo_pdf_surface_create_for_stream_internal(
        output,
        width_in_points,
        height_in_points,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_create(
    mut filename: *const libc::c_char,
    mut width_in_points: libc::c_double,
    mut height_in_points: libc::c_double,
) -> *mut cairo_surface_t {
    let mut output: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    output = _cairo_output_stream_create_for_filename(filename);
    if _cairo_output_stream_get_status(output) as u64 != 0 {
        return _cairo_surface_create_in_error(_cairo_output_stream_destroy(output));
    }
    return _cairo_pdf_surface_create_for_stream_internal(
        output,
        width_in_points,
        height_in_points,
    );
}
unsafe extern "C" fn _cairo_surface_is_pdf(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*surface).backend
        == &cairo_pdf_surface_backend as *const cairo_surface_backend_t) as libc::c_int;
}
unsafe extern "C" fn _extract_pdf_surface(
    mut surface: *mut cairo_surface_t,
    mut pdf_surface: *mut *mut cairo_pdf_surface_t,
) -> cairo_bool_t {
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status_ignored: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).status as u64 != 0 {
        return 0 as libc::c_int;
    }
    if (*surface).finished() != 0 {
        status_ignored = _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
        return 0 as libc::c_int;
    }
    if _cairo_surface_is_paginated(surface) == 0 {
        status_ignored = _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        ) as cairo_status_t;
        return 0 as libc::c_int;
    }
    target = _cairo_paginated_surface_get_target(surface);
    if (*target).status as u64 != 0 {
        status_ignored = _cairo_surface_set_error(
            surface,
            (*target).status as cairo_int_status_t,
        ) as cairo_status_t;
        return 0 as libc::c_int;
    }
    if (*target).finished() != 0 {
        status_ignored = _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
        ) as cairo_status_t;
        return 0 as libc::c_int;
    }
    if _cairo_surface_is_pdf(target) == 0 {
        status_ignored = _cairo_surface_set_error(
            surface,
            _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
        ) as cairo_status_t;
        return 0 as libc::c_int;
    }
    *pdf_surface = target as *mut cairo_pdf_surface_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_restrict_to_version(
    mut abstract_surface: *mut cairo_surface_t,
    mut version: cairo_pdf_version_t,
) {
    let mut surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    if _extract_pdf_surface(abstract_surface, &mut surface) == 0 {
        return;
    }
    if (version as libc::c_uint)
        < (::std::mem::size_of::<[cairo_pdf_version_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_pdf_version_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        (*surface).pdf_version = version;
    }
    _cairo_pdf_operators_enable_actual_text(
        &mut (*surface).pdf_operators,
        (version as libc::c_uint >= CAIRO_PDF_VERSION_1_5 as libc::c_int as libc::c_uint)
            as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_get_versions(
    mut versions: *mut *const cairo_pdf_version_t,
    mut num_versions: *mut libc::c_int,
) {
    if !versions.is_null() {
        *versions = _cairo_pdf_versions.as_ptr();
    }
    if !num_versions.is_null() {
        *num_versions = (::std::mem::size_of::<[cairo_pdf_version_t; 4]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_pdf_version_t>() as libc::c_ulong)
            as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_version_to_string(
    mut version: cairo_pdf_version_t,
) -> *const libc::c_char {
    if version as libc::c_uint
        >= (::std::mem::size_of::<[cairo_pdf_version_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_pdf_version_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        return 0 as *const libc::c_char;
    }
    return _cairo_pdf_version_strings[version as usize];
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_set_size(
    mut surface: *mut cairo_surface_t,
    mut width_in_points: libc::c_double,
    mut height_in_points: libc::c_double,
) {
    let mut pdf_surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _extract_pdf_surface(surface, &mut pdf_surface) == 0 {
        return;
    }
    _cairo_pdf_surface_set_size_internal(pdf_surface, width_in_points, height_in_points);
    status = _cairo_paginated_surface_set_size(
        (*pdf_surface).paginated_surface,
        width_in_points as libc::c_int,
        height_in_points as libc::c_int,
    );
    if status as u64 != 0 {
        status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_add_outline(
    mut surface: *mut cairo_surface_t,
    mut parent_id: libc::c_int,
    mut utf8: *const libc::c_char,
    mut link_attribs: *const libc::c_char,
    mut flags: cairo_pdf_outline_flags_t,
) -> libc::c_int {
    let mut pdf_surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut id: libc::c_int = 0 as libc::c_int;
    if _extract_pdf_surface(surface, &mut pdf_surface) == 0 {
        return 0 as libc::c_int;
    }
    status = _cairo_pdf_interchange_add_outline(
        pdf_surface,
        parent_id,
        utf8,
        link_attribs,
        flags,
        &mut id,
    ) as cairo_status_t;
    if status as u64 != 0 {
        status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_set_metadata(
    mut surface: *mut cairo_surface_t,
    mut metadata: cairo_pdf_metadata_t,
    mut utf8: *const libc::c_char,
) {
    let mut pdf_surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _extract_pdf_surface(surface, &mut pdf_surface) == 0 {
        return;
    }
    status = _cairo_pdf_interchange_set_metadata(pdf_surface, metadata, utf8)
        as cairo_status_t;
    if status as u64 != 0 {
        status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_set_custom_metadata(
    mut surface: *mut cairo_surface_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut pdf_surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _extract_pdf_surface(surface, &mut pdf_surface) == 0 {
        return;
    }
    status = _cairo_pdf_interchange_set_custom_metadata(pdf_surface, name, value)
        as cairo_status_t;
    if status as u64 != 0 {
        status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_set_page_label(
    mut surface: *mut cairo_surface_t,
    mut utf8: *const libc::c_char,
) {
    let mut pdf_surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    if _extract_pdf_surface(surface, &mut pdf_surface) == 0 {
        return;
    }
    free((*pdf_surface).current_page_label as *mut libc::c_void);
    let ref mut fresh17 = (*pdf_surface).current_page_label;
    *fresh17 = if !utf8.is_null() { strdup(utf8) } else { 0 as *mut libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn cairo_pdf_surface_set_thumbnail_size(
    mut surface: *mut cairo_surface_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut pdf_surface: *mut cairo_pdf_surface_t = 0 as *mut cairo_pdf_surface_t;
    if _extract_pdf_surface(surface, &mut pdf_surface) == 0 {
        return;
    }
    (*pdf_surface).thumbnail_width = width;
    (*pdf_surface).thumbnail_height = height;
}
unsafe extern "C" fn _cairo_pdf_surface_clear(mut surface: *mut cairo_pdf_surface_t) {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut pattern: *mut cairo_pdf_pattern_t = 0 as *mut cairo_pdf_pattern_t;
    let mut src_surface: *mut cairo_pdf_source_surface_t = 0
        as *mut cairo_pdf_source_surface_t;
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    size = _cairo_array_num_elements(&mut (*surface).page_patterns) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        pattern = _cairo_array_index(&mut (*surface).page_patterns, i as libc::c_uint)
            as *mut cairo_pdf_pattern_t;
        cairo_pattern_destroy((*pattern).pattern);
        i += 1;
    }
    _cairo_array_truncate(
        &mut (*surface).page_patterns,
        0 as libc::c_int as libc::c_uint,
    );
    size = _cairo_array_num_elements(&mut (*surface).page_surfaces) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        src_surface = _cairo_array_index(
            &mut (*surface).page_surfaces,
            i as libc::c_uint,
        ) as *mut cairo_pdf_source_surface_t;
        cairo_surface_destroy((*src_surface).surface);
        i += 1;
    }
    _cairo_array_truncate(
        &mut (*surface).page_surfaces,
        0 as libc::c_int as libc::c_uint,
    );
    size = _cairo_array_num_elements(&mut (*surface).smask_groups) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        _cairo_array_copy_element(
            &mut (*surface).smask_groups,
            i as libc::c_uint,
            &mut group as *mut *mut cairo_pdf_smask_group_t as *mut libc::c_void,
        );
        _cairo_pdf_smask_group_destroy(group);
        i += 1;
    }
    _cairo_array_truncate(
        &mut (*surface).smask_groups,
        0 as libc::c_int as libc::c_uint,
    );
    _cairo_array_truncate(
        &mut (*surface).knockout_group,
        0 as libc::c_int as libc::c_uint,
    );
    _cairo_array_truncate(&mut (*surface).page_annots, 0 as libc::c_int as libc::c_uint);
    if !((*surface).thumbnail_image).is_null() {
        cairo_surface_destroy(&mut (*(*surface).thumbnail_image).base);
    }
    let ref mut fresh18 = (*surface).thumbnail_image;
    *fresh18 = 0 as *mut cairo_image_surface_t;
}
unsafe extern "C" fn _cairo_pdf_group_resources_init(
    mut res: *mut cairo_pdf_group_resources_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int + 1 as libc::c_int {
        (*res).operators[i as usize] = 0 as libc::c_int;
        i += 1;
    }
    _cairo_array_init(
        &mut (*res).alphas,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*res).smasks,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*res).patterns,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*res).shadings,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*res).xobjects,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong as libc::c_uint,
    );
    _cairo_array_init(
        &mut (*res).fonts,
        ::std::mem::size_of::<cairo_pdf_font_t>() as libc::c_ulong as libc::c_uint,
    );
}
unsafe extern "C" fn _cairo_pdf_group_resources_fini(
    mut res: *mut cairo_pdf_group_resources_t,
) {
    _cairo_array_fini(&mut (*res).alphas);
    _cairo_array_fini(&mut (*res).smasks);
    _cairo_array_fini(&mut (*res).patterns);
    _cairo_array_fini(&mut (*res).shadings);
    _cairo_array_fini(&mut (*res).xobjects);
    _cairo_array_fini(&mut (*res).fonts);
}
unsafe extern "C" fn _cairo_pdf_group_resources_clear(
    mut res: *mut cairo_pdf_group_resources_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int + 1 as libc::c_int {
        (*res).operators[i as usize] = 0 as libc::c_int;
        i += 1;
    }
    _cairo_array_truncate(&mut (*res).alphas, 0 as libc::c_int as libc::c_uint);
    _cairo_array_truncate(&mut (*res).smasks, 0 as libc::c_int as libc::c_uint);
    _cairo_array_truncate(&mut (*res).patterns, 0 as libc::c_int as libc::c_uint);
    _cairo_array_truncate(&mut (*res).shadings, 0 as libc::c_int as libc::c_uint);
    _cairo_array_truncate(&mut (*res).xobjects, 0 as libc::c_int as libc::c_uint);
    _cairo_array_truncate(&mut (*res).fonts, 0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn _cairo_pdf_surface_add_operator(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
) {
    let mut res: *mut cairo_pdf_group_resources_t = &mut (*surface).resources;
    (*res).operators[op as usize] = 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_surface_add_alpha(
    mut surface: *mut cairo_pdf_surface_t,
    mut alpha: libc::c_double,
    mut index: *mut libc::c_int,
) -> cairo_int_status_t {
    let mut num_alphas: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut other: libc::c_double = 0.;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut res: *mut cairo_pdf_group_resources_t = &mut (*surface).resources;
    num_alphas = _cairo_array_num_elements(&mut (*res).alphas) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_alphas {
        _cairo_array_copy_element(
            &mut (*res).alphas,
            i as libc::c_uint,
            &mut other as *mut libc::c_double as *mut libc::c_void,
        );
        if alpha == other {
            *index = i;
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        i += 1;
    }
    status = _cairo_array_append(
        &mut (*res).alphas,
        &mut alpha as *mut libc::c_double as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    *index = (_cairo_array_num_elements(&mut (*res).alphas))
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_smask(
    mut surface: *mut cairo_pdf_surface_t,
    mut smask: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    return _cairo_array_append(
        &mut (*surface).resources.smasks,
        &mut smask as *mut cairo_pdf_resource_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    return _cairo_array_append(
        &mut (*surface).resources.patterns,
        &mut pattern as *mut cairo_pdf_resource_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_shading(
    mut surface: *mut cairo_pdf_surface_t,
    mut shading: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    return _cairo_array_append(
        &mut (*surface).resources.shadings,
        &mut shading as *mut cairo_pdf_resource_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_xobject(
    mut surface: *mut cairo_pdf_surface_t,
    mut xobject: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    return _cairo_array_append(
        &mut (*surface).resources.xobjects,
        &mut xobject as *mut cairo_pdf_resource_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_font(
    mut font_id: libc::c_uint,
    mut subset_id: libc::c_uint,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = closure as *mut cairo_pdf_surface_t;
    let mut font: cairo_pdf_font_t = cairo_pdf_font_t {
        font_id: 0,
        subset_id: 0,
        subset_resource: cairo_pdf_resource_t { id: 0 },
    };
    let mut num_fonts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut res: *mut cairo_pdf_group_resources_t = &mut (*surface).resources;
    num_fonts = _cairo_array_num_elements(&mut (*res).fonts) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_fonts {
        _cairo_array_copy_element(
            &mut (*res).fonts,
            i as libc::c_uint,
            &mut font as *mut cairo_pdf_font_t as *mut libc::c_void,
        );
        if font.font_id == font_id && font.subset_id == subset_id {
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        i += 1;
    }
    num_fonts = _cairo_array_num_elements(&mut (*surface).fonts) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_fonts {
        _cairo_array_copy_element(
            &mut (*surface).fonts,
            i as libc::c_uint,
            &mut font as *mut cairo_pdf_font_t as *mut libc::c_void,
        );
        if font.font_id == font_id && font.subset_id == subset_id {
            return _cairo_array_append(
                &mut (*res).fonts,
                &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
            ) as cairo_int_status_t;
        }
        i += 1;
    }
    font.font_id = font_id;
    font.subset_id = subset_id;
    font.subset_resource = _cairo_pdf_surface_new_object(surface);
    if font.subset_resource.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_array_append(
        &mut (*surface).fonts,
        &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    return _cairo_array_append(
        &mut (*res).fonts,
        &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_get_font_resource(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_id: libc::c_uint,
    mut subset_id: libc::c_uint,
) -> cairo_pdf_resource_t {
    let mut font: cairo_pdf_font_t = cairo_pdf_font_t {
        font_id: 0,
        subset_id: 0,
        subset_resource: cairo_pdf_resource_t { id: 0 },
    };
    let mut num_fonts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_fonts = _cairo_array_num_elements(&mut (*surface).fonts) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_fonts {
        _cairo_array_copy_element(
            &mut (*surface).fonts,
            i as libc::c_uint,
            &mut font as *mut cairo_pdf_font_t as *mut libc::c_void,
        );
        if font.font_id == font_id && font.subset_id == subset_id {
            return font.subset_resource;
        }
        i += 1;
    }
    font.subset_resource.id = 0 as libc::c_int as libc::c_uint;
    return font.subset_resource;
}
unsafe extern "C" fn _cairo_operator_to_pdf_blend_mode(
    mut op: cairo_operator_t,
) -> *const libc::c_char {
    match op as libc::c_uint {
        14 => return b"Multiply\0" as *const u8 as *const libc::c_char,
        15 => return b"Screen\0" as *const u8 as *const libc::c_char,
        16 => return b"Overlay\0" as *const u8 as *const libc::c_char,
        17 => return b"Darken\0" as *const u8 as *const libc::c_char,
        18 => return b"Lighten\0" as *const u8 as *const libc::c_char,
        19 => return b"ColorDodge\0" as *const u8 as *const libc::c_char,
        20 => return b"ColorBurn\0" as *const u8 as *const libc::c_char,
        21 => return b"HardLight\0" as *const u8 as *const libc::c_char,
        22 => return b"SoftLight\0" as *const u8 as *const libc::c_char,
        23 => return b"Difference\0" as *const u8 as *const libc::c_char,
        24 => return b"Exclusion\0" as *const u8 as *const libc::c_char,
        25 => return b"Hue\0" as *const u8 as *const libc::c_char,
        26 => return b"Saturation\0" as *const u8 as *const libc::c_char,
        27 => return b"Color\0" as *const u8 as *const libc::c_char,
        28 => return b"Luminosity\0" as *const u8 as *const libc::c_char,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | _ => {
            return b"Normal\0" as *const u8 as *const libc::c_char;
        }
    };
}
unsafe extern "C" fn _cairo_pdf_surface_emit_group_resources(
    mut surface: *mut cairo_pdf_surface_t,
    mut res: *mut cairo_pdf_group_resources_t,
    mut gs0: cairo_bool_t,
) {
    let mut num_alphas: libc::c_int = 0;
    let mut num_smasks: libc::c_int = 0;
    let mut num_resources: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut alpha: libc::c_double = 0.;
    let mut smask: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut pattern: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut shading: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut xobject: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut font: *mut cairo_pdf_font_t = 0 as *mut cairo_pdf_font_t;
    _cairo_output_stream_printf(
        (*surface).output,
        b"<<\n\0" as *const u8 as *const libc::c_char,
    );
    num_alphas = _cairo_array_num_elements(&mut (*res).alphas) as libc::c_int;
    num_smasks = _cairo_array_num_elements(&mut (*res).smasks) as libc::c_int;
    if num_alphas > 0 as libc::c_int || num_smasks > 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /ExtGState <<\n\0" as *const u8 as *const libc::c_char,
        );
        if gs0 != 0 {
            _cairo_output_stream_printf(
                (*surface).output,
                b"      /gs0 << /BM /Normal /SMask /None /CA 1.0 /ca 1.0 >>\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        i = 0 as libc::c_int;
        while i < CAIRO_OPERATOR_HSL_LUMINOSITY as libc::c_int + 1 as libc::c_int {
            if (*res).operators[i as usize] != 0 {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"      /b%d << /BM /%s >>\n\0" as *const u8 as *const libc::c_char,
                    i,
                    _cairo_operator_to_pdf_blend_mode(i as cairo_operator_t),
                );
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < num_alphas {
            _cairo_array_copy_element(
                &mut (*res).alphas,
                i as libc::c_uint,
                &mut alpha as *mut libc::c_double as *mut libc::c_void,
            );
            _cairo_output_stream_printf(
                (*surface).output,
                b"      /a%d << /CA %f /ca %f >>\n\0" as *const u8
                    as *const libc::c_char,
                i,
                alpha,
                alpha,
            );
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < num_smasks {
            smask = _cairo_array_index(&mut (*res).smasks, i as libc::c_uint)
                as *mut cairo_pdf_resource_t;
            _cairo_output_stream_printf(
                (*surface).output,
                b"      /s%d %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*smask).id,
                (*smask).id,
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"   >>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    num_resources = _cairo_array_num_elements(&mut (*res).patterns) as libc::c_int;
    if num_resources > 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Pattern <<\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < num_resources {
            pattern = _cairo_array_index(&mut (*res).patterns, i as libc::c_uint)
                as *mut cairo_pdf_resource_t;
            _cairo_output_stream_printf(
                (*surface).output,
                b" /p%d %d 0 R\0" as *const u8 as *const libc::c_char,
                (*pattern).id,
                (*pattern).id,
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" >>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    num_resources = _cairo_array_num_elements(&mut (*res).shadings) as libc::c_int;
    if num_resources > 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Shading <<\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < num_resources {
            shading = _cairo_array_index(&mut (*res).shadings, i as libc::c_uint)
                as *mut cairo_pdf_resource_t;
            _cairo_output_stream_printf(
                (*surface).output,
                b" /sh%d %d 0 R\0" as *const u8 as *const libc::c_char,
                (*shading).id,
                (*shading).id,
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" >>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    num_resources = _cairo_array_num_elements(&mut (*res).xobjects) as libc::c_int;
    if num_resources > 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /XObject <<\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < num_resources {
            xobject = _cairo_array_index(&mut (*res).xobjects, i as libc::c_uint)
                as *mut cairo_pdf_resource_t;
            _cairo_output_stream_printf(
                (*surface).output,
                b" /x%d %d 0 R\0" as *const u8 as *const libc::c_char,
                (*xobject).id,
                (*xobject).id,
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" >>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    num_resources = _cairo_array_num_elements(&mut (*res).fonts) as libc::c_int;
    if num_resources > 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Font <<\n\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < num_resources {
            font = _cairo_array_index(&mut (*res).fonts, i as libc::c_uint)
                as *mut cairo_pdf_font_t;
            _cairo_output_stream_printf(
                (*surface).output,
                b"      /f-%d-%d %d 0 R\n\0" as *const u8 as *const libc::c_char,
                (*font).font_id,
                (*font).subset_id,
                (*font).subset_resource.id,
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"   >>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn _cairo_pdf_surface_create_smask_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut extents: *const cairo_rectangle_int_t,
) -> *mut cairo_pdf_smask_group_t {
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    group = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cairo_pdf_smask_group_t>() as libc::c_ulong,
    ) as *mut cairo_pdf_smask_group_t;
    if group.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *mut cairo_pdf_smask_group_t;
    }
    (*group).group_res = _cairo_pdf_surface_new_object(surface);
    if (*group).group_res.id == 0 as libc::c_int as libc::c_uint {
        let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        free(group as *mut libc::c_void);
        return 0 as *mut cairo_pdf_smask_group_t;
    }
    (*group).width = (*surface).width;
    (*group).height = (*surface).height;
    if !extents.is_null() {
        (*group).extents = *extents;
    } else {
        (*group).extents.x = 0 as libc::c_int;
        (*group).extents.y = 0 as libc::c_int;
        (*group).extents.width = (*surface).width as libc::c_int;
        (*group).extents.height = (*surface).height as libc::c_int;
    }
    return group;
}
unsafe extern "C" fn _cairo_pdf_smask_group_destroy(
    mut group: *mut cairo_pdf_smask_group_t,
) {
    if (*group).operation as libc::c_uint == PDF_FILL as libc::c_int as libc::c_uint
        || (*group).operation as libc::c_uint
            == PDF_STROKE as libc::c_int as libc::c_uint
    {
        _cairo_path_fixed_fini(&mut (*group).path);
    }
    if !((*group).source).is_null() {
        cairo_pattern_destroy((*group).source);
    }
    if !((*group).mask).is_null() {
        cairo_pattern_destroy((*group).mask);
    }
    free((*group).utf8 as *mut libc::c_void);
    free((*group).glyphs as *mut libc::c_void);
    free((*group).clusters as *mut libc::c_void);
    if !((*group).scaled_font).is_null() {
        cairo_scaled_font_destroy((*group).scaled_font);
    }
    free(group as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_pdf_surface_add_smask_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut group: *mut cairo_pdf_smask_group_t,
) -> cairo_int_status_t {
    return _cairo_array_append(
        &mut (*surface).smask_groups,
        &mut group as *mut *mut cairo_pdf_smask_group_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_source_surface_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_pdf_source_surface_entry_t = key_a
        as *const cairo_pdf_source_surface_entry_t;
    let mut b: *const cairo_pdf_source_surface_entry_t = key_b
        as *const cairo_pdf_source_surface_entry_t;
    if (*a).interpolate != (*b).interpolate {
        return 0 as libc::c_int;
    }
    if !((*a).unique_id).is_null() && !((*b).unique_id).is_null()
        && (*a).unique_id_length == (*b).unique_id_length
    {
        return (memcmp(
            (*a).unique_id as *const libc::c_void,
            (*b).unique_id as *const libc::c_void,
            (*a).unique_id_length,
        ) == 0 as libc::c_int) as libc::c_int;
    }
    return ((*a).id == (*b).id) as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_source_surface_init_key(
    mut key: *mut cairo_pdf_source_surface_entry_t,
) {
    if !((*key).unique_id).is_null()
        && (*key).unique_id_length > 0 as libc::c_int as libc::c_ulong
    {
        (*key)
            .base
            .hash = _cairo_hash_bytes(
            5381 as libc::c_int as uintptr_t,
            (*key).unique_id as *const libc::c_void,
            (*key).unique_id_length as libc::c_uint,
        );
    } else {
        (*key).base.hash = (*key).id as uintptr_t;
    };
}
unsafe extern "C" fn _cairo_pdf_surface_acquire_source_image_from_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut image: *mut *mut cairo_image_surface_t,
    mut image_extra: *mut *mut libc::c_void,
) -> cairo_int_status_t {
    match (*pattern).type_0 as libc::c_uint {
        1 => {
            let mut surf_pat: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            return _cairo_surface_acquire_source_image(
                (*surf_pat).surface,
                image,
                image_extra,
            ) as cairo_int_status_t;
        }
        5 => {
            let mut surf: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
            surf = _cairo_raster_source_pattern_acquire(
                pattern,
                &mut (*surface).base,
                0 as *const cairo_rectangle_int_t,
            );
            if surf.is_null() {
                return CAIRO_INT_STATUS_UNSUPPORTED;
            }
            if _cairo_surface_is_image(surf) != 0 {} else {
                __assert_fail(
                    b"_cairo_surface_is_image (surf)\0" as *const u8
                        as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    1456 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 155],
                        &[libc::c_char; 155],
                    >(
                        b"cairo_int_status_t _cairo_pdf_surface_acquire_source_image_from_pattern(cairo_pdf_surface_t *, const cairo_pattern_t *, cairo_image_surface_t **, void **)\0",
                    ))
                        .as_ptr(),
                );
            }
            *image = surf as *mut cairo_image_surface_t;
        }
        0 | 2 | 3 | 4 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    1465 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 155],
                        &[libc::c_char; 155],
                    >(
                        b"cairo_int_status_t _cairo_pdf_surface_acquire_source_image_from_pattern(cairo_pdf_surface_t *, const cairo_pattern_t *, cairo_image_surface_t **, void **)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_release_source_image_from_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut image: *mut cairo_image_surface_t,
    mut image_extra: *mut libc::c_void,
) {
    match (*pattern).type_0 as libc::c_uint {
        1 => {
            let mut surf_pat: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            _cairo_surface_release_source_image((*surf_pat).surface, image, image_extra);
        }
        5 => {
            _cairo_raster_source_pattern_release(pattern, &mut (*image).base);
        }
        0 | 2 | 3 | 4 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    1494 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 139],
                        &[libc::c_char; 139],
                    >(
                        b"void _cairo_pdf_surface_release_source_image_from_pattern(cairo_pdf_surface_t *, const cairo_pattern_t *, cairo_image_surface_t *, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    };
}
unsafe extern "C" fn _get_source_surface_extents(
    mut source: *mut cairo_surface_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut bounded: *mut cairo_bool_t,
    mut subsurface: *mut cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    *bounded = 1 as libc::c_int;
    *subsurface = 0 as libc::c_int;
    if (*source).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        let mut free_me: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        if _cairo_surface_is_snapshot(source) != 0 {
            source = _cairo_surface_snapshot_get_target(source);
            free_me = source;
        }
        if (*(*source).backend).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
        {
            let mut sub: *mut cairo_surface_subsurface_t = source
                as *mut cairo_surface_subsurface_t;
            *extents = (*sub).extents;
            *subsurface = 1 as libc::c_int;
        } else {
            let mut box_0: cairo_box_t = cairo_box_t {
                p1: cairo_point_t { x: 0, y: 0 },
                p2: cairo_point_t { x: 0, y: 0 },
            };
            *bounded = _cairo_surface_get_extents(source, extents);
            if *bounded == 0 {
                status = _cairo_recording_surface_get_ink_bbox(
                    source as *mut cairo_recording_surface_t,
                    &mut box_0,
                    0 as *const cairo_matrix_t,
                ) as cairo_int_status_t;
                if status as u64 != 0 {
                    cairo_surface_destroy(free_me);
                    return status;
                }
                _cairo_box_round_to_rectangle(&mut box_0, extents);
            }
        }
        cairo_surface_destroy(free_me);
    } else {
        *bounded = _cairo_surface_get_extents(source, extents);
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_source_surface(
    mut surface: *mut cairo_pdf_surface_t,
    mut source_surface: *mut cairo_surface_t,
    mut source_pattern: *const cairo_pattern_t,
    mut op: cairo_operator_t,
    mut filter: cairo_filter_t,
    mut stencil_mask: cairo_bool_t,
    mut smask: cairo_bool_t,
    mut need_transp_group: cairo_bool_t,
    mut extents: *const cairo_rectangle_int_t,
    mut smask_res: *mut cairo_pdf_resource_t,
    mut pdf_source: *mut *mut cairo_pdf_source_surface_entry_t,
    mut x_offset: *mut libc::c_double,
    mut y_offset: *mut libc::c_double,
    mut source_extents: *mut cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut src_surface: cairo_pdf_source_surface_t = cairo_pdf_source_surface_t {
        type_0: CAIRO_PATTERN_TYPE_SOLID,
        surface: 0 as *mut cairo_surface_t,
        raster_pattern: 0 as *mut cairo_pattern_t,
        hash_entry: 0 as *mut cairo_pdf_source_surface_entry_t,
    };
    let mut surface_key: cairo_pdf_source_surface_entry_t = cairo_pdf_source_surface_entry_t {
        base: cairo_hash_entry_t { hash: 0 },
        id: 0,
        unique_id: 0 as *mut libc::c_uchar,
        unique_id_length: 0,
        operator: CAIRO_OPERATOR_CLEAR,
        interpolate: 0,
        stencil_mask: 0,
        smask: 0,
        need_transp_group: 0,
        surface_res: cairo_pdf_resource_t { id: 0 },
        smask_res: cairo_pdf_resource_t { id: 0 },
        emit_image: 0,
        bounded: 0,
        extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        required_extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
    };
    let mut surface_entry: *mut cairo_pdf_source_surface_entry_t = 0
        as *mut cairo_pdf_source_surface_entry_t;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut interpolate: cairo_bool_t = 0;
    let mut unique_id: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut unique_id_length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut op_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut subsurface: cairo_bool_t = 0;
    match filter as libc::c_uint {
        0 | 3 | 5 => {
            interpolate = 0 as libc::c_int;
        }
        1 | 2 | 4 | _ => {
            interpolate = 1 as libc::c_int;
        }
    }
    x = 0 as libc::c_int as libc::c_double;
    y = 0 as libc::c_int as libc::c_double;
    if !source_pattern.is_null() {
        if (*source_pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
        {
            status = _cairo_pdf_surface_acquire_source_image_from_pattern(
                surface,
                source_pattern,
                &mut image,
                &mut image_extra,
            );
            if status as u64 != 0 {
                return status;
            }
            source_surface = &mut (*image).base;
            cairo_surface_get_device_offset(source_surface, &mut x, &mut y);
        } else {
            let mut surface_pattern: *mut cairo_surface_pattern_t = source_pattern
                as *mut cairo_surface_pattern_t;
            source_surface = (*surface_pattern).surface;
        }
    }
    if !x_offset.is_null() {
        *x_offset = x;
    }
    if !y_offset.is_null() {
        *y_offset = y;
    }
    op_extents = *extents;
    if !source_pattern.is_null() {
        _cairo_box_from_rectangle(&mut box_0, extents);
        _cairo_matrix_transform_bounding_box_fixed(
            &(*source_pattern).matrix,
            &mut box_0,
            0 as *mut cairo_bool_t,
        );
        _cairo_box_round_to_rectangle(&mut box_0, &mut op_extents);
    }
    if !source_extents.is_null() {
        *source_extents = op_extents;
    }
    surface_key.id = (*source_surface).unique_id;
    surface_key.interpolate = interpolate;
    cairo_surface_get_mime_data(
        source_surface,
        b"application/x-cairo.uuid\0" as *const u8 as *const libc::c_char,
        &mut surface_key.unique_id as *mut *mut libc::c_uchar
            as *mut *const libc::c_uchar,
        &mut surface_key.unique_id_length,
    );
    _cairo_pdf_source_surface_init_key(&mut surface_key);
    surface_entry = _cairo_hash_table_lookup(
        (*surface).all_surfaces,
        &mut surface_key.base,
    ) as *mut cairo_pdf_source_surface_entry_t;
    if !surface_entry.is_null() {
        if !pdf_source.is_null() {
            *pdf_source = surface_entry;
        }
        if !source_pattern.is_null()
            && (*source_pattern).extend as libc::c_uint
                != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        {
            _cairo_unbounded_rectangle_init(&mut op_extents);
        }
        _cairo_rectangle_intersect(&mut op_extents, &mut (*surface_entry).extents);
        _cairo_rectangle_union(&mut (*surface_entry).required_extents, &mut op_extents);
        status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if status as libc::c_uint != 0 || !surface_entry.is_null() {
        if !source_pattern.is_null()
            && (*source_pattern).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
        {
            _cairo_pdf_surface_release_source_image_from_pattern(
                surface,
                source_pattern,
                image,
                image_extra,
            );
        }
        return status;
    }
    if !(surface_key.unique_id).is_null()
        && surface_key.unique_id_length > 0 as libc::c_int as libc::c_ulong
    {
        unique_id = (if surface_key.unique_id_length != 0 as libc::c_int as libc::c_ulong
        {
            malloc(surface_key.unique_id_length)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_uchar;
        if unique_id.is_null() {
            status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            current_block = 12435726376927362355;
        } else {
            unique_id_length = surface_key.unique_id_length;
            memcpy(
                unique_id as *mut libc::c_void,
                surface_key.unique_id as *const libc::c_void,
                unique_id_length,
            );
            current_block = 17075014677070940716;
        }
    } else {
        unique_id = 0 as *mut libc::c_uchar;
        unique_id_length = 0 as libc::c_int as libc::c_ulong;
        current_block = 17075014677070940716;
    }
    match current_block {
        17075014677070940716 => {
            surface_entry = (if ::std::mem::size_of::<cairo_pdf_source_surface_entry_t>()
                as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            {
                malloc(
                    ::std::mem::size_of::<cairo_pdf_source_surface_entry_t>()
                        as libc::c_ulong,
                )
            } else {
                0 as *mut libc::c_void
            }) as *mut cairo_pdf_source_surface_entry_t;
            if surface_entry.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            } else {
                if !pdf_source.is_null() {
                    *pdf_source = surface_entry;
                }
                (*surface_entry).id = surface_key.id;
                (*surface_entry).operator = op;
                (*surface_entry).interpolate = interpolate;
                (*surface_entry).stencil_mask = stencil_mask;
                (*surface_entry).smask = smask;
                (*surface_entry).need_transp_group = need_transp_group;
                (*surface_entry).unique_id_length = unique_id_length;
                let ref mut fresh19 = (*surface_entry).unique_id;
                *fresh19 = unique_id;
                if !smask_res.is_null() {
                    (*surface_entry).smask_res = *smask_res;
                } else {
                    (*surface_entry).smask_res.id = 0 as libc::c_int as libc::c_uint;
                }
                status = _get_source_surface_extents(
                    source_surface,
                    &mut (*surface_entry).extents,
                    &mut (*surface_entry).bounded,
                    &mut subsurface,
                );
                if !(status as u64 != 0) {
                    if subsurface != 0 {
                        *x_offset = -(*surface_entry).extents.x as libc::c_double;
                        *y_offset = -(*surface_entry).extents.y as libc::c_double;
                    }
                    if !source_pattern.is_null()
                        && (*source_pattern).extend as libc::c_uint
                            != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                    {
                        _cairo_unbounded_rectangle_init(&mut op_extents);
                    }
                    _cairo_rectangle_intersect(
                        &mut op_extents,
                        &mut (*surface_entry).extents,
                    );
                    (*surface_entry).required_extents = op_extents;
                    _cairo_pdf_source_surface_init_key(surface_entry);
                    src_surface.hash_entry = surface_entry;
                    if !source_pattern.is_null()
                        && (*source_pattern).type_0 as libc::c_uint
                            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int
                                as libc::c_uint
                    {
                        src_surface.type_0 = CAIRO_PATTERN_TYPE_RASTER_SOURCE;
                        src_surface.surface = 0 as *mut cairo_surface_t;
                        status = _cairo_pattern_create_copy(
                            &mut src_surface.raster_pattern,
                            source_pattern,
                        ) as cairo_int_status_t;
                        if status as u64 != 0 {
                            current_block = 17634908416286892863;
                        } else {
                            current_block = 14141370668937312244;
                        }
                    } else {
                        src_surface.type_0 = CAIRO_PATTERN_TYPE_SURFACE;
                        src_surface.surface = cairo_surface_reference(source_surface);
                        src_surface.raster_pattern = 0 as *mut cairo_pattern_t;
                        current_block = 14141370668937312244;
                    }
                    match current_block {
                        17634908416286892863 => {}
                        _ => {
                            (*surface_entry)
                                .surface_res = _cairo_pdf_surface_new_object(surface);
                            if (*surface_entry).surface_res.id
                                == 0 as libc::c_int as libc::c_uint
                            {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                    as cairo_int_status_t;
                            } else {
                                status = _cairo_pdf_surface_emit_surface(
                                    surface,
                                    &mut src_surface,
                                    1 as libc::c_int,
                                    &mut (*surface_entry).emit_image,
                                );
                                if !(status as u64 != 0) {
                                    if (*surface_entry).bounded != 0 {
                                        status = _cairo_array_append(
                                            &mut (*surface).page_surfaces,
                                            &mut src_surface as *mut cairo_pdf_source_surface_t
                                                as *const libc::c_void,
                                        ) as cairo_int_status_t;
                                        if status as u64 != 0 {
                                            current_block = 12077261700685387887;
                                        } else {
                                            current_block = 3812947724376655173;
                                        }
                                    } else {
                                        status = _cairo_array_append(
                                            &mut (*surface).doc_surfaces,
                                            &mut src_surface as *mut cairo_pdf_source_surface_t
                                                as *const libc::c_void,
                                        ) as cairo_int_status_t;
                                        if status as u64 != 0 {
                                            current_block = 12077261700685387887;
                                        } else {
                                            current_block = 3812947724376655173;
                                        }
                                    }
                                    match current_block {
                                        12077261700685387887 => {}
                                        _ => {
                                            status = _cairo_hash_table_insert(
                                                (*surface).all_surfaces,
                                                &mut (*surface_entry).base,
                                            ) as cairo_int_status_t;
                                            if !(status as u64 != 0) {
                                                if !source_pattern.is_null()
                                                    && (*source_pattern).type_0 as libc::c_uint
                                                        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int
                                                            as libc::c_uint
                                                {
                                                    _cairo_pdf_surface_release_source_image_from_pattern(
                                                        surface,
                                                        source_pattern,
                                                        image,
                                                        image_extra,
                                                    );
                                                }
                                                return CAIRO_STATUS_SUCCESS as libc::c_int
                                                    as cairo_int_status_t;
                                            }
                                        }
                                    }
                                }
                            }
                            if !source_pattern.is_null()
                                && (*source_pattern).type_0 as libc::c_uint
                                    == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int
                                        as libc::c_uint
                            {
                                cairo_pattern_destroy(src_surface.raster_pattern);
                            } else {
                                cairo_surface_destroy(src_surface.surface);
                            }
                        }
                    }
                }
                free(surface_entry as *mut libc::c_void);
            }
        }
        _ => {}
    }
    if !unique_id.is_null() {
        free(unique_id as *mut libc::c_void);
    }
    if !source_pattern.is_null()
        && (*source_pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        _cairo_pdf_surface_release_source_image_from_pattern(
            surface,
            source_pattern,
            image,
            image_extra,
        );
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_add_pdf_pattern_or_shading(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut op: cairo_operator_t,
    mut extents: *const cairo_rectangle_int_t,
    mut is_shading: cairo_bool_t,
    mut pattern_res: *mut cairo_pdf_resource_t,
    mut gstate_res: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut pdf_pattern: cairo_pdf_pattern_t = cairo_pdf_pattern_t {
        width: 0.,
        height: 0.,
        extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        pattern: 0 as *mut cairo_pattern_t,
        pattern_res: cairo_pdf_resource_t { id: 0 },
        gstate_res: cairo_pdf_resource_t { id: 0 },
        operator: CAIRO_OPERATOR_CLEAR,
        is_shading: 0,
        inverted_y_axis: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    pdf_pattern.is_shading = is_shading;
    pdf_pattern.operator = op;
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        (*pattern_res).id = 0 as libc::c_int as libc::c_uint;
        (*gstate_res).id = 0 as libc::c_int as libc::c_uint;
        return CAIRO_INT_STATUS_SUCCESS;
    }
    status = _cairo_pattern_create_copy(&mut pdf_pattern.pattern, pattern)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    pdf_pattern.pattern_res = _cairo_pdf_surface_new_object(surface);
    if pdf_pattern.pattern_res.id == 0 as libc::c_int as libc::c_uint {
        cairo_pattern_destroy(pdf_pattern.pattern);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    pdf_pattern.gstate_res.id = 0 as libc::c_int as libc::c_uint;
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
        || (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
        || (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        let mut min_alpha: libc::c_double = 0.;
        _cairo_pattern_alpha_range(pattern, &mut min_alpha, 0 as *mut libc::c_double);
        if !(min_alpha
            >= 0xff00 as libc::c_int as libc::c_double
                / 0xffff as libc::c_int as libc::c_double)
        {
            pdf_pattern.gstate_res = _cairo_pdf_surface_new_object(surface);
            if pdf_pattern.gstate_res.id == 0 as libc::c_int as libc::c_uint {
                cairo_pattern_destroy(pdf_pattern.pattern);
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
        }
    }
    pdf_pattern.width = (*surface).width;
    pdf_pattern.height = (*surface).height;
    if !extents.is_null() {
        pdf_pattern.extents = *extents;
    } else {
        pdf_pattern.extents.x = 0 as libc::c_int;
        pdf_pattern.extents.y = 0 as libc::c_int;
        pdf_pattern.extents.width = (*surface).width as libc::c_int;
        pdf_pattern.extents.height = (*surface).height as libc::c_int;
    }
    *pattern_res = pdf_pattern.pattern_res;
    *gstate_res = pdf_pattern.gstate_res;
    pdf_pattern
        .inverted_y_axis = if pdf_pattern.gstate_res.id != 0 {
        1 as libc::c_int
    } else {
        (*surface).in_xobject
    };
    status = _cairo_array_append(
        &mut (*surface).page_patterns,
        &mut pdf_pattern as *mut cairo_pdf_pattern_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        cairo_pattern_destroy(pdf_pattern.pattern);
        return status;
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _get_bbox_from_extents(
    mut extents: *const cairo_rectangle_int_t,
    mut bbox: *mut cairo_box_double_t,
) {
    (*bbox).p1.x = (*extents).x as libc::c_double;
    (*bbox).p1.y = (*extents).y as libc::c_double;
    (*bbox).p2.x = ((*extents).x + (*extents).width) as libc::c_double;
    (*bbox).p2.y = ((*extents).y + (*extents).height) as libc::c_double;
}
unsafe extern "C" fn _cairo_pdf_surface_add_pdf_shading(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut op: cairo_operator_t,
    mut extents: *const cairo_rectangle_int_t,
    mut shading_res: *mut cairo_pdf_resource_t,
    mut gstate_res: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    return _cairo_pdf_surface_add_pdf_pattern_or_shading(
        surface,
        pattern,
        op,
        extents,
        1 as libc::c_int,
        shading_res,
        gstate_res,
    );
}
unsafe extern "C" fn _cairo_pdf_surface_add_pdf_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut op: cairo_operator_t,
    mut extents: *const cairo_rectangle_int_t,
    mut pattern_res: *mut cairo_pdf_resource_t,
    mut gstate_res: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    return _cairo_pdf_surface_add_pdf_pattern_or_shading(
        surface,
        pattern,
        op,
        extents,
        0 as libc::c_int,
        pattern_res,
        gstate_res,
    );
}
unsafe extern "C" fn _cairo_pdf_surface_open_stream(
    mut surface: *mut cairo_pdf_surface_t,
    mut resource: *mut cairo_pdf_resource_t,
    mut compressed: cairo_bool_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> cairo_int_status_t {
    let mut ap: ::std::ffi::VaListImpl;
    let mut self_0: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut length: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut output: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    if !resource.is_null() {
        self_0 = *resource;
        _cairo_pdf_surface_update_object(surface, self_0);
    } else {
        self_0 = _cairo_pdf_surface_new_object(surface);
        if self_0.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    length = _cairo_pdf_surface_new_object(surface);
    if length.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if compressed != 0 {
        output = _cairo_deflate_stream_create((*surface).output);
        if _cairo_output_stream_get_status(output) as u64 != 0 {
            return _cairo_output_stream_destroy(output) as cairo_int_status_t;
        }
    }
    (*surface).pdf_stream.active = 1 as libc::c_int;
    (*surface).pdf_stream.self_0 = self_0;
    (*surface).pdf_stream.length = length;
    (*surface).pdf_stream.compressed = compressed;
    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    (*surface).current_operator = CAIRO_OPERATOR_OVER;
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Length %d 0 R\n\0" as *const u8 as *const libc::c_char,
        (*surface).pdf_stream.self_0.id,
        (*surface).pdf_stream.length.id,
    );
    if compressed != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Filter /FlateDecode\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if !fmt.is_null() {
        ap = args.clone();
        _cairo_output_stream_vprintf((*surface).output, fmt, ap.as_va_list());
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nstream\n\0" as *const u8 as *const libc::c_char,
    );
    (*surface)
        .pdf_stream
        .start_offset = _cairo_output_stream_get_position((*surface).output);
    if compressed != 0 {
        if ((*surface).pdf_stream.old_output).is_null() {} else {
            __assert_fail(
                b"surface->pdf_stream.old_output == NULL\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                1972 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 130],
                    &[libc::c_char; 130],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_open_stream(cairo_pdf_surface_t *, cairo_pdf_resource_t *, cairo_bool_t, const char *, ...)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh20 = (*surface).pdf_stream.old_output;
        *fresh20 = (*surface).output;
        let ref mut fresh21 = (*surface).output;
        *fresh21 = output;
        _cairo_pdf_operators_set_stream(
            &mut (*surface).pdf_operators,
            (*surface).output,
        );
    }
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_close_stream(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut length: libc::c_longlong = 0;
    if (*surface).pdf_stream.active == 0 {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if (*surface).pdf_stream.compressed != 0 {
        let mut status2: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
        status2 = _cairo_output_stream_destroy((*surface).output) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = status2;
        }
        let ref mut fresh22 = (*surface).output;
        *fresh22 = (*surface).pdf_stream.old_output;
        _cairo_pdf_operators_set_stream(
            &mut (*surface).pdf_operators,
            (*surface).output,
        );
        let ref mut fresh23 = (*surface).pdf_stream.old_output;
        *fresh23 = 0 as *mut cairo_output_stream_t;
    }
    length = _cairo_output_stream_get_position((*surface).output)
        - (*surface).pdf_stream.start_offset;
    _cairo_output_stream_printf(
        (*surface).output,
        b"\nendstream\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_update_object(surface, (*surface).pdf_stream.length);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n   %lld\nendobj\n\0" as *const u8 as *const libc::c_char,
        (*surface).pdf_stream.length.id,
        length,
    );
    (*surface).pdf_stream.active = 0 as libc::c_int;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = _cairo_output_stream_get_status((*surface).output)
            as cairo_int_status_t;
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_write_memory_stream(
    mut surface: *mut cairo_pdf_surface_t,
    mut mem_stream: *mut cairo_output_stream_t,
    mut resource: cairo_pdf_resource_t,
    mut resources: *mut cairo_pdf_group_resources_t,
    mut is_knockout_group: cairo_bool_t,
    mut bbox: *const cairo_box_double_t,
) {
    _cairo_pdf_surface_update_object(surface, resource);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /XObject\n   /Length %d\n\0" as *const u8
            as *const libc::c_char,
        resource.id,
        _cairo_memory_stream_length(mem_stream),
    );
    if (*surface).compress_streams != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Filter /FlateDecode\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Subtype /Form\n   /BBox [ %f %f %f %f ]\n   /Group <<\n      /Type /Group\n      /S /Transparency\n      /I true\n      /CS /DeviceRGB\n\0"
            as *const u8 as *const libc::c_char,
        (*bbox).p1.x,
        (*bbox).p1.y,
        (*bbox).p2.x,
        (*bbox).p2.y,
    );
    if is_knockout_group != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"      /K true\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"   >>\n   /Resources\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_emit_group_resources(surface, resources, 1 as libc::c_int);
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nstream\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_memory_stream_copy(mem_stream, (*surface).output);
    _cairo_output_stream_printf(
        (*surface).output,
        b"endstream\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn _cairo_pdf_surface_open_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut bbox: *const cairo_box_double_t,
    mut resource: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).pdf_stream.active == 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->pdf_stream.active == FALSE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2085 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 124],
                &[libc::c_char; 124],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_open_group(cairo_pdf_surface_t *, const cairo_box_double_t *, cairo_pdf_resource_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).group_stream.active == 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->group_stream.active == FALSE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2086 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 124],
                &[libc::c_char; 124],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_open_group(cairo_pdf_surface_t *, const cairo_box_double_t *, cairo_pdf_resource_t *)\0",
            ))
                .as_ptr(),
        );
    }
    (*surface).group_stream.active = 1 as libc::c_int;
    let ref mut fresh24 = (*surface).group_stream.mem_stream;
    *fresh24 = _cairo_memory_stream_create();
    if (*surface).compress_streams != 0 {
        let ref mut fresh25 = (*surface).group_stream.stream;
        *fresh25 = _cairo_deflate_stream_create((*surface).group_stream.mem_stream);
    } else {
        let ref mut fresh26 = (*surface).group_stream.stream;
        *fresh26 = (*surface).group_stream.mem_stream;
    }
    status = _cairo_output_stream_get_status((*surface).group_stream.stream)
        as cairo_int_status_t;
    let ref mut fresh27 = (*surface).group_stream.old_output;
    *fresh27 = (*surface).output;
    let ref mut fresh28 = (*surface).output;
    *fresh28 = (*surface).group_stream.stream;
    _cairo_pdf_operators_set_stream(&mut (*surface).pdf_operators, (*surface).output);
    _cairo_pdf_group_resources_clear(&mut (*surface).resources);
    if !resource.is_null() {
        (*surface).group_stream.resource = *resource;
    } else {
        (*surface).group_stream.resource = _cairo_pdf_surface_new_object(surface);
        if (*surface).group_stream.resource.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
    }
    (*surface).group_stream.is_knockout = 0 as libc::c_int;
    (*surface).group_stream.bbox = *bbox;
    _cairo_output_stream_printf(
        (*surface).output,
        b"/gs0 gs\n\0" as *const u8 as *const libc::c_char,
    );
    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    (*surface).current_operator = CAIRO_OPERATOR_OVER;
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_open_knockout_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut bbox: *const cairo_box_double_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_surface_open_group(
        surface,
        bbox,
        0 as *mut cairo_pdf_resource_t,
    );
    if status as u64 != 0 {
        return status;
    }
    (*surface).group_stream.is_knockout = 1 as libc::c_int;
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_pdf_surface_close_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut group: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut status2: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).pdf_stream.active == 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->pdf_stream.active == FALSE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_close_group(cairo_pdf_surface_t *, cairo_pdf_resource_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).group_stream.active == 1 as libc::c_int {} else {
        __assert_fail(
            b"surface->group_stream.active == TRUE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2146 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 97],
                &[libc::c_char; 97],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_close_group(cairo_pdf_surface_t *, cairo_pdf_resource_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if (*surface).compress_streams != 0 {
        status = _cairo_output_stream_destroy((*surface).group_stream.stream)
            as cairo_int_status_t;
        let ref mut fresh29 = (*surface).group_stream.stream;
        *fresh29 = 0 as *mut cairo_output_stream_t;
        _cairo_output_stream_printf(
            (*surface).group_stream.mem_stream,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
    }
    let ref mut fresh30 = (*surface).output;
    *fresh30 = (*surface).group_stream.old_output;
    _cairo_pdf_operators_set_stream(&mut (*surface).pdf_operators, (*surface).output);
    (*surface).group_stream.active = 0 as libc::c_int;
    _cairo_pdf_surface_write_memory_stream(
        surface,
        (*surface).group_stream.mem_stream,
        (*surface).group_stream.resource,
        &mut (*surface).resources,
        (*surface).group_stream.is_knockout,
        &mut (*surface).group_stream.bbox,
    );
    if !group.is_null() {
        *group = (*surface).group_stream.resource;
    }
    status2 = _cairo_output_stream_destroy((*surface).group_stream.mem_stream)
        as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = status2;
    }
    let ref mut fresh31 = (*surface).group_stream.mem_stream;
    *fresh31 = 0 as *mut cairo_output_stream_t;
    let ref mut fresh32 = (*surface).group_stream.stream;
    *fresh32 = 0 as *mut cairo_output_stream_t;
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_open_object_stream(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    if ((*surface).pdf_version as libc::c_uint)
        < CAIRO_PDF_VERSION_1_5 as libc::c_int as libc::c_uint
    {
        if (*surface).pdf_stream.active == 0 as libc::c_int {} else {
            __assert_fail(
                b"surface->pdf_stream.active == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                2187 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_open_object_stream(cairo_pdf_surface_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*surface).group_stream.active == 0 as libc::c_int {} else {
            __assert_fail(
                b"surface->group_stream.active == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                2188 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_open_object_stream(cairo_pdf_surface_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh33 = (*surface).object_stream.stream;
        *fresh33 = (*surface).output;
    } else {
        (*surface).object_stream.resource = _cairo_pdf_surface_new_object(surface);
        if (*surface).object_stream.resource.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        _cairo_array_truncate(
            &mut (*surface).object_stream.objects,
            0 as libc::c_int as libc::c_uint,
        );
        let ref mut fresh34 = (*surface).object_stream.stream;
        *fresh34 = _cairo_memory_stream_create();
        (*surface).object_stream.active = 1 as libc::c_int;
    }
    return _cairo_output_stream_get_status((*surface).object_stream.stream)
        as cairo_int_status_t;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_surface_object_begin(
    mut surface: *mut cairo_pdf_surface_t,
    mut resource: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut xref_obj: cairo_xref_stream_object_t = cairo_xref_stream_object_t {
        resource: cairo_pdf_resource_t { id: 0 },
        offset: 0,
    };
    let mut object: *mut cairo_pdf_object_t = 0 as *mut cairo_pdf_object_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).object_stream.active != 0 {
        xref_obj.resource = resource;
        xref_obj
            .offset = _cairo_output_stream_get_position((*surface).object_stream.stream);
        status = _cairo_array_append(
            &mut (*surface).object_stream.objects,
            &mut xref_obj as *mut cairo_xref_stream_object_t as *const libc::c_void,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        object = _cairo_array_index(
            &mut (*surface).objects,
            (resource.id).wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_pdf_object_t;
        (*object).type_0 = PDF_OBJECT_COMPRESSED;
        (*object).u.compressed_obj.xref_stream = (*surface).object_stream.resource;
        (*object)
            .u
            .compressed_obj
            .index = (_cairo_array_num_elements(&mut (*surface).object_stream.objects))
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    } else {
        _cairo_pdf_surface_update_object(surface, resource);
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n\0" as *const u8 as *const libc::c_char,
            resource.id,
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_pdf_surface_object_end(
    mut surface: *mut cairo_pdf_surface_t,
) {
    if (*surface).object_stream.active == 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"endobj\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn _cairo_xref_stream_object_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut a_obj: *const cairo_xref_stream_object_t = a
        as *const cairo_xref_stream_object_t;
    let mut b_obj: *const cairo_xref_stream_object_t = b
        as *const cairo_xref_stream_object_t;
    if (*a_obj).offset < (*b_obj).offset {
        return -(1 as libc::c_int)
    } else if (*a_obj).offset > (*b_obj).offset {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn _cairo_pdf_surface_close_object_stream(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut i: libc::c_int = 0;
    let mut num_objects: libc::c_int = 0;
    let mut xref_obj: *mut cairo_xref_stream_object_t = 0
        as *mut cairo_xref_stream_object_t;
    let mut start_pos: libc::c_longlong = 0;
    let mut length: libc::c_longlong = 0;
    let mut index_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut deflate_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut length_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut object: *mut cairo_pdf_object_t = 0 as *mut cairo_pdf_object_t;
    if (*surface).object_stream.active == 0 {
        let ref mut fresh35 = (*surface).object_stream.stream;
        *fresh35 = 0 as *mut cairo_output_stream_t;
        return CAIRO_INT_STATUS_SUCCESS;
    }
    num_objects = _cairo_array_num_elements(&mut (*surface).object_stream.objects)
        as libc::c_int;
    if num_objects == 0 as libc::c_int {
        object = _cairo_array_index(
            &mut (*surface).objects,
            ((*surface).object_stream.resource.id)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut cairo_pdf_object_t;
        (*object).type_0 = PDF_OBJECT_FREE;
        return CAIRO_INT_STATUS_SUCCESS;
    }
    index_stream = _cairo_memory_stream_create();
    _cairo_array_sort(
        &mut (*surface).object_stream.objects,
        Some(
            _cairo_xref_stream_object_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < num_objects {
        xref_obj = _cairo_array_index(
            &mut (*surface).object_stream.objects,
            i as libc::c_uint,
        ) as *mut cairo_xref_stream_object_t;
        _cairo_output_stream_printf(
            index_stream,
            b"%d %lld\n\0" as *const u8 as *const libc::c_char,
            (*xref_obj).resource.id,
            (*xref_obj).offset,
        );
        i += 1;
    }
    length_res = _cairo_pdf_surface_new_object(surface);
    if length_res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_pdf_surface_update_object(surface, (*surface).object_stream.resource);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /ObjStm\n   /Length %d 0 R\n   /N %d\n   /First %d\n\0"
            as *const u8 as *const libc::c_char,
        (*surface).object_stream.resource.id,
        length_res.id,
        num_objects,
        _cairo_memory_stream_length(index_stream),
    );
    if (*surface).compress_streams != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Filter /FlateDecode\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nstream\n\0" as *const u8 as *const libc::c_char,
    );
    start_pos = _cairo_output_stream_get_position((*surface).output);
    if (*surface).compress_streams != 0 {
        deflate_stream = _cairo_deflate_stream_create((*surface).output);
        _cairo_memory_stream_copy(index_stream, deflate_stream);
        _cairo_memory_stream_copy((*surface).object_stream.stream, deflate_stream);
        status = _cairo_output_stream_destroy(deflate_stream) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        length = _cairo_output_stream_get_position((*surface).output) - start_pos;
    } else {
        _cairo_memory_stream_copy(index_stream, (*surface).output);
        _cairo_memory_stream_copy((*surface).object_stream.stream, (*surface).output);
        length = _cairo_output_stream_get_position((*surface).output) - start_pos;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"\nendstream\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_update_object(surface, length_res);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n   %lld\nendobj\n\0" as *const u8 as *const libc::c_char,
        length_res.id,
        length,
    );
    status = _cairo_output_stream_destroy(index_stream) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_output_stream_destroy((*surface).object_stream.stream)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh36 = (*surface).object_stream.stream;
    *fresh36 = 0 as *mut cairo_output_stream_t;
    (*surface).object_stream.active = 0 as libc::c_int;
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_open_content_stream(
    mut surface: *mut cairo_pdf_surface_t,
    mut bbox: *const cairo_box_double_t,
    mut resource: *mut cairo_pdf_resource_t,
    mut is_form: cairo_bool_t,
    mut is_group: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).pdf_stream.active == 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->pdf_stream.active == FALSE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2366 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 161],
                &[libc::c_char; 161],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_open_content_stream(cairo_pdf_surface_t *, const cairo_box_double_t *, cairo_pdf_resource_t *, cairo_bool_t, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).group_stream.active == 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->group_stream.active == FALSE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2367 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 161],
                &[libc::c_char; 161],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_open_content_stream(cairo_pdf_surface_t *, const cairo_box_double_t *, cairo_pdf_resource_t *, cairo_bool_t, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    (*surface).content_resources = _cairo_pdf_surface_new_object(surface);
    if (*surface).content_resources.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    if is_form != 0 {
        if !bbox.is_null() {} else {
            __assert_fail(
                b"bbox != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                2374 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 161],
                    &[libc::c_char; 161],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_open_content_stream(cairo_pdf_surface_t *, const cairo_box_double_t *, cairo_pdf_resource_t *, cairo_bool_t, cairo_bool_t)\0",
                ))
                    .as_ptr(),
            );
        }
        if is_group != 0 {
            status = _cairo_pdf_surface_open_stream(
                surface,
                resource,
                (*surface).compress_streams,
                b"   /Type /XObject\n   /Subtype /Form\n   /BBox [ %f %f %f %f ]\n   /Group <<\n      /Type /Group\n      /S /Transparency\n      /I true\n      /CS /DeviceRGB\n   >>\n   /Resources %d 0 R\n\0"
                    as *const u8 as *const libc::c_char,
                (*bbox).p1.x,
                (*bbox).p1.y,
                (*bbox).p2.x,
                (*bbox).p2.y,
                (*surface).content_resources.id,
            );
        } else {
            status = _cairo_pdf_surface_open_stream(
                surface,
                resource,
                (*surface).compress_streams,
                b"   /Type /XObject\n   /Subtype /Form\n   /BBox [ %f %f %f %f ]\n   /Resources %d 0 R\n\0"
                    as *const u8 as *const libc::c_char,
                (*bbox).p1.x,
                (*bbox).p1.y,
                (*bbox).p2.x,
                (*bbox).p2.y,
                (*surface).content_resources.id,
            );
        }
    } else {
        status = _cairo_pdf_surface_open_stream(
            surface,
            resource,
            (*surface).compress_streams,
            0 as *const libc::c_char,
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"1 0 0 -1 0 %f cm\n\0" as *const u8 as *const libc::c_char,
            (*surface).height,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    (*surface).content = (*surface).pdf_stream.self_0;
    _cairo_output_stream_printf(
        (*surface).output,
        b"q\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_close_content_stream(
    mut surface: *mut cairo_pdf_surface_t,
    mut is_form: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).pdf_stream.active == 1 as libc::c_int {} else {
        __assert_fail(
            b"surface->pdf_stream.active == TRUE\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2438 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_close_content_stream(cairo_pdf_surface_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if (*surface).group_stream.active == 0 as libc::c_int {} else {
        __assert_fail(
            b"surface->group_stream.active == FALSE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2439 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_close_content_stream(cairo_pdf_surface_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"Q\n\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    if status as u64 != 0 {
        return status;
    }
    _cairo_pdf_surface_update_object(surface, (*surface).content_resources);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n\0" as *const u8 as *const libc::c_char,
        (*surface).content_resources.id,
    );
    _cairo_pdf_surface_emit_group_resources(surface, &mut (*surface).resources, is_form);
    _cairo_output_stream_printf(
        (*surface).output,
        b"endobj\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_source_surface_entry_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut surface_entry: *mut cairo_pdf_source_surface_entry_t = entry
        as *mut cairo_pdf_source_surface_entry_t;
    let mut patterns: *mut cairo_hash_table_t = closure as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(patterns, &mut (*surface_entry).base);
    free((*surface_entry).unique_id as *mut libc::c_void);
    free(surface_entry as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_pdf_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut offset: libc::c_longlong = 0;
    let mut catalog: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut doc_surface: cairo_pdf_source_surface_t = cairo_pdf_source_surface_t {
        type_0: CAIRO_PATTERN_TYPE_SOLID,
        surface: 0 as *mut cairo_surface_t,
        raster_pattern: 0 as *mut cairo_pattern_t,
        hash_entry: 0 as *mut cairo_pdf_source_surface_entry_t,
    };
    let mut global: *mut cairo_pdf_jbig2_global_t = 0 as *mut cairo_pdf_jbig2_global_t;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xref_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    if !((*surface).base.status as libc::c_uint
        != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint)
    {
        _cairo_pdf_surface_clear(surface);
        status = _cairo_pdf_surface_open_object_stream(surface) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_pdf_surface_write_patterns_and_smask_groups(surface, 1 as libc::c_int);
        status = (*surface).base.status;
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = _cairo_pdf_surface_emit_font_subsets(surface) as cairo_status_t;
        }
        status = _cairo_pdf_surface_write_pages(surface) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_interchange_write_document_objects(surface)
            as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        catalog = _cairo_pdf_surface_new_object(surface);
        if catalog.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY);
        }
        status = _cairo_pdf_surface_write_catalog(surface, catalog) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_close_object_stream(surface) as cairo_status_t;
        if status as u64 != 0 {
            return status;
        }
        if (*surface).pdf_version as libc::c_uint
            >= CAIRO_PDF_VERSION_1_5 as libc::c_int as libc::c_uint
        {
            xref_res = _cairo_pdf_surface_new_object(surface);
            status = _cairo_pdf_surface_write_xref_stream(
                surface,
                xref_res,
                catalog,
                (*surface).docinfo_res,
                &mut offset,
            ) as cairo_status_t;
        } else {
            offset = _cairo_pdf_surface_write_xref(surface);
            _cairo_output_stream_printf(
                (*surface).output,
                b"trailer\n<< /Size %d\n   /Root %d 0 R\n   /Info %d 0 R\n>>\n\0"
                    as *const u8 as *const libc::c_char,
                (*surface).next_available_resource.id,
                catalog.id,
                (*surface).docinfo_res.id,
            );
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"startxref\n%lld\n%%%%EOF\n\0" as *const u8 as *const libc::c_char,
            offset,
        );
    }
    status2 = _cairo_pdf_operators_fini(&mut (*surface).pdf_operators);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    status2 = _cairo_pdf_surface_close_stream(surface) as cairo_status_t;
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    if !((*surface).group_stream.stream).is_null() {
        status2 = _cairo_output_stream_destroy((*surface).group_stream.stream);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = status2;
        }
    }
    if !((*surface).group_stream.mem_stream).is_null() {
        status2 = _cairo_output_stream_destroy((*surface).group_stream.mem_stream);
        if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            status = status2;
        }
    }
    if (*surface).pdf_stream.active != 0 {
        let ref mut fresh37 = (*surface).output;
        *fresh37 = (*surface).pdf_stream.old_output;
    }
    if (*surface).group_stream.active != 0 {
        let ref mut fresh38 = (*surface).output;
        *fresh38 = (*surface).group_stream.old_output;
    }
    status2 = _cairo_output_stream_destroy((*surface).output);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    _cairo_pdf_group_resources_fini(&mut (*surface).resources);
    _cairo_array_fini(&mut (*surface).objects);
    _cairo_array_fini(&mut (*surface).pages);
    _cairo_array_fini(&mut (*surface).rgb_linear_functions);
    _cairo_array_fini(&mut (*surface).alpha_linear_functions);
    _cairo_array_fini(&mut (*surface).page_patterns);
    _cairo_array_fini(&mut (*surface).page_surfaces);
    _cairo_array_fini(&mut (*surface).object_stream.objects);
    size = _cairo_array_num_elements(&mut (*surface).doc_surfaces) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        _cairo_array_copy_element(
            &mut (*surface).doc_surfaces,
            i as libc::c_uint,
            &mut doc_surface as *mut cairo_pdf_source_surface_t as *mut libc::c_void,
        );
        cairo_surface_destroy(doc_surface.surface);
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).doc_surfaces);
    _cairo_hash_table_foreach(
        (*surface).all_surfaces,
        Some(
            _cairo_pdf_source_surface_entry_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*surface).all_surfaces as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*surface).all_surfaces);
    _cairo_array_fini(&mut (*surface).smask_groups);
    _cairo_array_fini(&mut (*surface).fonts);
    _cairo_array_fini(&mut (*surface).knockout_group);
    _cairo_array_fini(&mut (*surface).page_annots);
    _cairo_array_fini(&mut (*surface).forward_links);
    if !((*surface).font_subsets).is_null() {
        _cairo_scaled_font_subsets_destroy((*surface).font_subsets);
        let ref mut fresh39 = (*surface).font_subsets;
        *fresh39 = 0 as *mut cairo_scaled_font_subsets_t;
    }
    size = _cairo_array_num_elements(&mut (*surface).jbig2_global) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        global = _cairo_array_index(&mut (*surface).jbig2_global, i as libc::c_uint)
            as *mut cairo_pdf_jbig2_global_t;
        free((*global).id as *mut libc::c_void);
        if (*global).emitted == 0 {
            return _cairo_error(CAIRO_STATUS_JBIG2_GLOBAL_MISSING);
        }
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).jbig2_global);
    _cairo_array_fini(&mut (*surface).page_heights);
    size = _cairo_array_num_elements(&mut (*surface).page_labels) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        _cairo_array_copy_element(
            &mut (*surface).page_labels,
            i as libc::c_uint,
            &mut label as *mut *mut libc::c_char as *mut libc::c_void,
        );
        free(label as *mut libc::c_void);
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).page_labels);
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    _cairo_pdf_interchange_fini(surface);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_start_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut page: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).header_emitted == 0 {
        let mut version: *const libc::c_char = 0 as *const libc::c_char;
        match (*surface).pdf_version as libc::c_uint {
            0 => {
                version = b"1.4\0" as *const u8 as *const libc::c_char;
            }
            1 => {
                version = b"1.5\0" as *const u8 as *const libc::c_char;
            }
            2 => {
                version = b"1.6\0" as *const u8 as *const libc::c_char;
            }
            3 | _ => {
                version = b"1.7\0" as *const u8 as *const libc::c_char;
            }
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"%%PDF-%s\n\0" as *const u8 as *const libc::c_char,
            version,
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"%%%c%c%c%c\n\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            237 as libc::c_int,
            174 as libc::c_int,
            251 as libc::c_int,
        );
        (*surface).header_emitted = 1 as libc::c_int;
    }
    _cairo_pdf_group_resources_clear(&mut (*surface).resources);
    (*surface).in_xobject = 0 as libc::c_int;
    page = _cairo_pdf_surface_new_object(surface);
    if page.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    status = _cairo_array_append(
        &mut (*surface).pages,
        &mut page as *mut cairo_pdf_resource_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_has_fallback_images(
    mut abstract_surface: *mut libc::c_void,
    mut has_fallbacks: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut bbox: cairo_box_double_t = cairo_box_double_t {
        p1: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        p2: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
    };
    (*surface).has_fallback_images = has_fallbacks;
    (*surface).in_xobject = has_fallbacks;
    bbox.p1.x = 0 as libc::c_int as libc::c_double;
    bbox.p1.y = 0 as libc::c_int as libc::c_double;
    bbox.p2.x = (*surface).width;
    bbox.p2.y = (*surface).height;
    status = _cairo_pdf_surface_open_content_stream(
        surface,
        &mut bbox,
        0 as *mut cairo_pdf_resource_t,
        has_fallbacks,
        has_fallbacks,
    );
    if status as u64 != 0 {
        return status;
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_supports_fine_grained_fallbacks(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_surface_requires_thumbnail_image(
    mut abstract_surface: *mut libc::c_void,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) -> cairo_bool_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    if (*surface).thumbnail_width > 0 as libc::c_int
        && (*surface).thumbnail_height > 0 as libc::c_int
    {
        *width = (*surface).thumbnail_width;
        *height = (*surface).thumbnail_height;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_surface_set_thumbnail_image(
    mut abstract_surface: *mut libc::c_void,
    mut image: *mut cairo_image_surface_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let ref mut fresh40 = (*surface).thumbnail_image;
    *fresh40 = cairo_surface_reference(&mut (*image).base) as *mut cairo_image_surface_t;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_add_padded_image_surface(
    mut surface: *mut cairo_pdf_surface_t,
    mut source: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut pdf_source: *mut *mut cairo_pdf_source_surface_entry_t,
    mut x_offset: *mut libc::c_double,
    mut y_offset: *mut libc::c_double,
    mut source_extents: *mut cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut pad_image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut pad_pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
        base: cairo_pattern_t {
            ref_count: cairo_reference_count_t {
                ref_count: 0,
            },
            status: CAIRO_STATUS_SUCCESS,
            user_data: cairo_user_data_array_t {
                size: 0,
                num_elements: 0,
                element_size: 0,
                elements: 0 as *mut libc::c_char,
            },
            observers: cairo_list_t {
                next: 0 as *mut _cairo_list,
                prev: 0 as *mut _cairo_list,
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
    status = _cairo_pdf_surface_acquire_source_image_from_pattern(
        surface,
        source,
        &mut image,
        &mut image_extra,
    );
    if status as u64 != 0 {
        return status;
    }
    pad_image = &mut (*image).base;
    _cairo_box_from_rectangle(&mut box_0, extents);
    _cairo_matrix_transform_bounding_box_fixed(
        &(*source).matrix,
        &mut box_0,
        0 as *mut cairo_bool_t,
    );
    _cairo_box_round_to_rectangle(&mut box_0, &mut rect);
    w = (*image).width;
    h = (*image).height;
    if _cairo_fixed_integer_ceil(box_0.p1.x) < 0 as libc::c_int
        || _cairo_fixed_integer_ceil(box_0.p1.y) < 0 as libc::c_int
        || _cairo_fixed_integer_floor(box_0.p2.x) > w
        || _cairo_fixed_integer_floor(box_0.p2.y) > h
    {
        pad_image = _cairo_image_surface_create_with_content(
            (*image).base.content,
            rect.width,
            rect.height,
        );
        if (*pad_image).status as u64 != 0 {
            status = (*pad_image).status as cairo_int_status_t;
            current_block = 5323450097045724464;
        } else {
            _cairo_pattern_init_for_surface(&mut pad_pattern, &mut (*image).base);
            cairo_matrix_init_translate(
                &mut pad_pattern.base.matrix,
                rect.x as libc::c_double,
                rect.y as libc::c_double,
            );
            pad_pattern.base.extend = CAIRO_EXTEND_PAD;
            status = _cairo_surface_paint(
                pad_image,
                CAIRO_OPERATOR_SOURCE,
                &mut pad_pattern.base,
                0 as *const cairo_clip_t,
            ) as cairo_int_status_t;
            _cairo_pattern_fini(&mut pad_pattern.base);
            if status as u64 != 0 {
                current_block = 5323450097045724464;
            } else {
                current_block = 2668756484064249700;
            }
        }
    } else {
        current_block = 2668756484064249700;
    }
    match current_block {
        2668756484064249700 => {
            status = _cairo_pdf_surface_add_source_surface(
                surface,
                pad_image,
                0 as *const cairo_pattern_t,
                CAIRO_OPERATOR_OVER,
                (*source).filter,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                extents,
                0 as *mut cairo_pdf_resource_t,
                pdf_source,
                x_offset,
                y_offset,
                source_extents,
            );
            if !(status as u64 != 0) {
                if pad_image != &mut (*image).base as *mut cairo_surface_t {
                    *x_offset = rect.x as libc::c_double;
                    *y_offset = rect.y as libc::c_double;
                }
            }
        }
        _ => {}
    }
    if pad_image != &mut (*image).base as *mut cairo_surface_t {
        cairo_surface_destroy(pad_image);
    }
    _cairo_pdf_surface_release_source_image_from_pattern(
        surface,
        source,
        image,
        image_extra,
    );
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_smask(
    mut surface: *mut cairo_pdf_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut stencil_mask: cairo_bool_t,
    mut interpolate: cairo_bool_t,
    mut stream_res: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut alpha: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alpha_size: libc::c_ulong = 0;
    let mut pixel32: *mut uint32_t = 0 as *mut uint32_t;
    let mut pixel8: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    if (*image).format as libc::c_int == CAIRO_FORMAT_ARGB32 as libc::c_int
        || (*image).format as libc::c_int == CAIRO_FORMAT_RGB24 as libc::c_int
        || (*image).format as libc::c_int == CAIRO_FORMAT_A8 as libc::c_int
        || (*image).format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int
    {} else {
        __assert_fail(
            b"image->format == CAIRO_FORMAT_ARGB32 || image->format == CAIRO_FORMAT_RGB24 || image->format == CAIRO_FORMAT_A8 || image->format == CAIRO_FORMAT_A1\0"
                as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2856 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 149],
                &[libc::c_char; 149],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_smask(cairo_pdf_surface_t *, cairo_image_surface_t *, cairo_bool_t, cairo_bool_t, cairo_pdf_resource_t *)\0",
            ))
                .as_ptr(),
        );
    }
    transparency = _cairo_image_analyze_transparency(image);
    if stencil_mask != 0 {
        if transparency as libc::c_uint
            == CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
            || transparency as libc::c_uint
                == CAIRO_IMAGE_HAS_BILEVEL_ALPHA as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"transparency == CAIRO_IMAGE_IS_OPAQUE || transparency == CAIRO_IMAGE_HAS_BILEVEL_ALPHA\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                2861 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 149],
                    &[libc::c_char; 149],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_emit_smask(cairo_pdf_surface_t *, cairo_image_surface_t *, cairo_bool_t, cairo_bool_t, cairo_pdf_resource_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    } else if transparency as libc::c_uint
        != CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"transparency != CAIRO_IMAGE_IS_OPAQUE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            2863 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 149],
                &[libc::c_char; 149],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_smask(cairo_pdf_surface_t *, cairo_image_surface_t *, cairo_bool_t, cairo_bool_t, cairo_pdf_resource_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if transparency as libc::c_uint
        == CAIRO_IMAGE_HAS_BILEVEL_ALPHA as libc::c_int as libc::c_uint
        || transparency as libc::c_uint
            == CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
    {
        alpha_size = (((*image).width + 7 as libc::c_int) / 8 as libc::c_int
            * (*image).height) as libc::c_ulong;
        alpha = _cairo_malloc_ab(
            (((*image).width + 7 as libc::c_int) / 8 as libc::c_int) as size_t,
            (*image).height as size_t,
        ) as *mut libc::c_char;
    } else {
        alpha_size = ((*image).height * (*image).width) as libc::c_ulong;
        alpha = _cairo_malloc_ab((*image).height as size_t, (*image).width as size_t)
            as *mut libc::c_char;
    }
    if alpha.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    } else {
        i = 0 as libc::c_int;
        y = 0 as libc::c_int;
        while y < (*image).height {
            if transparency as libc::c_uint
                == CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
            {
                x = 0 as libc::c_int;
                while x < ((*image).width + 7 as libc::c_int) / 8 as libc::c_int {
                    let fresh41 = i;
                    i = i + 1;
                    *alpha
                        .offset(fresh41 as isize) = 0xff as libc::c_int as libc::c_char;
                    x += 1;
                }
            } else if (*image).format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {
                pixel8 = ((*image).data)
                    .offset((y as libc::c_long * (*image).stride) as isize)
                    as *mut uint8_t;
                x = 0 as libc::c_int;
                while x < ((*image).width + 7 as libc::c_int) / 8 as libc::c_int {
                    a = *pixel8 as libc::c_int;
                    a = (((a as libc::c_ulong).wrapping_mul(0x802 as libc::c_ulong)
                        & 0x22110 as libc::c_ulong
                        | (a as libc::c_ulong).wrapping_mul(0x8020 as libc::c_ulong)
                            & 0x88440 as libc::c_ulong)
                        .wrapping_mul(0x10101 as libc::c_ulong) >> 16 as libc::c_int)
                        as libc::c_int;
                    let fresh42 = i;
                    i = i + 1;
                    *alpha.offset(fresh42 as isize) = a as libc::c_char;
                    x += 1;
                    pixel8 = pixel8.offset(1);
                }
            } else {
                pixel8 = ((*image).data)
                    .offset((y as libc::c_long * (*image).stride) as isize)
                    as *mut uint8_t;
                pixel32 = ((*image).data)
                    .offset((y as libc::c_long * (*image).stride) as isize)
                    as *mut uint32_t;
                bit = 7 as libc::c_int;
                x = 0 as libc::c_int;
                while x < (*image).width {
                    if (*image).format as libc::c_int
                        == CAIRO_FORMAT_ARGB32 as libc::c_int
                    {
                        a = ((*pixel32 & 0xff000000 as libc::c_uint)
                            >> 24 as libc::c_int) as libc::c_int;
                        pixel32 = pixel32.offset(1);
                    } else {
                        a = *pixel8 as libc::c_int;
                        pixel8 = pixel8.offset(1);
                    }
                    if transparency as libc::c_uint
                        == CAIRO_IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
                    {
                        let fresh43 = i;
                        i = i + 1;
                        *alpha.offset(fresh43 as isize) = a as libc::c_char;
                    } else {
                        if bit == 7 as libc::c_int {
                            *alpha.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                        }
                        if a != 0 as libc::c_int {
                            let ref mut fresh44 = *alpha.offset(i as isize);
                            *fresh44 = (*fresh44 as libc::c_int
                                | (1 as libc::c_int) << bit) as libc::c_char;
                        }
                        bit -= 1;
                        if bit < 0 as libc::c_int {
                            bit = 7 as libc::c_int;
                            i += 1;
                        }
                    }
                    x += 1;
                }
                if bit != 7 as libc::c_int {
                    i += 1;
                }
            }
            y += 1;
        }
        if stencil_mask != 0 {
            status = _cairo_pdf_surface_open_stream(
                surface,
                stream_res,
                1 as libc::c_int,
                b"   /Type /XObject\n   /Subtype /Image\n   /ImageMask true\n   /Width %d\n   /Height %d\n   /Interpolate %s\n   /BitsPerComponent 1\n   /Decode [1 0]\n\0"
                    as *const u8 as *const libc::c_char,
                (*image).width,
                (*image).height,
                if interpolate != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        } else {
            status = _cairo_pdf_surface_open_stream(
                surface,
                stream_res,
                1 as libc::c_int,
                b"   /Type /XObject\n   /Subtype /Image\n   /Width %d\n   /Height %d\n   /ColorSpace /DeviceGray\n   /Interpolate %s\n   /BitsPerComponent %d\n\0"
                    as *const u8 as *const libc::c_char,
                (*image).width,
                (*image).height,
                if interpolate != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
                if transparency as libc::c_uint
                    == CAIRO_IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
                {
                    8 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            );
        }
        if !(status as u64 != 0) {
            _cairo_output_stream_write(
                (*surface).output,
                alpha as *const libc::c_void,
                alpha_size,
            );
            status = _cairo_pdf_surface_close_stream(surface);
        }
        free(alpha as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_image(
    mut surface: *mut cairo_pdf_surface_t,
    mut image_surf: *mut cairo_image_surface_t,
    mut surface_entry: *mut cairo_pdf_source_surface_entry_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_size: libc::c_ulong = 0;
    let mut pixel: *mut uint32_t = 0 as *mut uint32_t;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut smask: cairo_pdf_resource_t = {
        let mut init = _cairo_pdf_resource {
            id: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    let mut need_smask: cairo_bool_t = 0;
    let mut color: cairo_image_color_t = CAIRO_IMAGE_IS_COLOR;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    let mut smask_buf: [libc::c_char; 30] = [0; 30];
    image = image_surf;
    if (*image).format as libc::c_int != CAIRO_FORMAT_RGB24 as libc::c_int
        && (*image).format as libc::c_int != CAIRO_FORMAT_ARGB32 as libc::c_int
        && (*image).format as libc::c_int != CAIRO_FORMAT_A8 as libc::c_int
        && (*image).format as libc::c_int != CAIRO_FORMAT_A1 as libc::c_int
    {
        let mut surf: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        let mut pattern: cairo_surface_pattern_t = cairo_surface_pattern_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        surf = _cairo_image_surface_create_with_content(
            (*image_surf).base.content,
            (*image_surf).width,
            (*image_surf).height,
        );
        image = surf as *mut cairo_image_surface_t;
        if (*surf).status as u64 != 0 {
            status = (*surf).status as cairo_int_status_t;
            current_block = 2108473268176108562;
        } else {
            _cairo_pattern_init_for_surface(&mut pattern, &mut (*image_surf).base);
            status = _cairo_surface_paint(
                surf,
                CAIRO_OPERATOR_SOURCE,
                &mut pattern.base,
                0 as *const cairo_clip_t,
            ) as cairo_int_status_t;
            _cairo_pattern_fini(&mut pattern.base);
            if status as u64 != 0 {
                current_block = 2108473268176108562;
            } else {
                current_block = 8457315219000651999;
            }
        }
    } else {
        current_block = 8457315219000651999;
    }
    match current_block {
        8457315219000651999 => {
            if (*surface_entry).smask != 0 || (*surface_entry).stencil_mask != 0 {
                return _cairo_pdf_surface_emit_smask(
                    surface,
                    image,
                    (*surface_entry).stencil_mask,
                    (*surface_entry).interpolate,
                    &mut (*surface_entry).surface_res,
                );
            }
            color = _cairo_image_analyze_color(image);
            let mut current_block_19: u64;
            match color as libc::c_uint {
                0 => {
                    current_block_19 = 10173742499954295087;
                }
                1 => {
                    data_size = ((*image).height * (*image).width) as libc::c_ulong;
                    data = _cairo_malloc_ab(
                        (*image).width as size_t,
                        (*image).height as size_t,
                    ) as *mut libc::c_char;
                    current_block_19 = 17500079516916021833;
                }
                2 => {
                    data_size = (((*image).width + 7 as libc::c_int) / 8 as libc::c_int
                        * (*image).height) as libc::c_ulong;
                    data = _cairo_malloc_ab(
                        (((*image).width + 7 as libc::c_int) / 8 as libc::c_int)
                            as size_t,
                        (*image).height as size_t,
                    ) as *mut libc::c_char;
                    current_block_19 = 17500079516916021833;
                }
                3 | _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-pdf-surface.c\0" as *const u8
                                as *const libc::c_char,
                            3031 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 133],
                                &[libc::c_char; 133],
                            >(
                                b"cairo_int_status_t _cairo_pdf_surface_emit_image(cairo_pdf_surface_t *, cairo_image_surface_t *, cairo_pdf_source_surface_entry_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    current_block_19 = 10173742499954295087;
                }
            }
            match current_block_19 {
                10173742499954295087 => {
                    data_size = ((*image).height * (*image).width * 3 as libc::c_int)
                        as libc::c_ulong;
                    data = _cairo_malloc_abc(
                        (*image).width as size_t,
                        (*image).height as size_t,
                        3 as libc::c_int as size_t,
                    ) as *mut libc::c_char;
                }
                _ => {}
            }
            if data.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            } else {
                i = 0 as libc::c_int;
                y = 0 as libc::c_int;
                while y < (*image).height {
                    pixel = ((*image).data)
                        .offset((y as libc::c_long * (*image).stride) as isize)
                        as *mut uint32_t;
                    bit = 7 as libc::c_int;
                    x = 0 as libc::c_int;
                    while x < (*image).width {
                        let mut r: libc::c_int = 0;
                        let mut g: libc::c_int = 0;
                        let mut b: libc::c_int = 0;
                        if (*image).format as libc::c_int
                            == CAIRO_FORMAT_ARGB32 as libc::c_int
                        {
                            let mut a: uint8_t = 0;
                            a = ((*pixel & 0xff000000 as libc::c_uint)
                                >> 24 as libc::c_int) as uint8_t;
                            if a as libc::c_int == 0 as libc::c_int {
                                b = 0 as libc::c_int;
                                g = b;
                                r = g;
                            } else {
                                r = ((*pixel & 0xff0000 as libc::c_int as libc::c_uint)
                                    >> 16 as libc::c_int)
                                    .wrapping_mul(255 as libc::c_int as libc::c_uint)
                                    .wrapping_add(
                                        (a as libc::c_int / 2 as libc::c_int) as libc::c_uint,
                                    )
                                    .wrapping_div(a as libc::c_uint) as libc::c_int;
                                g = ((*pixel & 0xff00 as libc::c_int as libc::c_uint)
                                    >> 8 as libc::c_int)
                                    .wrapping_mul(255 as libc::c_int as libc::c_uint)
                                    .wrapping_add(
                                        (a as libc::c_int / 2 as libc::c_int) as libc::c_uint,
                                    )
                                    .wrapping_div(a as libc::c_uint) as libc::c_int;
                                b = ((*pixel & 0xff as libc::c_int as libc::c_uint)
                                    >> 0 as libc::c_int)
                                    .wrapping_mul(255 as libc::c_int as libc::c_uint)
                                    .wrapping_add(
                                        (a as libc::c_int / 2 as libc::c_int) as libc::c_uint,
                                    )
                                    .wrapping_div(a as libc::c_uint) as libc::c_int;
                            }
                        } else if (*image).format as libc::c_int
                            == CAIRO_FORMAT_RGB24 as libc::c_int
                        {
                            r = ((*pixel & 0xff0000 as libc::c_int as libc::c_uint)
                                >> 16 as libc::c_int) as libc::c_int;
                            g = ((*pixel & 0xff00 as libc::c_int as libc::c_uint)
                                >> 8 as libc::c_int) as libc::c_int;
                            b = ((*pixel & 0xff as libc::c_int as libc::c_uint)
                                >> 0 as libc::c_int) as libc::c_int;
                        } else {
                            b = 0 as libc::c_int;
                            g = b;
                            r = g;
                        }
                        match color as libc::c_uint {
                            0 | 3 => {
                                let fresh45 = i;
                                i = i + 1;
                                *data.offset(fresh45 as isize) = r as libc::c_char;
                                let fresh46 = i;
                                i = i + 1;
                                *data.offset(fresh46 as isize) = g as libc::c_char;
                                let fresh47 = i;
                                i = i + 1;
                                *data.offset(fresh47 as isize) = b as libc::c_char;
                            }
                            1 => {
                                let fresh48 = i;
                                i = i + 1;
                                *data.offset(fresh48 as isize) = r as libc::c_char;
                            }
                            2 => {
                                if bit == 7 as libc::c_int {
                                    *data.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                                }
                                if r != 0 as libc::c_int {
                                    let ref mut fresh49 = *data.offset(i as isize);
                                    *fresh49 = (*fresh49 as libc::c_int
                                        | (1 as libc::c_int) << bit) as libc::c_char;
                                }
                                bit -= 1;
                                if bit < 0 as libc::c_int {
                                    bit = 7 as libc::c_int;
                                    i += 1;
                                }
                            }
                            _ => {}
                        }
                        x += 1;
                        pixel = pixel.offset(1);
                    }
                    if bit != 7 as libc::c_int {
                        i += 1;
                    }
                    y += 1;
                }
                if (*surface_entry).smask_res.id != 0 as libc::c_int as libc::c_uint {
                    need_smask = 1 as libc::c_int;
                    smask = (*surface_entry).smask_res;
                    current_block = 16313536926714486912;
                } else {
                    need_smask = 0 as libc::c_int;
                    if (*image).format as libc::c_int
                        == CAIRO_FORMAT_ARGB32 as libc::c_int
                        || (*image).format as libc::c_int
                            == CAIRO_FORMAT_A8 as libc::c_int
                        || (*image).format as libc::c_int
                            == CAIRO_FORMAT_A1 as libc::c_int
                    {
                        transparency = _cairo_image_analyze_transparency(image);
                        if transparency as libc::c_uint
                            != CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
                        {
                            need_smask = 1 as libc::c_int;
                            smask = _cairo_pdf_surface_new_object(surface);
                            if smask.id == 0 as libc::c_int as libc::c_uint {
                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                    as cairo_int_status_t;
                                current_block = 4998797234622538045;
                            } else {
                                status = _cairo_pdf_surface_emit_smask(
                                    surface,
                                    image,
                                    0 as libc::c_int,
                                    (*surface_entry).interpolate,
                                    &mut smask,
                                );
                                if status as u64 != 0 {
                                    current_block = 4998797234622538045;
                                } else {
                                    current_block = 16313536926714486912;
                                }
                            }
                        } else {
                            current_block = 16313536926714486912;
                        }
                    } else {
                        current_block = 16313536926714486912;
                    }
                }
                match current_block {
                    16313536926714486912 => {
                        if need_smask != 0 {
                            snprintf(
                                smask_buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 30]>()
                                    as libc::c_ulong,
                                b"   /SMask %d 0 R\n\0" as *const u8 as *const libc::c_char,
                                smask.id,
                            );
                        } else {
                            smask_buf[0 as libc::c_int
                                as usize] = 0 as libc::c_int as libc::c_char;
                        }
                        status = _cairo_pdf_surface_open_stream(
                            surface,
                            &mut (*surface_entry).surface_res
                                as *mut cairo_pdf_resource_t,
                            1 as libc::c_int,
                            b"   /Type /XObject\n   /Subtype /Image\n   /Width %d\n   /Height %d\n   /ColorSpace %s\n   /Interpolate %s\n   /BitsPerComponent %d\n%s\0"
                                as *const u8 as *const libc::c_char,
                            (*image).width,
                            (*image).height,
                            if color as libc::c_uint
                                == CAIRO_IMAGE_IS_COLOR as libc::c_int as libc::c_uint
                            {
                                b"/DeviceRGB\0" as *const u8 as *const libc::c_char
                            } else {
                                b"/DeviceGray\0" as *const u8 as *const libc::c_char
                            },
                            if (*surface_entry).interpolate != 0 {
                                b"true\0" as *const u8 as *const libc::c_char
                            } else {
                                b"false\0" as *const u8 as *const libc::c_char
                            },
                            if color as libc::c_uint
                                == CAIRO_IMAGE_IS_MONOCHROME as libc::c_int as libc::c_uint
                            {
                                1 as libc::c_int
                            } else {
                                8 as libc::c_int
                            },
                            smask_buf.as_mut_ptr(),
                        );
                        if !(status as u64 != 0) {
                            _cairo_output_stream_write(
                                (*surface).output,
                                data as *const libc::c_void,
                                data_size,
                            );
                            status = _cairo_pdf_surface_close_stream(surface);
                        }
                    }
                    _ => {}
                }
                free(data as *mut libc::c_void);
            }
        }
        _ => {}
    }
    if image != image_surf {
        cairo_surface_destroy(&mut (*image).base);
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_lookup_jbig2_global(
    mut surface: *mut cairo_pdf_surface_t,
    mut global_id: *const libc::c_uchar,
    mut global_id_length: libc::c_ulong,
    mut entry: *mut *mut cairo_pdf_jbig2_global_t,
) -> cairo_int_status_t {
    let mut global: cairo_pdf_jbig2_global_t = cairo_pdf_jbig2_global_t {
        id: 0 as *mut libc::c_uchar,
        id_length: 0,
        res: cairo_pdf_resource_t { id: 0 },
        emitted: 0,
    };
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    size = _cairo_array_num_elements(&mut (*surface).jbig2_global) as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        *entry = _cairo_array_index(&mut (*surface).jbig2_global, i as libc::c_uint)
            as *mut cairo_pdf_jbig2_global_t;
        if !((**entry).id).is_null() && !global_id.is_null()
            && (**entry).id_length == global_id_length
            && memcmp(
                (**entry).id as *const libc::c_void,
                global_id as *const libc::c_void,
                global_id_length,
            ) == 0 as libc::c_int
        {
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        i += 1;
    }
    global
        .id = (if global_id_length != 0 as libc::c_int as libc::c_ulong {
        malloc(global_id_length)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if (global.id).is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    memcpy(
        global.id as *mut libc::c_void,
        global_id as *const libc::c_void,
        global_id_length,
    );
    global.id_length = global_id_length;
    global.res = _cairo_pdf_surface_new_object(surface);
    if global.res.id == 0 as libc::c_int as libc::c_uint {
        free(global.id as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    global.emitted = 0 as libc::c_int;
    status = _cairo_array_append(
        &mut (*surface).jbig2_global,
        &mut global as *mut cairo_pdf_jbig2_global_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    size = _cairo_array_num_elements(&mut (*surface).jbig2_global) as libc::c_int;
    *entry = _cairo_array_index(
        &mut (*surface).jbig2_global,
        (size - 1 as libc::c_int) as libc::c_uint,
    ) as *mut cairo_pdf_jbig2_global_t;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_jbig2_image(
    mut surface: *mut cairo_pdf_surface_t,
    mut source: *mut cairo_surface_t,
    mut surface_entry: *mut cairo_pdf_source_surface_entry_t,
    mut test: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut mime_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let mut info: cairo_image_info_t = cairo_image_info_t {
        width: 0,
        height: 0,
        num_components: 0,
        bits_per_component: 0,
    };
    let mut global_id: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut global_id_length: libc::c_ulong = 0;
    let mut global_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut global_data_length: libc::c_ulong = 0;
    let mut global_entry: *mut cairo_pdf_jbig2_global_t = 0
        as *mut cairo_pdf_jbig2_global_t;
    let mut smask_buf: [libc::c_char; 30] = [0; 30];
    let mut decode_parms_buf: [libc::c_char; 100] = [0; 100];
    cairo_surface_get_mime_data(
        source,
        b"application/x-cairo.jbig2\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_image_info_get_jbig2_info(&mut info, mime_data, mime_data_length);
    if status as u64 != 0 {
        return status;
    }
    if test != 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    cairo_surface_get_mime_data(
        source,
        b"application/x-cairo.jbig2-global-id\0" as *const u8 as *const libc::c_char,
        &mut global_id,
        &mut global_id_length,
    );
    if !global_id.is_null() && global_id_length > 0 as libc::c_int as libc::c_ulong {
        status = _cairo_pdf_surface_lookup_jbig2_global(
            surface,
            global_id,
            global_id_length,
            &mut global_entry,
        );
        if status as u64 != 0 {
            return status;
        }
        if (*global_entry).emitted == 0 {
            cairo_surface_get_mime_data(
                source,
                b"application/x-cairo.jbig2-global\0" as *const u8
                    as *const libc::c_char,
                &mut global_data,
                &mut global_data_length,
            );
            if !global_data.is_null() {
                status = _cairo_pdf_surface_open_stream(
                    surface,
                    &mut (*global_entry).res as *mut cairo_pdf_resource_t,
                    0 as libc::c_int,
                    0 as *const libc::c_char,
                );
                if status as u64 != 0 {
                    return status;
                }
                _cairo_output_stream_write(
                    (*surface).output,
                    global_data as *const libc::c_void,
                    global_data_length,
                );
                status = _cairo_pdf_surface_close_stream(surface);
                if status as u64 != 0 {
                    return status;
                }
                (*global_entry).emitted = 1 as libc::c_int;
            }
        }
        snprintf(
            decode_parms_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            b"   /DecodeParms << /JBIG2Globals %d 0 R >>\n\0" as *const u8
                as *const libc::c_char,
            (*global_entry).res.id,
        );
    } else {
        decode_parms_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    if (*surface_entry).smask_res.id != 0 {
        snprintf(
            smask_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            b"   /SMask %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface_entry).smask_res.id,
        );
    } else {
        smask_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    if (*surface_entry).stencil_mask != 0 {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /ImageMask true\n   /Width %d\n   /Height %d\n   /Interpolate %s\n   /BitsPerComponent 1\n   /Decode [1 0]\n   /Filter /JPXDecode\n%s\0"
                as *const u8 as *const libc::c_char,
            info.width,
            info.height,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            decode_parms_buf.as_mut_ptr(),
        );
    } else {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /Width %d\n   /Height %d\n   /ColorSpace /DeviceGray\n   /BitsPerComponent 1\n   /Interpolate %s\n%s   /Filter /JBIG2Decode\n%s\0"
                as *const u8 as *const libc::c_char,
            info.width,
            info.height,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            smask_buf.as_mut_ptr(),
            decode_parms_buf.as_mut_ptr(),
        );
    }
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_write(
        (*surface).output,
        mime_data as *const libc::c_void,
        mime_data_length,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_jpx_image(
    mut surface: *mut cairo_pdf_surface_t,
    mut source: *mut cairo_surface_t,
    mut surface_entry: *mut cairo_pdf_source_surface_entry_t,
    mut test: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut mime_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let mut info: cairo_image_info_t = cairo_image_info_t {
        width: 0,
        height: 0,
        num_components: 0,
        bits_per_component: 0,
    };
    let mut smask_buf: [libc::c_char; 30] = [0; 30];
    if ((*surface).pdf_version as libc::c_uint)
        < CAIRO_PDF_VERSION_1_5 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    cairo_surface_get_mime_data(
        source,
        b"image/jp2\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_image_info_get_jpx_info(&mut info, mime_data, mime_data_length);
    if status as u64 != 0 {
        return status;
    }
    if ((*surface_entry).smask != 0 || (*surface_entry).stencil_mask != 0)
        && info.num_components != 1 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*surface_entry).stencil_mask != 0 && info.bits_per_component != 1 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*surface_entry).smask_res.id != 0 {
        snprintf(
            smask_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            b"   /SMask %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface_entry).smask_res.id,
        );
    } else {
        smask_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    if test != 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if (*surface_entry).stencil_mask != 0 {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /ImageMask true\n   /Width %d\n   /Height %d\n   /Interpolate %s\n   /BitsPerComponent 1\n   /Decode [1 0]\n   /Filter /JPXDecode\n\0"
                as *const u8 as *const libc::c_char,
            info.width,
            info.height,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /Width %d\n   /Height %d\n   /Interpolate %s\n%s   /Filter /JPXDecode\n\0"
                as *const u8 as *const libc::c_char,
            info.width,
            info.height,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            smask_buf.as_mut_ptr(),
        );
    }
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_write(
        (*surface).output,
        mime_data as *const libc::c_void,
        mime_data_length,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_jpeg_image(
    mut surface: *mut cairo_pdf_surface_t,
    mut source: *mut cairo_surface_t,
    mut surface_entry: *mut cairo_pdf_source_surface_entry_t,
    mut test: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut mime_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let mut info: cairo_image_info_t = cairo_image_info_t {
        width: 0,
        height: 0,
        num_components: 0,
        bits_per_component: 0,
    };
    let mut colorspace: *const libc::c_char = 0 as *const libc::c_char;
    let mut smask_buf: [libc::c_char; 30] = [0; 30];
    cairo_surface_get_mime_data(
        source,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if (*source).status as u64 != 0 {
        return (*source).status as cairo_int_status_t;
    }
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_image_info_get_jpeg_info(&mut info, mime_data, mime_data_length);
    if status as u64 != 0 {
        return status;
    }
    if ((*surface_entry).smask != 0 || (*surface_entry).stencil_mask != 0)
        && info.num_components != 1 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*surface_entry).stencil_mask != 0 && info.bits_per_component != 1 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    match info.num_components {
        1 => {
            colorspace = b"/DeviceGray\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            colorspace = b"/DeviceRGB\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            colorspace = b"/DeviceCMYK\0" as *const u8 as *const libc::c_char;
        }
        _ => return CAIRO_INT_STATUS_UNSUPPORTED,
    }
    if test != 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if (*surface_entry).smask_res.id != 0 {
        snprintf(
            smask_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            b"   /SMask %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface_entry).smask_res.id,
        );
    } else {
        smask_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    if (*surface_entry).stencil_mask != 0 {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /ImageMask true\n   /Width %d\n   /Height %d\n   /Interpolate %s\n   /BitsPerComponent 1\n   /Decode [1 0]\n   /Filter /DCTDecode\n\0"
                as *const u8 as *const libc::c_char,
            info.width,
            info.height,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /Width %d\n   /Height %d\n   /ColorSpace %s\n   /Interpolate %s\n   /BitsPerComponent %d\n%s   /Filter /DCTDecode\n\0"
                as *const u8 as *const libc::c_char,
            info.width,
            info.height,
            colorspace,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            info.bits_per_component,
            smask_buf.as_mut_ptr(),
        );
    }
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_write(
        (*surface).output,
        mime_data as *const libc::c_void,
        mime_data_length,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_ccitt_image(
    mut surface: *mut cairo_pdf_surface_t,
    mut source: *mut cairo_surface_t,
    mut surface_entry: *mut cairo_pdf_source_surface_entry_t,
    mut test: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut ccitt_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ccitt_data_len: libc::c_ulong = 0;
    let mut ccitt_params_string: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ccitt_params_string_len: libc::c_ulong = 0;
    let mut params: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccitt_params: cairo_ccitt_params_t = cairo_ccitt_params_t {
        columns: 0,
        rows: 0,
        k: 0,
        end_of_line: 0,
        encoded_byte_align: 0,
        end_of_block: 0,
        black_is_1: 0,
        damaged_rows_before_error: 0,
    };
    let mut buf: [libc::c_char; 300] = [0; 300];
    cairo_surface_get_mime_data(
        source,
        b"image/g3fax\0" as *const u8 as *const libc::c_char,
        &mut ccitt_data,
        &mut ccitt_data_len,
    );
    if (*source).status as u64 != 0 {
        return (*source).status as cairo_int_status_t;
    }
    if ccitt_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    cairo_surface_get_mime_data(
        source,
        b"application/x-cairo.ccitt.params\0" as *const u8 as *const libc::c_char,
        &mut ccitt_params_string,
        &mut ccitt_params_string_len,
    );
    if (*source).status as u64 != 0 {
        return (*source).status as cairo_int_status_t;
    }
    if ccitt_params_string.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    params = malloc(
        ccitt_params_string_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        params as *mut libc::c_void,
        ccitt_params_string as *const libc::c_void,
        ccitt_params_string_len,
    );
    *params.offset(ccitt_params_string_len as isize) = 0 as libc::c_int as libc::c_char;
    status = _cairo_tag_parse_ccitt_params(params, &mut ccitt_params) as cairo_status_t;
    if status as u64 != 0 {
        return (*source).status as cairo_int_status_t;
    }
    free(params as *mut libc::c_void);
    if test != 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    p = buf.as_mut_ptr();
    *p = 0 as libc::c_int as libc::c_char;
    end = buf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 300]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    p = p
        .offset(
            snprintf(
                p,
                end.offset_from(p) as libc::c_long as libc::c_ulong,
                b"/Columns %d /Rows %d /K %d\0" as *const u8 as *const libc::c_char,
                ccitt_params.columns,
                ccitt_params.rows,
                ccitt_params.k,
            ) as isize,
        );
    if ccitt_params.end_of_line != 0 {
        p = p
            .offset(
                snprintf(
                    p,
                    end.offset_from(p) as libc::c_long as libc::c_ulong,
                    b" /EndOfLine true\0" as *const u8 as *const libc::c_char,
                ) as isize,
            );
    }
    if ccitt_params.encoded_byte_align != 0 {
        p = p
            .offset(
                snprintf(
                    p,
                    end.offset_from(p) as libc::c_long as libc::c_ulong,
                    b" /EncodedByteAlign true\0" as *const u8 as *const libc::c_char,
                ) as isize,
            );
    }
    if ccitt_params.end_of_block == 0 {
        p = p
            .offset(
                snprintf(
                    p,
                    end.offset_from(p) as libc::c_long as libc::c_ulong,
                    b" /EndOfBlock false\0" as *const u8 as *const libc::c_char,
                ) as isize,
            );
    }
    if ccitt_params.black_is_1 != 0 {
        p = p
            .offset(
                snprintf(
                    p,
                    end.offset_from(p) as libc::c_long as libc::c_ulong,
                    b" /BlackIs1 true\0" as *const u8 as *const libc::c_char,
                ) as isize,
            );
    }
    if ccitt_params.damaged_rows_before_error > 0 as libc::c_int {
        p = p
            .offset(
                snprintf(
                    p,
                    end.offset_from(p) as libc::c_long as libc::c_ulong,
                    b" /DamagedRowsBeforeError %d\0" as *const u8 as *const libc::c_char,
                    ccitt_params.damaged_rows_before_error,
                ) as isize,
            );
    }
    if (*surface_entry).stencil_mask != 0 {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /ImageMask true\n   /Width %d\n   /Height %d\n   /Interpolate %s\n   /BitsPerComponent 1\n   /Decode [1 0]\n   /Filter /CCITTFaxDecode\n   /DecodeParms << %s >> \0"
                as *const u8 as *const libc::c_char,
            ccitt_params.columns,
            ccitt_params.rows,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            buf.as_mut_ptr(),
        ) as cairo_status_t;
    } else {
        status = _cairo_pdf_surface_open_stream(
            surface,
            &mut (*surface_entry).surface_res as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            b"   /Type /XObject\n   /Subtype /Image\n   /Width %d\n   /Height %d\n   /ColorSpace /DeviceGray\n   /BitsPerComponent 1\n   /Interpolate %s\n   /Filter /CCITTFaxDecode\n   /DecodeParms << %s >> \0"
                as *const u8 as *const libc::c_char,
            ccitt_params.columns,
            ccitt_params.rows,
            if (*surface_entry).interpolate != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            buf.as_mut_ptr(),
        ) as cairo_status_t;
    }
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_output_stream_write(
        (*surface).output,
        ccitt_data as *const libc::c_void,
        ccitt_data_len,
    );
    status = _cairo_pdf_surface_close_stream(surface) as cairo_status_t;
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_recording_surface(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_source: *mut cairo_pdf_source_surface_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut old_surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut old_surface_bounded: cairo_bool_t = 0;
    let mut old_paginated_mode: cairo_paginated_mode_t = CAIRO_PAGINATED_MODE_ANALYZE;
    let mut old_clipper: cairo_surface_clipper_t = cairo_surface_clipper_t {
        clip: 0 as *mut cairo_clip_t,
        intersect_clip_path: None,
    };
    let mut old_in_xobject: cairo_bool_t = 0;
    let mut bbox: cairo_box_double_t = cairo_box_double_t {
        p1: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        p2: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut alpha: libc::c_int = 0 as libc::c_int;
    let mut free_me: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut source: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut extents: *const cairo_rectangle_int_t = 0 as *const cairo_rectangle_int_t;
    let mut is_subsurface: cairo_bool_t = 0;
    let mut transparency_group: cairo_bool_t = 0;
    let mut recording: *mut cairo_recording_surface_t = 0
        as *mut cairo_recording_surface_t;
    if (*pdf_source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"pdf_source->type == CAIRO_PATTERN_TYPE_SURFACE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            3639 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_recording_surface(cairo_pdf_surface_t *, cairo_pdf_source_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(*pdf_source).hash_entry).bounded != 0 {
        extents = &mut (*(*pdf_source).hash_entry).extents;
    } else {
        extents = &mut (*(*pdf_source).hash_entry).required_extents;
    }
    is_subsurface = 0 as libc::c_int;
    source = (*pdf_source).surface;
    if _cairo_surface_is_snapshot(source) != 0 {
        source = _cairo_surface_snapshot_get_target(source);
        free_me = source;
    }
    if (*(*source).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
    {
        let mut sub: *mut cairo_surface_subsurface_t = source
            as *mut cairo_surface_subsurface_t;
        source = (*sub).target;
        extents = &mut (*sub).extents;
        is_subsurface = 1 as libc::c_int;
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"source->type == CAIRO_SURFACE_TYPE_RECORDING\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            3660 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_recording_surface(cairo_pdf_surface_t *, cairo_pdf_source_surface_t *)\0",
            ))
                .as_ptr(),
        );
    }
    recording = source as *mut cairo_recording_surface_t;
    old_in_xobject = (*surface).in_xobject;
    old_surface_extents = (*surface).surface_extents;
    old_surface_bounded = (*surface).surface_bounded;
    old_paginated_mode = (*surface).paginated_mode;
    old_clipper = (*surface).clipper;
    (*surface).surface_extents = *extents;
    _cairo_surface_clipper_init(
        &mut (*surface).clipper,
        Some(
            _cairo_pdf_surface_clipper_intersect_clip_path
                as unsafe extern "C" fn(
                    *mut cairo_surface_clipper_t,
                    *mut cairo_path_fixed_t,
                    cairo_fill_rule_t,
                    libc::c_double,
                    cairo_antialias_t,
                ) -> cairo_status_t,
        ),
    );
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    (*surface).in_xobject = 1 as libc::c_int;
    (*surface).surface_extents = *extents;
    (*surface).surface_bounded = 1 as libc::c_int;
    (*surface).paginated_mode = CAIRO_PAGINATED_MODE_RENDER;
    _cairo_pdf_group_resources_clear(&mut (*surface).resources);
    _get_bbox_from_extents(extents, &mut bbox);
    transparency_group = ((*(*pdf_source).hash_entry).need_transp_group != 0
        || !((*(*pdf_source).hash_entry).operator as libc::c_uint
            == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
            && _cairo_recording_surface_has_only_bilevel_alpha(recording) != 0
            && _cairo_recording_surface_has_only_op_over(recording) != 0))
        as libc::c_int;
    status = _cairo_pdf_surface_open_content_stream(
        surface,
        &mut bbox,
        &mut (*(*pdf_source).hash_entry).surface_res,
        1 as libc::c_int,
        transparency_group,
    );
    if !(status as u64 != 0) {
        _cairo_output_stream_printf(
            (*surface).output,
            b"/gs0 gs\n\0" as *const u8 as *const libc::c_char,
        );
        if (*source).content as libc::c_uint
            == CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
        {
            status = _cairo_pdf_surface_add_alpha(surface, 1.0f64, &mut alpha);
            if status as u64 != 0 {
                current_block = 13720137178327677166;
            } else {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"q /a%d gs 0 0 0 rg %d %d %d %d re f Q\n\0" as *const u8
                        as *const libc::c_char,
                    alpha,
                    (*extents).x,
                    (*extents).y,
                    (*extents).width,
                    (*extents).height,
                );
                current_block = 10758786907990354186;
            }
        } else {
            current_block = 10758786907990354186;
        }
        match current_block {
            13720137178327677166 => {}
            _ => {
                status = _cairo_recording_surface_replay_region(
                    source,
                    if is_subsurface != 0 {
                        extents
                    } else {
                        0 as *const cairo_rectangle_int_t
                    },
                    &mut (*surface).base,
                    CAIRO_RECORDING_REGION_NATIVE,
                ) as cairo_int_status_t;
                if status as libc::c_uint
                    != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"status != CAIRO_INT_STATUS_UNSUPPORTED\0" as *const u8
                            as *const libc::c_char,
                        b"../src/cairo-pdf-surface.c\0" as *const u8
                            as *const libc::c_char,
                        3725 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 114],
                            &[libc::c_char; 114],
                        >(
                            b"cairo_int_status_t _cairo_pdf_surface_emit_recording_surface(cairo_pdf_surface_t *, cairo_pdf_source_surface_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if !(status as u64 != 0) {
                    status = _cairo_pdf_surface_close_content_stream(
                        surface,
                        1 as libc::c_int,
                    );
                    _cairo_surface_clipper_reset(&mut (*surface).clipper);
                    (*surface).clipper = old_clipper;
                    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
                    (*surface).in_xobject = old_in_xobject;
                    (*surface).paginated_mode = old_paginated_mode;
                    (*surface).surface_extents = old_surface_extents;
                    (*surface).surface_bounded = old_surface_bounded;
                }
            }
        }
    }
    cairo_surface_destroy(free_me);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_surface(
    mut surface: *mut cairo_pdf_surface_t,
    mut source: *mut cairo_pdf_source_surface_t,
    mut test: cairo_bool_t,
    mut is_image: *mut cairo_bool_t,
) -> cairo_int_status_t {
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        status = _cairo_pdf_surface_emit_jbig2_image(
            surface,
            (*source).surface,
            (*source).hash_entry,
            test,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            *is_image = 1 as libc::c_int;
            return status;
        }
        status = _cairo_pdf_surface_emit_jpx_image(
            surface,
            (*source).surface,
            (*source).hash_entry,
            test,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            *is_image = 1 as libc::c_int;
            return status;
        }
        status = _cairo_pdf_surface_emit_jpeg_image(
            surface,
            (*source).surface,
            (*source).hash_entry,
            test,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            *is_image = 1 as libc::c_int;
            return status;
        }
        status = _cairo_pdf_surface_emit_ccitt_image(
            surface,
            (*source).surface,
            (*source).hash_entry,
            test,
        );
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            *is_image = 1 as libc::c_int;
            return status;
        }
        if (*(*source).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
        {
            if test != 0 {
                *is_image = 0 as libc::c_int;
                return CAIRO_INT_STATUS_SUCCESS;
            } else {
                return _cairo_pdf_surface_emit_recording_surface(surface, source)
            }
        }
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        status = _cairo_surface_acquire_source_image(
            (*source).surface,
            &mut image,
            &mut image_extra,
        ) as cairo_int_status_t;
    } else {
        status = _cairo_pdf_surface_acquire_source_image_from_pattern(
            surface,
            (*source).raster_pattern,
            &mut image,
            &mut image_extra,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    if test != 0 {
        *is_image = 1 as libc::c_int;
    } else {
        status = _cairo_pdf_surface_emit_image(surface, image, (*source).hash_entry);
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        _cairo_surface_release_source_image((*source).surface, image, image_extra);
    } else {
        _cairo_pdf_surface_release_source_image_from_pattern(
            surface,
            (*source).raster_pattern,
            image,
            image_extra,
        );
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_surface_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_pattern: *mut cairo_pdf_pattern_t,
) -> cairo_int_status_t {
    let mut pattern: *mut cairo_pattern_t = (*pdf_pattern).pattern;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut cairo_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut pdf_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut extend: cairo_extend_t = cairo_pattern_get_extend(pattern);
    let mut xstep: libc::c_double = 0.;
    let mut ystep: libc::c_double = 0.;
    let mut pattern_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut x_offset: libc::c_double = 0.;
    let mut y_offset: libc::c_double = 0.;
    let mut draw_surface: [libc::c_char; 50] = [0; 50];
    let mut draw_surface2: [libc::c_char; 200] = [0; 200];
    let mut bbox: cairo_box_double_t = cairo_box_double_t {
        p1: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        p2: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
    };
    let mut mat: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut pdf_source: *mut cairo_pdf_source_surface_entry_t = 0
        as *mut cairo_pdf_source_surface_entry_t;
    let mut op_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"pattern->type == CAIRO_PATTERN_TYPE_SURFACE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            3866 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_surface_pattern(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*pattern).extend as libc::c_uint
        == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
    {
        status = _cairo_pdf_surface_add_padded_image_surface(
            surface,
            pattern,
            &mut (*pdf_pattern).extents,
            &mut pdf_source,
            &mut x_offset,
            &mut y_offset,
            &mut op_extents,
        );
    } else {
        status = _cairo_pdf_surface_add_source_surface(
            surface,
            0 as *mut cairo_surface_t,
            pattern,
            (*pdf_pattern).operator,
            (*pattern).filter,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            &mut (*pdf_pattern).extents,
            0 as *mut cairo_pdf_resource_t,
            &mut pdf_source,
            &mut x_offset,
            &mut y_offset,
            &mut op_extents,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    pattern_extents = (*pdf_source).extents;
    if (*pdf_source).bounded == 0 {
        extend = CAIRO_EXTEND_NONE;
        _cairo_rectangle_intersect(&mut pattern_extents, &mut op_extents);
    }
    match extend as libc::c_uint {
        3 | 0 => {
            let mut x1: libc::c_double = 0.0f64;
            let mut y1: libc::c_double = 0.0f64;
            let mut x2: libc::c_double = (*surface).surface_extents.width
                as libc::c_double;
            let mut y2: libc::c_double = (*surface).surface_extents.height
                as libc::c_double;
            _cairo_matrix_transform_bounding_box(
                &mut (*pattern).matrix,
                &mut x1,
                &mut y1,
                &mut x2,
                &mut y2,
                0 as *mut cairo_bool_t,
            );
            ystep = ceil(
                x2 - x1 + (y2 - y1) + pattern_extents.width as libc::c_double
                    + pattern_extents.height as libc::c_double,
            );
            xstep = ystep;
        }
        1 => {
            xstep = pattern_extents.width as libc::c_double;
            ystep = pattern_extents.height as libc::c_double;
        }
        2 => {
            pattern_extents.width *= 2 as libc::c_int;
            pattern_extents.height *= 2 as libc::c_int;
            xstep = pattern_extents.width as libc::c_double;
            ystep = pattern_extents.height as libc::c_double;
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    3947 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 105],
                        &[libc::c_char; 105],
                    >(
                        b"cairo_int_status_t _cairo_pdf_surface_emit_surface_pattern(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            xstep = 0 as libc::c_int as libc::c_double;
            ystep = 0 as libc::c_int as libc::c_double;
        }
    }
    cairo_p2d = (*pattern).matrix;
    status = cairo_matrix_invert(&mut cairo_p2d) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            3982 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_surface_pattern(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*pdf_pattern).inverted_y_axis != 0 {
        cairo_matrix_init(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
    } else {
        cairo_matrix_init(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
            0 as libc::c_int as libc::c_double,
            (*surface).height,
        );
    }
    cairo_matrix_multiply(&mut pdf_p2d, &mut cairo_p2d, &mut mat);
    cairo_matrix_translate(&mut pdf_p2d, x_offset, y_offset);
    if (*pdf_source).emit_image != 0 {
        cairo_matrix_translate(
            &mut pdf_p2d,
            0.0f64,
            (*pdf_source).extents.height as libc::c_double,
        );
        cairo_matrix_scale(&mut pdf_p2d, 1.0f64, -1.0f64);
    }
    _get_bbox_from_extents(&mut pattern_extents, &mut bbox);
    _cairo_pdf_surface_update_object(surface, (*pdf_pattern).pattern_res);
    status = _cairo_pdf_surface_open_stream(
        surface,
        &mut (*pdf_pattern).pattern_res as *mut cairo_pdf_resource_t,
        0 as libc::c_int,
        b"   /PatternType 1\n   /BBox [ %f %f %f %f ]\n   /XStep %f\n   /YStep %f\n   /TilingType 1\n   /PaintType 1\n   /Matrix [ %f %f %f %f %f %f ]\n   /Resources << /XObject << /x%d %d 0 R >> >>\n\0"
            as *const u8 as *const libc::c_char,
        bbox.p1.x,
        bbox.p1.y,
        bbox.p2.x,
        bbox.p2.y,
        xstep,
        ystep,
        pdf_p2d.xx,
        pdf_p2d.yx,
        pdf_p2d.xy,
        pdf_p2d.yy,
        pdf_p2d.x0,
        pdf_p2d.y0,
        (*pdf_source).surface_res.id,
        (*pdf_source).surface_res.id,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*pdf_source).emit_image != 0 {
        snprintf(
            draw_surface.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"q %d 0 0 %d 0 0 cm /x%d Do Q\0" as *const u8 as *const libc::c_char,
            (*pdf_source).extents.width,
            (*pdf_source).extents.height,
            (*pdf_source).surface_res.id,
        );
    } else {
        snprintf(
            draw_surface.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"/x%d Do\0" as *const u8 as *const libc::c_char,
            (*pdf_source).surface_res.id,
        );
    }
    if extend as libc::c_uint == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint {
        let mut p_extents: cairo_rectangle_int_t = (*pdf_source).extents;
        snprintf(
            draw_surface2.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong,
            b"%d %d %d %d re W n %s\0" as *const u8 as *const libc::c_char,
            p_extents.x,
            p_extents.y,
            p_extents.width,
            p_extents.height,
            draw_surface.as_mut_ptr(),
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"q %s Q\n\0" as *const u8 as *const libc::c_char,
            draw_surface2.as_mut_ptr(),
        );
        cairo_matrix_init_translate(
            &mut mat,
            p_extents.x as libc::c_double,
            p_extents.y as libc::c_double,
        );
        cairo_matrix_scale(
            &mut mat,
            -(1 as libc::c_int) as libc::c_double,
            1 as libc::c_int as libc::c_double,
        );
        cairo_matrix_translate(
            &mut mat,
            (-(2 as libc::c_int) * p_extents.width) as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
        cairo_matrix_translate(
            &mut mat,
            -p_extents.x as libc::c_double,
            -p_extents.y as libc::c_double,
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"q \0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).output, &mut mat);
        _cairo_output_stream_printf(
            (*surface).output,
            b" cm %s Q\n\0" as *const u8 as *const libc::c_char,
            draw_surface2.as_mut_ptr(),
        );
        cairo_matrix_init_translate(
            &mut mat,
            p_extents.x as libc::c_double,
            p_extents.y as libc::c_double,
        );
        cairo_matrix_scale(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
        );
        cairo_matrix_translate(
            &mut mat,
            0 as libc::c_int as libc::c_double,
            (-(2 as libc::c_int) * p_extents.height) as libc::c_double,
        );
        cairo_matrix_translate(
            &mut mat,
            -p_extents.x as libc::c_double,
            -p_extents.y as libc::c_double,
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"q \0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).output, &mut mat);
        _cairo_output_stream_printf(
            (*surface).output,
            b" cm %s Q\n\0" as *const u8 as *const libc::c_char,
            draw_surface2.as_mut_ptr(),
        );
        cairo_matrix_init_translate(
            &mut mat,
            p_extents.x as libc::c_double,
            p_extents.y as libc::c_double,
        );
        cairo_matrix_scale(
            &mut mat,
            -(1 as libc::c_int) as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
        );
        cairo_matrix_translate(
            &mut mat,
            (-(2 as libc::c_int) * p_extents.width) as libc::c_double,
            (-(2 as libc::c_int) * p_extents.height) as libc::c_double,
        );
        cairo_matrix_translate(
            &mut mat,
            -p_extents.x as libc::c_double,
            -p_extents.y as libc::c_double,
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"q \0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).output, &mut mat);
        _cairo_output_stream_printf(
            (*surface).output,
            b" cm %s Q\n\0" as *const u8 as *const libc::c_char,
            draw_surface2.as_mut_ptr(),
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).output,
            b" %s \n\0" as *const u8 as *const libc::c_char,
            draw_surface.as_mut_ptr(),
        );
    }
    status = _cairo_pdf_surface_close_stream(surface);
    if status as u64 != 0 {
        return status;
    }
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_surface_emit_rgb_linear_function(
    mut surface: *mut cairo_pdf_surface_t,
    mut stop1: *mut cairo_pdf_color_stop_t,
    mut stop2: *mut cairo_pdf_color_stop_t,
    mut function: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut elem: cairo_pdf_rgb_linear_function_t = cairo_pdf_rgb_linear_function_t {
        resource: cairo_pdf_resource_t { id: 0 },
        color1: [0.; 3],
        color2: [0.; 3],
    };
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    num_elems = _cairo_array_num_elements(&mut (*surface).rgb_linear_functions)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        _cairo_array_copy_element(
            &mut (*surface).rgb_linear_functions,
            i as libc::c_uint,
            &mut elem as *mut cairo_pdf_rgb_linear_function_t as *mut libc::c_void,
        );
        if !(memcmp(
            &mut *(elem.color1).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_double as *const libc::c_void,
            &mut *((*stop1).color).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_double as *const libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong),
        ) != 0 as libc::c_int)
        {
            if !(memcmp(
                &mut *(elem.color2).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut libc::c_double as *const libc::c_void,
                &mut *((*stop2).color).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut libc::c_double as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            ) != 0 as libc::c_int)
            {
                *function = elem.resource;
                return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            }
        }
        i += 1;
    }
    res = _cairo_pdf_surface_new_object(surface);
    if res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /FunctionType 2\n   /Domain [ 0 1 ]\n   /C0 [ %f %f %f ]\n   /C1 [ %f %f %f ]\n   /N 1\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        res.id,
        (*stop1).color[0 as libc::c_int as usize],
        (*stop1).color[1 as libc::c_int as usize],
        (*stop1).color[2 as libc::c_int as usize],
        (*stop2).color[0 as libc::c_int as usize],
        (*stop2).color[1 as libc::c_int as usize],
        (*stop2).color[2 as libc::c_int as usize],
    );
    elem.resource = res;
    memcpy(
        &mut *(elem.color1).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_double as *mut libc::c_void,
        &mut *((*stop1).color).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_double as *const libc::c_void,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        &mut *(elem.color2).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_double as *mut libc::c_void,
        &mut *((*stop2).color).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_double as *const libc::c_void,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    );
    status = _cairo_array_append(
        &mut (*surface).rgb_linear_functions,
        &mut elem as *mut cairo_pdf_rgb_linear_function_t as *const libc::c_void,
    ) as cairo_int_status_t;
    *function = res;
    return status;
}
unsafe extern "C" fn cairo_pdf_surface_emit_alpha_linear_function(
    mut surface: *mut cairo_pdf_surface_t,
    mut stop1: *mut cairo_pdf_color_stop_t,
    mut stop2: *mut cairo_pdf_color_stop_t,
    mut function: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut num_elems: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut elem: cairo_pdf_alpha_linear_function_t = cairo_pdf_alpha_linear_function_t {
        resource: cairo_pdf_resource_t { id: 0 },
        alpha1: 0.,
        alpha2: 0.,
    };
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    num_elems = _cairo_array_num_elements(&mut (*surface).alpha_linear_functions)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_elems {
        _cairo_array_copy_element(
            &mut (*surface).alpha_linear_functions,
            i as libc::c_uint,
            &mut elem as *mut cairo_pdf_alpha_linear_function_t as *mut libc::c_void,
        );
        if !(elem.alpha1 != (*stop1).color[3 as libc::c_int as usize]) {
            if !(elem.alpha2 != (*stop2).color[3 as libc::c_int as usize]) {
                *function = elem.resource;
                return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            }
        }
        i += 1;
    }
    res = _cairo_pdf_surface_new_object(surface);
    if res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /FunctionType 2\n   /Domain [ 0 1 ]\n   /C0 [ %f ]\n   /C1 [ %f ]\n   /N 1\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        res.id,
        (*stop1).color[3 as libc::c_int as usize],
        (*stop2).color[3 as libc::c_int as usize],
    );
    elem.resource = res;
    elem.alpha1 = (*stop1).color[3 as libc::c_int as usize];
    elem.alpha2 = (*stop2).color[3 as libc::c_int as usize];
    status = _cairo_array_append(
        &mut (*surface).alpha_linear_functions,
        &mut elem as *mut cairo_pdf_alpha_linear_function_t as *const libc::c_void,
    ) as cairo_int_status_t;
    *function = res;
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_stitched_colorgradient(
    mut surface: *mut cairo_pdf_surface_t,
    mut n_stops: libc::c_uint,
    mut stops: *mut cairo_pdf_color_stop_t,
    mut is_alpha: cairo_bool_t,
    mut function: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut i: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        if is_alpha != 0 {
            status = cairo_pdf_surface_emit_alpha_linear_function(
                surface,
                &mut *stops.offset(i as isize),
                &mut *stops
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
                &mut (*stops.offset(i as isize)).resource,
            );
            if status as u64 != 0 {
                return status;
            }
        } else {
            status = cairo_pdf_surface_emit_rgb_linear_function(
                surface,
                &mut *stops.offset(i as isize),
                &mut *stops
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
                &mut (*stops.offset(i as isize)).resource,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        i = i.wrapping_add(1);
    }
    res = _cairo_pdf_surface_new_object(surface);
    if res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /FunctionType 3\n   /Domain [ %f %f ]\n\0" as *const u8
            as *const libc::c_char,
        res.id,
        (*stops.offset(0 as libc::c_int as isize)).offset,
        (*stops.offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .offset,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Functions [ \0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 R \0" as *const u8 as *const libc::c_char,
            (*stops.offset(i as isize)).resource.id,
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Bounds [ \0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        _cairo_output_stream_printf(
            (*surface).output,
            b"%f \0" as *const u8 as *const libc::c_char,
            (*stops.offset(i as isize)).offset,
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Encode [ \0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < n_stops {
        _cairo_output_stream_printf(
            (*surface).output,
            b"0 1 \0" as *const u8 as *const libc::c_char,
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    *function = res;
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn calc_gradient_color(
    mut new_stop: *mut cairo_pdf_color_stop_t,
    mut stop1: *mut cairo_pdf_color_stop_t,
    mut stop2: *mut cairo_pdf_color_stop_t,
) {
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_double = (*stop1).offset
        / ((*stop1).offset + 1.0f64 - (*stop2).offset);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*new_stop)
            .color[i
            as usize] = (*stop1).color[i as usize]
            + offset * ((*stop2).color[i as usize] - (*stop1).color[i as usize]);
        i += 1;
    }
}
unsafe extern "C" fn _cairo_pdf_surface_emit_pattern_stops(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *mut cairo_gradient_pattern_t,
    mut color_function: *mut cairo_pdf_resource_t,
    mut alpha_function: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut allstops: *mut cairo_pdf_color_stop_t = 0 as *mut cairo_pdf_color_stop_t;
    let mut stops: *mut cairo_pdf_color_stop_t = 0 as *mut cairo_pdf_color_stop_t;
    let mut n_stops: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut emit_alpha: cairo_bool_t = 0 as libc::c_int;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    (*color_function).id = 0 as libc::c_int as libc::c_uint;
    (*alpha_function).id = 0 as libc::c_int as libc::c_uint;
    allstops = _cairo_malloc_ab(
        ((*pattern).n_stops).wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<cairo_pdf_color_stop_t>() as libc::c_ulong,
    ) as *mut cairo_pdf_color_stop_t;
    if allstops.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    stops = &mut *allstops.offset(1 as libc::c_int as isize)
        as *mut cairo_pdf_color_stop_t;
    n_stops = (*pattern).n_stops;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n_stops {
        (*stops.offset(i as isize))
            .color[0 as libc::c_int
            as usize] = (*((*pattern).stops).offset(i as isize)).color.red;
        (*stops.offset(i as isize))
            .color[1 as libc::c_int
            as usize] = (*((*pattern).stops).offset(i as isize)).color.green;
        (*stops.offset(i as isize))
            .color[2 as libc::c_int
            as usize] = (*((*pattern).stops).offset(i as isize)).color.blue;
        (*stops.offset(i as isize))
            .color[3 as libc::c_int
            as usize] = (*((*pattern).stops).offset(i as isize)).color.alpha;
        if !((*stops.offset(i as isize)).color[3 as libc::c_int as usize]
            >= 0xff00 as libc::c_int as libc::c_double
                / 0xffff as libc::c_int as libc::c_double)
        {
            emit_alpha = 1 as libc::c_int;
        }
        (*stops.offset(i as isize))
            .offset = (*((*pattern).stops).offset(i as isize)).offset;
        i = i.wrapping_add(1);
    }
    if (*pattern).base.extend as libc::c_uint
        == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
        || (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
    {
        if (*stops.offset(0 as libc::c_int as isize)).offset > 1e-6f64 {
            if (*pattern).base.extend as libc::c_uint
                == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
            {
                memcpy(
                    allstops as *mut libc::c_void,
                    stops as *const libc::c_void,
                    ::std::mem::size_of::<cairo_pdf_color_stop_t>() as libc::c_ulong,
                );
            } else {
                calc_gradient_color(
                    &mut *allstops.offset(0 as libc::c_int as isize),
                    &mut *stops.offset(0 as libc::c_int as isize),
                    &mut *stops
                        .offset(
                            n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ),
                );
            }
            stops = allstops;
            n_stops = n_stops.wrapping_add(1);
        }
        (*stops.offset(0 as libc::c_int as isize)).offset = 0.0f64;
        if (*stops
            .offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .offset < 1.0f64 - 1e-6f64
        {
            if (*pattern).base.extend as libc::c_uint
                == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
            {
                memcpy(
                    &mut *stops.offset(n_stops as isize) as *mut cairo_pdf_color_stop_t
                        as *mut libc::c_void,
                    &mut *stops
                        .offset(
                            n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) as *mut cairo_pdf_color_stop_t as *const libc::c_void,
                    ::std::mem::size_of::<cairo_pdf_color_stop_t>() as libc::c_ulong,
                );
            } else {
                calc_gradient_color(
                    &mut *stops.offset(n_stops as isize),
                    &mut *stops.offset(0 as libc::c_int as isize),
                    &mut *stops
                        .offset(
                            n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ),
                );
            }
            n_stops = n_stops.wrapping_add(1);
        }
        (*stops.offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .offset = 1.0f64;
    }
    if (*stops.offset(0 as libc::c_int as isize)).offset
        == (*stops
            .offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .offset
    {
        let mut pad_stops: [cairo_pdf_color_stop_t; 4] = [cairo_pdf_color_stop_t {
            offset: 0.,
            color: [0.; 4],
            resource: cairo_pdf_resource_t { id: 0 },
        }; 4];
        if (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"pattern->base.extend == CAIRO_EXTEND_PAD\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                4354 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 156],
                    &[libc::c_char; 156],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_emit_pattern_stops(cairo_pdf_surface_t *, cairo_gradient_pattern_t *, cairo_pdf_resource_t *, cairo_pdf_resource_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        pad_stops[1 as libc::c_int as usize] = *stops.offset(0 as libc::c_int as isize);
        pad_stops[0 as libc::c_int as usize] = pad_stops[1 as libc::c_int as usize];
        pad_stops[3 as libc::c_int
            as usize] = *stops
            .offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        pad_stops[2 as libc::c_int as usize] = pad_stops[3 as libc::c_int as usize];
        pad_stops[0 as libc::c_int as usize].offset = 0 as libc::c_int as libc::c_double;
        pad_stops[3 as libc::c_int as usize].offset = 1 as libc::c_int as libc::c_double;
        status = _cairo_pdf_surface_emit_stitched_colorgradient(
            surface,
            4 as libc::c_int as libc::c_uint,
            pad_stops.as_mut_ptr(),
            0 as libc::c_int,
            color_function,
        );
        if !(status as u64 != 0) {
            if emit_alpha != 0 {
                status = _cairo_pdf_surface_emit_stitched_colorgradient(
                    surface,
                    4 as libc::c_int as libc::c_uint,
                    pad_stops.as_mut_ptr(),
                    1 as libc::c_int,
                    alpha_function,
                );
                status as u64 != 0;
            }
        }
    } else if n_stops == 2 as libc::c_int as libc::c_uint {
        status = cairo_pdf_surface_emit_rgb_linear_function(
            surface,
            &mut *stops.offset(0 as libc::c_int as isize),
            &mut *stops
                .offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize),
            color_function,
        );
        if !(status as u64 != 0) {
            if emit_alpha != 0 {
                status = cairo_pdf_surface_emit_alpha_linear_function(
                    surface,
                    &mut *stops.offset(0 as libc::c_int as isize),
                    &mut *stops
                        .offset(
                            n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ),
                    alpha_function,
                );
                status as u64 != 0;
            }
        }
    } else {
        status = _cairo_pdf_surface_emit_stitched_colorgradient(
            surface,
            n_stops,
            stops,
            0 as libc::c_int,
            color_function,
        );
        if !(status as u64 != 0) {
            if emit_alpha != 0 {
                status = _cairo_pdf_surface_emit_stitched_colorgradient(
                    surface,
                    n_stops,
                    stops,
                    1 as libc::c_int,
                    alpha_function,
                );
                status as u64 != 0;
            }
        }
    }
    free(allstops as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_repeating_function(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *mut cairo_gradient_pattern_t,
    mut function: *mut cairo_pdf_resource_t,
    mut begin: libc::c_int,
    mut end: libc::c_int,
) -> cairo_int_status_t {
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut i: libc::c_int = 0;
    res = _cairo_pdf_surface_new_object(surface);
    if res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /FunctionType 3\n   /Domain [ %d %d ]\n\0" as *const u8
            as *const libc::c_char,
        res.id,
        begin,
        end,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Functions [ \0" as *const u8 as *const libc::c_char,
    );
    i = begin;
    while i < end {
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 R \0" as *const u8 as *const libc::c_char,
            (*function).id,
        );
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Bounds [ \0" as *const u8 as *const libc::c_char,
    );
    i = begin + 1 as libc::c_int;
    while i < end {
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d \0" as *const u8 as *const libc::c_char,
            i,
        );
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Encode [ \0" as *const u8 as *const libc::c_char,
    );
    i = begin;
    while i < end {
        if i % 2 as libc::c_int != 0
            && (*pattern).base.extend as libc::c_uint
                == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
        {
            _cairo_output_stream_printf(
                (*surface).output,
                b"1 0 \0" as *const u8 as *const libc::c_char,
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).output,
                b"0 1 \0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    *function = res;
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn cairo_pdf_surface_emit_transparency_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_pattern: *mut cairo_pdf_pattern_t,
    mut gstate_resource: cairo_pdf_resource_t,
    mut gradient_mask: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut smask_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut buf: [libc::c_char; 100] = [0; 100];
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    if (*pdf_pattern).is_shading != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            b"         /Shading\n            << /sh%d %d 0 R >>\n\0" as *const u8
                as *const libc::c_char,
            gradient_mask.id,
            gradient_mask.id,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            b"         /Pattern\n            << /p%d %d 0 R >>\n\0" as *const u8
                as *const libc::c_char,
            gradient_mask.id,
            gradient_mask.id,
        );
    }
    if (*pdf_pattern).is_shading != 0 {
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        _cairo_box_from_rectangle(&mut box_0, &mut (*pdf_pattern).extents);
        _cairo_box_to_doubles(&mut box_0, &mut x1, &mut y1, &mut x2, &mut y2);
        _cairo_matrix_transform_bounding_box(
            &mut (*(*pdf_pattern).pattern).matrix,
            &mut x1,
            &mut y1,
            &mut x2,
            &mut y2,
            0 as *mut cairo_bool_t,
        );
    } else {
        let mut box_1: cairo_box_double_t = cairo_box_double_t {
            p1: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            p2: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
        };
        _get_bbox_from_extents(&mut (*pdf_pattern).extents, &mut box_1);
        x1 = box_1.p1.x;
        y1 = box_1.p1.y;
        x2 = box_1.p2.x;
        y2 = box_1.p2.y;
    }
    status = _cairo_pdf_surface_open_stream(
        surface,
        0 as *mut cairo_pdf_resource_t,
        (*surface).compress_streams,
        b"   /Type /XObject\n   /Subtype /Form\n   /FormType 1\n   /BBox [ %f %f %f %f ]\n   /Resources\n      << /ExtGState\n            << /a0 << /ca 1 /CA 1 >>      >>\n%s      >>\n   /Group\n      << /Type /Group\n         /S /Transparency\n         /I true\n         /CS /DeviceGray\n      >>\n\0"
            as *const u8 as *const libc::c_char,
        x1,
        y1,
        x2,
        y2,
        buf.as_mut_ptr(),
    );
    if status as u64 != 0 {
        return status;
    }
    if (*pdf_pattern).is_shading != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"/a0 gs /sh%d sh\n\0" as *const u8 as *const libc::c_char,
            gradient_mask.id,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).output,
            b"q\n/a0 gs\n/Pattern cs /p%d scn\n0 0 %f %f re\nf\nQ\n\0" as *const u8
                as *const libc::c_char,
            gradient_mask.id,
            (*surface).width,
            (*surface).height,
        );
    }
    status = _cairo_pdf_surface_close_stream(surface);
    if status as u64 != 0 {
        return status;
    }
    smask_resource = _cairo_pdf_surface_new_object(surface);
    if smask_resource.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /Mask\n   /S /Luminosity\n   /G %d 0 R\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        smask_resource.id,
        (*surface).pdf_stream.self_0.id,
    );
    _cairo_pdf_surface_update_object(surface, gstate_resource);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /ExtGState\n   /SMask %d 0 R\n   /ca 1\n   /CA 1\n   /AIS false\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        gstate_resource.id,
        smask_resource.id,
    );
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_output_gradient(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_pattern: *const cairo_pdf_pattern_t,
    mut pattern_resource: cairo_pdf_resource_t,
    mut pat_to_pdf: *const cairo_matrix_t,
    mut start: *const cairo_circle_double_t,
    mut end: *const cairo_circle_double_t,
    mut domain: *const libc::c_double,
    mut colorspace: *const libc::c_char,
    mut color_function: cairo_pdf_resource_t,
) {
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n\0" as *const u8 as *const libc::c_char,
        pattern_resource.id,
    );
    if (*pdf_pattern).is_shading == 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"<< /Type /Pattern\n   /PatternType 2\n   /Matrix [ \0" as *const u8
                as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).output, pat_to_pdf);
        _cairo_output_stream_printf(
            (*surface).output,
            b" ]\n   /Shading\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*(*pdf_pattern).pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).output,
            b"      << /ShadingType 2\n         /ColorSpace %s\n         /Coords [ %f %f %f %f ]\n\0"
                as *const u8 as *const libc::c_char,
            colorspace,
            (*start).center.x,
            (*start).center.y,
            (*end).center.x,
            (*end).center.y,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).output,
            b"      << /ShadingType 3\n         /ColorSpace %s\n         /Coords [ %f %f %f %f %f %f ]\n\0"
                as *const u8 as *const libc::c_char,
            colorspace,
            (*start).center.x,
            (*start).center.y,
            if (*start).radius > 0 as libc::c_int as libc::c_double {
                (*start).radius
            } else {
                0 as libc::c_int as libc::c_double
            },
            (*end).center.x,
            (*end).center.y,
            if (*end).radius > 0 as libc::c_int as libc::c_double {
                (*end).radius
            } else {
                0 as libc::c_int as libc::c_double
            },
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"         /Domain [ %f %f ]\n\0" as *const u8 as *const libc::c_char,
        *domain.offset(0 as libc::c_int as isize),
        *domain.offset(1 as libc::c_int as isize),
    );
    if (*(*pdf_pattern).pattern).extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).output,
            b"         /Extend [ true true ]\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).output,
            b"         /Extend [ false false ]\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"         /Function %d 0 R\n      >>\n\0" as *const u8 as *const libc::c_char,
        color_function.id,
    );
    if (*pdf_pattern).is_shading == 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b">>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"endobj\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn _cairo_pdf_surface_emit_gradient(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_pattern: *mut cairo_pdf_pattern_t,
) -> cairo_int_status_t {
    let mut pattern: *mut cairo_gradient_pattern_t = (*pdf_pattern).pattern
        as *mut cairo_gradient_pattern_t;
    let mut color_function: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut alpha_function: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut pat_to_pdf: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut start: cairo_circle_double_t = cairo_circle_double_t {
        center: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        radius: 0.,
    };
    let mut end: cairo_circle_double_t = cairo_circle_double_t {
        center: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        radius: 0.,
    };
    let mut domain: [libc::c_double; 2] = [0.; 2];
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut mat: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    if (*pattern).n_stops != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"pattern->n_stops != 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            4693 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_gradient(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_pdf_surface_emit_pattern_stops(
        surface,
        pattern,
        &mut color_function,
        &mut alpha_function,
    );
    if status as u64 != 0 {
        return status;
    }
    pat_to_pdf = (*pattern).base.matrix;
    status = cairo_matrix_invert(&mut pat_to_pdf) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            4705 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_gradient(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*pdf_pattern).inverted_y_axis != 0 {
        cairo_matrix_init(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
    } else {
        cairo_matrix_init(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
            0 as libc::c_int as libc::c_double,
            (*surface).height,
        );
    }
    cairo_matrix_multiply(&mut pat_to_pdf, &mut pat_to_pdf, &mut mat);
    if (*pattern).base.extend as libc::c_uint
        == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
        || (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
    {
        let mut bounds_x1: libc::c_double = 0.;
        let mut bounds_x2: libc::c_double = 0.;
        let mut bounds_y1: libc::c_double = 0.;
        let mut bounds_y2: libc::c_double = 0.;
        let mut x_scale: libc::c_double = 0.;
        let mut y_scale: libc::c_double = 0.;
        let mut tolerance: libc::c_double = 0.;
        bounds_x1 = 0 as libc::c_int as libc::c_double;
        bounds_y1 = 0 as libc::c_int as libc::c_double;
        bounds_x2 = (*surface).width;
        bounds_y2 = (*surface).height;
        _cairo_matrix_transform_bounding_box(
            &mut (*pattern).base.matrix,
            &mut bounds_x1,
            &mut bounds_y1,
            &mut bounds_x2,
            &mut bounds_y2,
            0 as *mut cairo_bool_t,
        );
        x_scale = (*surface).base.x_resolution / (*surface).base.x_fallback_resolution;
        y_scale = (*surface).base.y_resolution / (*surface).base.y_fallback_resolution;
        tolerance = fabs(_cairo_matrix_compute_determinant(&mut (*pattern).base.matrix));
        tolerance
            /= _cairo_matrix_transformed_circle_major_axis(
                &mut (*pattern).base.matrix,
                1 as libc::c_int as libc::c_double,
            );
        tolerance *= if x_scale < y_scale { x_scale } else { y_scale };
        _cairo_gradient_pattern_box_to_parameter(
            pattern,
            bounds_x1,
            bounds_y1,
            bounds_x2,
            bounds_y2,
            tolerance,
            domain.as_mut_ptr(),
        );
    } else if (*((*pattern).stops).offset(0 as libc::c_int as isize)).offset
        == (*((*pattern).stops)
            .offset(
                ((*pattern).n_stops).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ))
            .offset
    {
        domain[0 as libc::c_int as usize] = 0.0f64;
        domain[1 as libc::c_int as usize] = 1.0f64;
        if (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"pattern->base.extend == CAIRO_EXTEND_PAD\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                4755 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_emit_gradient(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        domain[0 as libc::c_int
            as usize] = (*((*pattern).stops).offset(0 as libc::c_int as isize)).offset;
        domain[1 as libc::c_int
            as usize] = (*((*pattern).stops)
            .offset(
                ((*pattern).n_stops).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ))
            .offset;
    }
    _cairo_gradient_pattern_interpolate(
        pattern,
        domain[0 as libc::c_int as usize],
        &mut start,
    );
    _cairo_gradient_pattern_interpolate(
        pattern,
        domain[1 as libc::c_int as usize],
        &mut end,
    );
    if (*pattern).base.extend as libc::c_uint
        == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
        || (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
    {
        let mut repeat_begin: libc::c_int = 0;
        let mut repeat_end: libc::c_int = 0;
        repeat_begin = floor(domain[0 as libc::c_int as usize]) as libc::c_int;
        repeat_end = ceil(domain[1 as libc::c_int as usize]) as libc::c_int;
        status = _cairo_pdf_surface_emit_repeating_function(
            surface,
            pattern,
            &mut color_function,
            repeat_begin,
            repeat_end,
        );
        if status as u64 != 0 {
            return status;
        }
        if alpha_function.id != 0 as libc::c_int as libc::c_uint {
            status = _cairo_pdf_surface_emit_repeating_function(
                surface,
                pattern,
                &mut alpha_function,
                repeat_begin,
                repeat_end,
            );
            if status as u64 != 0 {
                return status;
            }
        }
    } else if (*pattern).n_stops <= 2 as libc::c_int as libc::c_uint {
        domain[0 as libc::c_int as usize] = 0.0f64;
        domain[1 as libc::c_int as usize] = 1.0f64;
    }
    _cairo_pdf_surface_update_object(surface, (*pdf_pattern).pattern_res);
    _cairo_pdf_surface_output_gradient(
        surface,
        pdf_pattern,
        (*pdf_pattern).pattern_res,
        &mut pat_to_pdf,
        &mut start,
        &mut end,
        domain.as_mut_ptr(),
        b"/DeviceRGB\0" as *const u8 as *const libc::c_char,
        color_function,
    );
    if alpha_function.id != 0 as libc::c_int as libc::c_uint {
        let mut mask_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
        if (*pdf_pattern).gstate_res.id != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"pdf_pattern->gstate_res.id != 0\0" as *const u8 as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                4811 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_emit_gradient(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        mask_resource = _cairo_pdf_surface_new_object(surface);
        if mask_resource.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        _cairo_pdf_surface_output_gradient(
            surface,
            pdf_pattern,
            mask_resource,
            &mut pat_to_pdf,
            &mut start,
            &mut end,
            domain.as_mut_ptr(),
            b"/DeviceGray\0" as *const u8 as *const libc::c_char,
            alpha_function,
        );
        status = cairo_pdf_surface_emit_transparency_group(
            surface,
            pdf_pattern,
            (*pdf_pattern).gstate_res,
            mask_resource,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_mesh_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_pattern: *mut cairo_pdf_pattern_t,
) -> cairo_int_status_t {
    let mut pat_to_pdf: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut pattern: *mut cairo_pattern_t = (*pdf_pattern).pattern;
    let mut shading: cairo_pdf_shading_t = cairo_pdf_shading_t {
        shading_type: 0,
        bits_per_coordinate: 0,
        bits_per_component: 0,
        bits_per_flag: 0,
        decode_array: 0 as *mut libc::c_double,
        decode_array_length: 0,
        data: 0 as *mut libc::c_uchar,
        data_length: 0,
    };
    let mut i: libc::c_int = 0;
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut mat: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    pat_to_pdf = (*pattern).matrix;
    status = cairo_matrix_invert(&mut pat_to_pdf) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            4849 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_mesh_pattern(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*pdf_pattern).inverted_y_axis != 0 {
        cairo_matrix_init(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
    } else {
        cairo_matrix_init(
            &mut mat,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
            0 as libc::c_int as libc::c_double,
            (*surface).height,
        );
    }
    cairo_matrix_multiply(&mut pat_to_pdf, &mut pat_to_pdf, &mut mat);
    status = _cairo_pdf_shading_init_color(
        &mut shading,
        pattern as *mut cairo_mesh_pattern_t,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    res = _cairo_pdf_surface_new_object(surface);
    if res.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /ShadingType %d\n   /ColorSpace /DeviceRGB\n   /BitsPerCoordinate %d\n   /BitsPerComponent %d\n   /BitsPerFlag %d\n   /Decode [\0"
            as *const u8 as *const libc::c_char,
        res.id,
        shading.shading_type,
        shading.bits_per_coordinate,
        shading.bits_per_component,
        shading.bits_per_flag,
    );
    i = 0 as libc::c_int;
    while i < shading.decode_array_length {
        _cairo_output_stream_printf(
            (*surface).output,
            b"%f \0" as *const u8 as *const libc::c_char,
            *(shading.decode_array).offset(i as isize),
        );
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n   /Length %ld\n>>\nstream\n\0" as *const u8 as *const libc::c_char,
        shading.data_length,
    );
    _cairo_output_stream_write(
        (*surface).output,
        shading.data as *const libc::c_void,
        shading.data_length,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"\nendstream\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_shading_fini(&mut shading);
    _cairo_pdf_surface_update_object(surface, (*pdf_pattern).pattern_res);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /Pattern\n   /PatternType 2\n   /Matrix [ \0" as *const u8
            as *const libc::c_char,
        (*pdf_pattern).pattern_res.id,
    );
    _cairo_output_stream_print_matrix((*surface).output, &mut pat_to_pdf);
    _cairo_output_stream_printf(
        (*surface).output,
        b" ]\n   /Shading %d 0 R\n>>\nendobj\n\0" as *const u8 as *const libc::c_char,
        res.id,
    );
    if (*pdf_pattern).gstate_res.id != 0 as libc::c_int as libc::c_uint {
        let mut mask_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
        res = _cairo_pdf_surface_new_object(surface);
        if res.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        status = _cairo_pdf_shading_init_alpha(
            &mut shading,
            pattern as *mut cairo_mesh_pattern_t,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /ShadingType %d\n   /ColorSpace /DeviceGray\n   /BitsPerCoordinate %d\n   /BitsPerComponent %d\n   /BitsPerFlag %d\n   /Decode [\0"
                as *const u8 as *const libc::c_char,
            res.id,
            shading.shading_type,
            shading.bits_per_coordinate,
            shading.bits_per_component,
            shading.bits_per_flag,
        );
        i = 0 as libc::c_int;
        while i < shading.decode_array_length {
            _cairo_output_stream_printf(
                (*surface).output,
                b"%f \0" as *const u8 as *const libc::c_char,
                *(shading.decode_array).offset(i as isize),
            );
            i += 1;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"]\n   /Length %ld\n>>\nstream\n\0" as *const u8 as *const libc::c_char,
            shading.data_length,
        );
        _cairo_output_stream_write(
            (*surface).output,
            shading.data as *const libc::c_void,
            shading.data_length,
        );
        _cairo_output_stream_printf(
            (*surface).output,
            b"\nendstream\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_shading_fini(&mut shading);
        mask_resource = _cairo_pdf_surface_new_object(surface);
        if mask_resource.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Pattern\n   /PatternType 2\n   /Matrix [ \0"
                as *const u8 as *const libc::c_char,
            mask_resource.id,
        );
        _cairo_output_stream_print_matrix((*surface).output, &mut pat_to_pdf);
        _cairo_output_stream_printf(
            (*surface).output,
            b" ]\n   /Shading %d 0 R\n>>\nendobj\n\0" as *const u8
                as *const libc::c_char,
            res.id,
        );
        status = cairo_pdf_surface_emit_transparency_group(
            surface,
            pdf_pattern,
            (*pdf_pattern).gstate_res,
            mask_resource,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pdf_pattern: *mut cairo_pdf_pattern_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    match (*(*pdf_pattern).pattern).type_0 as libc::c_uint {
        0 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    4992 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"cairo_int_status_t _cairo_pdf_surface_emit_pattern(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            status = _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH)
                as cairo_int_status_t;
        }
        1 | 5 => {
            status = _cairo_pdf_surface_emit_surface_pattern(surface, pdf_pattern);
        }
        2 | 3 => {
            status = _cairo_pdf_surface_emit_gradient(surface, pdf_pattern);
        }
        4 => {
            status = _cairo_pdf_surface_emit_mesh_pattern(surface, pdf_pattern);
        }
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    5011 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"cairo_int_status_t _cairo_pdf_surface_emit_pattern(cairo_pdf_surface_t *, cairo_pdf_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            status = _cairo_error(CAIRO_STATUS_PATTERN_TYPE_MISMATCH)
                as cairo_int_status_t;
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_paint_surface_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut alpha: libc::c_double,
    mut smask_res: *mut cairo_pdf_resource_t,
    mut stencil_mask: cairo_bool_t,
) -> cairo_int_status_t {
    let mut cairo_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut pdf_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut alpha_id: libc::c_int = 0;
    let mut x_offset: libc::c_double = 0.;
    let mut y_offset: libc::c_double = 0.;
    let mut pdf_source: *mut cairo_pdf_source_surface_entry_t = 0
        as *mut cairo_pdf_source_surface_entry_t;
    if (*source).extend as libc::c_uint
        == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        && !((*source).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
            && (*(*(source as *mut cairo_surface_pattern_t)).surface).type_0
                as libc::c_uint
                == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint)
    {
        status = _cairo_pdf_surface_add_padded_image_surface(
            surface,
            source,
            extents,
            &mut pdf_source,
            &mut x_offset,
            &mut y_offset,
            0 as *mut cairo_rectangle_int_t,
        );
    } else {
        status = _cairo_pdf_surface_add_source_surface(
            surface,
            0 as *mut cairo_surface_t,
            source,
            op,
            (*source).filter,
            stencil_mask,
            0 as libc::c_int,
            (alpha != 1.0f64) as libc::c_int,
            extents,
            smask_res,
            &mut pdf_source,
            &mut x_offset,
            &mut y_offset,
            0 as *mut cairo_rectangle_int_t,
        );
    }
    if status as u64 != 0 {
        return status;
    }
    cairo_p2d = (*source).matrix;
    status = cairo_matrix_invert(&mut cairo_p2d) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            5068 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 203],
                &[libc::c_char; 203],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_paint_surface_pattern(cairo_pdf_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_rectangle_int_t *, double, cairo_pdf_resource_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    pdf_p2d = (*surface).cairo_to_pdf;
    cairo_matrix_multiply(&mut pdf_p2d, &mut cairo_p2d, &mut pdf_p2d);
    cairo_matrix_translate(&mut pdf_p2d, x_offset, y_offset);
    if (*pdf_source).emit_image != 0 {
        let mut width: libc::c_int = 0;
        let mut height: libc::c_int = 0;
        if (*pdf_source).bounded != 0 {
            width = (*pdf_source).extents.width;
            height = (*pdf_source).extents.height;
        } else {
            width = 1 as libc::c_int;
            height = 1 as libc::c_int;
        }
        cairo_matrix_translate(&mut pdf_p2d, 0.0f64, height as libc::c_double);
        cairo_matrix_scale(&mut pdf_p2d, 1.0f64, -1.0f64);
        cairo_matrix_scale(
            &mut pdf_p2d,
            width as libc::c_double,
            height as libc::c_double,
        );
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if _cairo_matrix_is_identity(&mut pdf_p2d) == 0 {
        _cairo_output_stream_print_matrix((*surface).output, &mut pdf_p2d);
        _cairo_output_stream_printf(
            (*surface).output,
            b" cm\n\0" as *const u8 as *const libc::c_char,
        );
    }
    status = _cairo_pdf_surface_add_alpha(surface, alpha, &mut alpha_id);
    if status as u64 != 0 {
        return status;
    }
    if stencil_mask != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"/x%d Do\n\0" as *const u8 as *const libc::c_char,
            (*pdf_source).surface_res.id,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).output,
            b"/a%d gs /x%d Do\n\0" as *const u8 as *const libc::c_char,
            alpha_id,
            (*pdf_source).surface_res.id,
        );
    }
    return _cairo_pdf_surface_add_xobject(surface, (*pdf_source).surface_res);
}
unsafe extern "C" fn _cairo_pdf_surface_paint_gradient(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut alpha: libc::c_double,
) -> cairo_int_status_t {
    let mut shading_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut pat_to_pdf: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut alpha_id: libc::c_int = 0;
    status = _cairo_pdf_surface_add_pdf_shading(
        surface,
        source,
        op,
        extents,
        &mut shading_res,
        &mut gstate_res,
    );
    if status as libc::c_uint
        == CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_SUCCESS;
    }
    if status as u64 != 0 {
        return status;
    }
    pat_to_pdf = (*source).matrix;
    status = cairo_matrix_invert(&mut pat_to_pdf) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            5140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 158],
                &[libc::c_char; 158],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_paint_gradient(cairo_pdf_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_rectangle_int_t *, double)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(
        &mut pat_to_pdf,
        &mut pat_to_pdf,
        &mut (*surface).cairo_to_pdf,
    );
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if _cairo_matrix_is_identity(&mut pat_to_pdf) == 0 {
        _cairo_output_stream_print_matrix((*surface).output, &mut pat_to_pdf);
        _cairo_output_stream_printf(
            (*surface).output,
            b" cm\n\0" as *const u8 as *const libc::c_char,
        );
    }
    status = _cairo_pdf_surface_add_shading(surface, shading_res);
    if status as u64 != 0 {
        return status;
    }
    if gstate_res.id != 0 as libc::c_int as libc::c_uint {
        status = _cairo_pdf_surface_add_smask(surface, gstate_res);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"/s%d gs /sh%d sh\n\0" as *const u8 as *const libc::c_char,
            gstate_res.id,
            shading_res.id,
        );
    } else {
        status = _cairo_pdf_surface_add_alpha(surface, alpha, &mut alpha_id);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"/a%d gs /sh%d sh\n\0" as *const u8 as *const libc::c_char,
            alpha_id,
            shading_res.id,
        );
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_paint_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut alpha: libc::c_double,
    mut mask: cairo_bool_t,
) -> cairo_int_status_t {
    match (*source).type_0 as libc::c_uint {
        1 | 5 => {
            return _cairo_pdf_surface_paint_surface_pattern(
                surface,
                op,
                source,
                extents,
                alpha,
                0 as *mut cairo_pdf_resource_t,
                mask,
            );
        }
        2 | 3 | 4 => {
            return _cairo_pdf_surface_paint_gradient(surface, op, source, extents, alpha);
        }
        0 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    5208 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 171],
                        &[libc::c_char; 171],
                    >(
                        b"cairo_int_status_t _cairo_pdf_surface_paint_pattern(cairo_pdf_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_rectangle_int_t *, double, cairo_bool_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    };
}
unsafe extern "C" fn _can_paint_pattern(
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    match (*pattern).type_0 as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 | 5 => {
            return ((*pattern).extend as libc::c_uint
                == CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
                || (*pattern).extend as libc::c_uint
                    == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint) as libc::c_int;
        }
        2 | 3 => return 1 as libc::c_int,
        4 => return 0 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    5233 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 57],
                        &[libc::c_char; 57],
                    >(b"cairo_bool_t _can_paint_pattern(const cairo_pattern_t *)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn _cairo_pdf_surface_select_operator(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if op as libc::c_uint == (*surface).current_operator as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"/b%d gs\n\0" as *const u8 as *const libc::c_char,
        op as libc::c_uint,
    );
    (*surface).current_operator = op;
    _cairo_pdf_surface_add_operator(surface, op);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_select_pattern(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut pattern_res: cairo_pdf_resource_t,
    mut is_stroke: cairo_bool_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut alpha: libc::c_int = 0;
    let mut solid_color: *const cairo_color_t = 0 as *const cairo_color_t;
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        let mut solid: *const cairo_solid_pattern_t = pattern
            as *const cairo_solid_pattern_t;
        solid_color = &(*solid).color;
    }
    if !solid_color.is_null() {
        if (*surface).current_pattern_is_solid_color == 0 as libc::c_int
            || (*surface).current_color_red != (*solid_color).red
            || (*surface).current_color_green != (*solid_color).green
            || (*surface).current_color_blue != (*solid_color).blue
            || (*surface).current_color_is_stroke != is_stroke
        {
            status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                as cairo_int_status_t;
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"%f %f %f \0" as *const u8 as *const libc::c_char,
                (*solid_color).red,
                (*solid_color).green,
                (*solid_color).blue,
            );
            if is_stroke != 0 {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"RG \0" as *const u8 as *const libc::c_char,
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"rg \0" as *const u8 as *const libc::c_char,
                );
            }
            (*surface).current_color_red = (*solid_color).red;
            (*surface).current_color_green = (*solid_color).green;
            (*surface).current_color_blue = (*solid_color).blue;
            (*surface).current_color_is_stroke = is_stroke;
        }
        if (*surface).current_pattern_is_solid_color == 0 as libc::c_int
            || (*surface).current_color_alpha != (*solid_color).alpha
        {
            status = _cairo_pdf_surface_add_alpha(
                surface,
                (*solid_color).alpha,
                &mut alpha,
            );
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                as cairo_int_status_t;
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"/a%d gs\n\0" as *const u8 as *const libc::c_char,
                alpha,
            );
            (*surface).current_color_alpha = (*solid_color).alpha;
        }
        (*surface).current_pattern_is_solid_color = 1 as libc::c_int;
    } else {
        status = _cairo_pdf_surface_add_alpha(surface, 1.0f64, &mut alpha);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_add_pattern(surface, pattern_res);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
            as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        if (*surface).select_pattern_gstate_saved == 0 {
            _cairo_output_stream_printf(
                (*surface).output,
                b"q \0" as *const u8 as *const libc::c_char,
            );
        }
        if is_stroke != 0 {
            _cairo_output_stream_printf(
                (*surface).output,
                b"/Pattern CS /p%d SCN \0" as *const u8 as *const libc::c_char,
                pattern_res.id,
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).output,
                b"/Pattern cs /p%d scn \0" as *const u8 as *const libc::c_char,
                pattern_res.id,
            );
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"/a%d gs\n\0" as *const u8 as *const libc::c_char,
            alpha,
        );
        (*surface).select_pattern_gstate_saved = 1 as libc::c_int;
        (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    }
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_unselect_pattern(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if (*surface).select_pattern_gstate_saved != 0 {
        status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
            as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"Q\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
        (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    }
    (*surface).select_pattern_gstate_saved = 0 as libc::c_int;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_show_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_array_append(
        &mut (*surface).page_heights,
        &mut (*surface).height as *mut libc::c_double as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_array_append(
        &mut (*surface).page_labels,
        &mut (*surface).current_page_label as *mut *mut libc::c_char
            as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    let ref mut fresh50 = (*surface).current_page_label;
    *fresh50 = 0 as *mut libc::c_char;
    status = _cairo_pdf_interchange_end_page_content(surface);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_close_content_stream(surface, 0 as libc::c_int);
    if status as u64 != 0 {
        return status;
    }
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    status = _cairo_pdf_surface_write_page(surface);
    if status as u64 != 0 {
        return status;
    }
    _cairo_pdf_surface_clear(surface);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    if (*surface).surface_bounded != 0 {
        *rectangle = (*surface).surface_extents;
    }
    return (*surface).surface_bounded;
}
unsafe extern "C" fn _cairo_pdf_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    _cairo_font_options_init_default(options);
    cairo_font_options_set_hint_style(options, CAIRO_HINT_STYLE_NONE);
    cairo_font_options_set_hint_metrics(options, CAIRO_HINT_METRICS_OFF);
    cairo_font_options_set_antialias(options, CAIRO_ANTIALIAS_GRAY);
    _cairo_font_options_set_round_glyph_positions(options, CAIRO_ROUND_GLYPH_POS_OFF);
}
unsafe extern "C" fn _cairo_pdf_surface_write_pages(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut page: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut num_pages: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_surface_object_begin(surface, (*surface).pages_resource);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Type /Pages\n   /Kids [ \0" as *const u8 as *const libc::c_char,
    );
    num_pages = _cairo_array_num_elements(&mut (*surface).pages) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_pages {
        _cairo_array_copy_element(
            &mut (*surface).pages,
            i as libc::c_uint,
            &mut page as *mut cairo_pdf_resource_t as *mut libc::c_void,
        );
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"%d 0 R \0" as *const u8 as *const libc::c_char,
            page.id,
        );
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"   /Count %d\n\0" as *const u8 as *const libc::c_char,
        num_pages,
    );
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return CAIRO_INT_STATUS_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_utf8_to_pdf_string(
    mut utf8: *const libc::c_char,
    mut str_out: *mut *mut libc::c_char,
) -> cairo_int_status_t {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ascii: cairo_bool_t = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    ascii = 1 as libc::c_int;
    p = utf8 as *mut libc::c_uchar;
    len = 0 as libc::c_int;
    while *p != 0 {
        if (*p as libc::c_int) < 32 as libc::c_int
            || *p as libc::c_int > 126 as libc::c_int
        {
            ascii = 0 as libc::c_int;
            break;
        } else {
            if *p as libc::c_int == '(' as i32 || *p as libc::c_int == ')' as i32
                || *p as libc::c_int == '\\' as i32
            {
                len += 2 as libc::c_int;
            } else {
                len += 1;
            }
            p = p.offset(1);
        }
    }
    if ascii != 0 {
        str = (if len + 3 as libc::c_int != 0 as libc::c_int {
            malloc((len + 3 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if str.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        *str.offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
        p = utf8 as *mut libc::c_uchar;
        i = 1 as libc::c_int;
        while *p != 0 {
            if *p as libc::c_int == '(' as i32 || *p as libc::c_int == ')' as i32
                || *p as libc::c_int == '\\' as i32
            {
                let fresh51 = i;
                i = i + 1;
                *str.offset(fresh51 as isize) = '\\' as i32 as libc::c_char;
            }
            let fresh52 = i;
            i = i + 1;
            *str.offset(fresh52 as isize) = *p as libc::c_char;
            p = p.offset(1);
        }
        let fresh53 = i;
        i = i + 1;
        *str.offset(fresh53 as isize) = ')' as i32 as libc::c_char;
        let fresh54 = i;
        i = i + 1;
        *str.offset(fresh54 as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        let mut utf16: *mut uint16_t = 0 as *mut uint16_t;
        let mut utf16_len: libc::c_int = 0 as libc::c_int;
        status = _cairo_utf8_to_utf16(
            utf8,
            -(1 as libc::c_int),
            &mut utf16,
            &mut utf16_len,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        str = (if utf16_len * 4 as libc::c_int + 7 as libc::c_int != 0 as libc::c_int {
            malloc((utf16_len * 4 as libc::c_int + 7 as libc::c_int) as libc::c_ulong)
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_char;
        if str.is_null() {
            free(utf16 as *mut libc::c_void);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        strcpy(str, b"<FEFF\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < utf16_len {
            snprintf(
                str
                    .offset((4 as libc::c_int * i) as isize)
                    .offset(5 as libc::c_int as isize),
                5 as libc::c_int as libc::c_ulong,
                b"%04X\0" as *const u8 as *const libc::c_char,
                *utf16.offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
        strcat(str, b">\0" as *const u8 as *const libc::c_char);
        free(utf16 as *mut libc::c_void);
    }
    *str_out = str;
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_unicode_for_glyph(
    mut surface: *mut cairo_pdf_surface_t,
    mut utf8: *const libc::c_char,
) -> cairo_int_status_t {
    let mut utf16: *mut uint16_t = 0 as *mut uint16_t;
    let mut utf16_len: libc::c_int = 0 as libc::c_int;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_int = 0;
    if !utf8.is_null() && *utf8 as libc::c_int != 0 {
        status = _cairo_utf8_to_utf16(
            utf8,
            -(1 as libc::c_int),
            &mut utf16,
            &mut utf16_len,
        ) as cairo_int_status_t;
        if status as libc::c_uint
            == CAIRO_INT_STATUS_INVALID_STRING as libc::c_int as libc::c_uint
        {
            utf16 = 0 as *mut uint16_t;
            utf16_len = 0 as libc::c_int;
        } else if status as u64 != 0 {
            return status
        }
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"<\0" as *const u8 as *const libc::c_char,
    );
    if utf16.is_null() || utf16_len == 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).output,
            b"fffd\0" as *const u8 as *const libc::c_char,
        );
    } else {
        i = 0 as libc::c_int;
        while i < utf16_len {
            _cairo_output_stream_printf(
                (*surface).output,
                b"%04x\0" as *const u8 as *const libc::c_char,
                *utf16.offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">\0" as *const u8 as *const libc::c_char,
    );
    free(utf16 as *mut libc::c_void);
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _hash_data(
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
    mut initval: uint32_t,
) -> uint32_t {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut len: uint32_t = 0;
    len = length as uint32_t;
    b = 0x9e3779b9 as libc::c_uint;
    a = b;
    c = initval;
    while len >= 12 as libc::c_int as libc::c_uint {
        a = (a as libc::c_uint)
            .wrapping_add(
                (*data.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*data.offset(1 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(2 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(3 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int,
                    ),
            ) as uint32_t as uint32_t;
        b = (b as libc::c_uint)
            .wrapping_add(
                (*data.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*data.offset(5 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(6 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(7 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int,
                    ),
            ) as uint32_t as uint32_t;
        c = (c as libc::c_uint)
            .wrapping_add(
                (*data.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*data.offset(9 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(10 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*data.offset(11 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int,
                    ),
            ) as uint32_t as uint32_t;
        a = (a as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
        a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
        a ^= c >> 13 as libc::c_int;
        b = (b as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
        b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
        b ^= a << 8 as libc::c_int;
        c = (c as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
        c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
        c ^= b >> 13 as libc::c_int;
        a = (a as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
        a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
        a ^= c >> 12 as libc::c_int;
        b = (b as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
        b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
        b ^= a << 16 as libc::c_int;
        c = (c as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
        c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
        c ^= b >> 5 as libc::c_int;
        a = (a as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
        a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
        a ^= c >> 3 as libc::c_int;
        b = (b as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
        b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
        b ^= a << 10 as libc::c_int;
        c = (c as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
        c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
        c ^= b >> 15 as libc::c_int;
        data = data.offset(12 as libc::c_int as isize);
        len = (len as libc::c_uint).wrapping_sub(12 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
    c = (c as libc::c_uint).wrapping_add(length as libc::c_uint) as uint32_t as uint32_t;
    let mut current_block_50: u64;
    match len {
        11 => {
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*data.offset(10 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 9071535170274630892;
        }
        10 => {
            current_block_50 = 9071535170274630892;
        }
        9 => {
            current_block_50 = 16452053033226139942;
        }
        8 => {
            current_block_50 = 5474211551685377166;
        }
        7 => {
            current_block_50 = 17072031266154981142;
        }
        6 => {
            current_block_50 = 17854473668792935499;
        }
        5 => {
            current_block_50 = 662814695422129217;
        }
        4 => {
            current_block_50 = 16121025973615768663;
        }
        3 => {
            current_block_50 = 18442670791471855424;
        }
        2 => {
            current_block_50 = 18336776756768106650;
        }
        1 => {
            current_block_50 = 3229544136590896939;
        }
        _ => {
            current_block_50 = 313581471991351815;
        }
    }
    match current_block_50 {
        9071535170274630892 => {
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*data.offset(9 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 16452053033226139942;
        }
        _ => {}
    }
    match current_block_50 {
        16452053033226139942 => {
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*data.offset(8 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 5474211551685377166;
        }
        _ => {}
    }
    match current_block_50 {
        5474211551685377166 => {
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*data.offset(7 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 17072031266154981142;
        }
        _ => {}
    }
    match current_block_50 {
        17072031266154981142 => {
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*data.offset(6 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 17854473668792935499;
        }
        _ => {}
    }
    match current_block_50 {
        17854473668792935499 => {
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*data.offset(5 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 662814695422129217;
        }
        _ => {}
    }
    match current_block_50 {
        662814695422129217 => {
            b = (b as libc::c_uint)
                .wrapping_add(*data.offset(4 as libc::c_int as isize) as libc::c_uint)
                as uint32_t as uint32_t;
            current_block_50 = 16121025973615768663;
        }
        _ => {}
    }
    match current_block_50 {
        16121025973615768663 => {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*data.offset(3 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 18442670791471855424;
        }
        _ => {}
    }
    match current_block_50 {
        18442670791471855424 => {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*data.offset(2 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 18336776756768106650;
        }
        _ => {}
    }
    match current_block_50 {
        18336776756768106650 => {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*data.offset(1 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int,
                ) as uint32_t as uint32_t;
            current_block_50 = 3229544136590896939;
        }
        _ => {}
    }
    match current_block_50 {
        3229544136590896939 => {
            a = (a as libc::c_uint)
                .wrapping_add(*data.offset(0 as libc::c_int as isize) as libc::c_uint)
                as uint32_t as uint32_t;
        }
        _ => {}
    }
    a = (a as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
    a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
    a ^= c >> 13 as libc::c_int;
    b = (b as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
    b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
    b ^= a << 8 as libc::c_int;
    c = (c as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
    c ^= b >> 13 as libc::c_int;
    a = (a as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
    a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
    a ^= c >> 12 as libc::c_int;
    b = (b as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
    b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
    b ^= a << 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
    c ^= b >> 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
    a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
    a ^= c >> 3 as libc::c_int;
    b = (b as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
    b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
    b ^= a << 10 as libc::c_int;
    c = (c as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
    c ^= b >> 15 as libc::c_int;
    return c;
}
unsafe extern "C" fn _create_font_subset_tag(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut font_name: *const libc::c_char,
    mut tag: *mut libc::c_char,
) {
    let mut hash: uint32_t = 0;
    let mut i: libc::c_int = 0;
    hash = _hash_data(
        font_name as *mut libc::c_uchar,
        strlen(font_name) as libc::c_int,
        0 as libc::c_int as uint32_t,
    );
    hash = _hash_data(
        (*font_subset).glyphs as *mut libc::c_uchar,
        ((*font_subset).num_glyphs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            as libc::c_int,
        hash,
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        *tag
            .offset(
                i as isize,
            ) = ('A' as i32 as libc::c_uint)
            .wrapping_add(hash.wrapping_rem(26 as libc::c_int as libc::c_uint))
            as libc::c_char;
        hash = (hash as libc::c_uint).wrapping_div(26 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        i += 1;
    }
    *tag.offset(i as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_to_unicode_stream(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut stream: *mut cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut i: libc::c_uint = 0;
    let mut num_bfchar: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    (*stream).id = 0 as libc::c_int as libc::c_uint;
    status = _cairo_pdf_surface_open_stream(
        surface,
        0 as *mut cairo_pdf_resource_t,
        (*surface).compress_streams,
        0 as *const libc::c_char,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo\n<< /Registry (Adobe)\n   /Ordering (UCS)\n   /Supplement 0\n>> def\n/CMapName /Adobe-Identity-UCS def\n/CMapType 2 def\n1 begincodespacerange\n\0"
            as *const u8 as *const libc::c_char,
    );
    if (*font_subset).is_composite != 0 && (*font_subset).is_latin == 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"<0000> <ffff>\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).output,
            b"<00> <ff>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"endcodespacerange\n\0" as *const u8 as *const libc::c_char,
    );
    if (*font_subset).is_scaled != 0 {
        num_bfchar = (*font_subset).num_glyphs;
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d beginbfchar\n\0" as *const u8 as *const libc::c_char,
            if num_bfchar > 100 as libc::c_int as libc::c_uint {
                100 as libc::c_int as libc::c_uint
            } else {
                num_bfchar
            },
        );
        i = 0 as libc::c_int as libc::c_uint;
        while i < num_bfchar {
            if i != 0 as libc::c_int as libc::c_uint
                && i.wrapping_rem(100 as libc::c_int as libc::c_uint)
                    == 0 as libc::c_int as libc::c_uint
            {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"endbfchar\n%d beginbfchar\n\0" as *const u8 as *const libc::c_char,
                    if num_bfchar.wrapping_sub(i) > 100 as libc::c_int as libc::c_uint {
                        100 as libc::c_int as libc::c_uint
                    } else {
                        num_bfchar.wrapping_sub(i)
                    },
                );
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"<%02x> \0" as *const u8 as *const libc::c_char,
                i,
            );
            status = _cairo_pdf_surface_emit_unicode_for_glyph(
                surface,
                *((*font_subset).utf8).offset(i as isize),
            );
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            i = i.wrapping_add(1);
        }
    } else {
        num_bfchar = ((*font_subset).num_glyphs)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d beginbfchar\n\0" as *const u8 as *const libc::c_char,
            if num_bfchar > 100 as libc::c_int as libc::c_uint {
                100 as libc::c_int as libc::c_uint
            } else {
                num_bfchar
            },
        );
        i = 0 as libc::c_int as libc::c_uint;
        while i < num_bfchar {
            if i != 0 as libc::c_int as libc::c_uint
                && i.wrapping_rem(100 as libc::c_int as libc::c_uint)
                    == 0 as libc::c_int as libc::c_uint
            {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"endbfchar\n%d beginbfchar\n\0" as *const u8 as *const libc::c_char,
                    if num_bfchar.wrapping_sub(i) > 100 as libc::c_int as libc::c_uint {
                        100 as libc::c_int as libc::c_uint
                    } else {
                        num_bfchar.wrapping_sub(i)
                    },
                );
            }
            if (*font_subset).is_latin != 0 {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"<%02x> \0" as *const u8 as *const libc::c_char,
                    *((*font_subset).to_latin_char)
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ),
                );
            } else if (*font_subset).is_composite != 0 {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"<%04x> \0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"<%02x> \0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            }
            status = _cairo_pdf_surface_emit_unicode_for_glyph(
                surface,
                *((*font_subset).utf8)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
            );
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            i = i.wrapping_add(1);
        }
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"endbfchar\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n\0"
            as *const u8 as *const libc::c_char,
    );
    *stream = (*surface).pdf_stream.self_0;
    return _cairo_pdf_surface_close_stream(surface);
}
unsafe extern "C" fn _cairo_pdf_surface_emit_cff_font(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut subset: *mut cairo_cff_subset_t,
) -> cairo_int_status_t {
    let mut stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut descriptor: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut cidfont_dict: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut subset_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut to_unicode_stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut font: cairo_pdf_font_t = cairo_pdf_font_t {
        font_id: 0,
        subset_id: 0,
        subset_resource: cairo_pdf_resource_t { id: 0 },
    };
    let mut i: libc::c_uint = 0;
    let mut last_glyph: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut tag: [libc::c_char; 10] = [0; 10];
    _create_font_subset_tag(font_subset, (*subset).ps_name, tag.as_mut_ptr());
    subset_resource = _cairo_pdf_surface_get_font_resource(
        surface,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    if subset_resource.id == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    status = _cairo_pdf_surface_open_stream(
        surface,
        0 as *mut cairo_pdf_resource_t,
        1 as libc::c_int,
        if (*font_subset).is_latin != 0 {
            b"   /Subtype /Type1C\n\0" as *const u8 as *const libc::c_char
        } else {
            b"   /Subtype /CIDFontType0C\n\0" as *const u8 as *const libc::c_char
        },
    );
    if status as u64 != 0 {
        return status;
    }
    stream = (*surface).pdf_stream.self_0;
    _cairo_output_stream_write(
        (*surface).output,
        (*subset).data as *const libc::c_void,
        (*subset).data_length,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_emit_to_unicode_stream(
        surface,
        font_subset,
        &mut to_unicode_stream,
    );
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status;
    }
    descriptor = _cairo_pdf_surface_new_object(surface);
    if descriptor.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /FontDescriptor\n   /FontName /%s+%s\n\0" as *const u8
            as *const libc::c_char,
        descriptor.id,
        tag.as_mut_ptr(),
        (*subset).ps_name,
    );
    if !((*subset).family_name_utf8).is_null() {
        let mut pdf_str: *mut libc::c_char = 0 as *mut libc::c_char;
        status = _cairo_utf8_to_pdf_string((*subset).family_name_utf8, &mut pdf_str);
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            _cairo_output_stream_printf(
                (*surface).output,
                b"   /FontFamily %s\n\0" as *const u8 as *const libc::c_char,
                pdf_str,
            );
            free(pdf_str as *mut libc::c_void);
        } else if status as libc::c_uint
            != CAIRO_INT_STATUS_INVALID_STRING as libc::c_int as libc::c_uint
        {
            return status
        }
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Flags 4\n   /FontBBox [ %ld %ld %ld %ld ]\n   /ItalicAngle 0\n   /Ascent %ld\n   /Descent %ld\n   /CapHeight %ld\n   /StemV 80\n   /StemH 80\n   /FontFile3 %u 0 R\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        ((*subset).x_min * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).y_min * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).x_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).y_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).ascent * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).descent * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).y_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        stream.id,
    );
    if (*font_subset).is_latin != 0 {
        i = 255 as libc::c_int as libc::c_uint;
        while i >= 32 as libc::c_int as libc::c_uint {
            if *((*font_subset).latin_to_subset_glyph_index).offset(i as isize)
                > 0 as libc::c_int as libc::c_ulong
            {
                break;
            }
            i = i.wrapping_sub(1);
        }
        last_glyph = i;
        _cairo_pdf_surface_update_object(surface, subset_resource);
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Font\n   /Subtype /Type1\n   /BaseFont /%s+%s\n   /FirstChar 32\n   /LastChar %d\n   /FontDescriptor %d 0 R\n   /Encoding /WinAnsiEncoding\n   /Widths [\0"
                as *const u8 as *const libc::c_char,
            subset_resource.id,
            tag.as_mut_ptr(),
            (*subset).ps_name,
            last_glyph,
            descriptor.id,
        );
        i = 32 as libc::c_int as libc::c_uint;
        while i < last_glyph.wrapping_add(1 as libc::c_int as libc::c_uint) {
            let mut glyph: libc::c_int = *((*font_subset).latin_to_subset_glyph_index)
                .offset(i as isize) as libc::c_int;
            if glyph > 0 as libc::c_int {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b" %f\0" as *const u8 as *const libc::c_char,
                    *((*subset).widths).offset(glyph as isize)
                        * 1000 as libc::c_int as libc::c_double,
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b" 0\0" as *const u8 as *const libc::c_char,
                );
            }
            i = i.wrapping_add(1);
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" ]\n\0" as *const u8 as *const libc::c_char,
        );
        if to_unicode_stream.id != 0 as libc::c_int as libc::c_uint {
            _cairo_output_stream_printf(
                (*surface).output,
                b"    /ToUnicode %d 0 R\n\0" as *const u8 as *const libc::c_char,
                to_unicode_stream.id,
            );
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        cidfont_dict = _cairo_pdf_surface_new_object(surface);
        if cidfont_dict.id == 0 as libc::c_int as libc::c_uint {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Font\n   /Subtype /CIDFontType0\n   /BaseFont /%s+%s\n   /CIDSystemInfo\n   << /Registry (Adobe)\n      /Ordering (Identity)\n      /Supplement 0\n   >>\n   /FontDescriptor %d 0 R\n   /W [0 [\0"
                as *const u8 as *const libc::c_char,
            cidfont_dict.id,
            tag.as_mut_ptr(),
            (*subset).ps_name,
            descriptor.id,
        );
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font_subset).num_glyphs {
            _cairo_output_stream_printf(
                (*surface).output,
                b" %f\0" as *const u8 as *const libc::c_char,
                *((*subset).widths).offset(i as isize)
                    * 1000 as libc::c_int as libc::c_double,
            );
            i = i.wrapping_add(1);
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" ]]\n>>\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_surface_update_object(surface, subset_resource);
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Font\n   /Subtype /Type0\n   /BaseFont /%s+%s\n   /Encoding /Identity-H\n   /DescendantFonts [ %d 0 R]\n\0"
                as *const u8 as *const libc::c_char,
            subset_resource.id,
            tag.as_mut_ptr(),
            (*subset).ps_name,
            cidfont_dict.id,
        );
        if to_unicode_stream.id != 0 as libc::c_int as libc::c_uint {
            _cairo_output_stream_printf(
                (*surface).output,
                b"   /ToUnicode %d 0 R\n\0" as *const u8 as *const libc::c_char,
                to_unicode_stream.id,
            );
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
    }
    font.font_id = (*font_subset).font_id;
    font.subset_id = (*font_subset).subset_id;
    font.subset_resource = subset_resource;
    status = _cairo_array_append(
        &mut (*surface).fonts,
        &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
    ) as cairo_int_status_t;
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_cff_font_subset(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut subset: cairo_cff_subset_t = cairo_cff_subset_t {
        family_name_utf8: 0 as *mut libc::c_char,
        ps_name: 0 as *mut libc::c_char,
        widths: 0 as *mut libc::c_double,
        x_min: 0.,
        y_min: 0.,
        x_max: 0.,
        y_max: 0.,
        ascent: 0.,
        descent: 0.,
        data: 0 as *mut libc::c_char,
        data_length: 0,
    };
    let mut name: [libc::c_char; 64] = [0; 64];
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"CairoFont-%d-%d\0" as *const u8 as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    status = _cairo_cff_subset_init(&mut subset, name.as_mut_ptr(), font_subset)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_emit_cff_font(surface, font_subset, &mut subset);
    _cairo_cff_subset_fini(&mut subset);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_cff_fallback_font(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut subset: cairo_cff_subset_t = cairo_cff_subset_t {
        family_name_utf8: 0 as *mut libc::c_char,
        ps_name: 0 as *mut libc::c_char,
        widths: 0 as *mut libc::c_double,
        x_min: 0.,
        y_min: 0.,
        x_max: 0.,
        y_max: 0.,
        ascent: 0.,
        descent: 0.,
        data: 0 as *mut libc::c_char,
        data_length: 0,
    };
    let mut name: [libc::c_char; 64] = [0; 64];
    if (*font_subset).is_composite == 0 && (*font_subset).is_latin == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"CairoFont-%d-%d\0" as *const u8 as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    status = _cairo_cff_fallback_init(&mut subset, name.as_mut_ptr(), font_subset)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_emit_cff_font(surface, font_subset, &mut subset);
    _cairo_cff_fallback_fini(&mut subset);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_type1_font(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut subset: *mut cairo_type1_subset_t,
) -> cairo_int_status_t {
    let mut stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut descriptor: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut subset_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut to_unicode_stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut font: cairo_pdf_font_t = cairo_pdf_font_t {
        font_id: 0,
        subset_id: 0,
        subset_resource: cairo_pdf_resource_t { id: 0 },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut length: libc::c_ulong = 0;
    let mut i: libc::c_uint = 0;
    let mut last_glyph: libc::c_uint = 0;
    let mut tag: [libc::c_char; 10] = [0; 10];
    _create_font_subset_tag(font_subset, (*subset).base_font, tag.as_mut_ptr());
    subset_resource = _cairo_pdf_surface_get_font_resource(
        surface,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    if subset_resource.id == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    length = ((*subset).header_length)
        .wrapping_add((*subset).data_length)
        .wrapping_add((*subset).trailer_length);
    status = _cairo_pdf_surface_open_stream(
        surface,
        0 as *mut cairo_pdf_resource_t,
        1 as libc::c_int,
        b"   /Length1 %lu\n   /Length2 %lu\n   /Length3 %lu\n\0" as *const u8
            as *const libc::c_char,
        (*subset).header_length,
        (*subset).data_length,
        (*subset).trailer_length,
    );
    if status as u64 != 0 {
        return status;
    }
    stream = (*surface).pdf_stream.self_0;
    _cairo_output_stream_write(
        (*surface).output,
        (*subset).data as *const libc::c_void,
        length,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_emit_to_unicode_stream(
        surface,
        font_subset,
        &mut to_unicode_stream,
    );
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status;
    }
    last_glyph = ((*font_subset).num_glyphs)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    if (*font_subset).is_latin != 0 {
        i = 255 as libc::c_int as libc::c_uint;
        while i >= 32 as libc::c_int as libc::c_uint {
            if *((*font_subset).latin_to_subset_glyph_index).offset(i as isize)
                > 0 as libc::c_int as libc::c_ulong
            {
                break;
            }
            i = i.wrapping_sub(1);
        }
        last_glyph = i;
    }
    descriptor = _cairo_pdf_surface_new_object(surface);
    if descriptor.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /FontDescriptor\n   /FontName /%s+%s\n   /Flags 4\n   /FontBBox [ %ld %ld %ld %ld ]\n   /ItalicAngle 0\n   /Ascent %ld\n   /Descent %ld\n   /CapHeight %ld\n   /StemV 80\n   /StemH 80\n   /FontFile %u 0 R\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        descriptor.id,
        tag.as_mut_ptr(),
        (*subset).base_font,
        ((*subset).x_min * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).y_min * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).x_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).y_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).ascent * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).descent * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        ((*subset).y_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        stream.id,
    );
    _cairo_pdf_surface_update_object(surface, subset_resource);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /Font\n   /Subtype /Type1\n   /BaseFont /%s+%s\n   /FirstChar %d\n   /LastChar %d\n   /FontDescriptor %d 0 R\n\0"
            as *const u8 as *const libc::c_char,
        subset_resource.id,
        tag.as_mut_ptr(),
        (*subset).base_font,
        if (*font_subset).is_latin != 0 { 32 as libc::c_int } else { 0 as libc::c_int },
        last_glyph,
        descriptor.id,
    );
    if (*font_subset).is_latin != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"   /Encoding /WinAnsiEncoding\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Widths [\0" as *const u8 as *const libc::c_char,
    );
    if (*font_subset).is_latin != 0 {
        i = 32 as libc::c_int as libc::c_uint;
        while i < last_glyph.wrapping_add(1 as libc::c_int as libc::c_uint) {
            let mut glyph: libc::c_int = *((*font_subset).latin_to_subset_glyph_index)
                .offset(i as isize) as libc::c_int;
            if glyph > 0 as libc::c_int {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b" %f\0" as *const u8 as *const libc::c_char,
                    *((*subset).widths).offset(glyph as isize)
                        * 1000 as libc::c_int as libc::c_double,
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b" 0\0" as *const u8 as *const libc::c_char,
                );
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font_subset).num_glyphs {
            _cairo_output_stream_printf(
                (*surface).output,
                b" %f\0" as *const u8 as *const libc::c_char,
                *((*subset).widths).offset(i as isize)
                    * 1000 as libc::c_int as libc::c_double,
            );
            i = i.wrapping_add(1);
        }
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b" ]\n\0" as *const u8 as *const libc::c_char,
    );
    if to_unicode_stream.id != 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).output,
            b"    /ToUnicode %d 0 R\n\0" as *const u8 as *const libc::c_char,
            to_unicode_stream.id,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    font.font_id = (*font_subset).font_id;
    font.subset_id = (*font_subset).subset_id;
    font.subset_resource = subset_resource;
    return _cairo_array_append(
        &mut (*surface).fonts,
        &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_type1_font_subset(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut subset: cairo_type1_subset_t = cairo_type1_subset_t {
        base_font: 0 as *mut libc::c_char,
        widths: 0 as *mut libc::c_double,
        x_min: 0.,
        y_min: 0.,
        x_max: 0.,
        y_max: 0.,
        ascent: 0.,
        descent: 0.,
        data: 0 as *mut libc::c_char,
        header_length: 0,
        data_length: 0,
        trailer_length: 0,
    };
    let mut name: [libc::c_char; 64] = [0; 64];
    if (*font_subset).is_composite != 0 && (*font_subset).is_latin == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"CairoFont-%d-%d\0" as *const u8 as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    status = _cairo_type1_subset_init(
        &mut subset,
        name.as_mut_ptr(),
        font_subset,
        0 as libc::c_int,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_emit_type1_font(surface, font_subset, &mut subset);
    _cairo_type1_subset_fini(&mut subset);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_type1_fallback_font(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut subset: cairo_type1_subset_t = cairo_type1_subset_t {
        base_font: 0 as *mut libc::c_char,
        widths: 0 as *mut libc::c_double,
        x_min: 0.,
        y_min: 0.,
        x_max: 0.,
        y_max: 0.,
        ascent: 0.,
        descent: 0.,
        data: 0 as *mut libc::c_char,
        header_length: 0,
        data_length: 0,
        trailer_length: 0,
    };
    let mut name: [libc::c_char; 64] = [0; 64];
    if (*font_subset).is_composite != 0 && (*font_subset).is_latin == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"CairoFont-%d-%d\0" as *const u8 as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    status = _cairo_type1_fallback_init_binary(
        &mut subset,
        name.as_mut_ptr(),
        font_subset,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_emit_type1_font(surface, font_subset, &mut subset);
    _cairo_type1_fallback_fini(&mut subset);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_truetype_font_subset(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut descriptor: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut cidfont_dict: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut subset_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut to_unicode_stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut font: cairo_pdf_font_t = cairo_pdf_font_t {
        font_id: 0,
        subset_id: 0,
        subset_resource: cairo_pdf_resource_t { id: 0 },
    };
    let mut subset: cairo_truetype_subset_t = cairo_truetype_subset_t {
        family_name_utf8: 0 as *mut libc::c_char,
        ps_name: 0 as *mut libc::c_char,
        widths: 0 as *mut libc::c_double,
        x_min: 0.,
        y_min: 0.,
        x_max: 0.,
        y_max: 0.,
        ascent: 0.,
        descent: 0.,
        data: 0 as *mut libc::c_uchar,
        data_length: 0,
        string_offsets: 0 as *mut libc::c_ulong,
        num_string_offsets: 0,
    };
    let mut i: libc::c_uint = 0;
    let mut last_glyph: libc::c_uint = 0;
    let mut tag: [libc::c_char; 10] = [0; 10];
    subset_resource = _cairo_pdf_surface_get_font_resource(
        surface,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    if subset_resource.id == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    status = _cairo_truetype_subset_init_pdf(&mut subset, font_subset)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _create_font_subset_tag(font_subset, subset.ps_name, tag.as_mut_ptr());
    status = _cairo_pdf_surface_open_stream(
        surface,
        0 as *mut cairo_pdf_resource_t,
        1 as libc::c_int,
        b"   /Length1 %lu\n\0" as *const u8 as *const libc::c_char,
        subset.data_length,
    );
    if status as u64 != 0 {
        _cairo_truetype_subset_fini(&mut subset);
        return status;
    }
    stream = (*surface).pdf_stream.self_0;
    _cairo_output_stream_write(
        (*surface).output,
        subset.data as *const libc::c_void,
        subset.data_length,
    );
    status = _cairo_pdf_surface_close_stream(surface);
    if status as u64 != 0 {
        _cairo_truetype_subset_fini(&mut subset);
        return status;
    }
    status = _cairo_pdf_surface_emit_to_unicode_stream(
        surface,
        font_subset,
        &mut to_unicode_stream,
    );
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        _cairo_truetype_subset_fini(&mut subset);
        return status;
    }
    descriptor = _cairo_pdf_surface_new_object(surface);
    if descriptor.id == 0 as libc::c_int as libc::c_uint {
        _cairo_truetype_subset_fini(&mut subset);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /FontDescriptor\n   /FontName /%s+%s\n\0" as *const u8
            as *const libc::c_char,
        descriptor.id,
        tag.as_mut_ptr(),
        subset.ps_name,
    );
    if !(subset.family_name_utf8).is_null() {
        let mut pdf_str: *mut libc::c_char = 0 as *mut libc::c_char;
        status = _cairo_utf8_to_pdf_string(subset.family_name_utf8, &mut pdf_str);
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            _cairo_output_stream_printf(
                (*surface).output,
                b"   /FontFamily %s\n\0" as *const u8 as *const libc::c_char,
                pdf_str,
            );
            free(pdf_str as *mut libc::c_void);
        } else if status as libc::c_uint
            != CAIRO_INT_STATUS_INVALID_STRING as libc::c_int as libc::c_uint
        {
            return status
        }
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Flags %d\n   /FontBBox [ %ld %ld %ld %ld ]\n   /ItalicAngle 0\n   /Ascent %ld\n   /Descent %ld\n   /CapHeight %ld\n   /StemV 80\n   /StemH 80\n   /FontFile2 %u 0 R\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        if (*font_subset).is_latin != 0 { 32 as libc::c_int } else { 4 as libc::c_int },
        (subset.x_min * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        (subset.y_min * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        (subset.x_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        (subset.y_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        (subset.ascent * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        (subset.descent * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        (subset.y_max * 1000 as libc::c_int as libc::c_double) as libc::c_long,
        stream.id,
    );
    if (*font_subset).is_latin != 0 {
        i = 255 as libc::c_int as libc::c_uint;
        while i >= 32 as libc::c_int as libc::c_uint {
            if *((*font_subset).latin_to_subset_glyph_index).offset(i as isize)
                > 0 as libc::c_int as libc::c_ulong
            {
                break;
            }
            i = i.wrapping_sub(1);
        }
        last_glyph = i;
        _cairo_pdf_surface_update_object(surface, subset_resource);
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Font\n   /Subtype /TrueType\n   /BaseFont /%s+%s\n   /FirstChar 32\n   /LastChar %d\n   /FontDescriptor %d 0 R\n   /Encoding /WinAnsiEncoding\n   /Widths [\0"
                as *const u8 as *const libc::c_char,
            subset_resource.id,
            tag.as_mut_ptr(),
            subset.ps_name,
            last_glyph,
            descriptor.id,
        );
        i = 32 as libc::c_int as libc::c_uint;
        while i < last_glyph.wrapping_add(1 as libc::c_int as libc::c_uint) {
            let mut glyph: libc::c_int = *((*font_subset).latin_to_subset_glyph_index)
                .offset(i as isize) as libc::c_int;
            if glyph > 0 as libc::c_int {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b" %f\0" as *const u8 as *const libc::c_char,
                    *(subset.widths).offset(glyph as isize)
                        * 1000 as libc::c_int as libc::c_double,
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b" 0\0" as *const u8 as *const libc::c_char,
                );
            }
            i = i.wrapping_add(1);
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" ]\n\0" as *const u8 as *const libc::c_char,
        );
        if to_unicode_stream.id != 0 as libc::c_int as libc::c_uint {
            _cairo_output_stream_printf(
                (*surface).output,
                b"    /ToUnicode %d 0 R\n\0" as *const u8 as *const libc::c_char,
                to_unicode_stream.id,
            );
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        cidfont_dict = _cairo_pdf_surface_new_object(surface);
        if cidfont_dict.id == 0 as libc::c_int as libc::c_uint {
            _cairo_truetype_subset_fini(&mut subset);
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Font\n   /Subtype /CIDFontType2\n   /BaseFont /%s+%s\n   /CIDSystemInfo\n   << /Registry (Adobe)\n      /Ordering (Identity)\n      /Supplement 0\n   >>\n   /FontDescriptor %d 0 R\n   /W [0 [\0"
                as *const u8 as *const libc::c_char,
            cidfont_dict.id,
            tag.as_mut_ptr(),
            subset.ps_name,
            descriptor.id,
        );
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*font_subset).num_glyphs {
            _cairo_output_stream_printf(
                (*surface).output,
                b" %f\0" as *const u8 as *const libc::c_char,
                *(subset.widths).offset(i as isize)
                    * 1000 as libc::c_int as libc::c_double,
            );
            i = i.wrapping_add(1);
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b" ]]\n>>\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_pdf_surface_update_object(surface, subset_resource);
        _cairo_output_stream_printf(
            (*surface).output,
            b"%d 0 obj\n<< /Type /Font\n   /Subtype /Type0\n   /BaseFont /%s+%s\n   /Encoding /Identity-H\n   /DescendantFonts [ %d 0 R]\n\0"
                as *const u8 as *const libc::c_char,
            subset_resource.id,
            tag.as_mut_ptr(),
            subset.ps_name,
            cidfont_dict.id,
        );
        if to_unicode_stream.id != 0 as libc::c_int as libc::c_uint {
            _cairo_output_stream_printf(
                (*surface).output,
                b"   /ToUnicode %d 0 R\n\0" as *const u8 as *const libc::c_char,
                to_unicode_stream.id,
            );
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
        );
    }
    font.font_id = (*font_subset).font_id;
    font.subset_id = (*font_subset).subset_id;
    font.subset_resource = subset_resource;
    status = _cairo_array_append(
        &mut (*surface).fonts,
        &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
    ) as cairo_int_status_t;
    _cairo_truetype_subset_fini(&mut subset);
    return status;
}
unsafe extern "C" fn _cairo_pdf_emit_imagemask(
    mut image: *mut cairo_image_surface_t,
    mut stream: *mut cairo_output_stream_t,
) -> cairo_int_status_t {
    let mut byte: *mut uint8_t = 0 as *mut uint8_t;
    let mut output_byte: uint8_t = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut num_cols: libc::c_int = 0;
    if (*image).format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {} else {
        __assert_fail(
            b"image->format == CAIRO_FORMAT_A1\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            6425 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"cairo_int_status_t _cairo_pdf_emit_imagemask(cairo_image_surface_t *, cairo_output_stream_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_output_stream_printf(
        stream,
        b"BI\n/IM true\n/W %d\n/H %d\n/BPC 1\n/D [1 0]\n\0" as *const u8
            as *const libc::c_char,
        (*image).width,
        (*image).height,
    );
    _cairo_output_stream_printf(stream, b"ID \0" as *const u8 as *const libc::c_char);
    num_cols = ((*image).width + 7 as libc::c_int) / 8 as libc::c_int;
    row = 0 as libc::c_int;
    while row < (*image).height {
        byte = ((*image).data).offset((row as libc::c_long * (*image).stride) as isize);
        col = 0 as libc::c_int;
        while col < num_cols {
            output_byte = (((*byte as libc::c_ulong).wrapping_mul(0x802 as libc::c_ulong)
                & 0x22110 as libc::c_ulong
                | (*byte as libc::c_ulong).wrapping_mul(0x8020 as libc::c_ulong)
                    & 0x88440 as libc::c_ulong)
                .wrapping_mul(0x10101 as libc::c_ulong) >> 16 as libc::c_int) as uint8_t;
            _cairo_output_stream_write(
                stream,
                &mut output_byte as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            byte = byte.offset(1);
            col += 1;
        }
        row += 1;
    }
    _cairo_output_stream_printf(stream, b"\nEI\n\0" as *const u8 as *const libc::c_char);
    return _cairo_output_stream_get_status(stream) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_analyze_user_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = closure as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut status2: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut type3_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut null_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    null_stream = _cairo_null_stream_create();
    type3_surface = _cairo_type3_glyph_surface_create(
        (*font_subset).scaled_font,
        null_stream,
        Some(
            _cairo_pdf_emit_imagemask
                as unsafe extern "C" fn(
                    *mut cairo_image_surface_t,
                    *mut cairo_output_stream_t,
                ) -> cairo_int_status_t,
        ),
        (*surface).font_subsets,
        0 as libc::c_int,
    );
    if (*type3_surface).status as u64 != 0 {
        status2 = _cairo_output_stream_destroy(null_stream) as cairo_int_status_t;
        return (*type3_surface).status as cairo_int_status_t;
    }
    _cairo_type3_glyph_surface_set_font_subsets_callback(
        type3_surface as *mut libc::c_void,
        Some(
            _cairo_pdf_surface_add_font
                as unsafe extern "C" fn(
                    libc::c_uint,
                    libc::c_uint,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        status = _cairo_type3_glyph_surface_analyze_glyph(
            type3_surface as *mut libc::c_void,
            *((*font_subset).glyphs).offset(i as isize),
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    cairo_surface_destroy(type3_surface);
    status2 = _cairo_output_stream_destroy(null_stream) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        status = status2;
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_type3_font_subset(
    mut surface: *mut cairo_pdf_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_STATUS_SUCCESS as libc::c_int
        as cairo_int_status_t;
    let mut glyphs: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut encoding: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut char_procs: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut subset_resource: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut to_unicode_stream: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut font: cairo_pdf_font_t = cairo_pdf_font_t {
        font_id: 0,
        subset_id: 0,
        subset_resource: cairo_pdf_resource_t { id: 0 },
    };
    let mut widths: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_uint = 0;
    let mut font_bbox: cairo_box_t = {
        let mut init = _cairo_line {
            p1: {
                let mut init = _cairo_point {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            p2: {
                let mut init = _cairo_point {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut bbox: cairo_box_t = {
        let mut init = _cairo_line {
            p1: {
                let mut init = _cairo_point {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            p2: {
                let mut init = _cairo_point {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut type3_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if (*font_subset).num_glyphs == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    subset_resource = _cairo_pdf_surface_get_font_resource(
        surface,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    if subset_resource.id == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    glyphs = _cairo_malloc_ab(
        (*font_subset).num_glyphs as size_t,
        ::std::mem::size_of::<cairo_pdf_resource_t>() as libc::c_ulong,
    ) as *mut cairo_pdf_resource_t;
    if glyphs.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    widths = _cairo_malloc_ab(
        (*font_subset).num_glyphs as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if widths.is_null() {
        free(glyphs as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_pdf_group_resources_clear(&mut (*surface).resources);
    type3_surface = _cairo_type3_glyph_surface_create(
        (*font_subset).scaled_font,
        0 as *mut cairo_output_stream_t,
        Some(
            _cairo_pdf_emit_imagemask
                as unsafe extern "C" fn(
                    *mut cairo_image_surface_t,
                    *mut cairo_output_stream_t,
                ) -> cairo_int_status_t,
        ),
        (*surface).font_subsets,
        0 as libc::c_int,
    );
    if (*type3_surface).status as u64 != 0 {
        free(glyphs as *mut libc::c_void);
        free(widths as *mut libc::c_void);
        return (*type3_surface).status as cairo_int_status_t;
    }
    _cairo_type3_glyph_surface_set_font_subsets_callback(
        type3_surface as *mut libc::c_void,
        Some(
            _cairo_pdf_surface_add_font
                as unsafe extern "C" fn(
                    libc::c_uint,
                    libc::c_uint,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        status = _cairo_pdf_surface_open_stream(
            surface,
            0 as *mut cairo_pdf_resource_t,
            (*surface).compress_streams,
            0 as *const libc::c_char,
        );
        if status as u64 != 0 {
            break;
        }
        *glyphs.offset(i as isize) = (*surface).pdf_stream.self_0;
        status = _cairo_type3_glyph_surface_emit_glyph(
            type3_surface as *mut libc::c_void,
            (*surface).output,
            *((*font_subset).glyphs).offset(i as isize),
            &mut bbox,
            &mut *widths.offset(i as isize),
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            break;
        }
        status = _cairo_pdf_surface_close_stream(surface);
        if status as u64 != 0 {
            break;
        }
        if i == 0 as libc::c_int as libc::c_uint {
            font_bbox.p1.x = bbox.p1.x;
            font_bbox.p1.y = bbox.p1.y;
            font_bbox.p2.x = bbox.p2.x;
            font_bbox.p2.y = bbox.p2.y;
        } else {
            if bbox.p1.x < font_bbox.p1.x {
                font_bbox.p1.x = bbox.p1.x;
            }
            if bbox.p1.y < font_bbox.p1.y {
                font_bbox.p1.y = bbox.p1.y;
            }
            if bbox.p2.x > font_bbox.p2.x {
                font_bbox.p2.x = bbox.p2.x;
            }
            if bbox.p2.y > font_bbox.p2.y {
                font_bbox.p2.y = bbox.p2.y;
            }
        }
        i = i.wrapping_add(1);
    }
    cairo_surface_destroy(type3_surface);
    if status as u64 != 0 {
        free(glyphs as *mut libc::c_void);
        free(widths as *mut libc::c_void);
        return status;
    }
    encoding = _cairo_pdf_surface_new_object(surface);
    if encoding.id == 0 as libc::c_int as libc::c_uint {
        free(glyphs as *mut libc::c_void);
        free(widths as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /Encoding\n   /Differences [0\0" as *const u8
            as *const libc::c_char,
        encoding.id,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        _cairo_output_stream_printf(
            (*surface).output,
            b" /%d\0" as *const u8 as *const libc::c_char,
            i,
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n>>\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    char_procs = _cairo_pdf_surface_new_object(surface);
    if char_procs.id == 0 as libc::c_int as libc::c_uint {
        free(glyphs as *mut libc::c_void);
        free(widths as *mut libc::c_void);
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<<\n\0" as *const u8 as *const libc::c_char,
        char_procs.id,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        _cairo_output_stream_printf(
            (*surface).output,
            b" /%d %d 0 R\n\0" as *const u8 as *const libc::c_char,
            i,
            (*glyphs.offset(i as isize)).id,
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    free(glyphs as *mut libc::c_void);
    status = _cairo_pdf_surface_emit_to_unicode_stream(
        surface,
        font_subset,
        &mut to_unicode_stream,
    );
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        free(widths as *mut libc::c_void);
        return status;
    }
    _cairo_pdf_surface_update_object(surface, subset_resource);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /Font\n   /Subtype /Type3\n   /FontBBox [%f %f %f %f]\n   /FontMatrix [ 1 0 0 -1 0 0 ]\n   /Encoding %d 0 R\n   /CharProcs %d 0 R\n   /FirstChar 0\n   /LastChar %d\n\0"
            as *const u8 as *const libc::c_char,
        subset_resource.id,
        _cairo_fixed_to_double(font_bbox.p1.x),
        _cairo_fixed_to_double(font_bbox.p1.y),
        _cairo_fixed_to_double(font_bbox.p2.x),
        _cairo_fixed_to_double(font_bbox.p2.y),
        encoding.id,
        char_procs.id,
        ((*font_subset).num_glyphs).wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Widths [\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        _cairo_output_stream_printf(
            (*surface).output,
            b" %f\0" as *const u8 as *const libc::c_char,
            *widths.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    free(widths as *mut libc::c_void);
    _cairo_output_stream_printf(
        (*surface).output,
        b"   /Resources\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_emit_group_resources(
        surface,
        &mut (*surface).resources,
        0 as libc::c_int,
    );
    if to_unicode_stream.id != 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).output,
            b"    /ToUnicode %d 0 R\n\0" as *const u8 as *const libc::c_char,
            to_unicode_stream.id,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b">>\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    font.font_id = (*font_subset).font_id;
    font.subset_id = (*font_subset).subset_id;
    font.subset_resource = subset_resource;
    return _cairo_array_append(
        &mut (*surface).fonts,
        &mut font as *mut cairo_pdf_font_t as *const libc::c_void,
    ) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_unscaled_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = closure as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_surface_emit_cff_font_subset(surface, font_subset);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_pdf_surface_emit_truetype_font_subset(surface, font_subset);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_pdf_surface_emit_type1_font_subset(surface, font_subset);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_pdf_surface_emit_cff_fallback_font(surface, font_subset);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_pdf_surface_emit_type1_fallback_font(surface, font_subset);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            6710 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_unscaled_font_subset(cairo_scaled_font_subset_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_scaled_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = closure as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_surface_emit_type3_font_subset(surface, font_subset);
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            6725 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_scaled_font_subset(cairo_scaled_font_subset_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_font_subsets(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_scaled_font_subsets_foreach_user(
        (*surface).font_subsets,
        Some(
            _cairo_pdf_surface_analyze_user_font_subset
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_subset_t,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    ) as cairo_int_status_t;
    if !(status as u64 != 0) {
        status = _cairo_scaled_font_subsets_foreach_unscaled(
            (*surface).font_subsets,
            Some(
                _cairo_pdf_surface_emit_unscaled_font_subset
                    as unsafe extern "C" fn(
                        *mut cairo_scaled_font_subset_t,
                        *mut libc::c_void,
                    ) -> cairo_int_status_t,
            ),
            surface as *mut libc::c_void,
        ) as cairo_int_status_t;
        if !(status as u64 != 0) {
            status = _cairo_scaled_font_subsets_foreach_scaled(
                (*surface).font_subsets,
                Some(
                    _cairo_pdf_surface_emit_scaled_font_subset
                        as unsafe extern "C" fn(
                            *mut cairo_scaled_font_subset_t,
                            *mut libc::c_void,
                        ) -> cairo_int_status_t,
                ),
                surface as *mut libc::c_void,
            ) as cairo_int_status_t;
            if !(status as u64 != 0) {
                status = _cairo_scaled_font_subsets_foreach_user(
                    (*surface).font_subsets,
                    Some(
                        _cairo_pdf_surface_emit_scaled_font_subset
                            as unsafe extern "C" fn(
                                *mut cairo_scaled_font_subset_t,
                                *mut libc::c_void,
                            ) -> cairo_int_status_t,
                    ),
                    surface as *mut libc::c_void,
                ) as cairo_int_status_t;
            }
        }
    }
    _cairo_scaled_font_subsets_destroy((*surface).font_subsets);
    let ref mut fresh55 = (*surface).font_subsets;
    *fresh55 = 0 as *mut cairo_scaled_font_subsets_t;
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_write_catalog(
    mut surface: *mut cairo_pdf_surface_t,
    mut catalog: cairo_pdf_resource_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_pdf_surface_object_begin(surface, catalog) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Type /Catalog\n   /Pages %d 0 R\n\0" as *const u8 as *const libc::c_char,
        (*surface).pages_resource.id,
    );
    if (*surface).struct_tree_root.id != 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /StructTreeRoot %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface).struct_tree_root.id,
        );
        if (*surface).tagged != 0 {
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"   /MarkInfo << /Marked true >>\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if (*surface).outlines_dict_res.id != 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Outlines %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface).outlines_dict_res.id,
        );
    }
    if (*surface).page_labels_res.id != 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /PageLabels %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface).page_labels_res.id,
        );
    }
    if (*surface).names_dict_res.id != 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Names %d 0 R\n\0" as *const u8 as *const libc::c_char,
            (*surface).names_dict_res.id,
        );
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_write_xref(
    mut surface: *mut cairo_pdf_surface_t,
) -> libc::c_longlong {
    let mut object: *mut cairo_pdf_object_t = 0 as *mut cairo_pdf_object_t;
    let mut num_objects: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_longlong = 0;
    num_objects = _cairo_array_num_elements(&mut (*surface).objects) as libc::c_int;
    offset = _cairo_output_stream_get_position((*surface).output);
    _cairo_output_stream_printf(
        (*surface).output,
        b"xref\n%d %d\n\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        num_objects + 1 as libc::c_int,
    );
    _cairo_output_stream_printf(
        (*surface).output,
        b"0000000000 65535 f \n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < num_objects {
        object = _cairo_array_index(&mut (*surface).objects, i as libc::c_uint)
            as *mut cairo_pdf_object_t;
        _cairo_output_stream_printf(
            (*surface).output,
            b"%010lld 00000 n \n\0" as *const u8 as *const libc::c_char,
            (*object).u.offset,
        );
        i += 1;
    }
    return offset;
}
unsafe extern "C" fn _cairo_write_xref_stream_entry(
    mut stream: *mut cairo_output_stream_t,
    mut id: libc::c_int,
    mut type_0: libc::c_int,
    mut field2_size: libc::c_int,
    mut field2: libc::c_longlong,
    mut field3: libc::c_int,
    mut write_as_comments: cairo_bool_t,
) {
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut i: libc::c_int = 0;
    if write_as_comments != 0 {
        _cairo_output_stream_printf(
            stream,
            b"%% %5d %2d %10lld  %d\n\0" as *const u8 as *const libc::c_char,
            id,
            type_0,
            field2,
            field3,
        );
    } else {
        buf[0 as libc::c_int as usize] = type_0 as libc::c_char;
        i = field2_size - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            buf[(i + 1 as libc::c_int)
                as usize] = (field2 & 0xff as libc::c_int as libc::c_longlong)
                as libc::c_char;
            field2 >>= 8 as libc::c_int;
            i -= 1;
        }
        buf[(field2_size + 1 as libc::c_int)
            as usize] = (field3 >> 8 as libc::c_int) as libc::c_char;
        buf[(field2_size + 2 as libc::c_int)
            as usize] = (field3 & 0xff as libc::c_int) as libc::c_char;
        _cairo_output_stream_write(
            stream,
            buf.as_mut_ptr() as *const libc::c_void,
            (field2_size + 3 as libc::c_int) as size_t,
        );
    };
}
unsafe extern "C" fn _cairo_write_xref_stream_entries(
    mut surface: *mut cairo_pdf_surface_t,
    mut stream: *mut cairo_output_stream_t,
    mut field2_size: libc::c_int,
    mut write_as_comments: cairo_bool_t,
) {
    let mut object: *mut cairo_pdf_object_t = 0 as *mut cairo_pdf_object_t;
    let mut num_objects: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    _cairo_write_xref_stream_entry(
        stream,
        0 as libc::c_int,
        PDF_OBJECT_FREE as libc::c_int,
        field2_size,
        0 as libc::c_int as libc::c_longlong,
        0xffff as libc::c_int,
        write_as_comments,
    );
    num_objects = _cairo_array_num_elements(&mut (*surface).objects) as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_objects {
        object = _cairo_array_index(&mut (*surface).objects, i as libc::c_uint)
            as *mut cairo_pdf_object_t;
        if (*object).type_0 as libc::c_uint
            == PDF_OBJECT_UNCOMPRESSED as libc::c_int as libc::c_uint
        {
            _cairo_write_xref_stream_entry(
                stream,
                i + 1 as libc::c_int,
                (*object).type_0 as libc::c_int,
                field2_size,
                (*object).u.offset,
                0 as libc::c_int,
                write_as_comments,
            );
        } else if (*object).type_0 as libc::c_uint
            == PDF_OBJECT_COMPRESSED as libc::c_int as libc::c_uint
        {
            _cairo_write_xref_stream_entry(
                stream,
                i + 1 as libc::c_int,
                (*object).type_0 as libc::c_int,
                field2_size,
                (*object).u.compressed_obj.xref_stream.id as libc::c_longlong,
                (*object).u.compressed_obj.index,
                write_as_comments,
            );
        } else {
            _cairo_write_xref_stream_entry(
                stream,
                i + 1 as libc::c_int,
                PDF_OBJECT_FREE as libc::c_int,
                field2_size,
                0 as libc::c_int as libc::c_longlong,
                0xffff as libc::c_int,
                write_as_comments,
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn _cairo_pdf_surface_write_xref_stream(
    mut surface: *mut cairo_pdf_surface_t,
    mut xref_res: cairo_pdf_resource_t,
    mut root_res: cairo_pdf_resource_t,
    mut info_res: cairo_pdf_resource_t,
    mut xref_offset: *mut libc::c_longlong,
) -> cairo_int_status_t {
    let mut mem_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut xref_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut offset: libc::c_longlong = 0;
    let mut offset_bytes: libc::c_int = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    *xref_offset = _cairo_output_stream_get_position((*surface).output);
    offset_bytes = 0 as libc::c_int;
    offset = *xref_offset;
    while offset > 0 as libc::c_int as libc::c_longlong {
        offset >>= 8 as libc::c_int;
        offset_bytes += 1;
    }
    mem_stream = _cairo_memory_stream_create();
    xref_stream = _cairo_deflate_stream_create(mem_stream);
    _cairo_write_xref_stream_entries(
        surface,
        xref_stream,
        offset_bytes,
        0 as libc::c_int,
    );
    status = _cairo_output_stream_destroy(xref_stream);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_pdf_surface_update_object(surface, xref_res);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /XRef\n   /Length %d\n   /Filter /FlateDecode\n   /Size %d\n   /W [1 %d 2]\n   /Root %d 0 R\n   /Info %d 0 R\n>>\n\0"
            as *const u8 as *const libc::c_char,
        xref_res.id,
        _cairo_memory_stream_length(mem_stream),
        (*surface).next_available_resource.id,
        offset_bytes,
        root_res.id,
        info_res.id,
    );
    if (*surface).compress_streams == 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"%%   id   type  offset/obj  gen/index\n\0" as *const u8
                as *const libc::c_char,
        );
        _cairo_write_xref_stream_entries(
            surface,
            (*surface).output,
            offset_bytes,
            1 as libc::c_int,
        );
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"stream\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_memory_stream_copy(mem_stream, (*surface).output);
    status = _cairo_output_stream_destroy(mem_stream);
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"\nendstream\nendobj\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_write_mask_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut group: *mut cairo_pdf_smask_group_t,
) -> cairo_int_status_t {
    let mut mask_group: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut smask: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut smask_group: *mut cairo_pdf_smask_group_t = 0
        as *mut cairo_pdf_smask_group_t;
    let mut pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut bbox: cairo_box_double_t = cairo_box_double_t {
        p1: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        p2: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
    };
    _get_bbox_from_extents(&mut (*group).extents, &mut bbox);
    status = _cairo_pdf_surface_open_group(
        surface,
        &mut bbox,
        0 as *mut cairo_pdf_resource_t,
    );
    if status as u64 != 0 {
        return status;
    }
    if _can_paint_pattern((*group).mask) != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"q\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_pdf_surface_paint_pattern(
            surface,
            CAIRO_OPERATOR_OVER,
            (*group).mask,
            &mut (*group).extents,
            1.0f64,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"Q\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        pattern_res.id = 0 as libc::c_int as libc::c_uint;
        gstate_res.id = 0 as libc::c_int as libc::c_uint;
        status = _cairo_pdf_surface_add_pdf_pattern(
            surface,
            (*group).mask,
            CAIRO_OPERATOR_OVER,
            0 as *const cairo_rectangle_int_t,
            &mut pattern_res,
            &mut gstate_res,
        );
        if status as u64 != 0 {
            return status;
        }
        if gstate_res.id != 0 as libc::c_int as libc::c_uint {
            smask_group = _cairo_pdf_surface_create_smask_group(
                surface,
                &mut (*group).extents,
            );
            if smask_group.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            (*smask_group).width = (*group).width;
            (*smask_group).height = (*group).height;
            (*smask_group).operation = PDF_PAINT;
            let ref mut fresh56 = (*smask_group).source;
            *fresh56 = cairo_pattern_reference((*group).mask);
            (*smask_group).source_res = pattern_res;
            status = _cairo_pdf_surface_add_smask_group(surface, smask_group);
            if status as u64 != 0 {
                _cairo_pdf_smask_group_destroy(smask_group);
                return status;
            }
            status = _cairo_pdf_surface_add_smask(surface, gstate_res);
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_pdf_surface_add_xobject(surface, (*smask_group).group_res);
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"q /s%d gs /x%d Do Q\n\0" as *const u8 as *const libc::c_char,
                gstate_res.id,
                (*smask_group).group_res.id,
            );
        } else {
            status = _cairo_pdf_surface_select_pattern(
                surface,
                (*group).mask,
                pattern_res,
                0 as libc::c_int,
            );
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"%f %f %f %f re f\n\0" as *const u8 as *const libc::c_char,
                bbox.p1.x,
                bbox.p1.y,
                bbox.p2.x - bbox.p1.x,
                bbox.p2.y - bbox.p1.y,
            );
            status = _cairo_pdf_surface_unselect_pattern(surface);
            if status as u64 != 0 {
                return status;
            }
        }
    }
    status = _cairo_pdf_surface_close_group(surface, &mut mask_group);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_surface_open_group(surface, &mut bbox, &mut (*group).source_res);
    if status as u64 != 0 {
        return status;
    }
    if _can_paint_pattern((*group).source) != 0 {
        _cairo_output_stream_printf(
            (*surface).output,
            b"q\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_pdf_surface_paint_pattern(
            surface,
            CAIRO_OPERATOR_OVER,
            (*group).source,
            &mut (*group).extents,
            1.0f64,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"Q\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        pattern_res.id = 0 as libc::c_int as libc::c_uint;
        gstate_res.id = 0 as libc::c_int as libc::c_uint;
        status = _cairo_pdf_surface_add_pdf_pattern(
            surface,
            (*group).source,
            CAIRO_OPERATOR_OVER,
            0 as *const cairo_rectangle_int_t,
            &mut pattern_res,
            &mut gstate_res,
        );
        if status as u64 != 0 {
            return status;
        }
        if gstate_res.id != 0 as libc::c_int as libc::c_uint {
            smask_group = _cairo_pdf_surface_create_smask_group(
                surface,
                &mut (*group).extents,
            );
            if smask_group.is_null() {
                return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
            }
            (*smask_group).operation = PDF_PAINT;
            let ref mut fresh57 = (*smask_group).source;
            *fresh57 = cairo_pattern_reference((*group).source);
            (*smask_group).source_res = pattern_res;
            status = _cairo_pdf_surface_add_smask_group(surface, smask_group);
            if status as u64 != 0 {
                _cairo_pdf_smask_group_destroy(smask_group);
                return status;
            }
            status = _cairo_pdf_surface_add_smask(surface, gstate_res);
            if status as u64 != 0 {
                return status;
            }
            status = _cairo_pdf_surface_add_xobject(surface, (*smask_group).group_res);
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"q /s%d gs /x%d Do Q\n\0" as *const u8 as *const libc::c_char,
                gstate_res.id,
                (*smask_group).group_res.id,
            );
        } else {
            status = _cairo_pdf_surface_select_pattern(
                surface,
                (*group).source,
                pattern_res,
                0 as libc::c_int,
            );
            if status as u64 != 0 {
                return status;
            }
            _cairo_output_stream_printf(
                (*surface).output,
                b"%f %f %f %f re f\n\0" as *const u8 as *const libc::c_char,
                bbox.p1.x,
                bbox.p1.y,
                bbox.p2.x - bbox.p1.x,
                bbox.p2.y - bbox.p1.y,
            );
            status = _cairo_pdf_surface_unselect_pattern(surface);
            if status as u64 != 0 {
                return status;
            }
        }
    }
    status = _cairo_pdf_surface_close_group(surface, 0 as *mut cairo_pdf_resource_t);
    if status as u64 != 0 {
        return status;
    }
    smask = _cairo_pdf_surface_new_object(surface);
    if smask.id == 0 as libc::c_int as libc::c_uint {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /Mask\n   /S /Alpha\n   /G %d 0 R\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        smask.id,
        mask_group.id,
    );
    _cairo_pdf_surface_update_object(surface, (*group).group_res);
    _cairo_output_stream_printf(
        (*surface).output,
        b"%d 0 obj\n<< /Type /ExtGState\n   /SMask %d 0 R\n   /ca 1\n   /CA 1\n   /AIS false\n>>\nendobj\n\0"
            as *const u8 as *const libc::c_char,
        (*group).group_res.id,
        smask.id,
    );
    return _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_write_smask_group(
    mut surface: *mut cairo_pdf_surface_t,
    mut group: *mut cairo_pdf_smask_group_t,
) -> cairo_int_status_t {
    let mut old_width: libc::c_double = 0.;
    let mut old_height: libc::c_double = 0.;
    let mut old_in_xobject: cairo_bool_t = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut bbox: cairo_box_double_t = cairo_box_double_t {
        p1: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        p2: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
    };
    let mut old_surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    old_width = (*surface).width;
    old_height = (*surface).height;
    old_surface_extents = (*surface).surface_extents;
    old_in_xobject = (*surface).in_xobject;
    (*surface).in_xobject = 1 as libc::c_int;
    _cairo_pdf_surface_set_size_internal(surface, (*group).width, (*group).height);
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    if (*group).operation as libc::c_uint == PDF_MASK as libc::c_int as libc::c_uint {
        status = _cairo_pdf_surface_write_mask_group(surface, group);
    } else {
        _get_bbox_from_extents(&mut (*group).extents, &mut bbox);
        status = _cairo_pdf_surface_open_group(
            surface,
            &mut bbox,
            &mut (*group).group_res,
        );
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_select_pattern(
            surface,
            (*group).source,
            (*group).source_res,
            ((*group).operation as libc::c_uint
                == PDF_STROKE as libc::c_int as libc::c_uint) as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
        match (*group).operation as libc::c_uint {
            0 => {
                _cairo_output_stream_printf(
                    (*surface).output,
                    b"0 0 %f %f re f\n\0" as *const u8 as *const libc::c_char,
                    (*surface).width,
                    (*surface).height,
                );
            }
            1 => {
                if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                    __assert_fail(
                        b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                        b"../src/cairo-pdf-surface.c\0" as *const u8
                            as *const libc::c_char,
                        7232 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 106],
                            &[libc::c_char; 106],
                        >(
                            b"cairo_int_status_t _cairo_pdf_surface_write_smask_group(cairo_pdf_surface_t *, cairo_pdf_smask_group_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            }
            2 => {
                status = _cairo_pdf_operators_fill(
                    &mut (*surface).pdf_operators,
                    &mut (*group).path,
                    (*group).fill_rule,
                );
            }
            3 => {
                status = _cairo_pdf_operators_stroke(
                    &mut (*surface).pdf_operators,
                    &mut (*group).path,
                    &mut (*group).style,
                    &mut (*group).ctm,
                    &mut (*group).ctm_inverse,
                );
            }
            4 => {
                status = _cairo_pdf_operators_show_text_glyphs(
                    &mut (*surface).pdf_operators,
                    (*group).utf8,
                    (*group).utf8_len,
                    (*group).glyphs,
                    (*group).num_glyphs,
                    (*group).clusters,
                    (*group).num_clusters,
                    (*group).cluster_flags as cairo_text_cluster_flags_t,
                    (*group).scaled_font,
                );
            }
            _ => {}
        }
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_unselect_pattern(surface);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_close_group(surface, 0 as *mut cairo_pdf_resource_t);
    }
    (*surface).in_xobject = old_in_xobject;
    _cairo_pdf_surface_set_size_internal(surface, old_width, old_height);
    (*surface).surface_extents = old_surface_extents;
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_write_patterns_and_smask_groups(
    mut surface: *mut cairo_pdf_surface_t,
    mut finish: cairo_bool_t,
) -> cairo_int_status_t {
    let mut pattern: cairo_pdf_pattern_t = cairo_pdf_pattern_t {
        width: 0.,
        height: 0.,
        extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        pattern: 0 as *mut cairo_pattern_t,
        pattern_res: cairo_pdf_resource_t { id: 0 },
        gstate_res: cairo_pdf_resource_t { id: 0 },
        operator: CAIRO_OPERATOR_CLEAR,
        is_shading: 0,
        inverted_y_axis: 0,
    };
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    let mut src_surface: cairo_pdf_source_surface_t = cairo_pdf_source_surface_t {
        type_0: CAIRO_PATTERN_TYPE_SOLID,
        surface: 0 as *mut cairo_surface_t,
        raster_pattern: 0 as *mut cairo_pattern_t,
        hash_entry: 0 as *mut cairo_pdf_source_surface_entry_t,
    };
    let mut pattern_index: libc::c_uint = 0;
    let mut group_index: libc::c_uint = 0;
    let mut surface_index: libc::c_uint = 0;
    let mut doc_surface_index: libc::c_uint = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut is_image: cairo_bool_t = 0;
    pattern_index = 0 as libc::c_int as libc::c_uint;
    group_index = 0 as libc::c_int as libc::c_uint;
    surface_index = 0 as libc::c_int as libc::c_uint;
    doc_surface_index = 0 as libc::c_int as libc::c_uint;
    while pattern_index < _cairo_array_num_elements(&mut (*surface).page_patterns)
        || group_index < _cairo_array_num_elements(&mut (*surface).smask_groups)
        || surface_index < _cairo_array_num_elements(&mut (*surface).page_surfaces)
        || finish != 0
            && doc_surface_index
                < _cairo_array_num_elements(&mut (*surface).doc_surfaces)
    {
        while group_index < _cairo_array_num_elements(&mut (*surface).smask_groups) {
            _cairo_array_copy_element(
                &mut (*surface).smask_groups,
                group_index,
                &mut group as *mut *mut cairo_pdf_smask_group_t as *mut libc::c_void,
            );
            status = _cairo_pdf_surface_write_smask_group(surface, group);
            if status as u64 != 0 {
                return status;
            }
            group_index = group_index.wrapping_add(1);
        }
        while pattern_index < _cairo_array_num_elements(&mut (*surface).page_patterns) {
            _cairo_array_copy_element(
                &mut (*surface).page_patterns,
                pattern_index,
                &mut pattern as *mut cairo_pdf_pattern_t as *mut libc::c_void,
            );
            status = _cairo_pdf_surface_emit_pattern(surface, &mut pattern);
            if status as u64 != 0 {
                return status;
            }
            pattern_index = pattern_index.wrapping_add(1);
        }
        while surface_index < _cairo_array_num_elements(&mut (*surface).page_surfaces) {
            _cairo_array_copy_element(
                &mut (*surface).page_surfaces,
                surface_index,
                &mut src_surface as *mut cairo_pdf_source_surface_t as *mut libc::c_void,
            );
            status = _cairo_pdf_surface_emit_surface(
                surface,
                &mut src_surface,
                0 as libc::c_int,
                &mut is_image,
            );
            if status as u64 != 0 {
                return status;
            }
            surface_index = surface_index.wrapping_add(1);
        }
        if finish != 0 {
            while doc_surface_index
                < _cairo_array_num_elements(&mut (*surface).doc_surfaces)
            {
                _cairo_array_copy_element(
                    &mut (*surface).doc_surfaces,
                    doc_surface_index,
                    &mut src_surface as *mut cairo_pdf_source_surface_t
                        as *mut libc::c_void,
                );
                status = _cairo_pdf_surface_emit_surface(
                    surface,
                    &mut src_surface,
                    0 as libc::c_int,
                    &mut is_image,
                );
                if status as u64 != 0 {
                    return status;
                }
                doc_surface_index = doc_surface_index.wrapping_add(1);
            }
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_write_page(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut knockout: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut thumbnail_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut page: *mut cairo_pdf_resource_t = 0 as *mut cairo_pdf_resource_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut page_num: libc::c_uint = 0;
    let mut num_annots: libc::c_uint = 0;
    status = _cairo_pdf_surface_open_object_stream(surface);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_interchange_write_page_objects(surface);
    if status as u64 != 0 {
        return status;
    }
    _cairo_pdf_group_resources_clear(&mut (*surface).resources);
    if (*surface).has_fallback_images != 0 {
        let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut bbox: cairo_box_double_t = cairo_box_double_t {
            p1: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            p2: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
        };
        extents.x = 0 as libc::c_int;
        extents.y = 0 as libc::c_int;
        extents.width = ceil((*surface).width) as libc::c_int;
        extents.height = ceil((*surface).height) as libc::c_int;
        _get_bbox_from_extents(&mut extents, &mut bbox);
        status = _cairo_pdf_surface_open_knockout_group(surface, &mut bbox);
        if status as u64 != 0 {
            return status;
        }
        len = _cairo_array_num_elements(&mut (*surface).knockout_group);
        i = 0 as libc::c_int as libc::c_uint;
        while i < len {
            _cairo_array_copy_element(
                &mut (*surface).knockout_group,
                i,
                &mut res as *mut cairo_pdf_resource_t as *mut libc::c_void,
            );
            _cairo_output_stream_printf(
                (*surface).output,
                b"/x%d Do\n\0" as *const u8 as *const libc::c_char,
                res.id,
            );
            status = _cairo_pdf_surface_add_xobject(surface, res);
            if status as u64 != 0 {
                return status;
            }
            i = i.wrapping_add(1);
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"/x%d Do\n\0" as *const u8 as *const libc::c_char,
            (*surface).content.id,
        );
        status = _cairo_pdf_surface_add_xobject(surface, (*surface).content);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_close_group(surface, &mut knockout);
        if status as u64 != 0 {
            return status;
        }
        _cairo_pdf_group_resources_clear(&mut (*surface).resources);
        status = _cairo_pdf_surface_open_content_stream(
            surface,
            0 as *const cairo_box_double_t,
            0 as *mut cairo_pdf_resource_t,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"/x%d Do\n\0" as *const u8 as *const libc::c_char,
            knockout.id,
        );
        status = _cairo_pdf_surface_add_xobject(surface, knockout);
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_surface_close_content_stream(surface, 0 as libc::c_int);
        if status as u64 != 0 {
            return status;
        }
    }
    thumbnail_res.id = 0 as libc::c_int as libc::c_uint;
    if !((*surface).thumbnail_image).is_null() {
        let mut entry: cairo_pdf_source_surface_entry_t = cairo_pdf_source_surface_entry_t {
            base: cairo_hash_entry_t { hash: 0 },
            id: 0,
            unique_id: 0 as *mut libc::c_uchar,
            unique_id_length: 0,
            operator: CAIRO_OPERATOR_CLEAR,
            interpolate: 0,
            stencil_mask: 0,
            smask: 0,
            need_transp_group: 0,
            surface_res: cairo_pdf_resource_t { id: 0 },
            smask_res: cairo_pdf_resource_t { id: 0 },
            emit_image: 0,
            bounded: 0,
            extents: cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            required_extents: cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        };
        memset(
            &mut entry as *mut cairo_pdf_source_surface_entry_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<cairo_pdf_source_surface_entry_t>() as libc::c_ulong,
        );
        thumbnail_res = _cairo_pdf_surface_new_object(surface);
        entry.surface_res = thumbnail_res;
        _cairo_pdf_surface_emit_image(surface, (*surface).thumbnail_image, &mut entry);
    }
    page_num = _cairo_array_num_elements(&mut (*surface).pages);
    page = _cairo_array_index(
        &mut (*surface).pages,
        page_num.wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) as *mut cairo_pdf_resource_t;
    status = _cairo_pdf_surface_object_begin(surface, *page);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b"<< /Type /Page %% %d\n   /Parent %d 0 R\n   /MediaBox [ 0 0 %f %f ]\n   /Contents %d 0 R\n   /Group <<\n      /Type /Group\n      /S /Transparency\n      /I true\n      /CS /DeviceRGB\n   >>\n   /Resources %d 0 R\n\0"
            as *const u8 as *const libc::c_char,
        page_num,
        (*surface).pages_resource.id,
        (*surface).width,
        (*surface).height,
        (*surface).content.id,
        (*surface).content_resources.id,
    );
    if (*surface).page_parent_tree >= 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /StructParents %d\n\0" as *const u8 as *const libc::c_char,
            (*surface).page_parent_tree,
        );
    }
    num_annots = _cairo_array_num_elements(&mut (*surface).page_annots);
    if num_annots > 0 as libc::c_int as libc::c_uint {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Annots [ \0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int as libc::c_uint;
        while i < num_annots {
            _cairo_array_copy_element(
                &mut (*surface).page_annots,
                i,
                &mut res as *mut cairo_pdf_resource_t as *mut libc::c_void,
            );
            _cairo_output_stream_printf(
                (*surface).object_stream.stream,
                b"%d 0 R \0" as *const u8 as *const libc::c_char,
                res.id,
            );
            i = i.wrapping_add(1);
        }
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"]\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if thumbnail_res.id != 0 {
        _cairo_output_stream_printf(
            (*surface).object_stream.stream,
            b"   /Thumb %d 0 R\n\0" as *const u8 as *const libc::c_char,
            thumbnail_res.id,
        );
    }
    _cairo_output_stream_printf(
        (*surface).object_stream.stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_surface_object_end(surface);
    status = _cairo_pdf_surface_write_patterns_and_smask_groups(
        surface,
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    return _cairo_pdf_surface_close_object_stream(surface);
}
unsafe extern "C" fn _cairo_pdf_surface_analyze_surface_pattern_transparency(
    mut surface: *mut cairo_pdf_surface_t,
    mut pattern: *mut cairo_surface_pattern_t,
) -> cairo_int_status_t {
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    status = _cairo_surface_acquire_source_image(
        (*pattern).surface,
        &mut image,
        &mut image_extra,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    if (*image).base.status as u64 != 0 {
        return (*image).base.status as cairo_int_status_t;
    }
    transparency = _cairo_image_analyze_transparency(image);
    if transparency as libc::c_uint
        == CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
    {
        status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    } else {
        status = CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY;
    }
    _cairo_surface_release_source_image((*pattern).surface, image, image_extra);
    return status;
}
unsafe extern "C" fn _surface_pattern_supported(
    mut pattern: *mut cairo_surface_pattern_t,
) -> cairo_bool_t {
    let mut extend: cairo_extend_t = CAIRO_EXTEND_NONE;
    if (*(*pattern).surface).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if ((*(*(*pattern).surface).backend).acquire_source_image).is_none() {
        return 0 as libc::c_int;
    }
    extend = cairo_pattern_get_extend(&mut (*pattern).base);
    match extend as libc::c_uint {
        0 | 1 | 2 | 3 => return 1 as libc::c_int,
        _ => {}
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            7535 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"cairo_bool_t _surface_pattern_supported(cairo_surface_pattern_t *)\0"))
                .as_ptr(),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _pattern_supported(
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    match (*pattern).type_0 as libc::c_uint {
        0 | 2 | 3 | 4 | 5 => return 1 as libc::c_int,
        1 => return _surface_pattern_supported(pattern as *mut cairo_surface_pattern_t),
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                    7554 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 57],
                        &[libc::c_char; 57],
                    >(b"cairo_bool_t _pattern_supported(const cairo_pattern_t *)\0"))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn _pdf_operator_supported(mut op: cairo_operator_t) -> cairo_bool_t {
    match op as libc::c_uint {
        2 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 => {
            return 1 as libc::c_int;
        }
        0 | 1 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | _ => {
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn _cairo_pdf_surface_analyze_operation(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    if (*surface).force_fallbacks != 0
        && (*surface).paginated_mode as libc::c_uint
            == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if _pattern_supported(pattern) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if _pdf_operator_supported(op) != 0 {
        if (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        {
            let mut surface_pattern: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            if (*(*surface_pattern).surface).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
            {
                if (*pattern).extend as libc::c_uint
                    == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
                {
                    let mut box_0: cairo_box_t = cairo_box_t {
                        p1: cairo_point_t { x: 0, y: 0 },
                        p2: cairo_point_t { x: 0, y: 0 },
                    };
                    let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                    };
                    let mut rec_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                    };
                    _cairo_box_from_rectangle(&mut box_0, extents);
                    _cairo_matrix_transform_bounding_box_fixed(
                        &(*pattern).matrix,
                        &mut box_0,
                        0 as *mut cairo_bool_t,
                    );
                    _cairo_box_round_to_rectangle(&mut box_0, &mut rect);
                    if _cairo_surface_get_extents(
                        (*surface_pattern).surface,
                        &mut rec_extents,
                    ) != 0
                    {
                        if _cairo_fixed_integer_ceil(box_0.p1.x) < rec_extents.x
                            || _cairo_fixed_integer_ceil(box_0.p1.y) < rec_extents.y
                            || _cairo_fixed_integer_floor(box_0.p2.y)
                                > rec_extents.x + rec_extents.width
                            || _cairo_fixed_integer_floor(box_0.p2.y)
                                > rec_extents.y + rec_extents.height
                        {
                            return CAIRO_INT_STATUS_UNSUPPORTED;
                        }
                    }
                }
                return CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN;
            }
        }
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
        if (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        {
            let mut surface_pattern_0: *mut cairo_surface_pattern_t = pattern
                as *mut cairo_surface_pattern_t;
            if (*(*surface_pattern_0).surface).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
            {
                if _cairo_pattern_is_opaque(pattern, extents) != 0 {
                    return CAIRO_INT_STATUS_ANALYZE_RECORDING_SURFACE_PATTERN
                } else {
                    return CAIRO_INT_STATUS_UNSUPPORTED
                }
            } else {
                return _cairo_pdf_surface_analyze_surface_pattern_transparency(
                    surface,
                    surface_pattern_0,
                )
            }
        }
        if _cairo_pattern_is_opaque(pattern, extents) != 0 {
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t
        } else {
            return CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY
        }
    }
    return CAIRO_INT_STATUS_UNSUPPORTED;
}
unsafe extern "C" fn _cairo_pdf_surface_operation_supported(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    return (_cairo_pdf_surface_analyze_operation(surface, op, pattern, extents)
        as libc::c_uint != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_surface_start_fallback(
    mut surface: *mut cairo_pdf_surface_t,
) -> cairo_int_status_t {
    let mut bbox: cairo_box_double_t = cairo_box_double_t {
        p1: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
        p2: cairo_point_double_t {
            x: 0.,
            y: 0.,
        },
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_pdf_surface_close_content_stream(surface, 0 as libc::c_int);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_array_append(
        &mut (*surface).knockout_group,
        &mut (*surface).content as *mut cairo_pdf_resource_t as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_pdf_group_resources_clear(&mut (*surface).resources);
    bbox.p1.x = 0 as libc::c_int as libc::c_double;
    bbox.p1.y = 0 as libc::c_int as libc::c_double;
    bbox.p2.x = (*surface).width;
    bbox.p2.y = (*surface).height;
    status = _cairo_pdf_surface_open_content_stream(
        surface,
        &mut bbox,
        0 as *mut cairo_pdf_resource_t,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    return _cairo_pdf_interchange_begin_page_content(surface);
}
unsafe extern "C" fn _cairo_pdf_surface_emit_combined_smask(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    let mut src_width: libc::c_int = 0;
    let mut src_height: libc::c_int = 0;
    let mut mask_width: libc::c_int = 0;
    let mut mask_height: libc::c_int = 0;
    let mut src_x_offset: libc::c_double = 0.;
    let mut src_y_offset: libc::c_double = 0.;
    let mut mask_x_offset: libc::c_double = 0.;
    let mut mask_y_offset: libc::c_double = 0.;
    let mut src_x1: libc::c_double = 0.;
    let mut src_y1: libc::c_double = 0.;
    let mut src_x2: libc::c_double = 0.;
    let mut src_y2: libc::c_double = 0.;
    let mut mask_x1: libc::c_double = 0.;
    let mut mask_y1: libc::c_double = 0.;
    let mut mask_x2: libc::c_double = 0.;
    let mut mask_y2: libc::c_double = 0.;
    let mut p2u: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut src_radius: libc::c_double = 0.;
    let mut mask_radius: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut need_smask: cairo_bool_t = 0;
    let mut pdf_source: *mut cairo_pdf_source_surface_entry_t = 0
        as *mut cairo_pdf_source_surface_entry_t;
    if !(((*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        || (*source).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint)
        && ((*mask).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
            || (*mask).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint))
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*(*(source as *mut cairo_surface_pattern_t)).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*mask).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*(*(mask as *mut cairo_surface_pattern_t)).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*source).extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
        || (*mask).extend as libc::c_uint
            != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_pdf_surface_acquire_source_image_from_pattern(
        surface,
        source,
        &mut image,
        &mut image_extra,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*image).base.status as u64 != 0 {
        return (*image).base.status as cairo_int_status_t;
    }
    src_width = (*image).width;
    src_height = (*image).height;
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        cairo_surface_get_device_offset(
            &mut (*image).base,
            &mut src_x_offset,
            &mut src_y_offset,
        );
    } else {
        src_x_offset = 0 as libc::c_int as libc::c_double;
        src_y_offset = 0 as libc::c_int as libc::c_double;
    }
    transparency = _cairo_image_analyze_transparency(image);
    _cairo_pdf_surface_release_source_image_from_pattern(
        surface,
        source,
        image,
        image_extra,
    );
    if transparency as libc::c_uint
        != CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_pdf_surface_acquire_source_image_from_pattern(
        surface,
        mask,
        &mut image,
        &mut image_extra,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*image).base.status as u64 != 0 {
        return (*image).base.status as cairo_int_status_t;
    }
    mask_width = (*image).width;
    mask_height = (*image).height;
    if (*mask).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        cairo_surface_get_device_offset(
            &mut (*image).base,
            &mut mask_x_offset,
            &mut mask_y_offset,
        );
    } else {
        mask_x_offset = 0 as libc::c_int as libc::c_double;
        mask_y_offset = 0 as libc::c_int as libc::c_double;
    }
    transparency = _cairo_image_analyze_transparency(image);
    need_smask = (transparency as libc::c_uint
        != CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint) as libc::c_int;
    _cairo_pdf_surface_release_source_image_from_pattern(
        surface,
        mask,
        image,
        image_extra,
    );
    p2u = (*source).matrix;
    status = cairo_matrix_invert(&mut p2u) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            7817 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 180],
                &[libc::c_char; 180],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_combined_smask(cairo_pdf_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    src_x1 = 0 as libc::c_int as libc::c_double;
    src_y1 = 0 as libc::c_int as libc::c_double;
    src_x2 = src_width as libc::c_double;
    src_y2 = src_height as libc::c_double;
    cairo_matrix_transform_point(&mut p2u, &mut src_x1, &mut src_y1);
    cairo_matrix_transform_point(&mut p2u, &mut src_x2, &mut src_y2);
    src_radius = _cairo_matrix_transformed_circle_major_axis(&mut p2u, 0.5f64);
    p2u = (*mask).matrix;
    status = cairo_matrix_invert(&mut p2u) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_INT_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
            7829 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 180],
                &[libc::c_char; 180],
            >(
                b"cairo_int_status_t _cairo_pdf_surface_emit_combined_smask(cairo_pdf_surface_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    mask_x1 = 0 as libc::c_int as libc::c_double;
    mask_y1 = 0 as libc::c_int as libc::c_double;
    mask_x2 = mask_width as libc::c_double;
    mask_y2 = mask_height as libc::c_double;
    cairo_matrix_transform_point(&mut p2u, &mut mask_x1, &mut mask_y1);
    cairo_matrix_transform_point(&mut p2u, &mut mask_x2, &mut mask_y2);
    mask_radius = _cairo_matrix_transformed_circle_major_axis(&mut p2u, 0.5f64);
    if src_radius < mask_radius {
        e = src_radius;
    } else {
        e = mask_radius;
    }
    if fabs(src_x1 - mask_x1) > e || fabs(src_x2 - mask_x2) > e
        || fabs(src_y1 - mask_y1) > e || fabs(src_y2 - mask_y2) > e
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if fabs(src_x_offset - mask_x_offset) > e || fabs(src_y_offset - mask_y_offset) > e {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if need_smask != 0 {
        status = _cairo_pdf_surface_add_source_surface(
            surface,
            0 as *mut cairo_surface_t,
            mask,
            op,
            (*source).filter,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            extents,
            0 as *mut cairo_pdf_resource_t,
            &mut pdf_source,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut cairo_rectangle_int_t,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"q\n\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_pdf_surface_paint_surface_pattern(
        surface,
        op,
        source,
        extents,
        1.0f64,
        if need_smask != 0 {
            &mut (*pdf_source).surface_res
        } else {
            0 as *mut cairo_pdf_resource_t
        },
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).output,
        b"Q\n\0" as *const u8 as *const libc::c_char,
    );
    status = _cairo_output_stream_get_status((*surface).output) as cairo_int_status_t;
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_emit_stencil_mask(
    mut surface: *mut cairo_pdf_surface_t,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    let mut pattern_res: cairo_pdf_resource_t = {
        let mut init = _cairo_pdf_resource {
            id: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    if !((*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
        && ((*mask).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
            || (*mask).type_0 as libc::c_uint
                == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint))
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*mask).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*(*(mask as *mut cairo_surface_pattern_t)).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_pdf_surface_acquire_source_image_from_pattern(
        surface,
        mask,
        &mut image,
        &mut image_extra,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*image).base.status as u64 != 0 {
        return (*image).base.status as cairo_int_status_t;
    }
    transparency = _cairo_image_analyze_transparency(image);
    if transparency as libc::c_uint
        != CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
        && transparency as libc::c_uint
            != CAIRO_IMAGE_HAS_BILEVEL_ALPHA as libc::c_int as libc::c_uint
    {
        status = CAIRO_INT_STATUS_UNSUPPORTED;
    } else {
        status = _cairo_pdf_surface_select_pattern(
            surface,
            source,
            pattern_res,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
        status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
            as cairo_int_status_t;
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"q\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_pdf_surface_paint_surface_pattern(
            surface,
            op,
            mask,
            extents,
            1.0f64,
            0 as *mut cairo_pdf_resource_t,
            1 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            (*surface).output,
            b"Q\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_output_stream_get_status((*surface).output)
            as cairo_int_status_t;
    }
    _cairo_pdf_surface_release_source_image_from_pattern(
        surface,
        mask,
        image,
        image_extra,
    );
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_set_clip(
    mut surface: *mut cairo_pdf_surface_t,
    mut composite: *mut cairo_composite_rectangles_t,
) -> cairo_int_status_t {
    let mut clip: *mut cairo_clip_t = (*composite).clip;
    if _cairo_composite_rectangles_can_reduce_clip(composite, clip) != 0 {
        clip = 0 as *mut cairo_clip_t;
    }
    if clip.is_null() {
        if _cairo_composite_rectangles_can_reduce_clip(
            composite,
            (*surface).clipper.clip,
        ) != 0
        {
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    }
    return _cairo_surface_clipper_set_clip(&mut (*surface).clipper, clip)
        as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_pdf_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    let mut pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut extents: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_composite_rectangles_init_for_paint(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        clip,
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_interchange_add_operation_extents(surface, &mut extents.bounded);
    if status as u64 != 0 {
        return status;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_pdf_surface_analyze_operation(
            surface,
            op,
            source,
            &mut extents.bounded,
        );
    } else {
        if (*surface).paginated_mode as libc::c_uint
            == CAIRO_PAGINATED_MODE_FALLBACK as libc::c_int as libc::c_uint
        {
            status = _cairo_pdf_surface_start_fallback(surface);
            if status as u64 != 0 {
                current_block = 12877183573174693035;
            } else {
                current_block = 4166486009154926805;
            }
        } else {
            current_block = 4166486009154926805;
        }
        match current_block {
            12877183573174693035 => {}
            _ => {
                if _cairo_pdf_surface_operation_supported(
                    surface,
                    op,
                    source,
                    &mut extents.bounded,
                ) != 0
                {} else {
                    __assert_fail(
                        b"_cairo_pdf_surface_operation_supported (surface, op, source, &extents.bounded)\0"
                            as *const u8 as *const libc::c_char,
                        b"../src/cairo-pdf-surface.c\0" as *const u8
                            as *const libc::c_char,
                        8006 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 117],
                            &[libc::c_char; 117],
                        >(
                            b"cairo_int_status_t _cairo_pdf_surface_paint(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_clip_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                status = _cairo_pdf_surface_set_clip(surface, &mut extents);
                if !(status as u64 != 0) {
                    status = _cairo_pdf_surface_select_operator(surface, op);
                    if !(status as u64 != 0) {
                        status = _cairo_pdf_operators_flush(
                            &mut (*surface).pdf_operators,
                        ) as cairo_int_status_t;
                        if !(status as u64 != 0) {
                            if _can_paint_pattern(source) != 0 {
                                _cairo_output_stream_printf(
                                    (*surface).output,
                                    b"q\n\0" as *const u8 as *const libc::c_char,
                                );
                                status = _cairo_pdf_surface_paint_pattern(
                                    surface,
                                    op,
                                    source,
                                    &mut extents.bounded,
                                    1.0f64,
                                    0 as libc::c_int,
                                );
                                if !(status as u64 != 0) {
                                    _cairo_output_stream_printf(
                                        (*surface).output,
                                        b"Q\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    _cairo_composite_rectangles_fini(&mut extents);
                                    return _cairo_output_stream_get_status((*surface).output)
                                        as cairo_int_status_t;
                                }
                            } else {
                                pattern_res.id = 0 as libc::c_int as libc::c_uint;
                                gstate_res.id = 0 as libc::c_int as libc::c_uint;
                                status = _cairo_pdf_surface_add_pdf_pattern(
                                    surface,
                                    source,
                                    op,
                                    &mut extents.bounded,
                                    &mut pattern_res,
                                    &mut gstate_res,
                                );
                                if !(status as u64 != 0) {
                                    if gstate_res.id != 0 as libc::c_int as libc::c_uint {
                                        group = _cairo_pdf_surface_create_smask_group(
                                            surface,
                                            &mut extents.bounded,
                                        );
                                        if group.is_null() {
                                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                as cairo_int_status_t;
                                            current_block = 12877183573174693035;
                                        } else {
                                            (*group).operation = PDF_PAINT;
                                            status = _cairo_pattern_create_copy(
                                                &mut (*group).source,
                                                source,
                                            ) as cairo_int_status_t;
                                            if status as u64 != 0 {
                                                _cairo_pdf_smask_group_destroy(group);
                                                current_block = 12877183573174693035;
                                            } else {
                                                (*group).source_res = pattern_res;
                                                status = _cairo_pdf_surface_add_smask_group(surface, group);
                                                if status as u64 != 0 {
                                                    _cairo_pdf_smask_group_destroy(group);
                                                    current_block = 12877183573174693035;
                                                } else {
                                                    status = _cairo_pdf_surface_add_smask(surface, gstate_res);
                                                    if status as u64 != 0 {
                                                        current_block = 12877183573174693035;
                                                    } else {
                                                        status = _cairo_pdf_surface_add_xobject(
                                                            surface,
                                                            (*group).group_res,
                                                        );
                                                        if status as u64 != 0 {
                                                            current_block = 12877183573174693035;
                                                        } else {
                                                            _cairo_output_stream_printf(
                                                                (*surface).output,
                                                                b"q /s%d gs /x%d Do Q\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                gstate_res.id,
                                                                (*group).group_res.id,
                                                            );
                                                            current_block = 13763002826403452995;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        status = _cairo_pdf_surface_select_pattern(
                                            surface,
                                            source,
                                            pattern_res,
                                            0 as libc::c_int,
                                        );
                                        if status as u64 != 0 {
                                            current_block = 12877183573174693035;
                                        } else {
                                            _cairo_output_stream_printf(
                                                (*surface).output,
                                                b"%d %d %d %d re f\n\0" as *const u8 as *const libc::c_char,
                                                (*surface).surface_extents.x,
                                                (*surface).surface_extents.y,
                                                (*surface).surface_extents.width,
                                                (*surface).surface_extents.height,
                                            );
                                            status = _cairo_pdf_surface_unselect_pattern(surface);
                                            if status as u64 != 0 {
                                                current_block = 12877183573174693035;
                                            } else {
                                                current_block = 13763002826403452995;
                                            }
                                        }
                                    }
                                    match current_block {
                                        12877183573174693035 => {}
                                        _ => {
                                            _cairo_composite_rectangles_fini(&mut extents);
                                            return _cairo_output_stream_get_status((*surface).output)
                                                as cairo_int_status_t;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    let mut extents: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut alpha: libc::c_double = 0.;
    status = _cairo_composite_rectangles_init_for_mask(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        mask,
        clip,
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_interchange_add_operation_extents(surface, &mut extents.bounded);
    if status as u64 != 0 {
        return status;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        let mut source_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
        let mut mask_status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
        status = _cairo_pdf_surface_analyze_operation(
            surface,
            op,
            source,
            &mut extents.bounded,
        );
        if !(status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && (status as libc::c_uint)
                < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint)
        {
            source_status = status;
            if (*mask).has_component_alpha != 0 {
                status = CAIRO_INT_STATUS_UNSUPPORTED;
                current_block = 15904375183555213903;
            } else {
                status = _cairo_pdf_surface_analyze_operation(
                    surface,
                    op,
                    mask,
                    &mut extents.bounded,
                );
                if status as libc::c_uint
                    != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
                    && (status as libc::c_uint)
                        < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
                {
                    current_block = 17961593684671207894;
                } else {
                    current_block = 15904375183555213903;
                }
            }
            match current_block {
                17961593684671207894 => {}
                _ => {
                    mask_status = status;
                    _cairo_composite_rectangles_fini(&mut extents);
                    return _cairo_analysis_surface_merge_status(
                        source_status,
                        mask_status,
                    );
                }
            }
        }
    } else {
        if (*surface).paginated_mode as libc::c_uint
            == CAIRO_PAGINATED_MODE_FALLBACK as libc::c_int as libc::c_uint
        {
            status = _cairo_pdf_surface_start_fallback(surface);
            if status as u64 != 0 {
                current_block = 17961593684671207894;
            } else {
                current_block = 2719512138335094285;
            }
        } else {
            current_block = 2719512138335094285;
        }
        match current_block {
            17961593684671207894 => {}
            _ => {
                if _cairo_pdf_surface_operation_supported(
                    surface,
                    op,
                    source,
                    &mut extents.bounded,
                ) != 0
                {} else {
                    __assert_fail(
                        b"_cairo_pdf_surface_operation_supported (surface, op, source, &extents.bounded)\0"
                            as *const u8 as *const libc::c_char,
                        b"../src/cairo-pdf-surface.c\0" as *const u8
                            as *const libc::c_char,
                        8153 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 141],
                            &[libc::c_char; 141],
                        >(
                            b"cairo_int_status_t _cairo_pdf_surface_mask(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_clip_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if _cairo_pdf_surface_operation_supported(
                    surface,
                    op,
                    mask,
                    &mut extents.bounded,
                ) != 0
                {} else {
                    __assert_fail(
                        b"_cairo_pdf_surface_operation_supported (surface, op, mask, &extents.bounded)\0"
                            as *const u8 as *const libc::c_char,
                        b"../src/cairo-pdf-surface.c\0" as *const u8
                            as *const libc::c_char,
                        8154 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 141],
                            &[libc::c_char; 141],
                        >(
                            b"cairo_int_status_t _cairo_pdf_surface_mask(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_clip_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                status = _cairo_pattern_get_ink_extents(source, &mut r);
                if !(status as u64 != 0) {
                    _cairo_box_from_rectangle(&mut box_0, &mut r);
                    status = _cairo_composite_rectangles_intersect_source_extents(
                        &mut extents,
                        &mut box_0,
                    );
                    if !(status as u64 != 0) {
                        status = _cairo_pattern_get_ink_extents(mask, &mut r);
                        if !(status as u64 != 0) {
                            _cairo_box_from_rectangle(&mut box_0, &mut r);
                            status = _cairo_composite_rectangles_intersect_mask_extents(
                                &mut extents,
                                &mut box_0,
                            );
                            if !(status as u64 != 0) {
                                status = _cairo_pdf_surface_set_clip(surface, &mut extents);
                                if !(status as u64 != 0) {
                                    status = _cairo_pdf_surface_select_operator(surface, op);
                                    if !(status as u64 != 0) {
                                        status = _cairo_pdf_surface_emit_combined_smask(
                                            surface,
                                            op,
                                            source,
                                            mask,
                                            &mut extents.bounded,
                                        );
                                        if !(status as libc::c_uint
                                            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                                as libc::c_uint)
                                        {
                                            status = _cairo_pdf_surface_emit_stencil_mask(
                                                surface,
                                                op,
                                                source,
                                                mask,
                                                &mut extents.bounded,
                                            );
                                            if !(status as libc::c_uint
                                                != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int
                                                    as libc::c_uint)
                                            {
                                                if _cairo_pattern_is_constant_alpha(
                                                    mask,
                                                    &mut extents.bounded,
                                                    &mut alpha,
                                                ) != 0 && _can_paint_pattern(source) != 0
                                                {
                                                    status = _cairo_pdf_operators_flush(
                                                        &mut (*surface).pdf_operators,
                                                    ) as cairo_int_status_t;
                                                    if !(status as u64 != 0) {
                                                        _cairo_output_stream_printf(
                                                            (*surface).output,
                                                            b"q\n\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        status = _cairo_pdf_surface_paint_pattern(
                                                            surface,
                                                            op,
                                                            source,
                                                            &mut extents.bounded,
                                                            alpha,
                                                            0 as libc::c_int,
                                                        );
                                                        if !(status as u64 != 0) {
                                                            _cairo_output_stream_printf(
                                                                (*surface).output,
                                                                b"Q\n\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            _cairo_composite_rectangles_fini(&mut extents);
                                                            return _cairo_output_stream_get_status((*surface).output)
                                                                as cairo_int_status_t;
                                                        }
                                                    }
                                                } else {
                                                    group = _cairo_pdf_surface_create_smask_group(
                                                        surface,
                                                        &mut extents.bounded,
                                                    );
                                                    if group.is_null() {
                                                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                            as cairo_int_status_t;
                                                    } else {
                                                        (*group).operation = PDF_MASK;
                                                        status = _cairo_pattern_create_copy(
                                                            &mut (*group).source,
                                                            source,
                                                        ) as cairo_int_status_t;
                                                        if status as u64 != 0 {
                                                            _cairo_pdf_smask_group_destroy(group);
                                                        } else {
                                                            status = _cairo_pattern_create_copy(
                                                                &mut (*group).mask,
                                                                mask,
                                                            ) as cairo_int_status_t;
                                                            if status as u64 != 0 {
                                                                _cairo_pdf_smask_group_destroy(group);
                                                            } else {
                                                                (*group)
                                                                    .source_res = _cairo_pdf_surface_new_object(surface);
                                                                if (*group).source_res.id
                                                                    == 0 as libc::c_int as libc::c_uint
                                                                {
                                                                    _cairo_pdf_smask_group_destroy(group);
                                                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                                        as cairo_int_status_t;
                                                                } else {
                                                                    status = _cairo_pdf_surface_add_smask_group(surface, group);
                                                                    if status as u64 != 0 {
                                                                        _cairo_pdf_smask_group_destroy(group);
                                                                    } else {
                                                                        status = _cairo_pdf_surface_add_smask(
                                                                            surface,
                                                                            (*group).group_res,
                                                                        );
                                                                        if !(status as u64 != 0) {
                                                                            status = _cairo_pdf_surface_add_xobject(
                                                                                surface,
                                                                                (*group).source_res,
                                                                            );
                                                                            if !(status as u64 != 0) {
                                                                                status = _cairo_pdf_operators_flush(
                                                                                    &mut (*surface).pdf_operators,
                                                                                ) as cairo_int_status_t;
                                                                                if !(status as u64 != 0) {
                                                                                    _cairo_output_stream_printf(
                                                                                        (*surface).output,
                                                                                        b"q /s%d gs /x%d Do Q\n\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                        (*group).group_res.id,
                                                                                        (*group).source_res.id,
                                                                                    );
                                                                                    _cairo_composite_rectangles_fini(&mut extents);
                                                                                    return _cairo_output_stream_get_status((*surface).output)
                                                                                        as cairo_int_status_t;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_stroke(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut style: *const cairo_stroke_style_t,
    mut ctm: *const cairo_matrix_t,
    mut ctm_inverse: *const cairo_matrix_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    let mut pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut extents: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_composite_rectangles_init_for_stroke(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        path,
        style,
        ctm,
        clip,
    );
    if status as u64 != 0 {
        return status;
    }
    if extents.is_bounded != 0 {
        let mut mask: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        status = _cairo_path_fixed_stroke_extents(
            path,
            style,
            ctm,
            ctm_inverse,
            tolerance,
            &mut mask,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            current_block = 2098576368466208088;
        } else {
            _cairo_box_from_rectangle(&mut box_0, &mut mask);
            status = _cairo_composite_rectangles_intersect_mask_extents(
                &mut extents,
                &mut box_0,
            );
            if status as u64 != 0 {
                current_block = 2098576368466208088;
            } else {
                current_block = 2979737022853876585;
            }
        }
    } else {
        current_block = 2979737022853876585;
    }
    match current_block {
        2979737022853876585 => {
            status = _cairo_pdf_interchange_add_operation_extents(
                surface,
                &mut extents.bounded,
            );
            if !(status as u64 != 0) {
                if (*surface).paginated_mode as libc::c_uint
                    == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
                {
                    status = _cairo_pdf_surface_analyze_operation(
                        surface,
                        op,
                        source,
                        &mut extents.bounded,
                    );
                } else {
                    if _cairo_pdf_surface_operation_supported(
                        surface,
                        op,
                        source,
                        &mut extents.bounded,
                    ) != 0
                    {} else {
                        __assert_fail(
                            b"_cairo_pdf_surface_operation_supported (surface, op, source, &extents.bounded)\0"
                                as *const u8 as *const libc::c_char,
                            b"../src/cairo-pdf-surface.c\0" as *const u8
                                as *const libc::c_char,
                            8330 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 251],
                                &[libc::c_char; 251],
                            >(
                                b"cairo_int_status_t _cairo_pdf_surface_stroke(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    status = _cairo_pdf_surface_set_clip(surface, &mut extents);
                    if !(status as u64 != 0) {
                        pattern_res.id = 0 as libc::c_int as libc::c_uint;
                        gstate_res.id = 0 as libc::c_int as libc::c_uint;
                        status = _cairo_pdf_surface_add_pdf_pattern(
                            surface,
                            source,
                            op,
                            &mut extents.bounded,
                            &mut pattern_res,
                            &mut gstate_res,
                        );
                        if !(status as u64 != 0) {
                            status = _cairo_pdf_surface_select_operator(surface, op);
                            if !(status as u64 != 0) {
                                if gstate_res.id != 0 as libc::c_int as libc::c_uint {
                                    group = _cairo_pdf_surface_create_smask_group(
                                        surface,
                                        &mut extents.bounded,
                                    );
                                    if group.is_null() {
                                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                            as cairo_int_status_t;
                                        current_block = 2098576368466208088;
                                    } else {
                                        (*group).operation = PDF_STROKE;
                                        status = _cairo_pattern_create_copy(
                                            &mut (*group).source,
                                            source,
                                        ) as cairo_int_status_t;
                                        if status as u64 != 0 {
                                            _cairo_pdf_smask_group_destroy(group);
                                            current_block = 2098576368466208088;
                                        } else {
                                            (*group).source_res = pattern_res;
                                            status = _cairo_path_fixed_init_copy(
                                                &mut (*group).path,
                                                path,
                                            ) as cairo_int_status_t;
                                            if status as u64 != 0 {
                                                _cairo_pdf_smask_group_destroy(group);
                                                current_block = 2098576368466208088;
                                            } else {
                                                (*group).style = *style;
                                                (*group).ctm = *ctm;
                                                (*group).ctm_inverse = *ctm_inverse;
                                                status = _cairo_pdf_surface_add_smask_group(surface, group);
                                                if status as u64 != 0 {
                                                    _cairo_pdf_smask_group_destroy(group);
                                                    current_block = 2098576368466208088;
                                                } else {
                                                    status = _cairo_pdf_surface_add_smask(surface, gstate_res);
                                                    if status as u64 != 0 {
                                                        current_block = 2098576368466208088;
                                                    } else {
                                                        status = _cairo_pdf_surface_add_xobject(
                                                            surface,
                                                            (*group).group_res,
                                                        );
                                                        if status as u64 != 0 {
                                                            current_block = 2098576368466208088;
                                                        } else {
                                                            status = _cairo_pdf_operators_flush(
                                                                &mut (*surface).pdf_operators,
                                                            ) as cairo_int_status_t;
                                                            if status as u64 != 0 {
                                                                current_block = 2098576368466208088;
                                                            } else {
                                                                _cairo_output_stream_printf(
                                                                    (*surface).output,
                                                                    b"q /s%d gs /x%d Do Q\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    gstate_res.id,
                                                                    (*group).group_res.id,
                                                                );
                                                                current_block = 9859671972921157070;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    status = _cairo_pdf_surface_select_pattern(
                                        surface,
                                        source,
                                        pattern_res,
                                        1 as libc::c_int,
                                    );
                                    if status as u64 != 0 {
                                        current_block = 2098576368466208088;
                                    } else {
                                        status = _cairo_pdf_operators_stroke(
                                            &mut (*surface).pdf_operators,
                                            path,
                                            style,
                                            ctm,
                                            ctm_inverse,
                                        );
                                        if status as u64 != 0 {
                                            current_block = 2098576368466208088;
                                        } else {
                                            status = _cairo_pdf_surface_unselect_pattern(surface);
                                            if status as u64 != 0 {
                                                current_block = 2098576368466208088;
                                            } else {
                                                current_block = 9859671972921157070;
                                            }
                                        }
                                    }
                                }
                                match current_block {
                                    2098576368466208088 => {}
                                    _ => {
                                        _cairo_composite_rectangles_fini(&mut extents);
                                        return _cairo_output_stream_get_status((*surface).output)
                                            as cairo_int_status_t;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    let mut pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut extents: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    status = _cairo_composite_rectangles_init_for_fill(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        path,
        clip,
    );
    if status as u64 != 0 {
        return status;
    }
    if extents.is_bounded != 0 {
        let mut mask: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        _cairo_path_fixed_fill_extents(path, fill_rule, tolerance, &mut mask);
        _cairo_box_from_rectangle(&mut box_0, &mut mask);
        status = _cairo_composite_rectangles_intersect_mask_extents(
            &mut extents,
            &mut box_0,
        );
        if status as u64 != 0 {
            current_block = 4268859112086432337;
        } else {
            current_block = 2968425633554183086;
        }
    } else {
        current_block = 2968425633554183086;
    }
    match current_block {
        2968425633554183086 => {
            status = _cairo_pdf_interchange_add_operation_extents(
                surface,
                &mut extents.bounded,
            );
            if !(status as u64 != 0) {
                if (*surface).paginated_mode as libc::c_uint
                    == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
                {
                    status = _cairo_pdf_surface_analyze_operation(
                        surface,
                        op,
                        source,
                        &mut extents.bounded,
                    );
                } else {
                    if (*surface).paginated_mode as libc::c_uint
                        == CAIRO_PAGINATED_MODE_FALLBACK as libc::c_int as libc::c_uint
                    {
                        status = _cairo_pdf_surface_start_fallback(surface);
                        if status as u64 != 0 {
                            current_block = 4268859112086432337;
                        } else {
                            current_block = 2838571290723028321;
                        }
                    } else {
                        current_block = 2838571290723028321;
                    }
                    match current_block {
                        4268859112086432337 => {}
                        _ => {
                            if _cairo_pdf_surface_operation_supported(
                                surface,
                                op,
                                source,
                                &mut extents.bounded,
                            ) != 0
                            {} else {
                                __assert_fail(
                                    b"_cairo_pdf_surface_operation_supported (surface, op, source, &extents.bounded)\0"
                                        as *const u8 as *const libc::c_char,
                                    b"../src/cairo-pdf-surface.c\0" as *const u8
                                        as *const libc::c_char,
                                    8472 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 190],
                                        &[libc::c_char; 190],
                                    >(
                                        b"cairo_int_status_t _cairo_pdf_surface_fill(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_clip_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            status = _cairo_pdf_surface_set_clip(surface, &mut extents);
                            if !(status as u64 != 0) {
                                status = _cairo_pdf_surface_select_operator(surface, op);
                                if !(status as u64 != 0) {
                                    if _can_paint_pattern(source) != 0 {
                                        status = _cairo_pdf_operators_flush(
                                            &mut (*surface).pdf_operators,
                                        ) as cairo_int_status_t;
                                        if !(status as u64 != 0) {
                                            _cairo_output_stream_printf(
                                                (*surface).output,
                                                b"q\n\0" as *const u8 as *const libc::c_char,
                                            );
                                            status = _cairo_pdf_operators_clip(
                                                &mut (*surface).pdf_operators,
                                                path,
                                                fill_rule,
                                            );
                                            if !(status as u64 != 0) {
                                                status = _cairo_pdf_surface_paint_pattern(
                                                    surface,
                                                    op,
                                                    source,
                                                    &mut extents.bounded,
                                                    1.0f64,
                                                    0 as libc::c_int,
                                                );
                                                if !(status as u64 != 0) {
                                                    _cairo_output_stream_printf(
                                                        (*surface).output,
                                                        b"Q\n\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    status = _cairo_output_stream_get_status((*surface).output)
                                                        as cairo_int_status_t;
                                                }
                                            }
                                        }
                                    } else {
                                        pattern_res.id = 0 as libc::c_int as libc::c_uint;
                                        gstate_res.id = 0 as libc::c_int as libc::c_uint;
                                        status = _cairo_pdf_surface_add_pdf_pattern(
                                            surface,
                                            source,
                                            op,
                                            &mut extents.bounded,
                                            &mut pattern_res,
                                            &mut gstate_res,
                                        );
                                        if !(status as u64 != 0) {
                                            if gstate_res.id != 0 as libc::c_int as libc::c_uint {
                                                group = _cairo_pdf_surface_create_smask_group(
                                                    surface,
                                                    &mut extents.bounded,
                                                );
                                                if group.is_null() {
                                                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                        as cairo_int_status_t;
                                                    current_block = 4268859112086432337;
                                                } else {
                                                    (*group).operation = PDF_FILL;
                                                    status = _cairo_pattern_create_copy(
                                                        &mut (*group).source,
                                                        source,
                                                    ) as cairo_int_status_t;
                                                    if status as u64 != 0 {
                                                        _cairo_pdf_smask_group_destroy(group);
                                                        current_block = 4268859112086432337;
                                                    } else {
                                                        (*group).source_res = pattern_res;
                                                        status = _cairo_path_fixed_init_copy(
                                                            &mut (*group).path,
                                                            path,
                                                        ) as cairo_int_status_t;
                                                        if status as u64 != 0 {
                                                            _cairo_pdf_smask_group_destroy(group);
                                                            current_block = 4268859112086432337;
                                                        } else {
                                                            (*group).fill_rule = fill_rule;
                                                            status = _cairo_pdf_surface_add_smask_group(surface, group);
                                                            if status as u64 != 0 {
                                                                _cairo_pdf_smask_group_destroy(group);
                                                                current_block = 4268859112086432337;
                                                            } else {
                                                                status = _cairo_pdf_surface_add_smask(surface, gstate_res);
                                                                if status as u64 != 0 {
                                                                    current_block = 4268859112086432337;
                                                                } else {
                                                                    status = _cairo_pdf_surface_add_xobject(
                                                                        surface,
                                                                        (*group).group_res,
                                                                    );
                                                                    if status as u64 != 0 {
                                                                        current_block = 4268859112086432337;
                                                                    } else {
                                                                        status = _cairo_pdf_operators_flush(
                                                                            &mut (*surface).pdf_operators,
                                                                        ) as cairo_int_status_t;
                                                                        if status as u64 != 0 {
                                                                            current_block = 4268859112086432337;
                                                                        } else {
                                                                            _cairo_output_stream_printf(
                                                                                (*surface).output,
                                                                                b"q /s%d gs /x%d Do Q\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                gstate_res.id,
                                                                                (*group).group_res.id,
                                                                            );
                                                                            current_block = 2606304779496145856;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                status = _cairo_pdf_surface_select_pattern(
                                                    surface,
                                                    source,
                                                    pattern_res,
                                                    0 as libc::c_int,
                                                );
                                                if status as u64 != 0 {
                                                    current_block = 4268859112086432337;
                                                } else {
                                                    status = _cairo_pdf_operators_fill(
                                                        &mut (*surface).pdf_operators,
                                                        path,
                                                        fill_rule,
                                                    );
                                                    if status as u64 != 0 {
                                                        current_block = 4268859112086432337;
                                                    } else {
                                                        status = _cairo_pdf_surface_unselect_pattern(surface);
                                                        if status as u64 != 0 {
                                                            current_block = 4268859112086432337;
                                                        } else {
                                                            current_block = 2606304779496145856;
                                                        }
                                                    }
                                                }
                                            }
                                            match current_block {
                                                4268859112086432337 => {}
                                                _ => {
                                                    _cairo_composite_rectangles_fini(&mut extents);
                                                    return _cairo_output_stream_get_status((*surface).output)
                                                        as cairo_int_status_t;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_fill_stroke(
    mut abstract_surface: *mut libc::c_void,
    mut fill_op: cairo_operator_t,
    mut fill_source: *const cairo_pattern_t,
    mut fill_rule: cairo_fill_rule_t,
    mut fill_tolerance: libc::c_double,
    mut fill_antialias: cairo_antialias_t,
    mut path: *const cairo_path_fixed_t,
    mut stroke_op: cairo_operator_t,
    mut stroke_source: *const cairo_pattern_t,
    mut stroke_style: *const cairo_stroke_style_t,
    mut stroke_ctm: *const cairo_matrix_t,
    mut stroke_ctm_inverse: *const cairo_matrix_t,
    mut stroke_tolerance: libc::c_double,
    mut stroke_antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut fill_pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut stroke_pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut extents: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if _cairo_pattern_is_opaque(fill_source, 0 as *const cairo_rectangle_int_t) == 0
        || _cairo_pattern_is_opaque(stroke_source, 0 as *const cairo_rectangle_int_t)
            == 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if fill_op as libc::c_uint != stroke_op as libc::c_uint {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_composite_rectangles_init_for_stroke(
        &mut extents,
        &mut (*surface).base,
        stroke_op,
        stroke_source,
        path,
        stroke_style,
        stroke_ctm,
        clip,
    );
    if status as u64 != 0 {
        return status;
    }
    if extents.is_bounded != 0 {
        let mut mask: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let mut box_0: cairo_box_t = cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        };
        status = _cairo_path_fixed_stroke_extents(
            path,
            stroke_style,
            stroke_ctm,
            stroke_ctm_inverse,
            stroke_tolerance,
            &mut mask,
        ) as cairo_int_status_t;
        if status as u64 != 0 {
            current_block = 6828276715257822761;
        } else {
            _cairo_box_from_rectangle(&mut box_0, &mut mask);
            status = _cairo_composite_rectangles_intersect_mask_extents(
                &mut extents,
                &mut box_0,
            );
            if status as u64 != 0 {
                current_block = 6828276715257822761;
            } else {
                current_block = 8457315219000651999;
            }
        }
    } else {
        current_block = 8457315219000651999;
    }
    match current_block {
        8457315219000651999 => {
            status = _cairo_pdf_surface_set_clip(surface, &mut extents);
            if !(status as u64 != 0) {
                status = _cairo_pdf_surface_select_operator(surface, fill_op);
                if !(status as u64 != 0) {
                    if extents.is_bounded != 0 {
                        let mut mask_0: cairo_rectangle_int_t = cairo_rectangle_int_t {
                            x: 0,
                            y: 0,
                            width: 0,
                            height: 0,
                        };
                        let mut box_1: cairo_box_t = cairo_box_t {
                            p1: cairo_point_t { x: 0, y: 0 },
                            p2: cairo_point_t { x: 0, y: 0 },
                        };
                        _cairo_path_fixed_fill_extents(
                            path,
                            fill_rule,
                            fill_tolerance,
                            &mut mask_0,
                        );
                        _cairo_box_from_rectangle(&mut box_1, &mut mask_0);
                        status = _cairo_composite_rectangles_intersect_mask_extents(
                            &mut extents,
                            &mut box_1,
                        );
                        if status as u64 != 0 {
                            current_block = 6828276715257822761;
                        } else {
                            current_block = 14648156034262866959;
                        }
                    } else {
                        current_block = 14648156034262866959;
                    }
                    match current_block {
                        6828276715257822761 => {}
                        _ => {
                            status = _cairo_pdf_interchange_add_operation_extents(
                                surface,
                                &mut extents.bounded,
                            );
                            if !(status as u64 != 0) {
                                fill_pattern_res.id = 0 as libc::c_int as libc::c_uint;
                                gstate_res.id = 0 as libc::c_int as libc::c_uint;
                                status = _cairo_pdf_surface_add_pdf_pattern(
                                    surface,
                                    fill_source,
                                    fill_op,
                                    &mut extents.bounded,
                                    &mut fill_pattern_res,
                                    &mut gstate_res,
                                );
                                if !(status as u64 != 0) {
                                    if gstate_res.id == 0 as libc::c_int as libc::c_uint
                                    {} else {
                                        __assert_fail(
                                            b"gstate_res.id == 0\0" as *const u8 as *const libc::c_char,
                                            b"../src/cairo-pdf-surface.c\0" as *const u8
                                                as *const libc::c_char,
                                            8696 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 345],
                                                &[libc::c_char; 345],
                                            >(
                                                b"cairo_int_status_t _cairo_pdf_surface_fill_stroke(void *, cairo_operator_t, const cairo_pattern_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_path_fixed_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    stroke_pattern_res.id = 0 as libc::c_int as libc::c_uint;
                                    gstate_res.id = 0 as libc::c_int as libc::c_uint;
                                    status = _cairo_pdf_surface_add_pdf_pattern(
                                        surface,
                                        stroke_source,
                                        stroke_op,
                                        &mut extents.bounded,
                                        &mut stroke_pattern_res,
                                        &mut gstate_res,
                                    );
                                    if !(status as u64 != 0) {
                                        if gstate_res.id == 0 as libc::c_int as libc::c_uint
                                        {} else {
                                            __assert_fail(
                                                b"gstate_res.id == 0\0" as *const u8 as *const libc::c_char,
                                                b"../src/cairo-pdf-surface.c\0" as *const u8
                                                    as *const libc::c_char,
                                                8709 as libc::c_int as libc::c_uint,
                                                (*::std::mem::transmute::<
                                                    &[u8; 345],
                                                    &[libc::c_char; 345],
                                                >(
                                                    b"cairo_int_status_t _cairo_pdf_surface_fill_stroke(void *, cairo_operator_t, const cairo_pattern_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_path_fixed_t *, cairo_operator_t, const cairo_pattern_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                        status = _cairo_pdf_surface_select_pattern(
                                            surface,
                                            fill_source,
                                            fill_pattern_res,
                                            0 as libc::c_int,
                                        );
                                        if !(status as u64 != 0) {
                                            status = _cairo_pdf_surface_select_pattern(
                                                surface,
                                                stroke_source,
                                                stroke_pattern_res,
                                                1 as libc::c_int,
                                            );
                                            if !(status as u64 != 0) {
                                                status = _cairo_pdf_operators_fill_stroke(
                                                    &mut (*surface).pdf_operators,
                                                    path,
                                                    fill_rule,
                                                    stroke_style,
                                                    stroke_ctm,
                                                    stroke_ctm_inverse,
                                                );
                                                if !(status as u64 != 0) {
                                                    status = _cairo_pdf_surface_unselect_pattern(surface);
                                                    if !(status as u64 != 0) {
                                                        _cairo_composite_rectangles_fini(&mut extents);
                                                        return _cairo_output_stream_get_status((*surface).output)
                                                            as cairo_int_status_t;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_has_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_pdf_surface_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut utf8: *const libc::c_char,
    mut utf8_len: libc::c_int,
    mut glyphs: *mut cairo_glyph_t,
    mut num_glyphs: libc::c_int,
    mut clusters: *const cairo_text_cluster_t,
    mut num_clusters: libc::c_int,
    mut cluster_flags: cairo_text_cluster_flags_t,
    mut scaled_font: *mut cairo_scaled_font_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut current_block: u64;
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut group: *mut cairo_pdf_smask_group_t = 0 as *mut cairo_pdf_smask_group_t;
    let mut pattern_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut gstate_res: cairo_pdf_resource_t = cairo_pdf_resource_t { id: 0 };
    let mut extents: cairo_composite_rectangles_t = cairo_composite_rectangles_t {
        surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        source: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        destination: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        bounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        unbounded: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        is_bounded: 0,
        source_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        mask_sample_area: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        source_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        mask_pattern: cairo_pattern_union_t {
            base: cairo_pattern_t {
                ref_count: cairo_reference_count_t {
                    ref_count: 0,
                },
                status: CAIRO_STATUS_SUCCESS,
                user_data: cairo_user_data_array_t {
                    size: 0,
                    num_elements: 0,
                    element_size: 0,
                    elements: 0 as *mut libc::c_char,
                },
                observers: cairo_list_t {
                    next: 0 as *mut _cairo_list,
                    prev: 0 as *mut _cairo_list,
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut overlap: cairo_bool_t = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_composite_rectangles_init_for_glyphs(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        scaled_font,
        glyphs,
        num_glyphs,
        clip,
        &mut overlap,
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_pdf_interchange_add_operation_extents(surface, &mut extents.bounded);
    if status as u64 != 0 {
        return status;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_pdf_surface_analyze_operation(
            surface,
            op,
            source,
            &mut extents.bounded,
        );
    } else {
        if _cairo_pdf_surface_operation_supported(
            surface,
            op,
            source,
            &mut extents.bounded,
        ) != 0
        {} else {
            __assert_fail(
                b"_cairo_pdf_surface_operation_supported (surface, op, source, &extents.bounded)\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-pdf-surface.c\0" as *const u8 as *const libc::c_char,
                8790 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 255],
                    &[libc::c_char; 255],
                >(
                    b"cairo_int_status_t _cairo_pdf_surface_show_text_glyphs(void *, cairo_operator_t, const cairo_pattern_t *, const char *, int, cairo_glyph_t *, int, const cairo_text_cluster_t *, int, cairo_text_cluster_flags_t, cairo_scaled_font_t *, const cairo_clip_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        status = _cairo_pdf_surface_set_clip(surface, &mut extents);
        if !(status as u64 != 0) {
            pattern_res.id = 0 as libc::c_int as libc::c_uint;
            gstate_res.id = 0 as libc::c_int as libc::c_uint;
            status = _cairo_pdf_surface_add_pdf_pattern(
                surface,
                source,
                op,
                &mut extents.bounded,
                &mut pattern_res,
                &mut gstate_res,
            );
            if !(status as u64 != 0) {
                status = _cairo_pdf_surface_select_operator(surface, op);
                if !(status as u64 != 0) {
                    if gstate_res.id != 0 as libc::c_int as libc::c_uint {
                        group = _cairo_pdf_surface_create_smask_group(
                            surface,
                            &mut extents.bounded,
                        );
                        if group.is_null() {
                            status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                as cairo_int_status_t;
                            current_block = 11760347242806057917;
                        } else {
                            (*group).operation = PDF_SHOW_GLYPHS;
                            status = _cairo_pattern_create_copy(
                                &mut (*group).source,
                                source,
                            ) as cairo_int_status_t;
                            if status as u64 != 0 {
                                _cairo_pdf_smask_group_destroy(group);
                                current_block = 11760347242806057917;
                            } else {
                                (*group).source_res = pattern_res;
                                if utf8_len != 0 {
                                    let ref mut fresh58 = (*group).utf8;
                                    *fresh58 = (if utf8_len != 0 as libc::c_int {
                                        malloc(utf8_len as libc::c_ulong)
                                    } else {
                                        0 as *mut libc::c_void
                                    }) as *mut libc::c_char;
                                    if ((*group).utf8).is_null() {
                                        _cairo_pdf_smask_group_destroy(group);
                                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                            as cairo_int_status_t;
                                        current_block = 11760347242806057917;
                                    } else {
                                        memcpy(
                                            (*group).utf8 as *mut libc::c_void,
                                            utf8 as *const libc::c_void,
                                            utf8_len as libc::c_ulong,
                                        );
                                        current_block = 2873832966593178012;
                                    }
                                } else {
                                    current_block = 2873832966593178012;
                                }
                                match current_block {
                                    11760347242806057917 => {}
                                    _ => {
                                        (*group).utf8_len = utf8_len;
                                        if num_glyphs != 0 {
                                            let ref mut fresh59 = (*group).glyphs;
                                            *fresh59 = _cairo_malloc_ab(
                                                num_glyphs as size_t,
                                                ::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong,
                                            ) as *mut cairo_glyph_t;
                                            if ((*group).glyphs).is_null() {
                                                _cairo_pdf_smask_group_destroy(group);
                                                status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                    as cairo_int_status_t;
                                                current_block = 11760347242806057917;
                                            } else {
                                                memcpy(
                                                    (*group).glyphs as *mut libc::c_void,
                                                    glyphs as *const libc::c_void,
                                                    (::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
                                                        .wrapping_mul(num_glyphs as libc::c_ulong),
                                                );
                                                current_block = 15512526488502093901;
                                            }
                                        } else {
                                            current_block = 15512526488502093901;
                                        }
                                        match current_block {
                                            11760347242806057917 => {}
                                            _ => {
                                                (*group).num_glyphs = num_glyphs;
                                                if num_clusters != 0 {
                                                    let ref mut fresh60 = (*group).clusters;
                                                    *fresh60 = _cairo_malloc_ab(
                                                        num_clusters as size_t,
                                                        ::std::mem::size_of::<cairo_text_cluster_t>()
                                                            as libc::c_ulong,
                                                    ) as *mut cairo_text_cluster_t;
                                                    if ((*group).clusters).is_null() {
                                                        _cairo_pdf_smask_group_destroy(group);
                                                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY)
                                                            as cairo_int_status_t;
                                                        current_block = 11760347242806057917;
                                                    } else {
                                                        memcpy(
                                                            (*group).clusters as *mut libc::c_void,
                                                            clusters as *const libc::c_void,
                                                            (::std::mem::size_of::<cairo_text_cluster_t>()
                                                                as libc::c_ulong)
                                                                .wrapping_mul(num_clusters as libc::c_ulong),
                                                        );
                                                        current_block = 13678349939556791712;
                                                    }
                                                } else {
                                                    current_block = 13678349939556791712;
                                                }
                                                match current_block {
                                                    11760347242806057917 => {}
                                                    _ => {
                                                        (*group).num_clusters = num_clusters;
                                                        let ref mut fresh61 = (*group).scaled_font;
                                                        *fresh61 = cairo_scaled_font_reference(scaled_font);
                                                        status = _cairo_pdf_surface_add_smask_group(surface, group);
                                                        if status as u64 != 0 {
                                                            _cairo_pdf_smask_group_destroy(group);
                                                            current_block = 11760347242806057917;
                                                        } else {
                                                            status = _cairo_pdf_surface_add_smask(surface, gstate_res);
                                                            if status as u64 != 0 {
                                                                current_block = 11760347242806057917;
                                                            } else {
                                                                status = _cairo_pdf_surface_add_xobject(
                                                                    surface,
                                                                    (*group).group_res,
                                                                );
                                                                if status as u64 != 0 {
                                                                    current_block = 11760347242806057917;
                                                                } else {
                                                                    status = _cairo_pdf_operators_flush(
                                                                        &mut (*surface).pdf_operators,
                                                                    ) as cairo_int_status_t;
                                                                    if status as u64 != 0 {
                                                                        current_block = 11760347242806057917;
                                                                    } else {
                                                                        _cairo_output_stream_printf(
                                                                            (*surface).output,
                                                                            b"q /s%d gs /x%d Do Q\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            gstate_res.id,
                                                                            (*group).group_res.id,
                                                                        );
                                                                        current_block = 16029476503615101993;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        status = _cairo_pdf_surface_select_pattern(
                            surface,
                            source,
                            pattern_res,
                            0 as libc::c_int,
                        );
                        if status as u64 != 0 {
                            current_block = 11760347242806057917;
                        } else {
                            if _cairo_pattern_is_opaque(source, &mut extents.bounded)
                                == 0
                            {
                                status = _cairo_pdf_operators_flush(
                                    &mut (*surface).pdf_operators,
                                ) as cairo_int_status_t;
                                if status as u64 != 0 {
                                    current_block = 11760347242806057917;
                                } else {
                                    current_block = 16779030619667747692;
                                }
                            } else {
                                current_block = 16779030619667747692;
                            }
                            match current_block {
                                11760347242806057917 => {}
                                _ => {
                                    status = _cairo_pdf_operators_show_text_glyphs(
                                        &mut (*surface).pdf_operators,
                                        utf8,
                                        utf8_len,
                                        glyphs,
                                        num_glyphs,
                                        clusters,
                                        num_clusters,
                                        cluster_flags,
                                        scaled_font,
                                    );
                                    if status as u64 != 0 {
                                        current_block = 11760347242806057917;
                                    } else {
                                        status = _cairo_pdf_surface_unselect_pattern(surface);
                                        if status as u64 != 0 {
                                            current_block = 11760347242806057917;
                                        } else {
                                            current_block = 16029476503615101993;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        11760347242806057917 => {}
                        _ => {
                            _cairo_composite_rectangles_fini(&mut extents);
                            return _cairo_output_stream_get_status((*surface).output)
                                as cairo_int_status_t;
                        }
                    }
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_get_supported_mime_types(
    mut abstract_surface: *mut libc::c_void,
) -> *mut *const libc::c_char {
    return _cairo_pdf_supported_mime_types.as_mut_ptr();
}
unsafe extern "C" fn _cairo_pdf_surface_tag(
    mut abstract_surface: *mut libc::c_void,
    mut begin: cairo_bool_t,
    mut tag_name: *const libc::c_char,
    mut attributes: *const libc::c_char,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if begin != 0 {
        status = _cairo_pdf_interchange_tag_begin(surface, tag_name, attributes);
    } else {
        status = _cairo_pdf_interchange_tag_end(surface, tag_name);
    }
    return status;
}
unsafe extern "C" fn _cairo_pdf_surface_set_paginated_mode(
    mut abstract_surface: *mut libc::c_void,
    mut paginated_mode: cairo_paginated_mode_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_pdf_surface_t = abstract_surface
        as *mut cairo_pdf_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    (*surface).paginated_mode = paginated_mode;
    status = _cairo_pdf_interchange_begin_page_content(surface);
    if status as u64 != 0 {
        return status;
    }
    if paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        (*surface).surface_extents.x = 0 as libc::c_int;
        (*surface).surface_extents.y = 0 as libc::c_int;
        (*surface).surface_extents.width = ceil((*surface).width) as libc::c_int;
        (*surface).surface_extents.height = ceil((*surface).height) as libc::c_int;
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
static mut cairo_pdf_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_PDF,
            finish: Some(
                _cairo_pdf_surface_finish
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_status_t,
            ),
            create_context: Some(
                _cairo_default_context_create
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut cairo_t,
            ),
            create_similar: None,
            create_similar_image: None,
            map_to_image: None,
            unmap_image: None,
            source: Some(
                _cairo_surface_default_source
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> *mut cairo_surface_t,
            ),
            acquire_source_image: None,
            release_source_image: None,
            snapshot: None,
            copy_page: None,
            show_page: Some(
                _cairo_pdf_surface_show_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            get_extents: Some(
                _cairo_pdf_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_pdf_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_pdf_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_pdf_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_pdf_surface_stroke
                    as unsafe extern "C" fn(
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
            ),
            fill: Some(
                _cairo_pdf_surface_fill
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_path_fixed_t,
                        cairo_fill_rule_t,
                        libc::c_double,
                        cairo_antialias_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            fill_stroke: Some(
                _cairo_pdf_surface_fill_stroke
                    as unsafe extern "C" fn(
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
            ),
            show_glyphs: None,
            has_show_text_glyphs: Some(
                _cairo_pdf_surface_has_show_text_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            show_text_glyphs: Some(
                _cairo_pdf_surface_show_text_glyphs
                    as unsafe extern "C" fn(
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
            ),
            get_supported_mime_types: Some(
                _cairo_pdf_surface_get_supported_mime_types
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                    ) -> *mut *const libc::c_char,
            ),
            tag: Some(
                _cairo_pdf_surface_tag
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_bool_t,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> cairo_int_status_t,
            ),
        };
        init
    }
};
static mut cairo_pdf_surface_paginated_backend: cairo_paginated_surface_backend_t = unsafe {
    {
        let mut init = _cairo_paginated_surface_backend {
            start_page: Some(
                _cairo_pdf_surface_start_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            set_paginated_mode: Some(
                _cairo_pdf_surface_set_paginated_mode
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_paginated_mode_t,
                    ) -> cairo_int_status_t,
            ),
            set_bounding_box: None,
            set_fallback_images_required: Some(
                _cairo_pdf_surface_has_fallback_images
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_bool_t,
                    ) -> cairo_int_status_t,
            ),
            supports_fine_grained_fallbacks: Some(
                _cairo_pdf_surface_supports_fine_grained_fallbacks
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            requires_thumbnail_image: Some(
                _cairo_pdf_surface_requires_thumbnail_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> cairo_bool_t,
            ),
            set_thumbnail_image: Some(
                _cairo_pdf_surface_set_thumbnail_image
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_image_surface_t,
                    ) -> cairo_int_status_t,
            ),
        };
        init
    }
};
