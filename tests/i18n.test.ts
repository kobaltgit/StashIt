import { describe, it, expect } from "vitest";
import { translations } from "../src/i18n";

describe("i18n translations", () => {
  it("contains both ru and en languages", () => {
    expect(translations.ru).toBeDefined();
    expect(translations.en).toBeDefined();
  });

  it("has matching keys between ru and en", () => {
    const ruKeys = Object.keys(translations.ru).sort();
    const enKeys = Object.keys(translations.en).sort();
    expect(ruKeys).toEqual(enKeys);
  });

  it("formats themeTooltips correctly", () => {
    expect(translations.ru.themeTooltip("system")).toContain("Системная");
    expect(translations.ru.themeTooltip("dark")).toContain("Тёмная");
    expect(translations.ru.themeTooltip("light")).toContain("Светлая");

    expect(translations.en.themeTooltip("system")).toContain("System");
    expect(translations.en.themeTooltip("dark")).toContain("Dark");
    expect(translations.en.themeTooltip("light")).toContain("Light");
  });

  it("formats dragAll and dragSelected correctly", () => {
    expect(translations.ru.dragAll(5)).toContain("5");
    expect(translations.ru.dragSelected(3)).toContain("3");
    expect(translations.en.dragAll(5)).toContain("5");
    expect(translations.en.dragSelected(3)).toContain("3");
  });

  it("has valid hotkeyOptions and doubleTapOptions", () => {
    expect(translations.ru.hotkeyOptions.length).toBeGreaterThan(0);
    expect(translations.en.hotkeyOptions.length).toBe(translations.ru.hotkeyOptions.length);
    expect(translations.ru.doubleTapOptions.length).toBe(translations.en.doubleTapOptions.length);
  });
});
