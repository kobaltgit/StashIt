import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme.dart';
import '../i18n.dart';

class DownloadCta extends StatelessWidget {
  const DownloadCta({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 24, vertical: 60),
      padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 48),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: AppColors.borderSubtle),
        boxShadow: AppColors.isDark
            ? const []
            : [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.05),
                  blurRadius: 16,
                  offset: const Offset(0, 6),
                ),
              ],
      ),
      child: Column(
        children: [
          Text(
            Strings.get('Попробуйте StashIt прямо сейчас', 'Try StashIt Right Now'),
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
          ),
          const SizedBox(height: 12),
          Text(
            Strings.get(
              'Бесплатно, открытый исходный код под лицензией MIT, готов для Windows 10 & 11.',
              'Free, open-source under MIT license, ready for Windows 10 & 11.',
            ),
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 15, color: AppColors.textSecondary),
          ),
          const SizedBox(height: 28),
          ElevatedButton.icon(
            style: ElevatedButton.styleFrom(
              backgroundColor: AppColors.accent,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 18),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
            ),
            icon: const Icon(Icons.download),
            label: Text(
              Strings.get('Перейти к релизам на GitHub', 'Go to GitHub Releases'),
              style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
            ),
            onPressed: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt/releases/latest')),
          ),
        ],
      ),
    );
  }
}