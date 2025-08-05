# ~/.bashrc: executed by bash(1) for non-login shells.
# see /usr/share/doc/bash/examples/startup-files (in the package bash-doc)
# for examples

# If not running interactively, don't do anything
case $- in
*i*) ;;
*) return ;;
esac

# don't put duplicate lines or lines starting with space in the history.
# See bash(1) for more options
HISTCONTROL=ignoreboth

# append to the history file, don't overwrite it
shopt -s histappend

# for setting history length see HISTSIZE and HISTFILESIZE in bash(1)
HISTSIZE=1000
HISTFILESIZE=2000

# check the window size after each command and, if necessary,
# update the values of LINES and COLUMNS.
shopt -s checkwinsize

# If set, the pattern "**" used in a pathname expansion context will
# match all files and zero or more directories and subdirectories.
#shopt -s globstar

# make less more friendly for non-text input files, see lesspipe(1)
[ -x /usr/bin/lesspipe ] && eval "$(SHELL=/bin/sh lesspipe)"

# make man better with less
#export MANPAGER="less -R --use-color -Dd+r -Du+b"
export MANPAGER="nvim -c Man!"

# set variable identifying the chroot you work in (used in the prompt below)
if [ -z "${debian_chroot:-}" ] && [ -r /etc/debian_chroot ]; then
    debian_chroot=$(cat /etc/debian_chroot)
fi

# set a fancy prompt (non-color, unless we know we "want" color)
case "$TERM" in
xterm-color | *-256color) color_prompt=yes ;;
esac

# uncomment for a colored prompt, if the terminal has the capability; turned
# off by default to not distract the user: the focus in a terminal window
# should be on the output of commands, not on the prompt
#force_color_prompt=yes

if [ -n "$force_color_prompt" ]; then
    if [ -x /usr/bin/tput ] && tput setaf 1 >&/dev/null; then
        # We have color support; assume it's compliant with Ecma-48
        # (ISO/IEC-6429). (Lack of such support is extremely rare, and such
        # a case would tend to support setf rather than setaf.)
        color_prompt=yes
    else
        color_prompt=
    fi
fi

if [ "$color_prompt" = yes ]; then
    PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '
else
    PS1='${debian_chroot:+($debian_chroot)}\u@\h:\w\$ '
fi
unset color_prompt force_color_prompt

# If this is an xterm set the title to user@host:dir
case "$TERM" in
xterm* | rxvt*)
    PS1="\[\e]0;${debian_chroot:+($debian_chroot)}\u@\h: \w\a\]$PS1"
    ;;
*) ;;
esac

# enable color support of ls and also add handy aliases
if [ -x /usr/bin/dircolors ]; then
    test -r ~/.dircolors && eval "$(dircolors -b ~/.dircolors)" || eval "$(dircolors -b)"
    #alias ls='ls --color=auto'
    #alias dir='dir --color=auto'
    #alias vdir='vdir --color=auto'

    alias grep='grep --color=auto'
    alias fgrep='fgrep --color=auto'
    alias egrep='egrep --color=auto'
fi

# Add distro detection
function get_distro() {
    if [ -f /etc/os-release ]; then
        source /etc/os-release
        echo "$NAME"
    else
        echo "Unknown"
    fi
}
export STARSHIP_DISTRO="$(get_distro)"

# Set the default editor to NVIM
export EDITOR='nvim'
export VISUAL='nvim'
export WINE_BACKEND='wayland'
export WLR_RENDERER='vulkan'
export PATH="$HOME/.local/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"
export QT_QPA_PLATFORM='wayland'
export QT_QPA_PLATFORMTHEME='qt6ct'

# colored GCC warnings and errors
export GCC_COLORS='error=01;31:warning=01;35:note=01;36:caret=01;32:locus=01:quote=01'

# some more ls aliases
alias ls='exa -lah'
alias ll='exa -alF'
alias la='exa -A'
alias l='exa -CF'
alias cat='bat'
alias grep='rg'
alias mc='mc -X'
alias lz='lazygit'

# Add an "alert" alias for long running commands.  Use like so:
#   sleep 10; alert
alias alert='notify-send --urgency=low -i "$([ $? = 0 ] && echo terminal || echo error)" "$(history|tail -n1|sed -e '\''s/^\s*[0-9]\+\s*//;s/[;&|]\s*alert$//'\'')"'

# Alias definitions.
# You may want to put all your additions into a separate file like
# ~/.bash_aliases, instead of adding them here directly.
# See /usr/share/doc/bash-doc/examples in the bash-doc package.

if [ -f ~/.bash_aliases ]; then
    . ~/.bash_aliases
fi

# enable programmable completion features (you don't need to enable
# this, if it's already enabled in /etc/bash.bashrc and /etc/profile
# sources /etc/bash.bashrc).
if ! shopt -oq posix; then
    if [ -f /usr/share/bash-completion/bash_completion ]; then
        . /usr/share/bash-completion/bash_completion
    elif [ -f /etc/bash_completion ]; then
        . /etc/bash_completion
    fi
fi
# Starship
eval -- "$(/usr/sbin/starship init bash --print-full-init)"

# Zoxide
eval "$(zoxide init bash)"

#Yazi
function y() {
	local tmp="$(mktemp -t "yazi-cwd.XXXXXX")" cwd
	yazi "$@" --cwd-file="$tmp"
	IFS= read -r -d '' cwd < "$tmp"
	[ -n "$cwd" ] && [ "$cwd" != "$PWD" ] && builtin cd -- "$cwd"
	rm -f -- "$tmp"
}

# vifm
export VIFM="$HOME/.config/vifm"
alias v='vifm'
alias vcd='vifmcd'

# Fuzzy Finder
[ -f /usr/share/fzf/key-bindings.bash ] && source /usr/share/fzf/key-bindings.bash
[ -f /usr/share/fzf/completion.bash ] && source /usr/share/fzf/completion.bash
export FZF_DEFAULT_OPTS='--tiebreak=index --preview "bat --color=always --style=numbers {}" --preview-window=right:50%'

# FZF Rust integration
alias frs='find . -type f -name "*.rs" | fzf --preview "bat --color=always {}" | xargs nvim'
alias fxls='find . -type f -name "*.xlsx" | fzf --preview "xlsx2csv {} | head -n 10" | xargs cargo run -- --file'
alias fcb='cargo build -- $(find src -type f -name "*.rs" | fzf --preview "bat --color=always {}")'

#FastFetch
fastfetch

# Dysk
dysk

# AUTOMATICALLY GENERATED by `shtab`



_shtab_tldr_option_strings=('-h' '--help' '-v' '--version' '--search' '-u' '--update' '--update_cache' '-k' '--clear-cache' '-p' '--platform' '-l' '--list' '-s' '--source' '-c' '--color' '-r' '--render' '-L' '--language' '-m' '--markdown' '--short-options' '--long-options' '--print-completion')

_shtab_tldr_pos_0_COMPGEN=shtab_tldr_cmd_list

_shtab_tldr__p_choices=('android' 'freebsd' 'linux' 'netbsd' 'openbsd' 'osx' 'sunos' 'windows' 'common')
_shtab_tldr___platform_choices=('android' 'freebsd' 'linux' 'netbsd' 'openbsd' 'osx' 'sunos' 'windows' 'common')
_shtab_tldr___print_completion_choices=('bash' 'zsh' 'tcsh')

_shtab_tldr_pos_0_nargs=*
_shtab_tldr__h_nargs=0
_shtab_tldr___help_nargs=0
_shtab_tldr__v_nargs=0
_shtab_tldr___version_nargs=0
_shtab_tldr__u_nargs=0
_shtab_tldr___update_nargs=0
_shtab_tldr___update_cache_nargs=0
_shtab_tldr__k_nargs=0
_shtab_tldr___clear_cache_nargs=0
_shtab_tldr__l_nargs=0
_shtab_tldr___list_nargs=0
_shtab_tldr__c_nargs=0
_shtab_tldr___color_nargs=0
_shtab_tldr__r_nargs=0
_shtab_tldr___render_nargs=0
_shtab_tldr__m_nargs=0
_shtab_tldr___markdown_nargs=0
_shtab_tldr___short_options_nargs=0
_shtab_tldr___long_options_nargs=0


# Custom Preamble
shtab_tldr_cmd_list(){
          compgen -W "$("/usr/bin/python" -m tldr --list | sed 's/[^[:alnum:]_]/ /g')" -- "$1"
        }
# End Custom Preamble

# $1=COMP_WORDS[1]
_shtab_compgen_files() {
  compgen -f -- $1  # files
}

# $1=COMP_WORDS[1]
_shtab_compgen_dirs() {
  compgen -d -- $1  # recurse into subdirs
}

# $1=COMP_WORDS[1]
_shtab_replace_nonword() {
  echo "${1//[^[:word:]]/_}"
}

# set default values (called for the initial parser & any subparsers)
_set_parser_defaults() {
  local subparsers_var="${prefix}_subparsers[@]"
  sub_parsers=${!subparsers_var-}

  local current_option_strings_var="${prefix}_option_strings[@]"
  current_option_strings=${!current_option_strings_var}

  completed_positional_actions=0

  _set_new_action "pos_${completed_positional_actions}" true
}

# $1=action identifier
# $2=positional action (bool)
# set all identifiers for an action's parameters
_set_new_action() {
  current_action="${prefix}_$(_shtab_replace_nonword $1)"

  local current_action_compgen_var=${current_action}_COMPGEN
  current_action_compgen="${!current_action_compgen_var-}"

  local current_action_choices_var="${current_action}_choices[@]"
  current_action_choices="${!current_action_choices_var-}"

  local current_action_nargs_var="${current_action}_nargs"
  if [ -n "${!current_action_nargs_var-}" ]; then
    current_action_nargs="${!current_action_nargs_var}"
  else
    current_action_nargs=1
  fi

  current_action_args_start_index=$(( $word_index + 1 - $pos_only ))

  current_action_is_positional=$2
}

# Notes:
# `COMPREPLY`: what will be rendered after completion is triggered
# `completing_word`: currently typed word to generate completions for
# `${!var}`: evaluates the content of `var` and expand its content as a variable
#     hello="world"
#     x="hello"
#     ${!x} -> ${hello} -> "world"
_shtab_tldr() {
  local completing_word="${COMP_WORDS[COMP_CWORD]}"
  local completed_positional_actions
  local current_action
  local current_action_args_start_index
  local current_action_choices
  local current_action_compgen
  local current_action_is_positional
  local current_action_nargs
  local current_option_strings
  local sub_parsers
  COMPREPLY=()

  local prefix=_shtab_tldr
  local word_index=0
  local pos_only=0 # "--" delimeter not encountered yet
  _set_parser_defaults
  word_index=1

  # determine what arguments are appropriate for the current state
  # of the arg parser
  while [ $word_index -ne $COMP_CWORD ]; do
    local this_word="${COMP_WORDS[$word_index]}"

    if [[ $pos_only = 1 || " $this_word " != " -- " ]]; then
      if [[ -n $sub_parsers && " ${sub_parsers[@]} " == *" ${this_word} "* ]]; then
        # valid subcommand: add it to the prefix & reset the current action
        prefix="${prefix}_$(_shtab_replace_nonword $this_word)"
        _set_parser_defaults
      fi

      if [[ " ${current_option_strings[@]} " == *" ${this_word} "* ]]; then
        # a new action should be acquired (due to recognised option string or
        # no more input expected from current action);
        # the next positional action can fill in here
        _set_new_action $this_word false
      fi

      if [[ "$current_action_nargs" != "*" ]] && \
         [[ "$current_action_nargs" != "+" ]] && \
         [[ "$current_action_nargs" != *"..." ]] && \
         (( $word_index + 1 - $current_action_args_start_index - $pos_only >= \
            $current_action_nargs )); then
        $current_action_is_positional && let "completed_positional_actions += 1"
        _set_new_action "pos_${completed_positional_actions}" true
      fi
    else
      pos_only=1 # "--" delimeter encountered
    fi

    let "word_index+=1"
  done

  # Generate the completions

  if [[ $pos_only = 0 && "${completing_word}" == -* ]]; then
    # optional argument started: use option strings
    COMPREPLY=( $(compgen -W "${current_option_strings[*]}" -- "${completing_word}") )
  else
    # use choices & compgen
    local IFS=$'\n' # items may contain spaces, so delimit using newline
    COMPREPLY=( $([ -n "${current_action_compgen}" ] \
                  && "${current_action_compgen}" "${completing_word}") )
    unset IFS
    COMPREPLY+=( $(compgen -W "${current_action_choices[*]}" -- "${completing_word}") )
  fi

  return 0
}

complete -o filenames -F _shtab_tldr tldr
