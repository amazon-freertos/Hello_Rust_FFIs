# Overview

Test project to try generating FreeRTOS bindings for Rust. Targets an STM32L031K6 'Nucleo-32' board, to have a single easy test platform.

It's a little messy right now and the build process is a little fragmented, but it looks like the basic idea can work.

First, run `make` in the `ffi_dummy` directory to produce a header file contianing the FreeRTOS methods which will be available with the configs defined in the usual `FreeRTOSConfig.h` file. Then, run `make` in the `freertos_gen` directory to build FreeRTOS as a static library.

Then when you run `cargo build`, the build script will run `bindgen` and create a `freertos.rs` file containing the Rust/C interface. The `main.rs` file includes those bindings, and then you can call FreeRTOS functions from Rust. I think that's about all there is to it, but I haven't done much testing.

# Application

This is just a test of generating and using a FreeRTOS FFI for Rust. It creates a task which toggles an LED every second or so, and then it starts the scheduler.

Not counting the core interrupt handlers, it only calls three FreeRTOS functions: `xTaskCreate`, `vTaskDelay`, and `vTaskStartScheduler`. So it shows that the basic concept works, but it doesn't test very much.

I'm hoping that I'll be able to clean the syntax up a bit, too.
