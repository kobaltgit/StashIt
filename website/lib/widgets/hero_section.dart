import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme.dart';
import '../i18n.dart';

class HeroSection extends StatefulWidget {
  const HeroSection({super.key});

  @override
  State<HeroSection> createState() => _HeroSectionState();
}

class _ShelfItem {
  final String id;
  final String icon;
  final String name;
  final String meta;
  bool isSelected;

  _ShelfItem({
    required this.id,
    required this.icon,
    required this.name,
    required this.meta,
  }) : isSelected = true;
}

class _HeroSectionState extends State<HeroSection> {
  bool _isDarkTheme = true;
  bool _isDraggingOver = false;

  final List<_ShelfItem> _items = [
    _ShelfItem(id: '1', icon: '🖼️', name: 'presentation_mockup.png', meta: '2.4 МБ • .png'),
    _ShelfItem(id: '2', icon: '📄', name: 'project_specification.pdf', meta: '840 КБ • .pdf'),
    _ShelfItem(id: '3', icon: '📁', name: 'assets_bundle', meta: 'Папка • 12 файлов'),
  ];

  void _addItem() {
    final sampleItems = [
      _ShelfItem(id: UniqueKey().toString(), icon: '🌐', name: 'https://github.com/kobaltgit/StashIt', meta: 'Ссылка • web'),
      _ShelfItem(id: UniqueKey().toString(), icon: '📝', name: 'meeting_notes.txt', meta: '14 КБ • .txt'),
      _ShelfItem(id: UniqueKey().toString(), icon: '📊', name: 'quarterly_report.xlsx', meta: '1.8 МБ • .xlsx'),
      _ShelfItem(id: UniqueKey().toString(), icon: '🎨', name: 'app_icon_fluent.svg', meta: '45 КБ • .svg'),
    ];
    setState(() {
      _items.add(sampleItems[_items.length % sampleItems.length]);
    });
  }

  void _clearItems() {
    setState(() {
      _items.clear();
    });
  }

  void _toggleSelectAll() {
    final allSelected = _items.isNotEmpty && _items.every((it) => it.isSelected);
    setState(() {
      for (final it in _items) {
        it.isSelected = !allSelected;
      }
    });
  }

  void _removeItem(String id) {
    setState(() {
      _items.removeWhere((it) => it.id == id);
    });
  }

  @override
  Widget build(BuildContext context) {
    final bool isDark = _isDarkTheme;
    final Color shelfBg = isDark ? const Color(0xE6141B2D) : const Color(0xF2F8FAFC);
    final Color shelfBorder = _isDraggingOver
        ? const Color(0xFF0078D6)
        : (isDark ? const Color(0x3338BDF8) : const Color(0x330078D6));
    final Color textColor = isDark ? Colors.white : const Color(0xFF0F172A);
    final Color textSecondaryColor = isDark ? const Color(0xFF94A3B8) : const Color(0xFF64748B);
    final int selectedCount = _items.where((i) => i.isSelected).length;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: Column(
        children: [
          // Badge
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6),
            decoration: BoxDecoration(
              color: AppColors.surfaceCard,
              borderRadius: BorderRadius.circular(20),
              border: Border.all(color: AppColors.borderSubtle),
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                ClipRRect(
                  borderRadius: BorderRadius.circular(4),
                  child: Image.asset('assets/icon.png', width: 16, height: 16),
                ),
                const SizedBox(width: 8),
                Text(
                  Strings.get('Экосистема Kobalt Tools • Windows 10/11', 'Kobalt Tools Ecosystem • Windows 10/11'),
                  style: TextStyle(fontSize: 12, color: AppColors.textSecondary),
                ),
              ],
            ),
          ),
          const SizedBox(height: 20),

          // Title with Icon
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              ClipRRect(
                borderRadius: BorderRadius.circular(12),
                child: Image.asset('assets/icon.png', width: 56, height: 56),
              ),
              const SizedBox(width: 14),
              Text(
                'StashIt',
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 52,
                  fontWeight: FontWeight.w900,
                  color: AppColors.textPrimary,
                  letterSpacing: -0.5,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),

          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 720),
            child: Text(
              Strings.get(
                'Умный временный карман (Drag & Drop Shelf) для Windows 10 & 11 в стиле macOS Dropover. Появляется по встряске мыши или при начале перетаскивания. Никакого лишнего мусора на экране в покое.',
                'Smart temporary shelf for Windows 10 & 11 inspired by macOS Dropover. Appears via mouse shake or as soon as you drag. Zero clutter on your desktop when idle.',
              ),
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 18, color: AppColors.textSecondary, height: 1.5),
            ),
          ),
          const SizedBox(height: 32),

          // CTAs
          Wrap(
            spacing: 16,
            runSpacing: 12,
            alignment: WrapAlignment.center,
            children: [
              ElevatedButton.icon(
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.accent,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(horizontal: 26, vertical: 16),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                  elevation: 4,
                ),
                icon: const Icon(Icons.download),
                label: Text(
                  Strings.get('Скачать для Windows (.exe)', 'Download for Windows (.exe)'),
                  style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15),
                ),
                onPressed: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt/releases')),
              ),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: AppColors.textPrimary,
                  side: BorderSide(color: AppColors.borderSubtle),
                  padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                ),
                icon: const Icon(Icons.code, size: 20),
                label: const Text('GitHub', style: TextStyle(fontSize: 15)),
                onPressed: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt')),
              ),
            ],
          ),
          const SizedBox(height: 40),

          // Панель быстрого управления интерактивным карманом
          Wrap(
            spacing: 10,
            runSpacing: 8,
            alignment: WrapAlignment.center,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: AppColors.textPrimary,
                  backgroundColor: AppColors.isDark ? const Color(0x1AFFFFFF) : Colors.white,
                  side: BorderSide(color: AppColors.isDark ? const Color(0x3338BDF8) : const Color(0x330078D6)),
                  padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
                  elevation: AppColors.isDark ? 0 : 1,
                ),
                icon: Icon(
                  _isDarkTheme ? Icons.dark_mode : Icons.light_mode,
                  size: 16,
                  color: _isDarkTheme ? const Color(0xFF0284C7) : const Color(0xFFF59E0B),
                ),
                label: Text(
                  _isDarkTheme
                      ? Strings.get('Тема кармана: Тёмная', 'Pocket: Dark')
                      : Strings.get('Тема кармана: Светлая', 'Pocket: Light'),
                  style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
                ),
                onPressed: () => setState(() => _isDarkTheme = !_isDarkTheme),
              ),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: AppColors.textPrimary,
                  backgroundColor: AppColors.isDark ? const Color(0x1AFFFFFF) : Colors.white,
                  side: BorderSide(color: AppColors.borderSubtle),
                  padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
                  elevation: AppColors.isDark ? 0 : 1,
                ),
                icon: const Icon(Icons.add_circle, size: 16, color: Color(0xFF10B981)),
                label: Text(
                  Strings.get('+ Добавить файл', '+ Add Item'),
                  style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
                ),
                onPressed: _addItem,
              ),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: AppColors.textPrimary,
                  backgroundColor: AppColors.isDark ? const Color(0x1AFFFFFF) : Colors.white,
                  side: BorderSide(color: AppColors.borderSubtle),
                  padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
                  elevation: AppColors.isDark ? 0 : 1,
                ),
                icon: const Icon(Icons.select_all, size: 16, color: AppColors.accent),
                label: Text(
                  Strings.get('Выбрать всё', 'Select All'),
                  style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
                ),
                onPressed: _toggleSelectAll,
              ),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: AppColors.textPrimary,
                  backgroundColor: AppColors.isDark ? const Color(0x1AFFFFFF) : Colors.white,
                  side: BorderSide(color: AppColors.borderSubtle),
                  padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
                  elevation: AppColors.isDark ? 0 : 1,
                ),
                icon: const Icon(Icons.delete_outline, size: 16, color: Color(0xFFEF4444)),
                label: Text(
                  Strings.get('Очистить', 'Clear'),
                  style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
                ),
                onPressed: _clearItems,
              ),
            ],
          ),
          const SizedBox(height: 20),

          // --- ЖИВОЙ ИНТЕРАКТИВНЫЙ МАКЕТ СТЭША ---
          DragTarget<String>(
            onWillAcceptWithDetails: (_) {
              setState(() => _isDraggingOver = true);
              return true;
            },
            onLeave: (_) => setState(() => _isDraggingOver = false),
            onAcceptWithDetails: (_) {
              _addItem();
              setState(() => _isDraggingOver = false);
            },
            builder: (context, candidateData, rejectedData) {
              return AnimatedContainer(
                duration: const Duration(milliseconds: 250),
                width: 350,
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: shelfBg,
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(color: shelfBorder, width: _isDraggingOver ? 2.0 : 1.2),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withValues(alpha: isDark ? 0.6 : 0.15),
                      blurRadius: 36,
                      offset: const Offset(0, 18),
                    ),
                    BoxShadow(
                      color: const Color(0xFF0078D6).withValues(alpha: isDark ? 0.25 : 0.1),
                      blurRadius: 24,
                      spreadRadius: 1,
                    ),
                  ],
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Заголовок окна в стиле StashIt Windows 11
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Row(
                          children: [
                            ClipRRect(
                              borderRadius: BorderRadius.circular(6),
                              child: Image.asset('assets/icon.png', width: 22, height: 22),
                            ),
                            const SizedBox(width: 8),
                            Text(
                              'StashIt',
                              style: TextStyle(
                                fontSize: 13,
                                fontWeight: FontWeight.bold,
                                color: textColor,
                              ),
                            ),
                            const SizedBox(width: 8),
                            AnimatedContainer(
                              duration: const Duration(milliseconds: 200),
                              padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 2),
                              decoration: BoxDecoration(
                                color: AppColors.accent,
                                borderRadius: BorderRadius.circular(10),
                              ),
                              child: Text(
                                '${_items.length}',
                                style: const TextStyle(fontSize: 11, fontWeight: FontWeight.bold, color: Colors.white),
                              ),
                            ),
                          ],
                        ),
                        Row(
                          children: [
                            IconButton(
                              icon: Icon(
                                isDark ? Icons.light_mode : Icons.dark_mode,
                                size: 16,
                                color: textSecondaryColor,
                              ),
                              padding: EdgeInsets.zero,
                              constraints: const BoxConstraints(minWidth: 26, minHeight: 26),
                              tooltip: Strings.get('Сменить тему кармана', 'Toggle pocket theme'),
                              onPressed: () => setState(() => _isDarkTheme = !_isDarkTheme),
                            ),
                            const SizedBox(width: 4),
                            IconButton(
                              icon: Icon(Icons.add_circle_outline, size: 16, color: textSecondaryColor),
                              padding: EdgeInsets.zero,
                              constraints: const BoxConstraints(minWidth: 26, minHeight: 26),
                              tooltip: Strings.get('Добавить тестовый файл', 'Add sample item'),
                              onPressed: _addItem,
                            ),
                            const SizedBox(width: 4),
                            IconButton(
                              icon: Icon(Icons.select_all, size: 16, color: textSecondaryColor),
                              padding: EdgeInsets.zero,
                              constraints: const BoxConstraints(minWidth: 26, minHeight: 26),
                              tooltip: Strings.get('Выбрать все', 'Select all'),
                              onPressed: _toggleSelectAll,
                            ),
                          ],
                        ),
                      ],
                    ),
                    const SizedBox(height: 12),

                    // Мастер-ручка захвата всей пачки
                    if (_items.isNotEmpty)
                      Draggable<String>(
                        data: 'batch',
                        feedback: Material(
                          color: Colors.transparent,
                          child: Container(
                            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                            decoration: BoxDecoration(
                              color: AppColors.accent,
                              borderRadius: BorderRadius.circular(8),
                              boxShadow: const [BoxShadow(color: Colors.black45, blurRadius: 10)],
                            ),
                            child: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                const Icon(Icons.layers, size: 16, color: Colors.white),
                                const SizedBox(width: 8),
                                Text(
                                  Strings.get('Пачка: ${_items.length} файлов', 'Batch: ${_items.length} items'),
                                  style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
                                ),
                              ],
                            ),
                          ),
                        ),
                        child: Container(
                          margin: const EdgeInsets.only(bottom: 10),
                          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 7),
                          decoration: BoxDecoration(
                            color: AppColors.accent.withValues(alpha: isDark ? 0.18 : 0.12),
                            borderRadius: BorderRadius.circular(8),
                            border: Border.all(color: AppColors.accent.withValues(alpha: 0.35)),
                          ),
                          child: Row(
                            children: [
                              const Icon(Icons.open_with, size: 14, color: AppColors.accent),
                              const SizedBox(width: 8),
                              Expanded(
                                child: Text(
                                  Strings.get(
                                    'Перетащить всё ($selectedCount из ${_items.length})',
                                    'Drag all ($selectedCount of ${_items.length})',
                                  ),
                                  style: const TextStyle(fontSize: 11, fontWeight: FontWeight.w600, color: AppColors.accent),
                                ),
                              ),
                              const Icon(Icons.pan_tool_alt, size: 13, color: AppColors.accent),
                            ],
                          ),
                        ),
                      ),

                    // Список файлов
                    if (_items.isEmpty)
                      Container(
                        height: 120,
                        alignment: Alignment.center,
                        decoration: BoxDecoration(
                          color: isDark ? const Color(0x08FFFFFF) : const Color(0x08000000),
                          borderRadius: BorderRadius.circular(10),
                          border: Border.all(
                            color: isDark ? const Color(0x14FFFFFF) : const Color(0x14000000),
                            style: BorderStyle.solid,
                          ),
                        ),
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            const Text('📥', style: TextStyle(fontSize: 26)),
                            const SizedBox(height: 6),
                            Text(
                              Strings.get('Карман пуст', 'Shelf is empty'),
                              style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold, color: textColor),
                            ),
                            const SizedBox(height: 4),
                            TextButton.icon(
                              icon: const Icon(Icons.add, size: 14),
                              label: Text(
                                Strings.get('Добавить файл', 'Add file'),
                                style: const TextStyle(fontSize: 11),
                              ),
                              onPressed: _addItem,
                            ),
                          ],
                        ),
                      )
                    else
                      Column(
                        children: _items.map((it) {
                          return _buildInteractiveItem(it, isDark, textColor, textSecondaryColor);
                        }).toList(),
                      ),

                    const SizedBox(height: 12),

                    // Нижняя панель состояния и кнопка очистки
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          _items.isEmpty
                              ? Strings.get('Ожидание сброса...', 'Waiting for drop...')
                              : Strings.get(
                                  '${_items.length} эл. • Готово к переносу',
                                  '${_items.length} items • Ready to drag',
                                ),
                          style: TextStyle(fontSize: 10, color: textSecondaryColor),
                        ),
                        InkWell(
                          borderRadius: BorderRadius.circular(6),
                          onTap: _clearItems,
                          child: AnimatedContainer(
                            duration: const Duration(milliseconds: 200),
                            padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 5),
                            decoration: BoxDecoration(
                              color: isDark ? const Color(0x1AFFFFFF) : const Color(0x15000000),
                              borderRadius: BorderRadius.circular(6),
                              border: Border.all(
                                color: isDark ? const Color(0x22FFFFFF) : const Color(0x22000000),
                              ),
                            ),
                            child: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Icon(Icons.delete_sweep_outlined, size: 13, color: textSecondaryColor),
                                const SizedBox(width: 4),
                                Text(
                                  Strings.get('Очистить', 'Clear'),
                                  style: TextStyle(fontSize: 10, fontWeight: FontWeight.w600, color: textColor),
                                ),
                              ],
                            ),
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              );
            },
          ),
        ],
      ),
    );
  }

  Widget _buildInteractiveItem(_ShelfItem it, bool isDark, Color textColor, Color metaColor) {
    return Draggable<String>(
      data: it.id,
      feedback: Material(
        color: Colors.transparent,
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          decoration: BoxDecoration(
            color: const Color(0xFF1E293B),
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: const Color(0xFF38BDF8)),
            boxShadow: const [BoxShadow(color: Colors.black54, blurRadius: 12)],
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(it.icon, style: const TextStyle(fontSize: 16)),
              const SizedBox(width: 8),
              Text(
                it.name,
                style: const TextStyle(color: Colors.white, fontSize: 11, fontWeight: FontWeight.bold),
              ),
            ],
          ),
        ),
      ),
      child: Container(
        margin: const EdgeInsets.only(bottom: 6),
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
        decoration: BoxDecoration(
          color: it.isSelected
              ? (isDark ? const Color(0x1C38BDF8) : const Color(0x150078D6))
              : (isDark ? const Color(0x0CFFFFFF) : const Color(0x0A000000)),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(
            color: it.isSelected
                ? const Color(0x5538BDF8)
                : (isDark ? const Color(0x14FFFFFF) : const Color(0x14000000)),
          ),
        ),
        child: Row(
          children: [
            InkWell(
              onTap: () => setState(() => it.isSelected = !it.isSelected),
              child: Padding(
                padding: const EdgeInsets.only(right: 6),
                child: Icon(
                  it.isSelected ? Icons.check_box : Icons.check_box_outline_blank,
                  size: 15,
                  color: it.isSelected ? AppColors.accent : metaColor,
                ),
              ),
            ),
            Text(it.icon, style: const TextStyle(fontSize: 16)),
            const SizedBox(width: 8),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    it.name,
                    style: TextStyle(
                      fontSize: 11,
                      fontWeight: FontWeight.w600,
                      color: textColor,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                  Text(it.meta, style: TextStyle(fontSize: 9, color: metaColor)),
                ],
              ),
            ),
            IconButton(
              icon: const Icon(Icons.close, size: 13),
              color: metaColor,
              padding: EdgeInsets.zero,
              constraints: const BoxConstraints(minWidth: 20, minHeight: 20),
              tooltip: Strings.get('Удалить из кармана', 'Remove'),
              onPressed: () => _removeItem(it.id),
            ),
            const SizedBox(width: 2),
            const Icon(Icons.drag_indicator, size: 14, color: Color(0xFF64748B)),
          ],
        ),
      ),
    );
  }
}