{
  description = "Minimal development shell for RustyDataBaseNotes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      home-manager,
      ...
    }:
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
      commonPackagesFor =
        pkgs: with pkgs; [
          vim
          curl
          wget
          unzip
          zip
          jq
          tree
          dust
          procs

          uv

          nil
          nixd
          nixfmt

          nodejs
          pnpm
          zsh-powerlevel10k
        ];
      projectPackagesFor =
        pkgs: with pkgs; [
          rustup
          cargo-tauri
          pkg-config
          gcc
          clang
          mold
          gnumake
          cargo-nextest
          cargo-make

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
        ];

      # devcontainer 用のデフォルト値（必要に応じてオーバーライド可能）
      defaultUsername = "vscode";
      defaultHomeDirectory = "/home/${defaultUsername}";
    in
    {
      devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = (commonPackagesFor pkgs) ++ (projectPackagesFor pkgs);
            shellHook = ''
              export CODEX_HOME="$PWD/.codex"
            '';
          };
        }
      );

      formatter = forAllSystems ({ pkgs }: pkgs.nixfmt-tree);

      # devcontainer 内で home-manager switch --flake .#devcontainer で適用
      homeConfigurations.devcontainer = home-manager.lib.homeManagerConfiguration {
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
        modules = [
          ./home.nix
        ];
        extraSpecialArgs = {
          username = defaultUsername;
          homeDirectory = defaultHomeDirectory;
          commonPackages = commonPackagesFor nixpkgs.legacyPackages.x86_64-linux;
        };
      };
    };
}
