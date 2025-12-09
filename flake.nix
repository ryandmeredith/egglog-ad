{
  inputs = {
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { lib, ... }:
      {
        systems = lib.systems.flakeExposed;
        perSystem =
          { pkgs, ... }:
          {
            devShells.default = pkgs.mkShell {
              packages = with pkgs; [
                cargo
                rust-analyzer
                clippy
                rustfmt
                graphviz
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
                '
                exit
              '';
            };
          };
      }
    );
}
