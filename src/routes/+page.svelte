<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import SettingsModal from "../lib/SettingsModal.svelte";

  let monitoringStatus = null;
  let isLoading = false;
  let error = null;
  let statusInterval;
  let showSettings = false;

  async function toggleMonitoring() {
    isLoading = true;
    error = null;
    try {
      const result = await invoke("toggle_monitoring");
      console.log("Toggle result:", result);
      await updateStatus();
    } catch (e) {
      error = `Failed to toggle monitoring: ${e}`;
      console.error("Toggle error:", e);
    } finally {
      isLoading = false;
    }
  }

  async function controlMusic(action) {
    isLoading = true;
    error = null;
    try {
      const result = await invoke("control_music", { action });
      console.log("Music control result:", result);
      await updateStatus();
    } catch (e) {
      error = `Failed to control music: ${e}`;
      console.error("Music control error:", e);
    } finally {
      isLoading = false;
    }
  }

  async function updateStatus() {
    try {
      monitoringStatus = await invoke("get_monitoring_status");
    } catch (e) {
      console.error("Failed to get status:", e);
    }
  }

  onMount(async () => {
    updateStatus();
    // Update status every 3 seconds
    statusInterval = setInterval(updateStatus, 3000);
    
    // Listen for auto-open-settings event (production mode)
    const unlisten = await listen('auto-open-settings', () => {
      showSettings = true;
    });
    
    // Clean up listener on component destroy
    return unlisten;
  });

  onDestroy(() => {
    if (statusInterval) {
      clearInterval(statusInterval);
    }
  });

  function formatTimestamp(timestamp) {
    return new Date(timestamp * 1000).toLocaleTimeString();
  }

  function openSettings() {
    showSettings = true;
  }

  function closeSettings() {
    showSettings = false;
  }

  function handleConfigUpdated() {
    updateStatus(); // Refresh status after config update
  }
</script>

<main class="container">
  <header>
    <div class="header-content">
      <h1>🎵 SoundBreak</h1>
      <button class="settings-button" on:click={openSettings} title="Settings">
        ⚙️
      </button>
    </div>
  </header>

  {#if error}
    <div class="error">
      {error}
    </div>
  {/if}

  <div class="status-section">
    <div class="status-card">
      <div class="card-header">
        <h2>Monitoring</h2>
        <div class="status-indicator">
          <span class="status-dot {monitoringStatus?.is_active ? 'active' : 'inactive'}"></span>
          <span>{monitoringStatus?.is_active ? 'Active' : 'Inactive'}</span>
        </div>
      </div>

      <button
        on:click={toggleMonitoring}
        disabled={isLoading}
        class="primary-button"
      >
        {isLoading ? 'Loading...' : (monitoringStatus?.is_active ? 'Stop' : 'Start')}
      </button>

      {#if monitoringStatus?.last_action}
        <p class="last-action">{monitoringStatus.last_action}</p>
      {/if}
    </div>

    <div class="status-card">
      <div class="card-header">
        <h2>Meeting</h2>
        {#if monitoringStatus?.meeting_status}
          <div class="status-indicator">
            <span class="status-dot {monitoringStatus.meeting_status.in_meeting ? 'meeting' : 'no-meeting'}"></span>
            <span>{monitoringStatus.meeting_status.in_meeting ? 'Active' : 'None'}</span>
          </div>
        {/if}
      </div>

      {#if monitoringStatus?.meeting_status?.active_apps?.length > 0}
        <div class="app-list">
          {#each monitoringStatus.meeting_status.active_apps as app}
            <div class="app-item {app.is_running ? 'running' : 'stopped'}">
              <span class="app-status">{app.is_running ? '🟢' : '🔴'}</span>
              <span class="app-name">{app.name}</span>
            </div>
          {/each}
        </div>
      {:else}
        <p class="no-data">No apps configured</p>
      {/if}
    </div>

    <div class="status-card">
      <div class="card-header">
        <h2>Music</h2>
        {#if monitoringStatus?.music_status}
          <div class="status-indicator">
            <span class="status-dot {monitoringStatus.music_status.is_playing ? 'playing' : 'paused'}"></span>
            <span>{monitoringStatus.music_status.is_playing ? 'Playing' : 'Paused'}</span>
          </div>
        {/if}
      </div>

      {#if monitoringStatus?.music_status?.player_name || monitoringStatus?.music_status?.track_info}
        <div class="music-info">
          {#if monitoringStatus.music_status.player_name}
            <p class="player-name">{monitoringStatus.music_status.player_name}</p>
          {/if}
          {#if monitoringStatus.music_status.track_info}
            <p class="track-info">{monitoringStatus.music_status.track_info}</p>
          {/if}
        </div>
      {:else}
        <p class="no-data">No player detected</p>
      {/if}

      <div class="music-controls">
        <button on:click={() => controlMusic('pause')} disabled={isLoading} class="control-button">
          ⏸️
        </button>
        <button on:click={() => controlMusic('play')} disabled={isLoading} class="control-button">
          ▶️
        </button>
      </div>
    </div>
  </div>

  {#if monitoringStatus?.last_check}
    <footer class="last-update">
      Updated: {formatTimestamp(monitoringStatus.last_check)}
    </footer>
  {/if}
</main>

<SettingsModal 
  bind:isOpen={showSettings} 
  on:close={closeSettings}
  on:configUpdated={handleConfigUpdated}
/>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    font-size: 16px;
    line-height: 1.6;
    color: #2c3e50;
    background-color: #f8f9fa;
  }

  .container {
    max-width: 700px;
    margin: 0 auto;
    padding: 16px;
    min-height: 100vh;
  }

  header {
    margin-bottom: 20px;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .settings-button {
    background: white;
    border: 1px solid #e9ecef;
    border-radius: 8px;
    padding: 8px 12px;
    font-size: 1.2rem;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-top: 5px;
  }

  .settings-button:hover {
    background: #f8f9fa;
    border-color: #dee2e6;
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
  }

  h1 {
    font-size: 1.8rem;
    margin: 0;
    color: #2c3e50;
  }

  .error {
    background-color: #fee;
    color: #c33;
    padding: 15px;
    border-radius: 8px;
    margin-bottom: 20px;
    border: 1px solid #fcc;
  }

  .status-section {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 16px;
    margin-bottom: 20px;
  }

  .status-card {
    background: white;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border: 1px solid #e9ecef;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .status-card h2 {
    margin: 0;
    color: #2c3e50;
    font-size: 1.1rem;
    font-weight: 600;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
    font-size: 0.9rem;
  }

  .status-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    display: inline-block;
  }

  .status-dot.active {
    background-color: #27ae60;
    box-shadow: 0 0 0 2px rgba(39, 174, 96, 0.3);
  }

  .status-dot.inactive {
    background-color: #95a5a6;
  }

  .status-dot.meeting {
    background-color: #e74c3c;
    box-shadow: 0 0 0 2px rgba(231, 76, 60, 0.3);
  }

  .status-dot.no-meeting {
    background-color: #27ae60;
  }

  .status-dot.playing {
    background-color: #3498db;
    box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.3);
  }

  .status-dot.paused {
    background-color: #f39c12;
  }

  .primary-button {
    background-color: #3498db;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
    width: 100%;
  }

  .primary-button:hover:not(:disabled) {
    background-color: #2980b9;
  }

  .primary-button:disabled {
    background-color: #bdc3c7;
    cursor: not-allowed;
  }

  .control-button {
    background-color: #ecf0f1;
    color: #2c3e50;
    border: 1px solid #bdc3c7;
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .control-button:hover:not(:disabled) {
    background-color: #d5dbdb;
    border-color: #95a5a6;
  }

  .control-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .music-controls {
    display: flex;
    gap: 8px;
    margin-top: 15px;
    flex-wrap: wrap;
  }

  .last-action {
    margin-top: 12px;
    padding: 8px;
    background-color: #f8f9fa;
    border-radius: 4px;
    font-size: 0.85rem;
    color: #6c757d;
  }

  .app-list {
    margin-top: 8px;
  }

  .app-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
    font-size: 0.9rem;
  }

  .app-status {
    font-size: 0.8rem;
  }

  .app-name {
    font-weight: 500;
  }

  .music-info {
    margin: 8px 0;
  }

  .player-name, .track-info {
    margin: 4px 0;
    font-size: 0.9rem;
    color: #6c757d;
  }

  .no-data {
    margin: 8px 0;
    font-size: 0.9rem;
    color: #6c757d;
    font-style: italic;
  }

  .last-update {
    text-align: center;
    color: #7f8c8d;
    font-size: 0.8rem;
    margin: 16px 0 0 0;
    padding-top: 16px;
    border-top: 1px solid #e9ecef;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #ecf0f1;
      background-color: #2c3e50;
    }

    .status-card {
      background: #34495e;
      border-color: #4a5f7a;
    }

    .status-card h2 {
      color: #ecf0f1;
    }

    .control-button {
      background-color: #4a5f7a;
      color: #ecf0f1;
      border-color: #5d6d7e;
    }

    .control-button:hover:not(:disabled) {
      background-color: #5d6d7e;
    }

    .last-action {
      background-color: #4a5f7a;
      color: #bdc3c7;
    }

    .last-update {
      border-top-color: #4a5f7a;
    }

    .settings-button {
      background: #4a5f7a;
      border-color: #5d6d7e;
      color: #ecf0f1;
    }

    .settings-button:hover {
      background: #5d6d7e;
      border-color: #6c7b7f;
    }
  }
</style>
