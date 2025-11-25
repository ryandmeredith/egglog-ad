{
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
                egglog
                rlwrap
                graphviz
              ];
            };
          };
      }
    );
}
