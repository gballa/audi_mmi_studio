/**
 * TypeScript contract definitions matching Rust Tauri IPC domain types (§15, ADR-002)
 * and workstation domain state for MMI 3G+ customization.
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

/**
 * MMI Localisation string entry with multi-language definitions
 * and font bounding box constraints.
 */
export interface SystemString {
  id: string;
  category: 'Navigation' | 'Media' | 'Radio' | 'Telephone' | 'Car Setup' | 'Climate' | 'System Alerts';
  en: string;
  de: string;
  sq: string; // Gjuha Shqipe (Albanian)
  maxPixels: number; // Target width on 800x480 screen
  context: string;
}

/**
 * Map Database & Structure Info for HBNavDB / Orion ATLAS / MapStyles
 */
export interface MapDatabaseInfo {
  id: string;
  name: string;
  format: 'HBNavDB' | 'Orion ATLAS' | 'MapStyles XAR' | 'Geographic GDB';
  baselineVersion: string;
  sizeBytes: number;
  pageCount: number;
  pageSizeBytes: number;
  checksumType: string;
  modifiability: 'Modifiable (CRC Recalculated)' | 'Modifiable (XAR Unpack/Repack)' | 'Requires Signature Bypass';
  description: string;
}

/**
 * 2026 Road Network / POI update candidate for Albania & Western Balkans
 */
export interface MapUpdateItem {
  id: string;
  title: string;
  region: string;
  type: 'Motorway' | 'Tunnel' | 'Bypass' | 'Speed Matrix' | 'EV Charging Hub' | 'POI Database';
  lengthKm?: number;
  nodesAdded: number;
  description: string;
  speedLimitKmh?: number;
  status: 'Available' | 'Staged' | 'Injected';
  enabled: boolean;
}

export interface RegionalProfileConfig {
  code: string;
  name: string;
  estimatedSize: string;
  volumeCount: number;
  description: string;
}

/**
 * MMI UI System Element for Gemini Nano Banana AI restyling
 */
export interface AiAssetItem {
  id: string;
  name: string;
  category:
    | 'Corner Softkeys'
    | 'Drive Select'
    | 'Status Bar'
    | 'Submenus'
    | 'Navigation'
    | 'Gauges'
    | 'Climate'
    | 'Menu Icons'
    | 'Textures';
  width: number;
  height: number;
  description: string;
  promptSuggestion: string;
  originalStyle: string;
  currentStyle: string;
  aiPromptApplied?: string;
  modelUsed?: string;
  synthIdWatermark: boolean;
  status: 'Stock' | 'AI Modified';
}

/**
 * MMI Screen & UI Component Customization Configuration
 */
export interface MMIThemeConfig {
  accentColor: string;
  backgroundColor: string;
  surfaceColor: string;
  highlightColor: string;
  needleColor: string;
  fontFamily: 'AudiType-Normal' | 'AudiType-Bold' | 'AudiType-Extended' | 'AudiUnivers-540';
  fontSizeScale: number;
  letterSpacingPx: number;
  showCompass: boolean;
  showClock: boolean;
  showTemperature: boolean;
  showStatusBar: boolean;
  showClimateOverlay: boolean;
  ambientGlow: boolean;
  highContrast: boolean;
  language: 'sq' | 'en' | 'de';
  activeCarSilhouetteStyle: string;
  activeBackgroundTexture: 'default' | 'carbon_weave' | 'brushed_aluminum' | 'obsidian_night';
  activeNavArrowStyle: string;

  // 4 Corner Softkeys customization
  cornerBracketColor: string;
  cornerSoftkeys: {
    topLeft: { text: string; action: string; visible: boolean };
    topRight: { text: string; action: string; visible: boolean };
    bottomLeft: { text: string; action: string; visible: boolean };
    bottomRight: { text: string; action: string; visible: boolean };
  };

  // Audi Drive Select State
  activeDriveMode: 'comfort' | 'auto' | 'dynamic' | 'individual';
  driveSelectView: 'platter' | 'settings';
  driveSelectSettings: {
    engineGearbox: 'Comfort' | 'Auto' | 'Dynamic';
    steering: 'Comfort' | 'Auto' | 'Dynamic';
    suspension: 'Comfort' | 'Auto' | 'Dynamic';
  };

  // Status Bar items
  statusBar: {
    clockTime: string;
    isMuted: boolean;
    bluetoothConnected: boolean;
    signalBars: number; // 0..4
    googleServicesOnline: boolean;
    dataNetwork: '3G' | 'LTE' | '2G';
  };
}

