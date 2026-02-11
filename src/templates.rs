// HV Collector installer - simplified template system
// No AI provider configs needed, just docker-compose template

use color_eyre::Result;
use std::fs;
use std::path::Path;

/// Write fluentd configuration files to the project directory
pub fn write_fluentd_config(project_dir: &Path) -> Result<()> {
    const FLUENT_CONF: &str = include_str!("../fluentd/fluent.conf");
    
    let fluentd_dir = project_dir.join("fluentd");
    fs::create_dir_all(&fluentd_dir)?;
    
    let conf_path = fluentd_dir.join("fluent.conf");
    fs::write(&conf_path, FLUENT_CONF)?;
    
    Ok(())
}

/// Write fluentd Dockerfile to the project directory
pub fn write_fluentd_dockerfile(project_dir: &Path) -> Result<()> {
    const FLUENT_DOCKERFILE: &str = include_str!("../fluentd/Dockerfile");
    
    let fluentd_dir = project_dir.join("fluentd");
    fs::create_dir_all(&fluentd_dir)?;
    
    let dockerfile_path = fluentd_dir.join("Dockerfile");
    fs::write(&dockerfile_path, FLUENT_DOCKERFILE)?;
    
    Ok(())
}
