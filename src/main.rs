//注意未使用 裸机编译
#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[unsafe(no_mangle)]//使用c语言的调用约定
pub extern "C" fn _start() -> ! {
    //此处默认命名为_start
    loop {}
}

#[panic_handler]//panic时调用
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
