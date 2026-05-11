#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub fn run(n: u32) {
    for i in 0..n {
        core::hint::black_box(i);
    }
}

#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}
