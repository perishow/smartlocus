import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
// O proxy mapeia /api -> backend Rust (porta 3000), evitando CORS em dev.
export default defineConfig({
    plugins: [react()],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    server: {
        port: 5173,
        proxy: {
            "/api": {
                target: "http://localhost:3000",
                changeOrigin: true,
            },
        },
    },
});
