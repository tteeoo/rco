# Installation script for rco (https://github.com/tteeoo/rco)
# Builds program and copies it to /usr/bin/ to easily be ran
echo "install.sh: Attempting to compile rco"
sudo -u $SUDO_USER cargo build --release
cargexit=$?
if [ $cargexit -ne 0 ]; then
    echo "install.sh: Error compiling, do you have cargo installed? If so, make sure you are in the root directory of the rco repository"
    echo "If none of that helps, open an appropriate issue on GitHub"
    exit 1
fi
if [ ! -f ./target/release/rco ]; then
    echo "install.sh: Compiled binary not found, make sure you are in the root directory of the rco repository"
    exit 1
else
    echo "install.sh: Copying binary executable to /usr/bin/ for easy execution"
    cp ./target/release/rco /usr/bin/
    cpexit=$?
    if [ $cpexit -ne 0 ]; then
	echo "install.sh: Error copying"
	exit 1
    fi
fi
echo "install.sh: Installation successful!"
exit 0
