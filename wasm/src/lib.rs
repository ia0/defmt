#![no_std]

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::SeqCst;

#[no_mangle]
pub extern "C" fn main() {
    defmt::panic!("search {} me", 4);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}

#[defmt::global_logger]
struct Logger;

static ACQUIRED: AtomicBool = AtomicBool::new(false);
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        assert!(!ACQUIRED.swap(true, SeqCst));
        // SAFETY: We are in a critical section.
        let encoder = unsafe { &mut ENCODER };
        encoder.start_frame(write);
    }

    unsafe fn flush() {}

    unsafe fn release() {
        // SAFETY: We are in a critical section.
        let encoder = unsafe { &mut ENCODER };
        encoder.end_frame(write);

        assert!(ACQUIRED.swap(false, SeqCst));
    }

    unsafe fn write(bytes: &[u8]) {
        // SAFETY: We are in a critical section.
        let encoder = unsafe { &mut ENCODER };
        encoder.write(bytes, write);
    }
}

fn write(bytes: &[u8]) {
    extern "C" {
        fn print(x: u32);
    }

    for &byte in bytes {
        unsafe { print(byte as u32) };
    }
}
