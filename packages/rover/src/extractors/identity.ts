export class IdentityExtractor {
  // "payment to QuickMart Retail", "paid to: Tokyo Books Inc", "Paid ₹850 to Chai Point on",
  // "transferred $220.00 to Elena Rostova", "disbursed $8450.00 USD to Nexus Systems"
  private static paidTo =
    /\b(?:payment to|paid to|transferred to|recipient|paid\s+[\$€£₹\w\d.,\s]+?\s+to|sent a payment of [^\n]+ to|transferred [^\n]+ to|disbursed [^\n]+ to|payment of [^\n]+ to|from Merchant)\s*[:]?\s*([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+on|\s+was|\s+for|\s+at|\s+with|\.|\,|\;|\n|Txn|Ref|Status|Date|Transfer|Payment|Mandate)/i;

  // "receipt from Blue Bottle Coffee: $6.75"
  private static receiptFrom =
    /\b(?:digital receipt from|receipt from)\s+([A-Za-z0-9&.'\- ]{2,35}?)(?:\:|\.|\,|\;|\n|$)/i;

  // "Issuer: CloudScale Networks", "Invoice from CloudScale Networks"
  private static issuer =
    /\b(?:Issuer\s*[:]\s*|Invoice(?:\s+[A-Za-z0-9\-#]+)?\s+from\s+)([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+for|\:|\.|\,|\;|\n|$)/i;

  // "Provider: SoundWave Music", "subscription to SoundWave Premium"
  private static provider =
    /\b(?:Provider\s*[:]\s*|subscription to\s+)([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+renews|\.|\,|\;|\n|$)/i;

  // "from ShopQuick", "at Nordic Apparel", "from BookHaven", "ordering with KitchenCraft Supplies"
  private static store =
    /\b(?:shopping at|delivery from|package from|order from|order at|order with|ordering with|purchase from)\s+([A-Za-z0-9&.'\- ]{2,35}?)(?:\s+on|\s+is|\s+has|\s+will|\.|\,|\;|\n|!|$)/i;

  // "Alex Vance sent you an invitation", "Sarah Jenkins reviewed pull request", "David Miller commented"
  private static socialActor =
    /\b([A-Za-z0-9_'\- ]{2,30}?)\s+(?:sent you|mentioned you|reviewed pull request|commented on|liked your|reacted to|shared a post)\b/i;

  static extractPaidTo(text: string): string | null {
    const match = text.match(this.paidTo) || text.match(this.receiptFrom);
    if (match && match[1]) {
      const name = match[1].trim();
      const lower = name.toLowerCase();
      if (lower.startsWith('account ') || lower.startsWith('iban')) {
        return null;
      }
      return name;
    }
    return null;
  }

  static extractIssuer(text: string): string | null {
    const match = text.match(this.issuer);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractProvider(text: string): string | null {
    const match = text.match(this.provider);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractStore(text: string): string | null {
    const match = text.match(this.store);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractSocialActor(text: string): string | null {
    const match = text.match(this.socialActor);
    return match && match[1] ? match[1].trim() : null;
  }
}
