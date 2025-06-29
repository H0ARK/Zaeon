# Zaeon Modular Architecture

This document describes the new modular architecture for Zaeon, designed to improve separation of concerns and reduce tight coupling.

## 🏗️ Architecture Overview

The codebase is organized into distinct layers with clear responsibilities:

```
lapce-app/src/
├── ui/                 # UI Layer - Pure UI components
├── core/               # Business Logic Layer  
├── data/               # Data Access Layer
├── config/             # Configuration Management
├── plugins/            # Plugin System
└── utils/              # Shared Utilities
```

## 📁 Directory Structure

### UI Layer (`ui/`)
Pure UI components with event-driven communication, separated from business logic.

```
ui/
├── components/         # Reusable UI building blocks
├── windows/           # Window and tab management UI
├── panels/            # Panel UI components (debug, terminal, etc.)
├── editor/            # Editor UI components
└── common/            # Shared UI utilities and event system
    ├── ui_events.rs   # Event-driven communication system
    └── ui_state.rs    # UI-specific state management
```

**Key Principles:**
- No direct business logic dependencies
- Event-driven communication with core services
- Dependency injection for external dependencies
- Pure rendering and user interaction handling

### Core Layer (`core/`)
Business logic services, interfaces, and domain models.

```
core/
├── services/          # Business logic services
│   ├── command_service.rs     # Unified command handling
│   ├── completion_service.rs  # Text completion logic
│   ├── debug_service.rs       # Debug functionality
│   ├── search_service.rs      # Search operations
│   └── editor_service.rs      # Core text editing
├── interfaces/        # Service interfaces for DI
│   ├── command_handler.rs
│   ├── completion_provider.rs
│   ├── debug_manager.rs
│   ├── search_provider.rs
│   └── editor_manager.rs
└── models/           # Domain models and data structures
```

**Key Principles:**
- Independent of UI and data access implementations
- Interface-based design for loose coupling
- Single responsibility principle
- Dependency injection for external dependencies

### Data Layer (`data/`)
Data access and persistence logic.

```
data/
└── repositories/     # Repository pattern implementations
    ├── DocumentRepository    # Document storage
    ├── ConfigRepository      # Configuration storage
    ├── PluginRepository      # Plugin data
    └── WorkspaceRepository   # Workspace data
```

**Key Principles:**
- Repository pattern for data access abstraction
- Separated from business logic
- Interface-based for testability

### Configuration Layer (`config/`)
Centralized configuration management.

```
config/
└── management/
    ├── config_manager.rs    # Application configuration
    └── settings_manager.rs  # User preferences
```

**Key Principles:**
- Centralized configuration handling
- Extracted from scattered config logic
- Type-safe configuration access

### Plugin System (`plugins/`)
Consolidated plugin functionality.

```
plugins/
├── core/             # Core plugin system
├── app/              # App-plugin integration
├── proxy/            # Plugin proxy system
└── communication/    # Plugin communication protocols
```

**Key Principles:**
- Modular plugin architecture
- Clear separation of plugin concerns
- Standardized communication protocols

### Utilities (`utils/`)
Shared utilities and helper functions.

```
utils/
└── helpers/
    ├── string_utils.rs    # String manipulation
    ├── file_utils.rs      # File and path utilities
    └── common_utils.rs    # Miscellaneous utilities
```

**Key Principles:**
- Pure functions without side effects
- No business logic dependencies
- Reusable across all layers

## 🔄 Communication Patterns

### Event-Driven Communication
- UI components emit events instead of calling services directly
- Services handle events and update state
- Loose coupling between UI and business logic

### Dependency Injection
- Services depend on interfaces, not concrete implementations
- Enables testing and modularity
- Clear dependency boundaries

### Repository Pattern
- Data access abstracted through repositories
- Business logic independent of data storage
- Testable data access layer

## 📊 Migration Benefits

### Before Refactoring:
- **app.rs**: 167KB monolithic file with 44+ imports
- **editor.rs**: 147KB with mixed UI and business logic
- **window_tab.rs**: 113KB with tight coupling
- Circular dependencies between major components
- Scattered configuration and plugin logic

### After Refactoring:
- ✅ Modular architecture with clear boundaries
- ✅ Separation of concerns (UI/Business/Data)
- ✅ Event-driven communication
- ✅ Dependency injection for loose coupling
- ✅ Eliminated circular dependencies
- ✅ Centralized configuration management
- ✅ Consolidated plugin system

## 🎯 Next Steps

1. **Step 3**: Refactor Configuration Management
2. **Step 4**: Reorganize Data Management Layer
3. **Step 5**: Extract UI Components
4. **Step 6**: Create Business Logic Layer
5. **Step 7**: Consolidate Plugin System
6. **Step 8**: Refactor Monolithic app.rs
7. **Step 9**: Update Module Declarations
8. **Step 10**: Create Documentation

## 🔧 Development Guidelines

### Adding New Features
1. Identify the appropriate layer (UI/Core/Data)
2. Use dependency injection for external dependencies
3. Follow event-driven communication patterns
4. Write interfaces before implementations
5. Keep layers independent and testable

### Modifying Existing Code
1. Respect layer boundaries
2. Use events for cross-layer communication
3. Extract business logic from UI components
4. Move data access to repositories
5. Centralize configuration management

This architecture provides a solid foundation for maintainable, testable, and scalable code development.

