import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme.dart';
import '../i18n.dart';

class Footer extends StatelessWidget {
  const Footer({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 32),
      decoration: BoxDecoration(
        border: Border(top: BorderSide(color: AppColors.borderSubtle)),
      ),
      child: Center(
        child: Wrap(
          spacing: 16,
          runSpacing: 12,
          crossAxisAlignment: WrapCrossAlignment.center,
          alignment: WrapAlignment.center,
          children: [
            Text('© 2026 kobaltgit', style: TextStyle(color: AppColors.textMuted, fontSize: 13)),
            Text('•', style: TextStyle(color: AppColors.textMuted)),
            InkWell(
              onTap: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt/blob/master/LICENSE')),
              child: Text('MIT License', style: TextStyle(color: AppColors.textSecondary, fontSize: 13)),
            ),
            Text('•', style: TextStyle(color: AppColors.textMuted)),
            InkWell(
              onTap: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt')),
              child: Text('GitHub Repo', style: TextStyle(color: AppColors.textSecondary, fontSize: 13)),
            ),
            Text('•', style: TextStyle(color: AppColors.textMuted)),
            Text(
              Strings.get('Сделано для экосистемы Kobalt Tools', 'Crafted for Kobalt Tools Ecosystem'),
              style: TextStyle(color: AppColors.textMuted, fontSize: 13),
            ),
          ],
        ),
      ),
    );
  }
}