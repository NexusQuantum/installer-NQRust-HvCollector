-- HV Collector PostgreSQL Initialization Script
-- Creates schema and all tables for Hypervisor Collector
--
-- 📌 IMPORTANT: Manual Schema Sync Required
-- This file must be manually updated when reference/hypervisor-collector/models.py changes.
-- For production deployments, consider implementing Alembic migrations in the container.
-- Last synced with: reference/hypervisor-collector/models.py (2026-02-11)
--
-- ⚠️ NOTE: All tables are in 'fluentd' schema (hardcoded - PostgreSQL init doesn't support env vars)
--

-- Create schema
CREATE SCHEMA IF NOT EXISTS fluentd;

-- =============================================================================
-- FLUENTD LOGS TABLE
-- =============================================================================
CREATE TABLE IF NOT EXISTS fluentd.logs (
    id BIGSERIAL PRIMARY KEY,
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    tag VARCHAR(255),
    node VARCHAR(255),
    namespace VARCHAR(255),
    pod VARCHAR(255),
    container VARCHAR(255),
    stream VARCHAR(50),
    message TEXT,
    raw TEXT,
    level VARCHAR(32),
    syslog_priority INTEGER,
    systemd_unit VARCHAR(256),
    syslog_identifier VARCHAR(128),
    process_name VARCHAR(128),
    process_id INTEGER,
    systemd_cgroup VARCHAR(512),
    boot_id VARCHAR(64),
    machine_id VARCHAR(64),
    systemd_slice VARCHAR(256),
    transport VARCHAR(32)
);

-- =============================================================================
-- HYPERVISOR COLLECTOR TABLES
-- =============================================================================

-- 1. Hypervisor Inventory (VMs, Volumes, Images, Networks, etc.)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_inventory (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    source VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    "apiVersion" VARCHAR(255),
    kind VARCHAR(255),
    uid VARCHAR(255),
    node VARCHAR(255),
    phase VARCHAR(255),
    state VARCHAR(255),
    size_bytes BIGINT,
    robustness VARCHAR(255),
    actual_size_bytes BIGINT,
    pvc_name VARCHAR(255),
    pv_name VARCHAR(255),
    k8s_namespace VARCHAR(255),
    workload_name VARCHAR(255),
    workload_type VARCHAR(255),
    workload_status VARCHAR(255),
    node_ready VARCHAR(255),
    pod_restart_count INTEGER,
    raw TEXT,
    PRIMARY KEY (ts, cluster, source, namespace, name)
);

-- 2. Hypervisor Node Usage (CPU, Memory per node)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_node_usage (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    node VARCHAR(255) NOT NULL,
    cpu_mcores INTEGER,
    cpu_pct INTEGER,
    mem_bytes BIGINT,
    mem_pct INTEGER,
    raw TEXT,
    PRIMARY KEY (ts, cluster, node)
);

-- 3. Hypervisor Pod Usage (CPU, Memory per pod)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_pod_usage (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) NOT NULL,
    pod VARCHAR(255) NOT NULL,
    cpu_mcores INTEGER,
    mem_bytes BIGINT,
    raw TEXT,
    PRIMARY KEY (ts, cluster, namespace, pod)
);

-- 4. Hypervisor Collector Status (health check)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_collector_status (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    ok INTEGER,
    message TEXT,
    raw TEXT,
    PRIMARY KEY (ts, cluster)
);

-- 5. Hypervisor Dashboard Summary (counts: hosts, VMs, networks, etc.)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_dashboard_summary (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    hosts INTEGER,
    vms_total INTEGER,
    vms_running INTEGER,
    networks INTEGER,
    images INTEGER,
    volumes INTEGER,
    disks INTEGER,
    raw TEXT,
    PRIMARY KEY (ts, cluster)
);

-- 6. Hypervisor Capacity (CPU, Memory, Storage capacity/usage)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_capacity (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    cpu_capacity_cores INTEGER,
    cpu_reserved_cores INTEGER,
    cpu_used_cores INTEGER,
    mem_capacity_bytes BIGINT,
    mem_reserved_bytes BIGINT,
    mem_used_bytes BIGINT,
    storage_capacity_bytes BIGINT,
    storage_allocated_bytes BIGINT,
    storage_used_bytes BIGINT,
    raw TEXT,
    PRIMARY KEY (ts, cluster)
);

-- 7. Hypervisor Events (K8s events: FailedScheduling, OOMKilled, etc.)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_events (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    involved_kind VARCHAR(255),
    involved_namespace VARCHAR(255),
    involved_name VARCHAR(255),
    reason VARCHAR(255),
    message TEXT,
    event_type VARCHAR(255),
    count INTEGER,
    first_ts TIMESTAMP WITH TIME ZONE,
    last_ts TIMESTAMP WITH TIME ZONE,
    raw TEXT,
    PRIMARY KEY (ts, cluster, namespace, name)
);

-- 8. Hypervisor Cluster Metrics (from Prometheus: CPU%, Load, Memory%, Disk%, I/O, Network)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_cluster_metrics (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    cpu_util_pct FLOAT,
    mem_util_pct FLOAT,
    disk_util_pct FLOAT,
    load_1m FLOAT,
    load_5m FLOAT,
    load_15m FLOAT,
    disk_read_bytes_per_sec FLOAT,
    disk_write_bytes_per_sec FLOAT,
    network_rx_bytes_per_sec FLOAT,
    network_tx_bytes_per_sec FLOAT,
    raw TEXT,
    PRIMARY KEY (ts, cluster)
);

-- 9. Hypervisor VM Metrics (from Prometheus: per-VM CPU, Memory - KubeVirt metrics)
CREATE TABLE IF NOT EXISTS fluentd.hypervisor_vm_metrics (
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    cluster VARCHAR(255) NOT NULL,
    vm_name VARCHAR(255) NOT NULL,
    namespace VARCHAR(255),
    cpu_usage_seconds_total FLOAT,
    mem_used_bytes BIGINT,
    mem_available_bytes BIGINT,
    raw TEXT,
    PRIMARY KEY (ts, cluster, vm_name)
);

-- =============================================================================
-- INDEXES FOR PERFORMANCE
-- =============================================================================

-- Logs indexes
CREATE INDEX IF NOT EXISTS idx_logs_ts ON fluentd.logs(ts);
CREATE INDEX IF NOT EXISTS idx_logs_tag ON fluentd.logs(tag);
CREATE INDEX IF NOT EXISTS idx_logs_namespace ON fluentd.logs(namespace);
CREATE INDEX IF NOT EXISTS idx_logs_level ON fluentd.logs(level);
CREATE INDEX IF NOT EXISTS idx_logs_systemd_unit ON fluentd.logs(systemd_unit);
CREATE INDEX IF NOT EXISTS idx_logs_syslog_identifier ON fluentd.logs(syslog_identifier);

-- Inventory indexes
CREATE INDEX IF NOT EXISTS idx_inventory_ts ON fluentd.hypervisor_inventory(ts);
CREATE INDEX IF NOT EXISTS idx_inventory_cluster ON fluentd.hypervisor_inventory(cluster);
CREATE INDEX IF NOT EXISTS idx_inventory_kind ON fluentd.hypervisor_inventory(kind);
CREATE INDEX IF NOT EXISTS idx_inventory_node ON fluentd.hypervisor_inventory(node);

-- Node usage indexes
CREATE INDEX IF NOT EXISTS idx_node_usage_ts ON fluentd.hypervisor_node_usage(ts);
CREATE INDEX IF NOT EXISTS idx_node_usage_cluster ON fluentd.hypervisor_node_usage(cluster);

-- Pod usage indexes
CREATE INDEX IF NOT EXISTS idx_pod_usage_ts ON fluentd.hypervisor_pod_usage(ts);
CREATE INDEX IF NOT EXISTS idx_pod_usage_cluster ON fluentd.hypervisor_pod_usage(cluster);

-- Events indexes
CREATE INDEX IF NOT EXISTS idx_events_ts ON fluentd.hypervisor_events(ts);
CREATE INDEX IF NOT EXISTS idx_events_cluster ON fluentd.hypervisor_events(cluster);
CREATE INDEX IF NOT EXISTS idx_events_reason ON fluentd.hypervisor_events(reason);

-- Cluster metrics indexes
CREATE INDEX IF NOT EXISTS idx_cluster_metrics_ts ON fluentd.hypervisor_cluster_metrics(ts);

-- VM metrics indexes
CREATE INDEX IF NOT EXISTS idx_vm_metrics_ts ON fluentd.hypervisor_vm_metrics(ts);

-- =============================================================================
-- GRANT PERMISSIONS
-- =============================================================================

-- Grant schema permissions
GRANT USAGE ON SCHEMA fluentd TO postgres;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA fluentd TO postgres;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA fluentd TO postgres;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA fluentd GRANT ALL ON TABLES TO postgres;
ALTER DEFAULT PRIVILEGES IN SCHEMA fluentd GRANT ALL ON SEQUENCES TO postgres;
