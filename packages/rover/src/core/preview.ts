import type { Category, EmailRecordInput } from '../types/index.js';
import { cleanEmailBody } from './normalizer.js';

/**
 * Format currency with international symbols or code suffix.
 * Matches Rust preview.rs format_currency.
 */
export function formatCurrency(currency: string, amount: string | number): string {
  const currUpper = (currency || '').toUpperCase();
  switch (currUpper) {
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

/**
 * Clean up whitespace, strip newlines, and produce a readable snippet.
 * Matches Rust preview.rs clean_excerpt.
 */
export function cleanExcerpt(text: string): string {
  if (!text) return '';
  return text.replace(/\s+/g, ' ').trim();
}

/**
 * Truncate cleanly at a word boundary with an ellipsis (< maxLen).
 * Matches Rust preview.rs truncate_clean.
 */
export function truncateClean(text: string, maxLen: number = 95): string {
  if (!text) return '';
  if (text.length <= maxLen) return text;

  const end = Math.max(0, maxLen - 3);
  const slice = text.slice(0, end);
  const lastSpace = slice.lastIndexOf(' ');
  if (lastSpace > 0) {
    return `${slice.slice(0, lastSpace)}...`;
  }
  return `${slice}...`;
}

/**
 * Generate a concise snippet/summary from an email body for inbox listings.
 */
export function extractSummary(email: EmailRecordInput | string, maxLength: number = 100): string {
  const text = typeof email === 'string' ? email : cleanEmailBody(email);
  return truncateClean(cleanExcerpt(text), maxLength);
}

/**
 * Generate a concise, contextual preview (< 100 chars) for a processed email.
 * Replicates Rust preview.rs generate_preview logic with 100% parity.
 */
export function generatePreview(
  category: Category | string,
  data: any,
  subject: string,
  body: string
): string {
  const d = data || {};
  let preview = '';

  const cat = (category || '').toLowerCase();

  switch (cat) {
    case 'payment': {
      const amountStr = d.currency && d.amount
        ? formatCurrency(d.currency, d.amount)
        : d.amount
          ? `${d.amount}`
          : 'Payment';
      const recipientStr = d.recipient || 'Merchant';
      const statusStr = d.transaction_status || d.status || 'Processed';
      preview = `${amountStr} paid to ${recipientStr} · ${statusStr}`;
      break;
    }

    case 'otp': {
      const service = d.service_name || d.service || 'Service';
      const code = d.otp_code || d.code || '------';
      const exp = d.expiration_time || d.expires_in;
      if (exp) {
        preview = `${service} verification code: ${code} (Expires in ${exp})`;
      } else {
        preview = `${service} verification code: ${code}`;
      }
      break;
    }

    case 'job': {
      const pos = d.position || 'Role';
      const company = d.company || 'Company';
      const subLower = (subject || '').toLowerCase();
      if (d.is_job_alert || subLower.includes('job alert') || subLower.includes('jobs for you')) {
        let countDesc = `${pos} at ${company}`;
        if (d.job_count === 1) {
          countDesc = `${pos} at ${company}`;
        } else if (d.job_count) {
          countDesc = `${d.job_count} ${pos} jobs at ${company}`;
        }
        preview = `Job Alert · ${countDesc}`;
      } else if (subLower.includes('offer')) {
        preview = `Job offer · ${pos} at ${company}`;
      } else if (subLower.includes('interview')) {
        preview = `Interview invitation · ${pos} at ${company}`;
      } else {
        preview = `Application update · ${pos}`;
      }
      break;
    }

    case 'shopping': {
      const store = d.store || 'Order';
      const amountStr = d.currency && d.amount
        ? formatCurrency(d.currency, d.amount)
        : d.amount
          ? `$${d.amount}`
          : '';
      const status = d.delivery_status || d.status || 'Order confirmed';

      if (d.product) {
        if (amountStr) {
          preview = `${amountStr} · ${d.product} (${store})`;
        } else if (d.tracking_number) {
          preview = `${status} · ${d.product} (Tracking available)`;
        } else {
          preview = `${status} · ${d.product}`;
        }
      } else if (amountStr) {
        preview = `${amountStr} · ${store} purchase`;
      } else if (d.tracking_number) {
        preview = `${status} · Tracking available`;
      } else {
        preview = `${store} · ${status}`;
      }
      break;
    }

    case 'securityalert':
    case 'security_alert': {
      const alertType = d.alert_type || 'Security Alert';
      if (d.target_resource) {
        preview = `⚠️ ${alertType} · ${d.target_resource}`;
      } else {
        preview = `⚠️ ${alertType}`;
      }
      break;
    }

    case 'invoice': {
      const amountStr = d.currency && d.amount
        ? formatCurrency(d.currency, d.amount)
        : 'Invoice';
      const due = d.due_date;
      if (due) {
        preview = `Invoice received · ${amountStr} due ${due}`;
      } else {
        preview = `Invoice received · ${amountStr}`;
      }
      break;
    }

    case 'subscription': {
      const provider = d.provider || d.service || 'Subscription';
      const freq = d.billing_frequency || d.frequency || 'recurring';
      const renewal = d.renewal_date;
      if (renewal) {
        preview = `${provider} · ${freq} renewal on ${renewal}`;
      } else {
        preview = `${provider} · ${freq} renewal`;
      }
      break;
    }

    case 'travel': {
      const dest = d.destination || 'Trip';
      const status = d.booking_status || d.status || 'Confirmed';
      const dep = d.departure_date;
      if (dep) {
        preview = `Booking to ${dest} · ${status} for ${dep}`;
      } else {
        preview = `Booking to ${dest} · ${status}`;
      }
      break;
    }

    case 'newsletter': {
      const pubName = d.publisher || 'Newsletter';
      const topicOrHeadline = d.headline || d.main_topic || 'Latest updates';
      preview = `${pubName} · ${topicOrHeadline}`;
      break;
    }

    case 'social': {
      const platform = d.platform || 'Social';
      const activity = d.activity_type || 'Activity';
      const actor = d.actor || d.relevant_entity;
      if (actor) {
        preview = `${platform} · ${activity} from ${actor}`;
      } else {
        preview = `${platform} · ${activity}`;
      }
      break;
    }

    case 'developer': {
      if (d.package && d.version) {
        preview = `${d.status === 'failed' ? 'Build failed' : 'Published'}: ${d.package}@${d.version}`;
      } else {
        preview = cleanExcerpt(subject) || 'Developer update';
      }
      break;
    }

    case 'general': {
      const clean = cleanExcerpt(body);
      preview = clean.length > 0 ? clean : cleanExcerpt(subject);
      break;
    }

    default: {
      const clean = cleanExcerpt(body);
      preview = clean.length > 0 ? clean : cleanExcerpt(subject);
      break;
    }
  }

  return truncateClean(preview, 95);
}
