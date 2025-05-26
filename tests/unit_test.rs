use davinci_mcp_rs::error::ResolveError;

#[test]
fn test_error_types() {
    let error = ResolveError::ProjectNotFound {
        name: "Test Project".to_string(),
    };
    assert!(matches!(error, ResolveError::ProjectNotFound { .. }));
}

#[test]
fn test_error_display() {
    let error = ResolveError::ProjectNotFound {
        name: "Test Project".to_string(),
    };
    let error_string = format!("{}", error);
    assert!(error_string.contains("Test Project"));
}

#[test]
fn test_api_call_error() {
    let error = ResolveError::api_call("test_method", "Test error message");
    assert!(matches!(error, ResolveError::ApiCall { .. }));
}

#[test]
fn test_invalid_parameter_error() {
    let error = ResolveError::invalid_parameter("test_param", "Invalid value");
    assert!(matches!(error, ResolveError::InvalidParameter { .. }));
}

#[cfg(test)]
mod tools_tests {
    #[test]
    fn test_tool_validation() {
        // Add tests for tool validation logic
        assert!(true); // Placeholder
    }

    #[test]
    fn test_tool_execution() {
        // Add tests for tool execution
        assert!(true); // Placeholder
    }
}
