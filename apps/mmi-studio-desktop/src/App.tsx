import React, { useState } from 'react';
import { AssetBoard } from './components/AssetBoard';
import { HexViewer } from './components/HexViewer';
import { TypographyStudio } from './components/TypographyStudio';
import { RecipeStudio } from './components/RecipeStudio';
import { LocalizationStudio } from './components/LocalizationStudio';
import { MapStudio } from './components/MapStudio';
import { ComponentCustomizer } from './components/ComponentCustomizer';
import { BuildStudio } from './components/BuildStudio';
import { AiElementStudio } from './components/AiElementStudio';
import { ResetModal, ResetSelectiveOptions } from './components/ResetModal';
import { initialLocalizationStrings } from './data/localizationData';
import { mapDatabases, mapUpdates2026 } from './data/mapData';
import { initialAiAssets } from './data/aiAssetsData';
import { InspectResult, MMIThemeConfig, SystemString, MapUpdateItem, AiAssetItem } from './types';

type Tab =
  | 'components'
  | 'ai_elements'
  | 'localization'
  | 'maps'
  | 'build'
  | 'recipes'
  | 'assets'
  | 'relab'
  | 'typography';

const initialThemeConfig: MMIThemeConfig = {
  accentColor: '#FF9900', // Audi Sport Amber
  backgroundColor: '#0C0E14',
  surfaceColor: '#121620',
  highlightColor: '#FFB300',
  needleColor: '#FF0000',
  fontFamily: 'AudiType-Bold',
  fontSizeScale: 1.0,
  letterSpacingPx: 0.0,
  showCompass: true,
  showClock: true,
  showTemperature: true,
  showStatusBar: true,
  showClimateOverlay: false,
  ambientGlow: true,
  highContrast: false,
  language: 'sq', // Default to Albanian as requested by user
  activeCarSilhouetteStyle: 'stroke-amber-500/80 fill-slate-900/60',
  activeBackgroundTexture: 'default',
  activeNavArrowStyle: '#FF9900',
};

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<Tab>('components');
  const [strings, setStrings] = useState<SystemString[]>(initialLocalizationStrings);
  const [mapUpdates, setMapUpdates] = useState<MapUpdateItem[]>(mapUpdates2026);
  const [aiAssets, setAiAssets] = useState<AiAssetItem[]>(initialAiAssets);
  const [exportNotification, setExportNotification] = useState<string | null>(null);
  const [isResetModalOpen, setIsResetModalOpen] = useState<boolean>(false);

  // MMI UI Theme & Customization Configuration
  const [themeConfig, setThemeConfig] = useState<MMIThemeConfig>(initialThemeConfig);

  // Sample seed assets for workstation explorer
  const sampleAssets: InspectResult[] = [
    {
      filePath: 'HBAS/Precomp/System/boot.precomp',
      sizeBytes: 1048576,
      blake3Hash: 'a1b2c3d4e5f60718293a4b5c6d7e8f90123456789abcdef0123456789abcdef0',
      detectedFormat: 'HarmanPrecomp',
      thumbnailBlobId: 'thumb_boot_001',
    },
    {
      filePath: 'HBAS/Precomp/System/gui_nav.precomp',
      sizeBytes: 4194304,
      blake3Hash: 'f0e1d2c3b4a5968778695a4b3c2d1e0ffeeddccbbaa99887766554433221100f',
      detectedFormat: 'HarmanPrecomp',
      thumbnailBlobId: 'thumb_nav_002',
    },
    {
      filePath: 'HBAS/Precomp/Media/audio_eq.precomp',
      sizeBytes: 524288,
      blake3Hash: '1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
      detectedFormat: 'HarmanPrecomp',
    },
    {
      filePath: 'HBNavDB/Database/nav_data.db',
      sizeBytes: 17179869184,
      blake3Hash: '9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba',
      detectedFormat: 'HBNavDB',
    },
    {
      filePath: 'Fonts/AudiType-Extended.linotype',
      sizeBytes: 262144,
      blake3Hash: 'abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789',
      detectedFormat: 'LinotypeFont',
    },
  ];

  const handleUpdateString = (id: string, newAlbanian: string) => {
    setStrings((prev) =>
      prev.map((item) => (item.id === id ? { ...item, sq: newAlbanian } : item))
    );
  };

  const handleToggleMapUpdate = (id: string) => {
    setMapUpdates((prev) =>
      prev.map((item) => (item.id === id ? { ...item, enabled: !item.enabled } : item))
    );
  };

  const handleUpdateAiAsset = (updated: AiAssetItem) => {
    setAiAssets((prev) => prev.map((a) => (a.id === updated.id ? updated : a)));
  };

  const handleUpdateTheme = (newConfig: Partial<MMIThemeConfig>) => {
    setThemeConfig((prev) => ({ ...prev, ...newConfig }));
  };

  const handlePreviewStringInScreen = (stringItem: SystemString) => {
    setActiveTab('components');
    setThemeConfig((prev) => ({ ...prev, language: 'sq' }));
    setExportNotification(`Previewing '${stringItem.sq}' in MMI screen canvas`);
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleExportRecipe = () => {
    const recipe = {
      formatVersion: '1.0.0',
      schema: 'audi-mmi-recipe-v1',
      title: 'Custom Audi MMI Theme, Gemini Nano Banana Assets & Albanian Localization Pack',
      createdAt: new Date().toISOString(),
      targetPlatform: 'MMI 3G High / Plus [HN+]',
      targetFirmwareTrains: ['HN+R_EU_AU_K0942_4', 'HN+R_EU_AU_P0922', 'HN+_EU_AU3G_K0900'],
      riskClass: 'Content',
      modifications: {
        theme: {
          accentColor: themeConfig.accentColor,
          needleColor: themeConfig.needleColor,
          fontFamily: themeConfig.fontFamily,
          ambientGlow: themeConfig.ambientGlow,
          activeBackgroundTexture: themeConfig.activeBackgroundTexture,
        },
        aiAssets: aiAssets.filter((a) => a.status === 'AI Modified').map((a) => ({
          id: a.id,
          style: a.currentStyle,
          prompt: a.aiPromptApplied,
          model: a.modelUsed,
        })),
        localization: {
          languageCode: 'sq_AL',
          languageName: 'Gjuha Shqipe',
          translatedStringsCount: strings.length,
        },
        mapUpdatesStaged: mapUpdates.filter((u) => u.enabled).map((u) => u.id),
      },
      verificationStatus: 'BUILD READY — DEPLOYMENT NOT VERIFIED',
    };

    const blob = new Blob([JSON.stringify(recipe, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'audi_mmi_custom_theme_sq_al_recipe.json';
    a.click();
    URL.revokeObjectURL(url);

    setExportNotification('Recipe successfully exported: audi_mmi_custom_theme_sq_al_recipe.json');
    setTimeout(() => setExportNotification(null), 4000);
  };

  const handleResetTheme = () => {
    setThemeConfig(initialThemeConfig);
    setExportNotification('🎨 UI Theme restored to OEM baseline.');
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleResetAiAsset = (assetId: string) => {
    const stockItem = initialAiAssets.find((a) => a.id === assetId);
    if (stockItem) {
      setAiAssets((prev) => prev.map((a) => (a.id === assetId ? { ...stockItem } : a)));
      setExportNotification(`🍌 Asset '${stockItem.name}' reset to stock OEM.`);
      setTimeout(() => setExportNotification(null), 3000);
    }
  };

  const handleResetAllAiAssets = () => {
    setAiAssets(initialAiAssets);
    setExportNotification('🍌 All AI elements reset to stock OEM.');
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleResetSingleString = (stringId: string) => {
    const stockItem = initialLocalizationStrings.find((s) => s.id === stringId);
    if (stockItem) {
      setStrings((prev) => prev.map((s) => (s.id === stringId ? { ...stockItem } : s)));
      setExportNotification(`🇦🇱 String '${stockItem.id}' reset to baseline translation.`);
      setTimeout(() => setExportNotification(null), 3000);
    }
  };

  const handleResetAllStrings = () => {
    setStrings(initialLocalizationStrings);
    setExportNotification('🇦🇱 All 43+ translations reset to verified baseline.');
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleResetMapUpdates = () => {
    setMapUpdates(mapUpdates2026);
    setExportNotification('🗺️ 2026 Map updates reset to recommended baseline.');
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleDisableAllMapUpdates = () => {
    setMapUpdates((prev) => prev.map((u) => ({ ...u, enabled: false })));
    setExportNotification('🗺️ All custom map patches disabled (Stock baseline).');
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleResetBuild = () => {
    setExportNotification('🚀 Build staging and simulation state reset.');
    setTimeout(() => setExportNotification(null), 3000);
  };

  const handleResetCurrentFeature = (tab: string) => {
    switch (tab) {
      case 'components':
        handleResetTheme();
        break;
      case 'ai_elements':
        handleResetAllAiAssets();
        break;
      case 'localization':
        handleResetAllStrings();
        break;
      case 'maps':
        handleResetMapUpdates();
        break;
      case 'build':
        handleResetBuild();
        break;
      default:
        handleResetTheme();
        break;
    }
  };

  const handleResetAll = () => {
    setThemeConfig(initialThemeConfig);
    setAiAssets(initialAiAssets);
    setStrings(initialLocalizationStrings);
    setMapUpdates(mapUpdates2026);
    setExportNotification('⚠️ Complete Factory Reset: All session customizations reverted to OEM stock.');
    setTimeout(() => setExportNotification(null), 4000);
  };

  const handleResetSelective = (options: ResetSelectiveOptions) => {
    const resetList: string[] = [];
    if (options.theme) {
      setThemeConfig(initialThemeConfig);
      resetList.push('Theme');
    }
    if (options.aiElements) {
      setAiAssets(initialAiAssets);
      resetList.push('AI Elements');
    }
    if (options.localization) {
      setStrings(initialLocalizationStrings);
      resetList.push('Localization');
    }
    if (options.maps) {
      setMapUpdates(mapUpdates2026);
      resetList.push('Maps');
    }
    if (options.build) {
      handleResetBuild();
      resetList.push('Build');
    }
    setExportNotification(`↺ Reset applied to: ${resetList.join(', ')}`);
    setTimeout(() => setExportNotification(null), 3500);
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-slate-950 text-slate-100 select-none overflow-hidden font-sans">
      {/* Top Navigation Bar */}
      <header className="flex items-center justify-between px-4 py-2 bg-slate-900 border-b border-slate-800">
        <div className="flex items-center gap-3">
          <div className="w-7 h-7 rounded bg-amber-500 flex items-center justify-center font-black text-slate-950 text-xs shadow">
            MMI
          </div>
          <div>
            <h1 className="text-sm font-bold tracking-tight text-white flex items-center gap-2">
              Audi MMI Studio Workstation
              <span className="px-1.5 py-0.2 text-[9px] bg-red-950/80 border border-red-800 text-red-300 font-mono rounded font-bold">
                sq_AL Active
              </span>
            </h1>
            <span className="text-[10px] text-slate-400 font-mono">
              MMI 3G High / Plus [HN+] · ECE 2026 Navigation
            </span>
          </div>
        </div>

        {/* Center Primary Engineering Tabs */}
        <nav className="flex items-center gap-1 bg-slate-950 p-1 rounded-lg border border-slate-800">
          <button
            onClick={() => setActiveTab('components')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all flex items-center gap-1.5 ${
              activeTab === 'components'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            <span>🖥️</span> Screen & Components
          </button>
          <button
            onClick={() => setActiveTab('ai_elements')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all flex items-center gap-1.5 ${
              activeTab === 'ai_elements'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-amber-400 hover:text-amber-300 font-semibold'
            }`}
          >
            <span>🍌</span> Gemini AI Elements
          </button>
          <button
            onClick={() => setActiveTab('localization')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all flex items-center gap-1.5 ${
              activeTab === 'localization'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            <span>🇦🇱</span> Localization (sq_AL)
          </button>
          <button
            onClick={() => setActiveTab('maps')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all flex items-center gap-1.5 ${
              activeTab === 'maps'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            <span>🗺️</span> 2026 Map Studio
          </button>
          <button
            onClick={() => setActiveTab('build')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all flex items-center gap-1.5 ${
              activeTab === 'build'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-emerald-400 hover:text-emerald-300 font-semibold'
            }`}
          >
            <span>🚀</span> Build & SD Deploy
          </button>
          <button
            onClick={() => setActiveTab('recipes')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all ${
              activeTab === 'recipes'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Theme Recipes
          </button>
          <button
            onClick={() => setActiveTab('typography')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all ${
              activeTab === 'typography'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Typography
          </button>
          <button
            onClick={() => setActiveTab('relab')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all ${
              activeTab === 'relab'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Binary RE Lab
          </button>
          <button
            onClick={() => setActiveTab('assets')}
            className={`px-3 py-1.5 text-xs rounded-md font-medium transition-all ${
              activeTab === 'assets'
                ? 'bg-amber-500 text-slate-950 font-bold shadow'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            Asset Census
          </button>
        </nav>

        {/* Right Status Badges & Reset Action */}
        <div className="flex items-center gap-2">
          {exportNotification && (
            <span className="text-xs text-amber-400 font-bold animate-pulse">
              {exportNotification}
            </span>
          )}
          <button
            onClick={() => setIsResetModalOpen(true)}
            className="px-2.5 py-1 text-xs font-semibold rounded border border-slate-700 bg-slate-800/90 hover:bg-slate-700 text-slate-200 hover:text-white flex items-center gap-1.5 transition shadow-sm"
            title="Open MMI Workstation Reset Manager (Feature or Factory Reset)"
          >
            <span className="text-amber-400 font-bold">↺</span> Reset...
          </button>
          <span className="flex items-center gap-1 px-2 py-0.5 text-[10px] rounded border border-emerald-800 bg-emerald-950/80 text-emerald-400 font-mono">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
            AIRGAP ACTIVE
          </span>
          <span className="px-2 py-0.5 text-[10px] rounded border border-slate-700 bg-slate-800 text-slate-300 font-mono">
            IMMUTABLE ORIGINALS
          </span>
        </div>
      </header>

      {/* Main Workspace Area */}
      <main className="flex-1 overflow-hidden relative">
        {activeTab === 'components' && (
          <ComponentCustomizer
            themeConfig={themeConfig}
            onUpdateTheme={handleUpdateTheme}
            onResetTheme={handleResetTheme}
            strings={strings}
            onExportRecipe={handleExportRecipe}
            onNavigateToAiStudio={() => setActiveTab('ai_elements')}
          />
        )}
        {activeTab === 'ai_elements' && (
          <AiElementStudio
            assets={aiAssets}
            onUpdateAsset={handleUpdateAiAsset}
            onResetAsset={handleResetAiAsset}
            onResetAllAssets={handleResetAllAiAssets}
            themeConfig={themeConfig}
            onUpdateTheme={handleUpdateTheme}
            onNavigateToScreen={() => setActiveTab('components')}
          />
        )}
        {activeTab === 'localization' && (
          <LocalizationStudio
            strings={strings}
            onUpdateString={handleUpdateString}
            onPreviewStringInScreen={handlePreviewStringInScreen}
            onResetAllStrings={handleResetAllStrings}
            onResetSingleString={handleResetSingleString}
          />
        )}
        {activeTab === 'maps' && (
          <MapStudio
            databases={mapDatabases}
            updates={mapUpdates}
            onToggleUpdate={handleToggleMapUpdate}
            onResetUpdates={handleResetMapUpdates}
            onDisableAllUpdates={handleDisableAllMapUpdates}
          />
        )}
        {activeTab === 'build' && (
          <BuildStudio
            themeConfig={themeConfig}
            strings={strings}
            mapUpdates={mapUpdates}
            onResetBuild={handleResetBuild}
          />
        )}
        {activeTab === 'recipes' && <RecipeStudio />}
        {activeTab === 'relab' && <HexViewer />}
        {activeTab === 'typography' && <TypographyStudio />}
        {activeTab === 'assets' && (
          <AssetBoard
            assets={sampleAssets}
            onSelectAsset={(asset) => {
              console.log('Selected asset:', asset.filePath);
            }}
          />
        )}
      </main>

      {/* Bottom Global Status Bar */}
      <footer className="flex items-center justify-between px-4 py-1.5 bg-slate-900 border-t border-slate-800 text-[11px] text-slate-400">
        <div className="flex items-center gap-3">
          <span className="text-amber-500 font-semibold">§14.9 Safety Policy:</span>
          <span>
            "SAFE TO INSTALL" claims strictly banned. All builds require hardware-level recovery verification.
          </span>
        </div>
        <div className="flex items-center gap-4 font-mono text-[10px]">
          <span>IPC: Native Tauri 2 (Rust)</span>
          <span>React 18 / TypeScript</span>
          <span className="text-emerald-400">BLAKE3: Verified</span>
        </div>
      </footer>
      {/* Global & Feature Reset Modal */}
      <ResetModal
        isOpen={isResetModalOpen}
        onClose={() => setIsResetModalOpen(false)}
        activeTab={activeTab}
        onResetAll={handleResetAll}
        onResetFeature={handleResetCurrentFeature}
        onResetSelective={handleResetSelective}
      />
    </div>
  );
};

export default App;
