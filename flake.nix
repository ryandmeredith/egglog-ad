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
                xdot
              ];
              shellHook = ''
                nu --execute '
                  def gather [] {
                    parse --regex `Rule (?<rule>.*): search and apply \d+(?:\.\d+)?s, num matches (?<matches>\d+)`
                    | update matches { into int }
                    | sort-by matches
                  }
                  def new_rules [old: table] {
                    join $old rule
                    | where matches > $it.matches_
                    | get rule
                  }
                  def runtest [] {
                    rm --force expr.egg
                    egglog --threads 0 --to-svg test.egg
                  }
                '
                exit
              '';
            };
          };
      }
    );
}
