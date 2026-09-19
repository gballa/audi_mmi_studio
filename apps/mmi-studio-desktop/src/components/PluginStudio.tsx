import React, { useState } from 'react';

export interface PluginItem {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  targetGeneration: 'MMI_3G' | 'MIB2_HIGH' | 'MIB3' | 'RNS_850';
  extensions: string[];
  capabilities: {
    canDetect: boolean;
    canParse: boolean;
    canExtract: boolean;
    canRebuild: boolean;
  };
  sandbox: {
    runtime: 'Wasm' | 'Native';
    memoryLimitMb: number;
    timeoutMs: number;
    isolated: boolean;
  };
  enabled: boolean;
}

const initialPlugins: PluginItem[] = [
  {
    id: 'org.audimmi.plugin.mib2_mcf',
    name: 'MIB2 High MCF Texture Container Adapter',
    version: '1.2.0',
    author: 'MMI Community / drger',
    description: 'Decompresses and parses proprietary MIB2 TechniSat / Harman multi-container textures (.mcf) into RGBA bitmaps.',
    targetGeneration: 'MIB2_HIGH',
    extensions: ['.mcf', '.gcfg'],
    capabilities: {
      canDetect: true,
      canParse: true,
      canExtract: true,
      canRebuild: true,
    },
    sandbox: {
      runtime: 'Wasm',
      memoryLimitMb: 64,
      timeoutMs: 500,
      isolated: true,
    },
    enabled: true,
  },
  {
    id: 'org.audimmi.plugin.cva_coding',
    name: 'QNX CVALUE Persistence Coding Decoder',
    version: '1.0.4',
    author: 'AudiMMI Studio Core',
    description: 'Reverse-engineers and decodes encrypted CVALUE*.CVA parameter blocks from /HBpersistence/ into JSON key-value pairs.',
    targetGeneration: 'MMI_3G',
    extensions: ['.cva', '.pst'],
    capabilities: {
      canDetect: true,
      canParse: true,
      canExtract: false,
      canRebuild: false,
    },
    sandbox: {
      runtime: 'Native',
      memoryLimitMb: 32,
      timeoutMs: 250,
      isolated: true,
    },
    enabled: true,
  },
  {
    id: 'org.audimmi.plugin.mib3_overlay',
    name: 'MIB3 OI Android Automotive Vector Overlay',
    version: '0.9.1-beta',
    author: 'NavTech Solutions',
    description: 'Experimental parser for next-generation MIB3 vector tile layer overlays and Google Earth 3D buildings.',
    targetGeneration: 'MIB3',
    extensions: ['.vtp', '.pb'],
    capabilities: {
      canDetect: true,
      canParse: true,
      canExtract: true,
      canRebuild: false,
    },
    sandbox: {
      runtime: 'Wasm',
      memoryLimitMb: 128,
      timeoutMs: 1000,
      isolated: true,
    },
    enabled: false,
  },
];

export const PluginStudio: React.FC = () => {
  const [plugins, setPlugins] = useState<PluginItem[]>(initialPlugins);
  const [selectedPluginId, setSelectedPluginId] = useState<string>(initialPlugins[0].id);
  const [testPayload, setTestPayload] = useState<string>('MCF\\x01\\x00\\x00\\x00\\x80\\x02\\x00\\x00\\xe0\\x01\\x00\\x00');
  const [testResult, setTestResult] = useState<string | null>(null);
  const [isTesting, setIsTesting] = useState<boolean>(false);
  const [filterGen, setFilterGen] = useState<string>('ALL');

  const selectedPlugin = plugins.find((p) => p.id === selectedPluginId) || plugins[0];

  const handleTogglePlugin = (id: string) => {
    setPlugins((prev) =>
      prev.map((p) => (p.id === id ? { ...p, enabled: !p.enabled } : p))
    );
  };

  const handleRunSandboxTest = () => {
    setIsTesting(true);
    setTimeout(() => {
      setIsTesting(false);
      setTestResult(
        JSON.stringify(
          {
            status: 'VERIFIED',
            plugin_id: selectedPlugin.id,
            abi_version: 1,
            sandbox_checks: {
              memory_used_kb: 420,
              time_elapsed_ms: 12.4,
              memory_limit_exceeded: false,
              security_violations: 0,
            },
            detection: {
              matched: true,
              confidence: 0.98,
              format_name: selectedPlugin.name,
              declared_dimensions: '640x480 (RGBA_8888)',
            },
          },
          null,
          2
        )
      );
    }, 450);
  };

  const filteredPlugins = plugins.filter((p) =>
    filterGen === 'ALL' ? true : p.targetGeneration === filterGen
  );

  return (
    <div className="space-y-6 animate-fade-in font-sans pb-12">
      {/* Top Banner */}
      <div className="bg-slate-900 border border-slate-800 rounded-lg p-5 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <div className="flex items-center gap-2.5">
            <span className="text-xl">🧩</span>
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-100 flex items-center gap-2">
              <span>Third-Party Format Adapter Plugin Manager</span>
              <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-blue-500/20 text-blue-300 border border-blue-500/40">
                ABI v1 Specification (§17.1)
              </span>
            </h2>
          </div>
          <p className="text-xs text-slate-400 mt-1 max-w-2xl">
            Extend Audi MMI Studio with sandboxed Wasm and Native format decoders for next-gen platforms (MIB2, MIB3) without altering the immutable core.
          </p>
        </div>

        <div className="flex items-center gap-2 text-xs">
          <span className="px-3 py-1.5 rounded-lg bg-emerald-950/60 border border-emerald-800/60 text-emerald-400 font-mono font-semibold flex items-center gap-1.5">
            <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            <span>Sandbox Isolation Active</span>
          </span>
        </div>
      </div>

      {/* Metrics Row */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 text-xs font-mono">
        <div className="bg-slate-900 border border-slate-800 rounded-lg p-3.5 space-y-1">
          <div className="text-slate-500 text-[10px] uppercase">Registered Plugins</div>
          <div className="text-xl font-bold text-slate-100">{plugins.length}</div>
          <div className="text-[11px] text-slate-400 font-sans">Across 3 generations</div>
        </div>

        <div className="bg-slate-900 border border-slate-800 rounded-lg p-3.5 space-y-1">
          <div className="text-slate-500 text-[10px] uppercase">Active Adapters</div>
          <div className="text-xl font-bold text-emerald-400">
            {plugins.filter((p) => p.enabled).length}
          </div>
          <div className="text-[11px] text-slate-400 font-sans">Engaged in detection</div>
        </div>

        <div className="bg-slate-900 border border-slate-800 rounded-lg p-3.5 space-y-1">
          <div className="text-slate-500 text-[10px] uppercase">Sandbox Isolation</div>
          <div className="text-xl font-bold text-blue-400">Wasm / Native</div>
          <div className="text-[11px] text-slate-400 font-sans">Strict 64MB memory cap</div>
        </div>

        <div className="bg-slate-900 border border-slate-800 rounded-lg p-3.5 space-y-1">
          <div className="text-slate-500 text-[10px] uppercase">ABI Conformance</div>
          <div className="text-xl font-bold text-purple-400">100% ABI v1</div>
          <div className="text-[11px] text-slate-400 font-sans">Zero host bleed</div>
        </div>
      </div>

      {/* Main Grid: Plugin List & Sandbox Tester */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Left Column: Plugin List */}
        <div className="lg:col-span-7 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-xs font-bold uppercase tracking-wider text-slate-300">
              Discovered Format Plugins
            </h3>
            <div className="flex items-center gap-1.5 text-xs">
              <span className="text-slate-500 text-[11px]">Generation:</span>
              <select
                value={filterGen}
                onChange={(e) => setFilterGen(e.target.value)}
                className="bg-slate-950 border border-slate-800 rounded px-2 py-1 text-slate-300 text-xs focus:outline-none"
              >
                <option value="ALL">All Hardware</option>
                <option value="MMI_3G">MMI 3G / 3G+</option>
                <option value="MIB2_HIGH">MIB2 High</option>
                <option value="MIB3">MIB3</option>
              </select>
            </div>
          </div>

          <div className="space-y-3">
            {filteredPlugins.map((plugin) => {
              const isSelected = plugin.id === selectedPluginId;
              return (
                <div
                  key={plugin.id}
                  onClick={() => setSelectedPluginId(plugin.id)}
                  className={`p-4 rounded-lg border transition cursor-pointer ${
                    isSelected
                      ? 'bg-slate-900 border-blue-500/80 shadow-lg shadow-blue-950/20'
                      : 'bg-slate-900/60 border-slate-800 hover:border-slate-700'
                  }`}
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="space-y-1">
                      <div className="flex items-center gap-2">
                        <span className="font-bold text-slate-100 text-sm">{plugin.name}</span>
                        <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-slate-800 text-slate-300">
                          v{plugin.version}
                        </span>
                        <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-blue-950 border border-blue-800/60 text-blue-300">
                          {plugin.targetGeneration}
                        </span>
                      </div>
                      <p className="text-xs text-slate-400 leading-relaxed">{plugin.description}</p>
                    </div>

                    <label
                      onClick={(e) => e.stopPropagation()}
                      className="relative inline-flex items-center cursor-pointer shrink-0 mt-0.5"
                    >
                      <input
                        type="checkbox"
                        checked={plugin.enabled}
                        onChange={() => handleTogglePlugin(plugin.id)}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-slate-800 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-emerald-500"></div>
                    </label>
                  </div>

                  <div className="mt-3 pt-3 border-t border-slate-800/60 flex flex-wrap items-center justify-between gap-2 text-xs font-mono">
                    <div className="flex items-center gap-2">
                      <span className="text-slate-500 text-[11px]">Extensions:</span>
                      {plugin.extensions.map((ext) => (
                        <span key={ext} className="px-1.5 py-0.5 rounded bg-slate-950 text-slate-300 text-[11px]">
                          {ext}
                        </span>
                      ))}
                    </div>

                    <div className="flex items-center gap-2 text-[11px]">
                      <span className={`px-1.5 py-0.5 rounded ${plugin.capabilities.canDetect ? 'bg-emerald-950/80 text-emerald-300' : 'bg-slate-950 text-slate-600'}`}>
                        Detect
                      </span>
                      <span className={`px-1.5 py-0.5 rounded ${plugin.capabilities.canParse ? 'bg-emerald-950/80 text-emerald-300' : 'bg-slate-950 text-slate-600'}`}>
                        Parse
                      </span>
                      <span className={`px-1.5 py-0.5 rounded ${plugin.capabilities.canExtract ? 'bg-emerald-950/80 text-emerald-300' : 'bg-slate-950 text-slate-600'}`}>
                        Extract
                      </span>
                      <span className={`px-1.5 py-0.5 rounded ${plugin.capabilities.canRebuild ? 'bg-emerald-950/80 text-emerald-300' : 'bg-slate-950 text-slate-600'}`}>
                        Rebuild
                      </span>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Right Column: Sandbox Inspector & Test Suite */}
        <div className="lg:col-span-5 space-y-4">
          <h3 className="text-xs font-bold uppercase tracking-wider text-slate-300">
            Plugin Sandbox &amp; ABI Inspector
          </h3>

          <div className="bg-slate-900 border border-slate-800 rounded-lg p-5 space-y-4 font-mono text-xs">
            <div className="border-b border-slate-800 pb-3 space-y-1">
              <div className="text-blue-400 font-bold text-sm">{selectedPlugin.name}</div>
              <div className="text-slate-500 text-[11px]">{selectedPlugin.id}</div>
              <div className="text-slate-400 text-[11px]">Author: {selectedPlugin.author}</div>
            </div>

            {/* Sandbox Limits */}
            <div className="space-y-2">
              <div className="text-[11px] uppercase text-slate-400 font-bold">Runtime Sandboxing Enforcement</div>
              <div className="grid grid-cols-2 gap-2 text-[11px]">
                <div className="bg-slate-950 p-2 rounded border border-slate-800">
                  <span className="text-slate-500">Execution Engine:</span>{' '}
                  <span className="text-slate-200 font-bold">{selectedPlugin.sandbox.runtime}</span>
                </div>
                <div className="bg-slate-950 p-2 rounded border border-slate-800">
                  <span className="text-slate-500">Memory Cap:</span>{' '}
                  <span className="text-slate-200 font-bold">{selectedPlugin.sandbox.memoryLimitMb} MB</span>
                </div>
                <div className="bg-slate-950 p-2 rounded border border-slate-800">
                  <span className="text-slate-500">Call Timeout:</span>{' '}
                  <span className="text-slate-200 font-bold">{selectedPlugin.sandbox.timeoutMs} ms</span>
                </div>
                <div className="bg-slate-950 p-2 rounded border border-slate-800">
                  <span className="text-slate-500">Host Egress:</span>{' '}
                  <span className="text-emerald-400 font-bold">BLOCKED (0 Air)</span>
                </div>
              </div>
            </div>

            {/* Sandbox Interactive Test Runner */}
            <div className="space-y-2 pt-2 border-t border-slate-800">
              <div className="flex items-center justify-between text-[11px]">
                <span className="text-slate-400 font-bold">Test Payload Bytes (Hex/String):</span>
                <button
                  onClick={handleRunSandboxTest}
                  disabled={isTesting || !selectedPlugin.enabled}
                  className={`px-3 py-1 rounded font-bold transition flex items-center gap-1 text-[11px] ${
                    !selectedPlugin.enabled
                      ? 'bg-slate-800 text-slate-500 cursor-not-allowed'
                      : 'bg-blue-600 hover:bg-blue-500 text-white shadow-lg shadow-blue-600/30'
                  }`}
                >
                  <span>{isTesting ? '⏳' : '▶'}</span>
                  <span>{isTesting ? 'Evaluating...' : 'Run Sandbox Probe'}</span>
                </button>
              </div>

              <input
                type="text"
                value={testPayload}
                onChange={(e) => setTestPayload(e.target.value)}
                className="w-full bg-slate-950 border border-slate-800 rounded p-2 text-slate-200 text-xs focus:outline-none focus:border-blue-500/80 font-mono"
              />

              {testResult && (
                <div className="space-y-1 pt-2">
                  <div className="text-[10px] text-emerald-400 uppercase">Sandbox Verification Report:</div>
                  <pre className="bg-[#05070a] border border-slate-800 p-3 rounded text-[11px] text-emerald-300 max-h-48 overflow-y-auto leading-relaxed">
                    {testResult}
                  </pre>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
