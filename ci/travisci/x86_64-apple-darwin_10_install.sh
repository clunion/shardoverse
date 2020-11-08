echo in Script: $0

set -xe

echo TARGET=$TARGET
echo TRAVIS_TARGET=$TRAVIS_TARGET
echo TRAVIS_OS_NAME=$TRAVIS_OS_NAME

sort=gsort  # for `sort --sort-version`, from brew's coreutils.

# sudo apt-get update
# sudo apt-get install --assume-yes libgl1-mesa-dev
