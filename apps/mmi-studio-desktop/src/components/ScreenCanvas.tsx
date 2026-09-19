import React, { useState } from 'react';
import { MMIThemeConfig, SystemString } from '../types';

export interface ScreenCanvasProps {
  themeConfig: MMIThemeConfig;
  onUpdateTheme: (newConfig: Partial<MMIThemeConfig>) => void;
  activeScreenTab?: 'navigation' | 'car_setup' | 'media' | 'climate' | 'carousel';
  onSelectScreenTab?: (tab: 'navigation' | 'car_setup' | 'media' | 'climate' | 'carousel') => void;
  strings?: SystemString[];
  selectedElementId?: string | null;
  onSelectElement?: (elementId: string) => void;
  onNavigateToAiStudio?: (elementId?: string) => void;
}

export const ScreenCanvas: React.FC<ScreenCanvasProps> = ({
  themeConfig,
  onUpdateTheme,
  activeScreenTab = 'car_setup',
  onSelectScreenTab,
  strings = [],
  selectedElementId,
  onSelectElement,
  onNavigateToAiStudio,
}) => {
  const [knobRotation, setKnobRotation] = useState<number>(0);
  const [activeSubmenuRow, setActiveSubmenuRow] = useState<'engine' | 'steering' | 'suspension'>('steering');
  const [openDropdownRow, setOpenDropdownRow] = useState<'engine' | 'steering' | 'suspension' | null>(null);
  const [navViewMode, setNavViewMode] = useState<'perspective' | 'interactive_vector'>('interactive_vector');
  const [inspectModalElement, setInspectModalElement] = useState<string | null>(null);

  // Localized string helper
  const getStr = (id: string, fallback: string): string => {
    const item = strings.find((s) => s.id === id);
    if (!item) return fallback;
    if (themeConfig.language === 'sq') return item.sq;
    if (themeConfig.language === 'de') return item.de;
    return item.en;
  };

  const driveModes = ['comfort', 'auto', 'dynamic', 'individual'] as const;

  const handleRotateKnob = (direction: 'left' | 'right') => {
    setKnobRotation((prev) => prev + (direction === 'left' ? -30 : 30));
    const currentIndex = driveModes.indexOf(themeConfig.activeDriveMode);
    let nextIndex = direction === 'left' ? currentIndex - 1 : currentIndex + 1;
    if (nextIndex < 0) nextIndex = driveModes.length - 1;
    if (nextIndex >= driveModes.length) nextIndex = 0;
    onUpdateTheme({ activeDriveMode: driveModes[nextIndex] });
  };

  const handleCornerClick = (corner: 'topLeft' | 'topRight' | 'bottomLeft' | 'bottomRight') => {
    if (corner === 'bottomRight') {
      // Toggle Drive Select Platter vs Settings Submenu (matching media_1789830294659.png <-> media_1789830294656.png)
      const nextView = themeConfig.driveSelectView === 'platter' ? 'settings' : 'platter';
      onUpdateTheme({ driveSelectView: nextView });
      if (onSelectScreenTab) onSelectScreenTab('car_setup');
    } else if (corner === 'bottomLeft') {
      // Car systems overview
      if (onSelectScreenTab) onSelectScreenTab('car_setup');
      onUpdateTheme({ driveSelectView: 'platter' });
    } else if (corner === 'topLeft') {
      if (themeConfig.driveSelectView === 'settings') {
        onUpdateTheme({ driveSelectView: 'platter' });
      }
    }
  };

  const handleElementClick = (e: React.MouseEvent, elementId: string) => {
    e.stopPropagation();
    if (onSelectElement) onSelectElement(elementId);
    setInspectModalElement(elementId);
  };

  const bracketColor = themeConfig.cornerBracketColor || themeConfig.accentColor || '#E0001B';

  return (
    <div className="flex flex-col items-center justify-center p-4 bg-[#06080d] select-none text-slate-100">
      {/* Top Telemetry / Canvas Resolution Bar */}
      <div className="mb-2 flex items-center justify-between w-[800px] text-xs text-slate-400 font-mono">
        <div className="flex items-center gap-2">
          <span className="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_8px_rgba(16,185,129,0.8)]" />
          <span className="font-bold text-slate-200">AUDI MMI 3G+ DISPLAY SIMULATOR</span>
          <span className="text-slate-500">[800 × 480 WVGA @ 60 FPS]</span>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-amber-400 font-bold">
            {themeConfig.language === 'sq' ? '🇦🇱 Shqip' : themeConfig.language === 'de' ? '🇩🇪 Deutsch' : '🇬🇧 English'}
          </span>
          <span className="text-slate-600">|</span>
          <span className="text-slate-400">
            View: <span className="text-white font-bold">{activeScreenTab === 'car_setup' ? `Drive Select (${themeConfig.driveSelectView})` : activeScreenTab}</span>
          </span>
        </div>
      </div>

      {/* 800 x 480 Native MMI Screen Chassis */}
      <div
        className="relative bg-[#07090e] border-[5px] border-slate-800 rounded-2xl shadow-[0_0_60px_rgba(0,0,0,0.95)] overflow-hidden flex flex-col justify-between"
        style={{
          width: '800px',
          height: '480px',
          fontFamily: themeConfig.fontFamily.includes('Bold') ? 'sans-serif' : 'sans-serif',
        }}
        onClick={() => setInspectModalElement(null)}
      >
        {/* Background Texture Overlay */}
        {themeConfig.activeBackgroundTexture === 'carbon_weave' && (
          <div
            className="absolute inset-0 pointer-events-none opacity-20"
            style={{
              backgroundImage: `repeating-linear-gradient(45deg, #000 0px, #000 2px, #333 2px, #333 4px)`,
            }}
          />
        )}
        {themeConfig.activeBackgroundTexture === 'brushed_aluminum' && (
          <div
            className="absolute inset-0 pointer-events-none opacity-15"
            style={{
              backgroundImage: `linear-gradient(90deg, #1e293b 0%, #334155 50%, #1e293b 100%)`,
            }}
          />
        )}

        {/* ============================================================== */}
        {/* 1. TOP HEADER STRIP */}
        {/* ============================================================== */}
        <div className="relative z-10 flex items-center justify-between px-6 pt-3 pb-1 border-b border-slate-800/60 bg-gradient-to-b from-black/80 to-transparent">
          <div className="flex items-center gap-2">
            <span
              className="text-base font-bold tracking-tight text-white"
              style={{
                fontSize: `${16 * themeConfig.fontSizeScale}px`,
                letterSpacing: `${themeConfig.letterSpacingPx}px`,
              }}
            >
              {activeScreenTab === 'car_setup'
                ? themeConfig.driveSelectView === 'settings'
                  ? 'Audi drive select: Individual'
                  : getStr('CAR_DRIVE_SELECT', 'Audi drive select')
                : activeScreenTab === 'navigation'
                ? getStr('NAV_ROUTE_GUIDANCE', 'AUDI NAVIGATION PLUS')
                : activeScreenTab === 'media'
                ? 'Media · Jukebox'
                : activeScreenTab === 'climate'
                ? 'Climate Control'
                : 'Audi Multimedia Interface'}
            </span>
          </div>

          {/* Screen Quick Mode Badges */}
          <div className="flex items-center gap-2 text-[11px] font-mono">
            {activeScreenTab === 'car_setup' && (
              <span className="px-2 py-0.5 rounded bg-slate-900 border border-slate-700 text-slate-300 font-bold">
                {themeConfig.activeDriveMode.toUpperCase()}
              </span>
            )}
            <span className="text-slate-500">TMC PRO</span>
          </div>
        </div>

        {/* ============================================================== */}
        {/* 2. THE 4 CORNER INTERACTIVE SOFTKEYS (Authentic Red Brackets) */}
        {/* ============================================================== */}

        {/* Top-Left Corner Bracket */}
        <div
          onClick={(e) => {
            e.stopPropagation();
            handleCornerClick('topLeft');
            handleElementClick(e, 'corner_bracket_tl');
          }}
          className={`absolute top-2 left-3 z-30 flex items-center gap-2 cursor-pointer group p-1 rounded transition-all ${
            selectedElementId === 'corner_bracket_tl' ? 'ring-2 ring-amber-400 bg-amber-400/10' : ''
          }`}
          title="Top-Left Softkey (Options / Back)"
        >
          {/* Authentic Curved Red Bracket 「 */}
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="transition-transform group-hover:scale-110">
            <path
              d="M 2 20 L 2 6 Q 2 2 6 2 L 20 2"
              stroke={bracketColor}
              strokeWidth="3.5"
              strokeLinecap="round"
              style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 5px ${bracketColor})` : 'none' }}
            />
          </svg>
          {themeConfig.driveSelectView === 'settings' && (
            <span className="text-xs font-bold text-slate-300 group-hover:text-white transition">
              {themeConfig.language === 'sq' ? 'Kthehu' : 'Back'}
            </span>
          )}
        </div>

        {/* Top-Right Corner Bracket */}
        <div
          onClick={(e) => {
            e.stopPropagation();
            handleCornerClick('topRight');
            handleElementClick(e, 'corner_bracket_tr');
          }}
          className={`absolute top-2 right-3 z-30 flex items-center gap-2 cursor-pointer group p-1 rounded transition-all ${
            selectedElementId === 'corner_bracket_tr' ? 'ring-2 ring-amber-400 bg-amber-400/10' : ''
          }`}
          title="Top-Right Softkey (Contextual Option)"
        >
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="transition-transform group-hover:scale-110">
            <path
              d="M 22 20 L 22 6 Q 22 2 18 2 L 4 2"
              stroke={bracketColor}
              strokeWidth="3.5"
              strokeLinecap="round"
              style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 5px ${bracketColor})` : 'none' }}
            />
          </svg>
        </div>

        {/* Bottom-Left Corner Bracket + Text: "Car systems" (media_1789830294659.png) */}
        <div
          onClick={(e) => {
            e.stopPropagation();
            handleCornerClick('bottomLeft');
            handleElementClick(e, 'corner_bracket_bl');
          }}
          className={`absolute bottom-11 left-3 z-30 flex items-center gap-2 cursor-pointer group p-1.5 rounded transition-all ${
            selectedElementId === 'corner_bracket_bl' ? 'ring-2 ring-amber-400 bg-amber-400/10' : 'hover:bg-slate-900/60'
          }`}
          title="Bottom-Left Softkey (Car systems)"
        >
          {/* Authentic Curved Red Bracket ⌞ */}
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="transition-transform group-hover:scale-110">
            <path
              d="M 2 4 L 2 18 Q 2 22 6 22 L 20 22"
              stroke={bracketColor}
              strokeWidth="3.5"
              strokeLinecap="round"
              style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 5px ${bracketColor})` : 'none' }}
            />
          </svg>
          <span
            className="text-xs font-bold text-white tracking-wide group-hover:text-amber-300 transition"
            style={{
              fontSize: `${13 * themeConfig.fontSizeScale}px`,
              textShadow: '0 1px 3px rgba(0,0,0,0.9)',
            }}
          >
            {themeConfig.language === 'sq'
              ? 'Sistemet e veturës'
              : themeConfig.cornerSoftkeys.bottomLeft.text || 'Car systems'}
          </span>
        </div>

        {/* Bottom-Right Corner Bracket + Text: "Set individual" (media_1789830294659.png) */}
        <div
          onClick={(e) => {
            e.stopPropagation();
            handleCornerClick('bottomRight');
            handleElementClick(e, 'corner_bracket_br');
          }}
          className={`absolute bottom-11 right-3 z-30 flex items-center gap-2 cursor-pointer group p-1.5 rounded transition-all ${
            selectedElementId === 'corner_bracket_br' ? 'ring-2 ring-amber-400 bg-amber-400/10' : 'hover:bg-slate-900/60'
          }`}
          title="Bottom-Right Softkey (Set individual - Click to toggle Platter vs Settings)"
        >
          <span
            className="text-xs font-bold text-white tracking-wide group-hover:text-amber-300 transition"
            style={{
              fontSize: `${13 * themeConfig.fontSizeScale}px`,
              textShadow: '0 1px 3px rgba(0,0,0,0.9)',
            }}
          >
            {themeConfig.driveSelectView === 'settings'
              ? themeConfig.language === 'sq'
                ? 'Pamja 3D'
                : '3D Platter'
              : themeConfig.language === 'sq'
              ? 'Konfiguro Individual'
              : themeConfig.cornerSoftkeys.bottomRight.text || 'Set individual'}
          </span>
          {/* Authentic Curved Red Bracket ⌟ */}
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="transition-transform group-hover:scale-110">
            <path
              d="M 22 4 L 22 18 Q 22 22 18 22 L 4 22"
              stroke={bracketColor}
              strokeWidth="3.5"
              strokeLinecap="round"
              style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 5px ${bracketColor})` : 'none' }}
            />
          </svg>
        </div>

        {/* ============================================================== */}
        {/* 3. CENTRAL SCREEN VIEW STAGE */}
        {/* ============================================================== */}
        <div className="flex-1 relative flex flex-col justify-center items-center overflow-hidden px-8 py-2">

          {/* ------------------------------------------------------------ */}
          {/* VIEW A: AUDI DRIVE SELECT - 3D CAR PLATTER VIEW (media_1789830294659.png) */}
          {/* ------------------------------------------------------------ */}
          {activeScreenTab === 'car_setup' && themeConfig.driveSelectView === 'platter' && (
            <div className="relative w-full h-full flex flex-col items-center justify-between pt-2 pb-4">
              
              {/* 3D Elliptical Selection Platter & Car Asset */}
              <div
                className="relative w-full flex-1 flex items-center justify-center cursor-pointer group"
                onClick={(e) => handleElementClick(e, 'car_drive_select_platter')}
              >
                {/* 3D Perspective Elliptical Platter Podium */}
                <svg className="absolute w-[540px] h-[190px]" viewBox="0 0 540 190">
                  <defs>
                    <radialGradient id="platterGlow" cx="50%" cy="50%" r="50%">
                      <stop offset="0%" stopColor="#1e293b" stopOpacity="0.8" />
                      <stop offset="70%" stopColor="#0f172a" stopOpacity="0.6" />
                      <stop offset="100%" stopColor="#020617" stopOpacity="0.2" />
                    </radialGradient>
                    <filter id="neonRedShadow" x="-20%" y="-20%" width="140%" height="140%">
                      <feDropShadow dx="0" dy="0" stdDeviation="6" floodColor={bracketColor} floodOpacity="0.8" />
                    </filter>
                  </defs>

                  {/* Outer glowing platter ring */}
                  <ellipse
                    cx="270"
                    cy="115"
                    rx="250"
                    ry="62"
                    fill="url(#platterGlow)"
                    stroke={bracketColor}
                    strokeWidth="2.5"
                    filter="url(#neonRedShadow)"
                  />
                  {/* Inner platter plateau */}
                  <ellipse
                    cx="270"
                    cy="115"
                    rx="210"
                    ry="50"
                    fill="#0b0f19"
                    stroke="#334155"
                    strokeWidth="1.5"
                  />
                  {/* Subtle platter radial grid rings */}
                  <ellipse cx="270" cy="115" rx="160" ry="38" fill="none" stroke="#1e293b" strokeWidth="1" strokeDasharray="6,4" />
                </svg>

                {/* 3D Perspective Audi Sedan Vector Model (Isometric Angle) */}
                <div
                  className="relative z-10 w-[380px] h-[140px] flex items-center justify-center cursor-pointer transition-transform group-hover:scale-105"
                  onClick={(e) => handleElementClick(e, 'car_silhouette_sport')}
                  title="3D Audi Model - Click to restyle with Gemini AI"
                >
                  <svg viewBox="0 0 380 140" className="w-full h-full drop-shadow-[0_15px_15px_rgba(0,0,0,0.9)]">
                    {/* Shadow under car */}
                    <ellipse cx="190" cy="120" rx="150" ry="16" fill="#000000" opacity="0.85" />

                    {/* Car Body Silhouette - High Fidelity Audi Sedan */}
                    {/* Main lower body silhouette */}
                    <path
                      d="M 50 96 C 55 94, 65 78, 85 76 C 105 74, 130 75, 160 62 C 190 48, 240 46, 290 56 C 320 62, 340 76, 350 86 C 355 92, 355 98, 345 102 C 330 106, 290 106, 270 106 C 265 96, 255 90, 240 90 C 225 90, 215 96, 210 106 L 140 106 C 135 96, 125 90, 110 90 C 95 90, 85 96, 80 106 C 65 106, 50 102, 50 96 Z"
                      fill="#1a202c"
                      stroke="#4a5568"
                      strokeWidth="1.5"
                    />

                    {/* Aerodynamic Greenhouse / Roofline */}
                    <path
                      d="M 115 74 C 130 58, 165 48, 205 47 C 245 47, 275 56, 305 72 Z"
                      fill="#0f172a"
                      stroke="#64748b"
                      strokeWidth="1.5"
                    />

                    {/* Window Glass Accents */}
                    <path
                      d="M 130 71 C 145 56, 175 51, 205 51 L 205 71 Z"
                      fill="#38bdf8"
                      opacity="0.25"
                    />
                    <path
                      d="M 215 71 L 215 51 C 245 51, 270 58, 290 71 Z"
                      fill="#38bdf8"
                      opacity="0.25"
                    />

                    {/* Audi Characteristic Tornado Shoulder Line */}
                    <path
                      d="M 55 90 Q 200 78 348 88"
                      stroke="#94a3b8"
                      strokeWidth="1.2"
                      fill="none"
                      opacity="0.8"
                    />

                    {/* Front Headlamp (Matrix LED Accent) */}
                    <polygon points="52,91 68,88 64,96" fill="#f8fafc" opacity="0.9" style={{ filter: 'drop-shadow(0 0 4px #ffffff)' }} />

                    {/* Rear Tail Light (OLED Red Accent) */}
                    <path d="M 342 87 Q 352 90 348 97" stroke="#ef4444" strokeWidth="2.5" fill="none" style={{ filter: 'drop-shadow(0 0 5px #ef4444)' }} />

                    {/* Front Wheel & Alloy Rim */}
                    <circle cx="110" cy="106" r="18" fill="#0f172a" stroke="#475569" strokeWidth="3" />
                    <circle cx="110" cy="106" r="13" fill="#1e293b" stroke="#cbd5e1" strokeWidth="1" />
                    <circle cx="110" cy="106" r="4" fill="#64748b" />

                    {/* Rear Wheel & Alloy Rim */}
                    <circle cx="240" cy="106" r="18" fill="#0f172a" stroke="#475569" strokeWidth="3" />
                    <circle cx="240" cy="106" r="13" fill="#1e293b" stroke="#cbd5e1" strokeWidth="1" />
                    <circle cx="240" cy="106" r="4" fill="#64748b" />

                    {/* Audi 4-Rings Emulated on Grille */}
                    <g opacity="0.6" stroke="#e2e8f0" strokeWidth="0.8" fill="none">
                      <circle cx="58" cy="94" r="2.2" />
                      <circle cx="61" cy="94" r="2.2" />
                      <circle cx="64" cy="94" r="2.2" />
                      <circle cx="67" cy="94" r="2.2" />
                    </g>
                  </svg>
                </div>
              </div>

              {/* Mode Selector Pill Carousel (Comfort, Auto, Dynamic, Individual) */}
              {/* Exactly matching media_1789830294659.png */}
              <div className="relative z-20 flex items-center justify-center gap-4 mt-2">
                {[
                  { id: 'comfort', label: getStr('CAR_MODE_COMFORT', 'Comfort') },
                  { id: 'auto', label: 'Auto' },
                  { id: 'dynamic', label: getStr('CAR_MODE_DYNAMIC', 'Dynamic') },
                  { id: 'individual', label: getStr('CAR_MODE_INDIVIDUAL', 'Individual') },
                ].map((mode) => {
                  const isActive = themeConfig.activeDriveMode === mode.id;
                  return (
                    <div key={mode.id} className="relative flex flex-col items-center">
                      {/* Active Downward Arrow Indicator ▼ (matching media_1789830294659.png) */}
                      {isActive && (
                        <div
                          className="text-[10px] font-bold -mb-1 animate-bounce"
                          style={{ color: bracketColor }}
                        >
                          ▼
                        </div>
                      )}

                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          onUpdateTheme({ activeDriveMode: mode.id as any });
                          handleElementClick(e, `mode_pill_${mode.id}`);
                        }}
                        style={{
                          borderColor: isActive ? bracketColor : '#334155',
                          backgroundColor: isActive ? 'rgba(224, 0, 27, 0.25)' : '#0c1017',
                          color: isActive ? '#ffffff' : '#94a3b8',
                          boxShadow: isActive && themeConfig.ambientGlow ? `0 0 14px ${bracketColor}80` : 'none',
                        }}
                        className={`px-5 py-2 rounded-lg border-2 font-bold text-xs tracking-wider transition-all cursor-pointer flex items-center justify-center min-w-[96px] ${
                          isActive
                            ? 'scale-105 font-black text-white shadow-lg'
                            : 'hover:border-slate-500 hover:text-slate-200'
                        }`}
                      >
                        <span>{mode.label}</span>
                      </button>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          {/* ------------------------------------------------------------ */}
          {/* VIEW B: AUDI DRIVE SELECT - SETTINGS SUBMENU (media_1789830294656.png) */}
          {/* ------------------------------------------------------------ */}
          {activeScreenTab === 'car_setup' && themeConfig.driveSelectView === 'settings' && (
            <div
              className="relative w-full h-full flex items-center justify-center cursor-pointer"
              onClick={(e) => handleElementClick(e, 'settings_frame_border')}
            >
              {/* The Iconic Red Rounded Container Box with Left Crescent Arc */}
              <div
                className="relative w-[660px] h-[280px] rounded-2xl border-2 p-6 flex flex-col justify-between backdrop-blur-md"
                style={{
                  borderColor: bracketColor,
                  backgroundColor: 'rgba(10, 14, 22, 0.85)',
                  boxShadow: themeConfig.ambientGlow ? `0 0 25px ${bracketColor}40` : 'none',
                }}
              >
                {/* Left Crescent Arc Graphic */}
                <div className="absolute -left-3 top-1/2 -translate-y-1/2 w-6 h-36 border-l-4 rounded-full pointer-events-none" style={{ borderColor: bracketColor }} />

                {/* Submenu Header */}
                <div className="flex items-center justify-between pb-3 border-b border-slate-800">
                  <span className="text-sm font-bold uppercase tracking-wider text-slate-300">
                    {themeConfig.language === 'sq' ? 'Cilësimet e konfigurimit individual' : 'Individual mode setup'}
                  </span>
                  <span className="text-xs font-mono text-slate-400">MMI Dynamic Chassis</span>
                </div>

                {/* Setting Rows (Engine / gearbox, Steering, Suspension) */}
                <div className="space-y-3 my-auto">
                  {/* Row 1: Engine / gearbox */}
                  <div
                    onClick={(e) => {
                      e.stopPropagation();
                      setActiveSubmenuRow('engine');
                      handleElementClick(e, 'settings_row_highlight');
                    }}
                    className={`flex items-center justify-between p-3 rounded-lg border transition-all cursor-pointer relative ${
                      activeSubmenuRow === 'engine'
                        ? 'bg-[#150a0d] border-red-500/70 shadow-md'
                        : 'bg-[#0b0e17] border-slate-800/80 hover:border-slate-700'
                    }`}
                  >
                    <span className="text-sm font-semibold text-white">
                      {themeConfig.language === 'sq' ? 'Motori / kutia e marsheve' : 'Engine / gearbox'}
                    </span>
                    <div className="relative">
                      <button
                        type="button"
                        onClick={(e) => {
                          e.stopPropagation();
                          setActiveSubmenuRow('engine');
                          setOpenDropdownRow(openDropdownRow === 'engine' ? null : 'engine');
                          handleElementClick(e, 'settings_dropdown_pill');
                        }}
                        style={{
                          borderColor: bracketColor,
                          backgroundColor: '#1b0609',
                          boxShadow: themeConfig.ambientGlow ? `0 0 12px ${bracketColor}70` : 'none',
                        }}
                        className="px-4 py-1.5 rounded-md border text-xs font-bold text-white flex items-center gap-2 hover:scale-105 transition cursor-pointer select-none"
                      >
                        <span className={`text-[10px] transition-transform ${openDropdownRow === 'engine' ? 'rotate-180 text-amber-400 font-bold' : 'text-slate-300'}`}>▼</span>
                        <span>{themeConfig.driveSelectSettings.engineGearbox}</span>
                      </button>

                      {/* Dropdown Popup Menu with 100% Solid Matching Background */}
                      {openDropdownRow === 'engine' && (
                        <div
                          className="absolute top-full mt-1.5 right-0 w-36 bg-[#090d16] border-2 border-red-600 rounded-lg shadow-[0_20px_45px_rgba(0,0,0,0.98)] z-50 overflow-hidden py-1 text-xs"
                          onClick={(e) => e.stopPropagation()}
                        >
                          {(['Comfort', 'Auto', 'Dynamic'] as const).map((opt) => {
                            const isSelected = themeConfig.driveSelectSettings.engineGearbox === opt;
                            return (
                              <button
                                key={opt}
                                type="button"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  onUpdateTheme({
                                    driveSelectSettings: {
                                      ...themeConfig.driveSelectSettings,
                                      engineGearbox: opt,
                                    },
                                  });
                                  setOpenDropdownRow(null);
                                }}
                                className={`w-full px-3 py-2 text-left flex items-center justify-between font-bold transition cursor-pointer ${
                                  isSelected
                                    ? 'bg-red-950 text-red-200 font-extrabold border-l-2 border-red-500'
                                    : 'text-slate-200 hover:bg-slate-800 hover:text-white'
                                }`}
                              >
                                <span>{themeConfig.language === 'sq' ? (opt === 'Comfort' ? 'Komfort' : opt === 'Auto' ? 'Automatik' : 'Dinamik') : opt}</span>
                                {isSelected && <span className="text-red-400 font-mono">✓</span>}
                              </button>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  </div>

                  {/* Row 2: Steering */}
                  <div
                    onClick={(e) => {
                      e.stopPropagation();
                      setActiveSubmenuRow('steering');
                      handleElementClick(e, 'settings_row_highlight');
                    }}
                    className={`flex items-center justify-between p-3 rounded-lg border transition-all cursor-pointer relative ${
                      activeSubmenuRow === 'steering'
                        ? 'bg-[#150a0d] border-red-500/70 shadow-md'
                        : 'bg-[#0b0e17] border-slate-800/80 hover:border-slate-700'
                    }`}
                  >
                    <span className="text-sm font-semibold text-white">
                      {themeConfig.language === 'sq' ? 'Timoni' : 'Steering'}
                    </span>
                    <div className="relative">
                      <button
                        type="button"
                        onClick={(e) => {
                          e.stopPropagation();
                          setActiveSubmenuRow('steering');
                          setOpenDropdownRow(openDropdownRow === 'steering' ? null : 'steering');
                          handleElementClick(e, 'settings_dropdown_pill');
                        }}
                        style={{
                          borderColor: bracketColor,
                          backgroundColor: '#1b0609',
                          boxShadow: themeConfig.ambientGlow ? `0 0 12px ${bracketColor}70` : 'none',
                        }}
                        className="px-4 py-1.5 rounded-md border text-xs font-bold text-white flex items-center gap-2 hover:scale-105 transition cursor-pointer select-none"
                      >
                        <span className={`text-[10px] transition-transform ${openDropdownRow === 'steering' ? 'rotate-180 text-amber-400 font-bold' : 'text-slate-300'}`}>▼</span>
                        <span>{themeConfig.driveSelectSettings.steering}</span>
                      </button>

                      {/* Dropdown Popup Menu with 100% Solid Matching Background */}
                      {openDropdownRow === 'steering' && (
                        <div
                          className="absolute top-full mt-1.5 right-0 w-36 bg-[#090d16] border-2 border-red-600 rounded-lg shadow-[0_20px_45px_rgba(0,0,0,0.98)] z-50 overflow-hidden py-1 text-xs"
                          onClick={(e) => e.stopPropagation()}
                        >
                          {(['Comfort', 'Auto', 'Dynamic'] as const).map((opt) => {
                            const isSelected = themeConfig.driveSelectSettings.steering === opt;
                            return (
                              <button
                                key={opt}
                                type="button"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  onUpdateTheme({
                                    driveSelectSettings: {
                                      ...themeConfig.driveSelectSettings,
                                      steering: opt,
                                    },
                                  });
                                  setOpenDropdownRow(null);
                                }}
                                className={`w-full px-3 py-2 text-left flex items-center justify-between font-bold transition cursor-pointer ${
                                  isSelected
                                    ? 'bg-red-950 text-red-200 font-extrabold border-l-2 border-red-500'
                                    : 'text-slate-200 hover:bg-slate-800 hover:text-white'
                                }`}
                              >
                                <span>{themeConfig.language === 'sq' ? (opt === 'Comfort' ? 'Komfort' : opt === 'Auto' ? 'Automatik' : 'Dinamik') : opt}</span>
                                {isSelected && <span className="text-red-400 font-mono">✓</span>}
                              </button>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  </div>

                  {/* Row 3: Suspension */}
                  <div
                    onClick={(e) => {
                      e.stopPropagation();
                      setActiveSubmenuRow('suspension');
                      handleElementClick(e, 'settings_row_highlight');
                    }}
                    className={`flex items-center justify-between p-3 rounded-lg border transition-all cursor-pointer relative ${
                      activeSubmenuRow === 'suspension'
                        ? 'bg-[#150a0d] border-red-500/70 shadow-md'
                        : 'bg-[#0b0e17] border-slate-800/80 hover:border-slate-700'
                    }`}
                  >
                    <span className="text-sm font-semibold text-white">
                      {themeConfig.language === 'sq' ? 'Amortizimi / Pezullimi' : 'Suspension'}
                    </span>
                    <div className="relative">
                      <button
                        type="button"
                        onClick={(e) => {
                          e.stopPropagation();
                          setActiveSubmenuRow('suspension');
                          setOpenDropdownRow(openDropdownRow === 'suspension' ? null : 'suspension');
                          handleElementClick(e, 'settings_dropdown_pill');
                        }}
                        style={{
                          borderColor: bracketColor,
                          backgroundColor: '#1b0609',
                          boxShadow: themeConfig.ambientGlow ? `0 0 12px ${bracketColor}70` : 'none',
                        }}
                        className="px-4 py-1.5 rounded-md border text-xs font-bold text-white flex items-center gap-2 hover:scale-105 transition cursor-pointer select-none"
                      >
                        <span className={`text-[10px] transition-transform ${openDropdownRow === 'suspension' ? 'rotate-180 text-amber-400 font-bold' : 'text-slate-300'}`}>▼</span>
                        <span>{themeConfig.driveSelectSettings.suspension}</span>
                      </button>

                      {/* Dropdown Popup Menu with 100% Solid Matching Background */}
                      {openDropdownRow === 'suspension' && (
                        <div
                          className="absolute top-full mt-1.5 right-0 w-36 bg-[#090d16] border-2 border-red-600 rounded-lg shadow-[0_20px_45px_rgba(0,0,0,0.98)] z-50 overflow-hidden py-1 text-xs"
                          onClick={(e) => e.stopPropagation()}
                        >
                          {(['Comfort', 'Auto', 'Dynamic'] as const).map((opt) => {
                            const isSelected = themeConfig.driveSelectSettings.suspension === opt;
                            return (
                              <button
                                key={opt}
                                type="button"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  onUpdateTheme({
                                    driveSelectSettings: {
                                      ...themeConfig.driveSelectSettings,
                                      suspension: opt,
                                    },
                                  });
                                  setOpenDropdownRow(null);
                                }}
                                className={`w-full px-3 py-2 text-left flex items-center justify-between font-bold transition cursor-pointer ${
                                  isSelected
                                    ? 'bg-red-950 text-red-200 font-extrabold border-l-2 border-red-500'
                                    : 'text-slate-200 hover:bg-slate-800 hover:text-white'
                                }`}
                              >
                                <span>{themeConfig.language === 'sq' ? (opt === 'Comfort' ? 'Komfort' : opt === 'Auto' ? 'Automatik' : 'Dinamik') : opt}</span>
                                {isSelected && <span className="text-red-400 font-mono">✓</span>}
                              </button>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  </div>
                </div>

                {/* Submenu Bottom Prompt */}
                <div className="flex justify-between items-center text-xs text-slate-400 font-mono pt-2 border-t border-slate-800">
                  <span>Click setting dropdown pill to select Comfort, Auto, or Dynamic</span>
                  <span className="text-amber-400 font-semibold">Active: Individual</span>
                </div>
              </div>
            </div>
          )}

          {/* ------------------------------------------------------------ */}
          {/* VIEW C: NAVIGATION 3D ROUTE */}
          {/* ------------------------------------------------------------ */}
          {activeScreenTab === 'navigation' && (
            <div className="relative w-full h-full flex flex-col justify-between overflow-hidden bg-[#070b12]">
              {/* Mode Toggle Bar */}
              <div className="absolute top-2 right-4 z-20 flex items-center gap-1.5 bg-[#080d17] border border-slate-700/80 rounded-lg p-1 shadow-lg text-[10px] font-mono">
                <button
                  type="button"
                  onClick={() => setNavViewMode('perspective')}
                  className={`px-2 py-0.5 rounded transition cursor-pointer ${
                    navViewMode === 'perspective'
                      ? 'bg-red-950 text-red-300 font-bold border border-red-500/50'
                      : 'text-slate-400 hover:text-white'
                  }`}
                >
                  3D Horizon
                </button>
                <button
                  type="button"
                  onClick={() => setNavViewMode('interactive_vector')}
                  className={`px-2 py-0.5 rounded transition cursor-pointer ${
                    navViewMode === 'interactive_vector'
                      ? 'bg-red-950 text-red-300 font-bold border border-red-500/50'
                      : 'text-slate-400 hover:text-white'
                  }`}
                >
                  2026 Vector Map
                </button>
              </div>

              {/* VIEW 1: 3D Horizon Perspective */}
              {navViewMode === 'perspective' && (
                <div className="absolute inset-0">
                  <div className="absolute inset-0 opacity-40">
                    <svg className="w-full h-full" viewBox="0 0 800 380">
                      <line x1="0" y1="190" x2="800" y2="190" stroke="#1e293b" strokeWidth="1" />
                      <polygon points="0,190 800,190 800,380 0,380" fill="#0f172a" />
                      <polygon points="380,190 420,190 680,380 120,380" fill="#1e293b" />
                      <line x1="400" y1="190" x2="400" y2="380" stroke="#f8fafc" strokeWidth="4" strokeDasharray="16,16" />
                      <path
                        d="M 400 340 L 400 240 L 530 210"
                        fill="none"
                        stroke={bracketColor}
                        strokeWidth="8"
                        strokeLinecap="round"
                        style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 8px ${bracketColor})` : 'none' }}
                      />
                    </svg>
                  </div>
                </div>
              )}

              {/* VIEW 2: 2026 Interactive Vector Cartography */}
              {navViewMode === 'interactive_vector' && (
                <div className="absolute inset-0 overflow-hidden">
                  {/* Vector Map Canvas */}
                  <svg className="w-full h-full" viewBox="0 0 800 380">
                    {/* Topographic Landmass & Subtle Grid */}
                    <rect width="800" height="380" fill="#070a10" />
                    <defs>
                      <pattern id="navGrid" width="40" height="40" patternUnits="userSpaceOnUse">
                        <path d="M 40 0 L 0 0 0 40" fill="none" stroke="#111928" strokeWidth="1" />
                      </pattern>
                      <radialGradient id="vehiclePulse" cx="50%" cy="50%" r="50%">
                        <stop offset="0%" stopColor={bracketColor} stopOpacity="0.8" />
                        <stop offset="100%" stopColor={bracketColor} stopOpacity="0" />
                      </radialGradient>
                    </defs>
                    <rect width="800" height="380" fill="url(#navGrid)" />

                    {/* Secondary Arterial Roads */}
                    <path d="M 50 320 Q 250 280 400 220 T 750 150" fill="none" stroke="#1e293b" strokeWidth="6" strokeLinecap="round" />
                    <path d="M 120 40 Q 280 120 400 220 T 680 340" fill="none" stroke="#1e293b" strokeWidth="6" strokeLinecap="round" />

                    {/* Autostrada A1 Corridor (Primary Highway Dual Carriageway) */}
                    <path d="M 80 360 C 220 300, 320 250, 410 190 C 510 130, 620 90, 760 60" fill="none" stroke="#334155" strokeWidth="14" strokeLinecap="round" />
                    <path d="M 80 360 C 220 300, 320 250, 410 190 C 510 130, 620 90, 760 60" fill="none" stroke="#0f172a" strokeWidth="10" strokeLinecap="round" />

                    {/* Active Navigation Guidance Ribbon */}
                    <path
                      d="M 280 270 C 340 230, 410 190, 520 130 L 680 80"
                      fill="none"
                      stroke={bracketColor}
                      strokeWidth="6"
                      strokeLinecap="round"
                      style={{ filter: themeConfig.ambientGlow ? `drop-shadow(0 0 10px ${bracketColor})` : 'none' }}
                    />

                    {/* Junction Roundabout Node */}
                    <circle cx="410" cy="190" r="14" fill="#0f172a" stroke="#475569" strokeWidth="3" />
                    <circle cx="410" cy="190" r="4" fill="#64748b" />

                    {/* City Labels */}
                    <text x="140" y="320" fill="#94a3b8" fontSize="12" fontFamily="sans-serif" fontWeight="bold">TIRANA</text>
                    <text x="680" y="100" fill="#cbd5e1" fontSize="13" fontFamily="sans-serif" fontWeight="bold">DURRËS</text>
                    <text x="430" y="175" fill="#f59e0b" fontSize="10" fontFamily="sans-serif" fontWeight="600">Jct 4: Rruga e Kombit</text>

                    {/* POI Markers */}
                    {/* Fuel Station POI */}
                    <g transform="translate(480, 140)">
                      <circle cx="0" cy="0" r="10" fill="#1e293b" stroke="#3b82f6" strokeWidth="1.5" />
                      <text x="0" y="3" fill="#60a5fa" fontSize="9" textAnchor="middle" fontWeight="bold">⛽</text>
                      <text x="14" y="3" fill="#94a3b8" fontSize="9" fontFamily="monospace">Shell (350m)</text>
                    </g>

                    {/* Rest Area POI */}
                    <g transform="translate(240, 290)">
                      <circle cx="0" cy="0" r="10" fill="#1e293b" stroke="#10b981" strokeWidth="1.5" />
                      <text x="0" y="3" fill="#34d399" fontSize="9" textAnchor="middle" fontWeight="bold">🅿️</text>
                      <text x="14" y="3" fill="#94a3b8" fontSize="9" fontFamily="monospace">Rest Area (1.2km)</text>
                    </g>

                    {/* Vehicle GPS Position Indicator (Pulse + Chevron) */}
                    <circle cx="280" cy="270" r="22" fill="url(#vehiclePulse)" />
                    <circle cx="280" cy="270" r="8" fill="#ffffff" stroke={bracketColor} strokeWidth="3" />
                    <polygon points="280,260 274,276 280,272 286,276" fill={bracketColor} />
                  </svg>

                  {/* Telemetry HUD Box (Top Right under mode toggle) */}
                  <div className="absolute top-12 right-4 z-10 w-52 bg-[#080d17]/95 border border-slate-700/80 rounded-lg p-2.5 shadow-xl text-[10px] font-mono space-y-1">
                    <div className="flex justify-between items-center text-slate-300 font-bold border-b border-slate-800 pb-1">
                      <span className="flex items-center gap-1"><span className="text-amber-400">🛰️</span> GPS 3D FIX</span>
                      <span className="text-emerald-400 font-bold">9/12 SAT</span>
                    </div>
                    <div className="flex justify-between text-slate-400">
                      <span>Coordinates:</span>
                      <span className="text-slate-200">41.3275°N 19.8187°E</span>
                    </div>
                    <div className="flex justify-between text-slate-400">
                      <span>Elevation:</span>
                      <span className="text-slate-200">114 m AMSL</span>
                    </div>
                    <div className="flex justify-between text-slate-400">
                      <span>Heading / Azimuth:</span>
                      <span className="text-amber-400 font-bold">284° WNW</span>
                    </div>
                    <div className="flex justify-between text-slate-400">
                      <span>Cartography DB:</span>
                      <span className="text-emerald-400">FLDB 2026 (544B)</span>
                    </div>
                  </div>

                  {/* Scale Bar */}
                  <div className="absolute bottom-4 left-6 z-10 flex items-center gap-2 text-[10px] font-mono text-slate-400 bg-black/60 px-2 py-1 rounded border border-slate-800">
                    <div className="w-16 h-1 border-b-2 border-l-2 border-r-2 border-slate-400" />
                    <span>200 m</span>
                  </div>
                </div>
              )}

              {/* Maneuver Banner */}
              <div className="relative z-10 mx-6 mt-4 p-3 bg-[#080c14] border border-slate-800 rounded-lg flex items-center justify-between shadow-2xl backdrop-blur-md max-w-[500px]">
                <div className="flex items-center gap-3">
                  <div
                    onClick={(e) => handleElementClick(e, 'nav_turn_arrow')}
                    className="w-10 h-10 rounded-lg flex items-center justify-center font-bold text-xl cursor-pointer shadow"
                    style={{ backgroundColor: bracketColor, color: '#ffffff' }}
                  >
                    ⮡
                  </div>
                  <div>
                    <div className="text-sm font-bold text-white tracking-tight">
                      {getStr('NAV_NEXT_TURN', 'Turn right in 300 m onto Autostrada A1')}
                    </div>
                    <div className="text-[11px] text-slate-400 font-mono">
                      {themeConfig.language === 'sq' ? 'Drejt: Tirana / Durrës' : 'Toward: Munich / Ingolstadt'}
                    </div>
                  </div>
                </div>

                <div
                  onClick={(e) => handleElementClick(e, 'speed_roundel')}
                  className="w-10 h-10 rounded-full border-4 border-red-600 bg-white flex items-center justify-center text-black font-black text-xs font-mono shadow-lg cursor-pointer ml-3 shrink-0"
                >
                  130
                </div>
              </div>

              {/* 3D Compass (Only shown in perspective or if enabled) */}
              {themeConfig.showCompass && navViewMode === 'perspective' && (
                <div
                  onClick={(e) => handleElementClick(e, 'compass_rose')}
                  className="absolute right-8 top-16 z-10 w-16 h-16 rounded-full bg-black/80 border border-slate-700 flex flex-col items-center justify-center backdrop-blur-sm cursor-pointer shadow-lg"
                >
                  <div className="text-[9px] font-bold text-red-500">N</div>
                  <div className="w-1 h-7 rounded-full bg-red-600" style={{ transform: 'rotate(25deg)' }} />
                  <div className="text-[9px] font-bold text-slate-400">S</div>
                </div>
              )}
            </div>
          )}

          {/* ------------------------------------------------------------ */}
          {/* VIEW D: MEDIA JUKEBOX */}
          {/* ------------------------------------------------------------ */}
          {activeScreenTab === 'media' && (
            <div className="relative w-full h-full flex flex-col justify-between py-6 px-12">
              <div className="flex items-center gap-6 my-auto">
                <div
                  onClick={(e) => handleElementClick(e, 'menu_icon_media')}
                  className="w-28 h-28 rounded-lg bg-slate-800 border border-slate-700 flex items-center justify-center font-bold text-3xl text-slate-400 shadow-xl cursor-pointer"
                >
                  🎵
                </div>
                <div className="space-y-2">
                  <div className="text-xl font-bold text-white tracking-tight">
                    {themeConfig.language === 'sq' ? 'Këngë Tradicionale / Shqip Acoustic' : 'Audi Sound Selection Master'}
                  </div>
                  <div className="text-sm text-slate-300">
                    {themeConfig.language === 'sq' ? 'Artisti: Orkestra Shqiptare' : 'Artist: Master Symphony'}
                  </div>
                  <div className="text-xs text-slate-500 font-mono">
                    Jukebox HDD · FLAC 24-bit 96 kHz · Bang & Olufsen 3D Sound
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* ------------------------------------------------------------ */}
          {/* VIEW E: CLIMATE CONTROL */}
          {/* ------------------------------------------------------------ */}
          {activeScreenTab === 'climate' && (
            <div className="relative w-full h-full flex items-center justify-around">
              <div
                onClick={(e) => handleElementClick(e, 'climate_dial_glow')}
                className="text-center space-y-1 cursor-pointer"
              >
                <span className="text-xs text-slate-400 font-semibold block">Driver</span>
                <div className="text-4xl font-black font-mono text-red-500">21.5°C</div>
                <span className="text-[10px] text-emerald-400 font-mono">AUTO · AC ON</span>
              </div>
              <div className="p-4 bg-slate-900/80 border border-slate-800 rounded-xl flex flex-col items-center gap-2">
                <div className="text-xs text-slate-300 font-bold">SYNC</div>
                <div className="w-24 h-2 bg-slate-800 rounded-full overflow-hidden">
                  <div className="w-3/5 h-full bg-red-500" />
                </div>
                <span className="text-[10px] text-slate-400 font-mono">Blower Speed 3</span>
              </div>
              <div
                onClick={(e) => handleElementClick(e, 'climate_dial_glow')}
                className="text-center space-y-1 cursor-pointer"
              >
                <span className="text-xs text-slate-400 font-semibold block">Passenger</span>
                <div className="text-4xl font-black font-mono text-red-500">21.5°C</div>
                <span className="text-[10px] text-emerald-400 font-mono">AUTO · SYNC</span>
              </div>
            </div>
          )}

          {/* ------------------------------------------------------------ */}
          {/* VIEW F: CAROUSEL */}
          {/* ------------------------------------------------------------ */}
          {activeScreenTab === 'carousel' && (
            <div className="relative w-full h-full flex items-center justify-center">
              <div className="flex items-center gap-4 z-10">
                {[
                  { id: 'menu_icon_nav', label: 'Navigation', icon: '🧭' },
                  { id: 'menu_icon_tel', label: 'Telephone', icon: '📞' },
                  { id: 'menu_icon_car', label: 'Car Setup', icon: '🚗', active: true },
                  { id: 'menu_icon_media', label: 'Media', icon: '🎵' },
                ].map((item) => (
                  <div
                    key={item.id}
                    onClick={(e) => handleElementClick(e, item.id)}
                    className={`flex flex-col items-center p-3 rounded-xl border transition-all cursor-pointer ${
                      item.active
                        ? 'bg-red-500/20 border-red-500 scale-125 shadow-[0_0_20px_rgba(224,0,27,0.4)]'
                        : 'border-slate-800 bg-slate-900/60 opacity-60'
                    }`}
                  >
                    <span className="text-2xl">{item.icon}</span>
                    <span className="text-[11px] font-bold mt-1 text-white">{item.label}</span>
                  </div>
                ))}
              </div>
            </div>
          )}

        </div>

        {/* ============================================================== */}
        {/* 4. AUTHENTIC 32px BOTTOM STATUS BAR */}
        {/* Exactly matching media_1789830294656.png and media_1789830294659.png */}
        {/* ============================================================== */}
        <div className="relative z-20 h-8 px-6 bg-black/95 border-t border-slate-800/80 flex items-center justify-between text-xs font-mono select-none backdrop-blur-sm">
          {/* Left: Speaker Mute Icon */}
          <div
            onClick={(e) => handleElementClick(e, 'status_bar_mute')}
            className={`flex items-center gap-2 cursor-pointer p-0.5 rounded transition ${
              selectedElementId === 'status_bar_mute' ? 'ring-1 ring-amber-400' : ''
            }`}
            title="Audio Mute Indicator"
          >
            {themeConfig.statusBar.isMuted ? (
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                <line x1="23" y1="9" x2="17" y2="15" />
                <line x1="17" y1="9" x2="23" y2="15" />
              </svg>
            ) : (
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#94a3b8" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07" />
              </svg>
            )}
          </div>

          {/* Center: Digital 24h Clock (16:06 / 12:53) */}
          <div
            onClick={(e) => handleElementClick(e, 'status_bar_clock')}
            className={`flex items-center justify-center cursor-pointer px-2 py-0.5 rounded transition font-bold tracking-widest text-slate-100 ${
              selectedElementId === 'status_bar_clock' ? 'ring-1 ring-amber-400' : ''
            }`}
            title="Digital Clock"
          >
            {themeConfig.statusBar.clockTime || '16:06'}
          </div>

          {/* Right: Bluetooth, 4-bar Signal, Google, 3G data traffic */}
          <div className="flex items-center gap-3">
            {/* Bluetooth */}
            {themeConfig.statusBar.bluetoothConnected && (
              <div
                onClick={(e) => handleElementClick(e, 'status_bar_bluetooth')}
                className={`cursor-pointer p-0.5 rounded transition ${
                  selectedElementId === 'status_bar_bluetooth' ? 'ring-1 ring-amber-400' : ''
                }`}
                title="Bluetooth Connected"
              >
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#60a5fa" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                  <polyline points="6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5" />
                </svg>
              </div>
            )}

            {/* 4-Bar Signal Indicator */}
            <div
              onClick={(e) => handleElementClick(e, 'status_bar_signal')}
              className={`flex items-end gap-0.5 h-3.5 cursor-pointer p-0.5 rounded transition ${
                selectedElementId === 'status_bar_signal' ? 'ring-1 ring-amber-400' : ''
              }`}
              title="Cellular Signal Strength"
            >
              {[1, 2, 3, 4].map((bar) => (
                <div
                  key={bar}
                  className={`w-1 rounded-sm ${
                    bar <= themeConfig.statusBar.signalBars ? 'bg-emerald-400' : 'bg-slate-700'
                  }`}
                  style={{ height: `${bar * 3 + 2}px` }}
                />
              ))}
            </div>

            {/* Google Logo (Genuine Audi Connect) */}
            {themeConfig.statusBar.googleServicesOnline && (
              <div
                onClick={(e) => handleElementClick(e, 'status_bar_google')}
                className={`cursor-pointer px-1 rounded transition text-[11px] font-sans font-bold text-slate-200 tracking-tight flex items-center ${
                  selectedElementId === 'status_bar_google' ? 'ring-1 ring-amber-400' : ''
                }`}
                title="Google Connected Services"
              >
                Google
              </div>
            )}

            {/* 3G / LTE with bidirectional data traffic arrows ⇄ */}
            <div
              onClick={(e) => handleElementClick(e, 'status_bar_traffic')}
              className={`flex items-center gap-1 cursor-pointer px-1 rounded transition text-[10px] font-bold text-sky-400 ${
                selectedElementId === 'status_bar_traffic' ? 'ring-1 ring-amber-400' : ''
              }`}
              title="Data Network & Traffic"
            >
              <span>{themeConfig.statusBar.dataNetwork || '3G'}</span>
              <span className="text-[11px] tracking-tighter">⇄</span>
            </div>
          </div>
        </div>

        {/* Selected Element Quick-Edit Overlay Toast */}
        {inspectModalElement && (
          <div className="absolute top-12 left-1/2 -translate-x-1/2 z-40 bg-slate-900/95 border border-amber-500/80 rounded-xl px-4 py-2 shadow-2xl flex items-center gap-3 backdrop-blur-md animate-in fade-in zoom-in-95 duration-150">
            <span className="text-xs font-mono text-amber-400">
              Element: <span className="text-white font-bold">{inspectModalElement}</span>
            </span>
            {onNavigateToAiStudio && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onNavigateToAiStudio(inspectModalElement);
                }}
                className="px-2.5 py-1 bg-gradient-to-r from-amber-500 to-amber-400 text-slate-950 font-bold text-[11px] rounded shadow hover:brightness-110 flex items-center gap-1 transition"
              >
                <span>🍌</span> Restyle with Banana AI
              </button>
            )}
            <button
              onClick={(e) => {
                e.stopPropagation();
                setInspectModalElement(null);
              }}
              className="text-slate-400 hover:text-white text-xs px-1"
            >
              ✕
            </button>
          </div>
        )}
      </div>

      {/* ============================================================== */}
      {/* 5. VIRTUAL PHYSICAL AUDI MMI ROTARY CONSOLE CONTROLLER */}
      {/* Provides tactile automotive console interaction */}
      {/* ============================================================== */}
      <div className="mt-4 p-4 bg-slate-900/80 border border-slate-800 rounded-xl w-[800px] flex items-center justify-between shadow-xl">
        <div className="flex flex-col">
          <span className="text-xs font-bold text-slate-300 flex items-center gap-2">
            <span>🎛️</span>
            <span>MMI Physical Console Controller</span>
          </span>
          <span className="text-[10px] text-slate-500 font-mono mt-0.5">
            Rotary dial + 4 softkeys mapped directly to on-screen corner brackets
          </span>
        </div>

        {/* Hardkey Row (NAV, INFO, CAR, SETUP) */}
        <div className="flex items-center gap-1.5">
          {[
            { id: 'navigation', label: 'NAV' },
            { id: 'media', label: 'MEDIA' },
            { id: 'car_setup', label: 'CAR' },
            { id: 'climate', label: 'SETUP' },
          ].map((btn) => (
            <button
              key={btn.id}
              onClick={() => {
                if (onSelectScreenTab) onSelectScreenTab(btn.id as any);
                if (btn.id === 'car_setup') {
                  onUpdateTheme({ driveSelectView: 'platter' });
                }
              }}
              className={`px-3 py-1.5 rounded text-xs font-mono font-bold border transition ${
                activeScreenTab === btn.id
                  ? 'bg-red-600/30 border-red-500 text-white shadow'
                  : 'bg-slate-950 border-slate-800 text-slate-400 hover:text-slate-200'
              }`}
            >
              {btn.label}
            </button>
          ))}
        </div>

        {/* Rotary Dial with 4 Surrounding Softkey Buttons */}
        <div className="relative w-36 h-28 flex items-center justify-center bg-slate-950/80 border border-slate-800 rounded-xl p-2">
          {/* Top-Left Softkey */}
          <button
            onClick={() => handleCornerClick('topLeft')}
            className="absolute top-1.5 left-2 w-7 h-5 rounded bg-slate-800 hover:bg-red-700 active:scale-95 border border-slate-700 text-[9px] font-bold text-slate-300 transition flex items-center justify-center"
            title="Physical Softkey: Top-Left"
          >
            TL
          </button>

          {/* Top-Right Softkey */}
          <button
            onClick={() => handleCornerClick('topRight')}
            className="absolute top-1.5 right-2 w-7 h-5 rounded bg-slate-800 hover:bg-red-700 active:scale-95 border border-slate-700 text-[9px] font-bold text-slate-300 transition flex items-center justify-center"
            title="Physical Softkey: Top-Right"
          >
            TR
          </button>

          {/* Central Rotary Knob */}
          <div className="relative flex items-center justify-center group">
            {/* Left Rotate Arrow */}
            <button
              onClick={() => handleRotateKnob('left')}
              className="absolute -left-6 w-5 h-8 rounded bg-slate-900 border border-slate-800 hover:border-slate-600 text-slate-400 hover:text-white text-xs flex items-center justify-center transition"
              title="Rotate Left (Previous Mode)"
            >
              ◂
            </button>

            {/* Central Dial Body */}
            <div
              className="w-16 h-16 rounded-full bg-gradient-to-br from-slate-700 via-slate-800 to-slate-950 border-4 border-slate-600 shadow-xl flex items-center justify-center cursor-pointer transition-transform duration-200"
              style={{ transform: `rotate(${knobRotation}deg)` }}
              onClick={() => handleRotateKnob('right')}
              title="MMI Central Rotary Knob - Click to rotate"
            >
              {/* Inner knurled texture ring */}
              <div className="w-11 h-11 rounded-full border border-dashed border-slate-500 flex items-center justify-center">
                {/* Center Joystick / Indicator Nipple */}
                <div className="w-4 h-4 rounded-full bg-slate-950 border-2 border-red-500 shadow-inner" />
              </div>
            </div>

            {/* Right Rotate Arrow */}
            <button
              onClick={() => handleRotateKnob('right')}
              className="absolute -right-6 w-5 h-8 rounded bg-slate-900 border border-slate-800 hover:border-slate-600 text-slate-400 hover:text-white text-xs flex items-center justify-center transition"
              title="Rotate Right (Next Mode)"
            >
              ▸
            </button>
          </div>

          {/* Bottom-Left Softkey (Car systems) */}
          <button
            onClick={() => handleCornerClick('bottomLeft')}
            className="absolute bottom-1.5 left-2 w-7 h-5 rounded bg-slate-800 hover:bg-red-700 active:scale-95 border border-slate-700 text-[9px] font-bold text-slate-300 transition flex items-center justify-center"
            title="Physical Softkey: Bottom-Left (Car systems)"
          >
            BL
          </button>

          {/* Bottom-Right Softkey (Set individual) */}
          <button
            onClick={() => handleCornerClick('bottomRight')}
            className="absolute bottom-1.5 right-2 w-7 h-5 rounded bg-slate-800 hover:bg-red-700 active:scale-95 border border-slate-700 text-[9px] font-bold text-slate-300 transition flex items-center justify-center"
            title="Physical Softkey: Bottom-Right (Set individual)"
          >
            BR
          </button>
        </div>

        {/* Return / Back Hardkey */}
        <div className="flex flex-col gap-1.5">
          <button
            onClick={() => {
              if (themeConfig.driveSelectView === 'settings') {
                onUpdateTheme({ driveSelectView: 'platter' });
              }
            }}
            className="px-3 py-1 bg-slate-950 border border-slate-800 hover:border-slate-600 text-[11px] font-mono font-bold rounded text-slate-400 hover:text-white transition"
          >
            RETURN
          </button>
          <button
            onClick={() => {
              const nextView = themeConfig.driveSelectView === 'platter' ? 'settings' : 'platter';
              onUpdateTheme({ driveSelectView: nextView });
            }}
            className="px-3 py-1 bg-slate-950 border border-slate-800 hover:border-slate-600 text-[11px] font-mono font-bold rounded text-slate-400 hover:text-white transition"
          >
            OPTION
          </button>
        </div>
      </div>
    </div>
  );
};
