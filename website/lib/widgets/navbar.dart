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
    return Container(
      height: 64,
      padding: const EdgeInsets.symmetric(horizontal: 24),
      decoration: BoxDecoration(
        color: AppColors.background.withValues(alpha: 0.85),
        border: const Border(bottom: BorderSide(color: AppColors.borderSubtle)),
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
              const Text(
                'StashIt',
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 17, color: Colors.white, letterSpacing: 0.2),
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
  }
}