#set -ex
echo in Script: $0
echo in Script: x86_64-unknown-linux-gnu_10_install.sh

echo "--------------------------------------------------------"
uname -a

echo "--------------------------------------------------------"
sudo apt-cache search sdl

echo "--------------------------------------------------------"
sudo apt-get update

echo "--------------------------------------------------------"
sudo apt list --installed


