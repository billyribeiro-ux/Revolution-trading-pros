# Dashboard System - Quick Start Guide

## 🚀 Getting Started

### Backend Setup (5 minutes)

1. **Run Migrations**
```bash
cd backend
php artisan migrate
```

2. **Register API Routes**

Add to `routes/api.php`:
```php
require __DIR__.'/api_dashboard.php';
```

3. **Test API** (Optional)
```bash
php artisan serve
```

Visit: `http://localhost:8000/api/dashboards` (requires authentication)

### Frontend Setup (2 minutes)

Dashboard components are ready to use! No additional setup needed.

## 📊 Usage Examples

### Display User Dashboard

```svelte
<!-- routes/dashboard/+page.svelte -->
<script>
  import DashboardGrid from '$lib/components/dashboard/DashboardGrid.svelte';
</script>

<DashboardGrid dashboardType="user" />
```

### Display Admin Dashboard

```svelte
<!-- routes/admin/dashboard/+page.svelte -->
<script>
  import DashboardGrid from '$lib/components/dashboard/DashboardGrid.svelte';
</script>

<DashboardGrid dashboardType="admin" />
```

### Programmatic Dashboard Control

```typescript
import { dashboardStore } from '$lib/stores/dashboard';

// Load dashboard
await dashboardStore.loadDashboard('user');

// Add widget
await dashboardStore.addWidget({
  widget_type: 'revenue_mrr',
  title: 'Monthly Revenue',
  position_x: 0,
  position_y: 0,
  width: 6,
  height: 4,
  config: { period: '30d' }
});

// Update widget layout
await dashboardStore.updateWidgetLayout('widget-id', {
  x: 6,
  y: 0,
  width: 6,
  height: 4
});

// Remove widget
await dashboardStore.removeWidget('widget-id');

// Refresh widget data
await dashboardStore.refreshWidget('widget-id');
```

## 🎨 Available Widget Types

### Admin Widgets
- `system_health` - System status overview
- `revenue_mrr` - Monthly recurring revenue
- `user_growth` - User growth metrics
- `subscription_churn` - Churn rate analysis
- `email_performance` - Email campaign metrics
- `crm_pipeline` - Sales pipeline overview
- `recent_activity` - Activity timeline

### User Widgets
- `subscription_status` - Current subscription info
- `recent_courses` - Course progress
- `trading_performance` - Trading statistics
- `notifications` - User notifications

### Universal Widgets
- `funnel_conversion` - Conversion funnels
- `behavior_friction` - UX friction points
- `attribution_model` - Marketing attribution
- `automation_runs` - Automation statistics
- `form_submissions` - Form analytics
- `popup_performance` - Popup metrics
- `website_speed` - Performance metrics
- `integration_health` - Integration status

## 🔧 Customization

### Create Custom Widget

1. **Create Widget Component**

```svelte
<!-- frontend/src/lib/components/dashboard/widgets/CustomWidget.svelte -->
<script lang="ts">
  export let data: any;
  export let config: Record<string, any> = {};
</script>

<div class="custom-widget">
  <h3>{config.title || 'Custom Widget'}</h3>
  <div class="content">
    {JSON.stringify(data)}
  </div>
</div>

<style>
  .custom-widget {
    padding: 1rem;
  }
</style>
```

2. **Register in WidgetCard.svelte**

```typescript
import CustomWidget from './widgets/CustomWidget.svelte';

function getWidgetComponent(type: string) {
  switch (type) {
    case 'custom_widget':
      return CustomWidget;
    // ... other cases
  }
}
```

3. **Add Data Provider (Backend)**

```php
// backend/app/Services/Dashboard/WidgetDataProviderService.php

private function fetchWidgetData(DashboardWidget $widget): array
{
    return match ($widget->widget_type) {
        'custom_widget' => $this->getCustomWidgetData($widget),
        // ... other cases
    };
}

private function getCustomWidgetData(DashboardWidget $widget): array
{
    return [
        'value' => 123,
        'label' => 'Custom Metric',
    ];
}
```

## 📱 Responsive Design

Dashboards automatically adapt to screen sizes:
- **Desktop:** 12-column grid
- **Tablet:** Responsive stacking
- **Mobile:** Single column layout

## 🔐 Permissions

Dashboards respect user roles:

```php
// Check if user can access dashboard
DashboardPermission::canUserAccess($dashboardId, $userId, 'can_view');
```

## 🎯 Best Practices

1. **Widget Refresh Intervals**
   - Real-time data: 60 seconds
   - Standard metrics: 300 seconds (5 minutes)
   - Historical data: 3600 seconds (1 hour)

2. **Widget Sizing**
   - Minimum: 2x2 grid units
   - KPI cards: 4x4 or 6x4
   - Charts: 6x6 or 8x6
   - Activity feeds: 4x8

3. **Performance**
   - Limit widgets per dashboard: 12-15
   - Use caching for expensive queries
   - Lazy load widget data

## 🐛 Troubleshooting

### Dashboard not loading?
```typescript
// Check browser console for errors
// Verify API endpoint is accessible
// Ensure user is authenticated
```

### Widgets showing "No data"?
```php
// Check widget data provider
// Verify database has data
// Check cache configuration
```

### Layout not saving?
```typescript
// Verify API permissions
// Check network tab for errors
// Ensure widget IDs are correct
```

## 📚 API Reference

### Get Dashboard
```
GET /api/dashboards?type=user
Authorization: Bearer {token}
```

### Add Widget
```
POST /api/dashboards/{id}/widgets
Content-Type: application/json

{
  "widget_type": "revenue_mrr",
  "title": "Revenue",
  "position_x": 0,
  "position_y": 0,
  "width": 6,
  "height": 4,
  "config": {}
}
```

### Update Widget Layout
```
PUT /api/widgets/{id}/layout
Content-Type: application/json

{
  "x": 6,
  "y": 0,
  "width": 6,
  "height": 4
}
```

### Remove Widget
```
DELETE /api/widgets/{id}
```

## 🎓 Next Steps

1. **Customize default widgets** - Edit `DashboardService::getAdminDefaultWidgets()`
2. **Add custom widgets** - Create new widget components
3. **Enhance data providers** - Integrate with your data sources
4. **Style customization** - Modify widget CSS
5. **Add drag-and-drop** - Implement in Phase 2

## 💡 Tips

- Use `GenericWidget` for rapid prototyping
- Monitor widget refresh intervals to optimize performance
- Cache expensive queries in data providers
- Use activity logs to track user behavior
- Leverage system health metrics for monitoring

---

**Need Help?** Check `DASHBOARD_ARCHITECTURE.md` for detailed documentation.
