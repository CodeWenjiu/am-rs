build:
	@cargo build

disasm: build
	@cargo objdump -- -d > disasm.txt
