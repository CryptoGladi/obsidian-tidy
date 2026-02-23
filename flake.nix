{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (p: rust);

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        obsidian-tidy = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;

            postInstall = ''
              # Bash completions
              mkdir -p $out/share/bash-completion/completions
              $out/bin/obsidian-tidy completions bash > $out/share/bash-completion/completions/obsidian-tidy

              # Zsh completions
              mkdir -p $out/share/zsh/site-functions
              $out/bin/obsidian-tidy completions zsh > $out/share/zsh/site-functions/_obsidian-tidy

              # Fish completions
              mkdir -p $out/share/fish/vendor_completions.d
              $out/bin/obsidian-tidy completions fish > $out/share/fish/vendor_completions.d/obsidian-tidy.fish
            '';
          }
        );
      in
      {
        checks = {
          obsidian-tidy-tests = obsidian-tidy // {
            doCheck = true;
            cargoTestCommand = "cargo test --workspace";
          };

          obsidian-tidy-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --workspace -- --deny warnings";
            }
          );

          obsidian-tidy-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          obsidian-tidy-fmt = craneLib.cargoFmt {
            inherit src;
          };

          obsidian-tidy-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
          };

          obsidian-tidy-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
        };

        packages.default = obsidian-tidy // {
          doCheck = true;
          cargoTestCommand = "cargo test --workspace";
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = [
            rust
            pkgs.taplo
          ];

          shellHook = ''
            echo "Active nix develop"
          '';

          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
      }
    );
}
