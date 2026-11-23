export interface ExtractedAmount {
  value: string;
  currency: string;
  raw: string;
}

export function normalizeCurrency(raw: string): string {
  switch (raw.trim().toUpperCase()) {
    case '$':
    case 'USD':
      return 'USD';
    case '€':
    case 'EUR':
      return 'EUR';
    case '£':
    case 'GBP':
      return 'GBP';
    case '₹':
    case 'INR':
      return 'INR';
    case '¥':
    case 'JPY':
      return 'JPY';
    case 'CAD':
      return 'CAD';
    case 'AUD':
      return 'AUD';
    default:
      return raw.trim().toUpperCase();
  }
}

export function formatCurrency(currency: string, amount: string): string {
  switch (currency.toUpperCase()) {
    case 'INR':
      return `₹${amount}`;
    case 'USD':
      return `$${amount}`;
    case 'EUR':
      return `€${amount}`;
    case 'GBP':
      return `£${amount}`;
    case 'JPY':
      return `¥${amount}`;
    default:
      return `${amount} ${currency}`;
  }
}

export class AmountExtractor {
  // Matches e.g. INR 2,500.00, $49.99, €1200.00, £ 150.50, ₹ 850, ¥15000, CAD 160,000
  private static currencyFirst =
    /(?:^|[^\w])(?<curr>INR|USD|EUR|GBP|JPY|CAD|AUD|₹|\$|€|£|¥)\s*(?<val>\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?|\d+(?:\.\d{1,2})?)\b/i;

  // Matches e.g. 2500.00 INR, 49.99 USD, 1200.00 EUR, 150.50 GBP
  private static currencyAfter =
    /\b(?<val>\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?|\d+(?:\.\d{1,2})?)\s*(?<curr>INR|USD|EUR|GBP|JPY|CAD|AUD)\b/i;

  static extract(text: string): ExtractedAmount | null {
    const match1 = text.match(this.currencyFirst);
    if (match1 && match1.groups?.curr && match1.groups?.val) {
      const cleanVal = match1.groups.val.replace(/,/g, '');
      return {
        value: cleanVal,
        currency: normalizeCurrency(match1.groups.curr),
        raw: match1[0].trim(),
      };
    }

    const match2 = text.match(this.currencyAfter);
    if (match2 && match2.groups?.curr && match2.groups?.val) {
      const cleanVal = match2.groups.val.replace(/,/g, '');
      return {
        value: cleanVal,
        currency: normalizeCurrency(match2.groups.curr),
        raw: match2[0].trim(),
      };
    }

    return null;
  }
}
