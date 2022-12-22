use super::ffi;
use core::{mem::MaybeUninit, ptr::NonNull, slice};

use crate::trezorhal::{
    buffers::{get_jpeg_buffer, get_jpeg_work_buffer},
    tjpgdlib::{jd_decomp, jd_prepare, JDEC, JDR_OK, JRECT},
};
pub use ffi::buffer_jpeg_t as BufferJpeg;

impl Default for JDEC {
    fn default() -> Self {
        unsafe { MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

pub struct JpegInfo {
    pub width: u16,
    pub height: u16,
    pub mcu_height: u16,
}

pub struct JpegContext<'a> {
    data_read: usize,
    data_len: usize,
    buffer_width: i16,
    buffer_height: i16,
    current_line: i16,
    current_line_pix: i16,
    data: &'a [u8],
    buffer: &'a mut [u16],
}

unsafe fn jpeg_in_buffer(jd: *mut JDEC, buff: *mut u8, n_data: usize) -> usize {
    let context = unsafe { NonNull::new_unchecked((*jd).device as *mut JpegContext).as_mut() };
    let n_data = n_data as usize;
    if !buff.is_null() {
        let buff = unsafe { slice::from_raw_parts_mut(buff, n_data) };

        if (context.data_read + n_data) <= context.data_len {
            let _ = &buff[0..n_data]
                .copy_from_slice(&context.data[context.data_read..context.data_read + n_data]);
        } else {
            let rest = context.data_len - context.data_read;

            if rest > 0 {
                let _ = &buff[0..rest]
                    .copy_from_slice(&context.data[context.data_read..context.data_read + rest]);
            } else {
                // error - no data
                return 0;
            }
        }
    }

    context.data_read += n_data;
    n_data as _
}

unsafe fn jpeg_out_buffer(jd: *mut JDEC, bitmap_raw: &&mut [i32], rect: *mut JRECT) -> cty::c_int {
    let jd = unsafe { NonNull::new_unchecked(jd as *mut JDEC).as_mut() };
    let context = unsafe { NonNull::new_unchecked(jd.device as *mut JpegContext).as_mut() };
    let rect = unsafe { NonNull::new_unchecked(rect as *mut JRECT).as_mut() };

    let w = (rect.right - rect.left + 1) as i16;
    let h = (rect.bottom - rect.top + 1) as i16;
    let x = rect.left as i16;

    let bitmap =
        unsafe { slice::from_raw_parts(bitmap_raw.as_ptr() as *const u16, (w * h) as usize) };

    if h > context.buffer_height {
        // unsupported height, call and let know
        return 1;
    }

    let buffer_len = (context.buffer_width * context.buffer_height) as usize;

    for i in 0..h {
        for j in 0..w {
            let buffer_pos = ((x + j) + (i * context.buffer_width)) as usize;
            if buffer_pos < buffer_len {
                context.buffer[buffer_pos] = bitmap[(i * w + j) as usize];
            }
        }
    }

    context.current_line_pix += w;

    if context.current_line_pix >= context.buffer_width {
        context.current_line_pix = 0;
        context.current_line += (jd.msy * 8) as i16;
        // finished line, abort and continue later
        return 0;
    }

    1
}

pub fn jpeg_get_context<'a>(
    data: &'a [u8],
    buffer: &'a mut BufferJpeg,
    line_width: i16,
) -> JpegContext<'a> {
    JpegContext {
        data,
        data_read: 0,
        data_len: data.len(),
        buffer: &mut buffer.buffer,
        buffer_width: line_width,
        buffer_height: 16,
        current_line: 0,
        current_line_pix: 0,
    }
}

pub fn jpeg_buffer_prepare<'a>(jd: &mut JDEC, context: &'a mut JpegContext<'a>) {
    let work_buffer = unsafe { &get_jpeg_work_buffer(0, true).buffer };
    unsafe {
        jd_prepare(
            jd as *mut _,
            Some(jpeg_in_buffer),
            work_buffer.as_ptr() as *mut _,
            work_buffer.len() as _,
            context as *mut JpegContext as *mut _,
        );
    }
}

pub fn jpeg_buffer_decomp(jd: &mut JDEC) {
    unsafe {
        jd_decomp(jd as _, Some(jpeg_out_buffer), 0);
    }
}

pub fn jpeg_info(data: &[u8]) -> Result<JpegInfo, ()> {
    let mut jd = JDEC::default();

    let work_buffer = unsafe { &get_jpeg_work_buffer(0, true).buffer };
    let mut context = jpeg_get_context(data, unsafe { get_jpeg_buffer(0, false) }, 0);

    let res = unsafe {
        jd_prepare(
            &mut jd as *mut _,
            Some(jpeg_in_buffer),
            work_buffer.as_ptr() as *mut _,
            work_buffer.len() as _,
            &mut context as *mut JpegContext as *mut _,
        )
    };

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
