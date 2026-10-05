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
        'aRu': 'Поддерживаются 4 независимых способа: встряхивание мышью при перетаскивании (Shake), глобальный хоткей (по умолчанию Ctrl+Shift+Space), двойное быстрое нажатие 2×Ctrl и выдвижение от края экрана (Edge Dock). Каждый способ можно включить или выключить в настройках.',
        'aEn': 'StashIt supports 4 independent triggers: mouse shake while dragging, global shortcut (default Ctrl+Shift+Space), 2×Ctrl double-tap, and screen edge docking. Each method can be enabled or disabled in the settings menu.',
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
      {
        'qRu': 'Как обновляется StashIt?',
        'qEn': 'How does StashIt update?',
        'aRu': 'В программе есть раздел «О программе» с проверкой новых релизов прямо из GitHub Releases. Приложение может ненавязчиво уведомлять о свежих версиях или проверять их по нажатию кнопки.',
        'aEn': 'StashIt includes an "About" section checking GitHub Releases directly. It can notify you silently about new versions or check on demand.',
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
              style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
            ),
            const SizedBox(height: 24),
            ...faqs.map((faq) {
              return Container(
                margin: const EdgeInsets.only(bottom: 12),
                decoration: BoxDecoration(
                  color: AppColors.surfaceCard,
                  borderRadius: BorderRadius.circular(10),
                  border: Border.all(color: AppColors.borderSubtle),
                  boxShadow: AppColors.isDark
                      ? const []
                      : [
                          BoxShadow(
                            color: Colors.black.withValues(alpha: 0.03),
                            blurRadius: 8,
                            offset: const Offset(0, 2),
                          ),
                        ],
                ),
                child: ExpansionTile(
                  iconColor: AppColors.accent,
                  collapsedIconColor: AppColors.textSecondary,
                  title: Text(
                    Strings.get(faq['qRu']!, faq['qEn']!),
                    style: TextStyle(fontSize: 15, fontWeight: FontWeight.w600, color: AppColors.textPrimary),
                  ),
                  children: [
                    Padding(
                      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
                      child: Text(
                        Strings.get(faq['aRu']!, faq['aEn']!),
                        style: TextStyle(fontSize: 13, color: AppColors.textSecondary, height: 1.4),
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