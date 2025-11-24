<script lang="ts">
  import DashboardGrid from '$lib/components/dashboard/DashboardGrid.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let isAdmin = false;

  onMount(async () => {
    // Check if user is admin
    // This should integrate with your auth system
    const user = localStorage.getItem('user');
    if (user) {
      const userData = JSON.parse(user);
      isAdmin = userData.role === 'admin' || userData.is_admin;
    }

    if (!isAdmin) {
      goto('/dashboard');
    }
  });
</script>

<svelte:head>
  <title>Admin Dashboard | Revolution Trading Pros</title>
</svelte:head>

{#if isAdmin}
  <div class="admin-dashboard-page">
    <header class="dashboard-header">
      <div class="header-content">
        <h1>Admin Dashboard</h1>
        <p class="subtitle">System overview and management</p>
      </div>
      <div class="header-actions">
        <button class="btn-secondary">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M5 12h14"/>
          </svg>
          Add Widget
        </button>
        <button class="btn-secondary">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 20h9M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
          </svg>
          Customize
        </button>
      </div>
    </header>

    <main class="dashboard-main">
      <DashboardGrid dashboardType="admin" />
    </main>
  </div>
{/if}

<style>
  .admin-dashboard-page {
    min-height: 100vh;
    background: #f8f9fa;
  }

  .dashboard-header {
    background: white;
    border-bottom: 1px solid #e5e7eb;
    padding: 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1.5rem;
  }

  .header-content h1 {
    font-size: 2rem;
    font-weight: 700;
    color: #1f2937;
    margin: 0 0 0.5rem 0;
  }

  .subtitle {
    color: #6b7280;
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 1rem;
  }

  .btn-secondary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: white;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    color: #374151;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    background: #f9fafb;
    border-color: #9ca3af;
  }

  .dashboard-main {
    padding: 0;
  }
</style>
