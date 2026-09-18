import React, { useState } from 'react';
import { HexDumpResult, EntropyResult } from '../types';

interface HexViewerProps {
  hexData?: HexDumpResult;
  entropyData?: EntropyResult;
  onOffsetChange?: (newOffset: number) => void;
}

export const HexViewer: React.FC<HexViewerProps> = ({
  hexData = {
    offset: 0,
    length: 64,
    rows: [
      { offset: 0, hexBytes: ['4d', '4d', '49', '33', '47', '5f', '50', '4c', '55', '53', '00', '00', '01', '00', '00', '00'], ascii: 'MMI3G_PLUS......' },
      { offset: 16, hexBytes: ['50', '52', '45', '43', '4f', '4d', '50', '00', '10', '00', '00', '00', 'ff', 'ff', '00', '00'], ascii: 'PRECOMP.........' },
      { offset: 32, hexBytes: ['78', '9c', '63', '60', '60', '60', '04', '62', '10', '40', '00', '01', '00', '00', '00', 'ff'], ascii: 'x.c```.b.@......' },
      { offset: 48, hexBytes: ['00', '12', '34', '56', '78', '9a', 'bc', 'de', 'f0', '12', '34', '56', '78', '9a', 'bc', 'de'], ascii: '..4Vx.....4Vx...' },
    ],
  },
  entropyData = {
    averageEntropy: 4.82,
    classification: 'CompressedOrEncrypted',
    segments: [[0, 4.82]],
  },
}) => {
  const [selectedByte, setSelectedByte] = useState<{ row: number; col: number; val: string } | null>(null);

  const getEntropyColor = (entropy: number) => {
    if (entropy < 3.0) return 'bg-blue-500';
    if (entropy < 6.0) return 'bg-emerald-500';
    if (entropy < 7.2) return 'bg-amber-500';
    return 'bg-red-500';
  };

  const getClassificationBadge = (classification: string) => {
    switch (classification) {
      case 'Plaintext':
        return 'bg-blue-950 text-blue-300 border-blue-800';
      case 'StructuredOrCode':
        return 'bg-emerald-950 text-emerald-300 border-emerald-800';
      case 'CompressedOrEncrypted':
      case 'Compressed':
        return 'bg-amber-950 text-amber-300 border-amber-800';
      case 'EncryptedOrRandom':
      case 'HighEntropy':
        return 'bg-red-950 text-red-300 border-red-800';
      default:
        return 'bg-slate-800 text-slate-300 border-slate-700';
    }
  };

  const parseByteVal = (valStr: string) => {
    const val = parseInt(valStr, 16);
    if (isNaN(val)) return null;
    return {
      hex: `0x${valStr.toUpperCase()}`,
      dec: val,
      bin: val.toString(2).padStart(8, '0'),
      char: val >= 32 && val <= 126 ? String.fromCharCode(val) : '·',
    };
  };

  const parsedInfo = selectedByte ? parseByteVal(selectedByte.val) : null;

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 p-4 font-sans">
      {/* Header */}
      <div className="flex items-center justify-between pb-4 border-b border-slate-800">
        <div>
          <h1 className="text-xl font-bold tracking-tight">Binary RE Lab & Hex Inspector</h1>
          <p className="text-xs text-slate-400">
            Byte-level inspection, virtualized dump, and Shannon entropy analysis
          </p>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-xs text-slate-400 font-mono">
            Offset: 0x{hexData.offset.toString(16).padStart(8, '0')}
          </span>
          <span
            className={`px-2.5 py-0.5 text-xs rounded border font-mono ${getClassificationBadge(
              entropyData.classification
            )}`}
          >
            {entropyData.classification} ({entropyData.averageEntropy.toFixed(2)} bits/byte)
          </span>
        </div>
      </div>

      {/* Entropy Gauge Bar */}
      <div className="mt-4 bg-slate-900 border border-slate-800 rounded p-3">
        <div className="flex justify-between text-xs text-slate-400 mb-1">
          <span>Entropy Profile</span>
          <span>{entropyData.averageEntropy.toFixed(3)} / 8.000</span>
        </div>
        <div className="w-full bg-slate-950 rounded-full h-2.5 overflow-hidden border border-slate-800">
          <div
            className={`h-full ${getEntropyColor(entropyData.averageEntropy)} transition-all duration-300`}
            style={{ width: `${(entropyData.averageEntropy / 8.0) * 100}%` }}
          />
        </div>
      </div>

      {/* Main Grid: Hex table + Byte Inspector */}
      <div className="flex flex-1 gap-4 mt-4 overflow-hidden">
        {/* Hex Table */}
        <div className="flex-1 bg-slate-900 border border-slate-800 rounded p-4 overflow-auto font-mono text-xs">
          <div className="grid grid-cols-[80px_repeat(16,28px)_180px] gap-x-2 pb-2 mb-2 border-b border-slate-800 text-slate-500 font-semibold select-none">
            <div>Offset</div>
            {Array.from({ length: 16 }).map((_, i) => (
              <div key={i} className="text-center">
                {i.toString(16).toUpperCase().padStart(2, '0')}
              </div>
            ))}
            <div className="pl-4">Decoded Text</div>
          </div>

          {hexData.rows.map((row, rowIdx) => (
            <div
              key={row.offset}
              className="grid grid-cols-[80px_repeat(16,28px)_180px] gap-x-2 py-0.5 hover:bg-slate-800/50 rounded"
            >
              <div className="text-amber-500/80 select-none">
                0x{row.offset.toString(16).padStart(8, '0')}
              </div>
              {row.hexBytes.map((byte, colIdx) => {
                const isSelected =
                  selectedByte?.row === rowIdx && selectedByte?.col === colIdx;
                return (
                  <div
                    key={colIdx}
                    onClick={() =>
                      setSelectedByte({ row: rowIdx, col: colIdx, val: byte })
                    }
                    className={`text-center cursor-pointer rounded transition-colors ${
                      isSelected
                        ? 'bg-amber-500 text-black font-bold'
                        : byte === '00'
                        ? 'text-slate-600'
                        : 'text-slate-200 hover:bg-slate-700'
                    }`}
                  >
                    {byte}
                  </div>
                );
              })}
              <div className="pl-4 text-slate-400 select-none tracking-widest">
                {row.ascii}
              </div>
            </div>
          ))}
        </div>

        {/* Byte Inspector Panel */}
        <div className="w-64 bg-slate-900 border border-slate-800 rounded p-4 flex flex-col justify-between">
          <div>
            <h2 className="text-sm font-semibold text-slate-200 pb-2 border-b border-slate-800">
              Byte Inspector
            </h2>
            {parsedInfo ? (
              <div className="mt-3 space-y-2 text-xs">
                <div className="flex justify-between py-1 border-b border-slate-800">
                  <span className="text-slate-400">Hex</span>
                  <span className="font-mono text-amber-400">{parsedInfo.hex}</span>
                </div>
                <div className="flex justify-between py-1 border-b border-slate-800">
                  <span className="text-slate-400">Decimal</span>
                  <span className="font-mono text-slate-200">{parsedInfo.dec}</span>
                </div>
                <div className="flex justify-between py-1 border-b border-slate-800">
                  <span className="text-slate-400">Binary</span>
                  <span className="font-mono text-slate-200">{parsedInfo.bin}</span>
                </div>
                <div className="flex justify-between py-1 border-b border-slate-800">
                  <span className="text-slate-400">ASCII Character</span>
                  <span className="font-mono text-emerald-400 font-bold">{parsedInfo.char}</span>
                </div>
                <div className="flex justify-between py-1 border-b border-slate-800">
                  <span className="text-slate-400">Signed Int8</span>
                  <span className="font-mono text-slate-200">
                    {(parsedInfo.dec > 127 ? parsedInfo.dec - 256 : parsedInfo.dec)}
                  </span>
                </div>
              </div>
            ) : (
              <div className="text-xs text-slate-500 mt-4 italic">
                Click any byte in the hex matrix to inspect numeric representations.
              </div>
            )}
          </div>

          <div className="pt-4 border-t border-slate-800 text-[10px] text-slate-500">
            Read-only memory view · Safety locked [§14.9]
          </div>
        </div>
      </div>
    </div>
  );
};
