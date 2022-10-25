use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    beam_bvm_util::common::safe::halt::halt();
    loop { }
}