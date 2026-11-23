export interface ExtractedOtp {
  code: string;
  service: string | null;
  expiresIn: string | null;
}

export class OtpExtractor {
  // Pattern 1: "verification code is 482913", "security code 918204", "OTP is 334812", "Wire Authorization: 334812"
  private static codePattern1 =
    /\b(?:verification\s+code|security\s+code|login\s+code|otp|passcode|pin|authorization(?:\s+code)?)\s*(?:is|:|\b)\s*([0-9]{4,8})\b/i;

  // Pattern 2: "code: 772019", "OTP: 649201", "PIN: 554109"
  private static codePattern2 = /\b(?:code|otp|pin)\s*[:#]\s*([0-9]{4,8})\b/i;

  // Pattern 3: "820155 is your account recovery code"
  private static codePattern3 = /\b([0-9]{4,8})\s+is\s+your\b/i;

  // Pattern 4: "Enter security code 491028", "Enter verification code 772019", "Enter OTP 491028"
  private static codePattern4 =
    /\benter\s+(?:the\s+)?(?:security\s+code|verification\s+code|otp|authorization\s+code|pin)?\s*([0-9]{4,8})\b/i;

  // Expiry pattern: "expires in 10 minutes", "valid for 5 minutes", "active for 20 minutes"
  private static expiryPattern =
    /\b(?:valid\s+for|expires\s+in|active\s+for|expire\s+in)\s+([0-9]+\s+(?:minutes?|mins?|hours?|seconds?))\b/i;

  // Expiry within pattern: "within 24 hours"
  private static expiryWithin =
    /\b(?:within|expires\s+in|valid\s+for)\s+(\d+\s+(?:hours?|minutes?|mins?|days?))\b/i;

  // Service pattern: "for Acme Portal is", "sign in to your VaultCloud", "access to GamingNetwork"
  private static servicePattern =
    /\b(?:for|to your|on|to access|access to|sign in to your|withdrawal from|session on)\s+([A-Za-z0-9]{3,25}(?:\s+[A-Za-z0-9]{3,20})?)\b/i;

  static extract(subject: string, body: string): ExtractedOtp | null {
    const text = `${subject}\n${body}`;

    // 1. Bracket in subject e.g. [0313] Nintendo Account
    const bracketMatch = subject.match(/\[([0-9]{4,8})\]/);

    const codeMatch =
      bracketMatch ||
      text.match(this.codePattern1) ||
      text.match(this.codePattern2) ||
      text.match(this.codePattern3) ||
      text.match(this.codePattern4) ||
      text.match(/\b([0-9]{4,8})\b/);

    if (!codeMatch || !codeMatch[1]) return null;

    const code = codeMatch[1];

    const expMatch = text.match(this.expiryPattern) || text.match(this.expiryWithin);
    const expiresIn = expMatch ? expMatch[1] : null;

    let service: string | null = null;
    const servMatch = text.match(this.servicePattern);
    if (servMatch && servMatch[1]) {
      let s = servMatch[1].trim();
      if (s.toLowerCase().endsWith(' account')) {
        s = s.slice(0, -8).trim();
      }
      if (
        !s.toLowerCase().includes('corporate resources') &&
        !s.toLowerCase().includes('wire authorization') &&
        s !== 'Linux' &&
        s !== 'security' &&
        !s.toLowerCase().startsWith('creating')
      ) {
        service = s;
      }
    }

    return {
      code,
      service,
      expiresIn,
    };
  }
}
