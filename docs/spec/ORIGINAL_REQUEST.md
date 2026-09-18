# Original User Request

## Initial Request — 2026-09-18T08:58:30Z

# Teamwork Project Prompt — Draft

> Status: Launched
> Goal: Craft prompt → get user approval → delegate to teamwork_preview
> Requested team: Use a full team of agents.

Execute Pass A (Evidence & Census Audit) and produce the foundation for Pass B of **Audi MMI Studio** in accordance with `AUDI_MMI_STUDIO_AGENT_PROMPT.md`, treating `originals/` as an immutable evidence repository.

Working directory: `/Users/gerald/Antigravity/AudiMMI`
Integrity mode: development

## Requirements

### R1. Greenfield Verification & Tiered Scan (Pass A)
Perform repository verification confirming greenfield status (§0.0). Conduct a tiered scan (L0 Enumerate, L1 Identify, L2 Hash SHA-256/BLAKE3, L3 Table of Contents without extraction, L4 Sample ≤256 MiB / 5,000 files prioritizing metadata, headers, font tables, and visual assets). Never modify or create files within `originals/`.

### R2. Storage Manifest & Findings Ledger
Generate `originals-manifest.sqlite` capturing file metadata, BLAKE3 and SHA-256 hashes, detected mime/container types, and relationships, accompanied by a JSON export. Catalog all unverified or ambiguous structures into `docs/research/RQ-REGISTER.md`.

### R3. Asset Census
Catalog all candidate visual and font assets across `originals/` during sampling, capturing path, module, container, detected codec, dimensions, bit depth, palette details, and decode status into the census section of `SOURCE_AUDIT.md`.

### R4. Scope Conflict & Signed Artefact Gating
Produce `SOURCE_AUDIT.md` containing:
- § Scope Conflicts: catalog activation/licensing materials (`6.22.4 Vlasoff maps activation`, `License/`), classifying each item (IN SCOPE / OUT OF SCOPE — DOCUMENT ONLY / AMBIGUOUS — USER DECISION REQUIRED) without implementing bypasses.
- § Signed Artefacts: list all payloads with detached or embedded signatures (e.g. `MMI3G_ECE_Hi_R_6_36_0.pkg`, `MMI3GP_ECE_Hi_R_6_36_0.pkg`), enforcing `canEdit = NO` and `canRebuild = NO`.
- Stop after Pass A for explicit user determination on scope conflicts before proceeding to Pass B (`PROJECT_PLAN.md`).

### R5. Evidence Tagging & Compliance Check
Enforce evidence tagging discipline (`[EV:...]`, `[INF:...]`, `[UNK]`) across all generated documentation and audit deliverables. Implement and run `scripts/check-evidence-tags.py` to ensure zero untagged normative statements exist.

## Acceptance Criteria

### Corpus Safety & Immutability
- [ ] `originals/` is strictly read-only; no files created, modified, renamed, or deleted within `originals/`.
- [ ] Hashes (SHA-256 and BLAKE3) captured and stored in `originals-manifest.sqlite`.

### Evidence Deliverables
- [ ] `SOURCE_AUDIT.md` generated with complete sections for Repository Status, Discovered Domains, § Scope Conflicts, § Signed Artefacts, and § Asset Census.
- [ ] `docs/research/RQ-REGISTER.md` created with initial research questions for every unknown container/format.
- [ ] `originals-manifest.sqlite` and exported JSON exist and match.

### Scope & Policy Guardrails
- [ ] Signed payloads marked analysis-only; zero write or rebuild paths specified for signed artefacts.
- [ ] Clear categorization of activation/licensing material without implementing DRM or activation bypasses.
- [ ] Tag verification script `scripts/check-evidence-tags.py` passes on all generated documentation.
- [ ] Execution stops after Pass A deliverables are complete for user review prior to Pass B plan formulation.
