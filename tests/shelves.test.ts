import { describe, it, expect } from "vitest";
import {
  createDefaultShelf,
  createNewShelf,
  canCloseShelf,
  getTotalItemsCount,
  serializePinnedShelves,
  restoreShelvesFromConfig,
  type Shelf,
} from "../src/shelves";

describe("Shelves Management", () => {
  it("creates a default pinned shelf", () => {
    const main = createDefaultShelf("Основное");
    expect(main.id).toBe("main");
    expect(main.name).toBe("Основное");
    expect(main.pinned).toBe(true);
    expect(main.items).toEqual([]);
  });

  it("creates a new shelf with incremental name", () => {
    const list: Shelf[] = [createDefaultShelf("Основное")];
    const shelf2 = createNewShelf(list, (n) => `Полка ${n}`);
    expect(shelf2.name).toBe("Полка 2");
    expect(shelf2.pinned).toBe(false);
    expect(shelf2.id).not.toBe("main");
  });

  it("disallows closing the last remaining shelf or a pinned shelf", () => {
    const main = createDefaultShelf("Основное");
    // Only 1 shelf -> cannot close
    expect(canCloseShelf(main, 1)).toBe(false);

    // 2 shelves, but main is pinned -> cannot close
    expect(canCloseShelf(main, 2)).toBe(false);

    // 2 shelves, shelf2 is unpinned -> CAN close
    const shelf2: Shelf = { id: "2", name: "Полка 2", pinned: false, items: [] };
    expect(canCloseShelf(shelf2, 2)).toBe(true);
  });

  it("calculates total items across all shelves", () => {
    const s1: Shelf = {
      id: "1",
      name: "S1",
      pinned: true,
      items: [
        {
          id: "i1",
          kind: "file",
          name: "f1.txt",
          path: "/a",
          size_bytes: 10,
          formatted_size: "10 Б",
          extension: "txt",
          text_preview: null,
          created_at: 0,
        },
      ],
    };
    const s2: Shelf = {
      id: "2",
      name: "S2",
      pinned: false,
      items: [
        {
          id: "i2",
          kind: "file",
          name: "f2.txt",
          path: "/b",
          size_bytes: 20,
          formatted_size: "20 Б",
          extension: "txt",
          text_preview: null,
          created_at: 0,
        },
        {
          id: "i3",
          kind: "file",
          name: "f3.txt",
          path: "/c",
          size_bytes: 30,
          formatted_size: "30 Б",
          extension: "txt",
          text_preview: null,
          created_at: 0,
        },
      ],
    };
    expect(getTotalItemsCount([s1, s2])).toBe(3);
    expect(getTotalItemsCount([])).toBe(0);
  });

  it("serializes and restores pinned shelves", () => {
    const s1: Shelf = { id: "1", name: "Основное", pinned: true, items: [] };
    const s2: Shelf = { id: "2", name: "Временная", pinned: false, items: [] };
    const s3: Shelf = { id: "3", name: "В печать", pinned: true, items: [] };

    const serialized = serializePinnedShelves([s1, s2, s3]);
    expect(serialized).toEqual([
      { id: "1", name: "Основное", pinned: true },
      { id: "3", name: "В печать", pinned: true },
    ]);

    const restored = restoreShelvesFromConfig(serialized, "Основное");
    expect(restored.length).toBe(2);
    expect(restored[0].name).toBe("Основное");
    expect(restored[1].name).toBe("В печать");
    expect(restored[0].pinned).toBe(true);
    expect(restored[1].pinned).toBe(true);
  });

  it("restores default shelf when configs list is empty", () => {
    const restored = restoreShelvesFromConfig([], "Main");
    expect(restored.length).toBe(1);
    expect(restored[0].id).toBe("main");
    expect(restored[0].name).toBe("Main");
  });
});
