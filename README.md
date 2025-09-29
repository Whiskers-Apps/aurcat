# WIP
The program it's still in development phase.

# Features
## AUR
- [ ] Verify AUR Package Conflicts
- [ ] Verify AUR Package Dependencies
- [ ] Optional Package Review
- [ ] Uninstall Package

## Global
- [ ] Updates
- [ ] Install
- [ ] Uninstall
- [x] List Packages

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
