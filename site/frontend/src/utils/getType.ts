export function getTypeString(obj: any): string {
  return Object.prototype.toString
    .call(obj)
    .toLowerCase()
    .replace("[", "")
    .replace("]", "")
    .split(" ")[1];
}
