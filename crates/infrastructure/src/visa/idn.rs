const UNKNOWN: &str = "UNKNOWN";

/// Vendor/model/serial/firmware parsed out of a `*IDN?` response.
///
/// Per IEEE 488.2, a compliant `*IDN?` response has the form:
/// `<vendor>,<model>,<serial number>,<firmware revision>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdnFields {
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub firmware: String,
}

/// Parses a raw `*IDN?` response. Tolerant of missing fields (padded
/// with `"UNKNOWN"`) and of extra commas in the firmware/options
/// field (some instruments append option strings there).
pub fn parse_idn(raw: &str) -> IdnFields {
    let parts: Vec<&str> = raw.trim().split(',').map(str::trim).collect();

    let firmware = if parts.len() > 4 {
        parts[3..].join(",")
    } else {
        field(&parts, 3)
    };

    IdnFields {
        vendor: field(&parts, 0),
        model: field(&parts, 1),
        serial: field(&parts, 2),
        firmware,
    }
}

fn field(parts: &[&str], idx: usize) -> String {
    parts
        .get(idx)
        .copied()
        .filter(|s| !s.is_empty())
        .unwrap_or(UNKNOWN)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_idn_response() {
        let fields = parse_idn("Keysight Technologies,DSOX3024A,MY123456,02.41\n");
        assert_eq!(fields.vendor, "Keysight Technologies");
        assert_eq!(fields.model, "DSOX3024A");
        assert_eq!(fields.serial, "MY123456");
        assert_eq!(fields.firmware, "02.41");
    }

    #[test]
    fn pads_missing_fields_with_unknown() {
        let fields = parse_idn("OnlyVendor");
        assert_eq!(fields.vendor, "OnlyVendor");
        assert_eq!(fields.model, "UNKNOWN");
        assert_eq!(fields.serial, "UNKNOWN");
        assert_eq!(fields.firmware, "UNKNOWN");
    }

    #[test]
    fn joins_trailing_fields_into_firmware() {
        let fields = parse_idn("Acme,Model1,SN1,1.0,OPT01,OPT02");
        assert_eq!(fields.firmware, "1.0,OPT01,OPT02");
    }
}
