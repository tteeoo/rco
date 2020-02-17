# Uninstallation script for rco (https://github.com/tteeoo/rco)
# Removes program and (optionally) its configuration files
prompt () {
    read -p "uninstall.sh: Also remove configuration directory and files? (y/n) " x
    if [ $x = "y" ]; then
	echo "uninstall.sh: Removing configuration files"
	sudo -u $SUDO_USER rm -r $(getent passwd $SUDO_USER | cut -d: -f6)/.config/rco/
	rmexit=$?
	if [ $rmexit -ne 0 ]; then
	    echo "uninstall.sh: Error removing configuration, does the directory ~/.config/rco/ even exist?"
	    exit 1
	fi
	echo "uninstall.sh: Removing binary executable"
	rm /usr/bin/rco
	biexit=$?
	if [ $biexit -ne 0 ]; then
	    echo "uninstall.sh: Error removing executable, does the file /usr/bin/rco even exist?"
	    exit 1
	fi
    elif [ $x = "n" ]; then
	echo "uninstall.sh: Removing binary executable"
	rm /usr/bin/rco
	biexit=$?
	if [ $biexit -ne 0 ]; then
	    echo "uninstall.sh: Error removing executable, does the file /usr/bin/rco even exist?"
	    exit 1
	fi
    else
	echo "uninstall.sh: Please enter either \"y\" or \"n\""
	prompt
    fi
    echo "uninstall.sh: Uninstallation successful!"
    exit 0
}
prompt
