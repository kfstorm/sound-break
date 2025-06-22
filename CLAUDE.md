# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SoundBreak is a Tauri + SvelteKit desktop application for macOS that automatically pauses and resumes music when detecting meeting applications. It runs as a system tray application with real-time monitoring capabilities.

## Key Commands

### Development
- `pnpm install` - Install dependencies
- `pnpm tauri dev` - Start development mode (launches both frontend and backend)
- `pnpm tauri build` - Build production application
- `pnpm dev` - Start frontend only (Vite dev server)
- `pnpm build` - Build frontend only
- `pnpm check` - Run Svelte type checking
- `pnpm check:watch` - Run Svelte type checking in watch mode

### Prerequisites for Development
- Node.js (v18+) and pnpm package manager
- Rust toolchain and Tauri CLI
- macOS development environment

## Architecture

### Technology Stack
- **Backend**: Rust (Tauri v2) with native macOS APIs
- **Frontend**: SvelteKit + TypeScript + Vite
- **Target Platform**: macOS (with cross-platform architecture)
- **Key Dependencies**: tauri-plugin-autostart, MediaRemote framework

### Core Components (Rust backend)

- **`lib.rs`**: Main application entry point with Tauri commands and system tray setup
- **`monitoring_service.rs`**: Central monitoring loop and state management
- **`meeting_detector.rs`**: Process-based meeting detection with exact name matching
- **`music_controller.rs`**: Universal music control via MediaRemote framework
- **`config.rs`**: Configuration management with persistent storage

### Frontend Structure
- **`src/routes/+page.svelte`**: Main UI with status display and controls
- **`src/lib/SettingsModal.svelte`**: Settings modal for meeting app configuration
- **`src/routes/+layout.ts`**: SvelteKit layout configuration
- **`src/app.html`**: HTML template

### Detection Architecture
- **Meeting Detection**: Uses `pgrep` with exact process name matching (^pattern$)
  - Real-time process monitoring with 2-second polling
  - Configurable process names (default: "Lark Helper (Iron)" for Feishu)
  - Case-sensitive exact matching for reliability
- **Music Detection**: Uses macOS MediaRemote framework via AppleScript
  - Universal compatibility with any music player
  - System-level music control integration

### State Management
- Mutex-protected shared state across background threads
- Real-time status updates to system tray menu
- Persistent monitoring service with configurable settings

## Key Design Principles

### Universal Compatibility
- **Never hard-code specific applications**: All app detection must be configurable
- **Use system-level APIs**: Prefer native OS APIs over app-specific solutions
- **Universal music control**: Support any music player through system media keys
- **Configurable process lists**: Meeting app detection through user-configurable process names

### System Integration
- **System tray only**: No persistent window, runs as background service
- **Auto-start functionality**: Optional launch on login with system integration
- **Accessibility permissions**: Required for process monitoring and music control
- **macOS native behavior**: Respect macOS design guidelines and patterns

### Performance Considerations
- **Minimal system impact**: 2-second polling intervals for real-time monitoring
- **Efficient updates**: Only update tray menu when status actually changes
- **Background operation**: Main logic runs in Rust backend for performance

## Configuration Files

- **`tauri.conf.json`**: Tauri application configuration
- **`Cargo.toml`**: Rust dependencies and build configuration
- **`package.json`**: Node.js dependencies and scripts
- **`svelte.config.js`**: SvelteKit adapter configuration for static builds

## Important Notes

- Uses `tauri-plugin-autostart` for login integration
- Requires macOS accessibility permissions for process monitoring
- **Configuration persistence**: Meeting app settings are automatically saved to `~/Library/Application Support/com.kfstorm.sound-break/config.json`
- **Window behavior**: Main window shows automatically in development mode (`pnpm tauri dev`) but stays hidden in production builds
- **Production UX**: "Show Settings" tray menu item automatically opens settings modal in production
- No tests are currently implemented in the codebase
- Application hides from Dock using `ActivationPolicy::Accessory`
- Frontend-backend communication uses Tauri's invoke API
- Window close events are intercepted to hide rather than quit the application