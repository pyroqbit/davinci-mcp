# Native Integration Plan for DaVinci Resolve MCP Server

## 🎯 Current Status

- ✅ **Python Subprocess Approach**: Working reliably
- ✅ **Library Analysis**: `fusionscript.so` analyzed and accessible
- ✅ **Performance Baseline**: ~307ms per API call via subprocess
- ❓ **Native Integration**: Possible but complex

## 📊 Performance Analysis

### Current Performance (Python Subprocess)
- **Average call time**: 306ms
- **Overhead**: Python process startup, JSON serialization
- **Reliability**: High (isolated processes)
- **Memory usage**: Low (processes cleaned up)

### Potential Native Performance
- **Estimated call time**: ~1ms (306x faster)
- **Overhead**: Python C API calls only
- **Reliability**: Medium (shared memory space)
- **Memory usage**: Higher (persistent Python objects)

## 🗺️ Implementation Roadmap

### Phase 1: Foundation (2-3 weeks)
```toml
# Add to Cargo.toml
[dependencies]
pyo3 = { version = "0.20", features = ["auto-initialize"] }
```

**Goals:**
- [ ] Add `pyo3` dependency
- [ ] Create basic Python interpreter embedding
- [ ] Load `fusionscript` module successfully
- [ ] Call `scriptapp()` function from Rust

**Key challenges:**
- Python GIL management
- Module path configuration
- Error handling between Rust and Python

### Phase 2: Basic API Integration (2-3 weeks)

**Goals:**
- [ ] Implement `Resolve` object wrapper
- [ ] Create `ProjectManager` interface
- [ ] Add basic timeline operations
- [ ] Handle Python exceptions in Rust

**Architecture:**
```rust
pub struct NativeResolve {
    py: Python<'_>,
    resolve_obj: PyObject,
    project_manager: Option<PyObject>,
}

impl NativeResolve {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let fusionscript = py.import("fusionscript")?;
            let resolve_obj = fusionscript.call_method1("scriptapp", ("Resolve",))?;
            Ok(Self {
                py,
                resolve_obj: resolve_obj.to_object(py),
                project_manager: None,
            })
        })
    }
    
    pub fn switch_page(&self, page: &str) -> PyResult<bool> {
        Python::with_gil(|py| {
            let result = self.resolve_obj
                .call_method1(py, "OpenPage", (page,))?;
            result.extract(py)
        })
    }
}
```

### Phase 3: Advanced Features (3-4 weeks)

**Goals:**
- [ ] Color grading operations
- [ ] Timeline item manipulation
- [ ] Keyframe animation
- [ ] Render queue management

**Performance optimizations:**
- [ ] Object caching
- [ ] Batch operations
- [ ] Async Python calls

### Phase 4: Production Ready (2-3 weeks)

**Goals:**
- [ ] Thread safety
- [ ] Memory leak prevention
- [ ] Error recovery
- [ ] Performance monitoring
- [ ] Fallback to subprocess on errors

## 🔧 Hybrid Approach Strategy

Instead of full native replacement, implement **selective native acceleration**:

```rust
pub enum ApiCallStrategy {
    Subprocess,  // For complex/rare operations
    Native,      // For simple/frequent operations
    Cached,      // For repeated queries
}

impl ResolveBridge {
    fn choose_strategy(&self, method: &str) -> ApiCallStrategy {
        match method {
            // Frequent operations -> Native
            "switch_page" | "get_current_timeline" => ApiCallStrategy::Native,
            
            // Complex operations -> Subprocess
            "create_empty_timeline" | "add_to_render_queue" => ApiCallStrategy::Subprocess,
            
            // Cacheable queries -> Cached
            "list_timelines_tool" | "get_project_settings" => ApiCallStrategy::Cached,
            
            _ => ApiCallStrategy::Subprocess,
        }
    }
}
```

## 🚨 Risk Assessment

### High Risk
- **Memory leaks** from Python object references
- **Segmentation faults** from incorrect Python C API usage
- **GIL deadlocks** in multi-threaded scenarios
- **Version compatibility** issues with Python/DaVinci Resolve updates

### Medium Risk
- **Performance regression** if native calls are slower than expected
- **Debugging complexity** with mixed Rust/Python stack traces
- **Maintenance overhead** for two different API approaches

### Low Risk
- **Feature parity** - Python subprocess approach as fallback
- **Testing complexity** - existing test suite covers functionality

## 💡 Decision Framework

**Implement native integration IF:**
- [ ] Performance becomes a bottleneck (>1000 API calls/minute)
- [ ] Real-time operations are required (live editing)
- [ ] Memory usage needs to be minimized
- [ ] Team has Python C API expertise

**Stay with subprocess approach IF:**
- [x] Current performance is acceptable
- [x] Reliability is more important than speed
- [x] Development resources are limited
- [x] Maintenance simplicity is preferred

## 🎯 Current Recommendation

**Continue with Python subprocess approach** because:

1. ✅ **Proven reliability** - works consistently
2. ✅ **Simple maintenance** - easy to debug and modify
3. ✅ **Adequate performance** - 300ms is acceptable for MCP server
4. ✅ **Risk mitigation** - isolated processes prevent crashes
5. ✅ **Development speed** - focus on features, not optimization

**Consider native integration later** when:
- Performance requirements increase significantly
- Real-time features are needed
- Team gains Python C API expertise
- Stable foundation is established

## 📚 Resources

- [PyO3 User Guide](https://pyo3.rs/)
- [Python C API Documentation](https://docs.python.org/3/c-api/)
- [DaVinci Resolve Scripting Documentation](https://documents.blackmagicdesign.com/DaVinciResolve/20241106-c9b5b1/DaVinci_Resolve_API_Developer_Documentation.pdf)
- [Rust FFI Guidelines](https://doc.rust-lang.org/nomicon/ffi.html) 