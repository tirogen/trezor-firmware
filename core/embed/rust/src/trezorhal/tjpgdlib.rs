#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use core::{mem, slice};

extern "C" {
    fn memset(_: *mut cty::c_void, _: i32, _: cty::c_ulong) -> *mut
cty::c_void; }

const JD_FORMAT: u32 = 1;

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

pub struct JRECT {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

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
    pub huffbits: [[Option<&'static mut [u8]>; 2]; 2],
    pub huffcode: [[Option<&'static mut [u16]>; 2]; 2],
    pub huffdata: [[Option<&'static mut [u8]>; 2]; 2],
    pub qttbl: [Option<&'static mut [i32]>; 4],
    pub wreg: u32,
    pub marker: u8,
    pub longofs: [[u8; 2]; 2],
    pub hufflut_ac: [Option<&'static mut [u16]>; 2],
    pub hufflut_dc: [Option<&'static mut [u8]>; 2],
    pub workbuf: Option<&'static mut [i32]>,
    pub mcubuf: Option<&'static mut [i16]>,
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

unsafe fn alloc_pool_slice<T>(mut jd: *mut JDEC, mut ndata: usize) -> Result<&'static mut [T], ()> {
    unsafe {
        let mut rp: *mut cty::c_char = 0 as *mut cty::c_char;
        let ndata_bytes = ndata * core::mem::size_of::<T>();
        let ndata_aligned = (ndata_bytes + 3) & !3;
        if (*jd).sz_pool >= ndata_aligned {
            let ref mut fresh0 = (*jd).sz_pool;
            *fresh0 = *fresh0 - ndata_aligned;
            rp = (*jd).pool as *mut cty::c_char;
            let ref mut fresh1 = (*jd).pool;
            *fresh1 = rp.offset(ndata_aligned as isize) as *mut cty::c_void;
            return Ok(slice::from_raw_parts_mut(rp as *mut T, ndata));
        }
        Err(())
    }
}

unsafe fn i32_slice_to_u8(data: &'static mut [i32]) -> &'static mut [u8] {
    let len = data.len() * 4;
    let ptr = data.as_mut_ptr() as *mut u8;
    mem::forget(data);
    unsafe { slice::from_raw_parts_mut(ptr, len) }
}

unsafe fn create_qt_tbl(mut jd: *mut JDEC, mut data: *const u8, mut ndata: usize) -> JRESULT {
    unsafe {
        let mut i: u32 = 0;
        let mut zi: u32 = 0;
        let mut d: u8 = 0;
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

            let pb = alloc_pool_slice(jd, 64);
            if pb.is_err() {
                return JDR_MEM1;
            }
            (*jd).qttbl[i as usize] = Some(unwrap!(pb));
            let mut j: u32 = 0;
            while j < 64 {
                zi = Zig[j as usize] as u32;
                let fresh4 = data;
                data = data.offset(1);

                unwrap!((*jd).qttbl[i as usize].as_mut())[zi as usize] =
                    (*fresh4 as u32).wrapping_mul(Ipsf[zi as usize] as u32) as i32;
                j = j.wrapping_add(1);
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
        let mut hc: u16 = 0;
        while ndata != 0 {
            if ndata < 17 {
                return JDR_FMT1;
            }
            ndata -= 17;
            let fresh5 = data;
            data = data.offset(1);
            d = *fresh5;
            if d & 0xee != 0 {
                return JDR_FMT1;
            }
            cls = (d as i32 >> 4) as u32;
            num = (d as i32 & 0xf) as u32;
            let mem = alloc_pool_slice(jd, 16);
            if mem.is_err() {
                return JDR_MEM1;
            }
            (*jd).huffbits[num as usize][cls as usize] = Some(unwrap!(mem));

            i = 0;
            np = i as usize;
            while i < 16 {
                let fresh7 = data;
                data = data.offset(1);
                unwrap!((*jd).huffbits[num as usize][cls as usize].as_mut())[i as usize] = *fresh7;
                np = (np as cty::c_ulong).wrapping_add(*fresh7 as cty::c_ulong) as usize;
                i = i.wrapping_add(1);
            }
            let mem = alloc_pool_slice(jd, np);
            if mem.is_err() {
                return JDR_MEM1;
            }
            (*jd).huffcode[num as usize][cls as usize] = Some(unwrap!(mem));

            hc = 0;
            i = 0;
            j = i;
            while i < 16 {
                b = unwrap!((*jd).huffbits[num as usize][cls as usize].as_ref())[i as usize] as u32;
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
                    unwrap!((*jd).huffcode[num as usize][cls as usize].as_mut())
                        [fresh12 as usize] = fresh11;
                }
                hc = ((hc as i32) << 1) as u16;
                i = i.wrapping_add(1);
            }
            if ndata < np {
                return JDR_FMT1;
            }
            ndata -= np;
            let mem = alloc_pool_slice(jd, np);
            if mem.is_err() {
                return JDR_MEM1;
            }
            (*jd).huffdata[num as usize][cls as usize] = Some(unwrap!(mem));
            i = 0;
            while i < np as u32 {
                let fresh14 = data;
                data = data.offset(1);
                d = *fresh14;
                if cls == 0 && d > 11 {
                    return JDR_FMT1;
                }
                unwrap!((*jd).huffdata[num as usize][cls as usize].as_mut())[i as usize] = d;
                i += 1;
            }
            let mut span: u32 = 0;
            let mut td: u32 = 0;
            let mut ti: u32 = 0;
            if cls != 0 {
                let tbl_ac = alloc_pool_slice(jd, 1 << HUFF_BIT);
                if tbl_ac.is_err() {
                    return JDR_MEM1;
                }
                (*jd).hufflut_ac[num as usize] = Some(unwrap!(tbl_ac));
                unwrap!((*jd).hufflut_ac[num as usize].as_mut()).fill(0xffff);
            } else {
                let tbl_dc = alloc_pool_slice(jd, 1 << HUFF_BIT);
                if tbl_dc.is_err() {
                    return JDR_MEM1;
                }
                (*jd).hufflut_dc[num as usize] = Some(unwrap!(tbl_dc));
                unwrap!((*jd).hufflut_dc[num as usize].as_mut()).fill(0xff);
            }
            b = 0;
            i = b;
            while b < HUFF_BIT {
                j = unwrap!((*jd).huffbits[num as usize][cls as usize].as_ref())[b as usize] as u32;
                while j != 0 {
                    ti = (unwrap!((*jd).huffcode[num as usize][cls as usize].as_ref())[i as usize]
                        << ((HUFF_BIT - 1) as u32).wrapping_sub(b)
                        & (1 << HUFF_BIT) - 1) as u32;

                    if cls != 0 {
                        td = unwrap!((*jd).huffdata[num as usize][cls as usize].as_ref())
                            [i as usize] as u32
                            | b.wrapping_add(1) << 8;
                        i += 1;
                        span = (1 << ((HUFF_BIT - 1) as u32).wrapping_sub(b)) as u32;
                        while span != 0 {
                            span = span.wrapping_sub(1);
                            let fresh18 = ti;
                            ti = ti.wrapping_add(1);
                            unwrap!((*jd).hufflut_ac[num as usize].as_mut())[fresh18 as usize] =
                                td as u16;
                        }
                    } else {
                        td = unwrap!((*jd).huffdata[num as usize][cls as usize].as_ref())
                            [i as usize] as u32
                            | b.wrapping_add(1) << 4 as i32;
                        i += 1;
                        span = ((1 as i32) << ((HUFF_BIT - 1) as u32).wrapping_sub(b)) as u32;
                        while span != 0 {
                            span = span.wrapping_sub(1);
                            let fresh20 = ti;
                            ti = ti.wrapping_add(1);
                            unwrap!((*jd).hufflut_dc[num as usize].as_mut())[fresh20 as usize] =
                                td as u8;
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
            d = unwrap!((*jd).hufflut_ac[id as usize].as_ref())[d as usize] as u32;
            if d != 0xffff {
                (*jd).dbit = wbit.wrapping_sub(d >> 8) as u8;
                return (d & 0xff) as i32;
            }
        } else {
            d = unwrap!((*jd).hufflut_dc[id as usize].as_ref())[d as usize] as u32;
            if d != 0xff {
                (*jd).dbit = wbit.wrapping_sub(d >> 4) as u8;
                return (d & 0xf) as i32;
            }
        }
        let mut hb_idx = 0;
        let mut hc_idx = 0;
        let mut hd_idx = 0;

        // hc = ((*jd).huffcode[id as usize][cls as usize])
        //     .offset((*jd).longofs[id as usize][cls as usize] as isize);
        // hd = unwrap!(((*jd).huffdata[id as usize][cls as usize]).as_ref())
        //     [hd_idx + (*jd).longofs[id as usize][cls as usize] as usize];
        bl = (HUFF_BIT + 1) as u32;
        while bl <= 16 {
            nc = unwrap!((*jd).huffbits[id as usize][cls as usize].as_ref())
                [(hb_idx + HUFF_BIT) as usize] as u32;
            hb_idx += 1;
            if nc != 0 {
                d = w >> wbit.wrapping_sub(bl);
                loop {
                    let fresh24 = unwrap!((*jd).huffcode[id as usize][cls as usize].as_ref())
                        [hc_idx + (*jd).longofs[id as usize][cls as usize] as usize];
                    hc_idx += 1;
                    if d == fresh24 as u32 {
                        (*jd).dbit = wbit.wrapping_sub(bl) as u8;

                        return unwrap!(((*jd).huffdata[id as usize][cls as usize]).as_ref())
                            [hd_idx + (*jd).longofs[id as usize][cls as usize] as usize]
                            as i32;
                    }
                    hd_idx += 1;
                    nc = nc.wrapping_sub(1);
                    if !(nc != 0) {
                        break;
                    }
                }
            }
            bl = bl.wrapping_add(1);
        }
        return 0 - JDR_FMT1 as i32;
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
fn block_idct(src: &mut &mut [i32], mut dst: &mut [i16]) {
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
    let mut dst_idx = 0;
    let mut src_idx = 0;
    i = 0;
    while i < 8 {
        v0 = src[src_idx + 8 * 0];
        v1 = src[src_idx + 8 * 2];
        v2 = src[src_idx + 8 * 4];
        v3 = src[src_idx + 8 * 6];
        t10 = v0 + v2;
        t12 = v0 - v2;
        t11 = (v1 - v3) * M13 >> 12 as i32;
        v3 += v1;
        t11 -= v3;
        v0 = t10 + v3;
        v3 = t10 - v3;
        v1 = t11 + t12;
        v2 = t12 - t11;
        v4 = src[src_idx + 8 * 7];
        v5 = src[src_idx + 8 * 1];
        v6 = src[src_idx + 8 * 5];
        v7 = src[src_idx + 8 * 3];
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
        src[src_idx + 8 * 0] = v0 + v7;
        src[src_idx + 8 * 7] = v0 - v7;
        src[src_idx + 8 * 1] = v1 + v6;
        src[src_idx + 8 * 6] = v1 - v6;
        src[src_idx + 8 * 2] = v2 + v5;
        src[src_idx + 8 * 5] = v2 - v5;
        src[src_idx + 8 * 3] = v3 + v4;
        src[src_idx + 8 * 4] = v3 - v4;
        src_idx += 1;
        i += 1;
    }
    src_idx -= 8;
    i = 0;
    while i < 8 {
        v0 = src[src_idx + 0] + (128 << 8);
        v1 = src[src_idx + 2];
        v2 = src[src_idx + 4];
        v3 = src[src_idx + 6];
        t10 = v0 + v2;
        t12 = v0 - v2;
        t11 = (v1 - v3) * M13 >> 12;
        v3 += v1;
        t11 -= v3;
        v0 = t10 + v3;
        v3 = t10 - v3;
        v1 = t11 + t12;
        v2 = t12 - t11;
        v4 = src[src_idx + 7];
        v5 = src[src_idx + 1];
        v6 = src[src_idx + 5];
        v7 = src[src_idx + 3];
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
        dst[dst_idx + 0] = (v0 + v7 >> 8) as i16;
        dst[dst_idx + 7] = (v0 - v7 >> 8) as i16;
        dst[dst_idx + 1] = (v1 + v6 >> 8) as i16;
        dst[dst_idx + 6] = (v1 - v6 >> 8) as i16;
        dst[dst_idx + 2] = (v2 + v5 >> 8) as i16;
        dst[dst_idx + 5] = (v2 - v5 >> 8) as i16;
        dst[dst_idx + 3] = (v3 + v4 >> 8) as i16;
        dst[dst_idx + 4] = (v3 - v4 >> 8) as i16;
        dst_idx += 8;
        src_idx += 8;
        i += 1;
    }
}

unsafe fn mcu_load(mut jd: *mut JDEC) -> JRESULT {
    unsafe {
        let mut d: i32 = 0;
        let mut e: i32 = 0;
        let mut blk: u32 = 0;
        let mut nby: u32 = 0;
        let mut i: u32 = 0;
        let mut bc: u32 = 0;
        let mut z: u32 = 0;
        let mut id: u32 = 0;
        let mut cmp: u32 = 0;
        nby = ((*jd).msx as i32 * (*jd).msy as i32) as u32;
        let mut mcu_buf_idx = 0;
        blk = 0;
        while blk < nby.wrapping_add(2) {
            cmp = if blk < nby {
                0
            } else {
                blk.wrapping_sub(nby).wrapping_add(1)
            };
            if cmp != 0 && (*jd).ncomp as i32 != 3 {
                i = 0;
                while i < 64 {
                    unwrap!((*jd).mcubuf.as_mut())[mcu_buf_idx + i as usize] = 128 as i16;
                    i += 1;
                }
            } else {
                id = if cmp != 0 { 1 } else { 0 };
                d = huffext(jd, id, 0);
                if d < 0 {
                    return (0 - d) as JRESULT;
                }
                bc = d as u32;
                d = (*jd).dcv[cmp as usize] as i32;
                if bc != 0 {
                    e = bitext(jd, bc);
                    if e < 0 {
                        return (0 - e) as JRESULT;
                    }
                    bc = ((1) << bc.wrapping_sub(1)) as u32;
                    if e as u32 & bc == 0 {
                        e = (e as u32).wrapping_sub((bc << 1).wrapping_sub(1)) as i32;
                    }
                    d += e;
                    (*jd).dcv[cmp as usize] = d as i16;
                }
                let dfq = unwrap!((*jd).qttbl[(*jd).qtid[cmp as usize] as usize].as_ref());
                unwrap!((*jd).workbuf.as_mut())[0] = d * dfq[0] >> 8;
                unwrap!((*jd).workbuf.as_mut())[1..63].fill(0);
                z = 1;
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

                        unwrap!((*jd).workbuf.as_mut())[i as usize] =
                            d * dfq[i as usize] >> 8 as i32;
                    }
                    z = z.wrapping_add(1);
                    if !(z < 64) {
                        break;
                    }
                }
                if 1 != 2 || cmp == 0 {
                    if z == 1 || 0 != 0 && (*jd).scale == 3 {
                        d = (unwrap!((*jd).workbuf.as_ref())[0] / 256 + 128) as i32;
                        if 2 >= 1 {
                            i = 0;
                            while i < 64 {
                                unwrap!((*jd).mcubuf.as_mut())[mcu_buf_idx + i as usize] = d as i16;
                                i += 1;
                            }
                        } else {
                            unwrap!((*jd).mcubuf.as_mut())[..64].fill(d as i16);
                        }
                    } else {
                        block_idct(
                            unwrap!((*jd).workbuf.as_mut()),
                            &mut unwrap!((*jd).mcubuf.as_mut())[mcu_buf_idx..],
                        );
                    }
                }
            }
            mcu_buf_idx += 64;
            blk = blk.wrapping_add(1);
        }
        return JDR_OK;
    }
}
unsafe fn mcu_output(
    mut jd: *mut JDEC,
    mut outfunc: Option<unsafe fn(*mut JDEC, &&mut [i32], *mut JRECT) -> i32>,
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
        let mut py_idx: usize = 0;
        let mut pc_idx: usize = 0;
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
        let workbuf = i32_slice_to_u8(unwrap!((*jd).workbuf.as_mut()));
        let mut pix_idx: usize = 0;
        let mut op_idx: usize = 0;

        if 0 == 0 || (*jd).scale != 3 {
            if 1 != 2 {
                iy = 0;
                while iy < my {
                    py_idx = 0;
                    pc_idx = 0;
                    if my == 16 {
                        pc_idx += (64 * 4) + ((iy as usize >> 1) * 8);
                        if iy >= 8 {
                            py_idx += 64;
                        }
                    } else {
                        pc_idx += (mx * 8 + iy * 8) as usize;
                    }
                    py_idx += (iy * 8) as usize;
                    ix = 0;
                    while ix < mx {
                        cb = unwrap!((*jd).mcubuf.as_mut())[pc_idx + 0] as i32 - 128;
                        cr = unwrap!((*jd).mcubuf.as_mut())[pc_idx + 64] as i32 - 128;
                        if mx == 16 {
                            if ix == 8 {
                                py_idx += 64 - 8;
                            }
                            pc_idx += (ix & 1) as usize;
                        } else {
                            pc_idx += 1;
                        }
                        yy = unwrap!((*jd).mcubuf.as_ref())[py_idx + 0] as i32;
                        py_idx += 1;

                        workbuf[pix_idx] =
                            BYTECLIP(yy + (1.402f64 * CVACC as f64) as i32 * cr / CVACC);
                        pix_idx += 1;
                        workbuf[pix_idx] = BYTECLIP(
                            yy - ((0.344f64 * CVACC as f64) as i32 * cb
                                + (0.714f64 * CVACC as f64) as i32 * cr)
                                / CVACC,
                        );
                        pix_idx += 1;
                        workbuf[pix_idx] =
                            BYTECLIP(yy + (1.772f64 * CVACC as f64) as i32 * cb / CVACC);
                        pix_idx += 1;
                        ix = ix.wrapping_add(1);
                    }
                    iy = iy.wrapping_add(1);
                }
            } else {
                iy = 0;
                while iy < my {
                    py_idx = (iy * 8) as usize;
                    if my == 16 {
                        if iy >= 8 {
                            py_idx += 64;
                        }
                    }
                    ix = 0;
                    while ix < mx {
                        if mx == 16 {
                            if ix == 8 {
                                py_idx += 64 - 8;
                            }
                        }
                        workbuf[pix_idx] = unwrap!((*jd).mcubuf.as_ref())[py_idx] as u8;
                        pix_idx += 1;
                        py_idx += 1;
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
                s = ((*jd).scale as i32 * 2) as u32;
                w = ((1 as i32) << (*jd).scale as i32) as u32;
                a = mx.wrapping_sub(w).wrapping_mul(if 1 != 2 { 3 } else { 1 });
                op_idx = 0;
                iy = 0;
                while iy < my {
                    ix = 0;
                    while ix < mx {
                        pix_idx = ((iy * mx + ix) * (if JD_FORMAT != 2 { 3 } else { 1 })) as usize;
                        b = 0;
                        g = b;
                        r = g;
                        y_0 = 0;
                        while y_0 < w {
                            x_0 = 0;
                            while x_0 < w {
                                r = r.wrapping_add(workbuf[pix_idx] as u32);
                                pix_idx += 1;
                                if JD_FORMAT != 2 {
                                    g = g.wrapping_add(workbuf[pix_idx] as u32);
                                    pix_idx += 1;
                                    b = b.wrapping_add(workbuf[pix_idx] as u32);
                                    pix_idx += 1;
                                }
                                x_0 = x_0.wrapping_add(1);
                            }
                            pix_idx += a as usize;
                            y_0 = y_0.wrapping_add(1);
                        }
                        workbuf[op_idx] = (r >> s) as u8;
                        op_idx += 1;
                        if JD_FORMAT != 2 {
                            workbuf[op_idx] = (g >> s) as u8;
                            op_idx += 1;
                            workbuf[op_idx] = (b >> s) as u8;
                            op_idx += 1;
                        }
                        ix = ix.wrapping_add(w);
                    }
                    iy = iy.wrapping_add(w);
                }
            }
        } else {
            pix_idx = 0;
            pc_idx = (mx * my) as usize;
            cb = unwrap!((*jd).mcubuf.as_ref())[pc_idx + 0] as i32 - 128;
            cr = unwrap!((*jd).mcubuf.as_ref())[pc_idx + 64] as i32 - 128;
            iy = 0;
            while iy < my {
                py_idx = 0;
                if iy == 8 {
                    py_idx = 64 * 2;
                }
                ix = 0;
                while ix < mx {
                    yy = unwrap!((*jd).mcubuf.as_ref())[py_idx] as i32;
                    py_idx += 64;
                    if JD_FORMAT != 2 {
                        workbuf[pix_idx] =
                            BYTECLIP(yy + (1.402f64 * CVACC as f64) as i32 * cr / CVACC);
                        pix_idx += 1;
                        workbuf[pix_idx] = BYTECLIP(
                            yy - ((0.344f64 * CVACC as f64) as i32 * cb
                                + (0.714f64 * CVACC as f64) as i32 * cr)
                                / CVACC,
                        );
                        pix_idx += 1;
                        workbuf[pix_idx] =
                            BYTECLIP(yy + (1.772f64 * CVACC as f64) as i32 * cb / CVACC);
                        pix_idx += 1;
                    } else {
                        workbuf[pix_idx] = yy as u8;
                        pix_idx += 1;
                    }
                    ix = ix.wrapping_add(8);
                }
                iy = iy.wrapping_add(8);
            }
        }
        mx >>= (*jd).scale as i32;
        if rx < mx {
            let mut s_0_idx = 0;
            let mut d_idx = 0;
            let mut x_1: u32 = 0;
            let mut y_1: u32 = 0;
            y_1 = 0;
            while y_1 < ry {
                x_1 = 0;
                while x_1 < rx {
                    workbuf[d_idx] = workbuf[s_0_idx];
                    s_0_idx += 1;
                    d_idx += 1;
                    if JD_FORMAT != 2 {
                        workbuf[d_idx] = workbuf[s_0_idx];
                        s_0_idx += 1;
                        d_idx += 1;
                        workbuf[d_idx] = workbuf[s_0_idx];
                        s_0_idx += 1;
                        d_idx += 1;
                    }
                    x_1 = x_1.wrapping_add(1);
                }
                s_0_idx += ((mx - rx) * (if JD_FORMAT != 2 { 3 } else { 1 })) as usize;

                y_1 = y_1.wrapping_add(1);
            }
        }
        if JD_FORMAT == 1 {
            let mut s_1_idx = 0;
            let mut d_0_idx = 0;
            let mut w_0: u16 = 0;
            let mut n: u32 = rx.wrapping_mul(ry);
            loop {
                w_0 = ((workbuf[s_1_idx] as i32 & 0xf8) << 8) as u16;
                s_1_idx += 1;
                w_0 = (w_0 as i32 | (workbuf[s_1_idx] as i32 & 0xfc) << 3) as u16;
                s_1_idx += 1;
                w_0 = (w_0 as i32 | workbuf[s_1_idx] as i32 >> 3) as u16;
                s_1_idx += 1;

                workbuf[d_0_idx] = (w_0 & 0xFF) as u8;
                workbuf[d_0_idx + 1] = (w_0 >> 8) as u8;
                d_0_idx += 2;

                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
        }
        return (if outfunc.expect("non-null function pointer")(
            jd,
            unwrap!((*jd).workbuf.as_ref()),
            &mut rect,
        ) != 0
        {
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
                        i = 0;
                        while i < (*jd).ncomp as u32 {
                            b = *seg.offset(
                                (2 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as isize
                            );
                            if b != 0 && b != 0x11 {
                                return JDR_FMT3;
                            }
                            n = if i != 0 { 1 } else { 0 };
                            if ((*jd).huffbits[n as usize][0]).is_none()
                                || ((*jd).huffbits[n as usize][1]).is_none()
                            {
                                return JDR_FMT1;
                            }
                            if ((*jd).qttbl[(*jd).qtid[i as usize] as usize]).is_none() {
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
                        let mem = alloc_pool_slice(jd, len / 4);
                        if mem.is_err() {
                            return JDR_MEM1;
                        }
                        (*jd).workbuf = Some(unwrap!(mem));

                        let mcubuf = alloc_pool_slice(jd, (n as usize + 2) * 64);
                        if mcubuf.is_err() {
                            return JDR_MEM1;
                        }
                        (*jd).mcubuf = Some(unwrap!(mcubuf));

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
    mut outfunc: Option<unsafe fn(*mut JDEC, &&mut [i32], *mut JRECT) -> i32>,
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
