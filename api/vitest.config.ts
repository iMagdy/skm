import { defineConfig } from "vitest/config";

export default defineConfig({
  resolve: {
    alias: {
      "cloudflare:workers": new URL("./test/cloudflare-workers-stub.ts", import.meta.url).pathname
    }
  },
  test: {
    environment: "node",
    include: ["test/**/*.test.ts"]
  }
});
