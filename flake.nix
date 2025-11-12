{
  description = "Flake configuration for am-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      utils,
      rust-overlay,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # rust toolchain
            (rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "clippy"
                "rust-analyzer"
                "llvm-tools-preview"
              ];
              targets = [
                "riscv32i-unknown-none-elf"
                "riscv32im-unknown-none-elf"
                "riscv32imac-unknown-none-elf"
              ];
            })
            cargo-binutils
            cargo-edit

            # simulators and tools
            qemu
            spike
            dtc

            # scripts dependencies
            nushell
            just
          ];
        };
      }
    );
}
