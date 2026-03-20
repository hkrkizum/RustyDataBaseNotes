{
  description = "Minimal development shell for RustyDataBaseNotes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            pkgs = import nixpkgs { inherit system; };
          }
        );
      fontPackagesFor =
        pkgs: with pkgs; [
          noto-fonts-cjk-sans
          noto-fonts-cjk-serif
          noto-fonts-color-emoji
          nerd-fonts.meslo-lg
          plemoljp-nf
        ];
      projectPackagesFor =
        pkgs: with pkgs; [
          nodejs
          pnpm
          uv

          rustup
          cargo-tauri
          pkg-config
          gcc
          clang
          mold
          gnumake
          cargo-nextest
          cargo-make
          cargo-llvm-cov

          gtk3
          webkitgtk_4_1
          libsoup_3
          glib
          cairo
          pango
          gdk-pixbuf
          atk
          harfbuzz
          zlib
          openssl
          xdotool
          libayatana-appindicator
          librsvg
          file

          mesa
          libglvnd
        ];

    in
    {
      devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = (projectPackagesFor pkgs) ++ (fontPackagesFor pkgs);
            shellHook = ''
              export CODEX_HOME="$PWD/.codex"
              export FONTCONFIG_FILE="${pkgs.makeFontsConf { fontDirectories = fontPackagesFor pkgs; }}"
              export __EGL_VENDOR_LIBRARY_DIRS="${pkgs.mesa}/share/glvnd/egl_vendor.d"
              export LD_LIBRARY_PATH="${
                pkgs.lib.makeLibraryPath [
                  pkgs.mesa
                  pkgs.libglvnd
                ]
              }''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
            '';
          };
        }
      );

      formatter = forAllSystems ({ pkgs }: pkgs.nixfmt-tree);
    };
}
