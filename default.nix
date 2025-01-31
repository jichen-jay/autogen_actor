{
  pkgs ? import <nixpkgs> { },
  profile ? "release", # Use "release" by default; set to "dev" for debug builds
}:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "ractor_playground";
  version = "0.0.0";

  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  # Use a valid target triple
  cargoBuildTarget = "x86_64-unknown-linux-gnu";

  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.openssl
  ];

  RUSTFLAGS =
    if profile == "release" then
      "-C opt-level=z -C target-cpu=native -C codegen-units=1"
    else
      "-C debuginfo=2";

  CARGO_PROFILE_RELEASE_LTO = if profile == "release" then "thin" else null;

  stripAllList = if profile == "release" then [ "bin" ] else [ ];

  enableParallelBuilding = true;

  doCheck = false; # Disable cargoCheckHook to avoid issues with profiles

  buildPhase = ''
    export CARGO_BUILD_PROFILE=${profile}
    cargo build --profile ${profile} --target ${cargoBuildTarget}
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/${cargoBuildTarget}/${profile}/* $out/bin/ || echo "No binaries found in target/${cargoBuildTarget}/${profile}"
  '';
}
