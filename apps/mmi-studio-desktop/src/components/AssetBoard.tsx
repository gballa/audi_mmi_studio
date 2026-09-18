import React, { useState } from 'react';
import { InspectResult } from '../types';

interface AssetBoardProps {
  assets: InspectResult[];
  onSelectAsset: (asset: InspectResult) => void;
}

export const AssetBoard: React.FC<AssetBoardProps> = ({ assets, onSelectAsset }) => {
  const [filter, setFilter] = useState('');

  const filteredAssets = assets.filter((a) =>
    a.filePath.toLowerCase().includes(filter.toLowerCase()) ||
    a.detectedFormat.toLowerCase().includes(filter.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 p-4">
      <div className="flex items-center justify-between pb-4 border-b border-slate-800">
        <div>
          <h1 className="text-xl font-bold tracking-tight">Asset Board</h1>
          <p className="text-xs text-slate-400">Harman Precomp & Resource Explorer</p>
        </div>
        <input
          type="text"
          placeholder="Filter assets by path or format..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          className="px-3 py-1.5 bg-slate-900 border border-slate-700 rounded text-sm focus:outline-none focus:border-amber-500"
        />
      </div>

      <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-3 mt-4 overflow-y-auto flex-1">
        {filteredAssets.map((asset) => (
          <div
            key={asset.filePath}
            onClick={() => onSelectAsset(asset)}
            className="flex flex-col items-center p-2 rounded bg-slate-900 border border-slate-800 hover:border-amber-500 cursor-pointer transition-all hover:scale-105"
          >
            <div className="w-16 h-16 bg-slate-950 border border-slate-800 rounded flex items-center justify-center overflow-hidden mb-2">
              {asset.thumbnailBlobId ? (
                <div className="text-xs font-mono text-amber-400">THUMB</div>
              ) : (
                <div className="text-[10px] text-slate-600 font-mono">BIN</div>
              )}
            </div>
            <div className="text-[11px] font-medium text-slate-200 truncate w-full text-center">
              {asset.filePath.split('/').pop()}
            </div>
            <div className="text-[9px] text-slate-500 truncate w-full text-center">
              {asset.detectedFormat}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
