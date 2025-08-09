{
  lib,
  pkg-config,
  rustPlatform,
}:
let
  root = ../../../.;
  cargo = lib.importTOML "${root}/Cargo.toml";
in
rustPlatform.buildRustPackage {
  pname = "zankyou";
  version = cargo.workspace.package.version;
  src = root;
  cargoLock.lockFile = "${root}/Cargo.lock";
  nativeBuildInputs = [ pkg-config ];

  meta = {
    description = "A modern terminal music player written in Rust";
    homepage = "https://github.com/LunaPresent/zankyou";
  };
}
