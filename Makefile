ARCHS = riscv32i-nemu riscv32im-nemu

ifeq ($(filter clean help,$(MAKECMDGOALS)),)

ifeq ($(filter $(ARCHS), $(ARCH)), )
$(error Expected $$ARCH in {$(ARCHS)}, Got "$(ARCH)")
endif

ARCH_SPLIT = $(subst -, ,$(ARCH))
ISAs        = $(word 1,$(ARCH_SPLIT))
PLATFORM   = $(word 2,$(ARCH_SPLIT))

ifeq ($(ISAs), riscv32i)
	ISA = riscv32i-unknown-none-elf
endif

ifeq ($(PLATFORM), nemu)
	CONFIG ?= platform/nemu/config.toml
endif

endif

BIN ?= dummy

BUILD_ARGS = --bin $(BIN) --target $(ISA)
BUILD_DIRS = build/$(PLATFORM)/$(BIN)

default: build

build:
	@cargo  build $(BUILD_ARGS) --release

disasm:
	@mkdir -p $(BUILD_DIRS)
	@cargo  objdump $(BUILD_ARGS) -- -d > $(BUILD_DIRS)/image.txt
	@cargo  objcopy $(BUILD_ARGS) -- -O binary $(BUILD_DIRS)/image.bin

clean:
	@cargo clean
	@rm -rf build/

.PHONY: clean
