/**
 * test_rover.js
 * Comprehensive Relay Rover WASM Engine Test & Markdown Report Generator.
 * Displays comparison tables in terminal and writes full detailed tables & JSON responses to TEST_RESULTS.md.
 */

import fs from 'node:fs';
import path from 'node:path';
import { performance } from 'node:perf_hooks';
import { RelayRover, RoverIndex, cleanEmailBody, normalizeEmail, htmlToText } from './packages/rover/dist/index.js';

/**
 * Measure WASM cold-start compilation and warm execution latencies.
 */
function runBenchmark(emails, iterations = 30) {
  const wasmPath = path.resolve('./packages/rover/eng/relay_rover.wasm');
  const wasmBytes = fs.readFileSync(wasmPath);

  // 1. Cold start measurement
  const coldStart0 = performance.now();
  const testModule = new WebAssembly.Module(wasmBytes);
  const testInstance = new WebAssembly.Instance(testModule, {});
  const coldStart1 = performance.now();
  const coldStartMs = coldStart1 - coldStart0;

  // 2. Warmup engine
  for (let i = 0; i < 5; i++) {
    for (const email of emails) {
      RelayRover.process(email);
    }
  }

  // 3. Warm execution measurement
  const latenciesUs = [];
  const totalEmails = emails.length * iterations;

  const tStart = performance.now();
  for (let i = 0; i < iterations; i++) {
    for (const email of emails) {
      const t0 = performance.now();
      RelayRover.process(email);
      const t1 = performance.now();
      latenciesUs.push((t1 - t0) * 1000);
    }
  }
  const tEnd = performance.now();
  const totalDurationMs = tEnd - tStart;

  latenciesUs.sort((a, b) => a - b);
  const p50 = latenciesUs[Math.floor(latenciesUs.length * 0.50)];
  const p95 = latenciesUs[Math.floor(latenciesUs.length * 0.95)];
  const p99 = latenciesUs[Math.floor(latenciesUs.length * 0.99)];
  const min = latenciesUs[0];
  const max = latenciesUs[latenciesUs.length - 1];
  const avg = latenciesUs.reduce((acc, v) => acc + v, 0) / latenciesUs.length;
  const throughput = Math.round((totalEmails / (totalDurationMs / 1000)));

  return {
    coldStartMs: Number(coldStartMs.toFixed(2)),
    totalProcessed: totalEmails,
    totalDurationMs: Number(totalDurationMs.toFixed(2)),
    throughputEmailsPerSec: throughput,
    avgUs: Number(avg.toFixed(1)),
    p50Us: Number(p50.toFixed(1)),
    p95Us: Number(p95.toFixed(1)),
    p99Us: Number(p99.toFixed(1)),
    minUs: Number(min.toFixed(1)),
    maxUs: Number(max.toFixed(1)),
  };
}

/**
 * Generate Markdown comparison tables and full JSON responses report.
 */
function generateMarkdownReport(fiveTypesData, sampleData, outputFile = 'TEST_RESULTS.md') {
  let md = '';

  md += `# Relay Rover — Test & Production Intelligence Report\n\n`;
  md += `> **Engine:** Relay Rover (Zero-LLM Deterministic Rust Regex WebAssembly Engine)  \n`;
  md += `> **Package:** \`@rhq/rover\` (\`react-native-relay-rover\`)  \n`;
  md += `> **Generated At:** ${new Date().toISOString()}  \n`;
  md += `> **Architecture Alignment:** Fully aligned with [*Meet Rover: The Intelligence Behind Relay Mail*](https://blogs.relaymail.online/blog/rover)  \n\n`;

  md += `---\n\n`;

  // ---------------------------------------------------------------------------
  // Benchmark Section
  // ---------------------------------------------------------------------------
  const allEmails = [...(fiveTypesData?.emails || []), ...(sampleData?.emails || [])];
  if (allEmails.length > 0) {
    const bench = runBenchmark(allEmails, 30);
    md += `## 🚀 High-Resolution Performance & Benchmark\n\n`;
    md += `Rover runs directly inside the client JavaScript/React Native runtime via linear WebAssembly memory without child-process overhead or cloud LLM latency.\n\n`;
    md += `| Metric | Measured Value | Unit | Target / SLA |\n`;
    md += `|---|---|---|---|\n`;
    md += `| **Cold Start (Instantiation)** | \`${bench.coldStartMs} ms\` | ms | < 20 ms |\n`;
    md += `| **Median Latency (p50)** | \`${bench.p50Us} µs\` (\`${(bench.p50Us / 1000).toFixed(3)} ms\`) | µs / ms | < 500 µs |\n`;
    md += `| **95th Percentile (p95)** | \`${bench.p95Us} µs\` (\`${(bench.p95Us / 1000).toFixed(3)} ms\`) | µs / ms | < 1,500 µs |\n`;
    md += `| **99th Percentile (p99)** | \`${bench.p99Us} µs\` (\`${(bench.p99Us / 1000).toFixed(3)} ms\`) | µs / ms | < 3,000 µs |\n`;
    md += `| **Average Latency** | \`${bench.avgUs} µs\` | µs | < 800 µs |\n`;
    md += `| **Min Latency** | \`${bench.minUs} µs\` | µs | - |\n`;
    md += `| **Max Latency** | \`${bench.maxUs} µs\` | µs | - |\n`;
    md += `| **Throughput** | **\`${bench.throughputEmailsPerSec.toLocaleString()} emails/sec\`** | emails/s | > 1,000 emails/s |\n`;
    md += `| **Total Benchmark Volume** | \`${bench.totalProcessed} inferences\` | iterations | - |\n\n`;
    md += `---\n\n`;
  }

  // ---------------------------------------------------------------------------
  // Dual-Layer Intelligence Matrix
  // ---------------------------------------------------------------------------
  md += `## 🧠 Dual-Layer Intelligence Matrix\n\n`;
  md += `Rover implements dual-layer classification to bridge traditional inbox folders with purpose-driven intelligence:\n\n`;
  md += `1. **General Category (Folder Layer):** \`Primary\`, \`Social\`, \`Promotions\`, \`Updates\`, \`Forums\`\n`;
  md += `2. **Purpose Category (Action Layer):** \`Authentication\` (OTP), \`Transactions\` (Payment/Invoice), \`Deliveries\` (Shopping), \`Jobs\`, \`SecurityAlerts\`, \`Travel\`, \`Subscriptions\`, \`Social\`, \`Newsletters\`\n`;
  md += `3. **Urgency & Importance Engine:** \`Urgent\` (with alert banner \`is_alert: true\`), \`High\`, \`Normal\`, \`Low\`\n`;
  md += `4. **Security Classification:** \`Authentication\`, \`PasswordReset\`, \`BuildFailure\`, \`SecurityAlert\`, \`AccountChange\`\n`;
  md += `5. **Traceable Confidence:** Multi-factor scoring based on sender validation, entity structure, and pattern specificity.\n\n`;

  // ---------------------------------------------------------------------------
  // Intelligence Index Timelines
  // ---------------------------------------------------------------------------
  if (fiveTypesData && fiveTypesData.results.length > 0) {
    const timelines = RoverIndex.buildTimelines(fiveTypesData.results);
    md += `## 🔗 The Rover Intelligence Index: Connected Timelines\n\n`;
    md += `As outlined in the blog, Rover extracts deterministic \`timeline_key\` identifiers to seamlessly group related communications into continuous timelines:\n\n`;
    for (const [key, tl] of Object.entries(timelines.timelines)) {
      md += `### Timeline: \`${key}\` (${tl.primaryCategory || 'Activity'})\n\n`;
      tl.events.forEach((ev) => {
        md += `- **[${ev.preview}]** — Sender: \`${ev.sender || 'Unknown'}\` | Category: \`${ev.primary_category}\`\n`;
      });
      md += `\n`;
    }
    md += `---\n\n`;
  }

  // ---------------------------------------------------------------------------
  // Section 1: 5 Diverse Real-World Email Types
  // ---------------------------------------------------------------------------
  if (fiveTypesData && fiveTypesData.emails.length > 0) {
    const { emails, results } = fiveTypesData;

    md += `## 1. Real-World Emails: 5 Diverse Categories (\`sample_5_types.json\`)\n\n`;
    md += `Extracted from real emails in \`relay-emails.jsonl\` to test distinct category detection, typed extraction, and contextual preview generation.\n\n`;

    md += `### Summary Comparison Table\n\n`;
    md += `| # | Folder | Purpose | Importance | Contextual Preview | Email Subject & Sender | Extracted Rover Data |\n`;
    md += `|:---:|:---:|:---:|:---:|---|---|---|\n`;

    emails.forEach((mail, idx) => {
      const r = results[idx];
      const norm = normalizeEmail(mail);
      const folder = (r.classification?.general_category || 'Primary').toUpperCase();
      const purpose = r.primary_category.toUpperCase();
      const imp = (r.classification?.importance || 'Normal').toUpperCase();
      const preview = r.preview.replace(/\|/g, '\\|');
      const subject = norm.subject.replace(/\|/g, '\\|').slice(0, 50);
      const from = (norm.senderName ? `${norm.senderName} (${norm.sender})` : norm.sender).replace(/\|/g, '\\|');

      const cleanData = {};
      for (const [k, v] of Object.entries(r.extraction?.data || {})) {
        if (v !== null && v !== '' && v !== undefined && k !== 'raw_evidence') {
          cleanData[k] = v;
        }
      }
      const dataStr = Object.entries(cleanData).map(([k, v]) => `**${k}:** \`${v}\``).join('<br>');

      md += `| **${idx + 1}** | \`${folder}\` | \`${purpose}\` | \`${imp}\` | **${preview}** | ${subject}<br>*From: ${from}* | ${dataStr || '*(None)*'} |\n`;
    });

    md += `\n### Detailed Side-by-Side: Email Body vs Rover JSON Response\n\n`;

    emails.forEach((mail, idx) => {
      const r = results[idx];
      const norm = normalizeEmail(mail);
      const cleanBody = r.cleanBody || cleanEmailBody(mail);

      const cleanData = {};
      for (const [k, v] of Object.entries(r.extraction?.data || {})) {
        if (v !== null && v !== '' && v !== undefined) {
          cleanData[k] = v;
        }
      }

      const roverObject = {
        category: r.primary_category,
        classification: r.classification,
        preview: r.preview,
        timeline_key: r.timeline_key,
        data: cleanData,
        matched_categories: r.matched_categories,
      };

      md += `#### Email #${idx + 1}: [${r.primary_category.toUpperCase()}] ${norm.subject}\n\n`;
      md += `| Attribute | Details |\n`;
      md += `|---|---|\n`;
      md += `| **Sender** | \`${norm.senderName ? `${norm.senderName} <${norm.sender}>` : norm.sender}\` |\n`;
      md += `| **Subject** | ${norm.subject} |\n`;
      md += `| **Folder / General** | \`${r.classification?.general_category || 'Primary'}\` |\n`;
      md += `| **Purpose** | \`${r.primary_category.toUpperCase()}\` |\n`;
      md += `| **Importance / Urgency** | \`${r.classification?.importance || 'Normal'}\` (Alert: \`${r.classification?.is_alert || false}\`) |\n`;
      md += `| **Preview** | "${r.preview}" |\n`;
      if (r.timeline_key) {
        md += `| **Timeline Key** | \`${r.timeline_key}\` |\n`;
      }
      md += `\n`;

      md += `**Email Body Text:**\n`;
      md += `> ${cleanBody.slice(0, 300).replace(/\n/g, '\n> ')}${cleanBody.length > 300 ? '...' : ''}\n\n`;

      md += `**Rover JSON Response:**\n`;
      md += `\`\`\`json\n${JSON.stringify(roverObject, null, 2)}\n\`\`\`\n\n`;
      md += `---\n\n`;
    });
  }

  // ---------------------------------------------------------------------------
  // Section 2: Real Relay Mail Dataset (10 Records)
  // ---------------------------------------------------------------------------
  if (sampleData && sampleData.emails.length > 0) {
    const { emails, results } = sampleData;

    md += `## 2. Real Relay Mail Dataset (\`sample.json\`, 10 Records)\n\n`;
    md += `Real-world Relay emails containing HTML markup, complex multipart headers, digests, notifications, and transactions.\n\n`;

    md += `### Complete Comparison Table\n\n`;
    md += `| # | Subject & Sender | Clean Email Body Snippet | Rover Category & Preview | Extracted Rover Data (JSON) |\n`;
    md += `|:---:|---|---|---|---|\n`;

    emails.forEach((mail, idx) => {
      const r = results[idx];
      const norm = normalizeEmail(mail);
      const cleanBody = (r.cleanBody || cleanEmailBody(mail)).replace(/\|/g, '\\|');
      const bodySnippet = cleanBody.length > 100 ? cleanBody.slice(0, 97) + '...' : cleanBody;
      const sub = norm.subject.replace(/\|/g, '\\|');
      const from = (norm.senderName ? `${norm.senderName}` : norm.sender).replace(/\|/g, '\\|');
      const preview = r.preview.replace(/\|/g, '\\|');

      const cleanData = {};
      for (const [k, v] of Object.entries(r.extraction?.data || {})) {
        if (v !== null && v !== '' && v !== undefined && k !== 'raw_evidence') {
          cleanData[k] = v;
        }
      }
      const jsonStr = `\`${JSON.stringify(cleanData)}\``;

      md += `| **${idx + 1}** | **${sub.slice(0, 40)}**<br>*${from}* | ${bodySnippet} | \`${r.primary_category.toUpperCase()}\`<br>"${preview}" | ${jsonStr} |\n`;
    });

    md += `\n### Full JSON Extraction Dumps (\`sample.json\`)\n\n`;

    emails.forEach((mail, idx) => {
      const r = results[idx];
      const norm = normalizeEmail(mail);
      const cleanData = {};
      for (const [k, v] of Object.entries(r.extraction?.data || {})) {
        if (v !== null && v !== '' && v !== undefined) {
          cleanData[k] = v;
        }
      }

      md += `<details>\n<summary><b>[Email #${idx + 1}] [${r.primary_category.toUpperCase()}] ${norm.subject}</b></summary>\n\n`;
      md += `\`\`\`json\n`;
      md += JSON.stringify({
        id: mail.id || `email_${idx + 1}`,
        from: norm.senderName ? `${norm.senderName} <${norm.sender}>` : norm.sender,
        subject: norm.subject,
        category: r.primary_category,
        classification: r.classification,
        preview: r.preview,
        timeline_key: r.timeline_key,
        extracted: cleanData,
      }, null, 2);
      md += `\n\`\`\`\n\n`;
      md += `</details>\n\n`;
    });
  }

  fs.writeFileSync(outputFile, md, 'utf8');
  console.log(`\n✓ Full detailed report with UI tables and JSON responses generated: ${outputFile}`);
}

/**
 * Display terminal table and return data for report generation.
 */
export function displayEmailTable(filePath) {
  if (!fs.existsSync(filePath)) {
    console.error(`File not found: ${filePath}`);
    return null;
  }

  const raw = JSON.parse(fs.readFileSync(filePath, 'utf8'));
  const emails = Array.isArray(raw) ? raw : [raw];
  const results = RelayRover.processBatch(emails);

  const tableData = emails.map((mail, idx) => {
    const r = results[idx];
    let body = r.cleanBody || cleanEmailBody(mail);
    if (body.length > 90) {
      body = body.slice(0, 87) + '...';
    }

    const cleanData = {};
    for (const [k, v] of Object.entries(r.extraction?.data || {})) {
      if (v !== null && v !== '' && v !== undefined) {
        cleanData[k] = v;
      }
    }

    const rover = {
      category: r.primary_category,
      preview: r.preview,
      data: cleanData,
    };

    const subject = (mail.subject || '(No Subject)').replace(/\r?\n/g, ' ').trim();
    const displaySubject = subject.length > 32 ? subject.slice(0, 29) + '...' : subject;

    return {
      '#': idx + 1,
      'Subject': displaySubject,
      'Email Body': body,
      'rover': JSON.stringify(rover),
    };
  });

  console.table(tableData);
  return { emails, results };
}

// -----------------------------------------------------------------------------
// CLI Execution
// -----------------------------------------------------------------------------
const targetFile = process.argv[2];

if (targetFile) {
  const data = displayEmailTable(targetFile);
  if (data) {
    generateMarkdownReport(data, null, 'TEST_RESULTS.md');
  }
} else {
  let fiveTypesData = null;
  let sampleData = null;

  if (fs.existsSync('sample_5_types.json')) {
    console.log('\n=== REAL EMAILS: 5 DIVERSE CATEGORIES (sample_5_types.json) ===');
    fiveTypesData = displayEmailTable('sample_5_types.json');
  }

  if (fs.existsSync('sample.json')) {
    console.log('\n=== REAL EMAILS: RELAY MAIL RECORDS (sample.json) ===');
    sampleData = displayEmailTable('sample.json');
  }

  generateMarkdownReport(fiveTypesData, sampleData, 'TEST_RESULTS.md');
}
