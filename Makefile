# Put your source files here (or *.c, etc)

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Normally you shouldn't need to change anything below this line!
#######################################################################################

CC=arm-none-eabi-gcc
OBJCOPY=arm-none-eabi-objcopy

CFLAGS  = -g -O2 -Wall -Tlow_level/stm32_flash.ld
CFLAGS += -mlittle-endian -mthumb -mcpu=cortex-m4 -mthumb-interwork
CFLAGS += -mfloat-abi=hard -mfpu=fpv4-sp-d16
CFLAGS += -Ilow_level
CFLAGS += -Wl,--gc-sections


# add startup file to build
SRCS = low_level/startup_stm32f407xx.s
SRCS += libstm32_rust.a


.PHONY: proj

all: proj

proj: $(PROJ_NAME).elf

$(PROJ_NAME).elf: $(SRCS)
	$(CC) $(CFLAGS) $^ -o $@
	$(OBJCOPY) -O ihex $(PROJ_NAME).elf $(PROJ_NAME).hex
	$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin

libstm32_rust.a: build_libstm32
	cp target/thumbv7em-none-eabihf/release/libstm32_rust.a ./

.PHONY: build_libstm32
build_libstm32:
	xargo build --target thumbv7em-none-eabihf --release

clean:
	reset
	xargo clean
	rm -f *.o $(PROJ_NAME).elf $(PROJ_NAME).hex $(PROJ_NAME).bin libstm32_rust.a
