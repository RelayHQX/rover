export class TransactionExtractor {
  // Matches e.g. Txn Ref: TXN-89210492, Transaction ID: PF-994821, Reference: ET-7721839,
  // Mandate Ref: NC-0091823, Authorization code: TX-449102, Ride Ref: CC-109283, Transaction: SP-4910283
  private static txnId =
    /\b(?:Txn Ref|Transaction ID|Ref Number|Transaction Reference|Trans ID|Transfer ID|Gateway Ref|Refund ARN|Batch Ref|UPI Ref|Reference|Mandate Ref|Authorization code|Ride Ref|Transaction)\s*[:#]?\s*([A-Za-z0-9_-]{6,35})\b/i;

  // Matches e.g. Order #SQ-881920, Order ID: ORD-449102, Order number: BH-301928, Order: KC-20491, #NA-55102, Order number: GPA.3399-0949-2150-45472
  private static orderId =
    /\b(?:Order\s+ID|Order\s+number|Order\s*#|Order\s*:\s*#?|Order\s+Confirmation\s*#)\s*[:#]?\s*#?([A-Za-z0-9_.-]{5,35})\b/i;

  // Matches e.g. Tracking number: TRK-9901423, Tracking: SP-10293847, Courier tracking: UF-PARCEL-881
  private static trackingNum =
    /\b(?:Tracking\s*(?:number)?|Courier tracking|Courier)\s*[:#]?\s*([A-Za-z0-9_-]{6,35})\b/i;

  // Matches e.g. Booking ID: SK-78921, PNR SK-78921, Ref #GM-99182, Booking Confirmed: Villa Toscana #CS-1029
  private static bookingId =
    /\b(?:Booking\s+ID|PNR|Reservation\s+(?:Ref|Status)|Ref\s*#|Booking\s+Confirmed\s*:[^\n#]+#|ticket\s+confirmation\s*:\s*#)\s*[:#]?\s*#?([A-Za-z0-9_-]{5,25})\b/i;

  // Matches e.g. Invoice INV-2026-8812, Invoice Number: ME-9910482, Invoice #SLA-2026-301, Invoice GMP-1092, Bill INV-77410
  private static invoiceNum =
    /\b(?:Invoice\s*(?:Number|#)?|bill\s+#?)\s*[:#]?\s*#?([A-Za-z0-9]{2,10}(?:-[A-Za-z0-9]+)+|\d{6,20})\b/i;

  static extractTransactionId(text: string): string | null {
    const match = text.match(this.txnId);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractOrderId(text: string): string | null {
    const match = text.match(this.orderId);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractTrackingNumber(text: string): string | null {
    const match = text.match(this.trackingNum);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractBookingId(text: string): string | null {
    const match = text.match(this.bookingId);
    return match && match[1] ? match[1].trim() : null;
  }

  static extractInvoiceNumber(text: string): string | null {
    const match = text.match(this.invoiceNum);
    return match && match[1] ? match[1].trim() : null;
  }
}
