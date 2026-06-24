/// <reference types="vite/client" />

// Static SVG imports resolve to a URL string at build time. Without this
// declaration TypeScript's strict mode rejects `import x from "/logo.svg"`.
declare module "*.svg" {
  const src: string;
  export default src;
}
