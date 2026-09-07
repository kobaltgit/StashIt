import 'package:flutter/material.dart';
import 'i18n.dart';
import 'theme.dart';
import 'package:kobalt_ui/kobalt_ui.dart';
import 'widgets/hero_section.dart';
import 'widgets/features_grid.dart';
import 'widgets/comparison_table.dart';
import 'widgets/faq_section.dart';
import 'widgets/download_cta.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const KobaltWebsiteApp());
}

class KobaltWebsiteApp extends StatelessWidget {
  const KobaltWebsiteApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: siteThemeMode,
      builder: (context, themeMode, _) {
        return ValueListenableBuilder<AppLang>(
          valueListenable: currentLang,
          builder: (context, lang, _) {
            return MaterialApp(
              key: ValueKey('app_${lang}_$themeMode'),
              title: 'StashIt — Kobalt Tools',
              debugShowCheckedModeBanner: false,
              theme: buildAppLightTheme(),
              darkTheme: buildAppTheme(),
              themeMode: themeMode,
              home: const LandingPage(),
            );
          },
        );
      },
    );
  }
}

class LandingPage extends StatefulWidget {
  const LandingPage({super.key});

  @override
  State<LandingPage> createState() => _LandingPageState();
}

class _LandingPageState extends State<LandingPage> {
  final ScrollController _scrollController = ScrollController();
  final GlobalKey _featuresKey = GlobalKey();
  final GlobalKey _comparisonKey = GlobalKey();
  final GlobalKey _faqKey = GlobalKey();
  final GlobalKey _downloadKey = GlobalKey();

  void _scrollTo(GlobalKey key) {
    final ctx = key.currentContext;
    if (ctx != null) {
      Scrollable.ensureVisible(
        ctx,
        duration: const Duration(milliseconds: 600),
        curve: Curves.easeInOutCubic,
      );
    }
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: siteThemeMode,
      builder: (context, themeMode, _) {
        final isDark = themeMode == ThemeMode.dark;
        final bgColor = isDark ? const Color(0xFF0A0E1A) : const Color(0xFFF1F5F9);
        return Scaffold(
          backgroundColor: bgColor,
          body: Stack(
            children: [
              // Ambient Glow
              Positioned(
                top: -160,
                left: 0,
                right: 0,
                height: 650,
                child: Container(
                  decoration: BoxDecoration(
                    gradient: RadialGradient(
                      center: const Alignment(0.0, -0.6),
                      radius: 0.9,
                      colors: [
                        isDark ? const Color(0x330078D6) : const Color(0x180078D6),
                        isDark ? const Color(0x000A0E1A) : const Color(0x00F1F5F9),
                      ],
                    ),
                  ),
                ),
              ),
              Column(
                children: [
                  KobaltNavBar(
                    project: KobaltProjectId.stashIt,
                    version: 'v1.0.1',
                    isRussian: currentLang.value == AppLang.ru,
                    onLanguageToggle: toggleLanguage,
                    accentColor: AppColors.accent,
                    navLinks: [
                      KobaltNavLink(
                        label: Strings.get('Возможности', 'Features'),
                        onTap: () => _scrollTo(_featuresKey),
                      ),
                      KobaltNavLink(
                        label: Strings.get('Сравнение', 'Comparison'),
                        onTap: () => _scrollTo(_comparisonKey),
                      ),
                      KobaltNavLink(
                        label: Strings.get('FAQ', 'FAQ'),
                        onTap: () => _scrollTo(_faqKey),
                      ),
                    ],
                    onDownloadTap: () => _scrollTo(_downloadKey),
                    extraActions: [
                      IconButton(
                        icon: Icon(
                          isDark ? Icons.light_mode_rounded : Icons.dark_mode_rounded,
                          size: 18,
                          color: AppColors.textSecondary,
                        ),
                        onPressed: toggleSiteTheme,
                        tooltip: Strings.get(
                          'Сменить тему сайта (Светлая/Тёмная)',
                          'Toggle site theme (Light/Dark)',
                        ),
                      ),
                    ],
                  ),
                  Expanded(
                    child: SingleChildScrollView(
                      controller: _scrollController,
                      child: Column(
                        children: [
                          const HeroSection(),
                          FeaturesGrid(key: _featuresKey),
                          ComparisonTable(key: _comparisonKey),
                          FaqSection(key: _faqKey),
                          DownloadCta(key: _downloadKey),
                          KobaltFooter(
                            project: KobaltProjectId.stashIt,
                            version: 'v1.0.1',
                            isRussian: currentLang.value == AppLang.ru,
                            accentColor: AppColors.accent,
                            onBackToTop: () {
                              _scrollController.animateTo(
                                0,
                                duration: const Duration(milliseconds: 600),
                                curve: Curves.easeInOutCubic,
                              );
                            },
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        );
      },
    );
  }
}