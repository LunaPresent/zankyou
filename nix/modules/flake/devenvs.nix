{
  perSystem =
    { pkgs, self', ... }:
    {
      devShells.default = pkgs.mkShell {
        name = "zankyou";
        meta.description = "Development environment for zankyou";
        RUST_SRC_PATH = builtins.toString pkgs.rust.packages.stable.rustPlatform.rustLibSrc;

        nativeBuildInputs = [
          pkgs.cargo
          pkgs.pkg-config
          pkgs.rustc
          self'.formatter
        ];
      };
    };
}
