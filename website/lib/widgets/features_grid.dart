import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class FeaturesGrid extends StatelessWidget {
  const FeaturesGrid({super.key});

  @override
  Widget build(BuildContext context) {
    final features = [
      {
        'icon': Icons.vibration,
        'titleRu': 'Встряска мыши (Shake)',
        'titleEn': 'Shake to Show',
        'descRu': 'Зажмите файл и слегка качните мышь — карман мгновенно появится прямо у вашего курсора.',
        'descEn': 'Hold a file and shake your mouse slightly — the shelf immediately appears at your cursor.',
      },
      {
        'icon': Icons.drag_indicator,
        'titleRu': 'Сразу при перетаскивании',
        'titleEn': 'Auto on Drag',
        'descRu': 'Режим появления кармана сразу же при начале перетаскивания файлов или текста.',
        'descEn': 'Instant shelf trigger mode right as you begin dragging files or selected text.',
      },
      {
        'icon': Icons.dark_mode,
        'titleRu': 'Тёмная, Светлая и Системная темы',
        'titleEn': 'Dark, Light & System Themes',
        'descRu': 'Современный акрил Fluent с поддержкой светлого и тёмного оформления под стиль Windows 11.',
        'descEn': 'Modern Fluent acrylic supporting light and dark modes matching Windows 11 styling.',
      },
      {
        'icon': Icons.flash_on,
        'titleRu': 'До 25 МБ ОЗУ',
        'titleEn': 'Under 25 MB RAM',
        'descRu': 'Нативный движок на Rust 2021 и Tauri v2. Никаких фоновых процессов Chromium и Electron.',
        'descEn': 'Native Rust 2021 and Tauri v2 engine. Zero Chromium background services or Electron bloat.',
      },
      {
        'icon': Icons.layers,
        'titleRu': 'Захват стопки и автоочистка',
        'titleEn': 'Batch Drag & Smart Clear',
        'descRu': 'Вытягивайте всю стопку файлов разом через мастер-ручку с умным 5-секундным таймером автоочистки.',
        'descEn': 'Drag out the entire batch at once with master handle and smart 5s auto-clear timer.',
      },
      {
        'icon': Icons.power_settings_new,
        'titleRu': 'Чистый автозапуск',
        'titleEn': 'Clean Autostart',
        'descRu': 'Работа через ветку реестра HKCU без раздражающих всплывающих окон UAC администратора.',
        'descEn': 'Startup via HKCU registry without annoying administrator UAC permission prompts.',
      },
    ];

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: Column(
        children: [
          Text(
            Strings.get('Ключевые возможности', 'Key Features'),
            style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
          ),
          const SizedBox(height: 12),
          Text(
            Strings.get('Полная функциональность в стиле macOS Dropover на Windows', 'Full macOS Dropover-style experience on Windows'),
            style: const TextStyle(fontSize: 14, color: AppColors.textSecondary),
          ),
          const SizedBox(height: 32),
          Wrap(
            spacing: 20,
            runSpacing: 20,
            alignment: WrapAlignment.center,
            children: features.map((f) {
              return _FeatureCard(
                icon: f['icon'] as IconData,
                titleRu: f['titleRu'] as String,
                titleEn: f['titleEn'] as String,
                descRu: f['descRu'] as String,
                descEn: f['descEn'] as String,
              );
            }).toList(),
          ),
        ],
      ),
    );
  }
}

class _FeatureCard extends StatefulWidget {
  final IconData icon;
  final String titleRu;
  final String titleEn;
  final String descRu;
  final String descEn;

  const _FeatureCard({
    required this.icon,
    required this.titleRu,
    required this.titleEn,
    required this.descRu,
    required this.descEn,
  });

  @override
  State<_FeatureCard> createState() => _FeatureCardState();
}

class _FeatureCardState extends State<_FeatureCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOutCubic,
        transform: Matrix4.translationValues(0, _isHovered ? -4 : 0, 0),
        width: 320,
        padding: const EdgeInsets.all(22),
        decoration: BoxDecoration(
          color: _isHovered ? const Color(0x18FFFFFF) : AppColors.surfaceCard,
          borderRadius: BorderRadius.circular(14),
          border: Border.all(
            color: _isHovered ? const Color(0x880078D6) : AppColors.borderSubtle,
            width: _isHovered ? 1.5 : 1.0,
          ),
          boxShadow: _isHovered
              ? const [
                  BoxShadow(
                    color: Color(0x280078D6),
                    blurRadius: 20,
                    offset: Offset(0, 8),
                  )
                ]
              : const [],
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              padding: const EdgeInsets.all(10),
              decoration: BoxDecoration(
                color: _isHovered ? AppColors.accent : const Color(0x140078D6),
                borderRadius: BorderRadius.circular(10),
              ),
              child: Icon(
                widget.icon,
                color: _isHovered ? Colors.white : AppColors.accent,
                size: 24,
              ),
            ),
            const SizedBox(height: 16),
            Text(
              Strings.get(widget.titleRu, widget.titleEn),
              style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 8),
            Text(
              Strings.get(widget.descRu, widget.descEn),
              style: const TextStyle(fontSize: 13, color: AppColors.textSecondary, height: 1.45),
            ),
          ],
        ),
      ),
    );
  }
}
