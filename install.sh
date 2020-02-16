# Installation script for rco (https://github.com/tteeoo/rco)
# Builds program and copies it to /usr/bin/ to easily be ran
echo "rco-install.sh> Attempting to compile rco"
cargo build --release
cargexit=$?
if [ $cargexit -ne 0 ]; then
    echo "rco-install.sh> Error compiling, do you have cargo installed? If so, make sure you are working in the rco repository"
    echo "If none of that helps, open an appropriate issue on GitHub"
    exit 1
fi
if [ ! -f ./target/release/rco ]; then
    echo "rco-install.sh> Compiled binary not found, make sure you are in the root directory of the rco repository"
    exit 1
else
    echo "rco-install.sh> Copying binary executable to /usr/bin/ for easy execution"
    sudo cp ./target/release/rco /usr/bin/
    cpexit=$?
    if [ $cpexit -ne 0 ]; then
	echo "rco-install.sh> Error copying, do you have sudo installed and appropriate permissions?"
	echo "If you do not have root on this machine, you can still use rco by just running the binary (target/release/rco)"
	exit 1
    fi
fi
echo "rco-install.sh> Installation successful!"
exit 0
