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
            style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
          ),
          const SizedBox(height: 10),
          Text(
            Strings.get('Сравнение с аналогами на macOS (Dropover / Yoink) и громоздким софтом на Windows', 'Comparison with macOS tools (Dropover / Yoink) and heavy Windows alternatives'),
            style: const TextStyle(fontSize: 14, color: AppColors.textSecondary),
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
              ),
              child: Table(
                border: TableBorder.all(color: AppColors.borderSubtle, width: 0.5),
                children: [
                  TableRow(
                    decoration: const BoxDecoration(color: Color(0x15FFFFFF)),
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
                      _cell('Windows 10 & 11', color: Colors.greenAccent),
                      _cell('Только macOS / iPadOS'),
                      _cell('Windows / Mac'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('ОЗУ в фоне', 'Background RAM')),
                      _cell('< 25 МБ RAM', color: Colors.greenAccent),
                      _cell('~35–60 МБ'),
                      _cell('180–400 МБ'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('В покое на экране', 'Idle on Screen')),
                      _cell(Strings.get('100% Скрыт', '100% Invisible'), color: Colors.greenAccent),
                      _cell(Strings.get('100% Скрыт', '100% Invisible')),
                      _cell(Strings.get('Часто висит поверх окон', 'Often stays visible')),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Триггеры вызова', 'Triggers')),
                      _cell(Strings.get('Встряска + Перетаскивание', 'Shake + Drag Trigger'), color: Colors.greenAccent),
                      _cell(Strings.get('Встряска / Drag', 'Shake / Drag')),
                      _cell(Strings.get('Только хоткей / трей', 'Only hotkey / tray')),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Темы оформления', 'Themes')),
                      _cell(Strings.get('Dark / Light / System', 'Dark / Light / System'), color: Colors.greenAccent),
                      _cell('Dark / Light'),
                      _cell(Strings.get('Ограничено', 'Limited')),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Стек ядра', 'Core Stack')),
                      _cell('Rust 2021 + Tauri v2 + Svelte 5', color: Colors.greenAccent),
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
          color: color ?? (isHeader ? Colors.white : AppColors.textSecondary),
          fontSize: isHeader ? 13 : 12,
        ),
      ),
    );
  }
}