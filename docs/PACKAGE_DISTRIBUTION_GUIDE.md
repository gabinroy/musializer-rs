# Maintainer Package Distribution Guide: Musializer-RS

This guide explains how to set up the credentials and accounts for automated package distribution across **Arch/CachyOS (AUR)**, **Windows (Winget)**, **Flathub**, **Debian/Ubuntu (APT)**, and **Fedora (DNF/COPR)**.

---

## 1. Arch Linux / CachyOS AUR (`yay -S musializer-rs-bin`)

### Step 1: Create an AUR Account
1. Visit [aur.archlinux.org/register](https://aur.archlinux.org/register) and create a free account.
2. Generate an SSH keypair on your machine:
   ```bash
   ssh-keygen -t ed25519 -C "aur" -f ~/.ssh/aur_key
   ```
3. Copy your public key:
   ```bash
   cat ~/.ssh/aur_key.pub
   ```
4. Paste it into your AUR Account settings under **SSH Public Key**.

### Step 2: Add GitHub Secret
1. In your GitHub repository (`gabinroy/musializer-rs`), go to **Settings** > **Secrets and variables** > **Actions** > **New repository secret**.
2. Name: `AUR_SSH_PRIVATE_KEY`
3. Value: Paste the entire content of your private key (`~/.ssh/aur_key`).

> 🎉 Every tag push (`v0.1.0`, etc.) will now automatically update the AUR package!

---

## 2. Windows Package Manager (`winget install GabinRoy.MusializerRS`)

### Step 1: Create a GitHub Personal Access Token (PAT)
1. Go to [github.com/settings/tokens](https://github.com/settings/tokens) > **Generate new token (classic)**.
2. Note: `Winget Releaser`
3. Expiration: 90 days or No expiration.
4. Select Scopes:
   - `public_repo` (required to fork `microsoft/winget-pkgs` and submit pull requests).
5. Click **Generate token** and copy it.

### Step 2: Add GitHub Secret
1. In `gabinroy/musializer-rs` > **Settings** > **Secrets and variables** > **Actions** > **New repository secret**.
2. Name: `WINGET_TOKEN`
3. Value: Paste your GitHub PAT token.

---

## 3. Flathub (Flatpak Distribution)

### Submission Process (One-Time Setup):
1. Fork the [flathub/flathub](https://github.com/flathub/flathub) repository on GitHub.
2. Create a new branch `new-pr/org.musializer.MusializerRS`.
3. Add the files located in `distribution/flatpak/org.musializer.MusializerRS.yml` and `distribution/org.musializer.MusializerRS.metainfo.xml`.
4. Submit a Pull Request to `flathub/flathub`.
5. Once approved by the Flathub team, Flathub will create a dedicated repository `flathub/org.musializer.MusializerRS` where automated builds trigger on every new Git tag!

---

## 4. Debian / Ubuntu (`APT` Repository Hosting)

To enable users to run `sudo apt install musializer-rs`, you can use either **Cloudsmith** (recommended modern open-source repo host) or **Launchpad PPA**:

### Option A: Cloudsmith (Free for Open Source)
1. Register at [cloudsmith.com](https://cloudsmith.com).
2. Create a public repository named `musializer-rs`.
3. Users can then install Musializer-RS using:
   ```bash
   curl -1sLf 'https://dl.cloudsmith.io/public/gabinroy/musializer-rs/setup.deb.sh' | sudo -E bash
   sudo apt update
   sudo apt install musializer-rs
   ```

### Option B: Standalone `.deb` Releases
Direct `.deb` files are built automatically on every release. Users can simply run:
```bash
sudo dpkg -i musializer-rs-*.deb || sudo apt-get install -f -y
```

---

## 5. Fedora / RHEL (`DNF` Repository via Fedora COPR)

1. Register at [copr.fedorainfracloud.org](https://copr.fedorainfracloud.org/) using a free Fedora Account.
2. Click **New Project** > Name: `musializer-rs`.
3. Link your GitHub repository `gabinroy/musializer-rs` as the source webhook.
4. Users can now install it on Fedora with:
   ```bash
   sudo dnf copr enable gabinroy/musializer-rs
   sudo dnf install musializer-rs
   ```
