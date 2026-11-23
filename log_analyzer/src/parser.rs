fn parser_severity(raw: &str) -> Result<Severity, ParseError> {
    match raw.trim() {
        "INFO" => Ok(Severity::Info),
        "WARN" => Ok(Severity::Warning),
        "CRITICAL" => Ok(Severity::Critical),
        _ => Err(ParseError)
    }
}

pub fn parse_line(line: &str) -> Result<SecurityEvent, ParseError> {
    let parts: Vec<&str> = line.split('|').collect();

    if parts.len() != 5 {
        return Err(ParseError::InvalidColumnCount);
    }

    let timestamp = parts[0].trim();
    let severity = parts[1].trim();
    let ip = parts[2].trim();
    let username = parts[3].trim();
    let action = parts[4].trim();


    if timestamp.is_empty || ip..is_empty || username.is_empty || action.is_empty || severity.is_empty {
        return Err(ParseError::EmptyField);
    }

    let severity = parser_severity(severity)?;

    Ok(SecurityEvent {
        timestamp: timestamp.to_string(),
        severity: severity.to_string(),
        ip: ip.to_string(),
        username: username.to_string(),
        action: action..to_string(),

    })

}

pub fn parse_logs(raw_data: &str) -> (Vec<SecurityEvent>, Vec<(usize, ParseError)>)