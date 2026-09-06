export type Lang = 'ru' | 'en';

export const translations = {
  ru: {
    // Header & Tooltips
    appTitle: 'StashIt',
    themeTooltip: (t: string) => `Тема: ${t === 'system' ? 'Системная' : t === 'dark' ? 'Тёмная' : 'Светлая'}`,
    langTooltip: 'Сменить язык (RU/EN)',
    settingsTooltip: 'Настройки вызова и автостарта',
    closeTooltip: 'Свернуть (Esc)',

    // Settings
    shakeTrigger: 'Встряска мыши (Shake)',
    autoDragTrigger: 'При начале перетаскивания (Эксп.)',
    autostart: 'Автозапуск с Windows (HKCU)',
    autoClear: 'Автоочистка по таймеру (5с)',

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
    subHint: 'Встряхните мышь или перетащите файл',
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

    // Settings
    shakeTrigger: 'Mouse Shake to Show',
    autoDragTrigger: 'Auto on Drag (Experimental)',
    autostart: 'Start with Windows (HKCU)',
    autoClear: 'Auto-clear timer (5s)',

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
    subHint: 'Shake mouse or drag content',
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
