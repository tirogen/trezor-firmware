use crate::{
    trezorhal::{
        buffers::{get_jpeg_buffer, get_jpeg_work_buffer, BufferJpeg},
        display::pixeldata,
    },
    ui::{
        constant,
        constant::WIDTH,
        display::set_window,
        geometry::{Offset, Point, Rect},
    },
};
use core::{
    f64::consts::{FRAC_1_SQRT_2, SQRT_2},
    mem, slice,
};

const JD_FORMAT: u32 = 1;
const JD_USE_SCALE: u32 = 1;
const JD_FASTDECODE: u32 = 2;

const HUFF_BIT: u32 = 10;

const NUM_DEQUANTIZER_TABLES: usize = 4;

#[derive(PartialEq, Eq)]
pub enum JRESULT {
    FMT3 = 8,
    FMT2 = 7,
    FMT1 = 6,
    PAR = 5,
    MEM2 = 4,
    MEM1 = 3,
    INP = 2,
    INTR = 1,
    OK = 0,
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
    pub huffcode_len: [[usize; 2]; 2],
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
    pos: Option<Point>,
    data_read: usize,
    data_len: usize,
    buffer_width: i16,
    buffer_height: i16,
    current_line: i16,
    current_line_pix: i16,
    data: &'a [u8],
    pub buffer: &'static mut BufferJpeg,
}

static ZIG: [u8; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];
static IPFS: [u16; 64] = [
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
    (FRAC_1_SQRT_2 * 8192_f64) as u16,
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
    (FRAC_1_SQRT_2 * 8192_f64) as u16,
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

fn byte_clip(val: i32) -> u8 {
    if val < 0 {
        return 0;
    }
    if val > 255 {
        return 255;
    }
    val as u8
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
            jd.sz_pool -= ndata_aligned;
            return Ok(slice::from_raw_parts_mut(data.as_ptr() as *mut T, ndata));
        }
        Err(())
    }
}

fn create_qt_tbl(mut jd: &mut JDEC, mut ndata: usize) -> JRESULT {
    let mut i: u32;
    let mut d: u8;
    let mut data_idx = 0;
    while ndata != 0 {
        if ndata < 65 {
            return JRESULT::FMT1;
        }
        ndata -= 65;

        d = unwrap!(jd.inbuf.as_ref())[data_idx];
        data_idx += 1;
        if d & 0xf0 != 0 {
            return JRESULT::FMT1;
        }
        i = (d & 3) as u32;

        let pb = unsafe { alloc_pool_slice(jd, 64) };
        if pb.is_err() {
            return JRESULT::MEM1;
        }
        jd.qttbl[i as usize] = Some(unwrap!(pb));
        for zi in ZIG {
            unwrap!(jd.qttbl[i as usize].as_mut())[zi as usize] =
                ((unwrap!(jd.inbuf.as_ref())[data_idx] as u32) * IPFS[zi as usize] as u32) as i32;
            data_idx += 1;
        }
    }
    JRESULT::OK
}
fn create_huffman_tbl(mut jd: &mut JDEC, mut ndata: usize) -> JRESULT {
    let mut i: u32;
    let mut j: u32;
    let mut b: u32;
    let mut cls: usize;
    let mut num: usize;
    let mut np: usize;
    let mut d: u8;
    let mut hc: u16;
    let mut data_idx = 0;
    while ndata != 0 {
        if ndata < 17 {
            return JRESULT::FMT1;
        }
        ndata -= 17;
        d = unwrap!(jd.inbuf.as_ref())[data_idx];
        data_idx += 1;
        if d & 0xee != 0 {
            return JRESULT::FMT1;
        }
        cls = d as usize >> 4;
        num = d as usize & 0xf;
        let mem = unsafe { alloc_pool_slice(jd, 16) };
        if mem.is_err() {
            return JRESULT::MEM1;
        }
        jd.huffbits[num][cls] = Some(unwrap!(mem));

        i = 0;
        np = i as usize;
        while i < 16 {
            unwrap!(jd.huffbits[num][cls].as_mut())[i as usize] =
                unwrap!(jd.inbuf.as_ref())[data_idx];
            np += unwrap!(jd.inbuf.as_ref())[data_idx] as usize;
            data_idx += 1;
            i += 1;
        }
        let mem = unsafe { alloc_pool_slice(jd, np) };
        if mem.is_err() {
            return JRESULT::MEM1;
        }
        jd.huffcode[num][cls] = Some(unwrap!(mem));
        jd.huffcode_len[num][cls] = np;

        hc = 0;
        i = 0;
        j = i;
        while i < 16 {
            b = unwrap!(jd.huffbits[num][cls].as_ref())[i as usize] as u32;
            loop {
                let fresh10 = b;
                b -= 1;
                if fresh10 == 0 {
                    break;
                }
                let fresh11 = hc;
                hc += 1;
                let fresh12 = j;
                j += 1;
                unwrap!(jd.huffcode[num][cls].as_mut())[fresh12 as usize] = fresh11;
            }
            hc <<= 1;
            i += 1;
        }
        if ndata < np {
            return JRESULT::FMT1;
        }
        ndata -= np;
        let mem = unsafe { alloc_pool_slice(jd, np) };
        if mem.is_err() {
            return JRESULT::MEM1;
        }
        jd.huffdata[num][cls] = Some(unwrap!(mem));
        i = 0;
        while i < np as u32 {
            d = unwrap!(jd.inbuf.as_ref())[data_idx];
            data_idx += 1;
            if cls == 0 && d > 11 {
                return JRESULT::FMT1;
            }
            unwrap!(jd.huffdata[num][cls].as_mut())[i as usize] = d;
            i += 1;
        }
        if JD_FASTDECODE == 2 {
            let mut span: u32;
            let mut td: u32;
            let mut ti: u32;
            if cls != 0 {
                let tbl_ac = unsafe { alloc_pool_slice(jd, 1 << HUFF_BIT) };
                if tbl_ac.is_err() {
                    return JRESULT::MEM1;
                }
                jd.hufflut_ac[num] = Some(unwrap!(tbl_ac));
                unwrap!(jd.hufflut_ac[num].as_mut()).fill(0xffff);
            } else {
                let tbl_dc = unsafe { alloc_pool_slice(jd, 1 << HUFF_BIT) };
                if tbl_dc.is_err() {
                    return JRESULT::MEM1;
                }
                jd.hufflut_dc[num] = Some(unwrap!(tbl_dc));
                unwrap!(jd.hufflut_dc[num].as_mut()).fill(0xff);
            }
            b = 0;
            i = b;
            while b < HUFF_BIT {
                j = unwrap!(jd.huffbits[num][cls].as_ref())[b as usize] as u32;
                while j != 0 {
                    ti = ((unwrap!(jd.huffcode[num][cls].as_ref())[i as usize]
                        << (((HUFF_BIT - 1) as u32) - b))
                        & ((1 << HUFF_BIT) - 1)) as u32;

                    if cls != 0 {
                        td = unwrap!(jd.huffdata[num][cls].as_ref())[i as usize] as u32
                            | (b + 1) << 8;
                        i += 1;
                        span = 1 << ((HUFF_BIT - 1) - b);
                        while span != 0 {
                            span -= 1;
                            let fresh18 = ti;
                            ti += 1;
                            unwrap!(jd.hufflut_ac[num].as_mut())[fresh18 as usize] = td as u16;
                        }
                    } else {
                        td = unwrap!(jd.huffdata[num][cls].as_ref())[i as usize] as u32
                            | (b + 1) << 4;
                        i += 1;
                        span = 1 << ((HUFF_BIT - 1) - b);
                        while span != 0 {
                            span -= 1;
                            let fresh20 = ti;
                            ti += 1;
                            unwrap!(jd.hufflut_dc[num].as_mut())[fresh20 as usize] = td as u8;
                        }
                    }
                    j -= 1;
                }
                b += 1;
            }
            jd.longofs[num][cls] = i as u8;
        }
    }
    JRESULT::OK
}
fn huffext(mut jd: &mut JDEC, id: usize, cls: usize) -> Result<i32, JRESULT> {
    let mut dc: usize = jd.dctr;
    let mut dp: usize = jd.dptr;
    let mut d: u32;
    let mut flg: u32 = 0;
    let mut nc: u32;
    let mut bl: u32;
    let mut wbit: u32 = (jd.dbit as i32 % 32) as u32;
    let mut w: u32 = jd.wreg & ((1 << wbit) - 1);
    while wbit < 16 {
        if jd.marker != 0 {
            d = 0xff;
        } else {
            if dc == 0 {
                dp = 0;
                dc = jpeg_in(jd, Some(0), 512);
                if dc == 0 {
                    return Err(JRESULT::INP);
                }
            }
            d = unwrap!(jd.inbuf.as_mut())[dp] as u32;
            dp += 1;

            dc -= 1;
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
        w = w << 8 | d;
        wbit += 8;
    }
    jd.dctr = dc;
    jd.dptr = dp;
    jd.wreg = w;

    let mut hb_idx = 0;
    let mut hc_idx = 0;
    let mut hd_idx = 0;

    if JD_FASTDECODE == 2 {
        d = w >> (wbit - HUFF_BIT);
        if cls != 0 {
            d = unwrap!(jd.hufflut_ac[id].as_ref())[d as usize] as u32;
            if d != 0xffff {
                jd.dbit = (wbit - (d >> 8)) as u8;
                return Ok((d & 0xff) as i32);
            }
        } else {
            d = unwrap!(jd.hufflut_dc[id].as_ref())[d as usize] as u32;
            if d != 0xff {
                jd.dbit = (wbit - (d >> 4)) as u8;
                return Ok((d & 0xf) as i32);
            }
        }
        hb_idx = HUFF_BIT;
        hc_idx = jd.longofs[id][cls];
        hd_idx = jd.longofs[id][cls];
        bl = (HUFF_BIT + 1) as u32;
    } else {
        bl = 1;
    }

    while bl <= 16 {
        nc = unwrap!(jd.huffbits[id][cls].as_ref())[hb_idx as usize] as u32;
        hb_idx += 1;
        if nc != 0 {
            d = w >> (wbit - bl);
            loop {
                if hc_idx as usize >= jd.huffcode_len[id][cls] {
                    return Err(JRESULT::FMT1);
                }
                let fresh24 = unwrap!(jd.huffcode[id][cls].as_ref())[hc_idx as usize];
                hc_idx += 1;
                if d == fresh24 as u32 {
                    jd.dbit = (wbit - bl) as u8;
                    return Ok(unwrap!((jd.huffdata[id][cls]).as_ref())[hd_idx as usize] as i32);
                }
                hd_idx += 1;
                nc -= 1;
                if nc == 0 {
                    break;
                }
            }
        }
        bl += 1;
    }
    Err(JRESULT::FMT1)
}
fn bitext(mut jd: &mut JDEC, nbit: u32) -> Result<i32, JRESULT> {
    let mut dc: usize = jd.dctr;
    let mut dp: usize = jd.dptr;
    let mut d: u32;
    let mut flg: u32 = 0;
    let mut wbit: u32 = (jd.dbit as i32 % 32) as u32;
    let mut w: u32 = jd.wreg & ((1 << wbit) - 1);
    while wbit < nbit {
        if jd.marker != 0 {
            d = 0xff;
        } else {
            if dc == 0 {
                dp = 0;
                dc = jpeg_in(jd, Some(0), 512);
                if dc == 0 {
                    return Err(JRESULT::INP);
                }
            }
            d = unwrap!(jd.inbuf.as_ref())[dp] as u32;
            dp += 1;
            dc -= 1;
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
        w = w << 8 | d;
        wbit += 8;
    }
    jd.wreg = w;
    jd.dbit = (wbit - nbit) as u8;
    jd.dctr = dc;
    jd.dptr = dp;

    Ok((w >> ((wbit - nbit) % 32)) as i32)
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
                dc = jpeg_in(jd, Some(0), 512);
                if dc == 0 {
                    return JRESULT::INP;
                }
            }
            let fresh27 = unwrap!(jd.inbuf.as_ref())[dp] as u32;
            dp += 1;
            marker = ((marker as i32) << 8 | fresh27 as i32) as u16;
            dc -= 1;
            i += 1;
        }
        jd.dptr = dp;
        jd.dctr = dc;
    }
    if marker as i32 & 0xffd8 != 0xffd0 || marker as i32 & 7 != rstn as i32 & 7 {
        return JRESULT::FMT1;
    }
    jd.dbit = 0;
    jd.dcv[0] = 0;
    jd.dcv[1] = 0;
    jd.dcv[2] = 0;
    JRESULT::OK
}
fn block_idct(src: &mut &mut [i32], dst: &mut [i16]) {
    let m13: i32 = (SQRT_2 * 4096_f64) as i32;
    let m2: i32 = (1.08239f64 * 4096_f64) as i32;
    let m4: i32 = (2.61313f64 * 4096_f64) as i32;
    let m5: i32 = (1.84776f64 * 4096_f64) as i32;
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
    for idx in 0..8 {
        v0 = src[idx];
        v1 = src[idx + 8 * 2];
        v2 = src[idx + 8 * 4];
        v3 = src[idx + 8 * 6];
        t10 = v0 + v2;
        t12 = v0 - v2;
        t11 = ((v1 - v3) * m13) >> 12;
        v3 += v1;
        t11 -= v3;
        v0 = t10 + v3;
        v3 = t10 - v3;
        v1 = t11 + t12;
        v2 = t12 - t11;
        v4 = src[idx + 8 * 7];
        v5 = src[idx + 8];
        v6 = src[idx + 8 * 5];
        v7 = src[idx + 8 * 3];
        t10 = v5 - v4;
        t11 = v5 + v4;
        t12 = v6 - v7;
        v7 += v6;
        v5 = ((t11 - v7) * m13) >> 12;
        v7 += t11;
        t13 = ((t10 + t12) * m5) >> 12;
        v4 = t13 - ((t10 * m2) >> 12);
        v6 = t13 - ((t12 * m4) >> 12) - v7;
        v5 -= v6;
        v4 -= v5;
        src[idx] = v0 + v7;
        src[idx + 8 * 7] = v0 - v7;
        src[idx + 8] = v1 + v6;
        src[idx + 8 * 6] = v1 - v6;
        src[idx + 8 * 2] = v2 + v5;
        src[idx + 8 * 5] = v2 - v5;
        src[idx + 8 * 3] = v3 + v4;
        src[idx + 8 * 4] = v3 - v4;
    }
    for idx in (0..64).step_by(8) {
        v0 = src[idx] + (128 << 8);
        v1 = src[idx + 2];
        v2 = src[idx + 4];
        v3 = src[idx + 6];
        t10 = v0 + v2;
        t12 = v0 - v2;
        t11 = ((v1 - v3) * m13) >> 12;
        v3 += v1;
        t11 -= v3;
        v0 = t10 + v3;
        v3 = t10 - v3;
        v1 = t11 + t12;
        v2 = t12 - t11;
        v4 = src[idx + 7];
        v5 = src[idx + 1];
        v6 = src[idx + 5];
        v7 = src[idx + 3];
        t10 = v5 - v4;
        t11 = v5 + v4;
        t12 = v6 - v7;
        v7 += v6;
        v5 = ((t11 - v7) * m13) >> 12;
        v7 += t11;
        t13 = ((t10 + t12) * m5) >> 12;
        v4 = t13 - ((t10 * m2) >> 12);
        v6 = t13 - ((t12 * m4) >> 12) - v7;
        v5 -= v6;
        v4 -= v5;
        dst[idx] = ((v0 + v7) >> 8) as i16;
        dst[idx + 7] = ((v0 - v7) >> 8) as i16;
        dst[idx + 1] = ((v1 + v6) >> 8) as i16;
        dst[idx + 6] = ((v1 - v6) >> 8) as i16;
        dst[idx + 2] = ((v2 + v5) >> 8) as i16;
        dst[idx + 5] = ((v2 - v5) >> 8) as i16;
        dst[idx + 3] = ((v3 + v4) >> 8) as i16;
        dst[idx + 4] = ((v3 - v4) >> 8) as i16;
    }
}

fn mcu_load(mut jd: &mut JDEC) -> JRESULT {
    let mut d: i32;
    let mut e: i32;
    let mut blk: u32;
    let mut bc: u32;
    let mut z: u32;
    let mut id: u32;
    let mut cmp: u32;
    let nby = (jd.msx as i32 * jd.msy as i32) as u32;
    let mut mcu_buf_idx = 0;
    blk = 0;
    while blk < nby + 2 {
        cmp = if blk < nby { 0 } else { blk - nby + 1 };
        if cmp != 0 && jd.ncomp as i32 != 3 {
            for i in 0..64 {
                unwrap!(jd.mcubuf.as_mut())[mcu_buf_idx + i] = 128;
            }
        } else {
            id = if cmp != 0 { 1 } else { 0 };
            let res = huffext(jd, id as usize, 0);
            if let Ok(res) = res {
                d = res;
            } else {
                return res.err().unwrap();
            }
            bc = d as u32;
            d = jd.dcv[cmp as usize] as i32;
            if bc != 0 {
                let res = bitext(jd, bc);
                if let Ok(res) = res {
                    e = res;
                } else {
                    return res.err().unwrap();
                }
                bc = 1 << (bc - 1);
                if e as u32 & bc == 0 {
                    e -= ((bc << 1) - 1) as i32;
                }
                d += e;
                jd.dcv[cmp as usize] = d as i16;
            }
            let dqidx = jd.qtid[cmp as usize] as usize;
            if dqidx >= NUM_DEQUANTIZER_TABLES {
                return JRESULT::FMT1;
            }
            let dfq = unwrap!(jd.qttbl[dqidx].as_ref());
            unwrap!(jd.workbuf.as_mut())[0] = (d * dfq[0]) >> 8;
            unwrap!(jd.workbuf.as_mut())[1..64].fill(0);
            z = 1;
            loop {
                let res = huffext(jd, id as usize, 1);
                if let Ok(res) = res {
                    d = res;
                } else {
                    return res.err().unwrap();
                }
                if d == 0 {
                    break;
                }
                bc = d as u32;
                z += bc >> 4;
                if z >= 64 {
                    return JRESULT::FMT1;
                }
                bc &= 0xf;
                if bc != 0 {
                    let res = bitext(jd, bc);
                    if let Ok(res) = res {
                        d = res;
                    } else {
                        return res.err().unwrap();
                    }
                    bc = 1 << (bc - 1);
                    if d as u32 & bc == 0 {
                        d -= ((bc << 1) - 1) as i32;
                    }
                    let i = ZIG[z as usize] as u32;
                    let dqidx = jd.qtid[cmp as usize] as usize;
                    if dqidx >= NUM_DEQUANTIZER_TABLES {
                        return JRESULT::FMT1;
                    }
                    let dfq = unwrap!(jd.qttbl[dqidx].as_ref());
                    unwrap!(jd.workbuf.as_mut())[i as usize] = (d * dfq[i as usize]) >> 8;
                }
                z += 1;
                if z >= 64 {
                    break;
                }
            }
            if JD_FORMAT != 2 || cmp == 0 {
                if z == 1 || JD_USE_SCALE != 0 && jd.scale == 3 {
                    d = (unwrap!(jd.workbuf.as_ref())[0] / 256 + 128) as i32;
                    if JD_FASTDECODE >= 1 {
                        for i in 0..64 {
                            unwrap!(jd.mcubuf.as_mut())[mcu_buf_idx + i] = d as i16;
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
        blk += 1;
    }
    JRESULT::OK
}
fn mcu_output(jd: &mut JDEC, mut x: u32, mut y: u32) -> JRESULT {
    let cvacc: i32 = if mem::size_of::<i32>() > 2 { 1024 } else { 128 };
    let mut ix: u32;
    let mut iy: u32;
    let mut mx: u32;
    let mut yy: i32;
    let mut cb: i32;
    let mut cr: i32;
    let mut py_idx: usize;
    let mut pc_idx: usize;
    mx = (jd.msx as i32 * 8) as u32;
    let my = (jd.msy as i32 * 8) as u32;
    let mut rx = if (x + mx) <= jd.width as u32 {
        mx
    } else {
        jd.width as u32 - x
    };
    let mut ry = if (y + my) <= jd.height as u32 {
        my
    } else {
        jd.height as u32 - y
    };
    if JD_USE_SCALE != 0 {
        rx >>= jd.scale;
        ry >>= jd.scale;
        if rx == 0 || ry == 0 {
            /* Skip this MCU if all pixel is to be rounded off */
            return JRESULT::OK;
        }
        x >>= jd.scale;
        y >>= jd.scale;
    }
    let rect = Rect::from_top_left_and_size(
        Point::new(x as i16, y as i16),
        Offset::new(rx as i16, ry as i16),
    );
    let len = unwrap!(jd.workbuf.as_ref()).len() * 4;
    let ptr = unwrap!(jd.workbuf.as_mut()).as_mut_ptr() as *mut u8;
    let workbuf = unsafe { slice::from_raw_parts_mut(ptr, len) };

    let mut pix_idx: usize = 0;
    let mut op_idx: usize;

    if JD_USE_SCALE == 0 || jd.scale != 3 {
        if JD_FORMAT != 2 {
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
                    cb = unwrap!(jd.mcubuf.as_mut())[pc_idx] as i32 - 128;
                    cr = unwrap!(jd.mcubuf.as_mut())[pc_idx + 64] as i32 - 128;
                    if mx == 16 {
                        if ix == 8 {
                            py_idx += 64 - 8;
                        }
                        pc_idx += (ix & 1) as usize;
                    } else {
                        pc_idx += 1;
                    }
                    yy = unwrap!(jd.mcubuf.as_ref())[py_idx] as i32;
                    py_idx += 1;

                    workbuf[pix_idx] =
                        byte_clip(yy + (1.402f64 * cvacc as f64) as i32 * cr / cvacc);
                    pix_idx += 1;
                    workbuf[pix_idx] = byte_clip(
                        yy - ((0.344f64 * cvacc as f64) as i32 * cb
                            + (0.714f64 * cvacc as f64) as i32 * cr)
                            / cvacc,
                    );
                    pix_idx += 1;
                    workbuf[pix_idx] =
                        byte_clip(yy + (1.772f64 * cvacc as f64) as i32 * cb / cvacc);
                    pix_idx += 1;
                    ix += 1;
                }
                iy += 1;
            }
        } else {
            iy = 0;
            while iy < my {
                py_idx = (iy * 8) as usize;
                if my == 16 && iy >= 8 {
                    py_idx += 64;
                }
                ix = 0;
                while ix < mx {
                    if mx == 16 && ix == 8 {
                        py_idx += 64 - 8;
                    }
                    workbuf[pix_idx] = unwrap!(jd.mcubuf.as_ref())[py_idx] as u8;
                    pix_idx += 1;
                    py_idx += 1;
                    ix += 1;
                }
                iy += 1;
            }
        }
        if JD_USE_SCALE != 0 && jd.scale != 0 {
            let mut x_0: u32;
            let mut y_0: u32;
            let mut r: u32;
            let mut g: u32;
            let mut b: u32;
            let s = (jd.scale * 2) as u32;
            let w = 1 << jd.scale as u32;
            let a = (mx - w) * (if JD_FORMAT != 2 { 3 } else { 1 });
            op_idx = 0;
            iy = 0;
            while iy < my {
                ix = 0;
                while ix < mx {
                    pix_idx = ((iy * mx + ix) * (if JD_FORMAT != 2 { 3 } else { 1 })) as usize;
                    b = 0;
                    g = 0;
                    r = 0;
                    y_0 = 0;
                    while y_0 < w {
                        x_0 = 0;
                        while x_0 < w {
                            r += workbuf[pix_idx] as u32;
                            pix_idx += 1;
                            if JD_FORMAT != 2 {
                                g += workbuf[pix_idx] as u32;
                                pix_idx += 1;
                                b += workbuf[pix_idx] as u32;
                                pix_idx += 1;
                            }
                            x_0 += 1;
                        }
                        pix_idx += a as usize;
                        y_0 += 1;
                    }
                    workbuf[op_idx] = (r >> s) as u8;
                    op_idx += 1;
                    if JD_FORMAT != 2 {
                        workbuf[op_idx] = (g >> s) as u8;
                        op_idx += 1;
                        workbuf[op_idx] = (b >> s) as u8;
                        op_idx += 1;
                    }
                    ix += w;
                }
                iy += w;
            }
        }
    } else {
        pix_idx = 0;
        pc_idx = (mx * my) as usize;
        cb = unwrap!(jd.mcubuf.as_ref())[pc_idx] as i32 - 128;
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
                    workbuf[pix_idx] =
                        byte_clip(yy + (1.402f64 * cvacc as f64) as i32 * cr / cvacc);
                    pix_idx += 1;
                    workbuf[pix_idx] = byte_clip(
                        yy - ((0.344f64 * cvacc as f64) as i32 * cb
                            + (0.714f64 * cvacc as f64) as i32 * cr)
                            / cvacc,
                    );
                    pix_idx += 1;
                    workbuf[pix_idx] =
                        byte_clip(yy + (1.772f64 * cvacc as f64) as i32 * cb / cvacc);
                    pix_idx += 1;
                } else {
                    workbuf[pix_idx] = yy as u8;
                    pix_idx += 1;
                }
                ix += 8;
            }
            iy += 8;
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
                x_1 += 1;
            }
            s_0_idx += ((mx - rx) * (if JD_FORMAT != 2 { 3 } else { 1 })) as usize;

            y_1 += 1;
        }
    }
    if JD_FORMAT == 1 {
        let mut s_1_idx = 0;
        let mut d_0_idx = 0;
        let mut w_0: u16;
        for _ in 0..rx * ry {
            w_0 = ((workbuf[s_1_idx] as i32 & 0xf8) << 8) as u16;
            s_1_idx += 1;
            w_0 = (w_0 as i32 | (workbuf[s_1_idx] as i32 & 0xfc) << 3) as u16;
            s_1_idx += 1;
            w_0 = (w_0 as i32 | workbuf[s_1_idx] as i32 >> 3) as u16;
            s_1_idx += 1;

            workbuf[d_0_idx] = (w_0 & 0xFF) as u8;
            workbuf[d_0_idx + 1] = (w_0 >> 8) as u8;
            d_0_idx += 2;
        }
    }
    jpeg_out(jd, rect)
}

pub fn jd_init(data: &[u8]) -> JDEC {
    let pool = Some(unsafe { get_jpeg_work_buffer(0, true).buffer.as_mut_slice() });
    let pool_len = unwrap!(pool.as_ref()).len() as usize;

    let buffer = unsafe { get_jpeg_buffer(0, true) };

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
        huffcode_len: [[0; 2]; 2],
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
        buffer_width: WIDTH,
        buffer_height: 16,
        current_line: 0,
        current_line_pix: 0,
        ncomp: 0,
        nrst: 0,
        mcubuf: None,
        pool_start: 0,
        pos: None,
    }
}

pub fn jd_prepare(mut jd: &mut JDEC) -> JRESULT {
    let mut b: u8;
    let mut marker: u16;
    let mut n: u32;
    let mut i: u32;
    let mut ofs: u32;
    let mut len: usize;

    let mem = unsafe { alloc_pool_slice(jd, 512) };
    if mem.is_err() {
        return JRESULT::MEM1;
    }
    jd.inbuf = Some(unwrap!(mem));

    marker = 0;
    ofs = marker as u32;
    loop {
        if jpeg_in(jd, Some(0), 1) != 1 {
            return JRESULT::INP;
        }
        ofs += 1;
        marker = ((marker as i32) << 8 | unwrap!(jd.inbuf.as_ref())[0] as i32) as u16;
        if marker == 0xffd8 {
            break;
        }
    }
    loop {
        if jpeg_in(jd, Some(0), 4) != 4 {
            return JRESULT::INP;
        }
        marker = ((unwrap!(jd.inbuf.as_ref())[0] as i32) << 8
            | unwrap!(jd.inbuf.as_ref())[1] as i32) as u16;
        len = ((unwrap!(jd.inbuf.as_ref())[2] as i32) << 8 | unwrap!(jd.inbuf.as_ref())[3] as i32)
            as usize;
        if len <= 2 || marker >> 8 != 0xff {
            return JRESULT::FMT1;
        }
        len -= 2;
        ofs += (4 + len) as u32;

        match marker & 0xff {
            0xC0 => {
                if len > 512 {
                    return JRESULT::MEM2;
                }
                if jpeg_in(jd, Some(0), len) != len {
                    return JRESULT::INP;
                }
                jd.width = ((unwrap!(jd.inbuf.as_ref())[3] as i32) << 8
                    | unwrap!(jd.inbuf.as_ref())[4] as i32) as u16;
                jd.height = ((unwrap!(jd.inbuf.as_ref())[1] as i32) << 8
                    | unwrap!(jd.inbuf.as_ref())[2] as i32) as u16;
                jd.ncomp = unwrap!(jd.inbuf.as_ref())[5];
                if jd.ncomp != 3 && jd.ncomp != 1 {
                    return JRESULT::FMT3;
                }
                i = 0;
                while i < jd.ncomp as u32 {
                    b = unwrap!(jd.inbuf.as_ref())[(7 + 3 * i) as usize];
                    if i == 0 {
                        if b != 0x11 && b != 0x22 && b != 0x21 {
                            return JRESULT::FMT3;
                        }
                        jd.msx = (b as i32 >> 4) as u8;
                        jd.msy = (b as i32 & 15) as u8;
                    } else if b as i32 != 0x11 {
                        return JRESULT::FMT3;
                    }
                    jd.qtid[i as usize] = unwrap!(jd.inbuf.as_ref())[(8 + 3 * i) as usize];
                    if jd.qtid[i as usize] as i32 > 3 {
                        return JRESULT::FMT3;
                    }
                    i += 1;
                }
            }
            0xDD => {
                if len > 512 {
                    return JRESULT::MEM2;
                }
                if jpeg_in(jd, Some(0), len) != len {
                    return JRESULT::INP;
                }
                jd.nrst = ((unwrap!(jd.inbuf.as_ref())[0] as i32) << 8
                    | unwrap!(jd.inbuf.as_ref())[1] as i32) as u16;
            }
            0xC4 => {
                if len > 512 {
                    return JRESULT::MEM2;
                }
                if jpeg_in(jd, Some(0), len) != len {
                    return JRESULT::INP;
                }
                let res = create_huffman_tbl(jd, len);
                if res != JRESULT::OK {
                    return res;
                }
            }
            0xDB => {
                if len > 512 {
                    return JRESULT::MEM2;
                }
                if jpeg_in(jd, Some(0), len) != len {
                    return JRESULT::INP;
                }
                let res = create_qt_tbl(jd, len);
                if res != JRESULT::OK {
                    return res;
                }
            }
            0xDA => {
                if len > 512 {
                    return JRESULT::MEM2;
                }
                if jpeg_in(jd, Some(0), len) != len {
                    return JRESULT::INP;
                }
                if jd.width == 0 || jd.height == 0 {
                    return JRESULT::FMT1;
                }
                if unwrap!(jd.inbuf.as_ref())[0] as i32 != jd.ncomp as i32 {
                    return JRESULT::FMT3;
                }
                i = 0;
                while i < jd.ncomp as u32 {
                    b = unwrap!(jd.inbuf.as_ref())[(2 + 2 * i) as usize];
                    if b != 0 && b != 0x11 {
                        return JRESULT::FMT3;
                    }
                    n = if i != 0 { 1 } else { 0 };
                    if (jd.huffbits[n as usize][0]).is_none()
                        || (jd.huffbits[n as usize][1]).is_none()
                    {
                        return JRESULT::FMT1;
                    }
                    if (jd.qttbl[jd.qtid[i as usize] as usize]).is_none() {
                        return JRESULT::FMT1;
                    }
                    i += 1;
                }
                n = (jd.msy as i32 * jd.msx as i32) as u32;
                if n == 0 {
                    return JRESULT::FMT1;
                }
                len = (n * 64 * 3 + 64) as usize;
                if len < 256 {
                    len = 256;
                }
                let mem = unsafe { alloc_pool_slice(jd, len / 4) };
                if mem.is_err() {
                    return JRESULT::MEM1;
                }
                jd.workbuf = Some(unwrap!(mem));

                let mcubuf = unsafe { alloc_pool_slice(jd, (n as usize + 2) * 64) };
                if mcubuf.is_err() {
                    return JRESULT::MEM1;
                }
                jd.mcubuf = Some(unwrap!(mcubuf));

                ofs %= 512;
                if ofs != 0 {
                    jd.dctr = jpeg_in(jd, Some(ofs as usize), (512 - ofs) as usize);
                }
                jd.dptr = (ofs - (if JD_FASTDECODE != 0 { 0 } else { 1 })) as usize;
                return JRESULT::OK;
            }
            0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 | 0xC9 | 0xCA | 0xCB | 0xCD | 0xCF | 0xCE
            | 0xD9 => {
                return JRESULT::FMT3;
            }
            _ => {
                if jpeg_in(jd, None, len) != len {
                    return JRESULT::INP;
                }
            }
        }
    }
}

pub fn jd_decomp(mut jd: &mut JDEC, scale: u8) -> JRESULT {
    if scale > (if JD_USE_SCALE != 0 { 3 } else { 0 }) {
        return JRESULT::PAR;
    }
    jd.scale = scale;
    let mx = (jd.msx as i32 * 8) as u32;
    let my = (jd.msy as i32 * 8) as u32;
    let mut y = 0;
    while y < jd.height as u32 {
        let mut x = 0;
        while x < jd.width as u32 {
            if jd.nrst != 0 && {
                let val = jd.rst;
                jd.rst += 1;
                val == jd.nrst
            } {
                let val = jd.rsc;
                jd.rsc += 1;
                let rc = restart(jd, val);
                if rc != JRESULT::OK {
                    return rc;
                }
                jd.rst = 1;
            }
            let rc = mcu_load(jd);
            if rc != JRESULT::OK {
                return rc;
            }
            let rc = mcu_output(jd, x, y);
            if rc != JRESULT::OK {
                return rc;
            }
            x += mx;
        }
        y += my;
    }
    JRESULT::OK
}

fn jpeg_in(jd: &mut JDEC, inbuf_offset: Option<usize>, n_data: usize) -> usize {
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

fn jpeg_out(jd: &mut JDEC, rect: Rect) -> JRESULT {
    let w = rect.width();
    let h = rect.height();
    let x = rect.x0;

    let bitmap = unsafe {
        slice::from_raw_parts(
            unwrap!(jd.workbuf.as_ref()).as_ptr() as *const u16,
            (w * h) as usize,
        )
    };

    if let Some(pos) = jd.pos {
        let r = rect.translate(pos.into());
        let clamped = r.clamp(constant::screen());
        set_window(clamped);
        for py in r.y0..r.y1 {
            for px in r.x0..r.x1 {
                let p = Point::new(px, py);
                if clamped.contains(p) {
                    let off = p - r.top_left();
                    let c = bitmap[(off.y * w + off.x) as usize];
                    pixeldata(c);
                }
            }
        }
    } else {
        if h > jd.buffer_height {
            // unsupported height, call and let know
            return JRESULT::OK;
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

        if jd.current_line_pix >= jd.width as i16 {
            jd.current_line_pix = 0;
            jd.current_line += (jd.msy * 8) as i16;
            // finished line, abort and continue later
            return JRESULT::INTR;
        }
    }

    JRESULT::OK
}

pub fn jpeg(data: &[u8], pos: Point, scale: u8) {
    let mut jd: JDEC = jd_init(data);
    jd.pos = Some(pos);
    let res = jd_prepare(&mut jd);

    if res == JRESULT::OK {
        jd_decomp(&mut jd, scale);
    }
}

pub fn jpeg_info(data: &[u8]) -> Option<(Offset, u16)> {
    let mut jd: JDEC = jd_init(data);
    let res = jd_prepare(&mut jd);

    let mcu_height = jd.msy as u16 * 8;

    if mcu_height > 16 || res != JRESULT::OK {
        return None;
    }

    Some((Offset::new(jd.width as i16, jd.height as i16), mcu_height))
}
