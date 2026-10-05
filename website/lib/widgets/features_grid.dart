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
        'icon': Icons.tune,
        'titleRu': '4 способа активации',
        'titleEn': '4 Activation Triggers',
        'descRu': 'Встряхивание (Shake), глобальный хоткей (Ctrl+Shift+Space), двойной клик 2×Ctrl или край экрана (Edge Dock) — включайте любые в настройках.',
        'descEn': 'Mouse Shake, global hotkey (Ctrl+Shift+Space), 2×Ctrl double-tap, or screen edge dock — customize any trigger in settings.',
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
      {
        'icon': Icons.qr_code_scanner,
        'titleRu': 'Обмен со смартфоном по Wi-Fi',
        'titleEn': 'Two-Way Phone Drop (Wi-Fi QR)',
        'descRu': 'Сканируйте QR-код камерой телефона: передавайте фото и файлы с телефона прямо в карман на ПК и скачивайте файлы обратно без проводов и облаков.',
        'descEn': 'Scan the QR code with your phone camera: upload photos and files directly into your desktop shelf and download back with zero cables or cloud.',
      },
      {
        'icon': Icons.system_update_alt,
        'titleRu': 'Тихие обновления и Раздел «О программе»',
        'titleEn': 'Silent Updates & About Section',
        'descRu': 'Автоматическая ненавязчивая проверка новых релизов с GitHub, быстрое скачивание установщика или portable-версии.',
        'descEn': 'Automatic unobtrusive checks for new releases on GitHub with quick downloads for installer or portable builds.',
      },
    ];

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: Column(
        children: [
          Text(
            Strings.get('Ключевые возможности', 'Key Features'),
            style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
          ),
          const SizedBox(height: 12),
          Text(
            Strings.get('Полная функциональность в стиле macOS Dropover на Windows', 'Full macOS Dropover-style experience on Windows'),
            style: TextStyle(fontSize: 14, color: AppColors.textSecondary),
          ),
          const SizedBox(height: 32),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 1040),
            child: Wrap(
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
    final screenWidth = MediaQuery.of(context).size.width;
    final cardWidth = screenWidth < 360 ? screenWidth - 48 : 320.0;

    return SizedBox(
      width: cardWidth,
      height: 240,
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOutCubic,
          transform: Matrix4.translationValues(0, _isHovered ? -4 : 0, 0),
          width: cardWidth,
          height: 240,
          padding: const EdgeInsets.symmetric(horizontal: 22, vertical: 20),
        decoration: BoxDecoration(
          color: _isHovered ? AppColors.surfaceCardHover : AppColors.surfaceCard,
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
              : (AppColors.isDark
                  ? const []
                  : [
                      BoxShadow(
                        color: Colors.black.withValues(alpha: 0.04),
                        blurRadius: 10,
                        offset: const Offset(0, 4),
                      )
                    ]),
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
            const SizedBox(height: 14),
            SizedBox(
              height: 44,
              child: Text(
                Strings.get(widget.titleRu, widget.titleEn),
                style: TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: AppColors.textPrimary,
                  height: 1.25,
                ),
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
              ),
            ),
            const SizedBox(height: 6),
            Expanded(
              child: Text(
                Strings.get(widget.descRu, widget.descEn),
                style: TextStyle(fontSize: 13, color: AppColors.textSecondary, height: 1.45),
              ),
            ),
          ],
        ),
      ),
    ),
  );
}
}
