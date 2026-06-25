// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Component, type ErrorInfo, type ReactNode } from "react";

/**
 * Catches render-time errors anywhere in the React tree below it.
 *
 * Without this, an uncaught exception in any descendant (including
 * effects' setState-on-error paths) unmounts the entire app and leaves
 * a blank window with no signal to the user. Tauri's webview won't show
 * a helpful overlay in release builds - we have to render the error
 * ourselves.
 *
 * The fallback uses plain HTML + inline styles rather than Mantine
 * components, so it renders even if Mantine itself is the source of the
 * crash (otherwise the fallback would just retrigger the same error and
 * we'd be back to a blank window).
 */
interface State {
  error: Error | null;
  componentStack: string | null;
}

export class ErrorBoundary extends Component<{ children: ReactNode }, State> {
  state: State = { error: null, componentStack: null };

  static getDerivedStateFromError(error: Error): State {
    return { error, componentStack: null };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    // Make the error visible on the dev console even when devtools were
    // opened after the crash; React already logs once but a second log
    // tagged from here makes it easy to grep for in production logs.
    // eslint-disable-next-line no-console
    console.error("[calc] uncaught render error:", error, info);
    this.setState({ componentStack: info.componentStack ?? null });
  }

  render() {
    const { error, componentStack } = this.state;
    if (!error) return this.props.children;

    return (
      <div
        style={{
          padding: "32px",
          maxWidth: 800,
          margin: "0 auto",
          fontFamily:
            "'IBM Plex Sans', system-ui, -apple-system, sans-serif",
          color: "#1a1a1a",
          lineHeight: 1.55,
          overflow: "auto",
          maxHeight: "100vh",
        }}
      >
        <h2 style={{ color: "#c92a2a", marginTop: 0 }}>
          Something went wrong rendering the page
        </h2>
        <p style={{ color: "#555" }}>
          The calculator UI threw an error. The scoring engine itself is
          unaffected - you can still use the equivalent CLI command. Please
          share the message and stack below as a bug report.
        </p>

        <h3>Error</h3>
        <pre
          style={{
            background: "#fff5f5",
            border: "1px solid #ffe0e0",
            color: "#c92a2a",
            padding: 12,
            borderRadius: 6,
            whiteSpace: "pre-wrap",
            wordBreak: "break-word",
          }}
        >
          {error.name}: {error.message}
        </pre>

        {error.stack && (
          <>
            <h3>Stack</h3>
            <pre
              style={{
                background: "#f5f5f5",
                border: "1px solid #e0e0e0",
                padding: 12,
                borderRadius: 6,
                maxHeight: 260,
                overflow: "auto",
                fontSize: 12,
                fontFamily: "ui-monospace, Menlo, Consolas, monospace",
              }}
            >
              {error.stack}
            </pre>
          </>
        )}

        {componentStack && (
          <>
            <h3>Component tree</h3>
            <pre
              style={{
                background: "#f5f5f5",
                border: "1px solid #e0e0e0",
                padding: 12,
                borderRadius: 6,
                maxHeight: 260,
                overflow: "auto",
                fontSize: 12,
                fontFamily: "ui-monospace, Menlo, Consolas, monospace",
              }}
            >
              {componentStack}
            </pre>
          </>
        )}

        <p style={{ fontSize: 13, color: "#777" }}>
          Right-click anywhere and choose <em>Inspect</em> to open the
          developer console for the full picture (network, more logs),
          then reload with <kbd>Ctrl/Cmd-R</kbd>.
        </p>
      </div>
    );
  }
}
