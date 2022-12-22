#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use core::slice;

extern "C" {
    fn memset(_: *mut cty::c_void, _: i32, _: cty::c_ulong) -> *mut cty::c_void;
}

const HUFF_BIT: u32 = 10;

pub type JRESULT = u32;
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
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JDEC {
    pub dctr: usize,
    pub dptr: *mut u8,
    pub inbuf: *mut u8,
    pub dbit: u8,
    pub scale: u8,
    pub msx: u8,
    pub msy: u8,
    pub qtid: [u8; 3],
    pub ncomp: u8,
    pub dcv: [i16; 3],
    pub nrst: u16,
    pub rst: u16,
    pub rsc: u16,
    pub width: u16,
    pub height: u16,
    pub huffbits: [[*mut u8; 2]; 2],
    pub huffcode: [[*mut u16; 2]; 2],
    pub huffdata: [[*mut u8; 2]; 2],
    pub qttbl: [*mut i32; 4],
    pub wreg: u32,
    pub marker: u8,
    pub longofs: [[u8; 2]; 2],
    pub hufflut_ac: [*mut u16; 2],
    pub hufflut_dc: [*mut u8; 2],
    pub workbuf: *mut cty::c_void,
    pub mcubuf: *mut i16,
    pub pool: *mut cty::c_void,
    pub sz_pool: usize,
    pub infunc: Option<unsafe fn(*mut JDEC, *mut u8, usize) -> usize>,
    pub device: *mut cty::c_void,
}
static mut Zig: [u8; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];
static mut Ipsf: [u16; 64] = [
    (1.00000f64 * 8192_f64) as u16,
    (1.38704f64 * 8192_f64) as u16,
    (1.30656f64 * 8192_f64) as u16,
    (1.17588f64 * 8192_f64) as u16,
    (1.00000f64 * 8192_f64) as u16,
    (0.78570f64 * 8192_f64) as u16,
    (0.54120f64 * 8192_f64) as u16,
    (0.27590f64 * 8192_f64) as u16,
    (1.38704f64 * 8192_f64) as u16,
    (1.92388f64 * 8192_f64) as u16,
    (1.81226f64 * 8192_f64) as u16,
    (1.63099f64 * 8192_f64) as u16,
    (1.38704f64 * 8192_f64) as u16,
    (1.08979f64 * 8192_f64) as u16,
    (0.75066f64 * 8192_f64) as u16,
    (0.38268f64 * 8192_f64) as u16,
    (1.30656f64 * 8192_f64) as u16,
    (1.81226f64 * 8192_f64) as u16,
    (1.70711f64 * 8192_f64) as u16,
    (1.53636f64 * 8192_f64) as u16,
    (1.30656f64 * 8192_f64) as u16,
    (1.02656f64 * 8192_f64) as u16,
    (0.70711f64 * 8192_f64) as u16,
    (0.36048f64 * 8192_f64) as u16,
    (1.17588f64 * 8192_f64) as u16,
    (1.63099f64 * 8192_f64) as u16,
    (1.53636f64 * 8192_f64) as u16,
    (1.38268f64 * 8192_f64) as u16,
    (1.17588f64 * 8192_f64) as u16,
    (0.92388f64 * 8192_f64) as u16,
    (0.63638f64 * 8192_f64) as u16,
    (0.32442f64 * 8192_f64) as u16,
    (1.00000f64 * 8192_f64) as u16,
    (1.38704f64 * 8192_f64) as u16,
    (1.30656f64 * 8192_f64) as u16,
    (1.17588f64 * 8192_f64) as u16,
    (1.00000f64 * 8192_f64) as u16,
    (0.78570f64 * 8192_f64) as u16,
    (0.54120f64 * 8192_f64) as u16,
    (0.27590f64 * 8192_f64) as u16,
    (0.78570f64 * 8192_f64) as u16,
    (1.08979f64 * 8192_f64) as u16,
    (1.02656f64 * 8192_f64) as u16,
    (0.92388f64 * 8192_f64) as u16,
    (0.78570f64 * 8192_f64) as u16,
    (0.61732f64 * 8192_f64) as u16,
    (0.42522f64 * 8192_f64) as u16,
    (0.21677f64 * 8192_f64) as u16,
    (0.54120f64 * 8192_f64) as u16,
    (0.75066f64 * 8192_f64) as u16,
    (0.70711f64 * 8192_f64) as u16,
    (0.63638f64 * 8192_f64) as u16,
    (0.54120f64 * 8192_f64) as u16,
    (0.42522f64 * 8192_f64) as u16,
    (0.29290f64 * 8192_f64) as u16,
    (0.14932f64 * 8192_f64) as u16,
    (0.27590f64 * 8192_f64) as u16,
    (0.38268f64 * 8192_f64) as u16,
    (0.36048f64 * 8192_f64) as u16,
    (0.32442f64 * 8192_f64) as u16,
    (0.27590f64 * 8192_f64) as u16,
    (0.21678f64 * 8192_f64) as u16,
    (0.14932f64 * 8192_f64) as u16,
    (0.07612f64 * 8192_f64) as u16,
];

fn BYTECLIP(mut val: i32) -> u8 {
    if val < 0 {
        return 0;
    }
    if val > 255 {
        return 255;
    }
    return val as u8;
}

unsafe fn alloc_pool(mut jd: *mut JDEC, mut ndata: usize) -> *mut cty::c_void {
    unsafe {
        let mut rp: *mut cty::c_char = 0 as *mut cty::c_char;
        ndata = (ndata + 3) & !3;
        if (*jd).sz_pool >= ndata {
            let ref mut fresh0 = (*jd).sz_pool;
            *fresh0 = *fresh0 - ndata;
            rp = (*jd).pool as *mut cty::c_char;
            let ref mut fresh1 = (*jd).pool;
            *fresh1 = rp.offset(ndata as isize) as *mut cty::c_void;
        }
        return rp as *mut cty::c_void;
    }
}

unsafe fn alloc_pool_u8(mut jd: *mut JDEC, mut ndata: usize) -> Result<&'static mut [u8], ()> {
    unsafe {
        let mut rp: *mut cty::c_char = 0 as *mut cty::c_char;
        let ndata_aligned = (ndata + 3) & !3;
        if (*jd).sz_pool >= ndata_aligned {
            let ref mut fresh0 = (*jd).sz_pool;
            *fresh0 = *fresh0 - ndata_aligned;
            rp = (*jd).pool as *mut cty::c_char;
            let ref mut fresh1 = (*jd).pool;
            *fresh1 = rp.offset(ndata_aligned as isize) as *mut cty::c_void;
            return Ok(slice::from_raw_parts_mut(rp as *mut u8, ndata));
        }
        Err(())
    }
}

unsafe fn create_qt_tbl(mut jd: *mut JDEC, mut data: *const u8, mut ndata: usize) -> JRESULT {
    unsafe {
        let mut i: u32 = 0;
        let mut zi: u32 = 0;
        let mut d: u8 = 0;
        let mut pb: *mut i32 = 0 as *mut i32;
        while ndata != 0 {
            if ndata < 65 {
                return JDR_FMT1;
            }
            ndata -= 65;

            let fresh2 = data;
            data = data.offset(1);
            d = *fresh2;
            if d as i32 & 0xf0 != 0 {
                return JDR_FMT1;
            }
            i = (d as i32 & 3) as u32;
            pb = alloc_pool(jd, 64 * ::core::mem::size_of::<i32>()) as *mut i32;
            if pb.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh3 = (*jd).qttbl[i as usize];
            *fresh3 = pb;
            i = 0;
            while i < 64 {
                zi = Zig[i as usize] as u32;
                let fresh4 = data;
                data = data.offset(1);
                *pb.offset(zi as isize) =
                    (*fresh4 as u32).wrapping_mul(Ipsf[zi as usize] as u32) as i32;
                i = i.wrapping_add(1);
            }
        }
        return JDR_OK;
    }
}
unsafe fn create_huffman_tbl(mut jd: *mut JDEC, mut data: *const u8, mut ndata: usize) -> JRESULT {
    unsafe {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut b: u32 = 0;
        let mut cls: u32 = 0;
        let mut num: u32 = 0;
        let mut np: usize = 0;
        let mut d: u8 = 0;
        let mut pb: *mut u8 = 0 as *mut u8;
        let mut pd: *mut u8 = 0 as *mut u8;
        let mut hc: u16 = 0;
        let mut ph: *mut u16 = 0 as *mut u16;
        while ndata != 0 {
            if ndata < 17 {
                return JDR_FMT1;
            }
            ndata -= 17;
            let fresh5 = data;
            data = data.offset(1);
            d = *fresh5;
            if d as i32 & 0xee as i32 != 0 {
                return JDR_FMT1;
            }
            cls = (d as i32 >> 4) as u32;
            num = (d as i32 & 0xf) as u32;
            pb = alloc_pool(jd, 16) as *mut u8;
            if pb.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh6 = (*jd).huffbits[num as usize][cls as usize];
            *fresh6 = pb;
            i = 0;
            np = i as usize;
            while i < 16 {
                let fresh7 = data;
                data = data.offset(1);
                let ref mut fresh8 = *pb.offset(i as isize);
                *fresh8 = *fresh7;
                np = (np as cty::c_ulong).wrapping_add(*fresh8 as cty::c_ulong) as usize;
                i = i.wrapping_add(1);
            }
            ph = alloc_pool(jd, np * core::mem::size_of::<u16>()) as *mut u16;
            if ph.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh9 = (*jd).huffcode[num as usize][cls as usize];
            *fresh9 = ph;
            hc = 0;
            i = 0;
            j = i;
            while i < 16 {
                b = *pb.offset(i as isize) as u32;
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
                hc = ((hc as i32) << 1) as u16;
                i = i.wrapping_add(1);
            }
            if ndata < np {
                return JDR_FMT1;
            }
            ndata -= np;
            pd = alloc_pool(jd, np) as *mut u8;
            if pd.is_null() {
                return JDR_MEM1;
            }
            let ref mut fresh13 = (*jd).huffdata[num as usize][cls as usize];
            *fresh13 = pd;
            i = 0;
            while i < np as u32 {
                let fresh14 = data;
                data = data.offset(1);
                d = *fresh14;
                if cls == 0 && d as i32 > 11 as i32 {
                    return JDR_FMT1;
                }
                *pd.offset(i as isize) = d;
                i = i.wrapping_add(1);
            }
            let mut span: u32 = 0;
            let mut td: u32 = 0;
            let mut ti: u32 = 0;
            let mut tbl_ac: *mut u16 = 0 as *mut u16;
            let mut tbl_dc: *mut u8 = 0 as *mut u8;
            if cls != 0 {
                tbl_ac = alloc_pool(jd, (1 << 10) * ::core::mem::size_of::<u16>()) as *mut u16;
                if tbl_ac.is_null() {
                    return JDR_MEM1;
                }
                let ref mut fresh15 = (*jd).hufflut_ac[num as usize];
                *fresh15 = tbl_ac;
                memset(
                    tbl_ac as *mut cty::c_void,
                    0xff,
                    ((1 << HUFF_BIT) as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<u16>() as cty::c_ulong),
                );
            } else {
                tbl_dc = alloc_pool(jd, (1 << HUFF_BIT) * ::core::mem::size_of::<u8>()) as *mut u8;
                if tbl_dc.is_null() {
                    return JDR_MEM1;
                }
                let ref mut fresh16 = (*jd).hufflut_dc[num as usize];
                *fresh16 = tbl_dc;
                memset(
                    tbl_dc as *mut cty::c_void,
                    0xff,
                    ((1 << HUFF_BIT) as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<u8>() as cty::c_ulong),
                );
            }
            b = 0;
            i = b;
            while b < HUFF_BIT {
                j = *pb.offset(b as isize) as u32;
                while j != 0 {
                    ti = ((*ph.offset(i as isize) as i32)
                        << ((HUFF_BIT - 1) as u32).wrapping_sub(b)
                        & (1 << HUFF_BIT) - 1 as i32) as u32;
                    if cls != 0 {
                        let fresh17 = i;
                        i = i.wrapping_add(1);
                        td = *pd.offset(fresh17 as isize) as u32 | b.wrapping_add(1) << 8;
                        span = (1 << ((HUFF_BIT - 1) as u32).wrapping_sub(b)) as u32;
                        while span != 0 {
                            span = span.wrapping_sub(1);
                            let fresh18 = ti;
                            ti = ti.wrapping_add(1);
                            *tbl_ac.offset(fresh18 as isize) = td as u16;
                        }
                    } else {
                        let fresh19 = i;
                        i = i.wrapping_add(1);
                        td = *pd.offset(fresh19 as isize) as u32 | b.wrapping_add(1) << 4 as i32;
                        span = ((1 as i32) << ((HUFF_BIT - 1) as u32).wrapping_sub(b)) as u32;
                        while span != 0 {
                            span = span.wrapping_sub(1);
                            let fresh20 = ti;
                            ti = ti.wrapping_add(1);
                            *tbl_dc.offset(fresh20 as isize) = td as u8;
                        }
                    }
                    j = j.wrapping_sub(1);
                }
                b = b.wrapping_add(1);
            }
            (*jd).longofs[num as usize][cls as usize] = i as u8;
        }
        return JDR_OK;
    }
}
unsafe fn huffext(mut jd: *mut JDEC, mut id: u32, mut cls: u32) -> i32 {
    unsafe {
        let mut dc: usize = (*jd).dctr;
        let mut dp: *mut u8 = (*jd).dptr;
        let mut d: u32 = 0;
        let mut flg: u32 = 0;
        let mut hb: *const u8 = 0 as *const u8;
        let mut hd: *const u8 = 0 as *const u8;
        let mut hc: *const u16 = 0 as *const u16;
        let mut nc: u32 = 0;
        let mut bl: u32 = 0;
        let mut wbit: u32 = ((*jd).dbit as i32 % 32) as u32;
        let mut w: u32 = ((*jd).wreg as cty::c_ulong
            & ((1 as cty::c_ulong) << wbit).wrapping_sub(1 as i32 as cty::c_ulong))
            as u32;
        while wbit < 16 {
            if (*jd).marker != 0 {
                d = 0xff;
            } else {
                if dc == 0 {
                    dp = (*jd).inbuf;
                    dc = ((*jd).infunc).expect("non-null function pointer")(jd, dp, 512);
                    if dc == 0 {
                        return 0 as i32 - JDR_INP as i32;
                    }
                }
                let fresh21 = dp;
                dp = dp.offset(1);
                d = *fresh21 as u32;
                dc = dc.wrapping_sub(1);
                if flg != 0 {
                    flg = 0;
                    if d != 0 {
                        (*jd).marker = d as u8;
                    }
                    d = 0xff;
                } else if d == 0xff {
                    flg = 1;
                    continue;
                }
            }
            w = w << 8 as i32 | d;
            wbit = wbit.wrapping_add(8);
        }
        (*jd).dctr = dc;
        let ref mut fresh22 = (*jd).dptr;
        *fresh22 = dp;
        (*jd).wreg = w;
        d = w >> wbit.wrapping_sub(HUFF_BIT);
        if cls != 0 {
            d = *((*jd).hufflut_ac[id as usize]).offset(d as isize) as u32;
            if d != 0xffff {
                (*jd).dbit = wbit.wrapping_sub(d >> 8) as u8;
                return (d & 0xff) as i32;
            }
        } else {
            d = *((*jd).hufflut_dc[id as usize]).offset(d as isize) as u32;
            if d != 0xff {
                (*jd).dbit = wbit.wrapping_sub(d >> 4) as u8;
                return (d & 0xf) as i32;
            }
        }
        hb = ((*jd).huffbits[id as usize][cls as usize]).offset(HUFF_BIT as isize);
        hc = ((*jd).huffcode[id as usize][cls as usize])
            .offset((*jd).longofs[id as usize][cls as usize] as isize);
        hd = ((*jd).huffdata[id as usize][cls as usize])
            .offset((*jd).longofs[id as usize][cls as usize] as isize);
        bl = (HUFF_BIT + 1) as u32;
        while bl <= 16 {
            let fresh23 = hb;
            hb = hb.offset(1);
            nc = *fresh23 as u32;
            if nc != 0 {
                d = w >> wbit.wrapping_sub(bl);
                loop {
                    let fresh24 = hc;
                    hc = hc.offset(1);
                    if d == *fresh24 as u32 {
                        (*jd).dbit = wbit.wrapping_sub(bl) as u8;
                        return *hd as i32;
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
        return 0 as i32 - JDR_FMT1 as i32;
    }
}
unsafe fn bitext(mut jd: *mut JDEC, mut nbit: u32) -> i32 {
    unsafe {
        let mut dc: usize = (*jd).dctr;
        let mut dp: *mut u8 = (*jd).dptr;
        let mut d: u32 = 0;
        let mut flg: u32 = 0;
        let mut wbit: u32 = ((*jd).dbit as i32 % 32 as i32) as u32;
        let mut w: u32 = ((*jd).wreg as cty::c_ulong
            & ((1 as cty::c_ulong) << wbit).wrapping_sub(1 as i32 as cty::c_ulong))
            as u32;
        while wbit < nbit {
            if (*jd).marker != 0 {
                d = 0xff;
            } else {
                if dc == 0 {
                    dp = (*jd).inbuf;
                    dc = ((*jd).infunc).expect("non-null function pointer")(jd, dp, 512);
                    if dc == 0 {
                        return 0 as i32 - JDR_INP as i32;
                    }
                }
                let fresh25 = dp;
                dp = dp.offset(1);
                d = *fresh25 as u32;
                dc = dc.wrapping_sub(1);
                if flg != 0 {
                    flg = 0;
                    if d != 0 {
                        (*jd).marker = d as u8;
                    }
                    d = 0xff;
                } else if d == 0xff {
                    flg = 1;
                    continue;
                }
            }
            w = w << 8 as i32 | d;
            wbit = wbit.wrapping_add(8);
        }
        (*jd).wreg = w;
        (*jd).dbit = wbit.wrapping_sub(nbit) as u8;
        (*jd).dctr = dc;
        let ref mut fresh26 = (*jd).dptr;
        *fresh26 = dp;
        return (w >> wbit.wrapping_sub(nbit).wrapping_rem(32)) as i32;
    }
}
unsafe fn restart(mut jd: *mut JDEC, mut rstn: u16) -> JRESULT {
    unsafe {
        let mut i: u32 = 0;
        let mut dp: *mut u8 = (*jd).dptr;
        let mut dc: usize = (*jd).dctr;
        let mut marker: u16 = 0;
        if (*jd).marker != 0 {
            marker = (0xff00 as i32 | (*jd).marker as i32) as u16;
            (*jd).marker = 0;
        } else {
            marker = 0;
            i = 0;
            while i < 2 {
                if dc == 0 {
                    dp = (*jd).inbuf;
                    dc = ((*jd).infunc).expect("non-null function pointer")(jd, dp, 512);
                    if dc == 0 {
                        return JDR_INP;
                    }
                }
                let fresh27 = dp;
                dp = dp.offset(1);
                marker = ((marker as i32) << 8 as i32 | *fresh27 as i32) as u16;
                dc = dc.wrapping_sub(1);
                i = i.wrapping_add(1);
            }
            let ref mut fresh28 = (*jd).dptr;
            *fresh28 = dp;
            (*jd).dctr = dc;
        }
        if marker as i32 & 0xffd8 != 0xffd0 || marker as i32 & 7 != rstn as i32 & 7 {
            return JDR_FMT1;
        }
        (*jd).dbit = 0 as u8;
        let ref mut fresh29 = (*jd).dcv[0];
        *fresh29 = 0 as i16;
        let ref mut fresh30 = (*jd).dcv[1];
        *fresh30 = *fresh29;
        (*jd).dcv[2 as usize] = *fresh30;
        return JDR_OK;
    }
}
unsafe fn block_idct(mut src: *mut i32, mut dst: *mut i16) {
    unsafe {
        let M13: i32 = (1.41421f64 * 4096_f64) as i32;
        let M2: i32 = (1.08239f64 * 4096_f64) as i32;
        let M4: i32 = (2.61313f64 * 4096_f64) as i32;
        let M5: i32 = (1.84776f64 * 4096_f64) as i32;
        let mut v0: i32 = 0;
        let mut v1: i32 = 0;
        let mut v2: i32 = 0;
        let mut v3: i32 = 0;
        let mut v4: i32 = 0;
        let mut v5: i32 = 0;
        let mut v6: i32 = 0;
        let mut v7: i32 = 0;
        let mut t10: i32 = 0;
        let mut t11: i32 = 0;
        let mut t12: i32 = 0;
        let mut t13: i32 = 0;
        let mut i: i32 = 0;
        i = 0;
        while i < 8 {
            v0 = *src.offset((8 * 0) as isize);
            v1 = *src.offset((8 * 2) as isize);
            v2 = *src.offset((8 * 4) as isize);
            v3 = *src.offset((8 * 6) as isize);
            t10 = v0 + v2;
            t12 = v0 - v2;
            t11 = (v1 - v3) * M13 >> 12 as i32;
            v3 += v1;
            t11 -= v3;
            v0 = t10 + v3;
            v3 = t10 - v3;
            v1 = t11 + t12;
            v2 = t12 - t11;
            v4 = *src.offset((8 * 7) as isize);
            v5 = *src.offset((8 * 1) as isize);
            v6 = *src.offset((8 * 5) as isize);
            v7 = *src.offset((8 * 3) as isize);
            t10 = v5 - v4;
            t11 = v5 + v4;
            t12 = v6 - v7;
            v7 += v6;
            v5 = (t11 - v7) * M13 >> 12;
            v7 += t11;
            t13 = (t10 + t12) * M5 >> 12;
            v4 = t13 - (t10 * M2 >> 12);
            v6 = t13 - (t12 * M4 >> 12) - v7;
            v5 -= v6;
            v4 -= v5;
            *src.offset((8 * 0) as isize) = v0 + v7;
            *src.offset((8 * 7) as isize) = v0 - v7;
            *src.offset((8 * 1) as isize) = v1 + v6;
            *src.offset((8 * 6) as isize) = v1 - v6;
            *src.offset((8 * 2) as isize) = v2 + v5;
            *src.offset((8 * 5) as isize) = v2 - v5;
            *src.offset((8 * 3) as isize) = v3 + v4;
            *src.offset((8 * 4) as isize) = v3 - v4;
            src = src.offset(1);
            i += 1;
        }
        src = src.offset(-(8));
        i = 0 as i32;
        while i < 8 as i32 {
            v0 = (*src.offset(0) as cty::c_long + ((128 as cty::c_long) << 8)) as i32;
            v1 = *src.offset(2);
            v2 = *src.offset(4);
            v3 = *src.offset(6);
            t10 = v0 + v2;
            t12 = v0 - v2;
            t11 = (v1 - v3) * M13 >> 12;
            v3 += v1;
            t11 -= v3;
            v0 = t10 + v3;
            v3 = t10 - v3;
            v1 = t11 + t12;
            v2 = t12 - t11;
            v4 = *src.offset(7);
            v5 = *src.offset(1);
            v6 = *src.offset(5);
            v7 = *src.offset(3);
            t10 = v5 - v4;
            t11 = v5 + v4;
            t12 = v6 - v7;
            v7 += v6;
            v5 = (t11 - v7) * M13 >> 12;
            v7 += t11;
            t13 = (t10 + t12) * M5 >> 12;
            v4 = t13 - (t10 * M2 >> 12);
            v6 = t13 - (t12 * M4 >> 12) - v7;
            v5 -= v6;
            v4 -= v5;
            *dst.offset(0) = (v0 + v7 >> 8) as i16;
            *dst.offset(7) = (v0 - v7 >> 8) as i16;
            *dst.offset(1) = (v1 + v6 >> 8) as i16;
            *dst.offset(6) = (v1 - v6 >> 8) as i16;
            *dst.offset(2) = (v2 + v5 >> 8) as i16;
            *dst.offset(5) = (v2 - v5 >> 8) as i16;
            *dst.offset(3) = (v3 + v4 >> 8) as i16;
            *dst.offset(4) = (v3 - v4 >> 8) as i16;
            dst = dst.offset(8);
            src = src.offset(8);
            i += 1;
        }
    }
}
unsafe fn mcu_load(mut jd: *mut JDEC) -> JRESULT {
    unsafe {
        let mut tmp: *mut i32 = (*jd).workbuf as *mut i32;
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut blk: u32 = 0;
        let mut nby: u32 = 0;
        let mut i: u32 = 0;
        let mut bc: u32 = 0;
        let mut z: u32 = 0;
        let mut id: u32 = 0;
        let mut cmp: u32 = 0;
        let mut bp: *mut i16 = 0 as *mut i16;
        let mut dqf: *const i32 = 0 as *const i32;
        nby = ((*jd).msx as i32 * (*jd).msy as i32) as u32;
        bp = (*jd).mcubuf;
        blk = 0 as i32 as u32;
        while blk < nby.wrapping_add(2) {
            cmp = if blk < nby {
                0
            } else {
                blk.wrapping_sub(nby).wrapping_add(1)
            };
            if cmp != 0 && (*jd).ncomp as i32 != 3 {
                i = 0;
                while i < 64 {
                    let fresh31 = i;
                    i = i.wrapping_add(1);
                    *bp.offset(fresh31 as isize) = 128;
                }
            } else {
                id = (if cmp != 0 { 1 } else { 0 }) as u32;
                d = huffext(jd, id, 0);
                if d < 0 {
                    return (0 - d) as JRESULT;
                }
                bc = d as u32;
                d = (*jd).dcv[cmp as usize] as i32;
                if bc != 0 {
                    e = bitext(jd, bc);
                    if e < 0 as i32 {
                        return (0 as i32 - e) as JRESULT;
                    }
                    bc = ((1) << bc.wrapping_sub(1)) as u32;
                    if e as u32 & bc == 0 {
                        e = (e as u32).wrapping_sub((bc << 1).wrapping_sub(1)) as i32;
                    }
                    d += e;
                    (*jd).dcv[cmp as usize] = d as i16;
                }
                dqf = (*jd).qttbl[(*jd).qtid[cmp as usize] as usize];
                *tmp.offset(0) = d * *dqf.offset(0) >> 8 as i32;
                memset(
                    &mut *tmp.offset(1) as *mut i32 as *mut cty::c_void,
                    0 as i32,
                    (63 as i32 as cty::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<i32>() as cty::c_ulong),
                );
                z = 1 as i32 as u32;
                loop {
                    d = huffext(jd, id, 1);
                    if d == 0 {
                        break;
                    }
                    if d < 0 {
                        return (0 - d) as JRESULT;
                    }
                    bc = d as u32;
                    z = z.wrapping_add(bc >> 4 as i32);
                    if z >= 64 {
                        return JDR_FMT1;
                    }
                    bc &= 0xf;
                    if bc != 0 {
                        d = bitext(jd, bc);
                        if d < 0 {
                            return (0 - d) as JRESULT;
                        }
                        bc = ((1) << bc.wrapping_sub(1)) as u32;
                        if d as u32 & bc == 0 {
                            d = (d as u32).wrapping_sub((bc << 1 as i32).wrapping_sub(1)) as i32;
                        }
                        i = Zig[z as usize] as u32;
                        *tmp.offset(i as isize) = d * *dqf.offset(i as isize) >> 8 as i32;
                    }
                    z = z.wrapping_add(1);
                    if !(z < 64) {
                        break;
                    }
                }
                if 1 != 2 || cmp == 0 {
                    if z == 1 || 0 != 0 && (*jd).scale == 3 {
                        d = (*tmp / 256 + 128) as i32;
                        if 2 as i32 >= 1 as i32 {
                            i = 0;
                            while i < 64 {
                                let fresh32 = i;
                                i = i.wrapping_add(1);
                                *bp.offset(fresh32 as isize) = d as i16;
                            }
                        } else {
                            memset(bp as *mut cty::c_void, d, 64 as cty::c_ulong);
                        }
                    } else {
                        block_idct(tmp, bp);
                    }
                }
            }
            bp = bp.offset(64);
            blk = blk.wrapping_add(1);
        }
        return JDR_OK;
    }
}
unsafe fn mcu_output(
    mut jd: *mut JDEC,
    mut outfunc: Option<unsafe fn(*mut JDEC, *mut cty::c_void, *mut JRECT) -> i32>,
    mut x: u32,
    mut y: u32,
) -> JRESULT {
    unsafe {
        let CVACC: i32 = if ::core::mem::size_of::<i32>() as cty::c_ulong > 2 as i32 as cty::c_ulong
        {
            1024
        } else {
            128
        };
        let mut ix: u32 = 0;
        let mut iy: u32 = 0;
        let mut mx: u32 = 0;
        let mut my: u32 = 0;
        let mut rx: u32 = 0;
        let mut ry: u32 = 0;
        let mut yy: i32 = 0;
        let mut cb: i32 = 0;
        let mut cr: i32 = 0;
        let mut py: *mut i16 = 0 as *mut i16;
        let mut pc: *mut i16 = 0 as *mut i16;
        let mut pix: *mut u8 = 0 as *mut u8;
        let mut rect: JRECT = JRECT {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };
        mx = ((*jd).msx as i32 * 8) as u32;
        my = ((*jd).msy as i32 * 8) as u32;
        rx = if x.wrapping_add(mx) <= (*jd).width as u32 {
            mx
        } else {
            ((*jd).width as u32).wrapping_sub(x)
        };
        ry = if y.wrapping_add(my) <= (*jd).height as u32 {
            my
        } else {
            ((*jd).height as u32).wrapping_sub(y)
        };
        rect.left = x as u16;
        rect.right = x.wrapping_add(rx).wrapping_sub(1) as u16;
        rect.top = y as u16;
        rect.bottom = y.wrapping_add(ry).wrapping_sub(1) as u16;
        if 0 == 0 || (*jd).scale != 3 {
            pix = (*jd).workbuf as *mut u8;
            if 1 != 2 {
                iy = 0;
                while iy < my {
                    py = (*jd).mcubuf;
                    pc = py;
                    if my == 16 {
                        pc = pc.offset(
                            ((64 * 4) as u32).wrapping_add((iy >> 1 as i32).wrapping_mul(8))
                                as isize,
                        );
                        if iy >= 8 {
                            py = py.offset(64);
                        }
                    } else {
                        pc =
                            pc.offset(mx.wrapping_mul(8).wrapping_add(iy.wrapping_mul(8)) as isize);
                    }
                    py = py.offset(iy.wrapping_mul(8) as isize);
                    ix = 0 as i32 as u32;
                    while ix < mx {
                        cb = *pc.offset(0) as i32 - 128;
                        cr = *pc.offset(64) as i32 - 128;
                        if mx == 16 {
                            if ix == 8 {
                                py = py.offset((64 - 8) as isize);
                            }
                            pc = pc.offset((ix & 1) as isize);
                        } else {
                            pc = pc.offset(1);
                        }
                        let fresh33 = py;
                        py = py.offset(1);
                        yy = *fresh33 as i32;
                        let fresh34 = pix;
                        pix = pix.offset(1);
                        *fresh34 = BYTECLIP(yy + (1.402f64 * CVACC as f64) as i32 * cr / CVACC);
                        let fresh35 = pix;
                        pix = pix.offset(1);
                        *fresh35 = BYTECLIP(
                            yy - ((0.344f64 * CVACC as f64) as i32 * cb
                                + (0.714f64 * CVACC as f64) as i32 * cr)
                                / CVACC,
                        );
                        let fresh36 = pix;
                        pix = pix.offset(1);
                        *fresh36 = BYTECLIP(yy + (1.772f64 * CVACC as f64) as i32 * cb / CVACC);
                        ix = ix.wrapping_add(1);
                    }
                    iy = iy.wrapping_add(1);
                }
            } else {
                iy = 0 as i32 as u32;
                while iy < my {
                    py = ((*jd).mcubuf).offset(iy.wrapping_mul(8) as isize);
                    if my == 16 {
                        if iy >= 8 {
                            py = py.offset(64);
                        }
                    }
                    ix = 0;
                    while ix < mx {
                        if mx == 16 {
                            if ix == 8 {
                                py = py.offset((64 - 8) as isize);
                            }
                        }
                        let fresh37 = py;
                        py = py.offset(1);
                        let fresh38 = pix;
                        pix = pix.offset(1);
                        *fresh38 = *fresh37 as u8;
                        ix = ix.wrapping_add(1);
                    }
                    iy = iy.wrapping_add(1);
                }
            }
            if 0 != 0 && (*jd).scale != 0 {
                let mut x_0: u32 = 0;
                let mut y_0: u32 = 0;
                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;
                let mut s: u32 = 0;
                let mut w: u32 = 0;
                let mut a: u32 = 0;
                let mut op: *mut u8 = 0 as *mut u8;
                s = ((*jd).scale as i32 * 2) as u32;
                w = ((1 as i32) << (*jd).scale as i32) as u32;
                a = mx.wrapping_sub(w).wrapping_mul(
                    (if 1 as i32 != 2 as i32 {
                        3 as i32
                    } else {
                        1 as i32
                    }) as u32,
                );
                op = (*jd).workbuf as *mut u8;
                iy = 0;
                while iy < my {
                    ix = 0;
                    while ix < mx {
                        pix = ((*jd).workbuf as *mut u8).offset(
                            iy.wrapping_mul(mx)
                                .wrapping_add(ix)
                                .wrapping_mul((if 1 != 2 { 3 } else { 1 }) as u32)
                                as isize,
                        );
                        b = 0;
                        g = b;
                        r = g;
                        y_0 = 0;
                        while y_0 < w {
                            x_0 = 0;
                            while x_0 < w {
                                let fresh39 = pix;
                                pix = pix.offset(1);
                                r = r.wrapping_add(*fresh39 as u32);
                                if 1 as i32 != 2 as i32 {
                                    let fresh40 = pix;
                                    pix = pix.offset(1);
                                    g = g.wrapping_add(*fresh40 as u32);
                                    let fresh41 = pix;
                                    pix = pix.offset(1);
                                    b = b.wrapping_add(*fresh41 as u32);
                                }
                                x_0 = x_0.wrapping_add(1);
                            }
                            pix = pix.offset(a as isize);
                            y_0 = y_0.wrapping_add(1);
                        }
                        let fresh42 = op;
                        op = op.offset(1);
                        *fresh42 = (r >> s) as u8;
                        if 1 as i32 != 2 as i32 {
                            let fresh43 = op;
                            op = op.offset(1);
                            *fresh43 = (g >> s) as u8;
                            let fresh44 = op;
                            op = op.offset(1);
                            *fresh44 = (b >> s) as u8;
                        }
                        ix = ix.wrapping_add(w);
                    }
                    iy = iy.wrapping_add(w);
                }
            }
        } else {
            pix = (*jd).workbuf as *mut u8;
            pc = ((*jd).mcubuf).offset(mx.wrapping_mul(my) as isize);
            cb = *pc.offset(0) as i32 - 128 as i32;
            cr = *pc.offset(64) as i32 - 128 as i32;
            iy = 0 as i32 as u32;
            while iy < my {
                py = (*jd).mcubuf;
                if iy == 8 {
                    py = py.offset((64 * 2) as isize);
                }
                ix = 0 as i32 as u32;
                while ix < mx {
                    yy = *py as i32;
                    py = py.offset(64);
                    if 1 as i32 != 2 as i32 {
                        let fresh45 = pix;
                        pix = pix.offset(1);
                        *fresh45 = BYTECLIP(yy + (1.402f64 * CVACC as f64) as i32 * cr / CVACC);
                        let fresh46 = pix;
                        pix = pix.offset(1);
                        *fresh46 = BYTECLIP(
                            yy - ((0.344f64 * CVACC as f64) as i32 * cb
                                + (0.714f64 * CVACC as f64) as i32 * cr)
                                / CVACC,
                        );
                        let fresh47 = pix;
                        pix = pix.offset(1);
                        *fresh47 = BYTECLIP(yy + (1.772f64 * CVACC as f64) as i32 * cb / CVACC);
                    } else {
                        let fresh48 = pix;
                        pix = pix.offset(1);
                        *fresh48 = yy as u8;
                    }
                    ix = ix.wrapping_add(8);
                }
                iy = iy.wrapping_add(8);
            }
        }
        mx >>= (*jd).scale as i32;
        if rx < mx {
            let mut s_0: *mut u8 = 0 as *mut u8;
            let mut d: *mut u8 = 0 as *mut u8;
            let mut x_1: u32 = 0;
            let mut y_1: u32 = 0;
            d = (*jd).workbuf as *mut u8;
            s_0 = d;
            y_1 = 0 as i32 as u32;
            while y_1 < ry {
                x_1 = 0 as i32 as u32;
                while x_1 < rx {
                    let fresh49 = s_0;
                    s_0 = s_0.offset(1);
                    let fresh50 = d;
                    d = d.offset(1);
                    *fresh50 = *fresh49;
                    if 1 as i32 != 2 as i32 {
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
                s_0 = s_0.offset(mx.wrapping_sub(rx).wrapping_mul(
                    (if 1 as i32 != 2 as i32 {
                        3 as i32
                    } else {
                        1 as i32
                    }) as u32,
                ) as isize);
                y_1 = y_1.wrapping_add(1);
            }
        }
        if 1 as i32 == 1 as i32 {
            let mut s_1: *mut u8 = (*jd).workbuf as *mut u8;
            let mut w_0: u16 = 0;
            let mut d_0: *mut u16 = s_1 as *mut u16;
            let mut n: u32 = rx.wrapping_mul(ry);
            loop {
                let fresh55 = s_1;
                s_1 = s_1.offset(1);
                w_0 = ((*fresh55 as i32 & 0xf8 as i32) << 8 as i32) as u16;
                let fresh56 = s_1;
                s_1 = s_1.offset(1);
                w_0 = (w_0 as i32 | (*fresh56 as i32 & 0xfc as i32) << 3 as i32) as u16;
                let fresh57 = s_1;
                s_1 = s_1.offset(1);
                w_0 = (w_0 as i32 | *fresh57 as i32 >> 3 as i32) as u16;
                let fresh58 = d_0;
                d_0 = d_0.offset(1);
                *fresh58 = w_0;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
        }
        return (if outfunc.expect("non-null function pointer")(jd, (*jd).workbuf, &mut rect) != 0 {
            JDR_OK as i32
        } else {
            JDR_INTR as i32
        }) as JRESULT;
    }
}

pub unsafe fn jd_prepare(
    mut jd: *mut JDEC,
    mut infunc: Option<unsafe fn(*mut JDEC, *mut u8, usize) -> usize>,
    mut pool: *mut cty::c_void,
    mut sz_pool: usize,
    mut dev: *mut cty::c_void,
) -> JRESULT {
    unsafe {
        let mut seg: *mut u8 = 0 as *mut u8;
        let mut b: u8 = 0;
        let mut marker: u16 = 0;
        let mut n: u32 = 0;
        let mut i: u32 = 0;
        let mut ofs: u32 = 0;
        let mut len: usize = 0;
        let mut rc: JRESULT = JDR_OK;
        memset(
            jd as *mut cty::c_void,
            0 as i32,
            ::core::mem::size_of::<JDEC>() as cty::c_ulong,
        );
        let ref mut fresh59 = (*jd).pool;
        *fresh59 = pool;
        (*jd).sz_pool = sz_pool;
        let ref mut fresh60 = (*jd).infunc;
        *fresh60 = infunc;
        let ref mut fresh61 = (*jd).device;
        *fresh61 = dev;
        let ref mut fresh62 = (*jd).dcv[0];
        *fresh62 = 0;
        let ref mut fresh63 = (*jd).dcv[1];
        *fresh63 = *fresh62;
        (*jd).dcv[2 as usize] = *fresh63;
        (*jd).rsc = 0;
        (*jd).rst = 0;
        seg = alloc_pool(jd, 512) as *mut u8;
        let ref mut fresh64 = (*jd).inbuf;
        *fresh64 = seg;
        if seg.is_null() {
            return JDR_MEM1;
        }
        marker = 0;
        ofs = marker as u32;
        loop {
            if ((*jd).infunc).expect("non-null function pointer")(jd, seg, 1) != 1 {
                return JDR_INP;
            }
            ofs = ofs.wrapping_add(1);
            marker = ((marker as i32) << 8 | *seg.offset(0) as i32) as u16;
            if !(marker as i32 != 0xffd8) {
                break;
            }
        }
        loop {
            if ((*jd).infunc).expect("non-null function pointer")(jd, seg, 4) != 4 {
                return JDR_INP;
            }
            marker = ((*seg as i32) << 8 | *seg.offset(1) as i32) as u16;
            len = ((*seg.offset(2) as i32) << 8 as i32 | *seg.offset(2).offset(1) as i32) as usize;
            if len <= 2 || marker as i32 >> 8 != 0xff {
                return JDR_FMT1;
            }
            len = len.wrapping_sub(2);
            ofs = (ofs as usize).wrapping_add(4 + len) as u32;
            's_526: {
                let mut current_block_111: u64;
                match marker as i32 & 0xff as i32 {
                    192 => {
                        if len > 512 {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len) != len {
                            return JDR_INP;
                        }
                        (*jd).width = ((*(&mut *seg.offset(3) as *mut u8) as u16 as i32)
                            << 8 as i32
                            | *(&mut *seg.offset(3) as *mut u8).offset(1) as u16 as i32)
                            as u16;
                        (*jd).height = ((*(&mut *seg.offset(1) as *mut u8) as u16 as i32)
                            << 8 as i32
                            | *(&mut *seg.offset(1) as *mut u8).offset(1) as u16 as i32)
                            as u16;
                        (*jd).ncomp = *seg.offset(5);
                        if (*jd).ncomp as i32 != 3 as i32 && (*jd).ncomp as i32 != 1 as i32 {
                            return JDR_FMT3;
                        }
                        i = 0;
                        while i < (*jd).ncomp as u32 {
                            b = *seg.offset(
                                (7 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize
                            );
                            if i == 0 {
                                if b as i32 != 0x11 && b as i32 != 0x22 && b as i32 != 0x21 {
                                    return JDR_FMT3;
                                }
                                (*jd).msx = (b as i32 >> 4) as u8;
                                (*jd).msy = (b as i32 & 15) as u8;
                            } else if b as i32 != 0x11 {
                                return JDR_FMT3;
                            }
                            (*jd).qtid[i as usize] = *seg.offset(
                                (8 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as isize,
                            );
                            if (*jd).qtid[i as usize] as i32 > 3 {
                                return JDR_FMT3;
                            }
                            i = i.wrapping_add(1);
                        }
                        current_block_111 = 5265702136860997526;
                    }
                    221 => {
                        if len > 512 {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len) != len {
                            return JDR_INP;
                        }
                        (*jd).nrst = ((*seg as i32) << 8 | *seg.offset(1) as i32) as u16;
                        current_block_111 = 5265702136860997526;
                    }
                    196 => {
                        if len > 512 {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len) != len {
                            return JDR_INP;
                        }
                        rc = create_huffman_tbl(jd, seg, len);
                        if rc as u64 != 0 {
                            return rc;
                        }
                        current_block_111 = 5265702136860997526;
                    }
                    219 => {
                        if len > 512 {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len) != len {
                            return JDR_INP;
                        }
                        rc = create_qt_tbl(jd, seg, len);
                        if rc as u64 != 0 {
                            return rc;
                        }
                        current_block_111 = 5265702136860997526;
                    }
                    218 => {
                        if len > 512 {
                            return JDR_MEM2;
                        }
                        if ((*jd).infunc).expect("non-null function pointer")(jd, seg, len) != len {
                            return JDR_INP;
                        }
                        if (*jd).width == 0 || (*jd).height == 0 {
                            return JDR_FMT1;
                        }
                        if *seg.offset(0) as i32 != (*jd).ncomp as i32 {
                            return JDR_FMT3;
                        }
                        i = 0 as i32 as u32;
                        while i < (*jd).ncomp as u32 {
                            b = *seg.offset(
                                (2 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize
                            );
                            if b != 0 && b != 0x11 {
                                return JDR_FMT3;
                            }
                            n = if i != 0 { 1 } else { 0 };
                            if ((*jd).huffbits[n as usize][0]).is_null()
                                || ((*jd).huffbits[n as usize][1]).is_null()
                            {
                                return JDR_FMT1;
                            }
                            if ((*jd).qttbl[(*jd).qtid[i as usize] as usize]).is_null() {
                                return JDR_FMT1;
                            }
                            i = i.wrapping_add(1);
                        }
                        n = ((*jd).msy as i32 * (*jd).msx as i32) as u32;
                        if n == 0 {
                            return JDR_FMT1;
                        }
                        len = n.wrapping_mul(64).wrapping_mul(2).wrapping_add(64) as usize;
                        if len < 256 {
                            len = 256;
                        }
                        let ref mut fresh65 = (*jd).workbuf;
                        *fresh65 = alloc_pool(jd, len);
                        if ((*jd).workbuf).is_null() {
                            return JDR_MEM1;
                        }
                        let ref mut fresh66 = (*jd).mcubuf;
                        *fresh66 = alloc_pool(
                            jd,
                            (n.wrapping_add(2).wrapping_mul(64) as usize)
                                .wrapping_mul(::core::mem::size_of::<i16>() as usize),
                        ) as *mut i16;
                        if ((*jd).mcubuf).is_null() {
                            return JDR_MEM1;
                        }
                        ofs = ofs.wrapping_rem(512);
                        if ofs != 0 {
                            (*jd).dctr = ((*jd).infunc).expect("non-null function pointer")(
                                jd,
                                seg.offset(ofs as isize),
                                (512 as u32).wrapping_sub(ofs) as usize,
                            );
                        }
                        let ref mut fresh67 = (*jd).dptr;
                        *fresh67 = seg
                            .offset(ofs as isize)
                            .offset(-(if 2 != 0 { 0 } else { 1 }));
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
                        if ((*jd).infunc).expect("non-null function pointer")(jd, 0 as *mut u8, len)
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
        }
    }
}

pub unsafe fn jd_decomp(
    mut jd: *mut JDEC,
    mut outfunc: Option<unsafe fn(*mut JDEC, *mut cty::c_void, *mut JRECT) -> i32>,
    mut scale: u8,
) -> JRESULT {
    unsafe {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut mx: u32 = 0;
        let mut my: u32 = 0;
        let mut rc: JRESULT = JDR_OK;
        if scale > (if 0 != 0 { 3 } else { 0 }) {
            return JDR_PAR;
        }
        (*jd).scale = scale;
        mx = ((*jd).msx as i32 * 8 as i32) as u32;
        my = ((*jd).msy as i32 * 8 as i32) as u32;
        rc = JDR_OK;
        y = 0;
        while y < (*jd).height as u32 {
            x = 0;
            while x < (*jd).width as u32 {
                if (*jd).nrst as i32 != 0 && {
                    let ref mut fresh68 = (*jd).rst;
                    let fresh69 = *fresh68;
                    *fresh68 = (*fresh68).wrapping_add(1);
                    fresh69 as i32 == (*jd).nrst as i32
                } {
                    let ref mut fresh70 = (*jd).rsc;
                    let fresh71 = *fresh70;
                    *fresh70 = (*fresh70).wrapping_add(1);
                    rc = restart(jd, fresh71);
                    if rc as u32 != JDR_OK as u32 {
                        return rc;
                    }
                    (*jd).rst = 1;
                }
                rc = mcu_load(jd);
                if rc as u32 != JDR_OK as u32 {
                    return rc;
                }
                rc = mcu_output(jd, outfunc, x, y);
                if rc as u32 != JDR_OK as u32 {
                    return rc;
                }
                x = x.wrapping_add(mx);
            }
            y = y.wrapping_add(my);
        }
        return rc;
    }
}
