import react from "@vitejs/plugin-react";
/// <reference types="@batijs/core/types" />
import tailwindcss from "@tailwindcss/vite";
import vike from "vike/plugin";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [vike(), react(), tailwindcss()]
});
