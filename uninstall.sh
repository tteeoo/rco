# Uninstallation script for rco (https://github.com/tteeoo/rco)
# Removes program and (optionally) its configuration files
prompt () {
    read -p "rco-uninstall.sh> Also remove configuration directory and files? (y/n) " x
    if [ $x = "y" ]; then
	echo "rco-uninstall.sh> Removing configuration files"
	rm -r ~/.config/rco/
	rmexit=$?
	if [ $rmexit -ne 0 ]; then
	    echo "rco-uninstall.sh> Error removing configuration, does the directory ~/.config/rco/ even exist?"
	    exit 1
	fi
	echo "rco-uninstall.sh> Removing binary executable"
	sudo rm /usr/bin/rco
	biexit=$?
	if [ $biexit -ne 0 ]; then
	    echo "rco-uninstall.sh> Error removing executable, does the file /usr/bin/rco even exist?"
	    echo "Do you have sudo installed and appropriate permissions?"
	    exit 1
	fi
    elif [ $x = "n" ]; then
	echo "rco-uninstall.sh> Removing binary executable"
	sudo rm /usr/bin/rco
	biexit=$?
	if [ $biexit -ne 0 ]; then
	    echo "rco-uninstall.sh> Error removing executable, does the file /usr/bin/rco even exist?"
	    echo "Do you have sudo installed and appropriate permissions?"
	    exit 1
	fi
    else
	echo "rco-uninstall.sh> Please enter either \"y\" or \"n\""
	prompt
    fi
    echo "rco-uninstall.sh> Uninstallation successful!"
    exit 0
}
prompt
