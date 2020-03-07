#!/bin/sh
# -- Written using shell syntax
# -- We are using FUNCNAME array which is a bash-only (FIXME: implement for shell compatibility)

# Originally created by Termux project forked from https://github.com/termux/termux-packages/tree/837f365af8b962d0e809e650d19830d48bf12b89/packages/termux-tools under unlicense based on custom license https://github.com/termux/termux-packages/blob/837f365af8b962d0e809e650d19830d48bf12b89/LICENSE.md which does not clarify a license for shell files.
# Refactored and adapted for Gitpod project by Jacob Hrbek <kreyren@rixotstudio.cz> under GPL-3 (https://www.gnu.org/licenses/gpl-3.0.en.html) license in 2020.

: "
  This is a wrapper for apt-based distributions running on gitpod to allow the end-user to install packages as non-root.
"

# Functions used to handle output
ewarn() { printf 'WARN: %s\n' "$1" ;}
einfo() { printf 'INFO: %s\n' "$1" ;}
eerror() { printf 'ERROR: %s\n' "$1" ;}
efixme() { printf 'FIXME: %s\n' "$1" ;}
die() {
	case "$1" in
		[0-[0-2][0-5][0-5]])
			printf 'FATAL: %s\n' "$2"
			exit "$1" ;;
		fixme) 
			printf 'FIXME: %s\n' "$2"
			exit 36 ;;
		*)
			printf 'BUG: %s\n' "Unknown first argument '$1' has been parsed in die() function"
			exit 255
	esac
}

# Wrappers
ecd() { cd "$1" || die 1 "Unable to change directory to '$1'" ;}

# Directory used to set up the environment
targetDir="$HOME/kreyren"

# Argument resolution
while [ $# -ge 1 ]; do case "$1" in
	proof-of-concept)
		(
			ecd "$HOME/.cache"
			apt-get download "$2"
		)
		mkdir "$targetDir"
		dpkg -x "$2.deb" "$targetDir"
		shift 1 ;;
	help|--help|-help)
		die fixme "HELP_MSG"
		shift 1	;;
	-h|h)
		die fixme "SHORT_HELP_MSG"
		shift 1 ;;
	*) die 2 "Unknown argument '$1' has been parsed"
esac; done


### === ###
