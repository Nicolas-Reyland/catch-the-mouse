# catch-the-mouse
Use one mouse on multiple computers.

# Installation
In France, we call taht "flemme" (install cargo for rust)

### Windows :
```batch
git clone https://github.com/Nicolas-Reyland/catch-the-mouse
cd catch-the-mouse
install.bat
```

### Linux :
```bash
git clone https://github.com/Nicolas-Reyland/catch-the-mouse
cd catch-the-mouse
./install.sh
```

Additionally, install `libxdo-dev` and `libx11-dev` :
```bash
sudo apt-get install libxdo-dev
sudo apt install libx11-dev # Fedora/RHEL/CentOS: xorg-x11-server-devel
```

# Usage
As the 'cat' (server/computer connected to the mouse) do :
```bash
cd cat
cargo run YOUR_IP_ADDR:PORT
```

As the 'mouse' (client/computer without a mouse) do :
```bash
cd mouse
argo run CAT_IP_ADDR:PORT
```

# Rust
- You could ask me: 'Why are you doing this in Rust when a python script could suffice ?'
- I don't know either.
