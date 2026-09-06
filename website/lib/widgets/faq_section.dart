import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class FaqSection extends StatelessWidget {
  const FaqSection({super.key});

  @override
  Widget build(BuildContext context) {
    final faqs = [
      {
        'qRu': 'Требуются ли права администратора при установке?',
        'qEn': 'Does it require Administrator / UAC permissions?',
        'aRu': 'Нет. Приложение устанавливается в профиль пользователя, а автостарт прописывается в HKCU без навязчивых окон UAC.',
        'aEn': 'No. The app installs into the user profile, and autostart is written to HKCU without annoying UAC prompts.',
      },
      {
        'qRu': 'Где хранятся данные программы?',
        'qEn': 'Where is program data stored?',
        'aRu': 'Все данные и конфигурации сохраняются локально в папке %APPDATA%\\stashit. Никакой передачи в облако.',
        'aEn': 'All settings and data are stored locally in %APPDATA%\\stashit. Zero transmission to the cloud.',
      },
      {
        'qRu': 'Как вызвать окно утилиты?',
        'qEn': 'How do I open the app flyout?',
        'aRu': 'Кликните по иконке в системном трее Windows (около часов) или используйте назначенную горячую клавишу.',
        'aEn': 'Click on the icon in the Windows system tray (near the clock) or press the configured global hotkey.',
      },
    ];

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 800),
        child: Column(
          children: [
            Text(
              Strings.get('Часто задаваемые вопросы', 'Frequently Asked Questions'),
              style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 24),
            ...faqs.map((faq) {
              return Container(
                margin: const EdgeInsets.only(bottom: 12),
                decoration: BoxDecoration(
                  color: AppColors.surfaceCard,
                  borderRadius: BorderRadius.circular(10),
                  border: Border.all(color: AppColors.borderSubtle),
                ),
                child: ExpansionTile(
                  iconColor: AppColors.accent,
                  collapsedIconColor: AppColors.textSecondary,
                  title: Text(
                    Strings.get(faq['qRu']!, faq['qEn']!),
                    style: const TextStyle(fontSize: 15, fontWeight: FontWeight.w600, color: Colors.white),
                  ),
                  children: [
                    Padding(
                      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
                      child: Text(
                        Strings.get(faq['aRu']!, faq['aEn']!),
                        style: const TextStyle(fontSize: 13, color: AppColors.textSecondary, height: 1.4),
                      ),
                    ),
                  ],
                ),
              );
            }),
          ],
        ),
      ),
    );
  }
}