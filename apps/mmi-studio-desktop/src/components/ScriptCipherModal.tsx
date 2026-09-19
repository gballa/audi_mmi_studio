import React, { useState, useEffect } from 'react';

export const SEED_INIT = 0x001be3ac;

export function harmanTransform(data: Uint8Array): Uint8Array {
  let seed = SEED_INIT >>> 0;
  function prng(): number {
    const r0 = seed & 0xFF;
    const r1 = ((seed >>> 1) | (seed << 31)) >>> 0;
    const r3 = ((((r1 >>> 16) & 0xFF) + r1) >>> 0);
    const r1_new = (((r3 >>> 8) & 0xFF) << 16) >>> 0;
    const r3_new = (r3 - r1_new) >>> 0;
    seed = r3_new;
    return r0;
  }
  // Discard first PRNG call matching factory Harman/Becker SH-4 implementation
  prng();

  const out = new Uint8Array(data.length);
  for (let i = 0; i < data.length; i++) {
    out[i] = data[i] ^ prng();
  }
  return out;
}

function bytesToHex(bytes: Uint8Array, maxBytes = 128): string {
  const hex: string[] = [];
  const len = Math.min(bytes.length, maxBytes);
  for (let i = 0; i < len; i++) {
    hex.push(bytes[i].toString(16).padStart(2, '0'));
  }
  return hex.join(' ') + (bytes.length > maxBytes ? ` ... (+${bytes.length - maxBytes} more bytes)` : '');
}

interface ScriptCipherModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const ScriptCipherModal: React.FC<ScriptCipherModalProps> = ({ isOpen, onClose }) => {
  const defaultScript = `#!/bin/ksh
# Audi MMI 3G/3G+ SD Shell Script Autorun Launcher
# Automatically executed upon SD insertion by proc_scriptlauncher
export SDPATH="\${1:-\$(dirname \$0)}"
export PATH="\${PATH}:\${SDPATH}/bin"
export SDLIB="\${SDPATH}/lib"
export SDVAR="\${SDPATH}/var"
mount -uw "\$SDPATH" 2>/dev/null
cd "\$SDPATH"
exec ksh ./run.sh "\$SDPATH"
`;

  const [inputScript, setInputScript] = useState<string>(defaultScript);
  const [encodedBytes, setEncodedBytes] = useState<Uint8Array>(new Uint8Array(0));
  const [hexPreview, setHexPreview] = useState<string>('');
  const [activeTab, setActiveTab] = useState<'encode' | 'decode'>('encode');
  const [cipherNotice, setCipherNotice] = useState<string | null>(null);

  // Recalculate transform whenever inputScript changes
  useEffect(() => {
    const encoder = new TextEncoder();
    const data = encoder.encode(inputScript);
    const transformed = harmanTransform(data);
    setEncodedBytes(transformed);
    setHexPreview(bytesToHex(transformed));
  }, [inputScript]);

  if (!isOpen) return null;

  const handleCopyEncoded = () => {
    // Copy hex string to clipboard
    navigator.clipboard.writeText(bytesToHex(encodedBytes, 2048));
    setCipherNotice('Hex stream copied to clipboard!');
    setTimeout(() => setCipherNotice(null), 2500);
  };

  const handleDownloadFile = (filename: string, data: Uint8Array) => {
    const blob = new Blob([data.buffer as ArrayBuffer], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    setCipherNotice(`Downloaded ${filename} successfully!`);
    setTimeout(() => setCipherNotice(null), 2500);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-fade-in font-sans">
      <div className="bg-[#0b0e14] border border-slate-800 rounded-xl max-w-3xl w-full shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-800 bg-[#0d121c]">
          <div className="flex items-center gap-2.5">
            <span className="text-xl">🔐</span>
            <div>
              <h2 className="text-sm font-bold uppercase tracking-wider text-slate-100 flex items-center gap-2">
                <span>Harman PRNG Script Cipher</span>
                <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-amber-500/20 text-amber-300 border border-amber-500/40">
                  Seed: 0x001be3ac
                </span>
              </h2>
              <p className="text-xs text-slate-400 mt-0.5">
                Renesas SH-4 bit-rotation XOR stream cipher for factory QNX <code className="text-amber-300">proc_scriptlauncher</code>
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-200 p-1.5 rounded-lg hover:bg-slate-800 transition"
          >
            ✕
          </button>
        </div>

        {/* Action Tabs & Notice */}
        <div className="px-6 pt-3 flex items-center justify-between border-b border-slate-800/80 bg-slate-950/40">
          <div className="flex gap-2">
            <button
              onClick={() => setActiveTab('encode')}
              className={`px-3 py-1.5 text-xs font-bold border-b-2 transition ${
                activeTab === 'encode'
                  ? 'border-amber-400 text-amber-400'
                  : 'border-transparent text-slate-400 hover:text-slate-300'
              }`}
            >
              Encode Script (Plaintext ➔ copie_scr.sh)
            </button>
            <button
              onClick={() => setActiveTab('decode')}
              className={`px-3 py-1.5 text-xs font-bold border-b-2 transition ${
                activeTab === 'decode'
                  ? 'border-amber-400 text-amber-400'
                  : 'border-transparent text-slate-400 hover:text-slate-300'
              }`}
            >
              Decode Cipher (copie_scr.sh ➔ Plaintext)
            </button>
          </div>
          {cipherNotice && (
            <span className="text-xs text-emerald-400 font-medium animate-pulse">
              ✓ {cipherNotice}
            </span>
          )}
        </div>

        {/* Body */}
        <div className="p-6 space-y-4 overflow-y-auto flex-1 font-mono text-xs">
          {/* Editor Input */}
          <div className="space-y-1.5">
            <div className="flex items-center justify-between text-slate-400 text-[11px]">
              <span>
                {activeTab === 'encode' ? 'Plaintext Shell Script (Bash/Ksh):' : 'Encoded Stream / Plaintext Input:'}
              </span>
              <span>{inputScript.length} characters ({new TextEncoder().encode(inputScript).length} bytes)</span>
            </div>
            <textarea
              value={inputScript}
              onChange={(e) => setInputScript(e.target.value)}
              rows={8}
              className="w-full bg-[#05070a] border border-slate-800 rounded-lg p-3 text-slate-200 font-mono text-xs focus:outline-none focus:border-amber-500/80 resize-y"
              placeholder="#!/bin/sh&#10;echo 'Hello MMI 3G+'..."
            />
          </div>

          {/* Cipher Transformation Preview */}
          <div className="space-y-1.5">
            <div className="flex items-center justify-between text-slate-400 text-[11px]">
              <span className="flex items-center gap-2">
                <span className="w-2 h-2 rounded-full bg-amber-400"></span>
                <span>Harman SH-4 PRNG Cipher Output:</span>
              </span>
              <span className="text-amber-400 font-bold">{encodedBytes.length} Bytes Output</span>
            </div>
            <div className="bg-[#05070a] border border-slate-800 rounded-lg p-3 text-amber-300 text-[11px] leading-relaxed break-all select-all font-mono">
              {hexPreview || '(Empty input)'}
            </div>
          </div>

          {/* Technical Invariant Card */}
          <div className="bg-amber-950/20 border border-amber-800/40 rounded-lg p-3.5 space-y-1.5 text-slate-300 font-sans text-xs">
            <div className="flex items-center gap-2 font-bold text-amber-300">
              <span>⚡</span>
              <span>Symmetric Invertibility Property</span>
            </div>
            <p className="text-slate-400 text-[11px] leading-relaxed">
              Because this cipher applies symmetric XOR stream encryption (<code className="text-amber-300">byte ^ prng()</code>), encoding a plaintext script produces the exact binary needed for factory <code className="text-amber-300">proc_scriptlauncher</code>. Passing that binary back through this tool decrypts it back to 100% original plaintext.
            </p>
          </div>
        </div>

        {/* Footer Actions */}
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 px-6 py-4 border-t border-slate-800 bg-[#0d121c]">
          <div className="flex items-center gap-2">
            <button
              onClick={handleCopyEncoded}
              className="px-3.5 py-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold transition"
            >
              📋 Copy Hex Output
            </button>
            <button
              onClick={() => setInputScript(defaultScript)}
              className="px-3.5 py-1.5 rounded bg-slate-900 border border-slate-700 text-slate-300 hover:bg-slate-800 text-xs transition"
            >
              ↺ Reset Default
            </button>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => handleDownloadFile('copie_scr_plain.sh', new TextEncoder().encode(inputScript))}
              className="px-3.5 py-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold transition"
            >
              💾 Save copie_scr_plain.sh
            </button>
            <button
              onClick={() => handleDownloadFile('copie_scr.sh', encodedBytes)}
              className="px-4 py-1.5 rounded bg-amber-500 hover:bg-amber-400 text-black text-xs font-bold transition shadow-lg shadow-amber-500/20"
            >
              ⚡ Download copie_scr.sh (Encoded)
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
