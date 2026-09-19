import React, { useState, useMemo } from 'react';

interface TypographyStudioProps {
  initialText?: string;
  initialWidth?: number;
  initialHeight?: number;
}

export const TypographyStudio: React.FC<TypographyStudioProps> = ({
  initialText = 'AUDI NAVIGATION PLUS',
  initialWidth = 240,
  initialHeight = 36,
}) => {
  const [text, setText] = useState(initialText);
  const [boxWidth, setBoxWidth] = useState(initialWidth);
  const [boxHeight, setBoxHeight] = useState(initialHeight);
  const [fontSize, setFontSize] = useState(16);
  const [fontFamily, setFontFamily] = useState('AudiType-Bold');
  const [letterSpacing, setLetterSpacing] = useState(1.0);

  // Approximate glyph width estimator based on AudiType Linotype metrics
  const estimatedTextWidth = useMemo(() => {
    const avgCharFactor = fontFamily.includes('Bold') ? 0.65 : 0.58;
    const baseWidth = text.length * fontSize * avgCharFactor;
    const spacingWidth = (text.length - 1) * (letterSpacing - 1.0) * 8;
    return Math.round(baseWidth + spacingWidth);
  }, [text, fontSize, fontFamily, letterSpacing]);

  const isOverflow = estimatedTextWidth > boxWidth;
  const overflowPx = estimatedTextWidth - boxWidth;

  return (
    <div className="flex flex-col h-full w-full bg-slate-950 text-slate-100 p-4 md:p-6 font-sans overflow-y-auto min-w-0 space-y-4">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-slate-800 shrink-0">
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-xl font-bold tracking-tight text-white flex items-center gap-2">
              <span>🔤</span>
              <span>Typography Lab & Overflow Verifier</span>
            </h1>
            <span className="px-2 py-0.5 text-xs bg-slate-800 text-slate-300 font-mono rounded">
              Linotype Fonts
            </span>
          </div>
          <p className="text-xs text-slate-400 mt-0.5">
            AudiType glyph rasterization, bounding box boundary verification, and text clipping prevention
          </p>
        </div>

        <div className="flex items-center gap-3 shrink-0">
          {isOverflow ? (
            <span className="px-3 py-1 text-xs rounded-lg border bg-red-950/80 border-red-800 text-red-300 font-bold flex items-center gap-1.5 shadow-sm">
              <span>⚠ OVERFLOW DETECTED: +{overflowPx}px</span>
            </span>
          ) : (
            <span className="px-3 py-1 text-xs rounded-lg border bg-emerald-950/60 border-emerald-800 text-emerald-300 font-bold flex items-center gap-1.5 shadow-sm">
              <span>✓ WITHIN BOUNDS (Margin: {boxWidth - estimatedTextWidth}px)</span>
            </span>
          )}
        </div>
      </div>

      {/* Control Strip */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 p-4 bg-slate-900/80 border border-slate-800 rounded-xl shrink-0">
        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1.5 uppercase tracking-wider">
            Font Face (Linotype)
          </label>
          <select
            value={fontFamily}
            onChange={(e) => setFontFamily(e.target.value)}
            className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-lg text-xs text-slate-200 focus:outline-none focus:border-amber-500"
          >
            <option value="AudiType-Normal">AudiType Normal</option>
            <option value="AudiType-Bold">AudiType Bold</option>
            <option value="AudiType-Extended">AudiType Extended</option>
            <option value="AudiType-Mono">AudiType Monospace</option>
          </select>
        </div>

        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1.5 uppercase tracking-wider flex justify-between">
            <span>Font Size</span>
            <span className="font-mono text-amber-400 font-bold">{fontSize}px</span>
          </label>
          <input
            type="range"
            min="10"
            max="36"
            value={fontSize}
            onChange={(e) => setFontSize(Number(e.target.value))}
            className="w-full accent-amber-500 mt-1"
          />
        </div>

        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1.5 uppercase tracking-wider">
            Bounding Box (W × H)
          </label>
          <div className="flex gap-2">
            <input
              type="number"
              value={boxWidth}
              onChange={(e) => setBoxWidth(Math.max(50, Number(e.target.value)))}
              className="w-1/2 px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-xs font-mono text-white"
              placeholder="Width"
            />
            <input
              type="number"
              value={boxHeight}
              onChange={(e) => setBoxHeight(Math.max(16, Number(e.target.value)))}
              className="w-1/2 px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-xs font-mono text-white"
              placeholder="Height"
            />
          </div>
        </div>

        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1.5 uppercase tracking-wider flex justify-between">
            <span>Tracking / Letter Spacing</span>
            <span className="font-mono text-amber-400 font-bold">{letterSpacing.toFixed(1)}px</span>
          </label>
          <input
            type="range"
            min="0.5"
            max="3.0"
            step="0.1"
            value={letterSpacing}
            onChange={(e) => setLetterSpacing(Number(e.target.value))}
            className="w-full accent-amber-500 mt-1"
          />
        </div>
      </div>

      {/* Input String */}
      <div className="p-4 bg-slate-900/60 border border-slate-800 rounded-xl shrink-0">
        <label className="block text-xs font-semibold text-slate-400 mb-1.5 uppercase tracking-wider">
          Label String (Localized Key Preview)
        </label>
        <input
          type="text"
          value={text}
          onChange={(e) => setText(e.target.value)}
          placeholder="Enter label string to verify bounds..."
          className="w-full px-3.5 py-2.5 bg-slate-950 border border-slate-700 rounded-lg text-sm text-white font-medium focus:outline-none focus:border-amber-500 transition"
        />
      </div>

      {/* Interactive Preview Canvas */}
      <div className="flex-1 min-h-[260px] flex flex-col items-center justify-center p-8 bg-[#080b10] border border-slate-800 rounded-xl overflow-x-auto relative">
        <div className="text-center mb-6">
          <span className="text-xs font-mono text-slate-400">
            Target Hardware Slot ({boxWidth}px × {boxHeight}px) · Estimated Text: ~{estimatedTextWidth}px
          </span>
        </div>

        {/* Target Bounding Box Container */}
        <div className="max-w-full overflow-x-auto py-4 px-6 flex justify-center">
          <div
            className={`relative border-2 ${
              isOverflow
                ? 'border-red-500 bg-red-950/25 shadow-[0_0_25px_rgba(239,68,68,0.3)]'
                : 'border-emerald-500/80 bg-emerald-950/15 shadow-[0_0_15px_rgba(16,185,129,0.15)]'
            } rounded-lg flex items-center justify-center transition-all shrink-0 px-3`}
            style={{ width: `${boxWidth}px`, height: `${boxHeight}px` }}
          >
            <div
              className="text-slate-100 whitespace-nowrap select-none"
              style={{
                fontSize: `${fontSize}px`,
                letterSpacing: `${(letterSpacing - 1.0) * 4}px`,
                fontFamily: fontFamily.includes('Mono') ? 'monospace' : 'sans-serif',
                fontWeight: fontFamily.includes('Bold') ? 700 : 400,
              }}
            >
              {text}
            </div>

            {/* Dimension Guide Lines */}
            <div className="absolute -bottom-5 left-0 right-0 text-center text-[10px] text-slate-500 font-mono">
              Max: {boxWidth} px
            </div>
          </div>
        </div>

        {isOverflow && (
          <div className="mt-8 p-3.5 bg-red-950/90 border border-red-800 rounded-xl text-xs text-red-200 max-w-md text-center shadow-lg">
            <strong>Warning: Text Truncation Hazard.</strong> The label string exceeds its hardware allocation by{' '}
            <span className="font-mono font-bold text-red-300">+{overflowPx}px</span>. On the physical Audi MMI 3G display,
            this will result in clipping or ellipsis. Shorten wording or reduce font tracking.
          </div>
        )}
      </div>
    </div>
  );
};
