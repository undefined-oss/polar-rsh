// Execute the pinned Polar submodule source using Node's TypeScript transformer.
import { existsSync, readFileSync } from "node:fs";
import { registerHooks, stripTypeScriptTypes } from "node:module";
import { extname } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = pathToFileURL(`${process.argv[2]}/`).href;
registerHooks({
  resolve(specifier, context, nextResolve) {
    if (specifier.startsWith(".") && context.parentURL?.startsWith(root) && !extname(specifier)) {
      for (const suffix of [".ts", "/index.ts"]) {
        const candidate = new URL(specifier + suffix, context.parentURL);
        if (existsSync(fileURLToPath(candidate))) return nextResolve(candidate.href, context);
      }
    }
    return nextResolve(specifier, context);
  },
  load(url, context, nextLoad) {
    if (url.startsWith(root) && url.endsWith(".ts")) {
      return {
        format: "module",
        shortCircuit: true,
        source: stripTypeScriptTypes(readFileSync(fileURLToPath(url), "utf8"), {
          mode: "transform",
          sourceUrl: url,
        }),
      };
    }
    return nextLoad(url, context);
  },
});

let input = "";
for await (const chunk of process.stdin) input += chunk;
const { scenario, base_url: baseUrl, api_version: apiVersion } = JSON.parse(input);
if (!/^\d{4}-(0[1-9]|1[0-2])$/.test(apiVersion)) throw new Error("Invalid API version");
const { ClientBase } = await import(new URL("sdk/typescript/src/base.ts", root));
const { createProductsService } = await import(new URL(`sdk/typescript/src/${apiVersion}/services/products.ts`, root));
const { createCustomersService } = await import(new URL(`sdk/typescript/src/${apiVersion}/services/customers/index.ts`, root));
const client = new ClientBase({
  baseUrl,
  version: apiVersion,
  accessToken: "contract-token",
  timeout: (scenario.timeout_ms ?? 5000) / 1000,
  organizationId: scenario.organization_id ?? undefined,
});
const products = createProductsService(client);
const customers = createCustomersService(client);
const outcomes = [];
for (const action of scenario.actions) {
  const options = {};
  if (action.access_token !== undefined) options.accessToken = action.access_token;
  if (action.timeout_ms !== undefined) options.timeout = action.timeout_ms / 1000;
  try {
    let value;
    switch (action.operation) {
      case "products.list": value = await products.list(action.query, options); break;
      case "customers.get": value = await customers.get(action.id, options); break;
      case "customers.get_external": value = await customers.getExternal(action.id, options); break;
      case "customers.update": value = await customers.update(action.id, action.body, options); break;
      case "customers.delete": value = await customers.delete(action.id, action.query, options); break;
      default: throw new Error(`Unsupported contract operation: ${action.operation}`);
    }
    if (action.operation.startsWith("customers.") && action.operation !== "customers.delete") {
      value = Object.fromEntries(["id", "email", "name", "created_at", "organization_id"]
        .map((key) => [key, value[key]]));
    }
    outcomes.push({ ok: value ?? null });
  } catch (error) {
    if (typeof error.statusCode === "number") {
      const failure = { kind: "http", status: error.statusCode };
      if (error.statusCode === 429) failure.retry_after = error.retryAfter;
      else if (error.statusCode < 500) failure.body = error.error;
      outcomes.push({ error: failure });
    } else if (error.name === "PolarNetworkError" && /timeout|timed out/i.test(error.message)) {
      outcomes.push({ error: { kind: "timeout" } });
    } else {
      // Unexpected parser, loader, or transport errors must fail the harness.
      throw error;
    }
  }
}
process.stdout.write(JSON.stringify(outcomes));
