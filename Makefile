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

default: build

build:
	@cargo --config $(CONFIG) build --target $(ISA)

disasm: build
	@cargo --config $(CONFIG) objdump --target $(ISA) -- -d

clean:
	@cargo clean
	@rm disasm.txt

.PHONY: clean
