import React, { useState } from 'react';
import { AssetBoard } from './components/AssetBoard';
import { ScreenCanvas } from './components/ScreenCanvas';
import { HexViewer } from './components/HexViewer';
import { TypographyStudio } from './components/TypographyStudio';
import { RecipeStudio } from './components/RecipeStudio';
import { InspectResult, ScreenRenderResult } from './types';

type Tab = 'assets' | 'screen' | 'recipes' | 'relab' | 'typography';

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<Tab>('assets');
  const [screenMode, setScreenMode] = useState<'Day' | 'Night'>('Day');

  // Sample seed assets for workstation explorer
  const sampleAssets: InspectResult[] = [
    {
      filePath: 'HBAS/Precomp/System/boot.precomp',
      sizeBytes: 1048576,
      blake3Hash: 'a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0',
      detectedFormat: 'HarmanPrecomp',
      thumbnailBlobId: 'thumb_boot_001',
    },
    {
      filePath: 'HBAS/Precomp/System/gui_nav.precomp',
      sizeBytes: 4194304,
      blake3Hash: 'f0e1d2c3b4a5968778695a4b3c2d1e0ffeeddccbbaa99887766554433221100f',
      detectedFormat: 'HarmanPrecomp',
      thumbnailBlobId: 'thumb_nav_002',
    },
    {
      filePath: 'HBAS/Precomp/Media/audio_eq.precomp',
      sizeBytes: 524288,
      blake3Hash: '1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
      detectedFormat: 'HarmanPrecomp',
    },
    {
      filePath: 'HBNavDB/Database/nav_data.db',
      sizeBytes: 16777216,
      blake3Hash: '9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba',
      detectedFormat: 'HBNavDB',
    },
    {
      filePath: 'Fonts/AudiType-Extended.linotype',
      sizeBytes: 262144,
      blake3Hash: 'abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789',
      detectedFormat: 'LinotypeFont',
    },
  ];

  const [currentScreen, setCurrentScreen] = useState<ScreenRenderResult>({
    width: 800,
    height: 480,
    activeMode: 'Day',
    pixelCount: 384000,
  });

  const handleToggleMode = (mode: 'Day' | 'Night') => {
    setScreenMode(mode);
    setCurrentScreen((prev) => ({ ...prev, activeMode: mode }));
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-slate-950 text-slate-100 select-none overflow-hidden font-sans">
      {/* Top Application Bar */}
      <header className="flex items-center justify-between px-4 py-2.5 bg-slate-900 border-b border-slate-800">
        <div className="flex items-center gap-3">
          <div className="w-7 h-7 rounded bg-amber-500 flex items-center justify-center font-black text-slate-950 text-xs">
            MMI
          </div>
          <div>
            <h1 className="text-sm font-bold tracking-tight">Audi MMI Studio Workstation</h1>
            <span className="text-[10px] text-slate-400 font-mono">v0.1.0 · MMI 3G High / Plus [HN+]</span>
          </div>
        </div>

        {/* Center Tabs */}
        <nav className="flex items-center gap-1 bg-slate-950 p-1 rounded-lg border border-slate-800">
          <button
            onClick={() => setActiveTab('assets')}
            className={`px-3 py-1 text-xs rounded-md font-medium transition-all ${
              activeTab === 'assets'
                ? 'bg-amber-500 text-slate-950 font-semibold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Asset Board
          </button>
          <button
            onClick={() => setActiveTab('screen')}
            className={`px-3 py-1 text-xs rounded-md font-medium transition-all ${
              activeTab === 'screen'
                ? 'bg-amber-500 text-slate-950 font-semibold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Screen Canvas (800x480)
          </button>
          <button
            onClick={() => setActiveTab('recipes')}
            className={`px-3 py-1 text-xs rounded-md font-medium transition-all ${
              activeTab === 'recipes'
                ? 'bg-amber-500 text-slate-950 font-semibold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Theme Recipes
          </button>
          <button
            onClick={() => setActiveTab('relab')}
            className={`px-3 py-1 text-xs rounded-md font-medium transition-all ${
              activeTab === 'relab'
                ? 'bg-amber-500 text-slate-950 font-semibold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Binary RE Lab
          </button>
          <button
            onClick={() => setActiveTab('typography')}
            className={`px-3 py-1 text-xs rounded-md font-medium transition-all ${
              activeTab === 'typography'
                ? 'bg-amber-500 text-slate-950 font-semibold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Typography Studio
          </button>
        </nav>

        {/* Right Status Badges */}
        <div className="flex items-center gap-2">
          <span className="flex items-center gap-1 px-2 py-0.5 text-[10px] rounded border border-emerald-800 bg-emerald-950/80 text-emerald-400 font-mono">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
            AIRGAP ACTIVE
          </span>
          <span className="px-2 py-0.5 text-[10px] rounded border border-slate-700 bg-slate-800 text-slate-300 font-mono">
            IMMUTABLE ORIGINALS
          </span>
        </div>
      </header>

      {/* Main View Area */}
      <main className="flex-1 overflow-hidden relative">
        {activeTab === 'assets' && (
          <AssetBoard
            assets={sampleAssets}
            onSelectAsset={(asset) => {
              console.log('Selected asset:', asset.filePath);
            }}
          />
        )}
        {activeTab === 'screen' && (
          <ScreenCanvas
            currentScreen={currentScreen}
            onToggleMode={handleToggleMode}
          />
        )}
        {activeTab === 'recipes' && <RecipeStudio />}
        {activeTab === 'relab' && <HexViewer />}
        {activeTab === 'typography' && <TypographyStudio />}
      </main>

      {/* Bottom Global Status Bar */}
      <footer className="flex items-center justify-between px-4 py-1.5 bg-slate-900 border-t border-slate-800 text-[11px] text-slate-400">
        <div className="flex items-center gap-3">
          <span className="text-amber-500 font-semibold">§14.9 Safety Policy:</span>
          <span>"SAFE TO INSTALL" claims strictly banned. All builds require hardware-level recovery verification.</span>
        </div>
        <div className="flex items-center gap-4 font-mono text-[10px]">
          <span>IPC: Native Tauri 2 (Rust)</span>
          <span>React 19 / TypeScript</span>
          <span className="text-emerald-400">BLAKE3: Verified</span>
        </div>
      </footer>
    </div>
  );
};
export default App;
