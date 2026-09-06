import 'package:flutter/material.dart';

final ValueNotifier<ThemeMode> siteThemeMode = ValueNotifier<ThemeMode>(ThemeMode.dark);

void toggleSiteTheme() {
  siteThemeMode.value = siteThemeMode.value == ThemeMode.dark ? ThemeMode.light : ThemeMode.dark;
}

class AppColors {
  static bool get isDark => siteThemeMode.value == ThemeMode.dark;

  static const accent = Color(0xFF0078D6);
  static const accentHover = Color(0xFF1A88E1);

  static Color get background => isDark ? const Color(0xFF0A0E1A) : const Color(0xFFF1F5F9);
  static Color get surface => isDark ? const Color(0xFF141A29) : Colors.white;
  static Color get surfaceCard => isDark ? const Color(0x0AFFFFFF) : Colors.white;
  static Color get surfaceCardHover => isDark ? const Color(0x14FFFFFF) : const Color(0xFFE2E8F0);
  
  static Color get textPrimary => isDark ? Colors.white : const Color(0xFF0F172A);
  static Color get textSecondary => isDark ? const Color(0xFF94A3B8) : const Color(0xFF64748B);
  static Color get textMuted => isDark ? const Color(0xFF64748B) : const Color(0xFF94A3B8);
  static Color get borderSubtle => isDark ? const Color(0x1AFFFFFF) : const Color(0x1E000000);
}

ThemeData buildAppTheme() {
  return ThemeData.dark().copyWith(
    scaffoldBackgroundColor: const Color(0xFF0A0E1A),
    colorScheme: const ColorScheme.dark(
      primary: AppColors.accent,
      surface: Color(0xFF141A29),
    ),
  );
}

ThemeData buildAppLightTheme() {
  return ThemeData.light().copyWith(
    scaffoldBackgroundColor: const Color(0xFFF1F5F9),
    colorScheme: const ColorScheme.light(
      primary: AppColors.accent,
      surface: Colors.white,
    ),
  );
}
