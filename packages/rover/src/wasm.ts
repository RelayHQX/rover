/**
 * @rhq/rover/wasm
 * Optional WebAssembly distribution target for Relay Rover.
 * Only loaded when explicitly imported via '@rhq/rover/wasm'.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import type { EmailRecordInput, ProcessedEmail } from './types/index.js';
import {
  htmlToText as jsHtmlToText,
  cleanEmailBody as jsCleanBody,
  normalizeEmail as jsNormalize,
  extractSummary as jsSummary,
  RoverIndex,
  process as jsProcess,
  processBatch as jsProcessBatch,
} from './index.js';

let wasmInstance: WebAssembly.Instance | null = null;
let wasmExports: any = null;
let memory: WebAssembly.Memory | null = null;

export function init(customBytes?: ArrayBuffer | Uint8Array): any {
  if (wasmExports) {
    return wasmExports;
  }

  let bytes: any = customBytes;

  if (!bytes) {
    try {
      const __filename = fileURLToPath(import.meta.url);
      const __dirname = path.dirname(__filename);
      const possiblePaths = [
        path.resolve(__dirname, 'eng/relay_rover.wasm'),
        path.resolve(__dirname, '../eng/relay_rover.wasm'),
        path.resolve(__dirname, '../../eng/relay_rover.wasm'),
      ];
      for (const p of possiblePaths) {
        if (fs.existsSync(p)) {
          bytes = fs.readFileSync(p);
          break;
        }
      }
    } catch (_) {}
  }

  if (!bytes && typeof globalThis !== 'undefined' && (globalThis as any).__RELAY_ROVER_WASM__) {
    bytes = (globalThis as any).__RELAY_ROVER_WASM__;
  }

  if (!bytes) {
    // If WASM binary not found, fall back gracefully to TypeScript implementation
    return null;
  }

  const module = new WebAssembly.Module(bytes);
  wasmInstance = new WebAssembly.Instance(module, {});
  wasmExports = wasmInstance.exports;
  memory = wasmExports.memory as WebAssembly.Memory;
  return wasmExports;
}

function getExports(): any {
  if (!wasmExports) {
    init();
  }
  return wasmExports;
}

function writeStringToMemory(exp: any, mem: WebAssembly.Memory, str: string) {
  const encoder = new TextEncoder();
  const encoded = encoder.encode(str || '');
  const ptr = exp.rover_alloc(encoded.length);
  const heap = new Uint8Array(mem.buffer, ptr, encoded.length);
  heap.set(encoded);
  return { ptr, len: encoded.length };
}

function readStringFromMemory(exp: any, mem: WebAssembly.Memory, ptr: number): string {
  const view = new DataView(mem.buffer, ptr);
  const len = view.getUint32(0, true);
  const bytes = new Uint8Array(mem.buffer, ptr + 4, len);
  const text = new TextDecoder('utf-8').decode(bytes);
  exp.rover_dealloc(ptr, len + 4);
  return text;
}

export function classify(subject: string, body: string): string {
  const exp = getExports();
  if (exp && typeof exp.rover_classify === 'function' && memory) {
    const sub = writeStringToMemory(exp, memory, subject);
    const bod = writeStringToMemory(exp, memory, body);
    const resPtr = exp.rover_classify(sub.ptr, sub.len, bod.ptr, bod.len);
    const category = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(sub.ptr, sub.len);
    exp.rover_dealloc(bod.ptr, bod.len);
    return category;
  }
  return 'General';
}

export function generatePreview(subject: string, body: string): string {
  const exp = getExports();
  if (exp && typeof exp.rover_preview === 'function' && memory) {
    const sub = writeStringToMemory(exp, memory, subject);
    const bod = writeStringToMemory(exp, memory, body);
    const resPtr = exp.rover_preview(sub.ptr, sub.len, bod.ptr, bod.len);
    const preview = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(sub.ptr, sub.len);
    exp.rover_dealloc(bod.ptr, bod.len);
    return preview;
  }
  return '';
}

export function processEmail(subject: string, body: string): ProcessedEmail {
  const exp = getExports();
  if (exp && typeof exp.rover_process_email === 'function' && memory) {
    const sub = writeStringToMemory(exp, memory, subject);
    const bod = writeStringToMemory(exp, memory, body);
    const resPtr = exp.rover_process_email(sub.ptr, sub.len, bod.ptr, bod.len);
    const jsonStr = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(sub.ptr, sub.len);
    exp.rover_dealloc(bod.ptr, bod.len);
    return JSON.parse(jsonStr);
  }
  return jsProcess({ subject, body });
}

export function process(input: EmailRecordInput | string): ProcessedEmail {
  const exp = getExports();
  if (!exp) {
    return jsProcess(input);
  }
  if (typeof input === 'string') {
    const clean = jsCleanBody(input);
    const res = processEmail('', clean);
    return { ...res, cleanBody: clean };
  }
  if (input && typeof input === 'object') {
    const normalized = jsNormalize(input);
    let res: ProcessedEmail;
    if (exp && typeof exp.rover_process_email_json === 'function' && memory) {
      const jsonStr = JSON.stringify(normalized);
      const inMem = writeStringToMemory(exp, memory, jsonStr);
      const resPtr = exp.rover_process_email_json(inMem.ptr, inMem.len);
      const outStr = readStringFromMemory(exp, memory, resPtr);
      exp.rover_dealloc(inMem.ptr, inMem.len);
      res = JSON.parse(outStr);
    } else {
      res = processEmail(normalized.subject, normalized.body);
    }
    return {
      ...res,
      id: normalized.id,
      cleanBody: normalized.body,
      sender: normalized.sender,
      senderName: normalized.senderName,
    };
  }
  throw new TypeError('Expected email object or string');
}

export function processBatch(emails: EmailRecordInput[] | string): ProcessedEmail[] {
  const exp = getExports();
  if (!exp) {
    return jsProcessBatch(emails);
  }
  const emailsArray = typeof emails === 'string' ? JSON.parse(emails) : emails;
  const normalizedArray = emailsArray.map((m: any) =>
    typeof m === 'object' && m !== null ? jsNormalize(m) : m
  );

  if (exp && typeof exp.rover_process_batch_json === 'function' && memory) {
    const jsonString = JSON.stringify(normalizedArray);
    const input = writeStringToMemory(exp, memory, jsonString);
    const resPtr = exp.rover_process_batch_json(input.ptr, input.len);
    const resultsJson = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(input.ptr, input.len);

    const parsed = JSON.parse(resultsJson);
    if (Array.isArray(parsed)) {
      return parsed.map((item: any, idx: number) => ({
        ...item,
        cleanBody: normalizedArray[idx]?.body || '',
      }));
    }
    return parsed;
  }

  return normalizedArray.map((m: any) => process(m));
}

export {
  RoverIndex,
  jsHtmlToText as htmlToText,
  jsCleanBody as cleanEmailBody,
  jsNormalize as normalizeEmail,
  jsSummary as extractSummary,
};

export const RelayRoverWasm = {
  init,
  classify,
  generatePreview,
  processEmail,
  process,
  processBatch,
  htmlToText: jsHtmlToText,
  cleanEmailBody: jsCleanBody,
  normalizeEmail: jsNormalize,
  extractSummary: jsSummary,
  index: RoverIndex,
};

export default RelayRoverWasm;
