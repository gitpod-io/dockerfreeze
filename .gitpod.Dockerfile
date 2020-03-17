FROM gitpod/workspace-full-vnc

USER gitpod

RUN sudo apt-get update \
    && sudo apt-get install -y \
        shellcheck \
        tk-dev

RUN npm i -g bash-language-server
