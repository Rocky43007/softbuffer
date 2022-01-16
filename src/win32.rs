use std::os::raw::c_int;
use raw_window_handle::Win32Handle;
use crate::GraphicsContextImpl;
use winapi::um::wingdi::{BITMAPINFOHEADER, BI_BITFIELDS, RGBQUAD, StretchDIBits};
use winapi::um::winuser::{ValidateRect, GetDC};
use winapi::shared::windef::{HWND, HDC};

pub struct Win32Impl{
    window: HWND,
    dc: HDC
}

// Wrap this so we can have a proper number of bmiColors to write in
// From minifb
#[repr(C)]
struct BitmapInfo {
    pub bmi_header: BITMAPINFOHEADER,
    pub bmi_colors: [RGBQUAD; 3],
}

impl Win32Impl {

    pub unsafe fn new(handle: &Win32Handle) -> Self{
        let dc = GetDC(handle.hwnd as HWND);
        Self{
            dc,
            window: handle.hwnd as HWND
        }
    }

}

impl GraphicsContextImpl for Win32Impl {
    unsafe fn set_buffer(&mut self, buffer: &[u32], width: u16, height: u16) {
        let mut bitmap_info: BitmapInfo = std::mem::zeroed();

        bitmap_info.bmi_header.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_info.bmi_header.biPlanes = 1;
        bitmap_info.bmi_header.biBitCount = 32;
        bitmap_info.bmi_header.biCompression = BI_BITFIELDS;
        bitmap_info.bmi_header.biWidth = width as i32;
        bitmap_info.bmi_header.biHeight = -(height as i32);
        bitmap_info.bmi_colors[0].rgbRed = 0xff;
        bitmap_info.bmi_colors[1].rgbGreen = 0xff;
        bitmap_info.bmi_colors[2].rgbBlue = 0xff;

        StretchDIBits(
            self.dc,
            0,
            0,
            width as c_int,
            height as c_int,
            0,
            0,
            width as c_int,
            height as c_int,
            std::mem::transmute(buffer.as_ptr()),
            std::mem::transmute(&bitmap_info),
            winapi::um::wingdi::DIB_RGB_COLORS,
            winapi::um::wingdi::SRCCOPY
        );

        ValidateRect(self.window, std::ptr::null_mut());
    }
}