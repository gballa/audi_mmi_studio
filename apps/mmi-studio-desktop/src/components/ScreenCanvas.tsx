import React, { useState } from 'react';
import { ScreenRenderResult } from '../types';

interface ScreenCanvasProps {
  currentScreen?: ScreenRenderResult;
  onToggleMode: (mode: 'Day' | 'Night') => void;
}

export const ScreenCanvas: React.FC<ScreenCanvasProps> = ({
  currentScreen = { width: 800, height: 480, activeMode: 'Day', pixelCount: 384000 },
  onToggleMode,
}) => {
  const [selectedLayer, setSelectedLayer] = useState<string | null>(null);

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 p-4">
      <div className="flex items-center justify-between pb-4 border-b border-slate-800">
        <div>
          <h1 className="text-xl font-bold tracking-tight">Virtual Screen Canvas</h1>
          <p className="text-xs text-slate-400">
            Native 800x480 Virtual Cluster & Display Simulator (Derived Layout)
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => onToggleMode('Day')}
            className={`px-3 py-1 text-xs rounded border ${
              currentScreen.activeMode === 'Day'
                ? 'bg-amber-600 border-amber-500 text-white font-bold'
                : 'bg-slate-900 border-slate-700 text-slate-300'
            }`}
          >
            Day Mode
          </button>
          <button
            onClick={() => onToggleMode('Night')}
            className={`px-3 py-1 text-xs rounded border ${
              currentScreen.activeMode === 'Night'
                ? 'bg-indigo-700 border-indigo-600 text-white font-bold'
                : 'bg-slate-900 border-slate-700 text-slate-300'
            }`}
          >
            Night Mode
          </button>
        </div>
      </div>

      <div className="flex flex-1 items-center justify-center p-6 overflow-auto">
        <div
          className="relative bg-[#121418] border-2 border-slate-800 rounded shadow-2xl overflow-hidden flex flex-col justify-between"
          style={{ width: '800px', height: '480px' }}
        >
          {/* Top Status Bar */}
          <div className="flex justify-between items-center px-4 py-2 bg-slate-900/80 border-b border-slate-800 text-xs font-mono">
            <span className="text-slate-400">12:45 PM · 21.5°C</span>
            <span className="text-amber-500 font-bold">AUDI NAVIGATION PLUS</span>
            <span className="text-slate-400">LTE · TMC PRO</span>
          </div>

          {/* Central Map Canvas Simulation */}
          <div className="flex-1 flex flex-col items-center justify-center p-4">
            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-2 border border-amber-500/50 rounded-full flex items-center justify-center text-amber-500 font-mono text-xs animate-pulse">
                NAV
              </div>
              <div className="text-sm font-semibold text-slate-200">Ingolstadt, Germany</div>
              <div className="text-xs text-slate-500 mt-1">
                Display: {currentScreen.width}x{currentScreen.height} ({currentScreen.activeMode})
              </div>
            </div>
          </div>

          {/* Bottom Control Bar */}
          <div className="flex justify-between items-center px-4 py-2 bg-slate-900/80 border-t border-slate-800 text-xs">
            <span className="text-slate-400 font-mono">Mode: {currentScreen.activeMode}</span>
            <span className="text-slate-500 text-[10px]">Layout: Derived [EV:tree]</span>
          </div>
        </div>
      </div>
    </div>
  );
};
