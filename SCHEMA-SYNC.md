# Database Schema Management

## Current Approach: Manual Sync

The database schema is defined in `init-db.sql` and must be manually synchronized with the HV Collector codebase.

### Schema Source
- **Reference**: `reference/hypervisor-collector/models.py`
- **Implementation**: `init-db.sql`
- **Last Sync**: 2026-02-11

### Tables Included
1. `fluentd.logs` - Container/systemd logs
2. `hypervisor_inventory` - VMs, Volumes, Images, Networks
3. `hypervisor_node_usage` - Node CPU/Memory
4. `hypervisor_pod_usage` - Pod CPU/Memory  
5. `hypervisor_collector_status` - Health check
6. `hypervisor_dashboard_summary` - Dashboard counts
7. `hypervisor_capacity` - Capacity metrics
8. `hypervisor_events` - Kubernetes events
9. `hypervisor_cluster_metrics` - Prometheus cluster metrics
10. `hypervisor_vm_metrics` - Prometheus VM metrics

### Update Procedure

When `reference/hypervisor-collector/models.py` changes:

1. **Review changes** in models.py
2. **Update init-db.sql** with new/changed tables
3. **Update "Last Sync" date** in this file and init-db.sql header
4. **Test** with fresh database:
   ```bash
   docker compose -p hvcollector down -v
   docker compose -p hvcollector up -d
   docker exec -it hypervisor-postgres psql -U postgres -d hypervisor -c "\dt; \dt fluentd.*"
   ```

### Future Improvement

For production-grade deployments, consider:
- Moving migrations into the `hypervisor-collector` container
- Using Alembic `upgrade head` on container startup
- Automatic schema sync with code changes

### Manual Migration for Existing Installations

If schema changes after initial installation:

```bash
# Connect to database
docker exec -it hypervisor-postgres psql -U postgres -d hypervisor

# Run ALTER TABLE statements manually
# Example:
ALTER TABLE hypervisor_inventory ADD COLUMN new_field VARCHAR(255);
```
