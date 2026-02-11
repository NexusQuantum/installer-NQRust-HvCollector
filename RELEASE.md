# Release Guide

HV Collector Installer releases are automated via GitHub Actions when a release is published.

> Binary/package name: `nqrust-hvcollector`

## Prerequisites
- Write access to repository
- Updated version in `Cargo.toml`
- Release notes summary

## Release Steps

1. **Bump version**:
   ```bash
   cargo set-version <new-version>
   # OR edit Cargo.toml manually
   git commit -am "chore: bump version to <new-version>"
   ```

2. **Sanity checks** (optional):
   ```bash
   cargo fmt
   cargo check
   cargo test
   cargo deb
   ```

3. **Push to main**:
   ```bash
   git push origin main
   ```

4. **Create GitHub Release**:
   - Go to Releases → Draft new release
   - Tag: `v<new-version>` (create tag in UI)
   - Add release notes
   - Click **Publish release**

5. **Workflow runs** automatically (`.github/workflows/release.yml`)

## Build Artifacts

The workflow produces:
- `nqrust-hvcollector-linux-amd64.tar.gz` - Tarball with binary
- `nqrust-hvcollector-linux-amd64` - Raw ELF binary
- `nqrust-hvcollector_*.deb` - Versioned Debian package
- `nqrust-hvcollector_amd64.deb` - Stable alias (latest)
- `SHA256SUMS` - Checksums for all artifacts

## Verification

```bash
# Check checksums
sha256sum -c SHA256SUMS

# Inspect package
dpkg -c nqrust-hvcollector_*.deb

# Install package
sudo dpkg -i nqrust-hvcollector_*.deb

# Verify installation
nqrust-hvcollector --version
```

## Troubleshooting

- Workflow fails: Check Actions tab for error logs
- Version mismatch: Ensure `Cargo.toml` version matches release tag
- Build errors: Verify all dependencies in `Cargo.toml`
