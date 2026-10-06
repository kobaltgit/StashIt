export interface StashItem {
  id: string;
  kind: "file" | "folder" | "image" | "text" | "url";
  name: string;
  path: string | null;
  size_bytes: number | null;
  formatted_size: string;
  extension: string;
  text_preview: string | null;
  created_at: number;
}

export interface Shelf {
  id: string;
  name: string;
  pinned: boolean;
  items: StashItem[];
}

export interface ShelfConfig {
  id: string;
  name: string;
  pinned: boolean;
}

export function createDefaultShelf(name = "Основное"): Shelf {
  return {
    id: "main",
    name,
    pinned: true,
    items: [],
  };
}

export function createNewShelf(
  shelves: Shelf[],
  defaultPrefix: (n: number) => string,
  pinned = false
): Shelf {
  const nextNum = shelves.length + 1;
  return {
    id: `shelf_${Date.now()}_${Math.random().toString(36).substring(2, 7)}`,
    name: defaultPrefix(nextNum),
    pinned,
    items: [],
  };
}

export function canCloseShelf(shelf: Shelf, totalShelvesCount: number): boolean {
  if (totalShelvesCount <= 1) return false;
  return !shelf.pinned;
}

export function getTotalItemsCount(shelves: Shelf[]): number {
  return shelves.reduce((sum, s) => sum + s.items.length, 0);
}

export function serializePinnedShelves(shelves: Shelf[]): ShelfConfig[] {
  return shelves
    .filter((s) => s.pinned)
    .map((s) => ({
      id: s.id,
      name: s.name,
      pinned: true,
    }));
}

export function restoreShelvesFromConfig(
  configs: ShelfConfig[] | undefined | null,
  defaultName: string
): Shelf[] {
  if (!configs || configs.length === 0) {
    return [createDefaultShelf(defaultName)];
  }
  return configs.map((c) => ({
    id: c.id,
    name: c.name,
    pinned: true,
    items: [],
  }));
}
