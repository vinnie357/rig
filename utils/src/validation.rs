use anyhow::Result;

/// Validates RFC1035 compliance for network names
pub fn validate_network_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Network name cannot be empty"));
    }

    if name.len() > 63 {
        return Err(anyhow::anyhow!("Network name cannot exceed 63 characters"));
    }

    // Must start and end with alphanumeric character
    let first_char = name.chars().next().unwrap();
    let last_char = name.chars().last().unwrap();

    if !first_char.is_ascii_alphanumeric() || !last_char.is_ascii_alphanumeric() {
        return Err(anyhow::anyhow!(
            "Network name must start and end with alphanumeric characters"
        ));
    }

    // Can only contain alphanumeric characters and hyphens
    for char in name.chars() {
        if !char.is_ascii_alphanumeric() && char != '-' {
            return Err(anyhow::anyhow!(
                "Network name can only contain alphanumeric characters and hyphens"
            ));
        }
    }

    Ok(())
}

/// Validates application name within a network context
pub fn validate_app_name(name: &str) -> Result<()> {
    // Application names follow similar rules to network names
    validate_network_name(name)
}

/// Validates environment variable name
pub fn validate_env_var_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Environment variable name cannot be empty"));
    }

    // Environment variables should be uppercase with underscores
    for char in name.chars() {
        if !char.is_ascii_uppercase() && !char.is_ascii_digit() && char != '_' {
            return Err(anyhow::anyhow!(
                "Environment variable names should contain only uppercase letters, digits, and underscores"
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_network_name() {
        assert!(validate_network_name("my-network").is_ok());
        assert!(validate_network_name("network123").is_ok());
        assert!(validate_network_name("a").is_ok());

        assert!(validate_network_name("").is_err());
        assert!(validate_network_name("-network").is_err());
        assert!(validate_network_name("network-").is_err());
        assert!(validate_network_name("my_network").is_err());
        assert!(validate_network_name(&"a".repeat(64)).is_err());
    }

    #[test]
    fn test_validate_env_var_name() {
        assert!(validate_env_var_name("MY_VAR").is_ok());
        assert!(validate_env_var_name("API_KEY_123").is_ok());

        assert!(validate_env_var_name("").is_err());
        assert!(validate_env_var_name("my_var").is_err());
        assert!(validate_env_var_name("MY-VAR").is_err());
    }
}
