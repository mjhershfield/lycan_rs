# save this as shell.nix
{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [ 
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    # libgcc.lib
    # pkg-config
  ];

  LD_LIBRARY_PATH="${pkgs.libgcc.lib}/lib";
  # RUST_BACKTRACE = 1;
}
