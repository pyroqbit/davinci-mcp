#[cfg(test)]
mod tests {
    use libloading::Library;

    #[test]
    fn test_fusion_library_loading() {
        println!("🔧 Testing Fusion library loading...");

        let fusion_path = "/opt/resolve/libs/Fusion/fusionscript.so";

        println!("📁 Attempting to load: {}", fusion_path);

        unsafe {
            match Library::new(fusion_path) {
                Ok(lib) => {
                    println!("✅ Successfully loaded Fusion library!");

                    // Try to get a symbol
                    match lib.get::<fn()>(b"Resolve\0") {
                        Ok(_) => println!("✅ Found Resolve symbol!"),
                        Err(e) => println!("⚠️ Resolve symbol not found: {}", e),
                    }

                    // List some symbols if possible
                    println!("📋 Library loaded successfully");
                }
                Err(e) => {
                    println!("❌ Failed to load Fusion library: {}", e);
                    panic!("Cannot load Fusion library: {}", e);
                }
            }
        }
    }

    #[test]
    fn test_com_api_library_loading() {
        println!("🔧 Testing COM API library loading...");

        let com_api_path = "/opt/resolve/libs/libcom-api.so";

        println!("📁 Attempting to load: {}", com_api_path);

        unsafe {
            match Library::new(com_api_path) {
                Ok(_lib) => {
                    println!("✅ Successfully loaded COM API library!");
                }
                Err(e) => {
                    println!("⚠️ Failed to load COM API library: {}", e);
                    println!("💡 This is expected if the file doesn't exist");
                }
            }
        }
    }

    #[test]
    fn test_library_paths() {
        println!("🔍 Testing library paths...");

        let paths = vec![
            "/opt/resolve/libs/Fusion/fusionscript.so",
            "/opt/resolve/libs/libcom-api.so",
            "/opt/resolve/libs/Fusion/libfusioncontrols.so",
            "/opt/resolve/libs/Fusion/libfusiongraphics.so",
        ];

        for path in paths {
            if std::path::Path::new(path).exists() {
                println!("✅ Found: {}", path);
            } else {
                println!("❌ Missing: {}", path);
            }
        }
    }
}
