FROM debian:stable

# FIXME: Outputs `gitpod@ws-ce281d58-997b-44b8-9107-3f2da7feede3:/workspace/gitpod-tests1$` in terminal

# To avoid bricked workspaces (https://github.com/gitpod-io/gitpod/issues/1171)
ARG DEBIAN_FRONTEND=noninteractive

USER root

ENV LANG=en_US.UTF-8
ENV LC_ALL=C
# overwrite this env variable to use a different window manager
ENV WINDOW_MANAGER="openbox"

# Add VNC startup script
COPY gitpod/start-vnc-session.bash /usr/bin/start-vnc-session
RUN chmod +x /usr/bin/start-vnc-session

# Sudo
RUN true \
    # Make sure sudo is available
    && if ! apt list | grep -qP "^sudo\s{1}-.*"; then apt-get update; fi \
    # Install sudo if needed
    && if ! apt list --installed | grep -oP "^sudo\s{1}-.*"; then apt-get install -y sudo; fi \
    # FIXME: Sanitize
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
	&& if ! apt list --installed | grep -oP "^python3-pip -.*"; then apt-get install -y python3-pip; fi \
	&& if ! apt list --installed | grep -oP "^python3-tk -.*"; then apt-get install -y python3-tk; fi \
	&& if ! apt list --installed | grep -oP "^npm -.*"; then apt-get install -y npm; fi \
	&& if ! apt list --installed | grep -oP "^novnc -.*"; then apt-get install -y novnc; fi \
	# HOTFIX! Expected to allow change the window manager base on change of WINDOW_MANAGER envvar
	&& if ! apt list --installed | grep -oP "^openbox -.*"; then apt-get install -y openbox; fi \
	&& : "Clean repositories" \
	&& apt-get autoremove -y
	# DO_NOT_MERGE: Disabled for experiment
	# && : "Remove lists" \
	# && rm -rf /var/lib/apt/lists/*

# Add custom functions
RUN if ! grep -qF 'ix()' /etc/bash.bashrc; then printf '%s\n' \
	'# Custom' \
	"ix() { curl -F 'f:1=<-' ix.io 2>/dev/null ;}" \
	>> /etc/bash.bashrc; fi

USER gitpod

# Run echo so that it does not bother us later
RUN sudo printf '%s\n' "sudo ping"

# Management of bashrc
RUN if ! grep -qF "@gitpod" "$HOME/.bashrc"; then printf '%s\n' \
  "# shellcheck disable=SC2155 # FIXME?" \
  "# shellcheck disable=SC2025 # Does not apply to this usage" \
  "# shellcheck disable=SC1117 # Does not apply to this usage" \
  "export PS1=\"\033[1m\e[38;5;201m[ \t : \w : EXIT \$? ]\033[0m\e[38;5;011m\n\u@gitpod \\$ \[$(tput sgr0)\]\"" \
  >> "$HOME/.bashrc"; fi

# Cache npm
## FIXME: fails
#RUN npm i -g bash-language-server