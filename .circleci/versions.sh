#!/bin/bash

# Function taken from here for semver comparison
# https://gist.github.com/Ariel-Rodriguez/9e3c2163f4644d7a389759b224bfe7f3
semver_compare() {
  local version_a version_b pr_a pr_b
  # strip word "v" and extract first subset version (x.y.z from x.y.z-foo.n)
  version_a=$(echo "${1//v/}" | awk -F'-' '{print $1}')
  version_b=$(echo "${2//v/}" | awk -F'-' '{print $1}')

  if [ "$version_a" \= "$version_b" ]
  then
    # check for pre-release
    # extract pre-release (-foo.n from x.y.z-foo.n)
    pr_a=$(echo "$1" | awk -F'-' '{print $2}')
    pr_b=$(echo "$2" | awk -F'-' '{print $2}')

    ####
    # Return 0 when A is equal to B
    [ "$pr_a" \= "$pr_b" ] && echo 0 && return 0

    ####
    # Return 1

    # Case when A is not pre-release
    if [ -z "$pr_a" ]
    then
      echo 1 && return 0
    fi

    ####
    # Case when pre-release A exists and is greater than B's pre-release

    # extract numbers -rc.x --> x
    number_a=$(echo ${pr_a//[!0-9]/})
    number_b=$(echo ${pr_b//[!0-9]/})
    [ -z "${number_a}" ] && number_a=0
    [ -z "${number_b}" ] && number_b=0

    [ "$pr_a" \> "$pr_b" ] && [ -n "$pr_b" ] && [ "$number_a" -gt "$number_b" ] && echo 1 && return 0

    ####
    # Retrun -1 when A is lower than B
    echo -1 && return 0
  fi
  arr_version_a=(${version_a//./ })
  arr_version_b=(${version_b//./ })
  cursor=0
  # Iterate arrays from left to right and find the first difference
  while [ "$([ "${arr_version_a[$cursor]}" -eq "${arr_version_b[$cursor]}" ] && [ $cursor -lt ${#arr_version_a[@]} ] && echo true)" == true ]
  do
    cursor=$((cursor+1))
  done
  [ "${arr_version_a[$cursor]}" -gt "${arr_version_b[$cursor]}" ] && echo 1 || echo -1
}


export CURRENT_GIT_VERSION=$(git tag -l | tail -1 | sed 's/v//' | cut -f1 -d '-')
export CARGO_TOML_VERSION=$(cat ./weaviate-community/Cargo.toml | grep 'version = ' | head -1 | awk '{ print $3 }' | sed 's/"//g')
export README_VERSION=$(cat README.md | grep "weaviate-community = " | cut -f2 -d '"')

echo "CURRENT_GIT_VERSION: $CURRENT_GIT_VERSION"
echo "CARGO_TOML_VERSION: $CARGO_TOML_VERSION"
echo "README_VERSION: $README_VERSION"

# Check that the README is updated
if [ $CARGO_TOML_VERSION != $README_VERSION ]
then
    echo "README version should match the Cargo.toml version"
    exit 1
fi

# Check for a new version
if [ $(semver_compare $CARGO_TOML_VERSION $CURRENT_GIT_VERSION) == "1" ]
then
    echo "New version detected. Deploying new version to crates.io"
    echo "TODO"
elif [ $(semver_compare $CARGO_TOML_VERSION $CURRENT_GIT_VERSION) == "-1" ]
then
    echo "New version is lower than current. New version should be higher."
    exit 1
else
    echo "No new version located"
fi
