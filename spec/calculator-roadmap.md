# Clinical Calculator Roadmap

The clinical-calculator backlog, ordered by build priority. The original UK-first 50-tool list is below in Tiers 1-5; further candidates surfaced from sibling projects are under [Future candidates](#future-candidates). Engineering and infrastructure work that is not a new calculator lives in its own section at the end.

## Status legend

- `[x]` **Completed** - the calculator is shipped in `calc-core::all()` and returns a real score against literature vectors.
- `[x]` (stub) - **Completed as a proprietary / unavailable stub**. The calculator is listed (`calc list`) and computable, but returns a structured explanation instead of a score because its algorithm or content is licence-locked. Named on purpose; the open alternative is pointed to in the response.
- `[~]` **In-progress** - actively being implemented or under review.
- `[ ]` **Planned** - committed to build; the next batch.
- Items in [Future candidates](#future-candidates) are explicitly **under consideration** rather than committed - they get promoted to `Planned` when scheduled.

---

## Tier 1 - High-volume primary care / NHS-mandated

- [x] **QRISK3** - 10-year CVD risk - NICE NG238
- [x] **PHQ-9** - Depression severity & monitoring - NICE NG222
- [x] **GAD-7** - Anxiety severity - NICE CG113
- [x] **AUDIT** - Alcohol misuse screening (10-item) - NICE CG115
- [x] **AUDIT-C** - Brief alcohol screen (3-item) - NICE CG115
- [x] **eGFR (CKD-EPI 2021)** - Kidney function / CKD staging - NICE NG203
- [x] (stub) **MUST** - Malnutrition risk - NICE CG32
- [x] (stub) **FRAX** - 10-year osteoporotic fracture risk - NICE CG146
- [x] **QFracture** - Fracture risk (UK-native alternative to FRAX) - NICE CG146
- [x] **FIB-4** - Liver fibrosis screening (NAFLD/MASLD) - NICE NG49

## Tier 2 - Acute / emergency (high patient-safety impact)

- [x] **NEWS2** - Acute illness severity / sepsis trigger - NICE NG51; RCP / NHS England mandated
- [x] **CURB-65 / CRB-65** - Pneumonia severity; admission decision - NICE NG138; BTS
- [x] **Wells DVT** - Pre-test probability of DVT - NICE NG158
- [x] **Wells PE** - Pre-test probability of PE - NICE NG158
- [x] **GRACE** - In-hospital mortality in NSTEMI/ACS - NICE NG185, CG94
- [x] **CHA₂DS₂-VASc** - Stroke risk in AF; anticoagulation decision - NICE NG196
- [x] **HAS-BLED** - Bleeding risk in anticoagulated patients - NICE NG196, NG158
- [x] **ABCD2** - Short-term stroke risk after TIA - NICE NG128
- [x] **4AT** - Delirium screening at bedside - NICE CG103
- [x] **qSOFA** - Rapid sepsis identification - NICE NG51 (Sepsis-3)

## Tier 3 - Common chronic-disease management

- [x] **MRC Dyspnoea** - COPD breathlessness grading - NICE NG115; BTS
- [x] (stub) **CAT** - COPD symptom burden (GOLD classification) - NICE NG115
- [x] (stub) **ACQ** - Asthma control monitoring - BTS/NICE/SIGN asthma 2024
- [x] **IPSS** - Prostate / LUTS severity - NICE CG97
- [x] **DAS28** - Rheumatoid arthritis disease activity - NICE NG100
- [x] **uACR** - Albuminuria; CKD/diabetic nephropathy staging - NICE NG203, NG28
- [x] **eGFR + uACR heatmap** (`ckd_risk`) - KDIGO CKD risk category (G×A grid) - NICE NG203
- [x] **EPDS** - Perinatal depression / anxiety - NICE CG192; SIGN 169
- [x] (stub) **CFS** (Clinical Frailty Scale) - Frailty severity 1-9 - NICE NG56; NHS England
- [x] (stub) **MMSE** - Cognitive function / dementia monitoring - NICE NG97

## Tier 4 - Secondary care / specialist

- [x] **SOFA** - Organ dysfunction in ICU / sepsis definition - NICE NG51; Sepsis-3
- [x] **EuroSCORE II** - Operative mortality risk for cardiac surgery - NICE TA163, TA245
- [x] **HEART** - ED chest-pain risk stratification - NICE NG185
- [x] **TIMI** - UA/NSTEMI risk (simpler alternative to GRACE) - NICE CG94
- [x] **Padua Prediction Score** - VTE risk in medical inpatients - NICE NG89
- [x] (stub) **ELF Test** - Liver fibrosis second-line (serum biomarkers) - NICE NG49
- [x] **Child-Pugh** - Cirrhosis severity - NICE NG50
- [x] **MELD** (original 2001) - End-stage liver disease / transplant urgency - NICE NG50
- [x] **UKELD** - UK transplant listing threshold - NICE NG50; NHS Blood and Transplant
- [x] **Nottingham Hip Fracture Score** (NHFS) - 30-day mortality post hip fracture - NICE CG124; NHFD

## Tier 5 - Functional / PROMs / niche but guideline-endorsed

- [x] **AMTS** - Brief cognitive screen (embedded in NHFS) - NICE CG124
- [x] **Waterlow** - Pressure-ulcer risk - NICE CG179
- [x] (stub) **Oxford Hip Score** (OHS) - Patient-reported hip replacement outcome - NHS England PROMs
- [x] (stub) **Oxford Knee Score** (OKS) - Patient-reported knee replacement outcome - NHS England PROMs
- [x] **BODE Index** - COPD mortality prognostication - NICE NG115; BTS
- [x] (stub) **LANSS** - Neuropathic vs. nociceptive pain - NICE CG173
- [x] **ABPI** - Peripheral arterial disease / compression suitability - NICE NG19, CG168
- [x] **Gleason Grade Groups** - Prostate cancer aggressiveness - NICE NG131
- [x] **Nottingham Prognostic Index** - Breast cancer prognosis - NICE NG101
- [x] **CHALICE Rule** - CT head decision in paediatric head injury - NICE CG176; NG232

!!! success "Tier 1-5 complete (50 / 50)"
    Every entry on the original UK-first list ships in some form. The 10 marked `(stub)` are intentional "unavailable - proprietary" entries: they are listed, schemable, and embeddable, and invoking them returns a structured explanation plus the named open alternative rather than a score. See [`docs/calculators.md`](../docs/calculators.md#unavailable-on-principle).

---

## Future candidates

Calculators worth shipping but not on the original UK-first 50-tool list. They mainly originate from [MedikQuantis](https://medikquantis.me) (Laura Piró, Barcelona; MIT licensed) - a sibling project we are exploring a collaboration with. The full table with descriptions and clinical rationale lives in [`docs/calculators.md`](../docs/calculators.md#wishlist-candidates-for-future-addition).

### Future - universally-cited bedside scores

- [ ] **GCS** (Glasgow Coma Scale) - consciousness level
- [ ] **NIHSS** - acute stroke severity
- [ ] **Charlson Comorbidity Index** - 10-year mortality from comorbidities
- [ ] **APACHE II** - ICU severity / mortality
- [ ] **ASA Physical Status** - peri-operative classification
- [ ] **NYHA** - heart-failure functional class
- [ ] **MELD 3.0** - updated MELD (we ship the 2001 original)
- [ ] **CHA₂DS₂-VA** - 2024 ESC sex-free update of CHA₂DS₂-VASc
- [ ] **PERC** - PE rule-out criteria
- [ ] **Glasgow-Blatchford** - upper-GI bleed pre-endoscopy triage
- [ ] **Centor / McIsaac** - strep pharyngitis probability

### Future - single-formula clinical helpers

- [ ] **Albumin-corrected calcium** (Payne 1973)
- [ ] **Hyperglycaemia-corrected sodium** (Katz / Hillier)
- [ ] **Anion gap** - Na − (Cl + HCO₃), screens for HAGMA
- [ ] **FENa** - prerenal vs intrinsic AKI
- [ ] **PSA density** - PSA / prostate volume
- [ ] **Harris-Benedict** (Roza 1984) - BMR / daily energy estimate

### Future - specialty depth

- [ ] **Braden Scale**, **Norton Scale** - geriatrics (pressure ulcer; complements Waterlow)
- [ ] **Barthel Index** - geriatrics (ADLs)
- [ ] **RCRI** (Lee) - surgery (pre-op cardiac risk)
- [ ] **Caprini** - surgery (peri-op VTE; complements Padua for medical)
- [ ] **Hinchey** - surgery (acute diverticulitis anatomy)
- [ ] **LRINEC** - infectious diseases (necrotising fasciitis)
- [ ] **Pitt Bacteraemia** - infectious diseases (BSI severity)
- [ ] **Modified Duke criteria** - infectious diseases (endocarditis)
- [ ] **BASDAI** - rheumatology (ankylosing spondylitis)
- [ ] **PASI** - dermatology (psoriasis)
- [ ] **SCORAD** - dermatology (atopic dermatitis)
- [ ] **ORBIT** - cardiology (bleeding risk in AF, DOAC era)
- [ ] **EHRA** - cardiology (AF symptom classification)
- [ ] **SCORE2 / SCORE2-OP** - cardiology (ESC 2021 CV risk; verify licensing)
- [ ] **ASCVD Pooled Cohort** - cardiology (ACC/AHA 2013; US population)
- [ ] **FINDRISC** - endocrinology (T2DM risk)

### Future - paediatrics (special case)

- [ ] **RCPCH Digital Growth Charts** - UK-WHO (0-4y, WHO 2006) and UK90 (4-20y) reference data, gestational-age correction for prematurity, z-score / centile / SDS, chart rendering in the GUI. Needs the LMS reference tables (binary-size variable) and confirmation of RCPCH licensing terms.

---

## Engineering & infrastructure

Roadmap items that are not a new calculator but cover other aspects of the design and build. Status uses the same legend as above.

### Completed

- [x] **calc-core / calc-cli workspace** with one shared registry driving every surface (`calc_core::all()`).
- [x] **`Calculator` trait** with mandatory `license()`, registry-tested.
- [x] **Schema-driven input templates** (no per-calculator clap struct); registry round-trips for all 52 calculators.
- [x] **Proprietary "unavailable" stub treatment** - 10 named-not-hidden entries with open alternatives.
- [x] **Input-definition system** for clinician-asserted predicates (`spec/calculator-input-definitions.md`).
- [x] **Zensical docs site** deployed to GitHub Pages (`.github/workflows/deploy-docs-to-ghpages.yml`).
- [x] **AGENTS.md** + canonical `s/` script directory (`s/docs`, `s/install`).
- [x] **Tag taxonomy** in `calc-core/src/tags.rs` + `calc list --tag` CLI filter.
- [x] **SIGPIPE-clean stdout** so `calc list | head` is well-behaved.
- [x] **House-style alignment**: SPDX headers on every Rust source, AGPL-3.0-or-later, SHA-pinned GitHub Actions.

### In-progress

_None active right now._

### Planned

- [ ] **`cargo-dist` release pipeline** - prebuilt binaries for macOS / Linux / Windows, then publish `calc-cli` to crates.io so `cargo install calc-cli` works without `--git`.
- [ ] **`pacharanero/homebrew-tap` formula** - matches sibling repos (sct, dsc, gitehr).
- [ ] **Tauri desktop GUI** - standalone app whose headline is prominent copy-paste ("soft interoperability"). Cross-compiles to iOS / Android because `calc-core` is pure Rust.
- [ ] **Retire `.claude/skills/build-calculator/`** in favour of `spec/` + `examples/` + `AGENTS.md` as the authoring entry point (per the recommendation in `spec/calculators.md`).
- [ ] **Rewrite the build-calculator workflow** so it depends only on this repo's docs - no Claude-specific path.

### Future

- [ ] **Multilingual support** - implement `Locale` enum + `LocalizedString` per the design in [`spec/multilingual.md`](multilingual.md); validate with one calculator (FeverPAIN) and a native speaker before opening the catalogue for batched translation.
- [ ] **Translation reciprocity with MedikQuantis** - their Catalan and Spanish strings for the 14 calculators we both ship are exactly what we need; agree a shared tag taxonomy and a citation-shape so either project can ingest the other's metadata.
- [ ] **Reference MCP server** in this repo (today, embedding hosts roll their own from `calc_core::all()`).
- [ ] **`calc-web`** (single-file HTML calculators) returning, ideally with `calc-core` compiled to WebAssembly so the browser surface shares the engine. Currently deprioritised.
- [ ] **FHIR Observation export** for standardised exchange of results.
- [ ] **Unit conversion** (metric ↔ imperial) at the input boundary; today `--input` is units-explicit.
- [ ] **Printable / clipboard-friendly result formatting** beyond the existing text block (PDF? RTF? rich Markdown with citation links?).
- [ ] **User-defined / third-party calculators via a plugin system** - load extra calculators at runtime from a directory or URL.
- [ ] **Guideline-update registry** - a mechanism for re-verifying each calculator's licence and reference URL on a schedule.
- [ ] **High-risk-score alerts** when embedded in a recording host (e.g. NEWS2 ≥ 7 fires an event the host can subscribe to).
