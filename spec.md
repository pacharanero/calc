# Standalone Single-File Clinical Calculators — Specification

## Concept

Each clinical calculator is a **single, self-contained HTML file**. It requires no build step, no server, and no framework. It can be opened directly in a browser, served as a static file, embedded in another application, or loaded inside a desktop app.

The file contains everything it needs: structure, styles, logic, and content. The only permitted external dependency is a shared bridge module (`shared/gitehr-bridge.js`) that handles result dispatch, and optionally CDN-hosted CSS libraries.

---

## File Structure

```
  index.html                        ← gallery / landing page
  spec.md                           ← this document
  shared/
    gitehr-bridge.js                ← context detection + result dispatch
  calculators/
    <calculator-name>.html          ← one file per calculator
  clinical-source-references/
    <calculator-name>.xyz           ← authoritative clinical references for each calculator
```

Each `calculators/<calculator-name>.html` is independently deployable. None of them depend on each other.

---

## Calculator File Design Principles

### 1. Single file, no build step

The HTML file contains all markup, CSS, and JavaScript inline or via CDN link. It works when opened as `file://` only if no ES module imports are needed from sibling paths; otherwise it must be served over HTTP (any static file server suffices).

### 2. Bespoke UI per calculator

The layout, visual design, and interaction model are tailored to the clinical purpose of each tool. A questionnaire looks like a questionnaire. A unit converter looks like a converter. No two calculators need to look alike.

Design should be guided by:

- **Clinical context** — what is the clinician doing when they reach for this tool?
- **Patient-facing vs clinician-facing** — some tools are used by patients filling in forms; others are used by clinicians at the point of care
- **Cognitive load** — reduce it; show only what is needed at each step
- **Accessibility** — keyboard navigable, screen-reader compatible, sufficient contrast

### 3. Logic implemented in JavaScript

The scoring or calculation logic is implemented in plain JavaScript, inline in a `<script type="module">` block. No external JS libraries are required for the logic itself.

For calculators with non-trivial statistical logic (e.g. growth charts, complex lookup tables), Pyodide (Python-in-WASM) may be used to run the authoritative Python code directly in the browser, avoiding any reimplementation risk.

---

## The Git EHR Bridge (`shared/gitehr-bridge.js`)

A small ES module that makes each calculator context-aware without requiring the calculator author to know anything about the host environment.

### Context detection

| Context | Detection condition |
|---|---|
| **Tauri (GitEHR desktop)** | `window.__TAURI__` is present |
| **iframe embed** | `window.parent !== window` |
| **Standalone** | none of the above |

### Exported API

```js
import { sendResult, getPatientContext, getContext,
         saveButtonLabel, copyToClipboard } from '../shared/gitehr-bridge.js';
```

**`sendResult(data)`** — dispatch a completed result to the host:
- Tauri: `window.__TAURI__.event.emit('calculator-complete', data)`
- iframe: `window.parent.postMessage({ type: 'calculator-result', ...data }, '*')`
- Standalone: no-op (the calculator's own UI already shows the result)

**`getPatientContext()`** — reads URL query parameters injected by the host before opening the calculator. Returns an object with any of: `patient_id`, `given_name`, `family_name`, `dob`, or any other key the host chooses to pass.

**`getContext()`** — returns `'tauri' | 'iframe' | 'standalone'`

**`saveButtonLabel()`** — returns a context-appropriate label for the primary action button: `'Save to patient record'` / `'Send result'` / `'Copy result'`

**`copyToClipboard(data)`** — formats a plain-text summary of the result and copies it to the clipboard. Returns `true` on success.

### Result data schema

This mirrors the Python `CalculationResponse` model:

```json
{
  "calculator": "asrs_screener",
  "result": 5,
  "interpretation": "Positive screen: 5/6 items meet the frequency threshold...",
  "working": {
    "part_a_screen_result": "POSITIVE",
    "part_a_positive_item_count": 5,
    "total_score": 48
  },
  "reference": "Kessler RC et al. (2005). Psychol Med. 35(2):245-56.",
  "patient_context": { "patient_id": "...", "given_name": "..." }
}
```

---

## Embedding in GitEHR

GitEHR (a Tauri desktop application) opens a calculator by launching a `WebviewWindow` that loads the HTML file, optionally appending patient context to the URL:

```
calculators/asrs_screener.html?patient_id=abc123&given_name=Jane&family_name=Smith
```

The calculator renders normally. When the clinician completes it and activates the primary action, `sendResult(data)` fires the `calculator-complete` Tauri event. GitEHR handles this event and writes the result to the patient record — as a **journal entry** (immutable, timestamped, with YAML frontmatter) or a **state file** (updatable current clinical state), depending on the nature of the result.

For inline embedding within the main GitEHR UI (a panel rather than a new window), the same HTML is loaded in a Tauri webview element; `postMessage` is used instead of Tauri events.

### Patient pre-population

Any field the host passes as a URL query parameter is available via `getPatientContext()`. Calculators can use this to label results, pre-fill fields, or include patient identifiers in the dispatched result without prompting the user to re-enter information GitEHR already holds.

---

## Result Card UI Conventions

Every calculator renders a result card when the user has completed all inputs. The result card must contain these elements in this order:

### 1. Score summary and interpretation

Score tiles, a result badge (positive/negative/severity level), and a human-readable interpretation string. These are populated by JavaScript from the scoring function.

### 2. Breakdown (optional but recommended)

A collapsible `<details>` block showing per-item or per-criterion scores. Helps the clinician understand how the result was reached without cluttering the primary view.

### 3. Clipboard preview textarea

Before the action buttons, render an editable `<textarea>` pre-filled with the plain-text summary that will be copied to the clipboard:

```html
<div id="clipboard-preview-wrap" class="hidden mb-4">
  <label for="clipboard-preview"
         class="text-xs font-semibold text-gray-500 uppercase tracking-wide block mb-1">
    Text to copy — edit if needed
  </label>
  <textarea
    id="clipboard-preview"
    rows="7"
    class="w-full border border-gray-200 rounded-lg px-3 py-2 text-sm font-mono resize-y"
    style="outline-color: var(--mid-blue);"
  ></textarea>
</div>
```

JavaScript populates the textarea when the result is first shown, and hides it again on "Start over". The copy button reads directly from `previewTA.value` rather than regenerating the text, so any edits the clinician makes are preserved.

Use `formatClipboardText(resultData)` from the bridge for simple results, or a bespoke `buildSummaryText()` function for calculators whose clipboard output is richer than the generic schema (e.g. FeverPAIN, which includes a clinical narrative derived from the input values).

#### Dynamic refresh for treatment decisions

Any post-result selection that changes the clinical recommendation — prescribing strategy, dosing decision, follow-up advice — **must update the textarea in real time**. Store the last-computed score and interpretation at module level, and call a `refreshPreview()` function from every change listener that affects the summary text:

```js
let currentScore = null;
let currentInterp = null;

function refreshPreview() {
  if (currentScore === null) return;
  const previewTA = document.getElementById('clipboard-preview');
  if (previewTA) previewTA.value = buildSummaryText(currentScore, currentInterp);
}

// In renderActionButtons / showResult:
currentScore = score;
currentInterp = interp;

// In every treatment-decision change listener:
strategyRadio.addEventListener('change', () => { selectedStrategy = r.value; refreshPreview(); });
delayDaysSelect.addEventListener('change', () => { delayDays = dd.value; refreshPreview(); });
```

Clear `currentScore` and `currentInterp` on "Start over".

### 4. Action buttons

Rendered by JavaScript after context detection via `getContext()`:

| Context      | Primary button                            | Secondary button | Always present              |
| ------------ | ----------------------------------------- | ---------------- | --------------------------- |
| `tauri`      | "Save to patient record" (`sendResult`)   | —                | "Copy result", "Start over" |
| `iframe`     | "Send result" (`sendResult`)              | —                | "Copy result", "Start over" |
| `standalone` | —                                         | —                | "Copy result", "Start over" |

Use `saveButtonLabel()` for the primary button label. The copy button always copies from the preview textarea.

---

## Adding a new calculator

1. Create `ui/calculators/<calculator-name>.html`
2. Design a bespoke UI appropriate to the clinical purpose
3. Implement the scoring/calculation logic in a `<script type="module">` block, using the Python source as the reference
4. Import and call `sendResult(data)` when the user completes the tool
5. Import and call `getPatientContext()` to pre-populate or label results if context is provided
6. Verify the JS output matches the Python test vectors for all significant input combinations
7. Add a card to `ui/index.html`

---

## Constraints and non-goals

- **No build step** — no bundler, no transpiler, no package.json
- **No shared state between calculators** — each file is fully independent
- **No authentication or data storage** — the calculator computes and dispatches; persistence is the host's responsibility
- **No automatic form generation** — every UI element is intentional and hand-authored
- **CDN dependencies are acceptable** but must gracefully degrade if offline (show a message; do not silently fail)
