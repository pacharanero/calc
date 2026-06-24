// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

import React from "react";
import ReactDOM from "react-dom/client";
import { createTheme, MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";
import App from "./App";
import "./App.css";

// Theme mirrors the gitehr GUI so the two apps read as one product family:
//   - IBM Plex Sans for body, Space Grotesk for headings
//   - primaryColor "teal" matches the docs-site palette
//     (docs/stylesheets/extra.css: --calc-teal #0f766e / bright #14b8a6)
// `defaultRadius: "md"` keeps cards / buttons softly-rounded rather than
// boxy; clinicians have read enough hospital-IT chrome for a lifetime.
const theme = createTheme({
  fontFamily: "'IBM Plex Sans', system-ui, -apple-system, sans-serif",
  headings: { fontFamily: "'Space Grotesk', 'IBM Plex Sans', sans-serif" },
  primaryColor: "teal",
  defaultRadius: "md",
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider theme={theme} defaultColorScheme="auto">
      <Notifications position="top-right" />
      <App />
    </MantineProvider>
  </React.StrictMode>,
);
