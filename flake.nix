{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages.default = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;  # Run tests during build
          nativeBuildInputs = with pkgs; [ rustfmt rustPackages.clippy python3 ];

          # Run rustfmt and clippy checks before building
          preBuild = ''
            echo "Running rustfmt check..."
            cargo fmt -- --check

            echo "Running clippy..."
            cargo clippy --all-targets --all-features -- -D warnings
          '';
        };

        # Re-runs md over all docs and fails if any of them would change
        checks.docs = pkgs.runCommand "check-docs"
          { nativeBuildInputs = [ self.packages.${system}.default pkgs.python3 ]; }
          ''
            cp -r ${./.} src
            chmod -R u+w src
            cd src
            bash scripts/check-docs.sh
            touch $out
          '';

        devShells.default = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            python3  # Required for integration tests
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
