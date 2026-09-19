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
  const [selectedByte, setSelectedByte] = useState<{ row: number; col: number; val: string } | null>({
    row: 0,
    col: 0,
    val: hexData.rows[0]?.hexBytes[0] || '4D',
  });
  const [activeLabTab, setActiveLabTab] = useState<'hex' | 'qnx_terminal'>('hex');
  const [qnxCommand, setQnxCommand] = useState<string>('');
  const [terminalHistory, setTerminalHistory] = useState<Array<{ cmd: string; output: string[] }>>([
    {
      cmd: 'uname -a',
      output: [
        'QNX localhost 6.3.2 2008/05/21-14:32:15EDT SH-4 big endian',
        'Hardware: Harman/Becker MU9411 Automotive Infotainment Platform (Renesas SH7785)',
      ],
    },
    {
      cmd: 'df -h',
      output: [
        'Filesystem             Size  Used Avail Use% Mounted on',
        '/dev/fs0 (NOR IFS)      48M  1.8M   46M   4% /',
        '/dev/fs1 (EFS System)   40M  2.1M   37M   5% /mnt/efs',
        '/dev/hd0t77 (HDD Nav)   32G   18G   14G  56% /mnt/nav',
        '/fs/sda0 (SD Card 1)    32G  1.9M   31G   1% /fs/sda0',
      ],
    },
  ]);

  const handleExecuteQnxCommand = (rawCmd: string) => {
    const cmd = rawCmd.trim();
    if (!cmd) return;

    if (cmd === 'clear') {
      setTerminalHistory([]);
      setQnxCommand('');
      return;
    }

    let output: string[] = [];
    const lower = cmd.toLowerCase();

    if (lower === 'uname -a' || lower === 'uname') {
      output = [
        'QNX localhost 6.3.2 2008/05/21-14:32:15EDT SH-4 big endian',
        'Host: MU9411-HB | CPU: Renesas SH7785 @ 600 MHz | RAM: 512 MB DDR2',
      ];
    } else if (lower.startsWith('df')) {
      output = [
        'Filesystem             Size  Used Avail Use% Mounted on',
        '/dev/fs0 (NOR IFS)      48M  1.8M   46M   4% /',
        '/dev/fs1 (EFS System)   40M  2.1M   37M   5% /mnt/efs',
        '/dev/hd0t77 (HDD Nav)   32G   18G   14G  56% /mnt/nav',
        '/fs/sda0 (SD Card 1)    32G  1.9M   31G   1% /fs/sda0',
      ];
    } else if (lower.startsWith('pidin')) {
      output = [
        '     pid name               tid prio STATE       code  data        stack',
        '       1 proc/boot/procnto    1  10f RUN            0     0    4096(512K)',
        '       2 sbin/devf-ram        1  10r RECEIVE      64K  128K    4096(16K)',
        '       3 io-pkt-v4-hc         1  12r RECEIVE     512K  256K    8192(32K)',
        '       4 usr/bin/lsd.jxe      1  14r RECEIVE     1.8M  2.4M   16384(64K)',
        '       5 usr/bin/mmi-screen   1  15r RECEIVE     840K  1.2M    8192(32K)',
        '       6 bin/sh               1  10r RECEIVE     128K   64K    4096(16K)',
      ];
    } else if (lower.includes('copie_scr.sh')) {
      output = [
        'Executing /fs/sda0/copie_scr.sh through proc_scriptlauncher...',
        '[SYSLOG] Mount: /fs/sda0 (FAT32, cluster=32KB) — OK',
        '[SYSLOG] Parsing /fs/sda0/metainfo2.txt release=2026_ECE train=HN+R_EU_AU_K0942_4',
        '[SYSLOG] NOR flash unlocked at /dev/fs0 (48 MB partition)',
        '[SYSLOG] Flashing /fs/sda0/MU9411/ifs-root.ifs (1.84 MB) -> /dev/fs0 — CRC32 VALIDATED',
        '[SYSLOG] Flashing /fs/sda0/MU9411/efs-system.efs (2.12 MB) -> /dev/fs1 — CRC32 VALIDATED',
        '[SYSLOG] Syncing cartography to /mnt/nav/HBNavDB/nav_data.db (FLDB 544-byte pages)',
        '[SYSLOG] Triggering /fs/sda0/finalScript — All flash operations completed cleanly.',
        'Status: SUCCESS — READY FOR REBOOT (§14.9 Pre-flight simulation)',
      ];
    } else if (lower.includes('metainfo2.txt')) {
      output = [
        '[common]',
        'release = "2026_ECE"',
        'train = "HN+R_EU_AU_K0942_4"',
        'vendor = "Harman/Becker"',
        'variant = "MU9411"',
        '[MU9411_ifs_root]',
        'path = "MU9411/ifs-root.ifs"',
        'size = 1932400',
        'CheckSum.1 = "0x4a8f912c"',
      ];
    } else if (lower.startsWith('ls')) {
      output = [
        'total 36',
        '-rwxr-xr-x  1 root  root     498 Sep 19 19:15 metainfo2.txt',
        '-rwxr-xr-x  1 root  root     360 Sep 19 19:15 copie_scr.sh',
        '-rwxr-xr-x  1 root  root     355 Sep 19 19:15 stock_recovery.sh',
        '-rwxr-xr-x  1 root  root     152 Sep 19 19:15 finalScript',
        'drwxr-xr-x  2 root  root    4096 Sep 19 19:15 MU9411',
        'drwxr-xr-x  2 root  root    4096 Sep 19 19:15 HBNavDB',
        'drwxr-xr-x  2 root  root    4096 Sep 19 19:15 MapStyles',
      ];
    } else if (lower.startsWith('flashctl')) {
      output = [
        'flashctl: Device /dev/fs0 successfully unlocked.',
        'flashctl: Erased 768 sectors (64 KB/sector) — 0 bad blocks found.',
        'flashctl: Ready for streaming write.',
      ];
    } else if (lower === 'help') {
      output = [
        'Audi MMI 3G+ QNX Neutrino 6.3.2 Diagnostics Shell Emulator',
        'Available simulated commands:',
        '  • uname -a                   - Print operating system & CPU architecture',
        '  • df -h                      - Display partition capacities and mount points',
        '  • pidin                      - List QNX running processes & thread priorities',
        '  • ls -la /fs/sda0            - List SD Card insertion contents',
        '  • sh /fs/sda0/copie_scr.sh   - Dry-run the SD card flash launcher script',
        '  • cat /fs/sda0/metainfo2.txt - Inspect SWDL release manifest',
        '  • flashctl -p /dev/fs0 -e -v - Simulate NOR flash unlock & erase test',
        '  • clear                      - Clear terminal history',
      ];
    } else {
      output = [`${cmd}: command not found. Type 'help' for available diagnostic tools.`];
    }

    setTerminalHistory((prev) => [...prev, { cmd, output }]);
    setQnxCommand('');
  };

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
      signed: val > 127 ? val - 256 : val,
    };
  };

  const parsedInfo = selectedByte ? parseByteVal(selectedByte.val) : null;

  return (
    <div className="flex flex-col h-full w-full bg-slate-950 text-slate-100 p-4 md:p-6 font-sans overflow-hidden min-w-0">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-slate-800 shrink-0">
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-xl font-bold tracking-tight text-white flex items-center gap-2">
              <span>⚡</span>
              <span>Binary RE Lab & Hex Inspector</span>
            </h1>
            <span className="px-2 py-0.5 text-xs bg-slate-800 text-slate-300 font-mono rounded">
              QNX Bytecode
            </span>
          </div>
          <p className="text-xs text-slate-400 mt-0.5">
            Byte-level dissection, virtualized dump, and Shannon entropy analysis for MMI firmware binaries
          </p>
        </div>

        <div className="flex items-center gap-3 shrink-0 flex-wrap">
          {/* Sub-tab Switcher */}
          <div className="flex bg-slate-900 border border-slate-800 rounded-lg p-0.5 text-xs font-medium">
            <button
              type="button"
              onClick={() => setActiveLabTab('hex')}
              className={`px-3 py-1.5 rounded-md transition cursor-pointer ${
                activeLabTab === 'hex'
                  ? 'bg-amber-500 text-slate-950 font-bold shadow'
                  : 'text-slate-400 hover:text-white'
              }`}
            >
              🔬 Byte Matrix & Entropy
            </button>
            <button
              type="button"
              onClick={() => setActiveLabTab('qnx_terminal')}
              className={`px-3 py-1.5 rounded-md transition cursor-pointer flex items-center gap-1.5 ${
                activeLabTab === 'qnx_terminal'
                  ? 'bg-amber-500 text-slate-950 font-bold shadow'
                  : 'text-slate-400 hover:text-white'
              }`}
            >
              <span>🖥️</span>
              <span>Virtual QNX Shell</span>
              <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
            </button>
          </div>

          {activeLabTab === 'hex' && (
            <div className="hidden lg:flex items-center gap-2">
              <span className="text-xs text-slate-400 font-mono bg-slate-900 border border-slate-800 px-2.5 py-1 rounded">
                Offset: <strong className="text-amber-400">0x{hexData.offset.toString(16).padStart(8, '0')}</strong>
              </span>
              <span
                className={`px-2.5 py-1 text-xs rounded border font-mono font-bold ${getClassificationBadge(
                  entropyData.classification
                )}`}
              >
                {entropyData.classification} ({entropyData.averageEntropy.toFixed(2)} bits/byte)
              </span>
            </div>
          )}
        </div>
      </div>

      {/* VIEW A: Virtual QNX Neutrino 6.3.2 Terminal Simulator */}
      {activeLabTab === 'qnx_terminal' && (
        <div className="flex flex-col flex-1 mt-4 bg-[#05080f] border border-slate-800 rounded-xl overflow-hidden shadow-2xl min-w-0">
          {/* Terminal Title Bar */}
          <div className="flex items-center justify-between px-4 py-2 bg-[#090e18] border-b border-slate-800 text-xs font-mono text-slate-400">
            <div className="flex items-center gap-2">
              <div className="flex gap-1.5">
                <span className="w-2.5 h-2.5 rounded-full bg-red-500/80" />
                <span className="w-2.5 h-2.5 rounded-full bg-amber-500/80" />
                <span className="w-2.5 h-2.5 rounded-full bg-emerald-500/80" />
              </div>
              <span className="text-slate-300 font-bold">qnx-sh4@mu9411-automotive: ~# (QNX Neutrino 6.3.2)</span>
            </div>
            <span className="text-amber-400/90 text-[11px]">UART Serial Bridge: 115200 8N1</span>
          </div>

          {/* Quick Action Diagnostic Chips */}
          <div className="flex items-center gap-2 px-4 py-2 bg-[#060912] border-b border-slate-800/80 overflow-x-auto text-[11px] font-mono shrink-0">
            <span className="text-slate-500 font-semibold uppercase tracking-wider text-[10px] shrink-0">Quick Commands:</span>
            {[
              { label: 'uname -a', cmd: 'uname -a' },
              { label: 'df -h (Storage)', cmd: 'df -h' },
              { label: 'pidin (Processes)', cmd: 'pidin' },
              { label: 'ls -la /fs/sda0', cmd: 'ls -la /fs/sda0' },
              { label: 'sh /fs/sda0/copie_scr.sh', cmd: 'sh /fs/sda0/copie_scr.sh' },
              { label: 'cat metainfo2.txt', cmd: 'cat /fs/sda0/metainfo2.txt' },
              { label: 'flashctl unlock test', cmd: 'flashctl -p /dev/fs0 -e -v' },
              { label: 'help', cmd: 'help' },
              { label: 'clear', cmd: 'clear' },
            ].map((chip) => (
              <button
                key={chip.cmd}
                type="button"
                onClick={() => handleExecuteQnxCommand(chip.cmd)}
                className="px-2 py-0.5 rounded bg-slate-900 hover:bg-slate-800 border border-slate-800 hover:border-amber-500/50 text-slate-300 hover:text-white transition shrink-0 cursor-pointer"
              >
                {chip.label}
              </button>
            ))}
          </div>

          {/* Terminal Console Output Scroll Area */}
          <div className="flex-1 p-4 overflow-y-auto font-mono text-xs text-slate-300 space-y-3 select-text">
            <div className="text-slate-500 pb-2 border-b border-slate-800/50">
              QNX Neutrino RTOS 6.3.2 Service Pack 3 (SH-4 Big-Endian / MU9411 Infotainment MainUnit)<br />
              Connected to virtual QNX debug console. Type <span className="text-amber-400 font-bold">'help'</span> or click chips above to execute diagnostic scripts.
            </div>

            {terminalHistory.map((item, idx) => (
              <div key={idx} className="space-y-1">
                <div className="flex items-center gap-2 text-emerald-400 font-bold">
                  <span className="text-amber-400">mu9411-qnx#</span>
                  <span>{item.cmd}</span>
                </div>
                <div className="pl-4 text-slate-300 space-y-0.5 leading-relaxed">
                  {item.output.map((line, lIdx) => (
                    <div
                      key={lIdx}
                      className={
                        line.includes('SUCCESS') || line.includes('VALIDATED') || line.includes('OK')
                          ? 'text-emerald-400 font-semibold'
                          : line.includes('SYSLOG') || line.includes('Executing')
                          ? 'text-amber-400'
                          : line.includes('error') || line.includes('not found')
                          ? 'text-red-400'
                          : ''
                      }
                    >
                      {line}
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>

          {/* Command Prompt Input Bar */}
          <form
            onSubmit={(e) => {
              e.preventDefault();
              handleExecuteQnxCommand(qnxCommand);
            }}
            className="flex items-center gap-2 p-3 bg-[#090e18] border-t border-slate-800 font-mono text-xs shrink-0"
          >
            <span className="text-emerald-400 font-bold shrink-0">mu9411-qnx#</span>
            <input
              type="text"
              value={qnxCommand}
              onChange={(e) => setQnxCommand(e.target.value)}
              placeholder="e.g. sh /fs/sda0/copie_scr.sh, df -h, pidin, help..."
              className="flex-1 bg-black/60 border border-slate-700/80 rounded px-3 py-1.5 text-white font-mono text-xs focus:outline-none focus:border-amber-500"
            />
            <button
              type="submit"
              className="px-4 py-1.5 bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold text-xs rounded transition cursor-pointer shrink-0"
            >
              Run
            </button>
          </form>
        </div>
      )}

      {/* VIEW B: Byte Matrix & Hex Inspector */}
      {activeLabTab === 'hex' && (
        <>
          {/* Entropy Gauge Bar */}
          <div className="mt-4 bg-slate-900/80 border border-slate-800 rounded-xl p-3.5 shrink-0">
            <div className="flex justify-between text-xs text-slate-400 mb-1.5 font-mono">
              <span className="font-semibold text-slate-300 flex items-center gap-1.5">
                <span>📊</span> Shannon Entropy Distribution Profile
              </span>
              <span className="font-bold text-amber-400">{entropyData.averageEntropy.toFixed(3)} / 8.000 bits/byte</span>
            </div>
            <div className="w-full bg-slate-950 rounded-full h-2.5 overflow-hidden border border-slate-800 shadow-inner">
              <div
                className={`h-full ${getEntropyColor(entropyData.averageEntropy)} transition-all duration-500 shadow-[0_0_10px_currentColor]`}
                style={{ width: `${(entropyData.averageEntropy / 8.0) * 100}%` }}
              />
            </div>
          </div>

          {/* Main Grid: Hex table + Byte Inspector */}
          <div className="flex flex-1 gap-5 mt-4 overflow-hidden min-w-0">
            {/* Hex Table Viewport */}
            <div className="flex-1 bg-slate-900/90 border border-slate-800 rounded-xl p-4 overflow-auto font-mono text-xs shadow-inner min-w-0">
          <div className="grid grid-cols-[80px_repeat(16,28px)_180px] gap-x-2 pb-2 mb-2 border-b border-slate-800 text-slate-500 font-semibold select-none min-w-[720px]">
            <div>Offset</div>
            {Array.from({ length: 16 }).map((_, i) => (
              <div key={i} className="text-center">
                {i.toString(16).toUpperCase().padStart(2, '0')}
              </div>
            ))}
            <div className="pl-4">ASCII Decode</div>
          </div>

          <div className="space-y-1 min-w-[720px]">
            {hexData.rows.map((row, rowIdx) => (
              <div
                key={row.offset}
                className="grid grid-cols-[80px_repeat(16,28px)_180px] gap-x-2 py-1 hover:bg-slate-800/40 rounded transition-colors"
              >
                <div className="text-amber-500/90 select-none font-bold">
                  0x{row.offset.toString(16).padStart(8, '0')}
                </div>
                {row.hexBytes.map((byte, colIdx) => {
                  const isSelected = selectedByte?.row === rowIdx && selectedByte?.col === colIdx;
                  return (
                    <div
                      key={colIdx}
                      onClick={() => setSelectedByte({ row: rowIdx, col: colIdx, val: byte })}
                      className={`text-center cursor-pointer rounded transition-all select-none ${
                        isSelected
                          ? 'bg-amber-500 text-slate-950 font-bold scale-110 shadow-md ring-2 ring-amber-300'
                          : byte === '00'
                          ? 'text-slate-600'
                          : 'text-slate-200 hover:bg-slate-800 hover:text-white'
                      }`}
                    >
                      {byte}
                    </div>
                  );
                })}
                <div className="pl-4 text-slate-400 select-none tracking-widest font-mono">
                  {row.ascii}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Byte Inspector Panel */}
        <div className="w-72 xl:w-80 bg-slate-900/90 border border-slate-800 rounded-xl p-5 flex flex-col justify-between shrink-0 shadow-lg">
          <div>
            <div className="flex items-center justify-between pb-3 border-b border-slate-800">
              <h2 className="text-xs font-bold uppercase tracking-wider text-slate-300">
                Byte Inspector
              </h2>
              {selectedByte && (
                <span className="text-[11px] font-mono text-amber-400 font-bold">
                  Row {selectedByte.row} · Col {selectedByte.col}
                </span>
              )}
            </div>

            {parsedInfo ? (
              <div className="mt-4 space-y-2.5 text-xs">
                <div className="flex justify-between items-center py-1.5 border-b border-slate-800/80">
                  <span className="text-slate-400">Hex Value:</span>
                  <span className="font-mono text-amber-400 font-bold text-sm">{parsedInfo.hex}</span>
                </div>
                <div className="flex justify-between items-center py-1.5 border-b border-slate-800/80">
                  <span className="text-slate-400">Decimal (Unsigned):</span>
                  <span className="font-mono text-white font-bold">{parsedInfo.dec}</span>
                </div>
                <div className="flex justify-between items-center py-1.5 border-b border-slate-800/80">
                  <span className="text-slate-400">Binary (8-bit):</span>
                  <span className="font-mono text-slate-300 text-[11px] tracking-wider">{parsedInfo.bin}</span>
                </div>
                <div className="flex justify-between items-center py-1.5 border-b border-slate-800/80">
                  <span className="text-slate-400">ASCII Character:</span>
                  <span className="font-mono text-emerald-400 font-black text-base">{parsedInfo.char}</span>
                </div>
                <div className="flex justify-between items-center py-1.5 border-b border-slate-800/80">
                  <span className="text-slate-400">Signed Int8:</span>
                  <span className="font-mono text-slate-300">{parsedInfo.signed}</span>
                </div>
              </div>
            ) : (
              <div className="text-xs text-slate-500 mt-4 italic">
                Click any byte in the hex matrix to inspect numeric representations.
              </div>
            )}
          </div>

          <div className="pt-4 border-t border-slate-800 text-[10px] text-slate-500 font-mono">
            Read-only memory view · §14.9 Safety locked
          </div>
        </div>
      </div>
    </>
  )}
</div>
);
};
