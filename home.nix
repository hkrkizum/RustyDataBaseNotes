{
  pkgs,
  username,
  homeDirectory,
  commonPackages,
  ...
}:
{
  home.username = username;
  home.homeDirectory = homeDirectory;
  home.stateVersion = "25.11";

  programs.home-manager.enable = true;

  # ----------------------------
  # 基本パッケージ
  # ----------------------------
  home.packages = commonPackages;

  # ----------------------------
  # Git
  # ----------------------------
  programs.git = {
    enable = true;

    settings = {
      user.name = "Hikaru Koizumi";
      user.email = "you@example.com";

      init.defaultBranch = "main";
      pull.rebase = false;
      pull.ff = true;
      merge.ff = false;
      push.autoSetupRemote = true;
      core.editor = "vim";
      color.ui = true;

      alias = {
        st = "status -sb";
        co = "checkout";
        cb = "checkout -b";
        br = "branch";
        cm = "commit -m";
        lg = "log --oneline --graph --decorate --all";
      };
    };
  };

  # ----------------------------
  # zsh + Oh My Zsh + Powerlevel10k
  # ----------------------------
  programs.zsh = {
    enable = true;
    enableCompletion = true;
    autosuggestion.enable = true;
    syntaxHighlighting.enable = true;

    history = {
      size = 10000;
      save = 10000;
      ignoreDups = true;
      ignoreSpace = true;
      expireDuplicatesFirst = true;
      extended = true;
      share = true;
    };

    oh-my-zsh = {
      enable = true;
      plugins = [
        "git"
        "fzf"
        "sudo"
      ];
      theme = "powerlevel10k/powerlevel10k";
      custom = "$HOME/.oh-my-zsh/custom";
    };

    shellAliases = {
      ll = "eza -al --group-directories-first --icons=auto";
      la = "eza -a --group-directories-first --icons=auto";
      lt = "eza --tree --level=2 --icons=auto";
      ls = "eza --group-directories-first --icons=auto";

      cat = "bat";
      batcat = "bat";
      find = "fd";

      grep = "rg";
      v = "vim";
    };

    initContent = ''
      # powerlevel10k
      [[ -r ~/.p10k.zsh ]] && source ~/.p10k.zsh
    '';
  };

  # Oh My Zsh から powerlevel10k を見える場所へ配置
  home.file.".oh-my-zsh/custom/themes/powerlevel10k".source =
    "${pkgs.zsh-powerlevel10k}/share/zsh-powerlevel10k";

  # 最小の powerlevel10k 設定
  home.file.".p10k.zsh".text = ''
    typeset -g POWERLEVEL9K_INSTANT_PROMPT=quiet
    typeset -g POWERLEVEL9K_MODE=nerdfont-complete
    typeset -g POWERLEVEL9K_PROMPT_ON_NEWLINE=true
    typeset -g POWERLEVEL9K_MULTILINE_FIRST_PROMPT_PREFIX=""
    typeset -g POWERLEVEL9K_MULTILINE_LAST_PROMPT_PREFIX="❯ "
    typeset -g POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(dir vcs)
    typeset -g POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(status command_execution_time background_jobs time)
    typeset -g POWERLEVEL9K_SHORTEN_STRATEGY=truncate_to_unique
    typeset -g POWERLEVEL9K_TIME_FORMAT="%H:%M:%S"
  '';

  # ----------------------------
  # Modern CLI tools
  # ----------------------------
  programs.eza = {
    enable = true;
    enableZshIntegration = true;
    git = true;
    icons = "auto";
  };

  programs.fzf = {
    enable = true;
    enableZshIntegration = true;
  };

  programs.bat.enable = true;
  programs.fd.enable = true;
  programs.ripgrep.enable = true;

  programs.direnv = {
    enable = true;
    enableZshIntegration = true;
    nix-direnv.enable = true;
  };

  # ----------------------------
  # Environment variables
  # ----------------------------
  home.sessionPath = [
    "$HOME/.local/share/pnpm"
  ];

  home.sessionVariables = {
    EDITOR = "vim";
    VISUAL = "vim";
    PAGER = "bat";
    PNPM_HOME = "$HOME/.local/share/pnpm";
  };
}
