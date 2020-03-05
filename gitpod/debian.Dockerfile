FROM debian:latest

# FIXME: Outputs `gitpod@ws-ce281d58-997b-44b8-9107-3f2da7feede3:/workspace/gitpod-tests1$` in terminal

# To avoid bricked workspaces (https://github.com/gitpod-io/gitpod/issues/1171)
ARG DEBIAN_FRONTEND=noninteractive

USER root

ENV LANG=en_US.UTF-8
ENV LC_ALL=C

# Add 'gitpod' user
RUN useradd \
	--uid 33333 \
	--create-home --home-dir /home/gitpod \
	--shell /bin/bash \
	--password gitpod \
	gitpod || exit 1

# Sudo
RUN apt-get update && apt-get install -y sudo
# Run echo so that it does not bother us later
RUN sudo true

# ### Rust ###
# RUN true \
#     # FIXME: Sanitize
#     && apt-get update \
#     # FIXME: Sanitize
#     && apt-get install -yq \
#         # Enable Rust static binary builds
#         musl \
#         musl-dev \
#         musl-tools \
#     && cp /home/gitpod/.profile /home/gitpod/.profile_orig && \
#     && curl -fsSL https://sh.rustup.rs | sh -s -- -y \
#     && .cargo/bin/rustup toolchain install 1.41.1 \
#     && .cargo/bin/rustup default 1.41.1 \
#     # Save image size by removing now-redudant stable toolchain
#     && .cargo/bin/rustup toolchain uninstall stable \
#     && .cargo/bin/rustup component add \
#         rls \
#         rust-analysis \
#         rust-src \
#     && .cargo/bin/rustup completions bash | sudo tee /etc/bash_completion.d/rustup.bash-completion > /dev/null \
#     && .cargo/bin/rustup target add x86_64-unknown-linux-musl \
#     && grep -v -F -x -f /home/gitpod/.profile_orig /home/gitpod/.profile > /home/gitpod/.bashrc.d/80-rust \
#     && bash -lc "cargo install cargo-watch cargo-edit cargo-tree"

# Install dependencies
RUN true \
    && : "Update repositories if needed" \
    && if ! apt-cache search shellcheck | grep -qP "^shellcheck -.*"; then apt-get update; fi \
    && : "Install dependencies if needed" \
    && if ! apt list --installed | grep -oP "^shellcheck -.*"; then apt-get install -y shellcheck; fi \
    && : "Clean repositories" \
    && apt-get autoremove -y \
    && : "Remove lists" \
    && rm -rf /var/lib/apt/lists/*


# Add custom functions
RUN if ! grep -qF 'ix()' /etc/bash.bashrc; then printf '%s\n' \
	'# Custom' \
	"ix() { curl -F 'f:1=<-' ix.io 2>/dev/null ;}" \
	>> /etc/bash.bashrc; fi

USER gitpod

### Python ###
# FIXME-QA: Do not use pip? https://chriswarrick.com/blog/2018/09/04/python-virtual-environments/
ENV PATH="$HOME/.pyenv/bin:$HOME/.pyenv/shims:$PATH"
RUN true \
    && [ ! -d "$HOME/.bashrc.d" ] && mkdir "$HOME/.bashrc.d" \
    && sudo apt-get update \
    && sudo apt-get install -y --no-install-recommends make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev \
    && curl -fsSL https://github.com/pyenv/pyenv-installer/raw/master/bin/pyenv-installer | bash \
    # FIXME: Sanitize
    &&  printf '%s\n' \
          'eval "$(pyenv init -)"' \
          'eval "$(pyenv virtualenv-init -)"' \
        >> /home/gitpod/.bashrc.d/60-python \
    && pyenv install 2.7.17 \
    && pyenv install 3.7.6 \
    && pyenv global 2.7.17 3.7.6 \
    && pip2 install --upgrade pip \
    && pip2 install virtualenv pipenv pylint rope flake8 autopep8 pep8 pylama pydocstyle bandit notebook python-language-server[all]==0.25.0 \
    && pip3 install --upgrade pip \
    && pip3 install virtualenv pipenv pylint rope flake8 mypy autopep8 pep8 pylama pydocstyle bandit notebook python-language-server[all]==0.25.0 \
    && sudo rm -rf /tmp/*
# Gitpod will automatically add user site under `/workspace` to persist your packages.
# ENV PYTHONUSERBASE=/workspace/.pip-modules \
#    PIP_USER=yes

# Cache npm
RUN npm i -g bash-language-server