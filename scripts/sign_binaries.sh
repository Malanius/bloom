#!/bin/bash

##
#  Bloom
#
#  HTTP REST API caching middleware
#  Copyright: 2022, Valerian Saliou <valerian@valeriansaliou.name>
#  License: Mozilla Public License v2.0 (MPL v2.0)
##

# Read arguments
while [ "$1" != "" ]; do
    argument_key=`echo $1 | awk -F= '{print $1}'`
    argument_value=`echo $1 | awk -F= '{print $2}'`

    case $argument_key in
        -v | --version)
            # Notice: strip any leading 'v' to the version number
            BLOOM_VERSION="${argument_value/v}"
            ;;
        *)
            echo "Unknown argument received: '$argument_key'"
            exit 1
            ;;
    esac

    shift
done

# Ensure release version is provided
if [ -z "$BLOOM_VERSION" ]; then
  echo "No Bloom release version was provided, please provide it using '--version'"

  exit 1
fi

# Define sign pipeline
function sign_for_architecture {
    final_tar="v$BLOOM_VERSION-$1.tar.gz"
    gpg_signer="valerian@valeriansaliou.name"

    gpg -u "$gpg_signer" --armor --detach-sign "$final_tar"
    sign_result=$?

    if [ $sign_result -eq 0 ]; then
        echo "Result: Signed architecture: $1 for file: $final_tar"
    fi

    return $sign_result
}

# Run sign tasks
ABSPATH=$(cd "$(dirname "$0")"; pwd)
BASE_DIR="$ABSPATH/../"

rc=0

pushd "$BASE_DIR" > /dev/null
    echo "Executing sign steps for Bloom v$BLOOM_VERSION..."

    sign_for_architecture "amd64" && \
        sign_for_architecture "armhf"
    rc=$?

    if [ $rc -eq 0 ]; then
        echo "Success: Done executing sign steps for Bloom v$BLOOM_VERSION"
    else
        echo "Error: Failed executing sign steps for Bloom v$BLOOM_VERSION"
    fi
popd > /dev/null

exit $rc
