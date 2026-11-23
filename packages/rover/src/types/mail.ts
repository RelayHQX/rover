import type { ProcessedEmail } from './index.js';

export interface FullEmail {
  id: string;
  from?: any;
  to?: any;
  subject?: string;
  date?: string;
  timestamp?: number;
  internalDate?: string;
  html?: string;
  text?: string;
  snippet?: string;
  headers?: any;
  rover?: ProcessedEmail;
  [key: string]: any;
}

export interface InboxMessage {
  id: string;
  from?: string;
  senderEmail?: string;
  senderName?: string;
  subject?: string;
  date?: string;
  snippet?: string;
  [key: string]: any;
}
