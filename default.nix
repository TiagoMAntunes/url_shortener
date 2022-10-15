let
    pkgs = import <nixpkgs> {};
in
    pkgs.mkShell {
        nativeBuildInputs = [ pkgs.cargo pkgs.libiconv pkgs.docker-compose pkgs.rustc pkgs.rust-analyzer ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    }
