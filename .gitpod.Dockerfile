FROM gitpod/workspace-full

USER gitpod

RUN brew install shellcheck

RUN npm i -g bash-language-server
