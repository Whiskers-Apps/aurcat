# WIP
The program it's still in development phase.

# Features
## AUR
- [ ] Optional Package Review

## Global
- [ ] Updates
- [x] Install
- [x] Uninstall
- [x] Search
- [x] List

## Utils
- [x] Update Signing Keys
- [x] Remove Pacman Lock
- [x] Clear Pacman Cache
- [x] Fallback To Pacman Commands

# Dependencies
The program requires depedencies for some of the features:
```
sudo pacman -S pacman-contrib archlinux-keyring
```

- `pacman-contrib` To clear pacman cache.
- `archlinux-keyring` To update signing keys. Important from time to time.
