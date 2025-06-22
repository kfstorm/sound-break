# SoundBreak

SoundBreak is a macOS desktop application that automatically pauses and resumes music when detecting meeting applications. Built with Tauri + SvelteKit, it runs as a lightweight system tray application with real-time monitoring capabilities.

## Features

- 🎵 **Universal Music Control**: Works with any music player through macOS MediaRemote framework
- 🎤 **Smart Meeting Detection**: Configurable process-based detection for any meeting application
- 🔄 **Real-time Monitoring**: 2-second polling for instant response to meeting state changes
- 🚀 **Auto-start Support**: Optional launch on login with system integration
- 🖥️ **System Tray Integration**: Runs in background with live status indicators
- ⚙️ **Configurable Settings**: Customizable meeting app detection through exact process names
- 🔒 **Privacy-focused**: No network connections, all processing happens locally

## How It Works

1. **Meeting Detection**: Uses `pgrep` command to detect running meeting applications by exact process name matching
2. **Music Detection**: Leverages macOS MediaRemote framework to detect any music player's playback state
3. **Automatic Control**: When a meeting starts, automatically pauses music; resumes when meeting ends
4. **Background Operation**: Runs silently in the system tray with minimal resource usage

## Technical Stack

- **Backend**: Rust (Tauri v2) with native macOS APIs
- **Frontend**: SvelteKit + TypeScript + Vite
- **Build Tool**: Tauri CLI with pnpm
- **Key Dependencies**:
  - `tauri-plugin-autostart` for login integration
  - macOS MediaRemote framework for universal music control
  - Native `pgrep` for reliable process detection

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) toolchain
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)
- macOS development environment

### Installation

```bash
# Clone the repository
git clone https://github.com/kfstorm/sound-break.git
cd sound-break

# Install dependencies
pnpm install

# Install Tauri CLI
pnpm add -g @tauri-apps/cli

# Development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Configuration

### Meeting Applications

SoundBreak detects meeting applications by their exact process names. Common examples:

- **Feishu/Lark**: `Lark Helper (Iron)`
- **Zoom**: `zoom.us` or `Zoom`
- **Microsoft Teams**: `Microsoft Teams`
- **WebEx**: `Cisco Webex Meetings`

### Finding Process Names

To find the exact process name for your meeting application:

1. Start the meeting application
2. Open Terminal and run: `pgrep -l <partial_name>`
3. Use the exact process name shown in the output
4. Configure it in SoundBreak settings

### Auto-start Configuration

Enable auto-start through the system tray menu to have SoundBreak automatically launch when you log in to macOS.

## Architecture

### Core Components

- **MonitoringService**: Central monitoring loop and state management
- **MeetingDetector**: Process-based meeting detection with exact name matching
- **MusicController**: Universal music control via MediaRemote framework
- **System Tray**: Real-time status updates and user controls

### Detection Methods

- **Meeting Detection**: Uses `pgrep ^process_name$` for exact process matching
- **Music Detection**: Accesses MediaRemote framework through AppleScript
- **State Management**: Mutex-protected shared state across background threads

## Privacy & Security

- **No Network Access**: All functionality is local to your machine
- **Minimal Permissions**: Only requires accessibility permissions for process monitoring
- **No Data Collection**: No analytics, telemetry, or personal data is collected
- **Open Source**: Full source code is available for inspection

## System Requirements

- macOS 10.15 (Catalina) or later
- Intel or Apple Silicon Mac
- Accessibility permissions for process monitoring

## Permissions

On first run, macOS will prompt for accessibility permissions. This is required for:

- Process monitoring to detect meeting applications
- System-level music control integration

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following the coding guidelines in `.github/copilot-instructions.md`
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) for native desktop performance
- Uses [SvelteKit](https://kit.svelte.dev/) for the user interface
- Leverages macOS MediaRemote framework for universal music compatibility
