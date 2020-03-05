FROM debian:stable

# FIXME: Outputs `gitpod@ws-ce281d58-997b-44b8-9107-3f2da7feede3:/workspace/gitpod-tests1$` in terminal

# To avoid bricked workspaces (https://github.com/gitpod-io/gitpod/issues/1171)
ARG DEBIAN_FRONTEND=noninteractive

USER root

ENV LANG=en_US.UTF-8
ENV LC_ALL=C

# Sudo
RUN true \
    && apt-get update \
    && apt-get install -y sudo \
    && sed -i.bkp -e 's/%sudo\s\+ALL=(ALL\(:ALL\)\?)\s\+ALL/%sudo ALL=NOPASSWD:ALL/g' /etc/sudoers

# Add 'gitpod' user
RUN true \
	&& useradd \
		--uid 33333 \
		--groups sudo \
		--create-home --home-dir /home/gitpod \
		--shell /bin/bash \
		--password gitpod \
		gitpod || exit 1

# Install dependencies
RUN true \
	&& : "Update repositories if needed" \
	&& if ! apt list | grep -qP "^shellcheck\s{1}-.*|^python3\s{1}-.*"; then apt-get update; fi \
	&& : "Install dependencies if needed" \
	&& if ! apt list --installed | grep -oP "^shellcheck -.*"; then apt-get install -y shellcheck; fi \
	&& if ! apt list --installed | grep -oP "^python3 -.*"; then apt-get install -y python3; fi \
	&& if ! apt list --installed | grep -oP "^npm -.*"; then apt-get install -y npm; fi \
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

# Run echo so that it does not bother us later
RUN sudo printf '%s\n' "sudo ping"

# Cache npm
RUN npm i -g bash-language-server