# Configuration script for rco (https://github.com/tteeoo/rco)
# Creates directory and config file for rco
if [ ! -d ~/.config/rco ]; then
    echo "rco-configure.sh> Creating config directory (~/.config/rco/)"
    mkdir -p ~/.config/rco
    mkexit=$?
    if [ $mkexit -ne 0 ]; then
	echo "rco-configure.sh> Error creating config directory"
	exit 1
    fi
else
    echo "rco-configure.sh> Config directory found, not making it"
fi
if [ ! -f ~/.config/rco/config.csv ]; then
    if [ ! -f ./defaults/config.csv ]; then
	echo "rco-configure.sh> Default config file not found, make sure you are in the root directory of the rco repository"
	exit 1
    fi
    if [ ! -f ~/.config/rco/config.csv ]; then
	echo "rco-configure.sh> Creating config.csv file (~/.config/rco/config.csv)"
	cp ./defaults/config.csv ~/.config/rco/
    fi
    cpexit=$?
    if [ $cpexit -ne 0 ]; then
	echo "rco-configure.sh> Error copying default config.csv"
	exit 1
    fi
else
    echo "rco-configure.sh> Config file found, not making it"
fi
echo "rco-configure.sh> Configuration successful!"
exit 0
