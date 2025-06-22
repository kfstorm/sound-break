<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  let monitoringStatus = null;
  let isLoading = false;
  let error = null;
  let statusInterval;

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

  onMount(() => {
    updateStatus();
    // Update status every 3 seconds
    statusInterval = setInterval(updateStatus, 3000);
  });

  onDestroy(() => {
    if (statusInterval) {
      clearInterval(statusInterval);
    }
  });

  function formatTimestamp(timestamp) {
    return new Date(timestamp * 1000).toLocaleTimeString();
  }
</script>

<main class="container">
  <header>
    <h1>🎵 SoundBreak</h1>
    <p class="subtitle">Automatic music control for meetings</p>
  </header>

  {#if error}
    <div class="error">
      {error}
    </div>
  {/if}

  <div class="status-section">
    <div class="status-card">
      <h2>Monitoring Status</h2>
      <div class="status-indicator">
        <span class="status-dot {monitoringStatus?.is_active ? 'active' : 'inactive'}"></span>
        <span>{monitoringStatus?.is_active ? 'Active' : 'Inactive'}</span>
      </div>

      <button
        on:click={toggleMonitoring}
        disabled={isLoading}
        class="primary-button"
      >
        {isLoading ? 'Loading...' : (monitoringStatus?.is_active ? 'Stop Monitoring' : 'Start Monitoring')}
      </button>

      {#if monitoringStatus?.last_action}
        <p class="last-action">
          <strong>Last Action:</strong> {monitoringStatus.last_action}
        </p>
      {/if}
    </div>

    <div class="status-card">
      <h2>Meeting Status</h2>
      {#if monitoringStatus?.meeting_status}
        <div class="meeting-info">
          <div class="status-indicator">
            <span class="status-dot {monitoringStatus.meeting_status.in_meeting ? 'meeting' : 'no-meeting'}"></span>
            <span>{monitoringStatus.meeting_status.in_meeting ? 'In Meeting' : 'No Meeting'}</span>
          </div>

          {#if monitoringStatus.meeting_status.active_apps.length > 0}
            <div class="active-apps">
              <h4>Active Meeting Apps:</h4>
              <ul>
                {#each monitoringStatus.meeting_status.active_apps as app}
                  <li>{app.name}</li>
                {/each}
              </ul>
            </div>
          {/if}
        </div>
      {:else}
        <p>No meeting data available</p>
      {/if}
    </div>

    <div class="status-card">
      <h2>Music Status</h2>
      {#if monitoringStatus?.music_status}
        <div class="music-info">
          <div class="status-indicator">
            <span class="status-dot {monitoringStatus.music_status.is_playing ? 'playing' : 'paused'}"></span>
            <span>{monitoringStatus.music_status.is_playing ? 'Playing' : 'Paused'}</span>
          </div>

          {#if monitoringStatus.music_status.player_name}
            <p><strong>Player:</strong> {monitoringStatus.music_status.player_name}</p>
          {/if}

          {#if monitoringStatus.music_status.track_info}
            <p><strong>Track:</strong> {monitoringStatus.music_status.track_info}</p>
          {/if}
        </div>
      {:else}
        <p>No music data available</p>
      {/if}

      <div class="music-controls">
        <button on:click={() => controlMusic('pause')} disabled={isLoading} class="control-button">
          ⏸️ Pause
        </button>
        <button on:click={() => controlMusic('play')} disabled={isLoading} class="control-button">
          ▶️ Play
        </button>
      </div>
    </div>
  </div>

  <div class="info-section">
    <h3>How it works</h3>
    <p>SoundBreak automatically detects when you enter or exit a Feishu Meeting and pauses/resumes your music accordingly. The app runs in the background and can be controlled from the system tray.</p>

    <div class="features">
      <div class="feature">
        <span class="feature-icon">🎯</span>
        <span>Detects Feishu Meeting automatically</span>
      </div>
      <div class="feature">
        <span class="feature-icon">🎵</span>
        <span>Works with Spotify, Apple Music, and more</span>
      </div>
      <div class="feature">
        <span class="feature-icon">⚡</span>
        <span>Minimal system resource usage</span>
      </div>
    </div>
  </div>

  <footer>
    {#if monitoringStatus?.last_check}
      <p class="last-update">Last updated: {formatTimestamp(monitoringStatus.last_check)}</p>
    {/if}
  </footer>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    font-size: 16px;
    line-height: 1.6;
    color: #2c3e50;
    background-color: #f8f9fa;
  }

  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    min-height: 100vh;
  }

  header {
    text-align: center;
    margin-bottom: 30px;
  }

  h1 {
    font-size: 2.5rem;
    margin: 0;
    color: #2c3e50;
  }

  .subtitle {
    color: #7f8c8d;
    font-size: 1.1rem;
    margin: 10px 0 0 0;
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
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
    margin-bottom: 30px;
  }

  .status-card {
    background: white;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    border: 1px solid #e9ecef;
  }

  .status-card h2 {
    margin-top: 0;
    color: #2c3e50;
    font-size: 1.3rem;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 15px 0;
    font-weight: 500;
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
    margin-top: 15px;
    padding: 10px;
    background-color: #f8f9fa;
    border-radius: 6px;
    font-size: 0.9rem;
    color: #6c757d;
  }

  .active-apps ul {
    list-style: none;
    padding: 0;
    margin: 10px 0;
  }

  .active-apps li {
    background-color: #e3f2fd;
    color: #1976d2;
    padding: 6px 12px;
    border-radius: 20px;
    margin: 4px 0;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .info-section {
    background: white;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    border: 1px solid #e9ecef;
    margin-bottom: 20px;
  }

  .info-section h3 {
    margin-top: 0;
    color: #2c3e50;
  }

  .features {
    display: grid;
    gap: 12px;
    margin-top: 20px;
  }

  .feature {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .feature-icon {
    font-size: 1.2rem;
  }

  footer {
    text-align: center;
    color: #7f8c8d;
    font-size: 0.9rem;
  }

  .last-update {
    margin: 0;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #ecf0f1;
      background-color: #2c3e50;
    }

    .status-card,
    .info-section {
      background: #34495e;
      border-color: #4a5f7a;
    }

    .status-card h2,
    .info-section h3 {
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

    .active-apps li {
      background-color: #2980b9;
      color: #ecf0f1;
    }
  }
</style>
