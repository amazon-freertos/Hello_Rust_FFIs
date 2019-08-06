#![no_std]
#![no_main]
#![feature(asm)]
// Include C bindings.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("freertos.rs");

// Set panic behavior.
extern crate panic_halt;
//extern crate panic_abort;
//extern crate panic_semihosting;

// C-type FFI include(s).
use cstr_core::CStr;
use cty::{c_char, c_void};
// Generic ARM Cortex-M include(s).
use cortex_m_rt::{entry, exception};
// Chip-specific PAC include(s).
//use stm32_pac::stm32l0x1 as stm32;
use stm32l4::stm32l4x5 as stm32;

// System boots to ~2.1MHz oscillator.
#[no_mangle]
pub static SystemCoreClock: u32 = 2_097_000;

// Simple FreeRTOS task function.
// For now, use the 'context' pointer field to
// pass in a reference to the GPIO port struct.
#[no_mangle]
unsafe extern "C" fn blink_task(context: *mut c_void) {
    let gpiob: &mut stm32::GPIOB = &mut *(context as *mut stm32::GPIOB);
    // TODO: macros such as 'pdMS_TO_TICKS' are not currently
    // available. The preprocessor expands them, so 'bindgen'
    // has no way of knowing about them. But if we didn't
    // expand the macros, all of the #ifdefs would still be
    // there and bindgen wouldn't know how to generate the FFI
    // for this application's specific FreeRTOSConfig.
    // Anyways, at a 250Hz tick, 250 ticks should ~= 1 second.
    loop {
        gpiob.odr.write(|w| w.odr14().set_bit());
        vTaskDelay(250);
        gpiob.odr.write(|w| w.odr14().clear_bit());
        vTaskDelay(250);
    }
}

// Simple test program to start a FreeRTOS task from Rust.
#[entry]
fn main() -> ! {
    // Checkout STM32 peripheral singleton.
    let p = stm32::Peripherals::take().unwrap();
    let rcc = p.RCC;
    let mut gpiob = p.GPIOB;
    // Set up GPIO pin B14 as push-pull output.
    rcc.ahb2enr.modify(|_,w| w.gpioben().set_bit());
    gpiob.moder.write(|w| w.moder14().output());
    gpiob.otyper.write(|w| w.ot14().clear_bit());

    // Create the 'blinking LED' task.
    let gpiob_ref: *mut c_void = &mut gpiob as *mut _ as *mut c_void;
    let task_name: *const c_char = CStr::from_bytes_with_nul(b"Blink\0").unwrap().as_ptr();
    let mut task_handle: TaskHandle_t = 0 as TaskHandle_t;
    // TODO: Best way to access FreeRTOSConfig macros like 'configMINIMAL_STACK_SIZE'?
    unsafe {
        xTaskCreate(
            Some(blink_task),
            task_name,
            128,
            gpiob_ref,
            1,
            &mut task_handle,
        )
    };
    unsafe { vTaskStartScheduler() };

    // Scheduler is now running; this should never be reached.
    loop {}
}

// System interrupt handlers.
#[exception]
fn SVCall() -> ! {
    unsafe {
        vPortSVCHandler();
    }
}

#[exception]
fn PendSV() -> ! {
    unsafe {
        xPortPendSVHandler();
    }
}

#[exception]
fn SysTick() -> ! {
    unsafe {
        xPortSysTickHandler();
    }
}

// HardFault handler.
#[exception]
fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // Uncomment to have a breakpoint trigger while debugging:
    //cortex_m::asm::bkpt();
    // TODO: Print exception frame if semihosting is enabled?
    panic!();
}
