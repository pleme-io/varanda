{
  description = "varanda — family-facing PWA for the saguão fleet";

  nixConfig = {
    allow-import-from-derivation = true;
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    crate2nix.url = "github:nix-community/crate2nix";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
  };

  # WASM build via substrate's wasm-build helpers (see rust-wasm skill).
  # The skill notes there are several ways to wire wasm-pack into Nix;
  # this scaffold uses the most direct shape — devShell with trunk +
  # wasm-bindgen-cli, plus a `nix build` that runs `trunk build --release`.
  outputs = {self, nixpkgs, crate2nix, flake-utils, substrate, fenix, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = fenix.packages.${system}.stable.toolchain.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            trunk
            wasm-bindgen-cli
            wasm-pack
            binaryen     # wasm-opt
          ];
        };

        # Production bundle. Output is dist/ with index.html + .wasm + .js.
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "varanda";
          version = "0.1.0";
          src = self;
          buildInputs = [ rustToolchain pkgs.trunk pkgs.wasm-bindgen-cli pkgs.binaryen ];
          buildPhase = ''
            export HOME=$TMPDIR
            trunk build --release
          '';
          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };

        # Cloudflare Pages deploy via Wrangler — typed in shikumi later.
        # apps.deploy-pages = ...
      });
}
