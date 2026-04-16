{
  description = "Cloud game dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "cloud-game";
          buildInputs = [
            # Rust
            rustToolchain

            # Node.js for frontend (npm is bundled with nodejs)
            pkgs.nodejs_22

            # Useful extras
            pkgs.pkg-config
            pkgs.openssl
          ];

          env = {
            # Required for some crates (openssl-sys, etc.)
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };

          shellHook = ''
            echo "Rust $(rustc --version)"
            echo "Node $(node --version)"
            echo ""
            echo "Start MongoDB:  cd server && podman-compose up -d"
            echo "Run server:     cd server && cargo run"
            echo "Run frontend:   cd frontend && npm install && npm run dev"
          '';
        };
      }
    );
}
