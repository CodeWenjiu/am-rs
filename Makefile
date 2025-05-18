build:
	@cargo build

disasm: build
	@cargo objdump -- -d > disasm.txt

clean:
	@cargo clean
	@rm disasm.txt
