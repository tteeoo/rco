cargo build --release
mkdir ~/.config/rco
cp ./defaults/rco/* ~/.config/rco/
cp ./target/release/rco /usr/bin/
