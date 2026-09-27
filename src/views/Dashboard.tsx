import { createResource, type Component, Show } from "solid-js";
import { getApiKeyStatus, getAccountInfo, type Account } from "../lib/api";

const Dashboard: Component = () => {
  const [hasKey] = createResource(getApiKeyStatus);
  const [account] = createResource(
    () => hasKey(),
    async (authenticated) => {
      if (!authenticated) return null;
      try {
        return await getAccountInfo();
      } catch {
        return null;
      }
    }
  );

  return (
    <div>
      <h2
        class="text-2xl font-bold mb-4"
        style={{ color: "var(--gw2-gold)" }}
      >
        Dashboard
      </h2>

      <Show
        when={hasKey()}
        fallback={
          <div
            class="rounded-lg p-6 border border-white/10"
            style={{ "background-color": "var(--gw2-surface)" }}
          >
            <p class="mb-2">
              Welcome to GW2 Companion! Set up your API key in{" "}
              <strong>Settings</strong> to get started.
            </p>
            <p style={{ color: "var(--gw2-muted)" }}>
              You can create an API key at{" "}
              <span class="underline">account.arena.net/applications</span>
            </p>
          </div>
        }
      >
        <Show when={account()} keyed>
          {(acc) => (
            <div
              class="rounded-lg p-6 border border-white/10"
              style={{ "background-color": "var(--gw2-surface)" }}
            >
              <p class="text-lg font-medium">{acc.name}</p>
              <p style={{ color: "var(--gw2-muted)" }}>
                AP: {(acc.daily_ap ?? 0) + (acc.monthly_ap ?? 0)} · WvW Rank:{" "}
                {acc.wvw?.rank ?? "N/A"} · Fractal Level:{" "}
                {acc.fractal_level ?? "N/A"}
              </p>
            </div>
          )}
        </Show>
      </Show>
    </div>
  );
};

export default Dashboard;
