FROM gitpod/workspace-full-vnc

USER gitpod

RUN brew install tcl-tk

RUN sudo apt-get update \
    && sudo apt-get install -y \
        shellcheck \
        tk-dev \
        python-tk \
        python3-tk

RUN npm i -g bash-language-server

RUN pyenv install 3.6.2 && pyenv global 3.6.2
