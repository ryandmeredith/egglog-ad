{
  inputs = {
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    egglog-src = {
      url = "github:egraphs-good/egglog";
      flake = false;
    };
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      egglog-src,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { lib, ... }:
      {
        systems = lib.systems.flakeExposed;
        perSystem =
          { pkgs, ... }:
          let
            egglog1 = pkgs.rustPlatform.buildRustPackage {
              name = "egglog";
              version = "1.0.0";
              src = egglog-src;
              cargoLock.lockFile = "${egglog-src}/Cargo.lock";
              doCheck = false;
            };
          in
          {
            devShells.default = pkgs.mkShell {
              packages = with pkgs; [
                egglog1
                rlwrap
                graphviz
              ];
            };
          };
      }
    );
}
