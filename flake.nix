{
  description = "Rust dev shell with soulfind server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";

    soulfind-repo = {
      url = "github:soulfind-dev/soulfind";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, soulfind-repo, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        soulfind = pkgs.stdenv.mkDerivation {
          name = "soulfind";
          src = soulfind-repo;

          nativeBuildInputs = with pkgs; [
            ldc
            dub
            sqlite
          ];

          buildPhase = ''
            export HOME=$TMPDIR
            dub build --build=release --compiler=ldc2
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp bin/soulfind $out/bin/soulfind
            cp bin/soulsetup $out/bin/soulsetup
          '';
        };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;

        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            rustPackages.clippy
            pre-commit

            soulfind
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

          shellHook = ''
            echo -e "This is a dev shell for klymene!\
            \n  - run test soulseek server with soulfind\
            \n  - configure said server with soulsetup\
            \n  - your soulfind.db is ignored by .gitignore, so don't worry about deleting it"
          '';
        };
      }
    );
}
