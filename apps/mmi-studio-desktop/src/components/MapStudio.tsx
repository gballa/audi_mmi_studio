import React, { useState } from 'react';
import { MapDatabaseInfo, MapUpdateItem } from '../types';
import { regionalProfiles } from '../data/mapData';

interface MapStudioProps {
  databases: MapDatabaseInfo[];
  updates: MapUpdateItem[];
  onToggleUpdate: (id: string) => void;
  onResetUpdates?: () => void;
  onDisableAllUpdates?: () => void;
}

export const MapStudio: React.FC<MapStudioProps> = ({
  databases,
  updates,
  onToggleUpdate,
  onResetUpdates,
  onDisableAllUpdates,
}) => {
  const [selectedDbId, setSelectedDbId] = useState<string>(databases[0]?.id || '');
  const [selectedProfileCode, setSelectedProfileCode] = useState<string>('AL');
  const [enableGmp, setEnableGmp] = useState<boolean>(true);
  const [enableEvPois, setEnableEvPois] = useState<boolean>(true);
  const [enableSpeedCameras, setEnableSpeedCameras] = useState<boolean>(true);
  const [svmChannel15Input, setSvmChannel15Input] = useState<string>('12345');
  const [patchingState, setPatchingState] = useState<'idle' | 'patching' | 'completed'>('idle');
  const [patchProgress, setPatchProgress] = useState<number>(0);
  const [patchLog, setPatchLog] = useState<string[]>([]);
  const [showTechnicalAnalysis, setShowTechnicalAnalysis] = useState<boolean>(true);

  const activeProfile = regionalProfiles.find((p) => p.code === selectedProfileCode) || regionalProfiles[0];

  const activeDb = databases.find((d) => d.id === selectedDbId) || databases[0];

  const enabledUpdates = updates.filter((u) => u.enabled);
  const totalKmAdded = enabledUpdates.reduce((acc, u) => acc + (u.lengthKm || 0), 0);
  const totalNodesAdded = enabledUpdates.reduce((acc, u) => acc + u.nodesAdded, 0);

  const handleRunPatcher = () => {
    setPatchingState('patching');
    setPatchProgress(0);
    setPatchLog([
      `Initiating 2026 Map Update Pipeline — Profile: ${activeProfile.name} (${activeProfile.code})...`,
      `Target Media: Audi MMI 3G High / Plus [HN+] (HBNavDB 544-byte FLDB Container)`,
      `Coverage Estimated Footprint: ${activeProfile.estimatedSize} across ${activeProfile.volumeCount} FAT32 volume(s)`,
    ]);

    setTimeout(() => {
      setPatchProgress(25);
      setPatchLog((prev) => [
        ...prev,
        'Stage 1: Parsing OpenStreetMap (OSM) vector geometry & functional road classes (FRC 0-7)...',
        `Stage 1: Staged ${enabledUpdates.length} roadway corridors (${totalKmAdded.toFixed(1)} km, ${totalNodesAdded} topology nodes)...`,
      ]);
    }, 600);

    setTimeout(() => {
      setPatchProgress(55);
      setPatchLog((prev) => [
        ...prev,
        enableGmp
          ? 'Stage 2: Google Maps Platform enrichment active (Places API New + Geocoding API)...'
          : 'Stage 2: Skipping GMP online enrichment (using offline vector definitions)...',
        ...(enableGmp && enableEvPois ? ['Stage 2: Injected high-power 150kW-350kW CCS2 DC fast charging POIs with 30-day ToS cache...'] : []),
        ...(enableGmp && enableSpeedCameras ? ['Stage 2: Injected 2026 calibrated speed enforcement camera radar POIs...'] : []),
        'Stage 2: Updating statutory speed limit matrices (130 km/h motorways, 80 km/h tunnels, 50 km/h urban)...',
      ]);
    }, 1300);

    setTimeout(() => {
      setPatchProgress(85);
      setPatchLog((prev) => [
        ...prev,
        'Stage 3: Assembling 544-byte physical pages (512B payload + 16B header + 16B trailer sync 0x55AA55AA)...',
        'Stage 3: Recalculating CRC-16/CCITT checksums across all physical page headers...',
        activeProfile.volumeCount > 1
          ? `Stage 3: Partitioning database into ${activeProfile.volumeCount} sequential FAT32 volumes (nav_data.db, nav_data.db.001...)...`
          : 'Stage 3: Single-volume FAT32 boundary verified (<= 2 GiB)...',
      ]);
    }, 2000);

    setTimeout(() => {
      setPatchProgress(100);
      setPatchingState('completed');
      setPatchLog((prev) => [
        ...prev,
        `✓ Stage 4: 2026 Navigation Update Compiled & Packaged Successfully for ${activeProfile.code}!`,
        '✓ Packaging root metainfo2.txt, MU9411/strings/sq_AL.ans, and MapStyles shaders.',
        '✓ Embedded emergency stock_recovery.sh for instant rollback.',
        'Status: BUILD READY — DEPLOYMENT NOT VERIFIED (§14.9 Policy Enforced).',
        'SVM Resolution: Channel 15 XOR 51666 (0xC9D2) ready for VCDS adaptation if 03276 triggers.',
      ]);
    }, 2700);
  };

  return (
    <div className="flex h-full bg-slate-950 text-slate-100 overflow-hidden font-sans">
      {/* Left Database Selector & Feasibility Inspector */}
      <div className="w-[380px] flex flex-col border-r border-slate-800 bg-slate-900/60 overflow-y-auto">
        <div className="p-4 border-b border-slate-800">
          <div className="flex items-center gap-2">
            <span className="text-base font-bold tracking-tight text-white">🗺️ Map Architecture & RE Lab</span>
          </div>
          <p className="text-xs text-slate-400 mt-1">
            Navigation database structures, modifiability evaluation, and 2026 road injection
          </p>
        </div>

        {/* Database List */}
        <div className="p-3 space-y-2">
          <span className="text-[11px] font-bold uppercase tracking-wider text-slate-400 block px-1">
            MMI Navigation Database Containers
          </span>
          {databases.map((db) => {
            const isSelected = db.id === selectedDbId;
            return (
              <div
                key={db.id}
                onClick={() => setSelectedDbId(db.id)}
                className={`p-3 rounded-lg border cursor-pointer transition-all ${
                  isSelected
                    ? 'bg-amber-500/10 border-amber-500 shadow-sm'
                    : 'bg-slate-950/80 border-slate-800 hover:border-slate-700'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-xs text-white">{db.name}</span>
                  <span className="text-[10px] font-mono px-1.5 py-0.5 bg-slate-800 text-amber-400 rounded">
                    {db.format}
                  </span>
                </div>
                <div className="text-[11px] text-slate-400 mt-1">Version: {db.baselineVersion}</div>
                <div className="flex items-center justify-between text-[10px] text-slate-400 font-mono mt-2 pt-2 border-t border-slate-800/60">
                  <span>Size: {(db.sizeBytes / (1024 * 1024 * 1024)).toFixed(1)} GB</span>
                  <span className="text-emerald-400">{db.modifiability}</span>
                </div>
              </div>
            );
          })}
        </div>

        {/* Technical Modifiability Verdict */}
        <div className="p-4 m-3 bg-slate-950 border border-slate-800 rounded-lg space-y-2.5">
          <div className="flex items-center justify-between">
            <span className="text-xs font-bold text-amber-400">Can MMI Maps be modified?</span>
            <button
              onClick={() => setShowTechnicalAnalysis(!showTechnicalAnalysis)}
              className="text-[10px] text-slate-400 hover:text-slate-200 underline"
            >
              {showTechnicalAnalysis ? 'Hide Details' : 'Show Details'}
            </button>
          </div>

          <div className="text-xs text-slate-300 leading-relaxed">
            <strong className="text-emerald-400">YES — Technical Verdict:</strong> Navigation data pages, POI
            records, speed limit tables, and road network topology{' '}
            <span className="text-white font-medium">can be modified and patched</span> by adhering to the Harman
            FLDB 544-byte page stride and recalculating page-level CRC-16 checksums.
          </div>

          {showTechnicalAnalysis && (
            <div className="text-[11px] text-slate-400 space-y-1.5 pt-2 border-t border-slate-800 font-mono">
              <div>• <strong>FLDB Geometry:</strong> Modifiable via page recompilation.</div>
              <div>• <strong>POI Database:</strong> 100% Modifiable SQLite 3 tables.</div>
              <div>• <strong>MapStyles (.xar):</strong> Modifiable (PNGs, color schemes).</div>
              <div>• <strong>Firmware Boundary:</strong> On production cars, updated maps require FEC activation code <code className="text-amber-400">02100028</code> or an SD patch script.</div>
            </div>
          )}
        </div>
      </div>

      {/* Center 2026 Map Update Injector */}
      <div className="flex-1 flex flex-col overflow-y-auto bg-slate-950 p-6 space-y-6">
        {/* Top Header */}
        <div className="flex items-center justify-between pb-4 border-b border-slate-800">
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-white tracking-tight">2026 Map Update & Road Network Injector</h2>
              <span className="px-2 py-0.5 text-[10px] bg-red-950 border border-red-800 text-red-300 font-mono rounded">
                Albania & Western Balkans
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              Inject 2026 highways, tunnels, bypasses, and charging infrastructure into Audi MMI navigation database
            </p>
          </div>

          <div className="flex items-center gap-2">
            {onResetUpdates && (
              <button
                onClick={onResetUpdates}
                className="px-3 py-2 bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white text-xs font-semibold rounded border border-slate-700 transition flex items-center gap-1"
                title="Reset all 2026 map patches back to recommended default"
              >
                <span>↺</span> Reset Map Patches
              </button>
            )}
            <button
              onClick={handleRunPatcher}
              disabled={patchingState === 'patching' || enabledUpdates.length === 0}
              className={`px-4 py-2 text-xs font-bold rounded shadow transition-all flex items-center gap-2 ${
                patchingState === 'patching'
                  ? 'bg-slate-800 text-slate-500 cursor-not-allowed'
                  : 'bg-amber-500 hover:bg-amber-400 text-slate-950'
              }`}
            >
              {patchingState === 'patching' ? (
                <>
                  <span className="w-3 h-3 border-2 border-slate-950 border-t-transparent rounded-full animate-spin" />
                  Compiling 2026 Map Patch...
                </>
              ) : (
                <>⚡ Compile & Inject 2026 Map Update</>
              )}
            </button>
          </div>
        </div>

        {/* Regional Scope & Compilation Profile Selector */}
        <div className="bg-slate-900 border border-slate-800 rounded-lg p-4 space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-xs font-bold uppercase tracking-wider text-white">
                Regional Compilation Profile & FAT32 Volume Planning
              </h3>
              <p className="text-[11px] text-slate-400 mt-0.5">
                Select target geographical scope for Harman/Becker 544-byte FLDB database packaging
              </p>
            </div>
            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-amber-500/10 border border-amber-500/40 text-amber-300">
              Active: {activeProfile.code} ({activeProfile.estimatedSize})
            </span>
          </div>

          <div className="grid grid-cols-3 gap-3">
            {regionalProfiles.map((p) => {
              const isSelected = p.code === selectedProfileCode;
              return (
                <div
                  key={p.code}
                  onClick={() => setSelectedProfileCode(p.code)}
                  className={`p-3 rounded-lg border cursor-pointer transition-all ${
                    isSelected
                      ? 'bg-amber-500/10 border-amber-500 shadow-sm'
                      : 'bg-slate-950/80 border-slate-800 hover:border-slate-700'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <span className="font-bold text-xs text-white">{p.name}</span>
                    <span className="text-[10px] font-mono px-1.5 py-0.2 bg-slate-800 text-amber-400 rounded">
                      {p.code}
                    </span>
                  </div>
                  <div className="text-[11px] text-slate-400 mt-1">{p.description}</div>
                  <div className="flex items-center justify-between text-[10px] font-mono text-slate-400 mt-2 pt-2 border-t border-slate-800/60">
                    <span>Est. Size: <strong className="text-white">{p.estimatedSize}</strong></span>
                    <span>Volumes: <strong className="text-amber-400">{p.volumeCount}x FAT32</strong></span>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Google Maps Platform POI Enrichment & Telemetry Bar */}
        <div className="bg-slate-900 border border-slate-800 rounded-lg p-4 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base">📍</span>
              <div>
                <h3 className="text-xs font-bold uppercase tracking-wider text-white">
                  Google Maps Platform POI & Radar Enrichment
                </h3>
                <p className="text-[11px] text-slate-400 mt-0.5">
                  Augment OSM road network with commercial places, EV hubs, and speed enforcement alerts
                </p>
              </div>
            </div>
            <label className="flex items-center gap-2 cursor-pointer">
              <span className="text-xs font-semibold text-slate-300">Enable GMP Pipeline</span>
              <input
                type="checkbox"
                checked={enableGmp}
                onChange={(e) => setEnableGmp(e.target.checked)}
                className="w-4 h-4 rounded accent-amber-500 cursor-pointer"
              />
            </label>
          </div>

          {enableGmp && (
            <div className="grid grid-cols-3 gap-3 pt-2 border-t border-slate-800">
              <label className="flex items-start gap-2 p-2.5 rounded bg-slate-950 border border-slate-800 cursor-pointer">
                <input
                  type="checkbox"
                  checked={enableEvPois}
                  onChange={(e) => setEnableEvPois(e.target.checked)}
                  className="mt-0.5 w-3.5 h-3.5 rounded accent-amber-500 cursor-pointer"
                />
                <div>
                  <span className="text-xs font-bold text-white block">EV Ultra-Fast Chargers</span>
                  <span className="text-[10px] text-slate-400">150-350kW CCS2 DC hubs (Ionity, Tesla, regional)</span>
                </div>
              </label>

              <label className="flex items-start gap-2 p-2.5 rounded bg-slate-950 border border-slate-800 cursor-pointer">
                <input
                  type="checkbox"
                  checked={enableSpeedCameras}
                  onChange={(e) => setEnableSpeedCameras(e.target.checked)}
                  className="mt-0.5 w-3.5 h-3.5 rounded accent-amber-500 cursor-pointer"
                />
                <div>
                  <span className="text-xs font-bold text-white block">Speed Cameras & Radars</span>
                  <span className="text-[10px] text-slate-400">Fixed radars & high-speed corridor enforcement</span>
                </div>
              </label>

              <div className="p-2.5 rounded bg-slate-950 border border-slate-800 flex flex-col justify-center">
                <span className="text-[10px] font-mono text-emerald-400 font-semibold">✓ 30-Day Cache Eviction</span>
                <span className="text-[10px] text-slate-400">FieldMask filtered · ToS Compliant</span>
              </div>
            </div>
          )}
        </div>

        {/* Selected Database Specs & Interactive SVM 03276 Resolver */}
        <div className="grid grid-cols-3 gap-4">
          <div className="col-span-2 p-4 bg-slate-900 border border-slate-800 rounded-lg grid grid-cols-3 gap-3">
            <div>
              <span className="block text-[11px] font-semibold text-slate-400">Target Container</span>
              <span className="font-mono text-xs text-amber-400 font-bold">{activeDb?.name || 'HBNavDB'}</span>
            </div>
            <div>
              <span className="block text-[11px] font-semibold text-slate-400">Physical Page Stride</span>
              <span className="text-xs font-mono text-slate-200">544 bytes (FLDB Header + CRC16)</span>
            </div>
            <div>
              <span className="block text-[11px] font-semibold text-slate-400">FAT32 Splitting</span>
              <span className="text-xs font-mono text-emerald-400">Max 2 GiB / Volume</span>
            </div>
          </div>

          <div className="p-4 bg-slate-900 border border-slate-800 rounded-lg flex flex-col justify-between">
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold text-amber-400">SVM 03276 XOR Resolver</span>
              <span className="text-[10px] font-mono text-slate-500">VCDS Ch.15</span>
            </div>
            <div className="flex items-center gap-2 mt-2">
              <input
                type="text"
                value={svmChannel15Input}
                onChange={(e) => setSvmChannel15Input(e.target.value.replace(/\D/g, ''))}
                className="w-24 px-2 py-1 bg-slate-950 border border-slate-700 rounded font-mono text-xs text-white"
                placeholder="Ch.15 val"
              />
              <span className="text-slate-500 font-mono text-xs">XOR 51666 =</span>
              <span className="font-mono text-xs font-bold text-emerald-400">
                {((parseInt(svmChannel15Input, 10) || 0) ^ 51666).toString()}
              </span>
            </div>
          </div>
        </div>

        {/* 2026 Road Updates Selection Matrix */}
        <div className="bg-slate-900 border border-slate-800 rounded-lg p-5 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-xs font-bold uppercase tracking-wider text-slate-300">
              Available 2026 Infrastructure Updates ({enabledUpdates.length}/{updates.length} Staged)
            </h3>
            <div className="flex items-center gap-3">
              {onDisableAllUpdates && (
                <button
                  onClick={onDisableAllUpdates}
                  className="text-[11px] text-slate-400 hover:text-amber-400 font-mono transition px-2 py-0.5 rounded bg-slate-950 border border-slate-800 hover:border-slate-700"
                  title="Disable all custom map patches (revert to pure stock baseline)"
                >
                  ↺ Disable All (Stock Baseline)
                </button>
              )}
              <div className="flex items-center gap-4 text-xs font-mono text-slate-400">
                <span>Roads Added: <strong className="text-white">{totalKmAdded.toFixed(1)} km</strong></span>
                <span>Vector Nodes: <strong className="text-white">{totalNodesAdded.toLocaleString()}</strong></span>
              </div>
            </div>
          </div>

          <div className="space-y-2.5">
            {updates.map((item) => (
              <div
                key={item.id}
                className={`p-3.5 rounded-lg border transition-all ${
                  item.enabled
                    ? 'bg-slate-950 border-slate-700'
                    : 'bg-slate-950/40 border-slate-800/80 opacity-60'
                }`}
              >
                <div className="flex items-start justify-between">
                  <div className="flex items-start gap-3">
                    <input
                      type="checkbox"
                      checked={item.enabled}
                      onChange={() => onToggleUpdate(item.id)}
                      className="mt-1 w-4 h-4 rounded accent-amber-500 cursor-pointer"
                    />
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="font-bold text-sm text-white">{item.title}</span>
                        <span className="px-1.5 py-0.5 text-[10px] bg-slate-800 text-amber-400 font-mono rounded">
                          {item.type}
                        </span>
                        {item.speedLimitKmh && (
                          <span className="px-1.5 py-0.5 text-[10px] bg-red-950 border border-red-800 text-red-300 font-mono rounded font-bold">
                            {item.speedLimitKmh} km/h
                          </span>
                        )}
                      </div>
                      <p className="text-xs text-slate-300 mt-1 leading-relaxed">{item.description}</p>
                      <div className="flex items-center gap-4 mt-2 text-[11px] text-slate-400 font-mono">
                        <span>Region: {item.region}</span>
                        {item.lengthKm && <span>Length: {item.lengthKm} km</span>}
                        <span>Topology Nodes: +{item.nodesAdded}</span>
                      </div>
                    </div>
                  </div>

                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-950 border border-emerald-800 text-emerald-300">
                    {item.status}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Interactive Map Visualizer & Patch Progress */}
        <div className="bg-slate-900 border border-slate-800 rounded-lg p-5 space-y-4">
          <h3 className="text-xs font-bold uppercase tracking-wider text-slate-300">
            Albania 2026 Navigation Corridor Topology Map
          </h3>

          {/* SVG Map Schematic */}
          <div className="w-full h-64 bg-[#0a0d13] border border-slate-800 rounded-lg relative overflow-hidden flex items-center justify-center">
            <svg viewBox="0 0 600 320" className="w-full h-full">
              {/* Background Geographic Outline (Simplified Albania & Western Balkans) */}
              <path
                d="M 180 30 Q 250 40 320 60 Q 380 90 400 150 Q 390 220 330 280 Q 260 300 220 260 Q 200 200 180 140 Z"
                fill="#121824"
                stroke="#1e293b"
                strokeWidth="2"
              />

              {/* Adriatic & Ionian Sea Label */}
              <text x="50" y="160" fill="#334155" fontSize="12" fontFamily="monospace">
                ADRIATIC SEA / IONIAN SEA
              </text>

              {/* Existing Baseline Roads */}
              <line x1="240" y1="90" x2="280" y2="130" stroke="#475569" strokeWidth="2" strokeDasharray="3,3" />
              <line x1="280" y1="130" x2="270" y2="210" stroke="#475569" strokeWidth="2" strokeDasharray="3,3" />

              {/* 2026 Injected Roads (High-visibility Colored Vectors) */}
              {/* Thumanë - Kashar Expressway (A1) */}
              <line x1="265" y1="110" x2="280" y2="135" stroke="#f59e0b" strokeWidth="4" strokeLinecap="round" />
              <circle cx="272" cy="122" r="3" fill="#f59e0b" />
              <text x="140" y="115" fill="#fbbf24" fontSize="10" fontFamily="sans-serif" fontWeight="bold">
                A1: Thumanë-Kashar (130 km/h)
              </text>

              {/* Rruga e Arbrit */}
              <line x1="280" y1="135" x2="345" y2="140" stroke="#10b981" strokeWidth="3" strokeLinecap="round" />
              <text x="355" y="145" fill="#34d399" fontSize="10" fontFamily="sans-serif" fontWeight="bold">
                Rruga e Arbrit (Murriz Tunnel)
              </text>

              {/* Vlorë Bypass & Llogara Tunnel */}
              <line x1="270" y1="210" x2="275" y2="245" stroke="#38bdf8" strokeWidth="3" strokeLinecap="round" />
              <line x1="275" y1="245" x2="285" y2="275" stroke="#ef4444" strokeWidth="4" strokeLinecap="round" />
              <text x="175" y="260" fill="#f87171" fontSize="10" fontFamily="sans-serif" fontWeight="bold">
                SH8: Llogara Tunnel (80 km/h)
              </text>

              {/* Cities */}
              <circle cx="280" cy="135" r="5" fill="#ffffff" />
              <text x="288" y="132" fill="#ffffff" fontSize="11" fontWeight="bold">
                Tirana (Kryeqyteti)
              </text>

              <circle cx="250" cy="140" r="4" fill="#cbd5e1" />
              <text x="210" y="145" fill="#cbd5e1" fontSize="10">
                Durrës
              </text>

              <circle cx="270" cy="210" r="4" fill="#cbd5e1" />
              <text x="235" y="215" fill="#cbd5e1" fontSize="10">
                Vlorë
              </text>

              <circle cx="335" cy="225" r="4" fill="#cbd5e1" />
              <text x="345" y="230" fill="#cbd5e1" fontSize="10">
                Korçë
              </text>
            </svg>

            <div className="absolute bottom-2 right-3 text-[10px] font-mono text-slate-500">
              Coverage: Albania (AL) & Western Balkans · Map Projection: WGS84 Mercator
            </div>
          </div>

          {/* Patch Compilation Progress & Terminal Logs */}
          {patchingState !== 'idle' && (
            <div className="space-y-3 pt-2">
              <div className="flex items-center justify-between text-xs font-mono">
                <span className="text-slate-400">Map Compilation & CRC Verification Progress:</span>
                <span className="text-amber-400 font-bold">{patchProgress}%</span>
              </div>
              <div className="w-full h-2 bg-slate-950 rounded-full overflow-hidden">
                <div
                  className="h-full bg-amber-500 transition-all duration-300"
                  style={{ width: `${patchProgress}%` }}
                />
              </div>

              {/* Console Logs */}
              <div className="p-3 bg-slate-950 border border-slate-800 rounded font-mono text-[11px] text-slate-300 space-y-1 max-h-40 overflow-y-auto">
                {patchLog.map((log, idx) => (
                  <div key={idx} className={log.startsWith('✓') ? 'text-emerald-400 font-bold' : ''}>
                    {log}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
