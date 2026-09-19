import React, { useState } from 'react';
import { MMIThemeConfig, SystemString } from '../types';

interface ComponentCustomizerProps {
  themeConfig: MMIThemeConfig;
  onUpdateTheme: (newConfig: Partial<MMIThemeConfig>) => void;
  strings: SystemString[];
  onExportRecipe: () => void;
  onNavigateToAiStudio?: () => void;
}

export const ComponentCustomizer: React.FC<ComponentCustomizerProps> = ({
  themeConfig,
  onUpdateTheme,
  strings,
  onExportRecipe,
  onNavigateToAiStudio,
}) => {
  const [activeScreenTab, setActiveScreenTab] = useState<'navigation' | 'car_setup' | 'media' | 'climate' | 'carousel'>('navigation');
  const [activeDriveMode, setActiveDriveMode] = useState<'comfort' | 'auto' | 'dynamic' | 'individual'>('dynamic');

  // Helper to fetch localized string by key
  const getStr = (id: string, fallback: string): string => {
    const item = strings.find((s) => s.id === id);
    if (!item) return fallback;
    if (themeConfig.language === 'sq') return item.sq;
    if (themeConfig.language === 'de') return item.de;
    return item.en;
  };

  const presetAccents = [
    { name: 'Audi Sport Amber', color: '#FF9900' },
    { name: 'RS Misano Red', color: '#E0001B' },
    { name: 'Turbo Blue', color: '#0077FF' },
    { name: 'Lime Green', color: '#10B981' },
    { name: 'Glacier White', color: '#E2E8F0' },
  ];

  return (
    <div className="flex h-full bg-slate-950 text-slate-100 overflow-hidden font-sans">
      {/* Left Customizer Control Panel */}
      <div className="w-[360px] flex flex-col border-r border-slate-800 bg-slate-900/60 overflow-y-auto p-4 space-y-5">
        <div>
          <h2 className="text-base font-bold tracking-tight text-white flex items-center gap-2">
            <span>🎨 UI & Component Studio</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">
            Real-time 800x480 screen theming, font metrics, and component configuration
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
              { id: 'navigation', label: 'Navigation 3D' },
              { id: 'car_setup', label: 'Drive Select' },
              { id: 'media', label: 'Media Jukebox' },
              { id: 'climate', label: 'Climate Control' },
              { id: 'carousel', label: 'Main Carousel' },
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveScreenTab(tab.id as any)}
                className={`px-2.5 py-1.5 text-xs rounded border transition-all text-left ${
                  activeScreenTab === tab.id
                    ? 'bg-amber-500/20 border-amber-500 text-amber-400 font-bold'
                    : 'bg-slate-950 border-slate-800 text-slate-400 hover:text-slate-200'
                }`}
              >
                {tab.label}
              </button>
            ))}
          </div>
        </div>

        {/* Accent Color Palette */}
        <div className="space-y-2">
          <div className="flex justify-between items-center">
            <label className="text-xs font-bold uppercase tracking-wider text-slate-400">
              Accent Color
            </label>
            <span className="text-[10px] font-mono text-amber-400 font-bold">{themeConfig.accentColor}</span>
          </div>

          <div className="flex items-center gap-2">
            {presetAccents.map((p) => (
              <button
                key={p.color}
                onClick={() => onUpdateTheme({ accentColor: p.color })}
                title={p.name}
                style={{ backgroundColor: p.color }}
                className={`w-7 h-7 rounded-full border-2 transition-transform ${
                  themeConfig.accentColor === p.color
                    ? 'border-white scale-110 shadow-[0_0_10px_rgba(255,255,255,0.4)]'
                    : 'border-transparent hover:scale-105'
                }`}
              />
            ))}
            <input
              type="color"
              value={themeConfig.accentColor}
              onChange={(e) => onUpdateTheme({ accentColor: e.target.value })}
              className="w-7 h-7 rounded border border-slate-700 bg-transparent cursor-pointer"
              title="Custom Hex Color"
            />
          </div>
        </div>

        {/* Needle & Gauge Color */}
        <div className="space-y-2">
          <div className="flex justify-between items-center">
            <label className="text-xs font-bold uppercase tracking-wider text-slate-400">
              Gauge Needle & Indicators
            </label>
            <span className="text-[10px] font-mono text-slate-400">{themeConfig.needleColor}</span>
          </div>
          <div className="flex gap-2">
            {['#FF0000', '#FF9900', '#00E5FF', '#FFFFFF'].map((color) => (
              <button
                key={color}
                onClick={() => onUpdateTheme({ needleColor: color })}
                style={{ backgroundColor: color }}
                className={`w-6 h-6 rounded-md border ${
                  themeConfig.needleColor === color ? 'border-white ring-2 ring-amber-500' : 'border-slate-800'
                }`}
              />
            ))}
          </div>
        </div>

        {/* Typography Customizer */}
        <div className="space-y-3 pt-2 border-t border-slate-800">
          <label className="block text-xs font-bold uppercase tracking-wider text-slate-400">
            Typography & Font Rendering
          </label>

          <div>
            <span className="block text-[11px] text-slate-400 mb-1">Font Family (Linotype / MMI)</span>
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
            <div className="flex justify-between text-[11px] text-slate-400 mb-1">
              <span>Font Scale</span>
              <span>{Math.round(themeConfig.fontSizeScale * 100)}%</span>
            </div>
            <input
              type="range"
              min="0.8"
              max="1.3"
              step="0.05"
              value={themeConfig.fontSizeScale}
              onChange={(e) => onUpdateTheme({ fontSizeScale: Number(e.target.value) })}
              className="w-full accent-amber-500"
            />
          </div>

          {/* Diacritic Glyphs Test */}
          <div className="p-2 bg-slate-950 border border-slate-800 rounded flex items-center justify-between text-xs">
            <span className="text-slate-400">Albanian Diacritics:</span>
            <span className="font-bold text-amber-400 font-mono tracking-wider">ë  ç  Ë  Ç</span>
          </div>
        </div>

        {/* Component Visibility Toggles */}
        <div className="space-y-2 pt-2 border-t border-slate-800">
          <label className="block text-xs font-bold uppercase tracking-wider text-slate-400">
            Component Elements
          </label>

          <label className="flex items-center justify-between text-xs text-slate-300 cursor-pointer">
            <span>Show Top Status Bar</span>
            <input
              type="checkbox"
              checked={themeConfig.showStatusBar}
              onChange={(e) => onUpdateTheme({ showStatusBar: e.target.checked })}
              className="accent-amber-500"
            />
          </label>

          <label className="flex items-center justify-between text-xs text-slate-300 cursor-pointer">
            <span>Show Digital Clock</span>
            <input
              type="checkbox"
              checked={themeConfig.showClock}
              onChange={(e) => onUpdateTheme({ showClock: e.target.checked })}
              className="accent-amber-500"
            />
          </label>

          <label className="flex items-center justify-between text-xs text-slate-300 cursor-pointer">
            <span>Show Outside Temperature</span>
            <input
              type="checkbox"
              checked={themeConfig.showTemperature}
              onChange={(e) => onUpdateTheme({ showTemperature: e.target.checked })}
              className="accent-amber-500"
            />
          </label>

          <label className="flex items-center justify-between text-xs text-slate-300 cursor-pointer">
            <span>Show 3D Compass Rose</span>
            <input
              type="checkbox"
              checked={themeConfig.showCompass}
              onChange={(e) => onUpdateTheme({ showCompass: e.target.checked })}
              className="accent-amber-500"
            />
          </label>

          <label className="flex items-center justify-between text-xs text-slate-300 cursor-pointer">
            <span>Ambient UI Glow</span>
            <input
              type="checkbox"
              checked={themeConfig.ambientGlow}
              onChange={(e) => onUpdateTheme({ ambientGlow: e.target.checked })}
              className="accent-amber-500"
            />
          </label>
        </div>

        {/* Export Theme Recipe Button */}
        <div className="pt-2 space-y-2">
          {onNavigateToAiStudio && (
            <button
              onClick={onNavigateToAiStudio}
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
        </div>
      </div>

      {/* Center 800x480 Virtual Screen Display Stage */}
      <div className="flex-1 flex flex-col items-center justify-center p-6 bg-[#080a0f] overflow-auto">
        <div className="mb-3 flex items-center justify-between w-[800px] text-xs text-slate-400">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
            <span className="font-mono">MMI 3G+ VIRTUAL CLUSTER [800 × 480 @ 60 Hz]</span>
          </div>
          <span className="font-mono text-amber-400">
            Language: {themeConfig.language === 'sq' ? '🇦🇱 Gjuha Shqipe' : themeConfig.language === 'de' ? '🇩🇪 Deutsch' : '🇬🇧 English'}
          </span>
        </div>

        {/* Physical 800x480 Screen Chassis */}
        <div
          className="relative bg-[#0c0e14] border-4 border-slate-800 rounded-xl shadow-[0_0_50px_rgba(0,0,0,0.9)] overflow-hidden flex flex-col select-none"
          style={{
            width: '800px',
            height: '480px',
            fontFamily: themeConfig.fontFamily.includes('Bold') ? 'sans-serif' : 'sans-serif',
            fontWeight: themeConfig.fontFamily.includes('Bold') ? 700 : 500,
          }}
        >
          {/* Top Status Bar */}
          {themeConfig.showStatusBar && (
            <div className="flex justify-between items-center px-4 py-2 bg-black/60 border-b border-slate-800/80 text-xs font-mono z-10 backdrop-blur-sm">
              <div className="flex items-center gap-3 text-slate-300">
                {themeConfig.showClock && <span>14:35</span>}
                {themeConfig.showTemperature && <span>+22.0°C</span>}
              </div>
              <div
                className="font-bold tracking-wider"
                style={{
                  color: themeConfig.accentColor,
                  textShadow: themeConfig.ambientGlow ? `0 0 10px ${themeConfig.accentColor}` : 'none',
                }}
              >
                {getStr('NAV_ROUTE_GUIDANCE', 'AUDI NAVIGATION PLUS')}
              </div>
              <div className="flex items-center gap-3 text-slate-400 text-[11px]">
                <span>TMC PRO</span>
                <span className="text-emerald-400">3G LTE</span>
              </div>
            </div>
          )}

          {/* SCREEN VIEW 1: NAVIGATION */}
          {activeScreenTab === 'navigation' && (
            <div className="flex-1 relative flex flex-col justify-between overflow-hidden bg-[#0d121c]">
              {/* Virtual Map Road Geometry Graphics */}
              <div className="absolute inset-0 opacity-40">
                <svg className="w-full h-full" viewBox="0 0 800 400">
                  {/* Grid / Horizon lines */}
                  <line x1="0" y1="200" x2="800" y2="200" stroke="#1e293b" strokeWidth="1" />
                  <polygon points="0,200 800,200 800,400 0,400" fill="#0f172a" />
                  {/* 3D Perspective Road */}
                  <polygon points="380,200 420,200 650,400 150,400" fill="#1e293b" />
                  <line x1="400" y1="200" x2="400" y2="400" stroke="#f8fafc" strokeWidth="4" strokeDasharray="16,16" />
                  {/* Turn Route Vector */}
                  <path
                    d="M 400 360 L 400 250 L 520 220"
                    fill="none"
                    stroke={themeConfig.accentColor}
                    strokeWidth="8"
                    strokeLinecap="round"
                    style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 8px ${themeConfig.accentColor})` : 'none' }}
                  />
                </svg>
              </div>

              {/* Next Turn Banner (Top) */}
              <div className="relative z-10 m-4 p-3 bg-black/75 border border-slate-800 rounded-lg flex items-center justify-between backdrop-blur-md">
                <div className="flex items-center gap-3">
                  {/* Maneuver Arrow Icon */}
                  <div
                    className="w-10 h-10 rounded-lg flex items-center justify-center font-bold text-xl"
                    style={{ backgroundColor: themeConfig.accentColor, color: '#000' }}
                  >
                    ⮡
                  </div>
                  <div>
                    <div
                      className="text-sm font-bold text-white tracking-tight"
                      style={{ fontSize: `${15 * themeConfig.fontSizeScale}px` }}
                    >
                      {getStr('NAV_NEXT_TURN', 'Turn right in 300 m onto Autostrada A1')}
                    </div>
                    <div className="text-[11px] text-slate-400 font-mono mt-0.5">
                      {themeConfig.language === 'sq' ? 'Drejt: Tirana / Durrës' : 'Toward: Tirana / Durrës'}
                    </div>
                  </div>
                </div>

                {/* Speed Limit Badge */}
                <div className="w-10 h-10 rounded-full border-4 border-red-600 bg-white flex items-center justify-center text-black font-black text-xs font-mono shadow-lg">
                  130
                </div>
              </div>

              {/* 3D Compass Rose (Right) */}
              {themeConfig.showCompass && (
                <div className="absolute right-4 top-24 z-10 w-16 h-16 rounded-full bg-black/60 border border-slate-700 flex flex-col items-center justify-center backdrop-blur-sm">
                  <div className="text-[9px] font-bold text-red-500">N</div>
                  <div
                    className="w-1 h-7 rounded-full transition-transform duration-500"
                    style={{ backgroundColor: themeConfig.needleColor, transform: 'rotate(25deg)' }}
                  />
                  <div className="text-[9px] font-bold text-slate-400">S</div>
                </div>
              )}

              {/* Bottom Nav Stats Bar */}
              <div className="relative z-10 flex justify-between items-center px-4 py-2 bg-black/80 border-t border-slate-800 text-xs font-mono">
                <div className="flex gap-4 text-slate-300">
                  <span>ETA: 15:42</span>
                  <span>Dist: 74 km</span>
                </div>
                <div className="text-slate-400">
                  {themeConfig.language === 'sq' ? 'Harta: 2026 ECE Shqipëria' : 'Map: 2026 ECE Western Balkans'}
                </div>
              </div>
            </div>
          )}

          {/* SCREEN VIEW 2: CAR SETUP / DRIVE SELECT */}
          {activeScreenTab === 'car_setup' && (
            <div className="flex-1 relative flex flex-col items-center justify-between p-6 bg-[#0a0d13]">
              <div
                className="text-lg font-bold tracking-tight"
                style={{
                  color: themeConfig.accentColor,
                  textShadow: themeConfig.ambientGlow ? `0 0 12px ${themeConfig.accentColor}` : 'none',
                }}
              >
                {getStr('CAR_DRIVE_SELECT', 'Audi Drive Select')}
              </div>

              {/* Vehicle Silhouette & Drive Select Radial Dial */}
              <div className="relative w-full flex-1 flex items-center justify-center">
                <div className="flex items-center gap-6 z-10">
                  {[
                    { id: 'comfort', label: getStr('CAR_MODE_COMFORT', 'Comfort') },
                    { id: 'auto', label: 'Auto' },
                    { id: 'dynamic', label: getStr('CAR_MODE_DYNAMIC', 'Dynamic') },
                    { id: 'individual', label: getStr('CAR_MODE_INDIVIDUAL', 'Individual') },
                  ].map((mode) => {
                    const isSelected = activeDriveMode === mode.id;
                    return (
                      <button
                        key={mode.id}
                        onClick={() => setActiveDriveMode(mode.id as any)}
                        style={{
                          borderColor: isSelected ? themeConfig.accentColor : '#334155',
                          backgroundColor: isSelected ? `${themeConfig.accentColor}20` : '#0f172a',
                          color: isSelected ? '#ffffff' : '#94a3b8',
                        }}
                        className={`px-5 py-4 rounded-xl border-2 font-bold text-sm transition-all flex flex-col items-center gap-1 ${
                          isSelected ? 'scale-110 shadow-lg' : 'hover:border-slate-600'
                        }`}
                      >
                        <span>{mode.label}</span>
                        {isSelected && (
                          <span
                            className="w-2 h-2 rounded-full"
                            style={{ backgroundColor: themeConfig.needleColor }}
                          />
                        )}
                      </button>
                    );
                  })}
                </div>
              </div>

              {/* Diagnostics / Service info footer */}
              <div className="w-full flex justify-between text-xs text-slate-400 font-mono pt-3 border-t border-slate-800">
                <span>{getStr('CAR_OIL_LEVEL', 'Electronic Oil Level: OK')}</span>
                <span>{getStr('CAR_SERVICE_INTERVALS', 'Service in: 14,200 km')}</span>
              </div>
            </div>
          )}

          {/* SCREEN VIEW 3: MEDIA JUKEBOX */}
          {activeScreenTab === 'media' && (
            <div className="flex-1 relative flex flex-col justify-between p-6 bg-[#0b0e14]">
              {/* Media Sources Bar */}
              <div className="flex gap-2 pb-3 border-b border-slate-800 text-xs">
                {['MEDIA_JUKEBOX', 'MEDIA_SD_CARD_1', 'MEDIA_SD_CARD_2', 'MEDIA_BT_AUDIO'].map((k) => (
                  <span
                    key={k}
                    className={`px-3 py-1 rounded font-medium ${
                      k === 'MEDIA_JUKEBOX'
                        ? 'bg-amber-500/20 text-amber-400 font-bold border border-amber-500/50'
                        : 'text-slate-400'
                    }`}
                  >
                    {getStr(k, k)}
                  </span>
                ))}
              </div>

              {/* Now Playing Widget */}
              <div className="flex items-center gap-6 my-auto">
                <div className="w-32 h-32 rounded-lg bg-slate-800 border border-slate-700 flex items-center justify-center font-bold text-3xl text-slate-600 shadow-xl">
                  🎵
                </div>
                <div className="space-y-2">
                  <div
                    className="text-xl font-bold text-white tracking-tight"
                    style={{ fontSize: `${18 * themeConfig.fontSizeScale}px` }}
                  >
                    {themeConfig.language === 'sq' ? 'Këngë Tradicionale / Shqip Acoustic' : 'Audi Sound Selection'}
                  </div>
                  <div className="text-sm text-slate-300">
                    {themeConfig.language === 'sq' ? 'Artisti: Orkestra Shqiptare' : 'Artist: Master Symphony'}
                  </div>
                  <div className="text-xs text-slate-500 font-mono">
                    Album: Audi 3G+ High Fidelity Master · 320 kbps MP3
                  </div>
                </div>
              </div>

              {/* Audio Progress Bar */}
              <div className="space-y-1.5">
                <div className="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
                  <div
                    className="h-full rounded-full transition-all"
                    style={{ width: '45%', backgroundColor: themeConfig.accentColor }}
                  />
                </div>
                <div className="flex justify-between text-[11px] font-mono text-slate-400">
                  <span>02:14</span>
                  <span>04:58</span>
                </div>
              </div>
            </div>
          )}

          {/* SCREEN VIEW 4: CLIMATE CONTROL */}
          {activeScreenTab === 'climate' && (
            <div className="flex-1 relative flex flex-col justify-between p-6 bg-[#0a0d14]">
              <div className="text-center">
                <h3
                  className="text-base font-bold tracking-wider"
                  style={{ color: themeConfig.accentColor }}
                >
                  {getStr('CLIMATE_DRIVER_PASSENGER', 'Dual-Zone Auto Climate')}
                </h3>
              </div>

              <div className="flex items-center justify-around my-auto">
                {/* Driver Temp Dial */}
                <div className="text-center space-y-1">
                  <span className="text-xs text-slate-400 font-semibold block">
                    {themeConfig.language === 'sq' ? 'Shoferi' : 'Driver'}
                  </span>
                  <div
                    className="text-4xl font-black font-mono tracking-tight"
                    style={{ color: themeConfig.needleColor }}
                  >
                    21.5°C
                  </div>
                  <span className="text-[10px] text-emerald-400 font-mono">AUTO · AC ON</span>
                </div>

                {/* Central Blower Distribution */}
                <div className="p-4 bg-slate-900/80 border border-slate-800 rounded-xl flex flex-col items-center gap-2">
                  <div className="text-xs text-slate-300 font-bold">{getStr('CLIMATE_SYNC', 'SYNC')}</div>
                  <div className="w-24 h-2 bg-slate-800 rounded-full overflow-hidden">
                    <div className="w-3/5 h-full bg-amber-500" />
                  </div>
                  <span className="text-[10px] text-slate-400 font-mono">Blower Speed 3</span>
                </div>

                {/* Passenger Temp Dial */}
                <div className="text-center space-y-1">
                  <span className="text-xs text-slate-400 font-semibold block">
                    {themeConfig.language === 'sq' ? 'Pasagjeri' : 'Passenger'}
                  </span>
                  <div
                    className="text-4xl font-black font-mono tracking-tight"
                    style={{ color: themeConfig.needleColor }}
                  >
                    21.5°C
                  </div>
                  <span className="text-[10px] text-emerald-400 font-mono">AUTO · SYNC</span>
                </div>
              </div>

              <div className="flex justify-between items-center text-xs text-slate-400 font-mono pt-3 border-t border-slate-800">
                <span>{getStr('CLIMATE_DEFROST_MAX', 'Defrost MAX')}</span>
                <span>Air Quality Sensor: CLEAN</span>
              </div>
            </div>
          )}

          {/* SCREEN VIEW 5: MAIN CAROUSEL */}
          {activeScreenTab === 'carousel' && (
            <div className="flex-1 relative flex flex-col items-center justify-center p-6 bg-[#090c12]">
              {/* Rotating Arc Wheel Simulation */}
              <div className="relative w-80 h-44 flex items-center justify-center">
                <div
                  className="absolute inset-0 rounded-full border-4 border-dashed border-slate-800/80"
                  style={{ transform: 'rotate(-20deg)' }}
                />
                <div className="flex items-center gap-4 z-10">
                  {[
                    { label: themeConfig.language === 'sq' ? 'Udhëtimi' : 'Navigation', icon: '🧭' },
                    { label: themeConfig.language === 'sq' ? 'Telefoni' : 'Telephone', icon: '📞' },
                    { label: themeConfig.language === 'sq' ? 'Automjeti' : 'Car Setup', icon: '🚗', active: true },
                    { label: themeConfig.language === 'sq' ? 'Muzika' : 'Media', icon: '🎵' },
                  ].map((item, idx) => (
                    <div
                      key={idx}
                      className={`flex flex-col items-center p-3 rounded-xl border transition-all ${
                        item.active
                          ? 'bg-amber-500/20 border-amber-500 scale-125 shadow-[0_0_20px_rgba(245,158,11,0.3)]'
                          : 'border-slate-800 bg-slate-900/60 opacity-60'
                      }`}
                    >
                      <span className="text-2xl">{item.icon}</span>
                      <span
                        className="text-[11px] font-bold mt-1 text-white"
                        style={{ color: item.active ? themeConfig.accentColor : '#ffffff' }}
                      >
                        {item.label}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {/* Bottom Hardkey/Softkey Guide Bar */}
          <div className="flex justify-between items-center px-4 py-1.5 bg-slate-950 border-t border-slate-800/80 text-[10px] text-slate-500 font-mono">
            <span>[NAV] [INFO] [CAR] [SETUP]</span>
            <span>AUDI MMI 3G HIGH / PLUS SIMULATOR</span>
            <span>[RETURN] [OPTION]</span>
          </div>
        </div>
      </div>
    </div>
  );
};
