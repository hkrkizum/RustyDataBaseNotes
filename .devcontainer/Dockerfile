FROM mcr.microsoft.com/devcontainers/base:ubuntu

# systemd を PID 1 として使うためにインストール
RUN apt-get update \
    && apt-get install -y --no-install-recommends systemd systemd-sysv \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Determinate Nix installer をダウンロードだけしておく（/nix は volume で上書きされるため実行は postCreateCommand で行う）
RUN curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix -o /usr/local/bin/nix-installer \
    && chmod +x /usr/local/bin/nix-installer

# Nix + Home Manager のプロファイルパスを PATH に追加
ENV PATH="/home/vscode/.nix-profile/bin:/nix/var/nix/profiles/default/bin:${PATH}"

# systemd を PID 1 として起動
CMD ["/sbin/init"]
