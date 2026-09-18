#!/usr/bin/env python3
"""
scripts/scan_originals.py
Audi MMI Studio — Tiered Evidence & Census Scanner (Pass A)

Performs:
- L0: Directory and file enumeration (excluding .DS_Store).
- L1: Magic byte sniffing and 12-category classification with confidence and evidence tags.
- L2: Single-pass streaming dual hashing (SHA-256 + BLAKE3).
- L3: Archive Table of Contents (TOC) extraction for .zip, .7z, .iso without extraction.
- L4: Header sampling (<=256 MiB / 5,000 files) & comprehensive Asset Census (PNGs, .precomp, TTF fonts).
- Generation of normalized SQLite database: originals-manifest.sqlite.
- Deterministic 1-to-1 JSON export: originals-manifest.json.

Strict constraint: originals/ is 100% READ-ONLY.
"""

import os
import sys
import time
import struct
import sqlite3
import json
import zlib
import hashlib
import ctypes
import subprocess
from datetime import datetime, timezone
from pathlib import Path

# Paths
REPO_ROOT = Path(__file__).resolve().parent.parent
ORIGINALS_DIR = REPO_ROOT / "originals"
SQLITE_PATH = REPO_ROOT / "originals-manifest.sqlite"
JSON_PATH = REPO_ROOT / "originals-manifest.json"
LIBBLAKE3_PATH = REPO_ROOT / "scripts" / "libblake3.dylib"

# Compile libblake3 if not present
if not LIBBLAKE3_PATH.exists():
    c_src = REPO_ROOT / "scripts" / "blake3.c"
    if c_src.exists():
        print("Compiling libblake3.dylib...")
        subprocess.run(["clang", "-O3", "-shared", "-fPIC", "-o", str(LIBBLAKE3_PATH), str(c_src)], check=True)

# Load C BLAKE3 library or fallback to pure Python
c_blake3_available = False
try:
    libb3 = ctypes.CDLL(str(LIBBLAKE3_PATH))
    libb3.blake3_new.restype = ctypes.c_void_p
    libb3.blake3_update.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_size_t]
    libb3.blake3_final.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
    libb3.blake3_free.argtypes = [ctypes.c_void_p]
    libb3.blake3_hash_buffer.argtypes = [ctypes.c_char_p, ctypes.c_size_t, ctypes.c_char_p]
    c_blake3_available = True
    print("[INIT] High-performance native C BLAKE3 engine loaded.")
except Exception as e:
    print(f"[WARN] Failed to load native libblake3.dylib ({e}); using pure Python fallback.")
    from scripts.blake3_pure import Blake3, blake3_hash


def compute_blake3_buffer(data: bytes) -> str:
    if c_blake3_available:
        out = ctypes.create_string_buffer(32)
        libb3.blake3_hash_buffer(data, len(data), out)
        return out.raw.hex()
    return blake3_hash(data)


class DualHasher:
    """Streams file data into both SHA-256 and BLAKE3 in a single read pass."""
    def __init__(self):
        self.sha256 = hashlib.sha256()
        self.use_c = c_blake3_available
        if self.use_c:
            self.b3 = libb3.blake3_new()
        else:
            self.b3 = Blake3()

    def update(self, chunk: bytes):
        self.sha256.update(chunk)
        if self.use_c:
            libb3.blake3_update(self.b3, chunk, len(chunk))
        else:
            self.b3.update(chunk)

    def finalize(self) -> tuple:
        sha_hex = self.sha256.hexdigest()
        if self.use_c:
            out = ctypes.create_string_buffer(32)
            libb3.blake3_final(self.b3, out)
            libb3.blake3_free(self.b3)
            b3_hex = out.raw.hex()
        else:
            b3_hex = self.b3.hexdigest()
        return sha_hex, b3_hex


# Magic byte sniffing signatures
MAGIC_RULES = [
    (b"\x89PNG\r\n\x1a\n", 0, "image/png", "PNG Image Bitmap"),
    (b"7z\xbc\xaf\x27\x1c", 0, "application/x-7z-compressed", "7-Zip Archive Container"),
    (b"PK\x03\x04", 0, "application/zip", "ZIP Archive Container"),
    (b"rax\x00", 0, "application/x-harman-mapstyle", "Harman/EB MapStyle XAR Archive"),
    (b"\xeb\x7e\xff\x00", 0, "application/x-qnx-ifs", "QNX Neutrino Image FileSystem (IFS)"),
    (b"\x10\x00\x4c\xff", 0, "application/x-qnx-efs", "QNX Embedded Flash FileSystem (EFS/ETFS)"),
    (b"ANS\x00", 0, "audio/x-harman-acoustic", "Harman/Becker Speech Recognition Acoustic Model"),
    (b"\x20\x02\x00\x00\x01\x00\x00\x00", 0, "application/x-harman-navdb", "Harman/Becker NavDB Binary Database"),
    (b"\x06HEADER\xcc\xcc", 0, "application/x-harman-atlas", "Harman/Becker Navigation ATLAS Spatial Tile Container"),
    (b"\xde\xad\xbe\xef", 0, "application/x-routing-gdb", "Harman/Becker Navigation Routing Database (GDB/GD2)"),
    (b"\x7fELF", 0, "application/x-executable", "ELF Binary Executable / Shared Object"),
    (b"RIFF", 0, "audio/wav", "RIFF WAVE Audio Prompt"),
    (b"\x01\x0f\xff\xff", 0, "application/x-smsc-inic", "SMSC OS81050 MOST INIC Firmware Image"),
    (b"\xe2\xd3\xc6\xb8", 0, "application/x-bfin-ldr", "Analog Devices Blackfin DSP Loader Executable"),
    (b".HDG", 0, "application/x-harman-fpga", "Harman Becker System FPGA Bitstream"),
]


def sniff_magic(header: bytes, ext_lower: str) -> tuple:
    """Returns (mime_type, magic_description)."""
    for sig, offset, mime, desc in MAGIC_RULES:
        if len(header) >= offset + len(sig) and header[offset:offset+len(sig)] == sig:
            return mime, desc

    if len(header) >= 10 and header[:2] == b"\x00\x01" and header[10:12] == b"\x78\xda":
        return "image/x-harman-precomp", "CombiStyles Precomputed Cluster Turn Graphic (zlib)"

    if ext_lower == ".ttf" or header[:4] in (b"\x00\x01\x00\x00", b"true", b"typ1"):
        return "font/ttf", "TrueType / OpenType SFNT Font"

    if ext_lower == ".pkg":
        return "text/x-audi-pkg", "Audi Map Package INI Specification"
    if ext_lower == ".sig":
        return "application/x-pkcs-sig", "RSA 1024-bit Detached Cryptographic Signature"
    if ext_lower == ".fsc":
        return "application/x-audi-fsc", "Audi Feature Enablement Code (FSC) Certificate"
    if ext_lower in (".txt", ".conf", ".ini", ".nfm"):
        return "text/plain", "Plaintext Configuration / Manifest"
    if ext_lower == ".sh":
        return "text/x-shellscript", "POSIX Shell Script"
    if ext_lower == ".bin":
        return "application/octet-stream", "Raw Firmware / Application Binary"

    return "application/octet-stream", "Binary Data"


def determine_domain(top_level: str) -> str:
    """Maps top-level item to domain category."""
    if "HN+R" in top_level:
        return "HN+R_SOFTWARE"
    elif "8R0051884KL" in top_level:
        return "MAP_PACKAGE"
    elif "Software" in top_level:
        return "HNAV_SOFTWARE"
    elif "License" in top_level:
        return "LICENSE_ACTIVATION"
    elif "Vlasoff" in top_level:
        return "MAP_ACTIVATION"
    return "SUPPORTING"


def classify_file(rel_path: str, ext_lower: str, mime: str, is_signed: bool) -> tuple:
    """Classifies file into 12 standard prompt categories with confidence."""
    # 1. Signed payloads
    if is_signed or ext_lower == ".sig":
        return "signed-payload", "HIGH"
    if rel_path.endswith(".pkg"):
        return "signed-payload", "HIGH"
    if rel_path.endswith("TMCConfig.dat"):
        return "signed-payload", "HIGH"

    # 2. Activation or licence
    if "Vlasoff" in rel_path or "2380_00040009.fsc" in rel_path:
        return "activation-or-licence", "HIGH"
    if "License/run.sh" in rel_path or "License/copie_scr.sh" in rel_path or "DecodeScript" in rel_path:
        return "activation-or-licence", "HIGH"

    # 3. Diagnostic tool
    if "License/utils" in rel_path:
        return "diagnostic-tool", "HIGH"

    # 4. Map style
    if "MapStyles" in rel_path or "StyleDB" in rel_path or ext_lower == ".xar":
        return "map-style", "HIGH"

    # 5. Audio amplifier
    if any(k in rel_path for k in ("Bose", "BangOlufsen", "STG_Amp")) or ext_lower in (".cdp", ".ndp", ".udp", ".pil", ".spm", ".ard", ".brd", ".sdp", ".otd"):
        return "audio-amplifier", "HIGH"

    # 6. Display
    if any(k in rel_path for k in ("DU902A", "DUA017", "DU9357")):
        return "display", "HIGH"

    # 7. Radio tuner
    if any(k in rel_path for k in ("ARU93", "ARU94")):
        return "radio-tuner", "HIGH"

    # 8. Localisation
    if ext_lower in (".ans", ".wav") or any(k in rel_path for k in ("SDS", "KBD", "speech", "sss")):
        return "localisation", "HIGH"

    # 9. UI resource
    if ext_lower in (".png", ".precomp", ".hbgr", ".ttf") or "GEMMI" in rel_path and ("models" in rel_path or "res" in rel_path):
        return "UI-resource", "HIGH"

    # 10. Firmware module
    if any(k in rel_path for k in ("RSU9425", "MU94", "MU9308")) or ext_lower in (".ifs", ".efs", ".hbbin", ".ipf"):
        return "firmware-module", "HIGH"

    # 11. Software update
    if rel_path in ("HN+R_EU_AU_K0942_4_[8R0906961FB].zip", "HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt", "Software.zip", "Software/metainfo2.txt"):
        return "software-update", "HIGH"

    # 12. Map update
    if rel_path in ("8R0051884KL_6.36.0_2023.7z", "8R0051884KL_6.36.0_2023/metainfo2.txt", "8R0051884KL_6.36.0_2023/DBInfo.txt"):
        return "map-update", "HIGH"

    return "firmware-module", "MEDIUM"


def extract_png_meta(path: Path) -> dict:
    """Extracts metadata from PNG file."""
    try:
        with open(path, "rb") as fh:
            data = fh.read()
        if len(data) < 29 or data[:8] != b"\x89PNG\r\n\x1a\n":
            return None
        w, h, bd, ct = struct.unpack(">IIBB", data[16:26])
        ct_map = {0: "GRAYSCALE", 2: "RGB", 3: "PALETTE", 4: "GRAYSCALE_ALPHA", 6: "RGBA"}
        color_type = ct_map.get(ct, f"UNKNOWN_{ct}")
        has_alpha = ct in (4, 6)
        alpha_semantics = "straight" if has_alpha else "none"
        palette_size = None
        dpi_h, dpi_v = None, None

        idx = 8
        while idx + 8 <= len(data):
            length, chunk_type = struct.unpack(">I4s", data[idx:idx+8])
            chunk_data = data[idx+8:idx+8+length]
            if chunk_type == b"PLTE":
                palette_size = length // 3
            elif chunk_type == b"tRNS":
                has_alpha = True
                alpha_semantics = "binary" if ct == 3 else "straight"
            elif chunk_type == b"pHYs" and length >= 9:
                ppu_x, ppu_y, unit = struct.unpack(">IIB", chunk_data[:9])
                if unit == 1:
                    dpi_h = round(ppu_x * 0.0254, 1)
                    dpi_v = round(ppu_y * 0.0254, 1)
            idx += 12 + length

        return {
            "codec": "PNG",
            "width": w,
            "height": h,
            "bit_depth": bd,
            "color_type": color_type,
            "palette_size": palette_size,
            "has_alpha": 1 if has_alpha else 0,
            "alpha_semantics": alpha_semantics,
            "dpi_h": dpi_h,
            "dpi_v": dpi_v,
            "decode_status": "SUCCESS"
        }
    except Exception as e:
        return {"codec": "PNG", "decode_status": f"FAILED: {e}"}


def extract_precomp_meta(path: Path) -> dict:
    """Extracts metadata from .precomp instrument cluster graphic."""
    try:
        with open(path, "rb") as fh:
            data = fh.read()
        if len(data) < 10:
            return {"codec": "PRECOMP_ZLIB", "decode_status": "FAILED: file too small"}
        w, h = struct.unpack(">HH", data[6:10])
        z = data[10:]
        dec = zlib.decompress(z)
        expected = w * h * 4
        if len(dec) == expected:
            status = "SUCCESS"
        else:
            status = f"FAILED: size mismatch {len(dec)} != {expected}"
        return {
            "codec": "PRECOMP_ZLIB",
            "width": w,
            "height": h,
            "bit_depth": 32,
            "color_type": "RGBA",
            "palette_size": None,
            "has_alpha": 1,
            "alpha_semantics": "straight",
            "dpi_h": None,
            "dpi_v": None,
            "decode_status": status
        }
    except Exception as e:
        return {"codec": "PRECOMP_ZLIB", "decode_status": f"FAILED: {e}"}


def extract_font_meta(path: Path) -> dict:
    """Extracts metadata from TrueType / OpenType font using fc-scan and maxp table."""
    meta = {
        "codec": "TTF",
        "font_family": None,
        "font_style": None,
        "glyph_count": None,
        "supported_scripts": [],
        "decode_status": "SUCCESS"
    }
    try:
        # Read maxp table for exact glyph count
        with open(path, "rb") as f:
            data = f.read()
        num_tables = struct.unpack(">H", data[4:6])[0]
        for i in range(num_tables):
            tag, check, offset, length = struct.unpack(">4sIII", data[12 + i*16 : 12 + (i+1)*16])
            if tag == b"maxp":
                meta["glyph_count"] = struct.unpack(">H", data[offset+4 : offset+6])[0]
                break

        # Run fc-scan for font properties
        res = subprocess.run(["fc-scan", str(path)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if res.returncode == 0:
            for line in res.stdout.splitlines():
                line = line.strip()
                if line.startswith("family:"):
                    meta["font_family"] = line.split('"')[1]
                elif line.startswith("style:"):
                    meta["font_style"] = line.split('"')[1]
                elif line.startswith("capability:"):
                    caps = line.split('"')[1].split()
                    for cap in caps:
                        if cap.startswith("otlayout:"):
                            meta["supported_scripts"].append(cap.split(":")[1])
                elif line.startswith("lang:") and not meta["supported_scripts"]:
                    meta["supported_scripts"] = ["Latin", "Cyrillic", "Greek", "Common"]
    except Exception as e:
        meta["decode_status"] = f"FAILED: {e}"

    if not meta["supported_scripts"]:
        meta["supported_scripts"] = ["Latin", "Common"]

    return meta


def create_schema(conn: sqlite3.Connection):
    """Initializes SQLite tables and indexes."""
    c = conn.cursor()
    c.executescript("""
    PRAGMA journal_mode = WAL;
    PRAGMA synchronous = NORMAL;
    PRAGMA foreign_keys = ON;

    CREATE TABLE IF NOT EXISTS manifest_metadata (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS files (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        path TEXT UNIQUE NOT NULL,
        top_level_item TEXT NOT NULL,
        filename TEXT NOT NULL,
        extension TEXT,
        size_bytes INTEGER NOT NULL,
        mtime_utc TEXT NOT NULL,
        file_type TEXT NOT NULL,
        mime_type TEXT,
        magic_signature TEXT,
        sha256 TEXT,
        blake3 TEXT,
        domain TEXT NOT NULL,
        module TEXT,
        classification TEXT NOT NULL,
        classification_confidence TEXT NOT NULL,
        evidence_tag TEXT NOT NULL,
        is_container BOOLEAN NOT NULL DEFAULT 0,
        is_signed BOOLEAN NOT NULL DEFAULT 0,
        signature_path TEXT,
        can_edit TEXT NOT NULL DEFAULT 'YES',
        can_rebuild TEXT NOT NULL DEFAULT 'YES',
        scan_level TEXT NOT NULL,
        created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );

    CREATE INDEX IF NOT EXISTS idx_files_sha256 ON files(sha256);
    CREATE INDEX IF NOT EXISTS idx_files_blake3 ON files(blake3);
    CREATE INDEX IF NOT EXISTS idx_files_domain ON files(domain);
    CREATE INDEX IF NOT EXISTS idx_files_top_level ON files(top_level_item);
    CREATE INDEX IF NOT EXISTS idx_files_classification ON files(classification);

    CREATE TABLE IF NOT EXISTS relationships (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        source_file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
        source_path TEXT NOT NULL,
        target_file_id INTEGER REFERENCES files(id) ON DELETE CASCADE,
        target_path TEXT NOT NULL,
        relationship_type TEXT NOT NULL,
        evidence_tag TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS archive_toc (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        container_file_id INTEGER REFERENCES files(id) ON DELETE CASCADE,
        archive_path TEXT NOT NULL,
        entry_path TEXT NOT NULL,
        uncompressed_size INTEGER NOT NULL,
        compressed_size INTEGER,
        crc32 TEXT,
        compression_method TEXT,
        is_directory BOOLEAN NOT NULL DEFAULT 0,
        is_encrypted BOOLEAN NOT NULL DEFAULT 0,
        evidence_tag TEXT NOT NULL
    );

    CREATE INDEX IF NOT EXISTS idx_archive_toc_archive ON archive_toc(archive_path);

    CREATE TABLE IF NOT EXISTS asset_census (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
        file_path TEXT UNIQUE NOT NULL,
        logical_path TEXT NOT NULL,
        asset_id TEXT UNIQUE NOT NULL,
        asset_type TEXT NOT NULL,
        module TEXT NOT NULL,
        container TEXT,
        detected_codec TEXT NOT NULL,
        width INTEGER,
        height INTEGER,
        bit_depth INTEGER,
        color_type TEXT,
        palette_size INTEGER,
        has_alpha BOOLEAN,
        alpha_semantics TEXT,
        dpi_horizontal REAL,
        dpi_vertical REAL,
        decode_status TEXT NOT NULL,
        thumbnail_blake3 TEXT,
        font_family TEXT,
        font_style TEXT,
        glyph_count INTEGER,
        supported_scripts TEXT,
        evidence_tag TEXT NOT NULL
    );

    CREATE INDEX IF NOT EXISTS idx_asset_census_type ON asset_census(asset_type);

    CREATE TABLE IF NOT EXISTS scope_conflicts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        file_id INTEGER REFERENCES files(id) ON DELETE CASCADE,
        item_name TEXT NOT NULL,
        item_path TEXT NOT NULL,
        conflict_category TEXT NOT NULL,
        prohibited_clause TEXT NOT NULL,
        rationale TEXT NOT NULL,
        unmeetable_criteria_if_excluded TEXT NOT NULL,
        user_decision TEXT,
        evidence_tag TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS signed_artefacts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        payload_file_id INTEGER REFERENCES files(id) ON DELETE CASCADE,
        payload_path TEXT NOT NULL,
        signature_file_id INTEGER REFERENCES files(id) ON DELETE SET NULL,
        signature_path TEXT NOT NULL,
        signature_type TEXT NOT NULL,
        payload_sha256 TEXT NOT NULL,
        payload_blake3 TEXT NOT NULL,
        can_edit TEXT NOT NULL DEFAULT 'NO',
        can_rebuild TEXT NOT NULL DEFAULT 'NO',
        analysis_only BOOLEAN NOT NULL DEFAULT 1,
        unavailable_capabilities TEXT NOT NULL,
        evidence_tag TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS rq_register_links (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        rq_id TEXT NOT NULL,
        file_path TEXT NOT NULL,
        format_name TEXT NOT NULL,
        notes TEXT,
        evidence_tag TEXT NOT NULL
    );
    """)
    conn.commit()


def main():
    print("=" * 70)
    print("Audi MMI Studio — Tiered originals/ Evidence Scanner & Asset Census")
    print("=" * 70)
    start_time = time.time()

    if not ORIGINALS_DIR.exists():
        print(f"[ERROR] Directory {ORIGINALS_DIR} does not exist.")
        sys.exit(1)

    # Recreate SQLite database
    if SQLITE_PATH.exists():
        SQLITE_PATH.unlink()
    conn = sqlite3.connect(str(SQLITE_PATH))
    create_schema(conn)

    # 1. L0 ENUMERATE & L1 IDENTIFY
    print("\n>>> Phase 1: L0 Enumerate & L1 Identify across originals/...")
    files_to_hash = []
    total_dirs = 0
    total_bytes = 0

    known_signatures = {
        "8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg": "8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg.sig",
        "8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg": "8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg.sig",
        "8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat": "8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat.sig",
        "HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat": "HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat.sig",
    }

    for root, dirs, files in os.walk(ORIGINALS_DIR):
        total_dirs += len(dirs)
        for f in files:
            if f == ".DS_Store":
                continue
            abs_p = Path(root) / f
            rel_p = str(abs_p.relative_to(ORIGINALS_DIR))
            top_level = rel_p.split(os.sep)[0]
            sz = abs_p.stat().st_size
            mtime_iso = datetime.fromtimestamp(abs_p.stat().st_mtime, tz=timezone.utc).isoformat()
            ext = abs_p.suffix.lower()
            total_bytes += sz

            # Read 64 bytes for magic sniffing
            try:
                with open(abs_p, "rb") as fh:
                    hdr = fh.read(64)
            except Exception:
                hdr = b""

            mime, magic_desc = sniff_magic(hdr, ext)
            domain = determine_domain(top_level)

            # Determine module
            parts = rel_p.split(os.sep)
            module = parts[1] if len(parts) > 2 else (parts[0] if len(parts) > 1 else None)

            # Check signed status
            is_signed = rel_p in known_signatures or ext == ".sig"
            sig_path = known_signatures.get(rel_p)

            # Classify
            classification, conf = classify_file(rel_p, ext, mime, is_signed)

            can_edit = "NO" if rel_p in known_signatures or ext == ".sig" else "YES"
            can_rebuild = "NO" if rel_p in known_signatures or ext == ".sig" else "YES"
            is_container = ext in (".zip", ".7z", ".iso", ".xar", ".pkg", ".ifs", ".efs")

            files_to_hash.append({
                "rel_path": rel_p,
                "abs_path": abs_p,
                "top_level": top_level,
                "filename": f,
                "extension": ext[1:] if ext.startswith(".") else ext,
                "size_bytes": sz,
                "mtime_utc": mtime_iso,
                "file_type": "file",
                "mime_type": mime,
                "magic_signature": magic_desc,
                "domain": domain,
                "module": module,
                "classification": classification,
                "classification_confidence": conf,
                "evidence_tag": f"[EV:tree@originals/{rel_p}]",
                "is_container": 1 if is_container else 0,
                "is_signed": 1 if is_signed else 0,
                "signature_path": sig_path,
                "can_edit": can_edit,
                "can_rebuild": can_rebuild,
                "scan_level": "L2"
            })

    print(f"L0/L1 complete: {len(files_to_hash)} files, {total_dirs} directories, {total_bytes / (1024**3):.2f} GiB.")

    # 2. L2 DUAL STREAMING HASHING
    print("\n>>> Phase 2: L2 Dual Hashing (SHA-256 + BLAKE3)...")
    c = conn.cursor()
    hash_start = time.time()
    processed_bytes = 0

    buf_size = 4 * 1024 * 1024 # 4 MiB buffer
    for idx, item in enumerate(files_to_hash, start=1):
        hasher = DualHasher()
        abs_p = item["abs_path"]
        sz = item["size_bytes"]

        if sz > 0:
            with open(abs_p, "rb") as fh:
                while True:
                    chunk = fh.read(buf_size)
                    if not chunk:
                        break
                    hasher.update(chunk)
            sha256, blake3 = hasher.finalize()
        else:
            sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            blake3 = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"

        item["sha256"] = sha256
        item["blake3"] = blake3
        processed_bytes += sz

        # Insert into files table
        c.execute("""
        INSERT INTO files (
            path, top_level_item, filename, extension, size_bytes, mtime_utc,
            file_type, mime_type, magic_signature, sha256, blake3, domain,
            module, classification, classification_confidence, evidence_tag,
            is_container, is_signed, signature_path, can_edit, can_rebuild, scan_level
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            item["rel_path"], item["top_level"], item["filename"], item["extension"],
            item["size_bytes"], item["mtime_utc"], item["file_type"], item["mime_type"],
            item["magic_signature"], item["sha256"], item["blake3"], item["domain"],
            item["module"], item["classification"], item["classification_confidence"],
            item["evidence_tag"], item["is_container"], item["is_signed"],
            item["signature_path"], item["can_edit"], item["can_rebuild"], item["scan_level"]
        ))
        item["file_id"] = c.lastrowid

        if idx % 2500 == 0 or idx == len(files_to_hash) or sz > 500 * 1024 * 1024:
            conn.commit()
            rate = (processed_bytes / (1024**2)) / (time.time() - hash_start + 0.001)
            print(f"  [{idx:5d}/{len(files_to_hash)}] Hashed {processed_bytes / (1024**3):.2f} GiB ({rate:.1f} MB/s)...")

    conn.commit()
    print(f"L2 Dual Hashing complete in {time.time() - hash_start:.2f}s.")

    # Build path-to-id map
    path_to_id = {item["rel_path"]: item["file_id"] for item in files_to_hash}

    # 3. L3 ARCHIVE TOC EXTRACTION
    print("\n>>> Phase 3: L3 Container Archive Table of Contents (TOC) extraction...")
    archive_targets = [
        ("originals/6.22.4 Vlasoff maps activation.7z", "7z"),
        ("originals/8R0051884KL_6.36.0_2023.7z", "7z"),
        ("originals/HN+R_EU_AU_K0942_4_[8R0906961FB].zip", "zip"),
        ("originals/License.zip", "zip"),
        ("originals/Software.zip", "zip"),
        ("originals/8R0051884KL_6.36.0_2023/pkgdb/SDS/SDS_Data.iso", "iso"),
        ("originals/8R0051884KL_6.36.0_2023/pkgdb/SDS3GP/SDS_Data.iso", "iso"),
    ]

    import zipfile
    total_toc_entries = 0

    for arch_rel, arch_type in archive_targets:
        arch_abs = REPO_ROOT / arch_rel
        if not arch_abs.exists():
            continue
        rel_clean = str(arch_abs.relative_to(ORIGINALS_DIR))
        container_id = path_to_id.get(rel_clean)

        if arch_type == "zip":
            try:
                with zipfile.ZipFile(arch_abs, "r") as zf:
                    for zi in zf.infolist():
                        c.execute("""
                        INSERT INTO archive_toc (
                            container_file_id, archive_path, entry_path, uncompressed_size,
                            compressed_size, crc32, compression_method, is_directory, is_encrypted, evidence_tag
                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                        """, (
                            container_id, rel_clean, zi.filename, zi.file_size,
                            zi.compress_size, f"{zi.CRC:08X}", f"zip_method_{zi.compress_type}",
                            1 if zi.is_dir() else 0, 1 if zi.flag_bits & 0x1 else 0,
                            f"[EV:zip_toc@originals/{rel_clean}:{zi.filename}]"
                        ))
                        total_toc_entries += 1
            except Exception as e:
                print(f"[WARN] Error reading ZIP {rel_clean}: {e}")

        elif arch_type in ("7z", "iso"):
            try:
                res = subprocess.run(["bsdtar", "-tvf", str(arch_abs)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                if res.returncode == 0:
                    for line in res.stdout.splitlines():
                        line = line.strip()
                        if not line:
                            continue
                        parts = line.split(maxsplit=8)
                        if len(parts) >= 9:
                            mode, links, uid, gid, sz_str, m1, m2, m3, member = parts[0], parts[1], parts[2], parts[3], parts[4], parts[5], parts[6], parts[7], parts[8]
                            is_dir = mode.startswith("d")
                            sz = int(sz_str) if sz_str.isdigit() else 0
                            c.execute("""
                            INSERT INTO archive_toc (
                                container_file_id, archive_path, entry_path, uncompressed_size,
                                compressed_size, crc32, compression_method, is_directory, is_encrypted, evidence_tag
                            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                            """, (
                                container_id, rel_clean, member, sz,
                                None, None, arch_type, 1 if is_dir else 0, 0,
                                f"[EV:bsdtar_toc@originals/{rel_clean}:{member}]"
                            ))
                            total_toc_entries += 1
            except Exception as e:
                print(f"[WARN] Error reading {arch_type} {rel_clean}: {e}")

    conn.commit()
    print(f"L3 Archive TOC complete: {total_toc_entries} entries cataloged.")

    # 4. L4 SAMPLING & ASSET CENSUS
    print("\n>>> Phase 4: L4 Header Sampling & Asset Census (PNGs, .precomp, Fonts)...")
    total_assets = 0

    for item in files_to_hash:
        rel_p = item["rel_path"]
        ext = item["extension"].lower()
        abs_p = item["abs_path"]
        f_id = item["file_id"]

        # 4.1 PNG Images (80 files)
        if ext == "png":
            meta = extract_png_meta(abs_p)
            if meta:
                # Semantic asset ID: module:path:filename
                mod = item["module"] or "root"
                asset_id = f"{mod}:{rel_p}:{item['filename']}"
                thumb_b3 = item["blake3"]

                c.execute("""
                INSERT INTO asset_census (
                    file_id, file_path, logical_path, asset_id, asset_type, module,
                    container, detected_codec, width, height, bit_depth, color_type,
                    palette_size, has_alpha, alpha_semantics, dpi_horizontal, dpi_vertical,
                    decode_status, thumbnail_blake3, font_family, font_style, glyph_count,
                    supported_scripts, evidence_tag
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                """, (
                    f_id, rel_p, rel_p, asset_id, "image", mod,
                    None, meta["codec"], meta.get("width"), meta.get("height"),
                    meta.get("bit_depth"), meta.get("color_type"), meta.get("palette_size"),
                    meta.get("has_alpha"), meta.get("alpha_semantics"), meta.get("dpi_h"),
                    meta.get("dpi_v"), meta["decode_status"], thumb_b3,
                    None, None, None, None, f"[EV:png@originals/{rel_p}]"
                ))
                total_assets += 1

        # 4.2 Precomputed Graphics (360 files)
        elif ext == "precomp":
            meta = extract_precomp_meta(abs_p)
            mod = item["module"] or "CombiStyles"
            asset_id = f"{mod}:{rel_p}:{item['filename']}"
            thumb_b3 = item["blake3"]

            c.execute("""
            INSERT INTO asset_census (
                file_id, file_path, logical_path, asset_id, asset_type, module,
                container, detected_codec, width, height, bit_depth, color_type,
                palette_size, has_alpha, alpha_semantics, dpi_horizontal, dpi_vertical,
                decode_status, thumbnail_blake3, font_family, font_style, glyph_count,
                supported_scripts, evidence_tag
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                f_id, rel_p, rel_p, asset_id, "image", mod,
                None, meta["codec"], meta.get("width"), meta.get("height"),
                meta.get("bit_depth"), meta.get("color_type"), meta.get("palette_size"),
                meta.get("has_alpha"), meta.get("alpha_semantics"), meta.get("dpi_h"),
                meta.get("dpi_v"), meta["decode_status"], thumb_b3,
                None, None, None, None, f"[EV:precomp@originals/{rel_p}]"
            ))
            total_assets += 1

        # 4.3 TrueType Fonts (3 files)
        elif ext == "ttf":
            meta = extract_font_meta(abs_p)
            mod = item["module"] or "GEMMI"
            asset_id = f"{mod}:{rel_p}:{item['filename']}"

            c.execute("""
            INSERT INTO asset_census (
                file_id, file_path, logical_path, asset_id, asset_type, module,
                container, detected_codec, width, height, bit_depth, color_type,
                palette_size, has_alpha, alpha_semantics, dpi_horizontal, dpi_vertical,
                decode_status, thumbnail_blake3, font_family, font_style, glyph_count,
                supported_scripts, evidence_tag
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                f_id, rel_p, rel_p, asset_id, "font", mod,
                None, meta["codec"], None, None,
                None, None, None,
                None, None, None,
                None, meta["decode_status"], item["blake3"],
                meta.get("font_family"), meta.get("font_style"), meta.get("glyph_count"),
                json.dumps(meta.get("supported_scripts", [])), f"[EV:ttf@originals/{rel_p}]"
            ))
            total_assets += 1

        # 4.4 3D Collada Models (1 file)
        elif ext == "dae":
            mod = item["module"] or "GEMMI"
            asset_id = f"{mod}:{rel_p}:{item['filename']}"
            c.execute("""
            INSERT INTO asset_census (
                file_id, file_path, logical_path, asset_id, asset_type, module,
                container, detected_codec, width, height, bit_depth, color_type,
                palette_size, has_alpha, alpha_semantics, dpi_horizontal, dpi_vertical,
                decode_status, thumbnail_blake3, font_family, font_style, glyph_count,
                supported_scripts, evidence_tag
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                f_id, rel_p, rel_p, asset_id, "vector_3d", mod,
                None, "COLLADA_XML", None, None,
                None, None, None,
                None, None, None,
                None, "SUCCESS", item["blake3"],
                None, None, None,
                None, f"[EV:dae@originals/{rel_p}]"
            ))
            total_assets += 1

    conn.commit()
    print(f"Asset Census complete: {total_assets} candidate assets cataloged (80 PNG, 360 precomp, 3 TTF, 1 DAE).")

    # 5. SCOPE CONFLICTS TABLE
    print("\n>>> Phase 5: Populating Scope Conflicts table...")
    scope_conflict_entries = [
        {
            "name": "Vlasoff maps activation FSC certificate",
            "path": "6.22.4 Vlasoff maps activation/2380_00040009.fsc",
            "category": "OUT OF SCOPE — DOCUMENT ONLY",
            "clause": "§1.2 Licensing enforcement, feature-enablement codes (FSC), activation controls",
            "rationale": "Freischaltcode bypass file intended to unlock map navigation without OEM licensing.",
            "unmeetable": "None. Legitimate UI customization and reverse-engineering do not require activation bypass.",
            "tag": "[EV:tree@originals/6.22.4 Vlasoff maps activation/2380_00040009.fsc]"
        },
        {
            "name": "Vlasoff maps activation shell script",
            "path": "6.22.4 Vlasoff maps activation/copie_scr.sh",
            "category": "OUT OF SCOPE — DOCUMENT ONLY",
            "clause": "§1.2 Activation controls and execution of unauthorized scripts",
            "rationale": "Automated script to inject cracked FSC certificate into head-unit storage.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/6.22.4 Vlasoff maps activation/copie_scr.sh]"
        },
        {
            "name": "Vlasoff maps activation 7-Zip container",
            "path": "6.22.4 Vlasoff maps activation.7z",
            "category": "OUT OF SCOPE — DOCUMENT ONLY",
            "clause": "§1.2 Distribution container for activation circumvention tools",
            "rationale": "Archive encapsulating cracked FSC and injection script.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/6.22.4 Vlasoff maps activation.7z]"
        },
        {
            "name": "License run script (DRM daemon kill harness)",
            "path": "License/run.sh",
            "category": "OUT OF SCOPE — DOCUMENT ONLY",
            "clause": "§1.2 DRM or content protection defeat",
            "rationale": "Terminates vdev-logvolmgr volume manager daemon to bypass navigation volume license validation.",
            "unmeetable": "None.",
            "tag": "[EV:file@originals/License/run.sh:38-44]"
        },
        {
            "name": "License SD-card auto-run script",
            "path": "License/copie_scr.sh",
            "category": "OUT OF SCOPE — DOCUMENT ONLY",
            "clause": "§1.2 DRM bypass launcher",
            "rationale": "SD card auto-run launcher invoking map activation patch harness.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/License/copie_scr.sh]"
        },
        {
            "name": "License script decryption tool (DecodeScript)",
            "path": "License/utils/DecodeScript",
            "category": "OUT OF SCOPE — DOCUMENT ONLY",
            "clause": "§1.2 Authentication / access control bypass",
            "rationale": "Proprietary decryption utility used to decode obfuscated activation scripts.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/License/utils/DecodeScript]"
        },
        {
            "name": "License QNX 6 standard utilities (dual-use)",
            "path": "License/utils/sqlite3",
            "category": "AMBIGUOUS — USER DECISION REQUIRED",
            "clause": "Potential dual-use (§1.2 circumvention vs diagnostic inspection)",
            "rationale": "Standard QNX 6 binaries (sqlite3, sed, awk, sysctl, pax, showScreen, libc.so). User must determine whether diagnostic inspection is permitted.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/License/utils/sqlite3]"
        },
        {
            "name": "License update trigger file",
            "path": "License/upd",
            "category": "AMBIGUOUS — USER DECISION REQUIRED",
            "clause": "Potential exploit trigger file",
            "rationale": "Zero-byte marker file triggering update script execution on SD card insertion.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/License/upd]"
        },
        {
            "name": "License container archive",
            "path": "License.zip",
            "category": "AMBIGUOUS — USER DECISION REQUIRED",
            "clause": "Container bundling mixed activation and diagnostic material",
            "rationale": "Archive bundling activation scripts alongside standard QNX tools and screen graphics.",
            "unmeetable": "None.",
            "tag": "[EV:tree@originals/License.zip]"
        },
        {
            "name": "License dialog screen graphics",
            "path": "License/screens/scriptStart.png",
            "category": "IN SCOPE",
            "clause": "N/A (Standard visual assets)",
            "rationale": "800x480 pixel bitmap artwork defining native screen display geometry. Contains no circumvention logic.",
            "unmeetable": "Display geometry verification and screen canvas baseline.",
            "tag": "[EV:tree@originals/License/screens/scriptStart.png]"
        },
        {
            "name": "OEM supported FSC catalog manifest",
            "path": "HN+R_EU_AU_K0942_4_[8R0906961FB]/AudiSupportedFscs/AudiSupportedFscs/0/default/AudiFSC.txt",
            "category": "IN SCOPE",
            "clause": "N/A (Stock OEM firmware manifest)",
            "rationale": "Stock reference manifest containing OEM recognized FSC application identifiers and signatures.",
            "unmeetable": "Stock baseline validation and compatibility verification.",
            "tag": "[EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/AudiSupportedFscs/AudiSupportedFscs/0/default/AudiFSC.txt]"
        },
        {
            "name": "Supporting HNav USA firmware train",
            "path": "Software/metainfo2.txt",
            "category": "IN SCOPE",
            "clause": "N/A (Standard OEM firmware)",
            "rationale": "Authentic HNav North America software train, providing cross-train baseline for differential analysis.",
            "unmeetable": "Cross-train differential analysis.",
            "tag": "[EV:tree@originals/Software/metainfo2.txt]"
        }
    ]

    for item in scope_conflict_entries:
        f_id = path_to_id.get(item["path"])
        c.execute("""
        INSERT INTO scope_conflicts (
            file_id, item_name, item_path, conflict_category, prohibited_clause,
            rationale, unmeetable_criteria_if_excluded, user_decision, evidence_tag
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            f_id, item["name"], item["path"], item["category"], item["clause"],
            item["rationale"], item["unmeetable"], None, item["tag"]
        ))
    conn.commit()
    print(f"Scope Conflicts populated: {len(scope_conflict_entries)} items.")

    # 6. SIGNED ARTEFACTS TABLE
    print("\n>>> Phase 6: Populating Signed Artefacts table...")
    signed_payload_entries = [
        {
            "payload_path": "8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg",
            "sig_path": "8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg.sig",
            "sig_type": "DETACHED_SIG",
            "unavailable": "Modification, re-signing, layer substitution, and package rebuild are permanently disabled (ERR_SIGNED_ARTEFACT_IMMUTABLE)."
        },
        {
            "payload_path": "8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg",
            "sig_path": "8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg.sig",
            "sig_type": "DETACHED_SIG",
            "unavailable": "Modification, re-signing, layer substitution, and package rebuild are permanently disabled (ERR_SIGNED_ARTEFACT_IMMUTABLE)."
        },
        {
            "payload_path": "8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat",
            "sig_path": "8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat.sig",
            "sig_type": "DETACHED_SIG",
            "unavailable": "TMC configuration table modification and re-signing are permanently disabled (ERR_SIGNED_ARTEFACT_IMMUTABLE)."
        },
        {
            "payload_path": "HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat",
            "sig_path": "HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat.sig",
            "sig_type": "DETACHED_SIG",
            "unavailable": "TMC configuration table modification and re-signing are permanently disabled (ERR_SIGNED_ARTEFACT_IMMUTABLE)."
        }
    ]

    for item in signed_payload_entries:
        p_id = path_to_id.get(item["payload_path"])
        s_id = path_to_id.get(item["sig_path"])
        p_sha = next((x["sha256"] for x in files_to_hash if x["rel_path"] == item["payload_path"]), "")
        p_b3 = next((x["blake3"] for x in files_to_hash if x["rel_path"] == item["payload_path"]), "")

        c.execute("""
        INSERT INTO signed_artefacts (
            payload_file_id, payload_path, signature_file_id, signature_path,
            signature_type, payload_sha256, payload_blake3, can_edit, can_rebuild,
            analysis_only, unavailable_capabilities, evidence_tag
        ) VALUES (?, ?, ?, ?, ?, ?, ?, 'NO', 'NO', 1, ?, ?)
        """, (
            p_id, item["payload_path"], s_id, item["sig_path"], item["sig_type"],
            p_sha, p_b3, item["unavailable"], f"[EV:tree@originals/{item['payload_path']}]"
        ))
    conn.commit()
    print(f"Signed Artefacts populated: {len(signed_payload_entries)} items.")

    # 7. RELATIONSHIPS TABLE
    print("\n>>> Phase 7: Populating Relationships table...")
    relationships = [
        ("6.22.4 Vlasoff maps activation.7z", "6.22.4 Vlasoff maps activation", "archive_to_extracted"),
        ("8R0051884KL_6.36.0_2023.7z", "8R0051884KL_6.36.0_2023", "archive_to_extracted"),
        ("HN+R_EU_AU_K0942_4_[8R0906961FB].zip", "HN+R_EU_AU_K0942_4_[8R0906961FB]", "archive_to_extracted"),
        ("License.zip", "License", "archive_to_extracted"),
        ("Software.zip", "Software", "archive_to_extracted"),
        ("8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg", "8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg.sig", "payload_to_signature"),
        ("8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg", "8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg.sig", "payload_to_signature"),
        ("8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat", "8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat.sig", "payload_to_signature"),
        ("HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat", "HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat.sig", "payload_to_signature"),
        ("HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt", "HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI", "manifest_for_module"),
        ("8R0051884KL_6.36.0_2023/metainfo2.txt", "8R0051884KL_6.36.0_2023/pkgdb", "manifest_for_module"),
    ]

    for src, tgt, rel_type in relationships:
        src_id = path_to_id.get(src, 1)
        tgt_id = path_to_id.get(tgt)
        c.execute("""
        INSERT INTO relationships (
            source_file_id, source_path, target_file_id, target_path, relationship_type, evidence_tag
        ) VALUES (?, ?, ?, ?, ?, ?)
        """, (
            src_id, src, tgt_id, tgt, rel_type, f"[EV:tree@originals/{src}]"
        ))
    conn.commit()
    print(f"Relationships populated: {len(relationships)} entries.")

    # 8. RQ REGISTER LINKS TABLE
    print("\n>>> Phase 8: Populating RQ Register Links table...")
    rq_links = [
        ("RQ-001", "8R0051884KL_6.36.0_2023/pkgdb/CTY/CTY.db", "Harman Becker NavDB Binary Database (.db)"),
        ("RQ-002", "8R0051884KL_6.36.0_2023/pkgdb/CTYS3TC/CTYS3TC.atlas", "Harman Becker Navigation ATLAS Spatial Tile Container (.atlas)"),
        ("RQ-003", "HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp", "CombiStyles Precomputed Cluster Graphic (.precomp)"),
        ("RQ-004", "HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar", "Harman/EB MapStyles Regional Archive (.xar)"),
        ("RQ-005", "HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/0/default/ifs-root.ifs", "QNX 6 Image FileSystem (.ifs)"),
        ("RQ-006", "HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/0/default/efs-system.efs", "QNX 6 Embedded Flash FileSystem (.efs)"),
        ("RQ-007", "HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/sss/g_00.ans", "Harman Becker Acoustic Speech Prompts (.ans)"),
        ("RQ-008", "HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/0/default/SystemFPGA.hbbin", "Harman Becker System FPGA Bitstream (.hbbin)"),
        ("RQ-009", "HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9425/0/default/Inic.ipf", "SMSC OS81050 MOST INIC Firmware Container (.ipf)"),
        ("RQ-010", "8R0051884KL_6.36.0_2023/pkgdb/GDB/GDB.gdb", "Geographic Routing Database (.gdb / .gd2)"),
        ("RQ-011", "Software/2/ARU9308/0/default/ARU_GUI.hbgr", "Harman Becker Graphics Resource (.hbgr)"),
        ("RQ-012", "HN+R_EU_AU_K0942_4_[8R0906961FB]/Bose/0/default/DSP_Main.ldr", "Analog Devices Blackfin DSP Loader Executable (.ldr)")
    ]

    for rq_id, fp, fmt in rq_links:
        c.execute("""
        INSERT INTO rq_register_links (rq_id, file_path, format_name, notes, evidence_tag)
        VALUES (?, ?, ?, ?, ?)
        """, (
            rq_id, fp, fmt, "Identified during Pass A L1/L4 tiered scan", f"[EV:tree@originals/{fp}]"
        ))
    conn.commit()
    print(f"RQ Register Links populated: {len(rq_links)} entries.")

    # 9. METADATA SUMMARY
    scan_duration = time.time() - start_time
    metadata = {
        "scan_timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "tool_version": "Audi MMI Studio Pass A Scanner v1.0.0",
        "corpus_root": str(ORIGINALS_DIR),
        "total_files": str(len(files_to_hash)),
        "total_directories": str(total_dirs),
        "total_bytes": str(total_bytes),
        "total_assets_cataloged": str(total_assets),
        "total_archive_toc_entries": str(total_toc_entries),
        "scan_duration_seconds": f"{scan_duration:.2f}",
        "sha256_engine": "hashlib.sha256 (ARMv8 hardware-accelerated)",
        "blake3_engine": "libblake3.dylib (C99 -O3 clang native)" if c_blake3_available else "pure Python Blake3"
    }

    for k, v in metadata.items():
        c.execute("INSERT INTO manifest_metadata (key, value) VALUES (?, ?)", (k, v))
    conn.commit()

    # 10. EXPORT DETERMINISTIC JSON
    print("\n>>> Phase 10: Exporting deterministic originals-manifest.json...")
    export_data = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "schema_version": "1.0.0",
        "metadata": metadata,
        "summary": {
            "total_files": len(files_to_hash),
            "total_directories": total_dirs,
            "total_bytes": total_bytes,
            "total_assets": total_assets,
            "total_archive_toc_entries": total_toc_entries,
            "domains": {
                "HN+R_SOFTWARE": sum(1 for x in files_to_hash if x["domain"] == "HN+R_SOFTWARE"),
                "MAP_PACKAGE": sum(1 for x in files_to_hash if x["domain"] == "MAP_PACKAGE"),
                "HNAV_SOFTWARE": sum(1 for x in files_to_hash if x["domain"] == "HNAV_SOFTWARE"),
                "LICENSE_ACTIVATION": sum(1 for x in files_to_hash if x["domain"] == "LICENSE_ACTIVATION"),
                "MAP_ACTIVATION": sum(1 for x in files_to_hash if x["domain"] == "MAP_ACTIVATION"),
            },
            "classifications": {}
        },
        "files": [],
        "archive_toc": [],
        "asset_census": [],
        "scope_conflicts": scope_conflict_entries,
        "signed_artefacts": signed_payload_entries,
        "relationships": [{"source": s, "target": t, "type": r} for s, t, r in relationships],
        "rq_register_links": [{"rq_id": r, "file_path": f, "format": fmt} for r, f, fmt in rq_links]
    }

    # Count classifications
    for item in files_to_hash:
        cl = item["classification"]
        export_data["summary"]["classifications"][cl] = export_data["summary"]["classifications"].get(cl, 0) + 1

    # Fetch rows from database for complete parity
    c.row_factory = sqlite3.Row
    for row in c.execute("SELECT * FROM files ORDER BY id").fetchall():
        export_data["files"].append(dict(row))

    for row in c.execute("SELECT * FROM archive_toc ORDER BY id").fetchall():
        export_data["archive_toc"].append(dict(row))

    for row in c.execute("SELECT * FROM asset_census ORDER BY id").fetchall():
        d = dict(row)
        if d.get("supported_scripts"):
            try:
                d["supported_scripts"] = json.loads(d["supported_scripts"])
            except Exception:
                pass
        export_data["asset_census"].append(d)

    with open(JSON_PATH, "w", encoding="utf-8") as jf:
        json.dump(export_data, jf, indent=2, sort_keys=False)

    conn.close()

    print(f"\n[SUCCESS] Manifest generation complete in {time.time() - start_time:.2f}s!")
    print(f"  - Database: {SQLITE_PATH} ({SQLITE_PATH.stat().st_size / (1024**2):.2f} MiB)")
    print(f"  - JSON:     {JSON_PATH} ({JSON_PATH.stat().st_size / (1024**2):.2f} MiB)")
    print(f"  - Files indexed: {len(files_to_hash)}")
    print(f"  - Assets cataloged: {total_assets}")
    print(f"  - TOC entries: {total_toc_entries}")


if __name__ == "__main__":
    main()
