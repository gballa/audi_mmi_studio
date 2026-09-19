import React, { useState } from 'react';

export interface ThemePreset {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  baseTrain: string;
  targetTrains: string[];
  accentColor: string;
  nightColor: string;
  riskClass: 'Cosmetic' | 'Content' | 'Structural';
  operationsCount: number;
}

export const RecipeStudio: React.FC = () => {
  const presets: ThemePreset[] = [
    {
      id: 'audi_sport_amber',
      name: 'Audi Sport Amber Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'Golden amber turn indicators and warm high-contrast night palette for MMI 3G+ cluster display.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922', 'HN+_EU_AU3G_K0900'],
      accentColor: '#FFB300',
      nightColor: '#FF8F00',
      riskClass: 'Content',
      operationsCount: 3,
    },
    {
      id: 'rs_performance_red',
      name: 'RS Performance Red Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'RS performance high-visibility crimson accent scheme with sport cluster graphics.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922'],
      accentColor: '#E0001B',
      nightColor: '#B30000',
      riskClass: 'Content',
      operationsCount: 3,
    },
    {
      id: 'dark_line_minimalist',
      name: 'Dark Line Minimalist Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'Muted monochromatic high-contrast dark palette optimized for reduced night glare and OLED retrofits.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922', 'HN+_EU_AU3G_K0900'],
      accentColor: '#B0B0B0',
      nightColor: '#2A2A2A',
      riskClass: 'Content',
      operationsCount: 3,
    },
    {
      id: 'turbo_blue_dynamic',
      name: 'Turbo Blue S-Line Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'Vibrant neon blue accent with high-definition cyan dials and telemetry highlights.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922', 'HN+_EU_AU3G_K0900'],
      accentColor: '#0077FF',
      nightColor: '#003399',
      riskClass: 'Content',
      operationsCount: 3,
    },
  ];

  const [selectedId, setSelectedId] = useState<string>('audi_sport_amber');
  const [deployStatus, setDeployStatus] = useState<string | null>(null);

  const activeTheme = presets.find((p) => p.id === selectedId) || presets[0];

  const handleSimulateDeploy = () => {
    setDeployStatus(`Packaging update media for '${activeTheme.name}'...`);
    setTimeout(() => {
      setDeployStatus(
        `Media Volume created: FAT32 update image ready for SD card deployment. Status: BUILD READY — DEPLOYMENT NOT VERIFIED.`
      );
      setTimeout(() => setDeployStatus(null), 5000);
    }, 1200);
  };

  return (
    <div className="flex h-full w-full bg-slate-950 text-slate-100 overflow-hidden font-sans min-w-0">
      {/* Left Preset List Sidebar */}
      <div className="w-80 xl:w-96 flex flex-col shrink-0 h-full border-r border-slate-800 bg-slate-900/60 min-w-0">
        <div className="p-4 border-b border-slate-800 shrink-0">
          <h2 className="text-base font-bold tracking-tight text-white flex items-center gap-2">
            <span>📜</span>
            <span>Theme Recipe Library</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">
            Declarative JSON recipes & cross-firmware train rebasing
          </p>
        </div>

        <div className="flex-1 overflow-y-auto p-3 space-y-2.5 min-w-0">
          {presets.map((p) => {
            const isSelected = p.id === selectedId;
            return (
              <div
                key={p.id}
                onClick={() => setSelectedId(p.id)}
                className={`p-3.5 rounded-xl border cursor-pointer transition-all ${
                  isSelected
                    ? 'bg-amber-500/10 border-amber-500 shadow-md ring-1 ring-amber-500/30'
                    : 'bg-slate-900/70 border-slate-800 hover:border-slate-700'
                }`}
              >
                <div className="flex items-center justify-between mb-1.5">
                  <span className="font-bold text-xs text-white truncate max-w-[190px]">{p.name}</span>
                  <span
                    className="w-4 h-4 rounded-full border border-white/40 shadow-sm shrink-0"
                    style={{
                      backgroundColor: p.accentColor,
                      boxShadow: `0 0 10px ${p.accentColor}80`,
                    }}
                  />
                </div>
                <p className="text-[11px] text-slate-400 line-clamp-2 leading-relaxed mb-2">
                  {p.description}
                </p>
                <div className="flex items-center gap-1.5 text-[10px] font-mono">
                  <span className="px-2 py-0.5 rounded bg-slate-950 border border-slate-800 text-sky-400 font-semibold truncate max-w-[130px]">
                    {p.baseTrain.replace('HN+R_EU_AU_', '')}
                  </span>
                  <span className="px-2 py-0.5 rounded bg-amber-950/60 border border-amber-800/80 text-amber-300 font-semibold">
                    {p.riskClass}
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Right Recipe Details & Customizer Panel */}
      <div className="flex-1 min-w-0 flex flex-col h-full overflow-y-auto bg-slate-950 p-6 space-y-6">
        {/* Header Strip */}
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 pb-4 border-b border-slate-800 shrink-0">
          <div>
            <div className="flex items-center gap-3">
              <h2 className="text-xl font-bold text-white tracking-tight">{activeTheme.name}</h2>
              <span className="px-2 py-0.5 text-xs bg-slate-800 border border-slate-700 text-slate-300 font-mono rounded">
                v{activeTheme.version}
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-1">
              Author: <strong className="text-slate-200">{activeTheme.author}</strong> · Declarative JSON Recipe Spec
            </p>
          </div>

          <div className="flex items-center gap-2 shrink-0">
            <button
              onClick={handleSimulateDeploy}
              className="px-4 py-2 bg-gradient-to-r from-amber-500 to-amber-400 hover:from-amber-400 hover:to-amber-300 text-slate-950 text-xs font-bold rounded-lg shadow-lg shadow-amber-500/20 transition flex items-center gap-2"
            >
              <span>🚀</span> Package SD-Card Update
            </button>
          </div>
        </div>

        {/* Deploy Notification */}
        {deployStatus && (
          <div className="p-3 bg-slate-900 border border-emerald-500/80 rounded-xl text-xs font-mono text-emerald-300 flex items-center gap-2 shadow-lg animate-in fade-in">
            <span>✓</span>
            <span>{deployStatus}</span>
          </div>
        )}

        {/* Color Palette Display */}
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 p-4 bg-slate-900/80 border border-slate-800 rounded-xl">
          <div className="flex items-center gap-3">
            <div
              className="w-10 h-10 rounded-lg border-2 border-white/20 shadow-md shrink-0"
              style={{ backgroundColor: activeTheme.accentColor }}
            />
            <div>
              <span className="block text-[10px] uppercase tracking-wider font-bold text-slate-400">Day Accent</span>
              <span className="font-mono text-xs font-bold text-white">{activeTheme.accentColor}</span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div
              className="w-10 h-10 rounded-lg border-2 border-white/20 shadow-md shrink-0"
              style={{ backgroundColor: activeTheme.nightColor }}
            />
            <div>
              <span className="block text-[10px] uppercase tracking-wider font-bold text-slate-400">Night Accent</span>
              <span className="font-mono text-xs font-bold text-white">{activeTheme.nightColor}</span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-slate-950 border border-slate-800 flex items-center justify-center font-mono text-xs text-amber-400 font-bold shrink-0">
              {activeTheme.operationsCount}
            </div>
            <div>
              <span className="block text-[10px] uppercase tracking-wider font-bold text-slate-400">Operations</span>
              <span className="text-xs text-slate-200">Asset Injections</span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-slate-950 border border-slate-800 flex items-center justify-center font-mono text-xs text-emerald-400 font-bold shrink-0">
              §14.9
            </div>
            <div>
              <span className="block text-[10px] uppercase tracking-wider font-bold text-slate-400">Safety Class</span>
              <span className="text-xs text-slate-200">{activeTheme.riskClass}</span>
            </div>
          </div>
        </div>

        {/* Target Trains & Compatibility */}
        <div className="p-4 bg-slate-900/80 border border-slate-800 rounded-xl space-y-2">
          <span className="text-xs font-bold uppercase tracking-wider text-slate-400 block">
            Compatible Target Trains (Rebase Engine Supported)
          </span>
          <div className="flex flex-wrap gap-2">
            {activeTheme.targetTrains.map((train) => (
              <span
                key={train}
                className="px-2.5 py-1 rounded bg-slate-950 border border-slate-700 text-xs font-mono text-emerald-400 font-semibold"
              >
                {train}
              </span>
            ))}
          </div>
        </div>

        {/* Simulated Virtual Cluster Display */}
        <div className="flex-1 bg-[#080b10] border border-slate-800 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner min-h-[220px]">
          <span className="text-xs font-mono text-slate-500 mb-4">
            Virtual Cluster Turn Display (800x480 Preview)
          </span>
          <svg width="100" height="100" viewBox="0 0 100 100" className="drop-shadow-lg">
            <path
              d="M 50 15 L 85 50 L 65 50 L 65 85 L 35 85 L 35 50 L 15 50 Z"
              fill={activeTheme.accentColor}
              stroke="#ffffff"
              strokeWidth="2"
            />
          </svg>
          <div
            className="mt-4 text-sm font-bold tracking-tight"
            style={{ color: activeTheme.accentColor }}
          >
            150 m — Turn Right onto A9 Autobahn
          </div>
        </div>
      </div>
    </div>
  );
};
