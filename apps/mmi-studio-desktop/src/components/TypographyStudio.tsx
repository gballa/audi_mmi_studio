import React, { useState, useMemo } from 'react';

interface TypographyStudioProps {
  initialText?: string;
  initialWidth?: number;
  initialHeight?: number;
}

export const TypographyStudio: React.FC<TypographyStudioProps> = ({
  initialText = 'AUDI NAVIGATION PLUS',
  initialWidth = 240,
  initialHeight = 32,
}) => {
  const [text, setText] = useState(initialText);
  const [boxWidth, setBoxWidth] = useState(initialWidth);
  const [boxHeight, setBoxHeight] = useState(initialHeight);
  const [fontSize, setFontSize] = useState(16);
  const [fontFamily, setFontFamily] = useState('AudiType-Bold');
  const [letterSpacing, setLetterSpacing] = useState(1.0);

  // Approximate glyph width estimator based on AudiType Linotype metrics
  const estimatedTextWidth = useMemo(() => {
    // Average character width factor relative to font size for AudiType
    const avgCharFactor = fontFamily.includes('Bold') ? 0.65 : 0.58;
    const baseWidth = text.length * fontSize * avgCharFactor;
    const spacingWidth = (text.length - 1) * (letterSpacing - 1.0) * 8;
    return Math.round(baseWidth + spacingWidth);
  }, [text, fontSize, fontFamily, letterSpacing]);

  const isOverflow = estimatedTextWidth > boxWidth;
  const overflowPx = estimatedTextWidth - boxWidth;

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 p-4 font-sans">
      {/* Header */}
      <div className="flex items-center justify-between pb-4 border-b border-slate-800">
        <div>
          <h1 className="text-xl font-bold tracking-tight">Typography Studio</h1>
          <p className="text-xs text-slate-400">
            Linotype glyph rendering & label bounding-box overflow safety verifier
          </p>
        </div>
        <div className="flex items-center gap-3">
          {isOverflow ? (
            <span className="px-2.5 py-1 text-xs rounded border bg-red-950 border-red-800 text-red-300 font-bold flex items-center gap-1">
              <span>⚠ OVERFLOW DETECTED: +{overflowPx}px</span>
            </span>
          ) : (
            <span className="px-2.5 py-1 text-xs rounded border bg-emerald-950 border-emerald-800 text-emerald-300 font-bold flex items-center gap-1">
              <span>✓ WITHIN BOUNDS (Margin: {boxWidth - estimatedTextWidth}px)</span>
            </span>
          )}
        </div>
      </div>

      {/* Control Strip */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mt-4 p-4 bg-slate-900 border border-slate-800 rounded">
        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1">
            Font Face (Linotype / MMI)
          </label>
          <select
            value={fontFamily}
            onChange={(e) => setFontFamily(e.target.value)}
            className="w-full px-3 py-1.5 bg-slate-950 border border-slate-700 rounded text-xs focus:outline-none focus:border-amber-500"
          >
            <option value="AudiType-Normal">AudiType Normal</option>
            <option value="AudiType-Bold">AudiType Bold</option>
            <option value="AudiType-Extended">AudiType Extended</option>
            <option value="AudiType-Mono">AudiType Monospace</option>
          </select>
        </div>

        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1">
            Font Size: {fontSize}px
          </label>
          <input
            type="range"
            min="10"
            max="36"
            value={fontSize}
            onChange={(e) => setFontSize(Number(e.target.value))}
            className="w-full accent-amber-500"
          />
        </div>

        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1">
            Bounding Box: {boxWidth} × {boxHeight} px
          </label>
          <div className="flex gap-2">
            <input
              type="number"
              value={boxWidth}
              onChange={(e) => setBoxWidth(Math.max(50, Number(e.target.value)))}
              className="w-1/2 px-2 py-1 bg-slate-950 border border-slate-700 rounded text-xs"
              placeholder="Width"
            />
            <input
              type="number"
              value={boxHeight}
              onChange={(e) => setBoxHeight(Math.max(16, Number(e.target.value)))}
              className="w-1/2 px-2 py-1 bg-slate-950 border border-slate-700 rounded text-xs"
              placeholder="Height"
            />
          </div>
        </div>

        <div>
          <label className="block text-xs font-semibold text-slate-400 mb-1">
            Tracking / Letter Spacing: {letterSpacing.toFixed(1)}px
          </label>
          <input
            type="range"
            min="0.5"
            max="3.0"
            step="0.1"
            value={letterSpacing}
            onChange={(e) => setLetterSpacing(Number(e.target.value))}
            className="w-full accent-amber-500"
          />
        </div>
      </div>

      {/* Input String */}
      <div className="mt-4">
        <label className="block text-xs font-semibold text-slate-400 mb-1">
          Label String (Localized Key Preview)
        </label>
        <input
          type="text"
          value={text}
          onChange={(e) => setText(e.target.value)}
          placeholder="Enter label string to verify bounds..."
          className="w-full px-3 py-2 bg-slate-900 border border-slate-800 rounded text-sm focus:outline-none focus:border-amber-500"
        />
      </div>

      {/* Interactive Preview Canvas */}
      <div className="flex-1 flex flex-col items-center justify-center p-6 mt-4 bg-slate-900 border border-slate-800 rounded overflow-auto">
        <div className="text-center mb-4">
          <span className="text-xs text-slate-400">
            Target Render Box ({boxWidth}px × {boxHeight}px)
          </span>
        </div>

        {/* Target Bounding Box Container */}
        <div
          className={`relative border-2 ${
            isOverflow
              ? 'border-red-500 bg-red-950/20 shadow-[0_0_20px_rgba(239,68,68,0.2)]'
              : 'border-emerald-500 bg-emerald-950/10'
          } rounded flex items-center justify-center transition-all`}
          style={{ width: `${boxWidth}px`, height: `${boxHeight}px` }}
        >
          <div
            className="truncate text-slate-100 whitespace-nowrap select-none"
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
            {boxWidth} px (Text: ~{estimatedTextWidth} px)
          </div>
        </div>

        {isOverflow && (
          <div className="mt-8 p-3 bg-red-950/80 border border-red-800 rounded text-xs text-red-200 max-w-md text-center">
            <strong>Warning: Text Truncation Hazard.</strong> The label string exceeds its allocation by{' '}
            <span className="font-mono font-bold">{overflowPx}px</span>. On native hardware, this will result
            in clipping or ellipsis in the MMI menu system.
          </div>
        )}
      </div>
    </div>
  );
};
