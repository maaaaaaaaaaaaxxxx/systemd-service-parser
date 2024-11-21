use systemd_service_parser::*;
// Test cases
#[cfg(test)]
mod tests {
    use super::*;

// 1. Service does not have invalid sections (none of [Unit], [Service], [Install])
    #[ignore]
    #[test]
    fn test_invalid_section() {
        let content = r#"
        [Unit]
        Description=A systemd service

        [Something]
        Foo=bar
        "#;

        let result = parse_service_file(content);
        assert!(result.is_ok());

        let parsed_service = result.unwrap();

        assert_eq!(parsed_service.sections.len(), 1);
        assert_eq!(parsed_service.sections[0].header, "Unit");

        for section in parsed_service.sections {
            assert!(matches!(section.header.as_str(), "Unit" | "Service" | "Install"));
        }
    }

// 2. All units have at least one key-value pair.
    #[test]
    fn test_sections_have_key_value_pairs() {
        let content = r#"
        [Unit]
        Description=A systemd service

        [Service]
        ExecStart=/bin/bash

        [Install]
        WantedBy=multi-user.target
        "#;

        let parsed_service = parse_service_file(content).unwrap();

        for section in parsed_service.sections {
            assert!(!section.key_values.is_empty());
        }
    }

// 3. Service does not have duplicate sections
    #[ignore]
    #[test]
    fn test_no_duplicate_sections() {
        let content = r#"
        [Unit]
        Description=A systemd service

        [Unit]
        Description=A systemd service

        [Service]
        ExecStart=/bin/bash

        [Install]
        WantedBy=multi-user.target
        "#;

        let result = parse_service_file(content);
        assert!(result.is_err());

        let parsed_service = result.unwrap();
        let mut seen_headers = std::collections::HashSet::new();

        for section in parsed_service.sections {
            assert!(seen_headers.insert(section.header));
        }
    }
}
