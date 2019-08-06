# Makefile to produce a header file with all of the
# FreeRTOS methods available for a given configuration.
# Rust's `bindgen` utility can turn the header into an FFI.

.PHONY: all
all: freertos_ffi.h

# ('cpp' stands for 'C PreProcessor', not C++.)
preproc.e:
	#arm-none-eabi-cpp -I. -I.. -I./sys_dummies -I../freertos/include -I../freertos/portable/GCC/ARM_CM0 dummy.c > preproc.e
	arm-none-eabi-cpp -I. -I.. -I./sys_dummies -I../freertos/include -I../freertos/portable/GCC/ARM_CM4F dummy.c > preproc.e

# Remove comments ('#...') and empty lines.
freertos_ffi.h: preproc.e
	sed /^\#/d < preproc.e > mid.e
	sed /^$$/d < mid.e > freertos_ffi.h
	rm -f preproc.e mid.e

.PHONY: clean
clean:
	rm -f freertos_ffi.h
