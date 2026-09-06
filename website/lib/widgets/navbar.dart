import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme.dart';
import '../i18n.dart';

class NavBar extends StatelessWidget {
  final VoidCallback onFeaturesTap;
  final VoidCallback onComparisonTap;
  final VoidCallback onFaqTap;
  final VoidCallback onDownloadTap;

  const NavBar({
    super.key,
    required this.onFeaturesTap,
    required this.onComparisonTap,
    required this.onFaqTap,
    required this.onDownloadTap,
  });

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: siteThemeMode,
      builder: (context, mode, _) {
        final isDark = mode == ThemeMode.dark;
        final bgColor = isDark
            ? const Color(0xFF0A0E1A).withValues(alpha: 0.85)
            : const Color(0xFFFFFFFF).withValues(alpha: 0.85);
        final borderColor = isDark ? const Color(0x1AFFFFFF) : const Color(0x1E000000);
        final titleColor = isDark ? Colors.white : const Color(0xFF0F172A);

        return Container(
          height: 64,
          padding: const EdgeInsets.symmetric(horizontal: 24),
          decoration: BoxDecoration(
            color: bgColor,
            border: Border(bottom: BorderSide(color: borderColor)),
          ),
          child: Row(
            children: [
              Row(
                children: [
                  ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: Image.asset(
                      'assets/logo.png',
                      width: 30,
                      height: 30,
                      fit: BoxFit.cover,
                    ),
                  ),
                  const SizedBox(width: 10),
                  Text(
                    'StashIt',
                    style: TextStyle(
                      fontWeight: FontWeight.bold,
                      fontSize: 17,
                      color: titleColor,
                      letterSpacing: 0.2,
                    ),
                  ),
                ],
              ),
          const Spacer(),
          if (MediaQuery.of(context).size.width > 700) ...[
            TextButton(
              onPressed: onFeaturesTap,
              child: Text(Strings.get('Возможности', 'Features'), style: const TextStyle(color: AppColors.textSecondary)),
            ),
            TextButton(
              onPressed: onComparisonTap,
              child: Text(Strings.get('Сравнение', 'Comparison'), style: const TextStyle(color: AppColors.textSecondary)),
            ),
            TextButton(
              onPressed: onFaqTap,
              child: Text(Strings.get('FAQ', 'FAQ'), style: const TextStyle(color: AppColors.textSecondary)),
            ),
            const SizedBox(width: 8),
          ],
          ValueListenableBuilder<ThemeMode>(
            valueListenable: siteThemeMode,
            builder: (context, mode, _) {
              final isDark = mode == ThemeMode.dark;
              return IconButton(
                icon: Icon(
                  isDark ? Icons.light_mode : Icons.dark_mode,
                  size: 20,
                  color: AppColors.textSecondary,
                ),
                onPressed: toggleSiteTheme,
                tooltip: Strings.get('Сменить тему сайта (Светлая/Тёмная)', 'Toggle site theme (Light/Dark)'),
              );
            },
          ),
          IconButton(
            icon: const Icon(Icons.language, size: 20, color: AppColors.textSecondary),
            onPressed: toggleLanguage,
            tooltip: Strings.get('Сменить язык (RU/EN)', 'Toggle Language (RU/EN)'),
          ),
          const SizedBox(width: 8),
          ElevatedButton(
            style: ElevatedButton.styleFrom(
              backgroundColor: AppColors.accent,
              foregroundColor: Colors.white,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            ),
            onPressed: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt/releases')),
            child: Text(Strings.get('Скачать', 'Download')),
          ),
        ],
      ),
    );
      },
    );
  }
}