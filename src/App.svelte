<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let status = $state("Готов к работе");
  let itemsCount = $state(0);
  let isProcessing = $state(false);

  // Svelte 5 Runes: $derived
  let badgeText = $derived(itemsCount > 0 ? `${itemsCount} активных` : "В покое");

  async function handleAction() {
    isProcessing = true;
    status = "Выполнение операции...";
    try {
      // Пример вызова Tauri v2 команды
      const res = await invoke<string>("execute_action");
      status = res;
      itemsCount += 1;
    } catch (e) {
      status = `Ошибка: ${e}`;
    } finally {
      isProcessing = false;
    }
  }
</script>

<main class="flyout-container">
  <header class="flyout-header">
    <div class="brand">
      <div class="status-dot"></div>
      <h1>StashIt</h1>
    </div>
    <span class="badge">{badgeText}</span>
  </header>

  <section class="content">
    <p class="desc">Легковесный плавающий карман (Drag-and-Drop Shelf) для Windows 10 & 11 на Rust и Tauri v2.</p>
    
    <div class="status-card">
      <span class="status-label">Статус:</span>
      <span class="status-val">{status}</span>
    </div>

    <button 
      class="primary-btn" 
      disabled={isProcessing} 
      onclick={handleAction}
    >
      {isProcessing ? "Обработка..." : "Выполнить действие"}
    </button>
  </section>

  <footer class="flyout-footer">
    <span class="version">Kobalt Suite • v1.0.0</span>
  </footer>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI Variable Text", "Segoe UI", sans-serif;
    user-select: none;
    background: transparent;
    overflow: hidden;
  }

  .flyout-container {
    width: 320px;
    padding: 16px;
    box-sizing: border-box;
    background: rgba(18, 24, 38, 0.82);
    backdrop-filter: blur(28px) saturate(180%);
    -webkit-backdrop-filter: blur(28px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.5);
    color: #ffffff;
  }

  .flyout-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #10B981;
    box-shadow: 0 0 8px #10B981;
  }

  h1 {
    font-size: 14px;
    font-weight: 600;
    margin: 0;
    color: #ffffff;
  }

  .badge {
    font-size: 11px;
    padding: 2px 8px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.08);
    color: #94A3B8;
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .desc {
    font-size: 12px;
    color: #94A3B8;
    margin: 0;
    line-height: 1.4;
  }

  .status-card {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 8px;
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .status-label {
    font-size: 10px;
    text-transform: uppercase;
    color: #64748B;
    letter-spacing: 0.5px;
  }

  .status-val {
    font-size: 13px;
    color: #F1F5F9;
  }

  .primary-btn {
    background: #38BDF8;
    color: #ffffff;
    border: none;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .primary-btn:hover {
    filter: brightness(1.15);
  }

  .primary-btn:active {
    transform: scale(0.98);
  }

  .flyout-footer {
    margin-top: 14px;
    padding-top: 8px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
    text-align: center;
  }

  .version {
    font-size: 10px;
    color: #64748B;
  }
</style>