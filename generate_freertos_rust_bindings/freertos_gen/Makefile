# Makefile to build FreeRTOS as a static library.
TARGET = libfreertos_interop

TOOLCHAIN = /usr
CC = $(TOOLCHAIN)/bin/arm-none-eabi-gcc
AR = $(TOOLCHAIN)/bin/arm-none-eabi-ar

#CFLAGS += -mcpu=cortex-m0plus
CFLAGS += -mcpu=cortex-m4
CFLAGS += -mhard-float
CFLAGS += -mfloat-abi=hard
CFLAGS += -mfpu=fpv4-sp-d16
CFLAGS += -mthumb
CFLAGS += -Wall
#CFLAGS += -g
CFLAGS += -O1
CFLAGS += --specs=nosys.specs
CFLAGS += -I. -I..
CFLAGS += -I../freertos/include
CFLAGS += -I../freertos/portable/GCC/ARM_CM0

FR = ../freertos

C_SRC  = $(FR)/event_groups.c
C_SRC += $(FR)/list.c
C_SRC += $(FR)/queue.c
C_SRC += $(FR)/stream_buffer.c
C_SRC += $(FR)/tasks.c
C_SRC += $(FR)/timers.c
C_SRC += $(FR)/portable/MemMang/heap_4.c
C_SRC += $(FR)/portable/GCC/ARM_CM0/port.c
C_SRC += c_helpers.c

OBJS = $(C_SRC:.c=.o)

.PHONY: all
all: $(TARGET).a

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(TARGET).a: $(OBJS)
	$(AR) rcs $@ $^

.PHONY: clean
clean:
	rm -f $(OBJS)

.PHONY: clobber
clobber: clean
	rm -f $(TARGET).a
