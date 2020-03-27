#! /bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

generate_report() {
    local crate=$1
    cd $DIR/../target/debug/deps/
    for file in *-*.d; do
        local test_executable=${file%.*}
        local current=$(echo $file | cut -d- -f1)

        if ! [ "$current" = "$crate" ]; then
            continue
        fi        # Create directory for the coverage report

        mkdir -p ../test-coverage/$crate        # Create coverage report

        kcov\
            --include-pattern=react/$crate\
            --exclude-line=unreachable\
            --exclude-region='/* Exclude from code coverage - begin */:/* Exclude from code coverage - end */'\
            ../test-coverage/$crate\
            ${test_executable}
    done
}

(generate_report "route53_dyndns")
