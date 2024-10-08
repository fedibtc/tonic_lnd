{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};
        inherit (pkgs) buf protobuf;
      in {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs;
          mkShell {
            buildInputs = [cargo rustc rustfmt rustPackages.clippy buf protobuf];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;

            shellHook = ''
              echo "buf `${buf}/bin/buf --version`"
              ${protobuf}/bin/protoc --version
            '';
          };
      }
    );
}
