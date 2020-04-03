#! /bin/bash

set -e

# Go to project root folder
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $DIR/../../

generate_report() {
    local crate=$1
    for file in ./target/debug/deps/*-*.d; do
        local test_executable=${file%.*}
        local current=$(echo $file | cut -d- -f1 | awk -F/ '{print $NF}')

        if ! [ "$current" = "$crate" ]; then
            continue
        fi        # Create directory for the coverage report

        mkdir -p ./target/cov/

        kcov\
            --exclude-line=unreachable \
            --exclude-pattern=/.cargo,/usr/lib \
            --exclude-region='/* Exclude from code coverage - begin */:/* Exclude from code coverage - end */' \
            ./target/cov \
            ${test_executable}
    done
}

(generate_report "route53_dyndns")
