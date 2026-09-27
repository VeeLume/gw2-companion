import { createSignal, type Component } from "solid-js";
import Sidebar from "./components/Sidebar";
import Dashboard from "./views/Dashboard";

export type View =
  | "dashboard"
  | "legendaries"
  | "achievements"
  | "trading-post"
  | "checklist"
  | "settings";

const App: Component = () => {
  const [currentView, setCurrentView] = createSignal<View>("dashboard");

  return (
    <div class="flex h-screen">
      <Sidebar current={currentView()} onChange={setCurrentView} />
      <main class="flex-1 overflow-y-auto p-6">
        {/* TODO: Route to view components */}
        <Dashboard />
      </main>
    </div>
  );
};

export default App;
