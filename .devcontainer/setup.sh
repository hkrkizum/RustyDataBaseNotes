#!/bin/bash
set -e

NIX_BIN=/nix/var/nix/profiles/default/bin/nix
NIX_PROFILE=/nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh

# --- Nix install or recover ---
if [ ! -x "$NIX_BIN" ]; then
    echo "==> Installing Determinate Nix..."
    nix-installer install linux --no-confirm \
        --extra-conf 'sandbox = false' \
        --extra-conf 'build-users-group ='
else
    echo "==> Nix found in volume. Setting up container environment..."
fi

# --- nix.conf (container filesystem is ephemeral) ---
sudo mkdir -p /etc/nix
printf 'sandbox = false\nbuild-users-group =\n' | sudo tee /etc/nix/nix.conf >/dev/null

# --- systemd service for nix-daemon ---
sudo tee /etc/systemd/system/nix-daemon.service >/dev/null <<'UNIT'
[Unit]
Description=Nix Daemon

[Service]
ExecStart=/nix/var/nix/profiles/default/bin/nix-daemon

[Install]
WantedBy=multi-user.target
UNIT

echo "==> Starting nix-daemon via systemd..."
sudo systemctl daemon-reload
sudo systemctl enable --now nix-daemon

# --- Source Nix profile ---
. "$NIX_PROFILE"

# --- Home Manager ---
echo "==> Applying Home Manager configuration..."
nix run home-manager -- switch --flake .#devcontainer -b backup

# --- Set Home Manager zsh as login shell ---
HM_ZSH="$HOME/.nix-profile/bin/zsh"
if [ -x "$HM_ZSH" ]; then
    echo "$HM_ZSH" | sudo tee -a /etc/shells >/dev/null
    sudo chsh -s "$HM_ZSH" "$(whoami)"
    echo "==> Login shell set to $HM_ZSH"
fi

echo "==> Setup complete!"
