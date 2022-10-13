use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo_backend;
    pub type _cairo_damage;
    pub type _cairo_device;
    pub type _cairo_region;
    pub type cairo_compositor;
    pub type pixman_image;
    pub type _cairo_hash_table;
    pub type _cairo_scaled_font_subsets;
    fn cairo_version_string() -> *const libc::c_char;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn tmpfile() -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn rewind(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
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
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
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
    fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn _cairo_hash_table_foreach(
        hash_table: *mut cairo_hash_table_t,
        hash_callback: cairo_hash_callback_func_t,
        closure: *mut libc::c_void,
    );
    fn _cairo_hash_table_remove(
        hash_table: *mut cairo_hash_table_t,
        key: *mut cairo_hash_entry_t,
    );
    fn _cairo_hash_table_insert(
        hash_table: *mut cairo_hash_table_t,
        entry: *mut cairo_hash_entry_t,
    ) -> cairo_status_t;
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
    fn _cairo_box_from_rectangle(
        box_0: *mut cairo_box_t,
        rectangle: *const cairo_rectangle_int_t,
    );
    fn _cairo_box_round_to_rectangle(
        box_0: *const cairo_box_t,
        rectangle: *mut cairo_rectangle_int_t,
    );
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
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char) -> *mut libc::c_char;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static _cairo_pattern_white: cairo_solid_pattern_t;
    fn _cairo_color_init_rgba(
        color: *mut cairo_color_t,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
        alpha: libc::c_double,
    );
    fn _cairo_color_equal(
        color_a: *const cairo_color_t,
        color_b: *const cairo_color_t,
    ) -> cairo_bool_t;
    fn _cairo_font_options_init_default(options: *mut cairo_font_options_t);
    fn _cairo_font_options_set_round_glyph_positions(
        options: *mut cairo_font_options_t,
        round: cairo_round_glyph_positions_t,
    );
    fn _cairo_lzw_compress(
        data: *mut libc::c_uchar,
        size_in_out: *mut libc::c_ulong,
    ) -> *mut libc::c_uchar;
    fn _cairo_path_fixed_init(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_fini(path: *mut cairo_path_fixed_t);
    fn _cairo_path_fixed_move_to(
        path: *mut cairo_path_fixed_t,
        x: cairo_fixed_t,
        y: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_rel_line_to(
        path: *mut cairo_path_fixed_t,
        dx: cairo_fixed_t,
        dy: cairo_fixed_t,
    ) -> cairo_status_t;
    fn _cairo_path_fixed_close_path(path: *mut cairo_path_fixed_t) -> cairo_status_t;
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
    fn _cairo_pdf_operators_init(
        pdf_operators: *mut cairo_pdf_operators_t,
        stream: *mut cairo_output_stream_t,
        cairo_to_pdf: *mut cairo_matrix_t,
        font_subsets: *mut cairo_scaled_font_subsets_t,
        ps: cairo_bool_t,
    );
    fn _cairo_pdf_operators_set_stream(
        pdf_operators: *mut cairo_pdf_operators_t,
        stream: *mut cairo_output_stream_t,
    );
    fn _cairo_pdf_operators_set_cairo_to_pdf_matrix(
        pdf_operators: *mut cairo_pdf_operators_t,
        cairo_to_pdf: *mut cairo_matrix_t,
    );
    fn _cairo_pdf_operators_flush(
        pdf_operators: *mut cairo_pdf_operators_t,
    ) -> cairo_status_t;
    fn _cairo_pdf_operators_reset(pdf_operators: *mut cairo_pdf_operators_t);
    fn _cairo_pdf_operators_clip(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_stroke(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        style: *const cairo_stroke_style_t,
        ctm: *const cairo_matrix_t,
        ctm_inverse: *const cairo_matrix_t,
    ) -> cairo_int_status_t;
    fn _cairo_pdf_operators_fill(
        pdf_operators: *mut cairo_pdf_operators_t,
        path: *const cairo_path_fixed_t,
        fill_rule: cairo_fill_rule_t,
    ) -> cairo_int_status_t;
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
    fn _cairo_surface_clipper_set_clip(
        clipper: *mut cairo_surface_clipper_t,
        clip: *const cairo_clip_t,
    ) -> cairo_status_t;
    fn _cairo_surface_clipper_reset(clipper: *mut cairo_surface_clipper_t);
    fn _cairo_surface_clipper_init(
        clipper: *mut cairo_surface_clipper_t,
        intersect: cairo_surface_clipper_intersect_clip_path_func_t,
    );
    fn _cairo_pdf_shading_fini(shading: *mut cairo_pdf_shading_t);
    fn _cairo_pdf_shading_init_color(
        shading: *mut cairo_pdf_shading_t,
        pattern: *const cairo_mesh_pattern_t,
    ) -> cairo_status_t;
    fn _cairo_pattern_fini(pattern: *mut cairo_pattern_t);
    fn _cairo_pattern_init_for_surface(
        pattern: *mut cairo_surface_pattern_t,
        surface: *mut cairo_surface_t,
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
    fn _cairo_gradient_pattern_box_to_parameter(
        gradient: *const cairo_gradient_pattern_t,
        x0: libc::c_double,
        y0: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
        tolerance: libc::c_double,
        out_range: *mut libc::c_double,
    );
    fn _cairo_raster_source_pattern_acquire(
        abstract_pattern: *const cairo_pattern_t,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    fn _cairo_raster_source_pattern_release(
        abstract_pattern: *const cairo_pattern_t,
        surface: *mut cairo_surface_t,
    );
    fn _cairo_array_init(array: *mut cairo_array_t, element_size: libc::c_uint);
    fn _cairo_array_fini(array: *mut cairo_array_t);
    fn _cairo_array_truncate(array: *mut cairo_array_t, num_elements: libc::c_uint);
    fn _cairo_array_append(
        array: *mut cairo_array_t,
        element: *const libc::c_void,
    ) -> cairo_status_t;
    fn _cairo_array_index(
        array: *mut cairo_array_t,
        index: libc::c_uint,
    ) -> *mut libc::c_void;
    fn _cairo_array_copy_element(
        array: *const cairo_array_t,
        index: libc::c_uint,
        dst: *mut libc::c_void,
    );
    fn _cairo_array_num_elements(array: *const cairo_array_t) -> libc::c_uint;
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
    fn _cairo_scaled_font_subsets_create_simple() -> *mut cairo_scaled_font_subsets_t;
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
    fn _cairo_scaled_font_subset_create_glyph_names(
        subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_int_status_t;
    fn _cairo_truetype_subset_init_ps(
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
    fn _cairo_type1_fallback_init_hex(
        type_subset: *mut cairo_type1_subset_t,
        name: *const libc::c_char,
        font_subset: *mut cairo_scaled_font_subset_t,
    ) -> cairo_status_t;
    fn _cairo_type1_fallback_fini(subset: *mut cairo_type1_subset_t);
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
    fn _cairo_recording_surface_replay_region(
        surface: *mut cairo_surface_t,
        surface_extents: *const cairo_rectangle_int_t,
        target: *mut cairo_surface_t,
        region: cairo_recording_region_type_t,
    ) -> cairo_status_t;
    static _cairo_output_stream_nil: cairo_output_stream_t;
    fn _cairo_output_stream_init(
        stream: *mut cairo_output_stream_t,
        write_func: cairo_output_stream_write_func_t,
        flush_func: cairo_output_stream_flush_func_t,
        close_func: cairo_output_stream_close_func_t,
    );
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
    fn _cairo_output_stream_write_hex_string(
        stream: *mut cairo_output_stream_t,
        data: *const libc::c_uchar,
        length: size_t,
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
    fn _cairo_output_stream_get_status(
        stream: *mut cairo_output_stream_t,
    ) -> cairo_status_t;
    fn _cairo_output_stream_create_for_filename(
        filename: *const libc::c_char,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_output_stream_create_for_file(
        file: *mut FILE,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_create() -> *mut cairo_output_stream_t;
    fn _cairo_memory_stream_destroy(
        abstract_stream: *mut cairo_output_stream_t,
        data_out: *mut *mut libc::c_uchar,
        length_out: *mut libc::c_ulong,
    ) -> cairo_status_t;
    fn _cairo_base85_stream_create(
        output: *mut cairo_output_stream_t,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_deflate_stream_create(
        output: *mut cairo_output_stream_t,
    ) -> *mut cairo_output_stream_t;
    fn _cairo_type3_glyph_surface_create(
        scaled_font: *mut cairo_scaled_font_t,
        stream: *mut cairo_output_stream_t,
        emit_image: cairo_type3_glyph_surface_emit_image_t,
        font_subsets: *mut cairo_scaled_font_subsets_t,
        ps_output: cairo_bool_t,
    ) -> *mut cairo_surface_t;
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
    fn _cairo_image_info_get_jpeg_info(
        info: *mut cairo_image_info_t,
        data: *const libc::c_uchar,
        length: libc::c_ulong,
    ) -> cairo_int_status_t;
    fn _cairo_tag_parse_ccitt_params(
        attributes: *const libc::c_char,
        dest_attrs: *mut cairo_ccitt_params_t,
    ) -> cairo_int_status_t;
    fn _cairo_tag_parse_eps_params(
        attributes: *const libc::c_char,
        dest_attrs: *mut cairo_eps_params_t,
    ) -> cairo_int_status_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_point_int {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type cairo_point_int_t = _cairo_point_int;
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
pub type cairo_hash_keys_equal_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cairo_bool_t,
>;
pub type cairo_hash_callback_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type _cairo_ps_level = libc::c_uint;
pub const CAIRO_PS_LEVEL_3: _cairo_ps_level = 1;
pub const CAIRO_PS_LEVEL_2: _cairo_ps_level = 0;
pub type cairo_ps_level_t = _cairo_ps_level;
pub type cairo_ps_surface_t = cairo_ps_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_ps_surface {
    pub base: cairo_surface_t,
    pub final_stream: *mut cairo_output_stream_t,
    pub tmpfile: *mut FILE,
    pub stream: *mut cairo_output_stream_t,
    pub eps: cairo_bool_t,
    pub contains_eps: cairo_bool_t,
    pub content: cairo_content_t,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub document_bbox_p1: cairo_point_int_t,
    pub document_bbox_p2: cairo_point_int_t,
    pub surface_extents: cairo_rectangle_int_t,
    pub surface_bounded: cairo_bool_t,
    pub cairo_to_ps: cairo_matrix_t,
    pub paint_proc: cairo_bool_t,
    pub current_pattern_is_solid_color: cairo_bool_t,
    pub current_color: cairo_color_t,
    pub num_pages: libc::c_int,
    pub paginated_mode: cairo_paginated_mode_t,
    pub force_fallbacks: cairo_bool_t,
    pub has_creation_date: cairo_bool_t,
    pub creation_date: time_t,
    pub font_subsets: *mut cairo_scaled_font_subsets_t,
    pub document_media: cairo_list_t,
    pub dsc_header_comments: cairo_array_t,
    pub dsc_setup_comments: cairo_array_t,
    pub dsc_page_setup_comments: cairo_array_t,
    pub recording_surf_stack: cairo_array_t,
    pub dsc_comment_target: *mut cairo_array_t,
    pub ps_level: cairo_ps_level_t,
    pub ps_level_used: cairo_ps_level_t,
    pub clipper: cairo_surface_clipper_t,
    pub pdf_operators: cairo_pdf_operators_t,
    pub paginated_surface: *mut cairo_surface_t,
    pub forms: *mut cairo_hash_table_t,
    pub num_forms: libc::c_int,
    pub total_form_size: libc::c_long,
}
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
pub type cairo_page_media_t = _cairo_page_media;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_page_media {
    pub name: *mut libc::c_char,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub link: cairo_list_t,
}
pub type cairo_page_standard_media_t = _cairo_page_standard_media;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_page_standard_media {
    pub name: *const libc::c_char,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type cairo_ps_form_t = _cairo_ps_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ps_form {
    pub base: cairo_hash_entry_t,
    pub unique_id: *mut libc::c_uchar,
    pub unique_id_length: libc::c_ulong,
    pub is_image: cairo_bool_t,
    pub id: libc::c_int,
    pub src_surface: *mut cairo_surface_t,
    pub src_surface_extents: cairo_rectangle_int_t,
    pub src_surface_bounded: cairo_bool_t,
    pub filter: cairo_filter_t,
    pub required_extents: cairo_rectangle_int_t,
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
pub type cairo_ps_compress_t = libc::c_uint;
pub const CAIRO_PS_COMPRESS_DEFLATE: cairo_ps_compress_t = 2;
pub const CAIRO_PS_COMPRESS_LZW: cairo_ps_compress_t = 1;
pub const CAIRO_PS_COMPRESS_NONE: cairo_ps_compress_t = 0;
pub type string_array_stream_t = _string_array_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _string_array_stream {
    pub base: cairo_output_stream_t,
    pub output: *mut cairo_output_stream_t,
    pub column: libc::c_int,
    pub string_size: libc::c_int,
    pub tuple_count: libc::c_int,
    pub use_strings: cairo_bool_t,
}
pub type cairo_ps_color_stop_t = _cairo_ps_color_stop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_ps_color_stop {
    pub offset: libc::c_double,
    pub color: [libc::c_double; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cairo_emit_surface_params_t {
    pub src_surface: *mut cairo_surface_t,
    pub op: cairo_operator_t,
    pub src_surface_extents: *const cairo_rectangle_int_t,
    pub src_surface_bounded: cairo_bool_t,
    pub src_op_extents: *const cairo_rectangle_int_t,
    pub filter: cairo_filter_t,
    pub stencil_mask: cairo_bool_t,
    pub is_image: cairo_bool_t,
    pub approx_size: libc::c_long,
    pub eod_count: libc::c_int,
}
pub type cairo_emit_surface_mode_t = libc::c_uint;
pub const CAIRO_EMIT_SURFACE_EMIT_FORM: cairo_emit_surface_mode_t = 2;
pub const CAIRO_EMIT_SURFACE_EMIT: cairo_emit_surface_mode_t = 1;
pub const CAIRO_EMIT_SURFACE_ANALYZE: cairo_emit_surface_mode_t = 0;
pub type cairo_recording_region_type_t = libc::c_uint;
pub const CAIRO_RECORDING_REGION_IMAGE_FALLBACK: cairo_recording_region_type_t = 2;
pub const CAIRO_RECORDING_REGION_NATIVE: cairo_recording_region_type_t = 1;
pub const CAIRO_RECORDING_REGION_ALL: cairo_recording_region_type_t = 0;
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
pub type cairo_eps_params_t = _cairo_eps_params;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_eps_params {
    pub bbox: cairo_box_double_t,
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
#[inline(always)]
unsafe extern "C" fn _cairo_atomic_int_get(
    mut x: *mut cairo_atomic_int_t,
) -> cairo_atomic_int_t {
    return ::std::intrinsics::atomic_load(x);
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
unsafe extern "C" fn _cairo_matrix_is_identity(
    mut matrix: *const cairo_matrix_t,
) -> cairo_bool_t {
    return ((*matrix).xx == 1.0f64 && (*matrix).yx == 0.0f64 && (*matrix).xy == 0.0f64
        && (*matrix).yy == 1.0f64 && (*matrix).x0 == 0.0f64 && (*matrix).y0 == 0.0f64)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn _cairo_fixed_from_int(mut i: libc::c_int) -> cairo_fixed_t {
    return i << 8 as libc::c_int;
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
    let ref mut fresh8 = (*next).prev;
    *fresh8 = prev;
    let ref mut fresh9 = (*prev).next;
    *fresh9 = next;
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
static mut _cairo_ps_levels: [cairo_ps_level_t; 2] = [
    CAIRO_PS_LEVEL_2,
    CAIRO_PS_LEVEL_3,
];
static mut _cairo_ps_level_strings: [*const libc::c_char; 2] = [
    b"PS Level 2\0" as *const u8 as *const libc::c_char,
    b"PS Level 3\0" as *const u8 as *const libc::c_char,
];
static mut _cairo_ps_supported_mime_types: [*const libc::c_char; 4] = [
    b"image/jpeg\0" as *const u8 as *const libc::c_char,
    b"image/g3fax\0" as *const u8 as *const libc::c_char,
    b"application/x-cairo.ccitt.params\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut _cairo_page_standard_media: [cairo_page_standard_media_t; 17] = [
    {
        let mut init = _cairo_page_standard_media {
            name: b"A0\0" as *const u8 as *const libc::c_char,
            width: 2384 as libc::c_int,
            height: 3371 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"A1\0" as *const u8 as *const libc::c_char,
            width: 1685 as libc::c_int,
            height: 2384 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"A2\0" as *const u8 as *const libc::c_char,
            width: 1190 as libc::c_int,
            height: 1684 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"A3\0" as *const u8 as *const libc::c_char,
            width: 842 as libc::c_int,
            height: 1190 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"A4\0" as *const u8 as *const libc::c_char,
            width: 595 as libc::c_int,
            height: 842 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"A5\0" as *const u8 as *const libc::c_char,
            width: 420 as libc::c_int,
            height: 595 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"B4\0" as *const u8 as *const libc::c_char,
            width: 729 as libc::c_int,
            height: 1032 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"B5\0" as *const u8 as *const libc::c_char,
            width: 516 as libc::c_int,
            height: 729 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Letter\0" as *const u8 as *const libc::c_char,
            width: 612 as libc::c_int,
            height: 792 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Tabloid\0" as *const u8 as *const libc::c_char,
            width: 792 as libc::c_int,
            height: 1224 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Ledger\0" as *const u8 as *const libc::c_char,
            width: 1224 as libc::c_int,
            height: 792 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Legal\0" as *const u8 as *const libc::c_char,
            width: 612 as libc::c_int,
            height: 1008 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Statement\0" as *const u8 as *const libc::c_char,
            width: 396 as libc::c_int,
            height: 612 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Executive\0" as *const u8 as *const libc::c_char,
            width: 540 as libc::c_int,
            height: 720 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Folio\0" as *const u8 as *const libc::c_char,
            width: 612 as libc::c_int,
            height: 936 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"Quarto\0" as *const u8 as *const libc::c_char,
            width: 610 as libc::c_int,
            height: 780 as libc::c_int,
        };
        init
    },
    {
        let mut init = _cairo_page_standard_media {
            name: b"10x14\0" as *const u8 as *const libc::c_char,
            width: 720 as libc::c_int,
            height: 1008 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn _cairo_ps_form_init_key(mut key: *mut cairo_ps_form_t) {
    (*key)
        .base
        .hash = _cairo_hash_bytes(
        5381 as libc::c_int as uintptr_t,
        (*key).unique_id as *const libc::c_void,
        (*key).unique_id_length as libc::c_uint,
    );
}
unsafe extern "C" fn _cairo_ps_form_equal(
    mut key_a: *const libc::c_void,
    mut key_b: *const libc::c_void,
) -> cairo_bool_t {
    let mut a: *const cairo_ps_form_t = key_a as *const cairo_ps_form_t;
    let mut b: *const cairo_ps_form_t = key_b as *const cairo_ps_form_t;
    if (*a).filter as libc::c_uint != (*b).filter as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*a).unique_id_length != (*b).unique_id_length {
        return 0 as libc::c_int;
    }
    return (memcmp(
        (*a).unique_id as *const libc::c_void,
        (*b).unique_id as *const libc::c_void,
        (*a).unique_id_length,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn _cairo_ps_form_pluck(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut surface_entry: *mut cairo_ps_form_t = entry as *mut cairo_ps_form_t;
    let mut patterns: *mut cairo_hash_table_t = closure as *mut cairo_hash_table_t;
    _cairo_hash_table_remove(patterns, &mut (*surface_entry).base);
    free((*surface_entry).unique_id as *mut libc::c_void);
    cairo_surface_destroy((*surface_entry).src_surface);
    free(surface_entry as *mut libc::c_void);
}
unsafe extern "C" fn _cairo_ps_surface_emit_header(
    mut surface: *mut cairo_ps_surface_t,
) {
    let mut ctime_buf: [libc::c_char; 26] = [0; 26];
    let mut now: time_t = 0;
    let mut comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut num_comments: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut eps_header: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut has_bbox: cairo_bool_t = 0;
    if (*surface).has_creation_date != 0 {
        now = (*surface).creation_date;
    } else {
        now = time(0 as *mut time_t);
    }
    if (*surface).ps_level_used as libc::c_uint
        == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
    {
        level = 2 as libc::c_int;
    } else {
        level = 3 as libc::c_int;
    }
    if (*surface).eps != 0 {
        eps_header = b" EPSF-3.0\0" as *const u8 as *const libc::c_char;
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%!PS-Adobe-3.0%s\n%%%%Creator: cairo %s (https://cairographics.org)\n\0"
            as *const u8 as *const libc::c_char,
        eps_header,
        cairo_version_string(),
    );
    if (getenv(b"CAIRO_DEBUG_PS_NO_DATE\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"%%%%CreationDate: %s\0" as *const u8 as *const libc::c_char,
            ctime_r(&mut now, ctime_buf.as_mut_ptr()),
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%Pages: %d\n%%%%DocumentData: Clean7Bit\n%%%%LanguageLevel: %d\n\0"
            as *const u8 as *const libc::c_char,
        (*surface).num_pages,
        level,
    );
    if cairo_list_is_empty(&mut (*surface).document_media) == 0 {
        let mut page: *mut cairo_page_media_t = 0 as *mut cairo_page_media_t;
        let mut first: cairo_bool_t = 1 as libc::c_int;
        page = ({
            let mut mptr__: *const cairo_list_t = (*surface).document_media.next;
            (mptr__ as *mut libc::c_char).offset(-(16 as libc::c_ulong as isize))
                as *mut cairo_page_media_t
        });
        while &mut (*page).link as *mut cairo_list_t
            != &mut (*surface).document_media as *mut cairo_list_t
        {
            if first != 0 {
                _cairo_output_stream_printf(
                    (*surface).final_stream,
                    b"%%%%DocumentMedia: \0" as *const u8 as *const libc::c_char,
                );
                first = 0 as libc::c_int;
            } else {
                _cairo_output_stream_printf(
                    (*surface).final_stream,
                    b"%%%%+ \0" as *const u8 as *const libc::c_char,
                );
            }
            _cairo_output_stream_printf(
                (*surface).final_stream,
                b"%s %d %d 0 () ()\n\0" as *const u8 as *const libc::c_char,
                (*page).name,
                (*page).width,
                (*page).height,
            );
            page = ({
                let mut mptr__: *const cairo_list_t = (*page).link.next;
                (mptr__ as *mut libc::c_char).offset(-(16 as libc::c_ulong as isize))
                    as *mut cairo_page_media_t
            });
        }
    }
    has_bbox = 0 as libc::c_int;
    num_comments = _cairo_array_num_elements(&mut (*surface).dsc_header_comments)
        as libc::c_int;
    comments = _cairo_array_index(
        &mut (*surface).dsc_header_comments,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < num_comments {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            *comments.offset(i as isize),
        );
        if strncmp(
            *comments.offset(i as isize),
            b"%%BoundingBox:\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            has_bbox = 1 as libc::c_int;
        }
        free(*comments.offset(i as isize) as *mut libc::c_void);
        let ref mut fresh10 = *comments.offset(i as isize);
        *fresh10 = 0 as *mut libc::c_char;
        i += 1;
    }
    if has_bbox == 0 {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"%%%%BoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
            (*surface).document_bbox_p1.x,
            (*surface).document_bbox_p1.y,
            (*surface).document_bbox_p2.x,
            (*surface).document_bbox_p2.y,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndComments\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%BeginProlog\n\0" as *const u8 as *const libc::c_char,
    );
    if (*surface).eps != 0 {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"50 dict begin\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"/languagelevel where\n{ pop languagelevel } { 1 } ifelse\n%d lt { /Helvetica findfont 12 scalefont setfont 50 500 moveto\n  (This print job requires a PostScript Language Level %d printer.) show\n  showpage quit } if\n\0"
                as *const u8 as *const libc::c_char,
            level,
            level,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"/q { gsave } bind def\n/Q { grestore } bind def\n/cm { 6 array astore concat } bind def\n/w { setlinewidth } bind def\n/J { setlinecap } bind def\n/j { setlinejoin } bind def\n/M { setmiterlimit } bind def\n/d { setdash } bind def\n/m { moveto } bind def\n/l { lineto } bind def\n/c { curveto } bind def\n/h { closepath } bind def\n/re { exch dup neg 3 1 roll 5 3 roll moveto 0 rlineto\n      0 exch rlineto 0 rlineto closepath } bind def\n/S { stroke } bind def\n/f { fill } bind def\n/f* { eofill } bind def\n/n { newpath } bind def\n/W { clip } bind def\n/W* { eoclip } bind def\n/BT { } bind def\n/ET { } bind def\n/BDC { mark 3 1 roll /BDC pdfmark } bind def\n/EMC { mark /EMC pdfmark } bind def\n/cairo_store_point { /cairo_point_y exch def /cairo_point_x exch def } def\n/Tj { show currentpoint cairo_store_point } bind def\n/TJ {\n  {\n    dup\n    type /stringtype eq\n    { show } { -0.001 mul 0 cairo_font_matrix dtransform rmoveto } ifelse\n  } forall\n  currentpoint cairo_store_point\n} bind def\n/cairo_selectfont { cairo_font_matrix aload pop pop pop 0 0 6 array astore\n    cairo_font exch selectfont cairo_point_x cairo_point_y moveto } bind def\n/Tf { pop /cairo_font exch def /cairo_font_matrix where\n      { pop cairo_selectfont } if } bind def\n/Td { matrix translate cairo_font_matrix matrix concatmatrix dup\n      /cairo_font_matrix exch def dup 4 get exch 5 get cairo_store_point\n      /cairo_font where { pop cairo_selectfont } if } bind def\n/Tm { 2 copy 8 2 roll 6 array astore /cairo_font_matrix exch def\n      cairo_store_point /cairo_font where { pop cairo_selectfont } if } bind def\n/g { setgray } bind def\n/rg { setrgbcolor } bind def\n/d1 { setcachedevice } bind def\n/cairo_data_source {\n  CairoDataIndex CairoData length lt\n    { CairoData CairoDataIndex get /CairoDataIndex CairoDataIndex 1 add def }\n    { () } ifelse\n} def\n/cairo_flush_ascii85_file { cairo_ascii85_file status { cairo_ascii85_file flushfile } if } def\n/cairo_image { image cairo_flush_ascii85_file } def\n/cairo_imagemask { imagemask cairo_flush_ascii85_file } def\n\0"
            as *const u8 as *const libc::c_char,
    );
    if (*surface).eps == 0 {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"/cairo_set_page_size {\n  %% Change paper size, but only if different from previous paper size otherwise\n  %% duplex fails. PLRM specifies a tolerance of 5 pts when matching paper size\n  %% so we use the same when checking if the size changes.\n  /setpagedevice where {\n    pop currentpagedevice\n    /PageSize known {\n      2 copy\n      currentpagedevice /PageSize get aload pop\n      exch 4 1 roll\n      sub abs 5 gt\n      3 1 roll\n      sub abs 5 gt\n      or\n    } {\n      true\n    } ifelse\n    {\n      2 array astore\n      2 dict begin\n        /PageSize exch def\n        /ImagingBBox null def\n      currentdict end\n      setpagedevice\n    } {\n      pop pop\n    } ifelse\n  } {\n    pop\n  } ifelse\n} def\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if (*surface).contains_eps != 0 {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"/cairo_eps_begin {\n  /cairo_save_state save def\n  /dict_count countdictstack def\n  /op_count count 1 sub def\n  userdict begin\n  /showpage { } def\n  0 g 0 J 1 w 0 j 10 M [ ] 0 d n\n} bind def\n/cairo_eps_end {\n  count op_count sub { pop } repeat\n  countdictstack dict_count sub { end } repeat\n  cairo_save_state restore\n} bind def\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndProlog\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn _cairo_ps_surface_emit_type1_font_subset(
    mut surface: *mut cairo_ps_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut length: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"f-%d-%d\0" as *const u8 as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    status = _cairo_type1_subset_init(
        &mut subset,
        name.as_mut_ptr(),
        font_subset,
        1 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%BeginResource: font %s\n\0" as *const u8 as *const libc::c_char,
        subset.base_font,
    );
    length = (subset.header_length)
        .wrapping_add(subset.data_length)
        .wrapping_add(subset.trailer_length) as libc::c_int;
    _cairo_output_stream_write(
        (*surface).final_stream,
        subset.data as *const libc::c_void,
        length as size_t,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_type1_subset_fini(&mut subset);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_type1_font_fallback(
    mut surface: *mut cairo_ps_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut length: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    snprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"f-%d-%d\0" as *const u8 as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    status = _cairo_type1_fallback_init_hex(&mut subset, name.as_mut_ptr(), font_subset);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%BeginResource: font %s\n\0" as *const u8 as *const libc::c_char,
        subset.base_font,
    );
    length = (subset.header_length)
        .wrapping_add(subset.data_length)
        .wrapping_add(subset.trailer_length) as libc::c_int;
    _cairo_output_stream_write(
        (*surface).final_stream,
        subset.data as *const libc::c_void,
        length as size_t,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_type1_fallback_fini(&mut subset);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_truetype_font_subset(
    mut surface: *mut cairo_ps_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut begin: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    status = _cairo_truetype_subset_init_ps(&mut subset, font_subset);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%BeginResource: font %s\n\0" as *const u8 as *const libc::c_char,
        subset.ps_name,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"11 dict begin\n/FontType 42 def\n/FontName /%s def\n/PaintType 0 def\n/FontMatrix [ 1 0 0 1 0 0 ] def\n/FontBBox [ 0 0 0 0 ] def\n/Encoding 256 array def\n0 1 255 { Encoding exch /.notdef put } for\n\0"
            as *const u8 as *const libc::c_char,
        subset.ps_name,
    );
    if (*font_subset).is_latin != 0 {
        i = 1 as libc::c_int as libc::c_uint;
        while i < 256 as libc::c_int as libc::c_uint {
            if *((*font_subset).latin_to_subset_glyph_index).offset(i as isize)
                > 0 as libc::c_int as libc::c_ulong
            {
                if !((*font_subset).glyph_names).is_null() {
                    _cairo_output_stream_printf(
                        (*surface).final_stream,
                        b"Encoding %d /%s put\n\0" as *const u8 as *const libc::c_char,
                        i,
                        *((*font_subset).glyph_names)
                            .offset(
                                *((*font_subset).latin_to_subset_glyph_index)
                                    .offset(i as isize) as isize,
                            ),
                    );
                } else {
                    _cairo_output_stream_printf(
                        (*surface).final_stream,
                        b"Encoding %d /g%ld put\n\0" as *const u8 as *const libc::c_char,
                        i,
                        *((*font_subset).latin_to_subset_glyph_index).offset(i as isize),
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 1 as libc::c_int as libc::c_uint;
        while i < (*font_subset).num_glyphs {
            if !((*font_subset).glyph_names).is_null() {
                _cairo_output_stream_printf(
                    (*surface).final_stream,
                    b"Encoding %d /%s put\n\0" as *const u8 as *const libc::c_char,
                    i,
                    *((*font_subset).glyph_names).offset(i as isize),
                );
            } else {
                _cairo_output_stream_printf(
                    (*surface).final_stream,
                    b"Encoding %d /g%d put\n\0" as *const u8 as *const libc::c_char,
                    i,
                    i,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"/CharStrings %d dict dup begin\n/.notdef 0 def\n\0" as *const u8
            as *const libc::c_char,
        (*font_subset).num_glyphs,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        if !((*font_subset).glyph_names).is_null() {
            _cairo_output_stream_printf(
                (*surface).final_stream,
                b"/%s %d def\n\0" as *const u8 as *const libc::c_char,
                *((*font_subset).glyph_names).offset(i as isize),
                i,
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).final_stream,
                b"/g%d %d def\n\0" as *const u8 as *const libc::c_char,
                i,
                i,
            );
        }
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"end readonly def\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"/sfnts [\n\0" as *const u8 as *const libc::c_char,
    );
    begin = 0 as libc::c_int as libc::c_uint;
    end = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < subset.num_string_offsets {
        end = *(subset.string_offsets).offset(i as isize) as libc::c_uint;
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"<\0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_write_hex_string(
            (*surface).final_stream,
            (subset.data).offset(begin as isize),
            end.wrapping_sub(begin) as size_t,
        );
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"00>\n\0" as *const u8 as *const libc::c_char,
        );
        begin = end;
        i = i.wrapping_add(1);
    }
    if subset.data_length > end as libc::c_ulong {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"<\0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_write_hex_string(
            (*surface).final_stream,
            (subset.data).offset(end as isize),
            (subset.data_length).wrapping_sub(end as libc::c_ulong),
        );
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"00>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"] def\n/f-%d-%d currentdict end definefont pop\n\0" as *const u8
            as *const libc::c_char,
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_truetype_subset_fini(&mut subset);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_emit_imagemask(
    mut image: *mut cairo_image_surface_t,
    mut stream: *mut cairo_output_stream_t,
) -> cairo_int_status_t {
    let mut row: *mut uint8_t = 0 as *mut uint8_t;
    let mut byte: *mut uint8_t = 0 as *mut uint8_t;
    let mut rows: libc::c_int = 0;
    let mut cols: libc::c_int = 0;
    if (*image).format as libc::c_int == CAIRO_FORMAT_A1 as libc::c_int {} else {
        __assert_fail(
            b"image->format == CAIRO_FORMAT_A1\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            704 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"cairo_int_status_t _cairo_ps_emit_imagemask(cairo_image_surface_t *, cairo_output_stream_t *)\0",
            ))
                .as_ptr(),
        );
    }
    _cairo_output_stream_printf(
        stream,
        b"<<\n   /ImageType 1\n   /Width %d\n   /Height %d\n   /ImageMatrix [%d 0 0 %d 0 %d]\n   /Decode [1 0]\n   /BitsPerComponent 1\n\0"
            as *const u8 as *const libc::c_char,
        (*image).width,
        (*image).height,
        (*image).width,
        -(*image).height,
        (*image).height,
    );
    _cairo_output_stream_printf(
        stream,
        b"   /DataSource {<\n   \0" as *const u8 as *const libc::c_char,
    );
    row = (*image).data;
    rows = (*image).height;
    while rows != 0 {
        byte = row;
        cols = ((*image).width + 7 as libc::c_int) / 8 as libc::c_int;
        while cols != 0 {
            let mut output_byte: uint8_t = (((*byte as libc::c_ulong)
                .wrapping_mul(0x802 as libc::c_ulong) & 0x22110 as libc::c_ulong
                | (*byte as libc::c_ulong).wrapping_mul(0x8020 as libc::c_ulong)
                    & 0x88440 as libc::c_ulong)
                .wrapping_mul(0x10101 as libc::c_ulong) >> 16 as libc::c_int) as uint8_t;
            _cairo_output_stream_printf(
                stream,
                b"%02x \0" as *const u8 as *const libc::c_char,
                output_byte as libc::c_int,
            );
            byte = byte.offset(1);
            cols -= 1;
        }
        _cairo_output_stream_printf(
            stream,
            b"\n   \0" as *const u8 as *const libc::c_char,
        );
        row = row.offset((*image).stride as isize);
        rows -= 1;
    }
    _cairo_output_stream_printf(
        stream,
        b">}\n>>\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        stream,
        b"imagemask\n\0" as *const u8 as *const libc::c_char,
    );
    return _cairo_output_stream_get_status(stream) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_analyze_user_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = closure as *mut cairo_ps_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut i: libc::c_uint = 0;
    let mut type3_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    type3_surface = _cairo_type3_glyph_surface_create(
        (*font_subset).scaled_font,
        0 as *mut cairo_output_stream_t,
        Some(
            _cairo_ps_emit_imagemask
                as unsafe extern "C" fn(
                    *mut cairo_image_surface_t,
                    *mut cairo_output_stream_t,
                ) -> cairo_int_status_t,
        ),
        (*surface).font_subsets,
        1 as libc::c_int,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        status = _cairo_type3_glyph_surface_analyze_glyph(
            type3_surface as *mut libc::c_void,
            *((*font_subset).glyphs).offset(i as isize),
        );
        if status as u64 != 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    cairo_surface_finish(type3_surface);
    cairo_surface_destroy(type3_surface);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_emit_type3_font_subset(
    mut surface: *mut cairo_ps_surface_t,
    mut font_subset: *mut cairo_scaled_font_subset_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
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
    let mut width: libc::c_double = 0.;
    if (*font_subset).num_glyphs == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%BeginResource: font\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"8 dict begin\n/FontType 3 def\n/FontMatrix [1 0 0 -1 0 0] def\n/Encoding 256 array def\n0 1 255 { Encoding exch /.notdef put } for\n\0"
            as *const u8 as *const libc::c_char,
    );
    type3_surface = _cairo_type3_glyph_surface_create(
        (*font_subset).scaled_font,
        0 as *mut cairo_output_stream_t,
        Some(
            _cairo_ps_emit_imagemask
                as unsafe extern "C" fn(
                    *mut cairo_image_surface_t,
                    *mut cairo_output_stream_t,
                ) -> cairo_int_status_t,
        ),
        (*surface).font_subsets,
        1 as libc::c_int,
    );
    status = (*type3_surface).status;
    if status as u64 != 0 {
        return status;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        if !((*font_subset).glyph_names).is_null() {
            _cairo_output_stream_printf(
                (*surface).final_stream,
                b"Encoding %d /%s put\n\0" as *const u8 as *const libc::c_char,
                i,
                *((*font_subset).glyph_names).offset(i as isize),
            );
        } else {
            _cairo_output_stream_printf(
                (*surface).final_stream,
                b"Encoding %d /g%d put\n\0" as *const u8 as *const libc::c_char,
                i,
                i,
            );
        }
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"/Glyphs [\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*font_subset).num_glyphs {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"    { %% %d\n\0" as *const u8 as *const libc::c_char,
            i,
        );
        status = _cairo_type3_glyph_surface_emit_glyph(
            type3_surface as *mut libc::c_void,
            (*surface).final_stream,
            *((*font_subset).glyphs).offset(i as isize),
            &mut bbox,
            &mut width,
        );
        if status as u64 != 0 {
            break;
        }
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"    }\n\0" as *const u8 as *const libc::c_char,
        );
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
    cairo_surface_finish(type3_surface);
    cairo_surface_destroy(type3_surface);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"] def\n/FontBBox [%f %f %f %f] def\n/BuildChar {\n  exch /Glyphs get\n  exch get\n  10 dict begin exec end\n} bind def\ncurrentdict\nend\n/f-%d-%d exch definefont pop\n\0"
            as *const u8 as *const libc::c_char,
        _cairo_fixed_to_double(font_bbox.p1.x),
        -_cairo_fixed_to_double(font_bbox.p2.y),
        _cairo_fixed_to_double(font_bbox.p2.x),
        -_cairo_fixed_to_double(font_bbox.p1.y),
        (*font_subset).font_id,
        (*font_subset).subset_id,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_unscaled_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = closure as *mut cairo_ps_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_scaled_font_subset_create_glyph_names(font_subset);
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_ps_surface_emit_type1_font_subset(surface, font_subset)
        as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_ps_surface_emit_truetype_font_subset(surface, font_subset)
        as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_ps_surface_emit_type1_font_fallback(surface, font_subset)
        as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            898 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"cairo_int_status_t _cairo_ps_surface_emit_unscaled_font_subset(cairo_scaled_font_subset_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_emit_scaled_font_subset(
    mut font_subset: *mut cairo_scaled_font_subset_t,
    mut closure: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = closure as *mut cairo_ps_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    status = _cairo_scaled_font_subset_create_glyph_names(font_subset);
    if status as libc::c_uint != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        && (status as libc::c_uint)
            < CAIRO_INT_STATUS_LAST_STATUS as libc::c_int as libc::c_uint
    {
        return status;
    }
    status = _cairo_ps_surface_emit_type3_font_subset(surface, font_subset)
        as cairo_int_status_t;
    if status as libc::c_uint
        != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        return status;
    }
    if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            917 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"cairo_int_status_t _cairo_ps_surface_emit_scaled_font_subset(cairo_scaled_font_subset_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_INT_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_font_subsets(
    mut surface: *mut cairo_ps_surface_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_scaled_font_subsets_foreach_user(
        (*surface).font_subsets,
        Some(
            _cairo_ps_surface_analyze_user_font_subset
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_subset_t,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_scaled_font_subsets_foreach_unscaled(
        (*surface).font_subsets,
        Some(
            _cairo_ps_surface_emit_unscaled_font_subset
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_subset_t,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_scaled_font_subsets_foreach_scaled(
        (*surface).font_subsets,
        Some(
            _cairo_ps_surface_emit_scaled_font_subset
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_subset_t,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    );
    if status as u64 != 0 {
        return status;
    }
    return _cairo_scaled_font_subsets_foreach_user(
        (*surface).font_subsets,
        Some(
            _cairo_ps_surface_emit_scaled_font_subset
                as unsafe extern "C" fn(
                    *mut cairo_scaled_font_subset_t,
                    *mut libc::c_void,
                ) -> cairo_int_status_t,
        ),
        surface as *mut libc::c_void,
    );
}
unsafe extern "C" fn _cairo_ps_surface_emit_forms(
    mut surface: *mut cairo_ps_surface_t,
) -> cairo_int_status_t {
    _cairo_hash_table_foreach(
        (*surface).forms,
        Some(
            _cairo_ps_form_emit
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        surface as *mut libc::c_void,
    );
    return (*surface).base.status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_emit_body(
    mut surface: *mut cairo_ps_surface_t,
) -> cairo_status_t {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut n: libc::c_int = 0;
    if ferror((*surface).tmpfile) != 0 as libc::c_int {
        return _cairo_error(CAIRO_STATUS_TEMP_FILE_ERROR);
    }
    rewind((*surface).tmpfile);
    loop {
        n = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            (*surface).tmpfile,
        ) as libc::c_int;
        if !(n > 0 as libc::c_int) {
            break;
        }
        _cairo_output_stream_write(
            (*surface).final_stream,
            buf.as_mut_ptr() as *const libc::c_void,
            n as size_t,
        );
    }
    if ferror((*surface).tmpfile) != 0 as libc::c_int {
        return _cairo_error(CAIRO_STATUS_TEMP_FILE_ERROR);
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_footer(
    mut surface: *mut cairo_ps_surface_t,
) {
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%Trailer\n\0" as *const u8 as *const libc::c_char,
    );
    if (*surface).eps != 0 {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"end\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EOF\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn _path_covers_bbox(
    mut surface: *mut cairo_ps_surface_t,
    mut path: *mut cairo_path_fixed_t,
) -> cairo_bool_t {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    if _cairo_path_fixed_is_box(path, &mut box_0) != 0 {
        let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        _cairo_box_round_to_rectangle(&mut box_0, &mut rect);
        if _cairo_rectangle_intersect(&mut rect, &mut (*surface).surface_extents) != 0 {
            if rect.x == (*surface).surface_extents.x
                && rect.width == (*surface).surface_extents.width
                && rect.y == (*surface).surface_extents.y
                && rect.height == (*surface).surface_extents.height
            {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_ps_surface_clipper_intersect_clip_path(
    mut clipper: *mut cairo_surface_clipper_t,
    mut path: *mut cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
) -> cairo_status_t {
    let mut surface: *mut cairo_ps_surface_t = ({
        let mut mptr__: *const cairo_surface_clipper_t = clipper;
        (mptr__ as *mut libc::c_char).offset(-(688 as libc::c_ulong as isize))
            as *mut cairo_ps_surface_t
    });
    let mut stream: *mut cairo_output_stream_t = (*surface).stream;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*surface).paginated_mode as libc::c_uint
        != CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"surface->paginated_mode != CAIRO_PAGINATED_MODE_ANALYZE\0" as *const u8
                as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            1037 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 156],
                &[libc::c_char; 156],
            >(
                b"cairo_status_t _cairo_ps_surface_clipper_intersect_clip_path(cairo_surface_clipper_t *, cairo_path_fixed_t *, cairo_fill_rule_t, double, cairo_antialias_t)\0",
            ))
                .as_ptr(),
        );
    }
    if path.is_null() {
        status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators);
        if status as u64 != 0 {
            return status;
        }
        _cairo_output_stream_printf(
            stream,
            b"Q q\n\0" as *const u8 as *const libc::c_char,
        );
        (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
        _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
        return CAIRO_STATUS_SUCCESS;
    }
    if _path_covers_bbox(surface, path) != 0 {
        return CAIRO_STATUS_SUCCESS;
    }
    return _cairo_pdf_operators_clip(&mut (*surface).pdf_operators, path, fill_rule)
        as cairo_status_t;
}
unsafe extern "C" fn _ps_page_dimension_equal(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> cairo_bool_t {
    return (abs(a - b) < 5 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn _cairo_ps_surface_get_page_media(
    mut surface: *mut cairo_ps_surface_t,
) -> *const libc::c_char {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 50] = [0; 50];
    let mut page: *mut cairo_page_media_t = 0 as *mut cairo_page_media_t;
    let mut page_name: *const libc::c_char = 0 as *const libc::c_char;
    width = _cairo_lround((*surface).width);
    height = _cairo_lround((*surface).height);
    page = ({
        let mut mptr__: *const cairo_list_t = (*surface).document_media.next;
        (mptr__ as *mut libc::c_char).offset(-(16 as libc::c_ulong as isize))
            as *mut cairo_page_media_t
    });
    while &mut (*page).link as *mut cairo_list_t
        != &mut (*surface).document_media as *mut cairo_list_t
    {
        if _ps_page_dimension_equal(width, (*page).width) != 0
            && _ps_page_dimension_equal(height, (*page).height) != 0
        {
            return (*page).name;
        }
        page = ({
            let mut mptr__: *const cairo_list_t = (*page).link.next;
            (mptr__ as *mut libc::c_char).offset(-(16 as libc::c_ulong as isize))
                as *mut cairo_page_media_t
        });
    }
    page_name = 0 as *const libc::c_char;
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[cairo_page_standard_media_t; 17]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<cairo_page_standard_media_t>() as libc::c_ulong,
            ) as libc::c_int
    {
        if _ps_page_dimension_equal(width, _cairo_page_standard_media[i as usize].width)
            != 0
            && _ps_page_dimension_equal(
                height,
                _cairo_page_standard_media[i as usize].height,
            ) != 0
        {
            page_name = _cairo_page_standard_media[i as usize].name;
            width = _cairo_page_standard_media[i as usize].width;
            height = _cairo_page_standard_media[i as usize].height;
            break;
        } else {
            i += 1;
        }
    }
    page = (if ::std::mem::size_of::<cairo_page_media_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_page_media_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_page_media_t;
    if page.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *const libc::c_char;
    }
    if !page_name.is_null() {
        let ref mut fresh11 = (*page).name;
        *fresh11 = strdup(page_name);
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"%dx%dmm\0" as *const u8 as *const libc::c_char,
            _cairo_lround(
                (*surface).width * 25.4f64 / 72 as libc::c_int as libc::c_double,
            ),
            _cairo_lround(
                (*surface).height * 25.4f64 / 72 as libc::c_int as libc::c_double,
            ),
        );
        let ref mut fresh12 = (*page).name;
        *fresh12 = strdup(buf.as_mut_ptr());
    }
    if ((*page).name).is_null() {
        free(page as *mut libc::c_void);
        let mut status___0: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return 0 as *const libc::c_char;
    }
    (*page).width = width;
    (*page).height = height;
    cairo_list_add_tail(&mut (*page).link, &mut (*surface).document_media);
    return (*page).name;
}
unsafe extern "C" fn _cairo_ps_surface_create_for_stream_internal(
    mut stream: *mut cairo_output_stream_t,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> *mut cairo_surface_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status_ignored: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    surface = (if ::std::mem::size_of::<cairo_ps_surface_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<cairo_ps_surface_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut cairo_ps_surface_t;
    if surface.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        _cairo_surface_init(
            &mut (*surface).base,
            &cairo_ps_surface_backend,
            0 as *mut cairo_device_t,
            CAIRO_CONTENT_COLOR_ALPHA,
            1 as libc::c_int,
        );
        let ref mut fresh13 = (*surface).final_stream;
        *fresh13 = stream;
        let ref mut fresh14 = (*surface).tmpfile;
        *fresh14 = tmpfile();
        if ((*surface).tmpfile).is_null() {
            match *__errno_location() {
                12 => {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                }
                _ => {
                    status = _cairo_error(CAIRO_STATUS_TEMP_FILE_ERROR);
                }
            }
        } else {
            let ref mut fresh15 = (*surface).stream;
            *fresh15 = _cairo_output_stream_create_for_file((*surface).tmpfile);
            status = _cairo_output_stream_get_status((*surface).stream);
            if !(status as u64 != 0) {
                let ref mut fresh16 = (*surface).font_subsets;
                *fresh16 = _cairo_scaled_font_subsets_create_simple();
                if ((*surface).font_subsets).is_null() {
                    status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                } else {
                    _cairo_scaled_font_subsets_enable_latin_subset(
                        (*surface).font_subsets,
                        1 as libc::c_int,
                    );
                    (*surface).has_creation_date = 0 as libc::c_int;
                    (*surface).eps = 0 as libc::c_int;
                    (*surface).ps_level = CAIRO_PS_LEVEL_3;
                    (*surface).ps_level_used = CAIRO_PS_LEVEL_2;
                    (*surface).width = width;
                    (*surface).height = height;
                    cairo_matrix_init(
                        &mut (*surface).cairo_to_ps,
                        1 as libc::c_int as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        1 as libc::c_int as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                    );
                    (*surface).surface_extents.x = 0 as libc::c_int;
                    (*surface).surface_extents.y = 0 as libc::c_int;
                    (*surface)
                        .surface_extents
                        .width = ceil((*surface).width) as libc::c_int;
                    (*surface)
                        .surface_extents
                        .height = ceil((*surface).height) as libc::c_int;
                    (*surface).surface_bounded = 1 as libc::c_int;
                    (*surface).paginated_mode = CAIRO_PAGINATED_MODE_ANALYZE;
                    (*surface).force_fallbacks = 0 as libc::c_int;
                    (*surface).content = CAIRO_CONTENT_COLOR_ALPHA;
                    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
                    (*surface).document_bbox_p1.x = 0 as libc::c_int;
                    (*surface).document_bbox_p1.y = 0 as libc::c_int;
                    (*surface).document_bbox_p2.x = 0 as libc::c_int;
                    (*surface).document_bbox_p2.y = 0 as libc::c_int;
                    (*surface).total_form_size = 0 as libc::c_int as libc::c_long;
                    (*surface).contains_eps = 0 as libc::c_int;
                    (*surface).paint_proc = 0 as libc::c_int;
                    _cairo_surface_clipper_init(
                        &mut (*surface).clipper,
                        Some(
                            _cairo_ps_surface_clipper_intersect_clip_path
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
                        (*surface).stream,
                        &mut (*surface).cairo_to_ps,
                        (*surface).font_subsets,
                        1 as libc::c_int,
                    );
                    (*surface).num_pages = 0 as libc::c_int;
                    cairo_list_init(&mut (*surface).document_media);
                    _cairo_array_init(
                        &mut (*surface).dsc_header_comments,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    _cairo_array_init(
                        &mut (*surface).dsc_setup_comments,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    _cairo_array_init(
                        &mut (*surface).dsc_page_setup_comments,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    _cairo_array_init(
                        &mut (*surface).recording_surf_stack,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                            as libc::c_uint,
                    );
                    (*surface).num_forms = 0 as libc::c_int;
                    let ref mut fresh17 = (*surface).forms;
                    *fresh17 = _cairo_hash_table_create(
                        Some(
                            _cairo_ps_form_equal
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> cairo_bool_t,
                        ),
                    );
                    if ((*surface).forms).is_null() {
                        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
                    } else {
                        let ref mut fresh18 = (*surface).dsc_comment_target;
                        *fresh18 = &mut (*surface).dsc_header_comments;
                        let ref mut fresh19 = (*surface).paginated_surface;
                        *fresh19 = _cairo_paginated_surface_create(
                            &mut (*surface).base,
                            CAIRO_CONTENT_COLOR_ALPHA,
                            &cairo_ps_surface_paginated_backend,
                        );
                        status = (*(*surface).paginated_surface).status;
                        if status as libc::c_uint
                            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                        {
                            cairo_surface_destroy(&mut (*surface).base);
                            return (*surface).paginated_surface;
                        }
                    }
                    _cairo_scaled_font_subsets_destroy((*surface).font_subsets);
                }
            }
            status_ignored = _cairo_output_stream_destroy((*surface).stream);
            fclose((*surface).tmpfile);
        }
        free(surface as *mut libc::c_void);
    }
    status_ignored = _cairo_output_stream_destroy(stream);
    return _cairo_surface_create_in_error(status);
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_create(
    mut filename: *const libc::c_char,
    mut width_in_points: libc::c_double,
    mut height_in_points: libc::c_double,
) -> *mut cairo_surface_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    stream = _cairo_output_stream_create_for_filename(filename);
    if _cairo_output_stream_get_status(stream) as u64 != 0 {
        return _cairo_surface_create_in_error(_cairo_output_stream_destroy(stream));
    }
    return _cairo_ps_surface_create_for_stream_internal(
        stream,
        width_in_points,
        height_in_points,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_create_for_stream(
    mut write_func: cairo_write_func_t,
    mut closure: *mut libc::c_void,
    mut width_in_points: libc::c_double,
    mut height_in_points: libc::c_double,
) -> *mut cairo_surface_t {
    let mut stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    stream = _cairo_output_stream_create(write_func, None, closure);
    if _cairo_output_stream_get_status(stream) as u64 != 0 {
        return _cairo_surface_create_in_error(_cairo_output_stream_destroy(stream));
    }
    return _cairo_ps_surface_create_for_stream_internal(
        stream,
        width_in_points,
        height_in_points,
    );
}
unsafe extern "C" fn _cairo_surface_is_ps(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    return ((*surface).backend
        == &cairo_ps_surface_backend as *const cairo_surface_backend_t) as libc::c_int;
}
unsafe extern "C" fn _extract_ps_surface(
    mut surface: *mut cairo_surface_t,
    mut set_error_on_failure: cairo_bool_t,
    mut ps_surface: *mut *mut cairo_ps_surface_t,
) -> cairo_bool_t {
    let mut target: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if (*surface).status as u64 != 0 {
        return 0 as libc::c_int;
    }
    if (*surface).finished() != 0 {
        if set_error_on_failure != 0 {
            _cairo_surface_set_error(
                surface,
                _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
            );
        }
        return 0 as libc::c_int;
    }
    if _cairo_surface_is_paginated(surface) == 0 {
        if set_error_on_failure != 0 {
            _cairo_surface_set_error(
                surface,
                _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
            );
        }
        return 0 as libc::c_int;
    }
    target = _cairo_paginated_surface_get_target(surface);
    if (*target).status as u64 != 0 {
        if set_error_on_failure != 0 {
            _cairo_surface_set_error(surface, (*target).status as cairo_int_status_t);
        }
        return 0 as libc::c_int;
    }
    if (*target).finished() != 0 {
        if set_error_on_failure != 0 {
            _cairo_surface_set_error(
                surface,
                _cairo_error(CAIRO_STATUS_SURFACE_FINISHED) as cairo_int_status_t,
            );
        }
        return 0 as libc::c_int;
    }
    if _cairo_surface_is_ps(target) == 0 {
        if set_error_on_failure != 0 {
            _cairo_surface_set_error(
                surface,
                _cairo_error(CAIRO_STATUS_SURFACE_TYPE_MISMATCH) as cairo_int_status_t,
            );
        }
        return 0 as libc::c_int;
    }
    *ps_surface = target as *mut cairo_ps_surface_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_restrict_to_level(
    mut surface: *mut cairo_surface_t,
    mut level: cairo_ps_level_t,
) {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    if _extract_ps_surface(surface, 1 as libc::c_int, &mut ps_surface) == 0 {
        return;
    }
    if (level as libc::c_uint)
        < (::std::mem::size_of::<[cairo_ps_level_t; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_ps_level_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        (*ps_surface).ps_level = level;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_get_levels(
    mut levels: *mut *const cairo_ps_level_t,
    mut num_levels: *mut libc::c_int,
) {
    if !levels.is_null() {
        *levels = _cairo_ps_levels.as_ptr();
    }
    if !num_levels.is_null() {
        *num_levels = (::std::mem::size_of::<[cairo_ps_level_t; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_ps_level_t>() as libc::c_ulong)
            as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_level_to_string(
    mut level: cairo_ps_level_t,
) -> *const libc::c_char {
    if level as libc::c_uint
        >= (::std::mem::size_of::<[cairo_ps_level_t; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cairo_ps_level_t>() as libc::c_ulong)
            as libc::c_int as libc::c_uint
    {
        return 0 as *const libc::c_char;
    }
    return _cairo_ps_level_strings[level as usize];
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_set_eps(
    mut surface: *mut cairo_surface_t,
    mut eps: cairo_bool_t,
) {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    if _extract_ps_surface(surface, 1 as libc::c_int, &mut ps_surface) == 0 {
        return;
    }
    (*ps_surface).eps = eps;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_get_eps(
    mut surface: *mut cairo_surface_t,
) -> cairo_bool_t {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    if _extract_ps_surface(surface, 0 as libc::c_int, &mut ps_surface) == 0 {
        return 0 as libc::c_int;
    }
    return (*ps_surface).eps;
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_set_size(
    mut surface: *mut cairo_surface_t,
    mut width_in_points: libc::c_double,
    mut height_in_points: libc::c_double,
) {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if _extract_ps_surface(surface, 1 as libc::c_int, &mut ps_surface) == 0 {
        return;
    }
    (*ps_surface).width = width_in_points;
    (*ps_surface).height = height_in_points;
    cairo_matrix_init(
        &mut (*ps_surface).cairo_to_ps,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    (*ps_surface).surface_extents.x = 0 as libc::c_int;
    (*ps_surface).surface_extents.y = 0 as libc::c_int;
    (*ps_surface).surface_extents.width = ceil((*ps_surface).width) as libc::c_int;
    (*ps_surface).surface_extents.height = ceil((*ps_surface).height) as libc::c_int;
    _cairo_pdf_operators_set_cairo_to_pdf_matrix(
        &mut (*ps_surface).pdf_operators,
        &mut (*ps_surface).cairo_to_ps,
    );
    status = _cairo_paginated_surface_set_size(
        (*ps_surface).paginated_surface,
        width_in_points as libc::c_int,
        height_in_points as libc::c_int,
    );
    if status as u64 != 0 {
        status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_dsc_comment(
    mut surface: *mut cairo_surface_t,
    mut comment: *const libc::c_char,
) {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut comment_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if _extract_ps_surface(surface, 1 as libc::c_int, &mut ps_surface) == 0 {
        return;
    }
    if comment.is_null() {
        status = _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_NULL_POINTER as libc::c_int as cairo_int_status_t,
        ) as cairo_status_t;
        return;
    }
    if *comment.offset(0 as libc::c_int as isize) as libc::c_int != '%' as i32
        || strlen(comment) > 255 as libc::c_int as libc::c_ulong
    {
        status = _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_INVALID_DSC_COMMENT as libc::c_int as cairo_int_status_t,
        ) as cairo_status_t;
        return;
    }
    comment_copy = strdup(comment);
    if comment_copy.is_null() {
        status = _cairo_surface_set_error(
            surface,
            CAIRO_STATUS_NO_MEMORY as libc::c_int as cairo_int_status_t,
        ) as cairo_status_t;
        return;
    }
    status = _cairo_array_append(
        (*ps_surface).dsc_comment_target,
        &mut comment_copy as *mut *mut libc::c_char as *const libc::c_void,
    );
    if status as u64 != 0 {
        free(comment_copy as *mut libc::c_void);
        status = _cairo_surface_set_error(surface, status as cairo_int_status_t)
            as cairo_status_t;
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_dsc_begin_setup(
    mut surface: *mut cairo_surface_t,
) {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    if _extract_ps_surface(surface, 1 as libc::c_int, &mut ps_surface) == 0 {
        return;
    }
    if (*ps_surface).dsc_comment_target
        == &mut (*ps_surface).dsc_header_comments as *mut cairo_array_t
    {
        let ref mut fresh20 = (*ps_surface).dsc_comment_target;
        *fresh20 = &mut (*ps_surface).dsc_setup_comments;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cairo_ps_surface_dsc_begin_page_setup(
    mut surface: *mut cairo_surface_t,
) {
    let mut ps_surface: *mut cairo_ps_surface_t = 0 as *mut cairo_ps_surface_t;
    if _extract_ps_surface(surface, 1 as libc::c_int, &mut ps_surface) == 0 {
        return;
    }
    if (*ps_surface).dsc_comment_target
        == &mut (*ps_surface).dsc_header_comments as *mut cairo_array_t
        || (*ps_surface).dsc_comment_target
            == &mut (*ps_surface).dsc_setup_comments as *mut cairo_array_t
    {
        let ref mut fresh21 = (*ps_surface).dsc_comment_target;
        *fresh21 = &mut (*ps_surface).dsc_page_setup_comments;
    }
}
unsafe extern "C" fn _cairo_ps_surface_finish(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let mut i: libc::c_int = 0;
    let mut num_comments: libc::c_int = 0;
    let mut comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    status = (*surface).base.status;
    if !(status as u64 != 0) {
        _cairo_ps_surface_emit_header(surface);
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"%%%%BeginSetup\n\0" as *const u8 as *const libc::c_char,
        );
        num_comments = _cairo_array_num_elements(&mut (*surface).dsc_setup_comments)
            as libc::c_int;
        if num_comments != 0 {
            comments = _cairo_array_index(
                &mut (*surface).dsc_setup_comments,
                0 as libc::c_int as libc::c_uint,
            ) as *mut *mut libc::c_char;
            i = 0 as libc::c_int;
            while i < num_comments {
                _cairo_output_stream_printf(
                    (*surface).final_stream,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    *comments.offset(i as isize),
                );
                free(*comments.offset(i as isize) as *mut libc::c_void);
                let ref mut fresh22 = *comments.offset(i as isize);
                *fresh22 = 0 as *mut libc::c_char;
                i += 1;
            }
        }
        status = _cairo_ps_surface_emit_font_subsets(surface);
        if !(status as u64 != 0) {
            status = _cairo_ps_surface_emit_forms(surface) as cairo_status_t;
            if !(status as u64 != 0) {
                _cairo_output_stream_printf(
                    (*surface).final_stream,
                    b"%%%%EndSetup\n\0" as *const u8 as *const libc::c_char,
                );
                status = _cairo_ps_surface_emit_body(surface);
                if !(status as u64 != 0) {
                    _cairo_ps_surface_emit_footer(surface);
                }
            }
        }
    }
    _cairo_hash_table_foreach(
        (*surface).forms,
        Some(
            _cairo_ps_form_pluck
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        (*surface).forms as *mut libc::c_void,
    );
    _cairo_hash_table_destroy((*surface).forms);
    _cairo_scaled_font_subsets_destroy((*surface).font_subsets);
    status2 = _cairo_output_stream_destroy((*surface).stream);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    fclose((*surface).tmpfile);
    status2 = _cairo_output_stream_destroy((*surface).final_stream);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    while cairo_list_is_empty(&mut (*surface).document_media) == 0 {
        let mut page: *mut cairo_page_media_t = 0 as *mut cairo_page_media_t;
        page = ({
            let mut mptr__: *const cairo_list_t = (*surface).document_media.next;
            (mptr__ as *mut libc::c_char).offset(-(16 as libc::c_ulong as isize))
                as *mut cairo_page_media_t
        });
        cairo_list_del(&mut (*page).link);
        free((*page).name as *mut libc::c_void);
        free(page as *mut libc::c_void);
    }
    num_comments = _cairo_array_num_elements(&mut (*surface).dsc_header_comments)
        as libc::c_int;
    comments = _cairo_array_index(
        &mut (*surface).dsc_header_comments,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < num_comments {
        free(*comments.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).dsc_header_comments);
    num_comments = _cairo_array_num_elements(&mut (*surface).dsc_setup_comments)
        as libc::c_int;
    comments = _cairo_array_index(
        &mut (*surface).dsc_setup_comments,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < num_comments {
        free(*comments.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).dsc_setup_comments);
    num_comments = _cairo_array_num_elements(&mut (*surface).dsc_page_setup_comments)
        as libc::c_int;
    comments = _cairo_array_index(
        &mut (*surface).dsc_page_setup_comments,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < num_comments {
        free(*comments.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    _cairo_array_fini(&mut (*surface).dsc_page_setup_comments);
    _cairo_array_fini(&mut (*surface).recording_surf_stack);
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_start_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let ref mut fresh23 = (*surface).num_pages;
    *fresh23 += 1;
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_show_page(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    if !((*surface).clipper.clip).is_null() {
        _cairo_surface_clipper_reset(&mut (*surface).clipper);
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"Q Q\nshowpage\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn color_is_gray(
    mut red: libc::c_double,
    mut green: libc::c_double,
    mut blue: libc::c_double,
) -> cairo_bool_t {
    let epsilon: libc::c_double = 0.00001f64;
    return (fabs(red - green) < epsilon && fabs(red - blue) < epsilon) as libc::c_int;
}
unsafe extern "C" fn _cairo_ps_surface_acquire_source_surface_from_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
    mut src_surface_extents: *mut cairo_rectangle_int_t,
    mut src_surface_bounded: *mut cairo_bool_t,
    mut src_op_extents: *mut cairo_rectangle_int_t,
    mut source_surface: *mut *mut cairo_surface_t,
    mut x_offset: *mut libc::c_double,
    mut y_offset: *mut libc::c_double,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut bbox: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    *x_offset = 0 as libc::c_int as libc::c_double;
    *y_offset = 0 as libc::c_int as libc::c_double;
    _cairo_box_from_rectangle(&mut bbox, extents);
    _cairo_matrix_transform_bounding_box_fixed(
        &(*pattern).matrix,
        &mut bbox,
        0 as *mut cairo_bool_t,
    );
    _cairo_box_round_to_rectangle(&mut bbox, src_op_extents);
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        let mut surf: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        surf = _cairo_raster_source_pattern_acquire(
            pattern,
            &mut (*surface).base,
            src_op_extents,
        );
        if surf.is_null() {
            return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
        }
        *src_surface_bounded = _cairo_surface_get_extents(surf, src_surface_extents);
        cairo_surface_get_device_offset(surf, x_offset, y_offset);
        *source_surface = surf;
    } else if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        let mut surf_0: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
        *source_surface = (*(pattern as *mut cairo_surface_pattern_t)).surface;
        surf_0 = *source_surface;
        *src_surface_bounded = _cairo_surface_get_extents(surf_0, src_surface_extents);
        if (*surf_0).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
        {
            if _cairo_surface_is_snapshot(surf_0) != 0 {
                surf_0 = _cairo_surface_snapshot_get_target(surf_0);
            }
            if (*(*surf_0).backend).type_0 as libc::c_uint
                == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
            {
                let mut sub: *mut cairo_surface_subsurface_t = surf_0
                    as *mut cairo_surface_subsurface_t;
                *src_surface_extents = (*sub).extents;
                *src_surface_bounded = 1 as libc::c_int;
                *x_offset = -(*sub).extents.x as libc::c_double;
                *y_offset = -(*sub).extents.y as libc::c_double;
            }
            cairo_surface_destroy(surf_0);
        } else if (*surf_0).type_0 as libc::c_uint
            != CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        {
            let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
            let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
            status = _cairo_surface_acquire_source_image(
                surf_0,
                &mut image,
                &mut image_extra,
            );
            if status as u64 != 0 {
                return status;
            }
            *src_surface_bounded = _cairo_surface_get_extents(
                &mut (*image).base,
                src_surface_extents,
            );
            _cairo_surface_release_source_image(surf_0, image, image_extra);
        }
    } else if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            1965 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 253],
                &[libc::c_char; 253],
            >(
                b"cairo_status_t _cairo_ps_surface_acquire_source_surface_from_pattern(cairo_ps_surface_t *, const cairo_pattern_t *, const cairo_rectangle_int_t *, cairo_rectangle_int_t *, cairo_bool_t *, cairo_rectangle_int_t *, cairo_surface_t **, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_release_source_surface_from_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut source_surface: *mut cairo_surface_t,
) {
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        _cairo_raster_source_pattern_release(pattern, source_surface);
    }
}
unsafe extern "C" fn _cairo_ps_surface_create_padded_image_from_image(
    mut surface: *mut cairo_ps_surface_t,
    mut source: *mut cairo_image_surface_t,
    mut source_matrix: *const cairo_matrix_t,
    mut extents: *const cairo_rectangle_int_t,
    mut image: *mut *mut cairo_image_surface_t,
    mut image_extents: *mut cairo_rectangle_int_t,
) -> cairo_status_t {
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
    let mut pad_image: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    _cairo_box_from_rectangle(&mut box_0, extents);
    _cairo_matrix_transform_bounding_box_fixed(
        source_matrix,
        &mut box_0,
        0 as *mut cairo_bool_t,
    );
    _cairo_box_round_to_rectangle(&mut box_0, &mut rect);
    w = (*source).width;
    h = (*source).height;
    if _cairo_fixed_integer_ceil(box_0.p1.x) < 0 as libc::c_int
        || _cairo_fixed_integer_ceil(box_0.p1.y) < 0 as libc::c_int
        || _cairo_fixed_integer_floor(box_0.p2.y) > w
        || _cairo_fixed_integer_floor(box_0.p2.y) > h
    {
        pad_image = _cairo_image_surface_create_with_content(
            (*source).base.content,
            rect.width,
            rect.height,
        );
        if (*pad_image).status as u64 != 0 {
            return (*pad_image).status;
        }
        _cairo_pattern_init_for_surface(&mut pad_pattern, &mut (*source).base);
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
        *image = pad_image as *mut cairo_image_surface_t;
        (*image_extents).x = rect.x;
        (*image_extents).y = rect.y;
        (*image_extents).width = rect.width;
        (*image_extents).height = rect.height;
    } else {
        *image = 0 as *mut cairo_image_surface_t;
        status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    return status as cairo_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_analyze_surface_pattern_transparency(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut src_surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut src_surface_bounded: cairo_bool_t = 0;
    let mut src_op_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut source_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut x_offset: libc::c_double = 0.;
    let mut y_offset: libc::c_double = 0.;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    status = _cairo_ps_surface_acquire_source_surface_from_pattern(
        surface,
        pattern,
        extents,
        &mut src_surface_extents,
        &mut src_surface_bounded,
        &mut src_op_extents,
        &mut source_surface,
        &mut x_offset,
        &mut y_offset,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_surface_acquire_source_image(
        source_surface,
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
    match transparency as libc::c_uint {
        0 => {
            status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
        1 => {
            if (*surface).ps_level as libc::c_uint
                == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
            {
                status = CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY;
            } else {
                (*surface).ps_level_used = CAIRO_PS_LEVEL_3;
                status = CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
            }
        }
        2 => {
            status = CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY;
        }
        3 => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                    2099 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 152],
                        &[libc::c_char; 152],
                    >(
                        b"cairo_int_status_t _cairo_ps_surface_analyze_surface_pattern_transparency(cairo_ps_surface_t *, const cairo_pattern_t *, const cairo_rectangle_int_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        _ => {}
    }
    _cairo_surface_release_source_image(source_surface, image, image_extra);
    _cairo_ps_surface_release_source_surface_from_pattern(
        surface,
        pattern,
        source_surface,
    );
    return status;
}
unsafe extern "C" fn surface_pattern_supported(
    mut pattern: *const cairo_surface_pattern_t,
) -> cairo_bool_t {
    if (*(*pattern).surface).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if ((*(*(*pattern).surface).backend).acquire_source_image).is_none() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _gradient_pattern_supported(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    let mut min_alpha: libc::c_double = 0.;
    let mut max_alpha: libc::c_double = 0.;
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    _cairo_pattern_alpha_range(pattern, &mut min_alpha, &mut max_alpha);
    if min_alpha != max_alpha {
        return 0 as libc::c_int;
    }
    (*surface).ps_level_used = CAIRO_PS_LEVEL_3;
    return 1 as libc::c_int;
}
unsafe extern "C" fn pattern_supported(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
) -> cairo_bool_t {
    match (*pattern).type_0 as libc::c_uint {
        0 => return 1 as libc::c_int,
        2 | 3 | 4 => return _gradient_pattern_supported(surface, pattern),
        1 => return surface_pattern_supported(pattern as *mut cairo_surface_pattern_t),
        5 => return 1 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                    2168 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"cairo_bool_t pattern_supported(cairo_ps_surface_t *, const cairo_pattern_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn mask_supported(
    mut surface: *mut cairo_ps_surface_t,
    mut mask: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*mask).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
    {
        let mut surface_pattern: *mut cairo_surface_pattern_t = mask
            as *mut cairo_surface_pattern_t;
        if (*(*surface_pattern).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        {
            if _cairo_ps_surface_analyze_surface_pattern_transparency(
                surface,
                mask,
                extents,
            ) as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                (*surface).ps_level_used = CAIRO_PS_LEVEL_3;
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _cairo_ps_surface_analyze_operation(
    mut surface: *mut cairo_ps_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_int_status_t {
    let mut min_alpha: libc::c_double = 0.;
    if (*surface).force_fallbacks != 0
        && (*surface).paginated_mode as libc::c_uint
            == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if pattern_supported(surface, pattern) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if !(op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
        || op as libc::c_uint == CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint)
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if !mask.is_null() && mask_supported(surface, mask, extents) == 0 {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
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
    if op as libc::c_uint == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
        if !mask.is_null() {
            return CAIRO_INT_STATUS_UNSUPPORTED
        } else {
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t
        }
    }
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        || (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_RASTER_SOURCE as libc::c_int as libc::c_uint
    {
        return _cairo_ps_surface_analyze_surface_pattern_transparency(
            surface,
            pattern,
            extents,
        );
    }
    _cairo_pattern_alpha_range(pattern, &mut min_alpha, 0 as *mut libc::c_double);
    if min_alpha
        >= 0xff00 as libc::c_int as libc::c_double
            / 0xffff as libc::c_int as libc::c_double
    {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    return CAIRO_INT_STATUS_FLATTEN_TRANSPARENCY;
}
unsafe extern "C" fn _cairo_ps_surface_operation_supported(
    mut surface: *mut cairo_ps_surface_t,
    mut op: cairo_operator_t,
    mut pattern: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_bool_t {
    return (_cairo_ps_surface_analyze_operation(surface, op, pattern, mask, extents)
        as libc::c_uint != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn _base85_string_wrap_stream_write(
    mut base: *mut cairo_output_stream_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    let mut stream: *mut string_array_stream_t = base as *mut string_array_stream_t;
    let mut c: libc::c_uchar = 0;
    if length == 0 as libc::c_int as libc::c_uint {
        return CAIRO_STATUS_SUCCESS;
    }
    loop {
        let fresh24 = length;
        length = length.wrapping_sub(1);
        if !(fresh24 != 0) {
            break;
        }
        if (*stream).column == 0 as libc::c_int {
            if (*stream).use_strings != 0 {
                _cairo_output_stream_printf(
                    (*stream).output,
                    b"<~\0" as *const u8 as *const libc::c_char,
                );
                (*stream).column = 2 as libc::c_int;
            } else {
                _cairo_output_stream_printf(
                    (*stream).output,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                (*stream).column = 1 as libc::c_int;
            }
        }
        let fresh25 = data;
        data = data.offset(1);
        c = *fresh25;
        _cairo_output_stream_write(
            (*stream).output,
            &mut c as *mut libc::c_uchar as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        let ref mut fresh26 = (*stream).column;
        *fresh26 += 1;
        if c as libc::c_int == 'z' as i32 {
            (*stream).string_size += 4 as libc::c_int;
            (*stream).tuple_count = 0 as libc::c_int;
        } else {
            let ref mut fresh27 = (*stream).tuple_count;
            *fresh27 += 1;
            if *fresh27 == 5 as libc::c_int {
                (*stream).string_size += 4 as libc::c_int;
                (*stream).tuple_count = 0 as libc::c_int;
            }
        }
        if (*stream).use_strings != 0 && (*stream).tuple_count == 0 as libc::c_int
            && (*stream).string_size > 65535 as libc::c_int - 4 as libc::c_int
        {
            _cairo_output_stream_printf(
                (*stream).output,
                b"~>\n\0" as *const u8 as *const libc::c_char,
            );
            (*stream).string_size = 0 as libc::c_int;
            (*stream).column = 0 as libc::c_int;
        }
        if (*stream).column >= 72 as libc::c_int {
            _cairo_output_stream_printf(
                (*stream).output,
                b"\n \0" as *const u8 as *const libc::c_char,
            );
            (*stream).column = 1 as libc::c_int;
        }
    }
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _base85_string_wrap_stream_close(
    mut base: *mut cairo_output_stream_t,
) -> cairo_status_t {
    let mut stream: *mut string_array_stream_t = base as *mut string_array_stream_t;
    if (*stream).use_strings == 0 || (*stream).string_size != 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*stream).output,
            b"~>\0" as *const u8 as *const libc::c_char,
        );
    }
    return _cairo_output_stream_get_status((*stream).output);
}
unsafe extern "C" fn _base85_strings_stream_create(
    mut output: *mut cairo_output_stream_t,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut string_array_stream_t = 0 as *mut string_array_stream_t;
    stream = (if ::std::mem::size_of::<string_array_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<string_array_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut string_array_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            _base85_string_wrap_stream_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            _base85_string_wrap_stream_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh28 = (*stream).output;
    *fresh28 = output;
    (*stream).column = 0 as libc::c_int;
    (*stream).string_size = 0 as libc::c_int;
    (*stream).tuple_count = 0 as libc::c_int;
    (*stream).use_strings = 1 as libc::c_int;
    return &mut (*stream).base;
}
unsafe extern "C" fn _base85_wrap_stream_create(
    mut output: *mut cairo_output_stream_t,
) -> *mut cairo_output_stream_t {
    let mut stream: *mut string_array_stream_t = 0 as *mut string_array_stream_t;
    stream = (if ::std::mem::size_of::<string_array_stream_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        malloc(::std::mem::size_of::<string_array_stream_t>() as libc::c_ulong)
    } else {
        0 as *mut libc::c_void
    }) as *mut string_array_stream_t;
    if stream.is_null() {
        let mut status__: cairo_status_t = _cairo_error(CAIRO_STATUS_NO_MEMORY);
        return &_cairo_output_stream_nil as *const cairo_output_stream_t
            as *mut cairo_output_stream_t;
    }
    _cairo_output_stream_init(
        &mut (*stream).base,
        Some(
            _base85_string_wrap_stream_write
                as unsafe extern "C" fn(
                    *mut cairo_output_stream_t,
                    *const libc::c_uchar,
                    libc::c_uint,
                ) -> cairo_status_t,
        ),
        None,
        Some(
            _base85_string_wrap_stream_close
                as unsafe extern "C" fn(*mut cairo_output_stream_t) -> cairo_status_t,
        ),
    );
    let ref mut fresh29 = (*stream).output;
    *fresh29 = output;
    (*stream).column = 0 as libc::c_int;
    (*stream).string_size = 0 as libc::c_int;
    (*stream).tuple_count = 0 as libc::c_int;
    (*stream).use_strings = 0 as libc::c_int;
    return &mut (*stream).base;
}
unsafe extern "C" fn _cairo_ps_surface_flatten_image_transparency(
    mut surface: *mut cairo_ps_surface_t,
    mut image: *mut cairo_image_surface_t,
    mut opaque_image: *mut *mut cairo_image_surface_t,
) -> cairo_status_t {
    let mut opaque: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    opaque = cairo_image_surface_create(
        CAIRO_FORMAT_RGB24,
        (*image).width,
        (*image).height,
    );
    if (*opaque).status as u64 != 0 {
        return (*opaque).status;
    }
    if (*surface).content as libc::c_uint
        == CAIRO_CONTENT_COLOR_ALPHA as libc::c_int as libc::c_uint
    {
        status = _cairo_surface_paint(
            opaque,
            CAIRO_OPERATOR_SOURCE,
            &_cairo_pattern_white.base,
            0 as *const cairo_clip_t,
        );
        if status as u64 != 0 {
            cairo_surface_destroy(opaque);
            return status;
        }
    }
    _cairo_pattern_init_for_surface(&mut pattern, &mut (*image).base);
    pattern.base.filter = CAIRO_FILTER_NEAREST;
    status = _cairo_surface_paint(
        opaque,
        CAIRO_OPERATOR_OVER,
        &mut pattern.base,
        0 as *const cairo_clip_t,
    );
    _cairo_pattern_fini(&mut pattern.base);
    if status as u64 != 0 {
        cairo_surface_destroy(opaque);
        return status;
    }
    *opaque_image = opaque as *mut cairo_image_surface_t;
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_base85_string(
    mut surface: *mut cairo_ps_surface_t,
    mut data: *const libc::c_uchar,
    mut length: libc::c_ulong,
    mut compress: cairo_ps_compress_t,
    mut use_strings: cairo_bool_t,
) -> cairo_status_t {
    let mut base85_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut string_array_stream: *mut cairo_output_stream_t = 0
        as *mut cairo_output_stream_t;
    let mut deflate_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut data_compressed: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_compressed_size: libc::c_ulong = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut status2: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut this_cannot_be_handled: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if use_strings != 0 {
        string_array_stream = _base85_strings_stream_create((*surface).stream);
    } else {
        string_array_stream = _base85_wrap_stream_create((*surface).stream);
    }
    status = _cairo_output_stream_get_status(string_array_stream);
    if status as u64 != 0 {
        return _cairo_output_stream_destroy(string_array_stream);
    }
    base85_stream = _cairo_base85_stream_create(string_array_stream);
    status = _cairo_output_stream_get_status(base85_stream);
    if status as u64 != 0 {
        status2 = _cairo_output_stream_destroy(string_array_stream);
        return _cairo_output_stream_destroy(base85_stream);
    }
    status = CAIRO_STATUS_SUCCESS;
    match compress as libc::c_uint {
        0 => {
            _cairo_output_stream_write(
                base85_stream,
                data as *const libc::c_void,
                length,
            );
        }
        1 => {
            data_compressed_size = length;
            data_compressed = _cairo_lzw_compress(
                data as *mut libc::c_uchar,
                &mut data_compressed_size,
            );
            if data_compressed.is_null() {
                this_cannot_be_handled = _cairo_output_stream_destroy(
                    string_array_stream,
                );
                this_cannot_be_handled = _cairo_output_stream_destroy(base85_stream);
                return _cairo_error(CAIRO_STATUS_NO_MEMORY);
            }
            _cairo_output_stream_write(
                base85_stream,
                data_compressed as *const libc::c_void,
                data_compressed_size,
            );
            free(data_compressed as *mut libc::c_void);
        }
        2 => {
            deflate_stream = _cairo_deflate_stream_create(base85_stream);
            if _cairo_output_stream_get_status(deflate_stream) as u64 != 0 {
                return _cairo_output_stream_destroy(deflate_stream);
            }
            _cairo_output_stream_write(
                deflate_stream,
                data as *const libc::c_void,
                length,
            );
            status = _cairo_output_stream_destroy(deflate_stream);
            if status as u64 != 0 {
                this_cannot_be_handled = _cairo_output_stream_destroy(
                    string_array_stream,
                );
                this_cannot_be_handled = _cairo_output_stream_destroy(base85_stream);
                return status;
            }
        }
        _ => {}
    }
    status = _cairo_output_stream_destroy(base85_stream);
    status2 = _cairo_output_stream_destroy(string_array_stream);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
        status = status2;
    }
    return status;
}
unsafe extern "C" fn get_interpolate(mut filter: cairo_filter_t) -> *const libc::c_char {
    let mut interpolate: *const libc::c_char = 0 as *const libc::c_char;
    match filter as libc::c_uint {
        0 | 3 | 5 => {
            interpolate = b"false\0" as *const u8 as *const libc::c_char;
        }
        1 | 2 | 4 | _ => {
            interpolate = b"true\0" as *const u8 as *const libc::c_char;
        }
    }
    return interpolate;
}
unsafe extern "C" fn _cairo_ps_surface_emit_image(
    mut surface: *mut cairo_ps_surface_t,
    mut mode: cairo_emit_surface_mode_t,
    mut params: *mut cairo_emit_surface_params_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_size: libc::c_ulong = 0;
    let mut ps_image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut transparency: cairo_image_transparency_t = CAIRO_IMAGE_IS_OPAQUE;
    let mut use_mask: cairo_bool_t = 0;
    let mut pixel32: *mut uint32_t = 0 as *mut uint32_t;
    let mut pixel8: *mut uint8_t = 0 as *mut uint8_t;
    let mut bit: libc::c_int = 0;
    let mut color: cairo_image_color_t = CAIRO_IMAGE_IS_COLOR;
    let mut interpolate: *const libc::c_char = 0 as *const libc::c_char;
    let mut compress: cairo_ps_compress_t = CAIRO_PS_COMPRESS_NONE;
    let mut compress_filter: *const libc::c_char = 0 as *const libc::c_char;
    let mut image_surf: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut image_extra: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*(*params).src_surface).status as u64 != 0 {
        return (*(*params).src_surface).status;
    }
    status = _cairo_surface_acquire_source_image(
        (*params).src_surface,
        &mut image_surf,
        &mut image_extra,
    );
    if status as u64 != 0 {
        return status;
    }
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
        surf = _cairo_image_surface_create_with_content(
            (*image).base.content,
            (*image).width,
            (*image).height,
        );
        if (*surf).status as u64 != 0 {
            status = (*surf).status;
            current_block = 11395700565920852994;
        } else {
            _cairo_pattern_init_for_surface(&mut pattern, &mut (*image).base);
            status = _cairo_surface_paint(
                surf,
                CAIRO_OPERATOR_SOURCE,
                &mut pattern.base,
                0 as *const cairo_clip_t,
            );
            _cairo_pattern_fini(&mut pattern.base);
            image = surf as *mut cairo_image_surface_t;
            if status as u64 != 0 {
                current_block = 11395700565920852994;
            } else {
                current_block = 16203760046146113240;
            }
        }
    } else {
        current_block = 16203760046146113240;
    }
    match current_block {
        16203760046146113240 => {
            ps_image = image;
            interpolate = get_interpolate((*params).filter);
            if (*params).stencil_mask != 0 {
                use_mask = 0 as libc::c_int;
                color = CAIRO_IMAGE_IS_MONOCHROME;
                transparency = CAIRO_IMAGE_HAS_BILEVEL_ALPHA;
            } else {
                transparency = _cairo_image_analyze_transparency(image);
                if (*params).op as libc::c_uint
                    == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
                    || transparency as libc::c_uint
                        == CAIRO_IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
                    || transparency as libc::c_uint
                        == CAIRO_IMAGE_HAS_BILEVEL_ALPHA as libc::c_int as libc::c_uint
                        && (*surface).ps_level as libc::c_uint
                            == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
                {
                    status = _cairo_ps_surface_flatten_image_transparency(
                        surface,
                        image,
                        &mut ps_image,
                    );
                    if status as u64 != 0 {
                        return status;
                    }
                    use_mask = 0 as libc::c_int;
                } else if transparency as libc::c_uint
                    == CAIRO_IMAGE_IS_OPAQUE as libc::c_int as libc::c_uint
                {
                    use_mask = 0 as libc::c_int;
                } else {
                    use_mask = 1 as libc::c_int;
                }
                color = _cairo_image_analyze_color(ps_image);
            }
            let mut current_block_38: u64;
            match color as libc::c_uint {
                0 => {
                    current_block_38 = 1297980239089815;
                }
                1 => {
                    data_size = (*ps_image).width as libc::c_ulong;
                    current_block_38 = 2290177392965769716;
                }
                2 => {
                    data_size = (((*ps_image).width + 7 as libc::c_int)
                        / 8 as libc::c_int) as libc::c_ulong;
                    current_block_38 = 2290177392965769716;
                }
                3 | _ => {
                    if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                    {} else {
                        __assert_fail(
                            b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                            b"../src/cairo-ps-surface.c\0" as *const u8
                                as *const libc::c_char,
                            2672 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 124],
                                &[libc::c_char; 124],
                            >(
                                b"cairo_status_t _cairo_ps_surface_emit_image(cairo_ps_surface_t *, cairo_emit_surface_mode_t, cairo_emit_surface_params_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    current_block_38 = 1297980239089815;
                }
            }
            match current_block_38 {
                1297980239089815 => {
                    data_size = ((*ps_image).width * 3 as libc::c_int) as libc::c_ulong;
                }
                _ => {}
            }
            if use_mask != 0 {
                data_size = data_size
                    .wrapping_add(
                        (((*ps_image).width + 7 as libc::c_int) / 8 as libc::c_int)
                            as libc::c_ulong,
                    );
            }
            data_size = data_size.wrapping_mul((*ps_image).height as libc::c_ulong);
            data = (if data_size != 0 as libc::c_int as libc::c_ulong {
                malloc(data_size)
            } else {
                0 as *mut libc::c_void
            }) as *mut libc::c_uchar;
            if data.is_null() {
                status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
            } else {
                i = 0 as libc::c_int;
                y = 0 as libc::c_int;
                while y < (*ps_image).height {
                    if (*params).stencil_mask != 0 || use_mask != 0 {
                        if (*ps_image).format as libc::c_int
                            == CAIRO_FORMAT_A1 as libc::c_int
                        {
                            pixel8 = ((*ps_image).data)
                                .offset((y as libc::c_long * (*ps_image).stride) as isize)
                                as *mut uint8_t;
                            x = 0 as libc::c_int;
                            while x
                                < ((*ps_image).width + 7 as libc::c_int) / 8 as libc::c_int
                            {
                                a = *pixel8 as libc::c_int;
                                a = (((a as libc::c_ulong)
                                    .wrapping_mul(0x802 as libc::c_ulong)
                                    & 0x22110 as libc::c_ulong
                                    | (a as libc::c_ulong).wrapping_mul(0x8020 as libc::c_ulong)
                                        & 0x88440 as libc::c_ulong)
                                    .wrapping_mul(0x10101 as libc::c_ulong)
                                    >> 16 as libc::c_int) as libc::c_int;
                                let fresh30 = i;
                                i = i + 1;
                                *data.offset(fresh30 as isize) = a as libc::c_uchar;
                                x += 1;
                                pixel8 = pixel8.offset(1);
                            }
                        } else {
                            pixel8 = ((*ps_image).data)
                                .offset((y as libc::c_long * (*ps_image).stride) as isize)
                                as *mut uint8_t;
                            pixel32 = ((*ps_image).data)
                                .offset((y as libc::c_long * (*ps_image).stride) as isize)
                                as *mut uint32_t;
                            bit = 7 as libc::c_int;
                            x = 0 as libc::c_int;
                            while x < (*ps_image).width {
                                if (*ps_image).format as libc::c_int
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
                                    let fresh31 = i;
                                    i = i + 1;
                                    *data.offset(fresh31 as isize) = a as libc::c_uchar;
                                } else {
                                    if bit == 7 as libc::c_int {
                                        *data
                                            .offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if a != 0 as libc::c_int {
                                        let ref mut fresh32 = *data.offset(i as isize);
                                        *fresh32 = (*fresh32 as libc::c_int
                                            | (1 as libc::c_int) << bit) as libc::c_uchar;
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
                    }
                    if !((*params).stencil_mask != 0) {
                        pixel32 = ((*ps_image).data)
                            .offset((y as libc::c_long * (*ps_image).stride) as isize)
                            as *mut uint32_t;
                        bit = 7 as libc::c_int;
                        x = 0 as libc::c_int;
                        while x < (*ps_image).width {
                            let mut r: libc::c_int = 0;
                            let mut g: libc::c_int = 0;
                            let mut b: libc::c_int = 0;
                            if (*ps_image).format as libc::c_int
                                == CAIRO_FORMAT_ARGB32 as libc::c_int
                            {
                                if (*pixel32 & 0xff000000 as libc::c_uint)
                                    >> 24 as libc::c_int == 0 as libc::c_int as libc::c_uint
                                {
                                    b = 0 as libc::c_int;
                                    g = b;
                                    r = g;
                                } else {
                                    r = ((*pixel32 & 0xff0000 as libc::c_int as libc::c_uint)
                                        >> 16 as libc::c_int) as libc::c_int;
                                    g = ((*pixel32 & 0xff00 as libc::c_int as libc::c_uint)
                                        >> 8 as libc::c_int) as libc::c_int;
                                    b = ((*pixel32 & 0xff as libc::c_int as libc::c_uint)
                                        >> 0 as libc::c_int) as libc::c_int;
                                }
                            } else if (*ps_image).format as libc::c_int
                                == CAIRO_FORMAT_RGB24 as libc::c_int
                            {
                                r = ((*pixel32 & 0xff0000 as libc::c_int as libc::c_uint)
                                    >> 16 as libc::c_int) as libc::c_int;
                                g = ((*pixel32 & 0xff00 as libc::c_int as libc::c_uint)
                                    >> 8 as libc::c_int) as libc::c_int;
                                b = ((*pixel32 & 0xff as libc::c_int as libc::c_uint)
                                    >> 0 as libc::c_int) as libc::c_int;
                            } else {
                                b = 0 as libc::c_int;
                                g = b;
                                r = g;
                            }
                            match color as libc::c_uint {
                                0 | 3 => {
                                    let fresh33 = i;
                                    i = i + 1;
                                    *data.offset(fresh33 as isize) = r as libc::c_uchar;
                                    let fresh34 = i;
                                    i = i + 1;
                                    *data.offset(fresh34 as isize) = g as libc::c_uchar;
                                    let fresh35 = i;
                                    i = i + 1;
                                    *data.offset(fresh35 as isize) = b as libc::c_uchar;
                                }
                                1 => {
                                    let fresh36 = i;
                                    i = i + 1;
                                    *data.offset(fresh36 as isize) = r as libc::c_uchar;
                                }
                                2 => {
                                    if bit == 7 as libc::c_int {
                                        *data
                                            .offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if r != 0 as libc::c_int {
                                        let ref mut fresh37 = *data.offset(i as isize);
                                        *fresh37 = (*fresh37 as libc::c_int
                                            | (1 as libc::c_int) << bit) as libc::c_uchar;
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
                            pixel32 = pixel32.offset(1);
                        }
                        if bit != 7 as libc::c_int {
                            i += 1;
                        }
                    }
                    y += 1;
                }
                if (*surface).ps_level as libc::c_uint
                    == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
                {
                    compress = CAIRO_PS_COMPRESS_LZW;
                    compress_filter = b"LZWDecode\0" as *const u8 as *const libc::c_char;
                } else {
                    compress = CAIRO_PS_COMPRESS_DEFLATE;
                    compress_filter = b"FlateDecode\0" as *const u8
                        as *const libc::c_char;
                    (*surface).ps_level_used = CAIRO_PS_LEVEL_3;
                }
                if (*surface).paint_proc != 0 {
                    _cairo_output_stream_printf(
                        (*surface).stream,
                        b"/CairoData [\n\0" as *const u8 as *const libc::c_char,
                    );
                    status = _cairo_ps_surface_emit_base85_string(
                        surface,
                        data,
                        data_size,
                        compress,
                        1 as libc::c_int,
                    );
                    if status as u64 != 0 {
                        current_block = 16784384857398837809;
                    } else {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"] def\n\0" as *const u8 as *const libc::c_char,
                        );
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"/CairoDataIndex 0 def\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block = 8444733628337052024;
                    }
                } else {
                    _cairo_output_stream_printf(
                        (*surface).stream,
                        b"/cairo_ascii85_file currentfile /ASCII85Decode filter def\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    current_block = 8444733628337052024;
                }
                match current_block {
                    8444733628337052024 => {
                        if use_mask != 0 {
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"%s setcolorspace\n<<\n  /ImageType 3\n  /InterleaveType 2\n  /DataDict <<\n    /ImageType 1\n    /Width %d\n    /Height %d\n    /Interpolate %s\n    /BitsPerComponent %d\n    /Decode [ %s ]\n\0"
                                    as *const u8 as *const libc::c_char,
                                if color as libc::c_uint
                                    == CAIRO_IMAGE_IS_COLOR as libc::c_int as libc::c_uint
                                {
                                    b"/DeviceRGB\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"/DeviceGray\0" as *const u8 as *const libc::c_char
                                },
                                (*ps_image).width,
                                (*ps_image).height,
                                interpolate,
                                if color as libc::c_uint
                                    == CAIRO_IMAGE_IS_MONOCHROME as libc::c_int as libc::c_uint
                                {
                                    1 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                },
                                if color as libc::c_uint
                                    == CAIRO_IMAGE_IS_COLOR as libc::c_int as libc::c_uint
                                {
                                    b"0 1 0 1 0 1\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"0 1\0" as *const u8 as *const libc::c_char
                                },
                            );
                            if (*surface).paint_proc != 0 {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"    /DataSource { cairo_data_source } /%s filter\n\0"
                                        as *const u8 as *const libc::c_char,
                                    compress_filter,
                                );
                            } else {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"    /DataSource cairo_ascii85_file /%s filter\n\0"
                                        as *const u8 as *const libc::c_char,
                                    compress_filter,
                                );
                            }
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"    /ImageMatrix [ %d 0 0 %d 0 %d ]\n  >>\n  /MaskDict <<\n    /ImageType 1\n    /Width %d\n    /Height %d\n    /Interpolate %s\n    /BitsPerComponent 1\n    /Decode [ 1 0 ]\n    /ImageMatrix [ %d 0 0 %d 0 %d ]\n  >>\n>>\nimage\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*ps_image).width,
                                -(*ps_image).height,
                                (*ps_image).height,
                                (*ps_image).width,
                                (*ps_image).height,
                                interpolate,
                                (*ps_image).width,
                                -(*ps_image).height,
                                (*ps_image).height,
                            );
                        } else {
                            let mut decode: *const libc::c_char = 0
                                as *const libc::c_char;
                            if (*params).stencil_mask == 0 {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"%s setcolorspace\n\0" as *const u8 as *const libc::c_char,
                                    if color as libc::c_uint
                                        == CAIRO_IMAGE_IS_COLOR as libc::c_int as libc::c_uint
                                    {
                                        b"/DeviceRGB\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"/DeviceGray\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            if (*params).stencil_mask != 0 {
                                decode = b"1 0\0" as *const u8 as *const libc::c_char;
                            } else if color as libc::c_uint
                                == CAIRO_IMAGE_IS_COLOR as libc::c_int as libc::c_uint
                            {
                                decode = b"0 1 0 1 0 1\0" as *const u8
                                    as *const libc::c_char;
                            } else {
                                decode = b"0 1\0" as *const u8 as *const libc::c_char;
                            }
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"<<\n  /ImageType 1\n  /Width %d\n  /Height %d\n  /Interpolate %s\n  /BitsPerComponent %d\n  /Decode [ %s ]\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*ps_image).width,
                                (*ps_image).height,
                                interpolate,
                                if color as libc::c_uint
                                    == CAIRO_IMAGE_IS_MONOCHROME as libc::c_int as libc::c_uint
                                {
                                    1 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                },
                                decode,
                            );
                            if (*surface).paint_proc != 0 {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"  /DataSource { cairo_data_source } /%s filter\n\0"
                                        as *const u8 as *const libc::c_char,
                                    compress_filter,
                                );
                            } else {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"  /DataSource cairo_ascii85_file /%s filter\n\0"
                                        as *const u8 as *const libc::c_char,
                                    compress_filter,
                                );
                            }
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"  /ImageMatrix [ %d 0 0 %d 0 %d ]\n>>\n%s%s\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*ps_image).width,
                                -(*ps_image).height,
                                (*ps_image).height,
                                if (*surface).paint_proc != 0 {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"cairo_\0" as *const u8 as *const libc::c_char
                                },
                                if (*params).stencil_mask != 0 {
                                    b"imagemask\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"image\0" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                        if (*surface).paint_proc == 0 {
                            status = _cairo_ps_surface_emit_base85_string(
                                surface,
                                data,
                                data_size,
                                compress,
                                0 as libc::c_int,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"\n\0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            status = CAIRO_STATUS_SUCCESS;
                        }
                    }
                    _ => {}
                }
                free(data as *mut libc::c_void);
            }
            if use_mask == 0 && ps_image != image {
                cairo_surface_destroy(&mut (*ps_image).base);
            }
        }
        _ => {}
    }
    if image != image_surf {
        cairo_surface_destroy(&mut (*image).base);
    }
    _cairo_surface_release_source_image((*params).src_surface, image_surf, image_extra);
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_emit_jpeg_image(
    mut surface: *mut cairo_ps_surface_t,
    mut mode: cairo_emit_surface_mode_t,
    mut params: *mut cairo_emit_surface_params_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut mime_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut mime_data_length: libc::c_ulong = 0;
    let mut info: cairo_image_info_t = cairo_image_info_t {
        width: 0,
        height: 0,
        num_components: 0,
        bits_per_component: 0,
    };
    let mut colorspace: *const libc::c_char = 0 as *const libc::c_char;
    let mut decode: *const libc::c_char = 0 as *const libc::c_char;
    if (*(*params).src_surface).status as u64 != 0 {
        return (*(*params).src_surface).status as cairo_int_status_t;
    }
    cairo_surface_get_mime_data(
        (*params).src_surface,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        &mut mime_data,
        &mut mime_data_length,
    );
    if mime_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    status = _cairo_image_info_get_jpeg_info(&mut info, mime_data, mime_data_length)
        as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    match info.num_components {
        1 => {
            colorspace = b"/DeviceGray\0" as *const u8 as *const libc::c_char;
            decode = b"0 1\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            colorspace = b"/DeviceRGB\0" as *const u8 as *const libc::c_char;
            decode = b"0 1 0 1 0 1\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            colorspace = b"/DeviceCMYK\0" as *const u8 as *const libc::c_char;
            decode = b"0 1 0 1 0 1 0 1\0" as *const u8 as *const libc::c_char;
        }
        _ => return CAIRO_INT_STATUS_UNSUPPORTED,
    }
    if mode as libc::c_uint == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint
    {
        (*params).is_image = 1 as libc::c_int;
        (*params).approx_size = mime_data_length as libc::c_long;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if (*surface).paint_proc != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/CairoData [\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_ps_surface_emit_base85_string(
            surface,
            mime_data,
            mime_data_length,
            CAIRO_PS_COMPRESS_NONE,
            1 as libc::c_int,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).stream,
            b"] def\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/CairoDataIndex 0 def\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/cairo_ascii85_file currentfile /ASCII85Decode filter def\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"%s setcolorspace\n<<\n  /ImageType 1\n  /Width %d\n  /Height %d\n  /BitsPerComponent %d\n  /Interpolate %s\n  /Decode [ %s ]\n\0"
            as *const u8 as *const libc::c_char,
        colorspace,
        info.width,
        info.height,
        info.bits_per_component,
        get_interpolate((*params).filter),
        decode,
    );
    if (*surface).paint_proc != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"  /DataSource { cairo_data_source } /DCTDecode filter\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"  /DataSource cairo_ascii85_file /DCTDecode filter\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"  /ImageMatrix [ %d 0 0 %d 0 %d ]\n>>\n%simage\n\0" as *const u8
            as *const libc::c_char,
        info.width,
        -info.height,
        info.height,
        if (*surface).paint_proc != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"cairo_\0" as *const u8 as *const libc::c_char
        },
    );
    if (*surface).paint_proc == 0 {
        status = _cairo_ps_surface_emit_base85_string(
            surface,
            mime_data,
            mime_data_length,
            CAIRO_PS_COMPRESS_NONE,
            0 as libc::c_int,
        );
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_emit_ccitt_image(
    mut surface: *mut cairo_ps_surface_t,
    mut mode: cairo_emit_surface_mode_t,
    mut params: *mut cairo_emit_surface_params_t,
) -> cairo_int_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut ccitt_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ccitt_data_len: libc::c_ulong = 0;
    let mut ccitt_params_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ccitt_params_data_len: libc::c_ulong = 0;
    let mut ccitt_params_string: *mut libc::c_char = 0 as *mut libc::c_char;
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
    if (*(*params).src_surface).status as u64 != 0 {
        return (*(*params).src_surface).status as cairo_int_status_t;
    }
    cairo_surface_get_mime_data(
        (*params).src_surface,
        b"image/g3fax\0" as *const u8 as *const libc::c_char,
        &mut ccitt_data,
        &mut ccitt_data_len,
    );
    if ccitt_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    cairo_surface_get_mime_data(
        (*params).src_surface,
        b"application/x-cairo.ccitt.params\0" as *const u8 as *const libc::c_char,
        &mut ccitt_params_data,
        &mut ccitt_params_data_len,
    );
    if ccitt_params_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    ccitt_params_string = malloc(
        ccitt_params_data_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        ccitt_params_string as *mut libc::c_void,
        ccitt_params_data as *const libc::c_void,
        ccitt_params_data_len,
    );
    *ccitt_params_string
        .offset(ccitt_params_data_len as isize) = 0 as libc::c_int as libc::c_char;
    status = _cairo_tag_parse_ccitt_params(ccitt_params_string, &mut ccitt_params)
        as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    free(ccitt_params_string as *mut libc::c_void);
    if ccitt_params.columns <= 0 as libc::c_int || ccitt_params.rows <= 0 as libc::c_int
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if mode as libc::c_uint == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint
    {
        (*params).is_image = 1 as libc::c_int;
        (*params).approx_size = ccitt_data_len as libc::c_long;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if (*surface).paint_proc != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/CairoData [\n\0" as *const u8 as *const libc::c_char,
        );
        status = _cairo_ps_surface_emit_base85_string(
            surface,
            ccitt_data,
            ccitt_data_len,
            CAIRO_PS_COMPRESS_NONE,
            1 as libc::c_int,
        );
        if status as u64 != 0 {
            return status as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).stream,
            b"] def\n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/CairoDataIndex 0 def\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/cairo_ascii85_file currentfile /ASCII85Decode filter def\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*params).stencil_mask == 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"/DeviceGray setcolorspace\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"<<\n  /ImageType 1\n  /Width %d\n  /Height %d\n  /BitsPerComponent 1\n  /Interpolate %s\n  /Decode [ 0 1 ]\n\0"
            as *const u8 as *const libc::c_char,
        ccitt_params.columns,
        ccitt_params.rows,
        get_interpolate((*params).filter),
    );
    if (*surface).paint_proc != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"  /DataSource { cairo_data_source }\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"  /DataSource cairo_ascii85_file\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"  << /Columns %d /Rows %d /K %d\n\0" as *const u8 as *const libc::c_char,
        ccitt_params.columns,
        ccitt_params.rows,
        ccitt_params.k,
    );
    if ccitt_params.end_of_line != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"     /EndOfLine true\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if ccitt_params.encoded_byte_align != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"     /EncodedByteAlign true\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if ccitt_params.end_of_block == 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"     /EndOfBlock false\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if ccitt_params.black_is_1 != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"     /BlackIs1 true\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if ccitt_params.damaged_rows_before_error > 0 as libc::c_int {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"     /DamagedRowsBeforeError %d\n\0" as *const u8 as *const libc::c_char,
            ccitt_params.damaged_rows_before_error,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"  >> /CCITTFaxDecode filter\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"  /ImageMatrix [ %d 0 0 %d 0 %d ]\n>>\n%s%s\n\0" as *const u8
            as *const libc::c_char,
        ccitt_params.columns,
        -ccitt_params.rows,
        ccitt_params.rows,
        if (*surface).paint_proc != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"cairo_\0" as *const u8 as *const libc::c_char
        },
        if (*params).stencil_mask != 0 {
            b"imagemask\0" as *const u8 as *const libc::c_char
        } else {
            b"image\0" as *const u8 as *const libc::c_char
        },
    );
    if (*surface).paint_proc == 0 {
        status = _cairo_ps_surface_emit_base85_string(
            surface,
            ccitt_data,
            ccitt_data_len,
            CAIRO_PS_COMPRESS_NONE,
            0 as libc::c_int,
        );
    }
    return status as cairo_int_status_t;
}
unsafe extern "C" fn count_eod_strings(
    mut data: *const libc::c_uchar,
    mut data_len: libc::c_ulong,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = data;
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut first_char: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut eod_str: *const libc::c_char = b"|EOD|\0" as *const u8
        as *const libc::c_char;
    first_char = *eod_str.offset(0 as libc::c_int as isize) as libc::c_int;
    len = strlen(eod_str) as libc::c_int;
    p = data;
    end = data
        .offset(data_len as isize)
        .offset(-(len as isize))
        .offset(1 as libc::c_int as isize);
    count = 0 as libc::c_int;
    while p < end {
        p = memchr(
            p as *const libc::c_void,
            first_char,
            end.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_uchar;
        if p.is_null() {
            break;
        }
        if memcmp(
            p as *const libc::c_void,
            eod_str as *const libc::c_void,
            len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            count += 1;
            p = p.offset(len as isize);
        }
    }
    return count;
}
unsafe extern "C" fn _cairo_ps_surface_emit_eps(
    mut surface: *mut cairo_ps_surface_t,
    mut mode: cairo_emit_surface_mode_t,
    mut params: *mut cairo_emit_surface_params_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut eps_data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut eps_data_len: libc::c_ulong = 0;
    let mut eps_params_string: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut eps_params_string_len: libc::c_ulong = 0;
    let mut params_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eps_params: cairo_eps_params_t = cairo_eps_params_t {
        bbox: cairo_box_double_t {
            p1: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
            p2: cairo_point_double_t {
                x: 0.,
                y: 0.,
            },
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
    let mut eps_width: libc::c_double = 0.;
    let mut eps_height: libc::c_double = 0.;
    if (*(*params).src_surface).status as u64 != 0 {
        return (*(*params).src_surface).status;
    }
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_2 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    cairo_surface_get_mime_data(
        (*params).src_surface,
        b"application/postscript\0" as *const u8 as *const libc::c_char,
        &mut eps_data,
        &mut eps_data_len,
    );
    if eps_data.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    cairo_surface_get_mime_data(
        (*params).src_surface,
        b"application/x-cairo.eps.params\0" as *const u8 as *const libc::c_char,
        &mut eps_params_string,
        &mut eps_params_string_len,
    );
    if eps_params_string.is_null() {
        return CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as cairo_status_t;
    }
    params_string = malloc(
        eps_params_string_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        params_string as *mut libc::c_void,
        eps_params_string as *const libc::c_void,
        eps_params_string_len,
    );
    *params_string
        .offset(eps_params_string_len as isize) = 0 as libc::c_int as libc::c_char;
    status = _cairo_tag_parse_eps_params(params_string, &mut eps_params)
        as cairo_status_t;
    if status as u64 != 0 {
        return status;
    }
    if mode as libc::c_uint == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint
    {
        (*params).is_image = 0 as libc::c_int;
        (*params).approx_size = eps_data_len as libc::c_long;
        (*surface).contains_eps = 1 as libc::c_int;
        (*params).eod_count = count_eod_strings(eps_data, eps_data_len);
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).ps_level_used = CAIRO_PS_LEVEL_3;
    _cairo_output_stream_printf(
        (*surface).stream,
        b"cairo_eps_begin\n\0" as *const u8 as *const libc::c_char,
    );
    eps_width = eps_params.bbox.p2.x - eps_params.bbox.p1.x;
    eps_height = eps_params.bbox.p2.y - eps_params.bbox.p1.y;
    cairo_matrix_init_translate(
        &mut mat,
        (*(*params).src_surface_extents).x as libc::c_double,
        (*(*params).src_surface_extents).y as libc::c_double,
    );
    cairo_matrix_scale(
        &mut mat,
        (*(*params).src_surface_extents).width as libc::c_double / eps_width,
        (*(*params).src_surface_extents).height as libc::c_double / eps_height,
    );
    cairo_matrix_scale(
        &mut mat,
        1 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
    );
    cairo_matrix_translate(&mut mat, -eps_params.bbox.p1.x, -eps_params.bbox.p2.y);
    if _cairo_matrix_is_identity(&mut mat) == 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"[ \0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).stream, &mut mat);
        _cairo_output_stream_printf(
            (*surface).stream,
            b" ] concat\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"%f %f %f %f rectclip\n\0" as *const u8 as *const libc::c_char,
        eps_params.bbox.p1.x,
        eps_params.bbox.p1.y,
        eps_width,
        eps_height,
    );
    _cairo_output_stream_write(
        (*surface).stream,
        eps_data as *const libc::c_void,
        eps_data_len,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"\ncairo_eps_end\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_recording_surface(
    mut surface: *mut cairo_ps_surface_t,
    mut recording_surface: *mut cairo_surface_t,
    mut recording_extents: *const cairo_rectangle_int_t,
    mut subsurface: cairo_bool_t,
) -> cairo_status_t {
    let mut old_width: libc::c_double = 0.;
    let mut old_height: libc::c_double = 0.;
    let mut old_surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut old_surface_bounded: cairo_bool_t = 0;
    let mut old_cairo_to_ps: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut old_content: cairo_content_t = 0 as cairo_content_t;
    let mut old_clipper: cairo_surface_clipper_t = cairo_surface_clipper_t {
        clip: 0 as *mut cairo_clip_t,
        intersect_clip_path: None,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut free_me: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut id: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut recording_surf_stack_size: libc::c_int = 0;
    recording_surf_stack_size = _cairo_array_num_elements(
        &mut (*surface).recording_surf_stack,
    ) as libc::c_int;
    i = 0 as libc::c_int;
    while i < recording_surf_stack_size {
        _cairo_array_copy_element(
            &mut (*surface).recording_surf_stack,
            i as libc::c_uint,
            &mut id as *mut libc::c_uint as *mut libc::c_void,
        );
        if id == (*recording_surface).unique_id {
            return CAIRO_STATUS_SUCCESS;
        }
        i += 1;
    }
    id = (*recording_surface).unique_id;
    status = _cairo_array_append(
        &mut (*surface).recording_surf_stack,
        &mut id as *mut libc::c_uint as *const libc::c_void,
    ) as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    if _cairo_surface_is_snapshot(recording_surface) != 0 {
        recording_surface = _cairo_surface_snapshot_get_target(recording_surface);
        free_me = recording_surface;
    }
    old_content = (*surface).content;
    old_width = (*surface).width;
    old_height = (*surface).height;
    old_surface_extents = (*surface).surface_extents;
    old_surface_bounded = (*surface).surface_bounded;
    old_cairo_to_ps = (*surface).cairo_to_ps;
    old_clipper = (*surface).clipper;
    _cairo_surface_clipper_init(
        &mut (*surface).clipper,
        Some(
            _cairo_ps_surface_clipper_intersect_clip_path
                as unsafe extern "C" fn(
                    *mut cairo_surface_clipper_t,
                    *mut cairo_path_fixed_t,
                    cairo_fill_rule_t,
                    libc::c_double,
                    cairo_antialias_t,
                ) -> cairo_status_t,
        ),
    );
    (*surface).width = (*recording_extents).width as libc::c_double;
    (*surface).height = (*recording_extents).height as libc::c_double;
    (*surface).surface_extents = *recording_extents;
    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    cairo_matrix_init(
        &mut (*surface).cairo_to_ps,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    _cairo_pdf_operators_set_cairo_to_pdf_matrix(
        &mut (*surface).pdf_operators,
        &mut (*surface).cairo_to_ps,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"  q\n\0" as *const u8 as *const libc::c_char,
    );
    if (*recording_surface).content as libc::c_uint
        == CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
    {
        (*surface).content = CAIRO_CONTENT_COLOR;
        _cairo_output_stream_printf(
            (*surface).stream,
            b"  0 g %d %d %d %d rectfill\n\0" as *const u8 as *const libc::c_char,
            (*recording_extents).x,
            (*recording_extents).y,
            (*recording_extents).width,
            (*recording_extents).height,
        );
    }
    status = _cairo_recording_surface_replay_region(
        recording_surface,
        if subsurface != 0 {
            recording_extents
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
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            3412 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 142],
                &[libc::c_char; 142],
            >(
                b"cairo_status_t _cairo_ps_surface_emit_recording_surface(cairo_ps_surface_t *, cairo_surface_t *, const cairo_rectangle_int_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    if status as u64 != 0 {
        return status as cairo_status_t;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"  Q\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_surface_clipper_reset(&mut (*surface).clipper);
    (*surface).clipper = old_clipper;
    (*surface).content = old_content;
    (*surface).width = old_width;
    (*surface).height = old_height;
    (*surface).surface_extents = old_surface_extents;
    (*surface).surface_bounded = old_surface_bounded;
    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    (*surface).cairo_to_ps = old_cairo_to_ps;
    _cairo_pdf_operators_set_cairo_to_pdf_matrix(
        &mut (*surface).pdf_operators,
        &mut (*surface).cairo_to_ps,
    );
    cairo_surface_destroy(free_me);
    _cairo_array_truncate(
        &mut (*surface).recording_surf_stack,
        recording_surf_stack_size as libc::c_uint,
    );
    return status as cairo_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_flatten_transparency(
    mut surface: *mut cairo_ps_surface_t,
    mut color: *const cairo_color_t,
    mut red: *mut libc::c_double,
    mut green: *mut libc::c_double,
    mut blue: *mut libc::c_double,
) {
    *red = (*color).red;
    *green = (*color).green;
    *blue = (*color).blue;
    if !((*color).alpha_short as libc::c_int >= 0xff00 as libc::c_int) {
        *red *= (*color).alpha;
        *green *= (*color).alpha;
        *blue *= (*color).alpha;
        if (*surface).content as libc::c_uint
            == CAIRO_CONTENT_COLOR_ALPHA as libc::c_int as libc::c_uint
        {
            let mut one_minus_alpha: libc::c_double = 1.0f64 - (*color).alpha;
            *red += one_minus_alpha;
            *green += one_minus_alpha;
            *blue += one_minus_alpha;
        }
    }
}
unsafe extern "C" fn _cairo_ps_surface_emit_solid_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *mut cairo_solid_pattern_t,
) {
    let mut red: libc::c_double = 0.;
    let mut green: libc::c_double = 0.;
    let mut blue: libc::c_double = 0.;
    _cairo_ps_surface_flatten_transparency(
        surface,
        &mut (*pattern).color,
        &mut red,
        &mut green,
        &mut blue,
    );
    if color_is_gray(red, green, blue) != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%f g\n\0" as *const u8 as *const libc::c_char,
            red,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%f %f %f rg\n\0" as *const u8 as *const libc::c_char,
            red,
            green,
            blue,
        );
    };
}
unsafe extern "C" fn _cairo_ps_surface_use_form(
    mut surface: *mut cairo_ps_surface_t,
    mut params: *mut cairo_emit_surface_params_t,
    mut test: cairo_bool_t,
    mut ps_form: *mut *mut cairo_ps_form_t,
) -> cairo_int_status_t {
    let mut source_key: cairo_ps_form_t = cairo_ps_form_t {
        base: cairo_hash_entry_t { hash: 0 },
        unique_id: 0 as *mut libc::c_uchar,
        unique_id_length: 0,
        is_image: 0,
        id: 0,
        src_surface: 0 as *mut cairo_surface_t,
        src_surface_extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        src_surface_bounded: 0,
        filter: CAIRO_FILTER_FAST,
        required_extents: cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
    };
    let mut source_entry: *mut cairo_ps_form_t = 0 as *mut cairo_ps_form_t;
    let mut unique_id: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut unique_id_length: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut max_size: libc::c_long = 0;
    if (*params).op as libc::c_uint != CAIRO_OPERATOR_OVER as libc::c_int as libc::c_uint
        || (*params).stencil_mask != 0
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if (*(*(*params).src_surface).backend).type_0 as libc::c_uint
        == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    cairo_surface_get_mime_data(
        (*params).src_surface,
        b"application/x-cairo.uuid\0" as *const u8 as *const libc::c_char,
        &mut source_key.unique_id as *mut *mut libc::c_uchar
            as *mut *const libc::c_uchar,
        &mut source_key.unique_id_length,
    );
    if (source_key.unique_id).is_null()
        || source_key.unique_id_length == 0 as libc::c_int as libc::c_ulong
    {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    if test != 0 {
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    source_key.filter = (*params).filter;
    _cairo_ps_form_init_key(&mut source_key);
    source_entry = _cairo_hash_table_lookup((*surface).forms, &mut source_key.base)
        as *mut cairo_ps_form_t;
    if !source_entry.is_null() {
        _cairo_rectangle_union(
            &mut (*source_entry).required_extents,
            (*params).src_op_extents,
        );
        *ps_form = source_entry;
        return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
    }
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_3 as libc::c_int as libc::c_uint
    {
        max_size = (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
            as libc::c_long;
    } else {
        max_size = (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
            as libc::c_long;
    }
    if (*surface).total_form_size + (*params).approx_size > max_size {
        return CAIRO_INT_STATUS_UNSUPPORTED;
    }
    (*surface).total_form_size
        += ((*params).approx_size > max_size) as libc::c_int as libc::c_long;
    unique_id = (if source_key.unique_id_length != 0 as libc::c_int as libc::c_ulong {
        malloc(source_key.unique_id_length)
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_uchar;
    if unique_id.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
    }
    unique_id_length = source_key.unique_id_length;
    memcpy(
        unique_id as *mut libc::c_void,
        source_key.unique_id as *const libc::c_void,
        unique_id_length,
    );
    source_entry = calloc(
        ::std::mem::size_of::<cairo_ps_form_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut cairo_ps_form_t;
    if source_entry.is_null() {
        status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
    } else {
        (*source_entry).unique_id_length = unique_id_length;
        let ref mut fresh38 = (*source_entry).unique_id;
        *fresh38 = unique_id;
        let ref mut fresh39 = (*surface).num_forms;
        let fresh40 = *fresh39;
        *fresh39 = *fresh39 + 1;
        (*source_entry).id = fresh40;
        let ref mut fresh41 = (*source_entry).src_surface;
        *fresh41 = cairo_surface_reference((*params).src_surface);
        (*source_entry).src_surface_extents = *(*params).src_surface_extents;
        (*source_entry).src_surface_bounded = (*params).src_surface_bounded;
        (*source_entry).required_extents = *(*params).src_op_extents;
        (*source_entry).filter = (*params).filter;
        (*source_entry).is_image = (*params).is_image;
        _cairo_ps_form_init_key(source_entry);
        status = _cairo_hash_table_insert((*surface).forms, &mut (*source_entry).base);
        if !(status as u64 != 0) {
            *ps_form = source_entry;
            return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
        }
    }
    free(unique_id as *mut libc::c_void);
    free(source_entry as *mut libc::c_void);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_emit_form(
    mut surface: *mut cairo_ps_surface_t,
    mut params: *mut cairo_emit_surface_params_t,
    mut test: cairo_bool_t,
) -> cairo_int_status_t {
    let mut ps_form: *mut cairo_ps_form_t = 0 as *mut cairo_ps_form_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_ps_surface_use_form(surface, params, test, &mut ps_form)
        as cairo_status_t;
    if test != 0 || status as libc::c_uint != 0 {
        return status as cairo_int_status_t;
    }
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_3 as libc::c_int as libc::c_uint
    {
        (*surface).ps_level_used = CAIRO_PS_LEVEL_3;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"/cairoform-%d /Form findresource execform\n\0" as *const u8
            as *const libc::c_char,
        (*ps_form).id,
    );
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_emit_surface(
    mut surface: *mut cairo_ps_surface_t,
    mut mode: cairo_emit_surface_mode_t,
    mut params: *mut cairo_emit_surface_params_t,
) -> cairo_int_status_t {
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut old_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    let mut use_form: cairo_bool_t = 0;
    use_form = 0 as libc::c_int;
    if mode as libc::c_uint == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint
        || mode as libc::c_uint == CAIRO_EMIT_SURFACE_EMIT as libc::c_int as libc::c_uint
    {
        status = _cairo_ps_surface_emit_form(
            surface,
            params,
            (mode as libc::c_uint
                == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint)
                as libc::c_int,
        );
        use_form = (status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint) as libc::c_int;
        if status as libc::c_uint
            != CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            && status as libc::c_uint
                != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status;
        }
        if mode as libc::c_uint == CAIRO_EMIT_SURFACE_EMIT as libc::c_int as libc::c_uint
            && status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            return status;
        }
    }
    status = _cairo_ps_surface_emit_eps(surface, mode, params) as cairo_int_status_t;
    if status as libc::c_uint == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {
        (*params).is_image = 0 as libc::c_int;
    } else {
        if status as libc::c_uint
            != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
        {
            return status;
        }
        status = _cairo_ps_surface_emit_jpeg_image(surface, mode, params);
        if status as libc::c_uint
            == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            (*params).is_image = 1 as libc::c_int;
        } else {
            if status as libc::c_uint
                != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
            {
                return status;
            }
            status = _cairo_ps_surface_emit_ccitt_image(surface, mode, params);
            if status as libc::c_uint
                == CAIRO_INT_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                (*params).is_image = 1 as libc::c_int;
            } else {
                if status as libc::c_uint
                    != CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint
                {
                    return status;
                }
                if mode as libc::c_uint
                    == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint
                {
                    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                        as cairo_int_status_t;
                    if status as u64 != 0 {
                        return status;
                    }
                    old_stream = (*surface).stream;
                    let ref mut fresh42 = (*surface).stream;
                    *fresh42 = _cairo_memory_stream_create();
                    _cairo_pdf_operators_set_stream(
                        &mut (*surface).pdf_operators,
                        (*surface).stream,
                    );
                }
                if (*(*params).src_surface).type_0 as libc::c_uint
                    == CAIRO_SURFACE_TYPE_RECORDING as libc::c_int as libc::c_uint
                {
                    (*params).is_image = 0 as libc::c_int;
                    if (*(*(*params).src_surface).backend).type_0 as libc::c_uint
                        == CAIRO_SURFACE_TYPE_SUBSURFACE as libc::c_int as libc::c_uint
                    {
                        let mut sub: *mut cairo_surface_subsurface_t = (*params)
                            .src_surface as *mut cairo_surface_subsurface_t;
                        status = _cairo_ps_surface_emit_recording_surface(
                            surface,
                            (*sub).target,
                            &mut (*sub).extents,
                            1 as libc::c_int,
                        ) as cairo_int_status_t;
                    } else {
                        status = _cairo_ps_surface_emit_recording_surface(
                            surface,
                            (*params).src_surface,
                            (*params).src_op_extents,
                            0 as libc::c_int,
                        ) as cairo_int_status_t;
                    }
                } else {
                    (*params).is_image = 1 as libc::c_int;
                    status = _cairo_ps_surface_emit_image(surface, mode, params)
                        as cairo_int_status_t;
                }
                if mode as libc::c_uint
                    == CAIRO_EMIT_SURFACE_ANALYZE as libc::c_int as libc::c_uint
                {
                    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut length: libc::c_ulong = 0;
                    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                        as cairo_int_status_t;
                    if status as u64 != 0 {
                        return status;
                    }
                    status = _cairo_memory_stream_destroy(
                        (*surface).stream,
                        &mut data,
                        &mut length,
                    ) as cairo_int_status_t;
                    free(data as *mut libc::c_void);
                    if status as u64 != 0 {
                        return status;
                    }
                    (*params).approx_size = length as libc::c_long;
                    let ref mut fresh43 = (*surface).stream;
                    *fresh43 = old_stream;
                    _cairo_pdf_operators_set_stream(
                        &mut (*surface).pdf_operators,
                        (*surface).stream,
                    );
                }
            }
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_ps_form_emit(
    mut entry: *mut libc::c_void,
    mut closure: *mut libc::c_void,
) {
    let mut form: *mut cairo_ps_form_t = entry as *mut cairo_ps_form_t;
    let mut surface: *mut cairo_ps_surface_t = closure as *mut cairo_ps_surface_t;
    let mut params: cairo_emit_surface_params_t = cairo_emit_surface_params_t {
        src_surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        src_surface_extents: 0 as *const cairo_rectangle_int_t,
        src_surface_bounded: 0,
        src_op_extents: 0 as *const cairo_rectangle_int_t,
        filter: CAIRO_FILTER_FAST,
        stencil_mask: 0,
        is_image: 0,
        approx_size: 0,
        eod_count: 0,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
    let mut old_stream: *mut cairo_output_stream_t = 0 as *mut cairo_output_stream_t;
    params.src_surface = (*form).src_surface;
    params.op = CAIRO_OPERATOR_OVER;
    params.src_surface_extents = &mut (*form).src_surface_extents;
    params.src_surface_bounded = (*form).src_surface_bounded;
    params.src_op_extents = &mut (*form).required_extents;
    params.filter = (*form).filter;
    params.stencil_mask = 0 as libc::c_int;
    params.is_image = (*form).is_image;
    params.approx_size = 0 as libc::c_int as libc::c_long;
    params.eod_count = 0 as libc::c_int;
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%BeginResource: form cairoform-%d\n\0" as *const u8 as *const libc::c_char,
        (*form).id,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"/cairo_paint_form-%d\0" as *const u8 as *const libc::c_char,
        (*form).id,
    );
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_3 as libc::c_int as libc::c_uint
    {
        (*surface).paint_proc = 0 as libc::c_int;
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"\ncurrentfile\n<< /Filter /SubFileDecode\n   /DecodeParms << /EODString (%s) /EODCount 0 >>\n>> /ReusableStreamDecode filter\n\0"
                as *const u8 as *const libc::c_char,
            b"|EOD|\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*surface).paint_proc = 1 as libc::c_int;
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b" {\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"5 dict begin\n\0" as *const u8 as *const libc::c_char,
    );
    old_stream = (*surface).stream;
    let ref mut fresh44 = (*surface).stream;
    *fresh44 = (*surface).final_stream;
    _cairo_pdf_operators_set_stream(&mut (*surface).pdf_operators, (*surface).stream);
    status = _cairo_ps_surface_emit_surface(
        surface,
        CAIRO_EMIT_SURFACE_EMIT_FORM,
        &mut params,
    );
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
        as cairo_int_status_t;
    let ref mut fresh45 = (*surface).stream;
    *fresh45 = old_stream;
    _cairo_pdf_operators_set_stream(&mut (*surface).pdf_operators, (*surface).stream);
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"end\n\0" as *const u8 as *const libc::c_char,
    );
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_3 as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"%s\ndef\n\0" as *const u8 as *const libc::c_char,
            b"|EOD|\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"} bind def\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"\n/cairoform-%d\n<<\n  /FormType 1\n\0" as *const u8 as *const libc::c_char,
        (*form).id,
    );
    if (*form).is_image != 0 {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"  /BBox [ 0 0 1 1 ]\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b"  /BBox [ %d %d %d %d ]\n\0" as *const u8 as *const libc::c_char,
            (*form).required_extents.x,
            (*form).required_extents.y,
            (*form).required_extents.x + (*form).required_extents.width,
            (*form).required_extents.y + (*form).required_extents.height,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"  /Matrix [ 1 0 0 1 0 0 ]\n  /PaintProc { pop cairo_paint_form-%d\0"
            as *const u8 as *const libc::c_char,
        (*form).id,
    );
    if (*surface).ps_level as libc::c_uint
        == CAIRO_PS_LEVEL_3 as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).final_stream,
            b" dup 0 setfileposition cvx exec\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b" } bind\n>>\n/Form defineresource pop\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).final_stream,
        b"%%%%EndResource\n\0" as *const u8 as *const libc::c_char,
    );
    if status as u64 != 0 {
        (*surface).base.status = status as cairo_status_t;
    }
}
unsafe extern "C" fn _path_fixed_init_rectangle(
    mut path: *mut cairo_path_fixed_t,
    mut rect: *mut cairo_rectangle_int_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    _cairo_path_fixed_init(path);
    status = _cairo_path_fixed_move_to(
        path,
        _cairo_fixed_from_int((*rect).x),
        _cairo_fixed_from_int((*rect).y),
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            3846 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void _path_fixed_init_rectangle(cairo_path_fixed_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_path_fixed_rel_line_to(
        path,
        _cairo_fixed_from_int((*rect).width),
        _cairo_fixed_from_int(0 as libc::c_int),
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            3850 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void _path_fixed_init_rectangle(cairo_path_fixed_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_path_fixed_rel_line_to(
        path,
        _cairo_fixed_from_int(0 as libc::c_int),
        _cairo_fixed_from_int((*rect).height),
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            3854 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void _path_fixed_init_rectangle(cairo_path_fixed_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_path_fixed_rel_line_to(
        path,
        _cairo_fixed_from_int(-(*rect).width),
        _cairo_fixed_from_int(0 as libc::c_int),
    );
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            3858 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void _path_fixed_init_rectangle(cairo_path_fixed_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_path_fixed_close_path(path);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            3861 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void _path_fixed_init_rectangle(cairo_path_fixed_t *, cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn _cairo_ps_surface_paint_surface(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut op: cairo_operator_t,
    mut stencil_mask: cairo_bool_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut src_surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut src_surface_bounded: cairo_bool_t = 0;
    let mut src_op_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut source_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut x_offset: libc::c_double = 0.;
    let mut y_offset: libc::c_double = 0.;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut cairo_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut ps_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut path: cairo_path_fixed_t = cairo_path_fixed_t {
        last_move_point: cairo_point_t { x: 0, y: 0 },
        current_point: cairo_point_t { x: 0, y: 0 },
        has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [0; 1],
        c2rust_padding: [0; 3],
        extents: cairo_box_t {
            p1: cairo_point_t { x: 0, y: 0 },
            p2: cairo_point_t { x: 0, y: 0 },
        },
        buf: cairo_path_buf_fixed_t {
            base: cairo_path_buf_t {
                link: cairo_list_t {
                    next: 0 as *const _cairo_list as *mut _cairo_list,
                    prev: 0 as *const _cairo_list as *mut _cairo_list,
                },
                num_ops: 0,
                size_ops: 0,
                num_points: 0,
                size_points: 0,
                op: 0 as *mut cairo_path_op_t,
                points: 0 as *mut cairo_point_t,
            },
            op: [0; 27],
            points: [cairo_point_t { x: 0, y: 0 }; 54],
        },
    };
    let mut params: cairo_emit_surface_params_t = cairo_emit_surface_params_t {
        src_surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        src_surface_extents: 0 as *const cairo_rectangle_int_t,
        src_surface_bounded: 0,
        src_op_extents: 0 as *const cairo_rectangle_int_t,
        filter: CAIRO_FILTER_FAST,
        stencil_mask: 0,
        is_image: 0,
        approx_size: 0,
        eod_count: 0,
    };
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators);
    if status as u64 != 0 {
        return status;
    }
    status = _cairo_ps_surface_acquire_source_surface_from_pattern(
        surface,
        pattern,
        extents,
        &mut src_surface_extents,
        &mut src_surface_bounded,
        &mut src_op_extents,
        &mut source_surface,
        &mut x_offset,
        &mut y_offset,
    );
    if status as u64 != 0 {
        return status;
    }
    if (*pattern).extend as libc::c_uint
        == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        && (*pattern).type_0 as libc::c_uint
            == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
        && (*(*(pattern as *mut cairo_surface_pattern_t)).surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
    {
        let mut img: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
        img = source_surface as *mut cairo_image_surface_t;
        status = _cairo_ps_surface_create_padded_image_from_image(
            surface,
            img,
            &(*pattern).matrix,
            extents,
            &mut image,
            &mut src_surface_extents,
        );
        if status as u64 != 0 {
            current_block = 6689991402724200669;
        } else {
            x_offset = src_surface_extents.x as libc::c_double;
            y_offset = src_surface_extents.y as libc::c_double;
            current_block = 4956146061682418353;
        }
    } else {
        current_block = 4956146061682418353;
    }
    match current_block {
        4956146061682418353 => {
            _path_fixed_init_rectangle(&mut path, extents);
            status = _cairo_pdf_operators_clip(
                &mut (*surface).pdf_operators,
                &mut path,
                CAIRO_FILL_RULE_WINDING,
            ) as cairo_status_t;
            _cairo_path_fixed_fini(&mut path);
            if !(status as u64 != 0) {
                cairo_p2d = (*pattern).matrix;
                if (*surface).paginated_mode as libc::c_uint
                    == CAIRO_PAGINATED_MODE_FALLBACK as libc::c_int as libc::c_uint
                {
                    let mut x_scale: libc::c_double = cairo_p2d.xx;
                    let mut y_scale: libc::c_double = cairo_p2d.yy;
                    _cairo_output_stream_printf(
                        (*surface).stream,
                        b"%% Fallback Image: x=%f y=%f w=%d h=%d \0" as *const u8
                            as *const libc::c_char,
                        -cairo_p2d.x0 / x_scale,
                        -cairo_p2d.y0 / y_scale,
                        (src_surface_extents.width as libc::c_double / x_scale)
                            as libc::c_int,
                        (src_surface_extents.height as libc::c_double / y_scale)
                            as libc::c_int,
                    );
                    if x_scale == y_scale {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"res=%fppi \0" as *const u8 as *const libc::c_char,
                            x_scale * 72 as libc::c_int as libc::c_double,
                        );
                    } else {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"res=%fx%fppi \0" as *const u8 as *const libc::c_char,
                            x_scale * 72 as libc::c_int as libc::c_double,
                            y_scale * 72 as libc::c_int as libc::c_double,
                        );
                    }
                    _cairo_output_stream_printf(
                        (*surface).stream,
                        b"size=%ld\n\0" as *const u8 as *const libc::c_char,
                        src_surface_extents.width as libc::c_long
                            * src_surface_extents.height as libc::c_long
                            * 3 as libc::c_int as libc::c_long,
                    );
                } else if op as libc::c_uint
                    == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
                {
                    _cairo_output_stream_printf(
                        (*surface).stream,
                        b"%d g %d %d %d %d rectfill\n\0" as *const u8
                            as *const libc::c_char,
                        if (*surface).content as libc::c_uint
                            == CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
                        {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                        (*surface).surface_extents.x,
                        (*surface).surface_extents.y,
                        (*surface).surface_extents.width,
                        (*surface).surface_extents.height,
                    );
                }
                status = cairo_matrix_invert(&mut cairo_p2d);
                if status as libc::c_uint
                    == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                            as *const libc::c_char,
                        b"../src/cairo-ps-surface.c\0" as *const u8
                            as *const libc::c_char,
                        3964 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 151],
                            &[libc::c_char; 151],
                        >(
                            b"cairo_status_t _cairo_ps_surface_paint_surface(cairo_ps_surface_t *, const cairo_pattern_t *, cairo_rectangle_int_t *, cairo_operator_t, cairo_bool_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                ps_p2d = (*surface).cairo_to_ps;
                cairo_matrix_multiply(&mut ps_p2d, &mut cairo_p2d, &mut ps_p2d);
                cairo_matrix_translate(&mut ps_p2d, x_offset, y_offset);
                params
                    .src_surface = if !image.is_null() {
                    &mut (*image).base
                } else {
                    source_surface
                };
                params.op = op;
                params.src_surface_extents = &mut src_surface_extents;
                params.src_surface_bounded = src_surface_bounded;
                params.src_op_extents = &mut src_op_extents;
                params.filter = (*pattern).filter;
                params.stencil_mask = stencil_mask;
                params.is_image = 0 as libc::c_int;
                params.approx_size = 0 as libc::c_int as libc::c_long;
                status = _cairo_ps_surface_emit_surface(
                    surface,
                    CAIRO_EMIT_SURFACE_ANALYZE,
                    &mut params,
                ) as cairo_status_t;
                if !(status as u64 != 0) {
                    if params.is_image != 0 {
                        cairo_matrix_translate(
                            &mut ps_p2d,
                            0.0f64,
                            src_surface_extents.height as libc::c_double,
                        );
                        cairo_matrix_scale(&mut ps_p2d, 1.0f64, -1.0f64);
                        cairo_matrix_scale(
                            &mut ps_p2d,
                            src_surface_extents.width as libc::c_double,
                            src_surface_extents.height as libc::c_double,
                        );
                    }
                    if _cairo_matrix_is_identity(&mut ps_p2d) == 0 {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"[ \0" as *const u8 as *const libc::c_char,
                        );
                        _cairo_output_stream_print_matrix(
                            (*surface).stream,
                            &mut ps_p2d,
                        );
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b" ] concat\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    status = _cairo_ps_surface_emit_surface(
                        surface,
                        CAIRO_EMIT_SURFACE_EMIT,
                        &mut params,
                    ) as cairo_status_t;
                }
            }
        }
        _ => {}
    }
    if !image.is_null() {
        cairo_surface_destroy(&mut (*image).base);
    }
    _cairo_ps_surface_release_source_surface_from_pattern(
        surface,
        pattern,
        source_surface,
    );
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_emit_surface_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *mut cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut op: cairo_operator_t,
) -> cairo_status_t {
    let mut current_block: u64;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    let mut xstep: libc::c_double = 0.;
    let mut ystep: libc::c_double = 0.;
    let mut pattern_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut bounded: cairo_bool_t = 0;
    let mut cairo_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut ps_p2d: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut old_paint_proc: cairo_bool_t = 0;
    let mut x_offset: libc::c_double = 0.;
    let mut y_offset: libc::c_double = 0.;
    let mut source_surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut image: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
    let mut src_op_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut params: cairo_emit_surface_params_t = cairo_emit_surface_params_t {
        src_surface: 0 as *mut cairo_surface_t,
        op: CAIRO_OPERATOR_CLEAR,
        src_surface_extents: 0 as *const cairo_rectangle_int_t,
        src_surface_bounded: 0,
        src_op_extents: 0 as *const cairo_rectangle_int_t,
        filter: CAIRO_FILTER_FAST,
        stencil_mask: 0,
        is_image: 0,
        approx_size: 0,
        eod_count: 0,
    };
    let mut extend: cairo_extend_t = cairo_pattern_get_extend(pattern);
    cairo_p2d = (*pattern).matrix;
    status = cairo_matrix_invert(&mut cairo_p2d);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            4029 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"cairo_status_t _cairo_ps_surface_emit_surface_pattern(cairo_ps_surface_t *, cairo_pattern_t *, cairo_rectangle_int_t *, cairo_operator_t)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_ps_surface_acquire_source_surface_from_pattern(
        surface,
        pattern,
        extents,
        &mut pattern_extents,
        &mut bounded,
        &mut src_op_extents,
        &mut source_surface,
        &mut x_offset,
        &mut y_offset,
    );
    if status as u64 != 0 {
        return status;
    }
    if extend as libc::c_uint == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint {
        let mut img: *mut cairo_image_surface_t = 0 as *mut cairo_image_surface_t;
        if (*source_surface).type_0 as libc::c_uint
            == CAIRO_SURFACE_TYPE_IMAGE as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"source_surface->type == CAIRO_SURFACE_TYPE_IMAGE\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                4045 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 138],
                    &[libc::c_char; 138],
                >(
                    b"cairo_status_t _cairo_ps_surface_emit_surface_pattern(cairo_ps_surface_t *, cairo_pattern_t *, cairo_rectangle_int_t *, cairo_operator_t)\0",
                ))
                    .as_ptr(),
            );
        }
        img = source_surface as *mut cairo_image_surface_t;
        status = _cairo_ps_surface_create_padded_image_from_image(
            surface,
            img,
            &mut (*pattern).matrix,
            extents,
            &mut image,
            &mut pattern_extents,
        );
        if status as u64 != 0 {
            current_block = 1650126163338910239;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    match current_block {
        17833034027772472439 => {
            if !(status as u64 != 0) {
                if bounded == 0 {
                    extend = CAIRO_EXTEND_NONE;
                    _cairo_rectangle_intersect(
                        &mut pattern_extents,
                        &mut src_op_extents,
                    );
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
                        xstep = (pattern_extents.width * 2 as libc::c_int)
                            as libc::c_double;
                        ystep = (pattern_extents.height * 2 as libc::c_int)
                            as libc::c_double;
                    }
                    _ => {
                        if (b"reached\0" as *const u8 as *const libc::c_char).is_null()
                        {} else {
                            __assert_fail(
                                b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                                b"../src/cairo-ps-surface.c\0" as *const u8
                                    as *const libc::c_char,
                                4107 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 138],
                                    &[libc::c_char; 138],
                                >(
                                    b"cairo_status_t _cairo_ps_surface_emit_surface_pattern(cairo_ps_surface_t *, cairo_pattern_t *, cairo_rectangle_int_t *, cairo_operator_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        xstep = 0 as libc::c_int as libc::c_double;
                        ystep = 0 as libc::c_int as libc::c_double;
                    }
                }
                _cairo_output_stream_printf(
                    (*surface).stream,
                    b"/CairoPattern {\nq %d %d %d %d rectclip\n\0" as *const u8
                        as *const libc::c_char,
                    pattern_extents.x,
                    pattern_extents.y,
                    pattern_extents.width,
                    pattern_extents.height,
                );
                if extend as libc::c_uint
                    == CAIRO_EXTEND_REPEAT as libc::c_int as libc::c_uint
                    || extend as libc::c_uint
                        == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
                {
                    src_op_extents = pattern_extents;
                }
                old_paint_proc = (*surface).paint_proc;
                (*surface).paint_proc = 1 as libc::c_int;
                params
                    .src_surface = if !image.is_null() {
                    &mut (*image).base
                } else {
                    source_surface
                };
                params.op = op;
                params.src_surface_extents = &mut pattern_extents;
                params.src_surface_bounded = bounded;
                params.src_op_extents = &mut src_op_extents;
                params.filter = (*pattern).filter;
                params.stencil_mask = 0 as libc::c_int;
                params.is_image = 0 as libc::c_int;
                params.approx_size = 0 as libc::c_int as libc::c_long;
                status = _cairo_ps_surface_emit_surface(
                    surface,
                    CAIRO_EMIT_SURFACE_ANALYZE,
                    &mut params,
                ) as cairo_status_t;
                if !(status as u64 != 0) {
                    if params.is_image != 0 {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"[ %d 0 0 %d 0 0 ] concat\n\0" as *const u8
                                as *const libc::c_char,
                            pattern_extents.width,
                            pattern_extents.height,
                        );
                    }
                    if op as libc::c_uint
                        == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
                    {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"%d g %d %d %f %f rectfill\n\0" as *const u8
                                as *const libc::c_char,
                            if (*surface).content as libc::c_uint
                                == CAIRO_CONTENT_COLOR as libc::c_int as libc::c_uint
                            {
                                0 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                            pattern_extents.x,
                            pattern_extents.y,
                            xstep,
                            ystep,
                        );
                    }
                    status = _cairo_ps_surface_emit_surface(
                        surface,
                        CAIRO_EMIT_SURFACE_EMIT,
                        &mut params,
                    ) as cairo_status_t;
                    if !(status as u64 != 0) {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b" Q } bind def\n\0" as *const u8 as *const libc::c_char,
                        );
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"<< /PatternType 1\n   /PaintType 1\n   /TilingType 1\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"   /XStep %f /YStep %f\n\0" as *const u8
                                as *const libc::c_char,
                            xstep,
                            ystep,
                        );
                        if extend as libc::c_uint
                            == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
                        {
                            let mut mat: cairo_matrix_t = cairo_matrix_t {
                                xx: 0.,
                                yx: 0.,
                                xy: 0.,
                                yy: 0.,
                                x0: 0.,
                                y0: 0.,
                            };
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"   /BBox [%d %d %d %d]\n   /PaintProc {\n      pop CairoPattern\n\0"
                                    as *const u8 as *const libc::c_char,
                                pattern_extents.x,
                                pattern_extents.y,
                                pattern_extents.x
                                    + pattern_extents.width * 2 as libc::c_int,
                                pattern_extents.y
                                    + pattern_extents.height * 2 as libc::c_int,
                            );
                            cairo_matrix_init_translate(
                                &mut mat,
                                pattern_extents.x as libc::c_double,
                                pattern_extents.y as libc::c_double,
                            );
                            cairo_matrix_scale(
                                &mut mat,
                                -(1 as libc::c_int) as libc::c_double,
                                1 as libc::c_int as libc::c_double,
                            );
                            cairo_matrix_translate(
                                &mut mat,
                                (-(2 as libc::c_int) * pattern_extents.width)
                                    as libc::c_double,
                                0 as libc::c_int as libc::c_double,
                            );
                            cairo_matrix_translate(
                                &mut mat,
                                -pattern_extents.x as libc::c_double,
                                -pattern_extents.y as libc::c_double,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"      q [\0" as *const u8 as *const libc::c_char,
                            );
                            _cairo_output_stream_print_matrix(
                                (*surface).stream,
                                &mut mat,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"] concat CairoPattern Q\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            cairo_matrix_init_translate(
                                &mut mat,
                                pattern_extents.x as libc::c_double,
                                pattern_extents.y as libc::c_double,
                            );
                            cairo_matrix_scale(
                                &mut mat,
                                1 as libc::c_int as libc::c_double,
                                -(1 as libc::c_int) as libc::c_double,
                            );
                            cairo_matrix_translate(
                                &mut mat,
                                0 as libc::c_int as libc::c_double,
                                (-(2 as libc::c_int) * pattern_extents.height)
                                    as libc::c_double,
                            );
                            cairo_matrix_translate(
                                &mut mat,
                                -pattern_extents.x as libc::c_double,
                                -pattern_extents.y as libc::c_double,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"      q [\0" as *const u8 as *const libc::c_char,
                            );
                            _cairo_output_stream_print_matrix(
                                (*surface).stream,
                                &mut mat,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"] concat CairoPattern Q\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            cairo_matrix_init_translate(
                                &mut mat,
                                pattern_extents.x as libc::c_double,
                                pattern_extents.y as libc::c_double,
                            );
                            cairo_matrix_scale(
                                &mut mat,
                                -(1 as libc::c_int) as libc::c_double,
                                -(1 as libc::c_int) as libc::c_double,
                            );
                            cairo_matrix_translate(
                                &mut mat,
                                (-(2 as libc::c_int) * pattern_extents.width)
                                    as libc::c_double,
                                (-(2 as libc::c_int) * pattern_extents.height)
                                    as libc::c_double,
                            );
                            cairo_matrix_translate(
                                &mut mat,
                                -pattern_extents.x as libc::c_double,
                                -pattern_extents.y as libc::c_double,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"      q [\0" as *const u8 as *const libc::c_char,
                            );
                            _cairo_output_stream_print_matrix(
                                (*surface).stream,
                                &mut mat,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"] concat CairoPattern Q\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"   } bind\n\0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            if op as libc::c_uint
                                == CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint
                            {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"   /BBox [0 0 %f %f]\n\0" as *const u8
                                        as *const libc::c_char,
                                    xstep,
                                    ystep,
                                );
                            } else {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"   /BBox [%d %d %d %d]\n\0" as *const u8
                                        as *const libc::c_char,
                                    pattern_extents.x,
                                    pattern_extents.y,
                                    pattern_extents.x + pattern_extents.width,
                                    pattern_extents.y + pattern_extents.height,
                                );
                            }
                            _cairo_output_stream_printf(
                                (*surface).stream,
                                b"   /PaintProc { pop CairoPattern }\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b">>\n\0" as *const u8 as *const libc::c_char,
                        );
                        cairo_p2d = (*pattern).matrix;
                        status = cairo_matrix_invert(&mut cairo_p2d);
                        if status as libc::c_uint
                            == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"status == CAIRO_STATUS_SUCCESS\0" as *const u8
                                    as *const libc::c_char,
                                b"../src/cairo-ps-surface.c\0" as *const u8
                                    as *const libc::c_char,
                                4225 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 138],
                                    &[libc::c_char; 138],
                                >(
                                    b"cairo_status_t _cairo_ps_surface_emit_surface_pattern(cairo_ps_surface_t *, cairo_pattern_t *, cairo_rectangle_int_t *, cairo_operator_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        cairo_matrix_init_identity(&mut ps_p2d);
                        cairo_matrix_multiply(&mut ps_p2d, &mut cairo_p2d, &mut ps_p2d);
                        cairo_matrix_translate(&mut ps_p2d, x_offset, y_offset);
                        if (*(*(pattern as *mut cairo_surface_pattern_t)).surface).type_0
                            as libc::c_uint
                            != CAIRO_SURFACE_TYPE_RECORDING as libc::c_int
                                as libc::c_uint
                        {
                            cairo_matrix_translate(
                                &mut ps_p2d,
                                0.0f64,
                                pattern_extents.height as libc::c_double,
                            );
                            cairo_matrix_scale(&mut ps_p2d, 1.0f64, -1.0f64);
                        }
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"[ \0" as *const u8 as *const libc::c_char,
                        );
                        _cairo_output_stream_print_matrix(
                            (*surface).stream,
                            &mut ps_p2d,
                        );
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b" ]\nmakepattern setpattern\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        (*surface).paint_proc = old_paint_proc;
                    }
                }
            }
        }
        _ => {}
    }
    if !image.is_null() {
        cairo_surface_destroy(&mut (*image).base);
    }
    _cairo_ps_surface_release_source_surface_from_pattern(
        surface,
        pattern,
        source_surface,
    );
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_emit_linear_colorgradient(
    mut surface: *mut cairo_ps_surface_t,
    mut stop1: *mut cairo_ps_color_stop_t,
    mut stop2: *mut cairo_ps_color_stop_t,
) {
    _cairo_output_stream_printf(
        (*surface).stream,
        b"   << /FunctionType 2\n      /Domain [ 0 1 ]\n      /C0 [ %f %f %f ]\n      /C1 [ %f %f %f ]\n      /N 1\n   >>\n\0"
            as *const u8 as *const libc::c_char,
        (*stop1).color[0 as libc::c_int as usize],
        (*stop1).color[1 as libc::c_int as usize],
        (*stop1).color[2 as libc::c_int as usize],
        (*stop2).color[0 as libc::c_int as usize],
        (*stop2).color[1 as libc::c_int as usize],
        (*stop2).color[2 as libc::c_int as usize],
    );
}
unsafe extern "C" fn _cairo_ps_surface_emit_stitched_colorgradient(
    mut surface: *mut cairo_ps_surface_t,
    mut n_stops: libc::c_uint,
    mut stops: *mut cairo_ps_color_stop_t,
) {
    let mut i: libc::c_uint = 0;
    _cairo_output_stream_printf(
        (*surface).stream,
        b"<< /FunctionType 3\n   /Domain [ 0 1 ]\n   /Functions [\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        _cairo_ps_surface_emit_linear_colorgradient(
            surface,
            &mut *stops.offset(i as isize),
            &mut *stops.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"   ]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"   /Bounds [ \0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%f \0" as *const u8 as *const libc::c_char,
            (*stops.offset(i as isize)).offset,
        );
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"   /Encode [ 1 1 %d { pop 0 1 } for ]\n\0" as *const u8 as *const libc::c_char,
        n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b">>\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn calc_gradient_color(
    mut new_stop: *mut cairo_ps_color_stop_t,
    mut stop1: *mut cairo_ps_color_stop_t,
    mut stop2: *mut cairo_ps_color_stop_t,
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
unsafe extern "C" fn _cairo_ps_surface_emit_pattern_stops(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *mut cairo_gradient_pattern_t,
) -> cairo_status_t {
    let mut allstops: *mut cairo_ps_color_stop_t = 0 as *mut cairo_ps_color_stop_t;
    let mut stops: *mut cairo_ps_color_stop_t = 0 as *mut cairo_ps_color_stop_t;
    let mut i: libc::c_uint = 0;
    let mut n_stops: libc::c_uint = 0;
    allstops = _cairo_malloc_ab(
        ((*pattern).n_stops).wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<cairo_ps_color_stop_t>() as libc::c_ulong,
    ) as *mut cairo_ps_color_stop_t;
    if allstops.is_null() {
        return _cairo_error(CAIRO_STATUS_NO_MEMORY);
    }
    stops = &mut *allstops.offset(1 as libc::c_int as isize)
        as *mut cairo_ps_color_stop_t;
    n_stops = (*pattern).n_stops;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n_stops {
        let mut stop: *mut cairo_gradient_stop_t = &mut *((*pattern).stops)
            .offset(i as isize) as *mut cairo_gradient_stop_t;
        (*stops.offset(i as isize)).color[0 as libc::c_int as usize] = (*stop).color.red;
        (*stops.offset(i as isize))
            .color[1 as libc::c_int as usize] = (*stop).color.green;
        (*stops.offset(i as isize))
            .color[2 as libc::c_int as usize] = (*stop).color.blue;
        (*stops.offset(i as isize))
            .color[3 as libc::c_int as usize] = (*stop).color.alpha;
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
                    ::std::mem::size_of::<cairo_ps_color_stop_t>() as libc::c_ulong,
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
                    &mut *stops.offset(n_stops as isize) as *mut cairo_ps_color_stop_t
                        as *mut libc::c_void,
                    &mut *stops
                        .offset(
                            n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) as *mut cairo_ps_color_stop_t as *const libc::c_void,
                    ::std::mem::size_of::<cairo_ps_color_stop_t>() as libc::c_ulong,
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
    i = 0 as libc::c_int as libc::c_uint;
    while i < n_stops {
        let mut red: libc::c_double = 0.;
        let mut green: libc::c_double = 0.;
        let mut blue: libc::c_double = 0.;
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
        _cairo_color_init_rgba(
            &mut color,
            (*stops.offset(i as isize)).color[0 as libc::c_int as usize],
            (*stops.offset(i as isize)).color[1 as libc::c_int as usize],
            (*stops.offset(i as isize)).color[2 as libc::c_int as usize],
            (*stops.offset(i as isize)).color[3 as libc::c_int as usize],
        );
        _cairo_ps_surface_flatten_transparency(
            surface,
            &mut color,
            &mut red,
            &mut green,
            &mut blue,
        );
        (*stops.offset(i as isize)).color[0 as libc::c_int as usize] = red;
        (*stops.offset(i as isize)).color[1 as libc::c_int as usize] = green;
        (*stops.offset(i as isize)).color[2 as libc::c_int as usize] = blue;
        i = i.wrapping_add(1);
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"/CairoFunction\n\0" as *const u8 as *const libc::c_char,
    );
    if (*stops.offset(0 as libc::c_int as isize)).offset
        == (*stops
            .offset(n_stops.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .offset
    {
        let mut pad_stops: [cairo_ps_color_stop_t; 4] = [cairo_ps_color_stop_t {
            offset: 0.,
            color: [0.; 4],
        }; 4];
        if (*pattern).base.extend as libc::c_uint
            == CAIRO_EXTEND_PAD as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"pattern->base.extend == CAIRO_EXTEND_PAD\0" as *const u8
                    as *const libc::c_char,
                b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                4405 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 102],
                    &[libc::c_char; 102],
                >(
                    b"cairo_status_t _cairo_ps_surface_emit_pattern_stops(cairo_ps_surface_t *, cairo_gradient_pattern_t *)\0",
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
        _cairo_ps_surface_emit_stitched_colorgradient(
            surface,
            4 as libc::c_int as libc::c_uint,
            pad_stops.as_mut_ptr(),
        );
    } else if n_stops == 2 as libc::c_int as libc::c_uint {
        _cairo_ps_surface_emit_linear_colorgradient(
            surface,
            &mut *stops.offset(0 as libc::c_int as isize),
            &mut *stops.offset(1 as libc::c_int as isize),
        );
    } else {
        _cairo_ps_surface_emit_stitched_colorgradient(surface, n_stops, stops);
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"def\n\0" as *const u8 as *const libc::c_char,
    );
    free(allstops as *mut libc::c_void);
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_repeating_function(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *mut cairo_gradient_pattern_t,
    mut begin: libc::c_int,
    mut end: libc::c_int,
) -> cairo_status_t {
    _cairo_output_stream_printf(
        (*surface).stream,
        b"/CairoFunction\n<< /FunctionType 3\n   /Domain [ %d %d ]\n   /Functions [ %d {CairoFunction} repeat ]\n   /Bounds [ %d 1 %d {} for ]\n\0"
            as *const u8 as *const libc::c_char,
        begin,
        end,
        end - begin,
        begin + 1 as libc::c_int,
        end - 1 as libc::c_int,
    );
    if (*pattern).base.extend as libc::c_uint
        == CAIRO_EXTEND_REFLECT as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"   /Encode [ %d 1 %d { 2 mod 0 eq {0 1} {1 0} ifelse } for ]\n\0"
                as *const u8 as *const libc::c_char,
            begin,
            end - 1 as libc::c_int,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"   /Encode [ %d 1 %d { pop 0 1 } for ]\n\0" as *const u8
                as *const libc::c_char,
            begin,
            end - 1 as libc::c_int,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b">> def\n\0" as *const u8 as *const libc::c_char,
    );
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_emit_gradient(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *mut cairo_gradient_pattern_t,
    mut is_ps_pattern: cairo_bool_t,
) -> cairo_status_t {
    let mut pat_to_ps: cairo_matrix_t = cairo_matrix_t {
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
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pattern).n_stops != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"pattern->n_stops != 0\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            4473 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"cairo_status_t _cairo_ps_surface_emit_gradient(cairo_ps_surface_t *, cairo_gradient_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    status = _cairo_ps_surface_emit_pattern_stops(surface, pattern);
    if status as u64 != 0 {
        return status;
    }
    pat_to_ps = (*pattern).base.matrix;
    status = cairo_matrix_invert(&mut pat_to_ps);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            4482 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"cairo_status_t _cairo_ps_surface_emit_gradient(cairo_ps_surface_t *, cairo_gradient_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(&mut pat_to_ps, &mut pat_to_ps, &mut (*surface).cairo_to_ps);
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
                b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                4526 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 111],
                    &[libc::c_char; 111],
                >(
                    b"cairo_status_t _cairo_ps_surface_emit_gradient(cairo_ps_surface_t *, cairo_gradient_pattern_t *, cairo_bool_t)\0",
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
        status = _cairo_ps_surface_emit_repeating_function(
            surface,
            pattern,
            repeat_begin,
            repeat_end,
        );
        if status as u64 != 0 {
            return status;
        }
    } else if (*pattern).n_stops <= 2 as libc::c_int as libc::c_uint {
        domain[0 as libc::c_int as usize] = 0.0f64;
        domain[1 as libc::c_int as usize] = 1.0f64;
    }
    if is_ps_pattern != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"<< /PatternType 2\n   /Shading\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pattern).base.type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"   << /ShadingType 2\n      /ColorSpace /DeviceRGB\n      /Coords [ %f %f %f %f ]\n\0"
                as *const u8 as *const libc::c_char,
            start.center.x,
            start.center.y,
            end.center.x,
            end.center.y,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"   << /ShadingType 3\n      /ColorSpace /DeviceRGB\n      /Coords [ %f %f %f %f %f %f ]\n\0"
                as *const u8 as *const libc::c_char,
            start.center.x,
            start.center.y,
            if start.radius > 0 as libc::c_int as libc::c_double {
                start.radius
            } else {
                0 as libc::c_int as libc::c_double
            },
            end.center.x,
            end.center.y,
            if end.radius > 0 as libc::c_int as libc::c_double {
                end.radius
            } else {
                0 as libc::c_int as libc::c_double
            },
        );
    }
    if (*pattern).base.extend as libc::c_uint
        != CAIRO_EXTEND_NONE as libc::c_int as libc::c_uint
    {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"      /Extend [ true true ]\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"      /Extend [ false false ]\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if domain[0 as libc::c_int as usize] == 0.0f64
        && domain[1 as libc::c_int as usize] == 1.0f64
    {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"      /Function CairoFunction\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"      /Function <<\n         /FunctionType 3\n         /Domain [ 0 1 ]\n         /Bounds [ ]\n         /Encode [ %f %f ]\n         /Functions [ CairoFunction ]\n      >>\n\0"
                as *const u8 as *const libc::c_char,
            domain[0 as libc::c_int as usize],
            domain[1 as libc::c_int as usize],
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"   >>\n\0" as *const u8 as *const libc::c_char,
    );
    if is_ps_pattern != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b">>\n[ \0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).stream, &mut pat_to_ps);
        _cairo_output_stream_printf(
            (*surface).stream,
            b" ]\nmakepattern setpattern\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"shfill\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_emit_mesh_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *mut cairo_mesh_pattern_t,
    mut is_ps_pattern: cairo_bool_t,
) -> cairo_status_t {
    let mut pat_to_ps: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
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
    if _cairo_array_num_elements(&mut (*pattern).patches)
        == 0 as libc::c_int as libc::c_uint
    {
        return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    }
    pat_to_ps = (*pattern).base.matrix;
    status = cairo_matrix_invert(&mut pat_to_ps);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            4643 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 111],
                &[libc::c_char; 111],
            >(
                b"cairo_status_t _cairo_ps_surface_emit_mesh_pattern(cairo_ps_surface_t *, cairo_mesh_pattern_t *, cairo_bool_t)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(&mut pat_to_ps, &mut pat_to_ps, &mut (*surface).cairo_to_ps);
    status = _cairo_pdf_shading_init_color(&mut shading, pattern);
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"currentfile\n/ASCII85Decode filter /FlateDecode filter /ReusableStreamDecode filter\n\0"
            as *const u8 as *const libc::c_char,
    );
    status = _cairo_ps_surface_emit_base85_string(
        surface,
        shading.data,
        shading.data_length,
        CAIRO_PS_COMPRESS_DEFLATE,
        0 as libc::c_int,
    );
    if status as u64 != 0 {
        return status;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"\n/CairoData exch def\n\0" as *const u8 as *const libc::c_char,
    );
    if is_ps_pattern != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"<< /PatternType 2\n   /Shading\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"   << /ShadingType %d\n      /ColorSpace /DeviceRGB\n      /DataSource CairoData\n      /BitsPerCoordinate %d\n      /BitsPerComponent %d\n      /BitsPerFlag %d\n      /Decode [\0"
            as *const u8 as *const libc::c_char,
        shading.shading_type,
        shading.bits_per_coordinate,
        shading.bits_per_component,
        shading.bits_per_flag,
    );
    i = 0 as libc::c_int;
    while i < shading.decode_array_length {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%f \0" as *const u8 as *const libc::c_char,
            *(shading.decode_array).offset(i as isize),
        );
        i += 1;
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"]\n   >>\n\0" as *const u8 as *const libc::c_char,
    );
    if is_ps_pattern != 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b">>\n[ \n\0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).stream, &mut pat_to_ps);
        _cairo_output_stream_printf(
            (*surface).stream,
            b" ]\nmakepattern\nsetpattern\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"shfill\n\0" as *const u8 as *const libc::c_char,
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"currentdict /CairoData undef\n\0" as *const u8 as *const libc::c_char,
    );
    _cairo_pdf_shading_fini(&mut shading);
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_emit_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut pattern: *const cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut op: cairo_operator_t,
) -> cairo_status_t {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if (*pattern).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
    {
        let mut solid: *mut cairo_solid_pattern_t = pattern
            as *mut cairo_solid_pattern_t;
        if (*surface).current_pattern_is_solid_color == 0 as libc::c_int
            || _cairo_color_equal(&mut (*surface).current_color, &mut (*solid).color)
                == 0
        {
            status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators);
            if status as u64 != 0 {
                return status;
            }
            _cairo_ps_surface_emit_solid_pattern(
                surface,
                pattern as *mut cairo_solid_pattern_t,
            );
            (*surface).current_pattern_is_solid_color = 1 as libc::c_int;
            (*surface).current_color = (*solid).color;
        }
        return CAIRO_STATUS_SUCCESS;
    }
    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators);
    if status as u64 != 0 {
        return status;
    }
    match (*pattern).type_0 as libc::c_uint {
        0 => {
            _cairo_ps_surface_emit_solid_pattern(
                surface,
                pattern as *mut cairo_solid_pattern_t,
            );
        }
        1 | 5 => {
            status = _cairo_ps_surface_emit_surface_pattern(
                surface,
                pattern as *mut cairo_pattern_t,
                extents,
                op,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        2 | 3 => {
            status = _cairo_ps_surface_emit_gradient(
                surface,
                pattern as *mut cairo_gradient_pattern_t,
                1 as libc::c_int,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        4 => {
            status = _cairo_ps_surface_emit_mesh_pattern(
                surface,
                pattern as *mut cairo_mesh_pattern_t,
                1 as libc::c_int,
            );
            if status as u64 != 0 {
                return status;
            }
        }
        _ => {}
    }
    return CAIRO_STATUS_SUCCESS;
}
unsafe extern "C" fn _cairo_ps_surface_paint_gradient(
    mut surface: *mut cairo_ps_surface_t,
    mut source: *const cairo_pattern_t,
    mut extents: *const cairo_rectangle_int_t,
) -> cairo_status_t {
    let mut pat_to_ps: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    pat_to_ps = (*source).matrix;
    status = cairo_matrix_invert(&mut pat_to_ps);
    if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
            4794 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"cairo_status_t _cairo_ps_surface_paint_gradient(cairo_ps_surface_t *, const cairo_pattern_t *, const cairo_rectangle_int_t *)\0",
            ))
                .as_ptr(),
        );
    }
    cairo_matrix_multiply(&mut pat_to_ps, &mut pat_to_ps, &mut (*surface).cairo_to_ps);
    if _cairo_matrix_is_identity(&mut pat_to_ps) == 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"[\0" as *const u8 as *const libc::c_char,
        );
        _cairo_output_stream_print_matrix((*surface).stream, &mut pat_to_ps);
        _cairo_output_stream_printf(
            (*surface).stream,
            b"] concat\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*source).type_0 as libc::c_uint
        == CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
    {
        status = _cairo_ps_surface_emit_mesh_pattern(
            surface,
            source as *mut cairo_mesh_pattern_t,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
    } else {
        status = _cairo_ps_surface_emit_gradient(
            surface,
            source as *mut cairo_gradient_pattern_t,
            0 as libc::c_int,
        );
        if status as u64 != 0 {
            return status;
        }
    }
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_paint_pattern(
    mut surface: *mut cairo_ps_surface_t,
    mut source: *const cairo_pattern_t,
    mut extents: *mut cairo_rectangle_int_t,
    mut op: cairo_operator_t,
    mut stencil_mask: cairo_bool_t,
) -> cairo_status_t {
    match (*source).type_0 as libc::c_uint {
        1 | 5 => {
            return _cairo_ps_surface_paint_surface(
                surface,
                source,
                extents,
                op,
                stencil_mask,
            );
        }
        2 | 3 | 4 => return _cairo_ps_surface_paint_gradient(surface, source, extents),
        0 | _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                    4845 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 151],
                        &[libc::c_char; 151],
                    >(
                        b"cairo_status_t _cairo_ps_surface_paint_pattern(cairo_ps_surface_t *, const cairo_pattern_t *, cairo_rectangle_int_t *, cairo_operator_t, cairo_bool_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            return CAIRO_STATUS_SUCCESS;
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
        2 | 3 | 4 => return 1 as libc::c_int,
        _ => {
            if (b"reached\0" as *const u8 as *const libc::c_char).is_null() {} else {
                __assert_fail(
                    b"!\"reached\"\0" as *const u8 as *const libc::c_char,
                    b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                    4868 as libc::c_int as libc::c_uint,
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
unsafe extern "C" fn _cairo_ps_surface_get_extents(
    mut abstract_surface: *mut libc::c_void,
    mut rectangle: *mut cairo_rectangle_int_t,
) -> cairo_bool_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    if (*surface).surface_bounded != 0 {
        *rectangle = (*surface).surface_extents;
    }
    return (*surface).surface_bounded;
}
unsafe extern "C" fn _cairo_ps_surface_get_font_options(
    mut abstract_surface: *mut libc::c_void,
    mut options: *mut cairo_font_options_t,
) {
    _cairo_font_options_init_default(options);
    cairo_font_options_set_hint_style(options, CAIRO_HINT_STYLE_NONE);
    cairo_font_options_set_hint_metrics(options, CAIRO_HINT_METRICS_OFF);
    cairo_font_options_set_antialias(options, CAIRO_ANTIALIAS_GRAY);
    _cairo_font_options_set_round_glyph_positions(options, CAIRO_ROUND_GLYPH_POS_OFF);
}
unsafe extern "C" fn _cairo_ps_surface_set_clip(
    mut surface: *mut cairo_ps_surface_t,
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
unsafe extern "C" fn _cairo_ps_surface_paint(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let mut stream: *mut cairo_output_stream_t = (*surface).stream;
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_composite_rectangles_init_for_paint(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        clip,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_ps_surface_analyze_operation(
            surface,
            op,
            source,
            0 as *const cairo_pattern_t,
            &mut extents.bounded,
        ) as cairo_status_t;
    } else {
        if _cairo_ps_surface_operation_supported(
            surface,
            op,
            source,
            0 as *const cairo_pattern_t,
            &mut extents.bounded,
        ) != 0
        {} else {
            __assert_fail(
                b"_cairo_ps_surface_operation_supported (surface, op, source, NULL, &extents.bounded)\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                4937 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 116],
                    &[libc::c_char; 116],
                >(
                    b"cairo_int_status_t _cairo_ps_surface_paint(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_clip_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        status = _cairo_ps_surface_set_clip(surface, &mut extents) as cairo_status_t;
        if !(status as u64 != 0) {
            if _can_paint_pattern(source) != 0 {
                status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators);
                if !(status as u64 != 0) {
                    _cairo_output_stream_printf(
                        stream,
                        b"q\n\0" as *const u8 as *const libc::c_char,
                    );
                    status = _cairo_ps_surface_paint_pattern(
                        surface,
                        source,
                        &mut extents.bounded,
                        op,
                        0 as libc::c_int,
                    );
                    if !(status as u64 != 0) {
                        _cairo_output_stream_printf(
                            stream,
                            b"Q\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            } else {
                status = _cairo_ps_surface_emit_pattern(
                    surface,
                    source,
                    &mut extents.bounded,
                    op,
                );
                if !(status as u64 != 0) {
                    _cairo_output_stream_printf(
                        stream,
                        b"%d %d %d %d rectfill\n\0" as *const u8 as *const libc::c_char,
                        (*surface).surface_extents.x,
                        (*surface).surface_extents.y,
                        (*surface).surface_extents.width,
                        (*surface).surface_extents.height,
                    );
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_mask(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut mask: *const cairo_pattern_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let mut stream: *mut cairo_output_stream_t = (*surface).stream;
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    status = _cairo_composite_rectangles_init_for_mask(
        &mut extents,
        &mut (*surface).base,
        op,
        source,
        mask,
        clip,
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_ps_surface_analyze_operation(
            surface,
            op,
            source,
            mask,
            &mut extents.bounded,
        ) as cairo_status_t;
    } else {
        if _cairo_ps_surface_operation_supported(
            surface,
            op,
            source,
            mask,
            &mut extents.bounded,
        ) != 0
        {} else {
            __assert_fail(
                b"_cairo_ps_surface_operation_supported (surface, op, source, mask, &extents.bounded)\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                5001 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"cairo_int_status_t _cairo_ps_surface_mask(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_pattern_t *, const cairo_clip_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        status = _cairo_ps_surface_set_clip(surface, &mut extents) as cairo_status_t;
        if !(status as u64 != 0) {
            status = _cairo_ps_surface_emit_pattern(
                surface,
                source,
                &mut extents.bounded,
                op,
            );
            if !(status as u64 != 0) {
                _cairo_output_stream_printf(
                    stream,
                    b"q\n\0" as *const u8 as *const libc::c_char,
                );
                status = _cairo_ps_surface_paint_pattern(
                    surface,
                    mask,
                    &mut extents.bounded,
                    op,
                    1 as libc::c_int,
                );
                if !(status as u64 != 0) {
                    _cairo_output_stream_printf(
                        stream,
                        b"Q\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_stroke(
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
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
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
    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut b: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    status = _cairo_path_fixed_stroke_extents(
        path,
        style,
        ctm,
        ctm_inverse,
        tolerance,
        &mut r,
    ) as cairo_int_status_t;
    if !(status as u64 != 0) {
        _cairo_box_from_rectangle(&mut b, &mut r);
        status = _cairo_composite_rectangles_intersect_mask_extents(
            &mut extents,
            &mut b,
        );
        if !(status as u64 != 0) {
            if (*surface).paginated_mode as libc::c_uint
                == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
            {
                status = _cairo_ps_surface_analyze_operation(
                    surface,
                    op,
                    source,
                    0 as *const cairo_pattern_t,
                    &mut extents.bounded,
                );
            } else {
                if _cairo_ps_surface_operation_supported(
                    surface,
                    op,
                    source,
                    0 as *const cairo_pattern_t,
                    &mut extents.bounded,
                ) != 0
                {} else {
                    __assert_fail(
                        b"_cairo_ps_surface_operation_supported (surface, op, source, NULL, &extents.bounded)\0"
                            as *const u8 as *const libc::c_char,
                        b"../src/cairo-ps-surface.c\0" as *const u8
                            as *const libc::c_char,
                        5077 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 250],
                            &[libc::c_char; 250],
                        >(
                            b"cairo_int_status_t _cairo_ps_surface_stroke(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, const cairo_stroke_style_t *, const cairo_matrix_t *, const cairo_matrix_t *, double, cairo_antialias_t, const cairo_clip_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                status = _cairo_ps_surface_set_clip(surface, &mut extents);
                if !(status as u64 != 0) {
                    status = _cairo_ps_surface_emit_pattern(
                        surface,
                        source,
                        &mut extents.bounded,
                        op,
                    ) as cairo_int_status_t;
                    if !(status as u64 != 0) {
                        status = _cairo_pdf_operators_stroke(
                            &mut (*surface).pdf_operators,
                            path,
                            style,
                            ctm,
                            ctm_inverse,
                        );
                    }
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_fill(
    mut abstract_surface: *mut libc::c_void,
    mut op: cairo_operator_t,
    mut source: *const cairo_pattern_t,
    mut path: *const cairo_path_fixed_t,
    mut fill_rule: cairo_fill_rule_t,
    mut tolerance: libc::c_double,
    mut antialias: cairo_antialias_t,
    mut clip: *const cairo_clip_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
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
    let mut r: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut b: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    _cairo_path_fixed_fill_extents(path, fill_rule, tolerance, &mut r);
    _cairo_box_from_rectangle(&mut b, &mut r);
    status = _cairo_composite_rectangles_intersect_mask_extents(&mut extents, &mut b);
    if !(status as u64 != 0) {
        if (*surface).paginated_mode as libc::c_uint
            == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
        {
            status = _cairo_ps_surface_analyze_operation(
                surface,
                op,
                source,
                0 as *const cairo_pattern_t,
                &mut extents.bounded,
            );
        } else {
            if _cairo_ps_surface_operation_supported(
                surface,
                op,
                source,
                0 as *const cairo_pattern_t,
                &mut extents.bounded,
            ) != 0
            {} else {
                __assert_fail(
                    b"_cairo_ps_surface_operation_supported (surface, op, source, NULL, &extents.bounded)\0"
                        as *const u8 as *const libc::c_char,
                    b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                    5145 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 189],
                        &[libc::c_char; 189],
                    >(
                        b"cairo_int_status_t _cairo_ps_surface_fill(void *, cairo_operator_t, const cairo_pattern_t *, const cairo_path_fixed_t *, cairo_fill_rule_t, double, cairo_antialias_t, const cairo_clip_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators)
                as cairo_int_status_t;
            if !(status as u64 != 0) {
                status = _cairo_ps_surface_set_clip(surface, &mut extents);
                if !(status as u64 != 0) {
                    if _can_paint_pattern(source) != 0 {
                        _cairo_output_stream_printf(
                            (*surface).stream,
                            b"q\n\0" as *const u8 as *const libc::c_char,
                        );
                        status = _cairo_pdf_operators_clip(
                            &mut (*surface).pdf_operators,
                            path,
                            fill_rule,
                        );
                        if !(status as u64 != 0) {
                            status = _cairo_ps_surface_paint_pattern(
                                surface,
                                source,
                                &mut extents.bounded,
                                op,
                                0 as libc::c_int,
                            ) as cairo_int_status_t;
                            if !(status as u64 != 0) {
                                _cairo_output_stream_printf(
                                    (*surface).stream,
                                    b"Q\n\0" as *const u8 as *const libc::c_char,
                                );
                                _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
                            }
                        }
                    } else {
                        status = _cairo_ps_surface_emit_pattern(
                            surface,
                            source,
                            &mut extents.bounded,
                            op,
                        ) as cairo_int_status_t;
                        if !(status as u64 != 0) {
                            status = _cairo_pdf_operators_fill(
                                &mut (*surface).pdf_operators,
                                path,
                                fill_rule,
                            );
                        }
                    }
                }
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status;
}
unsafe extern "C" fn _cairo_ps_surface_has_show_text_glyphs(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
unsafe extern "C" fn _cairo_ps_surface_show_text_glyphs(
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
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
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
        },
        original_source_pattern: 0 as *const cairo_pattern_t,
        original_mask_pattern: 0 as *const cairo_pattern_t,
        clip: 0 as *mut cairo_clip_t,
    };
    let mut overlap: cairo_bool_t = 0;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
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
    ) as cairo_status_t;
    if status as u64 != 0 {
        return status as cairo_int_status_t;
    }
    if (*surface).paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_ANALYZE as libc::c_int as libc::c_uint
    {
        status = _cairo_ps_surface_analyze_operation(
            surface,
            op,
            source,
            0 as *const cairo_pattern_t,
            &mut extents.bounded,
        ) as cairo_status_t;
    } else {
        if _cairo_ps_surface_operation_supported(
            surface,
            op,
            source,
            0 as *const cairo_pattern_t,
            &mut extents.bounded,
        ) != 0
        {} else {
            __assert_fail(
                b"_cairo_ps_surface_operation_supported (surface, op, source, NULL, &extents.bounded)\0"
                    as *const u8 as *const libc::c_char,
                b"../src/cairo-ps-surface.c\0" as *const u8 as *const libc::c_char,
                5232 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 254],
                    &[libc::c_char; 254],
                >(
                    b"cairo_int_status_t _cairo_ps_surface_show_text_glyphs(void *, cairo_operator_t, const cairo_pattern_t *, const char *, int, cairo_glyph_t *, int, const cairo_text_cluster_t *, int, cairo_text_cluster_flags_t, cairo_scaled_font_t *, const cairo_clip_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        status = _cairo_ps_surface_set_clip(surface, &mut extents) as cairo_status_t;
        if !(status as u64 != 0) {
            status = _cairo_ps_surface_emit_pattern(
                surface,
                source,
                &mut extents.bounded,
                op,
            );
            if !(status as u64 != 0) {
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
                ) as cairo_status_t;
            }
        }
    }
    _cairo_composite_rectangles_fini(&mut extents);
    return status as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_get_supported_mime_types(
    mut abstract_surface: *mut libc::c_void,
) -> *mut *const libc::c_char {
    return _cairo_ps_supported_mime_types.as_mut_ptr();
}
unsafe extern "C" fn _cairo_ps_surface_set_paginated_mode(
    mut abstract_surface: *mut libc::c_void,
    mut paginated_mode: cairo_paginated_mode_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    (*surface).paginated_mode = paginated_mode;
    if paginated_mode as libc::c_uint
        == CAIRO_PAGINATED_MODE_RENDER as libc::c_int as libc::c_uint
    {
        (*surface).surface_extents.x = 0 as libc::c_int;
        (*surface).surface_extents.y = 0 as libc::c_int;
        (*surface).surface_extents.width = ceil((*surface).width) as libc::c_int;
        (*surface).surface_extents.height = ceil((*surface).height) as libc::c_int;
        if !((*surface).clipper.clip).is_null() {
            status = _cairo_pdf_operators_flush(&mut (*surface).pdf_operators);
            _cairo_output_stream_printf(
                (*surface).stream,
                b"Q q\n\0" as *const u8 as *const libc::c_char,
            );
            _cairo_surface_clipper_reset(&mut (*surface).clipper);
        }
    }
    return CAIRO_STATUS_SUCCESS as libc::c_int as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_set_bounding_box(
    mut abstract_surface: *mut libc::c_void,
    mut analysis_bbox: *mut cairo_box_t,
) -> cairo_int_status_t {
    let mut surface: *mut cairo_ps_surface_t = abstract_surface
        as *mut cairo_ps_surface_t;
    let mut i: libc::c_int = 0;
    let mut num_comments: libc::c_int = 0;
    let mut comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut has_page_media: cairo_bool_t = 0;
    let mut has_page_bbox: cairo_bool_t = 0;
    let mut page_media: *const libc::c_char = 0 as *const libc::c_char;
    let mut page_bbox: cairo_rectangle_int_t = cairo_rectangle_int_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut bbox_p1: cairo_point_int_t = cairo_point_int_t { x: 0, y: 0 };
    let mut bbox_p2: cairo_point_int_t = cairo_point_int_t { x: 0, y: 0 };
    _cairo_box_round_to_rectangle(analysis_bbox, &mut page_bbox);
    bbox_p1.x = page_bbox.x;
    bbox_p1
        .y = (ceil((*surface).height)
        - (page_bbox.y + page_bbox.height) as libc::c_double) as libc::c_int;
    bbox_p2.x = page_bbox.x + page_bbox.width;
    bbox_p2.y = (ceil((*surface).height) - page_bbox.y as libc::c_double) as libc::c_int;
    if (*surface).num_pages == 1 as libc::c_int {
        (*surface).document_bbox_p1 = bbox_p1;
        (*surface).document_bbox_p2 = bbox_p2;
    } else {
        if bbox_p1.x < (*surface).document_bbox_p1.x {
            (*surface).document_bbox_p1.x = bbox_p1.x;
        }
        if bbox_p1.y < (*surface).document_bbox_p1.y {
            (*surface).document_bbox_p1.y = bbox_p1.y;
        }
        if bbox_p2.x < (*surface).document_bbox_p2.x {
            (*surface).document_bbox_p2.x = bbox_p2.x;
        }
        if bbox_p2.y < (*surface).document_bbox_p2.y {
            (*surface).document_bbox_p2.y = bbox_p2.y;
        }
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"%%%%Page: %d %d\n\0" as *const u8 as *const libc::c_char,
        (*surface).num_pages,
        (*surface).num_pages,
    );
    _cairo_output_stream_printf(
        (*surface).stream,
        b"%%%%BeginPageSetup\n\0" as *const u8 as *const libc::c_char,
    );
    has_page_media = 0 as libc::c_int;
    has_page_bbox = 0 as libc::c_int;
    num_comments = _cairo_array_num_elements(&mut (*surface).dsc_page_setup_comments)
        as libc::c_int;
    comments = _cairo_array_index(
        &mut (*surface).dsc_page_setup_comments,
        0 as libc::c_int as libc::c_uint,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < num_comments {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            *comments.offset(i as isize),
        );
        if strncmp(
            *comments.offset(i as isize),
            b"%%PageMedia:\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            has_page_media = 1 as libc::c_int;
        }
        if strncmp(
            *comments.offset(i as isize),
            b"%%PageBoundingBox:\0" as *const u8 as *const libc::c_char,
            18 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            has_page_bbox = 1 as libc::c_int;
        }
        free(*comments.offset(i as isize) as *mut libc::c_void);
        let ref mut fresh46 = *comments.offset(i as isize);
        *fresh46 = 0 as *mut libc::c_char;
        i += 1;
    }
    _cairo_array_truncate(
        &mut (*surface).dsc_page_setup_comments,
        0 as libc::c_int as libc::c_uint,
    );
    if has_page_media == 0 && (*surface).eps == 0 {
        page_media = _cairo_ps_surface_get_page_media(surface);
        if page_media.is_null() {
            return _cairo_error(CAIRO_STATUS_NO_MEMORY) as cairo_int_status_t;
        }
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%%%%PageMedia: %s\n\0" as *const u8 as *const libc::c_char,
            page_media,
        );
    }
    if has_page_bbox == 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%%%%PageBoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
            bbox_p1.x,
            bbox_p1.y,
            bbox_p2.x,
            bbox_p2.y,
        );
    }
    if (*surface).eps == 0 {
        _cairo_output_stream_printf(
            (*surface).stream,
            b"%f %f cairo_set_page_size\n\0" as *const u8 as *const libc::c_char,
            ceil((*surface).width),
            ceil((*surface).height),
        );
    }
    _cairo_output_stream_printf(
        (*surface).stream,
        b"%%%%EndPageSetup\nq %d %d %d %d rectclip\n1 0 0 -1 0 %f cm q\n\0" as *const u8
            as *const libc::c_char,
        bbox_p1.x,
        bbox_p1.y,
        bbox_p2.x - bbox_p1.x,
        bbox_p2.y - bbox_p1.y,
        ceil((*surface).height),
    );
    (*surface).current_pattern_is_solid_color = 0 as libc::c_int;
    _cairo_pdf_operators_reset(&mut (*surface).pdf_operators);
    return _cairo_output_stream_get_status((*surface).stream) as cairo_int_status_t;
}
unsafe extern "C" fn _cairo_ps_surface_supports_fine_grained_fallbacks(
    mut abstract_surface: *mut libc::c_void,
) -> cairo_bool_t {
    return 1 as libc::c_int;
}
static mut cairo_ps_surface_backend: cairo_surface_backend_t = unsafe {
    {
        let mut init = _cairo_surface_backend {
            type_0: CAIRO_SURFACE_TYPE_PS,
            finish: Some(
                _cairo_ps_surface_finish
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
                _cairo_ps_surface_show_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            get_extents: Some(
                _cairo_ps_surface_get_extents
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_rectangle_int_t,
                    ) -> cairo_bool_t,
            ),
            get_font_options: Some(
                _cairo_ps_surface_get_font_options
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_font_options_t,
                    ) -> (),
            ),
            flush: None,
            mark_dirty_rectangle: None,
            paint: Some(
                _cairo_ps_surface_paint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            mask: Some(
                _cairo_ps_surface_mask
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_operator_t,
                        *const cairo_pattern_t,
                        *const cairo_pattern_t,
                        *const cairo_clip_t,
                    ) -> cairo_int_status_t,
            ),
            stroke: Some(
                _cairo_ps_surface_stroke
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
                _cairo_ps_surface_fill
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
            fill_stroke: None,
            show_glyphs: None,
            has_show_text_glyphs: Some(
                _cairo_ps_surface_has_show_text_glyphs
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            show_text_glyphs: Some(
                _cairo_ps_surface_show_text_glyphs
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
                _cairo_ps_surface_get_supported_mime_types
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                    ) -> *mut *const libc::c_char,
            ),
            tag: None,
        };
        init
    }
};
static mut cairo_ps_surface_paginated_backend: cairo_paginated_surface_backend_t = unsafe {
    {
        let mut init = _cairo_paginated_surface_backend {
            start_page: Some(
                _cairo_ps_surface_start_page
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_int_status_t,
            ),
            set_paginated_mode: Some(
                _cairo_ps_surface_set_paginated_mode
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        cairo_paginated_mode_t,
                    ) -> cairo_int_status_t,
            ),
            set_bounding_box: Some(
                _cairo_ps_surface_set_bounding_box
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cairo_box_t,
                    ) -> cairo_int_status_t,
            ),
            set_fallback_images_required: None,
            supports_fine_grained_fallbacks: Some(
                _cairo_ps_surface_supports_fine_grained_fallbacks
                    as unsafe extern "C" fn(*mut libc::c_void) -> cairo_bool_t,
            ),
            requires_thumbnail_image: None,
            set_thumbnail_image: None,
        };
        init
    }
};
