# Testing Notes - HV Collector Installer

## Test Scenarios

### 1. Standard Installation (Online)
**Prerequisites:**
- Docker + Docker Compose v2
- Valid kubeconfig.yaml
- GitHub PAT with `read:packages` scope

**Steps:**
1. Place `kubeconfig.yaml` in project directory
2. Run `./target/release/nqrust-hvcollector`
3. Authenticate with GitHub PAT when prompted (can skip if airgapped)
4. Fill in form:
   - PostgreSQL: host (`postgres`), user (`postgres`), password, port, db (`hypervisor`), schema (`fluentd`)
   - Hypervisor/SSH: host, user, password
   - Collector: interval (default: 60s), Prometheus URL (optional), retention days
5. Proceed with installation

**Expected:**
- `.env` file created with correct values
- All 4 services start:
  - `hypervisor-postgres` (PostgreSQL 15)
  - `hypervisor-collector`
  - `hypervisor-prometheus-pf`
  - `hypervisor-fluentd` (ports 24224, 9880)

**Verification:**
```bash
docker compose -p hvcollector ps
docker compose -p hvcollector logs hypervisor-collector
docker exec -it hypervisor-postgres psql -U postgres -d hypervisor
```

### 2. Airgapped Installation
**Build (online machine):**
```bash
./scripts/airgapped/build-single-binary.sh
```

**Expected output:**
- `nqrust-hvcollector-airgapped` binary (~400 MB)
- Contains all 4 Docker images embedded:
  - `postgres:15-alpine`
  - `ghcr.io/nexusquantum/hypervisor-collector:latest`
  - `bitnami/kubectl:latest`
  - `ghcr.io/nexusquantum/fluentd-hypervisor-collector:latest`

**Install (target machine):**
1. Transfer binary and `kubeconfig.yaml` to target machine
2. `chmod +x nqrust-hvcollector-airgapped`
3. `./nqrust-hvcollector-airgapped`
4. Images auto-load from embedded payload, then TUI installer starts
5. Complete TUI flow as in scenario 1

**Verification:**
```bash
docker images | grep -E "postgres|kubectl|hypervisor-collector|fluentd"
docker compose -p hvcollector ps
```

### 3. Configuration Validation
**Test invalid inputs:**
- Empty PostgreSQL host → Error: "PostgreSQL Host is required!"
- Invalid port (abc) → Error: "PostgreSQL Port must be a valid number!"
- Empty hypervisor host → Error: "Hypervisor Host is required!"
- Empty hypervisor password → Error: "Hypervisor Password is required!"

**Test defaults:**
- PostgreSQL host: `postgres`
- PostgreSQL user: `postgres`
- PostgreSQL database: `hypervisor`
- PostgreSQL schema: `fluentd`
- Interval: `60`
- Log retention days: `365`
- Data retention days: `30`

### 4. Service Health Checks
```bash
# PostgreSQL
docker exec hypervisor-postgres pg_isready -U postgres

# Fluentd HTTP input
curl -f http://localhost:9880/api/plugins.json

# Prometheus port-forward (inside collector network namespace)
# Check via collector logs — should show "Connected to Prometheus"
docker compose -p hvcollector logs hypervisor-collector | grep -i prometheus
```

### 5. Data Collection Verification
**Steps:**
1. Install and start all services
2. Wait 60 seconds (default interval)
3. Query PostgreSQL for collected data:
```sql
SELECT cluster, COUNT(*) FROM fluentd.hypervisor_inventory
  GROUP BY cluster ORDER BY cluster;

SELECT cluster, COUNT(*) FROM fluentd.hypervisor_node_usage
  GROUP BY cluster ORDER BY cluster;

SELECT COUNT(*) FROM fluentd.logs;
```

**Expected:**
- Non-empty result sets after first collection cycle
- Recent timestamps
- Valid metric values per cluster

### 6. Kubeconfig Validation
**Test missing kubeconfig:**
- Remove `kubeconfig.yaml`
- Run installer
- Confirmation screen shows: `✗ kubeconfig.yaml (missing)`
- Cannot proceed until file is present

**Test invalid kubeconfig:**
- Create invalid YAML file
- Collector service will fail to start
- Check logs: `docker compose -p hvcollector logs hypervisor-collector`

## Build Quality

```bash
cargo check   # No warnings
cargo clippy  # No warnings
cargo test    # All tests pass
cargo build --release
```

## Known Issues

None currently documented.

## Testing Checklist

- [ ] Standard online installation works
- [ ] Airgapped binary builds successfully
- [ ] Airgapped installation works offline (images load from binary)
- [ ] Form validation catches required field errors
- [ ] Form defaults are sensible
- [ ] .env file generated correctly
- [ ] docker-compose.yaml scaffolded when missing
- [ ] PostgreSQL connection succeeds
- [ ] Collector retrieves metrics from Kubernetes/Harvester
- [ ] Fluentd receives and stores logs in PostgreSQL
- [ ] All services restart after host reboot (restart: unless-stopped)

## Test Environments

**Recommended:**
- Ubuntu 22.04 LTS
- Docker 24.0+
- Kubernetes 1.24+ / Harvester 1.x

**Supported:**
- Any Linux with Docker + Docker Compose v2
- WSL2 (for development/testing only)
