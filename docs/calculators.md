# Calculator catalogue

42 active clinical calculators plus 10 named-but-unavailable (proprietary or licence-locked). Updated automatically from `calc list --format json`.

`calc list` prints this same catalogue at any time; `calc <name> --license` prints the algorithm's distribution licence for any single entry.

!!! info "Two kinds of entry"
    - **Active** calculators compute a real score. Their algorithm is either public-domain (implemented from primary literature) or open-source (notably QRISK3 and QFracture, ported from ClinRisk's LGPL-3 source).
    - **Unavailable** entries are named on purpose. They appear in `calc list`, but invoking them returns a structured explanation - owner, reason, and an open alternative where one exists. See [Unavailable on principle](#unavailable-on-principle).

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
