{ self, ... }:
{

  flake.overlays.default =
    final: prev:
    let
      inherit (prev.stdenv.hostPlatform) system;
    in
    if builtins.hasAttr system self.packages then
      { zankyou = self.packages.${system}.default; }
    else
      { };
}
