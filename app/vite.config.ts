import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  base: "/bichito-de-pda/",
  plugins: [react()],
  resolve: {
    preserveSymlinks: true,
  },
});
