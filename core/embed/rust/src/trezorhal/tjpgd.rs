use super::ffi;

use crate::trezorhal::{
    buffers::get_jpeg_buffer,
    tjpgdlib::{jd_init, jd_prepare, JDEC, JDR_OK},
};
pub use ffi::buffer_jpeg_t as BufferJpeg;

pub struct JpegInfo {
    pub width: u16,
    pub height: u16,
    pub mcu_height: u16,
}

// pub fn jpeg_get_context<'a>(
//     data: &'a [u8],
//     buffer: &'a mut BufferJpeg,
//     line_width: i16,
// ) -> JpegContext<'a> {
//     JpegContext {
//         data,
//         data_read: 0,
//         data_len: data.len(),
//         buffer: &mut buffer.buffer,
//         buffer_width: line_width,
//         buffer_height: 16,
//         current_line: 0,
//         current_line_pix: 0,
//     }
// }

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
