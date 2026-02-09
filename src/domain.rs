/// Normalize a domain to Pi-hole-compatible form
pub fn normalize_domain(domain: &str) -> Option<Box<str>> {
    let d = domain
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();

    if d.is_empty() || !d.contains('.') {
        return None;
    }

    Some(d.into_boxed_str())
}

/// Parse a single line using Pi-hole gravity rules
pub fn parse_gravity_line(line: &str) -> Option<Box<str>> {
    let mut s = line.trim();

    // Skip empty lines and full-line comments
    if s.is_empty() || s.starts_with('#') {
        return None;
    }

    // Strip inline comments
    if let Some(idx) = s.find('#') {
        s = &s[..idx];
    }

    let parts: Vec<&str> = s.split_whitespace().collect();

    let domain = match parts.as_slice() {
        // example.com
        [d] => *d,

        // 0.0.0.0 example.com
        [_ip, d] => *d,

        _ => return None,
    };

    // Reject unsupported syntax
    if domain.contains('*')
        || domain.contains('/')
        || domain.contains('|')
    {
        return None;
    }

    normalize_domain(domain)
}
