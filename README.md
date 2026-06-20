> [!IMPORTANT]
> **This repository has moved and is no longer maintained here.**
>
> The clinical calculators are now developed inside the main **GitEHR** repository: <https://github.com/gitehr/gitehr>
>
> They are built on a shared Rust engine (`calc-core`) that drives every surface from one source of truth - the CLI (`calc` / `gitehr calc`), the MCP server, the GitEHR desktop GUI, and the single-file web calculators. In the GitEHR repo:
>
> - Engine and result schema: `calc-core/`
> - CLI (standalone `calc` binary + reused by `gitehr calc`): `calc-cli/`
> - Single-file web calculators and the shared bridge: `calc-web/`
> - Specification and roadmap: `spec/calculators.md`, `spec/calculator-roadmap.md`
>
> Please direct all new work, issues, and contributions there. The contents below are retained for historical reference only.

---

# Clippable Clinical Calculators

Clinicians need access to clinical digital tools to provide good care. Yet the commercial incentives to add these calculators to EHRs are weak, and the technical and compliance barriers to building them are high. The result is a patchwork of calculators scattered across the web, often behind paywalls, and frequently implemented in ways that are difficult to use at the point of care.

This repository contains a collection of **standalone single-file clinical calculators** that are:

- **Open source** — anyone can view, use, modify, and share the code
- **Free to use** — no paywalls, no licenses, no restrictions
- **Easy to deploy** — each calculator is a single HTML file that can be opened directly
- **Context-aware** — they can detect if they're running inside a compatible EHR and dispatch results accordingly
- **Clippable** — results can be easily copied to the clipboard for use in other applications

### Soft Interoperability

'Soft' Interoperability is my term for this idea of 'copy and paste' interop. It empowers clinicians to use the tools they want to use, without being constrained by the limitations of their EHR, and enables clinicians to use their own decision-making capacity to determine if they want to use one of these calculators or not.

Copy and paste, despite being a common clinician workaround for the myriad deficiencies of EHRs, is often derided as a 'hack' or 'kludge' by software developers. We would all prefer the *real* kind of interoperability, where data flows seamlessly between systems without manual intervention. Until we get there, we need to embrace and optimise for the tools that clinicians are *actually* using, even if they're not perfect.

## Specification

see [spec.md](spec.md) for the technical specification of how these calculators are built and how they work.

## Skill

see
