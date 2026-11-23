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

    let test = "\r\n\r\nHello, Gart";

    for c in test.bytes() {
        unsafe { asm!("out dx, al", in("dx") 0x3F8, in("al") c ) }
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
