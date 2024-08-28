use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use tauri::api::path::app_config_dir;

#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    database_name: String,
}

#[derive(Serialize, Deserialize)]
struct AppSettings {
    theme: String,
    language: String,
    timezone: String,
    currency: String,
}

#[derive(Serialize, Deserialize)]
struct UserManagement {
    roles: Vec<String>,
    permissions: Value,
}

#[derive(Serialize, Deserialize)]
struct InventorySettings {
    default_category: String,
    low_stock_threshold: u32,
    enable_auto_restock: bool,
    auto_restock_level: u32,
}

#[derive(Serialize, Deserialize)]
struct EmailSettings {
    enabled: bool,
    smtp_server: String,
    smtp_port: u16,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SmsSettings {
    enabled: bool,
    api_endpoint: String,
    api_key: String,
}

#[derive(Serialize, Deserialize)]
struct NotificationSettings {
    email: EmailSettings,
    sms: SmsSettings,
}

#[derive(Serialize, Deserialize)]
struct PaymentGatewaySettings {
    provider: String,
    api_key: String,
}

#[derive(Serialize, Deserialize)]
struct ShippingServiceSettings {
    provider: String,
    api_key: String,
}

#[derive(Serialize, Deserialize)]
struct IntegrationSettings {
    payment_gateway: PaymentGatewaySettings,
    shipping_service: ShippingServiceSettings,
}

#[derive(Serialize, Deserialize)]
struct AccessControl {
    max_login_attempts: u32,
    lockout_duration: u32,
}

#[derive(Serialize, Deserialize)]
struct SecuritySettings {
    enable_encryption: bool,
    encryption_key: String,
    access_control: AccessControl,
}

#[derive(Serialize, Deserialize)]
struct ReportSchedule {
    daily: bool,
    weekly: bool,
    monthly: bool,
}

#[derive(Serialize, Deserialize)]
struct ReportingSettings {
    default_report_format: String,
    enable_scheduled_reports: bool,
    report_schedule: ReportSchedule,
}

#[derive(Serialize, Deserialize)]
struct Localization {
    language: String,
    date_format: String,
    time_format: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    database: DatabaseConfig,
    app_settings: AppSettings,
    user_management: UserManagement,
    inventory_settings: InventorySettings,
    notification_settings: NotificationSettings,
    integration_settings: IntegrationSettings,
    security_settings: SecuritySettings,
    reporting_settings: ReportingSettings,
    localization: Localization,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_config_file,
            read_config_file,
            edit_config_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_config_file(config: Config) -> Result<(), String> {
    let config_dir =
        app_config_dir(&tauri::Config::default()).ok_or("Failed to get config directory")?;
    let config_file_path = config_dir.join("marques.json");

    println!("Config file path: {:?}", config_file_path);

    let config_data = serde_json::to_string(&config).map_err(|e| e.to_string())?;

    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let mut file = File::create(config_file_path).map_err(|e| e.to_string())?;
    file.write_all(config_data.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn read_config_file() -> Result<String, String> {
    let config_dir =
        app_config_dir(&tauri::Config::default()).ok_or("Failed to get config directory")?;
    let config_file_path = config_dir.join("marques.json");

    let config_content = fs::read_to_string(config_file_path).map_err(|e| e.to_string())?;
    Ok(config_content)
}

#[tauri::command]
fn edit_config_file(updated_config: Value) -> Result<(), String> {
    let config_dir =
        app_config_dir(&tauri::Config::default()).ok_or("Failed to get config directory")?;
    let config_file_path = config_dir.join("marques.json");

    let config_content = fs::read_to_string(&config_file_path).map_err(|e| e.to_string())?;
    let mut config_json: Value =
        serde_json::from_str(&config_content).map_err(|e| e.to_string())?;

    if let Some(database) = updated_config.get("database") {
        if let Some(host) = database.get("host") {
            config_json["database"]["host"] = host.clone();
        }
        if let Some(port) = database.get("port") {
            config_json["database"]["port"] = port.clone();
        }
        if let Some(user) = database.get("user") {
            config_json["database"]["user"] = user.clone();
        }
        if let Some(password) = database.get("password") {
            config_json["database"]["password"] = password.clone();
        }
        if let Some(database_name) = database.get("database_name") {
            config_json["database"]["database_name"] = database_name.clone();
        }
    }

    if let Some(app_settings) = updated_config.get("app_settings") {
        if let Some(theme) = app_settings.get("theme") {
            config_json["app_settings"]["theme"] = theme.clone();
        }
        if let Some(language) = app_settings.get("language") {
            config_json["app_settings"]["language"] = language.clone();
        }
        if let Some(timezone) = app_settings.get("timezone") {
            config_json["app_settings"]["timezone"] = timezone.clone();
        }
        if let Some(currency) = app_settings.get("currency") {
            config_json["app_settings"]["currency"] = currency.clone();
        }
    }

    if let Some(user_management) = updated_config.get("user_management") {
        if let Some(roles) = user_management.get("roles") {
            config_json["user_management"]["roles"] = roles.clone();
        }
        if let Some(permissions) = user_management.get("permissions") {
            config_json["user_management"]["permissions"] = permissions.clone();
        }
    }

    if let Some(inventory_settings) = updated_config.get("inventory_settings") {
        if let Some(default_category) = inventory_settings.get("default_category") {
            config_json["inventory_settings"]["default_category"] = default_category.clone();
        }
        if let Some(low_stock_threshold) = inventory_settings.get("low_stock_threshold") {
            config_json["inventory_settings"]["low_stock_threshold"] = low_stock_threshold.clone();
        }
        if let Some(enable_auto_restock) = inventory_settings.get("enable_auto_restock") {
            config_json["inventory_settings"]["enable_auto_restock"] = enable_auto_restock.clone();
        }
        if let Some(auto_restock_level) = inventory_settings.get("auto_restock_level") {
            config_json["inventory_settings"]["auto_restock_level"] = auto_restock_level.clone();
        }
    }

    if let Some(notification_settings) = updated_config.get("notification_settings") {
        if let Some(email) = notification_settings.get("email") {
            if let Some(enabled) = email.get("enabled") {
                config_json["notification_settings"]["email"]["enabled"] = enabled.clone();
            }
            if let Some(smtp_server) = email.get("smtp_server") {
                config_json["notification_settings"]["email"]["smtp_server"] = smtp_server.clone();
            }
            if let Some(smtp_port) = email.get("smtp_port") {
                config_json["notification_settings"]["email"]["smtp_port"] = smtp_port.clone();
            }
            if let Some(username) = email.get("username") {
                config_json["notification_settings"]["email"]["username"] = username.clone();
            }
            if let Some(password) = email.get("password") {
                config_json["notification_settings"]["email"]["password"] = password.clone();
            }
        }
        if let Some(sms) = notification_settings.get("sms") {
            if let Some(enabled) = sms.get("enabled") {
                config_json["notification_settings"]["sms"]["enabled"] = enabled.clone();
            }
            if let Some(api_endpoint) = sms.get("api_endpoint") {
                config_json["notification_settings"]["sms"]["api_endpoint"] = api_endpoint.clone();
            }
            if let Some(api_key) = sms.get("api_key") {
                config_json["notification_settings"]["sms"]["api_key"] = api_key.clone();
            }
        }
    }

    if let Some(integration_settings) = updated_config.get("integration_settings") {
        if let Some(payment_gateway) = integration_settings.get("payment_gateway") {
            if let Some(provider) = payment_gateway.get("provider") {
                config_json["integration_settings"]["payment_gateway"]["provider"] =
                    provider.clone();
            }
            if let Some(api_key) = payment_gateway.get("api_key") {
                config_json["integration_settings"]["payment_gateway"]["api_key"] = api_key.clone();
            }
        }
        if let Some(shipping_service) = integration_settings.get("shipping_service") {
            if let Some(provider) = shipping_service.get("provider") {
                config_json["integration_settings"]["shipping_service"]["provider"] =
                    provider.clone();
            }
            if let Some(api_key) = shipping_service.get("api_key") {
                config_json["integration_settings"]["shipping_service"]["api_key"] =
                    api_key.clone();
            }
        }
    }

    if let Some(security_settings) = updated_config.get("security_settings") {
        if let Some(enable_encryption) = security_settings.get("enable_encryption") {
            config_json["security_settings"]["enable_encryption"] = enable_encryption.clone();
        }
        if let Some(encryption_key) = security_settings.get("encryption_key") {
            config_json["security_settings"]["encryption_key"] = encryption_key.clone();
        }
        if let Some(access_control) = security_settings.get("access_control") {
            if let Some(max_login_attempts) = access_control.get("max_login_attempts") {
                config_json["security_settings"]["access_control"]["max_login_attempts"] =
                    max_login_attempts.clone();
            }
            if let Some(lockout_duration) = access_control.get("lockout_duration") {
                config_json["security_settings"]["access_control"]["lockout_duration"] =
                    lockout_duration.clone();
            }
        }
    }

    if let Some(reporting_settings) = updated_config.get("reporting_settings") {
        if let Some(default_report_format) = reporting_settings.get("default_report_format") {
            config_json["reporting_settings"]["default_report_format"] =
                default_report_format.clone();
        }
        if let Some(enable_scheduled_reports) = reporting_settings.get("enable_scheduled_reports") {
            config_json["reporting_settings"]["enable_scheduled_reports"] =
                enable_scheduled_reports.clone();
        }
        if let Some(report_schedule) = reporting_settings.get("report_schedule") {
            if let Some(daily) = report_schedule.get("daily") {
                config_json["reporting_settings"]["report_schedule"]["daily"] = daily.clone();
            }
            if let Some(weekly) = report_schedule.get("weekly") {
                config_json["reporting_settings"]["report_schedule"]["weekly"] = weekly.clone();
            }
            if let Some(monthly) = report_schedule.get("monthly") {
                config_json["reporting_settings"]["report_schedule"]["monthly"] = monthly.clone();
            }
        }
    }

    if let Some(localization) = updated_config.get("localization") {
        if let Some(language) = localization.get("language") {
            config_json["localization"]["language"] = language.clone();
        }
        if let Some(date_format) = localization.get("date_format") {
            config_json["localization"]["date_format"] = date_format.clone();
        }
        if let Some(time_format) = localization.get("time_format") {
            config_json["localization"]["time_format"] = time_format.clone();
        }
    }

    let mut file = File::create(config_file_path).map_err(|e| e.to_string())?;
    file.write_all(config_json.to_string().as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
