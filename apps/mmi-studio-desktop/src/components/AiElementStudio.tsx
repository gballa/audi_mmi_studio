import React, { useState, useEffect } from 'react';
import { AiAssetItem, MMIThemeConfig } from '../types';

interface AiElementStudioProps {
  assets: AiAssetItem[];
  onUpdateAsset: (updatedAsset: AiAssetItem) => void;
  onResetAsset?: (id: string) => void;
  onResetAllAssets?: () => void;
  themeConfig: MMIThemeConfig;
  onUpdateTheme: (newConfig: Partial<MMIThemeConfig>) => void;
  onNavigateToScreen: () => void;
  initialSelectedAssetId?: string | null;
}

export const AiElementStudio: React.FC<AiElementStudioProps> = ({
  assets,
  onUpdateAsset,
  onResetAsset,
  onResetAllAssets,
  themeConfig,
  onUpdateTheme,
  onNavigateToScreen,
  initialSelectedAssetId,
}) => {
  const [selectedAssetId, setSelectedAssetId] = useState<string>(
    initialSelectedAssetId || assets[0]?.id || 'corner_bracket_bl'
  );
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [selectedModel, setSelectedModel] = useState<string>('gemini-3.1-flash-image');
  const [promptText, setPromptText] = useState<string>(assets[0]?.promptSuggestion || '');
  const [isAirlockOpen, setIsAirlockOpen] = useState<boolean>(false);
  const [isGenerating, setIsGenerating] = useState<boolean>(false);
  const [viewMode, setViewMode] = useState<'split' | 'side-by-side' | 'alpha'>('side-by-side');
  const [airlockTransmissionsCount, setAirlockTransmissionsCount] = useState<number>(3);
  const [airlockBytesCount, setAirlockBytesCount] = useState<number>(49152);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const categories = [
    'All',
    'Corner Softkeys',
    'Drive Select',
    'Submenus',
    'Status Bar',
    'Navigation',
    'Gauges',
    'Climate',
    'Menu Icons',
    'Textures',
  ];

  useEffect(() => {
    if (initialSelectedAssetId) {
      const match = assets.find((a) => a.id === initialSelectedAssetId);
      if (match) {
        setSelectedAssetId(match.id);
        setPromptText(match.aiPromptApplied || match.promptSuggestion);
        setSelectedCategory('All');
      }
    }
  }, [initialSelectedAssetId, assets]);

  const filteredAssets = assets.filter(
    (a) => selectedCategory === 'All' || a.category === selectedCategory
  );

  const activeAsset = assets.find((a) => a.id === selectedAssetId) || assets[0];

  const handleSelectAsset = (asset: AiAssetItem) => {
    setSelectedAssetId(asset.id);
    setPromptText(asset.aiPromptApplied || asset.promptSuggestion);
  };

  const handleApplyPreset = (presetText: string) => {
    setPromptText(presetText);
  };

  const handleConfirmAirlockAndGenerate = () => {
    setIsAirlockOpen(false);
    setIsGenerating(true);

    // Simulate Egress Airlock transmission & Gemini Nano Banana inference
    setTimeout(() => {
      setIsGenerating(false);
      setAirlockTransmissionsCount((prev) => prev + 1);
      setAirlockBytesCount((prev) => prev + activeAsset.width * activeAsset.height * 4);

      // Derive modified visual styling based on prompt keywords
      let newStyle = activeAsset.currentStyle;
      const lower = promptText.toLowerCase();
      if (lower.includes('crimson') || lower.includes('red') || lower.includes('rs') || lower.includes('misano')) {
        newStyle = '#E0001B';
      } else if (lower.includes('cyan') || lower.includes('blue') || lower.includes('ice') || lower.includes('turbo')) {
        newStyle = '#00D4FF';
      } else if (lower.includes('amber') || lower.includes('gold') || lower.includes('yellow')) {
        newStyle = '#FFB300';
      } else if (lower.includes('carbon')) {
        newStyle = 'carbon_fiber_matrix';
      } else if (lower.includes('emerald') || lower.includes('green') || lower.includes('lime')) {
        newStyle = '#10B981';
      } else if (lower.includes('white') || lower.includes('silver') || lower.includes('chrome')) {
        newStyle = '#E2E8F0';
      } else {
        newStyle = '#E0001B'; // Default RS red
      }

      const updated: AiAssetItem = {
        ...activeAsset,
        currentStyle: newStyle,
        aiPromptApplied: promptText,
        modelUsed: selectedModel,
        status: 'AI Modified',
      };

      onUpdateAsset(updated);
      setToastMessage(`Generated '${activeAsset.name}' with ${selectedModel}!`);
      setTimeout(() => setToastMessage(null), 3500);
    }, 1200);
  };

  const handleApplyToActiveScreen = () => {
    if (activeAsset.category === 'Corner Softkeys') {
      onUpdateTheme({ cornerBracketColor: activeAsset.currentStyle });
    } else if (activeAsset.id === 'gauge_needle') {
      onUpdateTheme({ needleColor: activeAsset.currentStyle });
    } else if (activeAsset.id === 'nav_turn_arrow') {
      onUpdateTheme({ accentColor: activeAsset.currentStyle });
    } else if (activeAsset.id === 'texture_carbon') {
      onUpdateTheme({ activeBackgroundTexture: 'carbon_weave' });
    } else if (activeAsset.id === 'texture_aluminum') {
      onUpdateTheme({ activeBackgroundTexture: 'brushed_aluminum' });
    } else if (activeAsset.id === 'car_silhouette_sport') {
      onUpdateTheme({ activeCarSilhouetteStyle: activeAsset.currentStyle });
    } else if (activeAsset.id === 'settings_frame_border') {
      onUpdateTheme({ cornerBracketColor: activeAsset.currentStyle, accentColor: activeAsset.currentStyle });
    } else if (activeAsset.category === 'Status Bar') {
      if (activeAsset.id === 'status_bar_traffic') {
        onUpdateTheme({
          statusBar: { ...themeConfig.statusBar, dataNetwork: activeAsset.currentStyle.includes('LTE') ? 'LTE' : '3G' },
        });
      }
    }

    setToastMessage(`Applied '${activeAsset.name}' directly to Virtual Screen!`);
    setTimeout(() => setToastMessage(null), 3000);
  };

  // Helper to render asset visually in preview box
  const renderAssetPreview = (asset: AiAssetItem, style: string) => {
    switch (asset.category) {
      case 'Corner Softkeys': {
        const isBracket = asset.id.includes('bracket');
        if (isBracket) {
          return (
            <div className="flex flex-col items-center gap-2">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none">
                <path
                  d="M 2 4 L 2 18 Q 2 22 6 22 L 20 22"
                  stroke={style.startsWith('#') ? style : '#E0001B'}
                  strokeWidth="3.5"
                  strokeLinecap="round"
                  style={{ filter: `drop-shadow(0 0 6px ${style.startsWith('#') ? style : '#E0001B'})` }}
                />
              </svg>
              <span className="text-xs font-mono font-bold text-slate-300">Corner Bracket</span>
            </div>
          );
        }
        return (
          <div className="px-4 py-2 bg-slate-900 border border-slate-700 rounded text-sm font-bold text-white shadow">
            Car systems
          </div>
        );
      }
      case 'Drive Select': {
        if (asset.id.includes('platter')) {
          return (
            <div className="w-56 h-24 flex items-center justify-center">
              <svg viewBox="0 0 200 80" className="w-full h-full">
                <ellipse cx="100" cy="40" rx="90" ry="26" fill="#0f172a" stroke={style.startsWith('#') ? style : '#E0001B'} strokeWidth="3" />
                <ellipse cx="100" cy="40" rx="65" ry="18" fill="#020617" stroke="#334155" strokeWidth="1.5" />
              </svg>
            </div>
          );
        }
        if (asset.id.includes('silhouette')) {
          return (
            <div className="text-5xl" style={{ filter: style.startsWith('#') ? `drop-shadow(0 0 10px ${style})` : 'none' }}>
              🏎️
            </div>
          );
        }
        return (
          <div className="flex flex-col items-center">
            <span className="text-xs font-bold text-red-500 mb-0.5">▼</span>
            <div
              className="px-4 py-1.5 rounded-lg border-2 text-xs font-bold text-white"
              style={{
                borderColor: style.startsWith('#') ? style : '#E0001B',
                backgroundColor: 'rgba(224, 0, 27, 0.25)',
              }}
            >
              Comfort
            </div>
          </div>
        );
      }
      case 'Submenus': {
        if (asset.id.includes('frame')) {
          return (
            <div
              className="w-48 h-28 rounded-xl border-2 p-2 flex flex-col justify-between"
              style={{ borderColor: style.startsWith('#') ? style : '#E0001B', backgroundColor: 'rgba(10,14,22,0.8)' }}
            >
              <div className="text-[10px] font-bold text-slate-300 border-b border-slate-800 pb-1">Setup Menu</div>
              <div className="text-[10px] text-slate-400">Left Crescent Arc</div>
            </div>
          );
        }
        return (
          <div
            className="px-3 py-1 rounded border text-xs font-bold text-white flex items-center gap-1.5"
            style={{
              borderColor: style.startsWith('#') ? style : '#E0001B',
              backgroundColor: 'rgba(224, 0, 27, 0.35)',
            }}
          >
            <span>▼</span>
            <span>Dynamic</span>
          </div>
        );
      }
      case 'Status Bar': {
        if (asset.id.includes('mute')) {
          return (
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
              <line x1="23" y1="9" x2="17" y2="15" />
              <line x1="17" y1="9" x2="23" y2="15" />
            </svg>
          );
        }
        if (asset.id.includes('clock')) {
          return <span className="text-2xl font-mono font-bold text-white tracking-wider">16:06</span>;
        }
        if (asset.id.includes('bluetooth')) {
          return (
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#60a5fa" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5" />
            </svg>
          );
        }
        if (asset.id.includes('signal')) {
          return (
            <div className="flex items-end gap-1 h-6">
              {[1, 2, 3, 4].map((b) => (
                <div key={b} className="w-1.5 bg-emerald-400 rounded-sm" style={{ height: `${b * 5 + 3}px` }} />
              ))}
            </div>
          );
        }
        if (asset.id.includes('google')) {
          return <span className="text-lg font-sans font-bold text-slate-100">Google</span>;
        }
        return (
          <div className="flex items-center gap-1 text-sm font-bold text-sky-400">
            <span>3G</span>
            <span>⇄</span>
          </div>
        );
      }
      case 'Gauges': {
        return (
          <div className="relative w-24 h-24 flex items-center justify-center">
            <div className="w-20 h-20 rounded-full border-2 border-dashed border-slate-700 flex items-center justify-center">
              <div
                className="w-1.5 h-16 rounded-full transition-all shadow-[0_0_8px_currentColor]"
                style={{ backgroundColor: style.startsWith('#') ? style : '#FF0000', color: style, transform: 'rotate(-45deg)' }}
              />
            </div>
          </div>
        );
      }
      case 'Navigation': {
        return (
          <div
            className="w-14 h-14 rounded-xl flex items-center justify-center text-2xl font-bold shadow-xl transition-all"
            style={{ backgroundColor: style.startsWith('#') ? style : '#FF9900', color: '#000' }}
          >
            ⮡
          </div>
        );
      }
      case 'Climate': {
        return (
          <div
            className="w-20 h-20 rounded-full border-4 flex items-center justify-center text-base font-mono text-white font-bold"
            style={{ borderColor: style.startsWith('#') ? style : '#FF9900' }}
          >
            21.5°
          </div>
        );
      }
      case 'Menu Icons': {
        return <div className="text-4xl">{asset.originalStyle}</div>;
      }
      case 'Textures': {
        return (
          <div className="w-44 h-20 rounded border border-slate-700 bg-slate-900 flex flex-col items-center justify-center text-xs font-mono text-amber-300">
            <span>{asset.name}</span>
            <span className="text-[10px] text-slate-500">{style}</span>
          </div>
        );
      }
      default:
        return <div className="text-sm text-slate-400">{asset.name}</div>;
    }
  };

  return (
    <div className="flex h-full bg-slate-950 text-slate-100 overflow-hidden font-sans">
      {/* Left System Element Selector */}
      <div className="w-80 flex flex-col border-r border-slate-800 bg-slate-900/60 overflow-y-auto">
        <div className="p-4 border-b border-slate-800">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-xl">🍌</span>
              <div>
                <h2 className="text-sm font-bold tracking-tight text-white">MMI UI Element Catalog</h2>
                <span className="text-[10px] text-amber-400 font-mono">Gemini Nano Banana Studio</span>
              </div>
            </div>
            {onResetAllAssets && (
              <button
                onClick={onResetAllAssets}
                className="text-[10px] text-slate-400 hover:text-amber-400 flex items-center gap-1 font-mono transition py-1 px-1.5 rounded hover:bg-slate-800 border border-transparent hover:border-slate-700"
                title="Reset all AI elements back to stock OEM"
              >
                <span>↺</span> Reset All
              </button>
            )}
          </div>
          <p className="text-[11px] text-slate-400 mt-1">
            Catalog of all genuine MMI UI elements ready to preview and restyle using Banana AI.
          </p>

          {/* Category Tabs */}
          <div className="flex flex-wrap gap-1 mt-3">
            {categories.map((cat) => (
              <button
                key={cat}
                onClick={() => setSelectedCategory(cat)}
                className={`px-2 py-0.5 text-[10px] rounded transition-all ${
                  selectedCategory === cat
                    ? 'bg-amber-500 text-slate-950 font-bold'
                    : 'bg-slate-800 text-slate-400 hover:text-slate-200'
                }`}
              >
                {cat}
              </button>
            ))}
          </div>
        </div>

        {/* Elements List */}
        <div className="p-3 space-y-2">
          {filteredAssets.map((asset) => {
            const isSelected = asset.id === selectedAssetId;
            return (
              <div
                key={asset.id}
                onClick={() => handleSelectAsset(asset)}
                className={`p-3 rounded-lg border cursor-pointer transition-all ${
                  isSelected
                    ? 'bg-amber-500/10 border-amber-500 shadow-sm'
                    : 'bg-slate-950 border-slate-800/80 hover:border-slate-700'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-xs text-white truncate max-w-[170px]">
                    {asset.name}
                  </span>
                  <span
                    className={`text-[9px] font-mono px-1.5 py-0.2 rounded font-bold ${
                      asset.status === 'AI Modified'
                        ? 'bg-amber-950 text-amber-300 border border-amber-800'
                        : 'bg-slate-800 text-slate-400'
                    }`}
                  >
                    {asset.status}
                  </span>
                </div>
                <div className="text-[11px] text-slate-400 mt-1 truncate">{asset.description}</div>
                <div className="flex items-center justify-between text-[10px] text-slate-500 font-mono mt-2 pt-2 border-t border-slate-800/50">
                  <span className="text-amber-400/90 font-semibold">{asset.category}</span>
                  <span>{asset.width}×{asset.height} px</span>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Center Live Asset Preview & Comparison */}
      <div className="flex-1 flex flex-col overflow-y-auto bg-slate-950 p-6 space-y-6">
        {/* Top Control Bar */}
        <div className="flex items-center justify-between pb-4 border-b border-slate-800">
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-white tracking-tight">{activeAsset.name}</h2>
              <span className="px-2 py-0.5 text-[10px] bg-slate-800 text-slate-300 font-mono rounded">
                Category: {activeAsset.category}
              </span>
              {activeAsset.synthIdWatermark && (
                <span className="px-2 py-0.5 text-[10px] bg-blue-950 border border-blue-800 text-blue-300 font-mono rounded">
                  SynthID Verified
                </span>
              )}
            </div>
            <p className="text-xs text-slate-400 mt-0.5">{activeAsset.description}</p>
          </div>

          <div className="flex items-center gap-2">
            {toastMessage && (
              <span className="text-xs text-emerald-400 font-semibold animate-pulse mr-2">
                {toastMessage}
              </span>
            )}
            <div className="flex bg-slate-900 border border-slate-800 rounded p-0.5 text-xs">
              <button
                onClick={() => setViewMode('side-by-side')}
                className={`px-2.5 py-1 rounded text-[11px] font-medium transition ${
                  viewMode === 'side-by-side' ? 'bg-amber-500 text-slate-950 font-bold' : 'text-slate-400'
                }`}
              >
                Side-by-Side
              </button>
              <button
                onClick={() => setViewMode('split')}
                className={`px-2.5 py-1 rounded text-[11px] font-medium transition ${
                  viewMode === 'split' ? 'bg-amber-500 text-slate-950 font-bold' : 'text-slate-400'
                }`}
              >
                Before / After Split
              </button>
            </div>
            <button
              onClick={handleApplyToActiveScreen}
              className="px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-bold rounded shadow transition flex items-center gap-1.5"
            >
              <span>✨</span> Apply to Live Screen
            </button>
            {onResetAsset && activeAsset.status === 'AI Modified' && (
              <button
                onClick={() => onResetAsset(activeAsset.id)}
                className="px-3 py-1.5 bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white text-xs font-semibold rounded border border-slate-700 transition flex items-center gap-1"
                title="Reset this element to stock OEM"
              >
                <span>↺</span> Reset
              </button>
            )}
            <button
              onClick={onNavigateToScreen}
              className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-semibold rounded border border-slate-700 transition"
            >
              👁️ View on 800x480 Screen
            </button>
          </div>
        </div>

        {/* Visual Inspection Stage */}
        <div className="grid grid-cols-2 gap-6 p-6 bg-[#0a0d13] border border-slate-800 rounded-xl min-h-[280px] relative overflow-hidden">
          {/* Left: Original Stock Asset */}
          <div className="flex flex-col items-center justify-center p-6 bg-black/50 border border-slate-800 rounded-lg relative">
            <span className="absolute top-3 left-3 text-[10px] font-mono text-slate-500 uppercase tracking-wider">
              [Stock Baseline OEM]
            </span>
            <div className="my-auto flex flex-col items-center justify-center">
              {renderAssetPreview(activeAsset, activeAsset.originalStyle)}
            </div>
            <div className="mt-4 text-[11px] font-mono text-slate-500">Original Format: QNX RLE8 / Vector / TrueType</div>
          </div>

          {/* Right: Gemini Nano Banana AI Modified Asset */}
          <div className="flex flex-col items-center justify-center p-6 bg-gradient-to-b from-amber-950/10 to-black/60 border border-amber-500/30 rounded-lg relative shadow-[0_0_30px_rgba(245,158,11,0.05)]">
            <span className="absolute top-3 left-3 text-[10px] font-mono text-amber-400 uppercase tracking-wider font-bold flex items-center gap-1">
              <span>🍌 Gemini Nano Banana Restyled</span>
            </span>

            <div className="my-auto flex flex-col items-center justify-center">
              {isGenerating ? (
                <div className="flex flex-col items-center gap-3">
                  <div className="w-8 h-8 border-3 border-amber-500 border-t-transparent rounded-full animate-spin" />
                  <span className="text-xs font-mono text-amber-400 animate-pulse">
                    Synthesizing with {selectedModel}...
                  </span>
                </div>
              ) : (
                renderAssetPreview(activeAsset, activeAsset.currentStyle)
              )}
            </div>

            <div className="mt-4 text-[11px] font-mono text-amber-400/90">
              {activeAsset.status === 'AI Modified' ? (
                <span>Model: {activeAsset.modelUsed || selectedModel} · SynthID Embedded</span>
              ) : (
                <span className="text-slate-500">Awaiting AI Prompt Generation</span>
              )}
            </div>
          </div>
        </div>

        {/* Bottom Gemini Nano Banana Prompting Cockpit */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-lg">🍌</span>
              <h3 className="text-xs font-bold uppercase tracking-wider text-slate-200">
                Gemini Nano Banana AI Asset Synthesis
              </h3>
            </div>

            <div className="flex items-center gap-3">
              <span className="text-[11px] text-slate-400 font-mono">Model:</span>
              <select
                value={selectedModel}
                onChange={(e) => setSelectedModel(e.target.value)}
                className="px-2.5 py-1 bg-slate-950 border border-slate-700 rounded text-xs text-amber-400 font-mono"
              >
                <option value="gemini-3.1-flash-image">gemini-3.1-flash-image (Nano Banana 2)</option>
                <option value="gemini-3.1-flash-lite-image">gemini-3.1-flash-lite-image (Nano Banana Lite)</option>
                <option value="gemini-2.5-flash-image">gemini-2.5-flash-image (Legacy Nano Banana)</option>
              </select>
            </div>
          </div>

          {/* Quick Automotive Prompt Presets */}
          <div className="flex items-center gap-2 flex-wrap">
            <span className="text-[11px] text-slate-400 font-semibold">Quick Presets:</span>
            {[
              'RS Crimson Sport with sharp glowing neon edge',
              'Audi Sport Amber with aerospace titanium finish',
              'Modern Neon Cyan with high-contrast night vision',
              'Matte 2x2 Carbon Fiber Twill Weave Pattern',
              'Glacier White Minimalist Nordic aesthetic',
            ].map((preset) => (
              <button
                key={preset}
                onClick={() => handleApplyPreset(preset)}
                className="px-2.5 py-1 bg-slate-950 hover:bg-slate-800 border border-slate-800 rounded text-[11px] text-slate-300 hover:text-white transition"
              >
                {preset}
              </button>
            ))}
          </div>

          {/* Prompt Text Input */}
          <div className="space-y-2">
            <textarea
              value={promptText}
              onChange={(e) => setPromptText(e.target.value)}
              placeholder="Describe how to restyle this asset (e.g. materials, glow, colors, bevels)..."
              rows={2}
              className="w-full p-3 bg-slate-950 border border-slate-700 rounded-lg text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-amber-500 transition"
            />
          </div>

          {/* Action Trigger */}
          <div className="flex items-center justify-between pt-1">
            <div className="flex items-center gap-4 text-[11px] font-mono text-slate-400">
              <span>Target Element: {activeAsset.name}</span>
              <span>Airlock Egress: {airlockTransmissionsCount} sent ({Math.round(airlockBytesCount / 1024)} KB)</span>
            </div>


            <button
              onClick={() => setIsAirlockOpen(true)}
              disabled={isGenerating || !promptText.trim()}
              className={`px-5 py-2.5 rounded-lg text-xs font-bold transition flex items-center gap-2 shadow ${
                isGenerating || !promptText.trim()
                  ? 'bg-slate-800 text-slate-500 cursor-not-allowed'
                  : 'bg-gradient-to-r from-amber-500 to-amber-400 hover:from-amber-400 hover:to-amber-300 text-slate-950 shadow-[0_0_20px_rgba(245,158,11,0.3)]'
              }`}
            >
              <span>🍌</span>
              <span>Synthesize with Gemini Nano Banana</span>
            </button>
          </div>
        </div>
      </div>

      {/* Egress Airlock Modal Confirmation (§14.9 Compliance) */}
      {isAirlockOpen && (
        <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
          <div className="bg-slate-900 border-2 border-amber-500/80 rounded-xl max-w-lg w-full p-6 space-y-4 shadow-2xl">
            <div className="flex items-center justify-between border-b border-slate-800 pb-3">
              <div className="flex items-center gap-2">
                <span className="text-xl">🛡️</span>
                <h3 className="text-sm font-bold text-white uppercase tracking-wider">
                  Egress Airlock: Banana AI Transmission
                </h3>
              </div>
              <span className="text-[10px] font-mono text-amber-400 bg-amber-950/60 px-2 py-0.5 rounded border border-amber-800">
                Safe Boundary Check
              </span>
            </div>

            <div className="text-xs text-slate-300 space-y-2">
              <p>
                You are about to transmit the prompt and bounding dimensions for <strong className="text-white">'{activeAsset.name}'</strong> to Gemini Nano Banana for synthesis.
              </p>
              <div className="p-3 bg-slate-950 rounded border border-slate-800 space-y-1 font-mono text-[11px]">
                <div><strong className="text-slate-400">Prompt:</strong> {promptText}</div>
                <div><strong className="text-slate-400">Slot Size:</strong> {activeAsset.width}×{activeAsset.height} px</div>
                <div><strong className="text-slate-400">Target Model:</strong> {selectedModel}</div>
                <div><strong className="text-slate-400">Watermark:</strong> SynthID Invisible Signature</div>
              </div>
            </div>

            <div className="flex items-center justify-end gap-2 pt-2 border-t border-slate-800">
              <button
                onClick={() => setIsAirlockOpen(false)}
                className="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs rounded font-medium transition"
              >
                Cancel
              </button>
              <button
                onClick={handleConfirmAirlockAndGenerate}
                className="px-4 py-2 bg-gradient-to-r from-amber-500 to-amber-400 text-slate-950 text-xs font-bold rounded shadow hover:brightness-110 transition flex items-center gap-1.5"
              >
                <span>🚀</span> Confirm & Synthesize
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
