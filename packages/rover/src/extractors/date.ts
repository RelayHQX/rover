export class DateExtractor {
  // Matches 2026-11-05
  private static isoDate = /\b(\d{4})-(\d{2})-(\d{2})\b/;

  // Matches 11/05/2026 or 05/11/2026
  private static slashDate = /\b(\d{2})\/(\d{2})\/(\d{4})\b/;

  // Matches e.g. 15 November 2026, Nov 12, 2026
  private static verbalDate =
    /\b(\d{1,2})(?:st|nd|rd|th)?\s+(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)(?:\s+(\d{4}))?\b/i;

  static extract(text: string): string | null {
    // Check ISO format first
    const isoMatch = text.match(this.isoDate);
    if (isoMatch) return isoMatch[0];

    // Check verbal format
    const verbalMatch = text.match(this.verbalDate);
    if (verbalMatch) {
      const day = parseInt(verbalMatch[1], 10);
      const month = verbalMatch[2].toLowerCase();
      const year = verbalMatch[3] || '2026';
      const monthNum = monthToNum(month);
      return `${year}-${String(monthNum).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
    }

    // Check slash format
    const slashMatch = text.match(this.slashDate);
    if (slashMatch) return slashMatch[0];

    return null;
  }
}

function monthToNum(month: string): number {
  if (month.startsWith('jan')) return 1;
  if (month.startsWith('feb')) return 2;
  if (month.startsWith('mar')) return 3;
  if (month.startsWith('apr')) return 4;
  if (month.startsWith('may')) return 5;
  if (month.startsWith('jun')) return 6;
  if (month.startsWith('jul')) return 7;
  if (month.startsWith('aug')) return 8;
  if (month.startsWith('sep')) return 9;
  if (month.startsWith('oct')) return 10;
  if (month.startsWith('nov')) return 11;
  if (month.startsWith('dec')) return 12;
  return 1;
}
