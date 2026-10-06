/**
 * Formats a byte number into a human readable string (Б, КБ, МБ, ГБ).
 */
export function formatBytes(bytes: number): string {
  if (bytes <= 0 || !Number.isFinite(bytes)) return "0 Б";
  const k = 1024;
  const sizes = ["Б", "КБ", "МБ", "ГБ"];
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(k)), sizes.length - 1);
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}
