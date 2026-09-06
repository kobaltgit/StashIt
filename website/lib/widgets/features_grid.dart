import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class FeaturesGrid extends StatelessWidget {
  const FeaturesGrid({super.key});

  @override
  Widget build(BuildContext context) {
    final features = [
      {
        'icon': Icons.flash_on,
        'titleRu': 'Сверхлегковесность',
        'titleEn': 'Ultra Lightweight',
        'descRu': 'Потребление памяти менее 25 МБ. Чистый Rust и Tauri v2 без Electron.',
        'descEn': 'Memory footprint under 25 MB RAM. Native Rust and Tauri v2 without Electron.',
      },
      {
        'icon': Icons.blur_on,
        'titleRu': 'Fluent Glassmorphism',
        'titleEn': 'Fluent Glassmorphism',
        'descRu': 'Акриловое полупрозрачное окно у системного трея с адаптацией под тему Windows.',
        'descEn': 'Acrylic blur flyout near tray automatically adapting to Windows dark/light mode.',
      },
      {
        'icon': Icons.lock,
        'titleRu': '100% Локально и Приватно',
        'titleEn': '100% Local & Private',
        'descRu': 'Никаких облаков и передачи данных. Все вычисления происходят на вашем ПК.',
        'descEn': 'Zero cloud dependencies. All data processing stays strictly on your computer.',
      },
      {
        'icon': Icons.power_settings_new,
        'titleRu': 'Чистый автозапуск',
        'titleEn': 'Clean Autostart',
        'descRu': 'Автостарт через реестр пользователя без запросов прав администратора (UAC).',
        'descEn': 'Startup via current user registry HKCU without annoying UAC administrator prompts.',
      },
    ];

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: Column(
        children: [
          Text(
            Strings.get('Ключевые преимущества', 'Key Features'),
            style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
          ),
          const SizedBox(height: 32),
          Wrap(
            spacing: 20,
            runSpacing: 20,
            children: features.map((f) {
              return Container(
                width: 300,
                padding: const EdgeInsets.all(20),
                decoration: BoxDecoration(
                  color: AppColors.surfaceCard,
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: AppColors.borderSubtle),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Icon(f['icon'] as IconData, color: AppColors.accent, size: 28),
                    const SizedBox(height: 14),
                    Text(
                      Strings.get(f['titleRu'] as String, f['titleEn'] as String),
                      style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      Strings.get(f['descRu'] as String, f['descEn'] as String),
                      style: const TextStyle(fontSize: 13, color: AppColors.textSecondary, height: 1.4),
                    ),
                  ],
                ),
              );
            }).toList(),
          ),
        ],
      ),
    );
  }
}
