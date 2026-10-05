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

  interface AppConfig {
    shake: boolean;
    hotkey: boolean;
    hotkey_combo: number;
    double_tap: boolean;
    double_tap_key: number;
    edge_dock: boolean;
    theme: ThemeMode;
    lang: Lang;
    auto_clear: boolean;
    auto_check_updates?: boolean;
  }

  interface UpdateCheckResult {
    has_update: boolean;
    current_version: string;
    latest_version: string;
    release_url: string;
    setup_url: string | null;
    portable_url: string | null;
    release_notes: string;
    published_at: string;
  }

  // --- Svelte 5 Runes ($state) ---
  let items = $state<StashItem[]>([]);
  let isDragOver = $state(false);
  let theme = $state<ThemeMode>("system");
  let effectiveTheme = $state<"dark" | "light">("dark");
  let lang = $state<Lang>("ru");
  let showSettings = $state(false);
  let activeSettingsTab = $state<"triggers" | "about">("triggers");
  let shakeEnabled = $state<boolean>(true);
  let hotkeyEnabled = $state<boolean>(true);
  let hotkeyCombo = $state<number>(0);
  let doubleTapEnabled = $state<boolean>(true);
  let doubleTapKey = $state<number>(0);
  let edgeDockEnabled = $state<boolean>(false);
  let autostartEnabled = $state(false);
  let autoCheckUpdates = $state(true);
  let isCheckingUpdates = $state(false);
  let updateError = $state<string | null>(null);
  let updateResult = $state<UpdateCheckResult | null>(null);
  let statusNotice = $state<string | null>(null);
  let edgeCollapseTimer: number | null = null;

  let autoClearEnabled = $state<boolean>(true);
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

  async function loadAppSettings() {
    try {
      const cfg = await invoke<AppConfig>("get_app_config");
      shakeEnabled = cfg.shake;
      hotkeyEnabled = cfg.hotkey;
      hotkeyCombo = cfg.hotkey_combo;
      doubleTapEnabled = cfg.double_tap;
      doubleTapKey = cfg.double_tap_key;
      edgeDockEnabled = cfg.edge_dock;
      theme = (cfg.theme as ThemeMode) || "system";
      lang = (cfg.lang as Lang) || "ru";
      autoClearEnabled = cfg.auto_clear;
      autoCheckUpdates = cfg.auto_check_updates ?? true;
      updateTheme();
    } catch (e) {
      console.error("Ошибка загрузки настроек:", e);
    }
  }

  async function saveAllSettings() {
    try {
      await invoke("save_app_config", {
        config: {
          shake: shakeEnabled,
          hotkey: hotkeyEnabled,
          hotkey_combo: Number(hotkeyCombo),
          double_tap: doubleTapEnabled,
          double_tap_key: Number(doubleTapKey),
          edge_dock: edgeDockEnabled,
          theme,
          lang,
          auto_clear: autoClearEnabled,
          auto_check_updates: autoCheckUpdates,
        },
      });
    } catch (e) {
      console.error("Ошибка сохранения настроек:", e);
    }
  }

  async function handleCheckUpdates(force: boolean = true) {
    isCheckingUpdates = true;
    updateError = null;
    try {
      const res = await invoke<UpdateCheckResult>("check_for_updates", { force });
      updateResult = res;
    } catch (err) {
      updateError = String(err);
    } finally {
      isCheckingUpdates = false;
    }
  }

  async function handleOpenUrl(url: string | null | undefined) {
    if (!url) return;
    try {
      await invoke("open_external_url", { url });
    } catch (err) {
      console.error("Ошибка открытия ссылки:", err);
    }
  }

  function toggleLang() {
    lang = lang === "ru" ? "en" : "ru";
    saveAllSettings();
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
    if (!autoClearEnabled) {
      cancelClearCountdown();
    }
    saveAllSettings();
  }

  function toggleAutoCheckSetting() {
    autoCheckUpdates = !autoCheckUpdates;
    saveAllSettings();
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
  }

  function cycleTheme() {
    if (theme === "system") theme = "dark";
    else if (theme === "dark") theme = "light";
    else theme = "system";
    updateTheme();
    saveAllSettings();
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

  async function exitApp() {
    try {
      await invoke("exit_app");
    } catch (e) {
      console.error("Ошибка закрытия StashIt:", e);
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
    localStorage.setItem("stashit_trigger_shake", String(shakeEnabled));
    localStorage.setItem("stashit_trigger_hotkey", String(hotkeyEnabled));
    localStorage.setItem("stashit_hotkey_combo", String(hotkeyCombo));
    localStorage.setItem("stashit_trigger_double_tap", String(doubleTapEnabled));
    localStorage.setItem("stashit_double_tap_key", String(doubleTapKey));
    localStorage.setItem("stashit_trigger_edge_dock", String(edgeDockEnabled));

    try {
      await invoke("set_trigger_mode", {
        shake: shakeEnabled,
        hotkey: hotkeyEnabled,
        hotkeyCombo: Number(hotkeyCombo),
        doubleTap: doubleTapEnabled,
        doubleTapKey: Number(doubleTapKey),
        edgeDock: edgeDockEnabled,
      });
    } catch (e) {
      console.error("Ошибка обновления триггеров:", e);
    }
  }

  function handleMouseLeaveWrapper() {
    if (edgeDockEnabled && itemsCount === 0 && !showSettings) {
      if (edgeCollapseTimer !== null) clearTimeout(edgeCollapseTimer);
      edgeCollapseTimer = window.setTimeout(() => {
        hideShelf();
        edgeCollapseTimer = null;
      }, 350);
    }
  }

  function handleMouseEnterWrapper() {
    if (edgeCollapseTimer !== null) {
      clearTimeout(edgeCollapseTimer);
      edgeCollapseTimer = null;
    }
  }

  async function copyAllItems() {
    const targetItems = selectedIds.size > 0
      ? items.filter((it) => selectedIds.has(String(it.id)))
      : items;

    if (targetItems.length === 0) return;

    // Пути к реальным файлам
    const filePaths = targetItems
      .filter((it) => it.path)
      .map((it) => it.path as string);

    // Для текстовых карточек и URL создаем временные файлы, чтобы Проводник мог вставить их как файлы
    const tempPaths: string[] = [];
    for (const it of targetItems.filter((it) => !it.path)) {
      try {
        const tempPath = await invoke<string>("create_temp_file", {
          name: it.name,
          content: it.text_preview ?? "",
          isUrl: it.kind === "url",
        });
        tempPaths.push(tempPath);
      } catch (err) {
        console.error("Ошибка создания temp файла для буфера:", err);
      }
    }

    const allPaths = [...filePaths, ...tempPaths];
    const allText = targetItems
      .map((it) => it.text_preview || it.path || "")
      .filter(Boolean)
      .join("\n");

    try {
      await invoke("copy_to_clipboard", {
        paths: allPaths,
        text: allText || null,
      });
      showNotice(t.copiedFiles(allPaths.length));
    } catch (err) {
      console.error("Ошибка нативного копирования:", err);
      if (allText) {
        await navigator.clipboard.writeText(allText);
        showNotice(t.copiedText);
      }
    }
  }

  async function copySingleItem(item: StashItem, e: MouseEvent) {
    e.stopPropagation();
    let itemPath = item.path;
    const text = item.text_preview || item.path || "";

    // Если это текст/URL, создаём временный файл для возможности вставки в Проводник
    if (!itemPath && text) {
      try {
        itemPath = await invoke<string>("create_temp_file", {
          name: item.name,
          content: item.text_preview ?? "",
          isUrl: item.kind === "url",
        });
      } catch (err) {
        console.error("Ошибка создания temp файла для карточки:", err);
      }
    }

    const paths = itemPath ? [itemPath] : [];

    try {
      await invoke("copy_to_clipboard", {
        paths,
        text: text || null,
      });
      showNotice(item.path ? t.fileCopied : t.textCopied);
    } catch (err) {
      console.error("Ошибка копирования карточки:", err);
      if (text) {
        await navigator.clipboard.writeText(text);
        showNotice(t.textCopied);
      }
    }
  }

  function showNotice(msg: string) {
    statusNotice = msg;
    setTimeout(() => {
      if (statusNotice === msg) statusNotice = null;
    }, 2000);
  }

  // Нативный OLE Drag Out одного элемента
  async function handleItemDragStart(e: DragEvent, item: StashItem) {
    cancelClearCountdown();
    e.preventDefault();
    if (item.path) {
      // Файлы, папки, изображения — нативный OLE drag
      try {
        await invoke("drag_item", { paths: [item.path] });
      } catch (err) {
        console.error("Ошибка native drag-out:", err);
      }
    } else {
      // Текст / URL — нативный IDataObject с CF_HDROP + CF_UNICODETEXT:
      // Проводник видит файл, редакторы/мессенджеры вставляют текст
      try {
        await invoke("drag_text_item", {
          name: item.name,
          content: item.text_preview ?? "",
          isUrl: item.kind === "url",
        });
      } catch (err) {
        console.error("Ошибка drag text/url:", err);
      }
    }
  }

  // Нативный OLE Drag Out всей стопки (всех выделенных или всех файлов кармана)
  async function handleDragAll(e: DragEvent) {
    cancelClearCountdown();
    e.preventDefault();

    const targetItems = selectedIds.size > 0
      ? items.filter((it) => selectedIds.has(String(it.id)))
      : items;

    // Пути к реальным файлам
    const filePaths = targetItems
      .filter((it) => it.path)
      .map((it) => it.path as string);

    // Текстовые / URL элементы — создаём временные файлы
    const tempPaths: string[] = [];
    for (const it of targetItems.filter((it) => !it.path)) {
      try {
        const tempPath = await invoke<string>("create_temp_file", {
          name: it.name,
          content: it.text_preview ?? "",
          isUrl: it.kind === "url",
        });
        tempPaths.push(tempPath);
      } catch (err) {
        console.error("Ошибка создания временного файла:", err);
      }
    }

    const allPaths = [...filePaths, ...tempPaths];
    if (allPaths.length > 0) {
      try {
        await invoke("drag_item", { paths: allPaths });
      } catch (err) {
        console.error("Ошибка drag all:", err);
      }
    }
  }

  onMount(() => {
    loadAppSettings();
    loadItems().then(() => syncSelectionWithItems());

    // Инициализируем приём файлов OLE Drop Target на дочерних окнах WebView2
    setTimeout(() => {
      invoke("init_drop_target").catch(console.error);
    }, 150);

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
      // Больше не прячем карман внезапно при отпускании ЛКМ,
      // позволяя спокойно перетащить или сбросить файл.
    });

    const unlistenAbout = listen("open-about-tab", () => {
      showSettings = true;
      activeSettingsTab = "about";
      if (!updateResult && !isCheckingUpdates) {
        handleCheckUpdates(false);
      }
    });

    const unlistenUpdateStatus = listen<UpdateCheckResult>("update-status", (event) => {
      updateResult = event.payload;
    });

    // Хоткей Escape для закрытия настроек или скрытия кармана, Ctrl+A для выделения всех
    const keyHandler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        if (showSettings) {
          showSettings = false;
        } else {
          hideShelf();
        }
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
      if (edgeCollapseTimer !== null) {
        clearTimeout(edgeCollapseTimer);
        edgeCollapseTimer = null;
      }
      unlistenUpdated.then((f) => f());
      unlistenCleared.then((f) => f());
      unlistenDragCompleted.then((f) => f());
      unlistenEnter.then((f) => f());
      unlistenLeave.then((f) => f());
      unlistenMouseUp.then((f) => f());
      unlistenAbout.then((f) => f());
      unlistenUpdateStatus.then((f) => f());
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
  onmouseleave={handleMouseLeaveWrapper}
  onmouseenter={handleMouseEnterWrapper}
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

      <!-- Свернуть в трей -->
      <button class="icon-btn minimize-btn" title={t.minimizeTooltip} onclick={hideShelf}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="5" y1="12" x2="19" y2="12"/></svg>
      </button>

      <!-- Выход из программы -->
      <button class="icon-btn close" title={t.quitTooltip} onclick={exitApp}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      </button>
    </div>
  </header>

  <!-- Уведомление о статусе -->
  {#if statusNotice}
    <div class="toast-notice">
      {statusNotice}
    </div>
  {/if}

  <!-- Панель настроек (с вкладками «Управление» и «О программе») -->
  {#if showSettings}
    <div class="settings-panel">
      <!-- Навигация по вкладкам -->
      <div class="settings-tabs">
        <button 
          class="tab-btn" 
          class:active={activeSettingsTab === "triggers"} 
          onclick={() => (activeSettingsTab = "triggers")}
        >
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
          <span>{t.tabSettings}</span>
        </button>
        <button 
          class="tab-btn" 
          class:active={activeSettingsTab === "about"} 
          onclick={() => {
            activeSettingsTab = "about";
            if (!updateResult && !isCheckingUpdates) {
              handleCheckUpdates(false);
            }
          }}
        >
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>
          <span>{t.tabAbout}</span>
          {#if updateResult?.has_update}
            <span class="update-badge-dot" title={t.updateAvailable}></span>
          {/if}
        </button>
      </div>

      {#if activeSettingsTab === "triggers"}
        <div class="tab-content">
          <div class="settings-title">{t.triggerSection}</div>

          <!-- 1. Встряхивание мыши (Shake) -->
          <div class="setting-item">
            <label>
              <input 
                type="checkbox" 
                checked={shakeEnabled} 
                onchange={(e) => {
                  shakeEnabled = e.currentTarget.checked;
                  saveAllSettings();
                }}
              />
              <span>{t.shakeTrigger}</span>
            </label>
          </div>

          <!-- 2. Глобальный хоткей -->
          <div class="setting-item with-select">
            <label>
              <input 
                type="checkbox" 
                checked={hotkeyEnabled} 
                onchange={(e) => {
                  hotkeyEnabled = e.currentTarget.checked;
                  saveAllSettings();
                }}
              />
              <span>{t.hotkeyTrigger}</span>
            </label>
            {#if hotkeyEnabled}
              <select 
                class="setting-select"
                value={hotkeyCombo} 
                onchange={(e) => {
                  hotkeyCombo = Number(e.currentTarget.value);
                  saveAllSettings();
                }}
              >
                {#each t.hotkeyOptions as opt}
                  <option value={opt.id}>{opt.label}</option>
                {/each}
              </select>
            {/if}
          </div>

          <!-- 3. Двойное нажатие (Double-tap) -->
          <div class="setting-item with-select">
            <label>
              <input 
                type="checkbox" 
                checked={doubleTapEnabled} 
                onchange={(e) => {
                  doubleTapEnabled = e.currentTarget.checked;
                  saveAllSettings();
                }}
              />
              <span>{t.doubleTapTrigger}</span>
            </label>
            {#if doubleTapEnabled}
              <select 
                class="setting-select"
                value={doubleTapKey} 
                onchange={(e) => {
                  doubleTapKey = Number(e.currentTarget.value);
                  saveAllSettings();
                }}
              >
                {#each t.doubleTapOptions as opt}
                  <option value={opt.id}>{opt.label}</option>
                {/each}
              </select>
            {/if}
          </div>

          <!-- 4. Прилипание к краю экрана (Edge Dock) -->
          <div class="setting-item">
            <label>
              <input 
                type="checkbox" 
                checked={edgeDockEnabled} 
                onchange={(e) => {
                  edgeDockEnabled = e.currentTarget.checked;
                  saveAllSettings();
                }}
              />
              <span>{t.edgeDockTrigger}</span>
            </label>
          </div>

          <div class="settings-divider"></div>

          <!-- Системные настройки -->
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

          <!-- Кнопка полного выхода из приложения -->
          <div class="setting-item quit-setting-item">
            <button class="quit-action-btn" onclick={exitApp}>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg>
              <span>{t.exitApp}</span>
            </button>
          </div>
        </div>
      {:else if activeSettingsTab === "about"}
        <div class="tab-content about-tab-content">
          <!-- Заголовок / Бренд -->
          <div class="about-hero">
            <div class="about-logo-wrapper">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
            </div>
            <div class="about-meta">
              <div class="about-title-row">
                <span class="about-app-name">StashIt</span>
                <span class="version-tag">{updateResult?.current_version ? `v${updateResult.current_version}` : t.versionBadge}</span>
              </div>
              <p class="about-desc">{t.aboutTagline}</p>
            </div>
          </div>

          <!-- Блок обновлений -->
          <div class="updater-card">
            <div class="updater-status-header">
              <div class="status-indicator-box">
                {#if isCheckingUpdates}
                  <span class="status-spinner"></span>
                  <span class="status-text">{t.checkingUpdates}</span>
                {:else if updateError}
                  <span class="status-dot error"></span>
                  <span class="status-text error">{t.updateCheckFailed}</span>
                {:else if updateResult?.has_update}
                  <span class="status-dot update"></span>
                  <span class="status-text update">{t.updateAvailable} <strong class="new-ver">v{updateResult.latest_version}</strong></span>
                {:else if updateResult && !updateResult.has_update}
                  <span class="status-dot ok"></span>
                  <span class="status-text ok">{t.updateLatest}</span>
                {:else}
                  <span class="status-dot idle"></span>
                  <span class="status-text">{t.versionBadge}</span>
                {/if}
              </div>

              <button 
                class="check-now-btn" 
                disabled={isCheckingUpdates}
                onclick={() => handleCheckUpdates(true)}
                title={t.checkUpdates}
              >
                <svg class:spin={isCheckingUpdates} width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
                <span>{t.checkUpdates}</span>
              </button>
            </div>

            {#if updateResult?.has_update}
              <div class="updater-actions">
                {#if updateResult.setup_url}
                  <button 
                    class="action-download-btn primary" 
                    onclick={() => handleOpenUrl(updateResult?.setup_url)}
                  >
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                    <span>{t.downloadInstaller}</span>
                  </button>
                {/if}
                {#if updateResult.portable_url}
                  <button 
                    class="action-download-btn" 
                    onclick={() => handleOpenUrl(updateResult?.portable_url)}
                  >
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 3v18"/></svg>
                    <span>{t.downloadPortable}</span>
                  </button>
                {/if}
                <button 
                  class="action-download-btn outline" 
                  onclick={() => handleOpenUrl(updateResult?.release_url)}
                >
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
                  <span>{t.viewReleaseNotes}</span>
                </button>
              </div>
            {/if}

            <div class="auto-check-row">
              <label>
                <input 
                  type="checkbox" 
                  checked={autoCheckUpdates} 
                  onchange={toggleAutoCheckSetting}
                />
                <div class="auto-check-texts">
                  <span class="label-title">{t.autoCheckUpdates}</span>
                  <span class="label-desc">{t.autoCheckUpdatesDesc}</span>
                </div>
              </label>
            </div>
          </div>

          <!-- Ссылки экосистемы -->
          <div class="about-links-section">
            <div class="standard-badge">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
              <span>{t.authorEco}</span>
            </div>

            <div class="about-links-grid">
              <button 
                class="eco-link-btn" 
                onclick={() => handleOpenUrl("https://github.com/kobaltgit/StashIt")}
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/></svg>
                <span>{t.linkGithub}</span>
              </button>

              <button 
                class="eco-link-btn" 
                onclick={() => handleOpenUrl("https://github.com/kobaltgit/StashIt/issues")}
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
                <span>{t.linkIssues}</span>
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {:else}
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
  {/if}

  <!-- Нижний футер со сводкой, захватом всей пачки и групповыми действиями -->
  {#if !showSettings && itemsCount > 0}
    <!-- Главная плашка перетаскивания всей пачки сразу -->
    <div 
      class="master-drag-handle"
      role="button"
      tabindex="0"
      draggable="true"
      ondragstart={handleDragAll}
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
    gap: 8px;
    animation: fadeIn 0.15s ease-out;
    flex: 1;
    overflow-y: auto;
  }

  .settings-panel::-webkit-scrollbar {
    width: 4px;
  }

  .settings-panel::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: 4px;
  }

  /* Табы */
  .settings-tabs {
    display: flex;
    gap: 4px;
    padding-bottom: 6px;
    border-bottom: 1px solid var(--border-color);
  }

  .tab-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 5px 8px;
    border-radius: 6px;
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    position: relative;
  }

  .tab-btn:hover {
    background: var(--card-hover);
    color: var(--text-main);
  }

  .tab-btn.active {
    background: var(--card-hover);
    color: var(--accent);
    border-color: rgba(56, 189, 248, 0.3);
  }

  .update-badge-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 6px #10b981;
    animation: pulse 1.5s infinite;
  }

  .tab-content {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }

  .about-tab-content {
    gap: 10px;
  }

  /* Раздел «О программе» */
  .about-hero {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 4px 2px;
  }

  .about-logo-wrapper {
    width: 38px;
    height: 38px;
    border-radius: 9px;
    background: linear-gradient(135deg, rgba(56, 189, 248, 0.15), rgba(16, 185, 129, 0.15));
    border: 1px solid rgba(56, 189, 248, 0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    flex-shrink: 0;
  }

  .about-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .about-title-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .about-app-name {
    font-size: 13.5px;
    font-weight: 700;
    color: var(--text-main);
  }

  .version-tag {
    font-size: 9.5px;
    font-weight: 700;
    padding: 1px 5px;
    border-radius: 5px;
    background: rgba(56, 189, 248, 0.15);
    color: var(--accent);
    border: 1px solid rgba(56, 189, 248, 0.25);
  }

  .about-desc {
    margin: 0;
    font-size: 10px;
    color: var(--text-muted);
    line-height: 1.3;
  }

  /* Карточка проверки обновлений */
  .updater-card {
    background: rgba(0, 0, 0, 0.15);
    border: 1px solid var(--card-border);
    border-radius: 8px;
    padding: 8px 10px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  :global([data-theme="light"]) .updater-card {
    background: rgba(255, 255, 255, 0.5);
  }

  .updater-status-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
  }

  .status-indicator-box {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 10.5px;
    min-width: 0;
    flex: 1;
  }

  .status-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text-muted);
  }

  .status-text.update {
    color: #10b981;
    font-weight: 600;
  }

  .status-text.error {
    color: #ef4444;
  }

  .status-text.ok {
    color: var(--text-muted);
  }

  .new-ver {
    color: #10b981;
  }

  .status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-dot.idle {
    background: #64748b;
  }

  .status-dot.ok {
    background: #10b981;
  }

  .status-dot.update {
    background: #38bdf8;
    box-shadow: 0 0 8px #38bdf8;
    animation: pulse 1.5s infinite;
  }

  .status-dot.error {
    background: #ef4444;
  }

  .status-spinner {
    width: 10px;
    height: 10px;
    border: 2px solid rgba(56, 189, 248, 0.2);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }

  .check-now-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 3px 7px;
    border-radius: 5px;
    font-size: 10px;
    font-weight: 600;
    background: var(--card-hover);
    color: var(--text-main);
    border: 1px solid var(--border-color);
    cursor: pointer;
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .check-now-btn:hover:not(:disabled) {
    border-color: var(--accent);
    color: var(--accent);
  }

  .check-now-btn:disabled {
    opacity: 0.6;
    cursor: default;
  }

  .updater-actions {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
    margin-top: 2px;
  }

  .action-download-btn {
    flex: 1;
    min-width: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    padding: 5px 8px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    border: 1px solid var(--border-color);
    background: var(--card-hover);
    color: var(--text-main);
  }

  .action-download-btn.primary {
    background: #0284c7;
    color: #ffffff;
    border-color: #0284c7;
  }

  .action-download-btn.primary:hover {
    background: #0369a1;
  }

  .action-download-btn.outline {
    background: transparent;
    color: var(--text-muted);
  }

  .action-download-btn.outline:hover {
    color: var(--text-main);
    border-color: var(--text-muted);
  }

  .auto-check-row {
    padding-top: 6px;
    border-top: 1px solid var(--border-color);
  }

  .auto-check-row label {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    cursor: pointer;
  }

  .auto-check-texts {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .label-title {
    font-size: 10.5px;
    color: var(--text-main);
  }

  .label-desc {
    font-size: 9px;
    color: var(--text-muted);
  }

  /* Ссылки экосистемы Kobalt */
  .about-links-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: auto;
    padding-top: 4px;
  }

  .standard-badge {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 9.5px;
    font-weight: 600;
    color: var(--accent);
    opacity: 0.9;
  }

  .about-links-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }

  .eco-link-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 6px;
    border-radius: 6px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    color: var(--text-muted);
    font-size: 10px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .eco-link-btn:hover {
    background: var(--card-hover);
    color: var(--text-main);
    border-color: var(--border-color);
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.85); }
  }

  .spin {
    animation: spin 0.8s linear infinite;
  }

  .settings-title {
    font-size: 9.5px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.6px;
    color: var(--accent);
    margin-bottom: 2px;
  }

  .settings-divider {
    height: 1px;
    background: var(--border-color);
    margin: 4px 0;
  }

  .setting-item.with-select {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
  }

  .setting-select {
    background: var(--card-hover);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-main);
    font-size: 10px;
    padding: 2px 4px;
    outline: none;
    cursor: pointer;
    max-width: 135px;
  }

  .setting-select option {
    background: #1e293b;
    color: #f1f5f9;
  }

  .setting-item label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    cursor: pointer;
    color: var(--text-muted);
    flex: 1;
  }

  .setting-item label:hover {
    color: var(--text-main);
  }

  .setting-item input[type="checkbox"] {
    accent-color: var(--accent);
  }

  .quit-setting-item {
    margin-top: 4px;
    padding-top: 4px;
    border-top: 1px solid var(--border-color);
  }

  .quit-action-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 6px 10px;
    background: rgba(239, 68, 68, 0.12);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s ease, color 0.15s ease;
  }

  .quit-action-btn:hover {
    background: rgba(239, 68, 68, 0.22);
    color: #dc2626;
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