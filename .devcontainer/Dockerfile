FROM archlinux/base

RUN pacman -Syu --noconfirm \
   && pacman -S --noconfirm git base-devel rust-std rust cargo neofetch \
   && pacman -Scc --noconfirm
