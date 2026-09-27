import type { Component } from "solid-js";
import type { View } from "../App";

interface SidebarProps {
  current: View;
  onChange: (view: View) => void;
}

const navItems: { view: View; label: string; icon: string }[] = [
  { view: "dashboard", label: "Dashboard", icon: "🏠" },
  { view: "legendaries", label: "Legendaries", icon: "⚔️" },
  { view: "achievements", label: "Achievements", icon: "🏆" },
  { view: "trading-post", label: "Trading Post", icon: "💰" },
  { view: "checklist", label: "Checklist", icon: "✅" },
  { view: "settings", label: "Settings", icon: "⚙️" },
];

const Sidebar: Component<SidebarProps> = (props) => {
  return (
    <nav
      class="w-56 flex flex-col border-r border-white/10 p-4"
      style={{ "background-color": "var(--gw2-surface)" }}
    >
      <h1
        class="text-lg font-bold mb-6 tracking-wide"
        style={{ color: "var(--gw2-gold)" }}
      >
        GW2 Companion
      </h1>

      <ul class="flex flex-col gap-1 flex-1">
        {navItems.map((item) => (
          <li>
            <button
              class="w-full text-left px-3 py-2 rounded-md text-sm transition-colors"
              classList={{
                "bg-white/10 font-medium": props.current === item.view,
                "hover:bg-white/5": props.current !== item.view,
              }}
              style={{
                color:
                  props.current === item.view
                    ? "var(--gw2-gold)"
                    : "var(--gw2-text)",
              }}
              onClick={() => props.onChange(item.view)}
            >
              <span class="mr-2">{item.icon}</span>
              {item.label}
            </button>
          </li>
        ))}
      </ul>
    </nav>
  );
};

export default Sidebar;
