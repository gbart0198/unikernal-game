#![no_std]
#![no_main]
#![allow(unused)]

use core::arch::asm;

use core::panic::PanicInfo;

use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

static mut BUF: [u32; 8192] = [0; 8192];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().iter().next() {
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch + i * 4;

                unsafe {
                    framebuffer
                        .address()
                        .cast::<u8>()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(0xFFFFFFFF);
                }
            }

            unsafe {
                BUF[0] = 0x00FFFFFF;
            }
            for i in 0..300_u64 {
                let pixel_offset = 900 + i * 4;
                unsafe {
                    framebuffer
                        .address()
                        .cast::<u8>()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(BUF[0]);
                }
            }
        }
    }

    main();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    main();
}

fn main() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
