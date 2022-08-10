use core::fmt;
use bootloader::boot_info::FrameBufferInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::Logger;
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::pixel_buffer::global_impl::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().logger.as_mut().unwrap().write_fmt(args).unwrap();
}

lazy_static! {
    pub static ref WRITER: Mutex<MaybeUninitializedLogger> =
        Mutex::new(MaybeUninitializedLogger::uninitialized());
}
pub struct MaybeUninitializedLogger {
    logger: Option<Logger>,
}

impl MaybeUninitializedLogger {
    pub fn uninitialized() -> Self {
        MaybeUninitializedLogger {
            logger: None
        }
    }
    fn _initialize(&mut self, framebuffer: &'static mut [u8], info: FrameBufferInfo) {
        self.logger = Some(Logger::new(framebuffer, info));
    }

    pub fn init(framebuffer: &'static mut [u8], info: FrameBufferInfo) {
        WRITER.lock()._initialize(framebuffer, info);
    }
}