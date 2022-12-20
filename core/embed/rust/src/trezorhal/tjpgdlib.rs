#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    fn memset(
        _: *mut cty::c_void,
        _: cty::c_int,
        _: cty::c_ulong,
    ) -> *mut cty::c_void;
}
pub type size_t = cty::c_ulong;
pub type int16_t = cty::c_short;
pub type int32_t = cty::c_int;
pub type uint8_t = cty::c_uchar;
pub type uint16_t = cty::c_ushort;
pub type uint32_t = cty::c_uint;
pub type jd_yuv_t = int16_t;
pub type JRESULT = cty::c_uint;
pub const JDR_FMT3: JRESULT = 8;
pub const JDR_FMT2: JRESULT = 7;
pub const JDR_FMT1: JRESULT = 6;
pub const JDR_PAR: JRESULT = 5;
pub const JDR_MEM2: JRESULT = 4;
pub const JDR_MEM1: JRESULT = 3;
pub const JDR_INP: JRESULT = 2;
pub const JDR_INTR: JRESULT = 1;
pub const JDR_OK: JRESULT = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JRECT {
    pub left: uint16_t,
    pub right: uint16_t,
    pub top: uint16_t,
    pub bottom: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JDEC {
    pub dctr: size_t,
    pub dptr: *mut uint8_t,
    pub inbuf: *mut uint8_t,
    pub dbit: uint8_t,
    pub scale: uint8_t,
    pub msx: uint8_t,
    pub msy: uint8_t,
    pub qtid: [uint8_t; 3],
    pub ncomp: uint8_t,
    pub dcv: [int16_t; 3],
    pub nrst: uint16_t,
    pub rst: uint16_t,
    pub rsc: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub huffbits: [[*mut uint8_t; 2]; 2],
    pub huffcode: [[*mut uint16_t; 2]; 2],
    pub huffdata: [[*mut uint8_t; 2]; 2],
    pub qttbl: [*mut int32_t; 4],
    pub wreg: uint32_t,
    pub marker: uint8_t,
    pub longofs: [[uint8_t; 2]; 2],
    pub hufflut_ac: [*mut uint16_t; 2],
    pub hufflut_dc: [*mut uint8_t; 2],
    pub workbuf: *mut cty::c_void,
    pub mcubuf: *mut jd_yuv_t,
    pub pool: *mut cty::c_void,
    pub sz_pool: size_t,
    pub infunc: Option::<
        unsafe extern "C" fn(*mut JDEC, *mut uint8_t, size_t) -> size_t,
    >,
    pub device: *mut cty::c_void,
}
static mut Zig: [uint8_t; 64] = [
    0 as cty::c_int as uint8_t,
    1 as cty::c_int as uint8_t,
    8 as cty::c_int as uint8_t,
    16 as cty::c_int as uint8_t,
    9 as cty::c_int as uint8_t,
    2 as cty::c_int as uint8_t,
    3 as cty::c_int as uint8_t,
    10 as cty::c_int as uint8_t,
    17 as cty::c_int as uint8_t,
    24 as cty::c_int as uint8_t,
    32 as cty::c_int as uint8_t,
    25 as cty::c_int as uint8_t,
    18 as cty::c_int as uint8_t,
    11 as cty::c_int as uint8_t,
    4 as cty::c_int as uint8_t,
    5 as cty::c_int as uint8_t,
    12 as cty::c_int as uint8_t,
    19 as cty::c_int as uint8_t,
    26 as cty::c_int as uint8_t,
    33 as cty::c_int as uint8_t,
    40 as cty::c_int as uint8_t,
    48 as cty::c_int as uint8_t,
    41 as cty::c_int as uint8_t,
    34 as cty::c_int as uint8_t,
    27 as cty::c_int as uint8_t,
    20 as cty::c_int as uint8_t,
    13 as cty::c_int as uint8_t,
    6 as cty::c_int as uint8_t,
    7 as cty::c_int as uint8_t,
    14 as cty::c_int as uint8_t,
    21 as cty::c_int as uint8_t,
    28 as cty::c_int as uint8_t,
    35 as cty::c_int as uint8_t,
    42 as cty::c_int as uint8_t,
    49 as cty::c_int as uint8_t,
    56 as cty::c_int as uint8_t,
    57 as cty::c_int as uint8_t,
    50 as cty::c_int as uint8_t,
    43 as cty::c_int as uint8_t,
    36 as cty::c_int as uint8_t,
    29 as cty::c_int as uint8_t,
    22 as cty::c_int as uint8_t,
    15 as cty::c_int as uint8_t,
    23 as cty::c_int as uint8_t,
    30 as cty::c_int as uint8_t,
    37 as cty::c_int as uint8_t,
    44 as cty::c_int as uint8_t,
    51 as cty::c_int as uint8_t,
    58 as cty::c_int as uint8_t,
    59 as cty::c_int as uint8_t,
    52 as cty::c_int as uint8_t,
    45 as cty::c_int as uint8_t,
    38 as cty::c_int as uint8_t,
    31 as cty::c_int as uint8_t,
    39 as cty::c_int as uint8_t,
    46 as cty::c_int as uint8_t,
    53 as cty::c_int as uint8_t,
    60 as cty::c_int as uint8_t,
    61 as cty::c_int as uint8_t,
    54 as cty::c_int as uint8_t,
    47 as cty::c_int as uint8_t,
    55 as cty::c_int as uint8_t,
    62 as cty::c_int as uint8_t,
    63 as cty::c_int as uint8_t,
];
static mut Ipsf: [uint16_t; 64] = [
    (1.00000f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.38704f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.30656f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.17588f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.00000f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.78570f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.54120f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.27590f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.38704f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.92388f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.81226f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.63099f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.38704f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.08979f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.75066f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.38268f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.30656f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.81226f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.70711f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.53636f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.30656f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.02656f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.70711f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.36048f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.17588f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.63099f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.53636f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.38268f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.17588f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.92388f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.63638f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.32442f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.00000f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.38704f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.30656f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.17588f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.00000f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.78570f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.54120f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.27590f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.78570f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.08979f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (1.02656f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.92388f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.78570f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.61732f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.42522f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.21677f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.54120f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.75066f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.70711f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.63638f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.54120f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.42522f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.29290f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.14932f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.27590f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.38268f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.36048f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.32442f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.27590f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.21678f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.14932f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
    (0.07612f64 * 8192 as cty::c_int as cty::c_double) as uint16_t,
];
unsafe extern "C" fn BYTECLIP(mut val: cty::c_int) -> uint8_t {
    if val < 0 as cty::c_int {
        return 0 as cty::c_int as uint8_t;
    }
    if val > 255 as cty::c_int {
        return 255 as cty::c_int as uint8_t;
    }
    return val as uint8_t;
}
unsafe extern "C" fn alloc_pool(
    mut jd: *mut JDEC,
    mut ndata: size_t,
) -> *mut cty::c_void {
    unsafe {
        let mut rp: *mut cty::c_char = 0 as *mut cty::c_char;
        ndata = ndata.wrapping_add(3 as cty::c_int as cty::c_ulong)
            & !(3 as cty::c_int) as cty::c_ulong;
        if (*jd).sz_pool >= ndata {
            let ref mut fresh0 = (*jd).sz_pool;
            *fresh0 = (*fresh0 as cty::c_ulong).wrapping_sub(ndata) as size_t as size_t;
            rp = (*jd).pool as *mut cty::c_char;
            let ref mut fresh1 = (*jd).pool;
            *fresh1 = rp.offset(ndata as isize) as *mut cty::c_void;
        }
        return rp as *mut cty::c_void;
    }
}
unsafe extern "C" fn create_qt_tbl(
    mut jd: *mut JDEC,
    mut data: *const uint8_t,
    mut ndata: size_t,
) -> JRESULT {
    unsafe {
        let mut i: cty::c_uint = 0;
        let mut zi: cty::c_uint = 0;
        let mut d: uint8_t = 0;
        let mut pb: *mut int32_t = 0 as *mut int32_t;
        while ndata != 0 {
            if ndata < 65 as cty::c_int as cty::c_ulong {
                return JDR_FMT1;
            }
            ndata = (ndata as cty::c_ulong).wrapping_sub(65 as cty::c_int as cty::c_ulong)
                as size_t as size_t;
            let fresh2 = data;
            data = data.offset(1);
            d = *fresh2;
            if d as cty::c_int & 0xf0 as cty::c_int != 0 {
                return JDR_FMT1;
            }
            i = (d as cty::c_int & 3 as cty::c_int) as cty::c_uint;
            pb = alloc_pool(
                jd,
                (64 as cty::c_int as cty::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<int32_t>() as cty::c_ulong),
            ) as *mut int32_t;
            if pb.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh3 = (*jd).qttbl[i as usize];
            *fresh3 = pb;
            i = 0 as cty::c_int as cty::c_uint;
            while i < 64 as cty::c_int as cty::c_uint {
                zi = Zig[i as usize] as cty::c_uint;
                let fresh4 = data;
                data = data.offset(1);
                *pb
                    .offset(
                        zi as isize,
                    ) = (*fresh4 as uint32_t).wrapping_mul(Ipsf[zi as usize] as cty::c_uint)
                    as int32_t;
                i = i.wrapping_add(1);
            }
        }
        return JDR_OK;
    }
}
unsafe extern "C" fn create_huffman_tbl(
    mut jd: *mut JDEC,
    mut data: *const uint8_t,
    mut ndata: size_t,
) -> JRESULT {
    unsafe {
        let mut i: cty::c_uint = 0;
        let mut j: cty::c_uint = 0;
        let mut b: cty::c_uint = 0;
        let mut cls: cty::c_uint = 0;
        let mut num: cty::c_uint = 0;
        let mut np: size_t = 0;
        let mut d: uint8_t = 0;
        let mut pb: *mut uint8_t = 0 as *mut uint8_t;
        let mut pd: *mut uint8_t = 0 as *mut uint8_t;
        let mut hc: uint16_t = 0;
        let mut ph: *mut uint16_t = 0 as *mut uint16_t;
        while ndata != 0 {
            if ndata < 17 as cty::c_int as cty::c_ulong {
                return JDR_FMT1;
            }
            ndata = (ndata as cty::c_ulong).wrapping_sub(17 as cty::c_int as cty::c_ulong)
                as size_t as size_t;
            let fresh5 = data;
            data = data.offset(1);
            d = *fresh5;
            if d as cty::c_int & 0xee as cty::c_int != 0 {
                return JDR_FMT1;
            }
            cls = (d as cty::c_int >> 4 as cty::c_int) as cty::c_uint;
            num = (d as cty::c_int & 0xf as cty::c_int) as cty::c_uint;
            pb = alloc_pool(jd, 16 as cty::c_int as size_t) as *mut uint8_t;
            if pb.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh6 = (*jd).huffbits[num as usize][cls as usize];
            *fresh6 = pb;
            i = 0 as cty::c_int as cty::c_uint;
            np = i as size_t;
            while i < 16 as cty::c_int as cty::c_uint {
                let fresh7 = data;
                data = data.offset(1);
                let ref mut fresh8 = *pb.offset(i as isize);
                *fresh8 = *fresh7;
                np = (np as cty::c_ulong).wrapping_add(*fresh8 as cty::c_ulong) as size_t
                    as size_t;
                i = i.wrapping_add(1);
            }
            ph = alloc_pool(
                jd,
                np.wrapping_mul(::core::mem::size_of::<uint16_t>() as cty::c_ulong),
            ) as *mut uint16_t;
            if ph.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh9 = (*jd).huffcode[num as usize][cls as usize];
            *fresh9 = ph;
            hc = 0 as cty::c_int as uint16_t;
            i = 0 as cty::c_int as cty::c_uint;
            j = i;
            while i < 16 as cty::c_int as cty::c_uint {
                b = *pb.offset(i as isize) as cty::c_uint;
                loop {
                    let fresh10 = b;
                    b = b.wrapping_sub(1);
                    if !(fresh10 != 0) {
                        break;
                    }
                    let fresh11 = hc;
                    hc = hc.wrapping_add(1);
                    let fresh12 = j;
                    j = j.wrapping_add(1);
                    *ph.offset(fresh12 as isize) = fresh11;
                }
                hc = ((hc as cty::c_int) << 1 as cty::c_int) as uint16_t;
                i = i.wrapping_add(1);
            }
            if ndata < np {
                return JDR_FMT1;
            }
            ndata = (ndata as cty::c_ulong).wrapping_sub(np) as size_t as size_t;
            pd = alloc_pool(jd, np) as *mut uint8_t;
            if pd.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh13 = (*jd).huffdata[num as usize][cls as usize];
            *fresh13 = pd;
            i = 0 as cty::c_int as cty::c_uint;
            while (i as cty::c_ulong) < np {
                let fresh14 = data;
                data = data.offset(1);
                d = *fresh14;
                if cls == 0 && d as cty::c_int > 11 as cty::c_int {
                    return JDR_FMT1;
                }
                *pd.offset(i as isize) = d;
                i = i.wrapping_add(1);
            }
            let mut span: cty::c_uint = 0;
            let mut td: cty::c_uint = 0;
            let mut ti: cty::c_uint = 0;
            let mut tbl_ac: *mut uint16_t = 0 as *mut uint16_t;
            let mut tbl_dc: *mut uint8_t = 0 as *mut uint8_t;
            if cls != 0 {
                tbl_ac = alloc_pool(
                    jd,
                    (((1 as cty::c_int) << 10 as cty::c_int) as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint16_t>() as cty::c_ulong),
                ) as *mut uint16_t;
                if tbl_ac.is_null() {
                    return JDR_MEM1;
                }
                let ref mut fresh15 = (*jd).hufflut_ac[num as usize];
                *fresh15 = tbl_ac;
                memset(
                    tbl_ac as *mut cty::c_void,
                    0xff as cty::c_int,
                    (((1 as cty::c_int) << 10 as cty::c_int) as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint16_t>() as cty::c_ulong),
                );
            } else {
                tbl_dc = alloc_pool(
                    jd,
                    (((1 as cty::c_int) << 10 as cty::c_int) as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as cty::c_ulong),
                ) as *mut uint8_t;
                if tbl_dc.is_null() {
                    return JDR_MEM1;
                }
                let ref mut fresh16 = (*jd).hufflut_dc[num as usize];
                *fresh16 = tbl_dc;
                memset(
                    tbl_dc as *mut cty::c_void,
                    0xff as cty::c_int,
                    (((1 as cty::c_int) << 10 as cty::c_int) as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as cty::c_ulong),
                );
            }
            b = 0 as cty::c_int as cty::c_uint;
            i = b;
            while b < 10 as cty::c_int as cty::c_uint {
                j = *pb.offset(b as isize) as cty::c_uint;
                while j != 0 {
                    ti = ((*ph.offset(i as isize) as cty::c_int)
                        << ((10 as cty::c_int - 1 as cty::c_int) as cty::c_uint)
                        .wrapping_sub(b)
                        & ((1 as cty::c_int) << 10 as cty::c_int) - 1 as cty::c_int)
                        as cty::c_uint;
                    if cls != 0 {
                        let fresh17 = i;
                        i = i.wrapping_add(1);
                        td = *pd.offset(fresh17 as isize) as cty::c_uint
                            | b.wrapping_add(1 as cty::c_int as cty::c_uint)
                            << 8 as cty::c_int;
                        span = ((1 as cty::c_int)
                            << ((10 as cty::c_int - 1 as cty::c_int) as cty::c_uint)
                            .wrapping_sub(b)) as cty::c_uint;
                        while span != 0 {
                            span = span.wrapping_sub(1);
                            let fresh18 = ti;
                            ti = ti.wrapping_add(1);
                            *tbl_ac.offset(fresh18 as isize) = td as uint16_t;
                        }
                    } else {
                        let fresh19 = i;
                        i = i.wrapping_add(1);
                        td = *pd.offset(fresh19 as isize) as cty::c_uint
                            | b.wrapping_add(1 as cty::c_int as cty::c_uint)
                            << 4 as cty::c_int;
                        span = ((1 as cty::c_int)
                            << ((10 as cty::c_int - 1 as cty::c_int) as cty::c_uint)
                            .wrapping_sub(b)) as cty::c_uint;
                        while span != 0 {
                            span = span.wrapping_sub(1);
                            let fresh20 = ti;
                            ti = ti.wrapping_add(1);
                            *tbl_dc.offset(fresh20 as isize) = td as uint8_t;
                        }
                    }
                    j = j.wrapping_sub(1);
                }
                b = b.wrapping_add(1);
            }
            (*jd).longofs[num as usize][cls as usize] = i as uint8_t;
        }
        return JDR_OK;
    }
}
unsafe extern "C" fn huffext(
    mut jd: *mut JDEC,
    mut id: cty::c_uint,
    mut cls: cty::c_uint,
) -> cty::c_int {
    unsafe {
        let mut dc: size_t = (*jd).dctr;
        let mut dp: *mut uint8_t = (*jd).dptr;
        let mut d: cty::c_uint = 0;
        let mut flg: cty::c_uint = 0 as cty::c_int as cty::c_uint;
        let mut hb: *const uint8_t = 0 as *const uint8_t;
        let mut hd: *const uint8_t = 0 as *const uint8_t;
        let mut hc: *const uint16_t = 0 as *const uint16_t;
        let mut nc: cty::c_uint = 0;
        let mut bl: cty::c_uint = 0;
        let mut wbit: cty::c_uint = ((*jd).dbit as cty::c_int % 32 as cty::c_int)
            as cty::c_uint;
        let mut w: uint32_t = ((*jd).wreg as cty::c_ulong
            & ((1 as cty::c_ulong) << wbit).wrapping_sub(1 as cty::c_int as cty::c_ulong))
            as uint32_t;
        while wbit < 16 as cty::c_int as cty::c_uint {
            if (*jd).marker != 0 {
                d = 0xff as cty::c_int as cty::c_uint;
            } else {
                if dc == 0 {
                    dp = (*jd).inbuf;
                    dc = ((*jd).infunc)
                        .expect(
                            "non-null function pointer",
                        )(jd, dp, 512 as cty::c_int as size_t);
                    if dc == 0 {
                        return 0 as cty::c_int - JDR_INP as cty::c_int;
                    }
                }
                let fresh21 = dp;
                dp = dp.offset(1);
                d = *fresh21 as cty::c_uint;
                dc = dc.wrapping_sub(1);
                if flg != 0 {
                    flg = 0 as cty::c_int as cty::c_uint;
                    if d != 0 as cty::c_int as cty::c_uint {
                        (*jd).marker = d as uint8_t;
                    }
                    d = 0xff as cty::c_int as cty::c_uint;
                } else if d == 0xff as cty::c_int as cty::c_uint {
                    flg = 1 as cty::c_int as cty::c_uint;
                    continue;
                }
            }
            w = w << 8 as cty::c_int | d;
            wbit = wbit.wrapping_add(8 as cty::c_int as cty::c_uint);
        }
        (*jd).dctr = dc;
        let ref mut fresh22 = (*jd).dptr;
        *fresh22 = dp;
        (*jd).wreg = w;
        d = w >> wbit.wrapping_sub(10 as cty::c_int as cty::c_uint);
        if cls != 0 {
            d = *((*jd).hufflut_ac[id as usize]).offset(d as isize) as cty::c_uint;
            if d != 0xffff as cty::c_int as cty::c_uint {
                (*jd).dbit = wbit.wrapping_sub(d >> 8 as cty::c_int) as uint8_t;
                return (d & 0xff as cty::c_int as cty::c_uint) as cty::c_int;
            }
        } else {
            d = *((*jd).hufflut_dc[id as usize]).offset(d as isize) as cty::c_uint;
            if d != 0xff as cty::c_int as cty::c_uint {
                (*jd).dbit = wbit.wrapping_sub(d >> 4 as cty::c_int) as uint8_t;
                return (d & 0xf as cty::c_int as cty::c_uint) as cty::c_int;
            }
        }
        hb = ((*jd).huffbits[id as usize][cls as usize]).offset(10 as cty::c_int as isize);
        hc = ((*jd).huffcode[id as usize][cls as usize])
            .offset((*jd).longofs[id as usize][cls as usize] as cty::c_int as isize);
        hd = ((*jd).huffdata[id as usize][cls as usize])
            .offset((*jd).longofs[id as usize][cls as usize] as cty::c_int as isize);
        bl = (10 as cty::c_int + 1 as cty::c_int) as cty::c_uint;
        while bl <= 16 as cty::c_int as cty::c_uint {
            let fresh23 = hb;
            hb = hb.offset(1);
            nc = *fresh23 as cty::c_uint;
            if nc != 0 {
                d = w >> wbit.wrapping_sub(bl);
                loop {
                    let fresh24 = hc;
                    hc = hc.offset(1);
                    if d == *fresh24 as cty::c_uint {
                        (*jd).dbit = wbit.wrapping_sub(bl) as uint8_t;
                        return *hd as cty::c_int;
                    }
                    hd = hd.offset(1);
                    nc = nc.wrapping_sub(1);
                    if !(nc != 0) {
                        break;
                    }
                }
            }
            bl = bl.wrapping_add(1);
        }
        return 0 as cty::c_int - JDR_FMT1 as cty::c_int;
    }
}
unsafe extern "C" fn bitext(mut jd: *mut JDEC, mut nbit: cty::c_uint) -> cty::c_int {
    unsafe {
        let mut dc: size_t = (*jd).dctr;
        let mut dp: *mut uint8_t = (*jd).dptr;
        let mut d: cty::c_uint = 0;
        let mut flg: cty::c_uint = 0 as cty::c_int as cty::c_uint;
        let mut wbit: cty::c_uint = ((*jd).dbit as cty::c_int % 32 as cty::c_int)
            as cty::c_uint;
        let mut w: uint32_t = ((*jd).wreg as cty::c_ulong
            & ((1 as cty::c_ulong) << wbit).wrapping_sub(1 as cty::c_int as cty::c_ulong))
            as uint32_t;
        while wbit < nbit {
            if (*jd).marker != 0 {
                d = 0xff as cty::c_int as cty::c_uint;
            } else {
                if dc == 0 {
                    dp = (*jd).inbuf;
                    dc = ((*jd).infunc)
                        .expect(
                            "non-null function pointer",
                        )(jd, dp, 512 as cty::c_int as size_t);
                    if dc == 0 {
                        return 0 as cty::c_int - JDR_INP as cty::c_int;
                    }
                }
                let fresh25 = dp;
                dp = dp.offset(1);
                d = *fresh25 as cty::c_uint;
                dc = dc.wrapping_sub(1);
                if flg != 0 {
                    flg = 0 as cty::c_int as cty::c_uint;
                    if d != 0 as cty::c_int as cty::c_uint {
                        (*jd).marker = d as uint8_t;
                    }
                    d = 0xff as cty::c_int as cty::c_uint;
                } else if d == 0xff as cty::c_int as cty::c_uint {
                    flg = 1 as cty::c_int as cty::c_uint;
                    continue;
                }
            }
            w = w << 8 as cty::c_int | d;
            wbit = wbit.wrapping_add(8 as cty::c_int as cty::c_uint);
        }
        (*jd).wreg = w;
        (*jd).dbit = wbit.wrapping_sub(nbit) as uint8_t;
        (*jd).dctr = dc;
        let ref mut fresh26 = (*jd).dptr;
        *fresh26 = dp;
        return (w >> wbit.wrapping_sub(nbit).wrapping_rem(32 as cty::c_int as cty::c_uint))
            as cty::c_int;
    }
}
unsafe extern "C" fn restart(mut jd: *mut JDEC, mut rstn: uint16_t) -> JRESULT {
    unsafe {
        let mut i: cty::c_uint = 0;
        let mut dp: *mut uint8_t = (*jd).dptr;
        let mut dc: size_t = (*jd).dctr;
        let mut marker: uint16_t = 0;
        if (*jd).marker != 0 {
            marker = (0xff00 as cty::c_int | (*jd).marker as cty::c_int) as uint16_t;
            (*jd).marker = 0 as cty::c_int as uint8_t;
        } else {
            marker = 0 as cty::c_int as uint16_t;
            i = 0 as cty::c_int as cty::c_uint;
            while i < 2 as cty::c_int as cty::c_uint {
                if dc == 0 {
                    dp = (*jd).inbuf;
                    dc = ((*jd).infunc)
                        .expect(
                            "non-null function pointer",
                        )(jd, dp, 512 as cty::c_int as size_t);
                    if dc == 0 {
                        return JDR_INP;
                    }
                }
                let fresh27 = dp;
                dp = dp.offset(1);
                marker = ((marker as cty::c_int) << 8 as cty::c_int
                    | *fresh27 as cty::c_int) as uint16_t;
                dc = dc.wrapping_sub(1);
                i = i.wrapping_add(1);
            }
            let ref mut fresh28 = (*jd).dptr;
            *fresh28 = dp;
            (*jd).dctr = dc;
        }
        if marker as cty::c_int & 0xffd8 as cty::c_int != 0xffd0 as cty::c_int
            || marker as cty::c_int & 7 as cty::c_int
            != rstn as cty::c_int & 7 as cty::c_int
        {
            return JDR_FMT1;
        }
        (*jd).dbit = 0 as cty::c_int as uint8_t;
        let ref mut fresh29 = (*jd).dcv[0 as cty::c_int as usize];
        *fresh29 = 0 as cty::c_int as int16_t;
        let ref mut fresh30 = (*jd).dcv[1 as cty::c_int as usize];
        *fresh30 = *fresh29;
        (*jd).dcv[2 as cty::c_int as usize] = *fresh30;
        return JDR_OK;
    }
}
unsafe extern "C" fn block_idct(mut src: *mut int32_t, mut dst: *mut jd_yuv_t) {
    unsafe {
        let M13: int32_t = (1.41421f64 * 4096 as cty::c_int as cty::c_double) as int32_t;
        let M2: int32_t = (1.08239f64 * 4096 as cty::c_int as cty::c_double) as int32_t;
        let M4: int32_t = (2.61313f64 * 4096 as cty::c_int as cty::c_double) as int32_t;
        let M5: int32_t = (1.84776f64 * 4096 as cty::c_int as cty::c_double) as int32_t;
        let mut v0: int32_t = 0;
        let mut v1: int32_t = 0;
        let mut v2: int32_t = 0;
        let mut v3: int32_t = 0;
        let mut v4: int32_t = 0;
        let mut v5: int32_t = 0;
        let mut v6: int32_t = 0;
        let mut v7: int32_t = 0;
        let mut t10: int32_t = 0;
        let mut t11: int32_t = 0;
        let mut t12: int32_t = 0;
        let mut t13: int32_t = 0;
        let mut i: cty::c_int = 0;
        i = 0 as cty::c_int;
        while i < 8 as cty::c_int {
            v0 = *src.offset((8 as cty::c_int * 0 as cty::c_int) as isize);
            v1 = *src.offset((8 as cty::c_int * 2 as cty::c_int) as isize);
            v2 = *src.offset((8 as cty::c_int * 4 as cty::c_int) as isize);
            v3 = *src.offset((8 as cty::c_int * 6 as cty::c_int) as isize);
            t10 = v0 + v2;
            t12 = v0 - v2;
            t11 = (v1 - v3) * M13 >> 12 as cty::c_int;
            v3 += v1;
            t11 -= v3;
            v0 = t10 + v3;
            v3 = t10 - v3;
            v1 = t11 + t12;
            v2 = t12 - t11;
            v4 = *src.offset((8 as cty::c_int * 7 as cty::c_int) as isize);
            v5 = *src.offset((8 as cty::c_int * 1 as cty::c_int) as isize);
            v6 = *src.offset((8 as cty::c_int * 5 as cty::c_int) as isize);
            v7 = *src.offset((8 as cty::c_int * 3 as cty::c_int) as isize);
            t10 = v5 - v4;
            t11 = v5 + v4;
            t12 = v6 - v7;
            v7 += v6;
            v5 = (t11 - v7) * M13 >> 12 as cty::c_int;
            v7 += t11;
            t13 = (t10 + t12) * M5 >> 12 as cty::c_int;
            v4 = t13 - (t10 * M2 >> 12 as cty::c_int);
            v6 = t13 - (t12 * M4 >> 12 as cty::c_int) - v7;
            v5 -= v6;
            v4 -= v5;
            *src.offset((8 as cty::c_int * 0 as cty::c_int) as isize) = v0 + v7;
            *src.offset((8 as cty::c_int * 7 as cty::c_int) as isize) = v0 - v7;
            *src.offset((8 as cty::c_int * 1 as cty::c_int) as isize) = v1 + v6;
            *src.offset((8 as cty::c_int * 6 as cty::c_int) as isize) = v1 - v6;
            *src.offset((8 as cty::c_int * 2 as cty::c_int) as isize) = v2 + v5;
            *src.offset((8 as cty::c_int * 5 as cty::c_int) as isize) = v2 - v5;
            *src.offset((8 as cty::c_int * 3 as cty::c_int) as isize) = v3 + v4;
            *src.offset((8 as cty::c_int * 4 as cty::c_int) as isize) = v3 - v4;
            src = src.offset(1);
            i += 1;
        }
        src = src.offset(-(8 as cty::c_int as isize));
        i = 0 as cty::c_int;
        while i < 8 as cty::c_int {
            v0 = (*src.offset(0 as cty::c_int as isize) as cty::c_long
                + ((128 as cty::c_long) << 8 as cty::c_int)) as int32_t;
            v1 = *src.offset(2 as cty::c_int as isize);
            v2 = *src.offset(4 as cty::c_int as isize);
            v3 = *src.offset(6 as cty::c_int as isize);
            t10 = v0 + v2;
            t12 = v0 - v2;
            t11 = (v1 - v3) * M13 >> 12 as cty::c_int;
            v3 += v1;
            t11 -= v3;
            v0 = t10 + v3;
            v3 = t10 - v3;
            v1 = t11 + t12;
            v2 = t12 - t11;
            v4 = *src.offset(7 as cty::c_int as isize);
            v5 = *src.offset(1 as cty::c_int as isize);
            v6 = *src.offset(5 as cty::c_int as isize);
            v7 = *src.offset(3 as cty::c_int as isize);
            t10 = v5 - v4;
            t11 = v5 + v4;
            t12 = v6 - v7;
            v7 += v6;
            v5 = (t11 - v7) * M13 >> 12 as cty::c_int;
            v7 += t11;
            t13 = (t10 + t12) * M5 >> 12 as cty::c_int;
            v4 = t13 - (t10 * M2 >> 12 as cty::c_int);
            v6 = t13 - (t12 * M4 >> 12 as cty::c_int) - v7;
            v5 -= v6;
            v4 -= v5;
            *dst
                .offset(
                    0 as cty::c_int as isize,
                ) = (v0 + v7 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    7 as cty::c_int as isize,
                ) = (v0 - v7 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    1 as cty::c_int as isize,
                ) = (v1 + v6 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    6 as cty::c_int as isize,
                ) = (v1 - v6 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    2 as cty::c_int as isize,
                ) = (v2 + v5 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    5 as cty::c_int as isize,
                ) = (v2 - v5 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    3 as cty::c_int as isize,
                ) = (v3 + v4 >> 8 as cty::c_int) as int16_t;
            *dst
                .offset(
                    4 as cty::c_int as isize,
                ) = (v3 - v4 >> 8 as cty::c_int) as int16_t;
            dst = dst.offset(8 as cty::c_int as isize);
            src = src.offset(8 as cty::c_int as isize);
            i += 1;
        }
    }
}
unsafe extern "C" fn mcu_load(mut jd: *mut JDEC) -> JRESULT {
    unsafe {
        let mut tmp: *mut int32_t = (*jd).workbuf as *mut int32_t;
        let mut d: cty::c_int = 0;
        let mut e: cty::c_int = 0;
        let mut blk: cty::c_uint = 0;
        let mut nby: cty::c_uint = 0;
        let mut i: cty::c_uint = 0;
        let mut bc: cty::c_uint = 0;
        let mut z: cty::c_uint = 0;
        let mut id: cty::c_uint = 0;
        let mut cmp: cty::c_uint = 0;
        let mut bp: *mut jd_yuv_t = 0 as *mut jd_yuv_t;
        let mut dqf: *const int32_t = 0 as *const int32_t;
        nby = ((*jd).msx as cty::c_int * (*jd).msy as cty::c_int) as cty::c_uint;
        bp = (*jd).mcubuf;
        blk = 0 as cty::c_int as cty::c_uint;
        while blk < nby.wrapping_add(2 as cty::c_int as cty::c_uint) {
            cmp = if blk < nby {
                0 as cty::c_int as cty::c_uint
            } else {
                blk.wrapping_sub(nby).wrapping_add(1 as cty::c_int as cty::c_uint)
            };
            if cmp != 0 && (*jd).ncomp as cty::c_int != 3 as cty::c_int {
                i = 0 as cty::c_int as cty::c_uint;
                while i < 64 as cty::c_int as cty::c_uint {
                    let fresh31 = i;
                    i = i.wrapping_add(1);
                    *bp.offset(fresh31 as isize) = 128 as cty::c_int as jd_yuv_t;
                }
            } else {
                id = (if cmp != 0 { 1 as cty::c_int } else { 0 as cty::c_int })
                    as cty::c_uint;
                d = huffext(jd, id, 0 as cty::c_int as cty::c_uint);
                if d < 0 as cty::c_int {
                    return (0 as cty::c_int - d) as JRESULT;
                }
                bc = d as cty::c_uint;
                d = (*jd).dcv[cmp as usize] as cty::c_int;
                if bc != 0 {
                    e = bitext(jd, bc);
                    if e < 0 as cty::c_int {
                        return (0 as cty::c_int - e) as JRESULT;
                    }
                    bc = ((1 as cty::c_int)
                        << bc.wrapping_sub(1 as cty::c_int as cty::c_uint))
                        as cty::c_uint;
                    if e as cty::c_uint & bc == 0 {
                        e = (e as cty::c_uint)
                            .wrapping_sub(
                                (bc << 1 as cty::c_int)
                                    .wrapping_sub(1 as cty::c_int as cty::c_uint),
                            ) as cty::c_int as cty::c_int;
                    }
                    d += e;
                    (*jd).dcv[cmp as usize] = d as int16_t;
                }
                dqf = (*jd).qttbl[(*jd).qtid[cmp as usize] as usize];
                *tmp
                    .offset(
                        0 as cty::c_int as isize,
                    ) = d * *dqf.offset(0 as cty::c_int as isize) >> 8 as cty::c_int;
                memset(
                    &mut *tmp.offset(1 as cty::c_int as isize) as *mut int32_t
                        as *mut cty::c_void,
                    0 as cty::c_int,
                    (63 as cty::c_int as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int32_t>() as cty::c_ulong),
                );
                z = 1 as cty::c_int as cty::c_uint;
                loop {
                    d = huffext(jd, id, 1 as cty::c_int as cty::c_uint);
                    if d == 0 as cty::c_int {
                        break;
                    }
                    if d < 0 as cty::c_int {
                        return (0 as cty::c_int - d) as JRESULT;
                    }
                    bc = d as cty::c_uint;
                    z = z.wrapping_add(bc >> 4 as cty::c_int);
                    if z >= 64 as cty::c_int as cty::c_uint {
                        return JDR_FMT1;
                    }
                    bc &= 0xf as cty::c_int as cty::c_uint;
                    if bc != 0 {
                        d = bitext(jd, bc);
                        if d < 0 as cty::c_int {
                            return (0 as cty::c_int - d) as JRESULT;
                        }
                        bc = ((1 as cty::c_int)
                            << bc.wrapping_sub(1 as cty::c_int as cty::c_uint))
                            as cty::c_uint;
                        if d as cty::c_uint & bc == 0 {
                            d = (d as cty::c_uint)
                                .wrapping_sub(
                                    (bc << 1 as cty::c_int)
                                        .wrapping_sub(1 as cty::c_int as cty::c_uint),
                                ) as cty::c_int as cty::c_int;
                        }
                        i = Zig[z as usize] as cty::c_uint;
                        *tmp
                            .offset(
                                i as isize,
                            ) = d * *dqf.offset(i as isize) >> 8 as cty::c_int;
                    }
                    z = z.wrapping_add(1);
                    if !(z < 64 as cty::c_int as cty::c_uint) {
                        break;
                    }
                }
                if 1 as cty::c_int != 2 as cty::c_int || cmp == 0 {
                    if z == 1 as cty::c_int as cty::c_uint
                        || 0 as cty::c_int != 0
                        && (*jd).scale as cty::c_int == 3 as cty::c_int
                    {
                        d = (*tmp / 256 as cty::c_int + 128 as cty::c_int) as jd_yuv_t
                            as cty::c_int;
                        if 2 as cty::c_int >= 1 as cty::c_int {
                            i = 0 as cty::c_int as cty::c_uint;
                            while i < 64 as cty::c_int as cty::c_uint {
                                let fresh32 = i;
                                i = i.wrapping_add(1);
                                *bp.offset(fresh32 as isize) = d as jd_yuv_t;
                            }
                        } else {
                            memset(
                                bp as *mut cty::c_void,
                                d,
                                64 as cty::c_int as cty::c_ulong,
                            );
                        }
                    } else {
                        block_idct(tmp, bp);
                    }
                }
            }
            bp = bp.offset(64 as cty::c_int as isize);
            blk = blk.wrapping_add(1);
        }
        return JDR_OK;
    }
}
unsafe extern "C" fn mcu_output(
    mut jd: *mut JDEC,
    mut outfunc: Option::<
        unsafe extern "C" fn(*mut JDEC, *mut cty::c_void, *mut JRECT) -> cty::c_int,
    >,
    mut x: cty::c_uint,
    mut y: cty::c_uint,
) -> JRESULT {
    unsafe {
        let CVACC: cty::c_int = if ::core::mem::size_of::<cty::c_int>() as cty::c_ulong
            > 2 as cty::c_int as cty::c_ulong
        {
            1024 as cty::c_int
        } else {
            128 as cty::c_int
        };
        let mut ix: cty::c_uint = 0;
        let mut iy: cty::c_uint = 0;
        let mut mx: cty::c_uint = 0;
        let mut my: cty::c_uint = 0;
        let mut rx: cty::c_uint = 0;
        let mut ry: cty::c_uint = 0;
        let mut yy: cty::c_int = 0;
        let mut cb: cty::c_int = 0;
        let mut cr: cty::c_int = 0;
        let mut py: *mut jd_yuv_t = 0 as *mut jd_yuv_t;
        let mut pc: *mut jd_yuv_t = 0 as *mut jd_yuv_t;
        let mut pix: *mut uint8_t = 0 as *mut uint8_t;
        let mut rect: JRECT = JRECT {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };
        mx = ((*jd).msx as cty::c_int * 8 as cty::c_int) as cty::c_uint;
        my = ((*jd).msy as cty::c_int * 8 as cty::c_int) as cty::c_uint;
        rx = if x.wrapping_add(mx) <= (*jd).width as cty::c_uint {
            mx
        } else {
            ((*jd).width as cty::c_uint).wrapping_sub(x)
        };
        ry = if y.wrapping_add(my) <= (*jd).height as cty::c_uint {
            my
        } else {
            ((*jd).height as cty::c_uint).wrapping_sub(y)
        };
        rect.left = x as uint16_t;
        rect
            .right = x.wrapping_add(rx).wrapping_sub(1 as cty::c_int as cty::c_uint)
            as uint16_t;
        rect.top = y as uint16_t;
        rect
            .bottom = y.wrapping_add(ry).wrapping_sub(1 as cty::c_int as cty::c_uint)
            as uint16_t;
        if 0 as cty::c_int == 0 || (*jd).scale as cty::c_int != 3 as cty::c_int {
            pix = (*jd).workbuf as *mut uint8_t;
            if 1 as cty::c_int != 2 as cty::c_int {
                iy = 0 as cty::c_int as cty::c_uint;
                while iy < my {
                    py = (*jd).mcubuf;
                    pc = py;
                    if my == 16 as cty::c_int as cty::c_uint {
                        pc = pc
                            .offset(
                                ((64 as cty::c_int * 4 as cty::c_int) as cty::c_uint)
                                    .wrapping_add(
                                        (iy >> 1 as cty::c_int)
                                            .wrapping_mul(8 as cty::c_int as cty::c_uint),
                                    ) as isize,
                            );
                        if iy >= 8 as cty::c_int as cty::c_uint {
                            py = py.offset(64 as cty::c_int as isize);
                        }
                    } else {
                        pc = pc
                            .offset(
                                mx
                                    .wrapping_mul(8 as cty::c_int as cty::c_uint)
                                    .wrapping_add(
                                        iy.wrapping_mul(8 as cty::c_int as cty::c_uint),
                                    ) as isize,
                            );
                    }
                    py = py
                        .offset(iy.wrapping_mul(8 as cty::c_int as cty::c_uint) as isize);
                    ix = 0 as cty::c_int as cty::c_uint;
                    while ix < mx {
                        cb = *pc.offset(0 as cty::c_int as isize) as cty::c_int
                            - 128 as cty::c_int;
                        cr = *pc.offset(64 as cty::c_int as isize) as cty::c_int
                            - 128 as cty::c_int;
                        if mx == 16 as cty::c_int as cty::c_uint {
                            if ix == 8 as cty::c_int as cty::c_uint {
                                py = py
                                    .offset((64 as cty::c_int - 8 as cty::c_int) as isize);
                            }
                            pc = pc.offset((ix & 1 as cty::c_int as cty::c_uint) as isize);
                        } else {
                            pc = pc.offset(1);
                        }
                        let fresh33 = py;
                        py = py.offset(1);
                        yy = *fresh33 as cty::c_int;
                        let fresh34 = pix;
                        pix = pix.offset(1);
                        *fresh34 = BYTECLIP(
                            yy
                                + (1.402f64 * CVACC as cty::c_double) as cty::c_int * cr
                                / CVACC,
                        );
                        let fresh35 = pix;
                        pix = pix.offset(1);
                        *fresh35 = BYTECLIP(
                            yy
                                - ((0.344f64 * CVACC as cty::c_double) as cty::c_int * cb
                                + (0.714f64 * CVACC as cty::c_double) as cty::c_int * cr)
                                / CVACC,
                        );
                        let fresh36 = pix;
                        pix = pix.offset(1);
                        *fresh36 = BYTECLIP(
                            yy
                                + (1.772f64 * CVACC as cty::c_double) as cty::c_int * cb
                                / CVACC,
                        );
                        ix = ix.wrapping_add(1);
                    }
                    iy = iy.wrapping_add(1);
                }
            } else {
                iy = 0 as cty::c_int as cty::c_uint;
                while iy < my {
                    py = ((*jd).mcubuf)
                        .offset(iy.wrapping_mul(8 as cty::c_int as cty::c_uint) as isize);
                    if my == 16 as cty::c_int as cty::c_uint {
                        if iy >= 8 as cty::c_int as cty::c_uint {
                            py = py.offset(64 as cty::c_int as isize);
                        }
                    }
                    ix = 0 as cty::c_int as cty::c_uint;
                    while ix < mx {
                        if mx == 16 as cty::c_int as cty::c_uint {
                            if ix == 8 as cty::c_int as cty::c_uint {
                                py = py
                                    .offset((64 as cty::c_int - 8 as cty::c_int) as isize);
                            }
                        }
                        let fresh37 = py;
                        py = py.offset(1);
                        let fresh38 = pix;
                        pix = pix.offset(1);
                        *fresh38 = *fresh37 as uint8_t;
                        ix = ix.wrapping_add(1);
                    }
                    iy = iy.wrapping_add(1);
                }
            }
            if 0 as cty::c_int != 0 && (*jd).scale as cty::c_int != 0 {
                let mut x_0: cty::c_uint = 0;
                let mut y_0: cty::c_uint = 0;
                let mut r: cty::c_uint = 0;
                let mut g: cty::c_uint = 0;
                let mut b: cty::c_uint = 0;
                let mut s: cty::c_uint = 0;
                let mut w: cty::c_uint = 0;
                let mut a: cty::c_uint = 0;
                let mut op: *mut uint8_t = 0 as *mut uint8_t;
                s = ((*jd).scale as cty::c_int * 2 as cty::c_int) as cty::c_uint;
                w = ((1 as cty::c_int) << (*jd).scale as cty::c_int) as cty::c_uint;
                a = mx
                    .wrapping_sub(w)
                    .wrapping_mul(
                        (if 1 as cty::c_int != 2 as cty::c_int {
                            3 as cty::c_int
                        } else {
                            1 as cty::c_int
                        }) as cty::c_uint,
                    );
                op = (*jd).workbuf as *mut uint8_t;
                iy = 0 as cty::c_int as cty::c_uint;
                while iy < my {
                    ix = 0 as cty::c_int as cty::c_uint;
                    while ix < mx {
                        pix = ((*jd).workbuf as *mut uint8_t)
                            .offset(
                                iy
                                    .wrapping_mul(mx)
                                    .wrapping_add(ix)
                                    .wrapping_mul(
                                        (if 1 as cty::c_int != 2 as cty::c_int {
                                            3 as cty::c_int
                                        } else {
                                            1 as cty::c_int
                                        }) as cty::c_uint,
                                    ) as isize,
                            );
                        b = 0 as cty::c_int as cty::c_uint;
                        g = b;
                        r = g;
                        y_0 = 0 as cty::c_int as cty::c_uint;
                        while y_0 < w {
                            x_0 = 0 as cty::c_int as cty::c_uint;
                            while x_0 < w {
                                let fresh39 = pix;
                                pix = pix.offset(1);
                                r = r.wrapping_add(*fresh39 as cty::c_uint);
                                if 1 as cty::c_int != 2 as cty::c_int {
                                    let fresh40 = pix;
                                    pix = pix.offset(1);
                                    g = g.wrapping_add(*fresh40 as cty::c_uint);
                                    let fresh41 = pix;
                                    pix = pix.offset(1);
                                    b = b.wrapping_add(*fresh41 as cty::c_uint);
                                }
                                x_0 = x_0.wrapping_add(1);
                            }
                            pix = pix.offset(a as isize);
                            y_0 = y_0.wrapping_add(1);
                        }
                        let fresh42 = op;
                        op = op.offset(1);
                        *fresh42 = (r >> s) as uint8_t;
                        if 1 as cty::c_int != 2 as cty::c_int {
                            let fresh43 = op;
                            op = op.offset(1);
                            *fresh43 = (g >> s) as uint8_t;
                            let fresh44 = op;
                            op = op.offset(1);
                            *fresh44 = (b >> s) as uint8_t;
                        }
                        ix = ix.wrapping_add(w);
                    }
                    iy = iy.wrapping_add(w);
                }
            }
        } else {
            pix = (*jd).workbuf as *mut uint8_t;
            pc = ((*jd).mcubuf).offset(mx.wrapping_mul(my) as isize);
            cb = *pc.offset(0 as cty::c_int as isize) as cty::c_int - 128 as cty::c_int;
            cr = *pc.offset(64 as cty::c_int as isize) as cty::c_int - 128 as cty::c_int;
            iy = 0 as cty::c_int as cty::c_uint;
            while iy < my {
                py = (*jd).mcubuf;
                if iy == 8 as cty::c_int as cty::c_uint {
                    py = py.offset((64 as cty::c_int * 2 as cty::c_int) as isize);
                }
                ix = 0 as cty::c_int as cty::c_uint;
                while ix < mx {
                    yy = *py as cty::c_int;
                    py = py.offset(64 as cty::c_int as isize);
                    if 1 as cty::c_int != 2 as cty::c_int {
                        let fresh45 = pix;
                        pix = pix.offset(1);
                        *fresh45 = BYTECLIP(
                            yy
                                + (1.402f64 * CVACC as cty::c_double) as cty::c_int * cr
                                / CVACC,
                        );
                        let fresh46 = pix;
                        pix = pix.offset(1);
                        *fresh46 = BYTECLIP(
                            yy
                                - ((0.344f64 * CVACC as cty::c_double) as cty::c_int * cb
                                + (0.714f64 * CVACC as cty::c_double) as cty::c_int * cr)
                                / CVACC,
                        );
                        let fresh47 = pix;
                        pix = pix.offset(1);
                        *fresh47 = BYTECLIP(
                            yy
                                + (1.772f64 * CVACC as cty::c_double) as cty::c_int * cb
                                / CVACC,
                        );
                    } else {
                        let fresh48 = pix;
                        pix = pix.offset(1);
                        *fresh48 = yy as uint8_t;
                    }
                    ix = ix.wrapping_add(8 as cty::c_int as cty::c_uint);
                }
                iy = iy.wrapping_add(8 as cty::c_int as cty::c_uint);
            }
        }
        mx >>= (*jd).scale as cty::c_int;
        if rx < mx {
            let mut s_0: *mut uint8_t = 0 as *mut uint8_t;
            let mut d: *mut uint8_t = 0 as *mut uint8_t;
            let mut x_1: cty::c_uint = 0;
            let mut y_1: cty::c_uint = 0;
            d = (*jd).workbuf as *mut uint8_t;
            s_0 = d;
            y_1 = 0 as cty::c_int as cty::c_uint;
            while y_1 < ry {
                x_1 = 0 as cty::c_int as cty::c_uint;
                while x_1 < rx {
                    let fresh49 = s_0;
                    s_0 = s_0.offset(1);
                    let fresh50 = d;
                    d = d.offset(1);
                    *fresh50 = *fresh49;
                    if 1 as cty::c_int != 2 as cty::c_int {
                        let fresh51 = s_0;
                        s_0 = s_0.offset(1);
                        let fresh52 = d;
                        d = d.offset(1);
                        *fresh52 = *fresh51;
                        let fresh53 = s_0;
                        s_0 = s_0.offset(1);
                        let fresh54 = d;
                        d = d.offset(1);
                        *fresh54 = *fresh53;
                    }
                    x_1 = x_1.wrapping_add(1);
                }
                s_0 = s_0
                    .offset(
                        mx
                            .wrapping_sub(rx)
                            .wrapping_mul(
                                (if 1 as cty::c_int != 2 as cty::c_int {
                                    3 as cty::c_int
                                } else {
                                    1 as cty::c_int
                                }) as cty::c_uint,
                            ) as isize,
                    );
                y_1 = y_1.wrapping_add(1);
            }
        }
        if 1 as cty::c_int == 1 as cty::c_int {
            let mut s_1: *mut uint8_t = (*jd).workbuf as *mut uint8_t;
            let mut w_0: uint16_t = 0;
            let mut d_0: *mut uint16_t = s_1 as *mut uint16_t;
            let mut n: cty::c_uint = rx.wrapping_mul(ry);
            loop {
                let fresh55 = s_1;
                s_1 = s_1.offset(1);
                w_0 = ((*fresh55 as cty::c_int & 0xf8 as cty::c_int) << 8 as cty::c_int)
                    as uint16_t;
                let fresh56 = s_1;
                s_1 = s_1.offset(1);
                w_0 = (w_0 as cty::c_int
                    | (*fresh56 as cty::c_int & 0xfc as cty::c_int) << 3 as cty::c_int)
                    as uint16_t;
                let fresh57 = s_1;
                s_1 = s_1.offset(1);
                w_0 = (w_0 as cty::c_int | *fresh57 as cty::c_int >> 3 as cty::c_int)
                    as uint16_t;
                let fresh58 = d_0;
                d_0 = d_0.offset(1);
                *fresh58 = w_0;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
        }
        return (if outfunc.expect("non-null function pointer")(jd, (*jd).workbuf, &mut rect)
            != 0
        {
            JDR_OK as cty::c_int
        } else {
            JDR_INTR as cty::c_int
        }) as JRESULT;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jd_prepare(
    mut jd: *mut JDEC,
    mut infunc: Option::<
        unsafe extern "C" fn(*mut JDEC, *mut uint8_t, size_t) -> size_t,
    >,
    mut pool: *mut cty::c_void,
    mut sz_pool: size_t,
    mut dev: *mut cty::c_void,
) -> JRESULT {
    unsafe {
        let mut seg: *mut uint8_t = 0 as *mut uint8_t;
        let mut b: uint8_t = 0;
        let mut marker: uint16_t = 0;
        let mut n: cty::c_uint = 0;
        let mut i: cty::c_uint = 0;
        let mut ofs: cty::c_uint = 0;
        let mut len: size_t = 0;
        let mut rc: JRESULT = JDR_OK;
        memset(
            jd as *mut cty::c_void,
            0 as cty::c_int,
            ::core::mem::size_of::<JDEC>() as cty::c_ulong,
        );
        let ref mut fresh59 = (*jd).pool;
        *fresh59 = pool;
        (*jd).sz_pool = sz_pool;
        let ref mut fresh60 = (*jd).infunc;
        *fresh60 = infunc;
        let ref mut fresh61 = (*jd).device;
        *fresh61 = dev;
        let ref mut fresh62 = (*jd).dcv[0 as cty::c_int as usize];
        *fresh62 = 0 as cty::c_int as int16_t;
        let ref mut fresh63 = (*jd).dcv[1 as cty::c_int as usize];
        *fresh63 = *fresh62;
        (*jd).dcv[2 as cty::c_int as usize] = *fresh63;
        (*jd).rsc = 0 as cty::c_int as uint16_t;
        (*jd).rst = 0 as cty::c_int as uint16_t;
        seg = alloc_pool(jd, 512 as cty::c_int as size_t) as *mut uint8_t;
        let ref mut fresh64 = (*jd).inbuf;
        *fresh64 = seg;
        if seg.is_null() {
            return JDR_MEM1;
        }
        marker = 0 as cty::c_int as uint16_t;
        ofs = marker as cty::c_uint;
        loop {
            if ((*jd).infunc)
                .expect("non-null function pointer")(jd, seg, 1 as cty::c_int as size_t)
                != 1 as cty::c_int as cty::c_ulong
            {
                return JDR_INP;
            }
            ofs = ofs.wrapping_add(1);
            marker = ((marker as cty::c_int) << 8 as cty::c_int
                | *seg.offset(0 as cty::c_int as isize) as cty::c_int) as uint16_t;
            if !(marker as cty::c_int != 0xffd8 as cty::c_int) {
                break;
            }
        }
        loop {
            if ((*jd).infunc)
                .expect("non-null function pointer")(jd, seg, 4 as cty::c_int as size_t)
                != 4 as cty::c_int as cty::c_ulong
            {
                return JDR_INP;
            }
            marker = ((*seg as uint16_t as cty::c_int) << 8 as cty::c_int
                | *seg.offset(1 as cty::c_int as isize) as uint16_t as cty::c_int)
                as uint16_t;
            len = ((*seg.offset(2 as cty::c_int as isize) as uint16_t as cty::c_int)
                << 8 as cty::c_int
                | *seg.offset(2 as cty::c_int as isize).offset(1 as cty::c_int as isize)
                as uint16_t as cty::c_int) as uint16_t as size_t;
            if len <= 2 as cty::c_int as cty::c_ulong
                || marker as cty::c_int >> 8 as cty::c_int != 0xff as cty::c_int
            {
                return JDR_FMT1;
            }
            len = (len as cty::c_ulong).wrapping_sub(2 as cty::c_int as cty::c_ulong)
                as size_t as size_t;
            ofs = (ofs as cty::c_ulong)
                .wrapping_add((4 as cty::c_int as cty::c_ulong).wrapping_add(len))
                as cty::c_uint as cty::c_uint;
            's_526: {
                let mut current_block_111: u64;
                match marker as cty::c_int & 0xff as cty::c_int {
                    192 => {
                        if len > 512 as cty::c_int as cty::c_ulong {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len)
                            != len
                        {
                            return JDR_INP;
                        }
                        (*jd)
                            .width = ((*(&mut *seg.offset(3 as cty::c_int as isize)
                            as *mut uint8_t) as uint16_t as cty::c_int) << 8 as cty::c_int
                            | *(&mut *seg.offset(3 as cty::c_int as isize) as *mut uint8_t)
                            .offset(1 as cty::c_int as isize) as uint16_t
                            as cty::c_int) as uint16_t;
                        (*jd)
                            .height = ((*(&mut *seg.offset(1 as cty::c_int as isize)
                            as *mut uint8_t) as uint16_t as cty::c_int) << 8 as cty::c_int
                            | *(&mut *seg.offset(1 as cty::c_int as isize) as *mut uint8_t)
                            .offset(1 as cty::c_int as isize) as uint16_t
                            as cty::c_int) as uint16_t;
                        (*jd).ncomp = *seg.offset(5 as cty::c_int as isize);
                        if (*jd).ncomp as cty::c_int != 3 as cty::c_int
                            && (*jd).ncomp as cty::c_int != 1 as cty::c_int
                        {
                            return JDR_FMT3;
                        }
                        i = 0 as cty::c_int as cty::c_uint;
                        while i < (*jd).ncomp as cty::c_uint {
                            b = *seg
                                .offset(
                                    (7 as cty::c_int as cty::c_uint)
                                        .wrapping_add(
                                            (3 as cty::c_int as cty::c_uint).wrapping_mul(i),
                                        ) as isize,
                                );
                            if i == 0 as cty::c_int as cty::c_uint {
                                if b as cty::c_int != 0x11 as cty::c_int
                                    && b as cty::c_int != 0x22 as cty::c_int
                                    && b as cty::c_int != 0x21 as cty::c_int
                                {
                                    return JDR_FMT3;
                                }
                                (*jd)
                                    .msx = (b as cty::c_int >> 4 as cty::c_int) as uint8_t;
                                (*jd)
                                    .msy = (b as cty::c_int & 15 as cty::c_int) as uint8_t;
                            } else if b as cty::c_int != 0x11 as cty::c_int {
                                return JDR_FMT3
                            }
                            (*jd)
                                .qtid[i
                                as usize] = *seg
                                .offset(
                                    (8 as cty::c_int as cty::c_uint)
                                        .wrapping_add(
                                            (3 as cty::c_int as cty::c_uint).wrapping_mul(i),
                                        ) as isize,
                                );
                            if (*jd).qtid[i as usize] as cty::c_int > 3 as cty::c_int {
                                return JDR_FMT3;
                            }
                            i = i.wrapping_add(1);
                        }
                        current_block_111 = 5265702136860997526;
                    }
                    221 => {
                        if len > 512 as cty::c_int as cty::c_ulong {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len)
                            != len
                        {
                            return JDR_INP;
                        }
                        (*jd)
                            .nrst = ((*seg as uint16_t as cty::c_int) << 8 as cty::c_int
                            | *seg.offset(1 as cty::c_int as isize) as uint16_t
                            as cty::c_int) as uint16_t;
                        current_block_111 = 5265702136860997526;
                    }
                    196 => {
                        if len > 512 as cty::c_int as cty::c_ulong {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len)
                            != len
                        {
                            return JDR_INP;
                        }
                        rc = create_huffman_tbl(jd, seg, len);
                        if rc as u64 != 0 {
                            return rc;
                        }
                        current_block_111 = 5265702136860997526;
                    }
                    219 => {
                        if len > 512 as cty::c_int as cty::c_ulong {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len)
                            != len
                        {
                            return JDR_INP;
                        }
                        rc = create_qt_tbl(jd, seg, len);
                        if rc as u64 != 0 {
                            return rc;
                        }
                        current_block_111 = 5265702136860997526;
                    }
                    218 => {
                        if len > 512 as cty::c_int as cty::c_ulong {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len)
                            != len
                        {
                            return JDR_INP;
                        }
                        if (*jd).width == 0 || (*jd).height == 0 {
                            return JDR_FMT1;
                        }
                        if *seg.offset(0 as cty::c_int as isize) as cty::c_int
                            != (*jd).ncomp as cty::c_int
                        {
                            return JDR_FMT3;
                        }
                        i = 0 as cty::c_int as cty::c_uint;
                        while i < (*jd).ncomp as cty::c_uint {
                            b = *seg
                                .offset(
                                    (2 as cty::c_int as cty::c_uint)
                                        .wrapping_add(
                                            (2 as cty::c_int as cty::c_uint).wrapping_mul(i),
                                        ) as isize,
                                );
                            if b as cty::c_int != 0 as cty::c_int
                                && b as cty::c_int != 0x11 as cty::c_int
                            {
                                return JDR_FMT3;
                            }
                            n = (if i != 0 { 1 as cty::c_int } else { 0 as cty::c_int })
                                as cty::c_uint;
                            if ((*jd).huffbits[n as usize][0 as cty::c_int as usize])
                                .is_null()
                                || ((*jd).huffbits[n as usize][1 as cty::c_int as usize])
                                .is_null()
                            {
                                return JDR_FMT1;
                            }
                            if ((*jd).qttbl[(*jd).qtid[i as usize] as usize]).is_null() {
                                return JDR_FMT1;
                            }
                            i = i.wrapping_add(1);
                        }
                        n = ((*jd).msy as cty::c_int * (*jd).msx as cty::c_int)
                            as cty::c_uint;
                        if n == 0 {
                            return JDR_FMT1;
                        }
                        len = n
                            .wrapping_mul(64 as cty::c_int as cty::c_uint)
                            .wrapping_mul(2 as cty::c_int as cty::c_uint)
                            .wrapping_add(64 as cty::c_int as cty::c_uint) as size_t;
                        if len < 256 as cty::c_int as cty::c_ulong {
                            len = 256 as cty::c_int as size_t;
                        }
                        let ref mut fresh65 = (*jd).workbuf;
                        *fresh65 = alloc_pool(jd, len);
                        if ((*jd).workbuf).is_null() {
                            return JDR_MEM1;
                        }
                        let ref mut fresh66 = (*jd).mcubuf;
                        *fresh66 = alloc_pool(
                            jd,
                            (n
                                .wrapping_add(2 as cty::c_int as cty::c_uint)
                                .wrapping_mul(64 as cty::c_int as cty::c_uint)
                                as cty::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<jd_yuv_t>() as cty::c_ulong,
                                ),
                        ) as *mut jd_yuv_t;
                        if ((*jd).mcubuf).is_null() {
                            return JDR_MEM1;
                        }
                        ofs = ofs.wrapping_rem(512 as cty::c_int as cty::c_uint);
                        if ofs != 0 {
                            (*jd)
                                .dctr = ((*jd).infunc)
                                .expect(
                                    "non-null function pointer",
                                )(
                                jd,
                                seg.offset(ofs as isize),
                                (512 as cty::c_int as cty::c_uint).wrapping_sub(ofs)
                                    as size_t,
                            );
                        }
                        let ref mut fresh67 = (*jd).dptr;
                        *fresh67 = seg
                            .offset(ofs as isize)
                            .offset(
                                -((if 2 as cty::c_int != 0 {
                                    0 as cty::c_int
                                } else {
                                    1 as cty::c_int
                                }) as isize),
                            );
                        return JDR_OK;
                    }
                    193 => {
                        current_block_111 = 12749676338018479376;
                    }
                    194 => {
                        current_block_111 = 12749676338018479376;
                    }
                    195 => {
                        current_block_111 = 7120504289787790845;
                    }
                    197 => {
                        current_block_111 = 11626555135028741001;
                    }
                    198 => {
                        current_block_111 = 12215488699659360936;
                    }
                    199 => {
                        current_block_111 = 5192055691381141330;
                    }
                    201 => {
                        current_block_111 = 1443089516996880600;
                    }
                    202 => {
                        current_block_111 = 15064317190960798138;
                    }
                    203 => {
                        current_block_111 = 14109165499131509865;
                    }
                    205 => {
                        current_block_111 = 9711326356574826945;
                    }
                    207 => {
                        current_block_111 = 13359995684220628626;
                    }
                    206 | 217 => {
                        current_block_111 = 13359995684220628626;
                    }
                    _ => {
                        if ((*jd).infunc)
                            .expect("non-null function pointer")(jd, 0 as *mut uint8_t, len)
                            != len
                        {
                            return JDR_INP;
                        }
                        current_block_111 = 5265702136860997526;
                    }
                }
                match current_block_111 {
                    12749676338018479376 => {
                        current_block_111 = 7120504289787790845;
                    }
                    5265702136860997526 => {
                        break 's_526;
                    }
                    _ => {}
                }
                match current_block_111 {
                    7120504289787790845 => {
                        current_block_111 = 11626555135028741001;
                    }
                    _ => {}
                }
                match current_block_111 {
                    11626555135028741001 => {
                        current_block_111 = 12215488699659360936;
                    }
                    _ => {}
                }
                match current_block_111 {
                    12215488699659360936 => {
                        current_block_111 = 5192055691381141330;
                    }
                    _ => {}
                }
                match current_block_111 {
                    5192055691381141330 => {
                        current_block_111 = 1443089516996880600;
                    }
                    _ => {}
                }
                match current_block_111 {
                    1443089516996880600 => {
                        current_block_111 = 15064317190960798138;
                    }
                    _ => {}
                }
                match current_block_111 {
                    15064317190960798138 => {
                        current_block_111 = 14109165499131509865;
                    }
                    _ => {}
                }
                match current_block_111 {
                    14109165499131509865 => {
                        current_block_111 = 9711326356574826945;
                    }
                    _ => {}
                }
                match current_block_111 {
                    9711326356574826945 => {}
                    _ => {}
                }
                return JDR_FMT3;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn jd_decomp(
    mut jd: *mut JDEC,
    mut outfunc: Option::<
        unsafe extern "C" fn(*mut JDEC, *mut cty::c_void, *mut JRECT) -> cty::c_int,
    >,
    mut scale: uint8_t,
) -> JRESULT {
    unsafe {
        let mut x: cty::c_uint = 0;
        let mut y: cty::c_uint = 0;
        let mut mx: cty::c_uint = 0;
        let mut my: cty::c_uint = 0;
        let mut rc: JRESULT = JDR_OK;
        if scale as cty::c_int
            > (if 0 as cty::c_int != 0 { 3 as cty::c_int } else { 0 as cty::c_int })
        {
            return JDR_PAR;
        }
        (*jd).scale = scale;
        mx = ((*jd).msx as cty::c_int * 8 as cty::c_int) as cty::c_uint;
        my = ((*jd).msy as cty::c_int * 8 as cty::c_int) as cty::c_uint;
        rc = JDR_OK;
        y = 0 as cty::c_int as cty::c_uint;
        while y < (*jd).height as cty::c_uint {
            x = 0 as cty::c_int as cty::c_uint;
            while x < (*jd).width as cty::c_uint {
                if (*jd).nrst as cty::c_int != 0
                    && {
                    let ref mut fresh68 = (*jd).rst;
                    let fresh69 = *fresh68;
                    *fresh68 = (*fresh68).wrapping_add(1);
                    fresh69 as cty::c_int == (*jd).nrst as cty::c_int
                }
                {
                    let ref mut fresh70 = (*jd).rsc;
                    let fresh71 = *fresh70;
                    *fresh70 = (*fresh70).wrapping_add(1);
                    rc = restart(jd, fresh71);
                    if rc as cty::c_uint != JDR_OK as cty::c_int as cty::c_uint {
                        return rc;
                    }
                    (*jd).rst = 1 as cty::c_int as uint16_t;
                }
                rc = mcu_load(jd);
                if rc as cty::c_uint != JDR_OK as cty::c_int as cty::c_uint {
                    return rc;
                }
                rc = mcu_output(jd, outfunc, x, y);
                if rc as cty::c_uint != JDR_OK as cty::c_int as cty::c_uint {
                    return rc;
                }
                x = x.wrapping_add(mx);
            }
            y = y.wrapping_add(my);
        }
        return rc;
    }
}
