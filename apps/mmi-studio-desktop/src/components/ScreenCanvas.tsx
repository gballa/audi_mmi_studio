import React, { useState, useRef, useEffect } from 'react';
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

  // 3D Interactive Vehicle Chassis State (Phase 2)
  type VehicleChassisId = 'a4_sedan' | 'a4_avant' | 'a5_coupe' | 'a6_allroad' | 'q5_suv' | 'r8_v10';
  const [selectedChassis, setSelectedChassis] = useState<VehicleChassisId>(
    (themeConfig.activeCarSilhouetteStyle as VehicleChassisId) || 'a4_sedan'
  );
  const [carOrbitAngle, setCarOrbitAngle] = useState<number>(0);

  const vehicleChassisList: {
    id: VehicleChassisId;
    name: string;
    badge: string;
    engine: string;
    power: string;
    drivetrain: string;
  }[] = [
    { id: 'a4_sedan', name: 'A4 Sedan', badge: 'B8.5 3.0 TDI', engine: '3.0 V6 TDI', power: '245 HP · 500 Nm', drivetrain: 'quattro Crown-Gear' },
    { id: 'a4_avant', name: 'S4 Avant', badge: 'B8.5 3.0 TFSI', engine: '3.0 V6 Supercharged', power: '333 HP · 440 Nm', drivetrain: 'quattro Sport Diff' },
    { id: 'a5_coupe', name: 'RS5 Coupe', badge: '8T 4.2 FSI', engine: '4.2 V8 High-Rev', power: '450 HP · 430 Nm', drivetrain: 'quattro Torque Vector' },
    { id: 'a6_allroad', name: 'A6 allroad', badge: 'C7 3.0 BiTDI', engine: '3.0 V6 BiTurbo', power: '313 HP · 650 Nm', drivetrain: 'Adaptive Air Suspension' },
    { id: 'q5_suv', name: 'SQ5 TDI', badge: '8R 3.0 BiTDI', engine: '3.0 V6 BiTurbo', power: '313 HP · 650 Nm', drivetrain: 'Permanent Torsen quattro' },
    { id: 'r8_v10', name: 'R8 V10 Plus', badge: 'Type 42 5.2 FSI', engine: '5.2 V10 Plus', power: '550 HP · 540 Nm', drivetrain: 'Mid-Engine AWD' },
  ];

  // Media Jukebox & Bang & Olufsen DSP Engine State
  const tracks = [
    {
      title: themeConfig.language === 'sq' ? 'Këngë Tradicionale / Shqip Acoustic Suite' : 'Audi Sound Selection Master Suite',
      artist: themeConfig.language === 'sq' ? 'Orkestra Shqiptare' : 'Ingolstadt Philharmonic Orchestra',
      album: 'MMI Jukebox Audiophile Edition',
      format: 'FLAC 24-bit 96 kHz',
      durationSec: 215,
      baseFreq: 432,
    },
    {
      title: 'Quattro Induction & Turbo Spool Acoustics',
      artist: 'Audi Sport Acoustic Engineering',
      album: 'V8 4.2 FSI Sound Experience',
      format: 'DTS-HD 5.1 Surround',
      durationSec: 178,
      baseFreq: 220,
    },
    {
      title: 'Bang & Olufsen 3D Sound Calibration Wave',
      artist: 'Acoustic Lens Lab Struer',
      album: '14-Speaker Cabin Alignment',
      format: 'Direct Stream Digital DSD64',
      durationSec: 240,
      baseFreq: 528,
    },
  ];

  const [activeTrackIdx, setActiveTrackIdx] = useState<number>(0);
  const [isPlaying, setIsPlaying] = useState<boolean>(false);
  const [playbackProgress, setPlaybackProgress] = useState<number>(34);
  const [soundFocus, setSoundFocus] = useState<'all' | 'front' | 'rear' | 'driver'>('driver');
  const [surroundLevel, setSurroundLevel] = useState<number>(4);
  const [volume, setVolume] = useState<number>(68);

  const audioCtxRef = useRef<AudioContext | null>(null);
  const osc1Ref = useRef<OscillatorNode | null>(null);
  const osc2Ref = useRef<OscillatorNode | null>(null);
  const gainNodeRef = useRef<GainNode | null>(null);
  const analyserRef = useRef<AnalyserNode | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const animFrameRef = useRef<number | null>(null);

  const stopAudioSynth = () => {
    if (osc1Ref.current) {
      try { osc1Ref.current.stop(); osc1Ref.current.disconnect(); } catch (_) {}
      osc1Ref.current = null;
    }
    if (osc2Ref.current) {
      try { osc2Ref.current.stop(); osc2Ref.current.disconnect(); } catch (_) {}
      osc2Ref.current = null;
    }
  };

  const startAudioSynth = (trackIdx: number) => {
    stopAudioSynth();
    try {
      const AudioCtxClass = window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
      if (!audioCtxRef.current) {
        audioCtxRef.current = new AudioCtxClass();
      }
      const ctx = audioCtxRef.current;
      if (ctx.state === 'suspended') {
        ctx.resume();
      }

      const analyser = ctx.createAnalyser();
      analyser.fftSize = 64;
      analyserRef.current = analyser;

      const masterGain = ctx.createGain();
      masterGain.gain.setValueAtTime((volume / 100) * 0.08, ctx.currentTime);
      gainNodeRef.current = masterGain;

      const baseF = tracks[trackIdx].baseFreq;
      const osc1 = ctx.createOscillator();
      osc1.type = 'sine';
      osc1.frequency.setValueAtTime(baseF, ctx.currentTime);

      const osc2 = ctx.createOscillator();
      osc2.type = 'triangle';
      osc2.frequency.setValueAtTime(baseF * 1.5, ctx.currentTime);

      const filter = ctx.createBiquadFilter();
      filter.type = 'lowpass';
      filter.frequency.setValueAtTime(1200, ctx.currentTime);

      osc1.connect(filter);
      osc2.connect(filter);
      filter.connect(masterGain);
      masterGain.connect(analyser);
      analyser.connect(ctx.destination);

      osc1.start();
      osc2.start();
      osc1Ref.current = osc1;
      osc2Ref.current = osc2;
    } catch (e) {
      console.warn('Web Audio synthesis error:', e);
    }
  };

  const togglePlayback = () => {
    if (isPlaying) {
      stopAudioSynth();
      setIsPlaying(false);
    } else {
      startAudioSynth(activeTrackIdx);
      setIsPlaying(true);
    }
  };

  const changeTrack = (delta: number) => {
    let next = activeTrackIdx + delta;
    if (next < 0) next = tracks.length - 1;
    if (next >= tracks.length) next = 0;
    setActiveTrackIdx(next);
    setPlaybackProgress(0);
    if (isPlaying) {
      startAudioSynth(next);
    }
  };

  useEffect(() => {
    if (gainNodeRef.current && audioCtxRef.current) {
      const vol = isPlaying ? (volume / 100) * 0.08 : 0;
      gainNodeRef.current.gain.setTargetAtTime(vol, audioCtxRef.current.currentTime, 0.05);
    }
  }, [volume, isPlaying]);

  useEffect(() => {
    return () => {
      stopAudioSynth();
      if (audioCtxRef.current) {
        audioCtxRef.current.close().catch(() => {});
      }
      if (animFrameRef.current) {
        cancelAnimationFrame(animFrameRef.current);
      }
    };
  }, []);

  // Visualizer Canvas render loop
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let step = 0;
    const render = () => {
      step++;
      const w = canvas.width;
      const h = canvas.height;
      ctx.clearRect(0, 0, w, h);

      const numBars = 22;
      const barWidth = (w - (numBars - 1) * 3) / numBars;

      let freqData: Uint8Array | null = null;
      if (isPlaying && analyserRef.current) {
        const buf = new ArrayBuffer(analyserRef.current.frequencyBinCount);
        const typedArr = new Uint8Array(buf);
        analyserRef.current.getByteFrequencyData(typedArr);
        freqData = typedArr;
      }

      for (let i = 0; i < numBars; i++) {
        let barHeight = 4;
        if (isPlaying && freqData) {
          const val = freqData[i % freqData.length] || 0;
          barHeight = Math.max(4, (val / 255) * (h - 6));
        } else if (isPlaying) {
          barHeight = Math.max(4, Math.sin(step * 0.15 + i * 0.4) * 14 + 18);
        } else {
          barHeight = 4;
        }

        const x = i * (barWidth + 3);
        const y = h - barHeight;

        const grad = ctx.createLinearGradient(0, y, 0, h);
        grad.addColorStop(0, '#ef4444');
        grad.addColorStop(0.6, '#f97316');
        grad.addColorStop(1, '#e11d48');

        ctx.fillStyle = isPlaying ? grad : '#334155';
        ctx.fillRect(x, y, barWidth, barHeight);
      }

      animFrameRef.current = requestAnimationFrame(render);
    };

    render();

    return () => {
      if (animFrameRef.current) {
        cancelAnimationFrame(animFrameRef.current);
      }
    };
  }, [isPlaying]);

  // Progress simulation ticker
  useEffect(() => {
    if (!isPlaying) return;
    const timer = setInterval(() => {
      setPlaybackProgress((prev) => (prev >= 100 ? 0 : prev + 0.5));
    }, 1000);
    return () => clearInterval(timer);
  }, [isPlaying]);

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

                {/* Chassis Model Selector Quick Switcher Bar */}
                <div className="absolute top-2 z-20 flex items-center justify-between w-[92%] px-3 py-1 rounded-lg bg-black/75 border border-slate-800/90 backdrop-blur-sm text-[10px] font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-amber-400 font-bold">CHASSIS:</span>
                    <div className="flex items-center gap-1">
                      {vehicleChassisList.map((chassis) => (
                        <button
                          key={chassis.id}
                          onClick={(e) => {
                            e.stopPropagation();
                            setSelectedChassis(chassis.id);
                            onUpdateTheme({ activeCarSilhouetteStyle: chassis.id });
                          }}
                          className={`px-2 py-0.5 rounded transition cursor-pointer ${
                            selectedChassis === chassis.id
                              ? 'bg-red-600 text-white font-bold shadow-[0_0_8px_rgba(239,68,68,0.7)]'
                              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
                          }`}
                          title={`${chassis.name} - ${chassis.badge}`}
                        >
                          {chassis.name}
                        </button>
                      ))}
                    </div>
                  </div>

                  {/* 3D Orbit Angle Controls */}
                  <div className="flex items-center gap-2 text-slate-400">
                    <span className="text-[9px] text-slate-500">ORBIT:</span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        setCarOrbitAngle((prev) => Math.max(-20, prev - 10));
                      }}
                      className="w-5 h-5 rounded bg-slate-900 border border-slate-700 hover:text-white flex items-center justify-center transition"
                      title="Orbit Left"
                    >
                      ⟲
                    </button>
                    <span className="text-slate-300 font-mono w-6 text-center">{carOrbitAngle}°</span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        setCarOrbitAngle((prev) => Math.min(20, prev + 10));
                      }}
                      className="w-5 h-5 rounded bg-slate-900 border border-slate-700 hover:text-white flex items-center justify-center transition"
                      title="Orbit Right"
                    >
                      ⟳
                    </button>
                  </div>
                </div>

                {/* 3D Perspective Vehicle Chassis Model (Isometric Angle with Orbit & Suspension) */}
                {(() => {
                  const isDynamic =
                    themeConfig.activeDriveMode === 'dynamic' ||
                    (themeConfig.activeDriveMode === 'individual' &&
                      themeConfig.driveSelectSettings.suspension === 'Dynamic');
                  const isComfort =
                    themeConfig.activeDriveMode === 'comfort' ||
                    (themeConfig.activeDriveMode === 'individual' &&
                      themeConfig.driveSelectSettings.suspension === 'Comfort');
                  const suspensionDropPx = isDynamic ? 8 : isComfort ? -4 : 0;
                  const currentSpec =
                    vehicleChassisList.find((c) => c.id === selectedChassis) || vehicleChassisList[0];

                  return (
                    <div
                      className="relative z-10 w-[420px] h-[155px] flex items-center justify-center cursor-pointer transition-transform duration-300 group-hover:scale-105"
                      style={{
                        perspective: '600px',
                        transform: `rotateY(${carOrbitAngle}deg)`,
                      }}
                      onClick={(e) => handleElementClick(e, `car_chassis_${selectedChassis}`)}
                      title={`${currentSpec.name} (${currentSpec.badge}) - Click to inspect`}
                    >
                      <svg viewBox="0 0 420 155" className="w-full h-full drop-shadow-[0_20px_20px_rgba(0,0,0,0.95)]">
                        {/* Shadow under car on the platter */}
                        <ellipse
                          cx="210"
                          cy="134"
                          rx="175"
                          ry="18"
                          fill="#000000"
                          opacity="0.9"
                        />
                        {/* Dynamic Neon Underbody Ground Glow */}
                        {isDynamic && (
                          <ellipse
                            cx="210"
                            cy="133"
                            rx="140"
                            ry="12"
                            fill="#ef4444"
                            opacity="0.25"
                            style={{ filter: 'blur(6px)' }}
                          />
                        )}

                        {/* =================================================== */}
                        {/* MODEL SPECIFIC BODY CONTOURS */}
                        {/* =================================================== */}
                        <g style={{ transform: `translateY(${suspensionDropPx}px)`, transition: 'transform 0.4s ease-out' }}>
                          {/* 1. AUDI A4 SEDAN (B8.5 Notchback) */}
                          {selectedChassis === 'a4_sedan' && (
                            <>
                              {/* Main lower body */}
                              <path
                                d="M 60 106 C 65 104, 78 88, 98 86 C 118 84, 145 85, 175 72 C 205 58, 258 56, 310 66 C 342 72, 365 86, 375 96 C 380 102, 380 108, 370 112 C 352 116, 312 116, 290 116 C 285 106, 274 100, 258 100 C 242 100, 231 106, 226 116 L 152 116 C 147 106, 136 100, 120 100 C 104 100, 93 106, 88 116 C 73 116, 60 112, 60 106 Z"
                                fill="#1e293b"
                                stroke="#475569"
                                strokeWidth="1.6"
                              />
                              {/* Greenhouse / Roofline */}
                              <path
                                d="M 128 84 C 144 67, 180 57, 222 56 C 265 56, 296 66, 328 82 Z"
                                fill="#0f172a"
                                stroke="#64748b"
                                strokeWidth="1.5"
                              />
                              {/* Windows */}
                              <path d="M 143 81 C 158 66, 190 60, 222 60 L 222 81 Z" fill="#38bdf8" opacity="0.25" />
                              <path d="M 232 81 L 232 60 C 264 60, 290 68, 312 81 Z" fill="#38bdf8" opacity="0.25" />
                              {/* Tornado line */}
                              <path d="M 65 100 Q 215 88 372 98" stroke="#94a3b8" strokeWidth="1.2" fill="none" opacity="0.85" />
                              {/* Tail bootlid lip */}
                              <path d="M 368 94 Q 376 96 374 101" stroke="#cbd5e1" strokeWidth="1.2" fill="none" />
                            </>
                          )}

                          {/* 2. AUDI S4 AVANT (B8.5 Wagon) */}
                          {selectedChassis === 'a4_avant' && (
                            <>
                              {/* Extended Wagon Body & Roofline */}
                              <path
                                d="M 60 106 C 65 104, 78 88, 98 86 C 118 84, 145 85, 175 72 C 205 58, 260 56, 335 58 C 362 60, 375 76, 378 96 C 380 104, 378 110, 368 112 C 352 116, 312 116, 290 116 C 285 106, 274 100, 258 100 C 242 100, 231 106, 226 116 L 152 116 C 147 106, 136 100, 120 100 C 104 100, 93 106, 88 116 C 73 116, 60 112, 60 106 Z"
                                fill="#1e293b"
                                stroke="#475569"
                                strokeWidth="1.6"
                              />
                              {/* Extended Roofline with Roof Rail */}
                              <path
                                d="M 128 84 C 144 67, 180 57, 222 56 C 275 56, 335 58, 364 68 L 368 84 Z"
                                fill="#0f172a"
                                stroke="#64748b"
                                strokeWidth="1.5"
                              />
                              {/* Silver Roof Rails */}
                              <path d="M 160 54 L 348 56" stroke="#e2e8f0" strokeWidth="2" strokeLinecap="round" opacity="0.9" />
                              {/* 3rd Wagon Rear Quarter Window */}
                              <path d="M 318 80 L 318 64 L 354 70 L 350 80 Z" fill="#38bdf8" opacity="0.25" />
                            </>
                          )}

                          {/* 3. AUDI RS5 COUPE (8T3) */}
                          {selectedChassis === 'a5_coupe' && (
                            <>
                              {/* Sweeping Fastback Coupe Silhouette */}
                              <path
                                d="M 55 108 C 60 106, 75 86, 96 84 C 118 82, 142 82, 172 70 C 202 56, 250 52, 308 62 C 346 68, 372 82, 382 94 C 386 100, 384 106, 374 110 C 354 116, 312 116, 290 116 C 285 106, 274 100, 258 100 C 242 100, 231 106, 226 116 L 152 116 C 147 106, 136 100, 120 100 C 104 100, 93 106, 88 116 C 73 116, 55 114, 55 108 Z"
                                fill="#1a1c23"
                                stroke="#ef4444"
                                strokeWidth="1.8"
                              />
                              {/* Raked Coupe Roofline */}
                              <path
                                d="M 122 82 C 140 64, 180 53, 226 53 C 272 53, 314 62, 355 84 Z"
                                fill="#090d16"
                                stroke="#94a3b8"
                                strokeWidth="1.5"
                              />
                              {/* Wide RS Haunches Flare */}
                              <path d="M 88 96 Q 120 90 152 96" stroke="#ef4444" strokeWidth="2" fill="none" opacity="0.9" />
                              <path d="M 226 96 Q 258 90 290 96" stroke="#ef4444" strokeWidth="2" fill="none" opacity="0.9" />
                            </>
                          )}

                          {/* 4. AUDI A6 ALLROAD (C7) */}
                          {selectedChassis === 'a6_allroad' && (
                            <>
                              {/* High-Stance Rugged Wagon */}
                              <path
                                d="M 55 102 C 60 100, 75 84, 96 82 C 118 80, 145 80, 176 68 C 208 54, 265 52, 342 54 C 370 56, 384 72, 388 92 C 390 100, 386 108, 376 110 C 354 114, 312 114, 290 114 C 285 104, 274 96, 258 96 C 242 96, 231 104, 226 114 L 152 114 C 147 104, 136 96, 120 96 C 104 96, 93 104, 88 114 C 73 114, 55 110, 55 102 Z"
                                fill="#27272a"
                                stroke="#71717a"
                                strokeWidth="1.6"
                              />
                              {/* Rugged Cladding Wheel Arches */}
                              <path d="M 82 114 C 84 98, 126 98, 128 114" stroke="#52525b" strokeWidth="4" fill="none" />
                              <path d="M 220 114 C 222 98, 264 98, 266 114" stroke="#52525b" strokeWidth="4" fill="none" />
                              {/* Stainless Steel Skid Plate Look */}
                              <line x1="58" y1="108" x2="84" y2="108" stroke="#d4d4d8" strokeWidth="2.5" />
                              <line x1="360" y1="108" x2="384" y2="108" stroke="#d4d4d8" strokeWidth="2.5" />
                            </>
                          )}

                          {/* 5. AUDI Q5 SUV (8R) */}
                          {selectedChassis === 'q5_suv' && (
                            <>
                              {/* High Riding SUV Profile */}
                              <path
                                d="M 52 98 C 56 94, 72 76, 94 74 C 116 72, 142 74, 172 60 C 202 46, 260 44, 330 48 C 362 52, 380 70, 384 92 C 386 102, 382 108, 372 112 C 354 116, 312 116, 290 116 C 285 104, 274 96, 258 96 C 242 96, 231 104, 226 116 L 152 116 C 147 104, 136 96, 120 96 C 104 96, 93 104, 88 116 C 73 116, 52 108, 52 98 Z"
                                fill="#1e293b"
                                stroke="#64748b"
                                strokeWidth="1.8"
                              />
                              {/* Tall SUV Greenhouse */}
                              <path
                                d="M 124 72 C 140 54, 178 44, 222 43 C 270 43, 330 46, 362 62 L 366 84 Z"
                                fill="#0f172a"
                                stroke="#94a3b8"
                                strokeWidth="1.6"
                              />
                              {/* Roof Rails */}
                              <path d="M 160 41 L 340 44" stroke="#cbd5e1" strokeWidth="2.5" strokeLinecap="round" />
                            </>
                          )}

                          {/* 6. AUDI R8 V10 PLUS (Type 42 Supercar) */}
                          {selectedChassis === 'r8_v10' && (
                            <>
                              {/* Low Slung Mid-Engine Supercar Wedge */}
                              <path
                                d="M 45 112 C 50 110, 70 94, 94 92 C 118 90, 140 90, 168 80 C 196 68, 235 66, 275 74 C 315 82, 350 94, 385 104 C 390 108, 386 114, 376 116 C 354 118, 312 118, 290 118 C 285 106, 274 100, 258 100 C 242 100, 231 106, 226 118 L 152 118 C 147 106, 136 100, 120 100 C 104 100, 93 106, 88 118 C 73 118, 45 116, 45 112 Z"
                                fill="#090d16"
                                stroke="#ef4444"
                                strokeWidth="2"
                              />
                              {/* Low Cab-Forward Supercar Canopy */}
                              <path
                                d="M 118 90 C 135 74, 170 66, 210 66 C 245 66, 270 74, 290 88 Z"
                                fill="#020617"
                                stroke="#e2e8f0"
                                strokeWidth="1.5"
                              />
                              {/* Iconic Carbon Fiber Sideblade */}
                              <path
                                d="M 235 72 L 255 74 L 248 114 L 232 112 Z"
                                fill="#18181b"
                                stroke="#ef4444"
                                strokeWidth="1.2"
                              />
                              {/* Mid-Engine Glass Cover with V10 Runners */}
                              <path d="M 258 76 L 310 85 L 305 92 L 252 83 Z" fill="#38bdf8" opacity="0.35" stroke="#64748b" strokeWidth="0.8" />
                              {/* Rear Aero Diffuser Strakes */}
                              <line x1="370" y1="114" x2="384" y2="114" stroke="#ef4444" strokeWidth="2.5" />
                            </>
                          )}

                          {/* Front Headlamp (Matrix LED DRL Beam) */}
                          <polygon
                            points="52,94 70,91 66,99"
                            fill="#ffffff"
                            opacity="0.95"
                            style={{ filter: isDynamic ? 'drop-shadow(0 0 8px #ffffff)' : 'drop-shadow(0 0 3px #ffffff)' }}
                          />
                          {/* Forward Road Projection Beam in Dynamic */}
                          {isDynamic && (
                            <polygon
                              points="52,94 15,125 75,128 66,99"
                              fill="#ffffff"
                              opacity="0.12"
                              style={{ filter: 'blur(3px)' }}
                            />
                          )}

                          {/* Rear Tail Light (OLED Red Bar) */}
                          <path
                            d="M 368 90 Q 378 93 374 100"
                            stroke="#ef4444"
                            strokeWidth="3"
                            fill="none"
                            style={{ filter: 'drop-shadow(0 0 6px #ef4444)' }}
                          />

                          {/* Audi 4-Rings Emulated on Grille */}
                          <g opacity="0.75" stroke="#f1f5f9" strokeWidth="0.9" fill="none">
                            <circle cx="62" cy="98" r="2.4" />
                            <circle cx="65.5" cy="98" r="2.4" />
                            <circle cx="69" cy="98" r="2.4" />
                            <circle cx="72.5" cy="98" r="2.4" />
                          </g>
                        </g>

                        {/* Wheels (Static Ground Contact Position) */}
                        {/* Front Wheel & Brembo Red Brake Caliper */}
                        <g>
                          <circle cx="120" cy="116" r="19" fill="#090d16" stroke="#475569" strokeWidth="3.5" />
                          {/* Glowing Red Caliper in Dynamic Mode */}
                          <path
                            d="M 112 104 A 14 14 0 0 1 126 104"
                            stroke={isDynamic ? '#ef4444' : '#64748b'}
                            strokeWidth="4"
                            fill="none"
                            style={{ filter: isDynamic ? 'drop-shadow(0 0 5px #ef4444)' : 'none' }}
                          />
                          <circle cx="120" cy="116" r="14" fill="#1e293b" stroke="#94a3b8" strokeWidth="1.2" />
                          {/* 5-Arm Rotor Alloy Spokes */}
                          <line x1="120" y1="116" x2="120" y2="104" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="120" y1="116" x2="131" y2="112" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="120" y1="116" x2="127" y2="125" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="120" y1="116" x2="113" y2="125" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="120" y1="116" x2="109" y2="112" stroke="#cbd5e1" strokeWidth="1.5" />
                          <circle cx="120" cy="116" r="4" fill="#475569" />
                        </g>

                        {/* Rear Wheel & Brake Caliper */}
                        <g>
                          <circle cx="258" cy="116" r="19" fill="#090d16" stroke="#475569" strokeWidth="3.5" />
                          <path
                            d="M 250 104 A 14 14 0 0 1 264 104"
                            stroke={isDynamic ? '#ef4444' : '#64748b'}
                            strokeWidth="4"
                            fill="none"
                            style={{ filter: isDynamic ? 'drop-shadow(0 0 5px #ef4444)' : 'none' }}
                          />
                          <circle cx="258" cy="116" r="14" fill="#1e293b" stroke="#94a3b8" strokeWidth="1.2" />
                          <line x1="258" y1="116" x2="258" y2="104" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="258" y1="116" x2="269" y2="112" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="258" y1="116" x2="265" y2="125" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="258" y1="116" x2="251" y2="125" stroke="#cbd5e1" strokeWidth="1.5" />
                          <line x1="258" y1="116" x2="247" y2="112" stroke="#cbd5e1" strokeWidth="1.5" />
                          <circle cx="258" cy="116" r="4" fill="#475569" />
                        </g>
                      </svg>

                      {/* Active Model Specification Pill Badge */}
                      <div className="absolute bottom-0 right-4 px-2 py-0.5 rounded bg-black/85 border border-slate-700/80 text-[9px] font-mono text-slate-300 flex items-center gap-1.5 shadow-md">
                        <span className="text-amber-400 font-bold">{currentSpec.badge}</span>
                        <span className="text-slate-600">|</span>
                        <span className="text-slate-400">{currentSpec.power}</span>
                        <span className="text-slate-600">|</span>
                        <span className={isDynamic ? 'text-red-400 font-bold' : isComfort ? 'text-cyan-400' : 'text-emerald-400'}>
                          {isDynamic ? '▼ -20mm RIDE' : isComfort ? '▲ +15mm AIR' : 'AUTO RIDE'}
                        </span>
                      </div>
                    </div>
                  );
                })()}
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
            <div className="relative w-full h-full flex flex-col justify-between p-4 bg-gradient-to-b from-[#090d14] via-[#05070a] to-[#030407]">
              {/* Media Sub-navigation Source Bar */}
              <div className="flex items-center justify-between px-3 py-1.5 bg-black/60 rounded-lg border border-slate-800/80 mb-2">
                <div className="flex items-center gap-2">
                  <button className="px-2.5 py-0.5 rounded bg-red-600/30 border border-red-500/60 text-[11px] font-bold text-red-300 flex items-center gap-1.5 shadow-sm">
                    <span className="w-1.5 h-1.5 rounded-full bg-red-400 animate-pulse" />
                    JUKEBOX HDD
                  </button>
                  <button className="px-2 py-0.5 rounded bg-slate-900/60 border border-slate-800 text-[11px] font-semibold text-slate-400 hover:text-slate-200 transition">
                    SD CARD 1
                  </button>
                  <button className="px-2 py-0.5 rounded bg-slate-900/60 border border-slate-800 text-[11px] font-semibold text-slate-400 hover:text-slate-200 transition">
                    SD CARD 2
                  </button>
                  <button className="px-2 py-0.5 rounded bg-slate-900/60 border border-slate-800 text-[11px] font-semibold text-slate-400 hover:text-slate-200 transition">
                    AMI / BT-AUDIO
                  </button>
                </div>
                <div className="flex items-center gap-2 text-[10px] font-mono text-slate-400">
                  <span className="text-amber-400 font-bold">BANG & OLUFSEN</span>
                  <span className="text-slate-600">|</span>
                  <span className="text-emerald-400">DSP ONLINE</span>
                </div>
              </div>

              {/* Main Two-Column Media & DSP Layout */}
              <div className="flex-1 grid grid-cols-12 gap-4 items-stretch min-h-0">
                {/* Left Column (6/12): Jukebox Player & Track Metadata */}
                <div className="col-span-7 flex flex-col justify-between bg-slate-950/70 border border-slate-800/80 rounded-xl p-3.5 shadow-lg">
                  <div className="flex items-start gap-4">
                    {/* Vinyl / CD Artwork Box with B&O Badge */}
                    <div
                      onClick={(e) => handleElementClick(e, 'media_cover_art')}
                      className={`relative w-24 h-24 rounded-lg bg-gradient-to-br from-slate-800 via-slate-900 to-black border-2 ${
                        isPlaying ? 'border-red-500 shadow-[0_0_20px_rgba(239,68,68,0.3)]' : 'border-slate-700'
                      } flex flex-col items-center justify-center cursor-pointer overflow-hidden transition-all duration-300 flex-shrink-0`}
                      title="Click to inspect cover art"
                    >
                      <div className={`text-3xl transition-transform duration-700 ${isPlaying ? 'rotate-12 scale-110' : ''}`}>
                        🎵
                      </div>
                      <div className="absolute bottom-1 px-1.5 py-0.5 rounded bg-black/80 text-[8px] font-mono font-bold text-amber-300 border border-amber-500/30">
                        B&O 3D
                      </div>
                      {/* Spinning groove ring simulation */}
                      {isPlaying && (
                        <div className="absolute inset-1 rounded-full border border-dashed border-red-400/40 animate-[spin_8s_linear_infinite] pointer-events-none" />
                      )}
                    </div>

                    {/* Track Title, Artist, Album, Bitrate */}
                    <div className="flex-1 min-w-0 space-y-1">
                      <div className="flex items-center gap-2">
                        <span className="px-1.5 py-0.2 rounded bg-red-950/80 border border-red-800/60 text-[9px] font-mono font-bold text-red-300">
                          TRACK {activeTrackIdx + 1}/{tracks.length}
                        </span>
                        <span className="text-[10px] font-mono text-slate-400 truncate">
                          {tracks[activeTrackIdx].format}
                        </span>
                      </div>
                      <div className="text-sm font-bold text-white truncate tracking-tight">
                        {tracks[activeTrackIdx].title}
                      </div>
                      <div className="text-xs text-slate-300 truncate">
                        {tracks[activeTrackIdx].artist}
                      </div>
                      <div className="text-[10px] text-slate-500 font-mono truncate">
                        {tracks[activeTrackIdx].album}
                      </div>
                    </div>
                  </div>

                  {/* Scrubber & Progress Bar */}
                  <div className="space-y-1.5 my-auto">
                    <div className="w-full bg-slate-900 h-2 rounded-full overflow-hidden border border-slate-800 relative cursor-pointer"
                      onClick={(e) => {
                        const rect = e.currentTarget.getBoundingClientRect();
                        const pct = Math.max(0, Math.min(100, ((e.clientX - rect.left) / rect.width) * 100));
                        setPlaybackProgress(pct);
                      }}
                    >
                      <div
                        className="h-full bg-gradient-to-r from-red-600 via-red-500 to-amber-500 transition-all duration-300"
                        style={{ width: `${playbackProgress}%` }}
                      />
                    </div>
                    <div className="flex justify-between text-[10px] font-mono text-slate-400">
                      <span>
                        {String(Math.floor((tracks[activeTrackIdx].durationSec * (playbackProgress / 100)) / 60)).padStart(2, '0')}:
                        {String(Math.floor((tracks[activeTrackIdx].durationSec * (playbackProgress / 100)) % 60)).padStart(2, '0')}
                      </span>
                      <span>
                        {String(Math.floor(tracks[activeTrackIdx].durationSec / 60)).padStart(2, '0')}:
                        {String(tracks[activeTrackIdx].durationSec % 60).padStart(2, '0')}
                      </span>
                    </div>
                  </div>

                  {/* Transport Controls & Volume */}
                  <div className="flex items-center justify-between pt-2 border-t border-slate-800/80">
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => changeTrack(-1)}
                        className="w-8 h-8 rounded-lg bg-slate-900 hover:bg-slate-800 border border-slate-700 text-slate-300 hover:text-white flex items-center justify-center transition active:scale-95 text-xs"
                        title="Previous Track"
                      >
                        ⏮
                      </button>
                      <button
                        onClick={togglePlayback}
                        className={`px-4 h-8 rounded-lg font-bold text-xs flex items-center gap-1.5 transition active:scale-95 shadow-md ${
                          isPlaying
                            ? 'bg-red-600 hover:bg-red-500 text-white shadow-red-600/40'
                            : 'bg-gradient-to-r from-red-700 to-red-600 hover:from-red-600 hover:to-red-500 text-white'
                        }`}
                        title="Toggle Playback (Web Audio API Synth)"
                      >
                        <span>{isPlaying ? '⏸ PAUSE' : '▶ PLAY'}</span>
                      </button>
                      <button
                        onClick={() => changeTrack(1)}
                        className="w-8 h-8 rounded-lg bg-slate-900 hover:bg-slate-800 border border-slate-700 text-slate-300 hover:text-white flex items-center justify-center transition active:scale-95 text-xs"
                        title="Next Track"
                      >
                        ⏭
                      </button>
                    </div>

                    {/* Volume Slider */}
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-slate-400">🔊</span>
                      <input
                        type="range"
                        min="0"
                        max="100"
                        value={volume}
                        onChange={(e) => setVolume(Number(e.target.value))}
                        className="w-20 accent-red-500 h-1 bg-slate-800 rounded cursor-pointer"
                        title={`Volume: ${volume}%`}
                      />
                      <span className="text-[10px] font-mono text-slate-300 w-6 text-right">
                        {volume}
                      </span>
                    </div>
                  </div>
                </div>

                {/* Right Column (5/12): Bang & Olufsen 3D Sound Stage & Spectrum Visualizer */}
                <div className="col-span-5 flex flex-col justify-between bg-slate-950/70 border border-slate-800/80 rounded-xl p-3 shadow-lg">
                  {/* B&O Sound Stage Cockpit Topology */}
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <span className="text-[11px] font-bold text-slate-200 tracking-wide">
                        3D SOUND FOCUS
                      </span>
                      <span className="text-[9px] font-mono text-amber-400 font-semibold px-1.5 py-0.5 rounded bg-amber-950/40 border border-amber-800/40">
                        14 SPEAKERS · 505W
                      </span>
                    </div>

                    {/* Visual Car Cabin Diagram */}
                    <div className="relative h-28 bg-[#070b12] rounded-lg border border-slate-800/90 overflow-hidden flex items-center justify-center">
                      {/* Car Body Outline */}
                      <div className="relative w-36 h-24 border border-slate-700/60 rounded-[28px] bg-slate-900/40 flex flex-col justify-between p-2">
                        {/* Windshield acoustic lens tweeters */}
                        <div className="flex justify-between items-center px-2">
                          <div
                            className={`w-2 h-2 rounded-full transition-all duration-300 ${
                              soundFocus === 'front' || soundFocus === 'all' || soundFocus === 'driver'
                                ? 'bg-red-500 shadow-[0_0_8px_#ef4444]'
                                : 'bg-slate-700'
                            }`}
                            title="Front Left Acoustic Lens"
                          />
                          <div
                            className={`w-2.5 h-1.5 rounded-sm transition-all duration-300 ${
                              soundFocus === 'front' || soundFocus === 'all'
                                ? 'bg-amber-400 shadow-[0_0_6px_#fbbf24]'
                                : 'bg-slate-700'
                            }`}
                            title="Center Speaker"
                          />
                          <div
                            className={`w-2 h-2 rounded-full transition-all duration-300 ${
                              soundFocus === 'front' || soundFocus === 'all'
                                ? 'bg-red-500 shadow-[0_0_8px_#ef4444]'
                                : 'bg-slate-700'
                            }`}
                            title="Front Right Acoustic Lens"
                          />
                        </div>

                        {/* Front Cabin Seats */}
                        <div className="flex justify-between px-3">
                          {/* Driver Seat */}
                          <div
                            className={`w-7 h-8 rounded border text-[8px] font-mono flex items-center justify-center font-bold transition-all ${
                              soundFocus === 'driver'
                                ? 'bg-red-900/80 border-red-500 text-white shadow-[0_0_12px_rgba(239,68,68,0.6)]'
                                : 'bg-slate-800/80 border-slate-700 text-slate-400'
                            }`}
                          >
                            DRV
                          </div>
                          {/* Passenger Seat */}
                          <div
                            className={`w-7 h-8 rounded border text-[8px] font-mono flex items-center justify-center transition-all ${
                              soundFocus === 'front' || soundFocus === 'all'
                                ? 'bg-slate-800/80 border-slate-600 text-slate-300'
                                : 'bg-slate-900/80 border-slate-800 text-slate-600'
                            }`}
                          >
                            PSG
                          </div>
                        </div>

                        {/* Rear Cabin & Subwoofer */}
                        <div className="flex justify-between items-center px-2">
                          <div
                            className={`w-2 h-2 rounded-full transition-all duration-300 ${
                              soundFocus === 'rear' || soundFocus === 'all'
                                ? 'bg-red-500 shadow-[0_0_8px_#ef4444]'
                                : 'bg-slate-700'
                            }`}
                            title="Rear Left Door"
                          />
                          <div
                            className={`w-3.5 h-2 rounded transition-all duration-300 ${
                              soundFocus === 'all' || soundFocus === 'rear'
                                ? 'bg-red-600 shadow-[0_0_10px_#dc2626]'
                                : 'bg-slate-700'
                            }`}
                            title="Parcel Shelf Subwoofer"
                          />
                          <div
                            className={`w-2 h-2 rounded-full transition-all duration-300 ${
                              soundFocus === 'rear' || soundFocus === 'all'
                                ? 'bg-red-500 shadow-[0_0_8px_#ef4444]'
                                : 'bg-slate-700'
                            }`}
                            title="Rear Right Door"
                          />
                        </div>
                      </div>

                      {/* Sound Wave Ripple Effect when Playing */}
                      {isPlaying && (
                        <div
                          className="absolute inset-0 pointer-events-none rounded-lg border border-red-500/20 animate-ping opacity-30"
                          style={{ animationDuration: '2.5s' }}
                        />
                      )}
                    </div>

                    {/* Sound Focus Preset Buttons */}
                    <div className="grid grid-cols-4 gap-1">
                      {[
                        { id: 'all', label: 'ALL' },
                        { id: 'front', label: 'FRONT' },
                        { id: 'rear', label: 'REAR' },
                        { id: 'driver', label: 'DRIVER' },
                      ].map((item) => (
                        <button
                          key={item.id}
                          onClick={() => setSoundFocus(item.id as 'all' | 'front' | 'rear' | 'driver')}
                          className={`py-1 rounded text-[10px] font-bold font-mono transition active:scale-95 ${
                            soundFocus === item.id
                              ? 'bg-red-600 text-white shadow-sm shadow-red-600/50 border border-red-400'
                              : 'bg-slate-900 hover:bg-slate-800 text-slate-400 border border-slate-800'
                          }`}
                        >
                          {item.label}
                        </button>
                      ))}
                    </div>
                  </div>

                  {/* Real-Time Acoustic Spectrum Visualizer */}
                  <div className="mt-2 pt-2 border-t border-slate-800/80">
                    <div className="flex items-center justify-between mb-1 text-[9px] font-mono text-slate-400">
                      <span>FFT REAL-TIME SPECTRUM</span>
                      <div className="flex items-center gap-1">
                        <span>3D LEVEL:</span>
                        <div className="flex gap-0.5">
                          {[1, 2, 3, 4, 5].map((lvl) => (
                            <button
                              key={lvl}
                              onClick={() => setSurroundLevel(lvl)}
                              className={`w-2 h-2 rounded-xs transition ${
                                lvl <= surroundLevel ? 'bg-red-500' : 'bg-slate-800'
                              }`}
                            />
                          ))}
                        </div>
                      </div>
                    </div>
                    <canvas
                      ref={canvasRef}
                      width={280}
                      height={32}
                      className="w-full h-8 bg-black/80 rounded border border-slate-800/80"
                    />
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
