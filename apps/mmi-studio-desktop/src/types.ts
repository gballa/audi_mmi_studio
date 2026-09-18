/**
 * TypeScript contract definitions matching Rust Tauri IPC domain types (§15, ADR-002).
 */

export interface InspectResult {
  filePath: string;
  sizeBytes: number;
  blake3Hash: string;
  detectedFormat: string;
  thumbnailBlobId?: string;
}

export interface HexRow {
  offset: number;
  hexBytes: string[];
  ascii: string;
}

export interface HexDumpResult {
  offset: number;
  length: number;
  rows: HexRow[];
}

export interface EntropyResult {
  averageEntropy: number;
  classification: string;
  segments: [number, number][];
}

export interface ScreenRenderResult {
  width: number;
  height: number;
  activeMode: 'Day' | 'Night';
  pixelCount: number;
}
