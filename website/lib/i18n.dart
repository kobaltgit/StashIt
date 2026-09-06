import 'package:flutter/foundation.dart';

enum AppLang { ru, en }

final ValueNotifier<AppLang> currentLang = ValueNotifier<AppLang>(AppLang.ru);

void toggleLanguage() {
  currentLang.value = currentLang.value == AppLang.ru ? AppLang.en : AppLang.ru;
}

class Strings {
  static String get(String ruText, String enText) {
    return currentLang.value == AppLang.ru ? ruText : enText;
  }
}
