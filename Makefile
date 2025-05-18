SRC = $(shell find src/ -name "*.rs")

IMAGE = am-hal

$(IMAGE).txt : $(SRC)
	@cargo build
	@cargo objdump -- -d > $(IMAGE).txt

run: $(IMAGE).txt
