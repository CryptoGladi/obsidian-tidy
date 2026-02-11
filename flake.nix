{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      flake-utils,
      naersk,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
          ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        };
      in
      {
        # For `nix build` & `nix run`:
        packages.default = naersk'.buildPackage {
          src = ./.;

          doCheck = true;
          cargoTestOptions = [
            "$cargo_release"
            ''-j "$NIX_BUILD_CORES"''
            "--workspace"
          ];
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ rust ];

          shellHook = ''
            echo "Active nix develop"
          '';
        };

        RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
      }
    );
}
