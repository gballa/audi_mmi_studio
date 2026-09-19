import React, { useState, useEffect, useRef } from 'react';

export type Tab =
  | 'components'
  | 'ai_elements'
  | 'localization'
  | 'maps'
  | 'build'
  | 'recipes'
  | 'assets'
  | 'relab'
  | 'typography';

interface HeaderProps {
  activeTab: Tab;
  onSelectTab: (tab: Tab) => void;
  onOpenResetModal: () => void;
  notification: string | null;
  onDismissNotification?: () => void;
  stringsCount?: number;
  activeLanguage?: string;
}

export const Header: React.FC<HeaderProps> = ({
  activeTab,
  onSelectTab,
  onOpenResetModal,
  notification,
  onDismissNotification,
  stringsCount = 43,
  activeLanguage = 'sq_AL',
}) => {
  const [isToolsOpen, setIsToolsOpen] = useState<boolean>(false);
  const [isHwInfoOpen, setIsHwInfoOpen] = useState<boolean>(false);
  const [isSecurityInfoOpen, setIsSecurityInfoOpen] = useState<boolean>(false);
  const toolsRef = useRef<HTMLDivElement>(null);
  const hwInfoRef = useRef<HTMLDivElement>(null);
  const secInfoRef = useRef<HTMLDivElement>(null);

  // Close dropdowns when clicking outside
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (toolsRef.current && !toolsRef.current.contains(e.target as Node)) {
        setIsToolsOpen(false);
      }
      if (hwInfoRef.current && !hwInfoRef.current.contains(e.target as Node)) {
        setIsHwInfoOpen(false);
      }
      if (secInfoRef.current && !secInfoRef.current.contains(e.target as Node)) {
        setIsSecurityInfoOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Keyboard shortcut navigation (1..5 and 'r' for reset)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (['INPUT', 'TEXTAREA'].includes((e.target as HTMLElement)?.tagName)) {
        return;
      }
      if (e.key === '1') onSelectTab('components');
      else if (e.key === '2') onSelectTab('ai_elements');
      else if (e.key === '3') onSelectTab('localization');
      else if (e.key === '4') onSelectTab('maps');
      else if (e.key === '5') onSelectTab('build');
      else if (e.key === 'r' || e.key === 'R') onOpenResetModal();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onSelectTab, onOpenResetModal]);

  const secondaryTools = [
    { id: 'relab' as Tab, label: 'Binary RE Lab', icon: '⚡', desc: 'Hex viewer, sliding entropy & container carving' },
    { id: 'assets' as Tab, label: 'Asset Census', icon: '🖼️', desc: 'Precomp, bitmaps, linotype font inspection' },
    { id: 'recipes' as Tab, label: 'Theme Recipes', icon: '📜', desc: 'Declarative JSON recipes & train rebasing' },
    { id: 'typography' as Tab, label: 'Typography Lab', icon: '🔤', desc: 'Font bounding box & text overflow analyzer' },
  ];

  const isSecondaryActive = secondaryTools.some((t) => t.id === activeTab);
  const activeSecondary = secondaryTools.find((t) => t.id === activeTab);

  return (
    <header className="relative flex items-center justify-between px-3 md:px-4 py-2 bg-slate-900/95 backdrop-blur-md border-b border-slate-800 z-30 shadow-md select-none gap-2 w-full min-w-0">
      {/* 1. Left: Audi Emblem, Branding & Train Popover */}
      <div className="flex items-center gap-2 md:gap-3 shrink-0">
        {/* Audi 4-Rings Emblem */}
        <div
          className="flex items-center gap-2 group cursor-pointer"
          onClick={() => setIsHwInfoOpen(!isHwInfoOpen)}
          title="Target Platform & Firmware Specification"
        >
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-slate-800 to-slate-950 border border-slate-700 flex items-center justify-center shadow-inner group-hover:border-amber-500/50 transition shrink-0">
            <svg viewBox="0 0 76 28" className="w-6 h-auto stroke-slate-300 group-hover:stroke-amber-400 transition" fill="none" strokeWidth="2.5">
              <circle cx="14" cy="14" r="12" />
              <circle cx="30" cy="14" r="12" />
              <circle cx="46" cy="14" r="12" />
              <circle cx="62" cy="14" r="12" />
            </svg>
          </div>

          <div>
            <div className="flex items-center gap-1.5 leading-tight">
              <span className="text-xs md:text-sm font-black tracking-tight text-white group-hover:text-amber-400 transition whitespace-nowrap">
                AUDI MMI STUDIO
              </span>
              <span className="px-1.5 py-0.2 text-[9px] bg-amber-500/10 border border-amber-500/40 text-amber-300 font-mono rounded font-bold">
                HN+
              </span>
            </div>
            <div className="hidden sm:flex items-center gap-1.5 text-[10px] text-slate-400 font-mono leading-none mt-0.5">
              <span className="hover:text-slate-200">HN+R_K0942_4</span>
              <span className="text-slate-600">·</span>
              <span className="text-red-400 font-semibold flex items-center gap-0.5 whitespace-nowrap">
                <span>🇦🇱</span> {activeLanguage}
              </span>
            </div>
          </div>
        </div>

        {/* Hardware Info Popover */}
        {isHwInfoOpen && (
          <div
            ref={hwInfoRef}
            className="absolute top-12 left-4 w-80 bg-slate-950 border border-slate-700 rounded-xl p-4 shadow-2xl z-50 text-xs space-y-2.5 animate-in fade-in zoom-in-95 duration-150"
          >
            <div className="flex items-center justify-between pb-2 border-b border-slate-800">
              <span className="font-bold text-white flex items-center gap-1.5">
                <span>🖥️</span> Target Hardware Specification
              </span>
              <button
                onClick={() => setIsHwInfoOpen(false)}
                className="text-slate-400 hover:text-white text-xs px-1"
              >
                ✕
              </button>
            </div>
            <div className="space-y-1.5 text-[11px] font-mono text-slate-300">
              <div className="flex justify-between"><span className="text-slate-500">Platform:</span> <span className="text-amber-400">Audi MMI 3G High / Plus [HN+]</span></div>
              <div className="flex justify-between"><span className="text-slate-500">Target Firmware:</span> <span className="text-white">HN+R_EU_AU_K0942_4</span></div>
              <div className="flex justify-between"><span className="text-slate-500">MainUnit Variant:</span> <span className="text-white">MU9411 (Harman/Becker)</span></div>
              <div className="flex justify-between"><span className="text-slate-500">Operating System:</span> <span className="text-white">QNX Neutrino RTOS 6.5.0</span></div>
              <div className="flex justify-between"><span className="text-slate-500">Display Resolution:</span> <span className="text-white">800 × 480 @ 60Hz 16:9</span></div>
              <div className="flex justify-between"><span className="text-slate-500">Navigation DB:</span> <span className="text-emerald-400">Harman FLDB 544-byte page</span></div>
              <div className="flex justify-between"><span className="text-slate-500">FEC Activation:</span> <span className="text-white">02100028 (2026 ECE Map)</span></div>
              <div className="flex justify-between"><span className="text-slate-500">Language Catalog:</span> <span className="text-amber-400">{stringsCount} Strings (Gjuha Shqipe)</span></div>
            </div>
          </div>
        )}
      </div>

      {/* 2. Center: Primary Engineering Workflows & Tools Dropdown */}
      <div className="flex items-center justify-center flex-1 min-w-0 px-1 overflow-x-auto no-scrollbar">
        <nav className="flex items-center gap-1 bg-slate-950/80 p-1 rounded-xl border border-slate-800/80 shadow-inner shrink-0">
          {/* Primary Tabs */}
          <button
            onClick={() => onSelectTab('components')}
            className={`px-2.5 md:px-3 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center gap-1.5 whitespace-nowrap ${
              activeTab === 'components'
                ? 'bg-amber-500 text-slate-950 font-bold shadow-md shadow-amber-500/20'
                : 'text-slate-400 hover:text-slate-100 hover:bg-slate-900/60'
            }`}
            title="Screen Canvas & Theme Editor [Hotkey: 1]"
          >
            <span>🎨</span>
            <span className="hidden sm:inline">Themes & UI</span>
          </button>

          <button
            onClick={() => onSelectTab('ai_elements')}
            className={`px-2.5 md:px-3 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center gap-1.5 whitespace-nowrap ${
              activeTab === 'ai_elements'
                ? 'bg-amber-500 text-slate-950 font-bold shadow-md shadow-amber-500/20'
                : 'text-amber-400 hover:text-amber-300 hover:bg-slate-900/60 font-semibold'
            }`}
            title="Gemini Nano Banana AI Elements [Hotkey: 2]"
          >
            <span>🍌</span>
            <span className="hidden sm:inline">Gemini AI</span>
          </button>

          <button
            onClick={() => onSelectTab('localization')}
            className={`px-2.5 md:px-3 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center gap-1.5 whitespace-nowrap ${
              activeTab === 'localization'
                ? 'bg-amber-500 text-slate-950 font-bold shadow-md shadow-amber-500/20'
                : 'text-slate-400 hover:text-slate-100 hover:bg-slate-900/60'
            }`}
            title="Albanian Language Localization [Hotkey: 3]"
          >
            <span>🇦🇱</span>
            <span className="hidden sm:inline">Shqip</span>
            <span className="text-[10px] text-amber-400/90 font-mono">({stringsCount})</span>
          </button>

          <button
            onClick={() => onSelectTab('maps')}
            className={`px-2.5 md:px-3 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center gap-1.5 whitespace-nowrap ${
              activeTab === 'maps'
                ? 'bg-amber-500 text-slate-950 font-bold shadow-md shadow-amber-500/20'
                : 'text-slate-400 hover:text-slate-100 hover:bg-slate-900/60'
            }`}
            title="2026 OSM & Google Maps Ingestion [Hotkey: 4]"
          >
            <span>🗺️</span>
            <span className="hidden sm:inline">2026 Maps</span>
          </button>

          <button
            onClick={() => onSelectTab('build')}
            className={`px-2.5 md:px-3.5 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center gap-1.5 whitespace-nowrap ${
              activeTab === 'build'
                ? 'bg-emerald-500 text-slate-950 font-bold shadow-md shadow-emerald-500/20'
                : 'text-emerald-400 hover:text-emerald-300 hover:bg-slate-900/60 font-semibold'
            }`}
            title="Build & SD Card Media Deployment [Hotkey: 5]"
          >
            <span>🚀</span>
            <span className="hidden sm:inline">SD Export</span>
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse ml-0.5" />
          </button>

          {/* Separator */}
          <div className="h-4 w-px bg-slate-800 mx-0.5" />

          {/* Engineering Tools Dropdown */}
          <div className="relative" ref={toolsRef}>
            <button
              onClick={() => setIsToolsOpen(!isToolsOpen)}
              className={`px-2 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center gap-1.5 whitespace-nowrap ${
                isSecondaryActive
                  ? 'bg-slate-800 text-amber-400 font-bold border border-amber-500/50'
                  : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900/60'
              }`}
            >
              <span>{isSecondaryActive && activeSecondary ? activeSecondary.icon : '🔬'}</span>
              <span className="hidden md:inline">{isSecondaryActive && activeSecondary ? activeSecondary.label : 'Tools'}</span>
              <span className={`text-[9px] transition-transform duration-200 ${isToolsOpen ? 'rotate-180' : ''}`}>▾</span>
            </button>

            {isToolsOpen && (
              <div className="absolute top-10 right-0 w-64 bg-slate-950 border border-slate-800 rounded-xl p-2 shadow-2xl z-50 text-xs space-y-1 animate-in fade-in zoom-in-95 duration-150">
                <div className="px-2 py-1 text-[10px] font-bold uppercase tracking-wider text-slate-500">
                  Reverse-Engineering & Diagnostic Tools
                </div>
                {secondaryTools.map((tool) => (
                  <button
                    key={tool.id}
                    onClick={() => {
                      onSelectTab(tool.id);
                      setIsToolsOpen(false);
                    }}
                    className={`w-full text-left p-2 rounded-lg flex items-start gap-2.5 transition ${
                      activeTab === tool.id
                        ? 'bg-amber-500/10 border border-amber-500/40 text-amber-300 font-bold'
                        : 'hover:bg-slate-900 text-slate-300 hover:text-white'
                    }`}
                  >
                    <span className="text-base">{tool.icon}</span>
                    <div>
                      <div className="font-semibold">{tool.label}</div>
                      <div className="text-[10px] text-slate-500">{tool.desc}</div>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>
        </nav>
      </div>

      {/* 3. Right: System Security HUD, Reset Action & Build Trigger */}
      <div className="flex items-center gap-2 shrink-0">
        {/* Security & Integrity Status Pill */}
        <div
          ref={secInfoRef}
          onClick={() => setIsSecurityInfoOpen(!isSecurityInfoOpen)}
          className="flex items-center gap-1.5 px-2 md:px-2.5 py-1.5 rounded-lg border border-slate-800 bg-slate-950/80 hover:border-slate-700 cursor-pointer transition select-none"
          title="Click to view offline security and immutability guarantees"
        >
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse shrink-0" />
          <span className="text-[10px] md:text-[11px] font-mono text-emerald-400 font-semibold tracking-wide whitespace-nowrap">
            AIRGAP
          </span>
          <span className="hidden xl:inline text-slate-700">|</span>
          <span className="hidden xl:inline text-[10px] font-mono text-slate-400 whitespace-nowrap">
            LOCKED
          </span>
        </div>

        {/* Security Popover */}
        {isSecurityInfoOpen && (
          <div className="absolute top-12 right-24 w-80 bg-slate-950 border border-slate-700 rounded-xl p-4 shadow-2xl z-50 text-xs space-y-2.5 animate-in fade-in zoom-in-95 duration-150">
            <div className="flex items-center justify-between pb-2 border-b border-slate-800">
              <span className="font-bold text-white flex items-center gap-1.5">
                <span>🛡️</span> Security & Integrity Guarantees
              </span>
              <button
                onClick={() => setIsSecurityInfoOpen(false)}
                className="text-slate-400 hover:text-white text-xs px-1"
              >
                ✕
              </button>
            </div>
            <div className="space-y-2 text-[11px] text-slate-300">
              <div className="flex items-start gap-2">
                <span className="text-emerald-400 font-bold">✓</span>
                <div>
                  <strong className="text-white">100% Offline Airgap:</strong> No network ports opened, zero external telemetry transmission.
                </div>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-emerald-400 font-bold">✓</span>
                <div>
                  <strong className="text-white">Immutable Originals:</strong> OEM firmware files in <code className="text-amber-400 font-mono">originals/</code> are read-only and cryptographically anchored.
                </div>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-emerald-400 font-bold">✓</span>
                <div>
                  <strong className="text-white">§14.9 Safety Policy Enforced:</strong> All outputs labeled <code className="text-emerald-400 font-mono">BUILD READY — DEPLOYMENT NOT VERIFIED</code>.
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Global Reset Action Button */}
        <button
          onClick={onOpenResetModal}
          className="px-2 md:px-2.5 py-1.5 text-xs font-semibold rounded-lg border border-slate-700/80 bg-slate-800/80 hover:bg-slate-700 text-slate-200 hover:text-white flex items-center gap-1 md:gap-1.5 transition shadow-sm hover:border-amber-500/50 shrink-0"
          title="Open MMI Workstation Reset Manager [Hotkey: R]"
        >
          <span className="text-amber-400 font-bold text-sm leading-none">↺</span>
          <span className="hidden sm:inline">Reset</span>
          <span className="hidden xl:inline text-[10px] font-mono text-slate-500 border border-slate-700 px-1 rounded">R</span>
        </button>
      </div>

      {/* Floating Notification Toast */}
      {notification && (
        <div className="absolute top-14 right-6 max-w-md bg-slate-900/95 border border-amber-500/60 shadow-2xl rounded-xl p-3 flex items-center gap-3 animate-in slide-in-from-top-2 duration-200 z-50">
          <span className="text-base text-amber-400 shrink-0">✨</span>
          <p className="text-xs text-slate-200 font-medium leading-tight flex-1">
            {notification}
          </p>
          {onDismissNotification && (
            <button
              onClick={onDismissNotification}
              className="text-slate-400 hover:text-white text-xs px-1 shrink-0"
            >
              ✕
            </button>
          )}
        </div>
      )}
    </header>
  );
};
