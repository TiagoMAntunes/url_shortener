let
    pkgs = import <nixpkgs> {};
in
    pkgs.mkShell {
        nativeBuildInputs = [ 
            pkgs.cargo 
            pkgs.libiconv 
            pkgs.docker-compose 
            pkgs.rustc 
            pkgs.rust-analyzer 
            pkgs.rustfmt 
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.pkg-config
            pkgs.openssl
            pkgs.diesel-cli
            pkgs.postgresql
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    }
