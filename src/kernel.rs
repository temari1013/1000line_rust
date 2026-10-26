#![no_main]
#![no_std]

use core::arch::asm;
use core::arch::naked_asm;
use core::panic::PanicInfo;
use core::ptr;



unsafe extern "C" {
    static  mut  __bss: u32;
    static  mut  __bss_end: u32;
    static __stack_top: u32;
}


fn kernel_main() {
    let bss_start = &raw mut __bss;
    let bss_end = &raw mut __bss_end ; 
    unsafe {
          ptr::write_bytes( bss_end, 0, bss_end as usize - bss_start as usize );
    }
    loop{}
}

#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
unsafe extern "C" fn boot(){
    naked_asm!( 
        "mv sp, {stack_top}" , 
        "j {kernel_main}" , 
        stack_top = sym __stack_top,
        kernel_main = sym kernel_main,
    );
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}


/*
typedef unsigned char uint8_t;
typedef unsigned int uint32_t;
typedef uint32_t size_t;

extern char __bss[], __bss_end[], __stack_top[];

void *memset(void *buf, char c, size_t n) {
    uint8_t *p = (uint8_t *) buf;
    while (n--)
        *p++ = c;
    return buf;
}

void kernel_main(void) {
    memset(__bss, 0, (size_t) __bss_end - (size_t) __bss);

    for (;;);
}

__attribute__((section(".text.boot")))
__attribute__((naked))
void boot(void) {
    __asm__ __volatile__(
        "mv sp, %[stack_top]\n"
        "j kernel_main\n"
        :
        : [stack_top] "r" (__stack_top)
    );
}
*/