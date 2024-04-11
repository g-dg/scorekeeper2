export function natcasecmp(a: string, b: string): number {
  return a.localeCompare(b, undefined, { numeric: true, sensitivity: "case" });
}
