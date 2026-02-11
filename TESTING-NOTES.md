# Testing Notes - HV Collector Installer

## Test Scenarios

### 1. Standard Installation (Online)
**Prerequisites:**
- Docker + Docker Compose v2
- Valid kubeconfig.yaml
- GitHub PAT with read:packages

**Steps:**
1. Place `kubeconfig.yaml` in project directory
2. Run `./target/release/nqrust-hvcollector install`
3. Authenticate with GitHub PAT
4. Fill in form:
   - PostgreSQL: host, user, password, port (5431), db, schema
   - Hypervisor: host, user, password
   - Collector: cluster name, interval
5. Proceed with installation

**Expected:**
- `.env` file created with correct values
- Docker Compose builds Fluentd image
- All 4 services start:
  - postgres (port 5431)
  - hypervisor-collector
  - prometheus-pf
  - fluentd (ports 24224, 9880)

**Verification:**
```bash
docker compose -p hvcollector ps
docker compose -p hvcollector logs hypervisor-collector
psql -h localhost -p 5431 -U postgres -d hypervisor
```

### 2. Airgapped Installation
**Build (online machine):**
```bash
./scripts/airgapped/build-single-binary.sh
```

**Expected output:**
- `nqrust-hvcollector-airgapped` binary (~2 GB)
- Contains all 4 Docker images embedded

**Install (airgapped machine):**
1. Transfer binary to isolated machine
2. Run `./nqrust-hvcollector-airgapped install`
3. Verify images load from embedded payload
4. Complete TUI flow as in scenario 1

**Verification:**
```bash
docker images | grep -E "postgres|kubectl|hypervisor-collector|fluentd"
```

### 3. Configuration Validation
**Test invalid inputs:**
- Empty PostgreSQL host → Error: "PostgreSQL Host is required!"
- Invalid port (abc) → Error: "PostgreSQL Port must be a valid number!"
- Empty hypervisor password → Error: "Hypervisor Password is required!"

**Test defaults:**
- PostgreSQL user: defaults to "postgres"
- PostgreSQL port: defaults to "5431"
- Cluster name: defaults to "harvester"
- Interval: defaults to "60"

### 4. Service Health Checks
```bash
# PostgreSQL
docker exec postgres pg_isready -U postgres

# Hypervisor Collector
curl -f http://localhost:8080/health || echo "No health endpoint"

# Fluentd
curl -f http://localhost:9880/api/plugins.json

# Prometheus port-forward
# Manual check: kubectl proxy running inside container
```

### 5. Data Collection Verification
**Steps:**
1. Install and start all services
2. Wait 60 seconds (default interval)
3. Query PostgreSQL for collected data:
```sql
SELECT * FROM fluentd.vm_metrics ORDER BY timestamp DESC LIMIT 10;
SELECT * FROM fluentd.node_metrics ORDER BY timestamp DESC LIMIT 10;
```

**Expected:**
- Non-empty result sets
- Recent timestamps
- Valid metric values

### 6. Kubeconfig Validation
**Test missing kubeconfig:**
- Remove `kubeconfig.yaml`
- Run installer
- Confirmation screen shows: "❌ kubeconfig.yaml"
- Cannot proceed until file is present

**Test invalid kubeconfig:**
- Create invalid YAML file
- Collector service will fail to start
- Check logs: `invalid kubeconfig` error

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
- [ ] Airgapped installation works offline
- [ ] Form validation catches errors
- [ ] PostgreSQL connection succeeds
- [ ] Collector retrieves metrics from Kubernetes
- [ ] Fluentd receives and stores logs in PostgreSQL
- [ ] All services restart after host reboot
- [ ] Update command works (if implemented)

## Test Environments

**Recommended:**
- Ubuntu 22.04 LTS
- Docker 24.0+
- Kubernetes 1.24+ (for Harvester clusters)

**Supported:**
- Any Linux with Docker + Docker Compose v2
- WSL2 (for development/testing)
