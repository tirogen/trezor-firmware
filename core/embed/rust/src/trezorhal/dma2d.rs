use core::mem::MaybeUninit;
use core::slice::from_raw_parts;
use super::ffi;

pub use ffi::DMA2D_HandleTypeDef;

impl Default for ffi::DMA2D_HandleTypeDef {
    fn default() -> Self {
        unsafe { MaybeUninit::<Self>::zeroed().assume_init() }
    }
}


pub fn dma2d_init() -> DMA2D_HandleTypeDef {
    let mut handle = DMA2D_HandleTypeDef::default();

    handle.Instance = ffi::DMA2D_BASE as _;
    handle.Init.Mode = ffi::DMA2D_M2M_BLEND;
    handle.Init.ColorMode = ffi::DMA2D_OUTPUT_RGB565;
    handle.Init.OutputOffset = 0;

    handle.LayerCfg[1].InputColorMode = ffi::DMA2D_INPUT_A4;
    handle.LayerCfg[1].InputOffset = 0;
    handle.LayerCfg[1].AlphaMode = 0;
    handle.LayerCfg[1].InputAlpha = 0xFFFFFFFF;

    handle.LayerCfg[0].InputColorMode = ffi::DMA2D_INPUT_RGB565;
    handle.LayerCfg[0].InputOffset = 0;
    handle.LayerCfg[0].AlphaMode = 0;
    handle.LayerCfg[0].InputAlpha = 0;

    unsafe {
        ffi::HAL_DMA2D_Init(&mut handle);
        ffi::HAL_DMA2D_ConfigLayer(&mut handle, 1);
        ffi::HAL_DMA2D_ConfigLayer(&mut handle, 0);
    }

    handle
}

pub fn dma2d_start(
    handle: &mut DMA2D_HandleTypeDef,
    fg_buffer: &'static [u8],
    bg_buffer: &'static [u16],
    data_len: i32,
) {
    unsafe {
        ffi::HAL_DMA2D_BlendingStart(handle,
                                     fg_buffer.as_ptr() as _,
                                     bg_buffer.as_ptr() as _,
                                     ffi::DISPLAY_DATA_ADDRESS as _,
                                     2,
                                     (data_len / 2) as u32) ;
    }
}

pub fn dma2d_wait_for_transfer(handle: &mut DMA2D_HandleTypeDef) {

    unsafe {
        while ffi::HAL_DMA2D_PollForTransfer(handle, 10) != ffi::HAL_StatusTypeDef_HAL_OK {}
    }
}

pub fn get_buffer_1() -> &'static [u16]{
    unsafe {
        from_raw_parts(ffi::display_get_line_buffer_1() as *const u16, 240)
    }
}

pub fn get_buffer_2() -> &'static [u16]{
    unsafe {
        from_raw_parts(ffi::display_get_line_buffer_2() as *const u16, 240)
    }
}

pub fn get_buffer_4bpp_1() -> &'static [u8]{
    unsafe {
        from_raw_parts(ffi::display_get_line_buffer_4bpp_1() as *const u8, 240 / 2)
    }
}

pub fn get_buffer_4bpp_2() -> &'static [u8]{
    unsafe {
        from_raw_parts(ffi::display_get_line_buffer_4bpp_2() as *const u8, 240 / 2)
    }
}
