{
  description = "Flake configuration for am-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        guidymlib = with pkgs; [
          pkg-config
          wayland

          libxkbcommon
          libGL
          vulkan-loader
          vulkan-headers
          vulkan-tools
          gtk3

          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
          xorg.libxcb
        ];

        DevEnv = pkgs.symlinkJoin {
          name = "dev-env";
          paths = with pkgs; [
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

            # GUI
            guidymlib
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ DevEnv ];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath guidymlib}";
        };
      }
    );
}
