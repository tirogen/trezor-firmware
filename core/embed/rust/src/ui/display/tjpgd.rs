/*----------------------------------------------------------------------------/
/ TJpgDec - Tiny JPEG Decompressor R0.03+trezor               (C)ChaN, 2021
/-----------------------------------------------------------------------------/
/ The TJpgDec is a generic JPEG decompressor module for tiny embedded systems.
/ This is a free software that opened for education, research and commercial
/  developments under license policy of following terms.
/
/  Copyright (C) 2021, ChaN, all right reserved.
/
/ * The TJpgDec module is a free software and there is NO WARRANTY.
/ * No restriction on use. You can use, modify and redistribute it for
/   personal, non-profit or commercial products UNDER YOUR RESPONSIBILITY.
/ * Redistributions of source code must retain the above copyright notice.
/
/-----------------------------------------------------------------------------/
/ Oct 04, 2011 R0.01  First release.
/ Feb 19, 2012 R0.01a Fixed decompression fails when scan starts with an escape seq.
/ Sep 03, 2012 R0.01b Added JD_TBLCLIP option.
/ Mar 16, 2019 R0.01c Supprted stdint.h.
/ Jul 01, 2020 R0.01d Fixed wrong integer type usage.
/ May 08, 2021 R0.02  Supprted grayscale image. Separated configuration options.
/ Jun 11, 2021 R0.02a Some performance improvement.
/ Jul 01, 2021 R0.03  Added JD_FASTDECODE option.
/                     Some performance improvement.
/ Jan 02, 2023        Rust version by Trezor Company, modified to meet our needs.

Trezor modifications:
 - included overflow detection from https://github.com/cmumford/TJpgDec
 - removed JD_FASTDECODE=0 option
 - removed JD_TBLCLIP option
 - allowed interrupted functionality
 - tighter integration into Trezor codebase by using our data structures
 - removed generic input and output functions, replaced by our specific functionality
/----------------------------------------------------------------------------*/

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

/// Specifies output pixel format.
///  0: RGB888 (24-bit/pix)
///  1: RGB565 (16-bit/pix)
///  2: Grayscale (8-bit/pix)
const JD_FORMAT: u32 = 1;

/// Switches output descaling feature.
/// 0: Disable
/// 1: Enable
const JD_USE_SCALE: u32 = 1;

/// Optimization level
/// 0: NOT IMPLEMENTED Basic optimization. Suitable for 8/16-bit MCUs.
/// 1: + 32-bit barrel shifter. Suitable for 32-bit MCUs.
/// 2: + Table conversion for huffman decoding (wants 6 << HUFF_BIT bytes of
/// RAM)
const JD_FASTDECODE: u32 = 2;

/// Specifies size of stream input buffer
const JD_SZBUF: usize = 512;

const HUFF_BIT: u32 = 10;
const HUFF_LEN: u32 = 1 << HUFF_BIT;
const HUFF_MASK: u32 = HUFF_LEN - 1;

const NUM_DEQUANTIZER_TABLES: usize = 4;

#[derive(PartialEq, Eq)]
pub enum Error {
    /// Interrupted by output function, call `jd_decomp` to continue.
    Interrupted,
    /// Device error or wrong termination of input stream.
    Input,
    /// Insufficient memory pool for the image.
    MemoryPool,
    /// Insufficient stream input buffer.
    MemoryInput,
    /// Parameter error.
    Parameter,
    /// Data format error (may be broken data).
    InvalidData,
    /// Not supported JPEG standard.
    UnsupportedJpeg,
}

pub struct JDEC<'i, 'o, 'w> {
    pub dctr: usize,
    pub dptr: usize,
    pub inbuf: &'w mut [u8],
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
    pub huffbits: [[&'w mut [u8]; 2]; 2],
    pub huffcode: [[&'w mut [u16]; 2]; 2],
    pub huffcode_len: [[usize; 2]; 2],
    pub huffdata: [[&'w mut [u8]; 2]; 2],
    pub qttbl: [&'w mut [i32]; 4],
    pub wreg: u32,
    pub marker: u8,
    pub longofs: [[u8; 2]; 2],
    pub hufflut_ac: [&'w mut [u16]; 2],
    pub hufflut_dc: [&'w mut [u8]; 2],
    pub workbuf: &'w mut [i32],
    pub mcubuf: &'w mut [i16],
    pub pool: &'w mut [u8],

    // context
    pos: Option<Point>,
    data_read: usize,
    data_len: usize,
    buffer_width: i16,
    buffer_height: i16,
    current_line: i16,
    current_line_pix: i16,
    data: &'i [u8],
    pub buffer: &'o mut BufferJpeg,
}

/// Zigzag-order to raster-order conversion table
#[rustfmt::skip]
const ZIG: [u8; 64] = [
     0,  1,  8, 16,  9,  2,  3, 10, 17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

macro_rules! f {
    ($num:expr) => {{
        ($num * 8192_f64) as u16
    }};
}

/// Input scale factor of Arai algorithm
/// (scaled up 16 bits for fixed point operations)
#[rustfmt::skip]
const IPSF: [u16; 64] = [
    f!(1.00000), f!(1.38704), f!(1.30656),       f!(1.17588), f!(1.00000), f!(0.78570), f!(0.54120),       f!(0.27590),
    f!(1.38704), f!(1.92388), f!(1.81226),       f!(1.63099), f!(1.38704), f!(1.08979), f!(0.75066),       f!(0.38268),
    f!(1.30656), f!(1.81226), f!(1.70711),       f!(1.53636), f!(1.30656), f!(1.02656), f!(FRAC_1_SQRT_2), f!(0.36048),
    f!(1.17588), f!(1.63099), f!(1.53636),       f!(1.38268), f!(1.17588), f!(0.92388), f!(0.63638),       f!(0.32442),
    f!(1.00000), f!(1.38704), f!(1.30656),       f!(1.17588), f!(1.00000), f!(0.78570), f!(0.54120),       f!(0.27590),
    f!(0.78570), f!(1.08979), f!(1.02656),       f!(0.92388), f!(0.78570), f!(0.61732), f!(0.42522),       f!(0.21677),
    f!(0.54120), f!(0.75066), f!(FRAC_1_SQRT_2), f!(0.63638), f!(0.54120), f!(0.42522), f!(0.29290),       f!(0.14932),
    f!(0.27590), f!(0.38268), f!(0.36048),       f!(0.32442), f!(0.27590), f!(0.21678), f!(0.14932),       f!(0.07612),
];

/// Allocate a memory block from memory pool
/// `jd`: decompressor object reference
/// `ndata` number of `T` items to allocate
fn alloc_pool_slice<'i, 'o, 'w, T>(
    jd: &mut JDEC<'i, 'o, 'w>,
    ndata: usize,
) -> Result<&'w mut [T], Error> {
    let ndata_bytes = ndata * mem::size_of::<T>();
    let ndata_aligned = (ndata_bytes + 3) & !3;
    if jd.pool.len() < ndata_aligned {
        // Err: not enough memory
        return Err(Error::MemoryPool);
    }

    // SAFETY:
    //  - Memory is valid because it comes from a valid slice.
    //  - Memory is initialized because here we consider integers always
    //    initialized.
    //  - The slices do not overlap and the original reference is overwritten,
    //    ensuring that the returned references are exclusive.
    unsafe {
        let data = slice::from_raw_parts_mut(jd.pool.as_mut_ptr() as _, ndata);
        let newpool = slice::from_raw_parts_mut(
            jd.pool.as_mut_ptr().add(ndata_aligned),
            jd.pool.len() - ndata_aligned,
        );
        jd.pool = newpool;
        Ok(data)
    }
}

/// Create de-quantization and prescaling tables with a DQT segment
/// `jd`: decompressor object reference
/// `ndata`: size of input data
fn create_qt_tbl(jd: &mut JDEC, mut ndata: usize) -> Result<(), Error> {
    let mut i: u32;
    let mut d: u8;
    let mut data_idx = 0;
    while ndata != 0 {
        // Process all tables in the segment
        if ndata < 65 {
            // Err: table size is unaligned
            return Err(Error::InvalidData);
        }
        ndata -= 65;

        d = jd.inbuf[data_idx]; // Get table property
        data_idx += 1;
        if d & 0xf0 != 0 {
            // Err: not 8-bit resolution
            return Err(Error::InvalidData);
        }
        i = (d & 3) as u32; // Get table ID

        // Allocate a memory block for the table
        // Register the table
        jd.qttbl[i as usize] = alloc_pool_slice(jd, 64)?;
        for zi in ZIG {
            // Load the table
            // Apply scale factor of Arai algorithm to the de-quantizers
            jd.qttbl[i as usize][zi as usize] =
                ((jd.inbuf[data_idx] as u32) * IPSF[zi as usize] as u32) as i32;
            data_idx += 1;
        }
    }
    Ok(())
}

/// Create huffman code tables with a DHT segment
/// `jd`: decompressor object reference
/// `ndata`: size of input data
fn create_huffman_tbl(jd: &mut JDEC, mut ndata: usize) -> Result<(), Error> {
    let mut j: u32;
    let mut b: u32;
    let mut cls: usize;
    let mut num: usize;
    let mut np: usize;
    let mut d: u8;
    let mut hc: u16;
    let mut data_idx = 0;
    while ndata != 0 {
        // Process all tables in the segment
        if ndata < 17 {
            // Err: wrong data size
            return Err(Error::InvalidData);
        }
        ndata -= 17;
        d = jd.inbuf[data_idx]; // Get table number and class
        data_idx += 1;
        if d & 0xee != 0 {
            // Err: invalid class/number
            return Err(Error::InvalidData);
        }
        cls = d as usize >> 4; // class = dc(0)/ac(1)
        num = d as usize & 0xf; // table number = 0/1
                                // Allocate a memory block for the bit distribution table
        jd.huffbits[num][cls] = alloc_pool_slice(jd, 16)?;

        np = 0;
        for i in 0..16 {
            // Load number of patterns for 1 to 16-bit code
            // Get sum of code words for each code
            jd.huffbits[num][cls][i] = jd.inbuf[data_idx];
            np += jd.inbuf[data_idx] as usize;
            data_idx += 1;
        }
        // Allocate a memory block for the code word table
        jd.huffcode[num][cls] = alloc_pool_slice(jd, np)?;
        jd.huffcode_len[num][cls] = np;

        // Re-build huffman code word table
        hc = 0;
        j = 0;
        for i in 0..16 {
            b = jd.huffbits[num][cls][i] as u32;
            while b > 0 {
                jd.huffcode[num][cls][j as usize] = hc;
                hc += 1;
                j += 1;
                b -= 1;
            }
            hc <<= 1;
        }
        if ndata < np {
            // Err: wrong data size
            return Err(Error::InvalidData);
        }
        ndata -= np;

        // Allocate a memory block for the decoded data
        jd.huffdata[num][cls] = alloc_pool_slice(jd, np)?;

        // Load decoded data corresponds to each code word
        for i in 0..np {
            d = jd.inbuf[data_idx];
            data_idx += 1;
            if cls == 0 && d > 11 {
                return Err(Error::InvalidData);
            }
            jd.huffdata[num][cls][i as usize] = d;
        }
        if JD_FASTDECODE == 2 {
            // Create fast huffman decode table
            let mut span: u32;
            let mut td: u32;
            let mut ti: u32;
            if cls != 0 {
                // LUT for AC elements
                jd.hufflut_ac[num] = alloc_pool_slice(jd, HUFF_LEN as usize)?;
                // Default value (0xFFFF: may be long code)
                jd.hufflut_ac[num].fill(0xffff);
            } else {
                // LUT for DC elements
                jd.hufflut_dc[num] = alloc_pool_slice(jd, HUFF_LEN as usize)?;
                // Default value (0xFF: may be long code)
                jd.hufflut_dc[num].fill(0xff);
            }
            let mut i = 0;

            // Create LUT
            for b in 0..HUFF_BIT {
                j = jd.huffbits[num][cls][b as usize] as u32;
                while j != 0 {
                    // Index of input pattern for the code
                    ti = (jd.huffcode[num][cls][i] << (((HUFF_BIT - 1) as u32) - b)) as u32
                        & HUFF_MASK;

                    if cls != 0 {
                        // b15..b8: code length, b7..b0: zero run and data length
                        td = jd.huffdata[num][cls][i] as u32 | (b + 1) << 8;
                        i += 1;
                        span = 1 << ((HUFF_BIT - 1) - b);
                        while span != 0 {
                            span -= 1;
                            jd.hufflut_ac[num][ti as usize] = td as u16;
                            ti += 1;
                        }
                    } else {
                        // b7..b4: code length, b3..b0: data length
                        td = jd.huffdata[num][cls][i] as u32 | (b + 1) << 4;
                        i += 1;
                        span = 1 << ((HUFF_BIT - 1) - b);
                        while span != 0 {
                            span -= 1;
                            jd.hufflut_dc[num][ti as usize] = td as u8;
                            ti += 1;
                        }
                    }
                    j -= 1;
                }
            }
            // Code table offset for long code
            jd.longofs[num][cls] = i as u8;
        }
    }
    Ok(())
}

/// Extract a huffman decoded data from input stream
/// `jd`: decompressor object reference
/// `id`: table ID (0:Y, 1:C)
/// `cls`: table class (0:DC, 1:AC)
#[optimize(speed)]
fn huffext(jd: &mut JDEC, id: usize, cls: usize) -> Result<i32, Error> {
    let mut dc: usize = jd.dctr;
    let mut dp: usize = jd.dptr;
    let mut d: u32;
    let mut flg: u32 = 0;
    let mut nc: u32;
    let mut bl: u32;
    let mut wbit: u32 = (jd.dbit as i32 % 32) as u32;
    let mut w: u32 = jd.wreg & ((1 << wbit) - 1);
    while wbit < 16 {
        // Prepare 16 bits into the working register
        if jd.marker != 0 {
            d = 0xff; // Input stream has stalled for a marker. Generate stuff
                      // bits
        } else {
            if dc == 0 {
                // Buffer empty, re-fill input buffer
                dp = 0; // Top of input buffer
                dc = jpeg_in(jd, Some(0), JD_SZBUF);
                if dc == 0 {
                    // Err: read error or wrong stream termination
                    return Err(Error::Input);
                }
            }
            d = jd.inbuf[dp] as u32;
            dp += 1;

            dc -= 1;
            if flg != 0 {
                // In flag sequence?
                flg = 0; // Exit flag sequence
                if d != 0 {
                    // Not an escape of 0xFF but a marker
                    jd.marker = d as u8;
                }
                d = 0xff;
            } else if d == 0xff {
                // Is start of flag sequence?
                // Enter flag sequence, get trailing byte
                flg = 1;
                continue;
            }
        }
        // Shift 8 bits in the working register
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
        // Table serch for the short codes
        d = w >> (wbit - HUFF_BIT); // Short code as table index
        if cls != 0 {
            // AC element
            d = jd.hufflut_ac[id][d as usize] as u32; // Table decode
            if d != 0xffff {
                // It is done if hit in short code
                jd.dbit = (wbit - (d >> 8)) as u8; // Snip the code length
                return Ok((d & 0xff) as i32); // b7..0: zero run and following
                                              // data bits
            }
        } else {
            // DC element
            d = jd.hufflut_dc[id][d as usize] as u32; // Table decode
            if d != 0xff {
                // It is done if hit in short code
                jd.dbit = (wbit - (d >> 4)) as u8; // Snip the code length
                return Ok((d & 0xf) as i32); // b3..0: following data bits
            }
        }

        // Incremental serch for the codes longer than HUFF_BIT
        hb_idx = HUFF_BIT; // Bit distribution table
        hc_idx = jd.longofs[id][cls]; // Code word table
        hd_idx = jd.longofs[id][cls]; // Data table
        bl = (HUFF_BIT + 1) as u32;
    } else {
        // Incremental search for all codes
        bl = 1;
    }

    // Incremental search
    while bl <= 16 {
        nc = jd.huffbits[id][cls][hb_idx as usize] as u32;
        hb_idx += 1;
        if nc != 0 {
            d = w >> (wbit - bl);
            loop {
                // Search the code word in this bit length
                if hc_idx as usize >= jd.huffcode_len[id][cls] {
                    return Err(Error::InvalidData);
                }
                let val = jd.huffcode[id][cls][hc_idx as usize];
                if d == val as u32 {
                    // Matched?
                    jd.dbit = (wbit - bl) as u8; // Snip the huffman code
                                                 // Return the decoded data
                    return Ok(jd.huffdata[id][cls][hd_idx as usize] as i32);
                }
                hc_idx += 1;
                hd_idx += 1;
                nc -= 1;
                if nc == 0 {
                    break;
                }
            }
        }
        bl += 1;
    }

    // Err: code not found (may be collapted data)
    Err(Error::InvalidData)
}

/// Extract N bits from input stream
/// `jd`: decompressor object reference
/// `nbit`: number of bits to extract (1 to 16)
#[optimize(speed)]
fn bitext(jd: &mut JDEC, nbit: u32) -> Result<i32, Error> {
    let mut dc: usize = jd.dctr;
    let mut dp: usize = jd.dptr;
    let mut d: u32;
    let mut flg: u32 = 0;
    let mut wbit: u32 = (jd.dbit as i32 % 32) as u32;
    let mut w: u32 = jd.wreg & ((1 << wbit) - 1);
    while wbit < nbit {
        // Prepare nbit bits into the working register
        if jd.marker != 0 {
            d = 0xff; // Input stream stalled, generate stuff bits
        } else {
            if dc == 0 {
                // Buffer empty, re-fill input buffer
                dp = 0; // Top of input buffer
                dc = jpeg_in(jd, Some(0), JD_SZBUF);
                if dc == 0 {
                    // Err: read error or wrong stream termination
                    return Err(Error::Input);
                }
            }
            d = jd.inbuf[dp] as u32;
            dp += 1;
            dc -= 1;
            if flg != 0 {
                // In flag sequence?
                flg = 0; // Exit flag sequence
                if d != 0 {
                    // Not an escape of 0xFF but a marker
                    jd.marker = d as u8;
                }
                d = 0xff;
            } else if d == 0xff {
                // Is start of flag sequence?
                flg = 1; // Enter flag sequence, get trailing byte
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

/// Process restart interval
/// `jd`: decompressor object reference
/// `rstn`: expected restart sequence number
#[optimize(speed)]
fn restart(jd: &mut JDEC, rstn: u16) -> Result<(), Error> {
    let mut dp = jd.dptr;
    let mut dc: usize = jd.dctr;
    let mut marker: u16;
    if jd.marker != 0 {
        // Generate a maker if it has been detected
        marker = 0xff00 | jd.marker as u16;
        jd.marker = 0;
    } else {
        marker = 0;
        for _ in 0..2 {
            // Get a restart marker
            if dc == 0 {
                // No input data is available, re-fill input buffer
                dp = 0;
                dc = jpeg_in(jd, Some(0), JD_SZBUF);
                if dc == 0 {
                    return Err(Error::Input);
                }
            }
            // Get a byte
            let b = jd.inbuf[dp] as u16;
            marker = marker << 8 | b;
            dp += 1;
            dc -= 1;
        }
        jd.dptr = dp;
        jd.dctr = dc;
    }

    // Check the marker
    if marker & 0xffd8 != 0xffd0 || marker & 7 != rstn & 7 {
        // Err: expected RSTn marker was not detected (may be collapted data)
        return Err(Error::InvalidData);
    }
    jd.dbit = 0; // Discard stuff bits
                 // Reset DC offset
    jd.dcv[0] = 0;
    jd.dcv[1] = 0;
    jd.dcv[2] = 0;
    Ok(())
}

/// Apply Inverse-DCT in Arai Algorithm
/// `src`: input block data (de-quantized and pre-scaled for Arai Algorithm)
/// `dst`: destination to store the block as byte array
#[optimize(speed)]
fn block_idct(src: &mut [i32], dst: &mut [i16]) {
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

    // Process columns
    for idx in 0..8 {
        // Get even elements
        v0 = src[idx];
        v1 = src[idx + 8 * 2];
        v2 = src[idx + 8 * 4];
        v3 = src[idx + 8 * 6];

        // Process the even elements
        t10 = v0 + v2;
        t12 = v0 - v2;
        t11 = ((v1 - v3) * m13) >> 12;
        v3 += v1;
        t11 -= v3;
        v0 = t10 + v3;
        v3 = t10 - v3;
        v1 = t11 + t12;
        v2 = t12 - t11;

        // Get odd elements
        v4 = src[idx + 8 * 7];
        v5 = src[idx + 8];
        v6 = src[idx + 8 * 5];
        v7 = src[idx + 8 * 3];

        // Process the odd elements
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

        // Write-back transformed values
        src[idx] = v0 + v7;
        src[idx + 8 * 7] = v0 - v7;
        src[idx + 8] = v1 + v6;
        src[idx + 8 * 6] = v1 - v6;
        src[idx + 8 * 2] = v2 + v5;
        src[idx + 8 * 5] = v2 - v5;
        src[idx + 8 * 3] = v3 + v4;
        src[idx + 8 * 4] = v3 - v4;
    }

    // Process rows
    for idx in (0..64).step_by(8) {
        // Get even elements
        v0 = src[idx] + (128 << 8); // remove DC offset (-128) here
        v1 = src[idx + 2];
        v2 = src[idx + 4];
        v3 = src[idx + 6];

        // Process the even elements
        t10 = v0 + v2;
        t12 = v0 - v2;
        t11 = ((v1 - v3) * m13) >> 12;
        v3 += v1;
        t11 -= v3;
        v0 = t10 + v3;
        v3 = t10 - v3;
        v1 = t11 + t12;
        v2 = t12 - t11;

        // Get odd elements
        v4 = src[idx + 7];
        v5 = src[idx + 1];
        v6 = src[idx + 5];
        v7 = src[idx + 3];

        // Process the odd elements
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

        // Descale the transformed values 8 bits and output a row
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

/// Load all blocks in an MCU into working buffer
/// `jd`: decompressor object reference
#[optimize(speed)]
fn mcu_load(jd: &mut JDEC) -> Result<(), Error> {
    let mut d: i32;
    let mut e: i32;
    let mut blk: u32;
    let mut bc: u32;
    let mut z: u32;
    let mut id: u32;
    let mut cmp: u32;
    let nby = (jd.msx as i32 * jd.msy as i32) as u32; // Number of Y blocks (1, 2 or 4)
    let mut mcu_buf_idx = 0; // Pointer to the first block of MCU
    blk = 0;
    while blk < nby + 2 {
        // Get nby Y blocks and two C blocks
        cmp = if blk < nby { 0 } else { blk - nby + 1 }; // Component number 0:Y, 1:Cb, 2:Cr
        if cmp != 0 && jd.ncomp as i32 != 3 {
            // Clear C blocks if not exist (monochrome image)
            for i in 0..64 {
                jd.mcubuf[mcu_buf_idx + i] = 128;
            }
        } else {
            // Load Y/C blocks from input stream
            id = if cmp != 0 { 1 } else { 0 }; // Huffman table ID of this component

            // Extract a DC element from input stream
            d = huffext(jd, id as usize, 0)?; // Extract a huffman coded data (bit length)
            bc = d as u32;
            d = jd.dcv[cmp as usize] as i32; // DC value of previous block
            if bc != 0 {
                // If there is any difference from previous block
                e = bitext(jd, bc)?; // Extract data bits
                bc = 1 << (bc - 1); // MSB position
                if e as u32 & bc == 0 {
                    e -= ((bc << 1) - 1) as i32; // Restore negative value if
                                                 // needed
                }
                d += e; // Get current value
                jd.dcv[cmp as usize] = d as i16; // Save current DC value for
                                                 // next block
            }
            // De-quantizer table ID for this component
            let dqidx = jd.qtid[cmp as usize] as usize;
            if dqidx >= NUM_DEQUANTIZER_TABLES {
                return Err(Error::InvalidData);
            }
            // De-quantize, apply scale factor of Arai algorithm and descale 8 bits
            let dfq = &jd.qttbl[dqidx];
            jd.workbuf[0] = (d * dfq[0]) >> 8;

            // Extract following 63 AC elements from input stream
            jd.workbuf[1..64].fill(0); // Initialize all AC elements
            z = 1; // Top of the AC elements (in zigzag-order)
            loop {
                // Extract a huffman coded value (zero runs and bit length)
                d = huffext(jd, id as usize, 1)?;
                if d == 0 {
                    // EOB?
                    break;
                }
                bc = d as u32;
                z += bc >> 4; // Skip leading zero run
                if z >= 64 {
                    // Too long zero run
                    return Err(Error::InvalidData);
                }
                bc &= 0xf;
                if bc != 0 {
                    // Bit length?
                    d = bitext(jd, bc)?; // Extract data bits
                    bc = 1 << (bc - 1); // MSB position
                    if d as u32 & bc == 0 {
                        // Restore negative value if needed
                        d -= ((bc << 1) - 1) as i32;
                    }
                    let i = ZIG[z as usize] as u32; // Get raster-order index
                                                    // De-quantize, apply scale factor of Arai algorithm and descale 8 bits
                    let dqidx = jd.qtid[cmp as usize] as usize;
                    if dqidx >= NUM_DEQUANTIZER_TABLES {
                        return Err(Error::InvalidData);
                    }
                    let dfq = &jd.qttbl[dqidx];
                    jd.workbuf[i as usize] = (d * dfq[i as usize]) >> 8;
                }
                z += 1;
                if z >= 64 {
                    break;
                }
            }

            // C components may not be processed if in grayscale output
            if JD_FORMAT != 2 || cmp == 0 {
                // If no AC element or scale ratio is 1/8, IDCT can be omitted and the block is
                // filled with DC value
                if z == 1 || JD_USE_SCALE != 0 && jd.scale == 3 {
                    d = (jd.workbuf[0] / 256 + 128) as i32;
                    if JD_FASTDECODE >= 1 {
                        for i in 0..64 {
                            jd.mcubuf[mcu_buf_idx + i] = d as i16;
                        }
                    } else {
                        jd.mcubuf[..64].fill(d as i16);
                    }
                } else {
                    // Apply IDCT and store the block to the MCU buffer
                    block_idct(jd.workbuf, &mut jd.mcubuf[mcu_buf_idx..]);
                }
            }
        }
        mcu_buf_idx += 64; // Next block
        blk += 1;
    }
    Ok(()) // All blocks have been loaded successfully
}

/// Output an MCU: Convert YCrCb to RGB and output it in RGB form
/// `jd`: decompressor object reference
/// `x`: MCU location in the image
/// `y`: MCU location in the image
#[optimize(speed)]
fn mcu_output(jd: &mut JDEC, mut x: u32, mut y: u32) -> Result<(), Error> {
    // Adaptive accuracy for both 16-/32-bit systems
    let cvacc: i32 = if mem::size_of::<i32>() > 2 { 1024 } else { 128 };
    let mut yy: i32;
    let mut cb: i32;
    let mut cr: i32;
    let mut py_idx: usize;
    let mut pc_idx: usize;

    // MCU size (pixel)
    let mut mx = (jd.msx as i32 * 8) as u32;
    let my = (jd.msy as i32 * 8) as u32;

    // Output rectangular size (it may be clipped at right/bottom end of image)
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
            // Skip this MCU if all pixel is to be rounded off
            return Ok(());
        }
        x >>= jd.scale;
        y >>= jd.scale;
    }
    let rect = Rect::from_top_left_and_size(
        Point::new(x as i16, y as i16),
        Offset::new(rx as i16, ry as i16),
    );
    let workbuf: &mut [u8] = reinterpret_slice(jd.workbuf);

    let mut pix_idx: usize = 0;
    let mut op_idx: usize;

    if JD_USE_SCALE == 0 || jd.scale != 3 {
        // Not for 1/8 scaling
        if JD_FORMAT != 2 {
            // RGB output (build an RGB MCU from Y/C component)
            for iy in 0..my {
                py_idx = 0;
                pc_idx = 0;
                if my == 16 {
                    // Double block height?
                    pc_idx += (64 * 4) + ((iy as usize >> 1) * 8);
                    if iy >= 8 {
                        py_idx += 64;
                    }
                } else {
                    // Single block height
                    pc_idx += (mx * 8 + iy * 8) as usize;
                }
                py_idx += (iy * 8) as usize;
                for ix in 0..mx {
                    cb = jd.mcubuf[pc_idx] as i32 - 128; // Get Cb/Cr component and remove offset
                    cr = jd.mcubuf[pc_idx + 64] as i32 - 128;
                    if mx == 16 {
                        // Double block width?
                        if ix == 8 {
                            // Jump to next block if double block height
                            py_idx += 64 - 8;
                        }
                        // Step forward chroma pointer every two pixels
                        pc_idx += (ix & 1) as usize;
                    } else {
                        // Single block width
                        // Step forward chroma pointer every pixel
                        pc_idx += 1;
                    }
                    // Get Y component
                    yy = jd.mcubuf[py_idx] as i32;
                    py_idx += 1;
                    // R
                    workbuf[pix_idx] =
                        (yy + (1.402f64 * cvacc as f64) as i32 * cr / cvacc).clamp(0, 255) as u8;
                    pix_idx += 1;
                    // G
                    workbuf[pix_idx] = (yy
                        - ((0.344f64 * cvacc as f64) as i32 * cb
                            + (0.714f64 * cvacc as f64) as i32 * cr)
                            / cvacc)
                        .clamp(0, 255) as u8;
                    pix_idx += 1;
                    // B
                    workbuf[pix_idx] =
                        (yy + (1.772f64 * cvacc as f64) as i32 * cb / cvacc).clamp(0, 255) as u8;
                    pix_idx += 1;
                }
            }
        } else {
            // Monochrome output (build a grayscale MCU from Y comopnent)

            for iy in 0..my {
                py_idx = (iy * 8) as usize;
                if my == 16 && iy >= 8 {
                    // Double block height?
                    py_idx += 64;
                }
                for ix in 0..mx {
                    if mx == 16 && ix == 8 {
                        // Double block width?
                        // Jump to next block if double block height
                        py_idx += 64 - 8;
                    }
                    // Get and store a Y value as grayscale
                    workbuf[pix_idx] = jd.mcubuf[py_idx] as u8;
                    pix_idx += 1;
                    py_idx += 1;
                }
            }
        }
        // Descale the MCU rectangular if needed
        if JD_USE_SCALE != 0 && jd.scale != 0 {
            // Get averaged RGB value of each square corresponds to a pixel
            let s = (jd.scale * 2) as u32; // Number of shifts for averaging
            let w = 1 << jd.scale as u32; // Width of square
            let a = (mx - w) * (if JD_FORMAT != 2 { 3 } else { 1 }); // Bytes to skip for next line in the square
            op_idx = 0;
            for iy in (0..my).step_by(w as usize) {
                for ix in (0..mx).step_by(w as usize) {
                    pix_idx = ((iy * mx + ix) * (if JD_FORMAT != 2 { 3 } else { 1 })) as usize;
                    let mut b = 0;
                    let mut g = 0;
                    let mut r = 0;
                    for _ in 0..w {
                        // Accumulate RGB value in the square
                        for _ in 0..w {
                            // Accumulate R or Y (monochrome output)
                            r += workbuf[pix_idx] as u32;
                            pix_idx += 1;
                            if JD_FORMAT != 2 {
                                // Accumulate G
                                g += workbuf[pix_idx] as u32;
                                pix_idx += 1;
                                // Accumulate B
                                b += workbuf[pix_idx] as u32;
                                pix_idx += 1;
                            }
                        }
                        pix_idx += a as usize;
                    }
                    // Put the averaged pixel value
                    // Put R or Y (monochrome output)
                    workbuf[op_idx] = (r >> s) as u8;
                    op_idx += 1;
                    if JD_FORMAT != 2 {
                        // RGB output?
                        // Put G
                        workbuf[op_idx] = (g >> s) as u8;
                        op_idx += 1;
                        // Put B
                        workbuf[op_idx] = (b >> s) as u8;
                        op_idx += 1;
                    }
                }
            }
        }
    } else {
        // For only 1/8 scaling (left-top pixel in each block are the DC value of the
        // block) Build a 1/8 descaled RGB MCU from discrete components
        pix_idx = 0;
        pc_idx = (mx * my) as usize;
        cb = jd.mcubuf[pc_idx] as i32 - 128; // Get Cb/Cr component and restore right level
        cr = jd.mcubuf[pc_idx + 64] as i32 - 128;

        for iy in (0..my).step_by(8) {
            py_idx = 0;
            if iy == 8 {
                py_idx = 64 * 2;
            }
            for _ in (0..mx).step_by(8) {
                // Get Y component
                yy = jd.mcubuf[py_idx] as i32;
                py_idx += 64;
                if JD_FORMAT != 2 {
                    // R
                    workbuf[pix_idx] =
                        (yy + (1.402f64 * cvacc as f64) as i32 * cr / cvacc).clamp(0, 255) as u8;
                    pix_idx += 1;
                    // G
                    workbuf[pix_idx] = (yy
                        - ((0.344f64 * cvacc as f64) as i32 * cb
                            + (0.714f64 * cvacc as f64) as i32 * cr)
                            / cvacc)
                        .clamp(0, 255) as u8;
                    //B
                    pix_idx += 1;
                    workbuf[pix_idx] =
                        (yy + (1.772f64 * cvacc as f64) as i32 * cb / cvacc).clamp(0, 255) as u8;
                    pix_idx += 1;
                } else {
                    workbuf[pix_idx] = yy as u8;
                    pix_idx += 1;
                }
            }
        }
    }

    // Squeeze up pixel table if a part of MCU is to be truncated
    mx >>= jd.scale as i32;
    if rx < mx {
        // Is the MCU spans right edge?
        let mut s_0_idx = 0;
        let mut d_idx = 0;
        for _ in 0..ry {
            for _ in 0..rx {
                // Copy effective pixels
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
            }
            // Skip truncated pixels
            s_0_idx += ((mx - rx) * (if JD_FORMAT != 2 { 3 } else { 1 })) as usize;
        }
    }

    // Convert RGB888 to RGB565 if needed
    if JD_FORMAT == 1 {
        let mut s_1_idx = 0;
        let mut d_0_idx = 0;
        let mut w_0: u16;
        for _ in 0..rx * ry {
            // RRRRR-----------
            w_0 = ((workbuf[s_1_idx] as i32 & 0xf8) << 8) as u16;
            s_1_idx += 1;
            // -----GGGGGG-----
            w_0 = (w_0 as i32 | (workbuf[s_1_idx] as i32 & 0xfc) << 3) as u16;
            s_1_idx += 1;
            // -----------BBBBB
            w_0 = (w_0 as i32 | workbuf[s_1_idx] as i32 >> 3) as u16;
            s_1_idx += 1;

            workbuf[d_0_idx] = (w_0 & 0xFF) as u8;
            workbuf[d_0_idx + 1] = (w_0 >> 8) as u8;
            d_0_idx += 2;
        }
    }

    // Output the rectangular
    jpeg_out(jd, rect)
}

fn reinterpret_slice<T>(slice: &mut [i32]) -> &mut [T] {
    let len = slice.len() * mem::size_of_val(&slice[0]) / mem::size_of::<T>();
    let ptr = slice.as_mut_ptr() as _;
    // SAFETY: Memory is valid because it comes from the backing slice. We only
    // convert between integer types which are always initialized.
    unsafe { slice::from_raw_parts_mut(ptr, len) }
}

pub fn jd_init(data: &[u8]) -> JDEC {
    let pool = unsafe { get_jpeg_work_buffer(0, true).buffer.as_mut_slice() };

    let buffer = unsafe { get_jpeg_buffer(0, true) };

    JDEC {
        dctr: 0,
        dptr: 0,
        inbuf: &mut [],
        dbit: 0,
        scale: 0,
        msx: 0,
        msy: 0,
        qtid: [0; 3],
        pool,
        dcv: [0; 3],
        rsc: 0,
        width: 0,
        height: 0,
        huffbits: [[&mut [], &mut []], [&mut [], &mut []]],
        huffcode: [[&mut [], &mut []], [&mut [], &mut []]],
        huffcode_len: [[0; 2]; 2],
        huffdata: [[&mut [], &mut []], [&mut [], &mut []]],
        qttbl: [&mut [], &mut [], &mut [], &mut []],
        wreg: 0,
        marker: 0,
        longofs: [[0; 2]; 2],
        hufflut_ac: [&mut [], &mut []],
        hufflut_dc: [&mut [], &mut []],
        workbuf: &mut [],
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
        mcubuf: &mut [],
        pos: None,
    }
}

/// Analyze the JPEG image and Initialize decompressor object
pub fn jd_prepare(jd: &mut JDEC) -> Result<(), Error> {
    let mut marker: u16;
    let mut ofs: u32;
    let mut len: usize;

    // Allocate stream input buffer
    jd.inbuf = alloc_pool_slice(jd, JD_SZBUF)?;

    // Find SOI marker
    marker = 0;
    ofs = marker as u32;
    loop {
        if jpeg_in(jd, Some(0), 1) != 1 {
            // Err: SOI was not detected
            return Err(Error::Input);
        }
        ofs += 1;
        marker = ((marker as i32) << 8 | jd.inbuf[0] as i32) as u16;
        if marker == 0xffd8 {
            break;
        }
    }
    loop {
        // Parse JPEG segments
        // Get a JPEG marker
        if jpeg_in(jd, Some(0), 4) != 4 {
            return Err(Error::Input);
        }
        // Marker
        marker = ((jd.inbuf[0] as i32) << 8 | jd.inbuf[1] as i32) as u16;
        // Length field
        len = ((jd.inbuf[2] as i32) << 8 | jd.inbuf[3] as i32) as usize;
        if len <= 2 || marker >> 8 != 0xff {
            return Err(Error::InvalidData);
        }
        len -= 2; // Segment content size
        ofs += (4 + len) as u32; // Number of bytes loaded

        match marker & 0xff {
            0xC0 => {
                // SOF0 (baseline JPEG)
                if len > JD_SZBUF {
                    return Err(Error::MemoryInput);
                }
                // Load segment data
                if jpeg_in(jd, Some(0), len) != len {
                    return Err(Error::Input);
                }
                // Image width in unit of pixel
                jd.width = ((jd.inbuf[3] as i32) << 8 | jd.inbuf[4] as i32) as u16;
                // Image height in unit of pixel
                jd.height = ((jd.inbuf[1] as i32) << 8 | jd.inbuf[2] as i32) as u16;
                // Number of color components
                jd.ncomp = jd.inbuf[5];
                if jd.ncomp != 3 && jd.ncomp != 1 {
                    // Err: Supports only Grayscale and Y/Cb/Cr
                    return Err(Error::UnsupportedJpeg);
                }
                // Check each image component
                for i in 0..jd.ncomp as usize {
                    // Get sampling factor
                    let b = jd.inbuf[7 + 3 * i];
                    if i == 0 {
                        // Y component
                        if b != 0x11 && b != 0x22 && b != 0x21 {
                            // Check sampling factor
                            // Err: Supports only 4:4:4, 4:2:0 or 4:2:2
                            return Err(Error::UnsupportedJpeg);
                        }
                        // Size of MCU [blocks]
                        jd.msx = (b as i32 >> 4) as u8;
                        jd.msy = (b as i32 & 15) as u8;
                    } else if b as i32 != 0x11 {
                        // Cb/Cr component
                        // Err: Sampling factor of Cb/Cr must be 1
                        return Err(Error::UnsupportedJpeg);
                    }
                    // Get dequantizer table ID for this component
                    jd.qtid[i] = jd.inbuf[8 + 3 * i];
                    if jd.qtid[i] as i32 > 3 {
                        // Err: Invalid ID
                        return Err(Error::UnsupportedJpeg);
                    }
                }
            }
            0xDD => {
                // DRI - Define Restart Interval
                if len > JD_SZBUF {
                    return Err(Error::MemoryInput);
                }
                // Load segment data
                if jpeg_in(jd, Some(0), len) != len {
                    return Err(Error::Input);
                }
                // Get restart interval (MCUs)
                jd.nrst = ((jd.inbuf[0] as i32) << 8 | jd.inbuf[1] as i32) as u16;
            }
            0xC4 => {
                // DHT - Define Huffman Tables
                if len > JD_SZBUF {
                    return Err(Error::MemoryInput);
                }
                // Load segment data
                if jpeg_in(jd, Some(0), len) != len {
                    return Err(Error::Input);
                }
                // Create huffman tables
                create_huffman_tbl(jd, len)?;
            }
            0xDB => {
                // DQT - Define Quantizer Tables
                if len > JD_SZBUF {
                    return Err(Error::MemoryInput);
                }
                // Load segment data
                if jpeg_in(jd, Some(0), len) != len {
                    return Err(Error::Input);
                }
                // Create de-quantizer tables
                create_qt_tbl(jd, len)?;
            }
            0xDA => {
                // SOS - Start of Scan
                if len > JD_SZBUF {
                    return Err(Error::MemoryInput);
                }
                // Load segment data
                if jpeg_in(jd, Some(0), len) != len {
                    return Err(Error::Input);
                }
                if jd.width == 0 || jd.height == 0 {
                    // Err: Invalid image size
                    return Err(Error::InvalidData);
                }
                if jd.inbuf[0] as i32 != jd.ncomp as i32 {
                    // Err: Wrong color components
                    return Err(Error::UnsupportedJpeg);
                }
                // Check if all tables corresponding to each components have been loaded
                for i in 0..jd.ncomp as usize {
                    // Get huffman table ID
                    let b = jd.inbuf[2 + 2 * i];
                    if b != 0 && b != 0x11 {
                        // Err: Different table number for DC/AC element
                        return Err(Error::UnsupportedJpeg);
                    }
                    let n = if i != 0 { 1 } else { 0 }; // Component class

                    // Check huffman table for this component
                    if (jd.huffbits[n][0]).is_empty() || (jd.huffbits[n][1]).is_empty() {
                        // Err: Not loaded
                        return Err(Error::InvalidData);
                    }
                    // Check dequantizer table for this component
                    if (jd.qttbl[jd.qtid[i] as usize]).is_empty() {
                        // Err: Not loaded
                        return Err(Error::InvalidData);
                    }
                }
                // Allocate working buffer for MCU and pixel output
                let n = jd.msy as i32 * jd.msx as i32; // Number of Y blocks in the MCU
                if n == 0 {
                    // Err: SOF0 has not been loaded
                    return Err(Error::InvalidData);
                }
                len = (n * 64 * 3 + 64) as usize; // Allocate buffer for IDCT and RGB output
                if len < 256 {
                    // but at least 256 byte is required for IDCT
                    len = 256;
                }

                jd.workbuf = alloc_pool_slice(jd, len / 4)?;

                // Allocate MCU working buffer
                jd.mcubuf = alloc_pool_slice(jd, (n as usize + 2) * 64)?;

                // Align stream read offset to JD_SZBUF
                ofs %= JD_SZBUF as u32;
                if ofs != 0 {
                    jd.dctr = jpeg_in(jd, Some(ofs as usize), (JD_SZBUF as u32 - ofs) as usize);
                }
                jd.dptr = (ofs - (if JD_FASTDECODE != 0 { 0 } else { 1 })) as usize;
                return Ok(()); // Initialization succeeded. Ready to
                               // decompress the JPEG image.
            }
            // SOF1, SOF2, SOF3, SOF5, SOF6, SOF7, SOF9, SOF10, SOF11, SOF13, SOF14, SOF15, EOI
            0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 | 0xC9 | 0xCA | 0xCB | 0xCD | 0xCF | 0xCE
            | 0xD9 => {
                // Unsupported JPEG standard (may be progressive JPEG)
                return Err(Error::UnsupportedJpeg);
            }
            _ => {
                // Unknown segment (comment, exif or etc..)
                // Skip segment data (null pointer specifies to remove data from the stream)
                if jpeg_in(jd, None, len) != len {
                    return Err(Error::Input);
                }
            }
        }
    }
}

/// Start to decompress the JPEG picture
/// `scale`: output de-scaling factor (0 to 3)
#[optimize(speed)]
pub fn jd_decomp(jd: &mut JDEC, scale: u8) -> Result<(), Error> {
    if scale > (if JD_USE_SCALE != 0 { 3 } else { 0 }) {
        return Err(Error::Parameter);
    }
    jd.scale = scale;
    let mx = (jd.msx as i32 * 8) as u32; // Size of the MCU (pixel)
    let my = (jd.msy as i32 * 8) as u32; // Size of the MCU (pixel)
    let mut y = 0;
    while y < jd.height as u32 {
        // Vertical loop of MCUs
        let mut x = 0;
        while x < jd.width as u32 {
            // Horizontal loop of MCUs
            if jd.nrst != 0 && {
                // Process restart interval if enabled
                let val = jd.rst;
                jd.rst += 1;
                val == jd.nrst
            } {
                let val = jd.rsc;
                jd.rsc += 1;
                restart(jd, val)?;
                jd.rst = 1;
            }
            // Load an MCU (decompress huffman coded stream, dequantize and apply IDCT)
            mcu_load(jd)?;
            // Output the MCU (YCbCr to RGB, scaling and output)
            mcu_output(jd, x, y)?;
            x += mx;
        }
        y += my;
    }
    Ok(())
}

#[optimize(speed)]
fn jpeg_in(jd: &mut JDEC, inbuf_offset: Option<usize>, n_data: usize) -> usize {
    if let Some(inbuf_offset) = inbuf_offset {
        if (jd.data_read + n_data) <= jd.data_len {
            jd.inbuf[inbuf_offset..inbuf_offset + n_data]
                .copy_from_slice(&jd.data[jd.data_read..jd.data_read + n_data]);
        } else {
            let rest = jd.data_len - jd.data_read;

            if rest > 0 {
                jd.inbuf[inbuf_offset..inbuf_offset + rest]
                    .copy_from_slice(&jd.data[jd.data_read..jd.data_read + rest]);
            } else {
                // error - no data
                return 0;
            }
        }
    }

    jd.data_read += n_data;
    n_data
}

#[optimize(speed)]
fn jpeg_out(jd: &mut JDEC, rect: Rect) -> Result<(), Error> {
    let w = rect.width();
    let h = rect.height();
    let x = rect.x0;

    let bitmap: &[u16] = reinterpret_slice(jd.workbuf);
    let bitmap = &bitmap[..(w * h) as usize];

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
            return Ok(());
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
            return Err(Error::Interrupted);
        }
    }

    Ok(())
}

pub fn jpeg(data: &[u8], pos: Point, scale: u8) {
    let mut jd: JDEC = jd_init(data);
    jd.pos = Some(pos);
    let res = jd_prepare(&mut jd);

    if res == Ok(()) {
        let _ = jd_decomp(&mut jd, scale);
    }
}

pub fn jpeg_info(data: &[u8]) -> Option<(Offset, u16)> {
    let mut jd: JDEC = jd_init(data);
    let res = jd_prepare(&mut jd);

    let mcu_height = jd.msy as u16 * 8;

    if mcu_height > 16 || res.is_err() {
        return None;
    }

    Some((Offset::new(jd.width as i16, jd.height as i16), mcu_height))
}

pub fn jpeg_test(data: &[u8]) -> bool {
    let mut jd: JDEC = jd_init(data);
    let res = jd_prepare(&mut jd);

    let mcu_height = jd.msy as u16 * 8;

    if mcu_height > 16 || res.is_err() {
        return false;
    }

    let mut res = jd_decomp(&mut jd, 0);
    while res == Err(Error::Interrupted) {
        res = jd_decomp(&mut jd, 0);
    }
    res.is_ok()
}
