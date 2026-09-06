import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class ComparisonTable extends StatelessWidget {
  const ComparisonTable({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: Column(
        children: [
          Text(
            Strings.get('Почему Kobalt Tools?', 'Why Kobalt Tools?'),
            style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
          ),
          const SizedBox(height: 24),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Container(
              decoration: BoxDecoration(
                color: AppColors.surfaceCard,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: AppColors.borderSubtle),
              ),
              child: Table(
                border: TableBorder.all(color: AppColors.borderSubtle, width: 0.5),
                children: [
                  TableRow(
                    decoration: const BoxDecoration(color: Color(0x15FFFFFF)),
                    children: [
                      _cell(Strings.get('Параметр', 'Metric'), isHeader: true),
                      _cell('StashIt', isHeader: true, color: AppColors.accent),
                      _cell(Strings.get('Типичный софт (Electron)', 'Typical Apps (Electron)'), isHeader: true),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('ОЗУ в фоне', 'Background RAM')),
                      _cell('< 25 МБ', color: Colors.greenAccent),
                      _cell('150–350 МБ'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Размер инсталлятора', 'Installer Size')),
                      _cell('~10–15 МБ', color: Colors.greenAccent),
                      _cell('80–120 МБ'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Нагрузка на CPU', 'Idle CPU Usage')),
                      _cell('0.0%', color: Colors.greenAccent),
                      _cell('0.5–2.0%'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Стек технологий', 'Tech Stack')),
                      _cell('Rust + Tauri v2 + Svelte 5'),
                      _cell('Node.js + Chromium'),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _cell(String text, {bool isHeader = false, Color? color}) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      child: Text(
        text,
        style: TextStyle(
          fontWeight: isHeader ? FontWeight.bold : FontWeight.normal,
          color: color ?? (isHeader ? Colors.white : AppColors.textSecondary),
          fontSize: isHeader ? 14 : 13,
        ),
      ),
    );
  }
}