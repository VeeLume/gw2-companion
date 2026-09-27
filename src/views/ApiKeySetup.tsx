import { createSignal, type Component } from "solid-js";
import { setApiKey } from "../lib/api";

const ApiKeySetup: Component = () => {
  const [key, setKey] = createSignal("");
  const [status, setStatus] = createSignal<"idle" | "loading" | "success" | "error">("idle");
  const [errorMsg, setErrorMsg] = createSignal("");

  const handleSubmit = async () => {
    if (!key().trim()) return;
    setStatus("loading");
    setErrorMsg("");

    try {
      await setApiKey(key().trim());
      setStatus("success");
    } catch (e) {
      setStatus("error");
      setErrorMsg(String(e));
    }
  };

  return (
    <div>
      <h2 class="text-2xl font-bold mb-4" style={{ color: "var(--gw2-gold)" }}>
        API Key Setup
      </h2>

      <div
        class="rounded-lg p-6 border border-white/10 max-w-lg"
        style={{ "background-color": "var(--gw2-surface)" }}
      >
        <p class="mb-4" style={{ color: "var(--gw2-muted)" }}>
          Create an API key at{" "}
          <span class="underline">account.arena.net/applications</span> with
          these permissions: account, characters, inventories, progression,
          tradingpost, unlocks, wallet.
        </p>

        <div class="flex gap-2">
          <input
            type="password"
            placeholder="Paste your API key..."
            class="flex-1 px-3 py-2 rounded-md bg-black/30 border border-white/10 text-sm focus:outline-none focus:border-white/30"
            style={{ color: "var(--gw2-text)" }}
            value={key()}
            onInput={(e) => setKey(e.currentTarget.value)}
          />
          <button
            class="px-4 py-2 rounded-md text-sm font-medium transition-colors"
            style={{
              "background-color": "var(--gw2-gold)",
              color: "var(--gw2-bg)",
            }}
            onClick={handleSubmit}
            disabled={status() === "loading"}
          >
            {status() === "loading" ? "Validating..." : "Save"}
          </button>
        </div>

        {status() === "success" && (
          <p class="mt-3 text-green-400 text-sm">API key validated and saved!</p>
        )}
        {status() === "error" && (
          <p class="mt-3 text-red-400 text-sm">{errorMsg()}</p>
        )}
      </div>
    </div>
  );
};

export default ApiKeySetup;
