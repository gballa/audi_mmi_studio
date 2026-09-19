import React, { useState } from 'react';
import { MMIThemeConfig, SystemString } from '../types';
import { ScreenCanvas } from './ScreenCanvas';

interface ComponentCustomizerProps {
  themeConfig: MMIThemeConfig;
  onUpdateTheme: (newConfig: Partial<MMIThemeConfig>) => void;
  onResetTheme?: () => void;
  strings: SystemString[];
  onExportRecipe: () => void;
  onNavigateToAiStudio?: (elementId?: string) => void;
}

export const ComponentCustomizer: React.FC<ComponentCustomizerProps> = ({
  themeConfig,
  onUpdateTheme,
  onResetTheme,
  strings,
  onExportRecipe,
  onNavigateToAiStudio,
}) => {
  const [activeScreenTab, setActiveScreenTab] = useState<'navigation' | 'car_setup' | 'media' | 'climate' | 'carousel'>('car_setup');
  const [selectedElementId, setSelectedElementId] = useState<string | null>(null);

  const presetAccents = [
    { name: 'RS Misano Red', color: '#E0001B' },
    { name: 'Audi Sport Amber', color: '#FF9900' },
    { name: 'Turbo Blue', color: '#0077FF' },
    { name: 'Lime Green', color: '#10B981' },
    { name: 'Glacier White', color: '#E2E8F0' },
  ];

  return (
    <div className="flex h-full bg-slate-950 text-slate-100 overflow-hidden font-sans">
      {/* Left Customizer Control Panel */}
      <div className="w-[380px] flex flex-col border-r border-slate-800 bg-slate-900/70 overflow-y-auto p-4 space-y-5">
        <div>
          <h2 className="text-base font-bold tracking-tight text-white flex items-center gap-2">
            <span>🎨 UI & Component Studio</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">
            Pixel-authentic Audi MMI 3G+ simulator, 4 corner softkeys & Drive Select customizer
          </p>
        </div>

        {/* Language Selector for Live Preview */}
        <div className="p-3 bg-slate-950 border border-slate-800 rounded-lg space-y-2">
          <label className="block text-xs font-bold uppercase tracking-wider text-slate-400">
            Active Display Language
          </label>
          <div className="grid grid-cols-3 gap-1.5">
            <button
              onClick={() => onUpdateTheme({ language: 'sq' })}
              className={`px-2 py-1.5 text-xs font-bold rounded transition-all flex items-center justify-center gap-1 ${
                themeConfig.language === 'sq'
                  ? 'bg-red-600 text-white shadow'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              <span>🇦🇱</span> Shqip
            </button>
            <button
              onClick={() => onUpdateTheme({ language: 'en' })}
              className={`px-2 py-1.5 text-xs font-bold rounded transition-all flex items-center justify-center gap-1 ${
                themeConfig.language === 'en'
                  ? 'bg-blue-600 text-white shadow'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              <span>🇬🇧</span> English
            </button>
            <button
              onClick={() => onUpdateTheme({ language: 'de' })}
              className={`px-2 py-1.5 text-xs font-bold rounded transition-all flex items-center justify-center gap-1 ${
                themeConfig.language === 'de'
                  ? 'bg-amber-600 text-white shadow'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              <span>🇩🇪</span> Deutsch
            </button>
          </div>
        </div>

        {/* Screen Selection Switcher */}
        <div className="space-y-1.5">
          <label className="block text-xs font-bold uppercase tracking-wider text-slate-400">
            Screen Template View
          </label>
          <div className="grid grid-cols-2 gap-1.5">
            {[
              { id: 'car_setup', label: '🚗 Drive Select' },
              { id: 'navigation', label: '🧭 Navigation 3D' },
              { id: 'media', label: '🎵 Media Jukebox' },
              { id: 'climate', label: '❄️ Climate Control' },
              { id: 'carousel', label: '🎡 Main Carousel' },
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => {
                  setActiveScreenTab(tab.id as any);
                  if (tab.id === 'car_setup') {
                    onUpdateTheme({ driveSelectView: 'platter' });
                  }
                }}
                className={`px-2.5 py-1.5 text-xs rounded border transition-all text-left ${
                  activeScreenTab === tab.id
                    ? 'bg-red-500/20 border-red-500 text-red-400 font-bold shadow-sm'
                    : 'bg-slate-950 border-slate-800 text-slate-400 hover:text-slate-200'
                }`}
              >
                {tab.label}
              </button>
            ))}
          </div>
        </div>

        {/* 1. SECTION: 4 CORNER INTERACTIVE SOFTKEYS */}
        <div className="p-3 bg-slate-950/80 border border-slate-800 rounded-lg space-y-3">
          <div className="flex items-center justify-between">
            <label className="text-xs font-bold uppercase tracking-wider text-red-400 flex items-center gap-1.5">
              <span>📐</span>
              <span>4 Corner Softkeys & Brackets</span>
            </label>
            <span className="text-[10px] text-slate-500 font-mono">MMI Dial Mapping</span>
          </div>

          <div>
            <div className="flex justify-between items-center mb-1">
              <span className="text-[11px] text-slate-400">Bracket Accent Glow Color:</span>
              <span className="text-[10px] font-mono text-red-400 font-bold">{themeConfig.cornerBracketColor}</span>
            </div>
            <div className="flex items-center gap-2">
              {presetAccents.map((p) => (
                <button
                  key={p.color}
                  onClick={() =>
                    onUpdateTheme({
                      cornerBracketColor: p.color,
                      accentColor: p.color,
                    })
                  }
                  title={p.name}
                  style={{ backgroundColor: p.color }}
                  className={`w-6 h-6 rounded-full border-2 transition-transform ${
                    themeConfig.cornerBracketColor === p.color
                      ? 'border-white scale-110 shadow-[0_0_8px_rgba(255,255,255,0.4)]'
                      : 'border-transparent hover:scale-105'
                  }`}
                />
              ))}
              <input
                type="color"
                value={themeConfig.cornerBracketColor || '#E0001B'}
                onChange={(e) =>
                  onUpdateTheme({
                    cornerBracketColor: e.target.value,
                    accentColor: e.target.value,
                  })
                }
                className="w-6 h-6 rounded border border-slate-700 bg-transparent cursor-pointer"
                title="Custom Hex Color"
              />
            </div>
          </div>

          {/* Corner Labels text */}
          <div className="space-y-2 pt-1 border-t border-slate-800/80">
            <div>
              <span className="text-[10px] font-mono text-slate-400">Bottom-Left Text (BL Softkey):</span>
              <input
                type="text"
                value={themeConfig.cornerSoftkeys.bottomLeft.text}
                onChange={(e) =>
                  onUpdateTheme({
                    cornerSoftkeys: {
                      ...themeConfig.cornerSoftkeys,
                      bottomLeft: { ...themeConfig.cornerSoftkeys.bottomLeft, text: e.target.value },
                    },
                  })
                }
                className="w-full mt-1 px-2.5 py-1 bg-slate-900 border border-slate-700 rounded text-xs text-white font-medium"
              />
            </div>

            <div>
              <span className="text-[10px] font-mono text-slate-400">Bottom-Right Text (BR Softkey):</span>
              <input
                type="text"
                value={themeConfig.cornerSoftkeys.bottomRight.text}
                onChange={(e) =>
                  onUpdateTheme({
                    cornerSoftkeys: {
                      ...themeConfig.cornerSoftkeys,
                      bottomRight: { ...themeConfig.cornerSoftkeys.bottomRight, text: e.target.value },
                    },
                  })
                }
                className="w-full mt-1 px-2.5 py-1 bg-slate-900 border border-slate-700 rounded text-xs text-white font-medium"
              />
            </div>
          </div>
        </div>

        {/* 2. SECTION: AUDI DRIVE SELECT CONTROLS */}
        {activeScreenTab === 'car_setup' && (
          <div className="p-3 bg-slate-950/80 border border-slate-800 rounded-lg space-y-3">
            <div className="flex items-center justify-between">
              <label className="text-xs font-bold uppercase tracking-wider text-amber-400 flex items-center gap-1.5">
                <span>🚗</span>
                <span>Audi Drive Select Controls</span>
              </label>
              <button
                onClick={() =>
                  onUpdateTheme({
                    driveSelectView: themeConfig.driveSelectView === 'platter' ? 'settings' : 'platter',
                  })
                }
                className="text-[10px] px-2 py-0.5 rounded bg-slate-800 hover:bg-slate-700 text-amber-300 font-mono transition"
              >
                Toggle {themeConfig.driveSelectView === 'platter' ? 'Settings' : 'Platter'}
              </button>
            </div>

            {/* Active Drive Mode Radio Pills */}
            <div>
              <span className="text-[11px] text-slate-400 block mb-1">Active Drive Mode:</span>
              <div className="grid grid-cols-2 gap-1.5">
                {(['comfort', 'auto', 'dynamic', 'individual'] as const).map((mode) => (
                  <button
                    key={mode}
                    onClick={() => onUpdateTheme({ activeDriveMode: mode })}
                    className={`px-2 py-1 text-xs rounded font-bold capitalize border transition ${
                      themeConfig.activeDriveMode === mode
                        ? 'bg-red-600/30 border-red-500 text-white'
                        : 'bg-slate-900 border-slate-800 text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    {mode}
                  </button>
                ))}
              </div>
            </div>

            {/* Submenu Dropdown Pickers */}
            <div className="space-y-1.5 pt-2 border-t border-slate-800/80">
              <span className="text-[11px] text-slate-400 block">Individual Mode Preset Parameters:</span>
              <div className="space-y-1 text-xs">
                {(['engineGearbox', 'steering', 'suspension'] as const).map((key) => (
                  <div key={key} className="flex items-center justify-between py-0.5">
                    <span className="text-slate-400 text-[11px] capitalize">
                      {key === 'engineGearbox' ? 'Engine / Gearbox' : key}:
                    </span>
                    <select
                      value={themeConfig.driveSelectSettings[key]}
                      onChange={(e) => {
                        onUpdateTheme({
                          driveSelectSettings: {
                            ...themeConfig.driveSelectSettings,
                            [key]: e.target.value as 'Comfort' | 'Auto' | 'Dynamic',
                          },
                        });
                      }}
                      className="px-2 py-1 rounded bg-[#0b0f19] border border-slate-700 text-amber-400 text-[11px] font-bold focus:border-red-500 focus:outline-none cursor-pointer"
                    >
                      <option value="Comfort">Comfort</option>
                      <option value="Auto">Auto</option>
                      <option value="Dynamic">Dynamic</option>
                    </select>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* 3. SECTION: STATUS BAR (32px bottom strip) CONTROLS */}
        <div className="p-3 bg-slate-950/80 border border-slate-800 rounded-lg space-y-2.5">
          <label className="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center gap-1.5">
            <span>📊</span>
            <span>Status Bar Controls</span>
          </label>

          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Digital Clock:</span>
            <input
              type="text"
              value={themeConfig.statusBar.clockTime}
              onChange={(e) =>
                onUpdateTheme({
                  statusBar: { ...themeConfig.statusBar, clockTime: e.target.value },
                })
              }
              className="w-16 px-1.5 py-0.5 bg-slate-900 border border-slate-700 rounded text-center text-xs font-mono text-white"
            />
          </div>

          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Audio Muted:</span>
            <input
              type="checkbox"
              checked={themeConfig.statusBar.isMuted}
              onChange={(e) =>
                onUpdateTheme({
                  statusBar: { ...themeConfig.statusBar, isMuted: e.target.checked },
                })
              }
              className="accent-red-500"
            />
          </div>

          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Bluetooth Connected:</span>
            <input
              type="checkbox"
              checked={themeConfig.statusBar.bluetoothConnected}
              onChange={(e) =>
                onUpdateTheme({
                  statusBar: { ...themeConfig.statusBar, bluetoothConnected: e.target.checked },
                })
              }
              className="accent-blue-500"
            />
          </div>

          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Cellular Signal ({themeConfig.statusBar.signalBars}/4):</span>
            <input
              type="range"
              min="0"
              max="4"
              value={themeConfig.statusBar.signalBars}
              onChange={(e) =>
                onUpdateTheme({
                  statusBar: { ...themeConfig.statusBar, signalBars: Number(e.target.value) },
                })
              }
              className="w-24 accent-emerald-500"
            />
          </div>

          <div className="flex items-center justify-between text-xs">
            <span className="text-slate-400">Google Online Services:</span>
            <input
              type="checkbox"
              checked={themeConfig.statusBar.googleServicesOnline}
              onChange={(e) =>
                onUpdateTheme({
                  statusBar: { ...themeConfig.statusBar, googleServicesOnline: e.target.checked },
                })
              }
              className="accent-emerald-500"
            />
          </div>
        </div>

        {/* 4. SECTION: TYPOGRAPHY & TEXTURES */}
        <div className="space-y-3 pt-2 border-t border-slate-800">
          <label className="block text-xs font-bold uppercase tracking-wider text-slate-400">
            Font & Background Texture
          </label>

          <div>
            <span className="block text-[11px] text-slate-400 mb-1">Font Family</span>
            <select
              value={themeConfig.fontFamily}
              onChange={(e) => onUpdateTheme({ fontFamily: e.target.value as any })}
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded text-xs text-slate-200"
            >
              <option value="AudiType-Normal">AudiType Normal</option>
              <option value="AudiType-Bold">AudiType Bold</option>
              <option value="AudiType-Extended">AudiType Extended</option>
              <option value="AudiUnivers-540">AudiUnivers 540 Medium</option>
            </select>
          </div>

          <div>
            <span className="block text-[11px] text-slate-400 mb-1">Background Surface Texture</span>
            <select
              value={themeConfig.activeBackgroundTexture}
              onChange={(e) => onUpdateTheme({ activeBackgroundTexture: e.target.value as any })}
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded text-xs text-slate-200"
            >
              <option value="default">Default Dark OEM</option>
              <option value="carbon_weave">Audi Sport Carbon Twill Weave</option>
              <option value="brushed_aluminum">Brushed Aluminum Inlay</option>
            </select>
          </div>

          <label className="flex items-center justify-between text-xs text-slate-300 cursor-pointer pt-1">
            <span>Ambient Neon Glow</span>
            <input
              type="checkbox"
              checked={themeConfig.ambientGlow}
              onChange={(e) => onUpdateTheme({ ambientGlow: e.target.checked })}
              className="accent-red-500"
            />
          </label>
        </div>

        {/* Action Buttons */}
        <div className="pt-2 space-y-2">
          {onNavigateToAiStudio && (
            <button
              onClick={() => onNavigateToAiStudio(selectedElementId || undefined)}
              className="w-full py-2 bg-gradient-to-r from-amber-500 to-amber-400 hover:from-amber-400 hover:to-amber-300 text-slate-950 text-xs font-bold rounded shadow transition flex items-center justify-center gap-1.5"
            >
              <span>🍌</span> Restyle Elements with Gemini AI
            </button>
          )}
          <button
            onClick={onExportRecipe}
            className="w-full py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded border border-slate-700 transition"
          >
            Export Custom Theme & UI Recipe
          </button>
          {onResetTheme && (
            <button
              onClick={onResetTheme}
              className="w-full py-2 bg-slate-900 hover:bg-slate-800 text-slate-400 hover:text-white text-xs font-medium rounded border border-slate-800 hover:border-slate-700 transition flex items-center justify-center gap-1.5"
              title="Revert all colors, needles, and component settings to OEM baseline"
            >
              <span>↺</span> Reset Theme to OEM Baseline
            </button>
          )}
        </div>
      </div>

      {/* Center 800x480 Virtual Screen Display Stage & Physical Console */}
      <div className="flex-1 flex flex-col items-center justify-center p-6 bg-[#04060a] overflow-auto">
        <ScreenCanvas
          themeConfig={themeConfig}
          onUpdateTheme={onUpdateTheme}
          activeScreenTab={activeScreenTab}
          onSelectScreenTab={setActiveScreenTab}
          strings={strings}
          selectedElementId={selectedElementId}
          onSelectElement={setSelectedElementId}
          onNavigateToAiStudio={onNavigateToAiStudio}
        />
      </div>
    </div>
  );
};
