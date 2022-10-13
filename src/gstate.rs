use crate::cairo::{Antialias, FillRule, Operator, StrokeStyle};
use crate::matrix::Matrix;

pub struct GState {
    op: Operator,
    opacity: f64,
    tolerance: f64,
    antialias: Antialias,
    stroke_style: StrokeStyle,
    fill_rule: FillRule,
    font_face: *mut FontFace,
    scaled_font: *mut ScaledFont,
    previous_scaled_font: *mut ScaledFont,
    font_matrix: Matrix,
    font_options: FontOptions,
    clip: *mut Clip,
    target: *mut Surface,
    parent_target: *mut Surface,
    original_target: *mut Surface,
    device_transform_observer: Observer,
    ctm: Matrix,
    ctm_inverse: Matrix,
    source_ctm_inverse: Matrix,
    is_identity: bool,
    source: *mut Pattern,
    next: Option<*mut GState>,
}

// #[inline]
// fn _cairo_gstate_user_to_backend(mut gstate: *mut GState, mut x: *mut f64, mut y: *mut f64) {
//     if gstate.is_identity == 0 {
//         _do_cairo_gstate_user_to_backend(gstate, x, y);
//     }
// }

// fn _cairo_gstate_update_device_transform(mut observer: *mut Observer, mut arg: *mut libc::c_void) {
//     let mut gstate: *mut GState = ({
//         let mut mptr__: *const Observer = observer;
//         (mptr__ as *mut libc::c_char).offset(-(248 as libc::c_ulong as isize)) as *mut GState
//     });

//     gstate.is_identity = gstate.ctm.is_identity() && gstate.target.device_transform.is_identity();
// }

// pub fn _cairo_gstate_init(gstate: *mut GState, target: *mut Surface) -> cairo_status_t {
//     gstate.next = None;

//     gstate.op = OPERATOR::OVER;
//     gstate.opacity = 1.0;

//     gstate.tolerance = CAIRO_GSTATE_TOLERANCE_DEFAULT;
//     gstate.antialias = CAIRO_ANTIALIAS_DEFAULT;

//     _cairo_stroke_style_init(&mut gstate.stroke_style);

//     gstate.fill_rule = FillRule::WINDING;

//     gstate.font_face = None;
//     gstate.scaled_font = None;
//     gstate.previous_scaled_font = None;

//     gstate.font_matrix.init_scale(
//         CAIRO_GSTATE_DEFAULT_FONT_SIZE,
//         CAIRO_GSTATE_DEFAULT_FONT_SIZE,
//     );

//     _cairo_font_options_init_default(&mut gstate.font_options);
//     gstate.clip = None;

//     gstate.target = cairo_surface_reference(&mut target);
//     gstate.parent_target = None;
//     gstate.original_target = cairo_surface_reference(target);

//     let ref mut fresh18 = gstate.device_transform_observer.callback;
//     *fresh18 =
//         Some(_cairo_gstate_update_device_transform as fn(*mut Observer, *mut libc::c_void) -> ());
//     cairo_list_add(
//         &mut gstate.device_transform_observer.link,
//         &mut (*gstate.target).device_transform_observers,
//     );
//     gstate.is_identity = _cairo_matrix_is_identity(&mut (*gstate.target).device_transform);
//     cairo_matrix_init_identity(&mut gstate.ctm);
//     gstate.ctm_inverse = gstate.ctm;
//     gstate.source_ctm_inverse = gstate.ctm;
//     let ref mut fresh19 = gstate.source;
//     *fresh19 = &_cairo_pattern_black.base as *const Pattern as *mut Pattern;
//     return target.status;
// }
// fn _cairo_gstate_init_copy(mut gstate: *mut GState, mut other: *mut GState) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     gstate.op = (*other).op;
//     gstate.opacity = (*other).opacity;
//     gstate.tolerance = (*other).tolerance;
//     gstate.antialias = (*other).antialias;
//     status = _cairo_stroke_style_init_copy(&mut gstate.stroke_style, &mut (*other).stroke_style);
//     if status as u64 != 0 {
//         return status;
//     }
//     gstate.fill_rule = (*other).fill_rule;
//     let ref mut fresh20 = gstate.font_face;
//     *fresh20 = cairo_font_face_reference((*other).font_face);
//     let ref mut fresh21 = gstate.scaled_font;
//     *fresh21 = cairo_scaled_font_reference((*other).scaled_font);
//     let ref mut fresh22 = gstate.previous_scaled_font;
//     *fresh22 = cairo_scaled_font_reference((*other).previous_scaled_font);
//     gstate.font_matrix = (*other).font_matrix;
//     _cairo_font_options_init_copy(&mut gstate.font_options, &mut (*other).font_options);
//     let ref mut fresh23 = gstate.clip;
//     *fresh23 = _cairo_clip_copy((*other).clip);
//     let ref mut fresh24 = gstate.target;
//     *fresh24 = cairo_surface_reference((*other).target);
//     let ref mut fresh25 = gstate.parent_target;
//     *fresh25 = 0 as *mut Surface;
//     let ref mut fresh26 = gstate.original_target;
//     *fresh26 = cairo_surface_reference((*other).original_target);
//     let ref mut fresh27 = gstate.device_transform_observer.callback;
//     *fresh27 =
//         Some(_cairo_gstate_update_device_transform as fn(*mut Observer, *mut libc::c_void) -> ());
//     cairo_list_add(
//         &mut gstate.device_transform_observer.link,
//         &mut (*gstate.target).device_transform_observers,
//     );
//     gstate.is_identity = (*other).is_identity;
//     gstate.ctm = (*other).ctm;
//     gstate.ctm_inverse = (*other).ctm_inverse;
//     gstate.source_ctm_inverse = (*other).source_ctm_inverse;
//     let ref mut fresh28 = gstate.source;
//     *fresh28 = cairo_pattern_reference((*other).source);
//     let ref mut fresh29 = gstate.next;
//     *fresh29 = 0 as *mut GState;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_fini(mut gstate: *mut GState) {
//     _cairo_stroke_style_fini(&mut gstate.stroke_style);
//     cairo_font_face_destroy(gstate.font_face);
//     let ref mut fresh30 = gstate.font_face;
//     *fresh30 = 0 as *mut FontFace;
//     cairo_scaled_font_destroy(gstate.previous_scaled_font);
//     let ref mut fresh31 = gstate.previous_scaled_font;
//     *fresh31 = 0 as *mut ScaledFont;
//     cairo_scaled_font_destroy(gstate.scaled_font);
//     let ref mut fresh32 = gstate.scaled_font;
//     *fresh32 = 0 as *mut ScaledFont;
//     _cairo_clip_destroy(gstate.clip);
//     cairo_list_del(&mut gstate.device_transform_observer.link);
//     cairo_surface_destroy(gstate.target);
//     let ref mut fresh33 = gstate.target;
//     *fresh33 = 0 as *mut Surface;
//     cairo_surface_destroy(gstate.parent_target);
//     let ref mut fresh34 = gstate.parent_target;
//     *fresh34 = 0 as *mut Surface;
//     cairo_surface_destroy(gstate.original_target);
//     let ref mut fresh35 = gstate.original_target;
//     *fresh35 = 0 as *mut Surface;
//     cairo_pattern_destroy(gstate.source);
//     let ref mut fresh36 = gstate.source;
//     *fresh36 = 0 as *mut Pattern;
// }

// pub fn _cairo_gstate_save(
//     mut gstate: *mut *mut GState,
//     mut freelist: *mut *mut GState,
// ) -> cairo_status_t {
//     let mut top: *mut GState = 0 as *mut GState;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     top = *freelist;
//     if top.is_null() {
//         top = (if ::std::mem::size_of::<GState>() as libc::c_ulong
//             != 0 as libc::c_int as libc::c_ulong
//         {
//             malloc(::std::mem::size_of::<GState>() as libc::c_ulong)
//         } else {
//             0 as *mut libc::c_void
//         }) as *mut GState;
//         if top.is_null() {
//             return _cairo_error(CAIRO_STATUS_NO_MEMORY);
//         }
//     } else {
//         *freelist = (*top).next;
//     }
//     status = _cairo_gstate_init_copy(top, *gstate);
//     if status as u64 != 0 {
//         let ref mut fresh37 = (*top).next;
//         *fresh37 = *freelist;
//         *freelist = top;
//         return status;
//     }
//     let ref mut fresh38 = (*top).next;
//     *fresh38 = *gstate;
//     *gstate = top;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_restore(
//     mut gstate: *mut *mut GState,
//     mut freelist: *mut *mut GState,
// ) -> cairo_status_t {
//     let mut top: *mut GState = 0 as *mut GState;
//     top = *gstate;
//     if ((*top).next).is_null() {
//         return _cairo_error(CAIRO_STATUS_INVALID_RESTORE);
//     }
//     *gstate = (*top).next;
//     _cairo_gstate_fini(top);
//     let ref mut fresh39 = (*top).next;
//     *fresh39 = *freelist;
//     *freelist = top;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_redirect_target(
//     mut gstate: *mut GState,
//     mut child: *mut Surface,
// ) -> cairo_status_t {
//     if (gstate.parent_target).is_null() {
//     } else {
//         __assert_fail(
//             b"gstate->parent_target == NULL\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo-gstate.c\0" as *const u8 as *const libc::c_char,
//             302 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 82], &[libc::c_char; 82]>(
//                 b"cairo_status_t _cairo_gstate_redirect_target(GState *, Surface *)\0",
//             ))
//             .as_ptr(),
//         );
//     }
//     let ref mut fresh40 = gstate.parent_target;
//     *fresh40 = gstate.target;
//     let ref mut fresh41 = gstate.target;
//     *fresh41 = cairo_surface_reference(child);
//     let ref mut fresh42 = gstate.is_identity;
//     *fresh42 &= _cairo_matrix_is_identity(&mut (*child).device_transform);
//     cairo_list_move(
//         &mut gstate.device_transform_observer.link,
//         &mut (*gstate.target).device_transform_observers,
//     );
//     _cairo_clip_destroy(gstate.clip);
//     let ref mut fresh43 = gstate.clip;
//     *fresh43 = _cairo_clip_copy_with_translation(
//         (*gstate.next).clip,
//         ((*child).device_transform.x0 - (*gstate.parent_target).device_transform.x0) as libc::c_int,
//         ((*child).device_transform.y0 - (*gstate.parent_target).device_transform.y0) as libc::c_int,
//     );
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_is_group(mut gstate: *mut GState) -> bool {
//     return (gstate.parent_target != 0 as *mut libc::c_void as *mut Surface) as libc::c_int;
// }

// pub fn _cairo_gstate_get_target(mut gstate: *mut GState) -> *mut Surface {
//     return gstate.target;
// }

// pub fn _cairo_gstate_get_original_target(mut gstate: *mut GState) -> *mut Surface {
//     return gstate.original_target;
// }

// pub fn _cairo_gstate_get_clip(mut gstate: *mut GState) -> *mut Clip {
//     return gstate.clip;
// }

// pub fn _cairo_gstate_set_source(
//     mut gstate: *mut GState,
//     mut source: *mut Pattern,
// ) -> cairo_status_t {
//     if (*source).status as u64 != 0 {
//         return (*source).status;
//     }
//     source = cairo_pattern_reference(source);
//     cairo_pattern_destroy(gstate.source);
//     let ref mut fresh44 = gstate.source;
//     *fresh44 = source;
//     gstate.source_ctm_inverse = gstate.ctm_inverse;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_source(mut gstate: *mut GState) -> *mut Pattern {
//     if gstate.source == &_cairo_pattern_black.base as *const Pattern as *mut Pattern {
//         let ref mut fresh45 = gstate.source;
//         *fresh45 = _cairo_pattern_create_solid(_cairo_stock_color(CAIRO_STOCK_BLACK));
//     }
//     return gstate.source;
// }

// pub fn _cairo_gstate_set_operator(mut gstate: *mut GState, mut op: Operator) -> cairo_status_t {
//     gstate.op = op;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_operator(mut gstate: *mut GState) -> Operator {
//     return gstate.op;
// }

// pub fn _cairo_gstate_set_opacity(mut gstate: *mut GState, mut op: f64) -> cairo_status_t {
//     gstate.opacity = op;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_opacity(mut gstate: *mut GState) -> f64 {
//     return gstate.opacity;
// }

// pub fn _cairo_gstate_set_tolerance(mut gstate: *mut GState, mut tolerance: f64) -> cairo_status_t {
//     gstate.tolerance = tolerance;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_tolerance(mut gstate: *mut GState) -> f64 {
//     return gstate.tolerance;
// }

// pub fn _cairo_gstate_set_fill_rule(
//     mut gstate: *mut GState,
//     mut fill_rule: FillRule,
// ) -> cairo_status_t {
//     gstate.fill_rule = fill_rule;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_fill_rule(mut gstate: *mut GState) -> FillRule {
//     return gstate.fill_rule;
// }

// pub fn _cairo_gstate_set_line_width(mut gstate: *mut GState, mut width: f64) -> cairo_status_t {
//     if gstate.stroke_style.is_hairline != 0 {
//         gstate.stroke_style.pre_hairline_line_width = width;
//     } else {
//         gstate.stroke_style.line_width = width;
//     }
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_line_width(mut gstate: *mut GState) -> f64 {
//     return gstate.stroke_style.line_width;
// }

// pub fn _cairo_gstate_set_hairline(
//     mut gstate: *mut GState,
//     mut set_hairline: bool,
// ) -> cairo_status_t {
//     if gstate.stroke_style.is_hairline != set_hairline {
//         gstate.stroke_style.is_hairline = set_hairline;
//         if set_hairline != 0 {
//             gstate.stroke_style.pre_hairline_line_width = gstate.stroke_style.line_width;
//             gstate.stroke_style.line_width = 0.0f64;
//         } else {
//             gstate.stroke_style.line_width = gstate.stroke_style.pre_hairline_line_width;
//         }
//     }
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_hairline(mut gstate: *mut GState) -> bool {
//     return gstate.stroke_style.is_hairline;
// }

// pub fn _cairo_gstate_set_line_cap(
//     mut gstate: *mut GState,
//     mut line_cap: cairo_line_cap_t,
// ) -> cairo_status_t {
//     gstate.stroke_style.line_cap = line_cap;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_line_cap(mut gstate: *mut GState) -> cairo_line_cap_t {
//     return gstate.stroke_style.line_cap;
// }

// pub fn _cairo_gstate_set_line_join(
//     mut gstate: *mut GState,
//     mut line_join: cairo_line_join_t,
// ) -> cairo_status_t {
//     gstate.stroke_style.line_join = line_join;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_line_join(mut gstate: *mut GState) -> cairo_line_join_t {
//     return gstate.stroke_style.line_join;
// }

// pub fn _cairo_gstate_set_dash(
//     mut gstate: *mut GState,
//     mut dash: *const f64,
//     mut num_dashes: libc::c_int,
//     mut offset: f64,
// ) -> cairo_status_t {
//     let mut dash_total: f64 = 0.;
//     let mut on_total: f64 = 0.;
//     let mut off_total: f64 = 0.;
//     let mut i: libc::c_int = 0;
//     let mut j: libc::c_int = 0;
//     free(gstate.stroke_style.dash as *mut libc::c_void);
//     gstate.stroke_style.num_dashes = num_dashes as libc::c_uint;
//     if gstate.stroke_style.num_dashes == 0 as libc::c_int as libc::c_uint {
//         let ref mut fresh46 = gstate.stroke_style.dash;
//         *fresh46 = 0 as *mut f64;
//         gstate.stroke_style.dash_offset = 0.0f64;
//         return CAIRO_STATUS_SUCCESS;
//     }
//     let ref mut fresh47 = gstate.stroke_style.dash;
//     *fresh47 = _cairo_malloc_ab(
//         gstate.stroke_style.num_dashes as size_t,
//         ::std::mem::size_of::<f64>() as libc::c_ulong,
//     ) as *mut f64;
//     if (gstate.stroke_style.dash).is_null() {
//         gstate.stroke_style.num_dashes = 0 as libc::c_int as libc::c_uint;
//         return _cairo_error(CAIRO_STATUS_NO_MEMORY);
//     }
//     dash_total = 0.0f64;
//     off_total = dash_total;
//     on_total = off_total;
//     j = 0 as libc::c_int;
//     i = j;
//     while i < num_dashes {
//         if *dash.offset(i as isize) < 0 as libc::c_int as f64 {
//             return _cairo_error(CAIRO_STATUS_INVALID_DASH);
//         }
//         if *dash.offset(i as isize) == 0 as libc::c_int as f64
//             && i > 0 as libc::c_int
//             && i < num_dashes - 1 as libc::c_int
//         {
//             i += 1;
//             if *dash.offset(i as isize) < 0 as libc::c_int as f64 {
//                 return _cairo_error(CAIRO_STATUS_INVALID_DASH);
//             }
//             *(gstate.stroke_style.dash).offset((j - 1 as libc::c_int) as isize) +=
//                 *dash.offset(i as isize);
//             let ref mut fresh48 = gstate.stroke_style.num_dashes;
//             *fresh48 = (*fresh48).wrapping_sub(2 as libc::c_int as libc::c_uint);
//         } else {
//             let fresh49 = j;
//             j = j + 1;
//             *(gstate.stroke_style.dash).offset(fresh49 as isize) = *dash.offset(i as isize);
//         }
//         if *dash.offset(i as isize) != 0. {
//             dash_total += *dash.offset(i as isize);
//             if i & 1 as libc::c_int == 0 as libc::c_int {
//                 on_total += *dash.offset(i as isize);
//             } else {
//                 off_total += *dash.offset(i as isize);
//             }
//         }
//         i += 1;
//     }
//     if dash_total == 0.0f64 {
//         return _cairo_error(CAIRO_STATUS_INVALID_DASH);
//     }
//     if gstate.stroke_style.num_dashes & 1 as libc::c_int as libc::c_uint != 0 {
//         dash_total *= 2 as libc::c_int as f64;
//         on_total += off_total;
//     }
//     if dash_total - on_total
//         < 1.0f64 / (2 as libc::c_int as f64 * ((1 as libc::c_int) << 8 as libc::c_int) as f64)
//     {
//         free(gstate.stroke_style.dash as *mut libc::c_void);
//         let ref mut fresh50 = gstate.stroke_style.dash;
//         *fresh50 = 0 as *mut f64;
//         gstate.stroke_style.num_dashes = 0 as libc::c_int as libc::c_uint;
//         gstate.stroke_style.dash_offset = 0.0f64;
//         return CAIRO_STATUS_SUCCESS;
//     }
//     offset = fmod(offset, dash_total);
//     if offset < 0.0f64 {
//         offset += dash_total;
//     }
//     if offset <= 0.0f64 {
//         offset = 0.0f64;
//     }
//     gstate.stroke_style.dash_offset = offset;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_dash(
//     mut gstate: *mut GState,
//     mut dashes: *mut f64,
//     mut num_dashes: *mut libc::c_int,
//     mut offset: *mut f64,
// ) {
//     if !dashes.is_null() {
//         memcpy(
//             dashes as *mut libc::c_void,
//             gstate.stroke_style.dash as *const libc::c_void,
//             (::std::mem::size_of::<f64>() as libc::c_ulong)
//                 .wrapping_mul(gstate.stroke_style.num_dashes as libc::c_ulong),
//         );
//     }
//     if !num_dashes.is_null() {
//         *num_dashes = gstate.stroke_style.num_dashes as libc::c_int;
//     }
//     if !offset.is_null() {
//         *offset = gstate.stroke_style.dash_offset;
//     }
// }

// pub fn _cairo_gstate_set_miter_limit(mut gstate: *mut GState, mut limit: f64) -> cairo_status_t {
//     gstate.stroke_style.miter_limit = limit;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_miter_limit(mut gstate: *mut GState) -> f64 {
//     return gstate.stroke_style.miter_limit;
// }

// pub fn _cairo_gstate_get_matrix(mut gstate: *mut GState, mut matrix: *mut Matrix) {
//     *matrix = gstate.ctm;
// }

// pub fn _cairo_gstate_translate(
//     mut gstate: *mut GState,
//     mut tx: f64,
//     mut ty: f64,
// ) -> cairo_status_t {
//     let mut tmp: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     if tx.is_finite() as i32 == 0 || ty.is_finite() as i32 == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     cairo_matrix_init_translate(&mut tmp, tx, ty);
//     cairo_matrix_multiply(&mut gstate.ctm, &mut tmp, &mut gstate.ctm);
//     gstate.is_identity = 0 as libc::c_int;
//     if _cairo_matrix_is_invertible(&mut gstate.ctm) == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     cairo_matrix_init_translate(&mut tmp, -tx, -ty);
//     cairo_matrix_multiply(&mut gstate.ctm_inverse, &mut gstate.ctm_inverse, &mut tmp);
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_scale(mut gstate: *mut GState, mut sx: f64, mut sy: f64) -> cairo_status_t {
//     let mut tmp: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     if sx * sy == 0.0f64 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     if sx.is_finite() as i32 == 0 || sy.is_finite() as i32 == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     cairo_matrix_init_scale(&mut tmp, sx, sy);
//     cairo_matrix_multiply(&mut gstate.ctm, &mut tmp, &mut gstate.ctm);
//     gstate.is_identity = 0 as libc::c_int;
//     if _cairo_matrix_is_invertible(&mut gstate.ctm) == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     cairo_matrix_init_scale(
//         &mut tmp,
//         1 as libc::c_int as f64 / sx,
//         1 as libc::c_int as f64 / sy,
//     );
//     cairo_matrix_multiply(&mut gstate.ctm_inverse, &mut gstate.ctm_inverse, &mut tmp);
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_rotate(mut gstate: *mut GState, mut angle: f64) -> cairo_status_t {
//     let mut tmp: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     if angle == 0.0f64 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if angle.is_finite() as i32 == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     cairo_matrix_init_rotate(&mut tmp, angle);
//     cairo_matrix_multiply(&mut gstate.ctm, &mut tmp, &mut gstate.ctm);
//     gstate.is_identity = 0 as libc::c_int;
//     if _cairo_matrix_is_invertible(&mut gstate.ctm) == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     cairo_matrix_init_rotate(&mut tmp, -angle);
//     cairo_matrix_multiply(&mut gstate.ctm_inverse, &mut gstate.ctm_inverse, &mut tmp);
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_transform(
//     mut gstate: *mut GState,
//     mut matrix: *const Matrix,
// ) -> cairo_status_t {
//     let mut tmp: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     if _cairo_matrix_is_invertible(matrix) == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     if _cairo_matrix_is_identity(matrix) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     tmp = *matrix;
//     status = cairo_matrix_invert(&mut tmp);
//     if status as u64 != 0 {
//         return status;
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     cairo_matrix_multiply(&mut gstate.ctm, matrix, &mut gstate.ctm);
//     cairo_matrix_multiply(&mut gstate.ctm_inverse, &mut gstate.ctm_inverse, &mut tmp);
//     gstate.is_identity = 0 as libc::c_int;
//     if _cairo_matrix_is_invertible(&mut gstate.ctm) == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_set_matrix(
//     mut gstate: *mut GState,
//     mut matrix: *const Matrix,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     if memcmp(
//         matrix as *const libc::c_void,
//         &mut gstate.ctm as *mut Matrix as *const libc::c_void,
//         ::std::mem::size_of::<Matrix>() as libc::c_ulong,
//     ) == 0 as libc::c_int
//     {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if _cairo_matrix_is_invertible(matrix) == 0 {
//         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
//     }
//     if _cairo_matrix_is_identity(matrix) != 0 {
//         _cairo_gstate_identity_matrix(gstate);
//         return CAIRO_STATUS_SUCCESS;
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     gstate.ctm = *matrix;
//     gstate.ctm_inverse = *matrix;
//     status = cairo_matrix_invert(&mut gstate.ctm_inverse);
//     if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint {
//     } else {
//         __assert_fail(
//             b"status == CAIRO_STATUS_SUCCESS\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo-gstate.c\0" as *const u8 as *const libc::c_char,
//             784 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 82], &[libc::c_char; 82]>(
//                 b"cairo_status_t _cairo_gstate_set_matrix(GState *, const Matrix *)\0",
//             ))
//             .as_ptr(),
//         );
//     }
//     gstate.is_identity = 0 as libc::c_int;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_identity_matrix(mut gstate: *mut GState) {
//     if _cairo_matrix_is_identity(&mut gstate.ctm) != 0 {
//         return;
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     cairo_matrix_init_identity(&mut gstate.ctm);
//     cairo_matrix_init_identity(&mut gstate.ctm_inverse);
//     gstate.is_identity = _cairo_matrix_is_identity(&mut (*gstate.target).device_transform);
// }

// pub fn _cairo_gstate_user_to_device(mut gstate: *mut GState, mut x: *mut f64, mut y: *mut f64) {
//     cairo_matrix_transform_point(&mut gstate.ctm, x, y);
// }

// Ok
pub fn _cairo_gstate_user_to_device_distance(gstate: &GState, dx: &mut f64, dy: &mut f64) {
    gstate.ctm.transform_distance(dx, dy);
}

// Ok
pub fn _cairo_gstate_device_to_user(gstate: &GState, x: &mut f64, y: &mut f64) {
    gstate.ctm_inverse.transform_point(x, y);
}

// Ok
pub fn _cairo_gstate_device_to_user_distance(gstate: &GState, dx: &mut f64, dy: &mut f64) {
    gstate.ctm_inverse.transform_distance(dx, dy);
}

// Ok
pub fn _do_cairo_gstate_user_to_backend(gstate: &GState, x: &mut f64, y: &mut f64) {
    gstate.ctm.transform_point(x, y);
    gstate.target.device_transform.transform_point(x, y);
}

// Ok
pub fn _do_cairo_gstate_user_to_backend_distance(gstate: &GState, x: &mut f64, y: &mut f64) {
    gstate.ctm.transform_distance(x, y);
    gstate.target.device_transform.transform_distance(x, y);
}

// Ok
pub fn _do_cairo_gstate_backend_to_user(gstate: &GState, x: &mut f64, y: &mut f64) {
    gstate.target.device_transform_inverse.transform_point(x, y);
    gstate.ctm_inverse.transform_point(x, y);
}

// Ok
pub fn _do_cairo_gstate_backend_to_user_distance(gstate: &GState, x: &mut f64, y: &mut f64) {
    gstate
        .target
        .device_transform_inverse
        .transform_distance(x, y);
    gstate.ctm_inverse.transform_distance(x, y);
}

// Ok
pub fn _cairo_gstate_backend_to_user_rectangle(
    gstate: &GState,
    x1: &mut f64,
    y1: &mut f64,
    x2: &mut f64,
    y2: &mut f64,
    is_tight: Option<&mut bool>,
) {
    if !gstate.target.device_transform_inverse.is_identity() || !gstate.ctm_inverse.is_identity() {
        let matrix_inverse =
            Matrix::multiply(&gstate.target.device_transform_inverse, &gstate.ctm_inverse);

        matrix_inverse.transform_bounding_box(x1, y1, x2, y2, is_tight);
    } else {
        if let Some(is_tight) = is_tight {
            *is_tight = true;
        }
    }
}

// Ok
pub fn _cairo_gstate_path_extents(
    mut gstate: *mut GState,
    mut path: *mut cairo_path_fixed_t,
    mut x1: Option<&mut f64>,
    mut y1: Option<&mut f64>,
    mut x2: Option<&mut f64>,
    mut y2: Option<&mut f64>,
) {
    let mut box_0: cairo_box_t = cairo_box_t {
        p1: cairo_point_t { x: 0, y: 0 },
        p2: cairo_point_t { x: 0, y: 0 },
    };
    let mut px1 = 0.0;
    let mut py1 = 0.0;
    let mut px2 = 0.0;
    let mut py2 = 0.0;

    if _cairo_path_fixed_extents(path, &mut box_0) {
        px1 = _cairo_fixed_to_double(box_0.p1.x);
        py1 = _cairo_fixed_to_double(box_0.p1.y);
        px2 = _cairo_fixed_to_double(box_0.p2.x);
        py2 = _cairo_fixed_to_double(box_0.p2.y);
        _cairo_gstate_backend_to_user_rectangle(
            gstate, &mut px1, &mut py1, &mut px2, &mut py2, None,
        );
    }

    if let Some(x1) = x1 {
        *x1 = px1;
    }
    if let Some(y1) = y1 {
        *y1 = py1;
    }
    if let Some(x2) = x2 {
        *x2 = px2;
    }
    if let Some(y2) = y2 {
        *y2 = py2;
    }
}

// fn _cairo_gstate_copy_pattern(mut pattern: *mut Pattern, mut original: *const Pattern) {
//     if _cairo_pattern_is_clear(original) != 0 {
//         _cairo_pattern_init_solid(
//             pattern as *mut cairo_solid_pattern_t,
//             _cairo_stock_color(CAIRO_STOCK_TRANSPARENT),
//         );
//         return;
//     }
//     if (*original).type_0 as libc::c_uint
//         == CAIRO_PATTERN_TYPE_LINEAR as libc::c_int as libc::c_uint
//         || (*original).type_0 as libc::c_uint
//             == CAIRO_PATTERN_TYPE_RADIAL as libc::c_int as libc::c_uint
//     {
//         let mut color: cairo_color_t = cairo_color_t {
//             red: 0.,
//             green: 0.,
//             blue: 0.,
//             alpha: 0.,
//             red_short: 0,
//             green_short: 0,
//             blue_short: 0,
//             alpha_short: 0,
//         };
//         if _cairo_gradient_pattern_is_solid(
//             original as *mut cairo_gradient_pattern_t,
//             0 as *const cairo_rectangle_int_t,
//             &mut color,
//         ) != 0
//         {
//             _cairo_pattern_init_solid(pattern as *mut cairo_solid_pattern_t, &mut color);
//             return;
//         }
//     }
//     _cairo_pattern_init_static_copy(pattern, original);
// }
// fn _cairo_gstate_copy_transformed_pattern(
//     mut gstate: *mut GState,
//     mut pattern: *mut Pattern,
//     mut original: *const Pattern,
//     mut ctm_inverse: *const Matrix,
// ) {
//     _cairo_gstate_copy_pattern(pattern, original);
//     if (*original).type_0 as libc::c_uint
//         == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
//     {
//         let mut surface_pattern: *mut cairo_surface_pattern_t = 0 as *mut cairo_surface_pattern_t;
//         let mut surface: *mut Surface = 0 as *mut Surface;
//         surface_pattern = original as *mut cairo_surface_pattern_t;
//         surface = (*surface_pattern).surface;
//         if _cairo_surface_has_device_transform(surface) != 0 {
//             _cairo_pattern_pretransform(pattern, &mut (*surface).device_transform);
//         }
//     }
//     if _cairo_matrix_is_identity(ctm_inverse) == 0 {
//         _cairo_pattern_transform(pattern, ctm_inverse);
//     }
//     if _cairo_surface_has_device_transform(gstate.target) != 0 {
//         _cairo_pattern_transform(pattern, &mut (*gstate.target).device_transform_inverse);
//     }
// }
// fn _cairo_gstate_copy_transformed_source(mut gstate: *mut GState, mut pattern: *mut Pattern) {
//     _cairo_gstate_copy_transformed_pattern(
//         gstate,
//         pattern,
//         gstate.source,
//         &mut gstate.source_ctm_inverse,
//     );
// }
// fn _cairo_gstate_copy_transformed_mask(
//     mut gstate: *mut GState,
//     mut pattern: *mut Pattern,
//     mut mask: *mut Pattern,
// ) {
//     _cairo_gstate_copy_transformed_pattern(gstate, pattern, mask, &mut gstate.ctm_inverse);
// }
// fn _reduce_op(mut gstate: *mut GState) -> Operator {
//     let mut op: Operator = CAIRO_OPERATOR_CLEAR;
//     let mut pattern: *const Pattern = 0 as *const Pattern;
//     op = gstate.op;
//     if op as libc::c_uint != CAIRO_OPERATOR_SOURCE as libc::c_int as libc::c_uint {
//         return op;
//     }
//     pattern = gstate.source;
//     if (*pattern).type_0 as libc::c_uint == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
//     {
//         let mut solid: *const cairo_solid_pattern_t = pattern as *mut cairo_solid_pattern_t;
//         if (*solid).color.alpha_short as libc::c_int <= 0xff as libc::c_int {
//             op = CAIRO_OPERATOR_CLEAR;
//         } else if (*gstate.target).content as libc::c_uint
//             & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
//             == 0 as libc::c_int as libc::c_uint
//         {
//             if (*solid).color.red_short as libc::c_int
//                 | (*solid).color.green_short as libc::c_int
//                 | (*solid).color.blue_short as libc::c_int
//                 <= 0xff as libc::c_int
//             {
//                 op = CAIRO_OPERATOR_CLEAR;
//             }
//         }
//     } else if (*pattern).type_0 as libc::c_uint
//         == CAIRO_PATTERN_TYPE_SURFACE as libc::c_int as libc::c_uint
//     {
//         let mut surface: *const cairo_surface_pattern_t = pattern as *mut cairo_surface_pattern_t;
//         if (*(*surface).surface).is_clear() as libc::c_int != 0
//             && (*(*surface).surface).content as libc::c_uint
//                 & CAIRO_CONTENT_ALPHA as libc::c_int as libc::c_uint
//                 != 0
//         {
//             op = CAIRO_OPERATOR_CLEAR;
//         }
//     } else {
//         let mut gradient: *const cairo_gradient_pattern_t =
//             pattern as *mut cairo_gradient_pattern_t;
//         if (*gradient).n_stops == 0 as libc::c_int as libc::c_uint {
//             op = CAIRO_OPERATOR_CLEAR;
//         }
//     }
//     return op;
// }
// fn _cairo_gstate_get_pattern_status(mut pattern: *const Pattern) -> cairo_status_t {
//     if (*pattern).type_0 as libc::c_uint == CAIRO_PATTERN_TYPE_MESH as libc::c_int as libc::c_uint
//         && !((*(pattern as *const cairo_mesh_pattern_t)).current_patch).is_null()
//     {
//         return CAIRO_STATUS_INVALID_MESH_CONSTRUCTION;
//     }
//     return (*pattern).status;
// }

// pub fn _cairo_gstate_paint(mut gstate: *mut GState) -> cairo_status_t {
//     let mut source_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
//         base: Pattern {
//             ref_count: cairo_reference_count_t { ref_count: 0 },
//             status: CAIRO_STATUS_SUCCESS,
//             user_data: cairo_user_data_array_t {
//                 size: 0,
//                 num_elements: 0,
//                 element_size: 0,
//                 elements: 0 as *const libc::c_char as *mut libc::c_char,
//             },
//             observers: cairo_list_t {
//                 next: 0 as *const _cairo_list as *mut _cairo_list,
//                 prev: 0 as *const _cairo_list as *mut _cairo_list,
//             },
//             type_0: CAIRO_PATTERN_TYPE_SOLID,
//             filter: CAIRO_FILTER_FAST,
//             extend: CAIRO_EXTEND_NONE,
//             has_component_alpha: 0,
//             is_userfont_foreground: 0,
//             matrix: Matrix {
//                 xx: 0.,
//                 yx: 0.,
//                 xy: 0.,
//                 yy: 0.,
//                 x0: 0.,
//                 y0: 0.,
//             },
//             opacity: 0.,
//         },
//     };
//     let mut pattern: *const Pattern = 0 as *const Pattern;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     let mut op: Operator = CAIRO_OPERATOR_CLEAR;
//     status = _cairo_gstate_get_pattern_status(gstate.source);
//     if status as u64 != 0 {
//         return status;
//     }
//     if gstate.op as libc::c_uint == CAIRO_OPERATOR_DEST as libc::c_int as libc::c_uint {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if _cairo_clip_is_all_clipped(gstate.clip) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     op = _reduce_op(gstate);
//     if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
//         pattern = &_cairo_pattern_clear.base;
//     } else {
//         _cairo_gstate_copy_transformed_source(gstate, &mut source_pattern.base);
//         pattern = &mut source_pattern.base;
//     }
//     return _cairo_surface_paint(gstate.target, op, pattern, gstate.clip);
// }

// pub fn _cairo_gstate_mask(mut gstate: *mut GState, mut mask: *mut Pattern) -> cairo_status_t {
//     let mut source_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
//         base: Pattern {
//             ref_count: cairo_reference_count_t { ref_count: 0 },
//             status: CAIRO_STATUS_SUCCESS,
//             user_data: cairo_user_data_array_t {
//                 size: 0,
//                 num_elements: 0,
//                 element_size: 0,
//                 elements: 0 as *const libc::c_char as *mut libc::c_char,
//             },
//             observers: cairo_list_t {
//                 next: 0 as *const _cairo_list as *mut _cairo_list,
//                 prev: 0 as *const _cairo_list as *mut _cairo_list,
//             },
//             type_0: CAIRO_PATTERN_TYPE_SOLID,
//             filter: CAIRO_FILTER_FAST,
//             extend: CAIRO_EXTEND_NONE,
//             has_component_alpha: 0,
//             is_userfont_foreground: 0,
//             matrix: Matrix {
//                 xx: 0.,
//                 yx: 0.,
//                 xy: 0.,
//                 yy: 0.,
//                 x0: 0.,
//                 y0: 0.,
//             },
//             opacity: 0.,
//         },
//     };
//     let mut mask_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
//         base: Pattern {
//             ref_count: cairo_reference_count_t { ref_count: 0 },
//             status: CAIRO_STATUS_SUCCESS,
//             user_data: cairo_user_data_array_t {
//                 size: 0,
//                 num_elements: 0,
//                 element_size: 0,
//                 elements: 0 as *const libc::c_char as *mut libc::c_char,
//             },
//             observers: cairo_list_t {
//                 next: 0 as *const _cairo_list as *mut _cairo_list,
//                 prev: 0 as *const _cairo_list as *mut _cairo_list,
//             },
//             type_0: CAIRO_PATTERN_TYPE_SOLID,
//             filter: CAIRO_FILTER_FAST,
//             extend: CAIRO_EXTEND_NONE,
//             has_component_alpha: 0,
//             is_userfont_foreground: 0,
//             matrix: Matrix {
//                 xx: 0.,
//                 yx: 0.,
//                 xy: 0.,
//                 yy: 0.,
//                 x0: 0.,
//                 y0: 0.,
//             },
//             opacity: 0.,
//         },
//     };
//     let mut source: *const Pattern = 0 as *const Pattern;
//     let mut op: Operator = CAIRO_OPERATOR_CLEAR;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_get_pattern_status(mask);
//     if status as u64 != 0 {
//         return status;
//     }
//     status = _cairo_gstate_get_pattern_status(gstate.source);
//     if status as u64 != 0 {
//         return status;
//     }
//     if gstate.op as libc::c_uint == CAIRO_OPERATOR_DEST as libc::c_int as libc::c_uint {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if _cairo_clip_is_all_clipped(gstate.clip) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if gstate.opacity == 1.0f64 {
//     } else {
//         __assert_fail(
//             b"gstate->opacity == 1.0\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo-gstate.c\0" as *const u8 as *const libc::c_char,
//             1128 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
//                 b"cairo_status_t _cairo_gstate_mask(GState *, Pattern *)\0",
//             ))
//             .as_ptr(),
//         );
//     }
//     if _cairo_pattern_is_opaque(mask, 0 as *const cairo_rectangle_int_t) != 0 {
//         return _cairo_gstate_paint(gstate);
//     }
//     if _cairo_pattern_is_clear(mask) != 0 && _cairo_operator_bounded_by_mask(gstate.op) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     op = _reduce_op(gstate);
//     if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
//         source = &_cairo_pattern_clear.base;
//     } else {
//         _cairo_gstate_copy_transformed_source(gstate, &mut source_pattern.base);
//         source = &mut source_pattern.base;
//     }
//     _cairo_gstate_copy_transformed_mask(gstate, &mut mask_pattern.base, mask);
//     if (*source).type_0 as libc::c_uint == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
//         && mask_pattern.base.type_0 as libc::c_uint
//             == CAIRO_PATTERN_TYPE_SOLID as libc::c_int as libc::c_uint
//         && _cairo_operator_bounded_by_source(op) != 0
//     {
//         let mut solid: *const cairo_solid_pattern_t = source as *mut cairo_solid_pattern_t;
//         let mut combined: cairo_color_t = cairo_color_t {
//             red: 0.,
//             green: 0.,
//             blue: 0.,
//             alpha: 0.,
//             red_short: 0,
//             green_short: 0,
//             blue_short: 0,
//             alpha_short: 0,
//         };
//         if mask_pattern.base.has_component_alpha != 0 {
//             combined.red = (*solid).color.red * mask_pattern.solid.color.red;
//             combined.green = (*solid).color.green * mask_pattern.solid.color.green;
//             combined.blue = (*solid).color.blue * mask_pattern.solid.color.blue;
//             combined.alpha = (*solid).color.alpha * mask_pattern.solid.color.alpha;
//         } else {
//             combined = (*solid).color;
//             _cairo_color_multiply_alpha(&mut combined, mask_pattern.solid.color.alpha);
//         }
//         _cairo_pattern_init_solid(&mut source_pattern.solid, &mut combined);
//         status = _cairo_surface_paint(gstate.target, op, &mut source_pattern.base, gstate.clip);
//     } else {
//         status = _cairo_surface_mask(
//             gstate.target,
//             op,
//             source,
//             &mut mask_pattern.base,
//             gstate.clip,
//         );
//     }
//     return status;
// }

// pub fn _cairo_gstate_stroke(
//     mut gstate: *mut GState,
//     mut path: *mut cairo_path_fixed_t,
// ) -> cairo_status_t {
//     let mut source_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
//         base: Pattern {
//             ref_count: cairo_reference_count_t { ref_count: 0 },
//             status: CAIRO_STATUS_SUCCESS,
//             user_data: cairo_user_data_array_t {
//                 size: 0,
//                 num_elements: 0,
//                 element_size: 0,
//                 elements: 0 as *const libc::c_char as *mut libc::c_char,
//             },
//             observers: cairo_list_t {
//                 next: 0 as *const _cairo_list as *mut _cairo_list,
//                 prev: 0 as *const _cairo_list as *mut _cairo_list,
//             },
//             type_0: CAIRO_PATTERN_TYPE_SOLID,
//             filter: CAIRO_FILTER_FAST,
//             extend: CAIRO_EXTEND_NONE,
//             has_component_alpha: 0,
//             is_userfont_foreground: 0,
//             matrix: Matrix {
//                 xx: 0.,
//                 yx: 0.,
//                 xy: 0.,
//                 yy: 0.,
//                 x0: 0.,
//                 y0: 0.,
//             },
//             opacity: 0.,
//         },
//     };
//     let mut style: StrokeStyle = StrokeStyle {
//         line_width: 0.,
//         line_cap: CAIRO_LINE_CAP_BUTT,
//         line_join: CAIRO_LINE_JOIN_MITER,
//         miter_limit: 0.,
//         dash: 0 as *mut f64,
//         num_dashes: 0,
//         dash_offset: 0.,
//         is_hairline: 0,
//         pre_hairline_line_width: 0.,
//     };
//     let mut dash: [f64; 2] = [0.; 2];
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     let mut aggregate_transform: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     let mut aggregate_transform_inverse: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     status = _cairo_gstate_get_pattern_status(gstate.source);
//     if status as u64 != 0 {
//         return status;
//     }
//     if gstate.op as libc::c_uint == CAIRO_OPERATOR_DEST as libc::c_int as libc::c_uint {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if gstate.stroke_style.line_width <= 0.0f64 && gstate.stroke_style.is_hairline == 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if _cairo_clip_is_all_clipped(gstate.clip) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if gstate.opacity == 1.0f64 {
//     } else {
//         __assert_fail(
//             b"gstate->opacity == 1.0\0" as *const u8 as *const libc::c_char,
//             b"../src/cairo-gstate.c\0" as *const u8 as *const libc::c_char,
//             1207 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 76], &[libc::c_char; 76]>(
//                 b"cairo_status_t _cairo_gstate_stroke(GState *, cairo_path_fixed_t *)\0",
//             ))
//             .as_ptr(),
//         );
//     }
//     cairo_matrix_multiply(
//         &mut aggregate_transform,
//         &mut gstate.ctm,
//         &mut (*gstate.target).device_transform,
//     );
//     cairo_matrix_multiply(
//         &mut aggregate_transform_inverse,
//         &mut (*gstate.target).device_transform_inverse,
//         &mut gstate.ctm_inverse,
//     );
//     memcpy(
//         &mut style as *mut StrokeStyle as *mut libc::c_void,
//         &mut gstate.stroke_style as *mut StrokeStyle as *const libc::c_void,
//         ::std::mem::size_of::<StrokeStyle>() as libc::c_ulong,
//     );
//     if _cairo_stroke_style_dash_can_approximate(
//         &mut gstate.stroke_style,
//         &mut aggregate_transform,
//         gstate.tolerance,
//     ) != 0
//     {
//         style.dash = dash.as_mut_ptr();
//         _cairo_stroke_style_dash_approximate(
//             &mut gstate.stroke_style,
//             &mut gstate.ctm,
//             gstate.tolerance,
//             &mut style.dash_offset,
//             style.dash,
//             &mut style.num_dashes,
//         );
//     }
//     _cairo_gstate_copy_transformed_source(gstate, &mut source_pattern.base);
//     return _cairo_surface_stroke(
//         gstate.target,
//         gstate.op,
//         &mut source_pattern.base,
//         path,
//         &mut style,
//         &mut aggregate_transform,
//         &mut aggregate_transform_inverse,
//         gstate.tolerance,
//         gstate.antialias,
//         gstate.clip,
//     );
// }

// pub fn _cairo_gstate_in_stroke(
//     mut gstate: *mut GState,
//     mut path: *mut cairo_path_fixed_t,
//     mut x: f64,
//     mut y: f64,
//     mut inside_ret: *mut bool,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
//         x: 0,
//         y: 0,
//         width: 0,
//         height: 0,
//     };
//     let mut limit: cairo_box_t = cairo_box_t {
//         p1: cairo_point_t { x: 0, y: 0 },
//         p2: cairo_point_t { x: 0, y: 0 },
//     };
//     let mut traps: cairo_traps_t = cairo_traps_t {
//         status: CAIRO_STATUS_SUCCESS,
//         bounds: cairo_box_t {
//             p1: cairo_point_t { x: 0, y: 0 },
//             p2: cairo_point_t { x: 0, y: 0 },
//         },
//         limits: 0 as *const cairo_box_t,
//         num_limits: 0,
//         maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
//         c2rust_padding: [0; 3],
//         num_traps: 0,
//         traps_size: 0,
//         traps: 0 as *mut cairo_trapezoid_t,
//         traps_embedded: [cairo_trapezoid_t {
//             top: 0,
//             bottom: 0,
//             left: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//             right: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//         }; 16],
//     };
//     if gstate.stroke_style.line_width <= 0.0f64 {
//         *inside_ret = 0 as libc::c_int;
//         return CAIRO_STATUS_SUCCESS;
//     }
//     _cairo_gstate_user_to_backend(gstate, &mut x, &mut y);
//     _cairo_path_fixed_approximate_stroke_extents(
//         path,
//         &mut gstate.stroke_style,
//         &mut gstate.ctm,
//         (*gstate.target).is_vector() as bool,
//         &mut extents,
//     );
//     if x < extents.x as f64
//         || x > (extents.x + extents.width) as f64
//         || y < extents.y as f64
//         || y > (extents.y + extents.height) as f64
//     {
//         *inside_ret = 0 as libc::c_int;
//         return CAIRO_STATUS_SUCCESS;
//     }
//     limit.p1.x = _cairo_fixed_from_double(x) - 1 as libc::c_int;
//     limit.p1.y = _cairo_fixed_from_double(y) - 1 as libc::c_int;
//     limit.p2.x = limit.p1.x + 2 as libc::c_int;
//     limit.p2.y = limit.p1.y + 2 as libc::c_int;
//     _cairo_traps_init(&mut traps);
//     _cairo_traps_limit(&mut traps, &mut limit, 1 as libc::c_int);
//     status = _cairo_path_fixed_stroke_polygon_to_traps(
//         path,
//         &mut gstate.stroke_style,
//         &mut gstate.ctm,
//         &mut gstate.ctm_inverse,
//         gstate.tolerance,
//         &mut traps,
//     ) as cairo_status_t;
//     if !(status as u64 != 0) {
//         *inside_ret = _cairo_traps_contain(&mut traps, x, y);
//     }
//     _cairo_traps_fini(&mut traps);
//     return status;
// }

// pub fn fill(gstate: &mut Self, path: *mut cairo_path_fixed_t) -> Result<()> {
//     _cairo_gstate_get_pattern_status(gstate.source)?;
//     if matches(gstate.op, CAIRO_OPERATOR_DEST) {
//         return Ok(());
//     }
//     if gstate.clip.is_all_clipped() {
//         return Ok(());
//     }

//     assert!(gstate.opacity == 1.0);

//     if _cairo_path_fixed_fill_is_empty(path) != 0 {
//         if _cairo_operator_bounded_by_mask(gstate.op) != 0 {
//             return CAIRO_STATUS_SUCCESS;
//         }
//         status = _cairo_surface_paint(
//             gstate.target,
//             OPERATOR::CLEAR,
//             &_cairo_pattern_clear.base,
//             gstate.clip,
//         );
//     } else {
//         let mut source_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
//             base: Pattern {
//                 ref_count: cairo_reference_count_t { ref_count: 0 },
//                 status: CAIRO_STATUS_SUCCESS,
//                 user_data: cairo_user_data_array_t {
//                     size: 0,
//                     num_elements: 0,
//                     element_size: 0,
//                     elements: 0 as *const libc::c_char as *mut libc::c_char,
//                 },
//                 observers: cairo_list_t {
//                     next: 0 as *const _cairo_list as *mut _cairo_list,
//                     prev: 0 as *const _cairo_list as *mut _cairo_list,
//                 },
//                 type_0: CAIRO_PATTERN_TYPE_SOLID,
//                 filter: CAIRO_FILTER_FAST,
//                 extend: CAIRO_EXTEND_NONE,
//                 has_component_alpha: 0,
//                 is_userfont_foreground: 0,
//                 matrix: Matrix {
//                     xx: 0.,
//                     yx: 0.,
//                     xy: 0.,
//                     yy: 0.,
//                     x0: 0.,
//                     y0: 0.,
//                 },
//                 opacity: 0.,
//             },
//         };
//         let mut pattern: *const Pattern = 0 as *const Pattern;
//         let mut op: Operator = CAIRO_OPERATOR_CLEAR;
//         let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
//             x: 0,
//             y: 0,
//             width: 0,
//             height: 0,
//         };
//         let mut box_0: cairo_box_t = cairo_box_t {
//             p1: cairo_point_t { x: 0, y: 0 },
//             p2: cairo_point_t { x: 0, y: 0 },
//         };
//         op = _reduce_op(gstate);
//         if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
//             pattern = &_cairo_pattern_clear.base;
//         } else {
//             _cairo_gstate_copy_transformed_source(gstate, &mut source_pattern.base);
//             pattern = &mut source_pattern.base;
//         }
//         if _cairo_surface_get_extents(gstate.target, &mut extents) != 0
//             && _cairo_path_fixed_is_box(path, &mut box_0) != 0
//             && box_0.p1.x <= _cairo_fixed_from_int(extents.x)
//             && box_0.p1.y <= _cairo_fixed_from_int(extents.y)
//             && box_0.p2.x >= _cairo_fixed_from_int(extents.x + extents.width)
//             && box_0.p2.y >= _cairo_fixed_from_int(extents.y + extents.height)
//         {
//             status = _cairo_surface_paint(gstate.target, op, pattern, gstate.clip);
//         } else {
//             status = _cairo_surface_fill(
//                 gstate.target,
//                 op,
//                 pattern,
//                 path,
//                 gstate.fill_rule,
//                 gstate.tolerance,
//                 gstate.antialias,
//                 gstate.clip,
//             );
//         }
//     }
//     return status;
// }

// pub fn _cairo_gstate_in_fill(
//     mut gstate: *mut GState,
//     mut path: *mut cairo_path_fixed_t,
//     mut x: f64,
//     mut y: f64,
// ) -> bool {
//     _cairo_gstate_user_to_backend(gstate, &mut x, &mut y);
//     return _cairo_path_fixed_in_fill(path, gstate.fill_rule, gstate.tolerance, x, y);
// }

// pub fn _cairo_gstate_in_clip(mut gstate: *mut GState, mut x: f64, mut y: f64) -> bool {
//     let mut clip: *mut Clip = gstate.clip;
//     let mut i: libc::c_int = 0;
//     if _cairo_clip_is_all_clipped(clip) != 0 {
//         return 0 as libc::c_int;
//     }
//     if clip.is_null() {
//         return 1 as libc::c_int;
//     }
//     _cairo_gstate_user_to_backend(gstate, &mut x, &mut y);
//     if x < (*clip).extents.x as f64
//         || x >= ((*clip).extents.x + (*clip).extents.width) as f64
//         || y < (*clip).extents.y as f64
//         || y >= ((*clip).extents.y + (*clip).extents.height) as f64
//     {
//         return 0 as libc::c_int;
//     }
//     if (*clip).num_boxes != 0 {
//         let mut fx: libc::c_int = 0;
//         let mut fy: libc::c_int = 0;
//         fx = _cairo_fixed_from_double(x);
//         fy = _cairo_fixed_from_double(y);
//         i = 0 as libc::c_int;
//         while i < (*clip).num_boxes {
//             if fx >= (*((*clip).boxes).offset(i as isize)).p1.x
//                 && fx <= (*((*clip).boxes).offset(i as isize)).p2.x
//                 && fy >= (*((*clip).boxes).offset(i as isize)).p1.y
//                 && fy <= (*((*clip).boxes).offset(i as isize)).p2.y
//             {
//                 break;
//             }
//             i += 1;
//         }
//         if i == (*clip).num_boxes {
//             return 0 as libc::c_int;
//         }
//     }
//     if !((*clip).path).is_null() {
//         let mut clip_path: *mut cairo_clip_path_t = (*clip).path;
//         loop {
//             if _cairo_path_fixed_in_fill(
//                 &mut (*clip_path).path,
//                 (*clip_path).fill_rule,
//                 (*clip_path).tolerance,
//                 x,
//                 y,
//             ) == 0
//             {
//                 return 0 as libc::c_int;
//             }
//             clip_path = (*clip_path).prev;
//             if clip_path.is_null() {
//                 break;
//             }
//         }
//     }
//     return 1 as libc::c_int;
// }

// pub fn _cairo_gstate_copy_page(mut gstate: *mut GState) -> cairo_status_t {
//     cairo_surface_copy_page(gstate.target);
//     return cairo_surface_status(gstate.target);
// }

// pub fn _cairo_gstate_show_page(mut gstate: *mut GState) -> cairo_status_t {
//     cairo_surface_show_page(gstate.target);
//     return cairo_surface_status(gstate.target);
// }
// fn _cairo_gstate_extents_to_user_rectangle(
//     mut gstate: *mut GState,
//     mut extents: *const cairo_box_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) {
//     let mut px1: f64 = 0.;
//     let mut py1: f64 = 0.;
//     let mut px2: f64 = 0.;
//     let mut py2: f64 = 0.;
//     px1 = _cairo_fixed_to_double((*extents).p1.x);
//     py1 = _cairo_fixed_to_double((*extents).p1.y);
//     px2 = _cairo_fixed_to_double((*extents).p2.x);
//     py2 = _cairo_fixed_to_double((*extents).p2.y);
//     _cairo_gstate_backend_to_user_rectangle(
//         gstate,
//         &mut px1,
//         &mut py1,
//         &mut px2,
//         &mut py2,
//         0 as *mut bool,
//     );
//     if !x1.is_null() {
//         *x1 = px1;
//     }
//     if !y1.is_null() {
//         *y1 = py1;
//     }
//     if !x2.is_null() {
//         *x2 = px2;
//     }
//     if !y2.is_null() {
//         *y2 = py2;
//     }
// }

// pub fn _cairo_gstate_stroke_extents(
//     mut gstate: *mut GState,
//     mut path: *mut cairo_path_fixed_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) -> cairo_status_t {
//     let mut status: cairo_int_status_t = CAIRO_INT_STATUS_SUCCESS;
//     let mut extents: cairo_box_t = cairo_box_t {
//         p1: cairo_point_t { x: 0, y: 0 },
//         p2: cairo_point_t { x: 0, y: 0 },
//     };
//     let mut empty: bool = 0;
//     if !x1.is_null() {
//         *x1 = 0.0f64;
//     }
//     if !y1.is_null() {
//         *y1 = 0.0f64;
//     }
//     if !x2.is_null() {
//         *x2 = 0.0f64;
//     }
//     if !y2.is_null() {
//         *y2 = 0.0f64;
//     }
//     if gstate.stroke_style.line_width <= 0.0f64 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     status = CAIRO_INT_STATUS_UNSUPPORTED;
//     if _cairo_path_fixed_stroke_is_rectilinear(path) != 0 {
//         let mut boxes: cairo_boxes_t = cairo_boxes_t {
//             status: CAIRO_STATUS_SUCCESS,
//             limit: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//             limits: 0 as *const cairo_box_t,
//             num_limits: 0,
//             num_boxes: 0,
//             is_pixel_aligned: 0,
//             chunks: _cairo_boxes_chunk {
//                 next: 0 as *mut _cairo_boxes_chunk,
//                 base: 0 as *mut cairo_box_t,
//                 count: 0,
//                 size: 0,
//             },
//             tail: 0 as *mut _cairo_boxes_chunk,
//             boxes_embedded: [cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             }; 32],
//         };
//         _cairo_boxes_init(&mut boxes);
//         status = _cairo_path_fixed_stroke_rectilinear_to_boxes(
//             path,
//             &mut gstate.stroke_style,
//             &mut gstate.ctm,
//             gstate.antialias,
//             &mut boxes,
//         );
//         empty = (boxes.num_boxes == 0 as libc::c_int) as libc::c_int;
//         if empty == 0 {
//             _cairo_boxes_extents(&mut boxes, &mut extents);
//         }
//         _cairo_boxes_fini(&mut boxes);
//     }
//     if status as libc::c_uint == CAIRO_INT_STATUS_UNSUPPORTED as libc::c_int as libc::c_uint {
//         let mut polygon: cairo_polygon_t = cairo_polygon_t {
//             status: CAIRO_STATUS_SUCCESS,
//             extents: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//             limit: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//             limits: 0 as *const cairo_box_t,
//             num_limits: 0,
//             num_edges: 0,
//             edges_size: 0,
//             edges: 0 as *mut cairo_edge_t,
//             edges_embedded: [cairo_edge_t {
//                 line: cairo_box_t {
//                     p1: cairo_point_t { x: 0, y: 0 },
//                     p2: cairo_point_t { x: 0, y: 0 },
//                 },
//                 top: 0,
//                 bottom: 0,
//                 dir: 0,
//             }; 32],
//         };
//         _cairo_polygon_init(&mut polygon, 0 as *const cairo_box_t, 0 as libc::c_int);
//         status = _cairo_path_fixed_stroke_to_polygon(
//             path,
//             &mut gstate.stroke_style,
//             &mut gstate.ctm,
//             &mut gstate.ctm_inverse,
//             gstate.tolerance,
//             &mut polygon,
//         ) as cairo_int_status_t;
//         empty = (polygon.num_edges == 0 as libc::c_int) as libc::c_int;
//         if empty == 0 {
//             extents = polygon.extents;
//         }
//         _cairo_polygon_fini(&mut polygon);
//     }
//     if empty == 0 {
//         _cairo_gstate_extents_to_user_rectangle(gstate, &mut extents, x1, y1, x2, y2);
//     }
//     return status as cairo_status_t;
// }

// pub fn _cairo_gstate_fill_extents(
//     mut gstate: *mut GState,
//     mut path: *mut cairo_path_fixed_t,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     let mut extents: cairo_box_t = cairo_box_t {
//         p1: cairo_point_t { x: 0, y: 0 },
//         p2: cairo_point_t { x: 0, y: 0 },
//     };
//     let mut empty: bool = 0;
//     if !x1.is_null() {
//         *x1 = 0.0f64;
//     }
//     if !y1.is_null() {
//         *y1 = 0.0f64;
//     }
//     if !x2.is_null() {
//         *x2 = 0.0f64;
//     }
//     if !y2.is_null() {
//         *y2 = 0.0f64;
//     }
//     if _cairo_path_fixed_fill_is_empty(path) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if _cairo_path_fixed_fill_is_rectilinear(path) != 0 {
//         let mut boxes: cairo_boxes_t = cairo_boxes_t {
//             status: CAIRO_STATUS_SUCCESS,
//             limit: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//             limits: 0 as *const cairo_box_t,
//             num_limits: 0,
//             num_boxes: 0,
//             is_pixel_aligned: 0,
//             chunks: _cairo_boxes_chunk {
//                 next: 0 as *mut _cairo_boxes_chunk,
//                 base: 0 as *mut cairo_box_t,
//                 count: 0,
//                 size: 0,
//             },
//             tail: 0 as *mut _cairo_boxes_chunk,
//             boxes_embedded: [cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             }; 32],
//         };
//         _cairo_boxes_init(&mut boxes);
//         status = _cairo_path_fixed_fill_rectilinear_to_boxes(
//             path,
//             gstate.fill_rule,
//             gstate.antialias,
//             &mut boxes,
//         );
//         empty = (boxes.num_boxes == 0 as libc::c_int) as libc::c_int;
//         if empty == 0 {
//             _cairo_boxes_extents(&mut boxes, &mut extents);
//         }
//         _cairo_boxes_fini(&mut boxes);
//     } else {
//         let mut traps: cairo_traps_t = cairo_traps_t {
//             status: CAIRO_STATUS_SUCCESS,
//             bounds: cairo_box_t {
//                 p1: cairo_point_t { x: 0, y: 0 },
//                 p2: cairo_point_t { x: 0, y: 0 },
//             },
//             limits: 0 as *const cairo_box_t,
//             num_limits: 0,
//             maybe_region_has_intersections_is_rectilinear_is_rectangular: [0; 1],
//             c2rust_padding: [0; 3],
//             num_traps: 0,
//             traps_size: 0,
//             traps: 0 as *mut cairo_trapezoid_t,
//             traps_embedded: [cairo_trapezoid_t {
//                 top: 0,
//                 bottom: 0,
//                 left: cairo_box_t {
//                     p1: cairo_point_t { x: 0, y: 0 },
//                     p2: cairo_point_t { x: 0, y: 0 },
//                 },
//                 right: cairo_box_t {
//                     p1: cairo_point_t { x: 0, y: 0 },
//                     p2: cairo_point_t { x: 0, y: 0 },
//                 },
//             }; 16],
//         };
//         _cairo_traps_init(&mut traps);
//         status =
//             _cairo_path_fixed_fill_to_traps(path, gstate.fill_rule, gstate.tolerance, &mut traps);
//         empty = (traps.num_traps == 0 as libc::c_int) as libc::c_int;
//         if empty == 0 {
//             _cairo_traps_extents(&mut traps, &mut extents);
//         }
//         _cairo_traps_fini(&mut traps);
//     }
//     if empty == 0 {
//         _cairo_gstate_extents_to_user_rectangle(gstate, &mut extents, x1, y1, x2, y2);
//     }
//     return status;
// }

// pub fn _cairo_gstate_reset_clip(mut gstate: *mut GState) -> cairo_status_t {
//     _cairo_clip_destroy(gstate.clip);
//     let ref mut fresh51 = gstate.clip;
//     *fresh51 = 0 as *mut Clip;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_clip(
//     mut gstate: *mut GState,
//     mut path: *mut cairo_path_fixed_t,
// ) -> cairo_status_t {
//     let ref mut fresh52 = gstate.clip;
//     *fresh52 = _cairo_clip_intersect_path(
//         gstate.clip,
//         path,
//         gstate.fill_rule,
//         gstate.tolerance,
//         gstate.antialias,
//     );
//     return CAIRO_STATUS_SUCCESS;
// }
// fn _cairo_gstate_int_clip_extents(
//     mut gstate: *mut GState,
//     mut extents: *mut cairo_rectangle_int_t,
// ) -> bool {
//     let mut is_bounded: bool = 0;
//     is_bounded = _cairo_surface_get_extents(gstate.target, extents);
//     if !(gstate.clip).is_null() {
//         _cairo_rectangle_intersect(extents, _cairo_clip_get_extents(gstate.clip));
//         is_bounded = 1 as libc::c_int;
//     }
//     return is_bounded;
// }

// pub fn _cairo_gstate_clip_extents(
//     mut gstate: *mut GState,
//     mut x1: *mut f64,
//     mut y1: *mut f64,
//     mut x2: *mut f64,
//     mut y2: *mut f64,
// ) -> bool {
//     let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
//         x: 0,
//         y: 0,
//         width: 0,
//         height: 0,
//     };
//     let mut px1: f64 = 0.;
//     let mut py1: f64 = 0.;
//     let mut px2: f64 = 0.;
//     let mut py2: f64 = 0.;
//     if _cairo_gstate_int_clip_extents(gstate, &mut extents) == 0 {
//         return 0 as libc::c_int;
//     }
//     px1 = extents.x as f64;
//     py1 = extents.y as f64;
//     px2 = (extents.x + extents.width) as f64;
//     py2 = (extents.y + extents.height) as f64;
//     _cairo_gstate_backend_to_user_rectangle(
//         gstate,
//         &mut px1,
//         &mut py1,
//         &mut px2,
//         &mut py2,
//         0 as *mut bool,
//     );
//     if !x1.is_null() {
//         *x1 = px1;
//     }
//     if !y1.is_null() {
//         *y1 = py1;
//     }
//     if !x2.is_null() {
//         *x2 = px2;
//     }
//     if !y2.is_null() {
//         *y2 = py2;
//     }
//     return 1 as libc::c_int;
// }

// pub fn _cairo_gstate_copy_clip_rectangle_list(
//     mut gstate: *mut GState,
// ) -> *mut cairo_rectangle_list_t {
//     let mut extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
//         x: 0,
//         y: 0,
//         width: 0,
//         height: 0,
//     };
//     let mut list: *mut cairo_rectangle_list_t = 0 as *mut cairo_rectangle_list_t;
//     let mut clip: *mut Clip = 0 as *mut Clip;
//     if _cairo_surface_get_extents(gstate.target, &mut extents) != 0 {
//         clip = _cairo_clip_copy_intersect_rectangle(gstate.clip, &mut extents);
//     } else {
//         clip = gstate.clip;
//     }
//     list = _cairo_clip_copy_rectangle_list(clip, gstate);
//     if clip != gstate.clip {
//         _cairo_clip_destroy(clip);
//     }
//     return list;
// }

// pub fn _cairo_gstate_tag_begin(
//     mut gstate: *mut GState,
//     mut tag_name: *const libc::c_char,
//     mut attributes: *const libc::c_char,
// ) -> cairo_status_t {
//     return _cairo_surface_tag(
//         gstate.target,
//         1 as libc::c_int,
//         tag_name,
//         if !attributes.is_null() {
//             attributes
//         } else {
//             b"\0" as *const u8 as *const libc::c_char
//         },
//     );
// }

// pub fn _cairo_gstate_tag_end(
//     mut gstate: *mut GState,
//     mut tag_name: *const libc::c_char,
// ) -> cairo_status_t {
//     return _cairo_surface_tag(
//         gstate.target,
//         0 as libc::c_int,
//         tag_name,
//         0 as *const libc::c_char,
//     );
// }
// fn _cairo_gstate_unset_scaled_font(mut gstate: *mut GState) {
//     if (gstate.scaled_font).is_null() {
//         return;
//     }
//     if !(gstate.previous_scaled_font).is_null() {
//         cairo_scaled_font_destroy(gstate.previous_scaled_font);
//     }
//     let ref mut fresh53 = gstate.previous_scaled_font;
//     *fresh53 = gstate.scaled_font;
//     let ref mut fresh54 = gstate.scaled_font;
//     *fresh54 = 0 as *mut ScaledFont;
// }

// pub fn _cairo_gstate_set_font_size(mut gstate: *mut GState, mut size: f64) -> cairo_status_t {
//     _cairo_gstate_unset_scaled_font(gstate);
//     cairo_matrix_init_scale(&mut gstate.font_matrix, size, size);
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_set_font_matrix(
//     mut gstate: *mut GState,
//     mut matrix: *const Matrix,
// ) -> cairo_status_t {
//     if memcmp(
//         matrix as *const libc::c_void,
//         &mut gstate.font_matrix as *mut Matrix as *const libc::c_void,
//         ::std::mem::size_of::<Matrix>() as libc::c_ulong,
//     ) == 0 as libc::c_int
//     {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     gstate.font_matrix = *matrix;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_font_matrix(mut gstate: *mut GState, mut matrix: *mut Matrix) {
//     *matrix = gstate.font_matrix;
// }

// pub fn _cairo_gstate_set_font_options(mut gstate: *mut GState, mut options: *const FontOptions) {
//     if memcmp(
//         options as *const libc::c_void,
//         &mut gstate.font_options as *mut FontOptions as *const libc::c_void,
//         ::std::mem::size_of::<FontOptions>() as libc::c_ulong,
//     ) == 0 as libc::c_int
//     {
//         return;
//     }
//     _cairo_gstate_unset_scaled_font(gstate);
//     _cairo_font_options_init_copy(&mut gstate.font_options, options);
// }

// pub fn _cairo_gstate_get_font_options(mut gstate: *mut GState, mut options: *mut FontOptions) {
//     *options = gstate.font_options;
// }

// pub fn _cairo_gstate_get_font_face(
//     mut gstate: *mut GState,
//     mut font_face: *mut *mut FontFace,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_ensure_font_face(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     *font_face = gstate.font_face;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_scaled_font(
//     mut gstate: *mut GState,
//     mut scaled_font: *mut *mut ScaledFont,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_ensure_scaled_font(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     *scaled_font = gstate.scaled_font;
//     return CAIRO_STATUS_SUCCESS;
// }
// fn _cairo_gstate_ensure_font_face(mut gstate: *mut GState) -> cairo_status_t {
//     let mut font_face: *mut FontFace = 0 as *mut FontFace;
//     if !(gstate.font_face).is_null() {
//         return (*gstate.font_face).status;
//     }
//     font_face = cairo_toy_font_face_create(
//         b"\0" as *const u8 as *const libc::c_char,
//         CAIRO_FONT_SLANT_NORMAL,
//         CAIRO_FONT_WEIGHT_NORMAL,
//     );
//     if (*font_face).status as u64 != 0 {
//         return (*font_face).status;
//     }
//     let ref mut fresh55 = gstate.font_face;
//     *fresh55 = font_face;
//     return CAIRO_STATUS_SUCCESS;
// }
// fn _cairo_gstate_ensure_scaled_font(mut gstate: *mut GState) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     let mut options: FontOptions = FontOptions {
//         antialias: CAIRO_ANTIALIAS_DEFAULT,
//         subpixel_order: CAIRO_SUBPIXEL_ORDER_DEFAULT,
//         lcd_filter: CAIRO_LCD_FILTER_DEFAULT,
//         hint_style: CAIRO_HINT_STYLE_DEFAULT,
//         hint_metrics: CAIRO_HINT_METRICS_DEFAULT,
//         round_glyph_positions: CAIRO_ROUND_GLYPH_POS_DEFAULT,
//         variations: 0 as *mut libc::c_char,
//         color_mode: CAIRO_COLOR_MODE_DEFAULT,
//         palette_index: 0,
//     };
//     let mut scaled_font: *mut ScaledFont = 0 as *mut ScaledFont;
//     let mut font_ctm: Matrix = Matrix {
//         xx: 0.,
//         yx: 0.,
//         xy: 0.,
//         yy: 0.,
//         x0: 0.,
//         y0: 0.,
//     };
//     if !(gstate.scaled_font).is_null() {
//         return (*gstate.scaled_font).status;
//     }
//     status = _cairo_gstate_ensure_font_face(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     cairo_surface_get_font_options(gstate.target, &mut options);
//     cairo_font_options_merge(&mut options, &mut gstate.font_options);
//     cairo_matrix_multiply(
//         &mut font_ctm,
//         &mut gstate.ctm,
//         &mut (*gstate.target).device_transform,
//     );
//     scaled_font = cairo_scaled_font_create(
//         gstate.font_face,
//         &mut gstate.font_matrix,
//         &mut font_ctm,
//         &mut options,
//     );
//     status = cairo_scaled_font_status(scaled_font);
//     if status as u64 != 0 {
//         return status;
//     }
//     let ref mut fresh56 = gstate.scaled_font;
//     *fresh56 = scaled_font;
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_get_font_extents(
//     mut gstate: *mut GState,
//     mut extents: *mut cairo_font_extents_t,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = _cairo_gstate_ensure_scaled_font(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     cairo_scaled_font_extents(gstate.scaled_font, extents);
//     return cairo_scaled_font_status(gstate.scaled_font);
// }

// pub fn _cairo_gstate_set_font_face(
//     mut gstate: *mut GState,
//     mut font_face: *mut FontFace,
// ) -> cairo_status_t {
//     if !font_face.is_null() && (*font_face).status as libc::c_uint != 0 {
//         return _cairo_error((*font_face).status);
//     }
//     if font_face == gstate.font_face {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     cairo_font_face_destroy(gstate.font_face);
//     let ref mut fresh57 = gstate.font_face;
//     *fresh57 = cairo_font_face_reference(font_face);
//     _cairo_gstate_unset_scaled_font(gstate);
//     return CAIRO_STATUS_SUCCESS;
// }

// pub fn _cairo_gstate_glyph_extents(
//     mut gstate: *mut GState,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
//     mut extents: *mut cairo_text_extents_t,
// ) -> cairo_status_t {
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_ensure_scaled_font(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     cairo_scaled_font_glyph_extents(gstate.scaled_font, glyphs, num_glyphs, extents);
//     return cairo_scaled_font_status(gstate.scaled_font);
// }

// pub fn _cairo_gstate_show_text_glyphs(
//     mut gstate: *mut GState,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
//     mut info: *mut cairo_glyph_text_info_t,
// ) -> cairo_status_t {
//     let mut current_block: u64;
//     let mut stack_transformed_glyphs: [cairo_glyph_t; 85] = [cairo_glyph_t {
//         index: 0,
//         x: 0.,
//         y: 0.,
//     }; 85];
//     let mut stack_transformed_clusters: [cairo_text_cluster_t; 256] = [cairo_text_cluster_t {
//         num_bytes: 0,
//         num_glyphs: 0,
//     }; 256];
//     let mut source_pattern: cairo_pattern_union_t = cairo_pattern_union_t {
//         base: Pattern {
//             ref_count: cairo_reference_count_t { ref_count: 0 },
//             status: CAIRO_STATUS_SUCCESS,
//             user_data: cairo_user_data_array_t {
//                 size: 0,
//                 num_elements: 0,
//                 element_size: 0,
//                 elements: 0 as *const libc::c_char as *mut libc::c_char,
//             },
//             observers: cairo_list_t {
//                 next: 0 as *const _cairo_list as *mut _cairo_list,
//                 prev: 0 as *const _cairo_list as *mut _cairo_list,
//             },
//             type_0: CAIRO_PATTERN_TYPE_SOLID,
//             filter: CAIRO_FILTER_FAST,
//             extend: CAIRO_EXTEND_NONE,
//             has_component_alpha: 0,
//             is_userfont_foreground: 0,
//             matrix: Matrix {
//                 xx: 0.,
//                 yx: 0.,
//                 xy: 0.,
//                 yy: 0.,
//                 x0: 0.,
//                 y0: 0.,
//             },
//             opacity: 0.,
//         },
//     };
//     let mut transformed_glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut pattern: *const Pattern = 0 as *const Pattern;
//     let mut transformed_clusters: *mut cairo_text_cluster_t = 0 as *mut cairo_text_cluster_t;
//     let mut op: Operator = CAIRO_OPERATOR_CLEAR;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_get_pattern_status(gstate.source);
//     if status as u64 != 0 {
//         return status;
//     }
//     if gstate.op as libc::c_uint == CAIRO_OPERATOR_DEST as libc::c_int as libc::c_uint {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     if _cairo_clip_is_all_clipped(gstate.clip) != 0 {
//         return CAIRO_STATUS_SUCCESS;
//     }
//     status = _cairo_gstate_ensure_scaled_font(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     transformed_glyphs = stack_transformed_glyphs.as_mut_ptr();
//     transformed_clusters = stack_transformed_clusters.as_mut_ptr();
//     if num_glyphs
//         > (::std::mem::size_of::<[cairo_glyph_t; 85]>() as libc::c_ulong)
//             .wrapping_div(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
//             as libc::c_int
//     {
//         transformed_glyphs = cairo_glyph_allocate(num_glyphs);
//         if transformed_glyphs.is_null() {
//             return _cairo_error(CAIRO_STATUS_NO_MEMORY);
//         }
//     }
//     if !info.is_null() {
//         if (*info).num_clusters
//             > (::std::mem::size_of::<[cairo_text_cluster_t; 256]>() as libc::c_ulong)
//                 .wrapping_div(::std::mem::size_of::<cairo_text_cluster_t>() as libc::c_ulong)
//                 as libc::c_int
//         {
//             transformed_clusters = cairo_text_cluster_allocate((*info).num_clusters);
//             if transformed_clusters.is_null() {
//                 status = _cairo_error(CAIRO_STATUS_NO_MEMORY);
//                 current_block = 15065954068683150512;
//             } else {
//                 current_block = 17478428563724192186;
//             }
//         } else {
//             current_block = 17478428563724192186;
//         }
//         match current_block {
//             15065954068683150512 => {}
//             _ => {
//                 _cairo_gstate_transform_glyphs_to_backend(
//                     gstate,
//                     glyphs,
//                     num_glyphs,
//                     (*info).clusters,
//                     (*info).num_clusters,
//                     (*info).cluster_flags,
//                     transformed_glyphs,
//                     &mut num_glyphs,
//                     transformed_clusters,
//                 );
//                 current_block = 13550086250199790493;
//             }
//         }
//     } else {
//         _cairo_gstate_transform_glyphs_to_backend(
//             gstate,
//             glyphs,
//             num_glyphs,
//             0 as *const cairo_text_cluster_t,
//             0 as libc::c_int,
//             0 as cairo_text_cluster_flags_t,
//             transformed_glyphs,
//             &mut num_glyphs,
//             0 as *mut cairo_text_cluster_t,
//         );
//         current_block = 13550086250199790493;
//     }
//     match current_block {
//         13550086250199790493 => {
//             if !(num_glyphs == 0 as libc::c_int) {
//                 op = _reduce_op(gstate);
//                 if op as libc::c_uint == CAIRO_OPERATOR_CLEAR as libc::c_int as libc::c_uint {
//                     pattern = &_cairo_pattern_clear.base;
//                 } else {
//                     _cairo_gstate_copy_transformed_source(gstate, &mut source_pattern.base);
//                     pattern = &mut source_pattern.base;
//                 }
//                 if cairo_surface_has_show_text_glyphs(gstate.target) != 0
//                     || _cairo_scaled_font_get_max_scale(gstate.scaled_font)
//                         <= 10240 as libc::c_int as f64
//                 {
//                     if !info.is_null() {
//                         status = _cairo_surface_show_text_glyphs(
//                             gstate.target,
//                             op,
//                             pattern,
//                             (*info).utf8,
//                             (*info).utf8_len,
//                             transformed_glyphs,
//                             num_glyphs,
//                             transformed_clusters,
//                             (*info).num_clusters,
//                             (*info).cluster_flags,
//                             gstate.scaled_font,
//                             gstate.clip,
//                         );
//                     } else {
//                         status = _cairo_surface_show_text_glyphs(
//                             gstate.target,
//                             op,
//                             pattern,
//                             0 as *const libc::c_char,
//                             0 as libc::c_int,
//                             transformed_glyphs,
//                             num_glyphs,
//                             0 as *const cairo_text_cluster_t,
//                             0 as libc::c_int,
//                             0 as cairo_text_cluster_flags_t,
//                             gstate.scaled_font,
//                             gstate.clip,
//                         );
//                     }
//                 } else {
//                     let mut path: cairo_path_fixed_t = cairo_path_fixed_t {
//                         last_move_point: cairo_point_t { x: 0, y: 0 },
//                         current_point: cairo_point_t { x: 0, y: 0 },
//                         has_current_point_needs_move_to_has_extents_has_curve_to_stroke_is_rectilinear_fill_is_rectilinear_fill_maybe_region_fill_is_empty: [0; 1],
//                         c2rust_padding: [0; 3],
//                         extents: cairo_box_t {
//                             p1: cairo_point_t { x: 0, y: 0 },
//                             p2: cairo_point_t { x: 0, y: 0 },
//                         },
//                         buf: cairo_path_buf_fixed_t {
//                             base: cairo_path_buf_t {
//                                 link: cairo_list_t {
//                                     next: 0 as *const _cairo_list as *mut _cairo_list,
//                                     prev: 0 as *const _cairo_list as *mut _cairo_list,
//                                 },
//                                 num_ops: 0,
//                                 size_ops: 0,
//                                 num_points: 0,
//                                 size_points: 0,
//                                 op: 0 as *mut cairo_path_op_t,
//                                 points: 0 as *mut cairo_point_t,
//                             },
//                             op: [0; 27],
//                             points: [cairo_point_t { x: 0, y: 0 }; 54],
//                         },
//                     };
//                     _cairo_path_fixed_init(&mut path);
//                     status = _cairo_scaled_font_glyph_path(
//                         gstate.scaled_font,
//                         transformed_glyphs,
//                         num_glyphs,
//                         &mut path,
//                     );
//                     if status as libc::c_uint == CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
//                     {
//                         status = _cairo_surface_fill(
//                             gstate.target,
//                             op,
//                             pattern,
//                             &mut path,
//                             CAIRO_FILL_RULE_WINDING,
//                             gstate.tolerance,
//                             (*gstate.scaled_font).options.antialias,
//                             gstate.clip,
//                         );
//                     }
//                     _cairo_path_fixed_fini(&mut path);
//                 }
//             }
//         }
//         _ => {}
//     }
//     if transformed_glyphs != stack_transformed_glyphs.as_mut_ptr() {
//         cairo_glyph_free(transformed_glyphs);
//     }
//     if transformed_clusters != stack_transformed_clusters.as_mut_ptr() {
//         cairo_text_cluster_free(transformed_clusters);
//     }
//     return status;
// }

// pub fn _cairo_gstate_glyph_path(
//     mut gstate: *mut GState,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
//     mut path: *mut cairo_path_fixed_t,
// ) -> cairo_status_t {
//     let mut stack_transformed_glyphs: [cairo_glyph_t; 85] = [cairo_glyph_t {
//         index: 0,
//         x: 0.,
//         y: 0.,
//     }; 85];
//     let mut transformed_glyphs: *mut cairo_glyph_t = 0 as *mut cairo_glyph_t;
//     let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
//     status = _cairo_gstate_ensure_scaled_font(gstate);
//     if status as u64 != 0 {
//         return status;
//     }
//     if num_glyphs
//         < (::std::mem::size_of::<[cairo_glyph_t; 85]>() as libc::c_ulong)
//             .wrapping_div(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong)
//             as libc::c_int
//     {
//         transformed_glyphs = stack_transformed_glyphs.as_mut_ptr();
//     } else {
//         transformed_glyphs = cairo_glyph_allocate(num_glyphs);
//         if transformed_glyphs.is_null() {
//             return _cairo_error(CAIRO_STATUS_NO_MEMORY);
//         }
//     }
//     _cairo_gstate_transform_glyphs_to_backend(
//         gstate,
//         glyphs,
//         num_glyphs,
//         0 as *const cairo_text_cluster_t,
//         0 as libc::c_int,
//         0 as cairo_text_cluster_flags_t,
//         transformed_glyphs,
//         &mut num_glyphs,
//         0 as *mut cairo_text_cluster_t,
//     );
//     status =
//         _cairo_scaled_font_glyph_path(gstate.scaled_font, transformed_glyphs, num_glyphs, path);
//     if transformed_glyphs != stack_transformed_glyphs.as_mut_ptr() {
//         cairo_glyph_free(transformed_glyphs);
//     }
//     return status;
// }

// pub fn _cairo_gstate_set_antialias(
//     mut gstate: *mut GState,
//     mut antialias: Antialias,
// ) -> cairo_status_t {
//     gstate.antialias = antialias;
//     return CAIRO_STATUS_SUCCESS;
// }

pub fn _cairo_gstate_get_antialias(gstate: &GState) -> Antialias {
    return gstate.antialias;
}

// fn _cairo_gstate_transform_glyphs_to_backend(
//     mut gstate: *mut GState,
//     mut glyphs: *const cairo_glyph_t,
//     mut num_glyphs: libc::c_int,
//     mut clusters: *const cairo_text_cluster_t,
//     mut num_clusters: libc::c_int,
//     mut cluster_flags: cairo_text_cluster_flags_t,
//     mut transformed_glyphs: *mut cairo_glyph_t,
//     mut num_transformed_glyphs: *mut libc::c_int,
//     mut transformed_clusters: *mut cairo_text_cluster_t,
// ) {
//     let mut surface_extents: cairo_rectangle_int_t = cairo_rectangle_int_t {
//         x: 0,
//         y: 0,
//         width: 0,
//         height: 0,
//     };
//     let mut ctm: *mut Matrix = &mut gstate.ctm;
//     let mut font_matrix: *mut Matrix = &mut gstate.font_matrix;
//     let mut device_transform: *mut Matrix = &mut (*gstate.target).device_transform;
//     let mut drop_0: bool = 0 as libc::c_int;
//     let mut x1: f64 = 0 as libc::c_int as f64;
//     let mut x2: f64 = 0 as libc::c_int as f64;
//     let mut y1: f64 = 0 as libc::c_int as f64;
//     let mut y2: f64 = 0 as libc::c_int as f64;
//     let mut i: libc::c_int = 0;
//     let mut j: libc::c_int = 0;
//     let mut k: libc::c_int = 0;
//     drop_0 = 1 as libc::c_int;
//     if _cairo_gstate_int_clip_extents(gstate, &mut surface_extents) == 0 {
//         drop_0 = 0 as libc::c_int;
//     } else {
//         let mut scale10: f64 =
//             10 as libc::c_int as f64 * _cairo_scaled_font_get_max_scale(gstate.scaled_font);
//         if surface_extents.width == 0 as libc::c_int || surface_extents.height == 0 as libc::c_int {
//             *num_transformed_glyphs = 0 as libc::c_int;
//             return;
//         }
//         x1 = surface_extents.x as f64 - scale10;
//         y1 = surface_extents.y as f64 - scale10;
//         x2 = (surface_extents.x + surface_extents.width) as f64 + scale10;
//         y2 = (surface_extents.y + surface_extents.height) as f64 + scale10;
//     }
//     if drop_0 == 0 {
//         *num_transformed_glyphs = num_glyphs;
//     }
//     j = 0 as libc::c_int;
//     if _cairo_matrix_is_identity(ctm) != 0
//         && _cairo_matrix_is_identity(device_transform) != 0
//         && (*font_matrix).x0 == 0 as libc::c_int as f64
//         && (*font_matrix).y0 == 0 as libc::c_int as f64
//     {
//         if drop_0 == 0 {
//             memcpy(
//                 transformed_glyphs as *mut libc::c_void,
//                 glyphs as *const libc::c_void,
//                 (num_glyphs as libc::c_ulong)
//                     .wrapping_mul(::std::mem::size_of::<cairo_glyph_t>() as libc::c_ulong),
//             );
//             memcpy(
//                 transformed_clusters as *mut libc::c_void,
//                 clusters as *const libc::c_void,
//                 (num_clusters as libc::c_ulong)
//                     .wrapping_mul(::std::mem::size_of::<cairo_text_cluster_t>() as libc::c_ulong),
//             );
//             j = num_glyphs;
//         } else if num_clusters == 0 as libc::c_int {
//             i = 0 as libc::c_int;
//             while i < num_glyphs {
//                 (*transformed_glyphs.offset(j as isize)).index = (*glyphs.offset(i as isize)).index;
//                 (*transformed_glyphs.offset(j as isize)).x = (*glyphs.offset(i as isize)).x;
//                 (*transformed_glyphs.offset(j as isize)).y = (*glyphs.offset(i as isize)).y;
//                 if x1 <= (*transformed_glyphs.offset(j as isize)).x
//                     && (*transformed_glyphs.offset(j as isize)).x <= x2
//                     && y1 <= (*transformed_glyphs.offset(j as isize)).y
//                     && (*transformed_glyphs.offset(j as isize)).y <= y2
//                 {
//                     j += 1;
//                 }
//                 i += 1;
//             }
//         } else {
//             let mut cur_glyph: *const cairo_glyph_t = 0 as *const cairo_glyph_t;
//             if cluster_flags as libc::c_uint
//                 & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//                 != 0
//             {
//                 cur_glyph = glyphs
//                     .offset(num_glyphs as isize)
//                     .offset(-(1 as libc::c_int as isize));
//             } else {
//                 cur_glyph = glyphs;
//             }
//             i = 0 as libc::c_int;
//             while i < num_clusters {
//                 let mut cluster_visible: bool = 0 as libc::c_int;
//                 k = 0 as libc::c_int;
//                 while k < (*clusters.offset(i as isize)).num_glyphs {
//                     (*transformed_glyphs.offset((j + k) as isize)).index = (*cur_glyph).index;
//                     (*transformed_glyphs.offset((j + k) as isize)).x = (*cur_glyph).x;
//                     (*transformed_glyphs.offset((j + k) as isize)).y = (*cur_glyph).y;
//                     if x1 <= (*transformed_glyphs.offset((j + k) as isize)).x
//                         && (*transformed_glyphs.offset((j + k) as isize)).x <= x2
//                         && y1 <= (*transformed_glyphs.offset((j + k) as isize)).y
//                         && (*transformed_glyphs.offset((j + k) as isize)).y <= y2
//                     {
//                         cluster_visible = 1 as libc::c_int;
//                     }
//                     if cluster_flags as libc::c_uint
//                         & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//                         != 0
//                     {
//                         cur_glyph = cur_glyph.offset(-1);
//                     } else {
//                         cur_glyph = cur_glyph.offset(1);
//                     }
//                     k += 1;
//                 }
//                 *transformed_clusters.offset(i as isize) = *clusters.offset(i as isize);
//                 if cluster_visible != 0 {
//                     j += k;
//                 } else {
//                     (*transformed_clusters.offset(i as isize)).num_glyphs = 0 as libc::c_int;
//                 }
//                 i += 1;
//             }
//         }
//     } else if _cairo_matrix_is_translation(ctm) != 0
//         && _cairo_matrix_is_translation(device_transform) != 0
//     {
//         let mut tx: f64 = (*font_matrix).x0 + (*ctm).x0 + (*device_transform).x0;
//         let mut ty: f64 = (*font_matrix).y0 + (*ctm).y0 + (*device_transform).y0;
//         if drop_0 == 0 || num_clusters == 0 as libc::c_int {
//             i = 0 as libc::c_int;
//             while i < num_glyphs {
//                 (*transformed_glyphs.offset(j as isize)).index = (*glyphs.offset(i as isize)).index;
//                 (*transformed_glyphs.offset(j as isize)).x = (*glyphs.offset(i as isize)).x + tx;
//                 (*transformed_glyphs.offset(j as isize)).y = (*glyphs.offset(i as isize)).y + ty;
//                 if drop_0 == 0
//                     || x1 <= (*transformed_glyphs.offset(j as isize)).x
//                         && (*transformed_glyphs.offset(j as isize)).x <= x2
//                         && y1 <= (*transformed_glyphs.offset(j as isize)).y
//                         && (*transformed_glyphs.offset(j as isize)).y <= y2
//                 {
//                     j += 1;
//                 }
//                 i += 1;
//             }
//             memcpy(
//                 transformed_clusters as *mut libc::c_void,
//                 clusters as *const libc::c_void,
//                 (num_clusters as libc::c_ulong)
//                     .wrapping_mul(::std::mem::size_of::<cairo_text_cluster_t>() as libc::c_ulong),
//             );
//         } else {
//             let mut cur_glyph_0: *const cairo_glyph_t = 0 as *const cairo_glyph_t;
//             if cluster_flags as libc::c_uint
//                 & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//                 != 0
//             {
//                 cur_glyph_0 = glyphs
//                     .offset(num_glyphs as isize)
//                     .offset(-(1 as libc::c_int as isize));
//             } else {
//                 cur_glyph_0 = glyphs;
//             }
//             i = 0 as libc::c_int;
//             while i < num_clusters {
//                 let mut cluster_visible_0: bool = 0 as libc::c_int;
//                 k = 0 as libc::c_int;
//                 while k < (*clusters.offset(i as isize)).num_glyphs {
//                     (*transformed_glyphs.offset((j + k) as isize)).index = (*cur_glyph_0).index;
//                     (*transformed_glyphs.offset((j + k) as isize)).x = (*cur_glyph_0).x + tx;
//                     (*transformed_glyphs.offset((j + k) as isize)).y = (*cur_glyph_0).y + ty;
//                     if x1 <= (*transformed_glyphs.offset((j + k) as isize)).x
//                         && (*transformed_glyphs.offset((j + k) as isize)).x <= x2
//                         && y1 <= (*transformed_glyphs.offset((j + k) as isize)).y
//                         && (*transformed_glyphs.offset((j + k) as isize)).y <= y2
//                     {
//                         cluster_visible_0 = 1 as libc::c_int;
//                     }
//                     if cluster_flags as libc::c_uint
//                         & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//                         != 0
//                     {
//                         cur_glyph_0 = cur_glyph_0.offset(-1);
//                     } else {
//                         cur_glyph_0 = cur_glyph_0.offset(1);
//                     }
//                     k += 1;
//                 }
//                 *transformed_clusters.offset(i as isize) = *clusters.offset(i as isize);
//                 if cluster_visible_0 != 0 {
//                     j += k;
//                 } else {
//                     (*transformed_clusters.offset(i as isize)).num_glyphs = 0 as libc::c_int;
//                 }
//                 i += 1;
//             }
//         }
//     } else {
//         let mut aggregate_transform: Matrix = Matrix {
//             xx: 0.,
//             yx: 0.,
//             xy: 0.,
//             yy: 0.,
//             x0: 0.,
//             y0: 0.,
//         };
//         cairo_matrix_init_translate(
//             &mut aggregate_transform,
//             gstate.font_matrix.x0,
//             gstate.font_matrix.y0,
//         );
//         cairo_matrix_multiply(&mut aggregate_transform, &mut aggregate_transform, ctm);
//         cairo_matrix_multiply(
//             &mut aggregate_transform,
//             &mut aggregate_transform,
//             device_transform,
//         );
//         if drop_0 == 0 || num_clusters == 0 as libc::c_int {
//             i = 0 as libc::c_int;
//             while i < num_glyphs {
//                 *transformed_glyphs.offset(j as isize) = *glyphs.offset(i as isize);
//                 cairo_matrix_transform_point(
//                     &mut aggregate_transform,
//                     &mut (*transformed_glyphs.offset(j as isize)).x,
//                     &mut (*transformed_glyphs.offset(j as isize)).y,
//                 );
//                 if drop_0 == 0
//                     || x1 <= (*transformed_glyphs.offset(j as isize)).x
//                         && (*transformed_glyphs.offset(j as isize)).x <= x2
//                         && y1 <= (*transformed_glyphs.offset(j as isize)).y
//                         && (*transformed_glyphs.offset(j as isize)).y <= y2
//                 {
//                     j += 1;
//                 }
//                 i += 1;
//             }
//             memcpy(
//                 transformed_clusters as *mut libc::c_void,
//                 clusters as *const libc::c_void,
//                 (num_clusters as libc::c_ulong)
//                     .wrapping_mul(::std::mem::size_of::<cairo_text_cluster_t>() as libc::c_ulong),
//             );
//         } else {
//             let mut cur_glyph_1: *const cairo_glyph_t = 0 as *const cairo_glyph_t;
//             if cluster_flags as libc::c_uint
//                 & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//                 != 0
//             {
//                 cur_glyph_1 = glyphs
//                     .offset(num_glyphs as isize)
//                     .offset(-(1 as libc::c_int as isize));
//             } else {
//                 cur_glyph_1 = glyphs;
//             }
//             i = 0 as libc::c_int;
//             while i < num_clusters {
//                 let mut cluster_visible_1: bool = 0 as libc::c_int;
//                 k = 0 as libc::c_int;
//                 while k < (*clusters.offset(i as isize)).num_glyphs {
//                     *transformed_glyphs.offset((j + k) as isize) = *cur_glyph_1;
//                     cairo_matrix_transform_point(
//                         &mut aggregate_transform,
//                         &mut (*transformed_glyphs.offset((j + k) as isize)).x,
//                         &mut (*transformed_glyphs.offset((j + k) as isize)).y,
//                     );
//                     if x1 <= (*transformed_glyphs.offset((j + k) as isize)).x
//                         && (*transformed_glyphs.offset((j + k) as isize)).x <= x2
//                         && y1 <= (*transformed_glyphs.offset((j + k) as isize)).y
//                         && (*transformed_glyphs.offset((j + k) as isize)).y <= y2
//                     {
//                         cluster_visible_1 = 1 as libc::c_int;
//                     }
//                     if cluster_flags as libc::c_uint
//                         & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//                         != 0
//                     {
//                         cur_glyph_1 = cur_glyph_1.offset(-1);
//                     } else {
//                         cur_glyph_1 = cur_glyph_1.offset(1);
//                     }
//                     k += 1;
//                 }
//                 *transformed_clusters.offset(i as isize) = *clusters.offset(i as isize);
//                 if cluster_visible_1 != 0 {
//                     j += k;
//                 } else {
//                     (*transformed_clusters.offset(i as isize)).num_glyphs = 0 as libc::c_int;
//                 }
//                 i += 1;
//             }
//         }
//     }
//     *num_transformed_glyphs = j;
//     if num_clusters != 0 as libc::c_int
//         && cluster_flags as libc::c_uint
//             & CAIRO_TEXT_CLUSTER_FLAG_BACKWARD as libc::c_int as libc::c_uint
//             != 0
//     {
//         i = 0 as libc::c_int;
//         loop {
//             j -= 1;
//             if !(i < j) {
//                 break;
//             }
//             let mut tmp: cairo_glyph_t = cairo_glyph_t {
//                 index: 0,
//                 x: 0.,
//                 y: 0.,
//             };
//             tmp = *transformed_glyphs.offset(i as isize);
//             *transformed_glyphs.offset(i as isize) = *transformed_glyphs.offset(j as isize);
//             *transformed_glyphs.offset(j as isize) = tmp;
//             i += 1;
//         }
//     }
// }
