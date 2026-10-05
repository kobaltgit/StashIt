export type Lang = 'ru' | 'en';

export const translations = {
  ru: {
    // Header & Tooltips
    appTitle: 'StashIt',
    themeTooltip: (t: string) => `Тема: ${t === 'system' ? 'Системная' : t === 'dark' ? 'Тёмная' : 'Светлая'}`,
    langTooltip: 'Сменить язык (RU/EN)',
    settingsTooltip: 'Настройки вызова и автостарта',
    closeTooltip: 'Свернуть (Esc)',
    minimizeTooltip: 'Свернуть в трей (Esc)',
    quitTooltip: 'Завершить работу StashIt',
    exitApp: 'Завершить работу StashIt (Выход)',

    // Tabs
    tabSettings: 'Управление',
    tabAbout: 'О программе',

    // About & Updates
    aboutTagline: 'Легковесный плавающий карман для Windows',
    versionBadge: 'v1.2.0',
    checkingUpdates: 'Проверка...',
    updateAvailable: 'Доступно обновление',
    updateLatest: 'У вас актуальная версия',
    updateCheckFailed: 'Ошибка проверки',
    checkUpdates: 'Проверить',
    downloadInstaller: 'Установщик',
    downloadPortable: 'Portable',
    viewReleaseNotes: 'Список изменений',
    autoCheckUpdates: 'Проверять обновления автоматически',
    autoCheckUpdatesDesc: 'Тихая проверка раз в неделю',
    linkWebsite: 'Сайт проекта',
    linkGithub: 'GitHub репозиторий',
    linkIssues: 'Сообщить об ошибке',
    authorEco: 'Kobalt Tools Standard',

    // Settings — Triggers
    triggerSection: 'Способы вызова кармана',
    shakeTrigger: 'Встряхивание мыши (с зажатым ЛКМ)',
    hotkeyTrigger: 'Глобальный хоткей',
    doubleTapTrigger: 'Двойное нажатие (Double-tap)',
    edgeDockTrigger: 'Край экрана (Edge Dock / Шторка)',
    autostart: 'Автозапуск с Windows (HKCU)',
    autoClear: 'Автоочистка по таймеру (5с)',

    // Shortcuts labels
    hotkeyOptions: [
      { id: 0, label: 'Ctrl + Shift + Space' },
      { id: 1, label: 'Alt + S' },
      { id: 2, label: 'Ctrl + Alt + S' },
      { id: 3, label: 'Win + Alt + S' },
      { id: 4, label: 'Ctrl + Shift + X' },
    ],
    doubleTapOptions: [
      { id: 0, label: '2× Ctrl (быстро)' },
      { id: 1, label: '2× Shift (быстро)' },
    ],

    // Notices
    autostartEnabled: 'Автозапуск включен',
    autostartDisabled: 'Автозапуск выключен',
    pocketCleared: 'Карман очищен',
    autoClearCancelled: 'Автоочистка отменена',
    copiedFiles: (n: number) => `Скопировано ${n} файл(ов)`,
    copiedText: 'Скопировано как текст',
    fileCopied: 'Файл скопирован',
    textCopied: 'Скопировано в буфер',

    // Dropzone & Items
    dropHint: 'Сбросьте файлы сюда',
    subHint: 'Встряхните мышь, нажмите хоткей, 2×Ctrl или поднесите к краю',
    itemTitle: 'Клик: выделить/снять. Зажмите и перетащите в папку',
    copyFile: 'Копировать файл (в буфер обмена)',
    removeItem: 'Удалить из кармана',

    // Batch Drag & Footer
    dragSelected: (sel: number) => `Перетащить выбранные (${sel})`,
    dragAll: (all: number) => `Перетащить всё (${all})`,
    dragHint: 'Зажмите и перетащите в целевую папку ВСЕ файлы сразу',
    deselectAll: 'Снять всё',
    selectAll: 'Выбрать все',
    copyButton: 'Копировать',
    clearButton: 'Очистить',
    clearCountdownButton: (s: number) => `Очистить (${s}с)`,
    clearButtonTooltip: 'Очистить весь карман',
    clearCountdownTooltip: 'Нажмите, чтобы отменить автоочистку или очистить сейчас',
  },
  en: {
    // Header & Tooltips
    appTitle: 'StashIt',
    themeTooltip: (t: string) => `Theme: ${t === 'system' ? 'System' : t === 'dark' ? 'Dark' : 'Light'}`,
    langTooltip: 'Switch Language (RU/EN)',
    settingsTooltip: 'Trigger & Autostart settings',
    closeTooltip: 'Hide (Esc)',
    minimizeTooltip: 'Minimize to tray (Esc)',
    quitTooltip: 'Quit StashIt completely',
    exitApp: 'Quit StashIt Application',

    // Tabs
    tabSettings: 'Controls',
    tabAbout: 'About',

    // About & Updates
    aboutTagline: 'Lightweight floating file shelf for Windows',
    versionBadge: 'v1.2.0',
    checkingUpdates: 'Checking...',
    updateAvailable: 'Update available',
    updateLatest: 'You have the latest version',
    updateCheckFailed: 'Check failed',
    checkUpdates: 'Check',
    downloadInstaller: 'Setup',
    downloadPortable: 'Portable',
    viewReleaseNotes: 'Release Notes',
    autoCheckUpdates: 'Auto-check for updates',
    autoCheckUpdatesDesc: 'Silent check once a week',
    linkWebsite: 'Official Website',
    linkGithub: 'GitHub Repository',
    linkIssues: 'Report an Issue',
    authorEco: 'Kobalt Tools Standard',

    // Settings — Triggers
    triggerSection: 'Shelf Activation Methods',
    shakeTrigger: 'Mouse Shake (while dragging)',
    hotkeyTrigger: 'Global Hotkey',
    doubleTapTrigger: 'Double-tap modifier key',
    edgeDockTrigger: 'Screen Edge Dock (Auto-expand)',
    autostart: 'Start with Windows (HKCU)',
    autoClear: 'Auto-clear timer (5s)',

    // Shortcuts labels
    hotkeyOptions: [
      { id: 0, label: 'Ctrl + Shift + Space' },
      { id: 1, label: 'Alt + S' },
      { id: 2, label: 'Ctrl + Alt + S' },
      { id: 3, label: 'Win + Alt + S' },
      { id: 4, label: 'Ctrl + Shift + X' },
    ],
    doubleTapOptions: [
      { id: 0, label: '2× Ctrl (quick)' },
      { id: 1, label: '2× Shift (quick)' },
    ],

    // Notices
    autostartEnabled: 'Autostart enabled',
    autostartDisabled: 'Autostart disabled',
    pocketCleared: 'Shelf cleared',
    autoClearCancelled: 'Auto-clear cancelled',
    copiedFiles: (n: number) => `Copied ${n} file(s)`,
    copiedText: 'Copied as text',
    fileCopied: 'File copied',
    textCopied: 'Copied to clipboard',

    // Dropzone & Items
    dropHint: 'Drop files here',
    subHint: 'Shake mouse, press hotkey, 2×Ctrl, or hover screen edge',
    itemTitle: 'Click: toggle selection. Drag to target folder',
    copyFile: 'Copy file (to clipboard)',
    removeItem: 'Remove from shelf',

    // Batch Drag & Footer
    dragSelected: (sel: number) => `Drag selected (${sel})`,
    dragAll: (all: number) => `Drag all (${all})`,
    dragHint: 'Click and drag ALL files into destination at once',
    deselectAll: 'Deselect all',
    selectAll: 'Select all',
    copyButton: 'Copy',
    clearButton: 'Clear',
    clearCountdownButton: (s: number) => `Clear (${s}s)`,
    clearButtonTooltip: 'Clear entire shelf',
    clearCountdownTooltip: 'Click to cancel auto-clear or clear right now',
  }
};
