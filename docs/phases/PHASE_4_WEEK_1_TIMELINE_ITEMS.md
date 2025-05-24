# Phase 4 Week 1: Timeline Item Manipulation - PLANNING 🎬

## 🎯 Overview
**Goal**: Implement 8 Timeline Item Manipulation tools to provide comprehensive control over timeline items including transforms, cropping, composite modes, and retiming.

**Target**: Expand from 28 to **36 tools** with professional-grade timeline item control.

## 🎬 Planned Timeline Item Manipulation Tools

### High Priority: Transform Properties (4 tools)
1. **`set_timeline_item_transform`** 
   - Pan, Tilt, ZoomX, ZoomY controls
   - Rotation and anchor point manipulation
   - Pitch and Yaw for 3D transforms
   - Precise numeric control

2. **`set_timeline_item_crop`**
   - Left, Right, Top, Bottom crop controls
   - Normalized crop values (0.0 to 1.0)
   - Per-item crop settings
   - Crop preservation during edits

3. **`set_timeline_item_composite`**
   - Composite mode selection (Normal, Add, Multiply, etc.)
   - Opacity control (0.0 to 1.0)
   - Blend mode optimization
   - Layer composition control

4. **`set_timeline_item_retime`**
   - Speed factor control (0.1x to 10x)
   - Retime process selection (NearestFrame, FrameBlend, OpticalFlow)
   - Frame rate conversion
   - Time remapping

### Medium Priority: Stabilization & Audio (2 tools)
5. **`set_timeline_item_stabilization`**
   - Enable/disable stabilization
   - Stabilization method (Perspective, Similarity, Translation)
   - Strength control (0.0 to 1.0)
   - Motion analysis parameters

6. **`set_timeline_item_audio`**
   - Volume level control
   - Pan control (-1.0 to 1.0)
   - EQ enable/disable
   - Audio property management

### Low Priority: Advanced Features (2 tools)
7. **`get_timeline_item_properties`**
   - Retrieve all timeline item properties
   - Property inspection and debugging
   - Current state reporting
   - Batch property reading

8. **`reset_timeline_item_properties`**
   - Reset to default values
   - Selective property reset
   - Batch reset operations
   - Property restoration

## 🏗️ Architecture Extensions

### Timeline Item State Management
```rust
#[derive(Debug, Clone, Default)]
struct TimelineItemState {
    /// Unique timeline item ID
    id: String,
    /// Transform properties
    transform: TransformProperties,
    /// Crop settings
    crop: CropProperties,
    /// Composite settings
    composite: CompositeProperties,
    /// Retiming settings
    retime: RetimeProperties,
    /// Stabilization settings
    stabilization: StabilizationProperties,
    /// Audio properties
    audio: AudioProperties,
}

#[derive(Debug, Clone, Default)]
struct TransformProperties {
    pan: f64,
    tilt: f64,
    zoom_x: f64,
    zoom_y: f64,
    rotation: f64,
    anchor_point_x: f64,
    anchor_point_y: f64,
    pitch: f64,
    yaw: f64,
}

#[derive(Debug, Clone, Default)]
struct CropProperties {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

#[derive(Debug, Clone, Default)]
struct CompositeProperties {
    mode: String,     // "Normal", "Add", "Multiply", etc.
    opacity: f64,     // 0.0 to 1.0
}

#[derive(Debug, Clone, Default)]
struct RetimeProperties {
    speed: f64,       // Speed factor
    process: String,  // "NearestFrame", "FrameBlend", "OpticalFlow"
}

#[derive(Debug, Clone, Default)]
struct StabilizationProperties {
    enabled: bool,
    method: String,   // "Perspective", "Similarity", "Translation"
    strength: f64,    // 0.0 to 1.0
}

#[derive(Debug, Clone, Default)]
struct AudioProperties {
    volume: f64,      // Volume level
    pan: f64,         // -1.0 to 1.0
    eq_enabled: bool,
}
```

### Bridge Extensions Required
- Add timeline item state tracking to `ResolveState`
- Implement timeline item lookup by ID
- Add timeline item property manipulation methods
- Integrate with existing timeline management

### Tool Request Types
- Create 8 new request structures in `tools/mod.rs`
- Add JSON schema validation for all parameters
- Implement parameter validation and defaults
- Add comprehensive error handling

### MCP Server Registration
- Register 8 new tools in `server.rs`
- Add detailed JSON schemas for each tool
- Implement tool routing in `handle_tool_call`
- Update tool count and server description

## 🎯 Implementation Plan

### Week 1 Goals
1. **Day 1-2**: Architecture design and timeline item state management
2. **Day 3-4**: Implement transform and crop tools (high priority)
3. **Day 5-6**: Implement composite and retime tools
4. **Day 7**: Testing, documentation, and validation

### Success Criteria
- ✅ All 8 timeline item tools implemented
- ✅ Comprehensive parameter validation
- ✅ Complete test coverage (8 new integration tests)
- ✅ JSON schema compliance
- ✅ Documentation updates
- ✅ Performance validation

## 🧪 Testing Strategy

### Integration Tests (8 new tests)
- `test_timeline_item_transform` - Transform property setting
- `test_timeline_item_crop` - Crop property setting
- `test_timeline_item_composite` - Composite mode/opacity
- `test_timeline_item_retime` - Speed and retime process
- `test_timeline_item_stabilization` - Stabilization settings
- `test_timeline_item_audio` - Audio property control
- `test_timeline_item_properties` - Property retrieval
- `test_timeline_item_reset` - Property reset operations

### Unit Tests
- Parameter validation tests
- Timeline item lookup tests
- Property calculation tests
- Error handling validation

## 📊 Expected Results

### Timeline Item Control Features
- ✅ Complete transform control (pan, tilt, zoom, rotation)
- ✅ Precise crop settings with normalized values
- ✅ Professional composite modes and opacity
- ✅ Advanced retiming with multiple processes
- ✅ Stabilization with configurable methods
- ✅ Audio property management
- ✅ Property inspection and debugging
- ✅ Reset and restoration capabilities

### Performance Targets
- **Property Updates**: Sub-millisecond response
- **Timeline Item Lookup**: O(1) hash map access
- **Batch Operations**: Efficient multi-item processing
- **Memory Usage**: Minimal per-item overhead

### Integration Points
- **Timeline Management**: Seamless integration with existing timeline tools
- **Media Pool**: Compatible with clip timeline placement
- **Color Operations**: Interoperable with color grading workflow
- **MCP Protocol**: Full schema compliance and type safety

## 🚀 Next Steps

1. **Immediate**: Start architecture design and state management
2. **Week 1**: Implement all 8 timeline item manipulation tools
3. **Week 2**: Phase 4 Week 2 - Keyframe Animation (6 tools)
4. **Week 3**: Phase 4 Week 3 - Advanced Rendering (4 tools)

## 🎉 Phase 4 Week 1 Vision

**Timeline Item Manipulation** will provide professional-grade control over every aspect of timeline items, from basic transforms to advanced retiming and stabilization. This forms the foundation for advanced animation and effects workflows in DaVinci Resolve.

**Status**: 📋 PLANNING → 🚧 READY TO BEGIN
**Target Tools**: 8 (28 → 36)
**Architecture**: Extended timeline item state management
**Integration**: Full MCP protocol compliance 