import { describe, it, expect } from "vitest";
import { formatBytes } from "../src/utils";

describe("formatBytes", () => {
  it("formats zero and negative bytes", () => {
    expect(formatBytes(0)).toBe("0 Б");
    expect(formatBytes(-100)).toBe("0 Б");
  });

  it("formats plain bytes (< 1KB)", () => {
    expect(formatBytes(500)).toBe("500 Б");
    expect(formatBytes(1023)).toBe("1023 Б");
  });

  it("formats kilobytes", () => {
    expect(formatBytes(1024)).toBe("1 КБ");
    expect(formatBytes(1536)).toBe("1.5 КБ");
  });

  it("formats megabytes", () => {
    expect(formatBytes(1024 * 1024)).toBe("1 МБ");
    expect(formatBytes(2.5 * 1024 * 1024)).toBe("2.5 МБ");
  });

  it("formats gigabytes", () => {
    expect(formatBytes(1024 * 1024 * 1024)).toBe("1 ГБ");
    expect(formatBytes(4.2 * 1024 * 1024 * 1024)).toBe("4.2 ГБ");
  });
});
