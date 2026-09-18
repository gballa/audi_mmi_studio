# Reference — Glossary of Terms & Acronyms

This glossary defines project-specific, automotive, and reverse-engineering terms used throughout **Audi MMI Studio**.

---

## Terms & Acronyms

### Automotive & MMI Systems
- **CAS**: Content-Addressed Storage. A data store where objects are addressed and retrieved solely by their cryptographic hash (BLAKE3).
- **GEM**: Green Engineering Menu. An in-vehicle developer diagnostic menu on Audi MMI 3G head units invoked by holding `[CAR] + [MENU]` for 5 seconds.
- **HBNavDB**: Harman/Becker Navigation Database. The proprietary spatial database (`.db`) with `FLDB` header used by MMI 3G navigation.
- **HN+ / HN+R**: Software train prefixes for Audi MMI 3G High (`HN+`) and MMI 3G Plus (`HN+R`).
- **IFS**: Image Filesystem. A QNX standalone bootable filesystem image containing startup instructions, kernel (`procnto`), and drivers.
- **INIC**: Intelligent Network Interface Controller. Hardware controller chip (e.g. SMSC OS81050) managing the vehicle MOST optical fiber network.
- **IPL**: Initial Program Loader. The low-level bootloader on Audi head units responsible for checking memory integrity and executing the QNX IFS kernel.
- **MOST**: Media Oriented Systems Transport. High-speed optical bus used in European vehicles for audio, video, and control data.
- **MU9411**: Main Unit software variant code for Audi MMI 3G High/Plus head units.
- **Precomp**: Proprietary zlib-compressed RGBA/RGB565 bitmap image container (`.precomp`) used for dashboard and UI screen styling.
- **REM**: Red Engineering Menu. The head unit update bootloader menu invoked by holding `[CAR] + [BACK]` to initiate firmware flashing from SD cards.
- **Software Train**: The formal firmware release identifier (e.g. `HN+R_EU_AU_K0942_4_[8R0906961FB]`) indicating hardware model, geographic region, and software revision.
- **StageStore**: Isolated copy-on-write workspace where candidate modifications and theme recipes are applied without touching `originals/`.
- **UART**: Universal Asynchronous Receiver-Transmitter. Hardware serial interface accessible via the MMI Quadlock connector providing a low-level QNX root shell at 115200 baud for emergency recovery.
- **XAR**: eXtensible ARchive (`rax\0`). Container format used for 3D map rendering textures and XML vector style rules.
