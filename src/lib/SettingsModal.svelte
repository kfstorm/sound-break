<script>
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher, onMount, onDestroy } from "svelte";

  export let isOpen = false;
  
  const dispatch = createEventDispatcher();
  
  let meetingConfig = { process_names: [] };
  let newProcessName = "";
  let isLoading = false;
  let error = null;
  let successMessage = null;

  async function loadConfig() {
    try {
      meetingConfig = await invoke("get_meeting_config");
    } catch (e) {
      console.error("Failed to load meeting config:", e);
      error = `Failed to load configuration: ${e}`;
    }
  }

  async function saveConfig() {
    isLoading = true;
    error = null;
    successMessage = null;
    
    try {
      // Filter out empty process names
      const filteredConfig = {
        process_names: meetingConfig.process_names.filter(name => name.trim() !== "")
      };
      
      await invoke("update_meeting_config", { config: filteredConfig });
      successMessage = "Configuration saved successfully!";
      setTimeout(() => {
        successMessage = null;
      }, 3000);
      
      // Dispatch event to parent to refresh status
      dispatch('configUpdated');
    } catch (e) {
      console.error("Failed to save meeting config:", e);
      error = `Failed to save configuration: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function addProcessName() {
    if (newProcessName.trim() !== "" && !meetingConfig.process_names.includes(newProcessName.trim())) {
      meetingConfig.process_names = [...meetingConfig.process_names, newProcessName.trim()];
      newProcessName = "";
    }
  }

  function removeProcessName(index) {
    meetingConfig.process_names = meetingConfig.process_names.filter((_, i) => i !== index);
  }

  function handleInputKeydown(event) {
    if (event.key === 'Enter') {
      addProcessName();
    }
  }

  function closeModal() {
    isOpen = false;
    error = null;
    successMessage = null;
    dispatch('close');
  }

  function handleOverlayKeydown(event) {
    if (event.key === 'Enter' || event.key === ' ') {
      closeModal();
    }
  }


  let globalKeydownHandler;

  // Handle global escape key
  function handleGlobalKeydown(event) {
    if (event.key === 'Escape' && isOpen) {
      closeModal();
    }
  }

  // Load config when modal opens and manage focus/keyboard
  $: if (isOpen) {
    loadConfig();
    // Focus the modal content when it opens for keyboard navigation
    setTimeout(() => {
      const modalContent = document.querySelector('.modal-content');
      if (modalContent) modalContent.focus();
    }, 100);
    
    // Add global escape key listener
    if (typeof document !== 'undefined') {
      document.addEventListener('keydown', handleGlobalKeydown);
    }
  } else {
    // Remove listener when modal closes
    if (typeof document !== 'undefined') {
      document.removeEventListener('keydown', handleGlobalKeydown);
    }
  }

  onDestroy(() => {
    // Cleanup on component destroy
    if (typeof document !== 'undefined') {
      document.removeEventListener('keydown', handleGlobalKeydown);
    }
  });
</script>

{#if isOpen}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div 
    class="modal-overlay" 
    role="button" 
    tabindex="0"
    aria-label="Close modal"
    on:click={closeModal}
    on:keydown={handleOverlayKeydown}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div 
      class="modal-content" 
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-title"
      tabindex="-1"
      on:click|stopPropagation
    >
      <div class="modal-header">
        <h2 id="modal-title">Settings</h2>
        <button class="close-button" on:click={closeModal}>&times;</button>
      </div>

      <div class="modal-body">
        {#if error}
          <div class="error-message">
            {error}
          </div>
        {/if}

        {#if successMessage}
          <div class="success-message">
            {successMessage}
          </div>
        {/if}

        <div class="section">
          <h3>Meeting Apps</h3>
          <p class="help-text">
            Add exact process names. Use <code>pgrep -l app_name</code> to find them.
          </p>

          <div class="process-list">
            {#each meetingConfig.process_names as processName, index}
              <div class="process-item">
                <input 
                  type="text" 
                  bind:value={meetingConfig.process_names[index]}
                  placeholder="Process name"
                  class="process-input"
                />
                <button 
                  class="remove-button" 
                  on:click={() => removeProcessName(index)}
                  title="Remove this process"
                >
                  ✕
                </button>
              </div>
            {/each}
          </div>

          <div class="add-process">
            <input 
              type="text" 
              bind:value={newProcessName}
              on:keydown={handleInputKeydown}
              placeholder="Add process..."
              class="add-input"
            />
            <button 
              class="add-button" 
              on:click={addProcessName}
              disabled={!newProcessName.trim()}
            >
              Add
            </button>
          </div>
        </div>

        <div class="section">
          <h3>Examples</h3>
          <div class="examples">
            <div class="example-item"><code>Lark Helper (Iron)</code></div>
            <div class="example-item"><code>zoom.us</code></div>
            <div class="example-item"><code>Microsoft Teams</code></div>
            <div class="example-item"><code>Cisco Webex Meetings</code></div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="secondary-button" on:click={closeModal}>
          Cancel
        </button>
        <button 
          class="primary-button" 
          on:click={saveConfig}
          disabled={isLoading || meetingConfig.process_names.length === 0}
        >
          {isLoading ? 'Saving...' : 'Save Settings'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow-y: auto;
    outline: none;
  }

  .modal-content:focus {
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3), 0 0 0 2px rgba(52, 152, 219, 0.5);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px 0 20px;
    border-bottom: 1px solid #e9ecef;
    margin-bottom: 16px;
  }

  .modal-header h2 {
    margin: 0;
    color: #2c3e50;
    font-size: 1.3rem;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #7f8c8d;
    padding: 0;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-button:hover {
    color: #2c3e50;
  }

  .modal-body {
    padding: 0 20px;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 16px 20px;
    border-top: 1px solid #e9ecef;
    margin-top: 16px;
  }

  .section {
    margin-bottom: 20px;
  }

  .section h3 {
    margin: 0 0 10px 0;
    color: #2c3e50;
    font-size: 1rem;
    font-weight: 600;
  }

  .help-text {
    color: #6c757d;
    font-size: 0.85rem;
    margin-bottom: 12px;
    line-height: 1.3;
  }

  .help-text code {
    background-color: #f8f9fa;
    padding: 2px 4px;
    border-radius: 3px;
    font-family: monospace;
    font-size: 0.85em;
  }

  .process-list {
    margin-bottom: 16px;
  }

  .process-item {
    display: flex;
    gap: 8px;
    margin-bottom: 8px;
    align-items: center;
  }

  .process-input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ced4da;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .process-input:focus {
    outline: none;
    border-color: #3498db;
    box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
  }

  .remove-button {
    background-color: #e74c3c;
    color: white;
    border: none;
    padding: 8px 10px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
    min-width: 32px;
  }

  .remove-button:hover {
    background-color: #c0392b;
  }

  .add-process {
    display: flex;
    gap: 8px;
  }

  .add-input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ced4da;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .add-input:focus {
    outline: none;
    border-color: #3498db;
    box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
  }

  .add-button {
    background-color: #27ae60;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .add-button:hover:not(:disabled) {
    background-color: #219a52;
  }

  .add-button:disabled {
    background-color: #95a5a6;
    cursor: not-allowed;
  }

  .examples {
    background-color: #f8f9fa;
    border-radius: 4px;
    padding: 12px;
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .example-item {
    font-size: 0.8rem;
  }

  .example-item code {
    background-color: #e9ecef;
    padding: 2px 6px;
    border-radius: 3px;
    font-family: monospace;
    font-size: 0.8em;
  }

  .error-message {
    background-color: #fee;
    color: #c33;
    padding: 12px;
    border-radius: 6px;
    margin-bottom: 16px;
    border: 1px solid #fcc;
    font-size: 0.9rem;
  }

  .success-message {
    background-color: #d4edda;
    color: #155724;
    padding: 12px;
    border-radius: 6px;
    margin-bottom: 16px;
    border: 1px solid #c3e6cb;
    font-size: 0.9rem;
  }

  .primary-button {
    background-color: #3498db;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .primary-button:hover:not(:disabled) {
    background-color: #2980b9;
  }

  .primary-button:disabled {
    background-color: #bdc3c7;
    cursor: not-allowed;
  }

  .secondary-button {
    background-color: transparent;
    color: #6c757d;
    border: 1px solid #ced4da;
    padding: 10px 20px;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .secondary-button:hover {
    background-color: #f8f9fa;
    border-color: #adb5bd;
  }

  @media (prefers-color-scheme: dark) {
    .modal-content {
      background: #34495e;
      color: #ecf0f1;
    }

    .modal-header {
      border-bottom-color: #4a5f7a;
    }

    .modal-header h2 {
      color: #ecf0f1;
    }

    .modal-footer {
      border-top-color: #4a5f7a;
    }

    .section h3 {
      color: #ecf0f1;
    }

    .help-text {
      color: #bdc3c7;
    }

    .help-text code {
      background-color: #4a5f7a;
      color: #ecf0f1;
    }

    .process-input, .add-input {
      background-color: #4a5f7a;
      border-color: #5d6d7e;
      color: #ecf0f1;
    }

    .process-input:focus, .add-input:focus {
      border-color: #3498db;
      box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.3);
    }

    .examples {
      background-color: #4a5f7a;
    }

    .example-item code {
      background-color: #5d6d7e;
      color: #ecf0f1;
    }

    .close-button {
      color: #bdc3c7;
    }

    .close-button:hover {
      color: #ecf0f1;
    }

    .secondary-button {
      color: #bdc3c7;
      border-color: #5d6d7e;
    }

    .secondary-button:hover {
      background-color: #4a5f7a;
      border-color: #6c7b7f;
    }
  }
</style>