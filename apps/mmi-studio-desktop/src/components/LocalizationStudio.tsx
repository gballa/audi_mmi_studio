import React, { useState, useMemo } from 'react';
import { SystemString } from '../types';

interface LocalizationStudioProps {
  strings: SystemString[];
  onUpdateString: (id: string, newAlbanian: string) => void;
  onPreviewStringInScreen?: (stringItem: SystemString) => void;
  onResetAllStrings?: () => void;
  onResetSingleString?: (id: string) => void;
}

export const LocalizationStudio: React.FC<LocalizationStudioProps> = ({
  strings,
  onUpdateString,
  onPreviewStringInScreen,
  onResetAllStrings,
  onResetSingleString,
}) => {
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [activeStringId, setActiveStringId] = useState<string>(strings[0]?.id || '');
  const [fontFamily, setFontFamily] = useState<'AudiType-Normal' | 'AudiType-Bold' | 'AudiType-Extended'>('AudiType-Bold');
  const [fontSize, setFontSize] = useState<number>(16);
  const [exportNotice, setExportNotice] = useState<string | null>(null);

  const categories = ['All', 'Navigation', 'Media', 'Radio', 'Telephone', 'Car Setup', 'Climate', 'System Alerts'];

  const filteredStrings = useMemo(() => {
    return strings.filter((item) => {
      const matchesCat = selectedCategory === 'All' || item.category === selectedCategory;
      const q = searchQuery.toLowerCase();
      const matchesSearch =
        item.en.toLowerCase().includes(q) ||
        item.sq.toLowerCase().includes(q) ||
        item.de.toLowerCase().includes(q) ||
        item.id.toLowerCase().includes(q);
      return matchesCat && matchesSearch;
    });
  }, [strings, selectedCategory, searchQuery]);

  const activeItem = strings.find((s) => s.id === activeStringId) || strings[0];

  // Precise glyph width estimator for AudiType fonts including Albanian diacritics (ë, ç, Ë, Ç)
  const calcTextWidth = (text: string, size: number, family: string): number => {
    const boldMultiplier = family === 'AudiType-Bold' ? 0.64 : family === 'AudiType-Extended' ? 0.72 : 0.58;
    let width = 0;
    for (let i = 0; i < text.length; i++) {
      const char = text[i];
      if (/[A-ZËÇMWQO#@]/.test(char)) {
        width += size * boldMultiplier * 1.25;
      } else if (/[ijl1!.,;:'|\s]/.test(char)) {
        width += size * boldMultiplier * 0.45;
      } else if (/[ëç]/.test(char)) {
        width += size * boldMultiplier * 1.05;
      } else {
        width += size * boldMultiplier;
      }
    }
    return Math.round(width);
  };

  const sqWidth = useMemo(() => {
    if (!activeItem) return 0;
    return calcTextWidth(activeItem.sq, fontSize, fontFamily);
  }, [activeItem, fontSize, fontFamily]);

  const enWidth = useMemo(() => {
    if (!activeItem) return 0;
    return calcTextWidth(activeItem.en, fontSize, fontFamily);
  }, [activeItem, fontSize, fontFamily]);

  const isSqOverflow = activeItem ? sqWidth > activeItem.maxPixels : false;
  const sqOverflowAmount = activeItem ? sqWidth - activeItem.maxPixels : 0;

  const handleExportCatalog = () => {
    const catalog = {
      locale: 'sq_AL',
      language: 'Gjuha Shqipe (Albanian)',
      targetSystem: 'Audi MMI 3G High / Plus [HN+]',
      generatedAt: new Date().toISOString(),
      stringCount: strings.length,
      encoding: 'UTF-8 / ISO-8859-16 (Balkan Latin-10)',
      entries: strings.map((s) => ({
        id: s.id,
        category: s.category,
        source_en: s.en,
        source_de: s.de,
        translated_sq: s.sq,
        maxPixels: s.maxPixels,
      })),
    };

    const blob = new Blob([JSON.stringify(catalog, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'audi_mmi3g_sq_AL_strings.json';
    a.click();
    URL.revokeObjectURL(url);

    setExportNotice('Exported sq_AL translation catalog (JSON & Flash-ready format)!');
    setTimeout(() => setExportNotice(null), 3500);
  };

  return (
    <div className="flex h-full w-full bg-slate-950 text-slate-100 overflow-hidden font-sans min-w-0">
      {/* Left String Directory */}
      <div className="w-80 xl:w-96 flex flex-col shrink-0 h-full border-r border-slate-800 bg-slate-900/60 min-w-0">
        {/* Header & Filter */}
        <div className="p-3 border-b border-slate-800 space-y-2 shrink-0">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold tracking-tight text-white">🇦🇱 Albanian Studio</span>
              <span className="px-1.5 py-0.5 text-[10px] bg-red-950/80 border border-red-800 text-red-300 font-mono rounded">
                sq_AL
              </span>
            </div>
            <span className="text-[11px] text-slate-400 font-mono">
              {filteredStrings.length}/{strings.length} keys
            </span>
          </div>

          {/* Search Box */}
          <input
            type="text"
            placeholder="Search string, ID, or Albanian..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-800 rounded text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-amber-500"
          />

          {/* Category Tabs */}
          <div className="flex flex-wrap gap-1 pt-1">
            {categories.map((cat) => (
              <button
                key={cat}
                onClick={() => setSelectedCategory(cat)}
                className={`px-2 py-0.5 text-[10px] rounded transition-all ${
                  selectedCategory === cat
                    ? 'bg-amber-500 text-slate-950 font-bold'
                    : 'bg-slate-800/80 text-slate-400 hover:text-slate-200'
                }`}
              >
                {cat}
              </button>
            ))}
          </div>
        </div>

        {/* String List */}
        <div className="flex-1 overflow-y-auto divide-y divide-slate-800/50 min-w-0">
          {filteredStrings.map((item) => {
            const isSelected = item.id === activeStringId;
            const w = calcTextWidth(item.sq, 14, fontFamily);
            const hasOverflow = w > item.maxPixels;

            return (
              <div
                key={item.id}
                onClick={() => setActiveStringId(item.id)}
                className={`p-2.5 cursor-pointer transition-colors ${
                  isSelected ? 'bg-amber-500/10 border-l-2 border-amber-500' : 'hover:bg-slate-800/40'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="text-[10px] font-mono text-slate-400 truncate max-w-[170px] xl:max-w-[210px]">{item.id}</span>
                  <div className="flex items-center gap-1.5 shrink-0">
                    {hasOverflow ? (
                      <span className="px-1.5 py-0.2 text-[9px] bg-red-950 border border-red-800 text-red-300 font-bold rounded">
                        OVERFLOW
                      </span>
                    ) : (
                      <span className="px-1.5 py-0.2 text-[9px] bg-emerald-950/60 border border-emerald-800/80 text-emerald-400 rounded">
                        FIT
                      </span>
                    )}
                  </div>
                </div>
                <div className="text-xs font-medium text-slate-200 mt-1 truncate">{item.sq}</div>
                <div className="text-[11px] text-slate-400 truncate mt-0.5">EN: {item.en}</div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Right String Inspector & Bounding Box Studio */}
      <div className="flex-1 min-w-0 flex flex-col h-full overflow-y-auto bg-slate-950 p-4 lg:p-6 space-y-5">
        {/* Top Action Bar */}
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-3 pb-4 border-b border-slate-800 shrink-0">
          <div className="min-w-0">
            <div className="flex items-center gap-2 flex-wrap">
              <h2 className="text-base md:text-lg font-bold text-white tracking-tight">Localization & Overflow Safety Engine</h2>
              <span className="px-2 py-0.5 text-[10px] bg-blue-950 border border-blue-800 text-blue-300 rounded font-mono shrink-0">
                TrueType / Linotype Rasterizer
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              MMI 3G High / Plus string table patching with real-time hardware bounding box verification
            </p>
          </div>

          <div className="flex items-center gap-2 flex-wrap shrink-0">
            {exportNotice && (
              <span className="text-xs text-emerald-400 font-medium animate-pulse mr-1">{exportNotice}</span>
            )}
            {onPreviewStringInScreen && activeItem && (
              <button
                onClick={() => onPreviewStringInScreen(activeItem)}
                className="px-2.5 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded border border-slate-700 transition flex items-center gap-1.5 shrink-0"
              >
                <span>👁️</span> Preview in Canvas
              </button>
            )}
            {onResetAllStrings && (
              <button
                onClick={onResetAllStrings}
                className="px-2.5 py-1.5 bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white text-xs font-semibold rounded border border-slate-700 transition flex items-center gap-1 shrink-0"
                title="Reset all strings back to verified baseline"
              >
                <span>↺</span> Reset All
              </button>
            )}
            <button
              onClick={handleExportCatalog}
              className="px-3 py-1.5 bg-amber-500 hover:bg-amber-400 text-slate-950 text-xs font-bold rounded shadow transition flex items-center gap-1.5 shrink-0"
            >
              <span>💾</span> Export sq_AL
            </button>
          </div>
        </div>

        {activeItem ? (
          <div className="space-y-5 min-w-0">
            {/* Metadata Banner */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-3 p-3.5 bg-slate-900 border border-slate-800 rounded-lg min-w-0">
              <div className="min-w-0">
                <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">Key ID</span>
                <span className="font-mono text-xs text-amber-400 font-bold truncate block">{activeItem.id}</span>
              </div>
              <div className="min-w-0">
                <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">MMI Subsystem</span>
                <span className="text-xs text-slate-200 font-medium truncate block">{activeItem.category}</span>
              </div>
              <div className="min-w-0">
                <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">Display Slot</span>
                <span className="text-xs font-mono text-slate-200 truncate block">{activeItem.maxPixels} px max</span>
              </div>
              <div className="min-w-0">
                <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">Context</span>
                <span className="text-xs text-slate-300 truncate block" title={activeItem.context}>
                  {activeItem.context}
                </span>
              </div>
            </div>

            {/* Translation Input Matrix */}
            <div className="space-y-4 bg-slate-900/60 p-4 md:p-5 rounded-lg border border-slate-800 min-w-0">
              <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400">Translation Matrix</h3>

              {/* English OEM Original */}
              <div className="space-y-1">
                <div className="flex justify-between text-xs">
                  <span className="text-slate-400 font-medium">English (OEM Reference):</span>
                  <span className="font-mono text-slate-400 text-[11px]">~{enWidth} px</span>
                </div>
                <div className="p-2.5 bg-slate-950 border border-slate-800 rounded text-sm text-slate-300 select-all">
                  {activeItem.en}
                </div>
              </div>

              {/* German OEM Original */}
              <div className="space-y-1">
                <div className="flex justify-between text-xs">
                  <span className="text-slate-400 font-medium">German (Audi OEM Stock):</span>
                </div>
                <div className="p-2.5 bg-slate-950 border border-slate-800 rounded text-sm text-slate-400 select-all">
                  {activeItem.de}
                </div>
              </div>

              {/* Albanian Interactive Translation */}
              <div className="space-y-2 pt-2 border-t border-slate-800">
                <div className="flex flex-wrap justify-between items-center gap-2 text-xs">
                  <div className="flex items-center gap-2">
                    <span className="font-bold text-amber-400">🇦🇱 Gjuha Shqipe (Albanian Translation):</span>
                    <span className="text-[10px] text-slate-400 font-mono">Supports ë, ç, Ë, Ç</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className={`font-mono font-bold text-xs ${isSqOverflow ? 'text-red-400' : 'text-emerald-400'}`}>
                      {sqWidth} px / {activeItem.maxPixels} px
                    </span>
                    {isSqOverflow ? (
                      <span className="px-2 py-0.5 text-[10px] bg-red-950 border border-red-800 text-red-300 font-bold rounded">
                        ⚠ OVERFLOW (+{sqOverflowAmount}px)
                      </span>
                    ) : (
                      <span className="px-2 py-0.5 text-[10px] bg-emerald-950 border border-emerald-800 text-emerald-300 font-bold rounded">
                        ✓ OK (Margin: {activeItem.maxPixels - sqWidth}px)
                      </span>
                    )}
                  </div>
                </div>

                <div className="flex flex-col sm:flex-row gap-2">
                  <input
                    type="text"
                    value={activeItem.sq}
                    onChange={(e) => onUpdateString(activeItem.id, e.target.value)}
                    className={`flex-1 px-3 py-2 bg-slate-950 border text-sm text-white font-medium rounded focus:outline-none transition min-w-0 ${
                      isSqOverflow ? 'border-red-500 shadow-[0_0_12px_rgba(239,68,68,0.2)]' : 'border-slate-700 focus:border-amber-500'
                    }`}
                  />
                  {/* Quick Diacritic Insert Buttons */}
                  <div className="flex items-center gap-1 shrink-0">
                    {['ë', 'ç', 'Ë', 'Ç'].map((char) => (
                      <button
                        key={char}
                        onClick={() => onUpdateString(activeItem.id, activeItem.sq + char)}
                        className="w-8 h-9 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded font-mono font-bold text-sm text-amber-400 transition"
                        title={`Insert ${char}`}
                      >
                        {char}
                      </button>
                    ))}
                    {onResetSingleString && (
                      <button
                        onClick={() => onResetSingleString(activeItem.id)}
                        className="px-2.5 h-9 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded font-medium text-xs text-slate-300 hover:text-white flex items-center gap-1 ml-1 transition"
                        title="Reset this string to baseline translation"
                      >
                        <span>↺</span>
                      </button>
                    )}
                  </div>
                </div>
              </div>
            </div>

            {/* Visual Bounding Box Simulator */}
            <div className="bg-slate-900 border border-slate-800 rounded-lg p-4 md:p-5 space-y-4 min-w-0">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400">
                  Hardware Text Bounding Box Simulation (MMI 800x480 Cluster)
                </h3>
                <div className="flex items-center gap-3 flex-wrap">
                  <select
                    value={fontFamily}
                    onChange={(e) => setFontFamily(e.target.value as any)}
                    className="px-2 py-1 bg-slate-950 border border-slate-700 rounded text-xs text-slate-200"
                  >
                    <option value="AudiType-Normal">AudiType Normal</option>
                    <option value="AudiType-Bold">AudiType Bold</option>
                    <option value="AudiType-Extended">AudiType Extended</option>
                  </select>

                  <div className="flex items-center gap-1.5 text-xs text-slate-400 font-mono">
                    <span>Size:</span>
                    <input
                      type="range"
                      min="12"
                      max="24"
                      value={fontSize}
                      onChange={(e) => setFontSize(Number(e.target.value))}
                      className="w-20 accent-amber-500"
                    />
                    <span>{fontSize}px</span>
                  </div>
                </div>
              </div>

              {/* Display Simulation Stage - with dedicated scroll container to prevent page overflow */}
              <div className="w-full overflow-x-auto p-6 bg-[#0c0e12] border border-slate-800/80 rounded-lg flex flex-col items-center justify-center relative min-w-0">
                <div className="text-[11px] text-slate-500 mb-2 font-mono">
                  Hardware UI Slot: {activeItem.maxPixels} px
                </div>

                {/* Target Hardware Bounding Box */}
                <div className="max-w-full overflow-x-auto py-2 px-4 flex justify-center">
                  <div
                    className={`relative border-2 ${
                      isSqOverflow
                        ? 'border-red-500 bg-red-950/20 shadow-[0_0_24px_rgba(239,68,68,0.25)]'
                        : 'border-emerald-500/80 bg-emerald-950/10'
                    } rounded p-2 flex items-center justify-center transition-all shrink-0`}
                    style={{ width: `${Math.min(Math.max(activeItem.maxPixels, 80), 1200)}px`, height: '48px' }}
                  >
                    <span
                      className="whitespace-nowrap select-none text-slate-100 font-semibold"
                      style={{
                        fontSize: `${fontSize}px`,
                        fontFamily: fontFamily.includes('Extended') ? 'sans-serif' : 'sans-serif',
                        fontWeight: fontFamily.includes('Bold') ? 700 : 500,
                      }}
                    >
                      {activeItem.sq}
                    </span>

                    {/* Absolute Pixel Guides */}
                    <div className="absolute -top-3 right-0 text-[9px] font-mono text-slate-400">
                      Max: {activeItem.maxPixels}px
                    </div>
                  </div>
                </div>

                {isSqOverflow && (
                  <div className="mt-4 p-3 bg-red-950/90 border border-red-800 rounded text-xs text-red-200 max-w-lg text-center">
                    <strong>Hardware Clipping Hazard:</strong> The Albanian translation is{' '}
                    <span className="font-mono font-bold text-red-300">+{sqOverflowAmount}px</span> wider than the
                    display box allocation. On the physical Audi MMI 3G display, characters beyond this limit will be
                    clipped or truncated.
                  </div>
                )}
              </div>
            </div>
          </div>
        ) : (
          <div className="flex-1 flex items-center justify-center text-slate-500 text-sm">
            Select a string from the left panel to inspect and translate.
          </div>
        )}
      </div>
    </div>
  );
};
