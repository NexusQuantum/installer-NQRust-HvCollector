#[derive(Debug, Clone, PartialEq)]
pub enum FocusState {
    Field(usize),
    SaveButton,
    CancelButton,
}

#[derive(Debug, Clone)]
pub struct FormData {
    // PostgreSQL settings
    pub(crate) postgres_host: String,
    pub(crate) postgres_user: String,
    pub(crate) postgres_password: String,
    pub(crate) postgres_port: String,
    pub(crate) postgres_db: String,
    pub(crate) postgres_schema: String,
    
    // Hypervisor/SSH settings
    pub(crate) hypervisor_host: String,
    pub(crate) hypervisor_user: String,
    pub(crate) hypervisor_password: String,
    
    // Collector settings (optional, has defaults)
    pub(crate) cluster_name: String,
    pub(crate) interval_seconds: String,
    pub(crate) prometheus_url: String,
    
    pub(crate) focus_state: FocusState,
    pub(crate) error_message: String,
}

impl FormData {
    pub fn new() -> Self {
        Self {
            // PostgreSQL defaults (container on localhost)
            postgres_host: "postgres".to_string(),
            postgres_user: "postgres".to_string(),
            postgres_password: String::new(),
            postgres_port: "5432".to_string(),
            postgres_db: "hypervisor".to_string(),
            postgres_schema: "fluentd".to_string(),
            
            // Hypervisor defaults
            hypervisor_host: String::new(),
            hypervisor_user: String::new(),
            hypervisor_password: String::new(),
            
            // Collector defaults
            cluster_name: "harvester".to_string(),
            interval_seconds: "60".to_string(),
            prometheus_url: "http://127.0.0.1:9090".to_string(),
            
            focus_state: FocusState::Field(0),
            error_message: String::new(),
        }
    }

    pub fn validate(&mut self) -> bool {
        // Required fields
        if self.postgres_host.trim().is_empty() {
            self.error_message = "PostgreSQL Host is required!".to_string();
            return false;
        }
        
        if self.postgres_password.trim().is_empty() {
            self.error_message = "PostgreSQL Password is required!".to_string();
            return false;
        }
        
        if self.hypervisor_host.trim().is_empty() {
            self.error_message = "Hypervisor Host is required!".to_string();
            return false;
        }
        
        if self.hypervisor_password.trim().is_empty() {
            self.error_message = "Hypervisor Password is required!".to_string();
            return false;
        }

        // Validate port is numeric
        if let Err(_) = self.postgres_port.parse::<u16>() {
            self.error_message = "PostgreSQL Port must be a valid number!".to_string();
            return false;
        }

        // Validate interval_seconds is numeric
        if let Err(_) = self.interval_seconds.parse::<u32>() {
            self.error_message = "Interval Seconds must be a valid number!".to_string();
            return false;
        }

        self.error_message.clear();
        true
    }

    pub fn get_current_value_mut(&mut self) -> &mut String {
        match &self.focus_state {
            FocusState::Field(idx) => match idx {
                0 => &mut self.postgres_host,
                1 => &mut self.postgres_user,
                2 => &mut self.postgres_password,
                3 => &mut self.postgres_port,
                4 => &mut self.postgres_db,
                5 => &mut self.postgres_schema,
                6 => &mut self.hypervisor_host,
                7 => &mut self.hypervisor_user,
                8 => &mut self.hypervisor_password,
                9 => &mut self.cluster_name,
                10 => &mut self.interval_seconds,
                11 => &mut self.prometheus_url,
                _ => &mut self.postgres_host,
            },
            _ => &mut self.postgres_host,
        }
    }

    pub fn get_total_fields(&self) -> usize {
        12 // All fields
    }
    
    pub fn get_field_label(&self, idx: usize) -> &str {
        match idx {
            0 => "PostgreSQL Host",
            1 => "PostgreSQL User",
            2 => "PostgreSQL Password",
            3 => "PostgreSQL Port",
            4 => "PostgreSQL Database",
            5 => "PostgreSQL Schema",
            6 => "Hypervisor Host",
            7 => "Hypervisor User",
            8 => "Hypervisor Password",
            9 => "Cluster Name",
            10 => "Interval Seconds",
            11 => "Prometheus URL (optional)",
            _ => "Unknown",
        }
    }
    
    pub fn is_field_required(&self, idx: usize) -> bool {
        // Required fields: 0, 2, 6, 8 (postgres host/pass, hypervisor host/pass)
        matches!(idx, 0 | 2 | 6 | 8)
    }
    
    pub fn is_password_field(&self, idx: usize) -> bool {
        matches!(idx, 2 | 8) // postgres_password, hypervisor_password
    }
}
