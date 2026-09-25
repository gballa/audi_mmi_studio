//! mmi-formats: Binary format adapters, metainfo2 parser, precomp codec, and Kaitai bindings.

pub mod adapter;
pub mod opaque_span;
pub mod metainfo2;
pub mod precomp;
pub mod mapstyle_xar;
pub mod hb_navdb;
pub mod hb_atlas;
pub mod qnx_ifs;
pub mod qnx_efs;
pub mod hb_ans;
pub mod hb_fpga;
pub mod smsc_ipf;
pub mod hb_gdb;
pub mod hb_grammar;
pub mod hb_lit;
pub mod adi_ldr;
pub mod script_cipher;
pub mod gate;

pub use adapter::{FormatAdapter, FormatCapabilities};
pub use script_cipher::{Mmi3gScriptCipher, SEED_INIT};
pub use opaque_span::OpaqueByteSpan;
pub use metainfo2::{MetaInfo2, MetaInfo2Adapter, MetaInfo2Builder, crc32_ieee, generate_block_crcs};
pub use precomp::{PrecompImage, PrecompAdapter, HEADER_SIZE, PRECOMP_MAGIC};
pub use mapstyle_xar::{MapStyleXar, MapStyleXarAdapter};
pub use hb_navdb::{HbNavDb, HbNavDbAdapter, HbNavDbHeader, FLDB_MAGIC};
pub use hb_lit::{
    compute_rotary_alpha_mask, LitDatabaseReader, LitSpellerBranch, LitSpellerNode,
    LitStreetRecord, LIT_ALPHA_MASK_A_Z, LIT_ALPHA_MASK_DIGITS, LIT_ALPHA_MASK_SPACE,
    LIT_ALPHA_MASK_SPECIAL,
};
pub use hb_atlas::{
    AtlasTile, AtlasTileIndexEntry, AtlasWriter, HbAtlas, HbAtlasAdapter, HbAtlasHeader,
    ATLAS_DEFAULT_BLOCK_SIZE, ATLAS_HEADER_MIN_SIZE, ATLAS_MAGIC_TAG,
};
pub use qnx_ifs::{QnxIfs, QnxIfsAdapter, QnxIfsHeader, QnxIfsBuilder, QNX_IFS_MAGIC, MAX_IFS_ROOT_SIZE, QNX_MACHINE_SH4};
pub use qnx_efs::{QnxEfs, QnxEfsAdapter, QnxEfsHeader, QnxEfsBuilder, QNX_F3S_MAGIC, MAX_EFS_SYSTEM_SIZE};
pub use hb_ans::{HbAns, HbAnsAdapter, HbAnsHeader, ANS_MAGIC};
pub use hb_fpga::{HbFpga, HbFpgaAdapter, HbFpgaHeader, FPGA_HDG_TAG};
pub use smsc_ipf::{SmscIpf, SmscIpfAdapter, SmscIpfHeader, SMSC_IPF_MAGIC};
pub use hb_gdb::{HbGdb, HbGdbAdapter, HbGdbHeader, GDB_MAGIC};
pub use hb_grammar::{HbGrammar, HbGrammarAdapter, HbGrammarHeader, HBGR_SIG};
pub use adi_ldr::{AdiLdr, AdiLdrAdapter, AdiLdrHeader, ADI_LDR_MAGIC};
pub use gate::{GateReport, GateVerdict, IdentityRebuildGate};
