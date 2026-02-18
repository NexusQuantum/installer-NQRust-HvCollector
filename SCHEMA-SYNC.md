# Database Schema Management

## Current Approach: Schema Baked into Postgres Image

The database schema is defined in `postgres/init.sql` inside the
[hypervisor-collector](https://github.com/NexusQuantum/hypervisor-collector) repo
and is **baked into the `postgres-hypervisor-collector` Docker image** at build time.

> **No manual `init-db.sql` sync is needed in this installer.**
> The schema is automatically applied when the postgres container starts for the first time.

### Schema Source

- **Reference**: `hypervisor-collector/models.py` (SQLAlchemy models)
- **Implementation**: `hypervisor-collector/postgres/init.sql` (baked into image)
- **Image**: `ghcr.io/nexusquantum/postgres-hypervisor-collector:latest`
- **Last updated**: 2026-02-18

### Tables (all in `fluentd` schema)

1. `fluentd.logs` — Container/systemd logs
2. `fluentd.hypervisor_inventory` — VMs, Volumes, Images, Networks
3. `fluentd.hypervisor_node_usage` — Node CPU/Memory
4. `fluentd.hypervisor_pod_usage` — Pod CPU/Memory
5. `fluentd.hypervisor_collector_status` — Health check
6. `fluentd.hypervisor_dashboard_summary` — Dashboard counts
7. `fluentd.hypervisor_capacity` — Capacity metrics
8. `fluentd.hypervisor_events` — Kubernetes events
9. `fluentd.hypervisor_cluster_metrics` — Prometheus cluster metrics
10. `fluentd.hypervisor_vm_metrics` — Prometheus VM metrics

### Update Procedure

When `models.py` changes in the hypervisor-collector repo:

1. **Update** `hypervisor-collector/postgres/init.sql` with new/changed tables
2. **Push** to `main` branch → CI/CD automatically rebuilds and pushes `postgres-hypervisor-collector:latest`
3. **Pull** the new image on the target server:
   ```bash
   docker compose pull postgres
   docker compose down -v   # ⚠️ drops existing data
   docker compose up -d
   ```

### Manual Migration for Existing Installations

If schema changes after initial installation (to avoid data loss):

```bash
# Connect to database
docker exec -it hypervisor-postgres psql -U postgres -d hypervisor

# Run ALTER TABLE statements manually
# Example:
ALTER TABLE fluentd.hypervisor_inventory ADD COLUMN new_field VARCHAR(255);
```

### Verify Schema

```bash
docker exec hypervisor-postgres psql -U postgres -d hypervisor -c "\dt fluentd.*"
```
