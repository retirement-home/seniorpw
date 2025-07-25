_senior_complete_entries () {
    declare storearg=()
    for i in "${!COMP_WORDS[@]}"; do
        if [[ ${COMP_WORDS[$i]} = "-s" || ${COMP_WORDS[$i]} = "--store" ]]; then
            storearg=("--store" ${COMP_WORDS[(($i + 1))]})
        fi
    done
    local prefix="$(senior ${storearg[@]} print-dir)"
    prefix="${prefix%/}/"
    local suffix=".age"
    local autoexpand=${1:-0}
    local dirsonly=${2:-0}

    local IFS=$'\n'
    local items=($(compgen -f $prefix$cur))

    # Remember the value of the first item, to see if it is a directory. If
    # it is a directory, then don't add a space to the completion
    local firstitem=""
    # Use counter, can't use ${#items[@]} as we skip hidden directories
    local i=0 item

    for item in ${items[@]}; do
        [[ $item =~ /\.[^/]*$ ]] && continue

        # if there is a unique match, and it is a directory with one entry
        # autocomplete the subentry as well (recursively)
        if [[ ${#items[@]} -eq 1 && $autoexpand -eq 1 ]]; then
            while [[ -d $item ]]; do
                local subitems=($(compgen -f "$item/"))
                local filtereditems=( ) item2
                for item2 in "${subitems[@]}"; do
                    [[ $item2 =~ /\.[^/]*$ ]] && continue
                    filtereditems+=( "$item2" )
                done
                if [[ ${#filtereditems[@]} -eq 1 ]]; then
                    item="${filtereditems[0]}"
                else
                    break
                fi
            done
        fi

        # append / to directories
        [[ -d $item ]] && item="$item/"

	# directories only
	[[ $dirsonly -eq 1 && ! -d $item ]] && continue

        item="${item%$suffix}"
        COMPREPLY+=("${item#$prefix}")
        if [[ $i -eq 0 ]]; then
            firstitem=$item
        fi
        let i+=1
    done

    # The only time we want to add a space to the end is if there is only
    # one match, and it is not a directory
    if [[ $i -gt 1 || ( $i -eq 1 && -d $firstitem ) ]]; then
        compopt -o nospace
    fi
}

_senior() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="senior"
                ;;
            senior,add-recipient)
                cmd="senior__add__recipient"
                ;;
            senior,cat)
                cmd="senior__cat"
                ;;
            senior,change-passphrase)
                cmd="senior__change__passphrase"
                ;;
            senior,clone)
                cmd="senior__clone"
                ;;
            senior,edit)
                cmd="senior__edit"
                ;;
            senior,git)
                cmd="senior__git"
                ;;
            senior,grep)
                cmd="senior__grep"
                ;;
            senior,help)
                cmd="senior__help"
                ;;
            senior,init)
                cmd="senior__init"
                ;;
            senior,mv)
                cmd="senior__mv"
                ;;
            senior,print-dir)
                cmd="senior__print__dir"
                ;;
            senior,reencrypt)
                cmd="senior__reencrypt"
                ;;
            senior,rm)
                cmd="senior__rm"
                ;;
            "senior,show"|"senior,s")
                cmd="senior__show"
                ;;
            senior,unlock)
                cmd="senior__unlock"
                ;;
            senior__help,add-recipient)
                cmd="senior__help__add__recipient"
                ;;
            senior__help,cat)
                cmd="senior__help__cat"
                ;;
            senior__help,change-passphrase)
                cmd="senior__help__change__passphrase"
                ;;
            senior__help,clone)
                cmd="senior__help__clone"
                ;;
            senior__help,edit)
                cmd="senior__help__edit"
                ;;
            senior__help,git)
                cmd="senior__help__git"
                ;;
            senior__help,grep)
                cmd="senior__help__grep"
                ;;
            senior__help,help)
                cmd="senior__help__help"
                ;;
            senior__help,init)
                cmd="senior__help__init"
                ;;
            senior__help,mv)
                cmd="senior__help__mv"
                ;;
            senior__help,print-dir)
                cmd="senior__help__print__dir"
                ;;
            senior__help,reencrypt)
                cmd="senior__help__reencrypt"
                ;;
            senior__help,rm)
                cmd="senior__help__rm"
                ;;
            senior__help,show)
                cmd="senior__help__show"
                ;;
            senior__help,unlock)
                cmd="senior__help__unlock"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        senior)
            opts="-s -h -V --store --help --version init clone edit show mv rm print-dir git add-recipient reencrypt change-passphrase grep cat unlock help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            local prefix="$(senior print-dir)"
            prefix="$(dirname "$prefix")"
            local stores=( ${prefix}/* )
            stores=( ${stores[@]#"$prefix"/} )
            case "${prev}" in
                --store)
                    COMPREPLY=( $(compgen -W "$(echo ${stores[@]})" -- "${cur}") )
                    return 0
                    ;;
                -s)
                    COMPREPLY=( $(compgen -W "$(echo ${stores[@]})" -- "${cur}") )
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__add__recipient)
            opts="-h --help <PUBLIC KEY> <ALIAS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__cat)
            _senior_complete_entries 0 1
            return 0
            ;;
        senior__change__passphrase)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__clone)
            opts="-i -h --identity --help <ADDRESS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --identity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__edit)
            _senior_complete_entries
            return 0
            ;;
        senior__git)
            COMPREPLY+=($(compgen -W "init push pull config log reflog rebase" -- ${cur}))
            ;;
        senior__grep)
            opts="-h --help <PATTERN_OR_CMD> [ARGS]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help)
            opts="init clone edit show mv rm print-dir git add-recipient reencrypt change-passphrase grep unlock help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__add__recipient)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__cat)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__change__passphrase)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__clone)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__edit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__git)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__grep)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__mv)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__print__dir)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__reencrypt)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__rm)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__help__unlock)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__init)
            opts="-i -a -h --identity --recipient-alias --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --identity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --recipient-alias)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__mv)
            _senior_complete_entries
            ;;
        senior__print__dir)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__reencrypt)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        senior__rm)
            COMPREPLY+=($(compgen -W "-r --recursive" -- ${cur}))
            _senior_complete_entries
            return 0
            ;;
        senior__show)
            opts="-k -c -h --key --clip --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                _senior_complete_entries 1
                return 0
            fi
            case "${prev}" in
                --key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    ;;
                -k)
                    COMPREPLY=($(compgen -f "${cur}"))
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            _senior_complete_entries 1
            return 0
            ;;
        senior__unlock)
            opts="-h --check --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

#complete -F _senior -o bashdefault -o default senior
complete -o filenames -F _senior senior

