FROM moby/buildkit:v0.33.1 AS buildkit-client

FROM archlinux:latest

ARG user=remote
ENV USERNAME=$user
ENV HOME=/home/$user

COPY --from=buildkit-client /usr/bin/buildctl /usr/local/bin/buildctl

RUN pacman -Syu --noconfirm \
  tmux \
  openssh \
  zsh \
  git \
  zoxide \
  starship \
  nvim \
  go \
  rustup \
  nodejs \
  npm \
  jdk-openjdk \
  python \
  python-pip \
  php \
  wget \
  unzip \
  ripgrep \
  jq \
  ast-grep \
  luarocks \
  ruby \
  julia \
  tree-sitter \
  tree-sitter-cli \
  base-devel \
  curl \
  tar \
  && useradd --create-home --shell /bin/zsh $USERNAME \
  && passwd -d $USERNAME \
  && mkdir -p /home/$USERNAME/.ssh \
  && chown $USERNAME:$USERNAME /home/$USERNAME/.ssh \
  && chmod 700 /home/$USERNAME/.ssh \
  && pacman -Scc --noconfirm

COPY --chown=$user:$user authorized_keys /home/$USERNAME/.ssh/authorized_keys
COPY --chown=$user:$user .zshrc /home/$USERNAME/.zshrc
COPY --chown=$user:$user .tmux.conf /home/$USERNAME/.tmux.conf
COPY --chown=$user:$user nvim /home/$USERNAME/.config/nvim
COPY entrypoint.sh /usr/local/bin/workspace-entrypoint

RUN chmod 0555 /usr/local/bin/workspace-entrypoint

USER $user
RUN RUNZSH=no CHSH=no KEEP_ZSHRC=yes \
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" \
  && mkdir -p "$HOME/.local/bin" \
  && GOBIN="$HOME/.local/bin" go install github.com/tmuxpack/tpack/cmd/tpack@latest \
  && "$HOME/.local/bin/tpack" install \
  && rustup default stable \
  && nvim --headless '+qa' \
  && nvim --headless '+MasonToolsInstallSync' '+qa'

EXPOSE 22
# Exposer le port SSH

USER root 
ENTRYPOINT ["/usr/local/bin/workspace-entrypoint"]
CMD ["/usr/bin/sshd", "-D", "-e"]
