if status is-interactive
    # Commands to run in interactive sessions can go here
	starship init fish | source
    alias ls="exa -lha"
    alias ff='nvim $(fzf --preview="batcat --color=always {}")'
    function fish_greeting
        neofetch|lolcat
    end
    #if $(date 
    #    ./update sh
end

function _z_cd
    cd $argv
    or return $status

    commandline -f repaint

    if test "$_ZO_ECHO" = "1"
        echo $PWD
    end
end

function z
    set argc (count $argv)

    if test $argc -eq 0
        _z_cd $HOME
    else if begin; test $argc -eq 1; and test $argv[1] = '-'; end
        _z_cd -
    else
        set -l _zoxide_result (zoxide query -- $argv)
        and _z_cd $_zoxide_result
    end
end

function zi
    set -l _zoxide_result (zoxide query -i -- $argv)
    and _z_cd $_zoxide_result
end


abbr -a za 'zoxide add'

abbr -a zq 'zoxide query'
abbr -a zqi 'zoxide query -i'

abbr -a zr 'zoxide remove'
function zri
    set -l _zoxide_result (zoxide query -i -- $argv)
    and zoxide remove $_zoxide_result
end


function _zoxide_hook --on-variable PWD
    zoxide add (pwd -L)
end


# Created by `pipx` on 2024-08-18 17:38:13
set PATH $PATH /home/groot/.local/bin
