import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme.dart';
import '../i18n.dart';

class HeroSection extends StatelessWidget {
  const HeroSection({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 50),
      child: Column(
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6),
            decoration: BoxDecoration(
              color: AppColors.surfaceCard,
              borderRadius: BorderRadius.circular(20),
              border: Border.all(color: AppColors.borderSubtle),
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                const Icon(Icons.verified, size: 16, color: AppColors.accent),
                const SizedBox(width: 8),
                Text(
                  Strings.get('Экосистема Kobalt Tools • Windows 10/11', 'Kobalt Tools Ecosystem • Windows 10/11'),
                  style: const TextStyle(fontSize: 12, color: AppColors.textSecondary),
                ),
              ],
            ),
          ),
          const SizedBox(height: 24),
          const Text(
            'StashIt',
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Colors.white, letterSpacing: -0.5),
          ),
          const SizedBox(height: 16),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 700),
            child: Text(
              Strings.get(
                'Умный временный карман (Drag & Drop Shelf) для Windows 10 & 11 в стиле macOS Dropover. Появляется по встряске мыши или при начале перетаскивания. Никакого лишнего мусора на экране в покое.',
                'Smart temporary shelf for Windows 10 & 11 inspired by macOS Dropover. Appears via mouse shake or as soon as you drag. Zero clutter on your desktop when idle.',
              ),
              textAlign: TextAlign.center,
              style: const TextStyle(fontSize: 18, color: AppColors.textSecondary, height: 1.5),
            ),
          ),
          const SizedBox(height: 32),
          Wrap(
            spacing: 16,
            runSpacing: 12,
            alignment: WrapAlignment.center,
            children: [
              ElevatedButton.icon(
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.accent,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                ),
                icon: const Icon(Icons.download),
                label: Text(
                  Strings.get('Скачать для Windows (.exe)', 'Download for Windows (.exe)'),
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
                onPressed: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt/releases')),
              ),
              const SizedBox(width: 16),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: Colors.white,
                  side: const BorderSide(color: AppColors.borderSubtle),
                  padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                ),
                icon: const Icon(Icons.code, size: 20),
                label: const Text('GitHub', style: TextStyle(fontSize: 16)),
                onPressed: () => launchUrl(Uri.parse('https://github.com/kobaltgit/StashIt')),
              ),
            ],
          ),
          const SizedBox(height: 48),

          // Интерактивный макет кармана StashIt в стиле Windows 11
          Container(
            width: 340,
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: const Color(0xD0121826),
              borderRadius: BorderRadius.circular(14),
              border: Border.all(color: const Color(0x3338BDF8), width: 1.5),
              boxShadow: const [
                BoxShadow(
                  color: Color(0x66000000),
                  blurRadius: 30,
                  offset: Offset(0, 15),
                ),
                BoxShadow(
                  color: Color(0x2238BDF8),
                  blurRadius: 20,
                  spreadRadius: 2,
                ),
              ],
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Row(
                      children: [
                        Container(
                          width: 8,
                          height: 8,
                          decoration: const BoxDecoration(
                            color: Color(0xFF10B981),
                            shape: BoxShape.circle,
                            boxShadow: [BoxShadow(color: Color(0xFF10B981), blurRadius: 6)],
                          ),
                        ),
                        const SizedBox(width: 8),
                        const Text(
                          'StashIt',
                          style: TextStyle(fontSize: 13, fontWeight: FontWeight.bold, color: Colors.white),
                        ),
                        const SizedBox(width: 8),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                          decoration: BoxDecoration(
                            color: AppColors.accent,
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: const Text('3', style: TextStyle(fontSize: 10, fontWeight: FontWeight.bold, color: Colors.white)),
                        ),
                      ],
                    ),
                    const Row(
                      children: [
                        Icon(Icons.brightness_4, size: 14, color: Color(0xFF94A3B8)),
                        SizedBox(width: 8),
                        Icon(Icons.settings, size: 14, color: Color(0xFF94A3B8)),
                        SizedBox(width: 8),
                        Icon(Icons.close, size: 14, color: Color(0xFF94A3B8)),
                      ],
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                _demoCard('🖼️', 'presentation_mockup.png', '2.4 МБ • .png'),
                const SizedBox(height: 6),
                _demoCard('📄', 'project_specification.pdf', '840 КБ • .pdf'),
                const SizedBox(height: 6),
                _demoCard('📁', 'assets_bundle', 'Папка • 12 файлов'),
                const SizedBox(height: 12),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      Strings.get('3.2 МБ • Готово к переносу', '3.2 MB • Ready to drag'),
                      style: const TextStyle(fontSize: 10, color: Color(0xFF94A3B8)),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                      decoration: BoxDecoration(
                        color: const Color(0x1AFFFFFF),
                        borderRadius: BorderRadius.circular(6),
                        border: Border.all(color: const Color(0x22FFFFFF)),
                      ),
                      child: Text(
                        Strings.get('Очистить', 'Clear'),
                        style: const TextStyle(fontSize: 10, color: Colors.white),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  static Widget _demoCard(String icon, String title, String meta) {
    return Container(
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: const Color(0x10FFFFFF),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: const Color(0x1AFFFFFF)),
      ),
      child: Row(
        children: [
          Text(icon, style: const TextStyle(fontSize: 16)),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  title,
                  style: const TextStyle(fontSize: 11, fontWeight: FontWeight.w600, color: Colors.white),
                  overflow: TextOverflow.ellipsis,
                ),
                Text(meta, style: const TextStyle(fontSize: 9, color: Color(0xFF94A3B8))),
              ],
            ),
          ),
          const Icon(Icons.drag_indicator, size: 14, color: Color(0xFF64748B)),
        ],
      ),
    );
  }
}