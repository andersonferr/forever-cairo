use std::f64::consts::PI;

use crate::matrix::Matrix;
use crate::cairo::Cairo;

fn _arc_error_normalized(mut angle: f64) -> f64 {
    return 2.0 / 27.0 * (angle / 4.0).sin().powi(6) / (angle / 4.).cos().powi(2);
}

fn _arc_max_angle_for_tolerance_normalized(tolerance: f64) -> f64 {
    #[derive(Copy, Clone)]
    pub struct Row {
        pub angle: f64,
        pub error: f64,
    }

    let mut table = [
        Row {
            angle: PI / 1.0,
            error: 0.0185185185185185036127,
        },
        Row {
            angle: PI / 2.0,
            error: 0.000272567143730179811158,
        },
        Row {
            angle: PI / 3.0,
            error: 2.38647043651461047433e-05,
        },
        Row {
            angle: PI / 4.0,
            error: 4.2455377443222443279e-06,
        },
        Row {
            angle: PI / 5.0,
            error: 1.11281001494389081528e-06,
        },
        Row {
            angle: PI / 6.0,
            error: 3.72662000942734705475e-07,
        },
        Row {
            angle: PI / 7.0,
            error: 1.47783685574284411325e-07,
        },
        Row {
            angle: PI / 8.0,
            error: 6.63240432022601149057e-08,
        },
        Row {
            angle: PI / 9.0,
            error: 3.2715520137536980553e-08,
        },
        Row {
            angle: PI / 10.0,
            error: 1.73863223499021216974e-08,
        },
        Row {
            angle: PI / 11.0,
            error: 9.81410988043554039085e-09,
        },
    ];

    let table_size = table.len();

    let mut i = 0_usize;
    let mut angle;
    let mut error;

    while i < table_size {
        if table[i].error < tolerance {
            return table[i].angle;
        }
        i += 1;
    }

    // this value is chosen arbitrarily.
    // this gives an error of about 1.74909e-20
    const max_segments: usize = 1000;

    loop {
        i += 1;
        angle = PI / i as f64;
        error = _arc_error_normalized(angle);

        if error <= tolerance || i >= max_segments {
            return angle;
        }
    }
}

fn _arc_segments_needed(angle: f64, radius: f64, ctm: &mut Matrix, tolerance: f64) -> isize {
    let major_axis = _cairo_matrix_transformed_circle_major_axis(ctm, radius);
    let max_angle = _arc_max_angle_for_tolerance_normalized(tolerance / major_axis);
    (angle.abs() / max_angle).ceil() as isize
}

fn _cairo_arc_segment(cr: &mut Cairo, xc: f64, yc: f64, radius: f64, angle_A: f64, angle_B: f64) {
    let r_sin_A = radius * angle_A.sin();
    let r_cos_A = radius * angle_A.cos();
    let r_sin_B = radius * angle_B.sin();
    let r_cos_B = radius * angle_B.cos();

    let h = 4.0 / 3.0 * ((angle_B - angle_A) / 4.0).tan();

    cairo_curve_to(
        cr,
        xc + r_cos_A - h * r_sin_A,
        yc + r_sin_A + h * r_cos_A,
        xc + r_cos_B + h * r_sin_B,
        yc + r_sin_B - h * r_cos_B,
        xc + r_cos_B,
        yc + r_sin_B,
    );
}
fn _cairo_arc_in_direction(
    cr: &mut Cairo,
    xc: f64,
    yc: f64,
    radius: f64,
    angle_min: f64,
    angle_max: f64,
    dir: cairo_direction_t,
) {
    if cairo_status(cr) as u64 != 0 {
        return;
    }

    assert!(angle_max >= angle_min);

    if angle_max - angle_min > 2.0 * PI * 65536.0 {
        angle_max = (angle_max - angle_min) % (2.0 * PI);
        angle_min = (angle_min) % (2.0 * PI);
        angle_max += angle_min + 2.0 * PI * 65536.0;
    }
    if angle_max - angle_min > PI {
        let angle_mid = angle_min + (angle_max - angle_min) / 2.0;
        if dir == CAIRO_DIRECTION_FORWARD {
            _cairo_arc_in_direction(cr, xc, yc, radius, angle_min, angle_mid, dir);
            _cairo_arc_in_direction(cr, xc, yc, radius, angle_mid, angle_max, dir);
        } else {
            _cairo_arc_in_direction(cr, xc, yc, radius, angle_mid, angle_max, dir);
            _cairo_arc_in_direction(cr, xc, yc, radius, angle_min, angle_mid, dir);
        }
    } else if angle_max != angle_min {
        let mut ctm= Matrix {
            xx: 0.0,
            yx: 0.0,
            xy: 0.0,
            yy: 0.0,
            x0: 0.0,
            y0: 0.0,
        };
        let mut i= 0;
        let mut segments = 0;
        let mut step 0.0;
        cairo_get_matrix(cr, &mut ctm);
        segments = _arc_segments_needed(
            angle_max - angle_min,
            radius,
            &mut ctm,
            cairo_get_tolerance(cr),
        );
        step = (angle_max - angle_min) / segments as f64;
        segments -= 1 as libc::c_int;
        if dir as libc::c_uint == CAIRO_DIRECTION_REVERSE as libc::c_int as libc::c_uint {
            let mut t: f64 = 0.;
            t = angle_min;
            angle_min = angle_max;
            angle_max = t;
            step = -step;
        }
        cairo_line_to(
            cr,
            xc + radius * cos(angle_min),
            yc + radius * sin(angle_min),
        );
        i = 0 as libc::c_int;
        while i < segments {
            _cairo_arc_segment(cr, xc, yc, radius, angle_min, angle_min + step);
            i += 1;
            angle_min += step;
        }
        _cairo_arc_segment(cr, xc, yc, radius, angle_min, angle_max);
    } else {
        cairo_line_to(
            cr,
            xc + radius * cos(angle_min),
            yc + radius * sin(angle_min),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_arc_path(
    mut cr: *mut cairo_t,
    mut xc: f64,
    mut yc: f64,
    mut radius: f64,
    mut angle1: f64,
    mut angle2: f64,
) {
    _cairo_arc_in_direction(cr, xc, yc, radius, angle1, angle2, CAIRO_DIRECTION_FORWARD);
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_arc_path_negative(
    mut cr: *mut cairo_t,
    mut xc: f64,
    mut yc: f64,
    mut radius: f64,
    mut angle1: f64,
    mut angle2: f64,
) {
    _cairo_arc_in_direction(cr, xc, yc, radius, angle2, angle1, CAIRO_DIRECTION_REVERSE);
}
