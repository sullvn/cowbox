let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  mkShell {
    buildInputs = [
      cargo
      clippy
      libiconv
      llvmPackages.lldb
      rust-analyzer
      rustc
      rustfmt
    ];
  }
