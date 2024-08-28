export interface GlobalConfig {
  database: Database;
  appSettings: AppSettings;
  userManagement: UserManagement;
  inventorySettings: InventorySettings;
  notificationSettings: NotificationSettings;
  integrationSettings: IntegrationSettings;
  securitySettings: SecuritySettings;
  reportingSettings: ReportingSettings;
  localization: Localization;
}

export interface Database {
  host: string;
  port: number;
  user: string;
  password: string;
  database_name: string;
}

export interface AppSettings {
  theme: string;
  language: string;
  timezone: string;
  currency: string;
}

export interface UserManagement {
  roles: string[];
  permissions: Permissions;
}

export interface Permissions {
  admin: string[];
  manager: string[];
  employee: string[];
}

export interface InventorySettings {
  defaultCategory: string;
  lowStockThreshold: number;
  enableAutoRestock: boolean;
  autoRestockLevel: number;
}

export interface NotificationSettings {
  email: Email;
  sms: Sms;
}

export interface Email {
  enabled: boolean;
  smtpServer: string;
  smtpPort: number;
  username: string;
  password: string;
}

export interface Sms {
  enabled: boolean;
  apiEndpoint: string;
  apiKey: string;
}

export interface IntegrationSettings {
  paymentGateway: PaymentGateway;
  shippingService: ShippingService;
}

export interface PaymentGateway {
  provider: string;
  apiKey: string;
}

export interface ShippingService {
  provider: string;
  apiKey: string;
}

export interface SecuritySettings {
  enableEncryption: boolean;
  encryptionKey: string;
  accessControl: AccessControl;
}

export interface AccessControl {
  maxLoginAttempts: number;
  lockoutDuration: number;
}

export interface ReportingSettings {
  defaultReportFormat: string;
  enableScheduledReports: boolean;
  reportSchedule: ReportSchedule;
}

export interface ReportSchedule {
  daily: boolean;
  weekly: boolean;
  monthly: boolean;
}

export interface Localization {
  language: string;
  dateFormat: string;
  timeFormat: string;
}
