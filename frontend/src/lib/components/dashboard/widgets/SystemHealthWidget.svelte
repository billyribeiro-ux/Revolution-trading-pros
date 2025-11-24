<script lang="ts">
  import type { SystemHealthData } from '$lib/types/dashboard';

  export let data: SystemHealthData | undefined;
  export let config: {
    show_all_services?: boolean;
    services_filter?: string[];
    refresh_rate?: number;
    show_metrics?: boolean;
  } = {};

  // Apply config filters
  $: filteredServices = config.services_filter 
    ? Object.entries(data?.services || {}).filter(([name]) => config.services_filter?.includes(name))
    : Object.entries(data?.services || {});

  function getStatusColor(status: string): string {
    switch (status) {
      case 'healthy': return '#10b981';
      case 'warning': return '#f59e0b';
      case 'critical': return '#ef4444';
      default: return '#6b7280';
    }
  }

  function getStatusIcon(status: string): string {
    switch (status) {
      case 'healthy': return '✓';
      case 'warning': return '⚠';
      case 'critical': return '✕';
      default: return '?';
    }
  }
</script>

{#if data}
  <div class="system-health">
    <div class="overall-status" style="border-color: {getStatusColor(data.overall_status)}">
      <span class="status-icon" style="color: {getStatusColor(data.overall_status)}">
        {getStatusIcon(data.overall_status)}
      </span>
      <div>
        <div class="status-label">Overall System Status</div>
        <div class="status-value" style="color: {getStatusColor(data.overall_status)}">
          {data.overall_status.toUpperCase()}
        </div>
      </div>
    </div>

    <div class="services-grid">
      {#each filteredServices as [serviceName, serviceData]}
        <div class="service-item">
          <div class="service-header">
            <span class="service-name">{serviceName}</span>
            <span 
              class="service-status" 
              style="background: {getStatusColor(serviceData.overall_status)}20; color: {getStatusColor(serviceData.overall_status)}"
            >
              {serviceData.overall_status}
            </span>
          </div>
          {#if config.show_metrics !== false && (serviceData.critical_count > 0 || serviceData.warning_count > 0)}
            <div class="service-issues">
              {#if serviceData.critical_count > 0}
                <span class="issue critical">{serviceData.critical_count} critical</span>
              {/if}
              {#if serviceData.warning_count > 0}
                <span class="issue warning">{serviceData.warning_count} warnings</span>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>

    <div class="last-updated">
      Last updated: {new Date(data.last_updated).toLocaleTimeString()}
    </div>
  </div>
{:else}
  <div class="no-data">Loading system health data...</div>
{/if}

<style>
  .system-health {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .overall-status {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    border-left: 4px solid;
    background: #f9fafb;
    border-radius: 8px;
  }

  .status-icon {
    font-size: 2rem;
    font-weight: bold;
  }

  .status-label {
    font-size: 0.75rem;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .status-value {
    font-size: 1.25rem;
    font-weight: 700;
    margin-top: 0.25rem;
  }

  .services-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .service-item {
    padding: 0.75rem;
    background: #f9fafb;
    border-radius: 8px;
    border: 1px solid #e5e7eb;
  }

  .service-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .service-name {
    font-weight: 600;
    color: #1f2937;
    text-transform: capitalize;
  }

  .service-status {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-weight: 500;
    text-transform: uppercase;
  }

  .service-issues {
    display: flex;
    gap: 0.5rem;
    font-size: 0.75rem;
  }

  .issue {
    padding: 0.125rem 0.5rem;
    border-radius: 4px;
  }

  .issue.critical {
    background: #fee;
    color: #c00;
  }

  .issue.warning {
    background: #fef3c7;
    color: #92400e;
  }

  .last-updated {
    font-size: 0.75rem;
    color: #6b7280;
    text-align: center;
  }

  .no-data {
    text-align: center;
    color: #6b7280;
    padding: 2rem;
  }
</style>
