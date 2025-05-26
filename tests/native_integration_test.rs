#[cfg(test)]
mod tests {
    use davinci_mcp_rs::native::test_native_integration;

    #[test]
    #[ignore] // Игнорируем по умолчанию, так как требует нативные библиотеки DaVinci Resolve
    fn test_native_davinci_resolve_integration() {
        println!("🧪 Native DaVinci Resolve Integration Test");
        println!("============================================");

        // Test native integration
        match test_native_integration() {
            Ok(()) => {
                println!("✅ Native integration test completed successfully!");
            }
            Err(e) => {
                println!("❌ Native integration test failed: {}", e);
                panic!("Native integration test failed: {}", e);
            }
        }
    }
}
