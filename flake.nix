{
  inputs = {
    pyproject-nix = {
      url = "github:pyproject-nix/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    uv2nix = {
      url = "github:pyproject-nix/uv2nix";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pyproject-build-systems = {
      url = "github:pyproject-nix/build-system-pkgs";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.uv2nix.follows = "uv2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      pyproject-nix,
      uv2nix,
      pyproject-build-systems,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;

      workspace = uv2nix.lib.workspace.loadWorkspace { workspaceRoot = ./.; };

      overlay = workspace.mkPyprojectOverlay {
        sourcePreference = "wheel";
      };

      editableOverlay = workspace.mkEditablePyprojectOverlay {
        root = "$REPO_ROOT";
      };

      pythonSets = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          python = pkgs.python3;
          qtEnv =
            with pkgs.qt6;
            env "qt-env" [
              qtbase
              qtdeclarative
              qtcharts
              qtgraphs
              qt3d
              qtvirtualkeyboard
              qtwebengine
              qt3d
              qtscxml
              qtwayland
              qtquick3d
            ];

          override = _final: prev: {
            pyside6-essentials = prev.pyside6-essentials.overrideAttrs (old: {
              buildInputs = (old.buildInputs or [ ]) ++ [
                qtEnv
                pkgs.gtk3
                pkgs.mysql80
                pkgs.libpq
                pkgs.firebird
                pkgs.cups.lib
                pkgs.unixODBC
              ];
              preBuild = (old.preBuild or "") + ''
                addAutoPatchelfSearchPath ${prev.shiboken6}/lib/python3.13/site-packages/shiboken6
                addAutoPatchelfSearchPath ${pkgs.rigsofrods-bin}/share/rigsofrods/lib
              '';
              autoPatchelfIgnoreMissingDeps = [
                "libclntsh.so.23.1"
                "libmimerapi.so"
              ];
            });
          };
        in
        (pkgs.callPackage pyproject-nix.build.packages {
          inherit python;
        }).overrideScope
          (
            lib.composeManyExtensions [
              pyproject-build-systems.overlays.wheel
              overlay
              override
            ]
          )
      );

    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          pythonSet = pythonSets.${system}.overrideScope editableOverlay;
          virtualenv = pythonSet.mkVirtualEnv "egglog-ad-dev-env" workspace.deps.all;
        in
        {
          default = pkgs.mkShell {
            packages = [
              virtualenv
              pkgs.uv
              pkgs.graphviz
            ];
            env = {
              UV_NO_SYNC = "1";
              UV_PYTHON = pythonSet.python.interpreter;
              UV_PYTHON_DOWNLOADS = "never";
            };
            shellHook = ''
              unset PYTHONPATH
              export REPO_ROOT=$(git rev-parse --show-toplevel)
            '';
          };
        }
      );

      packages = forAllSystems (system: {
        default = pythonSets.${system}.mkVirtualEnv "egglog-ad-env" workspace.deps.default;
      });
    };
}
