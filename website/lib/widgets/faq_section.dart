import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class FaqSection extends StatelessWidget {
  const FaqSection({super.key});

  @override
  Widget build(BuildContext context) {
    final faqs = [
      {
        'qRu': 'Как вызвать плавающий карман StashIt?',
        'qEn': 'How do I trigger the StashIt shelf?',
        'aRu': 'Зажмите левой кнопкой мыши любой файл/папку/текст и слегка встряхните мышь (Shake), либо включите режим появления сразу при начале перетаскивания (Auto on Drag). Также карман можно открыть кликом по иконке в трее.',
        'aEn': 'Hold any file/folder/text with your left mouse button and shake slightly (Shake), or enable instant appearance when dragging starts (Auto on Drag). You can also click the tray icon.',
      },
      {
        'qRu': 'Висит ли что-нибудь на экране, когда я не работаю с файлами?',
        'qEn': 'Does anything stay visible on screen when idle?',
        'aRu': 'Нет. StashIt на 100% невидим и скрыт в фоне, не занимая ни пикселя полезного пространства экрана.',
        'aEn': 'No. StashIt is 100% invisible and hidden in the background, consuming zero desktop screen real estate.',
      },
      {
        'qRu': 'Поддерживаются ли светлая и тёмная темы?',
        'qEn': 'Are Dark and Light themes supported?',
        'aRu': 'Да, прямо в верхнем тулбаре кармана есть переключатель между Тёмной, Светлой и Автоматической (системной) темой в стиле Windows 11 Fluent Acrylic.',
        'aEn': 'Yes, right in the shelf toolbar you can switch between Dark, Light, and Automatic (system) themes matching Windows 11 Fluent Acrylic.',
      },
      {
        'qRu': 'Требуются ли права администратора (UAC)?',
        'qEn': 'Does it require Administrator / UAC permissions?',
        'aRu': 'Нет. Приложение работает автономно, а автозагрузка безопасно настраивается через ветку реестра текущего пользователя HKCU без UAC.',
        'aEn': 'No. The app runs portably/per-user, and autostart is written safely to current user registry HKCU without UAC.',
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