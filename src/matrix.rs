use crate::cairo::Result;
use crate::error::Error;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub xx: f64,
    pub yx: f64,
    pub xy: f64,
    pub yy: f64,
    pub x0: f64,
    pub y0: f64,
}

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_line {
//     pub p1: cairo_point_t,
//     pub p2: cairo_point_t,
// }
// pub type cairo_point_t = _cairo_point;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _cairo_point {
//     pub x: cairo_fixed_t,
//     pub y: cairo_fixed_t,
// }
// pub type cairo_fixed_t = int32_t;
// pub type uint32_t = __uint32_t;
// pub type _cairo_filter = libc::c_uint;
// pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;
// pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
// pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
// pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
// pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
// pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
// pub type cairo_filter_t = _cairo_filter;
// pub type pixman_bool_t = libc::c_int;
// pub type pixman_fixed_16_16_t = int32_t;
// pub type pixman_fixed_t = pixman_fixed_16_16_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct pixman_vector {
//     pub vector: [pixman_fixed_t; 3],
// }
// pub type pixman_vector_t = pixman_vector;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct pixman_transform {
//     pub matrix: [[pixman_fixed_t; 3]; 3],
// }
// pub type pixman_transform_t = pixman_transform;
// pub type cairo_fixed_16_16_t = int32_t;
// pub type cairo_fixed_unsigned_t = uint32_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed {
//     pub d: f64,
//     pub i: [int32_t; 2],
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed_0 {
//     pub d: f64,
//     pub i: [int32_t; 2],
// }
// #[inline]
// pub(crate) fn cairo_fixed_16_16_from_double(
//     mut d: f64,
// ) -> cairo_fixed_16_16_t {
//     let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { d: 0. };
//     u.d = d + 103079215104.0f64;
//     return u.i[0];
// }
// #[inline]
// pub(crate) fn cairo_fixed_to_double(mut f: cairo_fixed_t) -> f64 {
//     return f as f64
//         / ((1 as libc::c_int) << 8 as libc::c_int) as f64;
// }
// #[inline]
// pub(crate) fn cairo_fixed_is_integer(mut f: cairo_fixed_t) -> libc::c_int {
//     return (f
//         & (-(1 as libc::c_int) as cairo_fixed_unsigned_t
//             >> 32 as libc::c_int - 8 as libc::c_int) as cairo_fixed_t
//         == 0 as libc::c_int) as libc::c_int;
// }
// #[inline]
// pub(crate) fn cairo_fixed_integer_part(mut f: cairo_fixed_t) -> libc::c_int {
//     return f >> 8 as libc::c_int;
// }
// #[inline]
// pub(crate) fn cairo_fixed_from_double(mut d: f64) -> cairo_fixed_t {
//     let mut u: C2RustUnnamed = C2RustUnnamed { d: 0. };
//     u
//         .d = d
//         + ((1 as libc::c_longlong) << 52 as libc::c_int - 8 as libc::c_int)
//             as f64 * 1.5f64;
//     return u.i[0];
// }
// #[inline]
// pub(crate) fn is_translation(
//     mut matrix: &Matrix,
// ) -> bool {
//     return (matrix.xx == 1.0f64 && matrix.yx == 0.0f64 && matrix.xy == 0.0f64
//         && matrix.yy == 1.0f64) as libc::c_int;
// }
// #[inline]
// pub(crate) fn cairo_lround(mut r: f64) -> libc::c_int {
//     return _cairo_round(r) as libc::c_int;
// }
// #[inline]
// pub(crate) fn cairo_round(mut r: f64) -> f64 {
//     return floor(r + 0.5f64);
// }
//

impl Matrix {
    #[inline]
    fn zeroed() -> Self {
        Self {
            xx: 0.0,
            yx: 0.0,
            xy: 0.0,
            yy: 0.0,
            x0: 0.0,
            y0: 0.0,
        }
    }

    #[inline]
    pub fn init(&mut self, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) {
        self.xx = xx;
        self.yx = yx;
        self.xy = xy;
        self.yy = yy;
        self.x0 = x0;
        self.y0 = y0;
    }

    #[inline]
    pub fn init_identity(&mut self) {
        self.init(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    }

    #[inline]
    pub(crate) fn get_affine(
        &self,
        xx: &mut f64,
        yx: &mut f64,
        xy: &mut f64,
        yy: &mut f64,
        x0: Option<&mut f64>,
        y0: Option<&mut f64>,
    ) {
        *xx = self.xx;
        *yx = self.yx;
        *xy = self.xy;
        *yy = self.yy;
        if let Some(x0) = x0 {
            *x0 = self.x0;
        }
        if let Some(y0) = y0 {
            *y0 = self.y0;
        }
    }

    #[inline]
    pub fn init_translate(&mut self, tx: f64, ty: f64) {
        self.init(1.0, 0.0, 0.0, 1.0, tx, ty);
    }

    #[inline]
    pub fn translate(&mut self, tx: f64, ty: f64) {
        let mut tmp: Matrix = Matrix::zeroed();
        tmp.init_translate(tx, ty);

        *self = Self::multiply(&tmp, self);
    }

    pub fn init_scale(&mut self, sx: f64, sy: f64) {
        self.init(sx, 0.0, 0.0, sy, 0.0, 0.0);
    }

    pub fn scale(matrix: &mut Matrix, sx: f64, sy: f64) {
        let mut tmp = Matrix::zeroed();
        tmp.init_scale(sx, sy);

        *matrix = Self::multiply(&tmp, matrix);
    }

    pub fn init_rotate(&mut self, radians: f64) {
        let s = radians.sin();
        let c = radians.cos();

        self.init(c, s, -s, c, 0.0, 0.0);
    }

    pub fn rotate(&mut self, radians: f64) {
        let mut tmp = Matrix::zeroed();
        tmp.init_rotate(radians);

        *self = Self::multiply(&mut tmp, self);
    }

    pub fn multiply(a: &Matrix, b: &Matrix) -> Self {
        let mut r: Matrix = Matrix::zeroed();
        Self::multiply_nonoverlapping(&mut r, a, b);

        r
    }

    pub(crate) fn multiply_nonoverlapping(r: &mut Matrix, a: &Matrix, b: &Matrix) {
        r.xx = a.xx * b.xx + a.yx * b.xy;
        r.yx = a.xx * b.yx + a.yx * b.yy;
        r.xy = a.xy * b.xx + a.yy * b.xy;
        r.yy = a.xy * b.yx + a.yy * b.yy;
        r.x0 = a.x0 * b.xx + a.y0 * b.xy + b.x0;
        r.y0 = a.x0 * b.yx + a.y0 * b.yy + b.y0;
    }

    #[inline]
    pub fn transform_distance(&self, dx: &mut f64, dy: &mut f64) {
        let new_x = self.xx * (*dx) + self.xy * (*dy);
        let new_y = self.yx * (*dx) + self.yy * (*dy);

        *dx = new_x;
        *dy = new_y;
    }

    #[inline]
    pub fn transform_point(&self, x: &mut f64, y: &mut f64) {
        self.transform_distance(x, y);
        *x += self.x0;
        *y += self.y0;
    }

    pub(crate) fn transform_bounding_box(
        &self,
        x1: &mut f64,
        y1: &mut f64,
        x2: &mut f64,
        y2: &mut f64,
        is_tight: Option<&mut bool>,
    ) {
        let mut quad_x = [0.0; 4];
        let mut quad_y = [0.0; 4];

        if self.xy == 0.0 && self.yx == 0.0 {
            if self.xx != 1.0 {
                quad_x[0] = (*x1) * self.xx;
                quad_x[1] = (*x2) * self.xx;
                if quad_x[0] < quad_x[1] {
                    *x1 = quad_x[0];
                    *x2 = quad_x[1];
                } else {
                    *x1 = quad_x[1];
                    *x2 = quad_x[0];
                }
            }
            if self.x0 != 0.0 {
                *x1 += self.x0;
                *x2 += self.x0;
            }
            if self.yy != 1.0 {
                quad_y[0] = (*y1) * self.yy;
                quad_y[1] = (*y2) * self.yy;
                if quad_y[0] < quad_y[1] {
                    *y1 = quad_y[0];
                    *y2 = quad_y[1];
                } else {
                    *y1 = quad_y[1];
                    *y2 = quad_y[0];
                }
            }
            if self.y0 != 0.0 {
                *y1 += self.y0;
                *y2 += self.y0;
            }

            if let Some(is_tight) = is_tight {
                *is_tight = true;
            }
            return;
        }
        quad_x[0] = *x1;
        quad_y[0] = *y1;
        self.transform_point(&mut quad_x[0], &mut quad_y[0]);

        quad_x[1] = *x2;
        quad_y[1] = *y1;
        self.transform_point(&mut quad_x[1], &mut quad_y[1]);

        quad_x[2] = *x1;
        quad_y[2] = *y2;
        self.transform_point(&mut quad_x[2], &mut quad_y[2]);

        quad_x[3] = *x2;
        quad_y[3] = *y2;
        self.transform_point(&mut quad_x[3], &mut quad_y[3]);

        let mut max_x = quad_x[0];
        let mut min_x = max_x;
        let mut max_y = quad_y[0];
        let mut min_y = max_y;
        let mut i = 1;
        while i < 4 {
            if quad_x[i] < min_x {
                min_x = quad_x[i];
            }
            if quad_x[i] > max_x {
                max_x = quad_x[i];
            }
            if quad_y[i] < min_y {
                min_y = quad_y[i];
            }
            if quad_y[i] > max_y {
                max_y = quad_y[i];
            }
            i += 1;
        }

        *x1 = min_x;
        *y1 = min_y;
        *x2 = max_x;
        *y2 = max_y;
        if let Some(is_tight) = is_tight {
            *is_tight = (quad_x[1] == quad_x[0]
                && quad_y[1] == quad_y[3]
                && quad_x[2] == quad_x[3]
                && quad_y[2] == quad_y[0])
                || (quad_x[1] == quad_x[3]
                    && quad_y[1] == quad_y[0]
                    && quad_x[2] == quad_x[0]
                    && quad_y[2] == quad_y[3]);
        }
    }

    // pub(crate) fn transform_bounding_box_fixed(
    //     mut matrix: &Matrix,
    //     mut bbox: &mut cairo_box_t,
    //     mut is_tight: &mut bool,
    // ) {
    //     let mut x1: f64 = 0.;
    //     let mut y1: f64 = 0.;
    //     let mut x2: f64 = 0.;
    //     let mut y2: f64 = 0.;
    //     _cairo_box_to_doubles(bbox, &mut x1, &mut y1, &mut x2, &mut y2);
    //     _cairo_matrix_transform_bounding_box(
    //         matrix,
    //         &mut x1,
    //         &mut y1,
    //         &mut x2,
    //         &mut y2,
    //         is_tight,
    //     );
    //     _cairo_box_from_doubles(bbox, &mut x1, &mut y1, &mut x2, &mut y2);
    // }

    #[inline]
    pub(crate) fn scalar_multiply(matrix: &mut Matrix, scalar: f64) {
        matrix.xx *= scalar;
        matrix.yx *= scalar;
        matrix.xy *= scalar;
        matrix.yy *= scalar;
        matrix.x0 *= scalar;
        matrix.y0 *= scalar;
    }

    pub(crate) fn compute_adjoint(&mut self) {
        let mut a = 0.0;
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;
        let mut tx = 0.0;
        let mut ty = 0.0;

        self.get_affine(&mut a, &mut b, &mut c, &mut d, Some(&mut tx), Some(&mut ty));
        self.init(d, -b, -c, a, c * ty - d * tx, b * tx - a * ty);
    }

    pub fn invert(&mut self) -> Result<()> {
        if self.xy == 0.0 && self.yx == 0.0 {
            self.x0 = -self.x0;
            self.y0 = -self.y0;
            if self.xx != 1.0 {
                if self.xx == 0.0 {
                    return Err(Error::InvalidMatrix);
                }
                self.xx = 1.0 / self.xx;
                self.x0 *= self.xx;
            }
            if self.yy != 1.0 {
                if self.yy == 0.0 {
                    return Err(Error::InvalidMatrix);
                }
                self.yy = 1.0 / self.yy;
                self.y0 *= self.yy;
            }
            return Ok(());
        }

        let det = self.compute_determinant();
        if !det.is_finite() {
            return Err(Error::InvalidMatrix);
        }
        if det == 0.0 {
            return Err(Error::InvalidMatrix);
        }

        Self::compute_adjoint(self);
        Self::scalar_multiply(self, 1.0 / det);

        Ok(())
    }

    // pub(crate) fn is_invertible(
    //     mut matrix: &Matrix,
    // ) -> bool {
    //     let mut det: f64 = 0.;
    //     det = _cairo_matrix_compute_determinant(matrix);
    //     return (det.is_finite() as i32 != 0 && det != 0.0f64) as libc::c_int;
    // }

    pub(crate) fn is_scale_0(&self) -> bool {
        self.xx == 0.0 && self.xy == 0.0 && self.yx == 0.0 && self.yy == 0.0
    }

    pub(crate) fn compute_determinant(&self) -> f64 {
        let a = self.xx;
        let b = self.yx;
        let c = self.xy;
        let d = self.yy;
        return a * d - b * c;
    }

    // pub(crate) fn compute_basis_scale_factors(
    //     mut matrix: &Matrix,
    //     mut basis_scale: &mut f64,
    //     mut normal_scale: &mut f64,
    //     mut x_basis: bool,
    // ) -> cairo_status_t {
    //     let mut det: f64 = 0.;
    //     det = _cairo_matrix_compute_determinant(matrix);
    //     if det.is_finite() as i32 == 0 {
    //         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
    //     }
    //     if det == 0.0 {
    //         *normal_scale = 0.0;
    //         *basis_scale = *normal_scale;
    //     } else {
    //         let mut x: f64 = (x_basis != 0 as libc::c_int) as libc::c_int
    //             as f64;
    //         let mut y: f64 = (x == 0.0)
    //             as libc::c_int as f64;
    //         let mut major: f64 = 0.;
    //         let mut minor: f64 = 0.;
    //         cairo_matrix_transform_distance(matrix, &mut x, &mut y);
    //         major = hypot(x, y);
    //         if det < 0.0 {
    //             det = -det;
    //         }
    //         if major != 0. {
    //             minor = det / major;
    //         } else {
    //             minor = 0.0f64;
    //         }
    //         if x_basis != 0 {
    //             *basis_scale = major;
    //             *normal_scale = minor;
    //         } else {
    //             *basis_scale = minor;
    //             *normal_scale = major;
    //         }
    //     }
    //     return CAIRO_STATUS_SUCCESS;
    // }
    //
    // pub(crate) fn is_integer_translation(
    //     mut matrix: &Matrix,
    //     mut itx: &mut libc::c_int,
    //     mut ity: &mut libc::c_int,
    // ) -> bool {
    //     if _cairo_matrix_is_translation(matrix) != 0 {
    //         let mut x0_fixed: cairo_fixed_t = _cairo_fixed_from_double(matrix.x0);
    //         let mut y0_fixed: cairo_fixed_t = _cairo_fixed_from_double(matrix.y0);
    //         if _cairo_fixed_is_integer(x0_fixed) != 0
    //             && _cairo_fixed_is_integer(y0_fixed) != 0
    //         {
    //             if !itx.is_null() {
    //                 *itx = _cairo_fixed_integer_part(x0_fixed);
    //             }
    //             if !ity.is_null() {
    //                 *ity = _cairo_fixed_integer_part(y0_fixed);
    //             }
    //             return 1 as libc::c_int;
    //         }
    //     }
    //     return 0 as libc::c_int;
    // }
    //
    // pub(crate) fn has_unity_scale(
    //     mut matrix: &Matrix,
    // ) -> bool {
    //     let mut det: f64 = _cairo_matrix_compute_determinant(matrix);
    //     if fabs(det * det - 1.0f64) < _cairo_fixed_to_double(1 as libc::c_int) {
    //         if fabs(matrix.xy) < _cairo_fixed_to_double(1 as libc::c_int)
    //             && fabs(matrix.yx) < _cairo_fixed_to_double(1 as libc::c_int)
    //         {
    //             return 1 as libc::c_int;
    //         }
    //         if fabs(matrix.xx) < _cairo_fixed_to_double(1 as libc::c_int)
    //             && fabs(matrix.yy) < _cairo_fixed_to_double(1 as libc::c_int)
    //         {
    //             return 1 as libc::c_int;
    //         }
    //     }
    //     return 0 as libc::c_int;
    // }
    //
    // pub(crate) fn is_pixel_exact(
    //     mut matrix: &Matrix,
    // ) -> bool {
    //     let mut x0_fixed: cairo_fixed_t = 0;
    //     let mut y0_fixed: cairo_fixed_t = 0;
    //     if _cairo_matrix_has_unity_scale(matrix) == 0 {
    //         return 0 as libc::c_int;
    //     }
    //     x0_fixed = _cairo_fixed_from_double(matrix.x0);
    //     y0_fixed = _cairo_fixed_from_double(matrix.y0);
    //     return (_cairo_fixed_is_integer(x0_fixed) != 0
    //         && _cairo_fixed_is_integer(y0_fixed) != 0) as libc::c_int;
    // }
    //
    // pub(crate) fn transformed_circle_major_axis(
    //     mut matrix: &Matrix,
    //     mut radius: f64,
    // ) -> f64 {
    //     let mut a: f64 = 0.;
    //     let mut b: f64 = 0.;
    //     let mut c: f64 = 0.;
    //     let mut d: f64 = 0.;
    //     let mut f: f64 = 0.;
    //     let mut g: f64 = 0.;
    //     let mut h: f64 = 0.;
    //     let mut i: f64 = 0.;
    //     let mut j: f64 = 0.;
    //     if _cairo_matrix_has_unity_scale(matrix) != 0 {
    //         return radius;
    //     }
    //     _cairo_matrix_get_affine(
    //         matrix,
    //         &mut a,
    //         &mut b,
    //         &mut c,
    //         &mut d,
    //         0 as &mut f64,
    //         0 as &mut f64,
    //     );
    //     i = a * a + b * b;
    //     j = c * c + d * d;
    //     f = 0.5f64 * (i + j);
    //     g = 0.5f64 * (i - j);
    //     h = a * c + b * d;
    //     return radius * sqrt(f + hypot(g, h));
    // }
    // static mut pixman_identity_transform: pixman_transform_t = {
    //     let mut init = pixman_transform {
    //         matrix: [
    //             [
    //                 (1 as libc::c_int) << 16 as libc::c_int,
    //                 0 as libc::c_int,
    //                 0 as libc::c_int,
    //             ],
    //             [
    //                 0 as libc::c_int,
    //                 (1 as libc::c_int) << 16 as libc::c_int,
    //                 0 as libc::c_int,
    //             ],
    //             [0 as libc::c_int, 0 as libc::c_int, (1 as libc::c_int) << 16 as libc::c_int],
    //         ],
    //     };
    //     init
    // };
    // pub(crate) fn to_pixman_matrix(
    //     mut matrix: &Matrix,
    //     mut pixman_transform: &mut pixman_transform_t,
    //     mut xc: f64,
    //     mut yc: f64,
    // ) -> cairo_status_t {
    //     let mut inv: Matrix = Matrix {
    //         xx: 0.,
    //         yx: 0.,
    //         xy: 0.,
    //         yy: 0.,
    //         x0: 0.,
    //         y0: 0.,
    //     };
    //     let mut max_iterations: libc::c_uint = 0;
    //     (*pixman_transform)
    //         .matrix[0 as libc::c_int
    //         as usize][0 as libc::c_int
    //         as usize] = _cairo_fixed_16_16_from_double(matrix.xx);
    //     (*pixman_transform)
    //         .matrix[0 as libc::c_int
    //         as usize][1 as libc::c_int
    //         as usize] = _cairo_fixed_16_16_from_double(matrix.xy);
    //     (*pixman_transform)
    //         .matrix[0 as libc::c_int
    //         as usize][2 as libc::c_int
    //         as usize] = _cairo_fixed_16_16_from_double(matrix.x0);
    //     (*pixman_transform)
    //         .matrix[1 as libc::c_int
    //         as usize][0 as libc::c_int
    //         as usize] = _cairo_fixed_16_16_from_double(matrix.yx);
    //     (*pixman_transform)
    //         .matrix[1 as libc::c_int
    //         as usize][1 as libc::c_int
    //         as usize] = _cairo_fixed_16_16_from_double(matrix.yy);
    //     (*pixman_transform)
    //         .matrix[1 as libc::c_int
    //         as usize][2 as libc::c_int
    //         as usize] = _cairo_fixed_16_16_from_double(matrix.y0);
    //     (*pixman_transform)
    //         .matrix[2][0] = 0 as libc::c_int;
    //     (*pixman_transform)
    //         .matrix[2][1] = 0 as libc::c_int;
    //     (*pixman_transform)
    //         .matrix[2 as libc::c_int
    //         as usize][2] = (1 as libc::c_int) << 16 as libc::c_int;
    //     if _cairo_matrix_has_unity_scale(matrix) != 0 {
    //         return CAIRO_STATUS_SUCCESS;
    //     }
    //     if fabs(matrix.xx)
    //         > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //             >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //         || fabs(matrix.xy)
    //             > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //         || fabs(matrix.x0)
    //             > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //         || fabs(matrix.yx)
    //             > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //         || fabs(matrix.yy)
    //             > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //         || fabs(matrix.y0)
    //             > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //     {
    //         return _cairo_error(CAIRO_STATUS_INVALID_MATRIX);
    //     }
    //     inv = *matrix;
    //     if cairo_matrix_invert(&mut inv) as libc::c_uint
    //         != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
    //     {
    //         return CAIRO_STATUS_SUCCESS;
    //     }
    //     max_iterations = 5 as libc::c_int as libc::c_uint;
    //     loop {
    //         let mut x: f64 = 0.;
    //         let mut y: f64 = 0.;
    //         let mut vector: pixman_vector_t = pixman_vector_t { vector: [0; 3] };
    //         let mut dx: cairo_fixed_16_16_t = 0;
    //         let mut dy: cairo_fixed_16_16_t = 0;
    //         vector.vector[0] = _cairo_fixed_16_16_from_double(xc);
    //         vector.vector[1] = _cairo_fixed_16_16_from_double(yc);
    //         vector
    //             .vector[2] = (1 as libc::c_int) << 16 as libc::c_int;
    //         if pixman_transform_point_3d(pixman_transform, &mut vector) == 0 {
    //             return CAIRO_STATUS_SUCCESS;
    //         }
    //         x = vector.vector[0] as f64
    //             / ((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 as f64;
    //         y = vector.vector[1] as f64
    //             / ((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 as f64;
    //         cairo_matrix_transform_point(&mut inv, &mut x, &mut y);
    //         x -= xc;
    //         y -= yc;
    //         cairo_matrix_transform_distance(matrix, &mut x, &mut y);
    //         dx = _cairo_fixed_16_16_from_double(x);
    //         dy = _cairo_fixed_16_16_from_double(y);
    //         let ref mut fresh0 = (*pixman_transform)
    //             .matrix[0][2];
    //         *fresh0 -= dx;
    //         let ref mut fresh1 = (*pixman_transform)
    //             .matrix[1][2];
    //         *fresh1 -= dy;
    //         if dx == 0 as libc::c_int && dy == 0 as libc::c_int {
    //             return CAIRO_STATUS_SUCCESS;
    //         }
    //         max_iterations = max_iterations.wrapping_sub(1);
    //         if !(max_iterations != 0) {
    //             break;
    //         }
    //     }
    //     return CAIRO_STATUS_SUCCESS;
    // }

    #[inline]
    pub(crate) fn pixman_nearest_sample(d: f64) -> f64 {
        (d - 0.5f64).ceil()
    }

    // pub(crate) fn is_pixman_translation(
    //     mut matrix: &Matrix,
    //     mut filter: cairo_filter_t,
    //     mut x_offset: &mut libc::c_int,
    //     mut y_offset: &mut libc::c_int,
    // ) -> bool {
    //     let mut tx: f64 = 0.;
    //     let mut ty: f64 = 0.;
    //     if _cairo_matrix_is_translation(matrix) == 0 {
    //         return 0 as libc::c_int;
    //     }
    //     if matrix.x0 == 0.0f64 && matrix.y0 == 0.0f64 {
    //         return 1 as libc::c_int;
    //     }
    //     tx = matrix.x0 + *x_offset as f64;
    //     ty = matrix.y0 + *y_offset as f64;
    //     if filter as libc::c_uint == CAIRO_FILTER_FAST as libc::c_int as libc::c_uint
    //         || filter as libc::c_uint == CAIRO_FILTER_NEAREST as libc::c_int as libc::c_uint
    //     {
    //         tx = _pixman_nearest_sample(tx);
    //         ty = _pixman_nearest_sample(ty);
    //     } else if tx != floor(tx) || ty != floor(ty) {
    //         return 0 as libc::c_int
    //     }
    //     if fabs(tx)
    //         > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //             >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //         || fabs(ty)
    //             > ((((1 as libc::c_int as uint32_t) << 16 as libc::c_int) as pixman_fixed_t
    //                 >> 1 as libc::c_int) - 1 as libc::c_int) as f64
    //     {
    //         return 0 as libc::c_int;
    //     }
    //     *x_offset = _cairo_lround(tx);
    //     *y_offset = _cairo_lround(ty);
    //     return 1 as libc::c_int;
    // }
    //
    // pub(crate) fn to_pixman_matrix_offset(
    //     mut matrix: &Matrix,
    //     mut filter: cairo_filter_t,
    //     mut xc: f64,
    //     mut yc: f64,
    //     mut out_transform: &mut pixman_transform_t,
    //     mut x_offset: &mut libc::c_int,
    //     mut y_offset: &mut libc::c_int,
    // ) -> cairo_status_t {
    //     let mut is_pixman_translation: bool = 0;
    //     is_pixman_translation = _cairo_matrix_is_pixman_translation(
    //         matrix,
    //         filter,
    //         x_offset,
    //         y_offset,
    //     );
    //     if is_pixman_translation != 0 {
    //         *out_transform = pixman_identity_transform;
    //         return CAIRO_INT_STATUS_NOTHING_TO_DO as libc::c_int as cairo_status_t;
    //     } else {
    //         let mut m: Matrix = Matrix {
    //             xx: 0.,
    //             yx: 0.,
    //             xy: 0.,
    //             yy: 0.,
    //             x0: 0.,
    //             y0: 0.,
    //         };
    //         m = *matrix;
    //         cairo_matrix_translate(
    //             &mut m,
    //             *x_offset as f64,
    //             *y_offset as f64,
    //         );
    //         if m.x0 != 0.0f64 || m.y0 != 0.0f64 {
    //             let mut tx: f64 = 0.;
    //             let mut ty: f64 = 0.;
    //             let mut norm: f64 = 0.;
    //             let mut i: libc::c_int = 0;
    //             let mut j: libc::c_int = 0;
    //             tx = m.x0;
    //             ty = m.y0;
    //             norm = if fabs(tx) > fabs(ty) { fabs(tx) } else { fabs(ty) };
    //             i = -(1 as libc::c_int);
    //             while i < 2 as libc::c_int {
    //                 j = -(1 as libc::c_int);
    //                 while j < 2 as libc::c_int {
    //                     let mut x: f64 = 0.;
    //                     let mut y: f64 = 0.;
    //                     let mut den: f64 = 0.;
    //                     let mut new_norm: f64 = 0.;
    //                     den = (m.xx + i as f64) * (m.yy + j as f64)
    //                         - m.xy * m.yx;
    //                     if !(fabs(den) < 2.2204460492503131e-16f64) {
    //                         x = m.y0 * m.xy - m.x0 * (m.yy + j as f64);
    //                         y = m.x0 * m.yx - m.y0 * (m.xx + i as f64);
    //                         den = 1.0 / den;
    //                         x *= den;
    //                         y *= den;
    //                         new_norm = if fabs(x) > fabs(y) { fabs(x) } else { fabs(y) };
    //                         if norm > new_norm {
    //                             norm = new_norm;
    //                             tx = x;
    //                             ty = y;
    //                         }
    //                     }
    //                     j += 2 as libc::c_int;
    //                 }
    //                 i += 2 as libc::c_int;
    //             }
    //             tx = floor(tx);
    //             ty = floor(ty);
    //             *x_offset = -tx as libc::c_int;
    //             *y_offset = -ty as libc::c_int;
    //             cairo_matrix_translate(&mut m, tx, ty);
    //         } else {
    //             *x_offset = 0 as libc::c_int;
    //             *y_offset = 0 as libc::c_int;
    //         }
    //         return _cairo_matrix_to_pixman_matrix(&mut m, out_transform, xc, yc);
    //     };
    // }
}
