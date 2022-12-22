use super::ffi;
use core::{mem::MaybeUninit};

use crate::trezorhal::{
    buffers::{get_jpeg_buffer, get_jpeg_work_buffer},
    tjpgdlib::{jd_decomp, jd_prepare, JDEC, JDR_OK},
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
    let work_buffer = unsafe { &mut get_jpeg_work_buffer(0, true).buffer };
    unsafe {
        jd_prepare(
            jd as *mut _,
            work_buffer,
            context as *mut JpegContext as *mut _,
        );
    }
}

pub fn jpeg_buffer_decomp(jd: &mut JDEC) {
    unsafe {
        jd_decomp(jd as _,  0);
    }
}

pub fn jpeg_info(data: &[u8]) -> Result<JpegInfo, ()> {
    let mut jd = JDEC::default();

    let work_buffer = unsafe { &mut get_jpeg_work_buffer(0, true).buffer };
    let mut context = jpeg_get_context(data, unsafe { get_jpeg_buffer(0, false) }, 0);

    let res = unsafe {
        jd_prepare(
            &mut jd as *mut _,
            work_buffer,
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
