# SoundBreak - Copilot Instructions

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

## Project Overview
SoundBreak is a Tauri + Svelte desktop application for macOS that automatically pauses and resumes music when detecting meeting applications. The app runs as a system tray application with real-time monitoring capabilities.

## Technical Stack
- **Backend**: Rust (Tauri v2) with native macOS APIs
- **Frontend**: SvelteKit + TypeScript + Vite
- **Target Platform**: macOS (with cross-platform architecture)
- **Build Tool**: Tauri CLI with pnpm
- **Key Dependencies**: tauri-plugin-autostart, MediaRemote framework

## Current Implementation Features
- **Real-time monitoring service** with configurable process detection
- **System tray integration** with live status updates and menu controls
- **Universal music control** via macOS MediaRemote framework (any music player)
- **Configurable meeting app detection** through exact process name matching
- **Auto-start functionality** with login integration
- **Background monitoring** with minimal system impact
- **Process-based detection** using `pgrep` for reliable real-time results

## Architecture Implementation
- **Modular Rust backend** with separate services:
  - `MonitoringService`: Core monitoring loop and state management
  - `MeetingDetector`: Process-based meeting detection with exact name matching
  - `MusicController`: Universal music control via MediaRemote framework
- **Tauri command interface** for frontend-backend communication
- **System tray integration** with real-time status updates and menu controls
- **Background thread monitoring** with 2-second polling intervals
- **State management** with Mutex-protected shared state across threads

## Detection Methods
- **Meeting Detection**: Uses `pgrep` with exact process name matching (^pattern$)
  - Real-time process monitoring without caching issues
  - Configurable process names (default: "Lark Helper (Iron)" for Feishu)
  - Case-sensitive exact matching for reliability
- **Music Detection**: Uses macOS MediaRemote framework via AppleScript
  - Universal compatibility with any music player
  - Detects playback state through system-level APIs
  - No dependency on specific music applications

## User Interface
- **System tray only**: No persistent window, runs as background service
- **Tray menu features**:
  - Real-time status indicators (monitoring, music, meeting states)
  - Toggle monitoring on/off
  - Auto-start on login control
  - Show settings window option
- **Settings window**: Configurable meeting app process names
- **Window behavior**: Hides to tray instead of closing

## Code Style Guidelines
- Use TypeScript for frontend code
- Follow Rust best practices for backend
- Prefer async/await patterns
- Use Tauri's invoke API for frontend-backend communication
- Keep UI minimal and functional

## Architecture Notes
- Main logic should be in Rust backend for performance
- Use Tauri commands for cross-language communication
- Leverage macOS native APIs for music control
- Use sysinfo crate for cross-platform process monitoring (no shell commands)
- System notifications for user feedback
- **No hard-coding**: Avoid hardcoding specific app names or process names
- **Universal detection**: Use system-level APIs that work with any music player
- **Configurable behavior**: Make app detection configurable through user settings

## macOS Specific Considerations
- Requires accessibility permissions for process monitoring
- Uses MediaRemote framework for universal music control
- System tray integration with native look and feel
- Respect macOS design guidelines
- **Universal compatibility**: No hard-coded music players or meeting apps
- **Configurable detection**: Process names should be user-configurable
- **System-level APIs**: Prefer native system APIs over app-specific solutions

## Generalization Principles
- **Never hard-code specific applications**: All app detection must be configurable
- **Use system-level APIs**: Prefer native OS APIs over app-specific solutions
- **Universal music control**: Support any music player through system media keys
- **Configurable process lists**: Meeting app detection through user-configurable process names
- **Cross-platform ready**: Use libraries like `sysinfo` for platform-agnostic code
- **Future-proof design**: Code should work with new apps without modification
