import { MapDatabaseInfo, MapUpdateItem } from '../types';

export const mapDatabases: MapDatabaseInfo[] = [
  {
    id: 'hbnavdb_main',
    name: 'HBNavDB Europe Navigation Database',
    format: 'HBNavDB',
    baselineVersion: 'ECE 6.15.5 (Stock 2012/2013)',
    sizeBytes: 17179869184, // 16 GB
    pageCount: 31580641,
    pageSizeBytes: 544,
    checksumType: 'CRC-16 / FLDB Page Header',
    modifiability: 'Modifiable (CRC Recalculated)',
    description:
      'Core navigation database containing road network geometry, attributes, turn restrictions, and spatial index (R-tree). Uses Harman FLDB container with 544-byte physical pages.',
  },
  {
    id: 'orion_atlas',
    name: 'Orion ATLAS Western Balkans & ECE',
    format: 'Orion ATLAS',
    baselineVersion: 'ATLAS v4.2.1-ECE',
    sizeBytes: 4294967296, // 4 GB
    pageCount: 8388608,
    pageSizeBytes: 512,
    checksumType: 'CRC-32 / Segment Table Checksum',
    modifiability: 'Modifiable (CRC Recalculated)',
    description:
      'High-speed routing graph and topology network. Contains node connectivity, lane guidance vectors, and link attributes.',
  },
  {
    id: 'mapstyles_xar',
    name: 'MapStyles 2D/3D Cartographic Rules',
    format: 'MapStyles XAR',
    baselineVersion: 'StyleSheet 3G+ Rel 2.4',
    sizeBytes: 134217728, // 128 MB
    pageCount: 1,
    pageSizeBytes: 134217728,
    checksumType: 'SHA-1 per-entry + Adler32 zlib',
    modifiability: 'Modifiable (XAR Unpack/Repack)',
    description:
      'Visual cartography styling package (`.xar` archive). Controls highway coloring (motorway blue/orange), building 3D extrusions, terrain elevation shaders, and icon rendering.',
  },
  {
    id: 'geo_routing_gdb',
    name: 'Geographic Routing & POI Database',
    format: 'Geographic GDB',
    baselineVersion: 'GDB-ECE-2013-Q1',
    sizeBytes: 2147483648, // 2 GB
    pageCount: 4194304,
    pageSizeBytes: 512,
    checksumType: 'SQLite 3 standard CRC32',
    modifiability: 'Modifiable (CRC Recalculated)',
    description:
      'Points of Interest, fuel stations, hospitals, speed camera alerts, and localized place-name lookup index.',
  },
];

export const mapUpdates2026: MapUpdateItem[] = [
  {
    id: 'upd_thumane_kashar',
    title: 'A1: Thumanë - Kashar Expressway (Tirana Airport Corridor)',
    region: 'Albania (Central)',
    type: 'Motorway',
    lengthKm: 21.0,
    nodesAdded: 1420,
    speedLimitKmh: 130,
    description:
      'Major 2024–2026 4-lane high-speed expressway bypassing Fushë-Krujë congestion, direct Rinas Airport connection and link to Tirana Bypass.',
    status: 'Available',
    enabled: true,
  },
  {
    id: 'upd_rruga_arbit',
    title: 'Rruga e Arbrit (Murriz Tunnel & Klos Bypass Corridor)',
    region: 'Albania (Eastern)',
    type: 'Tunnel',
    lengthKm: 27.5,
    nodesAdded: 980,
    speedLimitKmh: 90,
    description:
      'New connection between Tirana, Klos, Bulqizë and Peshkopi. Features Murriz Tunnel bypass reducing travel time to eastern border by 45 minutes.',
    status: 'Available',
    enabled: true,
  },
  {
    id: 'upd_llogara_tunnel',
    title: 'SH8: Llogara Bypass Tunnel (Vlorë - Palasë / Himarë)',
    region: 'Albania (South Coast)',
    type: 'Tunnel',
    lengthKm: 6.0,
    nodesAdded: 450,
    speedLimitKmh: 80,
    description:
      '6 km modern dual-tube tunnel piercing Llogara mountain ridge. Bypasses hazardous mountain switchbacks; connects Dukat directly to Albanian Riviera in 7 minutes.',
    status: 'Available',
    enabled: true,
  },
  {
    id: 'upd_vlore_bypass',
    title: 'Vlorë Coastal Bypass & Orikum Link (A2 Extension)',
    region: 'Albania (Southern)',
    type: 'Bypass',
    lengthKm: 29.0,
    nodesAdded: 1120,
    speedLimitKmh: 90,
    description:
      'Panoramic bypass connecting A2 Autostradë directly to southern Riviera corridor, removing transit traffic from central Vlorë city.',
    status: 'Available',
    enabled: true,
  },
  {
    id: 'upd_korce_erseke',
    title: 'Korçë - Ersekë Modernized Mountain Highway',
    region: 'Albania (South-Eastern)',
    type: 'Bypass',
    lengthKm: 35.0,
    nodesAdded: 870,
    speedLimitKmh: 80,
    description:
      'Complete reconstruction of the scenic plateau link connecting Korçë, Kolonjë, and Ersekë with widened 2-lane layout and reinforced bridges.',
    status: 'Available',
    enabled: true,
  },
  {
    id: 'upd_ev_charging_network',
    title: '2026 Western Balkans Ultra-Fast EV Charging POI Matrix',
    region: 'Albania, Kosovo, North Macedonia, Montenegro',
    type: 'EV Charging Hub',
    nodesAdded: 310,
    description:
      'Comprehensive high-power 150kW-350kW CCS2 DC fast-charger locations along Adriatic-Ionian corridor (Tirana, Durrës, Vlorë, Pristina, Skopje, Podgorica).',
    status: 'Available',
    enabled: true,
  },
  {
    id: 'upd_speed_limits_2026',
    title: '2026 Western Balkans Updated Speed Limits & Radar Geometry',
    region: 'Western Balkans Corridor',
    type: 'Speed Matrix',
    nodesAdded: 5400,
    description:
      'Refreshed speed restriction attributes matching 2026 road code: 130 km/h motorways, 110 km/h expressways, 90 km/h main interurban, with calibrated advisory curve speeds.',
    status: 'Available',
    enabled: true,
  },
];
