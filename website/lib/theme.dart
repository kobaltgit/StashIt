import 'package:flutter/material.dart';

class AppColors {
  static const background = Color(0xFF0A0E1A);
  static const surface = Color(0xFF141A29);
  static const surfaceCard = Color(0x0AFFFFFF);
  static const surfaceCardHover = Color(0x14FFFFFF);
  
  static const accent = Color(0xFF0078D6);
  static const accentHover = Color(0xFF1A88E1);
  
  static const textPrimary = Colors.white;
  static const textSecondary = Color(0xFF94A3B8);
  static const textMuted = Color(0xFF64748B);
  static const borderSubtle = Color(0x1AFFFFFF);

  static const heroGlowGradient = RadialGradient(
    center: Alignment(0.0, -0.6),
    radius: 0.9,
    colors: [
      Color(0x330078D6),
      Color(0x000A0E1A),
    ],
  );
}

ThemeData buildAppTheme() {
  return ThemeData.dark().copyWith(
    scaffoldBackgroundColor: AppColors.background,
    colorScheme: const ColorScheme.dark(
      primary: AppColors.accent,
      surface: AppColors.surface,
    ),
  );
}
