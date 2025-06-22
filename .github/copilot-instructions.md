# SoundBreak - Copilot Instructions

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

## Project Overview
SoundBreak is a Tauri + Svelte desktop application for macOS that automatically pauses and resumes music when detecting Feishu Meeting entry/exit.

## Technical Stack
- **Backend**: Rust (Tauri)
- **Frontend**: Svelte + TypeScript
- **Target Platform**: macOS (with future cross-platform support)
- **Build Tool**: Tauri CLI with pnpm

## Key Requirements
- Auto-detect meeting applications (configurable process names)
- Use macOS native music control APIs (not app-specific detection)
- Maintain extensibility for other meeting applications
- System tray integration
- Minimal performance impact
- **Universal compatibility**: No hard-coded music players or meeting apps
- **Configurable detection**: Process names should be user-configurable
- **System-level APIs**: Prefer native system APIs over app-specific solutions

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
