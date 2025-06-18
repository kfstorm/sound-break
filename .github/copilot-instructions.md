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
- Auto-detect Feishu Meeting (飞书) application state
- Use macOS native music control APIs (not app-specific detection)
- Maintain extensibility for other meeting applications
- System tray integration
- Minimal performance impact

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
- Process monitoring for meeting detection
- System notifications for user feedback

## macOS Specific Considerations
- Requires accessibility permissions for process monitoring
- Uses AppleScript for music control
- System tray integration with native look and feel
- Respect macOS design guidelines
