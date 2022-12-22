#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

use crate::trezorhal::buffers::{get_jpeg_work_buffer, BufferJpeg, get_jpeg_buffer};
use core::{mem, slice};

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

pub struct JDEC<'a> {
    pub dctr: usize,
    pub dptr: usize,
    pub inbuf: Option<&'static mut [u8]>,
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
    pub pool: Option<&'static mut [u8]>,
    pub sz_pool: usize,
    pub pool_start: usize,

    // context
    data_read: usize,
    data_len: usize,
    buffer_width: i16,
    buffer_height: i16,
    current_line: i16,
    current_line_pix: i16,
    data: &'a [u8],
    pub buffer: &'static mut BufferJpeg,
}

pub struct JpegInfo {
    pub width: u16,
    pub height: u16,
    pub mcu_height: u16,
}

static Zig: [u8; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];
static Ipsf: [u16; 64] = [
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

fn BYTECLIP(val: i32) -> u8 {
    if val < 0 {
        return 0;
    }
    if val > 255 {
        return 255;
    }
    return val as u8;
}

unsafe fn alloc_pool_slice<T>(mut jd: &mut JDEC, ndata: usize) -> Result<&'static mut [T], ()> {
    unsafe {
        let ndata_bytes = ndata * mem::size_of::<T>();
        let ndata_aligned = (ndata_bytes + 3) & !3;
        if jd.sz_pool >= ndata_aligned {
            let start = jd.pool_start;
            let end = jd.pool_start + ndata_aligned;
            let data = &mut unwrap!(jd.pool.as_mut())[start..end];
            jd.pool_start = end;
            jd.sz_pool = jd.sz_pool - ndata_aligned;
            return Ok(slice::from_raw_parts_mut(data.as_ptr() as *mut T, ndata));
        }
        Err(())
    }
}

fn create_qt_tbl(mut jd: &mut JDEC, mut ndata: usize) -> JRESULT {
    let mut i: u32;
    let mut zi: u32;
    let mut d: u8;
    let mut data_idx = 0;
    while ndata != 0 {
        if ndata < 65 {
            return JDR_FMT1;
        }
        ndata -= 65;

        d = unwrap!(jd.inbuf.as_ref())[data_idx];
        data_idx += 1;
        if d as i32 & 0xf0 != 0 {
            return JDR_FMT1;
        }
        i = (d as i32 & 3) as u32;

        let pb = unsafe { alloc_pool_slice(jd, 64) };
        if pb.is_err() {
            return JDR_MEM1;
        }
        jd.qttbl[i as usize] = Some(unwrap!(pb));
        let mut j: u32 = 0;
        while j < 64 {
            zi = Zig[j as usize] as u32;

            unwrap!(jd.qttbl[i as usize].as_mut())[zi as usize] = (unwrap!(jd.inbuf.as_ref())
                [data_idx]
                as u32)
                .wrapping_mul(Ipsf[zi as usize] as u32)
                as i32;
            j = j.wrapping_add(1);
            data_idx += 1;
        }
    }
    return JDR_OK;
}
fn create_huffman_tbl(mut jd: &mut JDEC, mut ndata: usize) -> JRESULT {
    let mut i: u32;
    let mut j: u32;
    let mut b: u32;
    let mut cls: u32;
    let mut num: u32;
    let mut np: usize;
    let mut d: u8;
    let mut hc: u16;
    let mut data_idx = 0;
    while ndata != 0 {
        if ndata < 17 {
            return JDR_FMT1;
        }
        ndata -= 17;
        d = unwrap!(jd.inbuf.as_ref())[data_idx];
        data_idx += 1;
        if d & 0xee != 0 {
            return JDR_FMT1;
        }
        cls = (d as i32 >> 4) as u32;
        num = (d as i32 & 0xf) as u32;
        let mem = unsafe { alloc_pool_slice(jd, 16) };
        if mem.is_err() {
            return JDR_MEM1;
        }
        jd.huffbits[num as usize][cls as usize] = Some(unwrap!(mem));

        i = 0;
        np = i as usize;
        while i < 16 {
            unwrap!(jd.huffbits[num as usize][cls as usize].as_mut())[i as usize] =
                unwrap!(jd.inbuf.as_ref())[data_idx];
            np = (np as cty::c_ulong)
                .wrapping_add(unwrap!(jd.inbuf.as_ref())[data_idx] as cty::c_ulong)
                as usize;
            data_idx += 1;
            i = i.wrapping_add(1);
        }
        let mem = unsafe { alloc_pool_slice(jd, np) };
        if mem.is_err() {
            return JDR_MEM1;
        }
        jd.huffcode[num as usize][cls as usize] = Some(unwrap!(mem));

        hc = 0;
        i = 0;
        j = i;
        while i < 16 {
            b = unwrap!(jd.huffbits[num as usize][cls as usize].as_ref())[i as usize] as u32;
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
                unwrap!(jd.huffcode[num as usize][cls as usize].as_mut())[fresh12 as usize] =
                    fresh11;
            }
            hc = ((hc as i32) << 1) as u16;
            i = i.wrapping_add(1);
        }
        if ndata < np {
            return JDR_FMT1;
        }
        ndata -= np;
        let mem = unsafe { alloc_pool_slice(jd, np) };
        if mem.is_err() {
            return JDR_MEM1;
        }
        jd.huffdata[num as usize][cls as usize] = Some(unwrap!(mem));
        i = 0;
        while i < np as u32 {
            d = unwrap!(jd.inbuf.as_ref())[data_idx];
            data_idx += 1;
            if cls == 0 && d > 11 {
                return JDR_FMT1;
            }
            unwrap!(jd.huffdata[num as usize][cls as usize].as_mut())[i as usize] = d;
            i += 1;
        }
        let mut span: u32;
        let mut td: u32;
        let mut ti: u32;
        if cls != 0 {
            let tbl_ac = unsafe { alloc_pool_slice(jd, 1 << HUFF_BIT) };
            if tbl_ac.is_err() {
                return JDR_MEM1;
            }
            jd.hufflut_ac[num as usize] = Some(unwrap!(tbl_ac));
            unwrap!(jd.hufflut_ac[num as usize].as_mut()).fill(0xffff);
        } else {
            let tbl_dc = unsafe { alloc_pool_slice(jd, 1 << HUFF_BIT) };
            if tbl_dc.is_err() {
                return JDR_MEM1;
            }
            jd.hufflut_dc[num as usize] = Some(unwrap!(tbl_dc));
            unwrap!(jd.hufflut_dc[num as usize].as_mut()).fill(0xff);
        }
        b = 0;
        i = b;
        while b < HUFF_BIT {
            j = unwrap!(jd.huffbits[num as usize][cls as usize].as_ref())[b as usize] as u32;
            while j != 0 {
                ti = (unwrap!(jd.huffcode[num as usize][cls as usize].as_ref())[i as usize]
                    << ((HUFF_BIT - 1) as u32).wrapping_sub(b)
                    & (1 << HUFF_BIT) - 1) as u32;

                if cls != 0 {
                    td = unwrap!(jd.huffdata[num as usize][cls as usize].as_ref())[i as usize]
                        as u32
                        | b.wrapping_add(1) << 8;
                    i += 1;
                    span = (1 << ((HUFF_BIT - 1) as u32).wrapping_sub(b)) as u32;
                    while span != 0 {
                        span = span.wrapping_sub(1);
                        let fresh18 = ti;
                        ti = ti.wrapping_add(1);
                        unwrap!(jd.hufflut_ac[num as usize].as_mut())[fresh18 as usize] = td as u16;
                    }
                } else {
                    td = unwrap!(jd.huffdata[num as usize][cls as usize].as_ref())[i as usize]
                        as u32
                        | b.wrapping_add(1) << 4 as i32;
                    i += 1;
                    span = ((1 as i32) << ((HUFF_BIT - 1) as u32).wrapping_sub(b)) as u32;
                    while span != 0 {
                        span = span.wrapping_sub(1);
                        let fresh20 = ti;
                        ti = ti.wrapping_add(1);
                        unwrap!(jd.hufflut_dc[num as usize].as_mut())[fresh20 as usize] = td as u8;
                    }
                }
                j = j.wrapping_sub(1);
            }
            b = b.wrapping_add(1);
        }
        jd.longofs[num as usize][cls as usize] = i as u8;
    }
    return JDR_OK;
}
fn huffext(mut jd: &mut JDEC, id: u32, cls: u32) -> i32 {
    let mut dc: usize = jd.dctr;
    let mut dp: usize = jd.dptr;
    let mut d: u32;
    let mut flg: u32 = 0;
    let mut nc: u32;
    let mut bl: u32;
    let mut wbit: u32 = (jd.dbit as i32 % 32) as u32;
    let mut w: u32 = (jd.wreg as cty::c_ulong
        & ((1 as cty::c_ulong) << wbit).wrapping_sub(1 as i32 as cty::c_ulong))
        as u32;
    while wbit < 16 {
        if jd.marker != 0 {
            d = 0xff;
        } else {
            if dc == 0 {
                dp = 0;
                dc = jpeg_in_buffer(jd, Some(0), 512);
                if dc == 0 {
                    return 0 as i32 - JDR_INP as i32;
                }
            }
            d = unwrap!(jd.inbuf.as_mut())[dp] as u32;
            dp += 1;

            dc = dc.wrapping_sub(1);
            if flg != 0 {
                flg = 0;
                if d != 0 {
                    jd.marker = d as u8;
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
    jd.dctr = dc;
    jd.dptr = dp;
    jd.wreg = w;
    d = w >> wbit.wrapping_sub(HUFF_BIT);
    if cls != 0 {
        d = unwrap!(jd.hufflut_ac[id as usize].as_ref())[d as usize] as u32;
        if d != 0xffff {
            jd.dbit = wbit.wrapping_sub(d >> 8) as u8;
            return (d & 0xff) as i32;
        }
    } else {
        d = unwrap!(jd.hufflut_dc[id as usize].as_ref())[d as usize] as u32;
        if d != 0xff {
            jd.dbit = wbit.wrapping_sub(d >> 4) as u8;
            return (d & 0xf) as i32;
        }
    }
    let mut hb_idx = 0;
    let mut hc_idx = 0;
    let mut hd_idx = 0;

    bl = (HUFF_BIT + 1) as u32;
    while bl <= 16 {
        nc = unwrap!(jd.huffbits[id as usize][cls as usize].as_ref())[(hb_idx + HUFF_BIT) as usize]
            as u32;
        hb_idx += 1;
        if nc != 0 {
            d = w >> wbit.wrapping_sub(bl);
            loop {
                let fresh24 = unwrap!(jd.huffcode[id as usize][cls as usize].as_ref())
                    [hc_idx + jd.longofs[id as usize][cls as usize] as usize];
                hc_idx += 1;
                if d == fresh24 as u32 {
                    jd.dbit = wbit.wrapping_sub(bl) as u8;

                    return unwrap!((jd.huffdata[id as usize][cls as usize]).as_ref())
                        [hd_idx + jd.longofs[id as usize][cls as usize] as usize]
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
fn bitext(mut jd: &mut JDEC, nbit: u32) -> i32 {
    let mut dc: usize = jd.dctr;
    let mut dp: usize = jd.dptr;
    let mut d: u32;
    let mut flg: u32 = 0;
    let mut wbit: u32 = (jd.dbit as i32 % 32 as i32) as u32;
    let mut w: u32 = (jd.wreg as cty::c_ulong
        & ((1 as cty::c_ulong) << wbit).wrapping_sub(1 as i32 as cty::c_ulong))
        as u32;
    while wbit < nbit {
        if jd.marker != 0 {
            d = 0xff;
        } else {
            if dc == 0 {
                dp = 0;
                dc = jpeg_in_buffer(jd, Some(0), 512);
                if dc == 0 {
                    return 0 as i32 - JDR_INP as i32;
                }
            }
            d = unwrap!(jd.inbuf.as_ref())[dp] as u32;
            dp += 1;
            dc = dc.wrapping_sub(1);
            if flg != 0 {
                flg = 0;
                if d != 0 {
                    jd.marker = d as u8;
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
    jd.wreg = w;
    jd.dbit = wbit.wrapping_sub(nbit) as u8;
    jd.dctr = dc;
    jd.dptr = dp;

    return (w >> wbit.wrapping_sub(nbit).wrapping_rem(32)) as i32;
}
fn restart(mut jd: &mut JDEC, rstn: u16) -> JRESULT {
    let mut i: u32;
    let mut dp = jd.dptr;
    let mut dc: usize = jd.dctr;
    let mut marker: u16;
    if jd.marker != 0 {
        marker = (0xff00 | jd.marker as i32) as u16;
        jd.marker = 0;
    } else {
        marker = 0;
        i = 0;
        while i < 2 {
            if dc == 0 {
                dp = 0;
                dc = jpeg_in_buffer(jd, Some(0), 512);
                if dc == 0 {
                    return JDR_INP;
                }
            }
            let fresh27 = unwrap!(jd.inbuf.as_ref())[dp] as u32;
            dp += 1;
            marker = ((marker as i32) << 8 | fresh27 as i32) as u16;
            dc = dc.wrapping_sub(1);
            i = i.wrapping_add(1);
        }
        jd.dptr = dp;
        jd.dctr = dc;
    }
    if marker as i32 & 0xffd8 != 0xffd0 || marker as i32 & 7 != rstn as i32 & 7 {
        return JDR_FMT1;
    }
    jd.dbit = 0;
    jd.dcv[0] = 0;
    jd.dcv[1] = 0;
    jd.dcv[2] = 0;
    return JDR_OK;
}
fn block_idct(src: &mut &mut [i32], dst: &mut [i16]) {
    let M13: i32 = (1.41421f64 * 4096_f64) as i32;
    let M2: i32 = (1.08239f64 * 4096_f64) as i32;
    let M4: i32 = (2.61313f64 * 4096_f64) as i32;
    let M5: i32 = (1.84776f64 * 4096_f64) as i32;
    let mut v0: i32;
    let mut v1: i32;
    let mut v2: i32;
    let mut v3: i32;
    let mut v4: i32;
    let mut v5: i32;
    let mut v6: i32;
    let mut v7: i32;
    let mut t10: i32;
    let mut t11: i32;
    let mut t12: i32;
    let mut t13: i32;
    let mut dst_idx = 0;
    let mut src_idx = 0;
    let mut i = 0;
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

fn mcu_load(mut jd: &mut JDEC) -> JRESULT {
    let mut d: i32;
    let mut e: i32;
    let mut blk: u32;
    let mut i: u32;
    let mut bc: u32;
    let mut z: u32;
    let mut id: u32;
    let mut cmp: u32;
    let nby = (jd.msx as i32 * jd.msy as i32) as u32;
    let mut mcu_buf_idx = 0;
    blk = 0;
    while blk < nby.wrapping_add(2) {
        cmp = if blk < nby {
            0
        } else {
            blk.wrapping_sub(nby).wrapping_add(1)
        };
        if cmp != 0 && jd.ncomp as i32 != 3 {
            i = 0;
            while i < 64 {
                unwrap!(jd.mcubuf.as_mut())[mcu_buf_idx + i as usize] = 128 as i16;
                i += 1;
            }
        } else {
            id = if cmp != 0 { 1 } else { 0 };
            d = huffext(jd, id, 0);
            if d < 0 {
                return (0 - d) as JRESULT;
            }
            bc = d as u32;
            d = jd.dcv[cmp as usize] as i32;
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
                jd.dcv[cmp as usize] = d as i16;
            }
            let dfq = unwrap!(jd.qttbl[jd.qtid[cmp as usize] as usize].as_ref());
            unwrap!(jd.workbuf.as_mut())[0] = d * dfq[0] >> 8;
            unwrap!(jd.workbuf.as_mut())[1..64].fill(0);
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

                    let dfq = unwrap!(jd.qttbl[jd.qtid[cmp as usize] as usize].as_ref());
                    unwrap!(jd.workbuf.as_mut())[i as usize] = d * dfq[i as usize] >> 8 as i32;
                }
                z = z.wrapping_add(1);
                if !(z < 64) {
                    break;
                }
            }
            if 1 != 2 || cmp == 0 {
                if z == 1 || 0 != 0 && jd.scale == 3 {
                    d = (unwrap!(jd.workbuf.as_ref())[0] / 256 + 128) as i32;
                    if 2 >= 1 {
                        i = 0;
                        while i < 64 {
                            unwrap!(jd.mcubuf.as_mut())[mcu_buf_idx + i as usize] = d as i16;
                            i += 1;
                        }
                    } else {
                        unwrap!(jd.mcubuf.as_mut())[..64].fill(d as i16);
                    }
                } else {
                    block_idct(
                        unwrap!(jd.workbuf.as_mut()),
                        &mut unwrap!(jd.mcubuf.as_mut())[mcu_buf_idx..],
                    );
                }
            }
        }
        mcu_buf_idx += 64;
        blk = blk.wrapping_add(1);
    }
    return JDR_OK;
}
fn mcu_output(jd: &mut JDEC, x: u32, y: u32) -> JRESULT {
    let CVACC: i32 = if ::core::mem::size_of::<i32>() as cty::c_ulong > 2 as i32 as cty::c_ulong {
        1024
    } else {
        128
    };
    let mut ix: u32;
    let mut iy: u32;
    let mut mx: u32;
    let mut yy: i32;
    let mut cb: i32;
    let mut cr: i32;
    let mut py_idx: usize;
    let mut pc_idx: usize;
    let mut rect: JRECT = JRECT {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    mx = (jd.msx as i32 * 8) as u32;
    let my = (jd.msy as i32 * 8) as u32;
    let rx = if x.wrapping_add(mx) <= jd.width as u32 {
        mx
    } else {
        (jd.width as u32).wrapping_sub(x)
    };
    let ry = if y.wrapping_add(my) <= jd.height as u32 {
        my
    } else {
        (jd.height as u32).wrapping_sub(y)
    };
    rect.left = x as u16;
    rect.right = x.wrapping_add(rx).wrapping_sub(1) as u16;
    rect.top = y as u16;
    rect.bottom = y.wrapping_add(ry).wrapping_sub(1) as u16;

    let len = unwrap!(jd.workbuf.as_ref()).len() * 4;
    let ptr = unwrap!(jd.workbuf.as_mut()).as_mut_ptr() as *mut u8;
    let workbuf = unsafe { slice::from_raw_parts_mut(ptr, len) };

    let mut pix_idx: usize = 0;
    let mut op_idx: usize;

    if 0 == 0 || jd.scale != 3 {
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
                    cb = unwrap!(jd.mcubuf.as_mut())[pc_idx + 0] as i32 - 128;
                    cr = unwrap!(jd.mcubuf.as_mut())[pc_idx + 64] as i32 - 128;
                    if mx == 16 {
                        if ix == 8 {
                            py_idx += 64 - 8;
                        }
                        pc_idx += (ix & 1) as usize;
                    } else {
                        pc_idx += 1;
                    }
                    yy = unwrap!(jd.mcubuf.as_ref())[py_idx + 0] as i32;
                    py_idx += 1;

                    workbuf[pix_idx] = BYTECLIP(yy + (1.402f64 * CVACC as f64) as i32 * cr / CVACC);
                    pix_idx += 1;
                    workbuf[pix_idx] = BYTECLIP(
                        yy - ((0.344f64 * CVACC as f64) as i32 * cb
                            + (0.714f64 * CVACC as f64) as i32 * cr)
                            / CVACC,
                    );
                    pix_idx += 1;
                    workbuf[pix_idx] = BYTECLIP(yy + (1.772f64 * CVACC as f64) as i32 * cb / CVACC);
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
                    workbuf[pix_idx] = unwrap!(jd.mcubuf.as_ref())[py_idx] as u8;
                    pix_idx += 1;
                    py_idx += 1;
                    ix = ix.wrapping_add(1);
                }
                iy = iy.wrapping_add(1);
            }
        }
        if 0 != 0 && jd.scale != 0 {
            let mut x_0: u32;
            let mut y_0: u32;
            let mut r: u32;
            let mut g: u32;
            let mut b: u32;
            let s = (jd.scale as i32 * 2) as u32;
            let w = ((1 as i32) << jd.scale as i32) as u32;
            let a = mx.wrapping_sub(w).wrapping_mul(if 1 != 2 { 3 } else { 1 });
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
        cb = unwrap!(jd.mcubuf.as_ref())[pc_idx + 0] as i32 - 128;
        cr = unwrap!(jd.mcubuf.as_ref())[pc_idx + 64] as i32 - 128;
        iy = 0;
        while iy < my {
            py_idx = 0;
            if iy == 8 {
                py_idx = 64 * 2;
            }
            ix = 0;
            while ix < mx {
                yy = unwrap!(jd.mcubuf.as_ref())[py_idx] as i32;
                py_idx += 64;
                if JD_FORMAT != 2 {
                    workbuf[pix_idx] = BYTECLIP(yy + (1.402f64 * CVACC as f64) as i32 * cr / CVACC);
                    pix_idx += 1;
                    workbuf[pix_idx] = BYTECLIP(
                        yy - ((0.344f64 * CVACC as f64) as i32 * cb
                            + (0.714f64 * CVACC as f64) as i32 * cr)
                            / CVACC,
                    );
                    pix_idx += 1;
                    workbuf[pix_idx] = BYTECLIP(yy + (1.772f64 * CVACC as f64) as i32 * cb / CVACC);
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
    mx >>= jd.scale as i32;
    if rx < mx {
        let mut s_0_idx = 0;
        let mut d_idx = 0;
        let mut x_1: u32;
        let mut y_1: u32;
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
        let mut w_0: u16;
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
    return (if jpeg_out_buffer(jd, &mut rect) != 0 {
        JDR_OK as i32
    } else {
        JDR_INTR as i32
    }) as JRESULT;
}

pub fn jd_init<'a>(data: &'a [u8], buffer: &'static mut BufferJpeg, buffer_width: i16) -> JDEC<'a> {
    let pool = Some(unsafe { get_jpeg_work_buffer(0, true).buffer.as_mut_slice() });
    let pool_len = unwrap!(pool.as_ref()).len() as usize;

    JDEC {
        dctr: 0,
        dptr: 0,
        inbuf: None,
        dbit: 0,
        scale: 0,
        msx: 0,
        msy: 0,
        qtid: [0; 3],
        sz_pool: pool_len,
        pool,
        dcv: [0; 3],
        rsc: 0,
        width: 0,
        height: 0,
        huffbits: [[None, None], [None, None]],
        huffcode: [[None, None], [None, None]],
        huffdata: [[None, None], [None, None]],
        qttbl: [None, None, None, None],
        wreg: 0,
        marker: 0,
        longofs: [[0; 2]; 2],
        hufflut_ac: [None, None],
        hufflut_dc: [None, None],
        workbuf: None,
        rst: 0,
        data_len: data.len(),
        data,
        data_read: 0,
        buffer,
        buffer_width,
        buffer_height: 16,
        current_line: 0,
        current_line_pix: 0,
        ncomp: 0,
        nrst: 0,
        mcubuf: None,
        pool_start: 0,
    }
}

pub fn jd_prepare(mut jd: &mut JDEC) -> JRESULT {
    let mut b: u8;
    let mut marker: u16;
    let mut n: u32;
    let mut i: u32;
    let mut ofs: u32;
    let mut len: usize;
    let mut rc: JRESULT;

    let mem = unsafe { alloc_pool_slice(jd, 512) };
    if mem.is_err() {
        return JDR_MEM1;
    }
    jd.inbuf = Some(unwrap!(mem));

    marker = 0;
    ofs = marker as u32;
    loop {
        if jpeg_in_buffer(jd, Some(0), 1) != 1 {
            return JDR_INP;
        }
        ofs = ofs.wrapping_add(1);
        marker = ((marker as i32) << 8 | unwrap!(jd.inbuf.as_ref())[0] as i32) as u16;
        if !(marker as i32 != 0xffd8) {
            break;
        }
    }
    loop {
        if jpeg_in_buffer(jd, Some(0), 4) != 4 {
            return JDR_INP;
        }
        marker = ((unwrap!(jd.inbuf.as_ref())[0] as i32) << 8
            | unwrap!(jd.inbuf.as_ref())[1] as i32) as u16;
        len = ((unwrap!(jd.inbuf.as_ref())[2] as i32) << 8 | unwrap!(jd.inbuf.as_ref())[3] as i32)
            as usize;
        if len <= 2 || marker as i32 >> 8 != 0xff {
            return JDR_FMT1;
        }
        len = len.wrapping_sub(2);
        ofs = (ofs as usize).wrapping_add(4 + len) as u32;
        's_526: {
            let mut current_block_111: u64;
            match marker as i32 & 0xff {
                192 => {
                    if len > 512 {
                        return JDR_MEM2;
                    }
                    if jpeg_in_buffer(jd, Some(0), len) != len {
                        return JDR_INP;
                    }
                    jd.width = ((unwrap!(jd.inbuf.as_ref())[3] as i32) << 8
                        | unwrap!(jd.inbuf.as_ref())[4] as i32)
                        as u16;
                    jd.height = ((unwrap!(jd.inbuf.as_ref())[1] as i32) << 8 as i32
                        | unwrap!(jd.inbuf.as_ref())[2] as i32)
                        as u16;
                    jd.ncomp = unwrap!(jd.inbuf.as_ref())[5];
                    if jd.ncomp != 3 && jd.ncomp != 1 {
                        return JDR_FMT3;
                    }
                    i = 0;
                    while i < jd.ncomp as u32 {
                        b = unwrap!(jd.inbuf.as_ref())
                            [(7 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as usize];
                        if i == 0 {
                            if b != 0x11 && b != 0x22 && b != 0x21 {
                                return JDR_FMT3;
                            }
                            jd.msx = (b as i32 >> 4) as u8;
                            jd.msy = (b as i32 & 15) as u8;
                        } else if b as i32 != 0x11 {
                            return JDR_FMT3;
                        }
                        jd.qtid[i as usize] = unwrap!(jd.inbuf.as_ref())
                            [(8 as u32).wrapping_add((3 as u32).wrapping_mul(i)) as usize];
                        if jd.qtid[i as usize] as i32 > 3 {
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
                    if jpeg_in_buffer(jd, Some(0), len) != len {
                        return JDR_INP;
                    }
                    jd.nrst = ((unwrap!(jd.inbuf.as_ref())[0] as i32) << 8
                        | unwrap!(jd.inbuf.as_ref())[1] as i32)
                        as u16;
                    current_block_111 = 5265702136860997526;
                }
                196 => {
                    if len > 512 {
                        return JDR_MEM2;
                    }
                    if jpeg_in_buffer(jd, Some(0), len) != len {
                        return JDR_INP;
                    }
                    rc = create_huffman_tbl(jd, len);
                    if rc as u64 != 0 {
                        return rc;
                    }
                    current_block_111 = 5265702136860997526;
                }
                219 => {
                    if len > 512 {
                        return JDR_MEM2;
                    }
                    if jpeg_in_buffer(jd, Some(0), len) != len {
                        return JDR_INP;
                    }
                    rc = create_qt_tbl(jd, len);
                    if rc as u64 != 0 {
                        return rc;
                    }
                    current_block_111 = 5265702136860997526;
                }
                218 => {
                    if len > 512 {
                        return JDR_MEM2;
                    }
                    if jpeg_in_buffer(jd, Some(0), len) != len {
                        return JDR_INP;
                    }
                    if jd.width == 0 || jd.height == 0 {
                        return JDR_FMT1;
                    }
                    if unwrap!(jd.inbuf.as_ref())[0] as i32 != jd.ncomp as i32 {
                        return JDR_FMT3;
                    }
                    i = 0;
                    while i < jd.ncomp as u32 {
                        b = unwrap!(jd.inbuf.as_ref())
                            [(2 as u32).wrapping_add((2 as u32).wrapping_mul(i)) as usize];
                        if b != 0 && b != 0x11 {
                            return JDR_FMT3;
                        }
                        n = if i != 0 { 1 } else { 0 };
                        if (jd.huffbits[n as usize][0]).is_none()
                            || (jd.huffbits[n as usize][1]).is_none()
                        {
                            return JDR_FMT1;
                        }
                        if (jd.qttbl[jd.qtid[i as usize] as usize]).is_none() {
                            return JDR_FMT1;
                        }
                        i = i.wrapping_add(1);
                    }
                    n = (jd.msy as i32 * jd.msx as i32) as u32;
                    if n == 0 {
                        return JDR_FMT1;
                    }
                    len = n.wrapping_mul(64).wrapping_mul(2).wrapping_add(64) as usize;
                    if len < 256 {
                        len = 256;
                    }
                    let mem = unsafe { alloc_pool_slice(jd, len / 4) };
                    if mem.is_err() {
                        return JDR_MEM1;
                    }
                    jd.workbuf = Some(unwrap!(mem));

                    let mcubuf = unsafe { alloc_pool_slice(jd, (n as usize + 2) * 64) };
                    if mcubuf.is_err() {
                        return JDR_MEM1;
                    }
                    jd.mcubuf = Some(unwrap!(mcubuf));

                    ofs = ofs.wrapping_rem(512);
                    if ofs != 0 {
                        jd.dctr = jpeg_in_buffer(jd, Some(ofs as usize), (512 - ofs) as usize);
                    }
                    jd.dptr = (ofs - (if 2 != 0 { 0 } else { 1 })) as usize;
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
                    if jpeg_in_buffer(jd, None, len) != len {
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

pub fn jd_decomp(mut jd: &mut JDEC, scale: u8) -> JRESULT {
    if scale > (if 0 != 0 { 3 } else { 0 }) {
        return JDR_PAR;
    }
    jd.scale = scale;
    let mx = (jd.msx as i32 * 8) as u32;
    let my = (jd.msy as i32 * 8) as u32;
    let mut rc = JDR_OK;
    let mut y = 0;
    while y < jd.height as u32 {
        let mut x = 0;
        while x < jd.width as u32 {
            if jd.nrst as i32 != 0 && {
                let ref mut fresh68 = jd.rst;
                let fresh69 = *fresh68;
                *fresh68 = (*fresh68).wrapping_add(1);
                fresh69 as i32 == jd.nrst as i32
            } {
                let ref mut fresh70 = jd.rsc;
                let fresh71 = *fresh70;
                *fresh70 = (*fresh70).wrapping_add(1);
                rc = restart(jd, fresh71);
                if rc as u32 != JDR_OK as u32 {
                    return rc;
                }
                jd.rst = 1;
            }
            rc = mcu_load(jd);
            if rc as u32 != JDR_OK as u32 {
                return rc;
            }
            rc = mcu_output(jd, x, y);
            if rc as u32 != JDR_OK as u32 {
                return rc;
            }
            x = x.wrapping_add(mx);
        }
        y = y.wrapping_add(my);
    }
    return rc;
}

fn jpeg_in_buffer(jd: &mut JDEC, inbuf_offset: Option<usize>, n_data: usize) -> usize {
    let n_data = n_data as usize;
    if let Some(inbuf_offset) = inbuf_offset {
        if (jd.data_read + n_data) <= jd.data_len {
            let _ = &unwrap!(jd.inbuf.as_mut())[inbuf_offset..inbuf_offset + n_data]
                .copy_from_slice(&jd.data[jd.data_read..jd.data_read + n_data]);
        } else {
            let rest = jd.data_len - jd.data_read;

            if rest > 0 {
                let _ = &unwrap!(jd.inbuf.as_mut())[inbuf_offset..inbuf_offset + rest]
                    .copy_from_slice(&jd.data[jd.data_read..jd.data_read + rest]);
            } else {
                // error - no data
                return 0;
            }
        }
    }

    jd.data_read += n_data;
    n_data as _
}

fn jpeg_out_buffer(jd: &mut JDEC, rect: &mut JRECT) -> i32 {
    let w = (rect.right - rect.left + 1) as i16;
    let h = (rect.bottom - rect.top + 1) as i16;
    let x = rect.left as i16;

    let bitmap = unsafe {
        slice::from_raw_parts(
            unwrap!(jd.workbuf.as_ref()).as_ptr() as *const u16,
            (w * h) as usize,
        )
    };

    if h > jd.buffer_height {
        // unsupported height, call and let know
        return 1;
    }

    let buffer_len = (jd.buffer_width * jd.buffer_height) as usize;

    for i in 0..h {
        for j in 0..w {
            let buffer_pos = ((x + j) + (i * jd.buffer_width)) as usize;
            if buffer_pos < buffer_len {
                jd.buffer.buffer[buffer_pos] = bitmap[(i * w + j) as usize];
            }
        }
    }

    jd.current_line_pix += w;

    if jd.current_line_pix >= jd.buffer_width {
        jd.current_line_pix = 0;
        jd.current_line += (jd.msy * 8) as i16;
        // finished line, abort and continue later
        return 0;
    }

    1
}

pub fn jpeg_info(data: &[u8]) -> Result<JpegInfo, ()> {
    let work_buffer = unsafe { get_jpeg_buffer(0, true) };
    let mut jd: JDEC = jd_init(data, work_buffer, 0);
    let res = jd_prepare(&mut jd);

    let info = JpegInfo {
        width: jd.width,
        height: jd.height,
        mcu_height: (jd.msy * 8) as u16,
    };

    if info.mcu_height > 16 || res != JDR_OK {
        return Err(());
    }

    Ok(info)
}
