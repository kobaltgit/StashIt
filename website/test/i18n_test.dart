import 'package:flutter_test/flutter_test.dart';
import 'package:stashit_website/i18n.dart';

void main() {
  group('Website i18n tests', () {
    setUp(() {
      currentLang.value = AppLang.ru;
    });

    test('Default language is RU', () {
      expect(currentLang.value, AppLang.ru);
      expect(Strings.get('Привет', 'Hello'), 'Привет');
    });

    test('Toggle language switches between RU and EN', () {
      toggleLanguage();
      expect(currentLang.value, AppLang.en);
      expect(Strings.get('Привет', 'Hello'), 'Hello');

      toggleLanguage();
      expect(currentLang.value, AppLang.ru);
      expect(Strings.get('Привет', 'Hello'), 'Привет');
    });
  });
}
