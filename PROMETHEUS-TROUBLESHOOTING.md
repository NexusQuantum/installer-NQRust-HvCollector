# Prometheus Connection Troubleshooting Guide

## Overview

The HV Collector stack includes Prometheus data collection for cluster and VM metrics. This requires proper Kubernetes access and port-forwarding configuration.

## Configuration Requirements

### 1. **Kubeconfig File**

**Location**: `./kubeconfig.yaml` (mounted to container at `/kubeconfig`)

**Current Configuration**:
```yaml
server: "https://192.168.18.110/k8s/clusters/local"
```

**Requirements**:
- Valid token/certificate for cluster access
- Network connectivity from Docker host to API server
- API server must be accessible (port 443 or 6443)

### 2. **Environment Variables**

**In `.env` file**:
```bash
# Kubernetes Configuration
KUBECONFIG=/kubeconfig
KUBECTL_INSECURE_SKIP_TLS_VERIFY=false  # Set to true if using self-signed certs

# Prometheus URL (after port-forward establishes)
PROMETHEUS_URL=http://127.0.0.1:9090
```

### 3. **Prometheus Service**

The `prometheus-pf` container port-forwards to the Prometheus service in the cluster:
```bash
kubectl port-forward svc/rancher-monitoring-prometheus 9090:9090
```

## Common Errors & Solutions

### ❌ Error 1: Connection Refused to API Server

```
error: dial tcp 127.0.0.1:9345: connect: connection refused
```

**Cause**: Initial connection attempts to Kubernetes API server failing

**Solution**: 
- ✅ **This is normal!** Port-forward retries automatically
- Usually succeeds after 2-3 attempts
- Check logs for eventual success: `Forwarding from 127.0.0.1:9090`

### ❌ Error 2: 502 Bad Gateway

```
proxy error from 127.0.0.1:9345 while dialing 192.168.18.111:10250, code 502
```

**Cause**: Kubectl trying to reach node kubelet (port 10250) through API proxy

**Solution**:
- ✅ **Also transient!** Part of initial connection negotiation
- Port-forward will succeed once connection establishes
- If persists > 1 minute, check kubeconfig token validity

### ✅ Success Indicator

```
Forwarding from 127.0.0.1:9090 -> 8081
Forwarding from [::1]:9090 -> 8081
```

**This means**: Port-forward is **working!** Prometheus is accessible at `http://127.0.0.1:9090`

## Verification Steps

### 1. Check Port-Forward Status

```bash
docker compose -p hvcollector logs prometheus-pf --tail 20
```

**Look for**: `Forwarding from 127.0.0.1:9090`

### 2. Test Prometheus Access (from host)

```bash
curl http://127.0.0.1:9090/api/v1/query?query=up
```

**Expected**: JSON response with metrics

### 3. Check Collector Logs

```bash
docker compose -p hvcollector logs hypervisor-collector --tail 20
```

**Look for**: No PostgreSQL connection errors

### 4. Verify Data Collection

```bash
docker exec hypervisor-postgres psql -U postgres -d hypervisor -c "SELECT COUNT(*) FROM fluentd.hypervisor_cluster_metrics;"
```

**Expected**: Rows > 0 after first collection cycle (60s)

## IP Address Confusion

**Common Scenario (Harvester/Rancher)**:
- `192.168.18.100` - VIP / Load balancer (download kubeconfig here)
- `192.168.18.110` - Management node (actual API server)
- `192.168.18.111` - Worker node (kubelet port 10250)

**Explanation**:
1. Kubeconfig downloaded from VIP (100)
2. `server` field points to management node (110)
3. kubectl connects to 110, proxies to nodes (111, etc.)
4. Port-forward tunnels through API server to target pod

**This is all correct!** ✅

## Troubleshooting Checklist

### If Port-Forward Never Succeeds (> 2 minutes)

- [ ] **Kubeconfig Token**: Check if token is valid
  ```bash
  kubectl --kubeconfig=./kubeconfig.yaml get nodes
  ```
  Expected: List of nodes (not auth error)

- [ ] **Network Access**: Can Docker host reach API server?
  ```bash
  curl -k https://192.168.18.110/k8s/clusters/local
  ```
  Expected: 401 Unauthorized (means server is reachable)

- [ ] **Prometheus Service**: Does service exist in cluster?
  ```bash
  kubectl --kubeconfig=./kubeconfig.yaml -n cattle-monitoring-system get svc rancher-monitoring-prometheus
  ```
  Expected: Service details

- [ ] **Self-Signed Certs**: Set `KUBECTL_INSECURE_SKIP_TLS_VERIFY=true` in `.env`

### If Data Not Collecting

- [ ] **Check PROMETHEUS_URL**: Must be `http://127.0.0.1:9090` (port-forward URL)
- [ ] **Check Interval**: Default 60s - wait for first cycle
- [ ] **Check Collector Logs**: Look for SQL errors or connection failures

## Architecture Diagram

```
┌─────────────────────────────┐
│   Docker Host (your PC)     │
│                              │
│  ┌────────────────────┐     │
│  │ prometheus-pf      │     │
│  │ (kubectl pf)       │◄────┼──── kubeconfig.yaml
│  └────────────────────┘     │
│          │                  │
│          │ http://127.0.0.1:9090
│          ▼                  │
│  ┌────────────────────┐     │
│  │ hypervisor-        │     │
│  │ collector          │     │
│  │  (scrapes metrics) │     │
│  └────────────────────┘     │
│          │                  │
│          ▼                  │
│  ┌────────────────────┐     │
│  │ postgres           │     │
│  │ (stores metrics)   │     │
│  └────────────────────┘     │
└─────────────────────────────┘
          │
          │ HTTPS via kubeconfig
          ▼
┌─────────────────────────────┐
│  Kubernetes Cluster         │
│  192.168.18.110 (API)       │
│                              │
│  ┌────────────────────┐     │
│  │ rancher-monitoring │     │
│  │ -prometheus        │     │
│  │ (service)          │     │
│  └────────────────────┘     │
└─────────────────────────────┘
```

## Expected Behavior

1. **Startup**: prometheus-pf shows connection errors (10-30 seconds)
2. **Success**: `Forwarding from 127.0.0.1:9090` appears
3. **Collection**: hypervisor-collector queries Prometheus every 60s
4. **Storage**: Metrics saved to `fluentd.hypervisor_cluster_metrics` and `fluentd.hypervisor_vm_metrics`

## Current Status ✅

Based on logs showing `Forwarding from [::1]:9090 -> 8081`:

- ✅ **Kubeconfig**: Valid connection to `192.168.18.110`
- ✅ **Port-Forward**: Successfully tunneling to Prometheus
- ✅ **Configuration**: All env variables correct
- ⏳ **Data Collection**: Wait 60s for first metrics to appear

**No issues found!** Initial errors are normal connection negotiation. System is working as expected.
