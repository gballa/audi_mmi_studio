import React, { useState } from 'react';

export interface ResetSelectiveOptions {
  theme: boolean;
  aiElements: boolean;
  localization: boolean;
  maps: boolean;
  build: boolean;
}

interface ResetModalProps {
  isOpen: boolean;
  onClose: () => void;
  activeTab: string;
  onResetAll: () => void;
  onResetFeature: (tab: string) => void;
  onResetSelective: (options: ResetSelectiveOptions) => void;
}

export const ResetModal: React.FC<ResetModalProps> = ({
  isOpen,
  onClose,
  activeTab,
  onResetAll,
  onResetFeature,
  onResetSelective,
}) => {
  const [selectedOptions, setSelectedOptions] = useState<ResetSelectiveOptions>({
    theme: true,
    aiElements: true,
    localization: true,
    maps: true,
    build: false,
  });

  const [confirmFactoryReset, setConfirmFactoryReset] = useState<boolean>(false);

  if (!isOpen) return null;

  const getFeatureDetails = (tab: string): { title: string; desc: string; icon: string } => {
    switch (tab) {
      case 'components':
        return {
          title: 'UI & Component Theme',
          desc: 'Reverts accent color, needle colors, fonts, ambient glow, and component layout to OEM default.',
          icon: '🎨',
        };
      case 'ai_elements':
        return {
          title: 'Gemini AI Elements',
          desc: 'Reverts all restyled needles, compass roses, silhouettes, dials, and textures back to stock OEM.',
          icon: '🍌',
        };
      case 'localization':
        return {
          title: 'Albanian Localization',
          desc: 'Reverts all 43+ system string translations back to the baseline verified Albanian catalog.',
          icon: '🇦🇱',
        };
      case 'maps':
        return {
          title: '2026 Navigation Map Patches',
          desc: 'Reverts staged road vectors (A1, Llogara, Arbrit), speed radar POIs, and EV hubs to default.',
          icon: '🗺️',
        };
      case 'build':
        return {
          title: 'Build Staging & Logs',
          desc: 'Clears build staging state, logs, and resets build status back to idle.',
          icon: '🚀',
        };
      default:
        return {
          title: 'Current Feature',
          desc: 'Reverts the active feature configuration to pristine defaults.',
          icon: '⚙️',
        };
    }
  };

  const currentFeature = getFeatureDetails(activeTab);

  const handleResetCurrentOnly = () => {
    onResetFeature(activeTab);
    onClose();
  };

  const handleResetSelectiveSubmit = () => {
    onResetSelective(selectedOptions);
    onClose();
  };

  const handleResetAllSubmit = () => {
    onResetAll();
    onClose();
  };

  const selectedCount = Object.values(selectedOptions).filter(Boolean).length;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4 animate-fade-in">
      <div className="w-full max-w-xl bg-slate-900 border border-slate-700 rounded-xl shadow-2xl overflow-hidden flex flex-col">
        {/* Header */}
        <div className="p-4 bg-slate-950/80 border-b border-slate-800 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-xl">↺</span>
            <div>
              <h3 className="text-base font-bold text-white tracking-tight flex items-center gap-2">
                Reset Workstation Configuration
              </h3>
              <p className="text-xs text-slate-400">
                Revert customizations back to factory stock baseline
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-md text-slate-400 hover:text-white hover:bg-slate-800 transition-all text-sm font-bold"
          >
            ✕
          </button>
        </div>

        {/* Modal Body */}
        <div className="p-5 space-y-5 overflow-y-auto max-h-[75vh]">
          {/* Section 1: Reset Active Feature Only */}
          <div className="p-4 bg-slate-950 border border-amber-500/30 rounded-lg space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold uppercase tracking-wider text-amber-400 flex items-center gap-1.5">
                <span>Option 1:</span> Reset Active Feature Only
              </span>
              <span className="px-2 py-0.5 rounded text-[10px] bg-amber-500/10 text-amber-300 font-mono border border-amber-500/20">
                Active: {currentFeature.icon} {currentFeature.title}
              </span>
            </div>
            <p className="text-xs text-slate-300">{currentFeature.desc}</p>
            <div className="pt-2">
              <button
                onClick={handleResetCurrentOnly}
                className="w-full py-2 px-3 bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/50 text-amber-200 hover:text-amber-100 rounded-lg text-xs font-bold transition-all flex items-center justify-center gap-2"
              >
                <span>↺</span> Reset Only {currentFeature.title}
              </button>
            </div>
          </div>

          {/* Section 2: Selective Reset */}
          <div className="p-4 bg-slate-950 border border-slate-800 rounded-lg space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold uppercase tracking-wider text-slate-300">
                Option 2: Selective Reset
              </span>
              <span className="text-[11px] text-slate-400 font-mono">
                {selectedCount} features selected
              </span>
            </div>
            <div className="space-y-2">
              <label className="flex items-center gap-2.5 p-2 rounded bg-slate-900/60 border border-slate-800/80 hover:border-slate-700 cursor-pointer text-xs">
                <input
                  type="checkbox"
                  checked={selectedOptions.theme}
                  onChange={(e) =>
                    setSelectedOptions((prev) => ({ ...prev, theme: e.target.checked }))
                  }
                  className="rounded border-slate-700 text-amber-500 focus:ring-0"
                />
                <span className="text-white font-medium">🎨 UI & Component Styling</span>
                <span className="text-slate-400 text-[11px] ml-auto">Colors, needles, fonts</span>
              </label>

              <label className="flex items-center gap-2.5 p-2 rounded bg-slate-900/60 border border-slate-800/80 hover:border-slate-700 cursor-pointer text-xs">
                <input
                  type="checkbox"
                  checked={selectedOptions.aiElements}
                  onChange={(e) =>
                    setSelectedOptions((prev) => ({ ...prev, aiElements: e.target.checked }))
                  }
                  className="rounded border-slate-700 text-amber-500 focus:ring-0"
                />
                <span className="text-white font-medium">🍌 Gemini AI Elements</span>
                <span className="text-slate-400 text-[11px] ml-auto">Revert to stock 8 assets</span>
              </label>

              <label className="flex items-center gap-2.5 p-2 rounded bg-slate-900/60 border border-slate-800/80 hover:border-slate-700 cursor-pointer text-xs">
                <input
                  type="checkbox"
                  checked={selectedOptions.localization}
                  onChange={(e) =>
                    setSelectedOptions((prev) => ({ ...prev, localization: e.target.checked }))
                  }
                  className="rounded border-slate-700 text-amber-500 focus:ring-0"
                />
                <span className="text-white font-medium">🇦🇱 Albanian Localization</span>
                <span className="text-slate-400 text-[11px] ml-auto">Reset 43+ strings</span>
              </label>

              <label className="flex items-center gap-2.5 p-2 rounded bg-slate-900/60 border border-slate-800/80 hover:border-slate-700 cursor-pointer text-xs">
                <input
                  type="checkbox"
                  checked={selectedOptions.maps}
                  onChange={(e) =>
                    setSelectedOptions((prev) => ({ ...prev, maps: e.target.checked }))
                  }
                  className="rounded border-slate-700 text-amber-500 focus:ring-0"
                />
                <span className="text-white font-medium">🗺️ 2026 Navigation Maps</span>
                <span className="text-slate-400 text-[11px] ml-auto">Roads, radars, EV hubs</span>
              </label>

              <label className="flex items-center gap-2.5 p-2 rounded bg-slate-900/60 border border-slate-800/80 hover:border-slate-700 cursor-pointer text-xs">
                <input
                  type="checkbox"
                  checked={selectedOptions.build}
                  onChange={(e) =>
                    setSelectedOptions((prev) => ({ ...prev, build: e.target.checked }))
                  }
                  className="rounded border-slate-700 text-amber-500 focus:ring-0"
                />
                <span className="text-white font-medium">🚀 Build Staging & Logs</span>
                <span className="text-slate-400 text-[11px] ml-auto">Reset build state</span>
              </label>
            </div>
            <button
              onClick={handleResetSelectiveSubmit}
              disabled={selectedCount === 0}
              className="w-full py-2 px-3 bg-slate-800 hover:bg-slate-700 disabled:opacity-40 disabled:cursor-not-allowed border border-slate-600 text-white rounded-lg text-xs font-bold transition-all"
            >
              Reset Selected Features ({selectedCount})
            </button>
          </div>

          {/* Section 3: Reset Everything (Factory Reset) */}
          <div className="p-4 bg-red-950/20 border border-red-900/50 rounded-lg space-y-3">
            <span className="text-xs font-bold uppercase tracking-wider text-red-400 flex items-center gap-1.5">
              <span>⚠️ Option 3:</span> Full Factory Reset (Reset Everything)
            </span>
            <p className="text-xs text-slate-300">
              Restores the entire workstation session to pristine OEM defaults across Theme, AI Assets,
              Albanian Translations, 2026 Maps, and Build Staging.
            </p>
            <div className="flex items-center gap-2 pt-1">
              <input
                type="checkbox"
                id="confirmResetAll"
                checked={confirmFactoryReset}
                onChange={(e) => setConfirmFactoryReset(e.target.checked)}
                className="rounded border-red-800 text-red-600 focus:ring-0"
              />
              <label htmlFor="confirmResetAll" className="text-xs text-red-300 font-medium cursor-pointer">
                I understand this will revert all session customizations to OEM factory defaults.
              </label>
            </div>
            <button
              onClick={handleResetAllSubmit}
              disabled={!confirmFactoryReset}
              className="w-full py-2 px-3 bg-red-600 hover:bg-red-500 disabled:opacity-40 disabled:cursor-not-allowed text-white rounded-lg text-xs font-bold transition-all shadow"
            >
              ⚠️ Reset Everything (Factory Reset)
            </button>
          </div>
        </div>

        {/* Footer Note */}
        <div className="p-3 bg-slate-950 border-t border-slate-800 flex items-center justify-between text-[10px] text-slate-400">
          <span className="flex items-center gap-1 font-mono">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400" />
            Originals in <code className="text-slate-300 font-bold">originals/</code> are 100% immutable
          </span>
          <button
            onClick={onClose}
            className="px-3 py-1 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded font-medium text-xs transition-all"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
};
