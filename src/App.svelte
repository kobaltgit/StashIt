<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  interface StashItem {
    id: String;
    kind: "file" | "folder" | "image" | "text" | "url";
    name: string;
    path: string | null;
    size_bytes: number | null;
    formatted_size: string;
    extension: string;
    text_preview: string | null;
    created_at: number;
  }

  import { translations, type Lang } from "./i18n";

  type ThemeMode = "system" | "dark" | "light";

  // --- Svelte 5 Runes ($state) ---
  let items = $state<StashItem[]>([]);
  let isDragOver = $state(false);
  let theme = $state<ThemeMode>((localStorage.getItem("stashit_theme") as ThemeMode) || "system");
  let effectiveTheme = $state<"dark" | "light">("dark");
  let lang = $state<Lang>((localStorage.getItem("stashit_lang") as Lang) || "ru");
  let showSettings = $state(false);
  let shakeEnabled = $state(true);
  let autoDragEnabled = $state(false);
  let autostartEnabled = $state(false);
  let statusNotice = $state<string | null>(null);

  let autoClearEnabled = $state<boolean>(
    localStorage.getItem("stashit_autoclear") !== "false"
  );
  let selectedIds = $state<Set<string>>(new Set());
  let clearCountdown = $state<number | null>(null);
  let countdownTimer: number | null = null;

  // --- Svelte 5 Runes ($derived) ---
  let itemsCount = $derived(items.length);
  let totalBytes = $derived(
    items.reduce((acc, it) => acc + (it.size_bytes || 0), 0)
  );
  let formattedTotalSize = $derived(formatBytes(totalBytes));

  let allSelected = $derived(
    itemsCount > 0 && selectedIds.size === itemsCount
  );
  let selectedCount = $derived(selectedIds.size);
  let t = $derived(translations[lang]);

  function toggleLang() {
    lang = lang === "ru" ? "en" : "ru";
    localStorage.setItem("stashit_lang", lang);
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 Б";
    const k = 1024;
    const sizes = ["Б", "КБ", "МБ", "ГБ"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }

  // Автоматически синхронизируем выделение: новые файлы сразу выделены
  function syncSelectionWithItems() {
    selectedIds = new Set(items.map((it) => String(it.id)));
  }

  function toggleSelectAll() {
    if (allSelected) {
      selectedIds = new Set();
    } else {
      selectedIds = new Set(items.map((it) => String(it.id)));
    }
  }

  function toggleItemSelection(id: string, e?: MouseEvent) {
    if (e) e.stopPropagation();
    cancelClearCountdown();
    const next = new Set(selectedIds);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    selectedIds = next;
  }

  // --- Таймер автоочистки 5 сек ---
  function startClearCountdown() {
    if (!autoClearEnabled || items.length === 0) return;
    cancelClearCountdown();
    clearCountdown = 5;

    countdownTimer = window.setInterval(() => {
      if (clearCountdown !== null && clearCountdown > 1) {
        clearCountdown -= 1;
      } else {
        cancelClearCountdown();
        clearAll();
      }
    }, 1000);
  }

  function cancelClearCountdown() {
    if (countdownTimer !== null) {
      clearInterval(countdownTimer);
      countdownTimer = null;
    }
    clearCountdown = null;
  }

  function toggleAutoClearSetting() {
    autoClearEnabled = !autoClearEnabled;
    localStorage.setItem("stashit_autoclear", String(autoClearEnabled));
    if (!autoClearEnabled) {
      cancelClearCountdown();
    }
  }

  // --- Управление темами ---
  function updateTheme() {
    if (theme === "system") {
      const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      effectiveTheme = isDark ? "dark" : "light";
    } else {
      effectiveTheme = theme;
    }
    document.documentElement.setAttribute("data-theme", effectiveTheme);
    localStorage.setItem("stashit_theme", theme);
  }

  function cycleTheme() {
    if (theme === "system") theme = "dark";
    else if (theme === "dark") theme = "light";
    else theme = "system";
    updateTheme();
  }

  // --- Tauri IPC операции ---
  async function loadItems() {
    try {
      items = await invoke<StashItem[]>("get_stash_items");
    } catch (e) {
      console.error("Ошибка загрузки элементов:", e);
    }
  }

  async function removeItem(id: String) {
    try {
      items = await invoke<StashItem[]>("remove_stash_item", { id });
      if (items.length === 0) {
        // Если карман опустел, автоскрытие
        setTimeout(hideShelf, 300);
      }
    } catch (e) {
      console.error("Ошибка удаления:", e);
    }
  }

  async function clearAll() {
    try {
      items = await invoke<StashItem[]>("clear_stash");
      showNotice(t.pocketCleared);
      setTimeout(hideShelf, 400);
    } catch (e) {
      console.error("Ошибка очистки:", e);
    }
  }

  async function hideShelf() {
    try {
      await invoke("hide_shelf");
    } catch (e) {
      console.error(e);
    }
  }

  async function toggleAutostartSetting() {
    try {
      const next = !autostartEnabled;
      autostartEnabled = await invoke<boolean>("toggle_autostart", { enable: next });
      showNotice(autostartEnabled ? t.autostartEnabled : t.autostartDisabled);
    } catch (e) {
      console.error("Ошибка автозапуска:", e);
    }
  }

  async function updateTriggerSettings() {
    try {
      await invoke("set_trigger_mode", {
        shake: shakeEnabled,
        autoDrag: autoDragEnabled,
      });
    } catch (e) {
      console.error(e);
    }
  }

  async function copyAllItems() {
    const filePaths = items
      .map((it) => it.path)
      .filter((p): p is string => Boolean(p));

    if (filePaths.length > 0) {
      try {
        await invoke("copy_to_clipboard", { paths: filePaths });
        showNotice(t.copiedFiles(filePaths.length));
        return;
      } catch (err) {
        console.error("Ошибка нативного копирования файлов:", err);
      }
    }

    // Запасной вариант для текстовых заметок или ссылок
    const fallbackText = items
      .map((it) => it.path || it.text_preview || "")
      .filter(Boolean)
      .join("\n");

    if (fallbackText) {
      await navigator.clipboard.writeText(fallbackText);
      showNotice(t.copiedText);
    }
  }

  async function copySingleItem(item: StashItem, e: MouseEvent) {
    e.stopPropagation();
    if (item.path) {
      try {
        await invoke("copy_to_clipboard", { paths: [item.path] });
        showNotice(t.fileCopied);
        return;
      } catch (err) {
        console.error("Ошибка копирования файла:", err);
      }
    }

    const text = item.path || item.text_preview || "";
    if (text) {
      await navigator.clipboard.writeText(text);
      showNotice(t.textCopied);
    }
  }

  function showNotice(msg: string) {
    statusNotice = msg;
    setTimeout(() => {
      if (statusNotice === msg) statusNotice = null;
    }, 2000);
  }

  // Нативный OLE Drag Out одного файла
  async function handleItemDragStart(e: DragEvent, item: StashItem) {
    cancelClearCountdown();
    if (item.path) {
      e.preventDefault();
      try {
        await invoke("drag_item", {
          paths: [item.path],
        });
      } catch (err) {
        console.error("Ошибка native drag-out:", err);
      }
    } else if (e.dataTransfer) {
      if (item.text_preview) {
        e.dataTransfer.setData("text/plain", item.text_preview);
      }
      e.dataTransfer.effectAllowed = "copyMove";
    }
  }

  // Нативный OLE Drag Out всей стопки (всех выделенных файлов или всех файлов кармана)
  async function handleDragAll(e: DragEvent) {
    cancelClearCountdown();
    const targetItems = selectedIds.size > 0
      ? items.filter((it) => selectedIds.has(String(it.id)))
      : items;

    const filePaths = targetItems
      .map((it) => it.path)
      .filter((p): p is string => Boolean(p));

    if (filePaths.length > 0) {
      e.preventDefault();
      try {
        await invoke("drag_item", {
          paths: filePaths,
        });
      } catch (err) {
        console.error("Ошибка drag all:", err);
      }
    } else if (e.dataTransfer) {
      const texts = targetItems
        .map((it) => it.path || it.text_preview || "")
        .filter(Boolean)
        .join("\n");
      if (texts) {
        e.dataTransfer.setData("text/plain", texts);
      }
      e.dataTransfer.effectAllowed = "copyMove";
    }
  }

  onMount(() => {
    updateTheme();
    loadItems().then(() => syncSelectionWithItems());

    // Проверка автозапуска
    invoke<boolean>("check_autostart")
      .then((res) => (autostartEnabled = res))
      .catch(() => {});

    // Слушатель системной темы
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const themeListener = () => {
      if (theme === "system") updateTheme();
    };
    mediaQuery.addEventListener("change", themeListener);

    // Слушатели событий Tauri
    const unlistenUpdated = listen<StashItem[]>("stash-updated", (event) => {
      items = event.payload;
      syncSelectionWithItems();
      cancelClearCountdown();
    });

    const unlistenCleared = listen("stash-cleared", () => {
      items = [];
      selectedIds = new Set();
      cancelClearCountdown();
    });

    const unlistenDragCompleted = listen<string[]>("drag-out-completed", (event) => {
      const droppedPaths = new Set(event.payload || []);
      if (droppedPaths.size === 0) return;

      // Если перетаскивали отдельный файл из нескольких — удаляем только его
      if (droppedPaths.size === 1 && items.length > 1) {
        const itemToRemove = items.find((it) => it.path && droppedPaths.has(it.path));
        if (itemToRemove) {
          removeItem(itemToRemove.id);
          return;
        }
      }

      // Если перетащили всю пачку или последний файл — запускаем 5с таймер автоочистки кнопки
      startClearCountdown();
    });

    const unlistenEnter = listen("drag-enter", () => {
      isDragOver = true;
      cancelClearCountdown();
    });

    const unlistenLeave = listen("drag-leave", () => {
      isDragOver = false;
    });

    const unlistenMouseUp = listen("global-mouse-up", () => {
      // Если мышь отпустили, а карман пуст — аккуратно скрываемся
      if (items.length === 0 && !isDragOver) {
        hideShelf();
      }
    });

    // Хоткей Escape для скрытия кармана, Ctrl+A для выделения всех
    const keyHandler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        hideShelf();
      } else if ((e.ctrlKey || e.metaKey) && (e.key === "a" || e.key === "A" || e.key === "ф" || e.key === "Ф")) {
        if (items.length > 0) {
          e.preventDefault();
          toggleSelectAll();
        }
      }
    };
    window.addEventListener("keydown", keyHandler);

    // Глобальные слушатели окна для предотвращения курсора-стопа при перетаскивании
    const windowDragOver = (e: DragEvent) => {
      e.preventDefault();
      if (e.dataTransfer) {
        e.dataTransfer.dropEffect = "copy";
      }
    };
    const windowDrop = (e: DragEvent) => {
      e.preventDefault();
    };
    window.addEventListener("dragover", windowDragOver);
    window.addEventListener("drop", windowDrop);

    return () => {
      mediaQuery.removeEventListener("change", themeListener);
      window.removeEventListener("keydown", keyHandler);
      window.removeEventListener("dragover", windowDragOver);
      window.removeEventListener("drop", windowDrop);
      cancelClearCountdown();
      unlistenUpdated.then((f) => f());
      unlistenCleared.then((f) => f());
      unlistenDragCompleted.then((f) => f());
      unlistenEnter.then((f) => f());
      unlistenLeave.then((f) => f());
      unlistenMouseUp.then((f) => f());
    };
  });

  async function handleContainerDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragOver = false;

    if (!e.dataTransfer) return;

    // 1. Проверяем файлы (если браузер / WebView2 передал File объекты с path)
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const filePaths: string[] = [];
      for (let i = 0; i < e.dataTransfer.files.length; i++) {
        const f = e.dataTransfer.files[i];
        const p = (f as any).path;
        if (p) {
          filePaths.push(p);
        }
      }

      if (filePaths.length > 0) {
        try {
          items = await invoke<StashItem[]>("add_stash_item_paths", { paths: filePaths });
          syncSelectionWithItems();
          return;
        } catch (err) {
          console.error("Ошибка добавления файлов через DOM drop:", err);
        }
      }
    }

    // 2. Проверяем URL, ссылки и текст из браузера
    const uriList = e.dataTransfer.getData("text/uri-list");
    const plainText = e.dataTransfer.getData("text/plain");
    const html = e.dataTransfer.getData("text/html");

    let contentToAdd = uriList || plainText;
    if (!contentToAdd && html) {
      const match = html.match(/<img[^>]+src=["']([^"']+)["']/i);
      if (match && match[1]) {
        contentToAdd = match[1];
      }
    }

    if (contentToAdd && contentToAdd.trim()) {
      const lines = contentToAdd
        .split(/\r?\n/)
        .map((l) => l.trim())
        .filter(Boolean);

      // Если ВСЕ непустые строки — URL, добавляем каждую как отдельный элемент.
      // Иначе добавляем весь текст одним вызовом как текстовую заметку.
      const allUrls = lines.length > 0 && lines.every((l) => l.startsWith("http://") || l.startsWith("https://"));

      if (allUrls) {
        for (const line of lines) {
          try {
            items = await invoke<StashItem[]>("add_stash_text", { text: line });
            syncSelectionWithItems();
          } catch (err) {
            console.error("Ошибка добавления ссылки:", err);
          }
        }
      } else {
        try {
          items = await invoke<StashItem[]>("add_stash_text", { text: contentToAdd.trim() });
          syncSelectionWithItems();
        } catch (err) {
          console.error("Ошибка добавления текста:", err);
        }
      }
    }
  }

  function handleContainerDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = "copy";
    }
    isDragOver = true;
  }

  function handleContainerDragEnter(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = "copy";
    }
    isDragOver = true;
  }

  function handleContainerDragLeave() {
    isDragOver = false;
  }
</script>

<main 
  class="shelf-wrapper" 
  class:drag-over={isDragOver}
  ondragenter={handleContainerDragEnter}
  ondragover={handleContainerDragOver}
  ondragleave={handleContainerDragLeave}
  ondrop={handleContainerDrop}
>
  <!-- Верхний тулбар -->
  <header class="shelf-header">
    <div class="brand">
      <div class="glow-indicator" class:active={itemsCount > 0}></div>
      <span class="app-title">StashIt</span>
      {#if itemsCount > 0}
        <span class="badge">{itemsCount}</span>
      {/if}
    </div>

    <div class="header-actions">
      <!-- Переключатель тем -->
      <button 
        class="icon-btn" 
        title={t.themeTooltip(theme)}
        onclick={cycleTheme}
      >
        {#if theme === "system"}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>
        {:else if theme === "dark"}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
        {:else}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
        {/if}
      </button>

      <!-- Переключатель языка RU/EN -->
      <button 
        class="icon-btn lang-btn" 
        title={t.langTooltip}
        onclick={toggleLang}
      >
        <span class="lang-text">{lang.toUpperCase()}</span>
      </button>

      <!-- Настройки -->
      <button 
        class="icon-btn" 
        title={t.settingsTooltip}
        class:active={showSettings}
        onclick={() => (showSettings = !showSettings)}
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
      </button>

      <!-- Скрыть / Закрыть -->
      <button class="icon-btn close" title={t.closeTooltip} onclick={hideShelf}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      </button>
    </div>
  </header>

  <!-- Панель настроек (дропдаун) -->
  {#if showSettings}
    <div class="settings-panel">
      <div class="setting-item">
        <label>
          <input 
            type="checkbox" 
            bind:checked={shakeEnabled} 
            onchange={updateTriggerSettings}
          />
          <span>{t.shakeTrigger}</span>
        </label>
      </div>
      <div class="setting-item">
        <label>
          <input 
            type="checkbox" 
            bind:checked={autoDragEnabled} 
            onchange={updateTriggerSettings}
          />
          <span>{t.autoDragTrigger}</span>
        </label>
      </div>
      <div class="setting-item">
        <label>
          <input 
            type="checkbox" 
            checked={autostartEnabled} 
            onchange={toggleAutostartSetting}
          />
          <span>{t.autostart}</span>
        </label>
      </div>
      <div class="setting-item">
        <label>
          <input 
            type="checkbox" 
            checked={autoClearEnabled} 
            onchange={toggleAutoClearSetting}
          />
          <span>{t.autoClear}</span>
        </label>
      </div>
    </div>
  {/if}

  <!-- Уведомление о статусе -->
  {#if statusNotice}
    <div class="toast-notice">
      {statusNotice}
    </div>
  {/if}

  <!-- Основной контент: Дроп-зона или Список файлов -->
  <div class="shelf-body">
    {#if itemsCount === 0}
      <div class="empty-dropzone" class:active-hover={isDragOver}>
        <div class="drop-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        </div>
        <p class="drop-hint">{t.dropHint}</p>
        <span class="sub-hint">{t.subHint}</span>
      </div>
    {:else}
      <div class="items-list">
        {#each items as item, index (item.id ?? `fallback_${index}`)}
          <div 
            class="item-card" 
            class:selected={selectedIds.has(String(item.id))}
            role="button"
            tabindex="0"
            draggable="true"
            ondragstart={(e) => handleItemDragStart(e, item)}
            ondragend={() => { if (!item.path) startClearCountdown(); }}
            onclick={() => toggleItemSelection(String(item.id))}
            onkeydown={(e) => {
              if (e.key === ' ' || e.key === 'Enter') {
                e.preventDefault();
                toggleItemSelection(String(item.id));
              }
            }}
            title={t.itemTitle}
          >
            <div 
              class="item-checkbox" 
              class:checked={selectedIds.has(String(item.id))}
              title={selectedIds.has(String(item.id)) ? t.deselectAll : t.selectAll}
            >
              {#if selectedIds.has(String(item.id))}
                ✓
              {/if}
            </div>

            <div class="item-type-icon" data-kind={item.kind}>
              {#if item.kind === 'folder'}
                📁
              {:else if item.kind === 'image'}
                🖼️
              {:else if item.kind === 'url'}
                🔗
              {:else if item.kind === 'text'}
                📝
              {:else}
                📄
              {/if}
            </div>

            <div class="item-info">
              <span class="item-name">{item.name}</span>
              <span class="item-meta">
                {item.formatted_size}
                {#if item.extension}
                  • .{item.extension}
                {/if}
              </span>
            </div>

            <div class="item-actions">
              <button 
                class="item-action-btn" 
                title={t.copyFile}
                onclick={(e) => copySingleItem(item, e)}
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
              </button>
              <button 
                class="item-action-btn remove" 
                title={t.removeItem}
                onclick={(e) => { e.stopPropagation(); cancelClearCountdown(); removeItem(item.id); }}
              >
                ✕
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Нижний футер со сводкой, захватом всей пачки и групповыми действиями -->
  {#if itemsCount > 0}
    <!-- Главная плашка перетаскивания всей пачки сразу -->
    <div 
      class="master-drag-handle"
      role="button"
      tabindex="0"
      draggable="true"
      ondragstart={handleDragAll}
      ondragend={() => {
        // Для файлов таймер приходит через Rust-событие drag-out-completed.
        // Если в пачке нет ни одного файла — все текст/URL, запускаем таймер здесь.
        const targetItems = selectedIds.size > 0
          ? items.filter((it) => selectedIds.has(String(it.id)))
          : items;
        if (!targetItems.some((it) => it.path)) startClearCountdown();
      }}
      title={t.dragHint}
    >
      <div class="drag-icon-grip">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="9" cy="5" r="1"/><circle cx="9" cy="12" r="1"/><circle cx="9" cy="19" r="1"/><circle cx="15" cy="5" r="1"/><circle cx="15" cy="12" r="1"/><circle cx="15" cy="19" r="1"/></svg>
      </div>
      <span class="master-drag-title">
        {#if selectedCount > 0 && selectedCount < itemsCount}
          {t.dragSelected(selectedCount)}
        {:else}
          {t.dragAll(itemsCount)}
        {/if}
      </span>
      <span class="master-drag-badge">{formattedTotalSize}</span>
    </div>

    <footer class="shelf-footer">
      <div class="selection-control">
        <button 
          class="text-link-btn" 
          onclick={toggleSelectAll}
          title={allSelected ? t.deselectAll : t.selectAll}
        >
          {allSelected ? t.deselectAll : t.selectAll}
        </button>
      </div>

      <div class="footer-buttons">
        <button class="action-btn" title={t.copyFile} onclick={copyAllItems}>
          {t.copyButton}
        </button>
        <button 
          class="action-btn danger" 
          class:countdown-active={clearCountdown !== null}
          title={clearCountdown !== null ? t.clearCountdownTooltip : t.clearButtonTooltip} 
          onclick={() => {
            if (clearCountdown !== null) {
              cancelClearCountdown();
              showNotice(t.autoClearCancelled);
            } else {
              clearAll();
            }
          }}
        >
          {#if clearCountdown !== null}
            {t.clearCountdownButton(clearCountdown)}
          {:else}
            {t.clearButton}
          {/if}
        </button>
      </div>
    </footer>
  {/if}
</main>

<style>
  :global(:root) {
    --bg-surface: rgba(18, 24, 38, 0.82);
    --border-color: rgba(255, 255, 255, 0.08);
    --text-main: #f8fafc;
    --text-muted: #94a3b8;
    --accent: #38bdf8;
    --card-bg: rgba(255, 255, 255, 0.05);
    --card-border: rgba(255, 255, 255, 0.08);
    --card-hover: rgba(56, 189, 248, 0.12);
  }

  :global([data-theme="light"]) {
    --bg-surface: rgba(245, 247, 250, 0.88);
    --border-color: rgba(0, 0, 0, 0.1);
    --text-main: #0f172a;
    --text-muted: #64748b;
    --accent: #0284c7;
    --card-bg: rgba(0, 0, 0, 0.03);
    --card-border: rgba(0, 0, 0, 0.06);
    --card-hover: rgba(2, 132, 199, 0.1);
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI Variable Text", "Segoe UI", sans-serif;
    user-select: none;
    background: transparent;
    overflow: hidden;
  }

  .shelf-wrapper {
    width: 340px;
    height: 420px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    padding: 12px;
    background: var(--bg-surface);
    backdrop-filter: blur(28px) saturate(190%);
    -webkit-backdrop-filter: blur(28px) saturate(190%);
    border: 1px solid var(--border-color);
    border-radius: 14px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.45);
    color: var(--text-main);
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .shelf-wrapper.drag-over {
    border-color: var(--accent);
    box-shadow: 0 0 20px rgba(56, 189, 248, 0.35);
  }

  .shelf-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-color);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .glow-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #64748b;
    transition: all 0.2s ease;
  }

  .glow-indicator.active {
    background: #10b981;
    box-shadow: 0 0 10px #10b981;
  }

  .app-title {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.3px;
  }

  .badge {
    font-size: 11px;
    font-weight: 600;
    padding: 2px 7px;
    border-radius: 10px;
    background: var(--accent);
    color: #ffffff;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    border-radius: 6px;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    background: var(--card-bg);
    color: var(--text-main);
  }

  .icon-btn.active {
    color: var(--accent);
  }

  .icon-btn.close:hover {
    color: #ef4444;
  }

  .lang-btn {
    font-family: inherit;
  }

  .lang-text {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.5px;
  }

  .settings-panel {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 8px;
    padding: 8px 10px;
    margin-top: 8px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    animation: fadeIn 0.15s ease-out;
  }

  .setting-item label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    cursor: pointer;
    color: var(--text-muted);
  }

  .setting-item label:hover {
    color: var(--text-main);
  }

  .setting-item input[type="checkbox"] {
    accent-color: var(--accent);
  }

  .toast-notice {
    background: var(--accent);
    color: #ffffff;
    font-size: 11px;
    padding: 4px 8px;
    border-radius: 6px;
    text-align: center;
    margin-top: 6px;
    animation: fadeIn 0.15s ease-out;
  }

  .shelf-body {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 8px 0;
  }

  .shelf-body::-webkit-scrollbar {
    width: 4px;
  }

  .shelf-body::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: 4px;
  }

  .empty-dropzone {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border: 1.5px dashed var(--border-color);
    border-radius: 10px;
    text-align: center;
    padding: 20px;
    box-sizing: border-box;
    transition: all 0.2s ease;
  }

  .empty-dropzone.active-hover {
    border-color: var(--accent);
    background: var(--card-hover);
  }

  .drop-icon {
    color: var(--text-muted);
    margin-bottom: 8px;
  }

  .drop-hint {
    margin: 0;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-main);
  }

  .sub-hint {
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-muted);
  }

  .items-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .item-card {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 8px;
    padding: 6px 8px;
    cursor: grab;
    transition: all 0.15s ease;
  }

  .item-card.selected {
    border-color: var(--accent);
    background: var(--card-hover);
  }

  .item-card:hover {
    border-color: var(--accent);
    background: var(--card-hover);
    transform: translateY(-1px);
  }

  .item-card:active {
    cursor: grabbing;
    opacity: 0.8;
  }

  .item-checkbox {
    width: 14px;
    height: 14px;
    border-radius: 4px;
    border: 1px solid var(--border-color);
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: bold;
    color: #ffffff;
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .item-checkbox.checked {
    background: var(--accent);
    border-color: var(--accent);
  }

  .master-drag-handle {
    margin: 8px 0 6px 0;
    padding: 8px 12px;
    border-radius: 8px;
    background: linear-gradient(135deg, rgba(56, 189, 248, 0.18), rgba(56, 189, 248, 0.08));
    border: 1px dashed var(--accent);
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: grab;
    user-select: none;
    transition: all 0.2s ease;
  }

  .master-drag-handle:hover {
    background: linear-gradient(135deg, rgba(56, 189, 248, 0.28), rgba(56, 189, 248, 0.15));
    border-style: solid;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(56, 189, 248, 0.2);
  }

  .master-drag-handle:active {
    cursor: grabbing;
    transform: scale(0.98);
  }

  .drag-icon-grip {
    color: var(--accent);
    display: flex;
    align-items: center;
  }

  .master-drag-title {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-main);
    flex: 1;
    margin-left: 8px;
  }

  .master-drag-badge {
    font-size: 10px;
    color: var(--accent);
    font-weight: 600;
    background: rgba(56, 189, 248, 0.15);
    padding: 2px 6px;
    border-radius: 4px;
  }

  .selection-control {
    display: flex;
    align-items: center;
  }

  .text-link-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
    text-underline-offset: 2px;
    transition: color 0.15s ease;
  }

  .text-link-btn:hover {
    color: var(--accent);
  }

  .action-btn.danger.countdown-active {
    background: #ef4444;
    color: #ffffff;
    border-color: #ef4444;
    font-weight: 600;
    animation: pulseClear 1s infinite alternate;
  }

  @keyframes pulseClear {
    0% {
      box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.4);
      transform: scale(1);
    }
    100% {
      box-shadow: 0 0 10px 3px rgba(239, 68, 68, 0.6);
      transform: scale(1.03);
    }
  }

  .item-type-icon {
    font-size: 16px;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .item-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .item-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-meta {
    font-size: 10px;
    color: var(--text-muted);
  }

  .item-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .item-action-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    border-radius: 4px;
    width: 22px;
    height: 22px;
    font-size: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    opacity: 0.7;
    transition: all 0.15s ease;
  }

  .item-action-btn:hover {
    opacity: 1;
    color: var(--accent);
    background: var(--card-hover);
  }

  .item-action-btn.remove:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.15);
  }

  .shelf-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 8px;
    border-top: 1px solid var(--border-color);
  }

  .footer-buttons {
    display: flex;
    gap: 6px;
  }

  .action-btn {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    color: var(--text-main);
    border-radius: 6px;
    padding: 4px 8px;
    font-size: 11px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  .action-btn.danger:hover {
    border-color: #ef4444;
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>