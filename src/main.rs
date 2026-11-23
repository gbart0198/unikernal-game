#![no_std]
#![no_main]
#![allow(unused)]

use core::arch::asm;

use core::fmt::{Error, Write};
use core::panic::PanicInfo;
use core::str::Bytes;

use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

pub struct SerialWriter {}

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe { asm!("out dx, al", in("dx") 0x3F8, in("al") c ) }
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    let test = "\r\n\r\nHello, Gart";

    write!(SerialWriter {}, "hello, {}", 42);

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
