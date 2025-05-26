#[cfg(test)]
mod tests {
    use std::process::Command;

    #[tokio::test]
    async fn test_native_python_integration_research() {
        println!("🔬 Исследование Native Python интеграции");
        println!("{}", "=".repeat(60));

        // Тест 1: Проверяем Python C API возможности
        println!("\n🐍 Test 1: Python C API capabilities");
        let python_script = r#"
import sys
import ctypes
import os

print(f"Python version: {sys.version}")
print(f"Python executable: {sys.executable}")

# Check if we can access Python C API
try:
    import ctypes.util
    python_lib = ctypes.util.find_library("python3.12")
    if python_lib:
        print(f"Python library found: {python_lib}")
    else:
        print("Python library not found via ctypes")
    
    # Check Python include directory
    import sysconfig
    include_dir = sysconfig.get_path('include')
    print(f"Python include directory: {include_dir}")
    
    # Check if Python.h exists
    python_h = os.path.join(include_dir, "Python.h")
    if os.path.exists(python_h):
        print(f"✅ Python.h found: {python_h}")
    else:
        print(f"❌ Python.h not found: {python_h}")
        
except Exception as e:
    print(f"Error checking Python C API: {e}")

# Test fusionscript loading
print("\n🎬 Testing fusionscript module loading")
try:
    sys.path.append("/opt/resolve/Developer/Scripting/Modules")
    import DaVinciResolveScript as dvr_script
    print("✅ DaVinciResolveScript imported successfully")
    
    # Check module attributes
    print(f"Module file: {dvr_script.__file__}")
    print(f"Module attributes: {dir(dvr_script)}")
    
    # Try to get the underlying C extension
    if hasattr(dvr_script, 'scriptapp'):
        print("✅ scriptapp function found")
        print(f"scriptapp type: {type(dvr_script.scriptapp)}")
    
except ImportError as e:
    print(f"❌ Cannot import DaVinciResolveScript: {e}")
except Exception as e:
    print(f"❌ Error: {e}")
"#;

        let output = Command::new("python3")
            .arg("-c")
            .arg(python_script)
            .output()
            .expect("Failed to execute Python script");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("📤 Python output:");
        println!("{}", stdout);

        if !stderr.is_empty() {
            println!("⚠️ Python errors:");
            println!("{}", stderr);
        }

        // Тест 2: Проверяем возможность создания Python embedding
        println!("\n🦀 Test 2: Rust Python embedding possibilities");

        // Check if we have pyo3 or similar crates available
        println!("💡 For native integration we would need:");
        println!("   1. pyo3 crate for Python C API bindings");
        println!("   2. Embedded Python interpreter in Rust");
        println!("   3. Direct access to fusionscript module");
        println!("   4. Object marshalling between Rust and Python");

        println!("\n🔧 Current approach comparison:");
        println!("   📊 Python subprocess: ✅ Working, ❌ Slower");
        println!("   🚀 Native embedding: ❓ Complex, ✅ Faster");
        println!("   🔗 Hybrid approach: ✅ Best of both");
    }

    #[tokio::test]
    async fn test_performance_comparison() {
        println!("⚡ Performance comparison: Subprocess vs Native");
        println!("{}", "=".repeat(60));

        let iterations = 5;

        // Test subprocess approach
        println!(
            "\n🐍 Testing subprocess approach ({} iterations)",
            iterations
        );
        let start = std::time::Instant::now();

        for i in 1..=iterations {
            let python_script = r#"
import sys
sys.path.append("/opt/resolve/Developer/Scripting/Modules")

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if resolve:
        project_manager = resolve.GetProjectManager()
        if project_manager:
            print("SUCCESS")
        else:
            print("FAILED")
    else:
        print("NO_RESOLVE")
except Exception as e:
    print(f"ERROR: {e}")
"#;

            let output = Command::new("python3")
                .arg("-c")
                .arg(python_script)
                .output()
                .expect("Failed to execute Python script");

            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("  Iteration {}: {}", i, stdout.trim());
        }

        let subprocess_duration = start.elapsed();
        println!("📊 Subprocess total time: {:?}", subprocess_duration);
        println!(
            "📊 Average per call: {:?}",
            subprocess_duration / iterations
        );

        // Simulate native approach timing
        println!("\n🚀 Simulated native approach timing");
        let start = std::time::Instant::now();

        for i in 1..=iterations {
            // Simulate native call overhead (much faster)
            std::thread::sleep(std::time::Duration::from_millis(1));
            println!("  Iteration {}: NATIVE_SUCCESS", i);
        }

        let native_duration = start.elapsed();
        println!("📊 Native total time: {:?}", native_duration);
        println!("📊 Average per call: {:?}", native_duration / iterations);

        let speedup = subprocess_duration.as_millis() as f64 / native_duration.as_millis() as f64;
        println!("🚀 Potential speedup: {:.1}x faster", speedup);
    }

    #[tokio::test]
    async fn test_native_integration_roadmap() {
        println!("🗺️ Native Integration Roadmap");
        println!("{}", "=".repeat(60));

        println!("\n📋 Phase 1: Research & Setup");
        println!("   ✅ Library loading (fusionscript.so)");
        println!("   ✅ Symbol analysis (PyInit_fusionscript)");
        println!("   ❓ Python C API integration");
        println!("   ❓ pyo3 crate integration");

        println!("\n📋 Phase 2: Basic Integration");
        println!("   ❓ Embed Python interpreter");
        println!("   ❓ Load fusionscript module");
        println!("   ❓ Call scriptapp() function");
        println!("   ❓ Handle Python objects in Rust");

        println!("\n📋 Phase 3: API Mapping");
        println!("   ❓ Resolve object methods");
        println!("   ❓ Project manager methods");
        println!("   ❓ Timeline methods");
        println!("   ❓ Error handling");

        println!("\n📋 Phase 4: Performance & Safety");
        println!("   ❓ Memory management");
        println!("   ❓ Thread safety");
        println!("   ❓ Error recovery");
        println!("   ❓ Resource cleanup");

        println!("\n💡 Recommendation:");
        println!("   🎯 Current Python subprocess approach is WORKING");
        println!("   🚀 Native integration is POSSIBLE but COMPLEX");
        println!("   ⚖️ Cost/benefit analysis needed");
        println!("   🔧 Hybrid approach might be optimal");

        println!("\n🔧 Hybrid Approach Ideas:");
        println!("   1. 🐍 Keep Python for complex operations");
        println!("   2. 🚀 Use native for simple/frequent calls");
        println!("   3. 📦 Cache Python objects between calls");
        println!("   4. 🔄 Connection pooling");
        println!("   5. ⚡ Async batching of operations");
    }
}
