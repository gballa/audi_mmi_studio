import React, { useState } from 'react';

export interface ThemePreset {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  baseTrain: string;
  targetTrains: string[];
  accentColor: string;
  nightColor: string;
  riskClass: 'Cosmetic' | 'Content' | 'Structural';
  operationsCount: number;
}

export const RecipeStudio: React.FC = () => {
  const presets: ThemePreset[] = [
    {
      id: 'audi_sport_amber',
      name: 'Audi Sport Amber Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'Golden amber turn indicators and warm high-contrast night palette for MMI 3G+ cluster display.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922', 'HN+_EU_AU3G_K0900'],
      accentColor: '#FFB300',
      nightColor: '#FF8F00',
      riskClass: 'Content',
      operationsCount: 3,
    },
    {
      id: 'rs_performance_red',
      name: 'RS Performance Red Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'RS performance high-visibility crimson accent scheme with sport cluster graphics.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922'],
      accentColor: '#E0001B',
      nightColor: '#B30000',
      riskClass: 'Content',
      operationsCount: 3,
    },
    {
      id: 'dark_line_minimalist',
      name: 'Dark Line Minimalist Theme',
      version: '1.0.0',
      author: 'Audi MMI Studio Workstation',
      description: 'Muted monochromatic high-contrast dark palette optimized for reduced night glare and OLED retrofits.',
      baseTrain: 'HN+R_EU_AU_K0942_4',
      targetTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922', 'HN+_EU_AU3G_K0900'],
      accentColor: '#B0B0B0',
      nightColor: '#2A2A2A',
      riskClass: 'Content',
      operationsCount: 3,
    },
  ];

  const [selectedId, setSelectedId] = useState<string>('audi_sport_amber');
  const [deployStatus, setDeployStatus] = useState<string | null>(null);

  const activeTheme = presets.find((p) => p.id === selectedId) || presets[0];

  const handleSimulateDeploy = () => {
    setDeployStatus(`Building update media for '${activeTheme.name}'...`);
    setTimeout(() => {
      setDeployStatus(`Media Volume created: FAT32 update image ready for SD card deployment. Status: BUILD READY — DEPLOYMENT NOT VERIFIED.`);
    }, 1000);
  };

  return (
    <div style={{ display: 'flex', height: '100%', gap: '24px', padding: '24px', background: '#12151a', color: '#e0e6ed', boxSizing: 'border-box' }}>
      {/* Preset List Sidebar */}
      <div style={{ width: '340px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <h3 style={{ margin: '0 0 8px 0', fontSize: '18px', color: '#fff' }}>Theme Recipe Library</h3>
        {presets.map((p) => {
          const isSelected = p.id === selectedId;
          return (
            <div
              key={p.id}
              onClick={() => setSelectedId(p.id)}
              style={{
                padding: '16px',
                borderRadius: '8px',
                background: isSelected ? '#1e2430' : '#161b22',
                border: isSelected ? `2px solid ${p.accentColor}` : '1px solid #28303e',
                cursor: 'pointer',
                transition: 'all 0.2s ease',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '8px' }}>
                <span style={{ fontWeight: 'bold', fontSize: '15px', color: '#fff' }}>{p.name}</span>
                <span
                  style={{
                    width: '18px',
                    height: '18px',
                    borderRadius: '50%',
                    background: p.accentColor,
                    boxShadow: `0 0 8px ${p.accentColor}80`,
                  }}
                />
              </div>
              <p style={{ margin: '0 0 10px 0', fontSize: '12px', color: '#8b949e', lineHeight: 1.4 }}>{p.description}</p>
              <div style={{ display: 'flex', gap: '8px', fontSize: '11px' }}>
                <span style={{ padding: '2px 8px', borderRadius: '4px', background: '#28303e', color: '#58a6ff' }}>{p.baseTrain}</span>
                <span style={{ padding: '2px 8px', borderRadius: '4px', background: '#382b15', color: '#e3b341' }}>{p.riskClass}</span>
              </div>
            </div>
          );
        })}
      </div>

      {/* Theme Details & Customizer Panel */}
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '20px', background: '#161b22', padding: '24px', borderRadius: '8px', border: '1px solid #28303e' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
          <div>
            <h2 style={{ margin: '0 0 6px 0', fontSize: '22px', color: '#fff' }}>{activeTheme.name}</h2>
            <div style={{ fontSize: '13px', color: '#8b949e' }}>
              Author: {activeTheme.author} • Version: {activeTheme.version} • Unit of Work: Declarative JSON
            </div>
          </div>
          <button
            onClick={handleSimulateDeploy}
            style={{
              padding: '10px 20px',
              borderRadius: '6px',
              background: activeTheme.accentColor,
              color: '#000',
              fontWeight: 'bold',
              border: 'none',
              cursor: 'pointer',
              fontSize: '14px',
              transition: 'opacity 0.2s',
            }}
          >
            Package SD-Card Update
          </button>
        </div>

        {deployStatus && (
          <div style={{ padding: '12px 16px', borderRadius: '6px', background: '#1c2d42', border: '1px solid #388bfd', color: '#79c0ff', fontSize: '13px' }}>
            {deployStatus}
          </div>
        )}

        {/* Color Palette Display */}
        <div style={{ display: 'flex', gap: '20px', background: '#1e2430', padding: '16px', borderRadius: '6px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <div style={{ width: '40px', height: '40px', borderRadius: '6px', background: activeTheme.accentColor, border: '1px solid #fff' }} />
            <div>
              <div style={{ fontSize: '11px', color: '#8b949e' }}>DAY ACCENT</div>
              <div style={{ fontFamily: 'monospace', fontWeight: 'bold' }}>{activeTheme.accentColor}</div>
            </div>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <div style={{ width: '40px', height: '40px', borderRadius: '6px', background: activeTheme.nightColor, border: '1px solid #fff' }} />
            <div>
              <div style={{ fontSize: '11px', color: '#8b949e' }}>NIGHT ACCENT</div>
              <div style={{ fontFamily: 'monospace', fontWeight: 'bold' }}>{activeTheme.nightColor}</div>
            </div>
          </div>
        </div>

        {/* Target Trains & Compatibility */}
        <div>
          <h4 style={{ margin: '0 0 10px 0', fontSize: '14px', color: '#c9d1d9' }}>Compatible Target Trains (Rebase Engine Supported)</h4>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
            {activeTheme.targetTrains.map((train) => (
              <span key={train} style={{ padding: '4px 10px', borderRadius: '4px', background: '#21262d', border: '1px solid #30363d', fontSize: '12px', fontFamily: 'monospace', color: '#7ee787' }}>
                {train}
              </span>
            ))}
          </div>
        </div>

        {/* Simulated Virtual Cluster Display */}
        <div style={{ flex: 1, background: '#0d1117', borderRadius: '8px', border: '1px solid #30363d', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', padding: '20px' }}>
          <div style={{ fontSize: '12px', color: '#8b949e', marginBottom: '16px' }}>Virtual Cluster Turn Display (800x480 Preview)</div>
          <svg width="120" height="120" viewBox="0 0 100 100">
            {/* Instrument turn arrow indicator rendered in active theme accent color */}
            <path
              d="M 50 15 L 85 50 L 65 50 L 65 85 L 35 85 L 35 50 L 15 50 Z"
              fill={activeTheme.accentColor}
              stroke="#ffffff"
              strokeWidth="2"
              filter="drop-shadow(0 0 8px rgba(0,0,0,0.8))"
            />
          </svg>
          <div style={{ marginTop: '16px', fontSize: '13px', fontWeight: 'bold', color: activeTheme.accentColor }}>
            150 m — Turn Right onto A9 Autobahn
          </div>
        </div>
      </div>
    </div>
  );
};
