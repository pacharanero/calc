# Calculator catalogue

42 active clinical calculators plus 10 named-but-unavailable (proprietary or licence-locked). Updated automatically from `calc list --format json`.

`calc list` prints this same catalogue at any time; `calc <name> --license` prints the algorithm's distribution licence for any single entry; `calc list --tag <tag>` filters by tag.

!!! info "Two kinds of entry"
    - **Active** calculators compute a real score. Their algorithm is either public-domain (implemented from primary literature) or open-source (notably QRISK3 and QFracture, ported from ClinRisk's LGPL-3 source).
    - **Unavailable** entries are named on purpose. They appear in `calc list`, but invoking them returns a structured explanation - owner, reason, and an open alternative where one exists. See [Unavailable on principle](#unavailable-on-principle).

## Filtering by tag

Every calculator carries one or more **tags** - specialty (where it is used) and status (`proprietary`, `nhs-mandated`, `screening`, `risk`, ...). Tags drive the groupings below, the `--tag` CLI filter, and the JSON output of `calc list`:

```bash
calc list --tag cardiology                       # everything in cardiology
calc list --tag primary-care --tag screening     # AND - narrows the filter
calc list --tag proprietary                      # the unshippable ones
calc --tags                                      # enumerate every tag, with counts
```

The full vocabulary lives in [`calc-core/src/tags.rs`](https://github.com/pacharanero/calc/blob/main/calc-core/src/tags.rs) and is reviewable in one file. New tags are added there only after at least two calculators want one.

## Primary care / NHS-mandated (high volume)

| Name | Title | What it is |
|---|---|---|
| `qrisk3` | QRISK3 | 10-year cardiovascular risk (NICE NG238). LGPL port of ClinRisk. |
| `qfracture` | QFracture | 10-year major osteoporotic and hip fracture risk; the open UK alternative to FRAX. |
| `phq9` | PHQ-9 | Nine-item depression severity (0-27); item 9 flags self-harm risk. |
| `gad7` | GAD-7 | Seven-item generalised anxiety severity (0-21); 10+ flags likely GAD. |
| `audit` | AUDIT | Ten-item WHO alcohol-use screen (0-40) with four risk zones. |
| `auditc` | AUDIT-C | Three-item AUDIT consumption subscale (0-12). |
| `egfr` | eGFR (CKD-EPI 2021) | Race-free eGFR from creatinine; reports CKD G-stage. |
| `fib4` | FIB-4 | Non-invasive screen for advanced liver fibrosis (NICE NG49). |
| `feverpain` | FeverPAIN | Five-item score guiding antibiotics in acute sore throat. |

## Acute / emergency

| Name | Title | What it is |
|---|---|---|
| `news2` | NEWS2 | NHS-mandated aggregate physiology score (RCP 2017). |
| `curb65` | CURB-65 | Severity and 30-day mortality risk in CAP (BTS / NICE NG138). |
| `wells_dvt` | Wells Score (DVT) | Pre-test probability of DVT (NICE NG158). |
| `wells_pe` | Wells Score (PE) | Pre-test probability of PE (NICE NG158). |
| `grace` | GRACE | In-hospital mortality risk in ACS (Granger 2003). |
| `heart` | HEART | 6-week MACE risk for ED chest pain. |
| `timi` | TIMI | 14-day risk in UA/NSTEMI (Antman 2000). |
| `cha2ds2vasc` | CHA2DS2-VASc | Stroke risk in AF (NICE NG196). |
| `hasbled` | HAS-BLED | Bleeding risk on anticoagulation (NICE NG196). |
| `abcd2` | ABCD2 | 2-day stroke risk after TIA. |
| `qsofa` | qSOFA | Bedside Sepsis-3 prompt. |
| `sofa` | SOFA | Six-organ dysfunction score (0-24); underpins Sepsis-3. |
| `fourat` | 4AT | Rapid bedside delirium screen. |
| `chalice` | CHALICE | Paediatric head-injury CT decision rule (NICE NG232). |
| `padua` | Padua | VTE risk in medical inpatients (NICE NG89). |

## Chronic disease / specialist

| Name | Title | What it is |
|---|---|---|
| `mrc_dyspnoea` | MRC Dyspnoea | Classic 1-5 breathlessness grade. |
| `ipss` | IPSS | International Prostate Symptom Score. |
| `das28` | DAS28 | Rheumatoid arthritis disease activity. |
| `uacr` | uACR | Urine albumin-to-creatinine ratio + KDIGO A-stage. |
| `ckd_risk` | KDIGO CKD risk | Combines eGFR G-stage and ACR A-stage into the KDIGO heatmap. |
| `epds` | EPDS | Edinburgh Postnatal Depression Scale. |
| `amts` | AMTS | Ten-item bedside cognitive screen. |
| `bode` | BODE | COPD prognostic index. |
| `child_pugh` | Child-Pugh | Cirrhosis severity class A/B/C. |
| `meld` | MELD | 3-month mortality in end-stage liver disease (Kamath 2001). |
| `ukeld` | UKELD | UK liver-transplant listing score (Barber 2011). |
| `nhfs` | Nottingham Hip Fracture Score | Preoperative 30-day mortality after hip-fracture surgery. |
| `euroscore2` | EuroSCORE II | Operative mortality after cardiac surgery (Nashef 2012). |
| `gleason` | Gleason Grade Group | ISUP/WHO grade group from prostate biopsy patterns. |
| `npi` | Nottingham Prognostic Index | Prognosis in primary operable breast cancer. |
| `abpi` | ABPI | Ankle-Brachial Pressure Index, per leg. |
| `waterlow` | Waterlow | Pressure-ulcer risk assessment. |
| `asrs` | ASRS-v1.1 | WHO-validated adult ADHD screener. |

## Unavailable on principle

These are listed - and therefore discoverable, schemable, and embeddable in a host - but invoking them returns a structured "unavailable" response, never a score. Where an open alternative exists in this catalogue, it is named in the response.

| Name | Reason in brief | Open alternative |
|---|---|---|
| `frax` | Trade secret; country coefficients never published. | `qfracture` |
| `mmse` | Copyrighted; commercial licensing since 2001. | `amts`, `fourat` |
| `must` | NICE-recommended but proprietary scoring tables. | - |
| `cat` | Copyrighted by GSK; restricted-use licence. | - |
| `acq` | Copyrighted by Elizabeth Juniper; per-use fees. | - |
| `elf` | Proprietary Siemens Healthineers assay. | `fib4` |
| `cfs` | Copyrighted; commercial use restricted. | - |
| `lanss` | Copyrighted; permission required. | - |
| `ohs` | Isis Innovation licence (commercial use). | - |
| `oks` | Isis Innovation licence (commercial use). | - |

Calling any of them produces a uniform response so a host can render the gap consistently:

```console
$ calc frax --input '{}'
frax = unavailable: proprietary
...
```

See [Why some calculators are unavailable](how-it-works.md#unavailable-on-principle) for the rationale.

## Wishlist (candidates for future addition)

Calculators below are clinically valuable and on the radar but not yet implemented. Several originate from [MedikQuantis](https://medikquantis.me) (Laura Piró, Barcelona, MIT) - a sibling project we're exploring collaboration with.

Contributions welcome. The shape of the work is documented in [How it works](how-it-works.md#embedding-calc-in-a-host) and the [build-calculator skill](https://github.com/pacharanero/calc/tree/main/.claude/skills/build-calculator).

### High priority

| Candidate | Specialty | Why |
|---|---|---|
| **GCS** (Glasgow Coma Scale) | neurology, emergency | Universal bedside score; trauma, sedation, neuro obs. |
| **NIHSS** | neurology, emergency | Acute stroke severity standard. |
| **Charlson Comorbidity Index** | internal medicine | Most-cited comorbidity index; 10-year mortality. |
| **APACHE II** | intensive-care | Classic ICU severity / mortality. |
| **MELD 3.0** | hepatology | Updated MELD (we ship the 2001 original). |
| **PERC** | emergency | PE rule-out criteria; complements Wells PE. |
| **Glasgow-Blatchford** | gastroenterology, emergency | Upper-GI bleed pre-endoscopy triage. |
| **Centor / McIsaac** | primary-care, emergency | Sore-throat triage; complements FeverPAIN. |
| **ASA Physical Status** | anaesthesia, surgery | Universal pre-op classification. |
| **NYHA** | cardiology | Heart-failure functional class. |
| **CHA2DS2-VA** | cardiology | 2024 ESC sex-free update of CHA2DS2-VASc. |

### Useful clinical helpers (simple one-formula calculators)

| Candidate | What it does |
|---|---|
| **Albumin-corrected calcium** (Payne 1973) | Adjusts total Ca for albumin. |
| **Hyperglycaemia-corrected sodium** (Katz/Hillier) | Expected Na at normoglycaemia (DKA workup). |
| **Anion gap** | Na − (Cl + HCO₃); screens for HAGMA. |
| **FENa** | Separates prerenal from intrinsic AKI. |
| **PSA density** | PSA / prostate volume; grey-zone PSA. |
| **Harris-Benedict** (Roza 1984) | BMR / daily energy estimate. |

### Specialty depth

| Candidate | Specialty |
|---|---|
| **Braden Scale**, **Norton Scale** | geriatrics (pressure-ulcer; we have Waterlow) |
| **Barthel Index** | geriatrics (ADLs) |
| **RCRI (Lee)** | surgery (pre-op cardiac risk) |
| **Caprini** | surgery (peri-op VTE; we have Padua for medical) |
| **LRINEC** | infectious-diseases (necrotising fasciitis lab indicator) |
| **Pitt Bacteraemia** | infectious-diseases (BSI severity) |
| **Modified Duke criteria** | infectious-diseases (endocarditis) |
| **Hinchey** | surgery (acute diverticulitis anatomy) |
| **BASDAI** | rheumatology (ankylosing spondylitis) |
| **PASI**, **SCORAD** | dermatology (psoriasis, atopic dermatitis) |
| **ORBIT**, **EHRA** | cardiology (bleeding / AF symptoms) |
| **SCORE2 / SCORE2-OP** | cardiology (ESC 2021 CV risk - check licensing) |
| **ASCVD Pooled Cohort** | cardiology (ACC/AHA 2013; US population) |
| **FINDRISC** | endocrinology (T2DM risk) |

### Why this list

Sibling open-source projects approach the same problem from different angles. MedikQuantis ships multilingual (Catalan, Spanish, English) cardiac/ICU/derm/surgery scoring; `calc` is UK-first with NICE-aligned screening, mental-health, perinatal, and "name the unavailable" stubs. Adding the candidates above is mostly mechanical (each is one Rust file plus literature-vector tests) and fills our specialist gaps directly. See [`spec/multilingual.md`](https://github.com/pacharanero/calc/blob/main/spec/multilingual.md) for the multilingual design that would let us ingest their translations directly.
