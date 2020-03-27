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

        mkdir -p $DIR/../target/cov/

        kcov\
            --exclude-line=unreachable \
            --exclude-pattern=/.cargo,/usr/lib \
            --exclude-region='/* Exclude from code coverage - begin */:/* Exclude from code coverage - end */' \
            $DIR/../target/cov \
            ${test_executable}
    done
}

(generate_report "route53_dyndns")

# Upload coverage to codecov
bash <(curl -s https://codecov.io/bash) &&
echo "Uploaded code coverage"
