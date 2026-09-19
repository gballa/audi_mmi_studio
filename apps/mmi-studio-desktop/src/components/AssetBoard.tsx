import React, { useState } from 'react';
import { InspectResult } from '../types';

interface AssetBoardProps {
  assets: InspectResult[];
  onSelectAsset?: (asset: InspectResult) => void;
}

export const AssetBoard: React.FC<AssetBoardProps> = ({ assets, onSelectAsset }) => {
  const [filter, setFilter] = useState('');
  const [selectedAsset, setSelectedAsset] = useState<InspectResult | null>(assets[0] || null);

  const filteredAssets = assets.filter(
    (a) =>
      a.filePath.toLowerCase().includes(filter.toLowerCase()) ||
      a.detectedFormat.toLowerCase().includes(filter.toLowerCase())
  );

  const handleCardClick = (asset: InspectResult) => {
    setSelectedAsset(asset);
    if (onSelectAsset) onSelectAsset(asset);
  };

  const formatBytes = (bytes: number): string => {
    if (bytes >= 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
    if (bytes >= 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${bytes} B`;
  };

  return (
    <div className="flex flex-col h-full w-full bg-slate-950 text-slate-100 p-4 md:p-6 overflow-hidden font-sans min-w-0">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-slate-800 shrink-0">
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-xl font-bold tracking-tight text-white">MMI Asset Census & File Inventory</h1>
            <span className="px-2 py-0.5 text-xs bg-slate-800 text-slate-300 font-mono rounded">
              {filteredAssets.length} Files
            </span>
          </div>
          <p className="text-xs text-slate-400 mt-0.5">
            Harman Precomp containers, linotype fonts, navigation databases, and binary payloads
          </p>
        </div>

        <div className="flex items-center gap-3">
          <input
            type="text"
            placeholder="Filter assets by path or format..."
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="px-3 py-1.5 bg-slate-900 border border-slate-700 rounded-lg text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-amber-500 w-64"
          />
        </div>
      </div>

      {/* Main Grid Stage */}
      <div className="flex flex-1 gap-6 mt-4 overflow-hidden min-w-0">
        {/* Assets Cards Grid */}
        <div className="flex-1 overflow-y-auto pr-2 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3.5 content-start">
          {filteredAssets.map((asset) => {
            const isSelected = selectedAsset?.filePath === asset.filePath;
            const filename = asset.filePath.split('/').pop() || asset.filePath;
            const folder = asset.filePath.substring(0, asset.filePath.lastIndexOf('/'));

            return (
              <div
                key={asset.filePath}
                onClick={() => handleCardClick(asset)}
                className={`p-4 rounded-xl border cursor-pointer transition-all flex flex-col justify-between ${
                  isSelected
                    ? 'bg-amber-500/10 border-amber-500 shadow-md ring-1 ring-amber-500/30'
                    : 'bg-slate-900/70 border-slate-800 hover:border-slate-700'
                }`}
              >
                <div className="flex items-start gap-3">
                  <div className="w-12 h-12 bg-slate-950 border border-slate-800 rounded-lg flex items-center justify-center overflow-hidden shrink-0 shadow-inner">
                    {asset.thumbnailBlobId ? (
                      <span className="text-xl">🖼️</span>
                    ) : asset.detectedFormat.includes('Nav') ? (
                      <span className="text-xl">🗺️</span>
                    ) : asset.detectedFormat.includes('Font') ? (
                      <span className="text-xl">🔤</span>
                    ) : (
                      <span className="text-xl">📦</span>
                    )}
                  </div>

                  <div className="flex-1 min-w-0">
                    <div className="font-bold text-xs text-white truncate" title={filename}>
                      {filename}
                    </div>
                    <div className="text-[10px] text-slate-400 font-mono truncate" title={folder}>
                      {folder || '/'}
                    </div>
                    <div className="mt-1 flex items-center gap-1.5">
                      <span className="px-1.5 py-0.2 rounded text-[9px] font-mono bg-slate-950 border border-slate-800 text-sky-400 font-semibold">
                        {asset.detectedFormat}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="mt-3 pt-2 border-t border-slate-800/80 flex items-center justify-between text-[11px] font-mono text-slate-400">
                  <span>{formatBytes(asset.sizeBytes)}</span>
                  <span className="text-[10px] text-slate-500 truncate max-w-[120px]" title={asset.blake3Hash}>
                    {asset.blake3Hash.substring(0, 10)}...
                  </span>
                </div>
              </div>
            );
          })}
        </div>

        {/* Selected Asset Detail Inspector */}
        {selectedAsset && (
          <div className="w-80 xl:w-96 bg-slate-900/90 border border-slate-800 rounded-xl p-5 shrink-0 flex flex-col justify-between overflow-y-auto space-y-4">
            <div>
              <div className="flex items-center justify-between border-b border-slate-800 pb-3 mb-3">
                <span className="text-xs font-bold uppercase tracking-wider text-amber-400">Asset Inspector</span>
                <span className="px-2 py-0.5 text-[10px] font-mono rounded bg-slate-950 border border-slate-800 text-slate-300">
                  {selectedAsset.detectedFormat}
                </span>
              </div>

              <div className="space-y-3 text-xs">
                <div>
                  <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">File Path</span>
                  <span className="font-mono text-xs text-white break-all block mt-0.5 bg-slate-950 p-2 rounded border border-slate-800">
                    {selectedAsset.filePath}
                  </span>
                </div>

                <div>
                  <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">Size in Bytes</span>
                  <span className="font-mono text-slate-200 text-xs">
                    {selectedAsset.sizeBytes.toLocaleString()} bytes ({formatBytes(selectedAsset.sizeBytes)})
                  </span>
                </div>

                <div>
                  <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">Blake3 Cryptographic Hash</span>
                  <span className="font-mono text-[10px] text-slate-400 break-all block mt-0.5 bg-slate-950 p-2 rounded border border-slate-800">
                    {selectedAsset.blake3Hash}
                  </span>
                </div>

                <div>
                  <span className="block text-[10px] font-bold text-slate-400 uppercase tracking-wider">Hardware Partition</span>
                  <span className="text-slate-300 text-xs mt-0.5 block">
                    {selectedAsset.filePath.startsWith('HBNavDB')
                      ? 'HDD Navigation Volume (/dev/hd0t177)'
                      : selectedAsset.filePath.startsWith('Fonts')
                      ? 'QNX EFS System Partition (/mnt/efs-system/usr/fonts)'
                      : 'QNX IFS Root Partition (/mnt/ifs-root)'}
                  </span>
                </div>
              </div>
            </div>

            <div className="pt-3 border-t border-slate-800 text-[10px] text-slate-500 font-mono">
              Immutable Original File Hash: §14.9 Cryptographically Anchored
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
