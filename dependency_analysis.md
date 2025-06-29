# Zaeon Project Dependency Analysis

## Overview
This analysis examines the current module dependencies in the lapce-app crate to identify coupling hotspots, circular dependencies, and areas requiring refactoring for better separation of concerns.

## Key Findings

### 1. Monolithic Files with Excessive Dependencies

#### app.rs (167KB, 44 imports)
**Status: CRITICAL - Massive coupling hotspot**
```rust
use crate::{
    about, alert,
    code_action::CodeActionStatus,
    command::{CommandKind, InternalCommand, LapceCommand, LapceWorkbenchCommand, WindowCommand},
    config::{LapceConfig, color::LapceColor, icon::LapceIcons, ui::TabSeparatorHeight, watcher::ConfigWatcher},
    db::LapceDb,
    debug::RunDebugMode,
    editor::{diff::diff_show_more_section_view, location::{EditorLocation, EditorPosition}, view::editor_container_view},
    editor_tab::{EditorTabChild, EditorTabData},
    // ... 30+ more imports
};
```
**Issues:**
- Imports from ALL major modules (UI, business logic, data, config)
- Violates single responsibility principle
- Central coordination point creating tight coupling

#### window_tab.rs (113KB, 40+ imports)
**Status: CRITICAL - Secondary coupling hotspot**
```rust
use crate::{
    about::AboutData, alert::{AlertBoxData, AlertButton},
    code_action::{CodeActionData, CodeActionStatus},
    command::{CommandExecuted, CommandKind, InternalCommand, LapceCommand, LapceWorkbenchCommand, WindowCommand},
    completion::{CompletionData, CompletionStatus},
    config::LapceConfig, db::LapceDb,
    debug::{DapData, LapceBreakpoint, RunDebugMode, RunDebugProcess},
    // ... 30+ more imports
};
```
**Issues:**
- Similar pattern to app.rs - imports everything
- Mixes UI state management with business logic
- Another central coordination point

#### editor.rs (147KB, 20+ imports)
**Status: HIGH - UI/Business logic coupling**
```rust
use crate::{
    command::{CommandKind, InternalCommand, LapceCommand, LapceWorkbenchCommand},
    completion::CompletionStatus, config::LapceConfig, db::LapceDb,
    doc::{Doc, DocContent}, editor_tab::EditorTabChild,
    // ... more mixed concerns
};
```

### 2. Dependency Categories Analysis

#### Configuration Dependencies (HIGH coupling)
Files importing `config::LapceConfig`:
- app.rs, editor.rs, window_tab.rs, completion.rs, inline_completion.rs
- markdown.rs, text_area.rs, editor/gutter.rs
- **Issue:** Configuration is accessed directly throughout the codebase

#### Database Dependencies (MEDIUM coupling)
Files importing `db::LapceDb`:
- app.rs, editor.rs, window_tab.rs, debug.rs, doc.rs
- **Issue:** Direct database access from UI and business logic layers

#### Command System Dependencies (HIGH coupling)
Files importing command types:
- app.rs, window_tab.rs, editor.rs, code_lens.rs, web_link.rs
- command.rs, keypress.rs, palette/kind.rs
- **Issue:** Command handling spread across multiple layers

### 3. Circular Dependency Risks

#### Identified Potential Cycles:
1. **app.rs ↔ window_tab.rs ↔ editor.rs**
   - All three import from each other's modules
   - Central coordination creates circular references

2. **editor.rs ↔ doc.rs ↔ completion.rs**
   - Editor depends on doc and completion
   - Completion depends on editor data
   - Doc may reference editor state

3. **panel/* ↔ window_tab.rs ↔ main_split.rs**
   - Panel views depend on window tab data
   - Window tab manages panel state
   - Main split coordinates both

### 4. Module Responsibility Violations

#### Mixed Concerns in Single Files:
- **app.rs**: UI rendering + state management + event handling + coordination
- **window_tab.rs**: Tab management + global state + event routing + UI coordination
- **editor.rs**: Text editing + UI rendering + command handling + completion management

#### Scattered Features:
- **Plugin system**: Split between plugin.rs and proxy/
- **Terminal**: Spread across terminal/, panel/terminal_view.rs
- **Debug**: debug.rs + panel/debug_view.rs + window_tab.rs integration

### 5. Coupling Hotspots by Import Count

| File | Import Count | Coupling Level | Priority |
|------|-------------|----------------|----------|
| app.rs | 44+ | CRITICAL | 1 |
| window_tab.rs | 40+ | CRITICAL | 2 |
| editor.rs | 20+ | HIGH | 3 |
| main_split.rs | 15+ | HIGH | 4 |
| panel/view.rs | 15+ | HIGH | 5 |

## Recommendations for Refactoring

### Phase 1: Foundation (Steps 1-2)
1. **Create modular directory structure** first
2. **Establish clear interfaces** between layers

### Phase 2: Layer Separation (Steps 3-6)
3. **Extract configuration management** - Create ConfigManager trait
4. **Isolate data access** - Implement repository patterns
5. **Separate UI components** - Pure UI with event-driven communication
6. **Extract business logic** - Service layer with dependency injection

### Phase 3: Complex Refactoring (Steps 7-8)
7. **Unify plugin system** - Coordinate app/proxy plugin architecture
8. **Break down app.rs** - Extract state management, event handling, lifecycle

### Dependency Rules for New Architecture

#### Allowed Dependencies:
- **UI Layer** → Core Layer (via interfaces)
- **Core Layer** → Data Layer (via repositories)
- **All Layers** → Config Layer (via manager)
- **All Layers** → Utils Layer

#### Forbidden Dependencies:
- **Data Layer** → UI Layer
- **Data Layer** → Core Layer
- **Config Layer** → Any other layer
- **Utils Layer** → Any other layer

## Critical Success Factors

1. **Break circular dependencies** before moving files
2. **Create interfaces first** before extracting implementations
3. **Maintain functionality** throughout the refactoring process
4. **Test each step** to ensure no regressions
5. **Update imports incrementally** to avoid compilation failures

## Files Requiring Immediate Attention

### Must Refactor (Blocking other work):
- `app.rs` - Central coordination point
- `window_tab.rs` - Secondary coordination point
- `config.rs` + `settings.rs` - Configuration access

### Should Refactor (High impact):
- `editor.rs` - Large UI component with mixed concerns
- `main_split.rs` - Layout management with business logic
- `doc.rs` - Document management with UI coupling

### Can Refactor Later (Lower risk):
- Panel views - Already somewhat modular
- Terminal components - Contained within terminal/
- Utility modules - Generally single-purpose

This analysis provides the foundation for the systematic refactoring outlined in the 10-step plan.

