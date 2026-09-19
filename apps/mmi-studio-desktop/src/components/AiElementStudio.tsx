import React, { useState } from 'react';
import { AiAssetItem, MMIThemeConfig } from '../types';

interface AiElementStudioProps {
  assets: AiAssetItem[];
  onUpdateAsset: (updatedAsset: AiAssetItem) => void;
  onResetAsset?: (id: string) => void;
  onResetAllAssets?: () => void;
  themeConfig: MMIThemeConfig;
  onUpdateTheme: (newConfig: Partial<MMIThemeConfig>) => void;
  onNavigateToScreen: () => void;
}

export const AiElementStudio: React.FC<AiElementStudioProps> = ({
  assets,
  onUpdateAsset,
  onResetAsset,
  onResetAllAssets,
  onUpdateTheme,
  onNavigateToScreen,
}) => {
  const [selectedAssetId, setSelectedAssetId] = useState<string>(assets[0]?.id || 'nav_turn_arrow');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [selectedModel, setSelectedModel] = useState<string>('gemini-3.1-flash-image');
  const [promptText, setPromptText] = useState<string>(assets[0]?.promptSuggestion || '');
  const [isAirlockOpen, setIsAirlockOpen] = useState<boolean>(false);
  const [isGenerating, setIsGenerating] = useState<boolean>(false);
  const [viewMode, setViewMode] = useState<'split' | 'side-by-side' | 'alpha'>('side-by-side');
  const [airlockTransmissionsCount, setAirlockTransmissionsCount] = useState<number>(3);
  const [airlockBytesCount, setAirlockBytesCount] = useState<number>(49152);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const categories = ['All', 'Gauges', 'Navigation', 'Vehicle', 'Climate', 'Menu Icons', 'Textures'];

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
      if (promptText.toLowerCase().includes('crimson') || promptText.toLowerCase().includes('red') || promptText.toLowerCase().includes('rs')) {
        newStyle = '#E0001B';
      } else if (promptText.toLowerCase().includes('cyan') || promptText.toLowerCase().includes('blue') || promptText.toLowerCase().includes('ice')) {
        newStyle = '#00D4FF';
      } else if (promptText.toLowerCase().includes('amber') || promptText.toLowerCase().includes('gold')) {
        newStyle = '#FFB300';
      } else if (promptText.toLowerCase().includes('carbon')) {
        newStyle = 'carbon_fiber_matrix';
      } else {
        newStyle = '#10B981'; // Lime / Emerald sport accent
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
    if (activeAsset.id === 'gauge_needle') {
      onUpdateTheme({ needleColor: activeAsset.currentStyle });
    } else if (activeAsset.id === 'nav_turn_arrow') {
      onUpdateTheme({ accentColor: activeAsset.currentStyle });
    } else if (activeAsset.id === 'texture_carbon') {
      onUpdateTheme({ activeBackgroundTexture: 'carbon_weave' });
    } else if (activeAsset.id === 'car_silhouette_sport') {
      onUpdateTheme({ activeCarSilhouetteStyle: activeAsset.currentStyle });
    }

    setToastMessage(`Applied '${activeAsset.name}' directly to Virtual Screen!`);
    setTimeout(() => setToastMessage(null), 3000);
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
            Select an MMI visual asset to preview and restyle using AI image generation.
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
                  <span>{asset.category}</span>
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
                Slot: {activeAsset.width}×{activeAsset.height} px
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
              <button
                onClick={() => setViewMode('alpha')}
                className={`px-2.5 py-1 rounded text-[11px] font-medium transition ${
                  viewMode === 'alpha' ? 'bg-amber-500 text-slate-950 font-bold' : 'text-slate-400'
                }`}
              >
                Alpha Mask
              </button>
            </div>
            <button
              onClick={handleApplyToActiveScreen}
              className="px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-bold rounded shadow transition"
            >
              Apply to Live Screen
            </button>
            {onResetAsset && activeAsset.status === 'AI Modified' && (
              <button
                onClick={() => onResetAsset(activeAsset.id)}
                className="px-3 py-1.5 bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white text-xs font-semibold rounded border border-slate-700 transition flex items-center gap-1"
                title="Reset this element to stock OEM"
              >
                <span>↺</span> Reset to Stock
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
        <div className="grid grid-cols-2 gap-6 p-6 bg-[#0a0d13] border border-slate-800 rounded-xl min-h-[300px] relative overflow-hidden">
          {/* Left: Original Stock Asset */}
          <div className="flex flex-col items-center justify-center p-6 bg-black/50 border border-slate-800 rounded-lg relative">
            <span className="absolute top-3 left-3 text-[10px] font-mono text-slate-500 uppercase tracking-wider">
              [Stock Baseline Asset]
            </span>

            {/* Simulated Visual Render */}
            <div className="my-auto flex flex-col items-center justify-center">
              {activeAsset.category === 'Gauges' && (
                <div className="relative w-24 h-24 flex items-center justify-center">
                  <div className="w-20 h-20 rounded-full border-2 border-dashed border-slate-700 flex items-center justify-center">
                    <div
                      className="w-1 h-16 rounded-full transition-all"
                      style={{ backgroundColor: activeAsset.originalStyle, transform: 'rotate(-45deg)' }}
                    />
                  </div>
                </div>
              )}
              {activeAsset.category === 'Navigation' && (
                <div className="w-16 h-16 rounded-xl bg-slate-800 border border-slate-700 flex items-center justify-center text-2xl font-bold text-amber-500">
                  ⮡
                </div>
              )}
              {activeAsset.category === 'Vehicle' && (
                <div className="text-5xl opacity-80">🚗</div>
              )}
              {activeAsset.category === 'Menu Icons' && (
                <div className="text-5xl opacity-80">{activeAsset.originalStyle}</div>
              )}
              {activeAsset.category === 'Climate' && (
                <div className="w-20 h-20 rounded-full border-4 border-amber-500/40 flex items-center justify-center text-sm font-mono text-slate-300">
                  21.5°
                </div>
              )}
              {activeAsset.category === 'Textures' && (
                <div className="w-48 h-24 rounded border border-slate-700 bg-slate-900/80 flex items-center justify-center text-xs font-mono text-slate-400">
                  Default Smooth Obsidian
                </div>
              )}
            </div>

            <div className="mt-4 text-[11px] font-mono text-slate-500">Original Format: RLE8 / Precomp Bitmap</div>
          </div>

          {/* Right: Gemini Nano Banana AI Modified Asset */}
          <div className="flex flex-col items-center justify-center p-6 bg-gradient-to-b from-amber-950/10 to-black/60 border border-amber-500/30 rounded-lg relative shadow-[0_0_30px_rgba(245,158,11,0.05)]">
            <span className="absolute top-3 left-3 text-[10px] font-mono text-amber-400 uppercase tracking-wider font-bold flex items-center gap-1">
              <span>🍌 Gemini Nano Banana Restyled</span>
            </span>

            {/* Simulated Visual Render */}
            <div className="my-auto flex flex-col items-center justify-center">
              {isGenerating ? (
                <div className="flex flex-col items-center gap-3">
                  <div className="w-8 h-8 border-3 border-amber-500 border-t-transparent rounded-full animate-spin" />
                  <span className="text-xs font-mono text-amber-400 animate-pulse">
                    Synthesizing with {selectedModel}...
                  </span>
                </div>
              ) : (
                <>
                  {activeAsset.category === 'Gauges' && (
                    <div className="relative w-24 h-24 flex items-center justify-center">
                      <div className="w-20 h-20 rounded-full border-2 border-amber-500/60 flex items-center justify-center shadow-[0_0_15px_rgba(245,158,11,0.2)]">
                        <div
                          className="w-1.5 h-16 rounded-full transition-all shadow-[0_0_10px_currentColor]"
                          style={{
                            backgroundColor: activeAsset.currentStyle,
                            color: activeAsset.currentStyle,
                            transform: 'rotate(-45deg)',
                          }}
                        />
                      </div>
                    </div>
                  )}
                  {activeAsset.category === 'Navigation' && (
                    <div
                      className="w-16 h-16 rounded-xl flex items-center justify-center text-2xl font-bold shadow-xl transition-all"
                      style={{
                        backgroundColor: activeAsset.currentStyle,
                        color: '#000',
                        boxShadow: `0 0 20px ${activeAsset.currentStyle}80`,
                      }}
                    >
                      ⮡
                    </div>
                  )}
                  {activeAsset.category === 'Vehicle' && (
                    <div
                      className="text-5xl transition-all"
                      style={{ filter: `drop-shadow(0 0 10px ${activeAsset.currentStyle})` }}
                    >
                      🏎️
                    </div>
                  )}
                  {activeAsset.category === 'Menu Icons' && (
                    <div
                      className="text-5xl transition-all"
                      style={{ filter: `drop-shadow(0 0 12px ${activeAsset.currentStyle})` }}
                    >
                      {activeAsset.originalStyle}
                    </div>
                  )}
                  {activeAsset.category === 'Climate' && (
                    <div
                      className="w-20 h-20 rounded-full border-4 flex items-center justify-center text-sm font-mono text-white font-bold transition-all"
                      style={{
                        borderColor: activeAsset.currentStyle,
                        boxShadow: `0 0 20px ${activeAsset.currentStyle}60`,
                      }}
                    >
                      21.5°
                    </div>
                  )}
                  {activeAsset.category === 'Textures' && (
                    <div className="w-48 h-24 rounded border border-amber-500/40 bg-[#0d121c] flex flex-col items-center justify-center text-xs font-mono text-amber-300 shadow-inner">
                      <span>Carbon Weave Matrix</span>
                      <span className="text-[9px] text-slate-500">2x2 Twill · Matte Clearcoat</span>
                    </div>
                  )}
                </>
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
              'RS Crimson Sport with sharp glowing glass tip',
              'Audi Sport Amber with aerospace titanium bevel',
              'Modern Neon Cyan with high-contrast night vision',
              'Matte 2x2 Carbon Fiber Twill Weave',
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

          {/* Generation & Egress Airlock Strip */}
          <div className="flex items-center justify-between pt-2 border-t border-slate-800/80">
            <div className="flex items-center gap-3 text-xs text-slate-400 font-mono">
              <span className="flex items-center gap-1">
                <span className="w-2 h-2 rounded-full bg-emerald-400" />
                Airlock Isolated
              </span>
              <span>•</span>
              <span>Calls this session: {airlockTransmissionsCount}</span>
              <span>•</span>
              <span>Bytes sent: {(airlockBytesCount / 1024).toFixed(1)} KB</span>
            </div>

            <div className="flex items-center gap-2">
              {onResetAsset && activeAsset.status === 'AI Modified' && (
                <button
                  onClick={() => onResetAsset(activeAsset.id)}
                  className="px-3.5 py-2.5 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-semibold rounded-lg border border-slate-700 transition flex items-center gap-1.5"
                >
                  <span>↺</span> Reset to Stock
                </button>
              )}
              <button
                onClick={() => setIsAirlockOpen(true)}
                disabled={isGenerating || !promptText.trim()}
                className="px-5 py-2.5 bg-amber-500 hover:bg-amber-400 disabled:bg-slate-800 disabled:text-slate-600 text-slate-950 text-xs font-bold rounded-lg shadow-lg flex items-center gap-2 transition-all scale-100 hover:scale-[1.02]"
              >
                <span>🍌</span> Generate with Gemini Nano Banana
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* MANDATORY EGRESS AIRLOCK CONFIRMATION MODAL (§12.3) */}
      {isAirlockOpen && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
          <div className="w-[500px] bg-slate-900 border-2 border-amber-500/80 rounded-xl shadow-2xl p-6 space-y-4">
            <div className="flex items-center justify-between border-b border-slate-800 pb-3">
              <div className="flex items-center gap-2">
                <span className="text-xl">🛡️</span>
                <div>
                  <h3 className="text-sm font-bold text-white uppercase tracking-wider">
                    Egress Airlock Confirmation (§12.3)
                  </h3>
                  <span className="text-[10px] text-amber-400 font-mono">Mandatory Single-Call Gate</span>
                </div>
              </div>
              <button
                onClick={() => setIsAirlockOpen(false)}
                className="text-slate-400 hover:text-white text-xs font-mono"
              >
                ✕ CANCEL
              </button>
            </div>

            <p className="text-xs text-slate-300 leading-relaxed">
              Audi MMI Studio operates offline-first. AI asset authoring is the single permitted network egress.
              Please review the exact asset and parameters to be transmitted:
            </p>

            <div className="p-3.5 bg-slate-950 border border-slate-800 rounded-lg space-y-2 text-xs font-mono">
              <div className="flex justify-between">
                <span className="text-slate-400">Target Asset:</span>
                <span className="text-white font-bold">{activeAsset.name}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-slate-400">Payload Dimensions:</span>
                <span className="text-amber-400">{activeAsset.width} × {activeAsset.height} px (RGBA8888)</span>
              </div>
              <div className="flex justify-between">
                <span className="text-slate-400">Resolved Model:</span>
                <span className="text-emerald-400">{selectedModel}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-slate-400">Egress Deny-List Check:</span>
                <span className="text-emerald-400 font-bold">✓ PASSED (Non-signed cosmetic asset)</span>
              </div>
              <div className="flex justify-between">
                <span className="text-slate-400">SynthID Watermarking:</span>
                <span className="text-blue-400">Mandatory Active</span>
              </div>
            </div>

            <div className="p-2.5 bg-amber-950/40 border border-amber-800/60 rounded text-[11px] text-amber-200">
              <strong>Egress Guarantee:</strong> No VIN, firmware binaries, file paths, or private vehicle metadata will ever be transmitted.
            </div>

            <div className="flex justify-end gap-3 pt-2">
              <button
                onClick={() => setIsAirlockOpen(false)}
                className="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-semibold rounded"
              >
                Deny & Abort
              </button>
              <button
                onClick={handleConfirmAirlockAndGenerate}
                className="px-5 py-2 bg-amber-500 hover:bg-amber-400 text-slate-950 text-xs font-bold rounded shadow"
              >
                Confirm Egress & Generate
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
