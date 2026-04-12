# Airgapped Installation Guide

Complete guide for building and using the NQRust HV Collector airgapped installer.

## Overview

The airgapped installer is a **self-extracting single binary** (~400 MB) that contains:
- NQRust HV Collector installer binary
- All Docker images (hypervisor-collector, fluentd-hypervisor-collector, postgres, kubectl)
- No internet connection required for installation

---

## Building the Airgapped Binary

### Prerequisites

**On the build machine (with internet):**
- Docker and Docker Compose installed
- Rust toolchain installed
- GitHub Container Registry access (for pulling images)
- ~5 GB free disk space

### Build Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/NexusQuantum/installer-NQRust-HvCollector.git
   cd installer-NQRust-HvCollector
   ```

2. **Login to GitHub Container Registry:**
   ```bash
   docker login ghcr.io
   # Username: your-github-username
   # Password: your-personal-access-token
   ```

3. **Run the build script:**
   ```bash
   chmod +x scripts/airgapped/build-single-binary.sh
   ./scripts/airgapped/build-single-binary.sh
   ```

   This will:
   - Build the Rust binary (`cargo build --release`)
   - Pull all 4 Docker images from registries
   - Save images to compressed tar.gz files
   - Bundle everything into a single payload
   - Create self-extracting binary: `nqrust-hvcollector-airgapped`

4. **Verify the build:**
   ```bash
   # Check file size (should be ~400 MB)
   ls -lh nqrust-hvcollector-airgapped

   # Verify checksum
   sha256sum -c nqrust-hvcollector-airgapped.sha256
   ```

### Build Output

```
nqrust-hvcollector-airgapped         (~400 MB) - Self-extracting binary
nqrust-hvcollector-airgapped.sha256  - Checksum file
build/
├── images/                          - Individual image tar.gz files
│   ├── hypervisor-collector.tar.gz
│   ├── fluentd-hypervisor-collector.tar.gz
│   ├── postgres.tar.gz
│   ├── kubectl.tar.gz
│   ├── manifest.json
│   └── SHA256SUMS
└── payload.tar.gz                   - Combined payload
```

---

## Transferring to Airgapped Environment

### Transfer Methods

**Option 1: USB Drive**
```bash
# Copy to USB
cp nqrust-hvcollector-airgapped /media/usb/
cp nqrust-hvcollector-airgapped.sha256 /media/usb/

# On airgapped machine
cp /media/usb/nqrust-hvcollector-airgapped ~/
cp /media/usb/nqrust-hvcollector-airgapped.sha256 ~/
```

**Option 2: SCP (if network is available)**
```bash
scp nqrust-hvcollector-airgapped user@target-host:~/
scp nqrust-hvcollector-airgapped.sha256 user@target-host:~/
```

**Option 3: Physical Media**
- Burn to DVD/Blu-ray
- Use external HDD/SSD

---

## Docker Airgapped Installer (Optional)

If the target machine **does not have Docker** yet, use this installer to install the full Docker stack offline:

- **Docker CE** (engine + CLI)
- **Docker Compose v2** (`docker compose` plugin)
- **Docker Buildx** (BuildKit)
- **containerd**

### Building the Docker Airgapped Bundle (machine with internet)

**Prerequisites:** Docker and `curl`/`gpg` on the build machine.

1. **Download packages for your target distro:**
   ```bash
   chmod +x scripts/airgapped/download-docker-packages.sh
   ./scripts/airgapped/download-docker-packages.sh ubuntu24.04
   ```
   Supported: `ubuntu24.04`, `ubuntu22.04`, `debian12`.

2. **Or build a full bundle (multiple distros + tarball):**
   ```bash
   chmod +x scripts/airgapped/build-docker-airgapped-bundle.sh
   ./scripts/airgapped/build-docker-airgapped-bundle.sh ubuntu24.04
   ```

3. **Output:**
   - `build/docker-packages/<distro>/` — folder with `.deb` files and `install-docker-offline.sh`
   - Or `build/docker-airgapped-YYYYMMDD.tar.gz` — tarball to transfer

### Installing Docker on the Target Machine

1. Transfer the folder (or tarball) to the machine (USB/SCP).
2. If you transferred a tarball:
   ```bash
   tar xzf docker-airgapped-*.tar.gz
   cd docker-packages/ubuntu24.04   # or your distro
   ```
3. Run the installer:
   ```bash
   chmod +x install-docker-offline.sh
   sudo ./install-docker-offline.sh
   ```
4. **Add your user to the `docker` group:**
   ```bash
   sudo usermod -aG docker $USER
   ```
   Then log out and back in.

---

## Installation on Target Machine

### Prerequisites

**On the target machine:**
- **Docker** (engine + CLI)
- **Docker Compose v2** (`docker compose` — Compose v2 plugin)
- **Docker Buildx** (BuildKit)
- **Access to Docker daemon:** user in `docker` group or use `sudo`
- ~5 GB free disk space
- Linux OS (tested on Ubuntu 20.04+, Debian 11+)
- `kubeconfig.yaml` for your Harvester/Kubernetes cluster

If Docker is not installed, use the [Docker Airgapped Installer](#docker-airgapped-installer-optional) above.

### Installation Steps

1. **Verify the binary:**
   ```bash
   sha256sum -c nqrust-hvcollector-airgapped.sha256
   ```
   Should output: `nqrust-hvcollector-airgapped: OK`

2. **Make executable:**
   ```bash
   chmod +x nqrust-hvcollector-airgapped
   ```

3. **Place `kubeconfig.yaml`** in the same directory as the binary.

4. **Run the installer:**
   ```bash
   ./nqrust-hvcollector-airgapped
   ```

### What Happens During Installation

```
1. Airgapped mode detected
   └─ Load embedded Docker images

2. Extract embedded Docker images
   ├─ Locate payload marker in binary
   ├─ Stream extract to /tmp/nqrust-*
   └─ Extract payload

3. Load images to Docker
   ├─ Load postgres:15-alpine
   ├─ Load ghcr.io/nexusquantum/hypervisor-collector:latest
   ├─ Load bitnami/kubectl:latest
   └─ Load ghcr.io/nexusquantum/fluentd-hypervisor-collector:latest

4. Cleanup temporary files
   └─ Delete /tmp/nqrust-* directory

5. Run installer TUI
   ├─ Generate .env file (if missing)
   └─ Deploy with docker compose up
```

### Expected Output

```
🔒 Airgapped mode detected
📦 Extracting embedded Docker images...
  Locating payload...
  Payload size: 0.38 GB
  Verifying payload integrity...
  ✓ Payload checksum: ...
  Extracting...
🐳 Loading images to Docker...
  Loading 4 Docker images...
  [1/4] postgres:15-alpine
  [2/4] ghcr.io/nexusquantum/hypervisor-collector:latest
  [3/4] bitnami/kubectl:latest
  [4/4] ghcr.io/nexusquantum/fluentd-hypervisor-collector:latest
  ✓ All images loaded successfully
🧹 Cleaning up temporary files...
✓ Airgapped setup complete!

[Installer TUI starts...]
```

---

## Post-Installation Verification

### Verify Docker Images Loaded

```bash
docker images | grep -E 'nexusquantum|postgres|kubectl'
```

Expected output:
```
ghcr.io/nexusquantum/hypervisor-collector           latest    ...
ghcr.io/nexusquantum/fluentd-hypervisor-collector   latest    ...
postgres                                             15-alpine ...
bitnami/kubectl                                      latest    ...
```

### Verify Services Running

```bash
docker compose -p hvcollector ps
```

All services should be in `Up` state:
- `hypervisor-postgres` — PostgreSQL 15
- `hypervisor-collector` — Metrics collector
- `hypervisor-prometheus-pf` — Prometheus port-forward
- `hypervisor-fluentd` — Log aggregation (ports 24224, 9880)

### View Logs

```bash
# Collector logs
docker compose -p hvcollector logs -f hypervisor-collector

# Fluentd logs
docker compose -p hvcollector logs -f fluentd

# Connect to PostgreSQL
docker exec -it hypervisor-postgres psql -U postgres -d hypervisor
```

---

## Troubleshooting

### Issue: "Docker is not installed or not in PATH"

**Solution:**
```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Start Docker daemon
sudo systemctl start docker
sudo systemctl enable docker
```

### Issue: "Docker daemon is not running"

**Solution:**
```bash
sudo systemctl start docker
```

### Issue: "Payload marker not found in binary"

**Cause:** Binary is corrupted or not the airgapped version

**Solution:**
- Verify checksum: `sha256sum -c nqrust-hvcollector-airgapped.sha256`
- Re-transfer from build machine
- Ensure you're using the airgapped binary, not the regular one

### Issue: "No space left on device"

**Cause:** Insufficient disk space for extraction

**Solution:**
```bash
# Check available space
df -h /tmp

# Need at least 2 GB free in /tmp for extraction
```

### Issue: Images already loaded, but want to re-extract

**Solution:**
```bash
# Remove existing images
docker rmi ghcr.io/nexusquantum/hypervisor-collector:latest
docker rmi ghcr.io/nexusquantum/fluentd-hypervisor-collector:latest
docker rmi postgres:15-alpine
docker rmi bitnami/kubectl:latest

# Run installer again
./nqrust-hvcollector-airgapped
```

---

## FAQ

**Q: How large is the airgapped binary?**
A: ~400 MB (contains 4 Docker images)

**Q: Can I use this on ARM64?**
A: Currently only `linux/amd64` is supported.

**Q: How do I update to a newer version?**
A: Download the new airgapped binary from the GitHub releases page and run it. The TUI installer will detect existing configuration and upgrade in place.

**Q: Does this work on Windows?**
A: No, currently Linux only.

**Q: What if I don't have a Prometheus endpoint?**
A: Leave `PROMETHEUS_URL` empty in the `.env` configuration. Prometheus metrics collection is optional.

---

## Security Considerations

1. **Verify checksums** before running the binary
2. **Use secure transfer** methods (encrypted USB, SCP with keys)
3. **Protect your `.env` file** — it contains database and hypervisor credentials
4. **Keep build machine secure** — it has access to your GitHub credentials

---

## Support

For issues or questions:
- GitHub Issues: [NexusQuantum/installer-NQRust-HvCollector](https://github.com/NexusQuantum/installer-NQRust-HvCollector/issues)
