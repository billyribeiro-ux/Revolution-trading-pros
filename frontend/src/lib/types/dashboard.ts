export interface Dashboard {
  id: string;
  user_id: number;
  dashboard_type: 'admin' | 'user' | 'custom';
  name: string;
  description?: string;
  is_default: boolean;
  is_public: boolean;
  grid_columns: number;
  grid_row_height: number;
  grid_gap: number;
  shared_with?: number[];
  required_role?: string;
  last_viewed_at?: string;
  created_at: string;
  updated_at: string;
}

export interface DashboardWidget {
  id: string;
  dashboard_id: string;
  widget_type: WidgetType;
  title: string;
  position_x: number;
  position_y: number;
  width: number;
  height: number;
  config?: Record<string, any>;
  data_provider?: string;
  refresh_interval: number;
  is_visible: boolean;
  data?: any;
}

export type WidgetType =
  | 'system_health'
  | 'revenue_mrr'
  | 'user_growth'
  | 'subscription_churn'
  | 'recent_activity'
  | 'email_performance'
  | 'crm_pipeline'
  | 'subscription_status'
  | 'recent_courses'
  | 'trading_performance'
  | 'notifications'
  | 'funnel_conversion'
  | 'behavior_friction'
  | 'attribution_model'
  | 'automation_runs'
  | 'form_submissions'
  | 'popup_performance'
  | 'website_speed'
  | 'integration_health';

export interface WidgetLayout {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface DashboardData {
  dashboard: Dashboard;
  widgets: DashboardWidget[];
}

export interface SystemHealthData {
  overall_status: 'healthy' | 'warning' | 'critical';
  services: Record<string, ServiceHealth>;
  last_updated: string;
}

export interface ServiceHealth {
  service_name: string;
  overall_status: 'healthy' | 'warning' | 'critical';
  metrics: any;
  critical_count: number;
  warning_count: number;
}

export interface ActivityLog {
  id: number;
  user?: {
    id: number;
    name: string;
    email: string;
  };
  entity_type: string;
  entity_id?: number;
  action: string;
  description: string;
  created_at: string;
}
