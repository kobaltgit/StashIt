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
            Strings.get('Почему StashIt в Kobalt Tools?', 'Why StashIt in Kobalt Tools?'),
            style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
          ),
          const SizedBox(height: 10),
          Text(
            Strings.get('Сравнение с аналогами на macOS (Dropover / Yoink) и громоздким софтом на Windows', 'Comparison with macOS tools (Dropover / Yoink) and heavy Windows alternatives'),
            style: TextStyle(fontSize: 14, color: AppColors.textSecondary),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 24),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 840),
            child: Container(
              decoration: BoxDecoration(
                color: AppColors.surfaceCard,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: AppColors.borderSubtle),
                boxShadow: AppColors.isDark
                    ? const []
                    : [
                        BoxShadow(
                          color: Colors.black.withValues(alpha: 0.04),
                          blurRadius: 10,
                          offset: const Offset(0, 4),
                        ),
                      ],
              ),
              child: Table(
                border: TableBorder.all(color: AppColors.borderSubtle, width: 0.5),
                children: [
                  TableRow(
                    decoration: BoxDecoration(
                      color: AppColors.isDark ? const Color(0x15FFFFFF) : const Color(0x0A000000),
                    ),
                    children: [
                      _cell(Strings.get('Параметр', 'Metric'), isHeader: true),
                      _cell('StashIt (Kobalt)', isHeader: true, color: AppColors.accent),
                      _cell('Dropover / Yoink', isHeader: true),
                      _cell(Strings.get('Аналоги на Electron', 'Electron Alternatives'), isHeader: true),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Платформа', 'Platform')),
                      _cell('Windows 10 & 11', color: AppColors.isDark ? Colors.greenAccent : const Color(0xFF059669)),
                      _cell('Только macOS / iPadOS'),
                      _cell('Windows / Mac'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('ОЗУ в фоне', 'Background RAM')),
                      _cell('< 25 МБ RAM', color: AppColors.isDark ? Colors.greenAccent : const Color(0xFF059669)),
                      _cell('~35–60 МБ'),
                      _cell('180–400 МБ'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('В покое на экране', 'Idle on Screen')),
                      _cell(Strings.get('100% Скрыт', '100% Invisible'), color: AppColors.isDark ? Colors.greenAccent : const Color(0xFF059669)),
                      _cell(Strings.get('100% Скрыт', '100% Invisible')),
                      _cell(Strings.get('Часто висит поверх окон', 'Often stays visible')),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Триггеры вызова', 'Triggers')),
                      _cell(Strings.get('Встряска + Перетаскивание', 'Shake + Drag Trigger'), color: AppColors.isDark ? Colors.greenAccent : const Color(0xFF059669)),
                      _cell(Strings.get('Встряска / Drag', 'Shake / Drag')),
                      _cell(Strings.get('Только хоткей / трей', 'Only hotkey / tray')),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Темы оформления', 'Themes')),
                      _cell(Strings.get('Dark / Light / System', 'Dark / Light / System'), color: AppColors.isDark ? Colors.greenAccent : const Color(0xFF059669)),
                      _cell('Dark / Light'),
                      _cell(Strings.get('Ограничено', 'Limited')),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Стек ядра', 'Core Stack')),
                      _cell('Rust 2021 + Tauri v2 + Svelte 5', color: AppColors.isDark ? Colors.greenAccent : const Color(0xFF059669)),
                      _cell('Swift / Cocoa'),
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
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
      child: Text(
        text,
        style: TextStyle(
          fontWeight: isHeader ? FontWeight.bold : FontWeight.normal,
          color: color ?? (isHeader ? AppColors.textPrimary : AppColors.textSecondary),
          fontSize: isHeader ? 13 : 12,
        ),
      ),
    );
  }
}