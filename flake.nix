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
    # Design system — single source of truth for color, typography,
    # spacing, shadow, motion, brand. Consumed via render to CSS at
    # build time. NEVER hand-author colors / fonts / spacing here.
    ishou = {
      url = "github:pleme-io/ishou";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {self, nixpkgs, crate2nix, flake-utils, substrate, fenix, ishou, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = fenix.packages.${system}.stable.toolchain.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Render the canonical pleme-io design tokens to CSS at build
        # time. The output is byte-identical across every consumer.
        ishouTokensCss = pkgs.runCommand "ishou-tokens.css" {} ''
          ${ishou.packages.${system}.default}/bin/ishou render --target css --out $out
        '';
      in {
        # Expose the tokens for downstream tools (testing, preview, …).
        packages.tokens-css = ishouTokensCss;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            trunk
            wasm-bindgen-cli
            wasm-pack
            binaryen     # wasm-opt
          ];
          shellHook = ''
            # Refresh the tokens.css symlink so trunk picks them up.
            mkdir -p public
            ln -sf ${ishouTokensCss} public/ishou-tokens.css
          '';
        };

        # Production bundle. Output is dist/ with index.html + .wasm + .js +
        # the rendered ishou tokens.
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "varanda";
          version = "0.1.0";
          src = self;
          buildInputs = [ rustToolchain pkgs.trunk pkgs.wasm-bindgen-cli pkgs.binaryen ];
          buildPhase = ''
            export HOME=$TMPDIR
            mkdir -p public
            cp ${ishouTokensCss} public/ishou-tokens.css
            trunk build --release
          '';
          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
      });
}
