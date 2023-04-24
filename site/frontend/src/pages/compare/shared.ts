export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  function padStr(i) {
    return (i < 10) ? "0" + i : "" + i;

  }
  return `${date.getUTCFullYear()}-${padStr(date.getUTCMonth() + 1)}-${padStr(date.getUTCDate())} `;
}
