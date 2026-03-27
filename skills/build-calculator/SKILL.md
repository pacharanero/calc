---
name: build-calculator
description: Use this skill when asked to build, scaffold, or add a new clinical calculator to this project. Covers the full workflow: reading the spec, designing bespoke UI, implementing scoring logic, wiring the bridge, and following all result-card conventions.
version: 1.0.0
---

# Build a Clinical Calculator

## Before you start

Read these files in full — they are the source of truth:

- `spec.md` — architecture, bridge API, result-card conventions, constraints
- `shared/gitehr-bridge.js` — the bridge module you will import
- An existing calculator (e.g. `calculators/feverpain.html`) — study its structure end-to-end before writing a single line

Also check `calculator-roadmap.md` for any notes on the calculator you are about to build.

---

## File to create

```
calculators/<calculator-name>.html
```

One self-contained HTML file. No build step. No external JS beyond the bridge and optional CDN CSS.

---

## Structure of a calculator file

```
<head>
  charset, viewport, title
  DaisyUI 5 + Tailwind CSS 4 via CDN          ← shared CDN stack
  <style type="text/tailwindcss">
    RCPCH brand tokens (--rcpch-dark-blue etc.)
    Layout, question rows, result card, buttons
  </style>
</head>

<body>
  <header class="site-header">               ← RCPCH branded header bar
  <div style="background: dark-blue">        ← hero / intro band with live counter
  <div class="disclaimer">                   ← clinical disclaimer
  <main>
    <!-- input fields / questions -->

    <!-- Result card (hidden until complete) -->
    <div id="result-card" class="hidden">
      <!-- score tiles -->
      <!-- interpretation text -->
      <!-- breakdown (collapsible details) -->
      <!-- reference citation -->

      <!-- Clipboard preview -->
      <div id="clipboard-preview-wrap" class="hidden mb-4">
        <label for="clipboard-preview" ...>Text to copy — edit if needed</label>
        <textarea id="clipboard-preview" rows="7|10" class="... font-mono resize-y"></textarea>
      </div>

      <!-- Action buttons (rendered by JS) -->
      <div id="result-actions"></div>
    </div>
  </main>
  <footer>

  <script type="module">
    import { sendResult, getPatientContext, getContext,
             saveButtonLabel, formatClipboardText }
      from '../shared/gitehr-bridge.js';

    // 1. Question / field data
    // 2. State object
    // 3. Scoring function (mirrors Python exactly)
    // 4. Interpret function
    // 5. Render inputs
    // 6. Handle input events → call showResult() when complete
    // 7. showResult()
    // 8. renderBreakdown()
    // 9. renderActionButtons()   ← see conventions below
    // 10. applyPatientContext()
    // 11. Init calls
  </script>
</body>
```

---

## RCPCH brand tokens (copy verbatim into every calculator)

```css
:root {
  --rcpch-dark-blue:  #003087;
  --rcpch-mid-blue:   #005EB8;
  --rcpch-light-blue: #41B6E6;
  --rcpch-green:      #009639;
  --rcpch-warm-grey:  #F2F2F0;
  --rcpch-mid-grey:   #D9D9D9;
}
```

---

## Scoring logic

- Implement scoring **in plain JavaScript**, inline in the `<script type="module">` block.
- Use the authoritative **Python source** in `clinical-source-references/` as the reference.
- Variable names and logic must mirror the Python exactly — this makes it trivially auditable.
- If the Python uses lookup tables or non-trivial statistics, consider Pyodide (see spec).

---

## Bridge usage

```js
import { sendResult, getPatientContext, getContext,
         saveButtonLabel, formatClipboardText }
  from '../shared/gitehr-bridge.js';

// Read patient context from URL params (may be empty in standalone)
const patientCtx = getPatientContext();

// Build result payload (mirrors Python CalculationResponse)
const resultData = {
  calculator:    'calculator_name',   // snake_case module name
  result:        primaryValue,        // number or short string
  interpretation: interpretationText,
  working:       { ...breakdown },    // all intermediate values
  reference:     'Author et al. ...',
};

// Dispatch to host
sendResult(resultData);

// Format clipboard text (for the preview textarea)
const text = formatClipboardText(resultData);
// OR write a bespoke buildSummaryText() when richer narrative is needed
```

---

## Result card conventions (mandatory)

Follow the order in `spec.md § Result Card UI Conventions`:

### Clipboard preview textarea

Always present. Populate it from `renderActionButtons` (or equivalent) when the result is first shown:

```js
const previewWrap = document.getElementById('clipboard-preview-wrap');
const previewTA   = document.getElementById('clipboard-preview');
previewTA.value = formatClipboardText(resultData);   // or buildSummaryText()
previewWrap.classList.remove('hidden');
```

Reset it on "Start over":

```js
document.getElementById('clipboard-preview-wrap').classList.add('hidden');
```

### Action buttons

```js
function renderActionButtons(resultData) {
  const container = document.getElementById('result-actions');
  container.innerHTML = '';

  const previewTA = document.getElementById('clipboard-preview');
  const ctx = getContext();

  if (ctx === 'tauri' || ctx === 'iframe') {
    const saveBtn = document.createElement('button');
    saveBtn.className = 'rcpch-btn';
    saveBtn.textContent = saveButtonLabel();
    saveBtn.addEventListener('click', () => sendResult(resultData));
    container.appendChild(saveBtn);
  }

  const copyBtn = document.createElement('button');
  copyBtn.className = 'rcpch-btn' +
    (ctx === 'tauri' || ctx === 'iframe' ? ' rcpch-btn-outline' : '');
  copyBtn.textContent = 'Copy result';
  copyBtn.addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(previewTA.value);
      copyBtn.textContent = 'Copied ✓';
    } catch {
      copyBtn.textContent = 'Copy failed';
    }
    setTimeout(() => (copyBtn.textContent = 'Copy result'), 2000);
  });
  container.appendChild(copyBtn);

  const resetBtn = document.createElement('button');
  resetBtn.className = 'rcpch-btn rcpch-btn-outline';
  resetBtn.textContent = 'Start over';
  resetBtn.addEventListener('click', resetAll);
  container.appendChild(resetBtn);
}
```

---

## Patient context

Always call `applyPatientContext()` on init. If the host passes patient info via URL params, display it unobtrusively in the header:

```js
function applyPatientContext() {
  const ctx = getPatientContext();
  if (ctx.given_name || ctx.family_name || ctx.patient_id) {
    // append a small tag near the header subtitle
  }
}
```

Include `patient_context: getPatientContext()` in the `resultData` payload passed to `sendResult`.

---

## Button CSS classes

```css
.rcpch-btn {
  background: var(--rcpch-mid-blue); color: white;
  border: none; border-radius: 0.4rem;
  padding: 0.6rem 1.4rem; font-weight: 600; cursor: pointer;
}
.rcpch-btn:hover { background: var(--rcpch-dark-blue); }
.rcpch-btn-outline {
  background: transparent; color: var(--rcpch-mid-blue);
  border: 2px solid var(--rcpch-mid-blue);
}
.rcpch-btn-outline:hover { background: var(--rcpch-mid-blue); color: white; }
```

---

## Checklist before finishing

- [ ] Scoring output matches Python test vectors for all significant inputs
- [ ] Result card shows in correct order: tiles → interpretation → breakdown → clipboard preview → action buttons
- [ ] Clipboard preview textarea appears when result is shown, hidden on reset
- [ ] Copy button reads from `previewTA.value`, not from a re-generated string
- [ ] `sendResult` is called with the full payload including `working` and `reference`
- [ ] `applyPatientContext()` is called on init
- [ ] "Start over" resets all answers, all visual state, hides result card and preview, scrolls to top
- [ ] Works as `file://` or over HTTP (check for any module import that would break `file://`)
- [ ] Keyboard navigable; sufficient colour contrast; screen-reader labels on interactive groups
- [ ] CDN dependency failure is handled gracefully (show a fallback message if Tailwind/DaisyUI doesn't load)
- [ ] Card added to `index.html`
