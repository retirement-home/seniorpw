{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: function nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        seniorpw = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          name = "seniorpw";
          src = builtins.path {
            path = ./.;
            name = "source";
          };

          sourceRoot = "source/src/seniorpw";
          cargoLock = {
            lockFile = ./src/seniorpw/Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.installShellFiles ];
          postInstall = ''
            mandir="$releaseDir/build/senior-*/out/man/*.1"
            for manfile in $mandir; do
              installManPage "$manfile"
            done

            installShellCompletion --bash ../completions/senior.bash
            installShellCompletion --zsh ../completions/senior.zsh
          '';

          meta = {
            description = "Password manager using age as backend; inspired by pass";
            homepage = "https://gitlab.com/retirement-home/seniorpw";
          };

          SENIORPW_ALT_MANDIR = "1";
        });

        default = seniorpw;
      });
    };
}
