use ::libc;
pub type __int128_t = i128;
pub type __uint128_t = bf16;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type cairo_uint64_t = uint64_t;
pub type cairo_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_uquorem64 {
    pub quo: cairo_uint64_t,
    pub rem: cairo_uint64_t,
}
pub type cairo_uquorem64_t = _cairo_uquorem64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_quorem64 {
    pub quo: cairo_int64_t,
    pub rem: cairo_int64_t,
}
pub type cairo_quorem64_t = _cairo_quorem64;
pub type uint128_t = __uint128_t;
pub type int128_t = __int128_t;
pub type cairo_uint128_t = uint128_t;
pub type cairo_int128_t = int128_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_uquorem128 {
    pub quo: cairo_uint128_t,
    pub rem: cairo_uint128_t,
}
pub type cairo_uquorem128_t = _cairo_uquorem128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_quorem128 {
    pub quo: cairo_int128_t,
    pub rem: cairo_int128_t,
}
pub type cairo_quorem128_t = _cairo_quorem128;
#[inline]
unsafe extern "C" fn _cairo_uint64_divrem(
    mut num: cairo_uint64_t,
    mut den: cairo_uint64_t,
) -> cairo_uquorem64_t {
    let mut qr: cairo_uquorem64_t = cairo_uquorem64_t {
        quo: 0,
        rem: 0,
    };
    qr.quo = num.wrapping_div(den);
    qr.rem = num.wrapping_rem(den);
    return qr;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_uint128_divrem(
    mut num: cairo_uint128_t,
    mut den: cairo_uint128_t,
) -> cairo_uquorem128_t {
    let mut qr: cairo_uquorem128_t = cairo_uquorem128_t {
        quo: 0,
        rem: 0,
    };
    qr.quo = num.wrapping_div(den);
    qr.rem = num.wrapping_rem(den);
    return qr;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_int128_divrem(
    mut num: cairo_int128_t,
    mut den: cairo_int128_t,
) -> cairo_quorem128_t {
    let mut num_neg: libc::c_int = (num < 0 as libc::c_int as i128) as libc::c_int;
    let mut den_neg: libc::c_int = (den < 0 as libc::c_int as i128) as libc::c_int;
    let mut uqr: cairo_uquorem128_t = cairo_uquorem128_t {
        quo: 0,
        rem: 0,
    };
    let mut qr: cairo_quorem128_t = cairo_quorem128_t {
        quo: 0,
        rem: 0,
    };
    if num_neg != 0 {
        num = -num;
    }
    if den_neg != 0 {
        den = -den;
    }
    uqr = _cairo_uint128_divrem(num as cairo_uint128_t, den as cairo_uint128_t);
    if num_neg != 0 {
        qr.rem = (uqr.rem).wrapping_neg() as cairo_int128_t;
    } else {
        qr.rem = uqr.rem as cairo_int128_t;
    }
    if num_neg != den_neg {
        qr.quo = (uqr.quo).wrapping_neg() as cairo_int128_t;
    } else {
        qr.quo = uqr.quo as cairo_int128_t;
    }
    return qr;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_uint_96by64_32x64_divrem(
    mut num: cairo_uint128_t,
    mut den: cairo_uint64_t,
) -> cairo_uquorem64_t {
    let mut result: cairo_uquorem64_t = cairo_uquorem64_t {
        quo: 0,
        rem: 0,
    };
    let mut B: cairo_uint64_t = (1 as libc::c_int as uint64_t) << 32 as libc::c_int
        | 0 as libc::c_int as libc::c_ulong;
    let mut x: cairo_uint64_t = (num >> 32 as libc::c_int) as uint64_t;
    result
        .quo = ((1 as libc::c_uint).wrapping_neg() as uint64_t) << 32 as libc::c_int
        | (1 as libc::c_uint).wrapping_neg() as libc::c_ulong;
    result.rem = den;
    if !(x < den) {
        return result;
    }
    if x < B {
        return _cairo_uint64_divrem(num as uint64_t, den)
    } else {
        let mut y: uint32_t = num as uint32_t;
        let mut u: uint32_t = (den >> 32 as libc::c_int) as uint32_t;
        let mut v: uint32_t = den as uint32_t;
        let mut quorem: cairo_uquorem64_t = cairo_uquorem64_t {
            quo: 0,
            rem: 0,
        };
        let mut remainder: cairo_uint64_t = 0;
        let mut quotient: uint32_t = 0;
        let mut q: uint32_t = 0;
        let mut r: uint32_t = 0;
        if u.wrapping_add(1 as libc::c_int as libc::c_uint) != 0 {
            quorem = _cairo_uint64_divrem(
                x,
                u.wrapping_add(1 as libc::c_int as libc::c_uint) as uint64_t,
            );
            q = quorem.quo as uint32_t;
            r = quorem.rem as uint32_t;
        } else {
            q = (x >> 32 as libc::c_int) as uint32_t;
            r = x as uint32_t;
        }
        quotient = q;
        if v != 0 {
            quorem = _cairo_uint64_divrem(
                (q as uint64_t).wrapping_mul(v.wrapping_neg() as libc::c_ulong),
                den,
            );
        } else {
            quorem = _cairo_uint64_divrem(
                (q as uint64_t) << 32 as libc::c_int | 0 as libc::c_int as libc::c_ulong,
                den,
            );
        }
        quotient = (quotient as libc::c_uint).wrapping_add(quorem.quo as uint32_t)
            as uint32_t as uint32_t;
        remainder = (r as uint64_t) << 32 as libc::c_int | y as libc::c_ulong;
        if !(remainder < den) {
            remainder = remainder.wrapping_sub(den);
            quotient = quotient.wrapping_add(1);
        }
        remainder = remainder.wrapping_add(quorem.rem);
        if !(remainder < den) || remainder < quorem.rem {
            remainder = remainder.wrapping_sub(den);
            quotient = quotient.wrapping_add(1);
        }
        result.quo = quotient as uint64_t;
        result.rem = remainder;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_int_96by64_32x64_divrem(
    mut num: cairo_int128_t,
    mut den: cairo_int64_t,
) -> cairo_quorem64_t {
    let mut num_neg: libc::c_int = (num < 0 as libc::c_int as i128) as libc::c_int;
    let mut den_neg: libc::c_int = (den < 0 as libc::c_int as libc::c_long)
        as libc::c_int;
    let mut nonneg_den: cairo_uint64_t = 0;
    let mut uqr: cairo_uquorem64_t = cairo_uquorem64_t {
        quo: 0,
        rem: 0,
    };
    let mut qr: cairo_quorem64_t = cairo_quorem64_t { quo: 0, rem: 0 };
    if num_neg != 0 {
        num = -num;
    }
    if den_neg != 0 {
        nonneg_den = -den as cairo_uint64_t;
    } else {
        nonneg_den = den as cairo_uint64_t;
    }
    uqr = _cairo_uint_96by64_32x64_divrem(num as cairo_uint128_t, nonneg_den);
    if uqr.rem == nonneg_den {
        qr
            .quo = ((0x7fffffff as libc::c_int as uint64_t) << 32 as libc::c_int
            | (1 as libc::c_uint).wrapping_neg() as libc::c_ulong) as cairo_int64_t;
        qr.rem = den;
        return qr;
    }
    if num_neg != 0 {
        qr.rem = (uqr.rem).wrapping_neg() as cairo_int64_t;
    } else {
        qr.rem = uqr.rem as cairo_int64_t;
    }
    if num_neg != den_neg {
        qr.quo = (uqr.quo).wrapping_neg() as cairo_int64_t;
    } else {
        qr.quo = uqr.quo as cairo_int64_t;
    }
    return qr;
}
